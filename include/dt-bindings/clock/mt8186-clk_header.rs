/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>
 */


/* MCUSYS */

pub const CLK_MCU_ARMPLL_LL_SEL: u32 = 0;
pub const CLK_MCU_ARMPLL_BL_SEL: u32 = 1;
pub const CLK_MCU_ARMPLL_BUS_SEL: u32 = 2;
pub const CLK_MCU_NR_CLK: u32 = 3;

/* TOPCKGEN */

pub const CLK_TOP_AXI: u32 = 0;
pub const CLK_TOP_SCP: u32 = 1;
pub const CLK_TOP_MFG: u32 = 2;
pub const CLK_TOP_CAMTG: u32 = 3;
pub const CLK_TOP_CAMTG1: u32 = 4;
pub const CLK_TOP_CAMTG2: u32 = 5;
pub const CLK_TOP_CAMTG3: u32 = 6;
pub const CLK_TOP_CAMTG4: u32 = 7;
pub const CLK_TOP_CAMTG5: u32 = 8;
pub const CLK_TOP_CAMTG6: u32 = 9;
pub const CLK_TOP_UART: u32 = 10;
pub const CLK_TOP_SPI: u32 = 11;
pub const CLK_TOP_MSDC50_0_HCLK: u32 = 12;
pub const CLK_TOP_MSDC50_0: u32 = 13;
pub const CLK_TOP_MSDC30_1: u32 = 14;
pub const CLK_TOP_AUDIO: u32 = 15;
pub const CLK_TOP_AUD_INTBUS: u32 = 16;
pub const CLK_TOP_AUD_1: u32 = 17;
pub const CLK_TOP_AUD_2: u32 = 18;
pub const CLK_TOP_AUD_ENGEN1: u32 = 19;
pub const CLK_TOP_AUD_ENGEN2: u32 = 20;
pub const CLK_TOP_DISP_PWM: u32 = 21;
pub const CLK_TOP_SSPM: u32 = 22;
pub const CLK_TOP_DXCC: u32 = 23;
pub const CLK_TOP_USB_TOP: u32 = 24;
pub const CLK_TOP_SRCK: u32 = 25;
pub const CLK_TOP_SPM: u32 = 26;
pub const CLK_TOP_I2C: u32 = 27;
pub const CLK_TOP_PWM: u32 = 28;
pub const CLK_TOP_SENINF: u32 = 29;
pub const CLK_TOP_SENINF1: u32 = 30;
pub const CLK_TOP_SENINF2: u32 = 31;
pub const CLK_TOP_SENINF3: u32 = 32;
pub const CLK_TOP_AES_MSDCFDE: u32 = 33;
pub const CLK_TOP_PWRAP_ULPOSC: u32 = 34;
pub const CLK_TOP_CAMTM: u32 = 35;
pub const CLK_TOP_VENC: u32 = 36;
pub const CLK_TOP_CAM: u32 = 37;
pub const CLK_TOP_IMG1: u32 = 38;
pub const CLK_TOP_IPE: u32 = 39;
pub const CLK_TOP_DPMAIF: u32 = 40;
pub const CLK_TOP_VDEC: u32 = 41;
pub const CLK_TOP_DISP: u32 = 42;
pub const CLK_TOP_MDP: u32 = 43;
pub const CLK_TOP_AUDIO_H: u32 = 44;
pub const CLK_TOP_UFS: u32 = 45;
pub const CLK_TOP_AES_FDE: u32 = 46;
pub const CLK_TOP_AUDIODSP: u32 = 47;
pub const CLK_TOP_DVFSRC: u32 = 48;
pub const CLK_TOP_DSI_OCC: u32 = 49;
pub const CLK_TOP_SPMI_MST: u32 = 50;
pub const CLK_TOP_SPINOR: u32 = 51;
pub const CLK_TOP_NNA: u32 = 52;
pub const CLK_TOP_NNA1: u32 = 53;
pub const CLK_TOP_NNA2: u32 = 54;
pub const CLK_TOP_SSUSB_XHCI: u32 = 55;
pub const CLK_TOP_SSUSB_TOP_1P: u32 = 56;
pub const CLK_TOP_SSUSB_XHCI_1P: u32 = 57;
pub const CLK_TOP_WPE: u32 = 58;
pub const CLK_TOP_DPI: u32 = 59;
pub const CLK_TOP_U3_OCC_250M: u32 = 60;
pub const CLK_TOP_U3_OCC_500M: u32 = 61;
pub const CLK_TOP_ADSP_BUS: u32 = 62;
pub const CLK_TOP_APLL_I2S0_MCK_SEL: u32 = 63;
pub const CLK_TOP_APLL_I2S1_MCK_SEL: u32 = 64;
pub const CLK_TOP_APLL_I2S2_MCK_SEL: u32 = 65;
pub const CLK_TOP_APLL_I2S4_MCK_SEL: u32 = 66;
pub const CLK_TOP_APLL_TDMOUT_MCK_SEL: u32 = 67;
pub const CLK_TOP_MAINPLL_D2: u32 = 68;
pub const CLK_TOP_MAINPLL_D2_D2: u32 = 69;
pub const CLK_TOP_MAINPLL_D2_D4: u32 = 70;
pub const CLK_TOP_MAINPLL_D2_D16: u32 = 71;
pub const CLK_TOP_MAINPLL_D3: u32 = 72;
pub const CLK_TOP_MAINPLL_D3_D2: u32 = 73;
pub const CLK_TOP_MAINPLL_D3_D4: u32 = 74;
pub const CLK_TOP_MAINPLL_D5: u32 = 75;
pub const CLK_TOP_MAINPLL_D5_D2: u32 = 76;
pub const CLK_TOP_MAINPLL_D5_D4: u32 = 77;
pub const CLK_TOP_MAINPLL_D7: u32 = 78;
pub const CLK_TOP_MAINPLL_D7_D2: u32 = 79;
pub const CLK_TOP_MAINPLL_D7_D4: u32 = 80;
pub const CLK_TOP_UNIVPLL: u32 = 81;
pub const CLK_TOP_UNIVPLL_D2: u32 = 82;
pub const CLK_TOP_UNIVPLL_D2_D2: u32 = 83;
pub const CLK_TOP_UNIVPLL_D2_D4: u32 = 84;
pub const CLK_TOP_UNIVPLL_D3: u32 = 85;
pub const CLK_TOP_UNIVPLL_D3_D2: u32 = 86;
pub const CLK_TOP_UNIVPLL_D3_D4: u32 = 87;
pub const CLK_TOP_UNIVPLL_D3_D8: u32 = 88;
pub const CLK_TOP_UNIVPLL_D3_D32: u32 = 89;
pub const CLK_TOP_UNIVPLL_D5: u32 = 90;
pub const CLK_TOP_UNIVPLL_D5_D2: u32 = 91;
pub const CLK_TOP_UNIVPLL_D5_D4: u32 = 92;
pub const CLK_TOP_UNIVPLL_D7: u32 = 93;
pub const CLK_TOP_UNIVPLL_192M: u32 = 94;
pub const CLK_TOP_UNIVPLL_192M_D4: u32 = 95;
pub const CLK_TOP_UNIVPLL_192M_D8: u32 = 96;
pub const CLK_TOP_UNIVPLL_192M_D16: u32 = 97;
pub const CLK_TOP_UNIVPLL_192M_D32: u32 = 98;
pub const CLK_TOP_APLL1_D2: u32 = 99;
pub const CLK_TOP_APLL1_D4: u32 = 100;
pub const CLK_TOP_APLL1_D8: u32 = 101;
pub const CLK_TOP_APLL2_D2: u32 = 102;
pub const CLK_TOP_APLL2_D4: u32 = 103;
pub const CLK_TOP_APLL2_D8: u32 = 104;
pub const CLK_TOP_MMPLL_D2: u32 = 105;
pub const CLK_TOP_TVDPLL_D2: u32 = 106;
pub const CLK_TOP_TVDPLL_D4: u32 = 107;
pub const CLK_TOP_TVDPLL_D8: u32 = 108;
pub const CLK_TOP_TVDPLL_D16: u32 = 109;
pub const CLK_TOP_TVDPLL_D32: u32 = 110;
pub const CLK_TOP_MSDCPLL_D2: u32 = 111;
pub const CLK_TOP_ULPOSC1: u32 = 112;
pub const CLK_TOP_ULPOSC1_D2: u32 = 113;
pub const CLK_TOP_ULPOSC1_D4: u32 = 114;
pub const CLK_TOP_ULPOSC1_D8: u32 = 115;
pub const CLK_TOP_ULPOSC1_D10: u32 = 116;
pub const CLK_TOP_ULPOSC1_D16: u32 = 117;
pub const CLK_TOP_ULPOSC1_D32: u32 = 118;
pub const CLK_TOP_ADSPPLL_D2: u32 = 119;
pub const CLK_TOP_ADSPPLL_D4: u32 = 120;
pub const CLK_TOP_ADSPPLL_D8: u32 = 121;
pub const CLK_TOP_NNAPLL_D2: u32 = 122;
pub const CLK_TOP_NNAPLL_D4: u32 = 123;
pub const CLK_TOP_NNAPLL_D8: u32 = 124;
pub const CLK_TOP_NNA2PLL_D2: u32 = 125;
pub const CLK_TOP_NNA2PLL_D4: u32 = 126;
pub const CLK_TOP_NNA2PLL_D8: u32 = 127;
pub const CLK_TOP_F_BIST2FPC: u32 = 128;
pub const CLK_TOP_466M_FMEM: u32 = 129;
pub const CLK_TOP_MPLL: u32 = 130;
pub const CLK_TOP_APLL12_CK_DIV0: u32 = 131;
pub const CLK_TOP_APLL12_CK_DIV1: u32 = 132;
pub const CLK_TOP_APLL12_CK_DIV2: u32 = 133;
pub const CLK_TOP_APLL12_CK_DIV4: u32 = 134;
pub const CLK_TOP_APLL12_CK_DIV_TDMOUT_M: u32 = 135;
pub const CLK_TOP_NR_CLK: u32 = 136;

