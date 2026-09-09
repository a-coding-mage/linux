/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-xtensa/sockios.h
 *
 * Socket-level I/O control calls.  Copied from MIPS.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995 by Ralf Baechle
 * Copyright (C) 2001 Tensilica Inc.
 */

/* Values corresponding to the _IOC/_IOR/_IOW definitions from asm/ioctl.h. */
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_DIRBITS: u32 = 2;
const IOC_NRMASK: u32 = (1 << IOC_NRBITS) - 1;
const IOC_TYPEMASK: u32 = (1 << IOC_TYPEBITS) - 1;
const IOC_SIZEMASK: u32 = (1 << IOC_SIZEBITS) - 1;
const IOC_DIRMASK: u32 = (1 << IOC_DIRBITS) - 1;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, kind: u32, nr: u32, size: u32) -> u32 {
    ((dir & IOC_DIRMASK) << IOC_DIRSHIFT)
        | ((kind & IOC_TYPEMASK) << IOC_TYPESHIFT)
        | ((nr & IOC_NRMASK) << IOC_NRSHIFT)
        | ((size & IOC_SIZEMASK) << IOC_SIZESHIFT)
}

const fn ior(kind: u8, nr: u32, size: u32) -> u32 {
    ioc(IOC_READ, kind as u32, nr, size)
}

const fn iow(kind: u8, nr: u32, size: u32) -> u32 {
    ioc(IOC_WRITE, kind as u32, nr, size)
}

/* Socket-level I/O control calls. */
pub const FIOGETOWN: u32 = ior(b'f', 123, core::mem::size_of::<i32>() as u32);
pub const FIOSETOWN: u32 = iow(b'f', 124, core::mem::size_of::<i32>() as u32);

pub const SIOCATMARK: u32 = ior(b's', 7, core::mem::size_of::<i32>() as u32);
pub const SIOCSPGRP: u32 = iow(b's', 8, core::mem::size_of::<i32>() as u32);
pub const SIOCGPGRP: u32 = ior(b's', 9, core::mem::size_of::<i32>() as u32);

pub const SIOCGSTAMP_OLD: u32 = 0x8906; // Get stamp (timeval)
pub const SIOCGSTAMPNS_OLD: u32 = 0x8907; // Get stamp (timespec)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
