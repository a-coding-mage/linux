/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2016 Rockchip Electronics Co. Ltd.
 * Author: Xing Zheng <zhengxing@rock-chips.com>
 */


/* core clocks */
pub const PLL_APLLL: u32 = 1;
pub const PLL_APLLB: u32 = 2;
pub const PLL_DPLL: u32 = 3;
pub const PLL_CPLL: u32 = 4;
pub const PLL_GPLL: u32 = 5;
pub const PLL_NPLL: u32 = 6;
pub const PLL_VPLL: u32 = 7;
pub const ARMCLKL: u32 = 8;
pub const ARMCLKB: u32 = 9;

/* sclk gates (special clocks) */
pub const SCLK_I2C1: u32 = 65;
pub const SCLK_I2C2: u32 = 66;
pub const SCLK_I2C3: u32 = 67;
pub const SCLK_I2C5: u32 = 68;
pub const SCLK_I2C6: u32 = 69;
pub const SCLK_I2C7: u32 = 70;
pub const SCLK_SPI0: u32 = 71;
pub const SCLK_SPI1: u32 = 72;
pub const SCLK_SPI2: u32 = 73;
pub const SCLK_SPI4: u32 = 74;
pub const SCLK_SPI5: u32 = 75;
pub const SCLK_SDMMC: u32 = 76;
pub const SCLK_SDIO: u32 = 77;
pub const SCLK_EMMC: u32 = 78;
pub const SCLK_TSADC: u32 = 79;
pub const SCLK_SARADC: u32 = 80;
pub const SCLK_UART0: u32 = 81;
pub const SCLK_UART1: u32 = 82;
pub const SCLK_UART2: u32 = 83;
pub const SCLK_UART3: u32 = 84;
pub const SCLK_SPDIF_8CH: u32 = 85;
pub const SCLK_I2S0_8CH: u32 = 86;
pub const SCLK_I2S1_8CH: u32 = 87;
pub const SCLK_I2S2_8CH: u32 = 88;
pub const SCLK_I2S_8CH_OUT: u32 = 89;
pub const SCLK_TIMER00: u32 = 90;
pub const SCLK_TIMER01: u32 = 91;
pub const SCLK_TIMER02: u32 = 92;
pub const SCLK_TIMER03: u32 = 93;
pub const SCLK_TIMER04: u32 = 94;
pub const SCLK_TIMER05: u32 = 95;
pub const SCLK_TIMER06: u32 = 96;
pub const SCLK_TIMER07: u32 = 97;
pub const SCLK_TIMER08: u32 = 98;
pub const SCLK_TIMER09: u32 = 99;
pub const SCLK_TIMER10: u32 = 100;
pub const SCLK_TIMER11: u32 = 101;
pub const SCLK_MACREF: u32 = 102;
pub const SCLK_MAC_RX: u32 = 103;
pub const SCLK_MAC_TX: u32 = 104;
pub const SCLK_MAC: u32 = 105;
pub const SCLK_MACREF_OUT: u32 = 106;
pub const SCLK_VOP0_PWM: u32 = 107;
pub const SCLK_VOP1_PWM: u32 = 108;
pub const SCLK_RGA_CORE: u32 = 109;
pub const SCLK_ISP0: u32 = 110;
pub const SCLK_ISP1: u32 = 111;
pub const SCLK_HDMI_CEC: u32 = 112;
pub const SCLK_HDMI_SFR: u32 = 113;
pub const SCLK_DP_CORE: u32 = 114;
pub const SCLK_PVTM_CORE_L: u32 = 115;
pub const SCLK_PVTM_CORE_B: u32 = 116;
pub const SCLK_PVTM_GPU: u32 = 117;
pub const SCLK_PVTM_DDR: u32 = 118;
pub const SCLK_MIPIDPHY_REF: u32 = 119;
pub const SCLK_MIPIDPHY_CFG: u32 = 120;
pub const SCLK_HSICPHY: u32 = 121;
pub const SCLK_USBPHY480M: u32 = 122;
pub const SCLK_USB2PHY0_REF: u32 = 123;
pub const SCLK_USB2PHY1_REF: u32 = 124;
pub const SCLK_UPHY0_TCPDPHY_REF: u32 = 125;
pub const SCLK_UPHY0_TCPDCORE: u32 = 126;
pub const SCLK_UPHY1_TCPDPHY_REF: u32 = 127;
pub const SCLK_UPHY1_TCPDCORE: u32 = 128;
pub const SCLK_USB3OTG0_REF: u32 = 129;
pub const SCLK_USB3OTG1_REF: u32 = 130;
pub const SCLK_USB3OTG0_SUSPEND: u32 = 131;
pub const SCLK_USB3OTG1_SUSPEND: u32 = 132;
pub const SCLK_CRYPTO0: u32 = 133;
pub const SCLK_CRYPTO1: u32 = 134;
pub const SCLK_CCI_TRACE: u32 = 135;
pub const SCLK_CS: u32 = 136;
pub const SCLK_CIF_OUT: u32 = 137;
pub const SCLK_PCIEPHY_REF: u32 = 138;
pub const SCLK_PCIE_CORE: u32 = 139;
pub const SCLK_M0_PERILP: u32 = 140;
pub const SCLK_M0_PERILP_DEC: u32 = 141;
pub const SCLK_CM0S: u32 = 142;
pub const SCLK_DBG_NOC: u32 = 143;
pub const SCLK_DBG_PD_CORE_B: u32 = 144;
pub const SCLK_DBG_PD_CORE_L: u32 = 145;
pub const SCLK_DFIMON0_TIMER: u32 = 146;
pub const SCLK_DFIMON1_TIMER: u32 = 147;
pub const SCLK_INTMEM0: u32 = 148;
pub const SCLK_INTMEM1: u32 = 149;
pub const SCLK_INTMEM2: u32 = 150;
pub const SCLK_INTMEM3: u32 = 151;
pub const SCLK_INTMEM4: u32 = 152;
pub const SCLK_INTMEM5: u32 = 153;
pub const SCLK_SDMMC_DRV: u32 = 154;
pub const SCLK_SDMMC_SAMPLE: u32 = 155;
pub const SCLK_SDIO_DRV: u32 = 156;
pub const SCLK_SDIO_SAMPLE: u32 = 157;
pub const SCLK_VDU_CORE: u32 = 158;
pub const SCLK_VDU_CA: u32 = 159;
pub const SCLK_PCIE_PM: u32 = 160;
pub const SCLK_SPDIF_REC_DPTX: u32 = 161;
pub const SCLK_DPHY_PLL: u32 = 162;
pub const SCLK_DPHY_TX0_CFG: u32 = 163;
pub const SCLK_DPHY_TX1RX1_CFG: u32 = 164;
pub const SCLK_DPHY_RX0_CFG: u32 = 165;
pub const SCLK_RMII_SRC: u32 = 166;
pub const SCLK_PCIEPHY_REF100M: u32 = 167;
pub const SCLK_DDRC: u32 = 168;
pub const SCLK_TESTCLKOUT1: u32 = 169;
pub const SCLK_TESTCLKOUT2: u32 = 170;

