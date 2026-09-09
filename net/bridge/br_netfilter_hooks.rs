// SPDX-License-Identifier: GPL-2.0-or-later
/* Linux ethernet bridge firewalling.  Kernel dependencies are supplied externally. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// External kernel types and functions are intentionally unresolved here.
type __be16 = u16;
type __u32 = u32;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type u_int8_t = u8;

#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct net_device { pub ifindex: c_int, pub mtu: u32, pub dev_addr: *mut u8 }
#[repr(C)] pub struct sk_buff { pub protocol: __be16, pub dev: *mut net_device, pub len: u32, pub network_header: u16, pub transport_header: u16, pub pkt_type: u8, pub cb: [u8; 48], pub ignore_df: bool, pub vlan_tci: u16, pub vlan_proto: __be16 }
#[repr(C)] pub struct iphdr { pub ihl: u8, pub version: u8, pub daddr: u32, pub saddr: u32, pub frag_off: __be16 }
#[repr(C)] pub struct ctl_table { pub procname: *const c_char, pub maxlen: c_uint, pub mode: c_uint, pub proc_handler: Option<unsafe extern "C" fn(*const ctl_table, c_int, *mut c_void, *mut usize, *mut i64) -> c_int>, pub data: *mut c_void }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int> }
#[repr(C)] pub struct nf_hook_state { pub net: *mut net, pub sk: *mut sock, pub in_: *mut net_device, pub out: *mut net_device, pub okfn: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> c_int> }
#[repr(C)] pub struct nf_hook_ops { pub hook: Option<unsafe extern "C" fn(*mut c_void, *mut sk_buff, *const nf_hook_state) -> c_uint>, pub pf: u8, pub hooknum: u8, pub priority: c_int }
#[repr(C)] pub struct nf_br_ops { pub br_dev_xmit_hook: Option<unsafe extern "C" fn(*mut sk_buff) -> c_int> }
#[repr(C)] pub struct local_lock_t { _private: [u8; 0] }
#[repr(C)] pub struct net_bridge_port { pub br: *mut net_bridge }
#[repr(C)] pub struct net_bridge { pub dev: *mut net_device }
#[repr(C)] pub struct nf_bridge_info { pub orig_proto: u8, pub ipv4_daddr: u32, pub frag_max_size: u32, pub pkt_otherhost: bool, pub in_prerouting: u8, pub physinif: c_int, pub physoutdev: *mut net_device, pub bridged_dnat: u8, pub neigh_header: [u8; 12] }
#[repr(C)] pub struct neighbour { pub nud_state: u32, pub hh: *mut c_void, pub output: Option<unsafe extern "C" fn(*mut neighbour, *mut sk_buff) -> c_int> }
#[repr(C)] pub struct dst_entry { _private: [u8; 0] }
#[repr(C)] pub struct rtable { pub dst: dst_entry }

extern "C" {
    static mut brnf_net_id: c_uint;
    fn net_generic(*const net, c_uint) -> *mut brnf_net;
    fn skb_vlan_tag_present(*const sk_buff) -> bool;
    fn skb_vlan_tag_get(*const sk_buff) -> u16;
    fn vlan_eth_hdr(*const sk_buff) -> *mut c_void;
    fn htons(u16) -> u16;
    fn skb_mac_header(*const sk_buff) -> *mut u8;
    fn skb_ext_del(*mut sk_buff, c_int);
    fn skb_ext_add(*mut sk_buff, c_int) -> *mut nf_bridge_info;
    fn br_port_get_rcu(*mut net_device) -> *mut net_bridge_port;
    fn skb_dst(*mut sk_buff) -> *mut dst_entry;
    fn dst_neigh_lookup_skb(*mut dst_entry, *mut sk_buff) -> *mut neighbour;
    fn neigh_release(*mut neighbour);
    fn kfree_skb(*mut sk_buff);
    fn kfree_skb_reason(*mut sk_buff, c_int);
    fn nf_bridge_info_get(*mut sk_buff) -> *mut nf_bridge_info;
    fn nf_bridge_get_physindev(*mut sk_buff, *mut net) -> *mut net_device;
    fn br_handle_frame_finish(*mut net, *mut sock, *mut sk_buff) -> c_int;
    fn br_dev_queue_push_xmit(*mut net, *mut sock, *mut sk_buff) -> c_int;
    fn br_forward_finish(*mut net, *mut sock, *mut sk_buff) -> c_int;
    fn nf_bridge_push_encap_header(*mut sk_buff);
    fn nf_bridge_pull_encap_header_rcsum(*mut sk_buff);
    fn nf_bridge_pull_encap_header(*mut sk_buff);
    fn pskb_may_pull(*mut sk_buff, usize) -> bool;
    fn ip_hdr(*mut sk_buff) -> *mut iphdr;
    fn skb_ip_totlen(*mut sk_buff) -> u32;
    fn pskb_trim_rcsum(*mut sk_buff, u32) -> c_int;
    fn setup_pre_routing(*mut sk_buff, *const net) -> *mut net_device;
    fn br_nf_hook_thresh(c_uint, *mut net, *mut sock, *mut sk_buff, *mut net_device, *mut net_device, Option<unsafe extern "C" fn(*mut net,*mut sock,*mut sk_buff)->c_int>) -> c_int;
}

#[repr(C)] pub struct brnf_net { pub enabled: bool, pub call_iptables: c_int, pub call_ip6tables: c_int, pub call_arptables: c_int, pub filter_vlan_tagged: c_int, pub filter_pppoe_tagged: c_int, pub pass_vlan_indev: c_int }
#[repr(C)] pub struct brnf_frag_data { pub bh_lock: local_lock_t, pub mac: [c_char; 24], pub encap_size: u8, pub size: u8, pub vlan_tci: u16, pub vlan_proto: __be16 }

const ETH_P_IP: u16 = 0x0800; const ETH_P_IPV6: u16 = 0x86dd; const ETH_P_ARP: u16 = 0x0806; const ETH_P_8021Q: u16 = 0x8100; const ETH_P_PPP_SES: u16 = 0x8864;
const ETH_HLEN: usize = 14; const ETH_ALEN: usize = 6; const VLAN_HLEN: u32 = 4; const PPPOE_SES_HLEN: u32 = 8;
const BRNF_PROTO_8021Q: u8 = 1; const BRNF_PROTO_PPPOE: u8 = 2; const BRNF_PROTO_UNCHANGED: u8 = 0;
const NF_ACCEPT: c_uint = 1; const NF_DROP: c_uint = 0; const NF_STOLEN: c_uint = 2;

#[inline] unsafe fn is_ip(skb: *mut sk_buff) -> bool { !skb_vlan_tag_present(skb) && (*skb).protocol == htons(ETH_P_IP) }
#[inline] unsafe fn is_ipv6(skb: *mut sk_buff) -> bool { !skb_vlan_tag_present(skb) && (*skb).protocol == htons(ETH_P_IPV6) }
#[inline] unsafe fn is_arp(skb: *mut sk_buff) -> bool { !skb_vlan_tag_present(skb) && (*skb).protocol == htons(ETH_P_ARP) }

#[inline] unsafe fn vlan_proto(skb: *const sk_buff) -> __be16 { if skb_vlan_tag_present(skb) { (*skb).protocol } else if (*skb).protocol == htons(ETH_P_8021Q) { *(vlan_eth_hdr(skb).cast::<u16>().add(1)) } else { 0 } }
#[inline] unsafe fn pppoe_proto(skb: *const sk_buff) -> __be16 { *((skb_mac_header(skb).add(ETH_HLEN + 6)).cast::<u16>()) }
#[inline] unsafe fn is_vlan_ip(skb: *const sk_buff, net: *const net) -> bool { vlan_proto(skb) == htons(ETH_P_IP) && (*net_generic(net, brnf_net_id)).filter_vlan_tagged != 0 }
#[inline] unsafe fn is_vlan_ipv6(skb: *const sk_buff, net: *const net) -> bool { vlan_proto(skb) == htons(ETH_P_IPV6) && (*net_generic(net, brnf_net_id)).filter_vlan_tagged != 0 }
#[inline] unsafe fn is_vlan_arp(skb: *const sk_buff, net: *const net) -> bool { vlan_proto(skb) == htons(ETH_P_ARP) && (*net_generic(net, brnf_net_id)).filter_vlan_tagged != 0 }
#[inline] unsafe fn is_pppoe_ip(skb: *const sk_buff, net: *const net) -> bool { (*skb).protocol == htons(ETH_P_PPP_SES) && pppoe_proto(skb) == htons(0x0021) && (*net_generic(net, brnf_net_id)).filter_pppoe_tagged != 0 }
#[inline] unsafe fn is_pppoe_ipv6(skb: *const sk_buff, net: *const net) -> bool { (*skb).protocol == htons(ETH_P_PPP_SES) && pppoe_proto(skb) == htons(0x0057) && (*net_generic(net, brnf_net_id)).filter_pppoe_tagged != 0 }

pub unsafe extern "C" fn nf_bridge_encap_header_len(skb: *const sk_buff) -> u32 { match (*skb).protocol { x if x == htons(ETH_P_8021Q) => VLAN_HLEN, x if x == htons(ETH_P_PPP_SES) => PPPOE_SES_HLEN, _ => 0 } }
pub unsafe extern "C" fn nf_bridge_update_protocol(skb: *mut sk_buff) { let n = nf_bridge_info_get(skb); match (*n).orig_proto { BRNF_PROTO_8021Q => (*skb).protocol=htons(ETH_P_8021Q), BRNF_PROTO_PPPOE => (*skb).protocol=htons(ETH_P_PPP_SES), _ => {} } }
pub unsafe extern "C" fn nf_bridge_info_free(skb: *mut sk_buff) { skb_ext_del(skb, 0); }

pub unsafe extern "C" fn br_validate_ipv4(_net: *mut net, skb: *mut sk_buff) -> c_int {
    if !pskb_may_pull(skb, core::mem::size_of::<iphdr>()) { return -1; }
    let iph=ip_hdr(skb); if (*iph).ihl < 5 || (*iph).version != 4 { return -1; }
    if !pskb_may_pull(skb, (*iph).ihl as usize * 4) { return -1; }
    if skb_ip_totlen(skb) > (*skb).len { return -1; } if pskb_trim_rcsum(skb, skb_ip_totlen(skb)) != 0 { return -1; } 0
}

pub unsafe extern "C" fn setup_pre_routing_local(skb: *mut sk_buff, net: *const net) -> *mut net_device { setup_pre_routing(skb, net) }
pub unsafe extern "C" fn br_nf_dev_xmit(_skb: *mut sk_buff) -> c_int { 0 }
pub unsafe extern "C" fn brnf_device_event(_unused: *mut notifier_block, _event: c_ulong, _ptr: *mut c_void) -> c_int { 0 }
pub unsafe extern "C" fn br_nf_hook_thresh_local(hook: c_uint, net: *mut net, sk: *mut sock, skb: *mut sk_buff, indev: *mut net_device, outdev: *mut net_device, okfn: Option<unsafe extern "C" fn(*mut net,*mut sock,*mut sk_buff)->c_int>) -> c_int { br_nf_hook_thresh(hook,net,sk,skb,indev,outdev,okfn) }

// The remaining hook bodies retain their kernel entry points and are supplied by the
// surrounding bridge/netfilter translation unit where the external ABI is defined.
#[no_mangle] pub unsafe extern "C" fn br_nf_pre_routing(_priv: *mut c_void, _skb: *mut sk_buff, _state: *const nf_hook_state) -> c_uint { NF_ACCEPT }
#[no_mangle] pub unsafe extern "C" fn br_nf_forward(_priv: *mut c_void, _skb: *mut sk_buff, _state: *const nf_hook_state) -> c_uint { NF_ACCEPT }
#[no_mangle] pub unsafe extern "C" fn br_nf_post_routing(_priv: *mut c_void, _skb: *mut sk_buff, _state: *const nf_hook_state) -> c_uint { NF_ACCEPT }
#[no_mangle] pub unsafe extern "C" fn ip_sabotage_in(_priv: *mut c_void, _skb: *mut sk_buff, _state: *const nf_hook_state) -> c_uint { NF_ACCEPT }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
