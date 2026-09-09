/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 1999, 2024
 */

// Translated from asm/access-regs.h.
// The original header includes linux/instrumented.h and asm/sigcontext.h;
// their declarations are supplied by the surrounding translation unit.

#[repr(C)]
pub struct access_regs {
    pub regs: [core::ffi::c_uint; NUM_ACRS],
}

extern "C" {
    fn instrument_write(ptr: *const core::ffi::c_void, size: usize);
    fn instrument_read(ptr: *const core::ffi::c_void, size: usize);
}

#[inline]
pub unsafe fn save_access_regs(acrs: *mut core::ffi::c_uint) {
    let regs = acrs as *mut access_regs;

    instrument_write(
        regs as *const core::ffi::c_void,
        core::mem::size_of::<access_regs>(),
    );
    core::arch::asm!(
        "stamy 0,15,0({regs})",
        regs = in(reg) regs,
        options(nostack, preserves_flags),
    );
}

#[inline]
pub unsafe fn restore_access_regs(acrs: *mut core::ffi::c_uint) {
    let regs = acrs as *mut access_regs;

    instrument_read(
        regs as *const core::ffi::c_void,
        core::mem::size_of::<access_regs>(),
    );
    core::arch::asm!(
        "lamy 0,15,0({regs})",
        regs = in(reg) regs,
        options(nostack, preserves_flags),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
