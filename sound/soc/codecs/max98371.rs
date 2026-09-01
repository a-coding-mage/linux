// SPDX-License-Identifier: GPL-2.0-only
/*
 * max98371.c -- ALSA SoC Stereo MAX98371 driver
 *
 * Copyright 2015-16 Maxim Integrated Products
 */

// Translated from Linux kernel C. External types, constants, macros, and
// functions are expected to be supplied by the surrounding driver bindings.

static monomix_text: [*const core::ffi::c_char; 3] = [
    c"Left".as_ptr(),
    c"Right".as_ptr(),
    c"LeftRightDiv2".as_ptr(),
];

static hpf_cutoff_txt: [*const core::ffi::c_char; 7] = [
    c"Disable".as_ptr(),
    c"DC Block".as_ptr(),
    c"50Hz".as_ptr(),
    c"100Hz".as_ptr(),
    c"200Hz".as_ptr(),
    c"400Hz".as_ptr(),
    c"800Hz".as_ptr(),
];

SOC_ENUM_SINGLE_DECL!(max98371_monomix, MAX98371_MONOMIX_CFG, 0, monomix_text);

SOC_ENUM_SINGLE_DECL!(max98371_hpf_cutoff, MAX98371_HPF, 0, hpf_cutoff_txt);

static max98371_dht_min_gain: &[u32] = &DECLARE_TLV_DB_RANGE![
    0, 1, TLV_DB_SCALE_ITEM!(537, 66, 0),
    2, 3, TLV_DB_SCALE_ITEM!(677, 82, 0),
    4, 5, TLV_DB_SCALE_ITEM!(852, 104, 0),
    6, 7, TLV_DB_SCALE_ITEM!(1072, 131, 0),
    8, 9, TLV_DB_SCALE_ITEM!(1350, 165, 0),
    10, 11, TLV_DB_SCALE_ITEM!(1699, 101, 0),
];

static max98371_dht_max_gain: &[u32] = &DECLARE_TLV_DB_RANGE![
    0, 1, TLV_DB_SCALE_ITEM!(537, 66, 0),
    2, 3, TLV_DB_SCALE_ITEM!(677, 82, 0),
    4, 5, TLV_DB_SCALE_ITEM!(852, 104, 0),
    6, 7, TLV_DB_SCALE_ITEM!(1072, 131, 0),
    8, 9, TLV_DB_SCALE_ITEM!(1350, 165, 0),
    10, 11, TLV_DB_SCALE_ITEM!(1699, 208, 0),
];

static max98371_dht_rot_gain: &[u32] = &DECLARE_TLV_DB_RANGE![
    0, 1, TLV_DB_SCALE_ITEM!(-50, -50, 0),
    2, 6, TLV_DB_SCALE_ITEM!(-100, -100, 0),
    7, 8, TLV_DB_SCALE_ITEM!(-800, -200, 0),
    9, 11, TLV_DB_SCALE_ITEM!(-1200, -300, 0),
    12, 13, TLV_DB_SCALE_ITEM!(-2000, -200, 0),
    14, 15, TLV_DB_SCALE_ITEM!(-2500, -500, 0),
];

