// SPDX-License-Identifier: GPL-2.0
/* Check that after SEQ number wrap-around:
 * 1. SEQ-extension has upper bytes set
 * 2. TCP connection is alive and no TCPAOBad segments
 * In order to test (2), the test doesn't just adjust seq number for a queue
 * on a connected socket, but migrates it to another sk+port number, so
 * that there won't be any delayed packets that will fail to verify
 * with the new SEQ numbers.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type ssize_t = isize;
type socklen_t = u32;

const NULL: *mut c_void = core::ptr::null_mut();

pub const nr_packets: c_uint = 1000;
pub const msg_len: c_uint = 1000;
pub const quota: c_uint = nr_packets * msg_len;
pub static mut client_new_port: c_uint = 0;

#[repr(C)]
pub struct tcp_sock_state_seq {
    pub seq: u32,
}

#[repr(C)]
pub struct tcp_sock_state_trw {
    pub snd_wl1: u32,
    pub rcv_wup: u32,
}

#[repr(C)]
pub struct tcp_sock_state {
    pub in_: tcp_sock_state_seq,
    pub out: tcp_sock_state_seq,
    pub trw: tcp_sock_state_trw,
}

#[repr(C)]
pub struct tcp_ao_repair {
    pub snd_sne: u32,
    pub rcv_sne: u32,
}

#[repr(C)]
pub struct tcp_counters {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
}

#[repr(C)]
pub union sockaddr_af {
    pub sin: sockaddr_in,
    pub sin6: sockaddr_in6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tcp_addr {
    _private: [u8; 0],
}

unsafe extern "C" {
    static test_family: c_int;
    static test_server_port: c_uint;
    static this_ip_addr: tcp_addr;
    static this_ip_dest: tcp_addr;

    static DEFAULT_TEST_PASSWORD: *const c_char;
    static TEST_TIMEOUT_SEC: c_uint;
    static TEST_CNT_GOOD: c_uint;
    static TCP_AO_SND_SNE_UPDATE: c_int;
    static TCP_AO_RCV_SNE_UPDATE: c_int;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut c_void, addrlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;

    fn test_error(fmt: *const c_char, ...);
    fn test_fail(fmt: *const c_char, ...);
    fn test_ok(fmt: *const c_char, ...);
    fn test_init(argc: c_int, server_fn: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>, client_fn: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>);

    fn test_enable_repair(sk: c_int);
    fn test_disable_repair(sk: c_int);
    fn test_sock_restore(sk: c_int, img: *mut tcp_sock_state, saddr: *mut sockaddr_af, daddr: tcp_addr, dport: c_uint);
    fn test_add_repaired_key(sk: c_int, password: *const c_char, prefix: c_int, addr: tcp_addr, ifindex: c_int, sndid: c_uint, rcvid: c_uint) -> c_int;
    fn test_ao_restore(sk: c_int, ao_img: *mut tcp_ao_repair);
    fn test_get_tcp_counters(sk: c_int, cnt: *mut tcp_counters) -> c_int;
    fn test_sock_state_free(img: *mut tcp_sock_state);

    fn test_listen_socket(addr: tcp_addr, port: c_uint, backlog: c_int) -> c_int;
    fn test_add_key(sk: c_int, password: *const c_char, addr: tcp_addr, ifindex: c_int, sndid: c_uint, rcvid: c_uint) -> c_int;
    fn synchronize_threads();
    fn test_wait_fd(fd: c_int, timeout_sec: c_uint, events: c_int) -> c_int;
    fn test_server_run(sk: c_int, quota: c_uint, timeout_sec: c_uint) -> ssize_t;
    fn netstat_get_one(name: *const c_char, arg: *mut c_void) -> u64;
    fn test_sock_checkpoint(sk: c_int, img: *mut tcp_sock_state, saddr: *mut sockaddr_af);
    fn test_ao_checkpoint(sk: c_int, ao_img: *mut tcp_ao_repair);
    fn test_kill_sk(sk: c_int);
    fn trace_ao_event_sne_expect(event: c_int, src: tcp_addr, dst: tcp_addr, sport: c_uint, dport: c_uint, sne: c_uint);
    fn test_assert_counters(arg: *mut c_void, before: *mut tcp_counters, after: *mut tcp_counters, mask: c_uint);
    fn test_connect_socket(sk: c_int, addr: tcp_addr, port: c_uint) -> c_int;
    fn test_client_verify(sk: c_int, msg_len: c_uint, nr_packets: c_uint) -> c_int;
}

const SOCK_STREAM: c_int = 1;
const IPPROTO_TCP: c_int = 6;

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

/* Move them closer to roll-over */
unsafe fn test_adjust_seqs(img: *mut tcp_sock_state, ao_img: *mut tcp_ao_repair, server: bool) {
    let new_seq1: u32;
    let new_seq2: u32;

    let _ = ao_img;

    /* make them roll-over during quota, but on different segments */
    if server {
        new_seq1 = (u32::MAX).wrapping_sub(msg_len);
        new_seq2 = (u32::MAX).wrapping_sub(quota.wrapping_sub(2u32.wrapping_mul(msg_len)));
    } else {
        new_seq1 = (u32::MAX).wrapping_sub(quota.wrapping_sub(2u32.wrapping_mul(msg_len)));
        new_seq2 = (u32::MAX).wrapping_sub(msg_len);
    }

    (*img).in_.seq = new_seq1;
    (*img).trw.snd_wl1 = (*img).in_.seq.wrapping_sub(msg_len);
    (*img).out.seq = new_seq2;
    (*img).trw.rcv_wup = (*img).in_.seq;
}

