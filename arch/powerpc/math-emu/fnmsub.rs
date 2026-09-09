// SPDX-License-Identifier: GPL-2.0
//
// The declarations below correspond to the types and soft-fp operations
// supplied by the C headers included by the original implementation.

use core::ffi::c_void;

#[repr(C)]
pub struct FpDouble {
    pub s: i64,
    pub f1: u64,
    pub f0: u64,
    pub e: i64,
    pub c: i64,
}

extern "C" {
    fn fp_unpack_dp(value: *mut FpDouble, source: *const c_void);
    fn fp_mul_d(result: *mut FpDouble, left: *const FpDouble, right: *const FpDouble);
    fn fp_add_d(result: *mut FpDouble, left: *const FpDouble, right: *const FpDouble);
    fn fp_pack_d(destination: *mut c_void, value: *const FpDouble);
    fn fp_set_exception(flag: i32);
    fn fp_cur_exceptions() -> i32;
}

pub const FP_CLS_ZERO: i64 = 0;
pub const FP_CLS_INF: i64 = 1;
pub const FP_CLS_NAN: i64 = 2;
pub const EFLAG_VXIMZ: i32 = 1;
pub const EFLAG_VXISI: i32 = 2;

pub unsafe fn fnmsub(
    fr_d: *mut c_void,
    fr_a: *mut c_void,
    fr_b: *mut c_void,
    fr_c: *mut c_void,
) -> i32 {
    let mut r = FpDouble { s: 0, f1: 0, f0: 0, e: 0, c: 0 };
    let mut a = FpDouble { s: 0, f1: 0, f0: 0, e: 0, c: 0 };
    let mut b = FpDouble { s: 0, f1: 0, f0: 0, e: 0, c: 0 };
    let mut c = FpDouble { s: 0, f1: 0, f0: 0, e: 0, c: 0 };
    let mut t = FpDouble { s: 0, f1: 0, f0: 0, e: 0, c: 0 };

    fp_unpack_dp(&mut a, fr_a as *const c_void);
    fp_unpack_dp(&mut b, fr_b as *const c_void);
    fp_unpack_dp(&mut c, fr_c as *const c_void);

    if (a.c == FP_CLS_INF && c.c == FP_CLS_ZERO)
        || (a.c == FP_CLS_ZERO && c.c == FP_CLS_INF)
    {
        fp_set_exception(EFLAG_VXIMZ);
    }

    fp_mul_d(&mut t, &a, &c);

    if b.c != FP_CLS_NAN {
        b.s ^= 1;
    }

    if t.s != b.s && t.c == FP_CLS_INF && b.c == FP_CLS_INF {
        fp_set_exception(EFLAG_VXISI);
    }

    fp_add_d(&mut r, &t, &b);

    if r.c != FP_CLS_NAN {
        r.s ^= 1;
    }

    fp_pack_d(fr_d, &r);

    fp_cur_exceptions()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
