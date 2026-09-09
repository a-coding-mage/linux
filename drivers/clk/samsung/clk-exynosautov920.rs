#![allow(non_upper_case_globals, dead_code, unused_variables, unused_mut)]

// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2024 Samsung Electronics Co., Ltd.
 * Author: Sunyeal Hong <sunyeal.hong@samsung.com>
 *
 * Common Clock Framework support for ExynosAuto v920 SoC.
 */

// dependency: <linux/clk-provider.h>
// dependency: <linux/of.h>
// dependency: <linux/platform_device.h>

// dependency: <dt-bindings/clock/samsung,exynosautov920.h>

// dependency: "clk.h"
// dependency: "clk-exynos-arm64.h"

/* NOTE: Must be equal to the last clock ID increased by one */
const CLKS_NR_TOP: usize = (DOUT_CLKCMU_TAA_NOC + 1);
const CLKS_NR_CPUCL0: usize = (CLK_DOUT_CPUCL0_NOCP + 1);
const CLKS_NR_CPUCL1: usize = (CLK_DOUT_CPUCL1_NOCP + 1);
const CLKS_NR_CPUCL2: usize = (CLK_DOUT_CPUCL2_NOCP + 1);
const CLKS_NR_PERIC0: usize = (CLK_DOUT_PERIC0_I3C + 1);
const CLKS_NR_PERIC1: usize = (CLK_DOUT_PERIC1_I3C + 1);
const CLKS_NR_MISC: usize = (CLK_DOUT_MISC_OSC_DIV2 + 1);
const CLKS_NR_HSI0: usize = (CLK_DOUT_HSI0_PCIE_APB + 1);
const CLKS_NR_HSI1: usize = (CLK_MOUT_HSI1_USBDRD + 1);
const CLKS_NR_HSI2: usize = (CLK_DOUT_HSI2_ETHERNET_PTP + 1);
const CLKS_NR_M2M: usize = (CLK_DOUT_M2M_NOCP + 1);
const CLKS_NR_MFC: usize = (CLK_DOUT_MFC_NOCP + 1);
const CLKS_NR_MFD: usize = (CLK_DOUT_MFD_NOCP + 1);
const CLKS_NR_G3D: usize = (CLK_MOUT_G3D_NOCP_USER + 1);

/* ---- CMU_TOP ------------------------------------------------------------ */

/* Register Offset definitions for CMU_TOP (0x11000000) */
const PLL_LOCKTIME_PLL_MMC: usize = 0x0004;
const PLL_LOCKTIME_PLL_SHARED0: usize = 0x0008;
const PLL_LOCKTIME_PLL_SHARED1: usize = 0x000c;
const PLL_LOCKTIME_PLL_SHARED2: usize = 0x0010;
const PLL_LOCKTIME_PLL_SHARED3: usize = 0x0014;
const PLL_LOCKTIME_PLL_SHARED4: usize = 0x0018;
const PLL_LOCKTIME_PLL_SHARED5: usize = 0x0018;
const PLL_CON0_PLL_MMC: usize = 0x0140;
const PLL_CON3_PLL_MMC: usize = 0x014c;
const PLL_CON0_PLL_SHARED0: usize = 0x0180;
const PLL_CON3_PLL_SHARED0: usize = 0x018c;
const PLL_CON0_PLL_SHARED1: usize = 0x01c0;
const PLL_CON3_PLL_SHARED1: usize = 0x01cc;
const PLL_CON0_PLL_SHARED2: usize = 0x0200;
const PLL_CON3_PLL_SHARED2: usize = 0x020c;
const PLL_CON0_PLL_SHARED3: usize = 0x0240;
const PLL_CON3_PLL_SHARED3: usize = 0x024c;
const PLL_CON0_PLL_SHARED4: usize = 0x0280;
const PLL_CON3_PLL_SHARED4: usize = 0x028c;
const PLL_CON0_PLL_SHARED5: usize = 0x02c0;
const PLL_CON3_PLL_SHARED5: usize = 0x02cc;

/* MUX */
const CLK_CON_MUX_MUX_CLKCMU_ACC_NOC: usize = 0x1000;
const CLK_CON_MUX_MUX_CLKCMU_APM_NOC: usize = 0x1004;
const CLK_CON_MUX_MUX_CLKCMU_AUD_CPU: usize = 0x1008;
const CLK_CON_MUX_MUX_CLKCMU_AUD_NOC: usize = 0x100c;
const CLK_CON_MUX_MUX_CLKCMU_CIS_MCLK0: usize = 0x1010;
const CLK_CON_MUX_MUX_CLKCMU_CIS_MCLK1: usize = 0x1014;
const CLK_CON_MUX_MUX_CLKCMU_CIS_MCLK2: usize = 0x1018;
const CLK_CON_MUX_MUX_CLKCMU_CIS_MCLK3: usize = 0x101c;
const CLK_CON_MUX_MUX_CLKCMU_CMU_BOOST: usize = 0x1020;
const CLK_CON_MUX_MUX_CLKCMU_CPUCL0_CLUSTER: usize = 0x1024;
const CLK_CON_MUX_MUX_CLKCMU_CPUCL0_DBG: usize = 0x1028;
const CLK_CON_MUX_MUX_CLKCMU_CPUCL0_SWITCH: usize = 0x102c;
const CLK_CON_MUX_MUX_CLKCMU_CPUCL1_CLUSTER: usize = 0x1030;
const CLK_CON_MUX_MUX_CLKCMU_CPUCL1_SWITCH: usize = 0x1034;
const CLK_CON_MUX_MUX_CLKCMU_CPUCL2_CLUSTER: usize = 0x1038;
const CLK_CON_MUX_MUX_CLKCMU_CPUCL2_SWITCH: usize = 0x103c;
const CLK_CON_MUX_MUX_CLKCMU_DNC_NOC: usize = 0x1040;
const CLK_CON_MUX_MUX_CLKCMU_DPTX_DPGTC: usize = 0x1044;
const CLK_CON_MUX_MUX_CLKCMU_DPTX_DPOSC: usize = 0x1048;
const CLK_CON_MUX_MUX_CLKCMU_DPTX_NOC: usize = 0x104c;
const CLK_CON_MUX_MUX_CLKCMU_DPUB_DSIM: usize = 0x1050;
const CLK_CON_MUX_MUX_CLKCMU_DPUB_NOC: usize = 0x1054;
const CLK_CON_MUX_MUX_CLKCMU_DPUF0_NOC: usize = 0x1058;
const CLK_CON_MUX_MUX_CLKCMU_DPUF1_NOC: usize = 0x105c;
const CLK_CON_MUX_MUX_CLKCMU_DPUF2_NOC: usize = 0x1060;
const CLK_CON_MUX_MUX_CLKCMU_DSP_NOC: usize = 0x1064;
const CLK_CON_MUX_MUX_CLKCMU_G3D_NOCP: usize = 0x1068;
const CLK_CON_MUX_MUX_CLKCMU_G3D_SWITCH: usize = 0x106c;
const CLK_CON_MUX_MUX_CLKCMU_GNPU_NOC: usize = 0x1070;
const CLK_CON_MUX_MUX_CLKCMU_HSI0_NOC: usize = 0x1074;
const CLK_CON_MUX_MUX_CLKCMU_ACC_ORB: usize = 0x1078;
const CLK_CON_MUX_MUX_CLKCMU_GNPU_XMAA: usize = 0x107c;
const CLK_CON_MUX_MUX_CLKCMU_HSI1_MMC_CARD: usize = 0x1080;
const CLK_CON_MUX_MUX_CLKCMU_HSI1_NOC: usize = 0x1084;
const CLK_CON_MUX_MUX_CLKCMU_HSI1_USBDRD: usize = 0x1088;
const CLK_CON_MUX_MUX_CLKCMU_HSI2_ETHERNET: usize = 0x108c;
const CLK_CON_MUX_MUX_CLKCMU_HSI2_NOC: usize = 0x1090;
const CLK_CON_MUX_MUX_CLKCMU_HSI2_NOC_UFS: usize = 0x1094;
const CLK_CON_MUX_MUX_CLKCMU_HSI2_UFS_EMBD: usize = 0x1098;
const CLK_CON_MUX_MUX_CLKCMU_ISP_NOC: usize = 0x109c;
const CLK_CON_MUX_MUX_CLKCMU_M2M_JPEG: usize = 0x10a0;
const CLK_CON_MUX_MUX_CLKCMU_M2M_NOC: usize = 0x10a4;
const CLK_CON_MUX_MUX_CLKCMU_MFC_MFC: usize = 0x10a8;
const CLK_CON_MUX_MUX_CLKCMU_MFC_WFD: usize = 0x10ac;
const CLK_CON_MUX_MUX_CLKCMU_MFD_NOC: usize = 0x10b0;
const CLK_CON_MUX_MUX_CLKCMU_MIF_NOCP: usize = 0x10b4;
const CLK_CON_MUX_MUX_CLKCMU_MIF_SWITCH: usize = 0x10b8;
const CLK_CON_MUX_MUX_CLKCMU_MISC_NOC: usize = 0x10bc;
const CLK_CON_MUX_MUX_CLKCMU_NOCL0_NOC: usize = 0x10c0;
const CLK_CON_MUX_MUX_CLKCMU_NOCL1_NOC: usize = 0x10c4;
const CLK_CON_MUX_MUX_CLKCMU_NOCL2_NOC: usize = 0x10c8;
const CLK_CON_MUX_MUX_CLKCMU_PERIC0_IP: usize = 0x10cc;
const CLK_CON_MUX_MUX_CLKCMU_PERIC0_NOC: usize = 0x10d0;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_IP: usize = 0x10d4;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_NOC: usize = 0x10d8;
const CLK_CON_MUX_MUX_CLKCMU_SDMA_NOC: usize = 0x10dc;
const CLK_CON_MUX_MUX_CLKCMU_SNW_NOC: usize = 0x10e0;
const CLK_CON_MUX_MUX_CLKCMU_SSP_NOC: usize = 0x10e4;
const CLK_CON_MUX_MUX_CLKCMU_TAA_NOC: usize = 0x10e8;
const CLK_CON_MUX_MUX_CLK_CMU_NOCP: usize = 0x10ec;
const CLK_CON_MUX_MUX_CLK_CMU_PLLCLKOUT: usize = 0x10f0;
const CLK_CON_MUX_MUX_CMU_CMUREF: usize = 0x10f4;

/* DIV */
const CLK_CON_DIV_CLKCMU_ACC_NOC: usize = 0x1800;
const CLK_CON_DIV_CLKCMU_APM_NOC: usize = 0x1804;
const CLK_CON_DIV_CLKCMU_AUD_CPU: usize = 0x1808;
const CLK_CON_DIV_CLKCMU_AUD_NOC: usize = 0x180c;
const CLK_CON_DIV_CLKCMU_CIS_MCLK0: usize = 0x1810;
const CLK_CON_DIV_CLKCMU_CIS_MCLK1: usize = 0x1814;
const CLK_CON_DIV_CLKCMU_CIS_MCLK2: usize = 0x1818;
const CLK_CON_DIV_CLKCMU_CIS_MCLK3: usize = 0x181c;
const CLK_CON_DIV_CLKCMU_CPUCL0_CLUSTER: usize = 0x1820;
const CLK_CON_DIV_CLKCMU_CPUCL0_DBG: usize = 0x1824;
const CLK_CON_DIV_CLKCMU_CPUCL0_SWITCH: usize = 0x1828;
const CLK_CON_DIV_CLKCMU_CPUCL1_CLUSTER: usize = 0x182c;
const CLK_CON_DIV_CLKCMU_CPUCL1_SWITCH: usize = 0x1830;
const CLK_CON_DIV_CLKCMU_CPUCL2_CLUSTER: usize = 0x1834;
const CLK_CON_DIV_CLKCMU_CPUCL2_SWITCH: usize = 0x1838;
const CLK_CON_DIV_CLKCMU_DNC_NOC: usize = 0x183c;
const CLK_CON_DIV_CLKCMU_DPTX_DPGTC: usize = 0x1840;
const CLK_CON_DIV_CLKCMU_DPTX_DPOSC: usize = 0x1844;
const CLK_CON_DIV_CLKCMU_DPTX_NOC: usize = 0x1848;
const CLK_CON_DIV_CLKCMU_DPUB_DSIM: usize = 0x184c;
const CLK_CON_DIV_CLKCMU_DPUB_NOC: usize = 0x1850;
const CLK_CON_DIV_CLKCMU_DPUF0_NOC: usize = 0x1854;
const CLK_CON_DIV_CLKCMU_DPUF1_NOC: usize = 0x1858;
const CLK_CON_DIV_CLKCMU_DPUF2_NOC: usize = 0x185c;
const CLK_CON_DIV_CLKCMU_DSP_NOC: usize = 0x1860;
const CLK_CON_DIV_CLKCMU_G3D_NOCP: usize = 0x1864;
const CLK_CON_DIV_CLKCMU_G3D_SWITCH: usize = 0x1868;
const CLK_CON_DIV_CLKCMU_GNPU_NOC: usize = 0x186c;
const CLK_CON_DIV_CLKCMU_HSI0_NOC: usize = 0x1870;
const CLK_CON_DIV_CLKCMU_ACC_ORB: usize = 0x1874;
const CLK_CON_DIV_CLKCMU_GNPU_XMAA: usize = 0x1878;
const CLK_CON_DIV_CLKCMU_HSI1_MMC_CARD: usize = 0x187c;
const CLK_CON_DIV_CLKCMU_HSI1_NOC: usize = 0x1880;
const CLK_CON_DIV_CLKCMU_HSI1_USBDRD: usize = 0x1884;
const CLK_CON_DIV_CLKCMU_HSI2_ETHERNET: usize = 0x1888;
const CLK_CON_DIV_CLKCMU_HSI2_NOC: usize = 0x188c;
const CLK_CON_DIV_CLKCMU_HSI2_NOC_UFS: usize = 0x1890;
const CLK_CON_DIV_CLKCMU_HSI2_UFS_EMBD: usize = 0x1894;
const CLK_CON_DIV_CLKCMU_ISP_NOC: usize = 0x1898;
const CLK_CON_DIV_CLKCMU_M2M_JPEG: usize = 0x189c;
const CLK_CON_DIV_CLKCMU_M2M_NOC: usize = 0x18a0;
const CLK_CON_DIV_CLKCMU_MFC_MFC: usize = 0x18a4;
const CLK_CON_DIV_CLKCMU_MFC_WFD: usize = 0x18a8;
const CLK_CON_DIV_CLKCMU_MFD_NOC: usize = 0x18ac;
const CLK_CON_DIV_CLKCMU_MIF_NOCP: usize = 0x18b0;
const CLK_CON_DIV_CLKCMU_MISC_NOC: usize = 0x18b4;
const CLK_CON_DIV_CLKCMU_NOCL0_NOC: usize = 0x18b8;
const CLK_CON_DIV_CLKCMU_NOCL1_NOC: usize = 0x18bc;
const CLK_CON_DIV_CLKCMU_NOCL2_NOC: usize = 0x18c0;
const CLK_CON_DIV_CLKCMU_PERIC0_IP: usize = 0x18c4;
const CLK_CON_DIV_CLKCMU_PERIC0_NOC: usize = 0x18c8;
const CLK_CON_DIV_CLKCMU_PERIC1_IP: usize = 0x18cc;
const CLK_CON_DIV_CLKCMU_PERIC1_NOC: usize = 0x18d0;
const CLK_CON_DIV_CLKCMU_SDMA_NOC: usize = 0x18d4;
const CLK_CON_DIV_CLKCMU_SNW_NOC: usize = 0x18d8;
const CLK_CON_DIV_CLKCMU_SSP_NOC: usize = 0x18dc;
const CLK_CON_DIV_CLKCMU_TAA_NOC: usize = 0x18e0;
const CLK_CON_DIV_CLK_ADD_CH_CLK: usize = 0x18e4;
const CLK_CON_DIV_CLK_CMU_PLLCLKOUT: usize = 0x18e8;
const CLK_CON_DIV_DIV_CLKCMU_CMU_BOOST: usize = 0x18ec;
const CLK_CON_DIV_DIV_CLK_CMU_NOCP: usize = 0x18f0;

