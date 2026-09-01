// SPDX-License-Identifier: GPL-2.0
//
// CS530x CODEC driver
//
// Copyright (C) 2025 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// Dependencies supplied by the original C includes:
// linux/module.h, linux/platform_device.h, linux/spi/spi.h, and "cs530x.h".

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

extern "C" {
    static cs530x_regmap_spi: regmap_config;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn spi_get_device_match_data(spi: *mut spi_device) -> *const c_void;
    fn cs530x_probe(cs530x: *mut cs530x_priv) -> c_int;
}

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;

extern "C" {
    static CS4282: c_ulong;
    static CS4302: c_ulong;
    static CS4304: c_ulong;
    static CS4308: c_ulong;
    static CS5302: c_ulong;
    static CS5304: c_ulong;
    static CS5308: c_ulong;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
pub struct cs530x_priv {
    pub regmap: *mut regmap,
    pub devtype: c_ulong,
    pub dev: *mut device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct spi_device_id {
    pub name: *const c_char,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub id_table: *const spi_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

static cs530x_of_match: [of_device_id; 8] = [
    of_device_id {
        compatible: b"cirrus,cs4282\0".as_ptr() as *const c_char,
        data: unsafe { CS4282 as *const c_void },
    },
    of_device_id {
        compatible: b"cirrus,cs4302\0".as_ptr() as *const c_char,
        data: unsafe { CS4302 as *const c_void },
    },
    of_device_id {
        compatible: b"cirrus,cs4304\0".as_ptr() as *const c_char,
        data: unsafe { CS4304 as *const c_void },
    },
    of_device_id {
        compatible: b"cirrus,cs4308\0".as_ptr() as *const c_char,
        data: unsafe { CS4308 as *const c_void },
    },
    of_device_id {
        compatible: b"cirrus,cs5302\0".as_ptr() as *const c_char,
        data: unsafe { CS5302 as *const c_void },
    },
    of_device_id {
        compatible: b"cirrus,cs5304\0".as_ptr() as *const c_char,
        data: unsafe { CS5304 as *const c_void },
    },
    of_device_id {
        compatible: b"cirrus,cs5304\0".as_ptr() as *const c_char,
        data: unsafe { CS5308 as *const c_void },
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, cs530x_of_match);

static cs530x_spi_id: [spi_device_id; 8] = [
    spi_device_id {
        name: b"cs4282\0".as_ptr() as *const c_char,
        driver_data: unsafe { CS4282 },
    },
    spi_device_id {
        name: b"cs4302\0".as_ptr() as *const c_char,
        driver_data: unsafe { CS4302 },
    },
    spi_device_id {
        name: b"cs4304\0".as_ptr() as *const c_char,
        driver_data: unsafe { CS4304 },
    },
    spi_device_id {
        name: b"cs4308\0".as_ptr() as *const c_char,
        driver_data: unsafe { CS4308 },
    },
    spi_device_id {
        name: b"cs5302\0".as_ptr() as *const c_char,
        driver_data: unsafe { CS5302 },
    },
    spi_device_id {
        name: b"cs5304\0".as_ptr() as *const c_char,
        driver_data: unsafe { CS5304 },
    },
    spi_device_id {
        name: b"cs5308\0".as_ptr() as *const c_char,
        driver_data: unsafe { CS5308 },
    },
    spi_device_id {
        name: ptr::null(),
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, cs530x_spi_id);

unsafe extern "C" fn cs530x_spi_probe(spi: *mut spi_device) -> c_int {
    let cs530x: *mut cs530x_priv;
    let dev: *mut device = &mut (*spi).dev;
    let ret: c_int;

    cs530x = devm_kzalloc(dev, size_of::<cs530x_priv>(), GFP_KERNEL) as *mut cs530x_priv;
    if cs530x == ptr::null_mut() {
        return -ENOMEM;
    }

    spi_set_drvdata(spi, cs530x as *mut c_void);

    (*cs530x).regmap = devm_regmap_init_spi(spi, &cs530x_regmap_spi);
    if IS_ERR((*cs530x).regmap as *const c_void) {
        ret = PTR_ERR((*cs530x).regmap as *const c_void);
        dev_err(
            dev,
            b"Failed to allocate register map: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    (*cs530x).devtype = spi_get_device_match_data(spi) as c_ulong;
    (*cs530x).dev = &mut (*spi).dev;

    cs530x_probe(cs530x)
}

static mut cs530x_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: b"cs530x\0".as_ptr() as *const c_char,
        of_match_table: cs530x_of_match.as_ptr(),
    },
    id_table: cs530x_spi_id.as_ptr(),
    probe: Some(cs530x_spi_probe),
};

// module_spi_driver(cs530x_spi_driver);
//
// MODULE_DESCRIPTION("SPI CS530X driver");
// MODULE_IMPORT_NS("SND_SOC_CS530X");
// MODULE_AUTHOR("Vitaly Rodionov <vitalyr@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
