// SPDX-License-Identifier: GPL-2.0
//
// Loongson ASoC Audio Machine driver
//
// Copyright (C) 2023-2026 Loongson Technology Corporation Limited
// Author: Yingkun Meng <mengyingkun@loongson.cn>
//         Binbin Zhou <zhoubinbin@loongson.cn>
//

static mut codec_name: [::core::ffi::c_char; SND_ACPI_I2C_ID_LEN] =
    [0; SND_ACPI_I2C_ID_LEN];

#[repr(C)]
struct loongson_card_data {
    snd_card: snd_soc_card,
    mclk_fs: ::core::ffi::c_uint,
    gpiod_hp_det: *mut gpio_desc,
    gpiod_hp_ctl: *mut gpio_desc,
    gpiod_spkr_en: *mut gpio_desc,
    cfg: *const loongson_card_config,
}

#[repr(C)]
struct loongson_card_config {
    fmt: ::core::ffi::c_uint,
    add_hp_jack: bool,
    add_dapm_widgets: bool,
    add_dapm_routes: bool,
}

static ls2k1000_card_config: loongson_card_config = loongson_card_config {
    fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_IB_NF | SND_SOC_DAIFMT_CBC_CFC,
    add_hp_jack: false,
    add_dapm_widgets: false,
    add_dapm_routes: false,
};

static ls2k0300_forever_pi_card_config: loongson_card_config = loongson_card_config {
    fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
    add_hp_jack: false,
    add_dapm_widgets: false,
    add_dapm_routes: false,
};

static ls2k0300_dl2k0300b_card_config: loongson_card_config = loongson_card_config {
    fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
    add_hp_jack: true,
    add_dapm_widgets: true,
    add_dapm_routes: true,
};

unsafe extern "C" fn loongson_asoc_machine_event(
    w: *mut snd_soc_dapm_widget,
    k: *mut snd_kcontrol,
    event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let card: *mut snd_soc_card = snd_soc_dapm_to_card((*w).dapm);
    let priv_: *mut loongson_card_data = snd_soc_card_get_drvdata(card) as *mut loongson_card_data;

    if snd_soc_dapm_widget_name_cmp(w, c"Speaker".as_ptr()) == 0 {
        gpiod_set_value_cansleep((*priv_).gpiod_spkr_en, SND_SOC_DAPM_EVENT_ON(event));
    }

    if snd_soc_dapm_widget_name_cmp(w, c"Headphone".as_ptr()) == 0 {
        gpiod_set_value_cansleep((*priv_).gpiod_hp_ctl, SND_SOC_DAPM_EVENT_ON(event));
    }

    0
}

static loongson_asoc_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_HP(c"Headphone".as_ptr(), Some(loongson_asoc_machine_event)),
    SND_SOC_DAPM_SPK(c"Speaker".as_ptr(), Some(loongson_asoc_machine_event)),
];

/* Headphones Jack */

static mut loongson_asoc_hp_jack: snd_soc_jack = unsafe { ::core::mem::zeroed() };

static mut loongson_asoc_hp_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
        ..unsafe { ::core::mem::zeroed() }
    },
    snd_soc_jack_pin {
        pin: c"Speaker".as_ptr(),
        mask: SND_JACK_HEADPHONE,
        invert: 1,
        ..unsafe { ::core::mem::zeroed() }
    },
];

static mut loongson_asoc_hp_jack_gpio: snd_soc_jack_gpio = snd_soc_jack_gpio {
    name: c"Headphones detection".as_ptr(),
    report: SND_JACK_HEADPHONE,
    debounce_time: 150,
    ..unsafe { ::core::mem::zeroed() }
};

