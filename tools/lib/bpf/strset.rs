// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2021 Facebook */
/* Translated from lib/bpf/strset.c. */
/* Dependencies in the original C source:
 * <stdint.h>, <stdlib.h>, <stdio.h>, <errno.h>, <linux/err.h>,
 * "hashmap.h", "libbpf_internal.h", "strset.h"
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

type size_t = usize;
type uintptr_t = usize;

const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EEXIST: c_int = 17;
const HASHMAP_ADD: c_int = 0;

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strset {
    strs_data: *mut c_void,
    strs_data_len: size_t,
    strs_data_cap: size_t,
    strs_data_max_len: size_t,

    /* lookup index for each unique string in strings set */
    strs_hash: *mut hashmap,
}

unsafe extern "C" {
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn str_hash(str_: *const c_char) -> size_t;
    fn hashmap__new(
        hash_fn: Option<unsafe extern "C" fn(c_long, *mut c_void) -> size_t>,
        equal_fn: Option<unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool>,
        ctx: *mut c_void,
    ) -> *mut hashmap;
    fn hashmap__add(map: *mut hashmap, key: c_long, value: c_long) -> c_int;
    fn hashmap__free(map: *mut hashmap);
    fn hashmap__find(map: *mut hashmap, key: c_long, value: *mut c_long) -> bool;
    fn hashmap__insert(
        map: *mut hashmap,
        key: c_long,
        value: c_long,
        strategy: c_int,
        old_key: *mut c_long,
        old_value: *mut c_long,
    ) -> c_int;
    fn libbpf_add_mem(
        data: *mut *mut c_void,
        cap_cnt: *mut size_t,
        elem_sz: size_t,
        cur_cnt: size_t,
        max_cnt: size_t,
        add_cnt: size_t,
    ) -> *mut c_void;
}

#[inline]
unsafe fn ERR_PTR(err: c_int) -> *mut strset {
    err as isize as *mut strset
}

#[inline]
unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as usize) >= usize::MAX - 4095
}

#[inline]
unsafe fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool {
    ptr.is_null() || unsafe { IS_ERR(ptr) }
}

unsafe extern "C" fn strset_hash_fn(key: c_long, ctx: *mut c_void) -> size_t {
    let s: *const strset = ctx as *const strset;
    let str_: *const c_char = unsafe { (*s).strs_data.cast::<c_char>().offset(key as isize) };

    unsafe { str_hash(str_) }
}

