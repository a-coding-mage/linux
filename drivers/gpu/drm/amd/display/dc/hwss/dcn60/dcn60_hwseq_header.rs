// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from dcn60_hwseq.h. Declarations supplied by included headers are
// referenced here and are expected to be provided by the surrounding crate.

use ::core::ffi::c_int;

#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pipe_ctx {
    _private: [u8; 0],
}

// C enum dc_status; its concrete definition is supplied by the translated
// core types header.
pub type dc_status = c_int;

extern "C" {
    pub fn dcn60_apply_ctx_to_hw(
        dc: *mut dc,
        context: *mut dc_state,
    ) -> dc_status;

    pub fn dcn60_apply_single_controller_ctx_to_hw(
        pipe_ctx: *mut pipe_ctx,
        context: *mut dc_state,
        dc: *mut dc,
    ) -> dc_status;

    pub fn dcn60_init_hw(dc: *mut dc);
    pub fn dcn60_set_cursor_attribute(pipe_ctx: *mut pipe_ctx);
    pub fn dcn60_update_cursor_offload_pipe(
        dc: *mut dc,
        pipe: *const pipe_ctx,
    );

    pub fn dcn60_program_perfmon(dc: *mut dc, context: *mut dc_state);
    pub fn dcn60_apply_idle_power_optimizations(
        dc: *mut dc,
        enable: bool,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
