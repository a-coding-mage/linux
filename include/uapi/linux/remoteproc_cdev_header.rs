/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * IOCTLs for Remoteproc's character device interface.
 *
 * Copyright (c) 2020, The Linux Foundation. All rights reserved.
 */

// C dependencies: <linux/ioctl.h>, <linux/types.h>

pub const RPROC_MAGIC: u32 = 0xB7;

/*
 * The RPROC_SET_SHUTDOWN_ON_RELEASE ioctl allows to enable/disable the shutdown of a remote
 * processor automatically when the controlling userpsace closes the char device interface.
 *
 * input parameter: integer
 *   0          : disable automatic shutdown
 *   other      : enable automatic shutdown
 */
pub const RPROC_SET_SHUTDOWN_ON_RELEASE: u32 = 0x4004_B701;

/*
 * The RPROC_GET_SHUTDOWN_ON_RELEASE ioctl gets information about whether the automatic shutdown of
 * a remote processor is enabled or disabled when the controlling userspace closes the char device
 * interface.
 *
 * output parameter: integer
 *   0          : automatic shutdown disable
 *   other      : automatic shutdown enable
 */
pub const RPROC_GET_SHUTDOWN_ON_RELEASE: u32 = 0x8004_B702;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
