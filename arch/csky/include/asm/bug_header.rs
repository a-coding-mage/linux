/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header __ASM_CSKY_BUG_H.
// The Linux compiler, const, types, and asm-generic bug headers provide
// dependencies and conditional context for this declaration-only header.

#[macro_export]
macro_rules! BUG {
    () => {{
        unsafe {
            core::arch::asm!("bkpt\n");
        }
        core::unreachable!();
    }};
}

// #define HAVE_ARCH_BUG
pub const HAVE_ARCH_BUG: bool = true;

// Contents supplied by <asm-generic/bug.h> are external to this translation.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn die(regs: *mut pt_regs, str_: *const core::ffi::c_char);
    pub fn do_trap(
        regs: *mut pt_regs,
        signo: core::ffi::c_int,
        code: core::ffi::c_int,
        addr: core::ffi::c_ulong,
    );

    pub fn show_regs(regs: *mut pt_regs);
    pub fn show_code(regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
