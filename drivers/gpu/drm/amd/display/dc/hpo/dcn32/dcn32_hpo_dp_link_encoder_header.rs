/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// Dependency supplied by the translated link_encoder interface.

// The C preprocessor macro expands to the corresponding register-field
// descriptor list. SE_SF and the register symbols are supplied externally.
#[macro_export]
macro_rules! DCN3_2_HPO_DP_LINK_ENC_MASK_SH_LIST {
    ($mask_sh:expr) => {
        SE_SF!(DP_LINK_ENC0_DP_LINK_ENC_CLOCK_CONTROL, DP_LINK_ENC_CLOCK_EN, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_CONTROL, DPHY_RESET, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_CONTROL, DPHY_ENABLE, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_CONTROL, PRECODER_ENABLE, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_CONTROL, MODE, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_CONTROL, NUM_LANES, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_STATUS, STATUS, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_STATUS, SAT_UPDATE_PENDING, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_STATUS, RATE_UPDATE_PENDING, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_TP_CUSTOM0, TP_CUSTOM, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_TP_CONFIG, TP_SELECT0, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_TP_CONFIG, TP_SELECT1, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_TP_CONFIG, TP_SELECT2, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_TP_CONFIG, TP_SELECT3, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_TP_CONFIG, TP_PRBS_SEL0, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_TP_CONFIG, TP_PRBS_SEL1, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_TP_CONFIG, TP_PRBS_SEL2, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_TP_CONFIG, TP_PRBS_SEL3, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_TP_SQ_PULSE, TP_SQ_PULSE_WIDTH, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_SAT_VC0, SAT_STREAM_SOURCE, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_SAT_VC0, SAT_SLOT_COUNT, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_VC_RATE_CNTL0, STREAM_VC_RATE_X, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_VC_RATE_CNTL0, STREAM_VC_RATE_Y, $mask_sh),
        SE_SF!(DP_DPHY_SYM320_DP_DPHY_SYM32_SAT_UPDATE, SAT_UPDATE, $mask_sh)
    };
}

extern "C" {
    pub fn dcn32_hpo_dp_link_enc_is_in_alt_mode(enc: *mut hpo_dp_link_encoder) -> bool;
    pub fn hpo_dp_link_encoder32_construct(
        enc31: *mut dcn31_hpo_dp_link_encoder,
        ctx: *mut dc_context,
        inst: u32,
        hpo_le_regs: *const dcn31_hpo_dp_link_encoder_registers,
        hpo_le_shift: *const dcn31_hpo_dp_link_encoder_shift,
        hpo_le_mask: *const dcn31_hpo_dp_link_encoder_mask,
    );

    // Duplicate declaration preserved from the source header.
    pub fn dcn32_hpo_dp_link_enc_is_in_alt_mode(
        enc: *mut hpo_dp_link_encoder,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
