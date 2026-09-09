/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of the Linux MPTCP trace-event header.
//!
//! The original file is consumed by the kernel tracepoint generator.  The
//! kernel types and helper functions referenced by the event assignments are
//! intentionally left as external dependencies.

pub const MAPPING_OK: u8 = 0;
pub const MAPPING_INVALID: u8 = 1;
pub const MAPPING_EMPTY: u8 = 2;
pub const MAPPING_DATA_FIN: u8 = 3;
pub const MAPPING_DUMMY: u8 = 4;

pub const fn show_mapping_status(status: u8) -> &'static str {
    match status {
        0 => "MAPPING_OK",
        1 => "MAPPING_INVALID",
        2 => "MAPPING_EMPTY",
        3 => "MAPPING_DATA_FIN",
        4 => "MAPPING_DUMMY",
        _ => "?",
    }
}

#[repr(C)]
pub struct MptcpSubflowGetSendEntry {
    pub active: bool,
    pub free: bool,
    pub snd_wnd: u32,
    pub pace: u32,
    pub backup: u8,
    pub ratio: u64,
}

#[repr(C)]
pub struct MptcpDumpMpextEntry {
    pub data_seq: u64,
    pub subflow_seq: u32,
    pub data_len: u16,
    pub csum: u16,
    pub use_map: u8,
    pub dsn64: u8,
    pub data_fin: u8,
    pub use_ack: u8,
    pub ack64: u8,
    pub mpc_map: u8,
    pub frozen: u8,
    pub reset_transient: u8,
    pub reset_reason: u8,
    pub csum_reqd: u8,
    pub infinite_map: u8,
}

#[repr(C)]
pub struct AckUpdateMskEntry {
    pub data_ack: u64,
    pub old_snd_una: u64,
    pub new_snd_una: u64,
    pub new_wnd_end: u64,
    pub msk_wnd_end: u64,
}

#[repr(C)]
pub struct SubflowCheckDataAvailEntry {
    pub status: u8,
    pub skb: *const core::ffi::c_void,
}

#[repr(C)]
pub struct MptcpRcvbufGrowEntry {
    pub time: core::ffi::c_int,
    pub rtt_us: u32,
    pub copied: u32,
    pub inq: u32,
    pub space: u32,
    pub ooo_space: u32,
    pub rcvbuf: u32,
    pub rcv_wnd: u32,
    pub scaling_ratio: u8,
    pub sport: u16,
    pub dport: u16,
    pub family: u16,
    pub saddr: [u8; 4],
    pub daddr: [u8; 4],
    pub saddr_v6: [u8; 16],
    pub daddr_v6: [u8; 16],
    pub skaddr: *const core::ffi::c_void,
}

// Trace event declarations and their original assignment/printing behavior:
//
// mptcp_subflow_get_send(subflow): active=mptcp_subflow_active(subflow);
// backup=subflow->backup || subflow->request_bkup; free is
// sk_stream_memory_free(subflow->tcp_sock) only for a full socket; snd_wnd and
// pace come from tcp_sk(ssk)->snd_wnd and READ_ONCE(ssk->sk_pacing_rate), or
// zero when unavailable; ratio is div_u64((u64)ssk->sk_wmem_queued << 32,
// pace) when pace is nonzero, otherwise zero.
//
// mptcp_dump_mpext is the shared event class for mptcp_sendmsg_frag and
// get_mapping_status.  It copies every field of struct mptcp_ext into
// MptcpDumpMpextEntry, including the forced u16 checksum.
//
// ack_update_msk copies data_ack, old_snd_una, new_snd_una, new_wnd_end, and
// msk_wnd_end into AckUpdateMskEntry.
//
// subflow_check_data_avail copies status and the skb pointer and prints
// show_mapping_status(status), skb.
//
// mptcp_rcvbuf_grow obtains msk=mptcp_sk(sk) and inet=inet_sk(sk), copies the
// receive-space, queue, window, address, port, family, and socket-pointer
// fields described by MptcpRcvbufGrowEntry, and stores IPv4/IPv6 addresses via
// TP_STORE_ADDRS.  Its out-of-order space is zero for an empty RB tree;
// otherwise it is MPTCP_SKB_CB(msk->ooo_last_skb)->end_seq - msk->ack_seq.

pub struct MptcpSendmsgFrag;
pub struct GetMappingStatus;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
