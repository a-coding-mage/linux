// SPDX-License-Identifier: GPL-2.0
/* Author: Dmitry Safonov <dima@arista.com> */
/* C dependencies: <inttypes.h>, "aolib.h" */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;

type bool_ = bool;
type size_t = usize;
type uint8_t = u8;
type uint64_t = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub union tcp_addr {
    pub raw: [u8; 16],
}

#[repr(C)]
pub struct tcp_counters {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netstat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcp_ao_repair {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcp_sock_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
}

#[repr(C)]
pub struct sockaddr_af {
    #[cfg(not(IPV6_TEST))]
    pub sin_port: u16,
    #[cfg(IPV6_TEST)]
    pub sin6_port: u16,
    pub storage: [u8; 126],
}

unsafe extern "C" {
    static test_family: c_int;
    static test_server_port: c_uint;

    static DEFAULT_TEST_PASSWORD: *const c_char;

    static TEST_FAMILY: c_int;
    static SOCK_STREAM: c_int;
    static IPPROTO_TCP: c_int;
    static TEST_CNT_GOOD: c_int;
    static TCP_AO_RNEXT_REQUEST: c_int;

    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: u32) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn test_error(fmt: *const c_char, ...);
    fn test_fail(fmt: *const c_char, ...);
    fn test_ok(fmt: *const c_char, ...);

    fn ip_addr_add(
        intf: *const c_char,
        family: c_int,
        addr: tcp_addr,
        prefix: uint8_t,
    ) -> c_int;
    fn link_set_up(intf: *const c_char) -> c_int;
    fn ip_route_add(intf: *const c_char, family: c_int, src: tcp_addr, dst: tcp_addr) -> c_int;

    fn tcp_addr_to_sockaddr_in(addr: *mut sockaddr_af, tcp_addr: *const tcp_addr, port: u16);

    fn test_add_key(
        sk: c_int,
        password: *const c_char,
        addr: tcp_addr,
        prefix: c_int,
        sndid: c_int,
        rcvid: c_int,
    ) -> c_int;
    fn test_add_repaired_key(
        sk: c_int,
        password: *const c_char,
        maclen: c_int,
        addr: tcp_addr,
        prefix: c_int,
        sndid: c_int,
        rcvid: c_int,
    ) -> c_int;

    fn netstat_read() -> *mut netstat;
    fn netstat_get(ns: *mut netstat, name: *const c_char, err: *mut c_void) -> uint64_t;
    fn netstat_print_diff(before: *mut netstat, after: *mut netstat);
    fn netstat_free(ns: *mut netstat);

    fn test_get_tcp_counters(sk: c_int, counters: *mut tcp_counters) -> c_int;
    fn test_assert_counters(
        tst: *const c_char,
        before: *const tcp_counters,
        after: *const tcp_counters,
        mask: c_int,
    ) -> c_int;

    fn __test_connect_socket(
        sk: c_int,
        dev: *const c_char,
        addr: *const sockaddr,
        addrlen: u32,
        flags: c_int,
    ) -> c_int;
    fn test_client_verify(sk: c_int, timeout: c_int, nr_packets: size_t) -> c_int;

    fn test_enable_repair(sk: c_int);
    fn test_disable_repair(sk: c_int);
    fn test_sock_checkpoint(sk: c_int, img: *mut tcp_sock_state, addr: *mut sockaddr_af);
    fn __test_sock_restore(
        sk: c_int,
        dev: *const c_char,
        img: *mut tcp_sock_state,
        src: *mut sockaddr_af,
        dst: *mut sockaddr_af,
        addrlen: u32,
    );
    fn test_sock_state_free(img: *mut tcp_sock_state);
    fn test_ao_checkpoint(sk: c_int, ao_img: *mut tcp_ao_repair);
    fn test_ao_restore(sk: c_int, ao_img: *mut tcp_ao_repair);
    fn test_kill_sk(sk: c_int);

    fn trace_ao_event_expect(
        event: c_int,
        saddr: tcp_addr,
        daddr: tcp_addr,
        sport: c_uint,
        dport: c_uint,
        family: c_int,
        l3index: c_int,
        sndid: c_int,
        rcvid: c_int,
        maclen: c_int,
        keyflags: c_int,
        rnext: c_int,
        current: c_int,
        sne: c_int,
    );

    fn test_init(nr_tests: c_int, fn_: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>, arg: *mut c_void);
}

