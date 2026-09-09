/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 *  m520xsim.h -- ColdFire 5207/5208 System Integration Module support.
 *
 *  (C) Copyright 2005, Intec Automation (mike@steroidmicros.com)
 */

/****************************************************************************/
/* C header guard: m520xsim_h */
/****************************************************************************/

pub const CPU_NAME: &str = "COLDFIRE(m520x)";
pub const CPU_INSTR_PER_JIFFY: i32 = 3;
// Supplied by the corresponding clock header/dependency.
pub const MCF_BUSCLK: i32 = MCF_CLK / 2;

// Dependency supplied by <asm/m52xxacr.h>.

/*
 *  Define the 520x SIM register set addresses.
 */
pub const MCFICM_INTC0: u32 = 0xFC048000; /* Base for Interrupt Ctrl 0 */
pub const MCFINTC_IPRH: u32 = 0x00; /* Interrupt pending 32-63 */
pub const MCFINTC_IPRL: u32 = 0x04; /* Interrupt pending 1-31 */
pub const MCFINTC_IMRH: u32 = 0x08; /* Interrupt mask 32-63 */
pub const MCFINTC_IMRL: u32 = 0x0c; /* Interrupt mask 1-31 */
pub const MCFINTC_INTFRCH: u32 = 0x10; /* Interrupt force 32-63 */
pub const MCFINTC_INTFRCL: u32 = 0x14; /* Interrupt force 1-31 */
pub const MCFINTC_SIMR: u32 = 0x1c; /* Set interrupt mask 0-63 */
pub const MCFINTC_CIMR: u32 = 0x1d; /* Clear interrupt mask 0-63 */
pub const MCFINTC_ICR0: u32 = 0x40; /* Base ICR register */

/*
 *  The common interrupt controller code just wants to know the absolute
 *  address to the SIMR and CIMR registers (not offsets into IPSBAR).
 *  The 520x family only has a single INTC unit.
 */
pub const MCFINTC0_SIMR: u32 = MCFICM_INTC0 + MCFINTC_SIMR;
pub const MCFINTC0_CIMR: u32 = MCFICM_INTC0 + MCFINTC_CIMR;
pub const MCFINTC0_ICR0: u32 = MCFICM_INTC0 + MCFINTC_ICR0;
pub const MCFINTC1_SIMR: i32 = 0;
pub const MCFINTC1_CIMR: i32 = 0;
pub const MCFINTC1_ICR0: i32 = 0;
pub const MCFINTC2_SIMR: i32 = 0;
pub const MCFINTC2_CIMR: i32 = 0;
pub const MCFINTC2_ICR0: i32 = 0;

pub const MCFINT_VECBASE: i32 = 64;
pub const MCFINT_UART0: i32 = 26; /* Interrupt number for UART0 */
pub const MCFINT_UART1: i32 = 27; /* Interrupt number for UART1 */
pub const MCFINT_UART2: i32 = 28; /* Interrupt number for UART2 */
pub const MCFINT_I2C0: i32 = 30; /* Interrupt number for I2C */
pub const MCFINT_QSPI: i32 = 31; /* Interrupt number for QSPI */
pub const MCFINT_FECRX0: i32 = 36; /* Interrupt number for FEC RX */
pub const MCFINT_FECTX0: i32 = 40; /* Interrupt number for FEC RX */
pub const MCFINT_FECENTC0: i32 = 42; /* Interrupt number for FEC RX */
pub const MCFINT_PIT1: i32 = 4; /* Interrupt number for PIT1 (PIT0 in processor) */

pub const MCF_IRQ_UART0: i32 = MCFINT_VECBASE + MCFINT_UART0;
pub const MCF_IRQ_UART1: i32 = MCFINT_VECBASE + MCFINT_UART1;
pub const MCF_IRQ_UART2: i32 = MCFINT_VECBASE + MCFINT_UART2;
pub const MCF_IRQ_FECRX0: i32 = MCFINT_VECBASE + MCFINT_FECRX0;
pub const MCF_IRQ_FECTX0: i32 = MCFINT_VECBASE + MCFINT_FECTX0;
pub const MCF_IRQ_FECENTC0: i32 = MCFINT_VECBASE + MCFINT_FECENTC0;
pub const MCF_IRQ_QSPI: i32 = MCFINT_VECBASE + MCFINT_QSPI;
pub const MCF_IRQ_PIT1: i32 = MCFINT_VECBASE + MCFINT_PIT1;
pub const MCF_IRQ_I2C0: i32 = MCFINT_VECBASE + MCFINT_I2C0;

