/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * da7213.h - DA7213 ASoC Codec Driver Platform Data
 *
 * Copyright (c) 2013 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da7213_micbias_voltage {
    DA7213_MICBIAS_1_6V = 0,
    DA7213_MICBIAS_2_2V = 1,
    DA7213_MICBIAS_2_5V = 2,
    DA7213_MICBIAS_3_0V = 3,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da7213_dmic_data_sel {
    DA7213_DMIC_DATA_LRISE_RFALL = 0,
    DA7213_DMIC_DATA_LFALL_RRISE = 1,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da7213_dmic_samplephase {
    DA7213_DMIC_SAMPLE_ON_CLKEDGE = 0,
    DA7213_DMIC_SAMPLE_BETWEEN_CLKEDGE = 1,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da7213_dmic_clk_rate {
    DA7213_DMIC_CLK_3_0MHZ = 0,
    DA7213_DMIC_CLK_1_5MHZ = 1,
}

#[repr(C)]
pub struct da7213_platform_data {
    /* Mic Bias voltage */
    pub micbias1_lvl: da7213_micbias_voltage,
    pub micbias2_lvl: da7213_micbias_voltage,

    /* DMIC config */
    pub dmic_data_sel: da7213_dmic_data_sel,
    pub dmic_samplephase: da7213_dmic_samplephase,
    pub dmic_clk_rate: da7213_dmic_clk_rate,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
