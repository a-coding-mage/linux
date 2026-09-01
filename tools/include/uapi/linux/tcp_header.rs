/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Definitions for the TCP protocol.
 *
 * Version:	@(#)tcp.h	1.0.2	04/28/93
 *
 * Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *
 *		This program is free software; you can redistribute it and/or
 *		modify it under the terms of the GNU General Public License
 *		as published by the Free Software Foundation; either version
 *		2 of the License, or (at your option) any later version.
 */

/* Dependencies from the C header:
 * #include <linux/types.h>
 * #include <asm/byteorder.h>
 * #include <linux/socket.h>
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcphdr {
    pub source: __be16,
    pub dest: __be16,
    pub seq: __be32,
    pub ack_seq: __be32,
    /*
     * C bitfields over one __u16:
     * little endian: res1:4, doff:4, fin:1, syn:1, rst:1, psh:1, ack:1, urg:1, ece:1, cwr:1
     * big endian:    doff:4, res1:4, cwr:1, ece:1, urg:1, ack:1, psh:1, rst:1, syn:1, fin:1
     */
    pub bitfield_1: __u16,
    pub window: __be16,
    pub check: __sum16,
    pub urg_ptr: __be16,
}

impl tcphdr {
    #[cfg(target_endian = "little")]
    pub fn res1(&self) -> __u16 {
        self.bitfield_1 & 0x000f
    }