static top_clk_regs: &[usize] = &[
	PLL_LOCKTIME_PLL_MMC,
	PLL_LOCKTIME_PLL_SHARED0,
	PLL_LOCKTIME_PLL_SHARED1,
	PLL_LOCKTIME_PLL_SHARED2,
	PLL_LOCKTIME_PLL_SHARED3,
	PLL_LOCKTIME_PLL_SHARED4,
	PLL_LOCKTIME_PLL_SHARED5,
	PLL_CON0_PLL_MMC,
	PLL_CON3_PLL_MMC,
	PLL_CON0_PLL_SHARED0,
	PLL_CON3_PLL_SHARED0,
	PLL_CON0_PLL_SHARED1,
	PLL_CON3_PLL_SHARED1,
	PLL_CON0_PLL_SHARED2,
	PLL_CON3_PLL_SHARED2,
	PLL_CON0_PLL_SHARED3,
	PLL_CON3_PLL_SHARED3,
	PLL_CON0_PLL_SHARED4,
	PLL_CON3_PLL_SHARED4,
	PLL_CON0_PLL_SHARED5,
	PLL_CON3_PLL_SHARED5,
	CLK_CON_MUX_MUX_CLKCMU_ACC_NOC,
	CLK_CON_MUX_MUX_CLKCMU_APM_NOC,
	CLK_CON_MUX_MUX_CLKCMU_AUD_CPU,
	CLK_CON_MUX_MUX_CLKCMU_AUD_NOC,
	CLK_CON_MUX_MUX_CLKCMU_CIS_MCLK0,
	CLK_CON_MUX_MUX_CLKCMU_CIS_MCLK1,
	CLK_CON_MUX_MUX_CLKCMU_CIS_MCLK2,
	CLK_CON_MUX_MUX_CLKCMU_CIS_MCLK3,
	CLK_CON_MUX_MUX_CLKCMU_CMU_BOOST,
	CLK_CON_MUX_MUX_CLKCMU_CPUCL0_CLUSTER,
	CLK_CON_MUX_MUX_CLKCMU_CPUCL0_DBG,
	CLK_CON_MUX_MUX_CLKCMU_CPUCL0_SWITCH,
	CLK_CON_MUX_MUX_CLKCMU_CPUCL1_CLUSTER,
	CLK_CON_MUX_MUX_CLKCMU_CPUCL1_SWITCH,
	CLK_CON_MUX_MUX_CLKCMU_CPUCL2_CLUSTER,
	CLK_CON_MUX_MUX_CLKCMU_CPUCL2_SWITCH,
	CLK_CON_MUX_MUX_CLKCMU_DNC_NOC,
	CLK_CON_MUX_MUX_CLKCMU_DPTX_DPGTC,
	CLK_CON_MUX_MUX_CLKCMU_DPTX_DPOSC,
	CLK_CON_MUX_MUX_CLKCMU_DPTX_NOC,
	CLK_CON_MUX_MUX_CLKCMU_DPUB_DSIM,
	CLK_CON_MUX_MUX_CLKCMU_DPUB_NOC,
	CLK_CON_MUX_MUX_CLKCMU_DPUF0_NOC,
	CLK_CON_MUX_MUX_CLKCMU_DPUF1_NOC,
	CLK_CON_MUX_MUX_CLKCMU_DPUF2_NOC,
	CLK_CON_MUX_MUX_CLKCMU_DSP_NOC,
	CLK_CON_MUX_MUX_CLKCMU_G3D_NOCP,
	CLK_CON_MUX_MUX_CLKCMU_G3D_SWITCH,
	CLK_CON_MUX_MUX_CLKCMU_GNPU_NOC,
	CLK_CON_MUX_MUX_CLKCMU_HSI0_NOC,
	CLK_CON_MUX_MUX_CLKCMU_ACC_ORB,
	CLK_CON_MUX_MUX_CLKCMU_GNPU_XMAA,
	CLK_CON_MUX_MUX_CLKCMU_HSI1_MMC_CARD,
	CLK_CON_MUX_MUX_CLKCMU_HSI1_NOC,
	CLK_CON_MUX_MUX_CLKCMU_HSI1_USBDRD,
	CLK_CON_MUX_MUX_CLKCMU_HSI2_ETHERNET,
	CLK_CON_MUX_MUX_CLKCMU_HSI2_NOC,
	CLK_CON_MUX_MUX_CLKCMU_HSI2_NOC_UFS,
	CLK_CON_MUX_MUX_CLKCMU_HSI2_UFS_EMBD,
	CLK_CON_MUX_MUX_CLKCMU_ISP_NOC,
	CLK_CON_MUX_MUX_CLKCMU_M2M_JPEG,
	CLK_CON_MUX_MUX_CLKCMU_M2M_NOC,
	CLK_CON_MUX_MUX_CLKCMU_MFC_MFC,
	CLK_CON_MUX_MUX_CLKCMU_MFC_WFD,
	CLK_CON_MUX_MUX_CLKCMU_MFD_NOC,
	CLK_CON_MUX_MUX_CLKCMU_MIF_NOCP,
	CLK_CON_MUX_MUX_CLKCMU_MIF_SWITCH,
	CLK_CON_MUX_MUX_CLKCMU_MISC_NOC,
	CLK_CON_MUX_MUX_CLKCMU_NOCL0_NOC,
	CLK_CON_MUX_MUX_CLKCMU_NOCL1_NOC,
	CLK_CON_MUX_MUX_CLKCMU_NOCL2_NOC,
	CLK_CON_MUX_MUX_CLKCMU_PERIC0_IP,
	CLK_CON_MUX_MUX_CLKCMU_PERIC0_NOC,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_IP,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_NOC,
	CLK_CON_MUX_MUX_CLKCMU_SDMA_NOC,
	CLK_CON_MUX_MUX_CLKCMU_SNW_NOC,
	CLK_CON_MUX_MUX_CLKCMU_SSP_NOC,
	CLK_CON_MUX_MUX_CLKCMU_TAA_NOC,
	CLK_CON_MUX_MUX_CLK_CMU_NOCP,
	CLK_CON_MUX_MUX_CLK_CMU_PLLCLKOUT,
	CLK_CON_MUX_MUX_CMU_CMUREF,
	CLK_CON_DIV_CLKCMU_ACC_NOC,
	CLK_CON_DIV_CLKCMU_APM_NOC,
	CLK_CON_DIV_CLKCMU_AUD_CPU,
	CLK_CON_DIV_CLKCMU_AUD_NOC,
	CLK_CON_DIV_CLKCMU_CIS_MCLK0,
	CLK_CON_DIV_CLKCMU_CIS_MCLK1,
	CLK_CON_DIV_CLKCMU_CIS_MCLK2,
	CLK_CON_DIV_CLKCMU_CIS_MCLK3,
	CLK_CON_DIV_CLKCMU_CPUCL0_CLUSTER,
	CLK_CON_DIV_CLKCMU_CPUCL0_DBG,
	CLK_CON_DIV_CLKCMU_CPUCL0_SWITCH,
	CLK_CON_DIV_CLKCMU_CPUCL1_CLUSTER,
	CLK_CON_DIV_CLKCMU_CPUCL1_SWITCH,
	CLK_CON_DIV_CLKCMU_CPUCL2_CLUSTER,
	CLK_CON_DIV_CLKCMU_CPUCL2_SWITCH,
	CLK_CON_DIV_CLKCMU_DNC_NOC,
	CLK_CON_DIV_CLKCMU_DPTX_DPGTC,
	CLK_CON_DIV_CLKCMU_DPTX_DPOSC,
	CLK_CON_DIV_CLKCMU_DPTX_NOC,
	CLK_CON_DIV_CLKCMU_DPUB_DSIM,
	CLK_CON_DIV_CLKCMU_DPUB_NOC,
	CLK_CON_DIV_CLKCMU_DPUF0_NOC,
	CLK_CON_DIV_CLKCMU_DPUF1_NOC,
	CLK_CON_DIV_CLKCMU_DPUF2_NOC,
	CLK_CON_DIV_CLKCMU_DSP_NOC,
	CLK_CON_DIV_CLKCMU_G3D_NOCP,
	CLK_CON_DIV_CLKCMU_G3D_SWITCH,
	CLK_CON_DIV_CLKCMU_GNPU_NOC,
	CLK_CON_DIV_CLKCMU_HSI0_NOC,
	CLK_CON_DIV_CLKCMU_ACC_ORB,
	CLK_CON_DIV_CLKCMU_GNPU_XMAA,
	CLK_CON_DIV_CLKCMU_HSI1_MMC_CARD,
	CLK_CON_DIV_CLKCMU_HSI1_NOC,
	CLK_CON_DIV_CLKCMU_HSI1_USBDRD,
	CLK_CON_DIV_CLKCMU_HSI2_ETHERNET,
	CLK_CON_DIV_CLKCMU_HSI2_NOC,
	CLK_CON_DIV_CLKCMU_HSI2_NOC_UFS,
	CLK_CON_DIV_CLKCMU_HSI2_UFS_EMBD,
	CLK_CON_DIV_CLKCMU_ISP_NOC,
	CLK_CON_DIV_CLKCMU_M2M_JPEG,
	CLK_CON_DIV_CLKCMU_M2M_NOC,
	CLK_CON_DIV_CLKCMU_MFC_MFC,
	CLK_CON_DIV_CLKCMU_MFC_WFD,
	CLK_CON_DIV_CLKCMU_MFD_NOC,
	CLK_CON_DIV_CLKCMU_MIF_NOCP,
	CLK_CON_DIV_CLKCMU_MISC_NOC,
	CLK_CON_DIV_CLKCMU_NOCL0_NOC,
	CLK_CON_DIV_CLKCMU_NOCL1_NOC,
	CLK_CON_DIV_CLKCMU_NOCL2_NOC,
	CLK_CON_DIV_CLKCMU_PERIC0_IP,
	CLK_CON_DIV_CLKCMU_PERIC0_NOC,
	CLK_CON_DIV_CLKCMU_PERIC1_IP,
	CLK_CON_DIV_CLKCMU_PERIC1_NOC,
	CLK_CON_DIV_CLKCMU_SDMA_NOC,
	CLK_CON_DIV_CLKCMU_SNW_NOC,
	CLK_CON_DIV_CLKCMU_SSP_NOC,
	CLK_CON_DIV_CLKCMU_TAA_NOC,
	CLK_CON_DIV_CLK_ADD_CH_CLK,
	CLK_CON_DIV_CLK_CMU_PLLCLKOUT,
	CLK_CON_DIV_DIV_CLKCMU_CMU_BOOST,
	CLK_CON_DIV_DIV_CLK_CMU_NOCP,
};

static top_pll_clks: &[samsung_pll_clock] = &[
	/* CMU_TOP_PURECLKCOMP */
	PLL(pll_531x, FOUT_SHARED0_PLL, "fout_shared0_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED0, PLL_CON3_PLL_SHARED0, core::ptr::null_mut()),
	PLL(pll_531x, FOUT_SHARED1_PLL, "fout_shared1_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED1, PLL_CON3_PLL_SHARED1, core::ptr::null_mut()),
	PLL(pll_531x, FOUT_SHARED2_PLL, "fout_shared2_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED2, PLL_CON3_PLL_SHARED2, core::ptr::null_mut()),
	PLL(pll_531x, FOUT_SHARED3_PLL, "fout_shared3_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED3, PLL_CON3_PLL_SHARED3, core::ptr::null_mut()),
	PLL(pll_531x, FOUT_SHARED4_PLL, "fout_shared4_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED4, PLL_CON3_PLL_SHARED4, core::ptr::null_mut()),
	PLL(pll_531x, FOUT_SHARED5_PLL, "fout_shared5_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED5, PLL_CON3_PLL_SHARED5, core::ptr::null_mut()),
	PLL(pll_531x, FOUT_MMC_PLL, "fout_mmc_pll", "oscclk",
	    PLL_LOCKTIME_PLL_MMC, PLL_CON3_PLL_MMC, core::ptr::null_mut()),
};

/* List of parent clocks for Muxes in CMU_TOP */
static mout_shared0_pll_p: &[&str] = &[ "oscclk", "fout_shared0_pll" };
static mout_shared1_pll_p: &[&str] = &[ "oscclk", "fout_shared1_pll" };
static mout_shared2_pll_p: &[&str] = &[ "oscclk", "fout_shared2_pll" };
static mout_shared3_pll_p: &[&str] = &[ "oscclk", "fout_shared3_pll" };
static mout_shared4_pll_p: &[&str] = &[ "oscclk", "fout_shared4_pll" };
static mout_shared5_pll_p: &[&str] = &[ "oscclk", "fout_shared5_pll" };
static mout_mmc_pll_p: &[&str] = &[ "oscclk", "fout_mmc_pll" };

static mout_clkcmu_cmu_boost_p: &[&str] = &[ "dout_shared2_div3", "dout_shared1_div4",
				   "dout_shared2_div4", "dout_shared4_div4" };

static mout_clkcmu_cmu_cmuref_p: &[&str] = &[ "oscclk", "dout_cmu_boost" };

static mout_clkcmu_acc_noc_p: &[&str] = &[ "dout_shared2_div2", "dout_shared0_div3",
				 "dout_shared4_div2", "dout_shared1_div3",
				 "dout_shared2_div3", "dout_shared5_div1",
				 "dout_shared3_div1", "oscclk" };

static mout_clkcmu_acc_orb_p: &[&str] = &[ "dout_shared2_div2", "dout_shared0_div3",
				 "dout_shared1_div2", "dout_shared1_div3",
				 "dout_shared2_div3", "fout_shared5_pll",
				 "fout_shared3_pll", "oscclk" };

static mout_clkcmu_apm_noc_p: &[&str] = &[ "dout_shared2_div2", "dout_shared1_div4",
				 "dout_shared2_div4", "dout_shared4_div4" };

static mout_clkcmu_aud_cpu_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2",
				 "dout_shared2_div2", "dout_shared0_div3",
				 "dout_shared4_div2", "dout_shared1_div3",
				 "dout_shared2_div3", "dout_shared4_div3" };

static mout_clkcmu_aud_noc_p: &[&str] = &[ "dout_shared2_div2", "dout_shared4_div2",
				 "dout_shared1_div2", "dout_shared2_div3" };

static mout_clkcmu_cpucl0_switch_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2",
				       "dout_shared2_div2", "dout_shared4_div2" };

static mout_clkcmu_cpucl0_cluster_p: &[&str] = &[ "fout_shared2_pll", "fout_shared4_pll",
					"dout_shared0_div2", "dout_shared1_div2",
					"dout_shared2_div2", "dout_shared4_div2",
					"dout_shared2_div3", "fout_shared3_pll" };

static mout_clkcmu_cpucl0_dbg_p: &[&str] = &[ "dout_shared2_div2", "dout_shared0_div3",
				    "dout_shared4_div2", "dout_shared0_div4" };

static mout_clkcmu_cpucl1_switch_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2",
				       "dout_shared2_div2", "dout_shared4_div2" };

static mout_clkcmu_cpucl1_cluster_p: &[&str] = &[ "fout_shared2_pll", "fout_shared4_pll",
					"dout_shared0_div2", "dout_shared1_div2",
					"dout_shared2_div2", "dout_shared4_div2",
					"dout_shared2_div3", "fout_shared3_pll" };

static mout_clkcmu_cpucl2_switch_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2",
				       "dout_shared2_div2", "dout_shared4_div2" };

static mout_clkcmu_cpucl2_cluster_p: &[&str] = &[ "fout_shared2_pll", "fout_shared4_pll",
					"dout_shared0_div2", "dout_shared1_div2",
					"dout_shared2_div2", "dout_shared4_div2",
					"dout_shared2_div3", "fout_shared3_pll" };

static mout_clkcmu_dnc_noc_p: &[&str] = &[ "dout_shared1_div2", "dout_shared2_div2",
				 "dout_shared0_div3", "dout_shared4_div2",
				 "dout_shared1_div3", "dout_shared2_div3",
				 "dout_shared1_div4", "fout_shared3_pll" };

static mout_clkcmu_dptx_noc_p: &[&str] = &[ "dout_shared4_div2", "dout_shared2_div3",
				  "dout_shared1_div4", "dout_shared2_div4" };

static mout_clkcmu_dptx_dpgtc_p: &[&str] = &[ "oscclk", "dout_shared2_div3",
				    "dout_shared2_div4", "dout_shared4_div4" };

static mout_clkcmu_dptx_dposc_p: &[&str] = &[ "oscclk", "dout_shared2_div4" };

static mout_clkcmu_dpub_noc_p: &[&str] = &[ "dout_shared4_div2", "dout_shared1_div3",
				 "dout_shared2_div3", "dout_shared1_div4",
				 "dout_shared2_div4", "dout_shared4_div4",
				 "fout_shared3_pll" };

static mout_clkcmu_dpub_dsim_p: &[&str] = &[ "dout_shared2_div3", "dout_shared2_div4" };

static mout_clkcmu_dpuf_noc_p: &[&str] = &[ "dout_shared4_div2", "dout_shared1_div3",
				   "dout_shared2_div3", "dout_shared1_div4",
				   "dout_shared2_div4", "dout_shared4_div4",
				   "fout_shared3_pll" };

static mout_clkcmu_dsp_noc_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2",
				 "dout_shared2_div2", "dout_shared0_div3",
				 "dout_shared4_div2", "dout_shared1_div3",
				 "fout_shared5_pll", "fout_shared3_pll" };

static mout_clkcmu_g3d_switch_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2",
				    "dout_shared2_div2", "dout_shared4_div2" };

static mout_clkcmu_g3d_nocp_p: &[&str] = &[ "dout_shared2_div3", "dout_shared1_div4",
				  "dout_shared2_div4", "dout_shared4_div4" };

static mout_clkcmu_gnpu_noc_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2",
				  "dout_shared2_div2", "dout_shared0_div3",
				  "dout_shared4_div2", "dout_shared2_div3",
				  "fout_shared5_pll", "fout_shared3_pll" };

static mout_clkcmu_hsi0_noc_p: &[&str] = &[ "dout_shared4_div2", "dout_shared2_div3",
				  "dout_shared1_div4", "dout_shared2_div4" };

static mout_clkcmu_hsi1_noc_p: &[&str] = &[ "dout_shared2_div3", "dout_shared1_div4",
				  "dout_shared2_div4", "dout_shared4_div4" };

static mout_clkcmu_hsi1_usbdrd_p: &[&str] = &[ "oscclk", "dout_shared2_div3",
				     "dout_shared2_div4", "dout_shared4_div4" };

static mout_clkcmu_hsi1_mmc_card_p: &[&str] = &[ "oscclk", "dout_shared2_div2",
				       "dout_shared4_div2", "fout_mmc_pll" };

static mout_clkcmu_hsi2_noc_p: &[&str] = &[ "dout_shared4_div2", "dout_shared2_div3",
				  "dout_shared1_div4", "dout_shared2_div4" };

static mout_clkcmu_hsi2_noc_ufs_p: &[&str] = &[ "dout_shared4_div2", "dout_shared2_div3",
				      "dout_shared1_div4", "dout_shared2_div2" };

static mout_clkcmu_hsi2_ufs_embd_p: &[&str] = &[ "oscclk", "dout_shared2_div3",
				       "dout_shared2_div4", "dout_shared4_div4" };

static mout_clkcmu_hsi2_ethernet_p: &[&str] = &[ "oscclk", "dout_shared2_div2",
				       "dout_shared0_div3", "dout_shared1_div3" };

static mout_clkcmu_isp_noc_p: &[&str] = &[ "dout_shared2_div2", "dout_shared0_div3",
				 "dout_shared4_div2", "dout_shared1_div3",
				 "dout_shared2_div3", "fout_shared5_pll",
				 "fout_shared3_pll", "oscclk" };

