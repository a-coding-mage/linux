/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Translated from rio_mport_cdev.h. */

#[repr(C)]
pub struct rio_mport_maint_io {
    pub rioid: u16,
    pub hopcount: u8,
    pub pad0: [u8; 5],
    pub offset: u32,
    pub length: u32,
    pub buffer: u64,
}

pub const RIO_TRANSFER_MODE_MAPPED: u32 = 1 << 0;
pub const RIO_TRANSFER_MODE_TRANSFER: u32 = 1 << 1;
pub const RIO_CAP_DBL_SEND: u32 = 1 << 2;
pub const RIO_CAP_DBL_RECV: u32 = 1 << 3;
pub const RIO_CAP_PW_SEND: u32 = 1 << 4;
pub const RIO_CAP_PW_RECV: u32 = 1 << 5;
pub const RIO_CAP_MAP_OUTB: u32 = 1 << 6;
pub const RIO_CAP_MAP_INB: u32 = 1 << 7;

#[repr(C)]
pub struct rio_mport_properties {
    pub hdid: u16,
    pub id: u8,
    pub index: u8,
    pub flags: u32,
    pub sys_size: u32,
    pub port_ok: u8,
    pub link_speed: u8,
    pub link_width: u8,
    pub pad0: u8,
    pub dma_max_sge: u32,
    pub dma_max_size: u32,
    pub dma_align: u32,
    pub transfer_mode: u32,
    pub cap_sys_size: u32,
    pub cap_addr_size: u32,
    pub cap_transfer_mode: u32,
    pub cap_mport: u32,
}

pub const RIO_DOORBELL: u32 = 1 << 0;
pub const RIO_PORTWRITE: u32 = 1 << 1;

#[repr(C)]
pub struct rio_doorbell { pub rioid: u16, pub payload: u16 }

#[repr(C)]
pub struct rio_doorbell_filter {
    pub rioid: u16,
    pub low: u16,
    pub high: u16,
    pub pad0: u16,
}

#[repr(C)]
pub struct rio_portwrite { pub payload: [u32; 16] }

#[repr(C)]
pub struct rio_pw_filter { pub mask: u32, pub low: u32, pub high: u32, pub pad0: u32 }

pub const RIO_MAP_ANY_ADDR: u64 = !0u64;

#[repr(C)]
pub struct rio_mmap {
    pub rioid: u16,
    pub pad0: [u16; 3],
    pub rio_addr: u64,
    pub length: u64,
    pub handle: u64,
    pub address: u64,
}

#[repr(C)]
pub struct rio_dma_mem { pub length: u64, pub dma_handle: u64, pub address: u64 }

#[repr(C)]
pub union rio_event_u { pub doorbell: rio_doorbell, pub portwrite: rio_portwrite }

#[repr(C)]
pub struct rio_event { pub header: u32, pub u: rio_event_u, pub pad0: u32 }

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rio_transfer_sync { RIO_TRANSFER_SYNC, RIO_TRANSFER_ASYNC, RIO_TRANSFER_FAF }

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rio_transfer_dir { RIO_TRANSFER_DIR_READ, RIO_TRANSFER_DIR_WRITE }

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rio_exchange {
    RIO_EXCHANGE_DEFAULT,
    RIO_EXCHANGE_NWRITE,
    RIO_EXCHANGE_SWRITE,
    RIO_EXCHANGE_NWRITE_R,
    RIO_EXCHANGE_SWRITE_R,
    RIO_EXCHANGE_NWRITE_R_ALL,
}

#[repr(C)]
pub struct rio_transfer_io {
    pub rio_addr: u64,
    pub loc_addr: u64,
    pub handle: u64,
    pub offset: u64,
    pub length: u64,
    pub rioid: u16,
    pub method: u16,
    pub completion_code: u32,
}

#[repr(C)]
pub struct rio_transaction {
    pub block: u64,
    pub count: u32,
    pub transfer_mode: u32,
    pub sync: u16,
    pub dir: u16,
    pub pad0: u32,
}

