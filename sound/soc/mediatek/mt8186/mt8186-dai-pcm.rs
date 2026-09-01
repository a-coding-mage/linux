// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI I2S Control
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>
//
// C dependencies translated as external Rust dependencies:
// <linux/regmap.h>, <sound/pcm_params.h>,
// "mt8186-afe-common.h", "mt8186-afe-gpio.h",
// "mt8186-interconnection.h".

#[repr(C)]
pub struct mtk_afe_pcm_priv {
    pub id: ::core::ffi::c_uint,
    pub fmt: ::core::ffi::c_uint,
    pub bck_invert: ::core::ffi::c_uint,
    pub lck_invert: ::core::ffi::c_uint,
}

#[repr(C)]
pub enum aud_tx_lch_rpt {
    AUD_TX_LCH_RPT_NO_REPEAT = 0,
    AUD_TX_LCH_RPT_REPEAT = 1,
}

#[repr(C)]
pub enum aud_vbt_16k_mode {
    AUD_VBT_16K_MODE_DISABLE = 0,
    AUD_VBT_16K_MODE_ENABLE = 1,
}

#[repr(C)]
pub enum aud_ext_modem {
    AUD_EXT_MODEM_SELECT_INTERNAL = 0,
    AUD_EXT_MODEM_SELECT_EXTERNAL = 1,
}

#[repr(C)]
pub enum aud_pcm_sync_type {
    /* bck sync length = 1 */
    AUD_PCM_ONE_BCK_CYCLE_SYNC = 0,
    /* bck sync length = PCM_INTF_CON1[9:13] */
    AUD_PCM_EXTENDED_BCK_CYCLE_SYNC = 1,
}

#[repr(C)]
pub enum aud_bt_mode {
    AUD_BT_MODE_DUAL_MIC_ON_TX = 0,
    AUD_BT_MODE_SINGLE_MIC_ON_TX = 1,
}

#[repr(C)]
pub enum aud_pcm_afifo_src {
    /* slave mode & external modem uses different crystal */
    AUD_PCM_AFIFO_ASRC = 0,
    /* slave mode & external modem uses the same crystal */
    AUD_PCM_AFIFO_AFIFO = 1,
}

#[repr(C)]
pub enum aud_pcm_clock_source {
    AUD_PCM_CLOCK_MASTER_MODE = 0,
    AUD_PCM_CLOCK_SLAVE_MODE = 1,
}

#[repr(C)]
pub enum aud_pcm_wlen {
    AUD_PCM_WLEN_PCM_32_BCK_CYCLES = 0,
    AUD_PCM_WLEN_PCM_64_BCK_CYCLES = 1,
}

#[repr(C)]
pub enum aud_pcm_24bit {
    AUD_PCM_24BIT_PCM_16_BITS = 0,
    AUD_PCM_24BIT_PCM_24_BITS = 1,
}

#[repr(C)]
pub enum aud_pcm_mode {
    AUD_PCM_MODE_PCM_MODE_8K = 0,
    AUD_PCM_MODE_PCM_MODE_16K = 1,
    AUD_PCM_MODE_PCM_MODE_32K = 2,
    AUD_PCM_MODE_PCM_MODE_48K = 3,
}

#[repr(C)]
pub enum aud_pcm_fmt {
    AUD_PCM_FMT_I2S = 0,
    AUD_PCM_FMT_EIAJ = 1,
    AUD_PCM_FMT_PCM_MODE_A = 2,
    AUD_PCM_FMT_PCM_MODE_B = 3,
}

#[repr(C)]
pub enum aud_bclk_out_inv {
    AUD_BCLK_OUT_INV_NO_INVERSE = 0,
    AUD_BCLK_OUT_INV_INVERSE = 1,
}

#[repr(C)]
pub enum aud_lrclk_out_inv {
    AUD_LRCLK_OUT_INV_NO_INVERSE = 0,
    AUD_LRCLK_OUT_INV_INVERSE = 1,
}

#[repr(C)]
pub enum aud_pcm_en {
    AUD_PCM_EN_DISABLE = 0,
    AUD_PCM_EN_ENABLE = 1,
}

