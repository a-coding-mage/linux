/*
 * Allwinner SoCs SRAM Controller Driver
 *
 * Copyright (C) 2015 Maxime Ripard
 *
 * Author: Maxime Ripard <maxime.ripard@free-electrons.com>
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// The C header depends on the externally defined `struct device` type.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    pub fn sunxi_sram_claim(dev: *mut device) -> ::core::ffi::c_int;
    pub fn sunxi_sram_release(dev: *mut device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
