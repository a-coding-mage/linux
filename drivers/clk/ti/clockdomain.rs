// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP clockdomain support
 *
 * Copyright (C) 2013 Texas Instruments, Inc.
 *
 * Tero Kristo <t-kristo@ti.com>
 */

// Linux clock, device-tree, and OMAP clock interfaces are supplied by the
// surrounding translation unit.

use core::ffi::{c_char, c_int, c_uint};

pub const TI_CLK_DISABLE_CLKDM_CONTROL: c_uint = 1 << 0;

#[repr(C)]
pub struct clk_hw {
    pub clk: *mut clk,
}

#[repr(C)]
pub struct clk_hw_omap {
    pub hw: clk_hw,
    pub clkdm: *mut clockdomain,
    pub clkdm_name: *const c_char,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clockdomain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[used]
pub static TI_CLKDM_MATCH_TABLE: [of_device_id; 2] = [
    of_device_id { compatible: c"ti,clockdomain".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

#[repr(C)]
pub struct clk_features {
    pub flags: c_uint,
}

#[repr(C)]
pub struct ti_clk_ll_ops_struct {
    pub clkdm_clk_enable: unsafe extern "C" fn(*mut clockdomain, *mut clk) -> c_int,
    pub clkdm_clk_disable: unsafe extern "C" fn(*mut clockdomain, *mut clk),
    pub clkdm_lookup: unsafe extern "C" fn(*const c_char) -> *mut clockdomain,
}

extern "C" {
    pub static mut ti_clk_ll_ops: *const ti_clk_ll_ops_struct;

    fn to_clk_hw_omap(hw: *mut clk_hw) -> *mut clk_hw_omap;
    fn ti_clk_get_features() -> *const clk_features;
    fn clk_hw_get_name(hw: *mut clk_hw) -> *const c_char;
    fn __clk_get_name(clk: *mut clk) -> *const c_char;
    fn __clk_get_hw(clk: *mut clk) -> *mut clk_hw;
    fn ti_dt_clk_name(node: *mut device_node) -> *const c_char;
    fn of_clk_get_parent_count(node: *mut device_node) -> c_uint;
    fn of_clk_get(node: *mut device_node, index: c_int) -> *mut clk;
    fn IS_ERR(ptr: *mut clk) -> bool;
    fn PTR_ERR(ptr: *mut clk) -> isize;
    fn clk_put(clk: *mut clk);
    fn omap2_clk_is_hw_omap(hw: *mut clk_hw) -> bool;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn WARN(condition: c_int, fmt: *const c_char, ...);
    fn for_each_matching_node(
        table: *const of_device_id,
        callback: unsafe fn(*mut device_node),
    );
}

pub unsafe fn omap2_clkops_enable_clkdm(hw: *mut clk_hw) -> c_int {
    let clk = to_clk_hw_omap(hw);
    let mut ret: c_int = 0;

    if (*clk).clkdm.is_null() {
        pr_err(c"%s: %s: no clkdm set ?!\n".as_ptr(), c"omap2_clkops_enable_clkdm".as_ptr(), clk_hw_get_name(hw));
        return -22;
    }

    if (*ti_clk_get_features()).flags & TI_CLK_DISABLE_CLKDM_CONTROL != 0 {
        pr_err(c"%s: %s: clkfw-based clockdomain control disabled ?!\n".as_ptr(), c"omap2_clkops_enable_clkdm".as_ptr(), clk_hw_get_name(hw));
        return 0;
    }

    ret = ((*ti_clk_ll_ops).clkdm_clk_enable)((*clk).clkdm, (*hw).clk);
    WARN(ret, c"%s: could not enable %s's clockdomain %s: %d\n".as_ptr(), c"omap2_clkops_enable_clkdm".as_ptr(), clk_hw_get_name(hw), (*clk).clkdm_name, ret);
    ret
}

pub unsafe fn omap2_clkops_disable_clkdm(hw: *mut clk_hw) {
    let clk = to_clk_hw_omap(hw);

    if (*clk).clkdm.is_null() {
        pr_err(c"%s: %s: no clkdm set ?!\n".as_ptr(), c"omap2_clkops_disable_clkdm".as_ptr(), clk_hw_get_name(hw));
        return;
    }

    if (*ti_clk_get_features()).flags & TI_CLK_DISABLE_CLKDM_CONTROL != 0 {
        pr_err(c"%s: %s: clkfw-based clockdomain control disabled ?!\n".as_ptr(), c"omap2_clkops_disable_clkdm".as_ptr(), clk_hw_get_name(hw));
        return;
    }

    ((*ti_clk_ll_ops).clkdm_clk_disable)((*clk).clkdm, (*hw).clk);
}

pub unsafe fn omap2_init_clk_clkdm(hw: *mut clk_hw) -> c_int {
    let clk = to_clk_hw_omap(hw);
    if (*clk).clkdm_name.is_null() {
        return 0;
    }

    let clk_name = __clk_get_name((*hw).clk);
    let clkdm = ((*ti_clk_ll_ops).clkdm_lookup)((*clk).clkdm_name);
    if !clkdm.is_null() {
        pr_debug(c"clock: associated clk %s to clkdm %s\n".as_ptr(), clk_name, (*clk).clkdm_name);
        (*clk).clkdm = clkdm;
    } else {
        pr_debug(c"clock: could not associate clk %s to clkdm %s\n".as_ptr(), clk_name, (*clk).clkdm_name);
    }
    0
}

unsafe fn of_ti_clockdomain_setup(node: *mut device_node) {
    let clkdm_name = ti_dt_clk_name(node);
    let num_clks = of_clk_get_parent_count(node);

    for i in 0..num_clks {
        let clk = of_clk_get(node, i as c_int);
        if IS_ERR(clk) {
            pr_err(c"%s: Failed get %pOF' clock nr %d (%ld)\n".as_ptr(), c"of_ti_clockdomain_setup".as_ptr(), node, i, PTR_ERR(clk));
            continue;
        }
        let clk_hw = __clk_get_hw(clk);
        if !omap2_clk_is_hw_omap(clk_hw) {
            pr_warn(c"can't setup clkdm for basic clk %s\n".as_ptr(), __clk_get_name(clk));
            clk_put(clk);
            continue;
        }
        (*to_clk_hw_omap(clk_hw)).clkdm_name = clkdm_name;
        omap2_init_clk_clkdm(clk_hw);
        clk_put(clk);
    }
}

pub unsafe fn ti_dt_clockdomains_setup() {
    for_each_matching_node(TI_CLKDM_MATCH_TABLE.as_ptr(), of_ti_clockdomain_setup);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
