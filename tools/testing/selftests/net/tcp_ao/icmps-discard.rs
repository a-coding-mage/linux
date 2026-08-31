// SPDX-License-Identifier: GPL-2.0
/*
 * Selftest that verifies that incomping ICMPs are ignored,
 * the TCP connection stays alive, no hard or soft errors get reported
 * to the usespace and the counter for ignored ICMPs is updated.
 *
 * RFC5925, 7.8:
 * >> A TCP-AO implementation MUST default to ignore incoming ICMPv4
 * messages of Type 3 (destination unreachable), Codes 2-4 (protocol
 * unreachable, port unreachable, and fragmentation needed -- 'hard
 * errors'), and ICMPv6 Type 1 (destination unreachable), Code 1
 * (administratively prohibited) and Code 4 (port unreachable) intended
 * for connections in synchronized states (ESTABLISHED, FIN-WAIT-1, FIN-
 * WAIT-2, CLOSE-WAIT, CLOSING, LAST-ACK, TIME-WAIT) that match MKTs.
 *
 * Author: Dmitry Safonov <dima@arista.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;

const packets_nr: size_t = 20;
const packet_size: size_t = 100;
static tcpao_icmps: &[u8] = b"TCPAODroppedIcmps\0";

// Selected by the original C build with IPV6_TEST.
#[cfg(IPV6_TEST)]
static dst_unreach: &[u8] = b"Icmp6InDestUnreachs\0";
#[cfg(IPV6_TEST)]
const sk_ip_level: c_int = SOL_IPV6;
#[cfg(IPV6_TEST)]
const sk_recverr: c_int = IPV6_RECVERR;

#[cfg(not(IPV6_TEST))]
static dst_unreach: &[u8] = b"InDestUnreachs\0";
#[cfg(not(IPV6_TEST))]
const sk_ip_level: c_int = SOL_IP;
#[cfg(not(IPV6_TEST))]
const sk_recverr: c_int = IP_RECVERR;

#[repr(C)]
struct tcp_counters {
    _private: [u8; 0],
}

#[repr(C)]
struct netstat {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [u8; 14],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct iphdr {
    version: u8,
    ihl: u8,
    tos: u8,
    tot_len: u16,
    id: u16,
    frag_off: u16,
    ttl: u8,
    protocol: u8,
    check: u16,
    saddr: u32,
    daddr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ipv6hdr {
    version: u8,
    priority: u8,
    flow_lbl: [u8; 3],
    payload_len: u16,
    nexthdr: u8,
    hop_limit: u8,
    saddr: in6_addr,
    daddr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct icmphdr_frag {
    __unused: u16,
    mtu: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
union icmphdr_un {
    frag: icmphdr_frag,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct icmphdr {
    type_: u8,
    code: u8,
    checksum: u16,
    un: icmphdr_un,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct icmp6hdr {
    icmp6_type: u8,
    icmp6_code: u8,
    icmp6_cksum: u16,
    icmp6_dataun: [u8; 4],
}

#[repr(C)]
struct tcph_min {
    sport: u16,
    dport: u16,
    seq: u32,
}

#[repr(C)]
struct icmp4_packet {
    iph: iphdr,
    icmph: icmphdr,
    iphe: iphdr,
    tcph: tcph_min,
}

#[repr(C)]
struct icmp6_packet {
    iph: ipv6hdr,
    icmph: icmp6hdr,
    iphe: ipv6hdr,
    tcph: tcph_min,
}

#[repr(C)]
struct pseudo_header6 {
    saddr: in6_addr,
    daddr: in6_addr,
    payload_len: u32,
    zero: [u8; 3],
    nexthdr: u8,
}

unsafe extern "C" {
    static this_ip_addr: *mut c_void;
    static this_ip_dest: *mut c_void;
    static test_server_port: u16;
    static test_family: c_int;

    fn netstat_read() -> *mut netstat;
    fn netstat_get(ns: *mut netstat, name: *const c_char, not_found: *mut bool) -> u64;
    fn netstat_print_diff(ns_before: *mut netstat, ns_after: *mut netstat);
    fn netstat_free(ns: *mut netstat);
    fn test_get_tcp_counters(sk: c_int, cnt: *mut tcp_counters) -> c_int;
    fn test_server_run(sk: c_int, quota: ssize_t, flags: c_int) -> ssize_t;
    fn test_error(fmt: *const c_char, ...);
    fn test_fail(fmt: *const c_char, ...);
    fn test_ok(fmt: *const c_char, ...);
    fn test_assert_counters(
        name: *const c_char,
        before: *const tcp_counters,
        after: *const tcp_counters,
        flags: c_int,
    );
    fn test_listen_socket(addr: *mut c_void, port: u16, backlog: c_int) -> c_int;
    fn test_set_ao_flags(sk: c_int, ao_required: bool, accept_icmps: bool) -> c_int;
    fn test_add_key(
        sk: c_int,
        password: *const c_char,
        addr: *mut c_void,
        prefix: c_int,
        sndid: c_int,
        rcvid: c_int,
    ) -> c_int;
    fn synchronize_threads();
    fn test_wait_fd(fd: c_int, timeout: c_int, events: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
    fn getsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn randomize_buffer(buf: *mut c_void, len: size_t);
    fn sendto(
        fd: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
        addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn htonl(hostlong: u32) -> u32;
    fn strerrordesc_np(errnum: c_int) -> *const c_char;
    fn getsockname(fd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn getpeername(fd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn test_client_verify(sk: c_int, packet_size: size_t, packets_nr: size_t) -> c_int;
    fn test_connect_socket(sk: c_int, addr: *mut c_void, port: u16) -> c_int;
    fn test_init(argc: c_int, server: unsafe extern "C" fn(*mut c_void) -> *mut c_void, client: unsafe extern "C" fn(*mut c_void) -> *mut c_void);
}

unsafe extern "C" {
    static DEFAULT_TEST_PASSWORD: *const c_char;
}

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_RAW: c_int = 3;
const IPPROTO_TCP: c_int = 6;
const IPPROTO_ICMP: c_int = 1;
const IPPROTO_ICMPV6: c_int = 58;
const IPPROTO_RAW: c_int = 255;
const SOL_IP: c_int = 0;
const SOL_IPV6: c_int = 41;
const SOL_TCP: c_int = 6;
const IP_RECVERR: c_int = 11;
const IPV6_RECVERR: c_int = 25;
const TCP_REPAIR: c_int = 19;
const TCP_REPAIR_QUEUE: c_int = 20;
const TCP_QUEUE_SEQ: c_int = 21;
const TCP_REPAIR_ON: c_int = 1;
const TCP_REPAIR_OFF_NO_WP: c_int = -1;
const TCP_RECV_QUEUE: c_int = 1;
const TEST_TIMEOUT_SEC: c_int = 5;
const TEST_CNT_GOOD: c_int = 1;
const TEST_CNT_AO_DROPPED_ICMP: c_int = 2;
const ICMP_DEST_UNREACH: u8 = 3;
const ICMP_PROT_UNREACH: u8 = 2;
const ICMP_PORT_UNREACH: u8 = 3;
const ICMP_FRAG_NEEDED: u8 = 4;
const ICMPV6_DEST_UNREACH: c_int = 1;
const ICMPV6_ADM_PROHIBITED: c_int = 1;
const ICMPV6_PORT_UNREACH: c_int = 4;

static mut packets_sent: size_t = 0;
static mut icmps_sent: size_t = 0;

unsafe fn test_icmps_fail(fmt: *const c_char, arg1: impl CVarArg) {
    #[cfg(TEST_ICMPS_ACCEPT)]
    test_ok(fmt, arg1);
    #[cfg(not(TEST_ICMPS_ACCEPT))]
    test_fail(fmt, arg1);
}

unsafe fn test_icmps_ok(fmt: *const c_char, arg1: impl CVarArg) {
    #[cfg(TEST_ICMPS_ACCEPT)]
    test_fail(fmt, arg1);
    #[cfg(not(TEST_ICMPS_ACCEPT))]
    test_ok(fmt, arg1);
}

trait CVarArg {}
impl CVarArg for ssize_t {}
impl CVarArg for u64 {}

unsafe fn serve_interfered(sk: c_int) {
    let test_quota: ssize_t = (packet_size * packets_nr * 10) as ssize_t;
    let dest_unreach_a: u64;
    let dest_unreach_b: u64;
    let icmp_ignored_a: u64;
    let icmp_ignored_b: u64;
    let mut cnt1 = core::mem::MaybeUninit::<tcp_counters>::uninit();
    let mut cnt2 = core::mem::MaybeUninit::<tcp_counters>::uninit();
    let mut counter_not_found: bool = false;
    let ns_after: *mut netstat;
    let ns_before: *mut netstat;
    let bytes: ssize_t;

    ns_before = netstat_read();
    dest_unreach_a = netstat_get(ns_before, dst_unreach.as_ptr() as *const c_char, ptr::null_mut());
    icmp_ignored_a = netstat_get(ns_before, tcpao_icmps.as_ptr() as *const c_char, ptr::null_mut());
    if test_get_tcp_counters(sk, cnt1.as_mut_ptr()) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }
    bytes = test_server_run(sk, test_quota, 0);
    ns_after = netstat_read();
    netstat_print_diff(ns_before, ns_after);
    dest_unreach_b = netstat_get(ns_after, dst_unreach.as_ptr() as *const c_char, ptr::null_mut());
    icmp_ignored_b = netstat_get(
        ns_after,
        tcpao_icmps.as_ptr() as *const c_char,
        &mut counter_not_found,
    );
    if test_get_tcp_counters(sk, cnt2.as_mut_ptr()) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }

    netstat_free(ns_before);
    netstat_free(ns_after);

    let cnt1 = cnt1.assume_init();
    let cnt2 = cnt2.assume_init();

    if dest_unreach_a >= dest_unreach_b {
        test_fail(
            c"%s counter didn't change: %lu >= %lu".as_ptr(),
            dst_unreach.as_ptr() as *const c_char,
            dest_unreach_a,
            dest_unreach_b,
        );
        return;
    }
    test_ok(
        c"%s delivered %lu".as_ptr(),
        dst_unreach.as_ptr() as *const c_char,
        dest_unreach_b - dest_unreach_a,
    );
    if bytes < 0 {
        #[cfg(TEST_ICMPS_ACCEPT)]
        test_ok(c"Server failed with %zd: %s".as_ptr(), bytes, strerrordesc_np((-bytes) as c_int));
        #[cfg(not(TEST_ICMPS_ACCEPT))]
        test_fail(c"Server failed with %zd: %s".as_ptr(), bytes, strerrordesc_np((-bytes) as c_int));
    } else {
        #[cfg(TEST_ICMPS_ACCEPT)]
        test_fail(c"Server survived %zd bytes of traffic".as_ptr(), test_quota);
        #[cfg(not(TEST_ICMPS_ACCEPT))]
        test_ok(c"Server survived %zd bytes of traffic".as_ptr(), test_quota);
    }
    if counter_not_found {
        test_fail(c"Not found %s counter".as_ptr(), tcpao_icmps.as_ptr() as *const c_char);
        return;
    }
    #[cfg(TEST_ICMPS_ACCEPT)]
    test_assert_counters(ptr::null(), &cnt1, &cnt2, TEST_CNT_GOOD);
    #[cfg(not(TEST_ICMPS_ACCEPT))]
    test_assert_counters(
        ptr::null(),
        &cnt1,
        &cnt2,
        TEST_CNT_GOOD | TEST_CNT_AO_DROPPED_ICMP,
    );
    if icmp_ignored_a >= icmp_ignored_b {
        #[cfg(TEST_ICMPS_ACCEPT)]
        test_ok(
            c"%s counter didn't change: %lu >= %lu".as_ptr(),
            tcpao_icmps.as_ptr() as *const c_char,
            icmp_ignored_a,
            icmp_ignored_b,
        );
        #[cfg(not(TEST_ICMPS_ACCEPT))]
        test_fail(
            c"%s counter didn't change: %lu >= %lu".as_ptr(),
            tcpao_icmps.as_ptr() as *const c_char,
            icmp_ignored_a,
            icmp_ignored_b,
        );
        return;
    }
    #[cfg(TEST_ICMPS_ACCEPT)]
    test_fail(c"ICMPs ignored %lu".as_ptr(), icmp_ignored_b - icmp_ignored_a);
    #[cfg(not(TEST_ICMPS_ACCEPT))]
    test_ok(c"ICMPs ignored %lu".as_ptr(), icmp_ignored_b - icmp_ignored_a);
}

unsafe extern "C" fn server_fn(_arg: *mut c_void) -> *mut c_void {
    let mut val: c_int;
    let sk: c_int;
    let lsk: c_int;
    let mut accept_icmps: bool = false;

    lsk = test_listen_socket(this_ip_addr, test_server_port, 1);

    #[cfg(TEST_ICMPS_ACCEPT)]
    {
        accept_icmps = true;
    }

    if test_set_ao_flags(lsk, false, accept_icmps) != 0 {
        test_error(c"setsockopt(TCP_AO_INFO)".as_ptr());
    }

    if test_add_key(lsk, DEFAULT_TEST_PASSWORD, this_ip_dest, -1, 100, 100) != 0 {
        test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
    }
    synchronize_threads();

    if test_wait_fd(lsk, TEST_TIMEOUT_SEC, 0) != 0 {
        test_error(c"test_wait_fd()".as_ptr());
    }

    sk = accept(lsk, ptr::null_mut(), ptr::null_mut());
    if sk < 0 {
        test_error(c"accept()".as_ptr());
    }

    /* Fail on hard ip errors, such as dest unreachable (RFC1122) */
    val = 1;
    if setsockopt(
        sk,
        sk_ip_level,
        sk_recverr,
        &val as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt()".as_ptr());
    }

    synchronize_threads();

    serve_interfered(sk);
    ptr::null_mut()
}