/* dai component */
pub static mtk_pcm_1_playback_ch1_mix: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1 Switch", AFE_CONN7, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1 Switch", AFE_CONN7, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1 Switch", AFE_CONN7_1, I_DL4_CH1, 1, 0),
];

pub static mtk_pcm_1_playback_ch2_mix: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2 Switch", AFE_CONN8, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2 Switch", AFE_CONN8, I_DL2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2 Switch", AFE_CONN8_1, I_DL4_CH2, 1, 0),
];

pub unsafe extern "C" fn mtk_pcm_en_event(
    w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let cmpnt: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;

    let _ = kcontrol;
    dev_dbg!(
        (*afe).dev,
        "%s(), name %s, event 0x%x\n",
        __func__,
        (*w).name,
        event
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8186_afe_gpio_request((*afe).dev, true, MT8186_DAI_PCM, 0);
        }
        SND_SOC_DAPM_POST_PMD => {
            mt8186_afe_gpio_request((*afe).dev, false, MT8186_DAI_PCM, 0);
        }
        _ => {}
    }

    0
}

/* pcm in/out lpbk */
pub static pcm_lpbk_mux_map: [*const ::core::ffi::c_char; 2] = [
    c"Normal".as_ptr(),
    c"Lpbk".as_ptr(),
];

pub static mut pcm_lpbk_mux_map_value: [::core::ffi::c_int; 2] = [0, 1];

SOC_VALUE_ENUM_SINGLE_AUTODISABLE_DECL!(
    pcm_in_lpbk_mux_map_enum,
    PCM_INTF_CON1,
    PCM_I2S_PCM_LOOPBACK_SFT,
    1,
    pcm_lpbk_mux_map,
    pcm_lpbk_mux_map_value
);

pub static pcm_in_lpbk_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!("PCM In Lpbk Select", pcm_in_lpbk_mux_map_enum);

SOC_VALUE_ENUM_SINGLE_AUTODISABLE_DECL!(
    pcm_out_lpbk_mux_map_enum,
    PCM_INTF_CON1,
    PCM_I2S_PCM_LOOPBACK_SFT,
    1,
    pcm_lpbk_mux_map,
    pcm_lpbk_mux_map_value
);

pub static pcm_out_lpbk_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!("PCM Out Lpbk Select", pcm_out_lpbk_mux_map_enum);

