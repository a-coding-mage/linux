// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017 Facebook
//
// Rust translation of test_l4lb_noinline.c.
// External Linux/BPF types, constants, helpers, map-definition macros, and
// section attributes are dependencies corresponding to the original includes:
// <linux/pkt_cls.h>, <linux/bpf.h>, <linux/in.h>, <linux/if_ether.h>,
// <linux/ip.h>, <linux/ipv6.h>, <linux/icmp.h>, <linux/icmpv6.h>,
// <linux/tcp.h>, <linux/udp.h>, <bpf/bpf_helpers.h>,
// "test_iptunnel_common.h", and <bpf/bpf_endian.h>.

type u32 = __u32;

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
const F_IPV6: __u8 = 1 << 0;
const F_HASH_NO_SRC_PORT: __u32 = 1 << 0;
const F_ICMP: __u8 = 1 << 0;
const F_SYN_SET: __u8 = 1 << 1;

#[inline(always)]
unsafe fn rol32(word: __u32, shift: u32) -> __u32 {
    word.wrapping_shl(shift) | word.wrapping_shr((0u32.wrapping_sub(shift)) & 31)
}

/* copy paste of jhash from kernel sources to make sure llvm
 * can compile it into valid sequence of bpf instructions
 */
macro_rules! __jhash_mix {
    ($a:ident, $b:ident, $c:ident) => {{
        $a = $a.wrapping_sub($c);
        $a ^= rol32($c, 4);
        $c = $c.wrapping_add($b);
        $b = $b.wrapping_sub($a);
        $b ^= rol32($a, 6);
        $a = $a.wrapping_add($c);
        $c = $c.wrapping_sub($b);
        $c ^= rol32($b, 8);
        $b = $b.wrapping_add($a);
        $a = $a.wrapping_sub($c);
        $a ^= rol32($c, 16);
        $c = $c.wrapping_add($b);
        $b = $b.wrapping_sub($a);
        $b ^= rol32($a, 19);
        $a = $a.wrapping_add($c);
        $c = $c.wrapping_sub($b);
        $c ^= rol32($b, 4);
        $b = $b.wrapping_add($a);
    }};
}

macro_rules! __jhash_final {
    ($a:ident, $b:ident, $c:ident) => {{
        $c ^= $b;
        $c = $c.wrapping_sub(rol32($b, 14));
        $a ^= $c;
        $a = $a.wrapping_sub(rol32($c, 11));
        $b ^= $a;
        $b = $b.wrapping_sub(rol32($a, 25));
        $c ^= $b;
        $c = $c.wrapping_sub(rol32($b, 16));
        $a ^= $c;
        $a = $a.wrapping_sub(rol32($c, 4));
        $b ^= $a;
        $b = $b.wrapping_sub(rol32($a, 14));
        $c ^= $b;
        $c = $c.wrapping_sub(rol32($b, 24));
    }};
}

