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
pub const MCF_BUSCLK: _ = MCF_CLK;

/* Define the 5280/5282 SIM register set addresses. */
pub const MCFICM_INTC0: _ = MCF_IPSBAR + 0x0c00; // Base for Interrupt Ctrl 0
pub const MCFICM_INTC1: _ = MCF_IPSBAR + 0x0d00; // Base for Interrupt Ctrl 0
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
pub const MCFSIM_DCR: _ = MCF_IPSBAR + 0x44;
pub const MCFSIM_DACR0: _ = MCF_IPSBAR + 0x48;
pub const MCFSIM_DMR0: _ = MCF_IPSBAR + 0x4c;
pub const MCFSIM_DACR1: _ = MCF_IPSBAR + 0x50;
pub const MCFSIM_DMR1: _ = MCF_IPSBAR + 0x54;

/* DMA unit base addresses. */
pub const MCFDMA_BASE0: _ = MCF_IPSBAR + 0x100;
pub const MCFDMA_BASE1: _ = MCF_IPSBAR + 0x140;
pub const MCFDMA_BASE2: _ = MCF_IPSBAR + 0x180;
pub const MCFDMA_BASE3: _ = MCF_IPSBAR + 0x1c0;
/* UART module. */
pub const MCFUART_BASE0: _ = MCF_IPSBAR + 0x200;
pub const MCFUART_BASE1: _ = MCF_IPSBAR + 0x240;
pub const MCFUART_BASE2: _ = MCF_IPSBAR + 0x280;
/* FEC ethernet module. */
pub const MCFFEC_BASE0: _ = MCF_IPSBAR + 0x1000;
pub const MCFFEC_SIZE0: i32 = 0x800;
/* QSPI module. */
pub const MCFQSPI_BASE: _ = MCF_IPSBAR + 0x340;
pub const MCFQSPI_SIZE: i32 = 0x40;
pub const MCFQSPI_CS0: i32 = 147;
pub const MCFQSPI_CS1: i32 = 148;
pub const MCFQSPI_CS2: i32 = 149;
pub const MCFQSPI_CS3: i32 = 150;

