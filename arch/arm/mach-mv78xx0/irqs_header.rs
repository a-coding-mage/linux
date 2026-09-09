/* SPDX-License-Identifier: GPL-2.0-only */
/* IRQ definitions for Marvell MV78xx0 SoCs */

/*
 * MV78xx0 Low Interrupt Controller
 */
pub const IRQ_MV78XX0_ERR: u32 = 0;
pub const IRQ_MV78XX0_SPI: u32 = 1;
pub const IRQ_MV78XX0_I2C_0: u32 = 2;
pub const IRQ_MV78XX0_I2C_1: u32 = 3;
pub const IRQ_MV78XX0_IDMA_0: u32 = 4;
pub const IRQ_MV78XX0_IDMA_1: u32 = 5;
pub const IRQ_MV78XX0_IDMA_2: u32 = 6;
pub const IRQ_MV78XX0_IDMA_3: u32 = 7;
pub const IRQ_MV78XX0_TIMER_0: u32 = 8;
pub const IRQ_MV78XX0_TIMER_1: u32 = 9;
pub const IRQ_MV78XX0_TIMER_2: u32 = 10;
pub const IRQ_MV78XX0_TIMER_3: u32 = 11;
pub const IRQ_MV78XX0_UART_0: u32 = 12;
pub const IRQ_MV78XX0_UART_1: u32 = 13;
pub const IRQ_MV78XX0_UART_2: u32 = 14;
pub const IRQ_MV78XX0_UART_3: u32 = 15;
pub const IRQ_MV78XX0_USB_0: u32 = 16;
pub const IRQ_MV78XX0_USB_1: u32 = 17;
pub const IRQ_MV78XX0_USB_2: u32 = 18;
pub const IRQ_MV78XX0_CRYPTO: u32 = 19;
pub const IRQ_MV78XX0_SDIO_0: u32 = 20;
pub const IRQ_MV78XX0_SDIO_1: u32 = 21;
pub const IRQ_MV78XX0_XOR_0: u32 = 22;
pub const IRQ_MV78XX0_XOR_1: u32 = 23;
pub const IRQ_MV78XX0_I2S_0: u32 = 24;
pub const IRQ_MV78XX0_I2S_1: u32 = 25;
pub const IRQ_MV78XX0_SATA: u32 = 26;
pub const IRQ_MV78XX0_TDMI: u32 = 27;

/*
 * MV78xx0 High Interrupt Controller
 */
pub const IRQ_MV78XX0_PCIE_00: u32 = 32;
pub const IRQ_MV78XX0_PCIE_01: u32 = 33;
pub const IRQ_MV78XX0_PCIE_02: u32 = 34;
pub const IRQ_MV78XX0_PCIE_03: u32 = 35;
pub const IRQ_MV78XX0_PCIE_10: u32 = 36;
pub const IRQ_MV78XX0_PCIE_11: u32 = 37;
pub const IRQ_MV78XX0_PCIE_12: u32 = 38;
pub const IRQ_MV78XX0_PCIE_13: u32 = 39;
pub const IRQ_MV78XX0_GE00_SUM: u32 = 40;
pub const IRQ_MV78XX0_GE00_RX: u32 = 41;
pub const IRQ_MV78XX0_GE00_TX: u32 = 42;
pub const IRQ_MV78XX0_GE00_MISC: u32 = 43;
pub const IRQ_MV78XX0_GE01_SUM: u32 = 44;
pub const IRQ_MV78XX0_GE01_RX: u32 = 45;
pub const IRQ_MV78XX0_GE01_TX: u32 = 46;
pub const IRQ_MV78XX0_GE01_MISC: u32 = 47;
pub const IRQ_MV78XX0_GE10_SUM: u32 = 48;
pub const IRQ_MV78XX0_GE10_RX: u32 = 49;
pub const IRQ_MV78XX0_GE10_TX: u32 = 50;
pub const IRQ_MV78XX0_GE10_MISC: u32 = 51;
pub const IRQ_MV78XX0_GE11_SUM: u32 = 52;
pub const IRQ_MV78XX0_GE11_RX: u32 = 53;
pub const IRQ_MV78XX0_GE11_TX: u32 = 54;
pub const IRQ_MV78XX0_GE11_MISC: u32 = 55;
pub const IRQ_MV78XX0_GPIO_0_7: u32 = 56;
pub const IRQ_MV78XX0_GPIO_8_15: u32 = 57;
pub const IRQ_MV78XX0_GPIO_16_23: u32 = 58;
pub const IRQ_MV78XX0_GPIO_24_31: u32 = 59;
pub const IRQ_MV78XX0_DB_IN: u32 = 60;
pub const IRQ_MV78XX0_DB_OUT: u32 = 61;

/*
 * MV78xx0 Error Interrupt Controller
 */
pub const IRQ_MV78XX0_GE_ERR: u32 = 70;

/*
 * MV78xx0 General Purpose Pins
 */
pub const IRQ_MV78XX0_GPIO_START: u32 = 96;
pub const NR_GPIO_IRQS: u32 = 32;

pub const MV78XX0_NR_IRQS: u32 = IRQ_MV78XX0_GPIO_START + NR_GPIO_IRQS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
