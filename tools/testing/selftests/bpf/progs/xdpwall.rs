// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/*
 * Translated from C. Original includes:
 * <stdbool.h>, <stdint.h>, <linux/stddef.h>, <linux/if_ether.h>,
 * <linux/in.h>, <linux/in6.h>, <linux/ip.h>, <linux/ipv6.h>,
 * <linux/tcp.h>, <linux/udp.h>, <linux/bpf.h>, <linux/types.h>,
 * <bpf/bpf_endian.h>, <bpf/bpf_helpers.h>
 */

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;

const BPF_MAP_TYPE_HASH: __u32 = 1;
const BPF_MAP_TYPE_ARRAY: __u32 = 2;
const BPF_MAP_TYPE_LPM_TRIE: __u32 = 11;
const BPF_F_NO_PREALLOC: __u32 = 1;

const ETH_P_IPV6: __u16 = 0x86dd;
const IPPROTO_ICMPV6: __u8 = 58;
const IPPROTO_TCP: __u8 = 6;
const IPPROTO_UDP: __u8 = 17;

const TCP_FLAG_FIN: __u32 = 0x00010000;
const TCP_FLAG_SYN: __u32 = 0x00020000;
const TCP_FLAG_RST: __u32 = 0x00040000;
const TCP_FLAG_ACK: __u32 = 0x00100000;

const XDP_DROP: i32 = 1;
const XDP_PASS: i32 = 2;

#[repr(u32)]
enum pkt_parse_err {
    NO_ERR,
    BAD_IP6_HDR,
    BAD_IP4GUE_HDR,
    BAD_IP6GUE_HDR,
}

#[repr(u32)]
enum pkt_flag {
    TUNNEL = 0x1,
    TCP_SYN = 0x2,
    QUIC_INITIAL_FLAG = 0x4,
    TCP_ACK = 0x8,
    TCP_RST = 0x10,
}

#[repr(C)]
struct v4_lpm_key {
    prefixlen: __u32,
    src: __u32,
}

#[repr(C)]
struct v4_lpm_val {
    key: v4_lpm_key,
    val: __u8,
}

#[repr(C)]
struct in6_addr {
    in6_u: [__u8; 16],
}

#[repr(C)]
struct ethhdr {
    h_dest: [__u8; 6],
    h_source: [__u8; 6],
    h_proto: __u16,
}

#[repr(C)]
struct iphdr {
    ihl_version: __u8,
    tos: __u8,
    tot_len: __u16,
    id: __u16,
    frag_off: __u16,
    ttl: __u8,
    protocol: __u8,
    check: __u16,
    saddr: __u32,
    daddr: __u32,
}

#[repr(C)]
struct ipv6hdr {
    priority_version: __u8,
    flow_lbl: [__u8; 3],
    payload_len: __u16,
    nexthdr: __u8,
    hop_limit: __u8,
    saddr: in6_addr,
    daddr: in6_addr,
}

#[repr(C)]
struct tcphdr {
    source: __u16,
    dest: __u16,
    seq: __u32,
    ack_seq: __u32,
    doff_res_flags: __u16,
    window: __u16,
    check: __u16,
    urg_ptr: __u16,
}

#[repr(C)]
struct udphdr {
    source: __u16,
    dest: __u16,
    len: __u16,
    check: __u16,
}

#[repr(C)]
struct xdp_md {
    data: __u32,
    data_end: __u32,
    data_meta: __u32,
    ingress_ifindex: __u32,
    rx_queue_index: __u32,
    egress_ifindex: __u32,
}

#[repr(C)]
struct bpf_map_def {
    type_: __u32,
    max_entries: __u32,
    key_size: __u32,
    value_size: __u32,
    map_flags: __u32,
}

/* Original BPF map declarations used __uint/__type and SEC(".maps"). */
#[no_mangle]
#[link_section = ".maps"]
static v6_addr_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 16,
    key_size: core::mem::size_of::<in6_addr>() as __u32,
    value_size: core::mem::size_of::<bool>() as __u32,
    map_flags: 0,
};

#[no_mangle]
#[link_section = ".maps"]
static v4_addr_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 16,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<bool>() as __u32,
    map_flags: 0,
};

