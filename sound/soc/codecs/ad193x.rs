// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * AD193X Audio Codec driver supporting AD1936/7/8/9
 *
 * Copyright 2010 Analog Devices Inc.
 */

/* C include dependencies removed:
 * linux/module.h, linux/kernel.h, linux/device.h, linux/regmap.h,
 * linux/slab.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
 * sound/initval.h, sound/soc.h, sound/tlv.h, and ad193x.h.
 */

/* codec private data */
#[repr(C)]
struct ad193x_priv {
    regmap: *mut regmap,
    type_: ad193x_type,
    sysclk: core::ffi::c_int,
}

/*
 * AD193X volume/mute/de-emphasis etc. controls
 */
static ad193x_deemp_0: &[u8] = b"None\0";
static ad193x_deemp_1: &[u8] = b"48kHz\0";
static ad193x_deemp_2: &[u8] = b"44.1kHz\0";
static ad193x_deemp_3: &[u8] = b"32kHz\0";
static ad193x_deemp: [*const core::ffi::c_char; 4] = [
    ad193x_deemp_0.as_ptr() as *const core::ffi::c_char,
    ad193x_deemp_1.as_ptr() as *const core::ffi::c_char,
    ad193x_deemp_2.as_ptr() as *const core::ffi::c_char,
    ad193x_deemp_3.as_ptr() as *const core::ffi::c_char,
];

SOC_ENUM_SINGLE_DECL!(ad193x_deemp_enum, AD193X_DAC_CTRL2, 1, ad193x_deemp);

DECLARE_TLV_DB_MINMAX!(adau193x_tlv, -9563, 0);

static ad193x_sb: [core::ffi::c_uint; 1] = [32];

static mut constr: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: ad193x_sb.as_ptr(),
    count: ARRAY_SIZE!(ad193x_sb),
};

static ad193x_snd_controls: [snd_kcontrol_new; 9] = [
    /* DAC volume control */
    SOC_DOUBLE_R_TLV!("DAC1 Volume", AD193X_DAC_L1_VOL, AD193X_DAC_R1_VOL, 0, 0xFF, 1, adau193x_tlv),
    SOC_DOUBLE_R_TLV!("DAC2 Volume", AD193X_DAC_L2_VOL, AD193X_DAC_R2_VOL, 0, 0xFF, 1, adau193x_tlv),
    SOC_DOUBLE_R_TLV!("DAC3 Volume", AD193X_DAC_L3_VOL, AD193X_DAC_R3_VOL, 0, 0xFF, 1, adau193x_tlv),
    SOC_DOUBLE_R_TLV!("DAC4 Volume", AD193X_DAC_L4_VOL, AD193X_DAC_R4_VOL, 0, 0xFF, 1, adau193x_tlv),

    /* DAC switch control */
    SOC_DOUBLE!("DAC1 Switch", AD193X_DAC_CHNL_MUTE, AD193X_DACL1_MUTE, AD193X_DACR1_MUTE, 1, 1),
    SOC_DOUBLE!("DAC2 Switch", AD193X_DAC_CHNL_MUTE, AD193X_DACL2_MUTE, AD193X_DACR2_MUTE, 1, 1),
    SOC_DOUBLE!("DAC3 Switch", AD193X_DAC_CHNL_MUTE, AD193X_DACL3_MUTE, AD193X_DACR3_MUTE, 1, 1),
    SOC_DOUBLE!("DAC4 Switch", AD193X_DAC_CHNL_MUTE, AD193X_DACL4_MUTE, AD193X_DACR4_MUTE, 1, 1),

    /* DAC de-emphasis */
    SOC_ENUM!("Playback Deemphasis", ad193x_deemp_enum),
];

static ad193x_adc_snd_controls: [snd_kcontrol_new; 3] = [
    /* ADC switch control */
    SOC_DOUBLE!("ADC1 Switch", AD193X_ADC_CTRL0, AD193X_ADCL1_MUTE, AD193X_ADCR1_MUTE, 1, 1),
    SOC_DOUBLE!("ADC2 Switch", AD193X_ADC_CTRL0, AD193X_ADCL2_MUTE, AD193X_ADCR2_MUTE, 1, 1),

    /* ADC high-pass filter */
    SOC_SINGLE!("ADC High Pass Filter Switch", AD193X_ADC_CTRL0, AD193X_ADC_HIGHPASS_FILTER, 1, 0),
];

