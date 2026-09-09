/* SPDX-License-Identifier: GPL-2.0 */

/*
 * linux/include/asm-sh/se7722.h
 *
 * Copyright (C) 2007  Nobuhiro Iwamatsu
 *
 * Hitachi UL SolutionEngine 7722 Support.
 */
// C dependencies: linux/sh_intc.h, asm/addrspace.h, and asm/io_generic.h.

/* Box specific addresses. */
pub const SE_AREA0_WIDTH: u32 = 4; // Area0: 32bit
pub const PA_ROM: u32 = 0xa0000000; // EPROM
pub const PA_ROM_SIZE: u32 = 0x00200000; // EPROM size 2M byte
pub const PA_FROM: u32 = 0xa1000000; // Flash-ROM
pub const PA_FROM_SIZE: u32 = 0x01000000; // Flash-ROM size 16M byte
pub const PA_EXT1: u32 = 0xa4000000;
pub const PA_EXT1_SIZE: u32 = 0x04000000;
pub const PA_SDRAM: u32 = 0xac000000; // DDR-SDRAM(Area3) 64MB
pub const PA_SDRAM_SIZE: u32 = 0x04000000;

pub const PA_EXT4: u32 = 0xb0000000;
pub const PA_EXT4_SIZE: u32 = 0x04000000;

pub const PA_PERIPHERAL: u32 = 0xb0000000;

pub const PA_PCIC: u32 = PA_PERIPHERAL; // MR-SHPC-01 PCMCIA
pub const PA_MRSHPC: u32 = PA_PERIPHERAL + 0x003fffe0; // MR-SHPC-01 PCMCIA controller
pub const PA_MRSHPC_MW1: u32 = PA_PERIPHERAL + 0x00400000; // MR-SHPC-01 memory window base
pub const PA_MRSHPC_MW2: u32 = PA_PERIPHERAL + 0x00500000; // MR-SHPC-01 attribute window base
pub const PA_MRSHPC_IO: u32 = PA_PERIPHERAL + 0x00600000; // MR-SHPC-01 I/O window base
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

pub const PA_LED: u32 = PA_PERIPHERAL + 0x00800000; // 8bit LED
pub const PA_FPGA: u32 = PA_PERIPHERAL + 0x01800000; // FPGA base address

// PA_AREA6_IO is supplied by the address-space dependencies.
pub const PA_LAN: u32 = PA_AREA6_IO + 0; // SMC LAN91C111

/* GPIO */
pub const FPGA_IN: u32 = 0xb1840000;
pub const FPGA_OUT: u32 = 0xb1840004;

pub const PORT_PECR: u32 = 0xa4050108;
pub const PORT_PJCR: u32 = 0xa4050110;
pub const PORT_PSELD: u32 = 0xa4050154;
pub const PORT_PSELB: u32 = 0xa4050150;
pub const PORT_PSELC: u32 = 0xa4050152;
pub const PORT_PKCR: u32 = 0xa4050112;
pub const PORT_PHCR: u32 = 0xa405010e;
pub const PORT_PLCR: u32 = 0xa4050114;
pub const PORT_PMCR: u32 = 0xa4050116;
pub const PORT_PRCR: u32 = 0xa405011c;
pub const PORT_PXCR: u32 = 0xa4050148;
pub const PORT_PSELA: u32 = 0xa405014e;
pub const PORT_PYCR: u32 = 0xa405014a;
pub const PORT_PZCR: u32 = 0xa405014c;
pub const PORT_HIZCRA: u32 = 0xa4050158;
pub const PORT_HIZCRC: u32 = 0xa405015c;

/* IRQ */
pub const IRQ0_IRQ: i32 = evt2irq(0x600);
pub const IRQ1_IRQ: i32 = evt2irq(0x620);

pub const SE7722_FPGA_IRQ_USB: i32 = 0; // IRQ0
pub const SE7722_FPGA_IRQ_SMC: i32 = 1; // IRQ0
pub const SE7722_FPGA_IRQ_MRSHPC0: i32 = 2; // IRQ1
pub const SE7722_FPGA_IRQ_MRSHPC1: i32 = 3; // IRQ1
pub const SE7722_FPGA_IRQ_MRSHPC2: i32 = 4; // IRQ1
pub const SE7722_FPGA_IRQ_MRSHPC3: i32 = 5; // IRQ1
pub const SE7722_FPGA_IRQ_NR: i32 = 6;

pub struct irq_domain {
    _opaque: [u8; 0],
}

/* arch/sh/boards/se/7722/irq.c */
unsafe extern "C" {
    pub static mut se7722_irq_domain: *mut irq_domain;
    pub fn init_se7722_IRQ();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
