/* SPDX-License-Identifier: GPL-2.0 */
/*
 * DFS referral cache routines
 *
 * Copyright (c) 2018-2019 Paulo Alcantara <palcantara@suse.de>
 */

// Dependencies supplied by the surrounding kernel/CIFS translation are intentionally
// referenced here rather than reimplemented: nls_table, list_head, uuid-related
// definitions, CIFS types, workqueue_struct, atomic_t, and proc_ops.

extern "C" {
    pub static mut dfscache_wq: *mut workqueue_struct;
    pub static mut dfs_cache_ttl: atomic_t;
}

#[repr(C)]
pub struct dfs_cache_tgt_list {
    pub tl_numtgts: ::core::ffi::c_int,
    pub tl_list: list_head,
}

#[repr(C)]
pub struct dfs_cache_tgt_iterator {
    pub it_name: *mut ::core::ffi::c_char,
    pub it_path_consumed: ::core::ffi::c_int,
    pub it_list: list_head,
}

// C macro: DFS_CACHE_TGT_LIST_INIT(var)
#[macro_export]
macro_rules! DFS_CACHE_TGT_LIST_INIT {
    ($var:expr) => {
        dfs_cache_tgt_list {
            tl_numtgts: 0,
            tl_list: LIST_HEAD_INIT!($var.tl_list),
        }
    };
}

// C macro: DFS_CACHE_TGT_LIST(var)
#[macro_export]
macro_rules! DFS_CACHE_TGT_LIST {
    ($var:ident) => {
        let mut $var: dfs_cache_tgt_list = DFS_CACHE_TGT_LIST_INIT!($var);
    };
}

extern "C" {
    pub fn dfs_cache_init() -> ::core::ffi::c_int;
    pub fn dfs_cache_destroy();
    pub static dfscache_proc_ops: proc_ops;

    pub fn dfs_cache_find(
        xid: ::core::ffi::c_uint,
        ses: *mut cifs_ses,
        cp: *const nls_table,
        remap: ::core::ffi::c_int,
        path: *const ::core::ffi::c_char,
        ref_: *mut dfs_info3_param,
        tgt_list: *mut dfs_cache_tgt_list,
    ) -> ::core::ffi::c_int;
    pub fn dfs_cache_noreq_find(
        path: *const ::core::ffi::c_char,
        ref_: *mut dfs_info3_param,
        tgt_list: *mut dfs_cache_tgt_list,
    ) -> ::core::ffi::c_int;
    pub fn dfs_cache_noreq_update_tgthint(
        path: *const ::core::ffi::c_char,
        it: *const dfs_cache_tgt_iterator,
    );
    pub fn dfs_cache_get_tgt_referral(
        path: *const ::core::ffi::c_char,
        it: *const dfs_cache_tgt_iterator,
        ref_: *mut dfs_info3_param,
    ) -> ::core::ffi::c_int;
    pub fn dfs_cache_get_tgt_share(
        path: *mut ::core::ffi::c_char,
        it: *const dfs_cache_tgt_iterator,
        share: *mut *mut ::core::ffi::c_char,
        prefix: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn dfs_cache_canonical_path(
        path: *const ::core::ffi::c_char,
        cp: *const nls_table,
        remap: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    pub fn dfs_cache_remount_fs(cifs_sb: *mut cifs_sb_info) -> ::core::ffi::c_int;
    pub fn dfs_cache_refresh(work: *mut work_struct);
}

#[inline]
pub unsafe fn dfs_cache_get_next_tgt(
    tl: *mut dfs_cache_tgt_list,
    it: *mut dfs_cache_tgt_iterator,
) -> *mut dfs_cache_tgt_iterator {
    if tl.is_null()
        || (*tl).tl_numtgts == 0
        || list_empty(&(*tl).tl_list)
        || it.is_null()
        || list_is_last(&(*it).it_list, &(*tl).tl_list)
    {
        return core::ptr::null_mut();
    }
    list_next_entry(it, it_list)
}

#[inline]
pub unsafe fn dfs_cache_get_tgt_iterator(
    tl: *mut dfs_cache_tgt_list,
) -> *mut dfs_cache_tgt_iterator {
    if tl.is_null() {
        return core::ptr::null_mut();
    }
    list_first_entry_or_null(&(*tl).tl_list, dfs_cache_tgt_iterator, it_list)
}

#[inline]
pub unsafe fn dfs_cache_free_tgts(tl: *mut dfs_cache_tgt_list) {
    if tl.is_null() || (*tl).tl_numtgts == 0 || list_empty(&(*tl).tl_list) {
        return;
    }
    let mut it: *mut dfs_cache_tgt_iterator;
    let mut nit: *mut dfs_cache_tgt_iterator;
    list_for_each_entry_safe!(it, nit, &mut (*tl).tl_list, it_list, {
        list_del(&mut (*it).it_list);
        kfree((*it).it_name);
        kfree(it);
    });
    (*tl).tl_numtgts = 0;
}

#[inline]
pub unsafe fn dfs_cache_get_tgt_name(
    it: *const dfs_cache_tgt_iterator,
) -> *mut ::core::ffi::c_char {
    if it.is_null() { core::ptr::null_mut() } else { (*it).it_name }
}

#[inline]
pub unsafe fn dfs_cache_get_nr_tgts(tl: *const dfs_cache_tgt_list) -> ::core::ffi::c_int {
    if tl.is_null() { 0 } else { (*tl).tl_numtgts }
}

#[inline]
pub unsafe fn dfs_cache_get_ttl() -> ::core::ffi::c_int {
    atomic_read(&dfs_cache_ttl)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
