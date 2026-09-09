/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * VTI CMA3000_Dxx Accelerometer driver
 *
 * Copyright (C) 2010 Texas Instruments
 * Author: Hemanth V <hemanthv@ti.com>
 */

pub const CMAMODE_DEFAULT: i32 = 0;
pub const CMAMODE_MEAS100: i32 = 1;
pub const CMAMODE_MEAS400: i32 = 2;
pub const CMAMODE_MEAS40: i32 = 3;
pub const CMAMODE_MOTDET: i32 = 4;
pub const CMAMODE_FF100: i32 = 5;
pub const CMAMODE_FF400: i32 = 6;
pub const CMAMODE_POFF: i32 = 7;

pub const CMARANGE_2G: i32 = 2000;
pub const CMARANGE_8G: i32 = 8000;

/**
 * struct cma3000_i2c_platform_data - CMA3000 Platform data
 * @fuzz_x: Noise on X Axis
 * @fuzz_y: Noise on Y Axis
 * @fuzz_z: Noise on Z Axis
 * @g_range: G range in milli g i.e 2000 or 8000
 * @mode: Operating mode
 * @mdthr: Motion detect threshold value
 * @mdfftmr: Motion detect and free fall time value
 * @ffthr: Free fall threshold value
 */
#[repr(C)]
pub struct cma3000_platform_data {
    pub fuzz_x: i32,
    pub fuzz_y: i32,
    pub fuzz_z: i32,
    pub g_range: i32,
    pub mode: u8,
    pub mdthr: u8,
    pub mdfftmr: u8,
    pub ffthr: u8,
    pub irqflags: core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
