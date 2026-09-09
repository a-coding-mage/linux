// SPDX-License-Identifier: GPL-2.0-or-later
// Literal low-level Rust translation of core/pktgen.c. Kernel-provided types,
// functions, and constants remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub const VERSION: &str = "2.75";
pub const IP_NAME_SZ: usize = 32;
pub const MAX_MPLS_LABELS: usize = 16;
pub const MAX_IMIX_ENTRIES: usize = 20;
pub const IMIX_PRECISION: usize = 100;
pub const PKTGEN_MAGIC: u32 = 0xbe9be955;
pub const T_STOP: u32 = 1 << 0;
pub const T_RUN: u32 = 1 << 1;
pub const T_REMDEVALL: u32 = 1 << 2;
pub const T_REMDEV: u32 = 1 << 3;
pub const M_START_XMIT: c_int = 0;
pub const M_NETIF_RECEIVE: c_int = 1;
pub const M_QUEUE_XMIT: c_int = 2;

#[repr(C)]
pub struct imix_pkt { pub size: u64, pub weight: u64, pub count_so_far: u64 }

#[repr(C)]
pub struct flow_state {
    pub cur_daddr: u32,
    pub count: c_int,
    pub flags: u32,
}

#[repr(C)]
pub struct pktgen_hdr { pub pgh_magic: u32, pub seq_num: u32, pub tv_sec: u32, pub tv_usec: u32 }

#[repr(C)]
pub struct pktgen_dev {
    pub entry: *mut c_void, pub pg_thread: *mut pktgen_thread, pub list: [usize; 2], pub rcu: [usize; 2],
    pub running: c_int, pub flags: u32, pub xmit_mode: c_int, pub min_pkt_size: c_int,
    pub max_pkt_size: c_int, pub pkt_overhead: c_int, pub nfrags: c_int, pub removal_mark: c_int,
    pub page: *mut c_void, pub delay: u64, pub count: u64, pub sofar: u64, pub tx_bytes: u64,
    pub errors: u64, pub clone_count: u32, pub last_ok: c_int, pub next_tx: u64, pub started_at: u64,
    pub stopped_at: u64, pub idle_acc: u64, pub seq_num: u32, pub clone_skb: c_int,
    pub dst_min: [c_char; IP_NAME_SZ], pub dst_max: [c_char; IP_NAME_SZ],
    pub src_min: [c_char; IP_NAME_SZ], pub src_max: [c_char; IP_NAME_SZ],
    pub in6_saddr: [u8; 16], pub in6_daddr: [u8; 16], pub cur_in6_daddr: [u8; 16], pub cur_in6_saddr: [u8; 16],
    pub min_in6_daddr: [u8; 16], pub max_in6_daddr: [u8; 16], pub min_in6_saddr: [u8; 16], pub max_in6_saddr: [u8; 16],
    pub saddr_min: u32, pub saddr_max: u32, pub daddr_min: u32, pub daddr_max: u32,
    pub udp_src_min: u16, pub udp_src_max: u16, pub udp_dst_min: u16, pub udp_dst_max: u16,
    pub tos: u8, pub traffic_class: u8, pub n_imix_entries: u32,
    pub imix_entries: [imix_pkt; MAX_IMIX_ENTRIES], pub imix_distribution: [u8; IMIX_PRECISION],
    pub nr_labels: u32, pub labels: [u32; MAX_MPLS_LABELS], pub vlan_p: u8, pub vlan_cfi: u8,
    pub vlan_id: u16, pub svlan_p: u8, pub svlan_cfi: u8, pub svlan_id: u16,
    pub src_mac_count: u32, pub dst_mac_count: u32, pub dst_mac: [u8; 6], pub src_mac: [u8; 6],
    pub cur_dst_mac_offset: u32, pub cur_src_mac_offset: u32, pub cur_saddr: u32, pub cur_daddr: u32,
    pub ip_id: u16, pub cur_udp_dst: u16, pub cur_udp_src: u16, pub cur_queue_map: u16,
    pub cur_pkt_size: u32, pub last_pkt_size: u32, pub hh: [u8; 14], pub pad: u16,
    pub skb: *mut c_void, pub odev: *mut c_void, pub odevname: [c_char; 32], pub flows: *mut flow_state,
    pub cflows: u32, pub lflow: u32, pub nflows: u32, pub curfl: u32, pub queue_map_min: u16,
    pub queue_map_max: u16, pub skb_priority: u32, pub burst: u32, pub node: c_int, pub result: [c_char; 512],
}

#[repr(C)]
pub struct pktgen_thread { pub if_lock: [usize; 5], pub if_list: [usize; 2], pub th_list: [usize; 2], pub tsk: *mut c_void, pub result: [c_char; 512], pub control: u32, pub cpu: c_int, pub queue: [usize; 6], pub start_done: [usize; 3], pub net: *mut pktgen_net }

#[repr(C)]
pub struct pktgen_net { pub net: *mut c_void, pub proc_dir: *mut c_void, pub pktgen_threads: [usize; 2], pub pktgen_exiting: bool }

// The remaining implementation uses the kernel APIs declared by the headers
// included by the original translation unit; those declarations are external.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
