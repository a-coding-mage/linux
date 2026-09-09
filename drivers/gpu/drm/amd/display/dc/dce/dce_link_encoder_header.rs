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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency declarations and preprocessor conditions are supplied externally.

#[macro_export]
macro_rules! TO_DCE110_LINK_ENC { ($link_encoder:expr) => { container_of!($link_encoder, dce110_link_encoder, base) }; }

// Not found regs in dce120 spec: BIOS_SCRATCH_2, DP_DPHY_INTERNAL_CTRL

#[macro_export]
macro_rules! AUX_REG_LIST { ($id:expr) => { SRI!(AUX_CONTROL, DP_AUX, $id), SRI!(AUX_DPHY_RX_CONTROL0, DP_AUX, $id), SRI!(AUX_DPHY_RX_CONTROL1, DP_AUX, $id) }; }
#[macro_export]
macro_rules! HPD_REG_LIST { ($id:expr) => { SRI!(DC_HPD_CONTROL, HPD, $id) }; }
#[macro_export]
macro_rules! LE_COMMON_REG_LIST_BASE { ($id:expr) => {
    SR!(DMCU_RAM_ACCESS_CTRL), SR!(DMCU_IRAM_RD_CTRL), SR!(DMCU_IRAM_RD_DATA),
    SR!(DMCU_INTERRUPT_TO_UC_EN_MASK), SRI!(DIG_BE_CNTL, DIG, $id), SRI!(DIG_BE_EN_CNTL, DIG, $id),
    SRI!(DP_CONFIG, DP, $id), SRI!(DP_DPHY_CNTL, DP, $id), SRI!(DP_DPHY_PRBS_CNTL, DP, $id),
    SRI!(DP_DPHY_SCRAM_CNTL, DP, $id), SRI!(DP_DPHY_SYM0, DP, $id), SRI!(DP_DPHY_SYM1, DP, $id),
    SRI!(DP_DPHY_SYM2, DP, $id), SRI!(DP_DPHY_TRAINING_PATTERN_SEL, DP, $id), SRI!(DP_LINK_CNTL, DP, $id),
    SRI!(DP_LINK_FRAMING_CNTL, DP, $id), SRI!(DP_MSE_SAT0, DP, $id), SRI!(DP_MSE_SAT1, DP, $id),
    SRI!(DP_MSE_SAT2, DP, $id), SRI!(DP_MSE_SAT_UPDATE, DP, $id), SRI!(DP_SEC_CNTL, DP, $id),
    SRI!(DP_VID_STREAM_CNTL, DP, $id), SRI!(DP_DPHY_FAST_TRAINING, DP, $id), SRI!(DP_SEC_CNTL1, DP, $id)
}; }
#[macro_export]
macro_rules! LE_COMMON_REG_LIST { ($id:expr) => { LE_COMMON_REG_LIST_BASE!($id), SRI!(DP_DPHY_BS_SR_SWAP_CNTL, DP, $id), SRI!(DP_DPHY_INTERNAL_CTRL, DP, $id), SR!(DCI_MEM_PWR_STATUS) }; }
#[cfg(feature = "CONFIG_DRM_AMD_DC_SI")]
#[macro_export]
macro_rules! LE_DCE60_REG_LIST { ($id:expr) => {
    SRI!(DP_DPHY_INTERNAL_CTRL, DP, $id), SR!(DMCU_RAM_ACCESS_CTRL), SR!(DMCU_IRAM_RD_CTRL),
    SR!(DMCU_IRAM_RD_DATA), SR!(DMCU_INTERRUPT_TO_UC_EN_MASK), SRI!(DIG_BE_CNTL, DIG, $id),
    SRI!(DIG_BE_EN_CNTL, DIG, $id), SRI!(DP_CONFIG, DP, $id), SRI!(DP_DPHY_CNTL, DP, $id),
    SRI!(DP_DPHY_PRBS_CNTL, DP, $id), SRI!(DP_DPHY_SYM0, DP, $id), SRI!(DP_DPHY_SYM1, DP, $id),
    SRI!(DP_DPHY_SYM2, DP, $id), SRI!(DP_DPHY_TRAINING_PATTERN_SEL, DP, $id), SRI!(DP_LINK_CNTL, DP, $id),
    SRI!(DP_LINK_FRAMING_CNTL, DP, $id), SRI!(DP_MSE_SAT0, DP, $id), SRI!(DP_MSE_SAT1, DP, $id),
    SRI!(DP_MSE_SAT2, DP, $id), SRI!(DP_MSE_SAT_UPDATE, DP, $id), SRI!(DP_SEC_CNTL, DP, $id),
    SRI!(DP_VID_STREAM_CNTL, DP, $id), SRI!(DP_DPHY_FAST_TRAINING, DP, $id), SRI!(DP_SEC_CNTL1, DP, $id)
}; }
#[macro_export]
macro_rules! LE_DCE80_REG_LIST { ($id:expr) => { SRI!(DP_DPHY_INTERNAL_CTRL, DP, $id), LE_COMMON_REG_LIST_BASE!($id), SR!(DAC_ENABLE) }; }
#[macro_export]
macro_rules! LE_DCE100_REG_LIST { ($id:expr) => { LE_COMMON_REG_LIST_BASE!($id), SRI!(DP_DPHY_BS_SR_SWAP_CNTL, DP, $id), SRI!(DP_DPHY_INTERNAL_CTRL, DP, $id), SR!(DCI_MEM_PWR_STATUS), SR!(DAC_ENABLE) }; }
#[macro_export]
macro_rules! LE_DCE110_REG_LIST { ($id:expr) => { LE_COMMON_REG_LIST_BASE!($id), SRI!(DP_DPHY_BS_SR_SWAP_CNTL, DP, $id), SRI!(DP_DPHY_INTERNAL_CTRL, DP, $id), SRI!(DP_DPHY_HBR2_PATTERN_CONTROL, DP, $id), SR!(DCI_MEM_PWR_STATUS) }; }
#[macro_export]
macro_rules! LE_DCE120_REG_LIST { ($id:expr) => { LE_COMMON_REG_LIST_BASE!($id), SRI!(DP_DPHY_BS_SR_SWAP_CNTL, DP, $id), SRI!(DP_DPHY_HBR2_PATTERN_CONTROL, DP, $id), SR!(DCI_MEM_PWR_STATUS) }; }

