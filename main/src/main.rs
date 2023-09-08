#![no_std]
#![no_main]

pub use rlib::*;

pub fn read_file(buf: &mut [u8]) -> Option<&[u8]> {
    use io::file::*;
    let file = File::new_raw(rlib::cstr!("bruh"), FileFlag::Read).ok()?;
    let read = file.read(buf).ok()?;
    if read > buf.len() {
        return None;
    }
    Some(&buf[..read])
}

#[no_mangle]
pub fn main() {
    let mut buf = [0u8; 32];
    if let Some(file) = read_file(&mut buf) {
        for b in file {
            arch::print_char(*b as char);
        }
        arch::print_char('\n');
    }
}
