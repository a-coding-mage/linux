/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * National Semiconductors LP3971 PMIC chip client interface
 *
 *  Copyright (C) 2009 Samsung Electronics
 *  Author: Marek Szyprowski <m.szyprowski@samsung.com>
 *
 * Based on wm8400.h
 */

// Dependency supplied by the Linux regulator machine interface.
pub struct regulator_init_data;

pub const LP3971_LDO1: core::ffi::c_int = 0;
pub const LP3971_LDO2: core::ffi::c_int = 1;
pub const LP3971_LDO3: core::ffi::c_int = 2;
pub const LP3971_LDO4: core::ffi::c_int = 3;
pub const LP3971_LDO5: core::ffi::c_int = 4;

pub const LP3971_DCDC1: core::ffi::c_int = 5;
pub const LP3971_DCDC2: core::ffi::c_int = 6;
pub const LP3971_DCDC3: core::ffi::c_int = 7;

pub const LP3971_NUM_REGULATORS: core::ffi::c_int = 8;

#[repr(C)]
pub struct lp3971_regulator_subdev {
    pub id: core::ffi::c_int,
    pub initdata: *mut regulator_init_data,
}

#[repr(C)]
pub struct lp3971_platform_data {
    pub num_regulators: core::ffi::c_int,
    pub regulators: *mut lp3971_regulator_subdev,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
