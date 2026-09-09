/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Copyright (c) 2023 Rockchip Electronics Co. Ltd.
 * Copyright (c) 2024 Collabora Ltd.
 *
 * Author: Elaine Zhang <zhangqing@rock-chips.com>
 * Author: Detlev Casanova <detlev.casanova@collabora.com>
 */


pub const SRST_A_TOP_BIU: u32 = 0;
pub const SRST_P_TOP_BIU: u32 = 1;
pub const SRST_A_TOP_MID_BIU: u32 = 2;
pub const SRST_A_SECURE_HIGH_BIU: u32 = 3;
pub const SRST_H_TOP_BIU: u32 = 4;

pub const SRST_H_VO0VOP_CHANNEL_BIU: u32 = 5;
pub const SRST_A_VO0VOP_CHANNEL_BIU: u32 = 6;

pub const SRST_BISRINTF: u32 = 7;

pub const SRST_H_AUDIO_BIU: u32 = 8;
pub const SRST_H_ASRC_2CH_0: u32 = 9;
pub const SRST_H_ASRC_2CH_1: u32 = 10;
pub const SRST_H_ASRC_4CH_0: u32 = 11;
pub const SRST_H_ASRC_4CH_1: u32 = 12;
pub const SRST_ASRC_2CH_0: u32 = 13;
pub const SRST_ASRC_2CH_1: u32 = 14;
pub const SRST_ASRC_4CH_0: u32 = 15;
pub const SRST_ASRC_4CH_1: u32 = 16;
pub const SRST_M_SAI0_8CH: u32 = 17;
pub const SRST_H_SAI0_8CH: u32 = 18;
pub const SRST_H_SPDIF_RX0: u32 = 19;
pub const SRST_M_SPDIF_RX0: u32 = 20;

pub const SRST_H_SPDIF_RX1: u32 = 21;
pub const SRST_M_SPDIF_RX1: u32 = 22;
pub const SRST_M_SAI1_8CH: u32 = 23;
pub const SRST_H_SAI1_8CH: u32 = 24;
pub const SRST_M_SAI2_2CH: u32 = 25;
pub const SRST_H_SAI2_2CH: u32 = 26;
pub const SRST_M_SAI3_2CH: u32 = 27;
pub const SRST_H_SAI3_2CH: u32 = 28;

pub const SRST_M_SAI4_2CH: u32 = 29;
pub const SRST_H_SAI4_2CH: u32 = 30;
pub const SRST_H_ACDCDIG_DSM: u32 = 31;
pub const SRST_M_ACDCDIG_DSM: u32 = 32;
pub const SRST_PDM1: u32 = 33;
pub const SRST_H_PDM1: u32 = 34;
pub const SRST_M_PDM1: u32 = 35;
pub const SRST_H_SPDIF_TX0: u32 = 36;
pub const SRST_M_SPDIF_TX0: u32 = 37;
pub const SRST_H_SPDIF_TX1: u32 = 38;
pub const SRST_M_SPDIF_TX1: u32 = 39;

pub const SRST_A_BUS_BIU: u32 = 40;
pub const SRST_P_BUS_BIU: u32 = 41;
pub const SRST_P_CRU: u32 = 42;
pub const SRST_H_CAN0: u32 = 43;
pub const SRST_CAN0: u32 = 44;
pub const SRST_H_CAN1: u32 = 45;
pub const SRST_CAN1: u32 = 46;
pub const SRST_P_INTMUX2BUS: u32 = 47;
pub const SRST_P_VCCIO_IOC: u32 = 48;
pub const SRST_H_BUS_BIU: u32 = 49;
pub const SRST_KEY_SHIFT: u32 = 50;

