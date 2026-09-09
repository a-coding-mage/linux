/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018 MediaTek Inc.
 * Author: Weiyi Lu <weiyi.lu@mediatek.com>
 */

// Conditional include guard: _DT_BINDINGS_CLK_MT8183_H
// Header guard definition: _DT_BINDINGS_CLK_MT8183_H

/* APMIXED */
pub const CLK_APMIXED_ARMPLL_LL: u32 = 0;
pub const CLK_APMIXED_ARMPLL_L: u32 = 1;
pub const CLK_APMIXED_CCIPLL: u32 = 2;
pub const CLK_APMIXED_MAINPLL: u32 = 3;
pub const CLK_APMIXED_UNIV2PLL: u32 = 4;
pub const CLK_APMIXED_MSDCPLL: u32 = 5;
pub const CLK_APMIXED_MMPLL: u32 = 6;
pub const CLK_APMIXED_MFGPLL: u32 = 7;
pub const CLK_APMIXED_TVDPLL: u32 = 8;
pub const CLK_APMIXED_APLL1: u32 = 9;
pub const CLK_APMIXED_APLL2: u32 = 10;
pub const CLK_APMIXED_SSUSB_26M: u32 = 11;
pub const CLK_APMIXED_APPLL_26M: u32 = 12;
pub const CLK_APMIXED_MIPIC0_26M: u32 = 13;
pub const CLK_APMIXED_MDPLLGP_26M: u32 = 14;
pub const CLK_APMIXED_MMSYS_26M: u32 = 15;
pub const CLK_APMIXED_UFS_26M: u32 = 16;
pub const CLK_APMIXED_MIPIC1_26M: u32 = 17;
pub const CLK_APMIXED_MEMPLL_26M: u32 = 18;
pub const CLK_APMIXED_CLKSQ_LVPLL_26M: u32 = 19;
pub const CLK_APMIXED_MIPID0_26M: u32 = 20;
pub const CLK_APMIXED_MIPID1_26M: u32 = 21;
pub const CLK_APMIXED_NR_CLK: u32 = 22;

