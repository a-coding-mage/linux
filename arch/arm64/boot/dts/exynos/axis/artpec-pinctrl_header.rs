/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Axis ARTPEC-8 SoC device tree pinctrl constants
 *
 * Copyright (c) 2025 Samsung Electronics Co., Ltd.
 *             https://www.samsung.com
 * Copyright (c) 2025  Axis Communications AB.
 *             https://www.axis.com
 */

// Translated from the C header; the original include guard is not executable
// Rust syntax and is therefore represented by this comment.

pub const ARTPEC_PIN_PULL_NONE: u32 = 0;
pub const ARTPEC_PIN_PULL_DOWN: u32 = 1;
pub const ARTPEC_PIN_PULL_UP: u32 = 3;

pub const ARTPEC_PIN_FUNC_INPUT: u32 = 0;
pub const ARTPEC_PIN_FUNC_OUTPUT: u32 = 1;
pub const ARTPEC_PIN_FUNC_2: u32 = 2;
pub const ARTPEC_PIN_FUNC_3: u32 = 3;
pub const ARTPEC_PIN_FUNC_4: u32 = 4;
pub const ARTPEC_PIN_FUNC_5: u32 = 5;
pub const ARTPEC_PIN_FUNC_6: u32 = 6;
pub const ARTPEC_PIN_FUNC_EINT: u32 = 0xf;
pub const ARTPEC_PIN_FUNC_F: u32 = ARTPEC_PIN_FUNC_EINT;

/* Drive strength for ARTPEC */
pub const ARTPEC_PIN_DRV_SR1: u32 = 0x8;
pub const ARTPEC_PIN_DRV_SR2: u32 = 0x9;
pub const ARTPEC_PIN_DRV_SR3: u32 = 0xa;
pub const ARTPEC_PIN_DRV_SR4: u32 = 0xb;
pub const ARTPEC_PIN_DRV_SR5: u32 = 0xc;
pub const ARTPEC_PIN_DRV_SR6: u32 = 0xd;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
