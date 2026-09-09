/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
/*
 * link_encoder.h
 *
 *  Created on: Oct 6, 2015
 *      Author: yonsun
 */

// Dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct encoder_init_data {
    pub channel: channel_id,
    pub connector: graphics_object_id,
    pub hpd_gpio: *mut gpio,
    pub hpd_source: hpd_source_id,
    /* TODO: in DAL2, here was pointer to EventManagerInterface */
    pub encoder: graphics_object_id,
    pub analog_encoder: graphics_object_id,
    pub analog_engine: engine_id,
    pub ctx: *mut dc_context,
    pub transmitter: transmitter,
    pub hpd_active_high: bool,
}

#[repr(C)]
pub union encoder_feature_flags {
    // C bit-fields are represented by their underlying 32-bit storage.
    pub bits: u32,
    pub raw: u32,
}

#[repr(C)]
pub struct encoder_feature_support {
    pub flags: encoder_feature_flags,
    pub max_hdmi_deep_color: dc_color_depth,
    pub max_hdmi_pixel_clock: std::ffi::c_uint,
    pub hdmi_ycbcr420_supported: bool,
    pub dp_ycbcr420_supported: bool,
    pub fec_supported: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum phy_source_select { PHY_SOURCE_DIG, PHY_SOURCE_HPO }

#[repr(C)]
pub struct link_encoder {
    pub funcs: *const link_encoder_funcs,
    pub aux_channel_offset: i32,
    pub ctx: *mut dc_context,
    pub id: graphics_object_id,
    pub analog_id: graphics_object_id,
    pub connector: graphics_object_id,
    pub output_signals: u32,
    pub preferred_engine: engine_id,
    pub analog_engine: engine_id,
    pub features: encoder_feature_support,
    pub transmitter: transmitter,
    pub hpd_gpio: *mut gpio,
    pub hpd_source: hpd_source_id,
    pub usbc_combo_phy: bool,
    pub txffe_state: u8,
    pub hpd_active_high: bool,
}

#[repr(C)]
pub struct link_enc_state { pub dphy_fec_en: u32, pub dphy_fec_ready_shadow: u32, pub dphy_fec_active_status: u32, pub dp_link_training_complete: u32 }

#[repr(C)]
pub struct frl_txffe { pub amplitude: [u32; 4], pub pre_emphasis: [u32; 4], pub post_emphasis: [u32; 4] }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum encoder_type_select { ENCODER_TYPE_DIG = 0, ENCODER_TYPE_HDMI_FRL = 1, ENCODER_TYPE_DP_128B132B = 2 }

pub type LinkEncoderReadState = unsafe extern "C" fn(*mut link_encoder, *mut link_enc_state);
pub type LinkEncoderValidate = unsafe extern "C" fn(*mut link_encoder, *const dc_stream_state) -> bool;

#[repr(C)]
pub struct link_encoder_funcs {
    pub read_state: Option<LinkEncoderReadState>,
    pub validate_output_with_stream: Option<LinkEncoderValidate>,
    pub hw_init: Option<unsafe extern "C" fn(*mut link_encoder)>,
    pub setup: Option<unsafe extern "C" fn(*mut link_encoder, signal_type)>,
    pub enable_tmds_output: Option<unsafe extern "C" fn(*mut link_encoder, clock_source_id, dc_color_depth, signal_type, u32)>,
    pub enable_dp_output: Option<unsafe extern "C" fn(*mut link_encoder, *const dc_link_settings, clock_source_id)>,
    pub enable_dp_mst_output: Option<unsafe extern "C" fn(*mut link_encoder, *const dc_link_settings, clock_source_id)>,
    pub enable_lvds_output: Option<unsafe extern "C" fn(*mut link_encoder, clock_source_id, u32)>,
    pub enable_analog_output: Option<unsafe extern "C" fn(*mut link_encoder, u32)>,
    pub disable_output: Option<unsafe extern "C" fn(*mut link_encoder, signal_type)>,
    pub dp_set_lane_settings: Option<unsafe extern "C" fn(*mut link_encoder, *const dc_link_settings, *const dc_lane_settings)>,
    pub dp_set_phy_pattern: Option<unsafe extern "C" fn(*mut link_encoder, *const encoder_set_dp_phy_pattern_param)>,
    pub update_mst_stream_allocation_table: Option<unsafe extern "C" fn(*mut link_encoder, *const link_mst_stream_allocation_table)>,
    pub psr_program_dp_dphy_fast_training: Option<unsafe extern "C" fn(*mut link_encoder, bool)>,
    pub psr_program_secondary_packet: Option<unsafe extern "C" fn(*mut link_encoder, std::ffi::c_uint)>,
    pub connect_dig_be_to_fe: Option<unsafe extern "C" fn(*mut link_encoder, engine_id, bool)>,
    pub enable_hpd: Option<unsafe extern "C" fn(*mut link_encoder)>,
    pub disable_hpd: Option<unsafe extern "C" fn(*mut link_encoder)>,
    pub is_dig_enabled: Option<unsafe extern "C" fn(*mut link_encoder) -> bool>,
    pub get_dig_frontend: Option<unsafe extern "C" fn(*mut link_encoder) -> std::ffi::c_uint>,
    pub destroy: Option<unsafe extern "C" fn(*mut *mut link_encoder)>,
    pub fec_set_enable: Option<unsafe extern "C" fn(*mut link_encoder, bool)>,
    pub fec_set_ready: Option<unsafe extern "C" fn(*mut link_encoder, bool)>,
    pub fec_is_active: Option<unsafe extern "C" fn(*mut link_encoder) -> bool>,
    pub is_in_alt_mode: Option<unsafe extern "C" fn(*mut link_encoder) -> bool>,
    pub get_max_link_cap: Option<unsafe extern "C" fn(*mut link_encoder, *mut dc_link_settings)>,
    pub get_dig_mode: Option<unsafe extern "C" fn(*mut link_encoder) -> signal_type>,
    pub dpcstx_set_order_invert_18_bit: Option<unsafe extern "C" fn(*mut link_encoder, bool)>,
    pub set_phy_source: Option<unsafe extern "C" fn(*mut link_encoder, phy_source_select, u32)>,
    pub dpcs_initialize_phy: Option<unsafe extern "C" fn(*mut link_encoder, u32, hdmi_frl_link_rate)>,
    pub dpcs_configure_phypll: Option<unsafe extern "C" fn(*mut link_encoder, u32, hdmi_frl_link_rate)>,
    pub dpcs_configure_dpcs: Option<unsafe extern "C" fn(*mut link_encoder)>,
    pub dpcs_enable_dpcs: Option<unsafe extern "C" fn(*mut link_encoder)>,
    pub prog_eq_setting: Option<unsafe extern "C" fn(*mut link_encoder, u8, bool, bool, bool, *const dc_hdmi_frl_link_settings)>,
    pub get_txffe: Option<unsafe extern "C" fn(*mut link_encoder, *mut frl_txffe)>,
    pub set_txffe: Option<unsafe extern "C" fn(*mut link_encoder, *mut frl_txffe)>,
    pub set_dio_phy_mux: Option<unsafe extern "C" fn(*mut link_encoder, encoder_type_select, u32)>,
    pub enable_dpia_output: Option<unsafe extern "C" fn(*mut link_encoder, *const dc_link_settings, u8, u8, u8)>,
    pub disable_dpia_output: Option<unsafe extern "C" fn(*mut link_encoder, u8, u8)>,
    pub get_hpd_state: Option<unsafe extern "C" fn(*mut link_encoder) -> bool>,
    pub program_hpd_filter: Option<unsafe extern "C" fn(*mut link_encoder, i32, i32) -> bool>,
    pub setup_ri_pj_check_in_sw_or_hw_mode: Option<unsafe extern "C" fn(*mut link_encoder, u8, bool)>,
}

/* Used to track assignments of links (display endpoints) to link encoders. */
#[repr(C)]
pub struct link_enc_assignment { pub valid: bool, pub ep_id: display_endpoint_id, pub eng_id: engine_id, pub stream: *mut dc_stream_state }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum link_enc_cfg_mode { LINK_ENC_CFG_STEADY, LINK_ENC_CFG_TRANSIENT }

#[repr(C)]
pub struct hpo_frl_link_encoder { pub funcs: *const hpo_frl_link_encoder_funcs, pub ctx: *mut dc_context, pub inst: i32 }

#[repr(C)]
pub struct hpo_frl_link_enc_state { pub link_enc_enabled: u32, pub link_active: u32, pub lane_count: u32 }

#[repr(C)]
pub struct hpo_frl_link_encoder_funcs {
    pub setup_link_encoder: Option<unsafe extern "C" fn(*mut hpo_frl_link_encoder, i32)>,
    pub disable_link_encoder: Option<unsafe extern "C" fn(*mut hpo_frl_link_encoder)>,
    pub set_hdmi_training_pattern: Option<unsafe extern "C" fn(*mut hpo_frl_link_encoder, u32, u32, u32, u32)>,
    pub get_hdmi_training_pattern: Option<unsafe extern "C" fn(*mut hpo_frl_link_encoder, *mut u32, *mut u32, *mut u32, *mut u32)>,
    pub enable_frl_phy_output: Option<unsafe extern "C" fn(*mut hpo_frl_link_encoder, *mut link_encoder, clock_source_id, hdmi_frl_link_rate)>,
    pub enable_output: Option<unsafe extern "C" fn(*mut hpo_frl_link_encoder)>,
    pub read_state: Option<unsafe extern "C" fn(*mut hpo_frl_link_encoder, *mut hpo_frl_link_enc_state)>,
    pub destroy: Option<unsafe extern "C" fn(*mut *mut hpo_frl_link_encoder)>,
    pub apply_vsdb_rcc_wa: Option<unsafe extern "C" fn(*mut hpo_frl_link_encoder)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dp2_link_mode { DP2_LINK_TRAINING_TPS1, DP2_LINK_TRAINING_TPS2, DP2_LINK_ACTIVE, DP2_TEST_PATTERN }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum dp2_phy_tp_select { DP_DPHY_TP_SELECT_TPS1, DP_DPHY_TP_SELECT_TPS2, DP_DPHY_TP_SELECT_PRBS, DP_DPHY_TP_SELECT_CUSTOM, DP_DPHY_TP_SELECT_SQUARE }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum dp2_phy_tp_prbs { DP_DPHY_TP_PRBS7, DP_DPHY_TP_PRBS9, DP_DPHY_TP_PRBS11, DP_DPHY_TP_PRBS15, DP_DPHY_TP_PRBS23, DP_DPHY_TP_PRBS31 }

#[repr(C)]
pub struct hpo_dp_link_enc_state { pub link_enc_enabled: u32, pub link_mode: u32, pub lane_count: u32, pub slot_count: [u32; 4], pub stream_src: [u32; 4], pub vc_rate_x: [u32; 4], pub vc_rate_y: [u32; 4] }

#[repr(C)]
pub struct hpo_dp_link_encoder { pub funcs: *const hpo_dp_link_encoder_funcs, pub ctx: *mut dc_context, pub inst: i32, pub preferred_engine: engine_id, pub transmitter: transmitter, pub hpd_source: hpd_source_id }

#[repr(C)]
pub struct hpo_dp_link_encoder_funcs {
    pub enable_link_phy: Option<unsafe extern "C" fn(*mut hpo_dp_link_encoder, *const dc_link_settings, transmitter, hpd_source_id)>,
    pub disable_link_phy: Option<unsafe extern "C" fn(*mut hpo_dp_link_encoder, signal_type)>,
    pub link_enable: Option<unsafe extern "C" fn(*mut hpo_dp_link_encoder, dc_lane_count)>,
    pub link_disable: Option<unsafe extern "C" fn(*mut hpo_dp_link_encoder)>,
    pub set_link_test_pattern: Option<unsafe extern "C" fn(*mut hpo_dp_link_encoder, *mut encoder_set_dp_phy_pattern_param)>,
    pub update_stream_allocation_table: Option<unsafe extern "C" fn(*mut hpo_dp_link_encoder, *const link_mst_stream_allocation_table)>,
    pub set_throttled_vcp_size: Option<unsafe extern "C" fn(*mut hpo_dp_link_encoder, u32, fixed31_32)>,
    pub is_in_alt_mode: Option<unsafe extern "C" fn(*mut hpo_dp_link_encoder) -> bool>,
    pub read_state: Option<unsafe extern "C" fn(*mut hpo_dp_link_encoder, *mut hpo_dp_link_enc_state)>,
    pub set_ffe: Option<unsafe extern "C" fn(*mut hpo_dp_link_encoder, *const dc_link_settings, u8)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