static mout_clkcmu_m2m_noc_p: &[&str] = &[ "dout_shared0_div3", "dout_shared4_div2",
				 "dout_shared2_div3", "dout_shared1_div4" };

static mout_clkcmu_m2m_jpeg_p: &[&str] = &[ "dout_shared0_div3", "dout_shared4_div2",
				  "dout_shared2_div3", "dout_shared1_div4" };

static mout_clkcmu_mfc_mfc_p: &[&str] = &[ "dout_shared0_div3", "dout_shared4_div2",
				 "dout_shared2_div3", "dout_shared1_div4" };

static mout_clkcmu_mfc_wfd_p: &[&str] = &[ "dout_shared0_div3", "dout_shared4_div2",
				 "dout_shared2_div3", "dout_shared1_div4" };

static mout_clkcmu_mfd_noc_p: &[&str] = &[ "dout_shared2_div2", "dout_shared0_div3",
				 "dout_shared4_div2", "dout_shared1_div3",
				 "dout_shared2_div3", "fout_shared5_pll",
				 "fout_shared3_pll", "oscclk" };

static mout_clkcmu_mif_switch_p: &[&str] = &[ "fout_shared0_pll", "fout_shared1_pll",
				    "fout_shared2_pll", "fout_shared4_pll",
				    "dout_shared0_div2", "dout_shared1_div2",
				    "dout_shared2_div2", "fout_shared5_pll" };

static mout_clkcmu_mif_nocp_p: &[&str] = &[ "dout_shared2_div3", "dout_shared1_div4",
				  "dout_shared2_div4", "dout_shared4_div4" };

static mout_clkcmu_misc_noc_p: &[&str] = &[ "dout_shared4_div2", "dout_shared2_div3",
				  "dout_shared1_div4", "dout_shared2_div4" };

static mout_clkcmu_nocl0_noc_p: &[&str] = &[ "dout_shared0_div2", "dout_shared1_div2",
				   "dout_shared2_div2", "dout_shared0_div3",
				   "dout_shared4_div2", "dout_shared1_div3",
				   "dout_shared2_div3", "fout_shared3_pll" };

static mout_clkcmu_nocl1_noc_p: &[&str] = &[ "dout_shared2_div2", "dout_shared0_div3",
				   "dout_shared4_div2", "dout_shared1_div3",
				   "dout_shared2_div3", "fout_shared5_pll",
				   "fout_shared3_pll", "oscclk" };

static mout_clkcmu_nocl2_noc_p: &[&str] = &[ "dout_shared2_div2", "dout_shared0_div3",
				   "dout_shared4_div2", "dout_shared1_div3",
				   "dout_shared2_div3", "fout_shared5_pll",
				   "fout_shared3_pll", "oscclk" };

static mout_clkcmu_peric0_noc_p: &[&str] = &[ "dout_shared2_div3", "dout_shared2_div4" };

static mout_clkcmu_peric0_ip_p: &[&str] = &[ "dout_shared2_div3", "dout_shared2_div4" };

static mout_clkcmu_peric1_noc_p: &[&str] = &[ "dout_shared2_div3", "dout_shared2_div4" };

static mout_clkcmu_peric1_ip_p: &[&str] = &[ "dout_shared2_div3", "dout_shared2_div4" };

static mout_clkcmu_sdma_noc_p: &[&str] = &[ "dout_shared1_div2", "dout_shared2_div2",
				  "dout_shared0_div3", "dout_shared4_div2",
				  "dout_shared1_div3", "dout_shared2_div3",
				  "dout_shared1_div4", "fout_shared3_pll" };

static mout_clkcmu_snw_noc_p: &[&str] = &[ "dout_shared2_div2", "dout_shared0_div3",
				 "dout_shared4_div2", "dout_shared1_div3",
				 "dout_shared2_div3", "fout_shared5_pll",
				 "fout_shared3_pll", "oscclk" };

static mout_clkcmu_ssp_noc_p: &[&str] = &[ "dout_shared2_div3", "dout_shared1_div4",
				  "dout_shared2_div2", "dout_shared4_div4" };

static mout_clkcmu_taa_noc_p: &[&str] = &[ "dout_shared2_div2", "dout_shared0_div3",
				 "dout_shared4_div2", "dout_shared1_div3",
				 "dout_shared2_div3", "fout_shared5_pll",
				 "fout_shared3_pll", "oscclk" };

static top_mux_clks: &[samsung_mux_clock] = &[
	/* CMU_TOP_PURECLKCOMP */
	MUX(MOUT_SHARED0_PLL, "mout_shared0_pll", mout_shared0_pll_p,
	    PLL_CON0_PLL_SHARED0, 4, 1),
	MUX(MOUT_SHARED1_PLL, "mout_shared1_pll", mout_shared1_pll_p,
	    PLL_CON0_PLL_SHARED1, 4, 1),
	MUX(MOUT_SHARED2_PLL, "mout_shared2_pll", mout_shared2_pll_p,
	    PLL_CON0_PLL_SHARED2, 4, 1),
	MUX(MOUT_SHARED3_PLL, "mout_shared3_pll", mout_shared3_pll_p,
	    PLL_CON0_PLL_SHARED3, 4, 1),
	MUX(MOUT_SHARED4_PLL, "mout_shared4_pll", mout_shared4_pll_p,
	    PLL_CON0_PLL_SHARED4, 4, 1),
	MUX(MOUT_SHARED5_PLL, "mout_shared5_pll", mout_shared5_pll_p,
	    PLL_CON0_PLL_SHARED5, 4, 1),
	MUX(MOUT_MMC_PLL, "mout_mmc_pll", mout_mmc_pll_p,
	    PLL_CON0_PLL_MMC, 4, 1),

	/* BOOST */
	MUX(MOUT_CLKCMU_CMU_BOOST, "mout_clkcmu_cmu_boost",
	    mout_clkcmu_cmu_boost_p, CLK_CON_MUX_MUX_CLKCMU_CMU_BOOST, 0, 2),
	MUX(MOUT_CLKCMU_CMU_CMUREF, "mout_clkcmu_cmu_cmuref",
	    mout_clkcmu_cmu_cmuref_p, CLK_CON_MUX_MUX_CMU_CMUREF, 0, 1),

	/* ACC */
	MUX(MOUT_CLKCMU_ACC_NOC, "mout_clkcmu_acc_noc",
	    mout_clkcmu_acc_noc_p, CLK_CON_MUX_MUX_CLKCMU_ACC_NOC, 0, 3),
	MUX(MOUT_CLKCMU_ACC_ORB, "mout_clkcmu_acc_orb",
	    mout_clkcmu_acc_orb_p, CLK_CON_MUX_MUX_CLKCMU_ACC_ORB, 0, 3),

	/* APM */
	MUX(MOUT_CLKCMU_APM_NOC, "mout_clkcmu_apm_noc",
	    mout_clkcmu_apm_noc_p, CLK_CON_MUX_MUX_CLKCMU_APM_NOC, 0, 2),

	/* AUD */
	MUX(MOUT_CLKCMU_AUD_CPU, "mout_clkcmu_aud_cpu",
	    mout_clkcmu_aud_cpu_p, CLK_CON_MUX_MUX_CLKCMU_AUD_CPU, 0, 3),
	MUX(MOUT_CLKCMU_AUD_NOC, "mout_clkcmu_aud_noc",
	    mout_clkcmu_aud_noc_p, CLK_CON_MUX_MUX_CLKCMU_AUD_NOC, 0, 2),

	/* CPUCL0 */
	MUX(MOUT_CLKCMU_CPUCL0_SWITCH, "mout_clkcmu_cpucl0_switch",
	    mout_clkcmu_cpucl0_switch_p, CLK_CON_MUX_MUX_CLKCMU_CPUCL0_SWITCH,
	    0, 2),
	MUX(MOUT_CLKCMU_CPUCL0_CLUSTER, "mout_clkcmu_cpucl0_cluster",
	    mout_clkcmu_cpucl0_cluster_p, CLK_CON_MUX_MUX_CLKCMU_CPUCL0_CLUSTER,
	    0, 3),
	MUX(MOUT_CLKCMU_CPUCL0_DBG, "mout_clkcmu_cpucl0_dbg",
	    mout_clkcmu_cpucl0_dbg_p, CLK_CON_MUX_MUX_CLKCMU_CPUCL0_DBG,
	    0, 2),

	/* CPUCL1 */
	MUX(MOUT_CLKCMU_CPUCL1_SWITCH, "mout_clkcmu_cpucl1_switch",
	    mout_clkcmu_cpucl1_switch_p, CLK_CON_MUX_MUX_CLKCMU_CPUCL1_SWITCH,
	    0, 2),
	MUX(MOUT_CLKCMU_CPUCL1_CLUSTER, "mout_clkcmu_cpucl1_cluster",
	    mout_clkcmu_cpucl1_cluster_p, CLK_CON_MUX_MUX_CLKCMU_CPUCL1_CLUSTER,
	    0, 3),

	/* CPUCL2 */
	MUX(MOUT_CLKCMU_CPUCL2_SWITCH, "mout_clkcmu_cpucl2_switch",
	    mout_clkcmu_cpucl2_switch_p, CLK_CON_MUX_MUX_CLKCMU_CPUCL2_SWITCH,
	    0, 2),
	MUX(MOUT_CLKCMU_CPUCL2_CLUSTER, "mout_clkcmu_cpucl2_cluster",
	    mout_clkcmu_cpucl2_cluster_p, CLK_CON_MUX_MUX_CLKCMU_CPUCL2_CLUSTER,
	    0, 3),

	/* DNC */
	MUX(MOUT_CLKCMU_DNC_NOC, "mout_clkcmu_dnc_noc",
	    mout_clkcmu_dnc_noc_p, CLK_CON_MUX_MUX_CLKCMU_DNC_NOC, 0, 3),

	/* DPTX */
	MUX(MOUT_CLKCMU_DPTX_NOC, "mout_clkcmu_dptx_noc",
	    mout_clkcmu_dptx_noc_p, CLK_CON_MUX_MUX_CLKCMU_DPTX_NOC, 0, 2),
	MUX(MOUT_CLKCMU_DPTX_DPGTC, "mout_clkcmu_dptx_dpgtc",
	    mout_clkcmu_dptx_dpgtc_p, CLK_CON_MUX_MUX_CLKCMU_DPTX_DPGTC, 0, 2),
	MUX(MOUT_CLKCMU_DPTX_DPOSC, "mout_clkcmu_dptx_dposc",
	    mout_clkcmu_dptx_dposc_p, CLK_CON_MUX_MUX_CLKCMU_DPTX_DPOSC, 0, 1),

	/* DPUB */
	MUX(MOUT_CLKCMU_DPUB_NOC, "mout_clkcmu_dpub_noc",
	    mout_clkcmu_dpub_noc_p, CLK_CON_MUX_MUX_CLKCMU_DPUB_NOC, 0, 3),
	MUX(MOUT_CLKCMU_DPUB_DSIM, "mout_clkcmu_dpub_dsim",
	    mout_clkcmu_dpub_dsim_p, CLK_CON_MUX_MUX_CLKCMU_DPUB_DSIM, 0, 1),

	/* DPUF */
	MUX(MOUT_CLKCMU_DPUF0_NOC, "mout_clkcmu_dpuf0_noc",
	    mout_clkcmu_dpuf_noc_p, CLK_CON_MUX_MUX_CLKCMU_DPUF0_NOC, 0, 3),
	MUX(MOUT_CLKCMU_DPUF1_NOC, "mout_clkcmu_dpuf1_noc",
	    mout_clkcmu_dpuf_noc_p, CLK_CON_MUX_MUX_CLKCMU_DPUF1_NOC, 0, 3),
	MUX(MOUT_CLKCMU_DPUF2_NOC, "mout_clkcmu_dpuf2_noc",
	    mout_clkcmu_dpuf_noc_p, CLK_CON_MUX_MUX_CLKCMU_DPUF2_NOC, 0, 3),

	/* DSP */
	MUX(MOUT_CLKCMU_DSP_NOC, "mout_clkcmu_dsp_noc",
	    mout_clkcmu_dsp_noc_p, CLK_CON_MUX_MUX_CLKCMU_DSP_NOC, 0, 3),

	/* G3D */
	MUX(MOUT_CLKCMU_G3D_SWITCH, "mout_clkcmu_g3d_switch",
	    mout_clkcmu_g3d_switch_p, CLK_CON_MUX_MUX_CLKCMU_G3D_SWITCH, 0, 2),
	MUX(MOUT_CLKCMU_G3D_NOCP, "mout_clkcmu_g3d_nocp",
	    mout_clkcmu_g3d_nocp_p, CLK_CON_MUX_MUX_CLKCMU_G3D_NOCP, 0, 2),

	/* GNPU */
	MUX(MOUT_CLKCMU_GNPU_NOC, "mout_clkcmu_gnpu_noc",
	    mout_clkcmu_gnpu_noc_p, CLK_CON_MUX_MUX_CLKCMU_GNPU_NOC, 0, 3),

	/* HSI0 */
	MUX(MOUT_CLKCMU_HSI0_NOC, "mout_clkcmu_hsi0_noc",
	    mout_clkcmu_hsi0_noc_p, CLK_CON_MUX_MUX_CLKCMU_HSI0_NOC, 0, 2),

	/* HSI1 */
	MUX(MOUT_CLKCMU_HSI1_NOC, "mout_clkcmu_hsi1_noc",
	    mout_clkcmu_hsi1_noc_p, CLK_CON_MUX_MUX_CLKCMU_HSI1_NOC,
	    0, 2),
	MUX(MOUT_CLKCMU_HSI1_USBDRD, "mout_clkcmu_hsi1_usbdrd",
	    mout_clkcmu_hsi1_usbdrd_p, CLK_CON_MUX_MUX_CLKCMU_HSI1_USBDRD,
	    0, 2),
	MUX(MOUT_CLKCMU_HSI1_MMC_CARD, "mout_clkcmu_hsi1_mmc_card",
	    mout_clkcmu_hsi1_mmc_card_p, CLK_CON_MUX_MUX_CLKCMU_HSI1_MMC_CARD,
	    0, 2),

	/* HSI2 */
	MUX(MOUT_CLKCMU_HSI2_NOC, "mout_clkcmu_hsi2_noc",
	    mout_clkcmu_hsi2_noc_p, CLK_CON_MUX_MUX_CLKCMU_HSI2_NOC,
	    0, 2),
	MUX(MOUT_CLKCMU_HSI2_NOC_UFS, "mout_clkcmu_hsi2_noc_ufs",
	    mout_clkcmu_hsi2_noc_ufs_p, CLK_CON_MUX_MUX_CLKCMU_HSI2_NOC_UFS,
	    0, 2),
	MUX(MOUT_CLKCMU_HSI2_UFS_EMBD, "mout_clkcmu_hsi2_ufs_embd",
	    mout_clkcmu_hsi2_ufs_embd_p, CLK_CON_MUX_MUX_CLKCMU_HSI2_UFS_EMBD,
	    0, 2),
	MUX(MOUT_CLKCMU_HSI2_ETHERNET, "mout_clkcmu_hsi2_ethernet",
	    mout_clkcmu_hsi2_ethernet_p, CLK_CON_MUX_MUX_CLKCMU_HSI2_ETHERNET,
	    0, 2),

	/* ISP */
	MUX(MOUT_CLKCMU_ISP_NOC, "mout_clkcmu_isp_noc",
	    mout_clkcmu_isp_noc_p, CLK_CON_MUX_MUX_CLKCMU_ISP_NOC, 0, 3),

	/* M2M */
	MUX(MOUT_CLKCMU_M2M_NOC, "mout_clkcmu_m2m_noc",
	    mout_clkcmu_m2m_noc_p, CLK_CON_MUX_MUX_CLKCMU_M2M_NOC, 0, 2),
	MUX(MOUT_CLKCMU_M2M_JPEG, "mout_clkcmu_m2m_jpeg",
	    mout_clkcmu_m2m_jpeg_p, CLK_CON_MUX_MUX_CLKCMU_M2M_JPEG, 0, 2),

	/* MFC */
	MUX(MOUT_CLKCMU_MFC_MFC, "mout_clkcmu_mfc_mfc",
	    mout_clkcmu_mfc_mfc_p, CLK_CON_MUX_MUX_CLKCMU_MFC_MFC, 0, 2),
	MUX(MOUT_CLKCMU_MFC_WFD, "mout_clkcmu_mfc_wfd",
	    mout_clkcmu_mfc_wfd_p, CLK_CON_MUX_MUX_CLKCMU_MFC_WFD, 0, 2),

	/* MFD */
	MUX(MOUT_CLKCMU_MFD_NOC, "mout_clkcmu_mfd_noc",
	    mout_clkcmu_mfd_noc_p, CLK_CON_MUX_MUX_CLKCMU_MFD_NOC, 0, 3),

	/* MIF */
	MUX(MOUT_CLKCMU_MIF_SWITCH, "mout_clkcmu_mif_switch",
	    mout_clkcmu_mif_switch_p, CLK_CON_MUX_MUX_CLKCMU_MIF_SWITCH, 0, 3),
	MUX(MOUT_CLKCMU_MIF_NOCP, "mout_clkcmu_mif_nocp",
	    mout_clkcmu_mif_nocp_p, CLK_CON_MUX_MUX_CLKCMU_MIF_NOCP, 0, 2),

	/* MISC */
	MUX(MOUT_CLKCMU_MISC_NOC, "mout_clkcmu_misc_noc",
	    mout_clkcmu_misc_noc_p, CLK_CON_MUX_MUX_CLKCMU_MISC_NOC, 0, 2),

	/* NOCL0 */
	MUX(MOUT_CLKCMU_NOCL0_NOC, "mout_clkcmu_nocl0_noc",
	    mout_clkcmu_nocl0_noc_p, CLK_CON_MUX_MUX_CLKCMU_NOCL0_NOC, 0, 3),

	/* NOCL1 */
	MUX(MOUT_CLKCMU_NOCL1_NOC, "mout_clkcmu_nocl1_noc",
	    mout_clkcmu_nocl1_noc_p, CLK_CON_MUX_MUX_CLKCMU_NOCL1_NOC, 0, 3),

	/* NOCL2 */
	MUX(MOUT_CLKCMU_NOCL2_NOC, "mout_clkcmu_nocl2_noc",
	    mout_clkcmu_nocl2_noc_p, CLK_CON_MUX_MUX_CLKCMU_NOCL2_NOC, 0, 3),

	/* PERIC0 */
	MUX(MOUT_CLKCMU_PERIC0_NOC, "mout_clkcmu_peric0_noc",
	    mout_clkcmu_peric0_noc_p, CLK_CON_MUX_MUX_CLKCMU_PERIC0_NOC, 0, 1),
	MUX(MOUT_CLKCMU_PERIC0_IP, "mout_clkcmu_peric0_ip",
	    mout_clkcmu_peric0_ip_p, CLK_CON_MUX_MUX_CLKCMU_PERIC0_IP, 0, 1),

	/* PERIC1 */
	MUX(MOUT_CLKCMU_PERIC1_NOC, "mout_clkcmu_peric1_noc",
	    mout_clkcmu_peric1_noc_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_NOC, 0, 1),
	MUX(MOUT_CLKCMU_PERIC1_IP, "mout_clkcmu_peric1_ip",
	    mout_clkcmu_peric1_ip_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_IP, 0, 1),

	/* SDMA */
	MUX(MOUT_CLKCMU_SDMA_NOC, "mout_clkcmu_sdma_noc",
	    mout_clkcmu_sdma_noc_p, CLK_CON_MUX_MUX_CLKCMU_SDMA_NOC, 0, 3),

	/* SNW */
	MUX(MOUT_CLKCMU_SNW_NOC, "mout_clkcmu_snw_noc",
	    mout_clkcmu_snw_noc_p, CLK_CON_MUX_MUX_CLKCMU_SNW_NOC, 0, 3),

	/* SSP */
	MUX(MOUT_CLKCMU_SSP_NOC, "mout_clkcmu_ssp_noc",
	    mout_clkcmu_ssp_noc_p, CLK_CON_MUX_MUX_CLKCMU_SSP_NOC, 0, 2),

	/* TAA */
	MUX(MOUT_CLKCMU_TAA_NOC, "mout_clkcmu_taa_noc",
	    mout_clkcmu_taa_noc_p, CLK_CON_MUX_MUX_CLKCMU_TAA_NOC, 0, 3),
};

