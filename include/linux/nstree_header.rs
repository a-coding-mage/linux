/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2025 Christian Brauner <brauner@kernel.org> */

// Translated from linux/nstree.h. C header dependencies are supplied by the
// surrounding kernel translation unit.

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct ns_common;
#[repr(C)]
pub struct ns_tree_root;
#[repr(C)]
pub struct ns_tree_node;
#[repr(C)]
pub struct rb_node;

extern "C" {
    pub static mut cgroup_ns_tree: ns_tree_root;
    pub static mut ipc_ns_tree: ns_tree_root;
    pub static mut mnt_ns_tree: ns_tree_root;
    pub static mut net_ns_tree: ns_tree_root;
    pub static mut pid_ns_tree: ns_tree_root;
    pub static mut time_ns_tree: ns_tree_root;
    pub static mut user_ns_tree: ns_tree_root;
    pub static mut uts_ns_tree: ns_tree_root;

    pub fn ns_tree_node_init(node: *mut ns_tree_node);
    pub fn ns_tree_root_init(root: *mut ns_tree_root);
    pub fn ns_tree_node_empty(node: *const ns_tree_node) -> bool;
    pub fn ns_tree_node_add(
        node: *mut ns_tree_node,
        root: *mut ns_tree_root,
        cmp: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> c_int>,
    ) -> *mut rb_node;
    pub fn ns_tree_node_del(node: *mut ns_tree_node, root: *mut ns_tree_root);

    pub fn __ns_tree_gen_id(ns: *mut ns_common, id: u64) -> u64;
    pub fn __ns_tree_add_raw(ns: *mut ns_common, ns_tree: *mut ns_tree_root);
    pub fn __ns_tree_remove(ns: *mut ns_common, ns_tree: *mut ns_tree_root);
    pub fn ns_tree_lookup_rcu(ns_id: u64, ns_type: c_int) -> *mut ns_common;
    pub fn __ns_tree_adjoined_rcu(
        ns: *mut ns_common,
        ns_tree: *mut ns_tree_root,
        previous: bool,
    ) -> *mut ns_common;
}

pub unsafe fn __ns_tree_add(ns: *mut ns_common, ns_tree: *mut ns_tree_root, id: u64) {
    __ns_tree_gen_id(ns, id);
    __ns_tree_add_raw(ns, ns_tree);
}

// C11 _Generic dispatches on the concrete namespace pointer type. Rust
// declarative macros cannot perform that type-based dispatch without the
// corresponding namespace types and conversion traits from dependent headers.
#[macro_export]
macro_rules! to_ns_tree {
    ($ns:expr) => {{
        compile_error!("to_ns_tree requires the concrete namespace type mapping");
        ::core::ptr::null_mut::<$crate::ns_tree_root>()
    }};
}

#[macro_export]
macro_rules! ns_tree_gen_id {
    ($ns:expr) => {
        unsafe {
            $crate::__ns_tree_gen_id(
                $crate::to_ns_common!($ns),
                if ($ns) == $crate::ns_init_ns!($ns) {
                    $crate::ns_init_id!($ns)
                } else {
                    0
                },
            )
        }
    };
}

/**
 * ns_tree_add_raw - Add a namespace to a namespace
 * @__ns: Namespace to add
 *
 * This function adds a namespace to the appropriate namespace tree
 * without assigning a id.
 */
#[macro_export]
macro_rules! ns_tree_add_raw {
    ($ns:expr) => {
        unsafe { $crate::__ns_tree_add_raw($crate::to_ns_common!($ns), $crate::to_ns_tree!($ns)) }
    };
}

/**
 * ns_tree_add - Add a namespace to a namespace tree
 * @__ns: Namespace to add
 *
 * This function assigns a new id to the namespace and adds it to the
 * appropriate namespace tree and list.
 */
#[macro_export]
macro_rules! ns_tree_add {
    ($ns:expr) => {
        unsafe {
            $crate::__ns_tree_add(
                $crate::to_ns_common!($ns),
                $crate::to_ns_tree!($ns),
                if ($ns) == $crate::ns_init_ns!($ns) { $crate::ns_init_id!($ns) } else { 0 },
            )
        }
    };
}

/**
 * ns_tree_remove - Remove a namespace from a namespace tree
 * @__ns: Namespace to remove
 *
 * This function removes a namespace from the appropriate namespace
 * tree and list.
 */
#[macro_export]
macro_rules! ns_tree_remove {
    ($ns:expr) => {
        unsafe { $crate::__ns_tree_remove($crate::to_ns_common!($ns), $crate::to_ns_tree!($ns)) }
    };
}

#[macro_export]
macro_rules! ns_tree_adjoined_rcu {
    ($ns:expr, $previous:expr) => {
        unsafe {
            $crate::__ns_tree_adjoined_rcu(
                $crate::to_ns_common!($ns),
                $crate::to_ns_tree!($ns),
                $previous,
            )
        }
    };
}

#[macro_export]
macro_rules! ns_tree_active {
    ($ns:expr) => {
        !unsafe { $crate::rb_empty_node!($crate::ns_common_tree_node!($crate::to_ns_common!($ns))) }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
