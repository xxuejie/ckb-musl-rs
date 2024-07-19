#![cfg_attr(target_arch = "riscv64", no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod tests;

#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

use core::ffi::c_char;

extern crate ckb_musl_rs;

extern "C" {
    fn call_to_c(a: *const c_char);
}

// ckb_musl_rs::use_brk_via_growing_elf_end!();
// ckb_musl_rs::use_brk_via_bss_value!(30 * 1024);
// ckb_musl_rs::disable_brk_and_mmap!();
// ckb_musl_rs::disable_stdout_to_ckb_debug!();

pub fn program_entry() -> i8 {
    ckb_musl_rs::init!();

    ckb_std::debug!("This is a sample contract!");

    let args = ckb_std::env::argv();
    let s = if args.len() > 0 { &*args[0] } else { c"42" };
    unsafe { call_to_c(s.as_ptr()) };

    0
}