pub static mtk_dai_pcm_widgets: [snd_soc_dapm_widget; 5] = [
    /* inter-connections */
    SND_SOC_DAPM_MIXER!(
        "PCM_1_PB_CH1",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_1_playback_ch1_mix,
        ARRAY_SIZE!(mtk_pcm_1_playback_ch1_mix)
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_1_PB_CH2",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_1_playback_ch2_mix,
        ARRAY_SIZE!(mtk_pcm_1_playback_ch2_mix)
    ),
    SND_SOC_DAPM_SUPPLY!(
        "PCM_1_EN",
        PCM_INTF_CON1,
        PCM_EN_SFT,
        0,
        mtk_pcm_en_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
    /* pcm in lpbk */
    SND_SOC_DAPM_MUX!(
        "PCM_In_Lpbk_Mux",
        SND_SOC_NOPM,
        0,
        0,
        &pcm_in_lpbk_mux_control
    ),
    /* pcm out lpbk */
    SND_SOC_DAPM_MUX!(
        "PCM_Out_Lpbk_Mux",
        SND_SOC_NOPM,
        0,
        0,
        &pcm_out_lpbk_mux_control
    ),
];

pub static mtk_dai_pcm_routes: [snd_soc_dapm_route; 12] = [
    snd_soc_dapm_route { sink: c"PCM 1 Playback".as_ptr(), control: ::core::ptr::null(), source: c"PCM_1_PB_CH1".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Playback".as_ptr(), control: ::core::ptr::null(), source: c"PCM_1_PB_CH2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Playback".as_ptr(), control: ::core::ptr::null(), source: c"PCM_1_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Capture".as_ptr(), control: ::core::ptr::null(), source: c"PCM_1_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH1".as_ptr(), control: c"DL2_CH1 Switch".as_ptr(), source: c"DL2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH2".as_ptr(), control: c"DL2_CH2 Switch".as_ptr(), source: c"DL2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH1".as_ptr(), control: c"DL4_CH1 Switch".as_ptr(), source: c"DL4".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH2".as_ptr(), control: c"DL4_CH2 Switch".as_ptr(), source: c"DL4".as_ptr() },
    /* pcm out lpbk */
    snd_soc_dapm_route { sink: c"PCM_Out_Lpbk_Mux".as_ptr(), control: c"Lpbk".as_ptr(), source: c"PCM 1 Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"I2S0".as_ptr(), control: ::core::ptr::null(), source: c"PCM_Out_Lpbk_Mux".as_ptr() },
    /* pcm in lpbk */
    snd_soc_dapm_route { sink: c"PCM_In_Lpbk_Mux".as_ptr(), control: c"Lpbk".as_ptr(), source: c"PCM 1 Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"I2S3".as_ptr(), control: ::core::ptr::null(), source: c"PCM_In_Lpbk_Mux".as_ptr() },
];

/* dai ops */
pub unsafe extern "C" fn mtk_dai_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv: *mut mt8186_afe_private = (*afe).platform_priv as *mut mt8186_afe_private;
    let p: *mut snd_soc_dapm_widget = snd_soc_dai_get_widget_playback(dai);
    let c: *mut snd_soc_dapm_widget = snd_soc_dai_get_widget_capture(dai);
    let pcm_id: ::core::ffi::c_int = (*dai).id;
    let pcm_priv: *mut mtk_afe_pcm_priv = (*afe_priv).dai_priv[pcm_id as usize] as *mut mtk_afe_pcm_priv;
    let rate: ::core::ffi::c_uint = params_rate(params);
    let rate_reg: ::core::ffi::c_uint = mt8186_rate_transform((*afe).dev, rate, (*dai).id);
    let format: snd_pcm_format_t = params_format(params);
    let data_width: ::core::ffi::c_uint = snd_pcm_format_width(format) as ::core::ffi::c_uint;
    let wlen_width: ::core::ffi::c_uint =
        snd_pcm_format_physical_width(format) as ::core::ffi::c_uint;
    let mut pcm_con: ::core::ffi::c_uint = 0;

    dev_dbg!(
        (*afe).dev,
        "%s(), id %d, stream %d, widget active p %d, c %d\n",
        __func__,
        (*dai).id,
        (*substream).stream,
        (*p).active,
        (*c).active
    );
    dev_dbg!(
        (*afe).dev,
        "%s(), rate %d, rate_reg %d, data_width %d, wlen_width %d\n",
        __func__,
        rate,
        rate_reg,
        data_width,
        wlen_width
    );

    if (*p).active != 0 || (*c).active != 0 {
        return 0;
    }

    match (*dai).id {
        MT8186_DAI_PCM => {
            pcm_con |= (aud_tx_lch_rpt::AUD_TX_LCH_RPT_NO_REPEAT as ::core::ffi::c_uint) << PCM_TX_LCH_RPT_SFT;
            pcm_con |= (aud_vbt_16k_mode::AUD_VBT_16K_MODE_DISABLE as ::core::ffi::c_uint) << PCM_VBT_16K_MODE_SFT;
            pcm_con |= (aud_ext_modem::AUD_EXT_MODEM_SELECT_EXTERNAL as ::core::ffi::c_uint) << PCM_EXT_MODEM_SFT;
            pcm_con |= (aud_pcm_sync_type::AUD_PCM_ONE_BCK_CYCLE_SYNC as ::core::ffi::c_uint) << PCM_SYNC_TYPE_SFT;
            pcm_con |= (aud_bt_mode::AUD_BT_MODE_DUAL_MIC_ON_TX as ::core::ffi::c_uint) << PCM_BT_MODE_SFT;
            pcm_con |= (aud_pcm_afifo_src::AUD_PCM_AFIFO_AFIFO as ::core::ffi::c_uint) << PCM_BYP_ASRC_SFT;
            pcm_con |= (aud_pcm_clock_source::AUD_PCM_CLOCK_MASTER_MODE as ::core::ffi::c_uint) << PCM_SLAVE_SFT;
            pcm_con |= 0 << PCM_SYNC_LENGTH_SFT;

            /* sampling rate */
            pcm_con |= rate_reg << PCM_MODE_SFT;

            /* format */
            pcm_con |= (*pcm_priv).fmt << PCM_FMT_SFT;

            /* 24bit data width */
            if data_width > 16 {
                pcm_con |= (aud_pcm_24bit::AUD_PCM_24BIT_PCM_24_BITS as ::core::ffi::c_uint) << PCM_24BIT_SFT;
            } else {
                pcm_con |= (aud_pcm_24bit::AUD_PCM_24BIT_PCM_16_BITS as ::core::ffi::c_uint) << PCM_24BIT_SFT;
            }

            /* wlen width*/
            if wlen_width > 16 {
                pcm_con |= (aud_pcm_wlen::AUD_PCM_WLEN_PCM_64_BCK_CYCLES as ::core::ffi::c_uint) << PCM_WLEN_SFT;
            } else {
                pcm_con |= (aud_pcm_wlen::AUD_PCM_WLEN_PCM_32_BCK_CYCLES as ::core::ffi::c_uint) << PCM_WLEN_SFT;
            }

            /* clock invert */
            pcm_con |= (*pcm_priv).lck_invert << PCM_SYNC_OUT_INV_SFT;
            pcm_con |= (*pcm_priv).bck_invert << PCM_BCLK_OUT_INV_SFT;

            regmap_update_bits((*afe).regmap, PCM_INTF_CON1, 0xfffffffe, pcm_con);
        }
        _ => {
            dev_err!((*afe).dev, "%s(), id %d not support\n", __func__, (*dai).id);
            return -EINVAL;
        }
    }

    0
}

pub unsafe extern "C" fn mtk_dai_pcm_set_fmt(
    dai: *mut snd_soc_dai,
    fmt: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv: *mut mt8186_afe_private = (*afe).platform_priv as *mut mt8186_afe_private;
    let pcm_priv: *mut mtk_afe_pcm_priv =
        (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_afe_pcm_priv;

    /* DAI mode*/
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            (*pcm_priv).fmt = aud_pcm_fmt::AUD_PCM_FMT_I2S as ::core::ffi::c_uint;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            (*pcm_priv).fmt = aud_pcm_fmt::AUD_PCM_FMT_EIAJ as ::core::ffi::c_uint;
        }
        SND_SOC_DAIFMT_DSP_A => {
            (*pcm_priv).fmt = aud_pcm_fmt::AUD_PCM_FMT_PCM_MODE_A as ::core::ffi::c_uint;
        }
        SND_SOC_DAIFMT_DSP_B => {
            (*pcm_priv).fmt = aud_pcm_fmt::AUD_PCM_FMT_PCM_MODE_B as ::core::ffi::c_uint;
        }
        _ => {
            (*pcm_priv).fmt = aud_pcm_fmt::AUD_PCM_FMT_I2S as ::core::ffi::c_uint;
        }
    }

    /* DAI clock inversion*/
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            (*pcm_priv).bck_invert =
                aud_bclk_out_inv::AUD_BCLK_OUT_INV_NO_INVERSE as ::core::ffi::c_uint;
            (*pcm_priv).lck_invert =
                aud_lrclk_out_inv::AUD_LRCLK_OUT_INV_NO_INVERSE as ::core::ffi::c_uint;
        }
        SND_SOC_DAIFMT_NB_IF => {
            (*pcm_priv).bck_invert =
                aud_bclk_out_inv::AUD_BCLK_OUT_INV_NO_INVERSE as ::core::ffi::c_uint;
            (*pcm_priv).lck_invert =
                aud_lrclk_out_inv::AUD_LRCLK_OUT_INV_INVERSE as ::core::ffi::c_uint;
        }
        SND_SOC_DAIFMT_IB_NF => {
            (*pcm_priv).bck_invert =
                aud_bclk_out_inv::AUD_BCLK_OUT_INV_INVERSE as ::core::ffi::c_uint;
            (*pcm_priv).lck_invert =
                aud_lrclk_out_inv::AUD_LRCLK_OUT_INV_NO_INVERSE as ::core::ffi::c_uint;
        }
        SND_SOC_DAIFMT_IB_IF => {
            (*pcm_priv).bck_invert =
                aud_bclk_out_inv::AUD_BCLK_OUT_INV_INVERSE as ::core::ffi::c_uint;
            (*pcm_priv).lck_invert =
                aud_lrclk_out_inv::AUD_LRCLK_OUT_INV_INVERSE as ::core::ffi::c_uint;
        }
        _ => {
            (*pcm_priv).bck_invert =
                aud_bclk_out_inv::AUD_BCLK_OUT_INV_NO_INVERSE as ::core::ffi::c_uint;
            (*pcm_priv).lck_invert =
                aud_lrclk_out_inv::AUD_LRCLK_OUT_INV_NO_INVERSE as ::core::ffi::c_uint;
        }
    }

    0
}

