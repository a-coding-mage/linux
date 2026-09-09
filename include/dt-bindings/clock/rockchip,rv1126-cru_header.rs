/* SPDX-License-Identifier: (GPL-2.0+ OR MIT) */
/*
//  * Copyright (c) 2019 Rockchip Electronics Co. Ltd.
//  * Author: Finley Xiao <finley.xiao@rock-chips.com>
//  */

// #ifndef _DT_BINDINGS_CLK_ROCKCHIP_RV1126_H
pub const _DT_BINDINGS_CLK_ROCKCHIP_RV1126_H: u32 = _DT_BINDINGS_CLK_ROCKCHIP_RV1126_H;

/* pmucru-clocks indices */

/* pll clocks */
pub const PLL_GPLL: u32 = 1;

/* sclk (special clocks) */
pub const CLK_OSC0_DIV32K: u32 = 2;
pub const CLK_RTC32K: u32 = 3;
pub const CLK_WIFI_DIV: u32 = 4;
pub const CLK_WIFI_OSC0: u32 = 5;
pub const CLK_WIFI: u32 = 6;
pub const CLK_PMU: u32 = 7;
pub const SCLK_UART1_DIV: u32 = 8;
pub const SCLK_UART1_FRACDIV: u32 = 9;
pub const SCLK_UART1_MUX: u32 = 10;
pub const SCLK_UART1: u32 = 11;
pub const CLK_I2C0: u32 = 12;
pub const CLK_I2C2: u32 = 13;
pub const CLK_CAPTURE_PWM0: u32 = 14;
pub const CLK_PWM0: u32 = 15;
pub const CLK_CAPTURE_PWM1: u32 = 16;
pub const CLK_PWM1: u32 = 17;
pub const CLK_SPI0: u32 = 18;
pub const DBCLK_GPIO0: u32 = 19;
pub const CLK_PMUPVTM: u32 = 20;
pub const CLK_CORE_PMUPVTM: u32 = 21;
pub const CLK_REF12M: u32 = 22;
pub const CLK_USBPHY_OTG_REF: u32 = 23;
pub const CLK_USBPHY_HOST_REF: u32 = 24;
pub const CLK_REF24M: u32 = 25;
pub const CLK_MIPIDSIPHY_REF: u32 = 26;

/* pclk */
pub const PCLK_PDPMU: u32 = 30;
pub const PCLK_PMU: u32 = 31;
pub const PCLK_UART1: u32 = 32;
pub const PCLK_I2C0: u32 = 33;
pub const PCLK_I2C2: u32 = 34;
pub const PCLK_PWM0: u32 = 35;
pub const PCLK_PWM1: u32 = 36;
pub const PCLK_SPI0: u32 = 37;
pub const PCLK_GPIO0: u32 = 38;
pub const PCLK_PMUSGRF: u32 = 39;
pub const PCLK_PMUGRF: u32 = 40;
pub const PCLK_PMUCRU: u32 = 41;
pub const PCLK_CHIPVEROTP: u32 = 42;
pub const PCLK_PDPMU_NIU: u32 = 43;
pub const PCLK_PMUPVTM: u32 = 44;
pub const PCLK_SCRKEYGEN: u32 = 45;

pub const CLKPMU_NR_CLKS: u32 = PCLK_SCRKEYGEN + 1;

/* cru-clocks indices */

/* pll clocks */
pub const PLL_APLL: u32 = 1;
pub const PLL_DPLL: u32 = 2;
pub const PLL_CPLL: u32 = 3;
pub const PLL_HPLL: u32 = 4;

