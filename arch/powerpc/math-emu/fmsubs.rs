// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux soft-fp and PowerPC math-emu support.

use core::ffi::c_void;

pub unsafe fn fmsubs(
    frD: *mut c_void,
    frA: *mut c_void,
    frB: *mut c_void,
    frC: *mut c_void,
) -> i32 {
    let mut r = FP_DECL_D!();
    let mut a = FP_DECL_D!();
    let mut b = FP_DECL_D!();
    let mut c = FP_DECL_D!();
    let mut t = FP_DECL_D!();
    FP_DECL_EX!();

    // #ifdef DEBUG
    // printk!("{}: {:p} {:p} {:p} {:p}\n", "fmsubs", frD, frA, frB, frC);
    // #endif

    FP_UNPACK_DP!(a, frA);
    FP_UNPACK_DP!(b, frB);
    FP_UNPACK_DP!(c, frC);

    // #ifdef DEBUG
    // printk!("A: {} {} {} {} ({})\n", a.s, a.f1, a.f0, a.e, a.c);
    // printk!("B: {} {} {} {} ({})\n", b.s, b.f1, b.f0, b.e, b.c);
    // printk!("C: {} {} {} {} ({})\n", c.s, c.f1, c.f0, c.e, c.c);
    // #endif

    if (a.c == FP_CLS_INF && c.c == FP_CLS_ZERO
        || a.c == FP_CLS_ZERO && c.c == FP_CLS_INF)
    {
        FP_SET_EXCEPTION!(EFLAG_VXIMZ);
    }

    FP_MUL_D!(t, a, c);

    if b.c != FP_CLS_NAN {
        b.s ^= 1;
    }

    if t.s != b.s && t.c == FP_CLS_INF && b.c == FP_CLS_INF {
        FP_SET_EXCEPTION!(EFLAG_VXISI);
    }

    FP_ADD_D!(r, t, b);

    // #ifdef DEBUG
    // printk!("D: {} {} {} {} ({})\n", r.s, r.f1, r.f0, r.e, r.c);
    // #endif

    __FP_PACK_DS!(frD, r);

    FP_CUR_EXCEPTIONS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