pub const SRST_P_I2C1: u32 = 51;
pub const SRST_P_I2C2: u32 = 52;
pub const SRST_P_I2C3: u32 = 53;
pub const SRST_P_I2C4: u32 = 54;
pub const SRST_P_I2C5: u32 = 55;
pub const SRST_P_I2C6: u32 = 56;
pub const SRST_P_I2C7: u32 = 57;
pub const SRST_P_I2C8: u32 = 58;
pub const SRST_P_I2C9: u32 = 59;
pub const SRST_P_WDT_BUSMCU: u32 = 60;
pub const SRST_T_WDT_BUSMCU: u32 = 61;
pub const SRST_A_GIC: u32 = 62;
pub const SRST_I2C1: u32 = 63;
pub const SRST_I2C2: u32 = 64;
pub const SRST_I2C3: u32 = 65;
pub const SRST_I2C4: u32 = 66;

pub const SRST_I2C5: u32 = 67;
pub const SRST_I2C6: u32 = 68;
pub const SRST_I2C7: u32 = 69;
pub const SRST_I2C8: u32 = 70;
pub const SRST_I2C9: u32 = 71;
pub const SRST_P_SARADC: u32 = 72;
pub const SRST_SARADC: u32 = 73;
pub const SRST_P_TSADC: u32 = 74;
pub const SRST_TSADC: u32 = 75;
pub const SRST_P_UART0: u32 = 76;
pub const SRST_P_UART2: u32 = 77;
pub const SRST_P_UART3: u32 = 78;
pub const SRST_P_UART4: u32 = 79;
pub const SRST_P_UART5: u32 = 80;
pub const SRST_P_UART6: u32 = 81;

pub const SRST_P_UART7: u32 = 82;
pub const SRST_P_UART8: u32 = 83;
pub const SRST_P_UART9: u32 = 84;
pub const SRST_P_UART10: u32 = 85;
pub const SRST_P_UART11: u32 = 86;
pub const SRST_S_UART0: u32 = 87;
pub const SRST_S_UART2: u32 = 88;
pub const SRST_S_UART3: u32 = 89;
pub const SRST_S_UART4: u32 = 90;
pub const SRST_S_UART5: u32 = 91;

pub const SRST_S_UART6: u32 = 92;
pub const SRST_S_UART7: u32 = 93;
pub const SRST_S_UART8: u32 = 94;
pub const SRST_S_UART9: u32 = 95;
pub const SRST_S_UART10: u32 = 96;
pub const SRST_S_UART11: u32 = 97;
pub const SRST_P_SPI0: u32 = 98;
pub const SRST_P_SPI1: u32 = 99;
pub const SRST_P_SPI2: u32 = 100;

pub const SRST_P_SPI3: u32 = 101;
pub const SRST_P_SPI4: u32 = 102;
pub const SRST_SPI0: u32 = 103;
pub const SRST_SPI1: u32 = 104;
pub const SRST_SPI2: u32 = 105;
pub const SRST_SPI3: u32 = 106;
pub const SRST_SPI4: u32 = 107;
pub const SRST_P_WDT0: u32 = 108;
pub const SRST_T_WDT0: u32 = 109;
pub const SRST_P_SYS_GRF: u32 = 110;
pub const SRST_P_PWM1: u32 = 111;
pub const SRST_PWM1: u32 = 112;

pub const SRST_P_BUSTIMER0: u32 = 113;
pub const SRST_P_BUSTIMER1: u32 = 114;
pub const SRST_TIMER0: u32 = 115;
pub const SRST_TIMER1: u32 = 116;
pub const SRST_TIMER2: u32 = 117;
pub const SRST_TIMER3: u32 = 118;
pub const SRST_TIMER4: u32 = 119;
pub const SRST_TIMER5: u32 = 120;
pub const SRST_P_BUSIOC: u32 = 121;
pub const SRST_P_MAILBOX0: u32 = 122;
pub const SRST_P_GPIO1: u32 = 123;

pub const SRST_GPIO1: u32 = 124;
pub const SRST_P_GPIO2: u32 = 125;
pub const SRST_GPIO2: u32 = 126;
pub const SRST_P_GPIO3: u32 = 127;
pub const SRST_GPIO3: u32 = 128;
pub const SRST_P_GPIO4: u32 = 129;
pub const SRST_GPIO4: u32 = 130;
pub const SRST_A_DECOM: u32 = 131;
pub const SRST_P_DECOM: u32 = 132;
pub const SRST_D_DECOM: u32 = 133;
pub const SRST_TIMER6: u32 = 134;
pub const SRST_TIMER7: u32 = 135;
pub const SRST_TIMER8: u32 = 136;
pub const SRST_TIMER9: u32 = 137;
pub const SRST_TIMER10: u32 = 138;

