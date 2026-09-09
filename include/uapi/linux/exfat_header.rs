/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2024 Unisoc Technologies Co., Ltd.
 */

// Dependencies supplied by the Linux UAPI environment:
// - `linux/types.h` supplies the C `__u32` type, represented here by `u32`.
// - `linux/ioctl.h` supplies the `_IOR` ioctl encoding macro.

/*
 * exfat-specific ioctl commands
 */

pub const EXFAT_IOC_SHUTDOWN: u32 = _IOR('X', 125, u32);

/*
 * Flags used by EXFAT_IOC_SHUTDOWN
 */

pub const EXFAT_GOING_DOWN_DEFAULT: u32 = 0x0; // default with full sync
pub const EXFAT_GOING_DOWN_FULLSYNC: u32 = 0x1; // going down with full sync
pub const EXFAT_GOING_DOWN_NOSYNC: u32 = 0x2; // going down

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
