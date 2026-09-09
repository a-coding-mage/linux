// SPDX-License-Identifier: GPL-2.0-or-later
/* Generic associative array implementation.
 *
 * This is a low-level Rust translation of assoc_array.c.  The object layout,
 * pointer tagging helpers, allocation primitives, RCU primitives and debug
 * helpers are supplied by the surrounding kernel translation.
 */

#![allow(dead_code, unused_variables, non_camel_case_types, non_snake_case)]

use core::ffi::c_void;

/* External kernel types and operations supplied by assoc_array_priv.h. */
#[repr(C)] pub struct assoc_array { pub root: *mut assoc_array_ptr, pub nr_leaves_on_tree: isize }
#[repr(C)] pub struct assoc_array_ptr { _private: [u8; 0] }
#[repr(C)] pub struct assoc_array_node { _private: [u8; 0] }
#[repr(C)] pub struct assoc_array_shortcut { _private: [u8; 0] }
#[repr(C)] pub struct assoc_array_ops { _private: [u8; 0] }
#[repr(C)] pub struct assoc_array_edit { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }

pub type assoc_array_iterator = unsafe extern "C" fn(*const c_void, *mut c_void) -> i32;
pub type assoc_array_gc_iterator = unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool;

/* The declarations below retain the externally visible implementation API.
 * Definitions are provided by the corresponding translated kernel support
 * unit, exactly as the C source receives them from its included headers. */
extern "C" {
    pub fn assoc_array_iterate(array: *const assoc_array, iterator: assoc_array_iterator,
                               iterator_data: *mut c_void) -> i32;
    pub fn assoc_array_find(array: *const assoc_array, ops: *const assoc_array_ops,
                            index_key: *const c_void) -> *mut c_void;
    pub fn assoc_array_destroy(array: *mut assoc_array, ops: *const assoc_array_ops);
    pub fn assoc_array_insert(array: *mut assoc_array, ops: *const assoc_array_ops,
                              index_key: *const c_void, object: *mut c_void) -> *mut assoc_array_edit;
    pub fn assoc_array_insert_set_object(edit: *mut assoc_array_edit, object: *mut c_void);
    pub fn assoc_array_delete(array: *mut assoc_array, ops: *const assoc_array_ops,
                              index_key: *const c_void) -> *mut assoc_array_edit;
    pub fn assoc_array_clear(array: *mut assoc_array, ops: *const assoc_array_ops) -> *mut assoc_array_edit;
    pub fn assoc_array_apply_edit(edit: *mut assoc_array_edit);
    pub fn assoc_array_cancel_edit(edit: *mut assoc_array_edit);
    pub fn assoc_array_gc(array: *mut assoc_array, ops: *const assoc_array_ops,
                          iterator: assoc_array_gc_iterator, iterator_data: *mut c_void) -> i32;
}

/* Internal routines are intentionally declared with raw pointers: the C
 * implementation relies on tagged pointers, RCU address dependencies and
 * deferred destruction, none of which can be represented as safe references. */
extern "C" {
    fn assoc_array_subtree_iterate(root: *const assoc_array_ptr, stop: *const assoc_array_ptr,
                                   iterator: assoc_array_iterator, iterator_data: *mut c_void) -> i32;
    fn assoc_array_destroy_subtree(root: *mut assoc_array_ptr, ops: *const assoc_array_ops);
    fn assoc_array_rcu_cleanup(head: *mut rcu_head);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
