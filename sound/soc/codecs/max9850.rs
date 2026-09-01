// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * max9850.rs  --  codec driver for max9850
 *
 * Copyright (C) 2011 taskit GmbH
 *
 * Author: Christian Glindkamp <christian.glindkamp@taskit.de>
 *
 * Initial development of this code was funded by
 * MICRONIC Computer Systeme GmbH, https://www.mcsberlin.de/
 */

// Rust translation of the original C implementation. Kernel, ASoC, regmap,
// I2C, TLV, module, and MAX9850 register symbols are provided by external
// bindings corresponding to the original C includes.

#[repr(C)]
pub struct max9850_priv {
    pub regmap: *mut regmap,
    pub sysclk: c_uint,
}

/* these registers are not used at the moment but provided for the sake of
 * completeness */
unsafe extern "C" fn max9850_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        MAX9850_STATUSA | MAX9850_STATUSB => true,
        _ => false,
    }
}

static max9850_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,

    max_register: MAX9850_DIGITAL_AUDIO,
    volatile_reg: Some(max9850_volatile_register),
    cache_type: REGCACHE_RBTREE,
    ..unsafe { core::mem::zeroed() }
};

static max9850_tlv: [c_uint; DECLARE_TLV_DB_RANGE_SIZE!(4)] = DECLARE_TLV_DB_RANGE!(
    0x18, 0x1f, TLV_DB_SCALE_ITEM!(-7450, 400, 0),
    0x20, 0x33, TLV_DB_SCALE_ITEM!(-4150, 200, 0),
    0x34, 0x37, TLV_DB_SCALE_ITEM!(-150, 100, 0),
    0x38, 0x3f, TLV_DB_SCALE_ITEM!(250, 50, 0)
);

static max9850_controls: [snd_kcontrol_new; 3] = [
    SOC_SINGLE_TLV!("Headphone Volume\0", MAX9850_VOLUME, 0, 0x3f, 1, max9850_tlv),
    SOC_SINGLE!("Headphone Switch\0", MAX9850_VOLUME, 7, 1, 1),
    SOC_SINGLE!("Mono Switch\0", MAX9850_GENERAL_PURPOSE, 2, 1, 0),
];

static max9850_mixer_controls: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE!("Line In Switch\0", MAX9850_ENABLE, 1, 1, 0),
];

static max9850_dapm_widgets: [snd_soc_dapm_widget; 14] = [
    SND_SOC_DAPM_SUPPLY!("Charge Pump 1\0", MAX9850_ENABLE, 4, 0, None, 0),
    SND_SOC_DAPM_SUPPLY!("Charge Pump 2\0", MAX9850_ENABLE, 5, 0, None, 0),
    SND_SOC_DAPM_SUPPLY!("MCLK\0", MAX9850_ENABLE, 6, 0, None, 0),
    SND_SOC_DAPM_SUPPLY!("SHDN\0", MAX9850_ENABLE, 7, 0, None, 0),
    SND_SOC_DAPM_MIXER_NAMED_CTL!(
        "Output Mixer\0",
        MAX9850_ENABLE,
        2,
        0,
        &max9850_mixer_controls[0],
        ARRAY_SIZE!(max9850_mixer_controls)
    ),
    SND_SOC_DAPM_PGA!("Headphone Output\0", MAX9850_ENABLE, 3, 0, None, 0),
    SND_SOC_DAPM_DAC!("DAC\0", "HiFi Playback\0", MAX9850_ENABLE, 0, 0),
    SND_SOC_DAPM_OUTPUT!("OUTL\0"),
    SND_SOC_DAPM_OUTPUT!("HPL\0"),
    SND_SOC_DAPM_OUTPUT!("OUTR\0"),
    SND_SOC_DAPM_OUTPUT!("HPR\0"),
    SND_SOC_DAPM_MIXER!("Line Input\0", SND_SOC_NOPM, 0, 0, None, 0),
    SND_SOC_DAPM_INPUT!("INL\0"),
    SND_SOC_DAPM_INPUT!("INR\0"),
];

