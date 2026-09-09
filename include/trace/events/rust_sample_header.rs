/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Tracepoints for `samples/rust/rust_print.rs`.
 *
 * Copyright (C) 2024 Google, Inc.
 */

// C preprocessor trace-system selection:
// #undef TRACE_SYSTEM
// #define TRACE_SYSTEM rust_sample

// The C header guard and TRACE_HEADER_MULTI_READ condition protect the
// TRACE_EVENT declaration from duplicate inclusion.

/// Data recorded by the `rust_sample_loaded` trace event.
#[repr(C)]
pub struct RustSampleLoadedEntry {
    pub magic_number: i32,
}

/// Arguments supplied to the `rust_sample_loaded` trace event.
#[repr(C)]
pub struct RustSampleLoadedArgs {
    pub magic_number: i32,
}

/// Trace event declaration corresponding to:
/// `TRACE_EVENT(rust_sample_loaded, TP_PROTO(int magic_number), ...)`.
///
/// The actual tracepoint registration, fast assignment, and printk handling
/// are provided by the kernel tracepoint implementation.
extern "C" {
    pub fn rust_sample_loaded(magic_number: i32);
}

// The C header's trace/define_trace.h include is intentionally omitted;
// it supplies build-time tracepoint definitions outside this declaration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
