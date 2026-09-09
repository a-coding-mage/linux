// SPDX-License-Identifier: GPL-2.0
// Translated from C. Dependencies supplied by the Linux math-emu environment
// are intentionally referenced here rather than reimplemented.

use core::ffi::c_void;

unsafe extern "C" {
    static mut __FPU_FPSCR: u32;

    #[cfg(feature = "DEBUG")]
    fn printk(fmt: *const i8, ...);
    #[cfg(feature = "DEBUG")]
    fn dump_double(value: *mut u32);
}

// FP_DECL_D(B) and FP_DECL_EX are declarations supplied by soft-fp.h.
// FP_UNPACK_DP and FP_TO_INT_D are the corresponding soft-fp operations.

pub unsafe fn fctiwz(frD: *mut u32, frB: *mut c_void) -> i32 {
    let mut b = FP_DECL_D!();
    FP_DECL_EX!();
    let fpscr: u32;
    let mut r: u32;

    fpscr = __FPU_FPSCR;
    __FPU_FPSCR &= !(3u32);
    __FPU_FPSCR |= FP_RND_ZERO;

    FP_UNPACK_DP!(b, frB);
    FP_TO_INT_D!(r, b, 32, 1);
    frD.add(1).write(r);

    __FPU_FPSCR = fpscr;

    #[cfg(feature = "DEBUG")]
    {
        printk(b"%s: D %p, B %p: \0".as_ptr() as *const i8);
        dump_double(frD);
        printk(b"\n\0".as_ptr() as *const i8);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
