/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

#[repr(C)]
pub struct dcn21_link_encoder {
    pub enc10: dcn10_link_encoder,
    pub phy_seq_cfg: dpcssys_phy_seq_cfg,
}

macro_rules! DPCS_DCN21_MASK_SH_LIST {
    ($mask_sh:ident) => {
        DPCS_DCN2_MASK_SH_LIST!($mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE3, RDPCS_PHY_TX_VBOOST_LVL, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE2, RDPCS_PHY_DP_MPLLB_CP_PROP_GS, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE0, RDPCS_PHY_RX_VREF_CTRL, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE0, RDPCS_PHY_DP_MPLLB_CP_INT_GS, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_DMCU_DPALT_DIS_BLOCK_REG, RDPCS_DMCU_DPALT_DIS_BLOCK_REG, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_CNTL15, RDPCS_PHY_SUP_PRE_HP, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_CNTL15, RDPCS_PHY_DP_TX0_VREGDRV_BYP, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_CNTL15, RDPCS_PHY_DP_TX1_VREGDRV_BYP, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_CNTL15, RDPCS_PHY_DP_TX2_VREGDRV_BYP, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_CNTL15, RDPCS_PHY_DP_TX3_VREGDRV_BYP, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_CNTL6, RDPCS_PHY_DPALT_DP4, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_CNTL6, RDPCS_PHY_DPALT_DISABLE, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE0, RDPCS_PHY_DP_TX0_EQ_MAIN, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE0, RDPCS_PHY_DP_TX0_EQ_PRE, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE0, RDPCS_PHY_DP_TX0_EQ_POST, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE1, RDPCS_PHY_DP_TX1_EQ_MAIN, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE1, RDPCS_PHY_DP_TX1_EQ_PRE, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE1, RDPCS_PHY_DP_TX1_EQ_POST, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE2, RDPCS_PHY_DP_TX2_EQ_MAIN, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE2, RDPCS_PHY_DP_TX2_EQ_PRE, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE2, RDPCS_PHY_DP_TX2_EQ_POST, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE3, RDPCS_PHY_DP_TX3_EQ_MAIN, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE3, RDPCS_PHY_DCO_FINETUNE, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE3, RDPCS_PHY_DCO_RANGE, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE3, RDPCS_PHY_DP_TX3_EQ_PRE, $mask_sh),
        LE_SF!(RDPCSTX0_RDPCSTX_PHY_FUSE3, RDPCS_PHY_DP_TX3_EQ_POST, $mask_sh),
        LE_SF!(DCIO_SOFT_RESET, UNIPHYA_SOFT_RESET, $mask_sh),
        LE_SF!(DCIO_SOFT_RESET, UNIPHYB_SOFT_RESET, $mask_sh),
        LE_SF!(DCIO_SOFT_RESET, UNIPHYC_SOFT_RESET, $mask_sh),
        LE_SF!(DCIO_SOFT_RESET, UNIPHYD_SOFT_RESET, $mask_sh),
        LE_SF!(DCIO_SOFT_RESET, UNIPHYE_SOFT_RESET, $mask_sh)
    };
}

macro_rules! DPCS_DCN21_REG_LIST {
    ($id:ident) => {
        DPCS_DCN2_REG_LIST!($id),
        SRI!(RDPCSTX_PHY_CNTL15, RDPCSTX, $id),
        SRI!(RDPCSTX_DMCU_DPALT_DIS_BLOCK_REG, RDPCSTX, $id)
    };
}

macro_rules! LINK_ENCODER_MASK_SH_LIST_DCN21 {
    ($mask_sh:ident) => {
        LINK_ENCODER_MASK_SH_LIST_DCN20!($mask_sh),
        LE_SF!(UNIPHYA_CHANNEL_XBAR_CNTL, UNIPHY_CHANNEL0_XBAR_SOURCE, $mask_sh),
        LE_SF!(UNIPHYA_CHANNEL_XBAR_CNTL, UNIPHY_CHANNEL1_XBAR_SOURCE, $mask_sh),
        LE_SF!(UNIPHYA_CHANNEL_XBAR_CNTL, UNIPHY_CHANNEL2_XBAR_SOURCE, $mask_sh),
        LE_SF!(UNIPHYA_CHANNEL_XBAR_CNTL, UNIPHY_CHANNEL3_XBAR_SOURCE, $mask_sh),
        SRI!(RDPCSTX_PHY_FUSE2, RDPCSTX, id),
        SRI!(RDPCSTX_PHY_FUSE3, RDPCSTX, id),
        SR!(RDPCSTX0_RDPCSTX_SCRATCH)
    };
}

extern "C" {
    pub fn dcn21_link_encoder_enable_dp_output(
        enc: *mut link_encoder,
        link_settings: *const dc_link_settings,
        clock_source: clock_source_id,
    );

    pub fn dcn21_link_encoder_construct(
        enc21: *mut dcn21_link_encoder,
        init_data: *const encoder_init_data,
        enc_features: *const encoder_feature_support,
        link_regs: *const dcn10_link_enc_registers,
        aux_regs: *const dcn10_link_enc_aux_registers,
        hpd_regs: *const dcn10_link_enc_hpd_registers,
        link_shift: *const dcn10_link_enc_shift,
        link_mask: *const dcn10_link_enc_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
