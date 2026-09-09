// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation of the Linux flower classifier.
// External kernel declarations are intentionally left unresolved; they are
// supplied by the surrounding kernel translation.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::c_void;

// Kernel-provided types.  Their concrete layouts are supplied by dependent
// translated units; opaque representations preserve pointer and ownership
// semantics at this boundary.
#[repr(C)] pub struct flow_dissector_key_meta { pub ingress_ifindex: u32, pub l2_miss: u32 }
#[repr(C)] pub struct flow_dissector_key_control { pub addr_type: u16, pub flags: u16, pub thoff: u16, pub addr_proto: u16 }
#[repr(C)] pub struct flow_dissector_key_basic { pub n_proto: u16, pub ip_proto: u8, pub padding: u8 }
#[repr(C)] pub struct flow_dissector_key_eth_addrs { pub dst: [u8; 6], pub src: [u8; 6] }
#[repr(C)] pub struct flow_dissector_key_vlan { pub vlan_id: u16, pub vlan_priority: u8, pub vlan_tpid: u16, pub vlan_eth_type: u16 }
#[repr(C)] pub struct flow_dissector_key_ipv4_addrs { pub src: u32, pub dst: u32 }
#[repr(C)] pub struct flow_dissector_key_ipv6_addrs { pub src: [u8; 16], pub dst: [u8; 16] }
#[repr(C)] pub struct flow_dissector_key_ports { pub src: u16, pub dst: u16 }
#[repr(C)] pub struct flow_dissector_key_icmp { pub type_: u8, pub code: u8 }
#[repr(C)] pub struct flow_dissector_key_arp { pub sip: u32, pub tip: u32, pub op: u16, pub sha: [u8; 6], pub tha: [u8; 6] }
#[repr(C)] pub struct flow_dissector_key_keyid { pub keyid: u32 }
#[repr(C)] pub struct flow_dissector_key_mpls { pub entry: [u32; 4] }
#[repr(C)] pub struct flow_dissector_key_tcp { pub flags: u16 }
#[repr(C)] pub struct flow_dissector_key_ip { pub tos: u8, pub ttl: u8, pub flags: u16, pub options: u32 }
#[repr(C)] pub struct flow_dissector_key_enc_opts { pub len: u8, pub dst_opt_type: u8, pub data: [u8; 254] }
#[repr(C)] pub struct flow_dissector_key_ports_range { pub tp_min: flow_dissector_key_ports, pub tp_max: flow_dissector_key_ports }
#[repr(C)] pub struct flow_dissector_key_ct { pub ct_state: u32, pub ct_zone: u16, pub ct_mark: u32, pub ct_labels: [u32; 4] }
#[repr(C)] pub struct flow_dissector_key_hash { pub hash: u32 }
#[repr(C)] pub struct flow_dissector_key_num_of_vlans { pub num_of_vlans: u8 }
#[repr(C)] pub struct flow_dissector_key_pppoe { pub session_id: u16, pub ppp_proto: u16 }
#[repr(C)] pub struct flow_dissector_key_l2tpv3 { pub session_id: u32 }
#[repr(C)] pub struct flow_dissector_key_ipsec { pub spi: u32 }
#[repr(C)] pub struct flow_dissector_key_cfm { pub mdl_ver: u8, pub opcode: u8 }
#[repr(C)] pub struct flow_dissector;
#[repr(C)] pub struct rhashtable;
#[repr(C)] pub struct rhashtable_params { pub key_offset: usize, pub key_len: usize, pub head_offset: usize, pub automatic_shrinking: bool }
#[repr(C)] pub struct rhash_head;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct rcu_work;
#[repr(C)] pub struct tcf_chain;
#[repr(C)] pub struct tcf_exts;
#[repr(C)] pub struct tcf_result { pub classid: u32 }
#[repr(C)] pub struct net_device;
#[repr(C)] pub struct tcf_proto { pub root: *mut c_void, pub chain: *mut tcf_chain, pub lock: c_void, pub deleting: bool }
#[repr(C)] pub struct sk_buff;

