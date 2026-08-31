/* SPDX-License-Identifier: GPL-2.0 */

/* Depends on Linux rbtree definitions for rb_root_cached and rb_node. */

/*
 * create node structs of the form:
 * struct my_node {
 *     struct rb_node rb_node;
 *     ... my data ...
 * };
 *
 * create list structs of the form:
 * struct mylist {
 *     struct rblist rblist;
 *     ... my data ...
 * };
 */

use core::ffi::c_void;

#[repr(C)]
pub struct rblist {
    pub entries: rb_root_cached,
    pub nr_entries: u32,

    pub node_cmp: Option<unsafe extern "C" fn(rbn: *mut rb_node, entry: *const c_void) -> i32>,
    pub node_new:
        Option<unsafe extern "C" fn(rlist: *mut rblist, new_entry: *const c_void) -> *mut rb_node>,
    pub node_delete: Option<unsafe extern "C" fn(rblist: *mut rblist, rb_node: *mut rb_node)>,
}

unsafe extern "C" {
    pub fn rblist__init(rblist: *mut rblist);
    pub fn rblist__exit(rblist: *mut rblist);
    pub fn rblist__delete(rblist: *mut rblist);
    pub fn rblist__add_node(rblist: *mut rblist, new_entry: *const c_void) -> i32;
    pub fn rblist__remove_node(rblist: *mut rblist, rb_node: *mut rb_node);
    pub fn rblist__find(rblist: *mut rblist, entry: *const c_void) -> *mut rb_node;
    pub fn rblist__findnew(rblist: *mut rblist, entry: *const c_void) -> *mut rb_node;
    pub fn rblist__entry(rblist: *const rblist, idx: u32) -> *mut rb_node;
}

#[inline]
pub unsafe fn rblist__empty(rblist: *const rblist) -> bool {
    unsafe { (*rblist).nr_entries == 0 }
}

#[inline]
pub unsafe fn rblist__nr_entries(rblist: *const rblist) -> u32 {
    unsafe { (*rblist).nr_entries }
}
