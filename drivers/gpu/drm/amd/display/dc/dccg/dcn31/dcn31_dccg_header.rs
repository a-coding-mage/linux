/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// Dependency supplied by dcn30/dcn30_dccg.h.

macro_rules! DCCG_REG_LIST_DCN31 {
    () => {
        SR!(DPPCLK_DTO_CTRL),
        DCCG_SRII!(DTO_PARAM, DPPCLK, 0), DCCG_SRII!(DTO_PARAM, DPPCLK, 1),
        DCCG_SRII!(DTO_PARAM, DPPCLK, 2), DCCG_SRII!(DTO_PARAM, DPPCLK, 3),
        DCCG_SRII!(CLOCK_CNTL, HDMICHARCLK, 0), SR!(PHYASYMCLK_CLOCK_CNTL),
        SR!(PHYBSYMCLK_CLOCK_CNTL), SR!(PHYCSYMCLK_CLOCK_CNTL),
        SR!(PHYDSYMCLK_CLOCK_CNTL), SR!(PHYESYMCLK_CLOCK_CNTL),
        SR!(DPSTREAMCLK_CNTL), SR!(HDMISTREAMCLK_CNTL), SR!(SYMCLK32_SE_CNTL),
        SR!(SYMCLK32_LE_CNTL), DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 0),
        DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 1), DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 2),
        DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 3), DCCG_SRII!(MODULO, DTBCLK_DTO, 0),
        DCCG_SRII!(MODULO, DTBCLK_DTO, 1), DCCG_SRII!(MODULO, DTBCLK_DTO, 2),
        DCCG_SRII!(MODULO, DTBCLK_DTO, 3), DCCG_SRII!(PHASE, DTBCLK_DTO, 0),
        DCCG_SRII!(PHASE, DTBCLK_DTO, 1), DCCG_SRII!(PHASE, DTBCLK_DTO, 2),
        DCCG_SRII!(PHASE, DTBCLK_DTO, 3), SR!(DCCG_AUDIO_DTBCLK_DTO_MODULO),
        SR!(DCCG_AUDIO_DTBCLK_DTO_PHASE), SR!(DCCG_AUDIO_DTO_SOURCE),
        SR!(DENTIST_DISPCLK_CNTL), SR!(DSCCLK0_DTO_PARAM), SR!(DSCCLK1_DTO_PARAM),
        SR!(DSCCLK2_DTO_PARAM), SR!(DSCCLK_DTO_CTRL), SR!(DCCG_GATE_DISABLE_CNTL),
        SR!(DCCG_GATE_DISABLE_CNTL2), SR!(DCCG_GATE_DISABLE_CNTL3),
        SR!(HDMISTREAMCLK0_DTO_PARAM), SR!(DC_MEM_GLOBAL_PWR_REQ_CNTL),
        SR!(MICROSECOND_TIME_BASE_DIV)
    };
}

