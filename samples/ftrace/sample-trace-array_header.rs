/* SPDX-License-Identifier: GPL-2.0 */

/*
 * If TRACE_SYSTEM is defined, that will be the directory created
 * in the ftrace directory under /sys/kernel/tracing/events/<system>.
 *
 * The original header is intentionally reread by the tracepoint
 * generation machinery; these Rust constants preserve its names and intent.
 */

pub const TRACE_SYSTEM: &str = "sample-subsystem";
pub const TRACE_SYSTEM_VAR: &str = "sample_subsystem";

/*
 * C header guard condition:
 * !defined(_SAMPLE_TRACE_ARRAY_H) || defined(TRACE_HEADER_MULTI_READ)
 *
 * The include of linux/tracepoint.h and trace/define_trace.h supplies the
 * TRACE_EVENT machinery externally and is therefore not implemented here.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SampleEventEntry {
    pub count: core::ffi::c_int,
    pub time: core::ffi::c_ulong,
}

impl SampleEventEntry {
    #[inline]
    pub unsafe fn fast_assign(
        entry: *mut SampleEventEntry,
        count: core::ffi::c_int,
        time: core::ffi::c_ulong,
    ) {
        (*entry).count = count;
        (*entry).time = time;
    }
}

/*
 * Equivalent TRACE_EVENT declaration:
 *
 * TRACE_EVENT(sample_event,
 *     TP_PROTO(int count, unsigned long time),
 *     TP_ARGS(count, time),
 *     TP_STRUCT__entry(
 *         __field(int, count)
 *         __field(unsigned long, time)
 *     ),
 *     TP_fast_assign(
 *         __entry->count = count;
 *         __entry->time = time;
 *     ),
 *     TP_printk("count value=%d at jiffies=%lu", __entry->count,
 *         __entry->time)
 * );
 */

pub const SAMPLE_EVENT_PRINTK: &str = "count value=%d at jiffies=%lu";

pub type SampleEventProto = unsafe extern "C" fn(
    count: core::ffi::c_int,
    time: core::ffi::c_ulong,
);

pub const TRACE_INCLUDE_PATH: &str = ".";
pub const TRACE_INCLUDE_FILE: &str = "sample-trace-array";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
