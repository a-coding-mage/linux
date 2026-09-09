/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2025 Christian Brauner <brauner@kernel.org> */

// Dependencies supplied by the corresponding Linux headers:
// rb_root, list_head, and atomic_t.

/**
 * struct ns_tree_root - Root of a namespace tree
 * @ns_rb: Red-black tree root for efficient lookups
 * @ns_list_head: List head for sequential iteration
 *
 * Each namespace tree maintains both an rbtree (for O(log n) lookups)
 * and a list (for efficient sequential iteration). The list is kept in
 * the same sorted order as the rbtree.
 */
#[repr(C)]
pub struct ns_tree_root {
    pub ns_rb: rb_root,
    pub ns_list_head: list_head,
}

/**
 * struct ns_tree_node - Node in a namespace tree
 * @ns_node: Red-black tree node
 * @ns_list_entry: List entry for sequential iteration
 *
 * Represents a namespace's position in a tree. Each namespace has
 * multiple tree nodes for different trees (unified, per-type, owner).
 */
#[repr(C)]
pub struct ns_tree_node {
    pub ns_node: rb_node,
    pub ns_list_entry: list_head,
}

/**
 * struct ns_tree - Namespace tree nodes and active reference count
 * @ns_id: Unique namespace identifier
 * @__ns_ref_active: Active reference count (do not use directly)
 * @ns_unified_node: Node in the global namespace tree
 * @ns_tree_node: Node in the per-type namespace tree
 * @ns_owner_node: Node in the owner namespace's tree of owned namespaces
 * @ns_owner_root: Root of the tree of namespaces owned by this namespace
 *                 (only used when this namespace is an owner)
 */
#[repr(C)]
pub struct ns_tree {
    pub ns_id: u64,
    pub __ns_ref_active: atomic_t,
    pub ns_unified_node: ns_tree_node,
    pub ns_tree_node: ns_tree_node,
    pub ns_owner_node: ns_tree_node,
    pub ns_owner_root: ns_tree_root,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
