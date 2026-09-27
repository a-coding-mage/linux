/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/
/*
 * m528xsim.h -- ColdFire 5280/5282 System Integration Module support.
 *
 * (C) Copyright 2003, Greg Ungerer (gerg@snapgear.com)
 */
/****************************************************************************/
// C header guard: m528xsim_h
// Dependency: asm/m52xxacr.h

pub const CPU_NAME: &str = "COLDFIRE(m528x)";
pub const CPU_INSTR_PER_JIFFY: i32 = 3;
pub const MCF_BUSCLK: usize = MCF_CLK;

/* Define the 5280/5282 SIM register set addresses. */
pub const MCFICM_INTC0: usize = MCF_IPSBAR + 0x0c00; // Base for Interrupt Ctrl 0
pub const MCFICM_INTC1: usize = MCF_IPSBAR + 0x0d00; // Base for Interrupt Ctrl 0
pub const MCFINTC_IPRH: i32 = 0x00; // Interrupt pending 32-63
pub const MCFINTC_IPRL: i32 = 0x04; // Interrupt pending 1-31
pub const MCFINTC_IMRH: i32 = 0x08; // Interrupt mask 32-63
pub const MCFINTC_IMRL: i32 = 0x0c; // Interrupt mask 1-31
pub const MCFINTC_INTFRCH: i32 = 0x10; // Interrupt force 32-63
pub const MCFINTC_INTFRCL: i32 = 0x14; // Interrupt force 1-31
pub const MCFINTC_IRLR: i32 = 0x18;
pub const MCFINTC_IACKL: i32 = 0x19;
pub const MCFINTC_ICR0: i32 = 0x40; // Base ICR register
pub const MCFINT_VECBASE: i32 = 64;
pub const MCFINT_UART0: i32 = 13;
pub const MCFINT_UART1: i32 = 14;
pub const MCFINT_UART2: i32 = 15;
pub const MCFINT_I2C0: i32 = 17;
pub const MCFINT_QSPI: i32 = 18;
pub const MCFINT_FECRX0: i32 = 23;
pub const MCFINT_FECTX0: i32 = 27;
pub const MCFINT_FECENTC0: i32 = 29;
pub const MCFINT_PIT1: i32 = 55;
pub const MCF_IRQ_UART0: i32 = MCFINT_VECBASE + MCFINT_UART0;
pub const MCF_IRQ_UART1: i32 = MCFINT_VECBASE + MCFINT_UART1;
pub const MCF_IRQ_UART2: i32 = MCFINT_VECBASE + MCFINT_UART2;
pub const MCF_IRQ_FECRX0: i32 = MCFINT_VECBASE + MCFINT_FECRX0;
pub const MCF_IRQ_FECTX0: i32 = MCFINT_VECBASE + MCFINT_FECTX0;
pub const MCF_IRQ_FECENTC0: i32 = MCFINT_VECBASE + MCFINT_FECENTC0;
pub const MCF_IRQ_QSPI: i32 = MCFINT_VECBASE + MCFINT_QSPI;
pub const MCF_IRQ_PIT1: i32 = MCFINT_VECBASE + MCFINT_PIT1;
pub const MCF_IRQ_I2C0: i32 = MCFINT_VECBASE + MCFINT_I2C0;

/* SDRAM configuration registers. */
pub const MCFSIM_DCR: usize = MCF_IPSBAR + 0x44;
pub const MCFSIM_DACR0: usize = MCF_IPSBAR + 0x48;
pub const MCFSIM_DMR0: usize = MCF_IPSBAR + 0x4c;
pub const MCFSIM_DACR1: usize = MCF_IPSBAR + 0x50;
pub const MCFSIM_DMR1: usize = MCF_IPSBAR + 0x54;