pub const DCLK_VOP0: u32 = 180;
pub const DCLK_VOP1: u32 = 181;
pub const DCLK_VOP0_DIV: u32 = 182;
pub const DCLK_VOP1_DIV: u32 = 183;
pub const DCLK_M0_PERILP: u32 = 184;
pub const DCLK_VOP0_FRAC: u32 = 185;
pub const DCLK_VOP1_FRAC: u32 = 186;

pub const FCLK_CM0S: u32 = 190;

/* aclk gates */
pub const ACLK_PERIHP: u32 = 192;
pub const ACLK_PERIHP_NOC: u32 = 193;
pub const ACLK_PERILP0: u32 = 194;
pub const ACLK_PERILP0_NOC: u32 = 195;
pub const ACLK_PERF_PCIE: u32 = 196;
pub const ACLK_PCIE: u32 = 197;
pub const ACLK_INTMEM: u32 = 198;
pub const ACLK_TZMA: u32 = 199;
pub const ACLK_DCF: u32 = 200;
pub const ACLK_CCI: u32 = 201;
pub const ACLK_CCI_NOC0: u32 = 202;
pub const ACLK_CCI_NOC1: u32 = 203;
pub const ACLK_CCI_GRF: u32 = 204;
pub const ACLK_CENTER: u32 = 205;
pub const ACLK_CENTER_MAIN_NOC: u32 = 206;
pub const ACLK_CENTER_PERI_NOC: u32 = 207;
pub const ACLK_GPU: u32 = 208;
pub const ACLK_PERF_GPU: u32 = 209;
pub const ACLK_GPU_GRF: u32 = 210;
pub const ACLK_DMAC0_PERILP: u32 = 211;
pub const ACLK_DMAC1_PERILP: u32 = 212;
pub const ACLK_GMAC: u32 = 213;
pub const ACLK_GMAC_NOC: u32 = 214;
pub const ACLK_PERF_GMAC: u32 = 215;
pub const ACLK_VOP0_NOC: u32 = 216;
pub const ACLK_VOP0: u32 = 217;
pub const ACLK_VOP1_NOC: u32 = 218;
pub const ACLK_VOP1: u32 = 219;
pub const ACLK_RGA: u32 = 220;
pub const ACLK_RGA_NOC: u32 = 221;
pub const ACLK_HDCP: u32 = 222;
pub const ACLK_HDCP_NOC: u32 = 223;
pub const ACLK_HDCP22: u32 = 224;
pub const ACLK_IEP: u32 = 225;
pub const ACLK_IEP_NOC: u32 = 226;
pub const ACLK_VIO: u32 = 227;
pub const ACLK_VIO_NOC: u32 = 228;
pub const ACLK_ISP0: u32 = 229;
pub const ACLK_ISP1: u32 = 230;
pub const ACLK_ISP0_NOC: u32 = 231;
pub const ACLK_ISP1_NOC: u32 = 232;
pub const ACLK_ISP0_WRAPPER: u32 = 233;
pub const ACLK_ISP1_WRAPPER: u32 = 234;
pub const ACLK_VCODEC: u32 = 235;
pub const ACLK_VCODEC_NOC: u32 = 236;
pub const ACLK_VDU: u32 = 237;
pub const ACLK_VDU_NOC: u32 = 238;
pub const ACLK_PERI: u32 = 239;
pub const ACLK_EMMC: u32 = 240;
pub const ACLK_EMMC_CORE: u32 = 241;
pub const ACLK_EMMC_NOC: u32 = 242;
pub const ACLK_EMMC_GRF: u32 = 243;
pub const ACLK_USB3: u32 = 244;
pub const ACLK_USB3_NOC: u32 = 245;
pub const ACLK_USB3OTG0: u32 = 246;
pub const ACLK_USB3OTG1: u32 = 247;
pub const ACLK_USB3_RKSOC_AXI_PERF: u32 = 248;
pub const ACLK_USB3_GRF: u32 = 249;
pub const ACLK_GIC: u32 = 250;
pub const ACLK_GIC_NOC: u32 = 251;
pub const ACLK_GIC_ADB400_CORE_L_2_GIC: u32 = 252;
pub const ACLK_GIC_ADB400_CORE_B_2_GIC: u32 = 253;
pub const ACLK_GIC_ADB400_GIC_2_CORE_L: u32 = 254;
pub const ACLK_GIC_ADB400_GIC_2_CORE_B: u32 = 255;
pub const ACLK_CORE_ADB400_CORE_L_2_CCI500: u32 = 256;
pub const ACLK_CORE_ADB400_CORE_B_2_CCI500: u32 = 257;
pub const ACLK_ADB400M_PD_CORE_L: u32 = 258;
pub const ACLK_ADB400M_PD_CORE_B: u32 = 259;
pub const ACLK_PERF_CORE_L: u32 = 260;
pub const ACLK_PERF_CORE_B: u32 = 261;
pub const ACLK_GIC_PRE: u32 = 262;
pub const ACLK_VOP0_PRE: u32 = 263;
pub const ACLK_VOP1_PRE: u32 = 264;

