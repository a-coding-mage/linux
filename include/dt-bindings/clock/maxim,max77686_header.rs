/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2014 Google, Inc
 *
 * Device Tree binding constants clocks for the Maxim 77686 PMIC.
 */

/* Fixed rate clocks. */

pub const MAX77686_CLK_AP: u32 = 0;
pub const MAX77686_CLK_CP: u32 = 1;
pub const MAX77686_CLK_PMIC: u32 = 2;

/* Total number of clocks. */
pub const MAX77686_CLKS_NUM: u32 = MAX77686_CLK_PMIC + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
