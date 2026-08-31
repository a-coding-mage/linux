// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 KylinSoft Corporation. */

/* Dependencies from the original C source:
 * #include <vmlinux.h>
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 * #include "bpf_experimental.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr;

const NR_NODES: i32 = 16;

#[repr(C)]
pub struct bpf_rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_refcount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct node_data {
    pub data: i32,
}

#[repr(C)]
pub struct tree_node {
    pub node: bpf_rb_node,
    pub key: u64,
    pub node_data: *mut node_data,
}

#[repr(C)]
pub struct tree_node_ref {
    pub ref_: bpf_refcount,
    pub node: bpf_rb_node,
    pub key: u64,
    pub node_data: *mut node_data,
}

/* private(name) SEC(".data." #name) __hidden __aligned(8) */
#[link_section = ".data.A"]
static mut root: MaybeUninit<bpf_rb_root> = MaybeUninit::uninit();
#[link_section = ".data.A"]
static mut lock: MaybeUninit<bpf_spin_lock> = MaybeUninit::uninit();

#[link_section = ".data.B"]
static mut root_r: MaybeUninit<bpf_rb_root> = MaybeUninit::uninit();
#[link_section = ".data.B"]
static mut lock_r: MaybeUninit<bpf_spin_lock> = MaybeUninit::uninit();

unsafe extern "C" {
    static can_loop: bool;

    fn bpf_obj_new_tree_node() -> *mut tree_node;
    fn bpf_obj_new_tree_node_ref() -> *mut tree_node_ref;
    fn bpf_obj_new_node_data() -> *mut node_data;
    fn bpf_obj_drop<T>(ptr: *mut T);
    fn bpf_kptr_xchg<T>(kptr: *mut *mut T, ptr: *mut T) -> *mut T;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_rbtree_add(
        root: *mut bpf_rb_root,
        node: *mut bpf_rb_node,
        less: unsafe extern "C" fn(*mut bpf_rb_node, *const bpf_rb_node) -> bool,
    );
    fn bpf_rbtree_root(root: *mut bpf_rb_root) -> *mut bpf_rb_node;
    fn bpf_rbtree_left(root: *mut bpf_rb_root, node: *mut bpf_rb_node) -> *mut bpf_rb_node;
    fn bpf_rbtree_right(root: *mut bpf_rb_root, node: *mut bpf_rb_node) -> *mut bpf_rb_node;
    fn bpf_rbtree_first(root: *mut bpf_rb_root) -> *mut bpf_rb_node;
    fn bpf_rbtree_remove(root: *mut bpf_rb_root, node: *mut bpf_rb_node) -> *mut bpf_rb_node;
    fn bpf_refcount_acquire(ptr: *mut tree_node_ref) -> *mut tree_node_ref;
}

unsafe fn container_of_tree_node_node(ptr: *mut bpf_rb_node) -> *mut tree_node {
    ptr.cast::<tree_node>()
}

unsafe fn container_of_tree_node_ref_node(ptr: *mut bpf_rb_node) -> *mut tree_node_ref {
    ptr.cast::<tree_node_ref>().byte_sub(core::mem::offset_of!(tree_node_ref, node))
}

unsafe extern "C" fn less(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let node_a: *mut tree_node;
    let node_b: *mut tree_node;

    node_a = container_of_tree_node_node(a);
    node_b = container_of_tree_node_node(b as *mut bpf_rb_node);

    (*node_a).key < (*node_b).key
}

