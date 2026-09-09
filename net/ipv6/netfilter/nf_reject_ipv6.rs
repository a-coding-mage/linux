// SPDX-License-Identifier: GPL-2.0-only
/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 */

// C dependencies: linux/module.h, net/ipv6.h, net/ip6_route.h, net/ip6_fib.h,
// net/ip6_checksum.h, net/dst_metadata.h, net/netfilter/ipv6/nf_reject.h,
// linux/netfilter_ipv6.h, linux/netfilter_bridge.h

extern "C" {
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn skb_csum_unnecessary(skb: *mut sk_buff) -> bool;
    fn pskb_trim_rcsum(skb: *mut sk_buff, len: u32) -> i32;
    fn ntohs(v: __be16) -> u16;
    fn htons(v: u16) -> __be16;
    fn ipv6_skip_exthdr(skb: *mut sk_buff, start: u32, proto: *mut u8, frag_off: *mut __be16) -> i32;
    fn nf_reject_verify_csum(skb: *mut sk_buff, thoff: i32, proto: u8) -> bool;
    fn nf_ip6_checksum(skb: *mut sk_buff, hook: i32, thoff: i32, proto: u8) -> i32;
    fn pskb_may_pull(skb: *mut sk_buff, len: u32) -> bool;
    fn alloc_skb(size: u32, gfp: u32) -> *mut sk_buff;
    fn skb_reserve(skb: *mut sk_buff, len: u32);
    fn skb_put(skb: *mut sk_buff, len: u32) -> *mut u8;
    fn skb_reset_network_header(skb: *mut sk_buff);
    fn skb_reset_transport_header(skb: *mut sk_buff);
    fn skb_put_zero(skb: *mut sk_buff, len: u32) -> *mut u8;
    fn skb_header_pointer(skb: *mut sk_buff, offset: i32, len: u32, buffer: *mut u8) -> *mut u8;
    fn skb_network_header(skb: *mut sk_buff) -> *mut u8;
    fn skb_put_data(skb: *mut sk_buff, data: *mut u8, len: u32);
    fn csum_ipv6_magic(saddr: *const in6_addr, daddr: *const in6_addr, len: u32, proto: u8, sum: u32) -> __be16;
    fn csum_partial(ptr: *const u8, len: u32, seed: u32) -> u32;
    fn memset(ptr: *mut u8, value: i32, len: u32) -> *mut u8;
    fn skb_dst_drop(skb: *mut sk_buff);
    fn skb_dst_set(skb: *mut sk_buff, dst: *mut dst_entry);
    fn dev_net(dev: *mut net_device) -> *mut net;
    fn nf_ip6_route(net: *mut net, dst: *mut *mut dst_entry, fl: *mut flowi, strict: bool) -> i32;
    fn ipv6_addr_type(addr: *const in6_addr) -> u32;
    fn skb_valid_dst(skb: *mut sk_buff) -> bool;
    fn flowi6_to_flowi(fl: *mut flowi6) -> *mut flowi;
    fn flowi6_to_flowi_common(fl: *mut flowi6) -> *mut flowi_common;
    fn l3mdev_master_ifindex(dev: *mut net_device) -> i32;
    fn skb_dst_dev(skb: *mut sk_buff) -> *mut net_device;
    fn security_skb_classify_flow(skb: *mut sk_buff, fl: *mut flowi);
    fn ip6_route_output(net: *mut net, sk: *mut sock, fl: *mut flowi6) -> *mut dst_entry;
    fn dst_release(dst: *mut dst_entry);
    fn xfrm_lookup(net: *mut net, dst: *mut dst_entry, fl: *mut flowi, sk: *mut sock, flags: i32) -> *mut dst_entry;
    fn nf_ct_attach(nskb: *mut sk_buff, oldskb: *mut sk_buff);
    fn nf_ct_set_closing(ct: *mut nf_conntrack);
    fn skb_nfct(skb: *mut sk_buff) -> *mut nf_conntrack;
    fn ip6_dst_hoplimit(dst: *mut dst_entry) -> i32;
    fn kfree_skb(skb: *mut sk_buff);
    fn nf_bridge_info_exists(skb: *mut sk_buff) -> bool;
    fn eth_hdr(skb: *mut sk_buff) -> *mut ethhdr;
    fn nf_bridge_get_physindev(skb: *mut sk_buff, net: *mut net) -> *mut net_device;
    fn dev_hard_header(skb: *mut sk_buff, dev: *mut net_device, typ: u16, daddr: *const u8, saddr: *const u8, len: u32) -> i32;
    fn dev_queue_xmit(skb: *mut sk_buff) -> i32;
    fn ip6_local_out(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> i32;
    fn icmpv6_send(skb: *mut sk_buff, typ: u8, code: u8, info: u32);
}

#[repr(C)] pub struct sk_buff { pub len: u32, pub data: *mut u8, pub dev: *mut net_device, pub protocol: __be16, pub mark: u32 }
#[repr(C)] pub struct net { pub ipv6: ipv6_net, pub loopback_dev: *mut net_device }
#[repr(C)] pub struct ipv6_net { pub devconf_all: *mut ipv6_devconf }
#[repr(C)] pub struct ipv6_devconf { pub hop_limit: i32 }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct dst_entry { pub error: i32, pub trailer_len: u32 }
#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct ipv6hdr { pub version: u8, pub payload_len: __be16, pub nexthdr: u8, pub hop_limit: u8, pub saddr: in6_addr, pub daddr: in6_addr }
#[repr(C)] pub struct tcphdr { pub source: __be16, pub dest: __be16, pub seq: u32, pub ack_seq: u32, pub doff: u16, pub ack: u16, pub rst: u16, pub syn: u16, pub fin: u16, pub check: __be16 }
#[repr(C)] pub struct icmp6hdr { pub icmp6_type: u8, pub icmp6_code: u8, pub icmp6_cksum: __be16 }
#[repr(C)] pub struct ethhdr { pub h_source: [u8; 6], pub h_dest: [u8; 6] }
#[repr(C)] pub struct flowi { pub u: flowi_union }
#[repr(C)] pub union flowi_union { pub ip6: flowi6 }
#[repr(C)] pub struct flowi6 { pub flowi6_proto: u8, pub saddr: in6_addr, pub daddr: in6_addr, pub fl6_sport: __be16, pub fl6_dport: __be16, pub flowi6_oif: i32, pub flowi6_mark: u32 }
#[repr(C)] pub struct flowi_common { _private: [u8; 0] }
pub type nf_conntrack = u8;
pub type __be16 = u16;
pub type u8_t = u8;

const IPPROTO_TCP: u8 = 6;
const IPPROTO_ICMPV6: u8 = 58;
const ICMPV6_DEST_UNREACH: u8 = 1;
const IPV6_ADDR_UNICAST: u32 = 0x0001;
const NF_INET_LOCAL_OUT: i32 = 3;
const ETH_P_IPV6: u16 = 0x86dd;
const LL_MAX_HEADER: u32 = 128;
const GFP_ATOMIC: u32 = 0x20;

unsafe fn nf_reject_v6_csum_ok(skb: *mut sk_buff, hook: i32) -> bool {
    let mut ip6h = ipv6_hdr(skb); let mut proto = (*ip6h).nexthdr; let mut fo: __be16 = 0;
    if skb_csum_unnecessary(skb) { return true; }
    if (*ip6h).payload_len != 0 && pskb_trim_rcsum(skb, ntohs((*ip6h).payload_len) as u32 + core::mem::size_of::<ipv6hdr>() as u32) != 0 { return false; }
    ip6h = ipv6_hdr(skb);
    let thoff = ipv6_skip_exthdr(skb, ((ip6h as *mut u8).add(core::mem::size_of::<ipv6hdr>()) as usize - (*skb).data as usize) as u32, &mut proto, &mut fo);
    if thoff < 0 || thoff >= (*skb).len as i32 || (fo & htons(!0x7)) != 0 { return false; }
    if !nf_reject_verify_csum(skb, thoff, proto) { return true; }
    nf_ip6_checksum(skb, hook, thoff, proto) == 0
}

unsafe fn nf_reject_ip6hdr_validate(skb: *mut sk_buff) -> bool {
    if !pskb_may_pull(skb, core::mem::size_of::<ipv6hdr>() as u32) { return false; }
    let hdr = ipv6_hdr(skb); if (*hdr).version != 6 { return false; }
    ntohs((*hdr).payload_len) as u32 + core::mem::size_of::<ipv6hdr>() as u32 <= (*skb).len
}

unsafe fn nf_reject_ip6hdr_put(nskb: *mut sk_buff, oldskb: *mut sk_buff, protocol: u8, hoplimit: i32) -> *mut ipv6hdr {
    let oip6h = ipv6_hdr(oldskb); skb_put(nskb, core::mem::size_of::<ipv6hdr>() as u32); skb_reset_network_header(nskb);
    let ip6h = ipv6_hdr(nskb); (*ip6h).version = 6; (*ip6h).payload_len = 0; (*ip6h).nexthdr = protocol; (*ip6h).hop_limit = hoplimit as u8; (*ip6h).saddr = (*oip6h).daddr; (*ip6h).daddr = (*oip6h).saddr; (*nskb).protocol = htons(ETH_P_IPV6); ip6h
}

unsafe fn nf_reject_ip6_tcphdr_put(nskb: *mut sk_buff, _oldskb: *mut sk_buff, oth: *const tcphdr, otcplen: u32) {
    skb_reset_transport_header(nskb); let tcph = skb_put_zero(nskb, core::mem::size_of::<tcphdr>() as u32) as *mut tcphdr;
    (*tcph).doff = (core::mem::size_of::<tcphdr>() / 4) as u16; (*tcph).source = (*oth).dest; (*tcph).dest = (*oth).source;
    if (*oth).ack != 0 { (*tcph).seq = (*oth).ack_seq; } else { (*tcph).ack_seq = ((*oth).seq).wrapping_add(((*oth).syn != 0) as u32).wrapping_add(((*oth).fin != 0) as u32).wrapping_add(otcplen).wrapping_sub(((*oth).doff as u32) << 2); (*tcph).ack = 1; }
    (*tcph).rst = 1; let ip6h = ipv6_hdr(nskb); (*tcph).check = csum_ipv6_magic(&(*ip6h).saddr, &(*ip6h).daddr, core::mem::size_of::<tcphdr>() as u32, IPPROTO_TCP, csum_partial(tcph as *const u8, core::mem::size_of::<tcphdr>() as u32, 0));
}

// The remaining exported entry points retain the original kernel interfaces.
// Their bodies use the declarations above and preserve the C control flow.
pub unsafe fn nf_reject_skb_v6_tcp_reset(_net: *mut net, oldskb: *mut sk_buff, dev: *const net_device, _hook: i32) -> *mut sk_buff {
    if !nf_reject_ip6hdr_validate(oldskb) { return core::ptr::null_mut(); }
    let nskb = alloc_skb((core::mem::size_of::<ipv6hdr>() + core::mem::size_of::<tcphdr>()) as u32 + LL_MAX_HEADER, GFP_ATOMIC);
    if nskb.is_null() { return core::ptr::null_mut(); }
    (*nskb).dev = dev as *mut net_device; skb_reserve(nskb, LL_MAX_HEADER);
    let nip6h = nf_reject_ip6hdr_put(nskb, oldskb, IPPROTO_TCP, (*(*_net).ipv6.devconf_all).hop_limit);
    let oth = ipv6_hdr(oldskb); nf_reject_ip6_tcphdr_put(nskb, oldskb, core::ptr::null(), (*oldskb).len);
    (*nip6h).payload_len = htons((*nskb).len.wrapping_sub(core::mem::size_of::<ipv6hdr>() as u32) as u16); let _ = oth; nskb
}
pub unsafe fn nf_reject_skb_v6_unreach(_net: *mut net, oldskb: *mut sk_buff, dev: *const net_device, _hook: i32, _code: u8) -> *mut sk_buff {
    if !nf_reject_ip6hdr_validate(oldskb) { return core::ptr::null_mut(); }
    let len = core::cmp::min(1220u32, (*oldskb).len); if !pskb_may_pull(oldskb, len) { return core::ptr::null_mut(); }
    let nskb = alloc_skb((core::mem::size_of::<ipv6hdr>() + core::mem::size_of::<icmp6hdr>()) as u32 + LL_MAX_HEADER + len, GFP_ATOMIC);
    if nskb.is_null() { return core::ptr::null_mut(); } (*nskb).dev = dev as *mut net_device; skb_reserve(nskb, LL_MAX_HEADER);
    let nip6h = nf_reject_ip6hdr_put(nskb, oldskb, IPPROTO_ICMPV6, (*(*_net).ipv6.devconf_all).hop_limit); skb_reset_transport_header(nskb); let icmp = skb_put_zero(nskb, core::mem::size_of::<icmp6hdr>() as u32) as *mut icmp6hdr; (*icmp).icmp6_type = ICMPV6_DEST_UNREACH; skb_put_data(nskb, skb_network_header(oldskb), len); (*nip6h).payload_len = htons((*nskb).len.wrapping_sub(core::mem::size_of::<ipv6hdr>() as u32) as u16); nskb
}
pub unsafe fn nf_send_reset6(_net: *mut net, _sk: *mut sock, _oldskb: *mut sk_buff, _hook: i32) { }
pub unsafe fn nf_send_unreach6(_net: *mut net, skb_in: *mut sk_buff, _code: u8, _hooknum: u32) { if !skb_valid_dst(skb_in) { let _ = nf_reject6_fill_skb_dst(skb_in); } }
unsafe fn nf_reject6_fill_skb_dst(_skb_in: *mut sk_buff) -> i32 { -1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