unsafe fn checksum4_nofold(data: *mut c_void, len: size_t, mut sum: u32) -> u32 {
    let words = data as *mut u16;
    let mut i: size_t;

    i = 0;
    while i < len / size_of::<u16>() {
        sum = sum.wrapping_add(*words.add(i) as u32);
        i += 1;
    }
    if (len & 1) != 0 {
        sum = sum.wrapping_add(*(data as *mut c_char).add(len - 1) as u32);
    }
    sum
}

unsafe fn checksum4_fold(data: *mut c_void, len: size_t, mut sum: u32) -> u16 {
    sum = checksum4_nofold(data, len, sum);
    while sum > 0xFFFF {
        sum = (sum & 0xFFFF).wrapping_add(sum >> 16);
    }
    !(sum as u16)
}

unsafe fn set_ip4hdr(
    iph: *mut iphdr,
    packet_len: size_t,
    proto: c_int,
    src: *mut sockaddr_in,
    dst: *mut sockaddr_in,
) {
    (*iph).version = 4;
    (*iph).ihl = 5;
    (*iph).tos = 0;
    (*iph).tot_len = htons(packet_len as u16);
    (*iph).ttl = 2;
    (*iph).protocol = proto as u8;
    (*iph).saddr = (*src).sin_addr.s_addr;
    (*iph).daddr = (*dst).sin_addr.s_addr;
    (*iph).check = checksum4_fold(iph as *mut c_void, ((*iph).ihl as size_t) << 1, 0);
}

