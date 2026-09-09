// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel Low Power Subsystem clocks.
 *
 * Copyright (C) 2013, Intel Corporation
 * Authors: Mika Westerberg <mika.westerberg@linux.intel.com>
 *          Heikki Krogerus <heikki.krogerus@linux.intel.com>
 */

// External Linux kernel declarations supplied by the surrounding build.
use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lpss_clk_data {
    pub name: *const u8,
    pub clk: *mut clk,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const u8,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn clk_register_fixed_rate(
        dev: *mut device,
        name: *const u8,
        parent_name: *const u8,
        flags: u32,
        rate: u32,
    ) -> *mut clk;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn ptr_err(ptr: *mut clk) -> i32;
}

const GFP_KERNEL: u32 = 0;

unsafe extern "C" fn lpss_atom_clk_probe(pdev: *mut platform_device) -> i32 {
    let mut drvdata: *mut lpss_clk_data;
    let mut clk: *mut clk;

    drvdata = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<lpss_clk_data>(),
        GFP_KERNEL,
    ) as *mut lpss_clk_data;
    if drvdata.is_null() {
        return -12;
    }

    /* LPSS free running clock */
    (*drvdata).name = b"lpss_clk\0".as_ptr();
    clk = clk_register_fixed_rate(
        &mut (*pdev).dev,
        (*drvdata).name,
        core::ptr::null(),
        0,
        100000000,
    );
    if (clk as isize) < 0 {
        return ptr_err(clk);
    }

    (*drvdata).clk = clk;
    platform_set_drvdata(pdev, drvdata as *mut c_void);
    0
}

static mut lpss_atom_clk_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"clk-lpss-atom\0".as_ptr(),
    },
    probe: Some(lpss_atom_clk_probe),
};

pub unsafe extern "C" fn lpss_atom_clk_init() -> i32 {
    platform_driver_register(&mut lpss_atom_clk_driver)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
