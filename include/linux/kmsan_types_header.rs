/* SPDX-License-Identifier: GPL-2.0 */
/*
 * A minimal header declaring types added by KMSAN to existing kernel structs.
 *
 * Copyright (C) 2017-2022 Google LLC
 * Author: Alexander Potapenko <glider@google.com>
 *
 */

// Dependency intent: types supplied by <linux/types.h>.

/* These constants are defined in the MSan LLVM instrumentation pass. */
pub const KMSAN_RETVAL_SIZE: usize = 800;
pub const KMSAN_PARAM_SIZE: usize = 800;

#[repr(C)]
pub struct kmsan_context_state {
    pub param_tls: [i8; KMSAN_PARAM_SIZE],
    pub retval_tls: [i8; KMSAN_RETVAL_SIZE],
    pub va_arg_tls: [i8; KMSAN_PARAM_SIZE],
    pub va_arg_origin_tls: [i8; KMSAN_PARAM_SIZE],
    pub va_arg_overflow_size_tls: u64,
    pub param_origin_tls: [i8; KMSAN_PARAM_SIZE],
    pub retval_origin_tls: u32,
}

#[repr(C)]
pub struct kmsan_ctx {
    pub cstate: kmsan_context_state,
    pub kmsan_in_runtime: i32,
    pub depth: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
