// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2021 Intel Corporation.
// Copyright(c) 2021 Nuvoton Corporation.

/*
 * Intel SOF Machine Driver with Nuvoton headphone codec NAU8825
 * and speaker codec RT1019P MAX98360a or MAX98373
 */

// C dependencies translated as external Rust dependency references:
// linux/i2c.h, linux/input.h, linux/module.h, linux/platform_device.h,
// linux/dmi.h, sound/core.h, sound/jack.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h, sound/sof.h, sound/soc-acpi.h,
// ../../codecs/nau8825.h, ../common/soc-intel-quirks.h,
// sof_board_helpers.h, sof_realtek_common.h, sof_maxim_common.h,
// sof_nuvoton_common.h
use crate::*;

static mut sof_nau8825_quirk: c_ulong = SOF_SSP_PORT_CODEC(0) as c_ulong;

static mut jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone Jack".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn sof_nau8825_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let ctx: *mut sof_card_private = snd_soc_card_get_drvdata((*rtd).card) as *mut sof_card_private;
    let component: *mut snd_soc_component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let jack: *mut snd_soc_jack = &mut (*ctx).headset_jack;
    let mut ret: c_int;

    /*
     * Headset buttons map to the google Reference headset.
     * These can be configured by userspace.
     */
    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"Headset Jack".as_ptr(),
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        jack,
        jack_pins.as_mut_ptr(),
        jack_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            c"Headset Jack creation failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

    ret = snd_soc_component_set_jack(component, jack, core::ptr::null_mut());
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            c"Headset Jack call-back failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret
}

unsafe extern "C" fn sof_nau8825_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
    let component: *mut snd_soc_component = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    snd_soc_component_set_jack(component, core::ptr::null_mut(), core::ptr::null_mut());
}

unsafe extern "C" fn sof_nau8825_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut clk_freq: c_int;
    let mut ret: c_int;

    clk_freq = sof_dai_get_bclk(rtd); /* BCLK freq */

    if clk_freq <= 0 {
        dev_err((*rtd).dev, c"get bclk freq failed: %d\n".as_ptr(), clk_freq);
        return -EINVAL;
    }

    /* Configure clock for codec */
    ret = snd_soc_dai_set_sysclk(codec_dai, NAU8825_CLK_FLL_BLK, 0, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*codec_dai).dev, c"can't set BCLK clock %d\n".as_ptr(), ret);
        return ret;
    }

    /* Configure pll for codec */
    ret = snd_soc_dai_set_pll(
        codec_dai,
        0,
        0,
        clk_freq,
        params_rate(params).wrapping_mul(256),
    );
    if ret < 0 {
        dev_err((*codec_dai).dev, c"can't set BCLK: %d\n".as_ptr(), ret);
        return ret;
    }

    ret
}

static sof_nau8825_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(sof_nau8825_hw_params),
};

unsafe extern "C" fn sof_card_late_probe(card: *mut snd_soc_card) -> c_int {
    let ctx: *mut sof_card_private = snd_soc_card_get_drvdata(card) as *mut sof_card_private;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let mut err: c_int;

    if (*ctx).amp_type == CODEC_MAX98373 {
        /* Disable Left and Right Spk pin after boot */
        snd_soc_dapm_disable_pin(dapm, c"Left Spk".as_ptr());
        snd_soc_dapm_disable_pin(dapm, c"Right Spk".as_ptr());
        err = snd_soc_dapm_sync(dapm);
        if err < 0 {
            return err;
        }
    }

    sof_intel_board_card_late_probe(card)
}

static sof_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_PIN_SWITCH(c"Headphone Jack".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Headset Mic".as_ptr()),
];

static sof_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_HP(c"Headphone Jack".as_ptr(), core::ptr::null_mut()),
    SND_SOC_DAPM_MIC(c"Headset Mic".as_ptr(), core::ptr::null_mut()),
];

