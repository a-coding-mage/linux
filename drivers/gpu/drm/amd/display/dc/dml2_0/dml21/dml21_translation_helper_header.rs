// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Forward declarations supplied by the corresponding C dependencies.
#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dcn_watermarks {
    _private: [u8; 0],
}

#[repr(C)]
pub union dcn_watermark_set {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pipe_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_plane_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_configuration_options {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_initialize_instance_in_out {
    _private: [u8; 0],
}

// Types declared by other DML headers.
#[repr(C)]
pub struct dml2_per_plane_programming {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_pipe_configuration_descriptor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_per_stream_programming {
    _private: [u8; 0],
}

extern "C" {
    pub fn dml21_populate_dml_init_params(
        dml_init: *mut dml2_initialize_instance_in_out,
        config: *const dml2_configuration_options,
        in_dc: *const dc,
    );
    pub fn dml21_map_dc_state_into_dml_display_cfg(
        in_dc: *const dc,
        context: *mut dc_state,
        dml_ctx: *mut dml2_context,
    ) -> bool;
    pub fn dml21_copy_clocks_to_dc_state(in_ctx: *mut dml2_context, context: *mut dc_state);
    pub fn dml21_extract_watermark_sets(
        in_dc: *const dc,
        watermarks: *mut dcn_watermark_set,
        in_ctx: *mut dml2_context,
    );
    pub fn dml21_map_hw_resources(dml_ctx: *mut dml2_context);
    pub fn dml21_get_pipe_mcache_config(
        context: *mut dc_state,
        pipe_ctx: *mut pipe_ctx,
        pln_prog: *mut dml2_per_plane_programming,
        mcache_pipe_config: *mut dml2_pipe_configuration_descriptor,
    );
    pub fn dml21_set_dc_p_state_type(
        pipe_ctx: *mut pipe_ctx,
        stream_programming: *mut dml2_per_stream_programming,
        sub_vp_enabled: bool,
    );
    pub fn map_plane_to_dml21_display_cfg(
        dml_ctx: *const dml2_context,
        stream_id: u32,
        plane: *const dc_plane_state,
        context: *const dc_state,
    ) -> u32;
    pub fn dml21_init_min_clocks_for_dc_state(in_ctx: *mut dml2_context, context: *mut dc_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
