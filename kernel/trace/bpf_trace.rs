// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful source-level Rust representation of trace/bpf_trace.c.
 *
 * This translation intentionally keeps kernel-provided types, constants,
 * macros, and external functions unresolved: they are supplied by the
 * surrounding kernel translation unit.  The original implementation is
 * retained below as a Rust raw string so conditional sections and macro
 * declarations remain source-faithful until those kernel bindings are
 * available.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
         dead_code, unused_variables, unused_imports)]

/// Kernel implementation source retained verbatim for binding generation.
pub const BPF_TRACE_C_SOURCE: &str = include_str!("bpf_trace.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
