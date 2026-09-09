/*
 *  NFSv4 file layout driver data structures.
 *
 *  Copyright (c) 2002
 *  The Regents of the University of Michigan
 *  All Rights Reserved
 *
 *  Dean Hildebrand <dhildebz@umich.edu>
 *
 *  Permission is granted to use, copy, create derivative works, and
 *  redistribute this software and such derivative works for any purpose,
 *  so long as the name of the University of Michigan is not used in any
 *  advertising or publicity pertaining to the use or distribution
 *  of this software without specific, written prior authorization.
 *
 *  This software is provided as is, without representation or warranty
 *  of any kind either express or implied, including without limitation
 *  the implied warranties of merchantability, fitness for a particular
 *  purpose, or noninfringement.
 */

// Dependency corresponding to: #include "../pnfs.h"

pub const NFS4_PNFS_MAX_STRIPE_CNT: u32 = 4096;
pub const NFS4_PNFS_MAX_MULTI_CNT: u32 = 256; // 256 fit into a u8 stripe_index

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum stripetype4 {
    STRIPE_SPARSE = 1,
    STRIPE_DENSE = 2,
}

#[repr(C)]
pub struct nfs4_file_layout_dsaddr {
    pub id_node: nfs4_deviceid_node,
    pub stripe_count: u32,
    pub stripe_indices: *mut u8,
    pub ds_num: u32,
    // Flexible array member: struct nfs4_pnfs_ds *ds_list[] __counted_by(ds_num)
    pub ds_list: [*mut nfs4_pnfs_ds; 0],
}

#[repr(C)]
pub struct nfs4_filelayout_segment {
    pub generic_hdr: pnfs_layout_segment,
    pub stripe_type: u32,
    pub commit_through_mds: u32,
    pub stripe_unit: u32,
    pub first_stripe_index: u32,
    pub pattern_offset: u64,
    pub deviceid: nfs4_deviceid,
    pub dsaddr: *mut nfs4_file_layout_dsaddr, // Point to GETDEVINFO data
    pub num_fh: core::ffi::c_uint,
    pub fh_array: *mut *mut nfs_fh,
}

#[repr(C)]
pub struct nfs4_filelayout {
    pub generic_hdr: pnfs_layout_hdr,
    pub commit_info: pnfs_ds_commit_info,
}

#[inline]
pub unsafe fn FILELAYOUT_FROM_HDR(lo: *mut pnfs_layout_hdr) -> *mut nfs4_filelayout {
    // Corresponds to Linux container_of(lo, struct nfs4_filelayout, generic_hdr).
    container_of!(lo, nfs4_filelayout, generic_hdr)
}

#[inline]
pub unsafe fn FILELAYOUT_LSEG(lseg: *mut pnfs_layout_segment) -> *mut nfs4_filelayout_segment {
    // Corresponds to Linux container_of(lseg, struct nfs4_filelayout_segment, generic_hdr).
    container_of!(lseg, nfs4_filelayout_segment, generic_hdr)
}

#[inline]
pub unsafe fn FILELAYOUT_DEVID_NODE(
    lseg: *mut pnfs_layout_segment,
) -> *mut nfs4_deviceid_node {
    &mut (*(*FILELAYOUT_LSEG(lseg)).dsaddr).id_node
}

#[inline]
pub unsafe fn filelayout_test_devid_invalid(node: *mut nfs4_deviceid_node) -> bool {
    test_bit(NFS_DEVICEID_INVALID, &(*node).flags)
}

extern "C" {
    pub fn filelayout_test_devid_unavailable(node: *mut nfs4_deviceid_node) -> bool;
    pub fn nfs4_fl_select_ds_fh(
        lseg: *mut pnfs_layout_segment,
        j: u32,
    ) -> *mut nfs_fh;
    pub fn nfs4_fl_calc_j_index(lseg: *mut pnfs_layout_segment, offset: loff_t) -> u32;
    pub fn nfs4_fl_calc_ds_index(lseg: *mut pnfs_layout_segment, j: u32) -> u32;
    pub fn nfs4_fl_prepare_ds(
        lseg: *mut pnfs_layout_segment,
        ds_idx: u32,
    ) -> *mut nfs4_pnfs_ds;
    pub fn nfs4_fl_alloc_deviceid_node(
        server: *mut nfs_server,
        pdev: *mut pnfs_device,
        gfp_flags: gfp_t,
    ) -> *mut nfs4_file_layout_dsaddr;
    pub fn nfs4_fl_put_deviceid(dsaddr: *mut nfs4_file_layout_dsaddr);
    pub fn nfs4_fl_free_deviceid(dsaddr: *mut nfs4_file_layout_dsaddr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
