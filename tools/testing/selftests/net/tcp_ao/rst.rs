// SPDX-License-Identifier: GPL-2.0
/*
 * The test checks that both active and passive reset have correct TCP-AO
 * signature. An "active" reset (abort) here is procured from closing
 * listen() socket with non-accepted connections in the queue:
 * inet_csk_listen_stop() => inet_child_forget() =>
 *                        => tcp_disconnect() => tcp_send_active_reset()
 *
 * The passive reset is quite hard to get on established TCP connections.
 * It could be procured from non-established states, but the synchronization
 * part from userspace in order to reliably get RST seems uneasy.
 * So, instead it's procured by corrupting SEQ number on TIMED-WAIT state.
 *
 * It's important to test both passive and active RST as they go through
 * different code-paths:
 * - tcp_send_active_reset() makes no-data skb, sends it with tcp_transmit_skb()
 * - tcp_v*_send_reset() create their reply skbs and send them with
 *   ip_send_unicast_reply()
 *
 * In both cases TCP-AO signatures have to be correct, which is verified by
 * (1) checking that the TCP-AO connection was reset and (2) TCP-AO counters.
 *
 * Author: Dmitry Safonov <dima@arista.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type time_t = i64;
type uint64_t = u64;

#[repr(C)]
pub struct netstat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcp_counters {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcp_ao_repair {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr_af {
    _private: [u8; 0],
}

#[repr(C)]
pub struct linger {
    pub l_onoff: c_int,
    pub l_linger: c_int,
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: time_t,
}

#[repr(C)]
pub struct fd_set {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcp_sock_seq {
    pub seq: u32,
}

#[repr(C)]
pub struct tcp_sock_state {
    pub out: tcp_sock_seq,
}

const SOL_SOCKET: c_int = 1;
const SO_LINGER: c_int = 13;
const SO_ERROR: c_int = 4;
const SOCK_STREAM: c_int = 1;
const IPPROTO_TCP: c_int = 6;
const ENOENT: c_int = 2;
const ETIMEDOUT: c_int = 110;
const ECONNRESET: c_int = 104;

const quota: size_t = 1000;
const packet_sz: size_t = 100;
/*
 * Backlog == 0 means 1 connection in queue, see:
 * commit 64a146513f8f ("[NET]: Revert incorrect accept queue...")
 */
const backlog: c_uint = 0;

