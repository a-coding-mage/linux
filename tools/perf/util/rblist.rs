// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on strlist.c by:
 * (c) 2009 Arnaldo Carvalho de Melo <acme@redhat.com>
 */

// Original C dependencies:
// #include <errno.h>
// #include <stdio.h>
// #include <stdlib.h>
// #include "rblist.h"

use core::ffi::c_void;

pub const EEXIST: i32 = 17;
pub const ENOMEM: i32 = 12;

#[repr(C)]
pub struct rb_node {
    pub rb_left: *mut rb_node,
    pub rb_right: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct rb_root_cached {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node,
}

#[repr(C)]
pub struct rblist {
    pub entries: rb_root_cached,
    pub nr_entries: u32,
    pub node_cmp: Option<unsafe extern "C" fn(*mut rb_node, *const c_void) -> i32>,
    pub node_new: Option<unsafe extern "C" fn(*mut rblist, *const c_void) -> *mut rb_node>,
    pub node_delete: Option<unsafe extern "C" fn(*mut rblist, *mut rb_node)>,
}

pub const RB_ROOT_CACHED: rb_root_cached = rb_root_cached {
    rb_root: rb_root {
        rb_node: core::ptr::null_mut(),
    },
    rb_leftmost: core::ptr::null_mut(),
};

unsafe extern "C" {
    pub fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, rb_link: *mut *mut rb_node);
    pub fn rb_insert_color_cached(node: *mut rb_node, root: *mut rb_root_cached, leftmost: bool);
    pub fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached);
    pub fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node;
    pub fn rb_next(node: *const rb_node) -> *mut rb_node;
    pub fn free(ptr: *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rblist__add_node(
    rblist: *mut rblist,
    new_entry: *const c_void,
) -> i32 {
    let mut p: *mut *mut rb_node = unsafe { &mut (*rblist).entries.rb_root.rb_node };
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let new_node: *mut rb_node;
    let mut leftmost: bool = true;

    unsafe {
        while !(*p).is_null() {
            let rc: i32;

            parent = *p;

            rc = ((*rblist).node_cmp.unwrap())(parent, new_entry);
            if rc > 0 {
                p = &mut (**p).rb_left;
            } else if rc < 0 {
                p = &mut (**p).rb_right;
                leftmost = false;
            } else {
                return -EEXIST;
            }
        }

        new_node = ((*rblist).node_new.unwrap())(rblist, new_entry);
        if new_node.is_null() {
            return -ENOMEM;
        }

        rb_link_node(new_node, parent, p);
        rb_insert_color_cached(new_node, &mut (*rblist).entries, leftmost);
        (*rblist).nr_entries = (*rblist).nr_entries.wrapping_add(1);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rblist__remove_node(rblist: *mut rblist, rb_node: *mut rb_node) {
    unsafe {
        rb_erase_cached(rb_node, &mut (*rblist).entries);
        (*rblist).nr_entries = (*rblist).nr_entries.wrapping_sub(1);
        ((*rblist).node_delete.unwrap())(rblist, rb_node);
    }
}

unsafe extern "C" fn __rblist__findnew(
    rblist: *mut rblist,
    entry: *const c_void,
    create: bool,
) -> *mut rb_node {
    let mut p: *mut *mut rb_node = unsafe { &mut (*rblist).entries.rb_root.rb_node };
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let mut new_node: *mut rb_node = core::ptr::null_mut();
    let mut leftmost: bool = true;

    unsafe {
        while !(*p).is_null() {
            let rc: i32;

            parent = *p;

            rc = ((*rblist).node_cmp.unwrap())(parent, entry);
            if rc > 0 {
                p = &mut (**p).rb_left;
            } else if rc < 0 {
                p = &mut (**p).rb_right;
                leftmost = false;
            } else {
                return parent;
            }
        }

        if create {
            new_node = ((*rblist).node_new.unwrap())(rblist, entry);
            if !new_node.is_null() {
                rb_link_node(new_node, parent, p);
                rb_insert_color_cached(new_node, &mut (*rblist).entries, leftmost);
                (*rblist).nr_entries = (*rblist).nr_entries.wrapping_add(1);
            }
        }
    }

    new_node
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rblist__find(
    rblist: *mut rblist,
    entry: *const c_void,
) -> *mut rb_node {
    unsafe { __rblist__findnew(rblist, entry, false) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rblist__findnew(
    rblist: *mut rblist,
    entry: *const c_void,
) -> *mut rb_node {
    unsafe { __rblist__findnew(rblist, entry, true) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rblist__init(rblist: *mut rblist) {
    unsafe {
        if !rblist.is_null() {
            (*rblist).entries = RB_ROOT_CACHED;
            (*rblist).nr_entries = 0;
        }
    }

    return;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rblist__exit(rblist: *mut rblist) {
    let mut pos: *mut rb_node;
    let mut next: *mut rb_node = unsafe { rb_first_cached(&(*rblist).entries) };

    unsafe {
        while !next.is_null() {
            pos = next;
            next = rb_next(pos);
            rblist__remove_node(rblist, pos);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rblist__delete(rblist: *mut rblist) {
    unsafe {
        if !rblist.is_null() {
            rblist__exit(rblist);
            free(rblist as *mut c_void);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rblist__entry(rblist: *const rblist, mut idx: u32) -> *mut rb_node {
    let mut node: *mut rb_node;

    unsafe {
        node = rb_first_cached(&(*rblist).entries);
        while !node.is_null() {
            if idx == 0 {
                return node;
            }
            idx = idx.wrapping_sub(1);
            node = rb_next(node);
        }
    }

    core::ptr::null_mut()
}
