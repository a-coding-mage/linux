// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// Rust translation of testing/selftests/bpf/progs/bpf_iter_tcp6.c.
// C dependencies: vmlinux.h, bpf_tracing_net.h, bpf/bpf_helpers.h,
// and bpf/bpf_endian.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type clock_t = i64;
type uid_t = u32;
type __u16 = u16;
type __u32 = u32;
type u64 = u64;

const USER_HZ: u64 = 100;
const NSEC_PER_SEC: u64 = 1000000000u64;

extern "C" {
    static CONFIG_HZ: u32;

    fn bpf_probe_read_kernel(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
    fn bpf_jiffies64() -> u64;
    fn bpf_ntohs(val: __u16) -> __u16;
    fn bpf_skc_to_tcp6_sock(sk: *mut sock_common) -> *mut tcp6_sock;
    fn bpf_skc_to_tcp_timewait_sock(sk: *mut sock_common) -> *mut tcp_timewait_sock;
    fn bpf_skc_to_tcp_request_sock(sk: *mut sock_common) -> *mut tcp_request_sock;
    fn BPF_SEQ_PRINTF(seq: *mut seq_file, fmt: *const u8, ...);
}

extern "C" {
    static TCP_PINGPONG_THRESH: u8;
    static TCP_INFINITE_SSTHRESH: u32;
    static ICSK_TIME_RETRANS: u8;
    static ICSK_TIME_REO_TIMEOUT: u8;
    static ICSK_TIME_LOSS_PROBE: u8;
    static ICSK_TIME_PROBE0: u8;
    static TCP_LISTEN: i32;
    static TCP_SYN_RECV: i32;
    static AF_INET6: u16;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
struct hlist_node {
    next: *mut hlist_node,
    pprev: *mut *mut hlist_node,
}

#[repr(C)]
struct timer_list {
    entry: hlist_node,
    expires: u64,
}

#[repr(C)]
struct refcount_struct {
    counter: i32,
}

#[repr(C)]
struct sock_refcnt {
    refs: refcount_struct,
}

#[repr(C)]
struct in6_addr {
    s6_addr32: [u32; 4],
}

#[repr(C)]
struct sock_common {
    skc_family: u16,
}

#[repr(C)]
struct sock {
    sk_common: sock_common,
    sk_socket: *const socket,
    sk_v6_daddr: in6_addr,
    sk_v6_rcv_saddr: in6_addr,
    tcp_retransmit_timer: timer_list,
    sk_state: i32,
    sk_ack_backlog: i32,
    sk_refcnt: sock_refcnt,
}

#[repr(C)]
struct socket {
    _private: [u8; 0],
}

#[repr(C)]
struct socket_alloc {
    socket: socket,
    vfs_inode: inode,
}

#[repr(C)]
struct inode {
    i_ino: u64,
}

#[repr(C)]
struct inet_sock {
    sk: sock,
    inet_dport: __u16,
    inet_sport: __u16,
}

#[repr(C)]
struct ack_block {
    pingpong: u8,
    ato: u64,
    quick: u32,
}

#[repr(C)]
struct fastopen_queue {
    max_qlen: i32,
}

#[repr(C)]
struct request_sock_queue {
    fastopenq: fastopen_queue,
}

#[repr(C)]
struct inet_connection_sock {
    icsk_inet: inet_sock,
    icsk_accept_queue: request_sock_queue,
    icsk_pending: u8,
    icsk_keepalive_timer: timer_list,
    icsk_retransmits: u32,
    icsk_probes_out: i32,
    icsk_rto: u64,
    icsk_ack: ack_block,
}

#[repr(C)]
struct tcp_sock {
    inet_conn: inet_connection_sock,
    rcv_nxt: i32,
    copied_seq: i32,
    write_seq: u32,
    snd_una: u32,
    snd_cwnd: u32,
    snd_ssthresh: u32,
}

#[repr(C)]
struct tcp6_sock {
    tcp: tcp_sock,
}

#[repr(C)]
struct inet_timewait_sock {
    tw_timer: timer_list,
    tw_v6_daddr: in6_addr,
    tw_v6_rcv_saddr: in6_addr,
    tw_dport: __u16,
    tw_sport: __u16,
    tw_substate: i32,
    tw_refcnt: sock_refcnt,
}

#[repr(C)]
struct tcp_timewait_sock {
    tw_sk: inet_timewait_sock,
}

#[repr(C)]
struct request_sock {
    rsk_timer: timer_list,
    num_timeout: u32,
}

#[repr(C)]
struct inet_request_sock {
    req: request_sock,
    ir_v6_loc_addr: in6_addr,
    ir_v6_rmt_addr: in6_addr,
    ir_num: __u16,
    ir_rmt_port: __u16,
}

#[repr(C)]
struct tcp_request_sock {
    req: inet_request_sock,
}

#[repr(C)]
struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_iter_meta {
    seq: *mut seq_file,
    seq_num: __u32,
}

#[repr(C)]
struct bpf_iter__tcp {
    meta: *mut bpf_iter_meta,
    sk_common: *mut sock_common,
    uid: uid_t,
}

unsafe fn hlist_unhashed_lockless(h: *const hlist_node) -> i32 {
    ((*h).pprev.is_null()) as i32
}

unsafe fn timer_pending(timer: *const timer_list) -> i32 {
    (hlist_unhashed_lockless(&(*timer).entry) == 0) as i32
}

unsafe fn jiffies_to_clock_t(x: u64) -> clock_t {
    /*
     * The implementation here tailored to a particular
     * setting of USER_HZ.
     */
    let tick_nsec: u64 = (NSEC_PER_SEC + CONFIG_HZ as u64 / 2) / CONFIG_HZ as u64;
    let user_hz_nsec: u64 = NSEC_PER_SEC / USER_HZ;

    if tick_nsec % user_hz_nsec == 0 {
        if CONFIG_HZ < USER_HZ as u32 {
            return (x * (USER_HZ / CONFIG_HZ as u64)) as clock_t;
        } else {
            return (x / (CONFIG_HZ as u64 / USER_HZ)) as clock_t;
        }
    }
    (x * tick_nsec / user_hz_nsec) as clock_t
}

unsafe fn jiffies_delta_to_clock_t(delta: i64) -> clock_t {
    if delta <= 0 {
        return 0;
    }

    jiffies_to_clock_t(delta as u64)
}

unsafe fn sock_i_ino(sk: *const sock) -> i64 {
    let sk_socket: *const socket = (*sk).sk_socket;
    let inode: *const inode;
    let mut ino: u64 = 0;

    if sk_socket.is_null() {
        return 0;
    }

    inode = &(*(sk_socket as *const socket_alloc)).vfs_inode;
    bpf_probe_read_kernel(
        &mut ino as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&ino) as u32,
        &(*inode).i_ino as *const _ as *const core::ffi::c_void,
    );
    ino as i64
}

unsafe fn inet_csk_in_pingpong_mode(icsk: *const inet_connection_sock) -> bool {
    (*icsk).icsk_ack.pingpong >= TCP_PINGPONG_THRESH
}

unsafe fn tcp_in_initial_slowstart(tcp: *const tcp_sock) -> bool {
    (*tcp).snd_ssthresh >= TCP_INFINITE_SSTHRESH
}

unsafe fn dump_tcp6_sock(seq: *mut seq_file, tp: *mut tcp6_sock, uid: uid_t, seq_num: __u32) -> i32 {
    let icsk: *const inet_connection_sock;
    let fastopenq: *const fastopen_queue;
    let dest: *const in6_addr;
    let src: *const in6_addr;
    let inet: *const inet_sock;
    let timer_expires: u64;
    let sp: *const sock;
    let destp: __u16;
    let srcp: __u16;
    let timer_active: i32;
    let mut rx_queue: i32;
    let state: i32;

    icsk = &(*tp).tcp.inet_conn;
    inet = &(*icsk).icsk_inet;
    sp = &(*inet).sk;
    fastopenq = &(*icsk).icsk_accept_queue.fastopenq;

    dest = &(*sp).sk_v6_daddr;
    src = &(*sp).sk_v6_rcv_saddr;
    destp = bpf_ntohs((*inet).inet_dport);
    srcp = bpf_ntohs((*inet).inet_sport);

    if (*icsk).icsk_pending == ICSK_TIME_RETRANS
        || (*icsk).icsk_pending == ICSK_TIME_REO_TIMEOUT
        || (*icsk).icsk_pending == ICSK_TIME_LOSS_PROBE
    {
        timer_active = 1;
        timer_expires = (*sp).tcp_retransmit_timer.expires;
    } else if (*icsk).icsk_pending == ICSK_TIME_PROBE0 {
        timer_active = 4;
        timer_expires = (*sp).tcp_retransmit_timer.expires;
    } else if timer_pending(&(*icsk).icsk_keepalive_timer) != 0 {
        timer_active = 2;
        timer_expires = (*icsk).icsk_keepalive_timer.expires;
    } else {
        timer_active = 0;
        timer_expires = bpf_jiffies64();
    }

    state = (*sp).sk_state;
    if state == TCP_LISTEN {
        rx_queue = (*sp).sk_ack_backlog;
    } else {
        rx_queue = (*tp).tcp.rcv_nxt - (*tp).tcp.copied_seq;
        if rx_queue < 0 {
            rx_queue = 0;
        }
    }

    BPF_SEQ_PRINTF(
        seq,
        b"%4d: %08X%08X%08X%08X:%04X %08X%08X%08X%08X:%04X \0".as_ptr(),
        seq_num,
        (*src).s6_addr32[0],
        (*src).s6_addr32[1],
        (*src).s6_addr32[2],
        (*src).s6_addr32[3],
        srcp,
        (*dest).s6_addr32[0],
        (*dest).s6_addr32[1],
        (*dest).s6_addr32[2],
        (*dest).s6_addr32[3],
        destp,
    );
    BPF_SEQ_PRINTF(
        seq,
        b"%02X %08X:%08X %02X:%08lX %08X %5u %8d %lu %d \0".as_ptr(),
        state,
        (*tp).tcp.write_seq.wrapping_sub((*tp).tcp.snd_una),
        rx_queue,
        timer_active,
        jiffies_delta_to_clock_t(timer_expires as i64 - bpf_jiffies64() as i64),
        (*icsk).icsk_retransmits,
        uid,
        (*icsk).icsk_probes_out,
        sock_i_ino(sp),
        (*sp).sk_refcnt.refs.counter,
    );
    BPF_SEQ_PRINTF(
        seq,
        b"%pK %lu %lu %u %u %d\n\0".as_ptr(),
        tp,
        jiffies_to_clock_t((*icsk).icsk_rto),
        jiffies_to_clock_t((*icsk).icsk_ack.ato),
        ((*icsk).icsk_ack.quick << 1) | inet_csk_in_pingpong_mode(icsk) as u32,
        (*tp).tcp.snd_cwnd,
        if state == TCP_LISTEN {
            (*fastopenq).max_qlen
        } else if tcp_in_initial_slowstart(&(*tp).tcp) {
            -1
        } else {
            (*tp).tcp.snd_ssthresh as i32
        },
    );

    0
}

unsafe fn dump_tw_sock(seq: *mut seq_file, ttw: *mut tcp_timewait_sock, uid: uid_t, seq_num: __u32) -> i32 {
    let tw: *mut inet_timewait_sock = &mut (*ttw).tw_sk;
    let dest: *const in6_addr;
    let src: *const in6_addr;
    let destp: __u16;
    let srcp: __u16;
    let delta: i64;

    delta = (*tw).tw_timer.expires as i64 - bpf_jiffies64() as i64;
    dest = &(*tw).tw_v6_daddr;
    src = &(*tw).tw_v6_rcv_saddr;
    destp = bpf_ntohs((*tw).tw_dport);
    srcp = bpf_ntohs((*tw).tw_sport);

    BPF_SEQ_PRINTF(
        seq,
        b"%4d: %08X%08X%08X%08X:%04X %08X%08X%08X%08X:%04X \0".as_ptr(),
        seq_num,
        (*src).s6_addr32[0],
        (*src).s6_addr32[1],
        (*src).s6_addr32[2],
        (*src).s6_addr32[3],
        srcp,
        (*dest).s6_addr32[0],
        (*dest).s6_addr32[1],
        (*dest).s6_addr32[2],
        (*dest).s6_addr32[3],
        destp,
    );

    BPF_SEQ_PRINTF(
        seq,
        b"%02X %08X:%08X %02X:%08lX %08X %5d %8d %d %d %pK\n\0".as_ptr(),
        (*tw).tw_substate,
        0,
        0,
        3,
        jiffies_delta_to_clock_t(delta),
        0,
        0,
        0,
        0,
        (*tw).tw_refcnt.refs.counter,
        tw,
    );

    0
}

unsafe fn dump_req_sock(seq: *mut seq_file, treq: *mut tcp_request_sock, uid: uid_t, seq_num: __u32) -> i32 {
    let irsk: *mut inet_request_sock = &mut (*treq).req;
    let req: *mut request_sock = &mut (*irsk).req;
    let src: *mut in6_addr;
    let dest: *mut in6_addr;
    let mut ttd: i64;

    ttd = (*req).rsk_timer.expires as i64 - bpf_jiffies64() as i64;
    src = &mut (*irsk).ir_v6_loc_addr;
    dest = &mut (*irsk).ir_v6_rmt_addr;

    if ttd < 0 {
        ttd = 0;
    }

    BPF_SEQ_PRINTF(
        seq,
        b"%4d: %08X%08X%08X%08X:%04X %08X%08X%08X%08X:%04X \0".as_ptr(),
        seq_num,
        (*src).s6_addr32[0],
        (*src).s6_addr32[1],
        (*src).s6_addr32[2],
        (*src).s6_addr32[3],
        (*irsk).ir_num,
        (*dest).s6_addr32[0],
        (*dest).s6_addr32[1],
        (*dest).s6_addr32[2],
        (*dest).s6_addr32[3],
        bpf_ntohs((*irsk).ir_rmt_port),
    );
    BPF_SEQ_PRINTF(
        seq,
        b"%02X %08X:%08X %02X:%08lX %08X %5d %8d %d %d %pK\n\0".as_ptr(),
        TCP_SYN_RECV,
        0,
        0,
        1,
        jiffies_to_clock_t(ttd as u64),
        (*req).num_timeout,
        uid,
        0,
        0,
        0,
        req,
    );

    0
}

#[link_section = "iter/tcp"]
#[no_mangle]
pub unsafe extern "C" fn dump_tcp6(ctx: *mut bpf_iter__tcp) -> i32 {
    let sk_common: *mut sock_common = (*ctx).sk_common;
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let tw: *mut tcp_timewait_sock;
    let req: *mut tcp_request_sock;
    let tp: *mut tcp6_sock;
    let uid: uid_t = (*ctx).uid;
    let seq_num: __u32;

    if sk_common == 0 as *mut sock_common {
        return 0;
    }

    seq_num = (*(*ctx).meta).seq_num;
    if seq_num == 0 {
        BPF_SEQ_PRINTF(
            seq,
            b"  sl  local_address                         remote_address                        st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode\n\0".as_ptr(),
        );
    }

    if (*sk_common).skc_family != AF_INET6 {
        return 0;
    }

    tp = bpf_skc_to_tcp6_sock(sk_common);
    if !tp.is_null() {
        return dump_tcp6_sock(seq, tp, uid, seq_num);
    }

    tw = bpf_skc_to_tcp_timewait_sock(sk_common);
    if !tw.is_null() {
        return dump_tw_sock(seq, tw, uid, seq_num);
    }

    req = bpf_skc_to_tcp_request_sock(sk_common);
    if !req.is_null() {
        return dump_req_sock(seq, req, uid, seq_num);
    }

    0
}