static max98371_reg: [reg_default; 67] = [
    reg_default { reg: 0x01, def: 0x00 },
    reg_default { reg: 0x02, def: 0x00 },
    reg_default { reg: 0x03, def: 0x00 },
    reg_default { reg: 0x04, def: 0x00 },
    reg_default { reg: 0x05, def: 0x00 },
    reg_default { reg: 0x06, def: 0x00 },
    reg_default { reg: 0x07, def: 0x00 },
    reg_default { reg: 0x08, def: 0x00 },
    reg_default { reg: 0x09, def: 0x00 },
    reg_default { reg: 0x0A, def: 0x00 },
    reg_default { reg: 0x10, def: 0x06 },
    reg_default { reg: 0x11, def: 0x08 },
    reg_default { reg: 0x14, def: 0x80 },
    reg_default { reg: 0x15, def: 0x00 },
    reg_default { reg: 0x16, def: 0x00 },
    reg_default { reg: 0x18, def: 0x00 },
    reg_default { reg: 0x19, def: 0x00 },
    reg_default { reg: 0x1C, def: 0x00 },
    reg_default { reg: 0x1D, def: 0x00 },
    reg_default { reg: 0x1E, def: 0x00 },
    reg_default { reg: 0x1F, def: 0x00 },
    reg_default { reg: 0x20, def: 0x00 },
    reg_default { reg: 0x21, def: 0x00 },
    reg_default { reg: 0x22, def: 0x00 },
    reg_default { reg: 0x23, def: 0x00 },
    reg_default { reg: 0x24, def: 0x00 },
    reg_default { reg: 0x25, def: 0x00 },
    reg_default { reg: 0x26, def: 0x00 },
    reg_default { reg: 0x27, def: 0x00 },
    reg_default { reg: 0x28, def: 0x00 },
    reg_default { reg: 0x29, def: 0x00 },
    reg_default { reg: 0x2A, def: 0x00 },
    reg_default { reg: 0x2B, def: 0x00 },
    reg_default { reg: 0x2C, def: 0x00 },
    reg_default { reg: 0x2D, def: 0x00 },
    reg_default { reg: 0x2E, def: 0x0B },
    reg_default { reg: 0x31, def: 0x00 },
    reg_default { reg: 0x32, def: 0x18 },
    reg_default { reg: 0x33, def: 0x00 },
    reg_default { reg: 0x34, def: 0x00 },
    reg_default { reg: 0x36, def: 0x00 },
    reg_default { reg: 0x37, def: 0x00 },
    reg_default { reg: 0x38, def: 0x00 },
    reg_default { reg: 0x39, def: 0x00 },
    reg_default { reg: 0x3A, def: 0x00 },
    reg_default { reg: 0x3B, def: 0x00 },
    reg_default { reg: 0x3C, def: 0x00 },
    reg_default { reg: 0x3D, def: 0x00 },
    reg_default { reg: 0x3E, def: 0x00 },
    reg_default { reg: 0x3F, def: 0x00 },
    reg_default { reg: 0x40, def: 0x00 },
    reg_default { reg: 0x41, def: 0x00 },
    reg_default { reg: 0x42, def: 0x00 },
    reg_default { reg: 0x43, def: 0x00 },
    reg_default { reg: 0x4A, def: 0x00 },
    reg_default { reg: 0x4B, def: 0x00 },
    reg_default { reg: 0x4C, def: 0x00 },
    reg_default { reg: 0x4D, def: 0x00 },
    reg_default { reg: 0x4E, def: 0x00 },
    reg_default { reg: 0x50, def: 0x00 },
    reg_default { reg: 0x51, def: 0x00 },
    reg_default { reg: 0x55, def: 0x00 },
    reg_default { reg: 0x58, def: 0x00 },
    reg_default { reg: 0x59, def: 0x00 },
    reg_default { reg: 0x5C, def: 0x00 },
    reg_default { reg: 0xFF, def: 0x43 },
];

unsafe extern "C" fn max98371_volatile_register(
    dev: *mut device,
    reg: core::ffi::c_uint,
) -> bool {
    match reg {
        MAX98371_IRQ_CLEAR1 | MAX98371_IRQ_CLEAR2 | MAX98371_IRQ_CLEAR3 | MAX98371_VERSION => true,
        _ => false,
    }
}

unsafe extern "C" fn max98371_readable_register(
    dev: *mut device,
    reg: core::ffi::c_uint,
) -> bool {
    match reg {
        MAX98371_SOFT_RESET => false,
        _ => true,
    }
}

static max98371_gain_tlv: &[u32] = &DECLARE_TLV_DB_RANGE![
    0, 7, TLV_DB_SCALE_ITEM!(0, 50, 0),
    8, 10, TLV_DB_SCALE_ITEM!(400, 100, 0),
];

static digital_tlv: &[u32] = &DECLARE_TLV_DB_SCALE!(-6300, 50, 1);

static max98371_snd_controls: [snd_kcontrol_new; 9] = [
    SOC_SINGLE_TLV!(
        c"Speaker Volume",
        MAX98371_GAIN,
        MAX98371_GAIN_SHIFT,
        (1 << MAX98371_GAIN_WIDTH) - 1,
        0,
        max98371_gain_tlv
    ),
    SOC_SINGLE_TLV!(
        c"Digital Volume",
        MAX98371_DIGITAL_GAIN,
        0,
        (1 << MAX98371_DIGITAL_GAIN_WIDTH) - 1,
        1,
        digital_tlv
    ),
    SOC_SINGLE_TLV!(
        c"Speaker DHT Max Volume",
        MAX98371_GAIN,
        0,
        (1 << MAX98371_DHT_MAX_WIDTH) - 1,
        0,
        max98371_dht_max_gain
    ),
    SOC_SINGLE_TLV!(
        c"Speaker DHT Min Volume",
        MAX98371_DHT_GAIN,
        0,
        (1 << MAX98371_DHT_GAIN_WIDTH) - 1,
        0,
        max98371_dht_min_gain
    ),
    SOC_SINGLE_TLV!(
        c"Speaker DHT Rotation Volume",
        MAX98371_DHT_GAIN,
        0,
        (1 << MAX98371_DHT_ROT_WIDTH) - 1,
        0,
        max98371_dht_rot_gain
    ),
    SOC_SINGLE!(c"DHT Attack Step", MAX98371_DHT, MAX98371_DHT_STEP, 3, 0),
    SOC_SINGLE!(c"DHT Attack Rate", MAX98371_DHT, 0, 7, 0),
    SOC_ENUM!(c"Monomix Select", max98371_monomix),
    SOC_ENUM!(c"HPF Cutoff", max98371_hpf_cutoff),
];

