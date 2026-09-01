// SPDX-License-Identifier: GPL-2.0+
//
// Bells audio support
//
// Copyright 2012 Wolfson Microelectronics

// C dependencies translated as external Rust dependencies:
// sound/soc.h, sound/soc-dapm.h, sound/jack.h, linux/module.h,
// ../codecs/wm5102.h, ../codecs/wm9081.h.

/* BCLK2 is fixed at this currently */
const BCLK2_RATE: c_int = 64 * 8000;

/*
 * Expect a 24.576MHz crystal if one is fitted (the driver will function
 * if this is not fitted).
 */
const MCLK_RATE: c_int = 24576000;

const SYS_AUDIO_RATE: c_int = 44100;
const SYS_MCLK_RATE: c_int = SYS_AUDIO_RATE * 512;

const DAI_AP_DSP: usize = 0;
const DAI_DSP_CODEC: usize = 1;
const DAI_CODEC_CP: usize = 2;
const DAI_CODEC_SUB: usize = 3;

#[repr(C)]
struct bells_drvdata {
    sysclk_rate: c_int,
    asyncclk_rate: c_int,
}

static mut wm2200_drvdata: bells_drvdata = bells_drvdata {
    sysclk_rate: 22579200,
    asyncclk_rate: 0,
};

static mut wm5102_drvdata: bells_drvdata = bells_drvdata {
    sysclk_rate: 45158400,
    asyncclk_rate: 49152000,
};

static mut wm5110_drvdata: bells_drvdata = bells_drvdata {
    sysclk_rate: 135475200,
    asyncclk_rate: 147456000,
};

