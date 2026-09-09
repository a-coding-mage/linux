// SPDX-License-Identifier: GPL-2.0+
/*
 * Based on drivers/clk/tegra/clk-emc.c
 * Copyright (c) 2014, NVIDIA CORPORATION.  All rights reserved.
 *
 * Author: Dmitry Osipenko <digetx@gmail.com>
 * Copyright (C) 2019 GRATE-DRIVER project
 */

// pr_fmt(fmt) = "tegra-emc-clk: " fmt

use core::ffi::c_void;

const CLK_SOURCE_EMC_2X_CLK_DIVISOR_MASK: u32 = 0xff;
const CLK_SOURCE_EMC_2X_CLK_SRC_MASK: u32 = 0xc000_0000;
const CLK_SOURCE_EMC_2X_CLK_SRC_SHIFT: u32 = 30;

const MC_EMC_SAME_FREQ: u32 = 1 << 16;
const USE_PLLM_UD: u32 = 1 << 29;

const EMC_SRC_PLL_M: u8 = 0;
const EMC_SRC_PLL_C: u8 = 1;
const EMC_SRC_PLL_P: u8 = 2;
const EMC_SRC_CLK_M: u8 = 3;

static EMC_PARENT_CLK_NAMES: [&str; 4] = ["pll_m", "pll_c", "pll_p", "clk_m"];

#[repr(C)]
pub struct clk_hw {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: libc::c_ulong,
    pub min_rate: libc::c_ulong,
    pub max_rate: libc::c_ulong,
    pub best_parent_rate: libc::c_ulong,
    pub best_parent_hw: *mut clk_hw,
}

pub type tegra20_clk_emc_round_cb = unsafe extern "C" fn(
    libc::c_ulong,
    libc::c_ulong,
    libc::c_ulong,
    *mut c_void,
) -> libc::c_long;

#[repr(C)]
struct tegra_clk_emc {
    hw: clk_hw,
    reg: *mut c_void,
    mc_same_freq: bool,
    want_low_jitter: bool,
    round_cb: Option<tegra20_clk_emc_round_cb>,
    cb_arg: *mut c_void,
}

extern "C" {
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn fence_udelay(delay: u32, reg: *mut c_void);
    fn div_frac_get(rate: libc::c_ulong, parent_rate: libc::c_ulong,
                    shift: u32, round: u32, flags: u32) -> u32;
    fn clk_hw_get_parent_by_index(hw: *mut clk_hw, index: u32) -> *mut clk_hw;
    fn clk_hw_get_rate(hw: *mut clk_hw) -> libc::c_ulong;
    fn __clk_lookup(name: *const libc::c_char) -> *mut clk;
    fn __clk_get_hw(clk: *mut clk) -> *mut clk_hw;
    fn clk_register(dev: *mut c_void, hw: *mut clk_hw) -> *mut clk;
    fn kzalloc(size: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn pr_err_once(fmt: *const libc::c_char, ...);
}

unsafe fn to_tegra_clk_emc(hw: *mut clk_hw) -> *mut tegra_clk_emc {
    hw as *mut tegra_clk_emc
}

unsafe fn emc_recalc_rate(hw: *mut clk_hw, parent_rate: libc::c_ulong) -> libc::c_ulong {
    let emc = to_tegra_clk_emc(hw);
    let val = readl_relaxed((*emc).reg);
    let div = val & CLK_SOURCE_EMC_2X_CLK_DIVISOR_MASK;
    (parent_rate * 2 + (div as libc::c_ulong + 2) - 1) / (div as libc::c_ulong + 2)
}

unsafe fn emc_get_parent(hw: *mut clk_hw) -> u8 {
    let emc = to_tegra_clk_emc(hw);
    (readl_relaxed((*emc).reg) >> CLK_SOURCE_EMC_2X_CLK_SRC_SHIFT) as u8
}

unsafe fn emc_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let emc = to_tegra_clk_emc(hw);
    let mut val = readl_relaxed((*emc).reg);
    val &= !CLK_SOURCE_EMC_2X_CLK_SRC_MASK;
    val |= (index as u32) << CLK_SOURCE_EMC_2X_CLK_SRC_SHIFT;
    let div = val & CLK_SOURCE_EMC_2X_CLK_DIVISOR_MASK;
    if index == EMC_SRC_PLL_M && div == 0 && (*emc).want_low_jitter { val |= USE_PLLM_UD; } else { val &= !USE_PLLM_UD; }
    if (*emc).mc_same_freq { val |= MC_EMC_SAME_FREQ; } else { val &= !MC_EMC_SAME_FREQ; }
    writel_relaxed(val, (*emc).reg);
    fence_udelay(1, (*emc).reg);
    0
}