unsafe extern "C" fn strset_equal_fn(key1: c_long, key2: c_long, ctx: *mut c_void) -> bool {
    let s: *const strset = ctx as *const strset;
    let str1: *const c_char = unsafe { (*s).strs_data.cast::<c_char>().offset(key1 as isize) };
    let str2: *const c_char = unsafe { (*s).strs_data.cast::<c_char>().offset(key2 as isize) };

    unsafe { strcmp(str1, str2) == 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strset__new(
    max_data_sz: size_t,
    init_data: *const c_char,
    init_data_sz: size_t,
) -> *mut strset {
    let set: *mut strset = unsafe { calloc(1, core::mem::size_of::<strset>()) as *mut strset };
    let hash: *mut hashmap;
    let mut err: c_int = -ENOMEM;

    if set.is_null() {
        return unsafe { ERR_PTR(-ENOMEM) };
    }

    hash = unsafe {
        hashmap__new(
            Some(strset_hash_fn),
            Some(strset_equal_fn),
            set as *mut c_void,
        )
    };
    if unsafe { IS_ERR(hash as *const c_void) } {
        goto_err_out(set, err);
        return unsafe { ERR_PTR(err) };
    }

    unsafe {
        (*set).strs_data_max_len = max_data_sz;
        (*set).strs_hash = hash;
    }

    if !init_data.is_null() {
        let mut off: c_long;

        unsafe {
            (*set).strs_data = malloc(init_data_sz);
        }
        if unsafe { (*set).strs_data.is_null() } {
            goto_err_out(set, err);
            return unsafe { ERR_PTR(err) };
        }

        unsafe {
            memcpy((*set).strs_data, init_data as *const c_void, init_data_sz);
            (*set).strs_data_len = init_data_sz;
            (*set).strs_data_cap = init_data_sz;
        }

        off = 0;
        while (off as size_t) < unsafe { (*set).strs_data_len } {
            /* hashmap__add() returns EEXIST if string with the same
             * content already is in the hash map
             */
            err = unsafe { hashmap__add(hash, off, off) };
            if err == -EEXIST {
                off += unsafe {
                    strlen((*set).strs_data.cast::<c_char>().offset(off as isize)) as c_long + 1
                };
                continue; /* duplicate */
            }
            if err != 0 {
                goto_err_out(set, err);
                return unsafe { ERR_PTR(err) };
            }
            off += unsafe {
                strlen((*set).strs_data.cast::<c_char>().offset(off as isize)) as c_long + 1
            };
        }
    }

    return set;
}

unsafe fn goto_err_out(set: *mut strset, _err: c_int) {
    unsafe {
        strset__free(set);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strset__free(set: *mut strset) {
    if unsafe { IS_ERR_OR_NULL(set as *const c_void) } {
        return;
    }

    unsafe {
        hashmap__free((*set).strs_hash);
        free((*set).strs_data);
        free(set as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strset__data_size(set: *const strset) -> size_t {
    unsafe { (*set).strs_data_len }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strset__data(set: *const strset) -> *const c_char {
    unsafe { (*set).strs_data as *const c_char }
}

unsafe fn strset_add_str_mem(set: *mut strset, add_sz: size_t) -> *mut c_void {
    unsafe {
        libbpf_add_mem(
            &mut (*set).strs_data,
            &mut (*set).strs_data_cap,
            1,
            (*set).strs_data_len,
            (*set).strs_data_max_len,
            add_sz,
        )
    }
}

unsafe fn strset_str_append(set: *mut strset, mut s: *const c_char) -> c_long {
    let old_data: uintptr_t = unsafe { (*set).strs_data as uintptr_t };
    let old_data_len: size_t = unsafe { (*set).strs_data_len };
    let old_s: uintptr_t = s as uintptr_t;
    let len: c_long = unsafe { strlen(s) as c_long + 1 };
    let p: *mut c_void;

    /*
     * Hashmap keys are always offsets within set->strs_data, so to even
     * look up some string from the "outside", we need to first append it
     * at the end, so that it can be addressed with an offset. Luckily,
     * until set->strs_data_len is incremented, that string is just a piece
     * of garbage for the rest of the code, so no harm, no foul. On the
     * other hand, if the string is unique, it's already appended and
     * ready to be used, only a simple set->strs_data_len increment away.
     */
    p = unsafe { strset_add_str_mem(set, len as size_t) };
    if p.is_null() {
        return -ENOMEM as c_long;
    }

    /*
     * The set->strs_data might have reallocated and if 's' pointed
     * to an internal string within the old buffer, then it became
     * dangling and needs to be reconstructed before the copy.
     */
    if old_data != 0
        && old_data != unsafe { (*set).strs_data as uintptr_t }
        && old_s >= old_data
        && old_s < old_data + old_data_len
    {
        s = unsafe {
            (*set)
                .strs_data
                .cast::<c_char>()
                .add(old_s.wrapping_sub(old_data))
        };
    }

    unsafe {
        memcpy(p, s as *const c_void, len as size_t);
    }

    return len;
}

/* Find string offset that corresponds to a given string *s*.
 * Returns:
 *   - >0 offset into string data, if string is found;
 *   - -ENOENT, if string is not in the string data;
 *   - <0, on any other error.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strset__find_str(set: *mut strset, s: *const c_char) -> c_int {
    let mut old_off: c_long = 0;
    let new_off: c_long;
    let len: c_long;

    len = unsafe { strset_str_append(set, s) };
    if len < 0 {
        return len as c_int;
    }

    new_off = unsafe { (*set).strs_data_len as c_long };

    if unsafe { hashmap__find((*set).strs_hash, new_off, &mut old_off) } {
        return old_off as c_int;
    }

    return -ENOENT;
}

/* Add a string s to the string data. If the string already exists, return its
 * offset within string data.
 * Returns:
 *   - > 0 offset into string data, on success;
 *   - < 0, on error.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strset__add_str(set: *mut strset, s: *const c_char) -> c_int {
    let mut old_off: c_long = 0;
    let new_off: c_long;
    let len: c_long;
    let mut err: c_int;

    len = unsafe { strset_str_append(set, s) };
    if len < 0 {
        return len as c_int;
    }

    new_off = unsafe { (*set).strs_data_len as c_long };

    /* Now attempt to add the string, but only if the string with the same
     * contents doesn't exist already (HASHMAP_ADD strategy). If such
     * string exists, we'll get its offset in old_off (that's old_key).
     */
    err = unsafe {
        hashmap__insert(
            (*set).strs_hash,
            new_off,
            new_off,
            HASHMAP_ADD,
            &mut old_off,
            ptr::null_mut(),
        )
    };
    if err == -EEXIST {
        return old_off as c_int; /* duplicated string, return existing offset */
    }
    if err != 0 {
        return err;
    }

    unsafe {
        (*set).strs_data_len += len as size_t;
    } /* new unique string, adjust data length */
    return new_off as c_int;
}
