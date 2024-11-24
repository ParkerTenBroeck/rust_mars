#![no_std]
#![allow(internal_features)]
#![feature(concat_idents)]
#![feature(lang_items)]
#![feature(allow_internal_unstable)]
#![feature(asm_experimental_arch)]
#![feature(naked_functions)]
#![feature(linkage)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(negative_impls)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(tuple_trait)]
#![feature(alloc_error_handler)]

pub mod prelude;

pub mod arch;
pub mod io;
pub mod rt;

pub mod sync;

#[cfg(feature = "alloc")]
extern crate alloc as alloc_crate;

#[cfg(feature = "alloc")]
pub use alloc_crate::*;

pub mod macros;
