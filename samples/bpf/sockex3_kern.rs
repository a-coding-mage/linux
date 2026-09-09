/* Copyright (c) 2015 PLUMgrid, http://plumgrid.com
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */
// Dependencies supplied by the Linux UAPI, BPF helpers, and bpf_legacy.h.

pub const IP_MF: u32 = 0x2000;
pub const IP_OFFSET: u32 = 0x1FFF;

pub const PARSE_VLAN: u32 = 1;
pub const PARSE_MPLS: u32 = 2;
pub const PARSE_IP: u32 = 3;
pub const PARSE_IPV6: u32 = 4;

#[repr(C)]
pub struct vlan_hdr {
    pub h_vlan_TCI: u16,
    pub h_vlan_encapsulated_proto: u16,
}

#[repr(C)]
pub union flow_key_record_ports {
    pub ports: u32,
    pub port16: [u16; 2],
}

#[repr(C)]
pub struct flow_key_record {
    pub src: u32,
    pub dst: u32,
    pub ports: flow_key_record_ports,
    pub ip_proto: u32,
}

#[repr(C)]
pub struct globals {
    pub flow: flow_key_record,
}

#[repr(C)]
pub struct pair {
    pub packets: u64,
    pub bytes: u64,
}

// External declarations supplied by included headers and the BPF runtime.
#[repr(C)]
pub struct __sk_buff {
    pub cb: [u32; 5],
    pub len: u32,
}

extern "C" {
    fn load_half(ctx: *mut __sk_buff, offset: u64) -> u32;
    fn load_word(ctx: *mut __sk_buff, offset: u64) -> u64;
    fn load_byte(ctx: *mut __sk_buff, offset: u64) -> u32;
    fn bpf_get_smp_processor_id() -> u32;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void, value: *const core::ffi::c_void, flags: u64) -> u64;
    fn bpf_tail_call(ctx: *mut __sk_buff, map: *mut core::ffi::c_void, index: u32);
}

