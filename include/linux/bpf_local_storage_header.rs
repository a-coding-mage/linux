/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2019 Facebook
 * Copyright 2020 Google LLC.
 */

// C dependencies: linux/bpf.h, linux/filter.h, linux/rculist.h,
// linux/list.h, linux/hash.h, linux/types.h, linux/bpf_mem_alloc.h,
// uapi/linux/btf.h, and asm/rqspinlock.h.

pub const BPF_LOCAL_STORAGE_CACHE_SIZE: usize = 16;

#[repr(C)]
pub struct bpf_local_storage_map_bucket {
    pub list: hlist_head,
    pub lock: rqspinlock_t,
}

extern "C" {
    pub fn rcu_dereference_check<T>(ptr: *mut T, condition: bool) -> *mut T;
    pub fn rcu_access_pointer<T>(ptr: *mut T) -> *mut T;
    pub fn bpf_rcu_lock_held() -> bool;
    pub fn rcu_read_lock_trace_held() -> bool;
}

#[inline]
pub unsafe fn bpf_local_storage_lookup(
    local_storage: *mut bpf_local_storage,
    smap: *mut bpf_local_storage_map,
    cacheit_lockit: bool,
) -> *mut bpf_local_storage_data {
    /* Fast path (cache hit) */
    let sdata = rcu_dereference_check(
        (*local_storage).cache[(*smap).cache_idx as usize],
        bpf_rcu_lock_held(),
    );
    if !sdata.is_null() && rcu_access_pointer((*sdata).smap) == smap {
        return sdata;
    }

    /* Slow path (cache miss). The hlist_for_each_entry_rcu dependency is external. */
    let mut selem: *mut bpf_local_storage_elem = core::ptr::null_mut();
    for candidate in hlist_for_each_entry_rcu::<bpf_local_storage_elem>(
        &mut (*local_storage).list,
        rcu_read_lock_trace_held(),
    ) {
        if rcu_access_pointer((*SDATA(candidate)).smap) == smap {
            selem = candidate;
            break;
        }
    }
    if selem.is_null() {
        return core::ptr::null_mut();
    }
    if cacheit_lockit {
        __bpf_local_storage_insert_cache(local_storage, smap, selem);
    }
    SDATA(selem)
}

/* Thp map is not the primary owner of a bpf_local_storage_elem.
 * Instead, the container object (eg. sk->sk_bpf_storage) is.
 *
 * The map (bpf_local_storage_map) is for two purposes
 * 1. Define the size of the "local storage".  It is
 *    the map's value_size.
 *
 * 2. Maintain a list to keep track of all elems such
 *    that they can be cleaned up during the map destruction.
 *
 * When a bpf local storage is being looked up for a
 * particular object,  the "bpf_map" pointer is actually used
 * as the "key" to search in the list of elem in
 * the respective bpf_local_storage owned by the object.
 *
 * e.g. sk->sk_bpf_storage is the mini-map with the "bpf_map" pointer
 * as the searching key.
 */
#[repr(C)]
pub struct bpf_local_storage_map {
    pub map: bpf_map,
    pub buckets: *mut bpf_local_storage_map_bucket,
    pub bucket_log: u32,
    pub elem_size: u16,
    pub cache_idx: u16,
}

#[repr(C)]
pub struct bpf_local_storage_data {
    pub smap: *mut bpf_local_storage_map,
    pub data: [u8; 0], // __aligned(8)
}

pub const SELEM_MAP_UNLINKED: u32 = 1 << 0;
pub const SELEM_STORAGE_UNLINKED: u32 = 1 << 1;
pub const SELEM_UNLINKED: u32 = SELEM_MAP_UNLINKED | SELEM_STORAGE_UNLINKED;
pub const SELEM_TOFREE: u32 = 1 << 2;

#[repr(C)]
pub union bpf_local_storage_elem_rcu_or_free {
    pub rcu: rcu_head,
    pub free_node: hlist_node,
}

#[repr(C)]
pub struct bpf_local_storage_elem {
    pub map_node: hlist_node,
    pub snode: hlist_node,
    pub local_storage: *mut bpf_local_storage,
    pub rcu_or_free: bpf_local_storage_elem_rcu_or_free,
    pub state: atomic_t,
    // 4 bytes hole
    pub sdata: bpf_local_storage_data, // ____cacheline_aligned
}

