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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// C header dependencies are supplied by other translated units.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum as_signal_type { AS_SIGNAL_TYPE_NONE = 0, AS_SIGNAL_TYPE_DVI, AS_SIGNAL_TYPE_HDMI, AS_SIGNAL_TYPE_LVDS, AS_SIGNAL_TYPE_DISPLAY_PORT, AS_SIGNAL_TYPE_GPU_PLL, AS_SIGNAL_TYPE_XGMI, AS_SIGNAL_TYPE_UNKNOWN }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bp_result { BP_RESULT_OK = 0, BP_RESULT_BADINPUT, BP_RESULT_BADBIOSTABLE, BP_RESULT_UNSUPPORTED, BP_RESULT_NORECORD, BP_RESULT_FAILURE }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bp_encoder_control_action { ENCODER_CONTROL_DISABLE = 0, ENCODER_CONTROL_ENABLE, ENCODER_CONTROL_SETUP, ENCODER_CONTROL_INIT }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bp_transmitter_control_action { TRANSMITTER_CONTROL_DISABLE = 0, TRANSMITTER_CONTROL_ENABLE, TRANSMITTER_CONTROL_BACKLIGHT_OFF, TRANSMITTER_CONTROL_BACKLIGHT_ON, TRANSMITTER_CONTROL_BACKLIGHT_BRIGHTNESS, TRANSMITTER_CONTROL_LCD_SETF_TEST_START, TRANSMITTER_CONTROL_LCD_SELF_TEST_STOP, TRANSMITTER_CONTROL_INIT, TRANSMITTER_CONTROL_DEACTIVATE, TRANSMITTER_CONTROL_ACTIAVATE, TRANSMITTER_CONTROL_SETUP, TRANSMITTER_CONTROL_SET_VOLTAGE_AND_PREEMPASIS, TRANSMITTER_CONTROL_POWER_ON, TRANSMITTER_CONTROL_POWER_OFF }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bp_external_encoder_control_action { EXTERNAL_ENCODER_CONTROL_DISABLE = 0, EXTERNAL_ENCODER_CONTROL_ENABLE = 1, EXTERNAL_ENCODER_CONTROL_INIT = 0x7, EXTERNAL_ENCODER_CONTROL_SETUP = 0xf, EXTERNAL_ENCODER_CONTROL_UNBLANK = 0x10, EXTERNAL_ENCODER_CONTROL_BLANK = 0x11, EXTERNAL_ENCODER_CONTROL_DAC_LOAD_DETECT = 0x12, EXTERNAL_ENCODER_CONTROL_DDC_SETUP = 0x14 }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bp_pipe_control_action { ASIC_PIPE_DISABLE = 0, ASIC_PIPE_ENABLE, ASIC_PIPE_INIT }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bp_lvtma_control_action { LVTMA_CONTROL_LCD_BLOFF = 2, LVTMA_CONTROL_LCD_BLON = 3, LVTMA_CONTROL_POWER_ON = 12, LVTMA_CONTROL_POWER_OFF = 13 }

#[repr(C)] pub struct bp_encoder_control { pub action: bp_encoder_control_action, pub engine_id: engine_id, pub transmitter: transmitter, pub signal: signal_type, pub lanes_number: dc_lane_count, pub color_depth: dc_color_depth, pub enable_dp_audio: bool, pub pixel_clock: u32 }
#[repr(C)] pub struct bp_external_encoder_control { pub action: bp_external_encoder_control_action, pub engine_id: engine_id, pub link_rate: dc_link_rate, pub lanes_number: dc_lane_count, pub signal: signal_type, pub color_depth: dc_color_depth, pub coherent: bool, pub encoder_id: graphics_object_id, pub connector_obj_id: graphics_object_id, pub pixel_clock: u32 }
#[repr(C)] pub struct bp_crtc_source_select { pub engine_id: engine_id, pub controller_id: controller_id, pub sink_signal: signal_type, pub color_depth: dc_color_depth }
#[repr(C)] pub struct bp_transmitter_control { pub action: bp_transmitter_control_action, pub engine_id: engine_id, pub transmitter: transmitter, pub lanes_number: dc_lane_count, pub pll_id: clock_source_id, pub signal: signal_type, pub color_depth: dc_color_depth, pub hpd_sel: hpd_source_id, pub txffe_sel: tx_ffe_id, pub hpo_engine_id: engine_id, pub connector_obj_id: graphics_object_id, pub pixel_clock: u32, pub lane_select: u32, pub lane_settings: u32, pub coherent: bool, pub multi_path: bool, pub single_pll_mode: bool }
#[repr(C)] pub struct bp_load_detection_parameters { pub engine_id: engine_id, pub device_id: u16 }

