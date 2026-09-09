// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the kernel soft-float environment:
// linux types, errno, uaccess, sfp-machine, soft-fp, double, and single.

use core::ffi::c_void;

pub unsafe fn fsubs(frD: *mut c_void, frA: *mut c_void, frB: *mut c_void) -> i32 {
    FP_DECL_D!(A);
    FP_DECL_D!(B);
    FP_DECL_D!(R);
    FP_DECL_EX!();

    // C build-time DEBUG conditional.
    #[cfg(feature = "DEBUG")]
    printk!("{}: {:p} {:p} {:p}\n", "fsubs", frD, frA, frB);

    FP_UNPACK_DP!(A, frA);
    FP_UNPACK_DP!(B, frB);

    #[cfg(feature = "DEBUG")]
    printk!("A: {} {} {} {} ({})\n", A_s, A_f1, A_f0, A_e, A_c);
    #[cfg(feature = "DEBUG")]
    printk!("B: {} {} {} {} ({})\n", B_s, B_f1, B_f0, B_e, B_c);

    if B_c != FP_CLS_NAN {
        B_s ^= 1;
    }

    if A_s != B_s && A_c == FP_CLS_INF && B_c == FP_CLS_INF {
        FP_SET_EXCEPTION!(EFLAG_VXISI);
    }

    FP_ADD_D!(R, A, B);

    #[cfg(feature = "DEBUG")]
    printk!("D: {} {} {} {} ({})\n", R_s, R_f1, R_f0, R_e, R_c);

    __FP_PACK_DS!(frD, R);

    FP_CUR_EXCEPTIONS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
