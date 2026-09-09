/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	m54xxsim.h -- ColdFire 547x/548x System Integration Unit support.
 */

// #include <asm/m54xxacr.h>

pub const CPU_NAME: &str = "COLDFIRE(m54xx)";
pub const CPU_INSTR_PER_JIFFY: i32 = 2;
pub const MCF_BUSCLK: i32 = MCF_CLK / 2;
pub const MACHINE: _ = MACH_M54XX;
pub const FPUTYPE: _ = FPU_COLDFIRE;
pub const IOMEMBASE: _ = MCF_MBAR;
pub const IOMEMSIZE: i32 = 0x01000000;

pub const MCFINT_VECBASE: i32 = 64;

/*
 *      Interrupt Controller Registers
 */
pub const MCFICM_INTC0: _ = MCF_MBAR + 0x700; // Base for Interrupt Ctrl 0

pub const MCFINTC_IPRH: i32 = 0x00; // Interrupt pending 32-63
pub const MCFINTC_IPRL: i32 = 0x04; // Interrupt pending 1-31
pub const MCFINTC_IMRH: i32 = 0x08; // Interrupt mask 32-63
pub const MCFINTC_IMRL: i32 = 0x0c; // Interrupt mask 1-31
pub const MCFINTC_INTFRCH: i32 = 0x10; // Interrupt force 32-63
pub const MCFINTC_INTFRCL: i32 = 0x14; // Interrupt force 1-31
pub const MCFINTC_IRLR: i32 = 0x18;
pub const MCFINTC_IACKL: i32 = 0x19;
pub const MCFINTC_ICR0: i32 = 0x40; // Base ICR register

/*
 *	UART module.
 */
pub const MCFUART_BASE0: _ = MCF_MBAR + 0x8600; // Base address UART0
pub const MCFUART_BASE1: _ = MCF_MBAR + 0x8700; // Base address UART1
pub const MCFUART_BASE2: _ = MCF_MBAR + 0x8800; // Base address UART2
pub const MCFUART_BASE3: _ = MCF_MBAR + 0x8900; // Base address UART3

/*
 *	Define system peripheral IRQ usage.
 */
pub const MCF_IRQ_TIMER: i32 = MCFINT_VECBASE + 54; // Slice Timer 0
pub const MCF_IRQ_PROFILER: i32 = MCFINT_VECBASE + 53; // Slice Timer 1
pub const MCF_IRQ_I2C0: i32 = MCFINT_VECBASE + 40;
pub const MCF_IRQ_UART0: i32 = MCFINT_VECBASE + 35;
pub const MCF_IRQ_UART1: i32 = MCFINT_VECBASE + 34;
pub const MCF_IRQ_UART2: i32 = MCFINT_VECBASE + 33;
pub const MCF_IRQ_UART3: i32 = MCFINT_VECBASE + 32;

/*
 *	Slice Timer support.
 */
pub const MCFSLT_TIMER0: _ = MCF_MBAR + 0x900; // Base addr TIMER0
pub const MCFSLT_TIMER1: _ = MCF_MBAR + 0x910; // Base addr TIMER1

/*
 *	Generic GPIO support
 */
pub const MCFGPIO_PODR: _ = MCF_MBAR + 0xA00;
pub const MCFGPIO_PDDR: _ = MCF_MBAR + 0xA10;
pub const MCFGPIO_PPDR: _ = MCF_MBAR + 0xA20;
pub const MCFGPIO_SETR: _ = MCF_MBAR + 0xA20;
pub const MCFGPIO_CLRR: _ = MCF_MBAR + 0xA30;

pub const MCFGPIO_PIN_MAX: i32 = 136; // 128 gpio + 8 eport
pub const MCFGPIO_IRQ_MAX: i32 = 8;
pub const MCFGPIO_IRQ_VECBASE: i32 = MCFINT_VECBASE;

/*
 *	EDGE Port support.
 */
pub const MCFEPORT_EPPAR: _ = MCF_MBAR + 0xf00; // Pin assignment
pub const MCFEPORT_EPDDR: _ = MCF_MBAR + 0xf04; // Data direction
pub const MCFEPORT_EPIER: _ = MCF_MBAR + 0xf05; // Interrupt enable
pub const MCFEPORT_EPDR: _ = MCF_MBAR + 0xf08; // Port data (w)
pub const MCFEPORT_EPPDR: _ = MCF_MBAR + 0xf09; // Port data (r)
pub const MCFEPORT_EPFR: _ = MCF_MBAR + 0xf0c; // Flags

/*
 *	Pin Assignment register definitions
 */
pub const MCFGPIO_PAR_FBCTL: _ = MCF_MBAR + 0xA40;
pub const MCFGPIO_PAR_FBCS: _ = MCF_MBAR + 0xA42;
pub const MCFGPIO_PAR_DMA: _ = MCF_MBAR + 0xA43;
pub const MCFGPIO_PAR_FECI2CIRQ: _ = MCF_MBAR + 0xA44;
pub const MCFGPIO_PAR_PCIBG: _ = MCF_MBAR + 0xA48; // PCI bus grant
pub const MCFGPIO_PAR_PCIBR: _ = MCF_MBAR + 0xA4A; // PCI
pub const MCFGPIO_PAR_PSC0: _ = MCF_MBAR + 0xA4F;
pub const MCFGPIO_PAR_PSC1: _ = MCF_MBAR + 0xA4E;
pub const MCFGPIO_PAR_PSC2: _ = MCF_MBAR + 0xA4D;
pub const MCFGPIO_PAR_PSC3: _ = MCF_MBAR + 0xA4C;
pub const MCFGPIO_PAR_DSPI: _ = MCF_MBAR + 0xA50;
pub const MCFGPIO_PAR_TIMER: _ = MCF_MBAR + 0xA52;

pub const MCF_PAR_SDA: i32 = 0x0008;
pub const MCF_PAR_SCL: i32 = 0x0004;
pub const MCF_PAR_PSC_TXD: i32 = 0x04;
pub const MCF_PAR_PSC_RXD: i32 = 0x08;
pub const MCF_PAR_PSC_CTS_GPIO: i32 = 0x00;
pub const MCF_PAR_PSC_CTS_BCLK: i32 = 0x80;
pub const MCF_PAR_PSC_CTS_CTS: i32 = 0xC0;
pub const MCF_PAR_PSC_RTS_GPIO: i32 = 0x00;
pub const MCF_PAR_PSC_RTS_FSYNC: i32 = 0x20;
pub const MCF_PAR_PSC_RTS_RTS: i32 = 0x30;
pub const MCF_PAR_PSC_CANRX: i32 = 0x40;

pub const MCF_PAR_FECI2CIRQ: _ = MCF_MBAR + 0x00000a44; // FEC/I2C/IRQ
pub const MCF_PAR_FECI2CIRQ_SDA: i32 = 1 << 3;
pub const MCF_PAR_FECI2CIRQ_SCL: i32 = 1 << 2;

/*
 * I2C module.
 */
pub const MCFI2C_BASE0: _ = MCF_MBAR + 0x8f00;
pub const MCFI2C_SIZE0: i32 = 0x40;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