unsafe fn jhash(key: *const core::ffi::c_void, mut length: u32, initval: u32) -> u32 {
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
        __jhash_mix!(a, b, c);
        length = length.wrapping_sub(12);
        k = k.add(12);
    }

    match length {
        12 => {
            c = c.wrapping_add((*(k.add(11)) as u32) << 24);
            c = c.wrapping_add((*(k.add(10)) as u32) << 16);
            c = c.wrapping_add((*(k.add(9)) as u32) << 8);
            c = c.wrapping_add(*(k.add(8)) as u32);
            b = b.wrapping_add((*(k.add(7)) as u32) << 24);
            b = b.wrapping_add((*(k.add(6)) as u32) << 16);
            b = b.wrapping_add((*(k.add(5)) as u32) << 8);
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32) << 24);
            a = a.wrapping_add((*(k.add(2)) as u32) << 16);
            a = a.wrapping_add((*(k.add(1)) as u32) << 8);
            a = a.wrapping_add(*(k.add(0)) as u32);
            __jhash_final!(a, b, c);
        }
        11 => {
            c = c.wrapping_add((*(k.add(10)) as u32) << 16);
            c = c.wrapping_add((*(k.add(9)) as u32) << 8);
            c = c.wrapping_add(*(k.add(8)) as u32);
            b = b.wrapping_add((*(k.add(7)) as u32) << 24);
            b = b.wrapping_add((*(k.add(6)) as u32) << 16);
            b = b.wrapping_add((*(k.add(5)) as u32) << 8);
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32) << 24);
            a = a.wrapping_add((*(k.add(2)) as u32) << 16);
            a = a.wrapping_add((*(k.add(1)) as u32) << 8);
            a = a.wrapping_add(*(k.add(0)) as u32);
            __jhash_final!(a, b, c);
        }
        10 => {
            c = c.wrapping_add((*(k.add(9)) as u32) << 8);
            c = c.wrapping_add(*(k.add(8)) as u32);
            b = b.wrapping_add((*(k.add(7)) as u32) << 24);
            b = b.wrapping_add((*(k.add(6)) as u32) << 16);
            b = b.wrapping_add((*(k.add(5)) as u32) << 8);
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32) << 24);
            a = a.wrapping_add((*(k.add(2)) as u32) << 16);
            a = a.wrapping_add((*(k.add(1)) as u32) << 8);
            a = a.wrapping_add(*(k.add(0)) as u32);
            __jhash_final!(a, b, c);
        }
        9 => {
            c = c.wrapping_add(*(k.add(8)) as u32);
            b = b.wrapping_add((*(k.add(7)) as u32) << 24);
            b = b.wrapping_add((*(k.add(6)) as u32) << 16);
            b = b.wrapping_add((*(k.add(5)) as u32) << 8);
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32) << 24);
            a = a.wrapping_add((*(k.add(2)) as u32) << 16);
            a = a.wrapping_add((*(k.add(1)) as u32) << 8);
            a = a.wrapping_add(*(k.add(0)) as u32);
            __jhash_final!(a, b, c);
        }
        8 => {
            b = b.wrapping_add((*(k.add(7)) as u32) << 24);
            b = b.wrapping_add((*(k.add(6)) as u32) << 16);
            b = b.wrapping_add((*(k.add(5)) as u32) << 8);
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32) << 24);
            a = a.wrapping_add((*(k.add(2)) as u32) << 16);
            a = a.wrapping_add((*(k.add(1)) as u32) << 8);
            a = a.wrapping_add(*(k.add(0)) as u32);
            __jhash_final!(a, b, c);
        }
        7 => {
            b = b.wrapping_add((*(k.add(6)) as u32) << 16);
            b = b.wrapping_add((*(k.add(5)) as u32) << 8);
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32) << 24);
            a = a.wrapping_add((*(k.add(2)) as u32) << 16);
            a = a.wrapping_add((*(k.add(1)) as u32) << 8);
            a = a.wrapping_add(*(k.add(0)) as u32);
            __jhash_final!(a, b, c);
        }
        6 => {
            b = b.wrapping_add((*(k.add(5)) as u32) << 8);
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32) << 24);
            a = a.wrapping_add((*(k.add(2)) as u32) << 16);
            a = a.wrapping_add((*(k.add(1)) as u32) << 8);
            a = a.wrapping_add(*(k.add(0)) as u32);
            __jhash_final!(a, b, c);
        }
        5 => {
            b = b.wrapping_add(*(k.add(4)) as u32);
            a = a.wrapping_add((*(k.add(3)) as u32) << 24);
            a = a.wrapping_add((*(k.add(2)) as u32) << 16);
            a = a.wrapping_add((*(k.add(1)) as u32) << 8);
            a = a.wrapping_add(*(k.add(0)) as u32);
            __jhash_final!(a, b, c);
        }
        4 => {
            a = a.wrapping_add((*(k.add(3)) as u32) << 24);
            a = a.wrapping_add((*(k.add(2)) as u32) << 16);
            a = a.wrapping_add((*(k.add(1)) as u32) << 8);
            a = a.wrapping_add(*(k.add(0)) as u32);
            __jhash_final!(a, b, c);
        }
        3 => {
            a = a.wrapping_add((*(k.add(2)) as u32) << 16);
            a = a.wrapping_add((*(k.add(1)) as u32) << 8);
            a = a.wrapping_add(*(k.add(0)) as u32);
            __jhash_final!(a, b, c);
        }
        2 => {
            a = a.wrapping_add((*(k.add(1)) as u32) << 8);
            a = a.wrapping_add(*(k.add(0)) as u32);
            __jhash_final!(a, b, c);
        }
        1 => {
            a = a.wrapping_add(*(k.add(0)) as u32);
            __jhash_final!(a, b, c);
        }
        0 => {
            /* Nothing left to add */
        }
        _ => {}
    }

    c
}

