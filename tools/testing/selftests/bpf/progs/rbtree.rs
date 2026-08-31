// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies: vmlinux.h, bpf/bpf_tracing.h, bpf/bpf_helpers.h,
// bpf/bpf_core_read.h, and "bpf_experimental.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::mem::offset_of;
use core::ptr;

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
    pub key: i64,
    pub data: i64,
    pub node: bpf_rb_node,
}

#[repr(C)]
pub struct root_nested_inner {
    pub glock: bpf_spin_lock,
    // __contains(node_data, node)
    pub root: bpf_rb_root,
}

#[repr(C)]
pub struct root_nested {
    pub inner: root_nested_inner,
}

unsafe extern "C" {
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_rbtree_add(
        root: *mut bpf_rb_root,
        node: *mut bpf_rb_node,
        less: unsafe extern "C" fn(*mut bpf_rb_node, *const bpf_rb_node) -> bool,
    );
    fn bpf_rbtree_remove(root: *mut bpf_rb_root, node: *mut bpf_rb_node) -> *mut bpf_rb_node;
    fn bpf_rbtree_first(root: *mut bpf_rb_root) -> *mut bpf_rb_node;
    fn bpf_obj_drop(obj: *mut node_data);
}

// bpf_obj_new(typeof(*ptr)) is supplied by BPF helper headers in C.
macro_rules! bpf_obj_new {
    ($ty:ty) => {
        bpf_obj_new::<$ty>()
    };
}

unsafe extern "Rust" {
    fn bpf_obj_new<T>() -> *mut T;
}

unsafe fn container_of_node_data_node(ptr: *mut bpf_rb_node) -> *mut node_data {
    (ptr as *mut u8).sub(offset_of!(node_data, node)) as *mut node_data
}

unsafe fn container_of_node_data_node_const(ptr: *const bpf_rb_node) -> *mut node_data {
    (ptr as *const u8).sub(offset_of!(node_data, node)) as *mut node_data
}

#[unsafe(no_mangle)]
pub static mut less_callback_ran: i64 = -1;
#[unsafe(no_mangle)]
pub static mut removed_key: i64 = -1;
#[unsafe(no_mangle)]
pub static mut first_data: [i64; 2] = [-1, -1];

// #define private(name) SEC(".data." #name) __hidden __attribute__((aligned(8)))
#[unsafe(link_section = ".data.A")]
#[repr(align(8))]
pub static mut glock: bpf_spin_lock = bpf_spin_lock { _private: [] };
#[unsafe(link_section = ".data.A")]
#[repr(align(8))]
// __contains(node_data, node)
pub static mut groot: bpf_rb_root = bpf_rb_root { _private: [] };
#[unsafe(link_section = ".data.A")]
#[repr(align(8))]
// __contains(node_data, node)
pub static mut groot_array: [bpf_rb_root; 2] = [
    bpf_rb_root { _private: [] },
    bpf_rb_root { _private: [] },
];
#[unsafe(link_section = ".data.A")]
#[repr(align(8))]
// __contains(node_data, node)
pub static mut groot_array_one: [bpf_rb_root; 1] = [bpf_rb_root { _private: [] }];
#[unsafe(link_section = ".data.B")]
#[repr(align(8))]
pub static mut groot_nested: root_nested = root_nested {
    inner: root_nested_inner {
        glock: bpf_spin_lock { _private: [] },
        root: bpf_rb_root { _private: [] },
    },
};

unsafe extern "C" fn less(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let node_a: *mut node_data;
    let node_b: *mut node_data;

    node_a = unsafe { container_of_node_data_node(a) };
    node_b = unsafe { container_of_node_data_node_const(b) };
    unsafe {
        less_callback_ran = 1;
        (*node_a).key < (*node_b).key
    }
}

