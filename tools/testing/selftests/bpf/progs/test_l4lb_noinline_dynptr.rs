// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017 Facebook

// Rust translation of testing/selftests/bpf/progs/test_l4lb_noinline_dynptr.c.
// Kernel/BPF types, constants, SEC annotations, map declarations, and helpers
// come from the original C includes:
// <linux/pkt_cls.h>, <linux/bpf.h>, <linux/in.h>, <linux/if_ether.h>,
// <linux/ip.h>, <linux/ipv6.h>, <linux/icmp.h>, <linux/icmpv6.h>,
// <linux/tcp.h>, <linux/udp.h>, <bpf/bpf_helpers.h>,
// "test_iptunnel_common.h", <bpf/bpf_endian.h>, and "bpf_kfuncs.h".

type u32 = ::core::ffi::c_uint;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __be32 = u32;

const JHASH_INITVAL: u32 = 0xdeadbeef;

const PCKT_FRAGMENTED: __u32 = 65343;
const IPV4_HDR_LEN_NO_OPT: __u64 = 20;
const IPV4_PLUS_ICMP_HDR: __u64 = 28;
const IPV6_PLUS_ICMP_HDR: __u64 = 48;
const RING_SIZE: __u32 = 2;
const MAX_VIPS: __u32 = 12;
const MAX_REALS: __u32 = 5;
const CTL_MAP_SIZE: __u32 = 16;
const CH_RINGS_SIZE: __u32 = MAX_VIPS * RING_SIZE;
const F_IPV6: __u32 = 1 << 0;
const F_HASH_NO_SRC_PORT: __u32 = 1 << 0;
const F_ICMP: __u8 = 1 << 0;
const F_SYN_SET: __u8 = 1 << 1;

