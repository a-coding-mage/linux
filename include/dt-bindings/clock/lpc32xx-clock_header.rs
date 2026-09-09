/*
 * Copyright (c) 2015 Vladimir Zapolskiy <vz@mleia.com>
 *
 * This code is released using a dual license strategy: BSD/GPL
 * You can choose the licence that better fits your requirements.
 *
 * Released under the terms of 3-clause BSD License
 * Released under the terms of GNU General Public License Version 2.0
 *
 */

/* LPC32XX System Control Block clocks */
pub const LPC32XX_CLK_RTC: u32 = 1;
pub const LPC32XX_CLK_DMA: u32 = 2;
pub const LPC32XX_CLK_MLC: u32 = 3;
pub const LPC32XX_CLK_SLC: u32 = 4;
pub const LPC32XX_CLK_LCD: u32 = 5;
pub const LPC32XX_CLK_MAC: u32 = 6;
pub const LPC32XX_CLK_SD: u32 = 7;
pub const LPC32XX_CLK_DDRAM: u32 = 8;
pub const LPC32XX_CLK_SSP0: u32 = 9;
pub const LPC32XX_CLK_SSP1: u32 = 10;
pub const LPC32XX_CLK_UART3: u32 = 11;
pub const LPC32XX_CLK_UART4: u32 = 12;
pub const LPC32XX_CLK_UART5: u32 = 13;
pub const LPC32XX_CLK_UART6: u32 = 14;
pub const LPC32XX_CLK_IRDA: u32 = 15;
pub const LPC32XX_CLK_I2C1: u32 = 16;
pub const LPC32XX_CLK_I2C2: u32 = 17;
pub const LPC32XX_CLK_TIMER0: u32 = 18;
pub const LPC32XX_CLK_TIMER1: u32 = 19;
pub const LPC32XX_CLK_TIMER2: u32 = 20;
pub const LPC32XX_CLK_TIMER3: u32 = 21;
pub const LPC32XX_CLK_TIMER4: u32 = 22;
pub const LPC32XX_CLK_TIMER5: u32 = 23;
pub const LPC32XX_CLK_WDOG: u32 = 24;
pub const LPC32XX_CLK_I2S0: u32 = 25;
pub const LPC32XX_CLK_I2S1: u32 = 26;
pub const LPC32XX_CLK_SPI1: u32 = 27;
pub const LPC32XX_CLK_SPI2: u32 = 28;
pub const LPC32XX_CLK_MCPWM: u32 = 29;
pub const LPC32XX_CLK_HSTIMER: u32 = 30;
pub const LPC32XX_CLK_KEY: u32 = 31;
pub const LPC32XX_CLK_PWM1: u32 = 32;
pub const LPC32XX_CLK_PWM2: u32 = 33;
pub const LPC32XX_CLK_ADC: u32 = 34;
pub const LPC32XX_CLK_HCLK_PLL: u32 = 35;
pub const LPC32XX_CLK_PERIPH: u32 = 36;

/* LPC32XX USB clocks */
pub const LPC32XX_USB_CLK_I2C: u32 = 1;
pub const LPC32XX_USB_CLK_DEVICE: u32 = 2;
pub const LPC32XX_USB_CLK_HOST: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
