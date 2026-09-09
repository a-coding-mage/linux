/* SPDX-License-Identifier: GPL-2.0 */
/*
 * nexthops in net namespaces
 */

// <linux/notifier.h>
// <linux/rbtree.h>

#[repr(C)]
pub struct netns_nexthop {
    pub rb_root: rb_root, // tree of nexthops by id
    pub devhash: *mut hlist_head, // nexthops by device

    pub seq: ::core::ffi::c_uint, // protected by rtnl_mutex
    pub last_id_allocated: u32,
    pub notifier_chain: blocking_notifier_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
