// SPDX-License-Identifier: GPL-2.0
/* Author: Dmitry Safonov <dima@arista.com> */
// C dependencies: <inttypes.h>, "aolib.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type uint8_t = u8;
type uint64_t = u64;
type test_cnt = c_int;
type fault_t = c_int;

const false_: bool = false;

const DEFAULT_TEST_PREFIX: uint8_t = 0;
const DEFAULT_TEST_ALGO: *const c_char = b"\0".as_ptr() as *const c_char;
const DEFAULT_TEST_PASSWORD: *const c_char = b"\0".as_ptr() as *const c_char;
const TEST_FAMILY: c_int = 0;
const TEST_WRONG_IP: *const c_char = b"\0".as_ptr() as *const c_char;
const TEST_NETWORK: *const c_char = b"\0".as_ptr() as *const c_char;

const IPPROTO_TCP: c_int = 6;
const TCP_AO_ADD_KEY: c_int = 0;
const SOCK_STREAM: c_int = 1;

const ETIMEDOUT: c_int = 110;
const EKEYREJECTED: c_int = 129;
const ECONNREFUSED: c_int = 111;

const TEST_CNT_GOOD: test_cnt = 0;
const TEST_CNT_NS_KEY_NOT_FOUND: test_cnt = 0;
const TEST_CNT_AO_REQUIRED: test_cnt = 0;
const TEST_CNT_BAD: test_cnt = 0;
const TEST_CNT_AO_KEY_NOT_FOUND: test_cnt = 0;

const FAULT_TIMEOUT: fault_t = 1;
const FAULT_KEYREJECT: fault_t = 2;

const TCP_AO_KEY_NOT_FOUND: c_int = 0;
const TCP_HASH_AO_REQUIRED: c_int = 0;
const TCP_AO_MISMATCH: c_int = 0;
const TCP_AO_SYNACK_NO_KEY: c_int = 0;
const TCP_AO_WRONG_MACLEN: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub union tcp_addr {
    _bindgen_union_align: [u8; 16],
}

#[repr(C)]
pub struct tcp_ao_add {
    _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
pub struct tcp_counters {
    _bindgen_opaque_blob: [u8; 0],
}

static mut sk_pair: c_int = 0;

unsafe extern "C" {
    static mut errno: c_int;
    static mut this_ip_addr: tcp_addr;
    static mut this_ip_dest: tcp_addr;
    static mut test_server_port: c_uint;
    static mut test_family: c_int;

    fn strlen(s: *const c_char) -> usize;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: u32,
    ) -> c_int;
    fn accept(socket: c_int, address: *mut c_void, address_len: *mut c_void) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;