    #[cfg(target_endian = "little")]
    pub fn set_res1(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x000f) | (value & 0x000f);
    }

    #[cfg(target_endian = "little")]
    pub fn doff(&self) -> __u16 {
        (self.bitfield_1 >> 4) & 0x000f
    }

    #[cfg(target_endian = "little")]
    pub fn set_doff(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x00f0) | ((value & 0x000f) << 4);
    }

    #[cfg(target_endian = "little")]
    pub fn fin(&self) -> __u16 {
        (self.bitfield_1 >> 8) & 0x0001
    }

    #[cfg(target_endian = "little")]
    pub fn set_fin(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0100) | ((value & 0x0001) << 8);
    }

    #[cfg(target_endian = "little")]
    pub fn syn(&self) -> __u16 {
        (self.bitfield_1 >> 9) & 0x0001
    }

    #[cfg(target_endian = "little")]
    pub fn set_syn(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0200) | ((value & 0x0001) << 9);
    }

    #[cfg(target_endian = "little")]
    pub fn rst(&self) -> __u16 {
        (self.bitfield_1 >> 10) & 0x0001
    }

    #[cfg(target_endian = "little")]
    pub fn set_rst(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0400) | ((value & 0x0001) << 10);
    }

    #[cfg(target_endian = "little")]
    pub fn psh(&self) -> __u16 {
        (self.bitfield_1 >> 11) & 0x0001
    }

    #[cfg(target_endian = "little")]
    pub fn set_psh(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0800) | ((value & 0x0001) << 11);
    }

    #[cfg(target_endian = "little")]
    pub fn ack(&self) -> __u16 {
        (self.bitfield_1 >> 12) & 0x0001
    }

    #[cfg(target_endian = "little")]
    pub fn set_ack(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x1000) | ((value & 0x0001) << 12);
    }

    #[cfg(target_endian = "little")]
    pub fn urg(&self) -> __u16 {
        (self.bitfield_1 >> 13) & 0x0001
    }

    #[cfg(target_endian = "little")]
    pub fn set_urg(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x2000) | ((value & 0x0001) << 13);
    }

    #[cfg(target_endian = "little")]
    pub fn ece(&self) -> __u16 {
        (self.bitfield_1 >> 14) & 0x0001
    }

    #[cfg(target_endian = "little")]
    pub fn set_ece(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x4000) | ((value & 0x0001) << 14);
    }

    #[cfg(target_endian = "little")]
    pub fn cwr(&self) -> __u16 {
        (self.bitfield_1 >> 15) & 0x0001
    }

    #[cfg(target_endian = "little")]
    pub fn set_cwr(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x8000) | ((value & 0x0001) << 15);
    }

    #[cfg(target_endian = "big")]
    pub fn doff(&self) -> __u16 {
        (self.bitfield_1 >> 12) & 0x000f
    }

    #[cfg(target_endian = "big")]
    pub fn set_doff(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0xf000) | ((value & 0x000f) << 12);
    }

    #[cfg(target_endian = "big")]
    pub fn res1(&self) -> __u16 {
        (self.bitfield_1 >> 8) & 0x000f
    }

    #[cfg(target_endian = "big")]
    pub fn set_res1(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0f00) | ((value & 0x000f) << 8);
    }

    #[cfg(target_endian = "big")]
    pub fn cwr(&self) -> __u16 {
        (self.bitfield_1 >> 7) & 0x0001
    }

    #[cfg(target_endian = "big")]
    pub fn set_cwr(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0080) | ((value & 0x0001) << 7);
    }

    #[cfg(target_endian = "big")]
    pub fn ece(&self) -> __u16 {
        (self.bitfield_1 >> 6) & 0x0001
    }

    #[cfg(target_endian = "big")]
    pub fn set_ece(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0040) | ((value & 0x0001) << 6);
    }

    #[cfg(target_endian = "big")]
    pub fn urg(&self) -> __u16 {
        (self.bitfield_1 >> 5) & 0x0001
    }

    #[cfg(target_endian = "big")]
    pub fn set_urg(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0020) | ((value & 0x0001) << 5);
    }

    #[cfg(target_endian = "big")]
    pub fn ack(&self) -> __u16 {
        (self.bitfield_1 >> 4) & 0x0001
    }

    #[cfg(target_endian = "big")]
    pub fn set_ack(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0010) | ((value & 0x0001) << 4);
    }

    #[cfg(target_endian = "big")]
    pub fn psh(&self) -> __u16 {
        (self.bitfield_1 >> 3) & 0x0001
    }

    #[cfg(target_endian = "big")]
    pub fn set_psh(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0008) | ((value & 0x0001) << 3);
    }

    #[cfg(target_endian = "big")]
    pub fn rst(&self) -> __u16 {
        (self.bitfield_1 >> 2) & 0x0001
    }

    #[cfg(target_endian = "big")]
    pub fn set_rst(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0004) | ((value & 0x0001) << 2);
    }

    #[cfg(target_endian = "big")]
    pub fn syn(&self) -> __u16 {
        (self.bitfield_1 >> 1) & 0x0001
    }

    #[cfg(target_endian = "big")]
    pub fn set_syn(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0002) | ((value & 0x0001) << 1);
    }

    #[cfg(target_endian = "big")]
    pub fn fin(&self) -> __u16 {
        self.bitfield_1 & 0x0001
    }

    #[cfg(target_endian = "big")]
    pub fn set_fin(&mut self, value: __u16) {
        self.bitfield_1 = (self.bitfield_1 & !0x0001) | (value & 0x0001);
    }
}

/*
 *	The union cast uses a gcc extension to avoid aliasing problems
 *  (union is compatible to any of its members)
 *  This means this part of the code is -fstrict-aliasing safe now.
 */
#[repr(C)]
pub union tcp_word_hdr {
    pub hdr: tcphdr,
    pub words: [__be32; 5],
}

pub unsafe fn tcp_flag_word(tp: *const tcphdr) -> __be32 {
    (*(tp as *const tcp_word_hdr)).words[3]
}

