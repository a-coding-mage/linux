// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/sockmap_strp.c.
// C includes referenced external test/libbpf/socket APIs supplied elsewhere.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const STRP_PKT_HEAD_LEN: c_int = 4;
const STRP_PKT_BODY_LEN: c_int = 6;
const STRP_PKT_FULL_LEN: c_int = STRP_PKT_HEAD_LEN + STRP_PKT_BODY_LEN;

const EAGAIN: c_int = 11;
const MSG_DONTWAIT: c_int = 0x40;
const IPPROTO_TCP: c_int = 6;
const TCP_NODELAY: c_int = 1;
const BPF_NOEXIST: u64 = 1;
const BPF_SK_SKB_STREAM_PARSER: c_uint = 4;
const BPF_SK_SKB_STREAM_VERDICT: c_uint = 5;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const FIONREAD: c_ulong = 0x541b;

type c_ulong = u64;

static packet: [c_char; STRP_PKT_FULL_LEN as usize] =
    [b'h' as c_char, b'e' as c_char, b'a' as c_char, b'd' as c_char,
     b'+' as c_char, b'b' as c_char, b'o' as c_char, b'd' as c_char,
     b'y' as c_char, 0];
static test_packet_num: c_int = 100;

#[repr(C)]
pub struct test_sockmap_strp {
    pub maps: test_sockmap_strp_maps,
    pub progs: test_sockmap_strp_progs,
    pub data: *mut test_sockmap_strp_data,
}

#[repr(C)]
pub struct test_sockmap_strp_maps {
    pub sock_map: *mut bpf_map,
}

#[repr(C)]
pub struct test_sockmap_strp_progs {
    pub prog_skb_parser_partial: *mut bpf_program,
    pub prog_skb_parser: *mut bpf_program,
    pub prog_skb_verdict_pass: *mut bpf_program,
    pub prog_skb_verdict: *mut bpf_program,
    pub prog_skb_parser_resize: *mut bpf_program,
}

#[repr(C)]
pub struct test_sockmap_strp_data {
    pub verdict_max_size: c_int,
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;
    static IO_TIMEOUT_SEC: c_int;

    fn recv_timeout(fd: c_int, buf: *mut c_void, len: usize, flags: c_int, timeout_sec: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: u32) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;

