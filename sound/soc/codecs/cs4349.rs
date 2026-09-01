// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs4349.c  --  CS4349 ALSA Soc Audio driver
 *
 * Copyright 2015 Cirrus Logic, Inc.
 *
 * Authors: Tim Howe <Tim.Howe@cirrus.com>
 */

/* Rust translation of the C implementation source.
 * C include dependencies are expected to be supplied by the surrounding kernel
 * binding layer, including cs4349.h and Linux/ALSA SoC declarations.
 */

#[repr(C)]
pub struct cs4349_private {
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub mode: ::core::ffi::c_uint,
    pub rate: ::core::ffi::c_int,
}

static cs4349_reg_defaults: [reg_default; 7] = [
    reg_default { reg: 2, def: 0x00 }, /* r02  - Mode Control */
    reg_default { reg: 3, def: 0x09 }, /* r03  - Volume, Mixing and Inversion Control */
    reg_default { reg: 4, def: 0x81 }, /* r04  - Mute Control */
    reg_default { reg: 5, def: 0x00 }, /* r05  - Channel A Volume Control */
    reg_default { reg: 6, def: 0x00 }, /* r06  - Channel B Volume Control */
    reg_default { reg: 7, def: 0xB1 }, /* r07  - Ramp and Filter Control */
    reg_default { reg: 8, def: 0x1C }, /* r08  - Misc. Control */
];

unsafe extern "C" fn cs4349_readable_register(
    _dev: *mut device,
    reg: ::core::ffi::c_uint,
) -> bool {
    match reg {
        CS4349_CHIPID..=CS4349_MISC => true,
        _ => false,
    }
}

unsafe extern "C" fn cs4349_writeable_register(
    _dev: *mut device,
    reg: ::core::ffi::c_uint,
) -> bool {
    match reg {
        CS4349_MODE..=CS4349_MISC => true,
        _ => false,
    }
}