/* TOPCKGEN */
pub const CLK_TOP_MUX_AXI: u32 = 0;
pub const CLK_TOP_MUX_MM: u32 = 1;
pub const CLK_TOP_MUX_CAM: u32 = 2;
pub const CLK_TOP_MUX_MFG: u32 = 3;
pub const CLK_TOP_MUX_CAMTG: u32 = 4;
pub const CLK_TOP_MUX_UART: u32 = 5;
pub const CLK_TOP_MUX_SPI: u32 = 6;
pub const CLK_TOP_MUX_MSDC50_0_HCLK: u32 = 7;
pub const CLK_TOP_MUX_MSDC50_0: u32 = 8;
pub const CLK_TOP_MUX_MSDC30_1: u32 = 9;
pub const CLK_TOP_MUX_MSDC30_2: u32 = 10;
pub const CLK_TOP_MUX_AUDIO: u32 = 11;
pub const CLK_TOP_MUX_AUD_INTBUS: u32 = 12;
pub const CLK_TOP_MUX_FPWRAP_ULPOSC: u32 = 13;
pub const CLK_TOP_MUX_SCP: u32 = 14;
pub const CLK_TOP_MUX_ATB: u32 = 15;
pub const CLK_TOP_MUX_SSPM: u32 = 16;
pub const CLK_TOP_MUX_DPI0: u32 = 17;
pub const CLK_TOP_MUX_SCAM: u32 = 18;
pub const CLK_TOP_MUX_AUD_1: u32 = 19;
pub const CLK_TOP_MUX_AUD_2: u32 = 20;
pub const CLK_TOP_MUX_DISP_PWM: u32 = 21;
pub const CLK_TOP_MUX_SSUSB_TOP_XHCI: u32 = 22;
pub const CLK_TOP_MUX_USB_TOP: u32 = 23;
pub const CLK_TOP_MUX_SPM: u32 = 24;
pub const CLK_TOP_MUX_I2C: u32 = 25;
pub const CLK_TOP_MUX_F52M_MFG: u32 = 26;
pub const CLK_TOP_MUX_SENINF: u32 = 27;
pub const CLK_TOP_MUX_DXCC: u32 = 28;
pub const CLK_TOP_MUX_CAMTG2: u32 = 29;
pub const CLK_TOP_MUX_AUD_ENG1: u32 = 30;
pub const CLK_TOP_MUX_AUD_ENG2: u32 = 31;
pub const CLK_TOP_MUX_FAES_UFSFDE: u32 = 32;
pub const CLK_TOP_MUX_FUFS: u32 = 33;
pub const CLK_TOP_MUX_IMG: u32 = 34;
pub const CLK_TOP_MUX_DSP: u32 = 35;
pub const CLK_TOP_MUX_DSP1: u32 = 36;
pub const CLK_TOP_MUX_DSP2: u32 = 37;
pub const CLK_TOP_MUX_IPU_IF: u32 = 38;
pub const CLK_TOP_MUX_CAMTG3: u32 = 39;
pub const CLK_TOP_MUX_CAMTG4: u32 = 40;
pub const CLK_TOP_MUX_PMICSPI: u32 = 41;
pub const CLK_TOP_SYSPLL_CK: u32 = 42;
pub const CLK_TOP_SYSPLL_D2: u32 = 43;
pub const CLK_TOP_SYSPLL_D3: u32 = 44;
pub const CLK_TOP_SYSPLL_D5: u32 = 45;
pub const CLK_TOP_SYSPLL_D7: u32 = 46;
pub const CLK_TOP_SYSPLL_D2_D2: u32 = 47;
pub const CLK_TOP_SYSPLL_D2_D4: u32 = 48;
pub const CLK_TOP_SYSPLL_D2_D8: u32 = 49;
pub const CLK_TOP_SYSPLL_D2_D16: u32 = 50;
pub const CLK_TOP_SYSPLL_D3_D2: u32 = 51;
pub const CLK_TOP_SYSPLL_D3_D4: u32 = 52;
pub const CLK_TOP_SYSPLL_D3_D8: u32 = 53;
pub const CLK_TOP_SYSPLL_D5_D2: u32 = 54;
pub const CLK_TOP_SYSPLL_D5_D4: u32 = 55;
pub const CLK_TOP_SYSPLL_D7_D2: u32 = 56;
pub const CLK_TOP_SYSPLL_D7_D4: u32 = 57;
pub const CLK_TOP_UNIVPLL_CK: u32 = 58;
pub const CLK_TOP_UNIVPLL_D2: u32 = 59;
pub const CLK_TOP_UNIVPLL_D3: u32 = 60;
pub const CLK_TOP_UNIVPLL_D5: u32 = 61;
pub const CLK_TOP_UNIVPLL_D7: u32 = 62;
pub const CLK_TOP_UNIVPLL_D2_D2: u32 = 63;
pub const CLK_TOP_UNIVPLL_D2_D4: u32 = 64;
pub const CLK_TOP_UNIVPLL_D2_D8: u32 = 65;
pub const CLK_TOP_UNIVPLL_D3_D2: u32 = 66;
pub const CLK_TOP_UNIVPLL_D3_D4: u32 = 67;
pub const CLK_TOP_UNIVPLL_D3_D8: u32 = 68;
pub const CLK_TOP_UNIVPLL_D5_D2: u32 = 69;
pub const CLK_TOP_UNIVPLL_D5_D4: u32 = 70;
pub const CLK_TOP_UNIVPLL_D5_D8: u32 = 71;
pub const CLK_TOP_APLL1_CK: u32 = 72;
pub const CLK_TOP_APLL1_D2: u32 = 73;
pub const CLK_TOP_APLL1_D4: u32 = 74;
pub const CLK_TOP_APLL1_D8: u32 = 75;
pub const CLK_TOP_APLL2_CK: u32 = 76;
pub const CLK_TOP_APLL2_D2: u32 = 77;
pub const CLK_TOP_APLL2_D4: u32 = 78;
pub const CLK_TOP_APLL2_D8: u32 = 79;
pub const CLK_TOP_TVDPLL_CK: u32 = 80;
pub const CLK_TOP_TVDPLL_D2: u32 = 81;
pub const CLK_TOP_TVDPLL_D4: u32 = 82;
pub const CLK_TOP_TVDPLL_D8: u32 = 83;
pub const CLK_TOP_TVDPLL_D16: u32 = 84;
pub const CLK_TOP_MSDCPLL_CK: u32 = 85;
pub const CLK_TOP_MSDCPLL_D2: u32 = 86;
pub const CLK_TOP_MSDCPLL_D4: u32 = 87;
pub const CLK_TOP_MSDCPLL_D8: u32 = 88;
pub const CLK_TOP_MSDCPLL_D16: u32 = 89;
pub const CLK_TOP_AD_OSC_CK: u32 = 90;
pub const CLK_TOP_OSC_D2: u32 = 91;
pub const CLK_TOP_OSC_D4: u32 = 92;
pub const CLK_TOP_OSC_D8: u32 = 93;
pub const CLK_TOP_OSC_D16: u32 = 94;
pub const CLK_TOP_F26M_CK_D2: u32 = 95;
pub const CLK_TOP_MFGPLL_CK: u32 = 96;
pub const CLK_TOP_UNIVP_192M_CK: u32 = 97;
pub const CLK_TOP_UNIVP_192M_D2: u32 = 98;
pub const CLK_TOP_UNIVP_192M_D4: u32 = 99;
pub const CLK_TOP_UNIVP_192M_D8: u32 = 100;
pub const CLK_TOP_UNIVP_192M_D16: u32 = 101;
pub const CLK_TOP_UNIVP_192M_D32: u32 = 102;
pub const CLK_TOP_MMPLL_CK: u32 = 103;
pub const CLK_TOP_MMPLL_D4: u32 = 104;
pub const CLK_TOP_MMPLL_D4_D2: u32 = 105;
pub const CLK_TOP_MMPLL_D4_D4: u32 = 106;
pub const CLK_TOP_MMPLL_D5: u32 = 107;
pub const CLK_TOP_MMPLL_D5_D2: u32 = 108;
pub const CLK_TOP_MMPLL_D5_D4: u32 = 109;
pub const CLK_TOP_MMPLL_D6: u32 = 110;
pub const CLK_TOP_MMPLL_D7: u32 = 111;
pub const CLK_TOP_CLK26M: u32 = 112;
pub const CLK_TOP_CLK13M: u32 = 113;
pub const CLK_TOP_ULPOSC: u32 = 114;
pub const CLK_TOP_UNIVP_192M: u32 = 115;
pub const CLK_TOP_MUX_APLL_I2S0: u32 = 116;
pub const CLK_TOP_MUX_APLL_I2S1: u32 = 117;
pub const CLK_TOP_MUX_APLL_I2S2: u32 = 118;
pub const CLK_TOP_MUX_APLL_I2S3: u32 = 119;
pub const CLK_TOP_MUX_APLL_I2S4: u32 = 120;
pub const CLK_TOP_MUX_APLL_I2S5: u32 = 121;
pub const CLK_TOP_APLL12_DIV0: u32 = 122;
pub const CLK_TOP_APLL12_DIV1: u32 = 123;
pub const CLK_TOP_APLL12_DIV2: u32 = 124;
pub const CLK_TOP_APLL12_DIV3: u32 = 125;
pub const CLK_TOP_APLL12_DIV4: u32 = 126;
pub const CLK_TOP_APLL12_DIVB: u32 = 127;
pub const CLK_TOP_UNIVPLL: u32 = 128;
pub const CLK_TOP_ARMPLL_DIV_PLL1: u32 = 129;
pub const CLK_TOP_ARMPLL_DIV_PLL2: u32 = 130;
pub const CLK_TOP_UNIVPLL_D3_D16: u32 = 131;
pub const CLK_TOP_NR_CLK: u32 = 132;