    fn test_sockmap_strp__open_and_load() -> *mut test_sockmap_strp;
    fn test_sockmap_strp__destroy(obj: *mut test_sockmap_strp);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, attach_type: c_uint, flags: c_uint) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_link_create(prog_fd: c_int, target_fd: c_int, attach_type: c_uint, opts: *const c_void) -> c_int;
    fn bpf_link_update(link_fd: c_int, new_prog_fd: c_int, opts: *const c_void) -> c_int;

    fn create_socket_pairs(
        family: c_int,
        sotype: c_int,
        c0: *mut c_int,
        c1: *mut c_int,
        p0: *mut c_int,
        p1: *mut c_int,
    ) -> c_int;
    fn create_pair(family: c_int, sotype: c_int, c: *mut c_int, p: *mut c_int) -> c_int;
    fn xsend(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> c_int;

    fn test__start_subtest(name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(actual: c_int, name: *const c_char) -> bool;
    fn ASSERT_TRUE(actual: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(actual: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR(actual: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

/* Current implementation of tcp_bpf_recvmsg_parser() invokes data_ready
 * with sk held if an skb exists in sk_receive_queue. Then for the
 * data_ready implementation of strparser, it will delay the read
 * operation if sk is held and EAGAIN is returned.
 */
unsafe fn sockmap_strp_consume_pre_data(p: c_int) -> c_int {
    let mut recvd: c_int;
    let mut retried = false;
    let mut rcv: [c_char; 10] = [0; 10];

    loop {
        errno = 0;
        recvd = recv_timeout(p, rcv.as_mut_ptr() as *mut c_void, size_of::<[c_char; 10]>(), 0, 1);
        if recvd < 0 && errno == EAGAIN && retried == false {
            /* On the first call, EAGAIN will certainly be returned.
             * A 1-second wait is enough for the workqueue to finish.
             */
            sleep(1);
            retried = true;
            continue;
        }
        break;
    }

    if !ASSERT_EQ(recvd, STRP_PKT_FULL_LEN, cstr!("recv error or truncated data"))
        || !ASSERT_OK(
            memcmp(packet.as_ptr() as *const c_void, rcv.as_ptr() as *const c_void, STRP_PKT_FULL_LEN as usize),
            cstr!("data mismatch"),
        )
    {
        return -1;
    }
    0
}

unsafe fn sockmap_strp_init(out_map: *mut c_int, pass: bool, need_parser: bool) -> *mut test_sockmap_strp {
    let mut strp: *mut test_sockmap_strp = ptr::null_mut();
    let verdict: c_int;
    let parser: c_int;
    let mut err: c_int;

    strp = test_sockmap_strp__open_and_load();
    *out_map = bpf_map__fd((*strp).maps.sock_map);

    if need_parser {
        parser = bpf_program__fd((*strp).progs.prog_skb_parser_partial);
    } else {
        parser = bpf_program__fd((*strp).progs.prog_skb_parser);
    }

    if pass {
        verdict = bpf_program__fd((*strp).progs.prog_skb_verdict_pass);
    } else {
        verdict = bpf_program__fd((*strp).progs.prog_skb_verdict);
    }

    err = bpf_prog_attach(parser, *out_map, BPF_SK_SKB_STREAM_PARSER, 0);
    if !ASSERT_OK(err, cstr!("bpf_prog_attach stream parser")) {
        test_sockmap_strp__destroy(strp);
        return ptr::null_mut();
    }

    err = bpf_prog_attach(verdict, *out_map, BPF_SK_SKB_STREAM_VERDICT, 0);
    if !ASSERT_OK(err, cstr!("bpf_prog_attach stream verdict")) {
        test_sockmap_strp__destroy(strp);
        return ptr::null_mut();
    }

    strp
}

#[repr(C)]
struct SendDir {
    data: [c_char; 7],
    data_len: c_int,
    send_cnt: c_int,
    receiver: *mut c_int,
}

/* Dispatch packets to different socket by packet size:
 *
 *                      ------  ------
 *                     | pkt4 || pkt1 |... > remote socket
 *  ------ ------     / ------  ------
 * | pkt8 | pkt7 |...
 *  ------ ------     \ ------  ------
 *                     | pkt3 || pkt2 |... > local socket
 *                      ------  ------
 */
unsafe fn test_sockmap_strp_dispatch_pkt(family: c_int, sotype: c_int) {
    let mut zero: c_int = 0;
    let mut one: c_int = 1;
    let mut recvd: c_int;
    let mut err: c_int;
    let mut map: c_int = 0;
    let mut c0: c_int = -1;
    let mut p0: c_int = -1;
    let mut c1: c_int = -1;
    let mut p1: c_int = -1;
    let mut strp: *mut test_sockmap_strp = ptr::null_mut();
    let test_cnt: c_int = 6;
    let mut rcv: [c_char; 10] = [0; 10];
    let mut send_dir = [
        /* data expected to deliver to local */
        SendDir { data: [b'l' as c_char, b'l' as c_char, b'l' as c_char, b'l' as c_char, b'l' as c_char, b'l' as c_char, 0], data_len: 6, send_cnt: 0, receiver: &mut p0 },
        /* data expected to deliver to remote */
        SendDir { data: [b'r' as c_char, b'r' as c_char, b'r' as c_char, b'r' as c_char, b'r' as c_char, 0, 0], data_len: 5, send_cnt: 0, receiver: &mut c1 },
    ];

    strp = sockmap_strp_init(&mut map, false, false);
    if !ASSERT_TRUE(strp as *const c_void, cstr!("sockmap_strp_init")) {
        return;
    }

    'out: {
        err = create_socket_pairs(family, sotype, &mut c0, &mut c1, &mut p0, &mut p1);
        if !ASSERT_OK(err, cstr!("create_socket_pairs()")) {
            break 'out;
        }

        'out_close: {
            err = bpf_map_update_elem(map, &mut zero as *mut _ as *const c_void, &mut p0 as *mut _ as *const c_void, BPF_NOEXIST);
            if !ASSERT_OK(err, cstr!("bpf_map_update_elem(p0)")) {
                break 'out_close;
            }

            err = bpf_map_update_elem(map, &mut one as *mut _ as *const c_void, &mut p1 as *mut _ as *const c_void, BPF_NOEXIST);
            if !ASSERT_OK(err, cstr!("bpf_map_update_elem(p1)")) {
                break 'out_close;
            }

            err = setsockopt(c1, IPPROTO_TCP, TCP_NODELAY, &mut zero as *mut _ as *const c_void, size_of::<c_int>() as u32);
            if !ASSERT_OK(err, cstr!("setsockopt(TCP_NODELAY)")) {
                break 'out_close;
            }

            /* deliver data with data size greater than 5 to local */
            (*(*strp).data).verdict_max_size = 5;

            for i in 0..test_cnt {
                let d = (i % 2) as usize;

                xsend(c0, send_dir[d].data.as_ptr() as *const c_void, send_dir[d].data_len as usize, 0);
                send_dir[d].send_cnt += 1;
            }

            for i in 0..2usize {
                for _j in 0..send_dir[i].send_cnt {
                    let expected = send_dir[i].data_len;

                    recvd = recv_timeout(
                        *send_dir[i].receiver,
                        rcv.as_mut_ptr() as *mut c_void,
                        expected as usize,
                        MSG_DONTWAIT,
                        IO_TIMEOUT_SEC,
                    );
                    if !ASSERT_EQ(recvd, expected, cstr!("recv_timeout()")) {
                        break 'out_close;
                    }
                    if !ASSERT_OK(
                        memcmp(send_dir[i].data.as_ptr() as *const c_void, rcv.as_ptr() as *const c_void, recvd as usize),
                        cstr!("data mismatch"),
                    ) {
                        break 'out_close;
                    }
                }
            }
        }
        close(c0);
        close(c1);
        close(p0);
        close(p1);
    }
    test_sockmap_strp__destroy(strp);
}

/* We have multiple packets in one skb
 * ------------ ------------ ------------
 * |  packet1  |   packet2  |  ...
 * ------------ ------------ ------------
 */
unsafe fn test_sockmap_strp_multiple_pkt(family: c_int, sotype: c_int) {
    let mut zero: c_int = 0;
    let mut sent: c_int;
    let mut recvd: c_int;
    let total: c_int;
    let mut err: c_int;
    let mut map: c_int = 0;
    let mut c: c_int = -1;
    let mut p: c_int = -1;
    let mut strp: *mut test_sockmap_strp = ptr::null_mut();
    let mut snd: *mut c_char = ptr::null_mut();
    let mut rcv: *mut c_char = ptr::null_mut();

    strp = sockmap_strp_init(&mut map, true, true);
    if !ASSERT_TRUE(strp as *const c_void, cstr!("sockmap_strp_init")) {
        return;
    }

    'out: {
        err = create_pair(family, sotype, &mut c, &mut p);
        if err != 0 {
            break 'out;
        }

        'out_close: {
            err = bpf_map_update_elem(map, &mut zero as *mut _ as *const c_void, &mut p as *mut _ as *const c_void, BPF_NOEXIST);
            if !ASSERT_OK(err, cstr!("bpf_map_update_elem(zero, p)")) {
                break 'out_close;
            }

            /* construct multiple packets in one buffer */
            total = test_packet_num * STRP_PKT_FULL_LEN;
            snd = malloc(total as usize) as *mut c_char;
            rcv = malloc((total + 1) as usize) as *mut c_char;
            if !ASSERT_TRUE(snd as *const c_void, cstr!("malloc(snd)"))
                || !ASSERT_TRUE(rcv as *const c_void, cstr!("malloc(rcv)"))
            {
                break 'out_close;
            }

            for i in 0..test_packet_num {
                ptr::copy_nonoverlapping(
                    packet.as_ptr(),
                    snd.add((i * STRP_PKT_FULL_LEN) as usize),
                    STRP_PKT_FULL_LEN as usize,
                );
            }

            sent = xsend(c, snd as *const c_void, total as usize, 0);
            if !ASSERT_EQ(sent, total, cstr!("xsend(c)")) {
                break 'out_close;
            }

            /* try to recv one more byte to avoid truncation check */
            recvd = recv_timeout(p, rcv as *mut c_void, (total + 1) as usize, MSG_DONTWAIT, IO_TIMEOUT_SEC);
            if !ASSERT_EQ(recvd, total, cstr!("recv(rcv)")) {
                break 'out_close;
            }

            /* we sent TCP segment with multiple encapsulation
             * then check whether packets are handled correctly
             */
            if !ASSERT_OK(memcmp(snd as *const c_void, rcv as *const c_void, total as usize), cstr!("data mismatch")) {
                break 'out_close;
            }
        }
        close(c);
        close(p);
        if !snd.is_null() {
            free(snd as *mut c_void);
        }
        if !rcv.is_null() {
            free(rcv as *mut c_void);
        }
    }
    test_sockmap_strp__destroy(strp);
}

/* Test strparser with partial read */
unsafe fn test_sockmap_strp_partial_read(family: c_int, sotype: c_int) {
    let mut zero: c_int = 0;
    let mut recvd: c_int;
    let mut off: c_int;
    let mut err: c_int;
    let mut map: c_int = 0;
    let mut c: c_int = -1;
    let mut p: c_int = -1;
    let mut strp: *mut test_sockmap_strp = ptr::null_mut();
    let mut rcv: [c_char; (STRP_PKT_FULL_LEN + 1) as usize] = [0; (STRP_PKT_FULL_LEN + 1) as usize];
    rcv[0] = b'0' as c_char;

    strp = sockmap_strp_init(&mut map, true, true);
    if !ASSERT_TRUE(strp as *const c_void, cstr!("sockmap_strp_init")) {
        return;
    }

    'out: {
        err = create_pair(family, sotype, &mut c, &mut p);
        if err != 0 {
            break 'out;
        }

        'out_close: {
            /* sk_data_ready of 'p' will be replaced by strparser handler */
            err = bpf_map_update_elem(map, &mut zero as *mut _ as *const c_void, &mut p as *mut _ as *const c_void, BPF_NOEXIST);
            if !ASSERT_OK(err, cstr!("bpf_map_update_elem(zero, p)")) {
                break 'out_close;
            }

            /* 1.1 send partial head, 1 byte header left */
            off = STRP_PKT_HEAD_LEN - 1;
            xsend(c, packet.as_ptr() as *const c_void, off as usize, 0);
            recvd = recv_timeout(p, rcv.as_mut_ptr() as *mut c_void, size_of_val(&rcv), MSG_DONTWAIT, 1);
            if !ASSERT_EQ(-1, recvd, cstr!("partial head sent, expected no data")) {
                break 'out_close;
            }

            /* 1.2 send remaining head and body */
            xsend(c, packet.as_ptr().add(off as usize) as *const c_void, (STRP_PKT_FULL_LEN - off) as usize, 0);
            recvd = recv_timeout(p, rcv.as_mut_ptr() as *mut c_void, size_of_val(&rcv), MSG_DONTWAIT, IO_TIMEOUT_SEC);
            if !ASSERT_EQ(recvd, STRP_PKT_FULL_LEN, cstr!("expected full data")) {
                break 'out_close;
            }

            /* 2.1 send partial head, 1 byte header left */
            off = STRP_PKT_HEAD_LEN - 1;
            xsend(c, packet.as_ptr() as *const c_void, off as usize, 0);

            /* 2.2 send remaining head and partial body, 1 byte body left */
            xsend(c, packet.as_ptr().add(off as usize) as *const c_void, (STRP_PKT_FULL_LEN - off - 1) as usize, 0);
            off = STRP_PKT_FULL_LEN - 1;
            recvd = recv_timeout(p, rcv.as_mut_ptr() as *mut c_void, size_of_val(&rcv), MSG_DONTWAIT, 1);
            if !ASSERT_EQ(-1, recvd, cstr!("partial body sent, expected no data")) {
                break 'out_close;
            }

            /* 2.3 send remaining body */
            xsend(c, packet.as_ptr().add(off as usize) as *const c_void, (STRP_PKT_FULL_LEN - off) as usize, 0);
            recvd = recv_timeout(p, rcv.as_mut_ptr() as *mut c_void, size_of_val(&rcv), MSG_DONTWAIT, IO_TIMEOUT_SEC);
            if !ASSERT_EQ(recvd, STRP_PKT_FULL_LEN, cstr!("expected full data")) {
                break 'out_close;
            }
        }
        close(c);
        close(p);
    }
    test_sockmap_strp__destroy(strp);
}

