/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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

// C dependencies: "core_types.h" and "dml/dcn10/dcn10_fpu.h".

// #define TO_DCN10_RES_POOL(pool) container_of(pool, struct dcn10_resource_pool, base)
// The C container_of operation preserves the original pointer/container
// relationship and depends on the external struct layout.
#[inline]
pub unsafe fn to_dcn10_res_pool(pool: *mut resource_pool) -> *mut dcn10_resource_pool {
    (pool as *mut u8).sub(core::mem::offset_of!(dcn10_resource_pool, base))
        as *mut dcn10_resource_pool
}

pub struct dc;
pub struct resource_pool;
pub struct _vcs_dpi_display_pipe_params_st;

extern "C" {
    pub static mut dcn1_0_ip: _vcs_dpi_ip_params_st;
    pub static mut dcn1_0_soc: _vcs_dpi_soc_bounding_box_st;
}

#[repr(C)]
pub struct dcn10_resource_pool {
    pub base: resource_pool,
}

extern "C" {
    pub fn dcn10_create_resource_pool(
        init_data: *const dc_init_data,
        dc: *mut dc,
    ) -> *mut resource_pool;

    pub fn dcn10_find_first_free_match_stream_enc_for_link(
        res_ctx: *mut resource_context,
        pool: *const resource_pool,
        stream: *mut dc_stream_state,
    ) -> *mut stream_encoder;

    pub fn dcn10_get_vstartup_for_pipe(pipe_ctx: *mut pipe_ctx) -> ::core::ffi::c_uint;

    pub fn dcn10_get_default_tiling_info(tiling_info: *mut dc_tiling_info);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
