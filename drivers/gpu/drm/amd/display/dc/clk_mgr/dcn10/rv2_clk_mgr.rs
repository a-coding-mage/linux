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
 *
 */

// Dependencies supplied by the corresponding C headers:
// #include "core_types.h"
// #include "clk_mgr_internal.h"
// #include "rv1_clk_mgr.h"
// #include "rv2_clk_mgr.h"
// #include "dce112/dce112_clk_mgr.h"

static mut rv2_clk_internal_funcs: clk_mgr_internal_funcs = clk_mgr_internal_funcs {
    set_dispclk: dce112_set_dispclk,
    set_dprefclk: dce112_set_dprefclk,
};

pub unsafe fn rv2_clk_mgr_construct(
    ctx: *mut dc_context,
    clk_mgr: *mut clk_mgr_internal,
    pp_smu: *mut pp_smu_funcs,
) {
    rv1_clk_mgr_construct(ctx, clk_mgr, pp_smu);

    (*clk_mgr).funcs = core::ptr::addr_of_mut!(rv2_clk_internal_funcs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
