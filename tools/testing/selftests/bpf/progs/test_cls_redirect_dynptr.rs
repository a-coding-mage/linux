// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2019, 2020 Cloudflare

// Translated from test_cls_redirect_dynptr.c. C include dependencies are
// expected to be supplied by the surrounding BPF/Rust build environment.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::{offset_of, size_of};
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type __be16 = u16;
type __be32 = u32;

const IP_OFFSET_MASK: u16 = 0x1FFF;
const IP_MF: u16 = 0x2000;

#[used]
#[link_section = "license"]
pub static mut _license: [u8; 13] = *b"Dual BSD/GPL\0";

/**
 * Destination port and IP used for UDP encapsulation.
 */
pub static ENCAPSULATION_PORT: __be16 = 0;
pub static ENCAPSULATION_IP: __be32 = 0;

#[repr(C)]
pub struct metrics_t {
    pub processed_packets_total: u64,
    pub l3_protocol_packets_total_ipv4: u64,
    pub l3_protocol_packets_total_ipv6: u64,
    pub l4_protocol_packets_total_tcp: u64,
    pub l4_protocol_packets_total_udp: u64,
    pub accepted_packets_total_syn: u64,
    pub accepted_packets_total_syn_cookies: u64,
    pub accepted_packets_total_last_hop: u64,
    pub accepted_packets_total_icmp_echo_request: u64,
    pub accepted_packets_total_established: u64,
    pub forwarded_packets_total_gue: u64,
    pub forwarded_packets_total_gre: u64,

    pub errors_total_unknown_l3_proto: u64,
    pub errors_total_unknown_l4_proto: u64,
    pub errors_total_malformed_ip: u64,
    pub errors_total_fragmented_ip: u64,
    pub errors_total_malformed_icmp: u64,
    pub errors_total_unwanted_icmp: u64,
    pub errors_total_malformed_icmp_pkt_too_big: u64,
    pub errors_total_malformed_tcp: u64,
    pub errors_total_malformed_udp: u64,
    pub errors_total_icmp_echo_replies: u64,
    pub errors_total_malformed_encapsulation: u64,
    pub errors_total_encap_adjust_failed: u64,
    pub errors_total_encap_buffer_too_small: u64,
    pub errors_total_redirect_loop: u64,
    pub errors_total_encap_mtu_violate: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum verdict_t {
    INVALID = 0,
    UNKNOWN,
    ECHO_REQUEST,
    SYN,
    SYN_COOKIE,
    ESTABLISHED,
}

#[repr(C)]
pub struct flow_ports_t {
    pub src: u16,
    pub dst: u16,
}

// Static assertions from C:
// sizeof(flow_ports_t) must match sport and dport in struct bpf_sock_tuple
// for both ipv4 and ipv6 tuple layouts.

#[repr(C)]
pub struct iphdr_info {
    pub hdr: *mut c_void,
    pub len: __u64,
}

type ret_t = i32;

/* This is a bit of a hack. We need a return value which allows us to
 * indicate that the regular flow of the program should continue,
 * while allowing functions to use XDP_PASS and XDP_DROP, etc.
 */
const CONTINUE_PROCESSING: ret_t = -1;

unsafe fn ipv4_is_fragment(ip: *const iphdr) -> bool {
    let frag_off = (*ip).frag_off & bpf_htons(IP_OFFSET_MASK);
    ((*ip).frag_off & bpf_htons(IP_MF)) != 0 || frag_off > 0
}

unsafe fn pkt_parse_ipv4(dynptr: *mut bpf_dynptr, offset: *mut __u64, iphdr: *mut iphdr) -> i32 {
    if bpf_dynptr_read(
        iphdr as *mut c_void,
        size_of::<iphdr>() as __u32,
        dynptr,
        *offset,
        0,
    ) != 0
    {
        return -1;
    }

    *offset += size_of::<iphdr>() as __u64;

    if (*iphdr).ihl < 5 {
        return -1;
    }

    /* skip ipv4 options */
    *offset += (((*iphdr).ihl - 5) as __u64) * 4;

    0
}

/* Parse the L4 ports from a packet, assuming a layout like TCP or UDP. */
unsafe fn pkt_parse_icmp_l4_ports(
    dynptr: *mut bpf_dynptr,
    offset: *mut __u64,
    ports: *mut flow_ports_t,
) -> bool {
    if bpf_dynptr_read(
        ports as *mut c_void,
        size_of::<flow_ports_t>() as __u32,
        dynptr,
        *offset,
        0,
    ) != 0
    {
        return false;
    }

    *offset += size_of::<flow_ports_t>() as __u64;

    /* Ports in the L4 headers are reversed, since we are parsing an ICMP
     * payload which is going towards the eyeball.
     */
    let dst = (*ports).src;
    (*ports).src = (*ports).dst;
    (*ports).dst = dst;
    true
}

fn pkt_checksum_fold(mut csum: u32) -> u16 {
    /* The highest reasonable value for an IPv4 header
     * checksum requires two folds, so we just do that always.
     */
    csum = (csum & 0xffff) + (csum >> 16);
    csum = (csum & 0xffff) + (csum >> 16);
    !(csum as u16)
}

unsafe fn pkt_ipv4_checksum(iph: *mut iphdr) {
    (*iph).check = 0;

    /* An IP header without options is 20 bytes. Two of those
     * are the checksum, which we always set to zero. Hence,
     * the maximum accumulated value is 18 / 2 * 0xffff = 0x8fff7,
     * which fits in 32 bit.
     */
    const _: [(); 20] = [(); size_of::<iphdr>()];
    let mut acc: u32 = 0;
    let ipw = iph as *mut u16;

    let mut i = 0usize;
    while i < size_of::<iphdr>() / 2 {
        acc = acc.wrapping_add(*ipw.add(i) as u32);
        i += 1;
    }

    (*iph).check = pkt_checksum_fold(acc);
}

unsafe fn pkt_skip_ipv6_extension_headers(
    dynptr: *mut bpf_dynptr,
    offset: *mut __u64,
    ipv6: *const ipv6hdr,
    upper_proto: *mut u8,
    is_fragment: *mut bool,
) -> bool {
    /* We understand five extension headers.
     * https://tools.ietf.org/html/rfc8200#section-4.1 states that all
     * headers should occur once, except Destination Options, which may
     * occur twice. Hence we give up after 6 headers.
     */
    #[repr(C)]
    struct exthdr_t {
        next: u8,
        len: u8,
    }
    let mut exthdr = exthdr_t {
        next: (*ipv6).nexthdr,
        len: 0,
    };
    *is_fragment = false;

    let mut i = 0;
    while i < 6 {
        match exthdr.next as i32 {
            IPPROTO_FRAGMENT => {
                *is_fragment = true;
                /* NB: We don't check that hdrlen == 0 as per spec. */
                if bpf_dynptr_read(
                    &mut exthdr as *mut _ as *mut c_void,
                    size_of::<exthdr_t>() as __u32,
                    dynptr,
                    *offset,
                    0,
                ) != 0
                {
                    return false;
                }
                *offset += (((exthdr.len as __u64) + 1) * 8) as __u64;
            }
            IPPROTO_HOPOPTS | IPPROTO_ROUTING | IPPROTO_DSTOPTS | IPPROTO_MH => {
                if bpf_dynptr_read(
                    &mut exthdr as *mut _ as *mut c_void,
                    size_of::<exthdr_t>() as __u32,
                    dynptr,
                    *offset,
                    0,
                ) != 0
                {
                    return false;
                }

                /* hdrlen is in 8-octet units, and excludes the first 8 octets. */
                *offset += (((exthdr.len as __u64) + 1) * 8) as __u64;

                /* Decode next header */
            }
            _ => {
                /* The next header is not one of the known extension
                 * headers, treat it as the upper layer header.
                 *
                 * This handles IPPROTO_NONE.
                 *
                 * Encapsulating Security Payload (50) and Authentication
                 * Header (51) also end up here (and will trigger an
                 * unknown proto error later). They have a custom header
                 * format and seem too esoteric to care about.
                 */
                *upper_proto = exthdr.next;
                return true;
            }
        }
        i += 1;
    }

    /* We never found an upper layer header. */
    false
}

unsafe fn pkt_parse_ipv6(
    dynptr: *mut bpf_dynptr,
    offset: *mut __u64,
    ipv6: *mut ipv6hdr,
    proto: *mut u8,
    is_fragment: *mut bool,
) -> i32 {
    if bpf_dynptr_read(
        ipv6 as *mut c_void,
        size_of::<ipv6hdr>() as __u32,
        dynptr,
        *offset,
        0,
    ) != 0
    {
        return -1;
    }

    *offset += size_of::<ipv6hdr>() as __u64;

    if !pkt_skip_ipv6_extension_headers(dynptr, offset, ipv6, proto, is_fragment) {
        return -1;
    }

    0
}

// Global metrics, per CPU. Original C used BPF map-definition macros.
#[link_section = ".maps"]
pub static mut metrics_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: 1,
    key_size: size_of::<u32>() as u32,
    value_size: size_of::<metrics_t>() as u32,
};

