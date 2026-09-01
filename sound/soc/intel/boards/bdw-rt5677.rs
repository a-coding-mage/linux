// SPDX-License-Identifier: GPL-2.0-only
/*
 * ASoC machine driver for Intel Broadwell platforms with RT5677 codec
 *
 * Copyright (c) 2014, The Chromium OS Authors.  All rights reserved.
 */

// C dependencies translated as external symbols/types expected from the
// surrounding kernel/ASoC bindings:
// linux/acpi.h, linux/module.h, linux/platform_device.h,
// linux/gpio/consumer.h, linux/delay.h, sound/core.h, sound/pcm.h,
// sound/soc.h, sound/pcm_params.h, sound/jack.h, sound/soc-acpi.h,
// ../../codecs/rt5677.h

#[repr(C)]
pub struct bdw_rt5677_priv {
    pub gpio_hp_en: *mut gpio_desc,
    pub component: *mut snd_soc_component,
}

unsafe extern "C" fn bdw_rt5677_event_hp(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card: *mut snd_soc_card = snd_soc_dapm_to_card((*w).dapm);
    let bdw_rt5677: *mut bdw_rt5677_priv =
        snd_soc_card_get_drvdata(card) as *mut bdw_rt5677_priv;

    if SND_SOC_DAPM_EVENT_ON(event) != 0 {
        msleep(70);
    }

    gpiod_set_value_cansleep((*bdw_rt5677).gpio_hp_en, SND_SOC_DAPM_EVENT_ON(event));

    0
}

static bdw_rt5677_widgets: [snd_soc_dapm_widget; 5] = [
    SND_SOC_DAPM_HP(c"Headphone".as_ptr(), Some(bdw_rt5677_event_hp)),
    SND_SOC_DAPM_SPK(c"Speaker".as_ptr(), None),
    SND_SOC_DAPM_MIC(c"Headset Mic".as_ptr(), None),
    SND_SOC_DAPM_MIC(c"Local DMICs".as_ptr(), None),
    SND_SOC_DAPM_MIC(c"Remote DMICs".as_ptr(), None),
];

static bdw_rt5677_map: [snd_soc_dapm_route; 18] = [
    /* Speakers */
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: core::ptr::null(),
        source: c"PDM1L".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: core::ptr::null(),
        source: c"PDM1R".as_ptr(),
    },
    /* Headset jack connectors */
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: core::ptr::null(),
        source: c"LOUT1".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: core::ptr::null(),
        source: c"LOUT2".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"IN1P".as_ptr(),
        control: core::ptr::null(),
        source: c"Headset Mic".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"IN1N".as_ptr(),
        control: core::ptr::null(),
        source: c"Headset Mic".as_ptr(),
    },
    /* Digital MICs
     * Local DMICs: the two DMICs on the mainboard
     * Remote DMICs: the two DMICs on the camera module
     */
    snd_soc_dapm_route {
        sink: c"DMIC L1".as_ptr(),
        control: core::ptr::null(),
        source: c"Remote DMICs".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DMIC R1".as_ptr(),
        control: core::ptr::null(),
        source: c"Remote DMICs".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DMIC L2".as_ptr(),
        control: core::ptr::null(),
        source: c"Local DMICs".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DMIC R2".as_ptr(),
        control: core::ptr::null(),
        source: c"Local DMICs".as_ptr(),
    },
    /* CODEC BE connections */
    snd_soc_dapm_route {
        sink: c"SSP0 CODEC IN".as_ptr(),
        control: core::ptr::null(),
        source: c"AIF1 Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"AIF1 Playback".as_ptr(),
        control: core::ptr::null(),
        source: c"SSP0 CODEC OUT".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DSP Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"DSP Buffer".as_ptr(),
    },
    /* DSP Clock Connections */
    snd_soc_dapm_route {
        sink: c"DSP Buffer".as_ptr(),
        control: core::ptr::null(),
        source: c"SSP0 CODEC IN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"SSP0 CODEC IN".as_ptr(),
        control: core::ptr::null(),
        source: c"DSPTX".as_ptr(),
    },
];