#[repr(C)] pub struct timing_flags { pub INTERLACE: u32, pub PIXEL_REPETITION: u32, pub HSYNC_POSITIVE_POLARITY: u32, pub VSYNC_POSITIVE_POLARITY: u32, pub HORZ_COUNT_BY_TWO: u32 }
#[repr(C)] pub struct bp_hw_crtc_timing_parameters { pub controller_id: controller_id, pub h_total: u32, pub h_addressable: u32, pub h_overscan_left: u32, pub h_overscan_right: u32, pub h_sync_start: u32, pub h_sync_width: u32, pub v_total: u32, pub v_addressable: u32, pub v_overscan_top: u32, pub v_overscan_bottom: u32, pub v_sync_start: u32, pub v_sync_width: u32, pub flags: timing_flags }
#[repr(C)] pub struct bp_adjust_pixel_clock_parameters { pub signal_type: signal_type, pub encoder_object_id: graphics_object_id, pub pixel_clock: u32, pub adjusted_pixel_clock: u32, pub reference_divider: u32, pub pixel_clock_post_divider: u32, pub ss_enable: bool }
#[repr(C)] pub struct program_pixel_clock_flags { pub FORCE_PROGRAMMING_OF_PLL: u32, pub USE_E_CLOCK_AS_SOURCE_FOR_D_CLOCK: u32, pub SET_EXTERNAL_REF_DIV_SRC: u32, pub SET_DISPCLK_DFS_BYPASS: u32, pub PROGRAM_PHY_PLL_ONLY: u32, pub SUPPORT_YUV_420: u32, pub SET_XTALIN_REF_SRC: u32, pub SET_GENLOCK_REF_DIV_SRC: u32 }
#[repr(C)] pub struct bp_pixel_clock_parameters { pub controller_id: controller_id, pub pll_id: clock_source_id, pub signal_type: signal_type, pub target_pixel_clock_100hz: u32, pub reference_divider: u32, pub feedback_divider: u32, pub fractional_feedback_divider: u32, pub pixel_clock_post_divider: u32, pub encoder_object_id: graphics_object_id, pub dfs_bypass_display_clock: u32, pub color_depth: transmitter_color_depth, pub flags: program_pixel_clock_flags }

#[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum bp_dce_clock_type { DCECLOCK_TYPE_DISPLAY_CLOCK = 0, DCECLOCK_TYPE_DPREFCLK = 1 }
#[repr(C)] pub struct set_dce_clock_flags { pub USE_GENERICA_AS_SOURCE_FOR_DPREFCLK: u32, pub USE_XTALIN_AS_SOURCE_FOR_DPREFCLK: u32, pub USE_PCIE_AS_SOURCE_FOR_DPREFCLK: u32, pub USE_GENLOCK_AS_SOURCE_FOR_DPREFCLK: u32 }
#[repr(C)] pub struct bp_set_dce_clock_parameters { pub pll_id: clock_source_id, pub target_clock_frequency: u32, pub clock_type: bp_dce_clock_type, pub flags: set_dce_clock_flags }
#[repr(C)] pub struct spread_spectrum_flags { pub CENTER_SPREAD: u32, pub EXTERNAL_SS: u32, pub DS_TYPE: u32 }
#[repr(C)] pub struct bp_spread_spectrum_ver1 { pub step: u32, pub delay: u32, pub range: u32 }
#[repr(C)] pub struct bp_spread_spectrum_ds { pub feedback_amount: u32, pub nfrac_amount: u32, pub ds_frac_size: u32 }
#[repr(C)] pub union bp_spread_spectrum_union { pub ver1: bp_spread_spectrum_ver1, pub ds: bp_spread_spectrum_ds }
#[repr(C)] pub struct bp_spread_spectrum_parameters { pub pll_id: clock_source_id, pub percentage: u32, pub ds_frac_amount: u32, pub params: bp_spread_spectrum_union, pub flags: spread_spectrum_flags }
#[repr(C)] pub struct bp_disp_connector_caps_info { pub INTERNAL_DISPLAY: u32, pub INTERNAL_DISPLAY_BL: u32, pub NO_DDC_PIN: u32 }
#[repr(C)] pub struct bp_encoder_cap_info { pub DP_HBR2_CAP: u32, pub DP_HBR2_EN: u32, pub DP_HBR3_EN: u32, pub HDMI_6GB_EN: u32, pub IS_DP2_CAPABLE: u32, pub DP_UHBR10_EN: u32, pub DP_UHBR13_5_EN: u32, pub DP_UHBR20_EN: u32, pub DP_IS_USB_C: u32, pub IS_HDMI_FRL_CAPABLE: u32, pub FRL_8G_EN: u32, pub FRL_10G_EN: u32, pub FRL_12G_EN: u32, pub RESERVED: u32 }
#[repr(C)] pub struct bp_soc_bb_info { pub dram_clock_change_latency_100ns: u32, pub dram_sr_exit_latency_100ns: u32, pub dram_sr_enter_exit_latency_100ns: u32 }
#[repr(C)] pub struct bp_connector_speed_cap_info { pub DP_HBR2_EN: u32, pub DP_HBR3_EN: u32, pub HDMI_6GB_EN: u32, pub DP_UHBR10_EN: u32, pub DP_UHBR13_5_EN: u32, pub DP_UHBR20_EN: u32, pub DP_IS_USB_C: u32, pub FRL_8G_EN: u32, pub FRL_10G_EN: u32, pub FRL_12G_EN: u32, pub FRL_16G_EN: u32, pub FRL_20G_EN: u32, pub FRL_24G_EN: u32, pub RESERVED: u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