unsafe fn get_global_metrics() -> *mut metrics_t {
    let mut key: u64 = 0;
    bpf_map_lookup_elem(
        &mut metrics_map as *mut _ as *mut c_void,
        &mut key as *mut _ as *mut c_void,
    ) as *mut metrics_t
}

unsafe fn accept_locally(skb: *mut __sk_buff, encap: *mut encap_headers_t) -> ret_t {
    let payload_off: i32 =
        (size_of::<encap_headers_t>() + size_of::<in_addr>() * ((*encap).unigue.hop_count as usize))
            as i32;
    let encap_overhead: i32 = payload_off - size_of::<ethhdr>() as i32;

    /* Changing the ethertype if the encapsulated packet is ipv6 */
    if (*encap).gue.proto_ctype as i32 == IPPROTO_IPV6 {
        (*encap).eth.h_proto = bpf_htons(ETH_P_IPV6 as u16);
    }

    if bpf_skb_adjust_room(
        skb,
        -encap_overhead,
        BPF_ADJ_ROOM_MAC,
        BPF_F_ADJ_ROOM_FIXED_GSO | BPF_F_ADJ_ROOM_NO_CSUM_RESET,
    ) != 0
        || bpf_csum_level(skb, BPF_CSUM_LEVEL_DEC) != 0
    {
        return TC_ACT_SHOT;
    }

    bpf_redirect((*skb).ifindex, BPF_F_INGRESS)
}