pub const TCP_FLAG_CWR: __be32 = __constant_cpu_to_be32(0x00800000);
pub const TCP_FLAG_ECE: __be32 = __constant_cpu_to_be32(0x00400000);
pub const TCP_FLAG_URG: __be32 = __constant_cpu_to_be32(0x00200000);
pub const TCP_FLAG_ACK: __be32 = __constant_cpu_to_be32(0x00100000);
pub const TCP_FLAG_PSH: __be32 = __constant_cpu_to_be32(0x00080000);
pub const TCP_FLAG_RST: __be32 = __constant_cpu_to_be32(0x00040000);
pub const TCP_FLAG_SYN: __be32 = __constant_cpu_to_be32(0x00020000);
pub const TCP_FLAG_FIN: __be32 = __constant_cpu_to_be32(0x00010000);
pub const TCP_RESERVED_BITS: __be32 = __constant_cpu_to_be32(0x0F000000);
pub const TCP_DATA_OFFSET: __be32 = __constant_cpu_to_be32(0xF0000000);

/*
 * TCP general constants
 */
pub const TCP_MSS_DEFAULT: u32 = 536; /* IPv4 (RFC1122, RFC2581) */
pub const TCP_MSS_DESIRED: u32 = 1220; /* IPv6 (tunneled), EDNS0 (RFC3226) */

/* TCP socket options */
pub const TCP_NODELAY: i32 = 1; /* Turn off Nagle's algorithm. */
pub const TCP_MAXSEG: i32 = 2; /* Limit MSS */
pub const TCP_CORK: i32 = 3; /* Never send partially complete segments */
pub const TCP_KEEPIDLE: i32 = 4; /* Start keeplives after this period */
pub const TCP_KEEPINTVL: i32 = 5; /* Interval between keepalives */
pub const TCP_KEEPCNT: i32 = 6; /* Number of keepalives before death */
pub const TCP_SYNCNT: i32 = 7; /* Number of SYN retransmits */
pub const TCP_LINGER2: i32 = 8; /* Life time of orphaned FIN-WAIT-2 state */
pub const TCP_DEFER_ACCEPT: i32 = 9; /* Wake up listener only when data arrive */
pub const TCP_WINDOW_CLAMP: i32 = 10; /* Bound advertised window */
pub const TCP_INFO: i32 = 11; /* Information about this connection. */
pub const TCP_QUICKACK: i32 = 12; /* Block/reenable quick acks */
pub const TCP_CONGESTION: i32 = 13; /* Congestion control algorithm */
pub const TCP_MD5SIG: i32 = 14; /* TCP MD5 Signature (RFC2385) */
pub const TCP_THIN_LINEAR_TIMEOUTS: i32 = 16; /* Use linear timeouts for thin streams*/
pub const TCP_THIN_DUPACK: i32 = 17; /* Fast retrans. after 1 dupack */
pub const TCP_USER_TIMEOUT: i32 = 18; /* How long for loss retry before timeout */
pub const TCP_REPAIR: i32 = 19; /* TCP sock is under repair right now */
pub const TCP_REPAIR_QUEUE: i32 = 20;
pub const TCP_QUEUE_SEQ: i32 = 21;
pub const TCP_REPAIR_OPTIONS: i32 = 22;
pub const TCP_FASTOPEN: i32 = 23; /* Enable FastOpen on listeners */
pub const TCP_TIMESTAMP: i32 = 24;
pub const TCP_NOTSENT_LOWAT: i32 = 25; /* limit number of unsent bytes in write queue */
pub const TCP_CC_INFO: i32 = 26; /* Get Congestion Control (optional) info */
pub const TCP_SAVE_SYN: i32 = 27; /* Record SYN headers for new connections */
pub const TCP_SAVED_SYN: i32 = 28; /* Get SYN headers recorded for connection */
pub const TCP_REPAIR_WINDOW: i32 = 29; /* Get/set window parameters */
pub const TCP_FASTOPEN_CONNECT: i32 = 30; /* Attempt FastOpen with connect */
pub const TCP_ULP: i32 = 31; /* Attach a ULP to a TCP connection */
pub const TCP_MD5SIG_EXT: i32 = 32; /* TCP MD5 Signature with extensions */
pub const TCP_FASTOPEN_KEY: i32 = 33; /* Set the key for Fast Open (cookie) */
pub const TCP_FASTOPEN_NO_COOKIE: i32 = 34; /* Enable TFO without a TFO cookie */
pub const TCP_ZEROCOPY_RECEIVE: i32 = 35;
pub const TCP_INQ: i32 = 36; /* Notify bytes available to read as a cmsg on read */

