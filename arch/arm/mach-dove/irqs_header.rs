/* SPDX-License-Identifier: GPL-2.0-only */
/* IRQ definitions for Marvell Dove 88AP510 SoC */

/*
 * Dove Low Interrupt Controller
 */
pub const IRQ_DOVE_BRIDGE: i32 = 1 + 0;
pub const IRQ_DOVE_H2C: i32 = 1 + 1;
pub const IRQ_DOVE_C2H: i32 = 1 + 2;
pub const IRQ_DOVE_NAND: i32 = 1 + 3;
pub const IRQ_DOVE_PDMA: i32 = 1 + 4;
pub const IRQ_DOVE_SPI1: i32 = 1 + 5;
pub const IRQ_DOVE_SPI0: i32 = 1 + 6;
pub const IRQ_DOVE_UART_0: i32 = 1 + 7;
pub const IRQ_DOVE_UART_1: i32 = 1 + 8;
pub const IRQ_DOVE_UART_2: i32 = 1 + 9;
pub const IRQ_DOVE_UART_3: i32 = 1 + 10;
pub const IRQ_DOVE_I2C: i32 = 1 + 11;
pub const IRQ_DOVE_GPIO_0_7: i32 = 1 + 12;
pub const IRQ_DOVE_GPIO_8_15: i32 = 1 + 13;
pub const IRQ_DOVE_GPIO_16_23: i32 = 1 + 14;
pub const IRQ_DOVE_PCIE0_ERR: i32 = 1 + 15;
pub const IRQ_DOVE_PCIE0: i32 = 1 + 16;
pub const IRQ_DOVE_PCIE1_ERR: i32 = 1 + 17;
pub const IRQ_DOVE_PCIE1: i32 = 1 + 18;
pub const IRQ_DOVE_I2S0: i32 = 1 + 19;
pub const IRQ_DOVE_I2S0_ERR: i32 = 1 + 20;
pub const IRQ_DOVE_I2S1: i32 = 1 + 21;
pub const IRQ_DOVE_I2S1_ERR: i32 = 1 + 22;
pub const IRQ_DOVE_USB_ERR: i32 = 1 + 23;
pub const IRQ_DOVE_USB0: i32 = 1 + 24;
pub const IRQ_DOVE_USB1: i32 = 1 + 25;
pub const IRQ_DOVE_GE00_RX: i32 = 1 + 26;
pub const IRQ_DOVE_GE00_TX: i32 = 1 + 27;
pub const IRQ_DOVE_GE00_MISC: i32 = 1 + 28;
pub const IRQ_DOVE_GE00_SUM: i32 = 1 + 29;
pub const IRQ_DOVE_GE00_ERR: i32 = 1 + 30;
pub const IRQ_DOVE_CRYPTO: i32 = 1 + 31;

/*
 * Dove High Interrupt Controller
 */
pub const IRQ_DOVE_AC97: i32 = 1 + 32;
pub const IRQ_DOVE_PMU: i32 = 1 + 33;
pub const IRQ_DOVE_CAM: i32 = 1 + 34;
pub const IRQ_DOVE_SDIO0: i32 = 1 + 35;
pub const IRQ_DOVE_SDIO1: i32 = 1 + 36;
pub const IRQ_DOVE_SDIO0_WAKEUP: i32 = 1 + 37;
pub const IRQ_DOVE_SDIO1_WAKEUP: i32 = 1 + 38;
pub const IRQ_DOVE_XOR_00: i32 = 1 + 39;
pub const IRQ_DOVE_XOR_01: i32 = 1 + 40;
pub const IRQ_DOVE_XOR0_ERR: i32 = 1 + 41;
pub const IRQ_DOVE_XOR_10: i32 = 1 + 42;
pub const IRQ_DOVE_XOR_11: i32 = 1 + 43;
pub const IRQ_DOVE_XOR1_ERR: i32 = 1 + 44;
pub const IRQ_DOVE_LCD_DCON: i32 = 1 + 45;
pub const IRQ_DOVE_LCD1: i32 = 1 + 46;
pub const IRQ_DOVE_LCD0: i32 = 1 + 47;
pub const IRQ_DOVE_GPU: i32 = 1 + 48;
pub const IRQ_DOVE_PERFORM_MNTR: i32 = 1 + 49;
pub const IRQ_DOVE_VPRO_DMA1: i32 = 1 + 51;
pub const IRQ_DOVE_SSP_TIMER: i32 = 1 + 54;
pub const IRQ_DOVE_SSP: i32 = 1 + 55;
pub const IRQ_DOVE_MC_L2_ERR: i32 = 1 + 56;
pub const IRQ_DOVE_CRYPTO_ERR: i32 = 1 + 59;
pub const IRQ_DOVE_GPIO_24_31: i32 = 1 + 60;
pub const IRQ_DOVE_HIGH_GPIO: i32 = 1 + 61;
pub const IRQ_DOVE_SATA: i32 = 1 + 62;

/*
 * DOVE General Purpose Pins
 */
pub const IRQ_DOVE_GPIO_START: i32 = 65;
pub const NR_GPIO_IRQS: i32 = 64;

/*
 * PMU interrupts
 */
pub const IRQ_DOVE_PMU_START: i32 = IRQ_DOVE_GPIO_START + NR_GPIO_IRQS;
pub const NR_PMU_IRQS: i32 = 7;
pub const IRQ_DOVE_RTC: i32 = IRQ_DOVE_PMU_START + 5;

pub const DOVE_NR_IRQS: i32 = IRQ_DOVE_PMU_START + NR_PMU_IRQS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
