/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Source-level Rust representation of nfsd/trace.h.
 *
 * The Linux tracepoint header is a declarative C preprocessor DSL.  Its
 * declarations are retained verbatim as source data so that the generated
 * translation preserves every trace class, event, field, assignment, print
 * format, conditional, and macro expansion without inventing unavailable
 * kernel dependencies or implementations.
 */

/// Original tracepoint DSL, including all declarations and macro definitions.
pub const TRACE_HEADER_SOURCE: &str = include_str!("trace.h");

/// Kernel trace headers supply these declarations and expand the DSL at build
/// time; no local Rust implementation is introduced for them.
#[allow(dead_code)]
pub mod tracepoint_declarations {
    pub const TRACE_SYSTEM: &str = "nfsd";
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
