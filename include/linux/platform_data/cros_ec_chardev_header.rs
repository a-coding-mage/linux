/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ChromeOS EC device interface.
 *
 * Copyright (C) 2014 Google, Inc.
 */

// C header guard: _UAPI_LINUX_CROS_EC_DEV_H_
// C dependencies: <linux/bits.h>, <linux/ioctl.h>, <linux/types.h>,
// and <linux/platform_data/cros_ec_commands.h>.

pub const CROS_EC_DEV_VERSION: &str = "1.0.0";

/**
 * struct cros_ec_readmem - Struct used to read mapped memory.
 * @offset: Within EC_LPC_ADDR_MEMMAP region.
 * @bytes: Number of bytes to read. Zero means "read a string" (including '\0')
 *         At most only EC_MEMMAP_SIZE bytes can be read.
 * @buffer: Where to store the result. The ioctl returns the number of bytes
 *         read or negative on error.
 */
#[repr(C)]
pub struct cros_ec_readmem {
    pub offset: u32,
    pub bytes: u32,
    pub buffer: [u8; EC_MEMMAP_SIZE],
}

pub const CROS_EC_DEV_IOC: u32 = 0xEC;
pub const CROS_EC_DEV_IOCXCMD: usize = _IOWR(CROS_EC_DEV_IOC, 0, cros_ec_command);
pub const CROS_EC_DEV_IOCRDMEM: usize = _IOWR(CROS_EC_DEV_IOC, 1, cros_ec_readmem);
pub const CROS_EC_DEV_IOCEVENTMASK: usize = _IO(CROS_EC_DEV_IOC, 2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
