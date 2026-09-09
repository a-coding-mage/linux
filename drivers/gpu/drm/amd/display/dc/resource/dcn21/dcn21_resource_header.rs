/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// Dependency supplied by the surrounding translation unit: "core_types.h".

#[macro_export]
macro_rules! TO_DCN21_RES_POOL {
    ($pool:expr) => {
        container_of!($pool, dcn21_resource_pool, base)
    };
}

#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource_pool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_display_pipe_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_ip_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _vcs_dpi_soc_bounding_box_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_init_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct display_e2e_pipe_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dcn21_resource_pool {
    pub base: resource_pool,
}

extern "C" {
    pub static mut dcn2_1_ip: _vcs_dpi_ip_params_st;
    pub static mut dcn2_1_soc: _vcs_dpi_soc_bounding_box_st;

    pub fn dcn21_create_resource_pool(
        init_data: *const dc_init_data,
        dc: *mut dc,
    ) -> *mut resource_pool;

    pub fn dcn21_fast_validate_bw(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt_out: *mut i32,
        pipe_split_from: *mut i32,
        vlevel_out: *mut i32,
        validate_mode: dc_validate_mode,
        allow_self_refresh_only: bool,
    ) -> bool;
}

// Supplied by core_types.h.
#[allow(non_camel_case_types)]
pub type dc_validate_mode = ::core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
