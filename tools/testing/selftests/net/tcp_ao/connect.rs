// SPDX-License-Identifier: GPL-2.0
/* Author: Dmitry Safonov <dima@arista.com> */
/* Translated from C. External symbols are provided by aolib.h/libc. */

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

type SizeT = usize;
type SSizeT = isize;
type Uint64T = u64;

const SOCK_STREAM: c_int = 1;
const IPPROTO_TCP: c_int = 6;

#[repr(C)]
pub struct netstat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcp_ao_key_counters {
    pub pkt_good: Uint64T,
}

#[repr(C)]
pub struct tcp_ao_info_counters {
    pub ao_info_pkt_good: Uint64T,
    pub key_cnts: [tcp_ao_key_counters; 1],
}

#[repr(C)]
pub struct tcp_counters {
    pub ao: tcp_ao_info_counters,
}

unsafe extern "C" {
    static this_ip_addr: c_void;
    static this_ip_dest: c_void;
    static test_server_port: c_int;
    static test_family: c_int;
    static DEFAULT_TEST_PASSWORD: *const c_char;
    static TEST_TIMEOUT_SEC: c_int;
    static TEST_CNT_GOOD: c_int;

    fn test_listen_socket(addr: *const c_void, port: c_int, backlog: c_int) -> c_int;
    fn test_add_key(
        sk: c_int,
        password: *const c_char,
        addr: *const c_void,
        prefix: c_int,
        sndid: c_int,
        rcvid: c_int,
    ) -> c_int;
    fn synchronize_threads();
    fn test_wait_fd(fd: c_int, timeout_sec: c_int, events: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut c_void, addrlen: *mut c_void) -> c_int;
    fn test_server_run(sk: c_int, a: c_int, b: c_int) -> SSizeT;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn test_connect_socket(sk: c_int, addr: *const c_void, port: c_int) -> c_int;
    fn netstat_read() -> *mut netstat;
    fn netstat_get(ns: *mut netstat, name: *const c_char, arg: *mut c_void) -> Uint64T;
    fn test_get_tcp_counters(sk: c_int, counters: *mut tcp_counters) -> c_int;
    fn test_client_verify(sk: c_int, value: c_int, nr_packets: SizeT) -> c_int;
    fn netstat_print_diff(ns_before: *mut netstat, ns_after: *mut netstat);
    fn netstat_free(ns: *mut netstat);
    fn test_assert_counters(
        name: *const c_char,
        before: *const tcp_counters,
        after: *const tcp_counters,
        cnt: c_int,
    ) -> c_int;
    fn test_init(nr_threads: c_int, server: unsafe extern "C" fn(*mut c_void) -> *mut c_void, client: unsafe extern "C" fn(*mut c_void) -> *mut c_void);

    fn test_error(fmt: *const c_char, ...);
    fn test_fail(fmt: *const c_char, ...);
    fn test_ok(fmt: *const c_char, ...);
}

unsafe extern "C" fn server_fn(_arg: *mut c_void) -> *mut c_void {
    let sk: c_int;
    let lsk: c_int;
    let bytes: SSizeT;

    lsk = test_listen_socket(&raw const this_ip_addr, test_server_port, 1);

    if test_add_key(
        lsk,
        DEFAULT_TEST_PASSWORD,
        &raw const this_ip_dest,
        -1,
        100,
        100,
    ) != 0
    {
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

    synchronize_threads();

    bytes = test_server_run(sk, 0, 0);

    test_fail(c"server served: %zd".as_ptr(), bytes);
    ptr::null_mut()
}

unsafe extern "C" fn client_fn(_arg: *mut c_void) -> *mut c_void {
    let sk: c_int = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    let before_aogood: Uint64T;
    let after_aogood: Uint64T;
    let nr_packets: SizeT = 20;
    let ns_before: *mut netstat;
    let ns_after: *mut netstat;
    let mut ao1: tcp_counters = std::mem::zeroed();
    let mut ao2: tcp_counters = std::mem::zeroed();

    if sk < 0 {
        test_error(c"socket()".as_ptr());
    }

    if test_add_key(
        sk,
        DEFAULT_TEST_PASSWORD,
        &raw const this_ip_dest,
        -1,
        100,
        100,
    ) != 0
    {
        test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
    }

    synchronize_threads();
    if test_connect_socket(sk, &raw const this_ip_dest, test_server_port) <= 0 {
        test_error(c"failed to connect()".as_ptr());
    }
    synchronize_threads();

    ns_before = netstat_read();
    before_aogood = netstat_get(ns_before, c"TCPAOGood".as_ptr(), ptr::null_mut());
    if test_get_tcp_counters(sk, &mut ao1) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }

    if test_client_verify(sk, 100, nr_packets) != 0 {
        test_fail(c"verify failed".as_ptr());
        return ptr::null_mut();
    }

    ns_after = netstat_read();
    after_aogood = netstat_get(ns_after, c"TCPAOGood".as_ptr(), ptr::null_mut());
    if test_get_tcp_counters(sk, &mut ao2) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }
    netstat_print_diff(ns_before, ns_after);
    netstat_free(ns_before);
    netstat_free(ns_after);

    if nr_packets > (after_aogood.wrapping_sub(before_aogood)) as SizeT {
        /* C used PRIu64 from <inttypes.h> for the uint64_t format fragments. */
        test_fail(
            c"TCPAOGood counter mismatch: %zu > (%lu - %lu)".as_ptr(),
            nr_packets,
            after_aogood,
            before_aogood,
        );
        return ptr::null_mut();
    }
    if test_assert_counters(c"connect".as_ptr(), &ao1, &ao2, TEST_CNT_GOOD) != 0 {
        return ptr::null_mut();
    }

    /* C used PRIu64 from <inttypes.h> for the uint64_t format fragments. */
    test_ok(
        c"connect TCPAOGood %lu/%lu/%lu => %lu/%lu/%lu, sent %zu".as_ptr(),
        before_aogood,
        ao1.ao.ao_info_pkt_good,
        ao1.ao.key_cnts[0].pkt_good,
        after_aogood,
        ao2.ao.ao_info_pkt_good,
        ao2.ao.key_cnts[0].pkt_good,
        nr_packets,
    );
    ptr::null_mut()
}

fn main() {
    unsafe {
        test_init(2, server_fn, client_fn);
    }
}
