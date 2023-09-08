use core::mem::MaybeUninit;

#[no_mangle]
#[naked]
#[link_section = ".text.start"]
extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm! {
            ".set noat",
            "la $gp, _gp",
            "la $sp, _sp ",
            "move $fp, $sp",
            "jal main",
            "1:",
            "li $2, 10",
            "syscall",
            "b 1b",
            options(noreturn),
        }
    }
}

#[inline(always)]
/// # Safety
/// this is the start of the heap dont touch it if you arent the global allocator ;)
pub unsafe fn heap_address() -> *mut u8 {
    let ret;
    core::arch::asm!(
        ".set noat",
        "la {0}, _heap",
        out(reg) ret
    );
    ret
}

pub fn str_to_cstr<R>(str: &str, usage: impl FnOnce(&core::ffi::CStr) -> R) -> R {
    stackalloc::stackalloc_uninit::<u8, _, _>(str.len() + 1, |v| {
        let v: &mut [u8] = unsafe {
            str.as_ptr().copy_to(v.as_mut_ptr().cast(), str.len());
            v.as_mut_ptr().add(str.len()).write(MaybeUninit::new(0));
            core::mem::transmute(v)
        };
        let cstr = unsafe { core::ffi::CStr::from_ptr(v.as_ptr().cast()) };
        usage(cstr)
    })
}

#[panic_handler]
#[cfg(feature = "provide_panic_handler")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::println!("PANIC AT THE DISCO: {:#?}", info);
    loop {
        crate::arch::halt();
    }
}
