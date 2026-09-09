/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Rust translation of uapi/linux/tcp.h. */

#[repr(C)]
pub struct tcphdr {
    pub source: __be16, pub dest: __be16, pub seq: __be32, pub ack_seq: __be32,
    /* ae:1, res1:3, doff:4, fin..cwr:1 (endianness-dependent C bitfields). */
    pub flags: __u16,
    pub window: __be16, pub check: __sum16, pub urg_ptr: __be16,
}

#[repr(C)]
pub union tcp_word_hdr { pub hdr: tcphdr, pub words: [__be32; 5] }

#[inline]
pub unsafe fn tcp_flag_word(tp: *mut tcphdr) -> __be32 {
    (*((tp as *mut tcp_word_hdr))).words[3]
}

pub const TCP_FLAG_AE: __be32 = 0x01000000;
pub const TCP_FLAG_CWR: __be32 = 0x00800000;
pub const TCP_FLAG_ECE: __be32 = 0x00400000;
pub const TCP_FLAG_URG: __be32 = 0x00200000;
pub const TCP_FLAG_ACK: __be32 = 0x00100000;
pub const TCP_FLAG_PSH: __be32 = 0x00080000;
pub const TCP_FLAG_RST: __be32 = 0x00040000;
pub const TCP_FLAG_SYN: __be32 = 0x00020000;
pub const TCP_FLAG_FIN: __be32 = 0x00010000;
pub const TCP_RESERVED_BITS: __be32 = 0x0E000000;
pub const TCP_DATA_OFFSET: __be32 = 0xF0000000;

pub const TCP_MSS_DEFAULT: u32 = 536;
pub const TCP_MSS_DESIRED: u32 = 1220;
pub const TCP_NODELAY: i32 = 1; pub const TCP_MAXSEG: i32 = 2; pub const TCP_CORK: i32 = 3;
pub const TCP_KEEPIDLE: i32 = 4; pub const TCP_KEEPINTVL: i32 = 5; pub const TCP_KEEPCNT: i32 = 6;
pub const TCP_SYNCNT: i32 = 7; pub const TCP_LINGER2: i32 = 8; pub const TCP_DEFER_ACCEPT: i32 = 9;
pub const TCP_WINDOW_CLAMP: i32 = 10; pub const TCP_INFO: i32 = 11; pub const TCP_QUICKACK: i32 = 12;
pub const TCP_CONGESTION: i32 = 13; pub const TCP_MD5SIG: i32 = 14; pub const TCP_THIN_LINEAR_TIMEOUTS: i32 = 16;
pub const TCP_THIN_DUPACK: i32 = 17; pub const TCP_USER_TIMEOUT: i32 = 18; pub const TCP_REPAIR: i32 = 19;
pub const TCP_REPAIR_QUEUE: i32 = 20; pub const TCP_QUEUE_SEQ: i32 = 21; pub const TCP_REPAIR_OPTIONS: i32 = 22;
pub const TCP_FASTOPEN: i32 = 23; pub const TCP_TIMESTAMP: i32 = 24; pub const TCP_NOTSENT_LOWAT: i32 = 25;
pub const TCP_CC_INFO: i32 = 26; pub const TCP_SAVE_SYN: i32 = 27; pub const TCP_SAVED_SYN: i32 = 28;
pub const TCP_REPAIR_WINDOW: i32 = 29; pub const TCP_FASTOPEN_CONNECT: i32 = 30; pub const TCP_ULP: i32 = 31;
pub const TCP_MD5SIG_EXT: i32 = 32; pub const TCP_FASTOPEN_KEY: i32 = 33; pub const TCP_FASTOPEN_NO_COOKIE: i32 = 34;
pub const TCP_ZEROCOPY_RECEIVE: i32 = 35; pub const TCP_INQ: i32 = 36; pub const TCP_CM_INQ: i32 = TCP_INQ;
pub const TCP_TX_DELAY: i32 = 37; pub const TCP_AO_ADD_KEY: i32 = 38; pub const TCP_AO_DEL_KEY: i32 = 39;
pub const TCP_AO_INFO: i32 = 40; pub const TCP_AO_GET_KEYS: i32 = 41; pub const TCP_AO_REPAIR: i32 = 42;
pub const TCP_IS_MPTCP: i32 = 43; pub const TCP_RTO_MAX_MS: i32 = 44; pub const TCP_RTO_MIN_US: i32 = 45;
pub const TCP_DELACK_MAX_US: i32 = 46;
pub const TCP_REPAIR_ON: i32 = 1; pub const TCP_REPAIR_OFF: i32 = 0; pub const TCP_REPAIR_OFF_NO_WP: i32 = -1;

