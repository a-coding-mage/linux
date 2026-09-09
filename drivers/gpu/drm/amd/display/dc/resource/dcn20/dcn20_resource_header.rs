/*
 * Copyright 2017 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit:
// core_types.h and dml/dcn20/dcn20_fpu.h

#[allow(non_camel_case_types)]
pub struct dc;
#[allow(non_camel_case_types)]
pub struct resource_pool;
#[allow(non_camel_case_types)]
pub struct _vcs_dpi_display_pipe_params_st;

#[repr(C)]
pub struct dcn20_resource_pool {
    pub base: resource_pool,
}

// C macro: container_of(pool, struct dcn20_resource_pool, base)
#[macro_export]
macro_rules! TO_DCN20_RES_POOL {
    ($pool:expr) => {
        container_of!($pool, $crate::dcn20_resource_pool, base)
    };
}

extern "C" {
    pub static mut dcn2_0_ip: _vcs_dpi_ip_params_st;
    pub static mut dcn2_0_nv14_ip: _vcs_dpi_ip_params_st;
    pub static mut dcn2_0_soc: _vcs_dpi_soc_bounding_box_st;
    pub static mut dcn2_0_nv14_soc: _vcs_dpi_soc_bounding_box_st;
    pub static mut dcn2_0_nv12_soc: _vcs_dpi_soc_bounding_box_st;

    pub fn dcn20_create_resource_pool(
        init_data: *const dc_init_data,
        dc: *mut dc,
    ) -> *mut resource_pool;
    pub fn dcn20_link_encoder_create(
        ctx: *mut dc_context,
        enc_init_data: *const encoder_init_data,
    ) -> *mut link_encoder;
    pub fn dcn20_calc_max_scaled_time(
        time_per_pixel: ::core::ffi::c_uint,
        mode: mmhubbub_wbif_mode,
        urgent_watermark: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint;
    pub fn dcn20_acquire_free_pipe_for_layer(cur_ctx: *const dc_state, new_ctx: *mut dc_state, pool: *const resource_pool, opp_head_pipe: *const pipe_ctx) -> *mut pipe_ctx;
    pub fn dcn20_release_pipe(context: *mut dc_state, pipe: *mut pipe_ctx, pool: *const resource_pool);
    pub fn dcn20_stream_encoder_create(eng_id: engine_id, ctx: *mut dc_context) -> *mut stream_encoder;
    pub fn dcn20_hwseq_create(ctx: *mut dc_context) -> *mut dce_hwseq;
    pub fn dcn20_get_dcc_compression_cap(dc: *const dc, input: *const dc_dcc_surface_param, output: *mut dc_surface_dcc_cap) -> bool;
    pub fn dcn20_dpp_destroy(dpp: *mut *mut dpp);
    pub fn dcn20_dpp_create(ctx: *mut dc_context, inst: u32) -> *mut dpp;
    pub fn dcn20_ipp_create(ctx: *mut dc_context, inst: u32) -> *mut input_pixel_processor;
    pub fn dcn20_opp_create(ctx: *mut dc_context, inst: u32) -> *mut output_pixel_processor;
    pub fn dcn20_aux_engine_create(ctx: *mut dc_context, inst: u32) -> *mut dce_aux;
    pub fn dcn20_i2c_hw_create(ctx: *mut dc_context, inst: u32) -> *mut dce_i2c_hw;
    pub fn dcn20_clock_source_destroy(clk_src: *mut *mut clock_source);
    pub fn dcn20_dsc_create(ctx: *mut dc_context, inst: u32) -> *mut display_stream_compressor;
    pub fn dcn20_dsc_destroy(dsc: *mut *mut display_stream_compressor);
    pub fn dcn20_hubp_create(ctx: *mut dc_context, inst: u32) -> *mut hubp;
    pub fn dcn20_timing_generator_create(ctx: *mut dc_context, instance: u32) -> *mut timing_generator;
    pub fn dcn20_mpc_create(ctx: *mut dc_context) -> *mut mpc;
    pub fn dcn20_hubbub_create(ctx: *mut dc_context) -> *mut hubbub;
    pub fn dcn20_dwbc_create(ctx: *mut dc_context, pool: *mut resource_pool) -> bool;
    pub fn dcn20_mmhubbub_create(ctx: *mut dc_context, pool: *mut resource_pool) -> bool;
    pub fn dcn20_set_mcif_arb_params(dc: *mut dc, context: *mut dc_state, pipes: *mut display_e2e_pipe_params_st, pipe_cnt: ::core::ffi::c_int);
    pub fn dcn20_validate_bandwidth(dc: *mut dc, context: *mut dc_state, validate_mode: dc_validate_mode) -> dc_status;
    pub fn dcn20_merge_pipes_for_validate(dc: *mut dc, context: *mut dc_state);
    pub fn dcn20_validate_apply_pipe_split_flags(dc: *mut dc, context: *mut dc_state, vlevel: ::core::ffi::c_int, split: *mut ::core::ffi::c_int, merge: *mut bool) -> ::core::ffi::c_int;
    pub fn dcn20_release_dsc(res_ctx: *mut resource_context, pool: *const resource_pool, dsc: *mut *mut display_stream_compressor);
    pub fn dcn20_validate_dsc(dc: *mut dc, new_ctx: *mut dc_state) -> bool;
    pub fn dcn20_split_stream_for_mpc(res_ctx: *mut resource_context, pool: *const resource_pool, primary_pipe: *mut pipe_ctx, secondary_pipe: *mut pipe_ctx);
    pub fn dcn20_split_stream_for_odm(dc: *const dc, res_ctx: *mut resource_context, prev_odm_pipe: *mut pipe_ctx, next_odm_pipe: *mut pipe_ctx) -> bool;
    pub fn dcn20_acquire_dsc(dc: *const dc, res_ctx: *mut resource_context, dsc: *mut *mut display_stream_compressor, pipe_idx: ::core::ffi::c_int);
    pub fn dcn20_find_secondary_pipe(dc: *mut dc, res_ctx: *mut resource_context, pool: *const resource_pool, primary_pipe: *const pipe_ctx) -> *mut pipe_ctx;
    pub fn dcn20_fast_validate_bw(dc: *mut dc, context: *mut dc_state, pipes: *mut display_e2e_pipe_params_st, pipe_cnt_out: *mut ::core::ffi::c_int, pipe_split_from: *mut ::core::ffi::c_int, vlevel_out: *mut ::core::ffi::c_int, validate_mode: dc_validate_mode) -> bool;
    pub fn dcn20_build_mapped_resource(dc: *const dc, context: *mut dc_state, stream: *mut dc_stream_state) -> dc_status;
    pub fn dcn20_add_stream_to_ctx(dc: *mut dc, new_ctx: *mut dc_state, dc_stream: *mut dc_stream_state) -> dc_status;
    pub fn dcn20_add_dsc_to_stream_resource(dc: *mut dc, dc_ctx: *mut dc_state, dc_stream: *mut dc_stream_state) -> dc_status;
    pub fn dcn20_remove_stream_from_ctx(dc: *mut dc, new_ctx: *mut dc_state, dc_stream: *mut dc_stream_state) -> dc_status;
    pub fn dcn20_patch_unknown_plane_state(plane_state: *mut dc_plane_state) -> dc_status;
    pub fn dcn20_build_pipe_pix_clk_params(pipe_ctx: *mut pipe_ctx);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
