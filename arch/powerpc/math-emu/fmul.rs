// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

// Dependencies supplied by the kernel soft-fp environment:
// <linux/types.h>, <linux/errno.h>, <linux/uaccess.h>,
// <asm/sfp-machine.h>, <math-emu/soft-fp.h>, and <math-emu/double.h>

extern "C" {
    fn printk(fmt: *const i8, ...) -> i32;
}

pub unsafe fn fmul(frD: *mut c_void, frA: *mut c_void, frB: *mut c_void) -> i32 {
    FP_DECL_D!(A);
    FP_DECL_D!(B);
    FP_DECL_D!(R);
    FP_DECL_EX!();

    #[cfg(feature = "DEBUG")]
    {
        printk(
            b"%s: %p %p %p\n\0".as_ptr() as *const i8,
            b"fmul\0".as_ptr() as *const i8,
            frD,
            frA,
            frB,
        );
    }

    FP_UNPACK_DP!(A, frA);
    FP_UNPACK_DP!(B, frB);

    #[cfg(feature = "DEBUG")]
    {
        printk(
            b"A: %ld %lu %lu %ld (%ld) [%08lx.%08lx %lx]\n\0".as_ptr() as *const i8,
            A_s,
            A_f1,
            A_f0,
            A_e,
            A_c,
            A_f1,
            A_f0,
            A_e + 1023,
        );
        printk(
            b"B: %ld %lu %lu %ld (%ld) [%08lx.%08lx %lx]\n\0".as_ptr() as *const i8,
            B_s,
            B_f1,
            B_f0,
            B_e,
            B_c,
            B_f1,
            B_f0,
            B_e + 1023,
        );
    }

    if (A_c == FP_CLS_INF && B_c == FP_CLS_ZERO
        || A_c == FP_CLS_ZERO && B_c == FP_CLS_INF)
    {
        FP_SET_EXCEPTION!(EFLAG_VXIMZ);
    }

    FP_MUL_D!(R, A, B);

    #[cfg(feature = "DEBUG")]
    {
        printk(
            b"D: %ld %lu %lu %ld (%ld) [%08lx.%08lx %lx]\n\0".as_ptr() as *const i8,
            R_s,
            R_f1,
            R_f0,
            R_e,
            R_c,
            R_f1,
            R_f0,
            R_e + 1023,
        );
    }

    __FP_PACK_D!(frD, R);

    FP_CUR_EXCEPTIONS!()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
