/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * da7219.h - DA7219 ASoC Codec Driver Platform Data
 *
 * Copyright (c) 2015 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */

/* Mic Bias */
#[repr(C)]
pub enum da7219_micbias_voltage {
    DA7219_MICBIAS_1_6V = 0,
    DA7219_MICBIAS_1_8V,
    DA7219_MICBIAS_2_0V,
    DA7219_MICBIAS_2_2V,
    DA7219_MICBIAS_2_4V,
    DA7219_MICBIAS_2_6V,
}

/* Mic input type */
#[repr(C)]
pub enum da7219_mic_amp_in_sel {
    DA7219_MIC_AMP_IN_SEL_DIFF = 0,
    DA7219_MIC_AMP_IN_SEL_SE_P,
    DA7219_MIC_AMP_IN_SEL_SE_N,
}

pub struct da7219_aad_pdata;

#[repr(C)]
pub enum da7219_dai_clks {
    DA7219_DAI_WCLK_IDX = 0,
    DA7219_DAI_BCLK_IDX,
    DA7219_DAI_NUM_CLKS,
}

#[repr(C)]
pub struct da7219_pdata {
    pub wakeup_source: bool,

    pub dai_clk_names: [*const core::ffi::c_char; DA7219_DAI_NUM_CLKS as usize],

    /* Mic */
    pub micbias_lvl: da7219_micbias_voltage,
    pub mic_amp_in_sel: da7219_mic_amp_in_sel,

    /* AAD */
    pub aad_pdata: *mut da7219_aad_pdata,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
