// SPDX-License-Identifier: GPL-2.0
/*
 * Implementation of the hash table type.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;

const GFP_KERNEL: core::ffi::c_uint = 0;
const __GFP_NOWARN: core::ffi::c_uint = 0;
const SLAB_PANIC: core::ffi::c_uint = 0;
const ENOMEM: core::ffi::c_int = 12;

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashtab_node {
    pub key: *mut c_void,
    pub datum: *mut c_void,
    pub next: *mut hashtab_node,
}

#[repr(C)]
pub struct hashtab {
    pub htable: *mut *mut hashtab_node,
    pub size: u32,
    pub nel: u32,
}

#[repr(C)]
pub struct hashtab_info {
    pub slots_used: u32,
    pub max_chain_len: u32,
    pub chain2_len_sum: u64,
}

unsafe extern "C" {
    fn roundup_pow_of_two(n: u32) -> u32;
    fn kzalloc(size: usize, flags: core::ffi::c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kmem_cache_zalloc(cachep: *mut kmem_cache, flags: core::ffi::c_uint) -> *mut c_void;
    fn kmem_cache_free(cachep: *mut kmem_cache, objp: *mut c_void);
    fn kmem_cache_create(
        name: *const core::ffi::c_char,
        size: usize,
        align: usize,
        flags: core::ffi::c_uint,
        ctor: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> *mut kmem_cache;
}

static mut hashtab_node_cachep: *mut kmem_cache = ptr::null_mut();

unsafe fn kzalloc_objs_hashtab_node_ptr(n: u32, flags: core::ffi::c_uint) -> *mut *mut hashtab_node {
    unsafe { kzalloc((n as usize).wrapping_mul(size_of::<*mut hashtab_node>()), flags) as *mut *mut hashtab_node }
}

/*
 * Here we simply round the number of elements up to the nearest power of two.
 * I tried also other options like rounding down or rounding to the closest
 * power of two (up or down based on which is closer), but I was unable to
 * find any significant difference in lookup/insert performance that would
 * justify switching to a different (less intuitive) formula. It could be that
 * a different formula is actually more optimal, but any future changes here
 * should be supported with performance/memory usage data.
 *
 * The total memory used by the htable arrays (only) with Fedora policy loaded
 * is approximately 163 KB at the time of writing.
 */
unsafe fn hashtab_compute_size(nel: u32) -> u32 {
    if nel == 0 {
        0
    } else {
        unsafe { roundup_pow_of_two(nel) }
    }
}