/* sclk (special clocks) */
pub const ARMCLK: u32 = 5;
pub const USB480M: u32 = 6;
pub const CLK_CORE_CPUPVTM: u32 = 7;
pub const CLK_CPUPVTM: u32 = 8;
pub const CLK_SCR1: u32 = 9;
pub const CLK_SCR1_CORE: u32 = 10;
pub const CLK_SCR1_RTC: u32 = 11;
pub const CLK_SCR1_JTAG: u32 = 12;
pub const SCLK_UART0_DIV: u32 = 13;
pub const SCLK_UART0_FRAC: u32 = 14;
pub const SCLK_UART0_MUX: u32 = 15;
pub const SCLK_UART0: u32 = 16;
pub const SCLK_UART2_DIV: u32 = 17;
pub const SCLK_UART2_FRAC: u32 = 18;
pub const SCLK_UART2_MUX: u32 = 19;
pub const SCLK_UART2: u32 = 20;
pub const SCLK_UART3_DIV: u32 = 21;
pub const SCLK_UART3_FRAC: u32 = 22;
pub const SCLK_UART3_MUX: u32 = 23;
pub const SCLK_UART3: u32 = 24;
pub const SCLK_UART4_DIV: u32 = 25;
pub const SCLK_UART4_FRAC: u32 = 26;
pub const SCLK_UART4_MUX: u32 = 27;
pub const SCLK_UART4: u32 = 28;
pub const SCLK_UART5_DIV: u32 = 29;
pub const SCLK_UART5_FRAC: u32 = 30;
pub const SCLK_UART5_MUX: u32 = 31;
pub const SCLK_UART5: u32 = 32;
pub const CLK_I2C1: u32 = 33;
pub const CLK_I2C3: u32 = 34;
pub const CLK_I2C4: u32 = 35;
pub const CLK_I2C5: u32 = 36;
pub const CLK_SPI1: u32 = 37;
pub const CLK_CAPTURE_PWM2: u32 = 38;
pub const CLK_PWM2: u32 = 39;
pub const DBCLK_GPIO1: u32 = 40;
pub const DBCLK_GPIO2: u32 = 41;
pub const DBCLK_GPIO3: u32 = 42;
pub const DBCLK_GPIO4: u32 = 43;
pub const CLK_SARADC: u32 = 44;
pub const CLK_TIMER0: u32 = 45;
pub const CLK_TIMER1: u32 = 46;
pub const CLK_TIMER2: u32 = 47;
pub const CLK_TIMER3: u32 = 48;
pub const CLK_TIMER4: u32 = 49;
pub const CLK_TIMER5: u32 = 50;
pub const CLK_CAN: u32 = 51;
pub const CLK_NPU_TSADC: u32 = 52;
pub const CLK_NPU_TSADCPHY: u32 = 53;
pub const CLK_CPU_TSADC: u32 = 54;
pub const CLK_CPU_TSADCPHY: u32 = 55;
pub const CLK_CRYPTO_CORE: u32 = 56;
pub const CLK_CRYPTO_PKA: u32 = 57;
pub const MCLK_I2S0_TX_DIV: u32 = 58;
pub const MCLK_I2S0_TX_FRACDIV: u32 = 59;
pub const MCLK_I2S0_TX_MUX: u32 = 60;
pub const MCLK_I2S0_TX: u32 = 61;
pub const MCLK_I2S0_RX_DIV: u32 = 62;
pub const MCLK_I2S0_RX_FRACDIV: u32 = 63;
pub const MCLK_I2S0_RX_MUX: u32 = 64;
pub const MCLK_I2S0_RX: u32 = 65;
pub const MCLK_I2S0_TX_OUT2IO: u32 = 66;
pub const MCLK_I2S0_RX_OUT2IO: u32 = 67;
pub const MCLK_I2S1_DIV: u32 = 68;
pub const MCLK_I2S1_FRACDIV: u32 = 69;
pub const MCLK_I2S1_MUX: u32 = 70;
pub const MCLK_I2S1: u32 = 71;
pub const MCLK_I2S1_OUT2IO: u32 = 72;
pub const MCLK_I2S2_DIV: u32 = 73;
pub const MCLK_I2S2_FRACDIV: u32 = 74;
pub const MCLK_I2S2_MUX: u32 = 75;
pub const MCLK_I2S2: u32 = 76;
pub const MCLK_I2S2_OUT2IO: u32 = 77;
pub const MCLK_PDM: u32 = 78;
pub const SCLK_ADUPWM_DIV: u32 = 79;
pub const SCLK_AUDPWM_FRACDIV: u32 = 80;
pub const SCLK_AUDPWM_MUX: u32 = 81;
pub const SCLK_AUDPWM: u32 = 82;
pub const CLK_ACDCDIG_ADC: u32 = 83;
pub const CLK_ACDCDIG_DAC: u32 = 84;
pub const CLK_ACDCDIG_I2C: u32 = 85;
pub const CLK_VENC_CORE: u32 = 86;
pub const CLK_VDEC_CORE: u32 = 87;
pub const CLK_VDEC_CA: u32 = 88;
pub const CLK_VDEC_HEVC_CA: u32 = 89;
pub const CLK_RGA_CORE: u32 = 90;
pub const CLK_IEP_CORE: u32 = 91;
pub const CLK_ISP_DIV: u32 = 92;
pub const CLK_ISP_NP5: u32 = 93;
pub const CLK_ISP_NUX: u32 = 94;
pub const CLK_ISP: u32 = 95;
pub const CLK_CIF_OUT_DIV: u32 = 96;
pub const CLK_CIF_OUT_FRACDIV: u32 = 97;
pub const CLK_CIF_OUT_MUX: u32 = 98;
pub const CLK_CIF_OUT: u32 = 99;
pub const CLK_MIPICSI_OUT_DIV: u32 = 100;
pub const CLK_MIPICSI_OUT_FRACDIV: u32 = 101;
pub const CLK_MIPICSI_OUT_MUX: u32 = 102;
pub const CLK_MIPICSI_OUT: u32 = 103;
pub const CLK_ISPP_DIV: u32 = 104;
pub const CLK_ISPP_NP5: u32 = 105;
pub const CLK_ISPP_NUX: u32 = 106;
pub const CLK_ISPP: u32 = 107;
pub const CLK_SDMMC: u32 = 108;
pub const SCLK_SDMMC_DRV: u32 = 109;
pub const SCLK_SDMMC_SAMPLE: u32 = 110;
pub const CLK_SDIO: u32 = 111;
pub const SCLK_SDIO_DRV: u32 = 112;
pub const SCLK_SDIO_SAMPLE: u32 = 113;
pub const CLK_EMMC: u32 = 114;
pub const SCLK_EMMC_DRV: u32 = 115;
pub const SCLK_EMMC_SAMPLE: u32 = 116;
pub const CLK_NANDC: u32 = 117;
pub const SCLK_SFC: u32 = 118;
pub const CLK_USBHOST_UTMI_OHCI: u32 = 119;
pub const CLK_USBOTG_REF: u32 = 120;
pub const CLK_GMAC_DIV: u32 = 121;
pub const CLK_GMAC_RGMII_M0: u32 = 122;
pub const CLK_GMAC_SRC_M0: u32 = 123;
pub const CLK_GMAC_RGMII_M1: u32 = 124;
pub const CLK_GMAC_SRC_M1: u32 = 125;
pub const CLK_GMAC_SRC: u32 = 126;
pub const CLK_GMAC_REF: u32 = 127;
pub const CLK_GMAC_TX_SRC: u32 = 128;
pub const CLK_GMAC_TX_DIV5: u32 = 129;
pub const CLK_GMAC_TX_DIV50: u32 = 130;
pub const RGMII_MODE_CLK: u32 = 131;
pub const CLK_GMAC_RX_SRC: u32 = 132;
pub const CLK_GMAC_RX_DIV2: u32 = 133;
pub const CLK_GMAC_RX_DIV20: u32 = 134;
pub const RMII_MODE_CLK: u32 = 135;
pub const CLK_GMAC_TX_RX: u32 = 136;
pub const CLK_GMAC_PTPREF: u32 = 137;
pub const CLK_GMAC_ETHERNET_OUT: u32 = 138;
pub const CLK_DDRPHY: u32 = 139;
pub const CLK_DDR_MON: u32 = 140;
pub const TMCLK_DDR_MON: u32 = 141;
pub const CLK_NPU_DIV: u32 = 142;
pub const CLK_NPU_NP5: u32 = 143;
pub const CLK_CORE_NPU: u32 = 144;
pub const CLK_CORE_NPUPVTM: u32 = 145;
pub const CLK_NPUPVTM: u32 = 146;
pub const SCLK_DDRCLK: u32 = 147;
pub const CLK_OTP: u32 = 148;

