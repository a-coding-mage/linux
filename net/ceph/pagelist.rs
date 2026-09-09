// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

// Supplied by the Linux kernel and Ceph headers.
pub type gfp_t = u32;
pub type size_t = usize;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct page {
    pub lru: list_head,
}

#[repr(C)]
pub struct refcount_t {
    pub refs: i32,
}

#[repr(C)]
pub struct ceph_pagelist {
    pub head: list_head,
    pub mapped_tail: *mut u8,
    pub length: size_t,
    pub room: size_t,
    pub free_list: list_head,
    pub num_pages_free: size_t,
    pub refcnt: refcount_t,
}

extern "C" {
    fn kmalloc_obj(size: size_t, flags: gfp_t) -> *mut ceph_pagelist;
    fn init_list_head(head: *mut list_head);
    fn kunmap(page: *mut page);
    fn refcount_dec_and_test(refcnt: *mut refcount_t) -> bool;
    fn refcount_set(refcnt: *mut refcount_t, value: i32);
    fn list_empty(head: *const list_head) -> bool;
    fn list_del(entry: *mut list_head);
    fn __free_page(page: *mut page);
    fn __page_cache_alloc(flags: gfp_t) -> *mut page;
    fn kmap(page: *mut page) -> *mut u8;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn ceph_pagelist_free_reserve(pl: *mut ceph_pagelist) -> i32;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, len: size_t) -> *mut c_void;
    fn bug_on(condition: bool);
}

const GFP_NOFS: gfp_t = 0;
const PAGE_SIZE: size_t = 4096;
const PAGE_SHIFT: usize = 12;
const PAGE_MASK: size_t = !(PAGE_SIZE - 1);

#[inline]
unsafe fn list_entry_page(entry: *mut list_head) -> *mut page {
    entry as *mut page
}

#[inline]
unsafe fn list_first_entry_page(head: *mut list_head) -> *mut page {
    (*head).next as *mut page
}

#[no_mangle]
pub unsafe extern "C" fn ceph_pagelist_alloc(gfp_flags: gfp_t) -> *mut ceph_pagelist {
    let pl = kmalloc_obj(core::mem::size_of::<ceph_pagelist>(), gfp_flags);
    if pl.is_null() {
        return core::ptr::null_mut();
    }

    init_list_head(&mut (*pl).head);
    (*pl).mapped_tail = core::ptr::null_mut();
    (*pl).length = 0;
    (*pl).room = 0;
    init_list_head(&mut (*pl).free_list);
    (*pl).num_pages_free = 0;
    refcount_set(&mut (*pl).refcnt, 1);

    pl
}

unsafe fn ceph_pagelist_unmap_tail(pl: *mut ceph_pagelist) {
    if !(*pl).mapped_tail.is_null() {
        let page = list_entry_page((*pl).head.prev);
        kunmap(page);
        (*pl).mapped_tail = core::ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn ceph_pagelist_release(pl: *mut ceph_pagelist) {
    if !refcount_dec_and_test(&mut (*pl).refcnt) {
        return;
    }
    ceph_pagelist_unmap_tail(pl);
    while !list_empty(&(*pl).head) {
        let page = list_first_entry_page(&mut (*pl).head);
        list_del(&mut (*page).lru);
        __free_page(page);
    }
    ceph_pagelist_free_reserve(pl);
    kfree(pl as *mut c_void);
}

unsafe fn ceph_pagelist_addpage(pl: *mut ceph_pagelist) -> i32 {
    let page;

    if (*pl).num_pages_free == 0 {
        page = __page_cache_alloc(GFP_NOFS);
    } else {
        page = list_first_entry_page(&mut (*pl).free_list);
        list_del(&mut (*page).lru);
        (*pl).num_pages_free -= 1;
    }
    if page.is_null() {
        return -12;
    }
    (*pl).room += PAGE_SIZE;
    ceph_pagelist_unmap_tail(pl);
    list_add_tail(&mut (*page).lru, &mut (*pl).head);
    (*pl).mapped_tail = kmap(page);
    0
}

#[no_mangle]
pub unsafe extern "C" fn ceph_pagelist_append(
    pl: *mut ceph_pagelist,
    mut buf: *const u8,
    mut len: size_t,
) -> i32 {
    while (*pl).room < len {
        let bit = (*pl).room;
        let offset = (*pl).length & !PAGE_MASK;
        memcpy((*pl).mapped_tail.add(offset) as *mut c_void, buf as *const c_void, bit);
        (*pl).length += bit;
        (*pl).room -= bit;
        buf = buf.add(bit);
        len -= bit;
        let ret = ceph_pagelist_addpage(pl);
        if ret != 0 {
            return ret;
        }
    }

    let offset = (*pl).length & !PAGE_MASK;
    memcpy((*pl).mapped_tail.add(offset) as *mut c_void, buf as *const c_void, len);
    (*pl).length += len;
    (*pl).room -= len;
    0
}

/* Allocate enough pages for a pagelist to append the given amount
 * of data without allocating.
 * Returns: 0 on success, -ENOMEM on error.
 */
#[no_mangle]
pub unsafe extern "C" fn ceph_pagelist_reserve(pl: *mut ceph_pagelist, mut space: size_t) -> i32 {
    if space <= (*pl).room {
        return 0;
    }
    space -= (*pl).room;
    space = (space + PAGE_SIZE - 1) >> PAGE_SHIFT; // conv to num pages

    while space > (*pl).num_pages_free {
        let page = __page_cache_alloc(GFP_NOFS);
        if page.is_null() {
            return -12;
        }
        list_add_tail(&mut (*page).lru, &mut (*pl).free_list);
        (*pl).num_pages_free += 1;
    }
    0
}

/* Free any pages that have been preallocated. */
#[no_mangle]
pub unsafe extern "C" fn ceph_pagelist_free_reserve(pl: *mut ceph_pagelist) -> i32 {
    while !list_empty(&(*pl).free_list) {
        let page = list_first_entry_page(&mut (*pl).free_list);
        list_del(&mut (*page).lru);
        __free_page(page);
        (*pl).num_pages_free -= 1;
    }
    bug_on((*pl).num_pages_free != 0);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
