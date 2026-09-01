/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/*
 * C dependencies removed from executable Rust:
 *   <errno.h>
 *   "bpf_arena_alloc.h"
 *   "bpf_arena_list.h"
 *
 * The __arena address-space qualifier, cast_kern/cast_user helpers, arena list
 * primitives, arena object, PAGE_SIZE, and NUMA_NO_NODE are supplied by those
 * dependencies in the original source.
 */

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

pub const ENOMEM: i32 = 12;

#[repr(C)]
pub struct htab_bucket {
    pub head: arena_list_head,
}

pub type htab_bucket_t = htab_bucket;

#[repr(C)]
pub struct htab {
    pub buckets: *mut htab_bucket_t,
    pub n_buckets: i32,
}

#[inline]
pub unsafe fn __select_bucket(htab: *mut htab, hash: __u32) -> *mut htab_bucket_t {
    let b: *mut htab_bucket_t = unsafe { (*htab).buckets };

    unsafe {
        cast_kern(b as *mut c_void);
        b.add((hash & (((*htab).n_buckets - 1) as __u32)) as usize)
    }
}

#[inline]
pub unsafe fn select_bucket(htab: *mut htab, hash: __u32) -> *mut arena_list_head_t {
    unsafe { ptr::addr_of_mut!((*__select_bucket(htab, hash)).head) }
}

#[repr(C)]
pub struct hashtab_elem {
    pub hash: i32,
    pub key: i32,
    pub value: i32,
    pub hash_node: arena_list_node,
}

pub type hashtab_elem_t = hashtab_elem;

pub unsafe fn lookup_elem_raw(
    head: *mut arena_list_head_t,
    hash: __u32,
    key: i32,
) -> *mut hashtab_elem_t {
    let mut l: *mut hashtab_elem_t;

    /*
     * Original C:
     *   list_for_each_entry(l, head, hash_node)
     *           if (l->hash == hash && l->key == key)
     *                   return l;
     *
     * list_for_each_entry is provided by bpf_arena_list.h. The Rust translation
     * keeps the same external list-iteration dependency.
     */
    l = unsafe { list_first_entry(head, hashtab_elem_hash_node_offset()) };
    while !l.is_null() {
        if unsafe { (*l).hash == hash as i32 && (*l).key == key } {
            return l;
        }
        l = unsafe { list_next_entry(l, head, hashtab_elem_hash_node_offset()) };
    }

    ptr::null_mut()
}

pub fn htab_hash(key: i32) -> i32 {
    key
}

#[no_mangle]
pub unsafe extern "C" fn htab_lookup_elem(htab: *mut htab, key: i32) -> i32 {
    let l_old: *mut hashtab_elem_t;
    let head: *mut arena_list_head_t;

    unsafe {
        cast_kern(htab as *mut c_void);
        head = select_bucket(htab, key as __u32);
        l_old = lookup_elem_raw(head, htab_hash(key) as __u32, key);
        if !l_old.is_null() {
            return (*l_old).value;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn htab_update_elem(htab: *mut htab, key: i32, value: i32) -> i32 {
    let mut l_new: *mut hashtab_elem_t = ptr::null_mut();
    let l_old: *mut hashtab_elem_t;
    let head: *mut arena_list_head_t;

    unsafe {
        cast_kern(htab as *mut c_void);
        head = select_bucket(htab, key as __u32);
        l_old = lookup_elem_raw(head, htab_hash(key) as __u32, key);

        l_new = bpf_alloc(size_of::<hashtab_elem_t>()) as *mut hashtab_elem_t;
        if l_new.is_null() {
            return -ENOMEM;
        }
        (*l_new).key = key;
        (*l_new).hash = htab_hash(key);
        (*l_new).value = value;

        list_add_head(ptr::addr_of_mut!((*l_new).hash_node), head);
        if !l_old.is_null() {
            list_del(ptr::addr_of_mut!((*l_old).hash_node));
            bpf_free(l_old as *mut c_void);
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn htab_init(htab: *mut htab) {
    let buckets: *mut c_void =
        unsafe { bpf_arena_alloc_pages(ptr::addr_of_mut!(arena), ptr::null_mut(), 2, NUMA_NO_NODE, 0) };

    unsafe {
        cast_user(buckets);
        (*htab).buckets = buckets as *mut htab_bucket_t;
        (*htab).n_buckets = (2 * PAGE_SIZE / size_of::<htab_bucket>() as i32) as i32;
    }
}

extern "C" {
    pub static mut arena: c_void;

    pub static PAGE_SIZE: i32;
    pub static NUMA_NO_NODE: i32;

    pub fn cast_kern(ptr: *mut c_void);
    pub fn cast_user(ptr: *mut c_void);

    pub fn bpf_alloc(size: usize) -> *mut c_void;
    pub fn bpf_free(ptr: *mut c_void);
    pub fn bpf_arena_alloc_pages(
        arena: *mut c_void,
        addr: *mut c_void,
        page_cnt: i32,
        node: i32,
        flags: i32,
    ) -> *mut c_void;

    pub fn list_add_head(node: *mut arena_list_node, head: *mut arena_list_head_t);
    pub fn list_del(node: *mut arena_list_node);

    /*
     * Rust stand-ins for bpf_arena_list.h's list_for_each_entry mechanics.
     * These represent the external macro behavior without implementing the
     * dependency in this isolated header translation.
     */
    pub fn list_first_entry(
        head: *mut arena_list_head_t,
        member_offset: usize,
    ) -> *mut hashtab_elem_t;
    pub fn list_next_entry(
        pos: *mut hashtab_elem_t,
        head: *mut arena_list_head_t,
        member_offset: usize,
    ) -> *mut hashtab_elem_t;
}

pub type __u32 = u32;
pub type arena_list_head_t = arena_list_head;

extern "C" {
    pub type arena_list_head;
    pub type arena_list_node;
}

#[inline]
pub const fn hashtab_elem_hash_node_offset() -> usize {
    let uninit = core::mem::MaybeUninit::<hashtab_elem>::uninit();
    let base = uninit.as_ptr();
    unsafe { (ptr::addr_of!((*base).hash_node) as usize).wrapping_sub(base as usize) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