/* dclk */
pub const DCLK_DECOM: u32 = 150;
pub const DCLK_VOP_DIV: u32 = 151;
pub const DCLK_VOP_FRACDIV: u32 = 152;
pub const DCLK_VOP_MUX: u32 = 153;
pub const DCLK_VOP: u32 = 154;
pub const DCLK_CIF: u32 = 155;
pub const DCLK_CIFLITE: u32 = 156;

/* aclk */
pub const ACLK_PDBUS: u32 = 160;
pub const ACLK_DMAC: u32 = 161;
pub const ACLK_DCF: u32 = 162;
pub const ACLK_SPINLOCK: u32 = 163;
pub const ACLK_DECOM: u32 = 164;
pub const ACLK_PDCRYPTO: u32 = 165;
pub const ACLK_CRYPTO: u32 = 166;
pub const ACLK_PDVEPU: u32 = 167;
pub const ACLK_VENC: u32 = 168;
pub const ACLK_PDVDEC: u32 = 169;
pub const ACLK_PDJPEG: u32 = 170;
pub const ACLK_VDEC: u32 = 171;
pub const ACLK_JPEG: u32 = 172;
pub const ACLK_PDVO: u32 = 173;
pub const ACLK_RGA: u32 = 174;
pub const ACLK_VOP: u32 = 175;
pub const ACLK_IEP: u32 = 176;
pub const ACLK_PDVI_DIV: u32 = 177;
pub const ACLK_PDVI_NP5: u32 = 178;
pub const ACLK_PDVI: u32 = 179;
pub const ACLK_ISP: u32 = 180;
pub const ACLK_CIF: u32 = 181;
pub const ACLK_CIFLITE: u32 = 182;
pub const ACLK_PDISPP_DIV: u32 = 183;
pub const ACLK_PDISPP_NP5: u32 = 184;
pub const ACLK_PDISPP: u32 = 185;
pub const ACLK_ISPP: u32 = 186;
pub const ACLK_PDPHP: u32 = 187;
pub const ACLK_PDUSB: u32 = 188;
pub const ACLK_USBOTG: u32 = 189;
pub const ACLK_PDGMAC: u32 = 190;
pub const ACLK_GMAC: u32 = 191;
pub const ACLK_PDNPU_DIV: u32 = 192;
pub const ACLK_PDNPU_NP5: u32 = 193;
pub const ACLK_PDNPU: u32 = 194;
pub const ACLK_NPU: u32 = 195;

