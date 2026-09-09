/* SPDX-License-Identifier: MIT */
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

// Dependencies are supplied by the surrounding display subsystem.

pub unsafe fn dpp35_dppclk_control(
    dpp_base: *mut dpp,
    dppclk_div: bool,
    enable: bool,
) {
    let dpp: *mut dcn20_dpp = TO_DCN20_DPP(dpp_base);

    if enable {
        if (*dpp).tf_mask.DPPCLK_RATE_CONTROL != 0 {
            REG_UPDATE_2!(dpp, DPP_CONTROL, DPPCLK_RATE_CONTROL, dppclk_div,
                DPP_CLOCK_ENABLE, 1);
        } else if (*dpp).dispclk_r_gate_disable {
            REG_UPDATE_2!(dpp, DPP_CONTROL, DPP_CLOCK_ENABLE, 1,
                DISPCLK_R_GATE_DISABLE, 1);
        } else {
            REG_UPDATE!(dpp, DPP_CONTROL, DPP_CLOCK_ENABLE, 1);
        }
    } else if (*dpp).dispclk_r_gate_disable {
        REG_UPDATE_2!(dpp, DPP_CONTROL, DPP_CLOCK_ENABLE, 0,
            DISPCLK_R_GATE_DISABLE, 0);
    } else {
        REG_UPDATE!(dpp, DPP_CONTROL, DPP_CLOCK_ENABLE, 0);
    }
}

pub unsafe fn dpp35_program_bias_and_scale_fcnv(
    dpp_base: *mut dpp,
    params: *mut dc_bias_and_scale,
) {
    let dpp: *mut dcn20_dpp = TO_DCN20_DPP(dpp_base);

    if !(*params).bias_and_scale_valid {
        REG_SET!(dpp, FCNV_FP_BIAS_R, 0, FCNV_FP_BIAS_R, 0);
        REG_SET!(dpp, FCNV_FP_BIAS_G, 0, FCNV_FP_BIAS_G, 0);
        REG_SET!(dpp, FCNV_FP_BIAS_B, 0, FCNV_FP_BIAS_B, 0);
        REG_SET!(dpp, FCNV_FP_SCALE_R, 0, FCNV_FP_SCALE_R, 0x1F000);
        REG_SET!(dpp, FCNV_FP_SCALE_G, 0, FCNV_FP_SCALE_G, 0x1F000);
        REG_SET!(dpp, FCNV_FP_SCALE_B, 0, FCNV_FP_SCALE_B, 0x1F000);
    } else {
        REG_SET!(dpp, FCNV_FP_BIAS_R, 0, FCNV_FP_BIAS_R, (*params).bias_red);
        REG_SET!(dpp, FCNV_FP_BIAS_G, 0, FCNV_FP_BIAS_G, (*params).bias_green);
        REG_SET!(dpp, FCNV_FP_BIAS_B, 0, FCNV_FP_BIAS_B, (*params).bias_blue);
        REG_SET!(dpp, FCNV_FP_SCALE_R, 0, FCNV_FP_SCALE_R, (*params).scale_red);
        REG_SET!(dpp, FCNV_FP_SCALE_G, 0, FCNV_FP_SCALE_G, (*params).scale_green);
        REG_SET!(dpp, FCNV_FP_SCALE_B, 0, FCNV_FP_SCALE_B, (*params).scale_blue);
    }
}

static mut dcn35_dpp_funcs: dpp_funcs = dpp_funcs {
    dpp_program_gamcor_lut: Some(dpp3_program_gamcor_lut),
    dpp_read_state: Some(dpp30_read_state),
    dpp_read_reg_state: Some(dpp30_read_reg_state),
    dpp_reset: Some(dpp_reset),
    dpp_set_scaler: Some(dpp1_dscl_set_scaler_manual_scale),
    dpp_get_optimal_number_of_taps: Some(dpp3_get_optimal_number_of_taps),
    dpp_set_gamut_remap: Some(dpp3_cm_set_gamut_remap),
    dpp_set_csc_adjustment: None,
    dpp_set_csc_default: None,
    dpp_program_regamma_pwl: None,
    dpp_set_pre_degam: Some(dpp3_set_pre_degam),
    dpp_program_input_lut: None,
    dpp_full_bypass: Some(dpp1_full_bypass),
    dpp_setup: Some(dpp3_cnv_setup),
    dpp_program_degamma_pwl: None,
    dpp_program_cm_dealpha: Some(dpp3_program_cm_dealpha),
    dpp_program_cm_bias: Some(dpp3_program_cm_bias),
    dpp_program_blnd_lut: None, // BLNDGAM is removed completely in DCN3.2 DPP
    dpp_program_shaper_lut: None, // CM SHAPER block is removed in DCN3.2 DPP
    dpp_program_3dlut: None, // CM 3DLUT block is removed in DCN3.2 DPP
    dpp_program_bias_and_scale: Some(dpp35_program_bias_and_scale_fcnv),
    dpp_cnv_set_alpha_keyer: Some(dpp2_cnv_set_alpha_keyer),
    set_cursor_attributes: Some(dpp3_set_cursor_attributes),
    set_cursor_position: Some(dpp1_set_cursor_position),
    set_optional_cursor_attributes: Some(dpp1_cnv_set_optional_cursor_attributes),
    dpp_dppclk_control: Some(dpp35_dppclk_control),
    dpp_set_hdr_multiplier: Some(dpp3_set_hdr_multiplier),
    dpp_get_gamut_remap: Some(dpp3_cm_get_gamut_remap),
};

pub unsafe fn dpp35_construct(
    dpp: *mut dcn3_dpp,
    ctx: *mut dc_context,
    inst: u32,
    tf_regs: *const dcn3_dpp_registers,
    tf_shift: *const dcn35_dpp_shift,
    tf_mask: *const dcn35_dpp_mask,
) -> bool {
    let ret = dpp32_construct(dpp, ctx, inst, tf_regs,
        tf_shift as *const dcn3_dpp_shift, tf_mask as *const dcn3_dpp_mask);
    (*dpp).base.funcs = &raw mut dcn35_dpp_funcs;
    // w/a for cursor memory stuck in LS by programming DISPCLK_R_GATE_DISABLE, limit w/a to some ASIC revs
    if (*(*dpp).base.ctx).asic_id.hw_internal_rev < 0x40 {
        (*dpp).dispclk_r_gate_disable = true;
    }
    ret
}

pub unsafe fn dpp35_set_fgcg(dpp: *mut dcn3_dpp, enable: bool) {
    REG_UPDATE!(dpp, DPP_CONTROL, DPP_FGCG_REP_DIS, !enable);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
