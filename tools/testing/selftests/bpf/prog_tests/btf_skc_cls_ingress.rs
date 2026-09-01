// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/* Translated from:
 * <netinet/in.h>, <arpa/inet.h>, <unistd.h>, <stdlib.h>, <string.h>,
 * <errno.h>, <sched.h>, <net/if.h>, <linux/compiler.h>, <bpf/libbpf.h>,
 * "network_helpers.h", "test_progs.h",
 * "test_btf_skc_cls_ingress.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const TEST_NS: *const c_char = b"skc_cls_ingress\0".as_ptr() as *const c_char;

const fn BIT(n: c_int) -> c_int {
    1 << n
}

const TEST_MODE_IPV4: c_int = BIT(0);
const TEST_MODE_IPV6: c_int = BIT(1);
const TEST_MODE_DUAL: c_int = TEST_MODE_IPV4 | TEST_MODE_IPV6;

const SERVER_ADDR_IPV4: *const c_char = b"127.0.0.1\0".as_ptr() as *const c_char;
const SERVER_ADDR_IPV6: *const c_char = b"::1\0".as_ptr() as *const c_char;
const SERVER_ADDR_DUAL: *const c_char = b"::0\0".as_ptr() as *const c_char;
/* RFC791, 576 for minimal IPv4 datagram, minus 40 bytes of TCP header */
const MIN_IPV4_MSS: c_uint = 536;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const IPPROTO_IPV6: c_int = 41;
const IPV6_V6ONLY: c_int = 26;
const BPF_TC_INGRESS: c_int = 1;
const USHRT_MAX: c_uint = 65535;

type SocklenT = u32;

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
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
struct sockaddr_storage {
    ss_family: u16,
    __ss_padding: [u8; 118],
    __ss_align: u64,
}

#[repr(C)]
struct bpf_tc_hook {
    sz: usize,
    ifindex: c_int,
    attach_point: c_int,
}

#[repr(C)]
struct bpf_tc_opts {
    sz: usize,
    prog_fd: c_int,
}

#[repr(C)]
struct network_helper_opts {
    post_socket_cb: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
}

#[repr(C)]
struct netns_obj {
    _private: [u8; 0],
}

#[repr(C)]
struct test_btf_skc_cls_ingress {
    progs: test_btf_skc_cls_ingress_progs,
    bss: *mut test_btf_skc_cls_ingress_bss,
}

#[repr(C)]
struct test_btf_skc_cls_ingress_progs {
    cls_ingress: *mut bpf_program,
}

