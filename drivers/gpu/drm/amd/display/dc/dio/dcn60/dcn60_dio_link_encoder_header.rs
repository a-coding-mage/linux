// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependency: dcn30/dcn30_dio_link_encoder.h

// Translation of LINK_ENCODER_MASK_SH_LIST_DCN60(mask_sh).
// The referenced LE_SF and SF macros are supplied by the dependent headers.
#[macro_export]
macro_rules! LINK_ENCODER_MASK_SH_LIST_DCN60 {
    ($mask_sh:expr) => {
        LE_SF!(DIG0_HDCP_I2C_CONTROL_0, HDCP_I2C_DISABLE, $mask_sh),
        LE_SF!(DIG0_HDCP_I2C_CONTROL_0, HDCP_I2C_DDC_SELECT, $mask_sh),
        LE_SF!(DIG0_HDCP_INT_CONTROL, HDCP_I2C_XFER_REQ_MASK, $mask_sh),
        LE_SF!(HPD0_DC_HPD_INT_STATUS, DC_HPD_SENSE, $mask_sh),
        LE_SF!(HPD0_DC_HPD_TOGGLE_FILT_CNTL, DC_HPD_CONNECT_INT_DELAY, $mask_sh),
        LE_SF!(HPD0_DC_HPD_TOGGLE_FILT_CNTL, DC_HPD_DISCONNECT_INT_DELAY, $mask_sh),
        LE_SF!(DC_GPIO_DDC1_MASK, AUX_PAD1_MODE, $mask_sh),
        SF!(HPD_CTRL, HPD1_Y_POL_INVERT, $mask_sh),
        SF!(HPD_CTRL, HPD2_Y_POL_INVERT, $mask_sh),
        SF!(HPD_CTRL, HPD3_Y_POL_INVERT, $mask_sh),
        SF!(HPD_CTRL, HPD4_Y_POL_INVERT, $mask_sh)
    };
}

unsafe extern "C" {
    pub fn dcn60_link_encoder_construct(
        enc20: *mut dcn20_link_encoder,
        init_data: *const encoder_init_data,
        enc_features: *const encoder_feature_support,
        link_regs: *const dcn10_link_enc_registers,
        aux_regs: *const dcn10_link_enc_aux_registers,
        hpd_regs: *const dcn10_link_enc_hpd_registers,
        link_shift: *const dcn10_link_enc_shift,
        link_mask: *const dcn10_link_enc_mask,
    );

    pub fn dpcs60_program_eq_setting(
        enc: *mut link_encoder,
        FFE_Level: u8,
        de_emphasis_only: bool,
        pre_shoot_only: bool,
        no_ffe: bool,
        link_settings: *const dc_hdmi_frl_link_settings,
    );

    pub fn enc60_hw_init(enc: *mut link_encoder);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