pub const SRST_TIMER11: u32 = 139;
pub const SRST_A_DMAC0: u32 = 140;
pub const SRST_A_DMAC1: u32 = 141;
pub const SRST_A_DMAC2: u32 = 142;
pub const SRST_A_SPINLOCK: u32 = 143;
pub const SRST_REF_PVTPLL_BUS: u32 = 144;
pub const SRST_H_I3C0: u32 = 145;
pub const SRST_H_I3C1: u32 = 146;
pub const SRST_H_BUS_CM0_BIU: u32 = 147;
pub const SRST_F_BUS_CM0_CORE: u32 = 148;
pub const SRST_T_BUS_CM0_JTAG: u32 = 149;

pub const SRST_P_INTMUX2PMU: u32 = 150;
pub const SRST_P_INTMUX2DDR: u32 = 151;
pub const SRST_P_PVTPLL_BUS: u32 = 152;
pub const SRST_P_PWM2: u32 = 153;
pub const SRST_PWM2: u32 = 154;
pub const SRST_FREQ_PWM1: u32 = 155;
pub const SRST_COUNTER_PWM1: u32 = 156;
pub const SRST_I3C0: u32 = 157;
pub const SRST_I3C1: u32 = 158;

pub const SRST_P_DDR_MON_CH0: u32 = 159;
pub const SRST_P_DDR_BIU: u32 = 160;
pub const SRST_P_DDR_UPCTL_CH0: u32 = 161;
pub const SRST_TM_DDR_MON_CH0: u32 = 162;
pub const SRST_A_DDR_BIU: u32 = 163;
pub const SRST_DFI_CH0: u32 = 164;
pub const SRST_DDR_MON_CH0: u32 = 165;
pub const SRST_P_DDR_HWLP_CH0: u32 = 166;
pub const SRST_P_DDR_MON_CH1: u32 = 167;
pub const SRST_P_DDR_HWLP_CH1: u32 = 168;

pub const SRST_P_DDR_UPCTL_CH1: u32 = 169;
pub const SRST_TM_DDR_MON_CH1: u32 = 170;
pub const SRST_DFI_CH1: u32 = 171;
pub const SRST_A_DDR01_MSCH0: u32 = 172;
pub const SRST_A_DDR01_MSCH1: u32 = 173;
pub const SRST_DDR_MON_CH1: u32 = 174;
pub const SRST_DDR_SCRAMBLE_CH0: u32 = 175;
pub const SRST_DDR_SCRAMBLE_CH1: u32 = 176;
pub const SRST_P_AHB2APB: u32 = 177;
pub const SRST_H_AHB2APB: u32 = 178;
pub const SRST_H_DDR_BIU: u32 = 179;
pub const SRST_F_DDR_CM0_CORE: u32 = 180;

pub const SRST_P_DDR01_MSCH0: u32 = 181;
pub const SRST_P_DDR01_MSCH1: u32 = 182;
pub const SRST_DDR_TIMER0: u32 = 183;
pub const SRST_DDR_TIMER1: u32 = 184;
pub const SRST_T_WDT_DDR: u32 = 185;
pub const SRST_P_WDT: u32 = 186;
pub const SRST_P_TIMER: u32 = 187;
pub const SRST_T_DDR_CM0_JTAG: u32 = 188;
pub const SRST_P_DDR_GRF: u32 = 189;

pub const SRST_DDR_UPCTL_CH0: u32 = 190;
pub const SRST_A_DDR_UPCTL_0_CH0: u32 = 191;
pub const SRST_A_DDR_UPCTL_1_CH0: u32 = 192;
pub const SRST_A_DDR_UPCTL_2_CH0: u32 = 193;
pub const SRST_A_DDR_UPCTL_3_CH0: u32 = 194;
pub const SRST_A_DDR_UPCTL_4_CH0: u32 = 195;

