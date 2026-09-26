/* SPDX-License-Identifier: GPL-2.0 */
//! Original union-find declarations and initialization at stable addresses.
/**
 * union_find.h - union-find data structure implementation
 *
 * This header provides functions and structures to implement the union-find
 * data structure. The union-find data structure is used to manage disjoint
 * sets and supports efficient union and find operations.
 *
 * See Documentation/core-api/union_find.rst for documentation and samples.
 */
pub use kernel::bindings::{uf_find, uf_node, uf_union};

/* This macro is used for static initialization of a union-find node. */
/// Initializes a statically located union-find node with its own address.
#[macro_export]
macro_rules! UF_INIT_NODE {
    ($node:expr) => {
        $crate::union_find::uf_node_initializer(core::ptr::addr_of_mut!($node))
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
/// Initializes a node at its final address as a singleton set.
///
/// # Safety
///
/// `node` must be live, writable, and exclusively accessible. Its address
/// must remain stable while any union-find operation or parent refers to it.
pub unsafe fn uf_node_init(node: *mut uf_node) {
    // SAFETY: the caller supplies exclusive access to a live, writable node.
    unsafe {
        (*node).parent = node;
        (*node).rank = 0;
    }
}

/// Creates the initializer for a node stored at `node`.
///
/// Store the result at that address before using the union-find operations.
pub const fn uf_node_initializer(node: *mut uf_node) -> uf_node {
    uf_node {
        parent: node,
        rank: 0,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
