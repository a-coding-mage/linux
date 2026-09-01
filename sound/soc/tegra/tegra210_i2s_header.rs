/* SPDX-License-Identifier: GPL-2.0-only
 * SPDX-FileCopyrightText: Copyright (c) 2020-2025 NVIDIA CORPORATION & AFFILIATES.
 * All rights reserved.
 *
 * tegra210_i2s.h - Definitions for Tegra210 I2S driver
 *
 */

/* Register offsets from I2S*_BASE */
pub const TEGRA210_I2S_RX_ENABLE: u32 = 0x0;
pub const TEGRA210_I2S_RX_SOFT_RESET: u32 = 0x4;
pub const TEGRA210_I2S_RX_STATUS: u32 = 0x0c;
pub const TEGRA210_I2S_RX_INT_STATUS: u32 = 0x10;
pub const TEGRA210_I2S_RX_INT_MASK: u32 = 0x14;
pub const TEGRA210_I2S_RX_INT_SET: u32 = 0x18;
pub const TEGRA210_I2S_RX_INT_CLEAR: u32 = 0x1c;
pub const TEGRA210_I2S_RX_CIF_CTRL: u32 = 0x20;
pub const TEGRA210_I2S_RX_CTRL: u32 = 0x24;
pub const TEGRA210_I2S_RX_SLOT_CTRL: u32 = 0x28;
pub const TEGRA210_I2S_RX_CLK_TRIM: u32 = 0x2c;
pub const TEGRA210_I2S_RX_CYA: u32 = 0x30;
pub const TEGRA210_I2S_RX_CIF_FIFO_STATUS: u32 = 0x34;
pub const TEGRA210_I2S_TX_ENABLE: u32 = 0x40;
pub const TEGRA210_I2S_TX_SOFT_RESET: u32 = 0x44;
pub const TEGRA210_I2S_TX_STATUS: u32 = 0x4c;
pub const TEGRA210_I2S_TX_INT_STATUS: u32 = 0x50;
pub const TEGRA210_I2S_TX_INT_MASK: u32 = 0x54;
pub const TEGRA210_I2S_TX_INT_SET: u32 = 0x58;
pub const TEGRA210_I2S_TX_INT_CLEAR: u32 = 0x5c;
pub const TEGRA210_I2S_TX_CIF_CTRL: u32 = 0x60;
pub const TEGRA210_I2S_TX_CTRL: u32 = 0x64;
pub const TEGRA210_I2S_TX_SLOT_CTRL: u32 = 0x68;
pub const TEGRA210_I2S_TX_CLK_TRIM: u32 = 0x6c;
pub const TEGRA210_I2S_TX_CYA: u32 = 0x70;
pub const TEGRA210_I2S_TX_CIF_FIFO_STATUS: u32 = 0x74;
pub const TEGRA210_I2S_ENABLE: u32 = 0x80;
pub const TEGRA210_I2S_SOFT_RESET: u32 = 0x84;
pub const TEGRA210_I2S_CG: u32 = 0x88;
pub const TEGRA210_I2S_STATUS: u32 = 0x8c;
pub const TEGRA210_I2S_INT_STATUS: u32 = 0x90;
pub const TEGRA210_I2S_CTRL: u32 = 0xa0;
pub const TEGRA210_I2S_TIMING: u32 = 0xa4;
pub const TEGRA210_I2S_SLOT_CTRL: u32 = 0xa8;
pub const TEGRA210_I2S_CLK_TRIM: u32 = 0xac;
pub const TEGRA210_I2S_CYA: u32 = 0xb0;

