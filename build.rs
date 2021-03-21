fn main() {
    windows::build!(
      windows::win32::intl::ImmGetDefaultIMEWnd,
      windows::win32::windows_and_messaging::GetForegroundWindow,
      windows::win32::windows_and_messaging::SendMessageA,
      windows::win32::windows_and_messaging::{LPARAM, WPARAM},
    );
}
