/* SPDX-License-Identifier: (GPL-2.0-only OR MIT) */
/*
 * Copyright (c) 2024 Amlogic, Inc. All rights reserved.
 */

/* RESET0 */
/*                                           0-3 */
pub const RESET_USB: u32 = 4;
/*                                           5-7 */
pub const RESET_USBPHY20: u32 = 8;
/*                                           9 */
pub const RESET_USB2DRD: u32 = 10;
/*                                           11-31 */

/* RESET1 */
pub const RESET_AUDIO: u32 = 32;
pub const RESET_AUDIO_VAD: u32 = 33;
/*                                           34 */
pub const RESET_DDR_APB: u32 = 35;
pub const RESET_DDR: u32 = 36;
/*                                           37-40 */
pub const RESET_DSPA_DEBUG: u32 = 41;
/*                                           42 */
pub const RESET_DSPA: u32 = 43;
/*                                           44-46 */
pub const RESET_NNA: u32 = 47;
pub const RESET_ETHERNET: u32 = 48;
/*                                           49-63 */

/* RESET2 */
pub const RESET_ABUS_ARB: u32 = 64;
pub const RESET_IRCTRL: u32 = 65;
/*                                           66 */
pub const RESET_TS_PLL: u32 = 67;
/*                                           68-72 */
pub const RESET_SPICC_0: u32 = 73;
pub const RESET_SPICC_1: u32 = 74;
pub const RESET_RSA: u32 = 75;

/*                                           76-79 */
pub const RESET_MSR_CLK: u32 = 80;
pub const RESET_SPIFC: u32 = 81;
pub const RESET_SAR_ADC: u32 = 82;
/*                                           83-90 */
pub const RESET_WATCHDOG: u32 = 91;
/*                                           92-95 */

/* RESET3 */
/*                                           96-127 */

/* RESET4 */
pub const RESET_RTC: u32 = 128;
/*                                           129-131 */
pub const RESET_PWM_AB: u32 = 132;
pub const RESET_PWM_CD: u32 = 133;
pub const RESET_PWM_EF: u32 = 134;
pub const RESET_PWM_GH: u32 = 135;
/*                                           104-105 */
pub const RESET_UART_A: u32 = 138;
pub const RESET_UART_B: u32 = 139;
pub const RESET_UART_C: u32 = 140;
pub const RESET_UART_D: u32 = 141;
pub const RESET_UART_E: u32 = 142;
/*                                           143*/
pub const RESET_I2C_S_A: u32 = 144;
pub const RESET_I2C_M_A: u32 = 145;
pub const RESET_I2C_M_B: u32 = 146;
pub const RESET_I2C_M_C: u32 = 147;
pub const RESET_I2C_M_D: u32 = 148;
/*                                           149-151 */
pub const RESET_SDEMMC_A: u32 = 152;
/*                                           153 */
pub const RESET_SDEMMC_C: u32 = 154;
/*                                           155-159*/

/* RESET5 */
/*                                           160-175 */
pub const RESET_BRG_AO_NIC_SYS: u32 = 176;
pub const RESET_BRG_AO_NIC_DSPA: u32 = 177;
pub const RESET_BRG_AO_NIC_MAIN: u32 = 178;
pub const RESET_BRG_AO_NIC_AUDIO: u32 = 179;
/*                                           180-183 */
pub const RESET_BRG_AO_NIC_ALL: u32 = 184;
pub const RESET_BRG_NIC_NNA: u32 = 185;
pub const RESET_BRG_NIC_SDIO: u32 = 186;
pub const RESET_BRG_NIC_EMMC: u32 = 187;
pub const RESET_BRG_NIC_DSU: u32 = 188;
pub const RESET_BRG_NIC_SYSCLK: u32 = 189;
pub const RESET_BRG_NIC_MAIN: u32 = 190;
pub const RESET_BRG_NIC_ALL: u32 = 191;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