unsafe fn size_of_val<T: ?Sized>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

/* Test simple socket read/write with strparser + FIONREAD */
unsafe fn test_sockmap_strp_pass(family: c_int, sotype: c_int, fionread: bool) {
    let mut zero: c_int = 0;
    let pkt_size: c_int = STRP_PKT_FULL_LEN;
    let mut sent: c_int;
    let mut recvd: c_int;
    let mut avail: c_int = 0;
    let mut err: c_int;
    let mut map: c_int = 0;
    let mut c: c_int = -1;
    let mut p: c_int = -1;
    let test_cnt: c_int = 10;
    let mut strp: *mut test_sockmap_strp = ptr::null_mut();
    let mut rcv: [c_char; (STRP_PKT_FULL_LEN + 1) as usize] = [0; (STRP_PKT_FULL_LEN + 1) as usize];
    rcv[0] = b'0' as c_char;

    strp = sockmap_strp_init(&mut map, true, true);
    if !ASSERT_TRUE(strp as *const c_void, cstr!("sockmap_strp_init")) {
        return;
    }

    'out: {
        err = create_pair(family, sotype, &mut c, &mut p);
        if err != 0 {
            break 'out;
        }

        'out_close: {
            /* inject some data before bpf process, it should be read
             * correctly because we check sk_receive_queue in
             * tcp_bpf_recvmsg_parser().
             */
            sent = xsend(c, packet.as_ptr() as *const c_void, pkt_size as usize, 0);
            if !ASSERT_EQ(sent, pkt_size, cstr!("xsend(pre-data)")) {
                break 'out_close;
            }

            /* sk_data_ready of 'p' will be replaced by strparser handler */
            err = bpf_map_update_elem(map, &mut zero as *mut _ as *const c_void, &mut p as *mut _ as *const c_void, BPF_NOEXIST);
            if !ASSERT_OK(err, cstr!("bpf_map_update_elem(p)")) {
                break 'out_close;
            }

            /* consume previous data we injected */
            if sockmap_strp_consume_pre_data(p) != 0 {
                break 'out_close;
            }

            /* Previously, we encountered issues such as deadlocks and
             * sequence errors that resulted in the inability to read
             * continuously. Therefore, we perform multiple iterations
             * of testing here.
             */
            for _i in 0..test_cnt {
                sent = xsend(c, packet.as_ptr() as *const c_void, pkt_size as usize, 0);
                if !ASSERT_EQ(sent, pkt_size, cstr!("xsend(c)")) {
                    break 'out_close;
                }

                recvd = recv_timeout(p, rcv.as_mut_ptr() as *mut c_void, size_of_val(&rcv), MSG_DONTWAIT, IO_TIMEOUT_SEC);
                if !ASSERT_EQ(recvd, pkt_size, cstr!("recv_timeout(p)"))
                    || !ASSERT_OK(
                        memcmp(packet.as_ptr() as *const c_void, rcv.as_ptr() as *const c_void, pkt_size as usize),
                        cstr!("memcmp, data mismatch"),
                    )
                {
                    break 'out_close;
                }
            }

            if fionread {
                sent = xsend(c, packet.as_ptr() as *const c_void, pkt_size as usize, 0);
                if !ASSERT_EQ(sent, pkt_size, cstr!("second xsend(c)")) {
                    break 'out_close;
                }

                err = ioctl(p, FIONREAD, &mut avail as *mut c_int);
                if !ASSERT_OK(err, cstr!("ioctl(FIONREAD) error"))
                    || !ASSERT_EQ(avail, pkt_size, cstr!("ioctl(FIONREAD)"))
                {
                    break 'out_close;
                }

                recvd = recv_timeout(p, rcv.as_mut_ptr() as *mut c_void, size_of_val(&rcv), MSG_DONTWAIT, IO_TIMEOUT_SEC);
                if !ASSERT_EQ(recvd, pkt_size, cstr!("second recv_timeout(p)"))
                    || !ASSERT_OK(
                        memcmp(packet.as_ptr() as *const c_void, rcv.as_ptr() as *const c_void, pkt_size as usize),
                        cstr!("second memcmp, data mismatch"),
                    )
                {
                    break 'out_close;
                }
            }
        }
        close(c);
        close(p);
    }
    test_sockmap_strp__destroy(strp);
}

