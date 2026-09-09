/* SPDX-License-Identifier: GPL-2.0 */
/* PPTP constants and structs */

/* Declarations from the included kernel headers are external dependencies. */

unsafe extern "C" {
    pub fn pptp_msg_name(msg: __u16) -> *const core::ffi::c_char;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pptp_ctrlsess_state {
    PPTP_SESSION_NONE,
    PPTP_SESSION_ERROR,
    PPTP_SESSION_STOPREQ,
    PPTP_SESSION_REQUESTED,
    PPTP_SESSION_CONFIRMED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum pptp_ctrlcall_state {
    PPTP_CALL_NONE,
    PPTP_CALL_ERROR,
    PPTP_CALL_OUT_REQ,
    PPTP_CALL_OUT_CONF,
    PPTP_CALL_IN_REQ,
    PPTP_CALL_IN_REP,
    PPTP_CALL_IN_CONF,
    PPTP_CALL_CLEAR_REQ,
}

#[repr(C)]
pub struct nf_ct_pptp_master {
    pub sstate: pptp_ctrlsess_state,
    pub cstate: pptp_ctrlcall_state,
    pub pac_call_id: __be16,
    pub pns_call_id: __be16,
    pub keymap: [*mut nf_ct_gre_keymap; IP_CT_DIR_MAX as usize],
}

#[repr(C)]
pub struct nf_nat_pptp {
    pub pns_call_id: __be16,
    pub pac_call_id: __be16,
}

pub const PPTP_PACKET_CONTROL: u32 = 1;
pub const PPTP_PACKET_MGMT: u32 = 2;
pub const PPTP_MAGIC_COOKIE: u32 = 0x1a2b3c4d;

#[repr(C)]
pub struct pptp_pkt_hdr {
    pub packetLength: __u16,
    pub packetType: __be16,
    pub magicCookie: __be32,
}

pub const PPTP_START_SESSION_REQUEST: u32 = 1;
pub const PPTP_START_SESSION_REPLY: u32 = 2;
pub const PPTP_STOP_SESSION_REQUEST: u32 = 3;
pub const PPTP_STOP_SESSION_REPLY: u32 = 4;
pub const PPTP_ECHO_REQUEST: u32 = 5;
pub const PPTP_ECHO_REPLY: u32 = 6;
pub const PPTP_OUT_CALL_REQUEST: u32 = 7;
pub const PPTP_OUT_CALL_REPLY: u32 = 8;
pub const PPTP_IN_CALL_REQUEST: u32 = 9;
pub const PPTP_IN_CALL_REPLY: u32 = 10;
pub const PPTP_IN_CALL_CONNECT: u32 = 11;
pub const PPTP_CALL_CLEAR_REQUEST: u32 = 12;
pub const PPTP_CALL_DISCONNECT_NOTIFY: u32 = 13;
pub const PPTP_WAN_ERROR_NOTIFY: u32 = 14;
pub const PPTP_SET_LINK_INFO: u32 = 15;
pub const PPTP_MSG_MAX: u32 = 15;

pub const PPTP_ERROR_CODE_NONE: u32 = 0;
pub const PPTP_NOT_CONNECTED: u32 = 1;
pub const PPTP_BAD_FORMAT: u32 = 2;
pub const PPTP_BAD_VALUE: u32 = 3;
pub const PPTP_NO_RESOURCE: u32 = 4;
pub const PPTP_BAD_CALLID: u32 = 5;
pub const PPTP_REMOVE_DEVICE_ERROR: u32 = 6;

#[repr(C)]
pub struct PptpControlHeader { pub messageType: __be16, pub reserved: __u16 }

pub const PPTP_FRAME_CAP_ASYNC: u32 = 0x1;
pub const PPTP_FRAME_CAP_SYNC: u32 = 0x2;
pub const PPTP_BEARER_CAP_ANALOG: u32 = 0x1;
pub const PPTP_BEARER_CAP_DIGITAL: u32 = 0x2;

#[repr(C)]
pub struct PptpStartSessionRequest {
    pub protocolVersion: __be16, pub reserved1: __u16, pub framingCapability: __be32,
    pub bearerCapability: __be32, pub maxChannels: __be16, pub firmwareRevision: __be16,
    pub hostName: [__u8; 64], pub vendorString: [__u8; 64],
}

pub const PPTP_START_OK: u32 = 1;
pub const PPTP_START_GENERAL_ERROR: u32 = 2;
pub const PPTP_START_ALREADY_CONNECTED: u32 = 3;
pub const PPTP_START_NOT_AUTHORIZED: u32 = 4;
pub const PPTP_START_UNKNOWN_PROTOCOL: u32 = 5;

#[repr(C)]
pub struct PptpStartSessionReply {
    pub protocolVersion: __be16, pub resultCode: __u8, pub generalErrorCode: __u8,
    pub framingCapability: __be32, pub bearerCapability: __be32, pub maxChannels: __be16,
    pub firmwareRevision: __be16, pub hostName: [__u8; 64], pub vendorString: [__u8; 64],
}

pub const PPTP_STOP_NONE: u32 = 1;
pub const PPTP_STOP_PROTOCOL: u32 = 2;
pub const PPTP_STOP_LOCAL_SHUTDOWN: u32 = 3;

#[repr(C)] pub struct PptpStopSessionRequest { pub reason: __u8, pub reserved1: __u8, pub reserved2: __u16 }
pub const PPTP_STOP_OK: u32 = 1;
pub const PPTP_STOP_GENERAL_ERROR: u32 = 2;
#[repr(C)] pub struct PptpStopSessionReply { pub resultCode: __u8, pub generalErrorCode: __u8, pub reserved1: __u16 }
#[repr(C)] pub struct PptpEchoRequest { pub identNumber: __be32 }
pub const PPTP_ECHO_OK: u32 = 1;
pub const PPTP_ECHO_GENERAL_ERROR: u32 = 2;
#[repr(C)] pub struct PptpEchoReply { pub identNumber: __be32, pub resultCode: __u8, pub generalErrorCode: __u8, pub reserved: __u16 }

pub const PPTP_ASYNC_FRAMING: u32 = 1;
pub const PPTP_SYNC_FRAMING: u32 = 2;
pub const PPTP_DONT_CARE_FRAMING: u32 = 3;
pub const PPTP_ANALOG_TYPE: u32 = 1;
pub const PPTP_DIGITAL_TYPE: u32 = 2;
pub const PPTP_DONT_CARE_BEARER_TYPE: u32 = 3;

#[repr(C)]
pub struct PptpOutCallRequest {
    pub callID: __be16, pub callSerialNumber: __be16, pub minBPS: __be32, pub maxBPS: __be32,
    pub bearerType: __be32, pub framingType: __be32, pub packetWindow: __be16, pub packetProcDelay: __be16,
    pub phoneNumberLength: __be16, pub reserved1: __u16, pub phoneNumber: [__u8; 64], pub subAddress: [__u8; 64],
}

pub const PPTP_OUTCALL_CONNECT: u32 = 1;
pub const PPTP_OUTCALL_GENERAL_ERROR: u32 = 2;
pub const PPTP_OUTCALL_NO_CARRIER: u32 = 3;
pub const PPTP_OUTCALL_BUSY: u32 = 4;
pub const PPTP_OUTCALL_NO_DIAL_TONE: u32 = 5;
pub const PPTP_OUTCALL_TIMEOUT: u32 = 6;
pub const PPTP_OUTCALL_DONT_ACCEPT: u32 = 7;

#[repr(C)] pub struct PptpOutCallReply { pub callID: __be16, pub peersCallID: __be16, pub resultCode: __u8, pub generalErrorCode: __u8, pub causeCode: __be16, pub connectSpeed: __be32, pub packetWindow: __be16, pub packetProcDelay: __be16, pub physChannelID: __be32 }
#[repr(C)] pub struct PptpInCallRequest { pub callID: __be16, pub callSerialNumber: __be16, pub callBearerType: __be32, pub physChannelID: __be32, pub dialedNumberLength: __be16, pub dialingNumberLength: __be16, pub dialedNumber: [__u8; 64], pub dialingNumber: [__u8; 64], pub subAddress: [__u8; 64] }
pub const PPTP_INCALL_ACCEPT: u32 = 1;
pub const PPTP_INCALL_GENERAL_ERROR: u32 = 2;
pub const PPTP_INCALL_DONT_ACCEPT: u32 = 3;
#[repr(C)] pub struct PptpInCallReply { pub callID: __be16, pub peersCallID: __be16, pub resultCode: __u8, pub generalErrorCode: __u8, pub packetWindow: __be16, pub packetProcDelay: __be16, pub reserved: __u16 }
#[repr(C)] pub struct PptpInCallConnected { pub peersCallID: __be16, pub reserved: __u16, pub connectSpeed: __be32, pub packetWindow: __be16, pub packetProcDelay: __be16, pub callFramingType: __be32 }
#[repr(C)] pub struct PptpClearCallRequest { pub callID: __be16, pub reserved: __u16 }
#[repr(C)] pub struct PptpCallDisconnectNotify { pub callID: __be16, pub resultCode: __u8, pub generalErrorCode: __u8, pub causeCode: __be16, pub reserved: __u16, pub callStatistics: [__u8; 128] }
#[repr(C)] pub struct PptpWanErrorNotify { pub peersCallID: __be16, pub reserved: __u16, pub crcErrors: __be32, pub framingErrors: __be32, pub hardwareOverRuns: __be32, pub bufferOverRuns: __be32, pub timeoutErrors: __be32, pub alignmentErrors: __be32 }
#[repr(C)] pub struct PptpSetLinkInfo { pub peersCallID: __be16, pub reserved: __u16, pub sendAccm: __be32, pub recvAccm: __be32 }

#[repr(C)]
pub union pptp_ctrl_union {
    pub sreq: PptpStartSessionRequest, pub srep: PptpStartSessionReply,
    pub streq: PptpStopSessionRequest, pub strep: PptpStopSessionReply,
    pub ocreq: PptpOutCallRequest, pub ocack: PptpOutCallReply,
    pub icreq: PptpInCallRequest, pub icack: PptpInCallReply,
    pub iccon: PptpInCallConnected, pub clrreq: PptpClearCallRequest,
    pub disc: PptpCallDisconnectNotify, pub wanerr: PptpWanErrorNotify,
    pub setlink: PptpSetLinkInfo,
}

#[repr(C)]
pub struct nf_nat_pptp_hook {
    pub outbound: Option<unsafe extern "C" fn(*mut sk_buff, *mut nf_conn, ip_conntrack_info, u32, *mut PptpControlHeader, *mut pptp_ctrl_union) -> i32>,
    pub inbound: Option<unsafe extern "C" fn(*mut sk_buff, *mut nf_conn, ip_conntrack_info, u32, *mut PptpControlHeader, *mut pptp_ctrl_union) -> i32>,
    pub exp_gre: Option<unsafe extern "C" fn(*mut nf_conntrack_expect, *mut nf_conntrack_expect)>,
    pub expectfn: Option<unsafe extern "C" fn(*mut nf_conn, *mut nf_conntrack_expect)>,
}

unsafe extern "C" {
    pub static mut nf_nat_pptp_hook: *const nf_nat_pptp_hook;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
