/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the Linux UAPI environment:
// linux/types.h, asm/ioctl.h, and asm/papr-miscdev.h.

pub type __u64 = u64;

/*
 * ioctl for /dev/papr-platform-dump. Returns a platform-dump handle fd
 * corresponding to dump tag.
 */
pub const PAPR_PLATFORM_DUMP_IOC_CREATE_HANDLE: u64 =
    _IOW(PAPR_MISCDEV_IOC_ID, 6, __u64);
pub const PAPR_PLATFORM_DUMP_IOC_INVALIDATE: u64 =
    _IOW(PAPR_MISCDEV_IOC_ID, 7, __u64);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
