/* SPDX-License-Identifier: GPL-2.0 */

// C preprocessor intent:
// TRACE_SYSTEM sync_trace
// TRACE_INCLUDE_PATH ../../drivers/dma-buf
// The include guard and TRACE_HEADER_MULTI_READ condition are omitted from
// executable Rust; Rust items are defined once by the module system.

// External dependency supplied by sync_debug.h.  The complete definition is
// intentionally not reproduced here.
#[repr(C)]
pub struct sync_timeline {
    _private: [u8; 0],
}

// TRACE_EVENT(sync_timeline,
//     TP_PROTO(struct sync_timeline *timeline),
//     TP_ARGS(timeline),
//     TP_STRUCT__entry(
//         __string(name, timeline->name)
//         __field(u32, value)
//     ),
//     TP_fast_assign(
//         __assign_str(name);
//         __entry->value = timeline->value;
//     ),
//     TP_printk("name=%s value=%d", __get_str(name), __entry->value)
// );
//
// Linux TRACE_EVENT expands this declaration into tracepoint machinery.  The
// event's externally supplied C entry point is represented here as an opaque
// declaration; its name and pointer argument preserve the source interface.
unsafe extern "C" {
    pub fn trace_sync_timeline(timeline: *mut sync_timeline);
}

// <trace/define_trace.h> supplies the generated tracepoint definitions in C.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