unsafe extern "C" {
    static mut errno: c_int;
    static mut this_ip_addr: sockaddr_af;
    static mut this_ip_dest: sockaddr_af;
    static mut test_family: c_int;
    static mut test_server_port: c_uint;
    static DEFAULT_TEST_PASSWORD: *const c_char;
    static TEST_TIMEOUT_SEC: time_t;
    static TEST_RETRANSMIT_SEC: c_uint;
    static TEST_CNT_GOOD: c_int;

    fn netstat_get(ns: *mut netstat, name: *const c_char, arg: *mut c_void) -> uint64_t;
    fn netstat_read() -> *mut netstat;
    fn netstat_free(ns: *mut netstat);
    fn test_fail(fmt: *const c_char, ...);
    fn test_ok(fmt: *const c_char, ...);
    fn test_error(fmt: *const c_char, ...);
    fn test_listen_socket(addr: sockaddr_af, port: c_uint, backlog: c_uint) -> c_int;
    fn test_add_key(
        sk: c_int,
        password: *const c_char,
        dest: sockaddr_af,
        prefix: c_int,
        sndid: c_uint,
        rcvid: c_uint,
    ) -> c_int;
    fn test_add_repaired_key(
        sk: c_int,
        password: *const c_char,
        flags: c_uint,
        dest: sockaddr_af,
        prefix: c_int,
        sndid: c_uint,
        rcvid: c_uint,
    ) -> c_int;
    fn test_get_tcp_counters(sk: c_int, cnt: *mut tcp_counters) -> c_int;
    fn synchronize_threads();
    fn test_wait_fd(sk: c_int, sec: time_t, arg: c_int) -> c_int;
    fn test_server_run(sk: c_int, quota: size_t, sec: time_t) -> ssize_t;
    fn test_assert_counters(
        msg: *const c_char,
        before: *mut tcp_counters,
        after: *mut tcp_counters,
        good: c_int,
    ) -> c_int;
    fn _test_connect_socket(sk: c_int, dest: sockaddr_af, port: c_uint, async_: bool) -> c_int;
    fn test_client_verify(sk: c_int, packet_sz: size_t, nr: size_t) -> c_int;
    fn test_connect_socket(sk: c_int, dest: sockaddr_af, port: c_uint) -> c_int;
    fn test_enable_repair(sk: c_int);
    fn test_disable_repair(sk: c_int);
    fn test_sock_checkpoint(sk: c_int, img: *mut tcp_sock_state, saddr: *mut sockaddr_af);
    fn test_ao_checkpoint(sk: c_int, ao_img: *mut tcp_ao_repair);
    fn test_kill_sk(sk: c_int);
    fn test_sock_restore(
        sk: c_int,
        img: *mut tcp_sock_state,
        saddr: *mut sockaddr_af,
        dest: sockaddr_af,
        port: c_uint,
    );
    fn test_ao_restore(sk: c_int, ao_img: *mut tcp_ao_repair);
    fn test_sock_state_free(img: *mut tcp_sock_state);
    fn test_init(argc: c_int, server: unsafe extern "C" fn(*mut c_void) -> *mut c_void, client: unsafe extern "C" fn(*mut c_void) -> *mut c_void);

    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn accept(socket: c_int, address: *mut c_void, address_len: *mut socklen_t) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *mut timeval,
    ) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;

    fn FD_ZERO(set: *mut fd_set);
    fn FD_SET(fd: c_int, set: *mut fd_set);
    fn FD_CLR(fd: c_int, set: *mut fd_set);
    fn FD_ISSET(fd: c_int, set: *mut fd_set) -> c_int;
}

unsafe fn netstats_check(before: *mut netstat, after: *mut netstat, msg: *mut c_char) {
    let mut before_cnt: uint64_t;
    let mut after_cnt: uint64_t;

    before_cnt = netstat_get(before, c"TCPAORequired".as_ptr(), core::ptr::null_mut());
    after_cnt = netstat_get(after, c"TCPAORequired".as_ptr(), core::ptr::null_mut());
    if after_cnt > before_cnt {
        test_fail(
            c"Segments without AO sign (%s): %lu => %lu".as_ptr(),
            msg,
            before_cnt,
            after_cnt,
        );
    } else {
        test_ok(c"No segments without AO sign (%s)".as_ptr(), msg);
    }

    before_cnt = netstat_get(before, c"TCPAOGood".as_ptr(), core::ptr::null_mut());
    after_cnt = netstat_get(after, c"TCPAOGood".as_ptr(), core::ptr::null_mut());
    if after_cnt <= before_cnt {
        test_fail(
            c"Signed AO segments (%s): %lu => %lu".as_ptr(),
            msg,
            before_cnt,
            after_cnt,
        );
    } else {
        test_ok(
            c"Signed AO segments (%s): %lu => %lu".as_ptr(),
            msg,
            before_cnt,
            after_cnt,
        );
    }

    before_cnt = netstat_get(before, c"TCPAOBad".as_ptr(), core::ptr::null_mut());
    after_cnt = netstat_get(after, c"TCPAOBad".as_ptr(), core::ptr::null_mut());
    if after_cnt > before_cnt {
        test_fail(
            c"Segments with bad AO sign (%s): %lu => %lu".as_ptr(),
            msg,
            before_cnt,
            after_cnt,
        );
    } else {
        test_ok(c"No segments with bad AO sign (%s)".as_ptr(), msg);
    }
}

/*
 * Another way to send RST, but not through tcp_v{4,6}_send_reset()
 * is tcp_send_active_reset(), that is not in reply to inbound segment,
 * but rather active send. It uses tcp_transmit_skb(), so that should
 * work, but as it also sends RST - nice that it can be covered as well.
 */