unsafe fn __jhash_nwords(mut a: u32, mut b: u32, mut c: u32, initval: u32) -> u32 {
    a = a.wrapping_add(initval);
    b = b.wrapping_add(initval);
    c = c.wrapping_add(initval);
    __jhash_final!(a, b, c);
    c
}

unsafe fn jhash_2words(a: u32, b: u32, initval: u32) -> u32 {
    __jhash_nwords(
        a,
        b,
        0,
        initval.wrapping_add(JHASH_INITVAL).wrapping_add(2 << 2),
    )
}

#[repr(C)]
union packet_description_src {
    src: __be32,
    srcv6: [__be32; 4],
}

#[repr(C)]
union packet_description_dst {
    dst: __be32,
    dstv6: [__be32; 4],
}

#[repr(C)]
union packet_description_ports {
    ports: __u32,
    port16: [__u16; 2],
}

#[repr(C)]
struct packet_description {
    src_u: packet_description_src,
    dst_u: packet_description_dst,
    ports_u: packet_description_ports,
    proto: __u8,
    flags: __u8,
}

#[repr(C)]
union ctl_value {
    value: __u64,
    ifindex: __u32,
    mac: [__u8; 6],
}

#[repr(C)]
struct vip_meta {
    flags: __u32,
    vip_num: __u32,
}

#[repr(C)]
union real_definition_dst {
    dst: __be32,
    dstv6: [__be32; 4],
}

#[repr(C)]
struct real_definition {
    dst_u: real_definition_dst,
    flags: __u8,
}

#[repr(C)]
struct vip_stats {
    bytes: __u64,
    pkts: __u64,
}

#[repr(C)]
struct eth_hdr {
    eth_dest: [core::ffi::c_uchar; ETH_ALEN],
    eth_source: [core::ffi::c_uchar; ETH_ALEN],
    eth_proto: core::ffi::c_ushort,
}

// Original BPF map definitions:
// vip_map: BPF_MAP_TYPE_HASH, max_entries MAX_VIPS, key struct vip, value struct vip_meta
// ch_rings: BPF_MAP_TYPE_ARRAY, max_entries CH_RINGS_SIZE, key __u32, value __u32
// reals: BPF_MAP_TYPE_ARRAY, max_entries MAX_REALS, key __u32, value struct real_definition
// stats: BPF_MAP_TYPE_PERCPU_ARRAY, max_entries MAX_VIPS, key __u32, value struct vip_stats
// ctl_array: BPF_MAP_TYPE_ARRAY, max_entries CTL_MAP_SIZE, key __u32, value struct ctl_value
extern "C" {
    static mut vip_map: core::ffi::c_void;
    static mut ch_rings: core::ffi::c_void;
    static mut reals: core::ffi::c_void;
    static mut stats: core::ffi::c_void;
    static mut ctl_array: core::ffi::c_void;
}

unsafe fn get_packet_hash(pckt: *mut packet_description, ipv6: bool) -> __u32 {
    if ipv6 {
        jhash_2words(
            jhash((*pckt).src_u.srcv6.as_ptr() as *const core::ffi::c_void, 16, MAX_VIPS),
            (*pckt).ports_u.ports,
            CH_RINGS_SIZE,
        )
    } else {
        jhash_2words((*pckt).src_u.src, (*pckt).ports_u.ports, CH_RINGS_SIZE)
    }
}