/* pclk gates */
pub const PCLK_PERIHP: u32 = 320;
pub const PCLK_PERIHP_NOC: u32 = 321;
pub const PCLK_PERILP0: u32 = 322;
pub const PCLK_PERILP1: u32 = 323;
pub const PCLK_PERILP1_NOC: u32 = 324;
pub const PCLK_PERILP_SGRF: u32 = 325;
pub const PCLK_PERIHP_GRF: u32 = 326;
pub const PCLK_PCIE: u32 = 327;
pub const PCLK_SGRF: u32 = 328;
pub const PCLK_INTR_ARB: u32 = 329;
pub const PCLK_CENTER_MAIN_NOC: u32 = 330;
pub const PCLK_CIC: u32 = 331;
pub const PCLK_COREDBG_B: u32 = 332;
pub const PCLK_COREDBG_L: u32 = 333;
pub const PCLK_DBG_CXCS_PD_CORE_B: u32 = 334;
pub const PCLK_DCF: u32 = 335;
pub const PCLK_GPIO2: u32 = 336;
pub const PCLK_GPIO3: u32 = 337;
pub const PCLK_GPIO4: u32 = 338;
pub const PCLK_GRF: u32 = 339;
pub const PCLK_HSICPHY: u32 = 340;
pub const PCLK_I2C1: u32 = 341;
pub const PCLK_I2C2: u32 = 342;
pub const PCLK_I2C3: u32 = 343;
pub const PCLK_I2C5: u32 = 344;
pub const PCLK_I2C6: u32 = 345;
pub const PCLK_I2C7: u32 = 346;
pub const PCLK_SPI0: u32 = 347;
pub const PCLK_SPI1: u32 = 348;
pub const PCLK_SPI2: u32 = 349;
pub const PCLK_SPI4: u32 = 350;
pub const PCLK_SPI5: u32 = 351;
pub const PCLK_UART0: u32 = 352;
pub const PCLK_UART1: u32 = 353;
pub const PCLK_UART2: u32 = 354;
pub const PCLK_UART3: u32 = 355;
pub const PCLK_TSADC: u32 = 356;
pub const PCLK_SARADC: u32 = 357;
pub const PCLK_GMAC: u32 = 358;
pub const PCLK_GMAC_NOC: u32 = 359;
pub const PCLK_TIMER0: u32 = 360;
pub const PCLK_TIMER1: u32 = 361;
pub const PCLK_EDP: u32 = 362;
pub const PCLK_EDP_NOC: u32 = 363;
pub const PCLK_EDP_CTRL: u32 = 364;
pub const PCLK_VIO: u32 = 365;
pub const PCLK_VIO_NOC: u32 = 366;
pub const PCLK_VIO_GRF: u32 = 367;
pub const PCLK_MIPI_DSI0: u32 = 368;
pub const PCLK_MIPI_DSI1: u32 = 369;
pub const PCLK_HDCP: u32 = 370;
pub const PCLK_HDCP_NOC: u32 = 371;
pub const PCLK_HDMI_CTRL: u32 = 372;
pub const PCLK_DP_CTRL: u32 = 373;
pub const PCLK_HDCP22: u32 = 374;
pub const PCLK_GASKET: u32 = 375;
pub const PCLK_DDR: u32 = 376;
pub const PCLK_DDR_MON: u32 = 377;
pub const PCLK_DDR_SGRF: u32 = 378;
pub const PCLK_ISP1_WRAPPER: u32 = 379;
pub const PCLK_WDT: u32 = 380;
pub const PCLK_EFUSE1024NS: u32 = 381;
pub const PCLK_EFUSE1024S: u32 = 382;
pub const PCLK_PMU_INTR_ARB: u32 = 383;
pub const PCLK_MAILBOX0: u32 = 384;
pub const PCLK_USBPHY_MUX_G: u32 = 385;
pub const PCLK_UPHY0_TCPHY_G: u32 = 386;
pub const PCLK_UPHY0_TCPD_G: u32 = 387;
pub const PCLK_UPHY1_TCPHY_G: u32 = 388;
pub const PCLK_UPHY1_TCPD_G: u32 = 389;
pub const PCLK_ALIVE: u32 = 390;

