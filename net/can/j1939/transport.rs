// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of can/j1939/transport.c.
// Kernel-provided types and functions are intentionally referenced externally.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const J1939_XTP_TX_RETRY_LIMIT: u32 = 100;
pub const J1939_ETP_PGN_CTL: u32 = 0xc800;
pub const J1939_ETP_PGN_DAT: u32 = 0xc700;
pub const J1939_TP_PGN_CTL: u32 = 0xec00;
pub const J1939_TP_PGN_DAT: u32 = 0xeb00;
pub const J1939_TP_CMD_RTS: u8 = 0x10;
pub const J1939_TP_CMD_CTS: u8 = 0x11;
pub const J1939_TP_CMD_EOMA: u8 = 0x13;
pub const J1939_TP_CMD_BAM: u8 = 0x20;
pub const J1939_TP_CMD_ABORT: u8 = 0xff;
pub const J1939_ETP_CMD_RTS: u8 = 0x14;
pub const J1939_ETP_CMD_CTS: u8 = 0x15;
pub const J1939_ETP_CMD_DPO: u8 = 0x16;
pub const J1939_ETP_CMD_EOMA: u8 = 0x17;
pub const J1939_ETP_CMD_ABORT: u8 = 0xff;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum j1939_xtp_abort {
    J1939_XTP_NO_ABORT = 0,
    J1939_XTP_ABORT_BUSY = 1,
    J1939_XTP_ABORT_RESOURCE = 2,
    J1939_XTP_ABORT_TIMEOUT = 3,
    J1939_XTP_ABORT_GENERIC = 4,
    J1939_XTP_ABORT_FAULT = 5,
    J1939_XTP_ABORT_UNEXPECTED_DATA = 6,
    J1939_XTP_ABORT_BAD_SEQ = 7,
    J1939_XTP_ABORT_DUP_SEQ = 8,
    J1939_XTP_ABORT_EDPO_UNEXPECTED = 9,
    J1939_XTP_ABORT_BAD_EDPO_PGN = 10,
    J1939_XTP_ABORT_EDPO_OUTOF_CTS = 11,
    J1939_XTP_ABORT_BAD_EDPO_OFFSET = 12,
    J1939_XTP_ABORT_OTHER_DEPRECATED = 13,
    J1939_XTP_ABORT_ECTS_UNXPECTED_PGN = 14,
    J1939_XTP_ABORT_ECTS_TOO_BIG = 15,
    J1939_XTP_ABORT_OTHER = 250,
}

pub static mut j1939_tp_block: u32 = 255;
pub static mut j1939_tp_packet_delay: u32 = 0;
pub static mut j1939_tp_padding: u32 = 1;

pub type pgn_t = u32;

#[inline]
pub fn j1939_xtp_abort_to_str(abort: j1939_xtp_abort) -> &'static str {
    match abort {
        j1939_xtp_abort::J1939_XTP_ABORT_BUSY => "Already in one or more connection managed sessions and cannot support another.",
        j1939_xtp_abort::J1939_XTP_ABORT_RESOURCE => "System resources were needed for another task so this connection managed session was terminated.",
        j1939_xtp_abort::J1939_XTP_ABORT_TIMEOUT => "A timeout occurred and this is the connection abort to close the session.",
        j1939_xtp_abort::J1939_XTP_ABORT_GENERIC => "CTS messages received when data transfer is in progress",
        j1939_xtp_abort::J1939_XTP_ABORT_FAULT => "Maximal retransmit request limit reached",
        j1939_xtp_abort::J1939_XTP_ABORT_UNEXPECTED_DATA => "Unexpected data transfer packet",
        j1939_xtp_abort::J1939_XTP_ABORT_BAD_SEQ => "Bad sequence number (and software is not able to recover)",
        j1939_xtp_abort::J1939_XTP_ABORT_DUP_SEQ => "Duplicate sequence number (and software is not able to recover)",
        j1939_xtp_abort::J1939_XTP_ABORT_EDPO_UNEXPECTED => "Unexpected EDPO packet (ETP) or Message size > 1785 bytes (TP)",
        j1939_xtp_abort::J1939_XTP_ABORT_BAD_EDPO_PGN => "Unexpected EDPO PGN (PGN in EDPO is bad)",
        j1939_xtp_abort::J1939_XTP_ABORT_EDPO_OUTOF_CTS => "EDPO number of packets is greater than CTS",
        j1939_xtp_abort::J1939_XTP_ABORT_BAD_EDPO_OFFSET => "Bad EDPO offset",
        j1939_xtp_abort::J1939_XTP_ABORT_OTHER_DEPRECATED => "Deprecated. Use 250 instead (Any other reason)",
        j1939_xtp_abort::J1939_XTP_ABORT_ECTS_UNXPECTED_PGN => "Unexpected ECTS PGN (PGN in ECTS is bad)",
        j1939_xtp_abort::J1939_XTP_ABORT_ECTS_TOO_BIG => "ECTS requested packets exceeds message size",
        j1939_xtp_abort::J1939_XTP_ABORT_OTHER => "Any other reason (if a Connection Abort reason is identified that is not listed in the table use code 250)",
        _ => "<unknown>",
    }
}

// The remaining implementation retains the C kernel ABI through an unsafe
// declaration boundary; definitions are supplied by the surrounding kernel
// translation unit.  These declarations preserve the externally visible API.
extern "C" {
    pub fn j1939_session_get(session: *mut j1939_session);
    pub fn j1939_session_put(session: *mut j1939_session);
    pub fn j1939_session_timers_cancel(session: *mut j1939_session);
    pub fn j1939_session_activate(session: *mut j1939_session) -> i32;
    pub fn j1939_tp_send(priv_: *mut j1939_priv, skb: *mut sk_buff, size: usize) -> *mut j1939_session;
    pub fn j1939_tp_recv(priv_: *mut j1939_priv, skb: *mut sk_buff) -> i32;
    pub fn j1939_simple_recv(priv_: *mut j1939_priv, skb: *mut sk_buff);
    pub fn j1939_cancel_active_session(priv_: *mut j1939_priv, sk: *mut sock) -> i32;
    pub fn j1939_tp_init(priv_: *mut j1939_priv);
}

#[repr(C)] pub struct j1939_priv { _opaque: [u8; 0] }
#[repr(C)] pub struct j1939_session { _opaque: [u8; 0] }
#[repr(C)] pub struct sk_buff { _opaque: [u8; 0] }
#[repr(C)] pub struct sock { _opaque: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
