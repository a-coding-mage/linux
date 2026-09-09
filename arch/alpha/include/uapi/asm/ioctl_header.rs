/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * The original linux ioctl numbering scheme was just a general
 * "anything goes" setup, where more or less random numbers were
 * assigned.  Sorry, I was clueless when I started out on this.
 *
 * On the alpha, we'll try to clean it up a bit, using a more sane
 * ioctl numbering, and also trying to be compatible with OSF/1 in the
 * process. I'd like to clean it up for the i386 as well, but
 * it's so painful recognizing both the new and the old numbers..
 */

pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;
pub const _IOC_SIZEBITS: u32 = 13;
pub const _IOC_DIRBITS: u32 = 3;

pub const _IOC_NRMASK: u32 = ((1u32 << _IOC_NRBITS) - 1);
pub const _IOC_TYPEMASK: u32 = ((1u32 << _IOC_TYPEBITS) - 1);
pub const _IOC_SIZEMASK: u32 = ((1u32 << _IOC_SIZEBITS) - 1);
pub const _IOC_DIRMASK: u32 = ((1u32 << _IOC_DIRBITS) - 1);

pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
pub const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;

/*
 * Direction bits _IOC_NONE could be 0, but OSF/1 gives it a bit.
 * And this turns out useful to catch old ioctl numbers in header
 * files for us.
 */
pub const _IOC_NONE: u32 = 1u32;
pub const _IOC_READ: u32 = 2u32;
pub const _IOC_WRITE: u32 = 4u32;

#[inline]
pub const fn _IOC(dir: u32, type_: u32, nr: u32, size: u32) -> u32 {
    ((dir << _IOC_DIRSHIFT)
        | (type_ << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | (size << _IOC_SIZESHIFT))
}

/* used to create numbers */
#[macro_export]
macro_rules! _IO {
    ($type:expr, $nr:expr) => {
        $crate::_IOC($crate::_IOC_NONE, $type, $nr, 0)
    };
}

#[macro_export]
macro_rules! _IOR {
    ($type:expr, $nr:expr, $size:ty) => {
        $crate::_IOC($crate::_IOC_READ, $type, $nr, core::mem::size_of::<$size>() as u32)
    };
}

#[macro_export]
macro_rules! _IOW {
    ($type:expr, $nr:expr, $size:ty) => {
        $crate::_IOC($crate::_IOC_WRITE, $type, $nr, core::mem::size_of::<$size>() as u32)
    };
}

#[macro_export]
macro_rules! _IOWR {
    ($type:expr, $nr:expr, $size:ty) => {
        $crate::_IOC($crate::_IOC_READ | $crate::_IOC_WRITE, $type, $nr, core::mem::size_of::<$size>() as u32)
    };
}

/* used to decode them.. */
#[inline]
pub const fn _IOC_DIR(nr: u32) -> u32 {
    (nr >> _IOC_DIRSHIFT) & _IOC_DIRMASK
}

#[inline]
pub const fn _IOC_TYPE(nr: u32) -> u32 {
    (nr >> _IOC_TYPESHIFT) & _IOC_TYPEMASK
}

#[inline]
pub const fn _IOC_NR(nr: u32) -> u32 {
    (nr >> _IOC_NRSHIFT) & _IOC_NRMASK
}

#[inline]
pub const fn _IOC_SIZE(nr: u32) -> u32 {
    (nr >> _IOC_SIZESHIFT) & _IOC_SIZEMASK
}

/* ...and for the drivers/sound files... */

pub const IOC_IN: u32 = _IOC_WRITE << _IOC_DIRSHIFT;
pub const IOC_OUT: u32 = _IOC_READ << _IOC_DIRSHIFT;
pub const IOC_INOUT: u32 = (_IOC_WRITE | _IOC_READ) << _IOC_DIRSHIFT;
pub const IOCSIZE_MASK: u32 = _IOC_SIZEMASK << _IOC_SIZESHIFT;
pub const IOCSIZE_SHIFT: u32 = _IOC_SIZESHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
