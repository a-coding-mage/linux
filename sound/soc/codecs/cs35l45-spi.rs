// SPDX-License-Identifier: GPL-2.0
//
// cs35l45-spi.c -- CS35L45 SPI driver
//
// Copyright 2019-2022 Cirrus Logic, Inc.
//
// Author: James Schulman <james.schulman@cirrus.com>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const CS35L45_SPI_MAX_FREQ: c_uint = 25_000_000;
const CONTROL_BUS_SPI: c_uint = 0;

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
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cs35l45_private {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub irq: c_int,
    pub bus_type: c_uint,
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
    pub max_speed_hz: c_uint,
    pub irq: c_int,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct spi_device_id {
    pub name: [c_char; 32],
    pub driver_data: usize,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub id_table: *const spi_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut spi_device)>,
}

unsafe extern "C" {
    static cs35l45_spi_regmap: regmap_config;
    static cs35l45_pm_ops: dev_pm_ops;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn spi_setup(spi: *mut spi_device) -> c_int;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn spi_get_drvdata(spi: *mut spi_device) -> *mut c_void;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn cs35l45_probe(cs35l45: *mut cs35l45_private) -> c_int;
    fn cs35l45_remove(cs35l45: *mut cs35l45_private);
}

#[inline]
unsafe fn pm_ptr<T>(ptr: *const T) -> *const T {
    ptr
}

unsafe extern "C" fn cs35l45_spi_probe(spi: *mut spi_device) -> c_int {
    let cs35l45: *mut cs35l45_private;
    let dev: *mut device = unsafe { &mut (*spi).dev };
    let ret: c_int;

    cs35l45 = unsafe {
        devm_kzalloc(
            dev,
            core::mem::size_of::<cs35l45_private>(),
            GFP_KERNEL,
        ) as *mut cs35l45_private
    };
    if cs35l45 == ptr::null_mut() {
        return -ENOMEM;
    }

    unsafe {
        (*spi).max_speed_hz = CS35L45_SPI_MAX_FREQ;
        spi_setup(spi);

        spi_set_drvdata(spi, cs35l45 as *mut c_void);
        (*cs35l45).regmap = devm_regmap_init_spi(spi, &cs35l45_spi_regmap);
    }
    if unsafe { IS_ERR((*cs35l45).regmap as *const c_void) } {
        ret = unsafe { PTR_ERR((*cs35l45).regmap as *const c_void) };
        unsafe {
            dev_err(
                dev,
                c"Failed to allocate register map: %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    unsafe {
        (*cs35l45).dev = dev;
        (*cs35l45).irq = (*spi).irq;
        (*cs35l45).bus_type = CONTROL_BUS_SPI;

        cs35l45_probe(cs35l45)
    }
}

unsafe extern "C" fn cs35l45_spi_remove(spi: *mut spi_device) {
    let cs35l45: *mut cs35l45_private =
        unsafe { spi_get_drvdata(spi) as *mut cs35l45_private };

    unsafe {
        cs35l45_remove(cs35l45);
    }
}

static cs35l45_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"cirrus,cs35l45".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, cs35l45_of_match);

static cs35l45_id_spi: [spi_device_id; 2] = [
    spi_device_id {
        name: [
            b'c' as c_char,
            b's' as c_char,
            b'3' as c_char,
            b'5' as c_char,
            b'l' as c_char,
            b'4' as c_char,
            b'5' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: 0,
    },
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(spi, cs35l45_id_spi);

static mut cs35l45_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"cs35l45".as_ptr(),
        of_match_table: cs35l45_of_match.as_ptr(),
        pm: unsafe { pm_ptr(&cs35l45_pm_ops) },
    },
    id_table: cs35l45_id_spi.as_ptr(),
    probe: Some(cs35l45_spi_probe),
    remove: Some(cs35l45_spi_remove),
};
// module_spi_driver(cs35l45_spi_driver);

// MODULE_DESCRIPTION("SPI CS35L45 driver");
// MODULE_AUTHOR("James Schulman, Cirrus Logic Inc, <james.schulman@cirrus.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_CS35L45");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
