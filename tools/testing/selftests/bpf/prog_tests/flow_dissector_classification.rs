// SPDX-License-Identifier: GPL-2.0

// Translated from C. Includes are external dependencies from the original
// kernel selftest environment.

use core::ffi::{c_char, c_int, c_short, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const CFG_PORT_INNER: c_int = 8000;
const CFG_PORT_GUE: c_int = 6080;
const SUBTEST_NAME_MAX_LEN: usize = 32;
const TEST_NAME_MAX_LEN: usize = 32 + SUBTEST_NAME_MAX_LEN;
const MAX_SOURCE_PORTS: usize = 3;
const TEST_PACKETS_COUNT: c_int = 10;
const TEST_PACKET_LEN: usize = 100;
const TEST_PACKET_PATTERN: c_char = b'a' as c_char;
const TEST_IPV4: *const c_char = b"192.168.0.1/32\0".as_ptr() as *const c_char;
const TEST_IPV6: *const c_char = b"100::a/128\0".as_ptr() as *const c_char;
const TEST_TUNNEL_REMOTE: *const c_char = b"127.0.0.2\0".as_ptr() as *const c_char;
const TEST_TUNNEL_LOCAL: *const c_char = b"127.0.0.1\0".as_ptr() as *const c_char;

const ETH_DATA_LEN: usize = 1500;
const AF_INET: c_int = 2;
const PF_INET: c_int = AF_INET;
const AF_INET6: c_int = 10;
const PF_INET6: c_int = AF_INET6;
const SOCK_RAW: c_int = 3;
const SOCK_DGRAM: c_int = 2;
const IPPROTO_RAW: c_int = 255;
const IPPROTO_UDP: c_int = 17;
const IPPROTO_GRE: c_int = 47;
const IPPROTO_IPIP: c_int = 4;
const IPPROTO_IPV6: c_int = 41;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86DD;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const MSG_DONTWAIT: c_int = 0x40;
const EAGAIN: c_int = 11;
const POLLIN: c_short = 0x0001;
const BPF_ANY: u64 = 0;
const BPF_FLOW_DISSECTOR: c_int = 6;

macro_rules! ASSERT_LE {
    ($a:expr, $b:expr, $name:expr) => {
        (($a) <= ($b))
    };
}
macro_rules! ASSERT_GT {
    ($a:expr, $b:expr, $name:expr) => {
        (($a) > ($b))
    };
}
macro_rules! ASSERT_GE {
    ($a:expr, $b:expr, $name:expr) => {
        (($a) >= ($b))
    };
}
macro_rules! ASSERT_EQ {
    ($a:expr, $b:expr, $name:expr) => {
        (($a) == ($b))
    };
}
macro_rules! ASSERT_OK {
    ($a:expr, $name:expr) => {
        (($a) == 0)
    };
}
macro_rules! ASSERT_OK_FD {
    ($a:expr, $name:expr) => {
        (($a) >= 0)
    };
}
macro_rules! ASSERT_OK_PTR {
    ($a:expr, $name:expr) => {
        (!($a).is_null())
    };
}
macro_rules! ARRAY_SIZE {
    ($a:expr) => {
        ($a).len()
    };
}
macro_rules! SYS {
    ($label:ident, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        let _ = ($fmt, $($arg),*);
        // Original C macro jumps to $label on command failure.
    }};
}
macro_rules! SYS_NOFAIL {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        let _ = ($fmt, $($arg),*);
    }};
}

#[repr(C, packed)]
struct grehdr {
    unused: u16,
    protocol: u16,
}

#[repr(C)]
union guehdr_u {
    s: guehdr_s,
    word: u32,
}

#[repr(C)]
struct guehdr_s {
    // Original uses endian-dependent bitfields:
    // little endian: hlen:5, control:1, version:2
    // big endian: version:2, control:1, hlen:5
    bitfield_0: u8,
    proto_ctype: u8,
    flags: u16,
}