/* INFRACFG_AO */

pub const CLK_INFRA_AO_PMIC_TMR: u32 = 0;
pub const CLK_INFRA_AO_PMIC_AP: u32 = 1;
pub const CLK_INFRA_AO_PMIC_MD: u32 = 2;
pub const CLK_INFRA_AO_PMIC_CONN: u32 = 3;
pub const CLK_INFRA_AO_SCP_CORE: u32 = 4;
pub const CLK_INFRA_AO_SEJ: u32 = 5;
pub const CLK_INFRA_AO_APXGPT: u32 = 6;
pub const CLK_INFRA_AO_ICUSB: u32 = 7;
pub const CLK_INFRA_AO_GCE: u32 = 8;
pub const CLK_INFRA_AO_THERM: u32 = 9;
pub const CLK_INFRA_AO_I2C_AP: u32 = 10;
pub const CLK_INFRA_AO_I2C_CCU: u32 = 11;
pub const CLK_INFRA_AO_I2C_SSPM: u32 = 12;
pub const CLK_INFRA_AO_I2C_RSV: u32 = 13;
pub const CLK_INFRA_AO_PWM_HCLK: u32 = 14;
pub const CLK_INFRA_AO_PWM1: u32 = 15;
pub const CLK_INFRA_AO_PWM2: u32 = 16;
pub const CLK_INFRA_AO_PWM3: u32 = 17;
pub const CLK_INFRA_AO_PWM4: u32 = 18;
pub const CLK_INFRA_AO_PWM5: u32 = 19;
pub const CLK_INFRA_AO_PWM: u32 = 20;
pub const CLK_INFRA_AO_UART0: u32 = 21;
pub const CLK_INFRA_AO_UART1: u32 = 22;
pub const CLK_INFRA_AO_UART2: u32 = 23;
pub const CLK_INFRA_AO_GCE_26M: u32 = 24;
pub const CLK_INFRA_AO_CQ_DMA_FPC: u32 = 25;
pub const CLK_INFRA_AO_BTIF: u32 = 26;
pub const CLK_INFRA_AO_SPI0: u32 = 27;
pub const CLK_INFRA_AO_MSDC0: u32 = 28;
pub const CLK_INFRA_AO_MSDCFDE: u32 = 29;
pub const CLK_INFRA_AO_MSDC1: u32 = 30;
pub const CLK_INFRA_AO_DVFSRC: u32 = 31;
pub const CLK_INFRA_AO_GCPU: u32 = 32;
pub const CLK_INFRA_AO_TRNG: u32 = 33;
pub const CLK_INFRA_AO_AUXADC: u32 = 34;
pub const CLK_INFRA_AO_CPUM: u32 = 35;
pub const CLK_INFRA_AO_CCIF1_AP: u32 = 36;
pub const CLK_INFRA_AO_CCIF1_MD: u32 = 37;
pub const CLK_INFRA_AO_AUXADC_MD: u32 = 38;
pub const CLK_INFRA_AO_AP_DMA: u32 = 39;
pub const CLK_INFRA_AO_XIU: u32 = 40;
pub const CLK_INFRA_AO_DEVICE_APC: u32 = 41;
pub const CLK_INFRA_AO_CCIF_AP: u32 = 42;
pub const CLK_INFRA_AO_DEBUGTOP: u32 = 43;
pub const CLK_INFRA_AO_AUDIO: u32 = 44;
pub const CLK_INFRA_AO_CCIF_MD: u32 = 45;
pub const CLK_INFRA_AO_DXCC_SEC_CORE: u32 = 46;
pub const CLK_INFRA_AO_DXCC_AO: u32 = 47;
pub const CLK_INFRA_AO_IMP_IIC: u32 = 48;
pub const CLK_INFRA_AO_DRAMC_F26M: u32 = 49;
pub const CLK_INFRA_AO_RG_PWM_FBCLK6: u32 = 50;
pub const CLK_INFRA_AO_SSUSB_TOP_HCLK: u32 = 51;
pub const CLK_INFRA_AO_DISP_PWM: u32 = 52;
pub const CLK_INFRA_AO_CLDMA_BCLK: u32 = 53;
pub const CLK_INFRA_AO_AUDIO_26M_BCLK: u32 = 54;
pub const CLK_INFRA_AO_SSUSB_TOP_P1_HCLK: u32 = 55;
pub const CLK_INFRA_AO_SPI1: u32 = 56;
pub const CLK_INFRA_AO_I2C4: u32 = 57;
pub const CLK_INFRA_AO_MODEM_TEMP_SHARE: u32 = 58;
pub const CLK_INFRA_AO_SPI2: u32 = 59;
pub const CLK_INFRA_AO_SPI3: u32 = 60;
pub const CLK_INFRA_AO_SSUSB_TOP_REF: u32 = 61;
pub const CLK_INFRA_AO_SSUSB_TOP_XHCI: u32 = 62;
pub const CLK_INFRA_AO_SSUSB_TOP_P1_REF: u32 = 63;
pub const CLK_INFRA_AO_SSUSB_TOP_P1_XHCI: u32 = 64;
pub const CLK_INFRA_AO_SSPM: u32 = 65;
pub const CLK_INFRA_AO_SSUSB_TOP_P1_SYS: u32 = 66;
pub const CLK_INFRA_AO_I2C5: u32 = 67;
pub const CLK_INFRA_AO_I2C5_ARBITER: u32 = 68;
pub const CLK_INFRA_AO_I2C5_IMM: u32 = 69;
pub const CLK_INFRA_AO_I2C1_ARBITER: u32 = 70;
pub const CLK_INFRA_AO_I2C1_IMM: u32 = 71;
pub const CLK_INFRA_AO_I2C2_ARBITER: u32 = 72;
pub const CLK_INFRA_AO_I2C2_IMM: u32 = 73;
pub const CLK_INFRA_AO_SPI4: u32 = 74;
pub const CLK_INFRA_AO_SPI5: u32 = 75;
pub const CLK_INFRA_AO_CQ_DMA: u32 = 76;
pub const CLK_INFRA_AO_BIST2FPC: u32 = 77;
pub const CLK_INFRA_AO_MSDC0_SELF: u32 = 78;
pub const CLK_INFRA_AO_SPINOR: u32 = 79;
pub const CLK_INFRA_AO_SSPM_26M_SELF: u32 = 80;
pub const CLK_INFRA_AO_SSPM_32K_SELF: u32 = 81;
pub const CLK_INFRA_AO_I2C6: u32 = 82;
pub const CLK_INFRA_AO_AP_MSDC0: u32 = 83;
pub const CLK_INFRA_AO_MD_MSDC0: u32 = 84;
pub const CLK_INFRA_AO_MSDC0_SRC: u32 = 85;
pub const CLK_INFRA_AO_MSDC1_SRC: u32 = 86;
pub const CLK_INFRA_AO_SEJ_F13M: u32 = 87;
pub const CLK_INFRA_AO_AES_TOP0_BCLK: u32 = 88;
pub const CLK_INFRA_AO_MCU_PM_BCLK: u32 = 89;
pub const CLK_INFRA_AO_CCIF2_AP: u32 = 90;
pub const CLK_INFRA_AO_CCIF2_MD: u32 = 91;
pub const CLK_INFRA_AO_CCIF3_AP: u32 = 92;
pub const CLK_INFRA_AO_CCIF3_MD: u32 = 93;
pub const CLK_INFRA_AO_FADSP_26M: u32 = 94;
pub const CLK_INFRA_AO_FADSP_32K: u32 = 95;
pub const CLK_INFRA_AO_CCIF4_AP: u32 = 96;
pub const CLK_INFRA_AO_CCIF4_MD: u32 = 97;
pub const CLK_INFRA_AO_FADSP: u32 = 98;
pub const CLK_INFRA_AO_FLASHIF_133M: u32 = 99;
pub const CLK_INFRA_AO_FLASHIF_66M: u32 = 100;
pub const CLK_INFRA_AO_NR_CLK: u32 = 101;

