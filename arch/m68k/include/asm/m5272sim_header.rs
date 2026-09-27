/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 * m5272sim.h -- ColdFire 5272 System Integration Module support.
 *
 * (C) Copyright 1999, Greg Ungerer (gerg@snapgear.com)
 * (C) Copyright 2000, Lineo Inc. (www.lineo.com)
 */

/****************************************************************************/
/* Dependency: asm/m52xxacr.h */

pub const CPU_NAME: &str = "COLDFIRE(m5272)";
pub const CPU_INSTR_PER_JIFFY: i32 = 3;
pub const MCF_BUSCLK: usize = MCF_CLK;

/* Define the 5272 SIM register set addresses. */
pub const MCFSIM_SCR: usize = MCF_MBAR + 0x04;
pub const MCFSIM_SPR: usize = MCF_MBAR + 0x06;
pub const MCFSIM_PMR: usize = MCF_MBAR + 0x08;
pub const MCFSIM_APMR: usize = MCF_MBAR + 0x0e;
pub const MCFSIM_DIR: usize = MCF_MBAR + 0x10;
pub const MCFSIM_ICR1: usize = MCF_MBAR + 0x20;
pub const MCFSIM_ICR2: usize = MCF_MBAR + 0x24;
pub const MCFSIM_ICR3: usize = MCF_MBAR + 0x28;
pub const MCFSIM_ICR4: usize = MCF_MBAR + 0x2c;
pub const MCFSIM_ISR: usize = MCF_MBAR + 0x30;
pub const MCFSIM_PITR: usize = MCF_MBAR + 0x34;
pub const MCFSIM_PIWR: usize = MCF_MBAR + 0x38;
pub const MCFSIM_PIVR: usize = MCF_MBAR + 0x3f;
pub const MCFSIM_WRRR: usize = MCF_MBAR + 0x280;
pub const MCFSIM_WIRR: usize = MCF_MBAR + 0x284;
pub const MCFSIM_WCR: usize = MCF_MBAR + 0x288;
pub const MCFSIM_WER: usize = MCF_MBAR + 0x28c;

pub const MCFSIM_CSBR0: usize = MCF_MBAR + 0x40;
pub const MCFSIM_CSOR0: usize = MCF_MBAR + 0x44;
pub const MCFSIM_CSBR1: usize = MCF_MBAR + 0x48;
pub const MCFSIM_CSOR1: usize = MCF_MBAR + 0x4c;
pub const MCFSIM_CSBR2: usize = MCF_MBAR + 0x50;
pub const MCFSIM_CSOR2: usize = MCF_MBAR + 0x54;
pub const MCFSIM_CSBR3: usize = MCF_MBAR + 0x58;
pub const MCFSIM_CSOR3: usize = MCF_MBAR + 0x5c;
pub const MCFSIM_CSBR4: usize = MCF_MBAR + 0x60;
pub const MCFSIM_CSOR4: usize = MCF_MBAR + 0x64;
pub const MCFSIM_CSBR5: usize = MCF_MBAR + 0x68;
pub const MCFSIM_CSOR5: usize = MCF_MBAR + 0x6c;
pub const MCFSIM_CSBR6: usize = MCF_MBAR + 0x70;
pub const MCFSIM_CSOR6: usize = MCF_MBAR + 0x74;
pub const MCFSIM_CSBR7: usize = MCF_MBAR + 0x78;
pub const MCFSIM_CSOR7: usize = MCF_MBAR + 0x7c;

