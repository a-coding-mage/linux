// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on intlist.c by:
 * (c) 2009 Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

const EINVAL: c_int = 22;

#[repr(C)]
pub struct rb_node {
    pub rb_parent_color: c_ulong,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct rblist {
    pub entries: rb_root,
    pub nr_entries: u32,
    pub node_cmp: Option<unsafe extern "C" fn(*mut rb_node, *const c_void) -> c_int>,
    pub node_new: Option<unsafe extern "C" fn(*mut rblist, *const c_void) -> *mut rb_node>,
    pub node_delete: Option<unsafe extern "C" fn(*mut rblist, *mut rb_node)>,
}

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
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;

    fn rblist__init(rblist: *mut rblist);
    fn rblist__add_node(rblist: *mut rblist, entry: *mut c_void) -> c_int;
    fn rblist__remove_node(rblist: *mut rblist, rb_node: *mut rb_node);
    fn rblist__findnew(rblist: *mut rblist, entry: *mut c_void) -> *mut rb_node;
    fn rblist__find(rblist: *mut rblist, entry: *mut c_void) -> *mut rb_node;
    fn rblist__delete(rblist: *mut rblist);
    fn rblist__entry(rblist: *const rblist, idx: u32) -> *mut rb_node;
}

unsafe fn int_node_from_rb_node(rb_node: *mut rb_node) -> *mut int_node {
    let offset = unsafe {
        ptr::addr_of!((*ptr::null::<int_node>()).rb_node) as usize
    };

    (rb_node as *mut u8).wrapping_sub(offset) as *mut int_node
}

unsafe extern "C" fn intlist__node_new(
    _rblist: *mut rblist,
    entry: *const c_void,
) -> *mut rb_node {
    let i = entry as c_ulong;
    let mut rc: *mut rb_node = ptr::null_mut();
    let node = unsafe { malloc(mem::size_of::<int_node>()) as *mut int_node };

    if !node.is_null() {
        unsafe {
            (*node).i = i;
            (*node).priv_ = ptr::null_mut();
            rc = ptr::addr_of_mut!((*node).rb_node);
        }
    }

    rc
}

unsafe fn int_node__delete(ilist: *mut int_node) {
    unsafe {
        free(ilist as *mut c_void);
    }
}

unsafe extern "C" fn intlist__node_delete(_rblist: *mut rblist, rb_node: *mut rb_node) {
    let node = unsafe { int_node_from_rb_node(rb_node) };

    unsafe {
        int_node__delete(node);
    }
}

unsafe extern "C" fn intlist__node_cmp(rb_node: *mut rb_node, entry: *const c_void) -> c_int {
    let i = entry as c_ulong;
    let node = unsafe { int_node_from_rb_node(rb_node) };

    unsafe {
        if (*node).i > i {
            return 1;
        } else if (*node).i < i {
            return -1;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intlist__add(ilist: *mut intlist, i: c_ulong) -> c_int {
    unsafe { rblist__add_node(ptr::addr_of_mut!((*ilist).rblist), i as *mut c_void) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intlist__remove(ilist: *mut intlist, node: *mut int_node) {
    unsafe {
        rblist__remove_node(
            ptr::addr_of_mut!((*ilist).rblist),
            ptr::addr_of_mut!((*node).rb_node),
        );
    }
}

unsafe fn __intlist__findnew(
    ilist: *mut intlist,
    i: c_ulong,
    create: bool,
) -> *mut int_node {
    let mut node: *mut int_node = ptr::null_mut();
    let rb_node: *mut rb_node;

    if ilist.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        if create {
            rb_node = rblist__findnew(ptr::addr_of_mut!((*ilist).rblist), i as *mut c_void);
        } else {
            rb_node = rblist__find(ptr::addr_of_mut!((*ilist).rblist), i as *mut c_void);
        }
    }

    if !rb_node.is_null() {
        node = unsafe { int_node_from_rb_node(rb_node) };
    }

    node
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intlist__find(ilist: *mut intlist, i: c_ulong) -> *mut int_node {
    unsafe { __intlist__findnew(ilist, i, false) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intlist__findnew(ilist: *mut intlist, i: c_ulong) -> *mut int_node {
    unsafe { __intlist__findnew(ilist, i, true) }
}

unsafe fn intlist__parse_list(ilist: *mut intlist, mut s: *const c_char) -> c_int {
    let mut sep: *mut c_char = ptr::null_mut();
    let mut err: c_int;

    loop {
        let value = unsafe { strtol(s, ptr::addr_of_mut!(sep), 10) as c_ulong };
        err = -EINVAL;
        unsafe {
            if *sep != b',' as c_char && *sep != b'\0' as c_char {
                break;
            }
        }
        err = unsafe { intlist__add(ilist, value) };
        if err != 0 {
            break;
        }
        s = unsafe { sep.add(1) };
        unsafe {
            if *sep == b'\0' as c_char {
                break;
            }
        }
    }

    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intlist__new(slist: *const c_char) -> *mut intlist {
    let ilist = unsafe { malloc(mem::size_of::<intlist>()) as *mut intlist };

    if !ilist.is_null() {
        unsafe {
            rblist__init(ptr::addr_of_mut!((*ilist).rblist));
            (*ilist).rblist.node_cmp = Some(intlist__node_cmp);
            (*ilist).rblist.node_new = Some(intlist__node_new);
            (*ilist).rblist.node_delete = Some(intlist__node_delete);

            if !slist.is_null() && intlist__parse_list(ilist, slist) != 0 {
                intlist__delete(ilist);
                return ptr::null_mut();
            }
        }
    }

    ilist
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intlist__delete(ilist: *mut intlist) {
    if !ilist.is_null() {
        unsafe {
            rblist__delete(ptr::addr_of_mut!((*ilist).rblist));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intlist__entry(
    ilist: *const intlist,
    idx: u32,
) -> *mut int_node {
    let mut node: *mut int_node = ptr::null_mut();
    let rb_node: *mut rb_node;

    unsafe {
        rb_node = rblist__entry(ptr::addr_of!((*ilist).rblist), idx);
    }
    if !rb_node.is_null() {
        node = unsafe { int_node_from_rb_node(rb_node) };
    }

    node
}
