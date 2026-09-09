// SPDX-License-Identifier: MIT
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding translated driver.

unsafe fn dccg314_trigger_dio_fifo_resync(dccg: *mut dccg) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    let mut dispclk_rdivider_value: u32 = 0;
    REG_GET!(dccg_dcn, DENTIST_DISPCLK_CNTL, DENTIST_DISPCLK_RDIVIDER, &mut dispclk_rdivider_value);
    REG_UPDATE!(dccg_dcn, DENTIST_DISPCLK_CNTL, DENTIST_DISPCLK_WDIVIDER, dispclk_rdivider_value);
}

unsafe fn dccg314_get_pixel_rate_div(dccg: *mut dccg, otg_inst: u32, k1: *mut u32, k2: *mut u32) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    let mut val_k1 = PIXEL_RATE_DIV_NA;
    let mut val_k2 = PIXEL_RATE_DIV_NA;
    *k1 = PIXEL_RATE_DIV_NA;
    *k2 = PIXEL_RATE_DIV_NA;
    match otg_inst {
        0 => REG_GET_2!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG0_PIXEL_RATE_DIVK1, &mut val_k1, OTG0_PIXEL_RATE_DIVK2, &mut val_k2),
        1 => REG_GET_2!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG1_PIXEL_RATE_DIVK1, &mut val_k1, OTG1_PIXEL_RATE_DIVK2, &mut val_k2),
        2 => REG_GET_2!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG2_PIXEL_RATE_DIVK1, &mut val_k1, OTG2_PIXEL_RATE_DIVK2, &mut val_k2),
        3 => REG_GET_2!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG3_PIXEL_RATE_DIVK1, &mut val_k1, OTG3_PIXEL_RATE_DIVK2, &mut val_k2),
        _ => { BREAK_TO_DEBUGGER!(); return; }
    }
    *k1 = val_k1;
    *k2 = val_k2;
}

unsafe fn dccg314_set_pixel_rate_div(dccg: *mut dccg, otg_inst: u32, k1: pixel_rate_div, k2: pixel_rate_div) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    let (mut cur_k1, mut cur_k2) = (PIXEL_RATE_DIV_NA, PIXEL_RATE_DIV_NA);
    if k1 == PIXEL_RATE_DIV_NA || k2 == PIXEL_RATE_DIV_NA { BREAK_TO_DEBUGGER!(); return; }
    dccg314_get_pixel_rate_div(dccg, otg_inst, &mut cur_k1, &mut cur_k2);
    if k1 == cur_k1 && k2 == cur_k2 { return; }
    match otg_inst {
        0 => REG_UPDATE_2!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG0_PIXEL_RATE_DIVK1, k1, OTG0_PIXEL_RATE_DIVK2, k2),
        1 => REG_UPDATE_2!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG1_PIXEL_RATE_DIVK1, k1, OTG1_PIXEL_RATE_DIVK2, k2),
        2 => REG_UPDATE_2!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG2_PIXEL_RATE_DIVK1, k1, OTG2_PIXEL_RATE_DIVK2, k2),
        3 => REG_UPDATE_2!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG3_PIXEL_RATE_DIVK1, k1, OTG3_PIXEL_RATE_DIVK2, k2),
        _ => { BREAK_TO_DEBUGGER!(); return; }
    }
}

unsafe fn dccg314_set_dtbclk_p_src(dccg: *mut dccg, src: streamclk_source, otg_inst: u32) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    let p_src_sel = if src == DTBCLK0 { 2 } else { 0 };
    match otg_inst {
        0 => if src == REFCLK { REG_UPDATE!(dccg_dcn, DTBCLK_P_CNTL, DTBCLK_P0_EN, 0) } else { REG_UPDATE_2!(dccg_dcn, DTBCLK_P_CNTL, DTBCLK_P0_SRC_SEL, p_src_sel, DTBCLK_P0_EN, 1) },
        1 => if src == REFCLK { REG_UPDATE!(dccg_dcn, DTBCLK_P_CNTL, DTBCLK_P1_EN, 0) } else { REG_UPDATE_2!(dccg_dcn, DTBCLK_P_CNTL, DTBCLK_P1_SRC_SEL, p_src_sel, DTBCLK_P1_EN, 1) },
        2 => if src == REFCLK { REG_UPDATE!(dccg_dcn, DTBCLK_P_CNTL, DTBCLK_P2_EN, 0) } else { REG_UPDATE_2!(dccg_dcn, DTBCLK_P_CNTL, DTBCLK_P2_SRC_SEL, p_src_sel, DTBCLK_P2_EN, 1) },
        3 => if src == REFCLK { REG_UPDATE!(dccg_dcn, DTBCLK_P_CNTL, DTBCLK_P3_EN, 0) } else { REG_UPDATE_2!(dccg_dcn, DTBCLK_P_CNTL, DTBCLK_P3_SRC_SEL, p_src_sel, DTBCLK_P3_EN, 1) },
        _ => { BREAK_TO_DEBUGGER!(); return; }
    }
}

