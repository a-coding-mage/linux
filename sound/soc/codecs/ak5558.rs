// SPDX-License-Identifier: GPL-2.0
//
// Audio driver for AK5558 ADC
//
// Copyright (C) 2015 Asahi Kasei Microdevices Corporation
// Copyright 2018 NXP

// C dependencies translated as external Rust dependencies:
// linux/delay.h, linux/gpio/consumer.h, linux/i2c.h, linux/module.h,
// linux/of.h, linux/pm_runtime.h, linux/regmap.h,
// linux/regulator/consumer.h, linux/slab.h, sound/initval.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-dapm.h, sound/tlv.h, "ak5558.h".

#[repr(C)]
enum ak555x_type {
    AK5558,
    AK5552,
}

const AK5558_NUM_SUPPLIES: usize = 2;
static ak5558_supply_names: [*const c_char; AK5558_NUM_SUPPLIES] = [
    c"DVDD".as_ptr(),
    c"AVDD".as_ptr(),
];

/* AK5558 Codec Private Data */
#[repr(C)]
struct ak5558_priv {
    supplies: [regulator_bulk_data; AK5558_NUM_SUPPLIES],
    regmap: *mut regmap,
    i2c: *mut i2c_client,
    reset_gpiod: *mut gpio_desc, /* Reset & Power down GPIO */
    slots: c_int,
    slot_width: c_int,
}

/* ak5558 register cache & default register settings */
static ak5558_reg: [reg_default; 6] = [
    reg_default { reg: 0x0, def: 0xff }, /* 0x00 AK5558_00_POWER_MANAGEMENT1 */
    reg_default { reg: 0x1, def: 0x01 }, /* 0x01 AK5558_01_POWER_MANAGEMENT2 */
    reg_default { reg: 0x2, def: 0x01 }, /* 0x02 AK5558_02_CONTROL1 */
    reg_default { reg: 0x3, def: 0x00 }, /* 0x03 AK5558_03_CONTROL2 */
    reg_default { reg: 0x4, def: 0x00 }, /* 0x04 AK5558_04_CONTROL3 */
    reg_default { reg: 0x5, def: 0x00 }, /* 0x05 AK5558_05_DSD */
];

static mono_texts: [*const c_char; 4] = [
    c"8 Slot".as_ptr(),
    c"2 Slot".as_ptr(),
    c"4 Slot".as_ptr(),
    c"1 Slot".as_ptr(),
];

static ak5558_mono_enum: [soc_enum; 1] = [
    SOC_ENUM_SINGLE(AK5558_01_POWER_MANAGEMENT2, 1, mono_texts.len(), mono_texts.as_ptr()),
];

static mono_5552_texts: [*const c_char; 4] = [
    c"2 Slot".as_ptr(),
    c"1 Slot (Fixed)".as_ptr(),
    c"2 Slot".as_ptr(),
    c"1 Slot (Optimal)".as_ptr(),
];

static ak5552_mono_enum: [soc_enum; 1] = [
    SOC_ENUM_SINGLE(
        AK5558_01_POWER_MANAGEMENT2,
        1,
        mono_5552_texts.len(),
        mono_5552_texts.as_ptr(),
    ),
];

static digfil_texts: [*const c_char; 4] = [
    c"Sharp Roll-Off".as_ptr(),
    c"Slow Roll-Off".as_ptr(),
    c"Short Delay Sharp Roll-Off".as_ptr(),
    c"Short Delay Slow Roll-Off".as_ptr(),
];

static ak5558_adcset_enum: [soc_enum; 1] = [
    SOC_ENUM_SINGLE(AK5558_04_CONTROL3, 0, digfil_texts.len(), digfil_texts.as_ptr()),
];

static ak5558_snd_controls: [snd_kcontrol_new; 2] = [
    SOC_ENUM(c"Monaural Mode".as_ptr(), ak5558_mono_enum[0]),
    SOC_ENUM(c"Digital Filter".as_ptr(), ak5558_adcset_enum[0]),
];

static ak5552_snd_controls: [snd_kcontrol_new; 2] = [
    SOC_ENUM(c"Monaural Mode".as_ptr(), ak5552_mono_enum[0]),
    SOC_ENUM(c"Digital Filter".as_ptr(), ak5558_adcset_enum[0]),
];

