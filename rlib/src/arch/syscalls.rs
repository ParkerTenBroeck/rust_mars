pub use super::call_ids::*;

#[inline(always)]
pub fn halt() -> ! {
    unsafe {
        core::arch::asm!{
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
        core::arch::asm!{
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
pub fn sbrk(size: u32) -> &'static mut [u8]{
    unsafe{
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
pub fn print_cstr(str: &core::ffi::CStr){
    unsafe {
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_STRING,
            in("$4") str.as_ptr() as u32,
        )
    }
}

#[inline(always)]
pub fn print_str(str: &str){
    for char in str.chars(){
        print_char(char)
    }
}

pub fn read_stdin(buf: &mut [u8]) -> usize{
    if buf.len() == 0{
        return 0;
    }
    if buf.len() == 1{
        buf[0] = 0;
        return 1;
    }
    let ptr = buf.as_mut_ptr();
    let capacity = buf.len();
    unsafe{
        core::arch::asm!(
            "syscall",
            in("$2") READ_STRING,
            in("$4") ptr,
            in("$5") capacity,
        )
    }
    let mut len = 1;
    for byte in buf{
        if *byte == 0{
            break;
        }
        len += 1;
    }
    len
}

#[inline(always)]
fn read_stdin_char() -> char{
    todo!()
}


#[inline(always)]
pub fn print_char(char: char){
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
    unsafe{
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_FLOAT,
            in("$f12") v1,
        )
    }
}

#[inline(always)]
pub fn print_f64(v1: f64) {
    unsafe{
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_DOUBLE,
            in("$f12") v1,
        )
    }
}

#[inline(always)]
pub fn systime() -> u64{
    let lower: u32;
    let upper: u32;
    unsafe{
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
        unsafe {
            core::arch::asm!(
                "syscall",
                in("$2") PRINT_INTEGER_UNSIGNED,
                in("$4") num,
            )
        }
    }
}

#[inline(always)]
pub fn read_stdin_i32() -> i32 {
    let mut ret = READ_INTEGER;
    unsafe{
        core::arch::asm!{
            "syscall",
            inout("$2") ret
        }
    }
    ret as i32
}

#[inline(always)]
pub fn read_stdin_f32() -> f32 {
    let out;
    unsafe{
        core::arch::asm!{
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
    unsafe{
        core::arch::asm!{
            "syscall",
            in("$2") READ_DOUBLE,
            out("$f0") out
        }
    }
    out
}

#[inline(always)]
pub fn sleep_ms(arg: i32) {
    unsafe{
        core::arch::asm!{
            "syscall",
            in("$2") SLEEP_MS,
            in("$4") arg
        }
    }
}

#[inline(always)]
pub fn stack_pointer() -> u32 {
    let sp;
    unsafe{
        core::arch::asm!{
            "addi {sp}, $29, 0",
            sp = out(reg) sp,
        }
    }
    sp
}

#[inline(always)]
pub fn data_seg_end() -> u32 {
    let out;
    unsafe{
        core::arch::asm!{
            "la {out}, _data_end",
            out = out(reg) out,
        }
    }
    out
}