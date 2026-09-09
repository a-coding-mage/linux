// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2006 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/* Kernel only BMAP related definitions and functions */

#[repr(C)]
pub struct xfs_bmbt_irec { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_extent_free_item { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_ifork { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_inode { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_mount { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_trans { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_bmalloca { _private: [u8; 0] }
#[repr(C)]
pub struct xfs_zone_alloc_ctx { _private: [u8; 0] }

#[cfg(CONFIG_XFS_RT)]
unsafe extern "C" {
    pub fn xfs_bmap_rtalloc(ap: *mut xfs_bmalloca) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_XFS_RT))]
#[inline]
pub unsafe fn xfs_bmap_rtalloc(_ap: *mut xfs_bmalloca) -> ::core::ffi::c_int {
    -EFSCORRUPTED
}

unsafe extern "C" {
    pub fn xfs_bmap_punch_delalloc_range(
        ip: *mut xfs_inode,
        whichfork: ::core::ffi::c_int,
        start_byte: xfs_off_t,
        end_byte: xfs_off_t,
        ac: *mut xfs_zone_alloc_ctx,
    );
}

#[repr(C)]
pub struct kgetbmap {
    pub bmv_offset: __s64, // file offset of segment in blocks
    pub bmv_block: __s64,  // starting block (64-bit daddr_t)
    pub bmv_length: __s64, // length of segment, blocks
    pub bmv_oflags: __s32, // output flags
}

unsafe extern "C" {
    pub fn xfs_getbmap(
        ip: *mut xfs_inode,
        bmv: *mut getbmapx,
        out: *mut kgetbmap,
    ) -> ::core::ffi::c_int;

    pub fn xfs_bmap_extsize_align(
        mp: *mut xfs_mount,
        gotp: *mut xfs_bmbt_irec,
        prevp: *mut xfs_bmbt_irec,
        extsz: xfs_extlen_t,
        rt: ::core::ffi::c_int,
        eof: ::core::ffi::c_int,
        delay: ::core::ffi::c_int,
        convert: ::core::ffi::c_int,
        offp: *mut xfs_fileoff_t,
        lenp: *mut xfs_extlen_t,
    ) -> ::core::ffi::c_int;

    pub fn xfs_bmap_adjacent(ap: *mut xfs_bmalloca) -> bool;

    pub fn xfs_bmap_last_extent(
        tp: *mut xfs_trans,
        ip: *mut xfs_inode,
        whichfork: ::core::ffi::c_int,
        rec: *mut xfs_bmbt_irec,
        is_empty: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

/* preallocation and hole punch interface */
#[repr(C)]
pub enum xfs_alloc_file_space_mode {
    XFS_ALLOC_FILE_SPACE_PREALLOC,
    XFS_ALLOC_FILE_SPACE_WRITE_ZEROES,
}

unsafe extern "C" {
    pub fn xfs_alloc_file_space(
        ip: *mut xfs_inode,
        offset: xfs_off_t,
        len: xfs_off_t,
        mode: xfs_alloc_file_space_mode,
    ) -> ::core::ffi::c_int;
    pub fn xfs_free_file_space(
        ip: *mut xfs_inode,
        offset: xfs_off_t,
        len: xfs_off_t,
        ac: *mut xfs_zone_alloc_ctx,
    ) -> ::core::ffi::c_int;
    pub fn xfs_collapse_file_space(
        ip: *mut xfs_inode,
        offset: xfs_off_t,
        len: xfs_off_t,
        ac: *mut xfs_zone_alloc_ctx,
    ) -> ::core::ffi::c_int;
    pub fn xfs_insert_file_space(
        ip: *mut xfs_inode,
        offset: xfs_off_t,
        len: xfs_off_t,
    ) -> ::core::ffi::c_int;

    /* EOF block manipulation functions */
    pub fn xfs_can_free_eofblocks(ip: *mut xfs_inode) -> bool;
    pub fn xfs_free_eofblocks(ip: *mut xfs_inode) -> ::core::ffi::c_int;
    pub fn xfs_swap_extents(
        ip: *mut xfs_inode,
        tip: *mut xfs_inode,
        sx: *mut xfs_swapext,
    ) -> ::core::ffi::c_int;
    pub fn xfs_fsb_to_db(ip: *mut xfs_inode, fsb: xfs_fsblock_t) -> xfs_daddr_t;
    pub fn xfs_bmap_count_leaves(ifp: *mut xfs_ifork, count: *mut xfs_filblks_t) -> xfs_extnum_t;
    pub fn xfs_bmap_count_blocks(
        tp: *mut xfs_trans,
        ip: *mut xfs_inode,
        whichfork: ::core::ffi::c_int,
        nextents: *mut xfs_extnum_t,
        count: *mut xfs_filblks_t,
    ) -> ::core::ffi::c_int;
    pub fn xfs_flush_unmap_range(
        ip: *mut xfs_inode,
        offset: xfs_off_t,
        len: xfs_off_t,
    ) -> ::core::ffi::c_int;
    pub fn xfs_bmap_replace_cow_mapping(
        ip: *mut xfs_inode,
        icur: *mut xfs_iext_cursor,
        got: *mut xfs_bmbt_irec,
        rep: *mut xfs_bmbt_irec,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