static ak5558_dapm_widgets: [snd_soc_dapm_widget; 17] = [
    /* Analog Input */
    SND_SOC_DAPM_INPUT(c"AIN1".as_ptr()),
    SND_SOC_DAPM_INPUT(c"AIN2".as_ptr()),
    SND_SOC_DAPM_INPUT(c"AIN3".as_ptr()),
    SND_SOC_DAPM_INPUT(c"AIN4".as_ptr()),
    SND_SOC_DAPM_INPUT(c"AIN5".as_ptr()),
    SND_SOC_DAPM_INPUT(c"AIN6".as_ptr()),
    SND_SOC_DAPM_INPUT(c"AIN7".as_ptr()),
    SND_SOC_DAPM_INPUT(c"AIN8".as_ptr()),
    SND_SOC_DAPM_ADC(c"ADC Ch1".as_ptr(), core::ptr::null(), AK5558_00_POWER_MANAGEMENT1, 0, 0),
    SND_SOC_DAPM_ADC(c"ADC Ch2".as_ptr(), core::ptr::null(), AK5558_00_POWER_MANAGEMENT1, 1, 0),
    SND_SOC_DAPM_ADC(c"ADC Ch3".as_ptr(), core::ptr::null(), AK5558_00_POWER_MANAGEMENT1, 2, 0),
    SND_SOC_DAPM_ADC(c"ADC Ch4".as_ptr(), core::ptr::null(), AK5558_00_POWER_MANAGEMENT1, 3, 0),
    SND_SOC_DAPM_ADC(c"ADC Ch5".as_ptr(), core::ptr::null(), AK5558_00_POWER_MANAGEMENT1, 4, 0),
    SND_SOC_DAPM_ADC(c"ADC Ch6".as_ptr(), core::ptr::null(), AK5558_00_POWER_MANAGEMENT1, 5, 0),
    SND_SOC_DAPM_ADC(c"ADC Ch7".as_ptr(), core::ptr::null(), AK5558_00_POWER_MANAGEMENT1, 6, 0),
    SND_SOC_DAPM_ADC(c"ADC Ch8".as_ptr(), core::ptr::null(), AK5558_00_POWER_MANAGEMENT1, 7, 0),
    SND_SOC_DAPM_AIF_OUT(c"SDTO".as_ptr(), c"Capture".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
];

static ak5552_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    /* Analog Input */
    SND_SOC_DAPM_INPUT(c"AIN1".as_ptr()),
    SND_SOC_DAPM_INPUT(c"AIN2".as_ptr()),
    SND_SOC_DAPM_ADC(c"ADC Ch1".as_ptr(), core::ptr::null(), AK5558_00_POWER_MANAGEMENT1, 0, 0),
    SND_SOC_DAPM_ADC(c"ADC Ch2".as_ptr(), core::ptr::null(), AK5558_00_POWER_MANAGEMENT1, 1, 0),
    SND_SOC_DAPM_AIF_OUT(c"SDTO".as_ptr(), c"Capture".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
];

static ak5558_intercon: [snd_soc_dapm_route; 16] = [
    snd_soc_dapm_route { sink: c"ADC Ch1".as_ptr(), control: core::ptr::null(), source: c"AIN1".as_ptr() },
    snd_soc_dapm_route { sink: c"SDTO".as_ptr(), control: core::ptr::null(), source: c"ADC Ch1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC Ch2".as_ptr(), control: core::ptr::null(), source: c"AIN2".as_ptr() },
    snd_soc_dapm_route { sink: c"SDTO".as_ptr(), control: core::ptr::null(), source: c"ADC Ch2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC Ch3".as_ptr(), control: core::ptr::null(), source: c"AIN3".as_ptr() },
    snd_soc_dapm_route { sink: c"SDTO".as_ptr(), control: core::ptr::null(), source: c"ADC Ch3".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC Ch4".as_ptr(), control: core::ptr::null(), source: c"AIN4".as_ptr() },
    snd_soc_dapm_route { sink: c"SDTO".as_ptr(), control: core::ptr::null(), source: c"ADC Ch4".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC Ch5".as_ptr(), control: core::ptr::null(), source: c"AIN5".as_ptr() },
    snd_soc_dapm_route { sink: c"SDTO".as_ptr(), control: core::ptr::null(), source: c"ADC Ch5".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC Ch6".as_ptr(), control: core::ptr::null(), source: c"AIN6".as_ptr() },
    snd_soc_dapm_route { sink: c"SDTO".as_ptr(), control: core::ptr::null(), source: c"ADC Ch6".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC Ch7".as_ptr(), control: core::ptr::null(), source: c"AIN7".as_ptr() },
    snd_soc_dapm_route { sink: c"SDTO".as_ptr(), control: core::ptr::null(), source: c"ADC Ch7".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC Ch8".as_ptr(), control: core::ptr::null(), source: c"AIN8".as_ptr() },
    snd_soc_dapm_route { sink: c"SDTO".as_ptr(), control: core::ptr::null(), source: c"ADC Ch8".as_ptr() },
];

static ak5552_intercon: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: c"ADC Ch1".as_ptr(), control: core::ptr::null(), source: c"AIN1".as_ptr() },
    snd_soc_dapm_route { sink: c"SDTO".as_ptr(), control: core::ptr::null(), source: c"ADC Ch1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC Ch2".as_ptr(), control: core::ptr::null(), source: c"AIN2".as_ptr() },
    snd_soc_dapm_route { sink: c"SDTO".as_ptr(), control: core::ptr::null(), source: c"ADC Ch2".as_ptr() },
];

unsafe extern "C" fn ak5558_set_mcki(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_update_bits(
        component,
        AK5558_02_CONTROL1,
        AK5558_CKS,
        AK5558_CKS_AUTO,
    )
}

unsafe extern "C" fn ak5558_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let ak5558: *mut ak5558_priv = snd_soc_component_get_drvdata(component) as *mut ak5558_priv;
    let bits: u8;
    let pcm_width: c_int = max(params_physical_width(params), (*ak5558).slot_width);

    match pcm_width {
        16 => {
            bits = AK5558_DIF_24BIT_MODE as u8;
        }
        32 => {
            bits = AK5558_DIF_32BIT_MODE as u8;
        }
        _ => {
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, AK5558_02_CONTROL1, AK5558_BITS, bits as c_uint);

    0
}

unsafe extern "C" fn ak5558_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let format: u8;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        SND_SOC_DAIFMT_CBP_CFP => {}
        SND_SOC_DAIFMT_CBC_CFP | SND_SOC_DAIFMT_CBP_CFC => {
            dev_err((*dai).dev, c"Clock mode unsupported".as_ptr());
            return -EINVAL;
        }
        _ => {
            dev_err((*dai).dev, c"Clock mode unsupported".as_ptr());
            return -EINVAL;
        }
    }

    /* set master/slave audio interface */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            format = AK5558_DIF_I2S_MODE as u8;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            format = AK5558_DIF_MSB_MODE as u8;
        }
        SND_SOC_DAIFMT_DSP_B => {
            format = AK5558_DIF_MSB_MODE as u8;
        }
        _ => {
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, AK5558_02_CONTROL1, AK5558_DIF, format as c_uint);

    0
}

unsafe extern "C" fn ak5558_set_tdm_slot(
    dai: *mut snd_soc_dai,
    _tx_mask: c_uint,
    _rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let ak5558: *mut ak5558_priv = snd_soc_component_get_drvdata(component) as *mut ak5558_priv;
    let tdm_mode: c_int;

    (*ak5558).slots = slots;
    (*ak5558).slot_width = slot_width;

    match slots * slot_width {
        128 => {
            tdm_mode = AK5558_MODE_TDM128;
        }
        256 => {
            tdm_mode = AK5558_MODE_TDM256;
        }
        512 => {
            tdm_mode = AK5558_MODE_TDM512;
        }
        _ => {
            tdm_mode = AK5558_MODE_NORMAL;
        }
    }

    snd_soc_component_update_bits(
        component,
        AK5558_03_CONTROL2,
        AK5558_MODE_BITS,
        tdm_mode as c_uint,
    );
    0
}

const AK5558_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static ak5558_rates: [c_uint; 17] = [
    8000, 11025, 16000, 22050, 32000, 44100, 48000, 88200, 96000, 176400, 192000,
    352800, 384000, 705600, 768000, 1411200, 2822400,
];

static ak5558_rate_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: ak5558_rates.len() as c_uint,
    list: ak5558_rates.as_ptr(),
};

