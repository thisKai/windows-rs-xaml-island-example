use {
    bindings::Windows::{
        Win32::{
            Foundation::HWND,
            System::WinRT::IDesktopWindowXamlSourceNative,
            UI::WindowsAndMessaging::{SetWindowPos, SWP_SHOWWINDOW},
        },
        UI::Xaml::{
            Controls::{Button, Grid, TextBlock},
            HorizontalAlignment,
            Hosting::DesktopWindowXamlSource,
            RoutedEventHandler, UIElement,
        },
    },
    windows::{Interface, IntoParam},
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::windows::WindowExtWindows,
        window::{Window, WindowBuilder},
    },
};

fn main() -> windows::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let island = XamlIsland::attached(&window)?;

    let grid = Grid::new()?;
    let button = Button::new()?;
    let text = TextBlock::new()?;
    text.SetText("blah")?;
    button.SetContent(&text)?;
    button.SetHorizontalAlignment(HorizontalAlignment::Center)?;
    button.Click(RoutedEventHandler::new(|sender, event| {
        println!("Click");
        Ok(())
    }))?;
    grid.Children()?.Append(&button)?;

    island.set_content(&grid)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => {
                    island.resize(new_size.width as _, new_size.height as _)
                }
                _ => (),
            },
            _ => (),
        }
    });
}

pub struct XamlIsland {
    hwnd: HWND,
    source: DesktopWindowXamlSource,
}
impl XamlIsland {
    pub fn attached(window: &Window) -> windows::Result<Self> {
        let source = DesktopWindowXamlSource::new()?;
        let interop: IDesktopWindowXamlSourceNative = source.cast()?;
        unsafe {
            interop.AttachToWindow(HWND(window.hwnd() as _))?;
        }
        let hwnd = unsafe { interop.get_WindowHandle() }?;
        let size = window.inner_size();

        let island = XamlIsland { hwnd, source };
        island.resize(size.width as _, size.height as _);

        Ok(island)
    }
    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            SetWindowPos(
                self.hwnd,
                HWND::default(),
                0,
                0,
                width,
                height,
                SWP_SHOWWINDOW,
            )
        };
    }
    pub fn set_content<'a>(&self, value: impl IntoParam<'a, UIElement>) -> windows::Result<()> {
        self.source.SetContent(value)
    }
}
