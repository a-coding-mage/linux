/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from the original header: Linux ioctl and fixed-width types.

#[repr(C)]
pub struct nosy_stats {
    pub total_packet_count: u32,
    pub lost_packet_count: u32,
}

// Linux ioctl encoding, corresponding to _IO, _IOR, and _IOW.
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;

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

const fn io(ty: u8, nr: u8) -> u32 {
    ioc(IOC_NONE, ty as u32, nr as u32, 0)
}

const fn ior<T>(ty: u8, nr: u8) -> u32 {
    ioc(IOC_READ, ty as u32, nr as u32, core::mem::size_of::<T>() as u32)
}

const fn iow<T>(ty: u8, nr: u8) -> u32 {
    ioc(IOC_WRITE, ty as u32, nr as u32, core::mem::size_of::<T>() as u32)
}

pub const NOSY_IOC_GET_STATS: u32 = ior::<nosy_stats>(b'&', 0);
pub const NOSY_IOC_START: u32 = io(b'&', 1);
pub const NOSY_IOC_STOP: u32 = io(b'&', 2);
pub const NOSY_IOC_FILTER: u32 = iow::<u32>(b'&', 2);

/*
 * Format of packets returned from the kernel driver:
 *
 *     quadlet with timestamp        (microseconds, CPU endian)
 *     quadlet-padded packet data...  (little endian)
 *     quadlet with ack               (little endian)
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