pub const SRST_DDR_UPCTL_CH1: u32 = 196;
pub const SRST_A_DDR_UPCTL_0_CH1: u32 = 197;
pub const SRST_A_DDR_UPCTL_1_CH1: u32 = 198;
pub const SRST_A_DDR_UPCTL_2_CH1: u32 = 199;
pub const SRST_A_DDR_UPCTL_3_CH1: u32 = 200;
pub const SRST_A_DDR_UPCTL_4_CH1: u32 = 201;

pub const SRST_REF_PVTPLL_DDR: u32 = 202;
pub const SRST_P_PVTPLL_DDR: u32 = 203;

pub const SRST_A_RKNN0: u32 = 204;
pub const SRST_A_RKNN0_BIU: u32 = 205;
pub const SRST_L_RKNN0_BIU: u32 = 206;

pub const SRST_A_RKNN1: u32 = 207;
pub const SRST_A_RKNN1_BIU: u32 = 208;
pub const SRST_L_RKNN1_BIU: u32 = 209;

pub const SRST_NPU_DAP: u32 = 210;
pub const SRST_L_NPUSUBSYS_BIU: u32 = 211;
pub const SRST_P_NPUTOP_BIU: u32 = 212;
pub const SRST_P_NPU_TIMER: u32 = 213;
pub const SRST_NPUTIMER0: u32 = 214;
pub const SRST_NPUTIMER1: u32 = 215;
pub const SRST_P_NPU_WDT: u32 = 216;
pub const SRST_T_NPU_WDT: u32 = 217;

pub const SRST_A_RKNN_CBUF: u32 = 218;
pub const SRST_A_RVCORE0: u32 = 219;
pub const SRST_P_NPU_GRF: u32 = 220;
pub const SRST_P_PVTPLL_NPU: u32 = 221;
pub const SRST_NPU_PVTPLL: u32 = 222;
pub const SRST_H_NPU_CM0_BIU: u32 = 223;
pub const SRST_F_NPU_CM0_CORE: u32 = 224;
pub const SRST_T_NPU_CM0_JTAG: u32 = 225;
pub const SRST_A_RKNNTOP_BIU: u32 = 226;
pub const SRST_H_RKNN_CBUF: u32 = 227;
pub const SRST_H_RKNNTOP_BIU: u32 = 228;

pub const SRST_H_NVM_BIU: u32 = 229;
pub const SRST_A_NVM_BIU: u32 = 230;
pub const SRST_S_FSPI: u32 = 231;
pub const SRST_H_FSPI: u32 = 232;
pub const SRST_C_EMMC: u32 = 233;
pub const SRST_H_EMMC: u32 = 234;
pub const SRST_A_EMMC: u32 = 235;
pub const SRST_B_EMMC: u32 = 236;
pub const SRST_T_EMMC: u32 = 237;

pub const SRST_P_GRF: u32 = 238;
pub const SRST_P_PHP_BIU: u32 = 239;
pub const SRST_A_PHP_BIU: u32 = 240;
pub const SRST_P_PCIE0: u32 = 241;
pub const SRST_PCIE0_POWER_UP: u32 = 242;

pub const SRST_A_USB3OTG1: u32 = 243;
pub const SRST_A_MMU0: u32 = 244;
pub const SRST_A_SLV_MMU0: u32 = 245;
pub const SRST_A_MMU1: u32 = 246;

pub const SRST_A_SLV_MMU1: u32 = 247;
pub const SRST_P_PCIE1: u32 = 248;
pub const SRST_PCIE1_POWER_UP: u32 = 249;

pub const SRST_RXOOB0: u32 = 250;
pub const SRST_RXOOB1: u32 = 251;
pub const SRST_PMALIVE0: u32 = 252;
pub const SRST_PMALIVE1: u32 = 253;
pub const SRST_A_SATA0: u32 = 254;
pub const SRST_A_SATA1: u32 = 255;
pub const SRST_ASIC1: u32 = 256;
pub const SRST_ASIC0: u32 = 257;

