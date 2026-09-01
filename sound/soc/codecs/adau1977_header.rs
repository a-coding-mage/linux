/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ADAU1977/ADAU1978/ADAU1979 driver
 *
 * Copyright 2014 Analog Devices Inc.
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// C header dependency: <linux/regmap.h>

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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum adau1977_type {
    ADAU1977,
    ADAU1978,
    ADAU1979,
}

unsafe extern "C" {
    pub fn adau1977_probe(
        dev: *mut device,
        regmap: *mut regmap,
        type_: adau1977_type,
        switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>,
    ) -> ::std::os::raw::c_int;

    pub static adau1977_regmap_config: regmap_config;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum adau1977_clk_id {
    ADAU1977_SYSCLK,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum adau1977_sysclk_src {
    ADAU1977_SYSCLK_SRC_MCLK,
    ADAU1977_SYSCLK_SRC_LRCLK,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
