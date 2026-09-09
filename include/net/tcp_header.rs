/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Definitions for the TCP module. Direct translation of tcp.h. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external; the original header includes them from Linux and net headers.

pub const FASTRETRANS_DEBUG: i32 = 1;

pub const MAX_TCP_HEADER: usize = l1_cache_align(128 + MAX_HEADER);
pub const MAX_TCP_OPTION_SPACE: i32 = 40;
pub const TCP_MIN_SND_MSS: i32 = 48;
pub const TCP_MIN_GSO_SIZE: i32 = TCP_MIN_SND_MSS - MAX_TCP_OPTION_SPACE;
pub const MAX_TCP_WINDOW: u32 = 32767;
pub const TCP_MIN_MSS: u32 = 88;
pub const TCP_BASE_MSS: i32 = 1024;
pub const TCP_PROBE_INTERVAL: i32 = 600;
pub const TCP_PROBE_THRESHOLD: i32 = 8;
pub const TCP_FASTRETRANS_THRESH: i32 = 3;
pub const TCP_MAX_QUICKACKS: u32 = 16;
pub const TCP_MAX_WSCALE: u32 = 14;
pub const TCP_ACCECN_OPTION_BEACON: i32 = 3;
pub const TCP_URG_VALID: i32 = 0x0100;
pub const TCP_URG_NOTYET: i32 = 0x0200;
pub const TCP_URG_READ: i32 = 0x0400;
pub const TCP_RETR1: i32 = 3;
pub const TCP_RETR2: i32 = 15;
pub const TCP_SYN_RETRIES: i32 = 6;
pub const TCP_SYNACK_RETRIES: i32 = 5;
pub const TCP_TIMEWAIT_LEN: i32 = 60 * HZ;
pub const TCP_FIN_TIMEOUT: i32 = TCP_TIMEWAIT_LEN;
pub const TCP_FIN_TIMEOUT_MAX: i32 = 120 * HZ;
pub const TCP_DELACK_MAX: u32 = HZ as u32 / 5;
pub const TCP_DELACK_MIN: u32 = HZ as u32 / 25;
pub const TCP_ATO_MIN: u32 = HZ as u32 / 25;
pub const TCP_RTO_MAX_SEC: i32 = 120;
pub const TCP_RTO_MAX: u32 = TCP_RTO_MAX_SEC as u32 * HZ as u32;
pub const TCP_RTO_MIN: u32 = HZ as u32 / 5;
pub const TCP_TIMEOUT_MIN: u32 = 2;
pub const TCP_TIMEOUT_MIN_US: u32 = 2 * USEC_PER_MSEC;
pub const TCP_TIMEOUT_INIT: u32 = HZ as u32;
pub const TCP_TIMEOUT_FALLBACK: u32 = 3 * HZ as u32;
pub const TCP_RESOURCE_PROBE_INTERVAL: u32 = HZ as u32 / 2;
pub const TCP_KEEPALIVE_TIME: i32 = 120 * 60 * HZ;
pub const TCP_KEEPALIVE_PROBES: i32 = 9;
pub const TCP_KEEPALIVE_INTVL: i32 = 75 * HZ;
pub const MAX_TCP_KEEPIDLE: i32 = 32767;
pub const MAX_TCP_KEEPINTVL: i32 = 32767;
pub const MAX_TCP_KEEPCNT: i32 = 127;
pub const MAX_TCP_SYNCNT: i32 = 127;
pub const TCP_PAWS_WRAP: i32 = INT_MAX / USEC_PER_SEC;
pub const TCP_PAWS_MSL: i32 = 60;
pub const TCP_PAWS_WINDOW: i32 = 1;

pub const TCPOPT_NOP: i32 = 1;
pub const TCPOPT_EOL: i32 = 0;
pub const TCPOPT_MSS: i32 = 2;
pub const TCPOPT_WINDOW: i32 = 3;
pub const TCPOPT_SACK_PERM: i32 = 4;
pub const TCPOPT_SACK: i32 = 5;
pub const TCPOPT_TIMESTAMP: i32 = 8;
pub const TCPOPT_MD5SIG: i32 = 19;
pub const TCPOPT_AO: i32 = 29;
pub const TCPOPT_MPTCP: i32 = 30;
pub const TCPOPT_FASTOPEN: i32 = 34;
pub const TCPOPT_ACCECN0: i32 = 172;
pub const TCPOPT_ACCECN1: i32 = 174;
pub const TCPOPT_EXP: i32 = 254;
pub const TCPOPT_FASTOPEN_MAGIC: u16 = 0xF989;
pub const TCPOPT_SMC_MAGIC: u32 = 0xE2D4C3D9;

