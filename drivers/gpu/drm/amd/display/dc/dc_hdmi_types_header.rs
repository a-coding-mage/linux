/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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
 */

// Dependency supplied by the surrounding translation unit: os_types.h

pub const DP_ADAPTOR_TYPE2_SIZE: u32 = 0x20;
pub const DP_ADAPTOR_TYPE2_REG_ID: u32 = 0x10;
pub const DP_ADAPTOR_TYPE2_REG_MAX_TMDS_CLK: u32 = 0x1D;
pub const DP_ADAPTOR_TYPE2_ID: u32 = 0xA0;
pub const DP_ADAPTOR_TYPE2_MAX_TMDS_CLK: u32 = 600;
pub const DP_ADAPTOR_TYPE2_MIN_TMDS_CLK: u32 = 25;
pub const DP_ADAPTOR_DVI_MAX_TMDS_CLK: u32 = 165000;
pub const DP_ADAPTOR_HDMI_SAFE_MAX_TMDS_CLK: u32 = 340000;

#[repr(C)]
pub struct dp_hdmi_dongle_signature_data {
    pub id: [i8; 15],
    pub eot: u8,
}

pub const DP_HDMI_DONGLE_ADDRESS: u32 = 0x40;
pub const DP_HDMI_DONGLE_SIGNATURE_EOT: u32 = 0x04;

pub const HDMI_SCDC_WRITE_UPDATE_0_ARRAY: u32 = 3;
pub const HDMI_SCDC_ADDRESS: u32 = 0x54;
pub const HDMI_SCDC_SINK_VERSION: u32 = 0x01;
pub const HDMI_SCDC_SOURCE_VERSION: u32 = 0x02;
pub const HDMI_SCDC_UPDATE_0: u32 = 0x10;
pub const HDMI_SCDC_TMDS_CONFIG: u32 = 0x20;
pub const HDMI_SCDC_SCRAMBLER_STATUS: u32 = 0x21;
pub const HDMI_SCDC_CONFIG_0: u32 = 0x30;
pub const HDMI_SCDC_CONFIG_1: u32 = 0x31;
pub const HDMI_SCDC_SOURCE_TEST_REQ: u32 = 0x35;
pub const HDMI_SCDC_STATUS_FLAGS: u32 = 0x40;
pub const HDMI_SCDC_LTP_REQ: u32 = 0x41;
pub const HDMI_SCDC_ERR_DETECT: u32 = 0x50;
pub const HDMI_SCDC_TEST_CONFIG: u32 = 0xC0;
pub const HDMI_SCDC_MANUFACTURER_OUI: u32 = 0xD0;
pub const HDMI_SCDC_DEVICE_ID: u32 = 0xDB;

pub const HDMI_IDCC_ADDRESS: u32 = 0x50;
pub const HDMI_IDCC_MARKER0: u32 = 0xAE;
pub const HDMI_IDCC_MARKER1: u32 = 0x6E;
pub const HDMI_IDCC_MARKER2: u32 = 0x60;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hdmi_idcc_scope { HDMI_IDCC_SCOPE_WRITE = 0x00, HDMI_IDCC_SCOPE_RW_CA = 0x01, HDMI_IDCC_SCOPE_RW_SINK = 0x02 }

#[repr(C)]
pub union hdmi_idcc_source_id { pub bits: [u8; 1], pub raw: u8 }
#[repr(C)]
pub union hdmi_idcc_cable_id { pub bits: [u8; 4], pub raw: [u8; 4] }
#[repr(C)]
pub union hdmi_scdc_update_read_data { pub byte: [u8; 2], pub fields: [u8; 2] }
#[repr(C)]
pub union hdmi_scdc_status_flags_data { pub byte: u8, pub fields: u8 }
#[repr(C)]
pub union hdmi_scdc_LTP_req_data { pub byte: [u8; 2], pub fields: [u8; 2] }
#[repr(C)]
pub union hdmi_scdc_ced_data { pub byte: [u8; 11], pub fields: [u8; 11] }
#[repr(C)]
pub union hdmi_scdc_manufacturer_OUI_data { pub byte: [u8; 3], pub fields: [u8; 3] }
#[repr(C)]
pub union hdmi_scdc_device_id_data { pub byte: u8, pub fields: u8 }
#[repr(C)]
pub union hdmi_scdc_configuration { pub byte: [u8; 2], pub fields: [u8; 2] }
#[repr(C)]
pub union hdmi_scdc_source_test_req { pub byte: u8, pub fields: u8 }
#[repr(C)]
pub union hdmi_scdc_test_config_Data { pub byte: u8, pub fields: u8 }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hdmi_frl_borrow_mode { HDMI_FRL_BORROW_MODE_NONE, HDMI_FRL_BORROW_MODE_FROM_ACTIVE, HDMI_FRL_BORROW_MODE_FROM_BLANK }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum link_result { LINK_RESULT_UNKNOWN = 0, LINK_RESULT_SUCCESS, LINK_RESULT_LOWER_LINKRATE, LINK_RESULT_TIMEOUT, LINK_RESULT_FALLBACK }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum hdmi_frl_link_rate { HDMI_FRL_LINK_RATE_DISABLE = 0, HDMI_FRL_LINK_RATE_3GBPS, HDMI_FRL_LINK_RATE_6GBPS, HDMI_FRL_LINK_RATE_6GBPS_4LANE, HDMI_FRL_LINK_RATE_8GBPS, HDMI_FRL_LINK_RATE_10GBPS, HDMI_FRL_LINK_RATE_12GBPS, HDMI_FRL_LINK_RATE_16GBPS, HDMI_FRL_LINK_RATE_20GBPS, HDMI_FRL_LINK_RATE_24GBPS }

#[repr(C)]
pub struct frl_borrow_params { pub audio_packets_line: i32, pub hc_active_target: i32, pub hc_blank_target: i32, pub borrow_mode: hdmi_frl_borrow_mode }
#[repr(C)]
pub struct dc_hdmi_frl_link_settings { pub frl_link_rate: hdmi_frl_link_rate, pub frl_num_lanes: u8, pub borrow_params: frl_borrow_params, pub average_tribyte_rate: i32 }
#[repr(C)]
pub struct dc_hdmi_frl_flags { pub force_frl_rate: u32, pub ignore_ffe: bool, pub select_ffe: i32, pub limit_ffe: i32, pub force_frl_always: bool, pub force_frl_dsc: bool, pub force_frl_max: bool, pub apply_vsdb_rcc_wa: bool }
#[repr(C)]
pub struct dc_hdmi_frl_link_training_overrides { pub force_frl_always: bool, pub force_frl_max: bool, pub max_retries: u8, pub valid: bool }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