static ad193x_dapm_widgets: [snd_soc_dapm_widget; 8] = [
    SND_SOC_DAPM_DAC!("DAC", "Playback", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_PGA!("DAC Output", AD193X_DAC_CTRL0, 0, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("PLL_PWR", AD193X_PLL_CLK_CTRL0, 0, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("SYSCLK", AD193X_PLL_CLK_CTRL0, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_VMID!("VMID"),
    SND_SOC_DAPM_OUTPUT!("DAC1OUT"),
    SND_SOC_DAPM_OUTPUT!("DAC2OUT"),
    SND_SOC_DAPM_OUTPUT!("DAC3OUT"),
    SND_SOC_DAPM_OUTPUT!("DAC4OUT"),
];

static ad193x_adc_widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_ADC!("ADC", "Capture", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_SUPPLY!("ADC_PWR", AD193X_ADC_CTRL0, 0, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_INPUT!("ADC1IN"),
    SND_SOC_DAPM_INPUT!("ADC2IN"),
];

unsafe extern "C" fn ad193x_check_pll(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*source).dapm);
    let ad193x: *mut ad193x_priv = snd_soc_component_get_drvdata(component) as *mut ad193x_priv;

    ((*ad193x).sysclk != 0) as core::ffi::c_int
}

static audio_paths: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: c_str!("DAC"), control: core::ptr::null(), source: c_str!("SYSCLK"), connected: None },
    snd_soc_dapm_route { sink: c_str!("DAC Output"), control: core::ptr::null(), source: c_str!("DAC"), connected: None },
    snd_soc_dapm_route { sink: c_str!("DAC Output"), control: core::ptr::null(), source: c_str!("VMID"), connected: None },
    snd_soc_dapm_route { sink: c_str!("DAC1OUT"), control: core::ptr::null(), source: c_str!("DAC Output"), connected: None },
    snd_soc_dapm_route { sink: c_str!("DAC2OUT"), control: core::ptr::null(), source: c_str!("DAC Output"), connected: None },
    snd_soc_dapm_route { sink: c_str!("DAC3OUT"), control: core::ptr::null(), source: c_str!("DAC Output"), connected: None },
    snd_soc_dapm_route { sink: c_str!("DAC4OUT"), control: core::ptr::null(), source: c_str!("DAC Output"), connected: None },
    snd_soc_dapm_route { sink: c_str!("SYSCLK"), control: core::ptr::null(), source: c_str!("PLL_PWR"), connected: Some(ad193x_check_pll) },
];

static ad193x_adc_audio_paths: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: c_str!("ADC"), control: core::ptr::null(), source: c_str!("SYSCLK"), connected: None },
    snd_soc_dapm_route { sink: c_str!("ADC"), control: core::ptr::null(), source: c_str!("ADC_PWR"), connected: None },
    snd_soc_dapm_route { sink: c_str!("ADC"), control: core::ptr::null(), source: c_str!("ADC1IN"), connected: None },
    snd_soc_dapm_route { sink: c_str!("ADC"), control: core::ptr::null(), source: c_str!("ADC2IN"), connected: None },
];

#[inline]
unsafe fn ad193x_has_adc(ad193x: *const ad193x_priv) -> bool {
    match (*ad193x).type_ {
        AD1933 | AD1934 => return false,
        _ => {}
    }

    true
}

/*
 * DAI ops entries
 */

unsafe extern "C" fn ad193x_mute(
    dai: *mut snd_soc_dai,
    mute: core::ffi::c_int,
    _direction: core::ffi::c_int,
) -> core::ffi::c_int {
    let ad193x: *mut ad193x_priv =
        snd_soc_component_get_drvdata((*dai).component) as *mut ad193x_priv;

    if mute != 0 {
        regmap_update_bits(
            (*ad193x).regmap,
            AD193X_DAC_CTRL2,
            AD193X_DAC_MASTER_MUTE,
            AD193X_DAC_MASTER_MUTE,
        );
    } else {
        regmap_update_bits((*ad193x).regmap, AD193X_DAC_CTRL2, AD193X_DAC_MASTER_MUTE, 0);
    }

    0
}

