// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2019, 2020 Cloudflare

// Rust translation of testing/selftests/bpf/progs/test_cls_redirect.c.
// C include dependencies are represented here as external types, constants,
// helper declarations, and layout-compatible structs where this file uses them.
// #pragma GCC diagnostic ignored "-Waddress-of-packed-member"
// SUBPROGS selected __noinline in C, otherwise __always_inline.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type __be16 = u16;
type __be32 = u32;
type ret_t = i32;
type net_ptr = *mut u8;

const IP_OFFSET_MASK: u16 = 0x1FFF;
const IP_MF: u16 = 0x2000;

const ETH_ALEN: usize = 6;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86DD;

const IPPROTO_IPIP: u8 = 4;
const IPPROTO_TCP: u8 = 6;
const IPPROTO_UDP: u8 = 17;
const IPPROTO_IPV6: u8 = 41;
const IPPROTO_ROUTING: u8 = 43;
const IPPROTO_FRAGMENT: u8 = 44;
const IPPROTO_GRE: u8 = 47;
const IPPROTO_ICMPV6: u8 = 58;
const IPPROTO_NONE: u8 = 59;
const IPPROTO_DSTOPTS: u8 = 60;
const IPPROTO_MH: u8 = 135;
const IPPROTO_ICMP: u8 = 1;
const IPPROTO_HOPOPTS: u8 = 0;

const ICMP_ECHOREPLY: u8 = 0;
const ICMP_DEST_UNREACH: u8 = 3;
const ICMP_ECHO: u8 = 8;
const ICMP_FRAG_NEEDED: u8 = 4;
const ICMPV6_PKT_TOOBIG: u8 = 2;
const ICMPV6_ECHO_REQUEST: u8 = 128;
const ICMPV6_ECHO_REPLY: u8 = 129;

const BPF_MAP_TYPE_PERCPU_ARRAY: u32 = 6;
const BPF_ADJ_ROOM_MAC: u32 = 1;
const BPF_ADJ_ROOM_NET: u32 = 0;
const BPF_F_ADJ_ROOM_FIXED_GSO: u64 = 1;
const BPF_F_ADJ_ROOM_NO_CSUM_RESET: u64 = 1 << 1;
const BPF_CSUM_LEVEL_INC: u64 = 1;
const BPF_CSUM_LEVEL_DEC: u64 = 2;
const BPF_F_INGRESS: u64 = 1;
const BPF_F_CURRENT_NETNS: u64 = !0u32 as u64;
const BPF_TCP_LISTEN: u32 = 10;
const BPF_TCP_ESTABLISHED: u32 = 1;

const TC_ACT_OK: ret_t = 0;
const TC_ACT_SHOT: ret_t = 2;

#[used]
#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 13] = *b"Dual BSD/GPL\0";

/**
 * Destination port and IP used for UDP encapsulation.
 */
#[no_mangle]
pub static ENCAPSULATION_PORT: __be16 = 0;
#[no_mangle]
pub static ENCAPSULATION_IP: __be32 = 0;

#[repr(C)]
pub struct metrics_t {
    processed_packets_total: u64,
    l3_protocol_packets_total_ipv4: u64,
    l3_protocol_packets_total_ipv6: u64,
    l4_protocol_packets_total_tcp: u64,
    l4_protocol_packets_total_udp: u64,
    accepted_packets_total_syn: u64,
    accepted_packets_total_syn_cookies: u64,
    accepted_packets_total_last_hop: u64,
    accepted_packets_total_icmp_echo_request: u64,
    accepted_packets_total_established: u64,
    forwarded_packets_total_gue: u64,
    forwarded_packets_total_gre: u64,

    errors_total_unknown_l3_proto: u64,
    errors_total_unknown_l4_proto: u64,
    errors_total_malformed_ip: u64,
    errors_total_fragmented_ip: u64,
    errors_total_malformed_icmp: u64,
    errors_total_unwanted_icmp: u64,
    errors_total_malformed_icmp_pkt_too_big: u64,
    errors_total_malformed_tcp: u64,
    errors_total_malformed_udp: u64,
    errors_total_icmp_echo_replies: u64,
    errors_total_malformed_encapsulation: u64,
    errors_total_encap_adjust_failed: u64,
    errors_total_encap_buffer_too_small: u64,
    errors_total_redirect_loop: u64,
    errors_total_encap_mtu_violate: u64,
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
    src: u16,
    dst: u16,
}

// C static asserts:
// flow_ports_t must match sport and dport in struct bpf_sock_tuple ipv4/ipv6.

/* This is a bit of a hack. We need a return value which allows us to
 * indicate that the regular flow of the program should continue,
 * while allowing functions to use XDP_PASS and XDP_DROP, etc.
 */
static CONTINUE_PROCESSING: ret_t = -1;

macro_rules! MAYBE_RETURN {
    ($x:expr) => {{
        let __ret: ret_t = $x;
        if __ret != CONTINUE_PROCESSING {
            return __ret;
        }
    }};
}

#[repr(C)]
pub struct __sk_buff {
    len: u32,
    pkt_type: u32,
    mark: u32,
    queue_mapping: u32,
    protocol: u32,
    vlan_present: u32,
    vlan_tci: u32,
    vlan_proto: u32,
    priority: u32,
    ingress_ifindex: u32,
    ifindex: u32,
    tc_index: u32,
    cb: [u32; 5],
    hash: u32,
    tc_classid: u32,
    data: u32,
    data_end: u32,
}

