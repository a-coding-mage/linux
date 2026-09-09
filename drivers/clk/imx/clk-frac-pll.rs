// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2018 NXP.
 *
 * This driver supports the fractional plls found in the imx8m SOCs
 *
 * Documentation for this fractional pll can be found at:
 *   https://www.nxp.com/docs/en/reference-manual/IMX8MDQLQRM.pdf#page=834
 */

use core::ffi::c_void;

const PLL_CFG0: usize = 0x0;
const PLL_CFG1: usize = 0x4;

const PLL_LOCK_STATUS: u32 = 1u32 << 31;
const PLL_PD_MASK: u32 = 1u32 << 19;
const PLL_BYPASS_MASK: u32 = 1u32 << 14;
const PLL_NEWDIV_VAL: u32 = 1u32 << 12;
const PLL_NEWDIV_ACK: u32 = 1u32 << 11;
const PLL_FRAC_DIV_MASK: u32 = 0x7fffff80;
const PLL_INT_DIV_MASK: u32 = 0x7f;
const PLL_OUTPUT_DIV_MASK: u32 = 0x1f;
const PLL_FRAC_DENOM: u64 = 0x1000000;

const PLL_FRAC_LOCK_TIMEOUT: u32 = 10000;
const PLL_FRAC_ACK_TIMEOUT: u32 = 500000;

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const i8,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const i8,
    pub num_parents: u8,
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: u64,
    pub best_parent_rate: u64,
}

#[repr(C)]
pub struct clk_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_prepared: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, u64) -> u64>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, u64, u64) -> i32>,
}

#[repr(C)]
struct clk_frac_pll {
    hw: clk_hw,
    base: *mut u8,
}

extern "C" {
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn readl_poll_timeout(addr: *mut u8, val: *mut u32, condition: u32, delay: u32, timeout: u32) -> i32;
    fn clk_hw_register(dev: *mut c_void, hw: *mut clk_hw) -> i32;
    fn kfree(ptr: *mut clk_frac_pll);
    fn kzalloc_obj() -> *mut clk_frac_pll;
}

#[inline]
unsafe fn field_get(mask: u32, value: u32) -> u32 {
    (value & mask) >> mask.trailing_zeros()
}

unsafe fn clk_wait_lock(pll: *mut clk_frac_pll) -> i32 {
    let mut val = 0u32;
    readl_poll_timeout((*pll).base, &mut val, val & PLL_LOCK_STATUS, 0, PLL_FRAC_LOCK_TIMEOUT)
}

unsafe fn clk_wait_ack(pll: *mut clk_frac_pll) -> i32 {
    if readl_relaxed((*pll).base) & (PLL_PD_MASK | PLL_BYPASS_MASK) != 0 {
        return 0;
    }
    let mut val = 0u32;
    readl_poll_timeout((*pll).base, &mut val, val & PLL_NEWDIV_ACK, 0, PLL_FRAC_ACK_TIMEOUT)
}

unsafe extern "C" fn clk_pll_prepare(hw: *mut clk_hw) -> i32 {
    let pll = hw as *mut clk_frac_pll;
    let mut val = readl_relaxed((*pll).base.add(PLL_CFG0));
    val &= !PLL_PD_MASK;
    writel_relaxed(val, (*pll).base.add(PLL_CFG0));
    clk_wait_lock(pll)
}

unsafe extern "C" fn clk_pll_unprepare(hw: *mut clk_hw) {
    let pll = hw as *mut clk_frac_pll;
    let mut val = readl_relaxed((*pll).base.add(PLL_CFG0));
    val |= PLL_PD_MASK;
    writel_relaxed(val, (*pll).base.add(PLL_CFG0));
}

unsafe extern "C" fn clk_pll_is_prepared(hw: *mut clk_hw) -> i32 {
    let pll = hw as *mut clk_frac_pll;
    let val = readl_relaxed((*pll).base.add(PLL_CFG0));
    if val & PLL_PD_MASK != 0 { 0 } else { 1 }
}

