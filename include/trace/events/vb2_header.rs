/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of the vb2 tracepoint header.
//!
//! The C header depends on `linux/tracepoint.h` and
//! `media/videobuf2-core.h`; those dependencies are intentionally not
//! reimplemented here.

use core::ffi::c_void;

/// Opaque dependency type supplied by `media/videobuf2-core.h`.
#[repr(C)]
pub struct vb2_queue {
    _private: [u8; 0],
}

/// Opaque dependency type supplied by `media/videobuf2-core.h`.
#[repr(C)]
pub struct vb2_buffer {
    _private: [u8; 0],
}

/// Entry data captured by `vb2_event_class`.
#[repr(C)]
pub struct vb2_event_class_entry {
    pub owner: *mut c_void,
    pub queued_count: u32,
    pub owned_by_drv_count: core::ffi::c_int,
    pub index: u32,
    pub type_: u32,
    pub bytesused: u32,
    pub timestamp: u64,
}

/// C `DECLARE_EVENT_CLASS(vb2_event_class, ...)`.
///
/// The assignment and printk expressions are retained here as the tracepoint
/// declaration's source-level behavior; queue and buffer layouts are supplied
/// by the external videobuf2 dependency.
#[inline]
pub unsafe fn vb2_event_class_assign(
    _q: *mut vb2_queue,
    _vb: *mut vb2_buffer,
    _entry: *mut vb2_event_class_entry,
) {
    // __entry->owner = q->owner;
    // __entry->queued_count = q->queued_count;
    // __entry->owned_by_drv_count = atomic_read(&q->owned_by_drv_count);
    // __entry->index = vb->index;
    // __entry->type = vb->type;
    // __entry->bytesused = vb->planes[0].bytesused;
    // __entry->timestamp = vb->timestamp;
    // The referenced C layouts are external to this isolated translation.
}

/// C `DEFINE_EVENT(vb2_event_class, vb2_buf_done, ...)`.
extern "C" {
    pub fn vb2_buf_done(q: *mut vb2_queue, vb: *mut vb2_buffer);

    /// C `DEFINE_EVENT(vb2_event_class, vb2_buf_queue, ...)`.
    pub fn vb2_buf_queue(q: *mut vb2_queue, vb: *mut vb2_buffer);

    /// C `DEFINE_EVENT(vb2_event_class, vb2_dqbuf, ...)`.
    pub fn vb2_dqbuf(q: *mut vb2_queue, vb: *mut vb2_buffer);

    /// C `DEFINE_EVENT(vb2_event_class, vb2_qbuf, ...)`.
    pub fn vb2_qbuf(q: *mut vb2_queue, vb: *mut vb2_buffer);
}

// The C header includes <trace/define_trace.h> outside its include guard.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
