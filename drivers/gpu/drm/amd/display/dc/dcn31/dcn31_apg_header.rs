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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// C dependency: container_of(apg, struct dcn31_apg, base)
macro_rules! DCN31_APG_FROM_APG {
    ($apg:expr) => { container_of!($apg, dcn31_apg, base) };
}

macro_rules! APG_DCN31_REG_LIST {
    ($id:expr) => {
        SRI!(APG_CONTROL, APG, $id),
        SRI!(APG_CONTROL2, APG, $id),
        SRI!(APG_MEM_PWR, APG, $id),
        SRI!(APG_DBG_GEN_CONTROL, APG, $id)
    };
}

#[repr(C)]
pub struct dcn31_apg_registers {
    pub APG_CONTROL: u32,
    pub APG_CONTROL2: u32,
    pub APG_MEM_PWR: u32,
    pub APG_DBG_GEN_CONTROL: u32,
}

macro_rules! DCN31_APG_MASK_SH_LIST {
    ($mask_sh:expr) => {
        SE_SF!(APG0_APG_CONTROL, APG_RESET, $mask_sh),
        SE_SF!(APG0_APG_CONTROL, APG_RESET_DONE, $mask_sh),
        SE_SF!(APG0_APG_CONTROL2, APG_ENABLE, $mask_sh),
        SE_SF!(APG0_APG_CONTROL2, APG_DP_AUDIO_STREAM_ID, $mask_sh),
        SE_SF!(APG0_APG_DBG_GEN_CONTROL, APG_DBG_AUDIO_CHANNEL_ENABLE, $mask_sh),
        SE_SF!(APG0_APG_MEM_PWR, APG_MEM_PWR_FORCE, $mask_sh)
    };
}

macro_rules! APG_DCN31_REG_FIELD_LIST {
    ($type:ty) => {
        pub APG_RESET: $type,
        pub APG_RESET_DONE: $type,
        pub APG_ENABLE: $type,
        pub APG_DP_AUDIO_STREAM_ID: $type,
        pub APG_DBG_AUDIO_CHANNEL_ENABLE: $type,
        pub APG_MEM_PWR_FORCE: $type
    };
}

// APG0_APG_DBG_GEN_CONTROL
pub const APG0_APG_DBG_GEN_CONTROL__APG_DBG_AUDIO_CHANNEL_ENABLE__SHIFT: u32 = 0x8;
pub const APG0_APG_DBG_GEN_CONTROL__APG_DBG_AUDIO_CHANNEL_ENABLE_MASK: u32 = 0x0000_FF00;

/* Not in DCN42B: APG_DBG_GEN_CONTROL, APG0_APG_DBG_60958 */
macro_rules! DCN42B_APG_MASK_SH_LIST {
    ($mask_sh:expr) => {
        SE_SF!(APG0_APG_CONTROL, APG_RESET, $mask_sh),
        SE_SF!(APG0_APG_CONTROL, APG_RESET_DONE, $mask_sh),
        SE_SF!(APG0_APG_CONTROL2, APG_ENABLE, $mask_sh),
        SE_SF!(APG0_APG_CONTROL2, APG_DP_AUDIO_STREAM_ID, $mask_sh),
        SE_SF!(APG0_APG_MEM_PWR, APG_MEM_PWR_FORCE, $mask_sh),
        SE_SF!(APG0_APG_DBG_GEN_CONTROL, APG_DBG_AUDIO_CHANNEL_ENABLE, $mask_sh)
    };
}

#[repr(C)]
pub struct dcn31_apg_shift {
    APG_DCN31_REG_FIELD_LIST!(u8);
}

#[repr(C)]
pub struct dcn31_apg_mask {
    APG_DCN31_REG_FIELD_LIST!(u32);
}

#[repr(C)]
pub struct apg {
    pub funcs: *const apg_funcs,
    pub ctx: *mut dc_context,
    pub inst: i32,
}

#[repr(C)]
pub struct apg_funcs {
    pub setup_hdmi_audio: Option<unsafe extern "C" fn(apg: *mut apg)>,
    pub se_audio_setup: Option<unsafe extern "C" fn(
        apg: *mut apg,
        az_inst: libc::c_uint,
        audio_info: *mut audio_info,
    )>,
    pub enable_apg: Option<unsafe extern "C" fn(apg: *mut apg)>,
    pub disable_apg: Option<unsafe extern "C" fn(apg: *mut apg)>,
}

#[repr(C)]
pub struct dcn31_apg {
    pub base: apg,
    pub regs: *const dcn31_apg_registers,
    pub apg_shift: *const dcn31_apg_shift,
    pub apg_mask: *const dcn31_apg_mask,
}

extern "C" {
    pub fn apg31_construct(
        apg3: *mut dcn31_apg,
        ctx: *mut dc_context,
        inst: u32,
        apg_regs: *const dcn31_apg_registers,
        apg_shift: *const dcn31_apg_shift,
        apg_mask: *const dcn31_apg_mask,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
