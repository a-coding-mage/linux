/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016 Chen-Yu Tsai
 *
 * Chen-Yu Tsai <wens@csie.org>
 */

// Dependency intent from the original header:
// #include <dt-bindings/clock/sun9i-a80-de.h>
// #include <dt-bindings/reset/sun9i-a80-de.h>

/* Intermediary clock dividers are not exported */
pub const CLK_FE0_DIV: i32 = 31;
pub const CLK_FE1_DIV: i32 = 32;
pub const CLK_FE2_DIV: i32 = 33;
pub const CLK_BE0_DIV: i32 = 34;
pub const CLK_BE1_DIV: i32 = 35;
pub const CLK_BE2_DIV: i32 = 36;

pub const CLK_NUMBER: i32 = CLK_BE2_DIV + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
