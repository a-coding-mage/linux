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

// Dependencies are supplied by the surrounding translated driver.

unsafe fn dccg32_trigger_dio_fifo_resync(dccg: *mut dccg) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    let mut dispclk_rdivider_value: u32 = 0;

    REG_GET!(dccg_dcn, DENTIST_DISPCLK_CNTL, DENTIST_DISPCLK_RDIVIDER, &mut dispclk_rdivider_value);

    /* Not valid for the WDIVIDER to be set to 0 */
    if dispclk_rdivider_value != 0 {
        REG_UPDATE!(dccg_dcn, DENTIST_DISPCLK_CNTL, DENTIST_DISPCLK_WDIVIDER, dispclk_rdivider_value);
    }
}

unsafe fn dccg32_get_pixel_rate_div(
    dccg: *mut dccg,
    otg_inst: u32,
    k1: *mut u32,
    k2: *mut u32,
) {
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

unsafe fn dccg32_set_pixel_rate_div(dccg: *mut dccg, otg_inst: u32, k1: pixel_rate_div, k2: pixel_rate_div) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    let mut cur_k1 = PIXEL_RATE_DIV_NA;
    let mut cur_k2 = PIXEL_RATE_DIV_NA;

    // Don't program 0xF into the register field. Not valid since
    // K1 / K2 field is only 1 / 2 bits wide
    if k1 == PIXEL_RATE_DIV_NA || k2 == PIXEL_RATE_DIV_NA { BREAK_TO_DEBUGGER!(); return; }
    dccg32_get_pixel_rate_div(dccg, otg_inst, &mut cur_k1, &mut cur_k2);
    if k1 == cur_k1 && k2 == cur_k2 { return; }

    match otg_inst {
        0 => REG_UPDATE_2!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG0_PIXEL_RATE_DIVK1, k1, OTG0_PIXEL_RATE_DIVK2, k2),
        1 => REG_UPDATE_2!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG1_PIXEL_RATE_DIVK1, k1, OTG1_PIXEL_RATE_DIVK2, k2),
        2 => REG_UPDATE_2!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG2_PIXEL_RATE_DIVK1, k1, OTG2_PIXEL_RATE_DIVK2, k2),
        3 => REG_UPDATE_2!(dccg_dcn, OTG_PIXEL_RATE_DIV, OTG3_PIXEL_RATE_DIVK1, k1, OTG3_PIXEL_RATE_DIVK2, k2),
        _ => { BREAK_TO_DEBUGGER!(); return; }
    }
}

unsafe fn dccg32_set_dtbclk_p_src(dccg: *mut dccg, src: streamclk_source, otg_inst: u32) {
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

/* Controls the generation of pixel valid for OTG in (OTG -> HPO case) */
unsafe fn dccg32_set_dtbclk_dto(dccg: *mut dccg, params: *const dtbclk_dto_params) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    let req_dtbclk_khz = (*params).pixclk_khz / 4;
    if (*params).ref_dtbclk_khz != 0 && req_dtbclk_khz != 0 {
        // phase / modulo = dtbclk / dtbclk ref
        let modulo = (*params).ref_dtbclk_khz * 1000;
        let phase = req_dtbclk_khz * 1000;
        REG_WRITE!(dccg_dcn, DTBCLK_DTO_MODULO[(*params).otg_inst], modulo);
        REG_WRITE!(dccg_dcn, DTBCLK_DTO_PHASE[(*params).otg_inst], phase);
        REG_UPDATE!(dccg_dcn, OTG_PIXEL_RATE_CNTL[(*params).otg_inst], DTBCLK_DTO_ENABLE[(*params).otg_inst], 1);
        REG_WAIT!(dccg_dcn, OTG_PIXEL_RATE_CNTL[(*params).otg_inst], DTBCLKDTO_ENABLE_STATUS[(*params).otg_inst], 1, 1, 100);
        dccg32_set_pixel_rate_div(dccg, (*params).otg_inst, PIXEL_RATE_DIV_BY_1, PIXEL_RATE_DIV_BY_1);
        REG_UPDATE!(dccg_dcn, OTG_PIXEL_RATE_CNTL[(*params).otg_inst], PIPE_DTO_SRC_SEL[(*params).otg_inst], 2);
    } else {
        REG_UPDATE_2!(dccg_dcn, OTG_PIXEL_RATE_CNTL[(*params).otg_inst], DTBCLK_DTO_ENABLE[(*params).otg_inst], 0, PIPE_DTO_SRC_SEL[(*params).otg_inst], if (*params).is_hdmi { 0 } else { 1 });
        REG_WRITE!(dccg_dcn, DTBCLK_DTO_MODULO[(*params).otg_inst], 0);
        REG_WRITE!(dccg_dcn, DTBCLK_DTO_PHASE[(*params).otg_inst], 0);
    }
}