unsafe extern "C" fn ak5558_startup(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) -> c_int {
    snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &ak5558_rate_constraints,
    )
}

static ak5558_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(ak5558_startup),
    hw_params: Some(ak5558_hw_params),
    set_fmt: Some(ak5558_set_dai_fmt),
    set_tdm_slot: Some(ak5558_set_tdm_slot),
};

static mut ak5558_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"ak5558-aif".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: AK5558_FORMATS,
    },
    ops: &ak5558_dai_ops,
};

static mut ak5552_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"ak5552-aif".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: AK5558_FORMATS,
    },
    ops: &ak5558_dai_ops,
};

unsafe extern "C" fn ak5558_reset(ak5558: *mut ak5558_priv, active: bool) {
    if (*ak5558).reset_gpiod.is_null() {
        return;
    }

    gpiod_set_value_cansleep((*ak5558).reset_gpiod, active as c_int);
    usleep_range(1000, 2000);
}

unsafe extern "C" fn ak5558_probe(component: *mut snd_soc_component) -> c_int {
    let ak5558: *mut ak5558_priv = snd_soc_component_get_drvdata(component) as *mut ak5558_priv;

    ak5558_reset(ak5558, false);
    ak5558_set_mcki(component)
}

unsafe extern "C" fn ak5558_remove(component: *mut snd_soc_component) {
    let ak5558: *mut ak5558_priv = snd_soc_component_get_drvdata(component) as *mut ak5558_priv;

    ak5558_reset(ak5558, true);
}

