/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * MAX517 DAC driver
 *
 * Copyright 2011 Roland Stigge <stigge@antcom.de>
 */

#[repr(C)]
pub struct max517_platform_data {
    pub vref_mv: [u16; 8],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
