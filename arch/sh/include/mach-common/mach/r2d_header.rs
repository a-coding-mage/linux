/* SPDX-License-Identifier: GPL-2.0 */

/*
 * linux/include/asm-sh/renesas_rts7751r2d.h
 *
 * Copyright (C) 2000  Atom Create Engineering Co., Ltd.
 *
 * Renesas Technology Sales RTS7751R2D support
 */

/* Board specific addresses. */

pub const PA_BCR: u32 = 0xa4000000; /* FPGA */
pub const PA_IRLMON: u32 = 0xa4000002; /* Interrupt Status control */
pub const PA_CFCTL: u32 = 0xa4000004; /* CF Timing control */
pub const PA_CFPOW: u32 = 0xa4000006; /* CF Power control */
pub const PA_DISPCTL: u32 = 0xa4000008; /* Display Timing control */
pub const PA_SDMPOW: u32 = 0xa400000a; /* SD Power control */
pub const PA_RTCCE: u32 = 0xa400000c; /* RTC(9701) Enable control */
pub const PA_PCICD: u32 = 0xa400000e; /* PCI Extension detect control */
pub const PA_VOYAGERRTS: u32 = 0xa4000020; /* VOYAGER Reset control */

pub const PA_R2D1_AXRST: u32 = 0xa4000022; /* AX_LAN Reset control */
pub const PA_R2D1_CFRST: u32 = 0xa4000024; /* CF Reset control */
pub const PA_R2D1_ADMRTS: u32 = 0xa4000026; /* SD Reset control */
pub const PA_R2D1_EXTRST: u32 = 0xa4000028; /* Extension Reset control */
pub const PA_R2D1_CFCDINTCLR: u32 = 0xa400002a; /* CF Insert Interrupt clear */

pub const PA_R2DPLUS_CFRST: u32 = 0xa4000022; /* CF Reset control */
pub const PA_R2DPLUS_ADMRTS: u32 = 0xa4000024; /* SD Reset control */
pub const PA_R2DPLUS_EXTRST: u32 = 0xa4000026; /* Extension Reset control */
pub const PA_R2DPLUS_CFCDINTCLR: u32 = 0xa4000028; /* CF Insert Interrupt clear */
pub const PA_R2DPLUS_KEYCTLCLR: u32 = 0xa400002a; /* Key Interrupt clear */

pub const PA_POWOFF: u32 = 0xa4000030; /* Board Power OFF control */
pub const PA_VERREG: u32 = 0xa4000032; /* FPGA Version Register */
pub const PA_INPORT: u32 = 0xa4000034; /* KEY Input Port control */
pub const PA_OUTPORT: u32 = 0xa4000036; /* LED control */
pub const PA_BVERREG: u32 = 0xa4000038; /* Board Revision Register */

pub const PA_AX88796L: u32 = 0xaa000400; /* AX88796L Area */
pub const PA_VOYAGER: u32 = 0xab000000; /* VOYAGER GX Area */
pub const PA_IDE_OFFSET: u32 = 0x1f0; /* CF IDE Offset */
pub const AX88796L_IO_BASE: u32 = 0x1000; /* AX88796L IO Base Address */

pub const IRLCNTR1: u32 = PA_BCR + 0; /* Interrupt Control Register1 */

pub const R2D_FPGA_IRQ_BASE: i32 = 100 + 16;

pub const IRQ_VOYAGER: i32 = R2D_FPGA_IRQ_BASE + 0;
pub const IRQ_EXT: i32 = R2D_FPGA_IRQ_BASE + 1;
pub const IRQ_TP: i32 = R2D_FPGA_IRQ_BASE + 2;
pub const IRQ_RTC_T: i32 = R2D_FPGA_IRQ_BASE + 3;
pub const IRQ_RTC_A: i32 = R2D_FPGA_IRQ_BASE + 4;
pub const IRQ_SDCARD: i32 = R2D_FPGA_IRQ_BASE + 5;
pub const IRQ_CF_CD: i32 = R2D_FPGA_IRQ_BASE + 6;
pub const IRQ_CF_IDE: i32 = R2D_FPGA_IRQ_BASE + 7;
pub const IRQ_AX88796: i32 = R2D_FPGA_IRQ_BASE + 8;
pub const IRQ_KEY: i32 = R2D_FPGA_IRQ_BASE + 9;
pub const IRQ_PCI_INTA: i32 = R2D_FPGA_IRQ_BASE + 10;
pub const IRQ_PCI_INTB: i32 = R2D_FPGA_IRQ_BASE + 11;
pub const IRQ_PCI_INTC: i32 = R2D_FPGA_IRQ_BASE + 12;
pub const IRQ_PCI_INTD: i32 = R2D_FPGA_IRQ_BASE + 13;

/* arch/sh/boards/renesas/rts7751r2d/irq.c */
extern "C" {
    pub fn init_rts7751r2d_IRQ();
    pub fn rts7751r2d_irq_demux(irq: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
