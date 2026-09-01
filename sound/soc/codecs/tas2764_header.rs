/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tas2764.h - ALSA SoC Texas Instruments TAS2764 Mono Audio Amplifier
 *
 * Copyright (C) 2020 Texas Instruments Incorporated -  https://www.ti.com
 *
 * Author: Dan Murphy <dmurphy@ti.com>
 */

// Header guard __TAS2764__ omitted in Rust.

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

/* Book Control Register */
pub const TAS2764_BOOKCTL_PAGE: u32 = 0;
pub const TAS2764_BOOKCTL_REG: u32 = 127;
pub const fn TAS2764_REG(page: u32, reg: u32) -> u32 {
    (page * 128) + reg
}

/* Page */
pub const TAS2764_PAGE: u32 = TAS2764_REG(0X0, 0x00);
pub const TAS2764_PAGE_PAGE_MASK: u32 = 255;

/* Software Reset */
pub const TAS2764_SW_RST: u32 = TAS2764_REG(0X0, 0x01);
pub const TAS2764_RST: u32 = BIT(0);

/* Power Control */
pub const TAS2764_PWR_CTRL: u32 = TAS2764_REG(0X0, 0x02);
pub const TAS2764_PWR_CTRL_MASK: u32 = GENMASK(2, 0);
pub const TAS2764_PWR_CTRL_ACTIVE: u32 = 0x0;
pub const TAS2764_PWR_CTRL_MUTE: u32 = BIT(0);
pub const TAS2764_PWR_CTRL_SHUTDOWN: u32 = BIT(1);
pub const TAS2764_PWR_CTRL_BOP_SRC: u32 = BIT(7);

pub const TAS2764_VSENSE_POWER_EN: u32 = 3;
pub const TAS2764_ISENSE_POWER_EN: u32 = 4;

/* DC Blocker Control */
pub const TAS2764_DC_BLK0: u32 = TAS2764_REG(0x0, 0x04);
pub const TAS2764_DC_BLK0_HPF_FREQ_PB_SHIFT: u32 = 0;

/* Digital Volume Control */
pub const TAS2764_DVC: u32 = TAS2764_REG(0X0, 0x1a);
pub const TAS2764_DVC_MAX: u32 = 0xc9;

pub const TAS2764_CHNL_0: u32 = TAS2764_REG(0X0, 0x03);

/* Miscellaneous */
pub const TAS2764_MISC_CFG1: u32 = TAS2764_REG(0x0, 0x06);
pub const TAS2764_MISC_CFG1_OCE_RETRY_SHIFT: u32 = 5;

/* TDM Configuration Reg0 */
pub const TAS2764_TDM_CFG0: u32 = TAS2764_REG(0X0, 0x08);
pub const TAS2764_TDM_CFG0_SMP_MASK: u32 = BIT(5);
pub const TAS2764_TDM_CFG0_SMP_48KHZ: u32 = 0x0;
pub const TAS2764_TDM_CFG0_SMP_44_1KHZ: u32 = BIT(5);
pub const TAS2764_TDM_CFG0_MASK: u32 = GENMASK(3, 1);
pub const TAS2764_TDM_CFG0_44_1_48KHZ: u32 = BIT(3);
pub const TAS2764_TDM_CFG0_88_2_96KHZ: u32 = BIT(3) | BIT(1);
pub const TAS2764_TDM_CFG0_FRAME_START: u32 = BIT(0);

/* TDM Configuration Reg1 */
pub const TAS2764_TDM_CFG1: u32 = TAS2764_REG(0X0, 0x09);
pub const TAS2764_TDM_CFG1_MASK: u32 = GENMASK(5, 1);
pub const TAS2764_TDM_CFG1_51_SHIFT: u32 = 1;
pub const TAS2764_TDM_CFG1_RX_MASK: u32 = BIT(0);
pub const TAS2764_TDM_CFG1_RX_RISING: u32 = 0x0;
pub const TAS2764_TDM_CFG1_RX_FALLING: u32 = BIT(0);

/* TDM Configuration Reg2 */
pub const TAS2764_TDM_CFG2: u32 = TAS2764_REG(0X0, 0x0a);
pub const TAS2764_TDM_CFG2_RXW_MASK: u32 = GENMASK(3, 2);
pub const TAS2764_TDM_CFG2_RXW_16BITS: u32 = 0x0;
pub const TAS2764_TDM_CFG2_RXW_24BITS: u32 = BIT(3);
pub const TAS2764_TDM_CFG2_RXW_32BITS: u32 = BIT(3) | BIT(2);
pub const TAS2764_TDM_CFG2_RXS_MASK: u32 = GENMASK(1, 0);
pub const TAS2764_TDM_CFG2_RXS_16BITS: u32 = 0x0;
pub const TAS2764_TDM_CFG2_RXS_24BITS: u32 = BIT(0);
pub const TAS2764_TDM_CFG2_RXS_32BITS: u32 = BIT(1);
pub const TAS2764_TDM_CFG2_SCFG_SHIFT: u32 = 4;

