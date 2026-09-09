/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */

// The C header is guarded by __IVPU_TRACE_H__ and supports
// TRACE_HEADER_MULTI_READ.  Rust items are emitted once by the module system.
// Dependencies supplied by the original driver are intentionally left opaque.

use core::ffi::c_char;

// Types supplied by ivpu_drv.h, ivpu_job.h, vpu_jsm_api.h, ivpu_jsm_msg.h,
// and ivpu_ipc.h.
#[allow(non_camel_case_types)]
pub enum ivpu_job {}
#[allow(non_camel_case_types)]
pub enum vpu_jsm_msg {}
#[allow(non_camel_case_types)]
pub enum vpu_ipc_msg_status {}

extern "C" {
    pub fn ivpu_jsm_msg_type_to_str(msg_type: u32) -> *const c_char;
}

#[repr(C)]
pub struct IvpuTracePmEntry {
    pub event: *const c_char,
}

#[repr(C)]
pub struct IvpuTraceJobEntry {
    pub event: *const c_char,
    pub ctx_id: u32,
    pub engine_id: u32,
    pub job_id: u32,
}

#[repr(C)]
pub struct IvpuTraceJsmEntry {
    pub event: *const c_char,
    pub type_: *const c_char,
    pub status: vpu_ipc_msg_status,
    pub request_id: u32,
    pub result: u32,
}

// TRACE_EVENT(pm, TP_PROTO(const char *event), ...)
// The tracepoint receives the event string and stores it in the entry.
#[inline]
pub unsafe fn trace_pm(event: *const c_char) -> IvpuTracePmEntry {
    IvpuTracePmEntry { event }
}

// TRACE_EVENT(job, TP_PROTO(const char *event, struct ivpu_job *job), ...)
// Field access mirrors __entry->ctx_id = job->file_priv->ctx.id,
// __entry->engine_id = job->engine_idx, and __entry->job_id = job->job_id.
#[inline]
pub unsafe fn trace_job(
    event: *const c_char,
    _job: *mut ivpu_job,
    ctx_id: u32,
    engine_id: u32,
    job_id: u32,
) -> IvpuTraceJobEntry {
    IvpuTraceJobEntry {
        event,
        ctx_id,
        engine_id,
        job_id,
    }
}

// TRACE_EVENT(jsm, TP_PROTO(const char *event, struct vpu_jsm_msg *msg), ...)
// The caller supplies the message fields represented by the C tracepoint.
#[inline]
pub unsafe fn trace_jsm(
    event: *const c_char,
    _msg: *mut vpu_jsm_msg,
    type_: *const c_char,
    status: vpu_ipc_msg_status,
    request_id: u32,
    result: u32,
) -> IvpuTraceJsmEntry {
    IvpuTraceJsmEntry {
        event,
        type_,
        status,
        request_id,
        result,
    }
}

// TRACE_INCLUDE_PATH .; trace/define_trace.h supplies the generated tracepoint
// plumbing in the original C build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