unsafe extern "C" fn ad193x_set_tdm_slot(
    dai: *mut snd_soc_dai,
    _tx_mask: core::ffi::c_uint,
    _rx_mask: core::ffi::c_uint,
    slots: core::ffi::c_int,
    _width: core::ffi::c_int,
) -> core::ffi::c_int {
    let ad193x: *mut ad193x_priv =
        snd_soc_component_get_drvdata((*dai).component) as *mut ad193x_priv;
    let channels: core::ffi::c_uint;

    match slots {
        2 => channels = AD193X_2_CHANNELS,
        4 => channels = AD193X_4_CHANNELS,
        8 => channels = AD193X_8_CHANNELS,
        16 => channels = AD193X_16_CHANNELS,
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*ad193x).regmap,
        AD193X_DAC_CTRL1,
        AD193X_DAC_CHAN_MASK,
        channels << AD193X_DAC_CHAN_SHFT,
    );
    if ad193x_has_adc(ad193x) {
        regmap_update_bits(
            (*ad193x).regmap,
            AD193X_ADC_CTRL2,
            AD193X_ADC_CHAN_MASK,
            channels << AD193X_ADC_CHAN_SHFT,
        );
    }

    0
}

unsafe extern "C" fn ad193x_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: core::ffi::c_uint,
) -> core::ffi::c_int {
    let ad193x: *mut ad193x_priv =
        snd_soc_component_get_drvdata((*codec_dai).component) as *mut ad193x_priv;
    let mut adc_serfmt: core::ffi::c_uint = 0;
    let mut dac_serfmt: core::ffi::c_uint = 0;
    let mut adc_fmt: core::ffi::c_uint = 0;
    let mut dac_fmt: core::ffi::c_uint = 0;

    /* At present, the driver only support AUX ADC mode(SND_SOC_DAIFMT_I2S
     * with TDM), ADC&DAC TDM mode(SND_SOC_DAIFMT_DSP_A) and DAC I2S mode
     * (SND_SOC_DAIFMT_I2S)
     */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            adc_serfmt |= AD193X_ADC_SERFMT_TDM;
            dac_serfmt |= AD193X_DAC_SERFMT_STEREO;
        }
        SND_SOC_DAIFMT_DSP_A => {
            adc_serfmt |= AD193X_ADC_SERFMT_AUX;
            dac_serfmt |= AD193X_DAC_SERFMT_TDM;
        }
        _ => {
            if ad193x_has_adc(ad193x) {
                return -EINVAL;
            }
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            /* normal bit clock + frame */
        }
        SND_SOC_DAIFMT_NB_IF => {
            /* normal bclk + invert frm */
            adc_fmt |= AD193X_ADC_LEFT_HIGH;
            dac_fmt |= AD193X_DAC_LEFT_HIGH;
        }
        SND_SOC_DAIFMT_IB_NF => {
            /* invert bclk + normal frm */
            adc_fmt |= AD193X_ADC_BCLK_INV;
            dac_fmt |= AD193X_DAC_BCLK_INV;
        }
        SND_SOC_DAIFMT_IB_IF => {
            /* invert bclk + frm */
            adc_fmt |= AD193X_ADC_LEFT_HIGH;
            adc_fmt |= AD193X_ADC_BCLK_INV;
            dac_fmt |= AD193X_DAC_LEFT_HIGH;
            dac_fmt |= AD193X_DAC_BCLK_INV;
        }
        _ => return -EINVAL,
    }

    /* For DSP_*, LRCLK's polarity must be inverted */
    if (fmt & SND_SOC_DAIFMT_DSP_A) != 0 {
        dac_fmt ^= AD193X_DAC_LEFT_HIGH;
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            adc_fmt |= AD193X_ADC_LCR_MASTER;
            adc_fmt |= AD193X_ADC_BCLK_MASTER;
            dac_fmt |= AD193X_DAC_LCR_MASTER;
            dac_fmt |= AD193X_DAC_BCLK_MASTER;
        }
        SND_SOC_DAIFMT_CBC_CFP => {
            adc_fmt |= AD193X_ADC_LCR_MASTER;
            dac_fmt |= AD193X_DAC_LCR_MASTER;
        }
        SND_SOC_DAIFMT_CBP_CFC => {
            adc_fmt |= AD193X_ADC_BCLK_MASTER;
            dac_fmt |= AD193X_DAC_BCLK_MASTER;
        }
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    if ad193x_has_adc(ad193x) {
        regmap_update_bits((*ad193x).regmap, AD193X_ADC_CTRL1, AD193X_ADC_SERFMT_MASK, adc_serfmt);
        regmap_update_bits((*ad193x).regmap, AD193X_ADC_CTRL2, AD193X_ADC_FMT_MASK, adc_fmt);
    }
    regmap_update_bits((*ad193x).regmap, AD193X_DAC_CTRL0, AD193X_DAC_SERFMT_MASK, dac_serfmt);
    regmap_update_bits((*ad193x).regmap, AD193X_DAC_CTRL1, AD193X_DAC_FMT_MASK, dac_fmt);

    0
}

