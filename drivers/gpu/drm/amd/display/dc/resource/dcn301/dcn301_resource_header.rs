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
 *
 */

// C header guard: _DCN301_RESOURCE_H_

// Dependency supplied by the surrounding translation unit: core_types.h

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

unsafe extern "C" {
    pub static mut dcn3_01_ip: _vcs_dpi_ip_params_st;
    pub static mut dcn3_01_soc: _vcs_dpi_soc_bounding_box_st;
}

#[repr(C)]
pub struct dcn301_resource_pool {
    pub base: resource_pool,
}

unsafe extern "C" {
    pub fn dcn301_create_resource_pool(
        init_data: *const dc_init_data,
        dc: *mut dc,
    ) -> *mut resource_pool;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