/* APMIXEDSYS */

pub const CLK_APMIXED_ARMPLL_LL: u32 = 0;
pub const CLK_APMIXED_ARMPLL_BL: u32 = 1;
pub const CLK_APMIXED_CCIPLL: u32 = 2;
pub const CLK_APMIXED_MAINPLL: u32 = 3;
pub const CLK_APMIXED_UNIV2PLL: u32 = 4;
pub const CLK_APMIXED_MSDCPLL: u32 = 5;
pub const CLK_APMIXED_MMPLL: u32 = 6;
pub const CLK_APMIXED_NNAPLL: u32 = 7;
pub const CLK_APMIXED_NNA2PLL: u32 = 8;
pub const CLK_APMIXED_ADSPPLL: u32 = 9;
pub const CLK_APMIXED_MFGPLL: u32 = 10;
pub const CLK_APMIXED_TVDPLL: u32 = 11;
pub const CLK_APMIXED_APLL1: u32 = 12;
pub const CLK_APMIXED_APLL2: u32 = 13;
pub const CLK_APMIXED_NR_CLK: u32 = 14;

/* IMP_IIC_WRAP */

pub const CLK_IMP_IIC_WRAP_AP_CLOCK_I2C0: u32 = 0;
pub const CLK_IMP_IIC_WRAP_AP_CLOCK_I2C1: u32 = 1;
pub const CLK_IMP_IIC_WRAP_AP_CLOCK_I2C2: u32 = 2;
pub const CLK_IMP_IIC_WRAP_AP_CLOCK_I2C3: u32 = 3;
pub const CLK_IMP_IIC_WRAP_AP_CLOCK_I2C4: u32 = 4;
pub const CLK_IMP_IIC_WRAP_AP_CLOCK_I2C5: u32 = 5;
pub const CLK_IMP_IIC_WRAP_AP_CLOCK_I2C6: u32 = 6;
pub const CLK_IMP_IIC_WRAP_AP_CLOCK_I2C7: u32 = 7;
pub const CLK_IMP_IIC_WRAP_AP_CLOCK_I2C8: u32 = 8;
pub const CLK_IMP_IIC_WRAP_AP_CLOCK_I2C9: u32 = 9;
pub const CLK_IMP_IIC_WRAP_NR_CLK: u32 = 10;

