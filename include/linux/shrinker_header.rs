/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

pub const SHRINKER_UNIT_BITS: usize = BITS_PER_LONG;

/*
 * Bitmap and deferred work of shrinker::id corresponding to memcg-aware
 * shrinkers, which have elements charged to the memcg.
 */
#[repr(C)]
pub struct shrinker_info_unit {
    pub nr_deferred: [atomic_long_t; SHRINKER_UNIT_BITS],
    pub map: [usize; SHRINKER_UNIT_BITS / (usize::BITS as usize)],
}

#[repr(C)]
pub struct shrinker_info {
    pub rcu: rcu_head,
    pub map_nr_max: ::core::ffi::c_int,
    pub unit: [*mut shrinker_info_unit; 0],
}

/*
 * This struct is used to pass information from page reclaim to the shrinkers.
 * We consolidate the values for easier extension later.
 *
 * The 'gfpmask' refers to the allocation we are currently trying to
 * fulfil.
 */
#[repr(C)]
pub struct shrink_control {
    pub gfp_mask: gfp_t,
    /* current node being shrunk (for NUMA aware shrinkers) */
    pub nid: ::core::ffi::c_int,
    /* How many objects scan_objects should scan and try to reclaim. */
    pub nr_to_scan: ::core::ffi::c_ulong,
    /* How many objects did scan_objects process? */
    pub nr_scanned: ::core::ffi::c_ulong,
    /* current memcg being shrunk (for memcg aware shrinkers) */
    pub memcg: *mut mem_cgroup,
}

pub const SHRINK_STOP: ::core::ffi::c_ulong = !0;
pub const SHRINK_EMPTY: ::core::ffi::c_ulong = !0 - 1;

#[repr(C)]
pub struct shrinker {
    pub count_objects: Option<unsafe extern "C" fn(*mut shrinker, *mut shrink_control) -> ::core::ffi::c_ulong>,
    pub scan_objects: Option<unsafe extern "C" fn(*mut shrinker, *mut shrink_control) -> ::core::ffi::c_ulong>,
    pub batch: ::core::ffi::c_long,
    pub seeks: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_uint,
    pub refcount: refcount_t,
    pub done: completion,
    pub rcu: rcu_head,
    pub private_data: *mut ::core::ffi::c_void,
    pub list: list_head,
    #[cfg(CONFIG_MEMCG)]
    pub id: ::core::ffi::c_int,
    #[cfg(CONFIG_SHRINKER_DEBUG)]
    pub debugfs_id: ::core::ffi::c_int,
    #[cfg(CONFIG_SHRINKER_DEBUG)]
    pub name: *const ::core::ffi::c_char,
    #[cfg(CONFIG_SHRINKER_DEBUG)]
    pub debugfs_entry: *mut dentry,
    pub nr_deferred: *mut atomic_long_t,
}

pub const DEFAULT_SEEKS: ::core::ffi::c_int = 2;
pub const SHRINKER_REGISTERED: ::core::ffi::c_uint = BIT(0);
pub const SHRINKER_ALLOCATED: ::core::ffi::c_uint = BIT(1);
pub const SHRINKER_NUMA_AWARE: ::core::ffi::c_uint = BIT(2);
pub const SHRINKER_MEMCG_AWARE: ::core::ffi::c_uint = BIT(3);
pub const SHRINKER_NONSLAB: ::core::ffi::c_uint = BIT(4);

unsafe extern "C" {
    pub fn shrinker_alloc(flags: ::core::ffi::c_uint, fmt: *const ::core::ffi::c_char, ...) -> *mut shrinker;
    pub fn shrinker_register(shrinker: *mut shrinker);
    pub fn shrinker_free(shrinker: *mut shrinker);

    #[cfg(CONFIG_SHRINKER_DEBUG)]
    pub fn shrinker_debugfs_rename(shrinker: *mut shrinker, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn shrinker_try_get(shrinker: *mut shrinker) -> bool {
    refcount_inc_not_zero(&mut (*shrinker).refcount)
}

#[inline]
pub unsafe fn shrinker_put(shrinker: *mut shrinker) {
    if refcount_dec_and_test(&mut (*shrinker).refcount) {
        complete(&mut (*shrinker).done);
    }
}

#[cfg(not(CONFIG_SHRINKER_DEBUG))]
#[inline]
pub unsafe fn shrinker_debugfs_rename(
    _shrinker: *mut shrinker,
    _fmt: *const ::core::ffi::c_char,
    ...,
) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