unsafe fn forward_with_gre(
    skb: *mut __sk_buff,
    dynptr: *mut bpf_dynptr,
    encap: *mut encap_headers_t,
    next_hop: *mut in_addr,
    metrics: *mut metrics_t,
) -> ret_t {
    let payload_off: i32 =
        (size_of::<encap_headers_t>() + size_of::<in_addr>() * ((*encap).unigue.hop_count as usize))
            as i32;
    let encap_overhead: i32 =
        payload_off - size_of::<ethhdr>() as i32 - size_of::<iphdr>() as i32;
    let delta: i32 = size_of::<gre_base_hdr>() as i32 - encap_overhead;
    let mut encap_buffer = [0u8; size_of::<encap_gre_t>()];
    let mut proto: u16 = ETH_P_IP as u16;
    let mut mtu_len: u32 = 0;

    (*metrics).forwarded_packets_total_gre += 1;

    /* Loop protection: the inner packet's TTL is decremented as a safeguard
     * against any forwarding loop. As the only interesting field is the TTL
     * hop limit for IPv6, it is easier to use bpf_skb_load_bytes/bpf_skb_store_bytes
     * as they handle the split packets if needed (no need for the data to be
     * in the linear section).
     */
    if (*encap).gue.proto_ctype as i32 == IPPROTO_IPV6 {
        proto = ETH_P_IPV6 as u16;
        let mut ttl: u8 = 0;

        let rc = bpf_skb_load_bytes(
            skb,
            payload_off + offset_of!(ipv6hdr, hop_limit) as i32,
            &mut ttl as *mut _ as *mut c_void,
            1,
        );
        if rc != 0 {
            (*metrics).errors_total_malformed_encapsulation += 1;
            return TC_ACT_SHOT;
        }

        if ttl == 0 {
            (*metrics).errors_total_redirect_loop += 1;
            return TC_ACT_SHOT;
        }

        ttl = ttl.wrapping_sub(1);
        let rc = bpf_skb_store_bytes(
            skb,
            payload_off + offset_of!(ipv6hdr, hop_limit) as i32,
            &mut ttl as *mut _ as *mut c_void,
            1,
            0,
        );
        if rc != 0 {
            (*metrics).errors_total_malformed_encapsulation += 1;
            return TC_ACT_SHOT;
        }
    } else {
        let mut ttl: u8 = 0;

        let rc = bpf_skb_load_bytes(
            skb,
            payload_off + offset_of!(iphdr, ttl) as i32,
            &mut ttl as *mut _ as *mut c_void,
            1,
        );
        if rc != 0 {
            (*metrics).errors_total_malformed_encapsulation += 1;
            return TC_ACT_SHOT;
        }

        if ttl == 0 {
            (*metrics).errors_total_redirect_loop += 1;
            return TC_ACT_SHOT;
        }

        /* IPv4 also has a checksum to patch. While the TTL is only one byte,
         * this function only works for 2 and 4 bytes arguments (the result is
         * the same).
         */
        let rc = bpf_l3_csum_replace(
            skb,
            payload_off + offset_of!(iphdr, check) as i32,
            ttl as u64,
            ttl.wrapping_sub(1) as u64,
            2,
        );
        if rc != 0 {
            (*metrics).errors_total_malformed_encapsulation += 1;
            return TC_ACT_SHOT;
        }

        ttl = ttl.wrapping_sub(1);
        let rc = bpf_skb_store_bytes(
            skb,
            payload_off + offset_of!(iphdr, ttl) as i32,
            &mut ttl as *mut _ as *mut c_void,
            1,
            0,
        );
        if rc != 0 {
            (*metrics).errors_total_malformed_encapsulation += 1;
            return TC_ACT_SHOT;
        }
    }

    if bpf_check_mtu(skb, (*skb).ifindex, &mut mtu_len, delta, 0) != 0 {
        (*metrics).errors_total_encap_mtu_violate += 1;
        return TC_ACT_SHOT;
    }

    if bpf_skb_adjust_room(
        skb,
        delta,
        BPF_ADJ_ROOM_NET,
        BPF_F_ADJ_ROOM_FIXED_GSO | BPF_F_ADJ_ROOM_NO_CSUM_RESET,
    ) != 0
        || bpf_csum_level(skb, BPF_CSUM_LEVEL_INC) != 0
    {
        (*metrics).errors_total_encap_adjust_failed += 1;
        return TC_ACT_SHOT;
    }

    if bpf_skb_pull_data(skb, size_of::<encap_gre_t>() as u32) != 0 {
        (*metrics).errors_total_encap_buffer_too_small += 1;
        return TC_ACT_SHOT;
    }

    let encap_gre = bpf_dynptr_slice_rdwr(
        dynptr,
        0,
        encap_buffer.as_mut_ptr() as *mut c_void,
        encap_buffer.len() as __u32,
    ) as *mut encap_gre_t;
    if encap_gre.is_null() {
        (*metrics).errors_total_encap_buffer_too_small += 1;
        return TC_ACT_SHOT;
    }

    (*encap_gre).ip.protocol = IPPROTO_GRE as u8;
    (*encap_gre).ip.daddr = (*next_hop).s_addr;
    (*encap_gre).ip.saddr = ENCAPSULATION_IP;
    (*encap_gre).ip.tot_len =
        bpf_htons(bpf_ntohs((*encap_gre).ip.tot_len).wrapping_add(delta as u16));
    (*encap_gre).gre.flags = 0;
    (*encap_gre).gre.protocol = bpf_htons(proto);
    pkt_ipv4_checksum(&mut (*encap_gre).ip);

    if encap_gre == encap_buffer.as_mut_ptr() as *mut encap_gre_t {
        bpf_dynptr_write(
            dynptr,
            0,
            encap_buffer.as_ptr() as *const c_void,
            encap_buffer.len() as __u32,
            0,
        );
    }

    bpf_redirect((*skb).ifindex, 0)
}