unsafe extern "C" fn loongson_asoc_machine_init(
    rtd: *mut snd_soc_pcm_runtime,
) -> ::core::ffi::c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let ls_priv: *mut loongson_card_data =
        snd_soc_card_get_drvdata(card) as *mut loongson_card_data;
    let mut ret: ::core::ffi::c_int = 0;

    if !(*(*ls_priv).cfg).add_hp_jack || (*ls_priv).gpiod_hp_det.is_null() {
        return 0;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        c"Headphones Jack".as_ptr(),
        SND_JACK_HEADPHONE,
        &raw mut loongson_asoc_hp_jack,
        loongson_asoc_hp_jack_pins.as_mut_ptr(),
        ARRAY_SIZE(&loongson_asoc_hp_jack_pins),
    );
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            c"Headphones Jack creation failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    loongson_asoc_hp_jack_gpio.desc = (*ls_priv).gpiod_hp_det;

    ret = snd_soc_jack_add_gpios(
        &raw mut loongson_asoc_hp_jack,
        1,
        &raw mut loongson_asoc_hp_jack_gpio,
    );
    if ret != 0 {
        dev_err((*rtd).dev, c"Headphone GPIO not added: %d\n".as_ptr(), ret);
    }

    ret
}

unsafe extern "C" fn loongson_card_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> ::core::ffi::c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let ls_card: *mut loongson_card_data =
        snd_soc_card_get_drvdata((*rtd).card) as *mut loongson_card_data;
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret: ::core::ffi::c_int;
    let mclk: ::core::ffi::c_int;

    if (*ls_card).mclk_fs == 0 {
        return 0;
    }

    mclk = ((*ls_card).mclk_fs * params_rate(params)) as ::core::ffi::c_int;
    ret = snd_soc_dai_set_sysclk(cpu_dai, 0, mclk, SND_SOC_CLOCK_OUT);
    if ret < 0 {
        dev_err((*codec_dai).dev, c"cpu_dai clock not set\n".as_ptr());
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, 0, mclk, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*codec_dai).dev, c"codec_dai clock not set\n".as_ptr());
        return ret;
    }

    snd_soc_runtime_set_dai_fmt(rtd, (*(*ls_card).cfg).fmt)
}

static loongson_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(loongson_card_hw_params),
    ..unsafe { ::core::mem::zeroed() }
};

