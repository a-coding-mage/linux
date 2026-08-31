// SPDX-License-Identifier: GPL-2.0-only
/*
 * call-path.h: Manipulate a tree data structure containing function call paths
 * Copyright (c) 2014, Intel Corporation.
 */

// Dependencies from the original C file:
// <linux/rbtree.h>, <linux/list.h>, <linux/zalloc.h>, <stdlib.h>,
// and "call-path.h".

pub type u64 = ::std::os::raw::c_ulonglong;

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
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct call_path {
    pub parent: *mut call_path,
    pub sym: *mut symbol,
    pub ip: u64,
    pub db_id: u64,
    pub in_kernel: bool,
    pub rb_node: rb_node,
    pub children: rb_root,
}

#[repr(C)]
pub struct call_path_block {
    pub node: list_head,
    pub cp: [call_path; CALL_PATH_BLOCK_SIZE],
}

#[repr(C)]
pub struct call_path_root {
    pub call_path: call_path,
    pub blocks: list_head,
    pub next: usize,
    pub sz: usize,
}

extern "C" {
    fn zalloc(size: usize) -> *mut ::std::os::raw::c_void;
    fn free(ptr: *mut ::std::os::raw::c_void);

    fn RB_CLEAR_NODE(node: *mut rb_node);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, rb_link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);

    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);

    fn list_last_entry_call_path_block(head: *mut list_head) -> *mut call_path_block;
    fn list_first_entry_call_path_block(head: *mut list_head) -> *mut call_path_block;
    fn list_next_entry_call_path_block(pos: *mut call_path_block) -> *mut call_path_block;
}

extern "C" {
    static CALL_PATH_BLOCK_SIZE: usize;
    static CALL_PATH_BLOCK_MASK: usize;
}

const RB_ROOT: rb_root = rb_root {
    rb_node: ::std::ptr::null_mut(),
};

unsafe fn rb_entry_call_path(ptr: *mut rb_node) -> *mut call_path {
    (ptr as *mut u8).offset(-(::std::mem::offset_of!(call_path, rb_node) as isize))
        as *mut call_path
}

unsafe fn list_entry_is_head_call_path_block(pos: *mut call_path_block, head: *mut list_head) -> bool {
    &mut (*pos).node as *mut list_head == head
}

unsafe fn call_path__init(
    cp: *mut call_path,
    parent: *mut call_path,
    sym: *mut symbol,
    ip: u64,
    in_kernel: bool,
) {
    (*cp).parent = parent;
    (*cp).sym = sym;
    (*cp).ip = if !sym.is_null() { 0 } else { ip };
    (*cp).db_id = 0;
    (*cp).in_kernel = in_kernel;
    RB_CLEAR_NODE(&mut (*cp).rb_node);
    (*cp).children = RB_ROOT;
}

#[no_mangle]
pub unsafe extern "C" fn call_path_root__new() -> *mut call_path_root {
    let cpr: *mut call_path_root;

    cpr = zalloc(::std::mem::size_of::<call_path_root>()) as *mut call_path_root;
    if cpr.is_null() {
        return ::std::ptr::null_mut();
    }
    call_path__init(
        &mut (*cpr).call_path,
        ::std::ptr::null_mut(),
        ::std::ptr::null_mut(),
        0,
        false,
    );
    INIT_LIST_HEAD(&mut (*cpr).blocks);
    cpr
}

#[no_mangle]
pub unsafe extern "C" fn call_path_root__free(cpr: *mut call_path_root) {
    let mut pos: *mut call_path_block;
    let mut n: *mut call_path_block;

    // C macro translation: list_for_each_entry_safe(pos, n, &cpr->blocks, node)
    pos = list_first_entry_call_path_block(&mut (*cpr).blocks);
    while !list_entry_is_head_call_path_block(pos, &mut (*cpr).blocks) {
        n = list_next_entry_call_path_block(pos);
        list_del_init(&mut (*pos).node);
        free(pos as *mut ::std::os::raw::c_void);
        pos = n;
    }
    free(cpr as *mut ::std::os::raw::c_void);
}

unsafe fn call_path__new(
    cpr: *mut call_path_root,
    parent: *mut call_path,
    sym: *mut symbol,
    ip: u64,
    in_kernel: bool,
) -> *mut call_path {
    let cpb: *mut call_path_block;
    let cp: *mut call_path;
    let n: usize;

    if (*cpr).next < (*cpr).sz {
        cpb = list_last_entry_call_path_block(&mut (*cpr).blocks);
    } else {
        cpb = zalloc(::std::mem::size_of::<call_path_block>()) as *mut call_path_block;
        if cpb.is_null() {
            return ::std::ptr::null_mut();
        }
        list_add_tail(&mut (*cpb).node, &mut (*cpr).blocks);
        (*cpr).sz += CALL_PATH_BLOCK_SIZE;
    }

    n = {
        let old = (*cpr).next;
        (*cpr).next = (*cpr).next.wrapping_add(1);
        old & CALL_PATH_BLOCK_MASK
    };
    cp = &mut (*cpb).cp[n];

    call_path__init(cp, parent, sym, ip, in_kernel);

    cp
}

#[no_mangle]
pub unsafe extern "C" fn call_path__findnew(
    cpr: *mut call_path_root,
    parent: *mut call_path,
    sym: *mut symbol,
    mut ip: u64,
    ks: u64,
) -> *mut call_path {
    let mut p: *mut *mut rb_node;
    let mut node_parent: *mut rb_node = ::std::ptr::null_mut();
    let mut cp: *mut call_path;
    let in_kernel: bool = ip >= ks;

    if !sym.is_null() {
        ip = 0;
    }

    if parent.is_null() {
        return call_path__new(cpr, parent, sym, ip, in_kernel);
    }

    p = &mut (*parent).children.rb_node;
    while !(*p).is_null() {
        node_parent = *p;
        cp = rb_entry_call_path(node_parent);

        if (*cp).sym == sym && (*cp).ip == ip {
            return cp;
        }

        if (sym as usize) < ((*cp).sym as usize)
            || (sym == (*cp).sym && ip < (*cp).ip)
        {
            p = &mut (**p).rb_left;
        } else {
            p = &mut (**p).rb_right;
        }
    }

    cp = call_path__new(cpr, parent, sym, ip, in_kernel);
    if cp.is_null() {
        return ::std::ptr::null_mut();
    }

    rb_link_node(&mut (*cp).rb_node, node_parent, p);
    rb_insert_color(&mut (*cp).rb_node, &mut (*parent).children);

    cp
}
