/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2020 MediaTek Inc.
 * Copyright (c) 2020 BayLibre, SAS.
 * Author: James Liao <jamesjj.liao@mediatek.com>
 *         Fabien Parent <fparent@baylibre.com>
 */

/* MT8167 is based on MT8516. */
// Dependency: declarations from <dt-bindings/clock/mt8516-clk.h>.

/* APMIXEDSYS */

pub const CLK_APMIXED_TVDPLL: i32 = CLK_APMIXED_NR_CLK + 0;
pub const CLK_APMIXED_LVDSPLL: i32 = CLK_APMIXED_NR_CLK + 1;
pub const CLK_APMIXED_HDMI_REF: i32 = CLK_APMIXED_NR_CLK + 2;
pub const MT8167_CLK_APMIXED_NR_CLK: i32 = CLK_APMIXED_NR_CLK + 3;

/* TOPCKGEN */

pub const CLK_TOP_DSI0_LNTC_DSICK: i32 = CLK_TOP_NR_CLK + 0;
pub const CLK_TOP_VPLL_DPIX: i32 = CLK_TOP_NR_CLK + 1;
pub const CLK_TOP_LVDSTX_CLKDIG_CTS: i32 = CLK_TOP_NR_CLK + 2;
pub const CLK_TOP_HDMTX_CLKDIG_CTS: i32 = CLK_TOP_NR_CLK + 3;
pub const CLK_TOP_LVDSPLL: i32 = CLK_TOP_NR_CLK + 4;
pub const CLK_TOP_LVDSPLL_D2: i32 = CLK_TOP_NR_CLK + 5;
pub const CLK_TOP_LVDSPLL_D4: i32 = CLK_TOP_NR_CLK + 6;
pub const CLK_TOP_LVDSPLL_D8: i32 = CLK_TOP_NR_CLK + 7;
pub const CLK_TOP_MIPI_26M: i32 = CLK_TOP_NR_CLK + 8;
pub const CLK_TOP_TVDPLL: i32 = CLK_TOP_NR_CLK + 9;
pub const CLK_TOP_TVDPLL_D2: i32 = CLK_TOP_NR_CLK + 10;
pub const CLK_TOP_TVDPLL_D4: i32 = CLK_TOP_NR_CLK + 11;
pub const CLK_TOP_TVDPLL_D8: i32 = CLK_TOP_NR_CLK + 12;
pub const CLK_TOP_TVDPLL_D16: i32 = CLK_TOP_NR_CLK + 13;
pub const CLK_TOP_PWM_MM: i32 = CLK_TOP_NR_CLK + 14;
pub const CLK_TOP_CAM_MM: i32 = CLK_TOP_NR_CLK + 15;
pub const CLK_TOP_MFG_MM: i32 = CLK_TOP_NR_CLK + 16;
pub const CLK_TOP_SPM_52M: i32 = CLK_TOP_NR_CLK + 17;
pub const CLK_TOP_MIPI_26M_DBG: i32 = CLK_TOP_NR_CLK + 18;
pub const CLK_TOP_SCAM_MM: i32 = CLK_TOP_NR_CLK + 19;
pub const CLK_TOP_SMI_MM: i32 = CLK_TOP_NR_CLK + 20;
pub const CLK_TOP_26M_HDMI_SIFM: i32 = CLK_TOP_NR_CLK + 21;
pub const CLK_TOP_26M_CEC: i32 = CLK_TOP_NR_CLK + 22;
pub const CLK_TOP_32K_CEC: i32 = CLK_TOP_NR_CLK + 23;
pub const CLK_TOP_GCPU_B: i32 = CLK_TOP_NR_CLK + 24;
pub const CLK_TOP_RG_VDEC: i32 = CLK_TOP_NR_CLK + 25;
pub const CLK_TOP_RG_FDPI0: i32 = CLK_TOP_NR_CLK + 26;
pub const CLK_TOP_RG_FDPI1: i32 = CLK_TOP_NR_CLK + 27;
pub const CLK_TOP_RG_AXI_MFG: i32 = CLK_TOP_NR_CLK + 28;
pub const CLK_TOP_RG_SLOW_MFG: i32 = CLK_TOP_NR_CLK + 29;
pub const CLK_TOP_GFMUX_EMI1X_SEL: i32 = CLK_TOP_NR_CLK + 30;
pub const CLK_TOP_CSW_MUX_MFG_SEL: i32 = CLK_TOP_NR_CLK + 31;
pub const CLK_TOP_CAMTG_MM_SEL: i32 = CLK_TOP_NR_CLK + 32;
pub const CLK_TOP_PWM_MM_SEL: i32 = CLK_TOP_NR_CLK + 33;
pub const CLK_TOP_SPM_52M_SEL: i32 = CLK_TOP_NR_CLK + 34;
pub const CLK_TOP_MFG_MM_SEL: i32 = CLK_TOP_NR_CLK + 35;
pub const CLK_TOP_SMI_MM_SEL: i32 = CLK_TOP_NR_CLK + 36;
pub const CLK_TOP_SCAM_MM_SEL: i32 = CLK_TOP_NR_CLK + 37;
pub const CLK_TOP_VDEC_MM_SEL: i32 = CLK_TOP_NR_CLK + 38;
pub const CLK_TOP_DPI0_MM_SEL: i32 = CLK_TOP_NR_CLK + 39;
pub const CLK_TOP_DPI1_MM_SEL: i32 = CLK_TOP_NR_CLK + 40;
pub const CLK_TOP_AXI_MFG_IN_SEL: i32 = CLK_TOP_NR_CLK + 41;
pub const CLK_TOP_SLOW_MFG_SEL: i32 = CLK_TOP_NR_CLK + 42;
pub const MT8167_CLK_TOP_NR_CLK: i32 = CLK_TOP_NR_CLK + 43;