macro_rules! DCCG_MASK_SH_LIST_DCN31 {
    ($mask_sh:expr) => {
        DCCG_SFI!(DPPCLK_DTO_CTRL, DTO_ENABLE, DPPCLK, 0, $mask_sh),
        DCCG_SFI!(DPPCLK_DTO_CTRL, DTO_DB_EN, DPPCLK, 0, $mask_sh),
        DCCG_SFI!(DPPCLK_DTO_CTRL, DTO_ENABLE, DPPCLK, 1, $mask_sh),
        DCCG_SFI!(DPPCLK_DTO_CTRL, DTO_DB_EN, DPPCLK, 1, $mask_sh),
        DCCG_SFI!(DPPCLK_DTO_CTRL, DTO_ENABLE, DPPCLK, 2, $mask_sh),
        DCCG_SFI!(DPPCLK_DTO_CTRL, DTO_DB_EN, DPPCLK, 2, $mask_sh),
        DCCG_SFI!(DPPCLK_DTO_CTRL, DTO_ENABLE, DPPCLK, 3, $mask_sh),
        DCCG_SFI!(DPPCLK_DTO_CTRL, DTO_DB_EN, DPPCLK, 3, $mask_sh),
        DCCG_SF!(DPPCLK0_DTO_PARAM, DPPCLK0_DTO_PHASE, $mask_sh),
        DCCG_SF!(DPPCLK0_DTO_PARAM, DPPCLK0_DTO_MODULO, $mask_sh),
        DCCG_SF!(HDMICHARCLK0_CLOCK_CNTL, HDMICHARCLK0_EN, $mask_sh),
        DCCG_SF!(HDMICHARCLK0_CLOCK_CNTL, HDMICHARCLK0_SRC_SEL, $mask_sh),
        DCCG_SF!(PHYASYMCLK_CLOCK_CNTL, PHYASYMCLK_FORCE_EN, $mask_sh),
        DCCG_SF!(PHYASYMCLK_CLOCK_CNTL, PHYASYMCLK_FORCE_SRC_SEL, $mask_sh),
        DCCG_SF!(PHYBSYMCLK_CLOCK_CNTL, PHYBSYMCLK_FORCE_EN, $mask_sh),
        DCCG_SF!(PHYBSYMCLK_CLOCK_CNTL, PHYBSYMCLK_FORCE_SRC_SEL, $mask_sh),
        DCCG_SF!(PHYCSYMCLK_CLOCK_CNTL, PHYCSYMCLK_FORCE_EN, $mask_sh),
        DCCG_SF!(PHYCSYMCLK_CLOCK_CNTL, PHYCSYMCLK_FORCE_SRC_SEL, $mask_sh),
        DCCG_SF!(PHYDSYMCLK_CLOCK_CNTL, PHYDSYMCLK_FORCE_EN, $mask_sh),
        DCCG_SF!(PHYDSYMCLK_CLOCK_CNTL, PHYDSYMCLK_FORCE_SRC_SEL, $mask_sh),
        DCCG_SF!(PHYESYMCLK_CLOCK_CNTL, PHYESYMCLK_FORCE_EN, $mask_sh),
        DCCG_SF!(PHYESYMCLK_CLOCK_CNTL, PHYESYMCLK_FORCE_SRC_SEL, $mask_sh),
        DCCG_SF!(DPSTREAMCLK_CNTL, DPSTREAMCLK_PIPE0_EN, $mask_sh),
        DCCG_SF!(DPSTREAMCLK_CNTL, DPSTREAMCLK_PIPE1_EN, $mask_sh),
        DCCG_SF!(DPSTREAMCLK_CNTL, DPSTREAMCLK_PIPE2_EN, $mask_sh),
        DCCG_SF!(DPSTREAMCLK_CNTL, DPSTREAMCLK_PIPE3_EN, $mask_sh),
        DCCG_SF!(HDMISTREAMCLK_CNTL, HDMISTREAMCLK0_SRC_SEL, $mask_sh),
        DCCG_SF!(HDMISTREAMCLK_CNTL, HDMISTREAMCLK0_DTO_FORCE_DIS, $mask_sh),
        DCCG_SF!(SYMCLK32_SE_CNTL, SYMCLK32_SE0_SRC_SEL, $mask_sh),
        DCCG_SF!(SYMCLK32_SE_CNTL, SYMCLK32_SE1_SRC_SEL, $mask_sh),
        DCCG_SF!(SYMCLK32_SE_CNTL, SYMCLK32_SE2_SRC_SEL, $mask_sh),
        DCCG_SF!(SYMCLK32_SE_CNTL, SYMCLK32_SE3_SRC_SEL, $mask_sh),
        DCCG_SF!(SYMCLK32_SE_CNTL, SYMCLK32_SE0_EN, $mask_sh),
        DCCG_SF!(SYMCLK32_SE_CNTL, SYMCLK32_SE1_EN, $mask_sh),
        DCCG_SF!(SYMCLK32_SE_CNTL, SYMCLK32_SE2_EN, $mask_sh),
        DCCG_SF!(SYMCLK32_SE_CNTL, SYMCLK32_SE3_EN, $mask_sh),
        DCCG_SF!(SYMCLK32_LE_CNTL, SYMCLK32_LE0_SRC_SEL, $mask_sh),
        DCCG_SF!(SYMCLK32_LE_CNTL, SYMCLK32_LE1_SRC_SEL, $mask_sh),
        DCCG_SF!(SYMCLK32_LE_CNTL, SYMCLK32_LE0_EN, $mask_sh),
        DCCG_SF!(SYMCLK32_LE_CNTL, SYMCLK32_LE1_EN, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, DTBCLK_DTO, ENABLE, 0, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, DTBCLK_DTO, ENABLE, 1, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, DTBCLK_DTO, ENABLE, 2, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, DTBCLK_DTO, ENABLE, 3, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, DTBCLKDTO, ENABLE_STATUS, 0, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, DTBCLKDTO, ENABLE_STATUS, 1, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, DTBCLKDTO, ENABLE_STATUS, 2, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, DTBCLKDTO, ENABLE_STATUS, 3, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, PIPE, DTO_SRC_SEL, 0, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, PIPE, DTO_SRC_SEL, 1, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, PIPE, DTO_SRC_SEL, 2, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, PIPE, DTO_SRC_SEL, 3, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, DTBCLK_DTO, DIV, 0, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, DTBCLK_DTO, DIV, 1, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, DTBCLK_DTO, DIV, 2, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, DTBCLK_DTO, DIV, 3, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, OTG, ADD_PIXEL, 0, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, OTG, ADD_PIXEL, 1, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, OTG, ADD_PIXEL, 2, $mask_sh),
        DCCG_SFII!(OTG, PIXEL_RATE_CNTL, OTG, ADD_PIXEL, 3, $mask_sh),
        DCCG_SF!(DCCG_AUDIO_DTO_SOURCE, DCCG_AUDIO_DTO_SEL, $mask_sh),
        DCCG_SF!(DCCG_AUDIO_DTO_SOURCE, DCCG_AUDIO_DTO0_SOURCE_SEL, $mask_sh),
        DCCG_SF!(DENTIST_DISPCLK_CNTL, DENTIST_DISPCLK_CHG_MODE, $mask_sh),
        DCCG_SF!(DSCCLK0_DTO_PARAM, DSCCLK0_DTO_PHASE, $mask_sh),
        DCCG_SF!(DSCCLK0_DTO_PARAM, DSCCLK0_DTO_MODULO, $mask_sh),
        DCCG_SF!(DSCCLK1_DTO_PARAM, DSCCLK1_DTO_PHASE, $mask_sh),
        DCCG_SF!(DSCCLK1_DTO_PARAM, DSCCLK1_DTO_MODULO, $mask_sh),
        DCCG_SF!(DSCCLK2_DTO_PARAM, DSCCLK2_DTO_PHASE, $mask_sh),
        DCCG_SF!(DSCCLK2_DTO_PARAM, DSCCLK2_DTO_MODULO, $mask_sh),
        DCCG_SF!(DSCCLK_DTO_CTRL, DSCCLK0_DTO_ENABLE, $mask_sh),
        DCCG_SF!(DSCCLK_DTO_CTRL, DSCCLK1_DTO_ENABLE, $mask_sh),
        DCCG_SF!(DSCCLK_DTO_CTRL, DSCCLK2_DTO_ENABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL2, PHYASYMCLK_GATE_DISABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL2, PHYBSYMCLK_GATE_DISABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL2, PHYCSYMCLK_GATE_DISABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL2, PHYDSYMCLK_GATE_DISABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL2, PHYESYMCLK_GATE_DISABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL3, DPSTREAMCLK_ROOT_GATE_DISABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL3, DPSTREAMCLK_GATE_DISABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL3, SYMCLK32_ROOT_SE0_GATE_DISABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL3, SYMCLK32_ROOT_SE1_GATE_DISABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL3, SYMCLK32_ROOT_SE2_GATE_DISABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL3, SYMCLK32_ROOT_SE3_GATE_DISABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL3, SYMCLK32_ROOT_LE0_GATE_DISABLE, $mask_sh),
        DCCG_SF!(DCCG_GATE_DISABLE_CNTL3, SYMCLK32_ROOT_LE1_GATE_DISABLE, $mask_sh),
        DCCG_SF!(HDMISTREAMCLK0_DTO_PARAM, HDMISTREAMCLK0_DTO_PHASE, $mask_sh),
        DCCG_SF!(HDMISTREAMCLK0_DTO_PARAM, HDMISTREAMCLK0_DTO_MODULO, $mask_sh)
    };
}

