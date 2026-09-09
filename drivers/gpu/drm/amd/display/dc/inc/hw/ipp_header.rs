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
 * Authors: AMD
 */

// Dependencies supplied by the corresponding translated headers:
// "hw_shared.h", "dc_hw_types.h"

pub const MAXTRIX_COEFFICIENTS_NUMBER: u32 = 12;
pub const MAXTRIX_COEFFICIENTS_WRAP_NUMBER: u32 = MAXTRIX_COEFFICIENTS_NUMBER + 4;
pub const MAX_OVL_MATRIX_COUNT: u32 = 12;

/* IPP RELATED */
#[repr(C)]
pub struct input_pixel_processor {
    pub ctx: *mut dc_context,
    pub inst: ::core::ffi::c_uint,
    pub funcs: *const ipp_funcs,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipp_prescale_mode {
    IPP_PRESCALE_MODE_BYPASS,
    IPP_PRESCALE_MODE_FIXED_SIGNED,
    IPP_PRESCALE_MODE_FLOAT_SIGNED,
    IPP_PRESCALE_MODE_FIXED_UNSIGNED,
    IPP_PRESCALE_MODE_FLOAT_UNSIGNED,
}

#[repr(C)]
pub struct ipp_prescale_params {
    pub mode: ipp_prescale_mode,
    pub bias: u16,
    pub scale: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ovl_color_space {
    OVL_COLOR_SPACE_UNKNOWN = 0,
    OVL_COLOR_SPACE_RGB,
    OVL_COLOR_SPACE_YUV601,
    OVL_COLOR_SPACE_YUV709,
}

#[repr(C)]
pub struct ipp_funcs {
    /* cursor */
    pub ipp_cursor_set_position: Option<unsafe extern "C" fn(
        ipp: *mut input_pixel_processor,
        position: *const dc_cursor_position,
        param: *const dc_cursor_mi_param,
    )>,

    pub ipp_cursor_set_attributes: Option<unsafe extern "C" fn(
        ipp: *mut input_pixel_processor,
        attributes: *const dc_cursor_attributes,
    )>,

    /* setup input pixel processing */

    /* put the entire pixel processor to bypass */
    pub ipp_full_bypass: Option<unsafe extern "C" fn(ipp: *mut input_pixel_processor)>,

    /* setup ipp to expand/convert input to pixel processor internal format */
    pub ipp_setup: Option<unsafe extern "C" fn(
        ipp: *mut input_pixel_processor,
        format: surface_pixel_format,
        mode: expansion_mode,
        input_csc_color_matrix: dc_csc_transform,
        input_color_space: dc_color_space,
    )>,

    /* DCE function to setup IPP.  TODO: see if we can consolidate to setup */
    pub ipp_program_prescale: Option<unsafe extern "C" fn(
        ipp: *mut input_pixel_processor,
        params: *mut ipp_prescale_params,
    )>,

    pub ipp_program_input_lut: Option<unsafe extern "C" fn(
        ipp: *mut input_pixel_processor,
        gamma: *const dc_gamma,
    )>,

    /* DEGAMMA RELATED */
    pub ipp_set_degamma: Option<unsafe extern "C" fn(
        ipp: *mut input_pixel_processor,
        mode: ipp_degamma_mode,
    )>,

    pub ipp_program_degamma_pwl: Option<unsafe extern "C" fn(
        ipp: *mut input_pixel_processor,
        params: *const pwl_params,
    )>,

    pub ipp_destroy: Option<unsafe extern "C" fn(ipp: *mut *mut input_pixel_processor)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