/* MFGCFG */

pub const CLK_MFG_BG3D: u32 = 0;
pub const CLK_MFG_NR_CLK: u32 = 1;

/* MMSYS */

pub const CLK_MM_DISP_MUTEX0: u32 = 0;
pub const CLK_MM_APB_MM_BUS: u32 = 1;
pub const CLK_MM_DISP_OVL0: u32 = 2;
pub const CLK_MM_DISP_RDMA0: u32 = 3;
pub const CLK_MM_DISP_OVL0_2L: u32 = 4;
pub const CLK_MM_DISP_WDMA0: u32 = 5;
pub const CLK_MM_DISP_RSZ0: u32 = 6;
pub const CLK_MM_DISP_AAL0: u32 = 7;
pub const CLK_MM_DISP_CCORR0: u32 = 8;
pub const CLK_MM_DISP_COLOR0: u32 = 9;
pub const CLK_MM_SMI_INFRA: u32 = 10;
pub const CLK_MM_DISP_DSC_WRAP0: u32 = 11;
pub const CLK_MM_DISP_GAMMA0: u32 = 12;
pub const CLK_MM_DISP_POSTMASK0: u32 = 13;
pub const CLK_MM_DISP_DITHER0: u32 = 14;
pub const CLK_MM_SMI_COMMON: u32 = 15;
pub const CLK_MM_DSI0: u32 = 16;
pub const CLK_MM_DISP_FAKE_ENG0: u32 = 17;
pub const CLK_MM_DISP_FAKE_ENG1: u32 = 18;
pub const CLK_MM_SMI_GALS: u32 = 19;
pub const CLK_MM_SMI_IOMMU: u32 = 20;
pub const CLK_MM_DISP_RDMA1: u32 = 21;
pub const CLK_MM_DISP_DPI: u32 = 22;
pub const CLK_MM_DSI0_DSI_CK_DOMAIN: u32 = 23;
pub const CLK_MM_DISP_26M: u32 = 24;
pub const CLK_MM_NR_CLK: u32 = 25;

