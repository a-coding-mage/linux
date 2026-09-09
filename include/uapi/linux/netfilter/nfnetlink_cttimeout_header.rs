/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// C dependency: <linux/netfilter/nfnetlink.h>

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ctnl_timeout_msg_types {
    IPCTNL_MSG_TIMEOUT_NEW,
    IPCTNL_MSG_TIMEOUT_GET,
    IPCTNL_MSG_TIMEOUT_DELETE,
    IPCTNL_MSG_TIMEOUT_DEFAULT_SET,
    IPCTNL_MSG_TIMEOUT_DEFAULT_GET,
    IPCTNL_MSG_TIMEOUT_MAX,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout {
    CTA_TIMEOUT_UNSPEC,
    CTA_TIMEOUT_NAME,
    CTA_TIMEOUT_L3PROTO,
    CTA_TIMEOUT_L4PROTO,
    CTA_TIMEOUT_DATA,
    CTA_TIMEOUT_USE,
    __CTA_TIMEOUT_MAX,
}
pub const CTA_TIMEOUT_MAX: i32 = __CTA_TIMEOUT_MAX as i32 - 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_generic {
    CTA_TIMEOUT_GENERIC_UNSPEC,
    CTA_TIMEOUT_GENERIC_TIMEOUT,
    __CTA_TIMEOUT_GENERIC_MAX,
}
pub const CTA_TIMEOUT_GENERIC_MAX: i32 = __CTA_TIMEOUT_GENERIC_MAX as i32 - 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_tcp {
    CTA_TIMEOUT_TCP_UNSPEC,
    CTA_TIMEOUT_TCP_SYN_SENT,
    CTA_TIMEOUT_TCP_SYN_RECV,
    CTA_TIMEOUT_TCP_ESTABLISHED,
    CTA_TIMEOUT_TCP_FIN_WAIT,
    CTA_TIMEOUT_TCP_CLOSE_WAIT,
    CTA_TIMEOUT_TCP_LAST_ACK,
    CTA_TIMEOUT_TCP_TIME_WAIT,
    CTA_TIMEOUT_TCP_CLOSE,
    CTA_TIMEOUT_TCP_SYN_SENT2,
    CTA_TIMEOUT_TCP_RETRANS,
    CTA_TIMEOUT_TCP_UNACK,
    __CTA_TIMEOUT_TCP_MAX,
}
pub const CTA_TIMEOUT_TCP_MAX: i32 = __CTA_TIMEOUT_TCP_MAX as i32 - 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_udp {
    CTA_TIMEOUT_UDP_UNSPEC,
    CTA_TIMEOUT_UDP_UNREPLIED,
    CTA_TIMEOUT_UDP_REPLIED,
    __CTA_TIMEOUT_UDP_MAX,
}
pub const CTA_TIMEOUT_UDP_MAX: i32 = __CTA_TIMEOUT_UDP_MAX as i32 - 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_udplite {
    CTA_TIMEOUT_UDPLITE_UNSPEC,
    CTA_TIMEOUT_UDPLITE_UNREPLIED,
    CTA_TIMEOUT_UDPLITE_REPLIED,
    __CTA_TIMEOUT_UDPLITE_MAX,
}
pub const CTA_TIMEOUT_UDPLITE_MAX: i32 = __CTA_TIMEOUT_UDPLITE_MAX as i32 - 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_icmp {
    CTA_TIMEOUT_ICMP_UNSPEC,
    CTA_TIMEOUT_ICMP_TIMEOUT,
    __CTA_TIMEOUT_ICMP_MAX,
}
pub const CTA_TIMEOUT_ICMP_MAX: i32 = __CTA_TIMEOUT_ICMP_MAX as i32 - 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_dccp {
    CTA_TIMEOUT_DCCP_UNSPEC,
    CTA_TIMEOUT_DCCP_REQUEST,
    CTA_TIMEOUT_DCCP_RESPOND,
    CTA_TIMEOUT_DCCP_PARTOPEN,
    CTA_TIMEOUT_DCCP_OPEN,
    CTA_TIMEOUT_DCCP_CLOSEREQ,
    CTA_TIMEOUT_DCCP_CLOSING,
    CTA_TIMEOUT_DCCP_TIMEWAIT,
    __CTA_TIMEOUT_DCCP_MAX,
}
pub const CTA_TIMEOUT_DCCP_MAX: i32 = __CTA_TIMEOUT_DCCP_MAX as i32 - 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_sctp {
    CTA_TIMEOUT_SCTP_UNSPEC,
    CTA_TIMEOUT_SCTP_CLOSED,
    CTA_TIMEOUT_SCTP_COOKIE_WAIT,
    CTA_TIMEOUT_SCTP_COOKIE_ECHOED,
    CTA_TIMEOUT_SCTP_ESTABLISHED,
    CTA_TIMEOUT_SCTP_SHUTDOWN_SENT,
    CTA_TIMEOUT_SCTP_SHUTDOWN_RECD,
    CTA_TIMEOUT_SCTP_SHUTDOWN_ACK_SENT,
    CTA_TIMEOUT_SCTP_HEARTBEAT_SENT,
    CTA_TIMEOUT_SCTP_HEARTBEAT_ACKED, // no longer used
    __CTA_TIMEOUT_SCTP_MAX,
}
pub const CTA_TIMEOUT_SCTP_MAX: i32 = __CTA_TIMEOUT_SCTP_MAX as i32 - 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_icmpv6 {
    CTA_TIMEOUT_ICMPV6_UNSPEC,
    CTA_TIMEOUT_ICMPV6_TIMEOUT,
    __CTA_TIMEOUT_ICMPV6_MAX,
}
pub const CTA_TIMEOUT_ICMPV6_MAX: i32 = __CTA_TIMEOUT_ICMPV6_MAX as i32 - 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_gre {
    CTA_TIMEOUT_GRE_UNSPEC,
    CTA_TIMEOUT_GRE_UNREPLIED,
    CTA_TIMEOUT_GRE_REPLIED,
    __CTA_TIMEOUT_GRE_MAX,
}
pub const CTA_TIMEOUT_GRE_MAX: i32 = __CTA_TIMEOUT_GRE_MAX as i32 - 1;

pub const CTNL_TIMEOUT_NAME_MAX: i32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