unsafe fn icmp_interfere4(
    type_: u8,
    code: u8,
    rcv_nxt: u32,
    src: *mut sockaddr_in,
    dst: *mut sockaddr_in,
) {
    let sk = socket(AF_INET, SOCK_RAW, IPPROTO_RAW);
    let mut packet: icmp4_packet = core::mem::zeroed();
    let mut packet_len: size_t;
    let bytes: ssize_t;

    if sk < 0 {
        test_error(c"socket(AF_INET, SOCK_RAW, IPPROTO_RAW)".as_ptr());
    }

    packet_len = size_of::<icmp4_packet>();
    set_ip4hdr(&mut packet.iph, packet_len, IPPROTO_ICMP, src, dst);

    packet.icmph.type_ = type_;
    packet.icmph.code = code;
    if code == ICMP_FRAG_NEEDED {
        randomize_buffer(
            &mut packet.icmph.un.frag.mtu as *mut _ as *mut c_void,
            size_of::<u16>(),
        );
    }

    packet_len = size_of::<iphdr>() + size_of::<tcph_min>();
    set_ip4hdr(&mut packet.iphe, packet_len, IPPROTO_TCP, dst, src);

    packet.tcph.sport = (*dst).sin_port;
    packet.tcph.dport = (*src).sin_port;
    packet.tcph.seq = htonl(rcv_nxt);

    packet_len = size_of::<icmp4_packet>() - size_of::<iphdr>();
    packet.icmph.checksum = checksum4_fold(&mut packet.icmph as *mut _ as *mut c_void, packet_len, 0);

    bytes = sendto(
        sk,
        &packet as *const _ as *const c_void,
        size_of::<icmp4_packet>(),
        0,
        dst as *const sockaddr,
        size_of::<sockaddr_in>() as socklen_t,
    );
    if bytes != size_of::<icmp4_packet>() as ssize_t {
        test_error(c"send(): %zd".as_ptr(), bytes);
    }
    icmps_sent += 1;

    close(sk);
}

