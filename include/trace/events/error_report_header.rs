/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Declarations for error reporting tracepoints.
 *
 * Copyright (C) 2021, Google LLC.
 */

//! Rust translation of `trace/events/error_report.h`.
//!
//! The Linux tracepoint declaration machinery represented by the C macros is
//! supplied by the surrounding tracepoint implementation.

use core::ffi::c_ulong;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorDetector {
    ERROR_DETECTOR_KFENCE,
    ERROR_DETECTOR_KASAN,
    ERROR_DETECTOR_WARN,
}

/*
 * error_detector_list:
 * Always end the list with an EMe.
 *
 * The C list expands to tracepoint enum definitions, symbolic-print entries,
 * and the final entry without a trailing comma.  Its Rust representation is
 * retained here as the corresponding value/string pairs.
 */
pub const ERROR_DETECTOR_LIST: &[(ErrorDetector, &str)] = &[
    (ErrorDetector::ERROR_DETECTOR_KFENCE, "kfence"),
    (ErrorDetector::ERROR_DETECTOR_KASAN, "kasan"),
    (ErrorDetector::ERROR_DETECTOR_WARN, "warning"),
];

#[repr(C)]
pub struct ErrorReportTemplateEntry {
    pub error_detector: ErrorDetector,
    pub id: c_ulong,
}

/* DECLARE_EVENT_CLASS(error_report_template, ...) */
pub type ErrorReportTemplate = ErrorReportTemplateEntry;

/**
 * error_report_end - called after printing the error report
 * @error_detector: short string describing the error detection tool
 * @id:             pseudo-unique descriptor identifying the report
 *                 (e.g. the memory access address)
 *
 * This event occurs right after a debugging tool finishes printing the error
 * report.
 */
/// Tracepoint event corresponding to `DEFINE_EVENT(error_report_template,
/// error_report_end, ...)`.
pub const ERROR_REPORT_END: &str = "error_report_end";

/*
 * The C TP_PROTO/TP_ARGS/TP_STRUCT__entry/TP_fast_assign/TP_printk macros
 * describe the following event payload and assignment behavior:
 *
 *     error_detector = error_detector;
 *     id = id;
 *     "[%s] %lx", show_error_detector_list(error_detector), id
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