/* hclk gates */
pub const HCLK_PERIHP: u32 = 448;
pub const HCLK_PERILP0: u32 = 449;
pub const HCLK_PERILP1: u32 = 450;
pub const HCLK_PERILP0_NOC: u32 = 451;
pub const HCLK_PERILP1_NOC: u32 = 452;
pub const HCLK_M0_PERILP: u32 = 453;
pub const HCLK_M0_PERILP_NOC: u32 = 454;
pub const HCLK_AHB1TOM: u32 = 455;
pub const HCLK_HOST0: u32 = 456;
pub const HCLK_HOST0_ARB: u32 = 457;
pub const HCLK_HOST1: u32 = 458;
pub const HCLK_HOST1_ARB: u32 = 459;
pub const HCLK_HSIC: u32 = 460;
pub const HCLK_SD: u32 = 461;
pub const HCLK_SDMMC: u32 = 462;
pub const HCLK_SDMMC_NOC: u32 = 463;
pub const HCLK_M_CRYPTO0: u32 = 464;
pub const HCLK_M_CRYPTO1: u32 = 465;
pub const HCLK_S_CRYPTO0: u32 = 466;
pub const HCLK_S_CRYPTO1: u32 = 467;
pub const HCLK_I2S0_8CH: u32 = 468;
pub const HCLK_I2S1_8CH: u32 = 469;
pub const HCLK_I2S2_8CH: u32 = 470;
pub const HCLK_SPDIF: u32 = 471;
pub const HCLK_VOP0_NOC: u32 = 472;
pub const HCLK_VOP0: u32 = 473;
pub const HCLK_VOP1_NOC: u32 = 474;
pub const HCLK_VOP1: u32 = 475;
pub const HCLK_ROM: u32 = 476;
pub const HCLK_IEP: u32 = 477;
pub const HCLK_IEP_NOC: u32 = 478;
pub const HCLK_ISP0: u32 = 479;
pub const HCLK_ISP1: u32 = 480;
pub const HCLK_ISP0_NOC: u32 = 481;
pub const HCLK_ISP1_NOC: u32 = 482;
pub const HCLK_ISP0_WRAPPER: u32 = 483;
pub const HCLK_ISP1_WRAPPER: u32 = 484;
pub const HCLK_RGA: u32 = 485;
pub const HCLK_RGA_NOC: u32 = 486;
pub const HCLK_HDCP: u32 = 487;
pub const HCLK_HDCP_NOC: u32 = 488;
pub const HCLK_HDCP22: u32 = 489;
pub const HCLK_VCODEC: u32 = 490;
pub const HCLK_VCODEC_NOC: u32 = 491;
pub const HCLK_VDU: u32 = 492;
pub const HCLK_VDU_NOC: u32 = 493;
pub const HCLK_SDIO: u32 = 494;
pub const HCLK_SDIO_NOC: u32 = 495;
pub const HCLK_SDIOAUDIO_NOC: u32 = 496;

/* pmu-clocks indices */

pub const PLL_PPLL: u32 = 1;

pub const SCLK_32K_SUSPEND_PMU: u32 = 2;
pub const SCLK_SPI3_PMU: u32 = 3;
pub const SCLK_TIMER12_PMU: u32 = 4;
pub const SCLK_TIMER13_PMU: u32 = 5;
pub const SCLK_UART4_PMU: u32 = 6;
pub const SCLK_PVTM_PMU: u32 = 7;
pub const SCLK_WIFI_PMU: u32 = 8;
pub const SCLK_I2C0_PMU: u32 = 9;
pub const SCLK_I2C4_PMU: u32 = 10;
pub const SCLK_I2C8_PMU: u32 = 11;

pub const PCLK_SRC_PMU: u32 = 19;
pub const PCLK_PMU: u32 = 20;
pub const PCLK_PMUGRF_PMU: u32 = 21;
pub const PCLK_INTMEM1_PMU: u32 = 22;
pub const PCLK_GPIO0_PMU: u32 = 23;
pub const PCLK_GPIO1_PMU: u32 = 24;
pub const PCLK_SGRF_PMU: u32 = 25;
pub const PCLK_NOC_PMU: u32 = 26;
pub const PCLK_I2C0_PMU: u32 = 27;
pub const PCLK_I2C4_PMU: u32 = 28;
pub const PCLK_I2C8_PMU: u32 = 29;
pub const PCLK_RKPWM_PMU: u32 = 30;
pub const PCLK_SPI3_PMU: u32 = 31;
pub const PCLK_TIMER_PMU: u32 = 32;
pub const PCLK_MAILBOX_PMU: u32 = 33;
pub const PCLK_UART4_PMU: u32 = 34;
pub const PCLK_WDT_M0_PMU: u32 = 35;