/* TDM Configuration Reg3 */
pub const TAS2764_TDM_CFG3: u32 = TAS2764_REG(0X0, 0x0c);
pub const TAS2764_TDM_CFG3_RXS_MASK: u32 = GENMASK(7, 4);
pub const TAS2764_TDM_CFG3_RXS_SHIFT: u32 = 0x4;
pub const TAS2764_TDM_CFG3_MASK: u32 = GENMASK(3, 0);

/* TDM Configuration Reg4 */
pub const TAS2764_TDM_CFG4: u32 = TAS2764_REG(0X0, 0x0d);
pub const TAS2764_TDM_CFG4_TX_MASK: u32 = BIT(0);
pub const TAS2764_TDM_CFG4_TX_RISING: u32 = 0x0;
pub const TAS2764_TDM_CFG4_TX_FALLING: u32 = BIT(0);

/* TDM Configuration Reg5 */
pub const TAS2764_TDM_CFG5: u32 = TAS2764_REG(0X0, 0x0e);
pub const TAS2764_TDM_CFG5_VSNS_MASK: u32 = BIT(6);
pub const TAS2764_TDM_CFG5_VSNS_ENABLE: u32 = BIT(6);
pub const TAS2764_TDM_CFG5_50_MASK: u32 = GENMASK(5, 0);

/* TDM Configuration Reg6 */
pub const TAS2764_TDM_CFG6: u32 = TAS2764_REG(0X0, 0x0f);
pub const TAS2764_TDM_CFG6_ISNS_MASK: u32 = BIT(6);
pub const TAS2764_TDM_CFG6_ISNS_ENABLE: u32 = BIT(6);
pub const TAS2764_TDM_CFG6_50_MASK: u32 = GENMASK(5, 0);

/* Interrupt Masks */
pub const TAS2764_INT_MASK0: u32 = TAS2764_REG(0x0, 0x3b);
pub const TAS2764_INT_MASK1: u32 = TAS2764_REG(0x0, 0x3c);
pub const TAS2764_INT_MASK2: u32 = TAS2764_REG(0x0, 0x40);
pub const TAS2764_INT_MASK3: u32 = TAS2764_REG(0x0, 0x41);
pub const TAS2764_INT_MASK4: u32 = TAS2764_REG(0x0, 0x3d);

/* Latched Fault Registers */
pub const TAS2764_INT_LTCH0: u32 = TAS2764_REG(0x0, 0x49);
pub const TAS2764_INT_LTCH1: u32 = TAS2764_REG(0x0, 0x4a);
pub const TAS2764_INT_LTCH1_0: u32 = TAS2764_REG(0x0, 0x4b);
pub const TAS2764_INT_LTCH2: u32 = TAS2764_REG(0x0, 0x4f);
pub const TAS2764_INT_LTCH3: u32 = TAS2764_REG(0x0, 0x50);
pub const TAS2764_INT_LTCH4: u32 = TAS2764_REG(0x0, 0x51);

/* Readout Registers */
pub const TAS2764_TEMP: u32 = TAS2764_REG(0x0, 0x56);

/* Clock/IRQ Settings */
pub const TAS2764_INT_CLK_CFG: u32 = TAS2764_REG(0x0, 0x5c);
pub const TAS2764_INT_CLK_CFG_IRQZ_CLR: u32 = BIT(2);

pub const TAS2764_BOP_CFG0: u32 = TAS2764_REG(0X0, 0x1d);

pub const TAS2764_SDOUT_HIZ_1: u32 = TAS2764_REG(0x1, 0x3d);
pub const TAS2764_SDOUT_HIZ_2: u32 = TAS2764_REG(0x1, 0x3e);
pub const TAS2764_SDOUT_HIZ_3: u32 = TAS2764_REG(0x1, 0x3f);
pub const TAS2764_SDOUT_HIZ_4: u32 = TAS2764_REG(0x1, 0x40);
pub const TAS2764_SDOUT_HIZ_5: u32 = TAS2764_REG(0x1, 0x41);
pub const TAS2764_SDOUT_HIZ_6: u32 = TAS2764_REG(0x1, 0x42);
pub const TAS2764_SDOUT_HIZ_7: u32 = TAS2764_REG(0x1, 0x43);
pub const TAS2764_SDOUT_HIZ_8: u32 = TAS2764_REG(0x1, 0x44);
pub const TAS2764_SDOUT_HIZ_9: u32 = TAS2764_REG(0x1, 0x45);
pub const TAS2764_SDOUT_HIZ_9_FORCE_0_EN: u32 = BIT(7);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
