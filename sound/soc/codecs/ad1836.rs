// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Audio Codec driver supporting:
 *  AD1835A, AD1836, AD1837A, AD1838A, AD1839A
 *
 * Copyright 2009-2011 Analog Devices Inc.
 */

/* Dependencies from Linux, ALSA SoC, SPI, regmap, and "ad1836.h" are external. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad1836_type {
    AD1835,
    AD1836,
    AD1838,
}

/* codec private data */
#[repr(C)]
pub struct ad1836_priv {
    pub type_: ad1836_type,
    pub regmap: *mut regmap,
}

/*
 * AD1836 volume/mute/de-emphasis etc. controls
 */
static ad1836_deemp: [*const i8; 4] = [
    c"None".as_ptr(),
    c"44.1kHz".as_ptr(),
    c"32kHz".as_ptr(),
    c"48kHz".as_ptr(),
];

SOC_ENUM_SINGLE_DECL!(
    ad1836_deemp_enum,
    AD1836_DAC_CTRL1,
    8,
    ad1836_deemp
);

macro_rules! AD1836_DAC_VOLUME {
    ($x:expr) => {
        SOC_DOUBLE_R!(
            concat!("DAC", stringify!($x), " Playback Volume"),
            AD1836_DAC_L_VOL!($x),
            AD1836_DAC_R_VOL!($x),
            0,
            0x3ff,
            0
        )
    };
}

macro_rules! AD1836_DAC_SWITCH {
    ($x:expr) => {
        SOC_DOUBLE!(
            concat!("DAC", stringify!($x), " Playback Switch"),
            AD1836_DAC_CTRL2,
            AD1836_MUTE_LEFT!($x),
            AD1836_MUTE_RIGHT!($x),
            1,
            1
        )
    };
}

macro_rules! AD1836_ADC_SWITCH {
    ($x:expr) => {
        SOC_DOUBLE!(
            concat!("ADC", stringify!($x), " Capture Switch"),
            AD1836_ADC_CTRL2,
            AD1836_MUTE_LEFT!($x),
            AD1836_MUTE_RIGHT!($x),
            1,
            1
        )
    };
}

static ad183x_dac_controls: [snd_kcontrol_new; 8] = [
    AD1836_DAC_VOLUME!(1),
    AD1836_DAC_SWITCH!(1),
    AD1836_DAC_VOLUME!(2),
    AD1836_DAC_SWITCH!(2),
    AD1836_DAC_VOLUME!(3),
    AD1836_DAC_SWITCH!(3),
    AD1836_DAC_VOLUME!(4),
    AD1836_DAC_SWITCH!(4),
];

static ad183x_dac_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_OUTPUT!(c"DAC1OUT"),
    SND_SOC_DAPM_OUTPUT!(c"DAC2OUT"),
    SND_SOC_DAPM_OUTPUT!(c"DAC3OUT"),
    SND_SOC_DAPM_OUTPUT!(c"DAC4OUT"),
];

static ad183x_dac_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: c"DAC1OUT".as_ptr(), control: core::ptr::null(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC2OUT".as_ptr(), control: core::ptr::null(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC3OUT".as_ptr(), control: core::ptr::null(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC4OUT".as_ptr(), control: core::ptr::null(), source: c"DAC".as_ptr() },
];

static ad183x_adc_controls: [snd_kcontrol_new; 3] = [
    AD1836_ADC_SWITCH!(1),
    AD1836_ADC_SWITCH!(2),
    AD1836_ADC_SWITCH!(3),
];

static ad183x_adc_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_INPUT!(c"ADC1IN"),
    SND_SOC_DAPM_INPUT!(c"ADC2IN"),
];

static ad183x_adc_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"ADC1IN".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"ADC2IN".as_ptr() },
];

static ad183x_controls: [snd_kcontrol_new; 2] = [
    /* ADC high-pass filter */
    SOC_SINGLE!(
        c"ADC High Pass Filter Switch",
        AD1836_ADC_CTRL1,
        AD1836_ADC_HIGHPASS_FILTER,
        1,
        0
    ),

    /* DAC de-emphasis */
    SOC_ENUM!(c"Playback Deemphasis", ad1836_deemp_enum),
];

