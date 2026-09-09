/* Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Dependency: ../dce/dce_transform.h

pub const LB_TOTAL_NUMBER_OF_ENTRIES: i32 = 1712;
pub const LB_BITS_PER_ENTRY: i32 = 144;

pub unsafe extern "C" fn dce110_transform_v_construct(
    xfm110: *mut dce_transform,
    ctx: *mut dc_context,
) -> bool;

pub unsafe extern "C" fn dce110_opp_v_set_csc_default(
    xfm: *mut transform,
    default_adjust: *const default_adjustment,
);

pub unsafe extern "C" fn dce110_opp_v_set_csc_adjustment(
    xfm: *mut transform,
    tbl_entry: *const out_csc_color_matrix,
);

pub unsafe extern "C" fn dce110_opp_program_regamma_pwl_v(
    xfm: *mut transform,
    params: *const pwl_params,
);

pub unsafe extern "C" fn dce110_opp_power_on_regamma_lut_v(
    xfm: *mut transform,
    power_on: bool,
);

pub unsafe extern "C" fn dce110_opp_set_regamma_mode_v(
    xfm: *mut transform,
    mode: opp_regamma,
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
