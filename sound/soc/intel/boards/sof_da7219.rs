// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2019 Intel Corporation.

/*
 * Intel SOF Machine driver for Dialog headphone codec
 */

/* C dependencies:
 * linux/input.h, linux/module.h, sound/jack.h, sound/pcm.h,
 * sound/pcm_params.h, linux/platform_device.h, sound/soc.h,
 * sound/soc-acpi.h, sound/sof.h, ../../codecs/da7219.h,
 * sof_board_helpers.h, sof_maxim_common.h
 */

/* Driver-specific board quirks: from bit 0 to 7 */
const SOF_DA7219_GLK_BOARD: c_ulong = BIT(0) as c_ulong;
const SOF_DA7219_CML_BOARD: c_ulong = BIT(1) as c_ulong;
const SOF_DA7219_JSL_BOARD: c_ulong = BIT(2) as c_ulong;
const SOF_DA7219_MCLK_EN: c_ulong = BIT(3) as c_ulong;

const DIALOG_CODEC_DAI: *const c_char = c_str!("da7219-hifi");

unsafe extern "C" fn platform_clock_control(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card: *mut snd_soc_card = snd_soc_dapm_to_card((*w).dapm);
    let ctx: *mut sof_card_private = snd_soc_card_get_drvdata(card) as *mut sof_card_private;
    let codec_dai: *mut snd_soc_dai;
    let mut ret: c_int = 0;

    if (*ctx).da7219.pll_bypass {
        return ret;
    }

    /* PLL SRM mode */
    codec_dai = snd_soc_card_get_codec_dai(card, DIALOG_CODEC_DAI);
    if codec_dai.is_null() {
        dev_err(
            (*card).dev,
            c_str!("Codec dai not found; Unable to set/unset codec pll\n"),
        );
        return -EIO;
    }

    if SND_SOC_DAPM_EVENT_OFF(event) {
        ret = snd_soc_dai_set_pll(codec_dai, 0, DA7219_SYSCLK_MCLK, 0, 0);
        if ret != 0 {
            dev_err((*card).dev, c_str!("failed to stop PLL: %d\n"), ret);
        }
    } else if SND_SOC_DAPM_EVENT_ON(event) {
        dev_dbg((*card).dev, c_str!("pll srm mode\n"));

        ret = snd_soc_dai_set_pll(
            codec_dai,
            0,
            DA7219_SYSCLK_PLL_SRM,
            0,
            DA7219_PLL_FREQ_OUT_98304,
        );
        if ret != 0 {
            dev_err((*card).dev, c_str!("failed to start PLL: %d\n"), ret);
        }
    }

    ret
}

static controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_PIN_SWITCH(c_str!("Headphone Jack")),
    SOC_DAPM_PIN_SWITCH(c_str!("Headset Mic")),
    SOC_DAPM_PIN_SWITCH(c_str!("Line Out")),
];

static widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_HP(c_str!("Headphone Jack"), core::ptr::null()),
    SND_SOC_DAPM_MIC(c_str!("Headset Mic"), core::ptr::null()),
    SND_SOC_DAPM_LINE(c_str!("Line Out"), core::ptr::null()),
    SND_SOC_DAPM_SUPPLY(
        c_str!("Platform Clock"),
        SND_SOC_NOPM,
        0,
        0,
        Some(platform_clock_control),
        SND_SOC_DAPM_POST_PMD | SND_SOC_DAPM_PRE_PMU,
    ),
];

