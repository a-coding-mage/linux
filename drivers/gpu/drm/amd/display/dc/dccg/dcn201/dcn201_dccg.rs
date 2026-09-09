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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding DCN implementation are intentionally
// left external, corresponding to the C includes in the original file.

extern "C" {
    fn dccg2_get_dccg_ref_freq(dccg: *mut dccg) -> u32;
    fn dccg2_set_fifo_errdet_ovr_en(dccg: *mut dccg, enable: bool);
    fn dccg2_otg_add_pixel(dccg: *mut dccg, otg_inst: u32);
    fn dccg2_otg_drop_pixel(dccg: *mut dccg, otg_inst: u32);
    fn dccg2_init(dccg: *mut dccg);
    fn dccg2_refclk_setup(dccg: *mut dccg, refclk: u32);
    fn dccg2_allow_clock_gating(dccg: *mut dccg, enable: bool);
    fn dccg2_enable_memory_low_power(dccg: *mut dccg, enable: bool);
    fn dccg2_is_s0i3_golden_init_wa_done(dccg: *mut dccg) -> bool;
    fn break_to_debugger();
}

#[repr(C)]
pub struct dccg {
    pub ctx: *mut dc_context,
    pub funcs: *const dccg_funcs,
}

#[repr(C)]
pub struct dcn_dccg {
    pub base: dccg,
    pub regs: *const dccg_registers,
    pub dccg_shift: *const dccg_shift,
    pub dccg_mask: *const dccg_mask,
}

#[repr(C)]
pub struct dccg_funcs {
    pub update_dpp_dto: Option<unsafe extern "C" fn(*mut dccg, i32, i32)>,
    pub get_dccg_ref_freq: Option<unsafe extern "C" fn(*mut dccg) -> u32>,
    pub set_fifo_errdet_ovr_en: Option<unsafe extern "C" fn(*mut dccg, bool)>,
    pub otg_add_pixel: Option<unsafe extern "C" fn(*mut dccg, u32)>,
    pub otg_drop_pixel: Option<unsafe extern "C" fn(*mut dccg, u32)>,
    pub dccg_init: Option<unsafe extern "C" fn(*mut dccg)>,
    pub refclk_setup: Option<unsafe extern "C" fn(*mut dccg, u32)>,
    pub allow_clock_gating: Option<unsafe extern "C" fn(*mut dccg, bool)>,
    pub enable_memory_low_power: Option<unsafe extern "C" fn(*mut dccg, bool)>,
    pub is_s0i3_golden_init_wa_done: Option<unsafe extern "C" fn(*mut dccg) -> bool>,
}

#[repr(C)] pub struct dc_context { _private: [u8; 0] }
#[repr(C)] pub struct dccg_registers { _private: [u8; 0] }
#[repr(C)] pub struct dccg_shift { _private: [u8; 0] }
#[repr(C)] pub struct dccg_mask { _private: [u8; 0] }

unsafe extern "C" fn dccg201_update_dpp_dto(
    _dccg: *mut dccg,
    _dpp_inst: i32,
    _req_dppclk: i32,
) {
    /* vbios handles it */
}

static DCCG201_FUNCS: dccg_funcs = dccg_funcs {
    update_dpp_dto: Some(dccg201_update_dpp_dto),
    get_dccg_ref_freq: Some(dccg2_get_dccg_ref_freq),
    set_fifo_errdet_ovr_en: Some(dccg2_set_fifo_errdet_ovr_en),
    otg_add_pixel: Some(dccg2_otg_add_pixel),
    otg_drop_pixel: Some(dccg2_otg_drop_pixel),
    dccg_init: Some(dccg2_init),
    refclk_setup: Some(dccg2_refclk_setup), // Deprecated - backward compatibility only
    allow_clock_gating: Some(dccg2_allow_clock_gating),
    enable_memory_low_power: Some(dccg2_enable_memory_low_power),
    is_s0i3_golden_init_wa_done: Some(dccg2_is_s0i3_golden_init_wa_done), // Deprecated
};

pub unsafe extern "C" fn dccg201_create(
    ctx: *mut dc_context,
    regs: *const dccg_registers,
    dccg_shift: *const dccg_shift,
    dccg_mask: *const dccg_mask,
) -> *mut dccg {
    let dccg_dcn = Box::into_raw(Box::new(std::mem::zeroed::<dcn_dccg>()));
    if dccg_dcn.is_null() {
        break_to_debugger();
        return std::ptr::null_mut();
    }

    let base = &mut (*dccg_dcn).base;
    base.ctx = ctx;
    base.funcs = &DCCG201_FUNCS;

    (*dccg_dcn).regs = regs;
    (*dccg_dcn).dccg_shift = dccg_shift;
    (*dccg_dcn).dccg_mask = dccg_mask;

    &mut (*dccg_dcn).base
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