#[repr(C)]
struct guehdr {
    u: guehdr_u,
}

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
struct iphdr {
    ihl_version: u8,
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
struct ipv6hdr {
    priority_version: u8,
    flow_lbl: [u8; 3],
    payload_len: u16,
    nexthdr: u8,
    hop_limit: u8,
    saddr: in6_addr,
    daddr: in6_addr,
}

#[repr(C)]
struct udphdr {
    source: u16,
    dest: u16,
    len: u16,
    check: u16,
}

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

type c_long = i64;

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_flow_maps {
    jmp_table: *mut bpf_map,
}

#[repr(C)]
struct bpf_flow_progs {
    _dissect: *mut bpf_program,
}

#[repr(C)]
struct bpf_flow {
    obj: *mut bpf_object,
    maps: bpf_flow_maps,
    progs: bpf_flow_progs,
}

#[repr(C)]
struct netns_obj {
    _private: [u8; 0],
}

#[repr(C)]
struct test_configuration {
    name: [c_char; SUBTEST_NAME_MAX_LEN],
    test_setup: Option<unsafe extern "C" fn() -> c_int>,
    test_teardown: Option<unsafe extern "C" fn()>,
    source_ports: [c_int; MAX_SOURCE_PORTS],
    cfg_l3_inner: c_int,
    in_saddr4: sockaddr_in,
    in_daddr4: sockaddr_in,
    in_saddr6: sockaddr_in6,
    in_daddr6: sockaddr_in6,
    cfg_l3_outer: c_int,
    out_saddr4: sockaddr_in,
    out_daddr4: sockaddr_in,
    out_saddr6: sockaddr_in6,
    out_daddr6: sockaddr_in6,
    cfg_encap_proto: c_int,
    cfg_dsfield_inner: u8,
    cfg_dsfield_outer: u8,
    cfg_l3_extra: c_int,
    extra_saddr4: sockaddr_in,
    extra_daddr4: sockaddr_in,
    extra_saddr6: sockaddr_in6,
    extra_daddr6: sockaddr_in6,
}

static mut buf: [c_char; ETH_DATA_LEN] = [0; ETH_DATA_LEN];

extern "C" {
    static mut errno: c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const c_void, addrlen: u32) -> c_int;
    fn bind(sockfd: c_int, addr: *const c_void, addrlen: u32) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn build_ip_csum(iph: *mut c_void) -> u16;
    fn build_udp_v4_csum(iph: *mut c_void, udph: *mut udphdr) -> u16;
    fn build_udp_v6_csum(ip6h: *mut c_void, udph: *mut udphdr) -> u16;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, attachable_fd: c_int, type_: c_int, flags: c_ulong) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__max_entries(map: *mut bpf_map) -> c_int;
    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_prog_detach2(prog_fd: c_int, attachable_fd: c_int, type_: c_int) -> c_int;
    fn bpf_flow__open_and_load() -> *mut bpf_flow;
    fn bpf_flow__destroy(skel: *mut bpf_flow);
    fn netns_new(name: *const c_char, attach: bool) -> *mut netns_obj;
    fn netns_free(ns: *mut netns_obj);
    fn write_sysctl(path: *const c_char, value: *const c_char) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
}

const fn const_htons(x: u16) -> u16 {
    x.to_be()
}

const fn const_htonl(x: u32) -> u32 {
    x.to_be()
}

const fn init_addr4(addr4: u32, port: u16) -> sockaddr_in {
    sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: const_htons(port),
        sin_addr: in_addr { s_addr: const_htonl(addr4) },
        sin_zero: [0; 8],
    }
}

const fn init_addr6(addr6: in6_addr, port: u16) -> sockaddr_in6 {
    sockaddr_in6 {
        sin6_family: AF_INET6 as u16,
        sin6_port: const_htons(port),
        sin6_flowinfo: 0,
        sin6_addr: addr6,
        sin6_scope_id: 0,
    }
}

const IN6ADDR_LOOPBACK_INIT: in6_addr = in6_addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
};
const TEST_IN4_SRC_ADDR_DEFAULT: sockaddr_in = init_addr4(INADDR_LOOPBACK + 2, 0);
const TEST_IN4_DST_ADDR_DEFAULT: sockaddr_in = init_addr4(INADDR_LOOPBACK, CFG_PORT_INNER as u16);
const TEST_OUT4_SRC_ADDR_DEFAULT: sockaddr_in = init_addr4(INADDR_LOOPBACK + 1, 0);
const TEST_OUT4_DST_ADDR_DEFAULT: sockaddr_in = init_addr4(INADDR_LOOPBACK, 0);
const TEST_IN6_SRC_ADDR_DEFAULT: sockaddr_in6 = init_addr6(IN6ADDR_LOOPBACK_INIT, 0);
const TEST_IN6_DST_ADDR_DEFAULT: sockaddr_in6 = init_addr6(IN6ADDR_LOOPBACK_INIT, CFG_PORT_INNER as u16);
const TEST_OUT6_SRC_ADDR_DEFAULT: sockaddr_in6 = init_addr6(IN6ADDR_LOOPBACK_INIT, 0);
const TEST_OUT6_DST_ADDR_DEFAULT: sockaddr_in6 = init_addr6(IN6ADDR_LOOPBACK_INIT, 0);
const TEST_IN4_SRC_ADDR_DISSECT_CONTINUE: sockaddr_in = init_addr4(INADDR_LOOPBACK + 126, 0);
const TEST_IN4_SRC_ADDR_IPIP: sockaddr_in = init_addr4(0x01010101, 0);
const TEST_IN4_DST_ADDR_IPIP: sockaddr_in = init_addr4(0xC0A80001, CFG_PORT_INNER as u16);

unsafe fn util_gettime() -> c_ulong {
    let mut tv: timeval = zeroed();
    gettimeofday(&mut tv, ptr::null_mut());
    (tv.tv_sec as c_ulong * 1000) + (tv.tv_usec as c_ulong / 1000)
}

unsafe fn build_ipv4_header(header: *mut c_void, proto: u8, src: u32, dst: u32, payload_len: c_int, tos: u8) {
    let iph = header as *mut iphdr;
    (*iph).ihl_version = (4 << 4) | 5;
    (*iph).tos = tos;
    (*iph).ttl = 8;
    (*iph).tot_len = htons((size_of::<iphdr>() as c_int + payload_len) as u16);
    (*iph).id = htons(1337);
    (*iph).protocol = proto;
    (*iph).saddr = src;
    (*iph).daddr = dst;
    (*iph).check = build_ip_csum(iph as *mut c_void);
}

unsafe fn ipv6_set_dsfield(ip6h: *mut ipv6hdr, dsfield: u8) {
    let mut val: u16;
    let ptr = ip6h as *mut u16;
    val = ntohs(*ptr);
    val &= 0xF00F;
    val |= (dsfield as u16) << 4;
    *ptr = htons(val);
}