static mut local_addr: tcp_addr = tcp_addr { raw: [0; 16] };

unsafe fn __setup_lo_intf(lo_intf: *const c_char, addr_str: *const c_char, prefix: uint8_t) {
    if inet_pton(TEST_FAMILY, addr_str, &raw mut local_addr as *mut tcp_addr as *mut c_void) != 1 {
        test_error(c"Can't convert local ip address".as_ptr());
    }

    if ip_addr_add(lo_intf, TEST_FAMILY, local_addr, prefix) != 0 {
        test_error(c"Failed to add %s ip address".as_ptr(), lo_intf);
    }

    if link_set_up(lo_intf) != 0 {
        test_error(c"Failed to bring %s up".as_ptr(), lo_intf);
    }

    if ip_route_add(lo_intf, TEST_FAMILY, local_addr, local_addr) != 0 {
        test_error(c"Failed to add a local route %s".as_ptr(), lo_intf);
    }
}

unsafe fn setup_lo_intf(lo_intf: *const c_char) {
    #[cfg(IPV6_TEST)]
    {
        __setup_lo_intf(lo_intf, c"::1".as_ptr(), 128);
    }
    #[cfg(not(IPV6_TEST))]
    {
        __setup_lo_intf(lo_intf, c"127.0.0.1".as_ptr(), 8);
    }
}

unsafe fn tcp_self_connect(
    tst: *const c_char,
    mut port: c_uint,
    different_keyids: bool_,
    check_restore: bool_,
) {
    let mut before: tcp_counters = mem::zeroed();
    let mut after: tcp_counters = mem::zeroed();
    let mut before_aogood: uint64_t;
    let mut after_aogood: uint64_t;
    let mut ns_before: *mut netstat;
    let mut ns_after: *mut netstat;
    let nr_packets: size_t = 20;
    let mut ao_img: tcp_ao_repair = mem::zeroed();
    let mut img: tcp_sock_state = mem::zeroed();
    let mut addr: sockaddr_af = mem::zeroed();
    let mut sk: c_int;

    tcp_addr_to_sockaddr_in(&mut addr, &raw const local_addr, htons(port as u16));

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error(c"socket()".as_ptr());
    }

    if different_keyids {
        if test_add_key(sk, DEFAULT_TEST_PASSWORD, local_addr, -1, 5, 7) != 0 {
            test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
        }
        if test_add_key(sk, DEFAULT_TEST_PASSWORD, local_addr, -1, 7, 5) != 0 {
            test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
        }
    } else {
        if test_add_key(sk, DEFAULT_TEST_PASSWORD, local_addr, -1, 100, 100) != 0 {
            test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
        }
    }

    if bind(
        sk,
        &addr as *const sockaddr_af as *const sockaddr,
        mem::size_of_val(&addr) as u32,
    ) < 0
    {
        test_error(c"bind()".as_ptr());
    }

    ns_before = netstat_read();
    before_aogood = netstat_get(ns_before, c"TCPAOGood".as_ptr(), core::ptr::null_mut());
    if test_get_tcp_counters(sk, &mut before) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }

    if __test_connect_socket(
        sk,
        c"lo".as_ptr(),
        &addr as *const sockaddr_af as *const sockaddr,
        mem::size_of_val(&addr) as u32,
        0,
    ) < 0
    {
        ns_after = netstat_read();
        netstat_print_diff(ns_before, ns_after);
        test_error(c"failed to connect()".as_ptr());
    }

    if test_client_verify(sk, 100, nr_packets) != 0 {
        test_fail(c"%s: tcp connection verify failed".as_ptr(), tst);
        close(sk);
        return;
    }

    ns_after = netstat_read();
    after_aogood = netstat_get(ns_after, c"TCPAOGood".as_ptr(), core::ptr::null_mut());
    if test_get_tcp_counters(sk, &mut after) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }
    if !check_restore {
        /* to debug: netstat_print_diff(ns_before, ns_after); */
        netstat_free(ns_before);
    }
    netstat_free(ns_after);

    if after_aogood <= before_aogood {
        test_fail(
            c"%s: TCPAOGood counter mismatch: %lu <= %lu".as_ptr(),
            tst,
            after_aogood,
            before_aogood,
        );
        close(sk);
        return;
    }

    if test_assert_counters(tst, &before, &after, TEST_CNT_GOOD) != 0 {
        close(sk);
        return;
    }

    if !check_restore {
        test_ok(
            c"%s: connect TCPAOGood %lu => %lu".as_ptr(),
            tst,
            before_aogood,
            after_aogood,
        );
        close(sk);
        return;
    }

    test_enable_repair(sk);
    test_sock_checkpoint(sk, &mut img, &mut addr);
    #[cfg(IPV6_TEST)]
    {
        addr.sin6_port = htons((port + 1) as u16);
    }
    #[cfg(not(IPV6_TEST))]
    {
        addr.sin_port = htons((port + 1) as u16);
    }
    test_ao_checkpoint(sk, &mut ao_img);
    test_kill_sk(sk);

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error(c"socket()".as_ptr());
    }

    test_enable_repair(sk);
    __test_sock_restore(
        sk,
        c"lo".as_ptr(),
        &mut img,
        &mut addr,
        &mut addr,
        mem::size_of_val(&addr) as u32,
    );
    if different_keyids {
        if test_add_repaired_key(sk, DEFAULT_TEST_PASSWORD, 0, local_addr, -1, 7, 5) != 0 {
            test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
        }
        if test_add_repaired_key(sk, DEFAULT_TEST_PASSWORD, 0, local_addr, -1, 5, 7) != 0 {
            test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
        }
    } else {
        if test_add_repaired_key(sk, DEFAULT_TEST_PASSWORD, 0, local_addr, -1, 100, 100) != 0 {
            test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
        }
    }
    test_ao_restore(sk, &mut ao_img);
    test_disable_repair(sk);
    test_sock_state_free(&mut img);
    if test_client_verify(sk, 100, nr_packets) != 0 {
        test_fail(c"%s: tcp connection verify failed".as_ptr(), tst);
        close(sk);
        return;
    }
    ns_after = netstat_read();
    after_aogood = netstat_get(ns_after, c"TCPAOGood".as_ptr(), core::ptr::null_mut());
    /* to debug: netstat_print_diff(ns_before, ns_after); */
    netstat_free(ns_before);
    netstat_free(ns_after);
    close(sk);
    if after_aogood <= before_aogood {
        test_fail(
            c"%s: TCPAOGood counter mismatch: %lu <= %lu".as_ptr(),
            tst,
            after_aogood,
            before_aogood,
        );
        return;
    }
    test_ok(
        c"%s: connect TCPAOGood %lu => %lu".as_ptr(),
        tst,
        before_aogood,
        after_aogood,
    );
}