pub const MCFSIM_SDCR: usize = MCF_MBAR + 0x180;
pub const MCFSIM_SDTR: usize = MCF_MBAR + 0x184;
pub const MCFSIM_DCAR0: usize = MCF_MBAR + 0x4c;
pub const MCFSIM_DCMR0: usize = MCF_MBAR + 0x50;
pub const MCFSIM_DCCR0: usize = MCF_MBAR + 0x57;
pub const MCFSIM_DCAR1: usize = MCF_MBAR + 0x58;
pub const MCFSIM_DCMR1: usize = MCF_MBAR + 0x5c;
pub const MCFSIM_DCCR1: usize = MCF_MBAR + 0x63;
pub const MCFUART_BASE0: usize = MCF_MBAR + 0x100;
pub const MCFUART_BASE1: usize = MCF_MBAR + 0x140;
pub const MCFSIM_PACNT: usize = MCF_MBAR + 0x80;
pub const MCFSIM_PADDR: usize = MCF_MBAR + 0x84;
pub const MCFSIM_PADAT: usize = MCF_MBAR + 0x86;
pub const MCFSIM_PBCNT: usize = MCF_MBAR + 0x88;
pub const MCFSIM_PBDDR: usize = MCF_MBAR + 0x8c;
pub const MCFSIM_PBDAT: usize = MCF_MBAR + 0x8e;
pub const MCFSIM_PCDDR: usize = MCF_MBAR + 0x94;
pub const MCFSIM_PCDAT: usize = MCF_MBAR + 0x96;
pub const MCFSIM_PDCNT: usize = MCF_MBAR + 0x98;
pub const MCFDMA_BASE0: usize = MCF_MBAR + 0xe0;
pub const MCFTIMER_BASE1: usize = MCF_MBAR + 0x200;
pub const MCFTIMER_BASE2: usize = MCF_MBAR + 0x220;
pub const MCFTIMER_BASE3: usize = MCF_MBAR + 0x240;
pub const MCFTIMER_BASE4: usize = MCF_MBAR + 0x260;
pub const MCFFEC_BASE0: usize = MCF_MBAR + 0x840;
pub const MCFFEC_SIZE0: i32 = 0x1d0;

/* Define system peripheral IRQ usage. */
pub const MCFINT_VECBASE: i32 = 64;
pub const MCF_IRQ_SPURIOUS: i32 = 64;
pub const MCF_IRQ_EINT1: i32 = 65;
pub const MCF_IRQ_EINT2: i32 = 66;
pub const MCF_IRQ_EINT3: i32 = 67;
pub const MCF_IRQ_EINT4: i32 = 68;
pub const MCF_IRQ_TIMER1: i32 = 69;
pub const MCF_IRQ_TIMER2: i32 = 70;
pub const MCF_IRQ_TIMER3: i32 = 71;
pub const MCF_IRQ_TIMER4: i32 = 72;
pub const MCF_IRQ_UART0: i32 = 73;
pub const MCF_IRQ_UART1: i32 = 74;
pub const MCF_IRQ_PLIP: i32 = 75;
pub const MCF_IRQ_PLIA: i32 = 76;
pub const MCF_IRQ_USB0: i32 = 77;
pub const MCF_IRQ_USB1: i32 = 78;
pub const MCF_IRQ_USB2: i32 = 79;
pub const MCF_IRQ_USB3: i32 = 80;
pub const MCF_IRQ_USB4: i32 = 81;
pub const MCF_IRQ_USB5: i32 = 82;
pub const MCF_IRQ_USB6: i32 = 83;
pub const MCF_IRQ_USB7: i32 = 84;
pub const MCF_IRQ_DMA: i32 = 85;
pub const MCF_IRQ_FECRX0: i32 = 86;
pub const MCF_IRQ_FECTX0: i32 = 87;
pub const MCF_IRQ_FECENTC0: i32 = 88;
pub const MCF_IRQ_QSPI: i32 = 89;
pub const MCF_IRQ_EINT5: i32 = 90;
pub const MCF_IRQ_EINT6: i32 = 91;
pub const MCF_IRQ_SWTO: i32 = 92;
pub const MCFINT_VECMAX: i32 = 95;
pub const MCF_IRQ_TIMER: i32 = MCF_IRQ_TIMER1;
pub const MCF_IRQ_PROFILER: i32 = MCF_IRQ_TIMER2;

/* Generic GPIO support */
pub const MCFGPIO_PIN_MAX: i32 = 48;
pub const MCFGPIO_IRQ_MAX: i32 = -1;
pub const MCFGPIO_IRQ_VECBASE: i32 = -1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
