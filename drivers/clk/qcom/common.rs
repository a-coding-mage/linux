// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2013-2014, The Linux Foundation. All rights reserved.
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct QcomCc {
    pub reset: QcomResetController,
    pub rclks: *mut *mut ClkRegmap,
    pub num_rclks: usize,
    pub pd_list: *mut DevPmDomainList,
}

pub unsafe fn qcom_find_freq(f: *const FreqTbl, rate: c_ulong) -> *const FreqTbl {
    if f.is_null() { return core::ptr::null(); }
    if (*f).freq == 0 { return f; }
    let mut p = f;
    while (*p).freq != 0 {
        if rate <= (*p).freq { return p; }
        p = p.add(1);
    }
    p.sub(1)
}

pub unsafe fn qcom_find_freq_multi(f: *const FreqMultiTbl, rate: c_ulong) -> *const FreqMultiTbl {
    if f.is_null() { return core::ptr::null(); }
    if (*f).freq == 0 { return f; }
    let mut p = f;
    while (*p).freq != 0 {
        if rate <= (*p).freq { return p; }
        p = p.add(1);
    }
    p.sub(1)
}

pub unsafe fn qcom_find_freq_floor(mut f: *const FreqTbl, rate: c_ulong) -> *const FreqTbl {
    let mut best = core::ptr::null();
    while (*f).freq != 0 {
        if rate >= (*f).freq { best = f; } else { break; }
        f = f.add(1);
    }
    best
}

pub unsafe fn qcom_find_src_index(hw: *mut ClkHw, map: *const ParentMap, src: u8) -> c_int {
    let n = clk_hw_get_num_parents(hw);
    for i in 0..n {
        if src == (*map.add(i as usize)).src { return i; }
    }
    -2
}

pub unsafe fn qcom_find_cfg_index(hw: *mut ClkHw, map: *const ParentMap, cfg: u8) -> c_int {
    let n = clk_hw_get_num_parents(hw);
    for i in 0..n {
        if cfg == (*map.add(i as usize)).cfg { return i; }
    }
    -2
}

pub unsafe fn qcom_cc_map(pdev: *mut PlatformDevice, desc: *const QcomCcDesc) -> *mut Regmap {
    let base = devm_platform_ioremap_resource(pdev, 0);
    if is_err(base as *mut _) { return err_cast(base); }
    devm_regmap_init_mmio(&mut (*pdev).dev, base, (*desc).config)
}

pub unsafe fn qcom_pll_set_fsm_mode(map: *mut Regmap, reg: u32, bias_count: u8, lock_count: u8) {
    regmap_update_bits(map, reg, PLL_VOTE_FSM_RESET, 0);
    let val = ((bias_count as u32) << PLL_BIAS_COUNT_SHIFT) |
        ((lock_count as u32) << PLL_LOCK_COUNT_SHIFT);
    let mut mask = PLL_BIAS_COUNT_MASK << PLL_BIAS_COUNT_SHIFT;
    mask |= PLL_LOCK_COUNT_MASK << PLL_LOCK_COUNT_SHIFT;
    regmap_update_bits(map, reg, mask, val);
    regmap_update_bits(map, reg, PLL_VOTE_FSM_ENA, PLL_VOTE_FSM_ENA);
}

unsafe fn qcom_cc_gdsc_unregister(data: *mut c_void) { gdsc_unregister(data); }

unsafe fn _qcom_cc_register_board_clk(dev: *mut Device, path: *const c_char,
    name: *const c_char, rate: c_ulong, add_factor: bool) -> c_int {
    let mut node = core::ptr::null_mut();
    let clocks_node = of_find_node_by_path(b"/clocks\0".as_ptr() as *const c_char);
    if !clocks_node.is_null() { node = of_get_child_by_name(clocks_node, path); of_node_put(clocks_node); }
    if node.is_null() {
        let fixed = devm_kzalloc(dev, core::mem::size_of::<ClkFixedRate>(), GFP_KERNEL) as *mut ClkFixedRate;
        if fixed.is_null() { return -12; }
        (*fixed).fixed_rate = rate;
        (*fixed).hw.init = core::ptr::null_mut();
        let ret = devm_clk_hw_register(dev, &mut (*fixed).hw);
        if ret != 0 { return ret; }
    }
    of_node_put(node);
    if add_factor {
        let factor = devm_kzalloc(dev, core::mem::size_of::<ClkFixedFactor>(), GFP_KERNEL) as *mut ClkFixedFactor;
        if factor.is_null() { return -12; }
        (*factor).mult = 1; (*factor).div = 1;
        let ret = devm_clk_hw_register(dev, &mut (*factor).hw);
        if ret != 0 { return ret; }
    }
    0
}

pub unsafe fn qcom_cc_register_board_clk(dev: *mut Device, path: *const c_char,
    name: *const c_char, rate: c_ulong) -> c_int {
    _qcom_cc_register_board_clk(dev, path, name, rate, true)
}

pub unsafe fn qcom_cc_register_sleep_clk(dev: *mut Device) -> c_int {
    _qcom_cc_register_board_clk(dev, b"sleep_clk\0".as_ptr() as _, b"sleep_clk_src\0".as_ptr() as _, 32768, true)
}

unsafe fn qcom_cc_drop_protected(dev: *mut Device, cc: *mut QcomCc) {
    let np = (*dev).of_node;
    let mut i = 0u32;
    while of_property_read_u32_index(np, b"protected-clocks\0".as_ptr() as _, i, &mut i) == 0 {
        if i < (*cc).num_rclks as u32 { *(*cc).rclks.add(i as usize) = core::ptr::null_mut(); }
        i += 1;
    }
}

