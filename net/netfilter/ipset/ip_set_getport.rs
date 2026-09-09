// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2003-2011 Jozsef Kadlecsik <kadlec@netfilter.org>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */

/* Get Layer-4 data from the packets */

// Linux kernel headers supplying the following types, constants, and functions
// are external dependencies of this translation.

use core::ffi::c_void;

type __be16 = u16;
type u8_alias = u8;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}
#[repr(C)]
struct iphdr {
    _prefix: [u8; 8],
    protocol: u8,
    _rest: [u8; 3],
    frag_off: __be16,
}
#[repr(C)]
struct ipv6hdr {
    _prefix: [u8; 6],
    nexthdr: u8,
    _rest: [u8; 33],
}
#[repr(C)]
struct tcphdr { source: __be16, dest: __be16, _rest: [u8; 16] }
#[repr(C)]
struct sctphdr { source: __be16, dest: __be16, _rest: [u8; 8] }
#[repr(C)]
struct udphdr { source: __be16, dest: __be16, _rest: [u8; 4] }
#[repr(C)]
struct icmphdr { type_: u8, code: u8, _rest: [u8; 6] }
#[repr(C)]
struct icmp6hdr { icmp6_type: u8, icmp6_code: u8, _rest: [u8; 36] }

extern "C" {
    fn skb_header_pointer(skb: *const sk_buff, offset: u32, len: usize,
                          buffer: *mut c_void) -> *const c_void;
    fn ip_hdr(skb: *const sk_buff) -> *const iphdr;
    fn ipv6_hdr(skb: *const sk_buff) -> *const ipv6hdr;
    fn skb_network_offset(skb: *const sk_buff) -> u32;
    fn ip_hdrlen(skb: *const sk_buff) -> u32;
    fn ipv6_skip_exthdr(skb: *const sk_buff, start: u32, nexthdr: *mut u8,
                        frag_off: *mut __be16) -> i32;
    fn htons(value: u16) -> __be16;
    fn ntohs(value: __be16) -> u16;
}

const IPPROTO_TCP: i32 = 6;
const IPPROTO_SCTP: i32 = 132;
const IPPROTO_UDP: i32 = 17;
const IPPROTO_UDPLITE: i32 = 136;
const IPPROTO_ICMP: i32 = 1;
const IPPROTO_ICMPV6: i32 = 58;
const IP_OFFSET: u16 = 0x1fff;

unsafe fn get_port(skb: *const sk_buff, protocol: i32, protooff: u32,
                   src: bool, port: *mut __be16, proto: *mut u8) -> bool {
    match protocol {
        IPPROTO_TCP => {
            let mut tcph = core::mem::MaybeUninit::<tcphdr>::uninit();
            let th = skb_header_pointer(skb, protooff, core::mem::size_of::<tcphdr>(),
                                         tcph.as_mut_ptr() as *mut c_void) as *const tcphdr;
            if th.is_null() { return false; }
            *port = if src { (*th).source } else { (*th).dest };
        }
        IPPROTO_SCTP => {
            let mut shdr = core::mem::MaybeUninit::<sctphdr>::uninit();
            let sh = skb_header_pointer(skb, protooff, core::mem::size_of::<sctphdr>(),
                                         shdr.as_mut_ptr() as *mut c_void) as *const sctphdr;
            if sh.is_null() { return false; }
            *port = if src { (*sh).source } else { (*sh).dest };
        }
        IPPROTO_UDP | IPPROTO_UDPLITE => {
            let mut udph = core::mem::MaybeUninit::<udphdr>::uninit();
            let uh = skb_header_pointer(skb, protooff, core::mem::size_of::<udphdr>(),
                                         udph.as_mut_ptr() as *mut c_void) as *const udphdr;
            if uh.is_null() { return false; }
            *port = if src { (*uh).source } else { (*uh).dest };
        }
        IPPROTO_ICMP => {
            let mut ich = core::mem::MaybeUninit::<icmphdr>::uninit();
            let ic = skb_header_pointer(skb, protooff, core::mem::size_of::<icmphdr>(),
                                        ich.as_mut_ptr() as *mut c_void) as *const icmphdr;
            if ic.is_null() { return false; }
            *port = htons((((*ic).type_ as u16) << 8) | (*ic).code as u16);
        }
        IPPROTO_ICMPV6 => {
            let mut ich = core::mem::MaybeUninit::<icmp6hdr>::uninit();
            let ic = skb_header_pointer(skb, protooff, core::mem::size_of::<icmp6hdr>(),
                                        ich.as_mut_ptr() as *mut c_void) as *const icmp6hdr;
            if ic.is_null() { return false; }
            *port = htons((((*ic).icmp6_type as u16) << 8) | (*ic).icmp6_code as u16);
        }
        _ => {}
    }
    *proto = protocol as u8;
    true
}

pub unsafe fn ip_set_get_ip4_port(skb: *const sk_buff, src: bool,
                                  port: *mut __be16, proto: *mut u8) -> bool {
    let iph = ip_hdr(skb);
    let protooff = skb_network_offset(skb) + ip_hdrlen(skb);
    let protocol = (*iph).protocol as i32;
    if protocol <= 0 { return false; }
    if ntohs((*iph).frag_off) & IP_OFFSET != 0 {
        match protocol {
            IPPROTO_TCP | IPPROTO_SCTP | IPPROTO_UDP | IPPROTO_UDPLITE | IPPROTO_ICMP => return false,
            _ => { *proto = protocol as u8; return true; }
        }
    }
    get_port(skb, protocol, protooff, src, port, proto)
}

// EXPORT_SYMBOL_GPL(ip_set_get_ip4_port);

// Preserved from the source: this block is enabled when CONFIG_IP6_NF_IPTABLES is enabled.
#[cfg(feature = "CONFIG_IP6_NF_IPTABLES")]
pub unsafe fn ip_set_get_ip6_port(skb: *const sk_buff, src: bool,
                                  port: *mut __be16, proto: *mut u8) -> bool {
    let mut nexthdr = (*ipv6_hdr(skb)).nexthdr;
    let mut frag_off: __be16 = 0;
    let protoff = ipv6_skip_exthdr(skb,
        skb_network_offset(skb) + core::mem::size_of::<ipv6hdr>() as u32,
        &mut nexthdr, &mut frag_off);
    if protoff < 0 || (frag_off & htons(!0x7)) != 0 { return false; }
    get_port(skb, nexthdr as i32, protoff as u32, src, port, proto)
}

// EXPORT_SYMBOL_GPL(ip_set_get_ip6_port);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