pub const TCP_CM_INQ: i32 = TCP_INQ;

pub const TCP_TX_DELAY: i32 = 37; /* delay outgoing packets by XX usec */

pub const TCP_REPAIR_ON: i32 = 1;
pub const TCP_REPAIR_OFF: i32 = 0;
pub const TCP_REPAIR_OFF_NO_WP: i32 = -1; /* Turn off without window probes */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_repair_opt {
    pub opt_code: __u32,
    pub opt_val: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_repair_window {
    pub snd_wl1: __u32,
    pub snd_wnd: __u32,
    pub max_window: __u32,

    pub rcv_wnd: __u32,
    pub rcv_wup: __u32,
}

pub const TCP_NO_QUEUE: i32 = 0;
pub const TCP_RECV_QUEUE: i32 = 1;
pub const TCP_SEND_QUEUE: i32 = 2;
pub const TCP_QUEUES_NR: i32 = 3;

/* why fastopen failed from client perspective */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tcp_fastopen_client_fail {
    TFO_STATUS_UNSPEC = 0, /* catch-all */
    TFO_COOKIE_UNAVAILABLE = 1, /* if not in TFO_CLIENT_NO_COOKIE mode */
    TFO_DATA_NOT_ACKED = 2, /* SYN-ACK did not ack SYN data */
    TFO_SYN_RETRANSMITTED = 3, /* SYN-ACK did not ack SYN data after timeout */
}

/* for TCP_INFO socket option */
pub const TCPI_OPT_TIMESTAMPS: i32 = 1;
pub const TCPI_OPT_SACK: i32 = 2;
pub const TCPI_OPT_WSCALE: i32 = 4;
pub const TCPI_OPT_ECN: i32 = 8; /* ECN was negociated at TCP session init */
pub const TCPI_OPT_ECN_SEEN: i32 = 16; /* we received at least one packet with ECT */
pub const TCPI_OPT_SYN_DATA: i32 = 32; /* SYN-ACK acked data in SYN sent or rcvd */

/*
 * Sender's congestion state indicating normal or abnormal situations
 * in the last round of packets sent. The state is driven by the ACK
 * information and timer events.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tcp_ca_state {
    /*
     * Nothing bad has been observed recently.
     * No apparent reordering, packet loss, or ECN marks.
     */
    TCP_CA_Open = 0,
    /*
     * The sender enters disordered state when it has received DUPACKs or
     * SACKs in the last round of packets sent. This could be due to packet
     * loss or reordering but needs further information to confirm packets
     * have been lost.
     */
    TCP_CA_Disorder = 1,
    /*
     * The sender enters Congestion Window Reduction (CWR) state when it
     * has received ACKs with ECN-ECE marks, or has experienced congestion
     * or packet discard on the sender host (e.g. qdisc).
     */
    TCP_CA_CWR = 2,
    /*
     * The sender is in fast recovery and retransmitting lost packets,
     * typically triggered by ACK events.
     */
    TCP_CA_Recovery = 3,
    /*
     * The sender is in loss recovery triggered by retransmission timeout.
     */
    TCP_CA_Loss = 4,
}

