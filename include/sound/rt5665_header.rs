/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/rt5665.h -- Platform data for RT5665
 *
 * Copyright 2016 Realtek Microelectronics
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rt5665_dmic1_data_pin {
    RT5665_DMIC1_NULL,
    RT5665_DMIC1_DATA_GPIO4,
    RT5665_DMIC1_DATA_IN2N,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rt5665_dmic2_data_pin {
    RT5665_DMIC2_NULL,
    RT5665_DMIC2_DATA_GPIO5,
    RT5665_DMIC2_DATA_IN2P,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rt5665_jd_src {
    RT5665_JD_NULL,
    RT5665_JD1,
}

#[repr(C)]
pub struct rt5665_platform_data {
    pub in1_diff: bool,
    pub in2_diff: bool,
    pub in3_diff: bool,
    pub in4_diff: bool,

    pub dmic1_data_pin: rt5665_dmic1_data_pin,
    pub dmic2_data_pin: rt5665_dmic2_data_pin,
    pub jd_src: rt5665_jd_src,

    pub sar_hs_type: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
