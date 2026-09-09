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

// C header guard: __DCN20_CLK_MGR_H__

unsafe extern "C" {
    pub fn dcn2_update_clocks(
        dccg: *mut clk_mgr,
        context: *mut dc_state,
        safe_to_lower: bool,
    );

    pub fn dcn2_update_clocks_fpga(
        clk_mgr: *mut clk_mgr,
        context: *mut dc_state,
        safe_to_lower: bool,
    );

    pub fn dcn20_update_clocks_update_dpp_dto(
        clk_mgr: *mut clk_mgr_internal,
        context: *mut dc_state,
        safe_to_lower: bool,
    );

    pub fn dcn2_init_clocks(clk_mgr: *mut clk_mgr);

    pub fn dcn20_clk_mgr_construct(
        ctx: *mut dc_context,
        clk_mgr: *mut clk_mgr_internal,
        pp_smu: *mut pp_smu_funcs,
        dccg: *mut dccg,
    );

    pub fn dentist_get_did_from_divider(divider: i32) -> u32;

    pub fn dcn2_get_clock(
        clk_mgr: *mut clk_mgr,
        context: *mut dc_state,
        clock_type: dc_clock_type,
        clock_cfg: *mut dc_clock_config,
    );

    pub fn dcn20_update_clocks_update_dentist(
        clk_mgr: *mut clk_mgr_internal,
        context: *mut dc_state,
    );

    pub fn dcn2_read_clocks_from_hw_dentist(clk_mgr_base: *mut clk_mgr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
