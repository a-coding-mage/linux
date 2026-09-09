/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: James Liao <jamesjj.liao@mediatek.com>
 */

/* TOPCKGEN */

pub const CLK_TOP_CLKPH_MCK_O: u32 = 1;
pub const CLK_TOP_USB_SYSPLL_125M: u32 = 3;
pub const CLK_TOP_HDMITX_DIG_CTS: u32 = 4;
pub const CLK_TOP_ARMCA7PLL_754M: u32 = 5;
pub const CLK_TOP_ARMCA7PLL_502M: u32 = 6;
pub const CLK_TOP_MAIN_H546M: u32 = 7;
pub const CLK_TOP_MAIN_H364M: u32 = 8;
pub const CLK_TOP_MAIN_H218P4M: u32 = 9;
pub const CLK_TOP_MAIN_H156M: u32 = 10;
pub const CLK_TOP_TVDPLL_445P5M: u32 = 11;
pub const CLK_TOP_TVDPLL_594M: u32 = 12;
pub const CLK_TOP_UNIV_624M: u32 = 13;
pub const CLK_TOP_UNIV_416M: u32 = 14;
pub const CLK_TOP_UNIV_249P6M: u32 = 15;
pub const CLK_TOP_UNIV_178P3M: u32 = 16;
pub const CLK_TOP_UNIV_48M: u32 = 17;
pub const CLK_TOP_CLKRTC_EXT: u32 = 18;
pub const CLK_TOP_CLKRTC_INT: u32 = 19;
pub const CLK_TOP_FPC: u32 = 20;
pub const CLK_TOP_HDMITXPLL_D2: u32 = 21;
pub const CLK_TOP_HDMITXPLL_D3: u32 = 22;
pub const CLK_TOP_ARMCA7PLL_D2: u32 = 23;
pub const CLK_TOP_ARMCA7PLL_D3: u32 = 24;
pub const CLK_TOP_APLL1: u32 = 25;
pub const CLK_TOP_APLL2: u32 = 26;
pub const CLK_TOP_DMPLL: u32 = 27;
pub const CLK_TOP_DMPLL_D2: u32 = 28;
pub const CLK_TOP_DMPLL_D4: u32 = 29;
pub const CLK_TOP_DMPLL_D8: u32 = 30;
pub const CLK_TOP_DMPLL_D16: u32 = 31;
pub const CLK_TOP_LVDSPLL_D2: u32 = 32;
pub const CLK_TOP_LVDSPLL_D4: u32 = 33;
pub const CLK_TOP_LVDSPLL_D8: u32 = 34;
pub const CLK_TOP_MMPLL: u32 = 35;
pub const CLK_TOP_MMPLL_D2: u32 = 36;
pub const CLK_TOP_MSDCPLL: u32 = 37;
pub const CLK_TOP_MSDCPLL_D2: u32 = 38;
pub const CLK_TOP_MSDCPLL_D4: u32 = 39;
pub const CLK_TOP_MSDCPLL2: u32 = 40;
pub const CLK_TOP_MSDCPLL2_D2: u32 = 41;
pub const CLK_TOP_MSDCPLL2_D4: u32 = 42;
pub const CLK_TOP_SYSPLL_D2: u32 = 43;
pub const CLK_TOP_SYSPLL1_D2: u32 = 44;
pub const CLK_TOP_SYSPLL1_D4: u32 = 45;
pub const CLK_TOP_SYSPLL1_D8: u32 = 46;
pub const CLK_TOP_SYSPLL1_D16: u32 = 47;
pub const CLK_TOP_SYSPLL_D3: u32 = 48;
pub const CLK_TOP_SYSPLL2_D2: u32 = 49;
pub const CLK_TOP_SYSPLL2_D4: u32 = 50;
pub const CLK_TOP_SYSPLL_D5: u32 = 51;
pub const CLK_TOP_SYSPLL3_D2: u32 = 52;
pub const CLK_TOP_SYSPLL3_D4: u32 = 53;
pub const CLK_TOP_SYSPLL_D7: u32 = 54;
pub const CLK_TOP_SYSPLL4_D2: u32 = 55;
pub const CLK_TOP_SYSPLL4_D4: u32 = 56;
pub const CLK_TOP_TVDPLL: u32 = 57;
pub const CLK_TOP_TVDPLL_D2: u32 = 58;
pub const CLK_TOP_TVDPLL_D4: u32 = 59;
pub const CLK_TOP_TVDPLL_D8: u32 = 60;
pub const CLK_TOP_TVDPLL_D16: u32 = 61;
pub const CLK_TOP_UNIVPLL_D2: u32 = 62;
pub const CLK_TOP_UNIVPLL1_D2: u32 = 63;
pub const CLK_TOP_UNIVPLL1_D4: u32 = 64;
pub const CLK_TOP_UNIVPLL1_D8: u32 = 65;
pub const CLK_TOP_UNIVPLL_D3: u32 = 66;
pub const CLK_TOP_UNIVPLL2_D2: u32 = 67;
pub const CLK_TOP_UNIVPLL2_D4: u32 = 68;
pub const CLK_TOP_UNIVPLL2_D8: u32 = 69;
pub const CLK_TOP_UNIVPLL_D5: u32 = 70;
pub const CLK_TOP_UNIVPLL3_D2: u32 = 71;
pub const CLK_TOP_UNIVPLL3_D4: u32 = 72;
pub const CLK_TOP_UNIVPLL3_D8: u32 = 73;
pub const CLK_TOP_UNIVPLL_D7: u32 = 74;
pub const CLK_TOP_UNIVPLL_D26: u32 = 75;
pub const CLK_TOP_UNIVPLL_D52: u32 = 76;
pub const CLK_TOP_VCODECPLL: u32 = 77;
pub const CLK_TOP_VCODECPLL_370P5: u32 = 78;
pub const CLK_TOP_VENCPLL: u32 = 79;
pub const CLK_TOP_VENCPLL_D2: u32 = 80;
pub const CLK_TOP_VENCPLL_D4: u32 = 81;
pub const CLK_TOP_AXI_SEL: u32 = 82;
pub const CLK_TOP_MEM_SEL: u32 = 83;
pub const CLK_TOP_DDRPHYCFG_SEL: u32 = 84;
pub const CLK_TOP_MM_SEL: u32 = 85;
pub const CLK_TOP_PWM_SEL: u32 = 86;
pub const CLK_TOP_VDEC_SEL: u32 = 87;
pub const CLK_TOP_VENC_SEL: u32 = 88;
pub const CLK_TOP_MFG_SEL: u32 = 89;
pub const CLK_TOP_CAMTG_SEL: u32 = 90;
pub const CLK_TOP_UART_SEL: u32 = 91;
pub const CLK_TOP_SPI_SEL: u32 = 92;
pub const CLK_TOP_USB20_SEL: u32 = 93;
pub const CLK_TOP_USB30_SEL: u32 = 94;
pub const CLK_TOP_MSDC50_0_H_SEL: u32 = 95;
pub const CLK_TOP_MSDC50_0_SEL: u32 = 96;
pub const CLK_TOP_MSDC30_1_SEL: u32 = 97;
pub const CLK_TOP_MSDC30_2_SEL: u32 = 98;
pub const CLK_TOP_MSDC30_3_SEL: u32 = 99;
pub const CLK_TOP_AUDIO_SEL: u32 = 100;
pub const CLK_TOP_AUD_INTBUS_SEL: u32 = 101;
pub const CLK_TOP_PMICSPI_SEL: u32 = 102;
pub const CLK_TOP_SCP_SEL: u32 = 103;
pub const CLK_TOP_ATB_SEL: u32 = 104;
pub const CLK_TOP_VENC_LT_SEL: u32 = 105;
pub const CLK_TOP_DPI0_SEL: u32 = 106;
pub const CLK_TOP_IRDA_SEL: u32 = 107;
pub const CLK_TOP_CCI400_SEL: u32 = 108;
pub const CLK_TOP_AUD_1_SEL: u32 = 109;
pub const CLK_TOP_AUD_2_SEL: u32 = 110;
pub const CLK_TOP_MEM_MFG_IN_SEL: u32 = 111;
pub const CLK_TOP_AXI_MFG_IN_SEL: u32 = 112;
pub const CLK_TOP_SCAM_SEL: u32 = 113;
pub const CLK_TOP_SPINFI_IFR_SEL: u32 = 114;
pub const CLK_TOP_HDMI_SEL: u32 = 115;
pub const CLK_TOP_DPILVDS_SEL: u32 = 116;
pub const CLK_TOP_MSDC50_2_H_SEL: u32 = 117;
pub const CLK_TOP_HDCP_SEL: u32 = 118;
pub const CLK_TOP_HDCP_24M_SEL: u32 = 119;
pub const CLK_TOP_RTC_SEL: u32 = 120;
pub const CLK_TOP_APLL1_DIV0: u32 = 121;
pub const CLK_TOP_APLL1_DIV1: u32 = 122;
pub const CLK_TOP_APLL1_DIV2: u32 = 123;
pub const CLK_TOP_APLL1_DIV3: u32 = 124;
pub const CLK_TOP_APLL1_DIV4: u32 = 125;
pub const CLK_TOP_APLL1_DIV5: u32 = 126;
pub const CLK_TOP_APLL2_DIV0: u32 = 127;
pub const CLK_TOP_APLL2_DIV1: u32 = 128;
pub const CLK_TOP_APLL2_DIV2: u32 = 129;
pub const CLK_TOP_APLL2_DIV3: u32 = 130;
pub const CLK_TOP_APLL2_DIV4: u32 = 131;
pub const CLK_TOP_APLL2_DIV5: u32 = 132;
pub const CLK_TOP_I2S0_M_SEL: u32 = 133;
pub const CLK_TOP_I2S1_M_SEL: u32 = 134;
pub const CLK_TOP_I2S2_M_SEL: u32 = 135;
pub const CLK_TOP_I2S3_M_SEL: u32 = 136;
pub const CLK_TOP_I2S3_B_SEL: u32 = 137;
pub const CLK_TOP_DSI0_DIG: u32 = 138;
pub const CLK_TOP_DSI1_DIG: u32 = 139;
pub const CLK_TOP_LVDS_PXL: u32 = 140;
pub const CLK_TOP_LVDS_CTS: u32 = 141;
pub const CLK_TOP_NR_CLK: u32 = 142;

