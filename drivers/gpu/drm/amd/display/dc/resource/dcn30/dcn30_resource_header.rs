/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
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

// Dependency equivalent of: #include "core_types.h"

#[allow(non_camel_case_types)]
pub type c_uint = ::core::ffi::c_uint;

// Forward declarations supplied by dependent translation units.
pub struct dc { _private: [u8; 0] }
pub struct resource_pool { _private: [u8; 0] }
pub struct _vcs_dpi_display_pipe_params_st { _private: [u8; 0] }
pub struct _vcs_dpi_ip_params_st { _private: [u8; 0] }
pub struct _vcs_dpi_soc_bounding_box_st { _private: [u8; 0] }
pub struct dc_init_data { _private: [u8; 0] }
pub struct dc_state { _private: [u8; 0] }
pub struct resource_context { _private: [u8; 0] }
pub struct dc_3dlut { _private: [u8; 0] }
pub struct dc_transfer_func { _private: [u8; 0] }
pub struct dc_stream_state { _private: [u8; 0] }
pub struct clk_bw_params { _private: [u8; 0] }
pub struct display_e2e_pipe_params_st { _private: [u8; 0] }

// External types supplied by core_types.h.
pub type mmhubbub_wbif_mode = u32;
pub type dc_validate_mode = u32;
pub type dc_status = u32;

#[repr(C)]
pub struct dcn30_resource_pool {
    pub base: resource_pool,
}

// Equivalent of container_of(pool, struct dcn30_resource_pool, base).
#[macro_export]
macro_rules! TO_DCN30_RES_POOL {
    ($pool:expr) => {{
        ($pool as *mut _ as *mut u8)
            .wrapping_sub(::core::mem::offset_of!($crate::dcn30_resource_pool, base))
            as *mut $crate::dcn30_resource_pool
    }};
}

unsafe extern "C" {
    pub static mut dcn3_0_ip: _vcs_dpi_ip_params_st;
    pub static mut dcn3_0_soc: _vcs_dpi_soc_bounding_box_st;

    pub fn dcn30_create_resource_pool(
        init_data: *const dc_init_data,
        dc: *mut dc,
    ) -> *mut resource_pool;

    pub fn dcn30_set_mcif_arb_params(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: ::core::ffi::c_int,
    );

    pub fn dcn30_calc_max_scaled_time(
        time_per_pixel: c_uint,
        mode: mmhubbub_wbif_mode,
        urgent_watermark: c_uint,
    ) -> c_uint;

    pub fn dcn30_validate_bandwidth(
        dc: *mut dc,
        context: *mut dc_state,
        validate_mode: dc_validate_mode,
    ) -> dc_status;

    pub fn dcn30_internal_validate_bw(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt_out: *mut ::core::ffi::c_int,
        vlevel_out: *mut ::core::ffi::c_int,
        validate_mode: dc_validate_mode,
        allow_self_refresh_only: bool,
    ) -> bool;

    pub fn dcn30_calculate_wm_and_dlg(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: ::core::ffi::c_int,
        vlevel: ::core::ffi::c_int,
    );

    pub fn dcn30_update_soc_for_wm_a(dc: *mut dc, context: *mut dc_state);

    pub fn dcn30_populate_dml_writeback_from_context(
        dc: *mut dc,
        res_ctx: *mut resource_context,
        pipes: *mut display_e2e_pipe_params_st,
    );

    pub fn dcn30_populate_dml_pipes_from_context(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        validate_mode: dc_validate_mode,
    ) -> ::core::ffi::c_int;

    pub fn dcn30_acquire_post_bldn_3dlut(
        res_ctx: *mut resource_context,
        pool: *const resource_pool,
        mpcc_id: ::core::ffi::c_int,
        lut: *mut *mut dc_3dlut,
        shaper: *mut *mut dc_transfer_func,
    ) -> bool;

    pub fn dcn30_release_post_bldn_3dlut(
        res_ctx: *mut resource_context,
        pool: *const resource_pool,
        lut: *mut *mut dc_3dlut,
        shaper: *mut *mut dc_transfer_func,
    ) -> bool;

    pub fn dcn30_add_stream_to_ctx(
        dc: *mut dc,
        new_ctx: *mut dc_state,
        dc_stream: *mut dc_stream_state,
    ) -> dc_status;

    pub fn dcn30_update_bw_bounding_box(dc: *mut dc, bw_params: *mut clk_bw_params);

    pub fn dcn30_can_support_mclk_switch_using_fw_based_vblank_stretch(
        dc: *mut dc,
        context: *mut dc_state,
    ) -> bool;

    pub fn dcn30_setup_mclk_switch_using_fw_based_vblank_stretch(
        dc: *mut dc,
        context: *mut dc_state,
    );

    pub fn dcn30_find_dummy_latency_index_for_fw_based_mclk_switch(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: ::core::ffi::c_int,
        vlevel: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
