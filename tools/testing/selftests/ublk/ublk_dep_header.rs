// Rust translation of testing/selftests/ublk/ublk_dep.h.
// Header guards and C preprocessor conditionals are represented by ordinary
// Rust items; the original macros were fallbacks when these names were absent.

const _IOC_NRBITS: u32 = 8;
const _IOC_TYPEBITS: u32 = 8;
const _IOC_SIZEBITS: u32 = 14;

const _IOC_NRSHIFT: u32 = 0;
const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;

const _IOC_WRITE: u32 = 1;
const _IOC_READ: u32 = 2;

const fn _ioc(dir: u32, type_: u32, nr: u32, size: u32) -> u64 {
    ((dir as u64) << _IOC_DIRSHIFT)
        | ((type_ as u64) << _IOC_TYPESHIFT)
        | ((nr as u64) << _IOC_NRSHIFT)
        | ((size as u64) << _IOC_SIZESHIFT)
}

const fn _iowr<T>(type_: u32, nr: u32) -> u64 {
    _ioc(
        _IOC_READ | _IOC_WRITE,
        type_,
        nr,
        core::mem::size_of::<T>() as u32,
    )
}

pub const UBLK_U_IO_REGISTER_IO_BUF: u64 = _iowr::<ublksrv_io_cmd>(b'u' as u32, 0x23);
pub const UBLK_U_IO_UNREGISTER_IO_BUF: u64 = _iowr::<ublksrv_io_cmd>(b'u' as u32, 0x24);

pub const UBLK_F_USER_RECOVERY_FAIL_IO: u64 = 1_u64 << 9;

pub const UBLK_F_ZONED: u64 = 1_u64 << 8;