SND_SOC_DAILINK_DEFS!(
    analog,
    DAILINK_COMP_ARRAY!(COMP_CPU!(c"loongson-i2s".as_ptr())),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

static mut loongson_dai_links: [snd_soc_dai_link; 1] = [snd_soc_dai_link {
    name: c"Loongson Audio Port".as_ptr(),
    stream_name: c"Loongson Audio".as_ptr(),
    init: Some(loongson_asoc_machine_init),
    SND_SOC_DAILINK_REG!(analog),
    ops: &loongson_ops,
    ..unsafe { ::core::mem::zeroed() }
}];

unsafe extern "C" fn loongson_card_acpi_find_device(
    card: *mut snd_soc_card,
    name: *const ::core::ffi::c_char,
) -> *mut acpi_device {
    let fwnode: *mut fwnode_handle = (*(*card).dev).fwnode;
    let mut args: fwnode_reference_args = ::core::mem::zeroed();
    let status: ::core::ffi::c_int;

    memset(
        &mut args as *mut _ as *mut ::core::ffi::c_void,
        0,
        ::core::mem::size_of_val(&args),
    );
    status = acpi_node_get_property_reference(fwnode, name, 0, &mut args);
    if status != 0 || !is_acpi_device_node(args.fwnode) {
        dev_err((*card).dev, c"No matching phy in ACPI table\n".as_ptr());
        return ::core::ptr::null_mut();
    }

    to_acpi_device_node(args.fwnode)
}

unsafe extern "C" fn loongson_card_parse_acpi(
    data: *mut loongson_card_data,
) -> ::core::ffi::c_int {
    let card: *mut snd_soc_card = &mut (*data).snd_card;
    let mut codec_dai_name: *const ::core::ffi::c_char = ::core::ptr::null();
    let mut adev: *mut acpi_device;
    let phy_dev: *mut device;
    let mut i: ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int;

    /* fixup platform name based on reference node */
    adev = loongson_card_acpi_find_device(card, c"cpu".as_ptr());
    if adev.is_null() {
        return -ENOENT;
    }

    phy_dev = acpi_get_first_physical_node(adev);
    if phy_dev.is_null() {
        return -EPROBE_DEFER;
    }

    /* fixup codec name based on reference node */
    adev = loongson_card_acpi_find_device(card, c"codec".as_ptr());
    if adev.is_null() {
        return -ENOENT;
    }
    snprintf(
        codec_name.as_mut_ptr(),
        codec_name.len(),
        c"i2c-%s".as_ptr(),
        acpi_dev_name(adev),
    );

    ret = device_property_read_string((*card).dev, c"codec-dai-name".as_ptr(), &mut codec_dai_name);
    if ret != 0 {
        return ret;
    }

    i = 0;
    while i < (*card).num_links {
        (*(*loongson_dai_links[i as usize].platforms)).name = dev_name(phy_dev);
        (*(*loongson_dai_links[i as usize].codecs)).name = codec_name.as_ptr();
        (*(*loongson_dai_links[i as usize].codecs)).dai_name = codec_dai_name;
        i += 1;
    }

    0
}

unsafe extern "C" fn loongson_card_parse_of(
    data: *mut loongson_card_data,
) -> ::core::ffi::c_int {
    let card: *mut snd_soc_card = &mut (*data).snd_card;
    let mut cpu: *mut device_node;
    let mut codec: *mut device_node;
    let dev: *mut device = (*card).dev;
    let mut ret: ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;

    (*data).gpiod_hp_det = devm_gpiod_get_optional(dev, c"hp-det".as_ptr(), GPIOD_IN);
    if IS_ERR((*data).gpiod_hp_det) {
        return PTR_ERR((*data).gpiod_hp_det) as ::core::ffi::c_int;
    }

    (*data).gpiod_hp_ctl = devm_gpiod_get_optional(dev, c"hp-ctl".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*data).gpiod_hp_ctl) {
        return PTR_ERR((*data).gpiod_hp_ctl) as ::core::ffi::c_int;
    }

    (*data).gpiod_spkr_en = devm_gpiod_get_optional(dev, c"spkr-en".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*data).gpiod_spkr_en) {
        return PTR_ERR((*data).gpiod_spkr_en) as ::core::ffi::c_int;
    }

    if (*(*data).cfg).add_dapm_routes {
        ret = snd_soc_of_parse_audio_routing(card, c"audio-routing".as_ptr());
        if ret != 0 {
            return ret;
        }
    }

    cpu = of_get_child_by_name((*dev).of_node, c"cpu".as_ptr());
    if cpu.is_null() {
        dev_err(dev, c"platform property missing or invalid\n".as_ptr());
        return -EINVAL;
    }

    codec = of_get_child_by_name((*dev).of_node, c"codec".as_ptr());
    if codec.is_null() {
        dev_err(dev, c"audio-codec property missing or invalid\n".as_ptr());
        of_node_put(cpu);
        return -EINVAL;
    }

    i = 0;
    while i < (*card).num_links {
        ret = snd_soc_of_get_dlc(
            cpu,
            ::core::ptr::null_mut(),
            loongson_dai_links[i as usize].cpus,
            0,
        );
        if ret < 0 {
            dev_err(dev, c"getting cpu dlc error (%d)\n".as_ptr(), ret);
            goto_err(cpu, codec);
            return ret;
        }
        (*loongson_dai_links[i as usize].platforms).of_node =
            (*loongson_dai_links[i as usize].cpus).of_node;

        ret = snd_soc_of_get_dlc(
            codec,
            ::core::ptr::null_mut(),
            loongson_dai_links[i as usize].codecs,
            0,
        );
        if ret < 0 {
            dev_err(dev, c"getting codec dlc error (%d)\n".as_ptr(), ret);
            goto_err(cpu, codec);
            return ret;
        }
        i += 1;
    }

    of_node_put(cpu);
    of_node_put(codec);

    0
}