unsafe extern "C" fn ad193x_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: core::ffi::c_int,
    freq: core::ffi::c_uint,
    dir: core::ffi::c_int,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let ad193x: *mut ad193x_priv = snd_soc_component_get_drvdata(component) as *mut ad193x_priv;

    if clk_id == AD193X_SYSCLK_MCLK {
        /* MCLK must be 512 x fs */
        if dir == SND_SOC_CLOCK_OUT || freq != 24576000 {
            return -EINVAL;
        }

        regmap_update_bits(
            (*ad193x).regmap,
            AD193X_PLL_CLK_CTRL1,
            AD193X_PLL_SRC_MASK,
            AD193X_PLL_DAC_SRC_MCLK | AD193X_PLL_CLK_SRC_MCLK,
        );

        snd_soc_dapm_sync(dapm);
        return 0;
    }
    match freq {
        12288000 | 18432000 | 24576000 | 36864000 => {
            (*ad193x).sysclk = freq as core::ffi::c_int;
            return 0;
        }
        _ => {}
    }
    -EINVAL
}

unsafe extern "C" fn ad193x_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let mut word_len: core::ffi::c_int = 0;
    let mut master_rate: core::ffi::c_int = 0;
    let component: *mut snd_soc_component = (*dai).component;
    let ad193x: *mut ad193x_priv = snd_soc_component_get_drvdata(component) as *mut ad193x_priv;
    let is_playback: bool = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let dacc0: u8;

    dev_dbg!(
        (*dai).dev,
        "%s() rate=%u format=%#x width=%u channels=%u\n",
        __func__,
        params_rate(params),
        params_format(params),
        params_width(params),
        params_channels(params)
    );

    /* bit size */
    match params_width(params) {
        16 => word_len = 3,
        20 => word_len = 1,
        24 | 32 => word_len = 0,
        _ => {}
    }

    match (*ad193x).sysclk {
        12288000 => master_rate = AD193X_PLL_INPUT_256,
        18432000 => master_rate = AD193X_PLL_INPUT_384,
        24576000 => master_rate = AD193X_PLL_INPUT_512,
        36864000 => master_rate = AD193X_PLL_INPUT_768,
        _ => {}
    }

    if is_playback {
        match params_rate(params) {
            48000 => dacc0 = AD193X_DAC_SR_48,
            96000 => dacc0 = AD193X_DAC_SR_96,
            192000 => dacc0 = AD193X_DAC_SR_192,
            _ => {
                dev_err!((*dai).dev, "invalid sampling rate: %d\n", params_rate(params));
                return -EINVAL;
            }
        }

        regmap_update_bits((*ad193x).regmap, AD193X_DAC_CTRL0, AD193X_DAC_SR_MASK, dacc0 as core::ffi::c_uint);
    }

    regmap_update_bits(
        (*ad193x).regmap,
        AD193X_PLL_CLK_CTRL0,
        AD193X_PLL_INPUT_MASK,
        master_rate as core::ffi::c_uint,
    );

    regmap_update_bits(
        (*ad193x).regmap,
        AD193X_DAC_CTRL2,
        AD193X_DAC_WORD_LEN_MASK,
        (word_len as core::ffi::c_uint) << AD193X_DAC_WORD_LEN_SHFT,
    );

    if ad193x_has_adc(ad193x) {
        regmap_update_bits(
            (*ad193x).regmap,
            AD193X_ADC_CTRL1,
            AD193X_ADC_WORD_LEN_MASK,
            word_len as core::ffi::c_uint,
        );
    }

    0
}