pub const FCLK_CM0S_SRC_PMU: u32 = 44;
pub const FCLK_CM0S_PMU: u32 = 45;
pub const SCLK_CM0S_PMU: u32 = 46;
pub const HCLK_CM0S_PMU: u32 = 47;
pub const DCLK_CM0S_PMU: u32 = 48;
pub const PCLK_INTR_ARB_PMU: u32 = 49;
pub const HCLK_NOC_PMU: u32 = 50;

/* soft-reset indices */

/* cru_softrst_con0 */
pub const SRST_CORE_L0: u32 = 0;
pub const SRST_CORE_B0: u32 = 1;
pub const SRST_CORE_PO_L0: u32 = 2;
pub const SRST_CORE_PO_B0: u32 = 3;
pub const SRST_L2_L: u32 = 4;
pub const SRST_L2_B: u32 = 5;
pub const SRST_ADB_L: u32 = 6;
pub const SRST_ADB_B: u32 = 7;
pub const SRST_A_CCI: u32 = 8;
pub const SRST_A_CCIM0_NOC: u32 = 9;
pub const SRST_A_CCIM1_NOC: u32 = 10;
pub const SRST_DBG_NOC: u32 = 11;

/* cru_softrst_con1 */
pub const SRST_CORE_L0_T: u32 = 16;
pub const SRST_CORE_L1: u32 = 17;
pub const SRST_CORE_L2: u32 = 18;
pub const SRST_CORE_L3: u32 = 19;
pub const SRST_CORE_PO_L0_T: u32 = 20;
pub const SRST_CORE_PO_L1: u32 = 21;
pub const SRST_CORE_PO_L2: u32 = 22;
pub const SRST_CORE_PO_L3: u32 = 23;
pub const SRST_A_ADB400_GIC2COREL: u32 = 24;
pub const SRST_A_ADB400_COREL2GIC: u32 = 25;
pub const SRST_P_DBG_L: u32 = 26;
pub const SRST_L2_L_T: u32 = 28;
pub const SRST_ADB_L_T: u32 = 29;
pub const SRST_A_RKPERF_L: u32 = 30;
pub const SRST_PVTM_CORE_L: u32 = 31;

/* cru_softrst_con2 */
pub const SRST_CORE_B0_T: u32 = 32;
pub const SRST_CORE_B1: u32 = 33;
pub const SRST_CORE_PO_B0_T: u32 = 36;
pub const SRST_CORE_PO_B1: u32 = 37;
pub const SRST_A_ADB400_GIC2COREB: u32 = 40;
pub const SRST_A_ADB400_COREB2GIC: u32 = 41;
pub const SRST_P_DBG_B: u32 = 42;
pub const SRST_L2_B_T: u32 = 43;
pub const SRST_ADB_B_T: u32 = 45;
pub const SRST_A_RKPERF_B: u32 = 46;
pub const SRST_PVTM_CORE_B: u32 = 47;

/* cru_softrst_con3 */
pub const SRST_A_CCI_T: u32 = 50;
pub const SRST_A_CCIM0_NOC_T: u32 = 51;
pub const SRST_A_CCIM1_NOC_T: u32 = 52;
pub const SRST_A_ADB400M_PD_CORE_B_T: u32 = 53;
pub const SRST_A_ADB400M_PD_CORE_L_T: u32 = 54;
pub const SRST_DBG_NOC_T: u32 = 55;
pub const SRST_DBG_CXCS: u32 = 56;
pub const SRST_CCI_TRACE: u32 = 57;
pub const SRST_P_CCI_GRF: u32 = 58;

/* cru_softrst_con4 */
pub const SRST_A_CENTER_MAIN_NOC: u32 = 64;
pub const SRST_A_CENTER_PERI_NOC: u32 = 65;
pub const SRST_P_CENTER_MAIN: u32 = 66;
pub const SRST_P_DDRMON: u32 = 67;
pub const SRST_P_CIC: u32 = 68;
pub const SRST_P_CENTER_SGRF: u32 = 69;
pub const SRST_DDR0_MSCH: u32 = 70;
pub const SRST_DDRCFG0_MSCH: u32 = 71;
pub const SRST_DDR0: u32 = 72;
pub const SRST_DDRPHY0: u32 = 73;
pub const SRST_DDR1_MSCH: u32 = 74;
pub const SRST_DDRCFG1_MSCH: u32 = 75;
pub const SRST_DDR1: u32 = 76;
pub const SRST_DDRPHY1: u32 = 77;
pub const SRST_DDR_CIC: u32 = 78;
pub const SRST_PVTM_DDR: u32 = 79;