/* MFGCFG */

pub const CLK_MFG_BAXI: i32 = 0;
pub const CLK_MFG_BMEM: i32 = 1;
pub const CLK_MFG_BG3D: i32 = 2;
pub const CLK_MFG_B26M: i32 = 3;
pub const CLK_MFG_NR_CLK: i32 = 4;

/* MMSYS */

pub const CLK_MM_SMI_COMMON: i32 = 0;
pub const CLK_MM_SMI_LARB0: i32 = 1;
pub const CLK_MM_CAM_MDP: i32 = 2;
pub const CLK_MM_MDP_RDMA: i32 = 3;
pub const CLK_MM_MDP_RSZ0: i32 = 4;
pub const CLK_MM_MDP_RSZ1: i32 = 5;
pub const CLK_MM_MDP_TDSHP: i32 = 6;
pub const CLK_MM_MDP_WDMA: i32 = 7;
pub const CLK_MM_MDP_WROT: i32 = 8;
pub const CLK_MM_FAKE_ENG: i32 = 9;
pub const CLK_MM_DISP_OVL0: i32 = 10;
pub const CLK_MM_DISP_RDMA0: i32 = 11;
pub const CLK_MM_DISP_RDMA1: i32 = 12;
pub const CLK_MM_DISP_WDMA: i32 = 13;
pub const CLK_MM_DISP_COLOR: i32 = 14;
pub const CLK_MM_DISP_CCORR: i32 = 15;
pub const CLK_MM_DISP_AAL: i32 = 16;
pub const CLK_MM_DISP_GAMMA: i32 = 17;
pub const CLK_MM_DISP_DITHER: i32 = 18;
pub const CLK_MM_DISP_UFOE: i32 = 19;
pub const CLK_MM_DISP_PWM_MM: i32 = 20;
pub const CLK_MM_DISP_PWM_26M: i32 = 21;
pub const CLK_MM_DSI_ENGINE: i32 = 22;
pub const CLK_MM_DSI_DIGITAL: i32 = 23;
pub const CLK_MM_DPI0_ENGINE: i32 = 24;
pub const CLK_MM_DPI0_PXL: i32 = 25;
pub const CLK_MM_LVDS_PXL: i32 = 26;
pub const CLK_MM_LVDS_CTS: i32 = 27;
pub const CLK_MM_DPI1_ENGINE: i32 = 28;
pub const CLK_MM_DPI1_PXL: i32 = 29;
pub const CLK_MM_HDMI_PXL: i32 = 30;
pub const CLK_MM_HDMI_SPDIF: i32 = 31;
pub const CLK_MM_HDMI_ADSP_BCK: i32 = 32;
pub const CLK_MM_HDMI_PLL: i32 = 33;
pub const CLK_MM_NR_CLK: i32 = 34;

/* IMGSYS */

pub const CLK_IMG_LARB1_SMI: i32 = 0;
pub const CLK_IMG_CAM_SMI: i32 = 1;
pub const CLK_IMG_CAM_CAM: i32 = 2;
pub const CLK_IMG_SEN_TG: i32 = 3;
pub const CLK_IMG_SEN_CAM: i32 = 4;
pub const CLK_IMG_VENC: i32 = 5;
pub const CLK_IMG_NR_CLK: i32 = 6;

/* VDECSYS */

pub const CLK_VDEC_CKEN: i32 = 0;
pub const CLK_VDEC_LARB1_CKEN: i32 = 1;
pub const CLK_VDEC_NR_CLK: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
