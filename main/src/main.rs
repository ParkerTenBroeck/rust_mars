#![no_std]
#![no_main]

use core::ffi::CStr;
pub use rlib::*;

#[no_mangle]
pub fn main() {
    for c in 'a'..='z'{
        rlib::arch::print_char(c);
    }
    println!("test: {}", 12);
    let cstr = unsafe { CStr::from_ptr(b"hello!\0".as_ptr().cast())};
    rlib::arch::print_cstr(cstr);
}