/* SEC("syscall") */
/* __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rbtree_search_kptr(ctx: *mut c_void) -> i64 {
    let mut tnode: *mut tree_node;
    let mut rb_n: *mut bpf_rb_node;
    let mut node_data: *mut node_data;
    let lookup_key: i32 = NR_NODES / 2;
    let lookup_data: i32 = NR_NODES / 2;
    let mut i: i32;
    let data: i32;
    let mut ret: i32 = 0;

    let _ = ctx;

    i = 0;
    while i < NR_NODES && can_loop {
        tnode = bpf_obj_new_tree_node();
        if tnode.is_null() {
            return line!() as i64;
        }

        node_data = bpf_obj_new_node_data();
        if node_data.is_null() {
            bpf_obj_drop(tnode);
            return line!() as i64;
        }

        (*tnode).key = i as u64;
        (*node_data).data = i;

        node_data = bpf_kptr_xchg(&mut (*tnode).node_data, node_data);
        if !node_data.is_null() {
            bpf_obj_drop(node_data);
        }

        bpf_spin_lock(lock.as_mut_ptr());
        bpf_rbtree_add(root.as_mut_ptr(), &mut (*tnode).node, less);
        bpf_spin_unlock(lock.as_mut_ptr());

        i += 1;
    }

    bpf_spin_lock(lock.as_mut_ptr());
    rb_n = bpf_rbtree_root(root.as_mut_ptr());
    while !rb_n.is_null() && can_loop {
        tnode = container_of_tree_node_node(rb_n);
        node_data = bpf_kptr_xchg(&mut (*tnode).node_data, ptr::null_mut());
        if node_data.is_null() {
            ret = line!() as i32;
            goto_fail_kptr(ret);
            bpf_spin_unlock(lock.as_mut_ptr());
            return ret as i64;
        }

        data = (*node_data).data;
        node_data = bpf_kptr_xchg(&mut (*tnode).node_data, node_data);
        if !node_data.is_null() {
            bpf_spin_unlock(lock.as_mut_ptr());
            bpf_obj_drop(node_data);
            return line!() as i64;
        }

        if lookup_key as u64 == (*tnode).key {
            if data == lookup_data {
                break;
            }

            ret = line!() as i32;
            goto_fail_kptr(ret);
            bpf_spin_unlock(lock.as_mut_ptr());
            return ret as i64;
        }

        if (lookup_key as u64) < (*tnode).key {
            rb_n = bpf_rbtree_left(root.as_mut_ptr(), rb_n);
        } else {
            rb_n = bpf_rbtree_right(root.as_mut_ptr(), rb_n);
        }
    }
    bpf_spin_unlock(lock.as_mut_ptr());

    while can_loop {
        bpf_spin_lock(lock.as_mut_ptr());
        rb_n = bpf_rbtree_first(root.as_mut_ptr());
        if rb_n.is_null() {
            bpf_spin_unlock(lock.as_mut_ptr());
            return 0;
        }

        rb_n = bpf_rbtree_remove(root.as_mut_ptr(), rb_n);
        if rb_n.is_null() {
            ret = line!() as i32;
            bpf_spin_unlock(lock.as_mut_ptr());
            return ret as i64;
        }
        bpf_spin_unlock(lock.as_mut_ptr());

        tnode = container_of_tree_node_node(rb_n);

        node_data = bpf_kptr_xchg(&mut (*tnode).node_data, ptr::null_mut());
        if !node_data.is_null() {
            bpf_obj_drop(node_data);
        }

        bpf_obj_drop(tnode);
    }

    0
}

unsafe fn goto_fail_kptr(_ret: i32) {}

unsafe extern "C" fn less_r(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let node_a: *mut tree_node_ref;
    let node_b: *mut tree_node_ref;

    node_a = container_of_tree_node_ref_node(a);
    node_b = container_of_tree_node_ref_node(b as *mut bpf_rb_node);

    (*node_a).key < (*node_b).key
}

/* SEC("syscall") */
/* __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rbtree_search_kptr_ref(ctx: *mut c_void) -> i64 {
    let mut tnode_r: *mut tree_node_ref;
    let mut tnode_m: *mut tree_node_ref;
    let mut rb_n: *mut bpf_rb_node;
    let mut node_data: *mut node_data;
    let lookup_key: i32 = NR_NODES / 2;
    let lookup_data: i32 = NR_NODES / 2;
    let mut i: i32;
    let data: i32;
    let mut ret: i32 = 0;

    let _ = ctx;

    i = 0;
    while i < NR_NODES && can_loop {
        tnode_r = bpf_obj_new_tree_node_ref();
        if tnode_r.is_null() {
            return line!() as i64;
        }

        node_data = bpf_obj_new_node_data();
        if node_data.is_null() {
            bpf_obj_drop(tnode_r);
            return line!() as i64;
        }

        (*tnode_r).key = i as u64;
        (*node_data).data = i;

        node_data = bpf_kptr_xchg(&mut (*tnode_r).node_data, node_data);
        if !node_data.is_null() {
            bpf_obj_drop(node_data);
        }

        /* Unused reference */
        tnode_m = bpf_refcount_acquire(tnode_r);
        if tnode_m.is_null() {
            return line!() as i64;
        }

        bpf_spin_lock(lock_r.as_mut_ptr());
        bpf_rbtree_add(root_r.as_mut_ptr(), &mut (*tnode_r).node, less_r);
        bpf_spin_unlock(lock_r.as_mut_ptr());

        bpf_obj_drop(tnode_m);

        i += 1;
    }

    bpf_spin_lock(lock_r.as_mut_ptr());
    rb_n = bpf_rbtree_root(root_r.as_mut_ptr());
    while !rb_n.is_null() && can_loop {
        tnode_r = container_of_tree_node_ref_node(rb_n);
        node_data = bpf_kptr_xchg(&mut (*tnode_r).node_data, ptr::null_mut());
        if node_data.is_null() {
            ret = line!() as i32;
            bpf_spin_unlock(lock_r.as_mut_ptr());
            return ret as i64;
        }

        data = (*node_data).data;
        node_data = bpf_kptr_xchg(&mut (*tnode_r).node_data, node_data);
        if !node_data.is_null() {
            bpf_spin_unlock(lock_r.as_mut_ptr());
            bpf_obj_drop(node_data);
            return line!() as i64;
        }

        if lookup_key as u64 == (*tnode_r).key {
            if data == lookup_data {
                break;
            }

            ret = line!() as i32;
            bpf_spin_unlock(lock_r.as_mut_ptr());
            return ret as i64;
        }

        if (lookup_key as u64) < (*tnode_r).key {
            rb_n = bpf_rbtree_left(root_r.as_mut_ptr(), rb_n);
        } else {
            rb_n = bpf_rbtree_right(root_r.as_mut_ptr(), rb_n);
        }
    }
    bpf_spin_unlock(lock_r.as_mut_ptr());

    while can_loop {
        bpf_spin_lock(lock_r.as_mut_ptr());
        rb_n = bpf_rbtree_first(root_r.as_mut_ptr());
        if rb_n.is_null() {
            bpf_spin_unlock(lock_r.as_mut_ptr());
            return 0;
        }

        rb_n = bpf_rbtree_remove(root_r.as_mut_ptr(), rb_n);
        if rb_n.is_null() {
            ret = line!() as i32;
            bpf_spin_unlock(lock_r.as_mut_ptr());
            return ret as i64;
        }
        bpf_spin_unlock(lock_r.as_mut_ptr());

        tnode_r = container_of_tree_node_ref_node(rb_n);

        node_data = bpf_kptr_xchg(&mut (*tnode_r).node_data, ptr::null_mut());
        if !node_data.is_null() {
            bpf_obj_drop(node_data);
        }

        bpf_obj_drop(tnode_r);
    }

    0
}

/* SEC("syscall") */
/* __failure __msg("R1 type=scalar expected=map_value, ptr_, ptr_") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn non_own_ref_kptr_xchg_no_lock(ctx: *mut c_void) -> i64 {
    let mut tnode: *mut tree_node;
    let mut rb_n: *mut bpf_rb_node;
    let mut node_data: *mut node_data;
    let data: i32;

    let _ = ctx;

    bpf_spin_lock(lock.as_mut_ptr());
    rb_n = bpf_rbtree_first(root.as_mut_ptr());
    if rb_n.is_null() {
        bpf_spin_unlock(lock.as_mut_ptr());
        return line!() as i64;
    }
    bpf_spin_unlock(lock.as_mut_ptr());

    tnode = container_of_tree_node_node(rb_n);
    node_data = bpf_kptr_xchg(&mut (*tnode).node_data, ptr::null_mut());
    if node_data.is_null() {
        return line!() as i64;
    }

    data = (*node_data).data;
    if data < 0 {
        return line!() as i64;
    }

    node_data = bpf_kptr_xchg(&mut (*tnode).node_data, node_data);
    if !node_data.is_null() {
        return line!() as i64;
    }

    0
}

/* char _license[] SEC("license") = "GPL"; */
#[link_section = "license"]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
