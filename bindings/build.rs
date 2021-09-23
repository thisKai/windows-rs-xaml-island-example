fn main() {
    windows::build!(
        Windows::Win32::Foundation::HWND,
        Windows::Win32::UI::WindowsAndMessaging::SetWindowPos,
        Windows::Win32::System::WinRT::IDesktopWindowXamlSourceNative2,
        Windows::UI::Xaml::Hosting::DesktopWindowXamlSource,
        Windows::UI::Xaml::Controls::{Button, TextBlock, Grid, UIElementCollection},
    );
}
