/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct nf_flow_table_stat {
    pub count_wq_add: u32,
    pub count_wq_del: u32,
    pub count_wq_stats: u32,
}

#[repr(C)]
pub struct netns_ft {
    // C: struct nf_flow_table_stat __percpu *stat;
    pub stat: *mut nf_flow_table_stat,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
