// SPDX-License-Identifier: GPL-2.0+
/*
 * Machine driver for AMD Stoney platform using ES8336 Codec
 *
 * Copyright 2022 Advanced Micro Devices, Inc.
 */

// C dependencies translated from:
// <sound/core.h>, <sound/soc.h>, <sound/pcm.h>, <sound/pcm_params.h>,
// <sound/soc-dapm.h>, <sound/jack.h>, <linux/device.h>, <linux/dmi.h>,
// <linux/gpio/consumer.h>, <linux/gpio/machine.h>, <linux/i2c.h>,
// <linux/input.h>, <linux/module.h>, <linux/platform_device.h>,
// <linux/acpi.h>, and "acp.h".

const DUAL_CHANNEL: c_uint = 2;
const DRV_NAME: *const c_char = c"acp2x_mach".as_ptr();
const ST_JADEITE: c_ulong = 1;
const ES8336_PLL_FREQ: c_uint = 48000 * 256;

static mut acp2x_machine_id: c_ulong = 0;
static mut st_jack: snd_soc_jack = unsafe { core::mem::zeroed() };
static mut codec_dev: *mut device = core::ptr::null_mut();
static mut gpio_pa: *mut gpio_desc = core::ptr::null_mut();

unsafe extern "C" fn sof_es8316_speaker_power_event(
    w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    if SND_SOC_DAPM_EVENT_ON(event) {
        gpiod_set_value_cansleep(gpio_pa, true);
    } else {
        gpiod_set_value_cansleep(gpio_pa, false);
    }

    0
}

static mut st_es8316_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn st_es8336_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let ret: c_int;
    let card: *mut snd_soc_card;
    let codec: *mut snd_soc_component;

    codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    card = (*rtd).card;

    ret = snd_soc_card_jack_new_pins(
        card,
        c"Headset".as_ptr(),
        SND_JACK_HEADSET | SND_JACK_BTN_0,
        &raw mut st_jack,
        st_es8316_jack_pins.as_mut_ptr(),
        ARRAY_SIZE(&st_es8316_jack_pins),
    );
    if ret != 0 {
        dev_err((*card).dev, c"HP jack creation failed %d\n".as_ptr(), ret);
        return ret;
    }
    snd_jack_set_key((*st_jack.jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    ret = snd_soc_component_set_jack(codec, &raw mut st_jack, core::ptr::null_mut());
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            c"Headset Jack call-back failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }
    0
}

static st_channels: [c_uint; 1] = [DUAL_CHANNEL];

static st_rates: [c_uint; 1] = [48000];

static st_constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: ARRAY_SIZE(&st_rates),
    list: st_rates.as_ptr(),
    mask: 0,
};

static st_constraints_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: ARRAY_SIZE(&st_channels),
    list: st_channels.as_ptr(),
    mask: 0,
};

unsafe extern "C" fn st_es8336_codec_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime;
    let rtd: *mut snd_soc_pcm_runtime;
    let card: *mut snd_soc_card;
    let machine: *mut acp_platform_info;
    let codec_dai: *mut snd_soc_dai;
    let ret: c_int;

    runtime = (*substream).runtime;
    rtd = snd_soc_substream_to_rtd(substream);
    card = (*rtd).card;
    machine = snd_soc_card_get_drvdata(card) as *mut acp_platform_info;
    codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    ret = snd_soc_dai_set_sysclk(codec_dai, 0, ES8336_PLL_FREQ, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*rtd).dev, c"can't set codec sysclk: %d\n".as_ptr(), ret);
        return ret;
    }
    (*runtime).hw.channels_max = DUAL_CHANNEL;
    snd_pcm_hw_constraint_list(
        runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        &st_constraints_channels,
    );
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &st_constraints_rates);

    (*machine).play_i2s_instance = I2S_MICSP_INSTANCE;
    (*machine).cap_i2s_instance = I2S_MICSP_INSTANCE;
    (*machine).capture_channel = CAP_CHANNEL0;
    0
}

