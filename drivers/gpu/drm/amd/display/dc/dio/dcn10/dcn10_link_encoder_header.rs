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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// C dependency: link_encoder.h and the register-description macros.

#[repr(C)]
pub struct dcn10_link_enc_aux_registers {
    pub AUX_CONTROL: u32, pub AUX_DPHY_RX_CONTROL0: u32,
    pub AUX_DPHY_TX_CONTROL: u32, pub AUX_DPHY_RX_CONTROL1: u32,
    pub DC_GPIO_DDC: u32,
}

#[repr(C)]
pub struct dcn10_link_enc_hpd_registers {
    pub DC_HPD_CONTROL: u32, pub DC_HPD_INT_STATUS: u32,
    pub DC_HPD_TOGGLE_FILT_CNTL: u32, pub HPD_CTRL: u32,
}

#[repr(C)]
pub struct dcn10_link_enc_registers {
    pub DIG_BE_CNTL: u32, pub DIG_BE_EN_CNTL: u32, pub DIG_CLOCK_PATTERN: u32,
    pub DP_CONFIG: u32, pub DP_DPHY_CNTL: u32, pub DP_DPHY_INTERNAL_CTRL: u32,
    pub DP_DPHY_PRBS_CNTL: u32, pub DP_DPHY_SCRAM_CNTL: u32,
    pub DP_DPHY_SYM0: u32, pub DP_DPHY_SYM1: u32, pub DP_DPHY_SYM2: u32,
    pub DP_DPHY_TRAINING_PATTERN_SEL: u32, pub DP_LINK_CNTL: u32,
    pub DP_LINK_FRAMING_CNTL: u32, pub DP_MSE_SAT0: u32, pub DP_MSE_SAT1: u32,
    pub DP_MSE_SAT2: u32, pub DP_MSE_SAT_UPDATE: u32, pub DP_SEC_CNTL: u32,
    pub DP_VID_STREAM_CNTL: u32, pub DP_DPHY_FAST_TRAINING: u32,
    pub DP_DPHY_BS_SR_SWAP_CNTL: u32, pub DP_DPHY_HBR2_PATTERN_CONTROL: u32,
    pub DP_SEC_CNTL1: u32, pub TMDS_CTL_BITS: u32,
    // DCCG
    pub CLOCK_ENABLE: u32,
    // DIG
    pub DIG_LANE_ENABLE: u32,
    // UNIPHY
    pub CHANNEL_XBAR_CNTL: u32,
    // DPCS
    pub RDPCSTX_PHY_CNTL3: u32, pub RDPCSTX_PHY_CNTL4: u32,
    pub RDPCSTX_PHY_CNTL5: u32, pub RDPCSTX_PHY_CNTL6: u32,
    pub RDPCSPIPE_PHY_CNTL6: u32, pub RDPCSTX_PHY_CNTL7: u32,
    pub RDPCSTX_PHY_CNTL8: u32, pub RDPCSTX_PHY_CNTL9: u32,
    pub RDPCSTX_PHY_CNTL10: u32, pub RDPCSTX_PHY_CNTL11: u32,
    pub RDPCSTX_PHY_CNTL12: u32, pub RDPCSTX_PHY_CNTL13: u32,
    pub RDPCSTX_PHY_CNTL14: u32, pub RDPCSTX_PHY_CNTL15: u32,
    pub RDPCSTX_CNTL: u32, pub RDPCSTX_CLOCK_CNTL: u32,
    pub RDPCSTX_PHY_CNTL0: u32, pub RDPCSTX_PHY_CNTL2: u32,
    pub RDPCSTX_PLL_UPDATE_DATA: u32, pub RDPCS_TX_CR_ADDR: u32,
    pub RDPCS_TX_CR_DATA: u32, pub DPCSTX_TX_CLOCK_CNTL: u32,
    pub DPCSTX_TX_CNTL: u32, pub RDPCSTX_INTERRUPT_CONTROL: u32,
    pub RDPCSTX_PHY_FUSE0: u32, pub RDPCSTX_PHY_FUSE1: u32,
    pub RDPCSTX_PHY_FUSE2: u32, pub RDPCSTX_PHY_FUSE3: u32,
    pub RDPCSTX_PHY_RX_LD_VAL: u32, pub DPCSTX_DEBUG_CONFIG: u32,
    pub RDPCSTX_DEBUG_CONFIG: u32, pub RDPCSTX0_RDPCSTX_SCRATCH: u32,
    pub RDPCSTX_DMCU_DPALT_DIS_BLOCK_REG: u32, pub DCIO_SOFT_RESET: u32,
    // indirect registers
    pub RAWLANE0_DIG_PCS_XF_RX_OVRD_IN_2: u32, pub RAWLANE0_DIG_PCS_XF_RX_OVRD_IN_3: u32,
    pub RAWLANE1_DIG_PCS_XF_RX_OVRD_IN_2: u32, pub RAWLANE1_DIG_PCS_XF_RX_OVRD_IN_3: u32,
    pub RAWLANE2_DIG_PCS_XF_RX_OVRD_IN_2: u32, pub RAWLANE2_DIG_PCS_XF_RX_OVRD_IN_3: u32,
    pub RAWLANE3_DIG_PCS_XF_RX_OVRD_IN_2: u32, pub RAWLANE3_DIG_PCS_XF_RX_OVRD_IN_3: u32,
    pub TMDS_DCBALANCER_CONTROL: u32, pub PHYA_LINK_CNTL2: u32, pub PHYB_LINK_CNTL2: u32,
    pub PHYC_LINK_CNTL2: u32, pub DIO_LINKA_CNTL: u32, pub DIO_LINKB_CNTL: u32,
    pub DIO_LINKC_CNTL: u32, pub DIO_LINKD_CNTL: u32, pub DIO_LINKE_CNTL: u32,
    pub DIO_LINKF_CNTL: u32, pub DIO_CLK_CNTL: u32, pub DIG_BE_CLK_CNTL: u32,
    pub HDCP_I2C_CONTROL_0: u32, pub HDCP_INT_CONTROL: u32,
}

