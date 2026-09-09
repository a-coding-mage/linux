// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, 2013, NVIDIA CORPORATION.  All rights reserved.
 */

// External Linux kernel and Tegra declarations are supplied by other files.

const OSC_CTRL: usize = 0x50;
const OSC_CTRL_OSC_FREQ_SHIFT: u32 = 28;
const OSC_CTRL_PLL_REF_DIV_SHIFT: u32 = 26;
const OSC_CTRL_MASK: u32 = 0x3f2 | (0xf << OSC_CTRL_OSC_FREQ_SHIFT);

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tegra_clk {
    _private: [u8; 0],
}

extern "C" {
    static mut osc_ctrl_ctx: u32;

    fn readl_relaxed(addr: *const core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn fence_udelay(usecs: u32, clk_base: *mut core::ffi::c_void);
    fn tegra_lookup_dt_id(id: u32, clks: *mut tegra_clk) -> *mut *mut clk;
    fn clk_register_fixed_rate(
        dev: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char,
        flags: u32,
        rate: core::ffi::c_ulong,
    ) -> *mut clk;
    fn clk_register_fixed_factor(
        dev: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char,
        flags: u32,
        mult: u32,
        div: u32,
    ) -> *mut clk;
    fn warn_on(condition: i32) -> bool;

    static tegra_clk_osc: u32;
    static tegra_clk_osc_div2: u32;
    static tegra_clk_osc_div4: u32;
    static tegra_clk_clk_m: u32;
    static tegra_clk_pll_ref: u32;
    static tegra_clk_clk_32k: u32;
}

#[no_mangle]
pub unsafe extern "C" fn tegra_osc_clk_init(
    clk_base: *mut core::ffi::c_void,
    clks: *mut tegra_clk,
    input_freqs: *mut core::ffi::c_ulong,
    num: u32,
    clk_m_div: u32,
    osc_freq: *mut core::ffi::c_ulong,
    pll_ref_freq: *mut core::ffi::c_ulong,
) -> i32 {
    let mut clk: *mut clk;
    let mut osc: *mut clk;
    let mut dt_clk: *mut *mut clk;
    let mut val: u32;
    let mut pll_ref_div: u32;
    let osc_idx: u32;

    val = readl_relaxed(clk_base.add(OSC_CTRL));
    osc_ctrl_ctx = val & OSC_CTRL_MASK;
    osc_idx = val >> OSC_CTRL_OSC_FREQ_SHIFT;

    if osc_idx < num {
        *osc_freq = *input_freqs.add(osc_idx as usize);
    } else {
        *osc_freq = 0;
    }

    if *osc_freq == 0 {
        warn_on(1);
        return -22;
    }

    dt_clk = tegra_lookup_dt_id(tegra_clk_osc, clks);
    if dt_clk.is_null() {
        return 0;
    }

    osc = clk_register_fixed_rate(core::ptr::null_mut(), c"osc".as_ptr(), core::ptr::null(), 0, *osc_freq);
    *dt_clk = osc;

    /* osc_div2 */
    dt_clk = tegra_lookup_dt_id(tegra_clk_osc_div2, clks);
    if !dt_clk.is_null() {
        clk = clk_register_fixed_factor(core::ptr::null_mut(), c"osc_div2".as_ptr(), c"osc".as_ptr(), 0, 1, 2);
        *dt_clk = clk;
    }

    /* osc_div4 */
    dt_clk = tegra_lookup_dt_id(tegra_clk_osc_div4, clks);
    if !dt_clk.is_null() {
        clk = clk_register_fixed_factor(core::ptr::null_mut(), c"osc_div4".as_ptr(), c"osc".as_ptr(), 0, 1, 4);
        *dt_clk = clk;
    }

    dt_clk = tegra_lookup_dt_id(tegra_clk_clk_m, clks);
    if dt_clk.is_null() {
        return 0;
    }

    clk = clk_register_fixed_factor(core::ptr::null_mut(), c"clk_m".as_ptr(), c"osc".as_ptr(), 0, 1, clk_m_div);
    *dt_clk = clk;

    /* pll_ref */
    val = (val >> OSC_CTRL_PLL_REF_DIV_SHIFT) & 3;
    pll_ref_div = 1 << val;
    dt_clk = tegra_lookup_dt_id(tegra_clk_pll_ref, clks);
    if dt_clk.is_null() {
        return 0;
    }

    clk = clk_register_fixed_factor(core::ptr::null_mut(), c"pll_ref".as_ptr(), c"osc".as_ptr(), 0, 1, pll_ref_div);
    *dt_clk = clk;

    if !pll_ref_freq.is_null() {
        *pll_ref_freq = *osc_freq / pll_ref_div as core::ffi::c_ulong;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn tegra_fixed_clk_init(tegra_clks: *mut tegra_clk) {
    let mut clk: *mut clk;
    let dt_clk: *mut *mut clk;

    /* clk_32k */
    let dt_clk = tegra_lookup_dt_id(tegra_clk_clk_32k, tegra_clks);
    if !dt_clk.is_null() {
        clk = clk_register_fixed_rate(core::ptr::null_mut(), c"clk_32k".as_ptr(), core::ptr::null(), 0, 32768);
        *dt_clk = clk;
    }
}

#[no_mangle]
pub unsafe extern "C" fn tegra_clk_osc_resume(clk_base: *mut core::ffi::c_void) {
    let mut val: u32;

    val = readl_relaxed(clk_base.add(OSC_CTRL)) & !OSC_CTRL_MASK;
    val |= osc_ctrl_ctx;
    writel_relaxed(val, clk_base.add(OSC_CTRL));
    fence_udelay(2, clk_base);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
