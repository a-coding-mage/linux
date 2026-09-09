// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * clk-xgene.c - AppliedMicro X-Gene Clock Interface
 * Rust translation of the implementation source.
 */

// Linux kernel dependencies supplied externally.
use core::ffi::{c_char, c_int, c_void};

type u32_t = u32;
type u64_t = u64;
type ulong = usize;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { pub full_name: *const c_char }
#[repr(C)] pub struct resource { pub name: *const c_char }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk_rate_request { pub rate: ulong, pub best_parent_rate: ulong }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct clk_ops {
    pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, ulong) -> ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, ulong, ulong) -> c_int>,
}
#[repr(C)] pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub flags: ulong,
    pub parent_names: *const *const c_char,
    pub num_parents: u32,
}

extern "C" {
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn writel_relaxed(data: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn writel(data: u32, addr: *mut c_void);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: ulong);
    fn __acquire(lock: *mut spinlock_t);
    fn __release(lock: *mut spinlock_t);
    fn clk_hw_get_name(hw: *mut clk_hw) -> *const c_char;
    fn clk_register(dev: *mut device, hw: *mut clk_hw) -> *mut clk;
    fn clk_register_clkdev(clk: *mut clk, name: *const c_char, con_id: *const c_char) -> c_int;
    fn of_device_is_compatible(np: *mut device_node, compat: *const c_char) -> bool;
    fn of_device_is_available(np: *mut device_node) -> bool;
    fn of_iomap(np: *mut device_node, index: c_int) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn of_clk_get_parent_name(np: *mut device_node, index: c_int) -> *const c_char;
    fn of_clk_add_provider(np: *mut device_node, get: *const c_void, clk: *mut clk) -> c_int;
    fn of_clk_src_simple_get: c_void;
    fn of_address_to_resource(np: *mut device_node, index: c_int, res: *mut resource) -> c_int;
    fn of_property_read_string(np: *mut device_node, name: *const c_char, out: *mut *const c_char) -> c_int;
    fn of_property_read_u32(np: *mut device_node, name: *const c_char, out: *mut u32) -> c_int;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn str_enabled_disabled(value: u32) -> *const c_char;
}

const N_DIV_RD: fn(u32) -> u32 = |src| src & 0x000001ff;
const SC_N_DIV_RD: fn(u32) -> u32 = |src| src & 0x0000007f;
const SC_OUTDIV2: fn(u32) -> u32 = |src| (src & 0x00000100) >> 8;
const CLKR_RD: fn(u32) -> u32 = |src| (src & 0x07000000) >> 24;
const CLKOD_RD: fn(u32) -> u32 = |src| (src & 0x00300000) >> 20;
const REGSPEC_RESET_F1_MASK: u32 = 0x00010000;
const CLKF_RD: fn(u32) -> u32 = |src| src & 0x000001ff;
const XGENE_CLK_DRIVER_VER: &str = "0.1";

static mut clk_lock: spinlock_t = spinlock_t { _private: [] };

#[inline] unsafe fn xgene_clk_read(csr: *mut c_void) -> u32 { readl_relaxed(csr) }
#[inline] unsafe fn xgene_clk_write(data: u32, csr: *mut c_void) { writel_relaxed(data, csr) }

#[repr(C)] #[derive(Copy, Clone, PartialEq)] enum xgene_pll_type { PLL_TYPE_PCP = 0, PLL_TYPE_SOC = 1 }
#[repr(C)] struct xgene_clk_pll { hw: clk_hw, reg: *mut c_void, lock: *mut spinlock_t, pll_offset: u32, r#type: xgene_pll_type, version: c_int }

unsafe fn xgene_clk_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    let pllclk = (hw as *mut u8).sub(core::mem::offset_of!(xgene_clk_pll, hw)) as *mut xgene_clk_pll;
    let data = xgene_clk_read((*pllclk).reg.add((*pllclk).pll_offset as usize));
    if data & REGSPEC_RESET_F1_MASK != 0 { 0 } else { 1 }
}

unsafe fn xgene_clk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: ulong) -> ulong {
    let pllclk = (hw as *mut u8).sub(core::mem::offset_of!(xgene_clk_pll, hw)) as *mut xgene_clk_pll;
    let pll = xgene_clk_read((*pllclk).reg.add((*pllclk).pll_offset as usize));
    let (fvco, nout): (ulong, ulong);
    if (*pllclk).version <= 1 {
        if (*pllclk).r#type == xgene_pll_type::PLL_TYPE_PCP { nout = 2; fvco = parent_rate * (N_DIV_RD(pll) as ulong + 4); }
        else { let nref = CLKR_RD(pll) as ulong + 1; nout = CLKOD_RD(pll) as ulong + 1; let nfb = CLKF_RD(pll) as ulong; fvco = (parent_rate / nref) * nfb; }
    } else { nout = if SC_OUTDIV2(pll) != 0 { 2 } else { 3 }; fvco = parent_rate * SC_N_DIV_RD(pll) as ulong; }
    fvco / nout
}

static xgene_clk_pll_ops: clk_ops = clk_ops { enable: None, disable: None, is_enabled: Some(xgene_clk_pll_is_enabled), recalc_rate: Some(xgene_clk_pll_recalc_rate), determine_rate: None, set_rate: None };

#[repr(C)] struct xgene_clk_pmd { hw: clk_hw, reg: *mut c_void, shift: u8, mask: u32, denom: u64, flags: u32, lock: *mut spinlock_t }
const XGENE_CLK_PMD_SCALE_INVERTED: u32 = 1 << 0;
const XGENE_CLK_PMD_SHIFT: u32 = 8;
const XGENE_CLK_PMD_WIDTH: u32 = 3;

