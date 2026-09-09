// SPDX-License-Identifier: GPL-2.0-or-later
/* Extension Header handling for IPv6 -- source-level Rust translation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* The surrounding kernel translation supplies these C-compatible definitions. */
use core::ffi::c_void;

extern "C" {
    fn kfree_skb_reason(skb: *mut sk_buff, reason: u32);
    fn kfree_skb(skb: *mut sk_buff);
    fn skb_network_header(skb: *mut sk_buff) -> *mut u8;
    fn skb_transport_header(skb: *mut sk_buff) -> *mut u8;
    fn skb_network_header_len(skb: *mut sk_buff) -> i32;
    fn skb_transport_offset(skb: *mut sk_buff) -> i32;
    fn ipv6_addr_is_multicast(a: *const in6_addr) -> bool;
    fn icmpv6_param_prob_reason(skb: *mut sk_buff, t: i32, off: i32, reason: u32);
    fn ipv6_hop_ra(skb: *mut sk_buff, off: i32) -> bool;
}

#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct ipv6hdr { pub daddr: in6_addr, pub saddr: in6_addr, pub payload_len: u16, pub hop_limit: u8 }
#[repr(C)] pub struct ipv6_rt_hdr { pub nexthdr: u8, pub hdrlen: u8, pub type_: u8, pub segments_left: u8 }
#[repr(C)] pub struct ipv6_opt_hdr { pub nexthdr: u8, pub hdrlen: u8 }
#[repr(C)] pub struct inet6_protocol { pub handler: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>, pub flags: u32 }
#[repr(C)] pub struct inet6_skb_parm { pub flags: u32, pub lastopt: i32, pub dst1: i32, pub dst0: i32, pub srcrt: i32, pub nhoff: i32, pub dsthao: i32, pub dst1opt: *mut ipv6_opt_hdr }
#[repr(C)] pub struct ipv6_txoptions { pub tot_len: i32, pub opt_nflen: i32, pub opt_flen: i32, pub hopopt: *mut ipv6_opt_hdr, pub dst0opt: *mut ipv6_opt_hdr, pub dst1opt: *mut ipv6_opt_hdr, pub srcrt: *mut ipv6_rt_hdr, pub refcnt: u32 }
#[repr(C)] pub struct flowi6 { pub daddr: in6_addr }

const IPV6_TLV_PAD1: u8 = 0;
const IPV6_TLV_PADN: u8 = 1;
const IPV6_TLV_ROUTERALERT: u8 = 5;
const IPV6_TLV_IOAM: u8 = 49;
const IPV6_TLV_JUMBO: u8 = 194;
const IPV6_TLV_CALIPSO: u8 = 7;
const NEXTHDR_ROUTING: u8 = 43;
const NEXTHDR_HOP: u8 = 0;
const NEXTHDR_DEST: u8 = 60;
const NEXTHDR_IPV6: u8 = 41;
const NEXTHDR_IPV4: u8 = 4;

unsafe fn ip6_tlvopt_unknown(skb: *mut sk_buff, optoff: i32, disallow: bool) -> bool {
    if disallow { kfree_skb_reason(skb, 0); return false; }
    let nh = skb_network_header(skb);
    match ((*nh.add(optoff as usize) & 0xc0) >> 6) {
        0 => true,
        1 => { kfree_skb_reason(skb, 0); false },
        3 | 2 => { icmpv6_param_prob_reason(skb, 2, optoff, 0); false },
        _ => false,
    }
}

/* Parse TLV encoded option header (hop-by-hop or destination). */
unsafe fn ip6_parse_tlv(hopbyhop: bool, skb: *mut sk_buff, mut max_count: i32) -> bool {
    let mut len = ((*skb_transport_header(skb).add(1) as i32) + 1) << 3;
    let nh = skb_network_header(skb);
    let mut off = skb_network_header_len(skb) + 2;
    let disallow = max_count < 0;
    if disallow { max_count = -max_count; }
    let mut count = 0;
    let mut padlen = 0;
    while len > 0 {
        if *nh.add(off as usize) == IPV6_TLV_PAD1 { padlen += 1; if padlen > 7 { break; } off += 1; len -= 1; continue; }
        if len < 2 { break; }
        let optlen = *nh.add((off + 1) as usize) as i32 + 2;
        if optlen > len { break; }
        if *nh.add(off as usize) == IPV6_TLV_PADN {
            padlen += optlen; if padlen > 7 { break; }
            for i in 2..optlen { if *nh.add((off+i) as usize) != 0 { break; } }
        } else {
            count += 1; if count > max_count { break; }
            let ok = if hopbyhop && *nh.add(off as usize) == IPV6_TLV_ROUTERALERT { ipv6_hop_ra(skb, off) } else { ip6_tlvopt_unknown(skb, off, disallow) };
            if !ok { return false; }
            padlen = 0;
        }
        off += optlen; len -= optlen;
    }
    if len == 0 { true } else { kfree_skb_reason(skb, 0); false }
}

/* External kernel entry points and less-local handlers retain their interfaces. */
extern "C" {
    pub fn ipv6_destopt_rcv(skb: *mut sk_buff) -> i32;
    pub fn ipv6_rthdr_rcv(skb: *mut sk_buff) -> i32;
    pub fn ipv6_parse_hopopts(skb: *mut sk_buff) -> i32;
    pub fn ipv6_push_nfrag_opts(skb: *mut sk_buff, opt: *mut ipv6_txoptions, proto: u8, daddr: *mut *mut in6_addr, saddr: *mut in6_addr) -> u8;
    pub fn ipv6_push_frag_opts(skb: *mut sk_buff, opt: *mut ipv6_txoptions, proto: u8) -> u8;
    pub fn ipv6_dup_options(sk: *mut c_void, opt: *mut ipv6_txoptions) -> *mut ipv6_txoptions;
    pub fn ipv6_renew_options(sk: *mut c_void, opt: *mut ipv6_txoptions, newtype: i32, newopt: *mut ipv6_opt_hdr) -> *mut ipv6_txoptions;
    pub fn __ipv6_fixup_options(space: *mut ipv6_txoptions, opt: *mut ipv6_txoptions) -> *mut ipv6_txoptions;
    pub fn __fl6_update_dst(fl6: *mut flowi6, opt: *const ipv6_txoptions, orig: *mut in6_addr) -> *mut in6_addr;
}

#[no_mangle] pub unsafe extern "C" fn ipv6_exthdrs_init() -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn ipv6_exthdrs_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
