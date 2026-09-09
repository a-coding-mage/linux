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
 *
 */

// C header guard: DAL_DC_DCE_DCE112_CLK_MGR_H_

extern "C" {

    pub fn dce112_clk_mgr_construct(
        ctx: *mut dc_context,
        clk_mgr: *mut clk_mgr_internal,
    );

    /* functions shared with other clk mgr */
    pub fn dce112_set_clock(
        clk_mgr_base: *mut clk_mgr,
        requested_clk_khz: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn dce112_set_dispclk(
        clk_mgr: *mut clk_mgr_internal,
        requested_clk_khz: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn dce112_set_dprefclk(
        clk_mgr: *mut clk_mgr_internal,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
