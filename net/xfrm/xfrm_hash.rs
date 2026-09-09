// SPDX-License-Identifier: GPL-2.0
/* xfrm_hash.c: Common hash table code.
 *
 * Copyright (C) 2006 David S. Miller (davem@davemloft.net)
 */

// The Linux kernel headers and "xfrm_hash.h" provide the types, constants,
// globals, and allocation routines referenced below.

extern "C" {
    static hashdist: bool;

    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn vzalloc(size: usize) -> *mut core::ffi::c_void;
    fn __get_free_pages(flags: u32, order: u32) -> usize;
    fn get_order(size: usize) -> u32;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn vfree(ptr: *mut core::ffi::c_void);
    fn free_pages(addr: usize, order: u32);
}

#[repr(C)]
pub struct hlist_head {
    _private: [u8; 0],
}

// These values are supplied by the kernel build environment.
const PAGE_SIZE: usize = 4096;
const GFP_KERNEL: u32 = 0;
const __GFP_NOWARN: u32 = 0;
const __GFP_ZERO: u32 = 0;

pub unsafe fn xfrm_hash_alloc(sz: u32) -> *mut hlist_head {
    let n: *mut hlist_head;

    if sz as usize <= PAGE_SIZE {
        n = kzalloc(sz as usize, GFP_KERNEL) as *mut hlist_head;
    } else if hashdist {
        n = vzalloc(sz as usize) as *mut hlist_head;
    } else {
        n = __get_free_pages(
            GFP_KERNEL | __GFP_NOWARN | __GFP_ZERO,
            get_order(sz as usize),
        ) as *mut hlist_head;
    }

    n
}

pub unsafe fn xfrm_hash_free(n: *mut hlist_head, sz: u32) {
    if sz as usize <= PAGE_SIZE {
        kfree(n as *mut core::ffi::c_void);
    } else if hashdist {
        vfree(n as *mut core::ffi::c_void);
    } else {
        free_pages(n as usize, get_order(sz as usize));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
