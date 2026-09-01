// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020 The Linux Foundation. All rights reserved.
 *
 * lpass-hdmi.c -- ALSA SoC HDMI-CPU DAI driver for QTi LPASS HDMI
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type snd_pcm_format_t = c_int;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_field {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct lpass_tx_ctl {
    pub soft_reset: *mut regmap_field,
}

#[repr(C)]
pub struct lpass_vbit_ctl {
    pub replace_vbit: *mut regmap_field,
    pub vbit_stream: *mut regmap_field,
}

#[repr(C)]
pub struct lpass_hdmi_tx_dmactl {
    pub use_hw_chs: *mut regmap_field,
    pub hw_chs_sel: *mut regmap_field,
    pub use_hw_usr: *mut regmap_field,
    pub hw_usr_sel: *mut regmap_field,
}

#[repr(C)]
pub struct lpass_dp_metadata_ctl {
    pub mute: *mut regmap_field,
    pub as_sdp_cc: *mut regmap_field,
    pub as_sdp_ct: *mut regmap_field,
    pub aif_db4: *mut regmap_field,
    pub frequency: *mut regmap_field,
    pub mst_index: *mut regmap_field,
    pub dptx_index: *mut regmap_field,
}

#[repr(C)]
pub struct lpass_sstream_ctl {
    pub sstream_en: *mut regmap_field,
    pub dma_sel: *mut regmap_field,
    pub auto_bbit_en: *mut regmap_field,
    pub layout: *mut regmap_field,
    pub layout_sp: *mut regmap_field,
    pub dp_audio: *mut regmap_field,
    pub set_sp_on_en: *mut regmap_field,
    pub dp_sp_b_hw_en: *mut regmap_field,
    pub dp_staffing_en: *mut regmap_field,
}

#[repr(C)]
pub struct lpass_data {
    pub meta_ctl: *mut lpass_dp_metadata_ctl,
    pub sstream_ctl: *mut lpass_sstream_ctl,
    pub tx_ctl: *mut lpass_tx_ctl,
    pub hdmitx_legacy_en: *mut regmap_field,
    pub hdmitx_parity_calc_en: *mut regmap_field,
    pub vbit_ctl: *mut lpass_vbit_ctl,
    pub hdmitx_ch_msb: [*mut regmap_field; 1],
    pub hdmitx_ch_lsb: [*mut regmap_field; 1],
    pub hdmi_tx_dmactl: [*mut lpass_hdmi_tx_dmactl; 1],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub prepare: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub trigger: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            cmd: c_int,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_format_width(format: snd_pcm_format_t) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn regmap_field_write(field: *mut regmap_field, val: c_uint) -> c_int;
}

const EINVAL: c_int = 22;

unsafe extern "C" {
    static LPASS_DP_AUDIO_BITWIDTH16: c_uint;
    static LPASS_DP_AUDIO_BITWIDTH24: c_uint;
    static LPASS_SAMPLING_FREQ32: c_uint;
    static LPASS_SAMPLING_FREQ44: c_uint;
    static LPASS_SAMPLING_FREQ48: c_uint;
    static LPASS_DATA_FORMAT_LINEAR: c_uint;
    static LPASS_DATA_FORMAT_SHIFT: c_uint;
    static LPASS_DATA_FORMAT_MASK: c_uint;
    static LPASS_FREQ_BIT_SHIFT: c_uint;
    static LPASS_FREQ_BIT_MASK: c_uint;
    static LPASS_WORDLENGTH_MASK: c_uint;
    static LPASS_TX_CTL_RESET: c_uint;
    static LPASS_TX_CTL_CLEAR: c_uint;
    static LPASS_HDMITX_LEGACY_DISABLE: c_uint;
    static HDMITX_PARITY_CALC_EN: c_uint;
    static REPLACE_VBIT: c_uint;
    static LINEAR_PCM_DATA: c_uint;
    static HW_MODE: c_uint;
    static SW_MODE: c_uint;
    static LPASS_MUTE_ENABLE: c_uint;
    static LPASS_META_DEFAULT_VAL: c_uint;
    static LPASS_SSTREAM_DISABLE: c_uint;
    static LPASS_SSTREAM_DEFAULT_ENABLE: c_uint;
    static LPASS_SSTREAM_DEFAULT_DISABLE: c_uint;
    static LPASS_LAYOUT_SP_DEFAULT: c_uint;
    static LPASS_SSTREAM_ENABLE: c_uint;
    static LPASS_MUTE_DISABLE: c_uint;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
}

unsafe extern "C" fn lpass_hdmi_daiops_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let drvdata = snd_soc_dai_get_drvdata(dai) as *mut lpass_data;
    let format: snd_pcm_format_t = params_format(params);
    let rate: c_uint = params_rate(params);
    let channels: c_uint = params_channels(params);
    let bitwidth: c_int;
    let word_length: c_uint;
    let ch_sts_buf0: c_uint;
    let ch_sts_buf1: c_uint;
    let data_format: c_uint;
    let sampling_freq: c_uint;
    let ch: c_uint = 0;
    let meta_ctl: *mut lpass_dp_metadata_ctl = (*drvdata).meta_ctl;
    let sstream_ctl: *mut lpass_sstream_ctl = (*drvdata).sstream_ctl;
    let mut ret: c_int;

    bitwidth = snd_pcm_format_width(format);
    if bitwidth < 0 {
        dev_err(
            (*dai).dev,
            c"%s invalid bit width given : %d\n".as_ptr(),
            c"lpass_hdmi_daiops_hw_params".as_ptr(),
            bitwidth,
        );
        return bitwidth;
    }

    match bitwidth {
        16 => {
            word_length = LPASS_DP_AUDIO_BITWIDTH16;
        }
        24 => {
            word_length = LPASS_DP_AUDIO_BITWIDTH24;
        }
        _ => {
            dev_err(
                (*dai).dev,
                c"%s invalid bit width given : %d\n".as_ptr(),
                c"lpass_hdmi_daiops_hw_params".as_ptr(),
                bitwidth,
            );
            return -EINVAL;
        }
    }

    match rate {
        32000 => {
            sampling_freq = LPASS_SAMPLING_FREQ32;
        }
        44100 => {
            sampling_freq = LPASS_SAMPLING_FREQ44;
        }
        48000 => {
            sampling_freq = LPASS_SAMPLING_FREQ48;
        }
        _ => {
            dev_err(
                (*dai).dev,
                c"%s invalid bit width given : %d\n".as_ptr(),
                c"lpass_hdmi_daiops_hw_params".as_ptr(),
                bitwidth,
            );
            return -EINVAL;
        }
    }
    data_format = LPASS_DATA_FORMAT_LINEAR;
    ch_sts_buf0 = (((data_format << LPASS_DATA_FORMAT_SHIFT) & LPASS_DATA_FORMAT_MASK)
        | ((sampling_freq << LPASS_FREQ_BIT_SHIFT) & LPASS_FREQ_BIT_MASK));
    ch_sts_buf1 = word_length & LPASS_WORDLENGTH_MASK;

    ret = regmap_field_write((*(*drvdata).tx_ctl).soft_reset, LPASS_TX_CTL_RESET);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*(*drvdata).tx_ctl).soft_reset, LPASS_TX_CTL_CLEAR);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*drvdata).hdmitx_legacy_en, LPASS_HDMITX_LEGACY_DISABLE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*drvdata).hdmitx_parity_calc_en, HDMITX_PARITY_CALC_EN);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*(*drvdata).vbit_ctl).replace_vbit, REPLACE_VBIT);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*(*drvdata).vbit_ctl).vbit_stream, LINEAR_PCM_DATA);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*drvdata).hdmitx_ch_msb[0], ch_sts_buf1);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*drvdata).hdmitx_ch_lsb[0], ch_sts_buf0);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*(*drvdata).hdmi_tx_dmactl[0]).use_hw_chs, HW_MODE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*(*drvdata).hdmi_tx_dmactl[0]).hw_chs_sel, SW_MODE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*(*drvdata).hdmi_tx_dmactl[0]).use_hw_usr, HW_MODE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*(*drvdata).hdmi_tx_dmactl[0]).hw_usr_sel, SW_MODE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*meta_ctl).mute, LPASS_MUTE_ENABLE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*meta_ctl).as_sdp_cc, channels.wrapping_sub(1));
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*meta_ctl).as_sdp_ct, LPASS_META_DEFAULT_VAL);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*meta_ctl).aif_db4, LPASS_META_DEFAULT_VAL);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*meta_ctl).frequency, sampling_freq);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*meta_ctl).mst_index, LPASS_META_DEFAULT_VAL);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*meta_ctl).dptx_index, LPASS_META_DEFAULT_VAL);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*sstream_ctl).sstream_en, LPASS_SSTREAM_DISABLE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*sstream_ctl).dma_sel, ch);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*sstream_ctl).auto_bbit_en, LPASS_SSTREAM_DEFAULT_ENABLE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*sstream_ctl).layout, LPASS_SSTREAM_DEFAULT_DISABLE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*sstream_ctl).layout_sp, LPASS_LAYOUT_SP_DEFAULT);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*sstream_ctl).dp_audio, LPASS_SSTREAM_DEFAULT_ENABLE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*sstream_ctl).set_sp_on_en, LPASS_SSTREAM_DEFAULT_ENABLE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*sstream_ctl).dp_sp_b_hw_en, LPASS_SSTREAM_DEFAULT_ENABLE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*sstream_ctl).dp_staffing_en, LPASS_SSTREAM_DEFAULT_ENABLE);

    ret
}

