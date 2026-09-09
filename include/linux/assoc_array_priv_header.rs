/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Private definitions for the generic associative array implementation.
 *
 * See Documentation/core-api/assoc_array.rst for information.
 *
 * Copyright (C) 2013 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* This content is conditional on CONFIG_ASSOCIATIVE_ARRAY in the C header. */

pub const ASSOC_ARRAY_FAN_OUT: usize = 16;
pub const ASSOC_ARRAY_FAN_MASK: usize = ASSOC_ARRAY_FAN_OUT - 1;
pub const ASSOC_ARRAY_LEVEL_STEP: usize = ilog2(ASSOC_ARRAY_FAN_OUT);
pub const ASSOC_ARRAY_LEVEL_STEP_MASK: usize = ASSOC_ARRAY_LEVEL_STEP - 1;
pub const ASSOC_ARRAY_KEY_CHUNK_MASK: usize = ASSOC_ARRAY_KEY_CHUNK_SIZE - 1;
pub const ASSOC_ARRAY_KEY_CHUNK_SHIFT: usize = ilog2(BITS_PER_LONG);

/* Undefined type representing a pointer with type information in the bottom
 * two bits.
 */
#[repr(C)]
pub struct assoc_array_ptr {
    _opaque: [u8; 0],
}

/* An N-way node in the tree. */
#[repr(C)]
pub struct assoc_array_node {
    pub back_pointer: *mut assoc_array_ptr,
    pub parent_slot: u8,
    pub slots: [*mut assoc_array_ptr; ASSOC_ARRAY_FAN_OUT],
    pub nr_leaves_on_branch: ::core::ffi::c_ulong,
}

/* A shortcut through the index space out to where a collection of nodes/leaves
 * with the same IDs live.
 */
#[repr(C)]
pub struct assoc_array_shortcut {
    pub back_pointer: *mut assoc_array_ptr,
    pub parent_slot: ::core::ffi::c_int,
    pub skip_to_level: ::core::ffi::c_int,
    pub next_node: *mut assoc_array_ptr,
    pub index_key: [::core::ffi::c_ulong; 0],
}

/* Preallocation cache. */
#[repr(C)]
pub struct assoc_array_edit {
    pub rcu: rcu_head,
    pub array: *mut assoc_array,
    pub ops: *const assoc_array_ops,
    pub ops_for_excised_subtree: *const assoc_array_ops,
    pub leaf: *mut assoc_array_ptr,
    pub leaf_p: *mut *mut assoc_array_ptr,
    pub dead_leaf: *mut assoc_array_ptr,
    pub new_meta: [*mut assoc_array_ptr; 3],
    pub excised_meta: [*mut assoc_array_ptr; 1],
    pub excised_subtree: *mut assoc_array_ptr,
    pub set_backpointers: [*mut *mut assoc_array_ptr; ASSOC_ARRAY_FAN_OUT],
    pub set_backpointers_to: *mut assoc_array_ptr,
    pub adjust_count_on: *mut assoc_array_node,
    pub adjust_count_by: ::core::ffi::c_long,
    pub set: [assoc_array_edit_set; 2],
    pub set_parent_slot: [assoc_array_edit_set_parent_slot; 1],
    pub segment_cache: [u8; ASSOC_ARRAY_FAN_OUT + 1],
}

#[repr(C)]
pub struct assoc_array_edit_set {
    pub ptr: *mut *mut assoc_array_ptr,
    pub to: *mut assoc_array_ptr,
}

#[repr(C)]
pub struct assoc_array_edit_set_parent_slot {
    pub p: *mut u8,
    pub to: u8,
}

pub const ASSOC_ARRAY_PTR_TYPE_MASK: usize = 0x1;
pub const ASSOC_ARRAY_PTR_LEAF_TYPE: usize = 0x0;
pub const ASSOC_ARRAY_PTR_META_TYPE: usize = 0x1;
pub const ASSOC_ARRAY_PTR_SUBTYPE_MASK: usize = 0x2;
pub const ASSOC_ARRAY_PTR_NODE_SUBTYPE: usize = 0x0;
pub const ASSOC_ARRAY_PTR_SHORTCUT_SUBTYPE: usize = 0x2;

#[inline]
pub unsafe fn assoc_array_ptr_is_meta(x: *const assoc_array_ptr) -> bool {
    x as usize & ASSOC_ARRAY_PTR_TYPE_MASK != 0
}

#[inline]
pub unsafe fn assoc_array_ptr_is_leaf(x: *const assoc_array_ptr) -> bool {
    !assoc_array_ptr_is_meta(x)
}

#[inline]
pub unsafe fn assoc_array_ptr_is_shortcut(x: *const assoc_array_ptr) -> bool {
    x as usize & ASSOC_ARRAY_PTR_SUBTYPE_MASK != 0
}

#[inline]
pub unsafe fn assoc_array_ptr_is_node(x: *const assoc_array_ptr) -> bool {
    !assoc_array_ptr_is_shortcut(x)
}

#[inline]
pub unsafe fn assoc_array_ptr_to_leaf(x: *const assoc_array_ptr) -> *mut ::core::ffi::c_void {
    (x as usize & !ASSOC_ARRAY_PTR_TYPE_MASK) as *mut ::core::ffi::c_void
}

#[inline]
pub unsafe fn __assoc_array_ptr_to_meta(x: *const assoc_array_ptr) -> usize {
    x as usize & !(ASSOC_ARRAY_PTR_SUBTYPE_MASK | ASSOC_ARRAY_PTR_TYPE_MASK)
}

#[inline]
pub unsafe fn assoc_array_ptr_to_node(x: *const assoc_array_ptr) -> *mut assoc_array_node {
    __assoc_array_ptr_to_meta(x) as *mut assoc_array_node
}

#[inline]
pub unsafe fn assoc_array_ptr_to_shortcut(x: *const assoc_array_ptr) -> *mut assoc_array_shortcut {
    __assoc_array_ptr_to_meta(x) as *mut assoc_array_shortcut
}

#[inline]
pub unsafe fn __assoc_array_x_to_ptr(p: *const ::core::ffi::c_void, t: usize) -> *mut assoc_array_ptr {
    ((p as usize) | t) as *mut assoc_array_ptr
}

#[inline]
pub unsafe fn assoc_array_leaf_to_ptr(p: *const ::core::ffi::c_void) -> *mut assoc_array_ptr {
    __assoc_array_x_to_ptr(p, ASSOC_ARRAY_PTR_LEAF_TYPE)
}

#[inline]
pub unsafe fn assoc_array_node_to_ptr(p: *const assoc_array_node) -> *mut assoc_array_ptr {
    __assoc_array_x_to_ptr(p as *const ::core::ffi::c_void,
                           ASSOC_ARRAY_PTR_META_TYPE | ASSOC_ARRAY_PTR_NODE_SUBTYPE)
}

#[inline]
pub unsafe fn assoc_array_shortcut_to_ptr(p: *const assoc_array_shortcut) -> *mut assoc_array_ptr {
    __assoc_array_x_to_ptr(p as *const ::core::ffi::c_void,
                           ASSOC_ARRAY_PTR_META_TYPE | ASSOC_ARRAY_PTR_SHORTCUT_SUBTYPE)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