/* APMIXED_SYS */

pub const CLK_APMIXED_ARMCA15PLL: u32 = 1;
pub const CLK_APMIXED_ARMCA7PLL: u32 = 2;
pub const CLK_APMIXED_MAINPLL: u32 = 3;
pub const CLK_APMIXED_UNIVPLL: u32 = 4;
pub const CLK_APMIXED_MMPLL: u32 = 5;
pub const CLK_APMIXED_MSDCPLL: u32 = 6;
pub const CLK_APMIXED_VENCPLL: u32 = 7;
pub const CLK_APMIXED_TVDPLL: u32 = 8;
pub const CLK_APMIXED_MPLL: u32 = 9;
pub const CLK_APMIXED_VCODECPLL: u32 = 10;
pub const CLK_APMIXED_APLL1: u32 = 11;
pub const CLK_APMIXED_APLL2: u32 = 12;
pub const CLK_APMIXED_LVDSPLL: u32 = 13;
pub const CLK_APMIXED_MSDCPLL2: u32 = 14;
pub const CLK_APMIXED_REF2USB_TX: u32 = 15;
pub const CLK_APMIXED_HDMI_REF: u32 = 16;
pub const CLK_APMIXED_NR_CLK: u32 = 17;

/* INFRA_SYS */

pub const CLK_INFRA_DBGCLK: u32 = 1;
pub const CLK_INFRA_SMI: u32 = 2;
pub const CLK_INFRA_AUDIO: u32 = 3;
pub const CLK_INFRA_GCE: u32 = 4;
pub const CLK_INFRA_L2C_SRAM: u32 = 5;
pub const CLK_INFRA_M4U: u32 = 6;
pub const CLK_INFRA_CPUM: u32 = 7;
pub const CLK_INFRA_KP: u32 = 8;
pub const CLK_INFRA_CEC: u32 = 9;
pub const CLK_INFRA_PMICSPI: u32 = 10;
pub const CLK_INFRA_PMICWRAP: u32 = 11;
pub const CLK_INFRA_CLK_13M: u32 = 12;
pub const CLK_INFRA_CA53SEL: u32 = 13;
pub const CLK_INFRA_CA72SEL: u32 = 14;
pub const CLK_INFRA_NR_CLK: u32 = 15;