unsafe extern "C" fn ak5558_runtime_suspend(dev: *mut device) -> c_int {
    let ak5558: *mut ak5558_priv = dev_get_drvdata(dev) as *mut ak5558_priv;

    regcache_cache_only((*ak5558).regmap, true);
    ak5558_reset(ak5558, true);

    regulator_bulk_disable((*ak5558).supplies.len() as c_int, (*ak5558).supplies.as_mut_ptr());
    0
}

unsafe extern "C" fn ak5558_runtime_resume(dev: *mut device) -> c_int {
    let ak5558: *mut ak5558_priv = dev_get_drvdata(dev) as *mut ak5558_priv;
    let mut ret: c_int;

    ret = regulator_bulk_enable((*ak5558).supplies.len() as c_int, (*ak5558).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, c"Failed to enable supplies: %d\n".as_ptr(), ret);
        return ret;
    }

    ak5558_reset(ak5558, true);
    ak5558_reset(ak5558, false);

    regcache_cache_only((*ak5558).regmap, false);
    regcache_mark_dirty((*ak5558).regmap);

    ret = regcache_sync((*ak5558).regmap);
    if ret != 0 {
        regcache_cache_only((*ak5558).regmap, true);
        regulator_bulk_disable((*ak5558).supplies.len() as c_int, (*ak5558).supplies.as_mut_ptr());
        return ret;
    }

    0
}

static ak5558_pm: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(ak5558_runtime_suspend),
    runtime_resume: Some(ak5558_runtime_resume),
    suspend: Some(pm_runtime_force_suspend),
    resume: Some(pm_runtime_force_resume),
};

static soc_codec_dev_ak5558: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ak5558_probe),
    remove: Some(ak5558_remove),
    controls: ak5558_snd_controls.as_ptr(),
    num_controls: ak5558_snd_controls.len() as c_uint,
    dapm_widgets: ak5558_dapm_widgets.as_ptr(),
    num_dapm_widgets: ak5558_dapm_widgets.len() as c_uint,
    dapm_routes: ak5558_intercon.as_ptr(),
    num_dapm_routes: ak5558_intercon.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static soc_codec_dev_ak5552: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ak5558_probe),
    remove: Some(ak5558_remove),
    controls: ak5552_snd_controls.as_ptr(),
    num_controls: ak5552_snd_controls.len() as c_uint,
    dapm_widgets: ak5552_dapm_widgets.as_ptr(),
    num_dapm_widgets: ak5552_dapm_widgets.len() as c_uint,
    dapm_routes: ak5552_intercon.as_ptr(),
    num_dapm_routes: ak5552_intercon.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static ak5558_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: AK5558_05_DSD,
    reg_defaults: ak5558_reg.as_ptr(),
    num_reg_defaults: ak5558_reg.len() as c_uint,
    cache_type: REGCACHE_RBTREE,
};

