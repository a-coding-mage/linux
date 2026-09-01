// SPDX-License-Identifier: GPL-2.0
//
// mt8186-mt6366.c
//	--  MT8186-MT6366 ALSA SoC machine driver
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>
//
// Copyright (c) 2024 Collabora Ltd.
//                    AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//

// Dependencies from:
// linux/gpio/consumer.h, linux/input.h, linux/module.h, linux/of.h,
// sound/jack.h, sound/pcm_params.h, sound/rt5682.h, sound/soc.h,
// codecs/da7219.h, codecs/mt6358.h, codecs/rt5682.h,
// common MediaTek AFE, SOF, soundcard, and mt8186 headers.

const RT1019_CODEC_DAI: &str = "HiFi";
const RT1019_DEV0_NAME: &str = "rt1019p";

const RT5682S_CODEC_DAI: &str = "rt5682s-aif1";
const RT5682S_DEV0_NAME: &str = "rt5682s.5-001a";

const DA7219_CODEC_DAI: &str = "da7219-hifi";
const DA7219_DEV_NAME: &str = "da7219.5-001a";

const SOF_DMA_DL1: &str = "SOF_DMA_DL1";
const SOF_DMA_DL2: &str = "SOF_DMA_DL2";
const SOF_DMA_UL1: &str = "SOF_DMA_UL1";
const SOF_DMA_UL2: &str = "SOF_DMA_UL2";

const DA7219_CODEC_PRESENT: u32 = BIT!(0);

#[repr(C)]
struct mt8186_mt6366_rt1019_rt5682s_priv {
    dmic_sel: *mut gpio_desc,
    dmic_switch: core::ffi::c_int,
}

#[repr(C)]
enum mt8186_jacks {
    MT8186_JACK_HEADSET,
    MT8186_JACK_HDMI,
    MT8186_JACK_MAX,
}

kernel_static! {
/* Headset jack detection DAPM pins */
static mut mt8186_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c_str!("Headphone"),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c_str!("Headset Mic"),
        mask: SND_JACK_MICROPHONE,
    },
];

static mut mt8186_mt6366_rt1019_rt5682s_codec_conf: [snd_soc_codec_conf; 3] = [
    snd_soc_codec_conf {
        dlc: COMP_CODEC_CONF!("mt6358-sound"),
        name_prefix: c_str!("Mt6366"),
    },
    snd_soc_codec_conf {
        dlc: COMP_CODEC_CONF!("bt-sco"),
        name_prefix: c_str!("Mt8186 bt"),
    },
    snd_soc_codec_conf {
        dlc: COMP_CODEC_CONF!("hdmi-audio-codec"),
        name_prefix: c_str!("Mt8186 hdmi"),
    },
];
}

unsafe extern "C" fn dmic_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> core::ffi::c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let card = snd_soc_dapm_to_card(dapm);
    let soc_card_data = snd_soc_card_get_drvdata(card) as *mut mtk_soc_card_data;
    let priv_ = (*soc_card_data).mach_priv as *mut mt8186_mt6366_rt1019_rt5682s_priv;

    (*ucontrol).value.integer.value[0] = (*priv_).dmic_switch as _;
    0
}

unsafe extern "C" fn dmic_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> core::ffi::c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let card = snd_soc_dapm_to_card(dapm);
    let soc_card_data = snd_soc_card_get_drvdata(card) as *mut mtk_soc_card_data;
    let priv_ = (*soc_card_data).mach_priv as *mut mt8186_mt6366_rt1019_rt5682s_priv;

    (*priv_).dmic_switch = (*ucontrol).value.integer.value[0] as core::ffi::c_int;
    if !(*priv_).dmic_sel.is_null() {
        gpiod_set_value((*priv_).dmic_sel, (*priv_).dmic_switch);
        dev_dbg!((*card).dev, "dmic_set_value %d\n", (*priv_).dmic_switch);
    }
    0
}

kernel_static! {
static dmic_mux_text: [*const core::ffi::c_char; 2] = [
    c_str!("Front Mic"),
    c_str!("Rear Mic"),
];

SOC_ENUM_SINGLE_DECL!(mt8186_dmic_enum, SND_SOC_NOPM, 0, dmic_mux_text);

static mt8186_dmic_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM_EXT!("DMIC Select Mux", mt8186_dmic_enum, dmic_get, dmic_set);

static dmic_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_MIC!("DMIC", NULL),
    SND_SOC_DAPM_MUX!("Dmic Mux", SND_SOC_NOPM, 0, 0, &mt8186_dmic_mux_control),
];

static dmic_map: [snd_soc_dapm_route; 2] = [
    /* digital mics */
    snd_soc_dapm_route { sink: c_str!("Dmic Mux"), control: c_str!("Front Mic"), source: c_str!("DMIC") },
    snd_soc_dapm_route { sink: c_str!("Dmic Mux"), control: c_str!("Rear Mic"), source: c_str!("DMIC") },
];
}

unsafe extern "C" fn primary_codec_init(rtd: *mut snd_soc_pcm_runtime) -> core::ffi::c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let soc_card_data = snd_soc_card_get_drvdata(card) as *mut mtk_soc_card_data;
    let priv_ = (*soc_card_data).mach_priv as *mut mt8186_mt6366_rt1019_rt5682s_priv;
    let mut ret: core::ffi::c_int;

    ret = mt8186_mt6366_init(rtd);

    if ret != 0 {
        dev_err!((*card).dev, "mt8186_mt6366_init failed: %d\n", ret);
        return ret;
    }

    if (*priv_).dmic_sel.is_null() {
        dev_dbg!((*card).dev, "dmic_sel is null\n");
        return 0;
    }

    ret = snd_soc_dapm_new_controls(dapm, dmic_widgets.as_ptr(), ARRAY_SIZE!(dmic_widgets));
    if ret != 0 {
        dev_err!((*card).dev, "DMic widget addition failed: %d\n", ret);
        /* Don't need to add routes if widget addition failed */
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, dmic_map.as_ptr(), ARRAY_SIZE!(dmic_map));

    if ret != 0 {
        dev_err!((*card).dev, "DMic map addition failed: %d\n", ret);
    }

    ret
}