unsafe extern "C" fn ad193x_startup(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_SAMPLE_BITS,
        core::ptr::addr_of!(constr),
    )
}

static ad193x_selectable_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_I2S
        | SND_SOC_POSSIBLE_DAIFMT_DSP_A
        | SND_SOC_POSSIBLE_DAIFMT_NB_NF
        | SND_SOC_POSSIBLE_DAIFMT_NB_IF
        | SND_SOC_POSSIBLE_DAIFMT_IB_NF
        | SND_SOC_POSSIBLE_DAIFMT_IB_IF;

static ad193x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(ad193x_startup),
    hw_params: Some(ad193x_hw_params),
    mute_stream: Some(ad193x_mute),
    set_tdm_slot: Some(ad193x_set_tdm_slot),
    set_sysclk: Some(ad193x_set_dai_sysclk),
    set_fmt: Some(ad193x_set_dai_fmt),
    auto_selectable_formats: &ad193x_selectable_formats,
    num_auto_selectable_formats: 1,
    no_capture_mute: 1,
};

/* codec DAI instance */
static mut ad193x_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c_str!("ad193x-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("Playback"),
        channels_min: 2,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000,
        formats: SNDRV_PCM_FMTBIT_S32_LE
            | SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c_str!("Capture"),
        channels_min: 2,
        channels_max: 4,
        rates: SNDRV_PCM_RATE_48000,
        formats: SNDRV_PCM_FMTBIT_S32_LE
            | SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &ad193x_dai_ops,
};

/* codec DAI instance for DAC only */
static mut ad193x_no_adc_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c_str!("ad193x-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("Playback"),
        channels_min: 2,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000,
        formats: SNDRV_PCM_FMTBIT_S32_LE
            | SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &ad193x_dai_ops,
};

/* codec register values to set after reset */
unsafe fn ad193x_reg_default_init(ad193x: *mut ad193x_priv) {
    static reg_init: [reg_sequence; 14] = [
        reg_sequence { reg: 0, def: 0x99 }, /* PLL_CLK_CTRL0: pll input: mclki/xi 12.288Mhz */
        reg_sequence { reg: 1, def: 0x04 }, /* PLL_CLK_CTRL1: no on-chip Vref */
        reg_sequence { reg: 2, def: 0x40 }, /* DAC_CTRL0: TDM mode */
        reg_sequence { reg: 3, def: 0x00 }, /* DAC_CTRL1: reset */
        reg_sequence { reg: 4, def: 0x1A }, /* DAC_CTRL2: 48kHz de-emphasis, unmute dac */
        reg_sequence { reg: 5, def: 0x00 }, /* DAC_CHNL_MUTE: unmute DAC channels */
        reg_sequence { reg: 6, def: 0x00 }, /* DAC_L1_VOL: no attenuation */
        reg_sequence { reg: 7, def: 0x00 }, /* DAC_R1_VOL: no attenuation */
        reg_sequence { reg: 8, def: 0x00 }, /* DAC_L2_VOL: no attenuation */
        reg_sequence { reg: 9, def: 0x00 }, /* DAC_R2_VOL: no attenuation */
        reg_sequence { reg: 10, def: 0x00 }, /* DAC_L3_VOL: no attenuation */
        reg_sequence { reg: 11, def: 0x00 }, /* DAC_R3_VOL: no attenuation */
        reg_sequence { reg: 12, def: 0x00 }, /* DAC_L4_VOL: no attenuation */
        reg_sequence { reg: 13, def: 0x00 }, /* DAC_R4_VOL: no attenuation */
    ];
    static reg_adc_init: [reg_sequence; 3] = [
        reg_sequence { reg: 14, def: 0x03 }, /* ADC_CTRL0: high-pass filter enable */
        reg_sequence { reg: 15, def: 0x43 }, /* ADC_CTRL1: sata delay=1, adc aux mode */
        reg_sequence { reg: 16, def: 0x00 }, /* ADC_CTRL2: reset */
    ];

    regmap_multi_reg_write((*ad193x).regmap, reg_init.as_ptr(), ARRAY_SIZE!(reg_init));

    if ad193x_has_adc(ad193x) {
        regmap_multi_reg_write((*ad193x).regmap, reg_adc_init.as_ptr(), ARRAY_SIZE!(reg_adc_init));
    }
}