unsafe extern "C" {
    static mut vip_map: ::core::ffi::c_void;
    static mut ch_rings: ::core::ffi::c_void;
    static mut reals: ::core::ffi::c_void;
    static mut stats: ::core::ffi::c_void;
    static mut ctl_array: ::core::ffi::c_void;

    fn bpf_map_lookup_elem(
        map: *mut ::core::ffi::c_void,
        key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn bpf_dynptr_slice(
        ptr: *mut bpf_dynptr,
        offset: __u64,
        buffer: *mut ::core::ffi::c_void,
        buffer__sz: __u32,
    ) -> *mut ::core::ffi::c_void;
    fn bpf_dynptr_slice_rdwr(
        ptr: *mut bpf_dynptr,
        offset: __u64,
        buffer: *mut ::core::ffi::c_void,
        buffer__sz: __u32,
    ) -> *mut ::core::ffi::c_void;
    fn bpf_dynptr_from_skb(skb: *mut __sk_buff, flags: __u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_write(
        ptr: *mut bpf_dynptr,
        offset: __u64,
        data: *const ::core::ffi::c_void,
        len: __u32,
        flags: __u64,
    ) -> i32;
    fn bpf_skb_set_tunnel_key(
        skb: *mut __sk_buff,
        key: *mut bpf_tunnel_key,
        size: __u32,
        flags: i32,
    ) -> i32;
    fn bpf_redirect(ifindex: __u32, flags: __u64) -> i32;
    fn bpf_ntohs(val: __u16) -> __u16;
    fn bpf_htons(val: __u16) -> __u16;
}

unsafe extern "C" {
    static TC_ACT_SHOT: i32;
    static TC_ACT_OK: i32;
    static TC_ACT_UNSPEC: i32;
    static ICMPV6_PKT_TOOBIG: __u8;
    static ICMP_DEST_UNREACH: __u8;
    static ICMP_FRAG_NEEDED: __u8;
    static IPPROTO_FRAGMENT: __u8;
    static IPPROTO_ICMPV6: __u8;
    static IPPROTO_ICMP: __u8;
    static IPPROTO_TCP: __u8;
    static IPPROTO_UDP: __u8;
    static ETH_P_IP: __u16;
    static ETH_P_IPV6: __u16;
    static BPF_F_TUNINFO_IPV6: i32;
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_tunnel_key {
    pub tunnel_id: __u32,
    pub remote_ipv4: __u32,
    pub tunnel_tos: __u8,
    pub tunnel_ttl: __u8,
    pub tunnel_ext: __u16,
    pub tunnel_label: __u32,
    pub local_ipv4: __u32,
    pub remote_ipv6: [__u32; 4],
    pub local_ipv6: [__u32; 4],
}

#[repr(C)]
pub union packet_description_src {
    pub src: __be32,
    pub srcv6: [__be32; 4],
}

#[repr(C)]
pub union packet_description_dst {
    pub dst: __be32,
    pub dstv6: [__be32; 4],
}

#[repr(C)]
pub union packet_description_ports {
    pub ports: __u32,
    pub port16: [__u16; 2],
}

#[repr(C)]
pub struct packet_description {
    pub src_u: packet_description_src,
    pub dst_u: packet_description_dst,
    pub ports_u: packet_description_ports,
    pub proto: __u8,
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
pub struct vip_meta {
    pub flags: __u32,
    pub vip_num: __u32,
}

#[repr(C)]
pub union real_definition_dst {
    pub dst: __be32,
    pub dstv6: [__be32; 4],
}

#[repr(C)]
pub struct real_definition {
    pub dst_u: real_definition_dst,
    pub flags: __u8,
}

#[repr(C)]
pub struct vip_stats {
    pub bytes: __u64,
    pub pkts: __u64,
}

#[repr(C)]
pub struct eth_hdr {
    pub eth_dest: [u8; 6],
    pub eth_source: [u8; 6],
    pub eth_proto: u16,
}

#[repr(C)]
pub union vip_daddr {
    pub v4: __be32,
    pub v6: [__be32; 4],
}

#[repr(C)]
pub struct vip {
    pub daddr: vip_daddr,
    pub dport: __u16,
    pub protocol: __u8,
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr32: [__be32; 4],
}

#[repr(C)]
pub struct ipv6hdr {
    pub payload_len: __u16,
    pub nexthdr: __u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
pub struct iphdr {
    pub ihl: __u8,
    pub protocol: __u8,
    pub tot_len: __u16,
    pub frag_off: __u16,
    pub saddr: __be32,
    pub daddr: __be32,
}

#[repr(C)]
pub struct icmp6hdr {
    pub icmp6_type: __u8,
}

#[repr(C)]
pub struct icmphdr {
    pub type_: __u8,
    pub code: __u8,
}

#[repr(C)]
pub struct udphdr {
    pub source: __u16,
    pub dest: __u16,
}

#[repr(C)]
pub struct tcphdr {
    pub source: __u16,
    pub dest: __u16,
    pub syn: __u16,
}

#[inline(always)]
unsafe fn rol32(word: __u32, shift: ::core::ffi::c_uint) -> __u32 {
    word.wrapping_shl(shift) | word.wrapping_shr((0u32.wrapping_sub(shift)) & 31)
}

/* copy paste of jhash from kernel sources to make sure llvm
 * can compile it into valid sequence of bpf instructions
 */
#[inline(always)]
unsafe fn __jhash_mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a = a.wrapping_sub(*c); *a ^= rol32(*c, 4);  *c = c.wrapping_add(*b);
    *b = b.wrapping_sub(*a); *b ^= rol32(*a, 6);  *a = a.wrapping_add(*c);
    *c = c.wrapping_sub(*b); *c ^= rol32(*b, 8);  *b = b.wrapping_add(*a);
    *a = a.wrapping_sub(*c); *a ^= rol32(*c, 16); *c = c.wrapping_add(*b);
    *b = b.wrapping_sub(*a); *b ^= rol32(*a, 19); *a = a.wrapping_add(*c);
    *c = c.wrapping_sub(*b); *c ^= rol32(*b, 4);  *b = b.wrapping_add(*a);
}

#[inline(always)]
unsafe fn __jhash_final(a: &mut u32, b: &mut u32, c: &mut u32) {
    *c ^= *b; *c = c.wrapping_sub(rol32(*b, 14));
    *a ^= *c; *a = a.wrapping_sub(rol32(*c, 11));
    *b ^= *a; *b = b.wrapping_sub(rol32(*a, 25));
    *c ^= *b; *c = c.wrapping_sub(rol32(*b, 16));
    *a ^= *c; *a = a.wrapping_sub(rol32(*c, 4));
    *b ^= *a; *b = b.wrapping_sub(rol32(*a, 14));
    *c ^= *b; *c = c.wrapping_sub(rol32(*b, 24));
}

#[inline(never)]
unsafe fn jhash(key: *const ::core::ffi::c_void, mut length: u32, initval: u32) -> u32 {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut k = key as *const u8;

    c = JHASH_INITVAL.wrapping_add(length).wrapping_add(initval);
    b = c;
    a = b;

    while length > 12 {
        a = a.wrapping_add(*(k as *const u32));
        b = b.wrapping_add(*(k.add(4) as *const u32));
        c = c.wrapping_add(*(k.add(8) as *const u32));
        __jhash_mix(&mut a, &mut b, &mut c);
        length = length.wrapping_sub(12);
        k = k.add(12);
    }
    match length {
        12 => { c = c.wrapping_add(((*k.add(11) as u32) << 24)); c = c.wrapping_add(((*k.add(10) as u32) << 16)); c = c.wrapping_add(((*k.add(9) as u32) << 8)); c = c.wrapping_add(*k.add(8) as u32); b = b.wrapping_add(((*k.add(7) as u32) << 24)); b = b.wrapping_add(((*k.add(6) as u32) << 16)); b = b.wrapping_add(((*k.add(5) as u32) << 8)); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3) as u32) << 24)); a = a.wrapping_add(((*k.add(2) as u32) << 16)); a = a.wrapping_add(((*k.add(1) as u32) << 8)); a = a.wrapping_add(*k.add(0) as u32); __jhash_final(&mut a, &mut b, &mut c); }
        11 => { c = c.wrapping_add(((*k.add(10) as u32) << 16)); c = c.wrapping_add(((*k.add(9) as u32) << 8)); c = c.wrapping_add(*k.add(8) as u32); b = b.wrapping_add(((*k.add(7) as u32) << 24)); b = b.wrapping_add(((*k.add(6) as u32) << 16)); b = b.wrapping_add(((*k.add(5) as u32) << 8)); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3) as u32) << 24)); a = a.wrapping_add(((*k.add(2) as u32) << 16)); a = a.wrapping_add(((*k.add(1) as u32) << 8)); a = a.wrapping_add(*k.add(0) as u32); __jhash_final(&mut a, &mut b, &mut c); }
        10 => { c = c.wrapping_add(((*k.add(9) as u32) << 8)); c = c.wrapping_add(*k.add(8) as u32); b = b.wrapping_add(((*k.add(7) as u32) << 24)); b = b.wrapping_add(((*k.add(6) as u32) << 16)); b = b.wrapping_add(((*k.add(5) as u32) << 8)); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3) as u32) << 24)); a = a.wrapping_add(((*k.add(2) as u32) << 16)); a = a.wrapping_add(((*k.add(1) as u32) << 8)); a = a.wrapping_add(*k.add(0) as u32); __jhash_final(&mut a, &mut b, &mut c); }
        9 => { c = c.wrapping_add(*k.add(8) as u32); b = b.wrapping_add(((*k.add(7) as u32) << 24)); b = b.wrapping_add(((*k.add(6) as u32) << 16)); b = b.wrapping_add(((*k.add(5) as u32) << 8)); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3) as u32) << 24)); a = a.wrapping_add(((*k.add(2) as u32) << 16)); a = a.wrapping_add(((*k.add(1) as u32) << 8)); a = a.wrapping_add(*k.add(0) as u32); __jhash_final(&mut a, &mut b, &mut c); }
        8 => { b = b.wrapping_add(((*k.add(7) as u32) << 24)); b = b.wrapping_add(((*k.add(6) as u32) << 16)); b = b.wrapping_add(((*k.add(5) as u32) << 8)); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3) as u32) << 24)); a = a.wrapping_add(((*k.add(2) as u32) << 16)); a = a.wrapping_add(((*k.add(1) as u32) << 8)); a = a.wrapping_add(*k.add(0) as u32); __jhash_final(&mut a, &mut b, &mut c); }
        7 => { b = b.wrapping_add(((*k.add(6) as u32) << 16)); b = b.wrapping_add(((*k.add(5) as u32) << 8)); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3) as u32) << 24)); a = a.wrapping_add(((*k.add(2) as u32) << 16)); a = a.wrapping_add(((*k.add(1) as u32) << 8)); a = a.wrapping_add(*k.add(0) as u32); __jhash_final(&mut a, &mut b, &mut c); }
        6 => { b = b.wrapping_add(((*k.add(5) as u32) << 8)); b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3) as u32) << 24)); a = a.wrapping_add(((*k.add(2) as u32) << 16)); a = a.wrapping_add(((*k.add(1) as u32) << 8)); a = a.wrapping_add(*k.add(0) as u32); __jhash_final(&mut a, &mut b, &mut c); }
        5 => { b = b.wrapping_add(*k.add(4) as u32); a = a.wrapping_add(((*k.add(3) as u32) << 24)); a = a.wrapping_add(((*k.add(2) as u32) << 16)); a = a.wrapping_add(((*k.add(1) as u32) << 8)); a = a.wrapping_add(*k.add(0) as u32); __jhash_final(&mut a, &mut b, &mut c); }
        4 => { a = a.wrapping_add(((*k.add(3) as u32) << 24)); a = a.wrapping_add(((*k.add(2) as u32) << 16)); a = a.wrapping_add(((*k.add(1) as u32) << 8)); a = a.wrapping_add(*k.add(0) as u32); __jhash_final(&mut a, &mut b, &mut c); }
        3 => { a = a.wrapping_add(((*k.add(2) as u32) << 16)); a = a.wrapping_add(((*k.add(1) as u32) << 8)); a = a.wrapping_add(*k.add(0) as u32); __jhash_final(&mut a, &mut b, &mut c); }
        2 => { a = a.wrapping_add(((*k.add(1) as u32) << 8)); a = a.wrapping_add(*k.add(0) as u32); __jhash_final(&mut a, &mut b, &mut c); }
        1 => { a = a.wrapping_add(*k.add(0) as u32); __jhash_final(&mut a, &mut b, &mut c); }
        0 => {}
        _ => {}
    }

    c
}

