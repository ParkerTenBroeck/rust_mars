#![no_std]
#![no_main]
#![feature(allocator_api)]

extern crate alloc;

use core::{fmt::Write, hint::black_box};

use alloc::{boxed::Box, vec::Vec};
pub use rlib::*;


#[global_allocator]
static ALLOCATOR: ll_alloc::Alloc = ll_alloc::Alloc::new();

pub fn read_file(buf: &mut [u8]) -> Option<&[u8]> {
    use io::file::*;
    let file = File::new_raw(rlib::cstr!("bruh"), FileFlag::Read).ok()?;
    let read = file.read_slice(buf).ok()?;
    {
        let read = file.read_slice(buf).ok()?;
    }
    if read > buf.len() {
        return None;
    }
    Some(&buf[..read])
}

#[no_mangle]
pub fn main() {
    use rlib::io::midi::*;

    rlib::arch::print_cstr(cstr!("hiii!\n"));

    let mut midi = rlib::io::midi::get_midi();
    midi.out_sync(
        Pitch::note(4, Note::C),
        1000,
        InsturmentClass::Piano(Class::C1),
        100,
    );

    let mut buf = [0u8; 55];
    if let Some(file) = read_file(&mut buf) {
        for b in file {
            arch::print_char(*b as char);
        }
        arch::print_char('\n');
    }
}
