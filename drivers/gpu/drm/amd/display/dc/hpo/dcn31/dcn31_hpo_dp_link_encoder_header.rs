/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency declarations supplied by the surrounding translation unit:
// link_encoder.h

#[repr(C)]
pub struct dcn31_hpo_dp_link_encoder_registers {
    pub DP_LINK_ENC_CLOCK_CONTROL: u32,
    pub DP_DPHY_SYM32_CONTROL: u32,
    pub DP_DPHY_SYM32_STATUS: u32,
    pub DP_DPHY_SYM32_TP_CONFIG: u32,
    pub DP_DPHY_SYM32_TP_PRBS_SEED0: u32,
    pub DP_DPHY_SYM32_TP_PRBS_SEED1: u32,
    pub DP_DPHY_SYM32_TP_PRBS_SEED2: u32,
    pub DP_DPHY_SYM32_TP_PRBS_SEED3: u32,
    pub DP_DPHY_SYM32_TP_SQ_PULSE: u32,
    pub DP_DPHY_SYM32_TP_CUSTOM0: u32,
    pub DP_DPHY_SYM32_TP_CUSTOM1: u32,
    pub DP_DPHY_SYM32_TP_CUSTOM2: u32,
    pub DP_DPHY_SYM32_TP_CUSTOM3: u32,
    pub DP_DPHY_SYM32_TP_CUSTOM4: u32,
    pub DP_DPHY_SYM32_TP_CUSTOM5: u32,
    pub DP_DPHY_SYM32_TP_CUSTOM6: u32,
    pub DP_DPHY_SYM32_TP_CUSTOM7: u32,
    pub DP_DPHY_SYM32_TP_CUSTOM8: u32,
    pub DP_DPHY_SYM32_TP_CUSTOM9: u32,
    pub DP_DPHY_SYM32_TP_CUSTOM10: u32,
    pub DP_DPHY_SYM32_SAT_VC0: u32,
    pub DP_DPHY_SYM32_SAT_VC1: u32,
    pub DP_DPHY_SYM32_SAT_VC2: u32,
    pub DP_DPHY_SYM32_SAT_VC3: u32,
    pub DP_DPHY_SYM32_VC_RATE_CNTL0: u32,
    pub DP_DPHY_SYM32_VC_RATE_CNTL1: u32,
    pub DP_DPHY_SYM32_VC_RATE_CNTL2: u32,
    pub DP_DPHY_SYM32_VC_RATE_CNTL3: u32,
    pub DP_DPHY_SYM32_SAT_UPDATE: u32,
    pub RDPCSTX_PHY_CNTL6: [u32; 5],
}

#[repr(C)]
pub struct dcn31_hpo_dp_link_encoder_shift {
    pub DP_LINK_ENC_CLOCK_EN: u8, pub DPHY_RESET: u8, pub DPHY_ENABLE: u8,
    pub PRECODER_ENABLE: u8, pub NUM_LANES: u8, pub MODE: u8, pub STATUS: u8,
    pub SAT_UPDATE_PENDING: u8, pub RATE_UPDATE_PENDING: u8, pub TP_CUSTOM: u8,
    pub TP_SELECT0: u8, pub TP_SELECT1: u8, pub TP_SELECT2: u8, pub TP_SELECT3: u8,
    pub TP_PRBS_SEL0: u8, pub TP_PRBS_SEL1: u8, pub TP_PRBS_SEL2: u8, pub TP_PRBS_SEL3: u8,
    pub TP_SQ_PULSE_WIDTH: u8, pub SAT_STREAM_SOURCE: u8, pub SAT_SLOT_COUNT: u8,
    pub STREAM_VC_RATE_X: u8, pub STREAM_VC_RATE_Y: u8, pub SAT_UPDATE: u8,
    pub RDPCS_PHY_DPALT_DISABLE: u8,
}

#[repr(C)]
pub struct dcn31_hpo_dp_link_encoder_mask {
    pub DP_LINK_ENC_CLOCK_EN: u32, pub DPHY_RESET: u32, pub DPHY_ENABLE: u32,
    pub PRECODER_ENABLE: u32, pub NUM_LANES: u32, pub MODE: u32, pub STATUS: u32,
    pub SAT_UPDATE_PENDING: u32, pub RATE_UPDATE_PENDING: u32, pub TP_CUSTOM: u32,
    pub TP_SELECT0: u32, pub TP_SELECT1: u32, pub TP_SELECT2: u32, pub TP_SELECT3: u32,
    pub TP_PRBS_SEL0: u32, pub TP_PRBS_SEL1: u32, pub TP_PRBS_SEL2: u32, pub TP_PRBS_SEL3: u32,
    pub TP_SQ_PULSE_WIDTH: u32, pub SAT_STREAM_SOURCE: u32, pub SAT_SLOT_COUNT: u32,
    pub STREAM_VC_RATE_X: u32, pub STREAM_VC_RATE_Y: u32, pub SAT_UPDATE: u32,
    pub RDPCS_PHY_DPALT_DISABLE: u32,
}

#[repr(C)]
pub struct dcn31_hpo_dp_link_encoder {
    pub base: hpo_dp_link_encoder,
    pub regs: *const dcn31_hpo_dp_link_encoder_registers,
    pub hpo_le_shift: *const dcn31_hpo_dp_link_encoder_shift,
    pub hpo_le_mask: *const dcn31_hpo_dp_link_encoder_mask,
}

extern "C" {
    pub fn hpo_dp_link_encoder31_construct(
        enc31: *mut dcn31_hpo_dp_link_encoder, ctx: *mut dc_context, inst: u32,
        hpo_le_regs: *const dcn31_hpo_dp_link_encoder_registers,
        hpo_le_shift: *const dcn31_hpo_dp_link_encoder_shift,
        hpo_le_mask: *const dcn31_hpo_dp_link_encoder_mask,
    );
    pub fn dcn31_hpo_dp_link_enc_enable_dp_output(
        enc: *mut hpo_dp_link_encoder, link_settings: *const dc_link_settings,
        transmitter: transmitter, hpd_source: hpd_source_id,
    );
    pub fn dcn31_hpo_dp_link_enc_disable_output(enc: *mut hpo_dp_link_encoder, signal: signal_type);
    pub fn dcn31_hpo_dp_link_enc_enable(enc: *mut hpo_dp_link_encoder, num_lanes: dc_lane_count);
    pub fn dcn31_hpo_dp_link_enc_disable(enc: *mut hpo_dp_link_encoder);
    pub fn dcn31_hpo_dp_link_enc_set_link_test_pattern(enc: *mut hpo_dp_link_encoder, tp_params: *mut encoder_set_dp_phy_pattern_param);
    pub fn dcn31_hpo_dp_link_enc_update_stream_allocation_table(enc: *mut hpo_dp_link_encoder, table: *const link_mst_stream_allocation_table);
    pub fn dcn31_hpo_dp_link_enc_set_throttled_vcp_size(enc: *mut hpo_dp_link_encoder, stream_encoder_inst: u32, avg_time_slots_per_mtp: fixed31_32);
    pub fn dcn31_hpo_dp_link_enc_read_state(enc: *mut hpo_dp_link_encoder, state: *mut hpo_dp_link_enc_state);
    pub fn dcn31_hpo_dp_link_enc_set_ffe(enc: *mut hpo_dp_link_encoder, link_settings: *const dc_link_settings, ffe_preset: u8);
    pub fn dcn31_fill_stream_allocation_row_info(stream_allocation: *const link_mst_stream_allocation, src: *mut u32, slots: *mut u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
