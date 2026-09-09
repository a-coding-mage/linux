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

// C dependency: #include "core_types.h"

#[allow(non_camel_case_types)]
extern "C" {
    pub static mut dcn3_1_ip: _vcs_dpi_ip_params_st;

    pub fn dcn31_validate_bandwidth(
        dc: *mut dc,
        context: *mut dc_state,
        validate_mode: dc_validate_mode,
    ) -> dc_status;
    pub fn dcn31_calculate_wm_and_dlg(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: ::core::ffi::c_int,
        vlevel: ::core::ffi::c_int,
    );
    pub fn dcn31_populate_dml_pipes_from_context(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        validate_mode: dc_validate_mode,
    ) -> ::core::ffi::c_int;
    pub fn dcn31_populate_dml_writeback_from_context(
        dc: *mut dc,
        res_ctx: *mut resource_context,
        pipes: *mut display_e2e_pipe_params_st,
    );
    pub fn dcn31_set_mcif_arb_params(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: ::core::ffi::c_int,
    );
    pub fn dcn31_create_resource_pool(
        init_data: *const dc_init_data,
        dc: *mut dc,
    ) -> *mut resource_pool;
    pub fn dcn31_get_det_buffer_size(context: *const dc_state) -> ::core::ffi::c_uint;
    pub fn dcn31_update_dc_state_for_encoder_switch(
        link: *mut dc_link,
        link_setting: *mut dc_link_settings,
        pipe_count: u8,
        pipes: *mut pipe_ctx,
        audio_output: *mut audio_output,
    ) -> dc_status;
}

#[repr(C)]
pub struct dcn31_resource_pool {
    pub base: resource_pool,
}

// Equivalent of the C container_of-based macro; `container_of` is supplied by
// the surrounding translation unit.
#[macro_export]
macro_rules! TO_DCN31_RES_POOL {
    ($pool:expr) => {
        container_of!($pool, dcn31_resource_pool, base)
    };
}

// temp: B0 specific before switch to dcn313 headers
pub const regPHYPLLF_PIXCLK_RESYNC_CNTL: u32 = 0x007e;
pub const regPHYPLLF_PIXCLK_RESYNC_CNTL_BASE_IDX: u32 = 1;
pub const regPHYPLLG_PIXCLK_RESYNC_CNTL: u32 = 0x005f;
pub const regPHYPLLG_PIXCLK_RESYNC_CNTL_BASE_IDX: u32 = 1;

// PHYPLLF_PIXCLK_RESYNC_CNTL
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_PIXCLK_RESYNC_ENABLE__SHIFT: u32 = 0x0;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_DEEP_COLOR_DTO_ENABLE_STATUS__SHIFT: u32 = 0x1;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_DCCG_DEEP_COLOR_CNTL__SHIFT: u32 = 0x4;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_PIXCLK_ENABLE__SHIFT: u32 = 0x8;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_PIXCLK_DOUBLE_RATE_ENABLE__SHIFT: u32 = 0x9;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_PIXCLK_RESYNC_ENABLE_MASK: u32 = 0x00000001;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_DEEP_COLOR_DTO_ENABLE_STATUS_MASK: u32 = 0x00000002;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_DCCG_DEEP_COLOR_CNTL_MASK: u32 = 0x00000030;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_PIXCLK_ENABLE_MASK: u32 = 0x00000100;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_PIXCLK_DOUBLE_RATE_ENABLE_MASK: u32 = 0x00000200;

// PHYPLLG_PIXCLK_RESYNC_CNTL
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_PIXCLK_RESYNC_ENABLE__SHIFT: u32 = 0x0;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_DEEP_COLOR_DTO_ENABLE_STATUS__SHIFT: u32 = 0x1;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_DCCG_DEEP_COLOR_CNTL__SHIFT: u32 = 0x4;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_PIXCLK_ENABLE__SHIFT: u32 = 0x8;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_PIXCLK_DOUBLE_RATE_ENABLE__SHIFT: u32 = 0x9;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_PIXCLK_RESYNC_ENABLE_MASK: u32 = 0x00000001;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_DEEP_COLOR_DTO_ENABLE_STATUS_MASK: u32 = 0x00000002;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_DCCG_DEEP_COLOR_CNTL_MASK: u32 = 0x00000030;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_PIXCLK_ENABLE_MASK: u32 = 0x00000100;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_PIXCLK_DOUBLE_RATE_ENABLE_MASK: u32 = 0x00000200;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