unsafe fn test_sk_restore(
    img: *mut tcp_sock_state,
    ao_img: *mut tcp_ao_repair,
    saddr: *mut sockaddr_af,
    daddr: tcp_addr,
    dport: c_uint,
    cnt: *mut tcp_counters,
) -> c_int {
    let sk: c_int;

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error(cstr(b"socket()\0"));
    }

    test_enable_repair(sk);
    test_sock_restore(sk, img, saddr, daddr, dport);
    if test_add_repaired_key(sk, DEFAULT_TEST_PASSWORD, 0, daddr, -1, 100, 100) != 0 {
        test_error(cstr(b"setsockopt(TCP_AO_ADD_KEY)\0"));
    }
    test_ao_restore(sk, ao_img);

    if test_get_tcp_counters(sk, cnt) != 0 {
        test_error(cstr(b"test_get_tcp_counters()\0"));
    }

    test_disable_repair(sk);
    test_sock_state_free(img);
    sk
}

unsafe extern "C" fn server_fn(arg: *mut c_void) -> *mut c_void {
    let before_good: u64;
    let after_good: u64;
    let after_bad: u64;
    let mut cnt1: tcp_counters = core::mem::zeroed();
    let mut cnt2: tcp_counters = core::mem::zeroed();
    let mut img: tcp_sock_state = core::mem::zeroed();
    let mut ao_img: tcp_ao_repair = core::mem::zeroed();
    let mut saddr: sockaddr_af = core::mem::zeroed();
    let mut bytes: ssize_t;
    let mut sk: c_int;
    let lsk: c_int;

    let _ = arg;

    lsk = test_listen_socket(this_ip_addr, test_server_port, 1);

    if test_add_key(lsk, DEFAULT_TEST_PASSWORD, this_ip_dest, -1, 100, 100) != 0 {
        test_error(cstr(b"setsockopt(TCP_AO_ADD_KEY)\0"));
    }

    synchronize_threads(); /* 1: MKT added => connect() */

    if test_wait_fd(lsk, TEST_TIMEOUT_SEC, 0) != 0 {
        test_error(cstr(b"test_wait_fd()\0"));
    }

    sk = accept(lsk, NULL, core::ptr::null_mut());
    if sk < 0 {
        test_error(cstr(b"accept()\0"));
    }

    synchronize_threads(); /* 2: accepted => send data */
    close(lsk);

    bytes = test_server_run(sk, quota, TEST_TIMEOUT_SEC);
    if bytes != quota as ssize_t {
        if bytes > 0 {
            test_fail(cstr(b"server served: %zd\0"), bytes);
        } else {
            test_fail(cstr(b"server returned: %zd\0"), bytes);
        }
        close(sk);
        return core::ptr::null_mut();
    }

    before_good = netstat_get_one(cstr(b"TCPAOGood\0"), NULL);

    synchronize_threads(); /* 3: restore the connection on another port */

    test_enable_repair(sk);
    test_sock_checkpoint(sk, &mut img, &mut saddr);
    test_ao_checkpoint(sk, &mut ao_img);
    test_kill_sk(sk);
    /* #ifdef IPV6_TEST */
    #[cfg(IPV6_TEST)]
    {
        saddr.sin6.sin6_port = htons(ntohs(saddr.sin6.sin6_port).wrapping_add(1));
    }
    /* #else */
    #[cfg(not(IPV6_TEST))]
    {
        saddr.sin.sin_port = htons(ntohs(saddr.sin.sin_port).wrapping_add(1));
    }
    /* #endif */
    test_adjust_seqs(&mut img, &mut ao_img, true);
    synchronize_threads(); /* 4: dump finished */
    sk = test_sk_restore(
        &mut img,
        &mut ao_img,
        &mut saddr,
        this_ip_dest,
        client_new_port,
        &mut cnt1,
    );

    trace_ao_event_sne_expect(TCP_AO_SND_SNE_UPDATE, this_ip_addr, this_ip_dest, test_server_port + 1, client_new_port, 1);
    trace_ao_event_sne_expect(TCP_AO_SND_SNE_UPDATE, this_ip_dest, this_ip_addr, client_new_port, test_server_port + 1, 1);
    trace_ao_event_sne_expect(TCP_AO_RCV_SNE_UPDATE, this_ip_addr, this_ip_dest, test_server_port + 1, client_new_port, 1);
    trace_ao_event_sne_expect(TCP_AO_RCV_SNE_UPDATE, this_ip_dest, this_ip_addr, client_new_port, test_server_port + 1, 1);
    synchronize_threads(); /* 5: verify the connection during SEQ-number rollover */
    bytes = test_server_run(sk, quota, TEST_TIMEOUT_SEC);
    if bytes != quota as ssize_t {
        if bytes > 0 {
            test_fail(cstr(b"server served: %zd\0"), bytes);
        } else {
            test_fail(cstr(b"server returned: %zd\0"), bytes);
        }
    } else {
        test_ok(cstr(b"server alive\0"));
    }

    synchronize_threads(); /* 6: verify counters after SEQ-number rollover */
    if test_get_tcp_counters(sk, &mut cnt2) != 0 {
        test_error(cstr(b"test_get_tcp_counters()\0"));
    }
    after_good = netstat_get_one(cstr(b"TCPAOGood\0"), NULL);

    test_assert_counters(NULL, &mut cnt1, &mut cnt2, TEST_CNT_GOOD);

    if after_good <= before_good {
        test_fail(
            cstr(b"TCPAOGood counter did not increase: %llu <= %llu\0"),
            after_good,
            before_good,
        );
    } else {
        test_ok(
            cstr(b"TCPAOGood counter increased %llu => %llu\0"),
            before_good,
            after_good,
        );
    }
    after_bad = netstat_get_one(cstr(b"TCPAOBad\0"), NULL);
    if after_bad != 0 {
        test_fail(cstr(b"TCPAOBad counter is non-zero: %llu\0"), after_bad);
    } else {
        test_ok(cstr(b"TCPAOBad counter didn't increase\0"));
    }
    test_enable_repair(sk);
    test_ao_checkpoint(sk, &mut ao_img);
    if ao_img.snd_sne != 0 && ao_img.rcv_sne != 0 {
        test_ok(
            cstr(b"SEQ extension incremented: %u/%u\0"),
            ao_img.snd_sne,
            ao_img.rcv_sne,
        );
    } else {
        test_fail(
            cstr(b"SEQ extension was not incremented: %u/%u\0"),
            ao_img.snd_sne,
            ao_img.rcv_sne,
        );
    }

    synchronize_threads(); /* 6: verified => closed */
    close(sk);
    core::ptr::null_mut()
}

