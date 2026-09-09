/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2024 Trond Myklebust <trond.myklebust@hammerspace.com>
 * Copyright (C) 2024 Mike Snitzer <snitzer@hammerspace.com>
 */

// TRACE_SYSTEM: nfs_localio
// The original include guard is intentionally omitted; Rust modules provide
// equivalent item-level inclusion protection.

// Supplied by the tracepoint dependencies included by the original header.
// The concrete definitions and tracepoint machinery remain external.

/// Event class corresponding to `nfs_local_client_event`.
///
/// C source equivalent:
///
/// ```c
/// TP_PROTO(const struct nfs_client *clp)
/// TP_STRUCT__entry(
///     __field(unsigned int, protocol)
///     __string(server, clp->cl_hostname)
/// )
/// TP_fast_assign(
///     __entry->protocol = clp->rpc_ops->version;
///     __assign_str(server);
/// )
/// TP_printk("server=%s NFSv%u", __get_str(server), __entry->protocol)
/// ```
//
// This is a tracepoint declaration rather than a Rust-callable function; the
// tracepoint implementation and the definitions of `nfs_client` and its RPC
// operations are provided by external dependencies.
pub mod nfs_local_client_event {
    /// Tracepoint entry layout: `protocol` followed by the variable `server`
    /// string captured from `clp->cl_hostname`.
    #[repr(C)]
    pub struct Entry {
        pub protocol: ::core::ffi::c_uint,
        // `server` is a C tracepoint string field with variable storage.
    }
}

/// Declaration of the `nfs_localio_enable_client` trace event.
// Equivalent to DEFINE_EVENT(nfs_local_client_event, ...).
pub const NFS_LOCALIO_ENABLE_CLIENT: &str = "nfs_localio_enable_client";

/// Declaration of the `nfs_localio_disable_client` trace event.
// Equivalent to DEFINE_EVENT(nfs_local_client_event, ...).
pub const NFS_LOCALIO_DISABLE_CLIENT: &str = "nfs_localio_disable_client";

// The original TRACE_INCLUDE_PATH is `.` and TRACE_INCLUDE_FILE is
// `localio_trace`; trace/define_trace.h supplies the generated tracepoints.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