unsafe fn get_packet_dst(
    real: *mut *mut real_definition,
    pckt: *mut packet_description,
    vip_info: *mut vip_meta,
    is_ipv6: bool,
) -> bool {
    let hash: __u32 = get_packet_hash(pckt, is_ipv6);
    let mut key: __u32 = RING_SIZE
        .wrapping_mul((*vip_info).vip_num)
        .wrapping_add(hash % RING_SIZE);
    let mut real_pos: *mut __u32;

    if hash != 0x358459b7 /* jhash of ipv4 packet */
        && hash != 0x2f4bc6bb
    /* jhash of ipv6 packet */
    {
        return false;
    }

    real_pos = bpf_map_lookup_elem(&mut ch_rings as *mut _ as *mut _, &mut key as *mut _ as *mut _)
        as *mut __u32;
    if real_pos.is_null() {
        return false;
    }
    key = *real_pos;
    *real = bpf_map_lookup_elem(&mut reals as *mut _ as *mut _, &mut key as *mut _ as *mut _)
        as *mut real_definition;
    if (*real).is_null() {
        return false;
    }
    true
}

unsafe fn parse_icmpv6(
    data: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
    mut off: __u64,
    pckt: *mut packet_description,
) -> core::ffi::c_int {
    let mut icmp_hdr: *mut icmp6hdr;
    let mut ip6h: *mut ipv6hdr;

    icmp_hdr = (data as *mut u8).add(off as usize) as *mut icmp6hdr;
    if icmp_hdr.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_SHOT;
    }
    if (*icmp_hdr).icmp6_type != ICMPV6_PKT_TOOBIG {
        return TC_ACT_OK;
    }
    off = off.wrapping_add(core::mem::size_of::<icmp6hdr>() as __u64);
    ip6h = (data as *mut u8).add(off as usize) as *mut ipv6hdr;
    if ip6h.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_SHOT;
    }
    (*pckt).proto = (*ip6h).nexthdr;
    (*pckt).flags |= F_ICMP;
    core::ptr::copy_nonoverlapping(
        (*ip6h).daddr.s6_addr32.as_ptr(),
        (*pckt).src_u.srcv6.as_mut_ptr(),
        4,
    );
    core::ptr::copy_nonoverlapping(
        (*ip6h).saddr.s6_addr32.as_ptr(),
        (*pckt).dst_u.dstv6.as_mut_ptr(),
        4,
    );
    TC_ACT_UNSPEC
}

unsafe fn parse_icmp(
    data: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
    mut off: __u64,
    pckt: *mut packet_description,
) -> core::ffi::c_int {
    let mut icmp_hdr: *mut icmphdr;
    let mut iph: *mut iphdr;

    icmp_hdr = (data as *mut u8).add(off as usize) as *mut icmphdr;
    if icmp_hdr.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_SHOT;
    }
    if (*icmp_hdr).type_ != ICMP_DEST_UNREACH || (*icmp_hdr).code != ICMP_FRAG_NEEDED {
        return TC_ACT_OK;
    }
    off = off.wrapping_add(core::mem::size_of::<icmphdr>() as __u64);
    iph = (data as *mut u8).add(off as usize) as *mut iphdr;
    if iph.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_SHOT;
    }
    if (*iph).ihl() != 5 {
        return TC_ACT_SHOT;
    }
    (*pckt).proto = (*iph).protocol;
    (*pckt).flags |= F_ICMP;
    (*pckt).src_u.src = (*iph).daddr;
    (*pckt).dst_u.dst = (*iph).saddr;
    TC_ACT_UNSPEC
}