pub const SRST_P_CSIDPHY1: u32 = 258;
pub const SRST_SCAN_CSIDPHY1: u32 = 259;

pub const SRST_P_SDGMAC_GRF: u32 = 260;
pub const SRST_P_SDGMAC_BIU: u32 = 261;
pub const SRST_A_SDGMAC_BIU: u32 = 262;
pub const SRST_H_SDGMAC_BIU: u32 = 263;
pub const SRST_A_GMAC0: u32 = 264;
pub const SRST_A_GMAC1: u32 = 265;
pub const SRST_P_GMAC0: u32 = 266;
pub const SRST_P_GMAC1: u32 = 267;
pub const SRST_H_SDIO: u32 = 268;

pub const SRST_H_SDMMC0: u32 = 269;
pub const SRST_S_FSPI1: u32 = 270;
pub const SRST_H_FSPI1: u32 = 271;
pub const SRST_A_DSMC_BIU: u32 = 272;
pub const SRST_A_DSMC: u32 = 273;
pub const SRST_P_DSMC: u32 = 274;
pub const SRST_H_HSGPIO: u32 = 275;
pub const SRST_HSGPIO: u32 = 276;
pub const SRST_A_HSGPIO: u32 = 277;

pub const SRST_H_RKVDEC: u32 = 278;
pub const SRST_H_RKVDEC_BIU: u32 = 279;
pub const SRST_A_RKVDEC_BIU: u32 = 280;
pub const SRST_RKVDEC_HEVC_CA: u32 = 281;
pub const SRST_RKVDEC_CORE: u32 = 282;

pub const SRST_A_USB_BIU: u32 = 283;
pub const SRST_P_USBUFS_BIU: u32 = 284;
pub const SRST_A_USB3OTG0: u32 = 285;
pub const SRST_A_UFS_BIU: u32 = 286;
pub const SRST_A_MMU2: u32 = 287;
pub const SRST_A_SLV_MMU2: u32 = 288;
pub const SRST_A_UFS_SYS: u32 = 289;

pub const SRST_A_UFS: u32 = 290;
pub const SRST_P_USBUFS_GRF: u32 = 291;
pub const SRST_P_UFS_GRF: u32 = 292;

pub const SRST_H_VPU_BIU: u32 = 293;
pub const SRST_A_JPEG_BIU: u32 = 294;
pub const SRST_A_RGA_BIU: u32 = 295;
pub const SRST_A_VDPP_BIU: u32 = 296;
pub const SRST_A_EBC_BIU: u32 = 297;
pub const SRST_H_RGA2E_0: u32 = 298;
pub const SRST_A_RGA2E_0: u32 = 299;
pub const SRST_CORE_RGA2E_0: u32 = 300;

pub const SRST_A_JPEG: u32 = 301;
pub const SRST_H_JPEG: u32 = 302;
pub const SRST_H_VDPP: u32 = 303;
pub const SRST_A_VDPP: u32 = 304;
pub const SRST_CORE_VDPP: u32 = 305;
pub const SRST_H_RGA2E_1: u32 = 306;
pub const SRST_A_RGA2E_1: u32 = 307;
pub const SRST_CORE_RGA2E_1: u32 = 308;
pub const SRST_H_EBC: u32 = 309;
pub const SRST_A_EBC: u32 = 310;
pub const SRST_D_EBC: u32 = 311;

pub const SRST_H_VEPU0_BIU: u32 = 312;
pub const SRST_A_VEPU0_BIU: u32 = 313;
pub const SRST_H_VEPU0: u32 = 314;
pub const SRST_A_VEPU0: u32 = 315;
pub const SRST_VEPU0_CORE: u32 = 316;

pub const SRST_A_VI_BIU: u32 = 317;
pub const SRST_H_VI_BIU: u32 = 318;
pub const SRST_P_VI_BIU: u32 = 319;
pub const SRST_D_VICAP: u32 = 320;
pub const SRST_A_VICAP: u32 = 321;
pub const SRST_H_VICAP: u32 = 322;
pub const SRST_ISP0: u32 = 323;
pub const SRST_ISP0_VICAP: u32 = 324;

