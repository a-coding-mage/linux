// SPDX-License-Identifier: GPL-2.0
/*
 * RZV2H CPG Library. This library provides common functions to calculate
 * PLL parameters for the RZV2H SoC.
 *
 * Copyright (C) 2026 Renesas Electronics Corp.
 */

// Linux headers and exported-symbol metadata are supplied by the surrounding
// translation unit.

pub const MILLI: u64 = 1_000;
pub const MEGA: u64 = 1_000_000;

#[repr(C)]
pub struct rzv2h_pll_limits {
    pub input_fref: u64,
    pub fout: rzv2h_range,
    pub p: rzv2h_range,
    pub s: rzv2h_range,
    pub m: rzv2h_range,
    pub k: rzv2h_range,
    pub fvco: rzv2h_range,
}

#[repr(C)]
pub struct rzv2h_range { pub min: u32, pub max: u32 }

#[repr(C)]
pub struct rzv2h_pll_pars {
    pub p: u32, pub s: u32, pub m: u32, pub k: i64,
    pub error_millihz: i64, pub freq_millihz: u64,
}

#[repr(C)]
pub struct rzv2h_pll_div_pars {
    pub pll: rzv2h_pll_pars,
    pub div: rzv2h_div_pars,
}

#[repr(C)]
pub struct rzv2h_div_pars {
    pub divider_value: u8, pub error_millihz: i64, pub freq_millihz: u64,
}

#[inline]
fn div_round_closest_ull(n: u64, d: u64) -> u64 { (n + d / 2) / d }
#[inline]
fn div_round_closest_s64(n: i64, d: i64) -> i64 { (n + if n >= 0 { d / 2 } else { -(d / 2) }) / d }

/// Finds the best combination of PLL parameters for a given frequency.
pub unsafe fn rzv2h_cpg_get_pll_pars(
    limits: *const rzv2h_pll_limits, pars: *mut rzv2h_pll_pars, freq_millihz: u64,
) -> bool {
    let l = &*limits;
    let input_fref = if l.input_fref != 0 { l.input_fref } else { 24 * MEGA };
    let fout_min_millihz = l.fout.min as u64 * MILLI;
    let fout_max_millihz = l.fout.max as u64 * MILLI;
    let mut p = rzv2h_pll_pars { p: 0, s: 0, m: 0, k: 0, error_millihz: 0, freq_millihz: 0 };
    let mut best = p;
    if freq_millihz > fout_max_millihz || freq_millihz < fout_min_millihz { return false; }
    best.error_millihz = i64::MAX;
    p.p = l.p.min;
    while p.p <= l.p.max {
        let fref = input_fref / p.p as u64;
        let mut divider = 1u64 << l.s.min;
        p.s = l.s.min;
        while p.s <= l.s.max {
            p.m = l.m.min;
            while p.m <= l.m.max {
                let output_m = div_round_closest_ull(p.m as u64 * fref * MILLI, divider);
                let output_k_range = div_round_closest_ull(fref * MILLI, 2 * divider);
                if !(freq_millihz < output_m - output_k_range || freq_millihz >= output_m + output_k_range) {
                    let output_k = freq_millihz as i64 - output_m as i64;
                    let pll_k = div_round_closest_s64(output_k * 65536 * divider as i64, fref as i64);
                    if pll_k >= l.k.min as i64 && pll_k <= l.k.max as i64 {
                        p.k = pll_k;
                        let fvco = (p.m as i64 * 65536 + p.k) as u64 * fref;
                        if fvco >= l.fvco.min as u64 * 65536 && fvco <= l.fvco.max as u64 * 65536 {
                            let mut output = (p.m as u64 * 65536 * input_fref) + p.k as u64 * input_fref;
                            output *= MILLI;
                            output = div_round_closest_ull(output, 65536 * p.p as u64 * divider);
                            if output >= fout_min_millihz && output <= fout_max_millihz {
                                p.error_millihz = freq_millihz as i64 - output as i64;
                                p.freq_millihz = output;
                                if p.error_millihz == 0 { *pars = p; return true; }
                                if best.error_millihz.abs() > p.error_millihz.abs() { best = p; }
                            }
                        }
                    }
                }
                p.m += 1;
            }
            p.s += 1; divider <<= 1;
        }
        p.p += 1;
    }
    if best.error_millihz == i64::MAX { return false; }
    *pars = best; true
}

/// Finds the best combination of PLL parameters and divider value.
pub unsafe fn rzv2h_cpg_get_pll_divs_pars(
    limits: *const rzv2h_pll_limits, pars: *mut rzv2h_pll_div_pars,
    table: *const u8, table_size: u8, freq_millihz: u64,
) -> bool {
    let mut p = rzv2h_pll_div_pars { pll: rzv2h_pll_pars { p: 0, s: 0, m: 0, k: 0, error_millihz: 0, freq_millihz: 0 }, div: rzv2h_div_pars { divider_value: 0, error_millihz: i64::MAX, freq_millihz: 0 } };
    let mut best = p;
    for i in 0..table_size as usize {
        let divider = *table.add(i);
        if !rzv2h_cpg_get_pll_pars(limits, &mut p.pll, freq_millihz * divider as u64) { continue; }
        p.div.divider_value = divider;
        p.div.freq_millihz = div_round_closest_ull(p.pll.freq_millihz, divider as u64);
        p.div.error_millihz = freq_millihz as i64 - p.div.freq_millihz as i64;
        if p.div.error_millihz == 0 { *pars = p; return true; }
        if best.div.error_millihz.abs() > p.div.error_millihz.abs() { best = p; }
    }
    if best.div.error_millihz == i64::MAX { return false; }
    *pars = best; true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