#[no_mangle]
pub unsafe extern "C" fn hashtab_init(h: *mut hashtab, nel_hint: u32) -> core::ffi::c_int {
    let size: u32 = unsafe { hashtab_compute_size(nel_hint) };

    /* should already be zeroed, but better be safe */
    unsafe {
        (*h).nel = 0;
        (*h).size = 0;
        (*h).htable = ptr::null_mut();

        if size != 0 {
            (*h).htable = kzalloc_objs_hashtab_node_ptr(size, GFP_KERNEL | __GFP_NOWARN);
            if (*h).htable.is_null() {
                return -ENOMEM;
            }
            (*h).size = size;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn __hashtab_insert(
    h: *mut hashtab,
    dst: *mut *mut hashtab_node,
    key: *mut c_void,
    datum: *mut c_void,
) -> core::ffi::c_int {
    let newnode: *mut hashtab_node;

    unsafe {
        newnode = kmem_cache_zalloc(hashtab_node_cachep, GFP_KERNEL) as *mut hashtab_node;
        if newnode.is_null() {
            return -ENOMEM;
        }
        (*newnode).key = key;
        (*newnode).datum = datum;
        (*newnode).next = *dst;
        *dst = newnode;

        (*h).nel = (*h).nel.wrapping_add(1);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn hashtab_destroy(h: *mut hashtab) {
    let mut i: u32;
    let mut cur: *mut hashtab_node;
    let mut temp: *mut hashtab_node;

    unsafe {
        i = 0;
        while i < (*h).size {
            cur = *(*h).htable.add(i as usize);
            while !cur.is_null() {
                temp = cur;
                cur = (*cur).next;
                kmem_cache_free(hashtab_node_cachep, temp as *mut c_void);
            }
            *(*h).htable.add(i as usize) = ptr::null_mut();
            i = i.wrapping_add(1);
        }

        kfree((*h).htable as *mut c_void);
        (*h).htable = ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn hashtab_map(
    h: *mut hashtab,
    apply: Option<unsafe extern "C" fn(k: *mut c_void, d: *mut c_void, args: *mut c_void) -> core::ffi::c_int>,
    args: *mut c_void,
) -> core::ffi::c_int {
    let mut i: u32;
    let mut ret: core::ffi::c_int;
    let mut cur: *mut hashtab_node;

    unsafe {
        i = 0;
        while i < (*h).size {
            cur = *(*h).htable.add(i as usize);
            while !cur.is_null() {
                ret = apply.unwrap()((*cur).key, (*cur).datum, args);
                if ret != 0 {
                    return ret;
                }
                cur = (*cur).next;
            }
            i = i.wrapping_add(1);
        }
    }
    0
}

/*
 * CONFIG_SECURITY_SELINUX_DEBUG:
 * The following function is conditionally compiled in C under this option.
 */
#[cfg(CONFIG_SECURITY_SELINUX_DEBUG)]
#[no_mangle]
pub unsafe extern "C" fn hashtab_stat(h: *mut hashtab, info: *mut hashtab_info) {
    let mut i: u32;
    let mut chain_len: u32;
    let mut slots_used: u32;
    let mut max_chain_len: u32;
    let mut chain2_len_sum: u64;
    let mut cur: *mut hashtab_node;

    unsafe {
        slots_used = 0;
        max_chain_len = 0;
        chain2_len_sum = 0;
        i = 0;
        while i < (*h).size {
            cur = *(*h).htable.add(i as usize);
            if !cur.is_null() {
                slots_used = slots_used.wrapping_add(1);
                chain_len = 0;
                while !cur.is_null() {
                    chain_len = chain_len.wrapping_add(1);
                    cur = (*cur).next;
                }

                if chain_len > max_chain_len {
                    max_chain_len = chain_len;
                }

                chain2_len_sum = chain2_len_sum
                    .wrapping_add((chain_len as u64).wrapping_mul(chain_len as u64));
            }
            i = i.wrapping_add(1);
        }

        (*info).slots_used = slots_used;
        (*info).max_chain_len = max_chain_len;
        (*info).chain2_len_sum = chain2_len_sum;
    }
}

#[no_mangle]
pub unsafe extern "C" fn hashtab_duplicate(
    new: *mut hashtab,
    orig: *const hashtab,
    copy: Option<
        unsafe extern "C" fn(
            new: *mut hashtab_node,
            orig: *const hashtab_node,
            args: *mut c_void,
        ) -> core::ffi::c_int,
    >,
    destroy: Option<unsafe extern "C" fn(k: *mut c_void, d: *mut c_void, args: *mut c_void) -> core::ffi::c_int>,
    args: *mut c_void,
) -> core::ffi::c_int {
    let mut orig_cur: *const hashtab_node;
    let mut cur: *mut hashtab_node;
    let mut tmp: *mut hashtab_node;
    let mut tail: *mut hashtab_node;
    let mut i: u32;
    let rc: core::ffi::c_int;

    unsafe {
        ptr::write_bytes(new, 0, 1);

        (*new).htable = kzalloc_objs_hashtab_node_ptr((*orig).size, GFP_KERNEL);
        if (*new).htable.is_null() {
            return -ENOMEM;
        }

        (*new).size = (*orig).size;

        i = 0;
        while i < (*orig).size {
            tail = ptr::null_mut();
            orig_cur = *(*orig).htable.add(i as usize) as *const hashtab_node;
            while !orig_cur.is_null() {
                tmp = kmem_cache_zalloc(hashtab_node_cachep, GFP_KERNEL) as *mut hashtab_node;
                if tmp.is_null() {
                    return hashtab_duplicate_error(new, destroy, args);
                }
                rc = copy.unwrap()(tmp, orig_cur, args);
                if rc != 0 {
                    kmem_cache_free(hashtab_node_cachep, tmp as *mut c_void);
                    return hashtab_duplicate_error(new, destroy, args);
                }
                (*tmp).next = ptr::null_mut();
                if tail.is_null() {
                    *(*new).htable.add(i as usize) = tmp;
                } else {
                    (*tail).next = tmp;
                }
                tail = tmp;
                (*new).nel = (*new).nel.wrapping_add(1);
                orig_cur = (*orig_cur).next;
            }
            i = i.wrapping_add(1);
        }
    }

    0
}

unsafe fn hashtab_duplicate_error(
    new: *mut hashtab,
    destroy: Option<unsafe extern "C" fn(k: *mut c_void, d: *mut c_void, args: *mut c_void) -> core::ffi::c_int>,
    args: *mut c_void,
) -> core::ffi::c_int {
    let mut i: u32;
    let mut cur: *mut hashtab_node;
    let mut tmp: *mut hashtab_node;

    unsafe {
        i = 0;
        while i < (*new).size {
            cur = *(*new).htable.add(i as usize);
            while !cur.is_null() {
                tmp = (*cur).next;
                destroy.unwrap()((*cur).key, (*cur).datum, args);
                kmem_cache_free(hashtab_node_cachep, cur as *mut c_void);
                cur = tmp;
            }
            i = i.wrapping_add(1);
        }
        kfree((*new).htable as *mut c_void);
        ptr::write_bytes(new, 0, 1);
    }
    -ENOMEM
}

#[no_mangle]
pub unsafe extern "C" fn hashtab_cache_init() {
    unsafe {
        hashtab_node_cachep = kmem_cache_create(
            c"hashtab_node".as_ptr(),
            size_of::<hashtab_node>(),
            0,
            SLAB_PANIC,
            None,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
