/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (c) 2015-2016, IBM Corporation.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

// Dependency equivalent of <linux/ioctl.h>.

pub const __BT_BMC_IOCTL_MAGIC: u8 = 0xb1;

// _IO(type, nr): no data transfer, with the Linux ioctl encoding.
pub const BT_BMC_IOCTL_SMS_ATN: usize =
    ((__BT_BMC_IOCTL_MAGIC as usize) << 8) | 0x00;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
