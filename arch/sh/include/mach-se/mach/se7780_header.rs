/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/asm-sh/se7780.h
 *
 * Copyright (C) 2006,2007  Nobuhiro Iwamatsu
 *
 * Hitachi UL SolutionEngine 7780 Support.
 */

// External dependencies from linux/sh_intc.h, asm/addrspace.h, and
// asm/io_generic.h are supplied by the surrounding translation unit.

/* Box specific addresses. */
pub const SE_AREA0_WIDTH: u32 = 4; /* Area0: 32bit */
pub const PA_ROM: u32 = 0xa0000000; /* EPROM */
pub const PA_ROM_SIZE: u32 = 0x00400000; /* EPROM size 4M byte */
pub const PA_FROM: u32 = 0xa1000000; /* Flash-ROM */
pub const PA_FROM_SIZE: u32 = 0x01000000; /* Flash-ROM size 16M byte */
pub const PA_EXT1: u32 = 0xa4000000;
pub const PA_EXT1_SIZE: u32 = 0x04000000;
pub const PA_SM501: u32 = PA_EXT1; /* Graphic IC (SM501) */
pub const PA_SM501_SIZE: u32 = PA_EXT1_SIZE; /* Graphic IC (SM501) */
pub const PA_SDRAM: u32 = 0xa8000000; /* DDR-SDRAM(Area2/3) 128MB */
pub const PA_SDRAM_SIZE: u32 = 0x08000000;

pub const PA_EXT4: u32 = 0xb0000000;
pub const PA_EXT4_SIZE: u32 = 0x04000000;
pub const PA_EXT_FLASH: u32 = PA_EXT4; /* Expansion Flash-ROM */

pub const PA_PERIPHERAL: u32 = PA_AREA6_IO; /* SW6-6=ON */

pub const PA_LAN: u32 = PA_PERIPHERAL + 0; /* SMC LAN91C111 */
pub const PA_LED_DISP: u32 = PA_PERIPHERAL + 0x02000000; /* 8words LED Display */
pub const DISP_CHAR_RAM: u32 = 7 << 3;
pub const DISP_SEL0_ADDR: u32 = DISP_CHAR_RAM + 0;
pub const DISP_SEL1_ADDR: u32 = DISP_CHAR_RAM + 1;
pub const DISP_SEL2_ADDR: u32 = DISP_CHAR_RAM + 2;
pub const DISP_SEL3_ADDR: u32 = DISP_CHAR_RAM + 3;
pub const DISP_SEL4_ADDR: u32 = DISP_CHAR_RAM + 4;
pub const DISP_SEL5_ADDR: u32 = DISP_CHAR_RAM + 5;
pub const DISP_SEL6_ADDR: u32 = DISP_CHAR_RAM + 6;
pub const DISP_SEL7_ADDR: u32 = DISP_CHAR_RAM + 7;

pub const DISP_UDC_RAM: u32 = 5 << 3;
pub const PA_FPGA: u32 = PA_PERIPHERAL + 0x03000000; /* FPGA base address */

/* FPGA register address and bit */
pub const FPGA_SFTRST: u32 = PA_FPGA + 0; /* Soft reset register */
pub const FPGA_INTMSK1: u32 = PA_FPGA + 2; /* Interrupt Mask register 1 */
pub const FPGA_INTMSK2: u32 = PA_FPGA + 4; /* Interrupt Mask register 2 */
pub const FPGA_INTSEL1: u32 = PA_FPGA + 6; /* Interrupt select register 1 */
pub const FPGA_INTSEL2: u32 = PA_FPGA + 8; /* Interrupt select register 2 */
pub const FPGA_INTSEL3: u32 = PA_FPGA + 10; /* Interrupt select register 3 */
pub const FPGA_PCI_INTSEL1: u32 = PA_FPGA + 12; /* PCI Interrupt select register 1 */
pub const FPGA_PCI_INTSEL2: u32 = PA_FPGA + 14; /* PCI Interrupt select register 2 */
pub const FPGA_INTSET: u32 = PA_FPGA + 16; /* IRQ/IRL select register */
pub const FPGA_INTSTS1: u32 = PA_FPGA + 18; /* Interrupt status register 1 */
pub const FPGA_INTSTS2: u32 = PA_FPGA + 20; /* Interrupt status register 2 */
pub const FPGA_REQSEL: u32 = PA_FPGA + 22; /* REQ/GNT select register */
pub const FPGA_DBG_LED: u32 = PA_FPGA + 32; /* Debug LED(D-LED[8:1] */
pub const PA_LED: u32 = FPGA_DBG_LED;
pub const FPGA_IVDRID: u32 = PA_FPGA + 36; /* iVDR ID Register */
pub const FPGA_IVDRPW: u32 = PA_FPGA + 38; /* iVDR Power ON Register */
pub const FPGA_MMCID: u32 = PA_FPGA + 40; /* MMC ID Register */

/* FPGA INTSEL position */
pub const IRQPOS_SMC91CX: u32 = 0 * 4;
pub const IRQPOS_SM501: u32 = 1 * 4;
pub const IRQPOS_EXTINT1: u32 = 0 * 4;
pub const IRQPOS_EXTINT2: u32 = 1 * 4;
pub const IRQPOS_EXTINT3: u32 = 2 * 4;
pub const IRQPOS_EXTINT4: u32 = 3 * 4;
pub const IRQPOS_PCCPW: u32 = 0 * 4;

/* External evt2irq macro is provided by the interrupt definitions. */
pub const IRQ_IDE0: u32 = evt2irq!(0xa60); /* iVDR */
pub const SMC_IRQ: u32 = evt2irq!(0x300);
pub const SM501_IRQ: u32 = evt2irq!(0x200);

/* interrupt pin */
pub const IRQPIN_EXTINT1: u32 = 0; /* IRQ0 pin */
pub const IRQPIN_EXTINT2: u32 = 1; /* IRQ1 pin */
pub const IRQPIN_EXTINT3: u32 = 2; /* IRQ2 pin */
pub const IRQPIN_SMC91CX: u32 = 3; /* IRQ3 pin */
pub const IRQPIN_EXTINT4: u32 = 4; /* IRQ4 pin */
pub const IRQPIN_PCC0: u32 = 5; /* IRQ5 pin */
pub const IRQPIN_PCC2: u32 = 6; /* IRQ6 pin */
pub const IRQPIN_SM501: u32 = 7; /* IRQ7 pin */
pub const IRQPIN_PCCPW: u32 = 7; /* IRQ7 pin */

/* arch/sh/boards/se/7780/irq.c */
unsafe extern "C" {
    pub fn init_se7780_IRQ();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
