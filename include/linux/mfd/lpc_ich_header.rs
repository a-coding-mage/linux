/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/drivers/mfd/lpc_ich.h
 *
 *  Copyright (c) 2012 Extreme Engineering Solution, Inc.
 *  Author: Aaron Sierra <asierra@xes-inc.com>
 */

// Dependency supplied by the Linux platform-data SPI definitions.

/* GPIO resources */
pub const ICH_RES_GPIO: ::core::ffi::c_int = 0;
pub const ICH_RES_GPE0: ::core::ffi::c_int = 1;

/* GPIO compatibility */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum lpc_gpio_versions {
    ICH_I3100_GPIO,
    ICH_V5_GPIO,
    ICH_V6_GPIO,
    ICH_V7_GPIO,
    ICH_V9_GPIO,
    ICH_V10CORP_GPIO,
    ICH_V10CONS_GPIO,
    AVOTON_GPIO,
}

pub struct lpc_ich_gpio_info;

#[repr(C)]
pub struct lpc_ich_info {
    pub name: [::core::ffi::c_char; 32],
    pub iTCO_version: ::core::ffi::c_uint,
    pub gpio_version: lpc_gpio_versions,
    pub spi_type: intel_spi_type,
    pub gpio_info: *const lpc_ich_gpio_info,
    pub use_gpio: u8,
}

extern "C" {
    pub static lpc_ich_gpio_swnode: software_node;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
