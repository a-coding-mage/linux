/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This header provides macros for X1830 DMA bindings.
 *
 * Copyright (c) 2019 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
 */

/*
 * Request type numbers for the X1830 DMA controller (written to the DRTn
 * register for the channel).
 */
pub const X1830_DMA_I2S0_TX: u32 = 0x6;
pub const X1830_DMA_I2S0_RX: u32 = 0x7;
pub const X1830_DMA_AUTO: u32 = 0x8;
pub const X1830_DMA_SADC_RX: u32 = 0x9;
pub const X1830_DMA_UART1_TX: u32 = 0x12;
pub const X1830_DMA_UART1_RX: u32 = 0x13;
pub const X1830_DMA_UART0_TX: u32 = 0x14;
pub const X1830_DMA_UART0_RX: u32 = 0x15;
pub const X1830_DMA_SSI0_TX: u32 = 0x16;
pub const X1830_DMA_SSI0_RX: u32 = 0x17;
pub const X1830_DMA_SSI1_TX: u32 = 0x18;
pub const X1830_DMA_SSI1_RX: u32 = 0x19;
pub const X1830_DMA_MSC0_TX: u32 = 0x1a;
pub const X1830_DMA_MSC0_RX: u32 = 0x1b;
pub const X1830_DMA_MSC1_TX: u32 = 0x1c;
pub const X1830_DMA_MSC1_RX: u32 = 0x1d;
pub const X1830_DMA_DMIC_RX: u32 = 0x21;
pub const X1830_DMA_SMB0_TX: u32 = 0x24;
pub const X1830_DMA_SMB0_RX: u32 = 0x25;
pub const X1830_DMA_SMB1_TX: u32 = 0x26;
pub const X1830_DMA_SMB1_RX: u32 = 0x27;
pub const X1830_DMA_DES_TX: u32 = 0x2e;
pub const X1830_DMA_DES_RX: u32 = 0x2f;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
