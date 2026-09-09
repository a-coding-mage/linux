/* SPDX-License-Identifier: GPL-2.0 */

// Kexec Handover Radix Tree.
//
// This is a radix tree implementation for tracking numeric keys across kexec
// transitions. It was developed for the KHO preserved memory map but is
// designed for broader use by any subsystem that needs to track keys.
// Conceptually speaking, the data structure is similar to a set. It tracks
// the presence or absence of numeric keys.
//
// The radix tree is a multi-level tree where leaf nodes are bitmaps
// representing individual keys.
//
// For the KHO preserved memory map, to allow pages of different sizes (orders)
// to be stored efficiently in a single tree, it uses a unique key encoding
// scheme. Each key is an unsigned long that combines a page's physical address
// and its order.
//
// Client code is responsible for allocating the root node of the tree,
// initializing the mutex lock, and managing its lifecycle. It must use the
// tree data structures defined in the KHO ABI,
// `include/linux/kho/abi/kexec_handover.h`.

// Supplied by the corresponding Linux/Rust dependency.
#[allow(non_camel_case_types)]
pub type phys_addr_t = u64;

// Supplied by the corresponding Linux/Rust dependency.
#[allow(non_camel_case_types)]
pub struct mutex;

pub struct kho_radix_node;

#[repr(C)]
pub struct kho_radix_tree {
    pub root: *mut kho_radix_node,
    pub lock: mutex, /* protects the tree's structure and root pointer */
}

/**
 * Callbacks for KHO radix tree walk.
 *
 * `leaf` is called on each present key in the radix tree.
 * `node` is called on each node of the radix tree itself. It receives the
 * physical address of the page containing the node.
 *
 * For each callback, a return value of 0 continues the walk and a non-zero
 * return value is directly returned to the caller.
 */
#[repr(C)]
pub struct kho_radix_walk_cb {
    pub leaf: Option<unsafe extern "C" fn(key: libc::c_ulong, data: *mut libc::c_void) -> libc::c_int>,
    pub node: Option<unsafe extern "C" fn(phys: phys_addr_t, data: *mut libc::c_void) -> libc::c_int>,
}

#[cfg(feature = "CONFIG_KEXEC_HANDOVER")]
extern "C" {
    pub fn kho_radix_add_key(tree: *mut kho_radix_tree, key: libc::c_ulong) -> libc::c_int;
    pub fn kho_radix_del_key(tree: *mut kho_radix_tree, key: libc::c_ulong);
    pub fn kho_radix_walk_tree(
        tree: *mut kho_radix_tree,
        cb: *const kho_radix_walk_cb,
        data: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn kho_radix_init_tree(
        tree: *mut kho_radix_tree,
        root: *mut kho_radix_node,
    ) -> libc::c_int;
    pub fn kho_radix_destroy_tree(tree: *mut kho_radix_tree);
}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
pub unsafe fn kho_radix_add_key(_tree: *mut kho_radix_tree, _key: libc::c_ulong) -> libc::c_int {
    -libc::EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
pub unsafe fn kho_radix_del_key(_tree: *mut kho_radix_tree, _key: libc::c_ulong) {}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
pub unsafe fn kho_radix_walk_tree(
    _tree: *mut kho_radix_tree,
    _cb: *const kho_radix_walk_cb,
    _data: *mut libc::c_void,
) -> libc::c_int {
    -libc::EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
pub unsafe fn kho_radix_init_tree(
    _tree: *mut kho_radix_tree,
    _root: *mut kho_radix_node,
) -> libc::c_int {
    0
}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER"))]
pub unsafe fn kho_radix_destroy_tree(_tree: *mut kho_radix_tree) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
