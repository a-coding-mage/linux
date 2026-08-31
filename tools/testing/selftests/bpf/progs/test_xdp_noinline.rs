// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017 Facebook
// Rust translation of testing/selftests/bpf/progs/test_xdp_noinline.c.
// C includes are dependencies supplied by the surrounding BPF build.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __be32 = u32;
type u32 = u32;

const JHASH_INITVAL: u32 = 0xdeadbeef;
const BPF_MAP_TYPE_HASH: __u32 = 1;
const BPF_MAP_TYPE_ARRAY: __u32 = 2;
const BPF_MAP_TYPE_LRU_HASH: __u32 = 9;
const BPF_MAP_TYPE_PERCPU_ARRAY: __u32 = 6;
const XDP_DROP: i32 = 1;
const XDP_PASS: i32 = 2;
const XDP_TX: i32 = 3;
const ETH_P_IP: __u32 = 0x0800;
const ETH_P_IPV6: __u32 = 0x86DD;
const IPPROTO_IPIP: __u8 = 4;
const IPPROTO_TCP: __u8 = 6;
const IPPROTO_UDP: __u8 = 17;
const IPPROTO_ICMP: __u8 = 1;
const IPPROTO_IPV6: __u8 = 41;

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
}

#[repr(C)]
pub struct in6_addr_u {
    pub u6_addr32: [__be32; 4],
}

#[repr(C)]
pub struct in6_addr {
    pub in6_u: in6_addr_u,
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: __u8,
    pub flow_lbl: [__u8; 3],
    pub payload_len: __u16,
    pub nexthdr: __u8,
    pub hop_limit: __u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

impl ipv6hdr {
    unsafe fn set_version(&mut self, version: __u8) {
        self.priority_version = (self.priority_version & 0x0f) | ((version & 0x0f) << 4);
    }

    unsafe fn set_priority(&mut self, priority: __u8) {
        self.priority_version = (self.priority_version & 0xf0) | (priority & 0x0f);
    }
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: __u8,
    pub tos: __u8,
    pub tot_len: __u16,
    pub id: __u16,
    pub frag_off: __u16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __u16,
    pub saddr: __be32,
    pub daddr: __be32,
}

impl iphdr {
    unsafe fn ihl(&self) -> __u8 {
        self.ihl_version & 0x0f
    }

    unsafe fn set_ihl(&mut self, ihl: __u8) {
        self.ihl_version = (self.ihl_version & 0xf0) | (ihl & 0x0f);
    }

    unsafe fn set_version(&mut self, version: __u8) {
        self.ihl_version = (self.ihl_version & 0x0f) | ((version & 0x0f) << 4);
    }
}

#[repr(C)]
pub struct icmphdr {
    pub type_: __u8,
    pub code: __u8,
    pub checksum: __u16,
    pub rest: __u32,
}

#[repr(C)]
pub struct icmp6hdr {
    pub icmp6_type: __u8,
    pub icmp6_code: __u8,
    pub icmp6_cksum: __u16,
    pub icmp6_dataun: __u32,
}

#[repr(C)]
pub struct tcphdr {
    pub source: __u16,
    pub dest: __u16,
    pub seq: __u32,
    pub ack_seq: __u32,
    pub doff_res_flags: __u16,
    pub window: __u16,
    pub check: __u16,
    pub urg_ptr: __u16,
}

impl tcphdr {
    unsafe fn syn(&self) -> bool {
        (self.doff_res_flags & 0x0002) != 0
    }
}

#[repr(C)]
pub struct udphdr {
    pub source: __u16,
    pub dest: __u16,
    pub len: __u16,
    pub check: __u16,
}

#[repr(C)]
pub union flow_key_src {
    pub src: __be32,
    pub srcv6: [__be32; 4],
}

#[repr(C)]
pub union flow_key_dst {
    pub dst: __be32,
    pub dstv6: [__be32; 4],
}

#[repr(C)]
pub union flow_key_ports {
    pub ports: __u32,
    pub port16: [__u16; 2],
}

#[repr(C)]
pub struct flow_key {
    pub src_u: flow_key_src,
    pub dst_u: flow_key_dst,
    pub ports_u: flow_key_ports,
    pub proto: __u8,
}

#[repr(C)]
pub struct packet_description {
    pub flow: flow_key,
    pub flags: __u8,
}

#[repr(C)]
pub union ctl_value_u {
    pub value: __u64,
    pub ifindex: __u32,
    pub mac: [__u8; 6],
}

#[repr(C)]
pub struct ctl_value {
    pub u: ctl_value_u,
}

#[repr(C)]
pub union vip_definition_u {
    pub vip: __be32,
    pub vipv6: [__be32; 4],
}

#[repr(C)]
pub struct vip_definition {
    pub u: vip_definition_u,
    pub port: __u16,
    pub family: __u16,
    pub proto: __u8,
}

#[repr(C)]
pub struct vip_meta {
    pub flags: __u32,
    pub vip_num: __u32,
}

#[repr(C)]
pub struct real_pos_lru {
    pub pos: __u32,
    pub atime: __u64,
}

#[repr(C)]
pub union real_definition_u {
    pub dst: __be32,
    pub dstv6: [__be32; 4],
}

#[repr(C)]
pub struct real_definition {
    pub u: real_definition_u,
    pub flags: __u8,
}

#[repr(C)]
pub struct lb_stats {
    pub v2: __u64,
    pub v1: __u64,
}

#[repr(C)]
pub struct eth_hdr {
    pub eth_dest: [u8; 6],
    pub eth_source: [u8; 6],
    pub eth_proto: u16,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
}

// SEC(".maps") map definitions from the C source.
#[no_mangle]
pub static mut vip_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 512,
    map_flags: 0,
};

