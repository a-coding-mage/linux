/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * header file for ADAV80X parts
 *
 * Copyright 2011 Analog Devices Inc.
 */

/* C dependency: <linux/regmap.h> */

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

unsafe extern "C" {
    pub static adav80x_regmap_config: regmap_config;
    pub fn adav80x_bus_probe(dev: *mut device, regmap: *mut regmap) -> ::std::os::raw::c_int;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum adav80x_pll_src {
    ADAV80X_PLL_SRC_XIN,
    ADAV80X_PLL_SRC_XTAL,
    ADAV80X_PLL_SRC_MCLKI,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum adav80x_pll {
    ADAV80X_PLL1 = 0,
    ADAV80X_PLL2 = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum adav80x_clk_src {
    ADAV80X_CLK_XIN = 0,
    ADAV80X_CLK_MCLKI = 1,
    ADAV80X_CLK_PLL1 = 2,
    ADAV80X_CLK_PLL2 = 3,
    ADAV80X_CLK_XTAL = 6,

    ADAV80X_CLK_SYSCLK1 = 6,
    ADAV80X_CLK_SYSCLK2 = 7,
    ADAV80X_CLK_SYSCLK3 = 8,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