unsafe extern "C" fn client_fn(_arg: *mut c_void) -> *mut c_void {
    let mut port: c_uint = test_server_port;

    setup_lo_intf(c"lo".as_ptr());

    tcp_self_connect(c"self-connect(same keyids)".as_ptr(), port, false, false);
    port = port.wrapping_add(1);

    /* expecting rnext to change based on the first segment RNext != Current */
    trace_ao_event_expect(
        TCP_AO_RNEXT_REQUEST,
        local_addr,
        local_addr,
        port,
        port,
        0,
        -1,
        -1,
        -1,
        -1,
        -1,
        7,
        5,
        -1,
    );
    tcp_self_connect(c"self-connect(different keyids)".as_ptr(), port, true, false);
    port = port.wrapping_add(1);
    tcp_self_connect(c"self-connect(restore)".as_ptr(), port, false, true);
    port = port.wrapping_add(2); /* restore test restores over different port */
    trace_ao_event_expect(
        TCP_AO_RNEXT_REQUEST,
        local_addr,
        local_addr,
        port,
        port,
        0,
        -1,
        -1,
        -1,
        -1,
        -1,
        7,
        5,
        -1,
    );
    /* intentionally on restore they are added to the socket in different order */
    trace_ao_event_expect(
        TCP_AO_RNEXT_REQUEST,
        local_addr,
        local_addr,
        port + 1,
        port + 1,
        0,
        -1,
        -1,
        -1,
        -1,
        -1,
        5,
        7,
        -1,
    );
    tcp_self_connect(
        c"self-connect(restore, different keyids)".as_ptr(),
        port,
        true,
        true,
    );
    port = port.wrapping_add(2); /* restore test restores over different port */

    core::ptr::null_mut()
}

fn main() {
    unsafe {
        test_init(5, Some(client_fn), core::ptr::null_mut());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
