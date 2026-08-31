/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */
/* Rust translation of bpf_tracing_net.h. */
/* C dependencies removed from executable Rust: <vmlinux.h>, <bpf/bpf_core_read.h>. */

pub const AF_INET: i32 = 2;
pub const AF_INET6: i32 = 10;

/* include/linux/net.h */
pub const SOCK_TYPE_MASK: i32 = 0xf;

pub const SOL_SOCKET: i32 = 1;
pub const SO_REUSEADDR: i32 = 2;
pub const SO_SNDBUF: i32 = 7;
pub const SO_RCVBUF: i32 = 8;
pub const SO_KEEPALIVE: i32 = 9;
pub const SO_PRIORITY: i32 = 12;
pub const SO_REUSEPORT: i32 = 15;
/* C conditional: powerpc uses 16, other targets use 18. */
#[cfg(target_arch = "powerpc")]
pub const SO_RCVLOWAT: i32 = 16;
#[cfg(not(target_arch = "powerpc"))]
pub const SO_RCVLOWAT: i32 = 18;
pub const SO_BINDTODEVICE: i32 = 25;
pub const SO_MARK: i32 = 36;
pub const SO_MAX_PACING_RATE: i32 = 47;
pub const SO_BINDTOIFINDEX: i32 = 62;
pub const SO_TXREHASH: i32 = 74;
pub const __SO_ACCEPTCON: i32 = 1 << 16;

pub const IP_TOS: i32 = 1;
pub const IP_TRANSPARENT: i32 = 19;

pub const SOL_IPV6: i32 = 41;
pub const IPV6_TCLASS: i32 = 67;
pub const IPV6_AUTOFLOWLABEL: i32 = 70;
pub const IPV6_TRANSPARENT: i32 = 75;

pub const TC_ACT_UNSPEC: i32 = -1;
pub const TC_ACT_OK: i32 = 0;
pub const TC_ACT_SHOT: i32 = 2;

pub const SOL_TCP: i32 = 6;
pub const TCP_NODELAY: i32 = 1;
pub const TCP_MAXSEG: i32 = 2;
pub const TCP_KEEPIDLE: i32 = 4;
pub const TCP_KEEPINTVL: i32 = 5;
pub const TCP_KEEPCNT: i32 = 6;
pub const TCP_SYNCNT: i32 = 7;
pub const TCP_WINDOW_CLAMP: i32 = 10;
pub const TCP_CONGESTION: i32 = 13;
pub const TCP_THIN_LINEAR_TIMEOUTS: i32 = 16;
pub const TCP_USER_TIMEOUT: i32 = 18;
pub const TCP_NOTSENT_LOWAT: i32 = 25;
pub const TCP_SAVE_SYN: i32 = 27;
pub const TCP_SAVED_SYN: i32 = 28;
pub const TCP_CA_NAME_MAX: i32 = 16;
pub const TCP_NAGLE_OFF: i32 = 1;
pub const TCP_RTO_MAX_MS: i32 = 44;

pub const TCP_ECN_OK: i32 = 1;
pub const TCP_ECN_QUEUE_CWR: i32 = 2;
pub const TCP_ECN_DEMAND_CWR: i32 = 4;
pub const TCP_ECN_SEEN: i32 = 8;

pub const TCP_CONG_NEEDS_ECN: i32 = 0x2;

pub const ICSK_TIME_RETRANS: i32 = 1;
pub const ICSK_TIME_PROBE0: i32 = 3;
pub const ICSK_TIME_LOSS_PROBE: i32 = 5;
pub const ICSK_TIME_REO_TIMEOUT: i32 = 6;

pub const ETH_ALEN: i32 = 6;
pub const ETH_HLEN: i32 = 14;
pub const ETH_P_IP: i32 = 0x0800;
pub const ETH_P_IPV6: i32 = 0x86DD;

pub const NEXTHDR_TCP: i32 = 6;

pub const TCPOPT_NOP: i32 = 1;
pub const TCPOPT_EOL: i32 = 0;
pub const TCPOPT_MSS: i32 = 2;
pub const TCPOPT_WINDOW: i32 = 3;
pub const TCPOPT_TIMESTAMP: i32 = 8;
pub const TCPOPT_SACK_PERM: i32 = 4;

pub const TCPOLEN_MSS: i32 = 4;
pub const TCPOLEN_WINDOW: i32 = 3;
pub const TCPOLEN_TIMESTAMP: i32 = 10;
pub const TCPOLEN_SACK_PERM: i32 = 2;

pub const CHECKSUM_NONE: i32 = 0;
pub const CHECKSUM_PARTIAL: i32 = 3;

pub const IFNAMSIZ: i32 = 16;

pub const RTF_GATEWAY: i32 = 0x0002;

