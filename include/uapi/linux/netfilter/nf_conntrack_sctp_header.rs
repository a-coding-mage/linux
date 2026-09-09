/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* SCTP tracking. */

// Dependency supplied by the surrounding UAPI translation:
// #include <linux/netfilter/nf_conntrack_tuple_common.h>

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum sctp_conntrack {
    SCTP_CONNTRACK_NONE = 0,
    SCTP_CONNTRACK_CLOSED,
    SCTP_CONNTRACK_COOKIE_WAIT,
    SCTP_CONNTRACK_COOKIE_ECHOED,
    SCTP_CONNTRACK_ESTABLISHED,
    SCTP_CONNTRACK_SHUTDOWN_SENT,
    SCTP_CONNTRACK_SHUTDOWN_RECD,
    SCTP_CONNTRACK_SHUTDOWN_ACK_SENT,
    SCTP_CONNTRACK_HEARTBEAT_SENT,
    SCTP_CONNTRACK_HEARTBEAT_ACKED, /* no longer used */
    SCTP_CONNTRACK_MAX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
