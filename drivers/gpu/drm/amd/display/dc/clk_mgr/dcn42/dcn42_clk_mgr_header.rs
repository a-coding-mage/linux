/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
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
 */

// Dependency declarations from clk_mgr_internal.h are supplied externally.

pub const NUM_CLOCK_SOURCES: usize = 5;
pub const DCN42_CLKIP_REFCLK: u32 = 48000;

pub struct dcn42_watermarks;

#[repr(C)]
pub struct dcn42_smu_watermark_set {
    pub wm_set: *mut dcn42_watermarks,
    pub mc_address: large_integer,
}

#[repr(C)]
pub struct dcn42_ss_info_table {
    pub ss_divider: u32,
    pub ss_percentage: [u32; NUM_CLOCK_SOURCES],
}

#[repr(C)]
pub struct clk_mgr_dcn42 {
    pub base: clk_mgr_internal,
    pub smu_wm_set: dcn42_smu_watermark_set,
}

extern "C" {
    pub fn dcn42_are_clock_states_equal(a: *mut dc_clocks, b: *mut dc_clocks) -> bool;
    pub fn dcn42_init_clocks(clk_mgr: *mut clk_mgr);
    pub fn dcn42_update_clocks(
        clk_mgr_base: *mut clk_mgr,
        context: *mut dc_state,
        safe_to_lower: bool,
    );

    pub fn dcn42_clk_mgr_construct(
        ctx: *mut dc_context,
        clk_mgr: *mut clk_mgr_dcn42,
        pp_smu: *mut pp_smu_funcs,
        dccg: *mut dccg,
    );

    pub fn dcn42_clk_mgr_destroy(clk_mgr_int: *mut clk_mgr_internal);

    pub fn dcn42_init_single_clock(
        entry_0: *mut core::ffi::c_uint,
        smu_entry_0: *mut u32,
        num_levels: u8,
    );
    pub fn dcn42_convert_wck_ratio(wck_ratio: u8) -> core::ffi::c_uint;
    pub static mut dcn42_ss_info_table: dcn42_ss_info_table;
    pub fn dcn42_build_watermark_ranges(
        bw_params: *mut clk_bw_params,
        table: *mut dcn42_watermarks,
    );
    pub fn dcn42_enable_pme_wa(clk_mgr_base: *mut clk_mgr);
    pub fn dcn42_notify_cstate_disable(clk_mgr_base: *mut clk_mgr, disable: bool);
    pub fn dcn42_notify_wm_ranges(clk_mgr_base: *mut clk_mgr);
    pub fn dcn42_set_low_power_state(clk_mgr_base: *mut clk_mgr);
    pub fn dcn42_exit_low_power_state(clk_mgr_base: *mut clk_mgr);
    pub fn dcn42_get_max_clock_khz(clk_mgr_base: *mut clk_mgr, clk_type: clk_type) -> core::ffi::c_uint;
    pub fn dcn42_is_smu_present(clk_mgr_base: *mut clk_mgr) -> bool;
    pub fn dcn42_has_active_display(dc: *mut dc, context: *const dc_state) -> bool;
    pub fn dcn42_get_active_display_cnt_wa(
        dc: *mut dc,
        context: *mut dc_state,
        all_active_disps: *mut core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn dcn42_update_clocks_update_dpp_dto(
        clk_mgr: *mut clk_mgr_internal,
        context: *mut dc_state,
        safe_to_lower: bool,
    );
    pub fn dcn42_update_clocks_update_dtb_dto(
        clk_mgr: *mut clk_mgr_internal,
        context: *mut dc_state,
        ref_dtbclk_khz: core::ffi::c_int,
    );
    pub fn dcn42_is_spll_ssc_enabled(clk_mgr_base: *mut clk_mgr) -> bool;
    pub fn dcn42_get_dpm_table_from_smu(
        clk_mgr: *mut clk_mgr_internal,
        smu_dpm_clks: *mut dcn42_smu_dpm_clks,
    );
    pub fn dcn42_get_smu_clocks(clk_mgr_int: *mut clk_mgr_internal);
    pub fn dcn42_update_clocks_fpga(
        clk_mgr: *mut clk_mgr,
        context: *mut dc_state,
        safe_to_lower: bool,
    );
    pub fn dcn42_get_dispclk_from_dentist(clk_mgr_base: *mut clk_mgr) -> core::ffi::c_int;
    pub fn dcn42_request_dtbclk(clk_mgr_base: *mut clk_mgr, enable: bool);
}

// Forward declaration for pointer parameter below.
pub struct dcn42_smu_dpm_clks;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