/*
 *  SDRAM configuration registers.
 */
pub const MCFSIM_SDMR: u32 = 0xFC0a8000; /* SDRAM Mode/Extended Mode Register */
pub const MCFSIM_SDCR: u32 = 0xFC0a8004; /* SDRAM Control Register */
pub const MCFSIM_SDCFG1: u32 = 0xFC0a8008; /* SDRAM Configuration Register 1 */
pub const MCFSIM_SDCFG2: u32 = 0xFC0a800c; /* SDRAM Configuration Register 2 */
pub const MCFSIM_SDCS0: u32 = 0xFC0a8110; /* SDRAM Chip Select 0 Configuration */
pub const MCFSIM_SDCS1: u32 = 0xFC0a8114; /* SDRAM Chip Select 1 Configuration */

/*
 * EPORT and GPIO registers.
 */
pub const MCFEPORT_EPPAR: u32 = 0xFC088000;
pub const MCFEPORT_EPDDR: u32 = 0xFC088002;
pub const MCFEPORT_EPIER: u32 = 0xFC088003;
pub const MCFEPORT_EPDR: u32 = 0xFC088004;
pub const MCFEPORT_EPPDR: u32 = 0xFC088005;
pub const MCFEPORT_EPFR: u32 = 0xFC088006;

pub const MCFGPIO_PODR_BUSCTL: u32 = 0xFC0A4000;
pub const MCFGPIO_PODR_BE: u32 = 0xFC0A4001;
pub const MCFGPIO_PODR_CS: u32 = 0xFC0A4002;
pub const MCFGPIO_PODR_FECI2C: u32 = 0xFC0A4003;
pub const MCFGPIO_PODR_QSPI: u32 = 0xFC0A4004;
pub const MCFGPIO_PODR_TIMER: u32 = 0xFC0A4005;
pub const MCFGPIO_PODR_UART: u32 = 0xFC0A4006;
pub const MCFGPIO_PODR_FECH: u32 = 0xFC0A4007;
pub const MCFGPIO_PODR_FECL: u32 = 0xFC0A4008;
pub const MCFGPIO_PDDR_BUSCTL: u32 = 0xFC0A400C;
pub const MCFGPIO_PDDR_BE: u32 = 0xFC0A400D;
pub const MCFGPIO_PDDR_CS: u32 = 0xFC0A400E;
pub const MCFGPIO_PDDR_FECI2C: u32 = 0xFC0A400F;
pub const MCFGPIO_PDDR_QSPI: u32 = 0xFC0A4010;
pub const MCFGPIO_PDDR_TIMER: u32 = 0xFC0A4011;
pub const MCFGPIO_PDDR_UART: u32 = 0xFC0A4012;
pub const MCFGPIO_PDDR_FECH: u32 = 0xFC0A4013;
pub const MCFGPIO_PDDR_FECL: u32 = 0xFC0A4014;
pub const MCFGPIO_PPDSDR_CS: u32 = 0xFC0A401A;
pub const MCFGPIO_PPDSDR_FECI2C: u32 = 0xFC0A401B;
pub const MCFGPIO_PPDSDR_QSPI: u32 = 0xFC0A401C;
pub const MCFGPIO_PPDSDR_TIMER: u32 = 0xFC0A401D;
pub const MCFGPIO_PPDSDR_UART: u32 = 0xFC0A401E;
pub const MCFGPIO_PPDSDR_FECH: u32 = 0xFC0A401F;
pub const MCFGPIO_PPDSDR_FECL: u32 = 0xFC0A4020;
pub const MCFGPIO_PCLRR_BUSCTL: u32 = 0xFC0A4024;
pub const MCFGPIO_PCLRR_BE: u32 = 0xFC0A4025;
pub const MCFGPIO_PCLRR_CS: u32 = 0xFC0A4026;
pub const MCFGPIO_PCLRR_FECI2C: u32 = 0xFC0A4027;
pub const MCFGPIO_PCLRR_QSPI: u32 = 0xFC0A4028;
pub const MCFGPIO_PCLRR_TIMER: u32 = 0xFC0A4029;
pub const MCFGPIO_PCLRR_UART: u32 = 0xFC0A402A;
pub const MCFGPIO_PCLRR_FECH: u32 = 0xFC0A402B;
pub const MCFGPIO_PCLRR_FECL: u32 = 0xFC0A402C;

