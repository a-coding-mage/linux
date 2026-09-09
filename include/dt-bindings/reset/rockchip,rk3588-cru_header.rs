/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Copyright (c) 2021, 2024 Rockchip Electronics Co. Ltd.
 * Copyright (c) 2022 Collabora Ltd.
 *
 * Author: Elaine Zhang <zhangqing@rock-chips.com>
 * Author: Sebastian Reichel <sebastian.reichel@collabora.com>
 */


pub const SRST_A_TOP_BIU: u32 = 0;
pub const SRST_P_TOP_BIU: u32 = 1;
pub const SRST_P_CSIPHY0: u32 = 2;
pub const SRST_CSIPHY0: u32 = 3;
pub const SRST_P_CSIPHY1: u32 = 4;
pub const SRST_CSIPHY1: u32 = 5;
pub const SRST_A_TOP_M500_BIU: u32 = 6;

pub const SRST_A_TOP_M400_BIU: u32 = 7;
pub const SRST_A_TOP_S200_BIU: u32 = 8;
pub const SRST_A_TOP_S400_BIU: u32 = 9;
pub const SRST_A_TOP_M300_BIU: u32 = 10;
pub const SRST_USBDP_COMBO_PHY0_INIT: u32 = 11;
pub const SRST_USBDP_COMBO_PHY0_CMN: u32 = 12;
pub const SRST_USBDP_COMBO_PHY0_LANE: u32 = 13;
pub const SRST_USBDP_COMBO_PHY0_PCS: u32 = 14;
pub const SRST_USBDP_COMBO_PHY1_INIT: u32 = 15;

pub const SRST_USBDP_COMBO_PHY1_CMN: u32 = 16;
pub const SRST_USBDP_COMBO_PHY1_LANE: u32 = 17;
pub const SRST_USBDP_COMBO_PHY1_PCS: u32 = 18;
pub const SRST_DCPHY0: u32 = 19;
pub const SRST_P_MIPI_DCPHY0: u32 = 20;
pub const SRST_P_MIPI_DCPHY0_GRF: u32 = 21;

pub const SRST_DCPHY1: u32 = 22;
pub const SRST_P_MIPI_DCPHY1: u32 = 23;
pub const SRST_P_MIPI_DCPHY1_GRF: u32 = 24;
pub const SRST_P_APB2ASB_SLV_CDPHY: u32 = 25;
pub const SRST_P_APB2ASB_SLV_CSIPHY: u32 = 26;
pub const SRST_P_APB2ASB_SLV_VCCIO3_5: u32 = 27;
pub const SRST_P_APB2ASB_SLV_VCCIO6: u32 = 28;
pub const SRST_P_APB2ASB_SLV_EMMCIO: u32 = 29;
pub const SRST_P_APB2ASB_SLV_IOC_TOP: u32 = 30;
pub const SRST_P_APB2ASB_SLV_IOC_RIGHT: u32 = 31;

pub const SRST_P_CRU: u32 = 32;
pub const SRST_A_CHANNEL_SECURE2VO1USB: u32 = 33;
pub const SRST_A_CHANNEL_SECURE2CENTER: u32 = 34;
pub const SRST_H_CHANNEL_SECURE2VO1USB: u32 = 35;
pub const SRST_H_CHANNEL_SECURE2CENTER: u32 = 36;

pub const SRST_P_CHANNEL_SECURE2VO1USB: u32 = 37;
pub const SRST_P_CHANNEL_SECURE2CENTER: u32 = 38;

pub const SRST_H_AUDIO_BIU: u32 = 39;
pub const SRST_P_AUDIO_BIU: u32 = 40;
pub const SRST_H_I2S0_8CH: u32 = 41;
pub const SRST_M_I2S0_8CH_TX: u32 = 42;
pub const SRST_M_I2S0_8CH_RX: u32 = 43;
pub const SRST_P_ACDCDIG: u32 = 44;
pub const SRST_H_I2S2_2CH: u32 = 45;
pub const SRST_H_I2S3_2CH: u32 = 46;

pub const SRST_M_I2S2_2CH: u32 = 47;
pub const SRST_M_I2S3_2CH: u32 = 48;
pub const SRST_DAC_ACDCDIG: u32 = 49;
pub const SRST_H_SPDIF0: u32 = 50;

pub const SRST_M_SPDIF0: u32 = 51;
pub const SRST_H_SPDIF1: u32 = 52;
pub const SRST_M_SPDIF1: u32 = 53;
pub const SRST_H_PDM1: u32 = 54;
pub const SRST_PDM1: u32 = 55;

pub const SRST_A_BUS_BIU: u32 = 56;
pub const SRST_P_BUS_BIU: u32 = 57;
pub const SRST_A_GIC: u32 = 58;
pub const SRST_A_GIC_DBG: u32 = 59;
pub const SRST_A_DMAC0: u32 = 60;
pub const SRST_A_DMAC1: u32 = 61;
pub const SRST_A_DMAC2: u32 = 62;
pub const SRST_P_I2C1: u32 = 63;
pub const SRST_P_I2C2: u32 = 64;
pub const SRST_P_I2C3: u32 = 65;
pub const SRST_P_I2C4: u32 = 66;
pub const SRST_P_I2C5: u32 = 67;
pub const SRST_P_I2C6: u32 = 68;
pub const SRST_P_I2C7: u32 = 69;
pub const SRST_P_I2C8: u32 = 70;

pub const SRST_I2C1: u32 = 71;
pub const SRST_I2C2: u32 = 72;
pub const SRST_I2C3: u32 = 73;
pub const SRST_I2C4: u32 = 74;
pub const SRST_I2C5: u32 = 75;
pub const SRST_I2C6: u32 = 76;
pub const SRST_I2C7: u32 = 77;
pub const SRST_I2C8: u32 = 78;
pub const SRST_P_CAN0: u32 = 79;
pub const SRST_CAN0: u32 = 80;
pub const SRST_P_CAN1: u32 = 81;
pub const SRST_CAN1: u32 = 82;
pub const SRST_P_CAN2: u32 = 83;
pub const SRST_CAN2: u32 = 84;
pub const SRST_P_SARADC: u32 = 85;