#[repr(C)]
struct test_btf_skc_cls_ingress_bss {
    srv_sa4: sockaddr_in,
    srv_sa6: sockaddr_in6,
    listen_tp_sport: c_int,
    req_sk_sport: c_int,
    recv_cookie: u64,
    gen_cookie: u64,
    linum: c_uint,
    mss: c_uint,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct test {
    desc: *const c_char,
    run: unsafe fn(*mut test_btf_skc_cls_ingress),
}

unsafe extern "C" {
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn netns_new(name: *const c_char, attach: bool) -> *mut netns_obj;
    fn netns_free(ns: *mut netns_obj);
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn write_sysctl(path: *const c_char, value: *const c_char) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: SocklenT,
    ) -> c_int;
    fn start_server_str(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        opts: *mut network_helper_opts,
    ) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut SocklenT) -> c_int;
    fn ntohs(netshort: u16) -> u16;
    fn connect_to_fd(fd: c_int, timeout_ms: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut SocklenT) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn test_btf_skc_cls_ingress__open_and_load() -> *mut test_btf_skc_cls_ingress;
    fn test_btf_skc_cls_ingress__destroy(skel: *mut test_btf_skc_cls_ingress);
    fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn prepare_netns(skel: *mut test_btf_skc_cls_ingress) -> *mut netns_obj {
    let mut qdisc_lo = bpf_tc_hook {
        sz: mem::size_of::<bpf_tc_hook>(),
        ifindex: 0,
        attach_point: BPF_TC_INGRESS,
    };
    let mut tc_attach = bpf_tc_opts {
        sz: mem::size_of::<bpf_tc_opts>(),
        prog_fd: bpf_program__fd((*skel).progs.cls_ingress),
    };
    let mut ns: *mut netns_obj = ptr::null_mut();

    ns = netns_new(TEST_NS, true);
    if !ASSERT_OK_PTR!(ns, b"create and join netns\0".as_ptr() as *const c_char) {
        return ns;
    }

    qdisc_lo.ifindex = if_nametoindex(b"lo\0".as_ptr() as *const c_char) as c_int;
    if !ASSERT_OK!(
        bpf_tc_hook_create(&mut qdisc_lo),
        b"qdisc add dev lo clsact\0".as_ptr() as *const c_char,
    ) {
        goto_free_ns(ns);
        return ptr::null_mut();
    }

    if !ASSERT_OK!(
        bpf_tc_attach(&mut qdisc_lo, &mut tc_attach),
        b"filter add dev lo ingress\0".as_ptr() as *const c_char,
    ) {
        goto_free_ns(ns);
        return ptr::null_mut();
    }

    /* Ensure 20 bytes options (i.e. in total 40 bytes tcp header) for the
     * bpf_tcp_gen_syncookie() helper.
     */
    if write_sysctl(
        b"/proc/sys/net/ipv4/tcp_window_scaling\0".as_ptr() as *const c_char,
        b"1\0".as_ptr() as *const c_char,
    ) != 0
        || write_sysctl(
            b"/proc/sys/net/ipv4/tcp_timestamps\0".as_ptr() as *const c_char,
            b"1\0".as_ptr() as *const c_char,
        ) != 0
        || write_sysctl(
            b"/proc/sys/net/ipv4/tcp_sack\0".as_ptr() as *const c_char,
            b"1\0".as_ptr() as *const c_char,
        ) != 0
    {
        goto_free_ns(ns);
        return ptr::null_mut();
    }

    return ns;
}

unsafe fn goto_free_ns(ns: *mut netns_obj) {
    netns_free(ns);
}

unsafe fn reset_test(skel: *mut test_btf_skc_cls_ingress) {
    ptr::write_bytes(
        &mut (*(*skel).bss).srv_sa4 as *mut sockaddr_in as *mut u8,
        0,
        mem::size_of_val(&(*(*skel).bss).srv_sa4),
    );
    ptr::write_bytes(
        &mut (*(*skel).bss).srv_sa6 as *mut sockaddr_in6 as *mut u8,
        0,
        mem::size_of_val(&(*(*skel).bss).srv_sa6),
    );
    (*(*skel).bss).listen_tp_sport = 0;
    (*(*skel).bss).req_sk_sport = 0;
    (*(*skel).bss).recv_cookie = 0;
    (*(*skel).bss).gen_cookie = 0;
    (*(*skel).bss).linum = 0;
    (*(*skel).bss).mss = 0;
}

unsafe fn print_err_line(skel: *mut test_btf_skc_cls_ingress) {
    if (*(*skel).bss).linum != 0 {
        printf(
            b"bpf prog error at line %u\n\0".as_ptr() as *const c_char,
            (*(*skel).bss).linum,
        );
    }
}

unsafe extern "C" fn v6only_true(fd: c_int, _opts: *mut c_void) -> c_int {
    let mode: c_int = true as c_int;

    setsockopt(
        fd,
        IPPROTO_IPV6,
        IPV6_V6ONLY,
        &mode as *const c_int as *const c_void,
        mem::size_of_val(&mode) as SocklenT,
    )
}

unsafe extern "C" fn v6only_false(fd: c_int, _opts: *mut c_void) -> c_int {
    let mode: c_int = false as c_int;

    setsockopt(
        fd,
        IPPROTO_IPV6,
        IPV6_V6ONLY,
        &mode as *const c_int as *const c_void,
        mem::size_of_val(&mode) as SocklenT,
    )
}

