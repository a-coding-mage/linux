/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * nodemanager.h
 *
 * Function prototypes
 *
 * Copyright (C) 2004 Oracle.  All rights reserved.
 */

// Dependency provided by ocfs2_nodemanager.h.
// Linux configfs and rbtree dependencies are provided externally.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum o2nm_fence_method {
    O2NM_FENCE_RESET = 0,
    O2NM_FENCE_PANIC,
    O2NM_FENCE_METHODS, /* Number of fence methods */
}

#[repr(C)]
pub struct o2nm_node {
    pub nd_lock: spinlock_t,
    pub nd_item: config_item,
    pub nd_name: [core::ffi::c_char; (O2NM_MAX_NAME_LEN + 1) as usize], /* replace? */
    pub nd_num: __u8,
    /* only one address per node, as attributes, for now. */
    pub nd_ipv4_address: __be32,
    pub nd_ipv4_port: __be16,
    pub nd_ip_node: rb_node,
    /* there can be only one local node for now */
    pub nd_local: core::ffi::c_int,
    pub nd_set_attributes: core::ffi::c_ulong,
}

#[repr(C)]
pub struct o2nm_cluster {
    pub cl_group: config_group,
    // C bit-field: unsigned cl_has_local:1.
    pub cl_has_local: core::ffi::c_uint,
    pub cl_local_node: u8,
    pub cl_nodes_lock: rwlock_t,
    pub cl_nodes: [*mut o2nm_node; O2NM_MAX_NODES as usize],
    pub cl_node_ip_tree: rb_root,
    pub cl_idle_timeout_ms: core::ffi::c_uint,
    pub cl_keepalive_delay_ms: core::ffi::c_uint,
    pub cl_reconnect_delay_ms: core::ffi::c_uint,
    pub cl_fence_method: o2nm_fence_method,

    /* this bitmap is part of a hack for disk bitmap.. will go eventually. - zab */
    pub cl_nodes_bitmap: [core::ffi::c_ulong; BITS_TO_LONGS(O2NM_MAX_NODES) as usize],
}

extern "C" {
    pub static mut o2nm_single_cluster: *mut o2nm_cluster;

    pub fn o2nm_this_node() -> u8;

    pub fn o2nm_configured_node_map(
        map: *mut core::ffi::c_ulong,
        bytes: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn o2nm_get_node_by_num(node_num: u8) -> *mut o2nm_node;
    pub fn o2nm_get_node_by_ip(addr: __be32) -> *mut o2nm_node;
    pub fn o2nm_node_get(node: *mut o2nm_node);
    pub fn o2nm_node_put(node: *mut o2nm_node);

    pub fn o2nm_depend_item(item: *mut config_item) -> core::ffi::c_int;
    pub fn o2nm_depend_item_unlocked(item: *mut config_item) -> core::ffi::c_int;
    pub fn o2nm_undepend_item(item: *mut config_item);
    pub fn o2nm_depend_node(node_num: u8) -> core::ffi::c_int;
    pub fn o2nm_undepend_node(node_num: u8);
    pub fn o2nm_depend_this_node() -> core::ffi::c_int;
    pub fn o2nm_undepend_this_node();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