// Register-field lists are retained as dependency-facing macro hooks.  The
// external register-generation layer supplies the concrete field constants.
#[macro_export] macro_rules! LE_SF { ($reg:ident, $field:ident, $post:ident) => { $field: u32 }; }
#[macro_export] macro_rules! TO_DCN10_LINK_ENC { ($link_encoder:expr) => { $link_encoder as *mut dcn10_link_encoder }; }

#[repr(C)]
pub struct dcn10_link_enc_shift {
    pub DIG_ENABLE: u8, pub DIG_HPD_SELECT: u8, pub DIG_MODE: u8,
    pub DIG_FE_SOURCE_SELECT: u8, pub DIG_CLOCK_PATTERN: u8,
    pub DPHY_BYPASS: u8, pub DPHY_ATEST_SEL_LANE0: u8,
    pub DPHY_ATEST_SEL_LANE1: u8, pub DPHY_ATEST_SEL_LANE2: u8,
    pub DPHY_ATEST_SEL_LANE3: u8, pub DPHY_PRBS_EN: u8, pub DPHY_PRBS_SEL: u8,
    pub DPHY_SYM1: u8, pub DPHY_SYM2: u8, pub DPHY_SYM3: u8, pub DPHY_SYM4: u8,
    pub DPHY_SYM5: u8, pub DPHY_SYM6: u8, pub DPHY_SYM7: u8, pub DPHY_SYM8: u8,
    pub DPHY_SCRAMBLER_BS_COUNT: u8, pub DPHY_SCRAMBLER_ADVANCE: u8,
    pub DPHY_RX_FAST_TRAINING_CAPABLE: u8, pub DPHY_LOAD_BS_COUNT: u8,
    pub DPHY_TRAINING_PATTERN_SEL: u8, pub DP_DPHY_HBR2_PATTERN_CONTROL: u8,
    pub DP_LINK_TRAINING_COMPLETE: u8, pub DP_IDLE_BS_INTERVAL: u8,
    pub DP_VBID_DISABLE: u8, pub DP_VID_ENHANCED_FRAME_MODE: u8,
    pub DP_VID_STREAM_ENABLE: u8, pub DP_UDI_LANES: u8,
    pub DP_SEC_GSP0_LINE_NUM: u8, pub DP_SEC_GSP0_PRIORITY: u8,
    pub DP_MSE_SAT_SRC0: u8, pub DP_MSE_SAT_SRC1: u8, pub DP_MSE_SAT_SRC2: u8,
    pub DP_MSE_SAT_SRC3: u8, pub DP_MSE_SAT_SLOT_COUNT0: u8,
    pub DP_MSE_SAT_SLOT_COUNT1: u8, pub DP_MSE_SAT_SLOT_COUNT2: u8,
    pub DP_MSE_SAT_SLOT_COUNT3: u8, pub DP_MSE_SAT_UPDATE: u8,
    pub DP_MSE_16_MTP_KEEPOUT: u8, pub DC_HPD_EN: u8, pub TMDS_CTL0: u8,
    pub AUX_HPD_SEL: u8, pub AUX_LS_READ_EN: u8, pub AUX_RX_RECEIVE_WINDOW: u8,
    pub DC_HPD_SENSE: u8, pub DC_HPD_CONNECT_INT_DELAY: u8,
    pub DC_HPD_DISCONNECT_INT_DELAY: u8,
}

#[repr(C)]
pub struct dcn10_link_enc_mask { pub fields: dcn10_link_enc_shift; }

