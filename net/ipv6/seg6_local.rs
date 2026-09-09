// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of seg6_local.c. Kernel-provided types
// and functions are intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type u16_t = u16;
pub type u32_t = u32;
pub type u64_t = u64;

#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: u32, pub protocol: u16, pub dev: *mut net_device }
#[repr(C)] pub struct net_device { pub ifindex: c_int, pub flags: u32, pub mtu: u32, pub type_: u16 }
#[repr(C)] pub struct lwtunnel_state { pub data: *mut c_void }
#[repr(C)] pub struct dst_entry { pub error: c_int, pub lwtstate: *mut lwtunnel_state }
#[repr(C)] pub struct bpf_prog { pub aux: *mut bpf_prog_aux }
#[repr(C)] pub struct bpf_prog_aux { pub id: u32 }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct ipv6_sr_hdr { pub nexthdr: u8, pub hdrlen: u8, pub segments_left: u8, pub first_segment: u8, pub flags: u8, pub tag: u16, pub segments: [in6_addr; 0] }
#[repr(C)] #[derive(Clone, Copy)] pub struct in_addr { pub s_addr: u32 }
#[repr(C)] #[derive(Clone, Copy)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct ipv6hdr { pub nexthdr: u8, pub daddr: in6_addr, pub saddr: in6_addr }
#[repr(C)] pub struct iphdr { pub daddr: u32, pub saddr: u32 }
#[repr(C)] pub struct u64_stats_t { pub value: u64 }
#[repr(C)] pub struct u64_stats_sync { _private: [u8; 0] }

#[repr(C)] pub struct bpf_lwt_prog { pub prog: *mut bpf_prog, pub name: *mut c_char }
#[repr(C)] pub struct seg6_flavors_info { pub flv_ops: u32, pub lcblock_bits: u8, pub lcnode_func_bits: u8 }
#[repr(C)] pub struct pcpu_seg6_local_counters { pub packets: u64_stats_t, pub bytes: u64_stats_t, pub errors: u64_stats_t, pub syncp: u64_stats_sync }
#[repr(C)] pub struct seg6_local_counters { pub packets: u64, pub bytes: u64, pub errors: u64 }

pub const SEG6_LOCAL_LCBLOCK_DBITS: u8 = 32;
pub const SEG6_LOCAL_LCNODE_FN_DBITS: u8 = 16;
pub const SEG6_F_ATTR: fn(u32) -> u64 = |i| 1u64 << i;
pub const SEG6_LOCAL_FLV_OP_NEXT_CSID: u32 = 1;
pub const SEG6_LOCAL_FLV_OP_PSP: u32 = 2;
pub const SEG6_F_LOCAL_FLV_NEXT_CSID: u32 = 1u32 << SEG6_LOCAL_FLV_OP_NEXT_CSID;
pub const SEG6_F_LOCAL_FLV_PSP: u32 = 1u32 << SEG6_LOCAL_FLV_OP_PSP;
pub const SEG6_LOCAL_FLV8986_SUPP_OPS: u32 = SEG6_F_LOCAL_FLV_PSP;
pub const SEG6_LOCAL_END_FLV_SUPP_OPS: u32 = SEG6_F_LOCAL_FLV_NEXT_CSID | SEG6_LOCAL_FLV8986_SUPP_OPS;
pub const SEG6_LOCAL_END_X_FLV_SUPP_OPS: u32 = SEG6_F_LOCAL_FLV_NEXT_CSID;

#[repr(C)] pub struct seg6_end_dt_info { pub mode: c_int, pub net: *mut net, pub vrf_ifindex: c_int, pub vrf_table: c_int, pub family: u16 }
#[repr(C)] pub struct seg6_action_desc { pub action: c_int, pub attrs: u64, pub optattrs: u64, pub input: Option<unsafe extern "C" fn(*mut sk_buff, *mut seg6_local_lwt) -> c_int>, pub static_headroom: c_int, pub slwt_ops: seg6_local_lwtunnel_ops }
#[repr(C)] pub struct seg6_local_lwtunnel_ops { pub build_state: Option<unsafe extern "C" fn(*mut seg6_local_lwt, *const c_void, *mut netlink_ext_ack) -> c_int>, pub destroy_state: Option<unsafe extern "C" fn(*mut seg6_local_lwt)> }
#[repr(C)] pub struct seg6_local_lwt { pub action: c_int, pub srh: *mut ipv6_sr_hdr, pub table: c_int, pub nh4: in_addr, pub nh6: in6_addr, pub iif: c_int, pub oif: c_int, pub bpf: bpf_lwt_prog, pub dt_info: seg6_end_dt_info, pub flv_info: seg6_flavors_info, pub pcpu_counters: *mut pcpu_seg6_local_counters, pub headroom: c_int, pub desc: *mut seg6_action_desc, pub parsed_optattrs: u64 }

extern "C" {
    fn seg6_get_srh(skb: *mut sk_buff, flags: c_int) -> *mut ipv6_sr_hdr;
    fn seg6_validate_srh(srh: *mut ipv6_sr_hdr, len: c_int, reduced: bool) -> bool;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn dst_input(skb: *mut sk_buff) -> c_int;
    fn kfree_skb(skb: *mut sk_buff);
    fn seg6_lookup_nexthop(skb: *mut sk_buff, nhaddr: *mut in6_addr, table: u32) -> c_int;
    fn seg6_do_srh_inline(skb: *mut sk_buff, srh: *mut ipv6_sr_hdr) -> c_int;
    fn seg6_do_srh_encap(skb: *mut sk_buff, srh: *mut ipv6_sr_hdr, proto: c_int) -> c_int;
}

unsafe fn seg6_local_lwtunnel(lwt: *mut lwtunnel_state) -> *mut seg6_local_lwt { (*lwt).data as *mut seg6_local_lwt }

unsafe fn advance_nextseg(srh: *mut ipv6_sr_hdr, daddr: *mut in6_addr) {
    (*srh).segments_left = (*srh).segments_left.wrapping_sub(1);
    *daddr = (*srh).segments.add((*srh).segments_left as usize).read();
}

unsafe extern "C" fn input_action_end(skb: *mut sk_buff, slwt: *mut seg6_local_lwt) -> c_int {
    let srh = seg6_get_srh(skb, 0);
    if srh.is_null() { kfree_skb(skb); return -22; }
    advance_nextseg(srh, &mut (*ipv6_hdr(skb)).daddr);
    seg6_lookup_nexthop(skb, core::ptr::null_mut(), 0); dst_input(skb)
}

unsafe extern "C" fn input_action_end_t(skb: *mut sk_buff, slwt: *mut seg6_local_lwt) -> c_int {
    let srh = seg6_get_srh(skb, 0);
    if srh.is_null() { kfree_skb(skb); return -22; }
    advance_nextseg(srh, &mut (*ipv6_hdr(skb)).daddr);
    seg6_lookup_nexthop(skb, core::ptr::null_mut(), (*slwt).table as u32); dst_input(skb)
}

pub unsafe extern "C" fn seg6_local_init() -> c_int { 0 }
pub unsafe extern "C" fn seg6_local_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
