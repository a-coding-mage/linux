/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Definitions for the TCP protocol. Translated from linux/tcp.h. */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
pub unsafe fn tcp_hdr(skb: *const sk_buff) -> *mut tcphdr { skb_transport_header(skb) as *mut tcphdr }
#[inline]
pub unsafe fn __tcp_hdrlen(th: *const tcphdr) -> u32 { ((*th).doff as u32) * 4 }
#[inline]
pub unsafe fn tcp_hdrlen(skb: *const sk_buff) -> u32 { __tcp_hdrlen(tcp_hdr(skb)) }
#[inline]
pub unsafe fn inner_tcp_hdr(skb: *const sk_buff) -> *mut tcphdr { skb_inner_transport_header(skb) as *mut tcphdr }
#[inline]
pub unsafe fn inner_tcp_hdrlen(skb: *const sk_buff) -> u32 { ((*inner_tcp_hdr(skb)).doff as u32) * 4 }

/** skb_tcp_all_headers - Returns size of all headers for a TCP packet */
#[inline]
pub unsafe fn skb_tcp_all_headers(skb: *const sk_buff) -> i32 { skb_transport_offset(skb) + tcp_hdrlen(skb) as i32 }
/** skb_inner_tcp_all_headers - Returns size of all headers for an encap TCP packet */
#[inline]
pub unsafe fn skb_inner_tcp_all_headers(skb: *const sk_buff) -> i32 { skb_inner_transport_offset(skb) + inner_tcp_hdrlen(skb) as i32 }
#[inline]
pub unsafe fn tcp_optlen(skb: *const sk_buff) -> u32 { (((*tcp_hdr(skb)).doff as u32) - 5) * 4 }

pub const TCP_FASTOPEN_COOKIE_MIN: usize = 4;
pub const TCP_FASTOPEN_COOKIE_MAX: usize = 16;
pub const TCP_FASTOPEN_COOKIE_SIZE: usize = 8;

#[repr(C)]
pub struct tcp_fastopen_cookie { pub val: [__le64; 2], pub len: i8, pub exp: bool }
#[repr(C)]
pub struct tcp_sack_block_wire { pub start_seq: __be32, pub end_seq: __be32 }
#[repr(C)]
pub struct tcp_sack_block { pub start_seq: u32, pub end_seq: u32 }
pub const TCP_SACK_SEEN: u32 = 1 << 0;
pub const TCP_DSACK_SEEN: u32 = 1 << 2;

#[repr(C)]
pub struct tcp_options_received {
    pub ts_recent_stamp: i32, pub ts_recent: u32, pub rcv_tsval: u32, pub rcv_tsecr: u32,
    pub saw_tstamp: u16, pub tstamp_ok: u16, pub dsack: u16, pub wscale_ok: u16,
    pub sack_ok: u16, pub smc_ok: u16, pub snd_wscale: u16, pub rcv_wscale: u16,
    pub accecn: u8, pub saw_unknown: u8, pub unused: u8, pub num_sacks: u8,
    pub user_mss: u16, pub mss_clamp: u16,
}
#[inline]
pub unsafe fn tcp_clear_options(rx_opt: *mut tcp_options_received) {
    (*rx_opt).tstamp_ok = 0; (*rx_opt).sack_ok = 0; (*rx_opt).wscale_ok = 0;
    (*rx_opt).snd_wscale = 0; /* CONFIG_SMC conditionally clears smc_ok. */
}
pub const TCP_NUM_SACKS: usize = 4;

