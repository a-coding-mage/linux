/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// C preprocessor macros retained as Rust macro equivalents; dependent macros
// and types are supplied by the surrounding translation unit.
macro_rules! DCN30_AFMT_FROM_AFMT { ($afmt:expr) => { container_of!($afmt, dcn30_afmt, base) }; }
macro_rules! AFMT_DCN3_REG_LIST {
    ($id:expr) => {
        SRI!(AFMT_INFOFRAME_CONTROL0, AFMT, $id), SRI!(AFMT_VBI_PACKET_CONTROL, AFMT, $id),
        SRI!(AFMT_AUDIO_PACKET_CONTROL, AFMT, $id), SRI!(AFMT_AUDIO_PACKET_CONTROL2, AFMT, $id),
        SRI!(AFMT_AUDIO_SRC_CONTROL, AFMT, $id), SRI!(AFMT_60958_0, AFMT, $id),
        SRI!(AFMT_60958_1, AFMT, $id), SRI!(AFMT_60958_2, AFMT, $id), SRI!(AFMT_MEM_PWR, AFMT, $id)
    };
}

#[repr(C)]
pub struct dcn30_afmt_registers {
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

macro_rules! AFMT_DCN3_REG_FIELD_LIST { ($t:ty) => {
    AFMT_AUDIO_INFO_UPDATE: $t, AFMT_AUDIO_SRC_SELECT: $t,
    AFMT_AUDIO_CHANNEL_ENABLE: $t, AFMT_60958_CS_UPDATE: $t,
    AFMT_AUDIO_LAYOUT_OVRD: $t, AFMT_60958_OSF_OVRD: $t,
    AFMT_60958_CS_CHANNEL_NUMBER_L: $t, AFMT_60958_CS_CLOCK_ACCURACY: $t,
    AFMT_60958_CS_CHANNEL_NUMBER_R: $t, AFMT_60958_CS_CHANNEL_NUMBER_2: $t,
    AFMT_60958_CS_CHANNEL_NUMBER_3: $t, AFMT_60958_CS_CHANNEL_NUMBER_4: $t,
    AFMT_60958_CS_CHANNEL_NUMBER_5: $t, AFMT_60958_CS_CHANNEL_NUMBER_6: $t,
    AFMT_60958_CS_CHANNEL_NUMBER_7: $t, AFMT_AUDIO_SAMPLE_SEND: $t,
    AFMT_MEM_PWR_FORCE: $t
}; }

#[repr(C)]
pub struct dcn30_afmt_shift {
    pub AFMT_AUDIO_INFO_UPDATE: u8, pub AFMT_AUDIO_SRC_SELECT: u8,
    pub AFMT_AUDIO_CHANNEL_ENABLE: u8, pub AFMT_60958_CS_UPDATE: u8,
    pub AFMT_AUDIO_LAYOUT_OVRD: u8, pub AFMT_60958_OSF_OVRD: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_L: u8, pub AFMT_60958_CS_CLOCK_ACCURACY: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_R: u8, pub AFMT_60958_CS_CHANNEL_NUMBER_2: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_3: u8, pub AFMT_60958_CS_CHANNEL_NUMBER_4: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_5: u8, pub AFMT_60958_CS_CHANNEL_NUMBER_6: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_7: u8, pub AFMT_AUDIO_SAMPLE_SEND: u8,
    pub AFMT_MEM_PWR_FORCE: u8,
}

#[repr(C)]
pub struct dcn30_afmt_mask {
    pub AFMT_AUDIO_INFO_UPDATE: u32, pub AFMT_AUDIO_SRC_SELECT: u32,
    pub AFMT_AUDIO_CHANNEL_ENABLE: u32, pub AFMT_60958_CS_UPDATE: u32,
    pub AFMT_AUDIO_LAYOUT_OVRD: u32, pub AFMT_60958_OSF_OVRD: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_L: u32, pub AFMT_60958_CS_CLOCK_ACCURACY: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_R: u32, pub AFMT_60958_CS_CHANNEL_NUMBER_2: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_3: u32, pub AFMT_60958_CS_CHANNEL_NUMBER_4: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_5: u32, pub AFMT_60958_CS_CHANNEL_NUMBER_6: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_7: u32, pub AFMT_AUDIO_SAMPLE_SEND: u32,
    pub AFMT_MEM_PWR_FORCE: u32,
}

#[repr(C)] pub struct afmt_funcs {
    pub setup_hdmi_audio: Option<unsafe extern "C" fn(*mut afmt)>,
    pub se_audio_setup: Option<unsafe extern "C" fn(*mut afmt, libc::c_uint, *mut audio_info)>,
    pub audio_mute_control: Option<unsafe extern "C" fn(*mut afmt, bool)>,
    pub audio_info_immediate_update: Option<unsafe extern "C" fn(*mut afmt)>,
    pub setup_dp_audio: Option<unsafe extern "C" fn(*mut afmt)>,
    pub afmt_poweron: Option<unsafe extern "C" fn(*mut afmt)>,
    pub afmt_powerdown: Option<unsafe extern "C" fn(*mut afmt)>,
}

#[repr(C)] pub struct afmt { pub funcs: *const afmt_funcs, pub ctx: *mut dc_context, pub inst: i32 }
#[repr(C)] pub struct dcn30_afmt {
    pub base: afmt,
    pub regs: *const dcn30_afmt_registers,
    pub afmt_shift: *const dcn30_afmt_shift,
    pub afmt_mask: *const dcn30_afmt_mask,
}

extern "C" {
    pub fn afmt3_setup_hdmi_audio(afmt: *mut afmt);
    pub fn afmt3_se_audio_setup(afmt: *mut afmt, az_inst: libc::c_uint, audio_info: *mut audio_info);
    pub fn afmt3_audio_mute_control(afmt: *mut afmt, mute: bool);
    pub fn afmt3_audio_info_immediate_update(afmt: *mut afmt);
    pub fn afmt3_setup_dp_audio(afmt: *mut afmt);
    pub fn afmt3_construct(afmt3: *mut dcn30_afmt, ctx: *mut dc_context, inst: u32,
        afmt_regs: *const dcn30_afmt_registers, afmt_shift: *const dcn30_afmt_shift,
        afmt_mask: *const dcn30_afmt_mask);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
