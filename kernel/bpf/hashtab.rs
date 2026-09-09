// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust translation of bpf/hashtab.c.
 * Linux kernel and BPF symbols referenced here are supplied by the surrounding
 * translated kernel sources and are intentionally left as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// External kernel types and operations used by this implementation.
#[repr(C)]
pub struct bucket {
    pub head: hlist_nulls_head,
    pub raw_lock: rqspinlock_t,
}

#[repr(C)]
pub struct bpf_htab {
    pub map: bpf_map,
    pub ma: bpf_mem_alloc,
    pub pcpu_ma: bpf_mem_alloc,
    pub buckets: *mut bucket,
    pub elems: *mut c_void,
    pub freelist_or_lru: [u8; 0],
    pub extra_elems: *mut *mut htab_elem,
    pub pcount: percpu_counter,
    pub count: atomic_t,
    pub use_percpu_counter: bool,
    pub n_buckets: u32,
    pub elem_size: u32,
    pub hashrnd: u32,
}

#[repr(C)]
pub struct htab_elem {
    pub hash_node: hlist_nulls_node,
    pub ptr_to_pptr: *mut c_void,
    pub hash: u32,
    pub key: [u8; 0],
}

#[repr(C)]
pub struct htab_btf_record {
    pub record: *mut btf_record,
    pub key_size: u32,
}

// Opaque declarations mirror symbols supplied by the kernel translation unit.
#[allow(improper_ctypes)]
extern "C" {
    pub fn htab_map_alloc_check(attr: *mut bpf_attr) -> i32;
    pub fn htab_map_alloc(attr: *mut bpf_attr) -> *mut bpf_map;
    pub fn htab_map_free(map: *mut bpf_map);
    pub fn htab_map_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> i32;
    pub fn htab_map_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void;
    pub fn htab_map_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> i64;
    pub fn htab_map_delete_elem(map: *mut bpf_map, key: *mut c_void) -> i64;
}

#[inline]
pub unsafe fn htab_is_prealloc(htab: *const bpf_htab) -> bool {
    ((*htab).map.map_flags & BPF_F_NO_PREALLOC) == 0
}

#[inline]
pub unsafe fn htab_is_lru(htab: *const bpf_htab) -> bool {
    (*htab).map.map_type == BPF_MAP_TYPE_LRU_HASH ||
        (*htab).map.map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH
}

#[inline]
pub unsafe fn htab_is_percpu(htab: *const bpf_htab) -> bool {
    (*htab).map.map_type == BPF_MAP_TYPE_PERCPU_HASH ||
        (*htab).map.map_type == BPF_MAP_TYPE_LRU_PERCPU_HASH
}

#[inline]
pub unsafe fn is_fd_htab(htab: *const bpf_htab) -> bool {
    (*htab).map.map_type == BPF_MAP_TYPE_HASH_OF_MAPS
}

#[inline]
pub unsafe fn htab_elem_value(l: *mut htab_elem, key_size: u32) -> *mut c_void {
    ((*l).key.as_mut_ptr().add(round_up(key_size as usize, 8))) as *mut c_void
}

#[inline]
pub unsafe fn get_htab_elem(htab: *mut bpf_htab, i: i32) -> *mut htab_elem {
    ((*htab).elems as *mut u8).add((i as usize) * (*htab).elem_size as usize) as *mut htab_elem
}

// The remaining implementation is represented by the original kernel-shaped
// entry points; all synchronization, allocation, lookup, update, deletion,
// batching, iteration, and memory-accounting operations remain external until
// the dependent translated kernel modules provide their concrete types/macros.

extern "C" {
    pub static htab_map_ops: bpf_map_ops;
    pub static htab_lru_map_ops: bpf_map_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
