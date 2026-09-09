/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Functions to access SY3686A power management chip.
 *
 * Copyright (C) 2021 reMarkable AS - http://www.remarkable.com/
 */

pub const SY7636A_REG_OPERATION_MODE_CRL: u32 = 0x00;
/* It is set if a gpio is used to control the regulator */
pub const SY7636A_OPERATION_MODE_CRL_VCOMCTL: u32 = 1 << 6;
pub const SY7636A_OPERATION_MODE_CRL_ONOFF: u32 = 1 << 7;
pub const SY7636A_REG_VCOM_ADJUST_CTRL_L: u32 = 0x01;
pub const SY7636A_REG_VCOM_ADJUST_CTRL_H: u32 = 0x02;
pub const SY7636A_REG_VCOM_ADJUST_CTRL_MASK: u32 = 0x01ff;
pub const SY7636A_REG_VLDO_VOLTAGE_ADJULST_CTRL: u32 = 0x03;
pub const SY7636A_REG_POWER_ON_DELAY_TIME: u32 = 0x06;
pub const SY7636A_REG_FAULT_FLAG: u32 = 0x07;
pub const SY7636A_FAULT_FLAG_PG: u32 = 1 << 0;
pub const SY7636A_REG_TERMISTOR_READOUT: u32 = 0x08;

pub const SY7636A_REG_MAX: u32 = 0x08;

pub const VCOM_ADJUST_CTRL_MASK: u32 = 0x1ff;
// Used to shift the high byte
pub const VCOM_ADJUST_CTRL_SHIFT: u32 = 8;
// Used to scale from VCOM_ADJUST_CTRL to mv
pub const VCOM_ADJUST_CTRL_SCAL: u32 = 10000;

pub const FAULT_FLAG_SHIFT: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