pub const SRST_P_TSADC: u32 = 86;
pub const SRST_TSADC: u32 = 87;
pub const SRST_P_UART1: u32 = 88;
pub const SRST_P_UART2: u32 = 89;
pub const SRST_P_UART3: u32 = 90;
pub const SRST_P_UART4: u32 = 91;
pub const SRST_P_UART5: u32 = 92;
pub const SRST_P_UART6: u32 = 93;
pub const SRST_P_UART7: u32 = 94;
pub const SRST_P_UART8: u32 = 95;
pub const SRST_P_UART9: u32 = 96;
pub const SRST_S_UART1: u32 = 97;

pub const SRST_S_UART2: u32 = 98;
pub const SRST_S_UART3: u32 = 99;
pub const SRST_S_UART4: u32 = 100;
pub const SRST_S_UART5: u32 = 101;
pub const SRST_S_UART6: u32 = 102;
pub const SRST_S_UART7: u32 = 103;

pub const SRST_S_UART8: u32 = 104;
pub const SRST_S_UART9: u32 = 105;
pub const SRST_P_SPI0: u32 = 106;
pub const SRST_P_SPI1: u32 = 107;
pub const SRST_P_SPI2: u32 = 108;
pub const SRST_P_SPI3: u32 = 109;
pub const SRST_P_SPI4: u32 = 110;
pub const SRST_SPI0: u32 = 111;
pub const SRST_SPI1: u32 = 112;
pub const SRST_SPI2: u32 = 113;
pub const SRST_SPI3: u32 = 114;
pub const SRST_SPI4: u32 = 115;

pub const SRST_P_WDT0: u32 = 116;
pub const SRST_T_WDT0: u32 = 117;
pub const SRST_P_SYS_GRF: u32 = 118;
pub const SRST_P_PWM1: u32 = 119;
pub const SRST_PWM1: u32 = 120;
pub const SRST_P_PWM2: u32 = 121;
pub const SRST_PWM2: u32 = 122;
pub const SRST_P_PWM3: u32 = 123;
pub const SRST_PWM3: u32 = 124;
pub const SRST_P_BUSTIMER0: u32 = 125;
pub const SRST_P_BUSTIMER1: u32 = 126;
pub const SRST_BUSTIMER0: u32 = 127;

pub const SRST_BUSTIMER1: u32 = 128;
pub const SRST_BUSTIMER2: u32 = 129;
pub const SRST_BUSTIMER3: u32 = 130;
pub const SRST_BUSTIMER4: u32 = 131;
pub const SRST_BUSTIMER5: u32 = 132;
pub const SRST_BUSTIMER6: u32 = 133;
pub const SRST_BUSTIMER7: u32 = 134;
pub const SRST_BUSTIMER8: u32 = 135;
pub const SRST_BUSTIMER9: u32 = 136;
pub const SRST_BUSTIMER10: u32 = 137;
pub const SRST_BUSTIMER11: u32 = 138;
pub const SRST_P_MAILBOX0: u32 = 139;
pub const SRST_P_MAILBOX1: u32 = 140;
pub const SRST_P_MAILBOX2: u32 = 141;
pub const SRST_P_GPIO1: u32 = 142;
pub const SRST_GPIO1: u32 = 143;

pub const SRST_P_GPIO2: u32 = 144;
pub const SRST_GPIO2: u32 = 145;
pub const SRST_P_GPIO3: u32 = 146;
pub const SRST_GPIO3: u32 = 147;
pub const SRST_P_GPIO4: u32 = 148;
pub const SRST_GPIO4: u32 = 149;
pub const SRST_A_DECOM: u32 = 150;
pub const SRST_P_DECOM: u32 = 151;
pub const SRST_D_DECOM: u32 = 152;
pub const SRST_P_TOP: u32 = 153;
pub const SRST_A_GICADB_GIC2CORE_BUS: u32 = 154;
pub const SRST_P_DFT2APB: u32 = 155;
pub const SRST_P_APB2ASB_MST_TOP: u32 = 156;
pub const SRST_P_APB2ASB_MST_CDPHY: u32 = 157;
pub const SRST_P_APB2ASB_MST_BOT_RIGHT: u32 = 158;

pub const SRST_P_APB2ASB_MST_IOC_TOP: u32 = 159;
pub const SRST_P_APB2ASB_MST_IOC_RIGHT: u32 = 160;
pub const SRST_P_APB2ASB_MST_CSIPHY: u32 = 161;
pub const SRST_P_APB2ASB_MST_VCCIO3_5: u32 = 162;
pub const SRST_P_APB2ASB_MST_VCCIO6: u32 = 163;
pub const SRST_P_APB2ASB_MST_EMMCIO: u32 = 164;
pub const SRST_A_SPINLOCK: u32 = 165;
pub const SRST_P_OTPC_NS: u32 = 166;
pub const SRST_OTPC_NS: u32 = 167;
pub const SRST_OTPC_ARB: u32 = 168;

pub const SRST_P_BUSIOC: u32 = 169;
pub const SRST_P_PMUCM0_INTMUX: u32 = 170;
pub const SRST_P_DDRCM0_INTMUX: u32 = 171;

