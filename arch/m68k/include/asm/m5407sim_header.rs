/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 *	m5407sim.h -- ColdFire 5407 System Integration Module support.
 *
 *	(C) Copyright 2000,  Lineo (www.lineo.com)
 *	(C) Copyright 1999,  Moreton Bay Ventures Pty Ltd.
 *
 *      Modified by David W. Miller for the MCF5307 Eval Board.
 */

/****************************************************************************/

// Dependency supplied by the corresponding m54xxacr translation.

pub const CPU_NAME: &str = "COLDFIRE(m5407)";
pub const CPU_INSTR_PER_JIFFY: i32 = 3;
pub const MCF_BUSCLK: _ = MCF_CLK / 2;

/* Define the 5407 SIM register set addresses. */
pub const MCFSIM_RSR: _ = MCF_MBAR + 0x00; // Reset Status
pub const MCFSIM_SYPCR: _ = MCF_MBAR + 0x01; // System Protection
pub const MCFSIM_SWIVR: _ = MCF_MBAR + 0x02; // SW Watchdog intr
pub const MCFSIM_SWSR: _ = MCF_MBAR + 0x03; // SW Watchdog service
pub const MCFSIM_PAR: _ = MCF_MBAR + 0x04; // Pin Assignment
pub const MCFSIM_IRQPAR: _ = MCF_MBAR + 0x06; // Intr Assignment
pub const MCFSIM_PLLCR: _ = MCF_MBAR + 0x08; // PLL Ctrl
pub const MCFSIM_MPARK: _ = MCF_MBAR + 0x0C; // BUS Master Ctrl
pub const MCFSIM_IPR: _ = MCF_MBAR + 0x40; // Interrupt Pending
pub const MCFSIM_IMR: _ = MCF_MBAR + 0x44; // Interrupt Mask
pub const MCFSIM_AVR: _ = MCF_MBAR + 0x4b; // Autovector Ctrl
pub const MCFSIM_ICR0: _ = MCF_MBAR + 0x4c; // Intr Ctrl reg 0
pub const MCFSIM_ICR1: _ = MCF_MBAR + 0x4d; // Intr Ctrl reg 1
pub const MCFSIM_ICR2: _ = MCF_MBAR + 0x4e; // Intr Ctrl reg 2
pub const MCFSIM_ICR3: _ = MCF_MBAR + 0x4f; // Intr Ctrl reg 3
pub const MCFSIM_ICR4: _ = MCF_MBAR + 0x50; // Intr Ctrl reg 4
pub const MCFSIM_ICR5: _ = MCF_MBAR + 0x51; // Intr Ctrl reg 5
pub const MCFSIM_ICR6: _ = MCF_MBAR + 0x52; // Intr Ctrl reg 6
pub const MCFSIM_ICR7: _ = MCF_MBAR + 0x53; // Intr Ctrl reg 7
pub const MCFSIM_ICR8: _ = MCF_MBAR + 0x54; // Intr Ctrl reg 8
pub const MCFSIM_ICR9: _ = MCF_MBAR + 0x55; // Intr Ctrl reg 9
pub const MCFSIM_ICR10: _ = MCF_MBAR + 0x56; // Intr Ctrl reg 10
pub const MCFSIM_ICR11: _ = MCF_MBAR + 0x57; // Intr Ctrl reg 11

pub const MCFSIM_CSAR0: _ = MCF_MBAR + 0x80; // CS 0 Address reg
pub const MCFSIM_CSMR0: _ = MCF_MBAR + 0x84; // CS 0 Mask reg
pub const MCFSIM_CSCR0: _ = MCF_MBAR + 0x8a; // CS 0 Control reg
pub const MCFSIM_CSAR1: _ = MCF_MBAR + 0x8c; // CS 1 Address reg
pub const MCFSIM_CSMR1: _ = MCF_MBAR + 0x90; // CS 1 Mask reg
pub const MCFSIM_CSCR1: _ = MCF_MBAR + 0x96; // CS 1 Control reg
pub const MCFSIM_CSAR2: _ = MCF_MBAR + 0x98; // CS 2 Address reg
pub const MCFSIM_CSMR2: _ = MCF_MBAR + 0x9c; // CS 2 Mask reg
pub const MCFSIM_CSCR2: _ = MCF_MBAR + 0xa2; // CS 2 Control reg
pub const MCFSIM_CSAR3: _ = MCF_MBAR + 0xa4; // CS 3 Address reg
pub const MCFSIM_CSMR3: _ = MCF_MBAR + 0xa8; // CS 3 Mask reg
pub const MCFSIM_CSCR3: _ = MCF_MBAR + 0xae; // CS 3 Control reg
pub const MCFSIM_CSAR4: _ = MCF_MBAR + 0xb0; // CS 4 Address reg
pub const MCFSIM_CSMR4: _ = MCF_MBAR + 0xb4; // CS 4 Mask reg
pub const MCFSIM_CSCR4: _ = MCF_MBAR + 0xba; // CS 4 Control reg
pub const MCFSIM_CSAR5: _ = MCF_MBAR + 0xbc; // CS 5 Address reg
pub const MCFSIM_CSMR5: _ = MCF_MBAR + 0xc0; // CS 5 Mask reg
pub const MCFSIM_CSCR5: _ = MCF_MBAR + 0xc6; // CS 5 Control reg
pub const MCFSIM_CSAR6: _ = MCF_MBAR + 0xc8; // CS 6 Address reg
pub const MCFSIM_CSMR6: _ = MCF_MBAR + 0xcc; // CS 6 Mask reg
pub const MCFSIM_CSCR6: _ = MCF_MBAR + 0xd2; // CS 6 Control reg
pub const MCFSIM_CSAR7: _ = MCF_MBAR + 0xd4; // CS 7 Address reg
pub const MCFSIM_CSMR7: _ = MCF_MBAR + 0xd8; // CS 7 Mask reg
pub const MCFSIM_CSCR7: _ = MCF_MBAR + 0xde; // CS 7 Control reg