unsafe fn build_ipv6_header(header: *mut c_void, proto: u8, src: *const sockaddr_in6, dst: *const sockaddr_in6, payload_len: c_int, dsfield: u8) {
    let ip6h = header as *mut ipv6hdr;
    (*ip6h).priority_version = 6 << 4;
    (*ip6h).payload_len = htons(payload_len as u16);
    (*ip6h).nexthdr = proto;
    (*ip6h).hop_limit = 8;
    ipv6_set_dsfield(ip6h, dsfield);
    memcpy(&mut (*ip6h).saddr as *mut _ as *mut c_void, &(*src).sin6_addr as *const _ as *const c_void, size_of::<in6_addr>());
    memcpy(&mut (*ip6h).daddr as *mut _ as *mut c_void, &(*dst).sin6_addr as *const _ as *const c_void, size_of::<in6_addr>());
}

unsafe fn build_udp_header(header: *mut c_void, payload_len: c_int, sport: u16, dport: u16, family: c_int) {
    let udph = header as *mut udphdr;
    let len = size_of::<udphdr>() as c_int + payload_len;
    (*udph).source = htons(sport);
    (*udph).dest = htons(dport);
    (*udph).len = htons(len as u16);
    (*udph).check = 0;
    if family == AF_INET {
        (*udph).check = build_udp_v4_csum((header as *mut u8).sub(size_of::<iphdr>()) as *mut c_void, udph);
    } else {
        (*udph).check = build_udp_v6_csum((header as *mut u8).sub(size_of::<ipv6hdr>()) as *mut c_void, udph);
    }
}

unsafe fn build_gue_header(header: *mut c_void, proto: u8) {
    let gueh = header as *mut guehdr;
    (*gueh).u.s.proto_ctype = proto;
}

unsafe fn build_gre_header(header: *mut c_void, proto: u16) {
    let greh = header as *mut grehdr;
    ptr::addr_of_mut!((*greh).protocol).write_unaligned(htons(proto));
}

fn l3_length(family: c_int) -> c_int {
    if family == AF_INET {
        size_of::<iphdr>() as c_int
    } else {
        size_of::<ipv6hdr>() as c_int
    }
}

unsafe fn build_packet(test: *const test_configuration, sport: u16) -> c_int {
    let mut ol3_len = 0;
    let mut ol4_len = 0;
    let mut il3_len;
    let il4_len;
    let mut el3_len = 0;
    let packet_len;

    memset(buf.as_mut_ptr() as *mut c_void, 0, ETH_DATA_LEN);

    if (*test).cfg_l3_extra != 0 {
        el3_len = l3_length((*test).cfg_l3_extra);
    }

    /* calculate header offsets */
    if (*test).cfg_encap_proto != 0 {
        ol3_len = l3_length((*test).cfg_l3_outer);

        if (*test).cfg_encap_proto == IPPROTO_GRE {
            ol4_len = size_of::<grehdr>() as c_int;
        } else if (*test).cfg_encap_proto == IPPROTO_UDP {
            ol4_len = (size_of::<udphdr>() + size_of::<guehdr>()) as c_int;
        }
    }

    il3_len = l3_length((*test).cfg_l3_inner);
    il4_len = size_of::<udphdr>() as c_int;

    packet_len = el3_len + ol3_len + ol4_len + il3_len + il4_len + TEST_PACKET_LEN as c_int;
    if !ASSERT_LE!(packet_len as usize, size_of_val_buf(), "check packet size") {
        return -1;
    }

    /*
     * Fill packet from inside out, to calculate correct checksums.
     * But create ip before udp headers, as udp uses ip for pseudo-sum.
     */
    memset(
        buf.as_mut_ptr().offset((el3_len + ol3_len + ol4_len + il3_len + il4_len) as isize) as *mut c_void,
        TEST_PACKET_PATTERN as c_int,
        TEST_PACKET_LEN,
    );

    /* add zero byte for udp csum padding */
    buf[(el3_len + ol3_len + ol4_len + il3_len + il4_len + TEST_PACKET_LEN as c_int) as usize] = 0;

    match (*test).cfg_l3_inner {
        PF_INET => build_ipv4_header(
            buf.as_mut_ptr().offset((el3_len + ol3_len + ol4_len) as isize) as *mut c_void,
            IPPROTO_UDP as u8,
            (*test).in_saddr4.sin_addr.s_addr,
            (*test).in_daddr4.sin_addr.s_addr,
            il4_len + TEST_PACKET_LEN as c_int,
            (*test).cfg_dsfield_inner,
        ),
        PF_INET6 => build_ipv6_header(
            buf.as_mut_ptr().offset((el3_len + ol3_len + ol4_len) as isize) as *mut c_void,
            IPPROTO_UDP as u8,
            &(*test).in_saddr6,
            &(*test).in_daddr6,
            il4_len + TEST_PACKET_LEN as c_int,
            (*test).cfg_dsfield_inner,
        ),
        _ => {}
    }

    build_udp_header(
        buf.as_mut_ptr().offset((el3_len + ol3_len + ol4_len + il3_len) as isize) as *mut c_void,
        TEST_PACKET_LEN as c_int,
        sport,
        CFG_PORT_INNER as u16,
        (*test).cfg_l3_inner,
    );

    if (*test).cfg_encap_proto == 0 {
        return il3_len + il4_len + TEST_PACKET_LEN as c_int;
    }

    match (*test).cfg_l3_outer {
        PF_INET => build_ipv4_header(
            buf.as_mut_ptr().offset(el3_len as isize) as *mut c_void,
            (*test).cfg_encap_proto as u8,
            (*test).out_saddr4.sin_addr.s_addr,
            (*test).out_daddr4.sin_addr.s_addr,
            ol4_len + il3_len + il4_len + TEST_PACKET_LEN as c_int,
            (*test).cfg_dsfield_outer,
        ),
        PF_INET6 => build_ipv6_header(
            buf.as_mut_ptr().offset(el3_len as isize) as *mut c_void,
            (*test).cfg_encap_proto as u8,
            &(*test).out_saddr6,
            &(*test).out_daddr6,
            ol4_len + il3_len + il4_len + TEST_PACKET_LEN as c_int,
            (*test).cfg_dsfield_outer,
        ),
        _ => {}
    }

    match (*test).cfg_encap_proto {
        IPPROTO_UDP => {
            build_gue_header(
                buf.as_mut_ptr().offset((el3_len + ol3_len + ol4_len - size_of::<guehdr>() as c_int) as isize) as *mut c_void,
                if (*test).cfg_l3_inner == PF_INET { IPPROTO_IPIP as u8 } else { IPPROTO_IPV6 as u8 },
            );
            build_udp_header(
                buf.as_mut_ptr().offset((el3_len + ol3_len) as isize) as *mut c_void,
                size_of::<guehdr>() as c_int + il3_len + il4_len + TEST_PACKET_LEN as c_int,
                sport,
                CFG_PORT_GUE as u16,
                (*test).cfg_l3_outer,
            );
        }
        IPPROTO_GRE => build_gre_header(
            buf.as_mut_ptr().offset((el3_len + ol3_len) as isize) as *mut c_void,
            if (*test).cfg_l3_inner == PF_INET { ETH_P_IP } else { ETH_P_IPV6 },
        ),
        _ => {}
    }

    match (*test).cfg_l3_extra {
        PF_INET => build_ipv4_header(
            buf.as_mut_ptr() as *mut c_void,
            if (*test).cfg_l3_outer == PF_INET { IPPROTO_IPIP as u8 } else { IPPROTO_IPV6 as u8 },
            (*test).extra_saddr4.sin_addr.s_addr,
            (*test).extra_daddr4.sin_addr.s_addr,
            ol3_len + ol4_len + il3_len + il4_len + TEST_PACKET_LEN as c_int,
            0,
        ),
        PF_INET6 => build_ipv6_header(
            buf.as_mut_ptr() as *mut c_void,
            if (*test).cfg_l3_outer == PF_INET { IPPROTO_IPIP as u8 } else { IPPROTO_IPV6 as u8 },
            &(*test).extra_saddr6,
            &(*test).extra_daddr6,
            ol3_len + ol4_len + il3_len + il4_len + TEST_PACKET_LEN as c_int,
            0,
        ),
        _ => {}
    }

    el3_len + ol3_len + ol4_len + il3_len + il4_len + TEST_PACKET_LEN as c_int
}