static audio_map: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route {
        sink: c_str!("Headphone Jack"),
        control: core::ptr::null(),
        source: c_str!("HPL"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Headphone Jack"),
        control: core::ptr::null(),
        source: c_str!("HPR"),
    },
    snd_soc_dapm_route {
        sink: c_str!("MIC"),
        control: core::ptr::null(),
        source: c_str!("Headset Mic"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Headphone Jack"),
        control: core::ptr::null(),
        source: c_str!("Platform Clock"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Headset Mic"),
        control: core::ptr::null(),
        source: c_str!("Platform Clock"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Line Out"),
        control: core::ptr::null(),
        source: c_str!("Platform Clock"),
    },
];

static mut jack_pins: [snd_soc_jack_pin; 3] = [
    snd_soc_jack_pin {
        pin: c_str!("Headphone Jack"),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c_str!("Headset Mic"),
        mask: SND_JACK_MICROPHONE,
    },
    snd_soc_jack_pin {
        pin: c_str!("Line Out"),
        mask: SND_JACK_LINEOUT,
    },
];

unsafe extern "C" fn da7219_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let ctx: *mut sof_card_private =
        snd_soc_card_get_drvdata((*rtd).card) as *mut sof_card_private;
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component: *mut snd_soc_component = (*codec_dai).component;
    let jack: *mut snd_soc_jack = &mut (*ctx).headset_jack;
    let mclk_rate: c_int;
    let mut ret: c_int;

    mclk_rate = sof_dai_get_mclk(rtd);
    if mclk_rate <= 0 {
        dev_err((*rtd).dev, c_str!("invalid mclk freq %d\n"), mclk_rate);
        return -EINVAL;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, DA7219_CLKSRC_MCLK, mclk_rate, SND_SOC_CLOCK_IN);
    if ret != 0 {
        dev_err((*rtd).dev, c_str!("fail to set sysclk, ret %d\n"), ret);
        return ret;
    }

    /*
     * Use PLL bypass mode if MCLK is available, be sure to set the
     * frequency of MCLK to 12.288 or 24.576MHz on topology side.
     */
    if (*ctx).da7219.mclk_en && (mclk_rate == 12288000 || mclk_rate == 24576000) {
        /* PLL bypass mode */
        dev_dbg(
            (*rtd).dev,
            c_str!("pll bypass mode, mclk rate %d\n"),
            mclk_rate,
        );

        ret = snd_soc_dai_set_pll(codec_dai, 0, DA7219_SYSCLK_MCLK, 0, 0);
        if ret != 0 {
            dev_err((*rtd).dev, c_str!("fail to set pll, ret %d\n"), ret);
            return ret;
        }

        (*ctx).da7219.pll_bypass = true;
    }

    /*
     * Headset buttons map to the google Reference headset.
     * These can be configured by userspace.
     */
    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c_str!("Headset Jack"),
        SND_JACK_HEADSET
            | SND_JACK_BTN_0
            | SND_JACK_BTN_1
            | SND_JACK_BTN_2
            | SND_JACK_BTN_3
            | SND_JACK_LINEOUT,
        jack,
        jack_pins.as_mut_ptr(),
        ARRAY_SIZE(&jack_pins),
    );
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            c_str!("Headset Jack creation failed: %d\n"),
            ret,
        );
        return ret;
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOICECOMMAND);

    ret = snd_soc_component_set_jack(component, jack, core::ptr::null_mut());
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            c_str!("fail to set component jack, ret %d\n"),
            ret,
        );
        return ret;
    }

    ret
}

unsafe extern "C" fn da7219_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
    let component: *mut snd_soc_component = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    snd_soc_component_set_jack(component, core::ptr::null_mut(), core::ptr::null_mut());
}

unsafe extern "C" fn card_late_probe(card: *mut snd_soc_card) -> c_int {
    let ctx: *mut sof_card_private = snd_soc_card_get_drvdata(card) as *mut sof_card_private;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let err: c_int;

    if (*ctx).amp_type == CODEC_MAX98373 {
        /* Disable Left and Right Spk pin after boot */
        snd_soc_dapm_disable_pin(dapm, c_str!("Left Spk"));
        snd_soc_dapm_disable_pin(dapm, c_str!("Right Spk"));
        err = snd_soc_dapm_sync(dapm);
        if err < 0 {
            return err;
        }
    }

    sof_intel_board_card_late_probe(card)
}

static mut card_da7219: snd_soc_card = snd_soc_card {
    name: c_str!("da7219"), /* the sof- prefix is added by the core */
    owner: THIS_MODULE,
    controls: controls.as_ptr(),
    num_controls: ARRAY_SIZE(&controls),
    dapm_widgets: widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&widgets),
    dapm_routes: audio_map.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&audio_map),
    fully_routed: true,
    late_probe: Some(card_late_probe),
    ..unsafe { core::mem::zeroed() }
};

static mut da7219_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c_str!("i2c-DLGS7219:00"),
    dai_name: DIALOG_CODEC_DAI,
    ..unsafe { core::mem::zeroed() }
}];

unsafe fn sof_card_dai_links_create(
    dev: *mut device,
    card: *mut snd_soc_card,
    ctx: *mut sof_card_private,
) -> c_int {
    let mut ret: c_int;

    ret = sof_intel_board_set_dai_link(dev, card, ctx);
    if ret != 0 {
        return ret;
    }

    if (*ctx).codec_link.is_null() {
        dev_err(dev, c_str!("codec link not available"));
        return -EINVAL;
    }

    /* codec-specific fields for headphone codec */
    (*(*ctx).codec_link).codecs = da7219_component.as_mut_ptr();
    (*(*ctx).codec_link).num_codecs = ARRAY_SIZE(&da7219_component);
    (*(*ctx).codec_link).init = Some(da7219_codec_init);
    (*(*ctx).codec_link).exit = Some(da7219_codec_exit);

    if (*ctx).amp_type == CODEC_NONE {
        return 0;
    }

    if (*ctx).amp_link.is_null() {
        dev_err(dev, c_str!("amp link not available"));
        return -EINVAL;
    }

    /* codec-specific fields for speaker amplifier */
    match (*ctx).amp_type {
        CODEC_MAX98357A => {
            max_98357a_dai_link((*ctx).amp_link);
        }
        CODEC_MAX98360A => {
            max_98360a_dai_link((*ctx).amp_link);
        }
        CODEC_MAX98373 => {
            max_98373_dai_link(dev, (*ctx).amp_link);
        }
        CODEC_MAX98390 => {
            max_98390_dai_link(dev, (*ctx).amp_link);
        }
        _ => {
            dev_err(dev, c_str!("invalid amp type %d\n"), (*ctx).amp_type);
            return -EINVAL;
        }
    }

    0
}

