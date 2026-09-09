/* SPDX-License-Identifier: GPL-2.0 */

/*
 * include/asm-sh/se/se7343.h
 *
 * Copyright (C) 2003 Takashi Kusuda <kusuda-takashi@hitachi-ul.co.jp>
 *
 * SH-Mobile SolutionEngine 7343 support
 */
/* Dependency: linux/sh_intc.h */

/* Box specific addresses. */

/* Area 0 */
pub const PA_ROM: u32 = 0x0000_0000; /* EPROM */
pub const PA_ROM_SIZE: u32 = 0x0040_0000; /* EPROM size 4M byte(Actually 2MB) */
pub const PA_FROM: u32 = 0x0040_0000; /* Flash ROM */
pub const PA_FROM_SIZE: u32 = 0x0040_0000; /* Flash size 4M byte */
pub const PA_SRAM: u32 = 0x0080_0000; /* SRAM */
/* The C header repeats PA_FROM_SIZE here (SRAM size 4M byte). */

/* Area 1 */
pub const PA_EXT1: u32 = 0x0400_0000;
pub const PA_EXT1_SIZE: u32 = 0x0400_0000;
/* Area 2 */
pub const PA_EXT2: u32 = 0x0800_0000;
pub const PA_EXT2_SIZE: u32 = 0x0400_0000;
/* Area 3 */
pub const PA_SDRAM: u32 = 0x0c00_0000;
pub const PA_SDRAM_SIZE: u32 = 0x0400_0000;
/* Area 4 */
pub const PA_PCIC: u32 = 0x1000_0000; /* MR-SHPC-01 PCMCIA */
pub const PA_MRSHPC: u32 = 0xb03f_ffe0; /* MR-SHPC-01 PCMCIA controller */
pub const PA_MRSHPC_MW1: u32 = 0xb040_0000; /* MR-SHPC-01 memory window base */
pub const PA_MRSHPC_MW2: u32 = 0xb050_0000; /* MR-SHPC-01 attribute window base */
pub const PA_MRSHPC_IO: u32 = 0xb060_0000; /* MR-SHPC-01 I/O window base */
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
pub const PA_LED: u32 = 0xb0c0_0000; /* LED */
pub const LED_SHIFT: u32 = 0;
pub const PA_DIPSW: u32 = 0xb090_0000; /* Dip switch 31 */
/* Area 5 */
pub const PA_EXT5: u32 = 0x1400_0000;
pub const PA_EXT5_SIZE: u32 = 0x0400_0000;
/* Area 6 */
pub const PA_LCD1: u32 = 0xb800_0000;
pub const PA_LCD2: u32 = 0xb880_0000;

pub const PORT_PACR: u32 = 0xA405_0100;
pub const PORT_PBCR: u32 = 0xA405_0102;
pub const PORT_PCCR: u32 = 0xA405_0104;
pub const PORT_PDCR: u32 = 0xA405_0106;
pub const PORT_PECR: u32 = 0xA405_0108;
pub const PORT_PFCR: u32 = 0xA405_010A;
pub const PORT_PGCR: u32 = 0xA405_010C;
pub const PORT_PHCR: u32 = 0xA405_010E;
pub const PORT_PJCR: u32 = 0xA405_0110;
pub const PORT_PKCR: u32 = 0xA405_0112;
pub const PORT_PLCR: u32 = 0xA405_0114;
pub const PORT_PMCR: u32 = 0xA405_0116;
pub const PORT_PNCR: u32 = 0xA405_0118;
pub const PORT_PQCR: u32 = 0xA405_011A;
pub const PORT_PRCR: u32 = 0xA405_011C;
pub const PORT_PSCR: u32 = 0xA405_011E;
pub const PORT_PTCR: u32 = 0xA405_0140;
pub const PORT_PUCR: u32 = 0xA405_0142;
pub const PORT_PVCR: u32 = 0xA405_0144;
pub const PORT_PWCR: u32 = 0xA405_0146;
pub const PORT_PYCR: u32 = 0xA405_0148;
pub const PORT_PZCR: u32 = 0xA405_014A;
pub const PORT_PSELA: u32 = 0xA405_014C;
pub const PORT_PSELB: u32 = 0xA405_014E;
pub const PORT_PSELC: u32 = 0xA405_0150;
pub const PORT_PSELD: u32 = 0xA405_0152;
pub const PORT_PSELE: u32 = 0xA405_0154;
pub const PORT_HIZCRA: u32 = 0xA405_0156;
pub const PORT_HIZCRB: u32 = 0xA405_0158;
pub const PORT_HIZCRC: u32 = 0xA405_015C;
pub const PORT_DRVCR: u32 = 0xA405_0180;

pub const PORT_PADR: u32 = 0xA405_0120;
pub const PORT_PBDR: u32 = 0xA405_0122;
pub const PORT_PCDR: u32 = 0xA405_0124;
pub const PORT_PDDR: u32 = 0xA405_0126;
pub const PORT_PEDR: u32 = 0xA405_0128;
pub const PORT_PFDR: u32 = 0xA405_012A;
pub const PORT_PGDR: u32 = 0xA405_012C;
pub const PORT_PHDR: u32 = 0xA405_012E;
pub const PORT_PJDR: u32 = 0xA405_0130;
pub const PORT_PKDR: u32 = 0xA405_0132;
pub const PORT_PLDR: u32 = 0xA405_0134;
pub const PORT_PMDR: u32 = 0xA405_0136;
pub const PORT_PNDR: u32 = 0xA405_0138;
pub const PORT_PQDR: u32 = 0xA405_013A;
pub const PORT_PRDR: u32 = 0xA405_013C;
pub const PORT_PTDR: u32 = 0xA405_0160;
pub const PORT_PUDR: u32 = 0xA405_0162;
pub const PORT_PVDR: u32 = 0xA405_0164;
pub const PORT_PWDR: u32 = 0xA405_0166;
pub const PORT_PYDR: u32 = 0xA405_0168;

pub const FPGA_IN: u32 = 0xb140_0000;
pub const FPGA_OUT: u32 = 0xb140_0002;

/* evt2irq is supplied by the SH interrupt-controller dependency. */
macro_rules! IRQ0_IRQ { () => { evt2irq(0x600) }; }
macro_rules! IRQ1_IRQ { () => { evt2irq(0x620) }; }
macro_rules! IRQ4_IRQ { () => { evt2irq(0x680) }; }
macro_rules! IRQ5_IRQ { () => { evt2irq(0x6a0) }; }

pub const SE7343_FPGA_IRQ_MRSHPC0: u32 = 0;
pub const SE7343_FPGA_IRQ_MRSHPC1: u32 = 1;
pub const SE7343_FPGA_IRQ_MRSHPC2: u32 = 2;
pub const SE7343_FPGA_IRQ_MRSHPC3: u32 = 3;
pub const SE7343_FPGA_IRQ_SMC: u32 = 6; /* EXT_IRQ2 */
pub const SE7343_FPGA_IRQ_USB: u32 = 8;
pub const SE7343_FPGA_IRQ_UARTA: u32 = 10;
pub const SE7343_FPGA_IRQ_UARTB: u32 = 11;
pub const SE7343_FPGA_IRQ_NR: u32 = 12;

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

/* arch/sh/boards/se/7343/irq.c */
extern "C" {
    pub static mut se7343_irq_domain: *mut irq_domain;
    pub fn init_7343se_IRQ();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
