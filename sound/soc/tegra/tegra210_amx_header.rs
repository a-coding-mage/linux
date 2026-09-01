/* SPDX-License-Identifier: GPL-2.0-only
 * SPDX-FileCopyrightText: Copyright (c) 2021-2025 NVIDIA CORPORATION. All rights reserved.
 *
 * tegra210_amx.h - Definitions for Tegra210 AMX driver
 */

/* Register offsets from TEGRA210_AMX*_BASE */
pub const TEGRA210_AMX_RX_STATUS: u32 = 0x0c;
pub const TEGRA210_AMX_RX_INT_STATUS: u32 = 0x10;
pub const TEGRA210_AMX_RX_INT_MASK: u32 = 0x14;
pub const TEGRA210_AMX_RX_INT_SET: u32 = 0x18;
pub const TEGRA210_AMX_RX_INT_CLEAR: u32 = 0x1c;
pub const TEGRA210_AMX_RX1_CIF_CTRL: u32 = 0x20;
pub const TEGRA210_AMX_RX2_CIF_CTRL: u32 = 0x24;
pub const TEGRA210_AMX_RX3_CIF_CTRL: u32 = 0x28;
pub const TEGRA210_AMX_RX4_CIF_CTRL: u32 = 0x2c;
pub const TEGRA210_AMX_TX_STATUS: u32 = 0x4c;
pub const TEGRA210_AMX_TX_INT_STATUS: u32 = 0x50;
pub const TEGRA210_AMX_TX_INT_MASK: u32 = 0x54;
pub const TEGRA210_AMX_TX_INT_SET: u32 = 0x58;
pub const TEGRA210_AMX_TX_INT_CLEAR: u32 = 0x5c;
pub const TEGRA210_AMX_TX_CIF_CTRL: u32 = 0x60;
pub const TEGRA210_AMX_ENABLE: u32 = 0x80;
pub const TEGRA210_AMX_SOFT_RESET: u32 = 0x84;
pub const TEGRA210_AMX_CG: u32 = 0x88;
pub const TEGRA210_AMX_STATUS: u32 = 0x8c;
pub const TEGRA210_AMX_INT_STATUS: u32 = 0x90;
pub const TEGRA210_AMX_CTRL: u32 = 0xa4;
pub const TEGRA210_AMX_OUT_BYTE_EN0: u32 = 0xa8;
pub const TEGRA210_AMX_CYA: u32 = 0xb0;
pub const TEGRA210_AMX_CFG_RAM_CTRL: u32 = 0xb8;
pub const TEGRA210_AMX_CFG_RAM_DATA: u32 = 0xbc;

pub const TEGRA194_AMX_RX1_FRAME_PERIOD: u32 = 0xc0;
pub const TEGRA194_AMX_RX4_FRAME_PERIOD: u32 = 0xcc;
pub const TEGRA194_AMX_RX4_LAST_FRAME_PERIOD: u32 = 0xdc;

pub const TEGRA264_AMX_STREAMS_AUTO_DISABLE: u32 = 0xb8;
pub const TEGRA264_AMX_CFG_RAM_CTRL: u32 = 0xc0;
pub const TEGRA264_AMX_CFG_RAM_DATA: u32 = 0xc4;
pub const TEGRA264_AMX_RX1_FRAME_PERIOD: u32 = 0xc8;
pub const TEGRA264_AMX_RX4_FRAME_PERIOD: u32 = 0xd4;
pub const TEGRA264_AMX_RX4_LAST_FRAME_PERIOD: u32 = 0xe4;

/* Fields in TEGRA210_AMX_ENABLE */
pub const TEGRA210_AMX_ENABLE_SHIFT: u32 = 0;

/* Fields in TEGRA210_AMX_CTRL */
pub const TEGRA210_AMX_CTRL_MSTR_RX_NUM_SHIFT: u32 = 14;
pub const TEGRA210_AMX_CTRL_MSTR_RX_NUM_MASK: u32 = 3 << TEGRA210_AMX_CTRL_MSTR_RX_NUM_SHIFT;