#[repr(C)]
pub struct dcn10_link_encoder {
    pub base: link_encoder,
    pub link_regs: *const dcn10_link_enc_registers,
    pub aux_regs: *const dcn10_link_enc_aux_registers,
    pub hpd_regs: *const dcn10_link_enc_hpd_registers,
    pub link_shift: *const dcn10_link_enc_shift,
    pub link_mask: *const dcn10_link_enc_mask,
}

extern "C" {
    pub fn dcn10_link_encoder_construct(enc10: *mut dcn10_link_encoder, init_data: *const encoder_init_data, enc_features: *const encoder_feature_support, link_regs: *const dcn10_link_enc_registers, aux_regs: *const dcn10_link_enc_aux_registers, hpd_regs: *const dcn10_link_enc_hpd_registers, link_shift: *const dcn10_link_enc_shift, link_mask: *const dcn10_link_enc_mask);
    pub fn dcn10_link_encoder_validate_dvi_output(enc10: *const dcn10_link_encoder, connector_signal: signal_type, signal: signal_type, crtc_timing: *const dc_crtc_timing) -> bool;
    pub fn dcn10_link_encoder_validate_rgb_output(enc10: *const dcn10_link_encoder, crtc_timing: *const dc_crtc_timing) -> bool;
    pub fn dcn10_link_encoder_validate_dp_output(enc10: *const dcn10_link_encoder, crtc_timing: *const dc_crtc_timing) -> bool;
    pub fn dcn10_link_encoder_validate_wireless_output(enc10: *const dcn10_link_encoder, crtc_timing: *const dc_crtc_timing) -> bool;
    pub fn dcn10_link_encoder_validate_output_with_stream(enc: *mut link_encoder, stream: *const dc_stream_state) -> bool;
    pub fn dcn10_link_encoder_hw_init(enc: *mut link_encoder);
    pub fn dcn10_link_encoder_destroy(enc: *mut *mut link_encoder);
    pub fn dcn10_link_encoder_setup(enc: *mut link_encoder, signal: signal_type);
    pub fn enc1_configure_encoder(enc10: *mut dcn10_link_encoder, link_settings: *const dc_link_settings);
    pub fn dcn10_link_encoder_enable_tmds_output(enc: *mut link_encoder, clock_source: clock_source_id, color_depth: dc_color_depth, signal: signal_type, pixel_clock: u32);
    pub fn dcn10_link_encoder_enable_tmds_output_with_clk_pattern_wa(enc: *mut link_encoder, clock_source: clock_source_id, color_depth: dc_color_depth, signal: signal_type, pixel_clock: u32);
    pub fn dcn10_link_encoder_enable_dp_output(enc: *mut link_encoder, link_settings: *const dc_link_settings, clock_source: clock_source_id);
    pub fn dcn10_link_encoder_enable_dp_mst_output(enc: *mut link_encoder, link_settings: *const dc_link_settings, clock_source: clock_source_id);
    pub fn dcn10_link_encoder_disable_output(enc: *mut link_encoder, signal: signal_type);
    pub fn dcn10_link_encoder_dp_set_lane_settings(enc: *mut link_encoder, link_settings: *const dc_link_settings, lane_settings: *const lane_settings);
    pub fn dcn10_link_encoder_dp_set_phy_pattern(enc: *mut link_encoder, param: *const encoder_set_dp_phy_pattern_param);
    pub fn dcn10_link_encoder_update_mst_stream_allocation_table(enc: *mut link_encoder, table: *const link_mst_stream_allocation_table);
    pub fn dcn10_link_encoder_connect_dig_be_to_fe(enc: *mut link_encoder, engine: engine_id, connect: bool);
    pub fn dcn10_link_encoder_set_dp_phy_pattern_training_pattern(enc: *mut link_encoder, index: u32);
    pub fn dcn10_link_encoder_enable_hpd(enc: *mut link_encoder);
    pub fn dcn10_link_encoder_disable_hpd(enc: *mut link_encoder);
    pub fn dcn10_psr_program_dp_dphy_fast_training(enc: *mut link_encoder, exit_link_training_required: bool);
    pub fn dcn10_psr_program_secondary_packet(enc: *mut link_encoder, sdp_transmit_line_num_deadline: u32);
    pub fn dcn10_is_dig_enabled(enc: *mut link_encoder) -> bool;
    pub fn dcn10_get_dig_frontend(enc: *mut link_encoder) -> u32;
    pub fn dcn10_aux_initialize(enc10: *mut dcn10_link_encoder);
    pub fn dcn10_get_dig_mode(enc: *mut link_encoder) -> signal_type;
    pub fn dcn10_link_encoder_get_max_link_cap(enc: *mut link_encoder, link_settings: *mut dc_link_settings);
    pub fn dcn10_get_hpd_state(enc: *mut link_encoder) -> bool;
    pub fn dcn10_program_hpd_filter(enc: *mut link_encoder, delay_on_connect_in_ms: i32, delay_on_disconnect_in_ms: i32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
