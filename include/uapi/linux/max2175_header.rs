/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * max2175.h
 *
 * Maxim Integrated MAX2175 RF to Bits tuner driver - user space header file.
 *
 * Copyright (C) 2016 Maxim Integrated Products
 * Copyright (C) 2017 Renesas Electronics Corporation
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 */

// Dependency supplied by the translated v4l2-controls definitions:
// V4L2_CID_USER_MAX217X_BASE

pub const V4L2_CID_MAX2175_I2S_ENABLE: u32 = V4L2_CID_USER_MAX217X_BASE + 0x01;
pub const V4L2_CID_MAX2175_HSLS: u32 = V4L2_CID_USER_MAX217X_BASE + 0x02;
pub const V4L2_CID_MAX2175_RX_MODE: u32 = V4L2_CID_USER_MAX217X_BASE + 0x03;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
