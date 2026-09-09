/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * da7218.h - DA7218 ASoC Codec Driver Platform Data
 *
 * Copyright (c) 2015 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */

/* Mic Bias */
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da7218_micbias_voltage {
    DA7218_MICBIAS_1_2V = -1,
    DA7218_MICBIAS_1_6V,
    DA7218_MICBIAS_1_8V,
    DA7218_MICBIAS_2_0V,
    DA7218_MICBIAS_2_2V,
    DA7218_MICBIAS_2_4V,
    DA7218_MICBIAS_2_6V,
    DA7218_MICBIAS_2_8V,
    DA7218_MICBIAS_3_0V,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da7218_mic_amp_in_sel {
    DA7218_MIC_AMP_IN_SEL_DIFF = 0,
    DA7218_MIC_AMP_IN_SEL_SE_P,
    DA7218_MIC_AMP_IN_SEL_SE_N,
}

/* DMIC */
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da7218_dmic_data_sel {
    DA7218_DMIC_DATA_LRISE_RFALL = 0,
    DA7218_DMIC_DATA_LFALL_RRISE,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da7218_dmic_samplephase {
    DA7218_DMIC_SAMPLE_ON_CLKEDGE = 0,
    DA7218_DMIC_SAMPLE_BETWEEN_CLKEDGE,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da7218_dmic_clk_rate {
    DA7218_DMIC_CLK_3_0MHZ = 0,
    DA7218_DMIC_CLK_1_5MHZ,
}

/* Headphone Detect */
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da7218_hpldet_jack_rate {
    DA7218_HPLDET_JACK_RATE_5US = 0,
    DA7218_HPLDET_JACK_RATE_10US,
    DA7218_HPLDET_JACK_RATE_20US,
    DA7218_HPLDET_JACK_RATE_40US,
    DA7218_HPLDET_JACK_RATE_80US,
    DA7218_HPLDET_JACK_RATE_160US,
    DA7218_HPLDET_JACK_RATE_320US,
    DA7218_HPLDET_JACK_RATE_640US,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da7218_hpldet_jack_debounce {
    DA7218_HPLDET_JACK_DEBOUNCE_OFF = 0,
    DA7218_HPLDET_JACK_DEBOUNCE_2,
    DA7218_HPLDET_JACK_DEBOUNCE_3,
    DA7218_HPLDET_JACK_DEBOUNCE_4,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum da7218_hpldet_jack_thr {
    DA7218_HPLDET_JACK_THR_84PCT = 0,
    DA7218_HPLDET_JACK_THR_88PCT,
    DA7218_HPLDET_JACK_THR_92PCT,
    DA7218_HPLDET_JACK_THR_96PCT,
}

#[repr(C)]
pub struct da7218_hpldet_pdata {
    pub jack_rate: da7218_hpldet_jack_rate,
    pub jack_debounce: da7218_hpldet_jack_debounce,
    pub jack_thr: da7218_hpldet_jack_thr,
    pub comp_inv: bool,
    pub hyst: bool,
    pub discharge: bool,
}

#[repr(C)]
pub struct da7218_pdata {
    /* Mic */
    pub micbias1_lvl: da7218_micbias_voltage,
    pub micbias2_lvl: da7218_micbias_voltage,
    pub mic1_amp_in_sel: da7218_mic_amp_in_sel,
    pub mic2_amp_in_sel: da7218_mic_amp_in_sel,

    /* DMIC */
    pub dmic1_data_sel: da7218_dmic_data_sel,
    pub dmic2_data_sel: da7218_dmic_data_sel,
    pub dmic1_samplephase: da7218_dmic_samplephase,
    pub dmic2_samplephase: da7218_dmic_samplephase,
    pub dmic1_clk_rate: da7218_dmic_clk_rate,
    pub dmic2_clk_rate: da7218_dmic_clk_rate,

    /* HP Diff Supply - DA7217 only */
    pub hp_diff_single_supply: bool,

    /* HP Detect - DA7218 only */
    pub hpldet_pdata: *mut da7218_hpldet_pdata,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
