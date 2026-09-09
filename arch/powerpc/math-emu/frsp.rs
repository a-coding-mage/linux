// SPDX-License-Identifier: GPL-2.0
//
// The declarations and floating-point operations below are provided by the
// corresponding kernel soft-fp dependencies.

use core::ffi::c_void;

pub unsafe fn frsp(frD: *mut c_void, frB: *mut c_void) -> i32 {
    FP_DECL_D!(B);
    FP_DECL_EX!();

    // C build-time DEBUG conditional:
    // printk("%s: D %p, B %p\n", __func__, frD, frB);

    FP_UNPACK_DP!(B, frB);

    // C build-time DEBUG conditional:
    // printk("B: %ld %lu %lu %ld (%ld)\n", B_s, B_f1, B_f0, B_e, B_c);

    __FP_PACK_DS!(frD, B);

    FP_CUR_EXCEPTIONS!()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