static top_div_clks: &[samsung_div_clock] = &[
	/* CMU_TOP_PURECLKCOMP */

	/* BOOST */
	DIV(DOUT_CLKCMU_CMU_BOOST, "dout_clkcmu_cmu_boost",
	    "mout_clkcmu_cmu_boost", CLK_CON_DIV_DIV_CLKCMU_CMU_BOOST, 0, 2),

	/* ACC */
	DIV(DOUT_CLKCMU_ACC_NOC, "dout_clkcmu_acc_noc",
	    "mout_clkcmu_acc_noc", CLK_CON_DIV_CLKCMU_ACC_NOC, 0, 4),
	DIV(DOUT_CLKCMU_ACC_ORB, "dout_clkcmu_acc_orb",
	    "mout_clkcmu_acc_orb", CLK_CON_DIV_CLKCMU_ACC_ORB, 0, 4),

	/* APM */
	DIV(DOUT_CLKCMU_APM_NOC, "dout_clkcmu_apm_noc",
	    "mout_clkcmu_apm_noc", CLK_CON_DIV_CLKCMU_APM_NOC, 0, 3),

	/* AUD */
	DIV(DOUT_CLKCMU_AUD_CPU, "dout_clkcmu_aud_cpu",
	    "mout_clkcmu_aud_cpu", CLK_CON_DIV_CLKCMU_AUD_CPU, 0, 3),
	DIV(DOUT_CLKCMU_AUD_NOC, "dout_clkcmu_aud_noc",
	    "mout_clkcmu_aud_noc", CLK_CON_DIV_CLKCMU_AUD_NOC, 0, 4),

	/* CPUCL0 */
	DIV(DOUT_CLKCMU_CPUCL0_SWITCH, "dout_clkcmu_cpucl0_switch",
	    "mout_clkcmu_cpucl0_switch",
	    CLK_CON_DIV_CLKCMU_CPUCL0_SWITCH, 0, 3),
	DIV(DOUT_CLKCMU_CPUCL0_CLUSTER, "dout_clkcmu_cpucl0_cluster",
	    "mout_clkcmu_cpucl0_cluster",
	    CLK_CON_DIV_CLKCMU_CPUCL0_CLUSTER, 0, 3),
	DIV(DOUT_CLKCMU_CPUCL0_DBG, "dout_clkcmu_cpucl0_dbg",
	    "mout_clkcmu_cpucl0_dbg",
	    CLK_CON_DIV_CLKCMU_CPUCL0_DBG, 0, 4),

	/* CPUCL1 */
	DIV(DOUT_CLKCMU_CPUCL1_SWITCH, "dout_clkcmu_cpucl1_switch",
	    "mout_clkcmu_cpucl1_switch",
	    CLK_CON_DIV_CLKCMU_CPUCL1_SWITCH, 0, 3),
	DIV(DOUT_CLKCMU_CPUCL1_CLUSTER, "dout_clkcmu_cpucl1_cluster",
	    "mout_clkcmu_cpucl1_cluster",
	    CLK_CON_DIV_CLKCMU_CPUCL1_CLUSTER, 0, 3),

	/* CPUCL2 */
	DIV(DOUT_CLKCMU_CPUCL2_SWITCH, "dout_clkcmu_cpucl2_switch",
	    "mout_clkcmu_cpucl2_switch",
	    CLK_CON_DIV_CLKCMU_CPUCL2_SWITCH, 0, 3),
	DIV(DOUT_CLKCMU_CPUCL2_CLUSTER, "dout_clkcmu_cpucl2_cluster",
	    "mout_clkcmu_cpucl2_cluster",
	    CLK_CON_DIV_CLKCMU_CPUCL2_CLUSTER, 0, 3),

	/* DNC */
	DIV(DOUT_CLKCMU_DNC_NOC, "dout_clkcmu_dnc_noc",
	    "mout_clkcmu_dnc_noc", CLK_CON_DIV_CLKCMU_DNC_NOC, 0, 4),

	/* DPTX */
	DIV(DOUT_CLKCMU_DPTX_NOC, "dout_clkcmu_dptx_noc",
	    "mout_clkcmu_dptx_noc", CLK_CON_DIV_CLKCMU_DPTX_NOC, 0, 4),
	DIV(DOUT_CLKCMU_DPTX_DPGTC, "dout_clkcmu_dptx_dpgtc",
	    "mout_clkcmu_dptx_dpgtc", CLK_CON_DIV_CLKCMU_DPTX_DPGTC, 0, 3),
	DIV(DOUT_CLKCMU_DPTX_DPOSC, "dout_clkcmu_dptx_dposc",
	    "mout_clkcmu_dptx_dposc", CLK_CON_DIV_CLKCMU_DPTX_DPOSC, 0, 5),

	/* DPUB */
	DIV(DOUT_CLKCMU_DPUB_NOC, "dout_clkcmu_dpub_noc",
	    "mout_clkcmu_dpub_noc", CLK_CON_DIV_CLKCMU_DPUB_NOC, 0, 4),
	DIV(DOUT_CLKCMU_DPUB_DSIM, "dout_clkcmu_dpub_dsim",
	    "mout_clkcmu_dpub_dsim", CLK_CON_DIV_CLKCMU_DPUB_DSIM, 0, 4),

	/* DPUF */
	DIV(DOUT_CLKCMU_DPUF0_NOC, "dout_clkcmu_dpuf0_noc",
	    "mout_clkcmu_dpuf0_noc", CLK_CON_DIV_CLKCMU_DPUF0_NOC, 0, 4),
	DIV(DOUT_CLKCMU_DPUF1_NOC, "dout_clkcmu_dpuf1_noc",
	    "mout_clkcmu_dpuf1_noc", CLK_CON_DIV_CLKCMU_DPUF1_NOC, 0, 4),
	DIV(DOUT_CLKCMU_DPUF2_NOC, "dout_clkcmu_dpuf2_noc",
	    "mout_clkcmu_dpuf2_noc", CLK_CON_DIV_CLKCMU_DPUF2_NOC, 0, 4),

	/* DSP */
	DIV(DOUT_CLKCMU_DSP_NOC, "dout_clkcmu_dsp_noc",
	    "mout_clkcmu_dsp_noc", CLK_CON_DIV_CLKCMU_DSP_NOC, 0, 4),

	/* G3D */
	DIV(DOUT_CLKCMU_G3D_SWITCH, "dout_clkcmu_g3d_switch",
	    "mout_clkcmu_g3d_switch", CLK_CON_DIV_CLKCMU_G3D_SWITCH, 0, 3),
	DIV(DOUT_CLKCMU_G3D_NOCP, "dout_clkcmu_g3d_nocp",
	    "mout_clkcmu_g3d_nocp", CLK_CON_DIV_CLKCMU_G3D_NOCP, 0, 3),

	/* GNPU */
	DIV(DOUT_CLKCMU_GNPU_NOC, "dout_clkcmu_gnpu_noc",
	    "mout_clkcmu_gnpu_noc", CLK_CON_DIV_CLKCMU_GNPU_NOC, 0, 4),

	/* HSI0 */
	DIV(DOUT_CLKCMU_HSI0_NOC, "dout_clkcmu_hsi0_noc",
	    "mout_clkcmu_hsi0_noc", CLK_CON_DIV_CLKCMU_HSI0_NOC, 0, 4),

	/* HSI1 */
	DIV(DOUT_CLKCMU_HSI1_NOC, "dout_clkcmu_hsi1_noc",
	    "mout_clkcmu_hsi1_noc", CLK_CON_DIV_CLKCMU_HSI1_NOC, 0, 4),
	DIV(DOUT_CLKCMU_HSI1_USBDRD, "dout_clkcmu_hsi1_usbdrd",
	    "mout_clkcmu_hsi1_usbdrd", CLK_CON_DIV_CLKCMU_HSI1_USBDRD, 0, 4),
	DIV(DOUT_CLKCMU_HSI1_MMC_CARD, "dout_clkcmu_hsi1_mmc_card",
	    "mout_clkcmu_hsi1_mmc_card", CLK_CON_DIV_CLKCMU_HSI1_MMC_CARD, 0, 9),

	/* HSI2 */
	DIV(DOUT_CLKCMU_HSI2_NOC, "dout_clkcmu_hsi2_noc",
	    "mout_clkcmu_hsi2_noc", CLK_CON_DIV_CLKCMU_HSI2_NOC, 0, 4),
	DIV(DOUT_CLKCMU_HSI2_NOC_UFS, "dout_clkcmu_hsi2_noc_ufs",
	    "mout_clkcmu_hsi2_noc_ufs", CLK_CON_DIV_CLKCMU_HSI2_NOC_UFS, 0, 4),
	DIV(DOUT_CLKCMU_HSI2_UFS_EMBD, "dout_clkcmu_hsi2_ufs_embd",
	    "mout_clkcmu_hsi2_ufs_embd", CLK_CON_DIV_CLKCMU_HSI2_UFS_EMBD, 0, 3),
	DIV(DOUT_CLKCMU_HSI2_ETHERNET, "dout_clkcmu_hsi2_ethernet",
	    "mout_clkcmu_hsi2_ethernet", CLK_CON_DIV_CLKCMU_HSI2_ETHERNET, 0, 3),

	/* ISP */
	DIV(DOUT_CLKCMU_ISP_NOC, "dout_clkcmu_isp_noc",
	    "mout_clkcmu_isp_noc", CLK_CON_DIV_CLKCMU_ISP_NOC, 0, 4),

	/* M2M */
	DIV(DOUT_CLKCMU_M2M_NOC, "dout_clkcmu_m2m_noc",
	    "mout_clkcmu_m2m_noc", CLK_CON_DIV_CLKCMU_M2M_NOC, 0, 4),
	DIV(DOUT_CLKCMU_M2M_JPEG, "dout_clkcmu_m2m_jpeg",
	    "mout_clkcmu_m2m_jpeg", CLK_CON_DIV_CLKCMU_M2M_JPEG, 0, 4),

	/* MFC */
	DIV(DOUT_CLKCMU_MFC_MFC, "dout_clkcmu_mfc_mfc",
	    "mout_clkcmu_mfc_mfc", CLK_CON_DIV_CLKCMU_MFC_MFC, 0, 4),
	DIV(DOUT_CLKCMU_MFC_WFD, "dout_clkcmu_mfc_wfd",
	    "mout_clkcmu_mfc_wfd", CLK_CON_DIV_CLKCMU_MFC_WFD, 0, 4),

	/* MFD */
	DIV(DOUT_CLKCMU_MFD_NOC, "dout_clkcmu_mfd_noc",
	    "mout_clkcmu_mfd_noc", CLK_CON_DIV_CLKCMU_MFD_NOC, 0, 4),

	/* MIF */
	DIV(DOUT_CLKCMU_MIF_NOCP, "dout_clkcmu_mif_nocp",
	    "mout_clkcmu_mif_nocp", CLK_CON_DIV_CLKCMU_MIF_NOCP, 0, 4),

	/* MISC */
	DIV(DOUT_CLKCMU_MISC_NOC, "dout_clkcmu_misc_noc",
	    "mout_clkcmu_misc_noc", CLK_CON_DIV_CLKCMU_MISC_NOC, 0, 4),

	/* NOCL0 */
	DIV(DOUT_CLKCMU_NOCL0_NOC, "dout_clkcmu_nocl0_noc",
	    "mout_clkcmu_nocl0_noc", CLK_CON_DIV_CLKCMU_NOCL0_NOC, 0, 4),

	/* NOCL1 */
	DIV(DOUT_CLKCMU_NOCL1_NOC, "dout_clkcmu_nocl1_noc",
	    "mout_clkcmu_nocl1_noc", CLK_CON_DIV_CLKCMU_NOCL1_NOC, 0, 4),

	/* NOCL2 */
	DIV(DOUT_CLKCMU_NOCL2_NOC, "dout_clkcmu_nocl2_noc",
	    "mout_clkcmu_nocl2_noc", CLK_CON_DIV_CLKCMU_NOCL2_NOC, 0, 4),

	/* PERIC0 */
	DIV(DOUT_CLKCMU_PERIC0_NOC, "dout_clkcmu_peric0_noc",
	    "mout_clkcmu_peric0_noc", CLK_CON_DIV_CLKCMU_PERIC0_NOC, 0, 4),
	DIV(DOUT_CLKCMU_PERIC0_IP, "dout_clkcmu_peric0_ip",
	    "mout_clkcmu_peric0_ip", CLK_CON_DIV_CLKCMU_PERIC0_IP, 0, 4),

	/* PERIC1 */
	DIV(DOUT_CLKCMU_PERIC1_NOC, "dout_clkcmu_peric1_noc",
	    "mout_clkcmu_peric1_noc", CLK_CON_DIV_CLKCMU_PERIC1_NOC, 0, 4),
	DIV(DOUT_CLKCMU_PERIC1_IP, "dout_clkcmu_peric1_ip",
	    "mout_clkcmu_peric1_ip", CLK_CON_DIV_CLKCMU_PERIC1_IP, 0, 4),

	/* SDMA */
	DIV(DOUT_CLKCMU_SDMA_NOC, "dout_clkcmu_sdma_noc",
	    "mout_clkcmu_sdma_noc", CLK_CON_DIV_CLKCMU_SDMA_NOC, 0, 4),

	/* SNW */
	DIV(DOUT_CLKCMU_SNW_NOC, "dout_clkcmu_snw_noc",
	    "mout_clkcmu_snw_noc", CLK_CON_DIV_CLKCMU_SNW_NOC, 0, 4),

	/* SSP */
	DIV(DOUT_CLKCMU_SSP_NOC, "dout_clkcmu_ssp_noc",
	    "mout_clkcmu_ssp_noc", CLK_CON_DIV_CLKCMU_SSP_NOC, 0, 4),

	/* TAA */
	DIV(DOUT_CLKCMU_TAA_NOC, "dout_clkcmu_taa_noc",
	    "mout_clkcmu_taa_noc", CLK_CON_DIV_CLKCMU_TAA_NOC, 0, 4),
};

