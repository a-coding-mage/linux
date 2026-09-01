// SPDX-License-Identifier: GPL-2.0
//
// Analog Devices ADAU7118 8 channel PDM-to-I2S/TDM Converter driver over I2C
//
// Copyright 2019 Analog Devices Inc.

// C dependencies translated as external Rust dependencies:
// linux/i2c.h, linux/module.h, linux/regmap.h, and "adau7118.h".

use core::ffi::{c_char, c_int, c_long, c_uint};

unsafe extern "C" {
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_long;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn adau7118_probe(dev: *mut device, map: *mut regmap, hw_mode: bool) -> c_int;
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
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
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
    pub id_table: *const i2c_device_id,
}

// Constants and register helpers are provided by the translated dependencies.

static adau7118_reg_defaults: [reg_default; 19] = [
    reg_default { reg: ADAU7118_REG_VENDOR_ID, def: 0x41 },
    reg_default { reg: ADAU7118_REG_DEVICE_ID1, def: 0x71 },
    reg_default { reg: ADAU7118_REG_DEVICE_ID2, def: 0x18 },
    reg_default { reg: ADAU7118_REG_REVISION_ID, def: 0x00 },
    reg_default { reg: ADAU7118_REG_ENABLES, def: 0x3F },
    reg_default { reg: ADAU7118_REG_DEC_RATIO_CLK_MAP, def: 0xC0 },
    reg_default { reg: ADAU7118_REG_HPF_CONTROL, def: 0xD0 },
    reg_default { reg: ADAU7118_REG_SPT_CTRL1, def: 0x41 },
    reg_default { reg: ADAU7118_REG_SPT_CTRL2, def: 0x00 },
    reg_default { reg: ADAU7118_REG_SPT_CX(0), def: 0x01 },
    reg_default { reg: ADAU7118_REG_SPT_CX(1), def: 0x11 },
    reg_default { reg: ADAU7118_REG_SPT_CX(2), def: 0x21 },
    reg_default { reg: ADAU7118_REG_SPT_CX(3), def: 0x31 },
    reg_default { reg: ADAU7118_REG_SPT_CX(4), def: 0x41 },
    reg_default { reg: ADAU7118_REG_SPT_CX(5), def: 0x51 },
    reg_default { reg: ADAU7118_REG_SPT_CX(6), def: 0x61 },
    reg_default { reg: ADAU7118_REG_SPT_CX(7), def: 0x71 },
    reg_default { reg: ADAU7118_REG_DRIVE_STRENGTH, def: 0x2a },
    reg_default { reg: ADAU7118_REG_RESET, def: 0x00 },
];

unsafe extern "C" fn adau7118_volatile(_dev: *mut device, reg: c_uint) -> bool {
    reg == ADAU7118_REG_RESET
}

static adau7118_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    reg_defaults: adau7118_reg_defaults.as_ptr(),
    num_reg_defaults: adau7118_reg_defaults.len() as c_uint,
    cache_type: REGCACHE_MAPLE,
    max_register: ADAU7118_REG_RESET,
    volatile_reg: Some(adau7118_volatile),
};

unsafe extern "C" fn adau7118_probe_i2c(i2c: *mut i2c_client) -> c_int {
    let map: *mut regmap;

    map = devm_regmap_init_i2c(i2c, &adau7118_regmap_config);
    if IS_ERR(map as *const core::ffi::c_void) {
        dev_err(
            &mut (*i2c).dev,
            c"Failed to init regmap %ld\n".as_ptr(),
            PTR_ERR(map as *const core::ffi::c_void),
        );
        return PTR_ERR(map as *const core::ffi::c_void) as c_int;
    }

    adau7118_probe(&mut (*i2c).dev, map, false)
}

static adau7118_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"adi,adau7118".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];
// MODULE_DEVICE_TABLE(of, adau7118_of_match);

static adau7118_id: [i2c_device_id; 2] = [
    i2c_device_id { name: c"adau7118".as_ptr() },
    i2c_device_id { name: core::ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, adau7118_id);

static mut adau7118_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"adau7118".as_ptr(),
        of_match_table: adau7118_of_match.as_ptr(),
    },
    probe: Some(adau7118_probe_i2c),
    id_table: adau7118_id.as_ptr(),
};
// module_i2c_driver(adau7118_driver);

// MODULE_AUTHOR("Nuno Sa <nuno.sa@analog.com>");
// MODULE_DESCRIPTION("ADAU7118 8 channel PDM-to-I2S/TDM Converter driver over I2C");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