unsafe extern "C" fn ak5558_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let ak5558: *mut ak5558_priv;
    let mut ret: c_int = 0;
    let dev_id: c_int;
    let mut i: c_int;

    ak5558 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<ak5558_priv>(),
        GFP_KERNEL,
    ) as *mut ak5558_priv;
    if ak5558.is_null() {
        return -ENOMEM;
    }

    (*ak5558).regmap = devm_regmap_init_i2c(i2c, &ak5558_regmap);
    if IS_ERR((*ak5558).regmap as *const c_void) {
        return PTR_ERR((*ak5558).regmap as *const c_void) as c_int;
    }

    i2c_set_clientdata(i2c, ak5558 as *mut c_void);
    (*ak5558).i2c = i2c;

    (*ak5558).reset_gpiod =
        devm_gpiod_get_optional(&mut (*i2c).dev, c"reset".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*ak5558).reset_gpiod as *const c_void) {
        return PTR_ERR((*ak5558).reset_gpiod as *const c_void) as c_int;
    }

    i = 0;
    while (i as usize) < (*ak5558).supplies.len() {
        (*ak5558).supplies[i as usize].supply = ak5558_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        &mut (*i2c).dev,
        (*ak5558).supplies.len() as c_int,
        (*ak5558).supplies.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err(&mut (*i2c).dev, c"Failed to request supplies: %d\n".as_ptr(), ret);
        return ret;
    }

    dev_id = of_device_get_match_data(&mut (*i2c).dev) as uintptr_t as c_int;
    match dev_id {
        x if x == ak555x_type::AK5552 as c_int => {
            ret = devm_snd_soc_register_component(
                &mut (*i2c).dev,
                &soc_codec_dev_ak5552,
                &mut ak5552_dai,
                1,
            );
        }
        x if x == ak555x_type::AK5558 as c_int => {
            ret = devm_snd_soc_register_component(
                &mut (*i2c).dev,
                &soc_codec_dev_ak5558,
                &mut ak5558_dai,
                1,
            );
        }
        _ => {
            dev_err(&mut (*i2c).dev, c"unexpected device type\n".as_ptr());
            return -EINVAL;
        }
    }
    if ret < 0 {
        dev_err(&mut (*i2c).dev, c"failed to register component: %d\n".as_ptr(), ret);
        return ret;
    }

    pm_runtime_enable(&mut (*i2c).dev);
    regcache_cache_only((*ak5558).regmap, true);

    0
}

unsafe extern "C" fn ak5558_i2c_remove(i2c: *mut i2c_client) {
    pm_runtime_disable(&mut (*i2c).dev);
}

// Original C used __maybe_unused on this OF match table.
static ak5558_i2c_dt_ids: [of_device_id; 3] = [
    of_device_id {
        compatible: c"asahi-kasei,ak5558".as_ptr(),
        data: ak555x_type::AK5558 as uintptr_t as *const c_void,
    },
    of_device_id {
        compatible: c"asahi-kasei,ak5552".as_ptr(),
        data: ak555x_type::AK5552 as uintptr_t as *const c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
MODULE_DEVICE_TABLE(of, ak5558_i2c_dt_ids);

static mut ak5558_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"ak5558".as_ptr(),
        of_match_table: of_match_ptr(ak5558_i2c_dt_ids.as_ptr()),
        pm: pm_ptr(&ak5558_pm),
    },
    probe: Some(ak5558_i2c_probe),
    remove: Some(ak5558_i2c_remove),
};

module_i2c_driver!(ak5558_i2c_driver);

MODULE_AUTHOR(c"Junichi Wakasugi <wakasugi.jb@om.asahi-kasei.co.jp>".as_ptr());
MODULE_AUTHOR(c"Mihai Serban <mihai.serban@nxp.com>".as_ptr());
MODULE_DESCRIPTION(c"ASoC AK5558 ADC driver".as_ptr());
MODULE_LICENSE(c"GPL v2".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