static bdw_rt5677_controls: [snd_kcontrol_new; 5] = [
    SOC_DAPM_PIN_SWITCH(c"Speaker".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Headphone".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Headset Mic".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Local DMICs".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Remote DMICs".as_ptr()),
];

static mut headphone_jack: snd_soc_jack = unsafe { core::mem::zeroed() };
static mut mic_jack: snd_soc_jack = unsafe { core::mem::zeroed() };

static mut headphone_jack_pin: snd_soc_jack_pin = snd_soc_jack_pin {
    pin: c"Headphone".as_ptr(),
    mask: SND_JACK_HEADPHONE,
};

static mut mic_jack_pin: snd_soc_jack_pin = snd_soc_jack_pin {
    pin: c"Headset Mic".as_ptr(),
    mask: SND_JACK_MICROPHONE,
};

static mut headphone_jack_gpio: snd_soc_jack_gpio = snd_soc_jack_gpio {
    name: c"plug-det".as_ptr(),
    report: SND_JACK_HEADPHONE,
    debounce_time: 200,
    ..unsafe { core::mem::zeroed() }
};

static mut mic_jack_gpio: snd_soc_jack_gpio = snd_soc_jack_gpio {
    name: c"mic-present".as_ptr(),
    report: SND_JACK_MICROPHONE,
    debounce_time: 200,
    invert: 1,
    ..unsafe { core::mem::zeroed() }
};

/* GPIO indexes defined by ACPI */
const RT5677_GPIO_PLUG_DET: c_uint = 0;
const RT5677_GPIO_MIC_PRESENT_L: c_uint = 1;
const RT5677_GPIO_HOTWORD_DET_L: c_uint = 2;
const RT5677_GPIO_DSP_INT: c_uint = 3;
const RT5677_GPIO_HP_AMP_SHDN_L: c_uint = 4;

static plug_det_gpio: acpi_gpio_params =
    acpi_gpio_params { crs_entry_index: RT5677_GPIO_PLUG_DET, line_index: 0, active_low: false };
static mic_present_gpio: acpi_gpio_params =
    acpi_gpio_params { crs_entry_index: RT5677_GPIO_MIC_PRESENT_L, line_index: 0, active_low: false };
static headphone_enable_gpio: acpi_gpio_params =
    acpi_gpio_params { crs_entry_index: RT5677_GPIO_HP_AMP_SHDN_L, line_index: 0, active_low: false };

static bdw_rt5677_gpios: [acpi_gpio_mapping; 4] = [
    acpi_gpio_mapping {
        name: c"plug-det-gpios".as_ptr(),
        data: &plug_det_gpio,
        size: 1,
    },
    acpi_gpio_mapping {
        name: c"mic-present-gpios".as_ptr(),
        data: &mic_present_gpio,
        size: 1,
    },
    acpi_gpio_mapping {
        name: c"headphone-enable-gpios".as_ptr(),
        data: &headphone_enable_gpio,
        size: 1,
    },
    acpi_gpio_mapping {
        name: core::ptr::null(),
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe extern "C" fn broadwell_ssp0_fixup(
    _rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let chan: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);

    /* The ADSP will convert the FE rate to 48k, stereo */
    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*chan).max = 2;
    (*chan).min = (*chan).max;

    /* set SSP0 to 16 bit */
    params_set_format(params, SNDRV_PCM_FORMAT_S16_LE);
    0
}

unsafe extern "C" fn bdw_rt5677_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut ret: c_int;

    ret = snd_soc_dai_set_sysclk(
        codec_dai,
        RT5677_SCLK_S_MCLK,
        24576000,
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        dev_err((*rtd).dev, c"can't set codec sysclk configuration\n".as_ptr());
        return ret;
    }

    ret
}