/* cru_softrst_con5 */
pub const SRST_A_VCODEC_NOC: u32 = 80;
pub const SRST_A_VCODEC: u32 = 81;
pub const SRST_H_VCODEC_NOC: u32 = 82;
pub const SRST_H_VCODEC: u32 = 83;
pub const SRST_A_VDU_NOC: u32 = 88;
pub const SRST_A_VDU: u32 = 89;
pub const SRST_H_VDU_NOC: u32 = 90;
pub const SRST_H_VDU: u32 = 91;
pub const SRST_VDU_CORE: u32 = 92;
pub const SRST_VDU_CA: u32 = 93;

/* cru_softrst_con6 */
pub const SRST_A_IEP_NOC: u32 = 96;
pub const SRST_A_VOP_IEP: u32 = 97;
pub const SRST_A_IEP: u32 = 98;
pub const SRST_H_IEP_NOC: u32 = 99;
pub const SRST_H_IEP: u32 = 100;
pub const SRST_A_RGA_NOC: u32 = 102;
pub const SRST_A_RGA: u32 = 103;
pub const SRST_H_RGA_NOC: u32 = 104;
pub const SRST_H_RGA: u32 = 105;
pub const SRST_RGA_CORE: u32 = 106;
pub const SRST_EMMC_NOC: u32 = 108;
pub const SRST_EMMC: u32 = 109;
pub const SRST_EMMC_GRF: u32 = 110;

/* cru_softrst_con7 */
pub const SRST_A_PERIHP_NOC: u32 = 112;
pub const SRST_P_PERIHP_GRF: u32 = 113;
pub const SRST_H_PERIHP_NOC: u32 = 114;
pub const SRST_USBHOST0: u32 = 115;
pub const SRST_HOSTC0_AUX: u32 = 116;
pub const SRST_HOST0_ARB: u32 = 117;
pub const SRST_USBHOST1: u32 = 118;
pub const SRST_HOSTC1_AUX: u32 = 119;
pub const SRST_HOST1_ARB: u32 = 120;
pub const SRST_SDIO0: u32 = 121;
pub const SRST_SDMMC: u32 = 122;
pub const SRST_HSIC: u32 = 123;
pub const SRST_HSIC_AUX: u32 = 124;
pub const SRST_AHB1TOM: u32 = 125;
pub const SRST_P_PERIHP_NOC: u32 = 126;
pub const SRST_HSICPHY: u32 = 127;

/* cru_softrst_con8 */
pub const SRST_A_PCIE: u32 = 128;
pub const SRST_P_PCIE: u32 = 129;
pub const SRST_PCIE_CORE: u32 = 130;
pub const SRST_PCIE_MGMT: u32 = 131;
pub const SRST_PCIE_MGMT_STICKY: u32 = 132;
pub const SRST_PCIE_PIPE: u32 = 133;
pub const SRST_PCIE_PM: u32 = 134;
pub const SRST_PCIEPHY: u32 = 135;
pub const SRST_A_GMAC_NOC: u32 = 136;
pub const SRST_A_GMAC: u32 = 137;
pub const SRST_P_GMAC_NOC: u32 = 138;
pub const SRST_P_GMAC_GRF: u32 = 140;
pub const SRST_HSICPHY_POR: u32 = 142;
pub const SRST_HSICPHY_UTMI: u32 = 143;

/* cru_softrst_con9 */
pub const SRST_USB2PHY0_POR: u32 = 144;
pub const SRST_USB2PHY0_UTMI_PORT0: u32 = 145;
pub const SRST_USB2PHY0_UTMI_PORT1: u32 = 146;
pub const SRST_USB2PHY0_EHCIPHY: u32 = 147;
pub const SRST_UPHY0_PIPE_L00: u32 = 148;
pub const SRST_UPHY0: u32 = 149;
pub const SRST_UPHY0_TCPDPWRUP: u32 = 150;
pub const SRST_USB2PHY1_POR: u32 = 152;
pub const SRST_USB2PHY1_UTMI_PORT0: u32 = 153;
pub const SRST_USB2PHY1_UTMI_PORT1: u32 = 154;
pub const SRST_USB2PHY1_EHCIPHY: u32 = 155;
pub const SRST_UPHY1_PIPE_L00: u32 = 156;
pub const SRST_UPHY1: u32 = 157;
pub const SRST_UPHY1_TCPDPWRUP: u32 = 158;

/* cru_softrst_con10 */
pub const SRST_A_PERILP0_NOC: u32 = 160;
pub const SRST_A_DCF: u32 = 161;
pub const SRST_GIC500: u32 = 162;
pub const SRST_DMAC0_PERILP0: u32 = 163;
pub const SRST_DMAC1_PERILP0: u32 = 164;
pub const SRST_TZMA: u32 = 165;
pub const SRST_INTMEM: u32 = 166;
pub const SRST_ADB400_MST0: u32 = 167;
pub const SRST_ADB400_MST1: u32 = 168;
pub const SRST_ADB400_SLV0: u32 = 169;
pub const SRST_ADB400_SLV1: u32 = 170;
pub const SRST_H_PERILP0: u32 = 171;
pub const SRST_H_PERILP0_NOC: u32 = 172;
pub const SRST_ROM: u32 = 173;
pub const SRST_CRYPTO0_S: u32 = 174;
pub const SRST_CRYPTO0_M: u32 = 175;

