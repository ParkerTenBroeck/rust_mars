use core::ops::{Range, RangeInclusive};

pub use super::call_ids::*;

#[inline(always)]
pub fn halt() -> ! {
    unsafe {
        core::arch::asm! {
            "syscall",
            in("$2") TERMINATE_EXEC
        }
    }
    unsafe {
        core::hint::unreachable_unchecked();
    }
}

#[inline(always)]
pub fn halt_code(code: i32) -> ! {
    unsafe {
        core::arch::asm! {
            "syscall",
            in("$2") TERMINATE_EXEC_VAL,
            in("$4") code
        }
    }
    unsafe {
        core::hint::unreachable_unchecked();
    }
}

#[inline(always)]
pub fn print_i32(num: i32) {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_INTEGER,
            in("$4") num,
        )
    }
}

#[inline(always)]
pub fn sbrk(size: u32) -> &'static mut [u8] {
    unsafe {
        let mut ret1 = ALLOCATE_HEAP;
        core::arch::asm!(
            "syscall",
            inout("$2") ret1,
            in("$4") size,
        );
        let ptr: *mut u8 = core::mem::transmute(ret1);
        core::slice::from_raw_parts_mut(ptr, size as usize)
    }
}

#[inline(always)]
pub fn print_cstr(str: &core::ffi::CStr) {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_STRING,
            in("$4") str.as_ptr() as u32,
        )
    }
}

#[inline(always)]
pub fn print_str(str: &str) {
    for char in str.chars() {
        print_char(char)
    }
}

pub fn read_stdin(buf: &mut [u8]) -> usize {
    if buf.len() == 0 {
        return 0;
    }
    if buf.len() == 1 {
        buf[0] = 0;
        return 1;
    }
    let ptr = buf.as_mut_ptr();
    let capacity = buf.len();
    unsafe {
        core::arch::asm!(
            "syscall",
            in("$2") READ_STRING,
            in("$4") ptr,
            in("$5") capacity,
        )
    }
    let mut len = 1;
    for byte in buf {
        if *byte == 0 {
            break;
        }
        len += 1;
    }
    len
}

#[inline(always)]
pub fn read_stdin_char() -> char {
    todo!()
}

#[inline(always)]
pub fn print_char(char: char) {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_CHARACTER,
            in("$4") char as u32,
        )
    }
}

#[inline(always)]
pub fn print_f32(v1: f32) {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_FLOAT,
            in("$f12") v1,
        )
    }
}

#[inline(always)]
pub fn print_f64(v1: f64) {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_DOUBLE,
            in("$f12") v1,
        )
    }
}

#[inline(always)]
pub fn systime() -> u64 {
    let lower: u32;
    let upper: u32;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("$2") SYSTEM_TIME_MS,
            out("$4") lower,
            out("$5") upper,
        )
    }
    lower as u64 | ((upper as u64) << 32)
}

#[inline(always)]
pub fn print_u32_hex(num: u32) {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_INTEGER_HEX,
            in("$4") num,
        )
    }
}

#[inline(always)]
pub fn print_u32_bin(num: u32) {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_INTEGER_BIN,
            in("$4") num,
        )
    }
}

#[inline(always)]
pub fn print_u32(num: u32) {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_INTEGER_UNSIGNED,
            in("$4") num,
        )
    }
}

#[inline(always)]
pub fn read_stdin_i32() -> i32 {
    let mut ret = READ_INTEGER;
    unsafe {
        core::arch::asm! {
            "syscall",
            inout("$2") ret
        }
    }
    ret as i32
}

#[inline(always)]
pub fn read_stdin_f32() -> f32 {
    let out;
    unsafe {
        core::arch::asm! {
            "syscall",
            in("$2") READ_FLOAT,
            out("$f0") out
        }
    }
    out
}

#[inline(always)]
pub fn read_stdin_f64() -> f64 {
    let out;
    unsafe {
        core::arch::asm! {
            "syscall",
            in("$2") READ_DOUBLE,
            out("$f0") out
        }
    }
    out
}

#[inline(always)]
pub fn sleep_ms(arg: i32) {
    unsafe {
        core::arch::asm! {
            "syscall",
            in("$2") SLEEP_MS,
            in("$4") arg
        }
    }
}