/* Test strparser with verdict mode */
unsafe fn test_sockmap_strp_verdict(family: c_int, sotype: c_int) {
    let mut zero: c_int = 0;
    let mut one: c_int = 1;
    let mut sent: c_int;
    let mut recvd: c_int;
    let off: c_int;
    let mut err: c_int;
    let mut map: c_int = 0;
    let mut c0: c_int = -1;
    let mut p0: c_int = -1;
    let mut c1: c_int = -1;
    let mut p1: c_int = -1;
    let mut strp: *mut test_sockmap_strp = ptr::null_mut();
    let mut rcv: [c_char; (STRP_PKT_FULL_LEN + 1) as usize] = [0; (STRP_PKT_FULL_LEN + 1) as usize];
    rcv[0] = b'0' as c_char;

    strp = sockmap_strp_init(&mut map, false, true);
    if !ASSERT_TRUE(strp as *const c_void, cstr!("sockmap_strp_init")) {
        return;
    }

    'out: {
        /* We simulate a reverse proxy server.
         * When p0 receives data from c0, we forward it to c1.
         * From c1's perspective, it will consider this data
         * as being sent by p1.
         */
        err = create_socket_pairs(family, sotype, &mut c0, &mut c1, &mut p0, &mut p1);
        if !ASSERT_OK(err, cstr!("create_socket_pairs()")) {
            break 'out;
        }

        'out_close: {
            err = bpf_map_update_elem(map, &mut zero as *mut _ as *const c_void, &mut p0 as *mut _ as *const c_void, BPF_NOEXIST);
            if !ASSERT_OK(err, cstr!("bpf_map_update_elem(p0)")) {
                break 'out_close;
            }

            err = bpf_map_update_elem(map, &mut one as *mut _ as *const c_void, &mut p1 as *mut _ as *const c_void, BPF_NOEXIST);
            if !ASSERT_OK(err, cstr!("bpf_map_update_elem(p1)")) {
                break 'out_close;
            }

            sent = xsend(c0, packet.as_ptr() as *const c_void, STRP_PKT_FULL_LEN as usize, 0);
            if !ASSERT_EQ(sent, STRP_PKT_FULL_LEN, cstr!("xsend(c0)")) {
                break 'out_close;
            }

            recvd = recv_timeout(c1, rcv.as_mut_ptr() as *mut c_void, size_of_val(&rcv), MSG_DONTWAIT, IO_TIMEOUT_SEC);
            if !ASSERT_EQ(recvd, STRP_PKT_FULL_LEN, cstr!("recv_timeout(c1)"))
                || !ASSERT_OK(
                    memcmp(packet.as_ptr() as *const c_void, rcv.as_ptr() as *const c_void, STRP_PKT_FULL_LEN as usize),
                    cstr!("received data does not match the sent data"),
                )
            {
                break 'out_close;
            }

            /* send again to ensure the stream is functioning correctly. */
            sent = xsend(c0, packet.as_ptr() as *const c_void, STRP_PKT_FULL_LEN as usize, 0);
            if !ASSERT_EQ(sent, STRP_PKT_FULL_LEN, cstr!("second xsend(c0)")) {
                break 'out_close;
            }

            /* partial read */
            off = STRP_PKT_FULL_LEN / 2;
            recvd = recv_timeout(c1, rcv.as_mut_ptr() as *mut c_void, off as usize, MSG_DONTWAIT, IO_TIMEOUT_SEC);
            recvd += recv_timeout(
                c1,
                rcv.as_mut_ptr().add(off as usize) as *mut c_void,
                size_of_val(&rcv) - off as usize,
                MSG_DONTWAIT,
                IO_TIMEOUT_SEC,
            );

            if !ASSERT_EQ(recvd, STRP_PKT_FULL_LEN, cstr!("partial recv_timeout(c1)"))
                || !ASSERT_OK(
                    memcmp(packet.as_ptr() as *const c_void, rcv.as_ptr() as *const c_void, STRP_PKT_FULL_LEN as usize),
                    cstr!("partial received data does not match the sent data"),
                )
            {
                break 'out_close;
            }
        }
        close(c0);
        close(c1);
        close(p0);
        close(p1);
    }
    test_sockmap_strp__destroy(strp);
}

