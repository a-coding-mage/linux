/* SPDX-License-Identifier: GPL-2.0-only */

#[repr(C)]
pub struct cgroup_value {
    pub egress_pkts: __u32,
    pub ingress_pkts: __u32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