unsafe fn run_test(skel: *mut test_btf_skc_cls_ingress, gen_cookies: bool, ip_mode: c_int) {
    let tcp_syncookies: *const c_char = if gen_cookies {
        b"2\0".as_ptr() as *const c_char
    } else {
        b"1\0".as_ptr() as *const c_char
    };
    let mut listen_fd: c_int = -1;
    let mut cli_fd: c_int = -1;
    let mut srv_fd: c_int = -1;
    let mut err: c_int;
    let mut opts: network_helper_opts = mem::zeroed();
    let mut addr: *mut sockaddr_storage;
    let mut srv_sa6: sockaddr_in6 = mem::zeroed();
    let mut srv_sa4: sockaddr_in = mem::zeroed();
    let mut addr_len: SocklenT;
    let sock_family: c_int;
    let srv_addr: *const c_char;
    let srv_port: c_int;

    match ip_mode {
        TEST_MODE_IPV4 => {
            sock_family = AF_INET;
            srv_addr = SERVER_ADDR_IPV4;
            addr = &mut srv_sa4 as *mut sockaddr_in as *mut sockaddr_storage;
            addr_len = mem::size_of_val(&srv_sa4) as SocklenT;
        }
        TEST_MODE_IPV6 => {
            opts.post_socket_cb = Some(v6only_true);
            sock_family = AF_INET6;
            srv_addr = SERVER_ADDR_IPV6;
            addr = &mut srv_sa6 as *mut sockaddr_in6 as *mut sockaddr_storage;
            addr_len = mem::size_of_val(&srv_sa6) as SocklenT;
        }
        TEST_MODE_DUAL => {
            opts.post_socket_cb = Some(v6only_false);
            sock_family = AF_INET6;
            srv_addr = SERVER_ADDR_DUAL;
            addr = &mut srv_sa6 as *mut sockaddr_in6 as *mut sockaddr_storage;
            addr_len = mem::size_of_val(&srv_sa6) as SocklenT;
        }
        _ => {
            PRINT_FAIL!(b"Unknown IP mode %d\0".as_ptr() as *const c_char, ip_mode);
            return;
        }
    }

    if write_sysctl(
        b"/proc/sys/net/ipv4/tcp_syncookies\0".as_ptr() as *const c_char,
        tcp_syncookies,
    ) != 0
    {
        return;
    }

    listen_fd = start_server_str(sock_family, SOCK_STREAM, srv_addr, 0, &mut opts);
    if !ASSERT_OK_FD!(listen_fd, b"start server\0".as_ptr() as *const c_char) {
        return;
    }

    err = getsockname(
        listen_fd,
        addr as *mut sockaddr,
        &mut addr_len as *mut SocklenT,
    );
    if !ASSERT_OK!(err, b"getsockname(listen_fd)\0".as_ptr() as *const c_char) {
        goto_done(listen_fd, cli_fd, srv_fd);
        return;
    }

    match ip_mode {
        TEST_MODE_IPV4 => {
            ptr::copy_nonoverlapping(
                &srv_sa4 as *const sockaddr_in,
                &mut (*(*skel).bss).srv_sa4 as *mut sockaddr_in,
                1,
            );
            srv_port = ntohs(srv_sa4.sin_port) as c_int;
        }
        TEST_MODE_IPV6 | TEST_MODE_DUAL => {
            ptr::copy_nonoverlapping(
                &srv_sa6 as *const sockaddr_in6,
                &mut (*(*skel).bss).srv_sa6 as *mut sockaddr_in6,
                1,
            );
            srv_port = ntohs(srv_sa6.sin6_port) as c_int;
        }
        _ => {
            goto_done(listen_fd, cli_fd, srv_fd);
            return;
        }
    }

    cli_fd = connect_to_fd(listen_fd, 0);
    if !ASSERT_OK_FD!(cli_fd, b"connect client\0".as_ptr() as *const c_char) {
        goto_done(listen_fd, cli_fd, srv_fd);
        return;
    }

    srv_fd = accept(listen_fd, ptr::null_mut(), ptr::null_mut());
    if !ASSERT_OK_FD!(srv_fd, b"accept connection\0".as_ptr() as *const c_char) {
        goto_done(listen_fd, cli_fd, srv_fd);
        return;
    }

    ASSERT_EQ!(
        (*(*skel).bss).listen_tp_sport,
        srv_port,
        b"listen tp src port\0".as_ptr() as *const c_char,
    );

    if !gen_cookies {
        ASSERT_EQ!(
            (*(*skel).bss).req_sk_sport,
            srv_port,
            b"request socket source port with syncookies disabled\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ!(
            (*(*skel).bss).gen_cookie,
            0,
            b"generated syncookie with syncookies disabled\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ!(
            (*(*skel).bss).recv_cookie,
            0,
            b"received syncookie with syncookies disabled\0".as_ptr() as *const c_char,
        );
    } else {
        ASSERT_EQ!(
            (*(*skel).bss).req_sk_sport,
            0,
            b"request socket source port with syncookies enabled\0".as_ptr() as *const c_char,
        );
        ASSERT_NEQ!(
            (*(*skel).bss).gen_cookie,
            0,
            b"syncookie properly generated\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ!(
            (*(*skel).bss).gen_cookie,
            (*(*skel).bss).recv_cookie,
            b"matching syncookies on client and server\0".as_ptr() as *const c_char,
        );
        ASSERT_GT!(
            (*(*skel).bss).mss,
            MIN_IPV4_MSS,
            b"MSS in cookie min value\0".as_ptr() as *const c_char,
        );
        ASSERT_LT!(
            (*(*skel).bss).mss,
            USHRT_MAX,
            b"MSS in cookie max value\0".as_ptr() as *const c_char,
        );
    }

    goto_done(listen_fd, cli_fd, srv_fd);
}

