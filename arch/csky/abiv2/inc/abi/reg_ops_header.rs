/* SPDX-License-Identifier: GPL-2.0 */

// Dependency corresponding to <asm/reg_ops.h>.
extern "C" {
    fn mfcr(reg: *const core::ffi::c_char) -> u32;
}

pub unsafe fn mfcr_hint() -> u32 {
    mfcr(b"cr31\0".as_ptr() as *const core::ffi::c_char)
}

pub unsafe fn mfcr_ccr2() -> u32 {
    mfcr(b"cr23\0".as_ptr() as *const core::ffi::c_char)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
