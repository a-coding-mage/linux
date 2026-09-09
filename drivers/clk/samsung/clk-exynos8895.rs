// Kernel-provided types, macros, constants, and functions remain external dependencies.
#![allow(non_upper_case_globals, non_snake_case, dead_code, unused_variables)]
// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024 Ivaylo Ivanov <ivo.ivanov.ivanov1@gmail.com>
 * Author: Ivaylo Ivanov <ivo.ivanov.ivanov1@gmail.com>
 *
 * Common Clock Framework support for Exynos8895 SoC.
 */

// external dependency: #include <linux/clk-provider.h>
// external dependency: #include <linux/of.h>
// external dependency: #include <linux/platform_device.h>

// external dependency: #include <dt-bindings/clock/samsung,exynos8895.h>

// external dependency: #include "clk.h"
// external dependency: #include "clk-exynos-arm64.h"

/* NOTE: Must be equal to the last clock ID increased by one */
const CLKS_NR_TOP: usize = (CLK_GOUT_CMU_VPU_BUS + 1);
const CLKS_NR_FSYS0: usize = (CLK_GOUT_FSYS0_XIU_P_FSYS0_ACLK + 1);
const CLKS_NR_FSYS1: usize = (CLK_GOUT_FSYS1_XIU_P_FSYS1_ACLK + 1);
const CLKS_NR_PERIC0: usize = (CLK_GOUT_PERIC0_USI03_I_SCLK_USI + 1);
const CLKS_NR_PERIC1: usize = (CLK_GOUT_PERIC1_XIU_P_PERIC1_ACLK + 1);
const CLKS_NR_PERIS: usize = (CLK_GOUT_PERIS_XIU_P_PERIS_ACLK + 1);

/* ---- CMU_TOP ------------------------------------------------------------- */

/* Register Offset definitions for CMU_TOP (0x15a80000) */
const PLL_LOCKTIME_PLL_SHARED0: usize = 0x0000;
const PLL_LOCKTIME_PLL_SHARED1: usize = 0x0004;
const PLL_LOCKTIME_PLL_SHARED2: usize = 0x0008;
const PLL_LOCKTIME_PLL_SHARED3: usize = 0x000c;
const PLL_LOCKTIME_PLL_SHARED4: usize = 0x0010;
const PLL_CON0_MUX_CP2AP_MIF_CLK_USER: usize = 0x0100;
const PLL_CON2_MUX_CP2AP_MIF_CLK_USER: usize = 0x0108;
const PLL_CON0_PLL_SHARED0: usize = 0x0120;
const PLL_CON0_PLL_SHARED1: usize = 0x0140;
const PLL_CON0_PLL_SHARED2: usize = 0x0160;
const PLL_CON0_PLL_SHARED3: usize = 0x0180;
const PLL_CON0_PLL_SHARED4: usize = 0x01a0;
const CLK_CON_MUX_MUX_CLKCMU_ABOX_CPUABOX: usize = 0x1000;
const CLK_CON_MUX_MUX_CLKCMU_APM_BUS: usize = 0x1004;
const CLK_CON_MUX_MUX_CLKCMU_BUS1_BUS: usize = 0x1008;
const CLK_CON_MUX_MUX_CLKCMU_BUSC_BUS: usize = 0x100c;
const CLK_CON_MUX_MUX_CLKCMU_BUSC_BUSPHSI2C: usize = 0x1010;
const CLK_CON_MUX_MUX_CLKCMU_CAM_BUS: usize = 0x1014;
const CLK_CON_MUX_MUX_CLKCMU_CAM_TPU0: usize = 0x1018;
const CLK_CON_MUX_MUX_CLKCMU_CAM_TPU1: usize = 0x101c;
const CLK_CON_MUX_MUX_CLKCMU_CAM_VRA: usize = 0x1020;
const CLK_CON_MUX_MUX_CLKCMU_CIS_CLK0: usize = 0x1024;
const CLK_CON_MUX_MUX_CLKCMU_CIS_CLK1: usize = 0x1028;
const CLK_CON_MUX_MUX_CLKCMU_CIS_CLK2: usize = 0x102c;
const CLK_CON_MUX_MUX_CLKCMU_CIS_CLK3: usize = 0x1030;
const CLK_CON_MUX_MUX_CLKCMU_CORE_BUS: usize = 0x1034;
const CLK_CON_MUX_MUX_CLKCMU_CPUCL0_SWITCH: usize = 0x1038;
const CLK_CON_MUX_MUX_CLKCMU_CPUCL1_SWITCH: usize = 0x103c;
const CLK_CON_MUX_MUX_CLKCMU_DBG_BUS: usize = 0x1040;
const CLK_CON_MUX_MUX_CLKCMU_DCAM_BUS: usize = 0x1044;
const CLK_CON_MUX_MUX_CLKCMU_DCAM_IMGD: usize = 0x1048;
const CLK_CON_MUX_MUX_CLKCMU_DPU_BUS: usize = 0x104c;
const CLK_CON_MUX_MUX_CLKCMU_DROOPDETECTOR: usize = 0x1050;
const CLK_CON_MUX_MUX_CLKCMU_DSP_BUS: usize = 0x1054;
const CLK_CON_MUX_MUX_CLKCMU_FSYS0_BUS: usize = 0x1058;
const CLK_CON_MUX_MUX_CLKCMU_FSYS0_DPGTC: usize = 0x105c;
const CLK_CON_MUX_MUX_CLKCMU_FSYS0_MMC_EMBD: usize = 0x1060;
const CLK_CON_MUX_MUX_CLKCMU_FSYS0_UFS_EMBD: usize = 0x1064;
const CLK_CON_MUX_MUX_CLKCMU_FSYS0_USBDRD30: usize = 0x1068;
const CLK_CON_MUX_MUX_CLKCMU_FSYS1_BUS: usize = 0x106c;
const CLK_CON_MUX_MUX_CLKCMU_FSYS1_MMC_CARD: usize = 0x1070;
const CLK_CON_MUX_MUX_CLKCMU_FSYS1_PCIE: usize = 0x1074;
const CLK_CON_MUX_MUX_CLKCMU_FSYS1_UFS_CARD: usize = 0x1078;
const CLK_CON_MUX_MUX_CLKCMU_G2D_G2D: usize = 0x107c;
const CLK_CON_MUX_MUX_CLKCMU_G2D_JPEG: usize = 0x1080;
const CLK_CON_MUX_MUX_CLKCMU_HPM: usize = 0x1084;
const CLK_CON_MUX_MUX_CLKCMU_IMEM_BUS: usize = 0x1088;
const CLK_CON_MUX_MUX_CLKCMU_ISPHQ_BUS: usize = 0x108c;
const CLK_CON_MUX_MUX_CLKCMU_ISPLP_BUS: usize = 0x1090;
const CLK_CON_MUX_MUX_CLKCMU_IVA_BUS: usize = 0x1094;
const CLK_CON_MUX_MUX_CLKCMU_MFC_BUS: usize = 0x1098;
const CLK_CON_MUX_MUX_CLKCMU_MIF_SWITCH: usize = 0x109c;
const CLK_CON_MUX_MUX_CLKCMU_PERIC0_BUS: usize = 0x10a0;
const CLK_CON_MUX_MUX_CLKCMU_PERIC0_UART_DBG: usize = 0x10a4;
const CLK_CON_MUX_MUX_CLKCMU_PERIC0_USI00: usize = 0x10a8;
const CLK_CON_MUX_MUX_CLKCMU_PERIC0_USI01: usize = 0x10ac;
const CLK_CON_MUX_MUX_CLKCMU_PERIC0_USI02: usize = 0x10b0;
const CLK_CON_MUX_MUX_CLKCMU_PERIC0_USI03: usize = 0x10b4;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_BUS: usize = 0x10b8;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_SPEEDY2: usize = 0x10bc;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_SPI_CAM0: usize = 0x10c0;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_SPI_CAM1: usize = 0x10c4;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_UART_BT: usize = 0x10c8;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI04: usize = 0x10cc;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI05: usize = 0x10d0;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI06: usize = 0x10d4;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI07: usize = 0x10d8;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI08: usize = 0x10dc;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI09: usize = 0x10e0;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI10: usize = 0x10e4;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI11: usize = 0x10e8;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI12: usize = 0x10ec;
const CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI13: usize = 0x10f0;
const CLK_CON_MUX_MUX_CLKCMU_PERIS_BUS: usize = 0x10f4;
const CLK_CON_MUX_MUX_CLKCMU_SRDZ_BUS: usize = 0x10f8;
const CLK_CON_MUX_MUX_CLKCMU_SRDZ_IMGD: usize = 0x10fc;
const CLK_CON_MUX_MUX_CLKCMU_VPU_BUS: usize = 0x1100;
const CLK_CON_MUX_MUX_CLK_CMU_CMUREF: usize = 0x1104;
const CLK_CON_MUX_MUX_CMU_CMUREF: usize = 0x1108;
const CLK_CON_DIV_CLKCMU_ABOX_CPUABOX: usize = 0x1800;
const CLK_CON_DIV_CLKCMU_APM_BUS: usize = 0x1804;
const CLK_CON_DIV_CLKCMU_BUS1_BUS: usize = 0x1808;
const CLK_CON_DIV_CLKCMU_BUSC_BUS: usize = 0x180c;
const CLK_CON_DIV_CLKCMU_BUSC_BUSPHSI2C: usize = 0x1810;
const CLK_CON_DIV_CLKCMU_CAM_BUS: usize = 0x1814;
const CLK_CON_DIV_CLKCMU_CAM_TPU0: usize = 0x1818;
const CLK_CON_DIV_CLKCMU_CAM_TPU1: usize = 0x181c;
const CLK_CON_DIV_CLKCMU_CAM_VRA: usize = 0x1820;
const CLK_CON_DIV_CLKCMU_CIS_CLK0: usize = 0x1824;
const CLK_CON_DIV_CLKCMU_CIS_CLK1: usize = 0x1828;
const CLK_CON_DIV_CLKCMU_CIS_CLK2: usize = 0x182c;
const CLK_CON_DIV_CLKCMU_CIS_CLK3: usize = 0x1830;
const CLK_CON_DIV_CLKCMU_CORE_BUS: usize = 0x1834;
const CLK_CON_DIV_CLKCMU_CPUCL0_SWITCH: usize = 0x1838;
const CLK_CON_DIV_CLKCMU_CPUCL1_SWITCH: usize = 0x183c;
const CLK_CON_DIV_CLKCMU_DBG_BUS: usize = 0x1840;
const CLK_CON_DIV_CLKCMU_DCAM_BUS: usize = 0x1844;
const CLK_CON_DIV_CLKCMU_DCAM_IMGD: usize = 0x1848;
const CLK_CON_DIV_CLKCMU_DPU_BUS: usize = 0x184c;
const CLK_CON_DIV_CLKCMU_DSP_BUS: usize = 0x1850;
const CLK_CON_DIV_CLKCMU_FSYS0_BUS: usize = 0x1854;
const CLK_CON_DIV_CLKCMU_FSYS0_DPGTC: usize = 0x1858;
const CLK_CON_DIV_CLKCMU_FSYS0_MMC_EMBD: usize = 0x185c;
const CLK_CON_DIV_CLKCMU_FSYS0_UFS_EMBD: usize = 0x1860;
const CLK_CON_DIV_CLKCMU_FSYS0_USBDRD30: usize = 0x1864;
const CLK_CON_DIV_CLKCMU_FSYS1_BUS: usize = 0x1868;
const CLK_CON_DIV_CLKCMU_FSYS1_MMC_CARD: usize = 0x186c;
const CLK_CON_DIV_CLKCMU_FSYS1_PCIE: usize = 0x1870;
const CLK_CON_DIV_CLKCMU_FSYS1_UFS_CARD: usize = 0x1874;
const CLK_CON_DIV_CLKCMU_G2D_G2D: usize = 0x1878;
const CLK_CON_DIV_CLKCMU_G2D_JPEG: usize = 0x187c;
const CLK_CON_DIV_CLKCMU_G3D_SWITCH: usize = 0x1880;
const CLK_CON_DIV_CLKCMU_HPM: usize = 0x1884;
const CLK_CON_DIV_CLKCMU_IMEM_BUS: usize = 0x1888;
const CLK_CON_DIV_CLKCMU_ISPHQ_BUS: usize = 0x188c;
const CLK_CON_DIV_CLKCMU_ISPLP_BUS: usize = 0x1890;
const CLK_CON_DIV_CLKCMU_IVA_BUS: usize = 0x1894;
const CLK_CON_DIV_CLKCMU_MFC_BUS: usize = 0x1898;
const CLK_CON_DIV_CLKCMU_MODEM_SHARED0: usize = 0x189c;
const CLK_CON_DIV_CLKCMU_MODEM_SHARED1: usize = 0x18a0;
const CLK_CON_DIV_CLKCMU_OTP: usize = 0x18a4;
const CLK_CON_DIV_CLKCMU_PERIC0_BUS: usize = 0x18a8;
const CLK_CON_DIV_CLKCMU_PERIC0_UART_DBG: usize = 0x18ac;
const CLK_CON_DIV_CLKCMU_PERIC0_USI00: usize = 0x18b0;
const CLK_CON_DIV_CLKCMU_PERIC0_USI01: usize = 0x18b4;
const CLK_CON_DIV_CLKCMU_PERIC0_USI02: usize = 0x18b8;
const CLK_CON_DIV_CLKCMU_PERIC0_USI03: usize = 0x18bc;
const CLK_CON_DIV_CLKCMU_PERIC1_BUS: usize = 0x18c0;
const CLK_CON_DIV_CLKCMU_PERIC1_SPEEDY2: usize = 0x18c4;
const CLK_CON_DIV_CLKCMU_PERIC1_SPI_CAM0: usize = 0x18c8;
const CLK_CON_DIV_CLKCMU_PERIC1_SPI_CAM1: usize = 0x18cc;
const CLK_CON_DIV_CLKCMU_PERIC1_UART_BT: usize = 0x18d0;
const CLK_CON_DIV_CLKCMU_PERIC1_USI04: usize = 0x18d4;
const CLK_CON_DIV_CLKCMU_PERIC1_USI05: usize = 0x18d8;
const CLK_CON_DIV_CLKCMU_PERIC1_USI06: usize = 0x18dc;
const CLK_CON_DIV_CLKCMU_PERIC1_USI07: usize = 0x18e0;
const CLK_CON_DIV_CLKCMU_PERIC1_USI08: usize = 0x18e4;
const CLK_CON_DIV_CLKCMU_PERIC1_USI09: usize = 0x18e8;
const CLK_CON_DIV_CLKCMU_PERIC1_USI10: usize = 0x18ec;
const CLK_CON_DIV_CLKCMU_PERIC1_USI11: usize = 0x18f0;
const CLK_CON_DIV_CLKCMU_PERIC1_USI12: usize = 0x18f4;
const CLK_CON_DIV_CLKCMU_PERIC1_USI13: usize = 0x18f8;
const CLK_CON_DIV_CLKCMU_PERIS_BUS: usize = 0x18fc;
const CLK_CON_DIV_CLKCMU_SRDZ_BUS: usize = 0x1900;
const CLK_CON_DIV_CLKCMU_SRDZ_IMGD: usize = 0x1904;
const CLK_CON_DIV_CLKCMU_VPU_BUS: usize = 0x1908;
const CLK_CON_DIV_DIV_CLK_CMU_CMUREF: usize = 0x190c;
const CLK_CON_DIV_DIV_CP2AP_MIF_CLK_DIV2: usize = 0x1910;
const CLK_CON_DIV_DIV_PLL_SHARED0_DIV2: usize = 0x1914;
const CLK_CON_DIV_DIV_PLL_SHARED0_DIV4: usize = 0x1918;
const CLK_CON_DIV_DIV_PLL_SHARED1_DIV2: usize = 0x191c;
const CLK_CON_DIV_DIV_PLL_SHARED1_DIV4: usize = 0x1920;
const CLK_CON_DIV_DIV_PLL_SHARED2_DIV2: usize = 0x1924;
const CLK_CON_DIV_DIV_PLL_SHARED3_DIV2: usize = 0x1928;
const CLK_CON_DIV_DIV_PLL_SHARED4_DIV2: usize = 0x192c;
const CLK_CON_GAT_CLKCMU_DROOPDETECTOR: usize = 0x2000;
const CLK_CON_GAT_CLKCMU_MIF_SWITCH: usize = 0x2004;
const CLK_CON_GAT_GATE_CLKCMU_ABOX_CPUABOX: usize = 0x2008;
const CLK_CON_GAT_GATE_CLKCMU_APM_BUS: usize = 0x200c;
const CLK_CON_GAT_GATE_CLKCMU_BUS1_BUS: usize = 0x2010;
const CLK_CON_GAT_GATE_CLKCMU_BUSC_BUS: usize = 0x2014;
const CLK_CON_GAT_GATE_CLKCMU_BUSC_BUSPHSI2C: usize = 0x2018;
const CLK_CON_GAT_GATE_CLKCMU_CAM_BUS: usize = 0x201c;
const CLK_CON_GAT_GATE_CLKCMU_CAM_TPU0: usize = 0x2020;
const CLK_CON_GAT_GATE_CLKCMU_CAM_TPU1: usize = 0x2024;
const CLK_CON_GAT_GATE_CLKCMU_CAM_VRA: usize = 0x2028;
const CLK_CON_GAT_GATE_CLKCMU_CIS_CLK0: usize = 0x202c;
const CLK_CON_GAT_GATE_CLKCMU_CIS_CLK1: usize = 0x2030;
const CLK_CON_GAT_GATE_CLKCMU_CIS_CLK2: usize = 0x2034;
const CLK_CON_GAT_GATE_CLKCMU_CIS_CLK3: usize = 0x2038;
const CLK_CON_GAT_GATE_CLKCMU_CORE_BUS: usize = 0x203c;
const CLK_CON_GAT_GATE_CLKCMU_CPUCL0_SWITCH: usize = 0x2040;
const CLK_CON_GAT_GATE_CLKCMU_CPUCL1_SWITCH: usize = 0x2044;
const CLK_CON_GAT_GATE_CLKCMU_DBG_BUS: usize = 0x2048;
const CLK_CON_GAT_GATE_CLKCMU_DCAM_BUS: usize = 0x204c;
const CLK_CON_GAT_GATE_CLKCMU_DCAM_IMGD: usize = 0x2050;
const CLK_CON_GAT_GATE_CLKCMU_DPU_BUS: usize = 0x2054;
const CLK_CON_GAT_GATE_CLKCMU_DSP_BUS: usize = 0x2058;
const CLK_CON_GAT_GATE_CLKCMU_FSYS0_BUS: usize = 0x205c;
const CLK_CON_GAT_GATE_CLKCMU_FSYS0_DPGTC: usize = 0x2060;
const CLK_CON_GAT_GATE_CLKCMU_FSYS0_MMC_EMBD: usize = 0x2064;
const CLK_CON_GAT_GATE_CLKCMU_FSYS0_UFS_EMBD: usize = 0x2068;
const CLK_CON_GAT_GATE_CLKCMU_FSYS0_USBDRD30: usize = 0x206c;
const CLK_CON_GAT_GATE_CLKCMU_FSYS1_BUS: usize = 0x2070;
const CLK_CON_GAT_GATE_CLKCMU_FSYS1_MMC_CARD: usize = 0x2074;
const CLK_CON_GAT_GATE_CLKCMU_FSYS1_PCIE: usize = 0x2078;
const CLK_CON_GAT_GATE_CLKCMU_FSYS1_UFS_CARD: usize = 0x207c;
const CLK_CON_GAT_GATE_CLKCMU_G2D_G2D: usize = 0x2080;
const CLK_CON_GAT_GATE_CLKCMU_G2D_JPEG: usize = 0x2084;
const CLK_CON_GAT_GATE_CLKCMU_G3D_SWITCH: usize = 0x2088;
const CLK_CON_GAT_GATE_CLKCMU_HPM: usize = 0x208c;
const CLK_CON_GAT_GATE_CLKCMU_IMEM_BUS: usize = 0x2090;
const CLK_CON_GAT_GATE_CLKCMU_ISPHQ_BUS: usize = 0x2094;
const CLK_CON_GAT_GATE_CLKCMU_ISPLP_BUS: usize = 0x2098;
const CLK_CON_GAT_GATE_CLKCMU_IVA_BUS: usize = 0x209c;
const CLK_CON_GAT_GATE_CLKCMU_MFC_BUS: usize = 0x20a0;
const CLK_CON_GAT_GATE_CLKCMU_MODEM_SHARED0: usize = 0x20a4;
const CLK_CON_GAT_GATE_CLKCMU_MODEM_SHARED1: usize = 0x20a8;
const CLK_CON_GAT_GATE_CLKCMU_PERIC0_BUS: usize = 0x20ac;
const CLK_CON_GAT_GATE_CLKCMU_PERIC0_UART_DBG: usize = 0x20b0;
const CLK_CON_GAT_GATE_CLKCMU_PERIC0_USI00: usize = 0x20b4;
const CLK_CON_GAT_GATE_CLKCMU_PERIC0_USI01: usize = 0x20b8;
const CLK_CON_GAT_GATE_CLKCMU_PERIC0_USI02: usize = 0x20bc;
const CLK_CON_GAT_GATE_CLKCMU_PERIC0_USI03: usize = 0x20c0;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_BUS: usize = 0x20c4;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_SPEEDY2: usize = 0x20c8;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_SPI_CAM0: usize = 0x20cc;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_SPI_CAM1: usize = 0x20d0;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_UART_BT: usize = 0x20d4;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI04: usize = 0x20d8;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI05: usize = 0x20dc;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI06: usize = 0x20e0;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI07: usize = 0x20e4;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI08: usize = 0x20e8;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI09: usize = 0x20ec;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI10: usize = 0x20f0;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI11: usize = 0x20f4;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI12: usize = 0x20f8;
const CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI13: usize = 0x20fc;
const CLK_CON_GAT_GATE_CLKCMU_PERIS_BUS: usize = 0x2100;
const CLK_CON_GAT_GATE_CLKCMU_SRDZ_BUS: usize = 0x2104;
const CLK_CON_GAT_GATE_CLKCMU_SRDZ_IMGD: usize = 0x2108;
const CLK_CON_GAT_GATE_CLKCMU_VPU_BUS: usize = 0x210c;