pub const TCPF_CA_Open: i32 = 1 << tcp_ca_state::TCP_CA_Open as i32;
pub const TCPF_CA_Disorder: i32 = 1 << tcp_ca_state::TCP_CA_Disorder as i32;
pub const TCPF_CA_CWR: i32 = 1 << tcp_ca_state::TCP_CA_CWR as i32;
pub const TCPF_CA_Recovery: i32 = 1 << tcp_ca_state::TCP_CA_Recovery as i32;
pub const TCPF_CA_Loss: i32 = 1 << tcp_ca_state::TCP_CA_Loss as i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_info {
    pub tcpi_state: __u8,
    pub tcpi_ca_state: __u8,
    pub tcpi_retransmits: __u8,
    pub tcpi_probes: __u8,
    pub tcpi_backoff: __u8,
    pub tcpi_options: __u8,
    /* __u8 tcpi_snd_wscale : 4, tcpi_rcv_wscale : 4; */
    pub bitfield_1: __u8,
    /* __u8 tcpi_delivery_rate_app_limited:1, tcpi_fastopen_client_fail:2; */
    pub bitfield_2: __u8,

    pub tcpi_rto: __u32,
    pub tcpi_ato: __u32,
    pub tcpi_snd_mss: __u32,
    pub tcpi_rcv_mss: __u32,

    pub tcpi_unacked: __u32,
    pub tcpi_sacked: __u32,
    pub tcpi_lost: __u32,
    pub tcpi_retrans: __u32,
    pub tcpi_fackets: __u32,

    /* Times. */
    pub tcpi_last_data_sent: __u32,
    pub tcpi_last_ack_sent: __u32, /* Not remembered, sorry. */
    pub tcpi_last_data_recv: __u32,
    pub tcpi_last_ack_recv: __u32,

    /* Metrics. */
    pub tcpi_pmtu: __u32,
    pub tcpi_rcv_ssthresh: __u32,
    pub tcpi_rtt: __u32,
    pub tcpi_rttvar: __u32,
    pub tcpi_snd_ssthresh: __u32,
    pub tcpi_snd_cwnd: __u32,
    pub tcpi_advmss: __u32,
    pub tcpi_reordering: __u32,

    pub tcpi_rcv_rtt: __u32,
    pub tcpi_rcv_space: __u32,

    pub tcpi_total_retrans: __u32,

    pub tcpi_pacing_rate: __u64,
    pub tcpi_max_pacing_rate: __u64,
    pub tcpi_bytes_acked: __u64, /* RFC4898 tcpEStatsAppHCThruOctetsAcked */
    pub tcpi_bytes_received: __u64, /* RFC4898 tcpEStatsAppHCThruOctetsReceived */
    pub tcpi_segs_out: __u32, /* RFC4898 tcpEStatsPerfSegsOut */
    pub tcpi_segs_in: __u32, /* RFC4898 tcpEStatsPerfSegsIn */

    pub tcpi_notsent_bytes: __u32,
    pub tcpi_min_rtt: __u32,
    pub tcpi_data_segs_in: __u32, /* RFC4898 tcpEStatsDataSegsIn */
    pub tcpi_data_segs_out: __u32, /* RFC4898 tcpEStatsDataSegsOut */

    pub tcpi_delivery_rate: __u64,

    pub tcpi_busy_time: __u64, /* Time (usec) busy sending data */
    pub tcpi_rwnd_limited: __u64, /* Time (usec) limited by receive window */
    pub tcpi_sndbuf_limited: __u64, /* Time (usec) limited by send buffer */

    pub tcpi_delivered: __u32,
    pub tcpi_delivered_ce: __u32,

    pub tcpi_bytes_sent: __u64, /* RFC4898 tcpEStatsPerfHCDataOctetsOut */
    pub tcpi_bytes_retrans: __u64, /* RFC4898 tcpEStatsPerfOctetsRetrans */
    pub tcpi_dsack_dups: __u32, /* RFC4898 tcpEStatsStackDSACKDups */
    pub tcpi_reord_seen: __u32, /* reordering events seen */

    pub tcpi_rcv_ooopack: __u32, /* Out-of-order packets received */

    pub tcpi_snd_wnd: __u32, /* peer's advertised receive window after
                              * scaling (bytes)
                              */
}

impl tcp_info {
    #[cfg(target_endian = "little")]
    pub fn tcpi_snd_wscale(&self) -> __u8 {
        self.bitfield_1 & 0x0f
    }

    #[cfg(target_endian = "little")]
    pub fn set_tcpi_snd_wscale(&mut self, value: __u8) {
        self.bitfield_1 = (self.bitfield_1 & !0x0f) | (value & 0x0f);
    }

