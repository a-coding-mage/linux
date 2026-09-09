/* SPDX-License-Identifier: (GPL-2.0-only OR MIT) */
/*
 * Copyright (c) 2024 Amlogic, Inc. All rights reserved.
 */

/* RESET0 */
/*                                      0-3     */
pub const RESET_USB: u32 = 4;
pub const RESET_U2DRD: u32 = 5;
pub const RESET_U3DRD: u32 = 6;
pub const RESET_U3DRD_PIPE0: u32 = 7;
pub const RESET_U2PHY20: u32 = 8;
pub const RESET_U2PHY21: u32 = 9;
pub const RESET_GDC: u32 = 10;
pub const RESET_HDMI20_AES: u32 = 11;
pub const RESET_HDMIRX: u32 = 12;
pub const RESET_HDMIRX_APB: u32 = 13;
pub const RESET_DEWARP: u32 = 14;
/*                                      15      */
pub const RESET_HDMITX_CAPB3: u32 = 16;
pub const RESET_BRG_VCBUG_DEC: u32 = 17;
pub const RESET_VCBUS: u32 = 18;
pub const RESET_VID_PLL_DIV: u32 = 19;
pub const RESET_VDI6: u32 = 20;
pub const RESET_GE2D: u32 = 21;
pub const RESET_HDMITXPHY: u32 = 22;
pub const RESET_VID_LOCK: u32 = 23;
pub const RESET_VENC0: u32 = 24;
pub const RESET_VDAC: u32 = 25;
pub const RESET_VENC2: u32 = 26;
pub const RESET_VENC1: u32 = 27;
pub const RESET_RDMA: u32 = 28;
pub const RESET_HDMITX: u32 = 29;
pub const RESET_VIU: u32 = 30;
pub const RESET_VENC: u32 = 31;

/* RESET1 */
pub const RESET_AUDIO: u32 = 32;
pub const RESET_MALI_CAPB3: u32 = 33;
pub const RESET_MALI: u32 = 34;
pub const RESET_DDR_APB: u32 = 35;
pub const RESET_DDR: u32 = 36;
pub const RESET_DOS_CAPB3: u32 = 37;
pub const RESET_DOS: u32 = 38;
pub const RESET_COMBO_DPHY_CHAN2: u32 = 39;
pub const RESET_DEBUG_B: u32 = 40;
pub const RESET_DEBUG_A: u32 = 41;
pub const RESET_DSP_B: u32 = 42;
pub const RESET_DSP_A: u32 = 43;
pub const RESET_PCIE_A: u32 = 44;
pub const RESET_PCIE_PHY: u32 = 45;
pub const RESET_PCIE_APB: u32 = 46;
pub const RESET_ANAKIN: u32 = 47;
pub const RESET_ETH: u32 = 48;
pub const RESET_EDP0_CTRL: u32 = 49;
pub const RESET_EDP1_CTRL: u32 = 50;
pub const RESET_COMBO_DPHY_CHAN0: u32 = 51;
pub const RESET_COMBO_DPHY_CHAN1: u32 = 52;
pub const RESET_DSI_LVDS_EDP_TOP: u32 = 53;
pub const RESET_PCIE1_PHY: u32 = 54;
pub const RESET_PCIE1_APB: u32 = 55;
pub const RESET_DDR_1: u32 = 56;
/*                                      57      */
pub const RESET_EDP1_PIPELINE: u32 = 58;
pub const RESET_EDP0_PIPELINE: u32 = 59;
pub const RESET_MIPI_DSI1_PHY: u32 = 60;
pub const RESET_MIPI_DSI0_PHY: u32 = 61;
pub const RESET_MIPI_DSI_A_HOST: u32 = 62;
pub const RESET_MIPI_DSI_B_HOST: u32 = 63;

/* RESET2 */
pub const RESET_DEVICE_MMC_ARB: u32 = 64;
pub const RESET_IR_CTRL: u32 = 65;
pub const RESET_TS_A73: u32 = 66;
pub const RESET_TS_A53: u32 = 67;
pub const RESET_SPICC_2: u32 = 68;
pub const RESET_SPICC_3: u32 = 69;
pub const RESET_SPICC_4: u32 = 70;
pub const RESET_SPICC_5: u32 = 71;
pub const RESET_SMART_CARD: u32 = 72;
pub const RESET_SPICC_0: u32 = 73;
pub const RESET_SPICC_1: u32 = 74;
pub const RESET_RSA: u32 = 75;
/*                                      76-79   */
pub const RESET_MSR_CLK: u32 = 80;
pub const RESET_SPIFC: u32 = 81;
pub const RESET_SAR_ADC: u32 = 82;
pub const RESET_BT: u32 = 83;
/*                                      84-87   */
pub const RESET_ACODEC: u32 = 88;
pub const RESET_CEC: u32 = 89;
pub const RESET_AFIFO: u32 = 90;
pub const RESET_WATCHDOG: u32 = 91;
/*                                      92-95   */

