// SPDX-License-Identifier: GPL-2.0-only
/*
 * MOXA ART SoCs clock driver.
 *
 * Copyright (C) 2013 Jonas Jensen
 *
 * Jonas Jensen <jonas.jensen@gmail.com>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct device_node {
    pub name: *const c_char,
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

extern "C" {
    fn of_property_read_string(
        node: *const device_node,
        propname: *const c_char,
        out_string: *mut *const c_char,
    ) -> i32;
    fn of_clk_get_parent_name(node: *const device_node, index: u32) -> *const c_char;
    fn of_iomap(node: *const device_node, index: i32) -> *mut c_void;
    fn readl(addr: *const c_void) -> u32;
    fn iounmap(addr: *mut c_void);
    fn clk_hw_register_fixed_factor(
        dev: *mut c_void,
        name: *const c_char,
        parent_name: *const c_char,
        flags: u32,
        mult: u32,
        div: u32,
    ) -> *mut clk_hw;
    fn clk_hw_register_clkdev(hw: *mut clk_hw, con_id: *const c_char, dev_id: *const c_char);
    fn of_clk_add_hw_provider(
        node: *const device_node,
        get: *const c_void,
        data: *mut clk_hw,
    ) -> i32;
    fn of_clk_hw_simple_get() -> *mut c_void;
    fn pr_err(fmt: *const c_char, ...);
}

#[inline]
unsafe fn is_err<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0
}

unsafe extern "C" fn moxart_of_pll_clk_init(node: *mut device_node) {
    let mut base: *mut c_void;
    let mut hw: *mut clk_hw;
    let mul: u32;
    let mut name: *const c_char = (*node).name;
    let parent_name: *const c_char;

    of_property_read_string(
        node,
        b"clock-output-names\0".as_ptr() as *const c_char,
        &mut name,
    );
    parent_name = of_clk_get_parent_name(node, 0);

    base = of_iomap(node, 0);
    if base.is_null() {
        pr_err(b"%pOF: of_iomap failed\n\0".as_ptr() as *const c_char, node);
        return;
    }

    mul = (readl(base.add(0x30)) >> 3) & 0x3f;
    iounmap(base);

    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), name, parent_name, 0, mul, 1);
    if is_err(hw) {
        pr_err(b"%pOF: failed to register clock\n\0".as_ptr() as *const c_char, node);
        return;
    }

    clk_hw_register_clkdev(hw, core::ptr::null(), name);
    of_clk_add_hw_provider(node, of_clk_hw_simple_get as *const c_void, hw);
}

unsafe extern "C" fn moxart_of_apb_clk_init(node: *mut device_node) {
    let mut base: *mut c_void;
    let mut hw: *mut clk_hw;
    let div: u32;
    let mut val: u32;
    let div_idx: [u32; 5] = [2, 3, 4, 6, 8];
    let mut name: *const c_char = (*node).name;
    let parent_name: *const c_char;

    of_property_read_string(
        node,
        b"clock-output-names\0".as_ptr() as *const c_char,
        &mut name,
    );
    parent_name = of_clk_get_parent_name(node, 0);

    base = of_iomap(node, 0);
    if base.is_null() {
        pr_err(b"%pOF: of_iomap failed\n\0".as_ptr() as *const c_char, node);
        return;
    }

    val = (readl(base.add(0xc)) >> 4) & 0x7;
    iounmap(base);

    if val > 4 {
        val = 0;
    }
    div = div_idx[val as usize] * 2;

    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), name, parent_name, 0, 1, div);
    if is_err(hw) {
        pr_err(b"%pOF: failed to register clock\n\0".as_ptr() as *const c_char, node);
        return;
    }

    clk_hw_register_clkdev(hw, core::ptr::null(), name);
    of_clk_add_hw_provider(node, of_clk_hw_simple_get as *const c_void, hw);
}

// CLK_OF_DECLARE(moxart_pll_clock, "moxa,moxart-pll-clock", moxart_of_pll_clk_init);
// CLK_OF_DECLARE(moxart_apb_clock, "moxa,moxart-apb-clock", moxart_of_apb_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
