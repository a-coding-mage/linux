/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (c) 2015-2018, Intel Corporation.
 */

// Translated from <linux/ioctl.h>; `_IO(type, nr)` encodes an ioctl with no
// argument payload. The constants below preserve the Linux ioctl encoding.
pub const __IPMI_BMC_IOCTL_MAGIC: u32 = 0xB1;

pub const IPMI_BMC_IOCTL_SET_SMS_ATN: u32 =
    ((__IPMI_BMC_IOCTL_MAGIC << 8) | 0x00);
pub const IPMI_BMC_IOCTL_CLEAR_SMS_ATN: u32 =
    ((__IPMI_BMC_IOCTL_MAGIC << 8) | 0x01);
pub const IPMI_BMC_IOCTL_FORCE_ABORT: u32 =
    ((__IPMI_BMC_IOCTL_MAGIC << 8) | 0x02);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
