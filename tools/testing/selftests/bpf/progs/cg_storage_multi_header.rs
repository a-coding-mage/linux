/* SPDX-License-Identifier: GPL-2.0-only */

#[repr(C)]
pub struct cgroup_value {
    pub egress_pkts: __u32,
    pub ingress_pkts: __u32,
}