/* hclk */
pub const HCLK_PDCORE_NIU: u32 = 200;
pub const HCLK_PDUSB: u32 = 201;
pub const HCLK_PDCRYPTO: u32 = 202;
pub const HCLK_CRYPTO: u32 = 203;
pub const HCLK_PDAUDIO: u32 = 204;
pub const HCLK_I2S0: u32 = 205;
pub const HCLK_I2S1: u32 = 206;
pub const HCLK_I2S2: u32 = 207;
pub const HCLK_PDM: u32 = 208;
pub const HCLK_AUDPWM: u32 = 209;
pub const HCLK_PDVEPU: u32 = 210;
pub const HCLK_VENC: u32 = 211;
pub const HCLK_PDVDEC: u32 = 212;
pub const HCLK_PDJPEG: u32 = 213;
pub const HCLK_VDEC: u32 = 214;
pub const HCLK_JPEG: u32 = 215;
pub const HCLK_PDVO: u32 = 216;
pub const HCLK_RGA: u32 = 217;
pub const HCLK_VOP: u32 = 218;
pub const HCLK_IEP: u32 = 219;
pub const HCLK_PDVI: u32 = 220;
pub const HCLK_ISP: u32 = 221;
pub const HCLK_CIF: u32 = 222;
pub const HCLK_CIFLITE: u32 = 223;
pub const HCLK_PDISPP: u32 = 224;
pub const HCLK_ISPP: u32 = 225;
pub const HCLK_PDPHP: u32 = 226;
pub const HCLK_PDSDMMC: u32 = 227;
pub const HCLK_SDMMC: u32 = 228;
pub const HCLK_PDSDIO: u32 = 229;
pub const HCLK_SDIO: u32 = 230;
pub const HCLK_PDNVM: u32 = 231;
pub const HCLK_EMMC: u32 = 232;
pub const HCLK_NANDC: u32 = 233;
pub const HCLK_SFC: u32 = 234;
pub const HCLK_SFCXIP: u32 = 235;
pub const HCLK_PDBUS: u32 = 236;
pub const HCLK_USBHOST: u32 = 237;
pub const HCLK_USBHOST_ARB: u32 = 238;
pub const HCLK_PDNPU: u32 = 239;
pub const HCLK_NPU: u32 = 240;

