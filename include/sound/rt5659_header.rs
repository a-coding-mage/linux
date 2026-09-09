/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/rt5659.h -- Platform data for RT5659
 *
 * Copyright 2013 Realtek Microelectronics
 */

#[repr(C)]
pub enum rt5659_dmic1_data_pin {
    RT5659_DMIC1_NULL,
    RT5659_DMIC1_DATA_IN2N,
    RT5659_DMIC1_DATA_GPIO5,
    RT5659_DMIC1_DATA_GPIO9,
    RT5659_DMIC1_DATA_GPIO11,
}

#[repr(C)]
pub enum rt5659_dmic2_data_pin {
    RT5659_DMIC2_NULL,
    RT5659_DMIC2_DATA_IN2P,
    RT5659_DMIC2_DATA_GPIO6,
    RT5659_DMIC2_DATA_GPIO10,
    RT5659_DMIC2_DATA_GPIO12,
}

#[repr(C)]
pub enum rt5659_jd_src {
    RT5659_JD_NULL,
    RT5659_JD3,
    RT5659_JD_HDA_HEADER,
}

#[repr(C)]
pub struct rt5659_platform_data {
    pub in1_diff: bool,
    pub in3_diff: bool,
    pub in4_diff: bool,

    pub ldo1_en: i32, /* GPIO for LDO1_EN */
    pub reset: i32, /* GPIO for RESET */

    pub dmic1_data_pin: rt5659_dmic1_data_pin,
    pub dmic2_data_pin: rt5659_dmic2_data_pin,
    pub jd_src: rt5659_jd_src,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