static st_es8336_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(st_es8336_codec_startup),
};

SND_SOC_DAILINK_DEF!(designware1, DAILINK_COMP_ARRAY!(COMP_CPU!(c"designware-i2s.1")));
SND_SOC_DAILINK_DEF!(
    codec,
    DAILINK_COMP_ARRAY!(COMP_CODEC!(c"i2c-ESSX8336:00", c"ES8316 HiFi"))
);
SND_SOC_DAILINK_DEF!(
    platform,
    DAILINK_COMP_ARRAY!(COMP_PLATFORM!(c"acp_audio_dma.0"))
);

static mut st_dai_es8336: [snd_soc_dai_link; 1] = [snd_soc_dai_link {
    name: c"amdes8336".as_ptr(),
    stream_name: c"ES8336 HiFi Play".as_ptr(),
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
    trigger_stop: SND_SOC_TRIGGER_ORDER_LDC,
    init: Some(st_es8336_init),
    ops: &st_es8336_ops,
    SND_SOC_DAILINK_REG!(designware1, codec, platform)
}];

static st_widgets: [snd_soc_dapm_widget; 5] = [
    SND_SOC_DAPM_SPK!(c"Speaker", None),
    SND_SOC_DAPM_HP!(c"Headphone", None),
    SND_SOC_DAPM_MIC!(c"Headset Mic", None),
    SND_SOC_DAPM_MIC!(c"Internal Mic", None),
    SND_SOC_DAPM_SUPPLY!(
        c"Speaker Power",
        SND_SOC_NOPM,
        0,
        0,
        Some(sof_es8316_speaker_power_event),
        SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU
    ),
];

static st_audio_route: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: core::ptr::null(),
        source: c"HPOL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: core::ptr::null(),
        source: c"HPOR".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: core::ptr::null(),
        source: c"HPOL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: core::ptr::null(),
        source: c"HPOR".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"MIC1".as_ptr(),
        control: core::ptr::null(),
        source: c"Headset Mic".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"MIC2".as_ptr(),
        control: core::ptr::null(),
        source: c"Internal Mic".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: core::ptr::null(),
        source: c"Speaker Power".as_ptr(),
    },
];

static st_mc_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_PIN_SWITCH!(c"Speaker"),
    SOC_DAPM_PIN_SWITCH!(c"Headphone"),
    SOC_DAPM_PIN_SWITCH!(c"Headset Mic"),
    SOC_DAPM_PIN_SWITCH!(c"Internal Mic"),
];

static pa_enable_gpio: acpi_gpio_params = acpi_gpio_params {
    crs_entry_index: 0,
    line_index: 0,
    active_low: false,
};

static acpi_es8336_gpios: [acpi_gpio_mapping; 2] = [
    acpi_gpio_mapping {
        name: c"pa-enable-gpios".as_ptr(),
        data: &pa_enable_gpio,
        size: 1,
    },
    acpi_gpio_mapping {
        name: core::ptr::null(),
        data: core::ptr::null(),
        size: 0,
    },
];

unsafe extern "C" fn st_es8336_late_probe(card: *mut snd_soc_card) -> c_int {
    let adev: *mut acpi_device;
    let mut ret: c_int;

    adev = acpi_dev_get_first_match_dev(c"ESSX8336".as_ptr(), core::ptr::null(), -1);
    if adev.is_null() {
        return -ENODEV;
    }

    codec_dev = acpi_get_first_physical_node(adev);
    acpi_dev_put(adev);
    if codec_dev.is_null() {
        dev_err((*card).dev, c"can not find codec dev\n".as_ptr());
        return -ENODEV;
    }

    ret = devm_acpi_dev_add_driver_gpios(codec_dev, acpi_es8336_gpios.as_ptr());
    if ret != 0 {
        dev_warn((*card).dev, c"Failed to add driver gpios\n".as_ptr());
    }

    gpio_pa = gpiod_get_optional(codec_dev, c"pa-enable".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR(gpio_pa) {
        ret = dev_err_probe(
            (*card).dev,
            PTR_ERR(gpio_pa),
            c"could not get pa-enable GPIO\n".as_ptr(),
        );
        put_device(codec_dev);
        return ret;
    }
    0
}

static mut st_card: snd_soc_card = snd_soc_card {
    name: c"acpes8336".as_ptr(),
    owner: THIS_MODULE,
    dai_link: unsafe { st_dai_es8336.as_mut_ptr() },
    num_links: ARRAY_SIZE(unsafe { &st_dai_es8336 }),
    dapm_widgets: st_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&st_widgets),
    dapm_routes: st_audio_route.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&st_audio_route),
    controls: st_mc_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&st_mc_controls),
    late_probe: Some(st_es8336_late_probe),
};

