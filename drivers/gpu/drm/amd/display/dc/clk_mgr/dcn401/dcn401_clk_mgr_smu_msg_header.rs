// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translation of the C header __DCN401_CLK_MGR_SMU_MSG_H_.
// Dependencies supplied by the surrounding translation unit:
//   os_types.h, core_types.h

use core::ffi::c_int;

#[repr(C)]
pub struct clk_mgr_internal {
    _private: [u8; 0],
}

extern "C" {
    pub fn dcn401_smu_get_smu_version(
        clk_mgr: *mut clk_mgr_internal,
        version: *mut c_int,
    ) -> bool;
    pub fn dcn401_smu_check_driver_if_version(clk_mgr: *mut clk_mgr_internal) -> bool;
    pub fn dcn401_smu_check_msg_header_version(clk_mgr: *mut clk_mgr_internal) -> bool;
    pub fn dcn401_smu_send_fclk_pstate_message(
        clk_mgr: *mut clk_mgr_internal,
        support: bool,
    );
    pub fn dcn401_smu_send_uclk_pstate_message(
        clk_mgr: *mut clk_mgr_internal,
        support: bool,
    );
    pub fn dcn401_smu_send_cab_for_uclk_message(
        clk_mgr: *mut clk_mgr_internal,
        num_ways: u32,
    );
    pub fn dcn401_smu_set_dram_addr_high(clk_mgr: *mut clk_mgr_internal, addr_high: u32);
    pub fn dcn401_smu_set_dram_addr_low(clk_mgr: *mut clk_mgr_internal, addr_low: u32);
    pub fn dcn401_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn401_smu_set_pme_workaround(clk_mgr: *mut clk_mgr_internal);
    pub fn dcn401_smu_set_hard_min_by_freq(
        clk_mgr: *mut clk_mgr_internal,
        clk: u32,
        freq_mhz: u16,
    ) -> u32;
    pub fn dcn401_smu_wait_for_dmub_ack_mclk(
        clk_mgr: *mut clk_mgr_internal,
        enable: bool,
    );
    pub fn dcn401_smu_indicate_drr_status(
        clk_mgr: *mut clk_mgr_internal,
        mod_drr_for_pstate: bool,
    );
    pub fn dcn401_smu_set_idle_uclk_fclk_hardmin(
        clk_mgr: *mut clk_mgr_internal,
        uclk_freq_mhz: u16,
        fclk_freq_mhz: u16,
    ) -> bool;
    pub fn dcn401_smu_set_active_uclk_fclk_hardmin(
        clk_mgr: *mut clk_mgr_internal,
        uclk_freq_mhz: u16,
        fclk_freq_mhz: u16,
    ) -> bool;
    pub fn dcn401_smu_set_subvp_uclk_fclk_hardmin(
        clk_mgr: *mut clk_mgr_internal,
        uclk_freq_mhz: u16,
        fclk_freq_mhz: u16,
    ) -> bool;
    pub fn dcn401_smu_set_min_deep_sleep_dcef_clk(
        clk_mgr: *mut clk_mgr_internal,
        freq_mhz: u32,
    );
    pub fn dcn401_smu_set_num_of_displays(
        clk_mgr: *mut clk_mgr_internal,
        num_displays: u32,
    );
    pub fn dcn401_smu_get_num_of_umc_channels(clk_mgr: *mut clk_mgr_internal) -> u32;
    pub fn dcn401_smu_get_dc_mode_max_dpm_freq(
        clk_mgr: *mut clk_mgr_internal,
        clk: u32,
    ) -> u32;
    pub fn dcn401_smu_get_dpm_freq_by_index(
        clk_mgr: *mut clk_mgr_internal,
        clk: u32,
        dpm_level: u8,
    ) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
