/* SPDX-License-Identifier: GPL-2.0 */
/**
 * union_find.h - union-find data structure implementation
 *
 * This header provides functions and structures to implement the union-find
 * data structure. The union-find data structure is used to manage disjoint
 * sets and supports efficient union and find operations.
 *
 * See Documentation/core-api/union_find.rst for documentation and samples.
 */

#[repr(C)]
pub struct uf_node {
    pub parent: *mut uf_node,
    pub rank: u32,
}

/* This macro is used for static initialization of a union-find node. */
#[macro_export]
macro_rules! UF_INIT_NODE {
    ($node:ident) => {
        uf_node {
            parent: core::ptr::addr_of_mut!($node),
            rank: 0,
        }
    };
}

/**
 * uf_node_init - Initialize a union-find node
 * @node: pointer to the union-find node to be initialized
 *
 * This function sets the parent of the node to itself and
 * initializes its rank to 0.
 */
#[inline]
pub unsafe fn uf_node_init(node: *mut uf_node) {
    (*node).parent = node;
    (*node).rank = 0;
}

/* find the root of a node */
unsafe extern "C" {
    pub fn uf_find(node: *mut uf_node) -> *mut uf_node;
}

/* Merge two intersecting nodes */
unsafe extern "C" {
    pub fn uf_union(node1: *mut uf_node, node2: *mut uf_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
