/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tegra210_sfc.h - Definitions for Tegra210 SFC driver
 *
 * Copyright (c) 2021-2023 NVIDIA CORPORATION.  All rights reserved.
 *
 */

/*
 * Header guard removed in Rust:
 * __TEGRA210_SFC_H__
 */

/*
 * SFC_RX registers are with respect to XBAR.
 * The data comes from XBAR to SFC.
 */
pub const TEGRA210_SFC_RX_STATUS: u32 = 0x0c;
pub const TEGRA210_SFC_RX_INT_STATUS: u32 = 0x10;
pub const TEGRA210_SFC_RX_INT_MASK: u32 = 0x14;
pub const TEGRA210_SFC_RX_INT_SET: u32 = 0x18;
pub const TEGRA210_SFC_RX_INT_CLEAR: u32 = 0x1c;
pub const TEGRA210_SFC_RX_CIF_CTRL: u32 = 0x20;
pub const TEGRA210_SFC_RX_FREQ: u32 = 0x24;

/*
 * SFC_TX registers are with respect to XBAR.
 * The data goes out of SFC.
 */
pub const TEGRA210_SFC_TX_STATUS: u32 = 0x4c;
pub const TEGRA210_SFC_TX_INT_STATUS: u32 = 0x50;
pub const TEGRA210_SFC_TX_INT_MASK: u32 = 0x54;
pub const TEGRA210_SFC_TX_INT_SET: u32 = 0x58;
pub const TEGRA210_SFC_TX_INT_CLEAR: u32 = 0x5c;
pub const TEGRA210_SFC_TX_CIF_CTRL: u32 = 0x60;
pub const TEGRA210_SFC_TX_FREQ: u32 = 0x64;

/* Register offsets from TEGRA210_SFC*_BASE */
pub const TEGRA210_SFC_ENABLE: u32 = 0x80;
pub const TEGRA210_SFC_SOFT_RESET: u32 = 0x84;
pub const TEGRA210_SFC_CG: u32 = 0x88;
pub const TEGRA210_SFC_STATUS: u32 = 0x8c;
pub const TEGRA210_SFC_INT_STATUS: u32 = 0x90;
pub const TEGRA210_SFC_COEF_RAM: u32 = 0xbc;
pub const TEGRA210_SFC_CFG_RAM_CTRL: u32 = 0xc0;
pub const TEGRA210_SFC_CFG_RAM_DATA: u32 = 0xc4;

/* Fields in TEGRA210_SFC_ENABLE */
pub const TEGRA210_SFC_EN_SHIFT: u32 = 0;
pub const TEGRA210_SFC_EN: u32 = 1 << TEGRA210_SFC_EN_SHIFT;

pub const TEGRA210_SFC_NUM_RATES: usize = 13;

/* Fields in TEGRA210_SFC_COEF_RAM */
pub const TEGRA210_SFC_COEF_RAM_EN: u32 = 1 << 0;

pub const TEGRA210_SFC_SOFT_RESET_EN: u32 = 1 << 0;

/* Coefficients */
pub const TEGRA210_SFC_COEF_RAM_DEPTH: u32 = 64;
pub const TEGRA210_SFC_RAM_CTRL_RW_WRITE: u32 = 1 << 14;
pub const TEGRA210_SFC_RAM_CTRL_ADDR_INIT_EN: u32 = 1 << 13;
pub const TEGRA210_SFC_RAM_CTRL_SEQ_ACCESS_EN: u32 = 1 << 12;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tegra210_sfc_path {
    SFC_RX_PATH = 0,
    SFC_TX_PATH = 1,
    SFC_PATHS = 2,
}

pub const SFC_PATHS: usize = tegra210_sfc_path::SFC_PATHS as usize;

/* External dependency from included kernel headers. */
pub enum regmap {}

#[repr(C)]
pub struct tegra210_sfc {
    pub mono_to_stereo: [u32; SFC_PATHS],
    pub stereo_to_mono: [u32; SFC_PATHS],
    pub srate_out: u32,
    pub srate_in: u32,
    pub regmap: *mut regmap,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
