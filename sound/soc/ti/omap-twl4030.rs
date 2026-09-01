// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap-twl4030.c  --  SoC audio for TI SoC based boards with twl4030 codec
 *
 * Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com
 * All rights reserved.
 *
 * Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 *
 * This driver replaces the following machine drivers:
 * omap3beagle (Author: Steve Sakoman <steve@sakoman.com>)
 * omap3evm (Author: Anuj Aggarwal <anuj.aggarwal@ti.com>)
 * overo (Author: Steve Sakoman <steve@sakoman.com>)
 * igep0020 (Author: Enric Balletbo i Serra <eballetbo@iseebcn.com>)
 * zoom2 (Author: Misael Lopez Cruz <misael.lopez@ti.com>)
 * sdp3430 (Author: Misael Lopez Cruz <misael.lopez@ti.com>)
 */

// C dependencies: linux/platform_device.h, linux/module.h, linux/of.h,
// sound/core.h, sound/pcm.h, sound/soc.h, sound/jack.h, "omap-mcbsp.h".

#[repr(C)]
pub struct omap_twl4030 {
    pub hs_jack_gpio: snd_soc_jack_gpio,
    pub hs_jack: snd_soc_jack,
}

unsafe extern "C" fn omap_twl4030_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let fmt: c_uint;

    match params_channels(params) {
        2 => {
            /* Stereo I2S mode */
            fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;
        }
        4 => {
            /* Four channel TDM mode */
            fmt = SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_IB_NF | SND_SOC_DAIFMT_CBP_CFP;
        }
        _ => {
            return -EINVAL;
        }
    }

    snd_soc_runtime_set_dai_fmt(rtd, fmt)
}

static omap_twl4030_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(omap_twl4030_hw_params),
    ..unsafe { core::mem::zeroed() }
};

