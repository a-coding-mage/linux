/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Helper module for board specific I2C bus registration
 *
 * Copyright (C) 2009 Nokia Corporation.
 */

// C header guard: __MACH_OMAP2_I2C_H

#[repr(C)]
pub struct omap_hwmod {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn omap_i2c_reset(oh: *mut omap_hwmod) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
