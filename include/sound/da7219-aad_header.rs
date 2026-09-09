/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * da7219-aad.h - DA7322 ASoC Codec AAD Driver Platform Data
 *
 * Copyright (c) 2015 Dialog Semiconductor Ltd.
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */

#[repr(C)]
pub enum da7219_aad_micbias_pulse_lvl {
    DA7219_AAD_MICBIAS_PULSE_LVL_OFF = 0,
    DA7219_AAD_MICBIAS_PULSE_LVL_2_8V = 6,
    DA7219_AAD_MICBIAS_PULSE_LVL_2_9V,
}

#[repr(C)]
pub enum da7219_aad_btn_cfg {
    DA7219_AAD_BTN_CFG_2MS = 1,
    DA7219_AAD_BTN_CFG_5MS,
    DA7219_AAD_BTN_CFG_10MS,
    DA7219_AAD_BTN_CFG_50MS,
    DA7219_AAD_BTN_CFG_100MS,
    DA7219_AAD_BTN_CFG_200MS,
    DA7219_AAD_BTN_CFG_500MS,
}

#[repr(C)]
pub enum da7219_aad_mic_det_thr {
    DA7219_AAD_MIC_DET_THR_200_OHMS = 0,
    DA7219_AAD_MIC_DET_THR_500_OHMS,
    DA7219_AAD_MIC_DET_THR_750_OHMS,
    DA7219_AAD_MIC_DET_THR_1000_OHMS,
}

#[repr(C)]
pub enum da7219_aad_jack_ins_deb {
    DA7219_AAD_JACK_INS_DEB_5MS = 0,
    DA7219_AAD_JACK_INS_DEB_10MS,
    DA7219_AAD_JACK_INS_DEB_20MS,
    DA7219_AAD_JACK_INS_DEB_50MS,
    DA7219_AAD_JACK_INS_DEB_100MS,
    DA7219_AAD_JACK_INS_DEB_200MS,
    DA7219_AAD_JACK_INS_DEB_500MS,
    DA7219_AAD_JACK_INS_DEB_1S,
}

#[repr(C)]
pub enum da7219_aad_jack_ins_det_pty {
    DA7219_AAD_JACK_INS_DET_PTY_LOW = 0,
    DA7219_AAD_JACK_INS_DET_PTY_HIGH,
}

#[repr(C)]
pub enum da7219_aad_jack_det_rate {
    DA7219_AAD_JACK_DET_RATE_32_64MS = 0,
    DA7219_AAD_JACK_DET_RATE_64_128MS,
    DA7219_AAD_JACK_DET_RATE_128_256MS,
    DA7219_AAD_JACK_DET_RATE_256_512MS,
}

#[repr(C)]
pub enum da7219_aad_jack_rem_deb {
    DA7219_AAD_JACK_REM_DEB_1MS = 0,
    DA7219_AAD_JACK_REM_DEB_5MS,
    DA7219_AAD_JACK_REM_DEB_10MS,
    DA7219_AAD_JACK_REM_DEB_20MS,
}

#[repr(C)]
pub enum da7219_aad_btn_avg {
    DA7219_AAD_BTN_AVG_1 = 0,
    DA7219_AAD_BTN_AVG_2,
    DA7219_AAD_BTN_AVG_4,
    DA7219_AAD_BTN_AVG_8,
}

#[repr(C)]
pub enum da7219_aad_adc_1bit_rpt {
    DA7219_AAD_ADC_1BIT_RPT_1 = 0,
    DA7219_AAD_ADC_1BIT_RPT_2,
    DA7219_AAD_ADC_1BIT_RPT_4,
    DA7219_AAD_ADC_1BIT_RPT_8,
}

#[repr(C)]
pub struct da7219_aad_pdata {
    pub irq: i32,

    pub micbias_pulse_lvl: da7219_aad_micbias_pulse_lvl,
    pub micbias_pulse_time: u32,
    pub btn_cfg: da7219_aad_btn_cfg,
    pub mic_det_thr: da7219_aad_mic_det_thr,
    pub jack_ins_deb: da7219_aad_jack_ins_deb,
    pub jack_ins_det_pty: da7219_aad_jack_ins_det_pty,
    pub jack_det_rate: da7219_aad_jack_det_rate,
    pub jack_rem_deb: da7219_aad_jack_rem_deb,

    pub a_d_btn_thr: u8,
    pub d_b_btn_thr: u8,
    pub b_c_btn_thr: u8,
    pub c_mic_btn_thr: u8,

    pub btn_avg: da7219_aad_btn_avg,
    pub adc_1bit_rpt: da7219_aad_adc_1bit_rpt,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
