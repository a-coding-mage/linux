/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2025, Luca Weiss <luca.weiss@fairphone.com>
 */

/* CAM_CC clocks */
pub const CAM_CC_PLL0: u32 = 0;
pub const CAM_CC_PLL0_OUT_EVEN: u32 = 1;
pub const CAM_CC_PLL0_OUT_ODD: u32 = 2;
pub const CAM_CC_PLL1: u32 = 3;
pub const CAM_CC_PLL1_OUT_EVEN: u32 = 4;
pub const CAM_CC_PLL2: u32 = 5;
pub const CAM_CC_PLL2_OUT_EVEN: u32 = 6;
pub const CAM_CC_PLL3: u32 = 7;
pub const CAM_CC_PLL3_OUT_EVEN: u32 = 8;
pub const CAM_CC_PLL4: u32 = 9;
pub const CAM_CC_PLL4_OUT_EVEN: u32 = 10;
pub const CAM_CC_PLL5: u32 = 11;
pub const CAM_CC_PLL5_OUT_EVEN: u32 = 12;
pub const CAM_CC_PLL6: u32 = 13;
pub const CAM_CC_PLL6_OUT_EVEN: u32 = 14;
pub const CAM_CC_BPS_AHB_CLK: u32 = 15;
pub const CAM_CC_BPS_AREG_CLK: u32 = 16;
pub const CAM_CC_BPS_CLK: u32 = 17;
pub const CAM_CC_BPS_CLK_SRC: u32 = 18;
pub const CAM_CC_CAMNOC_ATB_CLK: u32 = 19;
pub const CAM_CC_CAMNOC_AXI_CLK_SRC: u32 = 20;
pub const CAM_CC_CAMNOC_AXI_HF_CLK: u32 = 21;
pub const CAM_CC_CAMNOC_AXI_SF_CLK: u32 = 22;
pub const CAM_CC_CAMNOC_NRT_AXI_CLK: u32 = 23;
pub const CAM_CC_CAMNOC_RT_AXI_CLK: u32 = 24;
pub const CAM_CC_CCI_0_CLK: u32 = 25;
pub const CAM_CC_CCI_0_CLK_SRC: u32 = 26;
pub const CAM_CC_CCI_1_CLK: u32 = 27;
pub const CAM_CC_CCI_1_CLK_SRC: u32 = 28;
pub const CAM_CC_CORE_AHB_CLK: u32 = 29;
pub const CAM_CC_CPAS_AHB_CLK: u32 = 30;
pub const CAM_CC_CPHY_RX_CLK_SRC: u32 = 31;
pub const CAM_CC_CRE_AHB_CLK: u32 = 32;
pub const CAM_CC_CRE_CLK: u32 = 33;
pub const CAM_CC_CRE_CLK_SRC: u32 = 34;
pub const CAM_CC_CSI0PHYTIMER_CLK: u32 = 35;
pub const CAM_CC_CSI0PHYTIMER_CLK_SRC: u32 = 36;
pub const CAM_CC_CSI1PHYTIMER_CLK: u32 = 37;
pub const CAM_CC_CSI1PHYTIMER_CLK_SRC: u32 = 38;
pub const CAM_CC_CSI2PHYTIMER_CLK: u32 = 39;
pub const CAM_CC_CSI2PHYTIMER_CLK_SRC: u32 = 40;
pub const CAM_CC_CSI3PHYTIMER_CLK: u32 = 41;
pub const CAM_CC_CSI3PHYTIMER_CLK_SRC: u32 = 42;
pub const CAM_CC_CSIPHY0_CLK: u32 = 43;
pub const CAM_CC_CSIPHY1_CLK: u32 = 44;
pub const CAM_CC_CSIPHY2_CLK: u32 = 45;
pub const CAM_CC_CSIPHY3_CLK: u32 = 46;
pub const CAM_CC_FAST_AHB_CLK_SRC: u32 = 47;
pub const CAM_CC_GDSC_CLK: u32 = 48;
pub const CAM_CC_ICP_ATB_CLK: u32 = 49;
pub const CAM_CC_ICP_CLK: u32 = 50;
pub const CAM_CC_ICP_CLK_SRC: u32 = 51;
pub const CAM_CC_ICP_CTI_CLK: u32 = 52;
pub const CAM_CC_ICP_TS_CLK: u32 = 53;
pub const CAM_CC_MCLK0_CLK: u32 = 54;
pub const CAM_CC_MCLK0_CLK_SRC: u32 = 55;
pub const CAM_CC_MCLK1_CLK: u32 = 56;
pub const CAM_CC_MCLK1_CLK_SRC: u32 = 57;
pub const CAM_CC_MCLK2_CLK: u32 = 58;
pub const CAM_CC_MCLK2_CLK_SRC: u32 = 59;
pub const CAM_CC_MCLK3_CLK: u32 = 60;
pub const CAM_CC_MCLK3_CLK_SRC: u32 = 61;
pub const CAM_CC_MCLK4_CLK: u32 = 62;
pub const CAM_CC_MCLK4_CLK_SRC: u32 = 63;
pub const CAM_CC_OPE_0_AHB_CLK: u32 = 64;
pub const CAM_CC_OPE_0_AREG_CLK: u32 = 65;
pub const CAM_CC_OPE_0_CLK: u32 = 66;
pub const CAM_CC_OPE_0_CLK_SRC: u32 = 67;
pub const CAM_CC_SLEEP_CLK: u32 = 68;
pub const CAM_CC_SLEEP_CLK_SRC: u32 = 69;
pub const CAM_CC_SLOW_AHB_CLK_SRC: u32 = 70;
pub const CAM_CC_SOC_AHB_CLK: u32 = 71;
pub const CAM_CC_SYS_TMR_CLK: u32 = 72;
pub const CAM_CC_TFE_0_AHB_CLK: u32 = 73;
pub const CAM_CC_TFE_0_CLK: u32 = 74;
pub const CAM_CC_TFE_0_CLK_SRC: u32 = 75;
pub const CAM_CC_TFE_0_CPHY_RX_CLK: u32 = 76;
pub const CAM_CC_TFE_0_CSID_CLK: u32 = 77;
pub const CAM_CC_TFE_0_CSID_CLK_SRC: u32 = 78;
pub const CAM_CC_TFE_1_AHB_CLK: u32 = 79;
pub const CAM_CC_TFE_1_CLK: u32 = 80;
pub const CAM_CC_TFE_1_CLK_SRC: u32 = 81;
pub const CAM_CC_TFE_1_CPHY_RX_CLK: u32 = 82;
pub const CAM_CC_TFE_1_CSID_CLK: u32 = 83;
pub const CAM_CC_TFE_1_CSID_CLK_SRC: u32 = 84;
pub const CAM_CC_TFE_2_AHB_CLK: u32 = 85;
pub const CAM_CC_TFE_2_CLK: u32 = 86;
pub const CAM_CC_TFE_2_CLK_SRC: u32 = 87;
pub const CAM_CC_TFE_2_CPHY_RX_CLK: u32 = 88;
pub const CAM_CC_TFE_2_CSID_CLK: u32 = 89;
pub const CAM_CC_TFE_2_CSID_CLK_SRC: u32 = 90;
pub const CAM_CC_TOP_SHIFT_CLK: u32 = 91;
pub const CAM_CC_XO_CLK_SRC: u32 = 92;

