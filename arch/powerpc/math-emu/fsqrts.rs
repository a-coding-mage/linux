// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the kernel soft-fp and math-emu headers are
// intentionally referenced through their corresponding Rust macro interfaces.

pub unsafe fn fsqrts(frD: *mut core::ffi::c_void, frB: *mut core::ffi::c_void) -> i32 {
    FP_DECL_D!(B);
    FP_DECL_D!(R);
    FP_DECL_EX!();

    // #ifdef DEBUG
    #[cfg(feature = "DEBUG")]
    printk!("{}: {:p} {:p} {:p} {:p}\n", "fsqrts", frD, frB);
    // #endif

    FP_UNPACK_DP!(B, frB);

    // #ifdef DEBUG
    #[cfg(feature = "DEBUG")]
    printk!("B: {} {} {} {} ({})\n", B_s, B_f1, B_f0, B_e, B_c);
    // #endif

    if B_s != 0 && B_c != FP_CLS_ZERO {
        FP_SET_EXCEPTION!(EFLAG_VXSQRT);
    }
    if B_c == FP_CLS_NAN {
        FP_SET_EXCEPTION!(EFLAG_VXSNAN);
    }

    FP_SQRT_D!(R, B);

    // #ifdef DEBUG
    #[cfg(feature = "DEBUG")]
    printk!("R: {} {} {} {} ({})\n", R_s, R_f1, R_f0, R_e, R_c);
    // #endif

    __FP_PACK_DS!(frD, R);

    FP_CUR_EXCEPTIONS!()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
