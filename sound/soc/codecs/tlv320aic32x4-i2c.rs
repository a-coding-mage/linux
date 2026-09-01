// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2011-2019 NW Digital Radio
 *
 * Author: Annaliese McDermond <nh6z@nh6z.net>
 *
 * Based on sound/soc/codecs/wm8974 and TI driver for kernel 2.6.27.
 */

// C dependencies:
// #include <linux/i2c.h>
// #include <linux/module.h>
// #include <linux/of.h>
// #include <linux/regmap.h>
// #include <sound/soc.h>
// #include "tlv320aic32x4.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

pub type c_char = i8;
pub type c_int = i32;
pub type c_void = core::ffi::c_void;
pub type uintptr_t = usize;
pub type kernel_ulong_t = usize;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_range_cfg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub max_register: c_int,
    pub ranges: *const regmap_range_cfg,
    pub num_ranges: c_int,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const c_char,
    pub type_: *const c_char,
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

pub type aic32x4_type = c_int;

extern "C" {
    static aic32x4_regmap_pages: [regmap_range_cfg; 1];

    static AIC32X4_REFPOWERUP: c_int;
    static AIC32X4_TYPE_AIC32X4: aic32x4_type;
    static AIC32X4_TYPE_AIC32X6: aic32x4_type;
    static AIC32X4_TYPE_TAS2505: aic32x4_type;

    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
    fn aic32x4_probe(dev: *mut device, regmap: *mut regmap, type_: aic32x4_type) -> c_int;
    fn aic32x4_remove(dev: *mut device);
}

static aic32x4_i2c_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: unsafe { AIC32X4_REFPOWERUP },
    ranges: unsafe { aic32x4_regmap_pages.as_ptr() },
    num_ranges: 1,
};

unsafe extern "C" fn aic32x4_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let regmap: *mut regmap;
    let type_: aic32x4_type;

    regmap = devm_regmap_init_i2c(i2c, &aic32x4_i2c_regmap_config);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    type_ = i2c_get_match_data(i2c) as uintptr_t as aic32x4_type;

    return aic32x4_probe(&mut (*i2c).dev, regmap, type_);
}

unsafe extern "C" fn aic32x4_i2c_remove(i2c: *mut i2c_client) {
    aic32x4_remove(&mut (*i2c).dev);
}

const fn c_name_20(s: &[u8]) -> [c_char; 20] {
    let mut out = [0 as c_char; 20];
    let mut i = 0;
    while i < s.len() && i < 19 {
        out[i] = s[i] as c_char;
        i += 1;
    }
    out
}

static aic32x4_i2c_id: [i2c_device_id; 4] = [
    i2c_device_id {
        name: c_name_20(b"tlv320aic32x4"),
        driver_data: unsafe { AIC32X4_TYPE_AIC32X4 as kernel_ulong_t },
    },
    i2c_device_id {
        name: c_name_20(b"tlv320aic32x6"),
        driver_data: unsafe { AIC32X4_TYPE_AIC32X6 as kernel_ulong_t },
    },
    i2c_device_id {
        name: c_name_20(b"tas2505"),
        driver_data: unsafe { AIC32X4_TYPE_TAS2505 as kernel_ulong_t },
    },
    i2c_device_id {
        /* sentinel */
        name: [0; 20],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(i2c, aic32x4_i2c_id);

static aic32x4_of_id: [of_device_id; 4] = [
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: b"ti,tlv320aic32x4\0".as_ptr() as *const c_char,
        data: unsafe { AIC32X4_TYPE_AIC32X4 as uintptr_t as *const c_void },
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: b"ti,tlv320aic32x6\0".as_ptr() as *const c_char,
        data: unsafe { AIC32X4_TYPE_AIC32X6 as uintptr_t as *const c_void },
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: b"ti,tas2505\0".as_ptr() as *const c_char,
        data: unsafe { AIC32X4_TYPE_TAS2505 as uintptr_t as *const c_void },
    },
    of_device_id {
        /* senitel */
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, aic32x4_of_id);

static mut aic32x4_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"tlv320aic32x4\0".as_ptr() as *const c_char,
        of_match_table: aic32x4_of_id.as_ptr(),
    },
    probe: Some(aic32x4_i2c_probe),
    remove: Some(aic32x4_i2c_remove),
    id_table: aic32x4_i2c_id.as_ptr(),
};

// module_i2c_driver(aic32x4_i2c_driver);

// MODULE_DESCRIPTION("ASoC TLV320AIC32x4 codec driver I2C");
// MODULE_AUTHOR("Annaliese McDermond <nh6z@nh6z.net>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