    fn test_prepare_key(
        tmp: *mut tcp_ao_add,
        algo: *const c_char,
        in_addr: tcp_addr,
        set_current: bool,
        set_rnext: bool,
        prefix: uint8_t,
        vrf: c_int,
        sndid: uint8_t,
        rcvid: uint8_t,
        maclen: uint8_t,
        keyflags: c_int,
        keylen: usize,
        key: *const c_char,
    ) -> c_int;
    fn test_verify_socket_key(sk: c_int, tmp: *mut tcp_ao_add) -> c_int;
    fn test_listen_socket(addr: tcp_addr, port: c_uint, backlog: c_int) -> c_int;
    fn test_error(fmt: *const c_char, ...) -> !;
    fn test_fail(fmt: *const c_char, ...);
    fn test_ok(fmt: *const c_char, ...);
    fn test_get_tcp_counters(sk: c_int, cnt: *mut tcp_counters) -> c_int;
    fn test_assert_counters(
        tst_name: *const c_char,
        cnt1: *mut tcp_counters,
        cnt2: *mut tcp_counters,
        cnt_expected: test_cnt,
    );
    fn test_tcp_counters_free(cnt: *mut tcp_counters);
    fn synchronize_threads();
    fn test_skpair_wait_poll(
        lsk: c_int,
        flags: c_int,
        poll_cnt: test_cnt,
        sk_pair: *mut c_int,
    ) -> c_int;
    fn test_skpair_connect_poll(
        sk: c_int,
        addr: tcp_addr,
        port: c_uint,
        cnt_expected: test_cnt,
        sk_pair: *mut c_int,
    ) -> c_int;
    fn test_add_key(
        sk: c_int,
        pwd: *const c_char,
        addr: tcp_addr,
        prefix: uint8_t,
        sndid: uint8_t,
        rcvid: uint8_t,
    ) -> c_int;
    fn netstat_get_one(cnt_name: *const c_char, arg: *mut c_void) -> uint64_t;
    fn trace_ao_event_expect(
        event: c_int,
        saddr: tcp_addr,
        daddr: tcp_addr,
        family: c_int,
        port: c_uint,
        a: c_int,
        b: c_int,
        c: c_int,
        d: c_int,
        e: c_int,
        f: c_int,
        sndid: c_int,
        rcvid: c_int,
        g: c_int,
    );
    fn trace_hash_event_expect(
        event: c_int,
        saddr: tcp_addr,
        daddr: tcp_addr,
        family: c_int,
        port: c_uint,
        a: c_int,
        b: c_int,
        c: c_int,
        d: c_int,
        e: c_int,
        f: c_int,
    );
    fn trace_ao_event_sk_expect(
        event: c_int,
        saddr: tcp_addr,
        daddr: tcp_addr,
        port: c_uint,
        family: c_int,
        sndid: c_int,
        rcvid: c_int,
    );
    fn test_init(argc: c_int, server_fn: unsafe extern "C" fn(*mut c_void) -> *mut c_void, client_fn: unsafe extern "C" fn(*mut c_void) -> *mut c_void);
}

#[inline]
unsafe fn test_add_key_maclen(
    sk: c_int,
    key: *const c_char,
    maclen: uint8_t,
    in_addr: tcp_addr,
    mut prefix: uint8_t,
    sndid: uint8_t,
    rcvid: uint8_t,
) -> c_int {
    let mut tmp: tcp_ao_add = core::mem::zeroed();
    let mut err: c_int;

    if prefix > DEFAULT_TEST_PREFIX {
        prefix = DEFAULT_TEST_PREFIX;
    }

    err = test_prepare_key(
        &mut tmp,
        DEFAULT_TEST_ALGO,
        in_addr,
        false_,
        false_,
        prefix,
        0,
        sndid,
        rcvid,
        maclen,
        0,
        strlen(key),
        key,
    );
    if err != 0 {
        return err;
    }

    err = setsockopt(
        sk,
        IPPROTO_TCP,
        TCP_AO_ADD_KEY,
        &tmp as *const tcp_ao_add as *const c_void,
        size_of::<tcp_ao_add>() as u32,
    );
    if err < 0 {
        return -errno;
    }

    test_verify_socket_key(sk, &mut tmp)
}