/* CAMSYS */
pub const CLK_CAM_LARB6: u32 = 0;
pub const CLK_CAM_DFP_VAD: u32 = 1;
pub const CLK_CAM_CAM: u32 = 2;
pub const CLK_CAM_CAMTG: u32 = 3;
pub const CLK_CAM_SENINF: u32 = 4;
pub const CLK_CAM_CAMSV0: u32 = 5;
pub const CLK_CAM_CAMSV1: u32 = 6;
pub const CLK_CAM_CAMSV2: u32 = 7;
pub const CLK_CAM_CCU: u32 = 8;
pub const CLK_CAM_LARB3: u32 = 9;
pub const CLK_CAM_NR_CLK: u32 = 10;

/* INFRACFG_AO */
pub const CLK_INFRA_PMIC_TMR: u32 = 0;
pub const CLK_INFRA_PMIC_AP: u32 = 1;
pub const CLK_INFRA_PMIC_MD: u32 = 2;
pub const CLK_INFRA_PMIC_CONN: u32 = 3;
pub const CLK_INFRA_SCPSYS: u32 = 4;
pub const CLK_INFRA_SEJ: u32 = 5;
pub const CLK_INFRA_APXGPT: u32 = 6;
pub const CLK_INFRA_ICUSB: u32 = 7;
pub const CLK_INFRA_GCE: u32 = 8;
pub const CLK_INFRA_THERM: u32 = 9;
pub const CLK_INFRA_I2C0: u32 = 10;
pub const CLK_INFRA_I2C1: u32 = 11;
pub const CLK_INFRA_I2C2: u32 = 12;
pub const CLK_INFRA_I2C3: u32 = 13;
pub const CLK_INFRA_PWM_HCLK: u32 = 14;
pub const CLK_INFRA_PWM1: u32 = 15;
pub const CLK_INFRA_PWM2: u32 = 16;
pub const CLK_INFRA_PWM3: u32 = 17;
pub const CLK_INFRA_PWM4: u32 = 18;
pub const CLK_INFRA_PWM: u32 = 19;
pub const CLK_INFRA_UART0: u32 = 20;
pub const CLK_INFRA_UART1: u32 = 21;
pub const CLK_INFRA_UART2: u32 = 22;
pub const CLK_INFRA_UART3: u32 = 23;
pub const CLK_INFRA_GCE_26M: u32 = 24;
pub const CLK_INFRA_CQ_DMA_FPC: u32 = 25;
pub const CLK_INFRA_BTIF: u32 = 26;
pub const CLK_INFRA_SPI0: u32 = 27;
pub const CLK_INFRA_MSDC0: u32 = 28;
pub const CLK_INFRA_MSDC1: u32 = 29;
pub const CLK_INFRA_MSDC2: u32 = 30;
pub const CLK_INFRA_MSDC0_SCK: u32 = 31;
pub const CLK_INFRA_DVFSRC: u32 = 32;
pub const CLK_INFRA_GCPU: u32 = 33;
pub const CLK_INFRA_TRNG: u32 = 34;
pub const CLK_INFRA_AUXADC: u32 = 35;
pub const CLK_INFRA_CPUM: u32 = 36;
pub const CLK_INFRA_CCIF1_AP: u32 = 37;
pub const CLK_INFRA_CCIF1_MD: u32 = 38;
pub const CLK_INFRA_AUXADC_MD: u32 = 39;
pub const CLK_INFRA_MSDC1_SCK: u32 = 40;
pub const CLK_INFRA_MSDC2_SCK: u32 = 41;
pub const CLK_INFRA_AP_DMA: u32 = 42;
pub const CLK_INFRA_XIU: u32 = 43;
pub const CLK_INFRA_DEVICE_APC: u32 = 44;
pub const CLK_INFRA_CCIF_AP: u32 = 45;
pub const CLK_INFRA_DEBUGSYS: u32 = 46;
pub const CLK_INFRA_AUDIO: u32 = 47;
pub const CLK_INFRA_CCIF_MD: u32 = 48;
pub const CLK_INFRA_DXCC_SEC_CORE: u32 = 49;
pub const CLK_INFRA_DXCC_AO: u32 = 50;
pub const CLK_INFRA_DRAMC_F26M: u32 = 51;
pub const CLK_INFRA_IRTX: u32 = 52;
pub const CLK_INFRA_DISP_PWM: u32 = 53;
pub const CLK_INFRA_CLDMA_BCLK: u32 = 54;
pub const CLK_INFRA_AUDIO_26M_BCLK: u32 = 55;
pub const CLK_INFRA_SPI1: u32 = 56;
pub const CLK_INFRA_I2C4: u32 = 57;
pub const CLK_INFRA_MODEM_TEMP_SHARE: u32 = 58;
pub const CLK_INFRA_SPI2: u32 = 59;
pub const CLK_INFRA_SPI3: u32 = 60;
pub const CLK_INFRA_UNIPRO_SCK: u32 = 61;
pub const CLK_INFRA_UNIPRO_TICK: u32 = 62;
pub const CLK_INFRA_UFS_MP_SAP_BCLK: u32 = 63;
pub const CLK_INFRA_MD32_BCLK: u32 = 64;
pub const CLK_INFRA_SSPM: u32 = 65;
pub const CLK_INFRA_UNIPRO_MBIST: u32 = 66;
pub const CLK_INFRA_SSPM_BUS_HCLK: u32 = 67;
pub const CLK_INFRA_I2C5: u32 = 68;
pub const CLK_INFRA_I2C5_ARBITER: u32 = 69;
pub const CLK_INFRA_I2C5_IMM: u32 = 70;
pub const CLK_INFRA_I2C1_ARBITER: u32 = 71;
pub const CLK_INFRA_I2C1_IMM: u32 = 72;
pub const CLK_INFRA_I2C2_ARBITER: u32 = 73;
pub const CLK_INFRA_I2C2_IMM: u32 = 74;
pub const CLK_INFRA_SPI4: u32 = 75;
pub const CLK_INFRA_SPI5: u32 = 76;
pub const CLK_INFRA_CQ_DMA: u32 = 77;
pub const CLK_INFRA_UFS: u32 = 78;
pub const CLK_INFRA_AES_UFSFDE: u32 = 79;
pub const CLK_INFRA_UFS_TICK: u32 = 80;
pub const CLK_INFRA_MSDC0_SELF: u32 = 81;
pub const CLK_INFRA_MSDC1_SELF: u32 = 82;
pub const CLK_INFRA_MSDC2_SELF: u32 = 83;
pub const CLK_INFRA_SSPM_26M_SELF: u32 = 84;
pub const CLK_INFRA_SSPM_32K_SELF: u32 = 85;
pub const CLK_INFRA_UFS_AXI: u32 = 86;
pub const CLK_INFRA_I2C6: u32 = 87;
pub const CLK_INFRA_AP_MSDC0: u32 = 88;
pub const CLK_INFRA_MD_MSDC0: u32 = 89;
pub const CLK_INFRA_USB: u32 = 90;
pub const CLK_INFRA_DEVMPU_BCLK: u32 = 91;
pub const CLK_INFRA_CCIF2_AP: u32 = 92;
pub const CLK_INFRA_CCIF2_MD: u32 = 93;
pub const CLK_INFRA_CCIF3_AP: u32 = 94;
pub const CLK_INFRA_CCIF3_MD: u32 = 95;
pub const CLK_INFRA_SEJ_F13M: u32 = 96;
pub const CLK_INFRA_AES_BCLK: u32 = 97;
pub const CLK_INFRA_I2C7: u32 = 98;
pub const CLK_INFRA_I2C8: u32 = 99;
pub const CLK_INFRA_FBIST2FPC: u32 = 100;
pub const CLK_INFRA_NR_CLK: u32 = 101;

