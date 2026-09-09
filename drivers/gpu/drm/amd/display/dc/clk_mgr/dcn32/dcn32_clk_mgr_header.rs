/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// Opaque types supplied by the surrounding display-clock-manager code.
#[repr(C)]
pub struct clk_mgr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_mgr_internal {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pp_smu_funcs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dccg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_state {
    _private: [u8; 0],
}

extern "C" {
    pub fn dcn32_init_clocks(clk_mgr_base: *mut clk_mgr);

    pub fn dcn32_clk_mgr_construct(
        ctx: *mut dc_context,
        clk_mgr: *mut clk_mgr_internal,
        pp_smu: *mut pp_smu_funcs,
        dccg: *mut dccg,
    );

    pub fn dcn32_update_clocks_update_dpp_dto(
        clk_mgr: *mut clk_mgr_internal,
        context: *mut dc_state,
        safe_to_lower: bool,
    );

    pub fn dcn32_clk_mgr_destroy(clk_mgr: *mut clk_mgr_internal);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