/* cru_softrst_con11 */
pub const SRST_P_DCF: u32 = 176;
pub const SRST_CM0S_NOC: u32 = 177;
pub const SRST_CM0S: u32 = 178;
pub const SRST_CM0S_DBG: u32 = 179;
pub const SRST_CM0S_PO: u32 = 180;
pub const SRST_CRYPTO0: u32 = 181;
pub const SRST_P_PERILP1_SGRF: u32 = 182;
pub const SRST_P_PERILP1_GRF: u32 = 183;
pub const SRST_CRYPTO1_S: u32 = 184;
pub const SRST_CRYPTO1_M: u32 = 185;
pub const SRST_CRYPTO1: u32 = 186;
pub const SRST_GIC_NOC: u32 = 188;
pub const SRST_SD_NOC: u32 = 189;
pub const SRST_SDIOAUDIO_BRG: u32 = 190;

/* cru_softrst_con12 */
pub const SRST_H_PERILP1: u32 = 192;
pub const SRST_H_PERILP1_NOC: u32 = 193;
pub const SRST_H_I2S0_8CH: u32 = 194;
pub const SRST_H_I2S1_8CH: u32 = 195;
pub const SRST_H_I2S2_8CH: u32 = 196;
pub const SRST_H_SPDIF_8CH: u32 = 197;
pub const SRST_P_PERILP1_NOC: u32 = 198;
pub const SRST_P_EFUSE_1024: u32 = 199;
pub const SRST_P_EFUSE_1024S: u32 = 200;
pub const SRST_P_I2C0: u32 = 201;
pub const SRST_P_I2C1: u32 = 202;
pub const SRST_P_I2C2: u32 = 203;
pub const SRST_P_I2C3: u32 = 204;
pub const SRST_P_I2C4: u32 = 205;
pub const SRST_P_I2C5: u32 = 206;
pub const SRST_P_MAILBOX0: u32 = 207;

/* cru_softrst_con13 */
pub const SRST_P_UART0: u32 = 208;
pub const SRST_P_UART1: u32 = 209;
pub const SRST_P_UART2: u32 = 210;
pub const SRST_P_UART3: u32 = 211;
pub const SRST_P_SARADC: u32 = 212;
pub const SRST_P_TSADC: u32 = 213;
pub const SRST_P_SPI0: u32 = 214;
pub const SRST_P_SPI1: u32 = 215;
pub const SRST_P_SPI2: u32 = 216;
pub const SRST_P_SPI3: u32 = 217;
pub const SRST_P_SPI4: u32 = 218;
pub const SRST_SPI0: u32 = 219;
pub const SRST_SPI1: u32 = 220;
pub const SRST_SPI2: u32 = 221;
pub const SRST_SPI3: u32 = 222;
pub const SRST_SPI4: u32 = 223;

/* cru_softrst_con14 */
pub const SRST_I2S0_8CH: u32 = 224;
pub const SRST_I2S1_8CH: u32 = 225;
pub const SRST_I2S2_8CH: u32 = 226;
pub const SRST_SPDIF_8CH: u32 = 227;
pub const SRST_UART0: u32 = 228;
pub const SRST_UART1: u32 = 229;
pub const SRST_UART2: u32 = 230;
pub const SRST_UART3: u32 = 231;
pub const SRST_TSADC: u32 = 232;
pub const SRST_I2C0: u32 = 233;
pub const SRST_I2C1: u32 = 234;
pub const SRST_I2C2: u32 = 235;
pub const SRST_I2C3: u32 = 236;
pub const SRST_I2C4: u32 = 237;
pub const SRST_I2C5: u32 = 238;
pub const SRST_SDIOAUDIO_NOC: u32 = 239;

/* cru_softrst_con15 */
pub const SRST_A_VIO_NOC: u32 = 240;
pub const SRST_A_HDCP_NOC: u32 = 241;
pub const SRST_A_HDCP: u32 = 242;
pub const SRST_H_HDCP_NOC: u32 = 243;
pub const SRST_H_HDCP: u32 = 244;
pub const SRST_P_HDCP_NOC: u32 = 245;
pub const SRST_P_HDCP: u32 = 246;
pub const SRST_P_HDMI_CTRL: u32 = 247;
pub const SRST_P_DP_CTRL: u32 = 248;
pub const SRST_S_DP_CTRL: u32 = 249;
pub const SRST_C_DP_CTRL: u32 = 250;
pub const SRST_P_MIPI_DSI0: u32 = 251;
pub const SRST_P_MIPI_DSI1: u32 = 252;
pub const SRST_DP_CORE: u32 = 253;
pub const SRST_DP_I2S: u32 = 254;

/* cru_softrst_con16 */
pub const SRST_GASKET: u32 = 256;
pub const SRST_VIO_GRF: u32 = 258;
pub const SRST_DPTX_SPDIF_REC: u32 = 259;
pub const SRST_HDMI_CTRL: u32 = 260;
pub const SRST_HDCP_CTRL: u32 = 261;
pub const SRST_A_ISP0_NOC: u32 = 262;
pub const SRST_A_ISP1_NOC: u32 = 263;
pub const SRST_H_ISP0_NOC: u32 = 266;
pub const SRST_H_ISP1_NOC: u32 = 267;
pub const SRST_H_ISP0: u32 = 268;
pub const SRST_H_ISP1: u32 = 269;
pub const SRST_ISP0: u32 = 270;
pub const SRST_ISP1: u32 = 271;