static top_clk_regs: static const unsigned long top_clk_regs[]  = {[u64] = &[
	PLL_LOCKTIME_PLL_SHARED0,
	PLL_LOCKTIME_PLL_SHARED1,
	PLL_LOCKTIME_PLL_SHARED2,
	PLL_LOCKTIME_PLL_SHARED3,
	PLL_LOCKTIME_PLL_SHARED4,
	PLL_CON0_MUX_CP2AP_MIF_CLK_USER,
	PLL_CON2_MUX_CP2AP_MIF_CLK_USER,
	PLL_CON0_PLL_SHARED0,
	PLL_CON0_PLL_SHARED1,
	PLL_CON0_PLL_SHARED2,
	PLL_CON0_PLL_SHARED3,
	PLL_CON0_PLL_SHARED4,
	CLK_CON_MUX_MUX_CLKCMU_ABOX_CPUABOX,
	CLK_CON_MUX_MUX_CLKCMU_APM_BUS,
	CLK_CON_MUX_MUX_CLKCMU_BUS1_BUS,
	CLK_CON_MUX_MUX_CLKCMU_BUSC_BUS,
	CLK_CON_MUX_MUX_CLKCMU_BUSC_BUSPHSI2C,
	CLK_CON_MUX_MUX_CLKCMU_CAM_BUS,
	CLK_CON_MUX_MUX_CLKCMU_CAM_TPU0,
	CLK_CON_MUX_MUX_CLKCMU_CAM_TPU1,
	CLK_CON_MUX_MUX_CLKCMU_CAM_VRA,
	CLK_CON_MUX_MUX_CLKCMU_CIS_CLK0,
	CLK_CON_MUX_MUX_CLKCMU_CIS_CLK1,
	CLK_CON_MUX_MUX_CLKCMU_CIS_CLK2,
	CLK_CON_MUX_MUX_CLKCMU_CIS_CLK3,
	CLK_CON_MUX_MUX_CLKCMU_CORE_BUS,
	CLK_CON_MUX_MUX_CLKCMU_CPUCL0_SWITCH,
	CLK_CON_MUX_MUX_CLKCMU_CPUCL1_SWITCH,
	CLK_CON_MUX_MUX_CLKCMU_DBG_BUS,
	CLK_CON_MUX_MUX_CLKCMU_DCAM_BUS,
	CLK_CON_MUX_MUX_CLKCMU_DCAM_IMGD,
	CLK_CON_MUX_MUX_CLKCMU_DPU_BUS,
	CLK_CON_MUX_MUX_CLKCMU_DROOPDETECTOR,
	CLK_CON_MUX_MUX_CLKCMU_DSP_BUS,
	CLK_CON_MUX_MUX_CLKCMU_FSYS0_BUS,
	CLK_CON_MUX_MUX_CLKCMU_FSYS0_DPGTC,
	CLK_CON_MUX_MUX_CLKCMU_FSYS0_MMC_EMBD,
	CLK_CON_MUX_MUX_CLKCMU_FSYS0_UFS_EMBD,
	CLK_CON_MUX_MUX_CLKCMU_FSYS0_USBDRD30,
	CLK_CON_MUX_MUX_CLKCMU_FSYS1_BUS,
	CLK_CON_MUX_MUX_CLKCMU_FSYS1_MMC_CARD,
	CLK_CON_MUX_MUX_CLKCMU_FSYS1_PCIE,
	CLK_CON_MUX_MUX_CLKCMU_FSYS1_UFS_CARD,
	CLK_CON_MUX_MUX_CLKCMU_G2D_G2D,
	CLK_CON_MUX_MUX_CLKCMU_G2D_JPEG,
	CLK_CON_MUX_MUX_CLKCMU_HPM,
	CLK_CON_MUX_MUX_CLKCMU_IMEM_BUS,
	CLK_CON_MUX_MUX_CLKCMU_ISPHQ_BUS,
	CLK_CON_MUX_MUX_CLKCMU_ISPLP_BUS,
	CLK_CON_MUX_MUX_CLKCMU_IVA_BUS,
	CLK_CON_MUX_MUX_CLKCMU_MFC_BUS,
	CLK_CON_MUX_MUX_CLKCMU_MIF_SWITCH,
	CLK_CON_MUX_MUX_CLKCMU_PERIC0_BUS,
	CLK_CON_MUX_MUX_CLKCMU_PERIC0_UART_DBG,
	CLK_CON_MUX_MUX_CLKCMU_PERIC0_USI00,
	CLK_CON_MUX_MUX_CLKCMU_PERIC0_USI01,
	CLK_CON_MUX_MUX_CLKCMU_PERIC0_USI02,
	CLK_CON_MUX_MUX_CLKCMU_PERIC0_USI03,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_BUS,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_SPEEDY2,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_SPI_CAM0,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_SPI_CAM1,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_UART_BT,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI04,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI05,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI06,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI07,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI08,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI09,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI10,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI11,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI12,
	CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI13,
	CLK_CON_MUX_MUX_CLKCMU_PERIS_BUS,
	CLK_CON_MUX_MUX_CLKCMU_SRDZ_BUS,
	CLK_CON_MUX_MUX_CLKCMU_SRDZ_IMGD,
	CLK_CON_MUX_MUX_CLKCMU_VPU_BUS,
	CLK_CON_MUX_MUX_CLK_CMU_CMUREF,
	CLK_CON_MUX_MUX_CMU_CMUREF,
	CLK_CON_DIV_CLKCMU_ABOX_CPUABOX,
	CLK_CON_DIV_CLKCMU_APM_BUS,
	CLK_CON_DIV_CLKCMU_BUS1_BUS,
	CLK_CON_DIV_CLKCMU_BUSC_BUS,
	CLK_CON_DIV_CLKCMU_BUSC_BUSPHSI2C,
	CLK_CON_DIV_CLKCMU_CAM_BUS,
	CLK_CON_DIV_CLKCMU_CAM_TPU0,
	CLK_CON_DIV_CLKCMU_CAM_TPU1,
	CLK_CON_DIV_CLKCMU_CAM_VRA,
	CLK_CON_DIV_CLKCMU_CIS_CLK0,
	CLK_CON_DIV_CLKCMU_CIS_CLK1,
	CLK_CON_DIV_CLKCMU_CIS_CLK2,
	CLK_CON_DIV_CLKCMU_CIS_CLK3,
	CLK_CON_DIV_CLKCMU_CORE_BUS,
	CLK_CON_DIV_CLKCMU_CPUCL0_SWITCH,
	CLK_CON_DIV_CLKCMU_CPUCL1_SWITCH,
	CLK_CON_DIV_CLKCMU_DBG_BUS,
	CLK_CON_DIV_CLKCMU_DCAM_BUS,
	CLK_CON_DIV_CLKCMU_DCAM_IMGD,
	CLK_CON_DIV_CLKCMU_DPU_BUS,
	CLK_CON_DIV_CLKCMU_DSP_BUS,
	CLK_CON_DIV_CLKCMU_FSYS0_BUS,
	CLK_CON_DIV_CLKCMU_FSYS0_DPGTC,
	CLK_CON_DIV_CLKCMU_FSYS0_MMC_EMBD,
	CLK_CON_DIV_CLKCMU_FSYS0_UFS_EMBD,
	CLK_CON_DIV_CLKCMU_FSYS0_USBDRD30,
	CLK_CON_DIV_CLKCMU_FSYS1_BUS,
	CLK_CON_DIV_CLKCMU_FSYS1_MMC_CARD,
	CLK_CON_DIV_CLKCMU_FSYS1_PCIE,
	CLK_CON_DIV_CLKCMU_FSYS1_UFS_CARD,
	CLK_CON_DIV_CLKCMU_G2D_G2D,
	CLK_CON_DIV_CLKCMU_G2D_JPEG,
	CLK_CON_DIV_CLKCMU_G3D_SWITCH,
	CLK_CON_DIV_CLKCMU_HPM,
	CLK_CON_DIV_CLKCMU_IMEM_BUS,
	CLK_CON_DIV_CLKCMU_ISPHQ_BUS,
	CLK_CON_DIV_CLKCMU_ISPLP_BUS,
	CLK_CON_DIV_CLKCMU_IVA_BUS,
	CLK_CON_DIV_CLKCMU_MFC_BUS,
	CLK_CON_DIV_CLKCMU_MODEM_SHARED0,
	CLK_CON_DIV_CLKCMU_MODEM_SHARED1,
	CLK_CON_DIV_CLKCMU_OTP,
	CLK_CON_DIV_CLKCMU_PERIC0_BUS,
	CLK_CON_DIV_CLKCMU_PERIC0_UART_DBG,
	CLK_CON_DIV_CLKCMU_PERIC0_USI00,
	CLK_CON_DIV_CLKCMU_PERIC0_USI01,
	CLK_CON_DIV_CLKCMU_PERIC0_USI02,
	CLK_CON_DIV_CLKCMU_PERIC0_USI03,
	CLK_CON_DIV_CLKCMU_PERIC1_BUS,
	CLK_CON_DIV_CLKCMU_PERIC1_SPEEDY2,
	CLK_CON_DIV_CLKCMU_PERIC1_SPI_CAM0,
	CLK_CON_DIV_CLKCMU_PERIC1_SPI_CAM1,
	CLK_CON_DIV_CLKCMU_PERIC1_UART_BT,
	CLK_CON_DIV_CLKCMU_PERIC1_USI04,
	CLK_CON_DIV_CLKCMU_PERIC1_USI05,
	CLK_CON_DIV_CLKCMU_PERIC1_USI06,
	CLK_CON_DIV_CLKCMU_PERIC1_USI07,
	CLK_CON_DIV_CLKCMU_PERIC1_USI08,
	CLK_CON_DIV_CLKCMU_PERIC1_USI09,
	CLK_CON_DIV_CLKCMU_PERIC1_USI10,
	CLK_CON_DIV_CLKCMU_PERIC1_USI11,
	CLK_CON_DIV_CLKCMU_PERIC1_USI12,
	CLK_CON_DIV_CLKCMU_PERIC1_USI13,
	CLK_CON_DIV_CLKCMU_PERIS_BUS,
	CLK_CON_DIV_CLKCMU_SRDZ_BUS,
	CLK_CON_DIV_CLKCMU_SRDZ_IMGD,
	CLK_CON_DIV_CLKCMU_VPU_BUS,
	CLK_CON_DIV_DIV_CLK_CMU_CMUREF,
	CLK_CON_DIV_DIV_CP2AP_MIF_CLK_DIV2,
	CLK_CON_DIV_DIV_PLL_SHARED0_DIV2,
	CLK_CON_DIV_DIV_PLL_SHARED0_DIV4,
	CLK_CON_DIV_DIV_PLL_SHARED1_DIV2,
	CLK_CON_DIV_DIV_PLL_SHARED1_DIV4,
	CLK_CON_DIV_DIV_PLL_SHARED2_DIV2,
	CLK_CON_DIV_DIV_PLL_SHARED3_DIV2,
	CLK_CON_DIV_DIV_PLL_SHARED4_DIV2,
	CLK_CON_GAT_CLKCMU_DROOPDETECTOR,
	CLK_CON_GAT_CLKCMU_MIF_SWITCH,
	CLK_CON_GAT_GATE_CLKCMU_ABOX_CPUABOX,
	CLK_CON_GAT_GATE_CLKCMU_APM_BUS,
	CLK_CON_GAT_GATE_CLKCMU_BUS1_BUS,
	CLK_CON_GAT_GATE_CLKCMU_BUSC_BUS,
	CLK_CON_GAT_GATE_CLKCMU_BUSC_BUSPHSI2C,
	CLK_CON_GAT_GATE_CLKCMU_CAM_BUS,
	CLK_CON_GAT_GATE_CLKCMU_CAM_TPU0,
	CLK_CON_GAT_GATE_CLKCMU_CAM_TPU1,
	CLK_CON_GAT_GATE_CLKCMU_CAM_VRA,
	CLK_CON_GAT_GATE_CLKCMU_CIS_CLK0,
	CLK_CON_GAT_GATE_CLKCMU_CIS_CLK1,
	CLK_CON_GAT_GATE_CLKCMU_CIS_CLK2,
	CLK_CON_GAT_GATE_CLKCMU_CIS_CLK3,
	CLK_CON_GAT_GATE_CLKCMU_CORE_BUS,
	CLK_CON_GAT_GATE_CLKCMU_CPUCL0_SWITCH,
	CLK_CON_GAT_GATE_CLKCMU_CPUCL1_SWITCH,
	CLK_CON_GAT_GATE_CLKCMU_DBG_BUS,
	CLK_CON_GAT_GATE_CLKCMU_DCAM_BUS,
	CLK_CON_GAT_GATE_CLKCMU_DCAM_IMGD,
	CLK_CON_GAT_GATE_CLKCMU_DPU_BUS,
	CLK_CON_GAT_GATE_CLKCMU_DSP_BUS,
	CLK_CON_GAT_GATE_CLKCMU_FSYS0_BUS,
	CLK_CON_GAT_GATE_CLKCMU_FSYS0_DPGTC,
	CLK_CON_GAT_GATE_CLKCMU_FSYS0_MMC_EMBD,
	CLK_CON_GAT_GATE_CLKCMU_FSYS0_UFS_EMBD,
	CLK_CON_GAT_GATE_CLKCMU_FSYS0_USBDRD30,
	CLK_CON_GAT_GATE_CLKCMU_FSYS1_BUS,
	CLK_CON_GAT_GATE_CLKCMU_FSYS1_MMC_CARD,
	CLK_CON_GAT_GATE_CLKCMU_FSYS1_PCIE,
	CLK_CON_GAT_GATE_CLKCMU_FSYS1_UFS_CARD,
	CLK_CON_GAT_GATE_CLKCMU_G2D_G2D,
	CLK_CON_GAT_GATE_CLKCMU_G2D_JPEG,
	CLK_CON_GAT_GATE_CLKCMU_G3D_SWITCH,
	CLK_CON_GAT_GATE_CLKCMU_HPM,
	CLK_CON_GAT_GATE_CLKCMU_IMEM_BUS,
	CLK_CON_GAT_GATE_CLKCMU_ISPHQ_BUS,
	CLK_CON_GAT_GATE_CLKCMU_ISPLP_BUS,
	CLK_CON_GAT_GATE_CLKCMU_IVA_BUS,
	CLK_CON_GAT_GATE_CLKCMU_MFC_BUS,
	CLK_CON_GAT_GATE_CLKCMU_MODEM_SHARED0,
	CLK_CON_GAT_GATE_CLKCMU_MODEM_SHARED1,
	CLK_CON_GAT_GATE_CLKCMU_PERIC0_BUS,
	CLK_CON_GAT_GATE_CLKCMU_PERIC0_UART_DBG,
	CLK_CON_GAT_GATE_CLKCMU_PERIC0_USI00,
	CLK_CON_GAT_GATE_CLKCMU_PERIC0_USI01,
	CLK_CON_GAT_GATE_CLKCMU_PERIC0_USI02,
	CLK_CON_GAT_GATE_CLKCMU_PERIC0_USI03,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_BUS,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_SPEEDY2,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_SPI_CAM0,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_SPI_CAM1,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_UART_BT,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI04,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI05,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI06,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI07,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI08,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI09,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI10,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI11,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI12,
	CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI13,
	CLK_CON_GAT_GATE_CLKCMU_PERIS_BUS,
	CLK_CON_GAT_GATE_CLKCMU_SRDZ_BUS,
	CLK_CON_GAT_GATE_CLKCMU_SRDZ_IMGD,
	CLK_CON_GAT_GATE_CLKCMU_VPU_BUS,
];

static pll_shared0_rate_table: static const struct samsung_pll_rate_table pll_shared0_rate_table[]  = {[samsung_pll_rate_table] = &[
	pll_35xx_rate(26 * MHZ, 2132000000U, 328, 4, 0),
];

static pll_shared1_rate_table: static const struct samsung_pll_rate_table pll_shared1_rate_table[]  = {[samsung_pll_rate_table] = &[
	pll_35xx_rate(26 * MHZ, 1865500000U, 287, 4, 0),
];

static pll_shared2_rate_table: static const struct samsung_pll_rate_table pll_shared2_rate_table[]  = {[samsung_pll_rate_table] = &[
	pll_35xx_rate(26 * MHZ, 800000000U, 400, 13, 0),
];

static pll_shared3_rate_table: static const struct samsung_pll_rate_table pll_shared3_rate_table[]  = {[samsung_pll_rate_table] = &[
	pll_35xx_rate(26 * MHZ, 630000000U, 315, 13, 0),
];

static pll_shared4_rate_table: static const struct samsung_pll_rate_table pll_shared4_rate_table[]  = {[samsung_pll_rate_table] = &[
	pll_35xx_rate(26 * MHZ, 667333333U, 154, 6, 0),
];

static top_pll_clks: static const struct samsung_pll_clock top_pll_clks[]  = {[samsung_pll_clock] = &[
	/* CMU_TOP_PURECLKCOMP */
	PLL(pll_1051x, CLK_FOUT_SHARED0_PLL, "fout_shared0_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED0, PLL_CON0_PLL_SHARED0,
	    pll_shared0_rate_table),
	PLL(pll_1051x, CLK_FOUT_SHARED1_PLL, "fout_shared1_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED1, PLL_CON0_PLL_SHARED1,
	    pll_shared1_rate_table),
	PLL(pll_1052x, CLK_FOUT_SHARED2_PLL, "fout_shared2_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED2, PLL_CON0_PLL_SHARED2,
	    pll_shared2_rate_table),
	PLL(pll_1052x, CLK_FOUT_SHARED3_PLL, "fout_shared3_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED3, PLL_CON0_PLL_SHARED3,
	    pll_shared3_rate_table),
	PLL(pll_1052x, CLK_FOUT_SHARED4_PLL, "fout_shared4_pll", "oscclk",
	    PLL_LOCKTIME_PLL_SHARED4, PLL_CON0_PLL_SHARED4,
	    pll_shared4_rate_table),
];

/* List of parent clocks for Muxes in CMU_TOP */
static mout_pll_shared0_p: PNAME(mout_pll_shared0_p)		= {[PNAME(mout_pll_shared0_p)		= {str] = &[ "oscclk", "fout_shared0_pll" ];
static mout_pll_shared1_p: PNAME(mout_pll_shared1_p)		= {[PNAME(mout_pll_shared1_p)		= {str] = &[ "oscclk", "fout_shared1_pll" ];
static mout_pll_shared2_p: PNAME(mout_pll_shared2_p)		= {[PNAME(mout_pll_shared2_p)		= {str] = &[ "oscclk", "fout_shared2_pll" ];
static mout_pll_shared3_p: PNAME(mout_pll_shared3_p)		= {[PNAME(mout_pll_shared3_p)		= {str] = &[ "oscclk", "fout_shared3_pll" ];
static mout_pll_shared4_p: PNAME(mout_pll_shared4_p)		= {[PNAME(mout_pll_shared4_p)		= {str] = &[ "oscclk", "fout_shared4_pll" ];
static mout_cp2ap_mif_clk_user_p: PNAME(mout_cp2ap_mif_clk_user_p)	= {[PNAME(mout_cp2ap_mif_clk_user_p)	= {str] = &[ "oscclk" ];
static mout_cmu_abox_cpuabox_p: PNAME(mout_cmu_abox_cpuabox_p)		= {[PNAME(mout_cmu_abox_cpuabox_p)		= {str] = &[ "dout_cmu_shared0_div2",
					    "dout_cmu_shared1_div2",
					    "fout_shared2_pll",
					    "fout_shared4_pll" ];
static mout_cmu_apm_bus_p: PNAME(mout_cmu_apm_bus_p)		= {[PNAME(mout_cmu_apm_bus_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2" ];
static mout_cmu_bus1_bus_p: PNAME(mout_cmu_bus1_bus_p)		= {[PNAME(mout_cmu_bus1_bus_p)		= {str] = &[ "fout_shared4_pll",
					    "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2" ];
static mout_cmu_busc_bus_p: PNAME(mout_cmu_busc_bus_p)		= {[PNAME(mout_cmu_busc_bus_p)		= {str] = &[ "fout_shared4_pll",
					    "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_cp2ap_mif_clk_div2",
					    "oscclk", "oscclk", "oscclk" ];
static mout_cmu_busc_busphsi2c_p: PNAME(mout_cmu_busc_busphsi2c_p)	= {[PNAME(mout_cmu_busc_busphsi2c_p)	= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_cp2ap_mif_clk_div2",
					    "oscclk" ];
static mout_cmu_cam_bus_p: PNAME(mout_cmu_cam_bus_p)		= {[PNAME(mout_cmu_cam_bus_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_cam_tpu0_p: PNAME(mout_cmu_cam_tpu0_p)		= {[PNAME(mout_cmu_cam_tpu0_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_cam_tpu1_p: PNAME(mout_cmu_cam_tpu1_p)		= {[PNAME(mout_cmu_cam_tpu1_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_cam_vra_p: PNAME(mout_cmu_cam_vra_p)		= {[PNAME(mout_cmu_cam_vra_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_cis_clk0_p: PNAME(mout_cmu_cis_clk0_p)		= {[PNAME(mout_cmu_cis_clk0_p)		= {str] = &[ "oscclk",
					    "dout_cmu_shared2_div2" ];
static mout_cmu_cis_clk1_p: PNAME(mout_cmu_cis_clk1_p)		= {[PNAME(mout_cmu_cis_clk1_p)		= {str] = &[ "oscclk",
					    "dout_cmu_shared2_div2" ];
static mout_cmu_cis_clk2_p: PNAME(mout_cmu_cis_clk2_p)		= {[PNAME(mout_cmu_cis_clk2_p)		= {str] = &[ "oscclk",
					    "dout_cmu_shared2_div2" ];
static mout_cmu_cis_clk3_p: PNAME(mout_cmu_cis_clk3_p)		= {[PNAME(mout_cmu_cis_clk3_p)		= {str] = &[ "oscclk",
					    "dout_cmu_shared2_div2" ];
static mout_core_bus_p: PNAME(mout_core_bus_p)			= {[PNAME(mout_core_bus_p)			= {str] = &[ "dout_cmu_shared0_div2",
					    "dout_cmu_shared1_div2",
					    "fout_shared2_pll",
					    "fout_shared4_pll",
					    "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_cp2ap_mif_clk_div2" ];
static mout_cmu_cpucl0_switch_p: PNAME(mout_cmu_cpucl0_switch_p)		= {[PNAME(mout_cmu_cpucl0_switch_p)		= {str] = &[ "dout_cmu_shared0_div2",
					    "dout_cmu_shared1_div2",
					    "fout_shared2_pll",
					    "fout_shared4_pll" ];
static mout_cmu_cpucl1_switch_p: PNAME(mout_cmu_cpucl1_switch_p)		= {[PNAME(mout_cmu_cpucl1_switch_p)		= {str] = &[ "dout_cmu_shared0_div2",
					    "dout_cmu_shared1_div2",
					    "fout_shared2_pll",
					    "fout_shared4_pll" ];
static mout_cmu_dbg_bus_p: PNAME(mout_cmu_dbg_bus_p)		= {[PNAME(mout_cmu_dbg_bus_p)		= {str] = &[ "fout_shared2_pll",
					    "fout_shared4_pll",
					    "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4" ];
static mout_cmu_dcam_bus_p: PNAME(mout_cmu_dcam_bus_p)		= {[PNAME(mout_cmu_dcam_bus_p)		= {str] = &[ "fout_shared4_pll",
					    "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2" ];
static mout_cmu_dcam_imgd_p: PNAME(mout_cmu_dcam_imgd_p)		= {[PNAME(mout_cmu_dcam_imgd_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_dpu_bus_p: PNAME(mout_cmu_dpu_bus_p)		= {[PNAME(mout_cmu_dpu_bus_p)		= {str] = &[ "dout_cmu_shared0_div2",
					    "fout_shared3_pll",
					    "fout_shared4_pll",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "oscclk", "oscclk", "oscclk" ];
static mout_cmu_droopdetector_p: PNAME(mout_cmu_droopdetector_p)		= {[PNAME(mout_cmu_droopdetector_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div2",
					    "dout_cmu_shared1_div2",
					    "fout_shared2_pll" ];
static mout_cmu_dsp_bus_p: PNAME(mout_cmu_dsp_bus_p)		= {[PNAME(mout_cmu_dsp_bus_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_fsys0_bus_p: PNAME(mout_fsys0_bus_p)			= {[PNAME(mout_fsys0_bus_p)			= {str] = &[ "dout_cmu_shared1_div2",
					    "fout_shared2_pll",
					    "fout_shared4_pll",
					    "dout_cmu_shared0_div4" ];
static mout_fsys0_dpgtc_p: PNAME(mout_fsys0_dpgtc_p)		= {[PNAME(mout_fsys0_dpgtc_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_fsys0_mmc_embd_p: PNAME(mout_fsys0_mmc_embd_p)		= {[PNAME(mout_fsys0_mmc_embd_p)		= {str] = &[ "oscclk", "fout_shared2_pll",
					    "fout_shared4_pll",
					    "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "oscclk", "oscclk", "oscclk" ];
static mout_fsys0_ufs_embd_p: PNAME(mout_fsys0_ufs_embd_p)		= {[PNAME(mout_fsys0_ufs_embd_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_fsys0_usbdrd30_p: PNAME(mout_fsys0_usbdrd30_p)		= {[PNAME(mout_fsys0_usbdrd30_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_fsys1_bus_p: PNAME(mout_cmu_fsys1_bus_p)		= {[PNAME(mout_cmu_fsys1_bus_p)		= {str] = &[ "fout_shared2_pll",
					    "dout_cmu_shared0_div4" ];
static mout_cmu_fsys1_mmc_card_p: PNAME(mout_cmu_fsys1_mmc_card_p)	= {[PNAME(mout_cmu_fsys1_mmc_card_p)	= {str] = &[ "oscclk", "fout_shared2_pll",
					    "fout_shared4_pll",
					    "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "oscclk", "oscclk", "oscclk" ];
static mout_cmu_fsys1_pcie_p: PNAME(mout_cmu_fsys1_pcie_p)		= {[PNAME(mout_cmu_fsys1_pcie_p)		= {str] = &[ "oscclk", "fout_shared2_pll" ];
static mout_cmu_fsys1_ufs_card_p: PNAME(mout_cmu_fsys1_ufs_card_p)	= {[PNAME(mout_cmu_fsys1_ufs_card_p)	= {str] = &[ "oscclk",
					    "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_g2d_g2d_p: PNAME(mout_cmu_g2d_g2d_p)		= {[PNAME(mout_cmu_g2d_g2d_p)		= {str] = &[ "fout_shared4_pll",
					    "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2" ];
static mout_cmu_g2d_jpeg_p: PNAME(mout_cmu_g2d_jpeg_p)		= {[PNAME(mout_cmu_g2d_jpeg_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_hpm_p: PNAME(mout_cmu_hpm_p)			= {[PNAME(mout_cmu_hpm_p)			= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2" ];
static mout_cmu_imem_bus_p: PNAME(mout_cmu_imem_bus_p)		= {[PNAME(mout_cmu_imem_bus_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_isphq_bus_p: PNAME(mout_cmu_isphq_bus_p)		= {[PNAME(mout_cmu_isphq_bus_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_isplp_bus_p: PNAME(mout_cmu_isplp_bus_p)		= {[PNAME(mout_cmu_isplp_bus_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_iva_bus_p: PNAME(mout_cmu_iva_bus_p)		= {[PNAME(mout_cmu_iva_bus_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_mfc_bus_p: PNAME(mout_cmu_mfc_bus_p)		= {[PNAME(mout_cmu_mfc_bus_p)		= {str] = &[ "fout_shared4_pll",
					    "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2" ];
static mout_cmu_mif_switch_p: PNAME(mout_cmu_mif_switch_p)		= {[PNAME(mout_cmu_mif_switch_p)		= {str] = &[ "fout_shared0_pll",
					    "fout_shared1_pll",
					    "dout_cmu_shared0_div2",
					    "dout_cmu_shared1_div2",
					    "fout_shared2_pll",
					    "mout_cp2ap_mif_clk_user",
					    "oscclk", "oscclk" ];
static mout_cmu_peric0_bus_p: PNAME(mout_cmu_peric0_bus_p)		= {[PNAME(mout_cmu_peric0_bus_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2" ];
static mout_cmu_peric0_uart_dbg_p: PNAME(mout_cmu_peric0_uart_dbg_p)	= {[PNAME(mout_cmu_peric0_uart_dbg_p)	= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric0_usi00_p: PNAME(mout_cmu_peric0_usi00_p)		= {[PNAME(mout_cmu_peric0_usi00_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric0_usi01_p: PNAME(mout_cmu_peric0_usi01_p)		= {[PNAME(mout_cmu_peric0_usi01_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric0_usi02_p: PNAME(mout_cmu_peric0_usi02_p)		= {[PNAME(mout_cmu_peric0_usi02_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric0_usi03_p: PNAME(mout_cmu_peric0_usi03_p)		= {[PNAME(mout_cmu_peric0_usi03_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_bus_p: PNAME(mout_cmu_peric1_bus_p)		= {[PNAME(mout_cmu_peric1_bus_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2" ];
static mout_cmu_peric1_speedy2_p: PNAME(mout_cmu_peric1_speedy2_p)	= {[PNAME(mout_cmu_peric1_speedy2_p)	= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "oscclk" ];
static mout_cmu_peric1_spi_cam0_p: PNAME(mout_cmu_peric1_spi_cam0_p)	= {[PNAME(mout_cmu_peric1_spi_cam0_p)	= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_spi_cam1_p: PNAME(mout_cmu_peric1_spi_cam1_p)	= {[PNAME(mout_cmu_peric1_spi_cam1_p)	= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_uart_bt_p: PNAME(mout_cmu_peric1_uart_bt_p)	= {[PNAME(mout_cmu_peric1_uart_bt_p)	= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_usi04_p: PNAME(mout_cmu_peric1_usi04_p)		= {[PNAME(mout_cmu_peric1_usi04_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_usi05_p: PNAME(mout_cmu_peric1_usi05_p)		= {[PNAME(mout_cmu_peric1_usi05_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_usi06_p: PNAME(mout_cmu_peric1_usi06_p)		= {[PNAME(mout_cmu_peric1_usi06_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_usi07_p: PNAME(mout_cmu_peric1_usi07_p)		= {[PNAME(mout_cmu_peric1_usi07_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_usi08_p: PNAME(mout_cmu_peric1_usi08_p)		= {[PNAME(mout_cmu_peric1_usi08_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_usi09_p: PNAME(mout_cmu_peric1_usi09_p)		= {[PNAME(mout_cmu_peric1_usi09_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_usi10_p: PNAME(mout_cmu_peric1_usi10_p)		= {[PNAME(mout_cmu_peric1_usi10_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_usi11_p: PNAME(mout_cmu_peric1_usi11_p)		= {[PNAME(mout_cmu_peric1_usi11_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_usi12_p: PNAME(mout_cmu_peric1_usi12_p)		= {[PNAME(mout_cmu_peric1_usi12_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peric1_usi13_p: PNAME(mout_cmu_peric1_usi13_p)		= {[PNAME(mout_cmu_peric1_usi13_p)		= {str] = &[ "oscclk", "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_peris_bus_p: PNAME(mout_cmu_peris_bus_p)		= {[PNAME(mout_cmu_peris_bus_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared2_div2" ];
static mout_cmu_srdz_bus_p: PNAME(mout_cmu_srdz_bus_p)		= {[PNAME(mout_cmu_srdz_bus_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_srdz_imgd_p: PNAME(mout_cmu_srdz_imgd_p)		= {[PNAME(mout_cmu_srdz_imgd_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];
static mout_cmu_vpu_bus_p: PNAME(mout_cmu_vpu_bus_p)		= {[PNAME(mout_cmu_vpu_bus_p)		= {str] = &[ "dout_cmu_shared0_div4",
					    "dout_cmu_shared1_div4",
					    "dout_cmu_shared2_div2",
					    "dout_cmu_shared4_div2" ];

/*
 * Register name to clock name mangling strategy used in this file
 *
 * Replace PLL_CON0_PLL	           with CLK_MOUT_PLL and mout_pll
 * Replace CLK_CON_MUX_MUX_CLKCMU  with CLK_MOUT_CMU and mout_cmu
 * Replace CLK_CON_DIV_CLKCMU      with CLK_DOUT_CMU and dout_cmu
 * Replace CLK_CON_DIV_DIV_CLKCMU  with CLK_DOUT_CMU and dout_cmu
 * Replace CLK_CON_GAT_CLKCMU      with CLK_GOUT_CMU and gout_cmu
 * Replace CLK_CON_GAT_GATE_CLKCMU with CLK_GOUT_CMU and gout_cmu
 *
 * For gates remove _UID _BLK _IPCLKPORT and _RSTNSYNC
 */

static top_mux_clks: static const struct samsung_mux_clock top_mux_clks[]  = {[samsung_mux_clock] = &[
	MUX(CLK_MOUT_PLL_SHARED0, "mout_pll_shared0", mout_pll_shared0_p,
	    PLL_CON0_PLL_SHARED0, 4, 1),
	MUX(CLK_MOUT_PLL_SHARED1, "mout_pll_shared1", mout_pll_shared1_p,
	    PLL_CON0_PLL_SHARED1, 4, 1),
	MUX(CLK_MOUT_PLL_SHARED2, "mout_pll_shared2", mout_pll_shared2_p,
	    PLL_CON0_PLL_SHARED2, 4, 1),
	MUX(CLK_MOUT_PLL_SHARED3, "mout_pll_shared3", mout_pll_shared3_p,
	    PLL_CON0_PLL_SHARED3, 4, 1),
	MUX(CLK_MOUT_PLL_SHARED4, "mout_pll_shared4", mout_pll_shared4_p,
	    PLL_CON0_PLL_SHARED4, 4, 1),
	MUX(CLK_MOUT_CP2AP_MIF_CLK_USER, "mout_cp2ap_mif_clk_user",
	    mout_cp2ap_mif_clk_user_p, PLL_CON0_MUX_CP2AP_MIF_CLK_USER, 4, 1),
	MUX(CLK_MOUT_CMU_ABOX_CPUABOX, "mout_cmu_abox_cpuabox",
	    mout_cmu_abox_cpuabox_p, CLK_CON_MUX_MUX_CLKCMU_ABOX_CPUABOX,
	    0, 2),
	MUX(CLK_MOUT_CMU_APM_BUS, "mout_cmu_apm_bus", mout_cmu_apm_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_APM_BUS, 0, 1),
	MUX(CLK_MOUT_CMU_BUS1_BUS, "mout_cmu_bus1_bus", mout_cmu_bus1_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_BUS1_BUS, 0, 2),
	MUX(CLK_MOUT_CMU_BUSC_BUS, "mout_cmu_busc_bus", mout_cmu_busc_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_BUSC_BUS, 0, 3),
	MUX(CLK_MOUT_CMU_BUSC_BUSPHSI2C, "mout_cmu_busc_busphsi2c",
	    mout_cmu_busc_busphsi2c_p, CLK_CON_MUX_MUX_CLKCMU_BUSC_BUSPHSI2C,
	    0, 2),
	MUX(CLK_MOUT_CMU_CAM_BUS, "mout_cmu_cam_bus", mout_cmu_cam_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_CAM_BUS, 0, 2),
	MUX(CLK_MOUT_CMU_CAM_TPU0, "mout_cmu_cam_tpu0", mout_cmu_cam_tpu0_p,
	    CLK_CON_MUX_MUX_CLKCMU_CAM_TPU0, 0, 2),
	MUX(CLK_MOUT_CMU_CAM_TPU1, "mout_cmu_cam_tpu1", mout_cmu_cam_tpu1_p,
	    CLK_CON_MUX_MUX_CLKCMU_CAM_TPU1, 0, 2),
	MUX(CLK_MOUT_CMU_CAM_VRA, "mout_cmu_cam_vra", mout_cmu_cam_vra_p,
	    CLK_CON_MUX_MUX_CLKCMU_CAM_VRA, 0, 2),
	MUX(CLK_MOUT_CMU_CIS_CLK0, "mout_cmu_cis_clk0", mout_cmu_cis_clk0_p,
	    CLK_CON_MUX_MUX_CLKCMU_CIS_CLK0, 0, 1),
	MUX(CLK_MOUT_CMU_CIS_CLK1, "mout_cmu_cis_clk1", mout_cmu_cis_clk1_p,
	    CLK_CON_MUX_MUX_CLKCMU_CIS_CLK1, 0, 1),
	MUX(CLK_MOUT_CMU_CIS_CLK2, "mout_cmu_cis_clk2", mout_cmu_cis_clk2_p,
	    CLK_CON_MUX_MUX_CLKCMU_CIS_CLK2, 0, 1),
	MUX(CLK_MOUT_CMU_CIS_CLK3, "mout_cmu_cis_clk3", mout_cmu_cis_clk3_p,
	    CLK_CON_MUX_MUX_CLKCMU_CIS_CLK3, 0, 1),
	MUX(CLK_MOUT_CMU_CORE_BUS, "mout_core_bus", mout_core_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_CORE_BUS, 0, 3),
	MUX(CLK_MOUT_CMU_CPUCL0_SWITCH, "mout_cmu_cpucl0_switch",
	    mout_cmu_cpucl0_switch_p, CLK_CON_MUX_MUX_CLKCMU_CPUCL0_SWITCH,
	    0, 2),
	MUX(CLK_MOUT_CMU_CPUCL1_SWITCH, "mout_cmu_cpucl1_switch",
	    mout_cmu_cpucl1_switch_p, CLK_CON_MUX_MUX_CLKCMU_CPUCL1_SWITCH,
	    0, 2),
	MUX(CLK_MOUT_CMU_DBG_BUS, "mout_cmu_dbg_bus", mout_cmu_dbg_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_DBG_BUS, 0, 2),
	MUX(CLK_MOUT_CMU_DCAM_BUS, "mout_cmu_dcam_bus", mout_cmu_dcam_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_DCAM_BUS, 0, 2),
	MUX(CLK_MOUT_CMU_DCAM_IMGD, "mout_cmu_dcam_imgd", mout_cmu_dcam_imgd_p,
	    CLK_CON_MUX_MUX_CLKCMU_DCAM_IMGD, 0, 2),
	MUX(CLK_MOUT_CMU_DPU_BUS, "mout_cmu_dpu_bus", mout_cmu_dpu_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_DPU_BUS, 0, 3),
	MUX(CLK_MOUT_CMU_DROOPDETECTOR, "mout_cmu_droopdetector",
	    mout_cmu_droopdetector_p, CLK_CON_MUX_MUX_CLKCMU_DROOPDETECTOR,
	    0, 2),
	MUX(CLK_MOUT_CMU_DSP_BUS, "mout_cmu_dsp_bus", mout_cmu_dsp_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_DSP_BUS, 0, 2),
	MUX(CLK_MOUT_CMU_FSYS0_BUS, "mout_fsys0_bus", mout_fsys0_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_FSYS0_BUS, 0, 2),
	MUX(CLK_MOUT_CMU_FSYS0_DPGTC, "mout_fsys0_dpgtc", mout_fsys0_dpgtc_p,
	    CLK_CON_MUX_MUX_CLKCMU_FSYS0_DPGTC, 0, 2),
	MUX(CLK_MOUT_CMU_FSYS0_MMC_EMBD, "mout_fsys0_mmc_embd",
	    mout_fsys0_mmc_embd_p, CLK_CON_MUX_MUX_CLKCMU_FSYS0_MMC_EMBD,
	    0, 3),
	MUX(CLK_MOUT_CMU_FSYS0_UFS_EMBD, "mout_fsys0_ufs_embd",
	    mout_fsys0_ufs_embd_p, CLK_CON_MUX_MUX_CLKCMU_FSYS0_UFS_EMBD,
	    0, 2),
	MUX(CLK_MOUT_CMU_FSYS0_USBDRD30, "mout_fsys0_usbdrd30",
	    mout_fsys0_usbdrd30_p, CLK_CON_MUX_MUX_CLKCMU_FSYS0_USBDRD30,
	    0, 2),
	MUX(CLK_MOUT_CMU_FSYS1_BUS, "mout_cmu_fsys1_bus", mout_cmu_fsys1_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_FSYS1_BUS, 0, 1),
	MUX(CLK_MOUT_CMU_FSYS1_MMC_CARD, "mout_cmu_fsys1_mmc_card",
	    mout_cmu_fsys1_mmc_card_p, CLK_CON_MUX_MUX_CLKCMU_FSYS1_MMC_CARD,
	    0, 3),
	MUX(CLK_MOUT_CMU_FSYS1_PCIE, "mout_cmu_fsys1_pcie",
	    mout_cmu_fsys1_pcie_p, CLK_CON_MUX_MUX_CLKCMU_FSYS1_PCIE, 0, 1),
	MUX(CLK_MOUT_CMU_FSYS1_UFS_CARD, "mout_cmu_fsys1_ufs_card",
	    mout_cmu_fsys1_ufs_card_p, CLK_CON_MUX_MUX_CLKCMU_FSYS1_UFS_CARD,
	    0, 2),
	MUX(CLK_MOUT_CMU_G2D_G2D, "mout_cmu_g2d_g2d", mout_cmu_g2d_g2d_p,
	    CLK_CON_MUX_MUX_CLKCMU_G2D_G2D, 0, 2),
	MUX(CLK_MOUT_CMU_G2D_JPEG, "mout_cmu_g2d_jpeg", mout_cmu_g2d_jpeg_p,
	    CLK_CON_MUX_MUX_CLKCMU_G2D_JPEG, 0, 2),
	MUX(CLK_MOUT_CMU_HPM, "mout_cmu_hpm", mout_cmu_hpm_p,
	    CLK_CON_MUX_MUX_CLKCMU_HPM, 0, 2),
	MUX(CLK_MOUT_CMU_IMEM_BUS, "mout_cmu_imem_bus", mout_cmu_imem_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_IMEM_BUS, 0, 2),
	MUX(CLK_MOUT_CMU_ISPHQ_BUS, "mout_cmu_isphq_bus", mout_cmu_isphq_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_ISPHQ_BUS, 0, 2),
	MUX(CLK_MOUT_CMU_ISPLP_BUS, "mout_cmu_isplp_bus", mout_cmu_isplp_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_ISPLP_BUS, 0, 2),
	MUX(CLK_MOUT_CMU_IVA_BUS, "mout_cmu_iva_bus", mout_cmu_iva_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_IVA_BUS, 0, 2),
	MUX(CLK_MOUT_CMU_MFC_BUS, "mout_cmu_mfc_bus", mout_cmu_mfc_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_MFC_BUS, 0, 2),
	MUX(CLK_MOUT_CMU_MIF_SWITCH, "mout_cmu_mif_switch",
	    mout_cmu_mif_switch_p, CLK_CON_MUX_MUX_CLKCMU_MIF_SWITCH, 0, 3),
	MUX(CLK_MOUT_CMU_PERIC0_BUS, "mout_cmu_peric0_bus",
	    mout_cmu_peric0_bus_p, CLK_CON_MUX_MUX_CLKCMU_PERIC0_BUS, 0, 1),
	MUX(CLK_MOUT_CMU_PERIC0_UART_DBG, "mout_cmu_peric0_uart_dbg",
	    mout_cmu_peric0_uart_dbg_p, CLK_CON_MUX_MUX_CLKCMU_PERIC0_UART_DBG,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC0_USI00, "mout_cmu_peric0_usi00",
	    mout_cmu_peric0_usi00_p, CLK_CON_MUX_MUX_CLKCMU_PERIC0_USI00,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC0_USI01, "mout_cmu_peric0_usi01",
	    mout_cmu_peric0_usi01_p, CLK_CON_MUX_MUX_CLKCMU_PERIC0_USI01,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC0_USI02, "mout_cmu_peric0_usi02",
	    mout_cmu_peric0_usi02_p, CLK_CON_MUX_MUX_CLKCMU_PERIC0_USI02,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC0_USI03, "mout_cmu_peric0_usi03",
	    mout_cmu_peric0_usi03_p, CLK_CON_MUX_MUX_CLKCMU_PERIC0_USI03,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_BUS, "mout_cmu_peric1_bus",
	    mout_cmu_peric1_bus_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_BUS, 0, 1),
	MUX(CLK_MOUT_CMU_PERIC1_SPEEDY2, "mout_cmu_peric1_speedy2",
	    mout_cmu_peric1_speedy2_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_SPEEDY2,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_SPI_CAM0, "mout_cmu_peric1_spi_cam0",
	    mout_cmu_peric1_spi_cam0_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_SPI_CAM0,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_SPI_CAM1, "mout_cmu_peric1_spi_cam1",
	    mout_cmu_peric1_spi_cam1_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_SPI_CAM1,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_UART_BT, "mout_cmu_peric1_uart_bt",
	    mout_cmu_peric1_uart_bt_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_UART_BT,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_USI04, "mout_cmu_peric1_usi04",
	    mout_cmu_peric1_usi04_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI04,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_USI05, "mout_cmu_peric1_usi05",
	    mout_cmu_peric1_usi05_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI05,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_USI06, "mout_cmu_peric1_usi06",
	    mout_cmu_peric1_usi06_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI06,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_USI07, "mout_cmu_peric1_usi07",
	    mout_cmu_peric1_usi07_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI07,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_USI08, "mout_cmu_peric1_usi08",
	    mout_cmu_peric1_usi08_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI08,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_USI09, "mout_cmu_peric1_usi09",
	    mout_cmu_peric1_usi09_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI09,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_USI10, "mout_cmu_peric1_usi10",
	    mout_cmu_peric1_usi10_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI10,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_USI11, "mout_cmu_peric1_usi11",
	    mout_cmu_peric1_usi11_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI11,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_USI12, "mout_cmu_peric1_usi12",
	    mout_cmu_peric1_usi12_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI12,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIC1_USI13, "mout_cmu_peric1_usi13",
	    mout_cmu_peric1_usi13_p, CLK_CON_MUX_MUX_CLKCMU_PERIC1_USI13,
	    0, 2),
	MUX(CLK_MOUT_CMU_PERIS_BUS, "mout_cmu_peris_bus", mout_cmu_peris_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_PERIS_BUS, 0, 1),
	MUX(CLK_MOUT_CMU_SRDZ_BUS, "mout_cmu_srdz_bus", mout_cmu_srdz_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_SRDZ_BUS, 0, 2),
	MUX(CLK_MOUT_CMU_SRDZ_IMGD, "mout_cmu_srdz_imgd", mout_cmu_srdz_imgd_p,
	    CLK_CON_MUX_MUX_CLKCMU_SRDZ_IMGD, 0, 2),
	MUX(CLK_MOUT_CMU_VPU_BUS, "mout_cmu_vpu_bus", mout_cmu_vpu_bus_p,
	    CLK_CON_MUX_MUX_CLKCMU_VPU_BUS, 0, 2),
];

static top_div_clks: static const struct samsung_div_clock top_div_clks[]  = {[samsung_div_clock] = &[
	DIV(CLK_DOUT_CMU_ABOX_CPUABOX, "dout_cmu_cmu_abox_cpuabox",
	    "gout_cmu_abox_cpuabox", CLK_CON_DIV_CLKCMU_ABOX_CPUABOX, 0, 3),
	DIV(CLK_DOUT_CMU_APM_BUS, "dout_cmu_apm_bus", "gout_cmu_apm_bus",
	    CLK_CON_DIV_CLKCMU_APM_BUS, 0, 3),
	DIV(CLK_DOUT_CMU_BUS1_BUS, "dout_cmu_bus1_bus", "gout_cmu_bus1_bus",
	    CLK_CON_DIV_CLKCMU_BUS1_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_BUSC_BUS, "dout_cmu_busc_bus",
	    "gout_cmu_clkcmu_busc_bus", CLK_CON_DIV_CLKCMU_BUSC_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_BUSC_BUSPHSI2C, "dout_cmu_busc_busphsi2c",
	    "gout_cmu_busc_busphsi2c", CLK_CON_DIV_CLKCMU_BUSC_BUSPHSI2C,
	    0, 4),
	DIV(CLK_DOUT_CMU_CAM_BUS, "dout_cmu_cam_bus", "gout_cmu_cam_bus",
	    CLK_CON_DIV_CLKCMU_CAM_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_CAM_TPU0, "dout_cmu_cam_tpu0", "gout_cmu_cam_tpu0",
	    CLK_CON_DIV_CLKCMU_CAM_TPU0, 0, 4),
	DIV(CLK_DOUT_CMU_CAM_TPU1, "dout_cmu_cam_tpu1", "gout_cmu_cam_tpu1",
	    CLK_CON_DIV_CLKCMU_CAM_TPU1, 0, 4),
	DIV(CLK_DOUT_CMU_CAM_VRA, "dout_cmu_cam_vra", "gout_cmu_cam_vra",
	    CLK_CON_DIV_CLKCMU_CAM_VRA, 0, 4),
	DIV(CLK_DOUT_CMU_CIS_CLK0, "dout_cmu_cis_clk0", "gout_cmu_cis_clk0",
	    CLK_CON_DIV_CLKCMU_CIS_CLK0, 0, 5),
	DIV(CLK_DOUT_CMU_CIS_CLK1, "dout_cmu_cis_clk1", "gout_cmu_cis_clk1",
	    CLK_CON_DIV_CLKCMU_CIS_CLK1, 0, 5),
	DIV(CLK_DOUT_CMU_CIS_CLK2, "dout_cmu_cis_clk2", "gout_cmu_cis_clk2",
	    CLK_CON_DIV_CLKCMU_CIS_CLK2, 0, 5),
	DIV(CLK_DOUT_CMU_CIS_CLK3, "dout_cmu_cis_clk3", "gout_cmu_cis_clk3",
	    CLK_CON_DIV_CLKCMU_CIS_CLK3, 0, 5),
	DIV(CLK_DOUT_CMU_CORE_BUS, "dout_cmu_core_bus", "gout_cmu_core_bus",
	    CLK_CON_DIV_CLKCMU_CORE_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_CPUCL0_SWITCH, "dout_cmu_cpucl0_switch",
	    "gout_cmu_cpucl0_switch", CLK_CON_DIV_CLKCMU_CPUCL0_SWITCH, 0, 3),
	DIV(CLK_DOUT_CMU_CPUCL1_SWITCH, "dout_cmu_cpucl1_switch",
	    "gout_cmu_cpucl1_switch", CLK_CON_DIV_CLKCMU_CPUCL1_SWITCH, 0, 3),
	DIV(CLK_DOUT_CMU_DBG_BUS, "dout_cmu_dbg_bus", "gout_cmu_dbg_bus",
	    CLK_CON_DIV_CLKCMU_DBG_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_DCAM_BUS, "dout_cmu_dcam_bus", "gout_cmu_dcam_bus",
	    CLK_CON_DIV_CLKCMU_DCAM_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_DCAM_IMGD, "dout_cmu_dcam_imgd", "gout_cmu_dcam_imgd",
	    CLK_CON_DIV_CLKCMU_DCAM_IMGD, 0, 4),
	DIV(CLK_DOUT_CMU_DPU_BUS, "dout_cmu_dpu_bus", "gout_cmu_dpu_bus",
	    CLK_CON_DIV_CLKCMU_DPU_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_DSP_BUS, "dout_cmu_dsp_bus", "gout_cmu_dsp_bus",
	    CLK_CON_DIV_CLKCMU_DSP_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_FSYS0_BUS, "dout_cmu_fsys0_bus", "gout_cmu_fsys0_bus",
	    CLK_CON_DIV_CLKCMU_FSYS0_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_FSYS0_DPGTC, "dout_cmu_fsys0_dpgtc",
	    "gout_cmu_fsys0_dpgtc", CLK_CON_DIV_CLKCMU_FSYS0_DPGTC, 0, 3),
	DIV(CLK_DOUT_CMU_FSYS0_MMC_EMBD, "dout_cmu_fsys0_mmc_embd",
	    "gout_cmu_fsys0_mmc_embd", CLK_CON_DIV_CLKCMU_FSYS0_MMC_EMBD,
	    0, 9),
	DIV(CLK_DOUT_CMU_FSYS0_UFS_EMBD, "dout_cmu_fsys0_ufs_embd",
	    "gout_cmu_fsys0_ufs_embd", CLK_CON_DIV_CLKCMU_FSYS0_UFS_EMBD,
	    0, 3),
	DIV(CLK_DOUT_CMU_FSYS0_USBDRD30, "dout_cmu_fsys0_usbdrd30",
	    "gout_cmu_fsys0_usbdrd30", CLK_CON_DIV_CLKCMU_FSYS0_USBDRD30,
	    0, 4),
	DIV(CLK_DOUT_CMU_FSYS1_BUS, "dout_cmu_fsys1_bus", "gout_cmu_fsys1_bus",
	    CLK_CON_DIV_CLKCMU_FSYS1_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_FSYS1_MMC_CARD, "dout_cmu_fsys1_mmc_card",
	    "gout_cmu_fsys1_mmc_card", CLK_CON_DIV_CLKCMU_FSYS1_MMC_CARD,
	    0, 9),
	DIV(CLK_DOUT_CMU_FSYS1_UFS_CARD, "dout_cmu_fsys1_ufs_card",
	    "gout_cmu_fsys1_ufs_card", CLK_CON_DIV_CLKCMU_FSYS1_UFS_CARD,
	    0, 4),
	DIV(CLK_DOUT_CMU_G2D_G2D, "dout_cmu_g2d_g2d", "gout_cmu_g2d_g2d",
	    CLK_CON_DIV_CLKCMU_G2D_G2D, 0, 4),
	DIV(CLK_DOUT_CMU_G2D_JPEG, "dout_cmu_g2d_jpeg", "gout_cmu_g2d_jpeg",
	    CLK_CON_DIV_CLKCMU_G2D_JPEG, 0, 4),
	DIV(CLK_DOUT_CMU_G3D_SWITCH, "dout_cmu_g3d_switch",
	    "gout_cmu_g3d_switch", CLK_CON_DIV_CLKCMU_G3D_SWITCH, 0, 3),
	DIV(CLK_DOUT_CMU_HPM, "dout_cmu_hpm", "gout_cmu_hpm",
	    CLK_CON_DIV_CLKCMU_HPM, 0, 2),
	DIV(CLK_DOUT_CMU_IMEM_BUS, "dout_cmu_imem_bus", "gout_cmu_imem_bus",
	    CLK_CON_DIV_CLKCMU_IMEM_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_ISPHQ_BUS, "dout_cmu_isphq_bus", "gout_cmu_isphq_bus",
	    CLK_CON_DIV_CLKCMU_ISPHQ_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_ISPLP_BUS, "dout_cmu_isplp_bus", "gout_cmu_isplp_bus",
	    CLK_CON_DIV_CLKCMU_ISPLP_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_IVA_BUS, "dout_cmu_iva_bus", "gout_cmu_iva_bus",
	    CLK_CON_DIV_CLKCMU_IVA_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_MFC_BUS, "dout_cmu_mfc_bus", "gout_cmu_mfc_bus",
	    CLK_CON_DIV_CLKCMU_MFC_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_MODEM_SHARED0, "dout_cmu_modem_shared0",
	    "gout_cmu_modem_shared0", CLK_CON_DIV_CLKCMU_MODEM_SHARED0, 0, 3),
	DIV(CLK_DOUT_CMU_MODEM_SHARED1, "dout_cmu_modem_shared1",
	    "gout_cmu_modem_shared1", CLK_CON_DIV_CLKCMU_MODEM_SHARED1, 0, 3),
	DIV(CLK_DOUT_CMU_PERIC0_BUS, "dout_cmu_peric0_bus",
	    "gout_cmu_peric0_bus", CLK_CON_DIV_CLKCMU_PERIC0_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC0_UART_DBG, "dout_cmu_peric0_uart_dbg",
	    "gout_cmu_peric0_uart_dbg", CLK_CON_DIV_CLKCMU_PERIC0_UART_DBG,
	    0, 4),
	DIV(CLK_DOUT_CMU_PERIC0_USI00, "dout_cmu_peric0_usi00",
	    "gout_cmu_peric0_usi00", CLK_CON_DIV_CLKCMU_PERIC0_USI00, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC0_USI01, "dout_cmu_peric0_usi01",
	    "gout_cmu_peric0_usi01", CLK_CON_DIV_CLKCMU_PERIC0_USI01, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC0_USI02, "dout_cmu_peric0_usi02",
	    "gout_cmu_peric0_usi02", CLK_CON_DIV_CLKCMU_PERIC0_USI02, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC0_USI03, "dout_cmu_peric0_usi03",
	    "gout_cmu_peric0_usi03", CLK_CON_DIV_CLKCMU_PERIC0_USI03, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_BUS, "dout_cmu_peric1_bus",
	    "gout_cmu_peric1_bus", CLK_CON_DIV_CLKCMU_PERIC1_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_SPEEDY2, "dout_cmu_peric1_speedy2",
	    "gout_cmu_peric1_speedy2", CLK_CON_DIV_CLKCMU_PERIC1_SPEEDY2,
	    0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_SPI_CAM0, "dout_cmu_peric1_spi_cam0",
	    "gout_cmu_peric1_spi_cam0", CLK_CON_DIV_CLKCMU_PERIC1_SPI_CAM0,
	    0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_SPI_CAM1, "dout_cmu_peric1_spi_cam1",
	    "gout_cmu_peric1_spi_cam1", CLK_CON_DIV_CLKCMU_PERIC1_SPI_CAM1,
	    0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_UART_BT, "dout_cmu_peric1_uart_bt",
	    "gout_cmu_peric1_uart_bt", CLK_CON_DIV_CLKCMU_PERIC1_UART_BT,
	    0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_USI04, "dout_cmu_peric1_usi04",
	    "gout_cmu_peric1_usi04", CLK_CON_DIV_CLKCMU_PERIC1_USI04, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_USI05, "dout_cmu_peric1_usi05",
	    "gout_cmu_peric1_usi05", CLK_CON_DIV_CLKCMU_PERIC1_USI05, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_USI06, "dout_cmu_peric1_usi06",
	    "gout_cmu_peric1_usi06", CLK_CON_DIV_CLKCMU_PERIC1_USI06, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_USI07, "dout_cmu_peric1_usi07",
	    "gout_cmu_peric1_usi07", CLK_CON_DIV_CLKCMU_PERIC1_USI07, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_USI08, "dout_cmu_peric1_usi08",
	    "gout_cmu_peric1_usi08", CLK_CON_DIV_CLKCMU_PERIC1_USI08, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_USI09, "dout_cmu_peric1_usi09",
	    "gout_cmu_peric1_usi09", CLK_CON_DIV_CLKCMU_PERIC1_USI09, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_USI10, "dout_cmu_peric1_usi10",
	    "gout_cmu_peric1_usi10", CLK_CON_DIV_CLKCMU_PERIC1_USI10, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_USI11, "dout_cmu_peric1_usi11",
	    "gout_cmu_peric1_usi11", CLK_CON_DIV_CLKCMU_PERIC1_USI11, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_USI12, "dout_cmu_peric1_usi12",
	    "gout_cmu_peric1_usi12", CLK_CON_DIV_CLKCMU_PERIC1_USI12, 0, 4),
	DIV(CLK_DOUT_CMU_PERIC1_USI13, "dout_cmu_peric1_usi13",
	    "gout_cmu_peric1_usi13", CLK_CON_DIV_CLKCMU_PERIC1_USI13, 0, 4),
	DIV(CLK_DOUT_CMU_PERIS_BUS, "dout_cmu_peris_bus", "gout_cmu_peris_bus",
	    CLK_CON_DIV_CLKCMU_PERIS_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_SRDZ_BUS, "dout_cmu_srdz_bus", "gout_cmu_srdz_bus",
	    CLK_CON_DIV_CLKCMU_SRDZ_BUS, 0, 4),
	DIV(CLK_DOUT_CMU_SRDZ_IMGD, "dout_cmu_srdz_imgd", "gout_cmu_srdz_imgd",
	    CLK_CON_DIV_CLKCMU_SRDZ_IMGD, 0, 4),
	DIV(CLK_DOUT_CMU_VPU_BUS, "dout_cmu_vpu_bus", "gout_cmu_vpu_bus",
	    CLK_CON_DIV_CLKCMU_VPU_BUS, 0, 4),
];

static top_fixed_factor_clks: static const struct samsung_fixed_factor_clock top_fixed_factor_clks[]  = {[samsung_fixed_factor_clock] = &[
	FFACTOR(CLK_DOUT_CMU_SHARED0_DIV2, "dout_cmu_shared0_div2",
		"mout_pll_shared0", 1, 2, 0),
	FFACTOR(CLK_DOUT_CMU_SHARED0_DIV4, "dout_cmu_shared0_div4",
		"mout_pll_shared0", 1, 4, 0),
	FFACTOR(CLK_DOUT_CMU_SHARED1_DIV2, "dout_cmu_shared1_div2",
		"mout_pll_shared1", 1, 2, 0),
	FFACTOR(CLK_DOUT_CMU_SHARED1_DIV4, "dout_cmu_shared1_div4",
		"mout_pll_shared1", 1, 4, 0),
	FFACTOR(CLK_DOUT_CMU_SHARED2_DIV2, "dout_cmu_shared2_div2",
		"mout_pll_shared2", 1, 2, 0),
	FFACTOR(CLK_DOUT_CMU_SHARED3_DIV2, "dout_cmu_shared3_div2",
		"mout_pll_shared3", 1, 2, 0),
	FFACTOR(CLK_DOUT_CMU_SHARED4_DIV2, "dout_cmu_shared4_div2",
		"mout_pll_shared4", 1, 2, 0),
	FFACTOR(CLK_DOUT_CMU_FSYS1_PCIE, "dout_cmu_fsys1_pcie",
		"gout_cmu_fsys1_pcie", 1, 8, 0),
	FFACTOR(CLK_DOUT_CMU_CP2AP_MIF_CLK_DIV2, "dout_cmu_cp2ap_mif_clk_div2",
		"mout_cp2ap_mif_clk_user", 1, 2, 0),
	FFACTOR(CLK_DOUT_CMU_CMU_OTP, "dout_cmu_cmu_otp", "oscclk", 1, 8, 0),
];

static top_gate_clks: static const struct samsung_gate_clock top_gate_clks[]  = {[samsung_gate_clock] = &[
	GATE(CLK_GOUT_CMU_DROOPDETECTOR, "gout_droopdetector",
	     "mout_cmu_droopdetector", CLK_CON_GAT_CLKCMU_DROOPDETECTOR,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_MIF_SWITCH, "gout_cmu_mif_switch",
	     "mout_cmu_mif_switch", CLK_CON_GAT_CLKCMU_MIF_SWITCH, 21, 0, 0),
	GATE(CLK_GOUT_CMU_ABOX_CPUABOX, "gout_cmu_abox_cpuabox",
	     "mout_cmu_abox_cpuabox", CLK_CON_GAT_GATE_CLKCMU_ABOX_CPUABOX,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_APM_BUS, "gout_cmu_apm_bus", "mout_cmu_apm_bus",
	     CLK_CON_GAT_GATE_CLKCMU_APM_BUS, 21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_CMU_BUS1_BUS, "gout_cmu_bus1_bus", "mout_cmu_bus1_bus",
	     CLK_CON_GAT_GATE_CLKCMU_BUS1_BUS, 21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_CMU_BUSC_BUS, "gout_cmu_busc_bus", "mout_cmu_busc_bus",
	     CLK_CON_GAT_GATE_CLKCMU_BUSC_BUS, 21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_CMU_BUSC_BUSPHSI2C, "gout_cmu_busc_busphsi2c",
	     "mout_cmu_busc_busphsi2c", CLK_CON_GAT_GATE_CLKCMU_BUSC_BUSPHSI2C,
	     21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_CMU_CAM_BUS, "gout_cmu_cam_bus", "mout_cmu_cam_bus",
	     CLK_CON_GAT_GATE_CLKCMU_CAM_BUS, 21, 0, 0),
	GATE(CLK_GOUT_CMU_CAM_TPU0, "gout_cmu_cam_tpu0", "mout_cmu_cam_tpu0",
	     CLK_CON_GAT_GATE_CLKCMU_CAM_TPU0, 21, 0, 0),
	GATE(CLK_GOUT_CMU_CAM_TPU1, "gout_cmu_cam_tpu1", "mout_cmu_cam_tpu1",
	     CLK_CON_GAT_GATE_CLKCMU_CAM_TPU1, 21, 0, 0),
	GATE(CLK_GOUT_CMU_CAM_VRA, "gout_cmu_cam_vra", "mout_cmu_cam_vra",
	     CLK_CON_GAT_GATE_CLKCMU_CAM_VRA, 21, 0, 0),
	GATE(CLK_GOUT_CMU_CIS_CLK0, "gout_cmu_cis_clk0", "mout_cmu_cis_clk0",
	     CLK_CON_GAT_GATE_CLKCMU_CIS_CLK0, 21, 0, 0),
	GATE(CLK_GOUT_CMU_CIS_CLK1, "gout_cmu_cis_clk1", "mout_cmu_cis_clk1",
	     CLK_CON_GAT_GATE_CLKCMU_CIS_CLK1, 21, 0, 0),
	GATE(CLK_GOUT_CMU_CIS_CLK2, "gout_cmu_cis_clk2", "mout_cmu_cis_clk2",
	     CLK_CON_GAT_GATE_CLKCMU_CIS_CLK2, 21, 0, 0),
	GATE(CLK_GOUT_CMU_CIS_CLK3, "gout_cmu_cis_clk3", "mout_cmu_cis_clk3",
	     CLK_CON_GAT_GATE_CLKCMU_CIS_CLK3, 21, 0, 0),
	GATE(CLK_GOUT_CMU_CORE_BUS, "gout_cmu_core_bus", "mout_core_bus",
	     CLK_CON_GAT_GATE_CLKCMU_CORE_BUS, 21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_CMU_CPUCL0_SWITCH, "gout_cmu_cpucl0_switch",
	     "mout_cmu_cpucl0_switch", CLK_CON_GAT_GATE_CLKCMU_CPUCL0_SWITCH,
	     21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_CMU_CPUCL1_SWITCH, "gout_cmu_cpucl1_switch",
	     "mout_cmu_cpucl1_switch", CLK_CON_GAT_GATE_CLKCMU_CPUCL1_SWITCH,
	     21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_CMU_DBG_BUS, "gout_cmu_dbg_bus", "mout_cmu_dbg_bus",
	     CLK_CON_GAT_GATE_CLKCMU_DBG_BUS, 21, 0, 0),
	GATE(CLK_GOUT_CMU_DCAM_BUS, "gout_cmu_dcam_bus", "mout_cmu_dcam_bus",
	     CLK_CON_GAT_GATE_CLKCMU_DCAM_BUS, 21, 0, 0),
	GATE(CLK_GOUT_CMU_DCAM_IMGD, "gout_cmu_dcam_imgd",
	     "mout_cmu_dcam_imgd", CLK_CON_GAT_GATE_CLKCMU_DCAM_IMGD,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_DPU_BUS, "gout_cmu_dpu_bus", "mout_cmu_dpu_bus",
	     CLK_CON_GAT_GATE_CLKCMU_DPU_BUS, 21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_CMU_DSP_BUS, "gout_cmu_dsp_bus", "mout_cmu_dsp_bus",
	     CLK_CON_GAT_GATE_CLKCMU_DSP_BUS, 21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_CMU_FSYS0_BUS, "gout_cmu_fsys0_bus", "mout_fsys0_bus",
	     CLK_CON_GAT_GATE_CLKCMU_FSYS0_BUS, 21, 0, 0),
	GATE(CLK_GOUT_CMU_FSYS0_DPGTC, "gout_cmu_fsys0_dpgtc",
	     "mout_fsys0_dpgtc", CLK_CON_GAT_GATE_CLKCMU_FSYS0_DPGTC,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_FSYS0_MMC_EMBD, "gout_cmu_fsys0_mmc_embd",
	     "mout_fsys0_mmc_embd", CLK_CON_GAT_GATE_CLKCMU_FSYS0_MMC_EMBD,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_FSYS0_UFS_EMBD, "gout_cmu_fsys0_ufs_embd",
	     "mout_fsys0_ufs_embd", CLK_CON_GAT_GATE_CLKCMU_FSYS0_UFS_EMBD,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_FSYS0_USBDRD30, "gout_cmu_fsys0_usbdrd30",
	     "mout_fsys0_usbdrd30", CLK_CON_GAT_GATE_CLKCMU_FSYS0_USBDRD30,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_FSYS1_BUS, "gout_cmu_fsys1_bus",
	     "mout_cmu_fsys1_bus", CLK_CON_GAT_GATE_CLKCMU_FSYS1_BUS,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_FSYS1_MMC_CARD, "gout_cmu_fsys1_mmc_card",
	     "mout_cmu_fsys1_mmc_card", CLK_CON_GAT_GATE_CLKCMU_FSYS1_MMC_CARD,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_FSYS1_PCIE, "gout_cmu_fsys1_pcie",
	     "mout_cmu_fsys1_pcie", CLK_CON_GAT_GATE_CLKCMU_FSYS1_PCIE,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_FSYS1_UFS_CARD, "gout_cmu_fsys1_ufs_card",
	     "mout_cmu_fsys1_ufs_card", CLK_CON_GAT_GATE_CLKCMU_FSYS1_UFS_CARD,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_G2D_G2D, "gout_cmu_g2d_g2d", "mout_cmu_g2d_g2d",
	     CLK_CON_GAT_GATE_CLKCMU_G2D_G2D, 21, 0, 0),
	GATE(CLK_GOUT_CMU_G2D_JPEG, "gout_cmu_g2d_jpeg", "mout_cmu_g2d_jpeg",
	     CLK_CON_GAT_GATE_CLKCMU_G2D_JPEG, 21, 0, 0),
	GATE(CLK_GOUT_CMU_G3D_SWITCH, "gout_cmu_g3d_switch",
	     "fout_shared2_pll", CLK_CON_GAT_GATE_CLKCMU_G3D_SWITCH, 21, 0, 0),
	GATE(CLK_GOUT_CMU_HPM, "gout_cmu_hpm", "mout_cmu_hpm",
	     CLK_CON_GAT_GATE_CLKCMU_HPM, 21, 0, 0),
	GATE(CLK_GOUT_CMU_IMEM_BUS, "gout_cmu_imem_bus", "mout_cmu_imem_bus",
	     CLK_CON_GAT_GATE_CLKCMU_IMEM_BUS, 21, 0, 0),
	GATE(CLK_GOUT_CMU_ISPHQ_BUS, "gout_cmu_isphq_bus",
	     "mout_cmu_isphq_bus", CLK_CON_GAT_GATE_CLKCMU_ISPHQ_BUS,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_ISPLP_BUS, "gout_cmu_isplp_bus",
	     "mout_cmu_isplp_bus", CLK_CON_GAT_GATE_CLKCMU_ISPLP_BUS,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_IVA_BUS, "gout_cmu_iva_bus", "mout_cmu_iva_bus",
	     CLK_CON_GAT_GATE_CLKCMU_IVA_BUS, 21, 0, 0),
	GATE(CLK_GOUT_CMU_MFC_BUS, "gout_cmu_mfc_bus", "mout_cmu_mfc_bus",
	     CLK_CON_GAT_GATE_CLKCMU_MFC_BUS, 21, 0, 0),
	GATE(CLK_GOUT_CMU_MODEM_SHARED0, "gout_cmu_modem_shared0",
	     "dout_cmu_shared0_div2", CLK_CON_GAT_GATE_CLKCMU_MODEM_SHARED0,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_MODEM_SHARED1, "gout_cmu_modem_shared1",
	     "fout_shared2_pll", CLK_CON_GAT_GATE_CLKCMU_MODEM_SHARED1,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC0_BUS, "gout_cmu_peric0_bus",
	     "mout_cmu_peric0_bus", CLK_CON_GAT_GATE_CLKCMU_PERIC0_BUS,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC0_UART_DBG, "gout_cmu_peric0_uart_dbg",
	     "mout_cmu_peric0_uart_dbg",
	     CLK_CON_GAT_GATE_CLKCMU_PERIC0_UART_DBG, 21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC0_USI00, "gout_cmu_peric0_usi00",
	     "mout_cmu_peric0_usi00", CLK_CON_GAT_GATE_CLKCMU_PERIC0_USI00,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC0_USI01, "gout_cmu_peric0_usi01",
	     "mout_cmu_peric0_usi01", CLK_CON_GAT_GATE_CLKCMU_PERIC0_USI01,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC0_USI02, "gout_cmu_peric0_usi02",
	     "mout_cmu_peric0_usi02", CLK_CON_GAT_GATE_CLKCMU_PERIC0_USI02,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC0_USI03, "gout_cmu_peric0_usi03",
	     "mout_cmu_peric0_usi03", CLK_CON_GAT_GATE_CLKCMU_PERIC0_USI03,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_BUS, "gout_cmu_peric1_bus",
	     "mout_cmu_peric1_bus", CLK_CON_GAT_GATE_CLKCMU_PERIC1_BUS,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_SPEEDY2, "gout_cmu_peric1_speedy2",
	     "mout_cmu_peric1_speedy2", CLK_CON_GAT_GATE_CLKCMU_PERIC1_SPEEDY2,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_SPI_CAM0, "gout_cmu_peric1_spi_cam0",
	     "mout_cmu_peric1_spi_cam0",
	     CLK_CON_GAT_GATE_CLKCMU_PERIC1_SPI_CAM0, 21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_SPI_CAM1, "gout_cmu_peric1_spi_cam1",
	     "mout_cmu_peric1_spi_cam1",
	     CLK_CON_GAT_GATE_CLKCMU_PERIC1_SPI_CAM1, 21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_UART_BT, "gout_cmu_peric1_uart_bt",
	     "mout_cmu_peric1_uart_bt", CLK_CON_GAT_GATE_CLKCMU_PERIC1_UART_BT,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_USI04, "gout_cmu_peric1_usi04",
	     "mout_cmu_peric1_usi04", CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI04,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_USI05, "gout_cmu_peric1_usi05",
	     "mout_cmu_peric1_usi05", CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI05,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_USI06, "gout_cmu_peric1_usi06",
	     "mout_cmu_peric1_usi06", CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI06,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_USI07, "gout_cmu_peric1_usi07",
	     "mout_cmu_peric1_usi07", CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI07,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_USI08, "gout_cmu_peric1_usi08",
	     "mout_cmu_peric1_usi08", CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI08,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_USI09, "gout_cmu_peric1_usi09",
	     "mout_cmu_peric1_usi09", CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI09,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_USI10, "gout_cmu_peric1_usi10",
	     "mout_cmu_peric1_usi10", CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI10,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_USI11, "gout_cmu_peric1_usi11",
	     "mout_cmu_peric1_usi11", CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI11,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_USI12, "gout_cmu_peric1_usi12",
	     "mout_cmu_peric1_usi12", CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI12,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIC1_USI13, "gout_cmu_peric1_usi13",
	     "mout_cmu_peric1_usi13", CLK_CON_GAT_GATE_CLKCMU_PERIC1_USI13,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_PERIS_BUS, "gout_cmu_peris_bus",
	     "mout_cmu_peris_bus", CLK_CON_GAT_GATE_CLKCMU_PERIS_BUS,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_SRDZ_BUS, "gout_cmu_srdz_bus", "mout_cmu_srdz_bus",
	     CLK_CON_GAT_GATE_CLKCMU_SRDZ_BUS, 21, 0, 0),
	GATE(CLK_GOUT_CMU_SRDZ_IMGD, "gout_cmu_srdz_imgd",
	     "mout_cmu_srdz_imgd", CLK_CON_GAT_GATE_CLKCMU_SRDZ_IMGD,
	     21, 0, 0),
	GATE(CLK_GOUT_CMU_VPU_BUS, "gout_cmu_vpu_bus", "mout_cmu_vpu_bus",
	     CLK_CON_GAT_GATE_CLKCMU_VPU_BUS, 21, 0, 0),
];

static top_cmu_info: samsung_cmu_info = samsung_cmu_info {
.pll_clks = top_pll_clks,
.nr_pll_clks = top_pll_clks.len(),
.mux_clks = top_mux_clks,
.nr_mux_clks = top_mux_clks.len(),
.div_clks = top_div_clks,
.nr_div_clks = top_div_clks.len(),
.fixed_factor_clks = top_fixed_factor_clks,
.nr_fixed_factor_clks = top_fixed_factor_clks.len(),
.gate_clks = top_gate_clks,
.nr_gate_clks = top_gate_clks.len(),
.nr_clk_ids = CLKS_NR_TOP,
.clk_regs = top_clk_regs,
.nr_clk_regs = top_clk_regs.len(),
];

static void __init exynos8895_cmu_top_init(struct device_node *np)
{
	exynos_arm64_register_cmu(NULL, np, &top_cmu_info);
}

/* Register CMU_TOP early, as it's a dependency for other early domains */
CLK_OF_DECLARE(exynos8895_cmu_top, "samsung,exynos8895-cmu-top",
	       exynos8895_cmu_top_init);

/* ---- CMU_PERIS ----------------------------------------------------------- */

/* Register Offset definitions for CMU_PERIS (0x10010000) */
const PLL_CON0_MUX_CLKCMU_PERIS_BUS_USER: usize = 0x0100;
const PLL_CON2_MUX_CLKCMU_PERIS_BUS_USER: usize = 0x0108;
const CLK_CON_MUX_MUX_CLK_PERIS_GIC: usize = 0x1000;
const CLK_CON_GAT_CLK_BLK_PERIS_UID_PERIS_CMU_PERIS_IPCLKPORT_PCLK: usize = 0x2000;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_AD_AXI_P_PERIS_IPCLKPORT_ACLKM: usize = 0x2010;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_AD_AXI_P_PERIS_IPCLKPORT_ACLKS: usize = 0x2014;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_AXI2APB_PERISP0_IPCLKPORT_ACLK: usize = 0x2018;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_AXI2APB_PERISP1_IPCLKPORT_ACLK: usize = 0x201c;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_BUSIF_TMU_IPCLKPORT_PCLK: usize = 0x2020;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_GIC_IPCLKPORT_CLK: usize = 0x2024;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_LHM_AXI_P_PERIS_IPCLKPORT_I_CLK: usize = 0x2028;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_MCT_IPCLKPORT_PCLK: usize = 0x202c;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_OTP_CON_BIRA_IPCLKPORT_PCLK: usize = 0x2030;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_OTP_CON_TOP_IPCLKPORT_PCLK: usize = 0x2034;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_PMU_PERIS_IPCLKPORT_PCLK: usize = 0x2038;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_RSTNSYNC_CLK_PERIS_BUSP_IPCLKPORT_CLK: usize = 0x203c;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_RSTNSYNC_CLK_PERIS_GIC_IPCLKPORT_CLK: usize = 0x2040;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_SYSREG_PERIS_IPCLKPORT_PCLK: usize = 0x2044;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC00_IPCLKPORT_PCLK: usize = 0x2048;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC01_IPCLKPORT_PCLK: usize = 0x204c;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC02_IPCLKPORT_PCLK: usize = 0x2050;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC03_IPCLKPORT_PCLK: usize = 0x2054;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC04_IPCLKPORT_PCLK: usize = 0x2058;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC05_IPCLKPORT_PCLK: usize = 0x205c;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC06_IPCLKPORT_PCLK: usize = 0x2060;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC07_IPCLKPORT_PCLK: usize = 0x2064;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC08_IPCLKPORT_PCLK: usize = 0x2068;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC09_IPCLKPORT_PCLK: usize = 0x206c;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC10_IPCLKPORT_PCLK: usize = 0x2070;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC11_IPCLKPORT_PCLK: usize = 0x2074;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC12_IPCLKPORT_PCLK: usize = 0x2078;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC13_IPCLKPORT_PCLK: usize = 0x207c;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC14_IPCLKPORT_PCLK: usize = 0x2080;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC15_IPCLKPORT_PCLK: usize = 0x2084;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_WDT_CLUSTER0_IPCLKPORT_PCLK: usize = 0x2088;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_WDT_CLUSTER1_IPCLKPORT_PCLK: usize = 0x208c;
const CLK_CON_GAT_GOUT_BLK_PERIS_UID_XIU_P_PERIS_IPCLKPORT_ACLK: usize = 0x2090;

static peris_clk_regs: static const unsigned long peris_clk_regs[]  = {[u64] = &[
	PLL_CON0_MUX_CLKCMU_PERIS_BUS_USER,
	PLL_CON2_MUX_CLKCMU_PERIS_BUS_USER,
	CLK_CON_MUX_MUX_CLK_PERIS_GIC,
	CLK_CON_GAT_CLK_BLK_PERIS_UID_PERIS_CMU_PERIS_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_AD_AXI_P_PERIS_IPCLKPORT_ACLKM,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_AD_AXI_P_PERIS_IPCLKPORT_ACLKS,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_AXI2APB_PERISP0_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_AXI2APB_PERISP1_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_BUSIF_TMU_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_GIC_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_LHM_AXI_P_PERIS_IPCLKPORT_I_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_MCT_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_OTP_CON_BIRA_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_OTP_CON_TOP_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_PMU_PERIS_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_RSTNSYNC_CLK_PERIS_BUSP_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_RSTNSYNC_CLK_PERIS_GIC_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_SYSREG_PERIS_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC00_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC01_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC02_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC03_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC04_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC05_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC06_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC07_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC08_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC09_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC10_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC11_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC12_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC13_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC14_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC15_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_WDT_CLUSTER0_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_WDT_CLUSTER1_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIS_UID_XIU_P_PERIS_IPCLKPORT_ACLK,
];

/* List of parent clocks for Muxes in CMU_PERIS */
static mout_peris_bus_user_p: PNAME(mout_peris_bus_user_p)	= {[PNAME(mout_peris_bus_user_p)	= {str] = &[ "oscclk", "dout_cmu_peris_bus" ];
static mout_peris_gic_p: PNAME(mout_peris_gic_p)		= {[PNAME(mout_peris_gic_p)		= {str] = &[ "oscclk", "mout_peris_bus_user" ];

static peris_mux_clks: static const struct samsung_mux_clock peris_mux_clks[]  = {[samsung_mux_clock] = &[
	MUX(CLK_MOUT_PERIS_BUS_USER, "mout_peris_bus_user",
	    mout_peris_bus_user_p, PLL_CON0_MUX_CLKCMU_PERIS_BUS_USER, 4, 1),
	MUX(CLK_MOUT_PERIS_GIC, "mout_peris_gic",
	    mout_peris_gic_p, CLK_CON_MUX_MUX_CLK_PERIS_GIC, 0, 5),
];

static peris_gate_clks: static const struct samsung_gate_clock peris_gate_clks[]  = {[samsung_gate_clock] = &[
	GATE(CLK_GOUT_PERIS_CMU_PERIS_PCLK, "gout_peris_cmu_peris_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_CLK_BLK_PERIS_UID_PERIS_CMU_PERIS_IPCLKPORT_PCLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIS_AD_AXI_P_PERIS_ACLKM,
	     "gout_peris_ad_axi_p_peris_aclkm", "mout_peris_gic",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_AD_AXI_P_PERIS_IPCLKPORT_ACLKM,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIS_AD_AXI_P_PERIS_ACLKS,
	     "gout_peris_ad_axi_p_peris_aclks", "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_AD_AXI_P_PERIS_IPCLKPORT_ACLKS,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIS_AXI2APB_PERISP0_ACLK,
	     "gout_peris_axi2apb_perisp0_aclk", "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_AXI2APB_PERISP0_IPCLKPORT_ACLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIS_AXI2APB_PERISP1_ACLK,
	     "gout_peris_axi2apb_perisp1_aclk", "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_AXI2APB_PERISP1_IPCLKPORT_ACLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIS_BUSIF_TMU_PCLK, "gout_peris_busif_tmu_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_BUSIF_TMU_IPCLKPORT_PCLK,
	     21, 0, 0),
	/* GIC (interrupt controller) clock must be always running */
	GATE(CLK_GOUT_PERIS_GIC_CLK, "gout_peris_gic_clk",
	     "mout_peris_gic",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_GIC_IPCLKPORT_CLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIS_LHM_AXI_P_PERIS_I_CLK,
	     "gout_peris_lhm_axi_p_peris_i_clk", "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_LHM_AXI_P_PERIS_IPCLKPORT_I_CLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIS_MCT_PCLK, "gout_peris_mct_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_MCT_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_OTP_CON_BIRA_PCLK, "gout_peris_otp_con_bira_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_OTP_CON_BIRA_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIS_OTP_CON_TOP_PCLK, "gout_peris_otp_con_top_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_OTP_CON_TOP_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIS_PMU_PERIS_PCLK, "gout_peris_pmu_peris_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_PMU_PERIS_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIS_RSTNSYNC_CLK_PERIS_BUSP_CLK,
	     "gout_peris_rstnsync_clk_peris_busp_clk", "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_RSTNSYNC_CLK_PERIS_BUSP_IPCLKPORT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIS_RSTNSYNC_CLK_PERIS_GIC_CLK,
	     "gout_peris_rstnsync_clk_peris_gic_clk", "mout_peris_gic",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_RSTNSYNC_CLK_PERIS_GIC_IPCLKPORT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIS_SYSREG_PERIS_PCLK, "gout_peris_sysreg_peris_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_SYSREG_PERIS_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC00_PCLK, "gout_peris_tzpc00_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC00_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC01_PCLK, "gout_peris_tzpc01_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC01_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC02_PCLK, "gout_peris_tzpc02_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC02_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC03_PCLK, "gout_peris_tzpc03_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC03_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC04_PCLK, "gout_peris_tzpc04_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC04_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC05_PCLK, "gout_peris_tzpc05_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC05_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC06_PCLK, "gout_peris_tzpc06_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC06_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC07_PCLK, "gout_peris_tzpc07_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC07_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC08_PCLK, "gout_peris_tzpc08_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC08_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC09_PCLK, "gout_peris_tzpc09_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC09_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC10_PCLK, "gout_peris_tzpc10_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC10_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC11_PCLK, "gout_peris_tzpc11_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC11_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC12_PCLK, "gout_peris_tzpc12_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC12_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC13_PCLK, "gout_peris_tzpc13_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC13_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC14_PCLK, "gout_peris_tzpc14_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC14_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_TZPC15_PCLK, "gout_peris_tzpc15_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_TZPC15_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIS_WDT_CLUSTER0_PCLK, "gout_peris_wdt_cluster0_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_WDT_CLUSTER0_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIS_WDT_CLUSTER1_PCLK, "gout_peris_wdt_cluster1_pclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_WDT_CLUSTER1_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIS_XIU_P_PERIS_ACLK, "gout_peris_xiu_p_peris_aclk",
	     "mout_peris_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIS_UID_XIU_P_PERIS_IPCLKPORT_ACLK,
	     21, CLK_IGNORE_UNUSED, 0),
];

static peris_cmu_info: samsung_cmu_info = samsung_cmu_info {
.mux_clks = peris_mux_clks,
.nr_mux_clks = peris_mux_clks.len(),
.gate_clks = peris_gate_clks,
.nr_gate_clks = peris_gate_clks.len(),
.nr_clk_ids = CLKS_NR_PERIS,
.clk_regs = peris_clk_regs,
.nr_clk_regs = peris_clk_regs.len(),
.clk_name = "bus",
];

static void __init exynos8895_cmu_peris_init(struct device_node *np)
{
	exynos_arm64_register_cmu(NULL, np, &peris_cmu_info);
}

/* Register CMU_PERIS early, as it's needed for MCT timer */
CLK_OF_DECLARE(exynos8895_cmu_peris, "samsung,exynos8895-cmu-peris",
	       exynos8895_cmu_peris_init);

/* ---- CMU_FSYS0 ---------------------------------------------------------- */

/* Register Offset definitions for CMU_FSYS0 (0x11000000) */
const PLL_CON0_MUX_CLKCMU_FSYS0_BUS_USER: usize = 0x0100;
const PLL_CON2_MUX_CLKCMU_FSYS0_BUS_USER: usize = 0x0108;
const PLL_CON0_MUX_CLKCMU_FSYS0_DPGTC_USER: usize = 0x0120;
const PLL_CON2_MUX_CLKCMU_FSYS0_DPGTC_USER: usize = 0x0128;
const PLL_CON0_MUX_CLKCMU_FSYS0_MMC_EMBD_USER: usize = 0x0140;
const PLL_CON2_MUX_CLKCMU_FSYS0_MMC_EMBD_USER: usize = 0x0148;
const PLL_CON0_MUX_CLKCMU_FSYS0_UFS_EMBD_USER: usize = 0x0160;
const PLL_CON2_MUX_CLKCMU_FSYS0_UFS_EMBD_USER: usize = 0x0168;
const PLL_CON0_MUX_CLKCMU_FSYS0_USBDRD30_USER: usize = 0x0180;
const PLL_CON2_MUX_CLKCMU_FSYS0_USBDRD30_USER: usize = 0x0188;
const CLK_CON_GAT_CLK_BLK_FSYS0_UID_FSYS0_CMU_FSYS0_IPCLKPORT_PCLK: usize = 0x2000;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_AHBBR_FSYS0_IPCLKPORT_HCLK: usize = 0x2010;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_AXI2AHB_FSYS0_IPCLKPORT_ACLK: usize = 0x2014;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_AXI2AHB_USB_FSYS0_IPCLKPORT_ACLK: usize = 0x2018;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_AXI2APB_FSYS0_IPCLKPORT_ACLK: usize = 0x201c;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_BTM_FSYS0_IPCLKPORT_I_ACLK: usize = 0x2020;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_BTM_FSYS0_IPCLKPORT_I_PCLK: usize = 0x2024;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_DP_LINK_IPCLKPORT_I_GTC_EXT_CLK: usize = 0x202c;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_DP_LINK_IPCLKPORT_I_PCLK: usize = 0x2030;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_ETR_MIU_IPCLKPORT_I_ACLK: usize = 0x2034;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_ETR_MIU_IPCLKPORT_I_PCLK: usize = 0x2038;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_GPIO_FSYS0_IPCLKPORT_PCLK: usize = 0x203c;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_LHM_AXI_D_USBTV_IPCLKPORT_I_CLK: usize = 0x2040;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_LHM_AXI_G_ETR_IPCLKPORT_I_CLK: usize = 0x2044;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_LHM_AXI_P_FSYS0_IPCLKPORT_I_CLK: usize = 0x2048;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_LHS_ACEL_D_FSYS0_IPCLKPORT_I_CLK: usize = 0x204c;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_MMC_EMBD_IPCLKPORT_I_ACLK: usize = 0x2050;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_MMC_EMBD_IPCLKPORT_SDCLKIN: usize = 0x2054;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_PMU_FSYS0_IPCLKPORT_PCLK: usize = 0x2058;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_BCM_FSYS0_IPCLKPORT_ACLK: usize = 0x205c;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_BCM_FSYS0_IPCLKPORT_PCLK: usize = 0x2060;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_RSTNSYNC_CLK_FSYS0_BUS_IPCLKPORT_CLK: usize = 0x2064;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_SYSREG_FSYS0_IPCLKPORT_PCLK: usize = 0x2068;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_UFS_EMBD_IPCLKPORT_I_ACLK: usize = 0x206c;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_UFS_EMBD_IPCLKPORT_I_CLK_UNIPRO: usize = 0x2070;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_UFS_EMBD_IPCLKPORT_I_FMP_CLK: usize = 0x2074;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USB30DRD_ACLK: usize = 0x2078;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USB30DRD_REF_CLK: usize = 0x207c;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USB30DRD_SUSPEND_CLK: usize = 0x2080;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USBTVH_AHB_CLK: usize = 0x2084;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USBTVH_CORE_CLK: usize = 0x2088;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USBTVH_XIU_CLK: usize = 0x208c;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_US_D_FSYS0_USB_IPCLKPORT_ACLK: usize = 0x2090;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_XIU_D_FSYS0_IPCLKPORT_ACLK: usize = 0x2094;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_XIU_D_FSYS0_USB_IPCLKPORT_ACLK: usize = 0x2098;
const CLK_CON_GAT_GOUT_BLK_FSYS0_UID_XIU_P_FSYS0_IPCLKPORT_ACLK: usize = 0x209c;

static fsys0_clk_regs: static const unsigned long fsys0_clk_regs[]  = {[u64] = &[
	PLL_CON0_MUX_CLKCMU_FSYS0_BUS_USER,
	PLL_CON2_MUX_CLKCMU_FSYS0_BUS_USER,
	PLL_CON0_MUX_CLKCMU_FSYS0_DPGTC_USER,
	PLL_CON2_MUX_CLKCMU_FSYS0_DPGTC_USER,
	PLL_CON0_MUX_CLKCMU_FSYS0_MMC_EMBD_USER,
	PLL_CON2_MUX_CLKCMU_FSYS0_MMC_EMBD_USER,
	PLL_CON0_MUX_CLKCMU_FSYS0_UFS_EMBD_USER,
	PLL_CON2_MUX_CLKCMU_FSYS0_UFS_EMBD_USER,
	PLL_CON0_MUX_CLKCMU_FSYS0_USBDRD30_USER,
	PLL_CON2_MUX_CLKCMU_FSYS0_USBDRD30_USER,
	CLK_CON_GAT_CLK_BLK_FSYS0_UID_FSYS0_CMU_FSYS0_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_AHBBR_FSYS0_IPCLKPORT_HCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_AXI2AHB_FSYS0_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_AXI2AHB_USB_FSYS0_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_AXI2APB_FSYS0_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_BTM_FSYS0_IPCLKPORT_I_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_BTM_FSYS0_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_DP_LINK_IPCLKPORT_I_GTC_EXT_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_DP_LINK_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_ETR_MIU_IPCLKPORT_I_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_ETR_MIU_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_GPIO_FSYS0_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_LHM_AXI_D_USBTV_IPCLKPORT_I_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_LHM_AXI_G_ETR_IPCLKPORT_I_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_LHM_AXI_P_FSYS0_IPCLKPORT_I_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_LHS_ACEL_D_FSYS0_IPCLKPORT_I_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_MMC_EMBD_IPCLKPORT_I_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_MMC_EMBD_IPCLKPORT_SDCLKIN,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_PMU_FSYS0_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_BCM_FSYS0_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_BCM_FSYS0_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_RSTNSYNC_CLK_FSYS0_BUS_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_SYSREG_FSYS0_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_UFS_EMBD_IPCLKPORT_I_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_UFS_EMBD_IPCLKPORT_I_CLK_UNIPRO,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_UFS_EMBD_IPCLKPORT_I_FMP_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USB30DRD_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USB30DRD_REF_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USB30DRD_SUSPEND_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USBTVH_AHB_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USBTVH_CORE_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USBTVH_XIU_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_US_D_FSYS0_USB_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_XIU_D_FSYS0_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_XIU_D_FSYS0_USB_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS0_UID_XIU_P_FSYS0_IPCLKPORT_ACLK,
];

/* List of parent clocks for Muxes in CMU_FSYS0 */
static mout_fsys0_bus_user_p: PNAME(mout_fsys0_bus_user_p)		= {[PNAME(mout_fsys0_bus_user_p)		= {str] = &[ "oscclk", "dout_cmu_fsys0_bus" ];
static mout_fsys0_dpgtc_user_p: PNAME(mout_fsys0_dpgtc_user_p)		= {[PNAME(mout_fsys0_dpgtc_user_p)		= {str] = &[ "oscclk", "dout_cmu_fsys0_dpgtc" ];
static mout_fsys0_mmc_embd_user_p: PNAME(mout_fsys0_mmc_embd_user_p)	= {[PNAME(mout_fsys0_mmc_embd_user_p)	= {str] = &[ "oscclk",
					    "dout_cmu_fsys0_mmc_embd" ];
static mout_fsys0_ufs_embd_user_p: PNAME(mout_fsys0_ufs_embd_user_p)	= {[PNAME(mout_fsys0_ufs_embd_user_p)	= {str] = &[ "oscclk",
					    "dout_cmu_fsys0_ufs_embd" ];
static mout_fsys0_usbdrd30_user_p: PNAME(mout_fsys0_usbdrd30_user_p)	= {[PNAME(mout_fsys0_usbdrd30_user_p)	= {str] = &[ "oscclk",
					    "dout_cmu_fsys0_usbdrd30" ];

static fsys0_mux_clks: static const struct samsung_mux_clock fsys0_mux_clks[]  = {[samsung_mux_clock] = &[
	MUX(CLK_MOUT_FSYS0_BUS_USER, "mout_fsys0_bus_user",
	    mout_fsys0_bus_user_p, PLL_CON0_MUX_CLKCMU_FSYS0_BUS_USER, 4, 1),
	MUX(CLK_MOUT_FSYS0_DPGTC_USER, "mout_fsys0_dpgtc_user",
	    mout_fsys0_dpgtc_user_p, PLL_CON0_MUX_CLKCMU_FSYS0_DPGTC_USER,
	    4, 1),
	MUX_F(CLK_MOUT_FSYS0_MMC_EMBD_USER, "mout_fsys0_mmc_embd_user",
	      mout_fsys0_mmc_embd_user_p, PLL_CON0_MUX_CLKCMU_FSYS0_MMC_EMBD_USER,
	      4, 1, CLK_SET_RATE_PARENT, 0),
	MUX(CLK_MOUT_FSYS0_UFS_EMBD_USER, "mout_fsys0_ufs_embd_user",
	    mout_fsys0_ufs_embd_user_p, PLL_CON0_MUX_CLKCMU_FSYS0_UFS_EMBD_USER,
	    4, 1),
	MUX(CLK_MOUT_FSYS0_USBDRD30_USER, "mout_fsys0_usbdrd30_user",
	    mout_fsys0_usbdrd30_user_p, PLL_CON0_MUX_CLKCMU_FSYS0_USBDRD30_USER,
	    4, 1),
];

static fsys0_gate_clks: static const struct samsung_gate_clock fsys0_gate_clks[]  = {[samsung_gate_clock] = &[
	/* Disabling this clock makes the system hang. Mark the clock as critical. */
	GATE(CLK_GOUT_FSYS0_FSYS0_CMU_FSYS0_PCLK,
	     "gout_fsys0_fsys0_cmu_fsys0_pclk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_CLK_BLK_FSYS0_UID_FSYS0_CMU_FSYS0_IPCLKPORT_PCLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS0_AHBBR_FSYS0_HCLK, "gout_fsys0_ahbbr_fsys0_hclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_AHBBR_FSYS0_IPCLKPORT_HCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_AXI2AHB_FSYS0_ACLK,
	     "gout_fsys0_axi2ahb_fsys0_aclk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_AXI2AHB_FSYS0_IPCLKPORT_ACLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS0_AXI2AHB_USB_FSYS0_ACLK,
	     "gout_fsys0_axi2ahb_usb_fsys0_aclk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_AXI2AHB_USB_FSYS0_IPCLKPORT_ACLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS0_AXI2APB_FSYS0_ACLK,
	     "gout_fsys0_axi2apb_fsys0_aclk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_AXI2APB_FSYS0_IPCLKPORT_ACLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS0_BTM_FSYS0_I_ACLK, "gout_fsys0_btm_fsys0_i_aclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_BTM_FSYS0_IPCLKPORT_I_ACLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_BTM_FSYS0_I_PCLK, "gout_fsys0_btm_fsys0_i_pclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_BTM_FSYS0_IPCLKPORT_I_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_DP_LINK_I_GTC_EXT_CLK,
	     "gout_fsys0_dp_link_i_gtc_ext_clk", "mout_fsys0_dpgtc_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_DP_LINK_IPCLKPORT_I_GTC_EXT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_DP_LINK_I_PCLK, "gout_fsys0_dp_link_i_pclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_DP_LINK_IPCLKPORT_I_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_ETR_MIU_I_ACLK, "gout_fsys0_etr_miu_i_aclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_ETR_MIU_IPCLKPORT_I_ACLK, 21, 0, 0),
	GATE(CLK_GOUT_FSYS0_ETR_MIU_I_PCLK, "gout_fsys0_etr_miu_i_pclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_ETR_MIU_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_FSYS0_GPIO_FSYS0_PCLK, "gout_fsys0_gpio_fsys0_pclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_GPIO_FSYS0_IPCLKPORT_PCLK,
	     21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_FSYS0_LHM_AXI_D_USBTV_I_CLK,
	     "gout_fsys0_lhm_axi_d_usbtv_i_clk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_LHM_AXI_D_USBTV_IPCLKPORT_I_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_LHM_AXI_G_ETR_I_CLK,
	     "gout_fsys0_lhm_axi_g_etr_i_clk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_LHM_AXI_G_ETR_IPCLKPORT_I_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_LHM_AXI_P_FSYS0_I_CLK,
	     "gout_fsys0_lhm_axi_p_fsys0_i_clk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_LHM_AXI_P_FSYS0_IPCLKPORT_I_CLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS0_LHS_ACEL_D_FSYS0_I_CLK,
	     "gout_fsys0_lhs_acel_d_fsys0_i_clk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_LHS_ACEL_D_FSYS0_IPCLKPORT_I_CLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS0_MMC_EMBD_I_ACLK, "gout_fsys0_mmc_embd_i_aclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_MMC_EMBD_IPCLKPORT_I_ACLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_MMC_EMBD_SDCLKIN, "gout_fsys0_mmc_embd_sdclkin",
	     "mout_fsys0_mmc_embd_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_MMC_EMBD_IPCLKPORT_SDCLKIN,
	     21, CLK_SET_RATE_PARENT, 0),
	GATE(CLK_GOUT_FSYS0_PMU_FSYS0_PCLK, "gout_fsys0_pmu_fsys0_pclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_PMU_FSYS0_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_FSYS0_BCM_FSYS0_ACLK, "gout_fsys0_bcm_fsys0_aclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_BCM_FSYS0_IPCLKPORT_ACLK, 21, 0, 0),
	GATE(CLK_GOUT_FSYS0_BCM_FSYS0_PCLK, "gout_fsys0_bcm_fsys0_pclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_BCM_FSYS0_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_FSYS0_RSTNSYNC_CLK_FSYS0_BUS_CLK,
	     "gout_fsys0_rstnsync_clk_fsys0_bus_clk", "mout_fsys0_bus_user",
	      CLK_CON_GAT_GOUT_BLK_FSYS0_UID_RSTNSYNC_CLK_FSYS0_BUS_IPCLKPORT_CLK,
	      21, 0, 0),
	GATE(CLK_GOUT_FSYS0_SYSREG_FSYS0_PCLK, "gout_fsys0_sysreg_fsys0_pclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_SYSREG_FSYS0_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_UFS_EMBD_I_ACLK, "gout_fsys0_ufs_embd_i_aclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_UFS_EMBD_IPCLKPORT_I_ACLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_UFS_EMBD_I_CLK_UNIPRO,
	     "gout_fsys0_ufs_embd_i_clk_unipro", "mout_fsys0_ufs_embd_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_UFS_EMBD_IPCLKPORT_I_CLK_UNIPRO,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS0_UFS_EMBD_I_FMP_CLK,
	     "gout_fsys0_ufs_embd_i_fmp_clk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_UFS_EMBD_IPCLKPORT_I_FMP_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_USBTV_I_USB30DRD_ACLK,
	     "gout_fsys0_usbtv_i_usb30drd_aclk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USB30DRD_ACLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_USBTV_I_USB30DRD_REF_CLK,
	     "gout_fsys0_usbtv_i_usb30drd_ref_clk", "mout_fsys0_usbdrd30_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USB30DRD_REF_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_USBTV_I_USB30DRD_SUSPEND_CLK,
	     "gout_fsys0_usbtv_i_usb30drd_suspend_clk",
	     "mout_fsys0_usbdrd30_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USB30DRD_SUSPEND_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_USBTV_I_USBTVH_AHB_CLK,
	     "gout_fsys0_usbtv_i_usbtvh_ahb_clk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USBTVH_AHB_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_USBTV_I_USBTVH_CORE_CLK,
	     "gout_fsys0_usbtv_i_usbtvh_core_clk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USBTVH_CORE_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_USBTV_I_USBTVH_XIU_CLK,
	     "gout_fsys0_usbtv_i_usbtvh_xiu_clk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_USBTV_IPCLKPORT_I_USBTVH_XIU_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_US_D_FSYS0_USB_ACLK,
	     "gout_fsys0_us_d_fsys0_usb_aclk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_US_D_FSYS0_USB_IPCLKPORT_ACLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS0_XIU_D_FSYS0_ACLK, "gout_fsys0_xiu_d_fsys0_aclk",
	     "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_XIU_D_FSYS0_IPCLKPORT_ACLK,
	     21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_FSYS0_XIU_D_FSYS0_USB_ACLK,
	     "gout_fsys0_xiu_d_fsys0_usb_aclk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_XIU_D_FSYS0_USB_IPCLKPORT_ACLK,
	     21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_FSYS0_XIU_P_FSYS0_ACLK,
	     "gout_fsys0_xiu_p_fsys0_aclk", "mout_fsys0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS0_UID_XIU_P_FSYS0_IPCLKPORT_ACLK,
	     21, CLK_IGNORE_UNUSED, 0),
];

static fsys0_cmu_info: samsung_cmu_info = samsung_cmu_info {
.mux_clks = fsys0_mux_clks,
.nr_mux_clks = fsys0_mux_clks.len(),
.gate_clks = fsys0_gate_clks,
.nr_gate_clks = fsys0_gate_clks.len(),
.nr_clk_ids = CLKS_NR_FSYS0,
.clk_regs = fsys0_clk_regs,
.nr_clk_regs = fsys0_clk_regs.len(),
.clk_name = "bus",
];

/* ---- CMU_FSYS1 ---------------------------------------------------------- */

/* Register Offset definitions for CMU_FSYS1 (0x11400000) */
const PLL_CON0_MUX_CLKCMU_FSYS1_BUS_USER: usize = 0x0100;
const PLL_CON2_MUX_CLKCMU_FSYS1_BUS_USER: usize = 0x0108;
const PLL_CON0_MUX_CLKCMU_FSYS1_MMC_CARD_USER: usize = 0x0120;
const PLL_CON2_MUX_CLKCMU_FSYS1_MMC_CARD_USER: usize = 0x0128;
const PLL_CON0_MUX_CLKCMU_FSYS1_PCIE_USER: usize = 0x0140;
const PLL_CON2_MUX_CLKCMU_FSYS1_PCIE_USER: usize = 0x0148;
const PLL_CON0_MUX_CLKCMU_FSYS1_UFS_CARD_USER: usize = 0x0160;
const PLL_CON2_MUX_CLKCMU_FSYS1_UFS_CARD_USER: usize = 0x0168;
const CLK_CON_GAT_CLK_BLK_FSYS1_UID_PCIE_IPCLKPORT_PHY_REF_CLK_IN: usize = 0x2000;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_ADM_AHB_SSS_IPCLKPORT_HCLKM: usize = 0x2004;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_AHBBR_FSYS1_IPCLKPORT_HCLK: usize = 0x2008;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_AXI2AHB_FSYS1_IPCLKPORT_ACLK: usize = 0x200c;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_AXI2APB_FSYS1P0_IPCLKPORT_ACLK: usize = 0x2010;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_AXI2APB_FSYS1P1_IPCLKPORT_ACLK: usize = 0x2014;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_BTM_FSYS1_IPCLKPORT_I_ACLK: usize = 0x2018;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_BTM_FSYS1_IPCLKPORT_I_PCLK: usize = 0x201c;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_FSYS1_CMU_FSYS1_IPCLKPORT_PCLK: usize = 0x2024;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_GPIO_FSYS1_IPCLKPORT_PCLK: usize = 0x2028;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_LHM_AXI_P_FSYS1_IPCLKPORT_I_CLK: usize = 0x202c;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_LHS_ACEL_D_FSYS1_IPCLKPORT_I_CLK: usize = 0x2030;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_MMC_CARD_IPCLKPORT_I_ACLK: usize = 0x2034;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_MMC_CARD_IPCLKPORT_SDCLKIN: usize = 0x2038;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_DBI_ACLK_0: usize = 0x203c;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_DBI_ACLK_1: usize = 0x2040;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_IEEE1500_WRAPPER_FOR_PCIE_PHY_LC_X2_INST_0_I_SCL_APB_PCLK: usize = 0x2044;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_MSTR_ACLK_0: usize = 0x2048;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_MSTR_ACLK_1: usize = 0x204c;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_PCIE_SUB_CTRL_INST_0_I_DRIVER_APB_CLK: usize = 0x2050;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_PCIE_SUB_CTRL_INST_1_I_DRIVER_APB_CLK: usize = 0x2054;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_PIPE2_DIGITAL_X2_WRAP_INST_0_I_APB_PCLK_SCL: usize = 0x2058;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_SLV_ACLK_0: usize = 0x205c;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_SLV_ACLK_1: usize = 0x2060;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PMU_FSYS1_IPCLKPORT_PCLK: usize = 0x2068;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_BCM_FSYS1_IPCLKPORT_ACLK: usize = 0x206c;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_BCM_FSYS1_IPCLKPORT_PCLK: usize = 0x2070;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_RSTNSYNC_CLK_FSYS1_BUS_IPCLKPORT_CLK: usize = 0x2074;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_RTIC_IPCLKPORT_I_ACLK: usize = 0x207c;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_RTIC_IPCLKPORT_I_PCLK: usize = 0x2080;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_SSS_IPCLKPORT_I_ACLK: usize = 0x2084;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_SSS_IPCLKPORT_I_PCLK: usize = 0x2088;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_SYSREG_FSYS1_IPCLKPORT_PCLK: usize = 0x2090;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_TOE_WIFI0_IPCLKPORT_I_CLK: usize = 0x2094;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_TOE_WIFI1_IPCLKPORT_I_CLK: usize = 0x2098;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_UFS_CARD_IPCLKPORT_I_ACLK: usize = 0x209c;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_UFS_CARD_IPCLKPORT_I_CLK_UNIPRO: usize = 0x20a0;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_UFS_CARD_IPCLKPORT_I_FMP_CLK: usize = 0x20a4;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_XIU_D_FSYS1_IPCLKPORT_ACLK: usize = 0x20a8;
const CLK_CON_GAT_GOUT_BLK_FSYS1_UID_XIU_P_FSYS1_IPCLKPORT_ACLK: usize = 0x20ac;

static fsys1_clk_regs: static const unsigned long fsys1_clk_regs[]  = {[u64] = &[
	PLL_CON0_MUX_CLKCMU_FSYS1_BUS_USER,
	PLL_CON2_MUX_CLKCMU_FSYS1_BUS_USER,
	PLL_CON0_MUX_CLKCMU_FSYS1_MMC_CARD_USER,
	PLL_CON2_MUX_CLKCMU_FSYS1_MMC_CARD_USER,
	PLL_CON0_MUX_CLKCMU_FSYS1_PCIE_USER,
	PLL_CON2_MUX_CLKCMU_FSYS1_PCIE_USER,
	PLL_CON0_MUX_CLKCMU_FSYS1_UFS_CARD_USER,
	PLL_CON2_MUX_CLKCMU_FSYS1_UFS_CARD_USER,
	CLK_CON_GAT_CLK_BLK_FSYS1_UID_PCIE_IPCLKPORT_PHY_REF_CLK_IN,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_ADM_AHB_SSS_IPCLKPORT_HCLKM,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_AHBBR_FSYS1_IPCLKPORT_HCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_AXI2AHB_FSYS1_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_AXI2APB_FSYS1P0_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_AXI2APB_FSYS1P1_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_BTM_FSYS1_IPCLKPORT_I_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_BTM_FSYS1_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_FSYS1_CMU_FSYS1_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_GPIO_FSYS1_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_LHM_AXI_P_FSYS1_IPCLKPORT_I_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_LHS_ACEL_D_FSYS1_IPCLKPORT_I_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_MMC_CARD_IPCLKPORT_I_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_MMC_CARD_IPCLKPORT_SDCLKIN,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_DBI_ACLK_0,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_DBI_ACLK_1,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_IEEE1500_WRAPPER_FOR_PCIE_PHY_LC_X2_INST_0_I_SCL_APB_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_MSTR_ACLK_0,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_MSTR_ACLK_1,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_PCIE_SUB_CTRL_INST_0_I_DRIVER_APB_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_PCIE_SUB_CTRL_INST_1_I_DRIVER_APB_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_PIPE2_DIGITAL_X2_WRAP_INST_0_I_APB_PCLK_SCL,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_SLV_ACLK_0,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_SLV_ACLK_1,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PMU_FSYS1_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_BCM_FSYS1_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_BCM_FSYS1_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_RSTNSYNC_CLK_FSYS1_BUS_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_RTIC_IPCLKPORT_I_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_RTIC_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_SSS_IPCLKPORT_I_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_SSS_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_SYSREG_FSYS1_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_TOE_WIFI0_IPCLKPORT_I_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_TOE_WIFI1_IPCLKPORT_I_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_UFS_CARD_IPCLKPORT_I_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_UFS_CARD_IPCLKPORT_I_CLK_UNIPRO,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_UFS_CARD_IPCLKPORT_I_FMP_CLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_XIU_D_FSYS1_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_FSYS1_UID_XIU_P_FSYS1_IPCLKPORT_ACLK,
];

/* List of parent clocks for Muxes in CMU_FSYS1 */
static mout_fsys1_bus_user_p: PNAME(mout_fsys1_bus_user_p)		= {[PNAME(mout_fsys1_bus_user_p)		= {str] = &[ "oscclk", "dout_cmu_fsys1_bus" ];
static mout_fsys1_mmc_card_user_p: PNAME(mout_fsys1_mmc_card_user_p)	= {[PNAME(mout_fsys1_mmc_card_user_p)	= {str] = &[ "oscclk",
					    "dout_cmu_fsys1_mmc_card" ];
static mout_fsys1_pcie_user_p: PNAME(mout_fsys1_pcie_user_p)		= {[PNAME(mout_fsys1_pcie_user_p)		= {str] = &[ "oscclk", "dout_cmu_fsys1_pcie" ];
static mout_fsys1_ufs_card_user_p: PNAME(mout_fsys1_ufs_card_user_p)	= {[PNAME(mout_fsys1_ufs_card_user_p)	= {str] = &[ "oscclk",
					    "dout_cmu_fsys1_ufs_card" ];

static fsys1_mux_clks: static const struct samsung_mux_clock fsys1_mux_clks[]  = {[samsung_mux_clock] = &[
	MUX(CLK_MOUT_FSYS1_BUS_USER, "mout_fsys1_bus_user",
	    mout_fsys1_bus_user_p, PLL_CON0_MUX_CLKCMU_FSYS1_BUS_USER, 4, 1),
	MUX_F(CLK_MOUT_FSYS1_MMC_CARD_USER, "mout_fsys1_mmc_card_user",
	      mout_fsys1_mmc_card_user_p,
	      PLL_CON0_MUX_CLKCMU_FSYS1_MMC_CARD_USER,
	      4, 1, CLK_SET_RATE_PARENT, 0),
	MUX(CLK_MOUT_FSYS1_PCIE_USER, "mout_fsys1_pcie_user",
	    mout_fsys1_pcie_user_p, PLL_CON0_MUX_CLKCMU_FSYS1_PCIE_USER, 4, 1),
	MUX(CLK_MOUT_FSYS1_UFS_CARD_USER, "mout_fsys1_ufs_card_user",
	    mout_fsys1_ufs_card_user_p,
	    PLL_CON0_MUX_CLKCMU_FSYS1_UFS_CARD_USER, 4, 1),
];

static fsys1_gate_clks: static const struct samsung_gate_clock fsys1_gate_clks[]  = {[samsung_gate_clock] = &[
	GATE(CLK_GOUT_FSYS1_PCIE_PHY_REF_CLK_IN,
	     "gout_clk_blk_fsys1_pcie_phy_ref_clk_in", "mout_fsys1_pcie_user",
	     CLK_CON_GAT_CLK_BLK_FSYS1_UID_PCIE_IPCLKPORT_PHY_REF_CLK_IN,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_ADM_AHB_SSS_HCLKM, "gout_fsys1_adm_ahb_sss_hclkm",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_ADM_AHB_SSS_IPCLKPORT_HCLKM,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_AHBBR_FSYS1_HCLK, "gout_fsys1_ahbbr_fsys1_hclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_AHBBR_FSYS1_IPCLKPORT_HCLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS1_AXI2AHB_FSYS1_ACLK,
	     "gout_fsys1_axi2ahb_fsys1_aclk", "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_AXI2AHB_FSYS1_IPCLKPORT_ACLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS1_AXI2APB_FSYS1P0_ACLK,
	     "gout_fsys1_axi2apb_fsys1p0_aclk", "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_AXI2APB_FSYS1P0_IPCLKPORT_ACLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS1_AXI2APB_FSYS1P1_ACLK,
	     "gout_fsys1_axi2apb_fsys1p1_aclk", "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_AXI2APB_FSYS1P1_IPCLKPORT_ACLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS1_BTM_FSYS1_I_ACLK, "gout_fsys1_btm_fsys1_i_aclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_BTM_FSYS1_IPCLKPORT_I_ACLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_BTM_FSYS1_I_PCLK, "gout_fsys1_btm_fsys1_i_pclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_BTM_FSYS1_IPCLKPORT_I_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_FSYS1_CMU_FSYS1_PCLK,
	     "gout_fsys1_fsys1_cmu_fsys1_pclk", "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_FSYS1_CMU_FSYS1_IPCLKPORT_PCLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS1_GPIO_FSYS1_PCLK, "gout_fsys1_gpio_fsys1_pclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_GPIO_FSYS1_IPCLKPORT_PCLK,
	     21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_FSYS1_LHM_AXI_P_FSYS1_I_CLK,
	     "gout_fsys1_lhm_axi_p_fsys1_i_clk", "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_LHM_AXI_P_FSYS1_IPCLKPORT_I_CLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS1_LHS_ACEL_D_FSYS1_I_CLK,
	     "gout_fsys1_lhs_acel_d_fsys1_i_clk", "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_LHS_ACEL_D_FSYS1_IPCLKPORT_I_CLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_FSYS1_MMC_CARD_I_ACLK, "gout_fsys1_mmc_card_i_aclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_MMC_CARD_IPCLKPORT_I_ACLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_MMC_CARD_SDCLKIN, "gout_fsys1_mmc_card_sdclkin",
	     "mout_fsys1_mmc_card_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_MMC_CARD_IPCLKPORT_SDCLKIN,
	     21, CLK_SET_RATE_PARENT, 0),
	GATE(CLK_GOUT_FSYS1_PCIE_DBI_ACLK_0, "gout_fsys1_pcie_dbi_aclk_0",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_DBI_ACLK_0,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_PCIE_DBI_ACLK_1, "gout_fsys1_pcie_dbi_aclk_1",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_DBI_ACLK_1,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_PCIE_IEEE1500_WRAPPER_FOR_PCIE_PHY_LC_X2_INST_0_I_SCL_APB_PCLK,
	     "gout_fsys1_pcie_ieee1500_wrapper_for_pcie_phy_lc_x2_inst_0_i_scl_apb_pclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_IEEE1500_WRAPPER_FOR_PCIE_PHY_LC_X2_INST_0_I_SCL_APB_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_PCIE_MSTR_ACLK_0,
	     "gout_fsys1_pcie_mstr_aclk_0", "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_MSTR_ACLK_0,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_PCIE_MSTR_ACLK_1,
	     "gout_fsys1_pcie_mstr_aclk_1", "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_MSTR_ACLK_1,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_PCIE_PCIE_SUB_CTRL_INST_0_I_DRIVER_APB_CLK,
	     "gout_fsys1_pcie_pcie_sub_ctrl_inst_0_i_driver_apb_clk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_PCIE_SUB_CTRL_INST_0_I_DRIVER_APB_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_PCIE_PCIE_SUB_CTRL_INST_1_I_DRIVER_APB_CLK,
	     "gout_fsys1_pcie_pcie_sub_ctrl_inst_1_i_driver_apb_clk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_PCIE_SUB_CTRL_INST_1_I_DRIVER_APB_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_PCIE_PIPE2_DIGITAL_X2_WRAP_INST_0_I_APB_PCLK_SCL,
	     "gout_fsys1_pcie_pipe2_digital_x2_wrap_inst_0_i_apb_pclk_scl",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_PIPE2_DIGITAL_X2_WRAP_INST_0_I_APB_PCLK_SCL,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_PCIE_SLV_ACLK_0, "gout_fsys1_pcie_slv_aclk_0",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_SLV_ACLK_0,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_PCIE_SLV_ACLK_1, "gout_fsys1_pcie_slv_aclk_1",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PCIE_IPCLKPORT_SLV_ACLK_1,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_PMU_FSYS1_PCLK, "gout_fsys1_pmu_fsys1_pclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_PMU_FSYS1_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_BCM_FSYS1_ACLK, "gout_fsys1_bcm_fsys1_aclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_BCM_FSYS1_IPCLKPORT_ACLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_BCM_FSYS1_PCLK, "gout_fsys1_bcm_fsys1_pclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_BCM_FSYS1_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_RSTNSYNC_CLK_FSYS1_BUS_CLK,
	     "gout_fsys1_rstnsync_clk_fsys1_bus_clk", "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_RSTNSYNC_CLK_FSYS1_BUS_IPCLKPORT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_RTIC_I_ACLK, "gout_fsys1_rtic_i_aclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_RTIC_IPCLKPORT_I_ACLK, 21, 0, 0),
	GATE(CLK_GOUT_FSYS1_RTIC_I_PCLK, "gout_fsys1_rtic_i_pclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_RTIC_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_FSYS1_SSS_I_ACLK, "gout_fsys1_sss_i_aclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_SSS_IPCLKPORT_I_ACLK, 21, 0, 0),
	GATE(CLK_GOUT_FSYS1_SSS_I_PCLK, "gout_fsys1_sss_i_pclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_SSS_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_FSYS1_SYSREG_FSYS1_PCLK, "gout_fsys1_sysreg_fsys1_pclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_SYSREG_FSYS1_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_TOE_WIFI0_I_CLK, "gout_fsys1_toe_wifi0_i_clk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_TOE_WIFI0_IPCLKPORT_I_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_TOE_WIFI1_I_CLK, "gout_fsys1_toe_wifi1_i_clk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_TOE_WIFI1_IPCLKPORT_I_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_UFS_CARD_I_ACLK, "gout_fsys1_ufs_card_i_aclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_UFS_CARD_IPCLKPORT_I_ACLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_UFS_CARD_I_CLK_UNIPRO,
	     "gout_fsys1_ufs_card_i_clk_unipro", "mout_fsys1_ufs_card_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_UFS_CARD_IPCLKPORT_I_CLK_UNIPRO,
	     21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_FSYS1_UFS_CARD_I_FMP_CLK,
	     "gout_fsys1_ufs_card_i_fmp_clk", "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_UFS_CARD_IPCLKPORT_I_FMP_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_FSYS1_XIU_D_FSYS1_ACLK, "gout_fsys1_xiu_d_fsys1_aclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_XIU_D_FSYS1_IPCLKPORT_ACLK,
	     21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_FSYS1_XIU_P_FSYS1_ACLK, "gout_fsys1_xiu_p_fsys1_aclk",
	     "mout_fsys1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_FSYS1_UID_XIU_P_FSYS1_IPCLKPORT_ACLK,
	     21, CLK_IGNORE_UNUSED, 0),
];

static fsys1_cmu_info: samsung_cmu_info = samsung_cmu_info {
.mux_clks = fsys1_mux_clks,
.nr_mux_clks = fsys1_mux_clks.len(),
.gate_clks = fsys1_gate_clks,
.nr_gate_clks = fsys1_gate_clks.len(),
.nr_clk_ids = CLKS_NR_FSYS1,
.clk_regs = fsys1_clk_regs,
.nr_clk_regs = fsys1_clk_regs.len(),
.clk_name = "bus",
];

/* ---- CMU_PERIC0 ---------------------------------------------------------- */

/* Register Offset definitions for CMU_PERIC0 (0x10400000) */
const PLL_CON0_MUX_CLKCMU_PERIC0_BUS_USER: usize = 0x0100;
const PLL_CON2_MUX_CLKCMU_PERIC0_BUS_USER: usize = 0x0108;
const PLL_CON0_MUX_CLKCMU_PERIC0_UART_DBG_USER: usize = 0x0120;
const PLL_CON2_MUX_CLKCMU_PERIC0_UART_DBG_USER: usize = 0x0128;
const PLL_CON0_MUX_CLKCMU_PERIC0_USI00_USER: usize = 0x0140;
const PLL_CON2_MUX_CLKCMU_PERIC0_USI00_USER: usize = 0x0148;
const PLL_CON0_MUX_CLKCMU_PERIC0_USI01_USER: usize = 0x0160;
const PLL_CON2_MUX_CLKCMU_PERIC0_USI01_USER: usize = 0x0168;
const PLL_CON0_MUX_CLKCMU_PERIC0_USI02_USER: usize = 0x0180;
const PLL_CON2_MUX_CLKCMU_PERIC0_USI02_USER: usize = 0x0188;
const PLL_CON0_MUX_CLKCMU_PERIC0_USI03_USER: usize = 0x01a0;
const PLL_CON2_MUX_CLKCMU_PERIC0_USI03_USER: usize = 0x01a8;
const CLK_CON_GAT_CLK_BLK_PERIC0_UID_PERIC0_CMU_PERIC0_IPCLKPORT_PCLK: usize = 0x2000;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_AXI2APB_PERIC0_IPCLKPORT_ACLK: usize = 0x2014;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_GPIO_PERIC0_IPCLKPORT_PCLK: usize = 0x2018;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_LHM_AXI_P_PERIC0_IPCLKPORT_I_CLK: usize = 0x201c;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_PMU_PERIC0_IPCLKPORT_PCLK: usize = 0x2020;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_PWM_IPCLKPORT_I_PCLK_S0: usize = 0x2028;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_RSTNSYNC_CLK_PERIC0_BUSP_IPCLKPORT_CLK: usize = 0x202c;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_SPEEDY2_TSP_IPCLKPORT_CLK: usize = 0x2030;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_SYSREG_PERIC0_IPCLKPORT_PCLK: usize = 0x2034;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_UART_DBG_IPCLKPORT_EXT_UCLK: usize = 0x2038;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_UART_DBG_IPCLKPORT_PCLK: usize = 0x203c;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI00_IPCLKPORT_I_PCLK: usize = 0x2040;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI00_IPCLKPORT_I_SCLK_USI: usize = 0x2044;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI01_IPCLKPORT_I_PCLK: usize = 0x2048;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI01_IPCLKPORT_I_SCLK_USI: usize = 0x204c;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI02_IPCLKPORT_I_PCLK: usize = 0x2050;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI02_IPCLKPORT_I_SCLK_USI: usize = 0x2054;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI03_IPCLKPORT_I_PCLK: usize = 0x2058;
const CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI03_IPCLKPORT_I_SCLK_USI: usize = 0x205c;

static peric0_clk_regs: static const unsigned long peric0_clk_regs[]  = {[u64] = &[
	PLL_CON0_MUX_CLKCMU_PERIC0_BUS_USER,
	PLL_CON2_MUX_CLKCMU_PERIC0_BUS_USER,
	PLL_CON0_MUX_CLKCMU_PERIC0_UART_DBG_USER,
	PLL_CON2_MUX_CLKCMU_PERIC0_UART_DBG_USER,
	PLL_CON0_MUX_CLKCMU_PERIC0_USI00_USER,
	PLL_CON2_MUX_CLKCMU_PERIC0_USI00_USER,
	PLL_CON0_MUX_CLKCMU_PERIC0_USI01_USER,
	PLL_CON2_MUX_CLKCMU_PERIC0_USI01_USER,
	PLL_CON0_MUX_CLKCMU_PERIC0_USI02_USER,
	PLL_CON2_MUX_CLKCMU_PERIC0_USI02_USER,
	PLL_CON0_MUX_CLKCMU_PERIC0_USI03_USER,
	PLL_CON2_MUX_CLKCMU_PERIC0_USI03_USER,
	CLK_CON_GAT_CLK_BLK_PERIC0_UID_PERIC0_CMU_PERIC0_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_AXI2APB_PERIC0_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_GPIO_PERIC0_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_LHM_AXI_P_PERIC0_IPCLKPORT_I_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_PMU_PERIC0_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_PWM_IPCLKPORT_I_PCLK_S0,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_RSTNSYNC_CLK_PERIC0_BUSP_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_SPEEDY2_TSP_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_SYSREG_PERIC0_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_UART_DBG_IPCLKPORT_EXT_UCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_UART_DBG_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI00_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI00_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI01_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI01_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI02_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI02_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI03_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI03_IPCLKPORT_I_SCLK_USI,
];

/* List of parent clocks for Muxes in CMU_PERIC0 */
static mout_peric0_bus_user_p: PNAME(mout_peric0_bus_user_p)		= {[PNAME(mout_peric0_bus_user_p)		= {str] = &[ "oscclk", "dout_cmu_peric0_bus" ];
static mout_peric0_uart_dbg_user_p: PNAME(mout_peric0_uart_dbg_user_p)	= {[PNAME(mout_peric0_uart_dbg_user_p)	= {str] = &[ "oscclk",
					    "dout_cmu_peric0_uart_dbg" ];
static mout_peric0_usi00_user_p: PNAME(mout_peric0_usi00_user_p)		= {[PNAME(mout_peric0_usi00_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric0_usi00" ];
static mout_peric0_usi01_user_p: PNAME(mout_peric0_usi01_user_p)		= {[PNAME(mout_peric0_usi01_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric0_usi01" ];
static mout_peric0_usi02_user_p: PNAME(mout_peric0_usi02_user_p)		= {[PNAME(mout_peric0_usi02_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric0_usi02" ];
static mout_peric0_usi03_user_p: PNAME(mout_peric0_usi03_user_p)		= {[PNAME(mout_peric0_usi03_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric0_usi03" ];

static peric0_mux_clks: static const struct samsung_mux_clock peric0_mux_clks[]  = {[samsung_mux_clock] = &[
	MUX(CLK_MOUT_PERIC0_BUS_USER, "mout_peric0_bus_user",
	    mout_peric0_bus_user_p, PLL_CON0_MUX_CLKCMU_PERIC0_BUS_USER, 4, 1),
	MUX(CLK_MOUT_PERIC0_UART_DBG_USER, "mout_peric0_uart_dbg_user",
	    mout_peric0_uart_dbg_user_p,
	    PLL_CON0_MUX_CLKCMU_PERIC0_UART_DBG_USER, 4, 1),
	MUX(CLK_MOUT_PERIC0_USI00_USER, "mout_peric0_usi00_user",
	    mout_peric0_usi00_user_p, PLL_CON0_MUX_CLKCMU_PERIC0_USI00_USER,
	    4, 1),
	MUX(CLK_MOUT_PERIC0_USI01_USER, "mout_peric0_usi01_user",
	    mout_peric0_usi01_user_p, PLL_CON0_MUX_CLKCMU_PERIC0_USI01_USER,
	    4, 1),
	MUX(CLK_MOUT_PERIC0_USI02_USER, "mout_peric0_usi02_user",
	    mout_peric0_usi02_user_p, PLL_CON0_MUX_CLKCMU_PERIC0_USI02_USER,
	    4, 1),
	MUX(CLK_MOUT_PERIC0_USI03_USER, "mout_peric0_usi03_user",
	    mout_peric0_usi03_user_p, PLL_CON0_MUX_CLKCMU_PERIC0_USI03_USER,
	    4, 1),
];

static peric0_gate_clks: static const struct samsung_gate_clock peric0_gate_clks[]  = {[samsung_gate_clock] = &[
	GATE(CLK_GOUT_PERIC0_PERIC0_CMU_PERIC0_PCLK,
	     "gout_cperic0_peric0_cmu_peric0_pclk", "mout_peric0_bus_user",
	     CLK_CON_GAT_CLK_BLK_PERIC0_UID_PERIC0_CMU_PERIC0_IPCLKPORT_PCLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIC0_AXI2APB_PERIC0_ACLK,
	     "gout_peric0_axi2apb_peric0_aclk", "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_AXI2APB_PERIC0_IPCLKPORT_ACLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIC0_GPIO_PERIC0_PCLK, "gout_peric0_gpio_peric0_pclk",
	     "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_GPIO_PERIC0_IPCLKPORT_PCLK,
	     21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_PERIC0_LHM_AXI_P_PERIC0_I_CLK,
	     "gout_peric0_lhm_axi_p_peric0_i_clk", "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_LHM_AXI_P_PERIC0_IPCLKPORT_I_CLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIC0_PMU_PERIC0_PCLK, "gout_peric0_pmu_peric0_pclk",
	     "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_PMU_PERIC0_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC0_PWM_I_PCLK_S0, "gout_peric0_pwm_i_pclk_s0",
	     "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_PWM_IPCLKPORT_I_PCLK_S0,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC0_RSTNSYNC_CLK_PERIC0_BUSP_CLK,
	     "gout_peric0_rstnsync_clk_peric0_busp_clk",
	     "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_RSTNSYNC_CLK_PERIC0_BUSP_IPCLKPORT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC0_SPEEDY2_TSP_CLK, "gout_peric0_speedy2_tsp_clk",
	     "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_SPEEDY2_TSP_IPCLKPORT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC0_SYSREG_PERIC0_PCLK,
	     "gout_peric0_sysreg_peric0_pclk", "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_SYSREG_PERIC0_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC0_UART_DBG_EXT_UCLK,
	     "gout_peric0_uart_dbg_ext_uclk", "mout_peric0_uart_dbg_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_UART_DBG_IPCLKPORT_EXT_UCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC0_UART_DBG_PCLK, "gout_peric0_uart_dbg_pclk",
	     "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_UART_DBG_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC0_USI00_I_PCLK, "gout_peric0_usi00_i_pclk",
	     "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI00_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC0_USI00_I_SCLK_USI, "gout_peric0_usi00_i_sclk_usi",
	     "mout_peric0_usi00_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI00_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC0_USI01_I_PCLK, "gout_peric0_usi01_i_pclk",
	     "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI01_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC0_USI01_I_SCLK_USI, "gout_peric0_usi01_i_sclk_usi",
	     "mout_peric0_usi01_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI01_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC0_USI02_I_PCLK, "gout_peric0_usi02_i_pclk",
	     "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI02_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC0_USI02_I_SCLK_USI, "gout_peric0_usi02_i_sclk_usi",
	     "mout_peric0_usi02_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI02_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC0_USI03_I_PCLK, "gout_peric0_usi03_i_pclk",
	     "mout_peric0_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI03_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC0_USI03_I_SCLK_USI, "gout_peric0_usi03_i_sclk_usi",
	     "mout_peric0_usi03_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC0_UID_USI03_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
];

static peric0_cmu_info: samsung_cmu_info = samsung_cmu_info {
.mux_clks = peric0_mux_clks,
.nr_mux_clks = peric0_mux_clks.len(),
.gate_clks = peric0_gate_clks,
.nr_gate_clks = peric0_gate_clks.len(),
.nr_clk_ids = CLKS_NR_PERIC0,
.clk_regs = peric0_clk_regs,
.nr_clk_regs = peric0_clk_regs.len(),
.clk_name = "bus",
];

/* ---- CMU_PERIC1 ---------------------------------------------------------- */

/* Register Offset definitions for CMU_PERIC1 (0x10800000) */
const PLL_CON0_MUX_CLKCMU_PERIC1_BUS_USER: usize = 0x0100;
const PLL_CON2_MUX_CLKCMU_PERIC1_BUS_USER: usize = 0x0108;
const PLL_CON0_MUX_CLKCMU_PERIC1_SPEEDY2_USER: usize = 0x0120;
const PLL_CON2_MUX_CLKCMU_PERIC1_SPEEDY2_USER: usize = 0x0128;
const PLL_CON0_MUX_CLKCMU_PERIC1_SPI_CAM0_USER: usize = 0x0140;
const PLL_CON2_MUX_CLKCMU_PERIC1_SPI_CAM0_USER: usize = 0x0148;
const PLL_CON0_MUX_CLKCMU_PERIC1_SPI_CAM1_USER: usize = 0x0160;
const PLL_CON2_MUX_CLKCMU_PERIC1_SPI_CAM1_USER: usize = 0x0168;
const PLL_CON0_MUX_CLKCMU_PERIC1_UART_BT_USER: usize = 0x0180;
const PLL_CON2_MUX_CLKCMU_PERIC1_UART_BT_USER: usize = 0x0188;
const PLL_CON0_MUX_CLKCMU_PERIC1_USI04_USER: usize = 0x01a0;
const PLL_CON2_MUX_CLKCMU_PERIC1_USI04_USER: usize = 0x01a8;
const PLL_CON0_MUX_CLKCMU_PERIC1_USI05_USER: usize = 0x01c0;
const PLL_CON2_MUX_CLKCMU_PERIC1_USI05_USER: usize = 0x01c8;
const PLL_CON0_MUX_CLKCMU_PERIC1_USI06_USER: usize = 0x01e0;
const PLL_CON2_MUX_CLKCMU_PERIC1_USI06_USER: usize = 0x01e8;
const PLL_CON0_MUX_CLKCMU_PERIC1_USI07_USER: usize = 0x0200;
const PLL_CON2_MUX_CLKCMU_PERIC1_USI07_USER: usize = 0x0208;
const PLL_CON0_MUX_CLKCMU_PERIC1_USI08_USER: usize = 0x0220;
const PLL_CON2_MUX_CLKCMU_PERIC1_USI08_USER: usize = 0x0228;
const PLL_CON0_MUX_CLKCMU_PERIC1_USI09_USER: usize = 0x0240;
const PLL_CON2_MUX_CLKCMU_PERIC1_USI09_USER: usize = 0x0248;
const PLL_CON0_MUX_CLKCMU_PERIC1_USI10_USER: usize = 0x0260;
const PLL_CON2_MUX_CLKCMU_PERIC1_USI10_USER: usize = 0x0268;
const PLL_CON0_MUX_CLKCMU_PERIC1_USI11_USER: usize = 0x0280;
const PLL_CON2_MUX_CLKCMU_PERIC1_USI11_USER: usize = 0x0288;
const PLL_CON0_MUX_CLKCMU_PERIC1_USI12_USER: usize = 0x02a0;
const PLL_CON2_MUX_CLKCMU_PERIC1_USI12_USER: usize = 0x02a8;
const PLL_CON0_MUX_CLKCMU_PERIC1_USI13_USER: usize = 0x02c0;
const PLL_CON2_MUX_CLKCMU_PERIC1_USI13_USER: usize = 0x02c8;
const CLK_CON_GAT_CLK_BLK_PERIC1_UID_PERIC1_CMU_PERIC1_IPCLKPORT_PCLK: usize = 0x2000;
const CLK_CON_GAT_CLK_BLK_PERIC1_UID_RSTNSYNC_CLK_PERIC1_SPEEDY2_IPCLKPORT_CLK: usize = 0x200c;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_AXI2APB_PERIC1P0_IPCLKPORT_ACLK: usize = 0x201c;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_AXI2APB_PERIC1P1_IPCLKPORT_ACLK: usize = 0x2020;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_AXI2APB_PERIC1P2_IPCLKPORT_ACLK: usize = 0x2024;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_GPIO_PERIC1_IPCLKPORT_PCLK: usize = 0x2028;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_HSI2C_CAM0_IPCLKPORT_IPCLK: usize = 0x202c;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_HSI2C_CAM1_IPCLKPORT_IPCLK: usize = 0x2030;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_HSI2C_CAM2_IPCLKPORT_IPCLK: usize = 0x2034;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_HSI2C_CAM3_IPCLKPORT_IPCLK: usize = 0x2038;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_LHM_AXI_P_PERIC1_IPCLKPORT_I_CLK: usize = 0x203c;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_PMU_PERIC1_IPCLKPORT_PCLK: usize = 0x2040;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_RSTNSYNC_CLK_PERIC1_BUSP_IPCLKPORT_CLK: usize = 0x2044;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI1_IPCLKPORT_CLK: usize = 0x2048;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI1_IPCLKPORT_SCLK: usize = 0x204c;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI2_IPCLKPORT_CLK: usize = 0x2050;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI2_IPCLKPORT_SCLK: usize = 0x2054;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI_IPCLKPORT_CLK: usize = 0x2058;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI_IPCLKPORT_SCLK: usize = 0x205c;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_TSP1_IPCLKPORT_CLK: usize = 0x2060;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_TSP2_IPCLKPORT_CLK: usize = 0x2064;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPI_CAM0_IPCLKPORT_PCLK: usize = 0x2068;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPI_CAM0_IPCLKPORT_SPI_EXT_CLK: usize = 0x206c;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPI_CAM1_IPCLKPORT_PCLK: usize = 0x2070;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPI_CAM1_IPCLKPORT_SPI_EXT_CLK: usize = 0x2074;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SYSREG_PERIC1_IPCLKPORT_PCLK: usize = 0x2078;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_UART_BT_IPCLKPORT_EXT_UCLK: usize = 0x207c;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_UART_BT_IPCLKPORT_PCLK: usize = 0x2080;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI04_IPCLKPORT_I_PCLK: usize = 0x2084;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI04_IPCLKPORT_I_SCLK_USI: usize = 0x2088;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI05_IPCLKPORT_I_PCLK: usize = 0x208c;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI05_IPCLKPORT_I_SCLK_USI: usize = 0x2090;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI06_IPCLKPORT_I_PCLK: usize = 0x2094;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI06_IPCLKPORT_I_SCLK_USI: usize = 0x2098;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI07_IPCLKPORT_I_PCLK: usize = 0x209c;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI07_IPCLKPORT_I_SCLK_USI: usize = 0x20a0;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI08_IPCLKPORT_I_PCLK: usize = 0x20a4;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI08_IPCLKPORT_I_SCLK_USI: usize = 0x20a8;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI09_IPCLKPORT_I_PCLK: usize = 0x20ac;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI09_IPCLKPORT_I_SCLK_USI: usize = 0x20b0;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI10_IPCLKPORT_I_PCLK: usize = 0x20b4;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI10_IPCLKPORT_I_SCLK_USI: usize = 0x20b8;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI11_IPCLKPORT_I_PCLK: usize = 0x20bc;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI11_IPCLKPORT_I_SCLK_USI: usize = 0x20c0;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI12_IPCLKPORT_I_PCLK: usize = 0x20c4;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI12_IPCLKPORT_I_SCLK_USI: usize = 0x20c8;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI13_IPCLKPORT_I_PCLK: usize = 0x20cc;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI13_IPCLKPORT_I_SCLK_USI: usize = 0x20d0;
const CLK_CON_GAT_GOUT_BLK_PERIC1_UID_XIU_P_PERIC1_IPCLKPORT_ACLK: usize = 0x20d4;

static peric1_clk_regs: static const unsigned long peric1_clk_regs[]  = {[u64] = &[
	PLL_CON0_MUX_CLKCMU_PERIC1_BUS_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_BUS_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_SPEEDY2_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_SPEEDY2_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_SPI_CAM0_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_SPI_CAM0_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_SPI_CAM1_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_SPI_CAM1_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_UART_BT_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_UART_BT_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_USI04_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_USI04_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_USI05_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_USI05_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_USI06_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_USI06_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_USI07_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_USI07_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_USI08_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_USI08_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_USI09_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_USI09_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_USI10_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_USI10_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_USI11_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_USI11_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_USI12_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_USI12_USER,
	PLL_CON0_MUX_CLKCMU_PERIC1_USI13_USER,
	PLL_CON2_MUX_CLKCMU_PERIC1_USI13_USER,
	CLK_CON_GAT_CLK_BLK_PERIC1_UID_PERIC1_CMU_PERIC1_IPCLKPORT_PCLK,
	CLK_CON_GAT_CLK_BLK_PERIC1_UID_RSTNSYNC_CLK_PERIC1_SPEEDY2_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_AXI2APB_PERIC1P0_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_AXI2APB_PERIC1P1_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_AXI2APB_PERIC1P2_IPCLKPORT_ACLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_GPIO_PERIC1_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_HSI2C_CAM0_IPCLKPORT_IPCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_HSI2C_CAM1_IPCLKPORT_IPCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_HSI2C_CAM2_IPCLKPORT_IPCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_HSI2C_CAM3_IPCLKPORT_IPCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_LHM_AXI_P_PERIC1_IPCLKPORT_I_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_PMU_PERIC1_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_RSTNSYNC_CLK_PERIC1_BUSP_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI1_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI1_IPCLKPORT_SCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI2_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI2_IPCLKPORT_SCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI_IPCLKPORT_SCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_TSP1_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_TSP2_IPCLKPORT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPI_CAM0_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPI_CAM0_IPCLKPORT_SPI_EXT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPI_CAM1_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPI_CAM1_IPCLKPORT_SPI_EXT_CLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SYSREG_PERIC1_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_UART_BT_IPCLKPORT_EXT_UCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_UART_BT_IPCLKPORT_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI04_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI04_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI05_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI05_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI06_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI06_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI07_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI07_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI08_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI08_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI09_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI09_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI10_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI10_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI11_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI11_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI12_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI12_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI13_IPCLKPORT_I_PCLK,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI13_IPCLKPORT_I_SCLK_USI,
	CLK_CON_GAT_GOUT_BLK_PERIC1_UID_XIU_P_PERIC1_IPCLKPORT_ACLK,
];

/* List of parent clocks for Muxes in CMU_PERIC1 */
static mout_peric1_bus_user_p: PNAME(mout_peric1_bus_user_p)		= {[PNAME(mout_peric1_bus_user_p)		= {str] = &[ "oscclk", "dout_cmu_peric1_bus" ];
static mout_peric1_speedy2_user_p: PNAME(mout_peric1_speedy2_user_p)	= {[PNAME(mout_peric1_speedy2_user_p)	= {str] = &[ "oscclk",
					    "dout_cmu_peric1_speedy2" ];
static mout_peric1_spi_cam0_user_p: PNAME(mout_peric1_spi_cam0_user_p)	= {[PNAME(mout_peric1_spi_cam0_user_p)	= {str] = &[ "oscclk",
					    "dout_cmu_peric1_spi_cam0" ];
static mout_peric1_spi_cam1_user_p: PNAME(mout_peric1_spi_cam1_user_p)	= {[PNAME(mout_peric1_spi_cam1_user_p)	= {str] = &[ "oscclk",
					    "dout_cmu_peric1_spi_cam1" ];
static mout_peric1_uart_bt_user_p: PNAME(mout_peric1_uart_bt_user_p)	= {[PNAME(mout_peric1_uart_bt_user_p)	= {str] = &[ "oscclk",
					    "dout_cmu_peric1_uart_bt" ];
static mout_peric1_usi04_user_p: PNAME(mout_peric1_usi04_user_p)		= {[PNAME(mout_peric1_usi04_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric1_usi04" ];
static mout_peric1_usi05_user_p: PNAME(mout_peric1_usi05_user_p)		= {[PNAME(mout_peric1_usi05_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric1_usi05" ];
static mout_peric1_usi06_user_p: PNAME(mout_peric1_usi06_user_p)		= {[PNAME(mout_peric1_usi06_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric1_usi06" ];
static mout_peric1_usi07_user_p: PNAME(mout_peric1_usi07_user_p)		= {[PNAME(mout_peric1_usi07_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric1_usi07" ];
static mout_peric1_usi08_user_p: PNAME(mout_peric1_usi08_user_p)		= {[PNAME(mout_peric1_usi08_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric1_usi08" ];
static mout_peric1_usi09_user_p: PNAME(mout_peric1_usi09_user_p)		= {[PNAME(mout_peric1_usi09_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric1_usi09" ];
static mout_peric1_usi10_user_p: PNAME(mout_peric1_usi10_user_p)		= {[PNAME(mout_peric1_usi10_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric1_usi10" ];
static mout_peric1_usi11_user_p: PNAME(mout_peric1_usi11_user_p)		= {[PNAME(mout_peric1_usi11_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric1_usi11" ];
static mout_peric1_usi12_user_p: PNAME(mout_peric1_usi12_user_p)		= {[PNAME(mout_peric1_usi12_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric1_usi12" ];
static mout_peric1_usi13_user_p: PNAME(mout_peric1_usi13_user_p)		= {[PNAME(mout_peric1_usi13_user_p)		= {str] = &[ "oscclk",
					    "dout_cmu_peric1_usi13" ];

static peric1_mux_clks: static const struct samsung_mux_clock peric1_mux_clks[]  = {[samsung_mux_clock] = &[
	MUX(CLK_MOUT_PERIC1_BUS_USER, "mout_peric1_bus_user",
	    mout_peric1_bus_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_BUS_USER, 4, 1),
	MUX(CLK_MOUT_PERIC1_SPEEDY2_USER, "mout_peric1_speedy2_user",
	    mout_peric1_speedy2_user_p,
	    PLL_CON0_MUX_CLKCMU_PERIC1_SPEEDY2_USER, 4, 1),
	MUX(CLK_MOUT_PERIC1_SPI_CAM0_USER, "mout_peric1_spi_cam0_user",
	    mout_peric1_spi_cam0_user_p,
	    PLL_CON0_MUX_CLKCMU_PERIC1_SPI_CAM0_USER, 4, 1),
	MUX(CLK_MOUT_PERIC1_SPI_CAM1_USER, "mout_peric1_spi_cam1_user",
	    mout_peric1_spi_cam1_user_p,
	    PLL_CON0_MUX_CLKCMU_PERIC1_SPI_CAM1_USER, 4, 1),
	MUX(CLK_MOUT_PERIC1_UART_BT_USER, "mout_peric1_uart_bt_user",
	    mout_peric1_uart_bt_user_p,
	    PLL_CON0_MUX_CLKCMU_PERIC1_UART_BT_USER, 4, 1),
	MUX(CLK_MOUT_PERIC1_USI04_USER, "mout_peric1_usi04_user",
	    mout_peric1_usi04_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_USI04_USER,
	    4, 1),
	MUX(CLK_MOUT_PERIC1_USI05_USER, "mout_peric1_usi05_user",
	    mout_peric1_usi05_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_USI05_USER,
	    4, 1),
	MUX(CLK_MOUT_PERIC1_USI06_USER, "mout_peric1_usi06_user",
	    mout_peric1_usi06_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_USI06_USER,
	    4, 1),
	MUX(CLK_MOUT_PERIC1_USI07_USER, "mout_peric1_usi07_user",
	    mout_peric1_usi07_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_USI07_USER,
	    4, 1),
	MUX(CLK_MOUT_PERIC1_USI08_USER, "mout_peric1_usi08_user",
	    mout_peric1_usi08_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_USI08_USER,
	    4, 1),
	MUX(CLK_MOUT_PERIC1_USI09_USER, "mout_peric1_usi09_user",
	    mout_peric1_usi09_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_USI09_USER,
	    4, 1),
	MUX(CLK_MOUT_PERIC1_USI10_USER, "mout_peric1_usi10_user",
	    mout_peric1_usi10_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_USI10_USER,
	    4, 1),
	MUX(CLK_MOUT_PERIC1_USI11_USER, "mout_peric1_usi11_user",
	    mout_peric1_usi11_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_USI11_USER,
	    4, 1),
	MUX(CLK_MOUT_PERIC1_USI12_USER, "mout_peric1_usi12_user",
	    mout_peric1_usi12_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_USI12_USER,
	    4, 1),
	MUX(CLK_MOUT_PERIC1_USI13_USER, "mout_peric1_usi13_user",
	    mout_peric1_usi13_user_p, PLL_CON0_MUX_CLKCMU_PERIC1_USI13_USER,
	    4, 1),
];

static peric1_gate_clks: static const struct samsung_gate_clock peric1_gate_clks[]  = {[samsung_gate_clock] = &[
	GATE(CLK_GOUT_PERIC1_PERIC1_CMU_PERIC1_PCLK,
	     "gout_peric1_peric1_cmu_peric1_pclk", "mout_peric1_bus_user",
	     CLK_CON_GAT_CLK_BLK_PERIC1_UID_PERIC1_CMU_PERIC1_IPCLKPORT_PCLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIC1_RSTNSYNC_CLK_PERIC1_SPEEDY2_CLK,
	     "gout_peric1_rstnsync_clk_peric1_speedy2_clk",
	     "mout_peric1_speedy2_user",
	     CLK_CON_GAT_CLK_BLK_PERIC1_UID_RSTNSYNC_CLK_PERIC1_SPEEDY2_IPCLKPORT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_AXI2APB_PERIC1P0_ACLK,
	     "gout_peric1_axi2apb_peric1p0_aclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_AXI2APB_PERIC1P0_IPCLKPORT_ACLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIC1_AXI2APB_PERIC1P1_ACLK,
	     "gout_peric1_axi2apb_peric1p1_aclk", "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_AXI2APB_PERIC1P1_IPCLKPORT_ACLK,
	    21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIC1_AXI2APB_PERIC1P2_ACLK,
	     "gout_peric1_axi2apb_peric1p2_aclk", "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_AXI2APB_PERIC1P2_IPCLKPORT_ACLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIC1_GPIO_PERIC1_PCLK,
	     "gout_peric1_gpio_peric1_pclk", "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_GPIO_PERIC1_IPCLKPORT_PCLK,
	     21, CLK_IGNORE_UNUSED, 0),
	GATE(CLK_GOUT_PERIC1_HSI2C_CAM0_IPCLK, "gout_peric1_hsi2c_cam0_ipclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_HSI2C_CAM0_IPCLKPORT_IPCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_HSI2C_CAM1_IPCLK,
	     "gout_peric1_hsi2c_cam1_ipclk", "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_HSI2C_CAM1_IPCLKPORT_IPCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_HSI2C_CAM2_IPCLK,
	     "gout_peric1_hsi2c_cam2_ipclk", "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_HSI2C_CAM2_IPCLKPORT_IPCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_HSI2C_CAM3_IPCLK, "gout_peric1_hsi2c_cam3_ipclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_HSI2C_CAM3_IPCLKPORT_IPCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_LHM_AXI_P_PERIC1_I_CLK,
	     "gout_peric1_lhm_axi_p_peric1_i_clk", "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_LHM_AXI_P_PERIC1_IPCLKPORT_I_CLK,
	     21, CLK_IS_CRITICAL, 0),
	GATE(CLK_GOUT_PERIC1_PMU_PERIC1_PCLK, "gout_peric1_pmu_peric1_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_PMU_PERIC1_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_RSTNSYNC_CLK_PERIC1_BUSP_CLK,
	     "gout_peric1_rstnsync_clk_peric1_busp_clk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_RSTNSYNC_CLK_PERIC1_BUSP_IPCLKPORT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SPEEDY2_DDI1_CLK, "gout_peric1_speedy2_ddi1_clk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI1_IPCLKPORT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SPEEDY2_DDI1_SCLK,
	     "gout_peric1_speedy2_ddi1_sclk", "mout_peric1_speedy2_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI1_IPCLKPORT_SCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SPEEDY2_DDI2_CLK, "gout_peric1_speedy2_ddi2_clk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI2_IPCLKPORT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SPEEDY2_DDI2_SCLK,
	     "gout_peric1_speedy2_ddi2_sclk", "mout_peric1_speedy2_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI2_IPCLKPORT_SCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SPEEDY2_DDI_CLK, "gout_peric1_speedy2_ddi_clk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI_IPCLKPORT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SPEEDY2_DDI_SCLK, "gout_peric1_speedy2_ddi_sclk",
	     "mout_peric1_speedy2_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_DDI_IPCLKPORT_SCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SPEEDY2_TSP1_CLK, "gout_peric1_speedy2_tsp1_clk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_TSP1_IPCLKPORT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SPEEDY2_TSP2_CLK, "gout_peric1_speedy2_tsp2_clk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPEEDY2_TSP2_IPCLKPORT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SPI_CAM0_PCLK, "gout_peric1_spi_cam0_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPI_CAM0_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SPI_CAM0_SPI_EXT_CLK,
	     "gout_peric1_spi_cam0_spi_ext_clk", "mout_peric1_spi_cam0_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPI_CAM0_IPCLKPORT_SPI_EXT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SPI_CAM1_PCLK, "gout_peric1_spi_cam1_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPI_CAM1_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SPI_CAM1_SPI_EXT_CLK,
	     "gout_peric1_spi_cam1_spi_ext_clk", "mout_peric1_spi_cam1_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SPI_CAM1_IPCLKPORT_SPI_EXT_CLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_SYSREG_PERIC1_PCLK,
	     "gout_peric1_sysreg_peric1_pclk", "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_SYSREG_PERIC1_IPCLKPORT_PCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_UART_BT_EXT_UCLK, "gout_peric1_uart_bt_ext_uclk",
	     "mout_peric1_uart_bt_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_UART_BT_IPCLKPORT_EXT_UCLK,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_UART_BT_PCLK, "gout_peric1_uart_bt_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_UART_BT_IPCLKPORT_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI04_I_PCLK, "gout_peric1_usi04_i_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI04_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI04_I_SCLK_USI, "gout_peric1_usi04_i_sclk_usi",
	     "mout_peric1_usi04_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI04_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI05_I_PCLK, "gout_peric1_usi05_i_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI05_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI05_I_SCLK_USI, "gout_peric1_usi05_i_sclk_usi",
	     "mout_peric1_usi05_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI05_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI06_I_PCLK, "gout_peric1_usi06_i_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI06_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI06_I_SCLK_USI, "gout_peric1_usi06_i_sclk_usi",
	     "mout_peric1_usi06_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI06_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI07_I_PCLK, "gout_peric1_usi07_i_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI07_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI07_I_SCLK_USI, "gout_peric1_usi07_i_sclk_usi",
	     "mout_peric1_usi07_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI07_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI08_I_PCLK, "gout_peric1_usi08_i_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI08_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI08_I_SCLK_USI, "gout_peric1_usi08_i_sclk_usi",
	     "mout_peric1_usi08_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI08_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI09_I_PCLK, "gout_peric1_usi09_i_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI09_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI09_I_SCLK_USI, "gout_peric1_usi09_i_sclk_usi",
	     "mout_peric1_usi09_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI09_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI10_I_PCLK, "gout_peric1_usi10_i_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI10_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI10_I_SCLK_USI, "gout_peric1_usi10_i_sclk_usi",
	     "mout_peric1_usi10_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI10_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI11_I_PCLK, "gout_peric1_usi11_i_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI11_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI11_I_SCLK_USI, "gout_peric1_usi11_i_sclk_usi",
	     "mout_peric1_usi11_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI11_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI12_I_PCLK, "gout_peric1_usi12_i_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI12_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI12_I_SCLK_USI, "gout_peric1_usi12_i_sclk_usi",
	     "mout_peric1_usi12_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI12_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI13_I_PCLK, "gout_peric1_usi13_i_pclk",
	     "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI13_IPCLKPORT_I_PCLK, 21, 0, 0),
	GATE(CLK_GOUT_PERIC1_USI13_I_SCLK_USI, "gout_peric1_usi13_i_sclk_usi",
	     "mout_peric1_usi13_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_USI13_IPCLKPORT_I_SCLK_USI,
	     21, 0, 0),
	GATE(CLK_GOUT_PERIC1_XIU_P_PERIC1_ACLK,
	     "gout_peric1_xiu_p_peric1_aclk", "mout_peric1_bus_user",
	     CLK_CON_GAT_GOUT_BLK_PERIC1_UID_XIU_P_PERIC1_IPCLKPORT_ACLK,
	     21, CLK_IGNORE_UNUSED, 0),
];

static peric1_cmu_info: samsung_cmu_info = samsung_cmu_info {
.mux_clks = peric1_mux_clks,
.nr_mux_clks = peric1_mux_clks.len(),
.gate_clks = peric1_gate_clks,
.nr_gate_clks = peric1_gate_clks.len(),
.nr_clk_ids = CLKS_NR_PERIC1,
.clk_regs = peric1_clk_regs,
.nr_clk_regs = peric1_clk_regs.len(),
.clk_name = "bus",
];


#[no_mangle]
unsafe extern "C" fn exynos8895_cmu_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let info = of_device_get_match_data(dev);
    exynos_arm64_register_cmu(dev, (*pdev).of_node, info);
    0
}

static exynos8895_cmu_of_match: &[of_device_id] = &[
    of_device_id { compatible: "samsung,exynos8895-cmu-fsys0", data: &fsys0_cmu_info },
    of_device_id { compatible: "samsung,exynos8895-cmu-fsys1", data: &fsys1_cmu_info },
    of_device_id { compatible: "samsung,exynos8895-cmu-peric0", data: &peric0_cmu_info },
    of_device_id { compatible: "samsung,exynos8895-cmu-peric1", data: &peric1_cmu_info },
    of_device_id { compatible: "", data: core::ptr::null() },
];

static exynos8895_cmu_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: "exynos8895-cmu",
        of_match_table: exynos8895_cmu_of_match,
        suppress_bind_attrs: true,
    },
    probe: exynos8895_cmu_probe,
};

unsafe extern "C" fn exynos8895_cmu_init() -> i32 {
    platform_driver_register(&exynos8895_cmu_driver)
}

// core_initcall(exynos8895_cmu_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