/* pclk */
pub const PCLK_CPUPVTM: u32 = 245;
pub const PCLK_PDBUS: u32 = 246;
pub const PCLK_DCF: u32 = 247;
pub const PCLK_WDT: u32 = 248;
pub const PCLK_MAILBOX: u32 = 249;
pub const PCLK_UART0: u32 = 250;
pub const PCLK_UART2: u32 = 251;
pub const PCLK_UART3: u32 = 252;
pub const PCLK_UART4: u32 = 253;
pub const PCLK_UART5: u32 = 254;
pub const PCLK_I2C1: u32 = 255;
pub const PCLK_I2C3: u32 = 256;
pub const PCLK_I2C4: u32 = 257;
pub const PCLK_I2C5: u32 = 258;
pub const PCLK_SPI1: u32 = 259;
pub const PCLK_PWM2: u32 = 261;
pub const PCLK_GPIO1: u32 = 262;
pub const PCLK_GPIO2: u32 = 263;
pub const PCLK_GPIO3: u32 = 264;
pub const PCLK_GPIO4: u32 = 265;
pub const PCLK_SARADC: u32 = 266;
pub const PCLK_TIMER: u32 = 267;
pub const PCLK_DECOM: u32 = 268;
pub const PCLK_CAN: u32 = 269;
pub const PCLK_NPU_TSADC: u32 = 270;
pub const PCLK_CPU_TSADC: u32 = 271;
pub const PCLK_ACDCDIG: u32 = 272;
pub const PCLK_PDVO: u32 = 273;
pub const PCLK_DSIHOST: u32 = 274;
pub const PCLK_PDVI: u32 = 275;
pub const PCLK_CSIHOST: u32 = 276;
pub const PCLK_PDGMAC: u32 = 277;
pub const PCLK_GMAC: u32 = 278;
pub const PCLK_PDDDR: u32 = 279;
pub const PCLK_DDR_MON: u32 = 280;
pub const PCLK_PDNPU: u32 = 281;
pub const PCLK_NPUPVTM: u32 = 282;
pub const PCLK_PDTOP: u32 = 283;
pub const PCLK_TOPCRU: u32 = 284;
pub const PCLK_TOPGRF: u32 = 285;
pub const PCLK_CPUEMADET: u32 = 286;
pub const PCLK_DDRPHY: u32 = 287;
pub const PCLK_DSIPHY: u32 = 289;
pub const PCLK_CSIPHY0: u32 = 290;
pub const PCLK_CSIPHY1: u32 = 291;
pub const PCLK_USBPHY_HOST: u32 = 292;
pub const PCLK_USBPHY_OTG: u32 = 293;
pub const PCLK_OTP: u32 = 294;

pub const CLK_NR_CLKS: u32 = PCLK_OTP + 1;

/* pmu soft-reset indices */

/* pmu_cru_softrst_con0 */
pub const SRST_PDPMU_NIU_P: u32 = 0;
pub const SRST_PMU_SGRF_P: u32 = 1;
pub const SRST_PMU_SGRF_REMAP_P: u32 = 2;
pub const SRST_I2C0_P: u32 = 3;
pub const SRST_I2C0: u32 = 4;
pub const SRST_I2C2_P: u32 = 7;
pub const SRST_I2C2: u32 = 8;
pub const SRST_UART1_P: u32 = 9;
pub const SRST_UART1: u32 = 10;
pub const SRST_PWM0_P: u32 = 11;
pub const SRST_PWM0: u32 = 12;
pub const SRST_PWM1_P: u32 = 13;
pub const SRST_PWM1: u32 = 14;
pub const SRST_DDR_FAIL_SAFE: u32 = 15;

/* pmu_cru_softrst_con1 */
pub const SRST_GPIO0_P: u32 = 17;
pub const SRST_GPIO0_DB: u32 = 18;
pub const SRST_SPI0_P: u32 = 19;
pub const SRST_SPI0: u32 = 20;
pub const SRST_PMUGRF_P: u32 = 21;
pub const SRST_CHIPVEROTP_P: u32 = 22;
pub const SRST_PMUPVTM: u32 = 24;
pub const SRST_PMUPVTM_P: u32 = 25;
pub const SRST_PMUCRU_P: u32 = 30;

/* soft-reset indices */

