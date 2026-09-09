// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

// linux/export.h and linux/libgcc.h are supplied by the surrounding build.

const W_TYPE_SIZE: u32 = 32;

#[inline]
fn __ll_b() -> u32 {
    1u32 << (W_TYPE_SIZE / 2)
}

#[inline]
fn __ll_lowpart(t: u32) -> u32 {
    t & (__ll_b() - 1)
}

#[inline]
fn __ll_highpart(t: u32) -> u32 {
    t >> (W_TYPE_SIZE / 2)
}

/* If we still don't have umul_ppmm, define it using plain Rust. */
#[inline]
unsafe fn umul_ppmm(u: u32, v: u32) -> (u32, u32) {
    let ul: u32 = __ll_lowpart(u);
    let uh: u32 = __ll_highpart(u);
    let vl: u32 = __ll_lowpart(v);
    let vh: u32 = __ll_highpart(v);

    let x0: u32 = ul.wrapping_mul(vl);
    let mut x1: u32 = ul.wrapping_mul(vh);
    let x2: u32 = uh.wrapping_mul(vl);
    let mut x3: u32 = uh.wrapping_mul(vh);

    x1 = x1.wrapping_add(__ll_highpart(x0)); // this can't give carry
    x1 = x1.wrapping_add(x2); // but this indeed can
    if x1 < x2 { // did we get it?
        x3 = x3.wrapping_add(__ll_b()); // yes, add it in the proper pos
    }

    let w1 = x3.wrapping_add(__ll_highpart(x1));
    let w0 = __ll_lowpart(x1)
        .wrapping_mul(__ll_b())
        .wrapping_add(__ll_lowpart(x0));
    (w1, w0)
}

#[inline]
unsafe fn __umulsidi3(u: u32, v: u32) -> i64 {
    let (high, low) = umul_ppmm(u, v);
    (((high as u64) << 32) | low as u64) as i64
}

pub unsafe fn __muldi3(u: i64, v: i64) -> i64 {
    let uu = DWunion { ll: u };
    let vv = DWunion { ll: v };
    let mut w = DWunion { ll: __umulsidi3(uu.s.low, vv.s.low) };

    w.s.high = w.s.high.wrapping_add(
        uu.s.low.wrapping_mul(vv.s.high)
            .wrapping_add(uu.s.high.wrapping_mul(vv.s.low)),
    );

    w.ll
}

// EXPORT_SYMBOL(__muldi3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
