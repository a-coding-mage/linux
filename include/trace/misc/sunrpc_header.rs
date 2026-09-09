/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2021 Oracle and/or its affiliates.
 *
 * Common types and format specifiers for sunrpc.
 */

// Dependency intent: <linux/tracepoint.h>

pub const SUNRPC_TRACE_PID_SPECIFIER: &str = "%08x";
pub const SUNRPC_TRACE_CLID_SPECIFIER: &str = "%08x";
pub const SUNRPC_TRACE_TASK_SPECIFIER: &str =
    concat!("task:", SUNRPC_TRACE_PID_SPECIFIER, "@", SUNRPC_TRACE_CLID_SPECIFIER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
