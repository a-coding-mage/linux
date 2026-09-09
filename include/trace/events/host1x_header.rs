/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/trace/events/host1x.h
 *
 * host1x event logging to ftrace.
 *
 * Copyright (c) 2010-2013, NVIDIA Corporation.
 */

// TRACE_SYSTEM host1x
// The Linux ktime, tracepoint, and define_trace headers are supplied by the
// surrounding kernel integration and are intentionally not reimplemented here.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct Host1xBo {
    _private: [u8; 0],
}

pub type U32 = u32;

#[repr(C)]
pub struct Host1xEntry {
    pub name: *const c_char,
}

#[repr(C)]
pub struct Host1xCdmaPushEntry {
    pub name: *const c_char,
    pub op1: U32,
    pub op2: U32,
}

#[repr(C)]
pub struct Host1xCdmaPushWideEntry {
    pub name: *const c_char,
    pub op1: U32,
    pub op2: U32,
    pub op3: U32,
    pub op4: U32,
}

#[repr(C)]
pub struct Host1xCdmaPushGatherEntry {
    pub bo: *mut Host1xBo,
    pub words: U32,
    pub offset: U32,
    pub cmdbuf: bool,
    pub cmdbuf_data: *mut U32,
    pub cmdbuf_len: usize,
}

#[repr(C)]
pub struct Host1xChannelSubmitEntry {
    pub name: *const c_char,
    pub cmdbufs: U32,
    pub relocs: U32,
    pub syncpt_id: U32,
    pub syncpt_incrs: U32,
}

#[repr(C)]
pub struct Host1xChannelSubmittedEntry {
    pub name: *const c_char,
    pub syncpt_base: U32,
    pub syncpt_max: U32,
}

#[repr(C)]
pub struct Host1xChannelSubmitCompleteEntry {
    pub name: *const c_char,
    pub count: core::ffi::c_int,
    pub thresh: U32,
}

#[repr(C)]
pub struct Host1xWaitCdmaEntry {
    pub name: *const c_char,
    pub eventid: U32,
}

#[repr(C)]
pub struct Host1xSyncptLoadMinEntry {
    pub id: U32,
    pub val: U32,
}

#[repr(C)]
pub struct Host1xSyncptWaitCheckEntry {
    pub bo: *mut Host1xBo,
    pub offset: U32,
    pub syncpt_id: U32,
    pub thresh: U32,
    pub min: U32,
}

// Event class host1x: TP_PROTO(const char *name)
// TP_STRUCT__entry(__string(name, name))
// TP_printk("name=%s", __get_str(name))

// Events defined from the host1x class:
// host1x_channel_open, host1x_channel_release, host1x_cdma_begin,
// host1x_cdma_end.

// host1x_cdma_push: TP_PROTO(const char *name, u32 op1, u32 op2)
// TP_printk("name=%s, op1=%08x, op2=%08x", __get_str(name), op1, op2)

// host1x_cdma_push_wide: TP_PROTO(const char *name, u32 op1, u32 op2,
// u32 op3, u32 op4)
// TP_printk("name=%s, op1=%08x, op2=%08x, op3=%08x op4=%08x", ...)

// host1x_cdma_push_gather: TP_PROTO(const char *name, struct host1x_bo *bo,
// u32 words, u32 offset, void *cmdbuf)
// If cmdbuf is non-null, words u32 values are copied from cmdbuf + offset
// into the dynamic trace array before the remaining fields are assigned.

// host1x_channel_submit: TP_PROTO(const char *name, u32 cmdbufs, u32 relocs,
// u32 syncpt_id, u32 syncpt_incrs)

// host1x_channel_submitted: TP_PROTO(const char *name, u32 syncpt_base,
// u32 syncpt_max)

// host1x_channel_submit_complete: TP_PROTO(const char *name, int count,
// u32 thresh)

// host1x_wait_cdma: TP_PROTO(const char *name, u32 eventid)

// host1x_syncpt_load_min: TP_PROTO(u32 id, u32 val)

// host1x_syncpt_wait_check: TP_PROTO(struct host1x_bo *bo, u32 offset,
// u32 syncpt_id, u32 thresh, u32 min)

// The tracepoint implementations and formatting are provided by the kernel
// trace framework, corresponding to the declarations and assignments above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