/* cru_softrst_con0 */
pub const SRST_CORE0_PO: u32 = 0;
pub const SRST_CORE1_PO: u32 = 1;
pub const SRST_CORE2_PO: u32 = 2;
pub const SRST_CORE3_PO: u32 = 3;
pub const SRST_CORE0: u32 = 4;
pub const SRST_CORE1: u32 = 5;
pub const SRST_CORE2: u32 = 6;
pub const SRST_CORE3: u32 = 7;
pub const SRST_CORE0_DBG: u32 = 8;
pub const SRST_CORE1_DBG: u32 = 9;
pub const SRST_CORE2_DBG: u32 = 10;
pub const SRST_CORE3_DBG: u32 = 11;
pub const SRST_NL2: u32 = 12;
pub const SRST_CORE_NIU_A: u32 = 13;
pub const SRST_DBG_DAPLITE_P: u32 = 14;
pub const SRST_DAPLITE_P: u32 = 15;

/* cru_softrst_con1 */
pub const SRST_PDBUS_NIU1_A: u32 = 16;
pub const SRST_PDBUS_NIU1_H: u32 = 17;
pub const SRST_PDBUS_NIU1_P: u32 = 18;
pub const SRST_PDBUS_NIU2_A: u32 = 19;
pub const SRST_PDBUS_NIU2_H: u32 = 20;
pub const SRST_PDBUS_NIU3_A: u32 = 21;
pub const SRST_PDBUS_NIU3_H: u32 = 22;
pub const SRST_PDBUS_HOLD_NIU1_A: u32 = 23;
pub const SRST_DBG_NIU_P: u32 = 24;
pub const SRST_PDCORE_NIIU_H: u32 = 25;
pub const SRST_MUC_NIU: u32 = 26;
pub const SRST_DCF_A: u32 = 29;
pub const SRST_DCF_P: u32 = 30;
pub const SRST_SYSTEM_SRAM_A: u32 = 31;

/* cru_softrst_con2 */
pub const SRST_I2C1_P: u32 = 32;
pub const SRST_I2C1: u32 = 33;
pub const SRST_I2C3_P: u32 = 34;
pub const SRST_I2C3: u32 = 35;
pub const SRST_I2C4_P: u32 = 36;
pub const SRST_I2C4: u32 = 37;
pub const SRST_I2C5_P: u32 = 38;
pub const SRST_I2C5: u32 = 39;
pub const SRST_SPI1_P: u32 = 40;
pub const SRST_SPI1: u32 = 41;
pub const SRST_MCU_CORE: u32 = 42;
pub const SRST_PWM2_P: u32 = 44;
pub const SRST_PWM2: u32 = 45;
pub const SRST_SPINLOCK_A: u32 = 46;

/* cru_softrst_con3 */
pub const SRST_UART0_P: u32 = 48;
pub const SRST_UART0: u32 = 49;
pub const SRST_UART2_P: u32 = 50;
pub const SRST_UART2: u32 = 51;
pub const SRST_UART3_P: u32 = 52;
pub const SRST_UART3: u32 = 53;
pub const SRST_UART4_P: u32 = 54;
pub const SRST_UART4: u32 = 55;
pub const SRST_UART5_P: u32 = 56;
pub const SRST_UART5: u32 = 57;
pub const SRST_WDT_P: u32 = 58;
pub const SRST_SARADC_P: u32 = 59;
pub const SRST_GRF_P: u32 = 61;
pub const SRST_TIMER_P: u32 = 62;
pub const SRST_MAILBOX_P: u32 = 63;

/* cru_softrst_con4 */
pub const SRST_TIMER0: u32 = 64;
pub const SRST_TIMER1: u32 = 65;
pub const SRST_TIMER2: u32 = 66;
pub const SRST_TIMER3: u32 = 67;
pub const SRST_TIMER4: u32 = 68;
pub const SRST_TIMER5: u32 = 69;
pub const SRST_INTMUX_P: u32 = 70;
pub const SRST_GPIO1_P: u32 = 72;
pub const SRST_GPIO1_DB: u32 = 73;
pub const SRST_GPIO2_P: u32 = 74;
pub const SRST_GPIO2_DB: u32 = 75;
pub const SRST_GPIO3_P: u32 = 76;
pub const SRST_GPIO3_DB: u32 = 77;
pub const SRST_GPIO4_P: u32 = 78;
pub const SRST_GPIO4_DB: u32 = 79;