unsafe extern "C" fn bdw_rt5677_dsp_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut ret: c_int;

    ret = snd_soc_dai_set_sysclk(
        codec_dai,
        RT5677_SCLK_S_PLL1,
        24576000,
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        dev_err((*rtd).dev, c"can't set codec sysclk configuration\n".as_ptr());
        return ret;
    }
    ret = snd_soc_dai_set_pll(codec_dai, 0, RT5677_PLL1_S_MCLK, 24000000, 24576000);
    if ret < 0 {
        dev_err((*rtd).dev, c"can't set codec pll configuration\n".as_ptr());
        return ret;
    }

    0
}

static bdw_rt5677_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(bdw_rt5677_hw_params),
    ..unsafe { core::mem::zeroed() }
};

static bdw_rt5677_dsp_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(bdw_rt5677_dsp_hw_params),
    ..unsafe { core::mem::zeroed() }
};

static channels: [c_uint; 1] = [2];

static constraints_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: channels.len() as c_uint,
    list: channels.as_ptr(),
    mask: 0,
};

unsafe extern "C" fn bdw_rt5677_fe_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    /* Board supports stereo configuration only */
    (*runtime).hw.channels_max = 2;
    snd_pcm_hw_constraint_list(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        &constraints_channels,
    )
}

static bdw_rt5677_fe_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(bdw_rt5677_fe_startup),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn bdw_rt5677_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let bdw_rt5677: *mut bdw_rt5677_priv =
        snd_soc_card_get_drvdata((*rtd).card) as *mut bdw_rt5677_priv;
    let component: *mut snd_soc_component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    ret = devm_acpi_dev_add_driver_gpios((*component).dev, bdw_rt5677_gpios.as_ptr());
    if ret != 0 {
        dev_warn((*component).dev, c"Failed to add driver gpios\n".as_ptr());
    }

    /* Enable codec ASRC function for Stereo DAC/Stereo1 ADC/DMIC/I2S1.
     * The ASRC clock source is clk_i2s1_asrc.
     */
    rt5677_sel_asrc_clk_src(
        component,
        RT5677_DA_STEREO_FILTER | RT5677_AD_STEREO1_FILTER | RT5677_I2S1_SOURCE,
        RT5677_CLK_SEL_I2S1_ASRC,
    );
    /* Enable codec ASRC function for Mono ADC L.
     * The ASRC clock source is clk_sys2_asrc.
     */
    rt5677_sel_asrc_clk_src(component, RT5677_AD_MONO_L_FILTER, RT5677_CLK_SEL_SYS2);

    /* Request rt5677 GPIO for headphone amp control */
    (*bdw_rt5677).gpio_hp_en =
        gpiod_get((*component).dev, c"headphone-enable".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*bdw_rt5677).gpio_hp_en as *const c_void) != 0 {
        dev_err((*component).dev, c"Can't find HP_AMP_SHDN_L gpio\n".as_ptr());
        return PTR_ERR((*bdw_rt5677).gpio_hp_en as *const c_void);
    }

    /* Create and initialize headphone jack */
    if snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"Headphone Jack".as_ptr(),
        SND_JACK_HEADPHONE,
        &raw mut headphone_jack,
        &raw mut headphone_jack_pin,
        1,
    ) == 0
    {
        headphone_jack_gpio.gpiod_dev = (*component).dev;
        if snd_soc_jack_add_gpios(&raw mut headphone_jack, 1, &raw mut headphone_jack_gpio) != 0 {
            dev_err((*component).dev, c"Can't add headphone jack gpio\n".as_ptr());
        }
    } else {
        dev_err((*component).dev, c"Can't create headphone jack\n".as_ptr());
    }

    /* Create and initialize mic jack */
    if snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"Mic Jack".as_ptr(),
        SND_JACK_MICROPHONE,
        &raw mut mic_jack,
        &raw mut mic_jack_pin,
        1,
    ) == 0
    {
        mic_jack_gpio.gpiod_dev = (*component).dev;
        if snd_soc_jack_add_gpios(&raw mut mic_jack, 1, &raw mut mic_jack_gpio) != 0 {
            dev_err((*component).dev, c"Can't add mic jack gpio\n".as_ptr());
        }
    } else {
        dev_err((*component).dev, c"Can't create mic jack\n".as_ptr());
    }
    (*bdw_rt5677).component = component;

    snd_soc_dapm_force_enable_pin(dapm, c"MICBIAS1".as_ptr());
    0
}