unsafe fn test_sockmap_strp_parser_reject() {
    let mut strp: *mut test_sockmap_strp = ptr::null_mut();
    let parser_mod: c_int;
    let parser_ro: c_int;
    let mut link: c_int = 0;
    let mut err: c_int;
    let map: c_int;

    strp = test_sockmap_strp__open_and_load();
    if !ASSERT_OK_PTR(strp as *const c_void, cstr!("test_sockmap_strp__open_and_load")) {
        return;
    }

    map = bpf_map__fd((*strp).maps.sock_map);
    parser_mod = bpf_program__fd((*strp).progs.prog_skb_parser_resize);
    parser_ro = bpf_program__fd((*strp).progs.prog_skb_parser);

    err = bpf_prog_attach(parser_mod, map, BPF_SK_SKB_STREAM_PARSER, 0);
    ASSERT_ERR(err, cstr!("bpf_prog_attach parser_mod"));

    link = bpf_link_create(parser_ro, map, BPF_SK_SKB_STREAM_PARSER, ptr::null());
    if !ASSERT_GE(link, 0, cstr!("bpf_link_create parser_ro")) {
        if link >= 0 {
            close(link);
        }
        test_sockmap_strp__destroy(strp);
        return;
    }

    err = bpf_link_update(link, parser_mod, ptr::null());
    ASSERT_ERR(err, cstr!("bpf_link_update parser_mod"));

    if link >= 0 {
        close(link);
    }
    test_sockmap_strp__destroy(strp);
}