static top_fixed_factor_clks: &[samsung_fixed_factor_clock] = &[
	FFACTOR(DOUT_SHARED0_DIV1, "dout_shared0_div1",
		"mout_shared0_pll", 1, 1, 0),
	FFACTOR(DOUT_SHARED0_DIV2, "dout_shared0_div2",
		"mout_shared0_pll", 1, 2, 0),
	FFACTOR(DOUT_SHARED0_DIV3, "dout_shared0_div3",
		"mout_shared0_pll", 1, 3, 0),
	FFACTOR(DOUT_SHARED0_DIV4, "dout_shared0_div4",
		"mout_shared0_pll", 1, 4, 0),
	FFACTOR(DOUT_SHARED1_DIV1, "dout_shared1_div1",
		"mout_shared1_pll", 1, 1, 0),
	FFACTOR(DOUT_SHARED1_DIV2, "dout_shared1_div2",
		"mout_shared1_pll", 1, 2, 0),
	FFACTOR(DOUT_SHARED1_DIV3, "dout_shared1_div3",
		"mout_shared1_pll", 1, 3, 0),
	FFACTOR(DOUT_SHARED1_DIV4, "dout_shared1_div4",
		"mout_shared1_pll", 1, 4, 0),
	FFACTOR(DOUT_SHARED2_DIV1, "dout_shared2_div1",
		"mout_shared2_pll", 1, 1, 0),
	FFACTOR(DOUT_SHARED2_DIV2, "dout_shared2_div2",
		"mout_shared2_pll", 1, 2, 0),
	FFACTOR(DOUT_SHARED2_DIV3, "dout_shared2_div3",
		"mout_shared2_pll", 1, 3, 0),
	FFACTOR(DOUT_SHARED2_DIV4, "dout_shared2_div4",
		"mout_shared2_pll", 1, 4, 0),
	FFACTOR(DOUT_SHARED3_DIV1, "dout_shared3_div1",
		"mout_shared3_pll", 1, 1, 0),
	FFACTOR(DOUT_SHARED3_DIV2, "dout_shared3_div2",
		"mout_shared3_pll", 1, 2, 0),
	FFACTOR(DOUT_SHARED3_DIV3, "dout_shared3_div3",
		"mout_shared3_pll", 1, 3, 0),
	FFACTOR(DOUT_SHARED3_DIV4, "dout_shared3_div4",
		"mout_shared3_pll", 1, 4, 0),
	FFACTOR(DOUT_SHARED4_DIV1, "dout_shared4_div1",
		"mout_shared4_pll", 1, 1, 0),
	FFACTOR(DOUT_SHARED4_DIV2, "dout_shared4_div2",
		"mout_shared4_pll", 1, 2, 0),
	FFACTOR(DOUT_SHARED4_DIV3, "dout_shared4_div3",
		"mout_shared4_pll", 1, 3, 0),
	FFACTOR(DOUT_SHARED4_DIV4, "dout_shared4_div4",
		"mout_shared4_pll", 1, 4, 0),
	FFACTOR(DOUT_SHARED5_DIV1, "dout_shared5_div1",
		"mout_shared5_pll", 1, 1, 0),
	FFACTOR(DOUT_SHARED5_DIV2, "dout_shared5_div2",
		"mout_shared5_pll", 1, 2, 0),
	FFACTOR(DOUT_SHARED5_DIV3, "dout_shared5_div3",
		"mout_shared5_pll", 1, 3, 0),
	FFACTOR(DOUT_SHARED5_DIV4, "dout_shared5_div4",
		"mout_shared5_pll", 1, 4, 0),
	FFACTOR(DOUT_TCXO_DIV2, "dout_tcxo_div2",
		"oscclk", 1, 2, 0),
};

static top_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.pll_clks		= top_pll_clks,
	.nr_pll_clks		= top_pll_clks.len(),
	.mux_clks		= top_mux_clks,
	.nr_mux_clks		= top_mux_clks.len(),
	.div_clks		= top_div_clks,
	.nr_div_clks		= top_div_clks.len(),
	.fixed_factor_clks	= top_fixed_factor_clks,
	.nr_fixed_factor_clks	= top_fixed_factor_clks.len(),
	.nr_clk_ids		= CLKS_NR_TOP,
	.clk_regs		= top_clk_regs,
	.nr_clk_regs		= top_clk_regs.len(),
};

static void __init exynosautov920_cmu_top_init(struct device_node *np)
{
	exynos_arm64_register_cmu(core::ptr::null_mut(), np, &top_cmu_info);
}

/* Register CMU_TOP early, as it's a dependency for other early domains */
CLK_OF_DECLARE(exynosautov920_cmu_top, "samsung,exynosautov920-cmu-top",
	       exynosautov920_cmu_top_init);

/* ---- CMU_CPUCL0 --------------------------------------------------------- */

/* Register Offset definitions for CMU_CPUCL0 (0x1EC00000) */
const PLL_LOCKTIME_PLL_CPUCL0: usize = 0x0000;
const PLL_CON0_PLL_CPUCL0: usize = 0x0100;
const PLL_CON1_PLL_CPUCL0: usize = 0x0104;
const PLL_CON3_PLL_CPUCL0: usize = 0x010c;
const PLL_CON0_MUX_CLKCMU_CPUCL0_CLUSTER_USER: usize = 0x0600;
const PLL_CON0_MUX_CLKCMU_CPUCL0_DBG_USER: usize = 0x0610;
const PLL_CON0_MUX_CLKCMU_CPUCL0_SWITCH_USER: usize = 0x0620;

const CLK_CON_MUX_MUX_CLK_CPUCL0_CLUSTER: usize = 0x1000;
const CLK_CON_MUX_MUX_CLK_CPUCL0_CORE: usize = 0x1004;

const CLK_CON_DIV_DIV_CLK_CLUSTER0_ACLK: usize = 0x1800;
const CLK_CON_DIV_DIV_CLK_CLUSTER0_ATCLK: usize = 0x1804;
const CLK_CON_DIV_DIV_CLK_CLUSTER0_MPCLK: usize = 0x1808;
const CLK_CON_DIV_DIV_CLK_CLUSTER0_PCLK: usize = 0x180c;
const CLK_CON_DIV_DIV_CLK_CLUSTER0_PERIPHCLK: usize = 0x1810;
const CLK_CON_DIV_DIV_CLK_CPUCL0_DBG_NOC: usize = 0x181c;
const CLK_CON_DIV_DIV_CLK_CPUCL0_DBG_PCLKDBG: usize = 0x1820;
const CLK_CON_DIV_DIV_CLK_CPUCL0_NOCP: usize = 0x1824;

static cpucl0_clk_regs: &[usize] = &[
	PLL_LOCKTIME_PLL_CPUCL0,
	PLL_CON0_PLL_CPUCL0,
	PLL_CON1_PLL_CPUCL0,
	PLL_CON3_PLL_CPUCL0,
	PLL_CON0_MUX_CLKCMU_CPUCL0_CLUSTER_USER,
	PLL_CON0_MUX_CLKCMU_CPUCL0_DBG_USER,
	PLL_CON0_MUX_CLKCMU_CPUCL0_SWITCH_USER,
	CLK_CON_MUX_MUX_CLK_CPUCL0_CLUSTER,
	CLK_CON_MUX_MUX_CLK_CPUCL0_CORE,
	CLK_CON_DIV_DIV_CLK_CLUSTER0_ACLK,
	CLK_CON_DIV_DIV_CLK_CLUSTER0_ATCLK,
	CLK_CON_DIV_DIV_CLK_CLUSTER0_MPCLK,
	CLK_CON_DIV_DIV_CLK_CLUSTER0_PCLK,
	CLK_CON_DIV_DIV_CLK_CLUSTER0_PERIPHCLK,
	CLK_CON_DIV_DIV_CLK_CPUCL0_DBG_NOC,
	CLK_CON_DIV_DIV_CLK_CPUCL0_DBG_PCLKDBG,
	CLK_CON_DIV_DIV_CLK_CPUCL0_NOCP,
};

/* List of parent clocks for Muxes in CMU_CPUCL0 */
static mout_pll_cpucl0_p: &[&str] = &[ "oscclk", "fout_cpucl0_pll" };
static mout_cpucl0_cluster_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_cpucl0_cluster" };
static mout_cpucl0_dbg_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_cpucl0_dbg" };
static mout_cpucl0_switch_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_cpucl0_switch" };
static mout_cpucl0_cluster_p: &[&str] = &[ "oscclk", "mout_cpucl0_cluster_user",
						"mout_cpucl0_switch_user"};
static mout_cpucl0_core_p: &[&str] = &[ "oscclk", "mout_pll_cpucl0",
						"mout_cpucl0_switch_user"};

static cpu_pll_rates: &[samsung_pll_rate_table] = &[
	PLL_35XX_RATE(38400000U, 2400000000U, 250, 4, 0),
	PLL_35XX_RATE(38400000U, 2304000000U, 240, 4, 0),
	PLL_35XX_RATE(38400000U, 2208000000U, 230, 4, 0),
	PLL_35XX_RATE(38400000U, 2112000000U, 220, 4, 0),
	PLL_35XX_RATE(38400000U, 2016000000U, 210, 4, 0),
	PLL_35XX_RATE(38400000U, 1824000000U, 190, 4, 0),
	PLL_35XX_RATE(38400000U, 1680000000U, 175, 4, 0),
	PLL_35XX_RATE(38400000U, 1344000000U, 140, 4, 0),
	PLL_35XX_RATE(38400000U, 1152000000U, 120, 4, 0),
	PLL_35XX_RATE(38400000U, 576000000U, 120, 4, 1),
	PLL_35XX_RATE(38400000U, 288000000U, 120, 4, 2),
};

static cpucl0_pll_clks: &[samsung_pll_clock] = &[
	/* CMU_CPUCL0_PURECLKCOMP */
	PLL(pll_531x, CLK_FOUT_CPUCL0_PLL, "fout_cpucl0_pll", "oscclk",
	    PLL_LOCKTIME_PLL_CPUCL0, PLL_CON3_PLL_CPUCL0, cpu_pll_rates),
};

static cpucl0_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_PLL_CPUCL0, "mout_pll_cpucl0", mout_pll_cpucl0_p,
	    PLL_CON0_PLL_CPUCL0, 4, 1),
	MUX(CLK_MOUT_CPUCL0_CLUSTER_USER, "mout_cpucl0_cluster_user", mout_cpucl0_cluster_user_p,
	    PLL_CON0_MUX_CLKCMU_CPUCL0_CLUSTER_USER, 4, 1),
	MUX(CLK_MOUT_CPUCL0_DBG_USER, "mout_cpucl0_dbg_user", mout_cpucl0_dbg_user_p,
	    PLL_CON0_MUX_CLKCMU_CPUCL0_DBG_USER, 4, 1),
	MUX(CLK_MOUT_CPUCL0_SWITCH_USER, "mout_cpucl0_switch_user", mout_cpucl0_switch_user_p,
	    PLL_CON0_MUX_CLKCMU_CPUCL0_SWITCH_USER, 4, 1),
	MUX(CLK_MOUT_CPUCL0_CLUSTER, "mout_cpucl0_cluster", mout_cpucl0_cluster_p,
	    CLK_CON_MUX_MUX_CLK_CPUCL0_CLUSTER, 0, 2),
	MUX(CLK_MOUT_CPUCL0_CORE, "mout_cpucl0_core", mout_cpucl0_core_p,
	    CLK_CON_MUX_MUX_CLK_CPUCL0_CORE, 0, 2),
};