#[inline(never)]
unsafe fn __jhash_nwords(mut a: u32, mut b: u32, mut c: u32, initval: u32) -> u32 {
    a = a.wrapping_add(initval);
    b = b.wrapping_add(initval);
    c = c.wrapping_add(initval);
    __jhash_final(&mut a, &mut b, &mut c);
    c
}

#[inline(never)]
unsafe fn jhash_2words(a: u32, b: u32, initval: u32) -> u32 {
    __jhash_nwords(a, b, 0, initval.wrapping_add(JHASH_INITVAL).wrapping_add(2 << 2))
}

#[inline(never)]
unsafe fn get_packet_hash(pckt: *mut packet_description, ipv6: bool) -> __u32 {
    if ipv6 {
        jhash_2words(
            jhash((*pckt).src_u.srcv6.as_ptr() as *const ::core::ffi::c_void, 16, MAX_VIPS),
            (*pckt).ports_u.ports,
            CH_RINGS_SIZE,
        )
    } else {
        jhash_2words((*pckt).src_u.src, (*pckt).ports_u.ports, CH_RINGS_SIZE)
    }
}

#[inline(never)]
unsafe fn get_packet_dst(
    real: *mut *mut real_definition,
    pckt: *mut packet_description,
    vip_info: *mut vip_meta,
    is_ipv6: bool,
) -> bool {
    let hash: __u32 = get_packet_hash(pckt, is_ipv6);
    let mut key: __u32 = RING_SIZE.wrapping_mul((*vip_info).vip_num).wrapping_add(hash % RING_SIZE);
    let mut real_pos: *mut __u32;

    if hash != 0x358459b7 && hash != 0x2f4bc6bb {
        return false;
    }

    real_pos = bpf_map_lookup_elem(
        &raw mut ch_rings,
        &key as *const _ as *const ::core::ffi::c_void,
    ) as *mut __u32;
    if real_pos.is_null() {
        return false;
    }
    key = *real_pos;
    *real = bpf_map_lookup_elem(
        &raw mut reals,
        &key as *const _ as *const ::core::ffi::c_void,
    ) as *mut real_definition;
    if (*real).is_null() {
        return false;
    }
    true
}