#[no_mangle]
pub static mut lru_cache: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_LRU_HASH,
    max_entries: 300,
    map_flags: 1u32 << 1,
};

#[no_mangle]
pub static mut ch_rings: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 12 * 655,
    map_flags: 0,
};

#[no_mangle]
pub static mut reals: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 40,
    map_flags: 0,
};

#[no_mangle]
pub static mut stats: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: 515,
    map_flags: 0,
};

#[no_mangle]
pub static mut ctl_array: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 16,
    map_flags: 0,
};

extern "C" {
    fn bpf_xdp_adjust_head(xdp: *mut xdp_md, delta: i32) -> i64;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_map_update_elem(map: *mut c_void, key: *const c_void, value: *const c_void, flags: __u64) -> i64;
    fn bpf_ktime_get_ns() -> __u64;
    fn bpf_htons(x: __u16) -> __u16;
    fn bpf_ntohs(x: __u16) -> __u16;
}

#[inline(always)]
unsafe fn rol32(word: __u32, shift: u32) -> __u32 {
    word.wrapping_shl(shift).wrapping_or(word.wrapping_shr((0u32.wrapping_sub(shift)) & 31))
}

unsafe fn __jhash_mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a = a.wrapping_sub(*c);
    *a ^= rol32(*c, 4);
    *c = c.wrapping_add(*b);
    *b = b.wrapping_sub(*a);
    *b ^= rol32(*a, 6);
    *a = a.wrapping_add(*c);
    *c = c.wrapping_sub(*b);
    *c ^= rol32(*b, 8);
    *b = b.wrapping_add(*a);
    *a = a.wrapping_sub(*c);
    *a ^= rol32(*c, 16);
    *c = c.wrapping_add(*b);
    *b = b.wrapping_sub(*a);
    *b ^= rol32(*a, 19);
    *a = a.wrapping_add(*c);
    *c = c.wrapping_sub(*b);
    *c ^= rol32(*b, 4);
    *b = b.wrapping_add(*a);
}

unsafe fn __jhash_final(a: &mut u32, b: &mut u32, c: &mut u32) {
    *c ^= *b;
    *c = c.wrapping_sub(rol32(*b, 14));
    *a ^= *c;
    *a = a.wrapping_sub(rol32(*c, 11));
    *b ^= *a;
    *b = b.wrapping_sub(rol32(*a, 25));
    *c ^= *b;
    *c = c.wrapping_sub(rol32(*b, 16));
    *a ^= *c;
    *a = a.wrapping_sub(rol32(*c, 4));
    *b ^= *a;
    *b = b.wrapping_sub(rol32(*a, 14));
    *c ^= *b;
    *c = c.wrapping_sub(rol32(*b, 24));
}

