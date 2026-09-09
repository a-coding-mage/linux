/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/dreamcast/dma.h
 *
 * Copyright (C) 2003 Paul Mundt
 */

/* Number of DMA channels */
pub const G2_NR_DMA_CHANNELS: u32 = 4;

/* Channels for cascading */
pub const PVR2_CASCADE_CHAN: u32 = 2;
pub const G2_CASCADE_CHAN: u32 = 3;

/* PVR2 DMA Registers */
pub const PVR2_DMA_BASE: u32 = 0xa05f6800;
pub const PVR2_DMA_ADDR: u32 = PVR2_DMA_BASE + 0;
pub const PVR2_DMA_COUNT: u32 = PVR2_DMA_BASE + 4;
pub const PVR2_DMA_MODE: u32 = PVR2_DMA_BASE + 8;
pub const PVR2_DMA_LMMODE0: u32 = PVR2_DMA_BASE + 132;
pub const PVR2_DMA_LMMODE1: u32 = PVR2_DMA_BASE + 136;

/* G2 DMA Register */
pub const G2_DMA_BASE: u32 = 0xa05f7800;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