unsafe extern "C" fn cs4349_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    format: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let cs4349: *mut cs4349_private =
        snd_soc_component_get_drvdata(component) as *mut cs4349_private;
    let fmt: ::core::ffi::c_uint;

    fmt = format & SND_SOC_DAIFMT_FORMAT_MASK;

    match fmt {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J => {
            (*cs4349).mode = format & SND_SOC_DAIFMT_FORMAT_MASK;
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn cs4349_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let cs4349: *mut cs4349_private =
        snd_soc_component_get_drvdata(component) as *mut cs4349_private;
    let fmt: ::core::ffi::c_int;
    let ret: ::core::ffi::c_int;

    (*cs4349).rate = params_rate(params);

    match (*cs4349).mode {
        SND_SOC_DAIFMT_I2S => {
            fmt = DIF_I2S;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            fmt = DIF_LEFT_JST;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            match params_width(params) {
                16 => {
                    fmt = DIF_RGHT_JST16;
                }
                24 => {
                    fmt = DIF_RGHT_JST24;
                }
                _ => return -EINVAL,
            }
        }
        _ => return -EINVAL,
    }

    ret = snd_soc_component_update_bits(
        component,
        CS4349_MODE,
        DIF_MASK,
        MODE_FORMAT(fmt),
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn cs4349_mute(
    dai: *mut snd_soc_dai,
    mute: ::core::ffi::c_int,
    _direction: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mut reg: ::core::ffi::c_int;

    reg = 0;
    if mute != 0 {
        reg = MUTE_AB_MASK;
    }

    snd_soc_component_update_bits(component, CS4349_MUTE, MUTE_AB_MASK, reg)
}

static dig_tlv: [::core::ffi::c_uint; 4] = TLV_DB_SCALE_ITEM(-12750, 50, 0);

static chan_mix_texts: [*const ::core::ffi::c_char; 16] = [
    c"Mute".as_ptr(),
    c"MuteA".as_ptr(),
    c"MuteA SwapB".as_ptr(),
    c"MuteA MonoB".as_ptr(),
    c"SwapA MuteB".as_ptr(),
    c"BothR".as_ptr(),
    c"Swap".as_ptr(),
    c"SwapA MonoB".as_ptr(),
    c"MuteB".as_ptr(),
    c"Normal".as_ptr(),
    c"BothL".as_ptr(),
    c"MonoB".as_ptr(),
    c"MonoA MuteB".as_ptr(),
    c"MonoA".as_ptr(),
    c"MonoA SwapB".as_ptr(),
    c"Mono".as_ptr(),
    /*Normal == Channel A = Left, Channel B = Right*/
];

static fm_texts: [*const ::core::ffi::c_char; 4] = [
    c"Auto".as_ptr(),
    c"Single".as_ptr(),
    c"Double".as_ptr(),
    c"Quad".as_ptr(),
];

static deemph_texts: [*const ::core::ffi::c_char; 4] = [
    c"None".as_ptr(),
    c"44.1k".as_ptr(),
    c"48k".as_ptr(),
    c"32k".as_ptr(),
];

static softr_zeroc_texts: [*const ::core::ffi::c_char; 4] = [
    c"Immediate".as_ptr(),
    c"Zero Cross".as_ptr(),
    c"Soft Ramp".as_ptr(),
    c"SR on ZC".as_ptr(),
];

static mut deemph_values: [::core::ffi::c_int; 4] = [0, 4, 8, 12];

static mut softr_zeroc_values: [::core::ffi::c_int; 4] = [0, 64, 128, 192];

static chan_mix_enum: soc_enum =
    SOC_ENUM_SINGLE(CS4349_VMI, 0, ARRAY_SIZE(&chan_mix_texts), &chan_mix_texts);

static fm_mode_enum: soc_enum =
    SOC_ENUM_SINGLE(CS4349_MODE, 0, ARRAY_SIZE(&fm_texts), &fm_texts);

SOC_VALUE_ENUM_SINGLE_DECL!(
    deemph_enum,
    CS4349_MODE,
    0,
    DEM_MASK,
    deemph_texts,
    deemph_values
);

SOC_VALUE_ENUM_SINGLE_DECL!(
    softr_zeroc_enum,
    CS4349_RMPFLT,
    0,
    SR_ZC_MASK,
    softr_zeroc_texts,
    softr_zeroc_values
);

static cs4349_snd_controls: [snd_kcontrol_new; 15] = [
    SOC_DOUBLE_R_TLV!(
        c"Master Playback Volume",
        CS4349_VOLA,
        CS4349_VOLB,
        0,
        0xFF,
        1,
        dig_tlv
    ),
    SOC_ENUM!(c"Functional Mode", fm_mode_enum),
    SOC_ENUM!(c"De-Emphasis Control", deemph_enum),
    SOC_ENUM!(c"Soft Ramp Zero Cross Control", softr_zeroc_enum),
    SOC_ENUM!(c"Channel Mixer", chan_mix_enum),
    SOC_SINGLE!(c"VolA = VolB Switch", CS4349_VMI, 7, 1, 0),
    SOC_SINGLE!(c"InvertA Switch", CS4349_VMI, 6, 1, 0),
    SOC_SINGLE!(c"InvertB Switch", CS4349_VMI, 5, 1, 0),
    SOC_SINGLE!(c"Auto-Mute Switch", CS4349_MUTE, 7, 1, 0),
    SOC_SINGLE!(c"MUTEC A = B Switch", CS4349_MUTE, 5, 1, 0),
    SOC_SINGLE!(c"Soft Ramp Up Switch", CS4349_RMPFLT, 5, 1, 0),
    SOC_SINGLE!(c"Soft Ramp Down Switch", CS4349_RMPFLT, 4, 1, 0),
    SOC_SINGLE!(c"Slow Roll Off Filter Switch", CS4349_RMPFLT, 2, 1, 0),
    SOC_SINGLE!(c"Freeze Switch", CS4349_MISC, 5, 1, 0),
    SOC_SINGLE!(c"Popguard Switch", CS4349_MISC, 4, 1, 0),
];

static cs4349_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    SND_SOC_DAPM_DAC!(c"HiFi DAC", ::core::ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUTPUT!(c"OutputA"),
    SND_SOC_DAPM_OUTPUT!(c"OutputB"),
];

static cs4349_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: c"DAC Playback".as_ptr(),
        control: ::core::ptr::null(),
        source: c"OutputA".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DAC Playback".as_ptr(),
        control: ::core::ptr::null(),
        source: c"OutputB".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OutputA".as_ptr(),
        control: ::core::ptr::null(),
        source: c"HiFi DAC".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OutputB".as_ptr(),
        control: ::core::ptr::null(),
        source: c"HiFi DAC".as_ptr(),
    },
];

const CS4349_PCM_FORMATS: ::core::ffi::c_uint = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

const CS4349_PCM_RATES: ::core::ffi::c_uint = SNDRV_PCM_RATE_8000_192000;

static cs4349_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(cs4349_pcm_hw_params),
    set_fmt: Some(cs4349_set_dai_fmt),
    mute_stream: Some(cs4349_mute),
    no_capture_mute: 1,
};

static mut cs4349_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"cs4349_hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"DAC Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: CS4349_PCM_RATES,
        formats: CS4349_PCM_FORMATS,
    },
    ops: unsafe { &cs4349_dai_ops as *const snd_soc_dai_ops },
    symmetric_rate: 1,
};

