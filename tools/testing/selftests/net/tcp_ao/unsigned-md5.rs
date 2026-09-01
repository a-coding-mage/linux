// SPDX-License-Identifier: GPL-2.0
/* Author: Dmitry Safonov <dima@arista.com> */
// Translated from C source. External symbols are supplied by the test harness
// corresponding to aolib.h and system headers.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type TestCnt = c_int;
type FaultT = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub union tcp_addr {
    pub a4: in_addr,
    pub a6: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
pub struct tcp_counters {
    _unused: [u8; 0],
}

const DEFAULT_TEST_PASSWORD: *const c_char = b"DEFAULT_TEST_PASSWORD\0".as_ptr() as *const c_char;
const KCONFIG_NET_VRF: c_int = 0;
const KCONFIG_TCP_MD5: c_int = 0;
const TEST_FAMILY: c_int = 0;
const TEST_PREFIX: u8 = 0;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const IPPROTO_TCP: c_int = 6;
const ETIMEDOUT: c_int = 110;
const EKEYREJECTED: c_int = 129;
const ECONNREFUSED: c_int = 111;
const TCP_AO_KEYF_IFINDEX: u8 = 1;

const TEST_CNT_GOOD: TestCnt = 0;
const TEST_CNT_NS_MD5_UNEXPECTED: TestCnt = 0;
const TEST_CNT_AO_REQUIRED: TestCnt = 0;
const TEST_CNT_NS_KEY_NOT_FOUND: TestCnt = 0;
const TEST_CNT_NS_MD5_NOT_FOUND: TestCnt = 0;
const TEST_CNT_AO_KEY_NOT_FOUND: TestCnt = 0;

const FAULT_TIMEOUT: FaultT = 1;
const FAULT_KEYREJECT: FaultT = 2;
const FAULT_POSTINSTALL: FaultT = 3;
const FAULT_PREINSTALL_AO: FaultT = 4;
const FAULT_PREINSTALL_MD5: FaultT = 5;

const TCP_HASH_MD5_UNEXPECTED: c_int = 0;
const TCP_HASH_AO_REQUIRED: c_int = 0;
const TCP_AO_KEY_NOT_FOUND: c_int = 0;
const TCP_HASH_MD5_REQUIRED: c_int = 0;

const fn bit(nr: c_uint) -> c_uint {
    1u32 << nr
}

fn fault(inj: FaultT, type_: FaultT) -> bool {
    inj == type_
}

static MD5_PASSWORD: &[u8] =
    b"Some evil genius, enemy to mankind, must have been the first contriver.\0";
static mut md5_password: *const c_char = MD5_PASSWORD.as_ptr() as *const c_char;
static mut ao_password: *const c_char = DEFAULT_TEST_PASSWORD;
static mut sk_pair: c_int = 0;

static mut client2: tcp_addr = tcp_addr { a4: in_addr { s_addr: 0 } };
static mut client3: tcp_addr = tcp_addr { a4: in_addr { s_addr: 0 } };

static test_vrf_ifindex: c_int = 200;
static test_vrf_tabid: u8 = 42;

unsafe extern "C" {
    static mut this_ip_addr: tcp_addr;
    static mut this_ip_dest: tcp_addr;
    static mut veth_name: *const c_char;
    static mut test_server_port: c_uint;
    static mut test_family: c_int;
    static mut errno: c_int;

    fn kernel_config_has(config: c_int) -> bool;
    fn add_vrf(name: *const c_char, tabid: u8, ifindex: c_int, arg: c_int) -> c_int;
    fn link_set_up(name: *const c_char) -> c_int;
    fn ip_route_add_vrf(
        name: *const c_char,
        family: c_int,
        src: tcp_addr,
        dst: tcp_addr,
        tabid: u8,
    ) -> c_int;
    fn test_error(fmt: *const c_char, ...);
    fn should_skip_test(tst_name: *const c_char, config: c_int) -> bool;
    fn test_listen_socket(addr: tcp_addr, port: c_uint, backlog: c_int) -> c_int;
    fn test_set_md5(
        sk: c_int,
        addr: tcp_addr,
        prefix: u8,
        vrf: c_int,
        password: *const c_char,
    ) -> c_int;
    fn test_add_key(
        sk: c_int,
        password: *const c_char,
        addr: tcp_addr,
        prefix: u8,
        sndid: u8,
        rcvid: u8,
    ) -> c_int;
    fn test_set_ao_flags(sk: c_int, required: bool, accept_icmps: bool) -> c_int;
    fn netstat_get_one(name: *const c_char, arg: *mut c_void) -> u64;
    fn test_get_tcp_counters(sk: c_int, cnt: *mut tcp_counters) -> c_int;
    fn synchronize_threads();
    fn test_skpair_wait_poll(lsk: c_int, arg: c_int, poll_cnt: TestCnt, pair: *mut c_int) -> c_int;
    fn test_fail(fmt: *const c_char, ...);
    fn accept(fd: c_int, addr: *mut sockaddr, len: *mut c_void) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn test_ok(fmt: *const c_char, ...);
    fn test_assert_counters(
        tst_name: *const c_char,
        cnt1: *mut tcp_counters,
        cnt2: *mut tcp_counters,
        expected: TestCnt,
    );
    fn test_kill_sk(sk: c_int);
    fn ip_route_add(name: *const c_char, family: c_int, src: tcp_addr, dst: tcp_addr) -> c_int;
    fn bind(sk: c_int, addr: *const sockaddr, len: usize) -> c_int;
    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn test_skpair_connect_poll(
        sk: c_int,
        addr: tcp_addr,
        port: c_uint,
        arg: c_int,
        pair: *mut c_int,
    ) -> c_int;
    fn test_add_key_vrf(
        sk: c_int,
        password: *const c_char,
        keyflags: u8,
        addr: tcp_addr,
        prefix: u8,
        vrf: u8,
        sndid: u8,
        rcvid: u8,
    ) -> c_int;
    fn inet_pton(family: c_int, src: *const c_char, dst: *mut tcp_addr) -> c_int;
    fn ip_addr_add(name: *const c_char, family: c_int, addr: tcp_addr, prefix: u8) -> c_int;
    fn trace_hash_event_expect(
        event: c_int,
        src: tcp_addr,
        dst: tcp_addr,
        l3index: c_int,
        port: c_uint,
        a: c_int,
        b: c_int,
        c: c_int,
        d: c_int,
        e: c_int,
        f: c_int,
    );
    fn trace_ao_event_expect(
        event: c_int,
        src: tcp_addr,
        dst: tcp_addr,
        l3index: c_int,
        port: c_uint,
        a: c_int,
        b: c_int,
        c: c_int,
        d: c_int,
        e: c_int,
        f: c_int,
        sndid: u8,
        rcvid: u8,
        g: c_int,
    );
    fn test_init(nr: c_int, server: unsafe extern "C" fn(*mut c_void) -> *mut c_void, client: unsafe extern "C" fn(*mut c_void) -> *mut c_void);
}

unsafe fn c(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

unsafe fn test_client_ip(n: c_int) -> *const c_char {
    match n {
        2 => c(b"__TEST_CLIENT_IP(2)\0"),
        3 => c(b"__TEST_CLIENT_IP(3)\0"),
        _ => ptr::null(),
    }
}

unsafe fn setup_vrfs() {
    let mut err: c_int;

    if !kernel_config_has(KCONFIG_NET_VRF) {
        return;
    }

    err = add_vrf(c(b"ksft-vrf\0"), test_vrf_tabid, test_vrf_ifindex, -1);
    if err != 0 {
        test_error(c(b"Failed to add a VRF: %d\0"), err);
    }

    err = link_set_up(c(b"ksft-vrf\0"));
    if err != 0 {
        test_error(c(b"Failed to bring up a VRF\0"));
    }

    err = ip_route_add_vrf(veth_name, TEST_FAMILY, this_ip_addr, this_ip_dest, test_vrf_tabid);
    if err != 0 {
        test_error(c(b"Failed to add a route to VRF: %d\0"), err);
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn try_accept(
    tst_name: *const c_char,
    port: c_uint,
    md5_addr: *mut tcp_addr,
    md5_prefix: u8,
    ao_addr: *mut tcp_addr,
    ao_prefix: u8,
    set_ao_required: bool,
    sndid: u8,
    rcvid: u8,
    _vrf: u8,
    cnt_name: *const c_char,
    cnt_expected: TestCnt,
    needs_tcp_md5: c_int,
    inj: FaultT,
) {
    let mut cnt1: tcp_counters = tcp_counters { _unused: [] };
    let mut cnt2: tcp_counters = tcp_counters { _unused: [] };
    let mut before_cnt: u64 = 0;
    let mut after_cnt: u64;
    let poll_cnt: TestCnt = if cnt_expected == TEST_CNT_GOOD { 0 } else { cnt_expected };
    let lsk: c_int;
    let err: c_int;
    let mut sk: c_int = -1;

    if needs_tcp_md5 != 0 && should_skip_test(tst_name, KCONFIG_TCP_MD5) {
        return;
    }

    lsk = test_listen_socket(this_ip_addr, port, 1);

    if !md5_addr.is_null() && test_set_md5(lsk, *md5_addr, md5_prefix, -1, md5_password) != 0 {
        test_error(c(b"setsockopt(TCP_MD5SIG_EXT)\0"));
    }

    if !ao_addr.is_null() && test_add_key(lsk, ao_password, *ao_addr, ao_prefix, sndid, rcvid) != 0 {
        test_error(c(b"setsockopt(TCP_AO_ADD_KEY)\0"));
    }

    if set_ao_required && test_set_ao_flags(lsk, true, false) != 0 {
        test_error(c(b"setsockopt(TCP_AO_INFO)\0"));
    }

    if !cnt_name.is_null() {
        before_cnt = netstat_get_one(cnt_name, ptr::null_mut());
    }
    if !ao_addr.is_null() && test_get_tcp_counters(lsk, &mut cnt1) != 0 {
        test_error(c(b"test_get_tcp_counters()\0"));
    }

    synchronize_threads(); /* preparations done */

    err = test_skpair_wait_poll(lsk, 0, poll_cnt, &mut sk_pair);
    synchronize_threads(); /* connect()/accept() timeouts */
    if err == -ETIMEDOUT {
        sk_pair = err;
        if !fault(inj, FAULT_TIMEOUT) {
            test_fail(c(b"%s: timed out for accept()\0"), tst_name);
        }
    } else if err == -EKEYREJECTED {
        if !fault(inj, FAULT_KEYREJECT) {
            test_fail(c(b"%s: key was rejected\0"), tst_name);
        }
    } else if err < 0 {
        test_error(c(b"test_skpair_wait_poll()\0"));
    } else {
        if fault(inj, FAULT_TIMEOUT) {
            test_fail(c(b"%s: ready to accept\0"), tst_name);
        }

        sk = accept(lsk, ptr::null_mut(), ptr::null_mut());
        if sk < 0 {
            test_error(c(b"accept()\0"));
        } else if fault(inj, FAULT_TIMEOUT) {
            test_fail(c(b"%s: accepted\0"), tst_name);
        }
    }

    if !ao_addr.is_null() && test_get_tcp_counters(lsk, &mut cnt2) != 0 {
        test_error(c(b"test_get_tcp_counters()\0"));
    }
    close(lsk);

    if cnt_name.is_null() {
        test_ok(c(b"%s: no counter checks\0"), tst_name);
    } else {
        after_cnt = netstat_get_one(cnt_name, ptr::null_mut());

        if after_cnt <= before_cnt {
            test_fail(
                c(b"%s: %s counter did not increase: %llu <= %llu\0"),
                tst_name,
                cnt_name,
                after_cnt,
                before_cnt,
            );
        } else {
            test_ok(
                c(b"%s: counter %s increased %llu => %llu\0"),
                tst_name,
                cnt_name,
                before_cnt,
                after_cnt,
            );
        }
        if !ao_addr.is_null() {
            test_assert_counters(tst_name, &mut cnt1, &mut cnt2, cnt_expected);
        }
    }

    synchronize_threads(); /* test_kill_sk() */
    if sk >= 0 {
        test_kill_sk(sk);
    }
}

unsafe fn server_add_routes() {
    let family: c_int = TEST_FAMILY;

    synchronize_threads(); /* client_add_ips() */

    if ip_route_add(veth_name, family, this_ip_addr, client2) != 0 {
        test_error(c(b"Failed to add route\0"));
    }
    if ip_route_add(veth_name, family, this_ip_addr, client3) != 0 {
        test_error(c(b"Failed to add route\0"));
    }
}

unsafe fn server_add_fail_tests(port: *mut c_uint) {
    let mut addr_any: tcp_addr = tcp_addr { a4: in_addr { s_addr: 0 } };

    try_accept(c(b"TCP-AO established: add TCP-MD5 key\0"), *port, ptr::null_mut(), 0, &mut addr_any, 0, false, 100, 100, 0, c(b"TCPAOGood\0"), TEST_CNT_GOOD, 1, 0);
    *port += 1;
    try_accept(c(b"TCP-MD5 established: add TCP-AO key\0"), *port, &mut addr_any, 0, ptr::null_mut(), 0, false, 0, 0, 0, ptr::null(), 0, 1, 0);
    *port += 1;
    try_accept(c(b"non-signed established: add TCP-AO key\0"), *port, ptr::null_mut(), 0, ptr::null_mut(), 0, false, 0, 0, 0, c(b"CurrEstab\0"), 0, 0, 0);
    *port += 1;
}

unsafe fn server_vrf_tests(_port: *mut c_uint) {
    setup_vrfs();
}

unsafe extern "C" fn server_fn(_arg: *mut c_void) -> *mut c_void {
    let mut port: c_uint = test_server_port;
    let mut addr_any: tcp_addr = tcp_addr { a4: in_addr { s_addr: 0 } };

    server_add_routes();

    macro_rules! acc {
        ($name:expr, $md5:expr, $md5p:expr, $ao:expr, $aop:expr, $req:expr, $snd:expr, $rcv:expr, $vrf:expr, $cnt:expr, $exp:expr, $need:expr, $inj:expr) => {{
            try_accept(c($name), port, $md5, $md5p, $ao, $aop, $req, $snd, $rcv, $vrf, $cnt, $exp, $need, $inj);
            port += 1;
        }};
    }

    acc!(b"[server] AO server (INADDR_ANY): AO client\0", ptr::null_mut(), 0, &mut addr_any, 0, false, 100, 100, 0, c(b"TCPAOGood\0"), TEST_CNT_GOOD, 0, 0);
    acc!(b"[server] AO server (INADDR_ANY): MD5 client\0", ptr::null_mut(), 0, &mut addr_any, 0, false, 100, 100, 0, c(b"TCPMD5Unexpected\0"), TEST_CNT_NS_MD5_UNEXPECTED, 1, FAULT_TIMEOUT);
    acc!(b"[server] AO server (INADDR_ANY): no sign client\0", ptr::null_mut(), 0, &mut addr_any, 0, false, 100, 100, 0, c(b"TCPAORequired\0"), TEST_CNT_AO_REQUIRED, 0, FAULT_TIMEOUT);
    acc!(b"[server] AO server (AO_REQUIRED): AO client\0", ptr::null_mut(), 0, &mut this_ip_dest, TEST_PREFIX, true, 100, 100, 0, c(b"TCPAOGood\0"), TEST_CNT_GOOD, 0, 0);
    acc!(b"[server] AO server (AO_REQUIRED): unsigned client\0", ptr::null_mut(), 0, &mut this_ip_dest, TEST_PREFIX, true, 100, 100, 0, c(b"TCPAORequired\0"), TEST_CNT_AO_REQUIRED, 0, FAULT_TIMEOUT);

    acc!(b"[server] MD5 server (INADDR_ANY): AO client\0", &mut addr_any, 0, ptr::null_mut(), 0, false, 0, 0, 0, c(b"TCPAOKeyNotFound\0"), TEST_CNT_NS_KEY_NOT_FOUND, 1, FAULT_TIMEOUT);
    acc!(b"[server] MD5 server (INADDR_ANY): MD5 client\0", &mut addr_any, 0, ptr::null_mut(), 0, false, 0, 0, 0, ptr::null(), 0, 1, 0);
    acc!(b"[server] MD5 server (INADDR_ANY): no sign client\0", &mut addr_any, 0, ptr::null_mut(), 0, false, 0, 0, 0, c(b"TCPMD5NotFound\0"), TEST_CNT_NS_MD5_NOT_FOUND, 1, FAULT_TIMEOUT);

    acc!(b"[server] no sign server: AO client\0", ptr::null_mut(), 0, ptr::null_mut(), 0, false, 0, 0, 0, c(b"TCPAOKeyNotFound\0"), TEST_CNT_NS_KEY_NOT_FOUND, 0, FAULT_TIMEOUT);
    acc!(b"[server] no sign server: MD5 client\0", ptr::null_mut(), 0, ptr::null_mut(), 0, false, 0, 0, 0, c(b"TCPMD5Unexpected\0"), TEST_CNT_NS_MD5_UNEXPECTED, 1, FAULT_TIMEOUT);
    acc!(b"[server] no sign server: no sign client\0", ptr::null_mut(), 0, ptr::null_mut(), 0, false, 0, 0, 0, c(b"CurrEstab\0"), 0, 0, 0);

    acc!(b"[server] AO+MD5 server: AO client (matching)\0", &mut this_ip_dest, TEST_PREFIX, &mut client2, TEST_PREFIX, false, 100, 100, 0, c(b"TCPAOGood\0"), TEST_CNT_GOOD, 1, 0);
    acc!(b"[server] AO+MD5 server: AO client (misconfig, matching MD5)\0", &mut this_ip_dest, TEST_PREFIX, &mut client2, TEST_PREFIX, false, 100, 100, 0, c(b"TCPAOKeyNotFound\0"), TEST_CNT_AO_KEY_NOT_FOUND, 1, FAULT_TIMEOUT);
    acc!(b"[server] AO+MD5 server: AO client (misconfig, non-matching)\0", &mut this_ip_dest, TEST_PREFIX, &mut client2, TEST_PREFIX, false, 100, 100, 0, c(b"TCPAOKeyNotFound\0"), TEST_CNT_AO_KEY_NOT_FOUND, 1, FAULT_TIMEOUT);
    acc!(b"[server] AO+MD5 server: MD5 client (matching)\0", &mut this_ip_dest, TEST_PREFIX, &mut client2, TEST_PREFIX, false, 100, 100, 0, ptr::null(), 0, 1, 0);
    acc!(b"[server] AO+MD5 server: MD5 client (misconfig, matching AO)\0", &mut this_ip_dest, TEST_PREFIX, &mut client2, TEST_PREFIX, false, 100, 100, 0, c(b"TCPMD5Unexpected\0"), TEST_CNT_NS_MD5_UNEXPECTED, 1, FAULT_TIMEOUT);
    acc!(b"[server] AO+MD5 server: MD5 client (misconfig, non-matching)\0", &mut this_ip_dest, TEST_PREFIX, &mut client2, TEST_PREFIX, false, 100, 100, 0, c(b"TCPMD5Unexpected\0"), TEST_CNT_NS_MD5_UNEXPECTED, 1, FAULT_TIMEOUT);
    acc!(b"[server] AO+MD5 server: no sign client (unmatched)\0", &mut this_ip_dest, TEST_PREFIX, &mut client2, TEST_PREFIX, false, 100, 100, 0, c(b"CurrEstab\0"), 0, 1, 0);
    acc!(b"[server] AO+MD5 server: no sign client (misconfig, matching AO)\0", &mut this_ip_dest, TEST_PREFIX, &mut client2, TEST_PREFIX, false, 100, 100, 0, c(b"TCPAORequired\0"), TEST_CNT_AO_REQUIRED, 1, FAULT_TIMEOUT);
    acc!(b"[server] AO+MD5 server: no sign client (misconfig, matching MD5)\0", &mut this_ip_dest, TEST_PREFIX, &mut client2, TEST_PREFIX, false, 100, 100, 0, c(b"TCPMD5NotFound\0"), TEST_CNT_NS_MD5_NOT_FOUND, 1, FAULT_TIMEOUT);

    /* Key rejected by the other side, failing short through skpair */
    acc!(b"[server] AO+MD5 server: client with both [TCP-MD5] and TCP-AO keys\0", &mut this_ip_dest, TEST_PREFIX, &mut client2, TEST_PREFIX, false, 100, 100, 0, ptr::null(), 0, 1, FAULT_KEYREJECT);
    acc!(b"[server] AO+MD5 server: client with both TCP-MD5 and [TCP-AO] keys\0", &mut this_ip_dest, TEST_PREFIX, &mut client2, TEST_PREFIX, false, 100, 100, 0, ptr::null(), 0, 1, FAULT_KEYREJECT);

    server_add_fail_tests(&mut port);
    server_vrf_tests(&mut port);

    /* client exits */
    synchronize_threads();
    ptr::null_mut()
}

unsafe fn client_bind(sk: c_int, bind_addr: tcp_addr) -> c_int {
    // Original C selects sockaddr_in6 under IPV6_TEST, otherwise sockaddr_in.
    #[cfg(feature = "IPV6_TEST")]
    {
        let addr = sockaddr_in6 {
            sin6_family: AF_INET6 as u16,
            sin6_port: 0,
            sin6_flowinfo: 0,
            sin6_addr: bind_addr.a6,
            sin6_scope_id: 0,
        };
        bind(sk, &addr as *const sockaddr_in6 as *const sockaddr, size_of::<sockaddr_in6>())
    }
    #[cfg(not(feature = "IPV6_TEST"))]
    {
        let addr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: 0,
            sin_addr: bind_addr.a4,
        };
        bind(sk, &addr as *const sockaddr_in as *const sockaddr, size_of::<sockaddr_in>())
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn try_connect(
    tst_name: *const c_char,
    port: c_uint,
    md5_addr: *mut tcp_addr,
    md5_prefix: u8,
    ao_addr: *mut tcp_addr,
    ao_prefix: u8,
    sndid: u8,
    rcvid: u8,
    _vrf: u8,
    inj: FaultT,
    needs_tcp_md5: c_int,
    bind_addr: *mut tcp_addr,
) {
    let sk: c_int;
    let ret: c_int;

    if needs_tcp_md5 != 0 && should_skip_test(tst_name, KCONFIG_TCP_MD5) {
        return;
    }

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error(c(b"socket()\0"));
    }

    if !bind_addr.is_null() && client_bind(sk, *bind_addr) != 0 {
        test_error(c(b"bind()\0"));
    }

    if !md5_addr.is_null() && test_set_md5(sk, *md5_addr, md5_prefix, -1, md5_password) != 0 {
        test_error(c(b"setsockopt(TCP_MD5SIG_EXT)\0"));
    }

    if !ao_addr.is_null() && test_add_key(sk, ao_password, *ao_addr, ao_prefix, sndid, rcvid) != 0 {
        test_error(c(b"setsockopt(TCP_AO_ADD_KEY)\0"));
    }

    synchronize_threads(); /* preparations done */

    ret = test_skpair_connect_poll(sk, this_ip_dest, port, 0, &mut sk_pair);
    synchronize_threads(); /* connect()/accept() timeouts */
    if ret < 0 {
        sk_pair = ret;
        if fault(inj, FAULT_KEYREJECT) && ret == -EKEYREJECTED {
            test_ok(c(b"%s: connect() was prevented\0"), tst_name);
        } else if ret == -ETIMEDOUT && fault(inj, FAULT_TIMEOUT) {
            test_ok(c(b"%s\0"), tst_name);
        } else if ret == -ECONNREFUSED && (fault(inj, FAULT_TIMEOUT) || fault(inj, FAULT_KEYREJECT)) {
            test_ok(c(b"%s: refused to connect\0"), tst_name);
        } else {
            test_error(c(b"%s: connect() returned %d\0"), tst_name, ret);
        }
    } else {
        if fault(inj, FAULT_TIMEOUT) || fault(inj, FAULT_KEYREJECT) {
            test_fail(c(b"%s: connected\0"), tst_name);
        } else {
            test_ok(c(b"%s: connected\0"), tst_name);
        }
    }

    synchronize_threads(); /* test_kill_sk() */
    if ret > 0 {
        /* test_skpair_connect_poll() cleans up on failure */
        test_kill_sk(sk);
    }
}

const PREINSTALL_MD5_FIRST: c_uint = bit(0);
const PREINSTALL_AO: c_uint = bit(1);
const POSTINSTALL_AO: c_uint = bit(2);
const PREINSTALL_MD5: c_uint = bit(3);
const POSTINSTALL_MD5: c_uint = bit(4);

#[allow(clippy::too_many_arguments)]
unsafe fn try_add_key_vrf(
    sk: c_int,
    in_addr: tcp_addr,
    prefix: u8,
    mut vrf: c_int,
    sndid: u8,
    rcvid: u8,
    set_ao_required: bool,
) -> c_int {
    let mut keyflags: u8 = 0;

    if vrf >= 0 {
        keyflags |= TCP_AO_KEYF_IFINDEX;
    } else {
        vrf = 0;
    }
    if set_ao_required {
        let err = test_set_ao_flags(sk, true, false);

        if err != 0 {
            return err;
        }
    }
    test_add_key_vrf(sk, ao_password, keyflags, in_addr, prefix, vrf as u8, sndid, rcvid)
}

unsafe fn test_continue(tst_name: *const c_char, err: c_int, inj: FaultT, added_ao: bool) -> bool {
    let mut expected_to_fail: bool;

    expected_to_fail = fault(inj, FAULT_PREINSTALL_AO) && added_ao;
    expected_to_fail |= fault(inj, FAULT_PREINSTALL_MD5) && !added_ao;

    if err == 0 {
        if !expected_to_fail {
            return true;
        }
        test_fail(c(b"%s: setsockopt()s were expected to fail\0"), tst_name);
        return false;
    }
    if err != -EKEYREJECTED || !expected_to_fail {
        test_error(
            c(b"%s: setsockopt(%s) = %d\0"),
            tst_name,
            if added_ao { c(b"TCP_AO_ADD_KEY\0") } else { c(b"TCP_MD5SIG_EXT\0") },
            err,
        );
        return false;
    }
    test_ok(c(b"%s: prefailed as expected: %m\0"), tst_name);
    false
}

#[allow(clippy::too_many_arguments)]
unsafe fn open_add(
    tst_name: *const c_char,
    _port: c_uint,
    strategy: c_uint,
    md5_addr: tcp_addr,
    md5_prefix: u8,
    md5_vrf: c_int,
    ao_addr: tcp_addr,
    ao_prefix: u8,
    ao_vrf: c_int,
    set_ao_required: bool,
    sndid: u8,
    rcvid: u8,
    inj: FaultT,
) -> c_int {
    let sk: c_int;

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error(c(b"socket()\0"));
    }

    if client_bind(sk, this_ip_addr) != 0 {
        test_error(c(b"bind()\0"));
    }

    if (strategy & PREINSTALL_MD5_FIRST) != 0 {
        if test_set_md5(sk, md5_addr, md5_prefix, md5_vrf, md5_password) != 0 {
            test_error(c(b"setsockopt(TCP_MD5SIG_EXT)\0"));
        }
    }

    if (strategy & PREINSTALL_AO) != 0 {
        let err = try_add_key_vrf(sk, ao_addr, ao_prefix, ao_vrf, sndid, rcvid, set_ao_required);

        if !test_continue(tst_name, err, inj, true) {
            close(sk);
            return -1;
        }
    }

    if (strategy & PREINSTALL_MD5) != 0 {
        errno = 0;
        test_set_md5(sk, md5_addr, md5_prefix, md5_vrf, md5_password);
        if !test_continue(tst_name, -errno, inj, false) {
            close(sk);
            return -1;
        }
    }

    sk
}

#[allow(clippy::too_many_arguments)]
unsafe fn try_to_preadd(
    tst_name: *const c_char,
    port: c_uint,
    strategy: c_uint,
    md5_addr: tcp_addr,
    md5_prefix: u8,
    md5_vrf: c_int,
    ao_addr: tcp_addr,
    ao_prefix: u8,
    ao_vrf: c_int,
    set_ao_required: bool,
    sndid: u8,
    rcvid: u8,
    needs_tcp_md5: c_int,
    needs_vrf: c_int,
    inj: FaultT,
) {
    let sk: c_int;

    if needs_tcp_md5 != 0 && should_skip_test(tst_name, KCONFIG_TCP_MD5) {
        return;
    }
    if needs_vrf != 0 && should_skip_test(tst_name, KCONFIG_NET_VRF) {
        return;
    }

    sk = open_add(tst_name, port, strategy, md5_addr, md5_prefix, md5_vrf, ao_addr, ao_prefix, ao_vrf, set_ao_required, sndid, rcvid, inj);
    if sk < 0 {
        return;
    }

    test_ok(c(b"%s\0"), tst_name);
    close(sk);
}

#[allow(clippy::too_many_arguments)]
unsafe fn try_to_add(
    tst_name: *const c_char,
    port: c_uint,
    strategy: c_uint,
    md5_addr: tcp_addr,
    md5_prefix: u8,
    md5_vrf: c_int,
    ao_addr: tcp_addr,
    ao_prefix: u8,
    ao_vrf: c_int,
    sndid: u8,
    rcvid: u8,
    needs_tcp_md5: c_int,
    inj: FaultT,
) {
    let sk: c_int;
    let ret: c_int;

    if needs_tcp_md5 != 0 && should_skip_test(tst_name, KCONFIG_TCP_MD5) {
        return;
    }

    sk = open_add(tst_name, port, strategy, md5_addr, md5_prefix, md5_vrf, ao_addr, ao_prefix, ao_vrf, false, sndid, rcvid, inj);
    if sk < 0 {
        return;
    }

    synchronize_threads(); /* preparations done */

    ret = test_skpair_connect_poll(sk, this_ip_dest, port, 0, &mut sk_pair);

    synchronize_threads(); /* connect()/accept() timeouts */
    if ret < 0 {
        test_error(c(b"%s: connect() returned %d\0"), tst_name, ret);
    } else {
        if (strategy & POSTINSTALL_MD5) != 0 {
            if test_set_md5(sk, md5_addr, md5_prefix, md5_vrf, md5_password) != 0 {
                if fault(inj, FAULT_POSTINSTALL) {
                    test_ok(c(b"%s: postfailed as expected\0"), tst_name);
                } else {
                    test_error(c(b"setsockopt(TCP_MD5SIG_EXT)\0"));
                }
            } else if fault(inj, FAULT_POSTINSTALL) {
                test_fail(c(b"%s: post setsockopt() was expected to fail\0"), tst_name);
            }
        }

        if (strategy & POSTINSTALL_AO) != 0 {
            if try_add_key_vrf(sk, ao_addr, ao_prefix, ao_vrf, sndid, rcvid, false) != 0 {
                if fault(inj, FAULT_POSTINSTALL) {
                    test_ok(c(b"%s: postfailed as expected\0"), tst_name);
                } else {
                    test_error(c(b"setsockopt(TCP_AO_ADD_KEY)\0"));
                }
            } else if fault(inj, FAULT_POSTINSTALL) {
                test_fail(c(b"%s: post setsockopt() was expected to fail\0"), tst_name);
            }
        }
    }

    synchronize_threads(); /* test_kill_sk() */
    if ret > 0 {
        /* test_skpair_connect_poll() cleans up on failure */
        test_kill_sk(sk);
    }
}

unsafe fn client_add_ip(client: *mut tcp_addr, ip: *const c_char) {
    let err: c_int;
    let family: c_int = TEST_FAMILY;

    if inet_pton(family, ip, client) != 1 {
        test_error(c(b"Can't convert ip address %s\0"), ip);
    }

    err = ip_addr_add(veth_name, family, *client, TEST_PREFIX);
    if err != 0 {
        test_error(c(b"Failed to add ip address: %d\0"), err);
    }
}

unsafe fn client_add_ips() {
    client_add_ip(&mut client2, test_client_ip(2));
    client_add_ip(&mut client3, test_client_ip(3));
    synchronize_threads(); /* server_add_routes() */
}

unsafe fn client_add_fail_tests(port: *mut c_uint) {
    try_to_add(c(b"TCP-AO established: add TCP-MD5 key\0"), *port, POSTINSTALL_MD5 | PREINSTALL_AO, this_ip_dest, TEST_PREFIX, -1, this_ip_dest, TEST_PREFIX, 0, 100, 100, 1, FAULT_POSTINSTALL);
    *port += 1;
    try_to_add(c(b"TCP-MD5 established: add TCP-AO key\0"), *port, PREINSTALL_MD5 | POSTINSTALL_AO, this_ip_dest, TEST_PREFIX, -1, this_ip_dest, TEST_PREFIX, 0, 100, 100, 1, FAULT_POSTINSTALL);
    *port += 1;
    try_to_add(c(b"non-signed established: add TCP-AO key\0"), *port, POSTINSTALL_AO, this_ip_dest, TEST_PREFIX, -1, this_ip_dest, TEST_PREFIX, 0, 100, 100, 0, FAULT_POSTINSTALL);
    *port += 1;

    try_to_add(c(b"TCP-AO key intersects with existing TCP-MD5 key\0"), *port, PREINSTALL_MD5_FIRST | PREINSTALL_AO, this_ip_addr, TEST_PREFIX, -1, this_ip_addr, TEST_PREFIX, -1, 100, 100, 1, FAULT_PREINSTALL_AO);
    *port += 1;
    try_to_add(c(b"TCP-MD5 key intersects with existing TCP-AO key\0"), *port, PREINSTALL_MD5 | PREINSTALL_AO, this_ip_addr, TEST_PREFIX, -1, this_ip_addr, TEST_PREFIX, -1, 100, 100, 1, FAULT_PREINSTALL_MD5);
    *port += 1;

    try_to_preadd(c(b"TCP-MD5 key + TCP-AO required\0"), *port, PREINSTALL_MD5_FIRST | PREINSTALL_AO, this_ip_addr, TEST_PREFIX, -1, this_ip_addr, TEST_PREFIX, -1, true, 100, 100, 1, 0, FAULT_PREINSTALL_AO);
    *port += 1;
    try_to_preadd(c(b"TCP-AO required on socket + TCP-MD5 key\0"), *port, PREINSTALL_MD5 | PREINSTALL_AO, this_ip_addr, TEST_PREFIX, -1, this_ip_addr, TEST_PREFIX, -1, true, 100, 100, 1, 0, FAULT_PREINSTALL_MD5);
    *port += 1;
}

unsafe fn client_vrf_tests(port: *mut c_uint) {
    setup_vrfs();

    /* The following restrictions for setsockopt()s are expected:
     *
     * |--------------|-----------------|-------------|-------------|
     * |              | MD5 key without |   MD5 key   |   MD5 key   |
     * |              |     l3index     |  l3index=0  |  l3index=N  |
     * |--------------|-----------------|-------------|-------------|
     * |  TCP-AO key  |                 |             |             |
     * |  without     |     reject      |    reject   |    reject   |
     * |  l3index     |                 |             |             |
     * |--------------|-----------------|-------------|-------------|
     * |  TCP-AO key  |                 |             |             |
     * |  l3index=0   |     reject      |    reject   |    allow    |
     * |--------------|-----------------|-------------|-------------|
     * |  TCP-AO key  |                 |             |             |
     * |  l3index=N   |     reject      |    allow    |    reject   |
     * |--------------|-----------------|-------------|-------------|
     */
    macro_rules! preadd {
        ($name:expr, $strategy:expr, $md5_vrf:expr, $ao_vrf:expr, $fault:expr) => {{
            try_to_preadd(c($name), *port, $strategy, this_ip_addr, TEST_PREFIX, $md5_vrf, this_ip_addr, TEST_PREFIX, $ao_vrf, false, 100, 100, 1, 1, $fault);
            *port += 1;
        }};
    }

    preadd!(b"VRF: TCP-AO key (no l3index) + TCP-MD5 key (no l3index)\0", PREINSTALL_MD5 | PREINSTALL_AO, -1, -1, FAULT_PREINSTALL_MD5);
    preadd!(b"VRF: TCP-MD5 key (no l3index) + TCP-AO key (no l3index)\0", PREINSTALL_MD5_FIRST | PREINSTALL_AO, -1, -1, FAULT_PREINSTALL_AO);
    preadd!(b"VRF: TCP-AO key (no l3index) + TCP-MD5 key (l3index=0)\0", PREINSTALL_MD5 | PREINSTALL_AO, 0, -1, FAULT_PREINSTALL_MD5);
    preadd!(b"VRF: TCP-MD5 key (l3index=0) + TCP-AO key (no l3index)\0", PREINSTALL_MD5_FIRST | PREINSTALL_AO, 0, -1, FAULT_PREINSTALL_AO);
    preadd!(b"VRF: TCP-AO key (no l3index) + TCP-MD5 key (l3index=N)\0", PREINSTALL_MD5 | PREINSTALL_AO, test_vrf_ifindex, -1, FAULT_PREINSTALL_MD5);
    preadd!(b"VRF: TCP-MD5 key (l3index=N) + TCP-AO key (no l3index)\0", PREINSTALL_MD5_FIRST | PREINSTALL_AO, test_vrf_ifindex, -1, FAULT_PREINSTALL_AO);

    preadd!(b"VRF: TCP-AO key (l3index=0) + TCP-MD5 key (no l3index)\0", PREINSTALL_MD5 | PREINSTALL_AO, -1, 0, FAULT_PREINSTALL_MD5);
    preadd!(b"VRF: TCP-MD5 key (no l3index) + TCP-AO key (l3index=0)\0", PREINSTALL_MD5_FIRST | PREINSTALL_AO, -1, 0, FAULT_PREINSTALL_AO);
    preadd!(b"VRF: TCP-AO key (l3index=0) + TCP-MD5 key (l3index=0)\0", PREINSTALL_MD5 | PREINSTALL_AO, 0, 0, FAULT_PREINSTALL_MD5);
    preadd!(b"VRF: TCP-MD5 key (l3index=0) + TCP-AO key (l3index=0)\0", PREINSTALL_MD5_FIRST | PREINSTALL_AO, 0, 0, FAULT_PREINSTALL_AO);
    preadd!(b"VRF: TCP-AO key (l3index=0) + TCP-MD5 key (l3index=N)\0", PREINSTALL_MD5 | PREINSTALL_AO, test_vrf_ifindex, 0, 0);
    preadd!(b"VRF: TCP-MD5 key (l3index=N) + TCP-AO key (l3index=0)\0", PREINSTALL_MD5_FIRST | PREINSTALL_AO, test_vrf_ifindex, 0, 0);

    preadd!(b"VRF: TCP-AO key (l3index=N) + TCP-MD5 key (no l3index)\0", PREINSTALL_MD5 | PREINSTALL_AO, test_vrf_ifindex, -1, FAULT_PREINSTALL_MD5);
    preadd!(b"VRF: TCP-MD5 key (no l3index) + TCP-AO key (l3index=N)\0", PREINSTALL_MD5_FIRST | PREINSTALL_AO, -1, test_vrf_ifindex, FAULT_PREINSTALL_AO);
    preadd!(b"VRF: TCP-AO key (l3index=N) + TCP-MD5 key (l3index=0)\0", PREINSTALL_MD5 | PREINSTALL_AO, 0, test_vrf_ifindex, 0);
    preadd!(b"VRF: TCP-MD5 key (l3index=0) + TCP-AO key (l3index=N)\0", PREINSTALL_MD5_FIRST | PREINSTALL_AO, 0, test_vrf_ifindex, 0);
    preadd!(b"VRF: TCP-AO key (l3index=N) + TCP-MD5 key (l3index=N)\0", PREINSTALL_MD5 | PREINSTALL_AO, test_vrf_ifindex, test_vrf_ifindex, FAULT_PREINSTALL_MD5);
    preadd!(b"VRF: TCP-MD5 key (l3index=N) + TCP-AO key (l3index=N)\0", PREINSTALL_MD5_FIRST | PREINSTALL_AO, test_vrf_ifindex, test_vrf_ifindex, FAULT_PREINSTALL_AO);
}

unsafe extern "C" fn client_fn(_arg: *mut c_void) -> *mut c_void {
    let mut port: c_uint = test_server_port;
    let mut addr_any: tcp_addr = tcp_addr { a4: in_addr { s_addr: 0 } };

    client_add_ips();

    macro_rules! conn {
        ($name:expr, $md5:expr, $md5p:expr, $ao:expr, $aop:expr, $snd:expr, $rcv:expr, $vrf:expr, $inj:expr, $need:expr, $bind:expr) => {{
            try_connect(c($name), port, $md5, $md5p, $ao, $aop, $snd, $rcv, $vrf, $inj, $need, $bind);
            port += 1;
        }};
    }

    conn!(b"AO server (INADDR_ANY): AO client\0", ptr::null_mut(), 0, &mut addr_any, 0, 100, 100, 0, 0, 0, &mut this_ip_addr);
    trace_hash_event_expect(TCP_HASH_MD5_UNEXPECTED, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0);
    conn!(b"AO server (INADDR_ANY): MD5 client\0", &mut addr_any, 0, ptr::null_mut(), 0, 100, 100, 0, FAULT_TIMEOUT, 1, &mut this_ip_addr);
    trace_hash_event_expect(TCP_HASH_AO_REQUIRED, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0);
    conn!(b"AO server (INADDR_ANY): unsigned client\0", ptr::null_mut(), 0, ptr::null_mut(), 0, 100, 100, 0, FAULT_TIMEOUT, 0, &mut this_ip_addr);
    conn!(b"AO server (AO_REQUIRED): AO client\0", ptr::null_mut(), 0, &mut addr_any, 0, 100, 100, 0, 0, 0, &mut this_ip_addr);
    trace_hash_event_expect(TCP_HASH_AO_REQUIRED, client2, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0);
    conn!(b"AO server (AO_REQUIRED): unsigned client\0", ptr::null_mut(), 0, ptr::null_mut(), 0, 100, 100, 0, FAULT_TIMEOUT, 0, &mut client2);

    trace_ao_event_expect(TCP_AO_KEY_NOT_FOUND, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0, 100, 100, -1);
    conn!(b"MD5 server (INADDR_ANY): AO client\0", ptr::null_mut(), 0, &mut addr_any, 0, 100, 100, 0, FAULT_TIMEOUT, 1, &mut this_ip_addr);
    conn!(b"MD5 server (INADDR_ANY): MD5 client\0", &mut addr_any, 0, ptr::null_mut(), 0, 100, 100, 0, 0, 1, &mut this_ip_addr);
    trace_hash_event_expect(TCP_HASH_MD5_REQUIRED, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0);
    conn!(b"MD5 server (INADDR_ANY): no sign client\0", ptr::null_mut(), 0, ptr::null_mut(), 0, 100, 100, 0, FAULT_TIMEOUT, 1, &mut this_ip_addr);

    trace_ao_event_expect(TCP_AO_KEY_NOT_FOUND, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0, 100, 100, -1);
    conn!(b"no sign server: AO client\0", ptr::null_mut(), 0, &mut addr_any, 0, 100, 100, 0, FAULT_TIMEOUT, 0, &mut this_ip_addr);
    trace_hash_event_expect(TCP_HASH_MD5_UNEXPECTED, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0);
    conn!(b"no sign server: MD5 client\0", &mut addr_any, 0, ptr::null_mut(), 0, 100, 100, 0, FAULT_TIMEOUT, 1, &mut this_ip_addr);
    conn!(b"no sign server: no sign client\0", ptr::null_mut(), 0, ptr::null_mut(), 0, 100, 100, 0, 0, 0, &mut this_ip_addr);

    conn!(b"AO+MD5 server: AO client (matching)\0", ptr::null_mut(), 0, &mut addr_any, 0, 100, 100, 0, 0, 1, &mut client2);
    trace_ao_event_expect(TCP_AO_KEY_NOT_FOUND, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0, 100, 100, -1);
    conn!(b"AO+MD5 server: AO client (misconfig, matching MD5)\0", ptr::null_mut(), 0, &mut addr_any, 0, 100, 100, 0, FAULT_TIMEOUT, 1, &mut this_ip_addr);
    trace_ao_event_expect(TCP_AO_KEY_NOT_FOUND, client3, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0, 100, 100, -1);
    conn!(b"AO+MD5 server: AO client (misconfig, non-matching)\0", ptr::null_mut(), 0, &mut addr_any, 0, 100, 100, 0, FAULT_TIMEOUT, 1, &mut client3);
    conn!(b"AO+MD5 server: MD5 client (matching)\0", &mut addr_any, 0, ptr::null_mut(), 0, 100, 100, 0, 0, 1, &mut this_ip_addr);
    trace_hash_event_expect(TCP_HASH_MD5_UNEXPECTED, client2, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0);
    conn!(b"AO+MD5 server: MD5 client (misconfig, matching AO)\0", &mut addr_any, 0, ptr::null_mut(), 0, 100, 100, 0, FAULT_TIMEOUT, 1, &mut client2);
    trace_hash_event_expect(TCP_HASH_MD5_UNEXPECTED, client3, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0);
    conn!(b"AO+MD5 server: MD5 client (misconfig, non-matching)\0", &mut addr_any, 0, ptr::null_mut(), 0, 100, 100, 0, FAULT_TIMEOUT, 1, &mut client3);
    conn!(b"AO+MD5 server: no sign client (unmatched)\0", ptr::null_mut(), 0, ptr::null_mut(), 0, 100, 100, 0, 0, 1, &mut client3);
    trace_hash_event_expect(TCP_HASH_AO_REQUIRED, client2, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0);
    conn!(b"AO+MD5 server: no sign client (misconfig, matching AO)\0", ptr::null_mut(), 0, ptr::null_mut(), 0, 100, 100, 0, FAULT_TIMEOUT, 1, &mut client2);
    trace_hash_event_expect(TCP_HASH_MD5_REQUIRED, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0);
    conn!(b"AO+MD5 server: no sign client (misconfig, matching MD5)\0", ptr::null_mut(), 0, ptr::null_mut(), 0, 100, 100, 0, FAULT_TIMEOUT, 1, &mut this_ip_addr);

    conn!(b"AO+MD5 server: client with both [TCP-MD5] and TCP-AO keys\0", &mut this_ip_addr, TEST_PREFIX, &mut client2, TEST_PREFIX, 100, 100, 0, FAULT_KEYREJECT, 1, &mut this_ip_addr);
    conn!(b"AO+MD5 server: client with both TCP-MD5 and [TCP-AO] keys\0", &mut this_ip_addr, TEST_PREFIX, &mut client2, TEST_PREFIX, 100, 100, 0, FAULT_KEYREJECT, 1, &mut client2);

    client_add_fail_tests(&mut port);
    client_vrf_tests(&mut port);

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    test_init(73, server_fn, client_fn);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
