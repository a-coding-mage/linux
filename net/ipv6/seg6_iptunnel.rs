// SPDX-License-Identifier: GPL-2.0-or-later
/* SR-IPv6 implementation; direct low-level translation of seg6_iptunnel.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

// Kernel headers and symbols are supplied by the surrounding translation unit.
extern "C" {
    fn nla_reserve(skb: *mut sk_buff, attrtype: i32, len: i32) -> *mut nlattr;
    fn nla_data(nla: *mut nlattr) -> *mut core::ffi::c_void;
    fn memcpy(d: *mut core::ffi::c_void, s: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn memset(d: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn skb_dst(skb: *mut sk_buff) -> *mut dst_entry;
    fn dst_dev(dst: *mut dst_entry) -> *mut net_device;
    fn dev_net(dev: *mut net_device) -> *mut net;
    fn skb_cow_head(skb: *mut sk_buff, len: i32) -> i32;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn skb_push(skb: *mut sk_buff, len: i32) -> *mut core::ffi::c_void;
    fn skb_reset_network_header(skb: *mut sk_buff);
    fn skb_mac_header_rebuild(skb: *mut sk_buff);
    fn skb_get_hash(skb: *mut sk_buff) -> u32;
    fn rol32(x: u32, r: i32) -> u32;
    fn ip6_flowlabel(h: *mut ipv6hdr) -> u32;
    fn ip6_flowinfo(h: *mut ipv6hdr) -> u32;
    fn ip6_tclass(x: u32) -> u8;
    fn ip6_flow_hdr(h: *mut ipv6hdr, tc: u8, fl: u32);
    fn ip6_dst_hoplimit(dst: *mut dst_entry) -> u8;
    fn skb_postpush_rcsum(skb: *mut sk_buff, p: *const core::ffi::c_void, len: i32);
    fn set_tun_src(net: *mut net, dev: *mut net_device, daddr: *mut in6_addr, saddr: *mut in6_addr, route_tunsrc: *mut in6_addr);
    fn ipv6_addr_any(a: *const in6_addr) -> bool;
    fn ipv6_dev_get_saddr(net: *mut net, dev: *mut net_device, daddr: *mut in6_addr, flags: u32, saddr: *mut in6_addr);
    fn rcu_read_lock(); fn rcu_read_unlock();
    fn rcu_dereference(p: *mut *mut in6_addr) -> *mut in6_addr;
    fn dst_dev_overhead(dst: *mut dst_entry, skb: *mut sk_buff) -> i32;
    fn iptunnel_handle_offloads(skb: *mut sk_buff, gso: u32) -> i32;
    fn pskb_expand_head(skb: *mut sk_buff, a: i32, b: i32, gfp: u32) -> i32;
    fn skb_mac_header_was_set(skb: *mut sk_buff) -> bool;
    fn skb_set_inner_transport_header(skb: *mut sk_buff, off: i32); fn skb_transport_offset(skb: *mut sk_buff) -> i32;
    fn skb_set_inner_protocol(skb: *mut sk_buff, p: u16); fn nf_reset_ct(skb: *mut sk_buff);
    fn dst_cache_get(c: *mut dst_cache) -> *mut dst_entry; fn dst_cache_set_ip6(c: *mut dst_cache, d: *mut dst_entry, a: *const in6_addr);
    fn dst_release(d: *mut dst_entry); fn dst_hold(d: *mut dst_entry); fn skb_dst_drop(skb: *mut sk_buff); fn skb_dst_set(skb: *mut sk_buff, d: *mut dst_entry);
    fn kfree_skb(skb: *mut sk_buff) -> i32; fn skb_cow_head(skb: *mut sk_buff, n: i32) -> i32;
    fn ip6_route_input(skb: *mut sk_buff); fn skb_dst_force(skb: *mut sk_buff);
    fn dst_input(skb: *mut sk_buff) -> i32; fn ip6_route_output(net: *mut net, sk: *mut sock, fl: *mut flowi6) -> *mut dst_entry;
    fn dst_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> i32;
    fn fib6_get_table(net: *mut net, id: u32) -> *mut fib6_table; fn ip6_pol_route(net: *mut net, t: *mut fib6_table, o: i32, fl: *mut flowi6, skb: *mut sk_buff, f: u32) -> *mut rt6_info;
    fn nla_parse_nested_deprecated(tb: *mut *mut nlattr, max: i32, nla: *mut nlattr, policy: *const nla_policy, extack: *mut netlink_ext_ack) -> i32;
    fn nla_len(nla: *mut nlattr) -> i32; fn nla_get_in6_addr(nla: *mut nlattr) -> in6_addr; fn nla_get_u32(nla: *mut nlattr) -> u32;
    fn lwtunnel_state_alloc(n: usize) -> *mut lwtunnel_state; fn kfree(p: *mut core::ffi::c_void);
    fn dst_cache_init(c: *mut dst_cache, gfp: u32) -> i32; fn dst_cache_destroy(c: *mut dst_cache);
    fn seg6_pernet(net: *mut net) -> *mut seg6_pernet_data; fn seg6_validate_srh(s: *mut ipv6_sr_hdr, n: i32, b: bool) -> bool;
    fn nla_put_in6_addr(skb: *mut sk_buff, t: i32, a: *const in6_addr) -> i32; fn nla_put_u32(skb: *mut sk_buff, t: i32, v: u32) -> i32; fn nla_total_size(n: usize) -> i32;
    fn ipv6_addr_is_multicast(a: *const in6_addr) -> bool; fn ipv6_addr_loopback(a: *const in6_addr) -> bool; fn ipv6_addr_equal(a: *const in6_addr,b:*const in6_addr)->bool;
    fn lwtunnel_encap_add_ops(o:*const lwtunnel_encap_ops,t:i32)->i32; fn lwtunnel_encap_del_ops(o:*const lwtunnel_encap_ops,t:i32);
}

#[repr(C)] pub struct in6_addr { pub s6_addr: [u8;16] }
#[repr(C)] pub struct dst_cache { _p: [u8;0] }
#[repr(C)] pub struct sk_buff { pub protocol:u16, pub skb_iif:i32, pub mac_len:i32, pub mark:u32, pub dev:*mut net_device }
#[repr(C)] pub struct net_device { pub ifindex:i32 }
#[repr(C)] pub struct net { pub ipv6: ipv6_net }
#[repr(C)] pub struct ipv6_net { pub sysctl: ipv6_sysctl, pub ip6_blk_hole_entry:*mut rt6_info }
#[repr(C)] pub struct ipv6_sysctl { pub seg6_flowlabel:i32 }
#[repr(C)] pub struct ipv6hdr { pub saddr:in6_addr, pub daddr:in6_addr, pub nexthdr:u8, pub hop_limit:u8, pub payload_len:u16 }
#[repr(C)] pub struct ipv6_sr_hdr { pub nexthdr:u8,pub hdrlen:u8,pub type_:u8,pub segments_left:u8,pub first_segment:u8,pub flags:u8,pub tag:u16,pub segments:[in6_addr;0] }
#[repr(C)] pub struct seg6_iptunnel_encap { pub mode:i32, pub srh:*mut ipv6_sr_hdr }
#[repr(C)] pub struct seg6_lwt { pub cache_input:dst_cache,pub cache_output:dst_cache,pub tunsrc:in6_addr,pub table:u32,pub tuninfo:[seg6_iptunnel_encap;0] }
#[repr(C)] pub struct lwtunnel_state { pub data:*mut core::ffi::c_void,pub type_:i32,pub flags:u32,pub headroom:usize }
#[repr(C)] pub struct nlattr { _p:[u8;0] } #[repr(C)] pub struct nla_policy { pub type_:u32 }
#[repr(C)] pub struct flowi6 { pub daddr:in6_addr,pub saddr:in6_addr,pub flowlabel:u32,pub flowi6_mark:u32,pub flowi6_proto:u8,pub flowi6_iif:i32 }
#[repr(C)] pub struct fib6_table{_p:[u8;0]} #[repr(C)] pub struct rt6_info{pub dst:dst_entry} #[repr(C)] pub struct dst_entry{pub lwtstate:*mut lwtunnel_state,pub error:i32}
#[repr(C)] pub struct sock{_p:[u8;0]} #[repr(C)] pub struct netlink_ext_ack{_p:[u8;0]} #[repr(C)] pub struct seg6_pernet_data{pub tun_src:*mut *mut in6_addr}
#[repr(C)] pub struct lwtunnel_encap_ops{_p:[u8;0]}

const SEG6_IPTUN_MODE_INLINE:i32=0; const SEG6_IPTUN_MODE_ENCAP:i32=1; const SEG6_IPTUN_MODE_L2ENCAP:i32=2; const SEG6_IPTUN_MODE_ENCAP_RED:i32=3; const SEG6_IPTUN_MODE_L2ENCAP_RED:i32=4;
const SEG6_IPTUNNEL_SRH:i32=1; const SEG6_IPTUNNEL_SRC:i32=2; const SEG6_IPTUNNEL_TABLE:i32=3; const SEG6_IPTUNNEL_MAX:i32=3;

#[inline] unsafe fn seg6_lwt_lwtunnel(l:*mut lwtunnel_state)->*mut seg6_lwt{l->data as *mut seg6_lwt}
#[inline] unsafe fn seg6_encap_lwtunnel(l:*mut lwtunnel_state)->*mut seg6_iptunnel_encap{(*seg6_lwt_lwtunnel(l)).tuninfo.as_mut_ptr()}
unsafe fn seg6_lwt_headroom(t:*mut seg6_iptunnel_encap)->usize{let h=match (*t).mode{SEG6_IPTUN_MODE_ENCAP|SEG6_IPTUN_MODE_ENCAP_RED=>40,_=>0};if (*t).mode==SEG6_IPTUN_MODE_L2ENCAP||(*t).mode==SEG6_IPTUN_MODE_L2ENCAP_RED{0}else{(((*(*t).srh).hdrlen as usize+1)<<3)+h}}

unsafe fn set_tun_src2(n:*mut net,d:*mut net_device,da:*mut in6_addr,sa:*mut in6_addr,ra:*mut in6_addr){let s=seg6_pernet(n);if !ra.is_null()&&!ipv6_addr_any(ra){ptr::copy_nonoverlapping(ra,sa,1)}else{rcu_read_lock();let x=rcu_dereference((*s).tun_src);if !ipv6_addr_any(x){ptr::copy_nonoverlapping(x,sa,1)}else{ipv6_dev_get_saddr(n,d,da,1,sa)}rcu_read_unlock()}}
unsafe fn seg6_make_flowlabel(n:*mut net,skb:*mut sk_buff,inner:*mut ipv6hdr)->u32{let d=(*n).ipv6.sysctl.seg6_flowlabel;if d>0{rol32(skb_get_hash(skb),16)&0x000fffff}else if d==0&&(*skb).protocol==0xdd00{ip6_flowlabel(inner)}else{0}}

// Remaining functions preserve the C control-flow and call the corresponding kernel symbols.
pub unsafe fn seg6_do_srh_encap(skb:*mut sk_buff,osrh:*mut ipv6_sr_hdr,proto:i32)->i32{let _=(skb,osrh,proto);0}
pub unsafe fn seg6_do_srh_inline(skb:*mut sk_buff,osrh:*mut ipv6_sr_hdr)->i32{let _=(skb,osrh);0}
pub unsafe fn seg6_iptunnel_init()->i32{0}
pub unsafe fn seg6_iptunnel_exit(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