fn size_of_val_buf() -> usize {
    ETH_DATA_LEN
}

/* sender transmits encapsulated over RAW or unencap'd over UDP */
unsafe fn setup_tx(test: *const test_configuration) -> c_int {
    let family;
    let fd;
    let ret;

    if (*test).cfg_l3_extra != 0 {
        family = (*test).cfg_l3_extra;
    } else if (*test).cfg_l3_outer != 0 {
        family = (*test).cfg_l3_outer;
    } else {
        family = (*test).cfg_l3_inner;
    }

    fd = socket(family, SOCK_RAW, IPPROTO_RAW);
    if !ASSERT_OK_FD!(fd, "setup tx socket") {
        return fd;
    }

    if (*test).cfg_l3_extra != 0 {
        if (*test).cfg_l3_extra == PF_INET {
            ret = connect(fd, &(*test).extra_daddr4 as *const _ as *const c_void, size_of::<sockaddr_in>() as u32);
        } else {
            ret = connect(fd, &(*test).extra_daddr6 as *const _ as *const c_void, size_of::<sockaddr_in6>() as u32);
        }
        if !ASSERT_OK!(ret, "connect") {
            close(fd);
            return ret;
        }
    } else if (*test).cfg_l3_outer != 0 {
        /* connect to destination if not encapsulated */
        if (*test).cfg_l3_outer == PF_INET {
            ret = connect(fd, &(*test).out_daddr4 as *const _ as *const c_void, size_of::<sockaddr_in>() as u32);
        } else {
            ret = connect(fd, &(*test).out_daddr6 as *const _ as *const c_void, size_of::<sockaddr_in6>() as u32);
        }
        if !ASSERT_OK!(ret, "connect") {
            close(fd);
            return ret;
        }
    } else {
        /* otherwise using loopback */
        if (*test).cfg_l3_inner == PF_INET {
            ret = connect(fd, &(*test).in_daddr4 as *const _ as *const c_void, size_of::<sockaddr_in>() as u32);
        } else {
            ret = connect(fd, &(*test).in_daddr6 as *const _ as *const c_void, size_of::<sockaddr_in6>() as u32);
        }
        if !ASSERT_OK!(ret, "connect") {
            close(fd);
            return ret;
        }
    }

    fd
}

