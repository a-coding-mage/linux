// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017~2018 NXP
 *
 * Author: Dong Aisheng <aisheng.dong@nxp.com>
 *
 */

// Dependencies supplied by the surrounding kernel translation.

const PLL_CSR_OFFSET: usize = 0x0;
const PLL_VLD: u32 = 1 << 24;
const PLL_EN: u32 = 1 << 0;

const PLL_CFG_OFFSET: u32 = 0x08;
const IMX8ULP_PLL_CFG_OFFSET: u32 = 0x10;
const BP_PLL_MULT: u32 = 16;
const BM_PLL_MULT: u32 = 0x7f << 16;

const PLL_NUM_OFFSET: u32 = 0x10;
const IMX8ULP_PLL_NUM_OFFSET: u32 = 0x1c;

const PLL_DENOM_OFFSET: u32 = 0x14;
const IMX8ULP_PLL_DENOM_OFFSET: u32 = 0x18;

const MAX_MFD: u32 = 0x3fffffff;
const DEFAULT_MFD: u32 = 1000000;

#[repr(C)]
struct clk_pllv4 {
    hw: clk_hw,
    base: *mut core::ffi::c_void,
    cfg_offset: u32,
    num_offset: u32,
    denom_offset: u32,
    use_mult_range: bool,
}

static PLLV4_MULT_TABLE: [i32; 6] = [33, 27, 22, 20, 17, 16];
static PLLV4_MULT_RANGE: [i32; 2] = [54, 27];

const LOCK_TIMEOUT_US: u32 = USEC_PER_MSEC;

unsafe extern "C" {
    type clk_hw;
    type clk_rate_request;
    type clk_init_data;

    static clk_pllv4_ops: clk_ops;

    fn readl_poll_timeout(
        addr: *mut core::ffi::c_void,
        val: *mut u32,
        cond: u32,
        delay_us: u32,
        timeout_us: u32,
    ) -> i32;
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn clk_hw_get_name(hw: *mut clk_hw) -> *const core::ffi::c_char;
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> i32;
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
}

const USEC_PER_MSEC: u32 = 1000;

#[repr(C)]
struct clk_ops {
    recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32>,
    prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
    is_prepared: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
}

#[repr(C)]
struct imx_pllv4_type;

const IMX_PLLV4_IMX8ULP: imx_pllv4_type = unsafe { core::mem::zeroed() };
const IMX_PLLV4_IMX8ULP_1GHZ: imx_pllv4_type = unsafe { core::mem::zeroed() };

unsafe fn clk_pllv4_wait_lock(pll: *mut clk_pllv4) -> i32 {
    let mut csr = 0u32;
    readl_poll_timeout(
        (*pll).base.add(PLL_CSR_OFFSET),
        &mut csr,
        csr & PLL_VLD,
        0,
        LOCK_TIMEOUT_US,
    )
}

unsafe extern "C" fn clk_pllv4_is_prepared(hw: *mut clk_hw) -> i32 {
    let pll = hw as *mut clk_pllv4;
    if readl_relaxed((*pll).base) & PLL_EN != 0 { 1 } else { 0 }
}

unsafe extern "C" fn clk_pllv4_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let pll = hw as *mut clk_pllv4;
    let mut mult = readl_relaxed((*pll).base.add((*pll).cfg_offset as usize));
    mult = (mult & BM_PLL_MULT) >> BP_PLL_MULT;
    let mfn = readl_relaxed((*pll).base.add((*pll).num_offset as usize));
    let mfd = readl_relaxed((*pll).base.add((*pll).denom_offset as usize));
    let temp64 = ((parent_rate as u64) * mfn as u64) / mfd as u64;
    parent_rate * mult as usize + temp64 as u32 as usize
}

unsafe extern "C" fn clk_pllv4_determine_rate(_hw: *mut clk_hw, _req: *mut clk_rate_request) -> i32 { 0 }

unsafe extern "C" fn clk_pllv4_set_rate(_hw: *mut clk_hw, _rate: usize, _parent_rate: usize) -> i32 { 0 }

unsafe extern "C" fn clk_pllv4_prepare(_hw: *mut clk_hw) -> i32 { 0 }

unsafe extern "C" fn clk_pllv4_unprepare(_hw: *mut clk_hw) {}

#[no_mangle]
pub unsafe extern "C" fn imx_clk_hw_pllv4(
    _type_: imx_pllv4_type,
    _name: *const core::ffi::c_char,
    _parent_name: *const core::ffi::c_char,
    _base: *mut core::ffi::c_void,
) -> *mut clk_hw {
    // The remaining kernel clock framework types and helpers are supplied externally.
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
