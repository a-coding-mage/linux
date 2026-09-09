// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

/*
 * Check whether it is safe to free COW fork blocks from an inode. It is unsafe
 * to do so when an inode has dirty cache or I/O in-flight, even if no shared
 * extents exist in the data fork, because outstanding I/O may target blocks
 * that were speculatively allocated to the COW fork.
 */
#[inline]
pub unsafe fn xfs_can_free_cowblocks(ip: *mut xfs_inode) -> bool {
    let inode: *mut inode = VFS_I(ip);

    if (inode_state_read_once(inode) & I_DIRTY_PAGES) != 0
        || mapping_tagged((*inode).i_mapping, PAGECACHE_TAG_DIRTY)
        || mapping_tagged((*inode).i_mapping, PAGECACHE_TAG_WRITEBACK)
        || atomic_read(&(*inode).i_dio_count) != 0
    {
        return false;
    }
    true
}

extern "C" {
    pub fn xfs_reflink_trim_around_shared(
        ip: *mut xfs_inode,
        irec: *mut xfs_bmbt_irec,
        shared: *mut bool,
    ) -> ::core::ffi::c_int;
    pub fn xfs_bmap_trim_cow(
        ip: *mut xfs_inode,
        imap: *mut xfs_bmbt_irec,
        shared: *mut bool,
    ) -> ::core::ffi::c_int;

    pub fn xfs_reflink_allocate_cow(
        ip: *mut xfs_inode,
        imap: *mut xfs_bmbt_irec,
        cmap: *mut xfs_bmbt_irec,
        shared: *mut bool,
        lockmode: *mut uint,
        convert_now: bool,
    ) -> ::core::ffi::c_int;
    pub fn xfs_reflink_convert_cow(
        ip: *mut xfs_inode,
        offset: xfs_off_t,
        count: xfs_off_t,
    ) -> ::core::ffi::c_int;
    pub fn xfs_reflink_convert_cow_locked(
        ip: *mut xfs_inode,
        offset_fsb: xfs_fileoff_t,
        count_fsb: xfs_filblks_t,
    ) -> ::core::ffi::c_int;

    pub fn xfs_reflink_cancel_cow_blocks(
        ip: *mut xfs_inode,
        tpp: *mut *mut xfs_trans,
        offset_fsb: xfs_fileoff_t,
        end_fsb: xfs_fileoff_t,
        cancel_real: bool,
    ) -> ::core::ffi::c_int;
    pub fn xfs_reflink_cancel_cow_range(
        ip: *mut xfs_inode,
        offset: xfs_off_t,
        count: xfs_off_t,
        cancel_real: bool,
    ) -> ::core::ffi::c_int;
    pub fn xfs_reflink_end_cow(
        ip: *mut xfs_inode,
        offset: xfs_off_t,
        count: xfs_off_t,
    ) -> ::core::ffi::c_int;
    pub fn xfs_reflink_end_atomic_cow(
        ip: *mut xfs_inode,
        offset: xfs_off_t,
        count: xfs_off_t,
    ) -> ::core::ffi::c_int;
    pub fn xfs_reflink_recover_cow(mp: *mut xfs_mount) -> ::core::ffi::c_int;
    pub fn xfs_reflink_remap_range(
        file_in: *mut file,
        pos_in: loff_t,
        file_out: *mut file,
        pos_out: loff_t,
        len: loff_t,
        remap_flags: ::core::ffi::c_uint,
    ) -> loff_t;
    pub fn xfs_reflink_inode_has_shared_extents(
        tp: *mut xfs_trans,
        ip: *mut xfs_inode,
        has_shared: *mut bool,
    ) -> ::core::ffi::c_int;
    pub fn xfs_reflink_clear_inode_flag(
        ip: *mut xfs_inode,
        tpp: *mut *mut xfs_trans,
    ) -> ::core::ffi::c_int;
    pub fn xfs_reflink_unshare(
        ip: *mut xfs_inode,
        offset: xfs_off_t,
        len: xfs_off_t,
    ) -> ::core::ffi::c_int;
    pub fn xfs_reflink_remap_prep(
        file_in: *mut file,
        pos_in: loff_t,
        file_out: *mut file,
        pos_out: loff_t,
        len: *mut loff_t,
        remap_flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn xfs_reflink_remap_blocks(
        src: *mut xfs_inode,
        pos_in: loff_t,
        dest: *mut xfs_inode,
        pos_out: loff_t,
        remap_len: loff_t,
        remapped: *mut loff_t,
    ) -> ::core::ffi::c_int;
    pub fn xfs_reflink_update_dest(
        dest: *mut xfs_inode,
        newlen: xfs_off_t,
        cowextsize: xfs_extlen_t,
        remap_flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn xfs_reflink_supports_rextsize(mp: *mut xfs_mount, rextsize: ::core::ffi::c_uint) -> bool;
    pub fn xfs_reflink_max_atomic_cow(mp: *mut xfs_mount) -> xfs_extlen_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