#[inline]
pub unsafe fn set_seed(rand_id: u32, seed: u32) {
    core::arch::asm!{
        "syscall",
        in("$2") SET_RAND_SEED,
        in("$4") rand_id,
        in("$5") seed
    }
}

#[inline]
pub unsafe fn next_rand_int(rand_id: u32) -> u32 {
    let out;
    core::arch::asm!{
        "syscall",
        in("$2") GET_RAND_INT,
        inout("$4") rand_id => out
    }
    out
}

#[inline]
pub unsafe fn next_rand_int_range(rand_id: u32, range: Range<i32>) -> i32 {
    let out: i32;
    let calc_bound = range.end - range.start;
    core::arch::asm!{
        "syscall",
        in("$2") GET_RAND_INT_RANGE,
        inout("$4") rand_id => out,
        in("$5") calc_bound,
    }
    out + range.start
}

#[inline]
pub unsafe fn next_rand_int_range_inclusive(rand_id: u32, range: RangeInclusive<i32>) -> i32 {
    let out: i32;
    let calc_bound = range.end() - range.start() + 1;
    core::arch::asm!{
        "syscall",
        in("$2") GET_RAND_INT_RANGE,
        inout("$4") rand_id => out,
        in("$5") calc_bound,
    }
    out + range.start()
}

#[inline]
pub unsafe fn next_rand_f32(rand_id: u32) -> f32 {
    let out;
    core::arch::asm!{
        "syscall",
        in("$2") GET_RAND_FLOAT,
        in("$4") rand_id,
        out("$f0") out
    }
    out
}

#[inline]
pub unsafe fn next_rand_f64(rand_id: u32) -> f64 {
    let out;
    core::arch::asm!{
        "syscall",
        in("$2") GET_RAND_DOUBLE,
        in("$4") rand_id,
        out("$f0") out
    }
    out
}