pub const SRST_P_DDR_DFICTL_CH0: u32 = 172;
pub const SRST_P_DDR_MON_CH0: u32 = 173;
pub const SRST_P_DDR_STANDBY_CH0: u32 = 174;
pub const SRST_P_DDR_UPCTL_CH0: u32 = 175;
pub const SRST_TM_DDR_MON_CH0: u32 = 176;
pub const SRST_P_DDR_GRF_CH01: u32 = 177;
pub const SRST_DFI_CH0: u32 = 178;
pub const SRST_SBR_CH0: u32 = 179;
pub const SRST_DDR_UPCTL_CH0: u32 = 180;
pub const SRST_DDR_DFICTL_CH0: u32 = 181;
pub const SRST_DDR_MON_CH0: u32 = 182;
pub const SRST_DDR_STANDBY_CH0: u32 = 183;
pub const SRST_A_DDR_UPCTL_CH0: u32 = 184;
pub const SRST_P_DDR_DFICTL_CH1: u32 = 185;
pub const SRST_P_DDR_MON_CH1: u32 = 186;
pub const SRST_P_DDR_STANDBY_CH1: u32 = 187;

pub const SRST_P_DDR_UPCTL_CH1: u32 = 188;
pub const SRST_TM_DDR_MON_CH1: u32 = 189;
pub const SRST_DFI_CH1: u32 = 190;
pub const SRST_SBR_CH1: u32 = 191;
pub const SRST_DDR_UPCTL_CH1: u32 = 192;
pub const SRST_DDR_DFICTL_CH1: u32 = 193;
pub const SRST_DDR_MON_CH1: u32 = 194;
pub const SRST_DDR_STANDBY_CH1: u32 = 195;
pub const SRST_A_DDR_UPCTL_CH1: u32 = 196;
pub const SRST_A_DDR01_MSCH0: u32 = 197;
pub const SRST_A_DDR01_RS_MSCH0: u32 = 198;
pub const SRST_A_DDR01_FRS_MSCH0: u32 = 199;

pub const SRST_A_DDR01_SCRAMBLE0: u32 = 200;
pub const SRST_A_DDR01_FRS_SCRAMBLE0: u32 = 201;
pub const SRST_A_DDR01_MSCH1: u32 = 202;
pub const SRST_A_DDR01_RS_MSCH1: u32 = 203;
pub const SRST_A_DDR01_FRS_MSCH1: u32 = 204;
pub const SRST_A_DDR01_SCRAMBLE1: u32 = 205;
pub const SRST_A_DDR01_FRS_SCRAMBLE1: u32 = 206;
pub const SRST_P_DDR01_MSCH0: u32 = 207;
pub const SRST_P_DDR01_MSCH1: u32 = 208;

pub const SRST_P_DDR_DFICTL_CH2: u32 = 209;
pub const SRST_P_DDR_MON_CH2: u32 = 210;
pub const SRST_P_DDR_STANDBY_CH2: u32 = 211;
pub const SRST_P_DDR_UPCTL_CH2: u32 = 212;
pub const SRST_TM_DDR_MON_CH2: u32 = 213;
pub const SRST_P_DDR_GRF_CH23: u32 = 214;
pub const SRST_DFI_CH2: u32 = 215;
pub const SRST_SBR_CH2: u32 = 216;
pub const SRST_DDR_UPCTL_CH2: u32 = 217;
pub const SRST_DDR_DFICTL_CH2: u32 = 218;
pub const SRST_DDR_MON_CH2: u32 = 219;
pub const SRST_DDR_STANDBY_CH2: u32 = 220;
pub const SRST_A_DDR_UPCTL_CH2: u32 = 221;
pub const SRST_P_DDR_DFICTL_CH3: u32 = 222;
pub const SRST_P_DDR_MON_CH3: u32 = 223;
pub const SRST_P_DDR_STANDBY_CH3: u32 = 224;

pub const SRST_P_DDR_UPCTL_CH3: u32 = 225;
pub const SRST_TM_DDR_MON_CH3: u32 = 226;
pub const SRST_DFI_CH3: u32 = 227;
pub const SRST_SBR_CH3: u32 = 228;
pub const SRST_DDR_UPCTL_CH3: u32 = 229;
pub const SRST_DDR_DFICTL_CH3: u32 = 230;
pub const SRST_DDR_MON_CH3: u32 = 231;
pub const SRST_DDR_STANDBY_CH3: u32 = 232;
pub const SRST_A_DDR_UPCTL_CH3: u32 = 233;
pub const SRST_A_DDR23_MSCH2: u32 = 234;
pub const SRST_A_DDR23_RS_MSCH2: u32 = 235;
pub const SRST_A_DDR23_FRS_MSCH2: u32 = 236;

pub const SRST_A_DDR23_SCRAMBLE2: u32 = 237;
pub const SRST_A_DDR23_FRS_SCRAMBLE2: u32 = 238;
pub const SRST_A_DDR23_MSCH3: u32 = 239;
pub const SRST_A_DDR23_RS_MSCH3: u32 = 240;
pub const SRST_A_DDR23_FRS_MSCH3: u32 = 241;
pub const SRST_A_DDR23_SCRAMBLE3: u32 = 242;
pub const SRST_A_DDR23_FRS_SCRAMBLE3: u32 = 243;
pub const SRST_P_DDR23_MSCH2: u32 = 244;
pub const SRST_P_DDR23_MSCH3: u32 = 245;

pub const SRST_ISP1: u32 = 246;
pub const SRST_ISP1_VICAP: u32 = 247;
pub const SRST_A_ISP1_BIU: u32 = 248;
pub const SRST_H_ISP1_BIU: u32 = 249;

pub const SRST_A_RKNN1: u32 = 250;
pub const SRST_A_RKNN1_BIU: u32 = 251;
pub const SRST_H_RKNN1: u32 = 252;
pub const SRST_H_RKNN1_BIU: u32 = 253;

pub const SRST_A_RKNN2: u32 = 254;
pub const SRST_A_RKNN2_BIU: u32 = 255;
pub const SRST_H_RKNN2: u32 = 256;
pub const SRST_H_RKNN2_BIU: u32 = 257;