unsafe extern "C" fn st_es8336_quirk_cb(id: *const dmi_system_id) -> c_int {
    acp2x_machine_id = ST_JADEITE;
    1
}

static st_es8336_quirk_table: [dmi_system_id; 4] = [
    dmi_system_id {
        callback: Some(st_es8336_quirk_cb),
        matches: [
            DMI_EXACT_MATCH!(DMI_BOARD_VENDOR, c"AMD"),
            DMI_EXACT_MATCH!(DMI_PRODUCT_NAME, c"Jadeite"),
        ],
    },
    dmi_system_id {
        callback: Some(st_es8336_quirk_cb),
        matches: [
            DMI_EXACT_MATCH!(DMI_BOARD_VENDOR, c"IP3 Technology CO.,Ltd."),
            DMI_EXACT_MATCH!(DMI_PRODUCT_NAME, c"ASN1D"),
        ],
    },
    dmi_system_id {
        callback: Some(st_es8336_quirk_cb),
        matches: [
            DMI_EXACT_MATCH!(DMI_BOARD_VENDOR, c"Standard"),
            DMI_EXACT_MATCH!(DMI_PRODUCT_NAME, c"ASN10"),
        ],
    },
    dmi_system_id::default(),
];

unsafe extern "C" fn st_es8336_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let card: *mut snd_soc_card;
    let machine: *mut acp_platform_info;

    machine = devm_kzalloc(
        &raw mut (*pdev).dev,
        core::mem::size_of::<acp_platform_info>(),
        GFP_KERNEL,
    ) as *mut acp_platform_info;
    if machine.is_null() {
        return -ENOMEM;
    }

    dmi_check_system(st_es8336_quirk_table.as_ptr());
    match acp2x_machine_id {
        ST_JADEITE => {
            card = &raw mut st_card;
            st_card.dev = &raw mut (*pdev).dev;
        }
        _ => {
            return -ENODEV;
        }
    }

    platform_set_drvdata(pdev, card as *mut c_void);
    snd_soc_card_set_drvdata(card, machine as *mut c_void);
    ret = devm_snd_soc_register_card(&raw mut (*pdev).dev, &raw mut st_card);
    if ret != 0 {
        return dev_err_probe(
            &raw mut (*pdev).dev,
            ret,
            c"devm_snd_soc_register_card(%s) failed\n".as_ptr(),
            (*card).name,
        );
    }
    0
}

// CONFIG_ACPI conditional from the C source.
#[cfg(CONFIG_ACPI)]
static st_audio_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: *b"AMDI8336\0",
        driver_data: 0,
    },
    acpi_device_id::default(),
];
#[cfg(CONFIG_ACPI)]
MODULE_DEVICE_TABLE!(acpi, st_audio_acpi_match);

static mut st_mach_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"st-es8316".as_ptr(),
        acpi_match_table: ACPI_PTR!(st_audio_acpi_match),
        pm: &snd_soc_pm_ops,
    },
    probe: Some(st_es8336_probe),
};

module_platform_driver!(st_mach_driver);

MODULE_AUTHOR!(c"Vijendar.Mukunda@amd.com");
MODULE_DESCRIPTION!(c"st-es8316 audio support");
MODULE_LICENSE!(c"GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