unsafe fn dccg32_set_valid_pixel_rate(dccg: *mut dccg, ref_dtbclk_khz: i32, otg_inst: i32, pixclk_khz: i32) {
    let dto_params = dtbclk_dto_params { ref_dtbclk_khz, otg_inst, pixclk_khz, is_hdmi: true };
    dccg32_set_dtbclk_dto(dccg, &dto_params);
}

unsafe fn dccg32_get_dccg_ref_freq(_dccg: *mut dccg, xtalin_freq_in_khz: u32, dccg_ref_freq_in_khz: *mut u32) {
    /* Assume refclk is sourced from xtalin; expect 100MHz */
    *dccg_ref_freq_in_khz = xtalin_freq_in_khz;
}

unsafe fn dccg32_set_dpstreamclk(dccg: *mut dccg, src: streamclk_source, otg_inst: i32, dp_hpo_inst: i32) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    dccg32_set_dtbclk_p_src(dccg, DTBCLK0, otg_inst as u32);
    let en = if src == REFCLK { 0 } else { 1 };
    match dp_hpo_inst {
        0 => REG_UPDATE_2!(dccg_dcn, DPSTREAMCLK_CNTL, DPSTREAMCLK0_EN, en, DPSTREAMCLK0_SRC_SEL, otg_inst),
        1 => REG_UPDATE_2!(dccg_dcn, DPSTREAMCLK_CNTL, DPSTREAMCLK1_EN, en, DPSTREAMCLK1_SRC_SEL, otg_inst),
        2 => REG_UPDATE_2!(dccg_dcn, DPSTREAMCLK_CNTL, DPSTREAMCLK2_EN, en, DPSTREAMCLK2_SRC_SEL, otg_inst),
        3 => REG_UPDATE_2!(dccg_dcn, DPSTREAMCLK_CNTL, DPSTREAMCLK3_EN, en, DPSTREAMCLK3_SRC_SEL, otg_inst),
        _ => { BREAK_TO_DEBUGGER!(); return; }
    }
}

unsafe fn dccg32_otg_add_pixel(dccg: *mut dccg, otg_inst: u32) { let dccg_dcn = TO_DCN_DCCG!(dccg); REG_UPDATE!(dccg_dcn, OTG_PIXEL_RATE_CNTL[otg_inst], OTG_ADD_PIXEL[otg_inst], 1); }
unsafe fn dccg32_otg_drop_pixel(dccg: *mut dccg, otg_inst: u32) { let dccg_dcn = TO_DCN_DCCG!(dccg); REG_UPDATE!(dccg_dcn, OTG_PIXEL_RATE_CNTL[otg_inst], OTG_DROP_PIXEL[otg_inst], 1); }

unsafe fn dccg32_set_hdmistreamclk(dccg: *mut dccg, src: streamclk_source, otg_inst: u32) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    dccg32_set_dtbclk_p_src(dccg, DTBCLK0, otg_inst);
    if src == REFCLK { REG_UPDATE_2!(dccg_dcn, HDMISTREAMCLK_CNTL, HDMISTREAMCLK0_EN, 0, HDMISTREAMCLK0_DTO_FORCE_DIS, 1); }
    else { REG_UPDATE_3!(dccg_dcn, HDMISTREAMCLK_CNTL, HDMISTREAMCLK0_EN, 1, HDMISTREAMCLK0_SRC_SEL, otg_inst, HDMISTREAMCLK0_DTO_FORCE_DIS, 1); }
}