unsafe fn dccg314_set_dtbclk_dto(dccg: *mut dccg, params: *const dtbclk_dto_params) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    let req_dtbclk_khz = (*params).pixclk_khz / 4;
    if (*params).ref_dtbclk_khz != 0 && req_dtbclk_khz != 0 {
        let modulo = (*params).ref_dtbclk_khz * 1000;
        let phase = req_dtbclk_khz * 1000;
        REG_WRITE!(dccg_dcn, DTBCLK_DTO_MODULO[(*params).otg_inst], modulo);
        REG_WRITE!(dccg_dcn, DTBCLK_DTO_PHASE[(*params).otg_inst], phase);
        REG_UPDATE!(dccg_dcn, OTG_PIXEL_RATE_CNTL[(*params).otg_inst], DTBCLK_DTO_ENABLE[(*params).otg_inst], 1);
        REG_WAIT!(dccg_dcn, OTG_PIXEL_RATE_CNTL[(*params).otg_inst], DTBCLKDTO_ENABLE_STATUS[(*params).otg_inst], 1, 1, 100);
        dccg314_set_pixel_rate_div(dccg, (*params).otg_inst, PIXEL_RATE_DIV_BY_1, PIXEL_RATE_DIV_BY_1);
        REG_UPDATE!(dccg_dcn, OTG_PIXEL_RATE_CNTL[(*params).otg_inst], PIPE_DTO_SRC_SEL[(*params).otg_inst], 2);
    } else {
        REG_UPDATE_2!(dccg_dcn, OTG_PIXEL_RATE_CNTL[(*params).otg_inst], DTBCLK_DTO_ENABLE[(*params).otg_inst], 0, PIPE_DTO_SRC_SEL[(*params).otg_inst], 1);
        REG_WRITE!(dccg_dcn, DTBCLK_DTO_MODULO[(*params).otg_inst], 0);
        REG_WRITE!(dccg_dcn, DTBCLK_DTO_PHASE[(*params).otg_inst], 0);
    }
}

pub unsafe fn dccg314_set_dpstreamclk(dccg: *mut dccg, src: streamclk_source, otg_inst: i32, dp_hpo_inst: i32) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    dccg314_set_dtbclk_p_src(dccg, src, otg_inst as u32);
    match dp_hpo_inst {
        0 => REG_UPDATE_2!(dccg_dcn, DPSTREAMCLK_CNTL, DPSTREAMCLK0_EN, if src == REFCLK { 0 } else { 1 }, DPSTREAMCLK0_SRC_SEL, otg_inst),
        1 => REG_UPDATE_2!(dccg_dcn, DPSTREAMCLK_CNTL, DPSTREAMCLK1_EN, if src == REFCLK { 0 } else { 1 }, DPSTREAMCLK1_SRC_SEL, otg_inst),
        2 => REG_UPDATE_2!(dccg_dcn, DPSTREAMCLK_CNTL, DPSTREAMCLK2_EN, if src == REFCLK { 0 } else { 1 }, DPSTREAMCLK2_SRC_SEL, otg_inst),
        3 => REG_UPDATE_2!(dccg_dcn, DPSTREAMCLK_CNTL, DPSTREAMCLK3_EN, if src == REFCLK { 0 } else { 1 }, DPSTREAMCLK3_SRC_SEL, otg_inst),
        _ => { BREAK_TO_DEBUGGER!(); return; }
    }
}

unsafe fn dccg314_set_hdmistreamclk(dccg: *mut dccg, src: streamclk_source, otg_inst: u32) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    dccg314_set_dtbclk_p_src(dccg, src, otg_inst);
    if src == REFCLK { REG_UPDATE_2!(dccg_dcn, HDMISTREAMCLK_CNTL, HDMISTREAMCLK0_EN, 0, HDMISTREAMCLK0_DTO_FORCE_DIS, 1); }
    else { REG_UPDATE_3!(dccg_dcn, HDMISTREAMCLK_CNTL, HDMISTREAMCLK0_EN, 1, HDMISTREAMCLK0_SRC_SEL, otg_inst, HDMISTREAMCLK0_DTO_FORCE_DIS, 1); }
}

unsafe fn dccg314_init(dccg: *mut dccg) {
    for otg_inst in 0..4 { dccg31_disable_symclk32_se(dccg, otg_inst); }
    if (*(*dccg).ctx).dc.debug.root_clock_optimization.bits.symclk32_le { for otg_inst in 0..2 { dccg31_disable_symclk32_le(dccg, otg_inst); } }
    if (*(*dccg).ctx).dc.debug.root_clock_optimization.bits.dpstream { for otg_inst in 0..4 { dccg314_set_dpstreamclk(dccg, REFCLK, otg_inst, otg_inst); } }
    if (*(*dccg).ctx).dc.debug.root_clock_optimization.bits.physymclk { for otg_inst in 0..5 { dccg31_set_physymclk(dccg, otg_inst, PHYSYMCLK_FORCE_SRC_SYMCLK, false); } }
}

