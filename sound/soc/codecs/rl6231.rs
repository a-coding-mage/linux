// SPDX-License-Identifier: GPL-2.0-only
/*
 * rl6231.c - RL6231 class device shared support
 *
 * Copyright 2014 Realtek Semiconductor Corp.
 *
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/regmap.h>
// #include <linux/gcd.h>
// #include "rl6231.h"

use core::ffi::{c_int, c_uint};

extern "C" {
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
}

#[repr(C)]
pub struct regmap {
    _unused: [u8; 0],
}

// Provided by "rl6231.h" in the original source.
extern "C" {
    static RL6231_PLL_N_MAX: c_int;
    static RL6231_PLL_M_MAX: c_int;
    static RL6231_PLL_K_MAX: c_int;
    static RL6231_PLL_INP_MAX: c_uint;
    static RL6231_PLL_INP_MIN: c_uint;
}

// Provided by the surrounding kernel/Rust bindings.
extern "C" {
    static EINVAL: c_int;
    fn pr_warn(fmt: *const u8, ...) -> c_int;
    fn pr_debug(fmt: *const u8, ...) -> c_int;
    fn gcd(a: c_uint, b: c_uint) -> c_uint;
}

#[repr(C)]
pub struct rl6231_pll_code {
    pub m_bp: bool,
    pub k_bp: bool,
    pub m_code: c_int,
    pub n_code: c_int,
    pub k_code: c_int,
}

/**
 * rl6231_get_pre_div - Return the value of pre divider.
 *
 * @map: map for setting.
 * @reg: register.
 * @sft: shift.
 *
 * Return the value of pre divider from given register value.
 * Return negative error code for unexpected register value.
 */
#[no_mangle]
pub unsafe extern "C" fn rl6231_get_pre_div(
    map: *mut regmap,
    reg: c_uint,
    sft: c_int,
) -> c_int {
    let mut pd: c_int;
    let mut val: c_int = 0;

    regmap_read(map, reg, &mut val);

    val = (val >> sft) & 0x7;

    match val {
        0 | 1 | 2 | 3 => {
            pd = val + 1;
        }
        4 => {
            pd = 6;
        }
        5 => {
            pd = 8;
        }
        6 => {
            pd = 12;
        }
        7 => {
            pd = 16;
        }
        _ => {
            pd = -EINVAL;
        }
    }

    pd
}
// EXPORT_SYMBOL_GPL(rl6231_get_pre_div);

/**
 * rl6231_calc_dmic_clk - Calculate the frequency divider parameter of dmic.
 *
 * @rate: base clock rate.
 *
 * Choose divider parameter that gives the highest possible DMIC frequency in
 * 1MHz - 3MHz range.
 */
#[no_mangle]
pub unsafe extern "C" fn rl6231_calc_dmic_clk(rate: c_int) -> c_int {
    static DIV: [c_int; 6] = [2, 3, 4, 6, 8, 12];
    let mut i: usize;

    if rate < 1000000 * DIV[0] {
        pr_warn(b"Base clock rate %d is too low\n\0".as_ptr(), rate);
        return -EINVAL;
    }

    i = 0;
    while i < DIV.len() {
        if (DIV[i] % 3) == 0 {
            i += 1;
            continue;
        }
        /* find divider that gives DMIC frequency below 1.536MHz */
        if 1536000 * DIV[i] >= rate {
            return i as c_int;
        }
        i += 1;
    }

    pr_warn(b"Base clock rate %d is too high\n\0".as_ptr(), rate);
    -EINVAL
}
// EXPORT_SYMBOL_GPL(rl6231_calc_dmic_clk);

#[repr(C)]
struct pll_calc_map {
    pll_in: c_uint,
    pll_out: c_uint,
    k: c_int,
    n: c_int,
    m: c_int,
    m_bp: bool,
    k_bp: bool,
}

static PLL_PRESET_TABLE: [pll_calc_map; 5] = [
    pll_calc_map {
        pll_in: 19200000,
        pll_out: 4096000,
        k: 23,
        n: 14,
        m: 1,
        m_bp: false,
        k_bp: false,
    },
    pll_calc_map {
        pll_in: 19200000,
        pll_out: 24576000,
        k: 3,
        n: 30,
        m: 3,
        m_bp: false,
        k_bp: false,
    },
    pll_calc_map {
        pll_in: 48000000,
        pll_out: 3840000,
        k: 23,
        n: 2,
        m: 0,
        m_bp: false,
        k_bp: false,
    },
    pll_calc_map {
        pll_in: 3840000,
        pll_out: 24576000,
        k: 3,
        n: 30,
        m: 0,
        m_bp: true,
        k_bp: false,
    },
    pll_calc_map {
        pll_in: 3840000,
        pll_out: 22579200,
        k: 3,
        n: 5,
        m: 0,
        m_bp: true,
        k_bp: false,
    },
];

unsafe fn find_best_div(in_: c_uint, max: c_uint, div: c_uint) -> c_uint {
    let mut d: c_uint;

    if in_ <= max {
        return 1;
    }

    d = in_ / max;
    if in_ % max != 0 {
        d += 1;
    }

    while div % d != 0 {
        d += 1;
    }

    d
}

