/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/scmi.h.
// The Linux tracepoint registration and formatting machinery is supplied by
// external dependencies; the entry layouts and event declarations are kept
// here as their direct Rust equivalents.

pub const TRACE_SCMI_MAX_TAG_LEN: usize = 6;

#[repr(C)]
pub struct ScmiFcCallEntry {
    pub protocol_id: u8,
    pub msg_id: u8,
    pub res_id: u32,
    pub val1: u32,
    pub val2: u32,
}

#[repr(C)]
pub struct ScmiXferBeginEntry {
    pub transfer_id: i32,
    pub msg_id: u8,
    pub protocol_id: u8,
    pub seq: u16,
    pub poll: bool,
    pub inflight: i32,
}

#[repr(C)]
pub struct ScmiXferResponseWaitEntry {
    pub transfer_id: i32,
    pub msg_id: u8,
    pub protocol_id: u8,
    pub seq: u16,
    pub timeout: u32,
    pub poll: bool,
}

#[repr(C)]
pub struct ScmiXferEndEntry {
    pub transfer_id: i32,
    pub msg_id: u8,
    pub protocol_id: u8,
    pub seq: u16,
    pub status: i32,
    pub inflight: i32,
}

#[repr(C)]
pub struct ScmiRxDoneEntry {
    pub transfer_id: i32,
    pub msg_id: u8,
    pub protocol_id: u8,
    pub seq: u16,
    pub msg_type: u8,
}

#[repr(C)]
pub struct ScmiMsgDumpEntry {
    pub id: i32,
    pub channel_id: u8,
    pub protocol_id: u8,
    pub msg_id: u8,
    pub tag: [core::ffi::c_char; TRACE_SCMI_MAX_TAG_LEN],
    pub seq: u16,
    pub status: i32,
    pub len: usize,
    pub cmd: *mut u8,
}

/// Trace event: scmi_fc_call.
pub const SCMI_FC_CALL_FORMAT: &str = "pt=%02X msg_id=%02X res_id:%u vals=%u:%u";

/// Trace event: scmi_xfer_begin.
pub const SCMI_XFER_BEGIN_FORMAT: &str =
    "pt=%02X msg_id=%02X seq=%04X transfer_id=%X poll=%u inflight=%d";

/// Trace event: scmi_xfer_response_wait.
pub const SCMI_XFER_RESPONSE_WAIT_FORMAT: &str =
    "pt=%02X msg_id=%02X seq=%04X transfer_id=%X tmo_ms=%u poll=%u";

/// Trace event: scmi_xfer_end.
pub const SCMI_XFER_END_FORMAT: &str =
    "pt=%02X msg_id=%02X seq=%04X transfer_id=%X s=%d inflight=%d";

/// Trace event: scmi_rx_done.
pub const SCMI_RX_DONE_FORMAT: &str =
    "pt=%02X msg_id=%02X seq=%04X transfer_id=%X msg_type=%u";

/// Trace event: scmi_msg_dump.
pub const SCMI_MSG_DUMP_FORMAT: &str =
    "id=%d ch=%02X pt=%02X t=%s msg_id=%02X seq=%04X s=%d pyld=%s";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