unsafe extern "C" fn mt8186_headset_codec_init(rtd: *mut snd_soc_pcm_runtime) -> core::ffi::c_int {
    let cmpnt_afe = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe = snd_soc_component_get_drvdata(cmpnt_afe) as *mut mtk_base_afe;
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card) as *mut mtk_soc_card_data;
    let jack = &mut (*(*soc_card_data).card_data).jacks[mt8186_jacks::MT8186_JACK_HEADSET as usize] as *mut snd_soc_jack;
    let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let hs_keys_rt5682 = [KEY_PLAYPAUSE, KEY_VOLUMEUP, KEY_VOLUMEDOWN, KEY_VOICECOMMAND];
    let hs_keys_da7219 = [KEY_PLAYPAUSE, KEY_VOICECOMMAND, KEY_VOLUMEUP, KEY_VOLUMEDOWN];
    let hs_keys: *const core::ffi::c_int;
    let mut ret: core::ffi::c_int;
    let mut type_: core::ffi::c_int;

    ret = mt8186_dai_i2s_set_share(afe, c_str!("I2S1"), c_str!("I2S0"));
    if ret != 0 {
        dev_err!((*rtd).dev, "Failed to set up shared clocks\n");
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c_str!("Headset Jack"),
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        jack,
        mt8186_jack_pins.as_mut_ptr(),
        ARRAY_SIZE!(mt8186_jack_pins),
    );
    if ret != 0 {
        dev_err!((*rtd).dev, "Headset Jack creation failed: %d\n", ret);
        return ret;
    }

    if ((*(*soc_card_data).card_data).flags & DA7219_CODEC_PRESENT) != 0 {
        hs_keys = hs_keys_da7219.as_ptr();
    } else {
        hs_keys = hs_keys_rt5682.as_ptr();
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, *hs_keys.add(0));
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, *hs_keys.add(1));
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, *hs_keys.add(2));
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, *hs_keys.add(3));

    type_ = SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3;
    snd_soc_component_set_jack(cmpnt_codec, jack, &mut type_ as *mut _ as *mut core::ffi::c_void)
}

unsafe extern "C" fn mt8186_da7219_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> core::ffi::c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut codec_dai: *mut snd_soc_dai;
    let rate: core::ffi::c_uint = params_rate(params);
    let mclk_fs_ratio: core::ffi::c_uint = 256;
    let mclk_fs: core::ffi::c_uint = rate.wrapping_mul(mclk_fs_ratio);
    let mut freq: core::ffi::c_uint;
    let mut ret: core::ffi::c_int;
    let mut j: core::ffi::c_int = 0;

    ret = snd_soc_dai_set_sysclk(cpu_dai, 0, mclk_fs, SND_SOC_CLOCK_OUT);
    if ret < 0 {
        dev_err!((*rtd).dev, "failed to set cpu dai sysclk: %d\n", ret);
        return ret;
    }

    for_each_rtd_codec_dais!(rtd, j, codec_dai, {
        if strcmp((*(*codec_dai).component).name, c_str!("da7219.5-001a")) != 0 {
            continue;
        }

        ret = snd_soc_dai_set_sysclk(codec_dai, DA7219_CLKSRC_MCLK, mclk_fs, SND_SOC_CLOCK_IN);
        if ret < 0 {
            dev_err!((*rtd).dev, "failed to set sysclk: %d\n", ret);
            return ret;
        }

        if (rate % 8000) == 0 {
            freq = DA7219_PLL_FREQ_OUT_98304;
        } else {
            freq = DA7219_PLL_FREQ_OUT_90316;
        }

        ret = snd_soc_dai_set_pll(codec_dai, 0, DA7219_SYSCLK_PLL_SRM, 0, freq);
        if ret != 0 {
            dev_err!((*rtd).dev, "failed to start PLL: %d\n", ret);
            return ret;
        }
    });

    0
}

unsafe extern "C" fn mt8186_da7219_i2s_hw_free(substream: *mut snd_pcm_substream) -> core::ffi::c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut codec_dai: *mut snd_soc_dai;
    let mut j: core::ffi::c_int = 0;
    let mut ret: core::ffi::c_int;

    for_each_rtd_codec_dais!(rtd, j, codec_dai, {
        if strcmp((*(*codec_dai).component).name, c_str!("da7219.5-001a")) != 0 {
            continue;
        }

        ret = snd_soc_dai_set_pll(codec_dai, 0, DA7219_SYSCLK_MCLK, 0, 0);
        if ret < 0 {
            dev_err!((*rtd).dev, "failed to stop PLL: %d\n", ret);
            return ret;
        }
    });

    0
}

kernel_static! {
static mt8186_da7219_i2s_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8186_da7219_i2s_hw_params),
    hw_free: Some(mt8186_da7219_i2s_hw_free),
};
}

