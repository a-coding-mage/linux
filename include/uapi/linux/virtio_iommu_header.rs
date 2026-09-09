/* SPDX-License-Identifier: BSD-3-Clause */
/*
 * Virtio-iommu definition v0.12
 *
 * Copyright (C) 2019 Arm Ltd.
 */

/* Feature bits */
pub const VIRTIO_IOMMU_F_INPUT_RANGE: u32 = 0;
pub const VIRTIO_IOMMU_F_DOMAIN_RANGE: u32 = 1;
pub const VIRTIO_IOMMU_F_MAP_UNMAP: u32 = 2;
pub const VIRTIO_IOMMU_F_BYPASS: u32 = 3;
pub const VIRTIO_IOMMU_F_PROBE: u32 = 4;
pub const VIRTIO_IOMMU_F_MMIO: u32 = 5;
pub const VIRTIO_IOMMU_F_BYPASS_CONFIG: u32 = 6;

#[repr(C)]
pub struct virtio_iommu_range_64 { pub start: u64, pub end: u64 }

#[repr(C)]
pub struct virtio_iommu_range_32 { pub start: u32, pub end: u32 }

#[repr(C)]
pub struct virtio_iommu_config {
    /* Supported page sizes */
    pub page_size_mask: u64,
    /* Supported IOVA range */
    pub input_range: virtio_iommu_range_64,
    /* Max domain ID size */
    pub domain_range: virtio_iommu_range_32,
    /* Probe buffer size */
    pub probe_size: u32,
    pub bypass: u8,
    pub reserved: [u8; 3],
}

/* Request types */
pub const VIRTIO_IOMMU_T_ATTACH: u8 = 0x01;
pub const VIRTIO_IOMMU_T_DETACH: u8 = 0x02;
pub const VIRTIO_IOMMU_T_MAP: u8 = 0x03;
pub const VIRTIO_IOMMU_T_UNMAP: u8 = 0x04;
pub const VIRTIO_IOMMU_T_PROBE: u8 = 0x05;

/* Status types */
pub const VIRTIO_IOMMU_S_OK: u8 = 0x00;
pub const VIRTIO_IOMMU_S_IOERR: u8 = 0x01;
pub const VIRTIO_IOMMU_S_UNSUPP: u8 = 0x02;
pub const VIRTIO_IOMMU_S_DEVERR: u8 = 0x03;
pub const VIRTIO_IOMMU_S_INVAL: u8 = 0x04;
pub const VIRTIO_IOMMU_S_RANGE: u8 = 0x05;
pub const VIRTIO_IOMMU_S_NOENT: u8 = 0x06;
pub const VIRTIO_IOMMU_S_FAULT: u8 = 0x07;
pub const VIRTIO_IOMMU_S_NOMEM: u8 = 0x08;

#[repr(C)]
pub struct virtio_iommu_req_head { pub r#type: u8, pub reserved: [u8; 3] }

#[repr(C)]
pub struct virtio_iommu_req_tail { pub status: u8, pub reserved: [u8; 3] }

pub const VIRTIO_IOMMU_ATTACH_F_BYPASS: u32 = 1 << 0;

#[repr(C)]
pub struct virtio_iommu_req_attach {
    pub head: virtio_iommu_req_head,
    pub domain: u32,
    pub endpoint: u32,
    pub flags: u32,
    pub reserved: [u8; 4],
    pub tail: virtio_iommu_req_tail,
}

#[repr(C)]
pub struct virtio_iommu_req_detach {
    pub head: virtio_iommu_req_head,
    pub domain: u32,
    pub endpoint: u32,
    pub reserved: [u8; 8],
    pub tail: virtio_iommu_req_tail,
}

pub const VIRTIO_IOMMU_MAP_F_READ: u32 = 1 << 0;
pub const VIRTIO_IOMMU_MAP_F_WRITE: u32 = 1 << 1;
pub const VIRTIO_IOMMU_MAP_F_MMIO: u32 = 1 << 2;
pub const VIRTIO_IOMMU_MAP_F_MASK: u32 = VIRTIO_IOMMU_MAP_F_READ | VIRTIO_IOMMU_MAP_F_WRITE | VIRTIO_IOMMU_MAP_F_MMIO;

#[repr(C)]
pub struct virtio_iommu_req_map {
    pub head: virtio_iommu_req_head,
    pub domain: u32,
    pub virt_start: u64,
    pub virt_end: u64,
    pub phys_start: u64,
    pub flags: u32,
    pub tail: virtio_iommu_req_tail,
}

#[repr(C)]
pub struct virtio_iommu_req_unmap {
    pub head: virtio_iommu_req_head,
    pub domain: u32,
    pub virt_start: u64,
    pub virt_end: u64,
    pub reserved: [u8; 4],
    pub tail: virtio_iommu_req_tail,
}

pub const VIRTIO_IOMMU_PROBE_T_NONE: u16 = 0;
pub const VIRTIO_IOMMU_PROBE_T_RESV_MEM: u16 = 1;
pub const VIRTIO_IOMMU_PROBE_T_MASK: u16 = 0xfff;

#[repr(C)]
pub struct virtio_iommu_probe_property { pub r#type: u16, pub length: u16 }

pub const VIRTIO_IOMMU_RESV_MEM_T_RESERVED: u8 = 0;
pub const VIRTIO_IOMMU_RESV_MEM_T_MSI: u8 = 1;

#[repr(C)]
pub struct virtio_iommu_probe_resv_mem {
    pub head: virtio_iommu_probe_property,
    pub subtype: u8,
    pub reserved: [u8; 3],
    pub start: u64,
    pub end: u64,
}

#[repr(C)]
pub struct virtio_iommu_req_probe {
    pub head: virtio_iommu_req_head,
    pub endpoint: u32,
    pub reserved: [u8; 64],
    pub properties: [u8; 0],
    /*
     * Tail follows the variable-length properties array. No padding,
     * property lengths are all aligned on 8 bytes.
     */
}

/* Fault types */
pub const VIRTIO_IOMMU_FAULT_R_UNKNOWN: u8 = 0;
pub const VIRTIO_IOMMU_FAULT_R_DOMAIN: u8 = 1;
pub const VIRTIO_IOMMU_FAULT_R_MAPPING: u8 = 2;

pub const VIRTIO_IOMMU_FAULT_F_READ: u32 = 1 << 0;
pub const VIRTIO_IOMMU_FAULT_F_WRITE: u32 = 1 << 1;
pub const VIRTIO_IOMMU_FAULT_F_EXEC: u32 = 1 << 2;
pub const VIRTIO_IOMMU_FAULT_F_ADDRESS: u32 = 1 << 8;

#[repr(C)]
pub struct virtio_iommu_fault {
    pub reason: u8,
    pub reserved: [u8; 3],
    pub flags: u32,
    pub endpoint: u32,
    pub reserved2: [u8; 4],
    pub address: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
