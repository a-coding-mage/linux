/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * fan53555.h - Fairchild Regulator FAN53555 Driver
 *
 * Copyright (C) 2012 Marvell Technology Ltd.
 * Yunfan Zhang <yfzhang@marvell.com>
 */

/* VSEL ID */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Fan53555VselId {
    FAN53555_VSEL_ID_0 = 0,
    FAN53555_VSEL_ID_1,
}

/* Transition slew rate limiting from a low to high voltage.
 * -----------------------
 *   Bin |Slew Rate(mV/uS)
 * ------|----------------
 *   000 |    64.00
 * ------|----------------
 *   001 |    32.00
 * ------|----------------
 *   010 |    16.00
 * ------|----------------
 *   011 |     8.00
 * ------|----------------
 *   100 |     4.00
 * ------|----------------
 *   101 |     2.00
 * ------|----------------
 *   110 |     1.00
 * ------|----------------
 *   111 |     0.50
 * -----------------------
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Fan53555SlewRate {
    FAN53555_SLEW_RATE_64MV = 0,
    FAN53555_SLEW_RATE_32MV,
    FAN53555_SLEW_RATE_16MV,
    FAN53555_SLEW_RATE_8MV,
    FAN53555_SLEW_RATE_4MV,
    FAN53555_SLEW_RATE_2MV,
    FAN53555_SLEW_RATE_1MV,
    FAN53555_SLEW_RATE_0_5MV,
}

/* External dependency supplied by the regulator subsystem. */
pub struct regulator_init_data;

#[repr(C)]
pub struct fan53555_platform_data {
    pub regulator: *mut regulator_init_data,
    pub slew_rate: core::ffi::c_uint,
    /* Sleep VSEL ID */
    pub sleep_vsel_id: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