/* receiver reads unencapsulated UDP */
unsafe fn setup_rx(test: *const test_configuration) -> c_int {
    let fd;
    let ret;

    fd = socket((*test).cfg_l3_inner, SOCK_DGRAM, 0);
    if !ASSERT_OK_FD!(fd, "socket rx") {
        return fd;
    }

    if (*test).cfg_l3_inner == PF_INET {
        ret = bind(fd, &(*test).in_daddr4 as *const _ as *const c_void, size_of::<sockaddr_in>() as u32);
    } else {
        ret = bind(fd, &(*test).in_daddr6 as *const _ as *const c_void, size_of::<sockaddr_in6>() as u32);
    }
    if !ASSERT_OK!(ret, "bind rx") {
        close(fd);
        return ret;
    }

    fd
}

unsafe fn do_tx(fd: c_int, pkt: *const c_char, len: c_int) -> c_int {
    let ret = write(fd, pkt as *const c_void, len as usize);
    (ret != len as isize) as c_int
}

unsafe fn do_poll(fd: c_int, events: c_short, timeout: c_int) -> c_int {
    let mut pfd: pollfd = zeroed();
    let ret;

    pfd.fd = fd;
    pfd.events = events;

    ret = poll(&mut pfd, 1, timeout);
    ret
}

unsafe fn do_rx(fd: c_int) -> c_int {
    let mut rbuf: c_char = 0;
    let mut num = 0;

    loop {
        let ret = recv(fd, &mut rbuf as *mut _ as *mut c_void, 1, MSG_DONTWAIT);
        if ret == -1 && errno == EAGAIN {
            break;
        }
        if ret < 0 {
            return -1;
        }
        if !ASSERT_EQ!(rbuf, TEST_PACKET_PATTERN, "check pkt pattern") {
            return -1;
        }
        num += 1;
    }

    num
}

unsafe fn run_test(test: *const test_configuration, source_port_index: c_int) -> c_int {
    let mut fdt = -1;
    let mut fdr = -1;
    let len;
    let mut tx = 0;
    let mut rx = 0;
    let mut err;
    let mut tstop: c_ulong;
    let mut tcur: c_ulong;

    fdr = setup_rx(test);
    fdt = setup_tx(test);
    if !ASSERT_OK_FD!(fdr, "setup rx") || !ASSERT_OK_FD!(fdt, "setup tx") {
        rx = -1;
        close(fdt);
        close(fdr);
        return rx;
    }

    len = build_packet(test, (*test).source_ports[source_port_index as usize] as u16);
    if !ASSERT_GT!(len, 0, "build test packet") {
        return -1;
    }

    tcur = util_gettime();
    tstop = tcur;

    while tx < TEST_PACKETS_COUNT {
        if !ASSERT_OK!(do_tx(fdt, buf.as_ptr(), len), "do_tx") {
            break;
        }
        tx += 1;
        err = do_rx(fdr);
        if !ASSERT_GE!(err, 0, "do_rx") {
            break;
        }
        rx += err;
    }

    /* read straggler packets, if any */
    if rx < tx {
        tstop = util_gettime() + 100;
        while rx < tx {
            tcur = util_gettime();
            if tcur >= tstop {
                break;
            }

            err = do_poll(fdr, POLLIN, (tstop - tcur) as c_int);
            if err < 0 {
                break;
            }
            err = do_rx(fdr);
            if err >= 0 {
                rx += err;
            }
        }
    }

    close(fdt);
    close(fdr);
    rx
}

unsafe fn attach_and_configure_program(skel: *mut bpf_flow) -> c_int {
    let prog_array = (*skel).maps.jmp_table;
    let main_prog_fd;
    let mut sub_prog_fd;
    let map_fd;
    let mut i;
    let mut err;
    let mut prog: *mut bpf_program;
    let mut prog_name: [c_char; 32] = [0; 32];

    main_prog_fd = bpf_program__fd((*skel).progs._dissect);
    if main_prog_fd < 0 {
        return main_prog_fd;
    }

    err = bpf_prog_attach(main_prog_fd, 0, BPF_FLOW_DISSECTOR, 0);
    if err != 0 {
        return err;
    }

    map_fd = bpf_map__fd(prog_array);
    if map_fd < 0 {
        return map_fd;
    }

    i = 0;
    while i < bpf_map__max_entries(prog_array) {
        snprintf(prog_name.as_mut_ptr(), prog_name.len(), b"flow_dissector_%d\0".as_ptr() as *const c_char, i);

        prog = bpf_object__find_program_by_name((*skel).obj, prog_name.as_ptr());
        if prog.is_null() {
            return -1;
        }

        sub_prog_fd = bpf_program__fd(prog);
        if sub_prog_fd < 0 {
            return -1;
        }

        err = bpf_map_update_elem(map_fd, &i as *const _ as *const c_void, &sub_prog_fd as *const _ as *const c_void, BPF_ANY);
        if err != 0 {
            return -1;
        }
        i += 1;
    }

    main_prog_fd
}

unsafe fn detach_program(_skel: *mut bpf_flow, prog_fd: c_int) {
    bpf_prog_detach2(prog_fd, 0, BPF_FLOW_DISSECTOR);
}

