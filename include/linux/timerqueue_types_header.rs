/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by linux/rbtree_types.h and linux/types.h.

#[repr(C)]
pub struct timerqueue_node {
    pub node: rb_node,
    pub expires: ktime_t,
}

#[repr(C)]
pub struct timerqueue_head {
    pub rb_root: rb_root_cached,
}

#[repr(C)]
pub struct timerqueue_linked_node {
    pub node: rb_node_linked,
    pub expires: ktime_t,
}

#[repr(C)]
pub struct timerqueue_linked_head {
    pub rb_root: rb_root_linked,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
