use winit::platform::windows::WindowExtWindows;

mod bindings {
    windows::include_bindings!();
}
use {
    bindings::Windows::{
        Win32::UI::WindowsAndMessaging::{
            SetWindowPos, HWND, SWP_SHOWWINDOW,
        },
        UI::Xaml::{Hosting::DesktopWindowXamlSource, Controls::Button},
    },
    windows::{Abi, Guid, Interface, IntoParam, RawPtr, HRESULT},
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
};

fn main() -> windows::Result<()> {
    windows::initialize_sta()?;

    let desktop_source = DesktopWindowXamlSource::new()?;
    let interop: IDesktopWindowXamlSourceNative = desktop_source.cast()?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    interop.AttachToWindow(HWND(window.hwnd() as _))?;
    let xaml_island_hwnd = interop.GetWindowHandle()?;
    let size = window.inner_size();
    unsafe {
        SetWindowPos(
            xaml_island_hwnd,
            HWND::NULL,
            0,
            0,
            size.width as _,
            size.height as _,
            SWP_SHOWWINDOW,
        );
    }

    let button = Button::new()?;

    desktop_source.SetContent(&button)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct IDesktopWindowXamlSourceNative(::windows::IUnknown);
unsafe impl ::windows::Interface for IDesktopWindowXamlSourceNative {
    type Vtable = IDesktopWindowXamlSourceNative_abi;
    const IID: ::windows::Guid = ::windows::Guid::from_values(
        0x3cbcf1bf,
        0x2f76,
        0x4e9c,
        [0x96, 0xab, 0xe8, 0x4b, 0x37, 0x97, 0x25, 0x54],
    );
}
#[repr(C)]
pub struct IDesktopWindowXamlSourceNative_abi(
    pub unsafe extern "system" fn(this: RawPtr, iid: Guid, interface: *mut RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(RawPtr, HWND) -> HRESULT,
    pub unsafe extern "system" fn(RawPtr, *mut HWND) -> HRESULT,
);

impl IDesktopWindowXamlSourceNative {
    pub fn AttachToWindow<'a>(&self, wnd: impl IntoParam<'a, HWND>) -> windows::Result<()> {
        unsafe { (self.vtable().3)(self.abi(), wnd.into_param().abi()) }.ok()
    }
    pub fn GetWindowHandle(&self) -> windows::Result<HWND> {
        let mut result = HWND::NULL;
        unsafe { (self.vtable().4)(self.abi(), &mut result).from_abi(result) }
    }
}
