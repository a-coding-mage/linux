/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/uapi/linux/smiapp.h
 *
 * Generic driver for SMIA/SMIA++ compliant camera modules
 *
 * Copyright (C) 2014 Intel Corporation
 * Contact: Sakari Ailus <sakari.ailus@iki.fi>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 */

pub const V4L2_SMIAPP_TEST_PATTERN_MODE_DISABLED: i32 = 0;
pub const V4L2_SMIAPP_TEST_PATTERN_MODE_SOLID_COLOUR: i32 = 1;
pub const V4L2_SMIAPP_TEST_PATTERN_MODE_COLOUR_BARS: i32 = 2;
pub const V4L2_SMIAPP_TEST_PATTERN_MODE_COLOUR_BARS_GREY: i32 = 3;
pub const V4L2_SMIAPP_TEST_PATTERN_MODE_PN9: i32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