#[no_mangle]
#[link_section = ".maps"]
static v4_lpm_val_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_LPM_TRIE,
    max_entries: 16,
    key_size: core::mem::size_of::<v4_lpm_key>() as __u32,
    value_size: core::mem::size_of::<v4_lpm_val>() as __u32,
    map_flags: BPF_F_NO_PREALLOC,
};

#[no_mangle]
#[link_section = ".maps"]
static tcp_port_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 16,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<__u8>() as __u32,
    map_flags: 0,
};

#[no_mangle]
#[link_section = ".maps"]
static udp_port_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 16,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<__u16>() as __u32,
    map_flags: 0,
};

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum ip_type {
    V4 = 1,
    V6 = 2,
}

#[repr(C)]
struct fw_match_info {
    v4_src_ip_match: __u8,
    v6_src_ip_match: __u8,
    v4_src_prefix_match: __u8,
    v4_dst_prefix_match: __u8,
    tcp_dp_match: __u8,
    udp_sp_match: __u16,
    udp_dp_match: __u16,
    is_tcp: bool,
    is_tcp_syn: bool,
}

#[repr(C)]
union pkt_info_ip {
    ipv4: *mut iphdr,
    ipv6: *mut ipv6hdr,
}

#[repr(C)]
struct pkt_info {
    type_: ip_type,
    ip: pkt_info_ip,
    sport: i32,
    dport: i32,
    trans_hdr_offset: __u16,
    proto: __u8,
    flags: __u8,
}

