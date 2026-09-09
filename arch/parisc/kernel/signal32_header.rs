/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *    Copyright (C) 2001 Matthew Wilcox <willy at parisc-linux.org>
 *    Copyright (C) 2003 Carlos O'Donell <carlos at parisc-linux.org>
 */

// Dependency supplied by the surrounding kernel translation.

/* 32-bit ucontext as seen from an 64-bit kernel */
#[repr(C)]
pub struct compat_ucontext {
    pub uc_flags: compat_uint_t,
    pub uc_link: compat_uptr_t,
    pub uc_stack: compat_stack_t, // struct compat_sigaltstack (12 bytes)
    // FIXME: Pad out to get uc_mcontext to start at an 8-byte aligned boundary
    pub pad: [compat_uint_t; 1],
    pub uc_mcontext: compat_sigcontext,
    pub uc_sigmask: compat_sigset_t, // mask last for extensibility
}

/* ELF32 signal handling */

/* In a deft move of uber-hackery, we decide to carry the top half of all
 * 64-bit registers in a non-portable, non-ABI, hidden structure.
 * Userspace can read the hidden structure if it *wants* but is never
 * guaranteed to be in the same place. In fact the uc_sigmask from the
 * ucontext_t structure may push the hidden register file downards
 */
#[repr(C)]
pub struct compat_regfile {
    /* Upper half of all the 64-bit registers that were truncated
       on a copy to a 32-bit userspace */
    pub rf_gr: [compat_int_t; 32],
    pub rf_iasq: [compat_int_t; 2],
    pub rf_iaoq: [compat_int_t; 2],
    pub rf_sar: compat_int_t,
}

#[repr(C)]
pub struct compat_rt_sigframe {
    pub tramp: [u32; 2], // holds original return address
    pub info: compat_siginfo_t,
    pub uc: compat_ucontext,
    /* Hidden location of truncated registers, *must* be last. */
    pub regs: compat_regfile,
}

/*
 * The 32-bit ABI wants at least 48 bytes for a function call frame:
 * 16 bytes for arg0-arg3, and 32 bytes for magic (the only part of
 * which Linux/parisc uses is sp-20 for the saved return pointer...)
 * Then, the stack pointer must be rounded to a cache line (64 bytes).
 */
pub const SIGFRAME32: usize = 64;
pub const FUNCTIONCALLFRAME32: usize = 48;
pub const PARISC_RT_SIGFRAME_SIZE32: usize =
    ((core::mem::size_of::<compat_rt_sigframe>() + FUNCTIONCALLFRAME32) + SIGFRAME32)
        & !SIGFRAME32;

unsafe extern "C" {
    pub fn restore_sigcontext32(
        sc: *mut compat_sigcontext,
        rf: *mut compat_regfile,
        regs: *mut pt_regs,
    ) -> i64;

    pub fn setup_sigcontext32(
        sc: *mut compat_sigcontext,
        rf: *mut compat_regfile,
        regs: *mut pt_regs,
        in_syscall: i32,
    ) -> i64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