unsafe fn forward_to_next_hop(
    skb: *mut __sk_buff,
    dynptr: *mut bpf_dynptr,
    encap: *mut encap_headers_t,
    next_hop: *mut in_addr,
    metrics: *mut metrics_t,
) -> ret_t {
    /* swap L2 addresses */
    /* This assumes that packets are received from a router.
     * So just swapping the MAC addresses here will make the packet go back to
     * the router, which will send it to the appropriate machine.
     */
    let mut temp = [0u8; ETH_ALEN as usize];
    ptr::copy_nonoverlapping((*encap).eth.h_dest.as_ptr(), temp.as_mut_ptr(), temp.len());
    ptr::copy_nonoverlapping(
        (*encap).eth.h_source.as_ptr(),
        (*encap).eth.h_dest.as_mut_ptr(),
        (*encap).eth.h_dest.len(),
    );
    ptr::copy_nonoverlapping(
        temp.as_ptr(),
        (*encap).eth.h_source.as_mut_ptr(),
        (*encap).eth.h_source.len(),
    );

    if (*encap).unigue.next_hop == (*encap).unigue.hop_count - 1 && (*encap).unigue.last_hop_gre != 0
    {
        return forward_with_gre(skb, dynptr, encap, next_hop, metrics);
    }

    (*metrics).forwarded_packets_total_gue += 1;
    let old_saddr = (*encap).ip.saddr;
    (*encap).ip.saddr = (*encap).ip.daddr;
    (*encap).ip.daddr = (*next_hop).s_addr;
    if (*encap).unigue.next_hop < (*encap).unigue.hop_count {
        (*encap).unigue.next_hop += 1;
    }

    /* Remove ip->saddr, add next_hop->s_addr */
    let off = offset_of!(encap_headers_t, ip) + offset_of!(iphdr, check);
    let ret = bpf_l3_csum_replace(skb, off as i32, old_saddr as u64, (*next_hop).s_addr as u64, 4);
    if ret < 0 {
        return TC_ACT_SHOT;
    }

    bpf_redirect((*skb).ifindex, 0)
}

unsafe fn skip_next_hops(offset: *mut __u64, n: i32) -> ret_t {
    match n {
        1 => {
            *offset += size_of::<in_addr>() as __u64;
            CONTINUE_PROCESSING
        }
        0 => CONTINUE_PROCESSING,
        _ => TC_ACT_SHOT,
    }
}

/* Get the next hop from the GLB header.
 *
 * Sets next_hop->s_addr to 0 if there are no more hops left.
 * pkt is positioned just after the variable length GLB header
 * iff the call is successful.
 */
unsafe fn get_next_hop(
    dynptr: *mut bpf_dynptr,
    offset: *mut __u64,
    encap: *mut encap_headers_t,
    next_hop: *mut in_addr,
) -> ret_t {
    if (*encap).unigue.next_hop > (*encap).unigue.hop_count {
        return TC_ACT_SHOT;
    }

    /* Skip "used" next hops. */
    let ret = skip_next_hops(offset, (*encap).unigue.next_hop as i32);
    if ret != CONTINUE_PROCESSING {
        return ret;
    }

    if (*encap).unigue.next_hop == (*encap).unigue.hop_count {
        /* No more next hops, we are at the end of the GLB header. */
        (*next_hop).s_addr = 0;
        return CONTINUE_PROCESSING;
    }

    if bpf_dynptr_read(
        next_hop as *mut c_void,
        size_of::<in_addr>() as __u32,
        dynptr,
        *offset,
        0,
    ) != 0
    {
        return TC_ACT_SHOT;
    }

    *offset += size_of::<in_addr>() as __u64;

    /* Skip the remaining next hops (may be zero). */
    skip_next_hops(
        offset,
        ((*encap).unigue.hop_count - (*encap).unigue.next_hop - 1) as i32,
    )
}

/* Fill a bpf_sock_tuple to be used with the socket lookup functions.
 * This is a kludge that let's us work around verifier limitations:
 *
 *    fill_tuple(&t, foo, sizeof(struct iphdr), 123, 321)
 *
 * clang will substitute a constant for sizeof, which allows the verifier
 * to track it's value. Based on this, it can figure out the constant
 * return value, and calling code works while still being "generic" to
 * IPv4 and IPv6.
 */
unsafe fn fill_tuple(
    tuple: *mut bpf_sock_tuple,
    iph: *mut c_void,
    iphlen: u64,
    sport: u16,
    dport: u16,
) -> u64 {
    match iphlen as usize {
        n if n == size_of::<iphdr>() => {
            let ipv4 = iph as *mut iphdr;
            (*tuple).ipv4.daddr = (*ipv4).daddr;
            (*tuple).ipv4.saddr = (*ipv4).saddr;
            (*tuple).ipv4.sport = sport;
            (*tuple).ipv4.dport = dport;
            size_of_val_ipv4_tuple(tuple) as u64
        }

        n if n == size_of::<ipv6hdr>() => {
            let ipv6 = iph as *mut ipv6hdr;
            ptr::copy_nonoverlapping(
                &(*ipv6).daddr as *const _ as *const u8,
                &mut (*tuple).ipv6.daddr as *mut _ as *mut u8,
                size_of_val_ipv6_addr_tuple(tuple, true),
            );
            ptr::copy_nonoverlapping(
                &(*ipv6).saddr as *const _ as *const u8,
                &mut (*tuple).ipv6.saddr as *mut _ as *mut u8,
                size_of_val_ipv6_addr_tuple(tuple, false),
            );
            (*tuple).ipv6.sport = sport;
            (*tuple).ipv6.dport = dport;
            size_of_val_ipv6_tuple(tuple) as u64
        }

        _ => 0,
    }
}