/* DMA unit base addresses. */
pub const MCFDMA_BASE0: usize = MCF_IPSBAR + 0x100;
pub const MCFDMA_BASE1: usize = MCF_IPSBAR + 0x140;
pub const MCFDMA_BASE2: usize = MCF_IPSBAR + 0x180;
pub const MCFDMA_BASE3: usize = MCF_IPSBAR + 0x1c0;
/* UART module. */
pub const MCFUART_BASE0: usize = MCF_IPSBAR + 0x200;
pub const MCFUART_BASE1: usize = MCF_IPSBAR + 0x240;
pub const MCFUART_BASE2: usize = MCF_IPSBAR + 0x280;
/* FEC ethernet module. */
pub const MCFFEC_BASE0: usize = MCF_IPSBAR + 0x1000;
pub const MCFFEC_SIZE0: i32 = 0x800;
/* QSPI module. */
pub const MCFQSPI_BASE: usize = MCF_IPSBAR + 0x340;
pub const MCFQSPI_SIZE: i32 = 0x40;
pub const MCFQSPI_CS0: i32 = 147;
pub const MCFQSPI_CS1: i32 = 148;
pub const MCFQSPI_CS2: i32 = 149;
pub const MCFQSPI_CS3: i32 = 150;

/* GPIO registers. */
pub const MCFGPIO_PODR_A: usize = MCF_IPSBAR + 0x100000;
pub const MCFGPIO_PODR_B: usize = MCF_IPSBAR + 0x100001;
pub const MCFGPIO_PODR_C: usize = MCF_IPSBAR + 0x100002;
pub const MCFGPIO_PODR_D: usize = MCF_IPSBAR + 0x100003;
pub const MCFGPIO_PODR_E: usize = MCF_IPSBAR + 0x100004;
pub const MCFGPIO_PODR_F: usize = MCF_IPSBAR + 0x100005;
pub const MCFGPIO_PODR_G: usize = MCF_IPSBAR + 0x100006;
pub const MCFGPIO_PODR_H: usize = MCF_IPSBAR + 0x100007;
pub const MCFGPIO_PODR_J: usize = MCF_IPSBAR + 0x100008;
pub const MCFGPIO_PODR_DD: usize = MCF_IPSBAR + 0x100009;
pub const MCFGPIO_PODR_EH: usize = MCF_IPSBAR + 0x10000A;
pub const MCFGPIO_PODR_EL: usize = MCF_IPSBAR + 0x10000B;
pub const MCFGPIO_PODR_AS: usize = MCF_IPSBAR + 0x10000C;
pub const MCFGPIO_PODR_QS: usize = MCF_IPSBAR + 0x10000D;
pub const MCFGPIO_PODR_SD: usize = MCF_IPSBAR + 0x10000E;
pub const MCFGPIO_PODR_TC: usize = MCF_IPSBAR + 0x10000F;
pub const MCFGPIO_PODR_TD: usize = MCF_IPSBAR + 0x100010;
pub const MCFGPIO_PODR_UA: usize = MCF_IPSBAR + 0x100011;
pub const MCFGPIO_PDDR_A: usize = MCF_IPSBAR + 0x100014;
pub const MCFGPIO_PDDR_B: usize = MCF_IPSBAR + 0x100015;
pub const MCFGPIO_PDDR_C: usize = MCF_IPSBAR + 0x100016;
pub const MCFGPIO_PDDR_D: usize = MCF_IPSBAR + 0x100017;
pub const MCFGPIO_PDDR_E: usize = MCF_IPSBAR + 0x100018;
pub const MCFGPIO_PDDR_F: usize = MCF_IPSBAR + 0x100019;
pub const MCFGPIO_PDDR_G: usize = MCF_IPSBAR + 0x10001A;
pub const MCFGPIO_PDDR_H: usize = MCF_IPSBAR + 0x10001B;
pub const MCFGPIO_PDDR_J: usize = MCF_IPSBAR + 0x10001C;
pub const MCFGPIO_PDDR_DD: usize = MCF_IPSBAR + 0x10001D;
pub const MCFGPIO_PDDR_EH: usize = MCF_IPSBAR + 0x10001E;
pub const MCFGPIO_PDDR_EL: usize = MCF_IPSBAR + 0x10001F;
pub const MCFGPIO_PDDR_AS: usize = MCF_IPSBAR + 0x100020;
pub const MCFGPIO_PDDR_QS: usize = MCF_IPSBAR + 0x100021;
pub const MCFGPIO_PDDR_SD: usize = MCF_IPSBAR + 0x100022;
pub const MCFGPIO_PDDR_TC: usize = MCF_IPSBAR + 0x100023;
pub const MCFGPIO_PDDR_TD: usize = MCF_IPSBAR + 0x100024;
pub const MCFGPIO_PDDR_UA: usize = MCF_IPSBAR + 0x100025;
pub const MCFGPIO_PPDSDR_A: usize = MCF_IPSBAR + 0x100028;
pub const MCFGPIO_PPDSDR_B: usize = MCF_IPSBAR + 0x100029;
pub const MCFGPIO_PPDSDR_C: usize = MCF_IPSBAR + 0x10002A;
pub const MCFGPIO_PPDSDR_D: usize = MCF_IPSBAR + 0x10002B;
pub const MCFGPIO_PPDSDR_E: usize = MCF_IPSBAR + 0x10002C;
pub const MCFGPIO_PPDSDR_F: usize = MCF_IPSBAR + 0x10002D;
pub const MCFGPIO_PPDSDR_G: usize = MCF_IPSBAR + 0x10002E;
pub const MCFGPIO_PPDSDR_H: usize = MCF_IPSBAR + 0x10002F;
pub const MCFGPIO_PPDSDR_J: usize = MCF_IPSBAR + 0x100030;
pub const MCFGPIO_PPDSDR_DD: usize = MCF_IPSBAR + 0x100031;
pub const MCFGPIO_PPDSDR_EH: usize = MCF_IPSBAR + 0x100032;
pub const MCFGPIO_PPDSDR_EL: usize = MCF_IPSBAR + 0x100033;
pub const MCFGPIO_PPDSDR_AS: usize = MCF_IPSBAR + 0x100034;
pub const MCFGPIO_PPDSDR_QS: usize = MCF_IPSBAR + 0x100035;
pub const MCFGPIO_PPDSDR_SD: usize = MCF_IPSBAR + 0x100036;
pub const MCFGPIO_PPDSDR_TC: usize = MCF_IPSBAR + 0x100037;
pub const MCFGPIO_PPDSDR_TD: usize = MCF_IPSBAR + 0x100038;
pub const MCFGPIO_PPDSDR_UA: usize = MCF_IPSBAR + 0x100039;
pub const MCFGPIO_PCLRR_A: usize = MCF_IPSBAR + 0x10003C;
pub const MCFGPIO_PCLRR_B: usize = MCF_IPSBAR + 0x10003D;
pub const MCFGPIO_PCLRR_C: usize = MCF_IPSBAR + 0x10003E;
pub const MCFGPIO_PCLRR_D: usize = MCF_IPSBAR + 0x10003F;
pub const MCFGPIO_PCLRR_E: usize = MCF_IPSBAR + 0x100040;
pub const MCFGPIO_PCLRR_F: usize = MCF_IPSBAR + 0x100041;
pub const MCFGPIO_PCLRR_G: usize = MCF_IPSBAR + 0x100042;
pub const MCFGPIO_PCLRR_H: usize = MCF_IPSBAR + 0x100043;
pub const MCFGPIO_PCLRR_J: usize = MCF_IPSBAR + 0x100044;
pub const MCFGPIO_PCLRR_DD: usize = MCF_IPSBAR + 0x100045;
pub const MCFGPIO_PCLRR_EH: usize = MCF_IPSBAR + 0x100046;
pub const MCFGPIO_PCLRR_EL: usize = MCF_IPSBAR + 0x100047;
pub const MCFGPIO_PCLRR_AS: usize = MCF_IPSBAR + 0x100048;
pub const MCFGPIO_PCLRR_QS: usize = MCF_IPSBAR + 0x100049;
pub const MCFGPIO_PCLRR_SD: usize = MCF_IPSBAR + 0x10004A;
pub const MCFGPIO_PCLRR_TC: usize = MCF_IPSBAR + 0x10004B;
pub const MCFGPIO_PCLRR_TD: usize = MCF_IPSBAR + 0x10004C;
pub const MCFGPIO_PCLRR_UA: usize = MCF_IPSBAR + 0x10004D;
pub const MCFGPIO_PBCDPAR: usize = MCF_IPSBAR + 0x100050;
pub const MCFGPIO_PFPAR: usize = MCF_IPSBAR + 0x100051;
pub const MCFGPIO_PEPAR: usize = MCF_IPSBAR + 0x100052;
pub const MCFGPIO_PJPAR: usize = MCF_IPSBAR + 0x100054;
pub const MCFGPIO_PSDPAR: usize = MCF_IPSBAR + 0x100055;
pub const MCFGPIO_PASPAR: usize = MCF_IPSBAR + 0x100056;
pub const MCFGPIO_PEHLPAR: usize = MCF_IPSBAR + 0x100058;
pub const MCFGPIO_PQSPAR: usize = MCF_IPSBAR + 0x100059;
pub const MCFGPIO_PTCPAR: usize = MCF_IPSBAR + 0x10005A;
pub const MCFGPIO_PTDPAR: usize = MCF_IPSBAR + 0x10005B;
pub const MCFGPIO_PUAPAR: usize = MCF_IPSBAR + 0x10005C;