/* WPESYS */

pub const CLK_WPE_CK_EN: u32 = 0;
pub const CLK_WPE_SMI_LARB8_CK_EN: u32 = 1;
pub const CLK_WPE_SYS_EVENT_TX_CK_EN: u32 = 2;
pub const CLK_WPE_SMI_LARB8_PCLK_EN: u32 = 3;
pub const CLK_WPE_NR_CLK: u32 = 4;

/* IMGSYS1 */

pub const CLK_IMG1_LARB9_IMG1: u32 = 0;
pub const CLK_IMG1_LARB10_IMG1: u32 = 1;
pub const CLK_IMG1_DIP: u32 = 2;
pub const CLK_IMG1_GALS_IMG1: u32 = 3;
pub const CLK_IMG1_NR_CLK: u32 = 4;

/* IMGSYS2 */

pub const CLK_IMG2_LARB9_IMG2: u32 = 0;
pub const CLK_IMG2_LARB10_IMG2: u32 = 1;
pub const CLK_IMG2_MFB: u32 = 2;
pub const CLK_IMG2_WPE: u32 = 3;
pub const CLK_IMG2_MSS: u32 = 4;
pub const CLK_IMG2_GALS_IMG2: u32 = 5;
pub const CLK_IMG2_NR_CLK: u32 = 6;

