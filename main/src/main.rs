#![no_std]
#![no_main]

pub use rlib::*;

struct StaticBuf<'a>{
    buf: &'a mut [u8],
    written: usize,
}

impl<'a> StaticBuf<'a>{
    pub fn new(buf: &'a mut [u8]) -> Self{
        Self { buf, written: 0 }
    }
}

impl<'a> core::fmt::Write for StaticBuf<'a>{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.buf.len() - self.written > s.len(){
            let next = self.written+s.len();
            self.buf[self.written..next].copy_from_slice(s.as_bytes());
            self.written = next;
            Ok(())
        }else{
            Err(core::fmt::Error)
        }
    }
}

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
    {
        for _ in 0..5000{
            println!("{}", unsafe{arch::next_rand_f64(1)});
        }
        return;
    }
    let mut buf = [0u8; 32];
    if let Some(file) = read_file(&mut buf) {
        for b in file {
            arch::print_char(*b as char);
        }
        arch::print_char('\n');
    }
}