/* cru_softrst_con5 */
pub const SRST_CAN_P: u32 = 80;
pub const SRST_CAN: u32 = 81;
pub const SRST_DECOM_A: u32 = 85;
pub const SRST_DECOM_P: u32 = 86;
pub const SRST_DECOM_D: u32 = 87;
pub const SRST_PDCRYPTO_NIU_A: u32 = 88;
pub const SRST_PDCRYPTO_NIU_H: u32 = 89;
pub const SRST_CRYPTO_A: u32 = 90;
pub const SRST_CRYPTO_H: u32 = 91;
pub const SRST_CRYPTO_CORE: u32 = 92;
pub const SRST_CRYPTO_PKA: u32 = 93;
pub const SRST_SGRF_P: u32 = 95;

/* cru_softrst_con6 */
pub const SRST_PDAUDIO_NIU_H: u32 = 96;
pub const SRST_PDAUDIO_NIU_P: u32 = 97;
pub const SRST_I2S0_H: u32 = 98;
pub const SRST_I2S0_TX_M: u32 = 99;
pub const SRST_I2S0_RX_M: u32 = 100;
pub const SRST_I2S1_H: u32 = 101;
pub const SRST_I2S1_M: u32 = 102;
pub const SRST_I2S2_H: u32 = 103;
pub const SRST_I2S2_M: u32 = 104;
pub const SRST_PDM_H: u32 = 105;
pub const SRST_PDM_M: u32 = 106;
pub const SRST_AUDPWM_H: u32 = 107;
pub const SRST_AUDPWM: u32 = 108;
pub const SRST_ACDCDIG_P: u32 = 109;
pub const SRST_ACDCDIG: u32 = 110;

/* cru_softrst_con7 */
pub const SRST_PDVEPU_NIU_A: u32 = 112;
pub const SRST_PDVEPU_NIU_H: u32 = 113;
pub const SRST_VENC_A: u32 = 114;
pub const SRST_VENC_H: u32 = 115;
pub const SRST_VENC_CORE: u32 = 116;
pub const SRST_PDVDEC_NIU_A: u32 = 117;
pub const SRST_PDVDEC_NIU_H: u32 = 118;
pub const SRST_VDEC_A: u32 = 119;
pub const SRST_VDEC_H: u32 = 120;
pub const SRST_VDEC_CORE: u32 = 121;
pub const SRST_VDEC_CA: u32 = 122;
pub const SRST_VDEC_HEVC_CA: u32 = 123;
pub const SRST_PDJPEG_NIU_A: u32 = 124;
pub const SRST_PDJPEG_NIU_H: u32 = 125;
pub const SRST_JPEG_A: u32 = 126;
pub const SRST_JPEG_H: u32 = 127;

/* cru_softrst_con8 */
pub const SRST_PDVO_NIU_A: u32 = 128;
pub const SRST_PDVO_NIU_H: u32 = 129;
pub const SRST_PDVO_NIU_P: u32 = 130;
pub const SRST_RGA_A: u32 = 131;
pub const SRST_RGA_H: u32 = 132;
pub const SRST_RGA_CORE: u32 = 133;
pub const SRST_VOP_A: u32 = 134;
pub const SRST_VOP_H: u32 = 135;
pub const SRST_VOP_D: u32 = 136;
pub const SRST_TXBYTEHS_DSIHOST: u32 = 137;
pub const SRST_DSIHOST_P: u32 = 138;
pub const SRST_IEP_A: u32 = 139;
pub const SRST_IEP_H: u32 = 140;
pub const SRST_IEP_CORE: u32 = 141;
pub const SRST_ISP_RX_P: u32 = 142;

/* cru_softrst_con9 */
pub const SRST_PDVI_NIU_A: u32 = 144;
pub const SRST_PDVI_NIU_H: u32 = 145;
pub const SRST_PDVI_NIU_P: u32 = 146;
pub const SRST_ISP: u32 = 147;
pub const SRST_CIF_A: u32 = 148;
pub const SRST_CIF_H: u32 = 149;
pub const SRST_CIF_D: u32 = 150;
pub const SRST_CIF_P: u32 = 151;
pub const SRST_CIF_I: u32 = 152;
pub const SRST_CIF_RX_P: u32 = 153;
pub const SRST_PDISPP_NIU_A: u32 = 154;
pub const SRST_PDISPP_NIU_H: u32 = 155;
pub const SRST_ISPP_A: u32 = 156;
pub const SRST_ISPP_H: u32 = 157;
pub const SRST_ISPP: u32 = 158;
pub const SRST_CSIHOST_P: u32 = 159;

