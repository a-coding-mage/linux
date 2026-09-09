/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Our DIR and SIZE overlap in order to simulteneously provide
 * a non-zero _IOC_NONE (for binary compatibility) and
 * 14 bits of size as on i386. Here's the layout:
 *
 *   0xE0000000   DIR
 *   0x80000000     DIR = WRITE
 *   0x40000000     DIR = READ
 *   0x20000000     DIR = NONE
 *   0x3FFF0000   SIZE (overlaps NONE bit)
 *   0x0000FF00   TYPE
 *   0x000000FF   NR (CMD)
 */

pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;
pub const _IOC_SIZEBITS: u32 = 13; /* Actually 14, see below. */
pub const _IOC_DIRBITS: u32 = 3;

pub const _IOC_NRMASK: u32 = (1u32 << _IOC_NRBITS) - 1;
pub const _IOC_TYPEMASK: u32 = (1u32 << _IOC_TYPEBITS) - 1;
pub const _IOC_SIZEMASK: u32 = (1u32 << _IOC_SIZEBITS) - 1;
pub const _IOC_XSIZEMASK: u32 = (1u32 << (_IOC_SIZEBITS + 1)) - 1;
pub const _IOC_DIRMASK: u32 = (1u32 << _IOC_DIRBITS) - 1;

pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
pub const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;

pub const _IOC_NONE: u32 = 1;
pub const _IOC_READ: u32 = 2;
pub const _IOC_WRITE: u32 = 4;

#[macro_export]
macro_rules! _IOC {
    ($dir:expr, $type_:expr, $nr:expr, $size:expr) => {
        (($dir as u32) << $crate::_IOC_DIRSHIFT)
            | (($type_ as u32) << $crate::_IOC_TYPESHIFT)
            | (($nr as u32) << $crate::_IOC_NRSHIFT)
            | (($size as u32) << $crate::_IOC_SIZESHIFT)
    };
}

#[macro_export]
macro_rules! _IO {
    ($type_:expr, $nr:expr) => { $crate::_IOC!($crate::_IOC_NONE, $type_, $nr, 0) };
}

#[macro_export]
macro_rules! _IOR {
    ($type_:expr, $nr:expr, $size:ty) => {
        $crate::_IOC!($crate::_IOC_READ, $type_, $nr, core::mem::size_of::<$size>())
    };
}

#[macro_export]
macro_rules! _IOW {
    ($type_:expr, $nr:expr, $size:ty) => {
        $crate::_IOC!($crate::_IOC_WRITE, $type_, $nr, core::mem::size_of::<$size>())
    };
}

#[macro_export]
macro_rules! _IOWR {
    ($type_:expr, $nr:expr, $size:ty) => {
        $crate::_IOC!($crate::_IOC_READ | $crate::_IOC_WRITE, $type_, $nr,
                      core::mem::size_of::<$size>())
    };
}

/* Used to decode ioctl numbers in drivers despite the leading underscore... */
#[macro_export]
macro_rules! _IOC_DIR {
    ($nr:expr) => {{
        let nr = $nr as u32;
        let dir = (nr >> $crate::_IOC_DIRSHIFT) & $crate::_IOC_DIRMASK;
        if (dir & ($crate::_IOC_WRITE | $crate::_IOC_READ)) != 0 {
            dir & ($crate::_IOC_WRITE | $crate::_IOC_READ)
        } else {
            dir
        }
    }};
}

#[macro_export]
macro_rules! _IOC_TYPE {
    ($nr:expr) => { (($nr as u32 >> $crate::_IOC_TYPESHIFT) & $crate::_IOC_TYPEMASK) };
}

#[macro_export]
macro_rules! _IOC_NR {
    ($nr:expr) => { (($nr as u32 >> $crate::_IOC_NRSHIFT) & $crate::_IOC_NRMASK) };
}

#[macro_export]
macro_rules! _IOC_SIZE {
    ($nr:expr) => {{
        let nr = $nr as u32;
        let dir = (nr >> $crate::_IOC_DIRSHIFT) & $crate::_IOC_DIRMASK;
        if (dir & ($crate::_IOC_WRITE | $crate::_IOC_READ)) == 0 {
            0
        } else {
            (nr >> $crate::_IOC_SIZESHIFT) & $crate::_IOC_XSIZEMASK
        }
    }};
}

/* ...and for the PCMCIA and sound. */
pub const IOC_IN: u32 = _IOC_WRITE << _IOC_DIRSHIFT;
pub const IOC_OUT: u32 = _IOC_READ << _IOC_DIRSHIFT;
pub const IOC_INOUT: u32 = (_IOC_WRITE | _IOC_READ) << _IOC_DIRSHIFT;
pub const IOCSIZE_MASK: u32 = _IOC_XSIZEMASK << _IOC_SIZESHIFT;
pub const IOCSIZE_SHIFT: u32 = _IOC_SIZESHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
