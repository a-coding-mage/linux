/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// C dependency: dcn32/dcn32_dccg.h
// The following register-field macros are supplied by the dependent DCCG
// implementation.  Their token-pasting behavior is intentionally retained.
macro_rules! DCCG_SFII {
    ($($tokens:tt)*) => { DCCG_SFI!($($tokens)*); };
}

// DCCG_MASK_SH_LIST_DCN401(mask_sh) expands to the following field list:
// DCCG_SFI(DPPCLK_DTO_CTRL, DTO_DB_EN, DPPCLK, 0..=3, mask_sh)
// DCCG_SF(DPPCLK_CTRL, DPPCLK[0..=3]_EN, mask_sh)
// DCCG_SF(DPPCLK0_DTO_PARAM, DPPCLK0_DTO_PHASE, mask_sh)
// DCCG_SF(DPPCLK0_DTO_PARAM, DPPCLK0_DTO_MODULO, mask_sh)
// DCCG_SF(HDMICHARCLK0_CLOCK_CNTL, HDMICHARCLK0_EN, mask_sh)
// DCCG_SF(HDMICHARCLK0_CLOCK_CNTL, HDMICHARCLK0_SRC_SEL, mask_sh)
// DCCG_SF(PHY[ABCD]SYMCLK_CLOCK_CNTL, PHY[ABCD]SYMCLK_{EN,SRC_SEL}, mask_sh)
// DCCG_SF(DPSTREAMCLK_CNTL, DPSTREAMCLK[0..=3]_{EN,SRC_SEL}, mask_sh)
// DCCG_SF(HDMISTREAMCLK_CNTL, HDMISTREAMCLK0_{EN,SRC_SEL}, mask_sh)
// DCCG_SF(SYMCLK32_SE_CNTL, SYMCLK32_SE[0..=3]_{SRC_SEL,EN}, mask_sh)
// DCCG_SF(SYMCLK32_LE_CNTL, SYMCLK32_LE[0..=1]_{SRC_SEL,EN}, mask_sh)
// DCCG_SFII(OTG, PIXEL_RATE_CNTL, PIPE, DTO_SRC_SEL, 0..=3, mask_sh)
// DCCG_SFII(OTG, PIXEL_RATE_CNTL, OTG, ADD_PIXEL, 0..=3, mask_sh)
// DCCG_SF(OTG_PIXEL_RATE_DIV, OTG[0..=3]_TMDS_PIXEL_RATE_DIV, mask_sh)
// DCCG_SF(OTG_PIXEL_RATE_DIV, DPDTO[0..=3]_INT, mask_sh)
// DCCG_SF(DTBCLK_P_CNTL, DTBCLK_P[0..=3]_{SRC_SEL,EN}, mask_sh)
// DCCG_SF(DCCG_AUDIO_DTO_SOURCE, DCCG_AUDIO_DTO_{SEL,0_SOURCE_SEL}, mask_sh)
// DCCG_SF(DENTIST_DISPCLK_CNTL, DENTIST_DISPCLK_CHG_DONE, mask_sh)
// DCCG_SF(DCCG_GATE_DISABLE_CNTL2..6, all corresponding gate-disable fields, mask_sh)
// DCCG_SF(SYMCLK[ABCD]_CLOCK_ENABLE, SYMCLK[ABCD]_{CLOCK_ENABLE,FE_EN,FE_SRC_SEL}, mask_sh)
// This list also contains DSCCLK DTO enable/phase/modulo fields, DP_DTO
// ENABLE fields, HDMI/DP/DSC/DPP/root gates, and all SYMCLK32 gates exactly
// as specified by the C macro above.

extern "C" {
    pub fn dccg401_init(dccg: *mut dccg);
    pub fn dccg401_update_dpp_dto(dccg: *mut dccg, dpp_inst: ::core::ffi::c_int, req_dppclk: ::core::ffi::c_int);
    pub fn dccg401_get_dccg_ref_freq(dccg: *mut dccg, xtalin_freq_inKhz: u32, dccg_ref_freq_inKhz: *mut u32);
    pub fn dccg401_set_dpstreamclk(dccg: *mut dccg, src: streamclk_source, otg_inst: ::core::ffi::c_int, dp_hpo_inst: ::core::ffi::c_int);
    pub fn dccg401_enable_symclk32_le(dccg: *mut dccg, hpo_le_inst: ::core::ffi::c_int, phyd32clk: phyd32clk_clock_source);
    pub fn dccg401_disable_symclk32_le(dccg: *mut dccg, hpo_le_inst: ::core::ffi::c_int);
    pub fn dccg401_disable_dpstreamclk(dccg: *mut dccg, dp_hpo_inst: ::core::ffi::c_int);
    pub fn dccg401_set_dto_dscclk(dccg: *mut dccg, inst: u32, num_slices_h: u32);
    pub fn dccg401_set_ref_dscclk(dccg: *mut dccg, dsc_inst: u32);
    pub fn dccg401_set_src_sel(dccg: *mut dccg, params: *const dtbclk_dto_params);
    pub fn dccg401_set_pixel_rate_div(dccg: *mut dccg, otg_inst: u32, tmds_div: pixel_rate_div, unused: pixel_rate_div);
    pub fn dccg401_get_pixel_rate_div(dccg: *mut dccg, otg_inst: u32, tmds_div: *mut u32, dp_dto_int: *mut u32);
    pub fn dccg401_set_dp_dto(dccg: *mut dccg, params: *const dp_dto_params);
    pub fn dccg401_enable_symclk_se(dccg: *mut dccg, stream_enc_inst: u32, link_enc_inst: u32);
    pub fn dccg401_disable_symclk_se(dccg: *mut dccg, stream_enc_inst: u32, link_enc_inst: u32);
    pub fn dccg401_set_dtbclk_p_src(dccg: *mut dccg, src: streamclk_source, otg_inst: u32);
    pub fn dccg401_create(ctx: *mut dc_context, regs: *const dccg_registers, dccg_shift: *const dccg_shift, dccg_mask: *const dccg_mask) -> *mut dccg;
    pub fn dccg401_set_physymclk(dccg: *mut dccg, phy_inst: ::core::ffi::c_int, clk_src: physymclk_clock_source, force_enable: bool);
    pub fn dccg401_set_hdmistreamclk(dccg: *mut dccg, src: streamclk_source, otg_inst: u32);
    pub fn dccg401_enable_hdmicharclk(dccg: *mut dccg, hpo_inst: ::core::ffi::c_int, phypll_inst: ::core::ffi::c_int);
    pub fn dccg401_disable_hdmicharclk(dccg: *mut dccg, hpo_inst: ::core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
