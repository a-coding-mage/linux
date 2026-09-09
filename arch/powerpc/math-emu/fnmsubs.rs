// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the surrounding math-emulation implementation
// correspond to the C headers included by the original source.

pub unsafe fn fnmsubs(
    fr_d: *mut core::ffi::c_void,
    fr_a: *mut core::ffi::c_void,
    fr_b: *mut core::ffi::c_void,
    fr_c: *mut core::ffi::c_void,
) -> i32 {
    let mut r = FP_DECL_D!();
    let mut a = FP_DECL_D!();
    let mut b = FP_DECL_D!();
    let mut c = FP_DECL_D!();
    let mut t = FP_DECL_D!();
    FP_DECL_EX!();

    // #ifdef DEBUG
    // printk!("{}: {:p} {:p} {:p} {:p}\n", "fnmsubs", fr_d, fr_a, fr_b, fr_c);
    // #endif

    FP_UNPACK_DP!(a, fr_a);
    FP_UNPACK_DP!(b, fr_b);
    FP_UNPACK_DP!(c, fr_c);

    // #ifdef DEBUG
    // printk!("A: {} {} {} {} ({})\n", a_s, a_f1, a_f0, a_e, a_c);
    // printk!("B: {} {} {} {} ({})\n", b_s, b_f1, b_f0, b_e, b_c);
    // printk!("C: {} {} {} {} ({})\n", c_s, c_f1, c_f0, c_e, c_c);
    // #endif

    if (a.c == FP_CLS_INF && c.c == FP_CLS_ZERO)
        || (a.c == FP_CLS_ZERO && c.c == FP_CLS_INF)
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

    if r.c != FP_CLS_NAN {
        r.s ^= 1;
    }

    // #ifdef DEBUG
    // printk!("D: {} {} {} {} ({})\n", r_s, r_f1, r_f0, r_e, r_c);
    // #endif

    __FP_PACK_DS!(fr_d, r);

    FP_CUR_EXCEPTIONS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
