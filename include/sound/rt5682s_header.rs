/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/rt5682s.h -- Platform data for RT5682I-VS
 *
 * Copyright 2021 Realtek Microelectronics
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rt5682s_dmic1_data_pin {
    RT5682S_DMIC1_DATA_NULL,
    RT5682S_DMIC1_DATA_GPIO2,
    RT5682S_DMIC1_DATA_GPIO5,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rt5682s_dmic1_clk_pin {
    RT5682S_DMIC1_CLK_NULL,
    RT5682S_DMIC1_CLK_GPIO1,
    RT5682S_DMIC1_CLK_GPIO3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rt5682s_jd_src {
    RT5682S_JD_NULL,
    RT5682S_JD1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rt5682s_dai_clks {
    RT5682S_DAI_WCLK_IDX,
    RT5682S_DAI_BCLK_IDX,
    RT5682S_DAI_NUM_CLKS,
}

pub const RT5682S_LDO_1_607V: u32 = 0;
pub const RT5682S_LDO_1_5V: u32 = 1;
pub const RT5682S_LDO_1_406V: u32 = 2;
pub const RT5682S_LDO_1_731V: u32 = 3;

#[repr(C)]
pub struct rt5682s_platform_data {
    pub dmic1_data_pin: rt5682s_dmic1_data_pin,
    pub dmic1_clk_pin: rt5682s_dmic1_clk_pin,
    pub jd_src: rt5682s_jd_src,
    pub dmic_clk_rate: core::ffi::c_uint,
    pub dmic_delay: core::ffi::c_uint,
    pub amic_delay: core::ffi::c_uint,
    pub ldo_dacref: core::ffi::c_uint,
    pub dmic_clk_driving_high: bool,

    pub dai_clk_names: [*const core::ffi::c_char; RT5682S_DAI_NUM_CLKS as usize],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
