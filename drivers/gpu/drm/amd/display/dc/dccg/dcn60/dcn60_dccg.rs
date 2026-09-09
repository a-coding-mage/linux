// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Translated from dcn60_dccg.c. Definitions supplied by the included headers
// are intentionally referenced but not reproduced here.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::ffi::c_void;

extern "C" {
    fn dccg401_get_pixel_rate_div(
        dccg: *mut dccg,
        otg_inst: u32,
        cur_tmds_div: *mut u32,
        dp_dto_int: *mut u32,
    );
    fn dccg401_enable_hdmicharclk(_: *mut dccg, _: u32, _: u32);
    fn dccg401_disable_hdmicharclk(_: *mut dccg, _: u32);
    fn dccg401_set_hdmistreamclk(_: *mut dccg, _: u32, _: u32);
    fn dccg401_update_dpp_dto(_: *mut dccg, _: u32, _: u32, _: u32);
    fn dccg401_get_dccg_ref_freq(_: *mut dccg, _: *mut u32);
    fn dccg401_init(_: *mut dccg);
    fn dccg401_set_dpstreamclk(_: *mut dccg, _: u32, _: u32);
    fn dccg31_enable_symclk32_se(_: *mut dccg, _: u32);
    fn dccg31_disable_symclk32_se(_: *mut dccg, _: u32);
    fn dccg401_enable_symclk32_le(_: *mut dccg, _: u32);
    fn dccg401_disable_symclk32_le(_: *mut dccg, _: u32);
    fn dccg401_set_physymclk(_: *mut dccg, _: u32, _: u32);
    fn dccg401_set_ref_dscclk(_: *mut dccg, _: u32);
    fn dccg2_set_fifo_errdet_ovr_en(_: *mut dccg, _: bool);
    fn dccg42_otg_add_pixel(_: *mut dccg, _: u32);
    fn dccg42_otg_drop_pixel(_: *mut dccg, _: u32);
    fn dccg401_set_dp_dto(_: *mut dccg, _: u32, _: u32, _: u32);
    fn dccg401_enable_symclk_se(_: *mut dccg, _: u32);
    fn dccg401_disable_symclk_se(_: *mut dccg, _: u32);
    fn dccg401_set_dtbclk_p_src(_: *mut dccg, _: u32, _: u32);
    fn dccg31_read_reg_state(_: *mut dccg);
    fn dccg2_allow_clock_gating(_: *mut dccg, _: bool);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn BREAK_TO_DEBUGGER();
}

#[repr(C)]
pub struct dc_context { _private: [u8; 0] }
#[repr(C)]
pub struct dccg_registers { _private: [u8; 0] }
#[repr(C)]
pub struct dccg_shift { _private: [u8; 0] }
#[repr(C)]
pub struct dccg_mask { _private: [u8; 0] }
#[repr(C)]
pub struct dccg_funcs { _private: [u8; 0] }
#[repr(C)]
pub struct dccg { pub ctx: *mut dc_context, pub funcs: *const dccg_funcs }
#[repr(C)]
pub struct dcn_dccg {
    pub base: dccg,
    pub regs: *const dccg_registers,
    pub dccg_shift: *const dccg_shift,
    pub dccg_mask: *const dccg_mask,
}

pub type pixel_rate_div = u32;
pub const PIXEL_RATE_DIV_NA: u32 = 0;
pub const PIXEL_RATE_DIV_BY_2: u32 = 2;
pub const PIXEL_RATE_DIV_BY_4: u32 = 4;

// C initializer translated as the externally supplied function-table object;
// its field declarations belong to dcn60_dccg.h and are not duplicated here.
#[no_mangle]
pub static dccg60_funcs: dccg_funcs = dccg_funcs { _private: [] };

