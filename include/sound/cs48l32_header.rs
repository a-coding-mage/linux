/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Register definitions for Cirrus Logic CS48L32
 *
 * Copyright (C) 2017-2018, 2020, 2022, 2025 Cirrus Logic, Inc. and
 *               Cirrus Logic International Semiconductor Ltd.
 */

/* pll_id for snd_soc_component_set_pll() */
pub const CS48L32_FLL1_REFCLK: i32 = 1;

/* source for snd_soc_component_set_pll() */
pub const CS48L32_FLL_SRC_NONE: i32 = -1;
pub const CS48L32_FLL_SRC_MCLK1: i32 = 0;
pub const CS48L32_FLL_SRC_PDMCLK: i32 = 5;
pub const CS48L32_FLL_SRC_ASP1_BCLK: i32 = 8;
pub const CS48L32_FLL_SRC_ASP2_BCLK: i32 = 9;
pub const CS48L32_FLL_SRC_ASP1_FSYNC: i32 = 12;
pub const CS48L32_FLL_SRC_ASP2_FSYNC: i32 = 13;

/* clk_id for snd_soc_component_set_sysclk() and snd_soc_dai_set_sysclk() */
pub const CS48L32_CLK_SYSCLK_1: i32 = 1;
pub const CS48L32_CLK_SYSCLK_2: i32 = 2;
pub const CS48L32_CLK_SYSCLK_3: i32 = 3;
pub const CS48L32_CLK_SYSCLK_4: i32 = 4;
pub const CS48L32_CLK_DSPCLK: i32 = 7;
pub const CS48L32_CLK_PDM_FLLCLK: i32 = 13;

/* source for snd_soc_component_set_sysclk() */
pub const CS48L32_CLK_SRC_MCLK1: i32 = 0x0;
pub const CS48L32_CLK_SRC_FLL1: i32 = 0x4;
pub const CS48L32_CLK_SRC_ASP1_BCLK: i32 = 0x8;
pub const CS48L32_CLK_SRC_ASP2_BCLK: i32 = 0x9;

#[repr(C)]
pub struct cs48l32 {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub reset_gpio: *mut gpio_desc,
    pub mclk1: *mut clk,
    pub core_supplies: [regulator_bulk_data; 2],
    pub vdd_d: *mut regulator,
    pub irq: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
