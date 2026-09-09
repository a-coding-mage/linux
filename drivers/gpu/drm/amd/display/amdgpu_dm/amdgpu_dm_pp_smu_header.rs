/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Dependency declarations supplied by dm_pp_interface.h and related headers.

#[repr(C)]
pub struct amd_pp_display_configuration {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pp_smu_wm_range_sets {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dm_pp_wm_sets_with_clock_ranges_soc15 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn amdgpu_dm_smu_write_watermarks_table(adev: *mut amdgpu_device) -> ::core::ffi::c_int;
}

// The following declarations are enabled when CONFIG_DRM_AMD_DC_KUNIT_TEST is enabled.
#[cfg(feature = "CONFIG_DRM_AMD_DC_KUNIT_TEST")]
extern "C" {
    pub fn build_pm_display_cfg(
        pm_display_cfg: *mut amd_pp_display_configuration,
        pp_display_cfg: *const dm_pp_display_configuration,
    );
    pub fn build_wm_clock_ranges_soc15(
        ranges: *const pp_smu_wm_range_sets,
        wm_with_clock_ranges: *mut dm_pp_wm_sets_with_clock_ranges_soc15,
    );
    pub fn get_default_clock_levels(
        clk_type: dm_pp_clock_type,
        clks: *mut dm_pp_clock_levels,
    );
    pub fn dc_to_pp_clock_type(dm_pp_clk_type: dm_pp_clock_type) -> amd_pp_clock_type;
    pub fn pp_to_dc_clock_levels(
        pp_clks: *const amd_pp_clocks,
        dc_clks: *mut dm_pp_clock_levels,
        dc_clk_type: dm_pp_clock_type,
    );
    pub fn pp_to_dc_clock_levels_with_latency(
        pp_clks: *const pp_clock_levels_with_latency,
        clk_level_info: *mut dm_pp_clock_levels_with_latency,
        dc_clk_type: dm_pp_clock_type,
    );
    pub fn pp_to_dc_clock_levels_with_voltage(
        pp_clks: *const pp_clock_levels_with_voltage,
        clk_level_info: *mut dm_pp_clock_levels_with_voltage,
        dc_clk_type: dm_pp_clock_type,
    );
    pub fn cap_clock_levels_to_validation(
        dc_clks: *mut dm_pp_clock_levels,
        clk_type: dm_pp_clock_type,
        validation_clks: *const amd_pp_simple_clock_info,
    );
    pub fn pp_smu_nv_clock_id_to_pp(
        clock_id: pp_smu_nv_clock_id,
        clock_type: *mut amd_pp_clock_type,
    ) -> bool;
    pub fn pp_rv_set_wm_ranges(pp: *mut pp_smu, ranges: *mut pp_smu_wm_range_sets);
    pub fn pp_rv_set_pme_wa_enable(pp: *mut pp_smu);
    pub fn pp_rv_set_active_display_count(pp: *mut pp_smu, count: ::core::ffi::c_int);
    pub fn pp_rv_set_min_deep_sleep_dcfclk(pp: *mut pp_smu, clock: ::core::ffi::c_int);
    pub fn pp_rv_set_hard_min_dcefclk_by_freq(pp: *mut pp_smu, clock: ::core::ffi::c_int);
    pub fn pp_rv_set_hard_min_fclk_by_freq(pp: *mut pp_smu, mhz: ::core::ffi::c_int);
    pub fn pp_nv_set_wm_ranges(
        pp: *mut pp_smu,
        ranges: *mut pp_smu_wm_range_sets,
    ) -> pp_smu_status;
    pub fn pp_nv_set_display_count(pp: *mut pp_smu, count: ::core::ffi::c_int) -> pp_smu_status;
    pub fn pp_nv_set_min_deep_sleep_dcfclk(pp: *mut pp_smu, mhz: ::core::ffi::c_int)
        -> pp_smu_status;
    pub fn pp_nv_set_hard_min_dcefclk_by_freq(pp: *mut pp_smu, mhz: ::core::ffi::c_int)
        -> pp_smu_status;
    pub fn pp_nv_set_hard_min_uclk_by_freq(pp: *mut pp_smu, mhz: ::core::ffi::c_int)
        -> pp_smu_status;
    pub fn pp_nv_set_pstate_handshake_support(
        pp: *mut pp_smu,
        pstate_handshake_supported: bool,
    ) -> pp_smu_status;
    pub fn pp_nv_set_voltage_by_freq(
        pp: *mut pp_smu,
        clock_id: pp_smu_nv_clock_id,
        mhz: ::core::ffi::c_int,
    ) -> pp_smu_status;
    pub fn pp_nv_get_maximum_sustainable_clocks(
        pp: *mut pp_smu,
        max_clocks: *mut pp_smu_nv_clock_table,
    ) -> pp_smu_status;
    pub fn pp_nv_get_uclk_dpm_states(
        pp: *mut pp_smu,
        clock_values_in_khz: *mut ::core::ffi::c_uint,
        num_states: *mut ::core::ffi::c_uint,
    ) -> pp_smu_status;
    pub fn pp_rn_get_dpm_clock_table(
        pp: *mut pp_smu,
        clock_table: *mut dpm_clocks,
    ) -> pp_smu_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