/* PERICFG */
pub const CLK_PERI_AXI: u32 = 0;
pub const CLK_PERI_NR_CLK: u32 = 1;

/* MFGCFG */
pub const CLK_MFG_BG3D: u32 = 0;
pub const CLK_MFG_NR_CLK: u32 = 1;

/* IMG */
pub const CLK_IMG_OWE: u32 = 0;
pub const CLK_IMG_WPE_B: u32 = 1;
pub const CLK_IMG_WPE_A: u32 = 2;
pub const CLK_IMG_MFB: u32 = 3;
pub const CLK_IMG_RSC: u32 = 4;
pub const CLK_IMG_DPE: u32 = 5;
pub const CLK_IMG_FDVT: u32 = 6;
pub const CLK_IMG_DIP: u32 = 7;
pub const CLK_IMG_LARB2: u32 = 8;
pub const CLK_IMG_LARB5: u32 = 9;
pub const CLK_IMG_NR_CLK: u32 = 10;

/* MMSYS_CONFIG */
pub const CLK_MM_SMI_COMMON: u32 = 0;
pub const CLK_MM_SMI_LARB0: u32 = 1;
pub const CLK_MM_SMI_LARB1: u32 = 2;
pub const CLK_MM_GALS_COMM0: u32 = 3;
pub const CLK_MM_GALS_COMM1: u32 = 4;
pub const CLK_MM_GALS_CCU2MM: u32 = 5;
pub const CLK_MM_GALS_IPU12MM: u32 = 6;
pub const CLK_MM_GALS_IMG2MM: u32 = 7;
pub const CLK_MM_GALS_CAM2MM: u32 = 8;
pub const CLK_MM_GALS_IPU2MM: u32 = 9;
pub const CLK_MM_MDP_DL_TXCK: u32 = 10;
pub const CLK_MM_IPU_DL_TXCK: u32 = 11;
pub const CLK_MM_MDP_RDMA0: u32 = 12;
pub const CLK_MM_MDP_RDMA1: u32 = 13;
pub const CLK_MM_MDP_RSZ0: u32 = 14;
pub const CLK_MM_MDP_RSZ1: u32 = 15;
pub const CLK_MM_MDP_TDSHP: u32 = 16;
pub const CLK_MM_MDP_WROT0: u32 = 17;
pub const CLK_MM_FAKE_ENG: u32 = 18;
pub const CLK_MM_DISP_OVL0: u32 = 19;
pub const CLK_MM_DISP_OVL0_2L: u32 = 20;
pub const CLK_MM_DISP_OVL1_2L: u32 = 21;
pub const CLK_MM_DISP_RDMA0: u32 = 22;
pub const CLK_MM_DISP_RDMA1: u32 = 23;
pub const CLK_MM_DISP_WDMA0: u32 = 24;
pub const CLK_MM_DISP_COLOR0: u32 = 25;
pub const CLK_MM_DISP_CCORR0: u32 = 26;
pub const CLK_MM_DISP_AAL0: u32 = 27;
pub const CLK_MM_DISP_GAMMA0: u32 = 28;
pub const CLK_MM_DISP_DITHER0: u32 = 29;
pub const CLK_MM_DISP_SPLIT: u32 = 30;
pub const CLK_MM_DSI0_MM: u32 = 31;
pub const CLK_MM_DSI0_IF: u32 = 32;
pub const CLK_MM_DPI_MM: u32 = 33;
pub const CLK_MM_DPI_IF: u32 = 34;
pub const CLK_MM_FAKE_ENG2: u32 = 35;
pub const CLK_MM_MDP_DL_RX: u32 = 36;
pub const CLK_MM_IPU_DL_RX: u32 = 37;
pub const CLK_MM_26M: u32 = 38;
pub const CLK_MM_MMSYS_R2Y: u32 = 39;
pub const CLK_MM_DISP_RSZ: u32 = 40;
pub const CLK_MM_MDP_WDMA0: u32 = 41;
pub const CLK_MM_MDP_AAL: u32 = 42;
pub const CLK_MM_MDP_CCORR: u32 = 43;
pub const CLK_MM_DBI_MM: u32 = 44;
pub const CLK_MM_DBI_IF: u32 = 45;
pub const CLK_MM_NR_CLK: u32 = 46;

