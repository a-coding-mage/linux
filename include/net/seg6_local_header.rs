/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  SR-IPv6 implementation
 *
 *  Authors:
 *  David Lebrun <david.lebrun@uclouvain.be>
 *  eBPF support: Mathieu Xhonneux <m.xhonneux@gmail.com>
 */

// Translated from net/seg6_local.h.
// C dependencies supplied by the surrounding kernel translation are referenced
// here but are not defined in this file.

unsafe extern "C" {
    pub fn seg6_lookup_nexthop(
        skb: *mut sk_buff,
        nhaddr: *mut in6_addr,
        tbl_id: u32,
    ) -> i32;
    pub fn seg6_bpf_has_valid_srh(skb: *mut sk_buff) -> bool;
}

#[repr(C)]
pub struct seg6_bpf_srh_state {
    pub bh_lock: local_lock_t,
    pub srh: *mut ipv6_sr_hdr,
    pub hdrlen: u16,
    pub valid: bool,
}

// DECLARE_PER_CPU(struct seg6_bpf_srh_state, seg6_bpf_srh_states);
// The per-CPU storage declaration is represented as an external C symbol;
// per-CPU addressing remains a dependency of the surrounding kernel runtime.
unsafe extern "C" {
    pub static mut seg6_bpf_srh_states: seg6_bpf_srh_state;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