pub const SRST_A_RKNN_DSU0: u32 = 258;
pub const SRST_P_NPUTOP_BIU: u32 = 259;
pub const SRST_P_NPU_TIMER: u32 = 260;
pub const SRST_NPUTIMER0: u32 = 261;
pub const SRST_NPUTIMER1: u32 = 262;
pub const SRST_P_NPU_WDT: u32 = 263;
pub const SRST_T_NPU_WDT: u32 = 264;
pub const SRST_P_NPU_PVTM: u32 = 265;
pub const SRST_P_NPU_GRF: u32 = 266;
pub const SRST_NPU_PVTM: u32 = 267;

pub const SRST_NPU_PVTPLL: u32 = 268;
pub const SRST_H_NPU_CM0_BIU: u32 = 269;
pub const SRST_F_NPU_CM0_CORE: u32 = 270;
pub const SRST_T_NPU_CM0_JTAG: u32 = 271;
pub const SRST_A_RKNN0: u32 = 272;
pub const SRST_A_RKNN0_BIU: u32 = 273;
pub const SRST_H_RKNN0: u32 = 274;
pub const SRST_H_RKNN0_BIU: u32 = 275;

pub const SRST_H_NVM_BIU: u32 = 276;
pub const SRST_A_NVM_BIU: u32 = 277;
pub const SRST_H_EMMC: u32 = 278;
pub const SRST_A_EMMC: u32 = 279;
pub const SRST_C_EMMC: u32 = 280;
pub const SRST_B_EMMC: u32 = 281;
pub const SRST_T_EMMC: u32 = 282;
pub const SRST_S_SFC: u32 = 283;
pub const SRST_H_SFC: u32 = 284;
pub const SRST_H_SFC_XIP: u32 = 285;

pub const SRST_P_GRF: u32 = 286;
pub const SRST_P_DEC_BIU: u32 = 287;
pub const SRST_P_PHP_BIU: u32 = 288;
pub const SRST_A_PCIE_GRIDGE: u32 = 289;
pub const SRST_A_PHP_BIU: u32 = 290;
pub const SRST_A_GMAC0: u32 = 291;
pub const SRST_A_GMAC1: u32 = 292;
pub const SRST_A_PCIE_BIU: u32 = 293;
pub const SRST_PCIE0_POWER_UP: u32 = 294;
pub const SRST_PCIE1_POWER_UP: u32 = 295;
pub const SRST_PCIE2_POWER_UP: u32 = 296;

pub const SRST_PCIE3_POWER_UP: u32 = 297;
pub const SRST_PCIE4_POWER_UP: u32 = 298;
pub const SRST_P_PCIE0: u32 = 299;
pub const SRST_P_PCIE1: u32 = 300;
pub const SRST_P_PCIE2: u32 = 301;
pub const SRST_P_PCIE3: u32 = 302;

pub const SRST_P_PCIE4: u32 = 303;
pub const SRST_A_PHP_GIC_ITS: u32 = 304;
pub const SRST_A_MMU_PCIE: u32 = 305;
pub const SRST_A_MMU_PHP: u32 = 306;
pub const SRST_A_MMU_BIU: u32 = 307;

pub const SRST_A_USB3OTG2: u32 = 308;

pub const SRST_PMALIVE0: u32 = 309;
pub const SRST_PMALIVE1: u32 = 310;
pub const SRST_PMALIVE2: u32 = 311;
pub const SRST_A_SATA0: u32 = 312;
pub const SRST_A_SATA1: u32 = 313;
pub const SRST_A_SATA2: u32 = 314;
pub const SRST_RXOOB0: u32 = 315;
pub const SRST_RXOOB1: u32 = 316;
pub const SRST_RXOOB2: u32 = 317;
pub const SRST_ASIC0: u32 = 318;
pub const SRST_ASIC1: u32 = 319;
pub const SRST_ASIC2: u32 = 320;

pub const SRST_A_RKVDEC_CCU: u32 = 321;
pub const SRST_H_RKVDEC0: u32 = 322;
pub const SRST_A_RKVDEC0: u32 = 323;
pub const SRST_H_RKVDEC0_BIU: u32 = 324;
pub const SRST_A_RKVDEC0_BIU: u32 = 325;
pub const SRST_RKVDEC0_CA: u32 = 326;
pub const SRST_RKVDEC0_HEVC_CA: u32 = 327;
pub const SRST_RKVDEC0_CORE: u32 = 328;

pub const SRST_H_RKVDEC1: u32 = 329;
pub const SRST_A_RKVDEC1: u32 = 330;
pub const SRST_H_RKVDEC1_BIU: u32 = 331;
pub const SRST_A_RKVDEC1_BIU: u32 = 332;
pub const SRST_RKVDEC1_CA: u32 = 333;
pub const SRST_RKVDEC1_HEVC_CA: u32 = 334;
pub const SRST_RKVDEC1_CORE: u32 = 335;

pub const SRST_A_USB_BIU: u32 = 336;
pub const SRST_H_USB_BIU: u32 = 337;
pub const SRST_A_USB3OTG0: u32 = 338;
pub const SRST_A_USB3OTG1: u32 = 339;
pub const SRST_H_HOST0: u32 = 340;
pub const SRST_H_HOST_ARB0: u32 = 341;
pub const SRST_H_HOST1: u32 = 342;
pub const SRST_H_HOST_ARB1: u32 = 343;
pub const SRST_A_USB_GRF: u32 = 344;
pub const SRST_C_USB2P0_HOST0: u32 = 345;

pub const SRST_C_USB2P0_HOST1: u32 = 346;
pub const SRST_HOST_UTMI0: u32 = 347;
pub const SRST_HOST_UTMI1: u32 = 348;

