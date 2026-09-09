/* SPDX-License-Identifier: MIT */
/* Unified block-device I/O interface for Xen guest OSes. */

/* Dependencies supplied by Xen interface modules: grant_ref_t and ring types. */

pub type blkif_vdev_t = u16;
pub type blkif_sector_t = u64;

pub const BLKIF_OP_READ: u8 = 0;
pub const BLKIF_OP_WRITE: u8 = 1;
pub const BLKIF_OP_WRITE_BARRIER: u8 = 2;
pub const BLKIF_OP_FLUSH_DISKCACHE: u8 = 3;
pub const BLKIF_OP_DISCARD: u8 = 5;
pub const BLKIF_OP_INDIRECT: u8 = 6;

pub const BLKIF_MAX_SEGMENTS_PER_REQUEST: usize = 11;
pub const BLKIF_MAX_INDIRECT_PAGES_PER_REQUEST: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_request_segment {
    pub gref: grant_ref_t,
    pub first_sect: u8,
    pub last_sect: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct blkif_request_rw {
    pub nr_segments: u8,
    pub handle: blkif_vdev_t,
    #[cfg(not(CONFIG_X86_32))]
    pub _pad1: u32,
    pub id: u64,
    pub sector_number: blkif_sector_t,
    pub seg: [blkif_request_segment; BLKIF_MAX_SEGMENTS_PER_REQUEST],
}

pub const BLKIF_DISCARD_SECURE: u8 = 1 << 0;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct blkif_request_discard {
    pub flag: u8,
    pub _pad1: blkif_vdev_t,
    #[cfg(not(CONFIG_X86_32))]
    pub _pad2: u32,
    pub id: u64,
    pub sector_number: blkif_sector_t,
    pub nr_sectors: u64,
    pub _pad3: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct blkif_request_other {
    pub _pad1: u8,
    pub _pad2: blkif_vdev_t,
    #[cfg(not(CONFIG_X86_32))]
    pub _pad3: u32,
    pub id: u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct blkif_request_indirect {
    pub indirect_op: u8,
    pub nr_segments: u16,
    #[cfg(not(CONFIG_X86_32))]
    pub _pad1: u32,
    pub id: u64,
    pub sector_number: blkif_sector_t,
    pub handle: blkif_vdev_t,
    pub _pad2: u16,
    pub indirect_grefs: [grant_ref_t; BLKIF_MAX_INDIRECT_PAGES_PER_REQUEST],
    #[cfg(not(CONFIG_X86_32))]
    pub _pad3: u32,
    #[cfg(CONFIG_X86_32)]
    pub _pad3: u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union blkif_request_u {
    pub rw: blkif_request_rw,
    pub discard: blkif_request_discard,
    pub other: blkif_request_other,
    pub indirect: blkif_request_indirect,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct blkif_request {
    pub operation: u8,
    pub u: blkif_request_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blkif_response {
    pub id: u64,
    pub operation: u8,
    pub status: i16,
}

pub const BLKIF_RSP_EOPNOTSUPP: i16 = -2;
pub const BLKIF_RSP_ERROR: i16 = -1;
pub const BLKIF_RSP_OKAY: i16 = 0;

/* Equivalent of DEFINE_RING_TYPES(blkif, struct blkif_request, struct blkif_response). */

pub const VDISK_CDROM: u32 = 0x1;
pub const VDISK_REMOVABLE: u32 = 0x2;
pub const VDISK_READONLY: u32 = 0x4;

pub const XEN_IDE0_MAJOR: u32 = 3;
pub const XEN_IDE1_MAJOR: u32 = 22;
pub const XEN_SCSI_DISK0_MAJOR: u32 = 8;
pub const XEN_SCSI_DISK1_MAJOR: u32 = 65;
pub const XEN_SCSI_DISK2_MAJOR: u32 = 66;
pub const XEN_SCSI_DISK3_MAJOR: u32 = 67;
pub const XEN_SCSI_DISK4_MAJOR: u32 = 68;
pub const XEN_SCSI_DISK5_MAJOR: u32 = 69;
pub const XEN_SCSI_DISK6_MAJOR: u32 = 70;
pub const XEN_SCSI_DISK7_MAJOR: u32 = 71;
pub const XEN_SCSI_DISK8_MAJOR: u32 = 128;
pub const XEN_SCSI_DISK9_MAJOR: u32 = 129;
pub const XEN_SCSI_DISK10_MAJOR: u32 = 130;
pub const XEN_SCSI_DISK11_MAJOR: u32 = 131;
pub const XEN_SCSI_DISK12_MAJOR: u32 = 132;
pub const XEN_SCSI_DISK13_MAJOR: u32 = 133;
pub const XEN_SCSI_DISK14_MAJOR: u32 = 134;
pub const XEN_SCSI_DISK15_MAJOR: u32 = 135;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