unsafe extern "C" fn mt8186_rt5682s_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> core::ffi::c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let rate: core::ffi::c_uint = params_rate(params);
    let mclk_fs_ratio: core::ffi::c_uint = 128;
    let mclk_fs: core::ffi::c_uint = rate.wrapping_mul(mclk_fs_ratio);
    let bitwidth: core::ffi::c_int;
    let mut ret: core::ffi::c_int;

    bitwidth = snd_pcm_format_width(params_format(params));
    if bitwidth < 0 {
        dev_err!((*card).dev, "invalid bit width: %d\n", bitwidth);
        return bitwidth;
    }

    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x00, 0x0, 0x2, bitwidth);
    if ret != 0 {
        dev_err!((*card).dev, "failed to set tdm slot\n");
        return ret;
    }

    ret = snd_soc_dai_set_pll(
        codec_dai,
        RT5682_PLL1,
        RT5682_PLL1_S_BCLK1,
        params_rate(params).wrapping_mul(64),
        params_rate(params).wrapping_mul(512),
    );
    if ret != 0 {
        dev_err!((*card).dev, "failed to set pll\n");
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(
        codec_dai,
        RT5682_SCLK_S_PLL1,
        params_rate(params).wrapping_mul(512),
        SND_SOC_CLOCK_IN,
    );
    if ret != 0 {
        dev_err!((*card).dev, "failed to set sysclk\n");
        return ret;
    }

    snd_soc_dai_set_sysclk(cpu_dai, 0, mclk_fs, SND_SOC_CLOCK_OUT)
}

kernel_static! {
static mt8186_rt5682s_i2s_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8186_rt5682s_i2s_hw_params),
};
}

unsafe extern "C" fn mt8186_mt6366_rt1019_rt5682s_hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> core::ffi::c_int {
    let cmpnt_afe = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe = snd_soc_component_get_drvdata(cmpnt_afe) as *mut mtk_base_afe;
    let cmpnt_codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card) as *mut mtk_soc_card_data;
    let jack = &mut (*(*soc_card_data).card_data).jacks[mt8186_jacks::MT8186_JACK_HDMI as usize] as *mut snd_soc_jack;
    let mut ret: core::ffi::c_int;

    ret = mt8186_dai_i2s_set_share(afe, c_str!("I2S2"), c_str!("I2S3"));
    if ret != 0 {
        dev_err!((*rtd).dev, "Failed to set up shared clocks\n");
        return ret;
    }

    ret = snd_soc_card_jack_new((*rtd).card, c_str!("HDMI Jack"), SND_JACK_AVOUT, jack);
    if ret != 0 {
        dev_err!((*rtd).dev, "HDMI Jack creation failed: %d\n", ret);
        return ret;
    }

    snd_soc_component_set_jack(cmpnt_codec, jack, core::ptr::null_mut())
}

unsafe extern "C" fn mt8186_hw_params_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
    fmt: snd_pcm_format_t,
) -> core::ffi::c_int {
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);

    dev_dbg!((*rtd).dev, "%s(), fix format to %d\n", c_str!("mt8186_hw_params_fixup"), fmt);

    /* fix BE i2s channel to 2 channel */
    (*channels).min = 2;
    (*channels).max = 2;

    /* clean param mask first */
    snd_mask_reset_range(hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT), 0, SNDRV_PCM_FORMAT_LAST);

    params_set_format(params, fmt);

    0
}

unsafe extern "C" fn mt8186_i2s_hw_params_24le_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> core::ffi::c_int {
    mt8186_hw_params_fixup(rtd, params, SNDRV_PCM_FORMAT_S24_LE)
}

unsafe extern "C" fn mt8186_i2s_hw_params_32le_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> core::ffi::c_int {
    mt8186_hw_params_fixup(rtd, params, SNDRV_PCM_FORMAT_S32_LE)
}

/* fixup the BE DAI link to match any values from topology */
unsafe extern "C" fn mt8186_sof_dai_link_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> core::ffi::c_int {
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card) as *mut mtk_soc_card_data;
    let ret = mtk_sof_dai_link_fixup(rtd, params);

    if strcmp((*(*rtd).dai_link).name, c_str!("I2S0")) == 0
        || strcmp((*(*rtd).dai_link).name, c_str!("I2S1")) == 0
        || strcmp((*(*rtd).dai_link).name, c_str!("I2S2")) == 0
    {
        if ((*(*soc_card_data).card_data).flags & DA7219_CODEC_PRESENT) != 0 {
            mt8186_i2s_hw_params_32le_fixup(rtd, params);
        } else {
            mt8186_i2s_hw_params_24le_fixup(rtd, params);
        }
    } else if strcmp((*(*rtd).dai_link).name, c_str!("I2S3")) == 0 {
        if ((*(*soc_card_data).card_data).flags & DA7219_CODEC_PRESENT) != 0 {
            mt8186_i2s_hw_params_24le_fixup(rtd, params);
        } else {
            mt8186_i2s_hw_params_32le_fixup(rtd, params);
        }
    }

    ret
}

