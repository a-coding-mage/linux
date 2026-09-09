/* SPDX-License-Identifier: GPL-2.0 */
// Translation of trace/events/sock.h.
// C preprocessor conditions and included kernel definitions are supplied by
// the surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* family_names */
pub const FAMILY_NAMES: &[(u32, &str)] = &[
    (AF_INET, "AF_INET"),
    (AF_INET6, "AF_INET6"),
];

/* The protocol traced by inet_sock_set_state. */
pub const INET_PROTOCOL_NAMES: &[(u32, &str)] = &[
    (IPPROTO_TCP, "IPPROTO_TCP"),
    (IPPROTO_SCTP, "IPPROTO_SCTP"),
    (IPPROTO_MPTCP, "IPPROTO_MPTCP"),
];

pub const TCP_STATE_NAMES: &[(u32, &str)] = &[
    (TCP_ESTABLISHED, "TCP_ESTABLISHED"),
    (TCP_SYN_SENT, "TCP_SYN_SENT"),
    (TCP_SYN_RECV, "TCP_SYN_RECV"),
    (TCP_FIN_WAIT1, "TCP_FIN_WAIT1"),
    (TCP_FIN_WAIT2, "TCP_FIN_WAIT2"),
    (TCP_TIME_WAIT, "TCP_TIME_WAIT"),
    (TCP_CLOSE, "TCP_CLOSE"),
    (TCP_CLOSE_WAIT, "TCP_CLOSE_WAIT"),
    (TCP_LAST_ACK, "TCP_LAST_ACK"),
    (TCP_LISTEN, "TCP_LISTEN"),
    (TCP_CLOSING, "TCP_CLOSING"),
    (TCP_NEW_SYN_RECV, "TCP_NEW_SYN_RECV"),
];

pub const SKMEM_KIND_NAMES: &[(u32, &str)] = &[
    (SK_MEM_SEND, "SK_MEM_SEND"),
    (SK_MEM_RECV, "SK_MEM_RECV"),
];

#[repr(C)]
pub struct SockRcvqueueFullEntry {
    pub rmem_alloc: i32,
    pub truesize: u32,
    pub sk_rcvbuf: i32,
}

#[repr(C)]
pub struct SockExceedBufLimitEntry {
    pub name: [u8; 32],
    pub sysctl_mem: [c_long; 3],
    pub allocated: c_long,
    pub sysctl_rmem: i32,
    pub rmem_alloc: i32,
    pub sysctl_wmem: i32,
    pub wmem_alloc: i32,
    pub wmem_queued: i32,
    pub kind: i32,
}

#[repr(C)]
pub struct InetSockSetStateEntry {
    pub skaddr: *const c_void,
    pub oldstate: i32,
    pub newstate: i32,
    pub sport: u16,
    pub dport: u16,
    pub family: u16,
    pub protocol: u16,
    pub saddr: [u8; 4],
    pub daddr: [u8; 4],
    pub saddr_v6: [u8; 16],
    pub daddr_v6: [u8; 16],
}

#[repr(C)]
pub struct InetSkErrorReportEntry {
    pub error: i32,
    pub sport: u16,
    pub dport: u16,
    pub family: u16,
    pub protocol: u16,
    pub saddr: [u8; 4],
    pub daddr: [u8; 4],
    pub saddr_v6: [u8; 16],
    pub daddr_v6: [u8; 16],
}

#[repr(C)]
pub struct SkDataReadyEntry {
    pub skaddr: *const c_void,
    pub family: u16,
    pub protocol: u16,
    pub ip: c_ulong,
}

#[repr(C)]
pub struct SockMsgLengthEntry {
    pub sk: *mut c_void,
    pub family: u16,
    pub protocol: u16,
    pub ret: i32,
    pub flags: i32,
}

pub type c_void = core::ffi::c_void;
pub type c_long = isize;
pub type c_ulong = usize;

/*
 * Trace-event declarations retained as Rust metadata.  The fast assignments
 * and TP_printk formats are intentionally preserved verbatim in these notes;
 * their operations depend on the kernel types and helpers supplied by the
 * included headers (sock, skb, inet_sock, TP_STORE_ADDRS, and tracepoint).
 *
 * TRACE_EVENT(sock_rcvqueue_full, SockRcvqueueFullEntry)
 * TRACE_EVENT(sock_exceed_buf_limit, SockExceedBufLimitEntry)
 * TRACE_EVENT(inet_sock_set_state, InetSockSetStateEntry)
 * TRACE_EVENT(inet_sk_error_report, InetSkErrorReportEntry)
 * TRACE_EVENT(sk_data_ready, SkDataReadyEntry)
 * DECLARE_EVENT_CLASS(sock_msg_length, SockMsgLengthEntry)
 * DEFINE_EVENT(sock_msg_length, sock_send_length)
 * DEFINE_EVENT(sock_msg_length, sock_recv_length)
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
