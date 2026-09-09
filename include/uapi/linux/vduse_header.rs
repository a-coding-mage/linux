/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */

/* Linux UAPI integer types are represented by their fixed-width Rust equivalents. */

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

const fn ioc(dir: u32, ty: u32, nr: u32, size: usize) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | ((size as u32) << IOC_SIZESHIFT)
}
const fn io(ty: u32, nr: u32) -> u32 { ioc(IOC_NONE, ty, nr, 0) }
const fn ior<T>(ty: u32, nr: u32) -> u32 { ioc(IOC_READ, ty, nr, core::mem::size_of::<T>()) }
const fn iow<T>(ty: u32, nr: u32) -> u32 { ioc(IOC_WRITE, ty, nr, core::mem::size_of::<T>()) }
const fn iowr<T>(ty: u32, nr: u32) -> u32 { ioc(IOC_READ | IOC_WRITE, ty, nr, core::mem::size_of::<T>()) }

pub const VDUSE_BASE: u32 = 0x81;
pub const VDUSE_API_VERSION: u32 = 0;
pub const VDUSE_API_VERSION_1: u32 = 1;
pub const VDUSE_F_QUEUE_READY: u32 = 0;
pub const VDUSE_F_SUSPEND: u32 = 1;

pub const VDUSE_NAME_MAX: usize = 256;

#[repr(C)]
pub struct vduse_dev_config {
    pub name: [u8; VDUSE_NAME_MAX],
    pub vendor_id: u32,
    pub device_id: u32,
    pub features: u64,
    pub vq_num: u32,
    pub vq_align: u32,
    pub ngroups: u32,
    pub nas: u32,
    pub reserved: [u32; 11],
    pub config_size: u32,
    pub config: [u8; 0],
}

pub const VDUSE_GET_API_VERSION: u32 = ior::<u64>(VDUSE_BASE, 0x00);
pub const VDUSE_SET_API_VERSION: u32 = iow::<u64>(VDUSE_BASE, 0x01);
pub const VDUSE_CREATE_DEV: u32 = iow::<vduse_dev_config>(VDUSE_BASE, 0x02);
pub const VDUSE_DESTROY_DEV: u32 = iow::<[u8; VDUSE_NAME_MAX]>(VDUSE_BASE, 0x03);
pub const VDUSE_GET_FEATURES: u32 = ior::<u64>(VDUSE_BASE, 0x04);
pub const VDUSE_SET_FEATURES: u32 = iow::<u64>(VDUSE_BASE, 0x05);

#[repr(C)]
pub struct vduse_iotlb_entry { pub offset: u64, pub start: u64, pub last: u64, pub perm: u8 }
pub const VDUSE_ACCESS_RO: u8 = 0x1;
pub const VDUSE_ACCESS_WO: u8 = 0x2;
pub const VDUSE_ACCESS_RW: u8 = 0x3;
pub const VDUSE_IOTLB_GET_FD: u32 = iowr::<vduse_iotlb_entry>(VDUSE_BASE, 0x10);
pub const VDUSE_DEV_GET_FEATURES: u32 = ior::<u64>(VDUSE_BASE, 0x11);

#[repr(C)]
pub struct vduse_config_data { pub offset: u32, pub length: u32, pub buffer: [u8; 0] }
pub const VDUSE_DEV_SET_CONFIG: u32 = iow::<vduse_config_data>(VDUSE_BASE, 0x12);
pub const VDUSE_DEV_INJECT_CONFIG_IRQ: u32 = io(VDUSE_BASE, 0x13);

#[repr(C)]
pub struct vduse_vq_config { pub index: u32, pub max_size: u16, pub reserved1: u16, pub group: u32, pub reserved2: [u16; 10] }
pub const VDUSE_VQ_SETUP: u32 = iow::<vduse_vq_config>(VDUSE_BASE, 0x14);

#[repr(C)] pub struct vduse_vq_state_split { pub avail_index: u16 }
#[repr(C)] pub struct vduse_vq_state_packed { pub last_avail_counter: u16, pub last_avail_idx: u16, pub last_used_counter: u16, pub last_used_idx: u16 }
#[repr(C)] pub struct vduse_vq_group_asid { pub group: u32, pub asid: u32 }

