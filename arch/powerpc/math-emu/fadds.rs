// SPDX-License-Identifier: GPL-2.0
//
// Dependencies corresponding to the C headers are supplied by the surrounding
// math-emulation implementation.

pub unsafe fn fadds(fr_d: *mut core::ffi::c_void,
                    fr_a: *mut core::ffi::c_void,
                    fr_b: *mut core::ffi::c_void) -> i32 {
    let mut a = FP_DECL_D!(A);
    let mut b = FP_DECL_D!(B);
    let mut r = FP_DECL_D!(R);
    let mut ex = FP_DECL_EX!();

    #[cfg(feature = "DEBUG")]
    printk!("%s: %p %p %p\n", __func__, fr_d, fr_a, fr_b);

    FP_UNPACK_DP!(a, fr_a);
    FP_UNPACK_DP!(b, fr_b);

    #[cfg(feature = "DEBUG")]
    printk!("A: %ld %lu %lu %ld (%ld)\n", A_s, A_f1, A_f0, A_e, A_c);
    #[cfg(feature = "DEBUG")]
    printk!("B: %ld %lu %lu %ld (%ld)\n", B_s, B_f1, B_f0, B_e, B_c);

    FP_ADD_D!(r, a, b);

    #[cfg(feature = "DEBUG")]
    printk!("D: %ld %lu %lu %ld (%ld)\n", R_s, R_f1, R_f0, R_e, R_c);

    __FP_PACK_DS!(fr_d, r);

    FP_CUR_EXCEPTIONS!()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
