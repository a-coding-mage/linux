/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tle62x0.h - platform glue to Infineon TLE62x0 driver chips
 *
 * Copyright 2007 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 */

#[repr(C)]
pub struct tle62x0_pdata {
    pub init_state: u32,
    pub gpio_count: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