pub const TEGRA210_AMX_CTRL_RX_DEP_SHIFT: u32 = 12;
pub const TEGRA210_AMX_CTRL_RX_DEP_MASK: u32 = 3 << TEGRA210_AMX_CTRL_RX_DEP_SHIFT;

/* Fields in TEGRA210_AMX_CFG_RAM_CTRL */
pub const TEGRA210_AMX_CFG_RAM_CTRL_RW_SHIFT: u32 = 14;
pub const TEGRA210_AMX_CFG_RAM_CTRL_RW_WRITE: u32 = 1 << TEGRA210_AMX_CFG_RAM_CTRL_RW_SHIFT;

pub const TEGRA210_AMX_CFG_RAM_CTRL_ADDR_INIT_EN_SHIFT: u32 = 13;
pub const TEGRA210_AMX_CFG_RAM_CTRL_ADDR_INIT_EN: u32 =
    1 << TEGRA210_AMX_CFG_RAM_CTRL_ADDR_INIT_EN_SHIFT;

pub const TEGRA210_AMX_CFG_RAM_CTRL_SEQ_ACCESS_EN_SHIFT: u32 = 12;
pub const TEGRA210_AMX_CFG_RAM_CTRL_SEQ_ACCESS_EN: u32 =
    1 << TEGRA210_AMX_CFG_RAM_CTRL_SEQ_ACCESS_EN_SHIFT;

pub const TEGRA210_AMX_CFG_CTRL_RAM_ADDR_SHIFT: u32 = 0;

/* Fields in TEGRA210_AMX_SOFT_RESET */
pub const TEGRA210_AMX_SOFT_RESET_SOFT_EN: u32 = 1;
pub const TEGRA210_AMX_SOFT_RESET_SOFT_RESET_MASK: u32 = TEGRA210_AMX_SOFT_RESET_SOFT_EN;

pub const TEGRA210_AMX_AUDIOCIF_CH_STRIDE: u32 = 4;
pub const TEGRA_AMX_SLOTS_PER_WORD: u32 = 4;
pub const TEGRA210_AMX_RAM_DEPTH: u32 = 16;
pub const TEGRA210_AMX_MAP_STREAM_NUM_SHIFT: u32 = 6;
pub const TEGRA210_AMX_MAP_WORD_NUM_SHIFT: u32 = 2;
pub const TEGRA210_AMX_MAP_BYTE_NUM_SHIFT: u32 = 0;
pub const TEGRA210_AMX_BYTE_MASK_COUNT: u32 = 2;
pub const TEGRA210_AMX_MAX_CHANNEL: u32 = 16;
pub const TEGRA210_AMX_AUTO_DISABLE_OFFSET: u32 = 0;

pub const TEGRA264_AMX_RAM_DEPTH: u32 = 32;
pub const TEGRA264_AMX_BYTE_MASK_COUNT: u32 = 4;
pub const TEGRA264_AMX_MAX_CHANNEL: u32 = 32;
pub const TEGRA264_AMX_AUTO_DISABLE_OFFSET: u32 = 8;
pub const TEGRA_AMX_OUT_DAI_ID: u32 = 4;

pub const TEGRA210_AMX_WAIT_ON_ALL: u32 = 0;
pub const TEGRA210_AMX_WAIT_ON_ANY: u32 = 1;

#[repr(C)]
pub struct regmap_config {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct tegra210_amx_soc_data {
    pub regmap_conf: *const regmap_config,
    pub auto_disable: bool,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: u32,
    pub max_ch: u32,
    pub ram_depth: u32,
    pub byte_mask_size: u32,
    pub reg_offset: u32,
}

#[repr(C)]
pub struct tegra210_amx {
    pub soc_data: *const tegra210_amx_soc_data,
    pub byte_mask: *mut u32,
    pub map: *mut u16,
    pub regmap: *mut regmap,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