const GLK_LINK_ORDER: c_ulong = SOF_LINK_ORDER(
    SOF_LINK_AMP,
    SOF_LINK_CODEC,
    SOF_LINK_DMIC01,
    SOF_LINK_IDISP_HDMI,
    SOF_LINK_NONE,
    SOF_LINK_NONE,
    SOF_LINK_NONE,
);

const CML_LINK_ORDER: c_ulong = SOF_LINK_ORDER(
    SOF_LINK_AMP,
    SOF_LINK_CODEC,
    SOF_LINK_DMIC01,
    SOF_LINK_IDISP_HDMI,
    SOF_LINK_DMIC16K,
    SOF_LINK_NONE,
    SOF_LINK_NONE,
);

const JSL_LINK_ORDER: c_ulong = SOF_LINK_ORDER(
    SOF_LINK_AMP,
    SOF_LINK_CODEC,
    SOF_LINK_DMIC01,
    SOF_LINK_IDISP_HDMI,
    SOF_LINK_DMIC16K,
    SOF_LINK_NONE,
    SOF_LINK_NONE,
);

unsafe extern "C" fn audio_probe(pdev: *mut platform_device) -> c_int {
    let mach: *mut snd_soc_acpi_mach = (*(*pdev).dev).platform_data as *mut snd_soc_acpi_mach;
    let ctx: *mut sof_card_private;
    let card_name: *mut c_char;
    let mut board_quirk: c_ulong = 0;
    let mut ret: c_int;

    if !(*pdev).id_entry.is_null() && (*(*pdev).id_entry).driver_data != 0 {
        board_quirk = (*(*pdev).id_entry).driver_data as c_ulong;
    }

    dev_dbg(&mut (*pdev).dev, c_str!("board_quirk = %lx\n"), board_quirk);

    /* initialize ctx with board quirk */
    ctx = sof_intel_board_get_ctx(&mut (*pdev).dev, board_quirk);
    if ctx.is_null() {
        return -ENOMEM;
    }

    if (*mach).mach_params.codec_mask & IDISP_CODEC_MASK != 0 {
        (*ctx).hdmi.idisp_codec = true;
    }

    if board_quirk & SOF_DA7219_GLK_BOARD != 0 {
        /* dmic16k not support */
        (*ctx).dmic_be_num = 1;

        /* overwrite the DAI link order for GLK boards */
        (*ctx).link_order_overwrite = GLK_LINK_ORDER;

        /* backward-compatible with existing devices */
        match (*ctx).amp_type {
            CODEC_MAX98357A => {
                card_name = devm_kstrdup(&mut (*pdev).dev, c_str!("glkda7219max"), GFP_KERNEL);
                if card_name.is_null() {
                    return -ENOMEM;
                }

                card_da7219.name = card_name;
            }
            _ => {}
        }
    } else if board_quirk & SOF_DA7219_CML_BOARD != 0 {
        /* overwrite the DAI link order for CML boards */
        (*ctx).link_order_overwrite = CML_LINK_ORDER;

        /* backward-compatible with existing devices */
        match (*ctx).amp_type {
            CODEC_MAX98357A => {
                card_name = devm_kstrdup(&mut (*pdev).dev, c_str!("cmlda7219max"), GFP_KERNEL);
                if card_name.is_null() {
                    return -ENOMEM;
                }

                card_da7219.name = card_name;
            }
            CODEC_MAX98390 => {
                card_name =
                    devm_kstrdup(&mut (*pdev).dev, c_str!("cml_max98390_da7219"), GFP_KERNEL);
                if card_name.is_null() {
                    return -ENOMEM;
                }

                card_da7219.name = card_name;
            }
            _ => {}
        }
    } else if board_quirk & SOF_DA7219_JSL_BOARD != 0 {
        /* overwrite the DAI link order for JSL boards */
        (*ctx).link_order_overwrite = JSL_LINK_ORDER;

        /* backward-compatible with existing devices */
        match (*ctx).amp_type {
            CODEC_MAX98360A => {
                card_name = devm_kstrdup(&mut (*pdev).dev, c_str!("da7219max98360a"), GFP_KERNEL);
                if card_name.is_null() {
                    return -ENOMEM;
                }

                card_da7219.name = card_name;
            }
            CODEC_MAX98373 => {
                card_name = devm_kstrdup(&mut (*pdev).dev, c_str!("da7219max"), GFP_KERNEL);
                if card_name.is_null() {
                    return -ENOMEM;
                }

                card_da7219.name = card_name;
            }
            _ => {}
        }
    }

    if board_quirk & SOF_DA7219_MCLK_EN != 0 {
        (*ctx).da7219.mclk_en = true;
    }

    /* update dai_link */
    ret = sof_card_dai_links_create(&mut (*pdev).dev, &mut card_da7219, ctx);
    if ret != 0 {
        return ret;
    }

    /* update codec_conf */
    match (*ctx).amp_type {
        CODEC_MAX98373 => {
            max_98373_set_codec_conf(&mut card_da7219);
        }
        CODEC_MAX98390 => {
            max_98390_set_codec_conf(&mut (*pdev).dev, &mut card_da7219);
        }
        CODEC_MAX98357A | CODEC_MAX98360A | CODEC_NONE => {
            /* no codec conf required */
        }
        _ => {
            dev_err(&mut (*pdev).dev, c_str!("invalid amp type %d\n"), (*ctx).amp_type);
            return -EINVAL;
        }
    }

    card_da7219.dev = &mut (*pdev).dev;

    ret = snd_soc_fixup_dai_links_platform_name(&mut card_da7219, (*mach).mach_params.platform);
    if ret != 0 {
        return ret;
    }

    snd_soc_card_set_drvdata(&mut card_da7219, ctx as *mut c_void);

    devm_snd_soc_register_card(&mut (*pdev).dev, &mut card_da7219)
}