#[no_mangle]
pub unsafe extern "C" fn test_sockmap_strp() {
    if test__start_subtest(cstr!("sockmap strp tcp pass")) {
        test_sockmap_strp_pass(AF_INET, SOCK_STREAM, false);
    }
    if test__start_subtest(cstr!("sockmap strp tcp v6 pass")) {
        test_sockmap_strp_pass(AF_INET6, SOCK_STREAM, false);
    }
    if test__start_subtest(cstr!("sockmap strp tcp pass fionread")) {
        test_sockmap_strp_pass(AF_INET, SOCK_STREAM, true);
    }
    if test__start_subtest(cstr!("sockmap strp tcp v6 pass fionread")) {
        test_sockmap_strp_pass(AF_INET6, SOCK_STREAM, true);
    }
    if test__start_subtest(cstr!("sockmap strp tcp verdict")) {
        test_sockmap_strp_verdict(AF_INET, SOCK_STREAM);
    }
    if test__start_subtest(cstr!("sockmap strp tcp v6 verdict")) {
        test_sockmap_strp_verdict(AF_INET6, SOCK_STREAM);
    }
    if test__start_subtest(cstr!("sockmap strp tcp partial read")) {
        test_sockmap_strp_partial_read(AF_INET, SOCK_STREAM);
    }
    if test__start_subtest(cstr!("sockmap strp tcp multiple packets")) {
        test_sockmap_strp_multiple_pkt(AF_INET, SOCK_STREAM);
    }
    if test__start_subtest(cstr!("sockmap strp tcp dispatch")) {
        test_sockmap_strp_dispatch_pkt(AF_INET, SOCK_STREAM);
    }
    if test__start_subtest(cstr!("sockmap strp parser reject pkt mod")) {
        test_sockmap_strp_parser_reject();
    }
}
