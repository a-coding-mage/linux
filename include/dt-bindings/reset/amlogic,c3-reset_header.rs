/* SPDX-License-Identifier: (GPL-2.0-only OR MIT) */
/*
 * Copyright (c) 2023 Amlogic, Inc. All rights reserved.
 */

/* RESET0 */
/*						0-3 */
pub const RESET_USBCTRL: u32 = 4;
/*						5-7 */
pub const RESET_USBPHY20: u32 = 8;
/*						9 */
pub const RESET_USB2DRD: u32 = 10;
pub const RESET_MIPI_DSI_HOST: u32 = 11;
pub const RESET_MIPI_DSI_PHY: u32 = 12;
/*						13-20 */
pub const RESET_GE2D: u32 = 21;
pub const RESET_DWAP: u32 = 22;
/*						23-31 */

/* RESET1 */
pub const RESET_AUDIO: u32 = 32;
/*						33-34 */
pub const RESET_DDRAPB: u32 = 35;
pub const RESET_DDR: u32 = 36;
pub const RESET_DOS_CAPB3: u32 = 37;
pub const RESET_DOS: u32 = 38;
/*						39-46 */
pub const RESET_NNA: u32 = 47;
pub const RESET_ETHERNET: u32 = 48;
pub const RESET_ISP: u32 = 49;
pub const RESET_VC9000E_APB: u32 = 50;
pub const RESET_VC9000E_A: u32 = 51;
/*						52 */
pub const RESET_VC9000E_CORE: u32 = 53;
/*						54-63 */

/* RESET2 */
pub const RESET_ABUS_ARB: u32 = 64;
pub const RESET_IRCTRL: u32 = 65;
/*						66 */
pub const RESET_TEMP_PII: u32 = 67;
/*						68-72 */
pub const RESET_SPICC_0: u32 = 73;
pub const RESET_SPICC_1: u32 = 74;
pub const RESET_RSA: u32 = 75;

/*						76-79 */
pub const RESET_MSR_CLK: u32 = 80;
pub const RESET_SPIFC: u32 = 81;
pub const RESET_SAR_ADC: u32 = 82;
/*						83-87 */
pub const RESET_ACODEC: u32 = 88;
/*						89-90 */
pub const RESET_WATCHDOG: u32 = 91;
/*						92-95 */

/* RESET3 */
pub const RESET_ISP_NIC_GPV: u32 = 96;
pub const RESET_ISP_NIC_MAIN: u32 = 97;
pub const RESET_ISP_NIC_VCLK: u32 = 98;
pub const RESET_ISP_NIC_VOUT: u32 = 99;
pub const RESET_ISP_NIC_ALL: u32 = 100;
pub const RESET_VOUT: u32 = 101;
pub const RESET_VOUT_VENC: u32 = 102;
/*						103 */
pub const RESET_CVE_NIC_GPV: u32 = 104;
pub const RESET_CVE_NIC_MAIN: u32 = 105;
pub const RESET_CVE_NIC_GE2D: u32 = 106;
pub const RESET_CVE_NIC_DW: u32 = 106;
pub const RESET_CVE_NIC_CVE: u32 = 108;
pub const RESET_CVE_NIC_ALL: u32 = 109;
pub const RESET_CVE: u32 = 110;
/*						112-127 */

/* RESET4 */
pub const RESET_RTC: u32 = 128;
pub const RESET_PWM_AB: u32 = 129;
pub const RESET_PWM_CD: u32 = 130;
pub const RESET_PWM_EF: u32 = 131;
pub const RESET_PWM_GH: u32 = 132;
pub const RESET_PWM_IJ: u32 = 133;
pub const RESET_PWM_KL: u32 = 134;
pub const RESET_PWM_MN: u32 = 135;
/*						136-137 */
pub const RESET_UART_A: u32 = 138;
pub const RESET_UART_B: u32 = 139;
pub const RESET_UART_C: u32 = 140;
pub const RESET_UART_D: u32 = 141;
pub const RESET_UART_E: u32 = 142;
pub const RESET_UART_F: u32 = 143;
pub const RESET_I2C_S_A: u32 = 144;
pub const RESET_I2C_M_A: u32 = 145;
pub const RESET_I2C_M_B: u32 = 146;
pub const RESET_I2C_M_C: u32 = 147;
pub const RESET_I2C_M_D: u32 = 148;
/*						149-151 */
pub const RESET_SD_EMMC_A: u32 = 152;
pub const RESET_SD_EMMC_B: u32 = 153;
pub const RESET_SD_EMMC_C: u32 = 154;

/* RESET5 */
/*						160-172 */
pub const RESET_BRG_NIC_NNA: u32 = 173;
pub const RESET_BRG_MUX_NIC_MAIN: u32 = 174;
pub const RESET_BRG_AO_NIC_ALL: u32 = 175;
/*						176-183 */
pub const RESET_BRG_NIC_VAPB: u32 = 184;
pub const RESET_BRG_NIC_SDIO_B: u32 = 185;
pub const RESET_BRG_NIC_SDIO_A: u32 = 186;
pub const RESET_BRG_NIC_EMMC: u32 = 187;
pub const RESET_BRG_NIC_DSU: u32 = 188;
pub const RESET_BRG_NIC_SYSCLK: u32 = 189;
pub const RESET_BRG_NIC_MAIN: u32 = 190;
pub const RESET_BRG_NIC_ALL: u32 = 191;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
