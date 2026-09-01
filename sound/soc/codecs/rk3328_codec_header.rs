/* SPDX-License-Identifier: GPL-2.0 */
/*
 * rk3328 ALSA SoC Audio driver
 *
 * Copyright (c) 2017, Fuzhou Rockchip Electronics Co., Ltd All rights reserved.
 */

/* Derived from linux/bitfield.h-style BIT/GENMASK macros used by this header. */
pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

/* codec register */
pub const CODEC_RESET: u32 = 0x00 << 2;
pub const DAC_INIT_CTRL1: u32 = 0x03 << 2;
pub const DAC_INIT_CTRL2: u32 = 0x04 << 2;
pub const DAC_INIT_CTRL3: u32 = 0x05 << 2;
pub const DAC_PRECHARGE_CTRL: u32 = 0x22 << 2;
pub const DAC_PWR_CTRL: u32 = 0x23 << 2;
pub const DAC_CLK_CTRL: u32 = 0x24 << 2;
pub const HPMIX_CTRL: u32 = 0x25 << 2;
pub const DAC_SELECT: u32 = 0x26 << 2;
pub const HPOUT_CTRL: u32 = 0x27 << 2;
pub const HPOUTL_GAIN_CTRL: u32 = 0x28 << 2;
pub const HPOUTR_GAIN_CTRL: u32 = 0x29 << 2;
pub const HPOUT_POP_CTRL: u32 = 0x2a << 2;

/* REG00: CODEC_RESET */
pub const PWR_RST_BYPASS_DIS: u32 = 0x0 << 6;
pub const PWR_RST_BYPASS_EN: u32 = 0x1 << 6;
pub const DIG_CORE_RST: u32 = 0x0 << 1;
pub const DIG_CORE_WORK: u32 = 0x1 << 1;
pub const SYS_RST: u32 = 0x0 << 0;
pub const SYS_WORK: u32 = 0x1 << 0;

/* REG03: DAC_INIT_CTRL1 */
pub const PIN_DIRECTION_MASK: u32 = BIT(5);
pub const PIN_DIRECTION_IN: u32 = 0x0 << 5;
pub const PIN_DIRECTION_OUT: u32 = 0x1 << 5;
pub const DAC_I2S_MODE_MASK: u32 = BIT(4);
pub const DAC_I2S_MODE_SLAVE: u32 = 0x0 << 4;
pub const DAC_I2S_MODE_MASTER: u32 = 0x1 << 4;

/* REG04: DAC_INIT_CTRL2 */
pub const DAC_I2S_LRP_MASK: u32 = BIT(7);
pub const DAC_I2S_LRP_NORMAL: u32 = 0x0 << 7;
pub const DAC_I2S_LRP_REVERSAL: u32 = 0x1 << 7;
pub const DAC_VDL_MASK: u32 = GENMASK(6, 5);
pub const DAC_VDL_16BITS: u32 = 0x0 << 5;
pub const DAC_VDL_20BITS: u32 = 0x1 << 5;
pub const DAC_VDL_24BITS: u32 = 0x2 << 5;
pub const DAC_VDL_32BITS: u32 = 0x3 << 5;
pub const DAC_MODE_MASK: u32 = GENMASK(4, 3);
pub const DAC_MODE_RJM: u32 = 0x0 << 3;
pub const DAC_MODE_LJM: u32 = 0x1 << 3;
pub const DAC_MODE_I2S: u32 = 0x2 << 3;
pub const DAC_MODE_PCM: u32 = 0x3 << 3;
pub const DAC_LR_SWAP_MASK: u32 = BIT(2);
pub const DAC_LR_SWAP_DIS: u32 = 0x0 << 2;
pub const DAC_LR_SWAP_EN: u32 = 0x1 << 2;

/* REG05: DAC_INIT_CTRL3 */
pub const DAC_WL_MASK: u32 = GENMASK(3, 2);
pub const DAC_WL_16BITS: u32 = 0x0 << 2;
pub const DAC_WL_20BITS: u32 = 0x1 << 2;
pub const DAC_WL_24BITS: u32 = 0x2 << 2;
pub const DAC_WL_32BITS: u32 = 0x3 << 2;
pub const DAC_RST_MASK: u32 = BIT(1);
pub const DAC_RST_EN: u32 = 0x0 << 1;
pub const DAC_RST_DIS: u32 = 0x1 << 1;
pub const DAC_BCP_MASK: u32 = BIT(0);
pub const DAC_BCP_NORMAL: u32 = 0x0 << 0;
pub const DAC_BCP_REVERSAL: u32 = 0x1 << 0;