unsafe fn close_forced(sk: c_int) {
    let mut sl: linger = linger {
        l_onoff: 0,
        l_linger: 0,
    };

    sl.l_onoff = 1;
    sl.l_linger = 0;
    if setsockopt(
        sk,
        SOL_SOCKET,
        SO_LINGER,
        &sl as *const linger as *const c_void,
        core::mem::size_of::<linger>() as socklen_t,
    ) != 0
    {
        test_error(c"setsockopt(SO_LINGER)".as_ptr());
    }
    close(sk);
}

unsafe fn test_server_active_rst(mut port: c_uint) {
    let mut cnt1: tcp_counters = core::mem::zeroed();
    let mut cnt2: tcp_counters = core::mem::zeroed();
    let bytes: ssize_t;
    let sk: c_int;
    let lsk: c_int;

    lsk = test_listen_socket(this_ip_addr, port, backlog);
    if test_add_key(lsk, DEFAULT_TEST_PASSWORD, this_ip_dest, -1, 100, 100) != 0 {
        test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
    }
    if test_get_tcp_counters(lsk, &mut cnt1) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }

    synchronize_threads(); /* 1: MKT added */
    if test_wait_fd(lsk, TEST_TIMEOUT_SEC, 0) != 0 {
        test_error(c"test_wait_fd()".as_ptr());
    }

    sk = accept(lsk, core::ptr::null_mut(), core::ptr::null_mut());
    if sk < 0 {
        test_error(c"accept()".as_ptr());
    }

    synchronize_threads(); /* 2: connection accept()ed, another queued */
    if test_get_tcp_counters(lsk, &mut cnt2) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }

    synchronize_threads(); /* 3: close listen socket */
    close(lsk);
    bytes = test_server_run(sk, quota, 0);
    if bytes != quota as ssize_t {
        test_error(c"servered only %zd bytes".as_ptr(), bytes);
    } else {
        test_ok(c"servered %zd bytes".as_ptr(), bytes);
    }

    synchronize_threads(); /* 4: finishing up */
    close_forced(sk);

    synchronize_threads(); /* 5: closed active sk */

    synchronize_threads(); /* 6: counters checks */
    if test_assert_counters(c"active RST server".as_ptr(), &mut cnt1, &mut cnt2, TEST_CNT_GOOD) != 0 {
        test_fail(c"MKT counters (server) have not only good packets".as_ptr());
    } else {
        test_ok(c"MKT counters are good on server".as_ptr());
    }
}

unsafe fn test_server_passive_rst(port: c_uint) {
    let mut cnt1: tcp_counters = core::mem::zeroed();
    let mut cnt2: tcp_counters = core::mem::zeroed();
    let sk: c_int;
    let lsk: c_int;
    let bytes: ssize_t;

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
    if test_get_tcp_counters(sk, &mut cnt1) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }

    bytes = test_server_run(sk, quota, TEST_TIMEOUT_SEC);
    if bytes != quota as ssize_t {
        if bytes > 0 {
            test_fail(c"server served: %zd".as_ptr(), bytes);
        } else {
            test_fail(c"server returned %zd".as_ptr(), bytes);
        }
    }

    synchronize_threads(); /* 3: checkpoint the client */
    synchronize_threads(); /* 4: close the server, creating twsk */
    if test_get_tcp_counters(sk, &mut cnt2) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }
    close(sk);

    synchronize_threads(); /* 5: restore the socket, send more data */
    test_assert_counters(c"passive RST server".as_ptr(), &mut cnt1, &mut cnt2, TEST_CNT_GOOD);

    synchronize_threads(); /* 6: server exits */
}

unsafe extern "C" fn server_fn(_arg: *mut c_void) -> *mut c_void {
    let ns_before: *mut netstat;
    let ns_after: *mut netstat;
    let mut port: c_uint = test_server_port;

    ns_before = netstat_read();

    test_server_active_rst(port);
    port += 1;
    test_server_passive_rst(port);
    port += 1;

    ns_after = netstat_read();
    netstats_check(ns_before, ns_after, c"server".as_ptr() as *mut c_char);
    netstat_free(ns_after);
    netstat_free(ns_before);
    synchronize_threads(); /* exit */

    synchronize_threads(); /* don't race to exit() - client exits */
    core::ptr::null_mut()
}

