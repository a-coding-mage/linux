// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra210_mvc.h - Definitions for Tegra210 MVC driver
 *
 * Copyright (c) 2021 NVIDIA CORPORATION.  All rights reserved.
 *
 */

/*
 * Original C header guard removed:
 * __TEGRA210_MVC_H__
 */

use core::ffi::{c_int, c_uint};

/*
 * MVC_RX registers are with respect to XBAR.
 * The data comes from XBAR to MVC.
 */
pub const TEGRA210_MVC_RX_STATUS: c_uint = 0x0c;
pub const TEGRA210_MVC_RX_INT_STATUS: c_uint = 0x10;
pub const TEGRA210_MVC_RX_INT_MASK: c_uint = 0x14;
pub const TEGRA210_MVC_RX_INT_SET: c_uint = 0x18;
pub const TEGRA210_MVC_RX_INT_CLEAR: c_uint = 0x1c;
pub const TEGRA210_MVC_RX_CIF_CTRL: c_uint = 0x20;

/*
 * MVC_TX registers are with respect to XBAR.
 * The data goes out of MVC.
 */
pub const TEGRA210_MVC_TX_STATUS: c_uint = 0x4c;
pub const TEGRA210_MVC_TX_INT_STATUS: c_uint = 0x50;
pub const TEGRA210_MVC_TX_INT_MASK: c_uint = 0x54;
pub const TEGRA210_MVC_TX_INT_SET: c_uint = 0x58;
pub const TEGRA210_MVC_TX_INT_CLEAR: c_uint = 0x5c;
pub const TEGRA210_MVC_TX_CIF_CTRL: c_uint = 0x60;

/* Register offsets from TEGRA210_MVC*_BASE */
pub const TEGRA210_MVC_ENABLE: c_uint = 0x80;
pub const TEGRA210_MVC_SOFT_RESET: c_uint = 0x84;
pub const TEGRA210_MVC_CG: c_uint = 0x88;
pub const TEGRA210_MVC_STATUS: c_uint = 0x90;
pub const TEGRA210_MVC_INT_STATUS: c_uint = 0x94;
pub const TEGRA210_MVC_CTRL: c_uint = 0xa8;
pub const TEGRA210_MVC_SWITCH: c_uint = 0xac;
pub const TEGRA210_MVC_INIT_VOL: c_uint = 0xb0;
pub const TEGRA210_MVC_TARGET_VOL: c_uint = 0xd0;
pub const TEGRA210_MVC_DURATION: c_uint = 0xf0;
pub const TEGRA210_MVC_DURATION_INV: c_uint = 0xf4;
pub const TEGRA210_MVC_POLY_N1: c_uint = 0xf8;
pub const TEGRA210_MVC_POLY_N2: c_uint = 0xfc;
pub const TEGRA210_MVC_PEAK_CTRL: c_uint = 0x100;
pub const TEGRA210_MVC_CFG_RAM_CTRL: c_uint = 0x104;
pub const TEGRA210_MVC_CFG_RAM_DATA: c_uint = 0x108;
pub const TEGRA210_MVC_PEAK_VALUE: c_uint = 0x10c;
pub const TEGRA210_MVC_CONFIG_ERR_TYPE: c_uint = 0x12c;

/* Fields in TEGRA210_MVC_ENABLE */
pub const TEGRA210_MVC_EN_SHIFT: c_uint = 0;
pub const TEGRA210_MVC_EN: c_uint = 1 << TEGRA210_MVC_EN_SHIFT;

pub const TEGRA210_MVC_MUTE_SHIFT: c_uint = 8;
pub const TEGRA210_MUTE_MASK_EN: c_uint = 0xff;
pub const TEGRA210_MVC_MUTE_MASK: c_uint =
    TEGRA210_MUTE_MASK_EN << TEGRA210_MVC_MUTE_SHIFT;
pub const TEGRA210_MVC_MUTE_EN: c_uint = TEGRA210_MUTE_MASK_EN << TEGRA210_MVC_MUTE_SHIFT;
pub const TEGRA210_MVC_CH0_MUTE_EN: c_uint = 1;

pub const TEGRA210_MVC_PER_CHAN_CTRL_EN_SHIFT: c_uint = 30;
pub const TEGRA210_MVC_PER_CHAN_CTRL_EN_MASK: c_uint =
    1 << TEGRA210_MVC_PER_CHAN_CTRL_EN_SHIFT;