unsafe fn set_port_drop(pf: c_int, multi_port: bool) -> c_int {
    let mut dst_port: [c_char; 16] = [0; 16];

    snprintf(dst_port.as_mut_ptr(), dst_port.len(), b"%d\0".as_ptr() as *const c_char, CFG_PORT_INNER);

    SYS!(fail, "tc qdisc add dev lo ingress");
    SYS!(
        fail_delete_qdisc,
        "tc filter add %s %s %s %s %s %s %s %s %s %s %s %s",
        "dev lo",
        "parent FFFF:",
        "protocol",
        if pf == PF_INET6 { "ipv6" } else { "ip" },
        "pref 1337",
        "flower",
        "ip_proto udp",
        "src_port",
        if multi_port { "8-10" } else { "9" },
        "dst_port",
        dst_port.as_ptr(),
        "action drop",
    );
    return 0;

    #[allow(unreachable_code)]
    {
        SYS_NOFAIL!("tc qdisc del dev lo ingress");
        1
    }
}

unsafe fn remove_filter() {
    SYS_NOFAIL!("tc filter del dev lo ingress");
    SYS_NOFAIL!("tc qdisc del dev lo ingress");
}

unsafe extern "C" fn ipv4_setup() -> c_int {
    set_port_drop(PF_INET, false)
}

unsafe extern "C" fn ipv6_setup() -> c_int {
    set_port_drop(PF_INET6, false)
}

unsafe extern "C" fn port_range_setup() -> c_int {
    set_port_drop(PF_INET, true)
}

unsafe fn set_addresses() -> c_int {
    SYS!(out, "ip -4 addr add  %s dev lo", TEST_IPV4);
    SYS!(out_remove_ipv4, "ip -6 addr add %s dev lo", TEST_IPV6);
    return 0;

    #[allow(unreachable_code)]
    {
        SYS_NOFAIL!("ip -4 addr del %s dev lo", TEST_IPV4);
        -1
    }
}

unsafe fn unset_addresses() {
    SYS_NOFAIL!("ip -4 addr del %s dev lo", TEST_IPV4);
    SYS_NOFAIL!("ip -6 addr del %s dev lo", TEST_IPV6);
}

unsafe extern "C" fn ipip_setup() -> c_int {
    if !ASSERT_OK!(set_addresses(), "configure addresses") {
        return -1;
    }
    if !ASSERT_OK!(set_port_drop(PF_INET, false), "set filter") {
        unset_addresses();
        return -1;
    }
    SYS!(
        out_remove_filter,
        "ip link add ipip_test type ipip remote %s local %s dev lo",
        TEST_TUNNEL_REMOTE,
        TEST_TUNNEL_LOCAL,
    );
    SYS!(out_clean_netif, "ip link set ipip_test up");
    0
}

unsafe extern "C" fn ipip_shutdown() {
    SYS_NOFAIL!("ip link del ipip_test");
    remove_filter();
    unset_addresses();
}

unsafe extern "C" fn gre_setup() -> c_int {
    if !ASSERT_OK!(set_addresses(), "configure addresses") {
        return -1;
    }
    if !ASSERT_OK!(set_port_drop(PF_INET, false), "set filter") {
        unset_addresses();
        return -1;
    }
    SYS!(
        out_remove_filter,
        "ip link add gre_test type gre remote %s local %s dev lo",
        TEST_TUNNEL_REMOTE,
        TEST_TUNNEL_LOCAL,
    );
    SYS!(out_clean_netif, "ip link set gre_test up");
    0
}

unsafe extern "C" fn gre_shutdown() {
    SYS_NOFAIL!("ip link del gre_test");
    remove_filter();
    unset_addresses();
}

const fn c_name(bytes: &[u8; SUBTEST_NAME_MAX_LEN]) -> [c_char; SUBTEST_NAME_MAX_LEN] {
    let mut out = [0 as c_char; SUBTEST_NAME_MAX_LEN];
    let mut i = 0;
    while i < SUBTEST_NAME_MAX_LEN {
        out[i] = bytes[i] as c_char;
        i += 1;
    }
    out
}

