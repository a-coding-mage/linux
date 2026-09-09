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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the corresponding C/Rust headers.

#[allow(improper_ctypes)]
extern "C" {
    fn dccg2_update_dpp_dto();
    fn dccg2_get_dccg_ref_freq();
    fn dccg2_set_fifo_errdet_ovr_en();
    fn dccg2_otg_add_pixel();
    fn dccg2_otg_drop_pixel();
    fn dccg2_init();
    fn dccg2_refclk_setup();
    fn dccg2_allow_clock_gating();
    fn dccg2_enable_memory_low_power();
    fn dccg2_is_s0i3_golden_init_wa_done();
}

// Register helper macros from reg_helper.h are represented by the external
// register-access operations used below.

pub unsafe fn dccg3_enable_hdmicharclk(
    dccg: *mut dccg,
    hpo_inst: i32,
    phypll_inst: i32,
) {
    let dccg_dcn = to_dcn_dccg(dccg);

    assert!(hpo_inst >= 0 && phypll_inst >= 0);
    reg_update_2(
        (*(*dccg_dcn).regs).HDMICHARCLK_CLOCK_CNTL[hpo_inst as usize],
        HDMICHARCLK0_EN,
        1,
        HDMICHARCLK0_SRC_SEL,
        phypll_inst,
    );

    /* Enable FORCE_EN for SYMCLK */
    match phypll_inst {
        0 => reg_update_2(PHYASYMCLK_CLOCK_CNTL, PHYASYMCLK_FORCE_EN, 1,
                          PHYASYMCLK_FORCE_SRC_SEL, 1),
        1 => reg_update_2(PHYBSYMCLK_CLOCK_CNTL, PHYBSYMCLK_FORCE_EN, 1,
                          PHYBSYMCLK_FORCE_SRC_SEL, 1),
        2 => reg_update_2(PHYCSYMCLK_CLOCK_CNTL, PHYCSYMCLK_FORCE_EN, 1,
                          PHYCSYMCLK_FORCE_SRC_SEL, 1),
        _ => {
            break_to_debugger();
            return;
        }
    }
}

pub unsafe fn dccg3_disable_hdmicharclk(dccg: *mut dccg, hpo_inst: i32) {
    let dccg_dcn = to_dcn_dccg(dccg);
    reg_write((*(*dccg_dcn).regs).HDMICHARCLK_CLOCK_CNTL[hpo_inst as usize], 0);
}

static dccg3_funcs: dccg_funcs = dccg_funcs {
    enable_hdmicharclk: Some(dccg3_enable_hdmicharclk),
    disable_hdmicharclk: Some(dccg3_disable_hdmicharclk),
    update_dpp_dto: Some(dccg2_update_dpp_dto),
    get_dccg_ref_freq: Some(dccg2_get_dccg_ref_freq),
    set_fifo_errdet_ovr_en: Some(dccg2_set_fifo_errdet_ovr_en),
    otg_add_pixel: Some(dccg2_otg_add_pixel),
    otg_drop_pixel: Some(dccg2_otg_drop_pixel),
    dccg_init: Some(dccg2_init),
    refclk_setup: Some(dccg2_refclk_setup), // Deprecated - for backward compatibility only
    allow_clock_gating: Some(dccg2_allow_clock_gating),
    enable_memory_low_power: Some(dccg2_enable_memory_low_power),
    is_s0i3_golden_init_wa_done: Some(dccg2_is_s0i3_golden_init_wa_done), // Deprecated - for backward compatibility only
};

pub unsafe fn dccg3_create(
    ctx: *mut dc_context,
    regs: *const dccg_registers,
    dccg_shift: *const dccg_shift,
    dccg_mask: *const dccg_mask,
) -> *mut dccg {
    let dccg_dcn = kzalloc_obj::<dcn_dccg>();
    if dccg_dcn.is_null() {
        break_to_debugger();
        return core::ptr::null_mut();
    }
    let base = &mut (*dccg_dcn).base;
    (*base).ctx = ctx;
    (*base).funcs = &dccg3_funcs;
    (*dccg_dcn).regs = regs;
    (*dccg_dcn).dccg_shift = dccg_shift;
    (*dccg_dcn).dccg_mask = dccg_mask;
    base
}

pub unsafe fn dccg30_create(
    ctx: *mut dc_context,
    regs: *const dccg_registers,
    dccg_shift: *const dccg_shift,
    dccg_mask: *const dccg_mask,
) -> *mut dccg {
    let dccg_dcn = kzalloc_obj::<dcn_dccg>();
    if dccg_dcn.is_null() {
        break_to_debugger();
        return core::ptr::null_mut();
    }
    let base = &mut (*dccg_dcn).base;
    (*base).ctx = ctx;
    (*base).funcs = &dccg3_funcs;
    (*dccg_dcn).regs = regs;
    (*dccg_dcn).dccg_shift = dccg_shift;
    (*dccg_dcn).dccg_mask = dccg_mask;
    base
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
