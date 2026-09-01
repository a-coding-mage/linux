/* SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause */

/* Translated from a C header. C include guard / #pragma once omitted. */

pub const RB_MAXLVL_PRINT: i32 = 16;

#[repr(C)]
pub struct rbnode {
    pub parent: *mut rbnode,
    pub __bindgen_anon_1: rbnode__bindgen_ty_1,
    pub key: u64,
    /* Used as a linked list or to store KV pairs. */
    pub __bindgen_anon_2: rbnode__bindgen_ty_2,
    pub is_red: bool,
}

#[repr(C)]
pub union rbnode__bindgen_ty_1 {
    pub __bindgen_anon_1: rbnode__bindgen_ty_1__bindgen_ty_1,
    pub child: [*mut rbnode; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rbnode__bindgen_ty_1__bindgen_ty_1 {
    pub left: *mut rbnode,
    pub right: *mut rbnode,
}

#[repr(C)]
pub union rbnode__bindgen_ty_2 {
    pub next: *mut rbnode,
    pub value: u64,
}

/*
 * Does the rbtree allocate its own nodes, or do they get
 * allocated by the caller?
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rbtree_alloc {
    RB_ALLOC,
    RB_NOALLOC,
}

/*
 * Specify the behavior of rbtree insertions when the key is
 * already present in the tree.
 *
 * RB_DEFAULT: Default behavior, reject the new insert.
 *
 * RB_UPDATE: Update the existing value in the rbtree.
 * This updates the node itself, not just the value in
 * the existing node.
 *
 * RB_DUPLICATE: Allow nodes with identical keys in the rbtree.
 * Finding/popping/removing a key acts on any of the nodes
 * with the appropriate key - there is no ordering by time
 * of insertion.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rbtree_insert_mode {
    RB_DEFAULT,
    RB_UPDATE,
    RB_DUPLICATE,
}

#[repr(C)]
pub struct rbtree {
    pub root: *mut rbnode,
    pub alloc: rbtree_alloc,
    pub insert: rbtree_insert_mode,
}

/*
 * Original declarations were guarded by #ifdef __BPF__.
 * The Rust translation preserves the external C interface declarations.
 */
unsafe extern "C" {
    pub fn rb_create(alloc: rbtree_alloc, insert: rbtree_insert_mode) -> *mut rbtree;

    pub fn rb_destroy(rbtree: *mut rbtree) -> i32;
    pub fn rb_insert(rbtree: *mut rbtree, key: u64, value: u64) -> i32;
    pub fn rb_remove(rbtree: *mut rbtree, key: u64) -> i32;
    pub fn rb_find(rbtree: *mut rbtree, key: u64, value: *mut u64) -> i32;
    pub fn rb_print(rbtree: *mut rbtree) -> i32;
    pub fn rb_least(rbtree: *mut rbtree, key: *mut u64, value: *mut u64) -> i32;
    pub fn rb_pop(rbtree: *mut rbtree, key: *mut u64, value: *mut u64) -> i32;

    pub fn rb_insert_node(rbtree: *mut rbtree, node: *mut rbnode) -> i32;
    pub fn rb_remove_node(rbtree: *mut rbtree, node: *mut rbnode) -> i32;

    pub fn rb_node_alloc(key: u64, value: u64) -> *mut rbnode;
    pub fn rb_node_free(rbnode: *mut rbnode);

    pub fn rb_integrity_check(rbtree: *mut rbtree) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
