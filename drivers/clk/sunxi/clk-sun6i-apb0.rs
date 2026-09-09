// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Free Electrons
 *
 * Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
 *
 * Allwinner A31 APB0 clock driver
 */

// Translated dependencies from:
// <linux/clk-provider.h>, <linux/init.h>, <linux/of.h>,
// <linux/platform_device.h>

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct clk_div_table {
    pub val: c_uint,
    pub div: c_uint,
}

#[repr(C)]
pub struct device_node {
    pub name: *const c_char,
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
pub struct clk {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
    ) -> *mut c_void;
    fn ptr_err(ptr: *mut c_void) -> c_int;
    fn of_clk_get_parent_name(np: *mut device_node, index: c_int) -> *const c_char;
    fn of_property_read_string(
        np: *mut device_node,
        propname: *const c_char,
        out_string: *mut *const c_char,
    ) -> c_int;
    fn clk_register_divider_table(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        reg: *mut c_void,
        shift: u8,
        width: u8,
        clk_divider_flags: c_uint,
        table: *const clk_div_table,
        lock: *mut c_void,
    ) -> *mut clk;
    fn of_clk_add_provider(
        np: *mut device_node,
        get: *const c_void,
        data: *mut clk,
    ) -> c_int;
    fn of_clk_src_simple_get(_np: *mut device_node, _args: *const c_void) -> *mut clk;
    fn builtin_platform_driver(driver: *mut platform_driver);
}

type c_ulong = usize;

const EINVAL: c_int = 22;

/*
 * The APB0 clk has a configurable divisor.
 *
 * We must use a clk_div_table and not a regular power of 2
 * divisor here, because the first 2 values divide the clock
 * by 2.
 */
static SUN6I_A31_APB0_DIVS: [clk_div_table; 5] = [
    clk_div_table { val: 0, div: 2 },
    clk_div_table { val: 1, div: 2 },
    clk_div_table { val: 2, div: 4 },
    clk_div_table { val: 3, div: 8 },
    clk_div_table { val: 0, div: 0 }, // sentinel
];

unsafe extern "C" fn sun6i_a31_apb0_clk_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let mut clk_name = (*np).name;
    let clk_parent: *const c_char;
    let reg: *mut c_void;
    let clk: *mut clk;

    reg = devm_platform_ioremap_resource(pdev, 0);
    if (reg as isize) < 0 {
        return ptr_err(reg);
    }

    clk_parent = of_clk_get_parent_name(np, 0);
    if clk_parent.is_null() {
        return -EINVAL;
    }

    of_property_read_string(
        np,
        b"clock-output-names\0".as_ptr() as *const c_char,
        &mut clk_name,
    );

    clk = clk_register_divider_table(
        &mut (*pdev).dev,
        clk_name,
        clk_parent,
        0,
        reg,
        0,
        2,
        0,
        SUN6I_A31_APB0_DIVS.as_ptr(),
        core::ptr::null_mut(),
    );
    if (clk as isize) < 0 {
        return ptr_err(clk as *mut c_void);
    }

    of_clk_add_provider(np, of_clk_src_simple_get as *const c_void, clk)
}

static SUN6I_A31_APB0_CLK_DT_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: b"allwinner,sun6i-a31-apb0-clk\0".as_ptr() as *const c_char,
    },
    of_device_id { compatible: core::ptr::null() }, // sentinel
];

static mut SUN6I_A31_APB0_CLK_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"sun6i-a31-apb0-clk\0".as_ptr() as *const c_char,
        of_match_table: SUN6I_A31_APB0_CLK_DT_IDS.as_ptr(),
    },
    probe: Some(sun6i_a31_apb0_clk_probe),
};

// builtin_platform_driver(sun6i_a31_apb0_clk_driver);
unsafe fn register_builtin_driver() {
    builtin_platform_driver(&raw mut SUN6I_A31_APB0_CLK_DRIVER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
