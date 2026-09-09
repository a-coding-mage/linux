/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Device driver for regulators in MAX5970 and MAX5978 IC
 *
 * Copyright (c) 2022 9elements GmbH
 *
 * Author: Patrick Rudolph <patrick.rudolph@9elements.com>
 */

// Dependency intent: the original header includes <linux/regmap.h>.

pub const MAX5970_NUM_SWITCHES: usize = 2;
pub const MAX5978_NUM_SWITCHES: usize = 1;
pub const MAX5970_NUM_LEDS: usize = 4;

macro_rules! MAX5970_REG_CURRENT_L { ($ch:expr) => { 0x01 + ($ch) * 4 }; }
macro_rules! MAX5970_REG_CURRENT_H { ($ch:expr) => { 0x00 + ($ch) * 4 }; }
macro_rules! MAX5970_REG_VOLTAGE_L { ($ch:expr) => { 0x03 + ($ch) * 4 }; }
macro_rules! MAX5970_REG_VOLTAGE_H { ($ch:expr) => { 0x02 + ($ch) * 4 }; }
pub const MAX5970_REG_MON_RANGE: u32 = 0x18;
pub const MAX5970_MON_MASK: u32 = 0x3;
macro_rules! MAX5970_MON { ($reg:expr, $ch:expr) => { (($reg) >> (($ch) * 2)) & MAX5970_MON_MASK }; }
pub const MAX5970_MON_MAX_RANGE_UV: u32 = 16000000;

macro_rules! MAX5970_REG_CH_UV_WARN_H { ($ch:expr) => { 0x1A + ($ch) * 10 }; }
macro_rules! MAX5970_REG_CH_UV_WARN_L { ($ch:expr) => { 0x1B + ($ch) * 10 }; }
macro_rules! MAX5970_REG_CH_UV_CRIT_H { ($ch:expr) => { 0x1C + ($ch) * 10 }; }
macro_rules! MAX5970_REG_CH_UV_CRIT_L { ($ch:expr) => { 0x1D + ($ch) * 10 }; }
macro_rules! MAX5970_REG_CH_OV_WARN_H { ($ch:expr) => { 0x1E + ($ch) * 10 }; }
macro_rules! MAX5970_REG_CH_OV_WARN_L { ($ch:expr) => { 0x1F + ($ch) * 10 }; }
macro_rules! MAX5970_REG_CH_OV_CRIT_H { ($ch:expr) => { 0x20 + ($ch) * 10 }; }
macro_rules! MAX5970_REG_CH_OV_CRIT_L { ($ch:expr) => { 0x21 + ($ch) * 10 }; }

macro_rules! MAX5970_VAL2REG_H { ($x:expr) => { (($x) >> 2) & 0xFF }; }
macro_rules! MAX5970_VAL2REG_L { ($x:expr) => { ($x) & 0x3 }; }

macro_rules! MAX5970_REG_DAC_FAST { ($ch:expr) => { 0x2E + ($ch) }; }
pub const MAX5970_FAST2SLOW_RATIO: u32 = 200;

pub const MAX5970_REG_STATUS0: u32 = 0x31;
macro_rules! MAX5970_CB_IFAULTF { ($ch:expr) => { 1 << ($ch) }; }
macro_rules! MAX5970_CB_IFAULTS { ($ch:expr) => { 1 << (($ch) + 4) }; }

pub const MAX5970_REG_STATUS1: u32 = 0x32;
pub const STATUS1_PROT_MASK: u32 = 0x3;
macro_rules! STATUS1_PROT { ($reg:expr) => { (($reg) >> 6) & STATUS1_PROT_MASK }; }
pub const STATUS1_PROT_SHUTDOWN: u32 = 0;
pub const STATUS1_PROT_CLEAR_PG: u32 = 1;
pub const STATUS1_PROT_ALERT_ONLY: u32 = 2;

pub const MAX5970_REG_STATUS2: u32 = 0x33;
pub const MAX5970_IRNG_MASK: u32 = 0x3;
macro_rules! MAX5970_IRNG { ($reg:expr, $ch:expr) => { (($reg) >> (($ch) * 2)) & MAX5970_IRNG_MASK }; }

pub const MAX5970_REG_STATUS3: u32 = 0x34;
// BIT is supplied by the Linux dependency represented by the original header.
macro_rules! MAX5970_STATUS3_ALERT { () => { BIT(4) }; }
macro_rules! MAX5970_STATUS3_PG { ($ch:expr) => { BIT($ch) }; }

pub const MAX5970_REG_FAULT0: u32 = 0x35;
macro_rules! UV_STATUS_WARN { ($ch:expr) => { 1 << ($ch) }; }
macro_rules! UV_STATUS_CRIT { ($ch:expr) => { 1 << (($ch) + 4) }; }

pub const MAX5970_REG_FAULT1: u32 = 0x36;
macro_rules! OV_STATUS_WARN { ($ch:expr) => { 1 << ($ch) }; }
macro_rules! OV_STATUS_CRIT { ($ch:expr) => { 1 << (($ch) + 4) }; }

pub const MAX5970_REG_FAULT2: u32 = 0x37;
macro_rules! OC_STATUS_WARN { ($ch:expr) => { 1 << ($ch) }; }

pub const MAX5970_REG_CHXEN: u32 = 0x3b;
macro_rules! CHXEN { ($ch:expr) => { 3 << (($ch) * 2) }; }

pub const MAX5970_REG_LED_FLASH: u32 = 0x43;
pub const MAX_REGISTERS: u32 = 0x49;
pub const ADC_MASK: u32 = 0x3FF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