pub const TCP_INFINITE_SSTHRESH: i32 = 0x7fffffff;
pub const TCP_PINGPONG_THRESH: i32 = 3;

pub const FLAG_DATA_ACKED: i32 = 0x04; /* This ACK acknowledged new data.		*/
pub const FLAG_SYN_ACKED: i32 = 0x10; /* This ACK acknowledged SYN.		*/
pub const FLAG_DATA_SACKED: i32 = 0x20; /* New SACK.				*/
pub const FLAG_SND_UNA_ADVANCED: i32 = 0x400; /* Snd_una was changed (!= FLAG_DATA_ACKED) */
pub const FLAG_ACKED: i32 = FLAG_DATA_ACKED | FLAG_SYN_ACKED;
pub const FLAG_FORWARD_PROGRESS: i32 = FLAG_ACKED | FLAG_DATA_SACKED;

/* Field-alias macros from C, preserved as dependency intent:
 * fib_nh_dev        -> nh_common.nhc_dev
 * fib_nh_gw_family  -> nh_common.nhc_gw_family
 * fib_nh_gw6        -> nh_common.nhc_gw.ipv6
 * inet_daddr        -> sk.__sk_common.skc_daddr
 * inet_rcv_saddr    -> sk.__sk_common.skc_rcv_saddr
 * inet_dport        -> sk.__sk_common.skc_dport
 * udp_portaddr_hash -> inet.sk.__sk_common.skc_u16hashes[1]
 * ir_loc_addr       -> req.__req_common.skc_rcv_saddr
 * ir_num            -> req.__req_common.skc_num
 * ir_rmt_addr       -> req.__req_common.skc_daddr
 * ir_rmt_port       -> req.__req_common.skc_dport
 * ir_v6_rmt_addr    -> req.__req_common.skc_v6_daddr
 * ir_v6_loc_addr    -> req.__req_common.skc_v6_rcv_saddr
 * sk_num            -> __sk_common.skc_num
 * sk_dport          -> __sk_common.skc_dport
 * sk_family         -> __sk_common.skc_family
 * sk_rmem_alloc     -> sk_backlog.rmem_alloc
 * sk_refcnt         -> __sk_common.skc_refcnt
 * sk_state          -> __sk_common.skc_state
 * sk_net            -> __sk_common.skc_net
 * sk_rcv_saddr      -> __sk_common.skc_rcv_saddr
 * sk_v6_daddr       -> __sk_common.skc_v6_daddr
 * sk_v6_rcv_saddr   -> __sk_common.skc_v6_rcv_saddr
 * sk_flags          -> __sk_common.skc_flags
 * sk_reuse          -> __sk_common.skc_reuse
 * sk_cookie         -> __sk_common.skc_cookie
 * s6_addr32         -> in6_u.u6_addr32
 * tw_daddr          -> __tw_common.skc_daddr
 * tw_rcv_saddr      -> __tw_common.skc_rcv_saddr
 * tw_dport          -> __tw_common.skc_dport
 * tw_refcnt         -> __tw_common.skc_refcnt
 * tw_v6_daddr       -> __tw_common.skc_v6_daddr
 * tw_v6_rcv_saddr   -> __tw_common.skc_v6_rcv_saddr
 */

pub unsafe fn tcp_jiffies32() -> __u32 {
    bpf_jiffies64() as __u32
}

pub fn min<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

pub fn before(seq1: __u32, seq2: __u32) -> bool {
    (seq1.wrapping_sub(seq2) as __s32) < 0
}

pub fn after(seq2: __u32, seq1: __u32) -> bool {
    before(seq1, seq2)
}

pub unsafe fn inet_csk(sk: *const sock) -> *mut inet_connection_sock {
    sk as *mut inet_connection_sock
}

pub unsafe fn inet_csk_ca(sk: *const sock) -> *mut core::ffi::c_void {
    (*inet_csk(sk)).icsk_ca_priv.as_mut_ptr() as *mut core::ffi::c_void
}

pub unsafe fn tcp_sk(sk: *const sock) -> *mut tcp_sock {
    sk as *mut tcp_sock
}

pub unsafe fn tcp_in_slow_start(tp: *const tcp_sock) -> bool {
    (*tp).snd_cwnd < (*tp).snd_ssthresh
}

pub unsafe fn tcp_is_cwnd_limited(sk: *const sock) -> bool {
    let tp: *const tcp_sock = tcp_sk(sk);

    /* If in slow start, ensure cwnd grows to twice what was ACKed. */
    if tcp_in_slow_start(tp) {
        return (*tp).snd_cwnd < 2 * (*tp).max_packets_out;
    }

    BPF_CORE_READ_BITFIELD!(tp, is_cwnd_limited) != 0
}
