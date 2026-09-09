/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2015 Markus Reichl
 *
 * Device Tree binding constants clocks for the Samsung S2MPS11 PMIC.
 */

/* Fixed rate clocks. */

pub const S2MPS11_CLK_AP: i32 = 0;
pub const S2MPS11_CLK_CP: i32 = 1;
pub const S2MPS11_CLK_BT: i32 = 2;

/* Total number of clocks. */
pub const S2MPS11_CLKS_NUM: i32 = S2MPS11_CLK_BT + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
