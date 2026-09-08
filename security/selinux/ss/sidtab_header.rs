/* SPDX-License-Identifier: GPL-2.0 */
/*
 * A security identifier table (sidtab) is a lookup table
 * of security context structures indexed by SID value.
 *
 * Original author: Stephen Smalley, <stephen.smalley.work@gmail.com>
 * Author: Ondrej Mosnacek, <omosnacek@gmail.com>
 *
 * Copyright (C) 2018 Red Hat, Inc.
 */

/* Dependencies from the original header:
 * <linux/spinlock_types.h>, <linux/log2.h>, <linux/hashtable.h>, "context.h"
 */

#[repr(C)]
pub struct sidtab_entry {
    pub sid: u32,
    pub hash: u32,
    pub context: context,
    /*
     * Present when CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE > 0:
     * struct sidtab_str_cache __rcu *cache;
     */
    #[cfg(CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE_gt_0)]
    pub cache: *mut sidtab_str_cache,
    pub list: hlist_node,
}

#[repr(C)]
pub union sidtab_entry_inner {
    pub ptr_inner: *mut sidtab_node_inner,
    pub ptr_leaf: *mut sidtab_node_leaf,
}

/* align node size to page boundary */
pub const SIDTAB_NODE_ALLOC_SHIFT: usize = PAGE_SHIFT as usize;
pub const SIDTAB_NODE_ALLOC_SIZE: usize = PAGE_SIZE as usize;

pub const fn size_to_shift(size: usize) -> usize {
    if size == 1 {
        1
    } else {
        usize::BITS as usize - (size - 1).leading_zeros() as usize
    }
}

pub const SIDTAB_INNER_SHIFT: usize =
    SIDTAB_NODE_ALLOC_SHIFT - size_to_shift(core::mem::size_of::<sidtab_entry_inner>());
pub const SIDTAB_INNER_ENTRIES: usize = 1usize << SIDTAB_INNER_SHIFT;
pub const SIDTAB_LEAF_ENTRIES: usize =
    SIDTAB_NODE_ALLOC_SIZE / core::mem::size_of::<sidtab_entry>();

pub const SIDTAB_MAX_BITS: usize = 32;
pub const SIDTAB_MAX: u32 = u32::MAX;
/* ensure enough tree levels for SIDTAB_MAX entries */
pub const SIDTAB_MAX_LEVEL: usize = (SIDTAB_MAX_BITS - size_to_shift(SIDTAB_LEAF_ENTRIES))
    .div_ceil(SIDTAB_INNER_SHIFT);

#[repr(C)]
pub struct sidtab_node_leaf {
    pub entries: [sidtab_entry; SIDTAB_LEAF_ENTRIES],
}

#[repr(C)]
pub struct sidtab_node_inner {
    pub entries: [sidtab_entry_inner; SIDTAB_INNER_ENTRIES],
}

#[repr(C)]
pub struct sidtab_isid_entry {
    pub set: core::ffi::c_int,
    pub entry: sidtab_entry,
}

#[repr(C)]
pub struct sidtab_convert_params {
    pub args: *mut convert_context_args,
    pub target: *mut sidtab,
}

pub const SIDTAB_HASH_BITS: usize = CONFIG_SECURITY_SELINUX_SIDTAB_HASH_BITS as usize;
pub const SIDTAB_HASH_BUCKETS: usize = 1usize << SIDTAB_HASH_BITS;

#[repr(C)]
pub struct sidtab {
    /*
     * lock-free read access only for as many items as a prior read of
     * 'count'
     */
    pub roots: [sidtab_entry_inner; SIDTAB_MAX_LEVEL + 1],
    /*
     * access atomically via {READ|WRITE}_ONCE(); only increment under
     * spinlock
     */
    pub count: u32,
    /* access only under spinlock */
    pub convert: *mut sidtab_convert_params,
    pub frozen: bool,
    pub lock: spinlock_t,

