/* SPDX-License-Identifier: GPL-2.0 */
/*
 * m5307sim.h -- ColdFire 5307 System Integration Module support.
 *
 * (C) Copyright 1999, Moreton Bay Ventures Pty Ltd.
 * (C) Copyright 1999, Lineo (www.lineo.com)
 * Modified by David W. Miller for the MCF5307 Eval Board.
 */

// Dependency: symbols from asm/m53xxacr.h are supplied externally.

pub const CPU_NAME: &str = "COLDFIRE(m5307)";
pub const CPU_INSTR_PER_JIFFY: i32 = 3;
pub const MCF_BUSCLK: _ = MCF_CLK / 2;

pub const MCFSIM_RSR: _ = MCF_MBAR + 0x00; // Reset Status reg
pub const MCFSIM_SYPCR: _ = MCF_MBAR + 0x01; // System Protection
pub const MCFSIM_SWIVR: _ = MCF_MBAR + 0x02; // SW Watchdog intr
pub const MCFSIM_SWSR: _ = MCF_MBAR + 0x03; // SW Watchdog service
pub const MCFSIM_PAR: _ = MCF_MBAR + 0x04; // Pin Assignment
pub const MCFSIM_IRQPAR: _ = MCF_MBAR + 0x06; // Itr Assignment
pub const MCFSIM_PLLCR: _ = MCF_MBAR + 0x08; // PLL Ctrl Reg
pub const MCFSIM_MPARK: _ = MCF_MBAR + 0x0C; // BUS Master Ctrl
pub const MCFSIM_IPR: _ = MCF_MBAR + 0x40; // Interrupt Pend
pub const MCFSIM_IMR: _ = MCF_MBAR + 0x44; // Interrupt Mask
pub const MCFSIM_AVR: _ = MCF_MBAR + 0x4b; // Autovector Ctrl
pub const MCFSIM_ICR0: _ = MCF_MBAR + 0x4c;
pub const MCFSIM_ICR1: _ = MCF_MBAR + 0x4d;
pub const MCFSIM_ICR2: _ = MCF_MBAR + 0x4e;
pub const MCFSIM_ICR3: _ = MCF_MBAR + 0x4f;
pub const MCFSIM_ICR4: _ = MCF_MBAR + 0x50;
pub const MCFSIM_ICR5: _ = MCF_MBAR + 0x51;
pub const MCFSIM_ICR6: _ = MCF_MBAR + 0x52;
pub const MCFSIM_ICR7: _ = MCF_MBAR + 0x53;
pub const MCFSIM_ICR8: _ = MCF_MBAR + 0x54;
pub const MCFSIM_ICR9: _ = MCF_MBAR + 0x55;
pub const MCFSIM_ICR10: _ = MCF_MBAR + 0x56;
pub const MCFSIM_ICR11: _ = MCF_MBAR + 0x57;

pub const MCFSIM_CSAR0: _ = MCF_MBAR + 0x80;
pub const MCFSIM_CSMR0: _ = MCF_MBAR + 0x84;
pub const MCFSIM_CSCR0: _ = MCF_MBAR + 0x8a;
pub const MCFSIM_CSAR1: _ = MCF_MBAR + 0x8c;
pub const MCFSIM_CSMR1: _ = MCF_MBAR + 0x90;
pub const MCFSIM_CSCR1: _ = MCF_MBAR + 0x96;

#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSBAR: _ = MCF_MBAR + 0x98;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSBAMR: _ = MCF_MBAR + 0x9c;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSMR2: _ = MCF_MBAR + 0x9e;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSCR2: _ = MCF_MBAR + 0xa2;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSMR3: _ = MCF_MBAR + 0xaa;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSCR3: _ = MCF_MBAR + 0xae;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSMR4: _ = MCF_MBAR + 0xb6;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSCR4: _ = MCF_MBAR + 0xba;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSMR5: _ = MCF_MBAR + 0xc2;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSCR5: _ = MCF_MBAR + 0xc6;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSMR6: _ = MCF_MBAR + 0xce;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSCR6: _ = MCF_MBAR + 0xd2;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSMR7: _ = MCF_MBAR + 0xda;
#[cfg(feature = "CONFIG_OLDMASK")]
pub const MCFSIM_CSCR7: _ = MCF_MBAR + 0xde;

