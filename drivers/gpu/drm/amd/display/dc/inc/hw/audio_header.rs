/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Dependency declarations from audio_types.h are supplied externally.

#[repr(C)]
pub struct audio_funcs {
    pub endpoint_valid: Option<unsafe extern "C" fn(audio: *mut audio) -> bool>,

    pub hw_init: Option<unsafe extern "C" fn(audio: *mut audio)>,

    pub az_enable: Option<unsafe extern "C" fn(audio: *mut audio)>,

    pub az_disable: Option<unsafe extern "C" fn(audio: *mut audio)>,

    pub az_configure: Option<unsafe extern "C" fn(
        audio: *mut audio,
        signal: signal_type,
        crtc_info: *const audio_crtc_info,
        audio_info: *const audio_info,
        dp_link_info: *const audio_dp_link_info,
    )>,

    pub az_disable_hbr_audio: Option<unsafe extern "C" fn(audio: *mut audio)>,

    pub wall_dto_setup: Option<unsafe extern "C" fn(
        audio: *mut audio,
        signal: signal_type,
        crtc_info: *const audio_crtc_info,
        pll_info: *const audio_pll_info,
    )>,

    pub destroy: Option<unsafe extern "C" fn(audio: *mut *mut audio)>,
}

#[repr(C)]
pub struct audio {
    pub funcs: *const audio_funcs,
    pub ctx: *mut dc_context,
    pub inst: u32,
    pub enabled: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
