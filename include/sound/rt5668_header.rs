/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/rt5668.h -- Platform data for RT5668
 *
 * Copyright 2018 Realtek Microelectronics
 */

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rt5668_dmic1_data_pin {
    RT5668_DMIC1_NULL = 0,
    RT5668_DMIC1_DATA_GPIO2 = 1,
    RT5668_DMIC1_DATA_GPIO5 = 2,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rt5668_dmic1_clk_pin {
    RT5668_DMIC1_CLK_GPIO1 = 0,
    RT5668_DMIC1_CLK_GPIO3 = 1,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rt5668_jd_src {
    RT5668_JD_NULL = 0,
    RT5668_JD1 = 1,
}

#[repr(C)]
pub struct rt5668_platform_data {
    pub dmic1_data_pin: rt5668_dmic1_data_pin,
    pub dmic1_clk_pin: rt5668_dmic1_clk_pin,
    pub jd_src: rt5668_jd_src,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
