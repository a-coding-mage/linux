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
 */

// Dependency: declarations from audio.h are supplied by the surrounding translation unit.

// Register-list and field-list macros retain their source-level intent.
macro_rules! AUD_COMMON_REG_LIST { ($id:expr) => { (
    SRI!(AZALIA_F0_CODEC_ENDPOINT_INDEX, AZF0ENDPOINT, $id),
    SRI!(AZALIA_F0_CODEC_ENDPOINT_DATA, AZF0ENDPOINT, $id),
    SR!(AZALIA_F0_CODEC_FUNCTION_PARAMETER_STREAM_FORMATS),
    SR!(AZALIA_F0_CODEC_FUNCTION_PARAMETER_SUPPORTED_SIZE_RATES),
    SR!(AZALIA_F0_CODEC_FUNCTION_PARAMETER_POWER_STATES),
    SR!(DCCG_AUDIO_DTO_SOURCE), SR!(DCCG_AUDIO_DTO0_MODULE),
    SR!(DCCG_AUDIO_DTO0_PHASE), SR!(DCCG_AUDIO_DTO1_MODULE),
    SR!(DCCG_AUDIO_DTO1_PHASE)
) } }

macro_rules! SF { ($reg_name:ident, $field_name:ident, $post_fix:ident) => {
    $reg_name ## __ ## $field_name ## $post_fix
} }

// CONFIG_DRM_AMD_DC_SI controls whether the DCE6.0 list is available.

#[repr(C)]
pub struct dce_audio_registers {
    pub AZALIA_F0_CODEC_ENDPOINT_INDEX: u32,
    pub AZALIA_F0_CODEC_ENDPOINT_DATA: u32,
    pub AZALIA_F0_CODEC_FUNCTION_PARAMETER_STREAM_FORMATS: u32,
    pub AZALIA_F0_CODEC_FUNCTION_PARAMETER_SUPPORTED_SIZE_RATES: u32,
    pub AZALIA_F0_CODEC_FUNCTION_PARAMETER_POWER_STATES: u32,
    pub DCCG_AUDIO_DTO_SOURCE: u32,
    pub DCCG_AUDIO_DTO0_MODULE: u32,
    pub DCCG_AUDIO_DTO0_PHASE: u32,
    pub DCCG_AUDIO_DTO1_MODULE: u32,
    pub DCCG_AUDIO_DTO1_PHASE: u32,
    pub AUDIO_RATE_CAPABILITIES: u32,
}

#[repr(C)]
pub struct dce_audio_shift {
    pub AZALIA_ENDPOINT_REG_INDEX: u8,
    pub AZALIA_ENDPOINT_REG_DATA: u8,
    pub AUDIO_RATE_CAPABILITIES: u8,
    pub CLKSTOP: u8,
    pub EPSS: u8,
    pub DCCG_AUDIO_DTO0_SOURCE_SEL: u8,
    pub DCCG_AUDIO_DTO_SEL: u8,
    pub DCCG_AUDIO_DTO0_MODULE: u8,
    pub DCCG_AUDIO_DTO0_PHASE: u8,
    pub DCCG_AUDIO_DTO1_MODULE: u8,
    pub DCCG_AUDIO_DTO1_PHASE: u8,
    pub DCCG_AUDIO_DTO2_USE_512FBR_DTO: u8,
    pub DCCG_AUDIO_DTO0_USE_512FBR_DTO: u32,
    pub DCCG_AUDIO_DTO1_USE_512FBR_DTO: u32,
    pub CLOCK_GATING_DISABLE: u32,
}

#[repr(C)]
pub struct dce_audio_mask {
    pub AZALIA_ENDPOINT_REG_INDEX: u32,
    pub AZALIA_ENDPOINT_REG_DATA: u32,
    pub AUDIO_RATE_CAPABILITIES: u32,
    pub CLKSTOP: u32,
    pub EPSS: u32,
    pub DCCG_AUDIO_DTO0_SOURCE_SEL: u32,
    pub DCCG_AUDIO_DTO_SEL: u32,
    pub DCCG_AUDIO_DTO0_MODULE: u32,
    pub DCCG_AUDIO_DTO0_PHASE: u32,
    pub DCCG_AUDIO_DTO1_MODULE: u32,
    pub DCCG_AUDIO_DTO1_PHASE: u32,
    pub DCCG_AUDIO_DTO2_USE_512FBR_DTO: u32,
    pub DCCG_AUDIO_DTO0_USE_512FBR_DTO: u32,
    pub DCCG_AUDIO_DTO1_USE_512FBR_DTO: u32,
    pub CLOCK_GATING_DISABLE: u32,
}

#[repr(C)]
pub struct dce_audio {
    pub base: audio,
    pub regs: *const dce_audio_registers,
    pub shifts: *const dce_audio_shift,
    pub masks: *const dce_audio_mask,
}

extern "C" {
    pub fn dce_audio_create(ctx: *mut dc_context, inst: c_uint,
        reg: *const dce_audio_registers, shifts: *const dce_audio_shift,
        masks: *const dce_audio_mask) -> *mut audio;
    pub fn dce_aud_destroy(audio: *mut *mut audio);
    pub fn dce_aud_hw_init(audio: *mut audio);
    pub fn dce_aud_az_enable(audio: *mut audio);
    pub fn dce_aud_az_disable(audio: *mut audio);
    pub fn dce_aud_az_disable_hbr_audio(audio: *mut audio);
    pub fn dce_aud_az_configure(audio: *mut audio, signal: signal_type,
        crtc_info: *const audio_crtc_info, audio_info: *const audio_info,
        dp_link_info: *const audio_dp_link_info);
    pub fn dce_aud_wall_dto_setup(audio: *mut audio, signal: signal_type,
        crtc_info: *const audio_crtc_info, pll_info: *const audio_pll_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
