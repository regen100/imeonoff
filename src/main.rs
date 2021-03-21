extern crate getopts;
use getopts::Options;
use std::env;
use std::option::Option;

mod bindings {
    ::windows::include_bindings!();
}

use bindings::{
    windows::win32::intl::ImmGetDefaultIMEWnd,
    windows::win32::windows_and_messaging::GetForegroundWindow,
    windows::win32::windows_and_messaging::SendMessageA,
    windows::win32::windows_and_messaging::{LPARAM, WPARAM},
};

const WM_IME_CONTROL: u32 = 0x283;
const IMC_GETOPENSTATUS: WPARAM = WPARAM(0x0005);
const IMC_SETOPENSTATUS: WPARAM = WPARAM(0x0006);

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("s", "set", "set IME status", "STATUS");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!(f.to_string())
        }
    };
    let set: Option<isize> = matches.opt_str("s").map(|x| x.parse().unwrap());

    unsafe {
        let ime_wnd = ImmGetDefaultIMEWnd(GetForegroundWindow());
        let ime_before_status = SendMessageA(ime_wnd, WM_IME_CONTROL, IMC_GETOPENSTATUS, LPARAM(0));
        println!("{}", ime_before_status.0);
        if let Some(set) = set {
            SendMessageA(ime_wnd, WM_IME_CONTROL, IMC_SETOPENSTATUS, LPARAM(set));
        }
    }
}
