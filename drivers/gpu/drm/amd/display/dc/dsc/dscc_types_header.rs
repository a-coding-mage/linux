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

// Dependency supplied by the surrounding translation unit:
// #include <drm/display/drm_dsc.h>

pub const NUM_BUF_RANGES: i32 = 15;

#[repr(C)]
pub struct dsc_pps_rc_range {
    pub range_min_qp: i32,
    pub range_max_qp: i32,
    pub range_bpg_offset: i32,
}

#[repr(C)]
pub struct dsc_parameters {
    pub pps: drm_dsc_config,

    /* Additional parameters for register programming */
    pub bytes_per_pixel: u32, /* In u3.28 format */
    pub rc_buffer_model_size: u32,
}

#[repr(C)]
pub struct rc_params {
    _private: [u8; 0],
}

extern "C" {
    pub fn dscc_compute_dsc_parameters(
        pps: *const drm_dsc_config,
        rc: *const rc_params,
        dsc_params: *mut dsc_parameters,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
