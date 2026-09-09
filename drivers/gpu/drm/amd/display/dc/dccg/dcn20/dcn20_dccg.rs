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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies supplied by the surrounding translation unit.

pub unsafe fn dccg2_update_dpp_dto(dccg: *mut dccg, dpp_inst: i32, req_dppclk: i32) {
    let dccg_dcn = TO_DCN_DCCG(dccg);

    if (*dccg).ref_dppclk != 0 && req_dppclk != 0 {
        let ref_dppclk = (*dccg).ref_dppclk;
        let modulo: i32 = 0xff;
        // phase / modulo = dpp pipe clk / dpp global clk
        let mut phase = ((modulo * req_dppclk) + ref_dppclk - 1) / ref_dppclk;

        if phase > 0xff {
            ASSERT(false);
            phase = 0xff;
        }

        REG_SET_2((*dccg_dcn).regs, DPPCLK_DTO_PARAM[dpp_inst as usize], 0,
            DPPCLK0_DTO_PHASE, phase,
            DPPCLK0_DTO_MODULO, modulo);
        REG_UPDATE((*dccg_dcn).regs, DPPCLK_DTO_CTRL,
            DPPCLK_DTO_ENABLE[dpp_inst as usize], 1);
    } else {
        REG_UPDATE((*dccg_dcn).regs, DPPCLK_DTO_CTRL,
            DPPCLK_DTO_ENABLE[dpp_inst as usize], 0);
    }

    (*dccg).pipe_dppclk_khz[dpp_inst as usize] = req_dppclk;
}

pub unsafe fn dccg2_get_dccg_ref_freq(
    dccg: *mut dccg,
    xtalin_freq_inKhz: u32,
    dccg_ref_freq_inKhz: *mut u32,
) {
    let dccg_dcn = TO_DCN_DCCG(dccg);
    let mut clk_en: u32 = 0;
    let mut clk_sel: u32 = 0;

    REG_GET_2((*dccg_dcn).regs, REFCLK_CNTL, REFCLK_CLOCK_EN, &mut clk_en,
        REFCLK_SRC_SEL, &mut clk_sel);

    if clk_en != 0 {
        // DCN20 has never been validated for non-xtalin as reference
        // frequency. There's actually no way for DC to determine what
        // frequency a non-xtalin source is.
        ASSERT_CRITICAL(false);
    }

    *dccg_ref_freq_inKhz = xtalin_freq_inKhz;
}

pub unsafe fn dccg2_set_fifo_errdet_ovr_en(dccg: *mut dccg, en: bool) {
    let dccg_dcn = TO_DCN_DCCG(dccg);
    REG_UPDATE((*dccg_dcn).regs, DISPCLK_FREQ_CHANGE_CNTL,
        DCCG_FIFO_ERRDET_OVR_EN, if en { 1 } else { 0 });
}

pub unsafe fn dccg2_otg_add_pixel(dccg: *mut dccg, otg_inst: u32) {
    let dccg_dcn = TO_DCN_DCCG(dccg);
    REG_UPDATE_2((*dccg_dcn).regs, OTG_PIXEL_RATE_CNTL[otg_inst as usize],
        OTG_ADD_PIXEL[otg_inst as usize], 0,
        OTG_DROP_PIXEL[otg_inst as usize], 0);
    REG_UPDATE((*dccg_dcn).regs, OTG_PIXEL_RATE_CNTL[otg_inst as usize],
        OTG_ADD_PIXEL[otg_inst as usize], 1);
}

pub unsafe fn dccg2_otg_drop_pixel(dccg: *mut dccg, otg_inst: u32) {
    let dccg_dcn = TO_DCN_DCCG(dccg);
    REG_UPDATE_2((*dccg_dcn).regs, OTG_PIXEL_RATE_CNTL[otg_inst as usize],
        OTG_ADD_PIXEL[otg_inst as usize], 0,
        OTG_DROP_PIXEL[otg_inst as usize], 0);
    REG_UPDATE((*dccg_dcn).regs, OTG_PIXEL_RATE_CNTL[otg_inst as usize],
        OTG_DROP_PIXEL[otg_inst as usize], 1);
}