unsafe fn dccg32_enable_hdmicharclk(dccg: *mut dccg, hpo_inst: i32, phypll_inst: i32) {
    let dccg_dcn = TO_DCN_DCCG!(dccg);
    ASSERT!(hpo_inst >= 0 && phypll_inst >= 0);
    REG_UPDATE_2!(dccg_dcn, HDMICHARCLK_CLOCK_CNTL[hpo_inst], HDMICHARCLK0_EN, 1, HDMICHARCLK0_SRC_SEL, phypll_inst);
    match phypll_inst {
        0 => REG_UPDATE_2!(dccg_dcn, PHYASYMCLK_CLOCK_CNTL, PHYASYMCLK_FORCE_EN, 1, PHYASYMCLK_FORCE_SRC_SEL, 1),
        1 => REG_UPDATE_2!(dccg_dcn, PHYBSYMCLK_CLOCK_CNTL, PHYBSYMCLK_FORCE_EN, 1, PHYBSYMCLK_FORCE_SRC_SEL, 1),
        2 => REG_UPDATE_2!(dccg_dcn, PHYCSYMCLK_CLOCK_CNTL, PHYCSYMCLK_FORCE_EN, 1, PHYCSYMCLK_FORCE_SRC_SEL, 1),
        3 => REG_UPDATE_2!(dccg_dcn, PHYDSYMCLK_CLOCK_CNTL, PHYDSYMCLK_FORCE_EN, 1, PHYDSYMCLK_FORCE_SRC_SEL, 1),
        4 => REG_UPDATE_2!(dccg_dcn, PHYESYMCLK_CLOCK_CNTL, PHYESYMCLK_FORCE_EN, 1, PHYESYMCLK_FORCE_SRC_SEL, 1),
        _ => { BREAK_TO_DEBUGGER!(); return; }
    }
}

static mut dccg32_funcs: dccg_funcs = dccg_funcs {
    enable_hdmicharclk: Some(dccg32_enable_hdmicharclk), disable_hdmicharclk: Some(dccg3_disable_hdmicharclk),
    set_hdmistreamclk: Some(dccg32_set_hdmistreamclk), update_dpp_dto: Some(dccg2_update_dpp_dto),
    get_dccg_ref_freq: Some(dccg32_get_dccg_ref_freq), dccg_init: Some(dccg31_init), set_dpstreamclk: Some(dccg32_set_dpstreamclk),
    enable_symclk32_se: Some(dccg31_enable_symclk32_se), disable_symclk32_se: Some(dccg31_disable_symclk32_se),
    enable_symclk32_le: Some(dccg31_enable_symclk32_le), disable_symclk32_le: Some(dccg31_disable_symclk32_le),
    set_physymclk: Some(dccg31_set_physymclk), set_dtbclk_dto: Some(dccg32_set_dtbclk_dto),
    set_valid_pixel_rate: Some(dccg32_set_valid_pixel_rate), set_fifo_errdet_ovr_en: Some(dccg2_set_fifo_errdet_ovr_en),
    set_audio_dtbclk_dto: Some(dccg31_set_audio_dtbclk_dto), otg_add_pixel: Some(dccg32_otg_add_pixel),
    otg_drop_pixel: Some(dccg32_otg_drop_pixel), set_pixel_rate_div: Some(dccg32_set_pixel_rate_div),
    get_pixel_rate_div: Some(dccg32_get_pixel_rate_div), trigger_dio_fifo_resync: Some(dccg32_trigger_dio_fifo_resync),
    set_dtbclk_p_src: Some(dccg32_set_dtbclk_p_src), refclk_setup: Some(dccg2_refclk_setup),
    allow_clock_gating: Some(dccg2_allow_clock_gating), enable_memory_low_power: Some(dccg2_enable_memory_low_power),
    is_s0i3_golden_init_wa_done: Some(dccg2_is_s0i3_golden_init_wa_done),
};

unsafe fn dccg32_create(ctx: *mut dc_context, regs: *const dccg_registers, dccg_shift: *const dccg_shift, dccg_mask: *const dccg_mask) -> *mut dccg {
    let dccg_dcn: *mut dcn_dccg = kzalloc_obj!();
    if dccg_dcn.is_null() { BREAK_TO_DEBUGGER!(); return core::ptr::null_mut(); }
    let base = &mut (*dccg_dcn).base;
    base.ctx = ctx;
    base.funcs = &raw mut dccg32_funcs;
    (*dccg_dcn).regs = regs;
    (*dccg_dcn).dccg_shift = dccg_shift;
    (*dccg_dcn).dccg_mask = dccg_mask;
    &mut (*dccg_dcn).base
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
