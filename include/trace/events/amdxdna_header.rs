/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
 */

//! Rust representation of the tracepoint declarations from `amdxdna.h`.
//! The kernel tracepoint registration and emission machinery is external.

use core::ffi::{c_char, c_int};

/// Opaque declaration supplied by the DRM scheduler.
#[repr(C)]
pub struct DrmSchedJob {
    _private: [u8; 0],
}

/// Payload for `amdxdna_debug_point`.
#[repr(C)]
pub struct AmdxdnaDebugPointEntry {
    pub name: *const c_char,
    pub number: u64,
    pub str_: *const c_char,
}

/// `TRACE_EVENT(amdxdna_debug_point, ...)`
///
/// Print format: "%s:%llu %s".
pub type AmdxdnaDebugPoint = AmdxdnaDebugPointEntry;

/// Payload for `xdna_job`.
#[repr(C)]
pub struct XdnaJobEntry {
    pub name: *const c_char,
    pub str_: *const c_char,
    pub fence_context: u64,
    pub fence_seqno: u64,
    pub seq: u64,
    pub op: u32,
}

/// `TRACE_EVENT(xdna_job, ...)`
///
/// The `fence_context` and `fence_seqno` fields are read from the scheduler
/// job's finished fence before the remaining fields are assigned.
/// Print format: "fence=(context:%llu, seqno:%llu), %s seq#:%llu %s, op=%u".
pub type XdnaJob = XdnaJobEntry;

/// Payload shared by the `xdna_mbox_msg` event class and its events.
#[repr(C)]
pub struct XdnaMboxMsgEntry {
    pub name: *mut c_char,
    pub chann_id: u32,
    pub opcode: u32,
    pub msg_id: u32,
}

/// `DECLARE_EVENT_CLASS(xdna_mbox_msg, ...)`.
/// Print format: "%s.%d id 0x%x opcode 0x%x".
pub type XdnaMboxMsg = XdnaMboxMsgEntry;

/// `DEFINE_EVENT(xdna_mbox_msg, mbox_set_tail, ...)`.
pub type MboxSetTail = XdnaMboxMsgEntry;

/// `DEFINE_EVENT(xdna_mbox_msg, mbox_set_head, ...)`.
pub type MboxSetHead = XdnaMboxMsgEntry;

/// Payload shared by the `xdna_mbox_name_id` event class and its events.
#[repr(C)]
pub struct XdnaMboxNameIdEntry {
    pub name: *mut c_char,
    pub irq: c_int,
}

/// `DECLARE_EVENT_CLASS(xdna_mbox_name_id, ...)`.
/// Print format: "%s.%d".
pub type XdnaMboxNameId = XdnaMboxNameIdEntry;

/// `DEFINE_EVENT(xdna_mbox_name_id, mbox_irq_handle, ...)`.
pub type MboxIrqHandle = XdnaMboxNameIdEntry;

/// `DEFINE_EVENT(xdna_mbox_name_id, mbox_rx_worker, ...)`.
pub type MboxRxWorker = XdnaMboxNameIdEntry;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
