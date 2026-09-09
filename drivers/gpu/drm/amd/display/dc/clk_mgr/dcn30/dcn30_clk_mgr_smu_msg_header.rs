/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// Dependency supplied by core_types.h.

#[repr(C)]
pub struct clk_mgr_internal {
    _private: [u8; 0],
}

extern "C" {
    pub fn dcn30_smu_test_message(
        clk_mgr: *mut clk_mgr_internal,
        input: u32,
    ) -> bool;
    pub fn dcn30_smu_get_smu_version(
        clk_mgr: *mut clk_mgr_internal,
        version: *mut i32,
    ) -> bool;
    pub fn dcn30_smu_check_driver_if_version(clk_mgr: *mut clk_mgr_internal) -> bool;
    pub fn dcn30_smu_check_msg_header_version(clk_mgr: *mut clk_mgr_internal) -> bool;
    pub fn dcn30_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32);
    pub fn dcn30_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32);
    pub fn dcn30_smu_transfer_wm_table_smu_2_dram(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn30_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn30_smu_set_hard_min_by_freq(
        clk_mgr: *mut clk_mgr_internal,
        clk: u32,
        freq_mhz: u16,
    ) -> u32;
    pub fn dcn30_smu_set_hard_max_by_freq(
        clk_mgr: *mut clk_mgr_internal,
        clk: u32,
        freq_mhz: u16,
    ) -> u32;
    pub fn dcn30_smu_get_dpm_freq_by_index(
        clk_mgr: *mut clk_mgr_internal,
        clk: u32,
        dpm_level: u8,
    ) -> u32;
    pub fn dcn30_smu_get_dc_mode_max_dpm_freq(clk_mgr: *mut clk_mgr_internal, clk: u32) -> u32;
    pub fn dcn30_smu_set_min_deep_sleep_dcef_clk(clk_mgr: *mut clk_mgr_internal, freq_mhz: u32);
    pub fn dcn30_smu_set_num_of_displays(clk_mgr: *mut clk_mgr_internal, num_displays: u32);
    pub fn dcn30_smu_set_display_refresh_from_mall(
        clk_mgr: *mut clk_mgr_internal,
        enable: bool,
        cache_timer_delay: u8,
        cache_timer_scale: u8,
    );
    pub fn dcn30_smu_set_external_client_df_cstate_allow(
        clk_mgr: *mut clk_mgr_internal,
        enable: bool,
    );
    pub fn dcn30_smu_set_pme_workaround(clk_mgr: *mut clk_mgr_internal);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