static sof_map: [snd_soc_dapm_route; 3] = [
    /* HP jack connectors - unknown if we have jack detection */
    snd_soc_dapm_route {
        sink: c"Headphone Jack".as_ptr(),
        control: core::ptr::null(),
        source: c"HPOL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone Jack".as_ptr(),
        control: core::ptr::null(),
        source: c"HPOR".as_ptr(),
    },

    /* other jacks */
    snd_soc_dapm_route {
        sink: c"MIC".as_ptr(),
        control: core::ptr::null(),
        source: c"Headset Mic".as_ptr(),
    },
];

/* sof audio machine driver for nau8825 codec */
static mut sof_audio_card_nau8825: snd_soc_card = snd_soc_card {
    name: c"nau8825".as_ptr(), /* the sof- prefix is added by the core */
    owner: THIS_MODULE,
    controls: sof_controls.as_ptr(),
    num_controls: sof_controls.len() as c_int,
    dapm_widgets: sof_widgets.as_ptr(),
    num_dapm_widgets: sof_widgets.len() as c_int,
    dapm_routes: sof_map.as_ptr(),
    num_dapm_routes: sof_map.len() as c_int,
    fully_routed: true,
    late_probe: Some(sof_card_late_probe),
    ..unsafe { core::mem::zeroed() }
};

static mut nau8825_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"i2c-10508825:00".as_ptr(),
    dai_name: c"nau8825-hifi".as_ptr(),
    ..unsafe { core::mem::zeroed() }
}];