unsafe fn test_wait_fds(
    sk: *mut c_int,
    nr: size_t,
    is_writable: *mut bool,
    mut wait_for: ssize_t,
    sec: time_t,
) -> c_int {
    let mut tv: timeval = timeval { tv_sec: sec, tv_usec: 0 };
    let mut ptv: *mut timeval = core::ptr::null_mut();
    let mut left: fd_set = core::mem::zeroed();
    let mut i: size_t;
    let mut ret: c_int;

    FD_ZERO(&mut left);
    i = 0;
    while i < nr {
        FD_SET(*sk.add(i), &mut left);
        if !is_writable.is_null() {
            *is_writable.add(i) = false;
        }
        i += 1;
    }

    if sec != 0 {
        ptv = &mut tv;
    }

    loop {
        let mut is_empty: bool = true;
        let mut fds: fd_set = core::mem::zeroed();
        let mut efds: fd_set = core::mem::zeroed();
        let mut nfd: c_int = 0;

        FD_ZERO(&mut fds);
        FD_ZERO(&mut efds);
        i = 0;
        while i < nr {
            if FD_ISSET(*sk.add(i), &mut left) == 0 {
                i += 1;
                continue;
            }

            if *sk.add(i) > nfd {
                nfd = *sk.add(i);
            }

            FD_SET(*sk.add(i), &mut fds);
            FD_SET(*sk.add(i), &mut efds);
            is_empty = false;
            i += 1;
        }
        if is_empty {
            return -ENOENT;
        }

        errno = 0;
        ret = select(nfd + 1, core::ptr::null_mut(), &mut fds, &mut efds, ptv);
        if ret < 0 {
            return -errno;
        }
        if ret == 0 {
            return -ETIMEDOUT;
        }
        i = 0;
        while i < nr {
            if FD_ISSET(*sk.add(i), &mut fds) != 0 {
                if !is_writable.is_null() {
                    *is_writable.add(i) = true;
                }
                FD_CLR(*sk.add(i), &mut left);
                wait_for -= 1;
                i += 1;
                continue;
            }
            if FD_ISSET(*sk.add(i), &mut efds) != 0 {
                FD_CLR(*sk.add(i), &mut left);
                wait_for -= 1;
            }
            i += 1;
        }

        if wait_for <= 0 {
            break;
        }
    }

    0
}

