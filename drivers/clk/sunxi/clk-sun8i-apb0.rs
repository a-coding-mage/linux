// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Chen-Yu Tsai
 * Author: Chen-Yu Tsai <wens@csie.org>
 *
 * Allwinner A23 APB0 clock driver
 *
 * Based on clk-sun6i-apb0.c
 * Allwinner A31 APB0 clock driver
 *
 * Copyright (C) 2014 Free Electrons
 * Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device_node {
    pub name: *const c_char,
}
#[repr(C)]
pub struct clk;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct resource {
    pub start: usize,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}
#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

extern "C" {
    fn of_clk_get_parent_name(node: *mut device_node, index: u32) -> *const c_char;
    fn of_property_read_string(
        node: *mut device_node,
        name: *const c_char,
        value: *mut *const c_char,
    ) -> c_int;
    fn clk_register_divider(
        dev: *mut c_void,
        name: *const c_char,
        parent_name: *const c_char,
        flags: u32,
        reg: *mut c_void,
        shift: u8,
        width: u8,
        clk_divider_flags: u8,
        lock: *mut c_void,
    ) -> *mut clk;
    fn of_clk_add_provider(
        node: *mut device_node,
        get: Option<unsafe extern "C" fn(*mut device_node, *mut c_void) -> *mut clk>,
        data: *mut clk,
    ) -> c_int;
    fn of_clk_src_simple_get(node: *mut device_node, data: *mut c_void) -> *mut clk;
    fn clk_unregister_divider(clk: *mut clk);
    fn of_io_request_and_map(node: *mut device_node, index: u32, name: *const c_char) -> *mut c_void;
    fn of_node_full_name(node: *mut device_node) -> *const c_char;
    fn ptr_err(ptr: *mut c_void) -> isize;
    fn iounmap(addr: *mut c_void);
    fn of_address_to_resource(node: *mut device_node, index: u32, resource: *mut resource) -> c_int;
    fn resource_size(resource: *const resource) -> usize;
    fn release_mem_region(start: usize, size: usize);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut c_void;
}

const EINVAL: isize = 22;

unsafe fn sun8i_a23_apb0_register(node: *mut device_node, reg: *mut c_void) -> *mut clk {
    let mut clk_name = (*node).name;
    let clk_parent = of_clk_get_parent_name(node, 0);
    if clk_parent.is_null() {
        return (-EINVAL) as *mut clk;
    }

    let clock_output_names = b"clock-output-names\0";
    of_property_read_string(node, clock_output_names.as_ptr() as *const c_char, &mut clk_name);

    // The A23 APB0 clock is a standard 2 bit wide divider clock
    let clk = clk_register_divider(
        core::ptr::null_mut(), clk_name, clk_parent, 0, reg, 0, 2, 0,
        core::ptr::null_mut(),
    );
    if (clk as isize) < 0 {
        return clk;
    }

    let ret = of_clk_add_provider(node, Some(of_clk_src_simple_get), clk);
    if ret != 0 {
        clk_unregister_divider(clk);
        return (-ret as isize) as *mut clk;
    }
    clk
}

unsafe fn sun8i_a23_apb0_setup(node: *mut device_node) {
    let reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (reg as isize) < 0 {
        // This happens with clk nodes instantiated through mfd, as those do not
        // have their resources assigned in the device tree. Do not print an
        // error in this case.
        if ptr_err(reg) != -EINVAL {
            // pr_err("Could not get registers for a23-apb0-clk\n");
        }
        return;
    }

    let clk = sun8i_a23_apb0_register(node, reg);
    if (clk as isize) < 0 {
        iounmap(reg);
        let mut res = resource { start: 0 };
        of_address_to_resource(node, 0, &mut res);
        release_mem_region(res.start, resource_size(&res));
    }
}

// CLK_OF_DECLARE_DRIVER(sun8i_a23_apb0, "allwinner,sun8i-a23-apb0-clk",
//                       sun8i_a23_apb0_setup);

unsafe extern "C" fn sun8i_a23_apb0_clk_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let reg = devm_platform_ioremap_resource(pdev, 0);
    if (reg as isize) < 0 {
        return ptr_err(reg) as c_int;
    }
    let clk = sun8i_a23_apb0_register(np, reg);
    if (clk as isize) < 0 { ptr_err(clk as *mut c_void) as c_int } else { 0 }
}

static SUN8I_A23_APB0_CLK_DT_IDS: [of_device_id; 2] = [
    of_device_id { compatible: b"allwinner,sun8i-a23-apb0-clk\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

static mut SUN8I_A23_APB0_CLK_DRIVER: platform_driver = platform_driver {
    driver: driver {
        name: b"sun8i-a23-apb0-clk\0".as_ptr() as *const c_char,
        of_match_table: SUN8I_A23_APB0_CLK_DT_IDS.as_ptr(),
    },
    probe: Some(sun8i_a23_apb0_clk_probe),
};

// builtin_platform_driver(sun8i_a23_apb0_clk_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
