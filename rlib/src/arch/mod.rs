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
pub fn print_cstr(str: &core::ffi::CStr){
    unsafe{
        syscall_s_v::<PRINT_STRING>(str.as_ptr() as u32)
    }
}

#[inline(always)]
pub fn print_char(char: char){
    unsafe{
        syscall_s_v::<PRINT_CHARACTER>(char as u32)
    }
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
