/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/rt5660.h -- Platform data for RT5660
 *
 * Copyright 2016 Realtek Semiconductor Corp.
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rt5660_dmic1_data_pin {
    RT5660_DMIC1_NULL = 0,
    RT5660_DMIC1_DATA_GPIO2,
    RT5660_DMIC1_DATA_IN1P,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rt5660_platform_data {
    /* IN1 & IN3 can optionally be differential */
    pub in1_diff: bool,
    pub in3_diff: bool,
    pub use_ldo2: bool,
    pub poweroff_codec_in_suspend: bool,

    pub dmic1_data_pin: rt5660_dmic1_data_pin,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
