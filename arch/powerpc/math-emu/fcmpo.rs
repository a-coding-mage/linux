// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/math-emu environment:
// linux/types.h, linux/errno.h, linux/uaccess.h,
// asm/sfp-machine.h, math-emu/soft-fp.h, and math-emu/double.h.

use core::ffi::c_void;

pub unsafe fn fcmpo(ccr: *mut u32, crfD: i32, frA: *mut c_void, frB: *mut c_void) -> i32 {
    FP_DECL_D!(A);
    FP_DECL_D!(B);
    FP_DECL_EX!();
    let code: [isize; 4] = [1 << 3, 1 << 1, 1 << 2, 1 << 0];
    let mut cmp: isize;

    // #ifdef DEBUG
    // printk("%s: %p (%08x) %d %p %p\\n", __func__, ccr, *ccr, crfD, frA, frB);
    // #endif

    FP_UNPACK_DP!(A, frA);
    FP_UNPACK_DP!(B, frB);

    // #ifdef DEBUG
    // printk("A: %ld %lu %lu %ld (%ld)\\n", A_s, A_f1, A_f0, A_e, A_c);
    // printk("B: %ld %lu %lu %ld (%ld)\\n", B_s, B_f1, B_f0, B_e, B_c);
    // #endif

    if A_c == FP_CLS_NAN || B_c == FP_CLS_NAN {
        FP_SET_EXCEPTION!(EFLAG_VXVC);
    }

    FP_CMP_D!(cmp, A, B, 2);
    cmp = code[((cmp + 1) & 3) as usize];

    __FPU_FPSCR &= !0x1f000;
    __FPU_FPSCR |= cmp << 12;

    let shift = ((7 - crfD) << 2) as u32;
    *ccr &= !(15u32 << shift);
    *ccr |= (cmp as u32) << shift;

    // #ifdef DEBUG
    // printk("CR: %08x\\n", *ccr);
    // #endif

    FP_CUR_EXCEPTIONS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