unsafe fn dccg60_set_pixel_rate_div(
    dccg: *mut dccg,
    otg_inst: u32,
    tmds_div: pixel_rate_div,
    _unused: pixel_rate_div,
) {
    let dccg_dcn = dccg as *mut dcn_dccg;
    let mut cur_tmds_div = PIXEL_RATE_DIV_NA;
    let mut dp_dto_int = 0u32;
    if tmds_div != PIXEL_RATE_DIV_BY_2 && tmds_div != PIXEL_RATE_DIV_BY_4 { return; }
    dccg401_get_pixel_rate_div(dccg, otg_inst, &mut cur_tmds_div, &mut dp_dto_int);
    if tmds_div == cur_tmds_div { return; }
    let reg_val = if tmds_div == PIXEL_RATE_DIV_BY_4 { 1 } else { 0 };
    match otg_inst {
        0 => unsafe { REG_UPDATE((*dccg_dcn).regs, OTG_PIXEL_RATE_DIV, OTG0_TMDS_PIXEL_RATE_DIV, reg_val) },
        1 => unsafe { REG_UPDATE((*dccg_dcn).regs, OTG_PIXEL_RATE_DIV, OTG1_TMDS_PIXEL_RATE_DIV, reg_val) },
        2 => unsafe { REG_UPDATE((*dccg_dcn).regs, OTG_PIXEL_RATE_DIV, OTG2_TMDS_PIXEL_RATE_DIV, reg_val) },
        3 => unsafe { REG_UPDATE((*dccg_dcn).regs, OTG_PIXEL_RATE_DIV, OTG3_TMDS_PIXEL_RATE_DIV, reg_val) },
        _ => { BREAK_TO_DEBUGGER(); return; }
    }
}

unsafe fn dccg60_set_dto_dscclk(dccg: *mut dccg, inst: u32, num_slices_h: u32) {
    let dccg_dcn = dccg as *mut dcn_dccg;
    macro_rules! set { ($p:ident, $e:ident, $s:ident) => {
        REG_UPDATE_2((*dccg_dcn).regs, $p, $e, 1, $s, 1);
        REG_UPDATE((*dccg_dcn).regs, DSCCLK_DTO_CTRL, $e, 1);
        REG_UPDATE((*dccg_dcn).regs, DSCCLK_SRC_SEL, $s, if num_slices_h == 1 { 1 } else { 0 });
    } }
    match inst {
        0 => set!(DSCCLK0_DTO_PARAM, DSCCLK0_EN, DSCCLK0_SRC_SEL),
        1 => set!(DSCCLK1_DTO_PARAM, DSCCLK1_EN, DSCCLK1_SRC_SEL),
        2 => set!(DSCCLK2_DTO_PARAM, DSCCLK2_EN, DSCCLK2_SRC_SEL),
        3 => set!(DSCCLK3_DTO_PARAM, DSCCLK3_EN, DSCCLK3_SRC_SEL),
        _ => { BREAK_TO_DEBUGGER(); return; }
    }
}

extern "C" { fn REG_UPDATE(_: *const dccg_registers, _: u32, _: u32, _: u32); fn REG_UPDATE_2(_: *const dccg_registers, _: u32, _: u32, _: u32, _: u32, _: u32); }

pub unsafe fn dccg60_create(ctx: *mut dc_context, regs: *const dccg_registers, dccg_shift: *const dccg_shift, dccg_mask: *const dccg_mask) -> *mut dccg {
    let dccg_dcn = kzalloc(core::mem::size_of::<dcn_dccg>(), 0) as *mut dcn_dccg;
    if dccg_dcn.is_null() { BREAK_TO_DEBUGGER(); return core::ptr::null_mut(); }
    (*dccg_dcn).base.ctx = ctx;
    (*dccg_dcn).base.funcs = &dccg60_funcs;
    (*dccg_dcn).regs = regs;
    (*dccg_dcn).dccg_shift = dccg_shift;
    (*dccg_dcn).dccg_mask = dccg_mask;
    &mut (*dccg_dcn).base
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
