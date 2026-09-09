// SPDX-License-Identifier: GPL-2.0
// The C source includes Linux and soft-fp headers.  Their declarations and
// macro implementations are supplied by the surrounding math-emu code.

use core::ffi::c_void;

#[repr(C)]
pub struct FpDeclD {
    pub s: i64,
    pub f1: u64,
    pub f0: u64,
    pub e: i64,
    pub c: i64,
}

extern "C" {
    fn fp_unpack_dp(value: *mut FpDeclD, source: *const c_void);
    fn fp_sqrt_d(result: *mut FpDeclD, value: *const FpDeclD);
    fn fp_pack_d(destination: *mut c_void, value: *const FpDeclD);
    fn fp_set_exception(exception: i32);
    fn fp_cur_exceptions() -> i32;

    #[cfg(feature = "DEBUG")]
    fn printk(format: *const u8, ...);
}

// Values supplied by <math-emu/soft-fp.h>.
extern "C" {
    static FP_CLS_ZERO: i64;
    static FP_CLS_NAN: i64;
    static EFLAG_VXSQRT: i32;
    static EFLAG_VXSNAN: i32;
}

#[no_mangle]
pub unsafe extern "C" fn fsqrt(fr_d: *mut c_void, fr_b: *mut c_void) -> i32 {
    // FP_DECL_D(B);
    let mut b = FpDeclD { s: 0, f1: 0, f0: 0, e: 0, c: 0 };
    // FP_DECL_D(R);
    let mut r = FpDeclD { s: 0, f1: 0, f0: 0, e: 0, c: 0 };
    // FP_DECL_EX;

    #[cfg(feature = "DEBUG")]
    printk(b"%s: %p %p %p %p\0".as_ptr(), fr_d, fr_b);

    // FP_UNPACK_DP(B, frB);
    fp_unpack_dp(&mut b, fr_b);

    #[cfg(feature = "DEBUG")]
    printk(b"B: %ld %lu %lu %ld (%ld)\0".as_ptr(), b.s, b.f1, b.f0, b.e, b.c);

    if b.s != 0 && b.c != FP_CLS_ZERO {
        // FP_SET_EXCEPTION(EFLAG_VXSQRT);
        fp_set_exception(EFLAG_VXSQRT);
    }
    if b.c == FP_CLS_NAN {
        // FP_SET_EXCEPTION(EFLAG_VXSNAN);
        fp_set_exception(EFLAG_VXSNAN);
    }

    // FP_SQRT_D(R, B);
    fp_sqrt_d(&mut r, &b);

    #[cfg(feature = "DEBUG")]
    printk(b"R: %ld %lu %lu %ld (%ld)\0".as_ptr(), r.s, r.f1, r.f0, r.e, r.c);

    // __FP_PACK_D(frD, R);
    fp_pack_d(fr_d, &r);

    // FP_CUR_EXCEPTIONS;
    fp_cur_exceptions()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