unsafe fn try_accept(
    tst_name: *const c_char,
    mut port: c_uint,
    pwd: *const c_char,
    addr: tcp_addr,
    prefix: uint8_t,
    sndid: uint8_t,
    rcvid: uint8_t,
    maclen: uint8_t,
    cnt_name: *const c_char,
    cnt_expected: test_cnt,
    inj: fault_t,
) {
    let mut cnt1: tcp_counters = core::mem::zeroed();
    let mut cnt2: tcp_counters = core::mem::zeroed();
    let mut before_cnt: uint64_t = 0;
    let mut after_cnt: uint64_t = 0; /* silence GCC */
    let poll_cnt: test_cnt = if cnt_expected == TEST_CNT_GOOD { 0 } else { cnt_expected };
    let lsk: c_int;
    let mut err: c_int;
    let mut sk: c_int = 0;

    lsk = test_listen_socket(this_ip_addr, port, 1);

    if !pwd.is_null() && test_add_key_maclen(lsk, pwd, maclen, addr, prefix, sndid, rcvid) != 0 {
        test_error(b"setsockopt(TCP_AO_ADD_KEY)\0".as_ptr() as *const c_char);
    }

    if !cnt_name.is_null() {
        before_cnt = netstat_get_one(cnt_name, ptr::null_mut());
    }
    if !pwd.is_null() && test_get_tcp_counters(lsk, &mut cnt1) != 0 {
        test_error(b"test_get_tcp_counters()\0".as_ptr() as *const c_char);
    }

    synchronize_threads(); /* preparations done */

    err = test_skpair_wait_poll(lsk, 0, poll_cnt, &mut sk_pair);
    if err == -ETIMEDOUT {
        sk_pair = err;
        if !(inj == FAULT_TIMEOUT) {
            test_fail(b"%s: timed out for accept()\0".as_ptr() as *const c_char, tst_name);
        }
    } else if err == -EKEYREJECTED {
        if !(inj == FAULT_KEYREJECT) {
            test_fail(b"%s: key was rejected\0".as_ptr() as *const c_char, tst_name);
        }
    } else if err < 0 {
        test_error(b"test_skpair_wait_poll()\0".as_ptr() as *const c_char);
    } else {
        if inj == FAULT_TIMEOUT {
            test_fail(b"%s: ready to accept\0".as_ptr() as *const c_char, tst_name);
        }

        sk = accept(lsk, ptr::null_mut(), ptr::null_mut());
        if sk < 0 {
            test_error(b"accept()\0".as_ptr() as *const c_char);
        } else if inj == FAULT_TIMEOUT {
            test_fail(b"%s: accepted\0".as_ptr() as *const c_char, tst_name);
        }
    }

    synchronize_threads(); /* before counter checks */
    if !pwd.is_null() && test_get_tcp_counters(lsk, &mut cnt2) != 0 {
        test_error(b"test_get_tcp_counters()\0".as_ptr() as *const c_char);
    }

    close(lsk);

    if !pwd.is_null() {
        test_assert_counters(tst_name, &mut cnt1, &mut cnt2, cnt_expected);
    }

    if cnt_name.is_null() {
        synchronize_threads(); /* close() */
        if sk > 0 {
            close(sk);
        }
        return;
    }

    after_cnt = netstat_get_one(cnt_name, ptr::null_mut());

    if after_cnt <= before_cnt {
        test_fail(
            b"%s: %s counter did not increase: %lu <= %lu\0".as_ptr() as *const c_char,
            tst_name,
            cnt_name,
            after_cnt,
            before_cnt,
        );
    } else {
        test_ok(
            b"%s: counter %s increased %lu => %lu\0".as_ptr() as *const c_char,
            tst_name,
            cnt_name,
            before_cnt,
            after_cnt,
        );
    }

    synchronize_threads(); /* close() */
    if sk > 0 {
        close(sk);
    }
}

