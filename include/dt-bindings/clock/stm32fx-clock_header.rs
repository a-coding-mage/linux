/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * stm32fx-clock.h
 *
 * Copyright (C) 2016 STMicroelectronics
 * Author: Gabriel Fernandez for STMicroelectronics.
 */

/*
 * List of clocks which are not derived from system clock (SYSCLOCK)
 *
 * The index of these clocks is the secondary index of DT bindings
 * (see Documentation/devicetree/bindings/clock/st,stm32-rcc.yaml)
 *
 * e.g:
\t<assigned-clocks = <&rcc 1 CLK_LSE>;
*/

pub const SYSTICK: u32 = 0;
pub const FCLK: u32 = 1;
pub const CLK_LSI: u32 = 2;
pub const CLK_LSE: u32 = 3;
pub const CLK_HSE_RTC: u32 = 4;
pub const CLK_RTC: u32 = 5;
pub const PLL_VCO_I2S: u32 = 6;
pub const PLL_VCO_SAI: u32 = 7;
pub const CLK_LCD: u32 = 8;
pub const CLK_I2S: u32 = 9;
pub const CLK_SAI1: u32 = 10;
pub const CLK_SAI2: u32 = 11;
pub const CLK_I2SQ_PDIV: u32 = 12;
pub const CLK_SAIQ_PDIV: u32 = 13;
pub const CLK_HSI: u32 = 14;
pub const CLK_SYSCLK: u32 = 15;
pub const CLK_F469_DSI: u32 = 16;

pub const END_PRIMARY_CLK: u32 = 17;

pub const CLK_HDMI_CEC: u32 = 16;
pub const CLK_SPDIF: u32 = 17;
pub const CLK_USART1: u32 = 18;
pub const CLK_USART2: u32 = 19;
pub const CLK_USART3: u32 = 20;
pub const CLK_UART4: u32 = 21;
pub const CLK_UART5: u32 = 22;
pub const CLK_USART6: u32 = 23;
pub const CLK_UART7: u32 = 24;
pub const CLK_UART8: u32 = 25;
pub const CLK_I2C1: u32 = 26;
pub const CLK_I2C2: u32 = 27;
pub const CLK_I2C3: u32 = 28;
pub const CLK_I2C4: u32 = 29;
pub const CLK_LPTIMER: u32 = 30;
pub const CLK_PLL_SRC: u32 = 31;
pub const CLK_DFSDM1: u32 = 32;
pub const CLK_ADFSDM1: u32 = 33;
pub const CLK_F769_DSI: u32 = 34;
pub const END_PRIMARY_CLK_F7: u32 = 35;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
