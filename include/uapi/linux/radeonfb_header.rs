/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the Linux UAPI header.  The ioctl encoding follows the
// asm/ioctl.h _IOC layout; size_t is represented by Rust's usize.

pub const ATY_RADEON_LCD_ON: u32 = 0x00000001;
pub const ATY_RADEON_CRT_ON: u32 = 0x00000002;

const IOC_NRBITS: usize = 8;
const IOC_TYPEBITS: usize = 8;
const IOC_SIZEBITS: usize = 14;
const IOC_DIRBITS: usize = 2;
const IOC_NRSHIFT: usize = 0;
const IOC_TYPESHIFT: usize = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: usize = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: usize = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: usize = 1;
const IOC_READ: usize = 2;

const fn ioc(dir: usize, ioctl_type: usize, nr: usize, size: usize) -> usize {
    (dir << IOC_DIRSHIFT)
        | (ioctl_type << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)
}

const fn ior<T>(ioctl_type: usize, nr: usize) -> usize {
    ioc(IOC_READ, ioctl_type, nr, core::mem::size_of::<T>())
}

const fn iow<T>(ioctl_type: usize, nr: usize) -> usize {
    ioc(IOC_WRITE, ioctl_type, nr, core::mem::size_of::<T>())
}

pub const FBIO_RADEON_GET_MIRROR: usize = ior::<usize>(b'@' as usize, 3);
pub const FBIO_RADEON_SET_MIRROR: usize = iow::<usize>(b'@' as usize, 4);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
