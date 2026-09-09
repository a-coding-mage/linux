/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * USB CDC Device Management userspace API definitions
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * version 2 as published by the Free Software Foundation.
 */

// C header guard: _UAPI__LINUX_USB_CDC_WDM_H
// C dependency: <linux/types.h>

/*
 * This IOCTL is used to retrieve the wMaxCommand for the device,
 * defining the message limit for both reading and writing.
 *
 * For CDC WDM functions this will be the wMaxCommand field of the
 * Device Management Functional Descriptor.
 */
// Equivalent to _IOR('H', 0xA0, __u16): IOC_IN | (sizeof(__u16) << 16) |
// ('H' << 8) | 0xA0.
pub const IOCTL_WDM_MAX_COMMAND: u32 = 0x4002_48A0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