static cpucl0_div_clks: &[samsung_div_clock] = &[
	DIV(CLK_DOUT_CLUSTER0_ACLK, "dout_cluster0_aclk",
	    "mout_cpucl0_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER0_ACLK, 0, 4),
	DIV(CLK_DOUT_CLUSTER0_ATCLK, "dout_cluster0_atclk",
	    "mout_cpucl0_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER0_ATCLK, 0, 4),
	DIV(CLK_DOUT_CLUSTER0_MPCLK, "dout_cluster0_mpclk",
	    "mout_cpucl0_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER0_MPCLK, 0, 4),
	DIV(CLK_DOUT_CLUSTER0_PCLK, "dout_cluster0_pclk",
	    "mout_cpucl0_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER0_PCLK, 0, 4),
	DIV(CLK_DOUT_CLUSTER0_PERIPHCLK, "dout_cluster0_periphclk",
	    "mout_cpucl0_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER0_PERIPHCLK, 0, 4),
	DIV(CLK_DOUT_CPUCL0_DBG_NOC, "dout_cpucl0_dbg_noc",
	    "mout_cpucl0_dbg_user", CLK_CON_DIV_DIV_CLK_CPUCL0_DBG_NOC, 0, 3),
	DIV(CLK_DOUT_CPUCL0_DBG_PCLKDBG, "dout_cpucl0_dbg_pclkdbg",
	    "mout_cpucl0_dbg_user", CLK_CON_DIV_DIV_CLK_CPUCL0_DBG_PCLKDBG, 0, 3),
	DIV(CLK_DOUT_CPUCL0_NOCP, "dout_cpucl0_nocp",
	    "mout_cpucl0_cluster", CLK_CON_DIV_DIV_CLK_CPUCL0_NOCP, 0, 4),
};

static cpucl0_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.pll_clks		= cpucl0_pll_clks,
	.nr_pll_clks		= cpucl0_pll_clks.len(),
	.mux_clks		= cpucl0_mux_clks,
	.nr_mux_clks		= cpucl0_mux_clks.len(),
	.div_clks		= cpucl0_div_clks,
	.nr_div_clks		= cpucl0_div_clks.len(),
	.nr_clk_ids		= CLKS_NR_CPUCL0,
	.clk_regs		= cpucl0_clk_regs,
	.nr_clk_regs		= cpucl0_clk_regs.len(),
	.clk_name		= "cpucl0",
};

static void __init exynosautov920_cmu_cpucl0_init(struct device_node *np)
{
	exynos_arm64_register_cmu(core::ptr::null_mut(), np, &cpucl0_cmu_info);
}

/* Register CMU_CPUCL0 early, as CPU clocks should be available ASAP */
CLK_OF_DECLARE(exynosautov920_cmu_cpucl0, "samsung,exynosautov920-cmu-cpucl0",
	       exynosautov920_cmu_cpucl0_init);

/* ---- CMU_CPUCL1 --------------------------------------------------------- */

/* Register Offset definitions for CMU_CPUCL1 (0x1ED00000) */
const PLL_LOCKTIME_PLL_CPUCL1: usize = 0x0000;
const PLL_CON0_PLL_CPUCL1: usize = 0x0100;
const PLL_CON1_PLL_CPUCL1: usize = 0x0104;
const PLL_CON3_PLL_CPUCL1: usize = 0x010c;
const PLL_CON0_MUX_CLKCMU_CPUCL1_CLUSTER_USER: usize = 0x0600;
const PLL_CON0_MUX_CLKCMU_CPUCL1_SWITCH_USER: usize = 0x0610;

const CLK_CON_MUX_MUX_CLK_CPUCL1_CLUSTER: usize = 0x1000;
const CLK_CON_MUX_MUX_CLK_CPUCL1_CORE: usize = 0x1004;

const CLK_CON_DIV_DIV_CLK_CLUSTER1_ACLK: usize = 0x1800;
const CLK_CON_DIV_DIV_CLK_CLUSTER1_ATCLK: usize = 0x1804;
const CLK_CON_DIV_DIV_CLK_CLUSTER1_MPCLK: usize = 0x1808;
const CLK_CON_DIV_DIV_CLK_CLUSTER1_PCLK: usize = 0x180c;
const CLK_CON_DIV_DIV_CLK_CLUSTER1_PERIPHCLK: usize = 0x1810;
const CLK_CON_DIV_DIV_CLK_CPUCL1_NOCP: usize = 0x181c;

static cpucl1_clk_regs: &[usize] = &[
	PLL_LOCKTIME_PLL_CPUCL1,
	PLL_CON0_PLL_CPUCL1,
	PLL_CON1_PLL_CPUCL1,
	PLL_CON3_PLL_CPUCL1,
	PLL_CON0_MUX_CLKCMU_CPUCL1_CLUSTER_USER,
	PLL_CON0_MUX_CLKCMU_CPUCL1_SWITCH_USER,
	CLK_CON_MUX_MUX_CLK_CPUCL1_CLUSTER,
	CLK_CON_MUX_MUX_CLK_CPUCL1_CORE,
	CLK_CON_DIV_DIV_CLK_CLUSTER1_ACLK,
	CLK_CON_DIV_DIV_CLK_CLUSTER1_ATCLK,
	CLK_CON_DIV_DIV_CLK_CLUSTER1_MPCLK,
	CLK_CON_DIV_DIV_CLK_CLUSTER1_PCLK,
	CLK_CON_DIV_DIV_CLK_CLUSTER1_PERIPHCLK,
	CLK_CON_DIV_DIV_CLK_CPUCL1_NOCP,
};

/* List of parent clocks for Muxes in CMU_CPUCL1 */
static mout_pll_cpucl1_p: &[&str] = &[ "oscclk", "fout_cpucl1_pll" };
static mout_cpucl1_cluster_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_cpucl1_cluster" };
static mout_cpucl1_switch_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_cpucl1_switch" };
static mout_cpucl1_cluster_p: &[&str] = &[ "oscclk", "mout_cpucl1_cluster_user",
						"mout_cpucl1_switch_user"};
static mout_cpucl1_core_p: &[&str] = &[ "oscclk", "mout_pll_cpucl1",
						"mout_cpucl1_switch_user"};

static cpucl1_pll_clks: &[samsung_pll_clock] = &[
	/* CMU_CPUCL1_PURECLKCOMP */
	PLL(pll_531x, CLK_FOUT_CPUCL1_PLL, "fout_cpucl1_pll", "oscclk",
	    PLL_LOCKTIME_PLL_CPUCL1, PLL_CON3_PLL_CPUCL1, cpu_pll_rates),
};

static cpucl1_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_PLL_CPUCL1, "mout_pll_cpucl1", mout_pll_cpucl1_p,
	    PLL_CON0_PLL_CPUCL1, 4, 1),
	MUX(CLK_MOUT_CPUCL1_CLUSTER_USER, "mout_cpucl1_cluster_user", mout_cpucl1_cluster_user_p,
	    PLL_CON0_MUX_CLKCMU_CPUCL1_CLUSTER_USER, 4, 1),
	MUX(CLK_MOUT_CPUCL1_SWITCH_USER, "mout_cpucl1_switch_user", mout_cpucl1_switch_user_p,
	    PLL_CON0_MUX_CLKCMU_CPUCL1_SWITCH_USER, 4, 1),
	MUX(CLK_MOUT_CPUCL1_CLUSTER, "mout_cpucl1_cluster", mout_cpucl1_cluster_p,
	    CLK_CON_MUX_MUX_CLK_CPUCL1_CLUSTER, 0, 2),
	MUX(CLK_MOUT_CPUCL1_CORE, "mout_cpucl1_core", mout_cpucl1_core_p,
	    CLK_CON_MUX_MUX_CLK_CPUCL1_CORE, 0, 2),
};

static cpucl1_div_clks: &[samsung_div_clock] = &[
	DIV(CLK_DOUT_CLUSTER1_ACLK, "dout_cluster1_aclk",
	    "mout_cpucl1_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER1_ACLK, 0, 4),
	DIV(CLK_DOUT_CLUSTER1_ATCLK, "dout_cluster1_atclk",
	    "mout_cpucl1_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER1_ATCLK, 0, 4),
	DIV(CLK_DOUT_CLUSTER1_MPCLK, "dout_cluster1_mpclk",
	    "mout_cpucl1_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER1_MPCLK, 0, 4),
	DIV(CLK_DOUT_CLUSTER1_PCLK, "dout_cluster1_pclk",
	    "mout_cpucl1_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER1_PCLK, 0, 4),
	DIV(CLK_DOUT_CLUSTER1_PERIPHCLK, "dout_cluster1_periphclk",
	    "mout_cpucl1_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER1_PERIPHCLK, 0, 4),
	DIV(CLK_DOUT_CPUCL1_NOCP, "dout_cpucl1_nocp",
	    "mout_cpucl1_cluster", CLK_CON_DIV_DIV_CLK_CPUCL1_NOCP, 0, 4),
};

static cpucl1_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.pll_clks		= cpucl1_pll_clks,
	.nr_pll_clks		= cpucl1_pll_clks.len(),
	.mux_clks		= cpucl1_mux_clks,
	.nr_mux_clks		= cpucl1_mux_clks.len(),
	.div_clks		= cpucl1_div_clks,
	.nr_div_clks		= cpucl1_div_clks.len(),
	.nr_clk_ids		= CLKS_NR_CPUCL1,
	.clk_regs		= cpucl1_clk_regs,
	.nr_clk_regs		= cpucl1_clk_regs.len(),
	.clk_name		= "cpucl1",
};

static void __init exynosautov920_cmu_cpucl1_init(struct device_node *np)
{
	exynos_arm64_register_cmu(core::ptr::null_mut(), np, &cpucl1_cmu_info);
}

/* Register CMU_CPUCL1 early, as CPU clocks should be available ASAP */
CLK_OF_DECLARE(exynosautov920_cmu_cpucl1, "samsung,exynosautov920-cmu-cpucl1",
	       exynosautov920_cmu_cpucl1_init);

/* ---- CMU_CPUCL2 --------------------------------------------------------- */

/* Register Offset definitions for CMU_CPUCL2 (0x1EE00000) */
const PLL_LOCKTIME_PLL_CPUCL2: usize = 0x0000;
const PLL_CON0_PLL_CPUCL2: usize = 0x0100;
const PLL_CON1_PLL_CPUCL2: usize = 0x0104;
const PLL_CON3_PLL_CPUCL2: usize = 0x010c;
const PLL_CON0_MUX_CLKCMU_CPUCL2_CLUSTER_USER: usize = 0x0600;
const PLL_CON0_MUX_CLKCMU_CPUCL2_SWITCH_USER: usize = 0x0610;

const CLK_CON_MUX_MUX_CLK_CPUCL2_CLUSTER: usize = 0x1000;
const CLK_CON_MUX_MUX_CLK_CPUCL2_CORE: usize = 0x1004;

const CLK_CON_DIV_DIV_CLK_CLUSTER2_ACLK: usize = 0x1800;
const CLK_CON_DIV_DIV_CLK_CLUSTER2_ATCLK: usize = 0x1804;
const CLK_CON_DIV_DIV_CLK_CLUSTER2_MPCLK: usize = 0x1808;
const CLK_CON_DIV_DIV_CLK_CLUSTER2_PCLK: usize = 0x180c;
const CLK_CON_DIV_DIV_CLK_CLUSTER2_PERIPHCLK: usize = 0x1810;
const CLK_CON_DIV_DIV_CLK_CPUCL2_NOCP: usize = 0x181c;

static cpucl2_clk_regs: &[usize] = &[
	PLL_LOCKTIME_PLL_CPUCL2,
	PLL_CON0_PLL_CPUCL2,
	PLL_CON1_PLL_CPUCL2,
	PLL_CON3_PLL_CPUCL2,
	PLL_CON0_MUX_CLKCMU_CPUCL2_CLUSTER_USER,
	PLL_CON0_MUX_CLKCMU_CPUCL2_SWITCH_USER,
	CLK_CON_MUX_MUX_CLK_CPUCL2_CLUSTER,
	CLK_CON_MUX_MUX_CLK_CPUCL2_CORE,
	CLK_CON_DIV_DIV_CLK_CLUSTER2_ACLK,
	CLK_CON_DIV_DIV_CLK_CLUSTER2_ATCLK,
	CLK_CON_DIV_DIV_CLK_CLUSTER2_MPCLK,
	CLK_CON_DIV_DIV_CLK_CLUSTER2_PCLK,
	CLK_CON_DIV_DIV_CLK_CLUSTER2_PERIPHCLK,
	CLK_CON_DIV_DIV_CLK_CPUCL2_NOCP,
};

/* List of parent clocks for Muxes in CMU_CPUCL2 */
static mout_pll_cpucl2_p: &[&str] = &[ "oscclk", "fout_cpucl2_pll" };
static mout_cpucl2_cluster_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_cpucl2_cluster" };
static mout_cpucl2_switch_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_cpucl2_switch" };
static mout_cpucl2_cluster_p: &[&str] = &[ "oscclk", "mout_cpucl2_cluster_user",
						"mout_cpucl2_switch_user"};
static mout_cpucl2_core_p: &[&str] = &[ "oscclk", "mout_pll_cpucl2",
						"mout_cpucl2_switch_user"};

static cpucl2_pll_clks: &[samsung_pll_clock] = &[
	/* CMU_CPUCL2_PURECLKCOMP */
	PLL(pll_531x, CLK_FOUT_CPUCL2_PLL, "fout_cpucl2_pll", "oscclk",
	    PLL_LOCKTIME_PLL_CPUCL2, PLL_CON3_PLL_CPUCL2, cpu_pll_rates),
};

static cpucl2_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_PLL_CPUCL2, "mout_pll_cpucl2", mout_pll_cpucl2_p,
	    PLL_CON0_PLL_CPUCL2, 4, 1),
	MUX(CLK_MOUT_CPUCL2_CLUSTER_USER, "mout_cpucl2_cluster_user", mout_cpucl2_cluster_user_p,
	    PLL_CON0_MUX_CLKCMU_CPUCL2_CLUSTER_USER, 4, 1),
	MUX(CLK_MOUT_CPUCL2_SWITCH_USER, "mout_cpucl2_switch_user", mout_cpucl2_switch_user_p,
	    PLL_CON0_MUX_CLKCMU_CPUCL2_SWITCH_USER, 4, 1),
	MUX(CLK_MOUT_CPUCL2_CLUSTER, "mout_cpucl2_cluster", mout_cpucl2_cluster_p,
	    CLK_CON_MUX_MUX_CLK_CPUCL2_CLUSTER, 0, 2),
	MUX(CLK_MOUT_CPUCL2_CORE, "mout_cpucl2_core", mout_cpucl2_core_p,
	    CLK_CON_MUX_MUX_CLK_CPUCL2_CORE, 0, 2),
};

static cpucl2_div_clks: &[samsung_div_clock] = &[
	DIV(CLK_DOUT_CLUSTER2_ACLK, "dout_cluster2_aclk",
	    "mout_cpucl2_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER2_ACLK, 0, 4),
	DIV(CLK_DOUT_CLUSTER2_ATCLK, "dout_cluster2_atclk",
	    "mout_cpucl2_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER2_ATCLK, 0, 4),
	DIV(CLK_DOUT_CLUSTER2_MPCLK, "dout_cluster2_mpclk",
	    "mout_cpucl2_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER2_MPCLK, 0, 4),
	DIV(CLK_DOUT_CLUSTER2_PCLK, "dout_cluster2_pclk",
	    "mout_cpucl2_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER2_PCLK, 0, 4),
	DIV(CLK_DOUT_CLUSTER2_PERIPHCLK, "dout_cluster2_periphclk",
	    "mout_cpucl2_cluster", CLK_CON_DIV_DIV_CLK_CLUSTER2_PERIPHCLK, 0, 4),
	DIV(CLK_DOUT_CPUCL2_NOCP, "dout_cpucl2_nocp",
	    "mout_cpucl2_cluster", CLK_CON_DIV_DIV_CLK_CPUCL2_NOCP, 0, 4),
};

static cpucl2_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.pll_clks		= cpucl2_pll_clks,
	.nr_pll_clks		= cpucl2_pll_clks.len(),
	.mux_clks		= cpucl2_mux_clks,
	.nr_mux_clks		= cpucl2_mux_clks.len(),
	.div_clks		= cpucl2_div_clks,
	.nr_div_clks		= cpucl2_div_clks.len(),
	.nr_clk_ids		= CLKS_NR_CPUCL2,
	.clk_regs		= cpucl2_clk_regs,
	.nr_clk_regs		= cpucl2_clk_regs.len(),
	.clk_name		= "cpucl2",
};

static void __init exynosautov920_cmu_cpucl2_init(struct device_node *np)
{
	exynos_arm64_register_cmu(core::ptr::null_mut(), np, &cpucl2_cmu_info);
}

/* Register CMU_CPUCL2 early, as CPU clocks should be available ASAP */
CLK_OF_DECLARE(exynosautov920_cmu_cpucl2, "samsung,exynosautov920-cmu-cpucl2",
	       exynosautov920_cmu_cpucl2_init);

/* ---- CMU_PERIC0 --------------------------------------------------------- */

/* Register Offset definitions for CMU_PERIC0 (0x10800000) */
const PLL_CON0_MUX_CLKCMU_PERIC0_IP_USER: usize = 0x0600;
const PLL_CON0_MUX_CLKCMU_PERIC0_NOC_USER: usize = 0x0610;
const CLK_CON_MUX_MUX_CLK_PERIC0_I3C: usize = 0x1000;
const CLK_CON_MUX_MUX_CLK_PERIC0_USI00_USI: usize = 0x1004;
const CLK_CON_MUX_MUX_CLK_PERIC0_USI01_USI: usize = 0x1008;
const CLK_CON_MUX_MUX_CLK_PERIC0_USI02_USI: usize = 0x100c;
const CLK_CON_MUX_MUX_CLK_PERIC0_USI03_USI: usize = 0x1010;
const CLK_CON_MUX_MUX_CLK_PERIC0_USI04_USI: usize = 0x1014;
const CLK_CON_MUX_MUX_CLK_PERIC0_USI05_USI: usize = 0x1018;
const CLK_CON_MUX_MUX_CLK_PERIC0_USI06_USI: usize = 0x101c;
const CLK_CON_MUX_MUX_CLK_PERIC0_USI07_USI: usize = 0x1020;
const CLK_CON_MUX_MUX_CLK_PERIC0_USI08_USI: usize = 0x1024;
const CLK_CON_MUX_MUX_CLK_PERIC0_USI_I2C: usize = 0x1028;
const CLK_CON_DIV_DIV_CLK_PERIC0_I3C: usize = 0x1800;
const CLK_CON_DIV_DIV_CLK_PERIC0_USI00_USI: usize = 0x1804;
const CLK_CON_DIV_DIV_CLK_PERIC0_USI01_USI: usize = 0x1808;
const CLK_CON_DIV_DIV_CLK_PERIC0_USI02_USI: usize = 0x180c;
const CLK_CON_DIV_DIV_CLK_PERIC0_USI03_USI: usize = 0x1810;
const CLK_CON_DIV_DIV_CLK_PERIC0_USI04_USI: usize = 0x1814;
const CLK_CON_DIV_DIV_CLK_PERIC0_USI05_USI: usize = 0x1818;
const CLK_CON_DIV_DIV_CLK_PERIC0_USI06_USI: usize = 0x181c;
const CLK_CON_DIV_DIV_CLK_PERIC0_USI07_USI: usize = 0x1820;
const CLK_CON_DIV_DIV_CLK_PERIC0_USI08_USI: usize = 0x1824;
const CLK_CON_DIV_DIV_CLK_PERIC0_USI_I2C: usize = 0x1828;

static peric0_clk_regs: &[usize] = &[
	PLL_CON0_MUX_CLKCMU_PERIC0_IP_USER,
	PLL_CON0_MUX_CLKCMU_PERIC0_NOC_USER,
	CLK_CON_MUX_MUX_CLK_PERIC0_I3C,
	CLK_CON_MUX_MUX_CLK_PERIC0_USI00_USI,
	CLK_CON_MUX_MUX_CLK_PERIC0_USI01_USI,
	CLK_CON_MUX_MUX_CLK_PERIC0_USI02_USI,
	CLK_CON_MUX_MUX_CLK_PERIC0_USI03_USI,
	CLK_CON_MUX_MUX_CLK_PERIC0_USI04_USI,
	CLK_CON_MUX_MUX_CLK_PERIC0_USI05_USI,
	CLK_CON_MUX_MUX_CLK_PERIC0_USI06_USI,
	CLK_CON_MUX_MUX_CLK_PERIC0_USI07_USI,
	CLK_CON_MUX_MUX_CLK_PERIC0_USI08_USI,
	CLK_CON_MUX_MUX_CLK_PERIC0_USI_I2C,
	CLK_CON_DIV_DIV_CLK_PERIC0_I3C,
	CLK_CON_DIV_DIV_CLK_PERIC0_USI00_USI,
	CLK_CON_DIV_DIV_CLK_PERIC0_USI01_USI,
	CLK_CON_DIV_DIV_CLK_PERIC0_USI02_USI,
	CLK_CON_DIV_DIV_CLK_PERIC0_USI03_USI,
	CLK_CON_DIV_DIV_CLK_PERIC0_USI04_USI,
	CLK_CON_DIV_DIV_CLK_PERIC0_USI05_USI,
	CLK_CON_DIV_DIV_CLK_PERIC0_USI06_USI,
	CLK_CON_DIV_DIV_CLK_PERIC0_USI07_USI,
	CLK_CON_DIV_DIV_CLK_PERIC0_USI08_USI,
	CLK_CON_DIV_DIV_CLK_PERIC0_USI_I2C,
};

/* List of parent clocks for Muxes in CMU_PERIC0 */
static mout_peric0_ip_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_peric0_ip" };
static mout_peric0_noc_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_peric0_noc" };
static mout_peric0_usi_p: &[&str] = &[ "oscclk", "mout_peric0_ip_user" };

static peric0_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_PERIC0_IP_USER, "mout_peric0_ip_user",
	    mout_peric0_ip_user_p, PLL_CON0_MUX_CLKCMU_PERIC0_IP_USER, 4, 1),
	MUX(CLK_MOUT_PERIC0_NOC_USER, "mout_peric0_noc_user",
	    mout_peric0_noc_user_p, PLL_CON0_MUX_CLKCMU_PERIC0_NOC_USER, 4, 1),
	/* USI00 ~ USI08 */
	MUX(CLK_MOUT_PERIC0_USI00_USI, "mout_peric0_usi00_usi",
	    mout_peric0_usi_p, CLK_CON_MUX_MUX_CLK_PERIC0_USI00_USI, 0, 1),
	MUX(CLK_MOUT_PERIC0_USI01_USI, "mout_peric0_usi01_usi",
	    mout_peric0_usi_p, CLK_CON_MUX_MUX_CLK_PERIC0_USI01_USI, 0, 1),
	MUX(CLK_MOUT_PERIC0_USI02_USI, "mout_peric0_usi02_usi",
	    mout_peric0_usi_p, CLK_CON_MUX_MUX_CLK_PERIC0_USI02_USI, 0, 1),
	MUX(CLK_MOUT_PERIC0_USI03_USI, "mout_peric0_usi03_usi",
	    mout_peric0_usi_p, CLK_CON_MUX_MUX_CLK_PERIC0_USI03_USI, 0, 1),
	MUX(CLK_MOUT_PERIC0_USI04_USI, "mout_peric0_usi04_usi",
	    mout_peric0_usi_p, CLK_CON_MUX_MUX_CLK_PERIC0_USI04_USI, 0, 1),
	MUX(CLK_MOUT_PERIC0_USI05_USI, "mout_peric0_usi05_usi",
	    mout_peric0_usi_p, CLK_CON_MUX_MUX_CLK_PERIC0_USI05_USI, 0, 1),
	MUX(CLK_MOUT_PERIC0_USI06_USI, "mout_peric0_usi06_usi",
	    mout_peric0_usi_p, CLK_CON_MUX_MUX_CLK_PERIC0_USI06_USI, 0, 1),
	MUX(CLK_MOUT_PERIC0_USI07_USI, "mout_peric0_usi07_usi",
	    mout_peric0_usi_p, CLK_CON_MUX_MUX_CLK_PERIC0_USI07_USI, 0, 1),
	MUX(CLK_MOUT_PERIC0_USI08_USI, "mout_peric0_usi08_usi",
	    mout_peric0_usi_p, CLK_CON_MUX_MUX_CLK_PERIC0_USI08_USI, 0, 1),
	/* USI_I2C */
	MUX(CLK_MOUT_PERIC0_USI_I2C, "mout_peric0_usi_i2c",
	    mout_peric0_usi_p, CLK_CON_MUX_MUX_CLK_PERIC0_USI_I2C, 0, 1),
	/* USI_I3C */
	MUX(CLK_MOUT_PERIC0_I3C, "mout_peric0_i3c",
	    mout_peric0_usi_p, CLK_CON_MUX_MUX_CLK_PERIC0_I3C, 0, 1),
};

static peric0_div_clks: &[samsung_div_clock] = &[
	/* USI00 ~ USI08 */
	DIV(CLK_DOUT_PERIC0_USI00_USI, "dout_peric0_usi00_usi",
	    "mout_peric0_usi00_usi", CLK_CON_DIV_DIV_CLK_PERIC0_USI00_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC0_USI01_USI, "dout_peric0_usi01_usi",
	    "mout_peric0_usi01_usi", CLK_CON_DIV_DIV_CLK_PERIC0_USI01_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC0_USI02_USI, "dout_peric0_usi02_usi",
	    "mout_peric0_usi02_usi", CLK_CON_DIV_DIV_CLK_PERIC0_USI02_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC0_USI03_USI, "dout_peric0_usi03_usi",
	    "mout_peric0_usi03_usi", CLK_CON_DIV_DIV_CLK_PERIC0_USI03_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC0_USI04_USI, "dout_peric0_usi04_usi",
	    "mout_peric0_usi04_usi", CLK_CON_DIV_DIV_CLK_PERIC0_USI04_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC0_USI05_USI, "dout_peric0_usi05_usi",
	    "mout_peric0_usi05_usi", CLK_CON_DIV_DIV_CLK_PERIC0_USI05_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC0_USI06_USI, "dout_peric0_usi06_usi",
	    "mout_peric0_usi06_usi", CLK_CON_DIV_DIV_CLK_PERIC0_USI06_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC0_USI07_USI, "dout_peric0_usi07_usi",
	    "mout_peric0_usi07_usi", CLK_CON_DIV_DIV_CLK_PERIC0_USI07_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC0_USI08_USI, "dout_peric0_usi08_usi",
	    "mout_peric0_usi08_usi", CLK_CON_DIV_DIV_CLK_PERIC0_USI08_USI,
	    0, 4),
	/* USI_I2C */
	DIV(CLK_DOUT_PERIC0_USI_I2C, "dout_peric0_usi_i2c",
	    "mout_peric0_usi_i2c", CLK_CON_DIV_DIV_CLK_PERIC0_USI_I2C, 0, 4),
	/* USI_I3C */
	DIV(CLK_DOUT_PERIC0_I3C, "dout_peric0_i3c",
	    "mout_peric0_i3c", CLK_CON_DIV_DIV_CLK_PERIC0_I3C, 0, 4),
};

static peric0_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.mux_clks		= peric0_mux_clks,
	.nr_mux_clks		= peric0_mux_clks.len(),
	.div_clks		= peric0_div_clks,
	.nr_div_clks		= peric0_div_clks.len(),
	.nr_clk_ids		= CLKS_NR_PERIC0,
	.clk_regs		= peric0_clk_regs,
	.nr_clk_regs		= peric0_clk_regs.len(),
	.clk_name		= "noc",
};

/* ---- CMU_PERIC1 --------------------------------------------------------- */

/* Register Offset definitions for CMU_PERIC1 (0x10C00000) */
const PLL_CON0_MUX_CLKCMU_PERIC1_IP_USER: usize = 0x600;
const PLL_CON0_MUX_CLKCMU_PERIC1_NOC_USER: usize = 0x610;
const CLK_CON_MUX_MUX_CLK_PERIC1_I3C: usize = 0x1000;
const CLK_CON_MUX_MUX_CLK_PERIC1_USI09_USI: usize = 0x1004;
const CLK_CON_MUX_MUX_CLK_PERIC1_USI10_USI: usize = 0x1008;
const CLK_CON_MUX_MUX_CLK_PERIC1_USI11_USI: usize = 0x100c;
const CLK_CON_MUX_MUX_CLK_PERIC1_USI12_USI: usize = 0x1010;
const CLK_CON_MUX_MUX_CLK_PERIC1_USI13_USI: usize = 0x1014;
const CLK_CON_MUX_MUX_CLK_PERIC1_USI14_USI: usize = 0x1018;
const CLK_CON_MUX_MUX_CLK_PERIC1_USI15_USI: usize = 0x101c;
const CLK_CON_MUX_MUX_CLK_PERIC1_USI16_USI: usize = 0x1020;
const CLK_CON_MUX_MUX_CLK_PERIC1_USI17_USI: usize = 0x1024;
const CLK_CON_MUX_MUX_CLK_PERIC1_USI_I2C: usize = 0x1028;
const CLK_CON_DIV_DIV_CLK_PERIC1_I3C: usize = 0x1800;
const CLK_CON_DIV_DIV_CLK_PERIC1_USI09_USI: usize = 0x1804;
const CLK_CON_DIV_DIV_CLK_PERIC1_USI10_USI: usize = 0x1808;
const CLK_CON_DIV_DIV_CLK_PERIC1_USI11_USI: usize = 0x180c;
const CLK_CON_DIV_DIV_CLK_PERIC1_USI12_USI: usize = 0x1810;
const CLK_CON_DIV_DIV_CLK_PERIC1_USI13_USI: usize = 0x1814;
const CLK_CON_DIV_DIV_CLK_PERIC1_USI14_USI: usize = 0x1818;
const CLK_CON_DIV_DIV_CLK_PERIC1_USI15_USI: usize = 0x181c;
const CLK_CON_DIV_DIV_CLK_PERIC1_USI16_USI: usize = 0x1820;
const CLK_CON_DIV_DIV_CLK_PERIC1_USI17_USI: usize = 0x1824;
const CLK_CON_DIV_DIV_CLK_PERIC1_USI_I2C: usize = 0x1828;

static peric1_clk_regs: &[usize] = &[
	PLL_CON0_MUX_CLKCMU_PERIC1_IP_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_NOC_USER,
	CLK_CON_MUX_MUX_CLK_PERIC1_I3C,
	CLK_CON_MUX_MUX_CLK_PERIC1_USI09_USI,
	CLK_CON_MUX_MUX_CLK_PERIC1_USI10_USI,
	CLK_CON_MUX_MUX_CLK_PERIC1_USI11_USI,
	CLK_CON_MUX_MUX_CLK_PERIC1_USI12_USI,
	CLK_CON_MUX_MUX_CLK_PERIC1_USI13_USI,
	CLK_CON_MUX_MUX_CLK_PERIC1_USI14_USI,
	CLK_CON_MUX_MUX_CLK_PERIC1_USI15_USI,
	CLK_CON_MUX_MUX_CLK_PERIC1_USI16_USI,
	CLK_CON_MUX_MUX_CLK_PERIC1_USI17_USI,
	CLK_CON_MUX_MUX_CLK_PERIC1_USI_I2C,
	CLK_CON_DIV_DIV_CLK_PERIC1_I3C,
	CLK_CON_DIV_DIV_CLK_PERIC1_USI09_USI,
	CLK_CON_DIV_DIV_CLK_PERIC1_USI10_USI,
	CLK_CON_DIV_DIV_CLK_PERIC1_USI11_USI,
	CLK_CON_DIV_DIV_CLK_PERIC1_USI12_USI,
	CLK_CON_DIV_DIV_CLK_PERIC1_USI13_USI,
	CLK_CON_DIV_DIV_CLK_PERIC1_USI14_USI,
	CLK_CON_DIV_DIV_CLK_PERIC1_USI15_USI,
	CLK_CON_DIV_DIV_CLK_PERIC1_USI16_USI,
	CLK_CON_DIV_DIV_CLK_PERIC1_USI17_USI,
	CLK_CON_DIV_DIV_CLK_PERIC1_USI_I2C,
};

/* List of parent clocks for Muxes in CMU_PERIC1 */
static mout_peric1_ip_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_peric1_ip" };
static mout_peric1_noc_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_peric1_noc" };
static mout_peric1_usi_p: &[&str] = &[ "oscclk", "mout_peric1_ip_user" };

static peric1_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_PERIC1_IP_USER, "mout_peric1_ip_user",
	    mout_peric1_ip_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_IP_USER, 4, 1),
	MUX(CLK_MOUT_PERIC1_NOC_USER, "mout_peric1_noc_user",
	    mout_peric1_noc_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_NOC_USER, 4, 1),
	/* USI09 ~ USI17 */
	MUX(CLK_MOUT_PERIC1_USI09_USI, "mout_peric1_usi09_usi",
	    mout_peric1_usi_p, CLK_CON_MUX_MUX_CLK_PERIC1_USI09_USI, 0, 1),
	MUX(CLK_MOUT_PERIC1_USI10_USI, "mout_peric1_usi10_usi",
	    mout_peric1_usi_p, CLK_CON_MUX_MUX_CLK_PERIC1_USI10_USI, 0, 1),
	MUX(CLK_MOUT_PERIC1_USI11_USI, "mout_peric1_usi11_usi",
	    mout_peric1_usi_p, CLK_CON_MUX_MUX_CLK_PERIC1_USI11_USI, 0, 1),
	MUX(CLK_MOUT_PERIC1_USI12_USI, "mout_peric1_usi12_usi",
	    mout_peric1_usi_p, CLK_CON_MUX_MUX_CLK_PERIC1_USI12_USI, 0, 1),
	MUX(CLK_MOUT_PERIC1_USI13_USI, "mout_peric1_usi13_usi",
	    mout_peric1_usi_p, CLK_CON_MUX_MUX_CLK_PERIC1_USI13_USI, 0, 1),
	MUX(CLK_MOUT_PERIC1_USI14_USI, "mout_peric1_usi14_usi",
	    mout_peric1_usi_p, CLK_CON_MUX_MUX_CLK_PERIC1_USI14_USI, 0, 1),
	MUX(CLK_MOUT_PERIC1_USI15_USI, "mout_peric1_usi15_usi",
	    mout_peric1_usi_p, CLK_CON_MUX_MUX_CLK_PERIC1_USI15_USI, 0, 1),
	MUX(CLK_MOUT_PERIC1_USI16_USI, "mout_peric1_usi16_usi",
	    mout_peric1_usi_p, CLK_CON_MUX_MUX_CLK_PERIC1_USI16_USI, 0, 1),
	MUX(CLK_MOUT_PERIC1_USI17_USI, "mout_peric1_usi17_usi",
	    mout_peric1_usi_p, CLK_CON_MUX_MUX_CLK_PERIC1_USI17_USI, 0, 1),
	/* USI_I2C */
	MUX(CLK_MOUT_PERIC1_USI_I2C, "mout_peric1_usi_i2c",
	    mout_peric1_usi_p, CLK_CON_MUX_MUX_CLK_PERIC1_USI_I2C, 0, 1),
	/* USI_I3C */
	MUX(CLK_MOUT_PERIC1_I3C, "mout_peric1_i3c",
	    mout_peric1_usi_p, CLK_CON_MUX_MUX_CLK_PERIC1_I3C, 0, 1),
};

static peric1_div_clks: &[samsung_div_clock] = &[
	/* USI09 ~ USI17 */
	DIV(CLK_DOUT_PERIC1_USI09_USI, "dout_peric1_usi09_usi",
	    "mout_peric1_usi09_usi", CLK_CON_DIV_DIV_CLK_PERIC1_USI09_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC1_USI10_USI, "dout_peric1_usi10_usi",
	    "mout_peric1_usi10_usi", CLK_CON_DIV_DIV_CLK_PERIC1_USI10_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC1_USI11_USI, "dout_peric1_usi11_usi",
	    "mout_peric1_usi11_usi", CLK_CON_DIV_DIV_CLK_PERIC1_USI11_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC1_USI12_USI, "dout_peric1_usi12_usi",
	    "mout_peric1_usi12_usi", CLK_CON_DIV_DIV_CLK_PERIC1_USI12_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC1_USI13_USI, "dout_peric1_usi13_usi",
	    "mout_peric1_usi13_usi", CLK_CON_DIV_DIV_CLK_PERIC1_USI13_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC1_USI14_USI, "dout_peric1_usi14_usi",
	    "mout_peric1_usi14_usi", CLK_CON_DIV_DIV_CLK_PERIC1_USI14_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC1_USI15_USI, "dout_peric1_usi15_usi",
	    "mout_peric1_usi15_usi", CLK_CON_DIV_DIV_CLK_PERIC1_USI15_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC1_USI16_USI, "dout_peric1_usi16_usi",
	    "mout_peric1_usi16_usi", CLK_CON_DIV_DIV_CLK_PERIC1_USI16_USI,
	    0, 4),
	DIV(CLK_DOUT_PERIC1_USI17_USI, "dout_peric1_usi17_usi",
	    "mout_peric1_usi17_usi", CLK_CON_DIV_DIV_CLK_PERIC1_USI17_USI,
	    0, 4),
	/* USI_I2C */
	DIV(CLK_DOUT_PERIC1_USI_I2C, "dout_peric1_usi_i2c",
	    "mout_peric1_usi_i2c", CLK_CON_DIV_DIV_CLK_PERIC1_USI_I2C, 0, 4),
	/* USI_I3C */
	DIV(CLK_DOUT_PERIC1_I3C, "dout_peric1_i3c",
	    "mout_peric1_i3c", CLK_CON_DIV_DIV_CLK_PERIC1_I3C, 0, 4),
};

static peric1_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.mux_clks		= peric1_mux_clks,
	.nr_mux_clks		= peric1_mux_clks.len(),
	.div_clks		= peric1_div_clks,
	.nr_div_clks		= peric1_div_clks.len(),
	.nr_clk_ids		= CLKS_NR_PERIC1,
	.clk_regs		= peric1_clk_regs,
	.nr_clk_regs		= peric1_clk_regs.len(),
	.clk_name		= "noc",
};

/* ---- CMU_MISC --------------------------------------------------------- */

/* Register Offset definitions for CMU_MISC (0x10020000) */
const PLL_CON0_MUX_CLKCMU_MISC_NOC_USER: usize = 0x600;
const CLK_CON_MUX_MUX_CLK_MISC_GIC: usize = 0x1000;
const CLK_CON_DIV_CLKCMU_OTP: usize = 0x1800;
const CLK_CON_DIV_DIV_CLK_MISC_NOCP: usize = 0x1804;
const CLK_CON_DIV_DIV_CLK_MISC_OSC_DIV2: usize = 0x1808;

static misc_clk_regs: &[usize] = &[
	PLL_CON0_MUX_CLKCMU_MISC_NOC_USER,
	CLK_CON_MUX_MUX_CLK_MISC_GIC,
	CLK_CON_DIV_CLKCMU_OTP,
	CLK_CON_DIV_DIV_CLK_MISC_NOCP,
	CLK_CON_DIV_DIV_CLK_MISC_OSC_DIV2,
};

/* List of parent clocks for Muxes in CMU_MISC */
static mout_misc_noc_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_misc_noc" };
static mout_misc_gic_p: &[&str] = &[ "dout_misc_nocp", "oscclk" };

static misc_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_MISC_NOC_USER, "mout_misc_noc_user",
	    mout_misc_noc_user_p, PLL_CON0_MUX_CLKCMU_MISC_NOC_USER, 4, 1),
	MUX(CLK_MOUT_MISC_GIC, "mout_misc_gic",
	    mout_misc_gic_p, CLK_CON_MUX_MUX_CLK_MISC_GIC, 0, 1),
};