#[repr(C)] pub struct tcp_request_sock_ops;
#[repr(C)]
pub struct tcp_request_sock {
    pub req: inet_request_sock, pub af_specific: *const tcp_request_sock_ops,
    pub snt_synack: u64, pub tfo_listener: bool, pub is_mptcp: bool, pub req_usec_ts: bool,
    pub drop_req: bool, pub txhash: u32, pub rcv_isn: u32, pub snt_isn: u32, pub ts_off: u32,
    pub snt_tsval_first: u32, pub snt_tsval_last: u32, pub last_oow_ack_time: u32, pub rcv_nxt: u32,
    pub syn_tos: u8, pub accecn_ok: bool, pub syn_ect_snt: u8, pub syn_ect_rcv: u8,
    pub accecn_fail_mode: u8, pub saw_accecn_opt: u8, pub ao_keyid: u8, pub ao_rcv_next: u8,
    pub used_tcp_ao: bool,
}
#[inline] pub unsafe fn tcp_rsk_used_ao(_req: *const request_sock) -> bool { false /* CONFIG_TCP_AO selects used_tcp_ao. */ }
pub const TCP_RMEM_TO_WIN_SCALE: u32 = 8;

/* Cacheline-group annotations and C bit-fields are represented by their underlying scalar fields. */
#[repr(C)]
pub struct tcp_sock {
    pub inet_conn: inet_connection_sock,
    pub max_window: u32, pub rcv_ssthresh: u32, pub reordering: u32, pub notsent_lowat: u32,
    pub gso_segs: u16, pub retransmit_skb_hint: *mut sk_buff, pub tcp_clean_acked: Option<unsafe extern "C" fn(*mut sock, u32)>,
    pub tsoffset: u32, pub snd_wnd: u32, pub mss_cache: u32, pub snd_cwnd: u32, pub prr_out: u32,
    pub lost_out: u32, pub sacked_out: u32, pub tcp_header_len: u16, pub scaling_ratio: u8, pub repair: u8,
    pub tcp_usec_ts: u8, pub is_sack_reneg: u8, pub is_cwnd_limited: u8, pub recvmsg_inq: u8,
    pub copied_seq: u32, pub snd_wl1: u32, pub tlp_high_seq: u32, pub rttvar_us: u32, pub retrans_out: u32,
    pub advmss: u16, pub urg_data: u16, pub lost: u32, pub snd_ssthresh: u32, pub rtt_min: minmax,
    pub out_of_order_queue: rb_root,
    pub delivered: u32, pub delivered_ce: u32, pub bytes_acked: u64, pub bytes_sent: u64,
    pub first_tx_mstamp: u64, pub delivered_mstamp: u64, pub data_segs_out: u32, pub snd_sml: u32,
    pub chrono_type: u8, pub chrono_start: u32, pub chrono_stat: [u32; 3], pub write_seq: u32,
    pub pushed_seq: u32, pub lsndtime: u32, pub mdev_us: u32, pub rtt_seq: u32, pub max_packets_out: u32,
    pub cwnd_usage_seq: u32, pub rate_delivered: u32, pub rate_interval_us: u32, pub tcp_wstamp_ns: u64,
    pub accecn_opt_tstamp: u64, pub tsorted_sent_queue: list_head, pub highest_sack: *mut sk_buff, pub ecn_flags: u8,
    pub nonagle: u8, pub rate_app_limited: u8, pub received_ce_pending: u8, pub accecn_opt_sent_w_dsack: u8,
    pub unused2: u8, pub accecn_minlen: u8, pub est_ecnfield: u8, pub accecn_opt_demand: u8, pub prev_ecnfield: u8,
    pub pred_flags: __be32, pub tcp_clock_cache: u64, pub tcp_mstamp: u64, pub rcv_nxt: u32, pub snd_nxt: u32,
    pub snd_una: u32, pub window_clamp: u32, pub srtt_us: u32, pub packets_out: u32, pub snd_up: u32,
    pub received_ce: u32, pub received_ecn_bytes: [u32; 3], pub app_limited: u32, pub rcv_wnd: u32,
    pub rcv_mwnd_seq: u32, pub rcv_tstamp: u32, pub rx_opt: tcp_options_received, pub segs_in: u32, pub segs_out: u32,
    pub bytes_received: u64, pub data_segs_in: u32, pub rcv_wup: u32, pub rcv_rtt_last_tsecr: u32,
    pub delivered_ecn_bytes: [u32; 3], pub pkts_acked_ewma: u16,
    pub rcv_rtt_est: tcp_rtt_est, pub rcvq_space: tcp_rcvq_space,
    pub dsack_dups: u32, pub compressed_ack_rcv_nxt: u32, pub tsq_node: list_head, pub rack: tcp_rack,
    pub compressed_ack: u8, pub dup_ack_counter: u8, pub tlp_retrans: u8, pub syn_ect_snt: u8, pub syn_ect_rcv: u8,
    pub thin_lto: u8, pub fastopen_connect: u8, pub fastopen_no_cookie: u8, pub fastopen_client_fail: u8, pub frto: u8,
    pub repair_queue: u8, pub save_syn: u8, pub syn_data: u8, pub syn_fastopen: u8, pub syn_fastopen_exp: u8,
    pub syn_fastopen_ch: u8, pub syn_data_acked: u8, pub syn_fastopen_child: u8, pub keepalive_probes: u8,
    pub accecn_fail_mode: u8, pub saw_accecn_opt: u8, pub tcp_tx_delay: u32, pub mdev_max_us: u32, pub reord_seen: u32,
    pub snd_cwnd_cnt: u32, pub snd_cwnd_clamp: u32, pub snd_cwnd_used: u32, pub snd_cwnd_stamp: u32, pub prior_cwnd: u32,
    pub prr_delivered: u32, pub last_oow_ack_time: u32, pub pacing_timer: hrtimer, pub compressed_ack_timer: hrtimer,
    pub ooo_last_skb: *mut sk_buff, pub duplicate_sack: [tcp_sack_block; 1], pub selective_acks: [tcp_sack_block; 4],
    pub recv_sack_cache: [tcp_sack_block; 4], pub prior_ssthresh: u32, pub high_seq: u32, pub retrans_stamp: u32,
    pub undo_marker: u32, pub undo_retrans: i32, pub mtu_info: u32, pub bytes_retrans: u64, pub total_retrans: u32,
    pub rto_stamp: u32, pub total_rto: u16, pub total_rto_recoveries: u16, pub total_rto_time: u32, pub urg_seq: u32,
    pub keepalive_time: u32, pub keepalive_intvl: u32, pub linger2: i32, pub bpf_sock_ops_cb_flags: u8,
    pub bpf_chg_cc_inprogress: u8, pub timeout_rehash: u16, pub rcv_ooopack: u32, pub mtu_probe: tcp_mtu_probe,
    pub plb_rehash: u32, pub is_mptcp: bool, pub syn_smc: bool, pub smc_hs_congested: Option<unsafe extern "C" fn(*const sock) -> bool>,
    pub af_specific: *const tcp_sock_af_ops, pub md5sig_info: *mut tcp_md5sig_info, pub ao_info: *mut tcp_ao_info,
    pub fastopen_req: *mut tcp_fastopen_request, pub fastopen_rsk: *mut request_sock, pub saved_syn: *mut saved_syn,
}
#[repr(C)] pub struct tcp_rtt_est { pub rtt_us: u32, pub seq: u32, pub time: u64 }
#[repr(C)] pub struct tcp_rcvq_space { pub space: i32, pub seq: u32, pub time: u64 }
#[repr(C)] pub struct tcp_rack { pub mstamp: u64, pub rtt_us: u32, pub end_seq: u32, pub last_delivered: u32, pub reo_wnd_steps: u8, pub reo_wnd_persist: u8, pub dsack_seen: u8, pub advanced: u8 }
pub const TCP_RACK_RECOVERY_THRESH: u8 = 16;
#[repr(C)] pub struct tcp_mtu_probe { pub probe_seq_start: u32, pub probe_seq_end: u32 }

