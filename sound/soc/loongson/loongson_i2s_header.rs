/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ALSA I2S interface for the Loongson platform
 *
 * Copyright (C) 2023 Loongson Technology Corporation Limited
 * Author: Yingkun Meng <mengyingkun@loongson.cn>
 */

/* C header dependencies:
 * #include <linux/regmap.h>
 * #include <sound/dmaengine_pcm.h>
 */

/* I2S Common Registers */
pub const LS_I2S_VER: u32 = 0x00; /* I2S Version */
pub const LS_I2S_CFG: u32 = 0x04; /* I2S Config */
pub const LS_I2S_CTRL: u32 = 0x08; /* I2S Control */
pub const LS_I2S_RX_DATA: u32 = 0x0C; /* I2S DMA RX Address */
pub const LS_I2S_TX_DATA: u32 = 0x10; /* I2S DMA TX Address */

/* 2K2000 I2S Specify Registers */
pub const LS_I2S_CFG1: u32 = 0x14; /* I2S Config1 */

/* 7A2000 I2S Specify Registers */
pub const LS_I2S_TX_ORDER: u32 = 0x100; /* TX DMA Order */
pub const LS_I2S_RX_ORDER: u32 = 0x110; /* RX DMA Order */

/* Loongson I2S Control Register */
pub const I2S_CTRL_MCLK_READY: u32 = 1u32 << 16; /* MCLK ready */
pub const I2S_CTRL_MASTER: u32 = 1u32 << 15; /* Master mode */
pub const I2S_CTRL_MSB: u32 = 1u32 << 14; /* MSB bit order */
pub const I2S_CTRL_RX_EN: u32 = 1u32 << 13; /* RX enable */
pub const I2S_CTRL_TX_EN: u32 = 1u32 << 12; /* TX enable */
pub const I2S_CTRL_RX_DMA_EN: u32 = 1u32 << 11; /* DMA RX enable */
pub const I2S_CTRL_CLK_READY: u32 = 1u32 << 8; /* BCLK ready */
pub const I2S_CTRL_TX_DMA_EN: u32 = 1u32 << 7; /* DMA TX enable */
pub const I2S_CTRL_RESET: u32 = 1u32 << 4; /* Controller soft reset */
pub const I2S_CTRL_MCLK_EN: u32 = 1u32 << 3; /* Enable MCLK */
pub const I2S_CTRL_RX_INT_EN: u32 = 1u32 << 1; /* RX interrupt enable */
pub const I2S_CTRL_TX_INT_EN: u32 = 1u32 << 0; /* TX interrupt enable */

pub const LS_I2S_DRVNAME: &str = "loongson-i2s";

#[repr(C)]
pub struct loongson_idma_data {
    pub dev_addr: dma_addr_t, /* device physical address for DMA */
    pub order_addr: *mut core::ffi::c_void, /* DMA order register */
    pub irq: core::ffi::c_int, /* DMA irq */
}

#[repr(C)]
pub union loongson_i2s_playback {
    pub playback_dma_data: core::mem::ManuallyDrop<snd_dmaengine_dai_dma_data>,
    pub tx_dma_data: core::mem::ManuallyDrop<loongson_idma_data>,
}

#[repr(C)]
pub union loongson_i2s_capture {
    pub capture_dma_data: core::mem::ManuallyDrop<snd_dmaengine_dai_dma_data>,
    pub rx_dma_data: core::mem::ManuallyDrop<loongson_idma_data>,
}

#[repr(C)]
pub struct loongson_i2s {
    pub dev: *mut device,
    pub playback: loongson_i2s_playback,
    pub capture: loongson_i2s_capture,
    pub regmap: *mut regmap,
    pub reg_base: *mut core::ffi::c_void,
    pub rev_id: u32,
    pub clk_rate: u32,
    pub sysclk: u32,
}

unsafe extern "C" {
    pub static loongson_i2s_regmap_config: regmap_config;
    pub static loongson_i2s_pm: dev_pm_ops;
    pub static mut loongson_i2s_dai: snd_soc_dai_driver;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
