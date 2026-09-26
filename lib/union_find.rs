// SPDX-License-Identifier: GPL-2.0
//! Union by unsigned rank and path splitting, with the original C ABI.

use kernel::bindings::uf_node;

/**
 * uf_find - Find the root of a node and perform path compression
 * @node: the node to find the root of
 *
 * This function returns the root of the node by following the parent
 * pointers. It also performs path compression, making the tree shallower.
 *
 * Returns the root node of the set containing node.
 */
/// Finds a root, redirecting each visited node to its grandparent.
///
/// # Safety
///
/// `node` must belong to a finite, initialized forest whose nodes remain live
/// and at fixed addresses. The caller must exclusively access the forest.
#[no_mangle]
pub unsafe extern "C" fn uf_find(mut node: *mut uf_node) -> *mut uf_node {
    // SAFETY: the caller guarantees all parent links reach a live root and
    // that these raw-pointer updates cannot race with another forest access.
    unsafe {
        while (*node).parent != node {
            let parent = (*node).parent;
            (*node).parent = (*parent).parent;
            node = parent;
        }
    }
    node
}

/**
 * uf_union - Merge two sets, using union by rank
 * @node1: the first node
 * @node2: the second node
 *
 * This function merges the sets containing node1 and node2, by comparing
 * the ranks to keep the tree balanced.
 */
/// Merges two sets, retaining the first root when their ranks are equal.
///
/// # Safety
///
/// Both nodes must satisfy [`uf_find`]'s requirements. Their forests may
/// overlap, but the caller must exclusively access both during this call.
#[no_mangle]
pub unsafe extern "C" fn uf_union(node1: *mut uf_node, node2: *mut uf_node) {
    // SAFETY: both forests are valid and exclusively accessible by contract.
    unsafe {
        let root1 = uf_find(node1);
        let root2 = uf_find(node2);

        if root1 == root2 {
            return;
        }

        if (*root1).rank < (*root2).rank {
            (*root1).parent = root2;
        } else if (*root1).rank > (*root2).rank {
            (*root2).parent = root1;
        } else {
            (*root2).parent = root1;
            (*root1).rank = (*root1).rank.wrapping_add(1);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
