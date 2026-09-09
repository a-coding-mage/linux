/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/mach-rpc/include/mach/isa-dma.h
 *
 *  Copyright (C) 1997 Russell King
 */

pub const MAX_DMA_CHANNELS: i32 = 8;

pub const DMA_0: i32 = 0;
pub const DMA_1: i32 = 1;
pub const DMA_2: i32 = 2;
pub const DMA_3: i32 = 3;
pub const DMA_S0: i32 = 4;
pub const DMA_S1: i32 = 5;
pub const DMA_VIRTUAL_FLOPPY: i32 = 6;
pub const DMA_VIRTUAL_SOUND: i32 = 7;

pub const DMA_FLOPPY: i32 = DMA_VIRTUAL_FLOPPY;

pub const IOMD_DMA_BOUNDARY: usize = PAGE_SIZE - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
