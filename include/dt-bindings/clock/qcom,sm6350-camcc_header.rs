/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2022, The Linux Foundation. All rights reserved.
 * Copyright (c) 2022, Linaro Limited
 */

// C header guard: _DT_BINDINGS_CLK_QCOM_CAMCC_SM6350_H

/* CAMCC clocks */
pub const CAMCC_PLL2_OUT_EARLY: u32 = 0;
pub const CAMCC_PLL0: u32 = 1;
pub const CAMCC_PLL0_OUT_EVEN: u32 = 2;
pub const CAMCC_PLL1: u32 = 3;
pub const CAMCC_PLL1_OUT_EVEN: u32 = 4;
pub const CAMCC_PLL2: u32 = 5;
pub const CAMCC_PLL2_OUT_MAIN: u32 = 6;
pub const CAMCC_PLL3: u32 = 7;
pub const CAMCC_BPS_AHB_CLK: u32 = 8;
pub const CAMCC_BPS_AREG_CLK: u32 = 9;
pub const CAMCC_BPS_AXI_CLK: u32 = 10;
pub const CAMCC_BPS_CLK: u32 = 11;
pub const CAMCC_BPS_CLK_SRC: u32 = 12;
pub const CAMCC_CAMNOC_ATB_CLK: u32 = 13;
pub const CAMCC_CAMNOC_AXI_CLK: u32 = 14;
pub const CAMCC_CCI_0_CLK: u32 = 15;
pub const CAMCC_CCI_0_CLK_SRC: u32 = 16;
pub const CAMCC_CCI_1_CLK: u32 = 17;
pub const CAMCC_CCI_1_CLK_SRC: u32 = 18;
pub const CAMCC_CORE_AHB_CLK: u32 = 19;
pub const CAMCC_CPAS_AHB_CLK: u32 = 20;
pub const CAMCC_CPHY_RX_CLK_SRC: u32 = 21;
pub const CAMCC_CSI0PHYTIMER_CLK: u32 = 22;
pub const CAMCC_CSI0PHYTIMER_CLK_SRC: u32 = 23;
pub const CAMCC_CSI1PHYTIMER_CLK: u32 = 24;
pub const CAMCC_CSI1PHYTIMER_CLK_SRC: u32 = 25;
pub const CAMCC_CSI2PHYTIMER_CLK: u32 = 26;
pub const CAMCC_CSI2PHYTIMER_CLK_SRC: u32 = 27;
pub const CAMCC_CSI3PHYTIMER_CLK: u32 = 28;
pub const CAMCC_CSI3PHYTIMER_CLK_SRC: u32 = 29;
pub const CAMCC_CSIPHY0_CLK: u32 = 30;
pub const CAMCC_CSIPHY1_CLK: u32 = 31;
pub const CAMCC_CSIPHY2_CLK: u32 = 32;
pub const CAMCC_CSIPHY3_CLK: u32 = 33;
pub const CAMCC_FAST_AHB_CLK_SRC: u32 = 34;
pub const CAMCC_ICP_APB_CLK: u32 = 35;
pub const CAMCC_ICP_ATB_CLK: u32 = 36;
pub const CAMCC_ICP_CLK: u32 = 37;
pub const CAMCC_ICP_CLK_SRC: u32 = 38;
pub const CAMCC_ICP_CTI_CLK: u32 = 39;
pub const CAMCC_ICP_TS_CLK: u32 = 40;
pub const CAMCC_IFE_0_AXI_CLK: u32 = 41;
pub const CAMCC_IFE_0_CLK: u32 = 42;
pub const CAMCC_IFE_0_CLK_SRC: u32 = 43;
pub const CAMCC_IFE_0_CPHY_RX_CLK: u32 = 44;
pub const CAMCC_IFE_0_CSID_CLK: u32 = 45;
pub const CAMCC_IFE_0_CSID_CLK_SRC: u32 = 46;
pub const CAMCC_IFE_0_DSP_CLK: u32 = 47;
pub const CAMCC_IFE_1_AXI_CLK: u32 = 48;
pub const CAMCC_IFE_1_CLK: u32 = 49;
pub const CAMCC_IFE_1_CLK_SRC: u32 = 50;
pub const CAMCC_IFE_1_CPHY_RX_CLK: u32 = 51;
pub const CAMCC_IFE_1_CSID_CLK: u32 = 52;
pub const CAMCC_IFE_1_CSID_CLK_SRC: u32 = 53;
pub const CAMCC_IFE_1_DSP_CLK: u32 = 54;
pub const CAMCC_IFE_2_AXI_CLK: u32 = 55;
pub const CAMCC_IFE_2_CLK: u32 = 56;
pub const CAMCC_IFE_2_CLK_SRC: u32 = 57;
pub const CAMCC_IFE_2_CPHY_RX_CLK: u32 = 58;
pub const CAMCC_IFE_2_CSID_CLK: u32 = 59;
pub const CAMCC_IFE_2_CSID_CLK_SRC: u32 = 60;
pub const CAMCC_IFE_2_DSP_CLK: u32 = 61;
pub const CAMCC_IFE_LITE_CLK: u32 = 62;
pub const CAMCC_IFE_LITE_CLK_SRC: u32 = 63;
pub const CAMCC_IFE_LITE_CPHY_RX_CLK: u32 = 64;
pub const CAMCC_IFE_LITE_CSID_CLK: u32 = 65;
pub const CAMCC_IFE_LITE_CSID_CLK_SRC: u32 = 66;
pub const CAMCC_IPE_0_AHB_CLK: u32 = 67;
pub const CAMCC_IPE_0_AREG_CLK: u32 = 68;
pub const CAMCC_IPE_0_AXI_CLK: u32 = 69;
pub const CAMCC_IPE_0_CLK: u32 = 70;
pub const CAMCC_IPE_0_CLK_SRC: u32 = 71;
pub const CAMCC_JPEG_CLK: u32 = 72;
pub const CAMCC_JPEG_CLK_SRC: u32 = 73;
pub const CAMCC_LRME_CLK: u32 = 74;
pub const CAMCC_LRME_CLK_SRC: u32 = 75;
pub const CAMCC_MCLK0_CLK: u32 = 76;
pub const CAMCC_MCLK0_CLK_SRC: u32 = 77;
pub const CAMCC_MCLK1_CLK: u32 = 78;
pub const CAMCC_MCLK1_CLK_SRC: u32 = 79;
pub const CAMCC_MCLK2_CLK: u32 = 80;
pub const CAMCC_MCLK2_CLK_SRC: u32 = 81;
pub const CAMCC_MCLK3_CLK: u32 = 82;
pub const CAMCC_MCLK3_CLK_SRC: u32 = 83;
pub const CAMCC_MCLK4_CLK: u32 = 84;
pub const CAMCC_MCLK4_CLK_SRC: u32 = 85;
pub const CAMCC_SLOW_AHB_CLK_SRC: u32 = 86;
pub const CAMCC_SOC_AHB_CLK: u32 = 87;
pub const CAMCC_SYS_TMR_CLK: u32 = 88;

/* GDSCs */
pub const BPS_GDSC: u32 = 0;
pub const IPE_0_GDSC: u32 = 1;
pub const IFE_0_GDSC: u32 = 2;
pub const IFE_1_GDSC: u32 = 3;
pub const IFE_2_GDSC: u32 = 4;
pub const TITAN_TOP_GDSC: u32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