#[derive(Debug)]
pub enum ConfirmDialogResponse {
    Yes,
    No,
    Cancel,
}
#[inline(always)]
pub fn configm_dialog(message: &core::ffi::CStr) -> ConfirmDialogResponse {
    unsafe {
        let out: i32;
        core::arch::asm! {
            "syscall",
            in("$2") CONFIRM_DIALOG,
            inout("$4") message.as_ptr() => out
        };

        match out {
            0 => ConfirmDialogResponse::Yes,
            1 => ConfirmDialogResponse::No,
            2 => ConfirmDialogResponse::Cancel,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum InputDialogResponse<T> {
    Ok(T),
    InputCannotBeParsed,
    Canceled,
    OkButNoData,
    LengthOfInputExceededBuffer,
}
#[inline(always)]
pub fn input_dialog_int(message: &core::ffi::CStr) -> InputDialogResponse<i32> {
    unsafe {
        let value: i32;
        let discriminant: i32;
        core::arch::asm! {
            "syscall",
            in("$2") INPUT_DIALOG_INT,
            inout("$4") message.as_ptr() => value,
            out("$5") discriminant
        };

        match discriminant {
            0 => InputDialogResponse::Ok(value),
            -1 => InputDialogResponse::InputCannotBeParsed,
            -2 => InputDialogResponse::Canceled,
            -3 => InputDialogResponse::OkButNoData,
            _ => unreachable!(),
        }
    }
}

#[inline(always)]
pub fn input_dialog_f32(message: &core::ffi::CStr) -> InputDialogResponse<f32> {
    unsafe {
        let value: f32;
        let discriminant: i32;
        core::arch::asm! {
            "syscall",
            in("$2") INPUT_DIALOG_FLOAT,
            in("$4") message.as_ptr(),
            out("$f0") value,
            out("$5") discriminant
        };

        match discriminant {
            0 => InputDialogResponse::Ok(value),
            -1 => InputDialogResponse::InputCannotBeParsed,
            -2 => InputDialogResponse::Canceled,
            -3 => InputDialogResponse::OkButNoData,
            _ => unreachable!(),
        }
    }
}

#[inline(always)]
pub fn input_dialog_f64(message: &core::ffi::CStr) -> InputDialogResponse<f64> {
    unsafe {
        let value: f64;
        let discriminant: i32;
        core::arch::asm! {
            "syscall",
            in("$2") INPUT_DIALOG_DOUBLE,
            in("$4") message.as_ptr(),
            out("$f0") value,
            out("$5") discriminant
        };

        match discriminant {
            0 => InputDialogResponse::Ok(value),
            -1 => InputDialogResponse::InputCannotBeParsed,
            -2 => InputDialogResponse::Canceled,
            -3 => InputDialogResponse::OkButNoData,
            _ => unreachable!(),
        }
    }
}

#[inline(always)]
pub fn input_dialog_str(message: &core::ffi::CStr, buf: &mut [u8]) -> InputDialogResponse<usize> {
    let discriminant: i32;
    unsafe {
        core::arch::asm! {
            "syscall",
            in("$2") INPUT_DIALOG_STRING,
            inout("$4") message.as_ptr() => discriminant,
            in("$5") buf.as_ptr(),
            in("$6") buf.len()
        };
    }

    match discriminant {
        0 => {
            let mut len = 1;
            for byte in buf {
                if *byte == 0 {
                    break;
                }
                len += 1;
            }
            InputDialogResponse::Ok(len)
        }
        -1 => InputDialogResponse::InputCannotBeParsed,
        -2 => InputDialogResponse::Canceled,
        -3 => InputDialogResponse::OkButNoData,
        -4 => InputDialogResponse::LengthOfInputExceededBuffer,
        _ => unreachable!(),
    }
}

pub enum MessageKind {
    Error = 0,
    Information = 1,
    Warning = 2,
    Question = 3,
    Plain = 4,
}

#[inline(always)]
pub fn message_dialog(kind: MessageKind, message: &core::ffi::CStr) {
    unsafe {
        core::arch::asm! {
            "syscall",
            in("$2") MESSAGE_DIALOG,
            in("$4") message.as_ptr(),
            in("$5") kind as u32
        }
    }
}

pub const STDIN: FileDesciptor = FileDesciptor(0);
pub const STDOUT: FileDesciptor = FileDesciptor(1);
pub const STDERR: FileDesciptor = FileDesciptor(2);

#[derive(Debug, Clone, Copy)]
pub struct FileDesciptor(u32);

#[derive(Debug)]
pub struct FileIOErrorCode(i32);

#[derive(Debug, Clone, Copy)]
pub enum FileFlag {
    Read = 0,
    Write = 1,
    WriteOnlyWithCreateAppend = 9,
}

#[inline(always)]
pub unsafe fn open_file(
    file: &core::ffi::CStr,
    flag: FileFlag,
) -> Result<FileDesciptor, FileIOErrorCode> {
    let res: i32;
    unsafe {
        core::arch::asm! {
            "syscall",
            inout("$2") OPEN_FILE => res,
            in("$4") file.as_ptr(),
            in("$5") flag as u32,
            in("$6") 0
        }
    }
    if res < 0 {
        Err(FileIOErrorCode(res))
    } else {
        Ok(FileDesciptor(res as u32))
    }
}

#[inline(always)]
pub unsafe fn read_file(file: &FileDesciptor, buf: &mut [u8]) -> Result<usize, FileIOErrorCode> {
    let res: i32;
    unsafe {
        core::arch::asm! {
            "syscall",
            inout("$2") READ_FROM_FILE => res,
            in("$4") file.0,
            in("$5") buf.as_ptr(),
            in("$6") buf.len()
        }
    }
    if res < 0 {
        Err(FileIOErrorCode(res))
    } else {
        Ok(res as usize)
    }
}

#[inline(always)]
pub unsafe fn write_file(file: &mut FileDesciptor, buf: &[u8]) -> Result<usize, FileIOErrorCode> {
    let res: i32;
    unsafe {
        core::arch::asm! {
            "syscall",
            inout("$2") WRITE_TO_FILE => res,
            in("$4") file.0,
            in("$5") buf.as_ptr(),
            in("$6") buf.len()
        }
    }
    if res < 0 {
        Err(FileIOErrorCode(res))
    } else {
        Ok(res as usize)
    }
}

#[inline(always)]
pub unsafe fn close_file(file: FileDesciptor) {
    unsafe {
        core::arch::asm! {
            "syscall",
            in("$2") CLOSE_FILE,
            in("$4") file.0
        }
    }
}
