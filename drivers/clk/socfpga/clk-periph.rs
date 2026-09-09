// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright 2011-2012 Calxeda, Inc.
 *  Copyright (C) 2012-2013 Altera Corporation <www.altera.com>
 *
 * Based from clk-highbank.c
 */

// Linux headers and clk.h provide the types, constants, and functions used below.

#[repr(C)]
pub struct clk_hw {
    pub reg: *mut core::ffi::c_void,
    pub init: *mut clk_init_data,
}

#[repr(C)]
pub struct socfpga_periph_clk {
    pub hw: clk_hw_wrapper,
    pub div_reg: *mut u32,
    pub shift: u32,
    pub width: u32,
    pub fixed_div: u32,
}

#[repr(C)]
pub struct clk_hw_wrapper {
    pub hw: clk_hw,
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
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
}

extern "C" {
    static mut clk_mgr_base_addr: *mut u8;
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> i32;
    fn clk_hw_unregister(hw: *mut clk_hw);
    fn of_clk_add_hw_provider(node: *mut device_node, get: *const core::ffi::c_void,
                              hw: *mut clk_hw) -> i32;
    fn of_clk_hw_simple_get() -> !;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn of_property_read_u32(node: *mut device_node, name: *const core::ffi::c_char,
                            value: *mut u32) -> i32;
    fn of_property_read_u32_array(node: *mut device_node, name: *const core::ffi::c_char,
                                  value: *mut u32, count: usize) -> i32;
    fn of_property_read_string(node: *mut device_node, name: *const core::ffi::c_char,
                               value: *mut *const core::ffi::c_char) -> i32;
    fn of_clk_parent_fill(node: *mut device_node, parents: *mut *const core::ffi::c_char,
                          count: usize) -> u8;
}

#[repr(C)]
pub struct device_node {
    pub name: *const core::ffi::c_char,
}

const SOCFPGA_MAX_PARENTS: usize = 2;
const CLKMGR_DBCTRL: usize = 0;

unsafe fn clk_periclk_recalc_rate(hwclk: *mut clk_hw, mut parent_rate: usize) -> usize {
    let socfpgaclk = (hwclk as *mut u8).sub(core::mem::offset_of!(socfpga_periph_clk, hw))
        as *mut socfpga_periph_clk;
    let div: u32;

    if (*socfpgaclk).fixed_div != 0 {
        div = (*socfpgaclk).fixed_div;
    } else {
        if !(*socfpgaclk).div_reg.is_null() {
            let mut val = readl((*socfpgaclk).div_reg as *const core::ffi::c_void)
                >> (*socfpgaclk).shift;
            val &= (1u32 << ((*socfpgaclk).width - 1)) * 2 - 1;
            parent_rate /= (val + 1) as usize;
        }
        div = (readl((*socfpgaclk).hw.hw.reg as *const core::ffi::c_void) & 0x1ff) + 1;
    }

    parent_rate / div as usize
}

unsafe fn clk_periclk_get_parent(_hwclk: *mut clk_hw) -> u8 {
    let clk_src = readl(clk_mgr_base_addr.add(CLKMGR_DBCTRL) as *const core::ffi::c_void);
    (clk_src & 0x1) as u8
}

static periclk_ops: clk_ops = clk_ops {
    recalc_rate: Some(clk_periclk_recalc_rate),
    get_parent: Some(clk_periclk_get_parent),
};

unsafe fn __socfpga_periph_init(node: *mut device_node, ops: *const clk_ops) {
    let mut reg = 0u32;
    let mut clk_name = (*node).name;
    let mut parent_name = [core::ptr::null(); SOCFPGA_MAX_PARENTS];
    let mut init = clk_init_data { name: core::ptr::null(), ops, flags: 0,
                                    num_parents: 0, parent_names: core::ptr::null() };
    let mut fixed_div = 0u32;
    let mut div_reg = [0u32; 3];

    of_property_read_u32(node, b"reg\0".as_ptr() as *const _, &mut reg);
    let periph_clk = kzalloc_obj::<socfpga_periph_clk>();
    if periph_clk.is_null() {
        return;
    }
    (*periph_clk).hw.hw.reg = clk_mgr_base_addr.add(reg as usize) as *mut _;

    let rc = of_property_read_u32_array(node, b"div-reg\0".as_ptr() as *const _,
                                         div_reg.as_mut_ptr(), 3);
    if rc == 0 {
        (*periph_clk).div_reg = clk_mgr_base_addr.add(div_reg[0] as usize) as *mut u32;
        (*periph_clk).shift = div_reg[1];
        (*periph_clk).width = div_reg[2];
    } else {
        (*periph_clk).div_reg = core::ptr::null_mut();
    }

    if of_property_read_u32(node, b"fixed-divider\0".as_ptr() as *const _, &mut fixed_div) != 0 {
        (*periph_clk).fixed_div = 0;
    } else {
        (*periph_clk).fixed_div = fixed_div;
    }
    of_property_read_string(node, b"clock-output-names\0".as_ptr() as *const _, &mut clk_name);
    init.name = clk_name;
    init.num_parents = of_clk_parent_fill(node, parent_name.as_mut_ptr(), SOCFPGA_MAX_PARENTS);
    init.parent_names = parent_name.as_ptr();
    (*periph_clk).hw.hw.init = &mut init;
    let hw_clk = &mut (*periph_clk).hw.hw;

    if clk_hw_register(core::ptr::null_mut(), hw_clk) != 0 {
        pr_err(b"Could not register clock:%s\n\0".as_ptr() as *const _, clk_name);
        kfree(periph_clk as *mut _);
        return;
    }
    if of_clk_add_hw_provider(node, of_clk_hw_simple_get as *const _, hw_clk) != 0 {
        pr_err(b"Could not register clock provider for node:%s\n\0".as_ptr() as *const _, clk_name);
        clk_hw_unregister(hw_clk);
        kfree(periph_clk as *mut _);
    }
}

pub unsafe fn socfpga_periph_init(node: *mut device_node) {
    __socfpga_periph_init(node, &periclk_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
