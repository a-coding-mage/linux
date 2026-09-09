/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Freescale Semiconductor, Inc.
 */

// Translated from __LINUX_IMX7_IOMUXC_GPR_H.

pub const IOMUXC_GPR0: u32 = 0x00;
pub const IOMUXC_GPR1: u32 = 0x04;
pub const IOMUXC_GPR2: u32 = 0x08;
pub const IOMUXC_GPR3: u32 = 0x0c;
pub const IOMUXC_GPR4: u32 = 0x10;
pub const IOMUXC_GPR5: u32 = 0x14;
pub const IOMUXC_GPR6: u32 = 0x18;
pub const IOMUXC_GPR7: u32 = 0x1c;
pub const IOMUXC_GPR8: u32 = 0x20;
pub const IOMUXC_GPR9: u32 = 0x24;
pub const IOMUXC_GPR10: u32 = 0x28;
pub const IOMUXC_GPR11: u32 = 0x2c;
pub const IOMUXC_GPR12: u32 = 0x30;
pub const IOMUXC_GPR13: u32 = 0x34;
pub const IOMUXC_GPR14: u32 = 0x38;
pub const IOMUXC_GPR15: u32 = 0x3c;
pub const IOMUXC_GPR16: u32 = 0x40;
pub const IOMUXC_GPR17: u32 = 0x44;
pub const IOMUXC_GPR18: u32 = 0x48;
pub const IOMUXC_GPR19: u32 = 0x4c;
pub const IOMUXC_GPR20: u32 = 0x50;
pub const IOMUXC_GPR21: u32 = 0x54;
pub const IOMUXC_GPR22: u32 = 0x58;

/* For imx7d iomux gpr register field define */
pub const IMX7D_GPR1_IRQ_MASK: u32 = 0x1 << 12;
pub const IMX7D_GPR1_ENET1_TX_CLK_SEL_MASK: u32 = 0x1 << 13;
pub const IMX7D_GPR1_ENET2_TX_CLK_SEL_MASK: u32 = 0x1 << 14;
pub const IMX7D_GPR1_ENET_TX_CLK_SEL_MASK: u32 = 0x3 << 13;
pub const IMX7D_GPR1_ENET1_CLK_DIR_MASK: u32 = 0x1 << 17;
pub const IMX7D_GPR1_ENET2_CLK_DIR_MASK: u32 = 0x1 << 18;
pub const IMX7D_GPR1_ENET_CLK_DIR_MASK: u32 = 0x3 << 17;

pub const IMX7D_GPR5_CSI_MUX_CONTROL_MIPI: u32 = 0x1 << 4;

// BIT is supplied by the surrounding kernel translation/dependencies.
pub const IMX7D_GPR12_PCIE_PHY_REFCLK_SEL: u32 = BIT(5);

// BIT is supplied by the surrounding kernel translation/dependencies.
pub const IMX7D_GPR22_PCIE_PHY_PLL_LOCKED: u32 = BIT(31);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
