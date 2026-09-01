// SPDX-License-Identifier: GPL-2.0-or-later
// linux/sound/soc/bcm/bcm63xx-i2s.h
// Copyright (c) 2020 Broadcom Corporation
// Author: Kevin-Ke Li <kevin-ke.li@broadcom.com>

use core::ffi::c_int;

pub const I2S_DESC_FIFO_DEPTH: u32 = 8;
pub const I2S_MISC_CFG: u32 = 0x003C;
pub const I2S_PAD_LVL_LOOP_DIS_MASK: u32 = 1 << 2;
pub const I2S_PAD_LVL_LOOP_DIS_ENABLE: u32 = I2S_PAD_LVL_LOOP_DIS_MASK;

pub const I2S_TX_ENABLE_MASK: u32 = 1 << 31;
pub const I2S_TX_ENABLE: u32 = I2S_TX_ENABLE_MASK;
pub const I2S_TX_OUT_R: u32 = 1 << 19;
pub const I2S_TX_DATA_ALIGNMENT: u32 = 1 << 2;
pub const I2S_TX_DATA_ENABLE: u32 = 1 << 1;
pub const I2S_TX_CLOCK_ENABLE: u32 = 1 << 0;

pub const I2S_TX_DESC_OFF_LEVEL_SHIFT: u32 = 12;
pub const I2S_TX_DESC_OFF_LEVEL_MASK: u32 = 0x0F << I2S_TX_DESC_OFF_LEVEL_SHIFT;
pub const I2S_TX_DESC_IFF_LEVEL_SHIFT: u32 = 8;
pub const I2S_TX_DESC_IFF_LEVEL_MASK: u32 = 0x0F << I2S_TX_DESC_IFF_LEVEL_SHIFT;
pub const I2S_TX_DESC_OFF_INTR_EN_MSK: u32 = 1 << 1;
pub const I2S_TX_DESC_OFF_INTR_EN: u32 = I2S_TX_DESC_OFF_INTR_EN_MSK;

pub const I2S_TX_CFG: u32 = 0x0000;
pub const I2S_TX_IRQ_CTL: u32 = 0x0004;
pub const I2S_TX_IRQ_EN: u32 = 0x0008;
pub const I2S_TX_IRQ_IFF_THLD: u32 = 0x000c;
pub const I2S_TX_IRQ_OFF_THLD: u32 = 0x0010;
pub const I2S_TX_DESC_IFF_ADDR: u32 = 0x0014;
pub const I2S_TX_DESC_IFF_LEN: u32 = 0x0018;
pub const I2S_TX_DESC_OFF_ADDR: u32 = 0x001C;
pub const I2S_TX_DESC_OFF_LEN: u32 = 0x0020;
pub const I2S_TX_CFG_2: u32 = 0x0024;
pub const I2S_TX_SLAVE_MODE_SHIFT: u32 = 13;
pub const I2S_TX_SLAVE_MODE_MASK: u32 = 1 << I2S_TX_SLAVE_MODE_SHIFT;
pub const I2S_TX_SLAVE_MODE: u32 = I2S_TX_SLAVE_MODE_MASK;
pub const I2S_TX_MASTER_MODE: u32 = 0;
pub const I2S_TX_INTR_MASK: u32 = 0x0F;

pub const I2S_RX_ENABLE_MASK: u32 = 1 << 31;
pub const I2S_RX_ENABLE: u32 = I2S_RX_ENABLE_MASK;
pub const I2S_RX_IN_R: u32 = 1 << 19;
pub const I2S_RX_DATA_ALIGNMENT: u32 = 1 << 2;
pub const I2S_RX_CLOCK_ENABLE: u32 = 1 << 0;

pub const I2S_RX_DESC_OFF_LEVEL_SHIFT: u32 = 12;
pub const I2S_RX_DESC_OFF_LEVEL_MASK: u32 = 0x0F << I2S_RX_DESC_OFF_LEVEL_SHIFT;
pub const I2S_RX_DESC_IFF_LEVEL_SHIFT: u32 = 8;
pub const I2S_RX_DESC_IFF_LEVEL_MASK: u32 = 0x0F << I2S_RX_DESC_IFF_LEVEL_SHIFT;
pub const I2S_RX_DESC_OFF_INTR_EN_MSK: u32 = 1 << 1;
pub const I2S_RX_DESC_OFF_INTR_EN: u32 = I2S_RX_DESC_OFF_INTR_EN_MSK;

pub const I2S_RX_CFG: u32 = 0x0040; /* 20c0 */
pub const I2S_RX_IRQ_CTL: u32 = 0x0044;
pub const I2S_RX_IRQ_EN: u32 = 0x0048;
pub const I2S_RX_IRQ_IFF_THLD: u32 = 0x004C;
pub const I2S_RX_IRQ_OFF_THLD: u32 = 0x0050;
pub const I2S_RX_DESC_IFF_ADDR: u32 = 0x0054;
pub const I2S_RX_DESC_IFF_LEN: u32 = 0x0058;
pub const I2S_RX_DESC_OFF_ADDR: u32 = 0x005C;
pub const I2S_RX_DESC_OFF_LEN: u32 = 0x0060;
pub const I2S_RX_CFG_2: u32 = 0x0064;
pub const I2S_RX_SLAVE_MODE_SHIFT: u32 = 13;
pub const I2S_RX_SLAVE_MODE_MASK: u32 = 1 << I2S_RX_SLAVE_MODE_SHIFT;
pub const I2S_RX_SLAVE_MODE: u32 = I2S_RX_SLAVE_MODE_MASK;
pub const I2S_RX_MASTER_MODE: u32 = 0;
pub const I2S_RX_INTR_MASK: u32 = 0x0F;

pub const I2S_REG_MAX: u32 = 0x007C;

#[repr(C)]
pub struct bcm_i2s_priv {
    pub dev: *mut device,
    pub regmap_i2s: *mut regmap,
    pub i2s_clk: *mut clk,
    pub play_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub play_dma_desc: *mut i2s_dma_desc,
    pub capture_dma_desc: *mut i2s_dma_desc,
}

unsafe extern "C" {
    pub fn bcm63xx_soc_platform_probe(
        pdev: *mut platform_device,
        i2s_priv: *mut bcm_i2s_priv,
    ) -> c_int;
    pub fn bcm63xx_soc_platform_remove(pdev: *mut platform_device) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
