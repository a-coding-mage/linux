// SPDX-License-Identifier: GPL-2.0
// Dependency: struct uf_node is supplied by linux/union_find.h.

#[repr(C)]
pub struct uf_node {
    pub parent: *mut uf_node,
    pub rank: i32,
}

/**
 * uf_find - Find the root of a node and perform path compression
 * @node: the node to find the root of
 *
 * This function returns the root of the node by following the parent
 * pointers. It also performs path compression, making the tree shallower.
 *
 * Returns the root node of the set containing node.
 */
pub unsafe fn uf_find(mut node: *mut uf_node) -> *mut uf_node {
    let mut parent: *mut uf_node;

    while (*node).parent != node {
        parent = (*node).parent;
        (*node).parent = (*parent).parent;
        node = parent;
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
pub unsafe fn uf_union(node1: *mut uf_node, node2: *mut uf_node) {
    let root1: *mut uf_node = uf_find(node1);
    let root2: *mut uf_node = uf_find(node2);

    if root1 == root2 {
        return;
    }

    if (*root1).rank < (*root2).rank {
        (*root1).parent = root2;
    } else if (*root1).rank > (*root2).rank {
        (*root2).parent = root1;
    } else {
        (*root2).parent = root1;
        (*root1).rank += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
