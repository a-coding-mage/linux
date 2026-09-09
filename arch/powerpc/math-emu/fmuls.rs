// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the surrounding math-emu implementation:
// linux/types.h, linux/errno.h, linux/uaccess.h, asm/sfp-machine.h,
// math-emu/soft-fp.h, math-emu/double.h, and math-emu/single.h.

pub unsafe fn fmuls(frD: *mut core::ffi::c_void,
                    frA: *mut core::ffi::c_void,
                    frB: *mut core::ffi::c_void) -> core::ffi::c_int {
    FP_DECL_D!(A);
    FP_DECL_D!(B);
    FP_DECL_D!(R);
    FP_DECL_EX!();

    // #ifdef DEBUG
    // printk("%s: %p %p %p\n", __func__, frD, frA, frB);
    // #endif

    FP_UNPACK_DP!(A, frA);
    FP_UNPACK_DP!(B, frB);

    // #ifdef DEBUG
    // printk("A: %ld %lu %lu %ld (%ld) [%08lx.%08lx %lx]\n",
    //        A_s, A_f1, A_f0, A_e, A_c, A_f1, A_f0, A_e + 1023);
    // printk("B: %ld %lu %lu %ld (%ld) [%08lx.%08lx %lx]\n",
    //        B_s, B_f1, B_f0, B_e, B_c, B_f1, B_f0, B_e + 1023);
    // #endif

    if ((A_c == FP_CLS_INF && B_c == FP_CLS_ZERO)
        || (A_c == FP_CLS_ZERO && B_c == FP_CLS_INF))
    {
        FP_SET_EXCEPTION!(EFLAG_VXIMZ);
    }

    FP_MUL_D!(R, A, B);

    // #ifdef DEBUG
    // printk("D: %ld %lu %lu %ld (%ld) [%08lx.%08lx %lx]\n",
    //        R_s, R_f1, R_f0, R_e, R_c, R_f1, R_f0, R_e + 1023);
    // #endif

    __FP_PACK_DS!(frD, R);

    FP_CUR_EXCEPTIONS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