unsafe fn classify_tcp(
    skb: *mut __sk_buff,
    tuple: *mut bpf_sock_tuple,
    tuplen: u64,
    iph: *mut c_void,
    tcp: *mut tcphdr,
) -> verdict_t {
    let sk = bpf_skc_lookup_tcp(skb, tuple, tuplen as __u32, BPF_F_CURRENT_NETNS, 0);

    if sk.is_null() {
        return verdict_t::UNKNOWN;
    }

    if (*sk).state != BPF_TCP_LISTEN {
        bpf_sk_release(sk);
        return verdict_t::ESTABLISHED;
    }

    if !iph.is_null() && !tcp.is_null() {
        /* Kludge: we've run out of arguments, but need the length of the ip header. */
        let mut iphlen = size_of::<iphdr>() as u64;

        if tuplen == size_of_val_ipv6_tuple(tuple) as u64 {
            iphlen = size_of::<ipv6hdr>() as u64;
        }

        if bpf_tcp_check_syncookie(sk, iph, iphlen as __u32, tcp, size_of::<tcphdr>() as __u32) == 0 {
            bpf_sk_release(sk);
            return verdict_t::SYN_COOKIE;
        }
    }

    bpf_sk_release(sk);
    verdict_t::UNKNOWN
}

unsafe fn classify_udp(skb: *mut __sk_buff, tuple: *mut bpf_sock_tuple, tuplen: u64) -> verdict_t {
    let sk = bpf_sk_lookup_udp(skb, tuple, tuplen as __u32, BPF_F_CURRENT_NETNS, 0);

    if sk.is_null() {
        return verdict_t::UNKNOWN;
    }

    if (*sk).state == BPF_TCP_ESTABLISHED {
        bpf_sk_release(sk);
        return verdict_t::ESTABLISHED;
    }

    bpf_sk_release(sk);
    verdict_t::UNKNOWN
}

unsafe fn classify_icmp(
    skb: *mut __sk_buff,
    proto: u8,
    tuple: *mut bpf_sock_tuple,
    tuplen: u64,
    metrics: *mut metrics_t,
) -> verdict_t {
    match proto as i32 {
        IPPROTO_TCP => classify_tcp(skb, tuple, tuplen, ptr::null_mut(), ptr::null_mut()),
        IPPROTO_UDP => classify_udp(skb, tuple, tuplen),
        _ => {
            (*metrics).errors_total_malformed_icmp += 1;
            verdict_t::INVALID
        }
    }
}

unsafe fn process_icmpv4(
    skb: *mut __sk_buff,
    dynptr: *mut bpf_dynptr,
    offset: *mut __u64,
    metrics: *mut metrics_t,
) -> verdict_t {
    let mut icmp: icmphdr = core::mem::zeroed();
    let mut ipv4: iphdr = core::mem::zeroed();

    if bpf_dynptr_read(
        &mut icmp as *mut _ as *mut c_void,
        size_of::<icmphdr>() as __u32,
        dynptr,
        *offset,
        0,
    ) != 0
    {
        (*metrics).errors_total_malformed_icmp += 1;
        return verdict_t::INVALID;
    }

    *offset += size_of::<icmphdr>() as __u64;

    /* We should never receive encapsulated echo replies. */
    if icmp.type_ == ICMP_ECHOREPLY {
        (*metrics).errors_total_icmp_echo_replies += 1;
        return verdict_t::INVALID;
    }

    if icmp.type_ == ICMP_ECHO {
        return verdict_t::ECHO_REQUEST;
    }

    if icmp.type_ != ICMP_DEST_UNREACH || icmp.code != ICMP_FRAG_NEEDED {
        (*metrics).errors_total_unwanted_icmp += 1;
        return verdict_t::INVALID;
    }

    if pkt_parse_ipv4(dynptr, offset, &mut ipv4) != 0 {
        (*metrics).errors_total_malformed_icmp_pkt_too_big += 1;
        return verdict_t::INVALID;
    }

    /* The source address in the outer IP header is from the entity that
     * originated the ICMP message. Use the original IP header to restore
     * the correct flow tuple.
     */
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    tuple.ipv4.saddr = ipv4.daddr;
    tuple.ipv4.daddr = ipv4.saddr;

    if !pkt_parse_icmp_l4_ports(dynptr, offset, &mut tuple.ipv4.sport as *mut _ as *mut flow_ports_t)
    {
        (*metrics).errors_total_malformed_icmp_pkt_too_big += 1;
        return verdict_t::INVALID;
    }

    classify_icmp(skb, ipv4.protocol, &mut tuple, size_of_val_ipv4_tuple(&mut tuple) as u64, metrics)
}

unsafe fn process_icmpv6(
    dynptr: *mut bpf_dynptr,
    offset: *mut __u64,
    skb: *mut __sk_buff,
    metrics: *mut metrics_t,
) -> verdict_t {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let mut ipv6: ipv6hdr = core::mem::zeroed();
    let mut icmp6: icmp6hdr = core::mem::zeroed();
    let mut is_fragment = false;
    let mut l4_proto: u8 = 0;

    if bpf_dynptr_read(
        &mut icmp6 as *mut _ as *mut c_void,
        size_of::<icmp6hdr>() as __u32,
        dynptr,
        *offset,
        0,
    ) != 0
    {
        (*metrics).errors_total_malformed_icmp += 1;
        return verdict_t::INVALID;
    }

    /* We should never receive encapsulated echo replies. */
    if icmp6.icmp6_type == ICMPV6_ECHO_REPLY {
        (*metrics).errors_total_icmp_echo_replies += 1;
        return verdict_t::INVALID;
    }

    if icmp6.icmp6_type == ICMPV6_ECHO_REQUEST {
        return verdict_t::ECHO_REQUEST;
    }

    if icmp6.icmp6_type != ICMPV6_PKT_TOOBIG {
        (*metrics).errors_total_unwanted_icmp += 1;
        return verdict_t::INVALID;
    }

    if pkt_parse_ipv6(dynptr, offset, &mut ipv6, &mut l4_proto, &mut is_fragment) != 0 {
        (*metrics).errors_total_malformed_icmp_pkt_too_big += 1;
        return verdict_t::INVALID;
    }

    if is_fragment {
        (*metrics).errors_total_fragmented_ip += 1;
        return verdict_t::INVALID;
    }

    /* Swap source and dest addresses. */
    ptr::copy_nonoverlapping(
        &ipv6.daddr as *const _ as *const u8,
        &mut tuple.ipv6.saddr as *mut _ as *mut u8,
        size_of_val_ipv6_addr_tuple(&mut tuple, false),
    );
    ptr::copy_nonoverlapping(
        &ipv6.saddr as *const _ as *const u8,
        &mut tuple.ipv6.daddr as *mut _ as *mut u8,
        size_of_val_ipv6_addr_tuple(&mut tuple, true),
    );

    if !pkt_parse_icmp_l4_ports(dynptr, offset, &mut tuple.ipv6.sport as *mut _ as *mut flow_ports_t)
    {
        (*metrics).errors_total_malformed_icmp_pkt_too_big += 1;
        return verdict_t::INVALID;
    }

    classify_icmp(skb, l4_proto, &mut tuple, size_of_val_ipv6_tuple(&mut tuple) as u64, metrics)
}

