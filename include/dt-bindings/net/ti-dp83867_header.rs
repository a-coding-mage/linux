/* SPDX-License-Identifier: GPL-2.0-only OR MIT */
/*
 * Device Tree constants for the Texas Instruments DP83867 PHY
 *
 * Author: Dan Murphy <dmurphy@ti.com>
 *
 * Copyright (C) 2015-2024 Texas Instruments Incorporated - https://www.ti.com/
 */

/* PHY CTRL bits */
pub const DP83867_PHYCR_FIFO_DEPTH_3_B_NIB: u32 = 0x00;
pub const DP83867_PHYCR_FIFO_DEPTH_4_B_NIB: u32 = 0x01;
pub const DP83867_PHYCR_FIFO_DEPTH_6_B_NIB: u32 = 0x02;
pub const DP83867_PHYCR_FIFO_DEPTH_8_B_NIB: u32 = 0x03;

/* RGMIIDCTL internal delay for rx and tx */
pub const DP83867_RGMIIDCTL_250_PS: u32 = 0x0;
pub const DP83867_RGMIIDCTL_500_PS: u32 = 0x1;
pub const DP83867_RGMIIDCTL_750_PS: u32 = 0x2;
pub const DP83867_RGMIIDCTL_1_NS: u32 = 0x3;
pub const DP83867_RGMIIDCTL_1_25_NS: u32 = 0x4;
pub const DP83867_RGMIIDCTL_1_50_NS: u32 = 0x5;
pub const DP83867_RGMIIDCTL_1_75_NS: u32 = 0x6;
pub const DP83867_RGMIIDCTL_2_00_NS: u32 = 0x7;
pub const DP83867_RGMIIDCTL_2_25_NS: u32 = 0x8;
pub const DP83867_RGMIIDCTL_2_50_NS: u32 = 0x9;
pub const DP83867_RGMIIDCTL_2_75_NS: u32 = 0xa;
pub const DP83867_RGMIIDCTL_3_00_NS: u32 = 0xb;
pub const DP83867_RGMIIDCTL_3_25_NS: u32 = 0xc;
pub const DP83867_RGMIIDCTL_3_50_NS: u32 = 0xd;
pub const DP83867_RGMIIDCTL_3_75_NS: u32 = 0xe;
pub const DP83867_RGMIIDCTL_4_00_NS: u32 = 0xf;

/* IO_MUX_CFG - Clock output selection */
pub const DP83867_CLK_O_SEL_CHN_A_RCLK: u32 = 0x0;
pub const DP83867_CLK_O_SEL_CHN_B_RCLK: u32 = 0x1;
pub const DP83867_CLK_O_SEL_CHN_C_RCLK: u32 = 0x2;
pub const DP83867_CLK_O_SEL_CHN_D_RCLK: u32 = 0x3;
pub const DP83867_CLK_O_SEL_CHN_A_RCLK_DIV5: u32 = 0x4;
pub const DP83867_CLK_O_SEL_CHN_B_RCLK_DIV5: u32 = 0x5;
pub const DP83867_CLK_O_SEL_CHN_C_RCLK_DIV5: u32 = 0x6;
pub const DP83867_CLK_O_SEL_CHN_D_RCLK_DIV5: u32 = 0x7;
pub const DP83867_CLK_O_SEL_CHN_A_TCLK: u32 = 0x8;
pub const DP83867_CLK_O_SEL_CHN_B_TCLK: u32 = 0x9;
pub const DP83867_CLK_O_SEL_CHN_C_TCLK: u32 = 0xA;
pub const DP83867_CLK_O_SEL_CHN_D_TCLK: u32 = 0xB;
pub const DP83867_CLK_O_SEL_REF_CLK: u32 = 0xC;
/* Special flag to indicate clock should be off */
pub const DP83867_CLK_O_SEL_OFF: u32 = 0xFFFFFFFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