#[repr(C)]
pub struct bpf_sock {
    bound_dev_if: u32,
    family: u32,
    type_: u32,
    protocol: u32,
    mark: u32,
    priority: u32,
    src_ip4: u32,
    src_ip6: [u32; 4],
    src_port: u32,
    dst_port: u16,
    dst_ip4: u32,
    dst_ip6: [u32; 4],
    state: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sock_tuple_ipv4 {
    saddr: u32,
    daddr: u32,
    sport: u16,
    dport: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_sock_tuple_ipv6 {
    saddr: [u32; 4],
    daddr: [u32; 4],
    sport: u16,
    dport: u16,
}

#[repr(C)]
pub union bpf_sock_tuple {
    ipv4: bpf_sock_tuple_ipv4,
    ipv6: bpf_sock_tuple_ipv6,
}

#[repr(C)]
pub struct in_addr {
    s_addr: u32,
}

#[repr(C, packed)]
pub struct ethhdr {
    h_dest: [u8; ETH_ALEN],
    h_source: [u8; ETH_ALEN],
    h_proto: __be16,
}

#[repr(C, packed)]
pub struct iphdr {
    ihl_version: u8,
    tos: u8,
    tot_len: __be16,
    id: __be16,
    frag_off: __be16,
    ttl: u8,
    protocol: u8,
    check: __be16,
    saddr: __be32,
    daddr: __be32,
}

impl iphdr {
    unsafe fn ihl(&self) -> u8 {
        self.ihl_version & 0x0f
    }

    unsafe fn version(&self) -> u8 {
        self.ihl_version >> 4
    }
}

#[repr(C, packed)]
pub struct ipv6hdr {
    priority_version: u8,
    flow_lbl: [u8; 3],
    payload_len: __be16,
    nexthdr: u8,
    hop_limit: u8,
    saddr: [u32; 4],
    daddr: [u32; 4],
}

impl ipv6hdr {
    unsafe fn version(&self) -> u8 {
        self.priority_version >> 4
    }
}

#[repr(C, packed)]
pub struct tcphdr {
    source: __be16,
    dest: __be16,
    seq: u32,
    ack_seq: u32,
    doff_res_flags: u16,
    window: u16,
    check: u16,
    urg_ptr: u16,
}

impl tcphdr {
    unsafe fn syn(&self) -> bool {
        bpf_ntohs(self.doff_res_flags) & 0x0002 != 0
    }
}

#[repr(C, packed)]
pub struct udphdr {
    source: __be16,
    dest: __be16,
    len: __be16,
    check: __be16,
}

#[repr(C, packed)]
pub struct icmphdr {
    type_: u8,
    code: u8,
    checksum: u16,
    rest: u32,
}

#[repr(C, packed)]
pub struct icmp6hdr {
    icmp6_type: u8,
    icmp6_code: u8,
    icmp6_cksum: u16,
    icmp6_dataun: u32,
}

#[repr(C, packed)]
pub struct guehdr {
    first: u8,
    hlen: u8,
    proto_ctype: u8,
    flags: u8,
}

impl guehdr {
    unsafe fn variant(&self) -> u8 {
        self.first >> 6
    }

    unsafe fn control(&self) -> u8 {
        (self.first >> 5) & 1
    }
}

#[repr(C, packed)]
pub struct uniguehdr {
    first: u8,
    hop_count: u8,
    next_hop: u8,
    flags: u8,
}

impl uniguehdr {
    unsafe fn version(&self) -> u8 {
        self.first >> 4
    }

    unsafe fn reserved(&self) -> u8 {
        self.first & 0x0f
    }

    unsafe fn last_hop_gre(&self) -> bool {
        self.flags & 0x01 != 0
    }

    unsafe fn forward_syn(&self) -> bool {
        self.flags & 0x02 != 0
    }
}

#[repr(C, packed)]
pub struct gre_base_hdr {
    flags: __be16,
    protocol: __be16,
}

#[repr(C, packed)]
pub struct encap_headers_t {
    eth: ethhdr,
    ip: iphdr,
    udp: udphdr,
    gue: guehdr,
    unigue: uniguehdr,
}

#[repr(C, packed)]
pub struct encap_gre_t {
    eth: ethhdr,
    ip: iphdr,
    gre: gre_base_hdr,
}

/* Linux packet pointers are either aligned to NET_IP_ALIGN (aka 2 bytes),
 * or not aligned if the arch supports efficient unaligned access.
 *
 * Since the verifier ensures that eBPF packet accesses follow these rules,
 * we can tell LLVM to emit code as if we always had a larger alignment.
 * It will yell at us if we end up on a platform where this is not valid.
 */
#[repr(C)]
pub struct buf_t {
    skb: *mut __sk_buff,
    head: net_ptr,
    /* NB: tail mustn't have alignment other than 1, otherwise
     * LLVM will go and eliminate code, e.g. when checking packet lengths.
     */
    tail: *const u8,
}

#[repr(C)]
pub struct metrics_map_def {
    type_: u32,
    max_entries: u32,
}

/* Global metrics, per CPU
 */
#[used]
#[no_mangle]
#[link_section = ".maps"]
pub static metrics_map: metrics_map_def = metrics_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: 1,
};

extern "C" {
    fn bpf_skb_load_bytes(skb: *mut __sk_buff, off: u32, to: *mut c_void, len: u32) -> i32;
    fn bpf_skb_store_bytes(
        skb: *mut __sk_buff,
        off: u32,
        from: *const c_void,
        len: u32,
        flags: u64,
    ) -> i32;
    fn bpf_skb_adjust_room(skb: *mut __sk_buff, len_diff: i32, mode: u32, flags: u64) -> i32;
    fn bpf_csum_level(skb: *mut __sk_buff, level: u64) -> i32;
    fn bpf_redirect(ifindex: u32, flags: u64) -> i32;
    fn bpf_check_mtu(
        skb: *mut __sk_buff,
        ifindex: u32,
        mtu_len: *mut u32,
        len_diff: i32,
        flags: u64,
    ) -> i32;
    fn bpf_skb_pull_data(skb: *mut __sk_buff, len: u32) -> i32;
    fn bpf_l3_csum_replace(
        skb: *mut __sk_buff,
        off: u64,
        from: u64,
        to: u64,
        size: u64,
    ) -> i32;
    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut c_void;
    fn bpf_skc_lookup_tcp(
        skb: *mut __sk_buff,
        tuple: *mut bpf_sock_tuple,
        tuple_size: u32,
        netns: u64,
        flags: u64,
    ) -> *mut bpf_sock;
    fn bpf_sk_lookup_udp(
        skb: *mut __sk_buff,
        tuple: *mut bpf_sock_tuple,
        tuple_size: u32,
        netns: u64,
        flags: u64,
    ) -> *mut bpf_sock;
    fn bpf_sk_release(sk: *mut bpf_sock);
    fn bpf_tcp_check_syncookie(
        sk: *mut bpf_sock,
        iph: *mut c_void,
        iphlen: u32,
        th: *mut tcphdr,
        thlen: u32,
    ) -> i32;
}

#[inline(always)]
fn bpf_htons(x: u16) -> u16 {
    x.to_be()
}

#[inline(always)]
fn bpf_ntohs(x: u16) -> u16 {
    u16::from_be(x)
}

#[inline(always)]
unsafe fn buf_off(buf: *const buf_t) -> usize {
    /* Clang seems to optimize constructs like
     *    a - b + c
     * if c is known:
     *    r? = c
     *    r? -= b
     *    r? += a
     *
     * This is a problem if a and b are packet pointers,
     * since the verifier allows subtracting two pointers to
     * get a scalar, but not a scalar and a pointer.
     *
     * Use inline asm to break this optimization.
     */
    ((*buf).head as usize).wrapping_sub((*(*buf).skb).data as usize)
}

#[inline(always)]
unsafe fn buf_copy(buf: *mut buf_t, dst: *mut c_void, len: usize) -> bool {
    if bpf_skb_load_bytes((*buf).skb, buf_off(buf), dst, len as u32) != 0 {
        return false;
    }

    (*buf).head = (*buf).head.add(len);
    true
}

#[inline(always)]
unsafe fn buf_skip(buf: *mut buf_t, len: usize) -> bool {
    /* Check whether off + len is valid in the non-linear part. */
    if buf_off(buf).wrapping_add(len) > (*(*buf).skb).len as usize {
        return false;
    }

    (*buf).head = (*buf).head.add(len);
    true
}

/* Returns a pointer to the start of buf, or NULL if len is
 * larger than the remaining data. Consumes len bytes on a successful
 * call.
 *
 * If scratch is not NULL, the function will attempt to load non-linear
 * data via bpf_skb_load_bytes. On success, scratch is returned.
 */
#[inline(always)]
unsafe fn buf_assign(buf: *mut buf_t, len: usize, scratch: *mut c_void) -> *mut c_void {
    if ((*buf).head as usize).wrapping_add(len) > (*buf).tail as usize {
        if scratch.is_null() {
            return ptr::null_mut();
        }

        return if buf_copy(buf, scratch, len) {
            scratch
        } else {
            ptr::null_mut()
        };
    }

    let ptr = (*buf).head as *mut c_void;
    (*buf).head = (*buf).head.add(len);
    ptr
}

unsafe fn pkt_skip_ipv4_options(buf: *mut buf_t, ipv4: *const iphdr) -> bool {
    if (*ipv4).ihl() <= 5 {
        return true;
    }

    buf_skip(buf, ((*ipv4).ihl() - 5) as usize * 4)
}

unsafe fn ipv4_is_fragment(ip: *const iphdr) -> bool {
    let frag_off = (*ip).frag_off & bpf_htons(IP_OFFSET_MASK);
    ((*ip).frag_off & bpf_htons(IP_MF)) != 0 || frag_off > 0
}

#[inline(always)]
unsafe fn pkt_parse_ipv4(pkt: *mut buf_t, scratch: *mut iphdr) -> *mut iphdr {
    let ipv4 = buf_assign(pkt, size_of::<iphdr>(), scratch as *mut c_void) as *mut iphdr;
    if ipv4.is_null() {
        return ptr::null_mut();
    }

    if (*ipv4).ihl() < 5 {
        return ptr::null_mut();
    }

    if !pkt_skip_ipv4_options(pkt, ipv4) {
        return ptr::null_mut();
    }

    ipv4
}

/* Parse the L4 ports from a packet, assuming a layout like TCP or UDP. */
unsafe fn pkt_parse_icmp_l4_ports(pkt: *mut buf_t, ports: *mut flow_ports_t) -> bool {
    if !buf_copy(pkt, ports as *mut c_void, size_of::<flow_ports_t>()) {
        return false;
    }

    /* Ports in the L4 headers are reversed, since we are parsing an ICMP
     * payload which is going towards the eyeball.
     */
    let dst = (*ports).src;
    (*ports).src = (*ports).dst;
    (*ports).dst = dst;
    true
}

unsafe fn pkt_checksum_fold(mut csum: u32) -> u16 {
    /* The highest reasonable value for an IPv4 header
     * checksum requires two folds, so we just do that always.
     */
    csum = (csum & 0xffff).wrapping_add(csum >> 16);
    csum = (csum & 0xffff).wrapping_add(csum >> 16);
    !(csum as u16)
}

unsafe fn pkt_ipv4_checksum(iph: *mut iphdr) {
    (*iph).check = 0;

    /* An IP header without options is 20 bytes. Two of those
     * are the checksum, which we always set to zero. Hence,
     * the maximum accumulated value is 18 / 2 * 0xffff = 0x8fff7,
     * which fits in 32 bit.
     */
    // C static assert: sizeof(struct iphdr) == 20.
    let mut acc: u32 = 0;
    let ipw = iph as *const u16;

    for i in 0..(size_of::<iphdr>() / 2) {
        acc = acc.wrapping_add(*ipw.add(i) as u32);
    }

    (*iph).check = pkt_checksum_fold(acc);
}

#[repr(C)]
struct ipv6_exthdr {
    next: u8,
    len: u8,
}

unsafe fn pkt_skip_ipv6_extension_headers(
    pkt: *mut buf_t,
    ipv6: *const ipv6hdr,
    upper_proto: *mut u8,
    is_fragment: *mut bool,
) -> bool {
    /* We understand five extension headers.
     * https://tools.ietf.org/html/rfc8200#section-4.1 states that all
     * headers should occur once, except Destination Options, which may
     * occur twice. Hence we give up after 6 headers.
     */
    let mut exthdr = ipv6_exthdr {
        next: (*ipv6).nexthdr,
        len: 0,
    };
    *is_fragment = false;

    for _i in 0..6 {
        match exthdr.next {
            IPPROTO_FRAGMENT => {
                *is_fragment = true;
                /* NB: We don't check that hdrlen == 0 as per spec. */
                if !buf_copy(pkt, &mut exthdr as *mut _ as *mut c_void, size_of::<ipv6_exthdr>()) {
                    return false;
                }

                /* hdrlen is in 8-octet units, and excludes the first 8 octets. */
                if !buf_skip(pkt, (exthdr.len as usize + 1) * 8 - size_of::<ipv6_exthdr>()) {
                    return false;
                }
            }
            IPPROTO_HOPOPTS | IPPROTO_ROUTING | IPPROTO_DSTOPTS | IPPROTO_MH => {
                if !buf_copy(pkt, &mut exthdr as *mut _ as *mut c_void, size_of::<ipv6_exthdr>()) {
                    return false;
                }

                /* hdrlen is in 8-octet units, and excludes the first 8 octets. */
                if !buf_skip(pkt, (exthdr.len as usize + 1) * 8 - size_of::<ipv6_exthdr>()) {
                    return false;
                }
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
    }

    /* We never found an upper layer header. */
    false
}

/* This function has to be inlined, because the verifier otherwise rejects it
 * due to returning a pointer to the stack. This is technically correct, since
 * scratch is allocated on the stack. However, this usage should be safe since
 * it's the callers stack after all.
 */
#[inline(always)]
unsafe fn pkt_parse_ipv6(
    pkt: *mut buf_t,
    scratch: *mut ipv6hdr,
    proto: *mut u8,
    is_fragment: *mut bool,
) -> *mut ipv6hdr {
    let ipv6 = buf_assign(pkt, size_of::<ipv6hdr>(), scratch as *mut c_void) as *mut ipv6hdr;
    if ipv6.is_null() {
        return ptr::null_mut();
    }

    if !pkt_skip_ipv6_extension_headers(pkt, ipv6, proto, is_fragment) {
        return ptr::null_mut();
    }

    ipv6
}

unsafe fn get_global_metrics() -> *mut metrics_t {
    let key: u64 = 0;
    bpf_map_lookup_elem(
        &metrics_map as *const _ as *const c_void,
        &key as *const _ as *const c_void,
    ) as *mut metrics_t
}

unsafe fn accept_locally(skb: *mut __sk_buff, encap: *mut encap_headers_t) -> ret_t {
    let payload_off =
        size_of::<encap_headers_t>() + size_of::<in_addr>() * (*encap).unigue.hop_count as usize;
    let encap_overhead = payload_off as i32 - size_of::<ethhdr>() as i32;

    // Changing the ethertype if the encapsulated packet is ipv6
    if (*encap).gue.proto_ctype == IPPROTO_IPV6 {
        (*encap).eth.h_proto = bpf_htons(ETH_P_IPV6);
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
    encap: *mut encap_headers_t,
    next_hop: *mut in_addr,
    metrics: *mut metrics_t,
) -> ret_t {
    (*metrics).forwarded_packets_total_gre = (*metrics).forwarded_packets_total_gre.wrapping_add(1);

    let payload_off =
        size_of::<encap_headers_t>() + size_of::<in_addr>() * (*encap).unigue.hop_count as usize;
    let encap_overhead =
        payload_off as i32 - size_of::<ethhdr>() as i32 - size_of::<iphdr>() as i32;
    let delta = size_of::<gre_base_hdr>() as i32 - encap_overhead;
    let mut proto: u16 = ETH_P_IP;
    let mut mtu_len: u32 = 0;

    /* Loop protection: the inner packet's TTL is decremented as a safeguard
     * against any forwarding loop. As the only interesting field is the TTL
     * hop limit for IPv6, it is easier to use bpf_skb_load_bytes/bpf_skb_store_bytes
     * as they handle the split packets if needed (no need for the data to be
     * in the linear section).
     */
    if (*encap).gue.proto_ctype == IPPROTO_IPV6 {
        proto = ETH_P_IPV6;
        let mut ttl: u8 = 0;
        let mut rc: i32;

        rc = bpf_skb_load_bytes(
            skb,
            payload_off as u32 + 7,
            &mut ttl as *mut _ as *mut c_void,
            1,
        );
        if rc != 0 {
            (*metrics).errors_total_malformed_encapsulation =
                (*metrics).errors_total_malformed_encapsulation.wrapping_add(1);
            return TC_ACT_SHOT;
        }

        if ttl == 0 {
            (*metrics).errors_total_redirect_loop =
                (*metrics).errors_total_redirect_loop.wrapping_add(1);
            return TC_ACT_SHOT;
        }

        ttl = ttl.wrapping_sub(1);
        rc = bpf_skb_store_bytes(
            skb,
            payload_off as u32 + 7,
            &ttl as *const _ as *const c_void,
            1,
            0,
        );
        if rc != 0 {
            (*metrics).errors_total_malformed_encapsulation =
                (*metrics).errors_total_malformed_encapsulation.wrapping_add(1);
            return TC_ACT_SHOT;
        }
    } else {
        let mut ttl: u8 = 0;
        let mut rc: i32;

        rc = bpf_skb_load_bytes(
            skb,
            payload_off as u32 + 8,
            &mut ttl as *mut _ as *mut c_void,
            1,
        );
        if rc != 0 {
            (*metrics).errors_total_malformed_encapsulation =
                (*metrics).errors_total_malformed_encapsulation.wrapping_add(1);
            return TC_ACT_SHOT;
        }

        if ttl == 0 {
            (*metrics).errors_total_redirect_loop =
                (*metrics).errors_total_redirect_loop.wrapping_add(1);
            return TC_ACT_SHOT;
        }

        /* IPv4 also has a checksum to patch. While the TTL is only one byte,
         * this function only works for 2 and 4 bytes arguments (the result is
         * the same).
         */
        rc = bpf_l3_csum_replace(
            skb,
            payload_off as u64 + 10,
            ttl as u64,
            ttl.wrapping_sub(1) as u64,
            2,
        );
        if rc != 0 {
            (*metrics).errors_total_malformed_encapsulation =
                (*metrics).errors_total_malformed_encapsulation.wrapping_add(1);
            return TC_ACT_SHOT;
        }

        ttl = ttl.wrapping_sub(1);
        rc = bpf_skb_store_bytes(
            skb,
            payload_off as u32 + 8,
            &ttl as *const _ as *const c_void,
            1,
            0,
        );
        if rc != 0 {
            (*metrics).errors_total_malformed_encapsulation =
                (*metrics).errors_total_malformed_encapsulation.wrapping_add(1);
            return TC_ACT_SHOT;
        }
    }

    if bpf_check_mtu(skb, (*skb).ifindex, &mut mtu_len, delta, 0) != 0 {
        (*metrics).errors_total_encap_mtu_violate =
            (*metrics).errors_total_encap_mtu_violate.wrapping_add(1);
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
        (*metrics).errors_total_encap_adjust_failed =
            (*metrics).errors_total_encap_adjust_failed.wrapping_add(1);
        return TC_ACT_SHOT;
    }

    if bpf_skb_pull_data(skb, size_of::<encap_gre_t>() as u32) != 0 {
        (*metrics).errors_total_encap_buffer_too_small =
            (*metrics).errors_total_encap_buffer_too_small.wrapping_add(1);
        return TC_ACT_SHOT;
    }

    let mut pkt = buf_t {
        skb,
        head: (*skb).data as usize as *mut u8,
        tail: (*skb).data_end as usize as *mut u8,
    };

    let encap_gre = buf_assign(&mut pkt, size_of::<encap_gre_t>(), ptr::null_mut()) as *mut encap_gre_t;
    if encap_gre.is_null() {
        (*metrics).errors_total_encap_buffer_too_small =
            (*metrics).errors_total_encap_buffer_too_small.wrapping_add(1);
        return TC_ACT_SHOT;
    }

    (*encap_gre).ip.protocol = IPPROTO_GRE;
    (*encap_gre).ip.daddr = (*next_hop).s_addr;
    (*encap_gre).ip.saddr = ENCAPSULATION_IP;
    (*encap_gre).ip.tot_len = bpf_htons(bpf_ntohs((*encap_gre).ip.tot_len).wrapping_add(delta as u16));
    (*encap_gre).gre.flags = 0;
    (*encap_gre).gre.protocol = bpf_htons(proto);
    pkt_ipv4_checksum(&mut (*encap_gre).ip as *mut iphdr);

    bpf_redirect((*skb).ifindex, 0)
}

unsafe fn forward_to_next_hop(
    skb: *mut __sk_buff,
    encap: *mut encap_headers_t,
    next_hop: *mut in_addr,
    metrics: *mut metrics_t,
) -> ret_t {
    /* swap L2 addresses */
    /* This assumes that packets are received from a router.
     * So just swapping the MAC addresses here will make the packet go back to
     * the router, which will send it to the appropriate machine.
     */
    let mut temp = [0u8; ETH_ALEN];
    ptr::copy_nonoverlapping((*encap).eth.h_dest.as_ptr(), temp.as_mut_ptr(), temp.len());
    ptr::copy_nonoverlapping((*encap).eth.h_source.as_ptr(), (*encap).eth.h_dest.as_mut_ptr(), ETH_ALEN);
    ptr::copy_nonoverlapping(temp.as_ptr(), (*encap).eth.h_source.as_mut_ptr(), ETH_ALEN);

    if (*encap).unigue.next_hop == (*encap).unigue.hop_count.wrapping_sub(1)
        && (*encap).unigue.last_hop_gre()
    {
        return forward_with_gre(skb, encap, next_hop, metrics);
    }

    (*metrics).forwarded_packets_total_gue = (*metrics).forwarded_packets_total_gue.wrapping_add(1);
    let old_saddr = (*encap).ip.saddr;
    (*encap).ip.saddr = (*encap).ip.daddr;
    (*encap).ip.daddr = (*next_hop).s_addr;
    if (*encap).unigue.next_hop < (*encap).unigue.hop_count {
        (*encap).unigue.next_hop = (*encap).unigue.next_hop.wrapping_add(1);
    }

    /* Remove ip->saddr, add next_hop->s_addr */
    let off = size_of::<ethhdr>() + 10;
    let ret = bpf_l3_csum_replace(skb, off as u64, old_saddr as u64, (*next_hop).s_addr as u64, 4);
    if ret < 0 {
        return TC_ACT_SHOT;
    }

    bpf_redirect((*skb).ifindex, 0)
}

unsafe fn skip_next_hops(pkt: *mut buf_t, n: i32) -> ret_t {
    match n {
        1 => {
            if !buf_skip(pkt, size_of::<in_addr>()) {
                return TC_ACT_SHOT;
            }
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
unsafe fn get_next_hop(pkt: *mut buf_t, encap: *mut encap_headers_t, next_hop: *mut in_addr) -> ret_t {
    if (*encap).unigue.next_hop > (*encap).unigue.hop_count {
        return TC_ACT_SHOT;
    }

    /* Skip "used" next hops. */
    MAYBE_RETURN!(skip_next_hops(pkt, (*encap).unigue.next_hop as i32));

    if (*encap).unigue.next_hop == (*encap).unigue.hop_count {
        /* No more next hops, we are at the end of the GLB header. */
        (*next_hop).s_addr = 0;
        return CONTINUE_PROCESSING;
    }

    if !buf_copy(pkt, next_hop as *mut c_void, size_of::<in_addr>()) {
        return TC_ACT_SHOT;
    }

    /* Skip the remaining next hops (may be zero). */
    skip_next_hops(
        pkt,
        ((*encap).unigue.hop_count)
            .wrapping_sub((*encap).unigue.next_hop)
            .wrapping_sub(1) as i32,
    )
}

/* Fill a bpf_sock_tuple to be used with the socket lookup functions.
 * This is a kludge that let's us work around verifier limitations:
 *
 *    fill_tuple(&t, foo, sizeof(struct iphdr), 123, 321)
 *
 * clang will substitute a constant for sizeof, which allows the verifier
 * to track its value. Based on this, it can figure out the constant
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
        x if x == size_of::<iphdr>() => {
            let ipv4 = iph as *mut iphdr;
            (*tuple).ipv4.daddr = (*ipv4).daddr;
            (*tuple).ipv4.saddr = (*ipv4).saddr;
            (*tuple).ipv4.sport = sport;
            (*tuple).ipv4.dport = dport;
            size_of::<bpf_sock_tuple_ipv4>() as u64
        }

        x if x == size_of::<ipv6hdr>() => {
            let ipv6 = iph as *mut ipv6hdr;
            ptr::copy_nonoverlapping((*ipv6).daddr.as_ptr(), (*tuple).ipv6.daddr.as_mut_ptr(), 4);
            ptr::copy_nonoverlapping((*ipv6).saddr.as_ptr(), (*tuple).ipv6.saddr.as_mut_ptr(), 4);
            (*tuple).ipv6.sport = sport;
            (*tuple).ipv6.dport = dport;
            size_of::<bpf_sock_tuple_ipv6>() as u64
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
    let sk = bpf_skc_lookup_tcp(skb, tuple, tuplen as u32, BPF_F_CURRENT_NETNS, 0);
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
        if tuplen == size_of::<bpf_sock_tuple_ipv6>() as u64 {
            iphlen = size_of::<ipv6hdr>() as u64;
        }

        if bpf_tcp_check_syncookie(sk, iph, iphlen as u32, tcp, size_of::<tcphdr>() as u32) == 0 {
            bpf_sk_release(sk);
            return verdict_t::SYN_COOKIE;
        }
    }

    bpf_sk_release(sk);
    verdict_t::UNKNOWN
}

unsafe fn classify_udp(skb: *mut __sk_buff, tuple: *mut bpf_sock_tuple, tuplen: u64) -> verdict_t {
    let sk = bpf_sk_lookup_udp(skb, tuple, tuplen as u32, BPF_F_CURRENT_NETNS, 0);
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
    match proto {
        IPPROTO_TCP => classify_tcp(skb, tuple, tuplen, ptr::null_mut(), ptr::null_mut()),
        IPPROTO_UDP => classify_udp(skb, tuple, tuplen),
        _ => {
            (*metrics).errors_total_malformed_icmp =
                (*metrics).errors_total_malformed_icmp.wrapping_add(1);
            verdict_t::INVALID
        }
    }
}

unsafe fn process_icmpv4(pkt: *mut buf_t, metrics: *mut metrics_t) -> verdict_t {
    let mut icmp: icmphdr = core::mem::zeroed();
    if !buf_copy(pkt, &mut icmp as *mut _ as *mut c_void, size_of::<icmphdr>()) {
        (*metrics).errors_total_malformed_icmp =
            (*metrics).errors_total_malformed_icmp.wrapping_add(1);
        return verdict_t::INVALID;
    }

    /* We should never receive encapsulated echo replies. */
    if icmp.type_ == ICMP_ECHOREPLY {
        (*metrics).errors_total_icmp_echo_replies =
            (*metrics).errors_total_icmp_echo_replies.wrapping_add(1);
        return verdict_t::INVALID;
    }

    if icmp.type_ == ICMP_ECHO {
        return verdict_t::ECHO_REQUEST;
    }

    if icmp.type_ != ICMP_DEST_UNREACH || icmp.code != ICMP_FRAG_NEEDED {
        (*metrics).errors_total_unwanted_icmp =
            (*metrics).errors_total_unwanted_icmp.wrapping_add(1);
        return verdict_t::INVALID;
    }

    let mut _ip4: iphdr = core::mem::zeroed();
    let ipv4 = pkt_parse_ipv4(pkt, &mut _ip4);
    if ipv4.is_null() {
        (*metrics).errors_total_malformed_icmp_pkt_too_big =
            (*metrics).errors_total_malformed_icmp_pkt_too_big.wrapping_add(1);
        return verdict_t::INVALID;
    }

    /* The source address in the outer IP header is from the entity that
     * originated the ICMP message. Use the original IP header to restore
     * the correct flow tuple.
     */
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    tuple.ipv4.saddr = (*ipv4).daddr;
    tuple.ipv4.daddr = (*ipv4).saddr;

    if !pkt_parse_icmp_l4_ports(pkt, &mut tuple.ipv4.sport as *mut u16 as *mut flow_ports_t) {
        (*metrics).errors_total_malformed_icmp_pkt_too_big =
            (*metrics).errors_total_malformed_icmp_pkt_too_big.wrapping_add(1);
        return verdict_t::INVALID;
    }

    classify_icmp((*pkt).skb, (*ipv4).protocol, &mut tuple, size_of::<bpf_sock_tuple_ipv4>() as u64, metrics)
}

unsafe fn process_icmpv6(pkt: *mut buf_t, metrics: *mut metrics_t) -> verdict_t {
    let mut icmp6: icmp6hdr = core::mem::zeroed();
    if !buf_copy(pkt, &mut icmp6 as *mut _ as *mut c_void, size_of::<icmp6hdr>()) {
        (*metrics).errors_total_malformed_icmp =
            (*metrics).errors_total_malformed_icmp.wrapping_add(1);
        return verdict_t::INVALID;
    }

    /* We should never receive encapsulated echo replies. */
    if icmp6.icmp6_type == ICMPV6_ECHO_REPLY {
        (*metrics).errors_total_icmp_echo_replies =
            (*metrics).errors_total_icmp_echo_replies.wrapping_add(1);
        return verdict_t::INVALID;
    }

    if icmp6.icmp6_type == ICMPV6_ECHO_REQUEST {
        return verdict_t::ECHO_REQUEST;
    }

    if icmp6.icmp6_type != ICMPV6_PKT_TOOBIG {
        (*metrics).errors_total_unwanted_icmp =
            (*metrics).errors_total_unwanted_icmp.wrapping_add(1);
        return verdict_t::INVALID;
    }

    let mut is_fragment = false;
    let mut l4_proto: u8 = 0;
    let mut _ipv6: ipv6hdr = core::mem::zeroed();
    let ipv6 = pkt_parse_ipv6(pkt, &mut _ipv6, &mut l4_proto, &mut is_fragment);
    if ipv6.is_null() {
        (*metrics).errors_total_malformed_icmp_pkt_too_big =
            (*metrics).errors_total_malformed_icmp_pkt_too_big.wrapping_add(1);
        return verdict_t::INVALID;
    }

    if is_fragment {
        (*metrics).errors_total_fragmented_ip =
            (*metrics).errors_total_fragmented_ip.wrapping_add(1);
        return verdict_t::INVALID;
    }

    /* Swap source and dest addresses. */
    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    ptr::copy_nonoverlapping((*ipv6).daddr.as_ptr(), tuple.ipv6.saddr.as_mut_ptr(), 4);
    ptr::copy_nonoverlapping((*ipv6).saddr.as_ptr(), tuple.ipv6.daddr.as_mut_ptr(), 4);

    if !pkt_parse_icmp_l4_ports(pkt, &mut tuple.ipv6.sport as *mut u16 as *mut flow_ports_t) {
        (*metrics).errors_total_malformed_icmp_pkt_too_big =
            (*metrics).errors_total_malformed_icmp_pkt_too_big.wrapping_add(1);
        return verdict_t::INVALID;
    }

    classify_icmp((*pkt).skb, l4_proto, &mut tuple, size_of::<bpf_sock_tuple_ipv6>() as u64, metrics)
}

unsafe fn process_tcp(pkt: *mut buf_t, iph: *mut c_void, iphlen: u64, metrics: *mut metrics_t) -> verdict_t {
    (*metrics).l4_protocol_packets_total_tcp =
        (*metrics).l4_protocol_packets_total_tcp.wrapping_add(1);

    let mut _tcp: tcphdr = core::mem::zeroed();
    let tcp = buf_assign(pkt, size_of::<tcphdr>(), &mut _tcp as *mut _ as *mut c_void) as *mut tcphdr;
    if tcp.is_null() {
        (*metrics).errors_total_malformed_tcp =
            (*metrics).errors_total_malformed_tcp.wrapping_add(1);
        return verdict_t::INVALID;
    }

    if (*tcp).syn() {
        return verdict_t::SYN;
    }

    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let tuplen = fill_tuple(&mut tuple, iph, iphlen, (*tcp).source, (*tcp).dest);
    classify_tcp((*pkt).skb, &mut tuple, tuplen, iph, tcp)
}

unsafe fn process_udp(pkt: *mut buf_t, iph: *mut c_void, iphlen: u64, metrics: *mut metrics_t) -> verdict_t {
    (*metrics).l4_protocol_packets_total_udp =
        (*metrics).l4_protocol_packets_total_udp.wrapping_add(1);

    let mut _udp: udphdr = core::mem::zeroed();
    let udph = buf_assign(pkt, size_of::<udphdr>(), &mut _udp as *mut _ as *mut c_void) as *mut udphdr;
    if udph.is_null() {
        (*metrics).errors_total_malformed_udp =
            (*metrics).errors_total_malformed_udp.wrapping_add(1);
        return verdict_t::INVALID;
    }

    let mut tuple: bpf_sock_tuple = core::mem::zeroed();
    let tuplen = fill_tuple(&mut tuple, iph, iphlen, (*udph).source, (*udph).dest);
    classify_udp((*pkt).skb, &mut tuple, tuplen)
}

unsafe fn process_ipv4(pkt: *mut buf_t, metrics: *mut metrics_t) -> verdict_t {
    (*metrics).l3_protocol_packets_total_ipv4 =
        (*metrics).l3_protocol_packets_total_ipv4.wrapping_add(1);

    let mut _ip4: iphdr = core::mem::zeroed();
    let ipv4 = pkt_parse_ipv4(pkt, &mut _ip4);
    if ipv4.is_null() {
        (*metrics).errors_total_malformed_ip =
            (*metrics).errors_total_malformed_ip.wrapping_add(1);
        return verdict_t::INVALID;
    }

    if (*ipv4).version() != 4 {
        (*metrics).errors_total_malformed_ip =
            (*metrics).errors_total_malformed_ip.wrapping_add(1);
        return verdict_t::INVALID;
    }

    if ipv4_is_fragment(ipv4) {
        (*metrics).errors_total_fragmented_ip =
            (*metrics).errors_total_fragmented_ip.wrapping_add(1);
        return verdict_t::INVALID;
    }

    match (*ipv4).protocol {
        IPPROTO_ICMP => process_icmpv4(pkt, metrics),
        IPPROTO_TCP => process_tcp(pkt, ipv4 as *mut c_void, size_of::<iphdr>() as u64, metrics),
        IPPROTO_UDP => process_udp(pkt, ipv4 as *mut c_void, size_of::<iphdr>() as u64, metrics),
        _ => {
            (*metrics).errors_total_unknown_l4_proto =
                (*metrics).errors_total_unknown_l4_proto.wrapping_add(1);
            verdict_t::INVALID
        }
    }
}

unsafe fn process_ipv6(pkt: *mut buf_t, metrics: *mut metrics_t) -> verdict_t {
    (*metrics).l3_protocol_packets_total_ipv6 =
        (*metrics).l3_protocol_packets_total_ipv6.wrapping_add(1);

    let mut l4_proto: u8 = 0;
    let mut is_fragment = false;
    let mut _ipv6: ipv6hdr = core::mem::zeroed();
    let ipv6 = pkt_parse_ipv6(pkt, &mut _ipv6, &mut l4_proto, &mut is_fragment);
    if ipv6.is_null() {
        (*metrics).errors_total_malformed_ip =
            (*metrics).errors_total_malformed_ip.wrapping_add(1);
        return verdict_t::INVALID;
    }

    if (*ipv6).version() != 6 {
        (*metrics).errors_total_malformed_ip =
            (*metrics).errors_total_malformed_ip.wrapping_add(1);
        return verdict_t::INVALID;
    }

    if is_fragment {
        (*metrics).errors_total_fragmented_ip =
            (*metrics).errors_total_fragmented_ip.wrapping_add(1);
        return verdict_t::INVALID;
    }

    match l4_proto {
        IPPROTO_ICMPV6 => process_icmpv6(pkt, metrics),
        IPPROTO_TCP => process_tcp(pkt, ipv6 as *mut c_void, size_of::<ipv6hdr>() as u64, metrics),
        IPPROTO_UDP => process_udp(pkt, ipv6 as *mut c_void, size_of::<ipv6hdr>() as u64, metrics),
        _ => {
            (*metrics).errors_total_unknown_l4_proto =
                (*metrics).errors_total_unknown_l4_proto.wrapping_add(1);
            verdict_t::INVALID
        }
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn cls_redirect(skb: *mut __sk_buff) -> i32 {
    let metrics = get_global_metrics();
    if metrics.is_null() {
        return TC_ACT_SHOT;
    }

    (*metrics).processed_packets_total = (*metrics).processed_packets_total.wrapping_add(1);

    /* Pass bogus packets as long as we're not sure they're
     * destined for us.
     */
    if (*skb).protocol != bpf_htons(ETH_P_IP) as u32 {
        return TC_ACT_OK;
    }

    /* Make sure that all encapsulation headers are available in
     * the linear portion of the skb. This makes it easy to manipulate them.
     */
    if bpf_skb_pull_data(skb, size_of::<encap_headers_t>() as u32) != 0 {
        return TC_ACT_OK;
    }

    let mut pkt = buf_t {
        skb,
        head: (*skb).data as usize as *mut u8,
        tail: (*skb).data_end as usize as *mut u8,
    };

    let encap = buf_assign(&mut pkt, size_of::<encap_headers_t>(), ptr::null_mut()) as *mut encap_headers_t;
    if encap.is_null() {
        return TC_ACT_OK;
    }

    if (*encap).ip.ihl() != 5 {
        /* We never have any options. */
        return TC_ACT_OK;
    }

    if (*encap).ip.daddr != ENCAPSULATION_IP || (*encap).ip.protocol != IPPROTO_UDP {
        return TC_ACT_OK;
    }

    /* TODO Check UDP length? */
    if (*encap).udp.dest != ENCAPSULATION_PORT {
        return TC_ACT_OK;
    }

    /* We now know that the packet is destined to us, we can
     * drop bogus ones.
     */
    if ipv4_is_fragment(&mut (*encap).ip as *mut iphdr) {
        (*metrics).errors_total_fragmented_ip =
            (*metrics).errors_total_fragmented_ip.wrapping_add(1);
        return TC_ACT_SHOT;
    }

    if (*encap).gue.variant() != 0 {
        (*metrics).errors_total_malformed_encapsulation =
            (*metrics).errors_total_malformed_encapsulation.wrapping_add(1);
        return TC_ACT_SHOT;
    }

    if (*encap).gue.control() != 0 {
        (*metrics).errors_total_malformed_encapsulation =
            (*metrics).errors_total_malformed_encapsulation.wrapping_add(1);
        return TC_ACT_SHOT;
    }

    if (*encap).gue.flags != 0 {
        (*metrics).errors_total_malformed_encapsulation =
            (*metrics).errors_total_malformed_encapsulation.wrapping_add(1);
        return TC_ACT_SHOT;
    }

    if (*encap).gue.hlen != (size_of::<uniguehdr>() / 4) as u8 + (*encap).unigue.hop_count {
        (*metrics).errors_total_malformed_encapsulation =
            (*metrics).errors_total_malformed_encapsulation.wrapping_add(1);
        return TC_ACT_SHOT;
    }

    if (*encap).unigue.version() != 0 {
        (*metrics).errors_total_malformed_encapsulation =
            (*metrics).errors_total_malformed_encapsulation.wrapping_add(1);
        return TC_ACT_SHOT;
    }

    if (*encap).unigue.reserved() != 0 {
        return TC_ACT_SHOT;
    }

    let mut next_hop = in_addr { s_addr: 0 };
    MAYBE_RETURN!(get_next_hop(&mut pkt, encap, &mut next_hop));

    if next_hop.s_addr == 0 {
        (*metrics).accepted_packets_total_last_hop =
            (*metrics).accepted_packets_total_last_hop.wrapping_add(1);
        return accept_locally(skb, encap);
    }

    let verdict: verdict_t;
    match (*encap).gue.proto_ctype {
        IPPROTO_IPIP => {
            verdict = process_ipv4(&mut pkt, metrics);
        }
        IPPROTO_IPV6 => {
            verdict = process_ipv6(&mut pkt, metrics);
        }
        _ => {
            (*metrics).errors_total_unknown_l3_proto =
                (*metrics).errors_total_unknown_l3_proto.wrapping_add(1);
            return TC_ACT_SHOT;
        }
    }

    match verdict {
        verdict_t::INVALID => {
            /* metrics have already been bumped */
            return TC_ACT_SHOT;
        }
        verdict_t::UNKNOWN => {
            return forward_to_next_hop(skb, encap, &mut next_hop, metrics);
        }
        verdict_t::ECHO_REQUEST => {
            (*metrics).accepted_packets_total_icmp_echo_request =
                (*metrics).accepted_packets_total_icmp_echo_request.wrapping_add(1);
        }
        verdict_t::SYN => {
            if (*encap).unigue.forward_syn() {
                return forward_to_next_hop(skb, encap, &mut next_hop, metrics);
            }

            (*metrics).accepted_packets_total_syn =
                (*metrics).accepted_packets_total_syn.wrapping_add(1);
        }
        verdict_t::SYN_COOKIE => {
            (*metrics).accepted_packets_total_syn_cookies =
                (*metrics).accepted_packets_total_syn_cookies.wrapping_add(1);
        }
        verdict_t::ESTABLISHED => {
            (*metrics).accepted_packets_total_established =
                (*metrics).accepted_packets_total_established.wrapping_add(1);
        }
    }

    accept_locally(skb, encap)
}