/* T264 specific registers */
pub const TEGRA264_I2S_RX_FIFO_WR_ACCESS_MODE: u32 = 0x30;
pub const TEGRA264_I2S_RX_CYA: u32 = 0x3c;
pub const TEGRA264_I2S_RX_CIF_FIFO_STATUS: u32 = 0x40;
pub const TEGRA264_I2S_TX_ENABLE: u32 = 0x80;
pub const TEGRA264_I2S_TX_SOFT_RESET: u32 = 0x84;
pub const TEGRA264_I2S_TX_STATUS: u32 = 0x8c;
pub const TEGRA264_I2S_TX_INT_STATUS: u32 = 0x90;
pub const TEGRA264_I2S_TX_INT_MASK: u32 = 0x94;
pub const TEGRA264_I2S_TX_CIF_CTRL: u32 = 0xa0;
pub const TEGRA264_I2S_TX_FIFO_RD_ACCESS_MODE: u32 = 0xb0;
pub const TEGRA264_I2S_TX_FIFO_RD_DATA: u32 = 0xb4;
pub const TEGRA264_I2S_TX_FIFO_THRESHOLD: u32 = 0xb8;
pub const TEGRA264_I2S_TX_CYA: u32 = 0xbc;
pub const TEGRA264_I2S_TX_CIF_FIFO_STATUS: u32 = 0xc0;
pub const TEGRA264_I2S_ENABLE: u32 = 0x100;
pub const TEGRA264_I2S_CG: u32 = 0x108;
pub const TEGRA264_I2S_STATUS: u32 = 0x10c;
pub const TEGRA264_I2S_INT_STATUS: u32 = 0x110;
pub const TEGRA264_I2S_INT_SET: u32 = 0x114;
pub const TEGRA264_I2S_INT_MASK: u32 = 0x11c;
pub const TEGRA264_I2S_CTRL: u32 = 0x12c;
pub const TEGRA264_I2S_TIMING: u32 = 0x130;
pub const TEGRA264_I2S_CYA: u32 = 0x13c;
pub const TEGRA264_I2S_PIO_MODE_ENABLE: u32 = 0x140;
pub const TEGRA264_I2S_PAD_MACRO_STATUS: u32 = 0x144;

/* Bit fields, shifts and masks */
pub const I2S_DATA_SHIFT: u32 = 8;
pub const I2S_CTRL_DATA_OFFSET_MASK: u32 = 0x7ff << I2S_DATA_SHIFT;
pub const TEGRA264_I2S_FSYNC_WIDTH_SHIFT: u32 = 23;
pub const TEGRA264_I2S_CTRL_FSYNC_WIDTH_MASK: u32 = 0x1ff << TEGRA264_I2S_FSYNC_WIDTH_SHIFT;

pub const I2S_EN_SHIFT: u32 = 0;
pub const I2S_EN_MASK: u32 = 1 << I2S_EN_SHIFT;
pub const I2S_EN: u32 = 1 << I2S_EN_SHIFT;

pub const I2S_FSYNC_WIDTH_SHIFT: u32 = 24;
pub const I2S_CTRL_FSYNC_WIDTH_MASK: u32 = 0xff << I2S_FSYNC_WIDTH_SHIFT;

pub const I2S_POS_EDGE: u32 = 0;
pub const I2S_NEG_EDGE: u32 = 1;
pub const I2S_EDGE_SHIFT: u32 = 20;
pub const I2S_CTRL_EDGE_CTRL_MASK: u32 = 1 << I2S_EDGE_SHIFT;
pub const I2S_CTRL_EDGE_CTRL_POS_EDGE: u32 = I2S_POS_EDGE << I2S_EDGE_SHIFT;
pub const I2S_CTRL_EDGE_CTRL_NEG_EDGE: u32 = I2S_NEG_EDGE << I2S_EDGE_SHIFT;

pub const I2S_FMT_LRCK: u32 = 0;
pub const I2S_FMT_FSYNC: u32 = 1;
pub const I2S_FMT_SHIFT: u32 = 12;
pub const I2S_CTRL_FRAME_FMT_MASK: u32 = 7 << I2S_FMT_SHIFT;
pub const I2S_CTRL_FRAME_FMT_LRCK_MODE: u32 = I2S_FMT_LRCK << I2S_FMT_SHIFT;
pub const I2S_CTRL_FRAME_FMT_FSYNC_MODE: u32 = I2S_FMT_FSYNC << I2S_FMT_SHIFT;