unsafe fn goto_done(listen_fd: c_int, cli_fd: c_int, srv_fd: c_int) {
    if listen_fd != -1 {
        close(listen_fd);
    }
    if cli_fd != -1 {
        close(cli_fd);
    }
    if srv_fd != -1 {
        close(srv_fd);
    }
}

unsafe fn test_conn_ipv4(skel: *mut test_btf_skc_cls_ingress) {
    run_test(skel, false, TEST_MODE_IPV4);
}

unsafe fn test_conn_ipv6(skel: *mut test_btf_skc_cls_ingress) {
    run_test(skel, false, TEST_MODE_IPV6);
}

unsafe fn test_conn_dual(skel: *mut test_btf_skc_cls_ingress) {
    run_test(skel, false, TEST_MODE_DUAL);
}

unsafe fn test_syncookie_ipv4(skel: *mut test_btf_skc_cls_ingress) {
    run_test(skel, true, TEST_MODE_IPV4);
}

unsafe fn test_syncookie_ipv6(skel: *mut test_btf_skc_cls_ingress) {
    run_test(skel, true, TEST_MODE_IPV6);
}

unsafe fn test_syncookie_dual(skel: *mut test_btf_skc_cls_ingress) {
    run_test(skel, true, TEST_MODE_DUAL);
}

macro_rules! DEF_TEST {
    ($name:ident, $func:ident) => {
        test {
            desc: concat!(stringify!($name), "\0").as_ptr() as *const c_char,
            run: $func,
        }
    };
}

static TESTS: [test; 6] = [
    DEF_TEST!(conn_ipv4, test_conn_ipv4),
    DEF_TEST!(conn_ipv6, test_conn_ipv6),
    DEF_TEST!(conn_dual, test_conn_dual),
    DEF_TEST!(syncookie_ipv4, test_syncookie_ipv4),
    DEF_TEST!(syncookie_ipv6, test_syncookie_ipv6),
    DEF_TEST!(syncookie_dual, test_syncookie_dual),
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_btf_skc_cls_ingress() {
    let skel: *mut test_btf_skc_cls_ingress;
    let mut ns: *mut netns_obj;
    let mut i: usize;

    skel = test_btf_skc_cls_ingress__open_and_load();
    if !ASSERT_OK_PTR!(
        skel,
        b"test_btf_skc_cls_ingress__open_and_load\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    i = 0;
    while i < TESTS.len() {
        if !test__start_subtest(TESTS[i].desc) {
            i += 1;
            continue;
        }

        ns = prepare_netns(skel);
        if ns.is_null() {
            break;
        }

        (TESTS[i].run)(skel);

        print_err_line(skel);
        reset_test(skel);
        netns_free(ns);

        i += 1;
    }

    test_btf_skc_cls_ingress__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
