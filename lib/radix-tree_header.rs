// SPDX-License-Identifier: GPL-2.0+
/* radix-tree helpers that are only shared with xarray */

#[repr(C)]
pub struct kmem_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}

extern "C" {
    pub static mut radix_tree_node_cachep: *mut kmem_cache;
    pub fn radix_tree_node_rcu_free(head: *mut rcu_head);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
