// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2020 Cloudflare

// Translated from C implementation source. C includes referenced external
// system, libbpf, selftest, and skeleton definitions.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const ENCAP_IP: u32 = INADDR_LOOPBACK;
const ENCAP_PORT: u16 = 1234;

static mut duration: c_int = 0;

type socklen_t = u32;
type sa_family_t = u16;
type in_port_t = u16;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_DGRAM: c_int = 2;
const SOCK_STREAM: c_int = 1;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const ETH_P_IP: u16 = 0x0800;
const IPDEFTTL: u8 = 64;
const IPPROTO_IPIP: c_int = 4;
const IPPROTO_TCP: c_int = 6;
const IPPROTO_UDP: c_int = 17;
const IPPROTO_IPV6: c_int = 41;
const TC_ACT_REDIRECT: u32 = 7;

#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __data: [u8; 126],
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: u8,
    pub tos: u8,
    pub tot_len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: u16,
    pub saddr: u32,
    pub daddr: u32,
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: u8,
    pub flow_lbl: [u8; 3],
    pub payload_len: u16,
    pub nexthdr: u8,
    pub hop_limit: u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
pub struct tcphdr {
    pub source: u16,
    pub dest: u16,
    pub seq: u32,
    pub ack_seq: u32,
    pub doff_res_flags: u16,
    pub window: u16,
    pub check: u16,
    pub urg_ptr: u16,
}

#[repr(C)]
pub struct udphdr {
    pub source: u16,
    pub dest: u16,
    pub len: u16,
    pub check: u16,
}

#[repr(C)]
pub struct guehdr {
    pub hlen: u8,
    pub proto_ctype: u8,
    pub flags: u16,
}

#[repr(C)]
pub struct unigue {
    pub hop_count: u8,
}

