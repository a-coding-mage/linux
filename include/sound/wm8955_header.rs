/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Platform data for WM8955
 *
 * Copyright 2009 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

#[repr(C)]
pub struct wm8955_pdata {
    /* Configure LOUT2/ROUT2 to drive a speaker */
    pub out2_speaker: u32,

    /* Configure MONOIN+/- in differential mode */
    pub monoin_diff: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
