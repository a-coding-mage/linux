/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016 Icenowy Zheng <icenowy@aosc.io>
 */

// Dependency intent preserved from:
// #include <dt-bindings/clock/sun8i-de2.h>
// #include <dt-bindings/reset/sun8i-de2.h>

/* Intermediary clock dividers are not exported */
pub const CLK_MIXER0_DIV: i32 = 3;
pub const CLK_MIXER1_DIV: i32 = 4;
pub const CLK_WB_DIV: i32 = 5;
pub const CLK_ROT_DIV: i32 = 11;

pub const CLK_NUMBER_WITH_ROT: i32 = CLK_ROT_DIV + 1;
pub const CLK_NUMBER_WITHOUT_ROT: i32 = CLK_WB + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
