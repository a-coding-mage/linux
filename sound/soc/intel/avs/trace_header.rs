// SPDX-License-Identifier: GPL-2.0

// C trace metadata:
// TRACE_SYSTEM intel_avs

use core::ffi::{c_char, c_void};

// From linux/types.h.
pub type u8 = u8;
pub type u32 = u32;
pub type u64 = u64;
pub type size_t = usize;

// TRACE_EVENT(avs_dsp_core_op)
#[repr(C)]
pub struct trace_event_raw_avs_dsp_core_op {
    pub reg: u32,
    pub mask: u32,
    pub flag: bool,
}

// DECLARE_EVENT_CLASS(avs_ipc_msg_hdr)
#[repr(C)]
pub struct trace_event_raw_avs_ipc_msg_hdr {
    pub header: u64,
    pub sts: u32,
    pub lec: u32,
}

// TRACE_EVENT_CONDITION(avs_ipc_msg_payload)
#[repr(C)]
pub struct trace_event_raw_avs_ipc_msg_payload {
    pub offset: size_t,
    pub pos: size_t,
    pub total: size_t,
}

// TRACE_EVENT(avs_d0ix)
#[repr(C)]
pub struct trace_event_raw_avs_d0ix {
    pub proceed: bool,
    pub header: u64,
}

unsafe extern "C" {
    pub fn trace_avs_dsp_core_op(reg: u32, mask: u32, op: *const c_char, flag: bool);

    pub fn trace_avs_msg_payload(data: *const c_void, size: size_t);

    pub fn trace_avs_ipc_request_msg(header: u64, sts: u32, lec: u32);
    pub fn trace_avs_ipc_reply_msg(header: u64, sts: u32, lec: u32);
    pub fn trace_avs_ipc_notify_msg(header: u64, sts: u32, lec: u32);

    pub fn trace_avs_ipc_msg_payload(
        data: *const u8,
        size: size_t,
        offset: size_t,
        total: size_t,
    );

    pub fn trace_avs_d0ix(op: *const c_char, proceed: bool, header: u64);
}

#[repr(C)]
pub struct avs_ipc_msg {
    pub header: u64,
    pub data: *const c_void,
    pub size: size_t,
}

// #define trace_avs_request(msg, sts, lec)
pub unsafe fn trace_avs_request(msg: *const avs_ipc_msg, sts: u32, lec: u32) {
    unsafe {
        trace_avs_ipc_request_msg((*msg).header, sts, lec);
        trace_avs_msg_payload((*msg).data, (*msg).size);
    }
}

// #define trace_avs_reply(msg, sts, lec)
pub unsafe fn trace_avs_reply(msg: *const avs_ipc_msg, sts: u32, lec: u32) {
    unsafe {
        trace_avs_ipc_reply_msg((*msg).header, sts, lec);
        trace_avs_msg_payload((*msg).data, (*msg).size);
    }
}

// #define trace_avs_notify(msg, sts, lec)
pub unsafe fn trace_avs_notify(msg: *const avs_ipc_msg, sts: u32, lec: u32) {
    unsafe {
        trace_avs_ipc_notify_msg((*msg).header, sts, lec);
        trace_avs_msg_payload((*msg).data, (*msg).size);
    }
}

// TRACE_EVENT_CONDITION(avs_ipc_msg_payload): TP_CONDITION(data && size)
pub const fn avs_ipc_msg_payload_condition(data: *const u8, size: size_t) -> bool {
    !data.is_null() && size != 0
}

// This part must be outside protection:
// TRACE_INCLUDE_PATH .
// TRACE_INCLUDE_FILE trace
// #include <trace/define_trace.h>

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