/* GPIO registers. */
pub const MCFGPIO_PODR_A: _ = MCF_IPSBAR + 0x100000;
pub const MCFGPIO_PODR_B: _ = MCF_IPSBAR + 0x100001;
pub const MCFGPIO_PODR_C: _ = MCF_IPSBAR + 0x100002;
pub const MCFGPIO_PODR_D: _ = MCF_IPSBAR + 0x100003;
pub const MCFGPIO_PODR_E: _ = MCF_IPSBAR + 0x100004;
pub const MCFGPIO_PODR_F: _ = MCF_IPSBAR + 0x100005;
pub const MCFGPIO_PODR_G: _ = MCF_IPSBAR + 0x100006;
pub const MCFGPIO_PODR_H: _ = MCF_IPSBAR + 0x100007;
pub const MCFGPIO_PODR_J: _ = MCF_IPSBAR + 0x100008;
pub const MCFGPIO_PODR_DD: _ = MCF_IPSBAR + 0x100009;
pub const MCFGPIO_PODR_EH: _ = MCF_IPSBAR + 0x10000A;
pub const MCFGPIO_PODR_EL: _ = MCF_IPSBAR + 0x10000B;
pub const MCFGPIO_PODR_AS: _ = MCF_IPSBAR + 0x10000C;
pub const MCFGPIO_PODR_QS: _ = MCF_IPSBAR + 0x10000D;
pub const MCFGPIO_PODR_SD: _ = MCF_IPSBAR + 0x10000E;
pub const MCFGPIO_PODR_TC: _ = MCF_IPSBAR + 0x10000F;
pub const MCFGPIO_PODR_TD: _ = MCF_IPSBAR + 0x100010;
pub const MCFGPIO_PODR_UA: _ = MCF_IPSBAR + 0x100011;
pub const MCFGPIO_PDDR_A: _ = MCF_IPSBAR + 0x100014;
pub const MCFGPIO_PDDR_B: _ = MCF_IPSBAR + 0x100015;
pub const MCFGPIO_PDDR_C: _ = MCF_IPSBAR + 0x100016;
pub const MCFGPIO_PDDR_D: _ = MCF_IPSBAR + 0x100017;
pub const MCFGPIO_PDDR_E: _ = MCF_IPSBAR + 0x100018;
pub const MCFGPIO_PDDR_F: _ = MCF_IPSBAR + 0x100019;
pub const MCFGPIO_PDDR_G: _ = MCF_IPSBAR + 0x10001A;
pub const MCFGPIO_PDDR_H: _ = MCF_IPSBAR + 0x10001B;
pub const MCFGPIO_PDDR_J: _ = MCF_IPSBAR + 0x10001C;
pub const MCFGPIO_PDDR_DD: _ = MCF_IPSBAR + 0x10001D;
pub const MCFGPIO_PDDR_EH: _ = MCF_IPSBAR + 0x10001E;
pub const MCFGPIO_PDDR_EL: _ = MCF_IPSBAR + 0x10001F;
pub const MCFGPIO_PDDR_AS: _ = MCF_IPSBAR + 0x100020;
pub const MCFGPIO_PDDR_QS: _ = MCF_IPSBAR + 0x100021;
pub const MCFGPIO_PDDR_SD: _ = MCF_IPSBAR + 0x100022;
pub const MCFGPIO_PDDR_TC: _ = MCF_IPSBAR + 0x100023;
pub const MCFGPIO_PDDR_TD: _ = MCF_IPSBAR + 0x100024;
pub const MCFGPIO_PDDR_UA: _ = MCF_IPSBAR + 0x100025;
pub const MCFGPIO_PPDSDR_A: _ = MCF_IPSBAR + 0x100028;
pub const MCFGPIO_PPDSDR_B: _ = MCF_IPSBAR + 0x100029;
pub const MCFGPIO_PPDSDR_C: _ = MCF_IPSBAR + 0x10002A;
pub const MCFGPIO_PPDSDR_D: _ = MCF_IPSBAR + 0x10002B;
pub const MCFGPIO_PPDSDR_E: _ = MCF_IPSBAR + 0x10002C;
pub const MCFGPIO_PPDSDR_F: _ = MCF_IPSBAR + 0x10002D;
pub const MCFGPIO_PPDSDR_G: _ = MCF_IPSBAR + 0x10002E;
pub const MCFGPIO_PPDSDR_H: _ = MCF_IPSBAR + 0x10002F;
pub const MCFGPIO_PPDSDR_J: _ = MCF_IPSBAR + 0x100030;
pub const MCFGPIO_PPDSDR_DD: _ = MCF_IPSBAR + 0x100031;
pub const MCFGPIO_PPDSDR_EH: _ = MCF_IPSBAR + 0x100032;
pub const MCFGPIO_PPDSDR_EL: _ = MCF_IPSBAR + 0x100033;
pub const MCFGPIO_PPDSDR_AS: _ = MCF_IPSBAR + 0x100034;
pub const MCFGPIO_PPDSDR_QS: _ = MCF_IPSBAR + 0x100035;
pub const MCFGPIO_PPDSDR_SD: _ = MCF_IPSBAR + 0x100036;
pub const MCFGPIO_PPDSDR_TC: _ = MCF_IPSBAR + 0x100037;
pub const MCFGPIO_PPDSDR_TD: _ = MCF_IPSBAR + 0x100038;
pub const MCFGPIO_PPDSDR_UA: _ = MCF_IPSBAR + 0x100039;
pub const MCFGPIO_PCLRR_A: _ = MCF_IPSBAR + 0x10003C;
pub const MCFGPIO_PCLRR_B: _ = MCF_IPSBAR + 0x10003D;
pub const MCFGPIO_PCLRR_C: _ = MCF_IPSBAR + 0x10003E;
pub const MCFGPIO_PCLRR_D: _ = MCF_IPSBAR + 0x10003F;
pub const MCFGPIO_PCLRR_E: _ = MCF_IPSBAR + 0x100040;
pub const MCFGPIO_PCLRR_F: _ = MCF_IPSBAR + 0x100041;
pub const MCFGPIO_PCLRR_G: _ = MCF_IPSBAR + 0x100042;
pub const MCFGPIO_PCLRR_H: _ = MCF_IPSBAR + 0x100043;
pub const MCFGPIO_PCLRR_J: _ = MCF_IPSBAR + 0x100044;
pub const MCFGPIO_PCLRR_DD: _ = MCF_IPSBAR + 0x100045;
pub const MCFGPIO_PCLRR_EH: _ = MCF_IPSBAR + 0x100046;
pub const MCFGPIO_PCLRR_EL: _ = MCF_IPSBAR + 0x100047;
pub const MCFGPIO_PCLRR_AS: _ = MCF_IPSBAR + 0x100048;
pub const MCFGPIO_PCLRR_QS: _ = MCF_IPSBAR + 0x100049;
pub const MCFGPIO_PCLRR_SD: _ = MCF_IPSBAR + 0x10004A;
pub const MCFGPIO_PCLRR_TC: _ = MCF_IPSBAR + 0x10004B;
pub const MCFGPIO_PCLRR_TD: _ = MCF_IPSBAR + 0x10004C;
pub const MCFGPIO_PCLRR_UA: _ = MCF_IPSBAR + 0x10004D;
pub const MCFGPIO_PBCDPAR: _ = MCF_IPSBAR + 0x100050;
pub const MCFGPIO_PFPAR: _ = MCF_IPSBAR + 0x100051;
pub const MCFGPIO_PEPAR: _ = MCF_IPSBAR + 0x100052;
pub const MCFGPIO_PJPAR: _ = MCF_IPSBAR + 0x100054;
pub const MCFGPIO_PSDPAR: _ = MCF_IPSBAR + 0x100055;
pub const MCFGPIO_PASPAR: _ = MCF_IPSBAR + 0x100056;
pub const MCFGPIO_PEHLPAR: _ = MCF_IPSBAR + 0x100058;
pub const MCFGPIO_PQSPAR: _ = MCF_IPSBAR + 0x100059;
pub const MCFGPIO_PTCPAR: _ = MCF_IPSBAR + 0x10005A;
pub const MCFGPIO_PTDPAR: _ = MCF_IPSBAR + 0x10005B;
pub const MCFGPIO_PUAPAR: _ = MCF_IPSBAR + 0x10005C;