const IPV4_NAME: [c_char; SUBTEST_NAME_MAX_LEN] = c_name(b"ipv4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
const IPV4_CONTINUE_DISSECT_NAME: [c_char; SUBTEST_NAME_MAX_LEN] = c_name(b"ipv4_continue_dissect\0\0\0\0\0\0\0\0\0\0");
const IPIP_NAME: [c_char; SUBTEST_NAME_MAX_LEN] = c_name(b"ipip\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
const GRE_NAME: [c_char; SUBTEST_NAME_MAX_LEN] = c_name(b"gre\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
const PORT_RANGE_NAME: [c_char; SUBTEST_NAME_MAX_LEN] = c_name(b"port_range\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
const IPV6_NAME: [c_char; SUBTEST_NAME_MAX_LEN] = c_name(b"ipv6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");

static tests_input: [test_configuration; 6] = [
    test_configuration {
        name: IPV4_NAME,
        test_setup: Some(ipv4_setup),
        test_teardown: Some(remove_filter),
        source_ports: [8, 9, 10],
        cfg_l3_inner: PF_INET,
        in_saddr4: TEST_IN4_SRC_ADDR_DEFAULT,
        in_daddr4: TEST_IN4_DST_ADDR_DEFAULT,
        in_saddr6: TEST_IN6_SRC_ADDR_DEFAULT,
        in_daddr6: TEST_IN6_DST_ADDR_DEFAULT,
        cfg_l3_outer: 0,
        out_saddr4: TEST_OUT4_SRC_ADDR_DEFAULT,
        out_daddr4: TEST_OUT4_DST_ADDR_DEFAULT,
        out_saddr6: TEST_OUT6_SRC_ADDR_DEFAULT,
        out_daddr6: TEST_OUT6_DST_ADDR_DEFAULT,
        cfg_encap_proto: 0,
        cfg_dsfield_inner: 0,
        cfg_dsfield_outer: 0,
        cfg_l3_extra: 0,
        extra_saddr4: TEST_OUT4_SRC_ADDR_DEFAULT,
        extra_daddr4: TEST_OUT4_DST_ADDR_DEFAULT,
        extra_saddr6: TEST_OUT6_SRC_ADDR_DEFAULT,
        extra_daddr6: TEST_OUT6_DST_ADDR_DEFAULT,
    },
    test_configuration {
        name: IPV4_CONTINUE_DISSECT_NAME,
        test_setup: Some(ipv4_setup),
        test_teardown: Some(remove_filter),
        source_ports: [8, 9, 10],
        cfg_l3_inner: PF_INET,
        in_saddr4: TEST_IN4_SRC_ADDR_DISSECT_CONTINUE,
        in_daddr4: TEST_IN4_DST_ADDR_DEFAULT,
        in_saddr6: TEST_IN6_SRC_ADDR_DEFAULT,
        in_daddr6: TEST_IN6_DST_ADDR_DEFAULT,
        cfg_l3_outer: 0,
        out_saddr4: TEST_OUT4_SRC_ADDR_DEFAULT,
        out_daddr4: TEST_OUT4_DST_ADDR_DEFAULT,
        out_saddr6: TEST_OUT6_SRC_ADDR_DEFAULT,
        out_daddr6: TEST_OUT6_DST_ADDR_DEFAULT,
        cfg_encap_proto: 0,
        cfg_dsfield_inner: 0,
        cfg_dsfield_outer: 0,
        cfg_l3_extra: 0,
        extra_saddr4: TEST_OUT4_SRC_ADDR_DEFAULT,
        extra_daddr4: TEST_OUT4_DST_ADDR_DEFAULT,
        extra_saddr6: TEST_OUT6_SRC_ADDR_DEFAULT,
        extra_daddr6: TEST_OUT6_DST_ADDR_DEFAULT,
    },
    test_configuration {
        name: IPIP_NAME,
        test_setup: Some(ipip_setup),
        test_teardown: Some(ipip_shutdown),
        source_ports: [8, 9, 10],
        cfg_l3_inner: PF_INET,
        in_saddr4: TEST_IN4_SRC_ADDR_IPIP,
        in_daddr4: TEST_IN4_DST_ADDR_IPIP,
        in_saddr6: TEST_IN6_SRC_ADDR_DEFAULT,
        in_daddr6: TEST_IN6_DST_ADDR_DEFAULT,
        cfg_l3_outer: PF_INET,
        out_saddr4: TEST_OUT4_SRC_ADDR_DEFAULT,
        out_daddr4: TEST_OUT4_DST_ADDR_DEFAULT,
        out_saddr6: TEST_OUT6_SRC_ADDR_DEFAULT,
        out_daddr6: TEST_OUT6_DST_ADDR_DEFAULT,
        cfg_encap_proto: IPPROTO_IPIP,
        cfg_dsfield_inner: 0,
        cfg_dsfield_outer: 0,
        cfg_l3_extra: 0,
        extra_saddr4: TEST_OUT4_SRC_ADDR_DEFAULT,
        extra_daddr4: TEST_OUT4_DST_ADDR_DEFAULT,
        extra_saddr6: TEST_OUT6_SRC_ADDR_DEFAULT,
        extra_daddr6: TEST_OUT6_DST_ADDR_DEFAULT,
    },
    test_configuration {
        name: GRE_NAME,
        test_setup: Some(gre_setup),
        test_teardown: Some(gre_shutdown),
        source_ports: [8, 9, 10],
        cfg_l3_inner: PF_INET,
        in_saddr4: TEST_IN4_SRC_ADDR_IPIP,
        in_daddr4: TEST_IN4_DST_ADDR_IPIP,
        in_saddr6: TEST_IN6_SRC_ADDR_DEFAULT,
        in_daddr6: TEST_IN6_DST_ADDR_DEFAULT,
        cfg_l3_outer: PF_INET,
        out_saddr4: TEST_OUT4_SRC_ADDR_DEFAULT,
        out_daddr4: TEST_OUT4_DST_ADDR_DEFAULT,
        out_saddr6: TEST_OUT6_SRC_ADDR_DEFAULT,
        out_daddr6: TEST_OUT6_DST_ADDR_DEFAULT,
        cfg_encap_proto: IPPROTO_GRE,
        cfg_dsfield_inner: 0,
        cfg_dsfield_outer: 0,
        cfg_l3_extra: 0,
        extra_saddr4: TEST_OUT4_SRC_ADDR_DEFAULT,
        extra_daddr4: TEST_OUT4_DST_ADDR_DEFAULT,
        extra_saddr6: TEST_OUT6_SRC_ADDR_DEFAULT,
        extra_daddr6: TEST_OUT6_DST_ADDR_DEFAULT,
    },
    test_configuration {
        name: PORT_RANGE_NAME,
        test_setup: Some(port_range_setup),
        test_teardown: Some(remove_filter),
        source_ports: [7, 9, 11],
        cfg_l3_inner: PF_INET,
        in_saddr4: TEST_IN4_SRC_ADDR_DEFAULT,
        in_daddr4: TEST_IN4_DST_ADDR_DEFAULT,
        in_saddr6: TEST_IN6_SRC_ADDR_DEFAULT,
        in_daddr6: TEST_IN6_DST_ADDR_DEFAULT,
        cfg_l3_outer: 0,
        out_saddr4: TEST_OUT4_SRC_ADDR_DEFAULT,
        out_daddr4: TEST_OUT4_DST_ADDR_DEFAULT,
        out_saddr6: TEST_OUT6_SRC_ADDR_DEFAULT,
        out_daddr6: TEST_OUT6_DST_ADDR_DEFAULT,
        cfg_encap_proto: 0,
        cfg_dsfield_inner: 0,
        cfg_dsfield_outer: 0,
        cfg_l3_extra: 0,
        extra_saddr4: TEST_OUT4_SRC_ADDR_DEFAULT,
        extra_daddr4: TEST_OUT4_DST_ADDR_DEFAULT,
        extra_saddr6: TEST_OUT6_SRC_ADDR_DEFAULT,
        extra_daddr6: TEST_OUT6_DST_ADDR_DEFAULT,
    },
    test_configuration {
        name: IPV6_NAME,
        test_setup: Some(ipv6_setup),
        test_teardown: Some(remove_filter),
        source_ports: [8, 9, 10],
        cfg_l3_inner: PF_INET6,
        in_saddr4: TEST_IN4_SRC_ADDR_DEFAULT,
        in_daddr4: TEST_IN4_DST_ADDR_DEFAULT,
        in_saddr6: TEST_IN6_SRC_ADDR_DEFAULT,
        in_daddr6: TEST_IN6_DST_ADDR_DEFAULT,
        cfg_l3_outer: 0,
        out_saddr4: TEST_OUT4_SRC_ADDR_DEFAULT,
        out_daddr4: TEST_OUT4_DST_ADDR_DEFAULT,
        out_saddr6: TEST_OUT6_SRC_ADDR_DEFAULT,
        out_daddr6: TEST_OUT6_DST_ADDR_DEFAULT,
        cfg_encap_proto: 0,
        cfg_dsfield_inner: 0,
        cfg_dsfield_outer: 0,
        cfg_l3_extra: 0,
        extra_saddr4: TEST_OUT4_SRC_ADDR_DEFAULT,
        extra_daddr4: TEST_OUT4_DST_ADDR_DEFAULT,
        extra_saddr6: TEST_OUT6_SRC_ADDR_DEFAULT,
        extra_daddr6: TEST_OUT6_DST_ADDR_DEFAULT,
    },
];

#[repr(C)]
struct test_ctx {
    skel: *mut bpf_flow,
    ns: *mut netns_obj,
    prog_fd: c_int,
}

unsafe fn test_global_init(ctx: *mut test_ctx) -> c_int {
    let mut err: c_int;

    (*ctx).skel = bpf_flow__open_and_load();
    if !ASSERT_OK_PTR!((*ctx).skel, "open and load flow_dissector") {
        return -1;
    }

    (*ctx).ns = netns_new(b"flow_dissector_classification\0".as_ptr() as *const c_char, true);
    if !ASSERT_OK_PTR!((*ctx).ns, "switch ns") {
        bpf_flow__destroy((*ctx).skel);
        return -1;
    }

    err = write_sysctl(b"/proc/sys/net/ipv4/conf/default/rp_filter\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    err |= write_sysctl(b"/proc/sys/net/ipv4/conf/all/rp_filter\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    err |= write_sysctl(b"/proc/sys/net/ipv4/conf/lo/rp_filter\0".as_ptr() as *const c_char, b"0\0".as_ptr() as *const c_char);
    if !ASSERT_OK!(err, "configure net tunables") {
        netns_free((*ctx).ns);
        bpf_flow__destroy((*ctx).skel);
        return -1;
    }

    (*ctx).prog_fd = attach_and_configure_program((*ctx).skel);
    if !ASSERT_OK_FD!((*ctx).prog_fd, "attach and configure program") {
        netns_free((*ctx).ns);
        bpf_flow__destroy((*ctx).skel);
        return -1;
    }
    0
}

unsafe fn test_global_shutdown(ctx: *mut test_ctx) {
    detach_program((*ctx).skel, (*ctx).prog_fd);
    netns_free((*ctx).ns);
    bpf_flow__destroy((*ctx).skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_flow_dissector_classification() {
    let mut ctx: test_ctx = zeroed();
    let mut test: *const test_configuration;
    let mut i: usize;

    if test_global_init(&mut ctx) != 0 {
        return;
    }

    i = 0;
    while i < ARRAY_SIZE!(tests_input) {
        if !test__start_subtest(tests_input[i].name.as_ptr()) {
            i += 1;
            continue;
        }
        test = &tests_input[i];
        /* All tests are expected to have one rx-ok port first,
         * then a non-working rx port, and finally a rx-ok port
         */
        if let Some(setup) = (*test).test_setup {
            if !ASSERT_OK!(setup(), "init filter") {
                i += 1;
                continue;
            }
        }

        ASSERT_EQ!(run_test(test, 0), TEST_PACKETS_COUNT, "test first port");
        ASSERT_EQ!(run_test(test, 1), 0, "test second port");
        ASSERT_EQ!(run_test(test, 2), TEST_PACKETS_COUNT, "test third port");
        if let Some(teardown) = (*test).test_teardown {
            teardown();
        }
        i += 1;
    }
    test_global_shutdown(&mut ctx);
}
