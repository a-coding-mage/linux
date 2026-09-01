// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2011-2019 NW Digital Radio
 *
 * Author: Annaliese McDermond <nh6z@nh6z.net>
 *
 * Based on sound/soc/codecs/wm8974 and TI driver for kernel 2.6.27.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

type kernel_ulong_t = usize;
type uintptr_t = usize;

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
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub pad_bits: c_int,
    pub val_bits: c_int,
    pub read_flag_mask: c_int,
    pub max_register: c_int,
    pub ranges: *const regmap_range_cfg,
    pub num_ranges: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_device_id {
    pub name: [c_char; 32],
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
pub struct spi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut spi_device)>,
    pub id_table: *const spi_device_id,
}

type aic32x4_type = c_int;

extern "C" {
    static aic32x4_regmap_pages: [regmap_range_cfg; 1];

    static AIC32X4_REFPOWERUP: c_int;
    static AIC32X4_TYPE_AIC32X4: aic32x4_type;
    static AIC32X4_TYPE_AIC32X6: aic32x4_type;

    fn devm_regmap_init_spi(
        spi: *mut spi_device,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn spi_get_device_match_data(spi: *mut spi_device) -> *const c_void;
    fn aic32x4_probe(dev: *mut device, regmap: *mut regmap, type_: aic32x4_type) -> c_int;
    fn aic32x4_remove(dev: *mut device);
}

static aic32x4_spi_regmap_config: regmap_config = regmap_config {
    reg_bits: 7,
    pad_bits: 1,
    val_bits: 8,
    read_flag_mask: 0x01,
    max_register: unsafe { AIC32X4_REFPOWERUP },
    ranges: unsafe { aic32x4_regmap_pages.as_ptr() },
    num_ranges: 1,
};

unsafe extern "C" fn aic32x4_spi_probe(spi: *mut spi_device) -> c_int {
    let regmap: *mut regmap;
    let type_: aic32x4_type;

    regmap = devm_regmap_init_spi(spi, &aic32x4_spi_regmap_config);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    type_ = spi_get_device_match_data(spi) as uintptr_t as aic32x4_type;

    return aic32x4_probe(&mut (*spi).dev, regmap, type_);
}

unsafe extern "C" fn aic32x4_spi_remove(spi: *mut spi_device) {
    aic32x4_remove(&mut (*spi).dev);
}

const fn spi_name(name: &[u8]) -> [c_char; 32] {
    let mut out = [0 as c_char; 32];
    let mut i = 0;
    while i < name.len() && i < 31 {
        out[i] = name[i] as c_char;
        i += 1;
    }
    out
}

static aic32x4_spi_id: [spi_device_id; 3] = [
    spi_device_id {
        name: spi_name(b"tlv320aic32x4"),
        driver_data: unsafe { AIC32X4_TYPE_AIC32X4 as kernel_ulong_t },
    },
    spi_device_id {
        name: spi_name(b"tlv320aic32x6"),
        driver_data: unsafe { AIC32X4_TYPE_AIC32X6 as kernel_ulong_t },
    },
    spi_device_id {
        name: [0; 32],
        driver_data: 0,
    }, /* sentinel */
];
// MODULE_DEVICE_TABLE(spi, aic32x4_spi_id);

static aic32x4_of_id: [of_device_id; 3] = [
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
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    }, /* senitel */
];
// MODULE_DEVICE_TABLE(of, aic32x4_of_id);

static mut aic32x4_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: b"tlv320aic32x4\0".as_ptr() as *const c_char,
        of_match_table: aic32x4_of_id.as_ptr(),
    },
    probe: Some(aic32x4_spi_probe),
    remove: Some(aic32x4_spi_remove),
    id_table: aic32x4_spi_id.as_ptr(),
};

// module_spi_driver(aic32x4_spi_driver);

// MODULE_DESCRIPTION("ASoC TLV320AIC32x4 codec driver SPI");
// MODULE_AUTHOR("Annaliese McDermond <nh6z@nh6z.net>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
