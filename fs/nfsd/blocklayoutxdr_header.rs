/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from the C header: <linux/blkdev.h> and "xdr4.h".

use core::mem::ManuallyDrop;

pub const PNFS_BLOCK_LAYOUT4_SIZE: usize =
    core::mem::size_of::<__be32>() * 2 + // offset4
    core::mem::size_of::<__be32>() * 2 + // length4
    core::mem::size_of::<__be32>() +     // layoutiomode4
    core::mem::size_of::<__be32>() +     // layouttype4
    core::mem::size_of::<__be32>() +     // number of bytes
    core::mem::size_of::<__be32>();      // number of extents

#[repr(C)]
pub struct pnfs_block_extent {
    pub vol_id: nfsd4_deviceid,
    pub foff: u64,
    pub len: u64,
    pub soff: u64,
    pub es: pnfs_block_extent_state,
}

#[repr(C)]
pub struct pnfs_block_range {
    pub foff: u64,
    pub len: u64,
}

#[repr(C)]
pub struct pnfs_block_layout {
    pub nr_extents: u32,
    // C flexible array member, counted by nr_extents.
    pub extents: [pnfs_block_extent; 0],
}

/*
 * Random upper cap for the uuid length to avoid unbounded allocation.
 * Not actually limited by the protocol.
 */
pub const PNFS_BLOCK_UUID_LEN: usize = 128;

#[repr(C)]
pub struct pnfs_block_volume_simple {
    pub offset: u64,
    pub sig_len: u32,
    pub sig: [u8; PNFS_BLOCK_UUID_LEN],
}

#[repr(C)]
pub struct pnfs_block_volume_scsi {
    pub code_set: scsi_code_set,
    pub designator_type: scsi_designator_type,
    pub designator_len: core::ffi::c_int,
    pub designator: [u8; 256],
    pub pr_key: u64,
}

#[repr(C)]
pub union pnfs_block_volume_data {
    pub simple: ManuallyDrop<pnfs_block_volume_simple>,
    pub scsi: ManuallyDrop<pnfs_block_volume_scsi>,
}

#[repr(C)]
pub struct pnfs_block_volume {
    pub type_: pnfs_block_volume_type,
    pub data: pnfs_block_volume_data,
}

#[repr(C)]
pub struct pnfs_block_deviceaddr {
    pub nr_volumes: u32,
    // C flexible array member, counted by nr_volumes.
    pub volumes: [pnfs_block_volume; 0],
}

extern "C" {
    pub fn nfsd4_block_encode_getdeviceinfo(
        xdr: *mut xdr_stream,
        gdp: *const nfsd4_getdeviceinfo,
    ) -> __be32;
    pub fn nfsd4_block_encode_layoutget(
        xdr: *mut xdr_stream,
        lgp: *const nfsd4_layoutget,
    ) -> __be32;
    pub fn nfsd4_block_decode_layoutupdate(
        xdr: *mut xdr_stream,
        iomapp: *mut *mut iomap,
        nr_iomapsp: *mut core::ffi::c_int,
        block_size: u32,
    ) -> __be32;
    pub fn nfsd4_scsi_decode_layoutupdate(
        xdr: *mut xdr_stream,
        iomapp: *mut *mut iomap,
        nr_iomapsp: *mut core::ffi::c_int,
        block_size: u32,
    ) -> __be32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
