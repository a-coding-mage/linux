// SPDX-License-Identifier: GPL-2.0-only
/*
 * TI Fixed Factor Clock
 *
 * Copyright (C) 2013 Texas Instruments, Inc.
 *
 * Tero Kristo <t-kristo@ti.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[allow(non_camel_case_types)]
pub type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

extern "C" {
    fn ti_dt_clk_name(node: *mut device_node) -> *const core::ffi::c_char;
    fn of_property_read_u32(
        node: *mut device_node,
        propname: *const core::ffi::c_char,
        out_value: *mut u32,
    ) -> core::ffi::c_int;
    fn of_property_read_bool(
        node: *mut device_node,
        propname: *const core::ffi::c_char,
    ) -> bool;
    fn of_clk_get_parent_name(
        node: *mut device_node,
        index: core::ffi::c_int,
    ) -> *const core::ffi::c_char;
    fn clk_register_fixed_factor(
        dev: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char,
        flags: u32,
        mult: u32,
        div: u32,
    ) -> *mut clk;
    fn of_clk_add_provider(
        node: *mut device_node,
        get: unsafe extern "C" fn(*mut device_node, *mut core::ffi::c_void) -> *mut clk,
        data: *mut clk,
    ) -> core::ffi::c_int;
    fn of_clk_src_simple_get(
        node: *mut device_node,
        data: *mut core::ffi::c_void,
    ) -> *mut clk;
    fn of_ti_clk_autoidle_setup(node: *mut device_node);
    fn ti_clk_add_alias(clk: *mut clk, name: *const core::ffi::c_char);
    fn __is_err(ptr: *mut clk) -> bool;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

const CLK_SET_RATE_PARENT: u32 = 1 << 0;

/**
 * of_ti_fixed_factor_clk_setup - Setup function for TI fixed factor clock
 * @node: device node for this clock
 *
 * Sets up a simple fixed factor clock based on device tree info.
 */
unsafe extern "C" fn of_ti_fixed_factor_clk_setup(node: *mut device_node) {
    let clk_name = ti_dt_clk_name(node);
    let mut parent_name: *const core::ffi::c_char;
    let mut div: u32 = 0;
    let mut mult: u32 = 0;
    let mut flags: u32 = 0;

    if of_property_read_u32(node, c"ti,clock-div".as_ptr(), &mut div) != 0 {
        pr_err(c"%pOFn must have a clock-div property\n".as_ptr(), node);
        return;
    }

    if of_property_read_u32(node, c"ti,clock-mult".as_ptr(), &mut mult) != 0 {
        pr_err(c"%pOFn must have a clock-mult property\n".as_ptr(), node);
        return;
    }

    if of_property_read_bool(node, c"ti,set-rate-parent".as_ptr()) {
        flags |= CLK_SET_RATE_PARENT;
    }

    parent_name = of_clk_get_parent_name(node, 0);

    let clk = clk_register_fixed_factor(
        core::ptr::null_mut(),
        clk_name,
        parent_name,
        flags,
        mult,
        div,
    );

    if !__is_err(clk) {
        of_clk_add_provider(node, of_clk_src_simple_get, clk as *mut core::ffi::c_void);
        of_ti_clk_autoidle_setup(node);
        ti_clk_add_alias(clk, clk_name);
    }
}

// CLK_OF_DECLARE(ti_fixed_factor_clk, "ti,fixed-factor-clock",
//                of_ti_fixed_factor_clk_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