pub const MCFSIM_DCR: _ = MCF_MBAR + 0x100; // DRAM Control
pub const MCFSIM_DACR0: _ = MCF_MBAR + 0x108; // DRAM 0 Addr/Ctrl
pub const MCFSIM_DMR0: _ = MCF_MBAR + 0x10c; // DRAM 0 Mask
pub const MCFSIM_DACR1: _ = MCF_MBAR + 0x110; // DRAM 1 Addr/Ctrl
pub const MCFSIM_DMR1: _ = MCF_MBAR + 0x114; // DRAM 1 Mask

/* Timer module. */
pub const MCFTIMER_BASE1: _ = MCF_MBAR + 0x140; // Base of TIMER1
pub const MCFTIMER_BASE2: _ = MCF_MBAR + 0x180; // Base of TIMER2
pub const MCFUART_BASE0: _ = MCF_MBAR + 0x1c0; // Base address UART0
pub const MCFUART_BASE1: _ = MCF_MBAR + 0x200; // Base address UART1
pub const MCFSIM_PADDR: _ = MCF_MBAR + 0x244;
pub const MCFSIM_PADAT: _ = MCF_MBAR + 0x248;

/* DMA unit base addresses. */
pub const MCFDMA_BASE0: _ = MCF_MBAR + 0x300; // Base address DMA 0
pub const MCFDMA_BASE1: _ = MCF_MBAR + 0x340; // Base address DMA 1
pub const MCFDMA_BASE2: _ = MCF_MBAR + 0x380; // Base address DMA 2
pub const MCFDMA_BASE3: _ = MCF_MBAR + 0x3C0; // Base address DMA 3

/* Generic GPIO support */
pub const MCFGPIO_PIN_MAX: i32 = 16;
pub const MCFGPIO_IRQ_MAX: i32 = -1;
pub const MCFGPIO_IRQ_VECBASE: i32 = -1;

/* Some symbol defines for the above. */
pub const MCFSIM_SWDICR: _ = MCFSIM_ICR0; // Watchdog timer ICR
pub const MCFSIM_TIMER1ICR: _ = MCFSIM_ICR1; // Timer 1 ICR
pub const MCFSIM_TIMER2ICR: _ = MCFSIM_ICR2; // Timer 2 ICR
pub const MCFSIM_I2CICR: _ = MCFSIM_ICR3; // I2C ICR
pub const MCFSIM_UART1ICR: _ = MCFSIM_ICR4; // UART 1 ICR
pub const MCFSIM_UART2ICR: _ = MCFSIM_ICR5; // UART 2 ICR
pub const MCFSIM_DMA0ICR: _ = MCFSIM_ICR6; // DMA 0 ICR
pub const MCFSIM_DMA1ICR: _ = MCFSIM_ICR7; // DMA 1 ICR
pub const MCFSIM_DMA2ICR: _ = MCFSIM_ICR8; // DMA 2 ICR
pub const MCFSIM_DMA3ICR: _ = MCFSIM_ICR9; // DMA 3 ICR

/* Some symbol defines for the Parallel Port Pin Assignment Register */
pub const MCFSIM_PAR_DREQ0: i32 = 0x40; // Set to select DREQ0 input
// Clear to select par I/O
pub const MCFSIM_PAR_DREQ1: i32 = 0x20; // Select DREQ1 input
// Clear to select par I/O

/* Defines for the IRQPAR Register */
pub const IRQ5_LEVEL4: i32 = 0x80;
pub const IRQ3_LEVEL6: i32 = 0x40;
pub const IRQ1_LEVEL2: i32 = 0x20;

/* Define system peripheral IRQ usage. */
pub const MCF_IRQ_I2C0: i32 = 29; // I2C, Level 5
pub const MCF_IRQ_TIMER: i32 = 30; // Timer0, Level 6
pub const MCF_IRQ_PROFILER: i32 = 31; // Timer1, Level 7
pub const MCF_IRQ_UART0: i32 = 73; // UART0
pub const MCF_IRQ_UART1: i32 = 74; // UART1

/* I2C module */
pub const MCFI2C_BASE0: _ = MCF_MBAR + 0x280;
pub const MCFI2C_SIZE0: i32 = 0x40;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
