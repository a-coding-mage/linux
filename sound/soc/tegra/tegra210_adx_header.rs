/* SPDX-License-Identifier: GPL-2.0-only
 * SPDX-FileCopyrightText: Copyright (c) 2021-2025 NVIDIA CORPORATION. All rights reserved.
 *
 * tegra210_adx.h - Definitions for Tegra210 ADX driver
 *
 */

use core::ffi::c_uint;

// C header dependency: <linux/types.h>
// External kernel types are expected to be supplied by surrounding bindings:
// regmap_config, snd_kcontrol_new, regmap.

/* Register offsets from TEGRA210_ADX*_BASE */
pub const TEGRA210_ADX_RX_STATUS: c_uint = 0x0c;
pub const TEGRA210_ADX_RX_INT_STATUS: c_uint = 0x10;
pub const TEGRA210_ADX_RX_INT_MASK: c_uint = 0x14;
pub const TEGRA210_ADX_RX_INT_SET: c_uint = 0x18;
pub const TEGRA210_ADX_RX_INT_CLEAR: c_uint = 0x1c;
pub const TEGRA210_ADX_RX_CIF_CTRL: c_uint = 0x20;
pub const TEGRA210_ADX_TX_STATUS: c_uint = 0x4c;
pub const TEGRA210_ADX_TX_INT_STATUS: c_uint = 0x50;
pub const TEGRA210_ADX_TX_INT_MASK: c_uint = 0x54;
pub const TEGRA210_ADX_TX_INT_SET: c_uint = 0x58;
pub const TEGRA210_ADX_TX_INT_CLEAR: c_uint = 0x5c;
pub const TEGRA210_ADX_TX1_CIF_CTRL: c_uint = 0x60;
pub const TEGRA210_ADX_TX2_CIF_CTRL: c_uint = 0x64;
pub const TEGRA210_ADX_TX3_CIF_CTRL: c_uint = 0x68;
pub const TEGRA210_ADX_TX4_CIF_CTRL: c_uint = 0x6c;
pub const TEGRA210_ADX_ENABLE: c_uint = 0x80;
pub const TEGRA210_ADX_SOFT_RESET: c_uint = 0x84;
pub const TEGRA210_ADX_CG: c_uint = 0x88;
pub const TEGRA210_ADX_STATUS: c_uint = 0x8c;
pub const TEGRA210_ADX_INT_STATUS: c_uint = 0x90;
pub const TEGRA210_ADX_CTRL: c_uint = 0xa4;
pub const TEGRA210_ADX_IN_BYTE_EN0: c_uint = 0xa8;
pub const TEGRA210_ADX_IN_BYTE_EN1: c_uint = 0xac;
pub const TEGRA210_ADX_CFG_RAM_CTRL: c_uint = 0xb8;
pub const TEGRA210_ADX_CFG_RAM_DATA: c_uint = 0xbc;

pub const TEGRA264_ADX_CYA: c_uint = 0xb8;
pub const TEGRA264_ADX_CFG_RAM_CTRL: c_uint = 0xc0;
pub const TEGRA264_ADX_CFG_RAM_DATA: c_uint = 0xc4;

/* Fields in TEGRA210_ADX_ENABLE */
pub const TEGRA210_ADX_ENABLE_SHIFT: c_uint = 0;

/* Fields in TEGRA210_ADX_CFG_RAM_CTRL */
pub const TEGRA210_ADX_CFG_RAM_CTRL_RAM_ADDR_SHIFT: c_uint = 0;

pub const TEGRA210_ADX_CFG_RAM_CTRL_RW_SHIFT: c_uint = 14;
pub const TEGRA210_ADX_CFG_RAM_CTRL_RW_WRITE: c_uint =
    1 << TEGRA210_ADX_CFG_RAM_CTRL_RW_SHIFT;

pub const TEGRA210_ADX_CFG_RAM_CTRL_ADDR_INIT_EN_SHIFT: c_uint = 13;
pub const TEGRA210_ADX_CFG_RAM_CTRL_ADDR_INIT_EN: c_uint =
    1 << TEGRA210_ADX_CFG_RAM_CTRL_ADDR_INIT_EN_SHIFT;

pub const TEGRA210_ADX_CFG_RAM_CTRL_SEQ_ACCESS_EN_SHIFT: c_uint = 12;
pub const TEGRA210_ADX_CFG_RAM_CTRL_SEQ_ACCESS_EN: c_uint =
    1 << TEGRA210_ADX_CFG_RAM_CTRL_SEQ_ACCESS_EN_SHIFT;

/* Fields in TEGRA210_ADX_SOFT_RESET */
pub const TEGRA210_ADX_SOFT_RESET_SOFT_RESET_SHIFT: c_uint = 0;
pub const TEGRA210_ADX_SOFT_RESET_SOFT_RESET_MASK: c_uint =
    1 << TEGRA210_ADX_SOFT_RESET_SOFT_RESET_SHIFT;
pub const TEGRA210_ADX_SOFT_RESET_SOFT_EN: c_uint =
    1 << TEGRA210_ADX_SOFT_RESET_SOFT_RESET_SHIFT;
pub const TEGRA210_ADX_SOFT_RESET_SOFT_DEFAULT: c_uint =
    0 << TEGRA210_ADX_SOFT_RESET_SOFT_RESET_SHIFT;

pub const TEGRA210_ADX_AUDIOCIF_CH_STRIDE: c_uint = 4;
pub const TEGRA_ADX_SLOTS_PER_WORD: c_uint = 4;
pub const TEGRA210_ADX_RAM_DEPTH: c_uint = 16;
pub const TEGRA210_ADX_MAP_STREAM_NUMBER_SHIFT: c_uint = 6;
pub const TEGRA210_ADX_MAP_WORD_NUMBER_SHIFT: c_uint = 2;
pub const TEGRA210_ADX_MAP_BYTE_NUMBER_SHIFT: c_uint = 0;
pub const TEGRA210_ADX_BYTE_MASK_COUNT: c_uint = 2;
pub const TEGRA210_ADX_MAX_CHANNEL: c_uint = 16;
pub const TEGRA210_ADX_CYA_OFFSET: c_uint = 0;

pub const TEGRA264_ADX_RAM_DEPTH: c_uint = 32;
pub const TEGRA264_ADX_BYTE_MASK_COUNT: c_uint = 4;
pub const TEGRA264_ADX_MAX_CHANNEL: c_uint = 32;
pub const TEGRA264_ADX_CYA_OFFSET: c_uint = 8;

pub const TEGRA_ADX_IN_DAI_ID: c_uint = 4;

#[repr(C)]
pub struct tegra210_adx_soc_data {
    pub regmap_conf: *const regmap_config,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub max_ch: c_uint,
    pub ram_depth: c_uint,
    pub byte_mask_size: c_uint,
    pub cya_offset: c_uint,
}

#[repr(C)]
pub struct tegra210_adx {
    pub regmap: *mut regmap,
    pub byte_mask: *mut c_uint,
    pub map: *mut u16,
    pub soc_data: *const tegra210_adx_soc_data,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
