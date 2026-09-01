/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tegra210_mbdrc.h - Definitions for Tegra210 MBDRC driver
 *
 * Copyright (c) 2022, NVIDIA CORPORATION. All rights reserved.
 *
 */

/* C includes translated as external Rust dependencies:
 * <linux/platform_device.h> provides platform_device.
 * <sound/soc.h> provides snd_soc_component.
 */

/* Register offsets from TEGRA210_MBDRC*_BASE */
pub const TEGRA210_MBDRC_SOFT_RESET: u32 = 0x4;
pub const TEGRA210_MBDRC_CG: u32 = 0x8;
pub const TEGRA210_MBDRC_STATUS: u32 = 0xc;
pub const TEGRA210_MBDRC_CFG: u32 = 0x28;
pub const TEGRA210_MBDRC_CHANNEL_MASK: u32 = 0x2c;
pub const TEGRA210_MBDRC_MASTER_VOL: u32 = 0x30;
pub const TEGRA210_MBDRC_FAST_FACTOR: u32 = 0x34;

pub const TEGRA210_MBDRC_FILTER_COUNT: u32 = 3;
pub const TEGRA210_MBDRC_FILTER_PARAM_STRIDE: u32 = 0x4;

pub const TEGRA210_MBDRC_IIR_CFG: u32 = 0x38;
pub const TEGRA210_MBDRC_IN_ATTACK: u32 = 0x44;
pub const TEGRA210_MBDRC_IN_RELEASE: u32 = 0x50;
pub const TEGRA210_MBDRC_FAST_ATTACK: u32 = 0x5c;
pub const TEGRA210_MBDRC_IN_THRESHOLD: u32 = 0x68;
pub const TEGRA210_MBDRC_OUT_THRESHOLD: u32 = 0x74;
pub const TEGRA210_MBDRC_RATIO_1ST: u32 = 0x80;
pub const TEGRA210_MBDRC_RATIO_2ND: u32 = 0x8c;
pub const TEGRA210_MBDRC_RATIO_3RD: u32 = 0x98;
pub const TEGRA210_MBDRC_RATIO_4TH: u32 = 0xa4;
pub const TEGRA210_MBDRC_RATIO_5TH: u32 = 0xb0;
pub const TEGRA210_MBDRC_MAKEUP_GAIN: u32 = 0xbc;
pub const TEGRA210_MBDRC_INIT_GAIN: u32 = 0xc8;
pub const TEGRA210_MBDRC_GAIN_ATTACK: u32 = 0xd4;
pub const TEGRA210_MBDRC_GAIN_RELEASE: u32 = 0xe0;
pub const TEGRA210_MBDRC_FAST_RELEASE: u32 = 0xec;
pub const TEGRA210_MBDRC_CFG_RAM_CTRL: u32 = 0xf8;
pub const TEGRA210_MBDRC_CFG_RAM_DATA: u32 = 0x104;

pub const TEGRA210_MBDRC_MAX_REG: u32 = TEGRA210_MBDRC_CFG_RAM_DATA
    + (TEGRA210_MBDRC_FILTER_PARAM_STRIDE * (TEGRA210_MBDRC_FILTER_COUNT - 1));

/* Fields for TEGRA210_MBDRC_CFG */
pub const TEGRA210_MBDRC_CFG_RMS_OFFSET_SHIFT: u32 = 16;
pub const TEGRA210_MBDRC_CFG_RMS_OFFSET_MASK: u32 =
    0x1ff << TEGRA210_MBDRC_CFG_RMS_OFFSET_SHIFT;

pub const TEGRA210_MBDRC_CFG_PEAK_RMS_SHIFT: u32 = 14;
pub const TEGRA210_MBDRC_CFG_PEAK_RMS_MASK: u32 =
    0x1 << TEGRA210_MBDRC_CFG_PEAK_RMS_SHIFT;
