/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * sky81452.h	SKY81452 MFD driver
 *
 * Copyright 2014 Skyworks Solutions Inc.
 * Author : Gyungoh Yoo <jack.yoo@skyworksinc.com>
 */

/* Dependency: linux/regulator/machine.h */

#[repr(C)]
pub struct sky81452_platform_data {
    pub regulator_init_data: *mut regulator_init_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