pub const SRST_CORE_VPSS: u32 = 325;
pub const SRST_P_CSI_HOST_0: u32 = 326;
pub const SRST_P_CSI_HOST_1: u32 = 327;
pub const SRST_P_CSI_HOST_2: u32 = 328;
pub const SRST_P_CSI_HOST_3: u32 = 329;
pub const SRST_P_CSI_HOST_4: u32 = 330;

pub const SRST_CIFIN: u32 = 331;
pub const SRST_VICAP_I0CLK: u32 = 332;
pub const SRST_VICAP_I1CLK: u32 = 333;
pub const SRST_VICAP_I2CLK: u32 = 334;
pub const SRST_VICAP_I3CLK: u32 = 335;
pub const SRST_VICAP_I4CLK: u32 = 336;

pub const SRST_A_VOP_BIU: u32 = 337;
pub const SRST_A_VOP2_BIU: u32 = 338;
pub const SRST_H_VOP_BIU: u32 = 339;
pub const SRST_P_VOP_BIU: u32 = 340;
pub const SRST_H_VOP: u32 = 341;
pub const SRST_A_VOP: u32 = 342;
pub const SRST_D_VP0: u32 = 343;

pub const SRST_D_VP1: u32 = 344;
pub const SRST_D_VP2: u32 = 345;
pub const SRST_P_VOP2_BIU: u32 = 346;
pub const SRST_P_VOPGRF: u32 = 347;

pub const SRST_H_VO0_BIU: u32 = 348;
pub const SRST_P_VO0_BIU: u32 = 349;
pub const SRST_A_HDCP0_BIU: u32 = 350;
pub const SRST_P_VO0_GRF: u32 = 351;
pub const SRST_A_HDCP0: u32 = 352;
pub const SRST_H_HDCP0: u32 = 353;
pub const SRST_HDCP0: u32 = 354;

pub const SRST_P_DSIHOST0: u32 = 355;
pub const SRST_DSIHOST0: u32 = 356;
pub const SRST_P_HDMITX0: u32 = 357;
pub const SRST_HDMITX0_REF: u32 = 358;
pub const SRST_P_EDP0: u32 = 359;
pub const SRST_EDP0_24M: u32 = 360;

pub const SRST_M_SAI5_8CH: u32 = 361;
pub const SRST_H_SAI5_8CH: u32 = 362;
pub const SRST_M_SAI6_8CH: u32 = 363;
pub const SRST_H_SAI6_8CH: u32 = 364;
pub const SRST_H_SPDIF_TX2: u32 = 365;
pub const SRST_M_SPDIF_TX2: u32 = 366;
pub const SRST_H_SPDIF_RX2: u32 = 367;
pub const SRST_M_SPDIF_RX2: u32 = 368;

pub const SRST_H_SAI8_8CH: u32 = 369;
pub const SRST_M_SAI8_8CH: u32 = 370;

pub const SRST_H_VO1_BIU: u32 = 371;
pub const SRST_P_VO1_BIU: u32 = 372;
pub const SRST_M_SAI7_8CH: u32 = 373;
pub const SRST_H_SAI7_8CH: u32 = 374;
pub const SRST_H_SPDIF_TX3: u32 = 375;
pub const SRST_H_SPDIF_TX4: u32 = 376;
pub const SRST_H_SPDIF_TX5: u32 = 377;
pub const SRST_M_SPDIF_TX3: u32 = 378;

pub const SRST_DP0: u32 = 379;
pub const SRST_P_VO1_GRF: u32 = 380;
pub const SRST_A_HDCP1_BIU: u32 = 381;
pub const SRST_A_HDCP1: u32 = 382;
pub const SRST_H_HDCP1: u32 = 383;
pub const SRST_HDCP1: u32 = 384;
pub const SRST_H_SAI9_8CH: u32 = 385;
pub const SRST_M_SAI9_8CH: u32 = 386;
pub const SRST_M_SPDIF_TX4: u32 = 387;
pub const SRST_M_SPDIF_TX5: u32 = 388;

