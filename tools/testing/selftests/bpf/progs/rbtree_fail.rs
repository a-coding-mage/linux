// SPDX-License-Identifier: GPL-2.0
// C dependencies: <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>, "bpf_experimental.h", "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

type c_long = i64;

#[repr(C)]
pub struct bpf_rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct node_data {
    pub key: c_long,
    pub data: c_long,
    pub node: bpf_rb_node,
}

type rb_less_cb = unsafe extern "C" fn(*mut bpf_rb_node, *const bpf_rb_node) -> bool;

extern "C" {
    fn bpf_obj_new_node_data() -> *mut node_data;
    fn bpf_obj_drop(ptr: *mut c_void);
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_rbtree_add(root: *mut bpf_rb_root, node: *mut bpf_rb_node, less: rb_less_cb);
    fn bpf_rbtree_remove(root: *mut bpf_rb_root, node: *mut bpf_rb_node) -> *mut bpf_rb_node;
    fn bpf_rbtree_first(root: *mut bpf_rb_root) -> *mut bpf_rb_node;
    fn __sink(ptr: *mut node_data);
}

// #define private(name) SEC(".data." #name) __hidden __attribute__((aligned(8)))
// private(A) struct bpf_spin_lock glock;
#[no_mangle]
#[link_section = ".data.A"]
pub static mut glock: bpf_spin_lock = bpf_spin_lock { _private: [] };

// private(A) struct bpf_rb_root groot __contains(node_data, node);
#[no_mangle]
#[link_section = ".data.A"]
pub static mut groot: bpf_rb_root = bpf_rb_root { _private: [] };

// private(A) struct bpf_rb_root groot2 __contains(node_data, node);
#[no_mangle]
#[link_section = ".data.A"]
pub static mut groot2: bpf_rb_root = bpf_rb_root { _private: [] };

#[inline]
unsafe fn container_of_node_data_node(ptr: *const bpf_rb_node) -> *mut node_data {
    (ptr as *const u8).sub(core::mem::offset_of!(node_data, node)) as *mut node_data
}

unsafe extern "C" fn less(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let node_a: *mut node_data;
    let node_b: *mut node_data;

    node_a = container_of_node_data_node(a);
    node_b = container_of_node_data_node(b);

    (*node_a).key < (*node_b).key
}

// SEC("?tc")
// __failure __msg("bpf_spin_lock at off=16 must be held for bpf_rb_root")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn rbtree_api_nolock_add(ctx: *mut c_void) -> c_long {
    let n: *mut node_data;

    n = bpf_obj_new_node_data();
    if n.is_null() {
        return 1;
    }

    bpf_rbtree_add(&raw mut groot, &mut (*n).node, less);
    0
}

// SEC("?tc")
// __failure __msg("bpf_spin_lock at off=16 must be held for bpf_rb_root")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn rbtree_api_nolock_remove(ctx: *mut c_void) -> c_long {
    let n: *mut node_data;

    n = bpf_obj_new_node_data();
    if n.is_null() {
        return 1;
    }

    bpf_spin_lock(&raw mut glock);
    bpf_rbtree_add(&raw mut groot, &mut (*n).node, less);
    bpf_spin_unlock(&raw mut glock);

    bpf_rbtree_remove(&raw mut groot, &mut (*n).node);
    0
}

// SEC("?tc")
// __failure __msg("bpf_spin_lock at off=16 must be held for bpf_rb_root")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn rbtree_api_nolock_first(ctx: *mut c_void) -> c_long {
    bpf_rbtree_first(&raw mut groot);
    0
}

// SEC("?tc")
// __retval(0)
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn rbtree_api_remove_unadded_node(ctx: *mut c_void) -> c_long {
    let n: *mut node_data;
    let m: *mut node_data;
    let res_n: *mut bpf_rb_node;
    let res_m: *mut bpf_rb_node;

    n = bpf_obj_new_node_data();
    if n.is_null() {
        return 1;
    }

    m = bpf_obj_new_node_data();
    if m.is_null() {
        bpf_obj_drop(n as *mut c_void);
        return 1;
    }

    bpf_spin_lock(&raw mut glock);
    bpf_rbtree_add(&raw mut groot, &mut (*n).node, less);

    res_n = bpf_rbtree_remove(&raw mut groot, &mut (*n).node);

    res_m = bpf_rbtree_remove(&raw mut groot, &mut (*m).node);
    bpf_spin_unlock(&raw mut glock);

    bpf_obj_drop(m as *mut c_void);
    if !res_n.is_null() {
        bpf_obj_drop(container_of_node_data_node(res_n) as *mut c_void);
    }
    if !res_m.is_null() {
        bpf_obj_drop(container_of_node_data_node(res_m) as *mut c_void);
        /* m was not added to the rbtree */
        return 2;
    }

    0
}

// SEC("?tc")
// __failure __msg("Unreleased reference id=3 alloc_insn={{[0-9]+}}")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn rbtree_api_remove_no_drop(ctx: *mut c_void) -> c_long {
    let mut res: *mut bpf_rb_node;
    let n: *mut node_data;

    bpf_spin_lock(&raw mut glock);
    res = bpf_rbtree_first(&raw mut groot);
    if res.is_null() {
        bpf_spin_unlock(&raw mut glock);
        return 1;
    }

    res = bpf_rbtree_remove(&raw mut groot, res);

    if !res.is_null() {
        n = container_of_node_data_node(res);
        __sink(n);
    }
    bpf_spin_unlock(&raw mut glock);

    /* if (res) { bpf_obj_drop(n); } is missing here */
    0
}