#[repr(C)] pub struct tcp_repair_opt { pub opt_code: __u32, pub opt_val: __u32 }
#[repr(C)] pub struct tcp_repair_window { pub snd_wl1: __u32, pub snd_wnd: __u32, pub max_window: __u32, pub rcv_wnd: __u32, pub rcv_wup: __u32 }

pub const TCP_NO_QUEUE: i32 = 0; pub const TCP_RECV_QUEUE: i32 = 1; pub const TCP_SEND_QUEUE: i32 = 2; pub const TCP_QUEUES_NR: i32 = 3;
pub const TFO_STATUS_UNSPEC: i32 = 0; pub const TFO_COOKIE_UNAVAILABLE: i32 = 1; pub const TFO_DATA_NOT_ACKED: i32 = 2; pub const TFO_SYN_RETRANSMITTED: i32 = 3;
pub const TCPI_OPT_TIMESTAMPS: u32 = 1; pub const TCPI_OPT_SACK: u32 = 2; pub const TCPI_OPT_WSCALE: u32 = 4; pub const TCPI_OPT_ECN: u32 = 8; pub const TCPI_OPT_ECN_SEEN: u32 = 16; pub const TCPI_OPT_SYN_DATA: u32 = 32; pub const TCPI_OPT_USEC_TS: u32 = 64; pub const TCPI_OPT_TFO_CHILD: u32 = 128;
pub const TCP_CA_Open: i32 = 0; pub const TCPF_CA_Open: u32 = 1 << TCP_CA_Open; pub const TCP_CA_Disorder: i32 = 1; pub const TCPF_CA_Disorder: u32 = 1 << TCP_CA_Disorder; pub const TCP_CA_CWR: i32 = 2; pub const TCPF_CA_CWR: u32 = 1 << TCP_CA_CWR; pub const TCP_CA_Recovery: i32 = 3; pub const TCPF_CA_Recovery: u32 = 1 << TCP_CA_Recovery; pub const TCP_CA_Loss: i32 = 4; pub const TCPF_CA_Loss: u32 = 1 << TCP_CA_Loss;
pub const TCPI_ECN_MODE_DISABLED: u32 = 0; pub const TCPI_ECN_MODE_RFC3168: u32 = 1; pub const TCPI_ECN_MODE_ACCECN: u32 = 2; pub const TCPI_ECN_MODE_PENDING: u32 = 3;
pub const TCP_ACCECN_OPT_NOT_SEEN: u32 = 0; pub const TCP_ACCECN_OPT_EMPTY_SEEN: u32 = 1; pub const TCP_ACCECN_OPT_COUNTER_SEEN: u32 = 2; pub const TCP_ACCECN_OPT_FAIL_SEEN: u32 = 3;
pub const TCP_ACCECN_ACE_FAIL_SEND: u32 = 1 << 0; pub const TCP_ACCECN_ACE_FAIL_RECV: u32 = 1 << 1; pub const TCP_ACCECN_OPT_FAIL_SEND: u32 = 1 << 2; pub const TCP_ACCECN_OPT_FAIL_RECV: u32 = 1 << 3;

#[repr(C)] pub struct tcp_info {
    pub tcpi_state: __u8, pub tcpi_ca_state: __u8, pub tcpi_retransmits: __u8, pub tcpi_probes: __u8, pub tcpi_backoff: __u8, pub tcpi_options: __u8, pub tcpi_snd_wscale_rcv_wscale: __u8, pub tcpi_delivery_rate_app_limited_fastopen_client_fail: __u8,
    pub tcpi_rto: __u32, pub tcpi_ato: __u32, pub tcpi_snd_mss: __u32, pub tcpi_rcv_mss: __u32, pub tcpi_unacked: __u32, pub tcpi_sacked: __u32, pub tcpi_lost: __u32, pub tcpi_retrans: __u32, pub tcpi_fackets: __u32,
    pub tcpi_last_data_sent: __u32, pub tcpi_last_ack_sent: __u32, pub tcpi_last_data_recv: __u32, pub tcpi_last_ack_recv: __u32, pub tcpi_pmtu: __u32, pub tcpi_rcv_ssthresh: __u32, pub tcpi_rtt: __u32, pub tcpi_rttvar: __u32, pub tcpi_snd_ssthresh: __u32, pub tcpi_snd_cwnd: __u32, pub tcpi_advmss: __u32, pub tcpi_reordering: __u32, pub tcpi_rcv_rtt: __u32, pub tcpi_rcv_space: __u32, pub tcpi_total_retrans: __u32,
    pub tcpi_pacing_rate: __u64, pub tcpi_max_pacing_rate: __u64, pub tcpi_bytes_acked: __u64, pub tcpi_bytes_received: __u64, pub tcpi_segs_out: __u32, pub tcpi_segs_in: __u32, pub tcpi_notsent_bytes: __u32, pub tcpi_min_rtt: __u32, pub tcpi_data_segs_in: __u32, pub tcpi_data_segs_out: __u32, pub tcpi_delivery_rate: __u64, pub tcpi_busy_time: __u64, pub tcpi_rwnd_limited: __u64, pub tcpi_sndbuf_limited: __u64, pub tcpi_delivered: __u32, pub tcpi_delivered_ce: __u32, pub tcpi_bytes_sent: __u64, pub tcpi_bytes_retrans: __u64, pub tcpi_dsack_dups: __u32, pub tcpi_reord_seen: __u32, pub tcpi_rcv_ooopack: __u32, pub tcpi_snd_wnd: __u32, pub tcpi_rcv_wnd: __u32, pub tcpi_rehash: __u32, pub tcpi_total_rto: __u16, pub tcpi_total_rto_recoveries: __u16, pub tcpi_total_rto_time: __u32, pub tcpi_received_ce: __u32, pub tcpi_delivered_e1_bytes: __u32, pub tcpi_delivered_e0_bytes: __u32, pub tcpi_delivered_ce_bytes: __u32, pub tcpi_received_e1_bytes: __u32, pub tcpi_received_e0_bytes: __u32, pub tcpi_received_ce_bytes: __u32, pub tcpi_ecn_mode_accecn_opt_seen_accecn_fail_mode_options2: __u32,
}