pub const SRST_GPU: u32 = 389;
pub const SRST_A_S_GPU_BIU: u32 = 390;
pub const SRST_A_M0_GPU_BIU: u32 = 391;
pub const SRST_P_GPU_BIU: u32 = 392;
pub const SRST_P_GPU_GRF: u32 = 393;
pub const SRST_GPU_PVTPLL: u32 = 394;
pub const SRST_P_PVTPLL_GPU: u32 = 395;

pub const SRST_A_CENTER_BIU: u32 = 396;
pub const SRST_A_DMA2DDR: u32 = 397;
pub const SRST_A_DDR_SHAREMEM: u32 = 398;
pub const SRST_A_DDR_SHAREMEM_BIU: u32 = 399;
pub const SRST_H_CENTER_BIU: u32 = 400;
pub const SRST_P_CENTER_GRF: u32 = 401;
pub const SRST_P_DMA2DDR: u32 = 402;
pub const SRST_P_SHAREMEM: u32 = 403;
pub const SRST_P_CENTER_BIU: u32 = 404;

pub const SRST_LINKSYM_HDMITXPHY0: u32 = 405;

pub const SRST_DP0_PIXELCLK: u32 = 406;
pub const SRST_PHY_DP0_TX: u32 = 407;
pub const SRST_DP1_PIXELCLK: u32 = 408;
pub const SRST_DP2_PIXELCLK: u32 = 409;

pub const SRST_H_VEPU1_BIU: u32 = 410;
pub const SRST_A_VEPU1_BIU: u32 = 411;
pub const SRST_H_VEPU1: u32 = 412;
pub const SRST_A_VEPU1: u32 = 413;
pub const SRST_VEPU1_CORE: u32 = 414;

pub const SRST_P_PHPPHY_CRU: u32 = 415;
pub const SRST_P_APB2ASB_SLV_CHIP_TOP: u32 = 416;
pub const SRST_P_PCIE2_COMBOPHY0: u32 = 417;
pub const SRST_P_PCIE2_COMBOPHY0_GRF: u32 = 418;
pub const SRST_P_PCIE2_COMBOPHY1: u32 = 419;
pub const SRST_P_PCIE2_COMBOPHY1_GRF: u32 = 420;

pub const SRST_PCIE0_PIPE_PHY: u32 = 421;
pub const SRST_PCIE1_PIPE_PHY: u32 = 422;

pub const SRST_H_CRYPTO_NS: u32 = 423;
pub const SRST_H_TRNG_NS: u32 = 424;
pub const SRST_P_OTPC_NS: u32 = 425;
pub const SRST_OTPC_NS: u32 = 426;

pub const SRST_P_HDPTX_GRF: u32 = 427;
pub const SRST_P_HDPTX_APB: u32 = 428;
pub const SRST_P_MIPI_DCPHY: u32 = 429;
pub const SRST_P_DCPHY_GRF: u32 = 430;
pub const SRST_P_BOT0_APB2ASB: u32 = 431;
pub const SRST_P_BOT1_APB2ASB: u32 = 432;
pub const SRST_USB2DEBUG: u32 = 433;
pub const SRST_P_CSIPHY_GRF: u32 = 434;
pub const SRST_P_CSIPHY: u32 = 435;
pub const SRST_P_USBPHY_GRF_0: u32 = 436;
pub const SRST_P_USBPHY_GRF_1: u32 = 437;
pub const SRST_P_USBDP_GRF: u32 = 438;
pub const SRST_P_USBDPPHY: u32 = 439;
pub const SRST_USBDP_COMBO_PHY_INIT: u32 = 440;

pub const SRST_USBDP_COMBO_PHY_CMN: u32 = 441;
pub const SRST_USBDP_COMBO_PHY_LANE: u32 = 442;
pub const SRST_USBDP_COMBO_PHY_PCS: u32 = 443;
pub const SRST_M_MIPI_DCPHY: u32 = 444;
pub const SRST_S_MIPI_DCPHY: u32 = 445;
pub const SRST_SCAN_CSIPHY: u32 = 446;
pub const SRST_P_VCCIO6_IOC: u32 = 447;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
