extern crate getopts;
use getopts::Options;
use std::env;
use std::option::Option;

mod bindings {
    ::windows::include_bindings!();
}

use bindings::{
    Windows::Win32::Intl::ImmGetDefaultIMEWnd,
    Windows::Win32::WindowsAndMessaging::GetForegroundWindow,
    Windows::Win32::WindowsAndMessaging::SendMessageA,
    Windows::Win32::WindowsAndMessaging::{LPARAM, WPARAM},
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
            eprintln!("{}", f.to_string());
            std::process::exit(1);
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