pub const TCPOLEN_MSS: i32 = 4;
pub const TCPOLEN_WINDOW: i32 = 3;
pub const TCPOLEN_SACK_PERM: i32 = 2;
pub const TCPOLEN_TIMESTAMP: i32 = 10;
pub const TCPOLEN_MD5SIG: i32 = 18;
pub const TCPOLEN_FASTOPEN_BASE: i32 = 2;
pub const TCPOLEN_ACCECN_BASE: i32 = 2;
pub const TCPOLEN_EXP_FASTOPEN_BASE: i32 = 4;
pub const TCPOLEN_EXP_SMC_BASE: i32 = 6;
pub const TCPOLEN_TSTAMP_ALIGNED: i32 = 12;
pub const TCPOLEN_WSCALE_ALIGNED: i32 = 4;
pub const TCPOLEN_SACKPERM_ALIGNED: i32 = 4;
pub const TCPOLEN_SACK_BASE: i32 = 2;
pub const TCPOLEN_SACK_BASE_ALIGNED: i32 = 4;
pub const TCPOLEN_SACK_PERBLOCK: i32 = 8;
pub const TCPOLEN_MD5SIG_ALIGNED: i32 = 20;
pub const TCPOLEN_MSS_ALIGNED: i32 = 4;
pub const TCPOLEN_EXP_SMC_BASE_ALIGNED: i32 = 8;
pub const TCPOLEN_ACCECN_PERFIELD: i32 = 3;
pub const TCP_ACCECN_NUMFIELDS: i32 = 3;
pub const TCP_ACCECN_MAXSIZE: i32 = TCPOLEN_ACCECN_BASE + TCPOLEN_ACCECN_PERFIELD * TCP_ACCECN_NUMFIELDS;
pub const TCP_ACCECN_SAFETY_SHIFT: i32 = 1;
pub const TCP_NAGLE_OFF: i32 = 1;
pub const TCP_NAGLE_CORK: i32 = 2;
pub const TCP_NAGLE_PUSH: i32 = 4;
pub const TCP_THIN_LINEAR_RETRIES: i32 = 6;
pub const TCP_INIT_CWND: i32 = 10;
pub const TFO_CLIENT_ENABLE: i32 = 1;
pub const TFO_SERVER_ENABLE: i32 = 2;
pub const TFO_CLIENT_NO_COOKIE: i32 = 4;
pub const TFO_SERVER_COOKIE_NOT_REQD: i32 = 0x200;
pub const TFO_SERVER_WO_SOCKOPT1: i32 = 0x400;
pub const TCP_RACK_LOSS_DETECTION: i32 = 0x1;
pub const TCP_RACK_STATIC_REO_WND: i32 = 0x2;
pub const TCP_RACK_NO_DUPTHRESH: i32 = 0x4;

extern "C" {
    pub static mut tcp_hashinfo: inet_hashinfo;
    pub static mut sysctl_tcp_max_orphans: i32;
    pub static mut sysctl_tcp_mem: [i64; 3];
    pub static mut tcp_sockets_allocated: percpu_counter;
    pub static mut tcp_memory_pressure: u64;
    pub static mut tcp_prot: proto;
}

pub unsafe fn tcp_orphan_count_inc() { this_cpu_inc(tcp_orphan_count); }
pub unsafe fn tcp_orphan_count_dec() { this_cpu_dec(tcp_orphan_count); }
pub unsafe fn before(seq1: u32, seq2: u32) -> bool { (seq1.wrapping_sub(seq2) as i32) < 0 }
pub unsafe fn after(seq2: u32, seq1: u32) -> bool { before(seq1, seq2) }
pub unsafe fn between(seq1: u32, seq2: u32, seq3: u32) -> bool {
    seq3.wrapping_sub(seq2) >= seq1.wrapping_sub(seq2)
}

#[repr(C)]
pub struct tcp_splice_state {
    pub pipe: *mut pipe_inode_info,
    pub len: usize,
    pub flags: u32,
}

pub const TCP_ECN_MODE_RFC3168: u8 = BIT(0);
pub const TCP_ECN_QUEUE_CWR: u8 = BIT(1);
pub const TCP_ECN_DEMAND_CWR: u8 = BIT(2);
pub const TCP_ECN_SEEN: u8 = BIT(3);
pub const TCP_ECN_MODE_ACCECN: u8 = BIT(4);
pub const TCP_ECN_DISABLED: u8 = 0;
pub const TCP_ECN_MODE_PENDING: u8 = TCP_ECN_MODE_RFC3168 | TCP_ECN_MODE_ACCECN;
pub const TCP_ECN_MODE_ANY: u8 = TCP_ECN_MODE_RFC3168 | TCP_ECN_MODE_ACCECN;

