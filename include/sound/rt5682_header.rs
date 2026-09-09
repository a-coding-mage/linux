/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/rt5682.h -- Platform data for RT5682
 *
 * Copyright 2018 Realtek Microelectronics
 */

use core::ffi::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rt5682_dmic1_data_pin {
    RT5682_DMIC1_NULL,
    RT5682_DMIC1_DATA_GPIO2,
    RT5682_DMIC1_DATA_GPIO5,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rt5682_dmic1_clk_pin {
    RT5682_DMIC1_CLK_GPIO1,
    RT5682_DMIC1_CLK_GPIO3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rt5682_jd_src {
    RT5682_JD_NULL,
    RT5682_JD1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rt5682_dai_clks {
    RT5682_DAI_WCLK_IDX,
    RT5682_DAI_BCLK_IDX,
    RT5682_DAI_NUM_CLKS,
}

#[repr(C)]
pub struct rt5682_platform_data {
    pub dmic1_data_pin: rt5682_dmic1_data_pin,
    pub dmic1_clk_pin: rt5682_dmic1_clk_pin,
    pub jd_src: rt5682_jd_src,
    pub btndet_delay: u32,
    pub dmic_clk_rate: u32,
    pub dmic_delay: u32,
    pub dmic_clk_driving_high: bool,
    pub dai_clk_names: [*const c_char; rt5682_dai_clks::RT5682_DAI_NUM_CLKS as usize],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