unsafe fn goto_err(cpu: *mut device_node, codec: *mut device_node) {
    of_node_put(cpu);
    of_node_put(codec);
}

unsafe extern "C" fn loongson_asoc_card_probe(
    pdev: *mut platform_device,
) -> ::core::ffi::c_int {
    let mut ls_priv: *mut loongson_card_data;
    let dev: *mut device = &mut (*pdev).dev;
    let card: *mut snd_soc_card;
    let mut ret: ::core::ffi::c_int;

    ls_priv = devm_kzalloc(
        dev,
        ::core::mem::size_of::<loongson_card_data>(),
        GFP_KERNEL,
    ) as *mut loongson_card_data;
    if ls_priv.is_null() {
        return -ENOMEM;
    }

    (*ls_priv).cfg = device_get_match_data(dev) as *const loongson_card_config;
    if (*ls_priv).cfg.is_null() {
        return -EINVAL;
    }

    card = &mut (*ls_priv).snd_card;

    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).dai_link = loongson_dai_links.as_mut_ptr();
    (*card).num_links = ARRAY_SIZE(&loongson_dai_links);

    if (*(*ls_priv).cfg).add_dapm_widgets {
        (*card).dapm_widgets = loongson_asoc_dapm_widgets.as_ptr();
        (*card).num_dapm_widgets = ARRAY_SIZE(&loongson_asoc_dapm_widgets);
    }

    snd_soc_card_set_drvdata(card, ls_priv as *mut ::core::ffi::c_void);

    ret = device_property_read_string(dev, c"model".as_ptr(), &mut (*card).name);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Error parsing card name\n".as_ptr());
    }

    ret = device_property_read_u32(dev, c"mclk-fs".as_ptr(), &mut (*ls_priv).mclk_fs);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Error parsing mclk-fs\n".as_ptr());
    }

    ret = if has_acpi_companion(dev) {
        loongson_card_parse_acpi(ls_priv)
    } else {
        loongson_card_parse_of(ls_priv)
    };
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Error parsing acpi/of properties\n".as_ptr());
    }

    devm_snd_soc_register_card(dev, card)
}

static loongson_asoc_dt_ids: [of_device_id; 4] = [
    /* Loongson-2K1000/Loongson-2K2000/LS7A */
    of_device_id {
        compatible: c"loongson,ls-audio-card".as_ptr(),
        data: &ls2k1000_card_config as *const _ as *const ::core::ffi::c_void,
        ..unsafe { ::core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"loongson,ls2k0300-forever-pi-audio-card".as_ptr(),
        data: &ls2k0300_forever_pi_card_config as *const _ as *const ::core::ffi::c_void,
        ..unsafe { ::core::mem::zeroed() }
    },
    of_device_id {
        compatible: c"loongson,ls2k0300-dl2k0300b-audio-card".as_ptr(),
        data: &ls2k0300_dl2k0300b_card_config as *const _ as *const ::core::ffi::c_void,
        ..unsafe { ::core::mem::zeroed() }
    },
    of_device_id {
        /* sentinel */
        ..unsafe { ::core::mem::zeroed() }
    },
];
// MODULE_DEVICE_TABLE(of, loongson_asoc_dt_ids);

static mut loongson_audio_driver: platform_driver = platform_driver {
    probe: Some(loongson_asoc_card_probe),
    driver: device_driver {
        name: c"loongson-asoc-card".as_ptr(),
        pm: &snd_soc_pm_ops,
        of_match_table: loongson_asoc_dt_ids.as_ptr(),
        ..unsafe { ::core::mem::zeroed() }
    },
    ..unsafe { ::core::mem::zeroed() }
};
module_platform_driver!(loongson_audio_driver);

MODULE_DESCRIPTION!(c"Loongson ASoc Sound Card driver".as_ptr());
MODULE_AUTHOR!(c"Loongson Technology Corporation Limited".as_ptr());
MODULE_LICENSE!(c"GPL".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