pub const SRST_A_VDPU_BIU: u32 = 349;
pub const SRST_A_VDPU_LOW_BIU: u32 = 350;
pub const SRST_H_VDPU_BIU: u32 = 351;
pub const SRST_A_JPEG_DECODER_BIU: u32 = 352;
pub const SRST_A_VPU: u32 = 353;
pub const SRST_H_VPU: u32 = 354;
pub const SRST_A_JPEG_ENCODER0: u32 = 355;
pub const SRST_H_JPEG_ENCODER0: u32 = 356;
pub const SRST_A_JPEG_ENCODER1: u32 = 357;
pub const SRST_H_JPEG_ENCODER1: u32 = 358;
pub const SRST_A_JPEG_ENCODER2: u32 = 359;
pub const SRST_H_JPEG_ENCODER2: u32 = 360;

pub const SRST_A_JPEG_ENCODER3: u32 = 361;
pub const SRST_H_JPEG_ENCODER3: u32 = 362;
pub const SRST_A_JPEG_DECODER: u32 = 363;
pub const SRST_H_JPEG_DECODER: u32 = 364;
pub const SRST_H_IEP2P0: u32 = 365;
pub const SRST_A_IEP2P0: u32 = 366;
pub const SRST_IEP2P0_CORE: u32 = 367;
pub const SRST_H_RGA2: u32 = 368;
pub const SRST_A_RGA2: u32 = 369;
pub const SRST_RGA2_CORE: u32 = 370;
pub const SRST_H_RGA3_0: u32 = 371;
pub const SRST_A_RGA3_0: u32 = 372;
pub const SRST_RGA3_0_CORE: u32 = 373;

pub const SRST_H_RKVENC0_BIU: u32 = 374;
pub const SRST_A_RKVENC0_BIU: u32 = 375;
pub const SRST_H_RKVENC0: u32 = 376;
pub const SRST_A_RKVENC0: u32 = 377;
pub const SRST_RKVENC0_CORE: u32 = 378;

pub const SRST_H_RKVENC1_BIU: u32 = 379;
pub const SRST_A_RKVENC1_BIU: u32 = 380;
pub const SRST_H_RKVENC1: u32 = 381;
pub const SRST_A_RKVENC1: u32 = 382;
pub const SRST_RKVENC1_CORE: u32 = 383;

pub const SRST_A_VI_BIU: u32 = 384;
pub const SRST_H_VI_BIU: u32 = 385;
pub const SRST_P_VI_BIU: u32 = 386;
pub const SRST_D_VICAP: u32 = 387;
pub const SRST_A_VICAP: u32 = 388;
pub const SRST_H_VICAP: u32 = 389;
pub const SRST_ISP0: u32 = 390;
pub const SRST_ISP0_VICAP: u32 = 391;

pub const SRST_FISHEYE0: u32 = 392;
pub const SRST_FISHEYE1: u32 = 393;
pub const SRST_P_CSI_HOST_0: u32 = 394;
pub const SRST_P_CSI_HOST_1: u32 = 395;
pub const SRST_P_CSI_HOST_2: u32 = 396;
pub const SRST_P_CSI_HOST_3: u32 = 397;
pub const SRST_P_CSI_HOST_4: u32 = 398;
pub const SRST_P_CSI_HOST_5: u32 = 399;

pub const SRST_CSIHOST0_VICAP: u32 = 400;
pub const SRST_CSIHOST1_VICAP: u32 = 401;
pub const SRST_CSIHOST2_VICAP: u32 = 402;
pub const SRST_CSIHOST3_VICAP: u32 = 403;
pub const SRST_CSIHOST4_VICAP: u32 = 404;
pub const SRST_CSIHOST5_VICAP: u32 = 405;
pub const SRST_CIFIN: u32 = 406;

pub const SRST_A_VOP_BIU: u32 = 407;
pub const SRST_A_VOP_LOW_BIU: u32 = 408;
pub const SRST_H_VOP_BIU: u32 = 409;
pub const SRST_P_VOP_BIU: u32 = 410;
pub const SRST_H_VOP: u32 = 411;
pub const SRST_A_VOP: u32 = 412;
pub const SRST_D_VOP0: u32 = 413;
pub const SRST_D_VOP2HDMI_BRIDGE0: u32 = 414;
pub const SRST_D_VOP2HDMI_BRIDGE1: u32 = 415;

pub const SRST_D_VOP1: u32 = 416;
pub const SRST_D_VOP2: u32 = 417;
pub const SRST_D_VOP3: u32 = 418;
pub const SRST_P_VOPGRF: u32 = 419;
pub const SRST_P_DSIHOST0: u32 = 420;
pub const SRST_P_DSIHOST1: u32 = 421;
pub const SRST_DSIHOST0: u32 = 422;
pub const SRST_DSIHOST1: u32 = 423;
pub const SRST_VOP_PMU: u32 = 424;
pub const SRST_P_VOP_CHANNEL_BIU: u32 = 425;

pub const SRST_H_VO0_BIU: u32 = 426;
pub const SRST_H_VO0_S_BIU: u32 = 427;
pub const SRST_P_VO0_BIU: u32 = 428;
pub const SRST_P_VO0_S_BIU: u32 = 429;
pub const SRST_A_HDCP0_BIU: u32 = 430;
pub const SRST_P_VO0GRF: u32 = 431;
pub const SRST_H_HDCP_KEY0: u32 = 432;
pub const SRST_A_HDCP0: u32 = 433;
pub const SRST_H_HDCP0: u32 = 434;
pub const SRST_HDCP0: u32 = 435;

pub const SRST_P_TRNG0: u32 = 436;
pub const SRST_DP0: u32 = 437;
pub const SRST_DP1: u32 = 438;
pub const SRST_H_I2S4_8CH: u32 = 439;
pub const SRST_M_I2S4_8CH_TX: u32 = 440;
pub const SRST_H_I2S8_8CH: u32 = 441;

