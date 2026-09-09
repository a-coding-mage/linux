// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the kernel soft-fp environment are intentionally
// left as external Rust macros and symbols.

pub unsafe fn fmsub(
    frD: *mut core::ffi::c_void,
    frA: *mut core::ffi::c_void,
    frB: *mut core::ffi::c_void,
    frC: *mut core::ffi::c_void,
) -> i32 {
    FP_DECL_D!(R);
    FP_DECL_D!(A);
    FP_DECL_D!(B);
    FP_DECL_D!(C);
    FP_DECL_D!(T);
    FP_DECL_EX!();

    // Corresponds to the C build-time DEBUG conditional.
    #[cfg(feature = "DEBUG")]
    {
        printk!("{}: {:p} {:p} {:p} {:p}\n", "fmsub", frD, frA, frB, frC);
    }

    FP_UNPACK_DP!(A, frA);
    FP_UNPACK_DP!(B, frB);
    FP_UNPACK_DP!(C, frC);

    #[cfg(feature = "DEBUG")]
    {
        printk!("A: {} {} {} {} ({})\n", A_s, A_f1, A_f0, A_e, A_c);
        printk!("B: {} {} {} {} ({})\n", B_s, B_f1, B_f0, B_e, B_c);
        printk!("C: {} {} {} {} ({})\n", C_s, C_f1, C_f0, C_e, C_c);
    }

    if (A_c == FP_CLS_INF && C_c == FP_CLS_ZERO)
        || (A_c == FP_CLS_ZERO && C_c == FP_CLS_INF)
    {
        FP_SET_EXCEPTION!(EFLAG_VXIMZ);
    }

    FP_MUL_D!(T, A, C);

    if B_c != FP_CLS_NAN {
        B_s ^= 1;
    }

    if T_s != B_s && T_c == FP_CLS_INF && B_c == FP_CLS_INF {
        FP_SET_EXCEPTION!(EFLAG_VXISI);
    }

    FP_ADD_D!(R, T, B);

    #[cfg(feature = "DEBUG")]
    {
        printk!("D: {} {} {} {} ({})\n", R_s, R_f1, R_f0, R_e, R_c);
    }

    __FP_PACK_D!(frD, R);

    FP_CUR_EXCEPTIONS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
