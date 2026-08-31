// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * Generic non-thread safe hash map implementation.
 *
 * Copyright (c) 2019 Facebook
 */
/* Original C dependencies:
 * #include <stdint.h>
 * #include <stdlib.h>
 * #include <stdio.h>
 * #include <errno.h>
 * #include <linux/err.h>
 * #include "hashmap.h"
 */

/* make sure libbpf doesn't use kernel-only integer typedefs */
/* C-only: #pragma GCC poison u8 u16 u32 u64 s8 s16 s32 s64 */

/* prevent accidental re-addition of reallocarray() */
/* C-only: #pragma GCC poison reallocarray */

use core::ffi::{c_long, c_void};
use core::mem;
use core::ptr;

/* start with 4 buckets */
const HASHMAP_MIN_CAP_BITS: usize = 2;

const ENOMEM: i32 = 12;
const EEXIST: i32 = 17;
const ENOENT: i32 = 2;

pub type hashmap_hash_fn =
    Option<unsafe extern "C" fn(key: c_long, ctx: *mut c_void) -> usize>;
pub type hashmap_equal_fn =
    Option<unsafe extern "C" fn(a: c_long, b: c_long, ctx: *mut c_void) -> bool>;

#[repr(C)]
pub struct hashmap_entry {
    pub key: c_long,
    pub value: c_long,
    pub next: *mut hashmap_entry,
}

#[repr(C)]
pub struct hashmap {
    pub hash_fn: hashmap_hash_fn,
    pub equal_fn: hashmap_equal_fn,
    pub ctx: *mut c_void,
    pub buckets: *mut *mut hashmap_entry,
    pub cap: usize,
    pub cap_bits: usize,
    pub sz: usize,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hashmap_insert_strategy {
    HASHMAP_ADD,
    HASHMAP_SET,
    HASHMAP_UPDATE,
    HASHMAP_APPEND,
}

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn ERR_PTR(error: c_long) -> *mut hashmap;
    fn IS_ERR_OR_NULL(ptr: *const hashmap) -> bool;
    fn hash_bits(hash: usize, bits: usize) -> usize;
}

unsafe fn call_hash_fn(map: *const hashmap, key: c_long) -> usize {
    ((*map).hash_fn.expect("hashmap hash_fn is NULL"))(key, (*map).ctx)
}

unsafe fn call_equal_fn(map: *const hashmap, a: c_long, b: c_long) -> bool {
    ((*map).equal_fn.expect("hashmap equal_fn is NULL"))(a, b, (*map).ctx)
}

unsafe fn hashmap_add_entry(pprev: *mut *mut hashmap_entry, entry: *mut hashmap_entry) {
    (*entry).next = *pprev;
    *pprev = entry;
}