unsafe extern "C" fn bells_set_bias_level(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime;
    let codec_dai: *mut snd_soc_dai;
    let component: *mut snd_soc_component;
    let bells: *mut bells_drvdata = (*card).drvdata as *mut bells_drvdata;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, &mut *(*card).dai_link.add(DAI_DSP_CODEC));
    codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    component = (*codec_dai).component;

    if snd_soc_dapm_to_dev(dapm) != (*codec_dai).dev {
        return 0;
    }

    match level {
        SND_SOC_BIAS_PREPARE => {
            if snd_soc_dapm_get_bias_level(dapm) != SND_SOC_BIAS_STANDBY {
                return 0;
            }

            ret = snd_soc_component_set_pll(
                component,
                WM5102_FLL1,
                ARIZONA_FLL_SRC_MCLK1,
                MCLK_RATE,
                (*bells).sysclk_rate,
            );
            if ret < 0 {
                pr_err(c"Failed to start FLL: %d\n".as_ptr(), ret);
            }

            if (*bells).asyncclk_rate != 0 {
                ret = snd_soc_component_set_pll(
                    component,
                    WM5102_FLL2,
                    ARIZONA_FLL_SRC_AIF2BCLK,
                    BCLK2_RATE,
                    (*bells).asyncclk_rate,
                );
                if ret < 0 {
                    pr_err(c"Failed to start FLL: %d\n".as_ptr(), ret);
                }
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn bells_set_bias_level_post(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime;
    let codec_dai: *mut snd_soc_dai;
    let component: *mut snd_soc_component;
    let bells: *mut bells_drvdata = (*card).drvdata as *mut bells_drvdata;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, &mut *(*card).dai_link.add(DAI_DSP_CODEC));
    codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    component = (*codec_dai).component;

    if snd_soc_dapm_to_dev(dapm) != (*codec_dai).dev {
        return 0;
    }

    match level {
        SND_SOC_BIAS_STANDBY => {
            ret = snd_soc_component_set_pll(component, WM5102_FLL1, 0, 0, 0);
            if ret < 0 {
                pr_err(c"Failed to stop FLL: %d\n".as_ptr(), ret);
                return ret;
            }

            if (*bells).asyncclk_rate != 0 {
                ret = snd_soc_component_set_pll(component, WM5102_FLL2, 0, 0, 0);
                if ret < 0 {
                    pr_err(c"Failed to stop FLL: %d\n".as_ptr(), ret);
                    return ret;
                }
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn bells_late_probe(card: *mut snd_soc_card) -> c_int {
    let bells: *mut bells_drvdata = (*card).drvdata as *mut bells_drvdata;
    let mut rtd: *mut snd_soc_pcm_runtime;
    let wm0010: *mut snd_soc_component;
    let component: *mut snd_soc_component;
    let aif1_dai: *mut snd_soc_dai;
    let aif2_dai: *mut snd_soc_dai;
    let aif3_dai: *mut snd_soc_dai;
    let wm9081_dai: *mut snd_soc_dai;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, &mut *(*card).dai_link.add(DAI_AP_DSP));
    wm0010 = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    rtd = snd_soc_get_pcm_runtime(card, &mut *(*card).dai_link.add(DAI_DSP_CODEC));
    component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    aif1_dai = snd_soc_rtd_to_codec(rtd, 0);

    ret = snd_soc_component_set_sysclk(
        component,
        ARIZONA_CLK_SYSCLK,
        ARIZONA_CLK_SRC_FLL1,
        (*bells).sysclk_rate,
        SND_SOC_CLOCK_IN,
    );
    if ret != 0 {
        dev_err((*component).dev, c"Failed to set SYSCLK: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_component_set_sysclk(wm0010, 0, 0, SYS_MCLK_RATE, 0);
    if ret != 0 {
        dev_err((*wm0010).dev, c"Failed to set WM0010 clock: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(aif1_dai, ARIZONA_CLK_SYSCLK, 0, 0);
    if ret != 0 {
        dev_err((*aif1_dai).dev, c"Failed to set AIF1 clock: %d\n".as_ptr(), ret);
    }

    ret = snd_soc_component_set_sysclk(
        component,
        ARIZONA_CLK_OPCLK,
        0,
        SYS_MCLK_RATE,
        SND_SOC_CLOCK_OUT,
    );
    if ret != 0 {
        dev_err((*component).dev, c"Failed to set OPCLK: %d\n".as_ptr(), ret);
    }

    if (*card).num_rtd == DAI_CODEC_CP as c_int {
        return 0;
    }

    ret = snd_soc_component_set_sysclk(
        component,
        ARIZONA_CLK_ASYNCCLK,
        ARIZONA_CLK_SRC_FLL2,
        (*bells).asyncclk_rate,
        SND_SOC_CLOCK_IN,
    );
    if ret != 0 {
        dev_err((*component).dev, c"Failed to set ASYNCCLK: %d\n".as_ptr(), ret);
        return ret;
    }

    rtd = snd_soc_get_pcm_runtime(card, &mut *(*card).dai_link.add(DAI_CODEC_CP));
    aif2_dai = snd_soc_rtd_to_cpu(rtd, 0);

    ret = snd_soc_dai_set_sysclk(aif2_dai, ARIZONA_CLK_ASYNCCLK, 0, 0);
    if ret != 0 {
        dev_err((*aif2_dai).dev, c"Failed to set AIF2 clock: %d\n".as_ptr(), ret);
        return ret;
    }

    if (*card).num_rtd == DAI_CODEC_SUB as c_int {
        return 0;
    }

    rtd = snd_soc_get_pcm_runtime(card, &mut *(*card).dai_link.add(DAI_CODEC_SUB));
    aif3_dai = snd_soc_rtd_to_cpu(rtd, 0);
    wm9081_dai = snd_soc_rtd_to_codec(rtd, 0);

    ret = snd_soc_dai_set_sysclk(aif3_dai, ARIZONA_CLK_SYSCLK, 0, 0);
    if ret != 0 {
        dev_err((*aif1_dai).dev, c"Failed to set AIF1 clock: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_component_set_sysclk(
        (*wm9081_dai).component,
        WM9081_SYSCLK_MCLK,
        0,
        SYS_MCLK_RATE,
        0,
    );
    if ret != 0 {
        dev_err((*wm9081_dai).dev, c"Failed to set MCLK: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

static baseband_params: snd_soc_pcm_stream = snd_soc_pcm_stream {
    formats: SNDRV_PCM_FMTBIT_S32_LE,
    rate_min: 8000,
    rate_max: 8000,
    channels_min: 2,
    channels_max: 2,
};

static sub_params: snd_soc_pcm_stream = snd_soc_pcm_stream {
    formats: SNDRV_PCM_FMTBIT_S32_LE,
    rate_min: SYS_AUDIO_RATE,
    rate_max: SYS_AUDIO_RATE,
    channels_min: 2,
    channels_max: 2,
};

SND_SOC_DAILINK_DEFS!(
    wm2200_cpu_dsp,
    DAILINK_COMP_ARRAY!(COMP_CPU!(c"samsung-i2s.0")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(c"spi0.0", c"wm0010-sdi1")),
    DAILINK_COMP_ARRAY!(COMP_PLATFORM!(c"samsung-i2s.0"))
);

SND_SOC_DAILINK_DEFS!(
    wm2200_dsp_codec,
    DAILINK_COMP_ARRAY!(COMP_CPU!(c"wm0010-sdi2")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(c"wm2200.1-003a", c"wm2200"))
);

static mut bells_dai_wm2200: [snd_soc_dai_link; 2] = [
    snd_soc_dai_link {
        name: c"CPU-DSP".as_ptr(),
        stream_name: c"CPU-DSP".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        SND_SOC_DAILINK_REG!(wm2200_cpu_dsp)
    },
    snd_soc_dai_link {
        name: c"DSP-CODEC".as_ptr(),
        stream_name: c"DSP-CODEC".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        c2c_params: &sub_params,
        num_c2c_params: 1,
        ignore_suspend: 1,
        SND_SOC_DAILINK_REG!(wm2200_dsp_codec)
    },
];

SND_SOC_DAILINK_DEFS!(
    wm5102_cpu_dsp,
    DAILINK_COMP_ARRAY!(COMP_CPU!(c"samsung-i2s.0")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(c"spi0.0", c"wm0010-sdi1")),
    DAILINK_COMP_ARRAY!(COMP_PLATFORM!(c"samsung-i2s.0"))
);

SND_SOC_DAILINK_DEFS!(
    wm5102_dsp_codec,
    DAILINK_COMP_ARRAY!(COMP_CPU!(c"wm0010-sdi2")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(c"wm5102-codec", c"wm5102-aif1"))
);

SND_SOC_DAILINK_DEFS!(
    wm5102_baseband,
    DAILINK_COMP_ARRAY!(COMP_CPU!(c"wm5102-aif2")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(c"wm1250-ev1.1-0027", c"wm1250-ev1"))
);

SND_SOC_DAILINK_DEFS!(
    wm5102_sub,
    DAILINK_COMP_ARRAY!(COMP_CPU!(c"wm5102-aif3")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(c"wm9081.1-006c", c"wm9081-hifi"))
);

static mut bells_dai_wm5102: [snd_soc_dai_link; 4] = [
    snd_soc_dai_link {
        name: c"CPU-DSP".as_ptr(),
        stream_name: c"CPU-DSP".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        SND_SOC_DAILINK_REG!(wm5102_cpu_dsp)
    },
    snd_soc_dai_link {
        name: c"DSP-CODEC".as_ptr(),
        stream_name: c"DSP-CODEC".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        c2c_params: &sub_params,
        num_c2c_params: 1,
        ignore_suspend: 1,
        SND_SOC_DAILINK_REG!(wm5102_dsp_codec)
    },
    snd_soc_dai_link {
        name: c"Baseband".as_ptr(),
        stream_name: c"Baseband".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        ignore_suspend: 1,
        c2c_params: &baseband_params,
        num_c2c_params: 1,
        SND_SOC_DAILINK_REG!(wm5102_baseband)
    },
    snd_soc_dai_link {
        name: c"Sub".as_ptr(),
        stream_name: c"Sub".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        ignore_suspend: 1,
        c2c_params: &sub_params,
        num_c2c_params: 1,
        SND_SOC_DAILINK_REG!(wm5102_sub)
    },
];

SND_SOC_DAILINK_DEFS!(
    wm5110_cpu_dsp,
    DAILINK_COMP_ARRAY!(COMP_CPU!(c"samsung-i2s.0")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(c"spi0.0", c"wm0010-sdi1")),
    DAILINK_COMP_ARRAY!(COMP_PLATFORM!(c"samsung-i2s.0"))
);

SND_SOC_DAILINK_DEFS!(
    wm5110_dsp_codec,
    DAILINK_COMP_ARRAY!(COMP_CPU!(c"wm0010-sdi2")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(c"wm5110-codec", c"wm5110-aif1"))
);

SND_SOC_DAILINK_DEFS!(
    wm5110_baseband,
    DAILINK_COMP_ARRAY!(COMP_CPU!(c"wm5110-aif2")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(c"wm1250-ev1.1-0027", c"wm1250-ev1"))
);

SND_SOC_DAILINK_DEFS!(
    wm5110_sub,
    DAILINK_COMP_ARRAY!(COMP_CPU!(c"wm5110-aif3")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(c"wm9081.1-006c", c"wm9081-hifi"))
);

static mut bells_dai_wm5110: [snd_soc_dai_link; 4] = [
    snd_soc_dai_link {
        name: c"CPU-DSP".as_ptr(),
        stream_name: c"CPU-DSP".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        SND_SOC_DAILINK_REG!(wm5110_cpu_dsp)
    },
    snd_soc_dai_link {
        name: c"DSP-CODEC".as_ptr(),
        stream_name: c"DSP-CODEC".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        c2c_params: &sub_params,
        num_c2c_params: 1,
        ignore_suspend: 1,
        SND_SOC_DAILINK_REG!(wm5110_dsp_codec)
    },
    snd_soc_dai_link {
        name: c"Baseband".as_ptr(),
        stream_name: c"Baseband".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        ignore_suspend: 1,
        c2c_params: &baseband_params,
        num_c2c_params: 1,
        SND_SOC_DAILINK_REG!(wm5110_baseband)
    },
    snd_soc_dai_link {
        name: c"Sub".as_ptr(),
        stream_name: c"Sub".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        ignore_suspend: 1,
        c2c_params: &sub_params,
        num_c2c_params: 1,
        SND_SOC_DAILINK_REG!(wm5110_sub)
    },
];

static mut bells_codec_conf: [snd_soc_codec_conf; 1] = [snd_soc_codec_conf {
    dlc: COMP_CODEC_CONF!(c"wm9081.1-006c"),
    name_prefix: c"Sub".as_ptr(),
}];

static bells_widgets: [snd_soc_dapm_widget; 1] = [SND_SOC_DAPM_MIC!(c"DMIC", None)];

static bells_routes: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route {
        sink: c"Sub CLK_SYS".as_ptr(),
        control: core::ptr::null(),
        source: c"OPCLK".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"CLKIN".as_ptr(),
        control: core::ptr::null(),
        source: c"OPCLK".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DMIC".as_ptr(),
        control: core::ptr::null(),
        source: c"MICBIAS2".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"IN2L".as_ptr(),
        control: core::ptr::null(),
        source: c"DMIC".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"IN2R".as_ptr(),
        control: core::ptr::null(),
        source: c"DMIC".as_ptr(),
    },
];

static mut bells_cards: [snd_soc_card; 3] = [
    snd_soc_card {
        name: c"Bells WM2200".as_ptr(),
        owner: THIS_MODULE,
        dai_link: unsafe { bells_dai_wm2200.as_mut_ptr() },
        num_links: ARRAY_SIZE!(bells_dai_wm2200),
        codec_conf: unsafe { bells_codec_conf.as_mut_ptr() },
        num_configs: ARRAY_SIZE!(bells_codec_conf),
        late_probe: Some(bells_late_probe),
        dapm_widgets: bells_widgets.as_ptr(),
        num_dapm_widgets: ARRAY_SIZE!(bells_widgets),
        dapm_routes: bells_routes.as_ptr(),
        num_dapm_routes: ARRAY_SIZE!(bells_routes),
        set_bias_level: Some(bells_set_bias_level),
        set_bias_level_post: Some(bells_set_bias_level_post),
        drvdata: unsafe { &mut wm2200_drvdata as *mut bells_drvdata as *mut c_void },
    },
    snd_soc_card {
        name: c"Bells WM5102".as_ptr(),
        owner: THIS_MODULE,
        dai_link: unsafe { bells_dai_wm5102.as_mut_ptr() },
        num_links: ARRAY_SIZE!(bells_dai_wm5102),
        codec_conf: unsafe { bells_codec_conf.as_mut_ptr() },
        num_configs: ARRAY_SIZE!(bells_codec_conf),
        late_probe: Some(bells_late_probe),
        dapm_widgets: bells_widgets.as_ptr(),
        num_dapm_widgets: ARRAY_SIZE!(bells_widgets),
        dapm_routes: bells_routes.as_ptr(),
        num_dapm_routes: ARRAY_SIZE!(bells_routes),
        set_bias_level: Some(bells_set_bias_level),
        set_bias_level_post: Some(bells_set_bias_level_post),
        drvdata: unsafe { &mut wm5102_drvdata as *mut bells_drvdata as *mut c_void },
    },
    snd_soc_card {
        name: c"Bells WM5110".as_ptr(),
        owner: THIS_MODULE,
        dai_link: unsafe { bells_dai_wm5110.as_mut_ptr() },
        num_links: ARRAY_SIZE!(bells_dai_wm5110),
        codec_conf: unsafe { bells_codec_conf.as_mut_ptr() },
        num_configs: ARRAY_SIZE!(bells_codec_conf),
        late_probe: Some(bells_late_probe),
        dapm_widgets: bells_widgets.as_ptr(),
        num_dapm_widgets: ARRAY_SIZE!(bells_widgets),
        dapm_routes: bells_routes.as_ptr(),
        num_dapm_routes: ARRAY_SIZE!(bells_routes),
        set_bias_level: Some(bells_set_bias_level),
        set_bias_level_post: Some(bells_set_bias_level_post),
        drvdata: unsafe { &mut wm5110_drvdata as *mut bells_drvdata as *mut c_void },
    },
];

unsafe extern "C" fn bells_probe(pdev: *mut platform_device) -> c_int {
    let ret: c_int;

    bells_cards[(*pdev).id as usize].dev = &mut (*pdev).dev;

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, &mut bells_cards[(*pdev).id as usize]);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            c"snd_soc_register_card(%s) failed: %d\n".as_ptr(),
            bells_cards[(*pdev).id as usize].name,
            ret,
        );
    }

    ret
}

static mut bells_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"bells".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(bells_probe),
};

module_platform_driver!(bells_driver);

MODULE_DESCRIPTION!(c"Bells audio support");
MODULE_AUTHOR!(c"Mark Brown <broonie@opensource.wolfsonmicro.com>");
MODULE_LICENSE!(c"GPL");
MODULE_ALIAS!(c"platform:bells");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
