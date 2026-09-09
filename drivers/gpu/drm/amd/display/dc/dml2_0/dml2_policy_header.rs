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
 */

// Dependency supplied by display_mode_core_structs.h in the C source.

#[repr(C)]
pub struct dml2_policy_build_synthetic_soc_states_params {
    pub in_bbox: *const soc_bounding_box_st,
    pub in_states: *mut soc_states_st,
    pub out_states: *mut soc_states_st,
    pub dcfclk_stas_mhz: *mut ::core::ffi::c_int,
    pub num_dcfclk_stas: ::core::ffi::c_int,
}

#[repr(C)]
pub struct dml2_policy_build_synthetic_soc_states_scratch {
    pub entry: soc_state_bounding_box_st,
}

extern "C" {
    pub fn dml2_policy_build_synthetic_soc_states(
        s: *mut dml2_policy_build_synthetic_soc_states_scratch,
        p: *mut dml2_policy_build_synthetic_soc_states_params,
    ) -> ::core::ffi::c_int;

    pub fn build_unoptimized_policy_settings(
        project: dml_project_id,
        policy: *mut dml_mode_eval_policy_st,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