unsafe extern "C" fn max98371_dai_set_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: core::ffi::c_uint,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let max98371: *mut max98371_priv = snd_soc_component_get_drvdata(component) as *mut max98371_priv;
    let mut val: core::ffi::c_uint = 0;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => {
            dev_err((*component).dev, c"DAI clock mode unsupported".as_ptr());
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            val |= 0;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            val |= MAX98371_DAI_RIGHT;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            val |= MAX98371_DAI_LEFT;
        }
        _ => {
            dev_err((*component).dev, c"DAI wrong mode unsupported".as_ptr());
            return -EINVAL;
        }
    }
    regmap_update_bits(
        (*max98371).regmap,
        MAX98371_FMT,
        MAX98371_FMT_MODE_MASK,
        val,
    );
    0
}

unsafe extern "C" fn max98371_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let max98371: *mut max98371_priv = snd_soc_component_get_drvdata(component) as *mut max98371_priv;
    let mut ch_size: core::ffi::c_int;
    let channels: core::ffi::c_int = params_channels(params);
    let rate: core::ffi::c_int = params_rate(params);

    match params_format(params) {
        SNDRV_PCM_FORMAT_S8 => {
            regmap_update_bits(
                (*max98371).regmap,
                MAX98371_FMT,
                MAX98371_FMT_MASK,
                MAX98371_DAI_CHANSZ_16,
            );
            ch_size = 8;
        }
        SNDRV_PCM_FORMAT_S16_LE => {
            regmap_update_bits(
                (*max98371).regmap,
                MAX98371_FMT,
                MAX98371_FMT_MASK,
                MAX98371_DAI_CHANSZ_16,
            );
            ch_size = 16;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            regmap_update_bits(
                (*max98371).regmap,
                MAX98371_FMT,
                MAX98371_FMT_MASK,
                MAX98371_DAI_CHANSZ_32,
            );
            ch_size = 24;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            regmap_update_bits(
                (*max98371).regmap,
                MAX98371_FMT,
                MAX98371_FMT_MASK,
                MAX98371_DAI_CHANSZ_32,
            );
            ch_size = 32;
        }
        _ => return -EINVAL,
    }

    /* BCLK/LRCLK ratio calculation */
    let blr_clk_ratio: core::ffi::c_int = channels * ch_size;
    match blr_clk_ratio {
        32 => {
            regmap_update_bits(
                (*max98371).regmap,
                MAX98371_DAI_CLK,
                MAX98371_DAI_BSEL_MASK,
                MAX98371_DAI_BSEL_32,
            );
        }
        48 => {
            regmap_update_bits(
                (*max98371).regmap,
                MAX98371_DAI_CLK,
                MAX98371_DAI_BSEL_MASK,
                MAX98371_DAI_BSEL_48,
            );
        }
        64 => {
            regmap_update_bits(
                (*max98371).regmap,
                MAX98371_DAI_CLK,
                MAX98371_DAI_BSEL_MASK,
                MAX98371_DAI_BSEL_64,
            );
        }
        _ => return -EINVAL,
    }

    match rate {
        32000 => {
            regmap_update_bits(
                (*max98371).regmap,
                MAX98371_SPK_SR,
                MAX98371_SPK_SR_MASK,
                MAX98371_SPK_SR_32,
            );
        }
        44100 => {
            regmap_update_bits(
                (*max98371).regmap,
                MAX98371_SPK_SR,
                MAX98371_SPK_SR_MASK,
                MAX98371_SPK_SR_44,
            );
        }
        48000 => {
            regmap_update_bits(
                (*max98371).regmap,
                MAX98371_SPK_SR,
                MAX98371_SPK_SR_MASK,
                MAX98371_SPK_SR_48,
            );
        }
        88200 => {
            regmap_update_bits(
                (*max98371).regmap,
                MAX98371_SPK_SR,
                MAX98371_SPK_SR_MASK,
                MAX98371_SPK_SR_88,
            );
        }
        96000 => {
            regmap_update_bits(
                (*max98371).regmap,
                MAX98371_SPK_SR,
                MAX98371_SPK_SR_MASK,
                MAX98371_SPK_SR_96,
            );
        }
        _ => return -EINVAL,
    }

    /* enabling both the RX channels*/
    regmap_update_bits(
        (*max98371).regmap,
        MAX98371_MONOMIX_SRC,
        MAX98371_MONOMIX_SRC_MASK,
        MONOMIX_RX_0_1,
    );
    regmap_update_bits(
        (*max98371).regmap,
        MAX98371_DAI_CHANNEL,
        MAX98371_CHANNEL_MASK,
        MAX98371_CHANNEL_MASK,
    );
    0
}