unsafe extern "C" fn sof_card_dai_links_create(
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
        dev_err(dev, c"codec link not available".as_ptr());
        return -EINVAL;
    }

    /* codec-specific fields for headphone codec */
    (*(*ctx).codec_link).codecs = nau8825_component.as_mut_ptr();
    (*(*ctx).codec_link).num_codecs = nau8825_component.len() as c_int;
    (*(*ctx).codec_link).init = Some(sof_nau8825_codec_init);
    (*(*ctx).codec_link).exit = Some(sof_nau8825_codec_exit);
    (*(*ctx).codec_link).ops = &sof_nau8825_ops;

    if (*ctx).amp_type == CODEC_NONE {
        return 0;
    }

    if (*ctx).amp_link.is_null() {
        dev_err(dev, c"amp link not available".as_ptr());
        return -EINVAL;
    }

    /* codec-specific fields for speaker amplifier */
    match (*ctx).amp_type {
        CODEC_MAX98360A => {
            max_98360a_dai_link((*ctx).amp_link);
        }
        CODEC_MAX98373 => {
            max_98373_dai_link(dev, (*ctx).amp_link);
        }
        CODEC_NAU8318 => {
            nau8318_set_dai_link((*ctx).amp_link);
        }
        CODEC_RT1015P => {
            sof_rt1015p_dai_link((*ctx).amp_link);
        }
        CODEC_RT1019P => {
            sof_rt1019p_dai_link((*ctx).amp_link);
        }
        _ => {
            dev_err(dev, c"invalid amp type %d\n".as_ptr(), (*ctx).amp_type);
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn sof_audio_probe(pdev: *mut platform_device) -> c_int {
    let mach: *mut snd_soc_acpi_mach = (*(*pdev).dev).platform_data as *mut snd_soc_acpi_mach;
    let ctx: *mut sof_card_private;
    let mut ret: c_int;

    if !(*pdev).id_entry.is_null() && (*(*pdev).id_entry).driver_data != 0 {
        sof_nau8825_quirk = (*(*pdev).id_entry).driver_data as c_ulong;
    }

    dev_dbg(
        &mut (*pdev).dev,
        c"sof_nau8825_quirk = %lx\n".as_ptr(),
        sof_nau8825_quirk,
    );

    /* initialize ctx with board quirk */
    ctx = sof_intel_board_get_ctx(&mut (*pdev).dev, sof_nau8825_quirk);
    if ctx.is_null() {
        return -ENOMEM;
    }

    if (*mach).mach_params.codec_mask & IDISP_CODEC_MASK != 0 {
        (*ctx).hdmi.idisp_codec = true;
    }

    /* update dai_link */
    ret = sof_card_dai_links_create(&mut (*pdev).dev, &mut sof_audio_card_nau8825, ctx);
    if ret != 0 {
        return ret;
    }

    /* update codec_conf */
    match (*ctx).amp_type {
        CODEC_MAX98373 => {
            max_98373_set_codec_conf(&mut sof_audio_card_nau8825);
        }
        CODEC_RT1015P => {
            sof_rt1015p_codec_conf(&mut sof_audio_card_nau8825);
        }
        CODEC_MAX98360A | CODEC_NAU8318 | CODEC_RT1019P | CODEC_NONE => {
            /* no codec conf required */
        }
        _ => {
            dev_err(
                &mut (*pdev).dev,
                c"invalid amp type %d\n".as_ptr(),
                (*ctx).amp_type,
            );
            return -EINVAL;
        }
    }

    sof_audio_card_nau8825.dev = &mut (*pdev).dev;

    /* set platform name for each dailink */
    ret = snd_soc_fixup_dai_links_platform_name(
        &mut sof_audio_card_nau8825,
        (*mach).mach_params.platform,
    );
    if ret != 0 {
        return ret;
    }

    snd_soc_card_set_drvdata(&mut sof_audio_card_nau8825, ctx as *mut c_void);

    devm_snd_soc_register_card(&mut (*pdev).dev, &mut sof_audio_card_nau8825)
}

static board_ids: [platform_device_id; 5] = [
    platform_device_id {
        name: *b"adl_rt1019p_8825\0",
        driver_data: (SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(2) | SOF_NUM_IDISP_HDMI(4))
            as kernel_ulong_t,
    },
    platform_device_id {
        name: *b"adl_nau8825_def\0",
        driver_data: (SOF_SSP_PORT_CODEC(0)
            | SOF_SSP_PORT_AMP(1)
            | SOF_NUM_IDISP_HDMI(4)
            | SOF_SSP_PORT_BT_OFFLOAD(2)
            | SOF_BT_OFFLOAD_PRESENT) as kernel_ulong_t,
    },
    platform_device_id {
        name: *b"rpl_nau8825_def\0",
        driver_data: (SOF_SSP_PORT_CODEC(0)
            | SOF_SSP_PORT_AMP(1)
            | SOF_NUM_IDISP_HDMI(4)
            | SOF_SSP_PORT_BT_OFFLOAD(2)
            | SOF_BT_OFFLOAD_PRESENT) as kernel_ulong_t,
    },
    platform_device_id {
        name: *b"mtl_nau8825_def\0",
        driver_data: (SOF_SSP_PORT_CODEC(2)
            | SOF_SSP_PORT_AMP(0)
            | SOF_SSP_PORT_BT_OFFLOAD(1)
            | SOF_BT_OFFLOAD_PRESENT) as kernel_ulong_t,
    },
    platform_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
// MODULE_DEVICE_TABLE(platform, board_ids);

static mut sof_audio: platform_driver = platform_driver {
    probe: Some(sof_audio_probe),
    driver: device_driver {
        name: c"sof_nau8825".as_ptr(),
        pm: &snd_soc_pm_ops,
        ..unsafe { core::mem::zeroed() }
    },
    id_table: board_ids.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};
// module_platform_driver(sof_audio)

/* Module information */
// MODULE_DESCRIPTION("SOF Audio Machine driver for NAU8825");
// MODULE_AUTHOR("David Lin <ctlin0@nuvoton.com>");
// MODULE_AUTHOR("Mac Chiang <mac.chiang@intel.com>");
// MODULE_AUTHOR("Brent Lu <brent.lu@intel.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_BOARD_HELPERS");
// MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_MAXIM_COMMON");
// MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_NUVOTON_COMMON");
// MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_REALTEK_COMMON");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
