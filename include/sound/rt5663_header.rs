/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/rt5663.h -- Platform data for RT5663
 *
 * Copyright 2017 Realtek Semiconductor Corp.
 */

#[repr(C)]
pub struct rt5663_platform_data {
    pub dc_offset_l_manual: u32,
    pub dc_offset_r_manual: u32,
    pub dc_offset_l_manual_mic: u32,
    pub dc_offset_r_manual_mic: u32,

    pub impedance_sensing_num: u32,
    pub impedance_sensing_table: *mut u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
