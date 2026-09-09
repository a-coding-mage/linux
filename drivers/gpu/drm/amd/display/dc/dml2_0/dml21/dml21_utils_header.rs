// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from dml21_utils.h. C/C++ include and header-guard directives are
// intentionally omitted; referenced types and constants are supplied by the
// surrounding translation unit.

use core::ffi::c_int;

#[repr(C)]
pub struct dc_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_plane_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pipe_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_dmub_srv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_display_rq_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_display_dlg_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_display_ttu_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_per_stream_programming {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_per_plane_programming {
    _private: [u8; 0],
}

// The enum is defined by the DML2 dependency.
pub type dml2_source_format_class = u32;

unsafe extern "C" {
    pub fn dml21_helper_find_dml_pipe_idx_by_stream_id(
        ctx: *mut dml2_context,
        stream_id: u32,
    ) -> c_int;
    pub fn dml21_find_dml_pipe_idx_by_plane_id(
        ctx: *mut dml2_context,
        plane_id: u32,
    ) -> c_int;
    pub fn dml21_get_plane_id(
        state: *const dc_state,
        plane: *const dc_plane_state,
        plane_id: *mut u32,
    ) -> bool;
    pub fn dml21_pipe_populate_global_sync(
        dml_ctx: *mut dml2_context,
        context: *mut dc_state,
        pipe_ctx: *mut pipe_ctx,
        stream_programming: *mut dml2_per_stream_programming,
    );
    pub fn dml21_populate_mall_allocation_size(
        context: *mut dc_state,
        in_ctx: *mut dml2_context,
        pln_prog: *mut dml2_per_plane_programming,
        dc_pipe: *mut pipe_ctx,
    );
    pub fn check_dp2p0_output_encoder(pipe_ctx: *const pipe_ctx) -> bool;
    pub fn find_valid_pipe_idx_for_stream_index(
        dml_ctx: *const dml2_context,
        dml_pipe_idx: *mut u32,
        stream_index: u32,
    );
    pub fn find_pipe_regs_idx(
        dml_ctx: *const dml2_context,
        pipe: *mut pipe_ctx,
        pipe_regs_idx: *mut u32,
    );
    pub fn dml21_find_dc_pipes_for_plane(
        in_dc: *const dc,
        context: *mut dc_state,
        dml_ctx: *mut dml2_context,
        dc_main_pipes: *mut *mut pipe_ctx,
        dc_phantom_pipes: *mut *mut pipe_ctx,
        dml_plane_idx: c_int,
    ) -> c_int;
    pub fn dml21_program_dc_pipe(
        dml_ctx: *mut dml2_context,
        context: *mut dc_state,
        pipe_ctx: *mut pipe_ctx,
        pln_prog: *mut dml2_per_plane_programming,
        stream_prog: *mut dml2_per_stream_programming,
    );
    pub fn dml21_handle_phantom_streams_planes(
        in_dc: *const dc,
        context: *mut dc_state,
        dml_ctx: *mut dml2_context,
    );
    pub fn dml21_get_dc_plane_idx_from_plane_id(plane_id: u32) -> u32;
    pub fn dml21_build_fams2_programming(
        dc: *const dc,
        context: *mut dc_state,
        dml_ctx: *mut dml2_context,
    );
    pub fn dml21_is_plane1_enabled(source_format: dml2_source_format_class) -> bool;
    pub fn dml21_program_dc_mcif_arb_params(
        dml_ctx: *mut dml2_context,
        context: *mut dc_state,
        stream_prog: *mut dml2_per_stream_programming,
        wb_index: u32,
        dwb_inst: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