/* VDEC_GCON */
pub const CLK_VDEC_VDEC: u32 = 0;
pub const CLK_VDEC_LARB1: u32 = 1;
pub const CLK_VDEC_NR_CLK: u32 = 2;

/* VENC_GCON */
pub const CLK_VENC_LARB: u32 = 0;
pub const CLK_VENC_VENC: u32 = 1;
pub const CLK_VENC_JPGENC: u32 = 2;
pub const CLK_VENC_NR_CLK: u32 = 3;

/* AUDIO */
pub const CLK_AUDIO_TML: u32 = 0;
pub const CLK_AUDIO_DAC_PREDIS: u32 = 1;
pub const CLK_AUDIO_DAC: u32 = 2;
pub const CLK_AUDIO_ADC: u32 = 3;
pub const CLK_AUDIO_APLL_TUNER: u32 = 4;
pub const CLK_AUDIO_APLL2_TUNER: u32 = 5;
pub const CLK_AUDIO_24M: u32 = 6;
pub const CLK_AUDIO_22M: u32 = 7;
pub const CLK_AUDIO_AFE: u32 = 8;
pub const CLK_AUDIO_I2S4: u32 = 9;
pub const CLK_AUDIO_I2S3: u32 = 10;
pub const CLK_AUDIO_I2S2: u32 = 11;
pub const CLK_AUDIO_I2S1: u32 = 12;
pub const CLK_AUDIO_PDN_ADDA6_ADC: u32 = 13;
pub const CLK_AUDIO_TDM: u32 = 14;
pub const CLK_AUDIO_NR_CLK: u32 = 15;

