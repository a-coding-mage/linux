// SPDX-License-Identifier: GPL-2.0
/* Author: Dmitry Safonov <dima@arista.com> */
/* This is over-simplified TCP_REPAIR for TCP_ESTABLISHED sockets
 * It tests that TCP-AO enabled connection can be restored.
 * For the proper socket repair see:
 * https://github.com/checkpoint-restore/criu/blob/criu-dev/soccr/soccr.h
 */
// C dependencies: <inttypes.h>, "aolib.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

const nr_packets: usize = 20;
const msg_len: usize = 100;
const quota: usize = nr_packets * msg_len;

fn fault(inj: fault_t, type_: fault_t) -> bool {
    inj == type_
}

unsafe fn try_server_run(
    tst_name: *const c_char,
    mut port: c_uint,
    inj: fault_t,
    cnt_expected: test_cnt,
) {
    let poll_cnt: test_cnt = if cnt_expected == TEST_CNT_GOOD {
        0
    } else {
        cnt_expected
    };
    let mut cnt_name: *const c_char = c"TCPAOGood".as_ptr();
    let mut cnt1: tcp_counters = core::mem::zeroed();
    let mut cnt2: tcp_counters = core::mem::zeroed();
    let before_cnt: u64;
    let after_cnt: u64;
    let sk: c_int;
    let lsk: c_int;
    let mut dummy: c_int = 0;
    let mut bytes: ssize_t;

    if fault(inj, FAULT_TIMEOUT) {
        cnt_name = c"TCPAOBad".as_ptr();
    }
    lsk = test_listen_socket(this_ip_addr, port, 1);

    if test_add_key(lsk, DEFAULT_TEST_PASSWORD, this_ip_dest, -1, 100, 100) != 0 {
        test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
    }
    synchronize_threads(); /* 1: MKT added => connect() */

    if test_wait_fd(lsk, TEST_TIMEOUT_SEC, 0) != 0 {
        test_error(c"test_wait_fd()".as_ptr());
    }

    sk = accept(lsk, core::ptr::null_mut(), core::ptr::null_mut());
    if sk < 0 {
        test_error(c"accept()".as_ptr());
    }

    synchronize_threads(); /* 2: accepted => send data */
    close(lsk);

    bytes = test_server_run(sk, quota, TEST_TIMEOUT_SEC);
    if bytes != quota as ssize_t {
        test_fail(c"%s: server served: %zd".as_ptr(), tst_name, bytes);
        close(sk);
        return;
    }

    before_cnt = netstat_get_one(cnt_name, core::ptr::null_mut());
    if test_get_tcp_counters(sk, &mut cnt1) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }

    bytes = test_skpair_server(sk, quota, poll_cnt, &mut dummy);
    if fault(inj, FAULT_TIMEOUT) {
        if bytes > 0 {
            test_fail(c"%s: server served: %zd".as_ptr(), tst_name, bytes);
        } else {
            test_ok(c"%s: server couldn't serve".as_ptr(), tst_name);
        }
    } else if bytes != quota as ssize_t {
        test_fail(c"%s: server served: %zd".as_ptr(), tst_name, bytes);
    } else {
        test_ok(c"%s: server alive".as_ptr(), tst_name);
    }
    synchronize_threads(); /* 3: counters checks */
    if test_get_tcp_counters(sk, &mut cnt2) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }
    after_cnt = netstat_get_one(cnt_name, core::ptr::null_mut());

    test_assert_counters(tst_name, &cnt1, &cnt2, cnt_expected);

    if after_cnt <= before_cnt {
        test_fail(
            c"%s(server): %s counter did not increase: %llu <= %llu".as_ptr(),
            tst_name,
            cnt_name,
            after_cnt,
            before_cnt,
        );
    } else {
        test_ok(
            c"%s(server): counter %s increased %llu => %llu".as_ptr(),
            tst_name,
            cnt_name,
            before_cnt,
            after_cnt,
        );
    }

    /*
     * Before close() as that will send FIN and move the peer in TCP_CLOSE
     * and that will prevent reading AO counters from the peer's socket.
     */
    synchronize_threads(); /* 4: verified => closed */
    close(sk);
}

