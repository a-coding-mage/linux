/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 * m5206sim.h -- ColdFire 5206 System Integration Module support.
 *
 * (C) Copyright 1999, Greg Ungerer (gerg@snapgear.com)
 * (C) Copyright 2000, Lineo Inc. (www.lineo.com)
 */

/****************************************************************************/
/* The C header includes <asm/m52xxacr.h>; its symbols are supplied externally. */

pub const CPU_NAME: &str = "COLDFIRE(m5206)";
pub const CPU_INSTR_PER_JIFFY: i32 = 3;
pub const MCF_BUSCLK: _ = MCF_CLK;

/* Define the 5206 SIM register set addresses. */
pub const MCFSIM_SIMR: _ = MCF_MBAR + 0x03;
pub const MCFSIM_ICR1: _ = MCF_MBAR + 0x14;
pub const MCFSIM_ICR2: _ = MCF_MBAR + 0x15;
pub const MCFSIM_ICR3: _ = MCF_MBAR + 0x16;
pub const MCFSIM_ICR4: _ = MCF_MBAR + 0x17;
pub const MCFSIM_ICR5: _ = MCF_MBAR + 0x18;
pub const MCFSIM_ICR6: _ = MCF_MBAR + 0x19;
pub const MCFSIM_ICR7: _ = MCF_MBAR + 0x1a;
pub const MCFSIM_ICR8: _ = MCF_MBAR + 0x1b;
pub const MCFSIM_ICR9: _ = MCF_MBAR + 0x1c;
pub const MCFSIM_ICR10: _ = MCF_MBAR + 0x1d;
pub const MCFSIM_ICR11: _ = MCF_MBAR + 0x1e;
pub const MCFSIM_ICR12: _ = MCF_MBAR + 0x1f;
pub const MCFSIM_ICR13: _ = MCF_MBAR + 0x20;
/* C: #ifdef CONFIG_M5206e */
#[cfg(feature = "CONFIG_M5206e")]
pub const MCFSIM_ICR14: _ = MCF_MBAR + 0x21;
#[cfg(feature = "CONFIG_M5206e")]
pub const MCFSIM_ICR15: _ = MCF_MBAR + 0x22;

pub const MCFSIM_IMR: _ = MCF_MBAR + 0x36;
pub const MCFSIM_IPR: _ = MCF_MBAR + 0x3a;
pub const MCFSIM_RSR: _ = MCF_MBAR + 0x40;
pub const MCFSIM_SYPCR: _ = MCF_MBAR + 0x41;
pub const MCFSIM_SWIVR: _ = MCF_MBAR + 0x42;
pub const MCFSIM_SWSR: _ = MCF_MBAR + 0x43;
pub const MCFSIM_DCRR: _ = MCF_MBAR + 0x46;
pub const MCFSIM_DCTR: _ = MCF_MBAR + 0x4a;
pub const MCFSIM_DAR0: _ = MCF_MBAR + 0x4c;
pub const MCFSIM_DMR0: _ = MCF_MBAR + 0x50;
pub const MCFSIM_DCR0: _ = MCF_MBAR + 0x57;
pub const MCFSIM_DAR1: _ = MCF_MBAR + 0x58;
pub const MCFSIM_DMR1: _ = MCF_MBAR + 0x5c;
pub const MCFSIM_DCR1: _ = MCF_MBAR + 0x63;

pub const MCFSIM_CSAR0: _ = MCF_MBAR + 0x64;
pub const MCFSIM_CSMR0: _ = MCF_MBAR + 0x68;
pub const MCFSIM_CSCR0: _ = MCF_MBAR + 0x6e;
pub const MCFSIM_CSAR1: _ = MCF_MBAR + 0x70;
pub const MCFSIM_CSMR1: _ = MCF_MBAR + 0x74;
pub const MCFSIM_CSCR1: _ = MCF_MBAR + 0x7a;
pub const MCFSIM_CSAR2: _ = MCF_MBAR + 0x7c;
pub const MCFSIM_CSMR2: _ = MCF_MBAR + 0x80;
pub const MCFSIM_CSCR2: _ = MCF_MBAR + 0x86;
pub const MCFSIM_CSAR3: _ = MCF_MBAR + 0x88;
pub const MCFSIM_CSMR3: _ = MCF_MBAR + 0x8c;
pub const MCFSIM_CSCR3: _ = MCF_MBAR + 0x92;
pub const MCFSIM_CSAR4: _ = MCF_MBAR + 0x94;
pub const MCFSIM_CSMR4: _ = MCF_MBAR + 0x98;
pub const MCFSIM_CSCR4: _ = MCF_MBAR + 0x9e;
pub const MCFSIM_CSAR5: _ = MCF_MBAR + 0xa0;
pub const MCFSIM_CSMR5: _ = MCF_MBAR + 0xa4;
pub const MCFSIM_CSCR5: _ = MCF_MBAR + 0xaa;
pub const MCFSIM_CSAR6: _ = MCF_MBAR + 0xac;
pub const MCFSIM_CSMR6: _ = MCF_MBAR + 0xb0;
pub const MCFSIM_CSCR6: _ = MCF_MBAR + 0xb6;
pub const MCFSIM_CSAR7: _ = MCF_MBAR + 0xb8;
pub const MCFSIM_CSMR7: _ = MCF_MBAR + 0xbc;
pub const MCFSIM_CSCR7: _ = MCF_MBAR + 0xc2;
pub const MCFSIM_DMCR: _ = MCF_MBAR + 0xc6;

