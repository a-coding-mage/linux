// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for Altera Partial Reconfiguration IP Core
 *
 * Copyright (C) 2016-2017 Intel Corporation
 *
 * Based on socfpga-a10.c Copyright (C) 2015-2016 Altera Corporation
 *  by Alan Tull <atull@opensource.altera.com>
 */

// Dependencies supplied by the Linux kernel and the Altera PR IP core.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: c_int,
    ) -> *mut c_void;
    fn is_err(ptr: *const c_void) -> bool;
    fn ptr_err(ptr: *const c_void) -> c_int;
    fn alt_pr_register(dev: *mut device, reg_base: *mut c_void) -> c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

unsafe extern "C" fn alt_pr_platform_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = unsafe { &mut (*pdev).dev };
    let reg_base: *mut c_void;

    /* First mmio base is for register access */
    reg_base = unsafe { devm_platform_ioremap_resource(pdev, 0) };
    if unsafe { is_err(reg_base) } {
        return unsafe { ptr_err(reg_base) };
    }

    unsafe { alt_pr_register(dev, reg_base) }
}

static alt_pr_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"altr,a10-pr-ip\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

static mut alt_pr_platform_driver: platform_driver = platform_driver {
    probe: Some(alt_pr_platform_probe),
    driver: driver {
        name: b"alt_a10_pr_ip\0".as_ptr() as *const c_char,
        of_match_table: alt_pr_of_match.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, alt_pr_of_match);
// module_platform_driver(alt_pr_platform_driver);
// MODULE_AUTHOR("Matthew Gerlach <matthew.gerlach@linux.intel.com>");
// MODULE_DESCRIPTION("Altera Partial Reconfiguration IP Platform Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