static board_ids: [platform_device_id; 7] = [
    platform_device_id {
        name: c_str!("glk_da7219_def"),
        driver_data: (SOF_DA7219_GLK_BOARD | SOF_SSP_PORT_CODEC(2) | SOF_SSP_PORT_AMP(1))
            as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    platform_device_id {
        name: c_str!("cml_da7219_def"),
        driver_data: (SOF_DA7219_CML_BOARD | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(1))
            as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    platform_device_id {
        name: c_str!("jsl_da7219_def"),
        driver_data: (SOF_DA7219_JSL_BOARD | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(1))
            as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    platform_device_id {
        name: c_str!("adl_da7219_def"),
        driver_data: (SOF_DA7219_MCLK_EN
            | SOF_SSP_PORT_CODEC(0)
            | SOF_SSP_PORT_AMP(1)
            | SOF_NUM_IDISP_HDMI(4)
            | SOF_SSP_PORT_BT_OFFLOAD(2)
            | SOF_BT_OFFLOAD_PRESENT) as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    platform_device_id {
        name: c_str!("rpl_da7219_def"),
        driver_data: (SOF_DA7219_MCLK_EN
            | SOF_SSP_PORT_CODEC(0)
            | SOF_SSP_PORT_AMP(1)
            | SOF_NUM_IDISP_HDMI(4)
            | SOF_SSP_PORT_BT_OFFLOAD(2)
            | SOF_BT_OFFLOAD_PRESENT) as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    platform_device_id {
        name: c_str!("mtl_da7219_def"),
        driver_data: (SOF_DA7219_MCLK_EN
            | SOF_SSP_PORT_CODEC(2)
            | SOF_SSP_PORT_AMP(0)
            | SOF_SSP_PORT_BT_OFFLOAD(1)
            | SOF_BT_OFFLOAD_PRESENT) as kernel_ulong_t,
        ..unsafe { core::mem::zeroed() }
    },
    platform_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];

module_device_table!(platform, board_ids);

static mut audio: platform_driver = platform_driver {
    probe: Some(audio_probe),
    driver: device_driver {
        name: c_str!("sof_da7219"),
        pm: &snd_soc_pm_ops,
        ..unsafe { core::mem::zeroed() }
    },
    id_table: board_ids.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(audio);

/* Module information */
module_description!("ASoC Intel(R) SOF Machine driver for Dialog codec");
module_author!("Yong Zhi <yong.zhi@intel.com>");
module_author!("Brent Lu <brent.lu@intel.com>");
module_license!("GPL v2");
module_import_ns!("SND_SOC_INTEL_SOF_BOARD_HELPERS");
module_import_ns!("SND_SOC_INTEL_SOF_MAXIM_COMMON");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