unsafe extern "C" fn client_fn(arg: *mut c_void) -> *mut c_void {
    let before_good: u64;
    let after_good: u64;
    let after_bad: u64;
    let mut cnt1: tcp_counters = core::mem::zeroed();
    let mut cnt2: tcp_counters = core::mem::zeroed();
    let mut img: tcp_sock_state = core::mem::zeroed();
    let mut ao_img: tcp_ao_repair = core::mem::zeroed();
    let mut saddr: sockaddr_af = core::mem::zeroed();
    let mut sk: c_int;

    let _ = arg;

    sk = socket(test_family, SOCK_STREAM, IPPROTO_TCP);
    if sk < 0 {
        test_error(cstr(b"socket()\0"));
    }

    if test_add_key(sk, DEFAULT_TEST_PASSWORD, this_ip_dest, -1, 100, 100) != 0 {
        test_error(cstr(b"setsockopt(TCP_AO_ADD_KEY)\0"));
    }

    synchronize_threads(); /* 1: MKT added => connect() */
    if test_connect_socket(sk, this_ip_dest, test_server_port) <= 0 {
        test_error(cstr(b"failed to connect()\0"));
    }

    synchronize_threads(); /* 2: accepted => send data */
    if test_client_verify(sk, msg_len, nr_packets) != 0 {
        test_fail(cstr(b"pre-migrate verify failed\0"));
        return core::ptr::null_mut();
    }

    before_good = netstat_get_one(cstr(b"TCPAOGood\0"), NULL);

    synchronize_threads(); /* 3: restore the connection on another port */
    test_enable_repair(sk);
    test_sock_checkpoint(sk, &mut img, &mut saddr);
    test_ao_checkpoint(sk, &mut ao_img);
    test_kill_sk(sk);
    /* #ifdef IPV6_TEST */
    #[cfg(IPV6_TEST)]
    {
        client_new_port = ntohs(saddr.sin6.sin6_port).wrapping_add(1) as c_uint;
        saddr.sin6.sin6_port = htons(ntohs(saddr.sin6.sin6_port).wrapping_add(1));
    }
    /* #else */
    #[cfg(not(IPV6_TEST))]
    {
        client_new_port = ntohs(saddr.sin.sin_port).wrapping_add(1) as c_uint;
        saddr.sin.sin_port = htons(ntohs(saddr.sin.sin_port).wrapping_add(1));
    }
    /* #endif */
    test_adjust_seqs(&mut img, &mut ao_img, false);
    synchronize_threads(); /* 4: dump finished */
    sk = test_sk_restore(
        &mut img,
        &mut ao_img,
        &mut saddr,
        this_ip_dest,
        test_server_port + 1,
        &mut cnt1,
    );

    synchronize_threads(); /* 5: verify the connection during SEQ-number rollover */
    if test_client_verify(sk, msg_len, nr_packets) != 0 {
        test_fail(cstr(b"post-migrate verify failed\0"));
    } else {
        test_ok(cstr(b"post-migrate connection alive\0"));
    }

    synchronize_threads(); /* 5: verify counters after SEQ-number rollover */
    if test_get_tcp_counters(sk, &mut cnt2) != 0 {
        test_error(cstr(b"test_get_tcp_counters()\0"));
    }
    after_good = netstat_get_one(cstr(b"TCPAOGood\0"), NULL);

    test_assert_counters(NULL, &mut cnt1, &mut cnt2, TEST_CNT_GOOD);

    if after_good <= before_good {
        test_fail(
            cstr(b"TCPAOGood counter did not increase: %llu <= %llu\0"),
            after_good,
            before_good,
        );
    } else {
        test_ok(
            cstr(b"TCPAOGood counter increased %llu => %llu\0"),
            before_good,
            after_good,
        );
    }
    after_bad = netstat_get_one(cstr(b"TCPAOBad\0"), NULL);
    if after_bad != 0 {
        test_fail(cstr(b"TCPAOBad counter is non-zero: %llu\0"), after_bad);
    } else {
        test_ok(cstr(b"TCPAOBad counter didn't increase\0"));
    }

    synchronize_threads(); /* 6: verified => closed */
    close(sk);

    synchronize_threads(); /* don't race to exit: let server exit() */
    core::ptr::null_mut()
}

fn main() {
    unsafe {
        test_init(8, Some(server_fn), Some(client_fn));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