unsafe extern "C" fn server_fn(_arg: *mut c_void) -> *mut c_void {
    let mut wrong_addr: tcp_addr = core::mem::zeroed();
    let mut network_addr: tcp_addr = core::mem::zeroed();
    let mut port: c_uint = test_server_port;

    if inet_pton(TEST_FAMILY, TEST_WRONG_IP, &mut wrong_addr as *mut tcp_addr as *mut c_void) != 1 {
        test_error(b"Can't convert ip address %s\0".as_ptr() as *const c_char, TEST_WRONG_IP);
    }

    try_accept(b"Non-AO server + AO client\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, ptr::null(), this_ip_dest, -1i8 as uint8_t, 100, 100, 0, b"TCPAOKeyNotFound\0".as_ptr() as *const c_char, TEST_CNT_NS_KEY_NOT_FOUND, FAULT_TIMEOUT);

    try_accept(b"AO server + Non-AO client\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 100, 100, 0, b"TCPAORequired\0".as_ptr() as *const c_char, TEST_CNT_AO_REQUIRED, FAULT_TIMEOUT);

    try_accept(b"Wrong password\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, b"something that is not DEFAULT_TEST_PASSWORD\0".as_ptr() as *const c_char, this_ip_dest, -1i8 as uint8_t, 100, 100, 0, b"TCPAOBad\0".as_ptr() as *const c_char, TEST_CNT_BAD, FAULT_TIMEOUT);

    try_accept(b"Wrong rcv id\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 100, 101, 0, b"TCPAOKeyNotFound\0".as_ptr() as *const c_char, TEST_CNT_AO_KEY_NOT_FOUND, FAULT_TIMEOUT);

    try_accept(b"Wrong snd id\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 101, 100, 0, b"TCPAOGood\0".as_ptr() as *const c_char, TEST_CNT_GOOD, FAULT_TIMEOUT);

    try_accept(b"Different maclen\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 100, 100, 8, b"TCPAOBad\0".as_ptr() as *const c_char, TEST_CNT_BAD, FAULT_TIMEOUT);

    try_accept(b"Server: Wrong addr\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, wrong_addr, -1i8 as uint8_t, 100, 100, 0, b"TCPAOKeyNotFound\0".as_ptr() as *const c_char, TEST_CNT_AO_KEY_NOT_FOUND, FAULT_TIMEOUT);

    /* Key rejected by the other side, failing short through skpair */
    try_accept(b"Client: Wrong addr\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, ptr::null(), this_ip_dest, -1i8 as uint8_t, 100, 100, 0, ptr::null(), 0, FAULT_KEYREJECT);

    try_accept(b"rcv id != snd id\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 200, 100, 0, b"TCPAOGood\0".as_ptr() as *const c_char, TEST_CNT_GOOD, 0);

    if inet_pton(TEST_FAMILY, TEST_NETWORK, &mut network_addr as *mut tcp_addr as *mut c_void) != 1 {
        test_error(b"Can't convert ip address %s\0".as_ptr() as *const c_char, TEST_NETWORK);
    }

    try_accept(b"Server: prefix match\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, network_addr, 16, 100, 100, 0, b"TCPAOGood\0".as_ptr() as *const c_char, TEST_CNT_GOOD, 0);

    try_accept(b"Client: prefix match\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 100, 100, 0, b"TCPAOGood\0".as_ptr() as *const c_char, TEST_CNT_GOOD, 0);

    /* client exits */
    synchronize_threads();
    ptr::null_mut()
}

unsafe fn try_connect(
    tst_name: *const c_char,
    port: c_uint,
    pwd: *const c_char,
    addr: tcp_addr,
    prefix: uint8_t,
    sndid: uint8_t,
    rcvid: uint8_t,
    cnt_expected: test_cnt,
    inj: fault_t,
) {
    let mut cnt1: tcp_counters = core::mem::zeroed();
    let mut cnt2: tcp_counters = core::mem::zeroed();
    let sk: c_int;
    let ret: c_int;

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error(b"socket()\0".as_ptr() as *const c_char);
    }

    if !pwd.is_null() && test_add_key(sk, pwd, addr, prefix, sndid, rcvid) != 0 {
        test_error(b"setsockopt(TCP_AO_ADD_KEY)\0".as_ptr() as *const c_char);
    }

    if !pwd.is_null() && test_get_tcp_counters(sk, &mut cnt1) != 0 {
        test_error(b"test_get_tcp_counters()\0".as_ptr() as *const c_char);
    }

    synchronize_threads(); /* preparations done */

    ret = test_skpair_connect_poll(sk, this_ip_dest, port, cnt_expected, &mut sk_pair);
    synchronize_threads(); /* before counter checks */
    if ret < 0 {
        sk_pair = ret;
        if inj == FAULT_KEYREJECT && ret == -EKEYREJECTED {
            test_ok(b"%s: connect() was prevented\0".as_ptr() as *const c_char, tst_name);
        } else if ret == -ETIMEDOUT && inj == FAULT_TIMEOUT {
            test_ok(b"%s\0".as_ptr() as *const c_char, tst_name);
        } else if ret == -ECONNREFUSED && (inj == FAULT_TIMEOUT || inj == FAULT_KEYREJECT) {
            test_ok(b"%s: refused to connect\0".as_ptr() as *const c_char, tst_name);
        } else {
            test_error(b"%s: connect() returned %d\0".as_ptr() as *const c_char, tst_name, ret);
        }
        synchronize_threads(); /* close() */

        if ret > 0 {
            close(sk);
        }
        return;
    }

    if inj == FAULT_TIMEOUT || inj == FAULT_KEYREJECT {
        test_fail(b"%s: connected\0".as_ptr() as *const c_char, tst_name);
    } else {
        test_ok(b"%s: connected\0".as_ptr() as *const c_char, tst_name);
    }
    if !pwd.is_null() && ret > 0 {
        if test_get_tcp_counters(sk, &mut cnt2) != 0 {
            test_error(b"test_get_tcp_counters()\0".as_ptr() as *const c_char);
        }
        test_assert_counters(tst_name, &mut cnt1, &mut cnt2, cnt_expected);
    } else if !pwd.is_null() {
        test_tcp_counters_free(&mut cnt1);
    }

    synchronize_threads(); /* close() */

    if ret > 0 {
        close(sk);
    }
}

unsafe extern "C" fn client_fn(_arg: *mut c_void) -> *mut c_void {
    let mut wrong_addr: tcp_addr = core::mem::zeroed();
    let mut network_addr: tcp_addr = core::mem::zeroed();
    let addr_any: tcp_addr = core::mem::zeroed();
    let mut port: c_uint = test_server_port;

    if inet_pton(TEST_FAMILY, TEST_WRONG_IP, &mut wrong_addr as *mut tcp_addr as *mut c_void) != 1 {
        test_error(b"Can't convert ip address %s\0".as_ptr() as *const c_char, TEST_WRONG_IP);
    }

    trace_ao_event_expect(TCP_AO_KEY_NOT_FOUND, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0, 100, 100, -1);
    try_connect(b"Non-AO server + AO client\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 100, 100, 0, FAULT_TIMEOUT);

    trace_hash_event_expect(TCP_HASH_AO_REQUIRED, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0);
    try_connect(b"AO server + Non-AO client\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, ptr::null(), this_ip_dest, -1i8 as uint8_t, 100, 100, 0, FAULT_TIMEOUT);

    trace_ao_event_expect(TCP_AO_MISMATCH, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0, 100, 100, -1);
    try_connect(b"Wrong password\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 100, 100, 0, FAULT_TIMEOUT);

    trace_ao_event_expect(TCP_AO_KEY_NOT_FOUND, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0, 100, 100, -1);
    try_connect(b"Wrong rcv id\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 100, 100, 0, FAULT_TIMEOUT);

    /*
     * XXX: The test doesn't increase any counters, see tcp_make_synack().
     * Potentially, it can be speed up by setting sk_pair = -ETIMEDOUT
     * but the price would be increased complexity of the tracer thread.
     */
    trace_ao_event_sk_expect(TCP_AO_SYNACK_NO_KEY, this_ip_dest, addr_any, port, 0, 100, 100);
    try_connect(b"Wrong snd id\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 100, 100, 0, FAULT_TIMEOUT);

    trace_ao_event_expect(TCP_AO_WRONG_MACLEN, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0, 100, 100, -1);
    try_connect(b"Different maclen\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 100, 100, 0, FAULT_TIMEOUT);

    trace_ao_event_expect(TCP_AO_KEY_NOT_FOUND, this_ip_addr, this_ip_dest, -1, port, 0, 0, 1, 0, 0, 0, 100, 100, -1);
    try_connect(b"Server: Wrong addr\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 100, 100, 0, FAULT_TIMEOUT);

    try_connect(b"Client: Wrong addr\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, wrong_addr, -1i8 as uint8_t, 100, 100, 0, FAULT_KEYREJECT);

    try_connect(b"rcv id != snd id\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 100, 200, TEST_CNT_GOOD, 0);

    if inet_pton(TEST_FAMILY, TEST_NETWORK, &mut network_addr as *mut tcp_addr as *mut c_void) != 1 {
        test_error(b"Can't convert ip address %s\0".as_ptr() as *const c_char, TEST_NETWORK);
    }

    try_connect(b"Server: prefix match\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, this_ip_dest, -1i8 as uint8_t, 100, 100, TEST_CNT_GOOD, 0);

    try_connect(b"Client: prefix match\0".as_ptr() as *const c_char, { let old = port; port = port.wrapping_add(1); old }, DEFAULT_TEST_PASSWORD, network_addr, 16, 100, 100, TEST_CNT_GOOD, 0);

    ptr::null_mut()
}

pub unsafe fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    test_init(22, server_fn, client_fn);
    0
}
