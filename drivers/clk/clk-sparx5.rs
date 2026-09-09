// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Microchip Sparx5 SoC Clock driver.
 *
 * Copyright (c) 2019 Microchip Inc.
 *
 * Author: Lars Povlsen <lars.povlsen@microchip.com>
 */

// Linux kernel dependencies are supplied by other translation units.

const PLL_DIV: u32 = 0xff;
const PLL_PRE_DIV: u32 = 0x700;
const PLL_ROT_DIR: u32 = 1 << 11;
const PLL_ROT_SEL: u32 = 0x3000;
const PLL_ROT_ENA: u32 = 1 << 14;
const PLL_CLK_ENA: u32 = 1 << 15;

const MAX_SEL: usize = 4;
const MAX_PRE: usize = 1 << 3;

static SEL_RATES: [u8; MAX_SEL] = [0, 2 * 8, 2 * 4, 2 * 2];

static CLK_NAMES: [&str; N_CLOCKS] = [
    "core", "ddr", "cpu2", "arm2", "aux1", "aux2", "aux3", "aux4", "synce",
];

#[repr(C)]
pub struct S5HwClk {
    pub hw: ClkHw,
    pub reg: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct S5ClkData {
    pub base: *mut core::ffi::c_void,
    pub s5_hw: [S5HwClk; N_CLOCKS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct S5PllConf {
    pub freq: usize,
    pub div: u8,
    pub rot_ena: bool,
    pub rot_sel: u8,
    pub rot_dir: u8,
    pub pre_div: u8,
}

unsafe fn s5_calc_freq(parent_rate: usize, conf: *const S5PllConf) -> usize {
    let mut rate = parent_rate / (*conf).div as usize;

    if (*conf).rot_ena {
        let sign: isize = if (*conf).rot_dir != 0 { -1 } else { 1 };
        let divt = SEL_RATES[(*conf).rot_sel as usize] as isize * (1 + (*conf).pre_div as isize);
        let divb = divt + sign;
        rate = ((rate as isize * divt) / divb) as usize;
        rate = ((rate + 999) / 1000) * 1000;
    }

    rate
}

unsafe fn s5_search_fractional(rate: usize, parent_rate: usize, div: i32, conf: *mut S5PllConf) {
    let mut best = core::mem::zeroed::<S5PllConf>();
    let mut best_offset = rate;

    *conf = core::mem::zeroed();
    (*conf).div = div as u8;
    (*conf).rot_ena = true;

    for d in 0..=1 {
        if best_offset == 0 { break; }
        (*conf).rot_dir = (d != 0) as u8;
        for i in 0..MAX_PRE {
            if best_offset == 0 { break; }
            (*conf).pre_div = i as u8;
            for j in 1..MAX_SEL {
                if best_offset == 0 { break; }
                (*conf).rot_sel = j as u8;
                (*conf).freq = s5_calc_freq(parent_rate, conf);
                let cur_offset = rate.abs_diff((*conf).freq);
                if cur_offset < best_offset {
                    best_offset = cur_offset;
                    best = *conf;
                }
            }
        }
    }

    *conf = best;
}

unsafe fn s5_calc_params(rate: usize, parent_rate: usize, conf: *mut S5PllConf) -> usize {
    if parent_rate % rate != 0 {
        let mut alt1: S5PllConf = core::mem::zeroed();
        let mut alt2: S5PllConf = core::mem::zeroed();
        let mut div = ((parent_rate + rate / 2) / rate) as i32;
        s5_search_fractional(rate, parent_rate, div, &mut alt1);
        if alt1.freq == rate {
            *conf = alt1;
        } else {
            div = (parent_rate / rate) as i32;
            if div != alt1.div as i32 {
                s5_search_fractional(rate, parent_rate, div, &mut alt2);
                if rate.abs_diff(alt1.freq) < rate.abs_diff(alt2.freq) {
                    *conf = alt1;
                } else {
                    *conf = alt2;
                }
            }
        }
    } else {
        *conf = core::mem::zeroed();
        (*conf).div = (parent_rate / rate) as u8;
    }
    (*conf).freq
}

unsafe fn s5_pll_enable(hw: *mut ClkHw) -> i32 {
    let pll = hw as *mut S5HwClk;
    let mut val = readl((*pll).reg);
    val |= PLL_CLK_ENA;
    writel(val, (*pll).reg);
    0
}

unsafe fn s5_pll_disable(hw: *mut ClkHw) {
    let pll = hw as *mut S5HwClk;
    let mut val = readl((*pll).reg);
    val &= !PLL_CLK_ENA;
    writel(val, (*pll).reg);
}

unsafe fn s5_pll_set_rate(hw: *mut ClkHw, rate: usize, parent_rate: usize) -> i32 {
    let pll = hw as *mut S5HwClk;
    let mut conf = core::mem::zeroed::<S5PllConf>();
    let eff_rate = s5_calc_params(rate, parent_rate, &mut conf);
    if eff_rate != rate { return -EOPNOTSUPP; }

    let mut val = readl((*pll).reg) & PLL_CLK_ENA;
    val |= ((conf.div as u32) << 0) & PLL_DIV;
    if conf.rot_ena {
        val |= PLL_ROT_ENA;
        val |= ((conf.rot_sel as u32) << 12) & PLL_ROT_SEL;
        val |= ((conf.pre_div as u32) << 8) & PLL_PRE_DIV;
        if conf.rot_dir != 0 { val |= PLL_ROT_DIR; }
    }
    writel(val, (*pll).reg);
    0
}

unsafe fn s5_pll_recalc_rate(hw: *mut ClkHw, parent_rate: usize) -> usize {
    let pll = hw as *mut S5HwClk;
    let val = readl((*pll).reg);
    if val & PLL_CLK_ENA != 0 {
        let mut conf = core::mem::zeroed::<S5PllConf>();
        conf.div = (val & PLL_DIV) as u8;
        conf.pre_div = ((val & PLL_PRE_DIV) >> 8) as u8;
        conf.rot_ena = val & PLL_ROT_ENA != 0;
        conf.rot_dir = ((val & PLL_ROT_DIR) != 0) as u8;
        conf.rot_sel = ((val & PLL_ROT_SEL) >> 12) as u8;
        conf.freq = s5_calc_freq(parent_rate, &conf);
        conf.freq
    } else { 0 }
}

unsafe fn s5_pll_determine_rate(_hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let mut conf = core::mem::zeroed::<S5PllConf>();
    (*req).rate = s5_calc_params((*req).rate, (*req).best_parent_rate, &mut conf);
    0
}

// The remaining driver registration and platform-provider wiring depend on the
// Linux clock, device-tree, and platform APIs supplied by other files.
const N_CLOCKS: usize = 9;

extern "C" {
    static EOPNOTSUPP: i32;
    fn readl(reg: *mut core::ffi::c_void) -> u32;
    fn writel(val: u32, reg: *mut core::ffi::c_void);
}

#[repr(C)] pub struct ClkHw { _private: [u8; 0] }
#[repr(C)] pub struct ClkRateRequest { pub rate: usize, pub best_parent_rate: usize }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