/* cru_softrst_con17 */
pub const SRST_A_VOP0_NOC: u32 = 272;
pub const SRST_A_VOP1_NOC: u32 = 273;
pub const SRST_A_VOP0: u32 = 274;
pub const SRST_A_VOP1: u32 = 275;
pub const SRST_H_VOP0_NOC: u32 = 276;
pub const SRST_H_VOP1_NOC: u32 = 277;
pub const SRST_H_VOP0: u32 = 278;
pub const SRST_H_VOP1: u32 = 279;
pub const SRST_D_VOP0: u32 = 280;
pub const SRST_D_VOP1: u32 = 281;
pub const SRST_VOP0_PWM: u32 = 282;
pub const SRST_VOP1_PWM: u32 = 283;
pub const SRST_P_EDP_NOC: u32 = 284;
pub const SRST_P_EDP_CTRL: u32 = 285;

/* cru_softrst_con18 */
pub const SRST_A_GPU: u32 = 288;
pub const SRST_A_GPU_NOC: u32 = 289;
pub const SRST_A_GPU_GRF: u32 = 290;
pub const SRST_PVTM_GPU: u32 = 291;
pub const SRST_A_USB3_NOC: u32 = 292;
pub const SRST_A_USB3_OTG0: u32 = 293;
pub const SRST_A_USB3_OTG1: u32 = 294;
pub const SRST_A_USB3_GRF: u32 = 295;
pub const SRST_PMU: u32 = 296;

/* cru_softrst_con19 */
pub const SRST_P_TIMER0_5: u32 = 304;
pub const SRST_TIMER0: u32 = 305;
pub const SRST_TIMER1: u32 = 306;
pub const SRST_TIMER2: u32 = 307;
pub const SRST_TIMER3: u32 = 308;
pub const SRST_TIMER4: u32 = 309;
pub const SRST_TIMER5: u32 = 310;
pub const SRST_P_TIMER6_11: u32 = 311;
pub const SRST_TIMER6: u32 = 312;
pub const SRST_TIMER7: u32 = 313;
pub const SRST_TIMER8: u32 = 314;
pub const SRST_TIMER9: u32 = 315;
pub const SRST_TIMER10: u32 = 316;
pub const SRST_TIMER11: u32 = 317;
pub const SRST_P_INTR_ARB_PMU: u32 = 318;
pub const SRST_P_ALIVE_SGRF: u32 = 319;

/* cru_softrst_con20 */
pub const SRST_P_GPIO2: u32 = 320;
pub const SRST_P_GPIO3: u32 = 321;
pub const SRST_P_GPIO4: u32 = 322;
pub const SRST_P_GRF: u32 = 323;
pub const SRST_P_ALIVE_NOC: u32 = 324;
pub const SRST_P_WDT0: u32 = 325;
pub const SRST_P_WDT1: u32 = 326;
pub const SRST_P_INTR_ARB: u32 = 327;
pub const SRST_P_UPHY0_DPTX: u32 = 328;
pub const SRST_P_UPHY0_APB: u32 = 330;
pub const SRST_P_UPHY0_TCPHY: u32 = 332;
pub const SRST_P_UPHY1_TCPHY: u32 = 333;
pub const SRST_P_UPHY0_TCPDCTRL: u32 = 334;
pub const SRST_P_UPHY1_TCPDCTRL: u32 = 335;

/* pmu soft-reset indices */

/* pmu_cru_softrst_con0 */
pub const SRST_P_NOC: u32 = 0;
pub const SRST_P_INTMEM: u32 = 1;
pub const SRST_H_CM0S: u32 = 2;
pub const SRST_H_CM0S_NOC: u32 = 3;
pub const SRST_DBG_CM0S: u32 = 4;
pub const SRST_PO_CM0S: u32 = 5;
pub const SRST_P_SPI6: u32 = 6;
pub const SRST_SPI6: u32 = 7;
pub const SRST_P_TIMER_0_1: u32 = 8;
pub const SRST_P_TIMER_0: u32 = 9;
pub const SRST_P_TIMER_1: u32 = 10;
pub const SRST_P_UART4: u32 = 11;
pub const SRST_UART4: u32 = 12;
pub const SRST_P_WDT: u32 = 13;

/* pmu_cru_softrst_con1 */
pub const SRST_P_I2C6: u32 = 16;
pub const SRST_P_I2C7: u32 = 17;
pub const SRST_P_I2C8: u32 = 18;
pub const SRST_P_MAILBOX: u32 = 19;
pub const SRST_P_RKPWM: u32 = 20;
pub const SRST_P_PMUGRF: u32 = 21;
pub const SRST_P_SGRF: u32 = 22;
pub const SRST_P_GPIO0: u32 = 23;
pub const SRST_P_GPIO1: u32 = 24;
pub const SRST_P_CRU: u32 = 25;
pub const SRST_P_INTR: u32 = 26;
pub const SRST_PVTM: u32 = 27;
pub const SRST_I2C6: u32 = 28;
pub const SRST_I2C7: u32 = 29;
pub const SRST_I2C8: u32 = 30;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
