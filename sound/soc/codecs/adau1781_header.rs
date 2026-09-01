// SPDX-License-Identifier: GPL-2.0-only
/*
 * ADAU1381/ADAU1781 driver
 *
 * Copyright 2014 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// Depends on Linux regmap definitions and adau17x1 type definitions.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn adau1781_probe(
        dev: *mut device,
        regmap: *mut regmap,
        type_: adau17x1_type,
        switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>,
    ) -> ::core::ffi::c_int;

    pub static adau1781_regmap_config: regmap_config;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