#[repr(C)]
pub struct rio_async_tx_wait { pub token: u32, pub timeout: u32 }

pub const RIO_MAX_DEVNAME_SZ: usize = 20;

#[repr(C)]
pub struct rio_rdev_info {
    pub destid: u16,
    pub hopcount: u8,
    pub pad0: u8,
    pub comptag: u32,
    pub name: [core::ffi::c_char; RIO_MAX_DEVNAME_SZ + 1],
}

pub const RIO_MPORT_DRV_MAGIC: u8 = b'm';

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u8, nr: u32, size: usize) -> u32 {
    (dir << IOC_DIRSHIFT) | ((size as u32) << IOC_SIZESHIFT) |
        ((ty as u32) << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT)
}
const fn iow<T>(nr: u32) -> u32 { ioc(IOC_WRITE, RIO_MPORT_DRV_MAGIC, nr, core::mem::size_of::<T>()) }
const fn ior<T>(nr: u32) -> u32 { ioc(IOC_READ, RIO_MPORT_DRV_MAGIC, nr, core::mem::size_of::<T>()) }
const fn iowr<T>(nr: u32) -> u32 { ioc(IOC_READ | IOC_WRITE, RIO_MPORT_DRV_MAGIC, nr, core::mem::size_of::<T>()) }

pub const RIO_MPORT_MAINT_HDID_SET: u32 = iow::<u16>(1);
pub const RIO_MPORT_MAINT_COMPTAG_SET: u32 = iow::<u32>(2);
pub const RIO_MPORT_MAINT_PORT_IDX_GET: u32 = ior::<u32>(3);
pub const RIO_MPORT_GET_PROPERTIES: u32 = ior::<rio_mport_properties>(4);
pub const RIO_MPORT_MAINT_READ_LOCAL: u32 = ior::<rio_mport_maint_io>(5);
pub const RIO_MPORT_MAINT_WRITE_LOCAL: u32 = iow::<rio_mport_maint_io>(6);
pub const RIO_MPORT_MAINT_READ_REMOTE: u32 = ior::<rio_mport_maint_io>(7);
pub const RIO_MPORT_MAINT_WRITE_REMOTE: u32 = iow::<rio_mport_maint_io>(8);
pub const RIO_ENABLE_DOORBELL_RANGE: u32 = iow::<rio_doorbell_filter>(9);
pub const RIO_DISABLE_DOORBELL_RANGE: u32 = iow::<rio_doorbell_filter>(10);
pub const RIO_ENABLE_PORTWRITE_RANGE: u32 = iow::<rio_pw_filter>(11);
pub const RIO_DISABLE_PORTWRITE_RANGE: u32 = iow::<rio_pw_filter>(12);
pub const RIO_SET_EVENT_MASK: u32 = iow::<u32>(13);
pub const RIO_GET_EVENT_MASK: u32 = ior::<u32>(14);
pub const RIO_MAP_OUTBOUND: u32 = iowr::<rio_mmap>(15);
pub const RIO_UNMAP_OUTBOUND: u32 = iow::<rio_mmap>(16);
pub const RIO_MAP_INBOUND: u32 = iowr::<rio_mmap>(17);
pub const RIO_UNMAP_INBOUND: u32 = iow::<u64>(18);
pub const RIO_ALLOC_DMA: u32 = iowr::<rio_dma_mem>(19);
pub const RIO_FREE_DMA: u32 = iow::<u64>(20);
pub const RIO_TRANSFER: u32 = iowr::<rio_transaction>(21);
pub const RIO_WAIT_FOR_ASYNC: u32 = iow::<rio_async_tx_wait>(22);
pub const RIO_DEV_ADD: u32 = iow::<rio_rdev_info>(23);
pub const RIO_DEV_DEL: u32 = iow::<rio_rdev_info>(24);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