unsafe extern "C" fn bdw_rt5677_exit(rtd: *mut snd_soc_pcm_runtime) {
    let bdw_rt5677: *mut bdw_rt5677_priv =
        snd_soc_card_get_drvdata((*rtd).card) as *mut bdw_rt5677_priv;

    /*
     * The .exit() can be reached without going through the .init()
     * so explicitly test if the gpiod is valid
     */
    if IS_ERR_OR_NULL((*bdw_rt5677).gpio_hp_en as *const c_void) == 0 {
        gpiod_put((*bdw_rt5677).gpio_hp_en);
    }
}

/* broadwell digital audio interface glue - connects codec <--> CPU */
// SND_SOC_DAILINK_DEF(dummy, DAILINK_COMP_ARRAY(COMP_DUMMY()));
// SND_SOC_DAILINK_DEF(fe, DAILINK_COMP_ARRAY(COMP_CPU("System Pin")));
// SND_SOC_DAILINK_DEF(platform, DAILINK_COMP_ARRAY(COMP_PLATFORM("haswell-pcm-audio")));
// SND_SOC_DAILINK_DEF(be, DAILINK_COMP_ARRAY(COMP_CODEC("i2c-RT5677CE:00", "rt5677-aif1")));
// SND_SOC_DAILINK_DEF(ssp0_port, DAILINK_COMP_ARRAY(COMP_CPU("ssp0-port")));

/* Wake on voice interface */
// SND_SOC_DAILINK_DEFS(dsp,
//     DAILINK_COMP_ARRAY(COMP_CPU("spi-RT5677AA:00")),
//     DAILINK_COMP_ARRAY(COMP_CODEC("i2c-RT5677CE:00", "rt5677-dspbuffer")),
//     DAILINK_COMP_ARRAY(COMP_PLATFORM("spi-RT5677AA:00")));