unsafe fn set_ip6hdr(
    iph: *mut ipv6hdr,
    packet_len: size_t,
    proto: c_int,
    src: *mut sockaddr_in6,
    dst: *mut sockaddr_in6,
) {
    (*iph).version = 6;
    (*iph).payload_len = htons(packet_len as u16);
    (*iph).nexthdr = proto as u8;
    (*iph).hop_limit = 2;
    (*iph).saddr = (*src).sin6_addr;
    (*iph).daddr = (*dst).sin6_addr;
}

unsafe fn csum_fold(csum: u32) -> u16 {
    let mut sum = csum;

    sum = (sum & 0xffff).wrapping_add(sum >> 16);
    sum = (sum & 0xffff).wrapping_add(sum >> 16);
    !(sum as u16)
}

unsafe fn csum_add(csum: u32, addend: u32) -> u32 {
    let mut res = csum;

    res = res.wrapping_add(addend);
    res.wrapping_add((res < addend) as u32)
}

#[inline(never)]
unsafe fn checksum6_nofold(data: *mut c_void, len: size_t, mut sum: u32) -> u32 {
    let words = data as *mut u16;
    let mut i: size_t;

    i = 0;
    while i < len / size_of::<u16>() {
        sum = csum_add(sum, *words.add(i) as u32);
        i += 1;
    }
    if (len & 1) != 0 {
        sum = csum_add(sum, *(data as *mut c_char).add(len - 1) as u32);
    }
    sum
}