static ad183x_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    SND_SOC_DAPM_DAC!(
        c"DAC",
        c"Playback",
        AD1836_DAC_CTRL1,
        AD1836_DAC_POWERDOWN,
        1
    ),
    SND_SOC_DAPM_ADC!(c"ADC", c"Capture", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_SUPPLY!(
        c"ADC_PWR",
        AD1836_ADC_CTRL1,
        AD1836_ADC_POWERDOWN,
        1,
        core::ptr::null(),
        0
    ),
];

static ad183x_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: c"DAC".as_ptr(), control: core::ptr::null(), source: c"ADC_PWR".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"ADC_PWR".as_ptr() },
];

DECLARE_TLV_DB_SCALE!(ad1836_in_tlv, 0, 300, 0);

static ad1836_controls: [snd_kcontrol_new; 1] = [
    SOC_DOUBLE_TLV!(
        c"ADC2 Capture Volume",
        AD1836_ADC_CTRL1,
        3,
        0,
        4,
        0,
        ad1836_in_tlv
    ),
];

/*
 * DAI ops entries
 */

unsafe extern "C" fn ad1836_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: core::ffi::c_uint,
) -> core::ffi::c_int {
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        /*
         * at present, we support adc aux mode to interface with
         * blackfin sport tdm mode
         */
        SND_SOC_DAIFMT_DSP_A => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_IB_IF => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        /* ALCLK,ABCLK are both output, AD1836 can only be provider */
        SND_SOC_DAIFMT_CBP_CFP => {}
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn ad1836_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let ad1836: *mut ad1836_priv = snd_soc_component_get_drvdata((*dai).component) as *mut ad1836_priv;
    let mut word_len: core::ffi::c_int = 0;

    /* bit size */
    match params_width(params) {
        16 => word_len = AD1836_WORD_LEN_16,
        20 => word_len = AD1836_WORD_LEN_20,
        24 | 32 => word_len = AD1836_WORD_LEN_24,
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*ad1836).regmap,
        AD1836_DAC_CTRL1,
        AD1836_DAC_WORD_LEN_MASK,
        word_len << AD1836_DAC_WORD_LEN_OFFSET,
    );

    regmap_update_bits(
        (*ad1836).regmap,
        AD1836_ADC_CTRL2,
        AD1836_ADC_WORD_LEN_MASK,
        word_len << AD1836_ADC_WORD_OFFSET,
    );

    0
}

static ad1836_selectable_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_DSP_A | SND_SOC_POSSIBLE_DAIFMT_IB_IF;

static ad1836_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(ad1836_hw_params),
    set_fmt: Some(ad1836_set_dai_fmt),
    auto_selectable_formats: &ad1836_selectable_formats,
    num_auto_selectable_formats: 1,
};

macro_rules! AD183X_DAI {
    ($_name:expr, $num_dacs:expr, $num_adcs:expr) => {
        snd_soc_dai_driver {
            name: concat!($_name, "-hifi").as_ptr(),
            playback: snd_soc_pcm_stream {
                stream_name: c"Playback".as_ptr(),
                channels_min: 2,
                channels_max: ($num_dacs) * 2,
                rates: SNDRV_PCM_RATE_48000,
                formats: SNDRV_PCM_FMTBIT_S32_LE
                    | SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S20_3LE
                    | SNDRV_PCM_FMTBIT_S24_LE,
            },
            capture: snd_soc_pcm_stream {
                stream_name: c"Capture".as_ptr(),
                channels_min: 2,
                channels_max: ($num_adcs) * 2,
                rates: SNDRV_PCM_RATE_48000,
                formats: SNDRV_PCM_FMTBIT_S32_LE
                    | SNDRV_PCM_FMTBIT_S16_LE
                    | SNDRV_PCM_FMTBIT_S20_3LE
                    | SNDRV_PCM_FMTBIT_S24_LE,
            },
            ops: &ad1836_dai_ops,
        }
    };
}

static mut ad183x_dais: [snd_soc_dai_driver; 3] = [
    AD183X_DAI!("ad1835", 4, 1),
    AD183X_DAI!("ad1836", 3, 2),
    AD183X_DAI!("ad1838", 3, 1),
];