#[repr(C)] struct xgene_dev_parameters { csr_reg: *mut c_void, reg_clk_offset: u32, reg_clk_mask: u32, reg_csr_offset: u32, reg_csr_mask: u32, divider_reg: *mut c_void, reg_divider_offset: u32, reg_divider_shift: u32, reg_divider_width: u32 }
#[repr(C)] struct xgene_clk { hw: clk_hw, lock: *mut spinlock_t, param: xgene_dev_parameters }

unsafe fn xgene_clk_enable(hw: *mut clk_hw) -> c_int {
    let pclk = (hw as *mut u8).sub(core::mem::offset_of!(xgene_clk, hw)) as *mut xgene_clk;
    if !(*pclk).param.csr_reg.is_null() { let mut data = xgene_clk_read((*pclk).param.csr_reg.add((*pclk).param.reg_clk_offset as usize)); data |= (*pclk).param.reg_clk_mask; xgene_clk_write(data, (*pclk).param.csr_reg.add((*pclk).param.reg_clk_offset as usize)); data = xgene_clk_read((*pclk).param.csr_reg.add((*pclk).param.reg_csr_offset as usize)); data &= !(*pclk).param.reg_csr_mask; xgene_clk_write(data, (*pclk).param.csr_reg.add((*pclk).param.reg_csr_offset as usize)); } 0
}
unsafe fn xgene_clk_disable(hw: *mut clk_hw) { let pclk = (hw as *mut u8).sub(core::mem::offset_of!(xgene_clk, hw)) as *mut xgene_clk; if !(*pclk).param.csr_reg.is_null() { let mut data = xgene_clk_read((*pclk).param.csr_reg.add((*pclk).param.reg_csr_offset as usize)); data |= (*pclk).param.reg_csr_mask; xgene_clk_write(data, (*pclk).param.csr_reg.add((*pclk).param.reg_csr_offset as usize)); data = xgene_clk_read((*pclk).param.csr_reg.add((*pclk).param.reg_clk_offset as usize)); data &= !(*pclk).param.reg_clk_mask; xgene_clk_write(data, (*pclk).param.csr_reg.add((*pclk).param.reg_clk_offset as usize)); } }
unsafe fn xgene_clk_is_enabled(hw: *mut clk_hw) -> c_int { let pclk = (hw as *mut u8).sub(core::mem::offset_of!(xgene_clk, hw)) as *mut xgene_clk; if (*pclk).param.csr_reg.is_null() { return 1; } (xgene_clk_read((*pclk).param.csr_reg.add((*pclk).param.reg_clk_offset as usize)) & (*pclk).param.reg_clk_mask != 0) as c_int }
unsafe fn xgene_clk_recalc_rate(hw: *mut clk_hw, parent_rate: ulong) -> ulong { let pclk = (hw as *mut u8).sub(core::mem::offset_of!(xgene_clk, hw)) as *mut xgene_clk; if (*pclk).param.divider_reg.is_null() { parent_rate } else { let mut data = xgene_clk_read((*pclk).param.divider_reg.add((*pclk).param.reg_divider_offset as usize)); data >>= (*pclk).param.reg_divider_shift; parent_rate / ((data & ((1u32 << (*pclk).param.reg_divider_width) - 1)) as ulong) } }
unsafe fn xgene_clk_set_rate(hw: *mut clk_hw, mut rate: ulong, parent_rate: ulong) -> c_int { let pclk = (hw as *mut u8).sub(core::mem::offset_of!(xgene_clk, hw)) as *mut xgene_clk; let mut divider_save = 1; if !(*pclk).param.divider_reg.is_null() { if rate > parent_rate { rate = parent_rate; } divider_save = parent_rate / rate; let divider = ((divider_save as u32) & ((1u32 << (*pclk).param.reg_divider_width) - 1)) << (*pclk).param.reg_divider_shift; let mut data = xgene_clk_read((*pclk).param.divider_reg.add((*pclk).param.reg_divider_offset as usize)); data &= !(((1u32 << (*pclk).param.reg_divider_width) - 1) << (*pclk).param.reg_divider_shift); data |= divider; xgene_clk_write(data, (*pclk).param.divider_reg.add((*pclk).param.reg_divider_offset as usize)); } (parent_rate / divider_save) as c_int }
unsafe fn xgene_clk_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int { let pclk = (hw as *mut u8).sub(core::mem::offset_of!(xgene_clk, hw)) as *mut xgene_clk; let parent_rate = (*req).best_parent_rate; let divider = if (*pclk).param.divider_reg.is_null() { 1 } else { if (*req).rate > parent_rate { (*req).rate = parent_rate; } parent_rate / (*req).rate }; (*req).rate = parent_rate / divider; 0 }

#[allow(non_upper_case_globals)] static xgene_clk_ops: clk_ops = clk_ops { enable: Some(xgene_clk_enable), disable: Some(xgene_clk_disable), is_enabled: Some(xgene_clk_is_enabled), recalc_rate: Some(xgene_clk_recalc_rate), determine_rate: Some(xgene_clk_determine_rate), set_rate: Some(xgene_clk_set_rate) };

// Device-tree registration declarations corresponding to CLK_OF_DECLARE entries.
extern "C" {
    fn xgene_socpllclk_init(np: *mut device_node);
    fn xgene_pcppllclk_init(np: *mut device_node);
    fn xgene_pmdclk_init(np: *mut device_node);
    fn xgene_devclk_init(np: *mut device_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