/* Generic GPIO support */
pub const MCFGPIO_PODR: u32 = MCFGPIO_PODR_CS;
pub const MCFGPIO_PDDR: u32 = MCFGPIO_PDDR_CS;
pub const MCFGPIO_PPDR: u32 = MCFGPIO_PPDSDR_CS;
pub const MCFGPIO_SETR: u32 = MCFGPIO_PPDSDR_CS;
pub const MCFGPIO_CLRR: u32 = MCFGPIO_PCLRR_CS;
pub const MCFGPIO_PIN_MAX: i32 = 80;
pub const MCFGPIO_IRQ_MAX: i32 = 8;
pub const MCFGPIO_IRQ_VECBASE: i32 = MCFINT_VECBASE;
pub const MCF_GPIO_PAR_UART: u32 = 0xFC0A4036;
pub const MCF_GPIO_PAR_FECI2C: u32 = 0xFC0A4033;
pub const MCF_GPIO_PAR_QSPI: u32 = 0xFC0A4034;
pub const MCF_GPIO_PAR_FEC: u32 = 0xFC0A4038;
pub const MCF_GPIO_PAR_UART_PAR_URXD0: u32 = 0x0001;
pub const MCF_GPIO_PAR_UART_PAR_UTXD0: u32 = 0x0002;
pub const MCF_GPIO_PAR_UART_PAR_URXD1: u32 = 0x0040;
pub const MCF_GPIO_PAR_UART_PAR_UTXD1: u32 = 0x0080;
pub const MCF_GPIO_PAR_FECI2C_PAR_SDA_URXD2: u32 = 0x02;
pub const MCF_GPIO_PAR_FECI2C_PAR_SCL_UTXD2: u32 = 0x04;

/*
 *  PIT timer module.
 */
pub const MCFPIT_BASE1: u32 = 0xFC080000; /* Base address of TIMER1 */
pub const MCFPIT_BASE2: u32 = 0xFC084000; /* Base address of TIMER2 */

/*
 *  UART module.
 */
pub const MCFUART_BASE0: u32 = 0xFC060000; /* Base address of UART0 */
pub const MCFUART_BASE1: u32 = 0xFC064000; /* Base address of UART1 */
pub const MCFUART_BASE2: u32 = 0xFC068000; /* Base address of UART2 */

/*
 *  FEC module.
 */
pub const MCFFEC_BASE0: u32 = 0xFC030000; /* Base of FEC ethernet */
pub const MCFFEC_SIZE0: u32 = 0x800; /* Register set size */

/*
 *  QSPI module.
 */
pub const MCFQSPI_BASE: u32 = 0xFC05C000; /* Base of QSPI module */
pub const MCFQSPI_SIZE: u32 = 0x40; /* Register set size */
pub const MCFQSPI_CS0: i32 = 46;
pub const MCFQSPI_CS1: i32 = 47;
pub const MCFQSPI_CS2: i32 = 27;

/*
 *  Reset Control Unit.
 */
pub const MCF_RCR: u32 = 0xFC0A0000;
pub const MCF_RSR: u32 = 0xFC0A0001;
pub const MCF_RCR_SWRESET: u32 = 0x80; /* Software reset bit */
pub const MCF_RCR_FRCSTOUT: u32 = 0x40; /* Force external reset */

/*
 *  Power Management.
 */
pub const MCFPM_WCR: u32 = 0xfc040013;
pub const MCFPM_PPMSR0: u32 = 0xfc04002c;
pub const MCFPM_PPMCR0: u32 = 0xfc04002d;
pub const MCFPM_PPMHR0: u32 = 0xfc040030;
pub const MCFPM_PPMLR0: u32 = 0xfc040034;
pub const MCFPM_LPCR: u32 = 0xfc0a0007;

/*
 * I2C module.
 */
pub const MCFI2C_BASE0: u32 = 0xFC058000;
pub const MCFI2C_SIZE0: u32 = 0x40;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