pub const I2S_CTRL_MASTER_EN_SHIFT: u32 = 10;
pub const I2S_CTRL_MASTER_EN_MASK: u32 = 1 << I2S_CTRL_MASTER_EN_SHIFT;
pub const I2S_CTRL_MASTER_EN: u32 = 1 << I2S_CTRL_MASTER_EN_SHIFT;

pub const I2S_CTRL_LRCK_POL_SHIFT: u32 = 9;
pub const I2S_CTRL_LRCK_POL_MASK: u32 = 1 << I2S_CTRL_LRCK_POL_SHIFT;
pub const I2S_CTRL_LRCK_POL_LOW: u32 = 0 << I2S_CTRL_LRCK_POL_SHIFT;
pub const I2S_CTRL_LRCK_POL_HIGH: u32 = 1 << I2S_CTRL_LRCK_POL_SHIFT;

pub const I2S_CTRL_LPBK_SHIFT: u32 = 8;
pub const I2S_CTRL_LPBK_MASK: u32 = 1 << I2S_CTRL_LPBK_SHIFT;
pub const I2S_CTRL_LPBK_EN: u32 = 1 << I2S_CTRL_LPBK_SHIFT;

pub const I2S_BITS_8: u32 = 1;
pub const I2S_BITS_16: u32 = 3;
pub const I2S_BITS_24: u32 = 5;
pub const I2S_BITS_32: u32 = 7;
pub const I2S_CTRL_BIT_SIZE_MASK: u32 = 0x7;

pub const I2S_TIMING_CH_BIT_CNT_MASK: u32 = 0x7ff;
pub const I2S_TIMING_CH_BIT_CNT_SHIFT: u32 = 0;

pub const I2S_SOFT_RESET_SHIFT: u32 = 0;
pub const I2S_SOFT_RESET_MASK: u32 = 1 << I2S_SOFT_RESET_SHIFT;
pub const I2S_SOFT_RESET_EN: u32 = 1 << I2S_SOFT_RESET_SHIFT;

pub const I2S_RX_FIFO_DEPTH: u32 = 64;
pub const DEFAULT_I2S_RX_FIFO_THRESHOLD: u32 = 3;

pub const DEFAULT_I2S_SLOT_MASK: u32 = 0xffff;
pub const TEGRA210_I2S_TX_OFFSET: u32 = 0;
pub const TEGRA210_I2S_CTRL_OFFSET: u32 = 0;
pub const TEGRA210_I2S_MAX_CHANNEL: u32 = 16;

pub const TEGRA264_DEFAULT_I2S_SLOT_MASK: u32 = 0xffffffff;
pub const TEGRA264_I2S_TX_OFFSET: u32 = 0x40;
pub const TEGRA264_I2S_CTRL_OFFSET: u32 = 0x8c;
pub const TEGRA264_I2S_MAX_CHANNEL: u32 = 32;

#[repr(C)]
pub enum tegra210_i2s_path {
    I2S_RX_PATH = 0,
    I2S_TX_PATH = 1,
    I2S_PATHS = 2,
}

pub const I2S_PATHS: usize = tegra210_i2s_path::I2S_PATHS as usize;

#[repr(C)]
pub struct tegra_i2s_soc_data {
    pub regmap_conf: *const regmap_config,
    pub i2s_cmpnt: *const snd_soc_component_driver,
    pub max_ch: u32,
    pub enable_reg: u32,
    pub tx_offset: u32,
    pub i2s_ctrl_offset: u32,
    pub fsync_width_mask: u32,
    pub fsync_width_shift: u32,
    pub slot_mask: u32,
}

#[repr(C)]
pub struct tegra210_i2s {
    pub soc_data: *const tegra_i2s_soc_data,
    pub clk_i2s: *mut clk,
    pub clk_sync_input: *mut clk,
    pub regmap: *mut regmap,
    pub client_sample_format: i32,
    pub client_channels: u32,
    pub stereo_to_mono: [u32; I2S_PATHS],
    pub mono_to_stereo: [u32; I2S_PATHS],
    pub dai_fmt: u32,
    pub fsync_width: u32,
    pub bclk_ratio: u32,
    pub tx_mask: u32,
    pub rx_mask: u32,
    pub rx_fifo_th: u32,
    pub loopback: bool,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
