/* SPDX-License-Identifier: GPL-2.0 */
/*
 * m525xsim.h -- ColdFire 525x System Integration Module support.
 * Rust translation of the C header.
 */

// Dependency supplied by the surrounding translation unit: MCF_MBAR, MCF_CLK.
pub const CPU_NAME: &str = "COLDFIRE(m525x)";
pub const CPU_INSTR_PER_JIFFY: i32 = 3;
pub const MCF_BUSCLK: u32 = MCF_CLK / 2;
pub const MCF_MBAR2: u32 = 0x8000_0000;

pub const MCFSIM_RSR: u32 = MCF_MBAR + 0x00;
pub const MCFSIM_SYPCR: u32 = MCF_MBAR + 0x01;
pub const MCFSIM_SWIVR: u32 = MCF_MBAR + 0x02;
pub const MCFSIM_SWSR: u32 = MCF_MBAR + 0x03;
pub const MCFSIM_MPARK: u32 = MCF_MBAR + 0x0c;
pub const MCFSIM_IPR: u32 = MCF_MBAR + 0x40;
pub const MCFSIM_IMR: u32 = MCF_MBAR + 0x44;

pub const MCFSIM_ICR0: u32 = MCF_MBAR + 0x4c;
pub const MCFSIM_ICR1: u32 = MCF_MBAR + 0x4d;
pub const MCFSIM_ICR2: u32 = MCF_MBAR + 0x4e;
pub const MCFSIM_ICR3: u32 = MCF_MBAR + 0x4f;
pub const MCFSIM_ICR4: u32 = MCF_MBAR + 0x50;
pub const MCFSIM_ICR5: u32 = MCF_MBAR + 0x51;
pub const MCFSIM_ICR6: u32 = MCF_MBAR + 0x52;
pub const MCFSIM_ICR7: u32 = MCF_MBAR + 0x53;
pub const MCFSIM_ICR8: u32 = MCF_MBAR + 0x54;
pub const MCFSIM_ICR9: u32 = MCF_MBAR + 0x55;
pub const MCFSIM_ICR10: u32 = MCF_MBAR + 0x56;
pub const MCFSIM_ICR11: u32 = MCF_MBAR + 0x57;

pub const MCFSIM_CSAR0: u32 = MCF_MBAR + 0x80;
pub const MCFSIM_CSMR0: u32 = MCF_MBAR + 0x84;
pub const MCFSIM_CSCR0: u32 = MCF_MBAR + 0x8a;
pub const MCFSIM_CSAR1: u32 = MCF_MBAR + 0x8c;
pub const MCFSIM_CSMR1: u32 = MCF_MBAR + 0x90;
pub const MCFSIM_CSCR1: u32 = MCF_MBAR + 0x96;
pub const MCFSIM_CSAR2: u32 = MCF_MBAR + 0x98;
pub const MCFSIM_CSMR2: u32 = MCF_MBAR + 0x9c;
pub const MCFSIM_CSCR2: u32 = MCF_MBAR + 0xa2;
pub const MCFSIM_CSAR3: u32 = MCF_MBAR + 0xa4;
pub const MCFSIM_CSMR3: u32 = MCF_MBAR + 0xa8;
pub const MCFSIM_CSCR3: u32 = MCF_MBAR + 0xae;
pub const MCFSIM_CSAR4: u32 = MCF_MBAR + 0xb0;
pub const MCFSIM_CSMR4: u32 = MCF_MBAR + 0xb4;
pub const MCFSIM_CSCR4: u32 = MCF_MBAR + 0xba;

pub const MCFSIM_DCR: u32 = MCF_MBAR + 0x100;
pub const MCFSIM_DACR0: u32 = MCF_MBAR + 0x108;
pub const MCFSIM_DMR0: u32 = MCF_MBAR + 0x10c;
pub const MCFSIM_DACR1: u32 = MCF_MBAR + 0x110;
pub const MCFSIM_DMR1: u32 = MCF_MBAR + 0x114;

