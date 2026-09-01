/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * Generic non-thread safe hash map implementation.
 *
 * Copyright (c) 2019 Facebook
 */

/* C dependencies: <stdbool.h>, <stddef.h>, <limits.h> */

use core::ffi::{c_char, c_long, c_void};

pub type size_t = usize;

#[inline]
pub unsafe fn hash_bits(h: size_t, bits: i32) -> size_t {
    /* shuffle bits and return requested number of upper bits */
    if bits == 0 {
        return 0;
    }

    /*
     * C selects the multiplier from __SIZEOF_SIZE_T__ and shifts by the
     * corresponding integer width.
     */
    #[cfg(target_pointer_width = "64")]
    {
        return h.wrapping_mul(11400714819323198485usize) >> (64 - bits);
    }

    #[cfg(target_pointer_width = "32")]
    {
        return h.wrapping_mul(2654435769usize) >> (32 - bits);
    }

    #[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
    {
        compile_error!("Unsupported size_t size");
    }
}

/* generic C-string hashing function */
#[inline]
pub unsafe fn str_hash(mut s: *const c_char) -> size_t {
    let mut h: size_t = 0;

    while unsafe { *s } != 0 {
        h = h
            .wrapping_mul(31)
            .wrapping_add(unsafe { *s } as c_long as size_t);
        s = unsafe { s.add(1) };
    }
    h
}

pub type hashmap_hash_fn = Option<unsafe extern "C" fn(key: c_long, ctx: *mut c_void) -> size_t>;
pub type hashmap_equal_fn =
    Option<unsafe extern "C" fn(key1: c_long, key2: c_long, ctx: *mut c_void) -> bool>;

/*
 * Hashmap interface is polymorphic, keys and values could be either
 * long-sized integers or pointers, this is achieved as follows:
 * - interface functions that operate on keys and values are hidden
 *   behind auxiliary macros, e.g. hashmap_insert <-> hashmap__insert;
 * - these auxiliary macros cast the key and value parameters as
 *   long or long *, so the user does not have to specify the casts explicitly;
 * - for pointer parameters (e.g. old_key) the size of the pointed
 *   type is verified by hashmap_cast_ptr using _Static_assert;
 * - when iterating using hashmap__for_each_* forms
 *   hasmap_entry->key should be used for integer keys and
 *   hasmap_entry->pkey should be used for pointer keys,
 *   same goes for values.
 */
#[repr(C)]
pub union hashmap_entry_key {
    pub key: c_long,
    pub pkey: *const c_void,
}

#[repr(C)]
pub union hashmap_entry_value {
    pub value: c_long,
    pub pvalue: *mut c_void,
}

#[repr(C)]
pub struct hashmap_entry {
    pub key: hashmap_entry_key,
    pub value: hashmap_entry_value,
    pub next: *mut hashmap_entry,
}

#[repr(C)]
pub struct hashmap {
    pub hash_fn: hashmap_hash_fn,
    pub equal_fn: hashmap_equal_fn,
    pub ctx: *mut c_void,

    pub buckets: *mut *mut hashmap_entry,
    pub cap: size_t,
    pub cap_bits: size_t,
    pub sz: size_t,
}

unsafe extern "C" {
    pub fn hashmap__init(
        map: *mut hashmap,
        hash_fn: hashmap_hash_fn,
        equal_fn: hashmap_equal_fn,
        ctx: *mut c_void,
    );
    pub fn hashmap__new(
        hash_fn: hashmap_hash_fn,
        equal_fn: hashmap_equal_fn,
        ctx: *mut c_void,
    ) -> *mut hashmap;
    pub fn hashmap__clear(map: *mut hashmap);
    pub fn hashmap__free(map: *mut hashmap);

    pub fn hashmap__size(map: *const hashmap) -> size_t;
    pub fn hashmap__capacity(map: *const hashmap) -> size_t;
}

/*
 * Hashmap insertion strategy:
 * - HASHMAP_ADD - only add key/value if key doesn't exist yet;
 * - HASHMAP_SET - add key/value pair if key doesn't exist yet; otherwise,
 *   update value;
 * - HASHMAP_UPDATE - update value, if key already exists; otherwise, do
 *   nothing and return -ENOENT;
 * - HASHMAP_APPEND - always add key/value pair, even if key already exists.
 *   This turns hashmap into a multimap by allowing multiple values to be
 *   associated with the same key. Most useful read API for such hashmap is
 *   hashmap__for_each_key_entry() iteration. If hashmap__find() is still
 *   used, it will return last inserted key/value entry (first in a bucket
 *   chain).
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hashmap_insert_strategy {
    HASHMAP_ADD = 0,
    HASHMAP_SET = 1,
    HASHMAP_UPDATE = 2,
    HASHMAP_APPEND = 3,
}

/*
 * hashmap_cast_ptr(p) used _Static_assert to ensure the pointee is long-sized
 * or that p is a constant NULL, then cast p to long *.
 */
