// SPDX-License-Identifier: GPL-2.0
//
// cs35l41-spi.c -- CS35l41 SPI driver
//
// Copyright 2017-2021 Cirrus Logic, Inc.
//
// Author: David Rhodes	<david.rhodes@cirrus.com>

// C dependencies:
// linux/acpi.h, linux/delay.h, linux/init.h, linux/kernel.h, linux/module.h,
// linux/moduleparam.h, linux/platform_device.h, linux/spi/spi.h, "cs35l41.h"

extern "C" {
    static cs35l41_regmap_spi: regmap_config;
    static cs35l41_pm_ops: dev_pm_ops;

    fn dev_get_platdata(dev: *mut device) -> *mut cs35l41_hw_cfg;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn spi_setup(spi: *mut spi_device) -> core::ffi::c_int;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut core::ffi::c_void);
    fn spi_get_drvdata(spi: *mut spi_device) -> *mut core::ffi::c_void;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> core::ffi::c_long;
    fn dev_err_probe(
        dev: *mut device,
        err: core::ffi::c_long,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn cs35l41_probe(cs35l41: *mut cs35l41_private, hw_cfg: *mut cs35l41_hw_cfg)
        -> core::ffi::c_int;
    fn cs35l41_remove(cs35l41: *mut cs35l41_private);
    fn pm_ptr(ptr: *const dev_pm_ops) -> *const dev_pm_ops;
    fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id;
    fn ACPI_PTR(ptr: *const acpi_device_id) -> *const acpi_device_id;
    fn module_spi_driver(driver: *mut spi_driver);
}

type gfp_t = core::ffi::c_uint;

const GFP_KERNEL: gfp_t = 0;
const ENOMEM: core::ffi::c_int = 12;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cs35l41_hw_cfg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cs35l41_private {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq: core::ffi::c_int,
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
    pub max_speed_hz: core::ffi::c_uint,
    pub irq: core::ffi::c_int,
}

#[repr(C)]
pub struct spi_device_id {
    pub name: [core::ffi::c_char; 32],
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [core::ffi::c_char; 16],
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub id_table: *const spi_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut spi_device)>,
}

type kernel_ulong_t = core::ffi::c_ulong;

const CS35L41_SPI_MAX_FREQ: core::ffi::c_uint = 0;

const fn c_char_array_32(s: &[u8]) -> [core::ffi::c_char; 32] {
    let mut out = [0 as core::ffi::c_char; 32];
    let mut i = 0;

    while i < s.len() && i < 31 {
        out[i] = s[i] as core::ffi::c_char;
        i += 1;
    }

    out
}

const fn c_char_array_16(s: &[u8]) -> [core::ffi::c_char; 16] {
    let mut out = [0 as core::ffi::c_char; 16];
    let mut i = 0;

    while i < s.len() && i < 15 {
        out[i] = s[i] as core::ffi::c_char;
        i += 1;
    }

    out
}

static cs35l41_id_spi: [spi_device_id; 5] = [
    spi_device_id {
        name: c_char_array_32(b"cs35l40"),
        driver_data: 0,
    },
    spi_device_id {
        name: c_char_array_32(b"cs35l41"),
        driver_data: 0,
    },
    spi_device_id {
        name: c_char_array_32(b"cs35l51"),
        driver_data: 0,
    },
    spi_device_id {
        name: c_char_array_32(b"cs35l53"),
        driver_data: 0,
    },
    spi_device_id {
        name: [0 as core::ffi::c_char; 32],
        driver_data: 0,
    },
];

// MODULE_DEVICE_TABLE(spi, cs35l41_id_spi);

unsafe extern "C" fn cs35l41_spi_probe(spi: *mut spi_device) -> core::ffi::c_int {
    let regmap_config: *const regmap_config = &cs35l41_regmap_spi;
    let hw_cfg: *mut cs35l41_hw_cfg = dev_get_platdata(&mut (*spi).dev);
    let cs35l41: *mut cs35l41_private;
    let mut ret: core::ffi::c_int;

    cs35l41 = devm_kzalloc(
        &mut (*spi).dev,
        core::mem::size_of::<cs35l41_private>(),
        GFP_KERNEL,
    ) as *mut cs35l41_private;
    if cs35l41.is_null() {
        return -ENOMEM;
    }

    (*spi).max_speed_hz = CS35L41_SPI_MAX_FREQ;
    ret = spi_setup(spi);
    if ret < 0 {
        return ret;
    }

    spi_set_drvdata(spi, cs35l41 as *mut core::ffi::c_void);
    (*cs35l41).regmap = devm_regmap_init_spi(spi, regmap_config);
    if IS_ERR((*cs35l41).regmap as *const core::ffi::c_void) {
        return dev_err_probe(
            (*cs35l41).dev,
            PTR_ERR((*cs35l41).regmap as *const core::ffi::c_void),
            b"Failed to allocate register map\n\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    (*cs35l41).dev = &mut (*spi).dev;
    (*cs35l41).irq = (*spi).irq;

    cs35l41_probe(cs35l41, hw_cfg)
}

unsafe extern "C" fn cs35l41_spi_remove(spi: *mut spi_device) {
    let cs35l41: *mut cs35l41_private = spi_get_drvdata(spi) as *mut cs35l41_private;

    cs35l41_remove(cs35l41);
}

// CONFIG_OF conditional device match table.
static cs35l41_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: b"cirrus,cs35l40\0".as_ptr() as *const core::ffi::c_char,
    },
    of_device_id {
        compatible: b"cirrus,cs35l41\0".as_ptr() as *const core::ffi::c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, cs35l41_of_match);

// CONFIG_ACPI conditional device match table.
static cs35l41_acpi_match: [acpi_device_id; 3] = [
    acpi_device_id {
        id: c_char_array_16(b"CSC3541"), /* Cirrus Logic PnP ID + part ID */
        driver_data: 0,
    },
    acpi_device_id {
        id: c_char_array_16(b"CLSA3541"), /* Cirrus Logic PnP ID + part ID */
        driver_data: 0,
    },
    acpi_device_id {
        id: [0 as core::ffi::c_char; 16],
        driver_data: 0,
    },
];

// MODULE_DEVICE_TABLE(acpi, cs35l41_acpi_match);

static mut cs35l41_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: b"cs35l41\0".as_ptr() as *const core::ffi::c_char,
        pm: unsafe { pm_ptr(&cs35l41_pm_ops) },
        of_match_table: unsafe { of_match_ptr(cs35l41_of_match.as_ptr()) },
        acpi_match_table: unsafe { ACPI_PTR(cs35l41_acpi_match.as_ptr()) },
    },
    id_table: cs35l41_id_spi.as_ptr(),
    probe: Some(cs35l41_spi_probe),
    remove: Some(cs35l41_spi_remove),
};

#[used]
static __MODULE_SPI_DRIVER: unsafe extern "C" fn(*mut spi_driver) = module_spi_driver;

// module_spi_driver(cs35l41_spi_driver);
// MODULE_DESCRIPTION("SPI CS35L41 driver");
// MODULE_AUTHOR("David Rhodes, Cirrus Logic Inc, <david.rhodes@cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