kernel_static! {
/* FE */
SND_SOC_DAILINK_DEFS!(playback1, DAILINK_COMP_ARRAY!(COMP_CPU!("DL1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback12, DAILINK_COMP_ARRAY!(COMP_CPU!("DL12")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback2, DAILINK_COMP_ARRAY!(COMP_CPU!("DL2")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback3, DAILINK_COMP_ARRAY!(COMP_CPU!("DL3")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback4, DAILINK_COMP_ARRAY!(COMP_CPU!("DL4")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback5, DAILINK_COMP_ARRAY!(COMP_CPU!("DL5")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback6, DAILINK_COMP_ARRAY!(COMP_CPU!("DL6")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback7, DAILINK_COMP_ARRAY!(COMP_CPU!("DL7")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(playback8, DAILINK_COMP_ARRAY!(COMP_CPU!("DL8")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture1, DAILINK_COMP_ARRAY!(COMP_CPU!("UL1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture2, DAILINK_COMP_ARRAY!(COMP_CPU!("UL2")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture3, DAILINK_COMP_ARRAY!(COMP_CPU!("UL3")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture4, DAILINK_COMP_ARRAY!(COMP_CPU!("UL4")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture5, DAILINK_COMP_ARRAY!(COMP_CPU!("UL5")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture6, DAILINK_COMP_ARRAY!(COMP_CPU!("UL6")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(capture7, DAILINK_COMP_ARRAY!(COMP_CPU!("UL7")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

/* hostless */
SND_SOC_DAILINK_DEFS!(hostless_lpbk, DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless LPBK DAI")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hostless_fm, DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless FM DAI")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hostless_src1, DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless_SRC_1_DAI")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hostless_src_bargein, DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless_SRC_Bargein_DAI")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

/* BE */
SND_SOC_DAILINK_DEFS!(adda, DAILINK_COMP_ARRAY!(COMP_CPU!("ADDA")), DAILINK_COMP_ARRAY!(COMP_CODEC!("mt6358-sound", "mt6358-snd-codec-aif1"), COMP_CODEC!("dmic-codec", "dmic-hifi")), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(i2s0, DAILINK_COMP_ARRAY!(COMP_CPU!("I2S0")), DAILINK_COMP_ARRAY!(COMP_EMPTY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(i2s1, DAILINK_COMP_ARRAY!(COMP_CPU!("I2S1")), DAILINK_COMP_ARRAY!(COMP_EMPTY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(i2s2, DAILINK_COMP_ARRAY!(COMP_CPU!("I2S2")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(i2s3, DAILINK_COMP_ARRAY!(COMP_CPU!("I2S3")), DAILINK_COMP_ARRAY!(COMP_EMPTY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hw_gain1, DAILINK_COMP_ARRAY!(COMP_CPU!("HW Gain 1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hw_gain2, DAILINK_COMP_ARRAY!(COMP_CPU!("HW Gain 2")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hw_src1, DAILINK_COMP_ARRAY!(COMP_CPU!("HW_SRC_1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hw_src2, DAILINK_COMP_ARRAY!(COMP_CPU!("HW_SRC_2")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(connsys_i2s, DAILINK_COMP_ARRAY!(COMP_CPU!("CONNSYS_I2S")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(pcm1, DAILINK_COMP_ARRAY!(COMP_CPU!("PCM 1")), DAILINK_COMP_ARRAY!(COMP_CODEC!("bt-sco", "bt-sco-pcm-wb")), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(tdm_in, DAILINK_COMP_ARRAY!(COMP_CPU!("TDM IN")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));

/* hostless */
SND_SOC_DAILINK_DEFS!(hostless_ul1, DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless_UL1 DAI")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hostless_ul2, DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless_UL2 DAI")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hostless_ul3, DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless_UL3 DAI")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hostless_ul5, DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless_UL5 DAI")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hostless_ul6, DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless_UL6 DAI")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hostless_hw_gain_aaudio, DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless HW Gain AAudio DAI")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(hostless_src_aaudio, DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless SRC AAudio DAI")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(AFE_SOF_DL1, DAILINK_COMP_ARRAY!(COMP_CPU!("SOF_DL1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(AFE_SOF_DL2, DAILINK_COMP_ARRAY!(COMP_CPU!("SOF_DL2")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(AFE_SOF_UL1, DAILINK_COMP_ARRAY!(COMP_CPU!("SOF_UL1")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
SND_SOC_DAILINK_DEFS!(AFE_SOF_UL2, DAILINK_COMP_ARRAY!(COMP_CPU!("SOF_UL2")), DAILINK_COMP_ARRAY!(COMP_DUMMY!()), DAILINK_COMP_ARRAY!(COMP_EMPTY!()));
}

kernel_data! {
static g_sof_conn_streams: [sof_conn_stream; 4] = [
    { "I2S1", "AFE_SOF_DL1", SOF_DMA_DL1, SNDRV_PCM_STREAM_PLAYBACK },
    { "I2S3", "AFE_SOF_DL2", SOF_DMA_DL2, SNDRV_PCM_STREAM_PLAYBACK },
    { "Primary Codec", "AFE_SOF_UL1", SOF_DMA_UL1, SNDRV_PCM_STREAM_CAPTURE },
    { "I2S0", "AFE_SOF_UL2", SOF_DMA_UL2, SNDRV_PCM_STREAM_CAPTURE },
];

static mut mt8186_mt6366_rt1019_rt5682s_dai_links: [snd_soc_dai_link; 43] = [
    /* Front End DAI links */
    { .name = "Playback_1", .stream_name = "Playback_1", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .playback_only = 1, .dpcm_merged_format = 1, .dpcm_merged_chan = 1, .dpcm_merged_rate = 1, .ops = &mtk_soundcard_common_playback_ops, SND_SOC_DAILINK_REG(playback1) },
    { .name = "Playback_12", .stream_name = "Playback_12", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .playback_only = 1, SND_SOC_DAILINK_REG(playback12) },
    { .name = "Playback_2", .stream_name = "Playback_2", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .playback_only = 1, .dpcm_merged_format = 1, .dpcm_merged_chan = 1, .dpcm_merged_rate = 1, SND_SOC_DAILINK_REG(playback2) },
    { .name = "Playback_3", .stream_name = "Playback_3", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .playback_only = 1, .dpcm_merged_format = 1, .dpcm_merged_chan = 1, .dpcm_merged_rate = 1, .ops = &mtk_soundcard_common_playback_ops, SND_SOC_DAILINK_REG(playback3) },
    { .name = "Playback_4", .stream_name = "Playback_4", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .playback_only = 1, SND_SOC_DAILINK_REG(playback4) },
    { .name = "Playback_5", .stream_name = "Playback_5", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .playback_only = 1, SND_SOC_DAILINK_REG(playback5) },
    { .name = "Playback_6", .stream_name = "Playback_6", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .playback_only = 1, SND_SOC_DAILINK_REG(playback6) },
    { .name = "Playback_7", .stream_name = "Playback_7", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .playback_only = 1, SND_SOC_DAILINK_REG(playback7) },
    { .name = "Playback_8", .stream_name = "Playback_8", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .playback_only = 1, SND_SOC_DAILINK_REG(playback8) },
    { .name = "Capture_1", .stream_name = "Capture_1", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .capture_only = 1, SND_SOC_DAILINK_REG(capture1) },
    { .name = "Capture_2", .stream_name = "Capture_2", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .capture_only = 1, .dpcm_merged_format = 1, .dpcm_merged_chan = 1, .dpcm_merged_rate = 1, .ops = &mtk_soundcard_common_capture_ops, SND_SOC_DAILINK_REG(capture2) },
    { .name = "Capture_3", .stream_name = "Capture_3", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .capture_only = 1, SND_SOC_DAILINK_REG(capture3) },
    { .name = "Capture_4", .stream_name = "Capture_4", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .capture_only = 1, .dpcm_merged_format = 1, .dpcm_merged_chan = 1, .dpcm_merged_rate = 1, .ops = &mtk_soundcard_common_capture_ops, SND_SOC_DAILINK_REG(capture4) },
    { .name = "Capture_5", .stream_name = "Capture_5", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .capture_only = 1, SND_SOC_DAILINK_REG(capture5) },
    { .name = "Capture_6", .stream_name = "Capture_6", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .capture_only = 1, .dpcm_merged_format = 1, .dpcm_merged_chan = 1, .dpcm_merged_rate = 1, SND_SOC_DAILINK_REG(capture6) },
    { .name = "Capture_7", .stream_name = "Capture_7", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .capture_only = 1, SND_SOC_DAILINK_REG(capture7) },
    { .name = "Hostless_LPBK", .stream_name = "Hostless_LPBK", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hostless_lpbk) },
    { .name = "Hostless_FM", .stream_name = "Hostless_FM", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hostless_fm) },
    { .name = "Hostless_SRC_1", .stream_name = "Hostless_SRC_1", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hostless_src1) },
    { .name = "Hostless_SRC_Bargein", .stream_name = "Hostless_SRC_Bargein", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hostless_src_bargein) },
    { .name = "Hostless_HW_Gain_AAudio", .stream_name = "Hostless_HW_Gain_AAudio", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .capture_only = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hostless_hw_gain_aaudio) },
    { .name = "Hostless_SRC_AAudio", .stream_name = "Hostless_SRC_AAudio", .trigger = {SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE}, .dynamic = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hostless_src_aaudio) },
    /* Back End DAI links */
    { .name = "Primary Codec", .no_pcm = 1, .ignore_suspend = 1, .init = primary_codec_init, SND_SOC_DAILINK_REG(adda) },
    { .name = "I2S3", .no_pcm = 1, .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_IB_IF | SND_SOC_DAIFMT_CBP_CFP, .playback_only = 1, .ignore_suspend = 1, .init = mt8186_mt6366_rt1019_rt5682s_hdmi_init, SND_SOC_DAILINK_REG(i2s3) },
    { .name = "I2S0", .no_pcm = 1, .capture_only = 1, .ignore_suspend = 1, .ops = &mt8186_rt5682s_i2s_ops, SND_SOC_DAILINK_REG(i2s0) },
    { .name = "I2S1", .no_pcm = 1, .playback_only = 1, .ignore_suspend = 1, .init = mt8186_headset_codec_init, SND_SOC_DAILINK_REG(i2s1) },
    { .name = "I2S2", .no_pcm = 1, .capture_only = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(i2s2) },
    { .name = "HW Gain 1", .no_pcm = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hw_gain1) },
    { .name = "HW Gain 2", .no_pcm = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hw_gain2) },
    { .name = "HW_SRC_1", .no_pcm = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hw_src1) },
    { .name = "HW_SRC_2", .no_pcm = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hw_src2) },
    { .name = "CONNSYS_I2S", .no_pcm = 1, .capture_only = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(connsys_i2s) },
    { .name = "PCM 1", .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_IF, .no_pcm = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(pcm1) },
    { .name = "TDM IN", .no_pcm = 1, .capture_only = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(tdm_in) },
    /* dummy BE for ul memif to record from dl memif */
    { .name = "Hostless_UL1", .no_pcm = 1, .capture_only = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hostless_ul1) },
    { .name = "Hostless_UL2", .no_pcm = 1, .capture_only = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hostless_ul2) },
    { .name = "Hostless_UL3", .no_pcm = 1, .capture_only = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hostless_ul3) },
    { .name = "Hostless_UL5", .no_pcm = 1, .capture_only = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hostless_ul5) },
    { .name = "Hostless_UL6", .no_pcm = 1, .capture_only = 1, .ignore_suspend = 1, SND_SOC_DAILINK_REG(hostless_ul6) },
    /* SOF BE */
    { .name = "AFE_SOF_DL1", .no_pcm = 1, .playback_only = 1, SND_SOC_DAILINK_REG(AFE_SOF_DL1) },
    { .name = "AFE_SOF_DL2", .no_pcm = 1, .playback_only = 1, SND_SOC_DAILINK_REG(AFE_SOF_DL2) },
    { .name = "AFE_SOF_UL1", .no_pcm = 1, .capture_only = 1, SND_SOC_DAILINK_REG(AFE_SOF_UL1) },
    { .name = "AFE_SOF_UL2", .no_pcm = 1, .capture_only = 1, SND_SOC_DAILINK_REG(AFE_SOF_UL2) },
];

static mt8186_mt6366_da7219_max98357_widgets: [snd_soc_dapm_widget; 9] = [
    SND_SOC_DAPM_SPK("Speakers", NULL),
    SND_SOC_DAPM_HP("Headphones", NULL),
    SND_SOC_DAPM_MIC("Headset Mic", NULL),
    SND_SOC_DAPM_LINE("Line Out", NULL),
    SND_SOC_DAPM_LINE("HDMI1", NULL),
    SND_SOC_DAPM_MIXER(SOF_DMA_DL1, SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER(SOF_DMA_DL2, SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER(SOF_DMA_UL1, SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER(SOF_DMA_UL2, SND_SOC_NOPM, 0, 0, NULL, 0),
];

static mt8186_mt6366_rt1019_rt5682s_widgets: [snd_soc_dapm_widget; 8] = [
    SND_SOC_DAPM_SPK("Speakers", NULL),
    SND_SOC_DAPM_HP("Headphone", NULL),
    SND_SOC_DAPM_MIC("Headset Mic", NULL),
    SND_SOC_DAPM_OUTPUT("HDMI1"),
    SND_SOC_DAPM_MIXER(SOF_DMA_DL1, SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER(SOF_DMA_DL2, SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER(SOF_DMA_UL1, SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER(SOF_DMA_UL2, SND_SOC_NOPM, 0, 0, NULL, 0),
];

static mt8186_mt6366_rt1019_rt5682s_routes: [snd_soc_dapm_route; 11] = [
    /* SPK */ { "Speakers", NULL, "Speaker" },
    /* Headset */ { "Headphone", NULL, "HPOL" }, { "Headphone", NULL, "HPOR" }, { "IN1P", NULL, "Headset Mic" },
    /* HDMI */ { "HDMI1", NULL, "TX" },
    /* SOF Uplink */ { SOF_DMA_UL1, NULL, "UL1_CH1" }, { SOF_DMA_UL1, NULL, "UL1_CH2" }, { SOF_DMA_UL2, NULL, "UL2_CH1" }, { SOF_DMA_UL2, NULL, "UL2_CH2" },
    /* SOF Downlink */ { "DSP_DL1_VIRT", NULL, SOF_DMA_DL1 }, { "DSP_DL2_VIRT", NULL, SOF_DMA_DL2 },
];

static mt8186_mt6366_rt5650_routes: [snd_soc_dapm_route; 13] = [
    /* SPK */ { "Speakers", NULL, "SPOL" }, { "Speakers", NULL, "SPOR" },
    /* Headset */ { "Headphone", NULL, "HPOL" }, { "Headphone", NULL, "HPOR" }, { "IN1P", NULL, "Headset Mic" }, { "IN1N", NULL, "Headset Mic" },
    /* HDMI */ { "HDMI1", NULL, "TX" },
    /* SOF Uplink */ { SOF_DMA_UL1, NULL, "UL1_CH1" }, { SOF_DMA_UL1, NULL, "UL1_CH2" }, { SOF_DMA_UL2, NULL, "UL2_CH1" }, { SOF_DMA_UL2, NULL, "UL2_CH2" },
    /* SOF Downlink */ { "DSP_DL1_VIRT", NULL, SOF_DMA_DL1 }, { "DSP_DL2_VIRT", NULL, SOF_DMA_DL2 },
];

static mt8186_mt6366_da7219_max98357_controls: [snd_kcontrol_new; 5] = [
    SOC_DAPM_PIN_SWITCH("Speakers"),
    SOC_DAPM_PIN_SWITCH("Headphones"),
    SOC_DAPM_PIN_SWITCH("Headset Mic"),
    SOC_DAPM_PIN_SWITCH("Line Out"),
    SOC_DAPM_PIN_SWITCH("HDMI1"),
];

static mt8186_mt6366_rt1019_rt5682s_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_PIN_SWITCH("Speakers"),
    SOC_DAPM_PIN_SWITCH("Headphone"),
    SOC_DAPM_PIN_SWITCH("Headset Mic"),
    SOC_DAPM_PIN_SWITCH("HDMI1"),
];

static mut mt8186_mt6366_da7219_max98357_soc_card: snd_soc_card = {
    .name = "mt8186_da7219_max98357",
    .owner = THIS_MODULE,
    .dai_link = mt8186_mt6366_rt1019_rt5682s_dai_links,
    .num_links = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_dai_links),
    .controls = mt8186_mt6366_da7219_max98357_controls,
    .num_controls = ARRAY_SIZE(mt8186_mt6366_da7219_max98357_controls),
    .dapm_widgets = mt8186_mt6366_da7219_max98357_widgets,
    .num_dapm_widgets = ARRAY_SIZE(mt8186_mt6366_da7219_max98357_widgets),
    .dapm_routes = mt8186_mt6366_rt1019_rt5682s_routes,
    .num_dapm_routes = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_routes),
    .codec_conf = mt8186_mt6366_rt1019_rt5682s_codec_conf,
    .num_configs = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_codec_conf),
};

static mut mt8186_mt6366_rt1019_rt5682s_soc_card: snd_soc_card = {
    .name = "mt8186_rt1019_rt5682s",
    .owner = THIS_MODULE,
    .dai_link = mt8186_mt6366_rt1019_rt5682s_dai_links,
    .num_links = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_dai_links),
    .controls = mt8186_mt6366_rt1019_rt5682s_controls,
    .num_controls = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_controls),
    .dapm_widgets = mt8186_mt6366_rt1019_rt5682s_widgets,
    .num_dapm_widgets = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_widgets),
    .dapm_routes = mt8186_mt6366_rt1019_rt5682s_routes,
    .num_dapm_routes = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_routes),
    .codec_conf = mt8186_mt6366_rt1019_rt5682s_codec_conf,
    .num_configs = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_codec_conf),
};

static mut mt8186_mt6366_rt5682s_max98360_soc_card: snd_soc_card = {
    .name = "mt8186_rt5682s_max98360",
    .owner = THIS_MODULE,
    .dai_link = mt8186_mt6366_rt1019_rt5682s_dai_links,
    .num_links = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_dai_links),
    .controls = mt8186_mt6366_rt1019_rt5682s_controls,
    .num_controls = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_controls),
    .dapm_widgets = mt8186_mt6366_rt1019_rt5682s_widgets,
    .num_dapm_widgets = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_widgets),
    .dapm_routes = mt8186_mt6366_rt1019_rt5682s_routes,
    .num_dapm_routes = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_routes),
    .codec_conf = mt8186_mt6366_rt1019_rt5682s_codec_conf,
    .num_configs = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_codec_conf),
};

static mut mt8186_mt6366_rt5650_soc_card: snd_soc_card = {
    .name = "mt8186_rt5650",
    .owner = THIS_MODULE,
    .dai_link = mt8186_mt6366_rt1019_rt5682s_dai_links,
    .num_links = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_dai_links),
    .controls = mt8186_mt6366_rt1019_rt5682s_controls,
    .num_controls = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_controls),
    .dapm_widgets = mt8186_mt6366_rt1019_rt5682s_widgets,
    .num_dapm_widgets = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_widgets),
    .dapm_routes = mt8186_mt6366_rt5650_routes,
    .num_dapm_routes = ARRAY_SIZE(mt8186_mt6366_rt5650_routes),
    .codec_conf = mt8186_mt6366_rt1019_rt5682s_codec_conf,
    .num_configs = ARRAY_SIZE(mt8186_mt6366_rt1019_rt5682s_codec_conf),
};
}

unsafe extern "C" fn mt8186_mt6366_legacy_probe(
    soc_card_data: *mut mtk_soc_card_data,
) -> core::ffi::c_int {
    let card_data = (*soc_card_data).card_data;
    let card = (*card_data).card;
    let dev = (*card).dev;
    let mut dai_link: *mut snd_soc_dai_link;
    let headset_codec: *mut device_node;
    let playback_codec: *mut device_node;
    let mut ret: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;

    playback_codec = of_get_child_by_name((*dev).of_node, c_str!("playback-codecs"));
    if playback_codec.is_null() {
        return dev_err_probe(dev, -EINVAL, "Property 'playback-codecs' missing or invalid\n");
    }

    headset_codec = of_get_child_by_name((*dev).of_node, c_str!("headset-codec"));
    if headset_codec.is_null() {
        of_node_put(playback_codec);
        return dev_err_probe(dev, -EINVAL, "Property 'headset-codec' missing or invalid\n");
    }

    for_each_card_prelinks!(card, i, dai_link, {
        ret = mt8186_mt6366_card_set_be_link(dev, dai_link, playback_codec, c_str!("I2S3"));
        if ret != 0 {
            dev_err_probe(dev, ret, "%s set playback_codec fail\n", (*dai_link).name);
            break;
        }

        ret = mt8186_mt6366_card_set_be_link(dev, dai_link, headset_codec, c_str!("I2S0"));
        if ret != 0 {
            dev_err_probe(dev, ret, "%s set headset_codec fail\n", (*dai_link).name);
            break;
        }

        ret = mt8186_mt6366_card_set_be_link(dev, dai_link, headset_codec, c_str!("I2S1"));
        if ret != 0 {
            dev_err_probe(dev, ret, "%s set headset_codec fail\n", (*dai_link).name);
            break;
        }
    });
    of_node_put(headset_codec);
    of_node_put(playback_codec);

    ret
}

unsafe extern "C" fn mt8186_mt6366_soc_card_probe(
    soc_card_data: *mut mtk_soc_card_data,
    legacy: bool,
) -> core::ffi::c_int {
    let card_data = (*soc_card_data).card_data;
    let card = (*card_data).card;
    let mut dai_link: *mut snd_soc_dai_link;
    let mach_priv: *mut mt8186_mt6366_rt1019_rt5682s_priv;
    let dev = (*card).dev;
    let mut i: core::ffi::c_int = 0;
    let mut ret: core::ffi::c_int;

    mach_priv = devm_kzalloc(dev, core::mem::size_of::<mt8186_mt6366_rt1019_rt5682s_priv>(), GFP_KERNEL)
        as *mut mt8186_mt6366_rt1019_rt5682s_priv;
    if mach_priv.is_null() {
        return -ENOMEM;
    }

    (*soc_card_data).mach_priv = mach_priv as *mut core::ffi::c_void;

    (*mach_priv).dmic_sel = devm_gpiod_get_optional(dev, c_str!("dmic"), GPIOD_OUT_LOW);
    if IS_ERR((*mach_priv).dmic_sel) {
        return dev_err_probe(dev, PTR_ERR((*mach_priv).dmic_sel), "DMIC gpio failed\n");
    }

    for_each_card_prelinks!(card, i, dai_link, {
        if strcmp((*dai_link).name, c_str!("I2S0")) == 0
            || strcmp((*dai_link).name, c_str!("I2S1")) == 0
            || strcmp((*dai_link).name, c_str!("I2S2")) == 0
        {
            if ((*card_data).flags & DA7219_CODEC_PRESENT) != 0 {
                (*dai_link).be_hw_params_fixup = Some(mt8186_i2s_hw_params_32le_fixup);
                (*dai_link).ops = &mt8186_da7219_i2s_ops;
            } else {
                (*dai_link).be_hw_params_fixup = Some(mt8186_i2s_hw_params_24le_fixup);
                (*dai_link).ops = &mt8186_rt5682s_i2s_ops;
            }
        } else if strcmp((*dai_link).name, c_str!("I2S3")) == 0 {
            if ((*card_data).flags & DA7219_CODEC_PRESENT) != 0 {
                (*dai_link).be_hw_params_fixup = Some(mt8186_i2s_hw_params_24le_fixup);
            } else {
                (*dai_link).be_hw_params_fixup = Some(mt8186_i2s_hw_params_32le_fixup);
            }
        }
    });

    if legacy {
        ret = mt8186_mt6366_legacy_probe(soc_card_data);
        if ret != 0 {
            return ret;
        }
    }

    ret = mt8186_afe_gpio_init(dev);
    if ret != 0 {
        return dev_err_probe(dev, ret, "init AFE gpio error\n");
    }

    0
}

kernel_data! {
static mt8186_pcm_playback_channels: [core::ffi::c_uint; 1] = [2];
static mt8186_pcm_capture_channels: [core::ffi::c_uint; 2] = [1, 2];
static mt8186_pcm_rates: [core::ffi::c_uint; 1] = [48000];

static mt8186_rate_constraint: snd_pcm_hw_constraint_list = {
    .list = mt8186_pcm_rates,
    .count = ARRAY_SIZE(mt8186_pcm_rates)
};

static mt8186_pcm_constraints: [mtk_pcm_constraints_data; MTK_CONSTRAINT_CAPTURE + 1] = [
    [MTK_CONSTRAINT_PLAYBACK] = {
        .channels = &(const snd_pcm_hw_constraint_list) {
            .list = mt8186_pcm_playback_channels,
            .count = ARRAY_SIZE(mt8186_pcm_playback_channels)
        },
        .rates = &mt8186_rate_constraint,
    },
    [MTK_CONSTRAINT_CAPTURE] = {
        .channels = &(const snd_pcm_hw_constraint_list) {
            .list = mt8186_pcm_capture_channels,
            .count = ARRAY_SIZE(mt8186_pcm_capture_channels)
        },
        .rates = &mt8186_rate_constraint,
    }
];

static mt8186_sof_priv: mtk_sof_priv = {
    .conn_streams = g_sof_conn_streams,
    .num_streams = ARRAY_SIZE(g_sof_conn_streams),
    .sof_dai_link_fixup = mt8186_sof_dai_link_fixup
};

static mt8186_mt6366_da7219_max98357_pdata: mtk_soundcard_pdata = {
    .card_data = &(mtk_platform_card_data) {
        .card = &mt8186_mt6366_da7219_max98357_soc_card,
        .num_jacks = MT8186_JACK_MAX,
        .pcm_constraints = mt8186_pcm_constraints,
        .num_pcm_constraints = ARRAY_SIZE(mt8186_pcm_constraints),
        .flags = DA7219_CODEC_PRESENT,
    },
    .sof_priv = &mt8186_sof_priv,
    .soc_probe = mt8186_mt6366_soc_card_probe
};

static mt8186_mt6366_rt1019_rt5682s_pdata: mtk_soundcard_pdata = {
    .card_data = &(mtk_platform_card_data) {
        .card = &mt8186_mt6366_rt1019_rt5682s_soc_card,
        .num_jacks = MT8186_JACK_MAX,
        .pcm_constraints = mt8186_pcm_constraints,
        .num_pcm_constraints = ARRAY_SIZE(mt8186_pcm_constraints),
    },
    .sof_priv = &mt8186_sof_priv,
    .soc_probe = mt8186_mt6366_soc_card_probe
};

static mt8186_mt6366_rt5682s_max98360_pdata: mtk_soundcard_pdata = {
    .card_data = &(mtk_platform_card_data) {
        .card = &mt8186_mt6366_rt5682s_max98360_soc_card,
        .num_jacks = MT8186_JACK_MAX,
        .pcm_constraints = mt8186_pcm_constraints,
        .num_pcm_constraints = ARRAY_SIZE(mt8186_pcm_constraints),
    },
    .sof_priv = &mt8186_sof_priv,
    .soc_probe = mt8186_mt6366_soc_card_probe
};

static mt8186_mt6366_rt5650_pdata: mtk_soundcard_pdata = {
    .card_data = &(mtk_platform_card_data) {
        .card = &mt8186_mt6366_rt5650_soc_card,
        .num_jacks = MT8186_JACK_MAX,
        .pcm_constraints = mt8186_pcm_constraints,
        .num_pcm_constraints = ARRAY_SIZE(mt8186_pcm_constraints),
    },
    .sof_priv = &mt8186_sof_priv,
    .soc_probe = mt8186_mt6366_soc_card_probe
};

/* #if IS_ENABLED(CONFIG_OF) */
static mt8186_mt6366_dt_match: [of_device_id; 5] = [
    { .compatible = "mediatek,mt8186-mt6366-rt1019-rt5682s-sound", .data = &mt8186_mt6366_rt1019_rt5682s_pdata },
    { .compatible = "mediatek,mt8186-mt6366-rt5682s-max98360-sound", .data = &mt8186_mt6366_rt5682s_max98360_pdata },
    { .compatible = "mediatek,mt8186-mt6366-rt5650-sound", .data = &mt8186_mt6366_rt5650_pdata },
    { .compatible = "mediatek,mt8186-mt6366-da7219-max98357-sound", .data = &mt8186_mt6366_da7219_max98357_pdata },
    { /* sentinel */ }
];
MODULE_DEVICE_TABLE(of, mt8186_mt6366_dt_match);
/* #endif */

static mut mt8186_mt6366_driver: platform_driver = {
    .driver = {
        .name = "mt8186_mt6366",
        /* #if IS_ENABLED(CONFIG_OF) */
        .of_match_table = mt8186_mt6366_dt_match,
        /* #endif */
        .pm = &snd_soc_pm_ops,
    },
    .probe = mtk_soundcard_common_probe,
};

module_platform_driver(mt8186_mt6366_driver);

/* Module information */
MODULE_DESCRIPTION("MT8186-MT6366 ALSA SoC machine driver");
MODULE_AUTHOR("Jiaxin Yu <jiaxin.yu@mediatek.com>");
MODULE_LICENSE("GPL v2");
MODULE_ALIAS("mt8186_mt6366 soc card");
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
