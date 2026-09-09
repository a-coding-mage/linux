/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header includes <linux/ioctl.h>.  The ioctl encoding helpers below
 * preserve the corresponding _IO, _IOW, and _IOR definitions locally.
 */

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_DIRBITS: u32 = 2;

const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;

const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)
}

const fn io(ty: u32, nr: u32) -> u32 {
    ioc(IOC_NONE, ty, nr, 0)
}

const fn iow<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_WRITE, ty, nr, core::mem::size_of::<T>() as u32)
}

const fn ior<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_READ, ty, nr, core::mem::size_of::<T>() as u32)
}

/* version */
pub const VERSION_STR: &str = "1.00";

/* Driver name */
pub const GIO_DRIVER_NAME: &str = "/dev/giodrv";

/* Use 'k' as magic number */
pub const GIODRV_IOC_MAGIC: u32 = b'k' as u32;

pub const GIODRV_IOCRESET: u32 = io(GIODRV_IOC_MAGIC, 0);

/*
 * S means "Set" through a ptr,
 * T means "Tell" directly
 * G means "Get" (to a pointed var)
 * Q means "Query", response is on the return value
 * X means "eXchange": G and S atomically
 * H means "sHift": T and Q atomically
 */
pub const GIODRV_IOCSGIODATA1: u32 = iow::<*mut u8>(GIODRV_IOC_MAGIC, 1);
pub const GIODRV_IOCGGIODATA1: u32 = ior::<*mut u8>(GIODRV_IOC_MAGIC, 2);
pub const GIODRV_IOCSGIODATA2: u32 = iow::<*mut u16>(GIODRV_IOC_MAGIC, 3);
pub const GIODRV_IOCGGIODATA2: u32 = ior::<*mut u16>(GIODRV_IOC_MAGIC, 4);
pub const GIODRV_IOCSGIODATA4: u32 = iow::<*mut core::ffi::c_ulong>(GIODRV_IOC_MAGIC, 5);
pub const GIODRV_IOCGGIODATA4: u32 = ior::<*mut core::ffi::c_ulong>(GIODRV_IOC_MAGIC, 6);
pub const GIODRV_IOCSGIOSETADDR: u32 = iow::<*mut core::ffi::c_ulong>(GIODRV_IOC_MAGIC, 7);
pub const GIODRV_IOCHARDRESET: u32 = io(GIODRV_IOC_MAGIC, 8); /* debugging tool */
pub const GIODRV_IOC_MAXNR: u32 = 8;

pub const GIO_READ: u32 = 0x00000000;
pub const GIO_WRITE: u32 = 0x00000001;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