#[inline(never)]
unsafe fn parse_icmpv6(skb_ptr: *mut bpf_dynptr, mut off: __u64, pckt: *mut packet_description) -> i32 {
    let mut buffer: [__u8; ::core::mem::size_of::<ipv6hdr>()] = [0; ::core::mem::size_of::<ipv6hdr>()];
    let mut icmp_hdr: *mut icmp6hdr;
    let mut ip6h: *mut ipv6hdr;

    icmp_hdr = bpf_dynptr_slice(skb_ptr, off, buffer.as_mut_ptr() as *mut _, buffer.len() as __u32) as *mut icmp6hdr;
    if icmp_hdr.is_null() {
        return TC_ACT_SHOT;
    }

    if (*icmp_hdr).icmp6_type != ICMPV6_PKT_TOOBIG {
        return TC_ACT_OK;
    }
    off = off.wrapping_add(::core::mem::size_of::<icmp6hdr>() as __u64);
    ip6h = bpf_dynptr_slice(skb_ptr, off, buffer.as_mut_ptr() as *mut _, buffer.len() as __u32) as *mut ipv6hdr;
    if ip6h.is_null() {
        return TC_ACT_SHOT;
    }
    (*pckt).proto = (*ip6h).nexthdr;
    (*pckt).flags |= F_ICMP;
    ::core::ptr::copy_nonoverlapping((*ip6h).daddr.s6_addr32.as_ptr(), (*pckt).src_u.srcv6.as_mut_ptr(), 4);
    ::core::ptr::copy_nonoverlapping((*ip6h).saddr.s6_addr32.as_ptr(), (*pckt).dst_u.dstv6.as_mut_ptr(), 4);
    TC_ACT_UNSPEC
}

