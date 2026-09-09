/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2017 Icenowy Zheng <icenowy@aosc.xyz>
 */

// #include <dt-bindings/clock/sun50i-h6-r-ccu.h>
// #include <dt-bindings/reset/sun50i-h6-r-ccu.h>

/* AHB/APB bus clocks are not exported except APB1 for R_PIO */
pub const CLK_R_AHB: i32 = 1;

pub const CLK_R_APB2: i32 = 3;

pub const CLK_NUMBER: i32 = CLK_R_APB1_RTC + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
