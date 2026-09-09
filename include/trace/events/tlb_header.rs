/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the C tracepoint header. The Linux tracepoint and mm_types
// declarations are supplied by the surrounding translation unit.

// TLB_FLUSH_REASON
// First define the enum values exported to userspace by TRACE_DEFINE_ENUM().
pub const TLB_FLUSH_ON_TASK_SWITCH: i32 = 0;
pub const TLB_REMOTE_SHOOTDOWN: i32 = 1;
pub const TLB_LOCAL_SHOOTDOWN: i32 = 2;
pub const TLB_LOCAL_MM_SHOOTDOWN: i32 = 3;
pub const TLB_REMOTE_SEND_IPI: i32 = 4;
pub const TLB_REMOTE_WRONG_CPU: i32 = 5;

// The strings printed for the symbolic reason values.
pub static TLB_FLUSH_REASON: &[(i32, &str)] = &[
    (TLB_FLUSH_ON_TASK_SWITCH, "flush on task switch"),
    (TLB_REMOTE_SHOOTDOWN, "remote shootdown"),
    (TLB_LOCAL_SHOOTDOWN, "local shootdown"),
    (TLB_LOCAL_MM_SHOOTDOWN, "local MM shootdown"),
    (TLB_REMOTE_SEND_IPI, "remote IPI send"),
    (TLB_REMOTE_WRONG_CPU, "remote wrong CPU"),
];

#[repr(C)]
pub struct TlbFlushEntry {
    pub reason: i32,
    pub pages: ::core::ffi::c_ulong,
}

// TRACE_EVENT(tlb_flush,
//     TP_PROTO(int reason, unsigned long pages),
//     TP_ARGS(reason, pages),
//     TP_STRUCT__entry(__field(int, reason) __field(unsigned long, pages)),
//     TP_fast_assign(__entry->reason = reason; __entry->pages = pages;),
//     TP_printk("pages:%ld reason:%s (%d)", __entry->pages,
//         __print_symbolic(__entry->reason, TLB_FLUSH_REASON),
//         __entry->reason));
//
// The generated tracepoint registration and printing machinery is provided by
// the surrounding Linux tracepoint implementation.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
