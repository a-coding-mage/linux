/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright 2019 Google Inc
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 *
 * Provides a simple driver to control the ASPEED P2A interface which allows
 * the host to read and write to various regions of the BMC's memory.
 */

/* Linux type and ioctl definitions represented by their Rust equivalents. */

pub const ASPEED_P2A_CTRL_READ_ONLY: u32 = 0;
pub const ASPEED_P2A_CTRL_READWRITE: u32 = 1;

/*
 * This driver provides a mechanism for enabling or disabling the read-write
 * property of specific windows into the ASPEED BMC's memory.
 *
 * A user can map a region of the BMC's memory as read-only or read-write, with
 * the caveat that once any region is mapped, all regions are unlocked for
 * reading.
 */

/*
 * Unlock a region of BMC physical memory for access from the host.
 *
 * Also used to read back the optional memory-region configuration for the
 * driver.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_p2a_ctrl_mapping {
    pub addr: u64,
    pub length: u32,
    pub flags: u32,
}

pub const __ASPEED_P2A_CTRL_IOCTL_MAGIC: u32 = 0xb3;

/* Linux _IOC/_IOW/_IOWR encoding used by the ioctl declarations below. */
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> usize {
    ((dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)) as usize
}

const fn iow<T>(ty: u32, nr: u32) -> usize {
    ioc(IOC_WRITE, ty, nr, core::mem::size_of::<T>() as u32)
}

const fn iowr<T>(ty: u32, nr: u32) -> usize {
    ioc(IOC_READ | IOC_WRITE, ty, nr, core::mem::size_of::<T>() as u32)
}

/*
 * This IOCTL is meant to configure a region or regions of memory given a
 * starting address and length to be readable by the host, or
 * readable-writeable.
 */
pub const ASPEED_P2A_CTRL_IOCTL_SET_WINDOW: usize =
    iow::<aspeed_p2a_ctrl_mapping>(__ASPEED_P2A_CTRL_IOCTL_MAGIC, 0x00);

/*
 * This IOCTL is meant to read back to the user the base address and length of
 * the memory-region specified to the driver for use with mmap.
 */
pub const ASPEED_P2A_CTRL_IOCTL_GET_MEMORY_CONFIG: usize =
    iowr::<aspeed_p2a_ctrl_mapping>(__ASPEED_P2A_CTRL_IOCTL_MAGIC, 0x01);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
