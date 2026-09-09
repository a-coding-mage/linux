/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* ioctl command encoding: 32 bits total, command in lower 16 bits,
 * size of the parameter structure in the lower 14 bits of the
 * upper 16 bits.
 * Encoding the size of the parameter structure in the ioctl request
 * is useful for catching programs compiled with old versions
 * and to avoid overwriting user space outside the user buffer area.
 * The highest 2 bits are reserved for indicating the ``access mode''.
 * NOTE: This limits the max parameter size to 16kB -1 !
 */

/*
 * The following is for compatibility across the various Linux
 * platforms.  The generic ioctl numbering scheme doesn't really enforce
 * a type field.  De facto, however, the top 8 bits of the lower 16
 * bits are indeed used as a type field, so we might just as well make
 * this explicit here.  Please be sure to use the decoding macros
 * below from now on.
 */
pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;

/* Architectures may override _IOC_SIZEBITS and _IOC_DIRBITS before inclusion. */
pub const _IOC_SIZEBITS: u32 = 14;
pub const _IOC_DIRBITS: u32 = 2;

pub const _IOC_NRMASK: u32 = (1u32 << _IOC_NRBITS) - 1;
pub const _IOC_TYPEMASK: u32 = (1u32 << _IOC_TYPEBITS) - 1;
pub const _IOC_SIZEMASK: u32 = (1u32 << _IOC_SIZEBITS) - 1;
pub const _IOC_DIRMASK: u32 = (1u32 << _IOC_DIRBITS) - 1;

pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
pub const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;

/* Direction bits, which any architecture can choose to override before inclusion. */
pub const _IOC_NONE: u32 = 0u32;
pub const _IOC_WRITE: u32 = 1u32;
pub const _IOC_READ: u32 = 2u32;

macro_rules! _IOC {
    ($dir:expr, $type_:expr, $nr:expr, $size:expr) => {
        (($dir << _IOC_DIRSHIFT)
            | ($type_ << _IOC_TYPESHIFT)
            | ($nr << _IOC_NRSHIFT)
            | ($size << _IOC_SIZESHIFT))
    };
}

macro_rules! _IOC_TYPECHECK {
    ($t:ty) => {
        core::mem::size_of::<$t>()
    };
}

/* Used to create numbers. */
macro_rules! _IO {
    ($type_:expr, $nr:expr) => { _IOC!(_IOC_NONE, $type_, $nr, 0) };
}
macro_rules! _IOR {
    ($type_:expr, $nr:expr, $argtype:ty) => {
        _IOC!(_IOC_READ, $type_, $nr, _IOC_TYPECHECK!($argtype))
    };
}
macro_rules! _IOW {
    ($type_:expr, $nr:expr, $argtype:ty) => {
        _IOC!(_IOC_WRITE, $type_, $nr, _IOC_TYPECHECK!($argtype))
    };
}
macro_rules! _IOWR {
    ($type_:expr, $nr:expr, $argtype:ty) => {
        _IOC!(_IOC_READ | _IOC_WRITE, $type_, $nr, _IOC_TYPECHECK!($argtype))
    };
}
macro_rules! _IOR_BAD {
    ($type_:expr, $nr:expr, $argtype:ty) => { _IOC!(_IOC_READ, $type_, $nr, core::mem::size_of::<$argtype>()) };
}
macro_rules! _IOW_BAD {
    ($type_:expr, $nr:expr, $argtype:ty) => { _IOC!(_IOC_WRITE, $type_, $nr, core::mem::size_of::<$argtype>()) };
}
macro_rules! _IOWR_BAD {
    ($type_:expr, $nr:expr, $argtype:ty) => { _IOC!(_IOC_READ | _IOC_WRITE, $type_, $nr, core::mem::size_of::<$argtype>()) };
}

/* used to decode ioctl numbers.. */
macro_rules! _IOC_DIR { ($nr:expr) => { (($nr >> _IOC_DIRSHIFT) & _IOC_DIRMASK) }; }
macro_rules! _IOC_TYPE { ($nr:expr) => { (($nr >> _IOC_TYPESHIFT) & _IOC_TYPEMASK) }; }
macro_rules! _IOC_NR { ($nr:expr) => { (($nr >> _IOC_NRSHIFT) & _IOC_NRMASK) }; }
macro_rules! _IOC_SIZE { ($nr:expr) => { (($nr >> _IOC_SIZESHIFT) & _IOC_SIZEMASK) }; }

/* ...and for the drivers/sound files... */
pub const IOC_IN: u32 = _IOC_WRITE << _IOC_DIRSHIFT;
pub const IOC_OUT: u32 = _IOC_READ << _IOC_DIRSHIFT;
pub const IOC_INOUT: u32 = (_IOC_WRITE | _IOC_READ) << _IOC_DIRSHIFT;
pub const IOCSIZE_MASK: u32 = _IOC_SIZEMASK << _IOC_SIZESHIFT;
pub const IOCSIZE_SHIFT: u32 = _IOC_SIZESHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