/* VDECSYS */

pub const CLK_VDEC_LARB1_CKEN: u32 = 0;
pub const CLK_VDEC_LAT_CKEN: u32 = 1;
pub const CLK_VDEC_LAT_ACTIVE: u32 = 2;
pub const CLK_VDEC_LAT_CKEN_ENG: u32 = 3;
pub const CLK_VDEC_MINI_MDP_CKEN_CFG_RG: u32 = 4;
pub const CLK_VDEC_CKEN: u32 = 5;
pub const CLK_VDEC_ACTIVE: u32 = 6;
pub const CLK_VDEC_CKEN_ENG: u32 = 7;
pub const CLK_VDEC_NR_CLK: u32 = 8;

/* VENCSYS */

pub const CLK_VENC_CKE0_LARB: u32 = 0;
pub const CLK_VENC_CKE1_VENC: u32 = 1;
pub const CLK_VENC_CKE2_JPGENC: u32 = 2;
pub const CLK_VENC_CKE5_GALS: u32 = 3;
pub const CLK_VENC_NR_CLK: u32 = 4;

/* CAMSYS */

pub const CLK_CAM_LARB13: u32 = 0;
pub const CLK_CAM_DFP_VAD: u32 = 1;
pub const CLK_CAM_LARB14: u32 = 2;
pub const CLK_CAM: u32 = 3;
pub const CLK_CAMTG: u32 = 4;
pub const CLK_CAM_SENINF: u32 = 5;
pub const CLK_CAMSV1: u32 = 6;
pub const CLK_CAMSV2: u32 = 7;
pub const CLK_CAMSV3: u32 = 8;
pub const CLK_CAM_CCU0: u32 = 9;
pub const CLK_CAM_CCU1: u32 = 10;
pub const CLK_CAM_MRAW0: u32 = 11;
pub const CLK_CAM_FAKE_ENG: u32 = 12;
pub const CLK_CAM_CCU_GALS: u32 = 13;
pub const CLK_CAM2MM_GALS: u32 = 14;
pub const CLK_CAM_NR_CLK: u32 = 15;

