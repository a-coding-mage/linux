/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/asm-sh/se7724.h
 *
 * Copyright (C) 2009 Renesas Solutions Corp.
 *
 * Kuninori Morimoto <morimoto.kuninori@renesas.com>
 *
 * Hitachi UL SolutionEngine 7724 Support.
 *
 * Based on se7722.h
 * Copyright (C) 2007  Nobuhiro Iwamatsu
 *
 * C dependencies: linux/sh_intc.h, asm/addrspace.h, and asm/io_generic.h.
 */

/* SH Eth */
pub const SH_ETH_ADDR: u32 = 0xA4600000;
pub const SH_ETH_MAHR: u32 = SH_ETH_ADDR + 0x1C0;
pub const SH_ETH_MALR: u32 = SH_ETH_ADDR + 0x1C8;

pub const PA_LED: u32 = 0xba203000; /* 8bit LED */
pub const IRQ_MODE: u32 = 0xba200010;
pub const IRQ0_SR: u32 = 0xba200014;
pub const IRQ1_SR: u32 = 0xba200018;
pub const IRQ2_SR: u32 = 0xba20001c;
pub const IRQ0_MR: u32 = 0xba200020;
pub const IRQ1_MR: u32 = 0xba200024;
pub const IRQ2_MR: u32 = 0xba200028;

/* IRQ */
/* evt2irq is supplied by linux/sh_intc.h. */
pub const IRQ0_IRQ: u32 = evt2irq(0x600);
pub const IRQ1_IRQ: u32 = evt2irq(0x620);
pub const IRQ2_IRQ: u32 = evt2irq(0x640);

/* Bits in IRQ012 registers */
pub const SE7724_FPGA_IRQ_BASE: u32 = 220 + 16;

/* IRQ0 */
pub const IRQ0_BASE: u32 = SE7724_FPGA_IRQ_BASE;
pub const IRQ0_KEY: u32 = IRQ0_BASE + 12;
pub const IRQ0_RMII: u32 = IRQ0_BASE + 13;
pub const IRQ0_SMC: u32 = IRQ0_BASE + 14;
pub const IRQ0_MASK: u32 = 0x7fff;
pub const IRQ0_END: u32 = IRQ0_SMC;
/* IRQ1 */
pub const IRQ1_BASE: u32 = IRQ0_END + 1;
pub const IRQ1_TS: u32 = IRQ1_BASE + 0;
pub const IRQ1_MASK: u32 = 0x0001;
pub const IRQ1_END: u32 = IRQ1_TS;
/* IRQ2 */
pub const IRQ2_BASE: u32 = IRQ1_END + 1;
pub const IRQ2_USB0: u32 = IRQ1_BASE + 0;
pub const IRQ2_USB1: u32 = IRQ1_BASE + 1;
pub const IRQ2_MASK: u32 = 0x0003;
pub const IRQ2_END: u32 = IRQ2_USB1;

pub const SE7724_FPGA_IRQ_NR: u32 = IRQ2_END - IRQ0_BASE;

/* arch/sh/boards/se/7724/irq.c */
extern "C" {
    pub fn init_se7724_IRQ();
}

/* __IO_PREFIX is se7724 in the original header. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
