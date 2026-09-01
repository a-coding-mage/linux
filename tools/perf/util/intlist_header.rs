/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies from the original header:
 * #include <linux/rbtree.h>
 * #include <stdbool.h>
 * #include "rblist.h"
 */

use std::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct int_node {
    pub rb_node: rb_node,
    pub i: c_ulong,
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct intlist {
    pub rblist: rblist,
}

unsafe extern "C" {
    pub fn intlist__new(slist: *const c_char) -> *mut intlist;
    pub fn intlist__delete(ilist: *mut intlist);

    pub fn intlist__remove(ilist: *mut intlist, in_: *mut int_node);
    pub fn intlist__add(ilist: *mut intlist, i: c_ulong) -> c_int;

    pub fn intlist__entry(ilist: *const intlist, idx: ::std::ffi::c_uint) -> *mut int_node;
    pub fn intlist__find(ilist: *mut intlist, i: c_ulong) -> *mut int_node;
    pub fn intlist__findnew(ilist: *mut intlist, i: c_ulong) -> *mut int_node;

    pub fn rblist__empty(rblist: *const rblist) -> bool;
    pub fn rblist__nr_entries(rblist: *const rblist) -> ::std::ffi::c_uint;
    pub fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node;
    pub fn rb_next(node: *const rb_node) -> *mut rb_node;
}

#[inline]
pub unsafe fn intlist__has_entry(ilist: *mut intlist, i: c_ulong) -> bool {
    unsafe { !intlist__find(ilist, i).is_null() }
}

#[inline]
pub unsafe fn intlist__empty(ilist: *const intlist) -> bool {
    unsafe { rblist__empty(&(*ilist).rblist) }
}

#[inline]
pub unsafe fn intlist__nr_entries(ilist: *const intlist) -> ::std::ffi::c_uint {
    unsafe { rblist__nr_entries(&(*ilist).rblist) }
}

/* For intlist iteration */
#[inline]
pub unsafe fn intlist__first(ilist: *mut intlist) -> *mut int_node {
    let rn: *mut rb_node = unsafe { rb_first_cached(&(*ilist).rblist.entries) };
    if !rn.is_null() {
        unsafe { rb_entry_int_node(rn) }
    } else {
        ::std::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn intlist__next(in_: *mut int_node) -> *mut int_node {
    let rn: *mut rb_node;

    if in_.is_null() {
        return ::std::ptr::null_mut();
    }

    rn = unsafe { rb_next(&(*in_).rb_node) };
    if !rn.is_null() {
        unsafe { rb_entry_int_node(rn) }
    } else {
        ::std::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn rb_entry_int_node(rn: *mut rb_node) -> *mut int_node {
    unsafe { (rn as *mut u8).sub(::std::mem::offset_of!(int_node, rb_node)) as *mut int_node }
}

/**
 * intlist__for_each_entry      - iterate over a intlist
 * @pos:        the &struct int_node to use as a loop cursor.
 * @ilist:      the &struct intlist for loop.
 */
#[macro_export]
macro_rules! intlist__for_each_entry {
    ($pos:ident, $ilist:expr, $body:block) => {{
        $pos = unsafe { intlist__first($ilist) };
        while !$pos.is_null() {
            $body
            $pos = unsafe { intlist__next($pos) };
        }
    }};
}

/**
 * intlist__for_each_entry_safe - iterate over a intlist safe against removal of
 *                         int_node
 * @pos:        the &struct int_node to use as a loop cursor.
 * @n:          another &struct int_node to use as temporary storage.
 * @ilist:      the &struct intlist for loop.
 */
#[macro_export]
macro_rules! intlist__for_each_entry_safe {
    ($pos:ident, $n:ident, $ilist:expr, $body:block) => {{
        $pos = unsafe { intlist__first($ilist) };
        $n = unsafe { intlist__next($pos) };
        while !$pos.is_null() {
            $body
            $pos = $n;
            $n = unsafe { intlist__next($n) };
        }
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