/* CAMSYS_RAWA */

pub const CLK_CAM_RAWA_LARBX_RAWA: u32 = 0;
pub const CLK_CAM_RAWA: u32 = 1;
pub const CLK_CAM_RAWA_CAMTG_RAWA: u32 = 2;
pub const CLK_CAM_RAWA_NR_CLK: u32 = 3;

/* CAMSYS_RAWB */

pub const CLK_CAM_RAWB_LARBX_RAWB: u32 = 0;
pub const CLK_CAM_RAWB: u32 = 1;
pub const CLK_CAM_RAWB_CAMTG_RAWB: u32 = 2;
pub const CLK_CAM_RAWB_NR_CLK: u32 = 3;

/* MDPSYS */

pub const CLK_MDP_RDMA0: u32 = 0;
pub const CLK_MDP_TDSHP0: u32 = 1;
pub const CLK_MDP_IMG_DL_ASYNC0: u32 = 2;
pub const CLK_MDP_IMG_DL_ASYNC1: u32 = 3;
pub const CLK_MDP_DISP_RDMA: u32 = 4;
pub const CLK_MDP_HMS: u32 = 5;
pub const CLK_MDP_SMI0: u32 = 6;
pub const CLK_MDP_APB_BUS: u32 = 7;
pub const CLK_MDP_WROT0: u32 = 8;
pub const CLK_MDP_RSZ0: u32 = 9;
pub const CLK_MDP_HDR0: u32 = 10;
pub const CLK_MDP_MUTEX0: u32 = 11;
pub const CLK_MDP_WROT1: u32 = 12;
pub const CLK_MDP_RSZ1: u32 = 13;
pub const CLK_MDP_FAKE_ENG0: u32 = 14;
pub const CLK_MDP_AAL0: u32 = 15;
pub const CLK_MDP_DISP_WDMA: u32 = 16;
pub const CLK_MDP_COLOR: u32 = 17;
pub const CLK_MDP_IMG_DL_ASYNC2: u32 = 18;
pub const CLK_MDP_IMG_DL_RELAY0_ASYNC0: u32 = 19;
pub const CLK_MDP_IMG_DL_RELAY1_ASYNC1: u32 = 20;
pub const CLK_MDP_IMG_DL_RELAY2_ASYNC2: u32 = 21;
pub const CLK_MDP_NR_CLK: u32 = 22;

/* IPESYS */

pub const CLK_IPE_LARB19: u32 = 0;
pub const CLK_IPE_LARB20: u32 = 1;
pub const CLK_IPE_SMI_SUBCOM: u32 = 2;
pub const CLK_IPE_FD: u32 = 3;
pub const CLK_IPE_FE: u32 = 4;
pub const CLK_IPE_RSC: u32 = 5;
pub const CLK_IPE_DPE: u32 = 6;
pub const CLK_IPE_GALS_IPE: u32 = 7;
pub const CLK_IPE_NR_CLK: u32 = 8;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
