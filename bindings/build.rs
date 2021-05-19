fn main() {
    windows::build!(
        Windows::Foundation::EventRegistrationToken,
        Windows::Win32::UI::WindowsAndMessaging::{HWND, SetWindowPos, SWP_SHOWWINDOW},
        Windows::UI::Xaml::{FrameworkElement, RoutedEventHandler, HorizontalAlignment},
        Windows::UI::Xaml::Hosting::{DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory, WindowsXamlManager},
        Windows::UI::Xaml::Controls::{Button, TextBlock, Grid, UIElementCollection},
    );
}
