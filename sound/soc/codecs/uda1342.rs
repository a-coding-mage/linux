// SPDX-License-Identifier: GPL-2.0-only
//
// uda1342.c  --  UDA1342 ALSA SoC Codec driver
// Based on the WM87xx drivers by Liam Girdwood and Richard Purdie
//
// Copyright 2007 Dension Audio Systems Ltd.
// Copyright 2024 Loongson Technology Co.,Ltd.
//
// Modifications by Christian Pellegrin <chripell@evolware.org>
// Further cleanup and restructuring by:
//         Binbin Zhou <zhoubinbin@loongson.cn>

// Dependencies from Linux, ALSA SoC, regmap, I2C, PM runtime, and "uda1342.h"
// are expected to be supplied by the surrounding kernel Rust bindings.

const UDA134X_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

#[repr(C)]
struct uda1342_priv {
    sysclk: core::ffi::c_int,
    dai_fmt: core::ffi::c_int,

    provider_substream: *mut snd_pcm_substream,
    consumer_substream: *mut snd_pcm_substream,

    regmap: *mut regmap,
    i2c: *mut i2c_client,
}

static uda1342_reg_defaults: [reg_default; 7] = [
    reg_default { reg: 0x00, def: 0x1042 },
    reg_default { reg: 0x01, def: 0x0000 },
    reg_default { reg: 0x10, def: 0x0088 },
    reg_default { reg: 0x11, def: 0x0000 },
    reg_default { reg: 0x12, def: 0x0000 },
    reg_default { reg: 0x20, def: 0x0080 },
    reg_default { reg: 0x21, def: 0x0080 },
];

unsafe extern "C" fn uda1342_mute(
    dai: *mut snd_soc_dai,
    mute: core::ffi::c_int,
    direction: core::ffi::c_int,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let uda1342: *mut uda1342_priv = snd_soc_component_get_drvdata(component) as *mut uda1342_priv;
    let mask: u32;
    let mut val: u32 = 0;

    /* Master mute */
    mask = BIT(5);
    if mute != 0 {
        val = mask;
    }

    regmap_update_bits((*uda1342).regmap, 0x10, mask, val)
}

unsafe extern "C" fn uda1342_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let uda1342: *mut uda1342_priv = snd_soc_component_get_drvdata(component) as *mut uda1342_priv;
    let provider_runtime: *mut snd_pcm_runtime;

    if !(*uda1342).provider_substream.is_null() {
        provider_runtime = (*(*uda1342).provider_substream).runtime;

        snd_pcm_hw_constraint_single(
            (*substream).runtime,
            SNDRV_PCM_HW_PARAM_RATE,
            (*provider_runtime).rate,
        );
        snd_pcm_hw_constraint_single(
            (*substream).runtime,
            SNDRV_PCM_HW_PARAM_SAMPLE_BITS,
            (*provider_runtime).sample_bits,
        );

        (*uda1342).consumer_substream = substream;
    } else {
        (*uda1342).provider_substream = substream;
    }

    0
}

unsafe extern "C" fn uda1342_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let component: *mut snd_soc_component = (*dai).component;
    let uda1342: *mut uda1342_priv = snd_soc_component_get_drvdata(component) as *mut uda1342_priv;

    if (*uda1342).provider_substream == substream {
        (*uda1342).provider_substream = (*uda1342).consumer_substream;
    }

    (*uda1342).consumer_substream = core::ptr::null_mut();
}