#[cfg(feature = "CONFIG_M5206e")]
pub const MCFSIM_PAR: _ = MCF_MBAR + 0xca;
#[cfg(not(feature = "CONFIG_M5206e"))]
pub const MCFSIM_PAR: _ = MCF_MBAR + 0xcb;

pub const MCFTIMER_BASE1: _ = MCF_MBAR + 0x100;
pub const MCFTIMER_BASE2: _ = MCF_MBAR + 0x120;
pub const MCFSIM_PADDR: _ = MCF_MBAR + 0x1c5;
pub const MCFSIM_PADAT: _ = MCF_MBAR + 0x1c9;
pub const MCFDMA_BASE0: _ = MCF_MBAR + 0x200;
pub const MCFDMA_BASE1: _ = MCF_MBAR + 0x240;

#[cfg(feature = "CONFIG_NETtel")]
pub const MCFUART_BASE0: _ = MCF_MBAR + 0x180;
#[cfg(feature = "CONFIG_NETtel")]
pub const MCFUART_BASE1: _ = MCF_MBAR + 0x140;
#[cfg(not(feature = "CONFIG_NETtel"))]
pub const MCFUART_BASE0: _ = MCF_MBAR + 0x140;
#[cfg(not(feature = "CONFIG_NETtel"))]
pub const MCFUART_BASE1: _ = MCF_MBAR + 0x180;

/* Define system peripheral IRQ usage. */
pub const MCF_IRQ_I2C0: i32 = 29;
pub const MCF_IRQ_TIMER: i32 = 30;
pub const MCF_IRQ_PROFILER: i32 = 31;
pub const MCF_IRQ_UART0: i32 = 73;
pub const MCF_IRQ_UART1: i32 = 74;

/* Generic GPIO */
pub const MCFGPIO_PIN_MAX: i32 = 8;
pub const MCFGPIO_IRQ_VECBASE: i32 = -1;
pub const MCFGPIO_IRQ_MAX: i32 = -1;

/* Some symbol defines for the Parallel Port Pin Assignment Register. */
#[cfg(feature = "CONFIG_M5206e")]
pub const MCFSIM_PAR_DREQ0: i32 = 0x100; /* Set to select DREQ0 input; clear to select T0 input. */
#[cfg(feature = "CONFIG_M5206e")]
pub const MCFSIM_PAR_DREQ1: i32 = 0x200; /* Select DREQ1 input; clear to select T0 output. */

/* Some symbol defines for the Interrupt Control Register. */
pub const MCFSIM_SWDICR: _ = MCFSIM_ICR8;
pub const MCFSIM_TIMER1ICR: _ = MCFSIM_ICR9;
pub const MCFSIM_TIMER2ICR: _ = MCFSIM_ICR10;
pub const MCFSIM_I2CICR: _ = MCFSIM_ICR11;
pub const MCFSIM_UART1ICR: _ = MCFSIM_ICR12;
pub const MCFSIM_UART2ICR: _ = MCFSIM_ICR13;
#[cfg(feature = "CONFIG_M5206e")]
pub const MCFSIM_DMA1ICR: _ = MCFSIM_ICR14;
#[cfg(feature = "CONFIG_M5206e")]
pub const MCFSIM_DMA2ICR: _ = MCFSIM_ICR15;

/* I2C Controller */
pub const MCFI2C_BASE0: _ = MCF_MBAR + 0x1e0;
pub const MCFI2C_SIZE0: i32 = 0x40;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