#[repr(C)]
pub struct percpu_map_t {
    pub _opaque: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut percpu_map: percpu_map_t = percpu_map_t { _opaque: [] };

#[repr(C)]
pub struct hash_map_t {
    pub _opaque: [u8; 0],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut hash_map: hash_map_t = hash_map_t { _opaque: [] };

#[inline]
unsafe fn ip_is_fragment(ctx: *mut __sk_buff, nhoff: u64) -> u32 {
    load_half(ctx, nhoff + core::mem::offset_of!(iphdr, frag_off)) & (IP_MF | IP_OFFSET)
}

#[inline]
unsafe fn ipv6_addr_hash(ctx: *mut __sk_buff, off: u64) -> u32 {
    let w0 = load_word(ctx, off);
    let w1 = load_word(ctx, off + 4);
    let w2 = load_word(ctx, off + 8);
    let w3 = load_word(ctx, off + 12);
    (w0 ^ w1 ^ w2 ^ w3) as u32
}

unsafe fn this_cpu_globals() -> *mut globals {
    let key = bpf_get_smp_processor_id();
    bpf_map_lookup_elem(&mut percpu_map as *mut _ as *mut _, &key as *const _ as *const _) as *mut globals
}

unsafe fn update_stats(skb: *mut __sk_buff, g: *mut globals) {
    let key = (*g).flow;
    let value = bpf_map_lookup_elem(&mut hash_map as *mut _ as *mut _, &key as *const _ as *const _) as *mut pair;
    if !value.is_null() {
        (*value).packets = (*value).packets.wrapping_add(1);
        (*value).bytes = (*value).bytes.wrapping_add((*skb).len as u64);
    } else {
        let val = pair { packets: 1, bytes: (*skb).len as u64 };
        bpf_map_update_elem(&mut hash_map as *mut _ as *mut _, &key as *const _ as *const _, &val as *const _ as *const _, 0);
    }
}

#[repr(C)] pub struct iphdr { pub _pad: [u8; 12], pub frag_off: u16, pub _rest: [u8; 8] }
#[repr(C)] pub struct ipv6hdr { pub _pad: [u8; 6], pub nexthdr: u8, pub _rest: [u8; 33], pub saddr: [u8; 16], pub daddr: [u8; 16] }

const IPPROTO_GRE: u32 = 47;
const IPPROTO_IPIP: u32 = 4;
const IPPROTO_IPV6: u32 = 41;
const IPPROTO_TCP: u32 = 6;
const IPPROTO_UDP: u32 = 17;
const IPPROTO_ICMP: u32 = 1;
const GRE_VERSION: u32 = 0x0007;
const GRE_ROUTING: u32 = 0x4000;
const GRE_CSUM: u32 = 0x8000;
const GRE_KEY: u32 = 0x2000;
const GRE_SEQ: u32 = 0x1000;
const ETH_P_IP: u32 = 0x0800;
const ETH_P_IPV6: u32 = 0x86dd;
const ETH_P_8021Q: u32 = 0x8100;
const ETH_P_8021AD: u32 = 0x88a8;
const ETH_P_MPLS_UC: u32 = 0x8847;
const ETH_P_MPLS_MC: u32 = 0x8848;
const ETH_HLEN: u32 = 14;
const MPLS_LS_S_MASK: u32 = 0x00000100;

unsafe fn parse_eth_proto(skb: *mut __sk_buff, proto: u32);

#[inline(always)]
unsafe fn parse_ip_proto(skb: *mut __sk_buff, g: *mut globals, ip_proto: u32) {
    let mut nhoff = (*skb).cb[0];
    match ip_proto {
        IPPROTO_GRE => {
            let gre_flags = load_half(skb, nhoff as u64);
            let gre_proto = load_half(skb, nhoff as u64 + 2);
            if gre_flags & (GRE_VERSION | GRE_ROUTING) != 0 { return; }
            nhoff += 4;
            if gre_flags & GRE_CSUM != 0 { nhoff += 4; }
            if gre_flags & GRE_KEY != 0 { nhoff += 4; }
            if gre_flags & GRE_SEQ != 0 { nhoff += 4; }
            (*skb).cb[0] = nhoff;
            parse_eth_proto(skb, gre_proto);
        }
        IPPROTO_IPIP => parse_eth_proto(skb, ETH_P_IP),
        IPPROTO_IPV6 => parse_eth_proto(skb, ETH_P_IPV6),
        IPPROTO_TCP | IPPROTO_UDP => {
            (*g).flow.ports.ports = load_word(skb, nhoff as u64) as u32;
            (*g).flow.ip_proto = ip_proto;
            update_stats(skb, g);
        }
        IPPROTO_ICMP => { (*g).flow.ip_proto = ip_proto; update_stats(skb, g); }
        _ => {}
    }
}

#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn bpf_func_ip(skb: *mut __sk_buff) -> i32 {
    let g = this_cpu_globals(); if g.is_null() { return 0; }
    let nhoff = (*skb).cb[0];
    if ip_is_fragment(skb, nhoff as u64) != 0 { return 0; }
    let ip_proto = load_byte(skb, nhoff as u64 + core::mem::offset_of!(iphdr, _pad) as u64 + 9);
    if ip_proto != IPPROTO_GRE { (*g).flow.src = load_word(skb, nhoff as u64 + 12) as u32; (*g).flow.dst = load_word(skb, nhoff as u64 + 16) as u32; }
    let verlen = load_byte(skb, nhoff as u64);
    let nhoff = nhoff + ((verlen & 0xF) << 2);
    (*skb).cb[0] = nhoff; parse_ip_proto(skb, g, ip_proto); 0
}

#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn bpf_func_ipv6(skb: *mut __sk_buff) -> i32 {
    let g = this_cpu_globals(); if g.is_null() { return 0; }
    let nhoff = (*skb).cb[0];
    let ip_proto = load_byte(skb, nhoff as u64 + 6);
    (*g).flow.src = ipv6_addr_hash(skb, nhoff as u64 + 8);
    (*g).flow.dst = ipv6_addr_hash(skb, nhoff as u64 + 24);
    (*skb).cb[0] = nhoff + 40; parse_ip_proto(skb, g, ip_proto); 0
}

#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn bpf_func_vlan(skb: *mut __sk_buff) -> i32 {
    let nhoff = (*skb).cb[0]; let proto = load_half(skb, nhoff as u64 + 2); (*skb).cb[0] = nhoff + 4; parse_eth_proto(skb, proto); 0
}

#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn bpf_func_mpls(skb: *mut __sk_buff) -> i32 {
    let nhoff = (*skb).cb[0]; let label = load_word(skb, nhoff as u64) as u32; let nhoff = nhoff + 4; (*skb).cb[0] = nhoff;
    if label & MPLS_LS_S_MASK != 0 { if load_byte(skb, nhoff as u64) & 0xF0 == 4 { parse_eth_proto(skb, ETH_P_IP); } else { parse_eth_proto(skb, ETH_P_IPV6); } } else { parse_eth_proto(skb, ETH_P_MPLS_UC); } 0
}

unsafe fn parse_eth_proto(skb: *mut __sk_buff, proto: u32) {
    let target = match proto { ETH_P_8021Q | ETH_P_8021AD => PARSE_VLAN, ETH_P_MPLS_UC | ETH_P_MPLS_MC => PARSE_MPLS, ETH_P_IP => PARSE_IP, ETH_P_IPV6 => PARSE_IPV6, _ => return };
    bpf_tail_call(skb, core::ptr::null_mut(), target);
}

#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn main_prog(skb: *mut __sk_buff) -> i32 { (*skb).cb[0] = ETH_HLEN; let proto = load_half(skb, 12); parse_eth_proto(skb, proto); 0 }

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
