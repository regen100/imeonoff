fn main() {
    windows::build!(
      Windows::Win32::Intl::ImmGetDefaultIMEWnd,
      Windows::Win32::WindowsAndMessaging::GetForegroundWindow,
      Windows::Win32::WindowsAndMessaging::SendMessageA,
      Windows::Win32::WindowsAndMessaging::{LPARAM, WPARAM},
    );
}