/* cru_softrst_con10 */
pub const SRST_PDPHPMID_NIU_A: u32 = 160;
pub const SRST_PDPHPMID_NIU_H: u32 = 161;
pub const SRST_PDNVM_NIU_H: u32 = 163;
pub const SRST_SDMMC_H: u32 = 164;
pub const SRST_SDIO_H: u32 = 165;
pub const SRST_EMMC_H: u32 = 166;
pub const SRST_SFC_H: u32 = 167;
pub const SRST_SFCXIP_H: u32 = 168;
pub const SRST_SFC: u32 = 169;
pub const SRST_NANDC_H: u32 = 170;
pub const SRST_NANDC: u32 = 171;
pub const SRST_PDSDMMC_H: u32 = 173;
pub const SRST_PDSDIO_H: u32 = 174;

/* cru_softrst_con11 */
pub const SRST_PDUSB_NIU_A: u32 = 176;
pub const SRST_PDUSB_NIU_H: u32 = 177;
pub const SRST_USBHOST_H: u32 = 178;
pub const SRST_USBHOST_ARB_H: u32 = 179;
pub const SRST_USBHOST_UTMI: u32 = 180;
pub const SRST_USBOTG_A: u32 = 181;
pub const SRST_USBPHY_OTG_P: u32 = 182;
pub const SRST_USBPHY_HOST_P: u32 = 183;
pub const SRST_USBPHYPOR_OTG: u32 = 184;
pub const SRST_USBPHYPOR_HOST: u32 = 185;
pub const SRST_PDGMAC_NIU_A: u32 = 188;
pub const SRST_PDGMAC_NIU_P: u32 = 189;
pub const SRST_GMAC_A: u32 = 190;

/* cru_softrst_con12 */
pub const SRST_DDR_DFICTL_P: u32 = 193;
pub const SRST_DDR_MON_P: u32 = 194;
pub const SRST_DDR_STANDBY_P: u32 = 195;
pub const SRST_DDR_GRF_P: u32 = 196;
pub const SRST_DDR_MSCH_P: u32 = 197;
pub const SRST_DDR_SPLIT_A: u32 = 198;
pub const SRST_DDR_MSCH: u32 = 199;
pub const SRST_DDR_DFICTL: u32 = 202;
pub const SRST_DDR_STANDBY: u32 = 203;
pub const SRST_NPUMCU_NIU: u32 = 205;
pub const SRST_DDRPHY_P: u32 = 206;
pub const SRST_DDRPHY: u32 = 207;

/* cru_softrst_con13 */
pub const SRST_PDNPU_NIU_A: u32 = 208;
pub const SRST_PDNPU_NIU_H: u32 = 209;
pub const SRST_PDNPU_NIU_P: u32 = 210;
pub const SRST_NPU_A: u32 = 211;
pub const SRST_NPU_H: u32 = 212;
pub const SRST_NPU: u32 = 213;
pub const SRST_NPUPVTM_P: u32 = 214;
pub const SRST_NPUPVTM: u32 = 215;
pub const SRST_NPU_TSADC_P: u32 = 216;
pub const SRST_NPU_TSADC: u32 = 217;
pub const SRST_NPU_TSADCPHY: u32 = 218;
pub const SRST_CIFLITE_A: u32 = 220;
pub const SRST_CIFLITE_H: u32 = 221;
pub const SRST_CIFLITE_D: u32 = 222;
pub const SRST_CIFLITE_RX_P: u32 = 223;

/* cru_softrst_con14 */
pub const SRST_TOPNIU_P: u32 = 224;
pub const SRST_TOPCRU_P: u32 = 225;
pub const SRST_TOPGRF_P: u32 = 226;
pub const SRST_CPUEMADET_P: u32 = 227;
pub const SRST_CSIPHY0_P: u32 = 228;
pub const SRST_CSIPHY1_P: u32 = 229;
pub const SRST_DSIPHY_P: u32 = 230;
pub const SRST_CPU_TSADC_P: u32 = 232;
pub const SRST_CPU_TSADC: u32 = 233;
pub const SRST_CPU_TSADCPHY: u32 = 234;
pub const SRST_CPUPVTM_P: u32 = 235;
pub const SRST_CPUPVTM: u32 = 236;

// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