extern "C" {
    pub fn dccg31_create(ctx: *mut dc_context, regs: *const dccg_registers,
        dccg_shift: *const dccg_shift, dccg_mask: *const dccg_mask) -> *mut dccg;
    pub fn dccg31_init(dccg: *mut dccg);
    pub fn dccg31_enable_symclk32_se(dccg: *mut dccg, hpo_se_inst: ::core::ffi::c_int, phyd32clk: phyd32clk_clock_source);
    pub fn dccg31_disable_symclk32_se(dccg: *mut dccg, hpo_se_inst: ::core::ffi::c_int);
    pub fn dccg31_enable_symclk32_le(dccg: *mut dccg, hpo_le_inst: ::core::ffi::c_int, phyd32clk: phyd32clk_clock_source);
    pub fn dccg31_disable_symclk32_le(dccg: *mut dccg, hpo_le_inst: ::core::ffi::c_int);
    pub fn dccg31_set_symclk32_le_root_clock_gating(dccg: *mut dccg, hpo_le_inst: ::core::ffi::c_int, enable: bool);
    pub fn dccg31_set_physymclk(dccg: *mut dccg, phy_inst: ::core::ffi::c_int, clk_src: physymclk_clock_source, force_enable: bool);
    pub fn dccg31_set_audio_dtbclk_dto(dccg: *mut dccg, params: *const dtbclk_dto_params);
    pub fn dccg31_enable_hdmicharclk(dccg: *mut dccg, hpo_inst: ::core::ffi::c_int, phypll_inst: ::core::ffi::c_int);
    pub fn dccg31_disable_hdmicharclk(dccg: *mut dccg, hpo_inst: ::core::ffi::c_int);
    pub fn dccg31_set_hdmistreamclk(dccg: *mut dccg, src: streamclk_source, otg_inst: u32);
    pub fn dccg31_update_dpp_dto(dccg: *mut dccg, dpp_inst: ::core::ffi::c_int, req_dppclk: ::core::ffi::c_int);
    pub fn dccg31_get_dccg_ref_freq(dccg: *mut dccg, xtalin_freq_inKhz: u32, dccg_ref_freq_inKhz: *mut u32);
    pub fn dccg31_set_dpstreamclk(dccg: *mut dccg, src: streamclk_source, otg_inst: ::core::ffi::c_int, dp_hpo_inst: ::core::ffi::c_int);
    pub fn dccg31_set_dtbclk_dto(dccg: *mut dccg, params: *const dtbclk_dto_params);
    pub fn dccg31_otg_add_pixel(dccg: *mut dccg, otg_inst: u32);
    pub fn dccg31_otg_drop_pixel(dccg: *mut dccg, otg_inst: u32);
    pub fn dccg31_set_dispclk_change_mode(dccg: *mut dccg, change_mode: dentist_dispclk_change_mode);
    pub fn dccg31_disable_dscclk(dccg: *mut dccg, inst: ::core::ffi::c_int);
    pub fn dccg31_enable_dscclk(dccg: *mut dccg, inst: ::core::ffi::c_int);
    pub fn dccg31_read_reg_state(dccg: *mut dccg, dccg_reg_state: *mut dcn_dccg_reg_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