unsafe extern "C" fn server_fn(_arg: *mut c_void) -> *mut c_void {
    let mut port: c_uint = test_server_port;

    try_server_run(
        c"TCP-AO migrate to another socket (server)".as_ptr(),
        port,
        0,
        TEST_CNT_GOOD,
    );
    port += 1;
    try_server_run(
        c"TCP-AO with wrong send ISN (server)".as_ptr(),
        port,
        FAULT_TIMEOUT,
        TEST_CNT_BAD,
    );
    port += 1;
    try_server_run(
        c"TCP-AO with wrong receive ISN (server)".as_ptr(),
        port,
        FAULT_TIMEOUT,
        TEST_CNT_BAD,
    );
    port += 1;
    try_server_run(
        c"TCP-AO with wrong send SEQ ext number (server)".as_ptr(),
        port,
        FAULT_TIMEOUT,
        TEST_CNT_BAD,
    );
    port += 1;
    try_server_run(
        c"TCP-AO with wrong receive SEQ ext number (server)".as_ptr(),
        port,
        FAULT_TIMEOUT,
        TEST_CNT_NS_BAD | TEST_CNT_GOOD,
    );

    synchronize_threads(); /* don't race to exit: client exits */
    core::ptr::null_mut()
}

unsafe fn test_get_sk_checkpoint(
    server_port: c_uint,
    saddr: *mut sockaddr_af,
    img: *mut tcp_sock_state,
    ao_img: *mut tcp_ao_repair,
) {
    let sk: c_int;

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error(c"socket()".as_ptr());
    }

    if test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, -1, 100, 100) != 0 {
        test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
    }

    synchronize_threads(); /* 1: MKT added => connect() */
    if test_connect_socket(sk, this_ip_dest, server_port) <= 0 {
        test_error(c"failed to connect()".as_ptr());
    }

    synchronize_threads(); /* 2: accepted => send data */
    if test_client_verify(sk, msg_len, nr_packets) != 0 {
        test_fail(c"pre-migrate verify failed".as_ptr());
    }

    test_enable_repair(sk);
    test_sock_checkpoint(sk, img, saddr);
    test_ao_checkpoint(sk, ao_img);
    test_kill_sk(sk);
}

unsafe fn test_sk_restore(
    tst_name: *const c_char,
    server_port: c_uint,
    saddr: *mut sockaddr_af,
    img: *mut tcp_sock_state,
    ao_img: *mut tcp_ao_repair,
    inj: fault_t,
    cnt_expected: test_cnt,
) {
    let poll_cnt: test_cnt = if cnt_expected == TEST_CNT_GOOD {
        0
    } else {
        cnt_expected
    };
    let mut cnt_name: *const c_char = c"TCPAOGood".as_ptr();
    let mut cnt1: tcp_counters = core::mem::zeroed();
    let mut cnt2: tcp_counters = core::mem::zeroed();
    let before_cnt: u64;
    let after_cnt: u64;
    let sk: c_int;
    let mut dummy: c_int = 0;

    if fault(inj, FAULT_TIMEOUT) {
        cnt_name = c"TCPAOBad".as_ptr();
    }

    before_cnt = netstat_get_one(cnt_name, core::ptr::null_mut());
    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error(c"socket()".as_ptr());
    }

    test_enable_repair(sk);
    test_sock_restore(sk, img, saddr, this_ip_dest, server_port);
    if test_add_repaired_key(sk, DEFAULT_TEST_PASSWORD, 0, this_ip_dest, -1, 100, 100) != 0 {
        test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
    }
    test_ao_restore(sk, ao_img);

    if test_get_tcp_counters(sk, &mut cnt1) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }

    test_disable_repair(sk);
    test_sock_state_free(img);

    if test_skpair_client(sk, msg_len, nr_packets, poll_cnt, &mut dummy) != 0 {
        if fault(inj, FAULT_TIMEOUT) {
            test_ok(c"%s: post-migrate connection is broken".as_ptr(), tst_name);
        } else {
            test_fail(c"%s: post-migrate connection is working".as_ptr(), tst_name);
        }
    } else if fault(inj, FAULT_TIMEOUT) {
        test_fail(c"%s: post-migrate connection is working".as_ptr(), tst_name);
    } else {
        test_ok(c"%s: post-migrate connection is alive".as_ptr(), tst_name);
    }

    synchronize_threads(); /* 3: counters checks */
    if test_get_tcp_counters(sk, &mut cnt2) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }
    after_cnt = netstat_get_one(cnt_name, core::ptr::null_mut());

    test_assert_counters(tst_name, &cnt1, &cnt2, cnt_expected);

    if after_cnt <= before_cnt {
        test_fail(
            c"%s: %s counter did not increase: %llu <= %llu".as_ptr(),
            tst_name,
            cnt_name,
            after_cnt,
            before_cnt,
        );
    } else {
        test_ok(
            c"%s: counter %s increased %llu => %llu".as_ptr(),
            tst_name,
            cnt_name,
            before_cnt,
            after_cnt,
        );
    }
    synchronize_threads(); /* 4: verified => closed */
    close(sk);
}

