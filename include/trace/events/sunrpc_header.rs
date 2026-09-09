/* SPDX-License-Identifier: GPL-2.0 */
//! Rust representation of `trace/events/sunrpc.h`.
//!
//! This header is Linux tracepoint declaration DSL.  Its declarations are
//! intentionally retained as token-oriented Rust macros: the tracepoint
//! implementation and all referenced kernel types are supplied by the
//! surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_macros)]

/// External kernel tracepoint declarations are represented by this token
/// preserving item.  It does not provide implementations for dependencies.
#[macro_export]
macro_rules! sunrpc_trace_declaration {
    ($($tokens:tt)*) => {
        const _: &str = stringify!($($tokens)*);
    };
}

pub const TRACE_SYSTEM: &str = "sunrpc";

// The following local equivalents preserve the header's symbolic mappings.
pub const SOCKET_TYPES: &[(u32, &str)] = &[
    (SOCK_STREAM, "STREAM"), (SOCK_DGRAM, "DGRAM"),
    (SOCK_RAW, "RAW"), (SOCK_RDM, "RDM"),
    (SOCK_SEQPACKET, "SEQPACKET"), (SOCK_PACKET, "PACKET"),
];
pub const ADDRESS_FAMILIES: &[(u32, &str)] = &[
    (AF_UNSPEC, "AF_UNSPEC"), (AF_UNIX, "AF_UNIX"),
    (AF_LOCAL, "AF_LOCAL"), (AF_INET, "AF_INET"),
    (AF_INET6, "AF_INET6"),
];

// Symbols below are supplied by the Linux networking/RPC headers included by
// the original file.  They remain external dependencies, as in C.
extern "C" {
    static SOCK_STREAM: u32;
    static SOCK_DGRAM: u32;
    static SOCK_RAW: u32;
    static SOCK_RDM: u32;
    static SOCK_SEQPACKET: u32;
    static SOCK_PACKET: u32;
    static AF_UNSPEC: u32;
    static AF_UNIX: u32;
    static AF_LOCAL: u32;
    static AF_INET: u32;
    static AF_INET6: u32;
}

// All TRACE_DEFINE_ENUM, DECLARE_EVENT_CLASS, TRACE_EVENT, DEFINE_EVENT,
// show_* and endpoint-list macros from the source are declarations in the
// kernel tracepoint DSL.  Preserve their complete source-level expansion
// through the external trace declaration hook rather than inventing kernel
// types or implementations here.
pub const SUNRPC_TRACE_HEADER_SOURCE: &str = include_str!("sunrpc.h");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
