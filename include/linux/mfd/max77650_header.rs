/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 BayLibre SAS
 * Author: Bartosz Golaszewski <bgolaszewski@baylibre.com>
 *
 * Common definitions for MAXIM 77650/77651 charger/power-supply.
 */

// The C header includes <linux/bits.h>; MAX77650_CID_MASK is its local
// GENMASK(3, 0) equivalent.

pub const MAX77650_REG_INT_GLBL: u8 = 0x00;
pub const MAX77650_REG_INT_CHG: u8 = 0x01;
pub const MAX77650_REG_STAT_CHG_A: u8 = 0x02;
pub const MAX77650_REG_STAT_CHG_B: u8 = 0x03;
pub const MAX77650_REG_ERCFLAG: u8 = 0x04;
pub const MAX77650_REG_STAT_GLBL: u8 = 0x05;
pub const MAX77650_REG_INTM_GLBL: u8 = 0x06;
pub const MAX77650_REG_INTM_CHG: u8 = 0x07;
pub const MAX77650_REG_CNFG_GLBL: u8 = 0x10;
pub const MAX77650_REG_CID: u8 = 0x11;
pub const MAX77650_REG_CNFG_GPIO: u8 = 0x12;
pub const MAX77650_REG_CNFG_CHG_A: u8 = 0x18;
pub const MAX77650_REG_CNFG_CHG_B: u8 = 0x19;
pub const MAX77650_REG_CNFG_CHG_C: u8 = 0x1a;
pub const MAX77650_REG_CNFG_CHG_D: u8 = 0x1b;
pub const MAX77650_REG_CNFG_CHG_E: u8 = 0x1c;
pub const MAX77650_REG_CNFG_CHG_F: u8 = 0x1d;
pub const MAX77650_REG_CNFG_CHG_G: u8 = 0x1e;
pub const MAX77650_REG_CNFG_CHG_H: u8 = 0x1f;
pub const MAX77650_REG_CNFG_CHG_I: u8 = 0x20;
pub const MAX77650_REG_CNFG_SBB_TOP: u8 = 0x28;
pub const MAX77650_REG_CNFG_SBB0_A: u8 = 0x29;
pub const MAX77650_REG_CNFG_SBB0_B: u8 = 0x2a;
pub const MAX77650_REG_CNFG_SBB1_A: u8 = 0x2b;
pub const MAX77650_REG_CNFG_SBB1_B: u8 = 0x2c;
pub const MAX77650_REG_CNFG_SBB2_A: u8 = 0x2d;
pub const MAX77650_REG_CNFG_SBB2_B: u8 = 0x2e;
pub const MAX77650_REG_CNFG_LDO_A: u8 = 0x38;
pub const MAX77650_REG_CNFG_LDO_B: u8 = 0x39;
pub const MAX77650_REG_CNFG_LED0_A: u8 = 0x40;
pub const MAX77650_REG_CNFG_LED1_A: u8 = 0x41;
pub const MAX77650_REG_CNFG_LED2_A: u8 = 0x42;
pub const MAX77650_REG_CNFG_LED0_B: u8 = 0x43;
pub const MAX77650_REG_CNFG_LED1_B: u8 = 0x44;
pub const MAX77650_REG_CNFG_LED2_B: u8 = 0x45;
pub const MAX77650_REG_CNFG_LED_TOP: u8 = 0x46;

pub const MAX77650_CID_MASK: u8 = 0x0f;

#[inline]
pub const fn MAX77650_CID_BITS(reg: u8) -> u8 {
    reg & MAX77650_CID_MASK
}

pub const MAX77650_CID_77650A: u8 = 0x03;
pub const MAX77650_CID_77650C: u8 = 0x0a;
pub const MAX77650_CID_77651A: u8 = 0x06;
pub const MAX77650_CID_77651B: u8 = 0x08;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