pub const TEGRA210_MBDRC_CFG_PEAK: u32 = 1 << TEGRA210_MBDRC_CFG_PEAK_RMS_SHIFT;

pub const TEGRA210_MBDRC_CFG_FILTER_STRUCTURE_SHIFT: u32 = 13;
pub const TEGRA210_MBDRC_CFG_FILTER_STRUCTURE_MASK: u32 =
    0x1 << TEGRA210_MBDRC_CFG_FILTER_STRUCTURE_SHIFT;
pub const TEGRA210_MBDRC_CFG_FILTER_STRUCTURE_FLEX: u32 =
    1 << TEGRA210_MBDRC_CFG_FILTER_STRUCTURE_SHIFT;

pub const TEGRA210_MBDRC_CFG_SHIFT_CTRL_SHIFT: u32 = 8;
pub const TEGRA210_MBDRC_CFG_SHIFT_CTRL_MASK: u32 =
    0x1f << TEGRA210_MBDRC_CFG_SHIFT_CTRL_SHIFT;

pub const TEGRA210_MBDRC_CFG_FRAME_SIZE_SHIFT: u32 = 4;
pub const TEGRA210_MBDRC_CFG_FRAME_SIZE_MASK: u32 =
    0xf << TEGRA210_MBDRC_CFG_FRAME_SIZE_SHIFT;

pub const TEGRA210_MBDRC_CFG_MBDRC_MODE_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_CFG_MBDRC_MODE_MASK: u32 =
    0x3 << TEGRA210_MBDRC_CFG_MBDRC_MODE_SHIFT;
pub const TEGRA210_MBDRC_CFG_MBDRC_MODE_BYPASS: u32 =
    0 << TEGRA210_MBDRC_CFG_MBDRC_MODE_SHIFT;

/* Fields for TEGRA210_MBDRC_CHANNEL_MASK */
pub const TEGRA210_MBDRC_CHANNEL_MASK_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_CHANNEL_MASK_MASK: u32 =
    0xff << TEGRA210_MBDRC_CHANNEL_MASK_SHIFT;

/* Fields for TEGRA210_MBDRC_MASTER_VOL */
pub const TEGRA210_MBDRC_MASTER_VOL_SHIFT: u32 = 23;
pub const TEGRA210_MBDRC_MASTER_VOL_MIN: i32 = -256;
pub const TEGRA210_MBDRC_MASTER_VOL_MAX: i32 = 256;

/* Fields for TEGRA210_MBDRC_FAST_FACTOR */
pub const TEGRA210_MBDRC_FAST_FACTOR_RELEASE_SHIFT: u32 = 16;
pub const TEGRA210_MBDRC_FAST_FACTOR_RELEASE_MASK: u32 =
    0xffff << TEGRA210_MBDRC_FAST_FACTOR_RELEASE_SHIFT;

pub const TEGRA210_MBDRC_FAST_FACTOR_ATTACK_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_FAST_FACTOR_ATTACK_MASK: u32 =
    0xffff << TEGRA210_MBDRC_FAST_FACTOR_ATTACK_SHIFT;

/* Fields for TEGRA210_MBDRC_IIR_CFG */
pub const TEGRA210_MBDRC_IIR_CFG_NUM_STAGES_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_IIR_CFG_NUM_STAGES_MASK: u32 =
    0xf << TEGRA210_MBDRC_IIR_CFG_NUM_STAGES_SHIFT;

/* Fields for TEGRA210_MBDRC_IN_ATTACK */
pub const TEGRA210_MBDRC_IN_ATTACK_TC_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_IN_ATTACK_TC_MASK: u32 =
    0xffffffffu32 << TEGRA210_MBDRC_IN_ATTACK_TC_SHIFT;

/* Fields for TEGRA210_MBDRC_IN_RELEASE */
pub const TEGRA210_MBDRC_IN_RELEASE_TC_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_IN_RELEASE_TC_MASK: u32 =
    0xffffffffu32 << TEGRA210_MBDRC_IN_RELEASE_TC_SHIFT;