unsafe fn process_tcp(
    dynptr: *mut bpf_dynptr,
    offset: *mut __u64,
    skb: *mut __sk_buff,
    info: *mut iphdr_info,
    metrics: *mut metrics_t,
) -> verdict_t {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let mut tcp: tcphdr = core::mem::zeroed();

    (*metrics).l4_protocol_packets_total_tcp += 1;

    if bpf_dynptr_read(
        &mut tcp as *mut _ as *mut c_void,
        size_of::<tcphdr>() as __u32,
        dynptr,
        *offset,
        0,
    ) != 0
    {
        (*metrics).errors_total_malformed_tcp += 1;
        return verdict_t::INVALID;
    }

    *offset += size_of::<tcphdr>() as __u64;

    if tcp.syn != 0 {
        return verdict_t::SYN;
    }

    let tuplen = fill_tuple(&mut tuple, (*info).hdr, (*info).len, tcp.source, tcp.dest);
    classify_tcp(skb, &mut tuple, tuplen, (*info).hdr, &mut tcp)
}

unsafe fn process_udp(
    dynptr: *mut bpf_dynptr,
    offset: *mut __u64,
    skb: *mut __sk_buff,
    info: *mut iphdr_info,
    metrics: *mut metrics_t,
) -> verdict_t {
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let mut udph: udphdr = core::mem::zeroed();

    (*metrics).l4_protocol_packets_total_udp += 1;

    if bpf_dynptr_read(
        &mut udph as *mut _ as *mut c_void,
        size_of::<udphdr>() as __u32,
        dynptr,
        *offset,
        0,
    ) != 0
    {
        (*metrics).errors_total_malformed_udp += 1;
        return verdict_t::INVALID;
    }
    *offset += size_of::<udphdr>() as __u64;

    let tuplen = fill_tuple(&mut tuple, (*info).hdr, (*info).len, udph.source, udph.dest);
    classify_udp(skb, &mut tuple, tuplen)
}

unsafe fn process_ipv4(
    skb: *mut __sk_buff,
    dynptr: *mut bpf_dynptr,
    offset: *mut __u64,
    metrics: *mut metrics_t,
) -> verdict_t {
    let mut ipv4: iphdr = core::mem::zeroed();
    let mut info = iphdr_info {
        hdr: &mut ipv4 as *mut _ as *mut c_void,
        len: size_of::<iphdr>() as u64,
    };

    (*metrics).l3_protocol_packets_total_ipv4 += 1;

    if pkt_parse_ipv4(dynptr, offset, &mut ipv4) != 0 {
        (*metrics).errors_total_malformed_ip += 1;
        return verdict_t::INVALID;
    }

    if ipv4.version != 4 {
        (*metrics).errors_total_malformed_ip += 1;
        return verdict_t::INVALID;
    }

    if ipv4_is_fragment(&ipv4) {
        (*metrics).errors_total_fragmented_ip += 1;
        return verdict_t::INVALID;
    }

    match ipv4.protocol as i32 {
        IPPROTO_ICMP => process_icmpv4(skb, dynptr, offset, metrics),
        IPPROTO_TCP => process_tcp(dynptr, offset, skb, &mut info, metrics),
        IPPROTO_UDP => process_udp(dynptr, offset, skb, &mut info, metrics),
        _ => {
            (*metrics).errors_total_unknown_l4_proto += 1;
            verdict_t::INVALID
        }
    }
}

