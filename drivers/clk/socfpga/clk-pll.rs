// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright 2011-2012 Calxeda, Inc.
 *  Copyright (C) 2012-2013 Altera Corporation <www.altera.com>
 *
 * Based from clk-highbank.c
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

/* Clock bypass bits */
const MAINPLL_BYPASS: u32 = 1 << 0;
const SDRAMPLL_BYPASS: u32 = 1 << 1;
const SDRAMPLL_SRC_BYPASS: u32 = 1 << 2;
const PERPLL_BYPASS: u32 = 1 << 3;
const PERPLL_SRC_BYPASS: u32 = 1 << 4;

const SOCFPGA_PLL_BG_PWRDWN: u32 = 0;
const SOCFPGA_PLL_EXT_ENA: u32 = 1;
const SOCFPGA_PLL_PWR_DOWN: u32 = 2;
const SOCFPGA_PLL_DIVF_MASK: u32 = 0x0000FFF8;
const SOCFPGA_PLL_DIVF_SHIFT: u32 = 3;
const SOCFPGA_PLL_DIVQ_MASK: u32 = 0x003F0000;
const SOCFPGA_PLL_DIVQ_SHIFT: u32 = 16;

const CLK_MGR_PLL_CLK_SRC_SHIFT: u32 = 22;
const CLK_MGR_PLL_CLK_SRC_MASK: u32 = 0x3;

// The following types and functions are provided by the translated kernel headers.
#[repr(C)]
pub struct clk_hw {
    pub init: *mut clk_init_data,
}

#[repr(C)]
pub struct socfpga_pll_hw {
    pub hw: clk_hw,
    pub reg: *mut core::ffi::c_void,
    pub bit_idx: u32,
}

#[repr(C)]
pub struct socfpga_pll {
    pub hw: socfpga_pll_hw,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const core::ffi::c_char,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub num_parents: u8,
    pub parent_names: *const *const core::ffi::c_char,
}

#[repr(C)]
pub struct device_node {
    pub name: *const core::ffi::c_char,
}

extern "C" {
    static mut clk_mgr_base_addr: *mut u8;
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn of_property_read_u32(node: *mut device_node, name: *const core::ffi::c_char, value: *mut u32) -> i32;
    fn of_property_read_string(node: *mut device_node, name: *const core::ffi::c_char, value: *mut *const core::ffi::c_char) -> i32;
    fn of_find_compatible_node(from: *mut device_node, ty: *const core::ffi::c_char, compatible: *const core::ffi::c_char) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut u8;
    fn of_node_put(node: *mut device_node);
    fn of_clk_parent_fill(node: *mut device_node, parents: *mut *const core::ffi::c_char, size: u8) -> u8;
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> i32;
    fn clk_hw_unregister(hw: *mut clk_hw);
    fn of_clk_add_hw_provider(node: *mut device_node, get: *const core::ffi::c_void, hw: *mut clk_hw) -> i32;
    fn of_clk_hw_simple_get() -> *const core::ffi::c_void;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

const CLKMGR_BYPASS: usize = 0;
const SOCFPGA_MAX_PARENTS: u8 = 3;

unsafe fn clk_pll_recalc_rate(hwclk: *mut clk_hw, parent_rate: usize) -> usize {
    let socfpgaclk = (hwclk as *mut socfpga_pll).as_ref().unwrap();
    let reg = readl(socfpgaclk.hw.reg as *const core::ffi::c_void);
    let bypass = readl(clk_mgr_base_addr.add(CLKMGR_BYPASS) as *const core::ffi::c_void);
    if bypass & MAINPLL_BYPASS != 0 {
        return parent_rate;
    }

    let divf = (reg & SOCFPGA_PLL_DIVF_MASK) >> SOCFPGA_PLL_DIVF_SHIFT;
    let divq = (reg & SOCFPGA_PLL_DIVQ_MASK) >> SOCFPGA_PLL_DIVQ_SHIFT;
    let vco_freq = (parent_rate as u64).wrapping_mul((divf + 1) as u64);
    (vco_freq / (1 + divq as u64)) as usize
}

unsafe fn clk_pll_get_parent(hwclk: *mut clk_hw) -> u8 {
    let socfpgaclk = (hwclk as *mut socfpga_pll).as_ref().unwrap();
    let pll_src = readl(socfpgaclk.hw.reg as *const core::ffi::c_void);
    ((pll_src >> CLK_MGR_PLL_CLK_SRC_SHIFT) & CLK_MGR_PLL_CLK_SRC_MASK) as u8
}

static CLK_PLL_OPS: clk_ops = clk_ops {
    recalc_rate: Some(clk_pll_recalc_rate),
    get_parent: Some(clk_pll_get_parent),
};

unsafe fn __socfpga_pll_init(node: *mut device_node, ops: *const clk_ops) {
    let mut reg: u32 = 0;
    let mut hw_clk: *mut clk_hw;
    let pll_clk = kzalloc(core::mem::size_of::<socfpga_pll>(), 0) as *mut socfpga_pll;
    if pll_clk.is_null() {
        return;
    }

    let mut clk_name = (*node).name;
    of_property_read_u32(node, b"reg\0".as_ptr() as *const _, &mut reg);
    let clkmgr_np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"altr,clk-mgr\0".as_ptr() as *const _);
    clk_mgr_base_addr = of_iomap(clkmgr_np, 0);
    of_node_put(clkmgr_np);
    (*pll_clk).hw.reg = clk_mgr_base_addr.add(reg as usize) as *mut core::ffi::c_void;

    of_property_read_string(node, b"clock-output-names\0".as_ptr() as *const _, &mut clk_name);
    let mut parent_name: [*const core::ffi::c_char; SOCFPGA_MAX_PARENTS as usize] = [core::ptr::null(); SOCFPGA_MAX_PARENTS as usize];
    let mut init = clk_init_data {
        name: clk_name,
        ops,
        flags: 0,
        num_parents: of_clk_parent_fill(node, parent_name.as_mut_ptr(), SOCFPGA_MAX_PARENTS),
        parent_names: parent_name.as_ptr(),
    };
    (*pll_clk).hw.hw.init = &mut init;
    (*pll_clk).hw.bit_idx = SOCFPGA_PLL_EXT_ENA;
    hw_clk = &mut (*pll_clk).hw.hw;

    let rc = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if rc != 0 {
        pr_err(b"Could not register clock:%s\n\0".as_ptr() as *const _, clk_name);
        kfree(pll_clk as *mut _);
        return;
    }
    let rc = of_clk_add_hw_provider(node, of_clk_hw_simple_get(), hw_clk);
    if rc != 0 {
        pr_err(b"Could not register clock provider for node:%s\n\0".as_ptr() as *const _, clk_name);
        clk_hw_unregister(hw_clk);
        kfree(pll_clk as *mut _);
    }
}

pub unsafe fn socfpga_pll_init(node: *mut device_node) {
    __socfpga_pll_init(node, &CLK_PLL_OPS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