/* Fields for TEGRA210_MBDRC_FAST_ATTACK */
pub const TEGRA210_MBDRC_FAST_ATTACK_TC_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_FAST_ATTACK_TC_MASK: u32 =
    0xffffffffu32 << TEGRA210_MBDRC_FAST_ATTACK_TC_SHIFT;

/* Fields for TEGRA210_MBDRC_IN_THRESHOLD / TEGRA210_MBDRC_OUT_THRESHOLD */
pub const TEGRA210_MBDRC_THRESH_4TH_SHIFT: u32 = 24;
pub const TEGRA210_MBDRC_THRESH_4TH_MASK: u32 = 0xff << TEGRA210_MBDRC_THRESH_4TH_SHIFT;

pub const TEGRA210_MBDRC_THRESH_3RD_SHIFT: u32 = 16;
pub const TEGRA210_MBDRC_THRESH_3RD_MASK: u32 = 0xff << TEGRA210_MBDRC_THRESH_3RD_SHIFT;

pub const TEGRA210_MBDRC_THRESH_2ND_SHIFT: u32 = 8;
pub const TEGRA210_MBDRC_THRESH_2ND_MASK: u32 = 0xff << TEGRA210_MBDRC_THRESH_2ND_SHIFT;

pub const TEGRA210_MBDRC_THRESH_1ST_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_THRESH_1ST_MASK: u32 = 0xff << TEGRA210_MBDRC_THRESH_1ST_SHIFT;

/* Fields for TEGRA210_MBDRC_RATIO_1ST */
pub const TEGRA210_MBDRC_RATIO_1ST_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_RATIO_1ST_MASK: u32 = 0xffff << TEGRA210_MBDRC_RATIO_1ST_SHIFT;

/* Fields for TEGRA210_MBDRC_RATIO_2ND */
pub const TEGRA210_MBDRC_RATIO_2ND_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_RATIO_2ND_MASK: u32 = 0xffff << TEGRA210_MBDRC_RATIO_2ND_SHIFT;

/* Fields for TEGRA210_MBDRC_RATIO_3RD */
pub const TEGRA210_MBDRC_RATIO_3RD_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_RATIO_3RD_MASK: u32 = 0xffff << TEGRA210_MBDRC_RATIO_3RD_SHIFT;

/* Fields for TEGRA210_MBDRC_RATIO_4TH */
pub const TEGRA210_MBDRC_RATIO_4TH_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_RATIO_4TH_MASK: u32 = 0xffff << TEGRA210_MBDRC_RATIO_4TH_SHIFT;

/* Fields for TEGRA210_MBDRC_RATIO_5TH */
pub const TEGRA210_MBDRC_RATIO_5TH_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_RATIO_5TH_MASK: u32 = 0xffff << TEGRA210_MBDRC_RATIO_5TH_SHIFT;

/* Fields for TEGRA210_MBDRC_MAKEUP_GAIN */
pub const TEGRA210_MBDRC_MAKEUP_GAIN_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_MAKEUP_GAIN_MASK: u32 =
    0x3f << TEGRA210_MBDRC_MAKEUP_GAIN_SHIFT;

/* Fields for TEGRA210_MBDRC_INIT_GAIN */
pub const TEGRA210_MBDRC_INIT_GAIN_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_INIT_GAIN_MASK: u32 =
    0xffffffffu32 << TEGRA210_MBDRC_INIT_GAIN_SHIFT;

/* Fields for TEGRA210_MBDRC_GAIN_ATTACK */
pub const TEGRA210_MBDRC_GAIN_ATTACK_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_GAIN_ATTACK_MASK: u32 =
    0xffffffffu32 << TEGRA210_MBDRC_GAIN_ATTACK_SHIFT;

/* Fields for TEGRA210_MBDRC_GAIN_RELEASE */
pub const TEGRA210_MBDRC_GAIN_RELEASE_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_GAIN_RELEASE_MASK: u32 =
    0xffffffffu32 << TEGRA210_MBDRC_GAIN_RELEASE_SHIFT;

