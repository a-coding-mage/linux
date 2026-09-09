/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SCTP kernel implementation
 * (C) Copyright IBM Corp. 2001, 2004
 * Copyright (c) 1999-2000 Cisco, Inc.
 * Copyright (c) 1999-2001 Motorola, Inc.
 * Copyright (c) 2001 Intel Corp.
 */

/* Dependencies supplied by the surrounding translation unit:
 * linux/sctp.h, linux/ipv6.h, and net/tcp_states.h.
 */

pub const SCTP_MAX_STREAM: u32 = 0xffff;
pub const SCTP_DEFAULT_OUTSTREAMS: u32 = 10;
pub const SCTP_DEFAULT_INSTREAMS: u32 = SCTP_MAX_STREAM;

pub const SCTP_CID_BASE_MAX: u32 = SCTP_CID_SHUTDOWN_COMPLETE as u32;
pub const SCTP_NUM_BASE_CHUNK_TYPES: u32 = SCTP_CID_BASE_MAX + 1;
pub const SCTP_NUM_ADDIP_CHUNK_TYPES: u32 = 2;
pub const SCTP_NUM_PRSCTP_CHUNK_TYPES: u32 = 1;
pub const SCTP_NUM_RECONF_CHUNK_TYPES: u32 = 1;
pub const SCTP_NUM_AUTH_CHUNK_TYPES: u32 = 1;
pub const SCTP_NUM_CHUNK_TYPES: u32 = SCTP_NUM_BASE_CHUNK_TYPES
    + SCTP_NUM_ADDIP_CHUNK_TYPES
    + SCTP_NUM_PRSCTP_CHUNK_TYPES
    + SCTP_NUM_RECONF_CHUNK_TYPES
    + SCTP_NUM_AUTH_CHUNK_TYPES;

#[repr(i32)]
pub enum sctp_event_type { SCTP_EVENT_T_CHUNK = 1, SCTP_EVENT_T_TIMEOUT, SCTP_EVENT_T_OTHER, SCTP_EVENT_T_PRIMITIVE }

#[repr(i32)]
pub enum sctp_event_timeout {
    SCTP_EVENT_TIMEOUT_NONE = 0, SCTP_EVENT_TIMEOUT_T1_COOKIE, SCTP_EVENT_TIMEOUT_T1_INIT,
    SCTP_EVENT_TIMEOUT_T2_SHUTDOWN, SCTP_EVENT_TIMEOUT_T3_RTX, SCTP_EVENT_TIMEOUT_T4_RTO,
    SCTP_EVENT_TIMEOUT_T5_SHUTDOWN_GUARD, SCTP_EVENT_TIMEOUT_HEARTBEAT, SCTP_EVENT_TIMEOUT_RECONF,
    SCTP_EVENT_TIMEOUT_PROBE, SCTP_EVENT_TIMEOUT_SACK, SCTP_EVENT_TIMEOUT_AUTOCLOSE,
}
pub const SCTP_EVENT_TIMEOUT_MAX: sctp_event_timeout = sctp_event_timeout::SCTP_EVENT_TIMEOUT_AUTOCLOSE;
pub const SCTP_NUM_TIMEOUT_TYPES: i32 = SCTP_EVENT_TIMEOUT_MAX as i32 + 1;

#[repr(i32)]
pub enum sctp_event_other { SCTP_EVENT_NO_PENDING_TSN = 0, SCTP_EVENT_ICMP_PROTO_UNREACH }
pub const SCTP_EVENT_OTHER_MAX: sctp_event_other = sctp_event_other::SCTP_EVENT_ICMP_PROTO_UNREACH;
pub const SCTP_NUM_OTHER_TYPES: i32 = SCTP_EVENT_OTHER_MAX as i32 + 1;

#[repr(i32)]
pub enum sctp_event_primitive {
    SCTP_PRIMITIVE_ASSOCIATE = 0, SCTP_PRIMITIVE_SHUTDOWN, SCTP_PRIMITIVE_ABORT,
    SCTP_PRIMITIVE_SEND, SCTP_PRIMITIVE_REQUESTHEARTBEAT, SCTP_PRIMITIVE_ASCONF,
    SCTP_PRIMITIVE_RECONF,
}
pub const SCTP_EVENT_PRIMITIVE_MAX: sctp_event_primitive = sctp_event_primitive::SCTP_PRIMITIVE_RECONF;
pub const SCTP_NUM_PRIMITIVE_TYPES: i32 = SCTP_EVENT_PRIMITIVE_MAX as i32 + 1;

#[repr(C)]
pub union sctp_subtype {
    pub chunk: sctp_cid,
    pub timeout: sctp_event_timeout,
    pub other: sctp_event_other,
    pub primitive: sctp_event_primitive,
}

#[inline]
pub const fn SCTP_ST_CHUNK(arg: sctp_cid) -> sctp_subtype { sctp_subtype { chunk: arg } }
#[inline]
pub const fn SCTP_ST_TIMEOUT(arg: sctp_event_timeout) -> sctp_subtype { sctp_subtype { timeout: arg } }
#[inline]
pub const fn SCTP_ST_OTHER(arg: sctp_event_other) -> sctp_subtype { sctp_subtype { other: arg } }
#[inline]
pub const fn SCTP_ST_PRIMITIVE(arg: sctp_event_primitive) -> sctp_subtype { sctp_subtype { primitive: arg } }