#[inline]
pub unsafe fn hashmap_cast_ptr<T>(p: *mut T) -> *mut c_long {
    p as *mut c_long
}

/*
 * hashmap__insert() adds key/value entry w/ various semantics, depending on
 * provided strategy value. If a given key/value pair replaced already
 * existing key/value pair, both old key and old value will be returned
 * through old_key and old_value to allow calling code do proper memory
 * management.
 */
unsafe extern "C" {
    pub fn hashmap_insert(
        map: *mut hashmap,
        key: c_long,
        value: c_long,
        strategy: hashmap_insert_strategy,
        old_key: *mut c_long,
        old_value: *mut c_long,
    ) -> i32;
}

#[inline]
pub unsafe fn hashmap__insert(
    map: *mut hashmap,
    key: c_long,
    value: c_long,
    strategy: hashmap_insert_strategy,
    old_key: *mut c_long,
    old_value: *mut c_long,
) -> i32 {
    unsafe { hashmap_insert(map, key, value, strategy, old_key, old_value) }
}

#[inline]
pub unsafe fn hashmap__add(map: *mut hashmap, key: c_long, value: c_long) -> i32 {
    unsafe {
        hashmap__insert(
            map,
            key,
            value,
            hashmap_insert_strategy::HASHMAP_ADD,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    }
}

#[inline]
pub unsafe fn hashmap__set(
    map: *mut hashmap,
    key: c_long,
    value: c_long,
    old_key: *mut c_long,
    old_value: *mut c_long,
) -> i32 {
    unsafe {
        hashmap__insert(
            map,
            key,
            value,
            hashmap_insert_strategy::HASHMAP_SET,
            old_key,
            old_value,
        )
    }
}

#[inline]
pub unsafe fn hashmap__update(
    map: *mut hashmap,
    key: c_long,
    value: c_long,
    old_key: *mut c_long,
    old_value: *mut c_long,
) -> i32 {
    unsafe {
        hashmap__insert(
            map,
            key,
            value,
            hashmap_insert_strategy::HASHMAP_UPDATE,
            old_key,
            old_value,
        )
    }
}

#[inline]
pub unsafe fn hashmap__append(map: *mut hashmap, key: c_long, value: c_long) -> i32 {
    unsafe {
        hashmap__insert(
            map,
            key,
            value,
            hashmap_insert_strategy::HASHMAP_APPEND,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    }
}

unsafe extern "C" {
    pub fn hashmap_delete(
        map: *mut hashmap,
        key: c_long,
        old_key: *mut c_long,
        old_value: *mut c_long,
    ) -> bool;
}

#[inline]
pub unsafe fn hashmap__delete(
    map: *mut hashmap,
    key: c_long,
    old_key: *mut c_long,
    old_value: *mut c_long,
) -> bool {
    unsafe { hashmap_delete(map, key, old_key, old_value) }
}

unsafe extern "C" {
    pub fn hashmap_find(map: *const hashmap, key: c_long, value: *mut c_long) -> bool;
}

#[inline]
pub unsafe fn hashmap__find(map: *const hashmap, key: c_long, value: *mut c_long) -> bool {
    unsafe { hashmap_find(map, key, value) }
}

/*
 * hashmap__for_each_entry - iterate over all entries in hashmap
 * @map: hashmap to iterate
 * @cur: struct hashmap_entry * used as a loop cursor
 * @bkt: integer used as a bucket loop cursor
 */
/* C macro:
 * for (bkt = 0; bkt < (map)->cap; bkt++)
 *     for (cur = (map)->buckets[bkt]; cur; cur = cur->next)
 */

/*
 * hashmap__for_each_entry_safe - iterate over all entries in hashmap, safe
 * against removals
 * @map: hashmap to iterate
 * @cur: struct hashmap_entry * used as a loop cursor
 * @tmp: struct hashmap_entry * used as a temporary next cursor storage
 * @bkt: integer used as a bucket loop cursor
 */
/* C macro:
 * for (bkt = 0; bkt < (map)->cap; bkt++)
 *     for (cur = (map)->buckets[bkt];
 *          cur && ({tmp = cur->next; true; });
 *          cur = tmp)
 */

/*
 * hashmap__for_each_key_entry - iterate over entries associated with given key
 * @map: hashmap to iterate
 * @cur: struct hashmap_entry * used as a loop cursor
 * @key: key to iterate entries for
 */
/* C macro:
 * for (cur = (map)->buckets
 *              ? (map)->buckets[hash_bits((map)->hash_fn((_key), (map)->ctx), (map)->cap_bits)]
 *              : NULL;
 *      cur;
 *      cur = cur->next)
 *     if ((map)->equal_fn(cur->key, (_key), (map)->ctx))
 */

/* C macro:
 * hashmap__for_each_key_entry_safe(map, cur, tmp, _key)
 * stores cur->next in tmp before each body execution and filters entries with
 * equal_fn(cur->key, _key, ctx).
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