pub const MCFINTC2_INTBASE: u32 = MCF_MBAR2 + 0x168;
pub const MCFINTC2_INTPRI1: u32 = MCF_MBAR2 + 0x140;
pub const MCFINTC2_INTPRI2: u32 = MCF_MBAR2 + 0x144;
pub const MCFINTC2_INTPRI3: u32 = MCF_MBAR2 + 0x148;
pub const MCFINTC2_INTPRI4: u32 = MCF_MBAR2 + 0x14c;
pub const MCFINTC2_INTPRI5: u32 = MCF_MBAR2 + 0x150;
pub const MCFINTC2_INTPRI6: u32 = MCF_MBAR2 + 0x154;
pub const MCFINTC2_INTPRI7: u32 = MCF_MBAR2 + 0x158;
pub const MCFINTC2_INTPRI8: u32 = MCF_MBAR2 + 0x15c;
pub const fn mcfintc2_intpri_reg(i: i32) -> u32 { MCFINTC2_INTPRI1 + (((i - MCFINTC2_VECBASE) / 8) * 4) as u32 }
pub const fn mcfintc2_intpri_bits(b: u32, i: i32) -> u32 { b << (((i % 8) * 4) as u32) }

pub const MCFTIMER_BASE1: u32 = MCF_MBAR + 0x140;
pub const MCFTIMER_BASE2: u32 = MCF_MBAR + 0x180;
pub const MCFUART_BASE0: u32 = MCF_MBAR + 0x1c0;
pub const MCFUART_BASE1: u32 = MCF_MBAR + 0x200;
pub const MCFQSPI_BASE: u32 = MCF_MBAR + 0x400;
pub const MCFQSPI_SIZE: u32 = 0x40;

// CONFIG_M5249 is a build-time condition; the alternatives preserve the C preprocessor branches.
#[cfg(feature = "CONFIG_M5249")]
pub const MCFQSPI_CS0: i32 = 29;
#[cfg(feature = "CONFIG_M5249")]
pub const MCFQSPI_CS1: i32 = 24;
#[cfg(feature = "CONFIG_M5249")]
pub const MCFQSPI_CS2: i32 = 21;
#[cfg(feature = "CONFIG_M5249")]
pub const MCFQSPI_CS3: i32 = 22;
#[cfg(not(feature = "CONFIG_M5249"))]
pub const MCFQSPI_CS0: i32 = 15;
#[cfg(not(feature = "CONFIG_M5249"))]
pub const MCFQSPI_CS1: i32 = 16;
#[cfg(not(feature = "CONFIG_M5249"))]
pub const MCFQSPI_CS2: i32 = 24;
#[cfg(not(feature = "CONFIG_M5249"))]
pub const MCFQSPI_CS3: i32 = 28;

pub const MCFI2C_BASE0: u32 = MCF_MBAR + 0x280;
pub const MCFI2C_SIZE0: u32 = 0x20;
pub const MCFI2C_BASE1: u32 = MCF_MBAR2 + 0x440;
pub const MCFI2C_SIZE1: u32 = 0x20;
pub const MCFDMA_BASE0: u32 = MCF_MBAR + 0x300;
pub const MCFDMA_BASE1: u32 = MCF_MBAR + 0x340;
pub const MCFDMA_BASE2: u32 = MCF_MBAR + 0x380;
pub const MCFDMA_BASE3: u32 = MCF_MBAR + 0x3c0;

pub const MCFSIM_SWDICR: u32 = MCFSIM_ICR0;
pub const MCFSIM_TIMER1ICR: u32 = MCFSIM_ICR1;
pub const MCFSIM_TIMER2ICR: u32 = MCFSIM_ICR2;
pub const MCFSIM_I2CICR: u32 = MCFSIM_ICR3;
pub const MCFSIM_UART1ICR: u32 = MCFSIM_ICR4;
pub const MCFSIM_UART2ICR: u32 = MCFSIM_ICR5;
pub const MCFSIM_DMA0ICR: u32 = MCFSIM_ICR6;
pub const MCFSIM_DMA1ICR: u32 = MCFSIM_ICR7;
pub const MCFSIM_DMA2ICR: u32 = MCFSIM_ICR8;
pub const MCFSIM_DMA3ICR: u32 = MCFSIM_ICR9;
pub const MCFSIM_QSPIICR: u32 = MCFSIM_ICR10;