/* PIT timer base addresses. */
pub const MCFPIT_BASE1: usize = MCF_IPSBAR + 0x150000;
pub const MCFPIT_BASE2: usize = MCF_IPSBAR + 0x160000;
pub const MCFPIT_BASE3: usize = MCF_IPSBAR + 0x170000;
pub const MCFPIT_BASE4: usize = MCF_IPSBAR + 0x180000;
/* Edge Port registers. */
pub const MCFEPORT_EPPAR: usize = MCF_IPSBAR + 0x130000;
pub const MCFEPORT_EPDDR: usize = MCF_IPSBAR + 0x130002;
pub const MCFEPORT_EPIER: usize = MCF_IPSBAR + 0x130003;
pub const MCFEPORT_EPDR: usize = MCF_IPSBAR + 0x130004;
pub const MCFEPORT_EPPDR: usize = MCF_IPSBAR + 0x130005;
pub const MCFEPORT_EPFR: usize = MCF_IPSBAR + 0x130006;
/* Queued ADC registers. */
pub const MCFQADC_PORTQA: usize = MCF_IPSBAR + 0x190006;
pub const MCFQADC_PORTQB: usize = MCF_IPSBAR + 0x190007;
pub const MCFQADC_DDRQA: usize = MCF_IPSBAR + 0x190008;
pub const MCFQADC_DDRQB: usize = MCF_IPSBAR + 0x190009;
/* General Purpose Timers registers. */
pub const MCFGPTA_GPTPORT: usize = MCF_IPSBAR + 0x1A001D;
pub const MCFGPTA_GPTDDR: usize = MCF_IPSBAR + 0x1A001E;
pub const MCFGPTB_GPTPORT: usize = MCF_IPSBAR + 0x1B001D;
pub const MCFGPTB_GPTDDR: usize = MCF_IPSBAR + 0x1B001E;

