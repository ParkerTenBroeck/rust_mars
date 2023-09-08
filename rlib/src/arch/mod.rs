mod call_ids;
mod syscalls;
pub use syscalls::*;


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