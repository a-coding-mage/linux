// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2006-2007 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// The C header guard and include dependencies are omitted from executable Rust.

#[repr(C)]
pub struct xfs_mru_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfs_mru_cache_elem {
    pub list_node: crate::list_head,
    pub key: ::kernel::ffi::c_ulong,
}

/* Function pointer type for callback to free a client's data pointer. */
pub type xfs_mru_cache_free_func_t =
    Option<unsafe extern "C" fn(*mut ::kernel::ffi::c_void, *mut xfs_mru_cache_elem)>;

extern "C" {
    pub fn xfs_mru_cache_init() -> ::kernel::ffi::c_int;
    pub fn xfs_mru_cache_uninit();
    pub fn xfs_mru_cache_create(
        mrup: *mut *mut xfs_mru_cache,
        data: *mut ::kernel::ffi::c_void,
        lifetime_ms: ::kernel::ffi::c_uint,
        grp_count: ::kernel::ffi::c_uint,
        free_func: xfs_mru_cache_free_func_t,
    ) -> ::kernel::ffi::c_int;
    pub fn xfs_mru_cache_destroy(mru: *mut xfs_mru_cache);
    pub fn xfs_mru_cache_insert(
        mru: *mut xfs_mru_cache,
        key: ::kernel::ffi::c_ulong,
        elem: *mut xfs_mru_cache_elem,
    ) -> ::kernel::ffi::c_int;
    pub fn xfs_mru_cache_remove(
        mru: *mut xfs_mru_cache,
        key: ::kernel::ffi::c_ulong,
    ) -> *mut xfs_mru_cache_elem;
    pub fn xfs_mru_cache_delete(mru: *mut xfs_mru_cache, key: ::kernel::ffi::c_ulong);
    pub fn xfs_mru_cache_lookup(
        mru: *mut xfs_mru_cache,
        key: ::kernel::ffi::c_ulong,
    ) -> *mut xfs_mru_cache_elem;
    pub fn xfs_mru_cache_done(mru: *mut xfs_mru_cache);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
