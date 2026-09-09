/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */
/*
 * Copyright 2024 NXP
 */

// Translated from the C header; the original include guard is omitted.

pub const I2C_FM: i32 = 1 << 4;
pub const I2C_FM_PLUS: i32 = 0 << 4;

pub const I2C_FILTER: i32 = 0 << 5;
pub const I2C_NO_FILTER_HIGH_FREQUENCY: i32 = 1 << 5;
pub const I2C_NO_FILTER_LOW_FREQUENCY: i32 = 2 << 5;

pub const I3C_ADDR_METHOD_SETDASA: i32 = 1 << 0;
pub const I3C_ADDR_METHOD_SETAASA: i32 = 1 << 1;
pub const I3C_ADDR_METHOD_VENDOR: i32 = 1 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
