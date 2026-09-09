/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Pinctrl binding constants for GS101
 *
 * Copyright 2020-2023 Google LLC
 */

pub const GS101_PIN_PULL_NONE: u32 = 0;
pub const GS101_PIN_PULL_DOWN: u32 = 1;
pub const GS101_PIN_PULL_UP: u32 = 3;

/* Pin function in power down mode */
pub const GS101_PIN_PDN_OUT0: u32 = 0;
pub const GS101_PIN_PDN_OUT1: u32 = 1;
pub const GS101_PIN_PDN_INPUT: u32 = 2;
pub const GS101_PIN_PDN_PREV: u32 = 3;

/* GS101 drive strengths */
pub const GS101_PIN_DRV_2_5_MA: u32 = 0;
pub const GS101_PIN_DRV_5_MA: u32 = 1;
pub const GS101_PIN_DRV_7_5_MA: u32 = 2;
pub const GS101_PIN_DRV_10_MA: u32 = 3;

pub const GS101_PIN_FUNC_INPUT: u32 = 0;
pub const GS101_PIN_FUNC_OUTPUT: u32 = 1;
pub const GS101_PIN_FUNC_2: u32 = 2;
pub const GS101_PIN_FUNC_3: u32 = 3;
pub const GS101_PIN_FUNC_EINT: u32 = 0xf;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
