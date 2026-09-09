// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2002,2005 Silicon Graphics, Inc.
 * Copyright (c) 2010 David Chinner.
 * Copyright (c) 2011 Christoph Hellwig.
 * All Rights Reserved.
 */

// C header dependencies are supplied by other translation units.

/* Busy block/extent entry. */
#[repr(C)]
pub struct xfs_extent_busy {
    pub rb_node: rb_node,
    pub list: list_head,
    pub group: *mut xfs_group,
    pub bno: xfs_agblock_t,
    pub length: xfs_extlen_t,
    pub flags: ::core::ffi::c_uint,
}

pub const XFS_EXTENT_BUSY_DISCARDED: ::core::ffi::c_uint = 0x01;
pub const XFS_EXTENT_BUSY_SKIP_DISCARD: ::core::ffi::c_uint = 0x02;

/* List used to track groups of related busy extents through discard completion. */
#[repr(C)]
pub struct xfs_busy_extents {
    pub extent_list: list_head,
    pub endio_work: work_struct,
    pub owner: *mut ::core::ffi::c_void,
}

extern "C" {
    pub fn xfs_extent_busy_insert(
        tp: *mut xfs_trans,
        xg: *mut xfs_group,
        bno: xfs_agblock_t,
        len: xfs_extlen_t,
        flags: ::core::ffi::c_uint,
    );
    pub fn xfs_extent_busy_insert_discard(
        xg: *mut xfs_group,
        bno: xfs_agblock_t,
        len: xfs_extlen_t,
        busy_list: *mut list_head,
    );
    pub fn xfs_extent_busy_clear(list: *mut list_head, do_discard: bool);
    pub fn xfs_extent_busy_search(
        xg: *mut xfs_group,
        bno: xfs_agblock_t,
        len: xfs_extlen_t,
    ) -> ::core::ffi::c_int;
    pub fn xfs_extent_busy_reuse(
        xg: *mut xfs_group,
        fbno: xfs_agblock_t,
        flen: xfs_extlen_t,
        userdata: bool,
    );
    pub fn xfs_extent_busy_trim(
        xg: *mut xfs_group,
        minlen: xfs_extlen_t,
        maxlen: xfs_extlen_t,
        bno: *mut xfs_agblock_t,
        len: *mut xfs_extlen_t,
        busy_gen: *mut ::core::ffi::c_uint,
    ) -> bool;
    pub fn xfs_extent_busy_flush(
        tp: *mut xfs_trans,
        xg: *mut xfs_group,
        busy_gen: ::core::ffi::c_uint,
        alloc_flags: u32,
    ) -> ::core::ffi::c_int;
    pub fn xfs_extent_busy_wait_all(mp: *mut xfs_mount);
    pub fn xfs_extent_busy_list_empty(
        xg: *mut xfs_group,
        busy_gen: *mut ::core::ffi::c_uint,
    ) -> bool;
    pub fn xfs_extent_busy_alloc() -> *mut xfs_extent_busy_tree;

    pub fn xfs_extent_busy_ag_cmp(
        priv_: *mut ::core::ffi::c_void,
        a: *const list_head,
        b: *const list_head,
    ) -> ::core::ffi::c_int;

    pub fn list_sort(
        priv_: *mut ::core::ffi::c_void,
        list: *mut list_head,
        cmp: unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const list_head,
            *const list_head,
        ) -> ::core::ffi::c_int,
    );
}

#[inline]
pub unsafe fn xfs_extent_busy_sort(list: *mut list_head) {
    list_sort(::core::ptr::null_mut(), list, xfs_extent_busy_ag_cmp);
}

/*
 * Zoned RTGs don't need to track busy extents, as the actual block freeing
 * only happens by a zone reset, which forces out all transactions that
 * touched the to-be-reset zone first.
 */
#[macro_export]
macro_rules! xfs_group_has_extent_busy {
    ($mp:expr, $type_:expr) => {
        (($type_) == XG_TYPE_AG || !xfs_has_zoned($mp))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