static max9850_dapm_routes: [snd_soc_dapm_route; 14] = [
    /* output mixer */
    snd_soc_dapm_route { sink: c"Output Mixer".as_ptr(), control: core::ptr::null(), source: c"DAC".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"Output Mixer".as_ptr(), control: c"Line In Switch".as_ptr(), source: c"Line Input".as_ptr(), ..unsafe { core::mem::zeroed() } },

    /* outputs */
    snd_soc_dapm_route { sink: c"Headphone Output".as_ptr(), control: core::ptr::null(), source: c"Output Mixer".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"HPL".as_ptr(), control: core::ptr::null(), source: c"Headphone Output".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"HPR".as_ptr(), control: core::ptr::null(), source: c"Headphone Output".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"OUTL".as_ptr(), control: core::ptr::null(), source: c"Output Mixer".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"OUTR".as_ptr(), control: core::ptr::null(), source: c"Output Mixer".as_ptr(), ..unsafe { core::mem::zeroed() } },

    /* inputs */
    snd_soc_dapm_route { sink: c"Line Input".as_ptr(), control: core::ptr::null(), source: c"INL".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"Line Input".as_ptr(), control: core::ptr::null(), source: c"INR".as_ptr(), ..unsafe { core::mem::zeroed() } },

    /* supplies */
    snd_soc_dapm_route { sink: c"Output Mixer".as_ptr(), control: core::ptr::null(), source: c"Charge Pump 1".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"Output Mixer".as_ptr(), control: core::ptr::null(), source: c"Charge Pump 2".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"Output Mixer".as_ptr(), control: core::ptr::null(), source: c"SHDN".as_ptr(), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c"DAC".as_ptr(), control: core::ptr::null(), source: c"MCLK".as_ptr(), ..unsafe { core::mem::zeroed() } },
];

unsafe extern "C" fn max9850_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let max9850: *mut max9850_priv =
        snd_soc_component_get_drvdata(component) as *mut max9850_priv;
    let mut lrclk_div: u64;
    let sf: u8;
    let da: u8;

    if (*max9850).sysclk == 0 {
        return -EINVAL;
    }

    /* lrclk_div = 2^22 * rate / iclk with iclk = mclk / sf */
    sf = ((snd_soc_component_read(component, MAX9850_CLOCK) >> 2) + 1) as u8;
    lrclk_div = 1_u64 << 22;
    lrclk_div = lrclk_div.wrapping_mul(params_rate(params) as u64);
    lrclk_div = lrclk_div.wrapping_mul(sf as u64);
    do_div(&mut lrclk_div, (*max9850).sysclk);

    snd_soc_component_write(
        component,
        MAX9850_LRCLK_MSB,
        ((lrclk_div >> 8) & 0x7f) as c_uint,
    );
    snd_soc_component_write(component, MAX9850_LRCLK_LSB, (lrclk_div & 0xff) as c_uint);

    da = match params_width(params) {
        16 => 0,
        20 => 0x2,
        24 => 0x3,
        _ => return -EINVAL,
    };
    snd_soc_component_update_bits(component, MAX9850_DIGITAL_AUDIO, 0x3, da as c_uint);

    0
}

unsafe extern "C" fn max9850_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let max9850: *mut max9850_priv =
        snd_soc_component_get_drvdata(component) as *mut max9850_priv;

    /* calculate mclk -> iclk divider */
    if freq <= 13000000 {
        snd_soc_component_write(component, MAX9850_CLOCK, 0x0);
    } else if freq <= 26000000 {
        snd_soc_component_write(component, MAX9850_CLOCK, 0x4);
    } else if freq <= 40000000 {
        snd_soc_component_write(component, MAX9850_CLOCK, 0x8);
    } else {
        return -EINVAL;
    }

    (*max9850).sysclk = freq;
    0
}