unsafe fn jhash(key: *const c_void, mut length: u32, initval: u32) -> u32 {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut k = key as *const u8;

    c = JHASH_INITVAL.wrapping_add(length).wrapping_add(initval);
    b = c;
    a = b;

    while length > 12 {
        a = a.wrapping_add(ptr::read_unaligned(k as *const u32));
        b = b.wrapping_add(ptr::read_unaligned(k.add(4) as *const u32));
        c = c.wrapping_add(ptr::read_unaligned(k.add(8) as *const u32));
        __jhash_mix(&mut a, &mut b, &mut c);
        length -= 12;
        k = k.add(12);
    }
    match length {
        12 => {
            c = c.wrapping_add(((*k.add(11)) as u32) << 24);
            c = c.wrapping_add(((*k.add(10)) as u32) << 16);
            c = c.wrapping_add(((*k.add(9)) as u32) << 8);
            c = c.wrapping_add((*k.add(8)) as u32);
            b = b.wrapping_add(((*k.add(7)) as u32) << 24);
            b = b.wrapping_add(((*k.add(6)) as u32) << 16);
            b = b.wrapping_add(((*k.add(5)) as u32) << 8);
            b = b.wrapping_add((*k.add(4)) as u32);
            a = a.wrapping_add(((*k.add(3)) as u32) << 24);
            a = a.wrapping_add(((*k.add(2)) as u32) << 16);
            a = a.wrapping_add(((*k.add(1)) as u32) << 8);
            a = a.wrapping_add((*k.add(0)) as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        11 => {
            c = c.wrapping_add(((*k.add(10)) as u32) << 16);
            c = c.wrapping_add(((*k.add(9)) as u32) << 8);
            c = c.wrapping_add((*k.add(8)) as u32);
            b = b.wrapping_add(((*k.add(7)) as u32) << 24);
            b = b.wrapping_add(((*k.add(6)) as u32) << 16);
            b = b.wrapping_add(((*k.add(5)) as u32) << 8);
            b = b.wrapping_add((*k.add(4)) as u32);
            a = a.wrapping_add(((*k.add(3)) as u32) << 24);
            a = a.wrapping_add(((*k.add(2)) as u32) << 16);
            a = a.wrapping_add(((*k.add(1)) as u32) << 8);
            a = a.wrapping_add((*k.add(0)) as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        10 => {
            c = c.wrapping_add(((*k.add(9)) as u32) << 8);
            c = c.wrapping_add((*k.add(8)) as u32);
            b = b.wrapping_add(((*k.add(7)) as u32) << 24);
            b = b.wrapping_add(((*k.add(6)) as u32) << 16);
            b = b.wrapping_add(((*k.add(5)) as u32) << 8);
            b = b.wrapping_add((*k.add(4)) as u32);
            a = a.wrapping_add(((*k.add(3)) as u32) << 24);
            a = a.wrapping_add(((*k.add(2)) as u32) << 16);
            a = a.wrapping_add(((*k.add(1)) as u32) << 8);
            a = a.wrapping_add((*k.add(0)) as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        9 => {
            c = c.wrapping_add((*k.add(8)) as u32);
            b = b.wrapping_add(((*k.add(7)) as u32) << 24);
            b = b.wrapping_add(((*k.add(6)) as u32) << 16);
            b = b.wrapping_add(((*k.add(5)) as u32) << 8);
            b = b.wrapping_add((*k.add(4)) as u32);
            a = a.wrapping_add(((*k.add(3)) as u32) << 24);
            a = a.wrapping_add(((*k.add(2)) as u32) << 16);
            a = a.wrapping_add(((*k.add(1)) as u32) << 8);
            a = a.wrapping_add((*k.add(0)) as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        8 => {
            b = b.wrapping_add(((*k.add(7)) as u32) << 24);
            b = b.wrapping_add(((*k.add(6)) as u32) << 16);
            b = b.wrapping_add(((*k.add(5)) as u32) << 8);
            b = b.wrapping_add((*k.add(4)) as u32);
            a = a.wrapping_add(((*k.add(3)) as u32) << 24);
            a = a.wrapping_add(((*k.add(2)) as u32) << 16);
            a = a.wrapping_add(((*k.add(1)) as u32) << 8);
            a = a.wrapping_add((*k.add(0)) as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        7 => {
            b = b.wrapping_add(((*k.add(6)) as u32) << 16);
            b = b.wrapping_add(((*k.add(5)) as u32) << 8);
            b = b.wrapping_add((*k.add(4)) as u32);
            a = a.wrapping_add(((*k.add(3)) as u32) << 24);
            a = a.wrapping_add(((*k.add(2)) as u32) << 16);
            a = a.wrapping_add(((*k.add(1)) as u32) << 8);
            a = a.wrapping_add((*k.add(0)) as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        6 => {
            b = b.wrapping_add(((*k.add(5)) as u32) << 8);
            b = b.wrapping_add((*k.add(4)) as u32);
            a = a.wrapping_add(((*k.add(3)) as u32) << 24);
            a = a.wrapping_add(((*k.add(2)) as u32) << 16);
            a = a.wrapping_add(((*k.add(1)) as u32) << 8);
            a = a.wrapping_add((*k.add(0)) as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        5 => {
            b = b.wrapping_add((*k.add(4)) as u32);
            a = a.wrapping_add(((*k.add(3)) as u32) << 24);
            a = a.wrapping_add(((*k.add(2)) as u32) << 16);
            a = a.wrapping_add(((*k.add(1)) as u32) << 8);
            a = a.wrapping_add((*k.add(0)) as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        4 => {
            a = a.wrapping_add(((*k.add(3)) as u32) << 24);
            a = a.wrapping_add(((*k.add(2)) as u32) << 16);
            a = a.wrapping_add(((*k.add(1)) as u32) << 8);
            a = a.wrapping_add((*k.add(0)) as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        3 => {
            a = a.wrapping_add(((*k.add(2)) as u32) << 16);
            a = a.wrapping_add(((*k.add(1)) as u32) << 8);
            a = a.wrapping_add((*k.add(0)) as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        2 => {
            a = a.wrapping_add(((*k.add(1)) as u32) << 8);
            a = a.wrapping_add((*k.add(0)) as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        1 => {
            a = a.wrapping_add((*k.add(0)) as u32);
            __jhash_final(&mut a, &mut b, &mut c);
        }
        0 => {}
        _ => {}
    }

    c
}

#[no_mangle]
pub unsafe extern "C" fn __jhash_nwords(mut a: u32, mut b: u32, mut c: u32, initval: u32) -> u32 {
    a = a.wrapping_add(initval);
    b = b.wrapping_add(initval);
    c = c.wrapping_add(initval);
    __jhash_final(&mut a, &mut b, &mut c);
    c
}

#[no_mangle]
pub unsafe extern "C" fn jhash_2words(a: u32, b: u32, initval: u32) -> u32 {
    __jhash_nwords(a, b, 0, initval.wrapping_add(JHASH_INITVAL).wrapping_add(2 << 2))
}

unsafe fn calc_offset(is_ipv6: bool, is_icmp: bool) -> __u64 {
    let mut off = size_of::<eth_hdr>() as __u64;
    if is_ipv6 {
        off += size_of::<ipv6hdr>() as __u64;
        if is_icmp {
            off += (size_of::<icmp6hdr>() + size_of::<ipv6hdr>()) as __u64;
        }
    } else {
        off += size_of::<iphdr>() as __u64;
        if is_icmp {
            off += (size_of::<icmphdr>() + size_of::<iphdr>()) as __u64;
        }
    }
    off
}

unsafe fn parse_udp(data: *mut c_void, data_end: *mut c_void, is_ipv6: bool, pckt: *mut packet_description) -> bool {
    let is_icmp = !(((*pckt).flags & (1 << 0)) == 0);
    let off = calc_offset(is_ipv6, is_icmp);
    let udp = (data as *mut u8).add(off as usize) as *mut udphdr;

    if udp.add(1) as *mut c_void > data_end {
        return false;
    }
    if !is_icmp {
        (*pckt).flow.ports_u.port16[0] = (*udp).source;
        (*pckt).flow.ports_u.port16[1] = (*udp).dest;
    } else {
        (*pckt).flow.ports_u.port16[0] = (*udp).dest;
        (*pckt).flow.ports_u.port16[1] = (*udp).source;
    }
    true
}

unsafe fn parse_tcp(data: *mut c_void, data_end: *mut c_void, is_ipv6: bool, pckt: *mut packet_description) -> bool {
    let is_icmp = !(((*pckt).flags & (1 << 0)) == 0);
    let off = calc_offset(is_ipv6, is_icmp);
    let tcp = (data as *mut u8).add(off as usize) as *mut tcphdr;

    if tcp.add(1) as *mut c_void > data_end {
        return false;
    }
    if (*tcp).syn() {
        (*pckt).flags |= 1 << 1;
    }
    if !is_icmp {
        (*pckt).flow.ports_u.port16[0] = (*tcp).source;
        (*pckt).flow.ports_u.port16[1] = (*tcp).dest;
    } else {
        (*pckt).flow.ports_u.port16[0] = (*tcp).dest;
        (*pckt).flow.ports_u.port16[1] = (*tcp).source;
    }
    true
}

unsafe fn encap_v6(
    xdp: *mut xdp_md,
    cval: *mut ctl_value,
    pckt: *mut packet_description,
    dst: *mut real_definition,
    pkt_bytes: __u32,
) -> bool {
    if bpf_xdp_adjust_head(xdp, 0 - size_of::<ipv6hdr>() as i32) != 0 {
        return false;
    }
    let data = (*xdp).data as usize as *mut c_void;
    let data_end = (*xdp).data_end as usize as *mut c_void;
    let new_eth = data as *mut eth_hdr;
    let ip6h = (data as *mut u8).add(size_of::<eth_hdr>()) as *mut ipv6hdr;
    let old_eth = (data as *mut u8).add(size_of::<ipv6hdr>()) as *mut eth_hdr;
    if new_eth.add(1) as *mut c_void > data_end
        || old_eth.add(1) as *mut c_void > data_end
        || ip6h.add(1) as *mut c_void > data_end
    {
        return false;
    }
    ptr::copy_nonoverlapping((*cval).u.mac.as_ptr(), (*new_eth).eth_dest.as_mut_ptr(), 6);
    ptr::copy_nonoverlapping((*old_eth).eth_dest.as_ptr(), (*new_eth).eth_source.as_mut_ptr(), 6);
    (*new_eth).eth_proto = 56710;
    (*ip6h).set_version(6);
    (*ip6h).set_priority(0);
    ptr::write_bytes((*ip6h).flow_lbl.as_mut_ptr(), 0, (*ip6h).flow_lbl.len());

    (*ip6h).nexthdr = IPPROTO_IPV6;
    let ip_suffix = (*pckt).flow.src_u.srcv6[3] ^ ((*pckt).flow.ports_u.port16[0] as __u32);
    (*ip6h).payload_len = bpf_htons((pkt_bytes as usize + size_of::<ipv6hdr>()) as __u16);
    (*ip6h).hop_limit = 4;

    (*ip6h).saddr.in6_u.u6_addr32[0] = 1;
    (*ip6h).saddr.in6_u.u6_addr32[1] = 2;
    (*ip6h).saddr.in6_u.u6_addr32[2] = 3;
    (*ip6h).saddr.in6_u.u6_addr32[3] = ip_suffix;
    ptr::copy_nonoverlapping((*dst).u.dstv6.as_ptr(), (*ip6h).daddr.in6_u.u6_addr32.as_mut_ptr(), 4);
    true
}

// GCC-only pragmas in the C source disable ipa-sra around encap_v4.
unsafe fn encap_v4(
    xdp: *mut xdp_md,
    cval: *mut ctl_value,
    pckt: *mut packet_description,
    dst: *mut real_definition,
    pkt_bytes: __u32,
) -> bool {
    let mut ip_suffix: __u32 = bpf_ntohs((*pckt).flow.ports_u.port16[0]) as __u32;
    let mut csum: __u32 = 0;

    ip_suffix <<= 15;
    ip_suffix ^= (*pckt).flow.src_u.src;
    if bpf_xdp_adjust_head(xdp, 0 - size_of::<iphdr>() as i32) != 0 {
        return false;
    }
    let data = (*xdp).data as usize as *mut c_void;
    let data_end = (*xdp).data_end as usize as *mut c_void;
    let new_eth = data as *mut eth_hdr;
    let iph = (data as *mut u8).add(size_of::<eth_hdr>()) as *mut iphdr;
    let old_eth = (data as *mut u8).add(size_of::<iphdr>()) as *mut eth_hdr;
    if new_eth.add(1) as *mut c_void > data_end
        || old_eth.add(1) as *mut c_void > data_end
        || iph.add(1) as *mut c_void > data_end
    {
        return false;
    }
    ptr::copy_nonoverlapping((*cval).u.mac.as_ptr(), (*new_eth).eth_dest.as_mut_ptr(), 6);
    ptr::copy_nonoverlapping((*old_eth).eth_dest.as_ptr(), (*new_eth).eth_source.as_mut_ptr(), 6);
    (*new_eth).eth_proto = 8;
    (*iph).set_version(4);
    (*iph).set_ihl(5);
    (*iph).frag_off = 0;
    (*iph).protocol = IPPROTO_IPIP;
    (*iph).check = 0;
    (*iph).tos = 1;
    (*iph).tot_len = bpf_htons((pkt_bytes as usize + size_of::<iphdr>()) as __u16);
    /* don't update iph->daddr, since it will overwrite old eth_proto
     * and multiple iterations of bpf_prog_run() will fail
     */

    (*iph).saddr = ((0xFFFF0000 & ip_suffix) | 4268) ^ (*dst).u.dst;
    (*iph).ttl = 4;

    let mut next_iph_u16 = iph as *mut __u16;
    for _i in 0..(size_of::<iphdr>() >> 1) {
        csum = csum.wrapping_add(*next_iph_u16 as __u32);
        next_iph_u16 = next_iph_u16.add(1);
    }
    (*iph).check = !(((csum & 0xffff).wrapping_add(csum >> 16)) as __u16);
    if bpf_xdp_adjust_head(xdp, size_of::<iphdr>() as i32) != 0 {
        return false;
    }
    true
}

unsafe fn swap_mac_and_send(data: *mut c_void, _data_end: *mut c_void) -> i32 {
    let mut tmp_mac = [0u8; 6];
    let eth = data as *mut eth_hdr;

    ptr::copy_nonoverlapping((*eth).eth_source.as_ptr(), tmp_mac.as_mut_ptr(), 6);
    ptr::copy_nonoverlapping((*eth).eth_dest.as_ptr(), (*eth).eth_source.as_mut_ptr(), 6);
    ptr::copy_nonoverlapping(tmp_mac.as_ptr(), (*eth).eth_dest.as_mut_ptr(), 6);
    XDP_TX
}

unsafe fn send_icmp_reply(data: *mut c_void, data_end: *mut c_void) -> i32 {
    let mut tmp_addr: __u32;
    let mut csum: __u32 = 0;
    let mut off: __u64 = 0;

    if (data as *mut u8).add(size_of::<eth_hdr>() + size_of::<iphdr>() + size_of::<icmphdr>()) as *mut c_void > data_end {
        return XDP_DROP;
    }
    off += size_of::<eth_hdr>() as __u64;
    let iph = (data as *mut u8).add(off as usize) as *mut iphdr;
    off += size_of::<iphdr>() as __u64;
    let icmp_hdr = (data as *mut u8).add(off as usize) as *mut icmphdr;
    (*icmp_hdr).type_ = 0;
    (*icmp_hdr).checksum = (*icmp_hdr).checksum.wrapping_add(0x0007);
    (*iph).ttl = 4;
    tmp_addr = (*iph).daddr;
    (*iph).daddr = (*iph).saddr;
    (*iph).saddr = tmp_addr;
    (*iph).check = 0;
    let mut next_iph_u16 = iph as *mut __u16;
    for _i in 0..(size_of::<iphdr>() >> 1) {
        csum = csum.wrapping_add(*next_iph_u16 as __u32);
        next_iph_u16 = next_iph_u16.add(1);
    }
    (*iph).check = !(((csum & 0xffff).wrapping_add(csum >> 16)) as __u16);
    swap_mac_and_send(data, data_end)
}

unsafe fn send_icmp6_reply(data: *mut c_void, data_end: *mut c_void) -> i32 {
    let mut tmp_addr: [__be32; 4] = [0; 4];
    let mut off: __u64 = 0;

    if (data as *mut u8).add(size_of::<eth_hdr>() + size_of::<ipv6hdr>() + size_of::<icmp6hdr>()) as *mut c_void > data_end {
        return XDP_DROP;
    }
    off += size_of::<eth_hdr>() as __u64;
    let ip6h = (data as *mut u8).add(off as usize) as *mut ipv6hdr;
    off += size_of::<ipv6hdr>() as __u64;
    let icmp_hdr = (data as *mut u8).add(off as usize) as *mut icmp6hdr;
    (*icmp_hdr).icmp6_type = 129;
    (*icmp_hdr).icmp6_cksum = (*icmp_hdr).icmp6_cksum.wrapping_sub(0x0001);
    (*ip6h).hop_limit = 4;
    ptr::copy_nonoverlapping((*ip6h).saddr.in6_u.u6_addr32.as_ptr(), tmp_addr.as_mut_ptr(), 4);
    ptr::copy_nonoverlapping((*ip6h).daddr.in6_u.u6_addr32.as_ptr(), (*ip6h).saddr.in6_u.u6_addr32.as_mut_ptr(), 4);
    ptr::copy_nonoverlapping(tmp_addr.as_ptr(), (*ip6h).daddr.in6_u.u6_addr32.as_mut_ptr(), 4);
    swap_mac_and_send(data, data_end)
}

unsafe fn parse_icmpv6(data: *mut c_void, data_end: *mut c_void, mut off: __u64, pckt: *mut packet_description) -> i32 {
    let icmp_hdr = (data as *mut u8).add(off as usize) as *mut icmp6hdr;
    if icmp_hdr.add(1) as *mut c_void > data_end {
        return XDP_DROP;
    }
    if (*icmp_hdr).icmp6_type == 128 {
        return send_icmp6_reply(data, data_end);
    }
    if (*icmp_hdr).icmp6_type != 3 {
        return XDP_PASS;
    }
    off += size_of::<icmp6hdr>() as __u64;
    let ip6h = (data as *mut u8).add(off as usize) as *mut ipv6hdr;
    if ip6h.add(1) as *mut c_void > data_end {
        return XDP_DROP;
    }
    (*pckt).flow.proto = (*ip6h).nexthdr;
    (*pckt).flags |= 1 << 0;
    ptr::copy_nonoverlapping((*ip6h).daddr.in6_u.u6_addr32.as_ptr(), (*pckt).flow.src_u.srcv6.as_mut_ptr(), 4);
    ptr::copy_nonoverlapping((*ip6h).saddr.in6_u.u6_addr32.as_ptr(), (*pckt).flow.dst_u.dstv6.as_mut_ptr(), 4);
    -1
}

unsafe fn parse_icmp(data: *mut c_void, data_end: *mut c_void, mut off: __u64, pckt: *mut packet_description) -> i32 {
    let icmp_hdr = (data as *mut u8).add(off as usize) as *mut icmphdr;
    if icmp_hdr.add(1) as *mut c_void > data_end {
        return XDP_DROP;
    }
    if (*icmp_hdr).type_ == 8 {
        return send_icmp_reply(data, data_end);
    }
    if ((*icmp_hdr).type_ != 3) || ((*icmp_hdr).code != 4) {
        return XDP_PASS;
    }
    off += size_of::<icmphdr>() as __u64;
    let iph = (data as *mut u8).add(off as usize) as *mut iphdr;
    if iph.add(1) as *mut c_void > data_end {
        return XDP_DROP;
    }
    if (*iph).ihl() != 5 {
        return XDP_DROP;
    }
    (*pckt).flow.proto = (*iph).protocol;
    (*pckt).flags |= 1 << 0;
    (*pckt).flow.src_u.src = (*iph).daddr;
    (*pckt).flow.dst_u.dst = (*iph).saddr;
    -1
}

unsafe fn get_packet_hash(pckt: *mut packet_description, hash_16bytes: bool) -> __u32 {
    if hash_16bytes {
        jhash_2words(
            jhash((*pckt).flow.src_u.srcv6.as_ptr() as *const c_void, 16, 12),
            (*pckt).flow.ports_u.ports,
            24,
        )
    } else {
        jhash_2words((*pckt).flow.src_u.src, (*pckt).flow.ports_u.ports, 24)
    }
}

unsafe fn get_packet_dst(
    real: *mut *mut real_definition,
    pckt: *mut packet_description,
    vip_info: *mut vip_meta,
    is_ipv6: bool,
    lru_map: *mut c_void,
) -> bool {
    let mut new_dst_lru: real_pos_lru = core::mem::zeroed();
    let mut hash_16bytes = is_ipv6;
    let mut cur_time: __u64;

    if ((*vip_info).flags & (1 << 2)) != 0 {
        hash_16bytes = true;
    }
    if ((*vip_info).flags & (1 << 3)) != 0 {
        (*pckt).flow.ports_u.port16[0] = (*pckt).flow.ports_u.port16[1];
        ptr::write_bytes((*pckt).flow.src_u.srcv6.as_mut_ptr(), 0, 4);
    }
    let hash = get_packet_hash(pckt, hash_16bytes);
    if hash != 0x358459b7 && hash != 0x2f4bc6bb {
        return false;
    }
    let mut key: __u32 = 2 * (*vip_info).vip_num + hash % 2;
    let real_pos = bpf_map_lookup_elem(&mut ch_rings as *mut _ as *mut c_void, &key as *const _ as *const c_void) as *mut __u32;
    if real_pos.is_null() {
        return false;
    }
    key = *real_pos;
    *real = bpf_map_lookup_elem(&mut reals as *mut _ as *mut c_void, &key as *const _ as *const c_void) as *mut real_definition;
    if (*real).is_null() {
        return false;
    }
    if ((*vip_info).flags & (1 << 1)) == 0 {
        let conn_rate_key: __u32 = 512 + 2;
        let conn_rate_stats = bpf_map_lookup_elem(
            &mut stats as *mut _ as *mut c_void,
            &conn_rate_key as *const _ as *const c_void,
        ) as *mut lb_stats;

        if conn_rate_stats.is_null() {
            return true;
        }
        cur_time = bpf_ktime_get_ns();
        if ((cur_time.wrapping_sub((*conn_rate_stats).v2)) >> 32) > 0xffFFFF {
            (*conn_rate_stats).v1 = 1;
            (*conn_rate_stats).v2 = cur_time;
        } else {
            (*conn_rate_stats).v1 = (*conn_rate_stats).v1.wrapping_add(1);
            if (*conn_rate_stats).v1 >= 1 {
                return true;
            }
        }
        if (*pckt).flow.proto == IPPROTO_UDP {
            new_dst_lru.atime = cur_time;
        }
        new_dst_lru.pos = key;
        bpf_map_update_elem(
            lru_map,
            &(*pckt).flow as *const _ as *const c_void,
            &new_dst_lru as *const _ as *const c_void,
            0,
        );
    }
    true
}

unsafe fn connection_table_lookup(real: *mut *mut real_definition, pckt: *mut packet_description, lru_map: *mut c_void) {
    let dst_lru = bpf_map_lookup_elem(lru_map, &(*pckt).flow as *const _ as *const c_void) as *mut real_pos_lru;
    if dst_lru.is_null() {
        return;
    }
    if (*pckt).flow.proto == IPPROTO_UDP {
        let cur_time = bpf_ktime_get_ns();
        if cur_time.wrapping_sub((*dst_lru).atime) > 300000 {
            return;
        }
        (*dst_lru).atime = cur_time;
    }
    let key = (*dst_lru).pos;
    *real = bpf_map_lookup_elem(&mut reals as *mut _ as *mut c_void, &key as *const _ as *const c_void) as *mut real_definition;
}

/* don't believe your eyes!
 * below function has 6 arguments whereas bpf and llvm allow maximum of 5
 * but since it's _static_ llvm can optimize one argument away
 */
unsafe fn process_l3_headers_v6(
    pckt: *mut packet_description,
    protocol: *mut __u8,
    mut off: __u64,
    pkt_bytes: *mut __u16,
    extra_args: *mut *mut c_void,
) -> i32 {
    let data = *extra_args.add(0);
    let data_end = *extra_args.add(1);

    let ip6h = (data as *mut u8).add(off as usize) as *mut ipv6hdr;
    if ip6h.add(1) as *mut c_void > data_end {
        return XDP_DROP;
    }
    let iph_len = size_of::<ipv6hdr>() as __u64;
    *protocol = (*ip6h).nexthdr;
    (*pckt).flow.proto = *protocol;
    *pkt_bytes = bpf_ntohs((*ip6h).payload_len);
    off += iph_len;
    if *protocol == 45 {
        return XDP_DROP;
    } else if *protocol == 59 {
        let action = parse_icmpv6(data, data_end, off, pckt);
        if action >= 0 {
            return action;
        }
    } else {
        ptr::copy_nonoverlapping((*ip6h).saddr.in6_u.u6_addr32.as_ptr(), (*pckt).flow.src_u.srcv6.as_mut_ptr(), 4);
        ptr::copy_nonoverlapping((*ip6h).daddr.in6_u.u6_addr32.as_ptr(), (*pckt).flow.dst_u.dstv6.as_mut_ptr(), 4);
    }
    -1
}

unsafe fn process_l3_headers_v4(
    pckt: *mut packet_description,
    protocol: *mut __u8,
    mut off: __u64,
    pkt_bytes: *mut __u16,
    extra_args: *mut *mut c_void,
) -> i32 {
    let data = *extra_args.add(0);
    let data_end = *extra_args.add(1);

    let iph = (data as *mut u8).add(off as usize) as *mut iphdr;
    if iph.add(1) as *mut c_void > data_end {
        return XDP_DROP;
    }
    if (*iph).ihl() != 5 {
        return XDP_DROP;
    }
    *protocol = (*iph).protocol;
    (*pckt).flow.proto = *protocol;
    *pkt_bytes = bpf_ntohs((*iph).tot_len);
    off += 20;
    if ((*iph).frag_off & 65343) != 0 {
        return XDP_DROP;
    }
    if *protocol == IPPROTO_ICMP {
        let action = parse_icmp(data, data_end, off, pckt);
        if action >= 0 {
            return action;
        }
    } else {
        (*pckt).flow.src_u.src = (*iph).saddr;
        (*pckt).flow.dst_u.dst = (*iph).daddr;
    }
    -1
}

unsafe fn process_packet(
    mut data: *mut c_void,
    off: __u64,
    mut data_end: *mut c_void,
    is_ipv6: bool,
    xdp: *mut xdp_md,
) -> i32 {
    let mut dst: *mut real_definition = ptr::null_mut();
    let mut pckt: packet_description = core::mem::zeroed();
    let mut vip: vip_definition = core::mem::zeroed();
    let lru_map = &mut lru_cache as *mut _ as *mut c_void;
    let lru_stats_key: __u32 = 513;
    let mac_addr_pos: __u32 = 0;
    let stats_key: __u32 = 512;
    let mut pkt_bytes: __u16 = 0;
    let mut protocol: __u8 = 0;
    let mut extra_args: [*mut c_void; 2] = [data, data_end];

    let action = if is_ipv6 {
        process_l3_headers_v6(&mut pckt, &mut protocol, off, &mut pkt_bytes, extra_args.as_mut_ptr())
    } else {
        process_l3_headers_v4(&mut pckt, &mut protocol, off, &mut pkt_bytes, extra_args.as_mut_ptr())
    };
    if action >= 0 {
        return action;
    }
    protocol = pckt.flow.proto;
    if protocol == IPPROTO_TCP {
        if !parse_tcp(data, data_end, is_ipv6, &mut pckt) {
            return XDP_DROP;
        }
    } else if protocol == IPPROTO_UDP {
        if !parse_udp(data, data_end, is_ipv6, &mut pckt) {
            return XDP_DROP;
        }
    } else {
        return XDP_TX;
    }

    if is_ipv6 {
        ptr::copy_nonoverlapping(pckt.flow.dst_u.dstv6.as_ptr(), vip.u.vipv6.as_mut_ptr(), 4);
    } else {
        vip.u.vip = pckt.flow.dst_u.dst;
    }
    vip.port = pckt.flow.ports_u.port16[1];
    vip.proto = pckt.flow.proto;
    let mut vip_info = bpf_map_lookup_elem(&mut vip_map as *mut _ as *mut c_void, &vip as *const _ as *const c_void) as *mut vip_meta;
    if vip_info.is_null() {
        vip.port = 0;
        vip_info = bpf_map_lookup_elem(&mut vip_map as *mut _ as *mut c_void, &vip as *const _ as *const c_void) as *mut vip_meta;
        if vip_info.is_null() {
            return XDP_PASS;
        }
        if ((*vip_info).flags & (1 << 4)) == 0 {
            pckt.flow.ports_u.port16[1] = 0;
        }
    }
    if (data_end as usize).wrapping_sub(data as usize) > 1400 {
        return XDP_DROP;
    }
    let mut data_stats = bpf_map_lookup_elem(&mut stats as *mut _ as *mut c_void, &stats_key as *const _ as *const c_void) as *mut lb_stats;
    if data_stats.is_null() {
        return XDP_DROP;
    }
    (*data_stats).v1 = (*data_stats).v1.wrapping_add(1);
    if dst.is_null() {
        if ((*vip_info).flags & (1 << 0)) != 0 {
            pckt.flow.ports_u.port16[0] = 0;
        }
        if (pckt.flags & (1 << 1)) == 0 && ((*vip_info).flags & (1 << 1)) == 0 {
            connection_table_lookup(&mut dst, &mut pckt, lru_map);
        }
        if dst.is_null() {
            if pckt.flow.proto == IPPROTO_TCP {
                let lru_stats = bpf_map_lookup_elem(
                    &mut stats as *mut _ as *mut c_void,
                    &lru_stats_key as *const _ as *const c_void,
                ) as *mut lb_stats;

                if lru_stats.is_null() {
                    return XDP_DROP;
                }
                if (pckt.flags & (1 << 1)) != 0 {
                    (*lru_stats).v1 = (*lru_stats).v1.wrapping_add(1);
                } else {
                    (*lru_stats).v2 = (*lru_stats).v2.wrapping_add(1);
                }
            }
            if !get_packet_dst(&mut dst, &mut pckt, vip_info, is_ipv6, lru_map) {
                return XDP_DROP;
            }
            (*data_stats).v2 = (*data_stats).v2.wrapping_add(1);
        }
    }

    let cval = bpf_map_lookup_elem(
        &mut ctl_array as *mut _ as *mut c_void,
        &mac_addr_pos as *const _ as *const c_void,
    ) as *mut ctl_value;
    if cval.is_null() {
        return XDP_DROP;
    }
    if ((*dst).flags & (1 << 0)) != 0 {
        if !encap_v6(xdp, cval, &mut pckt, dst, pkt_bytes as __u32) {
            return XDP_DROP;
        }
    } else if !encap_v4(xdp, cval, &mut pckt, dst, pkt_bytes as __u32) {
        return XDP_DROP;
    }
    let vip_num = (*vip_info).vip_num;
    data_stats = bpf_map_lookup_elem(&mut stats as *mut _ as *mut c_void, &vip_num as *const _ as *const c_void) as *mut lb_stats;
    if data_stats.is_null() {
        return XDP_DROP;
    }
    (*data_stats).v1 = (*data_stats).v1.wrapping_add(1);
    (*data_stats).v2 = (*data_stats).v2.wrapping_add(pkt_bytes as __u64);

    data = (*xdp).data as usize as *mut c_void;
    data_end = (*xdp).data_end as usize as *mut c_void;
    if (data as *mut u8).add(4) as *mut c_void > data_end {
        return XDP_DROP;
    }
    *(data as *mut u32) = (*dst).u.dst;
    XDP_DROP
}

// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn balancer_ingress_v4(ctx: *mut xdp_md) -> i32 {
    let data = (*ctx).data as usize as *mut c_void;
    let data_end = (*ctx).data_end as usize as *mut c_void;
    let eth = data as *mut eth_hdr;
    let nh_off: __u32 = size_of::<eth_hdr>() as __u32;

    if (data as *mut u8).add(nh_off as usize) as *mut c_void > data_end {
        return XDP_DROP;
    }
    let eth_proto: __u32 = bpf_ntohs((*eth).eth_proto) as __u32;
    if eth_proto == ETH_P_IP {
        process_packet(data, nh_off as __u64, data_end, false, ctx)
    } else {
        XDP_DROP
    }
}

// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn balancer_ingress_v6(ctx: *mut xdp_md) -> i32 {
    let data = (*ctx).data as usize as *mut c_void;
    let data_end = (*ctx).data_end as usize as *mut c_void;
    let eth = data as *mut eth_hdr;
    let nh_off: __u32 = size_of::<eth_hdr>() as __u32;

    if (data as *mut u8).add(nh_off as usize) as *mut c_void > data_end {
        return XDP_DROP;
    }
    let eth_proto: __u32 = bpf_ntohs((*eth).eth_proto) as __u32;
    if eth_proto == ETH_P_IPV6 {
        process_packet(data, nh_off as __u64, data_end, true, ctx)
    } else {
        XDP_DROP
    }
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