unsafe fn __add_three(root: *mut bpf_rb_root, lock: *mut bpf_spin_lock) -> i64 {
    let mut n: *mut node_data;
    let m: *mut node_data;

    n = unsafe { bpf_obj_new!(node_data) };
    if n.is_null() {
        return 1;
    }
    unsafe {
        (*n).key = 5;
    }

    m = unsafe { bpf_obj_new!(node_data) };
    if m.is_null() {
        unsafe {
            bpf_obj_drop(n);
        }
        return 2;
    }
    unsafe {
        (*m).key = 1;
    }

    unsafe {
        bpf_spin_lock(lock);
        bpf_rbtree_add(root, ptr::addr_of_mut!((*n).node), less);
        bpf_rbtree_add(root, ptr::addr_of_mut!((*m).node), less);
        bpf_spin_unlock(lock);
    }

    n = unsafe { bpf_obj_new!(node_data) };
    if n.is_null() {
        return 3;
    }
    unsafe {
        (*n).key = 3;

        bpf_spin_lock(lock);
        bpf_rbtree_add(root, ptr::addr_of_mut!((*n).node), less);
        bpf_spin_unlock(lock);
    }
    0
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rbtree_add_nodes(ctx: *mut core::ffi::c_void) -> i64 {
    unsafe { __add_three(ptr::addr_of_mut!(groot), ptr::addr_of_mut!(glock)) }
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rbtree_add_nodes_nested(ctx: *mut core::ffi::c_void) -> i64 {
    unsafe {
        __add_three(
            ptr::addr_of_mut!(groot_nested.inner.root),
            ptr::addr_of_mut!(groot_nested.inner.glock),
        )
    }
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rbtree_add_and_remove(ctx: *mut core::ffi::c_void) -> i64 {
    let mut res: *mut bpf_rb_node = ptr::null_mut();
    let mut n: *mut node_data;
    let mut m: *mut node_data = ptr::null_mut();

    n = unsafe { bpf_obj_new!(node_data) };
    if n.is_null() {
        return 1;
    }
    unsafe {
        (*n).key = 5;
    }

    m = unsafe { bpf_obj_new!(node_data) };
    if m.is_null() {
        unsafe {
            if !n.is_null() {
                bpf_obj_drop(n);
            }
        }
        return 1;
    }
    unsafe {
        (*m).key = 3;

        bpf_spin_lock(ptr::addr_of_mut!(glock));
        bpf_rbtree_add(ptr::addr_of_mut!(groot), ptr::addr_of_mut!((*n).node), less);
        bpf_rbtree_add(ptr::addr_of_mut!(groot), ptr::addr_of_mut!((*m).node), less);
        res = bpf_rbtree_remove(ptr::addr_of_mut!(groot), ptr::addr_of_mut!((*n).node));
        bpf_spin_unlock(ptr::addr_of_mut!(glock));
    }

    if res.is_null() {
        return 1;
    }

    n = unsafe { container_of_node_data_node(res) };
    unsafe {
        removed_key = (*n).key;
        bpf_obj_drop(n);
    }

    0
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rbtree_add_and_remove_array(ctx: *mut core::ffi::c_void) -> i64 {
    let mut res1: *mut bpf_rb_node = ptr::null_mut();
    let mut res2: *mut bpf_rb_node = ptr::null_mut();
    let mut res3: *mut bpf_rb_node = ptr::null_mut();
    let mut nodes: [[*mut node_data; 2]; 3] = [[ptr::null_mut(), ptr::null_mut()], [ptr::null_mut(), ptr::null_mut()], [ptr::null_mut(), ptr::null_mut()]];
    let mut n: *mut node_data;
    let mut k1: i64 = -1;
    let mut k2: i64 = -1;
    let mut k3: i64 = -1;
    let mut i: i32;
    let mut j: i32;

    i = 0;
    while i < 3 {
        j = 0;
        while j < 2 {
            nodes[i as usize][j as usize] = unsafe { bpf_obj_new!(node_data) };
            if nodes[i as usize][j as usize].is_null() {
                i = 0;
                while i < 3 {
                    j = 0;
                    while j < 2 {
                        if !nodes[i as usize][j as usize].is_null() {
                            unsafe {
                                bpf_obj_drop(nodes[i as usize][j as usize]);
                            }
                        }
                        j += 1;
                    }
                    i += 1;
                }
                return 1;
            }
            unsafe {
                (*nodes[i as usize][j as usize]).key = (i * 2 + j) as i64;
            }
            j += 1;
        }
        i += 1;
    }

    unsafe {
        bpf_spin_lock(ptr::addr_of_mut!(glock));
    }
    i = 0;
    while i < 2 {
        j = 0;
        while j < 2 {
            unsafe {
                bpf_rbtree_add(
                    ptr::addr_of_mut!(groot_array[i as usize]),
                    ptr::addr_of_mut!((*nodes[i as usize][j as usize]).node),
                    less,
                );
            }
            j += 1;
        }
        i += 1;
    }
    j = 0;
    while j < 2 {
        unsafe {
            bpf_rbtree_add(
                ptr::addr_of_mut!(groot_array_one[0]),
                ptr::addr_of_mut!((*nodes[2][j as usize]).node),
                less,
            );
        }
        j += 1;
    }
    unsafe {
        res1 = bpf_rbtree_remove(
            ptr::addr_of_mut!(groot_array[0]),
            ptr::addr_of_mut!((*nodes[0][0]).node),
        );
        res2 = bpf_rbtree_remove(
            ptr::addr_of_mut!(groot_array[1]),
            ptr::addr_of_mut!((*nodes[1][0]).node),
        );
        res3 = bpf_rbtree_remove(
            ptr::addr_of_mut!(groot_array_one[0]),
            ptr::addr_of_mut!((*nodes[2][0]).node),
        );
        bpf_spin_unlock(ptr::addr_of_mut!(glock));
    }

    if !res1.is_null() {
        n = unsafe { container_of_node_data_node(res1) };
        unsafe {
            k1 = (*n).key;
            bpf_obj_drop(n);
        }
    }
    if !res2.is_null() {
        n = unsafe { container_of_node_data_node(res2) };
        unsafe {
            k2 = (*n).key;
            bpf_obj_drop(n);
        }
    }
    if !res3.is_null() {
        n = unsafe { container_of_node_data_node(res3) };
        unsafe {
            k3 = (*n).key;
            bpf_obj_drop(n);
        }
    }
    if k1 != 0 || k2 != 2 || k3 != 4 {
        return 2;
    }

    0
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rbtree_first_and_remove(ctx: *mut core::ffi::c_void) -> i64 {
    let mut res: *mut bpf_rb_node = ptr::null_mut();
    let mut n: *mut node_data;
    let mut m: *mut node_data;
    let mut o: *mut node_data;

    n = unsafe { bpf_obj_new!(node_data) };
    if n.is_null() {
        return 1;
    }
    unsafe {
        (*n).key = 3;
        (*n).data = 4;
    }

    m = unsafe { bpf_obj_new!(node_data) };
    if m.is_null() {
        unsafe {
            if !n.is_null() {
                bpf_obj_drop(n);
            }
        }
        return 1;
    }
    unsafe {
        (*m).key = 5;
        (*m).data = 6;
    }

    o = unsafe { bpf_obj_new!(node_data) };
    if o.is_null() {
        unsafe {
            if !n.is_null() {
                bpf_obj_drop(n);
            }
            if !m.is_null() {
                bpf_obj_drop(m);
            }
        }
        return 1;
    }
    unsafe {
        (*o).key = 1;
        (*o).data = 2;

        bpf_spin_lock(ptr::addr_of_mut!(glock));
        bpf_rbtree_add(ptr::addr_of_mut!(groot), ptr::addr_of_mut!((*n).node), less);
        bpf_rbtree_add(ptr::addr_of_mut!(groot), ptr::addr_of_mut!((*m).node), less);
        bpf_rbtree_add(ptr::addr_of_mut!(groot), ptr::addr_of_mut!((*o).node), less);

        res = bpf_rbtree_first(ptr::addr_of_mut!(groot));
        if res.is_null() {
            bpf_spin_unlock(ptr::addr_of_mut!(glock));
            return 2;
        }

        o = container_of_node_data_node(res);
        first_data[0] = (*o).data;

        res = bpf_rbtree_remove(ptr::addr_of_mut!(groot), ptr::addr_of_mut!((*o).node));
        bpf_spin_unlock(ptr::addr_of_mut!(glock));
    }

    if res.is_null() {
        return 5;
    }

    o = unsafe { container_of_node_data_node(res) };
    unsafe {
        removed_key = (*o).key;
        bpf_obj_drop(o);

        bpf_spin_lock(ptr::addr_of_mut!(glock));
        res = bpf_rbtree_first(ptr::addr_of_mut!(groot));
        if res.is_null() {
            bpf_spin_unlock(ptr::addr_of_mut!(glock));
            return 3;
        }

        o = container_of_node_data_node(res);
        first_data[1] = (*o).data;
        bpf_spin_unlock(ptr::addr_of_mut!(glock));
    }

    0
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rbtree_api_release_aliasing(ctx: *mut core::ffi::c_void) -> i64 {
    let mut n: *mut node_data;
    let mut m: *mut node_data;
    let mut o: *mut node_data;
    let mut res: *mut bpf_rb_node;
    let mut res2: *mut bpf_rb_node;

    n = unsafe { bpf_obj_new!(node_data) };
    if n.is_null() {
        return 1;
    }
    unsafe {
        (*n).key = 41;
        (*n).data = 42;

        bpf_spin_lock(ptr::addr_of_mut!(glock));
        bpf_rbtree_add(ptr::addr_of_mut!(groot), ptr::addr_of_mut!((*n).node), less);
        bpf_spin_unlock(ptr::addr_of_mut!(glock));

        bpf_spin_lock(ptr::addr_of_mut!(glock));

        /* m and o point to the same node,
         * but verifier doesn't know this
         */
        res = bpf_rbtree_first(ptr::addr_of_mut!(groot));
        if res.is_null() {
            bpf_spin_unlock(ptr::addr_of_mut!(glock));
            return 1;
        }
        o = container_of_node_data_node(res);

        res = bpf_rbtree_first(ptr::addr_of_mut!(groot));
        if res.is_null() {
            bpf_spin_unlock(ptr::addr_of_mut!(glock));
            return 1;
        }
        m = container_of_node_data_node(res);

        res = bpf_rbtree_remove(ptr::addr_of_mut!(groot), ptr::addr_of_mut!((*m).node));
        /* Retval of previous remove returns an owning reference to m,
         * which is the same node non-owning ref o is pointing at.
         * We can safely try to remove o as the second rbtree_remove will
         * return NULL since the node isn't in a tree.
         *
         * Previously we relied on the verifier type system + rbtree_remove
         * invalidating non-owning refs to ensure that rbtree_remove couldn't
         * fail, but now rbtree_remove does runtime checking so we no longer
         * invalidate non-owning refs after remove.
         */
        res2 = bpf_rbtree_remove(ptr::addr_of_mut!(groot), ptr::addr_of_mut!((*o).node));

        bpf_spin_unlock(ptr::addr_of_mut!(glock));

        if !res.is_null() {
            o = container_of_node_data_node(res);
            first_data[0] = (*o).data;
            bpf_obj_drop(o);
        }
        if !res2.is_null() {
            /* The second remove fails, so res2 is null and this doesn't
             * execute
             */
            m = container_of_node_data_node(res2);
            first_data[1] = (*m).data;
            bpf_obj_drop(m);
        }
    }
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