static max98371_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    SND_SOC_DAPM_DAC!(c"DAC", core::ptr::null(), MAX98371_SPK_ENABLE, 0, 0),
    SND_SOC_DAPM_SUPPLY!(
        c"Global Enable",
        MAX98371_GLOBAL_ENABLE,
        0,
        0,
        core::ptr::null(),
        0
    ),
    SND_SOC_DAPM_OUTPUT!(c"SPK_OUT"),
];

static max98371_audio_map: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route {
        sink: c"DAC".as_ptr(),
        control: core::ptr::null(),
        source: c"HiFi Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"SPK_OUT".as_ptr(),
        control: core::ptr::null(),
        source: c"DAC".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"SPK_OUT".as_ptr(),
        control: core::ptr::null(),
        source: c"Global Enable".as_ptr(),
    },
];

const MAX98371_RATES: core::ffi::c_uint = SNDRV_PCM_RATE_8000_48000;
const MAX98371_FORMATS: core::ffi::c_uint =
    SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_BE | SNDRV_PCM_FMTBIT_S32_BE;

static max98371_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(max98371_dai_set_fmt),
    hw_params: Some(max98371_dai_hw_params),
};

static mut max98371_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"max98371-aif1".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"HiFi Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: MAX98371_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &max98371_dai_ops,
    ..unsafe { core::mem::zeroed() }
}];

static max98371_component: snd_soc_component_driver = snd_soc_component_driver {
    controls: max98371_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(max98371_snd_controls),
    dapm_routes: max98371_audio_map.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(max98371_audio_map),
    dapm_widgets: max98371_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(max98371_dapm_widgets),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

static max98371_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: MAX98371_VERSION,
    reg_defaults: max98371_reg.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(max98371_reg),
    volatile_reg: Some(max98371_volatile_register),
    readable_reg: Some(max98371_readable_register),
    cache_type: REGCACHE_RBTREE,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn max98371_i2c_probe(i2c: *mut i2c_client) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int;
    let mut reg: core::ffi::c_int = 0;

    let max98371: *mut max98371_priv = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<max98371_priv>(),
        GFP_KERNEL,
    ) as *mut max98371_priv;
    if max98371.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, max98371 as *mut core::ffi::c_void);
    (*max98371).regmap = devm_regmap_init_i2c(i2c, &max98371_regmap);
    if IS_ERR((*max98371).regmap) {
        ret = PTR_ERR((*max98371).regmap) as core::ffi::c_int;
        dev_err(
            &mut (*i2c).dev,
            c"Failed to allocate regmap: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = regmap_read((*max98371).regmap, MAX98371_VERSION, &mut reg);
    if ret < 0 {
        dev_info(&mut (*i2c).dev, c"device error %d\n".as_ptr(), ret);
        return ret;
    }
    dev_info(&mut (*i2c).dev, c"device version %x\n".as_ptr(), reg);

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &max98371_component,
        max98371_dai.as_mut_ptr(),
        ARRAY_SIZE!(max98371_dai),
    );
    if ret < 0 {
        dev_err(
            &mut (*i2c).dev,
            c"Failed to register component: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }
    ret
}

static max98371_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: *b"max98371\0",
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

MODULE_DEVICE_TABLE!(i2c, max98371_i2c_id);

// Original C condition: #ifdef CONFIG_OF
#[cfg(CONFIG_OF)]
static max98371_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"maxim,max98371".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
#[cfg(CONFIG_OF)]
MODULE_DEVICE_TABLE!(of, max98371_of_match);

static mut max98371_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"max98371".as_ptr(),
        of_match_table: of_match_ptr!(max98371_of_match),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(max98371_i2c_probe),
    id_table: max98371_i2c_id.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

module_i2c_driver!(max98371_i2c_driver);

MODULE_AUTHOR!(c"anish kumar <yesanishhere@gmail.com>");
MODULE_DESCRIPTION!(c"ALSA SoC MAX98371 driver");
MODULE_LICENSE!(c"GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