pub const SRST_M_I2S8_8CH_TX: u32 = 442;
pub const SRST_H_SPDIF2_DP0: u32 = 443;
pub const SRST_M_SPDIF2_DP0: u32 = 444;
pub const SRST_H_SPDIF5_DP1: u32 = 445;
pub const SRST_M_SPDIF5_DP1: u32 = 446;

pub const SRST_A_HDCP1_BIU: u32 = 447;
pub const SRST_A_VO1_BIU: u32 = 448;
pub const SRST_H_VOP1_BIU: u32 = 449;
pub const SRST_H_VOP1_S_BIU: u32 = 450;
pub const SRST_P_VOP1_BIU: u32 = 451;
pub const SRST_P_VO1GRF: u32 = 452;
pub const SRST_P_VO1_S_BIU: u32 = 453;

pub const SRST_H_I2S7_8CH: u32 = 454;
pub const SRST_M_I2S7_8CH_RX: u32 = 455;
pub const SRST_H_HDCP_KEY1: u32 = 456;
pub const SRST_A_HDCP1: u32 = 457;
pub const SRST_H_HDCP1: u32 = 458;
pub const SRST_HDCP1: u32 = 459;
pub const SRST_P_TRNG1: u32 = 460;
pub const SRST_P_HDMITX0: u32 = 461;

pub const SRST_HDMITX0_REF: u32 = 462;
pub const SRST_P_HDMITX1: u32 = 463;
pub const SRST_HDMITX1_REF: u32 = 464;
pub const SRST_A_HDMIRX: u32 = 465;
pub const SRST_P_HDMIRX: u32 = 466;
pub const SRST_HDMIRX_REF: u32 = 467;

pub const SRST_P_EDP0: u32 = 468;
pub const SRST_EDP0_24M: u32 = 469;
pub const SRST_P_EDP1: u32 = 470;
pub const SRST_EDP1_24M: u32 = 471;
pub const SRST_M_I2S5_8CH_TX: u32 = 472;
pub const SRST_H_I2S5_8CH: u32 = 473;
pub const SRST_M_I2S6_8CH_TX: u32 = 474;

pub const SRST_M_I2S6_8CH_RX: u32 = 475;
pub const SRST_H_I2S6_8CH: u32 = 476;
pub const SRST_H_SPDIF3: u32 = 477;
pub const SRST_M_SPDIF3: u32 = 478;
pub const SRST_H_SPDIF4: u32 = 479;
pub const SRST_M_SPDIF4: u32 = 480;
pub const SRST_H_SPDIFRX0: u32 = 481;
pub const SRST_M_SPDIFRX0: u32 = 482;
pub const SRST_H_SPDIFRX1: u32 = 483;
pub const SRST_M_SPDIFRX1: u32 = 484;

pub const SRST_H_SPDIFRX2: u32 = 485;
pub const SRST_M_SPDIFRX2: u32 = 486;
pub const SRST_LINKSYM_HDMITXPHY0: u32 = 487;
pub const SRST_LINKSYM_HDMITXPHY1: u32 = 488;
pub const SRST_VO1_BRIDGE0: u32 = 489;
pub const SRST_VO1_BRIDGE1: u32 = 490;

pub const SRST_H_I2S9_8CH: u32 = 491;
pub const SRST_M_I2S9_8CH_RX: u32 = 492;
pub const SRST_H_I2S10_8CH: u32 = 493;
pub const SRST_M_I2S10_8CH_RX: u32 = 494;
pub const SRST_P_S_HDMIRX: u32 = 495;

pub const SRST_GPU: u32 = 496;
pub const SRST_SYS_GPU: u32 = 497;
pub const SRST_A_S_GPU_BIU: u32 = 498;
pub const SRST_A_M0_GPU_BIU: u32 = 499;
pub const SRST_A_M1_GPU_BIU: u32 = 500;
pub const SRST_A_M2_GPU_BIU: u32 = 501;
pub const SRST_A_M3_GPU_BIU: u32 = 502;
pub const SRST_P_GPU_BIU: u32 = 503;
pub const SRST_P_GPU_PVTM: u32 = 504;

pub const SRST_GPU_PVTM: u32 = 505;
pub const SRST_P_GPU_GRF: u32 = 506;
pub const SRST_GPU_PVTPLL: u32 = 507;
pub const SRST_GPU_JTAG: u32 = 508;

pub const SRST_A_AV1_BIU: u32 = 509;
pub const SRST_A_AV1: u32 = 510;
pub const SRST_P_AV1_BIU: u32 = 511;
pub const SRST_P_AV1: u32 = 512;

pub const SRST_A_DDR_BIU: u32 = 513;
pub const SRST_A_DMA2DDR: u32 = 514;
pub const SRST_A_DDR_SHAREMEM: u32 = 515;
pub const SRST_A_DDR_SHAREMEM_BIU: u32 = 516;
pub const SRST_A_CENTER_S200_BIU: u32 = 517;
pub const SRST_A_CENTER_S400_BIU: u32 = 518;
pub const SRST_H_AHB2APB: u32 = 519;
pub const SRST_H_CENTER_BIU: u32 = 520;
pub const SRST_F_DDR_CM0_CORE: u32 = 521;

pub const SRST_DDR_TIMER0: u32 = 522;
pub const SRST_DDR_TIMER1: u32 = 523;
pub const SRST_T_WDT_DDR: u32 = 524;
pub const SRST_T_DDR_CM0_JTAG: u32 = 525;
pub const SRST_P_CENTER_GRF: u32 = 526;
pub const SRST_P_AHB2APB: u32 = 527;
pub const SRST_P_WDT: u32 = 528;
pub const SRST_P_TIMER: u32 = 529;
pub const SRST_P_DMA2DDR: u32 = 530;
pub const SRST_P_SHAREMEM: u32 = 531;
pub const SRST_P_CENTER_BIU: u32 = 532;
pub const SRST_P_CENTER_CHANNEL_BIU: u32 = 533;

