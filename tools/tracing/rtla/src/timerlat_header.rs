// SPDX-License-Identifier: GPL-2.0

// C dependency intent: #include "osnoise.h"

use core::ffi::{c_char, c_int};

/*
 * Define timerlat tracing mode.
 *
 * There are three tracing modes:
 * - tracefs-only, used when BPF is unavailable.
 * - BPF-only, used when BPF is available and neither trace saving nor
 * auto-analysis are enabled.
 * - mixed mode, used when BPF is available and either trace saving or
 * auto-analysis is enabled (which rely on sample collection through
 * tracefs).
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum timerlat_tracing_mode {
    TRACING_MODE_BPF,
    TRACING_MODE_TRACEFS,
    TRACING_MODE_MIXED,
}

#[repr(C)]
pub struct timerlat_params {
    pub common: common_params,
    pub timerlat_period_us: i64,
    pub print_stack: i64,
    pub dma_latency: c_int,
    pub no_aa: bool,
    pub dump_tasks: bool,
    pub deepest_idle_state: c_int,
    pub mode: timerlat_tracing_mode,
    pub bpf_action_program: *const c_char,
    pub stack_format: stack_format,
    pub timerlat_align: bool,
    pub timerlat_align_us: u64,
}

// #define to_timerlat_params(ptr) container_of(ptr, struct timerlat_params, common)
#[inline]
pub unsafe fn to_timerlat_params(ptr: *mut common_params) -> *mut timerlat_params {
    (ptr as *mut u8).sub(core::mem::offset_of!(timerlat_params, common)) as *mut timerlat_params
}

unsafe extern "C" {
    pub fn timerlat_apply_config(
        tool: *mut osnoise_tool,
        params: *mut timerlat_params,
    ) -> c_int;
    pub fn timerlat_main(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn timerlat_enable(tool: *mut osnoise_tool) -> c_int;
    pub fn timerlat_analyze(tool: *mut osnoise_tool, stopped: bool);
    pub fn timerlat_free(tool: *mut osnoise_tool);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