#[repr(C)] pub struct dce110_link_enc_aux_registers { pub AUX_CONTROL: u32, pub AUX_DPHY_RX_CONTROL0: u32, pub AUX_DPHY_RX_CONTROL1: u32 }
#[repr(C)] pub struct dce110_link_enc_hpd_registers { pub DC_HPD_CONTROL: u32 }
#[repr(C)] pub struct dce110_link_enc_registers {
    pub MASTER_COMM_DATA_REG1: u32, pub MASTER_COMM_DATA_REG2: u32, pub MASTER_COMM_DATA_REG3: u32,
    pub MASTER_COMM_CMD_REG: u32, pub MASTER_COMM_CNTL_REG: u32, pub DMCU_RAM_ACCESS_CTRL: u32,
    pub DCI_MEM_PWR_STATUS: u32, pub DMU_MEM_PWR_CNTL: u32, pub DMCU_IRAM_RD_CTRL: u32,
    pub DMCU_IRAM_RD_DATA: u32, pub DMCU_INTERRUPT_TO_UC_EN_MASK: u32,
    pub DIG_BE_CNTL: u32, pub DIG_BE_EN_CNTL: u32, pub DP_CONFIG: u32, pub DP_DPHY_CNTL: u32,
    pub DP_DPHY_INTERNAL_CTRL: u32, pub DP_DPHY_PRBS_CNTL: u32, pub DP_DPHY_SCRAM_CNTL: u32,
    pub DP_DPHY_SYM0: u32, pub DP_DPHY_SYM1: u32, pub DP_DPHY_SYM2: u32,
    pub DP_DPHY_TRAINING_PATTERN_SEL: u32, pub DP_LINK_CNTL: u32, pub DP_LINK_FRAMING_CNTL: u32,
    pub DP_MSE_SAT0: u32, pub DP_MSE_SAT1: u32, pub DP_MSE_SAT2: u32, pub DP_MSE_SAT_UPDATE: u32,
    pub DP_SEC_CNTL: u32, pub DP_VID_STREAM_CNTL: u32, pub DP_DPHY_FAST_TRAINING: u32,
    pub DP_DPHY_BS_SR_SWAP_CNTL: u32, pub DP_DPHY_HBR2_PATTERN_CONTROL: u32, pub DP_SEC_CNTL1: u32,
    pub DAC_ENABLE: u32,
}
#[repr(C)] pub struct dce110_link_encoder { pub base: link_encoder, pub link_regs: *const dce110_link_enc_registers, pub aux_regs: *const dce110_link_enc_aux_registers, pub hpd_regs: *const dce110_link_enc_hpd_registers }

