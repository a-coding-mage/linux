/* SPDX-License-Identifier: GPL-2.0 */

/*
 * linux/include/asm-sh/sdk7780.h
 *
 * Renesas Solutions SH7780 SDK Support
 * Copyright (C) 2008 Nicholas Beck <nbeck@mpc-data.co.uk>
 */
/* C dependencies: linux/sh_intc.h, asm/addrspace.h, and asm/io_generic.h. */

/* Box specific addresses. */
pub const SE_AREA0_WIDTH: usize = 4; /* Area0: 32bit */
pub const PA_ROM: u32 = 0xa000_0000; /* EPROM */
pub const PA_ROM_SIZE: u32 = 0x0040_0000; /* EPROM size 4M byte */
pub const PA_FROM: u32 = 0xa080_0000; /* Flash-ROM */
pub const PA_FROM_SIZE: u32 = 0x0040_0000; /* Flash-ROM size 4M byte */
pub const PA_EXT1: u32 = 0xa400_0000;
pub const PA_EXT1_SIZE: u32 = 0x0400_0000;
pub const PA_SDRAM: u32 = 0xa800_0000; /* DDR-SDRAM(Area2/3) 128MB */
pub const PA_SDRAM_SIZE: u32 = 0x0800_0000;

pub const PA_EXT4: u32 = 0xb000_0000;
pub const PA_EXT4_SIZE: u32 = 0x0400_0000;
pub const PA_EXT_USER: u32 = PA_EXT4; /* User Expansion Space */

/* Supplied by asm/addrspace.h. */
pub const PA_PERIPHERAL: u32 = PA_AREA5_IO;

/* SRAM/Reserved */
pub const PA_RESERVED: u32 = PA_PERIPHERAL + 0;
/* FPGA base address */
pub const PA_FPGA: u32 = PA_PERIPHERAL + 0x0100_0000;
/* SMC LAN91C111 */
pub const PA_LAN: u32 = PA_PERIPHERAL + 0x0180_0000;

pub const FPGA_SRSTR: u32 = PA_FPGA + 0x000; /* System reset */
pub const FPGA_IRQ0SR: u32 = PA_FPGA + 0x010; /* IRQ0 status */
pub const FPGA_IRQ0MR: u32 = PA_FPGA + 0x020; /* IRQ0 mask */
pub const FPGA_BDMR: u32 = PA_FPGA + 0x030; /* Board operating mode */
pub const FPGA_INTT0PRTR: u32 = PA_FPGA + 0x040; /* Interrupt test mode0 port */
pub const FPGA_INTT0SELR: u32 = PA_FPGA + 0x050; /* Int. test mode0 select */
pub const FPGA_INTT1POLR: u32 = PA_FPGA + 0x060; /* Int. test mode0 polarity */
pub const FPGA_NMIR: u32 = PA_FPGA + 0x070; /* NMI source */
pub const FPGA_NMIMR: u32 = PA_FPGA + 0x080; /* NMI mask */
pub const FPGA_IRQR: u32 = PA_FPGA + 0x090; /* IRQX source */
pub const FPGA_IRQMR: u32 = PA_FPGA + 0x0a0; /* IRQX mask */
pub const FPGA_SLEDR: u32 = PA_FPGA + 0x0b0; /* LED control */
pub const PA_LED: u32 = FPGA_SLEDR;
pub const FPGA_MAPSWR: u32 = PA_FPGA + 0x0c0; /* Map switch */
pub const FPGA_FPVERR: u32 = PA_FPGA + 0x0d0; /* FPGA version */
pub const FPGA_FPDATER: u32 = PA_FPGA + 0x0e0; /* FPGA date */
pub const FPGA_RSE: u32 = PA_FPGA + 0x100; /* Reset source */
pub const FPGA_EASR: u32 = PA_FPGA + 0x110; /* External area select */
pub const FPGA_SPER: u32 = PA_FPGA + 0x120; /* Serial port enable */
pub const FPGA_IMSR: u32 = PA_FPGA + 0x130; /* Interrupt mode select */
pub const FPGA_PCIMR: u32 = PA_FPGA + 0x140; /* PCI Mode */
pub const FPGA_DIPSWMR: u32 = PA_FPGA + 0x150; /* DIPSW monitor */
pub const FPGA_FPODR: u32 = PA_FPGA + 0x160; /* Output port data */
pub const FPGA_ATAESR: u32 = PA_FPGA + 0x170; /* ATA extended bus status */
pub const FPGA_IRQPOLR: u32 = PA_FPGA + 0x180; /* IRQx polarity */

pub const SDK7780_NR_IRL: u32 = 15;

/* External dependency supplied by linux/sh_intc.h. */
unsafe extern "C" {
    pub fn evt2irq(event: u32) -> u32;
}

/* IDE/ATA interrupt */
#[inline]
pub unsafe fn IRQ_CFCARD() -> u32 {
    unsafe { evt2irq(0x3c0) }
}

/* SMC interrupt */
#[inline]
pub unsafe fn IRQ_ETHERNET() -> u32 {
    unsafe { evt2irq(0x2c0) }
}

/* arch/sh/boards/renesas/sdk7780/irq.c */
unsafe extern "C" {
    pub fn init_sdk7780_IRQ();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
