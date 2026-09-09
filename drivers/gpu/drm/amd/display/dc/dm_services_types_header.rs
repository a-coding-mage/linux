/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// C dependencies: os_types.h and dc_types.h.

pub struct pp_smu_funcs;

#[repr(C)]
pub struct dm_pp_clock_range { pub min_khz: i32, pub max_khz: i32 }

#[repr(C)]
pub struct dm_pp_gpu_clock_range {
    pub sclk: dm_pp_clock_range, pub mclk: dm_pp_clock_range,
    pub eclk: dm_pp_clock_range, pub dclk: dm_pp_clock_range,
}

#[repr(i32)]
pub enum dm_pp_clock_type {
    DM_PP_CLOCK_TYPE_DISPLAY_CLK = 1,
    DM_PP_CLOCK_TYPE_ENGINE_CLK,
    DM_PP_CLOCK_TYPE_MEMORY_CLK,
    DM_PP_CLOCK_TYPE_DCFCLK,
    DM_PP_CLOCK_TYPE_DCEFCLK,
    DM_PP_CLOCK_TYPE_SOCCLK,
    DM_PP_CLOCK_TYPE_PIXELCLK,
    DM_PP_CLOCK_TYPE_DISPLAYPHYCLK,
    DM_PP_CLOCK_TYPE_DPPCLK,
    DM_PP_CLOCK_TYPE_FCLK,
}

pub fn DC_DECODE_PP_CLOCK_TYPE(clk_type: dm_pp_clock_type) -> &'static str {
    match clk_type {
        dm_pp_clock_type::DM_PP_CLOCK_TYPE_DISPLAY_CLK => "Display",
        dm_pp_clock_type::DM_PP_CLOCK_TYPE_ENGINE_CLK => "Engine",
        dm_pp_clock_type::DM_PP_CLOCK_TYPE_MEMORY_CLK => "Memory",
        dm_pp_clock_type::DM_PP_CLOCK_TYPE_DCFCLK => "DCF",
        dm_pp_clock_type::DM_PP_CLOCK_TYPE_DCEFCLK => "DCEF",
        dm_pp_clock_type::DM_PP_CLOCK_TYPE_SOCCLK => "SoC",
        dm_pp_clock_type::DM_PP_CLOCK_TYPE_PIXELCLK => "Pixel",
        dm_pp_clock_type::DM_PP_CLOCK_TYPE_DISPLAYPHYCLK => "Display PHY",
        dm_pp_clock_type::DM_PP_CLOCK_TYPE_DPPCLK => "DPP",
        dm_pp_clock_type::DM_PP_CLOCK_TYPE_FCLK => "F",
    }
}

pub const DM_PP_MAX_CLOCK_LEVELS: usize = 16;
#[repr(C)] pub struct dm_pp_clock_levels { pub num_levels: u32, pub clocks_in_khz: [u32; DM_PP_MAX_CLOCK_LEVELS] }
#[repr(C)] pub struct dm_pp_clock_with_latency { pub clocks_in_khz: u32, pub latency_in_us: u32 }
#[repr(C)] pub struct dm_pp_clock_levels_with_latency { pub num_levels: u32, pub data: [dm_pp_clock_with_latency; DM_PP_MAX_CLOCK_LEVELS] }
#[repr(C)] pub struct dm_pp_clock_with_voltage { pub clocks_in_khz: u32, pub voltage_in_mv: u32 }
#[repr(C)] pub struct dm_pp_clock_levels_with_voltage { pub num_levels: u32, pub data: [dm_pp_clock_with_voltage; DM_PP_MAX_CLOCK_LEVELS] }

#[repr(C)] pub struct dm_pp_single_disp_config {
    pub signal: signal_type, pub transmitter: u8, pub ddi_channel_mapping: u8,
    pub pipe_idx: u8, pub src_height: u32, pub src_width: u32, pub v_refresh: u32,
    pub pixel_clock: u32, pub link_settings: dc_link_settings,
}