/* Fields for TEGRA210_MBDRC_FAST_RELEASE */
pub const TEGRA210_MBDRC_FAST_RELEASE_SHIFT: u32 = 0;
pub const TEGRA210_MBDRC_FAST_RELEASE_MASK: u32 =
    0xffffffffu32 << TEGRA210_MBDRC_FAST_RELEASE_SHIFT;

pub const TEGRA210_MBDRC_RAM_CTRL_RW_READ: u32 = 0;
pub const TEGRA210_MBDRC_RAM_CTRL_RW_WRITE: u32 = 1 << 14;
pub const TEGRA210_MBDRC_RAM_CTRL_ADDR_INIT_EN: u32 = 1 << 13;
pub const TEGRA210_MBDRC_RAM_CTRL_SEQ_ACCESS_EN: u32 = 1 << 12;
pub const TEGRA210_MBDRC_RAM_CTRL_RAM_ADDR_MASK: u32 = 0x1ff;

/*
 * Order and size of each structure element for following structures should not
 * be altered size order of elements and their size are based on PEQ co-eff ram
 * and shift ram layout.
 */
pub const TEGRA210_MBDRC_THRESHOLD_NUM: usize = 4;
pub const TEGRA210_MBDRC_RATIO_NUM: usize = TEGRA210_MBDRC_THRESHOLD_NUM + 1;
pub const TEGRA210_MBDRC_MAX_BIQUAD_STAGES: usize = 8;

/* Order of these enums are same as the order of band specific hw registers */
pub const MBDRC_LOW_BAND: u32 = 0;
pub const MBDRC_MID_BAND: u32 = 1;
pub const MBDRC_HIGH_BAND: u32 = 2;
pub const MBDRC_NUM_BAND: usize = 3;

#[repr(C)]
pub struct tegra210_mbdrc_band_params {
    pub band: u32,
    pub iir_stages: u32,
    pub in_attack_tc: u32,
    pub in_release_tc: u32,
    pub fast_attack_tc: u32,
    pub in_threshold: [u32; TEGRA210_MBDRC_THRESHOLD_NUM],
    pub out_threshold: [u32; TEGRA210_MBDRC_THRESHOLD_NUM],
    pub ratio: [u32; TEGRA210_MBDRC_RATIO_NUM],
    pub makeup_gain: u32,
    pub gain_init: u32,
    pub gain_attack_tc: u32,
    pub gain_release_tc: u32,
    pub fast_release_tc: u32,
    /* For biquad_params[][5] order of coeff is b0, b1, a0, a1, a2 */
    pub biquad_params: [u32; TEGRA210_MBDRC_MAX_BIQUAD_STAGES * 5],
}

#[repr(C)]
pub struct tegra210_mbdrc_config {
    pub mode: ::core::ffi::c_uint,
    pub rms_off: ::core::ffi::c_uint,
    pub peak_rms_mode: ::core::ffi::c_uint,
    pub filter_structure: ::core::ffi::c_uint,
    pub shift_ctrl: ::core::ffi::c_uint,
    pub frame_size: ::core::ffi::c_uint,
    pub channel_mask: ::core::ffi::c_uint,
    pub fa_factor: ::core::ffi::c_uint, /* Fast attack factor */
    pub fr_factor: ::core::ffi::c_uint, /* Fast release factor */
    pub band_params: [tegra210_mbdrc_band_params; MBDRC_NUM_BAND],
}

unsafe extern "C" {
    pub fn tegra210_mbdrc_regmap_init(pdev: *mut platform_device) -> ::core::ffi::c_int;
    pub fn tegra210_mbdrc_component_init(
        cmpnt: *mut snd_soc_component,
    ) -> ::core::ffi::c_int;
    pub fn tegra210_mbdrc_hw_params(cmpnt: *mut snd_soc_component) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
