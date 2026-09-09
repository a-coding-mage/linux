// SPDX-License-Identifier: GPL-2.0

// The definitions supplied by libgcc.h are represented here directly.
#[repr(C)]
pub struct DWstruct {
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
pub union DWunion {
    pub ll: i64,
    pub s: DWstruct,
}

const W_TYPE_SIZE: u32 = 32;
const LL_B: u32 = 1u32 << (W_TYPE_SIZE / 2);

#[inline]
fn ll_lowpart(t: u32) -> u32 {
    t & (LL_B - 1)
}

#[inline]
fn ll_highpart(t: u32) -> u32 {
    t >> (W_TYPE_SIZE / 2)
}

/* If we still don't have umul_ppmm, define it using plain C. */
#[inline]
fn umul_ppmm(u: u32, v: u32) -> (u32, u32) {
    let ul = ll_lowpart(u) as u16;
    let uh = ll_highpart(u) as u16;
    let vl = ll_lowpart(v) as u16;
    let vh = ll_highpart(v) as u16;

    let x0 = (ul as u32).wrapping_mul(vl as u32);
    let mut x1 = (ul as u32).wrapping_mul(vh as u32);
    let x2 = (uh as u32).wrapping_mul(vl as u32);
    let mut x3 = (uh as u32).wrapping_mul(vh as u32);

    x1 = x1.wrapping_add(ll_highpart(x0)); // this can't give carry
    x1 = x1.wrapping_add(x2); // but this indeed can
    if x1 < x2 { // did we get it?
        x3 = x3.wrapping_add(LL_B); // yes, add it in the proper pos
    }

    let w1 = x3.wrapping_add(ll_highpart(x1));
    let w0 = ll_lowpart(x1)
        .wrapping_mul(LL_B)
        .wrapping_add(ll_lowpart(x0));
    (w1, w0)
}

#[inline]
fn umulsidi3(u: u32, v: u32) -> i64 {
    let (high, low) = umul_ppmm(u, v);
    let value = ((high as u64) << 32) | low as u64;
    value as i64
}

pub extern "C" fn __muldi3(u: i64, v: i64) -> i64 {
    let uu = u as u64;
    let vv = v as u64;
    let mut high_low = umulsidi3(uu as u32, vv as u32) as u64;

    let uu_low = uu as u32;
    let uu_high = (uu >> 32) as u32;
    let vv_low = vv as u32;
    let vv_high = (vv >> 32) as u32;

    let cross = (uu_low as u64)
        .wrapping_mul(vv_high as u64)
        .wrapping_add((uu_high as u64).wrapping_mul(vv_low as u64));
    high_low = high_low.wrapping_add(cross << 32);

    high_low as i64
}

// EXPORT_SYMBOL(__muldi3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
