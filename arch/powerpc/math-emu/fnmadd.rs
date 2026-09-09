// SPDX-License-Identifier: GPL-2.0
//
// C dependencies:
// linux/types.h, linux/errno.h, linux/uaccess.h,
// asm/sfp-machine.h, math-emu/soft-fp.h, math-emu/double.h

pub unsafe fn fnmadd(
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

    // #ifdef DEBUG
    // printk("%s: %p %p %p %p\\n", __func__, frD, frA, frB, frC);
    // #endif

    FP_UNPACK_DP!(A, frA);
    FP_UNPACK_DP!(B, frB);
    FP_UNPACK_DP!(C, frC);

    // #ifdef DEBUG
    // printk("A: %ld %lu %lu %ld (%ld)\\n", A_s, A_f1, A_f0, A_e, A_c);
    // printk("B: %ld %lu %lu %ld (%ld)\\n", B_s, B_f1, B_f0, B_e, B_c);
    // printk("C: %ld %lu %lu %ld (%ld)\\n", C_s, C_f1, C_f0, C_e, C_c);
    // #endif

    if ((A_c == FP_CLS_INF && C_c == FP_CLS_ZERO)
        || (A_c == FP_CLS_ZERO && C_c == FP_CLS_INF))
    {
        FP_SET_EXCEPTION!(EFLAG_VXIMZ);
    }

    FP_MUL_D!(T, A, C);

    if (T_s != B_s && T_c == FP_CLS_INF && B_c == FP_CLS_INF) {
        FP_SET_EXCEPTION!(EFLAG_VXISI);
    }

    FP_ADD_D!(R, T, B);

    if (R_c != FP_CLS_NAN) {
        R_s ^= 1;
    }

    // #ifdef DEBUG
    // printk("D: %ld %lu %lu %ld (%ld)\\n", R_s, R_f1, R_f0, R_e, R_c);
    // #endif

    __FP_PACK_D!(frD, R);

    FP_CUR_EXCEPTIONS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