unsafe extern "C" fn uda1342_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let uda1342: *mut uda1342_priv = snd_soc_component_get_drvdata(component) as *mut uda1342_priv;
    let dev: *mut device = &mut (*(*uda1342).i2c).dev;
    let mut hw_params: u32 = 0;

    if substream == (*uda1342).consumer_substream {
        return 0;
    }

    /* set SYSCLK / fs ratio */
    match (*uda1342).sysclk / params_rate(params) {
        512 => {}
        384 => {
            hw_params |= BIT(4);
        }
        256 => {
            hw_params |= BIT(5);
        }
        _ => {
            dev_err(dev, c"unsupported frequency\n".as_ptr());
            return -EINVAL;
        }
    }

    /* set DAI format and word length */
    match ((*uda1342).dai_fmt as u32) & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {}
        SND_SOC_DAIFMT_RIGHT_J => {
            match params_width(params) {
                16 => {
                    hw_params |= BIT(1);
                }
                18 => {
                    hw_params |= BIT(2);
                }
                20 => {
                    hw_params |= BIT(2) | BIT(1);
                }
                _ => {
                    dev_err(dev, c"unsupported format (right)\n".as_ptr());
                    return -EINVAL;
                }
            }
        }
        SND_SOC_DAIFMT_LEFT_J => {
            hw_params |= BIT(3);
        }
        _ => {
            dev_err(dev, c"unsupported format\n".as_ptr());
            return -EINVAL;
        }
    }

    regmap_update_bits(
        (*uda1342).regmap,
        0x0,
        STATUS0_DAIFMT_MASK | STATUS0_SYSCLK_MASK,
        hw_params,
    )
}

unsafe extern "C" fn uda1342_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: core::ffi::c_int,
    freq: u32,
    dir: core::ffi::c_int,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let uda1342: *mut uda1342_priv = snd_soc_component_get_drvdata(component) as *mut uda1342_priv;
    let dev: *mut device = &mut (*(*uda1342).i2c).dev;

    /*
     * Anything between 256fs*8Khz and 512fs*48Khz should be acceptable
     * because the codec is slave. Of course limitations of the clock
     * master (the IIS controller) apply.
     * We'll error out on set_hw_params if it's not OK
     */
    if freq >= (256 * 8000) && freq <= (512 * 48000) {
        (*uda1342).sysclk = freq as core::ffi::c_int;
        return 0;
    }

    dev_err(dev, c"unsupported sysclk\n".as_ptr());

    -EINVAL
}

unsafe extern "C" fn uda1342_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: u32,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let uda1342: *mut uda1342_priv = snd_soc_component_get_drvdata(component) as *mut uda1342_priv;

    /* codec supports only full consumer mode */
    if (fmt & SND_SOC_DAIFMT_MASTER_MASK) != SND_SOC_DAIFMT_BC_FC {
        dev_err(&mut (*(*uda1342).i2c).dev, c"unsupported consumer mode.\n".as_ptr());
        return -EINVAL;
    }

    /* We can't setup DAI format here as it depends on the word bit num */
    /* so let's just store the value for later */
    (*uda1342).dai_fmt = fmt as core::ffi::c_int;

    0
}

static uda1342_snd_controls: [snd_kcontrol_new; 2] = [
    SOC_SINGLE(c"Master Playback Volume".as_ptr(), 0x11, 0, 0x3F, 1),
    SOC_SINGLE(c"Analog1 Volume".as_ptr(), 0x12, 0, 0x1F, 1),
];

/* Common DAPM widgets */
static uda1342_dapm_widgets: [snd_soc_dapm_widget; 8] = [
    SND_SOC_DAPM_INPUT(c"VINL1".as_ptr()),
    SND_SOC_DAPM_INPUT(c"VINR1".as_ptr()),
    SND_SOC_DAPM_INPUT(c"VINL2".as_ptr()),
    SND_SOC_DAPM_INPUT(c"VINR2".as_ptr()),

    SND_SOC_DAPM_DAC(c"DAC".as_ptr(), c"Playback".as_ptr(), 0, 1, 0),
    SND_SOC_DAPM_ADC(c"ADC".as_ptr(), c"Capture".as_ptr(), 0, 9, 0),

    SND_SOC_DAPM_OUTPUT(c"VOUTL".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"VOUTR".as_ptr()),
];

static uda1342_dapm_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"VINL1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"VINR1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"VINL2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"VINR2".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUTL".as_ptr(), control: core::ptr::null(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUTR".as_ptr(), control: core::ptr::null(), source: c"DAC".as_ptr() },
];

static uda1342_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(uda1342_startup),
    shutdown: Some(uda1342_shutdown),
    hw_params: Some(uda1342_hw_params),
    mute_stream: Some(uda1342_mute),
    set_sysclk: Some(uda1342_set_dai_sysclk),
    set_fmt: Some(uda1342_set_dai_fmt),
};