/* IPU_CONN */
pub const CLK_IPU_CONN_IPU: u32 = 0;
pub const CLK_IPU_CONN_AHB: u32 = 1;
pub const CLK_IPU_CONN_AXI: u32 = 2;
pub const CLK_IPU_CONN_ISP: u32 = 3;
pub const CLK_IPU_CONN_CAM_ADL: u32 = 4;
pub const CLK_IPU_CONN_IMG_ADL: u32 = 5;
pub const CLK_IPU_CONN_DAP_RX: u32 = 6;
pub const CLK_IPU_CONN_APB2AXI: u32 = 7;
pub const CLK_IPU_CONN_APB2AHB: u32 = 8;
pub const CLK_IPU_CONN_IPU_CAB1TO2: u32 = 9;
pub const CLK_IPU_CONN_IPU1_CAB1TO2: u32 = 10;
pub const CLK_IPU_CONN_IPU2_CAB1TO2: u32 = 11;
pub const CLK_IPU_CONN_CAB3TO3: u32 = 12;
pub const CLK_IPU_CONN_CAB2TO1: u32 = 13;
pub const CLK_IPU_CONN_CAB3TO1_SLICE: u32 = 14;
pub const CLK_IPU_CONN_NR_CLK: u32 = 15;

/* IPU_ADL */
pub const CLK_IPU_ADL_CABGEN: u32 = 0;
pub const CLK_IPU_ADL_NR_CLK: u32 = 1;

/* IPU_CORE0 */
pub const CLK_IPU_CORE0_JTAG: u32 = 0;
pub const CLK_IPU_CORE0_AXI: u32 = 1;
pub const CLK_IPU_CORE0_IPU: u32 = 2;
pub const CLK_IPU_CORE0_NR_CLK: u32 = 3;

/* IPU_CORE1 */
pub const CLK_IPU_CORE1_JTAG: u32 = 0;
pub const CLK_IPU_CORE1_AXI: u32 = 1;
pub const CLK_IPU_CORE1_IPU: u32 = 2;
pub const CLK_IPU_CORE1_NR_CLK: u32 = 3;

/* MCUCFG */
pub const CLK_MCU_MP0_SEL: u32 = 0;
pub const CLK_MCU_MP2_SEL: u32 = 1;
pub const CLK_MCU_BUS_SEL: u32 = 2;
pub const CLK_MCU_NR_CLK: u32 = 3;

// End of header guard /* _DT_BINDINGS_CLK_MT8183_H */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
