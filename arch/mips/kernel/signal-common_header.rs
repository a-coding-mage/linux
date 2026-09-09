/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1991, 1992  Linus Torvalds
 * Copyright (C) 1994 - 2000  Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 */

use core::ffi::c_void;

/* The C header guard is omitted in Rust; module inclusion supplies this role. */
/* #define DEBUG_SIG */

#[cfg(DEBUG_SIG)]
macro_rules! DEBUGP {
    ($fmt:literal $(, $args:expr)*) => {
        printk!(concat!("{}: ", $fmt), module_path!() $(, $args)*);
    };
}

#[cfg(not(DEBUG_SIG))]
macro_rules! DEBUGP {
    ($($args:tt)*) => {};
}

/*
 * Determine which stack to use..
 */
extern "C" {
    pub fn get_sigframe(
        ksig: *mut ksignal,
        regs: *mut pt_regs,
        frame_size: usize,
    ) -> *mut c_void;

    /* Check and clear pending FPU exceptions in saved CSR */
    pub fn fpcsr_pending(fpcsr: *mut u32) -> i32;

    /* Assembly functions to move context to/from the FPU */
    pub fn _save_fp_context(fpregs: *mut c_void, csr: *mut c_void) -> i32;
    pub fn _restore_fp_context(fpregs: *mut c_void, csr: *mut c_void) -> i32;

    pub fn _save_msa_all_upper(buf: *mut c_void) -> i32;
    pub fn _restore_msa_all_upper(buf: *mut c_void) -> i32;

    pub fn setup_sigcontext(regs: *mut pt_regs, sc: *mut sigcontext) -> i32;
    pub fn restore_sigcontext(regs: *mut pt_regs, sc: *mut sigcontext) -> i32;
}

/* Make sure we will not lose FPU ownership */
#[inline]
pub unsafe fn lock_fpu_owner() {
    preempt_disable();
    pagefault_disable();
}

#[inline]
pub unsafe fn unlock_fpu_owner() {
    pagefault_enable();
    preempt_enable();
}

/* Symbols supplied by the surrounding kernel translation. */
extern "C" {
    fn preempt_disable();
    fn pagefault_disable();
    fn pagefault_enable();
    fn preempt_enable();
}

/* Opaque types supplied by dependent kernel headers. */
#[repr(C)]
pub struct ksignal {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigcontext {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
