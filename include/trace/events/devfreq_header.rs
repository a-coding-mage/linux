/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of the Linux devfreq trace-event header.
//!
//! The C `TRACE_EVENT` declarations below describe tracepoint payloads and
//! their assignment/printing rules.  The devfreq and tracepoint definitions
//! are supplied by external dependencies.

use core::ffi::c_char;

/* C dependencies:
 *   #include <linux/devfreq.h>
 *   #include <linux/tracepoint.h>
 *   #include <trace/define_trace.h>
 */

/// Payload of the `devfreq_frequency` trace event.
#[repr(C)]
pub struct DevfreqFrequencyEntry {
    pub dev_name: *const c_char,
    pub freq: usize,
    pub prev_freq: usize,
    pub busy_time: usize,
    pub total_time: usize,
}

/// Payload of the `devfreq_monitor` trace event.
#[repr(C)]
pub struct DevfreqMonitorEntry {
    pub freq: usize,
    pub busy_time: usize,
    pub total_time: usize,
    pub polling_ms: u32,
    pub dev_name: *const c_char,
}

/// `devfreq_frequency` tracepoint assignment and print semantics.
///
/// `devfreq` is an external `struct devfreq *`; its fields and `dev_name`
/// helper are provided by the Linux devfreq dependency.
#[inline]
pub unsafe fn devfreq_frequency_assign(
    entry: *mut DevfreqFrequencyEntry,
    dev_name: *const c_char,
    freq: usize,
    prev_freq: usize,
    busy_time: usize,
    total_time: usize,
) {
    (*entry).dev_name = dev_name;
    (*entry).freq = freq;
    (*entry).prev_freq = prev_freq;
    (*entry).busy_time = busy_time;
    (*entry).total_time = total_time;
}

/// Computes the load printed by `devfreq_frequency` and `devfreq_monitor`.
#[inline]
pub const fn devfreq_load(busy_time: usize, total_time: usize) -> usize {
    if total_time == 0 {
        0
    } else {
        (100 * busy_time) / total_time
    }
}

/// `devfreq_monitor` tracepoint assignment semantics.
#[inline]
pub unsafe fn devfreq_monitor_assign(
    entry: *mut DevfreqMonitorEntry,
    freq: usize,
    busy_time: usize,
    total_time: usize,
    polling_ms: u32,
    dev_name: *const c_char,
) {
    (*entry).freq = freq;
    (*entry).busy_time = busy_time;
    (*entry).total_time = total_time;
    (*entry).polling_ms = polling_ms;
    (*entry).dev_name = dev_name;
}

/*
 * C print formats preserved from the trace events:
 *
 * devfreq_frequency:
 *   "dev_name=%-30s freq=%-12lu prev_freq=%-12lu load=%-2lu"
 *
 * devfreq_monitor:
 *   "dev_name=%-30s freq=%-12lu polling_ms=%-3u load=%-2lu"
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
