/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 */
/*
 * panel_cntl.h
 *
 *  Created on: Oct 6, 2015
 *      Author: yonsun
 */

// Dependency supplied by the surrounding translation unit: dc_types.h

pub const MAX_BACKLIGHT_LEVEL: u32 = 0xFFFF;

#[repr(C)]
pub struct panel_cntl_backlight_registers {
    pub BL_PWM_CNTL: ::core::ffi::c_uint,
    pub BL_PWM_CNTL2: ::core::ffi::c_uint,
    pub BL_PWM_PERIOD_CNTL: ::core::ffi::c_uint,
    pub LVTMA_PWRSEQ_REF_DIV_BL_PWM_REF_DIV: ::core::ffi::c_uint,
    pub PANEL_PWRSEQ_REF_DIV2: ::core::ffi::c_uint,
    pub USER_LEVEL: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct panel_cntl_funcs {
    pub destroy: Option<unsafe extern "C" fn(panel_cntl: *mut *mut panel_cntl)>,
    pub hw_init: Option<unsafe extern "C" fn(panel_cntl: *mut panel_cntl) -> u32>,
    pub is_panel_backlight_on:
        Option<unsafe extern "C" fn(panel_cntl: *mut panel_cntl) -> bool>,
    pub is_panel_powered_on:
        Option<unsafe extern "C" fn(panel_cntl: *mut panel_cntl) -> bool>,
    pub store_backlight_level: Option<unsafe extern "C" fn(panel_cntl: *mut panel_cntl)>,
    pub driver_set_backlight:
        Option<unsafe extern "C" fn(panel_cntl: *mut panel_cntl, backlight_pwm_u16_16: u32)>,
    pub get_current_backlight:
        Option<unsafe extern "C" fn(panel_cntl: *mut panel_cntl) -> u32>,
}

#[repr(C)]
pub struct panel_cntl_init_data {
    pub ctx: *mut dc_context,
    pub inst: u32,
    pub eng_id: u32,
}

#[repr(C)]
pub struct panel_cntl {
    pub funcs: *const panel_cntl_funcs,
    pub ctx: *mut dc_context,
    pub inst: u32,
    pub pwrseq_inst: u32,
    /* registers setting needs to be saved and restored at InitBacklight */
    pub stored_backlight_registers: panel_cntl_backlight_registers,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