#[inline(never)]
unsafe fn parse_icmp(skb_ptr: *mut bpf_dynptr, mut off: __u64, pckt: *mut packet_description) -> i32 {
    let mut buffer_icmp: [__u8; ::core::mem::size_of::<iphdr>()] = [0; ::core::mem::size_of::<iphdr>()];
    let mut buffer_ip: [__u8; ::core::mem::size_of::<iphdr>()] = [0; ::core::mem::size_of::<iphdr>()];
    let mut icmp_hdr: *mut icmphdr;
    let mut iph: *mut iphdr;

    icmp_hdr = bpf_dynptr_slice(skb_ptr, off, buffer_icmp.as_mut_ptr() as *mut _, buffer_icmp.len() as __u32) as *mut icmphdr;
    if icmp_hdr.is_null() {
        return TC_ACT_SHOT;
    }
    if (*icmp_hdr).type_ != ICMP_DEST_UNREACH || (*icmp_hdr).code != ICMP_FRAG_NEEDED {
        return TC_ACT_OK;
    }
    off = off.wrapping_add(::core::mem::size_of::<icmphdr>() as __u64);
    iph = bpf_dynptr_slice(skb_ptr, off, buffer_ip.as_mut_ptr() as *mut _, buffer_ip.len() as __u32) as *mut iphdr;
    if iph.is_null() || (*iph).ihl != 5 {
        return TC_ACT_SHOT;
    }
    (*pckt).proto = (*iph).protocol;
    (*pckt).flags |= F_ICMP;
    (*pckt).src_u.src = (*iph).daddr;
    (*pckt).dst_u.dst = (*iph).saddr;
    TC_ACT_UNSPEC
}