/**
 * rl6231_pll_calc - Calcualte PLL M/N/K code.
 * @freq_in: external clock provided to codec.
 * @freq_out: target clock which codec works on.
 * @pll_code: Pointer to structure with M, N, K, m_bypass and k_bypass flag.
 *
 * Calcualte M/N/K code to configure PLL for codec.
 *
 * Returns 0 for success or negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn rl6231_pll_calc(
    freq_in: c_uint,
    freq_out: c_uint,
    pll_code: *mut rl6231_pll_code,
) -> c_int {
    let max_n: c_int = RL6231_PLL_N_MAX;
    let max_m: c_int = RL6231_PLL_M_MAX;
    let mut i: usize;
    let mut k: c_int;
    let mut n_t: c_int;
    let mut k_t: c_int;
    let mut min_k: c_int;
    let mut max_k: c_int;
    let mut n: c_int = 0;
    let mut m: c_int = 0;
    let mut m_t: c_int;
    let mut red: c_uint;
    let mut pll_out: c_uint;
    let mut in_t: c_uint;
    let mut out_t: c_uint;
    let div: c_uint;
    let div_t: c_uint;
    let mut red_t: c_uint = freq_out.abs_diff(freq_in);
    let f_in: c_uint;
    let f_out: c_uint;
    let f_max: c_uint;
    let mut m_bypass: bool = false;
    let mut k_bypass: bool = false;

    if RL6231_PLL_INP_MAX < freq_in || RL6231_PLL_INP_MIN > freq_in {
        return -EINVAL;
    }

    i = 0;
    while i < PLL_PRESET_TABLE.len() {
        if freq_in == PLL_PRESET_TABLE[i].pll_in && freq_out == PLL_PRESET_TABLE[i].pll_out {
            k = PLL_PRESET_TABLE[i].k;
            m = PLL_PRESET_TABLE[i].m;
            n = PLL_PRESET_TABLE[i].n;
            m_bypass = PLL_PRESET_TABLE[i].m_bp;
            k_bypass = PLL_PRESET_TABLE[i].k_bp;
            pr_debug(b"Use preset PLL parameter table\n\0".as_ptr());
            goto_code_find(
                k, m_bypass, k_bypass, m, n, pll_code,
            );
            return 0;
        }
        i += 1;
    }

    min_k = (80000000u32 / freq_out) as c_int - 2;
    max_k = (150000000u32 / freq_out) as c_int - 2;
    if max_k > RL6231_PLL_K_MAX {
        max_k = RL6231_PLL_K_MAX;
    }
    if min_k > RL6231_PLL_K_MAX {
        min_k = RL6231_PLL_K_MAX;
        max_k = RL6231_PLL_K_MAX;
    }
    div_t = gcd(freq_in, freq_out);
    f_max = 0xffffffffu32 / RL6231_PLL_N_MAX as c_uint;
    div = find_best_div(freq_in, f_max, div_t);
    f_in = freq_in / div;
    f_out = freq_out / div;
    k = min_k;
    if min_k < -1 {
        min_k = -1;
    }
    k_t = min_k;
    'outer: while k_t <= max_k {
        n_t = 0;
        while n_t <= max_n {
            in_t = f_in.wrapping_mul((n_t + 2) as c_uint);
            pll_out = f_out.wrapping_mul((k_t + 2) as c_uint);
            if in_t == pll_out {
                m_bypass = true;
                n = n_t;
                k = k_t;
                break 'outer;
            }
            out_t = in_t / (k_t + 2) as c_uint;
            red = f_out.abs_diff(out_t);
            if red < red_t {
                m_bypass = true;
                n = n_t;
                m = 0;
                k = k_t;
                if red == 0 {
                    break 'outer;
                }
                red_t = red;
            }
            m_t = 0;
            while m_t <= max_m {
                out_t = in_t / ((m_t + 2) * (k_t + 2)) as c_uint;
                red = f_out.abs_diff(out_t);
                if red < red_t {
                    m_bypass = false;
                    n = n_t;
                    m = m_t;
                    k = k_t;
                    if red == 0 {
                        break 'outer;
                    }
                    red_t = red;
                }
                m_t += 1;
            }
            n_t += 1;
        }
        k_t += 1;
    }
    if !(k_t <= max_k && n_t <= max_n) {
        pr_debug(b"Only get approximation about PLL\n\0".as_ptr());
    }

    goto_code_find(k, m_bypass, k_bypass, m, n, pll_code);
    0
}
// EXPORT_SYMBOL_GPL(rl6231_pll_calc);

unsafe fn goto_code_find(
    mut k: c_int,
    m_bypass: bool,
    mut k_bypass: bool,
    m: c_int,
    n: c_int,
    pll_code: *mut rl6231_pll_code,
) {
    if k == -1 {
        k_bypass = true;
        k = 0;
    }

    (*pll_code).m_bp = m_bypass;
    (*pll_code).k_bp = k_bypass;
    (*pll_code).m_code = m;
    (*pll_code).n_code = n;
    (*pll_code).k_code = k;
}

#[no_mangle]
pub unsafe extern "C" fn rl6231_get_clk_info(sclk: c_int, mut rate: c_int) -> c_int {
    let mut i: usize;
    static PD: [c_int; 8] = [1, 2, 3, 4, 6, 8, 12, 16];

    if sclk <= 0 || rate <= 0 {
        return -EINVAL;
    }

    rate = rate << 8;
    i = 0;
    while i < PD.len() {
        if sclk == rate * PD[i] {
            return i as c_int;
        }
        i += 1;
    }

    -EINVAL
}
// EXPORT_SYMBOL_GPL(rl6231_get_clk_info);

// MODULE_DESCRIPTION("RL6231 class device shared support");
// MODULE_AUTHOR("Oder Chiou <oder_chiou@realtek.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