/* PERI_SYS */

pub const CLK_PERI_NFI: u32 = 1;
pub const CLK_PERI_THERM: u32 = 2;
pub const CLK_PERI_PWM1: u32 = 3;
pub const CLK_PERI_PWM2: u32 = 4;
pub const CLK_PERI_PWM3: u32 = 5;
pub const CLK_PERI_PWM4: u32 = 6;
pub const CLK_PERI_PWM5: u32 = 7;
pub const CLK_PERI_PWM6: u32 = 8;
pub const CLK_PERI_PWM7: u32 = 9;
pub const CLK_PERI_PWM: u32 = 10;
pub const CLK_PERI_USB0: u32 = 11;
pub const CLK_PERI_USB1: u32 = 12;
pub const CLK_PERI_AP_DMA: u32 = 13;
pub const CLK_PERI_MSDC30_0: u32 = 14;
pub const CLK_PERI_MSDC30_1: u32 = 15;
pub const CLK_PERI_MSDC30_2: u32 = 16;
pub const CLK_PERI_MSDC30_3: u32 = 17;
pub const CLK_PERI_NLI_ARB: u32 = 18;
pub const CLK_PERI_IRDA: u32 = 19;
pub const CLK_PERI_UART0: u32 = 20;
pub const CLK_PERI_UART1: u32 = 21;
pub const CLK_PERI_UART2: u32 = 22;
pub const CLK_PERI_UART3: u32 = 23;
pub const CLK_PERI_I2C0: u32 = 24;
pub const CLK_PERI_I2C1: u32 = 25;
pub const CLK_PERI_I2C2: u32 = 26;
pub const CLK_PERI_I2C3: u32 = 27;
pub const CLK_PERI_I2C4: u32 = 28;
pub const CLK_PERI_AUXADC: u32 = 29;
pub const CLK_PERI_SPI0: u32 = 30;
pub const CLK_PERI_I2C5: u32 = 31;
pub const CLK_PERI_NFIECC: u32 = 32;
pub const CLK_PERI_SPI: u32 = 33;
pub const CLK_PERI_IRRX: u32 = 34;
pub const CLK_PERI_I2C6: u32 = 35;
pub const CLK_PERI_UART0_SEL: u32 = 36;
pub const CLK_PERI_UART1_SEL: u32 = 37;
pub const CLK_PERI_UART2_SEL: u32 = 38;
pub const CLK_PERI_UART3_SEL: u32 = 39;
pub const CLK_PERI_NR_CLK: u32 = 40;