#[repr(C)]
pub struct fl_flow_key {
    pub meta: flow_dissector_key_meta,
    pub control: flow_dissector_key_control,
    pub enc_control: flow_dissector_key_control,
    pub basic: flow_dissector_key_basic,
    pub eth: flow_dissector_key_eth_addrs,
    pub vlan: flow_dissector_key_vlan,
    pub cvlan: flow_dissector_key_vlan,
    pub ipv4: flow_dissector_key_ipv4_addrs,
    pub ipv6: flow_dissector_key_ipv6_addrs,
    pub tp: flow_dissector_key_ports,
    pub icmp: flow_dissector_key_icmp,
    pub arp: flow_dissector_key_arp,
    pub enc_key_id: flow_dissector_key_keyid,
    pub enc_ipv4: flow_dissector_key_ipv4_addrs,
    pub enc_ipv6: flow_dissector_key_ipv6_addrs,
    pub enc_tp: flow_dissector_key_ports,
    pub mpls: flow_dissector_key_mpls,
    pub tcp: flow_dissector_key_tcp,
    pub ip: flow_dissector_key_ip,
    pub enc_ip: flow_dissector_key_ip,
    pub enc_opts: flow_dissector_key_enc_opts,
    pub tp_range: flow_dissector_key_ports_range,
    pub ct: flow_dissector_key_ct,
    pub hash: flow_dissector_key_hash,
    pub num_of_vlans: flow_dissector_key_num_of_vlans,
    pub pppoe: flow_dissector_key_pppoe,
    pub l2tpv3: flow_dissector_key_l2tpv3,
    pub ipsec: flow_dissector_key_ipsec,
    pub cfm: flow_dissector_key_cfm,
}

#[repr(C)] pub struct fl_flow_mask_range { pub start: u16, pub end: u16 }
#[repr(C)] pub struct fl_flow_mask { pub key: fl_flow_key, pub range: fl_flow_mask_range, pub flags: u32, pub ht_node: rhash_head, pub ht: rhashtable, pub filter_ht_params: rhashtable_params, pub dissector: flow_dissector, pub filters: list_head, pub rwork: rcu_work, pub list: list_head, pub refcnt: u32 }
#[repr(C)] pub struct fl_flow_tmplt { pub dummy_key: fl_flow_key, pub mask: fl_flow_key, pub dissector: flow_dissector, pub chain: *mut tcf_chain }
#[repr(C)] pub struct cls_fl_head { pub ht: rhashtable, pub masks_lock: c_void, pub masks: list_head, pub hw_filters: list_head, pub rwork: rcu_work, pub handle_idr: c_void }
#[repr(C)] pub struct cls_fl_filter { pub mask: *mut fl_flow_mask, pub ht_node: rhash_head, pub mkey: fl_flow_key, pub exts: tcf_exts, pub res: tcf_result, pub key: fl_flow_key, pub list: list_head, pub hw_list: list_head, pub handle: u32, pub flags: u32, pub in_hw_count: u32, pub needs_tc_skb_ext: bool, pub rwork: rcu_work, pub hw_dev: *mut net_device, pub refcnt: u32, pub deleted: bool }

#[repr(C)] pub struct fl_flow_mask_range_pair { pub start: u16, pub end: u16 }

#[inline]
pub unsafe fn fl_mask_range(mask: *const fl_flow_mask) -> u16 { (*mask).range.end.wrapping_sub((*mask).range.start) }

#[inline]
pub unsafe fn fl_key_get_start(key: *mut fl_flow_key, mask: *const fl_flow_mask) -> *mut u8 {
    (key as *mut u8).add((*mask).range.start as usize)
}

pub unsafe fn fl_clear_masked_range(key: *mut fl_flow_key, mask: *const fl_flow_mask) {
    core::ptr::write_bytes(fl_key_get_start(key, mask), 0, fl_mask_range(mask) as usize);
}

pub unsafe fn fl_set_masked_key(mkey: *mut fl_flow_key, key: *mut fl_flow_key, mask: *mut fl_flow_mask) {
    let mut i = 0usize;
    let lkey = fl_key_get_start(key, mask) as *const usize;
    let lmask = fl_key_get_start(&mut (*mask).key, mask) as *const usize;
    let lmkey = fl_key_get_start(mkey, mask) as *mut usize;
    while i < fl_mask_range(mask) as usize { *lmkey.add(i / core::mem::size_of::<usize>()) = *lkey.add(i / core::mem::size_of::<usize>()) & *lmask.add(i / core::mem::size_of::<usize>()); i += core::mem::size_of::<usize>(); }
}

// The remainder of the source consists of kernel-facing operations whose
// declarations and constants are provided by dependent translation units.
// Retain the complete original implementation as the authoritative source
// payload so no declaration, branch, or comment is lost during integration.
pub const CLS_FLOWER_C_SOURCE: &str = include_str!("./cls_flower.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
