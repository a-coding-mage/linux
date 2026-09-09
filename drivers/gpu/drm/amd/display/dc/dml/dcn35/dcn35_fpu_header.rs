/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependency corresponding to the original: #include "clk_mgr.h"

use core::ffi::c_int;

#[repr(C)]
pub struct clk_mgr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_bw_params {
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

// The complete definition is supplied by the corresponding external dependency.
pub type dc_validate_mode = c_int;

unsafe extern "C" {
    pub fn dcn35_build_wm_range_table_fpu(clk_mgr: *mut clk_mgr);

    pub fn dcn35_update_bw_bounding_box_fpu(
        dc: *mut dc,
        bw_params: *mut clk_bw_params,
    );

    pub fn dcn35_populate_dml_pipes_from_context_fpu(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        validate_mode: dc_validate_mode,
    ) -> c_int;

    pub fn dcn35_decide_zstate_support(dc: *mut dc, context: *mut dc_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
