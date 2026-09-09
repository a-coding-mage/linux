// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 Nuvoton Technology Corp.
 * Author: Chi-Fang Li <cfli0@nuvoton.com>
 */

// Kernel dependencies are supplied by the surrounding Rust translation.

const PLL_FREF_MAX_FREQ: u64 = 200 * HZ_PER_MHZ;
const PLL_FREF_MIN_FREQ: u64 = 1 * HZ_PER_MHZ;
const PLL_FREF_M_MAX_FREQ: u64 = 40 * HZ_PER_MHZ;
const PLL_FREF_M_MIN_FREQ: u64 = 10 * HZ_PER_MHZ;
const PLL_FCLK_MAX_FREQ: u64 = 2400 * HZ_PER_MHZ;
const PLL_FCLK_MIN_FREQ: u64 = 600 * HZ_PER_MHZ;
const PLL_FCLKO_MAX_FREQ: u64 = 2400 * HZ_PER_MHZ;
const PLL_FCLKO_MIN_FREQ: u64 = 85700 * HZ_PER_KHZ;
const PLL_SS_RATE: u32 = 0x77;
const PLL_SLOPE: u32 = 0x58CFA;

const REG_PLL_CTL0_OFFSET: usize = 0x0;
const REG_PLL_CTL1_OFFSET: usize = 0x4;
const REG_PLL_CTL2_OFFSET: usize = 0x8;

const SPLL0_CTL0_FBDIV: u32 = 0xff;
const SPLL0_CTL0_INDIV: u32 = 0xf00;
const SPLL0_CTL0_OUTDIV: u32 = 0x3000;
const SPLL0_CTL0_PD: u32 = 1 << 16;
const SPLL0_CTL0_BP: u32 = 1 << 17;

const PLL_CTL0_FBDIV: u32 = 0x7ff;
const PLL_CTL0_INDIV: u32 = 0x3f000;
const PLL_CTL0_MODE: u32 = 0xc0000;
const PLL_CTL0_SSRATE: u32 = 0x7ff00000;
const PLL_CTL1_PD: u32 = 1;
const PLL_CTL1_BP: u32 = 1 << 1;
const PLL_CTL1_OUTDIV: u32 = 0x70;
const PLL_CTL1_FRAC: u32 = 0xffffff00;
const PLL_CTL2_SLOPE: u32 = 0xffffff;

const INDIV_MIN: i32 = 1;
const INDIV_MAX: i32 = 63;
const FBDIV_MIN: i32 = 16;
const FBDIV_MAX: i32 = 2047;
const FBDIV_FRAC_MIN: i32 = 1600;
const FBDIV_FRAC_MAX: i32 = 204700;
const OUTDIV_MIN: i32 = 1;
const OUTDIV_MAX: i32 = 7;

const PLL_MODE_INT: u8 = 0;
const PLL_MODE_FRAC: u8 = 1;
const PLL_MODE_SS: u8 = 2;

#[repr(C)]
pub struct Ma35d1ClkPll {
    pub hw: ClkHw,
    pub id: u32,
    pub mode: u8,
    pub ctl0_base: *mut core::ffi::c_void,
    pub ctl1_base: *mut core::ffi::c_void,
    pub ctl2_base: *mut core::ffi::c_void,
}

#[inline]
unsafe fn to_ma35d1_clk_pll(hw: *mut ClkHw) -> *mut Ma35d1ClkPll {
    (hw as *mut u8).sub(core::mem::offset_of!(Ma35d1ClkPll, hw)) as *mut Ma35d1ClkPll
}

unsafe fn ma35d1_calc_smic_pll_freq(ctl: u32, parent: u64) -> u64 {
    if ctl & SPLL0_CTL0_BP != 0 { return parent; }
    let n = (ctl & SPLL0_CTL0_FBDIV) as u64;
    let m = ((ctl & SPLL0_CTL0_INDIV) >> 8) as u64;
    let p = ((ctl & SPLL0_CTL0_OUTDIV) >> 12) as u32;
    parent * n / (m * (1u64 << p))
}

