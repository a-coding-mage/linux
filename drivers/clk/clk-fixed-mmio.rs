// SPDX-License-Identifier: GPL-2.0

/*
 * Memory Mapped IO Fixed clock driver
 *
 * Copyright (C) 2018 Cadence Design Systems, Inc.
 *
 * Authors:
 *	Jan Kotas <jank@cadence.com>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

extern "C" {
    fn of_iomap(node: *mut device_node, index: c_int) -> *mut c_void;
    fn pr_err(format: *const c_char, ...);
    fn readl(addr: *mut c_void) -> u32;
    fn iounmap(addr: *mut c_void);
    fn of_property_read_string(
        node: *mut device_node,
        propname: *const c_char,
        out_string: *mut *const c_char,
    ) -> c_int;
    fn clk_hw_register_fixed_rate(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        flags: u32,
        rate: u32,
    ) -> *mut clk_hw;
    fn of_clk_add_hw_provider(
        node: *mut device_node,
        get: unsafe extern "C" fn(*mut device_node, *const c_void) -> *mut clk_hw,
        data: *mut clk_hw,
    ) -> c_int;
    fn clk_hw_unregister(clk: *mut clk_hw);
    fn of_clk_del_provider(node: *mut device_node);
    fn clk_hw_unregister_fixed_rate(clk: *mut clk_hw);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut clk_hw;
    fn of_clk_hw_simple_get(node: *mut device_node, data: *const c_void) -> *mut clk_hw;
}

const EIO: isize = 5;

unsafe fn err_ptr(error: isize) -> *mut clk_hw {
    error as *mut clk_hw
}

unsafe fn is_err(ptr: *mut clk_hw) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn ptr_err(ptr: *mut clk_hw) -> c_int {
    ptr as isize as c_int
}

unsafe fn fixed_mmio_clk_setup(node: *mut device_node) -> *mut clk_hw {
    let mut clk: *mut clk_hw;
    let mut clk_name: *const c_char = node as *const c_char;
    let base: *mut c_void;
    let freq: u32;
    let ret: c_int;

    base = of_iomap(node, 0);
    if base.is_null() {
        pr_err(b"%pOFn: failed to map address\0".as_ptr() as *const c_char, node);
        return err_ptr(-EIO);
    }

    freq = readl(base);
    iounmap(base);
    of_property_read_string(
        node,
        b"clock-output-names\0".as_ptr() as *const c_char,
        &mut clk_name,
    );

    clk = clk_hw_register_fixed_rate(core::ptr::null_mut(), clk_name, core::ptr::null(), 0, freq);
    if is_err(clk) {
        pr_err(
            b"%pOFn: failed to register fixed rate clock\0".as_ptr() as *const c_char,
            node,
        );
        return clk;
    }

    ret = of_clk_add_hw_provider(node, of_clk_hw_simple_get, clk);
    if ret != 0 {
        pr_err(
            b"%pOFn: failed to add clock provider\0".as_ptr() as *const c_char,
            node,
        );
        clk_hw_unregister(clk);
        clk = err_ptr(ret as isize);
    }

    clk
}

unsafe extern "C" fn of_fixed_mmio_clk_setup(node: *mut device_node) {
    fixed_mmio_clk_setup(node);
}

// Equivalent of CLK_OF_DECLARE(fixed_mmio_clk, "fixed-mmio-clock", of_fixed_mmio_clk_setup).

/*
 * This is not executed when of_fixed_mmio_clk_setup succeeded.
 */
unsafe extern "C" fn of_fixed_mmio_clk_probe(pdev: *mut platform_device) -> c_int {
    let clk: *mut clk_hw;

    clk = fixed_mmio_clk_setup((*pdev).dev.of_node);
    if is_err(clk) {
        return ptr_err(clk);
    }

    platform_set_drvdata(pdev, clk as *mut c_void);

    0
}

unsafe extern "C" fn of_fixed_mmio_clk_remove(pdev: *mut platform_device) {
    let clk: *mut clk_hw = platform_get_drvdata(pdev);

    of_clk_del_provider((*pdev).dev.of_node);
    clk_hw_unregister_fixed_rate(clk);
}

static OF_FIXED_MMIO_CLK_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: b"fixed-mmio-clock\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

// Equivalent of MODULE_DEVICE_TABLE(of, of_fixed_mmio_clk_ids).
static mut OF_FIXED_MMIO_CLK_DRIVER: platform_driver = platform_driver {
    driver: driver {
        name: b"of_fixed_mmio_clk\0".as_ptr() as *const c_char,
        of_match_table: OF_FIXED_MMIO_CLK_IDS.as_ptr(),
    },
    probe: Some(of_fixed_mmio_clk_probe),
    remove: Some(of_fixed_mmio_clk_remove),
};

// Equivalent of module_platform_driver(of_fixed_mmio_clk_driver).
// MODULE_AUTHOR("Jan Kotas <jank@cadence.com>");
// MODULE_DESCRIPTION("Memory Mapped IO Fixed clock driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