/* CAM_CC resets */
pub const CAM_CC_BPS_BCR: u32 = 0;
pub const CAM_CC_CAMNOC_BCR: u32 = 1;
pub const CAM_CC_CAMSS_TOP_BCR: u32 = 2;
pub const CAM_CC_CCI_0_BCR: u32 = 3;
pub const CAM_CC_CCI_1_BCR: u32 = 4;
pub const CAM_CC_CPAS_BCR: u32 = 5;
pub const CAM_CC_CRE_BCR: u32 = 6;
pub const CAM_CC_CSI0PHY_BCR: u32 = 7;
pub const CAM_CC_CSI1PHY_BCR: u32 = 8;
pub const CAM_CC_CSI2PHY_BCR: u32 = 9;
pub const CAM_CC_CSI3PHY_BCR: u32 = 10;
pub const CAM_CC_ICP_BCR: u32 = 11;
pub const CAM_CC_MCLK0_BCR: u32 = 12;
pub const CAM_CC_MCLK1_BCR: u32 = 13;
pub const CAM_CC_MCLK2_BCR: u32 = 14;
pub const CAM_CC_MCLK3_BCR: u32 = 15;
pub const CAM_CC_MCLK4_BCR: u32 = 16;
pub const CAM_CC_OPE_0_BCR: u32 = 17;
pub const CAM_CC_TFE_0_BCR: u32 = 18;
pub const CAM_CC_TFE_1_BCR: u32 = 19;
pub const CAM_CC_TFE_2_BCR: u32 = 20;

/* CAM_CC power domains */
pub const CAM_CC_CAMSS_TOP_GDSC: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
