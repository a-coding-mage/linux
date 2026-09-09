// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the kernel/math-emu environment are intentionally
// left as external Rust items/macros.

pub unsafe fn fdiv(frD: *mut core::ffi::c_void, frA: *mut core::ffi::c_void,
                   frB: *mut core::ffi::c_void) -> i32 {
    FP_DECL_D!(A);
    FP_DECL_D!(B);
    FP_DECL_D!(R);
    FP_DECL_EX!();

    #[cfg(feature = "DEBUG")]
    {
        printk!("%s: %p %p %p\n", "fdiv", frD, frA, frB);
    }

    FP_UNPACK_DP!(A, frA);
    FP_UNPACK_DP!(B, frB);

    #[cfg(feature = "DEBUG")]
    {
        printk!("A: %ld %lu %lu %ld (%ld)\n", A_s, A_f1, A_f0, A_e, A_c);
        printk!("B: %ld %lu %lu %ld (%ld)\n", B_s, B_f1, B_f0, B_e, B_c);
    }

    if A_c == FP_CLS_ZERO && B_c == FP_CLS_ZERO {
        FP_SET_EXCEPTION!(EFLAG_VXZDZ);
        #[cfg(feature = "DEBUG")]
        {
            printk!("%s: FPSCR_VXZDZ raised\n", "fdiv");
        }
    }
    if A_c == FP_CLS_INF && B_c == FP_CLS_INF {
        FP_SET_EXCEPTION!(EFLAG_VXIDI);
        #[cfg(feature = "DEBUG")]
        {
            printk!("%s: FPSCR_VXIDI raised\n", "fdiv");
        }
    }

    if B_c == FP_CLS_ZERO && A_c != FP_CLS_ZERO {
        FP_SET_EXCEPTION!(EFLAG_DIVZERO);
        if __FPU_TRAP_P!(EFLAG_DIVZERO) {
            return FP_CUR_EXCEPTIONS;
        }
    }
    FP_DIV_D!(R, A, B);

    #[cfg(feature = "DEBUG")]
    {
        printk!("D: %ld %lu %lu %ld (%ld)\n", R_s, R_f1, R_f0, R_e, R_c);
    }

    __FP_PACK_D!(frD, R);

    FP_CUR_EXCEPTIONS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