static misc_div_clks: &[samsung_div_clock] = &[
	DIV(CLK_DOUT_MISC_NOCP, "dout_misc_nocp",
	    "mout_misc_noc_user", CLK_CON_DIV_DIV_CLK_MISC_NOCP,
	    0, 3),
};

static misc_fixed_factor_clks: &[samsung_fixed_factor_clock] = &[
	FFACTOR(CLK_DOUT_MISC_OTP, "dout_misc_otp",
		"oscclk", 1, 10, 0),
	FFACTOR(CLK_DOUT_MISC_OSC_DIV2, "dout_misc_osc_div2",
		"oscclk", 1, 2, 0),
};

static misc_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.mux_clks		= misc_mux_clks,
	.nr_mux_clks		= misc_mux_clks.len(),
	.div_clks		= misc_div_clks,
	.nr_div_clks		= misc_div_clks.len(),
	.fixed_factor_clks	= misc_fixed_factor_clks,
	.nr_fixed_factor_clks	= misc_fixed_factor_clks.len(),
	.nr_clk_ids		= CLKS_NR_MISC,
	.clk_regs		= misc_clk_regs,
	.nr_clk_regs		= misc_clk_regs.len(),
	.clk_name		= "noc",
};

/* ---- CMU_HSI0 --------------------------------------------------------- */

/* Register Offset definitions for CMU_HSI0 (0x16000000) */
const PLL_CON0_MUX_CLKCMU_HSI0_NOC_USER: usize = 0x600;
const CLK_CON_DIV_DIV_CLK_HSI0_PCIE_APB: usize = 0x1800;

static hsi0_clk_regs: &[usize] = &[
	PLL_CON0_MUX_CLKCMU_HSI0_NOC_USER,
	CLK_CON_DIV_DIV_CLK_HSI0_PCIE_APB,
};

/* List of parent clocks for Muxes in CMU_HSI0 */
static mout_hsi0_noc_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_hsi0_noc" };

static hsi0_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_HSI0_NOC_USER, "mout_hsi0_noc_user",
	    mout_hsi0_noc_user_p, PLL_CON0_MUX_CLKCMU_HSI0_NOC_USER, 4, 1),
};

static hsi0_div_clks: &[samsung_div_clock] = &[
	DIV(CLK_DOUT_HSI0_PCIE_APB, "dout_hsi0_pcie_apb",
	    "mout_hsi0_noc_user", CLK_CON_DIV_DIV_CLK_HSI0_PCIE_APB,
	    0, 4),
};

static hsi0_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.mux_clks		= hsi0_mux_clks,
	.nr_mux_clks		= hsi0_mux_clks.len(),
	.div_clks		= hsi0_div_clks,
	.nr_div_clks		= hsi0_div_clks.len(),
	.nr_clk_ids		= CLKS_NR_HSI0,
	.clk_regs		= hsi0_clk_regs,
	.nr_clk_regs		= hsi0_clk_regs.len(),
	.clk_name		= "noc",
};

/* ---- CMU_HSI1 --------------------------------------------------------- */

/* Register Offset definitions for CMU_HSI1 (0x16400000) */
const PLL_CON0_MUX_CLKCMU_HSI1_MMC_CARD_USER: usize = 0x600;
const PLL_CON0_MUX_CLKCMU_HSI1_NOC_USER: usize = 0x610;
const PLL_CON0_MUX_CLKCMU_HSI1_USBDRD_USER: usize = 0x620;
const CLK_CON_MUX_MUX_CLK_HSI1_USBDRD: usize = 0x1000;

