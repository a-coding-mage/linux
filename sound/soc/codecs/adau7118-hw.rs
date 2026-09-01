// SPDX-License-Identifier: GPL-2.0
//
// Analog Devices ADAU7118 8 channel PDM-to-I2S/TDM Converter Standalone Hw
// driver
//
// Copyright 2019 Analog Devices Inc.

// C dependencies:
// #include <linux/module.h>
// #include <linux/platform_device.h>
// #include "adau7118.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

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
pub struct platform_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub id_table: *const platform_device_id,
}

extern "C" {
    fn adau7118_probe(dev: *mut device, regmap: *mut c_void, hw_mode: bool) -> c_int;
}

unsafe extern "C" fn adau7118_probe_hw(pdev: *mut platform_device) -> c_int {
    unsafe { adau7118_probe(core::ptr::addr_of_mut!((*pdev).dev), ptr::null_mut(), true) }
}

static ADAU7118_COMPATIBLE: &[u8; 13] = b"adi,adau7118\0";
static ADAU7118_NAME: &[u8; 9] = b"adau7118\0";

static ADAU7118_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: ADAU7118_COMPATIBLE.as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, adau7118_of_match);

static ADAU7118_ID: [platform_device_id; 2] = [
    platform_device_id {
        name: ADAU7118_NAME.as_ptr() as *const c_char,
    },
    platform_device_id { name: ptr::null() },
];

// MODULE_DEVICE_TABLE(platform, adau7118_id);

static mut ADAU7118_DRIVER_HW: platform_driver = platform_driver {
    driver: device_driver {
        name: ADAU7118_NAME.as_ptr() as *const c_char,
        of_match_table: ADAU7118_OF_MATCH.as_ptr(),
    },
    probe: Some(adau7118_probe_hw),
    id_table: ADAU7118_ID.as_ptr(),
};

// module_platform_driver(adau7118_driver_hw);

// MODULE_AUTHOR("Nuno Sa <nuno.sa@analog.com>");
// MODULE_DESCRIPTION("ADAU7118 8 channel PDM-to-I2S/TDM Converter driver for standalone hw mode");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