pub const MCF_IRQ_QSPI: i32 = 28;
pub const MCF_IRQ_I2C0: i32 = 29;
pub const MCF_IRQ_TIMER: i32 = 30;
pub const MCF_IRQ_PROFILER: i32 = 31;
pub const MCF_IRQ_UART0: i32 = 73;
pub const MCF_IRQ_UART1: i32 = 74;
pub const MCFINTC2_VECBASE: i32 = 128;
pub const MCF_IRQ_GPIO0: i32 = MCFINTC2_VECBASE + 32;
pub const MCF_IRQ_GPIO1: i32 = MCFINTC2_VECBASE + 33;
pub const MCF_IRQ_GPIO2: i32 = MCFINTC2_VECBASE + 34;
pub const MCF_IRQ_GPIO3: i32 = MCFINTC2_VECBASE + 35;
pub const MCF_IRQ_GPIO4: i32 = MCFINTC2_VECBASE + 36;
pub const MCF_IRQ_GPIO5: i32 = MCFINTC2_VECBASE + 37;
pub const MCF_IRQ_GPIO6: i32 = MCFINTC2_VECBASE + 38;
pub const MCF_IRQ_GPIO7: i32 = MCFINTC2_VECBASE + 39;
pub const MCF_IRQ_USBWUP: i32 = MCFINTC2_VECBASE + 40;
pub const MCF_IRQ_I2C1: i32 = MCFINTC2_VECBASE + 62;

pub const MCFSIM2_GPIOREAD: u32 = MCF_MBAR2 + 0x000;
pub const MCFSIM2_GPIOWRITE: u32 = MCF_MBAR2 + 0x004;
pub const MCFSIM2_GPIOENABLE: u32 = MCF_MBAR2 + 0x008;
pub const MCFSIM2_GPIOFUNC: u32 = MCF_MBAR2 + 0x00c;
pub const MCFSIM2_GPIO1READ: u32 = MCF_MBAR2 + 0x0b0;
pub const MCFSIM2_GPIO1WRITE: u32 = MCF_MBAR2 + 0x0b4;
pub const MCFSIM2_GPIO1ENABLE: u32 = MCF_MBAR2 + 0x0b8;
pub const MCFSIM2_GPIO1FUNC: u32 = MCF_MBAR2 + 0x0bc;
pub const MCFSIM2_GPIOINTSTAT: u32 = MCF_MBAR2 + 0xc0;
pub const MCFSIM2_GPIOINTCLEAR: u32 = MCF_MBAR2 + 0xc0;
pub const MCFSIM2_GPIOINTENABLE: u32 = MCF_MBAR2 + 0xc4;
pub const MCFSIM2_DMAROUTE: u32 = MCF_MBAR2 + 0x188;
pub const MCFSIM2_IDECONFIG1: u32 = MCF_MBAR2 + 0x18c;
pub const MCFSIM2_IDECONFIG2: u32 = MCF_MBAR2 + 0x190;
pub const MCFGPIO_PIN_MAX: i32 = 64;

#[cfg(feature = "CONFIG_M5249")]
pub const MCFGPIO_IRQ_MAX: i32 = -1;
#[cfg(feature = "CONFIG_M5249")]
pub const MCFGPIO_IRQ_VECBASE: i32 = -1;
#[cfg(not(feature = "CONFIG_M5249"))]
pub const MCFGPIO_IRQ_MAX: i32 = 7;
#[cfg(not(feature = "CONFIG_M5249"))]
pub const MCFGPIO_IRQ_VECBASE: i32 = MCF_IRQ_GPIO0;

// The __ASSEMBLER__ section defines the CONFIG_M5249C3-only m5249c3_setup
// assembly macro (including MMIO setup, PLL programming, chip-select setup,
// IDE configuration, and GPIO reset de-assertion). It has no direct Rust item;
// PLATFORM_SETUP aliases that assembler macro when CONFIG_M5249C3 is enabled.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
