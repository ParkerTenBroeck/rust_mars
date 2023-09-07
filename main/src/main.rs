#![no_std]
#![no_main]

use core::ffi::CStr;

pub use rlib::*;

#[no_mangle]
pub fn main() {
    let cstr = unsafe { CStr::from_ptr(b"hello!\0".as_ptr().cast())};
    rlib::arch::print_cstr(cstr);
}