static hsi1_clk_regs: &[usize] = &[
	PLL_CON0_MUX_CLKCMU_HSI1_MMC_CARD_USER,
	PLL_CON0_MUX_CLKCMU_HSI1_NOC_USER,
	PLL_CON0_MUX_CLKCMU_HSI1_USBDRD_USER,
	CLK_CON_MUX_MUX_CLK_HSI1_USBDRD,
};

/* List of parent clocks for Muxes in CMU_HSI1 */
static mout_hsi1_mmc_card_user_p: &[&str] = &["oscclk", "dout_clkcmu_hsi1_mmc_card"};
static mout_hsi1_noc_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_hsi1_noc" };
static mout_hsi1_usbdrd_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_hsi1_usbdrd" };
static mout_hsi1_usbdrd_p: &[&str] = &[ "dout_tcxo_div2", "mout_hsi1_usbdrd_user" };

static hsi1_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_HSI1_MMC_CARD_USER, "mout_hsi1_mmc_card_user",
	    mout_hsi1_mmc_card_user_p, PLL_CON0_MUX_CLKCMU_HSI1_MMC_CARD_USER, 4, 1),
	MUX(CLK_MOUT_HSI1_NOC_USER, "mout_hsi1_noc_user",
	    mout_hsi1_noc_user_p, PLL_CON0_MUX_CLKCMU_HSI1_NOC_USER, 4, 1),
	MUX(CLK_MOUT_HSI1_USBDRD_USER, "mout_hsi1_usbdrd_user",
	    mout_hsi1_usbdrd_user_p, PLL_CON0_MUX_CLKCMU_HSI1_USBDRD_USER, 4, 1),
	MUX(CLK_MOUT_HSI1_USBDRD, "mout_hsi1_usbdrd",
	    mout_hsi1_usbdrd_p, CLK_CON_MUX_MUX_CLK_HSI1_USBDRD, 4, 1),
};

static hsi1_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.mux_clks		= hsi1_mux_clks,
	.nr_mux_clks		= hsi1_mux_clks.len(),
	.nr_clk_ids		= CLKS_NR_HSI1,
	.clk_regs		= hsi1_clk_regs,
	.nr_clk_regs		= hsi1_clk_regs.len(),
	.clk_name		= "noc",
};

/* ---- CMU_HSI2 --------------------------------------------------------- */

/* Register Offset definitions for CMU_HSI2 (0x16b00000) */
const PLL_LOCKTIME_PLL_ETH: usize = 0x0;
const PLL_CON3_PLL_ETH: usize = 0x10c;
const PLL_CON0_MUX_CLKCMU_HSI2_ETHERNET_USER: usize = 0x600;
const PLL_CON0_MUX_CLKCMU_HSI2_NOC_UFS_USER: usize = 0x610;
const PLL_CON0_MUX_CLKCMU_HSI2_UFS_EMBD_USER: usize = 0x630;
const CLK_CON_MUX_MUX_CLK_HSI2_ETHERNET: usize = 0x1000;
const CLK_CON_DIV_DIV_CLK_HSI2_ETHERNET: usize = 0x1800;
const CLK_CON_DIV_DIV_CLK_HSI2_ETHERNET_PTP: usize = 0x1804;

static hsi2_clk_regs: &[usize] = &[
	PLL_LOCKTIME_PLL_ETH,
	PLL_CON3_PLL_ETH,
	PLL_CON0_MUX_CLKCMU_HSI2_ETHERNET_USER,
	PLL_CON0_MUX_CLKCMU_HSI2_NOC_UFS_USER,
	PLL_CON0_MUX_CLKCMU_HSI2_UFS_EMBD_USER,
	CLK_CON_MUX_MUX_CLK_HSI2_ETHERNET,
	CLK_CON_DIV_DIV_CLK_HSI2_ETHERNET,
	CLK_CON_DIV_DIV_CLK_HSI2_ETHERNET_PTP,
};

static hsi2_pll_clks: &[samsung_pll_clock] = &[
	/* CMU_HSI2_PLL */
	PLL(pll_531x, FOUT_PLL_ETH, "fout_pll_eth", "oscclk",
	    PLL_LOCKTIME_PLL_ETH, PLL_CON3_PLL_ETH, core::ptr::null_mut()),
};

/* List of parent clocks for Muxes in CMU_HSI2 */
static mout_clkcmu_hsi2_noc_ufs_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_hsi2_noc_ufs" };
static mout_clkcmu_hsi2_ufs_embd_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_hsi2_ufs_embd" };
static mout_hsi2_ethernet_p: &[&str] = &[ "fout_pll_eth", "mout_clkcmu_hsi2_ethernet_user" };
static mout_clkcmu_hsi2_ethernet_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_hsi2_ethernet" };

static hsi2_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_HSI2_NOC_UFS_USER, "mout_clkcmu_hsi2_noc_ufs_user",
	    mout_clkcmu_hsi2_noc_ufs_user_p, PLL_CON0_MUX_CLKCMU_HSI2_NOC_UFS_USER, 4, 1),
	MUX(CLK_MOUT_HSI2_UFS_EMBD_USER, "mout_clkcmu_hsi2_ufs_embd_user",
	    mout_clkcmu_hsi2_ufs_embd_user_p, PLL_CON0_MUX_CLKCMU_HSI2_UFS_EMBD_USER, 4, 1),
	MUX(CLK_MOUT_HSI2_ETHERNET, "mout_hsi2_ethernet",
	    mout_hsi2_ethernet_p, CLK_CON_MUX_MUX_CLK_HSI2_ETHERNET, 0, 1),
	MUX(CLK_MOUT_HSI2_ETHERNET_USER, "mout_clkcmu_hsi2_ethernet_user",
	    mout_clkcmu_hsi2_ethernet_user_p, PLL_CON0_MUX_CLKCMU_HSI2_ETHERNET_USER, 4, 1),
};

static hsi2_div_clks: &[samsung_div_clock] = &[
	DIV(CLK_DOUT_HSI2_ETHERNET, "dout_hsi2_ethernet",
	    "mout_hsi2_ethernet", CLK_CON_DIV_DIV_CLK_HSI2_ETHERNET,
	    0, 4),
	DIV(CLK_DOUT_HSI2_ETHERNET_PTP, "dout_hsi2_ethernet_ptp",
	    "mout_hsi2_ethernet", CLK_CON_DIV_DIV_CLK_HSI2_ETHERNET_PTP,
	    0, 4),
};

static hsi2_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.pll_clks               = hsi2_pll_clks,
	.nr_pll_clks            = hsi2_pll_clks.len(),
	.mux_clks               = hsi2_mux_clks,
	.nr_mux_clks            = hsi2_mux_clks.len(),
	.div_clks               = hsi2_div_clks,
	.nr_div_clks            = hsi2_div_clks.len(),
	.nr_clk_ids             = CLKS_NR_HSI2,
	.clk_regs               = hsi2_clk_regs,
	.nr_clk_regs            = hsi2_clk_regs.len(),
	.clk_name               = "noc",
};

/* ---- CMU_M2M --------------------------------------------------------- */

/* Register Offset definitions for CMU_M2M (0x1a800000) */
const PLL_CON0_MUX_CLKCMU_M2M_JPEG_USER: usize = 0x600;
const PLL_CON0_MUX_CLKCMU_M2M_NOC_USER: usize = 0x610;
const CLK_CON_DIV_DIV_CLK_M2M_NOCP: usize = 0x1800;

static m2m_clk_regs: &[usize] = &[
	PLL_CON0_MUX_CLKCMU_M2M_JPEG_USER,
	PLL_CON0_MUX_CLKCMU_M2M_NOC_USER,
	CLK_CON_DIV_DIV_CLK_M2M_NOCP,
};

/* List of parent clocks for Muxes in CMU_M2M */
static mout_clkcmu_m2m_noc_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_m2m_noc" };
static mout_clkcmu_m2m_jpeg_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_m2m_jpeg" };

static m2m_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_M2M_JPEG_USER, "mout_clkcmu_m2m_jpeg_user",
	    mout_clkcmu_m2m_jpeg_user_p, PLL_CON0_MUX_CLKCMU_M2M_JPEG_USER, 4, 1),
	MUX(CLK_MOUT_M2M_NOC_USER, "mout_clkcmu_m2m_noc_user",
	    mout_clkcmu_m2m_noc_user_p, PLL_CON0_MUX_CLKCMU_M2M_NOC_USER, 4, 1),
};

static m2m_div_clks: &[samsung_div_clock] = &[
	DIV(CLK_DOUT_M2M_NOCP, "dout_m2m_nocp",
	    "mout_clkcmu_m2m_noc_user", CLK_CON_DIV_DIV_CLK_M2M_NOCP,
	    0, 3),
};

static m2m_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.mux_clks               = m2m_mux_clks,
	.nr_mux_clks            = m2m_mux_clks.len(),
	.div_clks               = m2m_div_clks,
	.nr_div_clks            = m2m_div_clks.len(),
	.nr_clk_ids             = CLKS_NR_M2M,
	.clk_regs               = m2m_clk_regs,
	.nr_clk_regs            = m2m_clk_regs.len(),
	.clk_name               = "noc",
};

/* ---- CMU_MFC --------------------------------------------------------- */

/* Register Offset definitions for CMU_MFC (0x19c00000) */
const PLL_CON0_MUX_CLKCMU_MFC_MFC_USER: usize = 0x600;
const PLL_CON0_MUX_CLKCMU_MFC_WFD_USER: usize = 0x610;
const CLK_CON_DIV_DIV_CLK_MFC_NOCP: usize = 0x1800;

static mfc_clk_regs: &[usize] = &[
	PLL_CON0_MUX_CLKCMU_MFC_MFC_USER,
	PLL_CON0_MUX_CLKCMU_MFC_WFD_USER,
	CLK_CON_DIV_DIV_CLK_MFC_NOCP,
};

/* List of parent clocks for Muxes in CMU_MFC */
static mout_clkcmu_mfc_mfc_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_mfc_mfc" };
static mout_clkcmu_mfc_wfd_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_mfc_wfd" };

static mfc_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_MFC_MFC_USER, "mout_clkcmu_mfc_mfc_user",
	    mout_clkcmu_mfc_mfc_user_p, PLL_CON0_MUX_CLKCMU_MFC_MFC_USER, 4, 1),
	MUX(CLK_MOUT_MFC_WFD_USER, "mout_clkcmu_mfc_wfd_user",
	    mout_clkcmu_mfc_wfd_user_p, PLL_CON0_MUX_CLKCMU_MFC_WFD_USER, 4, 1),
};

static mfc_div_clks: &[samsung_div_clock] = &[
	DIV(CLK_DOUT_MFC_NOCP, "dout_mfc_nocp",
	    "mout_clkcmu_mfc_mfc_user", CLK_CON_DIV_DIV_CLK_MFC_NOCP,
	    0, 3),
};

static mfc_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.mux_clks               = mfc_mux_clks,
	.nr_mux_clks            = mfc_mux_clks.len(),
	.div_clks               = mfc_div_clks,
	.nr_div_clks            = mfc_div_clks.len(),
	.nr_clk_ids             = CLKS_NR_MFC,
	.clk_regs               = mfc_clk_regs,
	.nr_clk_regs            = mfc_clk_regs.len(),
	.clk_name               = "noc",
};

/* ---- CMU_MFD --------------------------------------------------------- */

/* Register Offset definitions for CMU_MFD (0x19e00000) */
const PLL_CON0_MUX_CLKCMU_MFD_NOC_USER: usize = 0x600;
const CLK_CON_DIV_DIV_CLK_MFD_NOCP: usize = 0x1800;

static mfd_clk_regs: &[usize] = &[
	PLL_CON0_MUX_CLKCMU_MFD_NOC_USER,
	CLK_CON_DIV_DIV_CLK_MFD_NOCP,
};

/* List of parent clocks for Muxes in CMU_MFD */
static mout_clkcmu_mfd_noc_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_mfd_noc" };

static mfd_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_MFD_NOC_USER, "mout_clkcmu_mfd_noc_user",
	    mout_clkcmu_mfd_noc_user_p, PLL_CON0_MUX_CLKCMU_MFD_NOC_USER, 4, 1),
};

static mfd_div_clks: &[samsung_div_clock] = &[
	DIV(CLK_DOUT_MFD_NOCP, "dout_mfd_nocp",
	    "mout_clkcmu_mfd_noc_user", CLK_CON_DIV_DIV_CLK_MFD_NOCP,
	    0, 3),
};

static mfd_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.mux_clks               = mfd_mux_clks,
	.nr_mux_clks            = mfd_mux_clks.len(),
	.div_clks               = mfd_div_clks,
	.nr_div_clks            = mfd_div_clks.len(),
	.nr_clk_ids             = CLKS_NR_MFD,
	.clk_regs               = mfd_clk_regs,
	.nr_clk_regs            = mfd_clk_regs.len(),
	.clk_name               = "noc",
};

/* ---- CMU_G3D --------------------------------------------------------- */

/* Register Offset definitions for CMU_G3D (0x1a000000) */
const PLL_LOCKTIME_PLL_G3D: usize = 0x0;
const PLL_CON3_PLL_G3D: usize = 0x10c;
const CLK_CON_MUX_MUX_CLK_G3D_NOC: usize = 0x1000;
const PLL_CON0_MUX_CLKCMU_G3D_NOCP_USER: usize = 0x600;
const PLL_CON0_MUX_CLKCMU_G3D_SWITCH_USER: usize = 0x610;

static g3d_clk_regs: &[usize] = &[
	PLL_LOCKTIME_PLL_G3D,
	PLL_CON3_PLL_G3D,
	CLK_CON_MUX_MUX_CLK_G3D_NOC,
	PLL_CON0_MUX_CLKCMU_G3D_NOCP_USER,
	PLL_CON0_MUX_CLKCMU_G3D_SWITCH_USER,
};

static g3d_pll_clks: &[samsung_pll_clock] = &[
	/* CMU_G3D_PLL */
	PLL(pll_531x, FOUT_PLL_G3D, "fout_pll_g3d", "oscclk",
	    PLL_LOCKTIME_PLL_G3D, PLL_CON3_PLL_G3D, core::ptr::null_mut()),
};

/* List of parent clocks for Muxes in CMU_G3D */
static mout_clk_g3d_noc_p: &[&str] = &[ "oscclk", "fout_pll_g3d", "mout_clkcmu_g3d_switch_user"};
static mout_clkcmu_g3d_switch_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_g3d_switch" };
static mout_clkcmu_g3d_nocp_user_p: &[&str] = &[ "oscclk", "dout_clkcmu_g3d_nocp" };

static g3d_mux_clks: &[samsung_mux_clock] = &[
	MUX(CLK_MOUT_G3D_NOC, "mout_clk_g3d_noc",
	    mout_clk_g3d_noc_p, CLK_CON_MUX_MUX_CLK_G3D_NOC, 0, 2),
	MUX(CLK_MOUT_G3D_SWITCH_USER, "mout_clkcmu_g3d_switch_user",
	    mout_clkcmu_g3d_switch_user_p, PLL_CON0_MUX_CLKCMU_G3D_SWITCH_USER, 4, 1),
	MUX(CLK_MOUT_G3D_NOCP_USER, "mout_clkcmu_g3d_nocp_user",
	    mout_clkcmu_g3d_nocp_user_p, PLL_CON0_MUX_CLKCMU_G3D_NOCP_USER, 4, 1),
};

static g3d_cmu_info: samsung_cmu_info = samsung_cmu_info {
	.pll_clks               = g3d_pll_clks,
	.nr_pll_clks            = g3d_pll_clks.len(),
	.mux_clks               = g3d_mux_clks,
	.nr_mux_clks            = g3d_mux_clks.len(),
	.nr_clk_ids             = CLKS_NR_G3D,
	.clk_regs               = g3d_clk_regs,
	.nr_clk_regs            = g3d_clk_regs.len(),
	.clk_name               = "noc",
};

unsafe fn exynosautov920_cmu_probe(struct platform_device *pdev)
{
	const struct samsung_cmu_info *info;
	struct device *dev = &pdev->dev;

	info = of_device_get_match_data(dev);
	exynos_arm64_register_cmu(dev, dev->of_node, info);

	return 0;
}

static exynosautov920_cmu_of_match: &[of_device_id] = &[
	{
		.compatible = "samsung,exynosautov920-cmu-peric0",
		.data = &peric0_cmu_info,
	}, {
		 .compatible = "samsung,exynosautov920-cmu-peric1",
		 .data = &peric1_cmu_info,
	}, {
		 .compatible = "samsung,exynosautov920-cmu-misc",
		 .data = &misc_cmu_info,
	}, {
		.compatible = "samsung,exynosautov920-cmu-hsi0",
		.data = &hsi0_cmu_info,
	}, {
		.compatible = "samsung,exynosautov920-cmu-hsi1",
		.data = &hsi1_cmu_info,
	}, {
		.compatible = "samsung,exynosautov920-cmu-hsi2",
		.data = &hsi2_cmu_info,
	}, {
		.compatible = "samsung,exynosautov920-cmu-m2m",
		.data = &m2m_cmu_info,
	}, {
		.compatible = "samsung,exynosautov920-cmu-mfc",
		.data = &mfc_cmu_info,
	}, {
		.compatible = "samsung,exynosautov920-cmu-mfd",
		.data = &mfd_cmu_info,
	}, {
		.compatible = "samsung,exynosautov920-cmu-g3d",
		.data = &g3d_cmu_info,
	},
	{ }
};

static struct platform_driver exynosautov920_cmu_driver  = {
	.driver = {
		.name = "exynosautov920-cmu",
		.of_match_table = exynosautov920_cmu_of_match,
		.suppress_bind_attrs = true,
	},
	.probe = exynosautov920_cmu_probe,
};

unsafe fn exynosautov920_cmu_init(void)
{
	return platform_driver_register(&exynosautov920_cmu_driver);
}
const _: fn() = exynosautov920_cmu_init;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
