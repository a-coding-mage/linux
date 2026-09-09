// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2003-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// Translated from xfs_iomap.h. The Linux iomap declarations and XFS integer
// aliases referenced here are supplied by external dependencies.

#[repr(C)]
pub struct xfs_inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_bmbt_irec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_zone_alloc_ctx {
    _private: [u8; 0],
}

extern "C" {
    pub fn xfs_iomap_write_direct(
        ip: *mut xfs_inode,
        offset_fsb: xfs_fileoff_t,
        count_fsb: xfs_fileoff_t,
        flags: ::core::ffi::c_uint,
        imap: *mut xfs_bmbt_irec,
        sequence: *mut u64,
    ) -> ::core::ffi::c_int;

    pub fn xfs_iomap_write_unwritten(
        ip: *mut xfs_inode,
        offset: xfs_off_t,
        length: xfs_off_t,
        update_isize: bool,
    ) -> ::core::ffi::c_int;

    pub fn xfs_iomap_eof_align_last_fsb(
        ip: *mut xfs_inode,
        end_fsb: xfs_fileoff_t,
    ) -> xfs_fileoff_t;

    pub fn xfs_iomap_inode_sequence(ip: *mut xfs_inode, iomap_flags: u16) -> u64;

    pub fn xfs_bmbt_to_iomap(
        ip: *mut xfs_inode,
        iomap: *mut iomap,
        imap: *mut xfs_bmbt_irec,
        mapping_flags: ::core::ffi::c_uint,
        iomap_flags: u16,
        sequence_cookie: u64,
    ) -> ::core::ffi::c_int;

    pub fn xfs_zero_range(
        ip: *mut xfs_inode,
        pos: loff_t,
        len: loff_t,
        ac: *mut xfs_zone_alloc_ctx,
        did_zero: *mut bool,
    ) -> ::core::ffi::c_int;

    pub fn xfs_truncate_page(
        ip: *mut xfs_inode,
        pos: loff_t,
        ac: *mut xfs_zone_alloc_ctx,
        did_zero: *mut bool,
    ) -> ::core::ffi::c_int;

    pub fn xfs_read_iomap_begin(
        inode: *mut inode,
        offset: loff_t,
        length: loff_t,
        flags: ::core::ffi::c_uint,
        iomap: *mut iomap,
        srcmap: *mut iomap,
    ) -> ::core::ffi::c_int;

    pub static xfs_buffered_write_iomap_ops: iomap_ops;
    pub static xfs_direct_write_iomap_ops: iomap_ops;
    pub static xfs_zoned_direct_write_iomap_ops: iomap_ops;
    pub static xfs_read_iomap_ops: iomap_ops;
    pub static xfs_seek_iomap_ops: iomap_ops;
    pub static xfs_xattr_iomap_ops: iomap_ops;
    pub static xfs_dax_write_iomap_ops: iomap_ops;
    pub static xfs_atomic_write_cow_iomap_ops: iomap_ops;
    pub static xfs_iomap_write_ops: iomap_write_ops;
}

#[inline]
pub unsafe fn xfs_aligned_fsb_count(
    offset_fsb: xfs_fileoff_t,
    mut count_fsb: xfs_filblks_t,
    extsz: xfs_extlen_t,
) -> xfs_filblks_t {
    if extsz != 0 {
        let mut align: xfs_extlen_t = (offset_fsb % extsz as xfs_fileoff_t) as xfs_extlen_t;
        if align != 0 {
            count_fsb = count_fsb.wrapping_add(align as xfs_filblks_t);
        }
        align = (count_fsb % extsz as xfs_filblks_t) as xfs_extlen_t;
        if align != 0 {
            count_fsb = count_fsb.wrapping_add((extsz - align) as xfs_filblks_t);
        }
    }
    count_fsb
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
