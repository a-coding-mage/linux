/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
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

// C dependency: dcn20/dcn20_link_encoder.h

// The following opaque declarations correspond to types supplied by the C
// dependency and are intentionally left incomplete.
pub enum dcn20_link_encoder {}
pub enum encoder_init_data {}
pub enum encoder_feature_support {}
pub enum dcn10_link_enc_registers {}
pub enum dcn10_link_enc_aux_registers {}
pub enum dcn10_link_enc_hpd_registers {}
pub enum dcn10_link_enc_shift {}
pub enum dcn10_link_enc_mask {}
pub enum link_encoder {}
pub enum dc_stream_state {}
pub enum dc_hdmi_frl_link_settings {}
pub enum frl_txffe {}

// C macro translation. SRI, LE_SF, LINK_ENCODER_MASK_SH_LIST_DCN20,
// and DPCS_DCN2_MASK_SH_LIST are supplied by dependent headers.
#[macro_export]
macro_rules! LE_DCN3_REG_LIST {
    ($id:expr) => {
        SRI!(DIG_BE_CNTL, DIG, $id),
        SRI!(DIG_BE_EN_CNTL, DIG, $id),
        SRI!(TMDS_CTL_BITS, DIG, $id),
        SRI!(TMDS_DCBALANCER_CONTROL, DIG, $id),
        SRI!(DP_CONFIG, DP, $id),
        SRI!(DP_DPHY_CNTL, DP, $id),
        SRI!(DP_DPHY_PRBS_CNTL, DP, $id),
        SRI!(DP_DPHY_SCRAM_CNTL, DP, $id),
        SRI!(DP_DPHY_SYM0, DP, $id),
        SRI!(DP_DPHY_SYM1, DP, $id),
        SRI!(DP_DPHY_SYM2, DP, $id),
        SRI!(DP_DPHY_TRAINING_PATTERN_SEL, DP, $id),
        SRI!(DP_LINK_CNTL, DP, $id),
        SRI!(DP_LINK_FRAMING_CNTL, DP, $id),
        SRI!(DP_MSE_SAT0, DP, $id),
        SRI!(DP_MSE_SAT1, DP, $id),
        SRI!(DP_MSE_SAT2, DP, $id),
        SRI!(DP_MSE_SAT_UPDATE, DP, $id),
        SRI!(DP_SEC_CNTL, DP, $id),
        SRI!(DP_VID_STREAM_CNTL, DP, $id),
        SRI!(DP_DPHY_FAST_TRAINING, DP, $id),
        SRI!(DP_SEC_CNTL1, DP, $id),
        SRI!(DP_DPHY_BS_SR_SWAP_CNTL, DP, $id),
        SRI!(DP_DPHY_HBR2_PATTERN_CONTROL, DP, $id)
    };
}

#[macro_export]
macro_rules! LINK_ENCODER_MASK_SH_LIST_DCN30 {
    ($mask_sh:expr) => {
        LINK_ENCODER_MASK_SH_LIST_DCN20!($mask_sh),
        LE_SF!(DIG0_TMDS_DCBALANCER_CONTROL, TMDS_SYNC_DCBAL_EN, $mask_sh)
    };
}

#[macro_export]
macro_rules! DPCS_DCN3_MASK_SH_LIST {
    ($mask_sh:expr) => {
        DPCS_DCN2_MASK_SH_LIST!($mask_sh),
        LE_SF!(DPCSTX0_DPCSTX_TX_CNTL, DPCS_TX_HDMI_FRL_MODE, $mask_sh),
        LE_SF!(DPCSTX0_DPCSTX_TX_CNTL, DPCS_TX_DATA_SWAP_10_BIT, $mask_sh),
        LE_SF!(DPCSTX0_DPCSTX_TX_CNTL, DPCS_TX_DATA_ORDER_INVERT_18_BIT, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_CNTL0, RDPCS_PHY_TX_VBOOST_LVL, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_CLOCK_CNTL, RDPCS_TX_CLK_EN, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_CNTL6, RDPCS_PHY_DPALT_DP4, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_CNTL6, RDPCS_PHY_DPALT_DISABLE, $mask_sh)
    };
}

extern "C" {
    pub fn dcn30_link_encoder_construct(
        enc20: *mut dcn20_link_encoder,
        init_data: *const encoder_init_data,
        enc_features: *const encoder_feature_support,
        link_regs: *const dcn10_link_enc_registers,
        aux_regs: *const dcn10_link_enc_aux_registers,
        hpd_regs: *const dcn10_link_enc_hpd_registers,
        link_shift: *const dcn10_link_enc_shift,
        link_mask: *const dcn10_link_enc_mask,
    );

    pub fn enc3_hw_init(enc: *mut link_encoder);

    pub fn dcn30_link_encoder_validate_output_with_stream(
        enc: *mut link_encoder,
        stream: *const dc_stream_state,
    ) -> bool;

    pub fn dpcs30_program_eq_setting(
        enc: *mut link_encoder,
        FFE_Level: u8,
        de_emphasis_only: bool,
        pre_shoot_only: bool,
        no_ffe: bool,
        link_settings: *const dc_hdmi_frl_link_settings,
    );

    pub fn dpcs30_get_txffe(enc: *mut link_encoder, lane_settings: *mut frl_txffe);

    pub fn dpcs30_set_txffe(enc: *mut link_encoder, lane_settings: *mut frl_txffe);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