/* definitions for generic gpio support */
pub const MCFGPIO_PODR: u32 = MCFGPIO_PODR_A;
pub const MCFGPIO_PDDR: u32 = MCFGPIO_PDDR_A;
pub const MCFGPIO_PPDR: u32 = MCFGPIO_PPDSDR_A;
pub const MCFGPIO_SETR: u32 = MCFGPIO_PPDSDR_A;
pub const MCFGPIO_CLRR: u32 = MCFGPIO_PCLRR_A;
pub const MCFGPIO_IRQ_MAX: i32 = 8;
pub const MCFGPIO_IRQ_VECBASE: i32 = MCFINT_VECBASE;
pub const MCFGPIO_PIN_MAX: i32 = 180;

/* Reset Control Unit (relative to IPSBAR). */
pub const MCF_RCR: usize = MCF_IPSBAR + 0x110000;
pub const MCF_RSR: usize = MCF_IPSBAR + 0x110001;
pub const MCF_RCR_SWRESET: i32 = 0x80; // Software reset bit
pub const MCF_RCR_FRCSTOUT: i32 = 0x40; // Force external reset
/* I2C module */
pub const MCFI2C_BASE0: usize = MCF_IPSBAR + 0x300;
pub const MCFI2C_SIZE0: i32 = 0x40;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