pub const MAX_WM_SETS: usize = 4;
#[repr(i32)] pub enum dm_pp_wm_set_id { WM_SET_A = 0, WM_SET_B, WM_SET_C, WM_SET_D, WM_SET_INVALID = 0xffff }
#[repr(C)] pub struct dm_pp_clock_range_for_wm_set { pub wm_set_id: dm_pp_wm_set_id, pub wm_min_eng_clk_in_khz: u32, pub wm_max_eng_clk_in_khz: u32, pub wm_min_mem_clk_in_khz: u32, pub wm_max_mem_clk_in_khz: u32 }
#[repr(C)] pub struct dm_pp_wm_sets_with_clock_ranges { pub num_wm_sets: u32, pub wm_clk_ranges: [dm_pp_clock_range_for_wm_set; MAX_WM_SETS] }
#[repr(C)] pub struct dm_pp_clock_range_for_dmif_wm_set_soc15 { pub wm_set_id: dm_pp_wm_set_id, pub wm_min_dcfclk_clk_in_khz: u32, pub wm_max_dcfclk_clk_in_khz: u32, pub wm_min_mem_clk_in_khz: u32, pub wm_max_mem_clk_in_khz: u32 }
#[repr(C)] pub struct dm_pp_clock_range_for_mcif_wm_set_soc15 { pub wm_set_id: dm_pp_wm_set_id, pub wm_min_socclk_clk_in_khz: u32, pub wm_max_socclk_clk_in_khz: u32, pub wm_min_mem_clk_in_khz: u32, pub wm_max_mem_clk_in_khz: u32 }
#[repr(C)] pub struct dm_pp_wm_sets_with_clock_ranges_soc15 { pub num_wm_dmif_sets: u32, pub num_wm_mcif_sets: u32, pub wm_dmif_clocks_ranges: [dm_pp_clock_range_for_dmif_wm_set_soc15; MAX_WM_SETS], pub wm_mcif_clocks_ranges: [dm_pp_clock_range_for_mcif_wm_set_soc15; MAX_WM_SETS] }

pub const MAX_DISPLAY_CONFIGS: usize = 6;
#[repr(C)] pub struct dm_pp_display_configuration {
    pub nb_pstate_switch_disable: bool, pub cpu_cc6_disable: bool, pub cpu_pstate_disable: bool,
    pub cpu_pstate_separation_time: u32, pub min_memory_clock_khz: u32, pub min_engine_clock_khz: u32,
    pub min_engine_clock_deep_sleep_khz: u32, pub avail_mclk_switch_time_us: u32,
    pub avail_mclk_switch_time_in_disp_active_us: u32, pub min_dcfclock_khz: u32,
    pub min_dcfc_deep_sleep_clock_khz: u32, pub disp_clk_khz: u32, pub all_displays_in_sync: bool,
    pub display_count: u8, pub disp_configs: [dm_pp_single_disp_config; MAX_DISPLAY_CONFIGS],
    pub crtc_index: u8, pub line_time_in_us: u32,
}

#[repr(C)] pub struct dm_bl_data_point { pub luminance: u8, pub signal_level: u8 }
pub const BL_DATA_POINTS: usize = 99;
#[repr(C)] pub struct dm_acpi_atif_backlight_caps { pub size: u16, pub flags: u16, pub error_code: u8, pub ac_level_percentage: u8, pub dc_level_percentage: u8, pub min_input_signal: u8, pub max_input_signal: u8, pub num_data_points: u8, pub data_points: [dm_bl_data_point; BL_DATA_POINTS] }

#[repr(i32)] pub enum dm_acpi_display_type { AcpiDisplayType_LCD1 = 0, AcpiDisplayType_CRT1 = 1, AcpiDisplayType_DFP1 = 3, AcpiDisplayType_CRT2 = 4, AcpiDisplayType_LCD2 = 5, AcpiDisplayType_DFP2 = 7, AcpiDisplayType_DFP3 = 9, AcpiDisplayType_DFP4 = 10, AcpiDisplayType_DFP5 = 11, AcpiDisplayType_DFP6 = 12 }
#[repr(C)] pub struct dm_pp_clock_for_voltage_req { pub clk_type: dm_pp_clock_type, pub clocks_in_khz: u32 }
#[repr(C)] pub struct dm_pp_static_clock_info { pub max_sclk_khz: u32, pub max_mclk_khz: u32 }
#[repr(C)] pub struct dtn_min_clk_info { pub disp_clk_khz: u32, pub min_engine_clock_khz: u32, pub min_memory_clock_khz: u32 }

#[repr(i32)] pub enum dm_dmub_wait_type { DM_DMUB_WAIT_TYPE_NO_WAIT, DM_DMUB_WAIT_TYPE_WAIT, DM_DMUB_WAIT_TYPE_WAIT_WITH_REPLY }
#[repr(i32)] pub enum dm_acpi_transition_link_type { hdmi_tmds, hdmi_frl, dp_8b_10b, dp_128b_132b, none }
#[repr(C)] pub struct dm_process_phy_transition_init_params { pub phy_id: u32, pub action: u8, pub sym_clock_10khz: u32, pub signal: signal_type, pub display_port_lanes_count: dc_lane_count, pub display_port_link_rate: dc_link_rate, pub transition_bitmask: u32, pub hdmi_frl_num_lanes: u8, pub hdmi_frl_link_rate: hdmi_frl_link_rate }
#[repr(C)] pub struct dm_process_phy_transition_input_params { pub phy_id: u32, pub transition_id: u32, pub phy_configuration: u32, pub data_rate: u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
