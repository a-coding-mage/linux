/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2020 Yangtao Li <frank@allwinnertech.com>
 */

// Translated from ccu-sun50i-a100-r.h.
// Dependencies originally provided by:
// - <dt-bindings/clock/sun50i-a100-r-ccu.h>
// - <dt-bindings/reset/sun50i-a100-r-ccu.h>

pub const CLK_R_CPUS: u32 = 0;
pub const CLK_R_AHB: u32 = 1;

/* exported except APB1 for R_PIO */

pub const CLK_R_APB2: u32 = 3;

pub const CLK_NUMBER: u32 = CLK_R_AHB_BUS_RTC + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