/* CONFIG_PM conditional code from C is preserved with cfg(CONFIG_PM). */
#[cfg(CONFIG_PM)]
unsafe extern "C" fn ad1836_suspend(component: *mut snd_soc_component) -> core::ffi::c_int {
    let ad1836: *mut ad1836_priv = snd_soc_component_get_drvdata(component) as *mut ad1836_priv;
    /* reset clock control mode */
    regmap_update_bits(
        (*ad1836).regmap,
        AD1836_ADC_CTRL2,
        AD1836_ADC_SERFMT_MASK,
        0,
    )
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn ad1836_resume(component: *mut snd_soc_component) -> core::ffi::c_int {
    let ad1836: *mut ad1836_priv = snd_soc_component_get_drvdata(component) as *mut ad1836_priv;
    /* restore clock control mode */
    regmap_update_bits(
        (*ad1836).regmap,
        AD1836_ADC_CTRL2,
        AD1836_ADC_SERFMT_MASK,
        AD1836_ADC_AUX,
    )
}

#[cfg(not(CONFIG_PM))]
const ad1836_suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> core::ffi::c_int> = None;
#[cfg(not(CONFIG_PM))]
const ad1836_resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> core::ffi::c_int> = None;

unsafe extern "C" fn ad1836_probe(component: *mut snd_soc_component) -> core::ffi::c_int {
    let ad1836: *mut ad1836_priv = snd_soc_component_get_drvdata(component) as *mut ad1836_priv;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let num_dacs: core::ffi::c_int;
    let num_adcs: core::ffi::c_int;
    let mut ret: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int;

    num_dacs = ad183x_dais[(*ad1836).type_ as usize].playback.channels_max / 2;
    num_adcs = ad183x_dais[(*ad1836).type_ as usize].capture.channels_max / 2;

    /* default setting for ad1836 */
    /* de-emphasis: 48kHz, power-on dac */
    regmap_write((*ad1836).regmap, AD1836_DAC_CTRL1, 0x300);
    /* unmute dac channels */
    regmap_write((*ad1836).regmap, AD1836_DAC_CTRL2, 0x0);
    /* high-pass filter enable, power-on adc */
    regmap_write((*ad1836).regmap, AD1836_ADC_CTRL1, 0x100);
    /* unmute adc channles, adc aux mode */
    regmap_write((*ad1836).regmap, AD1836_ADC_CTRL2, 0x180);
    /* volume */
    i = 1;
    while i <= num_dacs {
        regmap_write((*ad1836).regmap, AD1836_DAC_L_VOL!(i), 0x3ff);
        regmap_write((*ad1836).regmap, AD1836_DAC_R_VOL!(i), 0x3ff);
        i += 1;
    }

    if (*ad1836).type_ == ad1836_type::AD1836 {
        /* left/right diff:PGA/MUX */
        regmap_write((*ad1836).regmap, AD1836_ADC_CTRL3, 0x3a);
        ret = snd_soc_add_component_controls(
            component,
            ad1836_controls.as_ptr(),
            ARRAY_SIZE!(ad1836_controls),
        );
        if ret != 0 {
            return ret;
        }
    } else {
        regmap_write((*ad1836).regmap, AD1836_ADC_CTRL3, 0x00);
    }

    ret = snd_soc_add_component_controls(component, ad183x_dac_controls.as_ptr(), num_dacs * 2);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_add_component_controls(component, ad183x_adc_controls.as_ptr(), num_adcs);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_dapm_new_controls(dapm, ad183x_dac_dapm_widgets.as_ptr(), num_dacs);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_dapm_new_controls(dapm, ad183x_adc_dapm_widgets.as_ptr(), num_adcs);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, ad183x_dac_routes.as_ptr(), num_dacs);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, ad183x_adc_routes.as_ptr(), num_adcs);

    ret
}

/* power down chip */
unsafe extern "C" fn ad1836_remove(component: *mut snd_soc_component) {
    let ad1836: *mut ad1836_priv = snd_soc_component_get_drvdata(component) as *mut ad1836_priv;
    /* reset clock control mode */
    regmap_update_bits(
        (*ad1836).regmap,
        AD1836_ADC_CTRL2,
        AD1836_ADC_SERFMT_MASK,
        0,
    );
}

