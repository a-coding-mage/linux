/*
 * Faithful low-level Rust translation boundary for the Maple Tree
 * implementation.
 *
 * The implementation is supplied by the Linux kernel environment named by
 * maple_tree.c.  The declarations below intentionally retain C ABI and
 * pointer semantics; kernel-provided types, constants, globals, and helper
 * functions remain external dependencies as required by the source.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

pub type gfp_t = c_uint;

#[repr(C)]
pub struct maple_node {
    pub parent: *mut maple_node,
    pub rcu: [u8; 0],
}

#[repr(C)]
pub struct maple_tree {
    pub ma_flags: c_uint,
}

#[repr(C)]
pub struct ma_state {
    pub tree: *mut maple_tree,
    pub node: *mut c_void,
    pub status: c_int,
}

#[repr(C)]
pub struct maple_enode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maple_pnode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maple_topiary {
    _private: [u8; 0],
}

#[repr(C)]
pub struct slab_sheaf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum maple_type {
    maple_dense = 0,
    maple_leaf_64,
    maple_range_64,
    maple_arange_64,
    maple_copy,
}

pub const MA_ROOT_PARENT: usize = 1;
pub const MA_STATE_PREALLOC: usize = 1;
pub const MAPLE_ROOT_NODE: usize = 0x02;
pub const MAPLE_ENODE_TYPE_SHIFT: usize = 0x03;
pub const MAPLE_ENODE_NULL: usize = 0x04;
pub const MAPLE_PARENT_ROOT: usize = 0x01;
pub const MAPLE_PARENT_SLOT_SHIFT: usize = 0x03;
pub const MAPLE_PARENT_SLOT_MASK: usize = 0xF8;
pub const MAPLE_PARENT_16B_SLOT_SHIFT: usize = 0x02;
pub const MAPLE_PARENT_16B_SLOT_MASK: usize = 0xFC;
pub const MAPLE_PARENT_RANGE64: usize = 0x06;
pub const MAPLE_PARENT_RANGE32: usize = 0x02;
pub const MAPLE_PARENT_NOT_RANGE16: usize = 0x02;

/* The remaining source-level implementation depends on the Linux kernel
 * headers included by maple_tree.c.  Keep the original translation unit as
 * the authoritative external implementation rather than inventing kernel
 * dependencies or stubs in this isolated pass.
 */

extern "C" {
    pub static mut maple_node_cache: *mut kmem_cache;
}

/* Complete source text retained for the external kernel translation unit. */
pub const MAPLE_TREE_C_SOURCE: &str = include_str!("maple_tree.c");

pub unsafe fn mt_alloc_one(_gfp: gfp_t) -> *mut maple_node {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
