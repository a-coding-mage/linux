/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: __u32 is supplied by the Linux types bindings.

#[repr(i32)]
pub enum xt_cluster_flags {
    XT_CLUSTER_F_INV = 1 << 0,
}

#[repr(C)]
pub struct xt_cluster_match_info {
    pub total_nodes: u32,
    pub node_mask: u32,
    pub hash_seed: u32,
    pub flags: u32,
}

pub const XT_CLUSTER_NODES_MAX: u32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
