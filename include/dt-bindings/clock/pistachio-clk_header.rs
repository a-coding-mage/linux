/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014 Google, Inc.
 */

/* PLLs */
pub const CLK_MIPS_PLL: u32 = 0;
pub const CLK_AUDIO_PLL: u32 = 1;
pub const CLK_RPU_V_PLL: u32 = 2;
pub const CLK_RPU_L_PLL: u32 = 3;
pub const CLK_SYS_PLL: u32 = 4;
pub const CLK_WIFI_PLL: u32 = 5;
pub const CLK_BT_PLL: u32 = 6;

/* Fixed-factor clocks */
pub const CLK_WIFI_DIV4: u32 = 16;
pub const CLK_WIFI_DIV8: u32 = 17;

/* Gate clocks */
pub const CLK_MIPS: u32 = 32;
pub const CLK_AUDIO_IN: u32 = 33;
pub const CLK_AUDIO: u32 = 34;
pub const CLK_I2S: u32 = 35;
pub const CLK_SPDIF: u32 = 36;
pub const CLK_AUDIO_DAC: u32 = 37;
pub const CLK_RPU_V: u32 = 38;
pub const CLK_RPU_L: u32 = 39;
pub const CLK_RPU_SLEEP: u32 = 40;
pub const CLK_WIFI_PLL_GATE: u32 = 41;
pub const CLK_RPU_CORE: u32 = 42;
pub const CLK_WIFI_ADC: u32 = 43;
pub const CLK_WIFI_DAC: u32 = 44;
pub const CLK_USB_PHY: u32 = 45;
pub const CLK_ENET_IN: u32 = 46;
pub const CLK_ENET: u32 = 47;
pub const CLK_UART0: u32 = 48;
pub const CLK_UART1: u32 = 49;
pub const CLK_PERIPH_SYS: u32 = 50;
pub const CLK_SPI0: u32 = 51;
pub const CLK_SPI1: u32 = 52;
pub const CLK_EVENT_TIMER: u32 = 53;
pub const CLK_AUX_ADC_INTERNAL: u32 = 54;
pub const CLK_AUX_ADC: u32 = 55;
pub const CLK_SD_HOST: u32 = 56;
pub const CLK_BT: u32 = 57;
pub const CLK_BT_DIV4: u32 = 58;
pub const CLK_BT_DIV8: u32 = 59;
pub const CLK_BT_1MHZ: u32 = 60;

/* Divider clocks */
pub const CLK_MIPS_INTERNAL_DIV: u32 = 64;
pub const CLK_MIPS_DIV: u32 = 65;
pub const CLK_AUDIO_DIV: u32 = 66;
pub const CLK_I2S_DIV: u32 = 67;
pub const CLK_SPDIF_DIV: u32 = 68;
pub const CLK_AUDIO_DAC_DIV: u32 = 69;
pub const CLK_RPU_V_DIV: u32 = 70;
pub const CLK_RPU_L_DIV: u32 = 71;
pub const CLK_RPU_SLEEP_DIV: u32 = 72;
pub const CLK_RPU_CORE_DIV: u32 = 73;
pub const CLK_USB_PHY_DIV: u32 = 74;
pub const CLK_ENET_DIV: u32 = 75;
pub const CLK_UART0_INTERNAL_DIV: u32 = 76;
pub const CLK_UART0_DIV: u32 = 77;
pub const CLK_UART1_INTERNAL_DIV: u32 = 78;
pub const CLK_UART1_DIV: u32 = 79;
pub const CLK_SYS_INTERNAL_DIV: u32 = 80;
pub const CLK_SPI0_INTERNAL_DIV: u32 = 81;
pub const CLK_SPI0_DIV: u32 = 82;
pub const CLK_SPI1_INTERNAL_DIV: u32 = 83;
pub const CLK_SPI1_DIV: u32 = 84;
pub const CLK_EVENT_TIMER_INTERNAL_DIV: u32 = 85;
pub const CLK_EVENT_TIMER_DIV: u32 = 86;
pub const CLK_AUX_ADC_INTERNAL_DIV: u32 = 87;
pub const CLK_AUX_ADC_DIV: u32 = 88;
pub const CLK_SD_HOST_DIV: u32 = 89;
pub const CLK_BT_DIV: u32 = 90;
pub const CLK_BT_DIV4_DIV: u32 = 91;
pub const CLK_BT_DIV8_DIV: u32 = 92;
pub const CLK_BT_1MHZ_INTERNAL_DIV: u32 = 93;
pub const CLK_BT_1MHZ_DIV: u32 = 94;