unsafe fn dccg314_set_valid_pixel_rate(dccg: *mut dccg, ref_dtbclk_khz: i32, otg_inst: i32, pixclk_khz: i32) {
    let params = dtbclk_dto_params { ref_dtbclk_khz, otg_inst, pixclk_khz };
    dccg314_set_dtbclk_dto(dccg, &params);
}

unsafe fn dccg314_dpp_root_clock_control(dccg: *mut dccg, dpp_inst: u32, clock_on: bool) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    if (*dccg).dpp_clock_gated[dpp_inst as usize] != clock_on { return; }
    if clock_on {
        REG_UPDATE!(dccg_dcn, DPPCLK_DTO_CTRL, DPPCLK_DTO_ENABLE[dpp_inst], 0);
        REG_SET_2!(dccg_dcn, DPPCLK_DTO_PARAM[dpp_inst], 0, DPPCLK0_DTO_PHASE, 0xFF, DPPCLK0_DTO_MODULO, 0xFF);
    } else {
        REG_UPDATE!(dccg_dcn, DPPCLK_DTO_CTRL, DPPCLK_DTO_ENABLE[dpp_inst], 1);
        REG_SET_2!(dccg_dcn, DPPCLK_DTO_PARAM[dpp_inst], 0, DPPCLK0_DTO_PHASE, 0, DPPCLK0_DTO_MODULO, 1);
    }
    (*dccg).dpp_clock_gated[dpp_inst as usize] = !clock_on;
}

static dccg314_funcs: dccg_funcs = dccg_funcs {
    enable_hdmicharclk: dccg31_enable_hdmicharclk, disable_hdmicharclk: dccg31_disable_hdmicharclk,
    set_hdmistreamclk: dccg314_set_hdmistreamclk, update_dpp_dto: dccg31_update_dpp_dto,
    dpp_root_clock_control: dccg314_dpp_root_clock_control, get_dccg_ref_freq: dccg31_get_dccg_ref_freq,
    dccg_init: dccg314_init, set_dpstreamclk: dccg314_set_dpstreamclk,
    enable_symclk32_se: dccg31_enable_symclk32_se, disable_symclk32_se: dccg31_disable_symclk32_se,
    enable_symclk32_le: dccg31_enable_symclk32_le, disable_symclk32_le: dccg31_disable_symclk32_le,
    set_symclk32_le_root_clock_gating: dccg31_set_symclk32_le_root_clock_gating, set_physymclk: dccg31_set_physymclk,
    set_dtbclk_dto: dccg314_set_dtbclk_dto, set_audio_dtbclk_dto: dccg31_set_audio_dtbclk_dto,
    set_fifo_errdet_ovr_en: dccg2_set_fifo_errdet_ovr_en, otg_add_pixel: dccg31_otg_add_pixel,
    otg_drop_pixel: dccg31_otg_drop_pixel, set_dispclk_change_mode: dccg31_set_dispclk_change_mode,
    disable_dsc: dccg31_disable_dscclk, enable_dsc: dccg31_enable_dscclk,
    set_pixel_rate_div: dccg314_set_pixel_rate_div, get_pixel_rate_div: dccg314_get_pixel_rate_div,
    trigger_dio_fifo_resync: dccg314_trigger_dio_fifo_resync, set_valid_pixel_rate: dccg314_set_valid_pixel_rate,
    set_dtbclk_p_src: dccg314_set_dtbclk_p_src, dccg_read_reg_state: dccg31_read_reg_state,
    refclk_setup: dccg2_refclk_setup, allow_clock_gating: dccg2_allow_clock_gating,
    enable_memory_low_power: dccg2_enable_memory_low_power, is_s0i3_golden_init_wa_done: dccg2_is_s0i3_golden_init_wa_done,
};

pub unsafe fn dccg314_create(ctx: *mut dc_context, regs: *const dccg_registers, dccg_shift: *const dccg_shift, dccg_mask: *const dccg_mask) -> *mut dccg {
    let dccg_dcn = kzalloc_obj!(dcn_dccg);
    if dccg_dcn.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }
    (*dccg_dcn).base.ctx = ctx;
    (*dccg_dcn).base.funcs = &dccg314_funcs;
    (*dccg_dcn).regs = regs;
    (*dccg_dcn).dccg_shift = dccg_shift;
    (*dccg_dcn).dccg_mask = dccg_mask;
    &mut (*dccg_dcn).base
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