pub enum tsq_enum { TSQ_THROTTLED, TSQ_QUEUED, TCP_TSQ_DEFERRED, TCP_WRITE_TIMER_DEFERRED, TCP_DELACK_TIMER_DEFERRED, TCP_MTU_REDUCED_DEFERRED, TCP_ACK_DEFERRED }
pub const TSQF_THROTTLED: u32 = 1 << 0; pub const TSQF_QUEUED: u32 = 1 << 1; pub const TCPF_TSQ_DEFERRED: u32 = 1 << 2;
pub const TCPF_WRITE_TIMER_DEFERRED: u32 = 1 << 3; pub const TCPF_DELACK_TIMER_DEFERRED: u32 = 1 << 4;
pub const TCPF_MTU_REDUCED_DEFERRED: u32 = 1 << 5; pub const TCPF_ACK_DEFERRED: u32 = 1 << 6;
pub const TCP_DEFERRED_ALL: u32 = TCPF_TSQ_DEFERRED | TCPF_WRITE_TIMER_DEFERRED | TCPF_DELACK_TIMER_DEFERRED | TCPF_MTU_REDUCED_DEFERRED | TCPF_ACK_DEFERRED;

#[repr(C)] pub struct tcp_timewait_sock { pub tw_sk: inet_timewait_sock, pub tw_rcv_wnd: u32, pub tw_ts_offset: u32, pub tw_ts_recent: u32, pub tw_last_oow_ack_time: u32, pub tw_ts_recent_stamp: i32, pub tw_tx_delay: u32, pub tw_md5_key: *mut tcp_md5sig_key, pub ao_info: *mut tcp_ao_info }
#[inline] pub unsafe fn tcp_twsk(sk: *const sock) -> *mut tcp_timewait_sock { sk as *mut tcp_timewait_sock }
#[inline] pub unsafe fn tcp_passive_fastopen(sk: *const sock) -> bool { (*sk).sk_state == TCP_SYN_RECV && !(*tcp_sk(sk)).fastopen_rsk.is_null() }
#[inline] pub unsafe fn tcp_move_syn(tp: *mut tcp_sock, req: *mut request_sock) { (*tp).saved_syn = (*req).saved_syn; (*req).saved_syn = core::ptr::null_mut(); }
#[inline] pub unsafe fn tcp_saved_syn_free(tp: *mut tcp_sock) { kfree((*tp).saved_syn); (*tp).saved_syn = core::ptr::null_mut(); }
#[inline] pub unsafe fn tcp_saved_syn_len(s: *const saved_syn) -> u32 { (*s).mac_hdrlen + (*s).network_hdrlen + (*s).tcp_hdrlen }