    #[cfg(target_endian = "little")]
    pub fn tcpi_rcv_wscale(&self) -> __u8 {
        (self.bitfield_1 >> 4) & 0x0f
    }

    #[cfg(target_endian = "little")]
    pub fn set_tcpi_rcv_wscale(&mut self, value: __u8) {
        self.bitfield_1 = (self.bitfield_1 & !0xf0) | ((value & 0x0f) << 4);
    }

    #[cfg(target_endian = "big")]
    pub fn tcpi_snd_wscale(&self) -> __u8 {
        (self.bitfield_1 >> 4) & 0x0f
    }

    #[cfg(target_endian = "big")]
    pub fn set_tcpi_snd_wscale(&mut self, value: __u8) {
        self.bitfield_1 = (self.bitfield_1 & !0xf0) | ((value & 0x0f) << 4);
    }

    #[cfg(target_endian = "big")]
    pub fn tcpi_rcv_wscale(&self) -> __u8 {
        self.bitfield_1 & 0x0f
    }

    #[cfg(target_endian = "big")]
    pub fn set_tcpi_rcv_wscale(&mut self, value: __u8) {
        self.bitfield_1 = (self.bitfield_1 & !0x0f) | (value & 0x0f);
    }

    #[cfg(target_endian = "little")]
    pub fn tcpi_delivery_rate_app_limited(&self) -> __u8 {
        self.bitfield_2 & 0x01
    }

    #[cfg(target_endian = "little")]
    pub fn set_tcpi_delivery_rate_app_limited(&mut self, value: __u8) {
        self.bitfield_2 = (self.bitfield_2 & !0x01) | (value & 0x01);
    }

    #[cfg(target_endian = "little")]
    pub fn tcpi_fastopen_client_fail(&self) -> __u8 {
        (self.bitfield_2 >> 1) & 0x03
    }

    #[cfg(target_endian = "little")]
    pub fn set_tcpi_fastopen_client_fail(&mut self, value: __u8) {
        self.bitfield_2 = (self.bitfield_2 & !0x06) | ((value & 0x03) << 1);
    }

    #[cfg(target_endian = "big")]
    pub fn tcpi_delivery_rate_app_limited(&self) -> __u8 {
        (self.bitfield_2 >> 7) & 0x01
    }

    #[cfg(target_endian = "big")]
    pub fn set_tcpi_delivery_rate_app_limited(&mut self, value: __u8) {
        self.bitfield_2 = (self.bitfield_2 & !0x80) | ((value & 0x01) << 7);
    }

    #[cfg(target_endian = "big")]
    pub fn tcpi_fastopen_client_fail(&self) -> __u8 {
        (self.bitfield_2 >> 5) & 0x03
    }

    #[cfg(target_endian = "big")]
    pub fn set_tcpi_fastopen_client_fail(&mut self, value: __u8) {
        self.bitfield_2 = (self.bitfield_2 & !0x60) | ((value & 0x03) << 5);
    }
}

