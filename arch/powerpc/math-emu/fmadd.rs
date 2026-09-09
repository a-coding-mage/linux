// SPDX-License-Identifier: GPL-2.0
//
// Dependencies corresponding to the original Linux and soft-float headers are
// supplied by the surrounding translation unit.

use core::ffi::c_void;

pub unsafe fn fmadd(
    fr_d: *mut c_void,
    fr_a: *mut c_void,
    fr_b: *mut c_void,
    fr_c: *mut c_void,
) -> i32 {
    FP_DECL_D!(R);
    FP_DECL_D!(A);
    FP_DECL_D!(B);
    FP_DECL_D!(C);
    FP_DECL_D!(T);
    FP_DECL_EX!();

    #[cfg(feature = "DEBUG")]
    {
        printk!("{}: {:?} {:?} {:?} {:?}\n", "fmadd", fr_d, fr_a, fr_b, fr_c);
    }

    FP_UNPACK_DP!(A, fr_a);
    FP_UNPACK_DP!(B, fr_b);
    FP_UNPACK_DP!(C, fr_c);

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

    if T_s != B_s && T_c == FP_CLS_INF && B_c == FP_CLS_INF {
        FP_SET_EXCEPTION!(EFLAG_VXISI);
    }

    FP_ADD_D!(R, T, B);

    #[cfg(feature = "DEBUG")]
    {
        printk!("D: {} {} {} {} ({})\n", R_s, R_f1, R_f0, R_e, R_c);
    }

    __FP_PACK_D!(fr_d, R);

    FP_CUR_EXCEPTIONS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