static dapm_widgets: [snd_soc_dapm_widget; 12] = [
    SND_SOC_DAPM_SPK!("Earpiece Spk", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK!("Handsfree Spk", core::ptr::null_mut()),
    SND_SOC_DAPM_HP!("Headset Stereophone", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK!("Ext Spk", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK!("Carkit Spk", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC!("Main Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC!("Sub Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC!("Headset Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC!("Carkit Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC!("Digital0 Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC!("Digital1 Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE!("Line In", core::ptr::null_mut()),
];

static audio_map: [snd_soc_dapm_route; 23] = [
    /* Headset Stereophone:  HSOL, HSOR */
    snd_soc_dapm_route { sink: c_str!("Headset Stereophone"), control: core::ptr::null(), source: c_str!("HSOL"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("Headset Stereophone"), control: core::ptr::null(), source: c_str!("HSOR"), ..unsafe { core::mem::zeroed() } },
    /* External Speakers: HFL, HFR */
    snd_soc_dapm_route { sink: c_str!("Handsfree Spk"), control: core::ptr::null(), source: c_str!("HFL"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("Handsfree Spk"), control: core::ptr::null(), source: c_str!("HFR"), ..unsafe { core::mem::zeroed() } },
    /* External Speakers: PredrivL, PredrivR */
    snd_soc_dapm_route { sink: c_str!("Ext Spk"), control: core::ptr::null(), source: c_str!("PREDRIVEL"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("Ext Spk"), control: core::ptr::null(), source: c_str!("PREDRIVER"), ..unsafe { core::mem::zeroed() } },
    /* Carkit speakers:  CARKITL, CARKITR */
    snd_soc_dapm_route { sink: c_str!("Carkit Spk"), control: core::ptr::null(), source: c_str!("CARKITL"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("Carkit Spk"), control: core::ptr::null(), source: c_str!("CARKITR"), ..unsafe { core::mem::zeroed() } },
    /* Earpiece */
    snd_soc_dapm_route { sink: c_str!("Earpiece Spk"), control: core::ptr::null(), source: c_str!("EARPIECE"), ..unsafe { core::mem::zeroed() } },
    /* External Mics: MAINMIC, SUBMIC with bias */
    snd_soc_dapm_route { sink: c_str!("MAINMIC"), control: core::ptr::null(), source: c_str!("Main Mic"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("Main Mic"), control: core::ptr::null(), source: c_str!("Mic Bias 1"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("SUBMIC"), control: core::ptr::null(), source: c_str!("Sub Mic"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("Sub Mic"), control: core::ptr::null(), source: c_str!("Mic Bias 2"), ..unsafe { core::mem::zeroed() } },
    /* Headset Mic: HSMIC with bias */
    snd_soc_dapm_route { sink: c_str!("HSMIC"), control: core::ptr::null(), source: c_str!("Headset Mic"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("Headset Mic"), control: core::ptr::null(), source: c_str!("Headset Mic Bias"), ..unsafe { core::mem::zeroed() } },
    /* Digital Mics: DIGIMIC0, DIGIMIC1 with bias */
    snd_soc_dapm_route { sink: c_str!("DIGIMIC0"), control: core::ptr::null(), source: c_str!("Digital0 Mic"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("Digital0 Mic"), control: core::ptr::null(), source: c_str!("Mic Bias 1"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("DIGIMIC1"), control: core::ptr::null(), source: c_str!("Digital1 Mic"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("Digital1 Mic"), control: core::ptr::null(), source: c_str!("Mic Bias 2"), ..unsafe { core::mem::zeroed() } },
    /* Carkit In: CARKITMIC */
    snd_soc_dapm_route { sink: c_str!("CARKITMIC"), control: core::ptr::null(), source: c_str!("Carkit Mic"), ..unsafe { core::mem::zeroed() } },
    /* Aux In: AUXL, AUXR */
    snd_soc_dapm_route { sink: c_str!("AUXL"), control: core::ptr::null(), source: c_str!("Line In"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("AUXR"), control: core::ptr::null(), source: c_str!("Line In"), ..unsafe { core::mem::zeroed() } },
];

/* Headset jack detection DAPM pins */
static mut hs_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c_str!("Headset Mic"),
        mask: SND_JACK_MICROPHONE,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_jack_pin {
        pin: c_str!("Headset Stereophone"),
        mask: SND_JACK_HEADPHONE,
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe extern "C" fn omap_twl4030_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let priv_: *mut omap_twl4030 = snd_soc_card_get_drvdata(card) as *mut omap_twl4030;
    let mut ret: c_int;

    /*
     * This is a bit of a hack, but the GPIO is optional so we
     * only want to add the jack detection if the GPIO is there.
     */
    if of_property_present((*(*card).dev).of_node, c_str!("ti,jack-det-gpio")) {
        ret = snd_soc_card_jack_new_pins(
            (*rtd).card,
            c_str!("Headset Jack"),
            SND_JACK_HEADSET,
            &mut (*priv_).hs_jack,
            hs_jack_pins.as_mut_ptr(),
            ARRAY_SIZE(&hs_jack_pins),
        );
        if ret != 0 {
            return ret;
        }

        (*priv_).hs_jack_gpio.name = c_str!("ti,jack-det");
        (*priv_).hs_jack_gpio.report = SND_JACK_HEADSET;
        (*priv_).hs_jack_gpio.debounce_time = 200;
        (*priv_).hs_jack_gpio.gpiod_dev = (*card).dev;
        (*priv_).hs_jack_gpio.idx = 0;

        ret = snd_soc_jack_add_gpios(&mut (*priv_).hs_jack, 1, &mut (*priv_).hs_jack_gpio);
        if ret != 0 {
            return ret;
        }
    }

    0
}

/* Digital audio interface glue - connects codec <--> CPU */
SND_SOC_DAILINK_DEFS!(
    hifi,
    DAILINK_COMP_ARRAY!(COMP_CPU!("omap-mcbsp.2")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!("twl4030-codec", "twl4030-hifi")),
    DAILINK_COMP_ARRAY!(COMP_PLATFORM!("omap-mcbsp.2"))
);

SND_SOC_DAILINK_DEFS!(
    voice,
    DAILINK_COMP_ARRAY!(COMP_CPU!("omap-mcbsp.3")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!("twl4030-codec", "twl4030-voice")),
    DAILINK_COMP_ARRAY!(COMP_PLATFORM!("omap-mcbsp.3"))
);

static mut omap_twl4030_dai_links: [snd_soc_dai_link; 2] = [
    snd_soc_dai_link {
        name: c_str!("TWL4030 HiFi"),
        stream_name: c_str!("TWL4030 HiFi"),
        init: Some(omap_twl4030_init),
        ops: &omap_twl4030_ops,
        SND_SOC_DAILINK_REG!(hifi)
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_link {
        name: c_str!("TWL4030 Voice"),
        stream_name: c_str!("TWL4030 Voice"),
        dai_fmt: SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_IB_NF | SND_SOC_DAIFMT_CBP_CFP,
        SND_SOC_DAILINK_REG!(voice)
        ..unsafe { core::mem::zeroed() }
    },
];

/* Audio machine driver */
static mut omap_twl4030_card: snd_soc_card = snd_soc_card {
    owner: THIS_MODULE,
    dai_link: unsafe { omap_twl4030_dai_links.as_mut_ptr() },
    num_links: ARRAY_SIZE(unsafe { &omap_twl4030_dai_links }),
    dapm_widgets: dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&dapm_widgets),
    dapm_routes: audio_map.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&audio_map),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn omap_twl4030_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &mut omap_twl4030_card;
    let mut node: *mut device_node;
    let mut dai_node: *mut device_node;
    let priv_: *mut omap_twl4030;
    let mut prop: *mut property;
    let mut ret: c_int;

    node = (*pdev).dev.of_node;
    if node.is_null() {
        return -ENODEV;
    }

    (*card).dev = &mut (*pdev).dev;

    priv_ = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<omap_twl4030>(),
        GFP_KERNEL,
    ) as *mut omap_twl4030;
    if priv_.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_of_parse_card_name(card, c_str!("ti,model"));
    if ret != 0 {
        return ret;
    }

    if (*card).name.is_null() {
        dev_err(&mut (*pdev).dev, c_str!("Card name is not provided\n"));
        return -ENODEV;
    }

    dai_node = of_parse_phandle(node, c_str!("ti,mcbsp"), 0);
    if dai_node.is_null() {
        dev_err(&mut (*pdev).dev, c_str!("McBSP node is not provided\n"));
        return -EINVAL;
    }
    (*(*omap_twl4030_dai_links[0].cpus)).dai_name = core::ptr::null();
    (*(*omap_twl4030_dai_links[0].cpus)).of_node = dai_node;

    (*(*omap_twl4030_dai_links[0].platforms)).name = core::ptr::null();
    (*(*omap_twl4030_dai_links[0].platforms)).of_node = dai_node;

    dai_node = of_parse_phandle(node, c_str!("ti,mcbsp-voice"), 0);
    if dai_node.is_null() {
        (*card).num_links = 1;
    } else {
        (*(*omap_twl4030_dai_links[1].cpus)).dai_name = core::ptr::null();
        (*(*omap_twl4030_dai_links[1].cpus)).of_node = dai_node;

        (*(*omap_twl4030_dai_links[1].platforms)).name = core::ptr::null();
        (*(*omap_twl4030_dai_links[1].platforms)).of_node = dai_node;
    }

    /* Optional: audio routing can be provided */
    prop = of_find_property(node, c_str!("ti,audio-routing"), core::ptr::null_mut());
    if !prop.is_null() {
        ret = snd_soc_of_parse_audio_routing(card, c_str!("ti,audio-routing"));
        if ret != 0 {
            return ret;
        }

        (*card).fully_routed = 1;
    }

    snd_soc_card_set_drvdata(card, priv_ as *mut c_void);
    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        return dev_err_probe(
            &mut (*pdev).dev,
            ret,
            c_str!("devm_snd_soc_register_card() failed\n"),
        );
    }

    0
}

static omap_twl4030_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c_str!("ti,omap-twl4030"),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

/* MODULE_DEVICE_TABLE(of, omap_twl4030_of_match); */

static mut omap_twl4030_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("omap-twl4030"),
        pm: unsafe { &snd_soc_pm_ops },
        of_match_table: omap_twl4030_of_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(omap_twl4030_probe),
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(omap_twl4030_driver);

MODULE_AUTHOR!("Peter Ujfalusi <peter.ujfalusi@ti.com>");
MODULE_DESCRIPTION!("ALSA SoC for TI SoC based boards with twl4030 codec");
MODULE_LICENSE!("GPL");
MODULE_ALIAS!("platform:omap-twl4030");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
