/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tegra210_dmic.h - Definitions for Tegra210 DMIC driver
 *
 * Copyright (c) 2020 NVIDIA CORPORATION.  All rights reserved.
 *
 */

/* Register offsets from DMIC BASE */
pub const TEGRA210_DMIC_TX_STATUS: u32 = 0x0c;
pub const TEGRA210_DMIC_TX_INT_STATUS: u32 = 0x10;
pub const TEGRA210_DMIC_TX_INT_MASK: u32 = 0x14;
pub const TEGRA210_DMIC_TX_INT_SET: u32 = 0x18;
pub const TEGRA210_DMIC_TX_INT_CLEAR: u32 = 0x1c;
pub const TEGRA210_DMIC_TX_CIF_CTRL: u32 = 0x20;
pub const TEGRA210_DMIC_ENABLE: u32 = 0x40;
pub const TEGRA210_DMIC_SOFT_RESET: u32 = 0x44;
pub const TEGRA210_DMIC_CG: u32 = 0x48;
pub const TEGRA210_DMIC_STATUS: u32 = 0x4c;
pub const TEGRA210_DMIC_INT_STATUS: u32 = 0x50;
pub const TEGRA210_DMIC_CTRL: u32 = 0x64;
pub const TEGRA210_DMIC_DBG_CTRL: u32 = 0x70;
pub const TEGRA210_DMIC_DCR_BIQUAD_0_COEF_4: u32 = 0x88;
pub const TEGRA210_DMIC_LP_FILTER_GAIN: u32 = 0x8c;
pub const TEGRA210_DMIC_LP_BIQUAD_0_COEF_0: u32 = 0x90;
pub const TEGRA210_DMIC_LP_BIQUAD_0_COEF_1: u32 = 0x94;
pub const TEGRA210_DMIC_LP_BIQUAD_0_COEF_2: u32 = 0x98;
pub const TEGRA210_DMIC_LP_BIQUAD_0_COEF_3: u32 = 0x9c;
pub const TEGRA210_DMIC_LP_BIQUAD_0_COEF_4: u32 = 0xa0;
pub const TEGRA210_DMIC_LP_BIQUAD_1_COEF_0: u32 = 0xa4;
pub const TEGRA210_DMIC_LP_BIQUAD_1_COEF_1: u32 = 0xa8;
pub const TEGRA210_DMIC_LP_BIQUAD_1_COEF_2: u32 = 0xac;
pub const TEGRA210_DMIC_LP_BIQUAD_1_COEF_3: u32 = 0xb0;
pub const TEGRA210_DMIC_LP_BIQUAD_1_COEF_4: u32 = 0xb4;

/* Fields in TEGRA210_DMIC_CTRL */
pub const CH_SEL_SHIFT: u32 = 8;
pub const TEGRA210_DMIC_CTRL_CHANNEL_SELECT_MASK: u32 = 0x3 << CH_SEL_SHIFT;
pub const LRSEL_POL_SHIFT: u32 = 4;
pub const TEGRA210_DMIC_CTRL_LRSEL_POLARITY_MASK: u32 = 0x1 << LRSEL_POL_SHIFT;
pub const OSR_SHIFT: u32 = 0;
pub const TEGRA210_DMIC_CTRL_OSR_MASK: u32 = 0x3 << OSR_SHIFT;

pub const DMIC_OSR_FACTOR: u32 = 64;

pub const DEFAULT_GAIN_Q23: u32 = 0x800000;

/* Max boost gain factor used for mixer control */
pub const MAX_BOOST_GAIN: u32 = 25599;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum tegra_dmic_ch_select {
    DMIC_CH_SELECT_LEFT,
    DMIC_CH_SELECT_RIGHT,
    DMIC_CH_SELECT_STEREO,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum tegra_dmic_osr {
    DMIC_OSR_64,
    DMIC_OSR_128,
    DMIC_OSR_256,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum tegra_dmic_lrsel {
    DMIC_LRSEL_LEFT,
    DMIC_LRSEL_RIGHT,
}

#[repr(C)]
pub struct tegra210_dmic {
    pub clk_dmic: *mut clk,
    pub regmap: *mut regmap,
    pub mono_to_stereo: ::core::ffi::c_uint,
    pub stereo_to_mono: ::core::ffi::c_uint,
    pub boost_gain: ::core::ffi::c_uint,
    pub ch_select: ::core::ffi::c_uint,
    pub osr_val: ::core::ffi::c_uint,
    pub lrsel: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