pub const TEGRA210_MVC_PER_CHAN_CTRL_EN: c_uint = 1 << TEGRA210_MVC_PER_CHAN_CTRL_EN_SHIFT;

pub const TEGRA210_MVC_CURVE_TYPE_SHIFT: c_uint = 1;
pub const TEGRA210_MVC_CURVE_TYPE_MASK: c_uint = 1 << TEGRA210_MVC_CURVE_TYPE_SHIFT;

pub const TEGRA210_MVC_VOLUME_SWITCH_SHIFT: c_uint = 2;
pub const TEGRA210_MVC_VOLUME_SWITCH_MASK: c_uint = 1 << TEGRA210_MVC_VOLUME_SWITCH_SHIFT;
pub const TEGRA210_MVC_VOLUME_SWITCH_TRIGGER: c_uint =
    1 << TEGRA210_MVC_VOLUME_SWITCH_SHIFT;
pub const TEGRA210_MVC_CTRL_DEFAULT: c_uint = 0x40000003;

pub const TEGRA210_MVC_INIT_VOL_DEFAULT_POLY: c_uint = 0x01000000;
pub const TEGRA210_MVC_INIT_VOL_DEFAULT_LINEAR: c_uint = 0x00000000;

/* Fields in TEGRA210_MVC ram ctrl */
pub const TEGRA210_MVC_CFG_RAM_CTRL_RW_SHIFT: c_uint = 14;
pub const TEGRA210_MVC_CFG_RAM_CTRL_RW_WRITE: c_uint =
    1 << TEGRA210_MVC_CFG_RAM_CTRL_RW_SHIFT;

pub const TEGRA210_MVC_CFG_RAM_CTRL_ADDR_INIT_EN_SHIFT: c_uint = 13;
pub const TEGRA210_MVC_CFG_RAM_CTRL_ADDR_INIT_EN: c_uint =
    1 << TEGRA210_MVC_CFG_RAM_CTRL_ADDR_INIT_EN_SHIFT;

pub const TEGRA210_MVC_CFG_RAM_CTRL_SEQ_ACCESS_EN_SHIFT: c_uint = 12;
pub const TEGRA210_MVC_CFG_RAM_CTRL_SEQ_ACCESS_EN: c_uint =
    1 << TEGRA210_MVC_CFG_RAM_CTRL_SEQ_ACCESS_EN_SHIFT;

pub const TEGRA210_MVC_CFG_RAM_CTRL_ADDR_SHIFT: c_uint = 0;
pub const TEGRA210_MVC_CFG_RAM_CTRL_ADDR_MASK: c_uint =
    0x1ff << TEGRA210_MVC_CFG_RAM_CTRL_ADDR_SHIFT;

pub const REG_SIZE: c_uint = 4;
pub const TEGRA210_MVC_MAX_CHAN_COUNT: usize = 8;

pub const fn TEGRA210_MVC_REG_OFFSET(reg: c_uint, i: c_uint) -> c_uint {
    reg + (REG_SIZE * i)
}

pub const fn TEGRA210_MVC_GET_CHAN(reg: c_uint, base: c_uint) -> c_uint {
    ((reg) - (base)) / REG_SIZE
}

pub const fn TEGRA210_GET_MUTE_VAL(val: c_uint) -> c_uint {
    ((val) >> TEGRA210_MVC_MUTE_SHIFT) & TEGRA210_MUTE_MASK_EN
}

pub const NUM_GAIN_POLY_COEFFS: usize = 9;

pub const CURVE_POLY: c_uint = 0;
pub const CURVE_LINEAR: c_uint = 1;

#[repr(C)]
pub struct tegra210_mvc_gain_params {
    pub poly_coeff: [c_int; NUM_GAIN_POLY_COEFFS],
    pub poly_n1: c_int,
    pub poly_n2: c_int,
    pub duration: c_int,
    pub duration_inv: c_int,
}

/* Opaque external type supplied by another translated header. */
pub enum regmap {}

#[repr(C)]
pub struct tegra210_mvc {
    pub volume: [c_int; TEGRA210_MVC_MAX_CHAN_COUNT],
    pub curve_type: c_uint,
    pub ctrl_value: c_uint,
    pub regmap: *mut regmap,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
