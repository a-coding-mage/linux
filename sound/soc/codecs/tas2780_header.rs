/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TAS2780.h - ALSA SoC Texas Instruments TAS2780 Mono Audio Amplifier
 *
 * Copyright (C) 2020-2022 Texas Instruments Incorporated - https://www.ti.com
 *
 * Author: Raphael Xu <raphael-xu@ti.com>
 */

/* Header guard __TAS2780_H__ omitted in Rust. */

pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    let high = if h >= 31 {
        u32::MAX
    } else {
        (1u32 << (h + 1)) - 1
    };
    let low = if l == 0 { 0 } else { (1u32 << l) - 1 };

    high & !low
}

/* Book Control Register */
pub const TAS2780_BOOKCTL_PAGE: u32 = 0;
pub const TAS2780_BOOKCTL_REG: u32 = 127;
pub const fn TAS2780_REG(page: u32, reg: u32) -> u32 {
    (page * 128) + reg
}

/* Page */
pub const TAS2780_PAGE: u32 = TAS2780_REG(0x0, 0x00);
pub const TAS2780_PAGE_PAGE_MASK: u32 = 255;

/* Software Reset */
pub const TAS2780_SW_RST: u32 = TAS2780_REG(0x0, 0x01);
pub const TAS2780_RST: u32 = BIT(0);

/* Power Control */
pub const TAS2780_PWR_CTRL: u32 = TAS2780_REG(0x0, 0x02);
pub const TAS2780_PWR_CTRL_MASK: u32 = GENMASK(1, 0);
pub const TAS2780_PWR_CTRL_ACTIVE: u32 = 0x0;
pub const TAS2780_PWR_CTRL_MUTE: u32 = BIT(0);
pub const TAS2780_PWR_CTRL_SHUTDOWN: u32 = BIT(1);

pub const TAS2780_VSENSE_POWER_EN: u32 = 3;
pub const TAS2780_ISENSE_POWER_EN: u32 = 4;

/* Digital Volume Control */
pub const TAS2780_DVC: u32 = TAS2780_REG(0x0, 0x1a);
pub const TAS2780_DVC_MAX: u32 = 0xc9;

pub const TAS2780_CHNL_0: u32 = TAS2780_REG(0x0, 0x03);

/* TDM Configuration Reg0 */
pub const TAS2780_TDM_CFG0: u32 = TAS2780_REG(0x0, 0x08);
pub const TAS2780_TDM_CFG0_SMP_MASK: u32 = BIT(5);
pub const TAS2780_TDM_CFG0_SMP_48KHZ: u32 = 0x0;
pub const TAS2780_TDM_CFG0_SMP_44_1KHZ: u32 = BIT(5);
pub const TAS2780_TDM_CFG0_MASK: u32 = GENMASK(3, 1);
pub const TAS2780_TDM_CFG0_44_1_48KHZ: u32 = BIT(3);
pub const TAS2780_TDM_CFG0_88_2_96KHZ: u32 = BIT(3) | BIT(1);

/* TDM Configuration Reg1 */
pub const TAS2780_TDM_CFG1: u32 = TAS2780_REG(0x0, 0x09);
pub const TAS2780_TDM_CFG1_MASK: u32 = GENMASK(5, 1);
pub const TAS2780_TDM_CFG1_51_SHIFT: u32 = 1;
pub const TAS2780_TDM_CFG1_RX_MASK: u32 = BIT(0);
pub const TAS2780_TDM_CFG1_RX_RISING: u32 = 0x0;
pub const TAS2780_TDM_CFG1_RX_FALLING: u32 = BIT(0);

/* TDM Configuration Reg2 */
pub const TAS2780_TDM_CFG2: u32 = TAS2780_REG(0x0, 0x0a);
pub const TAS2780_TDM_CFG2_RXW_MASK: u32 = GENMASK(3, 2);
pub const TAS2780_TDM_CFG2_RXW_16BITS: u32 = 0x0;
pub const TAS2780_TDM_CFG2_RXW_24BITS: u32 = BIT(3);
pub const TAS2780_TDM_CFG2_RXW_32BITS: u32 = BIT(3) | BIT(2);
pub const TAS2780_TDM_CFG2_RXS_MASK: u32 = GENMASK(1, 0);
pub const TAS2780_TDM_CFG2_RXS_16BITS: u32 = 0x0;
pub const TAS2780_TDM_CFG2_RXS_24BITS: u32 = BIT(0);
pub const TAS2780_TDM_CFG2_RXS_32BITS: u32 = BIT(1);
pub const TAS2780_TDM_CFG2_SCFG_MASK: u32 = GENMASK(5, 4);
pub const TAS2780_TDM_CFG2_SCFG_I2S: u32 = 0x0;
pub const TAS2780_TDM_CFG2_SCFG_LEFT_J: u32 = BIT(4);
pub const TAS2780_TDM_CFG2_SCFG_RIGHT_J: u32 = BIT(5);

/* TDM Configuration Reg3 */
pub const TAS2780_TDM_CFG3: u32 = TAS2780_REG(0x0, 0x0c);
pub const TAS2780_TDM_CFG3_RXS_MASK: u32 = GENMASK(7, 4);
pub const TAS2780_TDM_CFG3_RXS_SHIFT: u32 = 0x4;
pub const TAS2780_TDM_CFG3_MASK: u32 = GENMASK(3, 0);

/* TDM Configuration Reg4 */
pub const TAS2780_TDM_CFG4: u32 = TAS2780_REG(0x0, 0x0d);
pub const TAS2780_TDM_CFG4_TX_OFFSET_MASK: u32 = GENMASK(3, 1);

/* TDM Configuration Reg5 */
pub const TAS2780_TDM_CFG5: u32 = TAS2780_REG(0x0, 0x0e);
pub const TAS2780_TDM_CFG5_VSNS_MASK: u32 = BIT(6);
pub const TAS2780_TDM_CFG5_VSNS_ENABLE: u32 = BIT(6);
pub const TAS2780_TDM_CFG5_50_MASK: u32 = GENMASK(5, 0);

/* TDM Configuration Reg6 */
pub const TAS2780_TDM_CFG6: u32 = TAS2780_REG(0x0, 0x0f);
pub const TAS2780_TDM_CFG6_ISNS_MASK: u32 = BIT(6);
pub const TAS2780_TDM_CFG6_ISNS_ENABLE: u32 = BIT(6);
pub const TAS2780_TDM_CFG6_50_MASK: u32 = GENMASK(5, 0);

/* IC CFG */
pub const TAS2780_IC_CFG: u32 = TAS2780_REG(0x0, 0x5c);
pub const TAS2780_IC_CFG_MASK: u32 = GENMASK(7, 6);
pub const TAS2780_IC_CFG_ENABLE: u32 = BIT(7) | BIT(6);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
