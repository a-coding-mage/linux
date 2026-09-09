// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Simon Wunderlich, Marek Lindner
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, corresponding to the C includes.

use core::ffi::c_void;
use core::ptr;

use crate::{batadv_hashtable, lock_class_key};

extern "C" {
    fn kfree(ptr: *mut c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn init_hlist_head(head: *mut c_void);
    fn spin_lock_init(lock: *mut c_void);
    fn atomic_set(value: *mut c_void, number: i32);
    fn lockdep_set_class(lock: *mut c_void, key: *mut lock_class_key);
}

const GFP_ATOMIC: u32 = 0x20;

/**
 * batadv_hash_init() - clear all buckets of a hashtable
 * @hash: hashtable to clear
 */
unsafe fn batadv_hash_init(hash: *mut batadv_hashtable) {
    let mut i: u32 = 0;

    while i < (*hash).size {
        init_hlist_head((*hash).table.add(i as usize) as *mut c_void);
        spin_lock_init((*hash).list_locks.add(i as usize) as *mut c_void);
        i = i.wrapping_add(1);
    }

    atomic_set(&mut (*hash).generation as *mut _ as *mut c_void, 0);
}

/**
 * batadv_hash_destroy() - Free only the hashtable and the hash itself
 * @hash: hash object to destroy
 */
#[no_mangle]
pub unsafe extern "C" fn batadv_hash_destroy(hash: *mut batadv_hashtable) {
    kfree((*hash).list_locks as *mut c_void);
    kfree((*hash).table as *mut c_void);
    kfree(hash as *mut c_void);
}

/**
 * batadv_hash_new() - Allocates and clears the hashtable
 * @size: number of hash buckets to allocate
 *
 * Return: newly allocated hashtable, NULL on errors
 */
#[no_mangle]
pub unsafe extern "C" fn batadv_hash_new(size: u32) -> *mut batadv_hashtable {
    let hash = kmalloc(core::mem::size_of::<batadv_hashtable>(), GFP_ATOMIC)
        as *mut batadv_hashtable;
    if hash.is_null() {
        return ptr::null_mut();
    }

    (*hash).table = kmalloc(
        core::mem::size_of_val(&*(*hash).table).wrapping_mul(size as usize),
        GFP_ATOMIC,
    ) as _;
    if (*hash).table.is_null() {
        kfree(hash as *mut c_void);
        return ptr::null_mut();
    }

    (*hash).list_locks = kmalloc(
        core::mem::size_of_val(&*(*hash).list_locks).wrapping_mul(size as usize),
        GFP_ATOMIC,
    ) as _;
    if (*hash).list_locks.is_null() {
        kfree((*hash).table as *mut c_void);
        kfree(hash as *mut c_void);
        return ptr::null_mut();
    }

    (*hash).size = size;
    batadv_hash_init(hash);
    hash
}

/**
 * batadv_hash_set_lock_class() - Set specific lockdep class for hash spinlocks
 * @hash: hash object to modify
 * @key: lockdep class key address
 */
#[no_mangle]
pub unsafe extern "C" fn batadv_hash_set_lock_class(
    hash: *mut batadv_hashtable,
    key: *mut lock_class_key,
) {
    let mut i: u32 = 0;

    while i < (*hash).size {
        lockdep_set_class((*hash).list_locks.add(i as usize) as *mut c_void, key);
        i = i.wrapping_add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
