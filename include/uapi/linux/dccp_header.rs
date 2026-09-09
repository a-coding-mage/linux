/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from the Linux UAPI DCCP header. */

/** Generic part of a DCCP packet header. */
#[repr(C)]
pub struct dccp_hdr {
    pub dccph_sport: __be16,
    pub dccph_dport: __be16,
    pub dccph_doff: __u8,
    /* Little endian: cscov:4, ccval:4; big endian: ccval:4, cscov:4. */
    pub dccph_cscov_ccval: __u8,
    pub dccph_checksum: __sum16,
    /* Little endian: x:1, type:4, reserved:3; big endian: reserved:3, type:4, x:1. */
    pub dccph_x_type_reserved: __u8,
    pub dccph_seq2: __u8,
    pub dccph_seq: __be16,
}

/** The low bits of a 48 bit sequence packet. */
#[repr(C)]
pub struct dccp_hdr_ext {
    pub dccph_seq_low: __be32,
}

/** Connection initiation request header. */
#[repr(C)]
pub struct dccp_hdr_request {
    pub dccph_req_service: __be32,
}

/** Acknowledgment bits common to most packets. */
#[repr(C)]
pub struct dccp_hdr_ack_bits {
    pub dccph_reserved1: __be16,
    pub dccph_ack_nr_high: __be16,
    pub dccph_ack_nr_low: __be32,
}

/** Connection initiation response header. */
#[repr(C)]
pub struct dccp_hdr_response {
    pub dccph_resp_ack: dccp_hdr_ack_bits,
    pub dccph_resp_service: __be32,
}

/** Unconditionally shut down a connection. */
#[repr(C)]
pub struct dccp_hdr_reset {
    pub dccph_reset_ack: dccp_hdr_ack_bits,
    pub dccph_reset_code: __u8,
    pub dccph_reset_data: [__u8; 3],
}

#[repr(i32)]
pub enum dccp_pkt_type {
    DCCP_PKT_REQUEST = 0,
    DCCP_PKT_RESPONSE,
    DCCP_PKT_DATA,
    DCCP_PKT_ACK,
    DCCP_PKT_DATAACK,
    DCCP_PKT_CLOSEREQ,
    DCCP_PKT_CLOSE,
    DCCP_PKT_RESET,
    DCCP_PKT_SYNC,
    DCCP_PKT_SYNCACK,
    DCCP_PKT_INVALID,
}

pub const DCCP_NR_PKT_TYPES: dccp_pkt_type = dccp_pkt_type::DCCP_PKT_INVALID;

pub unsafe fn dccp_packet_hdr_len(type_: __u8) -> usize {
    if type_ == dccp_pkt_type::DCCP_PKT_DATA as __u8 {
        return 0;
    }
    if type_ == dccp_pkt_type::DCCP_PKT_DATAACK as __u8
        || type_ == dccp_pkt_type::DCCP_PKT_ACK as __u8
        || type_ == dccp_pkt_type::DCCP_PKT_SYNC as __u8
        || type_ == dccp_pkt_type::DCCP_PKT_SYNCACK as __u8
        || type_ == dccp_pkt_type::DCCP_PKT_CLOSE as __u8
        || type_ == dccp_pkt_type::DCCP_PKT_CLOSEREQ as __u8
    {
        return core::mem::size_of::<dccp_hdr_ack_bits>();
    }
    if type_ == dccp_pkt_type::DCCP_PKT_REQUEST as __u8 {
        return core::mem::size_of::<dccp_hdr_request>();
    }
    if type_ == dccp_pkt_type::DCCP_PKT_RESPONSE as __u8 {
        return core::mem::size_of::<dccp_hdr_response>();
    }
    core::mem::size_of::<dccp_hdr_reset>()
}

#[repr(i32)]
pub enum dccp_reset_codes {
    DCCP_RESET_CODE_UNSPECIFIED = 0,
    DCCP_RESET_CODE_CLOSED,
    DCCP_RESET_CODE_ABORTED,
    DCCP_RESET_CODE_NO_CONNECTION,
    DCCP_RESET_CODE_PACKET_ERROR,
    DCCP_RESET_CODE_OPTION_ERROR,
    DCCP_RESET_CODE_MANDATORY_ERROR,
    DCCP_RESET_CODE_CONNECTION_REFUSED,
    DCCP_RESET_CODE_BAD_SERVICE_CODE,
    DCCP_RESET_CODE_TOO_BUSY,
    DCCP_RESET_CODE_BAD_INIT_COOKIE,
    DCCP_RESET_CODE_AGGRESSION_PENALTY,
    DCCP_MAX_RESET_CODES,
}