pub unsafe fn dccg2_init(dccg: *mut dccg) {
    let dccg_dcn = TO_DCN_DCCG(dccg);
    // Hardcoded register values for DCN20. These are specific to 100Mhz refclk.
    REG_WRITE((*dccg_dcn).regs, MICROSECOND_TIME_BASE_DIV, 0x00120264);
    REG_WRITE((*dccg_dcn).regs, MILLISECOND_TIME_BASE_DIV, 0x001186a0);
    REG_WRITE((*dccg_dcn).regs, DISPCLK_FREQ_CHANGE_CNTL, 0x0e01003c);
    if REG((*dccg_dcn).regs, REFCLK_CNTL) != 0 {
        REG_WRITE((*dccg_dcn).regs, REFCLK_CNTL, 0);
    }
}

pub unsafe fn dccg2_refclk_setup(dccg: *mut dccg) {
    let dccg_dcn = TO_DCN_DCCG(dccg);
    // REFCLK programming that must occur after hubbub initialization.
    if REG((*dccg_dcn).regs, REFCLK_CNTL) != 0 {
        REG_WRITE((*dccg_dcn).regs, REFCLK_CNTL, 0);
    }
}

pub unsafe fn dccg2_is_s0i3_golden_init_wa_done(dccg: *mut dccg) -> bool {
    let dccg_dcn = TO_DCN_DCCG(dccg);
    REG_READ((*dccg_dcn).regs, MICROSECOND_TIME_BASE_DIV) == 0x00120464
}

pub unsafe fn dccg2_allow_clock_gating(dccg: *mut dccg, allow: bool) {
    let dccg_dcn = TO_DCN_DCCG(dccg);
    if allow {
        REG_WRITE((*dccg_dcn).regs, DCCG_GATE_DISABLE_CNTL, 0);
        REG_WRITE((*dccg_dcn).regs, DCCG_GATE_DISABLE_CNTL2, 0);
    } else {
        REG_WRITE((*dccg_dcn).regs, DCCG_GATE_DISABLE_CNTL, 0xFFFFFFFF);
        REG_WRITE((*dccg_dcn).regs, DCCG_GATE_DISABLE_CNTL2, 0xFFFFFFFF);
    }
}

pub unsafe fn dccg2_enable_memory_low_power(dccg: *mut dccg, enable: bool) {
    let dccg_dcn = TO_DCN_DCCG(dccg);
    REG_UPDATE((*dccg_dcn).regs, DC_MEM_GLOBAL_PWR_REQ_CNTL,
        DC_MEM_GLOBAL_PWR_REQ_DIS, if enable { 0 } else { 1 });
}

pub static dccg2_funcs: dccg_funcs = dccg_funcs {
    update_dpp_dto: Some(dccg2_update_dpp_dto),
    get_dccg_ref_freq: Some(dccg2_get_dccg_ref_freq),
    set_fifo_errdet_ovr_en: Some(dccg2_set_fifo_errdet_ovr_en),
    otg_add_pixel: Some(dccg2_otg_add_pixel),
    otg_drop_pixel: Some(dccg2_otg_drop_pixel),
    dccg_init: Some(dccg2_init),
    refclk_setup: Some(dccg2_refclk_setup),
    allow_clock_gating: Some(dccg2_allow_clock_gating),
    enable_memory_low_power: Some(dccg2_enable_memory_low_power),
    is_s0i3_golden_init_wa_done: Some(dccg2_is_s0i3_golden_init_wa_done),
};

pub unsafe fn dccg2_create(
    ctx: *mut dc_context,
    regs: *const dccg_registers,
    dccg_shift: *const dccg_shift,
    dccg_mask: *const dccg_mask,
) -> *mut dccg {
    let dccg_dcn: *mut dcn_dccg = kzalloc_obj();
    if dccg_dcn.is_null() {
        BREAK_TO_DEBUGGER();
        return core::ptr::null_mut();
    }
    let base = &mut (*dccg_dcn).base;
    base.ctx = ctx;
    base.funcs = &dccg2_funcs;
    (*dccg_dcn).regs = regs;
    (*dccg_dcn).dccg_shift = dccg_shift;
    (*dccg_dcn).dccg_mask = dccg_mask;
    &mut (*dccg_dcn).base
}

pub unsafe fn dcn_dccg_destroy(dccg: *mut *mut dccg) {
    let dccg_dcn = TO_DCN_DCCG(*dccg);
    kfree(dccg_dcn);
    *dccg = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
