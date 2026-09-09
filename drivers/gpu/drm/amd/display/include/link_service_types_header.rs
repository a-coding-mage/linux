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

// Dependency declarations supplied by the surrounding translation.
pub struct ddc;
pub struct irq_manager;
pub struct drm_dp_mst_port;

#[repr(i32)]
pub enum dp_power_state {
    DP_POWER_STATE_D0 = 1,
    DP_POWER_STATE_D3,
}

#[repr(i32)]
pub enum edp_revision {
    EDP_REVISION_11 = 0x00,
    EDP_REVISION_12 = 0x01,
    EDP_REVISION_13 = 0x02,
}

pub const LINK_RATE_REF_FREQ_IN_KHZ: i32 = 27000;
pub const BITS_PER_DP_BYTE: i32 = 10;
pub const DATA_EFFICIENCY_8b_10b_x10000: i32 = 8000;
pub const DATA_EFFICIENCY_8b_10b_FEC_EFFICIENCY_x100: i32 = 97;
pub const DATA_EFFICIENCY_128b_132b_x10000: i32 = 9641;

#[repr(i32)]
pub enum lttpr_mode {
    LTTPR_MODE_UNKNOWN,
    LTTPR_MODE_NON_LTTPR,
    LTTPR_MODE_TRANSPARENT,
    LTTPR_MODE_NON_TRANSPARENT,
}

#[repr(C)]
pub struct link_training_settings {
    pub link_settings: dc_link_settings,
    pub voltage_swing: *mut dc_voltage_swing,
    pub pre_emphasis: *mut dc_pre_emphasis,
    pub post_cursor2: *mut dc_post_cursor2,
    pub should_set_fec_ready: bool,
    pub ffe_preset: *mut dc_dp_ffe_preset,
    pub cr_pattern_time: u16,
    pub eq_pattern_time: u16,
    pub cds_pattern_time: u16,
    pub pattern_for_cr: dc_dp_training_pattern,
    pub pattern_for_eq: dc_dp_training_pattern,
    pub pattern_for_cds: dc_dp_training_pattern,
    pub eq_wait_time_limit: u32,
    pub eq_loop_count_limit: u8,
    pub cds_wait_time_limit: u32,
    pub enhanced_framing: bool,
    pub lttpr_mode: lttpr_mode,
    pub lttpr_early_tps2: bool,
    pub disallow_per_lane_settings: bool,
    pub always_match_dpcd_with_hw_lane_settings: bool,
    pub hw_lane_settings: [dc_lane_settings; LANE_COUNT_DP_MAX],
    pub dpcd_lane_settings: [dpcd_training_lane; LANE_COUNT_DP_MAX],
}

#[repr(i32)]
pub enum dp_test_pattern {
    DP_TEST_PATTERN_VIDEO_MODE = 0,
    DP_TEST_PATTERN_PHY_PATTERN_BEGIN,
    DP_TEST_PATTERN_D102 = DP_TEST_PATTERN_PHY_PATTERN_BEGIN,
    DP_TEST_PATTERN_SYMBOL_ERROR,
    DP_TEST_PATTERN_PRBS7,
    DP_TEST_PATTERN_80BIT_CUSTOM,
    DP_TEST_PATTERN_CP2520_1,
    DP_TEST_PATTERN_CP2520_2,
    DP_TEST_PATTERN_HBR2_COMPLIANCE_EYE = DP_TEST_PATTERN_CP2520_2,
    DP_TEST_PATTERN_CP2520_3,
    DP_TEST_PATTERN_128b_132b_TPS1,
    DP_TEST_PATTERN_128b_132b_TPS2,
    DP_TEST_PATTERN_PRBS9,
    DP_TEST_PATTERN_PRBS11,
    DP_TEST_PATTERN_PRBS15,
    DP_TEST_PATTERN_PRBS23,
    DP_TEST_PATTERN_PRBS31,
    DP_TEST_PATTERN_264BIT_CUSTOM,
    DP_TEST_PATTERN_SQUARE_BEGIN,
    DP_TEST_PATTERN_SQUARE = DP_TEST_PATTERN_SQUARE_BEGIN,
    DP_TEST_PATTERN_SQUARE_PRESHOOT_DISABLED,
    DP_TEST_PATTERN_SQUARE_DEEMPHASIS_DISABLED,
    DP_TEST_PATTERN_SQUARE_PRESHOOT_DEEMPHASIS_DISABLED,
    DP_TEST_PATTERN_SQUARE_END = DP_TEST_PATTERN_SQUARE_PRESHOOT_DEEMPHASIS_DISABLED,
    DP_TEST_PATTERN_TRAINING_PATTERN1,
    DP_TEST_PATTERN_TRAINING_PATTERN2,
    DP_TEST_PATTERN_TRAINING_PATTERN3,
    DP_TEST_PATTERN_TRAINING_PATTERN4,
    DP_TEST_PATTERN_128b_132b_TPS1_TRAINING_MODE,
    DP_TEST_PATTERN_128b_132b_TPS2_TRAINING_MODE,
    DP_TEST_PATTERN_PHY_PATTERN_END = DP_TEST_PATTERN_128b_132b_TPS2_TRAINING_MODE,
    DP_TEST_PATTERN_COLOR_SQUARES,
    DP_TEST_PATTERN_COLOR_SQUARES_CEA,
    DP_TEST_PATTERN_VERTICAL_BARS,
    DP_TEST_PATTERN_HORIZONTAL_BARS,
    DP_TEST_PATTERN_COLOR_RAMP,
    DP_TEST_PATTERN_AUDIO_OPERATOR_DEFINED,
    DP_TEST_PATTERN_AUDIO_SAWTOOTH,
    DP_TEST_PATTERN_UNSUPPORTED,
}