unsafe fn test_client_active_rst(port: c_uint) {
    let mut i: c_int;
    let mut sk: [c_int; 3] = [0; 3];
    let mut err: c_int;
    let mut is_writable: [bool; 3] = [false; 3];
    let last: c_uint = sk.len() as c_uint - 1;

    i = 0;
    while i < sk.len() as c_int {
        sk[i as usize] = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
        if sk[i as usize] < 0 {
            test_error(c"socket()".as_ptr());
        }
        if test_add_key(
            sk[i as usize],
            DEFAULT_TEST_PASSWORD,
            this_ip_dest,
            -1,
            100,
            100,
        ) != 0
        {
            test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
        }
        i += 1;
    }

    synchronize_threads(); /* 1: MKT added */
    i = 0;
    while i < last as c_int {
        err = _test_connect_socket(sk[i as usize], this_ip_dest, port, i != 0);
        if err < 0 {
            test_error(c"failed to connect()".as_ptr());
        }
        i += 1;
    }

    synchronize_threads(); /* 2: two connections: one accept()ed, another queued */
    err = test_wait_fds(
        sk.as_mut_ptr(),
        last as size_t,
        is_writable.as_mut_ptr(),
        last as ssize_t,
        TEST_TIMEOUT_SEC,
    );
    if err < 0 {
        test_error(c"test_wait_fds(): %d".as_ptr(), err);
    }

    /* async connect() with third sk to get into request_sock_queue */
    err = _test_connect_socket(sk[last as usize], this_ip_dest, port, true);
    if err < 0 {
        test_error(c"failed to connect()".as_ptr());
    }

    synchronize_threads(); /* 3: close listen socket */
    if test_client_verify(sk[0], packet_sz, quota / packet_sz) != 0 {
        test_fail(c"Failed to send data on connected socket".as_ptr());
    } else {
        test_ok(c"Verified established tcp connection".as_ptr());
    }

    synchronize_threads(); /* 4: finishing up */

    synchronize_threads(); /* 5: closed active sk */
    /*
     * Wait for 2 connections: one accepted, another in the accept queue,
     * the one in request_sock_queue won't get fully established, so
     * doesn't receive an active RST, see inet_csk_listen_stop().
     */
    err = test_wait_fds(
        sk.as_mut_ptr(),
        last as size_t,
        core::ptr::null_mut(),
        last as ssize_t,
        TEST_TIMEOUT_SEC,
    );
    if err < 0 {
        test_error(c"select(): %d".as_ptr(), err);
    }

    i = 0;
    while i < sk.len() as c_int {
        let mut slen: socklen_t = core::mem::size_of::<c_int>() as socklen_t;

        if getsockopt(
            sk[i as usize],
            SOL_SOCKET,
            SO_ERROR,
            &mut err as *mut c_int as *mut c_void,
            &mut slen,
        ) != 0
        {
            test_error(c"getsockopt()".as_ptr());
        }
        if is_writable[i as usize] && err != ECONNRESET {
            test_fail(
                c"sk[%d] = %d, err = %d, connection wasn't reset".as_ptr(),
                i,
                sk[i as usize],
                err,
            );
        } else {
            test_ok(
                c"sk[%d] = %d%s".as_ptr(),
                i,
                sk[i as usize],
                if is_writable[i as usize] {
                    c", connection was reset".as_ptr()
                } else {
                    c"".as_ptr()
                },
            );
        }
        i += 1;
    }
    synchronize_threads(); /* 6: counters checks */
}