#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSAR2: _ = MCF_MBAR + 0x98;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSMR2: _ = MCF_MBAR + 0x9c;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSCR2: _ = MCF_MBAR + 0xa2;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSAR3: _ = MCF_MBAR + 0xa4;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSMR3: _ = MCF_MBAR + 0xa8;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSCR3: _ = MCF_MBAR + 0xae;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSAR4: _ = MCF_MBAR + 0xb0;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSMR4: _ = MCF_MBAR + 0xb4;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSCR4: _ = MCF_MBAR + 0xba;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSAR5: _ = MCF_MBAR + 0xbc;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSMR5: _ = MCF_MBAR + 0xc0;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSCR5: _ = MCF_MBAR + 0xc6;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSAR6: _ = MCF_MBAR + 0xc8;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSMR6: _ = MCF_MBAR + 0xcc;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSCR6: _ = MCF_MBAR + 0xd2;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSAR7: _ = MCF_MBAR + 0xd4;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSMR7: _ = MCF_MBAR + 0xd8;
#[cfg(not(feature = "CONFIG_OLDMASK"))]
pub const MCFSIM_CSCR7: _ = MCF_MBAR + 0xde;

pub const MCFSIM_DCR: _ = MCF_MBAR + 0x100;
pub const MCFSIM_DACR0: _ = MCF_MBAR + 0x108;
pub const MCFSIM_DMR0: _ = MCF_MBAR + 0x10c;
pub const MCFSIM_DACR1: _ = MCF_MBAR + 0x110;
pub const MCFSIM_DMR1: _ = MCF_MBAR + 0x114;
pub const MCFTIMER_BASE1: _ = MCF_MBAR + 0x140;
pub const MCFTIMER_BASE2: _ = MCF_MBAR + 0x180;
pub const MCFSIM_PADDR: _ = MCF_MBAR + 0x244;
pub const MCFSIM_PADAT: _ = MCF_MBAR + 0x248;
pub const MCFDMA_BASE0: _ = MCF_MBAR + 0x300;
pub const MCFDMA_BASE1: _ = MCF_MBAR + 0x340;
pub const MCFDMA_BASE2: _ = MCF_MBAR + 0x380;
pub const MCFDMA_BASE3: _ = MCF_MBAR + 0x3C0;

// CONFIG_NETtel and CONFIG_SECUREEDGEMP3 are build-time conditions.
#[cfg(any(feature = "CONFIG_NETtel", feature = "CONFIG_SECUREEDGEMP3"))]
pub const MCFUART_BASE0: _ = MCF_MBAR + 0x200;
#[cfg(any(feature = "CONFIG_NETtel", feature = "CONFIG_SECUREEDGEMP3"))]
pub const MCFUART_BASE1: _ = MCF_MBAR + 0x1c0;
#[cfg(not(any(feature = "CONFIG_NETtel", feature = "CONFIG_SECUREEDGEMP3")))]
pub const MCFUART_BASE0: _ = MCF_MBAR + 0x1c0;
#[cfg(not(any(feature = "CONFIG_NETtel", feature = "CONFIG_SECUREEDGEMP3")))]
pub const MCFUART_BASE1: _ = MCF_MBAR + 0x200;

pub const MCFGPIO_PIN_MAX: i32 = 16;
pub const MCFGPIO_IRQ_MAX: i32 = -1;
pub const MCFGPIO_IRQ_VECBASE: i32 = -1;

pub const MCF5307_CS2: u32 = 0x400000;
pub const MCF5307_CS3: u32 = 0x600000;
pub const MCF5307_CS4: u32 = 0x800000;
pub const MCF5307_CS5: u32 = 0xA00000;
pub const MCF5307_CS6: u32 = 0xC00000;
pub const MCF5307_CS7: u32 = 0xE00000;

pub const MCFSIM_SWDICR: _ = MCFSIM_ICR0;
pub const MCFSIM_TIMER1ICR: _ = MCFSIM_ICR1;
pub const MCFSIM_TIMER2ICR: _ = MCFSIM_ICR2;
pub const MCFSIM_I2CICR: _ = MCFSIM_ICR3;
pub const MCFSIM_UART1ICR: _ = MCFSIM_ICR4;
pub const MCFSIM_UART2ICR: _ = MCFSIM_ICR5;
pub const MCFSIM_DMA0ICR: _ = MCFSIM_ICR6;
pub const MCFSIM_DMA1ICR: _ = MCFSIM_ICR7;
pub const MCFSIM_DMA2ICR: _ = MCFSIM_ICR8;
pub const MCFSIM_DMA3ICR: _ = MCFSIM_ICR9;

pub const MCFSIM_PAR_DREQ0: u32 = 0x40;
pub const MCFSIM_PAR_DREQ1: u32 = 0x20;
pub const IRQ5_LEVEL4: u32 = 0x80;
pub const IRQ3_LEVEL6: u32 = 0x40;
pub const IRQ1_LEVEL2: u32 = 0x20;

pub const MCF_IRQ_I2C0: i32 = 29;
pub const MCF_IRQ_TIMER: i32 = 30;
pub const MCF_IRQ_PROFILER: i32 = 31;
pub const MCF_IRQ_UART0: i32 = 73;
pub const MCF_IRQ_UART1: i32 = 74;
pub const MCFI2C_BASE0: _ = MCF_MBAR + 0x280;
pub const MCFI2C_SIZE0: u32 = 0x40;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