#[inline(never)]
unsafe fn parse_udp(skb_ptr: *mut bpf_dynptr, off: __u64, pckt: *mut packet_description) -> bool {
    let mut buffer: [__u8; ::core::mem::size_of::<udphdr>()] = [0; ::core::mem::size_of::<udphdr>()];
    let mut udp: *mut udphdr;

    udp = bpf_dynptr_slice(skb_ptr, off, buffer.as_mut_ptr() as *mut _, buffer.len() as __u32) as *mut udphdr;
    if udp.is_null() {
        return false;
    }

    if ((*pckt).flags & F_ICMP) == 0 {
        (*pckt).ports_u.port16[0] = (*udp).source;
        (*pckt).ports_u.port16[1] = (*udp).dest;
    } else {
        (*pckt).ports_u.port16[0] = (*udp).dest;
        (*pckt).ports_u.port16[1] = (*udp).source;
    }
    true
}

#[inline(never)]
unsafe fn parse_tcp(skb_ptr: *mut bpf_dynptr, off: __u64, pckt: *mut packet_description) -> bool {
    let mut buffer: [__u8; ::core::mem::size_of::<tcphdr>()] = [0; ::core::mem::size_of::<tcphdr>()];
    let mut tcp: *mut tcphdr;

    tcp = bpf_dynptr_slice(skb_ptr, off, buffer.as_mut_ptr() as *mut _, buffer.len() as __u32) as *mut tcphdr;
    if tcp.is_null() {
        return false;
    }

    if (*tcp).syn != 0 {
        (*pckt).flags |= F_SYN_SET;
    }

    if ((*pckt).flags & F_ICMP) == 0 {
        (*pckt).ports_u.port16[0] = (*tcp).source;
        (*pckt).ports_u.port16[1] = (*tcp).dest;
    } else {
        (*pckt).ports_u.port16[0] = (*tcp).dest;
        (*pckt).ports_u.port16[1] = (*tcp).source;
    }
    true
}