    /*
     * Present when CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE > 0:
     * SID -> context string cache
     */
    #[cfg(CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE_gt_0)]
    pub cache_free_slots: u32,
    #[cfg(CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE_gt_0)]
    pub cache_lru_list: list_head,
    #[cfg(CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE_gt_0)]
    pub cache_lock: spinlock_t,

    /* index == SID - 1 (no entry for SECSID_NULL) */
    pub isids: [sidtab_isid_entry; SECINITSID_NUM],

    /* Hash table for fast reverse context-to-sid lookups. */
    pub context_to_sid: [hlist_head; SIDTAB_HASH_BUCKETS],
}

unsafe extern "C" {
    pub fn sidtab_init(s: *mut sidtab) -> core::ffi::c_int;
    pub fn sidtab_set_initial(
        s: *mut sidtab,
        sid: u32,
        context: *mut context,
    ) -> core::ffi::c_int;
    pub fn sidtab_search_entry(s: *mut sidtab, sid: u32) -> *mut sidtab_entry;
    pub fn sidtab_search_entry_force(s: *mut sidtab, sid: u32) -> *mut sidtab_entry;
}

#[inline]
pub unsafe fn sidtab_search(s: *mut sidtab, sid: u32) -> *mut context {
    let entry: *mut sidtab_entry = unsafe { sidtab_search_entry(s, sid) };

    if !entry.is_null() {
        unsafe { core::ptr::addr_of_mut!((*entry).context) }
    } else {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn sidtab_search_force(s: *mut sidtab, sid: u32) -> *mut context {
    let entry: *mut sidtab_entry = unsafe { sidtab_search_entry_force(s, sid) };

    if !entry.is_null() {
        unsafe { core::ptr::addr_of_mut!((*entry).context) }
    } else {
        core::ptr::null_mut()
    }
}

unsafe extern "C" {
    pub fn sidtab_convert(
        s: *mut sidtab,
        params: *mut sidtab_convert_params,
    ) -> core::ffi::c_int;

    pub fn sidtab_cancel_convert(s: *mut sidtab);

    /*
     * Original annotations:
     * __acquires(&s->lock)
     * __releases(&s->lock)
     */
    pub fn sidtab_freeze_begin(s: *mut sidtab, flags: *mut core::ffi::c_ulong);
    pub fn sidtab_freeze_end(s: *mut sidtab, flags: *mut core::ffi::c_ulong);

    pub fn sidtab_context_to_sid(
        s: *mut sidtab,
        context: *mut context,
        sid: *mut u32,
    ) -> core::ffi::c_int;

    pub fn sidtab_destroy(s: *mut sidtab);

    pub fn sidtab_hash_stats(sidtab: *mut sidtab, page: *mut core::ffi::c_char)
        -> core::ffi::c_int;
}

/* CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE > 0 */
#[cfg(CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE_gt_0)]
unsafe extern "C" {
    pub fn sidtab_sid2str_put(
        s: *mut sidtab,
        entry: *mut sidtab_entry,
        str_: *const core::ffi::c_char,
        str_len: u32,
    );
    pub fn sidtab_sid2str_get(
        s: *mut sidtab,
        entry: *mut sidtab_entry,
        out: *mut *mut core::ffi::c_char,
        out_len: *mut u32,
    ) -> core::ffi::c_int;
}

/* !(CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE > 0) */
#[cfg(not(CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE_gt_0))]
#[inline]
pub unsafe fn sidtab_sid2str_put(
    _s: *mut sidtab,
    _entry: *mut sidtab_entry,
    _str: *const core::ffi::c_char,
    _str_len: u32,
) {
}

#[cfg(not(CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE_gt_0))]
#[inline]
pub unsafe fn sidtab_sid2str_get(
    _s: *mut sidtab,
    _entry: *mut sidtab_entry,
    _out: *mut *mut core::ffi::c_char,
    _out_len: *mut u32,
) -> core::ffi::c_int {
    -ENOENT
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
