/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2014 Google, Inc
 *
 * Device Tree binding constants clocks for the Maxim 77802 PMIC.
 */

/* Fixed rate clocks. */

pub const MAX77802_CLK_32K_AP: i32 = 0;
pub const MAX77802_CLK_32K_CP: i32 = 1;

/* Total number of clocks. */
pub const MAX77802_CLKS_NUM: i32 = MAX77802_CLK_32K_CP + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