// SEC("?tc")
// __failure __msg("R2 expected pointer to allocated object")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn rbtree_api_add_to_multiple_trees(ctx: *mut c_void) -> c_long {
    let n: *mut node_data;

    n = bpf_obj_new_node_data();
    if n.is_null() {
        return 1;
    }

    bpf_spin_lock(&raw mut glock);
    bpf_rbtree_add(&raw mut groot, &mut (*n).node, less);

    /* This add should fail since n already in groot's tree */
    bpf_rbtree_add(&raw mut groot2, &mut (*n).node, less);
    bpf_spin_unlock(&raw mut glock);
    0
}

// SEC("?tc")
// __failure __msg("Possibly NULL pointer passed to trusted R2")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn rbtree_api_use_unchecked_remove_retval(ctx: *mut c_void) -> c_long {
    let mut res: *mut bpf_rb_node;

    bpf_spin_lock(&raw mut glock);

    res = bpf_rbtree_first(&raw mut groot);
    if res.is_null() {
        bpf_spin_unlock(&raw mut glock);
        return 1;
    }
    res = bpf_rbtree_remove(&raw mut groot, res);

    bpf_spin_unlock(&raw mut glock);

    bpf_spin_lock(&raw mut glock);
    /* Must check res for NULL before using in rbtree_add below */
    bpf_rbtree_add(&raw mut groot, res, less);
    bpf_spin_unlock(&raw mut glock);
    0
}

// SEC("?tc")
// __failure __msg("bpf_rbtree_remove can only take non-owning or refcounted bpf_rb_node pointer")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn rbtree_api_add_release_unlock_escape(ctx: *mut c_void) -> c_long {
    let n: *mut node_data;

    n = bpf_obj_new_node_data();
    if n.is_null() {
        return 1;
    }

    bpf_spin_lock(&raw mut glock);
    bpf_rbtree_add(&raw mut groot, &mut (*n).node, less);
    bpf_spin_unlock(&raw mut glock);

    bpf_spin_lock(&raw mut glock);
    /* After add() in previous critical section, n should be
     * release_on_unlock and released after previous spin_unlock,
     * so should not be possible to use it here
     */
    bpf_rbtree_remove(&raw mut groot, &mut (*n).node);
    bpf_spin_unlock(&raw mut glock);
    0
}

// SEC("?tc")
// __failure __msg("bpf_rbtree_remove can only take non-owning or refcounted bpf_rb_node pointer")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn rbtree_api_first_release_unlock_escape(ctx: *mut c_void) -> c_long {
    let res: *mut bpf_rb_node;
    let n: *mut node_data;

    bpf_spin_lock(&raw mut glock);
    res = bpf_rbtree_first(&raw mut groot);
    if res.is_null() {
        bpf_spin_unlock(&raw mut glock);
        return 1;
    }
    n = container_of_node_data_node(res);
    bpf_spin_unlock(&raw mut glock);

    bpf_spin_lock(&raw mut glock);
    /* After first() in previous critical section, n should be
     * release_on_unlock and released after previous spin_unlock,
     * so should not be possible to use it here
     */
    bpf_rbtree_remove(&raw mut groot, &mut (*n).node);
    bpf_spin_unlock(&raw mut glock);
    0
}

unsafe extern "C" fn less__bad_fn_call_add(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let node_a: *mut node_data;
    let node_b: *mut node_data;

    node_a = container_of_node_data_node(a);
    node_b = container_of_node_data_node(b);
    bpf_rbtree_add(&raw mut groot, &mut (*node_a).node, less);

    (*node_a).key < (*node_b).key
}

unsafe extern "C" fn less__bad_fn_call_remove(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let node_a: *mut node_data;
    let node_b: *mut node_data;

    node_a = container_of_node_data_node(a);
    node_b = container_of_node_data_node(b);
    bpf_rbtree_remove(&raw mut groot, &mut (*node_a).node);

    (*node_a).key < (*node_b).key
}

unsafe extern "C" fn less__bad_fn_call_first_unlock_after(
    a: *mut bpf_rb_node,
    b: *const bpf_rb_node,
) -> bool {
    let node_a: *mut node_data;
    let node_b: *mut node_data;

    node_a = container_of_node_data_node(a);
    node_b = container_of_node_data_node(b);
    bpf_rbtree_first(&raw mut groot);
    bpf_spin_unlock(&raw mut glock);

    (*node_a).key < (*node_b).key
}

#[inline(always)]
unsafe fn add_with_cb(cb: rb_less_cb) -> c_long {
    let n: *mut node_data;

    n = bpf_obj_new_node_data();
    if n.is_null() {
        return 1;
    }

    bpf_spin_lock(&raw mut glock);
    bpf_rbtree_add(&raw mut groot, &mut (*n).node, cb);
    bpf_spin_unlock(&raw mut glock);
    0
}

// SEC("?tc")
// __failure __msg("R2 expected pointer to allocated object")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn rbtree_api_add_bad_cb_bad_fn_call_add(ctx: *mut c_void) -> c_long {
    add_with_cb(less__bad_fn_call_add)
}

// SEC("?tc")
// __failure __msg("rbtree_remove not allowed in rbtree cb")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn rbtree_api_add_bad_cb_bad_fn_call_remove(ctx: *mut c_void) -> c_long {
    add_with_cb(less__bad_fn_call_remove)
}

// SEC("?tc")
// __failure __msg("can't spin_{lock,unlock} in rbtree cb")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn rbtree_api_add_bad_cb_bad_fn_call_first_unlock_after(
    ctx: *mut c_void,
) -> c_long {
    add_with_cb(less__bad_fn_call_first_unlock_after)
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
