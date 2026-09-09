/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * udf_fs_i.h
 *
 * This file is intended for the Linux kernel/module.
 *
 * COPYRIGHT
 *	This file is distributed under the terms of the GNU General Public
 *	License (GPL). Copies of the GPL can be obtained from:
 *		ftp://prep.ai.mit.edu/pub/gnu/GPL
 *	Each contributing author retains all rights to their own work.
 */

/* The Linux _IOR/_IOWR ioctl encodings used by the declarations below. */
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_READ: u32 = 2;
const IOC_WRITE: u32 = 1;

const fn ioc(dir: u32, ty: u32, nr: u32, size: usize) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | ((size as u32) << IOC_SIZESHIFT)
}

const fn ior<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_READ, ty, nr, core::mem::size_of::<T>())
}

const fn iowr<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_READ | IOC_WRITE, ty, nr, core::mem::size_of::<T>())
}

/* exported IOCTLs, we have 'l', 0x40-0x7f */
pub const UDF_GETEASIZE: u32 = ior::<i32>(b'l' as u32, 0x40);
pub const UDF_GETEABLOCK: u32 = ior::<*const core::ffi::c_void>(b'l' as u32, 0x41);
pub const UDF_GETVOLIDENT: u32 = ior::<*const core::ffi::c_void>(b'l' as u32, 0x42);
pub const UDF_RELOCATE_BLOCKS: u32 = iowr::<isize>(b'l' as u32, 0x43);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
