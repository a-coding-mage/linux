// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018, 2019 Oracle. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/RPC implementation:
// linux/sunrpc/clnt.h
// linux/sunrpc/sched.h
// linux/sunrpc/svc.h
// linux/sunrpc/svc_xprt.h
// linux/sunrpc/auth_gss.h
// linux/sunrpc/gss_err.h

// The C CREATE_TRACE_POINTS definition causes the RPC GSS tracepoint
// definitions from trace/events/rpcgss.h to be emitted in this translation
// unit. The corresponding Rust tracepoint declarations are supplied by the
// surrounding implementation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