pub const SRST_P_USBDPGRF0: u32 = 534;
pub const SRST_P_USBDPPHY0: u32 = 535;
pub const SRST_P_USBDPGRF1: u32 = 536;
pub const SRST_P_USBDPPHY1: u32 = 537;
pub const SRST_P_HDPTX0: u32 = 538;
pub const SRST_P_HDPTX1: u32 = 539;
pub const SRST_P_APB2ASB_SLV_BOT_RIGHT: u32 = 540;
pub const SRST_P_USB2PHY_U3_0_GRF0: u32 = 541;
pub const SRST_P_USB2PHY_U3_1_GRF0: u32 = 542;
pub const SRST_P_USB2PHY_U2_0_GRF0: u32 = 543;
pub const SRST_P_USB2PHY_U2_1_GRF0: u32 = 544;
pub const SRST_HDPTX0_ROPLL: u32 = 545;
pub const SRST_HDPTX0_LCPLL: u32 = 546;
pub const SRST_HDPTX0: u32 = 547;
pub const SRST_HDPTX1_ROPLL: u32 = 548;

pub const SRST_HDPTX1_LCPLL: u32 = 549;
pub const SRST_HDPTX1: u32 = 550;
pub const SRST_HDPTX0_HDMIRXPHY_SET: u32 = 551;
pub const SRST_USBDP_COMBO_PHY0: u32 = 552;
pub const SRST_USBDP_COMBO_PHY0_LCPLL: u32 = 553;
pub const SRST_USBDP_COMBO_PHY0_ROPLL: u32 = 554;
pub const SRST_USBDP_COMBO_PHY0_PCS_HS: u32 = 555;
pub const SRST_USBDP_COMBO_PHY1: u32 = 556;
pub const SRST_USBDP_COMBO_PHY1_LCPLL: u32 = 557;
pub const SRST_USBDP_COMBO_PHY1_ROPLL: u32 = 558;
pub const SRST_USBDP_COMBO_PHY1_PCS_HS: u32 = 559;
pub const SRST_HDMIHDP0: u32 = 560;
pub const SRST_HDMIHDP1: u32 = 561;

pub const SRST_A_VO1USB_TOP_BIU: u32 = 562;
pub const SRST_H_VO1USB_TOP_BIU: u32 = 563;

pub const SRST_H_SDIO_BIU: u32 = 564;
pub const SRST_H_SDIO: u32 = 565;
pub const SRST_SDIO: u32 = 566;

pub const SRST_H_RGA3_BIU: u32 = 567;
pub const SRST_A_RGA3_BIU: u32 = 568;
pub const SRST_H_RGA3_1: u32 = 569;
pub const SRST_A_RGA3_1: u32 = 570;
pub const SRST_RGA3_1_CORE: u32 = 571;

pub const SRST_REF_PIPE_PHY0: u32 = 572;
pub const SRST_REF_PIPE_PHY1: u32 = 573;
pub const SRST_REF_PIPE_PHY2: u32 = 574;

pub const SRST_P_PHPTOP_CRU: u32 = 575;
pub const SRST_P_PCIE2_GRF0: u32 = 576;
pub const SRST_P_PCIE2_GRF1: u32 = 577;
pub const SRST_P_PCIE2_GRF2: u32 = 578;
pub const SRST_P_PCIE2_PHY0: u32 = 579;
pub const SRST_P_PCIE2_PHY1: u32 = 580;
pub const SRST_P_PCIE2_PHY2: u32 = 581;
pub const SRST_P_PCIE3_PHY: u32 = 582;
pub const SRST_P_APB2ASB_SLV_CHIP_TOP: u32 = 583;
pub const SRST_PCIE30_PHY: u32 = 584;

pub const SRST_H_PMU1_BIU: u32 = 585;
pub const SRST_P_PMU1_BIU: u32 = 586;
pub const SRST_H_PMU_CM0_BIU: u32 = 587;
pub const SRST_F_PMU_CM0_CORE: u32 = 588;
pub const SRST_T_PMU1_CM0_JTAG: u32 = 589;

pub const SRST_DDR_FAIL_SAFE: u32 = 590;
pub const SRST_P_CRU_PMU1: u32 = 591;
pub const SRST_P_PMU1_GRF: u32 = 592;
pub const SRST_P_PMU1_IOC: u32 = 593;
pub const SRST_P_PMU1WDT: u32 = 594;
pub const SRST_T_PMU1WDT: u32 = 595;
pub const SRST_P_PMU1TIMER: u32 = 596;
pub const SRST_PMU1TIMER0: u32 = 597;
pub const SRST_PMU1TIMER1: u32 = 598;
pub const SRST_P_PMU1PWM: u32 = 599;
pub const SRST_PMU1PWM: u32 = 600;

pub const SRST_P_I2C0: u32 = 601;
pub const SRST_I2C0: u32 = 602;
pub const SRST_S_UART0: u32 = 603;
pub const SRST_P_UART0: u32 = 604;
pub const SRST_H_I2S1_8CH: u32 = 605;
pub const SRST_M_I2S1_8CH_TX: u32 = 606;
pub const SRST_M_I2S1_8CH_RX: u32 = 607;
pub const SRST_H_PDM0: u32 = 608;
pub const SRST_PDM0: u32 = 609;

pub const SRST_H_VAD: u32 = 610;
pub const SRST_HDPTX0_INIT: u32 = 611;
pub const SRST_HDPTX0_CMN: u32 = 612;
pub const SRST_HDPTX0_LANE: u32 = 613;
pub const SRST_HDPTX1_INIT: u32 = 614;

