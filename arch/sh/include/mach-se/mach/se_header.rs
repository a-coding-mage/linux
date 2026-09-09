/* SPDX-License-Identifier: GPL-2.0 */

/*
 * linux/include/asm-sh/hitachi_se.h
 *
 * Copyright (C) 2000  Kazumoto Kojima
 *
 * Hitachi SolutionEngine support
 */

/* Box specific addresses. */
pub const PA_ROM: usize = 0x0000_0000; /* EPROM */
pub const PA_ROM_SIZE: usize = 0x0040_0000; /* EPROM size 4M byte */
pub const PA_FROM: usize = 0x0100_0000; /* EPROM */
pub const PA_FROM_SIZE: usize = 0x0040_0000; /* EPROM size 4M byte */
pub const PA_EXT1: usize = 0x0400_0000;
pub const PA_EXT1_SIZE: usize = 0x0400_0000;
pub const PA_EXT2: usize = 0x0800_0000;
pub const PA_EXT2_SIZE: usize = 0x0400_0000;
pub const PA_SDRAM: usize = 0x0c00_0000;
pub const PA_SDRAM_SIZE: usize = 0x0400_0000;

pub const PA_EXT4: usize = 0x1200_0000;
pub const PA_EXT4_SIZE: usize = 0x0200_0000;
pub const PA_EXT5: usize = 0x1400_0000;
pub const PA_EXT5_SIZE: usize = 0x0400_0000;
pub const PA_PCIC: usize = 0x1800_0000; /* MR-SHPC-01 PCMCIA */

pub const PA_83902: usize = 0xb000_0000; /* DP83902A */
pub const PA_83902_IF: usize = 0xb004_0000; /* DP83902A remote io port */
pub const PA_83902_RST: usize = 0xb008_0000; /* DP83902A reset port */

pub const PA_SUPERIO: usize = 0xb040_0000; /* SMC37C935A super io chip */
pub const PA_DIPSW0: usize = 0xb080_0000; /* Dip switch 5,6 */
pub const PA_DIPSW1: usize = 0xb080_0002; /* Dip switch 7,8 */
pub const PA_LED: usize = 0xb0c0_0000; /* LED */
#[cfg(CONFIG_CPU_SUBTYPE_SH7705)]
pub const PA_BCR: usize = 0xb0e0_0000;
#[cfg(not(CONFIG_CPU_SUBTYPE_SH7705))]
pub const PA_BCR: usize = 0xb140_0000; /* FPGA */

pub const PA_MRSHPC: usize = 0xb83f_ffe0; /* MR-SHPC-01 PCMCIA controller */
pub const PA_MRSHPC_MW1: usize = 0xb840_0000; /* MR-SHPC-01 memory window base */
pub const PA_MRSHPC_MW2: usize = 0xb850_0000; /* MR-SHPC-01 attribute window base */
pub const PA_MRSHPC_IO: usize = 0xb860_0000; /* MR-SHPC-01 I/O window base */
pub const MRSHPC_OPTION: usize = PA_MRSHPC + 6;
pub const MRSHPC_CSR: usize = PA_MRSHPC + 8;
pub const MRSHPC_ISR: usize = PA_MRSHPC + 10;
pub const MRSHPC_ICR: usize = PA_MRSHPC + 12;
pub const MRSHPC_CPWCR: usize = PA_MRSHPC + 14;
pub const MRSHPC_MW0CR1: usize = PA_MRSHPC + 16;
pub const MRSHPC_MW1CR1: usize = PA_MRSHPC + 18;
pub const MRSHPC_IOWCR1: usize = PA_MRSHPC + 20;
pub const MRSHPC_MW0CR2: usize = PA_MRSHPC + 22;
pub const MRSHPC_MW1CR2: usize = PA_MRSHPC + 24;
pub const MRSHPC_IOWCR2: usize = PA_MRSHPC + 26;
pub const MRSHPC_CDCR: usize = PA_MRSHPC + 28;
pub const MRSHPC_PCIC_INFO: usize = PA_MRSHPC + 30;

pub const BCR_ILCRA: usize = PA_BCR + 0;
pub const BCR_ILCRB: usize = PA_BCR + 2;
pub const BCR_ILCRC: usize = PA_BCR + 4;
pub const BCR_ILCRD: usize = PA_BCR + 6;
pub const BCR_ILCRE: usize = PA_BCR + 8;
pub const BCR_ILCRF: usize = PA_BCR + 10;
pub const BCR_ILCRG: usize = PA_BCR + 12;

#[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
pub const INTC_IRR0: usize = 0xa400_0004;
#[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
pub const INTC_IRR1: usize = 0xa400_0006;
#[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
pub const INTC_IRR2: usize = 0xa400_0008;
#[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
pub const INTC_ICR0: usize = 0xffff_fee0;
#[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
pub const INTC_ICR1: usize = 0xa400_0010;
#[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
pub const INTC_ICR2: usize = 0xa400_0012;
#[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
pub const INTC_INTER: usize = 0xa400_0014;
#[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
pub const INTC_IPRC: usize = 0xa400_0016;
#[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
pub const INTC_IPRD: usize = 0xa400_0018;
#[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
pub const INTC_IPRE: usize = 0xa400_001a;
#[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
pub const IRQ0_IRQ: usize = evt2irq(0x600);
#[cfg(CONFIG_CPU_SUBTYPE_SH7709)]
pub const IRQ1_IRQ: usize = evt2irq(0x620);

#[cfg(CONFIG_CPU_SUBTYPE_SH7705)]
pub const IRQ_STNIC: usize = evt2irq(0x380);
#[cfg(CONFIG_CPU_SUBTYPE_SH7705)]
pub const IRQ_CFCARD: usize = evt2irq(0x3c0);
#[cfg(not(CONFIG_CPU_SUBTYPE_SH7705))]
pub const IRQ_STNIC: usize = evt2irq(0x340);
#[cfg(not(CONFIG_CPU_SUBTYPE_SH7705))]
pub const IRQ_CFCARD: usize = evt2irq(0x2e0);

/* SH Ether support (SH7710/SH7712) */
/* Base address */
pub const SH_ETH0_BASE: usize = 0xA700_0000;
pub const SH_ETH1_BASE: usize = 0xA700_0400;
pub const SH_TSU_BASE: usize = 0xA700_0800;
/* PHY ID */
#[cfg(CONFIG_CPU_SUBTYPE_SH7710)]
pub const PHY_ID: usize = 0x00;
#[cfg(CONFIG_CPU_SUBTYPE_SH7712)]
pub const PHY_ID: usize = 0x01;
/* Ether IRQ */
pub const SH_ETH0_IRQ: usize = evt2irq(0xc00);
pub const SH_ETH1_IRQ: usize = evt2irq(0xc20);
pub const SH_TSU_IRQ: usize = evt2irq(0xc40);

extern "C" {
    pub fn init_se_IRQ();
}

/* __IO_PREFIX is se; the asm/io_generic.h include supplies related declarations. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
