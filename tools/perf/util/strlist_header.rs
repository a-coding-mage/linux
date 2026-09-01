/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/strlist.h. */
/* Dependencies originally included: <linux/rbtree.h>, <stdbool.h>, "rblist.h". */

use core::ffi::c_char;
use core::ptr;

use crate::{rb_first_cached, rb_next, rb_node, rblist, rblist__empty, rblist__nr_entries};

#[repr(C)]
pub struct str_node {
    pub rb_node: rb_node,
    pub s: *const c_char,
}

#[repr(C)]
pub struct strlist {
    pub rblist: rblist,
    pub file_only: bool,
}

/*
 * @file_only: When dirname is present, only consider entries as filenames,
 *             that should not be added to the list if dirname/entry is not
 *             found
 */
#[repr(C)]
pub struct strlist_config {
    pub file_only: bool,
    pub dirname: *const c_char,
}

unsafe extern "C" {
    pub fn strlist__new(
        slist: *const c_char,
        config: *const strlist_config,
    ) -> *mut strlist;
    pub fn strlist__delete(slist: *mut strlist);

    pub fn strlist__remove(slist: *mut strlist, sn: *mut str_node);
    pub fn strlist__load(slist: *mut strlist, filename: *const c_char) -> i32;
    pub fn strlist__add(slist: *mut strlist, str_: *const c_char) -> i32;

    pub fn strlist__entry(slist: *const strlist, idx: u32) -> *mut str_node;
    pub fn strlist__find(slist: *mut strlist, entry: *const c_char) -> *mut str_node;
}

#[inline]
pub unsafe fn strlist__has_entry(slist: *mut strlist, entry: *const c_char) -> bool {
    unsafe { !strlist__find(slist, entry).is_null() }
}

#[inline]
pub unsafe fn strlist__empty(slist: *const strlist) -> bool {
    unsafe { rblist__empty(ptr::addr_of!((*slist).rblist)) }
}

#[inline]
pub unsafe fn strlist__nr_entries(slist: *const strlist) -> u32 {
    unsafe { rblist__nr_entries(ptr::addr_of!((*slist).rblist)) }
}

/* For strlist iteration */
#[inline]
pub unsafe fn strlist__first(slist: *mut strlist) -> *mut str_node {
    let rn: *mut rb_node = unsafe { rb_first_cached(ptr::addr_of_mut!((*slist).rblist.entries)) };
    if !rn.is_null() {
        rn.cast::<str_node>()
    } else {
        ptr::null_mut()
    }
}

#[inline]
pub unsafe fn strlist__next(sn: *mut str_node) -> *mut str_node {
    let rn: *mut rb_node;

    if sn.is_null() {
        return ptr::null_mut();
    }

    rn = unsafe { rb_next(ptr::addr_of_mut!((*sn).rb_node)) };
    if !rn.is_null() {
        rn.cast::<str_node>()
    } else {
        ptr::null_mut()
    }
}

/**
 * strlist_for_each      - iterate over a strlist
 * @pos:	the &struct str_node to use as a loop cursor.
 * @slist:	the &struct strlist for loop.
 *
 * Original C macro:
 * #define strlist__for_each_entry(pos, slist) \
 *         for (pos = strlist__first(slist); pos; pos = strlist__next(pos))
 */

/**
 * strlist_for_each_safe - iterate over a strlist safe against removal of
 *                         str_node
 * @pos:	the &struct str_node to use as a loop cursor.
 * @n:		another &struct str_node to use as temporary storage.
 * @slist:	the &struct strlist for loop.
 *
 * Original C macro:
 * #define strlist__for_each_entry_safe(pos, n, slist) \
 *         for (pos = strlist__first(slist), n = strlist__next(pos); pos; \
 *              pos = n, n = strlist__next(n))
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
