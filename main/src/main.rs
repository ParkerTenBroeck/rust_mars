#![no_std]
#![no_main]

use core::{ffi::CStr, hint::black_box};
pub use rlib::*;

static mut TEST: u32 = 2;

pub fn read_file(buf: &mut [u8]) -> Option<&[u8]>{
    let file = rlib::io::file::File::new_raw(rlib::cstr!("bruh"), arch::FileFlag::Read).ok()?;
    let read = file.read(buf).ok()?;
    if read > buf.len(){
        return None;
    }
    Some(&buf[..read])
}

#[no_mangle]
pub fn main() {
    let mut buf = &mut [0u8; 32];
    if let Some(file) = read_file(buf){
        for b in file{
            rlib::arch::print_char(*b as char);
        }
        rlib::arch::print_char('\n');
    }
}