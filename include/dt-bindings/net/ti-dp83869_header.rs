/* SPDX-License-Identifier: GPL-2.0-only OR MIT */
/*
 * Device Tree constants for the Texas Instruments DP83869 PHY
 *
 * Author: Dan Murphy <dmurphy@ti.com>
 *
 * Copyright (C) 2015-2024 Texas Instruments Incorporated - https://www.ti.com/
 */

/* PHY CTRL bits */
pub const DP83869_PHYCR_FIFO_DEPTH_3_B_NIB: u32 = 0x00;
pub const DP83869_PHYCR_FIFO_DEPTH_4_B_NIB: u32 = 0x01;
pub const DP83869_PHYCR_FIFO_DEPTH_6_B_NIB: u32 = 0x02;
pub const DP83869_PHYCR_FIFO_DEPTH_8_B_NIB: u32 = 0x03;

/* IO_MUX_CFG - Clock output selection */
pub const DP83869_CLK_O_SEL_CHN_A_RCLK: u32 = 0x0;
pub const DP83869_CLK_O_SEL_CHN_B_RCLK: u32 = 0x1;
pub const DP83869_CLK_O_SEL_CHN_C_RCLK: u32 = 0x2;
pub const DP83869_CLK_O_SEL_CHN_D_RCLK: u32 = 0x3;
pub const DP83869_CLK_O_SEL_CHN_A_RCLK_DIV5: u32 = 0x4;
pub const DP83869_CLK_O_SEL_CHN_B_RCLK_DIV5: u32 = 0x5;
pub const DP83869_CLK_O_SEL_CHN_C_RCLK_DIV5: u32 = 0x6;
pub const DP83869_CLK_O_SEL_CHN_D_RCLK_DIV5: u32 = 0x7;
pub const DP83869_CLK_O_SEL_CHN_A_TCLK: u32 = 0x8;
pub const DP83869_CLK_O_SEL_CHN_B_TCLK: u32 = 0x9;
pub const DP83869_CLK_O_SEL_CHN_C_TCLK: u32 = 0xa;
pub const DP83869_CLK_O_SEL_CHN_D_TCLK: u32 = 0xb;
pub const DP83869_CLK_O_SEL_REF_CLK: u32 = 0xc;

pub const DP83869_RGMII_COPPER_ETHERNET: u32 = 0x00;
pub const DP83869_RGMII_1000_BASE: u32 = 0x01;
pub const DP83869_RGMII_100_BASE: u32 = 0x02;
pub const DP83869_RGMII_SGMII_BRIDGE: u32 = 0x03;
pub const DP83869_1000M_MEDIA_CONVERT: u32 = 0x04;
pub const DP83869_100M_MEDIA_CONVERT: u32 = 0x05;
pub const DP83869_SGMII_COPPER_ETHERNET: u32 = 0x06;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