pub static mtk_dai_pcm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_pcm_hw_params),
    set_fmt: Some(mtk_dai_pcm_set_fmt),
};

/* dai driver */
pub const MTK_PCM_RATES: ::core::ffi::c_uint =
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000;

pub const MTK_PCM_FORMATS: ::core::ffi::c_ulong =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

pub static mut mtk_dai_pcm_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"PCM 1".as_ptr(),
    id: MT8186_DAI_PCM,
    playback: snd_soc_pcm_stream {
        stream_name: c"PCM 1 Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: MTK_PCM_RATES,
        formats: MTK_PCM_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"PCM 1 Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: MTK_PCM_RATES,
        formats: MTK_PCM_FORMATS,
    },
    ops: &mtk_dai_pcm_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
}];

pub unsafe extern "C" fn init_pcm_priv_data(afe: *mut mtk_base_afe) -> *mut mtk_afe_pcm_priv {
    let pcm_priv: *mut mtk_afe_pcm_priv;

    pcm_priv = devm_kzalloc(
        (*afe).dev,
        ::core::mem::size_of::<mtk_afe_pcm_priv>(),
        GFP_KERNEL,
    ) as *mut mtk_afe_pcm_priv;
    if pcm_priv.is_null() {
        return ::core::ptr::null_mut();
    }

    (*pcm_priv).id = MT8186_DAI_PCM as ::core::ffi::c_uint;
    (*pcm_priv).fmt = aud_pcm_fmt::AUD_PCM_FMT_I2S as ::core::ffi::c_uint;
    (*pcm_priv).bck_invert =
        aud_bclk_out_inv::AUD_BCLK_OUT_INV_NO_INVERSE as ::core::ffi::c_uint;
    (*pcm_priv).lck_invert =
        aud_lrclk_out_inv::AUD_LRCLK_OUT_INV_NO_INVERSE as ::core::ffi::c_uint;

    pcm_priv
}

pub unsafe extern "C" fn mt8186_dai_pcm_register(
    afe: *mut mtk_base_afe,
) -> ::core::ffi::c_int {
    let afe_priv: *mut mt8186_afe_private = (*afe).platform_priv as *mut mt8186_afe_private;
    let mut pcm_priv: *mut mtk_afe_pcm_priv;
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, ::core::mem::size_of_val(&*dai), GFP_KERNEL)
        as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_pcm_driver.as_mut_ptr();
    (*dai).num_dai_drivers = ARRAY_SIZE!(mtk_dai_pcm_driver);

    (*dai).dapm_widgets = mtk_dai_pcm_widgets.as_ptr();
    (*dai).num_dapm_widgets = ARRAY_SIZE!(mtk_dai_pcm_widgets);
    (*dai).dapm_routes = mtk_dai_pcm_routes.as_ptr();
    (*dai).num_dapm_routes = ARRAY_SIZE!(mtk_dai_pcm_routes);

    pcm_priv = init_pcm_priv_data(afe);
    if pcm_priv.is_null() {
        return -ENOMEM;
    }

    (*afe_priv).dai_priv[MT8186_DAI_PCM as usize] = pcm_priv as *mut ::core::ffi::c_void;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
