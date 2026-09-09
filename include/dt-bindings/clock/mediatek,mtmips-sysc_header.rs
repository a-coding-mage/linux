/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Author: Sergio Paracuellos <sergio.paracuellos@gmail.com>
 */

/* Ralink RT-2880 clocks */
pub const RT2880_CLK_XTAL: u32 = 0;
pub const RT2880_CLK_CPU: u32 = 1;
pub const RT2880_CLK_BUS: u32 = 2;
pub const RT2880_CLK_TIMER: u32 = 3;
pub const RT2880_CLK_WATCHDOG: u32 = 4;
pub const RT2880_CLK_UART: u32 = 5;
pub const RT2880_CLK_I2C: u32 = 6;
pub const RT2880_CLK_UARTLITE: u32 = 7;
pub const RT2880_CLK_ETHERNET: u32 = 8;
pub const RT2880_CLK_WMAC: u32 = 9;

/* Ralink RT-305X clocks */
pub const RT305X_CLK_XTAL: u32 = 0;
pub const RT305X_CLK_CPU: u32 = 1;
pub const RT305X_CLK_BUS: u32 = 2;
pub const RT305X_CLK_TIMER: u32 = 3;
pub const RT305X_CLK_WATCHDOG: u32 = 4;
pub const RT305X_CLK_UART: u32 = 5;
pub const RT305X_CLK_I2C: u32 = 6;
pub const RT305X_CLK_I2S: u32 = 7;
pub const RT305X_CLK_SPI1: u32 = 8;
pub const RT305X_CLK_SPI2: u32 = 9;
pub const RT305X_CLK_UARTLITE: u32 = 10;
pub const RT305X_CLK_ETHERNET: u32 = 11;
pub const RT305X_CLK_WMAC: u32 = 12;

/* Ralink RT-3352 clocks */
pub const RT3352_CLK_XTAL: u32 = 0;
pub const RT3352_CLK_CPU: u32 = 1;
pub const RT3352_CLK_PERIPH: u32 = 2;
pub const RT3352_CLK_BUS: u32 = 3;
pub const RT3352_CLK_TIMER: u32 = 4;
pub const RT3352_CLK_WATCHDOG: u32 = 5;
pub const RT3352_CLK_UART: u32 = 6;
pub const RT3352_CLK_I2C: u32 = 7;
pub const RT3352_CLK_I2S: u32 = 8;
pub const RT3352_CLK_SPI1: u32 = 9;
pub const RT3352_CLK_SPI2: u32 = 10;
pub const RT3352_CLK_UARTLITE: u32 = 11;
pub const RT3352_CLK_ETHERNET: u32 = 12;
pub const RT3352_CLK_WMAC: u32 = 13;

/* Ralink RT-3883 clocks */
pub const RT3883_CLK_XTAL: u32 = 0;
pub const RT3883_CLK_CPU: u32 = 1;
pub const RT3883_CLK_BUS: u32 = 2;
pub const RT3883_CLK_PERIPH: u32 = 3;
pub const RT3883_CLK_TIMER: u32 = 4;
pub const RT3883_CLK_WATCHDOG: u32 = 5;
pub const RT3883_CLK_UART: u32 = 6;
pub const RT3883_CLK_I2C: u32 = 7;
pub const RT3883_CLK_I2S: u32 = 8;
pub const RT3883_CLK_SPI1: u32 = 9;
pub const RT3883_CLK_SPI2: u32 = 10;
pub const RT3883_CLK_UARTLITE: u32 = 11;
pub const RT3883_CLK_ETHERNET: u32 = 12;
pub const RT3883_CLK_WMAC: u32 = 13;

/* Ralink RT-5350 clocks */
pub const RT5350_CLK_XTAL: u32 = 0;
pub const RT5350_CLK_CPU: u32 = 1;
pub const RT5350_CLK_BUS: u32 = 2;
pub const RT5350_CLK_PERIPH: u32 = 3;
pub const RT5350_CLK_TIMER: u32 = 4;
pub const RT5350_CLK_WATCHDOG: u32 = 5;
pub const RT5350_CLK_UART: u32 = 6;
pub const RT5350_CLK_I2C: u32 = 7;
pub const RT5350_CLK_I2S: u32 = 8;
pub const RT5350_CLK_SPI1: u32 = 9;
pub const RT5350_CLK_SPI2: u32 = 10;
pub const RT5350_CLK_UARTLITE: u32 = 11;
pub const RT5350_CLK_ETHERNET: u32 = 12;
pub const RT5350_CLK_WMAC: u32 = 13;

/* Ralink MT-7620 clocks */
pub const MT7620_CLK_XTAL: u32 = 0;
pub const MT7620_CLK_PLL: u32 = 1;
pub const MT7620_CLK_CPU: u32 = 2;
pub const MT7620_CLK_PERIPH: u32 = 3;
pub const MT7620_CLK_BUS: u32 = 4;
pub const MT7620_CLK_BBPPLL: u32 = 5;
pub const MT7620_CLK_SDHC: u32 = 6;
pub const MT7620_CLK_TIMER: u32 = 7;
pub const MT7620_CLK_WATCHDOG: u32 = 8;
pub const MT7620_CLK_UART: u32 = 9;
pub const MT7620_CLK_I2C: u32 = 10;
pub const MT7620_CLK_I2S: u32 = 11;
pub const MT7620_CLK_SPI1: u32 = 12;
pub const MT7620_CLK_SPI2: u32 = 13;
pub const MT7620_CLK_UARTLITE: u32 = 14;
pub const MT7620_CLK_MMC: u32 = 15;
pub const MT7620_CLK_WMAC: u32 = 16;

/* Ralink MT-76X8 clocks */
pub const MT76X8_CLK_XTAL: u32 = 0;
pub const MT76X8_CLK_CPU: u32 = 1;
pub const MT76X8_CLK_BBPPLL: u32 = 2;
pub const MT76X8_CLK_PCMI2S: u32 = 3;
pub const MT76X8_CLK_PERIPH: u32 = 4;
pub const MT76X8_CLK_BUS: u32 = 5;
pub const MT76X8_CLK_SDHC: u32 = 6;
pub const MT76X8_CLK_TIMER: u32 = 7;
pub const MT76X8_CLK_WATCHDOG: u32 = 8;
pub const MT76X8_CLK_I2C: u32 = 9;
pub const MT76X8_CLK_I2S: u32 = 10;
pub const MT76X8_CLK_SPI1: u32 = 11;
pub const MT76X8_CLK_SPI2: u32 = 12;
pub const MT76X8_CLK_UART0: u32 = 13;
pub const MT76X8_CLK_UART1: u32 = 14;
pub const MT76X8_CLK_UART2: u32 = 15;
pub const MT76X8_CLK_MMC: u32 = 16;
pub const MT76X8_CLK_WMAC: u32 = 17;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