pub const DCCPO_PADDING: i32 = 0;
pub const DCCPO_MANDATORY: i32 = 1;
pub const DCCPO_MIN_RESERVED: i32 = 3;
pub const DCCPO_MAX_RESERVED: i32 = 31;
pub const DCCPO_CHANGE_L: i32 = 32;
pub const DCCPO_CONFIRM_L: i32 = 33;
pub const DCCPO_CHANGE_R: i32 = 34;
pub const DCCPO_CONFIRM_R: i32 = 35;
pub const DCCPO_NDP_COUNT: i32 = 37;
pub const DCCPO_ACK_VECTOR_0: i32 = 38;
pub const DCCPO_ACK_VECTOR_1: i32 = 39;
pub const DCCPO_TIMESTAMP: i32 = 41;
pub const DCCPO_TIMESTAMP_ECHO: i32 = 42;
pub const DCCPO_ELAPSED_TIME: i32 = 43;
pub const DCCPO_MAX: i32 = 45;
pub const DCCPO_MIN_RX_CCID_SPECIFIC: i32 = 128;
pub const DCCPO_MAX_RX_CCID_SPECIFIC: i32 = 191;
pub const DCCPO_MIN_TX_CCID_SPECIFIC: i32 = 192;
pub const DCCPO_MAX_TX_CCID_SPECIFIC: i32 = 255;
pub const DCCP_SINGLE_OPT_MAXLEN: i32 = 253;

pub const DCCPC_CCID2: i32 = 2;
pub const DCCPC_CCID3: i32 = 3;

#[repr(i32)]
pub enum dccp_feature_numbers {
    DCCPF_RESERVED = 0, DCCPF_CCID, DCCPF_SHORT_SEQNOS, DCCPF_SEQUENCE_WINDOW,
    DCCPF_ECN_INCAPABLE, DCCPF_ACK_RATIO, DCCPF_SEND_ACK_VECTOR, DCCPF_SEND_NDP_COUNT,
    DCCPF_MIN_CSUM_COVER, DCCPF_DATA_CHECKSUM, DCCPF_MIN_CCID_SPECIFIC = 128,
    DCCPF_SEND_LEV_RATE = 192, DCCPF_MAX_CCID_SPECIFIC = 255,
}

#[repr(i32)]
pub enum dccp_cmsg_type { DCCP_SCM_PRIORITY = 1, DCCP_SCM_QPOLICY_MAX = 0xFFFF, DCCP_SCM_MAX }

#[repr(i32)]
pub enum dccp_packet_dequeueing_policy { DCCPQ_POLICY_SIMPLE, DCCPQ_POLICY_PRIO, DCCPQ_POLICY_MAX }

pub const DCCP_SOCKOPT_PACKET_SIZE: i32 = 1;
pub const DCCP_SOCKOPT_SERVICE: i32 = 2;
pub const DCCP_SOCKOPT_CHANGE_L: i32 = 3;
pub const DCCP_SOCKOPT_CHANGE_R: i32 = 4;
pub const DCCP_SOCKOPT_GET_CUR_MPS: i32 = 5;
pub const DCCP_SOCKOPT_SERVER_TIMEWAIT: i32 = 6;
pub const DCCP_SOCKOPT_SEND_CSCOV: i32 = 10;
pub const DCCP_SOCKOPT_RECV_CSCOV: i32 = 11;
pub const DCCP_SOCKOPT_AVAILABLE_CCIDS: i32 = 12;
pub const DCCP_SOCKOPT_CCID: i32 = 13;
pub const DCCP_SOCKOPT_TX_CCID: i32 = 14;
pub const DCCP_SOCKOPT_RX_CCID: i32 = 15;
pub const DCCP_SOCKOPT_QPOLICY_ID: i32 = 16;
pub const DCCP_SOCKOPT_QPOLICY_TXQLEN: i32 = 17;
pub const DCCP_SOCKOPT_CCID_RX_INFO: i32 = 128;
pub const DCCP_SOCKOPT_CCID_TX_INFO: i32 = 192;
pub const DCCP_SERVICE_LIST_MAX_LEN: i32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
