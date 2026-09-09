/* SPDX-License-Identifier: (GPL-2.0+ OR MIT) */
/*
 * Copyright (c) 2021 Amlogic, Inc. All rights reserved.
 * Author: Zelong Dong <zelong.dong@amlogic.com>
 */

/* RESET0 */
pub const RESET_USB_DDR0: u32 = 0;
pub const RESET_USB_DDR1: u32 = 1;
pub const RESET_USB_DDR2: u32 = 2;
pub const RESET_USB_DDR3: u32 = 3;
pub const RESET_USBCTRL: u32 = 4;
pub const RESET_USBPHY20: u32 = 8;
pub const RESET_USBPHY21: u32 = 9;
pub const RESET_HDMITX_APB: u32 = 16;
pub const RESET_BRG_VCBUS_DEC: u32 = 17;
pub const RESET_VCBUS: u32 = 18;
pub const RESET_VID_PLL_DIV: u32 = 19;
pub const RESET_VDI6: u32 = 20;
pub const RESET_GE2D: u32 = 21;
pub const RESET_HDMITXPHY: u32 = 22;
pub const RESET_VID_LOCK: u32 = 23;
pub const RESET_VENCL: u32 = 24;
pub const RESET_VDAC: u32 = 25;
pub const RESET_VENCP: u32 = 26;
pub const RESET_VENCI: u32 = 27;
pub const RESET_RDMA: u32 = 28;
pub const RESET_HDMI_TX: u32 = 29;
pub const RESET_VIU: u32 = 30;
pub const RESET_VENC: u32 = 31;

/* RESET1 */
pub const RESET_AUDIO: u32 = 32;
pub const RESET_MALI_APB: u32 = 33;
pub const RESET_MALI: u32 = 34;
pub const RESET_DDR_APB: u32 = 35;
pub const RESET_DDR: u32 = 36;
pub const RESET_DOS_APB: u32 = 37;
pub const RESET_DOS: u32 = 38;
pub const RESET_ETH: u32 = 48;
pub const RESET_DEMOD: u32 = 52;

/* RESET2 */
pub const RESET_ABUS_ARB: u32 = 64;
pub const RESET_IR_CTRL: u32 = 65;
pub const RESET_TEMPSENSOR_DDR: u32 = 66;
pub const RESET_TEMPSENSOR_PLL: u32 = 67;
pub const RESET_SMART_CARD: u32 = 72;
pub const RESET_SPICC0: u32 = 73;
pub const RESET_RSA: u32 = 75;
pub const RESET_MSR_CLK: u32 = 80;
pub const RESET_SPIFC: u32 = 81;
pub const RESET_SARADC: u32 = 82;
pub const RESET_ACODEC: u32 = 88;
pub const RESET_CEC: u32 = 89;
pub const RESET_AFIFO: u32 = 90;
pub const RESET_WATCHDOG: u32 = 91;

/* RESET3 and RESET4 */
pub const RESET_PWM_AB: u32 = 132;
pub const RESET_PWM_CD: u32 = 133;
pub const RESET_PWM_EF: u32 = 134;
pub const RESET_PWM_GH: u32 = 135;
pub const RESET_PWM_IJ: u32 = 136;
pub const RESET_UART_A: u32 = 138;
pub const RESET_UART_B: u32 = 139;
pub const RESET_UART_C: u32 = 140;
pub const RESET_UART_D: u32 = 141;
pub const RESET_UART_E: u32 = 142;
pub const RESET_I2C_S_A: u32 = 144;
pub const RESET_I2C_M_A: u32 = 145;
pub const RESET_I2C_M_B: u32 = 146;
pub const RESET_I2C_M_C: u32 = 147;
pub const RESET_I2C_M_D: u32 = 148;
pub const RESET_I2C_M_E: u32 = 149;
pub const RESET_SD_EMMC_A: u32 = 152;
pub const RESET_SD_EMMC_B: u32 = 153;
pub const RESET_NAND_EMMC: u32 = 154;

/* RESET5 */
pub const RESET_BRG_VDEC_PIPL0: u32 = 160;
pub const RESET_BRG_HEVCF_PIPL0: u32 = 161;
pub const RESET_BRG_HCODEC_PIPL0: u32 = 163;
pub const RESET_BRG_GE2D_PIPL0: u32 = 164;
pub const RESET_BRG_VPU_PIPL0: u32 = 165;
pub const RESET_BRG_CPU_PIPL0: u32 = 166;
pub const RESET_BRG_MALI_PIPL0: u32 = 167;
pub const RESET_BRG_MALI_PIPL1: u32 = 169;
pub const RESET_BRG_HEVCF_PIPL1: u32 = 172;
pub const RESET_BRG_HEVCB_PIPL1: u32 = 173;
pub const RESET_RAMA: u32 = 184;
pub const RESET_BRG_NIC_VAPB: u32 = 187;
pub const RESET_BRG_NIC_DSU: u32 = 188;
pub const RESET_BRG_NIC_SYSCLK: u32 = 189;
pub const RESET_BRG_NIC_MAIN: u32 = 190;
pub const RESET_BRG_NIC_ALL: u32 = 191;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
