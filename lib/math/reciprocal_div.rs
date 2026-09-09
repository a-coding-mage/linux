// SPDX-License-Identifier: GPL-2.0
//
// Dependencies corresponding to the Linux kernel headers used by the C
// implementation are supplied by the surrounding translation unit.

/*
 * For a description of the algorithm please have a look at
 * include/linux/reciprocal_div.h
 */

pub unsafe fn reciprocal_value(d: u32) -> reciprocal_value {
    let mut r: reciprocal_value;
    let mut m: u64;
    let l: i32;

    l = fls(d.wrapping_sub(1));
    m = (1u64 << 32) * ((1u64 << l) - d as u64);
    m /= d as u64;
    m = m.wrapping_add(1);

    r.m = m as u32;
    r.sh1 = min(l, 1);
    r.sh2 = max(l - 1, 0);

    r
}

pub unsafe fn reciprocal_value_adv(d: u32, prec: u8) -> reciprocal_value_adv {
    let mut r: reciprocal_value_adv;
    let l: u32;
    let mut post_shift: u32;
    let mut mhigh: u64;
    let mut mlow: u64;

    /* ceil(log2(d)) */
    l = fls(d.wrapping_sub(1)) as u32;
    /* NOTE: mlow/mhigh could overflow u64 when l == 32. This case needs to
     * be handled before calling "reciprocal_value_adv", please see the
     * comment at include/linux/reciprocal_div.h.
     */
    WARN!(
        l == 32,
        "ceil(log2(0x%08x)) == 32, {} doesn't support such divisor",
        d,
        "reciprocal_value_adv"
    );
    post_shift = l;
    mlow = 1u64 << (32 + l);
    mlow /= d as u64;
    mhigh = (1u64 << (32 + l)) + (1u64 << (32 + l - prec as u32));
    mhigh /= d as u64;

    while post_shift > 0 {
        let lo = mlow >> 1;
        let hi = mhigh >> 1;

        if lo >= hi {
            break;
        }

        mlow = lo;
        mhigh = hi;
        post_shift -= 1;
    }

    r.m = mhigh as u32;
    r.sh = post_shift;
    r.exp = l;
    r.is_wide_m = mhigh > U32_MAX as u64;

    r
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
