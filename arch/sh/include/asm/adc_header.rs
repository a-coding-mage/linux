/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2004  Andriy Skulysh
 */

// Dependency intent: declarations from <cpu/adc.h> are supplied externally.

extern "C" {
    pub fn adc_single(channel: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