extern "C" {
    pub fn dce110_link_encoder_construct(enc110: *mut dce110_link_encoder, init_data: *const encoder_init_data, enc_features: *const encoder_feature_support, link_regs: *const dce110_link_enc_registers, aux_regs: *const dce110_link_enc_aux_registers, hpd_regs: *const dce110_link_enc_hpd_registers);
    #[cfg(feature = "CONFIG_DRM_AMD_DC_SI")] pub fn dce60_link_encoder_construct(enc110: *mut dce110_link_encoder, init_data: *const encoder_init_data, enc_features: *const encoder_feature_support, link_regs: *const dce110_link_enc_registers, aux_regs: *const dce110_link_enc_aux_registers, hpd_regs: *const dce110_link_enc_hpd_registers);
    pub fn dce110_link_encoder_validate_dvi_output(enc110: *const dce110_link_encoder, connector_signal: signal_type, signal: signal_type, crtc_timing: *const dc_crtc_timing) -> bool;
    pub fn dce110_link_encoder_validate_dp_output(enc110: *const dce110_link_encoder, crtc_timing: *const dc_crtc_timing) -> bool;
    pub fn dce110_link_encoder_validate_wireless_output(enc110: *const dce110_link_encoder, crtc_timing: *const dc_crtc_timing) -> bool;
    pub fn dce110_link_encoder_validate_output_with_stream(enc: *mut link_encoder, stream: *const dc_stream_state) -> bool;
    pub fn dce110_link_encoder_hw_init(enc: *mut link_encoder);
    pub fn dce110_link_encoder_destroy(enc: *mut *mut link_encoder);
    pub fn dce110_link_encoder_setup(enc: *mut link_encoder, signal: signal_type);
    pub fn dce110_link_encoder_enable_tmds_output(enc: *mut link_encoder, clock_source: clock_source_id, color_depth: dc_color_depth, signal: signal_type, pixel_clock: u32);
    pub fn dce110_link_encoder_enable_dp_output(enc: *mut link_encoder, link_settings: *const dc_link_settings, clock_source: clock_source_id);
    pub fn dce110_link_encoder_enable_dp_mst_output(enc: *mut link_encoder, link_settings: *const dc_link_settings, clock_source: clock_source_id);
    pub fn dce110_link_encoder_enable_lvds_output(enc: *mut link_encoder, clock_source: clock_source_id, pixel_clock: u32);
    pub fn dce110_link_encoder_enable_analog_output(enc: *mut link_encoder, pixel_clock: u32);
    pub fn dce110_link_encoder_disable_output(enc: *mut link_encoder, signal: signal_type);
    pub fn dce110_link_encoder_dp_set_lane_settings(enc: *mut link_encoder, link_settings: *const dc_link_settings, lane_settings: *const dc_lane_settings);
    pub fn dce110_link_encoder_dp_set_phy_pattern(enc: *mut link_encoder, param: *const encoder_set_dp_phy_pattern_param);
    pub fn dce110_link_encoder_update_mst_stream_allocation_table(enc: *mut link_encoder, table: *const link_mst_stream_allocation_table);
    pub fn dce110_link_encoder_connect_dig_be_to_fe(enc: *mut link_encoder, engine: engine_id, connect: bool);
    pub fn dce110_get_dig_frontend(enc: *mut link_encoder) -> u32;
    pub fn dce110_link_encoder_set_dp_phy_pattern_training_pattern(enc: *mut link_encoder, index: u32);
    pub fn dce110_link_encoder_enable_hpd(enc: *mut link_encoder);
    pub fn dce110_link_encoder_disable_hpd(enc: *mut link_encoder);
    pub fn dce110_psr_program_dp_dphy_fast_training(enc: *mut link_encoder, exit_link_training_required: bool);
    pub fn dce110_psr_program_secondary_packet(enc: *mut link_encoder, sdp_transmit_line_num_deadline: u32);
    pub fn dce110_is_dig_enabled(enc: *mut link_encoder) -> bool;
    pub fn dce110_link_encoder_get_max_link_cap(enc: *mut link_encoder, link_settings: *mut dc_link_settings);
    pub fn dce110_get_hpd_state(enc: *mut link_encoder) -> bool;
    pub fn dce110_program_hpd_filter(enc: *mut link_encoder, delay_on_connect_in_ms: i32, delay_on_disconnect_in_ms: i32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