unsafe fn process_ipv6(
    skb: *mut __sk_buff,
    dynptr: *mut bpf_dynptr,
    offset: *mut __u64,
    metrics: *mut metrics_t,
) -> verdict_t {
    let mut ipv6: ipv6hdr = core::mem::zeroed();
    let mut info = iphdr_info {
        hdr: &mut ipv6 as *mut _ as *mut c_void,
        len: size_of::<ipv6hdr>() as u64,
    };
    let mut l4_proto: u8 = 0;
    let mut is_fragment = false;

    (*metrics).l3_protocol_packets_total_ipv6 += 1;

    if pkt_parse_ipv6(dynptr, offset, &mut ipv6, &mut l4_proto, &mut is_fragment) != 0 {
        (*metrics).errors_total_malformed_ip += 1;
        return verdict_t::INVALID;
    }

    if ipv6.version != 6 {
        (*metrics).errors_total_malformed_ip += 1;
        return verdict_t::INVALID;
    }

    if is_fragment {
        (*metrics).errors_total_fragmented_ip += 1;
        return verdict_t::INVALID;
    }

    match l4_proto as i32 {
        IPPROTO_ICMPV6 => process_icmpv6(dynptr, offset, skb, metrics),
        IPPROTO_TCP => process_tcp(dynptr, offset, skb, &mut info, metrics),
        IPPROTO_UDP => process_udp(dynptr, offset, skb, &mut info, metrics),
        _ => {
            (*metrics).errors_total_unknown_l4_proto += 1;
            verdict_t::INVALID
        }
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn cls_redirect(skb: *mut __sk_buff) -> i32 {
    let mut encap_buffer = [0u8; size_of::<encap_headers_t>()];
    let mut dynptr: bpf_dynptr = core::mem::zeroed();
    let mut next_hop: in_addr = core::mem::zeroed();
    /* Tracks offset of the dynptr. This will be unnecessary once
     * bpf_dynptr_advance() is available.
     */
    let mut off: __u64 = 0;

    bpf_dynptr_from_skb(skb, 0, &mut dynptr);

    let metrics = get_global_metrics();
    if metrics.is_null() {
        return TC_ACT_SHOT;
    }

    (*metrics).processed_packets_total += 1;

    /* Pass bogus packets as long as we're not sure they're
     * destined for us.
     */
    if (*skb).protocol != bpf_htons(ETH_P_IP as u16) {
        return TC_ACT_OK;
    }

    /* Make sure that all encapsulation headers are available in
     * the linear portion of the skb. This makes it easy to manipulate them.
     */
    if bpf_skb_pull_data(skb, size_of::<encap_headers_t>() as u32) != 0 {
        return TC_ACT_OK;
    }

    let encap = bpf_dynptr_slice_rdwr(
        &mut dynptr,
        0,
        encap_buffer.as_mut_ptr() as *mut c_void,
        encap_buffer.len() as __u32,
    ) as *mut encap_headers_t;
    if encap.is_null() {
        return TC_ACT_OK;
    }

    off += size_of::<encap_headers_t>() as __u64;

    if (*encap).ip.ihl != 5 {
        /* We never have any options. */
        return TC_ACT_OK;
    }

    if (*encap).ip.daddr != ENCAPSULATION_IP || (*encap).ip.protocol as i32 != IPPROTO_UDP {
        return TC_ACT_OK;
    }

    /* TODO Check UDP length? */
    if (*encap).udp.dest != ENCAPSULATION_PORT {
        return TC_ACT_OK;
    }

    /* We now know that the packet is destined to us, we can
     * drop bogus ones.
     */
    if ipv4_is_fragment(&mut (*encap).ip) {
        (*metrics).errors_total_fragmented_ip += 1;
        return TC_ACT_SHOT;
    }

    if (*encap).gue.variant != 0 {
        (*metrics).errors_total_malformed_encapsulation += 1;
        return TC_ACT_SHOT;
    }

    if (*encap).gue.control != 0 {
        (*metrics).errors_total_malformed_encapsulation += 1;
        return TC_ACT_SHOT;
    }

    if (*encap).gue.flags != 0 {
        (*metrics).errors_total_malformed_encapsulation += 1;
        return TC_ACT_SHOT;
    }

    if (*encap).gue.hlen != (size_of::<uniguehdr>() / 4) as u8 + (*encap).unigue.hop_count {
        (*metrics).errors_total_malformed_encapsulation += 1;
        return TC_ACT_SHOT;
    }

    if (*encap).unigue.version != 0 {
        (*metrics).errors_total_malformed_encapsulation += 1;
        return TC_ACT_SHOT;
    }

    if (*encap).unigue.reserved != 0 {
        return TC_ACT_SHOT;
    }

    let mut maybe_ret = get_next_hop(&mut dynptr, &mut off, encap, &mut next_hop);
    if maybe_ret != CONTINUE_PROCESSING {
        return maybe_ret;
    }

    if next_hop.s_addr == 0 {
        (*metrics).accepted_packets_total_last_hop += 1;
        return accept_locally(skb, encap);
    }

    let verdict = match (*encap).gue.proto_ctype as i32 {
        IPPROTO_IPIP => process_ipv4(skb, &mut dynptr, &mut off, metrics),
        IPPROTO_IPV6 => process_ipv6(skb, &mut dynptr, &mut off, metrics),
        _ => {
            (*metrics).errors_total_unknown_l3_proto += 1;
            return TC_ACT_SHOT;
        }
    };

    match verdict {
        verdict_t::INVALID => {
            /* metrics have already been bumped */
            return TC_ACT_SHOT;
        }
        verdict_t::UNKNOWN => {
            return forward_to_next_hop(skb, &mut dynptr, encap, &mut next_hop, metrics);
        }
        verdict_t::ECHO_REQUEST => {
            (*metrics).accepted_packets_total_icmp_echo_request += 1;
        }
        verdict_t::SYN => {
            if (*encap).unigue.forward_syn != 0 {
                return forward_to_next_hop(skb, &mut dynptr, encap, &mut next_hop, metrics);
            }

            (*metrics).accepted_packets_total_syn += 1;
        }
        verdict_t::SYN_COOKIE => {
            (*metrics).accepted_packets_total_syn_cookies += 1;
        }
        verdict_t::ESTABLISHED => {
            (*metrics).accepted_packets_total_established += 1;
        }
    }

    maybe_ret = accept_locally(skb, encap);

    if encap == encap_buffer.as_mut_ptr() as *mut encap_headers_t {
        bpf_dynptr_write(
            &mut dynptr,
            0,
            encap_buffer.as_ptr() as *const c_void,
            encap_buffer.len() as __u32,
            0,
        );
    }

    maybe_ret
}

fn size_of_val_ipv4_tuple(_tuple: *mut bpf_sock_tuple) -> usize {
    size_of::<bpf_sock_tuple_ipv4>()
}

fn size_of_val_ipv6_tuple(_tuple: *mut bpf_sock_tuple) -> usize {
    size_of::<bpf_sock_tuple_ipv6>()
}

fn size_of_val_ipv6_addr_tuple(_tuple: *mut bpf_sock_tuple, _daddr: bool) -> usize {
    size_of::<in6_addr>()
}

extern "C" {
    fn bpf_dynptr_read(dst: *mut c_void, len: __u32, src: *mut bpf_dynptr, offset: __u64, flags: __u64)
        -> i32;
    fn bpf_dynptr_write(
        dst: *mut bpf_dynptr,
        offset: __u64,
        src: *const c_void,
        len: __u32,
        flags: __u64,
    ) -> i32;
    fn bpf_dynptr_from_skb(skb: *mut __sk_buff, flags: __u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_slice_rdwr(
        ptr: *mut bpf_dynptr,
        offset: __u64,
        buffer: *mut c_void,
        buffer__sz: __u32,
    ) -> *mut c_void;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *mut c_void) -> *mut c_void;
    fn bpf_skb_adjust_room(skb: *mut __sk_buff, len_diff: i32, mode: u32, flags: u64) -> i32;
    fn bpf_csum_level(skb: *mut __sk_buff, level: u64) -> i32;
    fn bpf_redirect(ifindex: u32, flags: u64) -> i32;
    fn bpf_skb_load_bytes(skb: *mut __sk_buff, offset: i32, to: *mut c_void, len: u32) -> i32;
    fn bpf_skb_store_bytes(
        skb: *mut __sk_buff,
        offset: i32,
        from: *mut c_void,
        len: u32,
        flags: u64,
    ) -> i32;
    fn bpf_l3_csum_replace(skb: *mut __sk_buff, offset: i32, from: u64, to: u64, size: u64)
        -> i32;
    fn bpf_check_mtu(
        skb: *mut __sk_buff,
        ifindex: u32,
        mtu_len: *mut u32,
        len_diff: i32,
        flags: u64,
    ) -> i32;
    fn bpf_skb_pull_data(skb: *mut __sk_buff, len: u32) -> i32;
    fn bpf_skc_lookup_tcp(
        skb: *mut __sk_buff,
        tuple: *mut bpf_sock_tuple,
        tuple_size: __u32,
        netns: __u64,
        flags: __u64,
    ) -> *mut bpf_sock;
    fn bpf_sk_lookup_udp(
        skb: *mut __sk_buff,
        tuple: *mut bpf_sock_tuple,
        tuple_size: __u32,
        netns: __u64,
        flags: __u64,
    ) -> *mut bpf_sock;
    fn bpf_sk_release(sk: *mut bpf_sock);
    fn bpf_tcp_check_syncookie(
        sk: *mut bpf_sock,
        iph: *mut c_void,
        iphlen: __u32,
        th: *mut tcphdr,
        thlen: __u32,
    ) -> i64;
}

extern "Rust" {
    fn bpf_htons(x: u16) -> u16;
    fn bpf_ntohs(x: u16) -> u16;
}

// External kernel/BPF/test header items referenced by the translated source.
extern "Rust" {
    type bpf_dynptr;
    type __sk_buff;
    type bpf_sock;
    type bpf_sock_tuple;
    type bpf_sock_tuple_ipv4;
    type bpf_sock_tuple_ipv6;
    type bpf_map_def;
    type iphdr;
    type ipv6hdr;
    type icmphdr;
    type icmp6hdr;
    type tcphdr;
    type udphdr;
    type ethhdr;
    type in_addr;
    type in6_addr;
    type gre_base_hdr;
    type encap_headers_t;
    type encap_gre_t;
    type uniguehdr;

    static BPF_MAP_TYPE_PERCPU_ARRAY: u32;
    static BPF_ADJ_ROOM_MAC: u32;
    static BPF_ADJ_ROOM_NET: u32;
    static BPF_F_ADJ_ROOM_FIXED_GSO: u64;
    static BPF_F_ADJ_ROOM_NO_CSUM_RESET: u64;
    static BPF_CSUM_LEVEL_DEC: u64;
    static BPF_CSUM_LEVEL_INC: u64;
    static BPF_F_INGRESS: u64;
    static BPF_F_CURRENT_NETNS: u64;
    static BPF_TCP_LISTEN: u32;
    static BPF_TCP_ESTABLISHED: u32;

    static ETH_ALEN: u32;
    static ETH_P_IP: u32;
    static ETH_P_IPV6: u32;
    static TC_ACT_OK: i32;
    static TC_ACT_SHOT: i32;

    static IPPROTO_FRAGMENT: i32;
    static IPPROTO_HOPOPTS: i32;
    static IPPROTO_ROUTING: i32;
    static IPPROTO_DSTOPTS: i32;
    static IPPROTO_MH: i32;
    static IPPROTO_IPV6: i32;
    static IPPROTO_GRE: i32;
    static IPPROTO_IPIP: i32;
    static IPPROTO_ICMP: i32;
    static IPPROTO_ICMPV6: i32;
    static IPPROTO_TCP: i32;
    static IPPROTO_UDP: i32;

    static ICMP_ECHOREPLY: u8;
    static ICMP_ECHO: u8;
    static ICMP_DEST_UNREACH: u8;
    static ICMP_FRAG_NEEDED: u8;
    static ICMPV6_ECHO_REPLY: u8;
    static ICMPV6_ECHO_REQUEST: u8;
    static ICMPV6_PKT_TOOBIG: u8;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
