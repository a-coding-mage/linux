// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */

// The C source defines CREATE_TRACE_POINTS and includes ivpu_trace.h when
// __CHECKER__ is not defined. The corresponding trace-point declarations and
// definitions are supplied by the Rust translation of that dependency.
#[cfg(not(__CHECKER__))]
pub const CREATE_TRACE_POINTS: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
