/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Author: Kevin Wells <kevin.wells@nxp.com>
 *
 * Copyright (C) 2008 NXP Semiconductors
 * Copyright 2023 Timesys Corporation <piotr.wojtaszczyk@timesys.com>
 */

/* Dependencies from the original C header:
 * <linux/bitfield.h>
 * <linux/types.h>
 * <linux/regmap.h>
 */

#[repr(C)]
pub struct lpc3xxx_i2s_info {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub lock: mutex, /* To serialize user-space access */
    pub regs: *mut regmap,
    pub streams_in_use: u32,
    pub clkrate: u32,
    pub freq: i32,
    pub playback_dma_config: snd_dmaengine_dai_dma_data,
    pub capture_dma_config: snd_dmaengine_dai_dma_data,
}

unsafe extern "C" {
    pub fn lpc3xxx_pcm_register(pdev: *mut platform_device) -> i32;
}

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn FIELD_PREP(mask: u32, val: u32) -> u32 {
    (val << mask.trailing_zeros()) & mask
}

/* I2S controller register offsets */
pub const LPC3XXX_REG_I2S_DAO: u32 = 0x00;
pub const LPC3XXX_REG_I2S_DAI: u32 = 0x04;
pub const LPC3XXX_REG_I2S_TX_FIFO: u32 = 0x08;
pub const LPC3XXX_REG_I2S_RX_FIFO: u32 = 0x0C;
pub const LPC3XXX_REG_I2S_STAT: u32 = 0x10;
pub const LPC3XXX_REG_I2S_DMA0: u32 = 0x14;
pub const LPC3XXX_REG_I2S_DMA1: u32 = 0x18;
pub const LPC3XXX_REG_I2S_IRQ: u32 = 0x1C;
pub const LPC3XXX_REG_I2S_TX_RATE: u32 = 0x20;
pub const LPC3XXX_REG_I2S_RX_RATE: u32 = 0x24;

/* i2s_daO i2s_dai register definitions */
pub const LPC3XXX_I2S_WW8: u32 = FIELD_PREP(0x3, 0); /* Word width is 8bit */
pub const LPC3XXX_I2S_WW16: u32 = FIELD_PREP(0x3, 1); /* Word width is 16bit */
pub const LPC3XXX_I2S_WW32: u32 = FIELD_PREP(0x3, 3); /* Word width is 32bit */
pub const LPC3XXX_I2S_MONO: u32 = BIT(2); /* Mono */
pub const LPC3XXX_I2S_STOP: u32 = BIT(3); /* Stop, diables the access to FIFO, mutes the channel */
pub const LPC3XXX_I2S_RESET: u32 = BIT(4); /* Reset the channel */
pub const LPC3XXX_I2S_WS_SEL: u32 = BIT(5); /* Channel Master(0) or slave(1) mode select */
pub const fn LPC3XXX_I2S_WS_HP(s: u32) -> u32 {
    FIELD_PREP(0x7FC0, s)
} /* Word select half period - 1 */
pub const LPC3XXX_I2S_MUTE: u32 = BIT(15); /* Mute the channel, Transmit channel only */

pub const LPC3XXX_I2S_WW32_HP: u32 = 0x1f; /* Word select half period for 32bit word width */
pub const LPC3XXX_I2S_WW16_HP: u32 = 0x0f; /* Word select half period for 16bit word width */
pub const LPC3XXX_I2S_WW8_HP: u32 = 0x7; /* Word select half period for 8bit word width */

/* i2s_stat register definitions */
pub const LPC3XXX_I2S_IRQ_STAT: u32 = BIT(0);
pub const LPC3XXX_I2S_DMA0_REQ: u32 = BIT(1);
pub const LPC3XXX_I2S_DMA1_REQ: u32 = BIT(2);

/* i2s_dma0 Configuration register definitions */
pub const LPC3XXX_I2S_DMA0_RX_EN: u32 = BIT(0); /* Enable RX DMA1 */
pub const LPC3XXX_I2S_DMA0_TX_EN: u32 = BIT(1); /* Enable TX DMA1 */
pub const fn LPC3XXX_I2S_DMA0_RX_DEPTH(s: u32) -> u32 {
    FIELD_PREP(0xF00, s)
} /* Set the DMA1 RX Request level */
pub const fn LPC3XXX_I2S_DMA0_TX_DEPTH(s: u32) -> u32 {
    FIELD_PREP(0xF0000, s)
} /* Set the DMA1 TX Request level */

/* i2s_dma1 Configuration register definitions */
pub const LPC3XXX_I2S_DMA1_RX_EN: u32 = BIT(0); /* Enable RX DMA1 */
pub const LPC3XXX_I2S_DMA1_TX_EN: u32 = BIT(1); /* Enable TX DMA1 */
pub const fn LPC3XXX_I2S_DMA1_RX_DEPTH(s: u32) -> u32 {
    FIELD_PREP(0x700, s)
} /* Set the DMA1 RX Request level */
pub const fn LPC3XXX_I2S_DMA1_TX_DEPTH(s: u32) -> u32 {
    FIELD_PREP(0x70000, s)
} /* Set the DMA1 TX Request level */

/* i2s_irq register definitions */
pub const LPC3XXX_I2S_RX_IRQ_EN: u32 = BIT(0); /* Enable RX IRQ */
pub const LPC3XXX_I2S_TX_IRQ_EN: u32 = BIT(1); /* Enable TX IRQ */
pub const fn LPC3XXX_I2S_IRQ_RX_DEPTH(s: u32) -> u32 {
    FIELD_PREP(0xFF00, s)
} /* valid values ar 0 to 7 */
pub const fn LPC3XXX_I2S_IRQ_TX_DEPTH(s: u32) -> u32 {
    FIELD_PREP(0xFF0000, s)
} /* valid values ar 0 to 7 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