unsafe extern "C" fn clk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let pll = hw as *mut clk_frac_pll;
    let mut val = readl_relaxed((*pll).base.add(PLL_CFG0));
    let divq = (field_get(PLL_OUTPUT_DIV_MASK, val) as u64 + 1) * 2;
    val = readl_relaxed((*pll).base.add(PLL_CFG1));
    let divff = field_get(PLL_FRAC_DIV_MASK, val) as u64;
    let divfi = field_get(PLL_INT_DIV_MASK, val) as u64;
    let temp64 = parent_rate.wrapping_mul(8).wrapping_mul(divff) / PLL_FRAC_DENOM / divq;
    parent_rate.wrapping_mul(8).wrapping_mul(divfi + 1) / divq + temp64
}

unsafe extern "C" fn clk_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let _ = hw;
    let parent_rate = (*req).best_parent_rate.wrapping_mul(8);
    (*req).rate = (*req).rate.wrapping_mul(2);
    let divfi = (*req).rate / parent_rate;
    let divff = (*req).rate.wrapping_sub(divfi * parent_rate).wrapping_mul(PLL_FRAC_DENOM) / parent_rate;
    let temp64 = parent_rate * divff / PLL_FRAC_DENOM;
    (*req).rate = (parent_rate * divfi + temp64) / 2;
    0
}

/*
 * To simplify the clock calculation, we can keep the 'PLL_OUTPUT_VAL' at zero
 * (means the PLL output will be divided by 2). So the PLL output can use the
 * below formula:
 * pllout = parent_rate * 8 / 2 * DIVF_VAL;
 * where DIVF_VAL = 1 + DIVFI + DIVFF / 2^24.
 */
unsafe extern "C" fn clk_pll_set_rate(hw: *mut clk_hw, mut rate: u64, mut parent_rate: u64) -> i32 {
    let pll = hw as *mut clk_frac_pll;
    parent_rate *= 8;
    rate *= 2;
    let divfi = rate / parent_rate;
    let divff = rate.wrapping_sub(parent_rate * divfi) * PLL_FRAC_DENOM / parent_rate;
    let mut val = readl_relaxed((*pll).base.add(PLL_CFG1));
    val &= !(PLL_FRAC_DIV_MASK | PLL_INT_DIV_MASK);
    val |= ((divff as u32) << 7) | (divfi as u32 - 1);
    writel_relaxed(val, (*pll).base.add(PLL_CFG1));
    val = readl_relaxed((*pll).base.add(PLL_CFG0));
    val &= !0x1f;
    writel_relaxed(val, (*pll).base.add(PLL_CFG0));
    val = readl_relaxed((*pll).base.add(PLL_CFG0));
    val |= PLL_NEWDIV_VAL;
    writel_relaxed(val, (*pll).base.add(PLL_CFG0));
    let ret = clk_wait_ack(pll);
    val = readl_relaxed((*pll).base.add(PLL_CFG0));
    val &= !PLL_NEWDIV_VAL;
    writel_relaxed(val, (*pll).base.add(PLL_CFG0));
    ret
}

static CLK_FRAC_PLL_OPS: clk_ops = clk_ops {
    prepare: Some(clk_pll_prepare), unprepare: Some(clk_pll_unprepare),
    is_prepared: Some(clk_pll_is_prepared), recalc_rate: Some(clk_pll_recalc_rate),
    determine_rate: Some(clk_pll_determine_rate), set_rate: Some(clk_pll_set_rate),
};

#[no_mangle]
pub unsafe extern "C" fn imx_clk_hw_frac_pll(name: *const i8, parent_name: *const i8, base: *mut u8) -> *mut clk_hw {
    let pll = kzalloc_obj();
    if pll.is_null() { return (-12isize) as *mut clk_hw; }
    let init = Box::new(clk_init_data { name, ops: &CLK_FRAC_PLL_OPS, flags: 0, parent_names: &parent_name, num_parents: 1 });
    (*pll).base = base;
    (*pll).hw.init = Box::into_raw(init);
    let hw = &mut (*pll).hw as *mut clk_hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 { kfree(pll); return ret as isize as *mut clk_hw; }
    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