pub unsafe fn tcp_ecn_mode_any(tp: *const tcp_sock) -> bool { ((*tp).ecn_flags & TCP_ECN_MODE_ANY) != 0 }
pub unsafe fn tcp_ecn_mode_rfc3168(tp: *const tcp_sock) -> bool { ((*tp).ecn_flags & TCP_ECN_MODE_ANY) == TCP_ECN_MODE_RFC3168 }
pub unsafe fn tcp_ecn_mode_accecn(tp: *const tcp_sock) -> bool { ((*tp).ecn_flags & TCP_ECN_MODE_ANY) == TCP_ECN_MODE_ACCECN }
pub unsafe fn tcp_ecn_disabled(tp: *const tcp_sock) -> bool { !tcp_ecn_mode_any(tp) }
pub unsafe fn tcp_ecn_mode_pending(tp: *const tcp_sock) -> bool { ((*tp).ecn_flags & TCP_ECN_MODE_PENDING) == TCP_ECN_MODE_PENDING }
pub unsafe fn tcp_ecn_mode_set(tp: *mut tcp_sock, mode: u8) { (*tp).ecn_flags = ((*tp).ecn_flags & !TCP_ECN_MODE_ANY) | mode; }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum tcp_tw_status { TCP_TW_SUCCESS = 0, TCP_TW_RST = 1, TCP_TW_ACK = 2, TCP_TW_SYN = 3, TCP_TW_ACK_OOW = 4 }

extern "C" {
    pub fn tcp_orphan_count_sum() -> i32;
    pub fn tcp_time_wait(sk: *mut sock, state: i32, timeo: i32);
    pub fn sk_forced_mem_schedule(sk: *mut sock, size: i32);
    pub fn tcp_check_oom(sk: *const sock, shift: i32) -> bool;
    pub fn tcp_tsq_work_init();
    pub fn tcp_v4_err(skb: *mut sk_buff, arg: u32) -> i32;
    pub fn tcp_shutdown(sk: *mut sock, how: i32);
    pub fn tcp_v4_rcv(skb: *mut sk_buff) -> i32;
    pub fn tcp_remove_empty_skb(sk: *mut sock);
    pub fn tcp_sendmsg(sk: *mut sock, msg: *mut msghdr, size: usize) -> i32;
    pub fn tcp_sendmsg_locked(sk: *mut sock, msg: *mut msghdr, size: usize) -> i32;
    pub fn tcp_sendmsg_fastopen(sk: *mut sock, msg: *mut msghdr, copied: *mut i32, size: usize, uarg: *mut ubuf_info) -> i32;
    pub fn tcp_splice_eof(sock: *mut socket);
    pub fn tcp_send_mss(sk: *mut sock, size_goal: *mut i32, flags: i32) -> i32;
    pub fn tcp_wmem_schedule(sk: *mut sock, copy: i32) -> i32;
    pub fn tcp_push(sk: *mut sock, flags: i32, mss_now: i32, nonagle: i32, size_goal: i32);
    pub fn tcp_release_cb(sk: *mut sock);
    pub fn tcp_write_timer_handler(sk: *mut sock);
    pub fn tcp_delack_timer_handler(sk: *mut sock);
    pub fn tcp_ioctl(sk: *mut sock, cmd: i32, karg: *mut i32) -> i32;
    pub fn tcp_rcv_state_process(sk: *mut sock, skb: *mut sk_buff) -> skb_drop_reason;
    pub fn tcp_rcv_established(sk: *mut sock, skb: *mut sk_buff);
    pub fn tcp_rcvbuf_grow(sk: *mut sock, newval: u32);
    pub fn tcp_rcv_space_adjust(sk: *mut sock);
    pub fn tcp_twsk_unique(sk: *mut sock, sktw: *mut sock, twp: *mut core::ffi::c_void) -> i32;
    pub fn tcp_twsk_destructor(sk: *mut sock);
    pub fn tcp_twsk_purge(net_exit_list: *mut list_head);
    pub fn tcp_splice_data_recv(rd_desc: *mut read_descriptor_t, skb: *mut sk_buff, offset: u32, len: usize) -> i32;
    pub fn tcp_splice_read(sk: *mut socket, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: usize, flags: u32) -> isize;
    pub fn tcp_stream_alloc_skb(sk: *mut sock, gfp: gfp_t, force_schedule: bool) -> *mut sk_buff;
    pub fn tcp_timewait_state_process(tw: *mut inet_timewait_sock, skb: *mut sk_buff, th: *const tcphdr, tw_isn: *mut u32, drop_reason: *mut skb_drop_reason) -> tcp_tw_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