unsafe extern "C" fn client_fn(_arg: *mut c_void) -> *mut c_void {
    let mut port: c_uint = test_server_port;
    let mut tcp_img: tcp_sock_state = core::mem::zeroed();
    let mut ao_img: tcp_ao_repair = core::mem::zeroed();
    let mut saddr: sockaddr_af = core::mem::zeroed();

    test_get_sk_checkpoint(port, &mut saddr, &mut tcp_img, &mut ao_img);
    test_sk_restore(
        c"TCP-AO migrate to another socket (client)".as_ptr(),
        port,
        &mut saddr,
        &mut tcp_img,
        &mut ao_img,
        0,
        TEST_CNT_GOOD,
    );
    port += 1;

    test_get_sk_checkpoint(port, &mut saddr, &mut tcp_img, &mut ao_img);
    ao_img.snt_isn += 1;
    trace_ao_event_expect(
        TCP_AO_MISMATCH,
        this_ip_addr,
        this_ip_dest,
        -1,
        port as c_int,
        0,
        -1,
        -1,
        -1,
        -1,
        -1,
        100,
        100,
        -1,
    );
    trace_ao_event_expect(
        TCP_AO_MISMATCH,
        this_ip_dest,
        this_ip_addr,
        port as c_int,
        -1,
        0,
        -1,
        -1,
        -1,
        -1,
        -1,
        100,
        100,
        -1,
    );
    test_sk_restore(
        c"TCP-AO with wrong send ISN (client)".as_ptr(),
        port,
        &mut saddr,
        &mut tcp_img,
        &mut ao_img,
        FAULT_TIMEOUT,
        TEST_CNT_BAD,
    );
    port += 1;

    test_get_sk_checkpoint(port, &mut saddr, &mut tcp_img, &mut ao_img);
    ao_img.rcv_isn += 1;
    trace_ao_event_expect(
        TCP_AO_MISMATCH,
        this_ip_addr,
        this_ip_dest,
        -1,
        port as c_int,
        0,
        -1,
        -1,
        -1,
        -1,
        -1,
        100,
        100,
        -1,
    );
    trace_ao_event_expect(
        TCP_AO_MISMATCH,
        this_ip_dest,
        this_ip_addr,
        port as c_int,
        -1,
        0,
        -1,
        -1,
        -1,
        -1,
        -1,
        100,
        100,
        -1,
    );
    test_sk_restore(
        c"TCP-AO with wrong receive ISN (client)".as_ptr(),
        port,
        &mut saddr,
        &mut tcp_img,
        &mut ao_img,
        FAULT_TIMEOUT,
        TEST_CNT_BAD,
    );
    port += 1;

    test_get_sk_checkpoint(port, &mut saddr, &mut tcp_img, &mut ao_img);
    ao_img.snd_sne += 1;
    trace_ao_event_expect(
        TCP_AO_MISMATCH,
        this_ip_addr,
        this_ip_dest,
        -1,
        port as c_int,
        0,
        -1,
        -1,
        -1,
        -1,
        -1,
        100,
        100,
        -1,
    );
    /* not expecting server => client mismatches as only snd sne is broken */
    test_sk_restore(
        c"TCP-AO with wrong send SEQ ext number (client)".as_ptr(),
        port,
        &mut saddr,
        &mut tcp_img,
        &mut ao_img,
        FAULT_TIMEOUT,
        TEST_CNT_NS_BAD | TEST_CNT_GOOD,
    );
    port += 1;

    test_get_sk_checkpoint(port, &mut saddr, &mut tcp_img, &mut ao_img);
    ao_img.rcv_sne += 1;
    /* not expecting client => server mismatches as only rcv sne is broken */
    trace_ao_event_expect(
        TCP_AO_MISMATCH,
        this_ip_dest,
        this_ip_addr,
        port as c_int,
        -1,
        0,
        -1,
        -1,
        -1,
        -1,
        -1,
        100,
        100,
        -1,
    );
    test_sk_restore(
        c"TCP-AO with wrong receive SEQ ext number (client)".as_ptr(),
        port,
        &mut saddr,
        &mut tcp_img,
        &mut ao_img,
        FAULT_TIMEOUT,
        TEST_CNT_NS_GOOD | TEST_CNT_BAD,
    );

    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    test_init(21, Some(server_fn), Some(client_fn));
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
