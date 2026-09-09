/* SPDX-License-Identifier: GPL-2.0 */

/*
 * linux/include/asm-sh/hitachi_7751se.h
 *
 * Copyright (C) 2000  Kazumoto Kojima
 *
 * Hitachi SolutionEngine support
 *
 * Modified for 7751 Solution Engine by
 * Ian da Silva and Jeremy Siegel, 2001.
 */

/* Dependency: linux/sh_intc.h */

/* Box specific addresses. */

pub const PA_ROM: u32 = 0x0000_0000; /* EPROM */
pub const PA_ROM_SIZE: u32 = 0x0040_0000; /* EPROM size 4M byte */
pub const PA_FROM: u32 = 0x0100_0000; /* EPROM */
pub const PA_FROM_SIZE: u32 = 0x0040_0000; /* EPROM size 4M byte */
pub const PA_EXT1: u32 = 0x0400_0000;
pub const PA_EXT1_SIZE: u32 = 0x0400_0000;
pub const PA_EXT2: u32 = 0x0800_0000;
pub const PA_EXT2_SIZE: u32 = 0x0400_0000;
pub const PA_SDRAM: u32 = 0x0c00_0000;
pub const PA_SDRAM_SIZE: u32 = 0x0400_0000;

pub const PA_EXT4: u32 = 0x1200_0000;
pub const PA_EXT4_SIZE: u32 = 0x0200_0000;
pub const PA_EXT5: u32 = 0x1400_0000;
pub const PA_EXT5_SIZE: u32 = 0x0400_0000;
pub const PA_PCIC: u32 = 0x1800_0000; /* MR-SHPC-01 PCMCIA */

pub const PA_DIPSW0: u32 = 0xb900_0000; /* Dip switch 5,6 */
pub const PA_DIPSW1: u32 = 0xb900_0002; /* Dip switch 7,8 */
pub const PA_LED: u32 = 0xba00_0000; /* LED */
pub const PA_BCR: u32 = 0xbb00_0000; /* FPGA on the MS7751SE01 */

pub const PA_MRSHPC: u32 = 0xb83f_ffe0; /* MR-SHPC-01 PCMCIA controller */
pub const PA_MRSHPC_MW1: u32 = 0xb840_0000; /* MR-SHPC-01 memory window base */
pub const PA_MRSHPC_MW2: u32 = 0xb850_0000; /* MR-SHPC-01 attribute window base */
pub const PA_MRSHPC_IO: u32 = 0xb860_0000; /* MR-SHPC-01 I/O window base */
pub const MRSHPC_MODE: u32 = PA_MRSHPC + 4;
pub const MRSHPC_OPTION: u32 = PA_MRSHPC + 6;
pub const MRSHPC_CSR: u32 = PA_MRSHPC + 8;
pub const MRSHPC_ISR: u32 = PA_MRSHPC + 10;
pub const MRSHPC_ICR: u32 = PA_MRSHPC + 12;
pub const MRSHPC_CPWCR: u32 = PA_MRSHPC + 14;
pub const MRSHPC_MW0CR1: u32 = PA_MRSHPC + 16;
pub const MRSHPC_MW1CR1: u32 = PA_MRSHPC + 18;
pub const MRSHPC_IOWCR1: u32 = PA_MRSHPC + 20;
pub const MRSHPC_MW0CR2: u32 = PA_MRSHPC + 22;
pub const MRSHPC_MW1CR2: u32 = PA_MRSHPC + 24;
pub const MRSHPC_IOWCR2: u32 = PA_MRSHPC + 26;
pub const MRSHPC_CDCR: u32 = PA_MRSHPC + 28;
pub const MRSHPC_PCIC_INFO: u32 = PA_MRSHPC + 30;

pub const BCR_ILCRA: u32 = PA_BCR + 0;
pub const BCR_ILCRB: u32 = PA_BCR + 2;
pub const BCR_ILCRC: u32 = PA_BCR + 4;
pub const BCR_ILCRD: u32 = PA_BCR + 6;
pub const BCR_ILCRE: u32 = PA_BCR + 8;
pub const BCR_ILCRF: u32 = PA_BCR + 10;
pub const BCR_ILCRG: u32 = PA_BCR + 12;

/* Requires the externally supplied evt2irq mapping. */
pub const IRQ_79C973: _ = evt2irq(0x3a0);

unsafe extern "C" {
    pub fn init_7751se_IRQ();
}

/* __IO_PREFIX sh7751se; declarations from asm/io_generic.h are external. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
