/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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

// C dependency: dcn20/dcn20_link_encoder.h

// Opaque declarations supplied by the dependent headers.
#[repr(C)]
pub struct dcn20_link_encoder {
    _private: [u8; 0],
}
#[repr(C)]
pub struct encoder_init_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct encoder_feature_support {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dcn10_link_enc_registers {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dcn10_link_enc_aux_registers {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dcn10_link_enc_hpd_registers {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dcn10_link_enc_shift {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dcn10_link_enc_mask {
    _private: [u8; 0],
}
#[repr(C)]
pub struct link_encoder {
    _private: [u8; 0],
}

// LE_DCN301_REG_LIST(id)
#[macro_export]
macro_rules! LE_DCN301_REG_LIST {
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

// LINK_ENCODER_MASK_SH_LIST_DCN301(mask_sh)
#[macro_export]
macro_rules! LINK_ENCODER_MASK_SH_LIST_DCN301 {
    ($mask_sh:expr) => {
        LINK_ENCODER_MASK_SH_LIST_DCN20!($mask_sh),
        LE_SF!(DIG0_TMDS_DCBALANCER_CONTROL, TMDS_SYNC_DCBAL_EN, $mask_sh)
    };
}

// DPCS_DCN301_MASK_SH_LIST(mask_sh)
#[macro_export]
macro_rules! DPCS_DCN301_MASK_SH_LIST {
    ($mask_sh:expr) => {
        DPCS_DCN2_MASK_SH_LIST!($mask_sh),
        LE_SF!(DPCSTX0_DPCSTX_TX_CNTL, DPCS_TX_HDMI_FRL_MODE, $mask_sh),
        LE_SF!(DPCSTX0_DPCSTX_TX_CNTL, DPCS_TX_DATA_SWAP_10_BIT, $mask_sh),
        LE_SF!(DPCSTX0_DPCSTX_TX_CNTL, DPCS_TX_DATA_ORDER_INVERT_18_BIT, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_CNTL0, RDPCS_PHY_TX_VBOOST_LVL, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_CLOCK_CNTL, RDPCS_TX_CLK_EN, $mask_sh)
    };
}

extern "C" {
    pub fn dcn301_link_encoder_construct(
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
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
