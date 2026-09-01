// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// C dependencies: vmlinux.h, bpf_tracing_net.h, bpf/bpf_helpers.h,
// and bpf/bpf_endian.h.

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe fn hlist_unhashed_lockless(h: *const hlist_node) -> i32 {
    ((*h).pprev.is_null()) as i32
}

unsafe fn timer_pending(timer: *const timer_list) -> i32 {
    (!hlist_unhashed_lockless(&(*timer).entry as *const hlist_node) != 0) as i32
}

extern "C" {
    #[link_name = "CONFIG_HZ"]
    static CONFIG_HZ: u32;
}

const USER_HZ: u64 = 100;
const NSEC_PER_SEC: u64 = 1000000000;

unsafe fn jiffies_to_clock_t(x: c_ulong) -> clock_t {
    /*
     * The implementation here tailored to a particular
     * setting of USER_HZ.
     */
    let tick_nsec: u64 = (NSEC_PER_SEC + CONFIG_HZ as u64 / 2) / CONFIG_HZ as u64;
    let user_hz_nsec: u64 = NSEC_PER_SEC / USER_HZ;

    if tick_nsec % user_hz_nsec == 0 {
        if CONFIG_HZ as u64 < USER_HZ {
            return x.wrapping_mul((USER_HZ / CONFIG_HZ as u64) as c_ulong) as clock_t;
        } else {
            return x.wrapping_div((CONFIG_HZ as u64 / USER_HZ) as c_ulong) as clock_t;
        }
    }
    x.wrapping_mul(tick_nsec as c_ulong)
        .wrapping_div(user_hz_nsec as c_ulong) as clock_t
}

unsafe fn jiffies_delta_to_clock_t(delta: c_long) -> clock_t {
    if delta <= 0 {
        return 0;
    }

    jiffies_to_clock_t(delta as c_ulong)
}

unsafe fn sock_i_ino(sk: *const sock) -> c_long {
    let sk_socket: *const socket = (*sk).sk_socket;
    let inode: *const inode;
    let mut ino: c_ulong;

    if sk_socket.is_null() {
        return 0;
    }

    inode = &(*(sk_socket as *const socket_alloc)).vfs_inode as *const inode;
    bpf_probe_read_kernel(
        &mut ino as *mut c_ulong as *mut c_void,
        core::mem::size_of_val(&ino) as u32,
        &(*inode).i_ino as *const _ as *const c_void,
    );
    ino as c_long
}

unsafe fn inet_csk_in_pingpong_mode(icsk: *const inet_connection_sock) -> bool {
    (*icsk).icsk_ack.pingpong >= TCP_PINGPONG_THRESH
}

unsafe fn tcp_in_initial_slowstart(tcp: *const tcp_sock) -> bool {
    (*tcp).snd_ssthresh >= TCP_INFINITE_SSTHRESH
}

unsafe fn dump_tcp_sock(
    seq: *mut seq_file,
    tp: *mut tcp_sock,
    uid: uid_t,
    seq_num: __u32,
) -> i32 {
    let icsk: *const inet_connection_sock;
    let fastopenq: *const fastopen_queue;
    let inet: *const inet_sock;
    let timer_expires: c_ulong;
    let sp: *const sock;
    let destp: __u16;
    let srcp: __u16;
    let dest: __be32;
    let src: __be32;
    let timer_active: i32;
    let mut rx_queue: i32;
    let state: i32;

    icsk = &(*tp).inet_conn as *const inet_connection_sock;
    inet = &(*icsk).icsk_inet as *const inet_sock;
    sp = &(*inet).sk as *const sock;
    fastopenq = &(*icsk).icsk_accept_queue.fastopenq as *const fastopen_queue;

    dest = (*inet).inet_daddr;
    src = (*inet).inet_rcv_saddr;
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
    } else if timer_pending(&(*icsk).icsk_keepalive_timer as *const timer_list) != 0 {
        timer_active = 2;
        timer_expires = (*icsk).icsk_keepalive_timer.expires;
    } else {
        timer_active = 0;
        timer_expires = bpf_jiffies64() as c_ulong;
    }

    state = (*sp).sk_state as i32;
    if state == TCP_LISTEN {
        rx_queue = (*sp).sk_ack_backlog as i32;
    } else {
        rx_queue = (*tp).rcv_nxt.wrapping_sub((*tp).copied_seq) as i32;
        if rx_queue < 0 {
            rx_queue = 0;
        }
    }

    BPF_SEQ_PRINTF!(
        seq,
        "%4d: %08X:%04X %08X:%04X ",
        seq_num,
        src,
        srcp,
        dest,
        destp
    );
    BPF_SEQ_PRINTF!(
        seq,
        "%02X %08X:%08X %02X:%08lX %08X %5u %8d %lu %d ",
        state,
        (*tp).write_seq.wrapping_sub((*tp).snd_una),
        rx_queue,
        timer_active,
        jiffies_delta_to_clock_t(timer_expires.wrapping_sub(bpf_jiffies64() as c_ulong) as c_long),
        (*icsk).icsk_retransmits,
        uid,
        (*icsk).icsk_probes_out,
        sock_i_ino(sp),
        (*sp).sk_refcnt.refs.counter
    );
    BPF_SEQ_PRINTF!(
        seq,
        "%pK %lu %lu %u %u %d\n",
        tp,
        jiffies_to_clock_t((*icsk).icsk_rto as c_ulong),
        jiffies_to_clock_t((*icsk).icsk_ack.ato as c_ulong),
        ((*icsk).icsk_ack.quick << 1) | inet_csk_in_pingpong_mode(icsk) as u32,
        (*tp).snd_cwnd,
        if state == TCP_LISTEN {
            (*fastopenq).max_qlen as i32
        } else if tcp_in_initial_slowstart(tp) {
            -1
        } else {
            (*tp).snd_ssthresh as i32
        }
    );

    0
}

