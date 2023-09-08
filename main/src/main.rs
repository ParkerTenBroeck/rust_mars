#![no_std]
#![no_main]

use core::{ffi::CStr, hint::black_box};
pub use rlib::*;

static mut TEST: u32 = 2;

#[no_mangle]
pub fn main() {

    // let res = rlib::rt::str_to_cstr("test string!\n", |cstr|{
    //     rlib::arch::input_dialog_int(cstr)
    // });
    // println!("{:#?}", res);

    rlib::arch::message_dialog(arch::MessageKind::Error, rlib::cstr!("Error :O"));
    rlib::arch::message_dialog(arch::MessageKind::Warning, rlib::cstr!("Warning >:)"));
    rlib::arch::message_dialog(arch::MessageKind::Information, rlib::cstr!("UwU"));
    rlib::arch::message_dialog(arch::MessageKind::Question, rlib::cstr!(">.<"));
    rlib::arch::message_dialog(arch::MessageKind::Plain, rlib::cstr!("'*'"));

    if true{
        rlib::arch::halt_code(5);
    }

    let val = rlib::arch::read_stdin_i32();
    if val <= 34{
        println!("{}", factorial(val as u128));
    }else{
        println!("Number too large! >.<");
    }
    rlib::arch::sleep_ms(3000);

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


    // // rand_fn();
    // // rand_fn();
    // // test();
    // let mem = rlib::arch::sbrk(10);
    // println!("{:?}", mem.as_ptr());

    // // let mem = &mut [0u8; 32];
    // let sp = rlib::arch::stack_pointer();
    // rlib::arch::print_u32_hex(sp);
    // rlib::arch::print_char('\n');

    // let sp = rlib::arch::data_seg_end();
    // rlib::arch::print_u32_hex(sp);
    // rlib::arch::print_char('\n');

    // for _ in 0..1000{
    //     let mem = rlib::arch::sbrk(unsafe{10});
    //     println!("{:?}, {}, {:?}", mem.as_ptr(), mem.len(), mem);
    // }
    // // println!("{:#?}", 4.0f32);
    // // // rlib::arch::print_i32(mem as i32);
    // // let mem = rlib::arch::sbrk(32);
    // // rlib::arch::print_i32(mem as i32);
    // let mem = &mut [0u8; 32];
    // println!("{:?}", mem);

    let v1 = black_box(12.0);
    rlib::arch::print_f32(55.0 * v1);
    rlib::arch::print_char('\n');
    rlib::arch::print_f64(55.0 * (v1 as f64));
    rlib::arch::print_char('\n');
    rlib::arch::print_cstr(rlib::cstr!("Hello\n"));
}

pub fn factorial(val: u128) -> u128{
    (2..=val).product()
}

// #[inline(never)]
// fn test(){
//     let test_v = black_box(32);
//     let sp = rlib::arch::stack_pointer();
//     rlib::arch::print_u32_hex(sp);
//     rlib::arch::print_char('\n');
//     test();
//     rlib::arch::print_i32(test_v);
// }

// fn rand_fn(val: impl core::fmt::Debug){
//     // let mem = rlib::arch::sbrk(10);
//     println!("{:?}", val);
// }