unsafe fn emc_set_rate(hw: *mut clk_hw, rate: libc::c_ulong, parent_rate: libc::c_ulong) -> i32 {
    let emc = to_tegra_clk_emc(hw);
    let div = div_frac_get(rate, parent_rate, 8, 1, 0);
    let mut val = readl_relaxed((*emc).reg);
    val &= !CLK_SOURCE_EMC_2X_CLK_DIVISOR_MASK;
    val |= div;
    let index = (val >> CLK_SOURCE_EMC_2X_CLK_SRC_SHIFT) as u8;
    if index == EMC_SRC_PLL_M && div == 0 && (*emc).want_low_jitter { val |= USE_PLLM_UD; } else { val &= !USE_PLLM_UD; }
    if (*emc).mc_same_freq { val |= MC_EMC_SAME_FREQ; } else { val &= !MC_EMC_SAME_FREQ; }
    writel_relaxed(val, (*emc).reg);
    fence_udelay(1, (*emc).reg);
    0
}

unsafe fn emc_set_rate_and_parent(hw: *mut clk_hw, rate: libc::c_ulong, parent_rate: libc::c_ulong, index: u8) -> i32 {
    let emc = to_tegra_clk_emc(hw);
    let div = div_frac_get(rate, parent_rate, 8, 1, 0);
    let mut val = readl_relaxed((*emc).reg);
    val &= !CLK_SOURCE_EMC_2X_CLK_SRC_MASK;
    val |= (index as u32) << CLK_SOURCE_EMC_2X_CLK_SRC_SHIFT;
    val &= !CLK_SOURCE_EMC_2X_CLK_DIVISOR_MASK;
    val |= div;
    if index == EMC_SRC_PLL_M && div == 0 && (*emc).want_low_jitter { val |= USE_PLLM_UD; } else { val &= !USE_PLLM_UD; }
    if (*emc).mc_same_freq { val |= MC_EMC_SAME_FREQ; } else { val &= !MC_EMC_SAME_FREQ; }
    writel_relaxed(val, (*emc).reg);
    fence_udelay(1, (*emc).reg);
    0
}

unsafe fn emc_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let emc = to_tegra_clk_emc(hw);
    let cb = match (*emc).round_cb { Some(cb) => cb, None => return -22 };
    let emc_rate = cb((*req).rate, (*req).min_rate, (*req).max_rate, (*emc).cb_arg);
    if emc_rate < 0 { return emc_rate as i32; }
    for i in 0..EMC_PARENT_CLK_NAMES.len() {
        let parent_hw = clk_hw_get_parent_by_index(hw, i as u32);
        let parent_rate = if (*req).best_parent_hw == parent_hw {
            (*req).best_parent_rate
        } else {
            clk_hw_get_rate(parent_hw)
        };
        if emc_rate as libc::c_ulong > parent_rate { continue; }
        let div = div_frac_get(emc_rate as libc::c_ulong, parent_rate, 8, 1, 0);
        let divided_rate = (parent_rate * 2 + (div as libc::c_ulong + 2) - 1) / (div as libc::c_ulong + 2);
        if divided_rate != emc_rate as libc::c_ulong { continue; }
        (*req).best_parent_rate = parent_rate;
        (*req).best_parent_hw = parent_hw;
        (*req).rate = emc_rate as libc::c_ulong;
        return 0;
    }
    -22
}

// Equivalent to the C clk_ops table; callbacks are retained as file-local functions above.

// The remaining clock-framework structures and registration helpers are supplied externally.
pub unsafe fn tegra20_clk_set_emc_round_callback(round_cb: Option<tegra20_clk_emc_round_cb>, cb_arg: *mut c_void) {
    let name = b"emc\0";
    let clk = __clk_lookup(name.as_ptr() as *const libc::c_char);
    if !clk.is_null() {
        let hw = __clk_get_hw(clk);
        let emc = to_tegra_clk_emc(hw);
        (*emc).round_cb = round_cb;
        (*emc).cb_arg = cb_arg;
    }
}

pub unsafe fn tegra20_clk_emc_driver_available(emc_hw: *mut clk_hw) -> bool {
    (*to_tegra_clk_emc(emc_hw)).round_cb.is_some()
}

pub unsafe fn tegra20_clk_register_emc(ioaddr: *mut c_void, low_jitter: bool) -> *mut clk {
    let emc = kzalloc(core::mem::size_of::<tegra_clk_emc>()) as *mut tegra_clk_emc;
    if emc.is_null() { return core::ptr::null_mut(); }
    (*emc).reg = ioaddr;
    (*emc).want_low_jitter = low_jitter;
    let clk = clk_register(core::ptr::null_mut(), &mut (*emc).hw);
    if clk.is_null() { kfree(emc as *mut c_void); return core::ptr::null_mut(); }
    clk
}

pub unsafe fn tegra20_clk_prepare_emc_mc_same_freq(emc_clk: *mut clk, same: bool) -> i32 {
    if emc_clk.is_null() { return -22; }
    let emc = to_tegra_clk_emc(__clk_get_hw(emc_clk));
    (*emc).mc_same_freq = same;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
