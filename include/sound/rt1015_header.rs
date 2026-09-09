/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/rt1015.h -- Platform data for RT1015
 *
 * Copyright 2020 Realtek Microelectronics
 */

#[repr(C)]
pub struct rt1015_platform_data {
    pub power_up_delay_ms: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
