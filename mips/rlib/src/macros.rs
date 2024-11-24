#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let mut out = $crate::io::stdout();
        core::fmt::Write::write_fmt(&mut out, core::format_args!($($arg)*)).expect("Cant write?");
        drop(out);
    }};
}
#[macro_export]
#[allow_internal_unstable(format_args_nl)]
macro_rules! println {
    () => {{
        println!("\n")
    }};
    ($($arg:tt)*) => {{
        let mut out = $crate::io::stdout();
        core::fmt::Write::write_fmt(&mut out, core::format_args_nl!($($arg)*)).expect("Cant write?");
        drop(out);
    }};
}

#[macro_export]
macro_rules! cstr {
    ($data:literal) => {
        unsafe {
            core::ffi::CStr::from_ptr(&concat!($data, "\0").as_bytes()[0] as *const u8 as *const i8)
        }
    };
}
