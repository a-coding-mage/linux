/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tegra20_ac97.h - Definitions for the Tegra20 AC97 controller driver
 *
 * Copyright (c) 2012 Lucas Stach <dev@lynxeye.de>
 *
 * Partly based on code copyright/by:
 *
 * Copyright (c) 2011,2012 Toradex Inc.
 */

/* Depends on declarations from "tegra_pcm.h". */

pub const TEGRA20_AC97_CTRL: u32 = 0x00;
pub const TEGRA20_AC97_CMD: u32 = 0x04;
pub const TEGRA20_AC97_STATUS1: u32 = 0x08;
/* ... */
pub const TEGRA20_AC97_FIFO1_SCR: u32 = 0x1c;
/* ... */
pub const TEGRA20_AC97_FIFO_TX1: u32 = 0x40;
pub const TEGRA20_AC97_FIFO_RX1: u32 = 0x80;

/* TEGRA20_AC97_CTRL */
pub const TEGRA20_AC97_CTRL_STM2_EN: u32 = 1 << 16;
pub const TEGRA20_AC97_CTRL_DOUBLE_SAMPLING_EN: u32 = 1 << 11;
pub const TEGRA20_AC97_CTRL_IO_CNTRL_EN: u32 = 1 << 10;
pub const TEGRA20_AC97_CTRL_HSET_DAC_EN: u32 = 1 << 9;
pub const TEGRA20_AC97_CTRL_LINE2_DAC_EN: u32 = 1 << 8;
pub const TEGRA20_AC97_CTRL_PCM_LFE_EN: u32 = 1 << 7;
pub const TEGRA20_AC97_CTRL_PCM_SUR_EN: u32 = 1 << 6;
pub const TEGRA20_AC97_CTRL_PCM_CEN_DAC_EN: u32 = 1 << 5;
pub const TEGRA20_AC97_CTRL_LINE1_DAC_EN: u32 = 1 << 4;
pub const TEGRA20_AC97_CTRL_PCM_DAC_EN: u32 = 1 << 3;
pub const TEGRA20_AC97_CTRL_COLD_RESET: u32 = 1 << 2;
pub const TEGRA20_AC97_CTRL_WARM_RESET: u32 = 1 << 1;
pub const TEGRA20_AC97_CTRL_STM_EN: u32 = 1 << 0;

/* TEGRA20_AC97_CMD */
pub const TEGRA20_AC97_CMD_CMD_ADDR_SHIFT: u32 = 24;
pub const TEGRA20_AC97_CMD_CMD_ADDR_MASK: u32 = 0xff << TEGRA20_AC97_CMD_CMD_ADDR_SHIFT;
pub const TEGRA20_AC97_CMD_CMD_DATA_SHIFT: u32 = 8;
pub const TEGRA20_AC97_CMD_CMD_DATA_MASK: u32 = 0xffff << TEGRA20_AC97_CMD_CMD_DATA_SHIFT;
pub const TEGRA20_AC97_CMD_CMD_ID_SHIFT: u32 = 2;
pub const TEGRA20_AC97_CMD_CMD_ID_MASK: u32 = 0x3 << TEGRA20_AC97_CMD_CMD_ID_SHIFT;
pub const TEGRA20_AC97_CMD_BUSY: u32 = 1 << 0;

/* TEGRA20_AC97_STATUS1 */
pub const TEGRA20_AC97_STATUS1_STA_ADDR1_SHIFT: u32 = 24;
pub const TEGRA20_AC97_STATUS1_STA_ADDR1_MASK: u32 =
    0xff << TEGRA20_AC97_STATUS1_STA_ADDR1_SHIFT;
pub const TEGRA20_AC97_STATUS1_STA_DATA1_SHIFT: u32 = 8;
pub const TEGRA20_AC97_STATUS1_STA_DATA1_MASK: u32 =
    0xffff << TEGRA20_AC97_STATUS1_STA_DATA1_SHIFT;
pub const TEGRA20_AC97_STATUS1_STA_VALID1: u32 = 1 << 2;
pub const TEGRA20_AC97_STATUS1_STANDBY1: u32 = 1 << 1;
pub const TEGRA20_AC97_STATUS1_CODEC1_RDY: u32 = 1 << 0;

/* TEGRA20_AC97_FIFO1_SCR */
pub const TEGRA20_AC97_FIFO_SCR_REC_MT_CNT_SHIFT: u32 = 27;
pub const TEGRA20_AC97_FIFO_SCR_REC_MT_CNT_MASK: u32 =
    0x1f << TEGRA20_AC97_FIFO_SCR_REC_MT_CNT_SHIFT;
pub const TEGRA20_AC97_FIFO_SCR_PB_MT_CNT_SHIFT: u32 = 22;
pub const TEGRA20_AC97_FIFO_SCR_PB_MT_CNT_MASK: u32 =
    0x1f << TEGRA20_AC97_FIFO_SCR_PB_MT_CNT_SHIFT;
pub const TEGRA20_AC97_FIFO_SCR_REC_OVERRUN_INT_STA: u32 = 1 << 19;
pub const TEGRA20_AC97_FIFO_SCR_PB_UNDERRUN_INT_STA: u32 = 1 << 18;
pub const TEGRA20_AC97_FIFO_SCR_REC_FORCE_MT: u32 = 1 << 17;
pub const TEGRA20_AC97_FIFO_SCR_PB_FORCE_MT: u32 = 1 << 16;
pub const TEGRA20_AC97_FIFO_SCR_REC_FULL_EN: u32 = 1 << 15;
pub const TEGRA20_AC97_FIFO_SCR_REC_3QRT_FULL_EN: u32 = 1 << 14;
pub const TEGRA20_AC97_FIFO_SCR_REC_QRT_FULL_EN: u32 = 1 << 13;
pub const TEGRA20_AC97_FIFO_SCR_REC_EMPTY_EN: u32 = 1 << 12;
pub const TEGRA20_AC97_FIFO_SCR_PB_NOT_FULL_EN: u32 = 1 << 11;
pub const TEGRA20_AC97_FIFO_SCR_PB_QRT_MT_EN: u32 = 1 << 10;
pub const TEGRA20_AC97_FIFO_SCR_PB_3QRT_MT_EN: u32 = 1 << 9;
pub const TEGRA20_AC97_FIFO_SCR_PB_EMPTY_MT_EN: u32 = 1 << 8;

#[repr(C)]
pub struct tegra20_ac97 {
    pub clk_ac97: *mut clk,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub playback_dma_data: snd_dmaengine_dai_dma_data,
    pub reset: *mut reset_control,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub sync_gpio: *mut gpio_desc,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
