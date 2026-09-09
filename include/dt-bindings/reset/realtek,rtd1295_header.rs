/* SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause) */
/*
 * Realtek RTD1295 reset controllers
 *
 * Copyright (c) 2017 Andreas Färber
 */

/* soft reset 1 */
pub const RTD1295_RSTN_MISC: u32 = 0;
pub const RTD1295_RSTN_NAT: u32 = 1;
pub const RTD1295_RSTN_USB3_PHY0_POW: u32 = 2;
pub const RTD1295_RSTN_GSPI: u32 = 3;
pub const RTD1295_RSTN_USB3_P0_MDIO: u32 = 4;
pub const RTD1295_RSTN_SATA_0: u32 = 5;
pub const RTD1295_RSTN_USB: u32 = 6;
pub const RTD1295_RSTN_SATA_PHY_0: u32 = 7;
pub const RTD1295_RSTN_USB_PHY0: u32 = 8;
pub const RTD1295_RSTN_USB_PHY1: u32 = 9;
pub const RTD1295_RSTN_SATA_PHY_POW_0: u32 = 10;
pub const RTD1295_RSTN_SATA_FUNC_EXIST_0: u32 = 11;
pub const RTD1295_RSTN_HDMI: u32 = 12;
pub const RTD1295_RSTN_VE1: u32 = 13;
pub const RTD1295_RSTN_VE2: u32 = 14;
pub const RTD1295_RSTN_VE3: u32 = 15;
pub const RTD1295_RSTN_ETN: u32 = 16;
pub const RTD1295_RSTN_AIO: u32 = 17;
pub const RTD1295_RSTN_GPU: u32 = 18;
pub const RTD1295_RSTN_TVE: u32 = 19;
pub const RTD1295_RSTN_VO: u32 = 20;
pub const RTD1295_RSTN_LVDS: u32 = 21;
pub const RTD1295_RSTN_SE: u32 = 22;
pub const RTD1295_RSTN_DCU: u32 = 23;
pub const RTD1295_RSTN_DC_PHY: u32 = 24;
pub const RTD1295_RSTN_CP: u32 = 25;
pub const RTD1295_RSTN_MD: u32 = 26;
pub const RTD1295_RSTN_TP: u32 = 27;
pub const RTD1295_RSTN_AE: u32 = 28;
pub const RTD1295_RSTN_NF: u32 = 29;
pub const RTD1295_RSTN_MIPI: u32 = 30;
pub const RTD1295_RSTN_RSA: u32 = 31;

/* soft reset 2 */
pub const RTD1295_RSTN_ACPU: u32 = 0;
pub const RTD1295_RSTN_JPEG: u32 = 1;
pub const RTD1295_RSTN_USB_PHY3: u32 = 2;
pub const RTD1295_RSTN_USB_PHY2: u32 = 3;
pub const RTD1295_RSTN_USB3_PHY1_POW: u32 = 4;
pub const RTD1295_RSTN_USB3_P1_MDIO: u32 = 5;
pub const RTD1295_RSTN_PCIE0_STITCH: u32 = 6;
pub const RTD1295_RSTN_PCIE0_PHY: u32 = 7;
pub const RTD1295_RSTN_PCIE0: u32 = 8;
pub const RTD1295_RSTN_PCR_CNT: u32 = 9;
pub const RTD1295_RSTN_CR: u32 = 10;
pub const RTD1295_RSTN_EMMC: u32 = 11;
pub const RTD1295_RSTN_SDIO: u32 = 12;
pub const RTD1295_RSTN_PCIE0_CORE: u32 = 13;
pub const RTD1295_RSTN_PCIE0_POWER: u32 = 14;
pub const RTD1295_RSTN_PCIE0_NONSTICH: u32 = 15;
pub const RTD1295_RSTN_PCIE1_PHY: u32 = 16;
pub const RTD1295_RSTN_PCIE1: u32 = 17;
pub const RTD1295_RSTN_I2C_5: u32 = 18;
pub const RTD1295_RSTN_PCIE1_STITCH: u32 = 19;
pub const RTD1295_RSTN_PCIE1_CORE: u32 = 20;
pub const RTD1295_RSTN_PCIE1_POWER: u32 = 21;
pub const RTD1295_RSTN_PCIE1_NONSTICH: u32 = 22;
pub const RTD1295_RSTN_I2C_4: u32 = 23;
pub const RTD1295_RSTN_I2C_3: u32 = 24;
pub const RTD1295_RSTN_I2C_2: u32 = 25;
pub const RTD1295_RSTN_I2C_1: u32 = 26;
pub const RTD1295_RSTN_UR2: u32 = 27;
pub const RTD1295_RSTN_UR1: u32 = 28;
pub const RTD1295_RSTN_MISC_SC: u32 = 29;
pub const RTD1295_RSTN_CBUS_TX: u32 = 30;
pub const RTD1295_RSTN_SDS_PHY: u32 = 31;

/* soft reset 3 */
pub const RTD1295_RSTN_SB2: u32 = 0;

/* soft reset 4 */
pub const RTD1295_RSTN_DCPHY_CRT: u32 = 0;
pub const RTD1295_RSTN_DCPHY_ALERT_RX: u32 = 1;
pub const RTD1295_RSTN_DCPHY_PTR: u32 = 2;
pub const RTD1295_RSTN_DCPHY_LDO: u32 = 3;
pub const RTD1295_RSTN_DCPHY_SSC_DIG: u32 = 4;
pub const RTD1295_RSTN_HDMIRX: u32 = 5;
pub const RTD1295_RSTN_CBUSRX: u32 = 6;
pub const RTD1295_RSTN_SATA_PHY_POW_1: u32 = 7;
pub const RTD1295_RSTN_SATA_FUNC_EXIST_1: u32 = 8;
pub const RTD1295_RSTN_SATA_PHY_1: u32 = 9;
pub const RTD1295_RSTN_SATA_1: u32 = 10;
pub const RTD1295_RSTN_FAN: u32 = 11;
pub const RTD1295_RSTN_HDMIRX_WRAP: u32 = 12;
pub const RTD1295_RSTN_PCIE0_PHY_MDIO: u32 = 13;
pub const RTD1295_RSTN_PCIE1_PHY_MDIO: u32 = 14;
pub const RTD1295_RSTN_DISP: u32 = 15;

/* iso reset */
pub const RTD1295_ISO_RSTN_IR: u32 = 1;
pub const RTD1295_ISO_RSTN_CEC0: u32 = 2;
pub const RTD1295_ISO_RSTN_CEC1: u32 = 3;
pub const RTD1295_ISO_RSTN_DP: u32 = 4;
pub const RTD1295_ISO_RSTN_CBUSTX: u32 = 5;
pub const RTD1295_ISO_RSTN_CBUSRX: u32 = 6;
pub const RTD1295_ISO_RSTN_EFUSE: u32 = 7;
pub const RTD1295_ISO_RSTN_UR0: u32 = 8;
pub const RTD1295_ISO_RSTN_GMAC: u32 = 9;
pub const RTD1295_ISO_RSTN_GPHY: u32 = 10;
pub const RTD1295_ISO_RSTN_I2C_0: u32 = 11;
pub const RTD1295_ISO_RSTN_I2C_1: u32 = 12;
pub const RTD1295_ISO_RSTN_CBUS: u32 = 13;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
