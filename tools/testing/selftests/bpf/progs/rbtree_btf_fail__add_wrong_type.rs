// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_core_read.h>
// #include "bpf_experimental.h"

use core::ffi::c_void;
use core::mem::MaybeUninit;

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
    pub key: i32,
    pub data: i32,
    pub node: bpf_rb_node,
}

#[repr(C)]
pub struct node_data2 {
    pub key: i32,
    pub node: bpf_rb_node,
    pub data: i32,
}

unsafe extern "C" {
    fn bpf_obj_new_impl(local_type_id: u64, meta: *mut c_void) -> *mut c_void;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_rbtree_add(
        root: *mut bpf_rb_root,
        node: *mut bpf_rb_node,
        less: unsafe extern "C" fn(*mut bpf_rb_node, *const bpf_rb_node) -> bool,
    );
}

#[inline]
unsafe fn container_of_bpf_rb_node_node_data2(ptr: *const bpf_rb_node) -> *mut node_data2 {
    let uninit = MaybeUninit::<node_data2>::uninit();
    let base = uninit.as_ptr();
    let offset = unsafe { &raw const (*base).node as usize - base as usize };

    (ptr as *const u8).wrapping_sub(offset) as *mut node_data2
}

unsafe extern "C" fn less2(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let node_a: *mut node_data2;
    let node_b: *mut node_data2;

    node_a = unsafe { container_of_bpf_rb_node_node_data2(a) };
    node_b = unsafe { container_of_bpf_rb_node_node_data2(b) };

    unsafe { (*node_a).key < (*node_b).key }
}

// #define private(name) SEC(".data." #name) __hidden __attribute__((aligned(8)))
#[unsafe(link_section = ".data.A")]
#[unsafe(no_mangle)]
static mut glock: bpf_spin_lock = bpf_spin_lock { _private: [] };

// Original declaration: private(A) struct bpf_rb_root groot __contains(node_data, node);
#[unsafe(link_section = ".data.A")]
#[unsafe(no_mangle)]
static mut groot: bpf_rb_root = bpf_rb_root { _private: [] };

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rbtree_api_add__add_wrong_type(ctx: *mut c_void) -> i64 {
    let mut n: *mut node_data2;

    n = unsafe { bpf_obj_new_impl(0, core::ptr::null_mut()) as *mut node_data2 };
    if n.is_null() {
        return 1;
    }

    unsafe { bpf_spin_lock(&raw mut glock) };
    unsafe { bpf_rbtree_add(&raw mut groot, &raw mut (*n).node, less2) };
    unsafe { bpf_spin_unlock(&raw mut glock) };
    return 0;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
