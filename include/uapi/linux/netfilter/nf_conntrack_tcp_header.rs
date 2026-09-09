/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* TCP tracking. */

/* Corresponds to <linux/types.h>. */

/* This is exposed to userspace (ctnetlink) */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tcp_conntrack {
    TCP_CONNTRACK_NONE,
    TCP_CONNTRACK_SYN_SENT,
    TCP_CONNTRACK_SYN_RECV,
    TCP_CONNTRACK_ESTABLISHED,
    TCP_CONNTRACK_FIN_WAIT,
    TCP_CONNTRACK_CLOSE_WAIT,
    TCP_CONNTRACK_LAST_ACK,
    TCP_CONNTRACK_TIME_WAIT,
    TCP_CONNTRACK_CLOSE,
    TCP_CONNTRACK_LISTEN, /* obsolete */
    TCP_CONNTRACK_MAX,
    TCP_CONNTRACK_IGNORE,
    TCP_CONNTRACK_RETRANS,
    TCP_CONNTRACK_UNACK,
    TCP_CONNTRACK_TIMEOUT_MAX,
}

pub const TCP_CONNTRACK_SYN_SENT2: tcp_conntrack = tcp_conntrack::TCP_CONNTRACK_LISTEN;

/* Window scaling is advertised by the sender */
pub const IP_CT_TCP_FLAG_WINDOW_SCALE: u8 = 0x01;

/* SACK is permitted by the sender */
pub const IP_CT_TCP_FLAG_SACK_PERM: u8 = 0x02;

/* This sender sent FIN first */
pub const IP_CT_TCP_FLAG_CLOSE_INIT: u8 = 0x04;

/* Be liberal in window checking */
pub const IP_CT_TCP_FLAG_BE_LIBERAL: u8 = 0x08;

/* Has unacknowledged data */
pub const IP_CT_TCP_FLAG_DATA_UNACKNOWLEDGED: u8 = 0x10;

/* The field td_maxack has been set */
pub const IP_CT_TCP_FLAG_MAXACK_SET: u8 = 0x20;

/* Marks possibility for expected RFC5961 challenge ACK */
pub const IP_CT_EXP_CHALLENGE_ACK: u8 = 0x40;

/* Simultaneous open initialized */
pub const IP_CT_TCP_SIMULTANEOUS_OPEN: u8 = 0x80;

#[repr(C)]
pub struct nf_ct_tcp_flags {
    pub flags: u8,
    pub mask: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