#[repr(C)]
pub struct bpf_local_storage {
    pub cache: [*mut bpf_local_storage_data; BPF_LOCAL_STORAGE_CACHE_SIZE],
    pub list: hlist_head,
    pub owner: *mut core::ffi::c_void,
    pub rcu: rcu_head,
    pub lock: rqspinlock_t,
    pub mem_charge: u64,
    pub owner_refcnt: refcount_t,
}

// U16_MAX is much more than enough for sk local storage considering a tcp_sock is ~2k.
// Requires the external kernel constants KMALLOC_MAX_SIZE, MAX_BPF_STACK, and U16_MAX.
pub const BPF_LOCAL_STORAGE_MAX_VALUE_SIZE: u32 =
    min_t::<u32>(
        KMALLOC_MAX_SIZE - MAX_BPF_STACK - core::mem::size_of::<bpf_local_storage_elem>() as u32,
        U16_MAX - core::mem::size_of::<bpf_local_storage_elem>() as u32,
    );

// C container_of((_SDATA), struct bpf_local_storage_elem, sdata).
#[macro_export]
macro_rules! SELEM {
    ($sdata:expr) => {
        container_of!($sdata, bpf_local_storage_elem, sdata)
    };
}

#[inline]
pub unsafe fn SDATA(selem: *mut bpf_local_storage_elem) -> *mut bpf_local_storage_data {
    core::ptr::addr_of_mut!((*selem).sdata)
}

#[repr(C)]
pub struct bpf_local_storage_cache {
    pub idx_lock: spinlock_t,
    pub idx_usage_counts: [u64; BPF_LOCAL_STORAGE_CACHE_SIZE],
}

// C macro DEFINE_BPF_STORAGE_CACHE(name), including its lock initializer.
#[macro_export]
macro_rules! DEFINE_BPF_STORAGE_CACHE {
    ($name:ident) => {
        static mut $name: bpf_local_storage_cache = bpf_local_storage_cache {
            idx_lock: __SPIN_LOCK_UNLOCKED!($name.idx_lock),
            idx_usage_counts: [0; BPF_LOCAL_STORAGE_CACHE_SIZE],
        };
    };
}

extern "C" {
    pub fn bpf_local_storage_map_alloc_check(attr: *mut bpf_attr) -> i32;
    pub fn bpf_local_storage_map_alloc(
        attr: *mut bpf_attr,
        cache: *mut bpf_local_storage_cache,
    ) -> *mut bpf_map;
    pub fn __bpf_local_storage_insert_cache(
        local_storage: *mut bpf_local_storage,
        smap: *mut bpf_local_storage_map,
        selem: *mut bpf_local_storage_elem,
    );
    pub fn bpf_local_storage_destroy(local_storage: *mut bpf_local_storage) -> u32;
    pub fn bpf_local_storage_map_free(map: *mut bpf_map, cache: *mut bpf_local_storage_cache);
    pub fn bpf_local_storage_map_check_btf(
        map: *mut bpf_map, btf: *const btf, key_type: *const btf_type, value_type: *const btf_type,
    ) -> i32;
    pub fn bpf_selem_link_storage_nolock(local_storage: *mut bpf_local_storage, selem: *mut bpf_local_storage_elem);
    pub fn bpf_selem_unlink(selem: *mut bpf_local_storage_elem) -> i32;
    pub fn bpf_selem_link_map(smap: *mut bpf_local_storage_map, local_storage: *mut bpf_local_storage, selem: *mut bpf_local_storage_elem) -> i32;
    pub fn bpf_selem_alloc(smap: *mut bpf_local_storage_map, owner: *mut core::ffi::c_void, value: *mut core::ffi::c_void, swap_uptrs: bool) -> *mut bpf_local_storage_elem;
    pub fn bpf_selem_free(selem: *mut bpf_local_storage_elem, reuse_now: bool);
    pub fn bpf_local_storage_alloc(owner: *mut core::ffi::c_void, smap: *mut bpf_local_storage_map, first_selem: *mut bpf_local_storage_elem) -> i32;
    pub fn bpf_local_storage_update(owner: *mut core::ffi::c_void, smap: *mut bpf_local_storage_map, value: *mut core::ffi::c_void, map_flags: u64, swap_uptrs: bool) -> *mut bpf_local_storage_data;
    pub fn bpf_local_storage_map_mem_usage(map: *const bpf_map) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
