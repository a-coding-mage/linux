/*
 * Copyright 2012-16 Advanced Micro Devices, Inc.
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
// dc_types.h, dmub_cmd.h

use core::ffi::c_void;

#[repr(C)]
pub struct dc_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct psr_context {
    _private: [u8; 0],
}

// External enum types from dc_types.h.
pub type dc_psr_state = c_void;
pub type psr_residency_mode = c_void;

#[repr(C)]
pub struct dmub_psr {
    pub ctx: *mut dc_context,
    pub funcs: *const dmub_psr_funcs,
}

#[repr(C)]
pub struct dmub_psr_funcs {
    pub psr_copy_settings: Option<unsafe extern "C" fn(
        dmub: *mut dmub_psr,
        link: *mut dc_link,
        psr_context: *mut psr_context,
        panel_inst: u8,
    ) -> bool>,
    pub psr_enable: Option<unsafe extern "C" fn(
        dmub: *mut dmub_psr,
        enable: bool,
        wait: bool,
        panel_inst: u8,
    )>,
    pub psr_get_state: Option<unsafe extern "C" fn(
        dmub: *mut dmub_psr,
        dc_psr_state: *mut dc_psr_state,
        panel_inst: u8,
    )>,
    pub psr_set_level: Option<unsafe extern "C" fn(
        dmub: *mut dmub_psr,
        psr_level: u16,
        panel_inst: u8,
    )>,
    pub psr_force_static: Option<unsafe extern "C" fn(
        dmub: *mut dmub_psr,
        panel_inst: u8,
    )>,
    pub psr_get_residency: Option<unsafe extern "C" fn(
        dmub: *mut dmub_psr,
        residency: *mut u32,
        panel_inst: u8,
        mode: psr_residency_mode,
    )>,
    pub psr_set_sink_vtotal_in_psr_active: Option<unsafe extern "C" fn(
        dmub: *mut dmub_psr,
        psr_vtotal_idle: u16,
        psr_vtotal_su: u16,
    )>,
    pub psr_set_power_opt: Option<unsafe extern "C" fn(
        dmub: *mut dmub_psr,
        power_opt: u32,
        panel_inst: u8,
    )>,
}

unsafe extern "C" {
    pub fn dmub_psr_create(ctx: *mut dc_context) -> *mut dmub_psr;
    pub fn dmub_psr_destroy(dmub: *mut *mut dmub_psr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