extern "C" {
    fn bpf_map_lookup_elem(map: *const bpf_map_def, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

#[inline(always)]
fn bpf_ntohs(x: __u16) -> __u16 {
    __u16::from_be(x)
}

#[inline(always)]
fn bpf_htons(x: __u16) -> __u16 {
    x.to_be()
}

#[inline(always)]
unsafe fn tcp_flag_word(tcp: *mut tcphdr) -> __u32 {
    ((*tcp).doff_res_flags as __u32) << 16
}

#[inline(always)]
unsafe fn parse_ethhdr(data: *mut core::ffi::c_void, data_end: *mut core::ffi::c_void) -> *mut ethhdr {
    let eth = data as *mut ethhdr;

    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return core::ptr::null_mut();
    }

    eth
}

#[inline(always)]
unsafe fn filter_ipv6_addr(ipv6addr: *const in6_addr) -> __u8 {
    let leaf: *mut __u8;

    leaf = bpf_map_lookup_elem(&v6_addr_map, ipv6addr as *const core::ffi::c_void) as *mut __u8;

    if !leaf.is_null() { *leaf } else { 0 }
}

#[inline(always)]
unsafe fn filter_ipv4_addr(ipaddr: __u32) -> __u8 {
    let leaf: *mut __u8;

    leaf = bpf_map_lookup_elem(
        &v4_addr_map,
        &ipaddr as *const __u32 as *const core::ffi::c_void,
    ) as *mut __u8;

    if !leaf.is_null() { *leaf } else { 0 }
}

#[inline(always)]
unsafe fn filter_ipv4_lpm(ipaddr: __u32) -> __u8 {
    let mut v4_key: v4_lpm_key = core::mem::zeroed();
    let lpm_val: *mut v4_lpm_val;

    v4_key.src = ipaddr;
    v4_key.prefixlen = 32;

    lpm_val = bpf_map_lookup_elem(
        &v4_lpm_val_map,
        &v4_key as *const v4_lpm_key as *const core::ffi::c_void,
    ) as *mut v4_lpm_val;

    if !lpm_val.is_null() { (*lpm_val).val } else { 0 }
}

#[inline(always)]
unsafe fn filter_src_dst_ip(info: *mut pkt_info, match_info: *mut fw_match_info) {
    if (*info).type_ == ip_type::V6 {
        (*match_info).v6_src_ip_match = filter_ipv6_addr(&(*(*info).ip.ipv6).saddr);
    } else if (*info).type_ == ip_type::V4 {
        (*match_info).v4_src_ip_match = filter_ipv4_addr((*(*info).ip.ipv4).saddr);
        (*match_info).v4_src_prefix_match = filter_ipv4_lpm((*(*info).ip.ipv4).saddr);
        (*match_info).v4_dst_prefix_match = filter_ipv4_lpm((*(*info).ip.ipv4).daddr);
    }
}

#[inline(always)]
unsafe fn get_transport_hdr(
    offset: __u16,
    data: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    if offset > 255 || (data as *mut u8).add(offset as usize) as *mut core::ffi::c_void > data_end {
        return core::ptr::null_mut();
    }

    (data as *mut u8).add(offset as usize) as *mut core::ffi::c_void
}

#[inline(always)]
unsafe fn tcphdr_only_contains_flag(tcp: *mut tcphdr, flag: __u32) -> bool {
    (tcp_flag_word(tcp) & (TCP_FLAG_ACK | TCP_FLAG_RST | TCP_FLAG_SYN | TCP_FLAG_FIN)) == flag
}

#[inline(always)]
unsafe fn set_tcp_flags(info: *mut pkt_info, tcp: *mut tcphdr) {
    if tcphdr_only_contains_flag(tcp, TCP_FLAG_SYN) {
        (*info).flags |= pkt_flag::TCP_SYN as __u8;
    } else if tcphdr_only_contains_flag(tcp, TCP_FLAG_ACK) {
        (*info).flags |= pkt_flag::TCP_ACK as __u8;
    } else if tcphdr_only_contains_flag(tcp, TCP_FLAG_RST) {
        (*info).flags |= pkt_flag::TCP_RST as __u8;
    }
}

#[inline(always)]
unsafe fn parse_tcp(
    info: *mut pkt_info,
    transport_hdr: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
) -> bool {
    let tcp = transport_hdr as *mut tcphdr;

    if tcp.add(1) as *mut core::ffi::c_void > data_end {
        return false;
    }

    (*info).sport = bpf_ntohs((*tcp).source) as i32;
    (*info).dport = bpf_ntohs((*tcp).dest) as i32;
    set_tcp_flags(info, tcp);

    true
}

#[inline(always)]
unsafe fn parse_udp(
    info: *mut pkt_info,
    transport_hdr: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
) -> bool {
    let udp = transport_hdr as *mut udphdr;

    if udp.add(1) as *mut core::ffi::c_void > data_end {
        return false;
    }

    (*info).sport = bpf_ntohs((*udp).source) as i32;
    (*info).dport = bpf_ntohs((*udp).dest) as i32;

    true
}

#[inline(always)]
unsafe fn filter_tcp_port(port: i32) -> __u8 {
    let leaf = bpf_map_lookup_elem(
        &tcp_port_map,
        &port as *const i32 as *const core::ffi::c_void,
    ) as *mut __u8;

    if !leaf.is_null() { *leaf } else { 0 }
}

#[inline(always)]
unsafe fn filter_udp_port(port: i32) -> __u16 {
    let leaf = bpf_map_lookup_elem(
        &udp_port_map,
        &port as *const i32 as *const core::ffi::c_void,
    ) as *mut __u16;

    if !leaf.is_null() { *leaf } else { 0 }
}

#[inline(always)]
unsafe fn filter_transport_hdr(
    transport_hdr: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
    info: *mut pkt_info,
    match_info: *mut fw_match_info,
) -> bool {
    if (*info).proto == IPPROTO_TCP {
        if !parse_tcp(info, transport_hdr, data_end) {
            return false;
        }

        (*match_info).is_tcp = true;
        (*match_info).is_tcp_syn = ((*info).flags & pkt_flag::TCP_SYN as __u8) > 0;

        (*match_info).tcp_dp_match = filter_tcp_port((*info).dport);
    } else if (*info).proto == IPPROTO_UDP {
        if !parse_udp(info, transport_hdr, data_end) {
            return false;
        }

        (*match_info).udp_dp_match = filter_udp_port((*info).dport);
        (*match_info).udp_sp_match = filter_udp_port((*info).sport);
    }

    true
}

#[inline(always)]
unsafe fn parse_gue_v6(
    info: *mut pkt_info,
    ip6h: *mut ipv6hdr,
    data_end: *mut core::ffi::c_void,
) -> __u8 {
    let udp = ip6h.add(1) as *mut udphdr;
    let encap_data = udp.add(1) as *mut core::ffi::c_void;

    if udp.add(1) as *mut core::ffi::c_void > data_end {
        return pkt_parse_err::BAD_IP6_HDR as __u8;
    }

    if (*udp).dest != bpf_htons(6666) {
        return pkt_parse_err::NO_ERR as __u8;
    }

    (*info).flags |= pkt_flag::TUNNEL as __u8;

    if (encap_data as *mut u8).add(1) as *mut core::ffi::c_void > data_end {
        return pkt_parse_err::BAD_IP6GUE_HDR as __u8;
    }

    if (*(encap_data as *mut __u8) & 0x30) != 0 {
        let inner_ip6h = encap_data as *mut ipv6hdr;

        if inner_ip6h.add(1) as *mut core::ffi::c_void > data_end {
            return pkt_parse_err::BAD_IP6GUE_HDR as __u8;
        }

        (*info).type_ = ip_type::V6;
        (*info).proto = (*inner_ip6h).nexthdr;
        (*info).ip.ipv6 = inner_ip6h;
        (*info).trans_hdr_offset = (*info)
            .trans_hdr_offset
            .wrapping_add(core::mem::size_of::<ipv6hdr>() as __u16)
            .wrapping_add(core::mem::size_of::<udphdr>() as __u16);
    } else {
        let inner_ip4h = encap_data as *mut iphdr;

        if inner_ip4h.add(1) as *mut core::ffi::c_void > data_end {
            return pkt_parse_err::BAD_IP6GUE_HDR as __u8;
        }

        (*info).type_ = ip_type::V4;
        (*info).proto = (*inner_ip4h).protocol;
        (*info).ip.ipv4 = inner_ip4h;
        (*info).trans_hdr_offset = (*info)
            .trans_hdr_offset
            .wrapping_add(core::mem::size_of::<iphdr>() as __u16)
            .wrapping_add(core::mem::size_of::<udphdr>() as __u16);
    }

    pkt_parse_err::NO_ERR as __u8
}

#[inline(always)]
unsafe fn parse_ipv6_gue(
    info: *mut pkt_info,
    data: *mut core::ffi::c_void,
    data_end: *mut core::ffi::c_void,
) -> __u8 {
    let ip6h = (data as *mut u8).add(core::mem::size_of::<ethhdr>()) as *mut ipv6hdr;

    if ip6h.add(1) as *mut core::ffi::c_void > data_end {
        return pkt_parse_err::BAD_IP6_HDR as __u8;
    }

    (*info).proto = (*ip6h).nexthdr;
    (*info).ip.ipv6 = ip6h;
    (*info).type_ = ip_type::V6;
    (*info).trans_hdr_offset =
        (core::mem::size_of::<ethhdr>() + core::mem::size_of::<ipv6hdr>()) as __u16;

    if (*info).proto == IPPROTO_UDP {
        return parse_gue_v6(info, ip6h, data_end);
    }

    pkt_parse_err::NO_ERR as __u8
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn edgewall(ctx: *mut xdp_md) -> i32 {
    let data_end = (*ctx).data_end as usize as *mut core::ffi::c_void;
    let data = (*ctx).data as usize as *mut core::ffi::c_void;
    let mut match_info: fw_match_info = core::mem::zeroed();
    let mut info: pkt_info = core::mem::zeroed();
    let transport_hdr: *mut core::ffi::c_void;
    let eth: *mut ethhdr;
    let filter_res: bool;
    let proto: __u32;

    eth = parse_ethhdr(data, data_end);
    if eth.is_null() {
        return XDP_DROP;
    }

    proto = (*eth).h_proto as __u32;
    if proto != bpf_htons(ETH_P_IPV6) as __u32 {
        return XDP_DROP;
    }

    if parse_ipv6_gue(&mut info, data, data_end) != 0 {
        return XDP_DROP;
    }

    if info.proto == IPPROTO_ICMPV6 {
        return XDP_PASS;
    }

    if info.proto != IPPROTO_TCP && info.proto != IPPROTO_UDP {
        return XDP_DROP;
    }

    filter_src_dst_ip(&mut info, &mut match_info);

    transport_hdr = get_transport_hdr(info.trans_hdr_offset, data, data_end);
    if transport_hdr.is_null() {
        return XDP_DROP;
    }

    filter_res = filter_transport_hdr(transport_hdr, data_end, &mut info, &mut match_info);
    if !filter_res {
        return XDP_DROP;
    }

    if match_info.is_tcp && !match_info.is_tcp_syn {
        return XDP_PASS;
    }

    XDP_DROP
}

#[no_mangle]
#[link_section = "license"]
pub static LICENSE: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
