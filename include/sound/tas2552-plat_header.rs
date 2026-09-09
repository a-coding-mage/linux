/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TAS2552 driver platform header
 *
 * Copyright (C) 2014 Texas Instruments Inc.
 *
 * Author: Dan Murphy <dmurphy@ti.com>
 */

#[repr(C)]
pub struct tas2552_platform_data {
    pub enable_gpio: ::core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
