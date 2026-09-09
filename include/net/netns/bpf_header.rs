/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BPF programs attached to network namespace
 */

// Dependency supplied by the surrounding translated code: <linux/list.h>.

pub enum bpf_prog {}
pub enum bpf_prog_array {}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netns_bpf_attach_type {
    NETNS_BPF_INVALID = -1,
    NETNS_BPF_FLOW_DISSECTOR = 0,
    NETNS_BPF_SK_LOOKUP,
    MAX_NETNS_BPF_ATTACH_TYPE,
}

#[repr(C)]
pub struct netns_bpf {
    /* Array of programs to run compiled from progs or links */
    // __rcu annotation retained semantically as a raw pointer.
    pub run_array: [*mut bpf_prog_array; MAX_NETNS_BPF_ATTACH_TYPE as usize],
    pub progs: [*mut bpf_prog; MAX_NETNS_BPF_ATTACH_TYPE as usize],
    pub links: [list_head; MAX_NETNS_BPF_ATTACH_TYPE as usize],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
