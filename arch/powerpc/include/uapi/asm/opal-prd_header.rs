/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * OPAL Runtime Diagnostics interface driver
 * Supported on POWERNV platform
 *
 * (C) Copyright IBM 2015
 *
 * Author: Vaidyanathan Srinivasan <svaidy at linux.vnet.ibm.com>
 * Author: Jeremy Kerr <jk@ozlabs.org>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

/**
 * The version of the kernel interface of the PRD system. This describes the
 * interface available for the /dev/opal-prd device. The actual PRD message
 * layout and content is private to the firmware <--> userspace interface, so
 * is not covered by this versioning.
 *
 * Future interface versions are backwards-compatible; if a later kernel
 * version is encountered, functionality provided in earlier versions
 * will work.
 */
pub const OPAL_PRD_KERNEL_VERSION: u32 = 1;

// Linux ioctl encoding corresponding to _IOR/_IOW in the source header.
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_DIRBITS: u32 = 2;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_READ: u32 = 2;
const IOC_WRITE: u32 = 1;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)
}

#[repr(C)]
pub struct opal_prd_info {
    pub version: u64,
    pub reserved: [u64; 3],
}

#[repr(C)]
pub struct opal_prd_scom {
    pub chip: u64,
    pub addr: u64,
    pub data: u64,
    pub rc: i64,
}

pub const OPAL_PRD_GET_INFO: u32 = ioc(
    IOC_READ,
    b'o' as u32,
    0x01,
    core::mem::size_of::<opal_prd_info>() as u32,
);
pub const OPAL_PRD_SCOM_READ: u32 = ioc(
    IOC_READ,
    b'o' as u32,
    0x02,
    core::mem::size_of::<opal_prd_scom>() as u32,
);
pub const OPAL_PRD_SCOM_WRITE: u32 = ioc(
    IOC_WRITE,
    b'o' as u32,
    0x03,
    core::mem::size_of::<opal_prd_scom>() as u32,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