/* REG22: DAC_PRECHARGE_CTRL */
pub const DAC_CHARGE_XCHARGE_MASK: u32 = BIT(7);
pub const DAC_CHARGE_DISCHARGE: u32 = 0x0 << 7;
pub const DAC_CHARGE_PRECHARGE: u32 = 0x1 << 7;
pub const DAC_CHARGE_CURRENT_64I_MASK: u32 = BIT(6);
pub const DAC_CHARGE_CURRENT_64I: u32 = 0x1 << 6;
pub const DAC_CHARGE_CURRENT_32I_MASK: u32 = BIT(5);
pub const DAC_CHARGE_CURRENT_32I: u32 = 0x1 << 5;
pub const DAC_CHARGE_CURRENT_16I_MASK: u32 = BIT(4);
pub const DAC_CHARGE_CURRENT_16I: u32 = 0x1 << 4;
pub const DAC_CHARGE_CURRENT_08I_MASK: u32 = BIT(3);
pub const DAC_CHARGE_CURRENT_08I: u32 = 0x1 << 3;
pub const DAC_CHARGE_CURRENT_04I_MASK: u32 = BIT(2);
pub const DAC_CHARGE_CURRENT_04I: u32 = 0x1 << 2;
pub const DAC_CHARGE_CURRENT_02I_MASK: u32 = BIT(1);
pub const DAC_CHARGE_CURRENT_02I: u32 = 0x1 << 1;
pub const DAC_CHARGE_CURRENT_I_MASK: u32 = BIT(0);
pub const DAC_CHARGE_CURRENT_I: u32 = 0x1 << 0;
pub const DAC_CHARGE_CURRENT_ALL_MASK: u32 = GENMASK(6, 0);
pub const DAC_CHARGE_CURRENT_ALL_OFF: u32 = 0x00;
pub const DAC_CHARGE_CURRENT_ALL_ON: u32 = 0x7f;

/* REG23: DAC_PWR_CTRL */
pub const DAC_PWR_MASK: u32 = BIT(6);
pub const DAC_PWR_OFF: u32 = 0x0 << 6;
pub const DAC_PWR_ON: u32 = 0x1 << 6;
pub const DACL_PATH_REFV_MASK: u32 = BIT(5);
pub const DACL_PATH_REFV_OFF: u32 = 0x0 << 5;
pub const DACL_PATH_REFV_ON: u32 = 0x1 << 5;
pub const HPOUTL_ZERO_CROSSING_MASK: u32 = BIT(4);
pub const HPOUTL_ZERO_CROSSING_OFF: u32 = 0x0 << 4;
pub const HPOUTL_ZERO_CROSSING_ON: u32 = 0x1 << 4;
pub const DACR_PATH_REFV_MASK: u32 = BIT(1);
pub const DACR_PATH_REFV_OFF: u32 = 0x0 << 1;
pub const DACR_PATH_REFV_ON: u32 = 0x1 << 1;
pub const HPOUTR_ZERO_CROSSING_MASK: u32 = BIT(0);
pub const HPOUTR_ZERO_CROSSING_OFF: u32 = 0x0 << 0;
pub const HPOUTR_ZERO_CROSSING_ON: u32 = 0x1 << 0;

/* REG24: DAC_CLK_CTRL */
pub const DACL_REFV_MASK: u32 = BIT(7);
pub const DACL_REFV_OFF: u32 = 0x0 << 7;
pub const DACL_REFV_ON: u32 = 0x1 << 7;
pub const DACL_CLK_MASK: u32 = BIT(6);
pub const DACL_CLK_OFF: u32 = 0x0 << 6;
pub const DACL_CLK_ON: u32 = 0x1 << 6;
pub const DACL_MASK: u32 = BIT(5);
pub const DACL_OFF: u32 = 0x0 << 5;
pub const DACL_ON: u32 = 0x1 << 5;
pub const DACL_INIT_MASK: u32 = BIT(4);
pub const DACL_INIT_OFF: u32 = 0x0 << 4;
pub const DACL_INIT_ON: u32 = 0x1 << 4;
pub const DACR_REFV_MASK: u32 = BIT(3);
pub const DACR_REFV_OFF: u32 = 0x0 << 3;
pub const DACR_REFV_ON: u32 = 0x1 << 3;
pub const DACR_CLK_MASK: u32 = BIT(2);
pub const DACR_CLK_OFF: u32 = 0x0 << 2;
pub const DACR_CLK_ON: u32 = 0x1 << 2;
pub const DACR_MASK: u32 = BIT(1);
pub const DACR_OFF: u32 = 0x0 << 1;
pub const DACR_ON: u32 = 0x1 << 1;
pub const DACR_INIT_MASK: u32 = BIT(0);
pub const DACR_INIT_OFF: u32 = 0x0 << 0;
pub const DACR_INIT_ON: u32 = 0x1 << 0;