unsafe fn ma35d1_calc_pll_freq(mode: u8, reg: *const u32, parent: u64) -> u64 {
    if *reg.add(1) & PLL_CTL1_BP != 0 { return parent; }
    let n = (*reg & PLL_CTL0_FBDIV) as u64;
    let m = ((*reg & PLL_CTL0_INDIV) >> 12) as u64;
    let p = ((*reg.add(1) & PLL_CTL1_OUTDIV) >> 4) as u64;
    if mode == PLL_MODE_INT { parent * n / (m * p) } else {
        let x = ((*reg.add(1) & PLL_CTL1_FRAC) >> 8) as u64;
        let n = n * 1000 + (x * 1000 + (1u64 << 23)) / (1u64 << 24);
        parent * n / (1000 * m * p)
    }
}

unsafe fn ma35d1_pll_find_closest(pll: *mut Ma35d1ClkPll, rate: u64, parent: u64,
                                  reg: *mut u32, freq: *mut u64) -> i32 {
    *freq = 0;
    if rate < PLL_FCLKO_MIN_FREQ || rate > PLL_FCLKO_MAX_FREQ { return -22; }
    let (fmin, fmax) = if (*pll).mode == PLL_MODE_INT { (FBDIV_MIN, FBDIV_MAX) } else { (FBDIV_FRAC_MIN, FBDIV_FRAC_MAX) };
    let mut min_diff = u64::MAX;
    let mut m = INDIV_MIN;
    while m <= INDIV_MAX { let mut n = fmin;
        while n <= fmax { let mut p = OUTDIV_MIN;
            while p <= OUTDIV_MAX {
                let tmp = parent / m as u64;
                if tmp >= PLL_FREF_M_MIN_FREQ && tmp <= PLL_FREF_M_MAX_FREQ {
                    let mut fclk = parent * n as u64 / m as u64;
                    if (*pll).mode != PLL_MODE_INT { fclk /= 100; }
                    if fclk >= PLL_FCLK_MIN_FREQ && fclk <= PLL_FCLK_MAX_FREQ {
                        let fout = fclk / p as u64;
                        if fout >= PLL_FCLKO_MIN_FREQ && fout <= PLL_FCLKO_MAX_FREQ {
                            let diff = if rate > fout { rate - fout } else { fout - rate };
                            if diff < min_diff { *reg = ((*reg) & !PLL_CTL0_INDIV) | ((m as u32) << 12) | ((*reg) & !PLL_CTL0_FBDIV) | n as u32; *reg.add(1) = ((p as u32) << 4); *freq = fout; min_diff = diff; if diff == 0 { break; } }
                        }
                    }
                }
                p += 1;
            } n += 1;
        } m += 1;
    }
    if *freq == 0 { -22 } else { 0 }
}

// Kernel-facing operations and registration use the corresponding external kernel bindings.
pub unsafe fn ma35d1_reg_clk_pll(dev: *mut Device, id: u32, mode: u8, name: *const i8,
                                 parent_hw: *mut ClkHw, base: *mut core::ffi::c_void) -> *mut ClkHw {
    let pll = devm_kzalloc(dev, core::mem::size_of::<Ma35d1ClkPll>(), GFP_KERNEL) as *mut Ma35d1ClkPll;
    if pll.is_null() { return ERR_PTR(-12); }
    (*pll).id = id; (*pll).mode = mode;
    (*pll).ctl0_base = (base as *mut u8).add(REG_PLL_CTL0_OFFSET) as *mut _;
    (*pll).ctl1_base = (base as *mut u8).add(REG_PLL_CTL1_OFFSET) as *mut _;
    (*pll).ctl2_base = (base as *mut u8).add(REG_PLL_CTL2_OFFSET) as *mut _;
    (*pll).hw.init = clk_init_for_pll(name, parent_hw, id == CAPLL || id == DDRPLL, &mut *pll);
    let ret = devm_clk_hw_register(dev, &mut (*pll).hw);
    if ret != 0 { return ERR_PTR(ret); }
    &mut (*pll).hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