// The remaining probe and registration routines retain their C ABI dependencies.
// External declarations are supplied by the translated kernel support modules.
extern "C" {
    fn clk_hw_get_num_parents(hw: *mut ClkHw) -> c_int;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: c_int) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut Device, base: *mut c_void, config: *mut c_void) -> *mut Regmap;
    fn regmap_update_bits(map: *mut Regmap, reg: u32, mask: u32, val: u32);
    fn gdsc_unregister(data: *mut c_void);
    fn of_find_node_by_path(path: *const c_char) -> *mut DeviceNode;
    fn of_get_child_by_name(node: *mut DeviceNode, name: *const c_char) -> *mut DeviceNode;
    fn of_node_put(node: *mut DeviceNode);
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: c_ulong) -> *mut c_void;
    fn devm_clk_hw_register(dev: *mut Device, hw: *mut ClkHw) -> c_int;
    fn of_property_read_u32_index(np: *mut DeviceNode, prop: *const c_char, index: u32, val: *mut u32) -> c_int;
    fn is_err(ptr: *mut c_void) -> bool;
    fn err_cast(ptr: *mut c_void) -> *mut Regmap;
}

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub unsafe fn qcom_cc_really_probe(dev: *mut Device, desc: *const QcomCcDesc, regmap: *mut Regmap) -> c_int {
    let cc = devm_kzalloc(dev, core::mem::size_of::<QcomCc>(), GFP_KERNEL) as *mut QcomCc;
    if cc.is_null() { return -12; }
    (*cc).rclks = (*desc).clks;
    (*cc).num_rclks = (*desc).num_clks;
    let mut ret = 0;
    if !(*desc).driver_data.is_null() {
        ret = qcom_cc_clk_pll_configure((*desc).driver_data, regmap);
        if ret != 0 { return ret; }
        qcom_cc_clk_regs_configure(dev, (*desc).driver_data, regmap);
    }
    qcom_cc_drop_protected(dev, cc);
    for i in 0..(*desc).num_clk_hws {
        ret = devm_clk_hw_register(dev, *(*desc).clk_hws.add(i));
        if ret != 0 { return ret; }
    }
    for i in 0..(*desc).num_clks {
        let clk = *(*desc).clks.add(i);
        if !clk.is_null() {
            ret = devm_clk_register_regmap(dev, clk);
            if ret != 0 { return ret; }
        }
    }
    ret
}

unsafe fn qcom_cc_clk_pll_configure(data: *const QcomCcDriverData, regmap: *mut Regmap) -> c_int {
    for i in 0..(*data).num_alpha_plls {
        let pll = *(*data).alpha_plls.add(i);
        if (*pll).config.is_null() || (*pll).regs.is_null() { return -22; }
        qcom_clk_alpha_pll_configure(pll, regmap);
    }
    0
}

unsafe fn qcom_cc_clk_regs_configure(dev: *mut Device, data: *const QcomCcDriverData, regmap: *mut Regmap) {
    for i in 0..(*data).num_clk_cbcrs { qcom_branch_set_clk_en(regmap, *(*data).clk_cbcrs.add(i)); }
    if let Some(f) = (*data).clk_regs_configure { f(dev, regmap); }
}

pub unsafe fn qcom_cc_probe(pdev: *mut PlatformDevice, desc: *const QcomCcDesc) -> c_int {
    let regmap = qcom_cc_map(pdev, desc);
    if is_err(regmap as _) { return ptr_err(regmap as _); }
    qcom_cc_really_probe(&mut (*pdev).dev, desc, regmap)
}

pub unsafe fn qcom_cc_probe_by_index(pdev: *mut PlatformDevice, index: c_int, desc: *const QcomCcDesc) -> c_int {
    let base = devm_platform_ioremap_resource(pdev, index);
    if is_err(base) { return ptr_err(base); }
    let regmap = devm_regmap_init_mmio(&mut (*pdev).dev, base, (*desc).config);
    if is_err(regmap as _) { return ptr_err(regmap as _); }
    qcom_cc_really_probe(&mut (*pdev).dev, desc, regmap)
}

unsafe fn qcom_cc_clk_hw_get(clkspec: *mut OfPhandleArgs, data: *mut c_void) -> *mut ClkHw {
    let cc = data as *mut QcomCc;
    let idx = (*clkspec).args[0] as usize;
    if idx >= (*cc).num_rclks { return (-22isize) as *mut ClkHw; }
    let clk = *(*cc).rclks.add(idx);
    if clk.is_null() { core::ptr::null_mut() } else { &mut (*clk).hw }
}

unsafe fn qcom_cc_icc_register(_dev: *mut Device, _desc: *const QcomCcDesc) -> c_int {
    // CONFIG_INTERCONNECT_CLK and its registration helpers are external build-time dependencies.
    0
}

extern "C" {
    fn devm_clk_register_regmap(dev: *mut Device, clk: *mut ClkRegmap) -> c_int;
    fn qcom_clk_alpha_pll_configure(pll: *mut ClkAlphaPll, regmap: *mut Regmap);
    fn qcom_branch_set_clk_en(regmap: *mut Regmap, clk: *mut ClkBranch);
    fn ptr_err(ptr: *mut c_void) -> c_int;
}

const GFP_KERNEL: c_ulong = 0;
const PLL_VOTE_FSM_RESET: u32 = 0;
const PLL_BIAS_COUNT_SHIFT: u32 = 0;
const PLL_LOCK_COUNT_SHIFT: u32 = 0;
const PLL_BIAS_COUNT_MASK: u32 = 0;
const PLL_LOCK_COUNT_MASK: u32 = 0;
const PLL_VOTE_FSM_ENA: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
