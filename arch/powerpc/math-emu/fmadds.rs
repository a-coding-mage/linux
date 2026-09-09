// SPDX-License-Identifier: GPL-2.0
//
// The declarations and operations used below are supplied by the kernel
// soft-fp, double, and single-precision emulation headers.

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
    fn fp_unpack_dp(x: *mut FpDouble, p: *const c_void);
    fn fp_mul_d(r: *mut FpDouble, a: *const FpDouble, b: *const FpDouble);
    fn fp_add_d(r: *mut FpDouble, a: *const FpDouble, b: *const FpDouble);
    fn fp_pack_ds(p: *mut c_void, r: *const FpDouble);
    fn fp_set_exception(flag: i32);
    fn fp_cur_exceptions() -> i32;
}

// FP_CLS_INF, FP_CLS_ZERO, and EFLAG_* are provided by soft-fp.h.
extern "C" {
    static FP_CLS_INF: i64;
    static FP_CLS_ZERO: i64;
    static EFLAG_VXIMZ: i32;
    static EFLAG_VXISI: i32;
}

pub unsafe fn fmadds(
    fr_d: *mut c_void,
    fr_a: *mut c_void,
    fr_b: *mut c_void,
    fr_c: *mut c_void,
) -> i32 {
    // FP_DECL_D(R);
    let mut r = FpDouble { s: 0, f1: 0, f0: 0, e: 0, c: 0 };
    // FP_DECL_D(A);
    let mut a = FpDouble { s: 0, f1: 0, f0: 0, e: 0, c: 0 };
    // FP_DECL_D(B);
    let mut b = FpDouble { s: 0, f1: 0, f0: 0, e: 0, c: 0 };
    // FP_DECL_D(C);
    let mut c = FpDouble { s: 0, f1: 0, f0: 0, e: 0, c: 0 };
    // FP_DECL_D(T);
    let mut t = FpDouble { s: 0, f1: 0, f0: 0, e: 0, c: 0 };
    // FP_DECL_EX;

    #[cfg(feature = "DEBUG")]
    unsafe {
        // printk("%s: %p %p %p %p\n", __func__, frD, frA, frB, frC);
    }

    fp_unpack_dp(&mut a, fr_a);
    fp_unpack_dp(&mut b, fr_b);
    fp_unpack_dp(&mut c, fr_c);

    #[cfg(feature = "DEBUG")]
    unsafe {
        // printk("A: %ld %lu %lu %ld (%ld)\n", A_s, A_f1, A_f0, A_e, A_c);
        // printk("B: %ld %lu %lu %ld (%ld)\n", B_s, B_f1, B_f0, B_e, B_c);
        // printk("C: %ld %lu %lu %ld (%ld)\n", C_s, C_f1, C_f0, C_e, C_c);
    }

    if (a.c == FP_CLS_INF && c.c == FP_CLS_ZERO
        || a.c == FP_CLS_ZERO && c.c == FP_CLS_INF)
    {
        fp_set_exception(EFLAG_VXIMZ);
    }

    fp_mul_d(&mut t, &a, &c);

    if t.s != b.s && t.c == FP_CLS_INF && b.c == FP_CLS_INF {
        fp_set_exception(EFLAG_VXISI);
    }

    fp_add_d(&mut r, &t, &b);

    #[cfg(feature = "DEBUG")]
    unsafe {
        // printk("D: %ld %lu %lu %ld (%ld)\n", R_s, R_f1, R_f0, R_e, R_c);
    }

    fp_pack_ds(fr_d, &r);

    fp_cur_exceptions()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
