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
 *
 */

// Interface to CPLIB/PSP to enable ASSR

use core::ffi::c_void;

pub struct dc_link;

#[repr(C)]
pub struct cp_psp_stream_config {
    pub otg_inst: u8,
    pub dig_be: u8,
    pub dig_fe: u8,
    pub link_enc_idx: u8,
    pub stream_enc_idx: u8,
    pub dio_output_idx: u8,
    pub phy_idx: u8,
    pub assr_enabled: u8,
    pub mst_enabled: u8,
    pub frl_enabled: u8,
    pub dp2_enabled: u8,
    pub usb4_enabled: u8,
    pub dm_stream_ctx: *mut c_void,
    pub dpms_off: bool,
}

#[repr(C)]
pub struct cp_psp_funcs {
    pub enable_assr:
        Option<unsafe extern "C" fn(handle: *mut c_void, link: *mut dc_link) -> bool>,
    pub update_stream_config:
        Option<unsafe extern "C" fn(handle: *mut c_void, config: *mut cp_psp_stream_config)>,
}

#[repr(C)]
pub struct cp_psp {
    pub handle: *mut c_void,
    pub funcs: cp_psp_funcs,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