unsafe fn dump_tw_sock(
    seq: *mut seq_file,
    ttw: *mut tcp_timewait_sock,
    uid: uid_t,
    seq_num: __u32,
) -> i32 {
    let tw: *mut inet_timewait_sock = &mut (*ttw).tw_sk as *mut inet_timewait_sock;
    let destp: __u16;
    let srcp: __u16;
    let dest: __be32;
    let src: __be32;
    let delta: c_long;

    delta = (*tw).tw_timer.expires.wrapping_sub(bpf_jiffies64() as c_ulong) as c_long;
    dest = (*tw).tw_daddr;
    src = (*tw).tw_rcv_saddr;
    destp = bpf_ntohs((*tw).tw_dport);
    srcp = bpf_ntohs((*tw).tw_sport);

    BPF_SEQ_PRINTF!(
        seq,
        "%4d: %08X:%04X %08X:%04X ",
        seq_num,
        src,
        srcp,
        dest,
        destp
    );

    BPF_SEQ_PRINTF!(
        seq,
        "%02X %08X:%08X %02X:%08lX %08X %5d %8d %d %d %pK\n",
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
        tw
    );

    0
}

unsafe fn dump_req_sock(
    seq: *mut seq_file,
    treq: *mut tcp_request_sock,
    uid: uid_t,
    seq_num: __u32,
) -> i32 {
    let irsk: *mut inet_request_sock = &mut (*treq).req as *mut inet_request_sock;
    let req: *mut request_sock = &mut (*irsk).req as *mut request_sock;
    let mut ttd: c_long;

    ttd = (*req).rsk_timer.expires.wrapping_sub(bpf_jiffies64() as c_ulong) as c_long;

    if ttd < 0 {
        ttd = 0;
    }

    BPF_SEQ_PRINTF!(
        seq,
        "%4d: %08X:%04X %08X:%04X ",
        seq_num,
        (*irsk).ir_loc_addr,
        (*irsk).ir_num,
        (*irsk).ir_rmt_addr,
        bpf_ntohs((*irsk).ir_rmt_port)
    );
    BPF_SEQ_PRINTF!(
        seq,
        "%02X %08X:%08X %02X:%08lX %08X %5d %8d %d %d %pK\n",
        TCP_SYN_RECV,
        0,
        0,
        1,
        jiffies_to_clock_t(ttd as c_ulong),
        (*req).num_timeout,
        uid,
        0,
        0,
        0,
        req
    );

    0
}

#[no_mangle]
#[link_section = "iter/tcp"]
pub unsafe extern "C" fn dump_tcp4(ctx: *mut bpf_iter__tcp) -> i32 {
    let sk_common: *mut sock_common = (*ctx).sk_common;
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let mut tw: *mut tcp_timewait_sock;
    let mut req: *mut tcp_request_sock;
    let mut tp: *mut tcp_sock;
    let uid: uid_t = (*ctx).uid;
    let seq_num: __u32;

    if sk_common == core::ptr::null_mut() {
        return 0;
    }

    seq_num = (*(*ctx).meta).seq_num;
    if seq_num == 0 {
        BPF_SEQ_PRINTF!(
            seq,
            "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode\n"
        );
    }

    if (*sk_common).skc_family as i32 != AF_INET {
        return 0;
    }

    tp = bpf_skc_to_tcp_sock(sk_common);
    if !tp.is_null() {
        return dump_tcp_sock(seq, tp, uid, seq_num);
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

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
