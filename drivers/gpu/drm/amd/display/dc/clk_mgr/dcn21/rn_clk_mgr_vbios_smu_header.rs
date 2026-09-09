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

// C forward declaration: enum dcn_pwr_state;
#[repr(C)]
pub enum dcn_pwr_state {}

// Supplied by the surrounding C/Rust translation unit.
#[repr(C)]
pub struct clk_mgr_internal {
    _opaque: [u8; 0],
}

extern "C" {
    pub fn rn_vbios_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal) -> i32;
    pub fn rn_vbios_smu_set_dispclk(
        clk_mgr: *mut clk_mgr_internal,
        requested_dispclk_khz: i32,
    ) -> i32;
    pub fn rn_vbios_smu_set_hard_min_dcfclk(
        clk_mgr: *mut clk_mgr_internal,
        requested_dcfclk_khz: i32,
    ) -> i32;
    pub fn rn_vbios_smu_set_min_deep_sleep_dcfclk(
        clk_mgr: *mut clk_mgr_internal,
        requested_min_ds_dcfclk_khz: i32,
    ) -> i32;
    pub fn rn_vbios_smu_set_phyclk(
        clk_mgr: *mut clk_mgr_internal,
        requested_phyclk_khz: i32,
    );
    pub fn rn_vbios_smu_set_dppclk(
        clk_mgr: *mut clk_mgr_internal,
        requested_dpp_khz: i32,
    ) -> i32;
    pub fn rn_vbios_smu_set_dcn_low_power_state(
        clk_mgr: *mut clk_mgr_internal,
        state: dcn_pwr_state,
    );
    pub fn rn_vbios_smu_enable_48mhz_tmdp_refclk_pwrdwn(
        clk_mgr: *mut clk_mgr_internal,
        enable: bool,
    );
    pub fn rn_vbios_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal);
    pub fn rn_vbios_smu_is_periodic_retraining_disabled(
        clk_mgr: *mut clk_mgr_internal,
    ) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
