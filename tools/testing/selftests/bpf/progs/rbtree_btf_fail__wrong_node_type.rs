// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies: <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>, "bpf_experimental.h"

/* BTF load should fail as bpf_rb_root __contains this type and points to
 * 'node', but 'node' is not a bpf_rb_node
 */
#[repr(C)]
pub struct node_data {
    pub key: i32,
    pub data: i32,
    pub node: bpf_list_node,
}

// private(name) expands to SEC(".data." #name) __hidden __attribute__((aligned(8)))
#[repr(align(8))]
#[unsafe(link_section = ".data.A")]
static mut glock: bpf_spin_lock = unsafe { core::mem::zeroed() };

// Original declaration used: __contains(node_data, node)
#[repr(align(8))]
#[unsafe(link_section = ".data.A")]
static mut groot: bpf_rb_root = unsafe { core::mem::zeroed() };

unsafe extern "C" {
    fn bpf_obj_new_impl(local_type_id: u64, meta: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_rbtree_first(root: *mut bpf_rb_root) -> *mut bpf_rb_node;
}

#[unsafe(link_section = "tc")]
pub unsafe extern "C" fn rbtree_api_add__wrong_node_type(ctx: *mut core::ffi::c_void) -> i64 {
    let mut n: *mut node_data;

    n = bpf_obj_new_impl(
        core::any::type_name::<node_data>().as_ptr() as u64,
        core::ptr::null_mut(),
    ) as *mut node_data;
    if n.is_null() {
        return 1;
    }

    bpf_spin_lock(core::ptr::addr_of_mut!(glock));
    bpf_rbtree_first(core::ptr::addr_of_mut!(groot));
    bpf_spin_unlock(core::ptr::addr_of_mut!(glock));
    return 0;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
