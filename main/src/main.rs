#![no_std]
#![no_main]

use core::{ffi::CStr, hint::black_box};
pub use rlib::*;

static mut TEST: u32 = 2;

#[no_mangle]
pub fn main() {
    rlib::arch::print_str("asdasdasd\n");
    for c in 'a'..='z'{
        rlib::arch::print_char(c);
    }

    unsafe{TEST = black_box(10);}

    for _ in 0..500{
        let val = rlib::arch::systime();
        unsafe{TEST = val as u32;};

        if unsafe{TEST == 55}{
            break;
        }
        println!("{}", val);
        // println!("{:?}", rlib::arch::systime());
        // rand_fn(val);
        // rlib::arch::print_i32_hex((val >> 32) as u32);
        // rlib::arch::print_i32_hex(val as u32);
        // rlib::arch::print_char('\n');
        // println!("{}", rlib::arch::systime());
    }


    // rand_fn();
    // rand_fn();
    // test();
    // let mem = rlib::arch::sbrk(10);
    // println!("{:?}", mem.as_ptr());

    // let mem = rlib::arch::sbrk(unsafe{TEST});
    let mem = &mut [0u8; 32];
    println!("{:?}", mem.as_ptr());
    // // rlib::arch::print_i32(mem as i32);
    // let mem = rlib::arch::sbrk(32);
    // rlib::arch::print_i32(mem as i32);
    let mem = &mut [0u8; 32];
    println!("{:?}", mem);

    let v1 = black_box(12.0);
    rlib::arch::print_f32(55.0 * v1);
    rlib::arch::print_char('\n');
    rlib::arch::print_f64(55.0 * (v1 as f64));
    rlib::arch::print_char('\n');
    rlib::arch::print_cstr(rlib::cstr!("Hello\n"));
}

#[inline(never)]
fn test(){
    let test_v = black_box(32);
    let sp = rlib::arch::stack_pointer();
    rlib::arch::print_i32_hex(sp);
    rlib::arch::print_char('\n');
    test();
    rlib::arch::print_i32(test_v);
}

fn rand_fn(val: impl core::fmt::Debug){
    // let mem = rlib::arch::sbrk(10);
    println!("{:?}", val);
}