/* Mux clocks */
pub const CLK_AUDIO_REF_MUX: u32 = 96;
pub const CLK_MIPS_PLL_MUX: u32 = 97;
pub const CLK_AUDIO_PLL_MUX: u32 = 98;
pub const CLK_AUDIO_MUX: u32 = 99;
pub const CLK_RPU_V_PLL_MUX: u32 = 100;
pub const CLK_RPU_L_PLL_MUX: u32 = 101;
pub const CLK_RPU_L_MUX: u32 = 102;
pub const CLK_WIFI_PLL_MUX: u32 = 103;
pub const CLK_WIFI_DIV4_MUX: u32 = 104;
pub const CLK_WIFI_DIV8_MUX: u32 = 105;
pub const CLK_RPU_CORE_MUX: u32 = 106;
pub const CLK_SYS_PLL_MUX: u32 = 107;
pub const CLK_ENET_MUX: u32 = 108;
pub const CLK_EVENT_TIMER_MUX: u32 = 109;
pub const CLK_SD_HOST_MUX: u32 = 110;
pub const CLK_BT_PLL_MUX: u32 = 111;
pub const CLK_DEBUG_MUX: u32 = 112;

pub const CLK_NR_CLKS: u32 = 113;

/* Peripheral gate clocks */
pub const PERIPH_CLK_SYS: u32 = 0;
pub const PERIPH_CLK_SYS_BUS: u32 = 1;
pub const PERIPH_CLK_DDR: u32 = 2;
pub const PERIPH_CLK_ROM: u32 = 3;
pub const PERIPH_CLK_COUNTER_FAST: u32 = 4;
pub const PERIPH_CLK_COUNTER_SLOW: u32 = 5;
pub const PERIPH_CLK_IR: u32 = 6;
pub const PERIPH_CLK_WD: u32 = 7;
pub const PERIPH_CLK_PDM: u32 = 8;
pub const PERIPH_CLK_PWM: u32 = 9;
pub const PERIPH_CLK_I2C0: u32 = 10;
pub const PERIPH_CLK_I2C1: u32 = 11;
pub const PERIPH_CLK_I2C2: u32 = 12;
pub const PERIPH_CLK_I2C3: u32 = 13;

/* Peripheral divider clocks */
pub const PERIPH_CLK_ROM_DIV: u32 = 32;
pub const PERIPH_CLK_COUNTER_FAST_DIV: u32 = 33;
pub const PERIPH_CLK_COUNTER_SLOW_PRE_DIV: u32 = 34;
pub const PERIPH_CLK_COUNTER_SLOW_DIV: u32 = 35;
pub const PERIPH_CLK_IR_PRE_DIV: u32 = 36;
pub const PERIPH_CLK_IR_DIV: u32 = 37;
pub const PERIPH_CLK_WD_PRE_DIV: u32 = 38;
pub const PERIPH_CLK_WD_DIV: u32 = 39;
pub const PERIPH_CLK_PDM_PRE_DIV: u32 = 40;
pub const PERIPH_CLK_PDM_DIV: u32 = 41;
pub const PERIPH_CLK_PWM_PRE_DIV: u32 = 42;
pub const PERIPH_CLK_PWM_DIV: u32 = 43;
pub const PERIPH_CLK_I2C0_PRE_DIV: u32 = 44;
pub const PERIPH_CLK_I2C0_DIV: u32 = 45;
pub const PERIPH_CLK_I2C1_PRE_DIV: u32 = 46;
pub const PERIPH_CLK_I2C1_DIV: u32 = 47;
pub const PERIPH_CLK_I2C2_PRE_DIV: u32 = 48;
pub const PERIPH_CLK_I2C2_DIV: u32 = 49;
pub const PERIPH_CLK_I2C3_PRE_DIV: u32 = 50;
pub const PERIPH_CLK_I2C3_DIV: u32 = 51;

pub const PERIPH_CLK_NR_CLKS: u32 = 52;

/* System gate clocks */
pub const SYS_CLK_I2C0: u32 = 0;
pub const SYS_CLK_I2C1: u32 = 1;
pub const SYS_CLK_I2C2: u32 = 2;
pub const SYS_CLK_I2C3: u32 = 3;
pub const SYS_CLK_I2S_IN: u32 = 4;
pub const SYS_CLK_PAUD_OUT: u32 = 5;
pub const SYS_CLK_SPDIF_OUT: u32 = 6;
pub const SYS_CLK_SPI0_MASTER: u32 = 7;
pub const SYS_CLK_SPI0_SLAVE: u32 = 8;
pub const SYS_CLK_PWM: u32 = 9;
pub const SYS_CLK_UART0: u32 = 10;
pub const SYS_CLK_UART1: u32 = 11;
pub const SYS_CLK_SPI1: u32 = 12;
pub const SYS_CLK_MDC: u32 = 13;
pub const SYS_CLK_SD_HOST: u32 = 14;
pub const SYS_CLK_ENET: u32 = 15;
pub const SYS_CLK_IR: u32 = 16;
pub const SYS_CLK_WD: u32 = 17;
pub const SYS_CLK_TIMER: u32 = 18;
pub const SYS_CLK_I2S_OUT: u32 = 24;
pub const SYS_CLK_SPDIF_IN: u32 = 25;
pub const SYS_CLK_EVENT_TIMER: u32 = 26;
pub const SYS_CLK_HASH: u32 = 27;

pub const SYS_CLK_NR_CLKS: u32 = 28;

/* Gates for external input clocks */
pub const EXT_CLK_AUDIO_IN: u32 = 0;
pub const EXT_CLK_ENET_IN: u32 = 1;

pub const EXT_CLK_NR_CLKS: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
