/*
 * Copyright (c) 2015 Joachim Eastwood <manabian@gmail.com>
 *
 * This code is released using a dual license strategy: BSD/GPL
 * You can choose the licence that better fits your requirements.
 *
 * Released under the terms of 3-clause BSD License
 * Released under the terms of GNU General Public License Version 2.0
 *
 */

/* LPC18xx/43xx base clock ids */
pub const BASE_SAFE_CLK: i32 = 0;
pub const BASE_USB0_CLK: i32 = 1;
pub const BASE_PERIPH_CLK: i32 = 2;
pub const BASE_USB1_CLK: i32 = 3;
pub const BASE_CPU_CLK: i32 = 4;
pub const BASE_SPIFI_CLK: i32 = 5;
pub const BASE_SPI_CLK: i32 = 6;
pub const BASE_PHY_RX_CLK: i32 = 7;
pub const BASE_PHY_TX_CLK: i32 = 8;
pub const BASE_APB1_CLK: i32 = 9;
pub const BASE_APB3_CLK: i32 = 10;
pub const BASE_LCD_CLK: i32 = 11;
pub const BASE_ADCHS_CLK: i32 = 12;
pub const BASE_SDIO_CLK: i32 = 13;
pub const BASE_SSP0_CLK: i32 = 14;
pub const BASE_SSP1_CLK: i32 = 15;
pub const BASE_UART0_CLK: i32 = 16;
pub const BASE_UART1_CLK: i32 = 17;
pub const BASE_UART2_CLK: i32 = 18;
pub const BASE_UART3_CLK: i32 = 19;
pub const BASE_OUT_CLK: i32 = 20;
pub const BASE_RES1_CLK: i32 = 21;
pub const BASE_RES2_CLK: i32 = 22;
pub const BASE_RES3_CLK: i32 = 23;
pub const BASE_RES4_CLK: i32 = 24;
pub const BASE_AUDIO_CLK: i32 = 25;
pub const BASE_CGU_OUT0_CLK: i32 = 26;
pub const BASE_CGU_OUT1_CLK: i32 = 27;
pub const BASE_CLK_MAX: i32 = BASE_CGU_OUT1_CLK + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
