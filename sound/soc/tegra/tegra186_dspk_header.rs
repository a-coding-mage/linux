/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tegra186_dspk.h - Definitions for Tegra186 DSPK driver
 *
 * Copyright (c) 2020 NVIDIA CORPORATION. All rights reserved.
 *
 */

/* Register offsets from DSPK BASE */
pub const TEGRA186_DSPK_RX_STATUS: u32 = 0x0c;
pub const TEGRA186_DSPK_RX_INT_STATUS: u32 = 0x10;
pub const TEGRA186_DSPK_RX_INT_MASK: u32 = 0x14;
pub const TEGRA186_DSPK_RX_INT_SET: u32 = 0x18;
pub const TEGRA186_DSPK_RX_INT_CLEAR: u32 = 0x1c;
pub const TEGRA186_DSPK_RX_CIF_CTRL: u32 = 0x20;
pub const TEGRA186_DSPK_ENABLE: u32 = 0x40;
pub const TEGRA186_DSPK_SOFT_RESET: u32 = 0x44;
pub const TEGRA186_DSPK_CG: u32 = 0x48;
pub const TEGRA186_DSPK_STATUS: u32 = 0x4c;
pub const TEGRA186_DSPK_INT_STATUS: u32 = 0x50;
pub const TEGRA186_DSPK_CORE_CTRL: u32 = 0x60;
pub const TEGRA186_DSPK_CODEC_CTRL: u32 = 0x64;

/* DSPK CORE CONTROL fields */
pub const CH_SEL_SHIFT: u32 = 8;
pub const TEGRA186_DSPK_CHANNEL_SELECT_MASK: u32 = 0x3 << CH_SEL_SHIFT;
pub const DSPK_OSR_SHIFT: u32 = 4;
pub const TEGRA186_DSPK_OSR_MASK: u32 = 0x3 << DSPK_OSR_SHIFT;
pub const LRSEL_POL_SHIFT: u32 = 0;
pub const TEGRA186_DSPK_CTRL_LRSEL_POLARITY_MASK: u32 = 0x1 << LRSEL_POL_SHIFT;
pub const TEGRA186_DSPK_RX_FIFO_DEPTH: u32 = 64;

pub const DSPK_OSR_FACTOR: u32 = 32;

/* DSPK interface clock ratio */
pub const DSPK_CLK_RATIO: u32 = 4;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum tegra_dspk_osr {
    DSPK_OSR_32,
    DSPK_OSR_64,
    DSPK_OSR_128,
    DSPK_OSR_256,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum tegra_dspk_ch_sel {
    DSPK_CH_SELECT_LEFT,
    DSPK_CH_SELECT_RIGHT,
    DSPK_CH_SELECT_STEREO,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum tegra_dspk_lrsel {
    DSPK_LRSEL_LEFT,
    DSPK_LRSEL_RIGHT,
}

#[repr(C)]
pub struct tegra186_dspk {
    pub rx_fifo_th: ::core::ffi::c_uint,
    pub osr_val: ::core::ffi::c_uint,
    pub lrsel: ::core::ffi::c_uint,
    pub ch_sel: ::core::ffi::c_uint,
    pub mono_to_stereo: ::core::ffi::c_uint,
    pub stereo_to_mono: ::core::ffi::c_uint,
    pub clk_dspk: *mut clk,
    pub regmap: *mut regmap,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
