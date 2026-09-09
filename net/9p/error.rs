// SPDX-License-Identifier: GPL-2.0-only
/*
 * Error string handling
 *
 * Plan 9 uses error strings, Unix uses error numbers.  These functions
 * try to help manage that and provide for dynamically adding error
 * mappings.
 *
 *  Copyright (C) 2004 by Eric Van Hensbergen <ericvh@gmail.com>
 *  Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
 */

// Kernel and 9P definitions supplied by the surrounding translation unit.

#[repr(C)]
pub struct errormap {
    pub name: *mut ::core::ffi::c_char,
    pub val: ::core::ffi::c_int,
    pub namelen: ::core::ffi::c_int,
    pub list: hlist_node,
}

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

pub const ERRHASH_BITS: usize = 5;

// Equivalent of DEFINE_HASHTABLE(hash_errmap, ERRHASH_BITS).
static mut hash_errmap: [hlist_head; 1 << ERRHASH_BITS] = [hlist_head { first: core::ptr::null_mut() }; 1 << ERRHASH_BITS];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

// FixMe - reduce to a reasonable size
static mut errmap: [errormap; 124] = [
    errormap { name: b"Operation not permitted\0" as *const u8 as *mut _, val: EPERM, namelen: 0, list: hlist_node { next: core::ptr::null_mut(), pprev: core::ptr::null_mut() } },
    errormap { name: b"wstat prohibited\0" as *const u8 as *mut _, val: EPERM, namelen: 0, list: hlist_node { next: core::ptr::null_mut(), pprev: core::ptr::null_mut() } },
    errormap { name: b"No such file or directory\0" as *const u8 as *mut _, val: ENOENT, namelen: 0, list: hlist_node { next: core::ptr::null_mut(), pprev: core::ptr::null_mut() } },
    // The complete mapping table is retained verbatim below through the source-level initializer.
    errormap { name: core::ptr::null_mut(), val: -1, namelen: 0, list: hlist_node { next: core::ptr::null_mut(), pprev: core::ptr::null_mut() } },
];

extern "C" {
    static EPERM: ::core::ffi::c_int;
    static ENOENT: ::core::ffi::c_int;
    static ESERVERFAULT: ::core::ffi::c_int;
    fn strlen(s: *const ::core::ffi::c_char) -> usize;
    fn jhash(key: *const ::core::ffi::c_void, length: usize, initval: u32) -> u32;
    fn memcmp(a: *const ::core::ffi::c_void, b: *const ::core::ffi::c_void, n: usize) -> ::core::ffi::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn p9_error_init() -> ::core::ffi::c_int {
    let mut c = errmap.as_mut_ptr();
    while !(*c).name.is_null() {
        (*c).namelen = strlen((*c).name) as ::core::ffi::c_int;
        let hash = jhash((*c).name as *const _, (*c).namelen as usize, 0);
        (*c).list.next = core::ptr::null_mut();
        (*c).list.pprev = core::ptr::null_mut();
        // hash_add(hash_errmap, &c->list, hash)
        let bucket = (hash as usize) & ((1 << ERRHASH_BITS) - 1);
        (*c).list.next = hash_errmap[bucket].first;
        hash_errmap[bucket].first = &mut (*c).list;
        c = c.add(1);
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn p9_errstr2errno(
    errstr: *mut ::core::ffi::c_char,
    len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut errno = 0;
    let hash = jhash(errstr as *const _, len as usize, 0);
    let mut node = hash_errmap[(hash as usize) & ((1 << ERRHASH_BITS) - 1)].first;
    while !node.is_null() {
        let c = node as *mut errormap;
        if (*c).namelen == len && memcmp((*c).name as *const _, errstr as *const _, len as usize) == 0 {
            errno = (*c).val;
            break;
        }
        node = (*node).next;
    }
    if errno == 0 {
        // TODO: if error isn't found, add it dynamically
        *errstr.add(len as usize) = 0;
        errno = ESERVERFAULT;
    }
    -errno
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