pub const SRST_HDPTX1_CMN: u32 = 615;
pub const SRST_HDPTX1_LANE: u32 = 616;
pub const SRST_M_MIPI_DCPHY0: u32 = 617;
pub const SRST_S_MIPI_DCPHY0: u32 = 618;
pub const SRST_M_MIPI_DCPHY1: u32 = 619;
pub const SRST_S_MIPI_DCPHY1: u32 = 620;
pub const SRST_OTGPHY_U3_0: u32 = 621;
pub const SRST_OTGPHY_U3_1: u32 = 622;
pub const SRST_OTGPHY_U2_0: u32 = 623;
pub const SRST_OTGPHY_U2_1: u32 = 624;

pub const SRST_P_PMU0GRF: u32 = 625;
pub const SRST_P_PMU0IOC: u32 = 626;
pub const SRST_P_GPIO0: u32 = 627;
pub const SRST_GPIO0: u32 = 628;

pub const SRST_A_SECURE_NS_BIU: u32 = 629;
pub const SRST_H_SECURE_NS_BIU: u32 = 630;
pub const SRST_A_SECURE_S_BIU: u32 = 631;
pub const SRST_H_SECURE_S_BIU: u32 = 632;
pub const SRST_P_SECURE_S_BIU: u32 = 633;
pub const SRST_CRYPTO_CORE: u32 = 634;

pub const SRST_CRYPTO_PKA: u32 = 635;
pub const SRST_CRYPTO_RNG: u32 = 636;
pub const SRST_A_CRYPTO: u32 = 637;
pub const SRST_H_CRYPTO: u32 = 638;
pub const SRST_KEYLADDER_CORE: u32 = 639;
pub const SRST_KEYLADDER_RNG: u32 = 640;
pub const SRST_A_KEYLADDER: u32 = 641;
pub const SRST_H_KEYLADDER: u32 = 642;
pub const SRST_P_OTPC_S: u32 = 643;
pub const SRST_OTPC_S: u32 = 644;
pub const SRST_WDT_S: u32 = 645;

pub const SRST_T_WDT_S: u32 = 646;
pub const SRST_H_BOOTROM: u32 = 647;
pub const SRST_A_DCF: u32 = 648;
pub const SRST_P_DCF: u32 = 649;
pub const SRST_H_BOOTROM_NS: u32 = 650;
pub const SRST_P_KEYLADDER: u32 = 651;
pub const SRST_H_TRNG_S: u32 = 652;

pub const SRST_H_TRNG_NS: u32 = 653;
pub const SRST_D_SDMMC_BUFFER: u32 = 654;
pub const SRST_H_SDMMC: u32 = 655;
pub const SRST_H_SDMMC_BUFFER: u32 = 656;
pub const SRST_SDMMC: u32 = 657;
pub const SRST_P_TRNG_CHK: u32 = 658;
pub const SRST_TRNG_S: u32 = 659;

pub const SRST_A_HDMIRX_BIU: u32 = 660;

/* SCMI Secure Resets */

/* Name=SECURE_SOFTRST_CON00,Offset=0xA00 */
pub const SCMI_SRST_A_SECURE_NS_BIU: u32 = 10;
pub const SCMI_SRST_H_SECURE_NS_BIU: u32 = 11;
pub const SCMI_SRST_A_SECURE_S_BIU: u32 = 12;
pub const SCMI_SRST_H_SECURE_S_BIU: u32 = 13;
pub const SCMI_SRST_P_SECURE_S_BIU: u32 = 14;
pub const SCMI_SRST_CRYPTO_CORE: u32 = 15;
/* Name=SECURE_SOFTRST_CON01,Offset=0xA04 */
pub const SCMI_SRST_CRYPTO_PKA: u32 = 16;
pub const SCMI_SRST_CRYPTO_RNG: u32 = 17;
pub const SCMI_SRST_A_CRYPTO: u32 = 18;
pub const SCMI_SRST_H_CRYPTO: u32 = 19;
pub const SCMI_SRST_KEYLADDER_CORE: u32 = 25;
pub const SCMI_SRST_KEYLADDER_RNG: u32 = 26;
pub const SCMI_SRST_A_KEYLADDER: u32 = 27;
pub const SCMI_SRST_H_KEYLADDER: u32 = 28;
pub const SCMI_SRST_P_OTPC_S: u32 = 29;
pub const SCMI_SRST_OTPC_S: u32 = 30;
pub const SCMI_SRST_WDT_S: u32 = 31;
/* Name=SECURE_SOFTRST_CON02,Offset=0xA08 */
pub const SCMI_SRST_T_WDT_S: u32 = 32;
pub const SCMI_SRST_H_BOOTROM: u32 = 33;
pub const SCMI_SRST_A_DCF: u32 = 34;
pub const SCMI_SRST_P_DCF: u32 = 35;
pub const SCMI_SRST_H_BOOTROM_NS: u32 = 37;
pub const SCMI_SRST_P_KEYLADDER: u32 = 46;
pub const SCMI_SRST_H_TRNG_S: u32 = 47;
/* Name=SECURE_SOFTRST_CON03,Offset=0xA0C */
pub const SCMI_SRST_H_TRNG_NS: u32 = 48;
pub const SCMI_SRST_D_SDMMC_BUFFER: u32 = 49;
pub const SCMI_SRST_H_SDMMC: u32 = 50;
pub const SCMI_SRST_H_SDMMC_BUFFER: u32 = 51;
pub const SCMI_SRST_SDMMC: u32 = 52;
pub const SCMI_SRST_P_TRNG_CHK: u32 = 53;
pub const SCMI_SRST_TRNG_S: u32 = 54;




// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
