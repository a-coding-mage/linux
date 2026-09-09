// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2012 John Crispin <john@phrozen.org>
 *  Copyright (C) 2010 Sameer Ahmad, Lantiq GmbH
 */

use core::ffi::c_void;

// Linux kernel declarations supplied by the surrounding translation unit.
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

extern "C" {
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: i32,
        res: *mut *mut c_void,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *mut c_void) -> bool;
    fn PTR_ERR(ptr: *mut c_void) -> i32;
    fn dev_info(dev: *const device, fmt: *const core::ffi::c_char, ...);
    fn ltq_w8(value: u8, address: *mut c_void);
    fn ltq_r8(address: *mut c_void) -> u8;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

/* Bias and regulator Setup Register */
const DCDC_BIAS_VREG0: u8 = 0xa;
/* Bias and regulator Setup Register */
const DCDC_BIAS_VREG1: u8 = 0xb;

unsafe fn dcdc_w8(x: u8, y: usize) {
    ltq_w8(x, dcdc_membase.add(y));
}

unsafe fn dcdc_r8(x: usize) -> u8 {
    ltq_r8(dcdc_membase.add(x))
}

static mut dcdc_membase: *mut c_void = core::ptr::null_mut();

unsafe extern "C" fn dcdc_probe(pdev: *mut platform_device) -> i32 {
    dcdc_membase = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if IS_ERR(dcdc_membase) {
        return PTR_ERR(dcdc_membase);
    }

    dev_info(
        &(*pdev).dev,
        b"Core Voltage : %d mV\n\0".as_ptr() as *const core::ffi::c_char,
        dcdc_r8(DCDC_BIAS_VREG1 as usize) as i32 * 8,
    );

    0
}

static dcdc_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"lantiq,dcdc-xrx200\0".as_ptr() as *const core::ffi::c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

static mut dcdc_driver: platform_driver = platform_driver {
    probe: Some(dcdc_probe),
    driver: driver {
        name: b"dcdc-xrx200\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: dcdc_match.as_ptr(),
    },
};

unsafe extern "C" fn dcdc_init() -> i32 {
    let ret = platform_driver_register(&raw mut dcdc_driver);

    if ret != 0 {
        pr_info(b"dcdc: Error registering platform driver\n\0".as_ptr() as *const core::ffi::c_char);
    }
    ret
}

// arch_initcall(dcdc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
