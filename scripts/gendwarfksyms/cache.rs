// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Google LLC
 */

// Dependency declarations supplied by gendwarfksyms.h are intentionally left
// external to this translation unit.

#[repr(C)]
pub struct cache_item {
    pub key: ::std::ffi::c_ulong,
    pub value: ::std::ffi::c_int,
    pub hash: hlist_node,
}

extern "C" {
    fn xmalloc(size: usize) -> *mut ::std::ffi::c_void;
    fn free(ptr: *mut ::std::ffi::c_void);

    fn hash_add(head: *mut ::std::ffi::c_void, node: *mut hlist_node, key: u32);
    fn hash_32(key: ::std::ffi::c_ulong) -> u32;
    fn hash_init(head: *mut ::std::ffi::c_void);
}

#[repr(C)]
pub struct hlist_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cache {
    pub cache: ::std::ffi::c_void,
}

pub unsafe fn cache_set(cache: *mut cache, key: ::std::ffi::c_ulong, value: ::std::ffi::c_int) {
    let ci = xmalloc(::std::mem::size_of::<cache_item>()) as *mut cache_item;
    (*ci).key = key;
    (*ci).value = value;
    hash_add(
        &mut (*cache).cache as *mut ::std::ffi::c_void,
        &mut (*ci).hash,
        hash_32(key),
    );
}

pub unsafe fn cache_get(cache: *mut cache, key: ::std::ffi::c_ulong) -> ::std::ffi::c_int {
    // Equivalent of hash_for_each_possible(cache->cache, ci, hash, hash_32(key)).
    // The hash-table iteration primitive is supplied by the external header.
    let mut ci: *mut cache_item = ::std::ptr::null_mut();
    while hash_for_each_possible_next(
        &mut (*cache).cache as *mut ::std::ffi::c_void,
        &mut ci,
        hash_32(key),
    ) {
        if (*ci).key == key {
            return (*ci).value;
        }
    }

    -1
}

pub unsafe fn cache_init(cache: *mut cache) {
    hash_init(&mut (*cache).cache as *mut ::std::ffi::c_void);
}

pub unsafe fn cache_free(cache: *mut cache) {
    let mut tmp: *mut hlist_node = ::std::ptr::null_mut();
    let mut ci: *mut cache_item = ::std::ptr::null_mut();

    // Equivalent of hash_for_each_safe(cache->cache, ci, tmp, hash).
    while hash_for_each_safe_next(
        &mut (*cache).cache as *mut ::std::ffi::c_void,
        &mut ci,
        &mut tmp,
    ) {
        free(ci as *mut ::std::ffi::c_void);
    }

    hash_init(&mut (*cache).cache as *mut ::std::ffi::c_void);
}

extern "C" {
    fn hash_for_each_possible_next(
        head: *mut ::std::ffi::c_void,
        item: *mut *mut cache_item,
        key: u32,
    ) -> bool;
    fn hash_for_each_safe_next(
        head: *mut ::std::ffi::c_void,
        item: *mut *mut cache_item,
        tmp: *mut *mut hlist_node,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