unsafe fn test_client_passive_rst(port: c_uint) {
    let mut cnt1: tcp_counters = core::mem::zeroed();
    let mut cnt2: tcp_counters = core::mem::zeroed();
    let mut ao_img: tcp_ao_repair = core::mem::zeroed();
    let mut img: tcp_sock_state = core::mem::zeroed();
    let mut saddr: sockaddr_af = core::mem::zeroed();
    let mut sk: c_int;
    let mut err: c_int;

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error(c"socket()".as_ptr());
    }

    if test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, -1, 100, 100) != 0 {
        test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
    }

    synchronize_threads(); /* 1: MKT added => connect() */
    if test_connect_socket(sk, this_ip_dest, port) <= 0 {
        test_error(c"failed to connect()".as_ptr());
    }

    synchronize_threads(); /* 2: accepted => send data */
    if test_client_verify(sk, packet_sz, quota / packet_sz) != 0 {
        test_fail(c"Failed to send data on connected socket".as_ptr());
    } else {
        test_ok(c"Verified established tcp connection".as_ptr());
    }

    synchronize_threads(); /* 3: checkpoint the client */
    test_enable_repair(sk);
    test_sock_checkpoint(sk, &mut img, &mut saddr);
    test_ao_checkpoint(sk, &mut ao_img);
    test_disable_repair(sk);

    synchronize_threads(); /* 4: close the server, creating twsk */

    /*
     * The "corruption" in SEQ has to be small enough to fit into TCP
     * window, see tcp_timewait_state_process() for out-of-window
     * segments.
     */
    img.out.seq = img.out.seq.wrapping_add(5); /* 5 is more noticeable in tcpdump than 1 */

    /*
     * FIXME: This is kind-of ugly and dirty, but it works.
     *
     * At this moment, the server has close'ed(sk).
     * The passive RST that is being targeted here is new data after
     * half-duplex close, see tcp_timewait_state_process() => TCP_TW_RST
     *
     * What is needed here is:
     * (1) wait for FIN from the server
     * (2) make sure that the ACK from the client went out
     * (3) make sure that the ACK was received and processed by the server
     *
     * Otherwise, the data that will be sent from "repaired" socket
     * post SEQ corruption may get to the server before it's in
     * TCP_FIN_WAIT2.
     *
     * (1) is easy with select()/poll()
     * (2) is possible by polling tcpi_state from TCP_INFO
     * (3) is quite complex: as server's socket was already closed,
     *     probably the way to do it would be tcp-diag.
     */
    sleep(TEST_RETRANSMIT_SEC);

    synchronize_threads(); /* 5: restore the socket, send more data */
    test_kill_sk(sk);

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error(c"socket()".as_ptr());
    }

    test_enable_repair(sk);
    test_sock_restore(sk, &mut img, &mut saddr, this_ip_dest, port);
    if test_add_repaired_key(sk, DEFAULT_TEST_PASSWORD, 0, this_ip_dest, -1, 100, 100) != 0 {
        test_error(c"setsockopt(TCP_AO_ADD_KEY)".as_ptr());
    }
    test_ao_restore(sk, &mut ao_img);

    if test_get_tcp_counters(sk, &mut cnt1) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }

    test_disable_repair(sk);
    test_sock_state_free(&mut img);

    /*
     * This is how "passive reset" is acquired in this test from TCP_TW_RST:
     *
     * IP 10.0.254.1.7011 > 10.0.1.1.59772: Flags [P.], seq 901:1001, ack 1001, win 249,
     *    options [tcp-ao keyid 100 rnextkeyid 100 mac 0x10217d6c36a22379086ef3b1], length 100
     * IP 10.0.254.1.7011 > 10.0.1.1.59772: Flags [F.], seq 1001, ack 1001, win 249,
     *    options [tcp-ao keyid 100 rnextkeyid 100 mac 0x104ffc99b98c10a5298cc268], length 0
     * IP 10.0.1.1.59772 > 10.0.254.1.7011: Flags [.], ack 1002, win 251,
     *    options [tcp-ao keyid 100 rnextkeyid 100 mac 0xe496dd4f7f5a8a66873c6f93,nop,nop,sack 1 {1001:1002}], length 0
     * IP 10.0.1.1.59772 > 10.0.254.1.7011: Flags [P.], seq 1006:1106, ack 1001, win 251,
     *    options [tcp-ao keyid 100 rnextkeyid 100 mac 0x1b5f3330fb23fbcd0c77d0ca], length 100
     * IP 10.0.254.1.7011 > 10.0.1.1.59772: Flags [R], seq 3215596252, win 0,
     *    options [tcp-ao keyid 100 rnextkeyid 100 mac 0x0bcfbbf497bce844312304b2], length 0
     */
    err = test_client_verify(sk, packet_sz, quota / packet_sz);
    /* Make sure that the connection was reset, not timeouted */
    if err != 0 && err == -ECONNRESET {
        test_ok(c"client sock was passively reset post-seq-adjust".as_ptr());
    } else if err != 0 {
        test_fail(c"client sock was not reset post-seq-adjust: %d".as_ptr(), err);
    } else {
        test_fail(c"client sock is yet connected post-seq-adjust".as_ptr());
    }

    if test_get_tcp_counters(sk, &mut cnt2) != 0 {
        test_error(c"test_get_tcp_counters()".as_ptr());
    }

    synchronize_threads(); /* 6: server exits */
    close(sk);
    test_assert_counters(c"client passive RST".as_ptr(), &mut cnt1, &mut cnt2, TEST_CNT_GOOD);
}

unsafe extern "C" fn client_fn(_arg: *mut c_void) -> *mut c_void {
    let ns_before: *mut netstat;
    let ns_after: *mut netstat;
    let mut port: c_uint = test_server_port;

    ns_before = netstat_read();

    test_client_active_rst(port);
    port += 1;
    test_client_passive_rst(port);
    port += 1;

    ns_after = netstat_read();
    netstats_check(ns_before, ns_after, c"client".as_ptr() as *mut c_char);
    netstat_free(ns_after);
    netstat_free(ns_before);

    synchronize_threads(); /* exit */
    core::ptr::null_mut()
}

fn main() {
    unsafe {
        test_init(15, server_fn, client_fn);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
