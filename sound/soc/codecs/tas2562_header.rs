/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tas2562.h - ALSA SoC Texas Instruments TAS2562 Mono Audio Amplifier
 *
 * Copyright (C) 2019 Texas Instruments Incorporated -  https://www.ti.com
 *
 * Author: Dan Murphy <dmurphy@ti.com>
 */

pub const TAS2562_PAGE_CTRL: u32 = 0x00;

pub const fn TAS2562_REG(page: u32, reg: u32) -> u32 {
    (page * 128) + reg
}

pub const TAS2562_SW_RESET: u32 = TAS2562_REG(0, 0x01);
pub const TAS2562_PWR_CTRL: u32 = TAS2562_REG(0, 0x02);
pub const TAS2562_PB_CFG1: u32 = TAS2562_REG(0, 0x03);
pub const TAS2562_MISC_CFG1: u32 = TAS2562_REG(0, 0x04);
pub const TAS2562_MISC_CFG2: u32 = TAS2562_REG(0, 0x05);

pub const TAS2562_TDM_CFG0: u32 = TAS2562_REG(0, 0x06);
pub const TAS2562_TDM_CFG1: u32 = TAS2562_REG(0, 0x07);
pub const TAS2562_TDM_CFG2: u32 = TAS2562_REG(0, 0x08);
pub const TAS2562_TDM_CFG3: u32 = TAS2562_REG(0, 0x09);
pub const TAS2562_TDM_CFG4: u32 = TAS2562_REG(0, 0x0a);
pub const TAS2562_TDM_CFG5: u32 = TAS2562_REG(0, 0x0b);
pub const TAS2562_TDM_CFG6: u32 = TAS2562_REG(0, 0x0c);
pub const TAS2562_TDM_CFG7: u32 = TAS2562_REG(0, 0x0d);
pub const TAS2562_TDM_CFG8: u32 = TAS2562_REG(0, 0x0e);
pub const TAS2562_TDM_CFG9: u32 = TAS2562_REG(0, 0x0f);
pub const TAS2562_TDM_CFG10: u32 = TAS2562_REG(0, 0x10);
pub const TAS2562_TDM_DET: u32 = TAS2562_REG(0, 0x11);
pub const TAS2562_REV_ID: u32 = TAS2562_REG(0, 0x7d);

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

pub const TAS2562_RX_OFF_MASK: u32 = GENMASK(5, 1);
pub const TAS2562_TX_OFF_MASK: u32 = GENMASK(3, 1);
pub const TAS2562_RIGHT_SLOT_SHIFT: u32 = 4;

/* Page 2 */
pub const TAS2562_DVC_CFG1: u32 = TAS2562_REG(2, 0x0c);
pub const TAS2562_DVC_CFG2: u32 = TAS2562_REG(2, 0x0d);
pub const TAS2562_DVC_CFG3: u32 = TAS2562_REG(2, 0x0e);
pub const TAS2562_DVC_CFG4: u32 = TAS2562_REG(2, 0x0f);

pub const TAS2562_RESET: u32 = BIT(0);

pub const TAS2562_MODE_MASK: u32 = GENMASK(1, 0);
pub const TAS2562_ACTIVE: u32 = 0x0;
pub const TAS2562_MUTE: u32 = 0x1;
pub const TAS2562_SHUTDOWN: u32 = 0x2;

pub const TAS2562_TDM_CFG1_RX_EDGE_MASK: u32 = BIT(0);
pub const TAS2562_TDM_CFG1_RX_FALLING: u32 = 1;

pub const TAS2562_TDM_CFG0_RAMPRATE_MASK: u32 = BIT(5);
pub const TAS2562_TDM_CFG0_RAMPRATE_44_1: u32 = BIT(5);
pub const TAS2562_TDM_CFG0_SAMPRATE_MASK: u32 = GENMASK(3, 1);
pub const TAS2562_TDM_CFG0_SAMPRATE_7305_8KHZ: u32 = 0x0 << 1;
pub const TAS2562_TDM_CFG0_SAMPRATE_14_7_16KHZ: u32 = 0x1 << 1;
pub const TAS2562_TDM_CFG0_SAMPRATE_22_05_24KHZ: u32 = 0x2 << 1;
pub const TAS2562_TDM_CFG0_SAMPRATE_29_4_32KHZ: u32 = 0x3 << 1;
pub const TAS2562_TDM_CFG0_SAMPRATE_44_1_48KHZ: u32 = 0x4 << 1;
pub const TAS2562_TDM_CFG0_SAMPRATE_88_2_96KHZ: u32 = 0x5 << 1;
pub const TAS2562_TDM_CFG0_SAMPRATE_176_4_192KHZ: u32 = 0x6 << 1;

pub const TAS2562_TDM_CFG2_RIGHT_JUSTIFY: u32 = BIT(6);

pub const TAS2562_TDM_CFG2_RXLEN_MASK: u32 = GENMASK(1, 0);
pub const TAS2562_TDM_CFG2_RXLEN_16B: u32 = 0x0;
pub const TAS2562_TDM_CFG2_RXLEN_24B: u32 = BIT(0);
pub const TAS2562_TDM_CFG2_RXLEN_32B: u32 = BIT(1);

pub const TAS2562_TDM_CFG2_RXWLEN_MASK: u32 = GENMASK(3, 2);
pub const TAS2562_TDM_CFG2_RXWLEN_16B: u32 = 0x0;
pub const TAS2562_TDM_CFG2_RXWLEN_20B: u32 = BIT(2);
pub const TAS2562_TDM_CFG2_RXWLEN_24B: u32 = BIT(3);
pub const TAS2562_TDM_CFG2_RXWLEN_32B: u32 = BIT(2) | BIT(3);

pub const TAS2562_VSENSE_POWER_EN: u32 = 2;
pub const TAS2562_ISENSE_POWER_EN: u32 = 3;

pub const TAS2562_TDM_CFG5_VSNS_EN: u32 = BIT(6);
pub const TAS2562_TDM_CFG5_VSNS_SLOT_MASK: u32 = GENMASK(5, 0);

pub const TAS2562_TDM_CFG6_ISNS_EN: u32 = BIT(6);
pub const TAS2562_TDM_CFG6_ISNS_SLOT_MASK: u32 = GENMASK(5, 0);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