#[repr(C)] pub struct tcp_md5sig { pub tcpm_addr: __kernel_sockaddr_storage, pub tcpm_flags: __u8, pub tcpm_prefixlen: __u8, pub tcpm_keylen: __u16, pub tcpm_ifindex: i32, pub tcpm_key: [__u8; 80] }
#[repr(C)] pub struct tcp_diag_md5sig { pub tcpm_family: __u8, pub tcpm_prefixlen: __u8, pub tcpm_keylen: __u16, pub tcpm_addr: [__be32; 4], pub tcpm_key: [__u8; 80] }
pub const TCP_MD5SIG_MAXKEYLEN: usize = 80;
pub const TCP_MD5SIG_FLAG_PREFIX: u32 = 1; pub const TCP_MD5SIG_FLAG_IFINDEX: u32 = 2;

pub const TCP_AO_MAXKEYLEN: usize = 80; pub const TCP_AO_KEYF_IFINDEX: u32 = 1 << 0; pub const TCP_AO_KEYF_EXCLUDE_OPT: u32 = 1 << 1;
#[repr(C, align(8))] pub struct tcp_ao_add { pub addr: __kernel_sockaddr_storage, pub alg_name: [i8; 64], pub ifindex: __s32, pub flags: __u32, pub reserved2: __u16, pub prefix: __u8, pub sndid: __u8, pub rcvid: __u8, pub maclen: __u8, pub keyflags: __u8, pub keylen: __u8, pub key: [__u8; 80] }
#[repr(C, align(8))] pub struct tcp_ao_del { pub addr: __kernel_sockaddr_storage, pub ifindex: __s32, pub flags: __u32, pub reserved2: __u16, pub prefix: __u8, pub sndid: __u8, pub rcvid: __u8, pub current_key: __u8, pub rnext: __u8, pub keyflags: __u8 }
#[repr(C, align(8))] pub struct tcp_ao_info_opt { pub flags: __u32, pub reserved2: __u16, pub current_key: __u8, pub rnext: __u8, pub pkt_good: __u64, pub pkt_bad: __u64, pub pkt_key_not_found: __u64, pub pkt_ao_required: __u64, pub pkt_dropped_icmp: __u64 }
#[repr(C, align(8))] pub struct tcp_ao_getsockopt { pub addr: __kernel_sockaddr_storage, pub alg_name: [i8; 64], pub key: [__u8; 80], pub nkeys: __u32, pub flags: __u16, pub sndid: __u8, pub rcvid: __u8, pub prefix: __u8, pub maclen: __u8, pub keyflags: __u8, pub keylen: __u8, pub ifindex: __s32, pub pkt_good: __u64, pub pkt_bad: __u64 }
#[repr(C, align(8))] pub struct tcp_ao_repair { pub snt_isn: __be32, pub rcv_isn: __be32, pub snd_sne: __u32, pub rcv_sne: __u32 }

pub const TCP_RECEIVE_ZEROCOPY_FLAG_TLB_CLEAN_HINT: u32 = 1;
#[repr(C)] pub struct tcp_zerocopy_receive { pub address: __u64, pub length: __u32, pub recv_skip_hint: __u32, pub inq: __u32, pub err: __s32, pub copybuf_address: __u64, pub copybuf_len: __s32, pub flags: __u32, pub msg_control: __u64, pub msg_controllen: __u64, pub msg_flags: __u32, pub reserved: __u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
