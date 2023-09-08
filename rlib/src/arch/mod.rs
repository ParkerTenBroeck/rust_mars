mod call_ids;
mod syscalls;
pub use call_ids::*;
pub use syscalls::*;

#[inline(always)]
pub fn halt() -> ! {
    unsafe {
        syscall_v_v::<TERMINATE_EXEC>();
    }
    unsafe {
        core::hint::unreachable_unchecked();
    }
}

#[inline(always)]
pub fn print_i32(num: i32) {
    unsafe {
        syscall_s_v::<PRINT_INTEGER>(num as u32);
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

// #[inline(always)]
pub fn print_cstr(str: &core::ffi::CStr){
    unsafe{
        syscall_s_v::<PRINT_STRING>(str.as_ptr() as u32)
    }
}

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
pub fn print_char(char: char){
    unsafe{
        syscall_s_v::<PRINT_CHARACTER>(char as u32)
    }
}

pub fn print_f32(v1: f32) {
    unsafe{
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_FLOAT,
            in("$f12") v1,
        )
    }
}

pub fn print_f64(v1: f64) {
    unsafe{
        core::arch::asm!(
            "syscall",
            in("$2") PRINT_DOUBLE,
            in("$f12") v1,
        )
    }
}

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

pub fn print_i32_hex(num: u32) {
    unsafe {
        syscall_s_v::<PRINT_INTEGER_HEX>(num);
    }
}

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

// #[inline(always)]
// pub fn print_zero_term_str(str: &str) {
//     unsafe {
//         syscall_s_v::<GET_INSTRUCTIONS_RAN>(str.as_ptr().addr() as u32);
//     }
// }

// #[inline(always)]
// pub fn print_str(str: &str) {
//     unsafe { syscall_ss_v::<PRINT_STR>(str.as_ptr() as u32, str.len() as u32) }
// }



// #[inline(always)]
// pub fn print_char(char: char) {
//     unsafe {
//         syscall_s_v::<PRINT_CHARACTER>(char as u32);
//     }
// }

// #[inline(always)]
// pub fn sleep_ms(ms: u32) {
//     unsafe {
//         syscall_s_v::<SLEEP_MS>(ms);
//     }
// }

// #[inline(always)]
// pub fn sleep_d_ms(ms: u32) {
//     unsafe {
//         syscall_s_v::<SLEEP_D_MS>(ms);
//     }
// }

// #[inline(always)]
// pub fn current_time_nanos() -> u64 {
//     unsafe { syscall_v_d::<CURRENT_TIME_NANOS>() }
// }

// pub fn is_key_pressed(char: char) -> bool {
//     unsafe { syscall_s_s::<IS_KEY_PRESSED>(char as u32) != 0 }
// }

// // #[inline(always)]
// // pub fn read_i32() -> i32 {
// //     unsafe { syscall_0_1::<5>() as i32 }
// // }

// pub fn rand_range(min: i32, max: i32) -> i32 {
//     unsafe { syscall_ss_s::<GENERATE_THREAD_RANDOM_NUMBER>(min as u32, max as u32) as i32 }
// }

// #[inline(always)]
// pub fn sleep_delta_mills(mills: u32) {
//     unsafe {
//         syscall_1_0::<106>(mills);
//     }
// }

// #[inline(always)]
// pub fn sleep_mills(mills: u32) {
//     unsafe {
//         syscall_1_0::<105>(mills);
//     }
// }

// #[inline(always)]
// pub fn get_micros() -> u64 {
//     unsafe { syscall_0_2_s::<108>() }
// }

// #[inline(always)]
// pub fn get_nanos() -> u64 {
//     unsafe { syscall_0_2_s::<109>() }
// }