static mut uda1342_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"uda1342-hifi".as_ptr(),
    /* playback capabilities */
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: UDA134X_FORMATS,
    },
    /* capture capabilities */
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: UDA134X_FORMATS,
    },
    /* pcm operations */
    ops: &uda1342_dai_ops,
};

static soc_component_dev_uda1342: snd_soc_component_driver = snd_soc_component_driver {
    controls: uda1342_snd_controls.as_ptr(),
    num_controls: uda1342_snd_controls.len() as u32,
    dapm_widgets: uda1342_dapm_widgets.as_ptr(),
    num_dapm_widgets: uda1342_dapm_widgets.len() as u32,
    dapm_routes: uda1342_dapm_routes.as_ptr(),
    num_dapm_routes: uda1342_dapm_routes.len() as u32,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static uda1342_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    max_register: 0x21,
    reg_defaults: uda1342_reg_defaults.as_ptr(),
    num_reg_defaults: uda1342_reg_defaults.len() as u32,
    cache_type: REGCACHE_MAPLE,
};

unsafe extern "C" fn uda1342_i2c_probe(i2c: *mut i2c_client) -> core::ffi::c_int {
    let uda1342: *mut uda1342_priv;

    uda1342 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<uda1342_priv>(),
        GFP_KERNEL,
    ) as *mut uda1342_priv;
    if uda1342.is_null() {
        return -ENOMEM;
    }

    (*uda1342).regmap = devm_regmap_init_i2c(i2c, &uda1342_regmap);
    if IS_ERR((*uda1342).regmap as *const core::ffi::c_void) {
        return PTR_ERR((*uda1342).regmap as *const core::ffi::c_void) as core::ffi::c_int;
    }

    i2c_set_clientdata(i2c, uda1342 as *mut core::ffi::c_void);
    (*uda1342).i2c = i2c;

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_uda1342,
        &mut uda1342_dai,
        1,
    )
}

unsafe extern "C" fn uda1342_suspend(dev: *mut device) -> core::ffi::c_int {
    let uda1342: *mut uda1342_priv = dev_get_drvdata(dev) as *mut uda1342_priv;

    regcache_cache_only((*uda1342).regmap, true);

    0
}

unsafe extern "C" fn uda1342_resume(dev: *mut device) -> core::ffi::c_int {
    let uda1342: *mut uda1342_priv = dev_get_drvdata(dev) as *mut uda1342_priv;
    let ret: core::ffi::c_int;

    regcache_cache_only((*uda1342).regmap, false);
    regcache_mark_dirty((*uda1342).regmap);
    ret = regcache_sync((*uda1342).regmap);
    if ret != 0 {
        regcache_cache_only((*uda1342).regmap, true);
        regcache_mark_dirty((*uda1342).regmap);
        return ret;
    }

    0
}

static uda1342_pm_ops: dev_pm_ops =
    DEFINE_RUNTIME_DEV_PM_OPS(uda1342_suspend, uda1342_resume, None);

static uda1342_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: c"uda1342".as_ptr() },
    i2c_device_id::default(),
];
MODULE_DEVICE_TABLE(i2c, uda1342_i2c_id);

static uda1342_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"nxp,uda1342".as_ptr() },
    of_device_id::default(),
];
MODULE_DEVICE_TABLE(of, uda1342_of_match);

static mut uda1342_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"uda1342".as_ptr(),
        of_match_table: uda1342_of_match.as_ptr(),
        pm: pm_sleep_ptr(&uda1342_pm_ops),
    },
    probe: Some(uda1342_i2c_probe),
    id_table: uda1342_i2c_id.as_ptr(),
};
module_i2c_driver!(uda1342_i2c_driver);

MODULE_DESCRIPTION!(c"UDA1342 ALSA soc codec driver");
MODULE_AUTHOR!(c"Zoltan Devai, Christian Pellegrin <chripell@evolware.org>");
MODULE_AUTHOR!(c"Binbin Zhou <zhoubinbin@loongson.cn>");
MODULE_LICENSE!(c"GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
