/* SPDX-License-Identifier: GPL-2.0-only */

pub const QORIQ_CLK_SYSCLK: i32 = 0;
pub const QORIQ_CLK_CMUX: i32 = 1;
pub const QORIQ_CLK_HWACCEL: i32 = 2;
pub const QORIQ_CLK_FMAN: i32 = 3;
pub const QORIQ_CLK_PLATFORM_PLL: i32 = 4;
pub const QORIQ_CLK_CORECLK: i32 = 5;

#[inline]
pub const fn QORIQ_CLK_PLL_DIV(x: i32) -> i32 {
    x - 1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
