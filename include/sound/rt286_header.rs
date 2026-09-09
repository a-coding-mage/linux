/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/rt286.h -- Platform data for RT286
 *
 * Copyright 2013 Realtek Microelectronics
 */

#[repr(C)]
pub struct rt286_platform_data {
    pub cbj_en: bool,  /*combo jack enable*/
    pub gpio2_en: bool, /*GPIO2 enable*/
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