#[repr(C)]
pub struct encap_headers_t {
    pub eth: ethhdr,
    pub ip: iphdr,
    pub udp: udphdr,
    pub gue: guehdr,
    pub unigue: unigue,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *mut c_void,
    pub data_out: *mut c_void,
    pub data_size_in: u32,
    pub data_size_out: u32,
    pub retval: u32,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_cls_redirect_rodata {
    pub ENCAPSULATION_IP: u32,
    pub ENCAPSULATION_PORT: u16,
}

#[repr(C)]
pub struct test_cls_redirect_progs {
    pub cls_redirect: *mut bpf_program,
}

#[repr(C)]
pub struct test_cls_redirect {
    pub rodata: *mut test_cls_redirect_rodata,
    pub progs: test_cls_redirect_progs,
}

#[repr(C)]
pub struct test_cls_redirect_dynptr {
    pub rodata: *mut test_cls_redirect_rodata,
    pub progs: test_cls_redirect_progs,
}

#[repr(C)]
pub struct test_cls_redirect_subprogs {
    pub rodata: *mut test_cls_redirect_rodata,
    pub progs: test_cls_redirect_progs,
}

unsafe extern "C" {
    static in6addr_loopback: in6_addr;

    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn getpeername(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn htonl(hostlong: u32) -> u32;
    fn htons(hostshort: u16) -> u16;
    fn mempcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;

    fn start_server_addr(
        type_: c_int,
        addr: *const sockaddr_storage,
        len: socklen_t,
        opts: *mut c_void,
    ) -> c_int;
    fn connect_to_addr(
        type_: c_int,
        addr: *const sockaddr_storage,
        len: socklen_t,
        opts: *mut c_void,
    ) -> c_int;

    fn CHECK_FAIL(condition: bool) -> bool;
    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn PRINT_FAIL(fmt: *const c_char, ...) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn test_cls_redirect__open() -> *mut test_cls_redirect;
    fn test_cls_redirect__load(skel: *mut test_cls_redirect) -> c_int;
    fn test_cls_redirect__destroy(skel: *mut test_cls_redirect);

    fn test_cls_redirect_dynptr__open() -> *mut test_cls_redirect_dynptr;
    fn test_cls_redirect_dynptr__load(skel: *mut test_cls_redirect_dynptr) -> c_int;
    fn test_cls_redirect_dynptr__destroy(skel: *mut test_cls_redirect_dynptr);

    fn test_cls_redirect_subprogs__open() -> *mut test_cls_redirect_subprogs;
    fn test_cls_redirect_subprogs__load(skel: *mut test_cls_redirect_subprogs) -> c_int;
    fn test_cls_redirect_subprogs__destroy(skel: *mut test_cls_redirect_subprogs);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum type_ {
    UDP,
    TCP,
    __NR_KIND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum hops {
    NO_HOPS,
    ONE_HOP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum flags {
    NONE,
    SYN,
    ACK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum conn {
    KNOWN_CONN,
    UNKNOWN_CONN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum result {
    ACCEPT,
    FORWARD,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct test_cfg {
    type_: type_,
    result: result,
    conn: conn,
    hops: hops,
    flags: flags,
}

static mut tests: [test_cfg; 7] = [
    test_cfg { type_: type_::TCP, result: result::ACCEPT, conn: conn::UNKNOWN_CONN, hops: hops::NO_HOPS, flags: flags::SYN },
    test_cfg { type_: type_::TCP, result: result::ACCEPT, conn: conn::UNKNOWN_CONN, hops: hops::NO_HOPS, flags: flags::ACK },
    test_cfg { type_: type_::TCP, result: result::FORWARD, conn: conn::UNKNOWN_CONN, hops: hops::ONE_HOP, flags: flags::ACK },
    test_cfg { type_: type_::TCP, result: result::ACCEPT, conn: conn::KNOWN_CONN, hops: hops::ONE_HOP, flags: flags::ACK },
    test_cfg { type_: type_::UDP, result: result::ACCEPT, conn: conn::UNKNOWN_CONN, hops: hops::NO_HOPS, flags: flags::NONE },
    test_cfg { type_: type_::UDP, result: result::FORWARD, conn: conn::UNKNOWN_CONN, hops: hops::ONE_HOP, flags: flags::NONE },
    test_cfg { type_: type_::UDP, result: result::ACCEPT, conn: conn::KNOWN_CONN, hops: hops::ONE_HOP, flags: flags::NONE },
];

unsafe fn set_up_conn(
    addr: *const sockaddr_storage,
    len: socklen_t,
    type_: c_int,
    server: *mut c_int,
    conn: *mut c_int,
    src: *mut sockaddr_storage,
    dst: *mut sockaddr_storage,
) -> bool {
    let mut ss: sockaddr_storage = zeroed();
    let mut slen: socklen_t = size_of::<sockaddr_storage>() as socklen_t;

    *server = start_server_addr(type_, addr, len, ptr::null_mut());
    if *server < 0 {
        return false;
    }

    if CHECK_FAIL(getsockname(*server, &mut ss as *mut _ as *mut sockaddr, &mut slen) != 0) {
        close(*server);
        *server = -1;
        return false;
    }

    *conn = connect_to_addr(type_, &ss, slen, ptr::null_mut());
    if *conn < 0 {
        close(*server);
        *server = -1;
        return false;
    }

    /* We want to simulate packets arriving at conn, so we have to
     * swap src and dst.
     */
    slen = size_of::<sockaddr_storage>() as socklen_t;
    if CHECK_FAIL(getsockname(*conn, dst as *mut sockaddr, &mut slen) != 0) {
        close(*conn);
        *conn = -1;
        close(*server);
        *server = -1;
        return false;
    }

    slen = size_of::<sockaddr_storage>() as socklen_t;
    if CHECK_FAIL(getpeername(*conn, src as *mut sockaddr, &mut slen) != 0) {
        close(*conn);
        *conn = -1;
        close(*server);
        *server = -1;
        return false;
    }

    true
}

unsafe fn prepare_addr(addr: *mut sockaddr_storage, family: c_int) -> socklen_t {
    memset(addr as *mut c_void, 0, size_of::<sockaddr_storage>());

    match family {
        AF_INET => {
            let addr4 = addr as *mut sockaddr_in;
            (*addr4).sin_family = family as sa_family_t;
            (*addr4).sin_addr.s_addr = htonl(INADDR_LOOPBACK);
            size_of::<sockaddr_in>() as socklen_t
        }
        AF_INET6 => {
            let addr6 = addr as *mut sockaddr_in6;
            (*addr6).sin6_family = family as sa_family_t;
            (*addr6).sin6_addr = in6addr_loopback;
            size_of::<sockaddr_in6>() as socklen_t
        }
        _ => {
            fprintf(stderr, b"Invalid family %d\0".as_ptr() as *const c_char, family);
            0
        }
    }
}

unsafe fn was_decapsulated(tattr: *mut bpf_test_run_opts) -> bool {
    (*tattr).data_size_out < (*tattr).data_size_in
}

unsafe fn test_str(buf: *mut c_void, len: usize, test: *const test_cfg, family: c_int) -> c_int {
    let mut family_str = b"IPv4\0".as_ptr() as *const c_char;
    let mut type_str = b"TCP\0".as_ptr() as *const c_char;
    let mut conn_str = b"known\0".as_ptr() as *const c_char;
    let mut hops_str = b"no hops\0".as_ptr() as *const c_char;
    let mut result_str = b"accept\0".as_ptr() as *const c_char;
    let mut flags_str = b"none\0".as_ptr() as *const c_char;

    if family == AF_INET6 {
        family_str = b"IPv6\0".as_ptr() as *const c_char;
    }
    if (*test).type_ == type_::UDP {
        type_str = b"UDP\0".as_ptr() as *const c_char;
    }
    if (*test).conn == conn::UNKNOWN_CONN {
        conn_str = b"unknown\0".as_ptr() as *const c_char;
    }
    if (*test).hops == hops::ONE_HOP {
        hops_str = b"one hop\0".as_ptr() as *const c_char;
    }
    if (*test).result == result::FORWARD {
        result_str = b"forward\0".as_ptr() as *const c_char;
    }
    if (*test).flags == flags::SYN {
        flags_str = b"SYN\0".as_ptr() as *const c_char;
    } else if (*test).flags == flags::ACK {
        flags_str = b"ACK\0".as_ptr() as *const c_char;
    }

    snprintf(
        buf as *mut c_char,
        len,
        b"%s %s %s %s (%s, flags: %s)\0".as_ptr() as *const c_char,
        family_str,
        type_str,
        result_str,
        conn_str,
        hops_str,
        flags_str,
    )
}

unsafe fn encap_init(encap: *mut encap_headers_t, hop_count: u8, proto: u8) {
    let hlen: u8 = (size_of::<guehdr>() / size_of::<u32>()) as u8 + hop_count;
    *encap = encap_headers_t {
        eth: ethhdr {
            h_dest: [0; 6],
            h_source: [0; 6],
            h_proto: htons(ETH_P_IP),
        },
        ip: iphdr {
            ihl_version: (4 << 4) | 5,
            tos: 0,
            tot_len: 0,
            id: 0,
            frag_off: 0,
            ttl: IPDEFTTL,
            protocol: IPPROTO_UDP as u8,
            check: 0,
            saddr: 0,
            daddr: htonl(ENCAP_IP),
        },
        udp: udphdr {
            source: 0,
            dest: htons(ENCAP_PORT),
            len: 0,
            check: 0,
        },
        gue: guehdr {
            hlen,
            proto_ctype: proto,
            flags: 0,
        },
        unigue: unigue { hop_count },
    };
}

unsafe fn build_input(
    test: *const test_cfg,
    buf: *mut c_void,
    src: *const sockaddr_storage,
    dst: *const sockaddr_storage,
) -> usize {
    let src_in6 = src as *const sockaddr_in6;
    let dst_in6 = dst as *const sockaddr_in6;
    let src_in = src as *const sockaddr_in;
    let dst_in = dst as *const sockaddr_in;
    let family: sa_family_t = (*src).ss_family;
    let mut sport: in_port_t;
    let dport: in_port_t;
    let mut encap: encap_headers_t = zeroed();
    let mut ip: iphdr;
    let mut ipv6: ipv6hdr;
    let mut tcp: tcphdr;
    let mut udp: udphdr;
    let mut next_hop: in_addr;
    let mut p = buf as *mut u8;
    let mut proto: c_int;

    sport = if family as c_int == AF_INET { (*src_in).sin_port } else { (*src_in6).sin6_port };
    dport = if family as c_int == AF_INET { (*dst_in).sin_port } else { (*dst_in6).sin6_port };

    proto = IPPROTO_IPIP;
    if family as c_int == AF_INET6 {
        proto = IPPROTO_IPV6;
    }

    encap_init(
        &mut encap,
        if (*test).hops == hops::ONE_HOP { 1 } else { 0 },
        proto as u8,
    );
    p = mempcpy(p as *mut c_void, &encap as *const _ as *const c_void, size_of::<encap_headers_t>()) as *mut u8;

    if (*test).hops == hops::ONE_HOP {
        next_hop = in_addr { s_addr: htonl(0x7f000002) };
        p = mempcpy(p as *mut c_void, &next_hop as *const _ as *const c_void, size_of::<in_addr>()) as *mut u8;
    }

    proto = IPPROTO_TCP;
    if (*test).type_ == type_::UDP {
        proto = IPPROTO_UDP;
    }

    match family as c_int {
        AF_INET => {
            ip = iphdr {
                ihl_version: (4 << 4) | 5,
                tos: 0,
                tot_len: 0,
                id: 0,
                frag_off: 0,
                ttl: IPDEFTTL,
                protocol: proto as u8,
                check: 0,
                saddr: (*src_in).sin_addr.s_addr,
                daddr: (*dst_in).sin_addr.s_addr,
            };
            p = mempcpy(p as *mut c_void, &ip as *const _ as *const c_void, size_of::<iphdr>()) as *mut u8;
        }
        AF_INET6 => {
            ipv6 = ipv6hdr {
                priority_version: 6 << 4,
                flow_lbl: [0; 3],
                payload_len: 0,
                nexthdr: proto as u8,
                hop_limit: IPDEFTTL,
                saddr: (*src_in6).sin6_addr,
                daddr: (*dst_in6).sin6_addr,
            };
            p = mempcpy(p as *mut c_void, &ipv6 as *const _ as *const c_void, size_of::<ipv6hdr>()) as *mut u8;
        }
        _ => return 0,
    }

    if (*test).conn == conn::UNKNOWN_CONN {
        sport = sport.wrapping_sub(1);
    }

    match (*test).type_ {
        type_::TCP => {
            tcp = zeroed();
            tcp.source = sport;
            tcp.dest = dport;
            if (*test).flags == flags::SYN {
                tcp.doff_res_flags |= (1u16 << 1).to_be();
            }
            if (*test).flags == flags::ACK {
                tcp.doff_res_flags |= (1u16 << 4).to_be();
            }
            p = mempcpy(p as *mut c_void, &tcp as *const _ as *const c_void, size_of::<tcphdr>()) as *mut u8;
        }
        type_::UDP => {
            udp = udphdr {
                source: sport,
                dest: dport,
                len: 0,
                check: 0,
            };
            p = mempcpy(p as *mut c_void, &udp as *const _ as *const c_void, size_of::<udphdr>()) as *mut u8;
        }
        _ => return 0,
    }

    p.offset_from(buf as *mut u8) as usize
}

unsafe fn close_fds(fds: *mut c_int, n: c_int) {
    let mut i: c_int = 0;

    while i < n {
        if *fds.offset(i as isize) > 0 {
            close(*fds.offset(i as isize));
        }
        i += 1;
    }
}

unsafe fn test_cls_redirect_common(prog: *mut bpf_program) {
    let mut tattr = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: ptr::null_mut(),
        data_out: ptr::null_mut(),
        data_size_in: 0,
        data_size_out: 0,
        retval: 0,
    };
    let families: [c_int; 2] = [AF_INET, AF_INET6];
    let mut ss: sockaddr_storage = zeroed();
    let mut slen: socklen_t;
    let mut i: usize;
    let mut j: usize;
    let mut err: c_int;
    let prog_fd: c_int;
    let mut servers: [[c_int; 2]; type_::__NR_KIND as usize] = [[0; 2]; type_::__NR_KIND as usize];
    let mut conns: [[c_int; 2]; type_::__NR_KIND as usize] = [[0; 2]; type_::__NR_KIND as usize];
    let mut srcs: [[sockaddr_storage; 2]; type_::__NR_KIND as usize] = zeroed();
    let mut dsts: [[sockaddr_storage; 2]; type_::__NR_KIND as usize] = zeroed();

    i = 0;
    while i < families.len() {
        slen = prepare_addr(&mut ss, families[i]);
        if CHECK_FAIL(slen == 0) {
            close_fds(servers.as_mut_ptr() as *mut c_int, (size_of_val(&servers) / size_of::<c_int>()) as c_int);
            close_fds(conns.as_mut_ptr() as *mut c_int, (size_of_val(&conns) / size_of::<c_int>()) as c_int);
            return;
        }

        if CHECK_FAIL(!set_up_conn(
            &ss,
            slen,
            SOCK_DGRAM,
            &mut servers[type_::UDP as usize][i],
            &mut conns[type_::UDP as usize][i],
            &mut srcs[type_::UDP as usize][i],
            &mut dsts[type_::UDP as usize][i],
        )) {
            close_fds(servers.as_mut_ptr() as *mut c_int, (size_of_val(&servers) / size_of::<c_int>()) as c_int);
            close_fds(conns.as_mut_ptr() as *mut c_int, (size_of_val(&conns) / size_of::<c_int>()) as c_int);
            return;
        }

        if CHECK_FAIL(!set_up_conn(
            &ss,
            slen,
            SOCK_STREAM,
            &mut servers[type_::TCP as usize][i],
            &mut conns[type_::TCP as usize][i],
            &mut srcs[type_::TCP as usize][i],
            &mut dsts[type_::TCP as usize][i],
        )) {
            close_fds(servers.as_mut_ptr() as *mut c_int, (size_of_val(&servers) / size_of::<c_int>()) as c_int);
            close_fds(conns.as_mut_ptr() as *mut c_int, (size_of_val(&conns) / size_of::<c_int>()) as c_int);
            return;
        }
        i += 1;
    }

    prog_fd = bpf_program__fd(prog);
    i = 0;
    while i < tests.len() {
        let test: *mut test_cfg = &mut tests[i];

        j = 0;
        while j < families.len() {
            let src: *mut sockaddr_storage = &mut srcs[(*test).type_ as usize][j];
            let dst: *mut sockaddr_storage = &mut dsts[(*test).type_ as usize][j];
            let mut input: [c_char; 256] = [0; 256];
            let mut tmp: [c_char; 256] = [0; 256];

            test_str(tmp.as_mut_ptr() as *mut c_void, size_of_val(&tmp), test, families[j]);
            if !test__start_subtest(tmp.as_ptr()) {
                j += 1;
                continue;
            }

            tattr.data_out = tmp.as_mut_ptr() as *mut c_void;
            tattr.data_size_out = size_of_val(&tmp) as u32;

            tattr.data_in = input.as_mut_ptr() as *mut c_void;
            tattr.data_size_in = build_input(test, input.as_mut_ptr() as *mut c_void, src, dst) as u32;
            if CHECK_FAIL(tattr.data_size_in == 0) {
                j += 1;
                continue;
            }

            err = bpf_prog_test_run_opts(prog_fd, &mut tattr);
            if CHECK_FAIL(err != 0) {
                j += 1;
                continue;
            }

            if tattr.retval != TC_ACT_REDIRECT {
                PRINT_FAIL(
                    b"expected TC_ACT_REDIRECT, got %d\n\0".as_ptr() as *const c_char,
                    tattr.retval,
                );
                j += 1;
                continue;
            }

            match (*test).result {
                result::ACCEPT => {
                    if CHECK_FAIL(!was_decapsulated(&mut tattr)) {
                        j += 1;
                        continue;
                    }
                }
                result::FORWARD => {
                    if CHECK_FAIL(was_decapsulated(&mut tattr)) {
                        j += 1;
                        continue;
                    }
                }
            }
            j += 1;
        }
        i += 1;
    }

    close_fds(servers.as_mut_ptr() as *mut c_int, (size_of_val(&servers) / size_of::<c_int>()) as c_int);
    close_fds(conns.as_mut_ptr() as *mut c_int, (size_of_val(&conns) / size_of::<c_int>()) as c_int);
}

unsafe fn test_cls_redirect_dynptr() {
    let skel: *mut test_cls_redirect_dynptr;
    let err: c_int;

    skel = test_cls_redirect_dynptr__open();
    if !ASSERT_OK_PTR(skel as *mut c_void, b"skel_open\0".as_ptr() as *const c_char) {
        return;
    }

    (*(*skel).rodata).ENCAPSULATION_IP = htonl(ENCAP_IP);
    (*(*skel).rodata).ENCAPSULATION_PORT = htons(ENCAP_PORT);

    err = test_cls_redirect_dynptr__load(skel);
    if !ASSERT_OK(err, b"skel_load\0".as_ptr() as *const c_char) {
        test_cls_redirect_dynptr__destroy(skel);
        return;
    }

    test_cls_redirect_common((*skel).progs.cls_redirect);

    test_cls_redirect_dynptr__destroy(skel);
}

unsafe fn test_cls_redirect_inlined() {
    let skel: *mut test_cls_redirect;
    let err: c_int;

    skel = test_cls_redirect__open();
    if CHECK(
        skel.is_null(),
        b"skel_open\0".as_ptr() as *const c_char,
        b"failed\n\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    (*(*skel).rodata).ENCAPSULATION_IP = htonl(ENCAP_IP);
    (*(*skel).rodata).ENCAPSULATION_PORT = htons(ENCAP_PORT);

    err = test_cls_redirect__load(skel);
    if CHECK(
        err != 0,
        b"skel_load\0".as_ptr() as *const c_char,
        b"failed: %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        test_cls_redirect__destroy(skel);
        return;
    }

    test_cls_redirect_common((*skel).progs.cls_redirect);

    test_cls_redirect__destroy(skel);
}

unsafe fn test_cls_redirect_subprogs() {
    let skel: *mut test_cls_redirect_subprogs;
    let err: c_int;

    skel = test_cls_redirect_subprogs__open();
    if CHECK(
        skel.is_null(),
        b"skel_open\0".as_ptr() as *const c_char,
        b"failed\n\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    (*(*skel).rodata).ENCAPSULATION_IP = htonl(ENCAP_IP);
    (*(*skel).rodata).ENCAPSULATION_PORT = htons(ENCAP_PORT);

    err = test_cls_redirect_subprogs__load(skel);
    if CHECK(
        err != 0,
        b"skel_load\0".as_ptr() as *const c_char,
        b"failed: %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        test_cls_redirect_subprogs__destroy(skel);
        return;
    }

    test_cls_redirect_common((*skel).progs.cls_redirect);

    test_cls_redirect_subprogs__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cls_redirect() {
    if test__start_subtest(b"cls_redirect_inlined\0".as_ptr() as *const c_char) {
        test_cls_redirect_inlined();
    }
    if test__start_subtest(b"cls_redirect_subprogs\0".as_ptr() as *const c_char) {
        test_cls_redirect_subprogs();
    }
    if test__start_subtest(b"cls_redirect_dynptr\0".as_ptr() as *const c_char) {
        test_cls_redirect_dynptr();
    }
}
