/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ADAU1372 driver
 *
 * Copyright 2016 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

/* Dependency intent from C header: <linux/regmap.h> supplies struct regmap,
 * struct regmap_config, and struct of_device_id. */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static adau1372_of_match: [of_device_id; 0];

    pub fn adau1372_probe(
        dev: *mut device,
        regmap: *mut regmap,
        switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>,
    ) -> core::ffi::c_int;

    pub static adau1372_regmap_config: regmap_config;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