unsafe extern "C" fn max9850_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let mut da: u8 = 0;

    /* set clock provider for audio interface */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            da |= MAX9850_MASTER as u8;
        }
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            da |= MAX9850_DLY as u8;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            da |= MAX9850_RTJ as u8;
        }
        SND_SOC_DAIFMT_LEFT_J => {}
        _ => return -EINVAL,
    }

    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => {
            da |= (MAX9850_BCINV | MAX9850_INV) as u8;
        }
        SND_SOC_DAIFMT_IB_NF => {
            da |= MAX9850_BCINV as u8;
        }
        SND_SOC_DAIFMT_NB_IF => {
            da |= MAX9850_INV as u8;
        }
        _ => return -EINVAL,
    }

    /* set da */
    snd_soc_component_write(component, MAX9850_DIGITAL_AUDIO, da as c_uint);

    0
}

unsafe extern "C" fn max9850_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let max9850: *mut max9850_priv =
        snd_soc_component_get_drvdata(component) as *mut max9850_priv;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let ret: c_int;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                ret = regcache_sync((*max9850).regmap);
                if ret != 0 {
                    dev_err(
                        (*component).dev,
                        c"Failed to sync cache: %d\n".as_ptr(),
                        ret,
                    );
                    return ret;
                }
            }
        }
        SND_SOC_BIAS_OFF => {}
    }
    0
}

const MAX9850_RATES: c_uint = SNDRV_PCM_RATE_8000_48000;

const MAX9850_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static max9850_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(max9850_hw_params),
    set_sysclk: Some(max9850_set_dai_sysclk),
    set_fmt: Some(max9850_set_dai_fmt),
    ..unsafe { core::mem::zeroed() }
};

static mut max9850_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"max9850-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: MAX9850_RATES,
        formats: MAX9850_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &max9850_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn max9850_probe(component: *mut snd_soc_component) -> c_int {
    /* enable zero-detect */
    snd_soc_component_update_bits(component, MAX9850_GENERAL_PURPOSE, 1, 1);
    /* enable slew-rate control */
    snd_soc_component_update_bits(component, MAX9850_VOLUME, 0x40, 0x40);
    /* set slew-rate 125ms */
    snd_soc_component_update_bits(component, MAX9850_CHARGE_PUMP, 0xff, 0xc0);

    0
}

static soc_component_dev_max9850: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(max9850_probe),
    set_bias_level: Some(max9850_set_bias_level),
    controls: max9850_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(max9850_controls),
    dapm_widgets: max9850_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(max9850_dapm_widgets),
    dapm_routes: max9850_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(max9850_dapm_routes),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn max9850_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let max9850: *mut max9850_priv;
    let ret: c_int;

    max9850 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<max9850_priv>(),
        GFP_KERNEL,
    ) as *mut max9850_priv;
    if max9850.is_null() {
        return -ENOMEM;
    }

    (*max9850).regmap = devm_regmap_init_i2c(i2c, &max9850_regmap);
    if IS_ERR((*max9850).regmap as *const c_void) {
        return PTR_ERR((*max9850).regmap as *const c_void) as c_int;
    }

    i2c_set_clientdata(i2c, max9850 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_max9850,
        &mut max9850_dai,
        1,
    );
    ret
}

static max9850_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: *b"max9850\0",
        ..unsafe { core::mem::zeroed() }
    },
    i2c_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
MODULE_DEVICE_TABLE!(i2c, max9850_i2c_id);

static mut max9850_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"max9850".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(max9850_i2c_probe),
    id_table: max9850_i2c_id.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

module_i2c_driver!(max9850_i2c_driver);

MODULE_AUTHOR!("Christian Glindkamp <christian.glindkamp@taskit.de>");
MODULE_DESCRIPTION!("ASoC MAX9850 codec driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
