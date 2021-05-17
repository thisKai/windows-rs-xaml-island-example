fn main() {
    embed_resource::compile("resources.rc");
    windows::build!(
        Windows::Win32::UI::WindowsAndMessaging::{HWND, SetWindowPos, SWP_SHOWWINDOW},
        Windows::UI::Xaml::Hosting::{DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory, WindowsXamlManager},
        Windows::UI::Xaml::Controls::Button,
    );
}