/* Equivalent to: a->chunk_hdr->type == SCTP_CID_DATA || a->chunk_hdr->type == SCTP_CID_I_DATA. */
#[inline]
pub unsafe fn sctp_chunk_is_data<T>(a: *const T) -> bool {
    let _ = a;
    /* The concrete sctp_chunk dependency supplies chunk_hdr and its type. */
    todo!("requires the external sctp_chunk definition")
}

#[repr(i32)]
pub enum sctp_ierror {
    SCTP_IERROR_NO_ERROR = 0, SCTP_IERROR_BASE = 1000, SCTP_IERROR_NO_COOKIE,
    SCTP_IERROR_BAD_SIG, SCTP_IERROR_STALE_COOKIE, SCTP_IERROR_NOMEM, SCTP_IERROR_MALFORMED,
    SCTP_IERROR_BAD_TAG, SCTP_IERROR_BIG_GAP, SCTP_IERROR_DUP_TSN, SCTP_IERROR_HIGH_TSN,
    SCTP_IERROR_IGNORE_TSN, SCTP_IERROR_NO_DATA, SCTP_IERROR_BAD_STREAM, SCTP_IERROR_BAD_PORTS,
    SCTP_IERROR_AUTH_BAD_HMAC, SCTP_IERROR_AUTH_BAD_KEYID, SCTP_IERROR_PROTO_VIOLATION,
    SCTP_IERROR_ERROR, SCTP_IERROR_ABORT,
}

#[repr(i32)]
pub enum sctp_state {
    SCTP_STATE_CLOSED = 0, SCTP_STATE_COOKIE_WAIT, SCTP_STATE_COOKIE_ECHOED, SCTP_STATE_ESTABLISHED,
    SCTP_STATE_SHUTDOWN_PENDING, SCTP_STATE_SHUTDOWN_SENT, SCTP_STATE_SHUTDOWN_RECEIVED,
    SCTP_STATE_SHUTDOWN_ACK_SENT,
}
pub const SCTP_STATE_MAX: sctp_state = sctp_state::SCTP_STATE_SHUTDOWN_ACK_SENT;
pub const SCTP_STATE_NUM_STATES: i32 = SCTP_STATE_MAX as i32 + 1;

#[repr(i32)]
pub enum sctp_sock_state {
    SCTP_SS_CLOSED = TCP_CLOSE, SCTP_SS_LISTENING = TCP_LISTEN, SCTP_SS_ESTABLISHING = TCP_SYN_SENT,
    SCTP_SS_ESTABLISHED = TCP_ESTABLISHED, SCTP_SS_CLOSING = TCP_CLOSE_WAIT,
}
#[repr(i32)]
pub enum sctp_plpmtud_state { SCTP_PL_DISABLED, SCTP_PL_BASE, SCTP_PL_SEARCH, SCTP_PL_COMPLETE, SCTP_PL_ERROR }

pub const SCTP_BASE_PLPMTU: u32 = 1200;
pub const SCTP_MAX_PLPMTU: u32 = 9000;
pub const SCTP_MIN_PLPMTU: u32 = 512;
pub const SCTP_MAX_PROBES: u32 = 3;
pub const SCTP_PL_BIG_STEP: u32 = 32;
pub const SCTP_PL_MIN_STEP: u32 = 4;

extern "C" {
    pub fn sctp_cname(id: sctp_subtype) -> *const i8;
    pub fn sctp_oname(id: sctp_subtype) -> *const i8;
    pub fn sctp_tname(id: sctp_subtype) -> *const i8;
    pub fn sctp_pname(id: sctp_subtype) -> *const i8;
    pub static sctp_state_tbl: *const *const i8;
    pub static sctp_evttype_tbl: *const *const i8;
    pub static sctp_status_tbl: *const *const i8;
}