static soc_component_dev_ad1836: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ad1836_probe),
    remove: Some(ad1836_remove),
    suspend: Some(ad1836_suspend),
    resume: Some(ad1836_resume),
    controls: ad183x_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(ad183x_controls),
    dapm_widgets: ad183x_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(ad183x_dapm_widgets),
    dapm_routes: ad183x_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(ad183x_dapm_routes),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static ad1836_reg_defaults: [reg_default; 13] = [
    reg_default { reg: AD1836_DAC_CTRL1, def: 0x0000 },
    reg_default { reg: AD1836_DAC_CTRL2, def: 0x0000 },
    reg_default { reg: AD1836_DAC_L_VOL!(0), def: 0x0000 },
    reg_default { reg: AD1836_DAC_R_VOL!(0), def: 0x0000 },
    reg_default { reg: AD1836_DAC_L_VOL!(1), def: 0x0000 },
    reg_default { reg: AD1836_DAC_R_VOL!(1), def: 0x0000 },
    reg_default { reg: AD1836_DAC_L_VOL!(2), def: 0x0000 },
    reg_default { reg: AD1836_DAC_R_VOL!(2), def: 0x0000 },
    reg_default { reg: AD1836_DAC_L_VOL!(3), def: 0x0000 },
    reg_default { reg: AD1836_DAC_R_VOL!(3), def: 0x0000 },
    reg_default { reg: AD1836_ADC_CTRL1, def: 0x0000 },
    reg_default { reg: AD1836_ADC_CTRL2, def: 0x0000 },
    reg_default { reg: AD1836_ADC_CTRL3, def: 0x0000 },
];

static ad1836_regmap_config: regmap_config = regmap_config {
    val_bits: 12,
    reg_bits: 4,
    read_flag_mask: 0x08,

    max_register: AD1836_ADC_CTRL3,
    reg_defaults: ad1836_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(ad1836_reg_defaults),
    cache_type: REGCACHE_MAPLE,
};

unsafe extern "C" fn ad1836_spi_probe(spi: *mut spi_device) -> core::ffi::c_int {
    let ad1836: *mut ad1836_priv;
    let ret: core::ffi::c_int;

    ad1836 = devm_kzalloc(
        &mut (*spi).dev,
        core::mem::size_of::<ad1836_priv>(),
        GFP_KERNEL,
    ) as *mut ad1836_priv;
    if ad1836.is_null() {
        return -ENOMEM;
    }

    (*ad1836).regmap = devm_regmap_init_spi(spi, &ad1836_regmap_config);
    if IS_ERR((*ad1836).regmap) {
        return PTR_ERR((*ad1836).regmap);
    }

    (*ad1836).type_ = (*spi_get_device_id(spi)).driver_data as ad1836_type;

    spi_set_drvdata(spi, ad1836 as *mut core::ffi::c_void);

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &soc_component_dev_ad1836,
        &mut ad183x_dais[(*ad1836).type_ as usize],
        1,
    );
    ret
}

static ad1836_ids: [spi_device_id; 6] = [
    spi_device_id { name: *b"ad1835\0", driver_data: ad1836_type::AD1835 as _ },
    spi_device_id { name: *b"ad1836\0", driver_data: ad1836_type::AD1836 as _ },
    spi_device_id { name: *b"ad1837\0", driver_data: ad1836_type::AD1835 as _ },
    spi_device_id { name: *b"ad1838\0", driver_data: ad1836_type::AD1838 as _ },
    spi_device_id { name: *b"ad1839\0", driver_data: ad1836_type::AD1838 as _ },
    spi_device_id::default(),
];
MODULE_DEVICE_TABLE!(spi, ad1836_ids);

static mut ad1836_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c"ad1836".as_ptr(),
    },
    probe: Some(ad1836_spi_probe),
    id_table: ad1836_ids.as_ptr(),
};

module_spi_driver!(ad1836_spi_driver);

MODULE_DESCRIPTION!(c"ASoC ad1836 driver");
MODULE_AUTHOR!(c"Barry Song <21cnbao@gmail.com>");
MODULE_LICENSE!(c"GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
