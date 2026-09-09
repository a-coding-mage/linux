/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Loongson-1 clock tree IDs
 *
 * Copyright (C) 2023 Keguang Zhang <keguang.zhang@gmail.com>
 */

pub const LS1X_CLKID_PLL: u32 = 0;
pub const LS1X_CLKID_CPU: u32 = 1;
pub const LS1X_CLKID_DC: u32 = 2;
pub const LS1X_CLKID_AHB: u32 = 3;
pub const LS1X_CLKID_APB: u32 = 4;

pub const CLK_NR_CLKS: u32 = LS1X_CLKID_APB + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