/* IMG_SYS */

pub const CLK_IMG_LARB2_SMI: u32 = 1;
pub const CLK_IMG_CAM_SMI: u32 = 2;
pub const CLK_IMG_CAM_CAM: u32 = 3;
pub const CLK_IMG_SEN_TG: u32 = 4;
pub const CLK_IMG_SEN_CAM: u32 = 5;
pub const CLK_IMG_CAM_SV: u32 = 6;
pub const CLK_IMG_FD: u32 = 7;
pub const CLK_IMG_NR_CLK: u32 = 8;

/* MFG_SYS */

pub const CLK_MFG_AXI: u32 = 0;
pub const CLK_MFG_MEM: u32 = 1;
pub const CLK_MFG_G3D: u32 = 2;
pub const CLK_MFG_26M: u32 = 3;

/* MM_SYS */

pub const CLK_MM_SMI_COMMON: u32 = 1;
pub const CLK_MM_SMI_LARB0: u32 = 2;
pub const CLK_MM_CAM_MDP: u32 = 3;
pub const CLK_MM_MDP_RDMA0: u32 = 4;
pub const CLK_MM_MDP_RDMA1: u32 = 5;
pub const CLK_MM_MDP_RSZ0: u32 = 6;
pub const CLK_MM_MDP_RSZ1: u32 = 7;
pub const CLK_MM_MDP_RSZ2: u32 = 8;
pub const CLK_MM_MDP_TDSHP0: u32 = 9;
pub const CLK_MM_MDP_TDSHP1: u32 = 10;
pub const CLK_MM_MDP_WDMA: u32 = 11;
pub const CLK_MM_MDP_WROT0: u32 = 12;
pub const CLK_MM_MDP_WROT1: u32 = 13;
pub const CLK_MM_FAKE_ENG: u32 = 14;
pub const CLK_MM_MUTEX_32K: u32 = 15;
pub const CLK_MM_DISP_OVL0: u32 = 16;
pub const CLK_MM_DISP_OVL1: u32 = 17;
pub const CLK_MM_DISP_RDMA0: u32 = 18;
pub const CLK_MM_DISP_RDMA1: u32 = 19;
pub const CLK_MM_DISP_RDMA2: u32 = 20;
pub const CLK_MM_DISP_WDMA0: u32 = 21;
pub const CLK_MM_DISP_WDMA1: u32 = 22;
pub const CLK_MM_DISP_COLOR0: u32 = 23;
pub const CLK_MM_DISP_COLOR1: u32 = 24;
pub const CLK_MM_DISP_AAL: u32 = 25;
pub const CLK_MM_DISP_GAMMA: u32 = 26;
pub const CLK_MM_DISP_UFOE: u32 = 27;
pub const CLK_MM_DISP_SPLIT0: u32 = 28;
pub const CLK_MM_DISP_SPLIT1: u32 = 29;
pub const CLK_MM_DISP_MERGE: u32 = 30;
pub const CLK_MM_DISP_OD: u32 = 31;
pub const CLK_MM_DISP_PWM0MM: u32 = 32;
pub const CLK_MM_DISP_PWM026M: u32 = 33;
pub const CLK_MM_DISP_PWM1MM: u32 = 34;
pub const CLK_MM_DISP_PWM126M: u32 = 35;
pub const CLK_MM_DSI0_ENGINE: u32 = 36;
pub const CLK_MM_DSI0_DIGITAL: u32 = 37;
pub const CLK_MM_DSI1_ENGINE: u32 = 38;
pub const CLK_MM_DSI1_DIGITAL: u32 = 39;
pub const CLK_MM_DPI_PIXEL: u32 = 40;
pub const CLK_MM_DPI_ENGINE: u32 = 41;
pub const CLK_MM_DPI1_PIXEL: u32 = 42;
pub const CLK_MM_DPI1_ENGINE: u32 = 43;
pub const CLK_MM_HDMI_PIXEL: u32 = 44;
pub const CLK_MM_HDMI_PLLCK: u32 = 45;
pub const CLK_MM_HDMI_AUDIO: u32 = 46;
pub const CLK_MM_HDMI_SPDIF: u32 = 47;
pub const CLK_MM_LVDS_PIXEL: u32 = 48;
pub const CLK_MM_LVDS_CTS: u32 = 49;
pub const CLK_MM_SMI_LARB4: u32 = 50;
pub const CLK_MM_HDMI_HDCP: u32 = 51;
pub const CLK_MM_HDMI_HDCP24M: u32 = 52;
pub const CLK_MM_NR_CLK: u32 = 53;

/* VDEC_SYS */

pub const CLK_VDEC_CKEN: u32 = 1;
pub const CLK_VDEC_LARB_CKEN: u32 = 2;
pub const CLK_VDEC_NR_CLK: u32 = 3;

/* VENC_SYS */

pub const CLK_VENC_CKE0: u32 = 1;
pub const CLK_VENC_CKE1: u32 = 2;
pub const CLK_VENC_CKE2: u32 = 3;
pub const CLK_VENC_CKE3: u32 = 4;
pub const CLK_VENC_NR_CLK: u32 = 5;

/* VENCLT_SYS */

pub const CLK_VENCLT_CKE0: u32 = 1;
pub const CLK_VENCLT_CKE1: u32 = 2;
pub const CLK_VENCLT_NR_CLK: u32 = 3;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