#[repr(C)]
pub union vduse_vq_info_state { pub split: vduse_vq_state_split, pub packed: vduse_vq_state_packed }
#[repr(C)]
pub struct vduse_vq_info { pub index: u32, pub num: u32, pub desc_addr: u64, pub driver_addr: u64, pub device_addr: u64, pub state: vduse_vq_info_state, pub ready: u8 }
pub const VDUSE_VQ_GET_INFO: u32 = iowr::<vduse_vq_info>(VDUSE_BASE, 0x15);

#[repr(C)] pub struct vduse_vq_eventfd { pub index: u32, pub fd: i32 }
pub const VDUSE_EVENTFD_DEASSIGN: i32 = -1;
pub const VDUSE_VQ_SETUP_KICKFD: u32 = iow::<vduse_vq_eventfd>(VDUSE_BASE, 0x16);
pub const VDUSE_VQ_INJECT_IRQ: u32 = iow::<u32>(VDUSE_BASE, 0x17);

#[repr(C)] pub struct vduse_iova_umem { pub uaddr: u64, pub iova: u64, pub size: u64, pub asid: u32, pub reserved: [u32; 5] }
pub const VDUSE_IOTLB_REG_UMEM: u32 = iow::<vduse_iova_umem>(VDUSE_BASE, 0x18);
pub const VDUSE_IOTLB_DEREG_UMEM: u32 = iow::<vduse_iova_umem>(VDUSE_BASE, 0x19);
#[repr(C)] pub struct vduse_iova_info { pub start: u64, pub last: u64, pub capability: u64, pub asid: u32, pub reserved: [u32; 5] }
pub const VDUSE_IOVA_CAP_UMEM: u64 = 1 << 0;
pub const VDUSE_IOTLB_GET_INFO: u32 = iowr::<vduse_iova_info>(VDUSE_BASE, 0x1a);
#[repr(C)] pub struct vduse_iotlb_entry_v2 { pub offset: u64, pub start: u64, pub last: u64, pub perm: u8, pub padding: [u8; 7], pub asid: u32, pub reserved: [u32; 11] }
pub const VDUSE_IOTLB_GET_FD2: u32 = iowr::<vduse_iotlb_entry_v2>(VDUSE_BASE, 0x1b);

#[repr(u32)]
pub enum vduse_req_type { VDUSE_GET_VQ_STATE, VDUSE_SET_STATUS, VDUSE_UPDATE_IOTLB, VDUSE_SET_VQ_GROUP_ASID, VDUSE_SET_VQ_READY, VDUSE_SUSPEND }
#[repr(C)] pub union vduse_vq_state_data { pub split: vduse_vq_state_split, pub packed: vduse_vq_state_packed }
#[repr(C)] pub struct vduse_vq_state { pub index: u32, pub state: vduse_vq_state_data }
#[repr(C)] pub struct vduse_dev_status { pub status: u8 }
#[repr(C)] pub struct vduse_iova_range { pub start: u64, pub last: u64 }
#[repr(C)] pub struct vduse_iova_range_v2 { pub start: u64, pub last: u64, pub asid: u32, pub padding: u32 }
#[repr(C)] pub struct vduse_vq_ready { pub num: u32, pub ready: u32 }
#[repr(C)] pub union vduse_dev_request_data { pub vq_state: vduse_vq_state, pub s: vduse_dev_status, pub iova: vduse_iova_range, pub iova_v2: vduse_iova_range_v2, pub vq_group_asid: vduse_vq_group_asid, pub vq_ready: vduse_vq_ready, pub padding: [u32; 32] }
#[repr(C)] pub struct vduse_dev_request { pub type_: u32, pub request_id: u32, pub reserved: [u32; 4], pub data: vduse_dev_request_data }
#[repr(C)] pub union vduse_dev_response_data { pub vq_state: vduse_vq_state, pub padding: [u32; 32] }
#[repr(C)] pub struct vduse_dev_response { pub request_id: u32, pub result: u32, pub reserved: [u32; 4], pub data: vduse_dev_response_data }
pub const VDUSE_REQ_RESULT_OK: u32 = 0x00;
pub const VDUSE_REQ_RESULT_FAILED: u32 = 0x01;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