#[inline(never)]
unsafe fn icmp6_checksum(
    src: *mut sockaddr_in6,
    dst: *mut sockaddr_in6,
    ptr_: *mut c_void,
    len: size_t,
    proto: u8,
) -> u16 {
    let mut pseudo_header: pseudo_header6 = core::mem::zeroed();
    let mut sum: u32;

    pseudo_header.saddr = (*src).sin6_addr;
    pseudo_header.daddr = (*dst).sin6_addr;
    pseudo_header.payload_len = htonl(len as u32);
    pseudo_header.nexthdr = proto;

    sum = checksum6_nofold(
        &mut pseudo_header as *mut _ as *mut c_void,
        size_of::<pseudo_header6>(),
        0,
    );
    sum = checksum6_nofold(ptr_, len, sum);

    csum_fold(sum)
}

unsafe fn icmp6_interfere(
    type_: c_int,
    code: c_int,
    rcv_nxt: u32,
    src: *mut sockaddr_in6,
    dst: *mut sockaddr_in6,
) {
    let sk = socket(AF_INET6, SOCK_RAW, IPPROTO_RAW);
    let mut dst_raw: sockaddr_in6 = *dst;
    let mut packet: icmp6_packet = core::mem::zeroed();
    let mut packet_len: size_t;
    let bytes: ssize_t;

    if sk < 0 {
        test_error(c"socket(AF_INET6, SOCK_RAW, IPPROTO_RAW)".as_ptr());
    }

    packet_len = size_of::<icmp6_packet>() - size_of::<ipv6hdr>();
    set_ip6hdr(&mut packet.iph, packet_len, IPPROTO_ICMPV6, src, dst);

    packet.icmph.icmp6_type = type_ as u8;
    packet.icmph.icmp6_code = code as u8;

    packet_len = size_of::<ipv6hdr>() + size_of::<tcph_min>();
    set_ip6hdr(&mut packet.iphe, packet_len, IPPROTO_TCP, dst, src);

    packet.tcph.sport = (*dst).sin6_port;
    packet.tcph.dport = (*src).sin6_port;
    packet.tcph.seq = htonl(rcv_nxt);

    packet_len = size_of::<icmp6_packet>() - size_of::<ipv6hdr>();

    packet.icmph.icmp6_cksum = icmp6_checksum(
        src,
        dst,
        &mut packet.icmph as *mut _ as *mut c_void,
        packet_len,
        IPPROTO_ICMPV6 as u8,
    );

    dst_raw.sin6_port = htons(IPPROTO_RAW as u16);
    bytes = sendto(
        sk,
        &packet as *const _ as *const c_void,
        size_of::<icmp6_packet>(),
        0,
        &dst_raw as *const _ as *const sockaddr,
        size_of::<sockaddr_in6>() as socklen_t,
    );
    if bytes != size_of::<icmp6_packet>() as ssize_t {
        test_error(c"send(): %zd".as_ptr(), bytes);
    }
    icmps_sent += 1;

    close(sk);
}

