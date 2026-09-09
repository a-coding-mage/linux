// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit:
// os_types.h, core_types.h, and dalsmc.h

#[repr(C)]
pub struct clk_mgr_internal {
    _private: [u8; 0],
}

extern "C" {
    pub fn dcn60_smu_set_hard_min_by_freq(
        clk_mgr: *mut clk_mgr_internal,
        clk: u32,
        freq_mhz: u16,
    ) -> u32;

    pub fn dcn60_smu_set_stutter_efficiency(
        clk_mgr: *mut clk_mgr_internal,
        base_efficiency: u8,
        low_power_efficiency: u8,
    );

    pub fn dcn60_smu_set_min_deep_sleep_dcfclk(
        clk_mgr: *mut clk_mgr_internal,
        freq_mhz: u32,
    );

    pub fn dcn60_smu_set_pme_workaround(clk_mgr: *mut clk_mgr_internal);

    pub fn dcn60_smu_indicate_pstate_status(
        clk_mgr: *mut clk_mgr_internal,
        allow_fclk: bool,
        allow_uclk: bool,
        wait_resp: bool,
        drr_enable: bool,
        alt_ch_enable: bool,
    );

    pub fn dcn60_smu_update_utm_qos_request(
        clk_mgr: *mut clk_mgr_internal,
        latency_sop_index: u32,
        nominal_bandwidth_KBps: u32,
        urgent_bandwidth_KBps: u32,
        lsdma_bandwidth_KBps: u32,
    ) -> bool;

    pub fn dcn60_smu_set_soc_utm_table(
        clk_mgr: *mut clk_mgr_internal,
        dram_addr: i64,
    ) -> bool;

    pub fn dcn60_smu_get_dal_init_table(
        clk_mgr: *mut clk_mgr_internal,
        init_table: *mut *const DalInitTable_t,
    ) -> bool;

    pub fn dcn60_smu_get_msg_header_version(
        clk_mgr: *mut clk_mgr_internal,
        version: *mut u32,
    ) -> bool;

    pub fn dcn60_smu_set_display_idle_optimization(
        clk_mgr: *mut clk_mgr_internal,
        is_idle: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