static soc_component_dev_cs4349: snd_soc_component_driver = snd_soc_component_driver {
    controls: cs4349_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&cs4349_snd_controls),
    dapm_widgets: cs4349_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&cs4349_dapm_widgets),
    dapm_routes: cs4349_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&cs4349_routes),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static cs4349_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: CS4349_MISC,
    reg_defaults: cs4349_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&cs4349_reg_defaults),
    readable_reg: Some(cs4349_readable_register),
    writeable_reg: Some(cs4349_writeable_register),
    cache_type: REGCACHE_MAPLE,
};

unsafe extern "C" fn cs4349_i2c_probe(client: *mut i2c_client) -> ::core::ffi::c_int {
    let cs4349: *mut cs4349_private;
    let ret: ::core::ffi::c_int;

    cs4349 = devm_kzalloc(
        &mut (*client).dev,
        ::core::mem::size_of::<cs4349_private>(),
        GFP_KERNEL,
    ) as *mut cs4349_private;
    if cs4349.is_null() {
        return -ENOMEM;
    }

    (*cs4349).regmap = devm_regmap_init_i2c(client, &cs4349_regmap);
    if IS_ERR((*cs4349).regmap as *const ::core::ffi::c_void) {
        ret = PTR_ERR((*cs4349).regmap as *const ::core::ffi::c_void);
        dev_err(
            &mut (*client).dev,
            c"regmap_init() failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    /* Reset the Device */
    (*cs4349).reset_gpio =
        devm_gpiod_get_optional(&mut (*client).dev, c"reset".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*cs4349).reset_gpio as *const ::core::ffi::c_void) {
        return PTR_ERR((*cs4349).reset_gpio as *const ::core::ffi::c_void);
    }

    gpiod_set_value_cansleep((*cs4349).reset_gpio, 1);

    i2c_set_clientdata(client, cs4349 as *mut ::core::ffi::c_void);

    devm_snd_soc_register_component(
        &mut (*client).dev,
        &soc_component_dev_cs4349,
        &mut cs4349_dai,
        1,
    )
}

unsafe extern "C" fn cs4349_i2c_remove(client: *mut i2c_client) {
    let cs4349: *mut cs4349_private = i2c_get_clientdata(client) as *mut cs4349_private;

    /* Hold down reset */
    gpiod_set_value_cansleep((*cs4349).reset_gpio, 0);
}

unsafe extern "C" fn cs4349_runtime_suspend(dev: *mut device) -> ::core::ffi::c_int {
    let cs4349: *mut cs4349_private = dev_get_drvdata(dev) as *mut cs4349_private;
    let ret: ::core::ffi::c_int;

    ret = regmap_update_bits((*cs4349).regmap, CS4349_MISC, PWR_DWN, PWR_DWN);
    if ret < 0 {
        return ret;
    }

    regcache_cache_only((*cs4349).regmap, true);

    /* Hold down reset */
    gpiod_set_value_cansleep((*cs4349).reset_gpio, 0);

    0
}

unsafe extern "C" fn cs4349_runtime_resume(dev: *mut device) -> ::core::ffi::c_int {
    let cs4349: *mut cs4349_private = dev_get_drvdata(dev) as *mut cs4349_private;
    let mut ret: ::core::ffi::c_int;

    ret = regmap_update_bits((*cs4349).regmap, CS4349_MISC, PWR_DWN, 0);
    if ret < 0 {
        return ret;
    }

    gpiod_set_value_cansleep((*cs4349).reset_gpio, 1);

    regcache_cache_only((*cs4349).regmap, false);
    ret = regcache_sync((*cs4349).regmap);
    if ret != 0 {
        regcache_cache_only((*cs4349).regmap, true);
        regcache_mark_dirty((*cs4349).regmap);
        gpiod_set_value_cansleep((*cs4349).reset_gpio, 0);
        return ret;
    }

    0
}

static cs4349_runtime_pm: dev_pm_ops = dev_pm_ops {
    RUNTIME_PM_OPS!(cs4349_runtime_suspend, cs4349_runtime_resume, NULL)
};

static cs4349_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"cirrus,cs4349".as_ptr(),
    },
    of_device_id {},
];

MODULE_DEVICE_TABLE!(of, cs4349_of_match);

static cs4349_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: *b"cs4349\0",
    },
    i2c_device_id {},
];

MODULE_DEVICE_TABLE!(i2c, cs4349_i2c_id);

static mut cs4349_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"cs4349".as_ptr(),
        of_match_table: cs4349_of_match.as_ptr(),
        pm: pm_ptr(&cs4349_runtime_pm),
    },
    id_table: cs4349_i2c_id.as_ptr(),
    probe: Some(cs4349_i2c_probe),
    remove: Some(cs4349_i2c_remove),
};

module_i2c_driver!(cs4349_i2c_driver);

MODULE_AUTHOR!(c"Tim Howe <tim.howe@cirrus.com>");
MODULE_DESCRIPTION!(c"Cirrus Logic CS4349 ALSA SoC Codec Driver");
MODULE_LICENSE!(c"GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