pub unsafe extern "C" fn tcp_get_timestamping_opt_stats(_: *const sock, _: *const sk_buff, _: *const sk_buff) -> *mut sk_buff;
#[inline] pub unsafe fn tcp_mss_clamp(tp: *const tcp_sock, mss: u16) -> u16 { let u = (*tp).rx_opt.user_mss; if u != 0 && u < mss { u } else { mss } }
pub unsafe extern "C" fn tcp_skb_shift(_: *mut sk_buff, _: *mut sk_buff, _: i32, _: i32) -> i32;
pub unsafe extern "C" fn __tcp_sock_set_cork(_: *mut sock, _: bool); pub unsafe extern "C" fn tcp_sock_set_cork(_: *mut sock, _: bool);
pub unsafe extern "C" fn tcp_sock_set_keepcnt(_: *mut sock, _: i32) -> i32; pub unsafe extern "C" fn tcp_sock_set_keepidle_locked(_: *mut sock, _: i32) -> i32;
pub unsafe extern "C" fn tcp_sock_set_keepidle(_: *mut sock, _: i32) -> i32; pub unsafe extern "C" fn tcp_sock_set_keepintvl(_: *mut sock, _: i32) -> i32;
pub unsafe extern "C" fn __tcp_sock_set_nodelay(_: *mut sock, _: bool); pub unsafe extern "C" fn tcp_sock_set_nodelay(_: *mut sock);
pub unsafe extern "C" fn tcp_sock_set_quickack(_: *mut sock, _: i32); pub unsafe extern "C" fn tcp_sock_set_syncnt(_: *mut sock, _: i32) -> i32;
pub unsafe extern "C" fn tcp_sock_set_user_timeout(_: *mut sock, _: i32) -> i32; pub unsafe extern "C" fn tcp_sock_set_maxseg(_: *mut sock, _: i32) -> i32;
#[inline] pub unsafe fn dst_tcp_usec_ts(dst: *const dst_entry) -> bool { dst_feature(dst, RTAX_FEATURE_TCP_USEC_TS) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
