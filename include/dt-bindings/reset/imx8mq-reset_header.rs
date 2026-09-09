/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 Zodiac Inflight Innovations
 *
 * Author: Andrey Smirnov <andrew.smirnov@gmail.com>
 */

pub const IMX8MQ_RESET_A53_CORE_POR_RESET0: u32 = 0;
pub const IMX8MQ_RESET_A53_CORE_POR_RESET1: u32 = 1;
pub const IMX8MQ_RESET_A53_CORE_POR_RESET2: u32 = 2;
pub const IMX8MQ_RESET_A53_CORE_POR_RESET3: u32 = 3;
pub const IMX8MQ_RESET_A53_CORE_RESET0: u32 = 4;
pub const IMX8MQ_RESET_A53_CORE_RESET1: u32 = 5;
pub const IMX8MQ_RESET_A53_CORE_RESET2: u32 = 6;
pub const IMX8MQ_RESET_A53_CORE_RESET3: u32 = 7;
pub const IMX8MQ_RESET_A53_DBG_RESET0: u32 = 8;
pub const IMX8MQ_RESET_A53_DBG_RESET1: u32 = 9;
pub const IMX8MQ_RESET_A53_DBG_RESET2: u32 = 10;
pub const IMX8MQ_RESET_A53_DBG_RESET3: u32 = 11;
pub const IMX8MQ_RESET_A53_ETM_RESET0: u32 = 12;
pub const IMX8MQ_RESET_A53_ETM_RESET1: u32 = 13;
pub const IMX8MQ_RESET_A53_ETM_RESET2: u32 = 14;
pub const IMX8MQ_RESET_A53_ETM_RESET3: u32 = 15;
pub const IMX8MQ_RESET_A53_SOC_DBG_RESET: u32 = 16;
pub const IMX8MQ_RESET_A53_L2RESET: u32 = 17;
pub const IMX8MQ_RESET_SW_NON_SCLR_M4C_RST: u32 = 18;
pub const IMX8MQ_RESET_OTG1_PHY_RESET: u32 = 19;
pub const IMX8MQ_RESET_OTG2_PHY_RESET: u32 = 20; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_MIPI_DSI_RESET_BYTE_N: u32 = 21; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_MIPI_DSI_RESET_N: u32 = 22; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_MIPI_DSI_DPI_RESET_N: u32 = 23; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_MIPI_DSI_ESC_RESET_N: u32 = 24; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_MIPI_DSI_PCLK_RESET_N: u32 = 25; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_PCIEPHY: u32 = 26; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_PCIEPHY_PERST: u32 = 27; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_PCIE_CTRL_APPS_EN: u32 = 28; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_PCIE_CTRL_APPS_TURNOFF: u32 = 29; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_HDMI_PHY_APB_RESET: u32 = 30; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_DISP_RESET: u32 = 31;
pub const IMX8MQ_RESET_GPU_RESET: u32 = 32;
pub const IMX8MQ_RESET_VPU_RESET: u32 = 33; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_PCIEPHY2: u32 = 34; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_PCIEPHY2_PERST: u32 = 35; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_PCIE2_CTRL_APPS_EN: u32 = 36; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_PCIE2_CTRL_APPS_TURNOFF: u32 = 37; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_MIPI_CSI1_CORE_RESET: u32 = 38; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_MIPI_CSI1_PHY_REF_RESET: u32 = 39; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_MIPI_CSI1_ESC_RESET: u32 = 40; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_MIPI_CSI2_CORE_RESET: u32 = 41; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_MIPI_CSI2_PHY_REF_RESET: u32 = 42; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_MIPI_CSI2_ESC_RESET: u32 = 43; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_DDRC1_PRST: u32 = 44; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_DDRC1_CORE_RESET: u32 = 45; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_DDRC1_PHY_RESET: u32 = 46; /* i.MX8MN does NOT support */
pub const IMX8MQ_RESET_DDRC2_PRST: u32 = 47; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_DDRC2_CORE_RESET: u32 = 48; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_DDRC2_PHY_RESET: u32 = 49; /* i.MX8MM/i.MX8MN does NOT support */
pub const IMX8MQ_RESET_SW_M4C_RST: u32 = 50;
pub const IMX8MQ_RESET_SW_M4P_RST: u32 = 51;
pub const IMX8MQ_RESET_M4_ENABLE: u32 = 52;

pub const IMX8MQ_RESET_NUM: u32 = 53;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