/* netlink attributes types for SCM_TIMESTAMPING_OPT_STATS */
pub const TCP_NLA_PAD: i32 = 0;
pub const TCP_NLA_BUSY: i32 = 1; /* Time (usec) busy sending data */
pub const TCP_NLA_RWND_LIMITED: i32 = 2; /* Time (usec) limited by receive window */
pub const TCP_NLA_SNDBUF_LIMITED: i32 = 3; /* Time (usec) limited by send buffer */
pub const TCP_NLA_DATA_SEGS_OUT: i32 = 4; /* Data pkts sent including retransmission */
pub const TCP_NLA_TOTAL_RETRANS: i32 = 5; /* Data pkts retransmitted */
pub const TCP_NLA_PACING_RATE: i32 = 6; /* Pacing rate in bytes per second */
pub const TCP_NLA_DELIVERY_RATE: i32 = 7; /* Delivery rate in bytes per second */
pub const TCP_NLA_SND_CWND: i32 = 8; /* Sending congestion window */
pub const TCP_NLA_REORDERING: i32 = 9; /* Reordering metric */
pub const TCP_NLA_MIN_RTT: i32 = 10; /* minimum RTT */
pub const TCP_NLA_RECUR_RETRANS: i32 = 11; /* Recurring retransmits for the current pkt */
pub const TCP_NLA_DELIVERY_RATE_APP_LMT: i32 = 12; /* delivery rate application limited ? */
pub const TCP_NLA_SNDQ_SIZE: i32 = 13; /* Data (bytes) pending in send queue */
pub const TCP_NLA_CA_STATE: i32 = 14; /* ca_state of socket */
pub const TCP_NLA_SND_SSTHRESH: i32 = 15; /* Slow start size threshold */
pub const TCP_NLA_DELIVERED: i32 = 16; /* Data pkts delivered incl. out-of-order */
pub const TCP_NLA_DELIVERED_CE: i32 = 17; /* Like above but only ones w/ CE marks */
pub const TCP_NLA_BYTES_SENT: i32 = 18; /* Data bytes sent including retransmission */
pub const TCP_NLA_BYTES_RETRANS: i32 = 19; /* Data bytes retransmitted */
pub const TCP_NLA_DSACK_DUPS: i32 = 20; /* DSACK blocks received */
pub const TCP_NLA_REORD_SEEN: i32 = 21; /* reordering events seen */
pub const TCP_NLA_SRTT: i32 = 22; /* smoothed RTT in usecs */
pub const TCP_NLA_TIMEOUT_REHASH: i32 = 23; /* Timeout-triggered rehash attempts */
pub const TCP_NLA_BYTES_NOTSENT: i32 = 24; /* Bytes in write queue not yet sent */
pub const TCP_NLA_EDT: i32 = 25; /* Earliest departure time (CLOCK_MONOTONIC) */

/* for TCP_MD5SIG socket option */
pub const TCP_MD5SIG_MAXKEYLEN: usize = 80;

/* tcp_md5sig extension flags for TCP_MD5SIG_EXT */
pub const TCP_MD5SIG_FLAG_PREFIX: i32 = 0x1; /* address prefix length */
pub const TCP_MD5SIG_FLAG_IFINDEX: i32 = 0x2; /* ifindex set */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_md5sig {
    pub tcpm_addr: __kernel_sockaddr_storage, /* address associated */
    pub tcpm_flags: __u8, /* extension flags */
    pub tcpm_prefixlen: __u8, /* address prefix */
    pub tcpm_keylen: __u16, /* key length */
    pub tcpm_ifindex: ::core::ffi::c_int, /* device index for scope */
    pub tcpm_key: [__u8; TCP_MD5SIG_MAXKEYLEN], /* key (binary) */
}

/* INET_DIAG_MD5SIG */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_diag_md5sig {
    pub tcpm_family: __u8,
    pub tcpm_prefixlen: __u8,
    pub tcpm_keylen: __u16,
    pub tcpm_addr: [__be32; 4],
    pub tcpm_key: [__u8; TCP_MD5SIG_MAXKEYLEN],
}

/* setsockopt(fd, IPPROTO_TCP, TCP_ZEROCOPY_RECEIVE, ...) */

pub const TCP_RECEIVE_ZEROCOPY_FLAG_TLB_CLEAN_HINT: i32 = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_zerocopy_receive {
    pub address: __u64, /* in: address of mapping */
    pub length: __u32, /* in/out: number of bytes to map/mapped */
    pub recv_skip_hint: __u32, /* out: amount of bytes to skip */
    pub inq: __u32, /* out: amount of bytes in read queue */
    pub err: __s32, /* out: socket error */
    pub copybuf_address: __u64, /* in: copybuf address (small reads) */
    pub copybuf_len: __s32, /* in/out: copybuf bytes avail/used or error */
    pub flags: __u32, /* in: flags */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
