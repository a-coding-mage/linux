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

// Dependencies supplied by the surrounding translation unit:
// reg_helper.h, core_types.h, and dcn301_dccg.h

macro_rules! to_dcn_dccg {
    ($dccg:expr) => {
        container_of!($dccg, dcn_dccg, base)
    };
}

macro_rules! reg {
    ($dccg_dcn:expr, $reg:ident) => {
        (*(*$dccg_dcn).regs).$reg
    };
}

macro_rules! fn_field {
    ($dccg_dcn:expr, $field_name:ident) => {
        ((*(*$dccg_dcn).dccg_shift).$field_name,
         (*(*$dccg_dcn).dccg_mask).$field_name)
    };
}

macro_rules! ctx {
    ($dccg_dcn:expr) => {
        (*$dccg_dcn).base.ctx
    };
}

macro_rules! dc_logger {
    ($dccg:expr) => {
        (*$dccg).ctx.logger
    };
}

static DCCG301_FUNCS: dccg_funcs = dccg_funcs {
    update_dpp_dto: dccg2_update_dpp_dto,
    get_dccg_ref_freq: dccg2_get_dccg_ref_freq,
    set_fifo_errdet_ovr_en: dccg2_set_fifo_errdet_ovr_en,
    otg_add_pixel: dccg2_otg_add_pixel,
    otg_drop_pixel: dccg2_otg_drop_pixel,
    dccg_init: dccg2_init,
    refclk_setup: dccg2_refclk_setup, // Deprecated - for backward compatibility only
    allow_clock_gating: dccg2_allow_clock_gating,
    enable_memory_low_power: dccg2_enable_memory_low_power,
    is_s0i3_golden_init_wa_done: dccg2_is_s0i3_golden_init_wa_done, // Deprecated - for backward compatibility only
};

pub unsafe extern "C" fn dccg301_create(
    ctx: *mut dc_context,
    regs: *const dccg_registers,
    dccg_shift: *const dccg_shift,
    dccg_mask: *const dccg_mask,
) -> *mut dccg {
    let dccg_dcn: *mut dcn_dccg = {
        // Equivalent to kzalloc_obj(*dccg_dcn): zero-initialized heap storage.
        Box::into_raw(Box::new(std::mem::zeroed::<dcn_dccg>()))
    };
    let base: *mut dccg;

    if dccg_dcn.is_null() {
        BREAK_TO_DEBUGGER!();
        return std::ptr::null_mut();
    }

    base = &mut (*dccg_dcn).base;
    (*base).ctx = ctx;
    (*base).funcs = &DCCG301_FUNCS;

    (*dccg_dcn).regs = regs;
    (*dccg_dcn).dccg_shift = dccg_shift;
    (*dccg_dcn).dccg_mask = dccg_mask;

    &mut (*dccg_dcn).base
}

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

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
