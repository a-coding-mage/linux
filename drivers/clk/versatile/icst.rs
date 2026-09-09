// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/common/icst307.c
 *
 *  Copyright (C) 2003 Deep Blue Solutions, Ltd, All Rights Reserved.
 *
 *  Support functions for calculating clocks/divisors for the ICST307
 *  clock generators.  See https://www.idt.com/ for more information
 *  on these devices.
 *
 *  This is an almost identical implementation to the ICST525 clock generator.
 *  The s2div and idx2s files are different
 */

/* Dependencies supplied by the surrounding translation unit. */

/*
 * Divisors for each OD setting.
 */
pub static icst307_s2div: [u8; 8] = [10, 2, 8, 4, 5, 7, 3, 6];
pub static icst525_s2div: [u8; 8] = [10, 2, 8, 4, 5, 7, 9, 6];

pub unsafe fn icst_hz(p: *const icst_params, vco: icst_vco) -> c_ulong {
    let mut dividend: u64 = (*p).ref_ * 2 * (vco.v as u64 + 8);
    let divisor: u32 = (vco.r as u32 + 2) * (*p).s2div[vco.s as usize] as u32;

    dividend /= divisor as u64;
    dividend as c_ulong
}

/*
 * Ascending divisor S values.
 */
pub static icst307_idx2s: [u8; 8] = [1, 6, 3, 4, 7, 5, 2, 0];
pub static icst525_idx2s: [u8; 8] = [1, 3, 4, 7, 5, 2, 6, 0];

pub unsafe fn icst_hz_to_vco(p: *const icst_params, freq: c_ulong) -> icst_vco {
    let mut vco = icst_vco {
        s: 1,
        v: (*p).vd_max,
        r: (*p).rd_max,
    };
    let mut f: c_ulong;
    let mut i: c_uint = 0;
    let mut best: c_uint = c_uint::MAX;

    /*
     * First, find the PLL output divisor such
     * that the PLL output is within spec.
     */
    loop {
        f = freq * (*p).s2div[(*p).idx2s[i as usize] as usize] as c_ulong;

        if f > (*p).vco_min && f <= (*p).vco_max {
            break;
        }
        i += 1;
        if i >= 8 {
            return vco;
        }
    }

    vco.s = (*p).idx2s[i as usize];

    /*
     * Now find the closest divisor combination
     * which gives a PLL output of 'f'.
     */
    let mut rd = (*p).rd_min;
    while rd <= (*p).rd_max {
        let fref_div: c_ulong = (2 * (*p).ref_) / rd as c_ulong;
        let vd: c_uint = ((f + fref_div / 2) / fref_div) as c_uint;
        if vd < (*p).vd_min || vd > (*p).vd_max {
            rd += 1;
            continue;
        }

        let f_pll: c_ulong = fref_div * vd as c_ulong;
        let mut f_diff: c_long = f_pll as c_long - f as c_long;
        if f_diff < 0 {
            f_diff = -f_diff;
        }

        if (f_diff as c_uint) < best {
            vco.v = vd - 8;
            vco.r = rd - 2;
            if f_diff == 0 {
                break;
            }
            best = f_diff as c_uint;
        }
        rd += 1;
    }

    vco
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
