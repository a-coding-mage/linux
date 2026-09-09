/* SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause) */
/*
 * Realtek RTD1195 reset controllers
 *
 * Copyright (c) 2017 Andreas Färber
 */

/* soft reset 1 */
pub mod soft_reset_1 {
    pub const RTD1195_RSTN_MISC: u32 = 0;
    pub const RTD1195_RSTN_RNG: u32 = 1;
    pub const RTD1195_RSTN_USB3_POW: u32 = 2;
    pub const RTD1195_RSTN_GSPI: u32 = 3;
    pub const RTD1195_RSTN_USB3_P0_MDIO: u32 = 4;
    pub const RTD1195_RSTN_VE_H265: u32 = 5;
    pub const RTD1195_RSTN_USB: u32 = 6;
    pub const RTD1195_RSTN_USB_PHY0: u32 = 8;
    pub const RTD1195_RSTN_USB_PHY1: u32 = 9;
    pub const RTD1195_RSTN_HDMIRX: u32 = 11;
    pub const RTD1195_RSTN_HDMI: u32 = 12;
    pub const RTD1195_RSTN_ETN: u32 = 14;
    pub const RTD1195_RSTN_AIO: u32 = 15;
    pub const RTD1195_RSTN_GPU: u32 = 16;
    pub const RTD1195_RSTN_VE_H264: u32 = 17;
    pub const RTD1195_RSTN_VE_JPEG: u32 = 18;
    pub const RTD1195_RSTN_TVE: u32 = 19;
    pub const RTD1195_RSTN_VO: u32 = 20;
    pub const RTD1195_RSTN_LVDS: u32 = 21;
    pub const RTD1195_RSTN_SE: u32 = 22;
    pub const RTD1195_RSTN_DCU: u32 = 23;
    pub const RTD1195_RSTN_DC_PHY: u32 = 24;
    pub const RTD1195_RSTN_CP: u32 = 25;
    pub const RTD1195_RSTN_MD: u32 = 26;
    pub const RTD1195_RSTN_TP: u32 = 27;
    pub const RTD1195_RSTN_AE: u32 = 28;
    pub const RTD1195_RSTN_NF: u32 = 29;
    pub const RTD1195_RSTN_MIPI: u32 = 30;
}

/* soft reset 2 */
pub mod soft_reset_2 {
    pub const RTD1195_RSTN_ACPU: u32 = 0;
    pub const RTD1195_RSTN_VCPU: u32 = 1;
    pub const RTD1195_RSTN_PCR: u32 = 9;
    pub const RTD1195_RSTN_CR: u32 = 10;
    pub const RTD1195_RSTN_EMMC: u32 = 11;
    pub const RTD1195_RSTN_SDIO: u32 = 12;
    pub const RTD1195_RSTN_I2C_5: u32 = 18;
    pub const RTD1195_RSTN_RTC: u32 = 20;
    pub const RTD1195_RSTN_I2C_4: u32 = 23;
    pub const RTD1195_RSTN_I2C_3: u32 = 24;
    pub const RTD1195_RSTN_I2C_2: u32 = 25;
    pub const RTD1195_RSTN_I2C_1: u32 = 26;
    pub const RTD1195_RSTN_UR1: u32 = 28;
}

/* soft reset 3 */
pub mod soft_reset_3 {
    pub const RTD1195_RSTN_SB2: u32 = 0;
}

/* iso soft reset */
pub mod iso_soft_reset {
    pub const RTD1195_ISO_RSTN_VFD: u32 = 0;
    pub const RTD1195_ISO_RSTN_IR: u32 = 1;
    pub const RTD1195_ISO_RSTN_CEC0: u32 = 2;
    pub const RTD1195_ISO_RSTN_CEC1: u32 = 3;
    pub const RTD1195_ISO_RSTN_DP: u32 = 4;
    pub const RTD1195_ISO_RSTN_CBUSTX: u32 = 5;
    pub const RTD1195_ISO_RSTN_CBUSRX: u32 = 6;
    pub const RTD1195_ISO_RSTN_EFUSE: u32 = 7;
    pub const RTD1195_ISO_RSTN_UR0: u32 = 8;
    pub const RTD1195_ISO_RSTN_GMAC: u32 = 9;
    pub const RTD1195_ISO_RSTN_GPHY: u32 = 10;
    pub const RTD1195_ISO_RSTN_I2C_0: u32 = 11;
    pub const RTD1195_ISO_RSTN_I2C_6: u32 = 12;
    pub const RTD1195_ISO_RSTN_CBUS: u32 = 13;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
