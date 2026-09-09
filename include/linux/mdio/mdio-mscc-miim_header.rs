/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Driver for the MDIO interface of Microsemi network switches.
 *
 * Author: Colin Foster <colin.foster@in-advantage.com>
 * Copyright (C) 2021 Innovative Advantage
 */

use core::ffi::{c_char, c_int};

// External types supplied by Linux kernel dependencies.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mii_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mscc_miim_setup(
        device: *mut device,
        bus: *mut *mut mii_bus,
        name: *const c_char,
        mii_regmap: *mut regmap,
        status_offset: c_int,
        ignore_read_errors: bool,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