static mut bdw_rt5677_dais: [snd_soc_dai_link; 3] = [
    /* Front End DAI links */
    snd_soc_dai_link {
        name: c"System PCM".as_ptr(),
        stream_name: c"System Playback/Capture".as_ptr(),
        nonatomic: 1,
        dynamic: 1,
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        ops: &bdw_rt5677_fe_ops,
        // SND_SOC_DAILINK_REG(fe, dummy, platform)
        ..unsafe { core::mem::zeroed() }
    },
    /* Non-DPCM links */
    snd_soc_dai_link {
        name: c"Codec DSP".as_ptr(),
        stream_name: c"Wake on Voice".as_ptr(),
        capture_only: 1,
        ops: &bdw_rt5677_dsp_ops,
        // SND_SOC_DAILINK_REG(dsp)
        ..unsafe { core::mem::zeroed() }
    },
    /* Back End DAI links */
    snd_soc_dai_link {
        /* SSP0 - Codec */
        name: c"Codec".as_ptr(),
        id: 0,
        nonatomic: 1,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        ignore_pmdown_time: 1,
        be_hw_params_fixup: Some(broadwell_ssp0_fixup),
        ops: &bdw_rt5677_ops,
        init: Some(bdw_rt5677_init),
        exit: Some(bdw_rt5677_exit),
        // SND_SOC_DAILINK_REG(ssp0_port, be, platform)
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe extern "C" fn bdw_rt5677_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let bdw_rt5677: *mut bdw_rt5677_priv =
        snd_soc_card_get_drvdata(card) as *mut bdw_rt5677_priv;
    let mut dapm: *mut snd_soc_dapm_context;

    if !(*bdw_rt5677).component.is_null() {
        dapm = snd_soc_component_to_dapm((*bdw_rt5677).component);
        snd_soc_dapm_disable_pin(dapm, c"MICBIAS1".as_ptr());
    }
    0
}

unsafe extern "C" fn bdw_rt5677_resume_post(card: *mut snd_soc_card) -> c_int {
    let bdw_rt5677: *mut bdw_rt5677_priv =
        snd_soc_card_get_drvdata(card) as *mut bdw_rt5677_priv;
    let mut dapm: *mut snd_soc_dapm_context;

    if !(*bdw_rt5677).component.is_null() {
        dapm = snd_soc_component_to_dapm((*bdw_rt5677).component);
        snd_soc_dapm_force_enable_pin(dapm, c"MICBIAS1".as_ptr());
    }
    0
}

/* use space before codec name to simplify card ID, and simplify driver name */
const SOF_CARD_NAME: *const c_char = c"bdw rt5677".as_ptr(); /* card name will be 'sof-bdw rt5677' */
const SOF_DRIVER_NAME: *const c_char = c"SOF".as_ptr();

const CARD_NAME: *const c_char = c"bdw-rt5677".as_ptr();
const DRIVER_NAME: *const c_char = core::ptr::null(); /* card name will be used for driver name */

/* ASoC machine driver for Broadwell DSP + RT5677 */
static mut bdw_rt5677_card: snd_soc_card = snd_soc_card {
    name: CARD_NAME,
    driver_name: DRIVER_NAME,
    owner: THIS_MODULE,
    dai_link: unsafe { bdw_rt5677_dais.as_mut_ptr() },
    num_links: unsafe { bdw_rt5677_dais.len() as c_int },
    dapm_widgets: bdw_rt5677_widgets.as_ptr(),
    num_dapm_widgets: bdw_rt5677_widgets.len() as c_int,
    dapm_routes: bdw_rt5677_map.as_ptr(),
    num_dapm_routes: bdw_rt5677_map.len() as c_int,
    controls: bdw_rt5677_controls.as_ptr(),
    num_controls: bdw_rt5677_controls.len() as c_int,
    fully_routed: true,
    suspend_pre: Some(bdw_rt5677_suspend_pre),
    resume_post: Some(bdw_rt5677_resume_post),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn bdw_rt5677_probe(pdev: *mut platform_device) -> c_int {
    let mut bdw_rt5677: *mut bdw_rt5677_priv;
    let mut mach: *mut snd_soc_acpi_mach;
    let mut ret: c_int;

    bdw_rt5677_card.dev = &raw mut (*pdev).dev;

    /* Allocate driver private struct */
    bdw_rt5677 = devm_kzalloc(
        &raw mut (*pdev).dev,
        core::mem::size_of::<bdw_rt5677_priv>(),
        GFP_KERNEL,
    ) as *mut bdw_rt5677_priv;
    if bdw_rt5677.is_null() {
        return -ENOMEM;
    }

    /* override platform name, if required */
    mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;
    ret = snd_soc_fixup_dai_links_platform_name(
        &raw mut bdw_rt5677_card,
        (*mach).mach_params.platform,
    );
    if ret != 0 {
        return ret;
    }

    /* set card and driver name */
    if snd_soc_acpi_sof_parent(&raw mut (*pdev).dev) != 0 {
        bdw_rt5677_card.name = SOF_CARD_NAME;
        bdw_rt5677_card.driver_name = SOF_DRIVER_NAME;
    } else {
        bdw_rt5677_card.name = CARD_NAME;
        bdw_rt5677_card.driver_name = DRIVER_NAME;
    }

    snd_soc_card_set_drvdata(&raw mut bdw_rt5677_card, bdw_rt5677 as *mut c_void);

    devm_snd_soc_register_card(&raw mut (*pdev).dev, &raw mut bdw_rt5677_card)
}

static mut bdw_rt5677_audio: platform_driver = platform_driver {
    probe: Some(bdw_rt5677_probe),
    driver: device_driver {
        name: c"bdw-rt5677".as_ptr(),
        pm: &raw const snd_soc_pm_ops,
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(bdw_rt5677_audio);

/* Module information */
MODULE_AUTHOR!(c"Ben Zhang");
MODULE_DESCRIPTION!(c"Intel Broadwell RT5677 machine driver");
MODULE_LICENSE!(c"GPL v2");
MODULE_ALIAS!(c"platform:bdw-rt5677");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