/* PIT timer base addresses. */
pub const MCFPIT_BASE1: _ = MCF_IPSBAR + 0x150000;
pub const MCFPIT_BASE2: _ = MCF_IPSBAR + 0x160000;
pub const MCFPIT_BASE3: _ = MCF_IPSBAR + 0x170000;
pub const MCFPIT_BASE4: _ = MCF_IPSBAR + 0x180000;
/* Edge Port registers. */
pub const MCFEPORT_EPPAR: _ = MCF_IPSBAR + 0x130000;
pub const MCFEPORT_EPDDR: _ = MCF_IPSBAR + 0x130002;
pub const MCFEPORT_EPIER: _ = MCF_IPSBAR + 0x130003;
pub const MCFEPORT_EPDR: _ = MCF_IPSBAR + 0x130004;
pub const MCFEPORT_EPPDR: _ = MCF_IPSBAR + 0x130005;
pub const MCFEPORT_EPFR: _ = MCF_IPSBAR + 0x130006;
/* Queued ADC registers. */
pub const MCFQADC_PORTQA: _ = MCF_IPSBAR + 0x190006;
pub const MCFQADC_PORTQB: _ = MCF_IPSBAR + 0x190007;
pub const MCFQADC_DDRQA: _ = MCF_IPSBAR + 0x190008;
pub const MCFQADC_DDRQB: _ = MCF_IPSBAR + 0x190009;
/* General Purpose Timers registers. */
pub const MCFGPTA_GPTPORT: _ = MCF_IPSBAR + 0x1A001D;
pub const MCFGPTA_GPTDDR: _ = MCF_IPSBAR + 0x1A001E;
pub const MCFGPTB_GPTPORT: _ = MCF_IPSBAR + 0x1B001D;
pub const MCFGPTB_GPTDDR: _ = MCF_IPSBAR + 0x1B001E;

/* definitions for generic gpio support */
pub const MCFGPIO_PODR: _ = MCFGPIO_PODR_A;
pub const MCFGPIO_PDDR: _ = MCFGPIO_PDDR_A;
pub const MCFGPIO_PPDR: _ = MCFGPIO_PPDSDR_A;
pub const MCFGPIO_SETR: _ = MCFGPIO_PPDSDR_A;
pub const MCFGPIO_CLRR: _ = MCFGPIO_PCLRR_A;
pub const MCFGPIO_IRQ_MAX: i32 = 8;
pub const MCFGPIO_IRQ_VECBASE: i32 = MCFINT_VECBASE;
pub const MCFGPIO_PIN_MAX: i32 = 180;

/* Reset Control Unit (relative to IPSBAR). */
pub const MCF_RCR: _ = MCF_IPSBAR + 0x110000;
pub const MCF_RSR: _ = MCF_IPSBAR + 0x110001;
pub const MCF_RCR_SWRESET: i32 = 0x80; // Software reset bit
pub const MCF_RCR_FRCSTOUT: i32 = 0x40; // Force external reset
/* I2C module */
pub const MCFI2C_BASE0: _ = MCF_IPSBAR + 0x300;
pub const MCFI2C_SIZE0: i32 = 0x40;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