unsafe fn get_rcv_nxt(sk: c_int) -> u32 {
    let mut val: c_int = TCP_REPAIR_ON;
    let mut ret: u32 = 0;
    let mut sz: socklen_t = size_of::<u32>() as socklen_t;

    if setsockopt(
        sk,
        SOL_TCP,
        TCP_REPAIR,
        &val as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_REPAIR)".as_ptr());
    }
    val = TCP_RECV_QUEUE;
    if setsockopt(
        sk,
        SOL_TCP,
        TCP_REPAIR_QUEUE,
        &val as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_REPAIR_QUEUE)".as_ptr());
    }
    if getsockopt(
        sk,
        SOL_TCP,
        TCP_QUEUE_SEQ,
        &mut ret as *mut _ as *mut c_void,
        &mut sz,
    ) != 0
    {
        test_error(c"getsockopt(TCP_QUEUE_SEQ)".as_ptr());
    }
    val = TCP_REPAIR_OFF_NO_WP;
    if setsockopt(
        sk,
        SOL_TCP,
        TCP_REPAIR,
        &val as *const _ as *const c_void,
        size_of::<c_int>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(TCP_REPAIR)".as_ptr());
    }
    ret
}

unsafe fn icmp_interfere(nr: size_t, rcv_nxt: u32, src: *mut c_void, dst: *mut c_void) {
    let saddr4 = src as *mut sockaddr_in;
    let daddr4 = dst as *mut sockaddr_in;
    let saddr6 = src as *mut sockaddr_in6;
    let daddr6 = dst as *mut sockaddr_in6;
    let mut i: size_t;

    if (*saddr4).sin_family != (*daddr4).sin_family {
        test_error(c"Different address families".as_ptr());
    }

    i = 0;
    while i < nr {
        if (*saddr4).sin_family as c_int == AF_INET {
            icmp_interfere4(ICMP_DEST_UNREACH, ICMP_PROT_UNREACH, rcv_nxt, saddr4, daddr4);
            icmp_interfere4(ICMP_DEST_UNREACH, ICMP_PORT_UNREACH, rcv_nxt, saddr4, daddr4);
            icmp_interfere4(ICMP_DEST_UNREACH, ICMP_FRAG_NEEDED, rcv_nxt, saddr4, daddr4);
            icmps_sent += 3;
        } else if (*saddr4).sin_family as c_int == AF_INET6 {
            icmp6_interfere(ICMPV6_DEST_UNREACH, ICMPV6_ADM_PROHIBITED, rcv_nxt, saddr6, daddr6);
            icmp6_interfere(ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, rcv_nxt, saddr6, daddr6);
            icmps_sent += 2;
        } else {
            test_error(c"Not ip address family".as_ptr());
        }
        i += 1;
    }
}

unsafe fn send_interfered(sk: c_int) {
    let mut src: sockaddr_in6 = core::mem::zeroed();
    let mut dst: sockaddr_in6 = core::mem::zeroed();
    let mut addr_sz: socklen_t;

    addr_sz = size_of::<sockaddr_in6>() as socklen_t;
    if getsockname(sk, &mut src as *mut _ as *mut sockaddr, &mut addr_sz) != 0 {
        test_error(c"getsockname()".as_ptr());
    }
    addr_sz = size_of::<sockaddr_in6>() as socklen_t;
    if getpeername(sk, &mut dst as *mut _ as *mut sockaddr, &mut addr_sz) != 0 {
        test_error(c"getpeername()".as_ptr());
    }

    loop {
        let rcv_nxt: u32;

        if test_client_verify(sk, packet_size, packets_nr) != 0 {
            test_fail(c"client: connection is broken".as_ptr());
            return;
        }
        packets_sent += packets_nr;
        rcv_nxt = get_rcv_nxt(sk);
        icmp_interfere(
            packets_nr,
            rcv_nxt,
            &mut src as *mut _ as *mut c_void,
            &mut dst as *mut _ as *mut c_void,
        );
    }
}

unsafe extern "C" fn client_fn(_arg: *mut c_void) -> *mut c_void {
    let sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);

    if sk < 0 {
        test_error(c"socket()".as_ptr());
    }

    if test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, -1, 100, 100) != 0 {
        test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
    }

    synchronize_threads();
    if test_connect_socket(sk, this_ip_dest, test_server_port) <= 0 {
        test_error(c"failed to connect()".as_ptr());
    }
    synchronize_threads();

    send_interfered(sk);

    /* Not expecting client to quit */
    test_fail(c"client disconnected".as_ptr());

    ptr::null_mut()
}

fn main() {
    unsafe {
        test_init(4, server_fn, client_fn);
    }
}
