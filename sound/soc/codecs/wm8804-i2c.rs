// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8804-i2c.c  --  WM8804 S/PDIF transceiver driver - I2C
 *
 * Copyright 2015 Cirrus Logic Inc
 *
 * Author: Charles Keepax <ckeepax@opensource.wolfsonmicro.com>
 */

// C dependencies: <linux/init.h>, <linux/module.h>, <linux/i2c.h>,
// <linux/acpi.h>, and "wm8804.h".

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
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [::core::ffi::c_char; 20],
    pub driver_data: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct of_device_id {
    pub name: [::core::ffi::c_char; 32],
    pub type_: [::core::ffi::c_char; 32],
    pub compatible: [::core::ffi::c_char; 128],
    pub data: *const ::core::ffi::c_void,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [::core::ffi::c_char; 16],
    pub driver_data: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const ::core::ffi::c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    static wm8804_regmap_config: regmap_config;
    static wm8804_pm: dev_pm_ops;

    fn devm_regmap_init_i2c(
        i2c: *mut i2c_client,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const ::core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const ::core::ffi::c_void) -> ::core::ffi::c_long;
    fn wm8804_probe(dev: *mut device, regmap: *mut regmap) -> ::core::ffi::c_int;
    fn wm8804_remove(dev: *mut device);
}

const fn c_char_array_20(s: &[u8]) -> [::core::ffi::c_char; 20] {
    let mut out = [0 as ::core::ffi::c_char; 20];
    let mut i = 0;

    while i < s.len() && i < 20 {
        out[i] = s[i] as ::core::ffi::c_char;
        i += 1;
    }

    out
}

const fn c_char_array_16(s: &[u8]) -> [::core::ffi::c_char; 16] {
    let mut out = [0 as ::core::ffi::c_char; 16];
    let mut i = 0;

    while i < s.len() && i < 16 {
        out[i] = s[i] as ::core::ffi::c_char;
        i += 1;
    }

    out
}

const fn c_char_array_128(s: &[u8]) -> [::core::ffi::c_char; 128] {
    let mut out = [0 as ::core::ffi::c_char; 128];
    let mut i = 0;

    while i < s.len() && i < 128 {
        out[i] = s[i] as ::core::ffi::c_char;
        i += 1;
    }

    out
}

unsafe extern "C" fn wm8804_i2c_probe(i2c: *mut i2c_client) -> ::core::ffi::c_int {
    let regmap: *mut regmap;

    regmap = devm_regmap_init_i2c(i2c, &wm8804_regmap_config);
    if IS_ERR(regmap as *const ::core::ffi::c_void) {
        return PTR_ERR(regmap as *const ::core::ffi::c_void) as ::core::ffi::c_int;
    }

    wm8804_probe(&mut (*i2c).dev, regmap)
}

unsafe extern "C" fn wm8804_i2c_remove(i2c: *mut i2c_client) {
    wm8804_remove(&mut (*i2c).dev);
}

static wm8804_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: c_char_array_20(b"wm8804"),
        driver_data: 0,
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, wm8804_i2c_id);

// #if defined(CONFIG_OF)
static wm8804_of_match: [of_device_id; 2] = [
    of_device_id {
        name: [0; 32],
        type_: [0; 32],
        compatible: c_char_array_128(b"wlf,wm8804"),
        data: ::core::ptr::null(),
    },
    of_device_id {
        name: [0; 32],
        type_: [0; 32],
        compatible: [0; 128],
        data: ::core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, wm8804_of_match);
// #endif

// #ifdef CONFIG_ACPI
static wm8804_acpi_match: [acpi_device_id; 3] = [
    acpi_device_id {
        id: c_char_array_16(b"1AEC8804"),
        driver_data: 0,
    }, /* Wolfson PCI ID + part ID */
    acpi_device_id {
        id: c_char_array_16(b"10138804"),
        driver_data: 0,
    }, /* Cirrus Logic PCI ID + part ID */
    acpi_device_id {
        id: [0; 16],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, wm8804_acpi_match);
// #endif

unsafe fn pm_ptr(ptr: *const dev_pm_ops) -> *const dev_pm_ops {
    ptr
}

unsafe fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id {
    ptr
}

unsafe fn ACPI_PTR(ptr: *const acpi_device_id) -> *const acpi_device_id {
    ptr
}

static mut wm8804_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"wm8804\0".as_ptr() as *const ::core::ffi::c_char,
        pm: unsafe { pm_ptr(&wm8804_pm) },
        of_match_table: unsafe { of_match_ptr(wm8804_of_match.as_ptr()) },
        acpi_match_table: unsafe { ACPI_PTR(wm8804_acpi_match.as_ptr()) },
    },
    probe: Some(wm8804_i2c_probe),
    remove: Some(wm8804_i2c_remove),
    id_table: wm8804_i2c_id.as_ptr(),
};

// module_i2c_driver(wm8804_i2c_driver);

// MODULE_DESCRIPTION("ASoC WM8804 driver - I2C");
// MODULE_AUTHOR("Charles Keepax <ckeepax@opensource.wolfsonmicro.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
