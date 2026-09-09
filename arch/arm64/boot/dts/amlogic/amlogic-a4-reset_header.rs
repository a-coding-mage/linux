/* SPDX-License-Identifier: (GPL-2.0-only OR MIT) */
/*
 * Copyright (c) 2024 Amlogic, Inc. All rights reserved.
 */

/* Translated from amlogic-a4-reset.h. */

/* RESET0 */
/*						0-3 */
pub const RESET_USB: u32 = 4;
/*						5-6*/
pub const RESET_U2PHY22: u32 = 7;
pub const RESET_USBPHY20: u32 = 8;
pub const RESET_U2PHY21: u32 = 9;
pub const RESET_USB2DRD: u32 = 10;
pub const RESET_U2H: u32 = 11;
pub const RESET_LED_CTRL: u32 = 12;
/*						13-31 */

/* RESET1 */
pub const RESET_AUDIO: u32 = 32;
pub const RESET_AUDIO_VAD: u32 = 33;
/*						34*/
pub const RESET_DDR_APB: u32 = 35;
pub const RESET_DDR: u32 = 36;
pub const RESET_VOUT_VENC: u32 = 37;
pub const RESET_VOUT: u32 = 38;
/*						39-47 */
pub const RESET_ETHERNET: u32 = 48;
/*						49-63 */

/* RESET2 */
pub const RESET_DEVICE_MMC_ARB: u32 = 64;
pub const RESET_IRCTRL: u32 = 65;
/*						66*/
pub const RESET_TS_PLL: u32 = 67;
/*						68-72*/
pub const RESET_SPICC_0: u32 = 73;
pub const RESET_SPICC_1: u32 = 74;
/*						75-79*/
pub const RESET_MSR_CLK: u32 = 80;
/*						81*/
pub const RESET_SAR_ADC: u32 = 82;
/*						83-87*/
pub const RESET_ACODEC: u32 = 88;
/*						89-90*/
pub const RESET_WATCHDOG: u32 = 91;
/*						92-95*/

/* RESET3 */
/*						96-127 */

/* RESET4 */
/*						128-131 */
pub const RESET_PWM_AB: u32 = 132;
pub const RESET_PWM_CD: u32 = 133;
pub const RESET_PWM_EF: u32 = 134;
pub const RESET_PWM_GH: u32 = 135;
/*						136-137*/
pub const RESET_UART_A: u32 = 138;
pub const RESET_UART_B: u32 = 139;
/*						140*/
pub const RESET_UART_D: u32 = 141;
pub const RESET_UART_E: u32 = 142;
/*						143-144*/
pub const RESET_I2C_M_A: u32 = 145;
pub const RESET_I2C_M_B: u32 = 146;
pub const RESET_I2C_M_C: u32 = 147;
pub const RESET_I2C_M_D: u32 = 148;
/*						149-151*/
pub const RESET_SDEMMC_A: u32 = 152;
/*						153*/
pub const RESET_SDEMMC_C: u32 = 154;
/*						155-159*/

/* RESET5 */
/*						160-175*/
pub const RESET_BRG_AO_NIC_SYS: u32 = 176;
/*						177*/
pub const RESET_BRG_AO_NIC_MAIN: u32 = 178;
pub const RESET_BRG_AO_NIC_AUDIO: u32 = 179;
/*						180-183*/
pub const RESET_BRG_AO_NIC_ALL: u32 = 184;
/*						185*/
pub const RESET_BRG_NIC_SDIO: u32 = 186;
pub const RESET_BRG_NIC_EMMC: u32 = 187;
pub const RESET_BRG_NIC_DSU: u32 = 188;
pub const RESET_BRG_NIC_CLK81: u32 = 189;
pub const RESET_BRG_NIC_MAIN: u32 = 190;
pub const RESET_BRG_NIC_ALL: u32 = 191;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