/* REG25: HPMIX_CTRL*/
pub const HPMIXL_MASK: u32 = BIT(6);
pub const HPMIXL_DIS: u32 = 0x0 << 6;
pub const HPMIXL_EN: u32 = 0x1 << 6;
pub const HPMIXL_INIT_MASK: u32 = BIT(5);
pub const HPMIXL_INIT_DIS: u32 = 0x0 << 5;
pub const HPMIXL_INIT_EN: u32 = 0x1 << 5;
pub const HPMIXL_INIT2_MASK: u32 = BIT(4);
pub const HPMIXL_INIT2_DIS: u32 = 0x0 << 4;
pub const HPMIXL_INIT2_EN: u32 = 0x1 << 4;
pub const HPMIXR_MASK: u32 = BIT(2);
pub const HPMIXR_DIS: u32 = 0x0 << 2;
pub const HPMIXR_EN: u32 = 0x1 << 2;
pub const HPMIXR_INIT_MASK: u32 = BIT(1);
pub const HPMIXR_INIT_DIS: u32 = 0x0 << 1;
pub const HPMIXR_INIT_EN: u32 = 0x1 << 1;
pub const HPMIXR_INIT2_MASK: u32 = BIT(0);
pub const HPMIXR_INIT2_DIS: u32 = 0x0 << 0;
pub const HPMIXR_INIT2_EN: u32 = 0x1 << 0;

/* REG26: DAC_SELECT */
pub const DACL_SELECT_MASK: u32 = BIT(4);
pub const DACL_UNSELECT: u32 = 0x0 << 4;
pub const DACL_SELECT: u32 = 0x1 << 4;
pub const DACR_SELECT_MASK: u32 = BIT(0);
pub const DACR_UNSELECT: u32 = 0x0 << 0;
pub const DACR_SELECT: u32 = 0x1 << 0;

/* REG27: HPOUT_CTRL */
pub const HPOUTL_MASK: u32 = BIT(7);
pub const HPOUTL_DIS: u32 = 0x0 << 7;
pub const HPOUTL_EN: u32 = 0x1 << 7;
pub const HPOUTL_INIT_MASK: u32 = BIT(6);
pub const HPOUTL_INIT_DIS: u32 = 0x0 << 6;
pub const HPOUTL_INIT_EN: u32 = 0x1 << 6;
pub const HPOUTL_MUTE_MASK: u32 = BIT(5);
pub const HPOUTL_MUTE: u32 = 0x0 << 5;
pub const HPOUTL_UNMUTE: u32 = 0x1 << 5;
pub const HPOUTR_MASK: u32 = BIT(4);
pub const HPOUTR_DIS: u32 = 0x0 << 4;
pub const HPOUTR_EN: u32 = 0x1 << 4;
pub const HPOUTR_INIT_MASK: u32 = BIT(3);
pub const HPOUTR_INIT_DIS: u32 = 0x0 << 3;
pub const HPOUTR_INIT_EN: u32 = 0x1 << 3;
pub const HPOUTR_MUTE_MASK: u32 = BIT(2);
pub const HPOUTR_MUTE: u32 = 0x0 << 2;
pub const HPOUTR_UNMUTE: u32 = 0x1 << 2;

/* REG28: HPOUTL_GAIN_CTRL */
pub const HPOUTL_GAIN_MASK: u32 = GENMASK(4, 0);

/* REG29: HPOUTR_GAIN_CTRL */
pub const HPOUTR_GAIN_MASK: u32 = GENMASK(4, 0);

/* REG2a: HPOUT_POP_CTRL */
pub const HPOUTR_POP_MASK: u32 = GENMASK(5, 4);
pub const HPOUTR_POP_XCHARGE: u32 = 0x1 << 4;
pub const HPOUTR_POP_WORK: u32 = 0x2 << 4;
pub const HPOUTL_POP_MASK: u32 = GENMASK(1, 0);
pub const HPOUTL_POP_XCHARGE: u32 = 0x1 << 0;
pub const HPOUTL_POP_WORK: u32 = 0x2 << 0;

pub const RK3328_HIFI: u32 = 0;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rk3328_reg_msk_val {
    pub reg: ::core::ffi::c_uint,
    pub msk: ::core::ffi::c_uint,
    pub val: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
