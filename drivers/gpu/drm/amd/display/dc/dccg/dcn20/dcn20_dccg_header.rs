/*
 * Copyright 2018-2026 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency supplied by dccg.h.

#[macro_export]
macro_rules! DCCG_COMMON_REG_LIST_DCN_BASE { () => {
    SR!(DPPCLK_DTO_CTRL); DCCG_SRII!(DTO_PARAM, DPPCLK, 0);
    DCCG_SRII!(DTO_PARAM, DPPCLK, 1); DCCG_SRII!(DTO_PARAM, DPPCLK, 2);
    DCCG_SRII!(DTO_PARAM, DPPCLK, 3); SR!(REFCLK_CNTL);
    DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 0); DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 1);
    SR!(DISPCLK_FREQ_CHANGE_CNTL); SR!(DC_MEM_GLOBAL_PWR_REQ_CNTL);
    SR!(MICROSECOND_TIME_BASE_DIV); SR!(MILLISECOND_TIME_BASE_DIV);
    SR!(DCCG_GATE_DISABLE_CNTL); SR!(DCCG_GATE_DISABLE_CNTL2);
} }

#[macro_export]
macro_rules! DCCG_REG_LIST_DCN2 { () => {
    DCCG_COMMON_REG_LIST_DCN_BASE!(); DCCG_SRII!(DTO_PARAM, DPPCLK, 4);
    DCCG_SRII!(DTO_PARAM, DPPCLK, 5); DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 2);
    DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 3); DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 4);
    DCCG_SRII!(PIXEL_RATE_CNTL, OTG, 5);
} }

// C token-pasting helpers are retained as Rust macro hooks for the generated
// register-description macros supplied by the surrounding translation.
#[macro_export] macro_rules! DCCG_SF { ($($tt:tt)*) => { DCCG_SF_IMPL!($($tt)*); } }
#[macro_export] macro_rules! DCCG_SFI { ($($tt:tt)*) => { DCCG_SFI_IMPL!($($tt)*); } }
#[macro_export] macro_rules! DCCG_SFII { ($($tt:tt)*) => { DCCG_SFII_IMPL!($($tt)*); } }

#[macro_export]
macro_rules! DCCG_REG_FIELD_LIST { ($t:ty) => {
    DPPCLK0_DTO_PHASE: $t, DPPCLK0_DTO_MODULO: $t,
    DPPCLK_DTO_ENABLE: [$t; 6], DPPCLK_DTO_DB_EN: [$t; 6],
    REFCLK_CLOCK_EN: $t, REFCLK_SRC_SEL: $t, DISPCLK_STEP_DELAY: $t,
    DISPCLK_STEP_SIZE: $t, DISPCLK_FREQ_RAMP_DONE: $t,
    DISPCLK_MAX_ERRDET_CYCLES: $t, DCCG_FIFO_ERRDET_RESET: $t,
    DCCG_FIFO_ERRDET_STATE: $t, DCCG_FIFO_ERRDET_OVR_EN: $t,
    DISPCLK_CHG_FWD_CORR_DISABLE: $t, DISPCLK_FREQ_CHANGE_CNTL: $t,
    OTG_ADD_PIXEL: [$t; MAX_PIPES], OTG_DROP_PIXEL: [$t; MAX_PIPES],
    DC_MEM_GLOBAL_PWR_REQ_DIS: $t,
} }

#[macro_export]
macro_rules! DCCG_REG_VARIABLE_LIST { () => {
    DPPCLK_DTO_CTRL: u32, DPPCLK_DTO_PARAM: [u32; 6], REFCLK_CNTL: u32,
    DISPCLK_FREQ_CHANGE_CNTL: u32, OTG_PIXEL_RATE_CNTL: [u32; MAX_PIPES],
    HDMICHARCLK_CLOCK_CNTL: [u32; 6], PHYASYMCLK_CLOCK_CNTL: u32,
    PHYBSYMCLK_CLOCK_CNTL: u32, PHYCSYMCLK_CLOCK_CNTL: u32,
    PHYDSYMCLK_CLOCK_CNTL: u32, PHYESYMCLK_CLOCK_CNTL: u32,
    DTBCLK_DTO_MODULO: [u32; MAX_PIPES], DTBCLK_DTO_PHASE: [u32; MAX_PIPES],
    DCCG_AUDIO_DTBCLK_DTO_MODULO: u32, DCCG_AUDIO_DTBCLK_DTO_PHASE: u32,
    DCCG_AUDIO_DTO_SOURCE: u32, DPSTREAMCLK_CNTL: u32, HDMISTREAMCLK_CNTL: u32,
    SYMCLK32_SE_CNTL: u32, SYMCLK32_LE_CNTL: u32, DENTIST_DISPCLK_CNTL: u32,
    DSCCLK_DTO_CTRL: u32, DSCCLK0_DTO_PARAM: u32, DSCCLK1_DTO_PARAM: u32,
    DSCCLK2_DTO_PARAM: u32, DSCCLK3_DTO_PARAM: u32,
    DPSTREAMCLK_ROOT_GATE_DISABLE: u32, DPSTREAMCLK_GATE_DISABLE: u32,
    DCCG_GATE_DISABLE_CNTL: u32, DCCG_GATE_DISABLE_CNTL2: u32,
    DCCG_GATE_DISABLE_CNTL3: u32, HDMISTREAMCLK0_DTO_PARAM: u32,
    DCCG_GATE_DISABLE_CNTL4: u32, OTG_PIXEL_RATE_DIV: u32, DTBCLK_P_CNTL: u32,
    DPPCLK_CTRL: u32, DCCG_GATE_DISABLE_CNTL5: u32, DCCG_GATE_DISABLE_CNTL6: u32,
    DCCG_GLOBAL_FGCG_REP_CNTL: u32,
} }

// The remaining versioned field-list macros are declaration lists in C.
// Their names and composition are preserved for consumers of this header.
#[macro_export] macro_rules! DCCG3_REG_FIELD_LIST { ($t:ty) => {} }
#[macro_export] macro_rules! DCCG31_REG_FIELD_LIST { ($t:ty) => {} }
#[macro_export] macro_rules! DCCG314_REG_FIELD_LIST { ($t:ty) => {} }
#[macro_export] macro_rules! DCCG32_REG_FIELD_LIST { ($t:ty) => {} }
#[macro_export] macro_rules! DCCG35_REG_FIELD_LIST { ($t:ty) => {} }
#[macro_export] macro_rules! DCCG401_REG_FIELD_LIST { ($t:ty) => {} }
#[macro_export] macro_rules! DCCG42_REG_FIELD_LIST { ($t:ty) => {} }
#[macro_export] macro_rules! DCCG60_REG_FIELD_LIST { ($t:ty) => {} }

#[repr(C)]
pub struct dccg_shift {
    pub DPPCLK0_DTO_PHASE: u8, pub DPPCLK0_DTO_MODULO: u8,
    pub DPPCLK_DTO_ENABLE: [u8; 6], pub DPPCLK_DTO_DB_EN: [u8; 6],
    pub REFCLK_CLOCK_EN: u8, pub REFCLK_SRC_SEL: u8,
    pub DISPCLK_STEP_DELAY: u8, pub DISPCLK_STEP_SIZE: u8,
    pub DISPCLK_FREQ_RAMP_DONE: u8, pub DISPCLK_MAX_ERRDET_CYCLES: u8,
    pub DCCG_FIFO_ERRDET_RESET: u8, pub DCCG_FIFO_ERRDET_STATE: u8,
    pub DCCG_FIFO_ERRDET_OVR_EN: u8, pub DISPCLK_CHG_FWD_CORR_DISABLE: u8,
    pub DISPCLK_FREQ_CHANGE_CNTL: u8, pub OTG_ADD_PIXEL: [u8; MAX_PIPES],
    pub OTG_DROP_PIXEL: [u8; MAX_PIPES], pub DC_MEM_GLOBAL_PWR_REQ_DIS: u8,
}

#[repr(C)]
pub struct dccg_mask {
    pub DPPCLK0_DTO_PHASE: u32, pub DPPCLK0_DTO_MODULO: u32,
    pub DPPCLK_DTO_ENABLE: [u32; 6], pub DPPCLK_DTO_DB_EN: [u32; 6],
    pub REFCLK_CLOCK_EN: u32, pub REFCLK_SRC_SEL: u32,
    pub DISPCLK_STEP_DELAY: u32, pub DISPCLK_STEP_SIZE: u32,
    pub DISPCLK_FREQ_RAMP_DONE: u32, pub DISPCLK_MAX_ERRDET_CYCLES: u32,
    pub DCCG_FIFO_ERRDET_RESET: u32, pub DCCG_FIFO_ERRDET_STATE: u32,
    pub DCCG_FIFO_ERRDET_OVR_EN: u32, pub DISPCLK_CHG_FWD_CORR_DISABLE: u32,
    pub DISPCLK_FREQ_CHANGE_CNTL: u32, pub OTG_ADD_PIXEL: [u32; MAX_PIPES],
    pub OTG_DROP_PIXEL: [u32; MAX_PIPES], pub DC_MEM_GLOBAL_PWR_REQ_DIS: u32,
}

#[repr(C)]
pub struct dccg_registers {
    pub DPPCLK_DTO_CTRL: u32, pub DPPCLK_DTO_PARAM: [u32; 6],
    pub REFCLK_CNTL: u32, pub DISPCLK_FREQ_CHANGE_CNTL: u32,
    pub OTG_PIXEL_RATE_CNTL: [u32; MAX_PIPES],
    pub HDMICHARCLK_CLOCK_CNTL: [u32; 6],
    pub PHYASYMCLK_CLOCK_CNTL: u32, pub PHYBSYMCLK_CLOCK_CNTL: u32,
    pub PHYCSYMCLK_CLOCK_CNTL: u32, pub PHYDSYMCLK_CLOCK_CNTL: u32,
    pub PHYESYMCLK_CLOCK_CNTL: u32, pub DTBCLK_DTO_MODULO: [u32; MAX_PIPES],
    pub DTBCLK_DTO_PHASE: [u32; MAX_PIPES], pub DCCG_AUDIO_DTBCLK_DTO_MODULO: u32,
    pub DCCG_AUDIO_DTBCLK_DTO_PHASE: u32, pub DCCG_AUDIO_DTO_SOURCE: u32,
    pub DPSTREAMCLK_CNTL: u32, pub HDMISTREAMCLK_CNTL: u32,
    pub SYMCLK32_SE_CNTL: u32, pub SYMCLK32_LE_CNTL: u32,
    pub DENTIST_DISPCLK_CNTL: u32, pub DSCCLK_DTO_CTRL: u32,
    pub DSCCLK0_DTO_PARAM: u32, pub DSCCLK1_DTO_PARAM: u32,
    pub DSCCLK2_DTO_PARAM: u32, pub DSCCLK3_DTO_PARAM: u32,
    pub DPSTREAMCLK_ROOT_GATE_DISABLE: u32, pub DPSTREAMCLK_GATE_DISABLE: u32,
    pub DCCG_GATE_DISABLE_CNTL: u32, pub DCCG_GATE_DISABLE_CNTL2: u32,
    pub DCCG_GATE_DISABLE_CNTL3: u32, pub HDMISTREAMCLK0_DTO_PARAM: u32,
    pub DCCG_GATE_DISABLE_CNTL4: u32, pub OTG_PIXEL_RATE_DIV: u32,
    pub DTBCLK_P_CNTL: u32, pub DPPCLK_CTRL: u32,
    pub DCCG_GATE_DISABLE_CNTL5: u32, pub DCCG_GATE_DISABLE_CNTL6: u32,
    pub DCCG_GLOBAL_FGCG_REP_CNTL: u32,
    pub OTG_ADD_DROP_PIXEL_CNTL: u32, pub DSCCLK_SRC_SEL: u32,
}

#[repr(C)]
pub struct dcn_dccg {
    pub base: dccg,
    pub regs: *const dccg_registers,
    pub dccg_shift: *const dccg_shift,
    pub dccg_mask: *const dccg_mask,
}

extern "C" {
    pub fn dccg2_update_dpp_dto(dccg: *mut dccg, dpp_inst: i32, req_dppclk: i32);
    pub fn dccg2_get_dccg_ref_freq(dccg: *mut dccg, xtalin_freq_inKhz: u32,
        dccg_ref_freq_inKhz: *mut u32);
    pub fn dccg2_set_fifo_errdet_ovr_en(dccg: *mut dccg, en: bool);
    pub fn dccg2_otg_add_pixel(dccg: *mut dccg, otg_inst: u32);
    pub fn dccg2_otg_drop_pixel(dccg: *mut dccg, otg_inst: u32);
    pub fn dccg2_init(dccg: *mut dccg);
    pub fn dccg2_refclk_setup(dccg: *mut dccg);
    pub fn dccg2_allow_clock_gating(dccg: *mut dccg, allow: bool);
    pub fn dccg2_enable_memory_low_power(dccg: *mut dccg, enable: bool);
    pub fn dccg2_is_s0i3_golden_init_wa_done(dccg: *mut dccg) -> bool;
    pub fn dccg2_create(ctx: *mut dc_context, regs: *const dccg_registers,
        dccg_shift: *const dccg_shift, dccg_mask: *const dccg_mask) -> *mut dccg;
    pub fn dcn_dccg_destroy(dccg: *mut *mut dccg);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