unsafe fn parse_udp(
    data: *mut core::ffi::c_void,
    off: __u64,
    data_end: *mut core::ffi::c_void,
    pckt: *mut packet_description,
) -> bool {
    let mut udp: *mut udphdr;
    udp = (data as *mut u8).add(off as usize) as *mut udphdr;

    if udp.add(1) as *mut core::ffi::c_void > data_end {
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

unsafe fn parse_tcp(
    data: *mut core::ffi::c_void,
    off: __u64,
    data_end: *mut core::ffi::c_void,
    pckt: *mut packet_description,
) -> bool {
    let mut tcp: *mut tcphdr;

    tcp = (data as *mut u8).add(off as usize) as *mut tcphdr;
    if tcp.add(1) as *mut core::ffi::c_void > data_end {
        return false;
    }

    if (*tcp).syn() != 0 {
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

unsafe fn process_packet(
    data: *mut core::ffi::c_void,
    mut off: __u64,
    data_end: *mut core::ffi::c_void,
    is_ipv6: bool,
    skb: *mut __sk_buff,
) -> core::ffi::c_int {
    let pkt_start: *mut core::ffi::c_void = (*skb).data as usize as *mut core::ffi::c_void;
    let mut pckt: packet_description = core::mem::zeroed();
    let eth: *mut eth_hdr = pkt_start as *mut eth_hdr;
    let mut tkey: bpf_tunnel_key = core::mem::zeroed();
    let mut data_stats: *mut vip_stats;
    let mut dst: *mut real_definition = core::ptr::null_mut();
    let mut vip_info: *mut vip_meta;
    let mut cval: *mut ctl_value;
    let mut v4_intf_pos: __u32 = 1;
    let mut v6_intf_pos: __u32 = 2;
    let mut ip6h: *mut ipv6hdr;
    let mut vip: vip = core::mem::zeroed();
    let mut iph: *mut iphdr;
    let mut tun_flag: core::ffi::c_int = 0;
    let mut pkt_bytes: __u16;
    let mut iph_len: __u64;
    let mut ifindex: __u32;
    let mut protocol: __u8;
    let mut vip_num: __u32;
    let mut action: core::ffi::c_int;

    tkey.tunnel_ttl = 64;
    if is_ipv6 {
        ip6h = (data as *mut u8).add(off as usize) as *mut ipv6hdr;
        if ip6h.add(1) as *mut core::ffi::c_void > data_end {
            return TC_ACT_SHOT;
        }

        iph_len = core::mem::size_of::<ipv6hdr>() as __u64;
        protocol = (*ip6h).nexthdr;
        pckt.proto = protocol;
        pkt_bytes = bpf_ntohs((*ip6h).payload_len);
        off = off.wrapping_add(iph_len);
        if protocol as core::ffi::c_int == IPPROTO_FRAGMENT {
            return TC_ACT_SHOT;
        } else if protocol as core::ffi::c_int == IPPROTO_ICMPV6 {
            action = parse_icmpv6(data, data_end, off, &mut pckt);
            if action >= 0 {
                return action;
            }
            off = off.wrapping_add(IPV6_PLUS_ICMP_HDR);
        } else {
            core::ptr::copy_nonoverlapping(
                (*ip6h).saddr.s6_addr32.as_ptr(),
                pckt.src_u.srcv6.as_mut_ptr(),
                4,
            );
            core::ptr::copy_nonoverlapping(
                (*ip6h).daddr.s6_addr32.as_ptr(),
                pckt.dst_u.dstv6.as_mut_ptr(),
                4,
            );
        }
    } else {
        iph = (data as *mut u8).add(off as usize) as *mut iphdr;
        if iph.add(1) as *mut core::ffi::c_void > data_end {
            return TC_ACT_SHOT;
        }
        if (*iph).ihl() != 5 {
            return TC_ACT_SHOT;
        }

        protocol = (*iph).protocol;
        pckt.proto = protocol;
        pkt_bytes = bpf_ntohs((*iph).tot_len);
        off = off.wrapping_add(IPV4_HDR_LEN_NO_OPT);

        if ((*iph).frag_off & PCKT_FRAGMENTED as __be16) != 0 {
            return TC_ACT_SHOT;
        }
        if protocol as core::ffi::c_int == IPPROTO_ICMP {
            action = parse_icmp(data, data_end, off, &mut pckt);
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

    if protocol as core::ffi::c_int == IPPROTO_TCP {
        if !parse_tcp(data, off, data_end, &mut pckt) {
            return TC_ACT_SHOT;
        }
    } else if protocol as core::ffi::c_int == IPPROTO_UDP {
        if !parse_udp(data, off, data_end, &mut pckt) {
            return TC_ACT_SHOT;
        }
    } else {
        return TC_ACT_SHOT;
    }

    if is_ipv6 {
        core::ptr::copy_nonoverlapping(pckt.dst_u.dstv6.as_ptr(), vip.daddr.v6.as_mut_ptr(), 4);
    } else {
        vip.daddr.v4 = pckt.dst_u.dst;
    }

    vip.dport = pckt.ports_u.port16[1];
    vip.protocol = pckt.proto;
    vip_info = bpf_map_lookup_elem(&mut vip_map as *mut _ as *mut _, &mut vip as *mut _ as *mut _)
        as *mut vip_meta;
    if vip_info.is_null() {
        vip.dport = 0;
        vip_info =
            bpf_map_lookup_elem(&mut vip_map as *mut _ as *mut _, &mut vip as *mut _ as *mut _)
                as *mut vip_meta;
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

    if ((*dst).flags & F_IPV6) != 0 {
        cval = bpf_map_lookup_elem(
            &mut ctl_array as *mut _ as *mut _,
            &mut v6_intf_pos as *mut _ as *mut _,
        ) as *mut ctl_value;
        if cval.is_null() {
            return TC_ACT_SHOT;
        }
        ifindex = (*cval).ifindex;
        core::ptr::copy_nonoverlapping(
            (*dst).dst_u.dstv6.as_ptr(),
            tkey.remote_ipv6.as_mut_ptr(),
            4,
        );
        tun_flag = BPF_F_TUNINFO_IPV6;
    } else {
        cval = bpf_map_lookup_elem(
            &mut ctl_array as *mut _ as *mut _,
            &mut v4_intf_pos as *mut _ as *mut _,
        ) as *mut ctl_value;
        if cval.is_null() {
            return TC_ACT_SHOT;
        }
        ifindex = (*cval).ifindex;
        tkey.remote_ipv4 = (*dst).dst_u.dst;
    }
    vip_num = (*vip_info).vip_num;
    data_stats = bpf_map_lookup_elem(
        &mut stats as *mut _ as *mut _,
        &mut vip_num as *mut _ as *mut _,
    ) as *mut vip_stats;
    if data_stats.is_null() {
        return TC_ACT_SHOT;
    }
    (*data_stats).pkts = (*data_stats).pkts.wrapping_add(1);
    (*data_stats).bytes = (*data_stats).bytes.wrapping_add(pkt_bytes as __u64);
    bpf_skb_set_tunnel_key(
        skb,
        &mut tkey as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&tkey) as __u32,
        tun_flag,
    );
    *((eth as *mut u8).add(core::mem::offset_of!(eth_hdr, eth_dest)) as *mut u32) =
        tkey.remote_ipv4;
    bpf_redirect(ifindex, 0)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn balancer_ingress(ctx: *mut __sk_buff) -> core::ffi::c_int {
    let data_end: *mut core::ffi::c_void = (*ctx).data_end as usize as *mut core::ffi::c_void;
    let data: *mut core::ffi::c_void = (*ctx).data as usize as *mut core::ffi::c_void;
    let eth: *mut eth_hdr = data as *mut eth_hdr;
    let mut eth_proto: __u32;
    let mut nh_off: __u32;

    nh_off = core::mem::size_of::<eth_hdr>() as __u32;
    if (data as *mut u8).add(nh_off as usize) as *mut core::ffi::c_void > data_end {
        return TC_ACT_SHOT;
    }
    eth_proto = (*eth).eth_proto as __u32;
    if eth_proto == bpf_htons(ETH_P_IP) as __u32 {
        process_packet(data, nh_off as __u64, data_end, false, ctx)
    } else if eth_proto == bpf_htons(ETH_P_IPV6) as __u32 {
        process_packet(data, nh_off as __u64, data_end, true, ctx)
    } else {
        TC_ACT_SHOT
    }
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];