/* RESET3 */
pub const RESET_BRG_NIC1_GPV: u32 = 96;
pub const RESET_BRG_NIC2_GPV: u32 = 97;
pub const RESET_BRG_NIC3_GPV: u32 = 98;
pub const RESET_BRG_NIC4_GPV: u32 = 99;
pub const RESET_BRG_NIC5_GPV: u32 = 100;
/*                                      101-121 */
pub const RESET_MIPI_ISP: u32 = 122;
pub const RESET_BRG_ADB_MALI_1: u32 = 123;
pub const RESET_BRG_ADB_MALI_0: u32 = 124;
pub const RESET_BRG_ADB_A73: u32 = 125;
pub const RESET_BRG_ADB_A53: u32 = 126;
pub const RESET_BRG_CCI: u32 = 127;

/* RESET4 */
pub const RESET_PWM_AO_AB: u32 = 128;
pub const RESET_PWM_AO_CD: u32 = 129;
pub const RESET_PWM_AO_EF: u32 = 130;
pub const RESET_PWM_AO_GH: u32 = 131;
pub const RESET_PWM_AB: u32 = 132;
pub const RESET_PWM_CD: u32 = 133;
pub const RESET_PWM_EF: u32 = 134;
/*                                      135-137 */
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
pub const RESET_I2C_M_E: u32 = 149;
pub const RESET_I2C_M_F: u32 = 150;
pub const RESET_I2C_M_AO_A: u32 = 151;
pub const RESET_SD_EMMC_A: u32 = 152;
pub const RESET_SD_EMMC_B: u32 = 153;
pub const RESET_SD_EMMC_C: u32 = 154;
pub const RESET_I2C_M_AO_B: u32 = 155;
pub const RESET_TS_GPU: u32 = 156;
pub const RESET_TS_NNA: u32 = 157;
pub const RESET_TS_VPN: u32 = 158;
pub const RESET_TS_HEVC: u32 = 159;

/* RESET5 */
pub const RESET_BRG_NOC_DDR_1: u32 = 160;
pub const RESET_BRG_NOC_DDR_0: u32 = 161;
pub const RESET_BRG_NOC_MAIN: u32 = 162;
pub const RESET_BRG_NOC_ALL: u32 = 163;
/*                                      164-167 */
pub const RESET_BRG_NIC2_SYS: u32 = 168;
pub const RESET_BRG_NIC2_MAIN: u32 = 169;
pub const RESET_BRG_NIC2_HDMI: u32 = 170;
pub const RESET_BRG_NIC2_ALL: u32 = 171;
pub const RESET_BRG_NIC3_WAVE: u32 = 172;
pub const RESET_BRG_NIC3_VDEC: u32 = 173;
pub const RESET_BRG_NIC3_HEVCF: u32 = 174;
pub const RESET_BRG_NIC3_HEVCB: u32 = 175;
pub const RESET_BRG_NIC3_HCODEC: u32 = 176;
pub const RESET_BRG_NIC3_GE2D: u32 = 177;
pub const RESET_BRG_NIC3_GDC: u32 = 178;
pub const RESET_BRG_NIC3_AMLOGIC: u32 = 179;
pub const RESET_BRG_NIC3_MAIN: u32 = 180;
pub const RESET_BRG_NIC3_ALL: u32 = 181;
pub const RESET_BRG_NIC5_VPU: u32 = 182;
/*                                      183-185 */
pub const RESET_BRG_NIC4_DSPB: u32 = 186;
pub const RESET_BRG_NIC4_DSPA: u32 = 187;
pub const RESET_BRG_NIC4_VAPB: u32 = 188;
pub const RESET_BRG_NIC4_CLK81: u32 = 189;
pub const RESET_BRG_NIC4_MAIN: u32 = 190;
pub const RESET_BRG_NIC4_ALL: u32 = 191;

/* RESET6 */
pub const RESET_BRG_VDEC_PIPEL: u32 = 192;
pub const RESET_BRG_HEVCF_DMC_PIPEL: u32 = 193;
pub const RESET_BRG_NIC2TONIC4_PIPEL: u32 = 194;
pub const RESET_BRG_HDMIRXTONIC2_PIPEL: u32 = 195;
pub const RESET_BRG_SECTONIC4_PIPEL: u32 = 196;
pub const RESET_BRG_VPUTONOC_PIPEL: u32 = 197;
pub const RESET_BRG_NIC4TONOC_PIPEL: u32 = 198;
pub const RESET_BRG_NIC3TONOC_PIPEL: u32 = 199;
pub const RESET_BRG_NIC2TONOC_PIPEL: u32 = 200;
pub const RESET_BRG_NNATONOC_PIPEL: u32 = 201;
pub const RESET_BRG_FRISP3_PIPEL: u32 = 202;
pub const RESET_BRG_FRISP2_PIPEL: u32 = 203;
pub const RESET_BRG_FRISP1_PIPEL: u32 = 204;
pub const RESET_BRG_FRISP0_PIPEL: u32 = 205;
/*                                      206-217 */
pub const RESET_BRG_AMPIPE_NAND: u32 = 218;
pub const RESET_BRG_AMPIPE_ETH: u32 = 219;
/*                                      220     */
pub const RESET_BRG_AM2AXI0: u32 = 221;
pub const RESET_BRG_AM2AXI1: u32 = 222;
pub const RESET_BRG_AM2AXI2: u32 = 223;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