unsafe extern "C" fn lpass_hdmi_daiops_prepare(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut ret: c_int;
    let drvdata = snd_soc_dai_get_drvdata(dai) as *mut lpass_data;

    ret = regmap_field_write((*(*drvdata).sstream_ctl).sstream_en, LPASS_SSTREAM_ENABLE);
    if ret != 0 {
        return ret;
    }

    ret = regmap_field_write((*(*drvdata).meta_ctl).mute, LPASS_MUTE_DISABLE);

    ret
}

unsafe extern "C" fn lpass_hdmi_daiops_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let drvdata = snd_soc_dai_get_drvdata(dai) as *mut lpass_data;
    let meta_ctl: *mut lpass_dp_metadata_ctl = (*drvdata).meta_ctl;
    let sstream_ctl: *mut lpass_sstream_ctl = (*drvdata).sstream_ctl;
    let mut ret: c_int = -EINVAL;

    if cmd == SNDRV_PCM_TRIGGER_START
        || cmd == SNDRV_PCM_TRIGGER_RESUME
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
    {
        ret = regmap_field_write((*sstream_ctl).sstream_en, LPASS_SSTREAM_ENABLE);
        if ret != 0 {
            return ret;
        }

        ret = regmap_field_write((*meta_ctl).mute, LPASS_MUTE_DISABLE);
        if ret != 0 {
            return ret;
        }
    } else if cmd == SNDRV_PCM_TRIGGER_STOP
        || cmd == SNDRV_PCM_TRIGGER_SUSPEND
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
    {
        ret = regmap_field_write((*sstream_ctl).sstream_en, LPASS_SSTREAM_DISABLE);
        if ret != 0 {
            return ret;
        }

        ret = regmap_field_write((*meta_ctl).mute, LPASS_MUTE_ENABLE);
        if ret != 0 {
            return ret;
        }

        ret = regmap_field_write((*sstream_ctl).dp_audio, 0);
        if ret != 0 {
            return ret;
        }
    }
    ret
}

#[unsafe(no_mangle)]
pub static asoc_qcom_lpass_hdmi_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(lpass_hdmi_daiops_hw_params),
    prepare: Some(lpass_hdmi_daiops_prepare),
    trigger: Some(lpass_hdmi_daiops_trigger),
};

// EXPORT_SYMBOL_GPL(asoc_qcom_lpass_hdmi_dai_ops);

// MODULE_DESCRIPTION("QTi LPASS HDMI Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