unsafe fn hashmap_del_entry(pprev: *mut *mut hashmap_entry, entry: *mut hashmap_entry) {
    *pprev = (*entry).next;
    (*entry).next = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn hashmap__init(
    map: *mut hashmap,
    hash_fn: hashmap_hash_fn,
    equal_fn: hashmap_equal_fn,
    ctx: *mut c_void,
) {
    (*map).hash_fn = hash_fn;
    (*map).equal_fn = equal_fn;
    (*map).ctx = ctx;

    (*map).buckets = ptr::null_mut();
    (*map).cap = 0;
    (*map).cap_bits = 0;
    (*map).sz = 0;
}

#[no_mangle]
pub unsafe extern "C" fn hashmap__new(
    hash_fn: hashmap_hash_fn,
    equal_fn: hashmap_equal_fn,
    ctx: *mut c_void,
) -> *mut hashmap {
    let map = malloc(mem::size_of::<hashmap>()) as *mut hashmap;

    if map.is_null() {
        return ERR_PTR(-(ENOMEM as c_long));
    }
    hashmap__init(map, hash_fn, equal_fn, ctx);
    map
}

#[no_mangle]
pub unsafe extern "C" fn hashmap__clear(map: *mut hashmap) {
    let mut bkt: usize = 0;

    while bkt < (*map).cap {
        let mut cur = *(*map).buckets.add(bkt);
        while !cur.is_null() {
            let tmp = (*cur).next;
            free(cur as *mut c_void);
            cur = tmp;
        }
        bkt += 1;
    }
    free((*map).buckets as *mut c_void);
    (*map).buckets = ptr::null_mut();
    (*map).cap = 0;
    (*map).cap_bits = 0;
    (*map).sz = 0;
}

#[no_mangle]
pub unsafe extern "C" fn hashmap__free(map: *mut hashmap) {
    if IS_ERR_OR_NULL(map) {
        return;
    }

    hashmap__clear(map);
    free(map as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn hashmap__size(map: *const hashmap) -> usize {
    (*map).sz
}

#[no_mangle]
pub unsafe extern "C" fn hashmap__capacity(map: *const hashmap) -> usize {
    (*map).cap
}

unsafe fn hashmap_needs_to_grow(map: *mut hashmap) -> bool {
    /* grow if empty or more than 75% filled */
    ((*map).cap == 0) || (((*map).sz + 1) * 4 / 3 > (*map).cap)
}

unsafe fn hashmap_grow(map: *mut hashmap) -> i32 {
    let mut cur: *mut hashmap_entry;
    let mut tmp: *mut hashmap_entry;
    let mut h: usize;
    let mut bkt: usize;

    let mut new_cap_bits = (*map).cap_bits + 1;
    if new_cap_bits < HASHMAP_MIN_CAP_BITS {
        new_cap_bits = HASHMAP_MIN_CAP_BITS;
    }

    let new_cap = 1usize << new_cap_bits;
    let new_buckets = calloc(new_cap, mem::size_of::<*mut hashmap_entry>())
        as *mut *mut hashmap_entry;
    if new_buckets.is_null() {
        return -ENOMEM;
    }

    bkt = 0;
    while bkt < (*map).cap {
        cur = *(*map).buckets.add(bkt);
        while !cur.is_null() {
            tmp = (*cur).next;
            h = hash_bits(call_hash_fn(map, (*cur).key), new_cap_bits);
            hashmap_add_entry(new_buckets.add(h), cur);
            cur = tmp;
        }
        bkt += 1;
    }

    (*map).cap = new_cap;
    (*map).cap_bits = new_cap_bits;
    free((*map).buckets as *mut c_void);
    (*map).buckets = new_buckets;

    0
}

unsafe fn hashmap_find_entry(
    map: *const hashmap,
    key: c_long,
    hash: usize,
    pprev: *mut *mut *mut hashmap_entry,
    entry: *mut *mut hashmap_entry,
) -> bool {
    let mut prev_ptr: *mut *mut hashmap_entry;
    let mut cur: *mut hashmap_entry;

    if (*map).buckets.is_null() {
        return false;
    }

    prev_ptr = (*map).buckets.add(hash);
    cur = *prev_ptr;
    while !cur.is_null() {
        if call_equal_fn(map, (*cur).key, key) {
            if !pprev.is_null() {
                *pprev = prev_ptr;
            }
            *entry = cur;
            return true;
        }
        prev_ptr = &mut (*cur).next;
        cur = (*cur).next;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn hashmap_insert(
    map: *mut hashmap,
    key: c_long,
    value: c_long,
    strategy: hashmap_insert_strategy,
    old_key: *mut c_long,
    old_value: *mut c_long,
) -> i32 {
    let mut entry: *mut hashmap_entry = ptr::null_mut();
    let mut h: usize;
    let err: i32;

    if !old_key.is_null() {
        *old_key = 0;
    }
    if !old_value.is_null() {
        *old_value = 0;
    }

    h = hash_bits(call_hash_fn(map, key), (*map).cap_bits);
    if strategy != hashmap_insert_strategy::HASHMAP_APPEND
        && hashmap_find_entry(map, key, h, ptr::null_mut(), &mut entry)
    {
        if !old_key.is_null() {
            *old_key = (*entry).key;
        }
        if !old_value.is_null() {
            *old_value = (*entry).value;
        }

        if strategy == hashmap_insert_strategy::HASHMAP_SET
            || strategy == hashmap_insert_strategy::HASHMAP_UPDATE
        {
            (*entry).key = key;
            (*entry).value = value;
            return 0;
        } else if strategy == hashmap_insert_strategy::HASHMAP_ADD {
            return -EEXIST;
        }
    }

    if strategy == hashmap_insert_strategy::HASHMAP_UPDATE {
        return -ENOENT;
    }

    if hashmap_needs_to_grow(map) {
        err = hashmap_grow(map);
        if err != 0 {
            return err;
        }
        h = hash_bits(call_hash_fn(map, key), (*map).cap_bits);
    }

    entry = malloc(mem::size_of::<hashmap_entry>()) as *mut hashmap_entry;
    if entry.is_null() {
        return -ENOMEM;
    }

    (*entry).key = key;
    (*entry).value = value;
    hashmap_add_entry((*map).buckets.add(h), entry);
    (*map).sz += 1;

    0
}

#[no_mangle]
pub unsafe extern "C" fn hashmap_find(
    map: *const hashmap,
    key: c_long,
    value: *mut c_long,
) -> bool {
    let mut entry: *mut hashmap_entry = ptr::null_mut();
    let h: usize;

    h = hash_bits(call_hash_fn(map, key), (*map).cap_bits);
    if !hashmap_find_entry(map, key, h, ptr::null_mut(), &mut entry) {
        return false;
    }

    if !value.is_null() {
        *value = (*entry).value;
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn hashmap_delete(
    map: *mut hashmap,
    key: c_long,
    old_key: *mut c_long,
    old_value: *mut c_long,
) -> bool {
    let mut pprev: *mut *mut hashmap_entry = ptr::null_mut();
    let mut entry: *mut hashmap_entry = ptr::null_mut();
    let h: usize;

    h = hash_bits(call_hash_fn(map, key), (*map).cap_bits);
    if !hashmap_find_entry(map, key, h, &mut pprev, &mut entry) {
        return false;
    }

    if !old_key.is_null() {
        *old_key = (*entry).key;
    }
    if !old_value.is_null() {
        *old_value = (*entry).value;
    }

    hashmap_del_entry(pprev, entry);
    free(entry as *mut c_void);
    (*map).sz -= 1;

    true
}