#[inline]
pub fn IS_DP_PHY_SQUARE_PATTERN(test_pattern: dp_test_pattern) -> bool {
    (dp_test_pattern::DP_TEST_PATTERN_SQUARE_BEGIN as i32 <= test_pattern as i32)
        && (test_pattern as i32 <= dp_test_pattern::DP_TEST_PATTERN_SQUARE_END as i32)
}

#[inline]
pub fn IS_DP_PHY_PATTERN(test_pattern: dp_test_pattern) -> bool {
    ((dp_test_pattern::DP_TEST_PATTERN_PHY_PATTERN_BEGIN as i32 <= test_pattern as i32)
        && (test_pattern as i32 <= dp_test_pattern::DP_TEST_PATTERN_PHY_PATTERN_END as i32))
        || test_pattern as i32 == dp_test_pattern::DP_TEST_PATTERN_VIDEO_MODE as i32
}

#[repr(i32)]
pub enum dp_test_pattern_color_space {
    DP_TEST_PATTERN_COLOR_SPACE_RGB,
    DP_TEST_PATTERN_COLOR_SPACE_YCBCR601,
    DP_TEST_PATTERN_COLOR_SPACE_YCBCR709,
    DP_TEST_PATTERN_COLOR_SPACE_UNDEFINED,
}

#[repr(i32)]
pub enum dp_panel_mode {
    DP_PANEL_MODE_DEFAULT,
    DP_PANEL_MODE_EDP,
    DP_PANEL_MODE_SPECIAL,
}

#[repr(i32)]
pub enum dpcd_source_sequence {
    DPCD_SOURCE_SEQ_AFTER_CONNECT_DIG_FE_OTG = 1,
    DPCD_SOURCE_SEQ_AFTER_DP_STREAM_ATTR,
    DPCD_SOURCE_SEQ_AFTER_UPDATE_INFO_FRAME,
    DPCD_SOURCE_SEQ_AFTER_CONNECT_DIG_FE_BE,
    DPCD_SOURCE_SEQ_AFTER_ENABLE_LINK_PHY,
    DPCD_SOURCE_SEQ_AFTER_SET_SOURCE_PATTERN,
    DPCD_SOURCE_SEQ_AFTER_ENABLE_AUDIO_STREAM,
    DPCD_SOURCE_SEQ_AFTER_ENABLE_DP_VID_STREAM,
    DPCD_SOURCE_SEQ_AFTER_DISABLE_DP_VID_STREAM,
    DPCD_SOURCE_SEQ_AFTER_FIFO_STEER_RESET,
    DPCD_SOURCE_SEQ_AFTER_DISABLE_AUDIO_STREAM,
    DPCD_SOURCE_SEQ_AFTER_DISABLE_LINK_PHY,
    DPCD_SOURCE_SEQ_AFTER_DISCONNECT_DIG_FE_BE,
}

#[repr(C)]
pub struct dpcd_training_lane_set_bits {
    pub raw: u8,
}

#[repr(C)]
pub union dpcd_training_lane_set {
    pub bits: dpcd_training_lane_set_bits,
    pub raw: u8,
}

#[repr(C)]
pub struct dc_dp_mst_stream_allocation {
    pub vcp_id: u8,
    pub slot_count: u8,
}

#[repr(C)]
pub struct dc_dp_mst_stream_allocation_table {
    pub stream_count: i32,
    pub stream_allocations: [dc_dp_mst_stream_allocation; MAX_CONTROLLER_NUM],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
