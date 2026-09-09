/*
 * Copyright 2012-16 Advanced Micro Devices, Inc.
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

// C header dependency: #include "dc.h"

/* functions shared by other dce clk mgrs */
extern "C" {
    pub fn dce_adjust_dp_ref_freq_for_ss(
        clk_mgr_dce: *mut struct clk_mgr_internal,
        dp_ref_clk_khz: i32,
    ) -> i32;

    pub fn dce_get_dp_ref_freq_khz(clk_mgr_base: *mut struct clk_mgr) -> i32;

    pub fn dce_get_max_pixel_clock_for_all_paths(context: *mut struct dc_state) -> u32;

    pub fn dce_clk_mgr_construct(
        ctx: *mut struct dc_context,
        clk_mgr_dce: *mut struct clk_mgr_internal,
    );

    pub fn dce_clock_read_ss_info(dccg_dce: *mut struct clk_mgr_internal);

    pub fn dce12_get_dp_ref_freq_khz(dccg: *mut struct clk_mgr) -> i32;

    pub fn dce_set_clock(
        clk_mgr_base: *mut struct clk_mgr,
        requested_clk_khz: i32,
    );

    pub fn dce_clk_mgr_destroy(clk_mgr: *mut *mut struct clk_mgr);

    pub fn dentist_get_divider_from_did(did: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
