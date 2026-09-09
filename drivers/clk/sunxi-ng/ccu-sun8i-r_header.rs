/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016 Icenowy <icenowy@aosc.xyz>
 */

// Dependency intent from the original header:
// #include <dt-bindings/clock/sun8i-r-ccu.h>
// #include <dt-bindings/reset/sun8i-r-ccu.h>

/* AHB/APB bus clocks are not exported */
pub const CLK_AHB0: u32 = 1;
pub const CLK_APB0: u32 = 2;

pub const CLK_NUMBER: u32 = CLK_IR + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
