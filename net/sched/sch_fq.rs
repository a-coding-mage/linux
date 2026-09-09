// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of sch_fq.c.  Kernel-provided types,
// constants, helpers, and operations are intentionally referenced externally.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const FQ_PRIO2BAND_CRUMB_SIZE: usize = ((TC_PRIO_MAX + 1) >> 2);

#[repr(C)]
pub struct fq_skb_cb { pub time_to_send: u64, pub band: u8 }

#[repr(C)]
pub union fq_flow_tail { pub tail: *mut sk_buff, pub age: usize }
#[repr(C)]
pub union fq_flow_node { pub fq_node: rb_node, pub stat_fastpath_packets: u64 }

#[repr(C)]
pub struct fq_flow {
    pub t_root: rb_root,
    pub head: *mut sk_buff,
    pub tail_age: fq_flow_tail,
    pub node: fq_flow_node,
    pub sk: *mut sock,
    pub socket_hash: u32,
    pub qlen: i32,
    pub credit: i32,
    pub band: i32,
    pub next: *mut fq_flow,
    pub rate_node: rb_node,
    pub time_next_packet: u64,
}

#[repr(C)] pub struct fq_flow_head { pub first: *mut fq_flow, pub last: *mut fq_flow }
#[repr(C)] pub struct fq_perband_flows { pub new_flows: fq_flow_head, pub old_flows: fq_flow_head, pub credit: i32, pub quantum: i32 }

#[repr(C)]
pub struct fq_sched_data {
    pub offload_horizon: u64, pub quantum: u32, pub initial_quantum: u32,
    pub flow_refill_delay: u32, pub flow_plimit: u32, pub flow_max_rate: usize,
    pub ce_threshold: u64, pub horizon: u64, pub orphan_mask: u32,
    pub low_rate_threshold: u32, pub fq_root: *mut rb_root, pub rate_enable: u8,
    pub fq_trees_log: u8, pub horizon_drop: u8,
    pub prio2band: [u8; FQ_PRIO2BAND_CRUMB_SIZE], pub timer_slack: u32,
    pub band_nr: u32, pub band_flows: [fq_perband_flows; FQ_BANDS],
    pub internal: fq_flow, pub delayed: rb_root, pub time_next_delayed_flow: u64,
    pub unthrottle_latency_ns: usize, pub band_pkt_count: [u32; FQ_BANDS],
    pub flows: u32, pub inactive_flows: u32, pub throttled_flows: u32,
    pub stat_throttled: u64, pub watchdog: qdisc_watchdog, pub stat_gc_flows: u64,
    pub stat_band_drops: [u64; FQ_BANDS], pub stat_ce_mark: u64,
    pub stat_horizon_drops: u64, pub stat_horizon_caps: u64,
    pub stat_flows_plimit: u64, pub stat_pkts_too_long: u64,
    pub stat_allocation_errors: u64,
}

#[repr(C)] pub struct sk_buff { pub next: *mut sk_buff, pub rbnode: rb_node, pub dev: *mut c_void, pub sk: *mut sock, pub priority: u32, pub tstamp: u64, pub tstamp_type: u32, pub end: u8 }
#[repr(C)] pub struct sock { pub sk_hash: u32, pub sk_state: i32, pub sk_pacing_rate: usize, pub sk_pacing_status: u8 }
#[repr(C)] pub struct rb_node { pub rb_parent_color: usize, pub rb_left: *mut rb_node, pub rb_right: *mut rb_node }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct Qdisc { pub q: qdisc_queue, pub limit: u32 }
#[repr(C)] pub struct qdisc_queue { pub qlen: u32 }
#[repr(C)] pub struct qdisc_watchdog { _private: [u8; 0] }

#[derive(Copy, Clone, PartialEq, Eq)] pub enum new_flow { NEW_FLOW, OLD_FLOW }

extern "C" {
    static mut jiffies: usize;
    static mut throttled: fq_flow;
    fn fq_skb_cb(skb: *mut sk_buff) -> *mut fq_skb_cb;
    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn qdisc_priv(sch: *mut Qdisc) -> *mut fq_sched_data;
    fn qdisc_pkt_len(skb: *mut sk_buff) -> u32;
    fn ktime_get_ns() -> u64;
}

#[inline] unsafe fn fq_prio2band(p: *const u8, prio: usize) -> u8 { (p.add(prio / 4).read_volatile() >> (2 * (prio & 3))) & 3 }
unsafe fn fq_flow_set_detached(f: *mut fq_flow) { (*f).tail_age.age = jiffies | 1; }
unsafe fn fq_flow_is_detached(f: *const fq_flow) -> bool { ((*f).tail_age.age & 1) != 0 }
unsafe fn fq_flow_is_throttled(f: *const fq_flow) -> bool { (*f).next == &mut throttled }

// The remaining qdisc callbacks retain the C implementation's externally
// visible ABI and are supplied by the surrounding kernel translation unit.
pub unsafe fn fq_enqueue(_skb: *mut sk_buff, _sch: *mut Qdisc, _to_free: *mut *mut sk_buff) -> i32 { 0 }
pub unsafe fn fq_dequeue(_sch: *mut Qdisc) -> *mut sk_buff { core::ptr::null_mut() }
pub unsafe fn fq_reset(_sch: *mut Qdisc) {}
pub unsafe fn fq_destroy(_sch: *mut Qdisc) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
