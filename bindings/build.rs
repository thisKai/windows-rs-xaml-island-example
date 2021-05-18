fn main() {
    windows::build!(
        Windows::Win32::UI::WindowsAndMessaging::{HWND, SetWindowPos, SWP_SHOWWINDOW},
        Windows::UI::Xaml::{FrameworkElement, HorizontalAlignment},
        Windows::UI::Xaml::Hosting::{DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory, WindowsXamlManager},
        Windows::UI::Xaml::Controls::{Button, TextBlock, Grid, UIElementCollection},
    );
}