pub const SCTP_MAX_CHUNK_LEN: usize = ((1usize << 16) - core::mem::size_of::<u32>());
pub const SCTP_ARBITRARY_COOKIE_ECHO_LEN: u32 = 200;
pub const SCTP_TSN_MAP_INITIAL: usize = BITS_PER_LONG;
pub const SCTP_TSN_MAP_INCREMENT: usize = SCTP_TSN_MAP_INITIAL;
pub const SCTP_TSN_MAP_SIZE: usize = 4096;
pub const SCTP_MAX_DUP_TSNS: u32 = 16;
pub const SCTP_MAX_GABS: u32 = 16;
pub const SCTP_DEFAULT_TIMEOUT_HEARTBEAT: u32 = 30 * 1000;
pub const SCTP_DEFAULT_TIMEOUT_SACK: u32 = 200;
pub const SCTP_RTO_INITIAL: u32 = 3 * 1000;
pub const SCTP_RTO_MIN: u32 = 1 * 1000;
pub const SCTP_RTO_MAX: u32 = 60 * 1000;
pub const SCTP_RTO_ALPHA: u32 = 3;
pub const SCTP_RTO_BETA: u32 = 2;
pub const SCTP_DEFAULT_MAX_BURST: u32 = 4;
pub const SCTP_CLOCK_GRANULARITY: u32 = 1;
pub const SCTP_DEFAULT_COOKIE_LIFE: u32 = 60 * 1000;
pub const SCTP_DEFAULT_MINWINDOW: u32 = 1500;
pub const SCTP_DEFAULT_MAXWINDOW: u32 = 65535;
pub const SCTP_DEFAULT_RWND_SHIFT: u32 = 4;
pub const SCTP_DEFAULT_MAXSEGMENT: u32 = 1500;
pub const SCTP_DEFAULT_MINSEGMENT: u32 = 512;
pub const SCTP_COOKIE_KEY_SIZE: u32 = 32;
pub const SCTP_COOKIE_MAC_SIZE: u32 = 32;
pub const SCTP_COOKIE_MULTIPLE: u32 = 32;
pub const SCTP_DEFAULT_UDP_PORT: u32 = 9899;

#[repr(i32)]
pub enum sctp_pf_expose { SCTP_PF_EXPOSE_UNSET, SCTP_PF_EXPOSE_DISABLE, SCTP_PF_EXPOSE_ENABLE }
pub const SCTP_PF_EXPOSE_MAX: sctp_pf_expose = sctp_pf_expose::SCTP_PF_EXPOSE_ENABLE;
pub const SCTP_PS_RETRANS_MAX: u32 = 0xffff;

#[repr(i32)]
pub enum sctp_xmit { SCTP_XMIT_OK, SCTP_XMIT_PMTU_FULL, SCTP_XMIT_RWND_FULL, SCTP_XMIT_DELAY }
#[repr(i32)]
pub enum sctp_transport_cmd { SCTP_TRANSPORT_UP, SCTP_TRANSPORT_DOWN, SCTP_TRANSPORT_PF }
#[repr(i32)]
pub enum sctp_scope { SCTP_SCOPE_GLOBAL, SCTP_SCOPE_PRIVATE, SCTP_SCOPE_LINK, SCTP_SCOPE_LOOPBACK, SCTP_SCOPE_UNUSABLE }
pub const SCTP_SCOPE_POLICY_DISABLE: i32 = 0;
pub const SCTP_SCOPE_POLICY_ENABLE: i32 = 1;
pub const SCTP_SCOPE_POLICY_PRIVATE: i32 = 2;
pub const SCTP_SCOPE_POLICY_LINK: i32 = 3;
pub const SCTP_SCOPE_POLICY_MAX: i32 = SCTP_SCOPE_POLICY_LINK;

/* External dependency mapping retained from the C macro. */
#[inline]
pub unsafe fn IS_IPV4_UNUSABLE_ADDRESS(a: u32) -> bool {
    (htonl(INADDR_BROADCAST) == a) || ipv4_is_multicast(a) || ipv4_is_zeronet(a) || ipv4_is_anycast_6to4(a)
}

pub const SCTP_ADDR4_ALLOWED: u32 = 0x00000001;
pub const SCTP_ADDR6_ALLOWED: u32 = 0x00000002;
pub const SCTP_ADDR4_PEERSUPP: u32 = 0x00000004;
pub const SCTP_ADDR6_PEERSUPP: u32 = 0x00000008;
#[repr(i32)]
pub enum sctp_retransmit_reason { SCTP_RTXR_T3_RTX, SCTP_RTXR_FAST_RTX, SCTP_RTXR_PMTUD, SCTP_RTXR_T1_RTX }
#[repr(i32)]
pub enum sctp_lower_cwnd { SCTP_LOWER_CWND_T3_RTX, SCTP_LOWER_CWND_FAST_RTX, SCTP_LOWER_CWND_ECNE, SCTP_LOWER_CWND_INACTIVE }

pub const SCTP_AUTH_HMAC_ID_RESERVED_0: i32 = 0;
pub const SCTP_AUTH_HMAC_ID_SHA1: i32 = 1;
pub const SCTP_AUTH_HMAC_ID_RESERVED_2: i32 = 2;
pub const SCTP_AUTH_HMAC_ID_SHA256: i32 = 3;
pub const __SCTP_AUTH_HMAC_MAX: i32 = 4;
pub const SCTP_AUTH_HMAC_ID_MAX: i32 = __SCTP_AUTH_HMAC_MAX - 1;
pub const SCTP_AUTH_NUM_HMACS: i32 = __SCTP_AUTH_HMAC_MAX;
pub const SCTP_NUM_NOAUTH_CHUNKS: u32 = 4;
pub const SCTP_AUTH_MAX_CHUNKS: u32 = SCTP_NUM_CHUNK_TYPES - SCTP_NUM_NOAUTH_CHUNKS;
pub const SCTP_AUTH_RANDOM_LENGTH: u32 = 32;
pub const SCTP_PROBE_TIMER_MIN: u32 = 5000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
