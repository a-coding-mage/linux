/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * basic functions for devices following the "stmp" style register layout
 *
 * Copyright (C) 2011 Wolfram Sang, Pengutronix e.K.
 */

// C header guard: __STMP_DEVICE_H__

pub const STMP_OFFSET_REG_SET: u32 = 0x4;
pub const STMP_OFFSET_REG_CLR: u32 = 0x8;
pub const STMP_OFFSET_REG_TOG: u32 = 0xc;

extern "C" {
    pub fn stmp_reset_block(regs: *mut core::ffi::c_void) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
