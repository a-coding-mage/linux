// SPDX-License-Identifier: GPL-2.0
//
// C preprocessor state:
// #undef TRACE_SYSTEM
// #define TRACE_SYSTEM test
//
// The original header guard was:
// #if !defined(_TRACE_TEST_H) || defined(TRACE_HEADER_MULTI_READ)
// #define _TRACE_TEST_H

// Dependency supplied by the kernel tracepoint infrastructure:
// #include <linux/tracepoint.h>

#[repr(C)]
pub struct FtraceTestFilterEntry {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
    pub e: i32,
    pub f: i32,
    pub g: i32,
    pub h: i32,
}

/// Translation of the `TRACE_EVENT(ftrace_test_filter, ...)` event.
///
/// The C declaration has eight `int` arguments and stores them, in order, in
/// the event entry before formatting them as:
/// `a %d, b %d, c %d, d %d, e %d, f %d, g %d, h %d`.
#[inline]
pub const fn ftrace_test_filter_entry(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    f: i32,
    g: i32,
    h: i32,
) -> FtraceTestFilterEntry {
    FtraceTestFilterEntry { a, b, c, d, e, f, g, h }
}

// #endif /* _TRACE_TEST_H || TRACE_HEADER_MULTI_READ */

// The following C include-path configuration is retained as dependency
// intent.  The generated trace definitions are supplied externally:
// #undef TRACE_INCLUDE_PATH
// #undef TRACE_INCLUDE_FILE
// #define TRACE_INCLUDE_PATH .
// #define TRACE_INCLUDE_FILE trace_events_filter_test

// This part must be outside protection.
// #include <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