#[inline(never)]
unsafe fn process_packet(
    skb_ptr: *mut bpf_dynptr,
    eth: *mut eth_hdr,
    mut off: __u64,
    is_ipv6: bool,
    skb: *mut __sk_buff,
) -> i32 {
    let mut pckt: packet_description = ::core::mem::zeroed();
    let mut tkey: bpf_tunnel_key = ::core::mem::zeroed();
    let mut data_stats: *mut vip_stats;
    let mut dst: *mut real_definition = ::core::ptr::null_mut();
    let mut vip_info: *mut vip_meta;
    let mut cval: *mut ctl_value;
    let mut v4_intf_pos: __u32 = 1;
    let mut v6_intf_pos: __u32 = 2;
    let mut ip6h: *mut ipv6hdr;
    let mut vip: vip = ::core::mem::zeroed();
    let mut iph: *mut iphdr;
    let mut tun_flag: i32 = 0;
    let pkt_bytes: __u16;
    let iph_len: __u64;
    let ifindex: __u32;
    let mut protocol: __u8;
    let vip_num: __u32;
    let mut action: i32;

    tkey.tunnel_ttl = 64;
    if is_ipv6 {
        let mut buffer: [__u8; ::core::mem::size_of::<ipv6hdr>()] = [0; ::core::mem::size_of::<ipv6hdr>()];

        ip6h = bpf_dynptr_slice(skb_ptr, off, buffer.as_mut_ptr() as *mut _, buffer.len() as __u32) as *mut ipv6hdr;
        if ip6h.is_null() {
            return TC_ACT_SHOT;
        }

        iph_len = ::core::mem::size_of::<ipv6hdr>() as __u64;
        protocol = (*ip6h).nexthdr;
        pckt.proto = protocol;
        pkt_bytes = bpf_ntohs((*ip6h).payload_len);
        off = off.wrapping_add(iph_len);
        if protocol == IPPROTO_FRAGMENT {
            return TC_ACT_SHOT;
        } else if protocol == IPPROTO_ICMPV6 {
            action = parse_icmpv6(skb_ptr, off, &mut pckt);
            if action >= 0 {
                return action;
            }
            off = off.wrapping_add(IPV6_PLUS_ICMP_HDR);
        } else {
            ::core::ptr::copy_nonoverlapping((*ip6h).saddr.s6_addr32.as_ptr(), pckt.src_u.srcv6.as_mut_ptr(), 4);
            ::core::ptr::copy_nonoverlapping((*ip6h).daddr.s6_addr32.as_ptr(), pckt.dst_u.dstv6.as_mut_ptr(), 4);
        }
    } else {
        let mut buffer: [__u8; ::core::mem::size_of::<iphdr>()] = [0; ::core::mem::size_of::<iphdr>()];

        iph = bpf_dynptr_slice(skb_ptr, off, buffer.as_mut_ptr() as *mut _, buffer.len() as __u32) as *mut iphdr;
        if iph.is_null() || (*iph).ihl != 5 {
            return TC_ACT_SHOT;
        }

        protocol = (*iph).protocol;
        pckt.proto = protocol;
        pkt_bytes = bpf_ntohs((*iph).tot_len);
        off = off.wrapping_add(IPV4_HDR_LEN_NO_OPT);

        if ((*iph).frag_off as __u32 & PCKT_FRAGMENTED) != 0 {
            return TC_ACT_SHOT;
        }
        if protocol == IPPROTO_ICMP {
            action = parse_icmp(skb_ptr, off, &mut pckt);
            if action >= 0 {
                return action;
            }
            off = off.wrapping_add(IPV4_PLUS_ICMP_HDR);
        } else {
            pckt.src_u.src = (*iph).saddr;
            pckt.dst_u.dst = (*iph).daddr;
        }
    }
    protocol = pckt.proto;

    if protocol == IPPROTO_TCP {
        if !parse_tcp(skb_ptr, off, &mut pckt) {
            return TC_ACT_SHOT;
        }
    } else if protocol == IPPROTO_UDP {
        if !parse_udp(skb_ptr, off, &mut pckt) {
            return TC_ACT_SHOT;
        }
    } else {
        return TC_ACT_SHOT;
    }

    if is_ipv6 {
        ::core::ptr::copy_nonoverlapping(pckt.dst_u.dstv6.as_ptr(), vip.daddr.v6.as_mut_ptr(), 4);
    } else {
        vip.daddr.v4 = pckt.dst_u.dst;
    }

    vip.dport = pckt.ports_u.port16[1];
    vip.protocol = pckt.proto;
    vip_info = bpf_map_lookup_elem(
        &raw mut vip_map,
        &vip as *const _ as *const ::core::ffi::c_void,
    ) as *mut vip_meta;
    if vip_info.is_null() {
        vip.dport = 0;
        vip_info = bpf_map_lookup_elem(
            &raw mut vip_map,
            &vip as *const _ as *const ::core::ffi::c_void,
        ) as *mut vip_meta;
        if vip_info.is_null() {
            return TC_ACT_SHOT;
        }
        pckt.ports_u.port16[1] = 0;
    }

    if ((*vip_info).flags & F_HASH_NO_SRC_PORT) != 0 {
        pckt.ports_u.port16[0] = 0;
    }

    if !get_packet_dst(&mut dst, &mut pckt, vip_info, is_ipv6) {
        return TC_ACT_SHOT;
    }

    if ((*dst).flags as __u32 & F_IPV6) != 0 {
        cval = bpf_map_lookup_elem(
            &raw mut ctl_array,
            &mut v6_intf_pos as *mut _ as *const ::core::ffi::c_void,
        ) as *mut ctl_value;
        if cval.is_null() {
            return TC_ACT_SHOT;
        }
        ifindex = (*cval).u.ifindex;
        ::core::ptr::copy_nonoverlapping((*dst).dst_u.dstv6.as_ptr(), tkey.remote_ipv6.as_mut_ptr(), 4);
        tun_flag = BPF_F_TUNINFO_IPV6;
    } else {
        cval = bpf_map_lookup_elem(
            &raw mut ctl_array,
            &mut v4_intf_pos as *mut _ as *const ::core::ffi::c_void,
        ) as *mut ctl_value;
        if cval.is_null() {
            return TC_ACT_SHOT;
        }
        ifindex = (*cval).u.ifindex;
        tkey.remote_ipv4 = (*dst).dst_u.dst;
    }
    vip_num = (*vip_info).vip_num;
    data_stats = bpf_map_lookup_elem(
        &raw mut stats,
        &vip_num as *const _ as *const ::core::ffi::c_void,
    ) as *mut vip_stats;
    if data_stats.is_null() {
        return TC_ACT_SHOT;
    }
    (*data_stats).pkts = (*data_stats).pkts.wrapping_add(1);
    (*data_stats).bytes = (*data_stats).bytes.wrapping_add(pkt_bytes as __u64);
    bpf_skb_set_tunnel_key(skb, &mut tkey, ::core::mem::size_of::<bpf_tunnel_key>() as __u32, tun_flag);
    *(eth as *mut u32) = tkey.remote_ipv4;
    bpf_redirect(ifindex, 0)
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn balancer_ingress(ctx: *mut __sk_buff) -> i32 {
    let mut buffer: [__u8; ::core::mem::size_of::<eth_hdr>()] = [0; ::core::mem::size_of::<eth_hdr>()];
    let mut ptr: bpf_dynptr = ::core::mem::zeroed();
    let mut eth: *mut eth_hdr;
    let mut eth_proto: __u32;
    let nh_off: __u32;
    let err: i32;

    nh_off = ::core::mem::size_of::<eth_hdr>() as __u32;

    bpf_dynptr_from_skb(ctx, 0, &mut ptr);
    eth = bpf_dynptr_slice_rdwr(&mut ptr, 0, buffer.as_mut_ptr() as *mut _, buffer.len() as __u32) as *mut eth_hdr;
    if eth.is_null() {
        return TC_ACT_SHOT;
    }
    eth_proto = (*eth).eth_proto as __u32;
    if eth_proto == bpf_htons(ETH_P_IP) as __u32 {
        err = process_packet(&mut ptr, eth, nh_off as __u64, false, ctx);
    } else if eth_proto == bpf_htons(ETH_P_IPV6) as __u32 {
        err = process_packet(&mut ptr, eth, nh_off as __u64, true, ctx);
    } else {
        return TC_ACT_SHOT;
    }

    if eth == buffer.as_mut_ptr() as *mut eth_hdr {
        bpf_dynptr_write(&mut ptr, 0, buffer.as_ptr() as *const _, buffer.len() as __u32, 0);
    }

    err
}

// SEC("license")
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
