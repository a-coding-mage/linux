/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// The following macros refer to dependency-provided `container_of`, `SRI`, and
// `SE_SF` macros and are preserved for source-level compatibility.
macro_rules! DCN31_AFMT_FROM_AFMT {
    ($afmt:expr) => { container_of!($afmt, dcn31_afmt, base) };
}

macro_rules! AFMT_DCN31_REG_LIST {
    ($id:expr) => {
        SRI!(AFMT_INFOFRAME_CONTROL0, AFMT, $id),
        SRI!(AFMT_VBI_PACKET_CONTROL, AFMT, $id),
        SRI!(AFMT_AUDIO_PACKET_CONTROL, AFMT, $id),
        SRI!(AFMT_AUDIO_PACKET_CONTROL2, AFMT, $id),
        SRI!(AFMT_AUDIO_SRC_CONTROL, AFMT, $id),
        SRI!(AFMT_60958_0, AFMT, $id),
        SRI!(AFMT_60958_1, AFMT, $id),
        SRI!(AFMT_60958_2, AFMT, $id),
        SRI!(AFMT_MEM_PWR, AFMT, $id)
    };
}

#[repr(C)]
pub struct dcn31_afmt_registers {
    pub AFMT_INFOFRAME_CONTROL0: u32,
    pub AFMT_VBI_PACKET_CONTROL: u32,
    pub AFMT_AUDIO_PACKET_CONTROL: u32,
    pub AFMT_AUDIO_PACKET_CONTROL2: u32,
    pub AFMT_AUDIO_SRC_CONTROL: u32,
    pub AFMT_60958_0: u32,
    pub AFMT_60958_1: u32,
    pub AFMT_60958_2: u32,
    pub AFMT_MEM_PWR: u32,
}

macro_rules! DCN31_AFMT_MASK_SH_LIST {
    ($mask_sh:expr) => {
        SE_SF!(AFMT0_AFMT_INFOFRAME_CONTROL0, AFMT_AUDIO_INFO_UPDATE, $mask_sh),
        SE_SF!(AFMT0_AFMT_AUDIO_SRC_CONTROL, AFMT_AUDIO_SRC_SELECT, $mask_sh),
        SE_SF!(AFMT0_AFMT_AUDIO_PACKET_CONTROL2, AFMT_AUDIO_CHANNEL_ENABLE, $mask_sh),
        SE_SF!(AFMT0_AFMT_AUDIO_PACKET_CONTROL, AFMT_60958_CS_UPDATE, $mask_sh),
        SE_SF!(AFMT0_AFMT_AUDIO_PACKET_CONTROL2, AFMT_AUDIO_LAYOUT_OVRD, $mask_sh),
        SE_SF!(AFMT0_AFMT_AUDIO_PACKET_CONTROL2, AFMT_60958_OSF_OVRD, $mask_sh),
        SE_SF!(AFMT0_AFMT_60958_0, AFMT_60958_CS_CHANNEL_NUMBER_L, $mask_sh),
        SE_SF!(AFMT0_AFMT_60958_0, AFMT_60958_CS_CLOCK_ACCURACY, $mask_sh),
        SE_SF!(AFMT0_AFMT_60958_1, AFMT_60958_CS_CHANNEL_NUMBER_R, $mask_sh),
        SE_SF!(AFMT0_AFMT_60958_2, AFMT_60958_CS_CHANNEL_NUMBER_2, $mask_sh),
        SE_SF!(AFMT0_AFMT_60958_2, AFMT_60958_CS_CHANNEL_NUMBER_3, $mask_sh),
        SE_SF!(AFMT0_AFMT_60958_2, AFMT_60958_CS_CHANNEL_NUMBER_4, $mask_sh),
        SE_SF!(AFMT0_AFMT_60958_2, AFMT_60958_CS_CHANNEL_NUMBER_5, $mask_sh),
        SE_SF!(AFMT0_AFMT_60958_2, AFMT_60958_CS_CHANNEL_NUMBER_6, $mask_sh),
        SE_SF!(AFMT0_AFMT_60958_2, AFMT_60958_CS_CHANNEL_NUMBER_7, $mask_sh),
        SE_SF!(AFMT0_AFMT_AUDIO_PACKET_CONTROL, AFMT_AUDIO_SAMPLE_SEND, $mask_sh),
        SE_SF!(AFMT0_AFMT_MEM_PWR, AFMT_MEM_PWR_FORCE, $mask_sh),
        SE_SF!(AFMT0_AFMT_MEM_PWR, AFMT_MEM_PWR_DIS, $mask_sh),
        SE_SF!(AFMT0_AFMT_MEM_PWR, AFMT_MEM_PWR_STATE, $mask_sh)
    };
}

macro_rules! AFMT_DCN31_REG_FIELD_LIST {
    ($type:ty) => {
        AFMT_AUDIO_INFO_UPDATE: $type,
        AFMT_AUDIO_SRC_SELECT: $type,
        AFMT_AUDIO_CHANNEL_ENABLE: $type,
        AFMT_60958_CS_UPDATE: $type,
        AFMT_AUDIO_LAYOUT_OVRD: $type,
        AFMT_60958_OSF_OVRD: $type,
        AFMT_60958_CS_CHANNEL_NUMBER_L: $type,
        AFMT_60958_CS_CLOCK_ACCURACY: $type,
        AFMT_60958_CS_CHANNEL_NUMBER_R: $type,
        AFMT_60958_CS_CHANNEL_NUMBER_2: $type,
        AFMT_60958_CS_CHANNEL_NUMBER_3: $type,
        AFMT_60958_CS_CHANNEL_NUMBER_4: $type,
        AFMT_60958_CS_CHANNEL_NUMBER_5: $type,
        AFMT_60958_CS_CHANNEL_NUMBER_6: $type,
        AFMT_60958_CS_CHANNEL_NUMBER_7: $type,
        AFMT_AUDIO_SAMPLE_SEND: $type,
        AFMT_MEM_PWR_FORCE: $type,
        AFMT_MEM_PWR_DIS: $type,
        AFMT_MEM_PWR_STATE: $type,
    };
}

#[repr(C)]
pub struct dcn31_afmt_shift {
    AFMT_DCN31_REG_FIELD_LIST!(u8);
}

#[repr(C)]
pub struct dcn31_afmt_mask {
    AFMT_DCN31_REG_FIELD_LIST!(u32);
}

#[repr(C)]
pub struct dcn31_afmt {
    pub base: afmt,
    pub regs: *const dcn31_afmt_registers,
    pub afmt_shift: *const dcn31_afmt_shift,
    pub afmt_mask: *const dcn31_afmt_mask,
}

extern "C" {
    pub fn afmt31_poweron(afmt: *mut afmt);
    pub fn afmt31_powerdown(afmt: *mut afmt);
    pub fn afmt31_construct(
        afmt31: *mut dcn31_afmt,
        ctx: *mut dc_context,
        inst: u32,
        afmt_regs: *const dcn31_afmt_registers,
        afmt_shift: *const dcn31_afmt_shift,
        afmt_mask: *const dcn31_afmt_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