unsafe extern "C" fn ad193x_component_probe(
    component: *mut snd_soc_component,
) -> core::ffi::c_int {
    let ad193x: *mut ad193x_priv = snd_soc_component_get_drvdata(component) as *mut ad193x_priv;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let mut num: core::ffi::c_int;
    let mut ret: core::ffi::c_int;

    /* default setting for ad193x */
    ad193x_reg_default_init(ad193x);

    /* adc only */
    if ad193x_has_adc(ad193x) {
        /* add adc controls */
        num = ARRAY_SIZE!(ad193x_adc_snd_controls);
        ret = snd_soc_add_component_controls(component, ad193x_adc_snd_controls.as_ptr(), num);
        if ret != 0 {
            return ret;
        }

        /* add adc widgets */
        num = ARRAY_SIZE!(ad193x_adc_widgets);
        ret = snd_soc_dapm_new_controls(dapm, ad193x_adc_widgets.as_ptr(), num);
        if ret != 0 {
            return ret;
        }

        /* add adc routes */
        num = ARRAY_SIZE!(ad193x_adc_audio_paths);
        ret = snd_soc_dapm_add_routes(dapm, ad193x_adc_audio_paths.as_ptr(), num);
        if ret != 0 {
            return ret;
        }
    }

    0
}

static soc_component_dev_ad193x: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ad193x_component_probe),
    controls: ad193x_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(ad193x_snd_controls),
    dapm_widgets: ad193x_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(ad193x_dapm_widgets),
    dapm_routes: audio_paths.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(audio_paths),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

#[no_mangle]
pub static ad193x_regmap_config: regmap_config = regmap_config {
    max_register: AD193X_NUM_REGS - 1,
};
EXPORT_SYMBOL_GPL!(ad193x_regmap_config);

#[no_mangle]
pub unsafe extern "C" fn ad193x_probe(
    dev: *mut device,
    regmap: *mut regmap,
    type_: ad193x_type,
) -> core::ffi::c_int {
    let ad193x: *mut ad193x_priv;

    if IS_ERR(regmap as *const core::ffi::c_void) {
        return PTR_ERR(regmap as *const core::ffi::c_void);
    }

    ad193x = devm_kzalloc(dev, core::mem::size_of::<ad193x_priv>(), GFP_KERNEL) as *mut ad193x_priv;
    if ad193x.is_null() {
        return -ENOMEM;
    }

    (*ad193x).regmap = regmap;
    (*ad193x).type_ = type_;

    dev_set_drvdata(dev, ad193x as *mut core::ffi::c_void);

    if ad193x_has_adc(ad193x) {
        return devm_snd_soc_register_component(
            dev,
            &soc_component_dev_ad193x,
            core::ptr::addr_of_mut!(ad193x_dai),
            1,
        );
    }
    devm_snd_soc_register_component(
        dev,
        &soc_component_dev_ad193x,
        core::ptr::addr_of_mut!(ad193x_no_adc_dai),
        1,
    )
}
EXPORT_SYMBOL_GPL!(ad193x_probe);

MODULE_DESCRIPTION!("ASoC ad193x driver");
MODULE_AUTHOR!("Barry Song <21cnbao@gmail.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
