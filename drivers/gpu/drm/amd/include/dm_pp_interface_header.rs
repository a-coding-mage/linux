/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 */

// Dependency supplied by dm_services_types.h.

pub const PP_MAX_CLOCK_LEVELS: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum amd_pp_display_config_type {
    AMD_PP_DisplayConfigType_None = 0,
    AMD_PP_DisplayConfigType_DP54,
    AMD_PP_DisplayConfigType_DP432,
    AMD_PP_DisplayConfigType_DP324,
    AMD_PP_DisplayConfigType_DP27,
    AMD_PP_DisplayConfigType_DP243,
    AMD_PP_DisplayConfigType_DP216,
    AMD_PP_DisplayConfigType_DP162,
    AMD_PP_DisplayConfigType_HDMI6G,
    AMD_PP_DisplayConfigType_HDMI297,
    AMD_PP_DisplayConfigType_HDMI162,
    AMD_PP_DisplayConfigType_LVDS,
    AMD_PP_DisplayConfigType_DVI,
    AMD_PP_DisplayConfigType_WIRELESS,
    AMD_PP_DisplayConfigType_VGA,
}

#[repr(C)]
pub struct single_display_configuration {
    pub controller_index: u32,
    pub controller_id: u32,
    pub signal_type: u32,
    pub display_state: u32,
    /* phy id for the primary internal transmitter */
    pub primary_transmitter_phyi_d: u8,
    /* bitmap with the active lanes */
    pub primary_transmitter_active_lanemap: u8,
    /* phy id for the secondary internal transmitter (for dual-link dvi) */
    pub secondary_transmitter_phy_id: u8,
    /* bitmap with the active lanes */
    pub secondary_transmitter_active_lanemap: u8,
    /* misc phy settings for SMU. */
    pub config_flags: u32,
    pub display_type: u32,
    pub view_resolution_cx: u32,
    pub view_resolution_cy: u32,
    pub displayconfigtype: amd_pp_display_config_type,
    pub vertical_refresh: u32, /* for active display */
    pub pixel_clock: u32, /* Pixel clock in KHz (for HDMI only: normalized) */
}

pub const MAX_NUM_DISPLAY: usize = 32;

#[repr(C)]
pub struct amd_pp_display_configuration {
    pub nb_pstate_switch_disable: bool, /* controls NB PState switch */
    pub cpu_cc6_disable: bool, /* controls CPU CState switch ( on or off) */
    pub cpu_pstate_disable: bool,
    pub cpu_pstate_separation_time: u32,
    pub num_display: u32, /* total number of display*/
    pub num_path_including_non_display: u32,
    pub crossfire_display_index: u32,
    pub min_mem_set_clock: u32,
    pub min_core_set_clock: u32,
    /* unit 10KHz x bit*/
    pub min_bus_bandwidth: u32,
    /* minimum required stutter sclk, in 10khz uint32_t ulMinCoreSetClk;*/
    pub min_core_set_clock_in_sr: u32,
    pub displays: [single_display_configuration; MAX_NUM_DISPLAY],
    pub vrefresh: u32, /* for active display*/
    pub min_vblank_time: u32, /* for active display*/
    pub multi_monitor_in_sync: bool,
    /* Controller Index of primary display - used in MCLK SMC switching hang
     * SW Workaround*/
    pub crtc_index: u32,
    /* htotal*1000/pixelclk - used in MCLK SMC switching hang SW Workaround*/
    pub line_time_in_us: u32,
    pub invalid_vblank_time: bool,
    pub display_clk: u32,
    /*
     * for given display configuration if multimonitormnsync == false then
     * Memory clock DPMS with this latency or below is allowed, DPMS with
     * higher latency not allowed.
     */
    pub dce_tolerable_mclk_in_active_latency: u32,
    pub min_dcef_set_clk: u32,
    pub min_dcef_deep_sleep_set_clk: u32,
}

#[repr(C)]
pub struct amd_pp_simple_clock_info {
    pub engine_max_clock: u32,
    pub memory_max_clock: u32,
}

#[repr(C)]
pub struct amd_pp_clock_info {
    pub min_engine_clock: u32,
    pub max_engine_clock: u32,
    pub min_memory_clock: u32,
    pub max_memory_clock: u32,
    pub min_bus_bandwidth: u32,
    pub max_bus_bandwidth: u32,
    pub max_engine_clock_in_sr: u32,
    pub min_engine_clock_in_sr: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum amd_pp_clock_type {
    amd_pp_disp_clock = 1,
    amd_pp_sys_clock,
    amd_pp_mem_clock,
    amd_pp_dcef_clock,
    amd_pp_soc_clock,
    amd_pp_pixel_clock,
    amd_pp_phy_clock,
    amd_pp_dcf_clock,
    amd_pp_dpp_clock,
}

pub const amd_pp_f_clock: amd_pp_clock_type = amd_pp_clock_type::amd_pp_dcef_clock;

pub const MAX_NUM_CLOCKS: usize = 16;

#[repr(C)]
pub struct amd_pp_clocks {
    pub count: u32,
    pub clock: [u32; MAX_NUM_CLOCKS],
    pub latency: [u32; MAX_NUM_CLOCKS],
}

#[repr(C)]
pub struct pp_clock_with_latency {
    pub clocks_in_khz: u32,
    pub latency_in_us: u32,
}

#[repr(C)]
pub struct pp_clock_levels_with_latency {
    pub num_levels: u32,
    pub data: [pp_clock_with_latency; PP_MAX_CLOCK_LEVELS],
}

#[repr(C)]
pub struct pp_clock_with_voltage {
    pub clocks_in_khz: u32,
    pub voltage_in_mv: u32,
}

#[repr(C)]
pub struct pp_clock_levels_with_voltage {
    pub num_levels: u32,
    pub data: [pp_clock_with_voltage; PP_MAX_CLOCK_LEVELS],
}

#[repr(C)]
pub struct pp_display_clock_request {
    pub clock_type: amd_pp_clock_type,
    pub clock_freq_in_khz: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
