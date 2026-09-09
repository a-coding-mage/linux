/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2017 Impinj, Inc.
 *
 * Author: Andrey Smirnov <andrew.smirnov@gmail.com>
 */

pub const IMX7_RESET_A7_CORE_POR_RESET0: i32 = 0;
pub const IMX7_RESET_A7_CORE_POR_RESET1: i32 = 1;
pub const IMX7_RESET_A7_CORE_RESET0: i32 = 2;
pub const IMX7_RESET_A7_CORE_RESET1: i32 = 3;
pub const IMX7_RESET_A7_DBG_RESET0: i32 = 4;
pub const IMX7_RESET_A7_DBG_RESET1: i32 = 5;
pub const IMX7_RESET_A7_ETM_RESET0: i32 = 6;
pub const IMX7_RESET_A7_ETM_RESET1: i32 = 7;
pub const IMX7_RESET_A7_SOC_DBG_RESET: i32 = 8;
pub const IMX7_RESET_A7_L2RESET: i32 = 9;
pub const IMX7_RESET_SW_M4C_RST: i32 = 10;
pub const IMX7_RESET_SW_M4P_RST: i32 = 11;
pub const IMX7_RESET_EIM_RST: i32 = 12;
pub const IMX7_RESET_HSICPHY_PORT_RST: i32 = 13;
pub const IMX7_RESET_USBPHY1_POR: i32 = 14;
pub const IMX7_RESET_USBPHY1_PORT_RST: i32 = 15;
pub const IMX7_RESET_USBPHY2_POR: i32 = 16;
pub const IMX7_RESET_USBPHY2_PORT_RST: i32 = 17;
pub const IMX7_RESET_MIPI_PHY_MRST: i32 = 18;
pub const IMX7_RESET_MIPI_PHY_SRST: i32 = 19;

/*
 * IMX7_RESET_PCIEPHY is a logical reset line combining PCIEPHY_BTN
 * and PCIEPHY_G_RST
 */
pub const IMX7_RESET_PCIEPHY: i32 = 20;
pub const IMX7_RESET_PCIEPHY_PERST: i32 = 21;

/*
 * IMX7_RESET_PCIE_CTRL_APPS_EN is not strictly a reset line, but it
 * can be used to inhibit PCIe LTTSM, so, in a way, it can be thoguht
 * of as one
 */
pub const IMX7_RESET_PCIE_CTRL_APPS_EN: i32 = 22;
pub const IMX7_RESET_DDRC_PRST: i32 = 23;
pub const IMX7_RESET_DDRC_CORE_RST: i32 = 24;

pub const IMX7_RESET_PCIE_CTRL_APPS_TURNOFF: i32 = 25;

pub const IMX7_RESET_NUM: i32 = 26;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
