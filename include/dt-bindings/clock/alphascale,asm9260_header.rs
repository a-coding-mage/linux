/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2014 Oleksij Rempel <linux@rempel-privat.de>
 */

/* ahb gate */
pub const CLKID_AHB_ROM: u32 = 0;
pub const CLKID_AHB_RAM: u32 = 1;
pub const CLKID_AHB_GPIO: u32 = 2;
pub const CLKID_AHB_MAC: u32 = 3;
pub const CLKID_AHB_EMI: u32 = 4;
pub const CLKID_AHB_USB0: u32 = 5;
pub const CLKID_AHB_USB1: u32 = 6;
pub const CLKID_AHB_DMA0: u32 = 7;
pub const CLKID_AHB_DMA1: u32 = 8;
pub const CLKID_AHB_UART0: u32 = 9;
pub const CLKID_AHB_UART1: u32 = 10;
pub const CLKID_AHB_UART2: u32 = 11;
pub const CLKID_AHB_UART3: u32 = 12;
pub const CLKID_AHB_UART4: u32 = 13;
pub const CLKID_AHB_UART5: u32 = 14;
pub const CLKID_AHB_UART6: u32 = 15;
pub const CLKID_AHB_UART7: u32 = 16;
pub const CLKID_AHB_UART8: u32 = 17;
pub const CLKID_AHB_UART9: u32 = 18;
pub const CLKID_AHB_I2S0: u32 = 19;
pub const CLKID_AHB_I2C0: u32 = 20;
pub const CLKID_AHB_I2C1: u32 = 21;
pub const CLKID_AHB_SSP0: u32 = 22;
pub const CLKID_AHB_IOCONFIG: u32 = 23;
pub const CLKID_AHB_WDT: u32 = 24;
pub const CLKID_AHB_CAN0: u32 = 25;
pub const CLKID_AHB_CAN1: u32 = 26;
pub const CLKID_AHB_MPWM: u32 = 27;
pub const CLKID_AHB_SPI0: u32 = 28;
pub const CLKID_AHB_SPI1: u32 = 29;
pub const CLKID_AHB_QEI: u32 = 30;
pub const CLKID_AHB_QUADSPI0: u32 = 31;
pub const CLKID_AHB_CAMIF: u32 = 32;
pub const CLKID_AHB_LCDIF: u32 = 33;
pub const CLKID_AHB_TIMER0: u32 = 34;
pub const CLKID_AHB_TIMER1: u32 = 35;
pub const CLKID_AHB_TIMER2: u32 = 36;
pub const CLKID_AHB_TIMER3: u32 = 37;
pub const CLKID_AHB_IRQ: u32 = 38;
pub const CLKID_AHB_RTC: u32 = 39;
pub const CLKID_AHB_NAND: u32 = 40;
pub const CLKID_AHB_ADC0: u32 = 41;
pub const CLKID_AHB_LED: u32 = 42;
pub const CLKID_AHB_DAC0: u32 = 43;
pub const CLKID_AHB_LCD: u32 = 44;
pub const CLKID_AHB_I2S1: u32 = 45;
pub const CLKID_AHB_MAC1: u32 = 46;

/* divider */
pub const CLKID_SYS_CPU: u32 = 47;
pub const CLKID_SYS_AHB: u32 = 48;
pub const CLKID_SYS_I2S0M: u32 = 49;
pub const CLKID_SYS_I2S0S: u32 = 50;
pub const CLKID_SYS_I2S1M: u32 = 51;
pub const CLKID_SYS_I2S1S: u32 = 52;
pub const CLKID_SYS_UART0: u32 = 53;
pub const CLKID_SYS_UART1: u32 = 54;
pub const CLKID_SYS_UART2: u32 = 55;
pub const CLKID_SYS_UART3: u32 = 56;
pub const CLKID_SYS_UART4: u32 = 56;
pub const CLKID_SYS_UART5: u32 = 57;
pub const CLKID_SYS_UART6: u32 = 58;
pub const CLKID_SYS_UART7: u32 = 59;
pub const CLKID_SYS_UART8: u32 = 60;
pub const CLKID_SYS_UART9: u32 = 61;
pub const CLKID_SYS_SPI0: u32 = 62;
pub const CLKID_SYS_SPI1: u32 = 63;
pub const CLKID_SYS_QUADSPI: u32 = 64;
pub const CLKID_SYS_SSP0: u32 = 65;
pub const CLKID_SYS_NAND: u32 = 66;
pub const CLKID_SYS_TRACE: u32 = 67;
pub const CLKID_SYS_CAMM: u32 = 68;
pub const CLKID_SYS_WDT: u32 = 69;
pub const CLKID_SYS_CLKOUT: u32 = 70;
pub const CLKID_SYS_MAC: u32 = 71;
pub const CLKID_SYS_LCD: u32 = 72;
pub const CLKID_SYS_ADCANA: u32 = 73;

pub const MAX_CLKS: u32 = 74;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
