// SPDX-License-Identifier: GPL-2.0-only
/*
 * ak4535.c  --  AK4535 ALSA Soc Audio driver
 *
 * Copyright 2005 Openedhand Ltd.
 *
 * Author: Richard Purdie <richard@openedhand.com>
 *
 * Based on wm8753.c by Liam Girdwood
 */

// C dependency intent:
// linux/module.h, linux/moduleparam.h, linux/init.h, linux/delay.h,
// linux/pm.h, linux/i2c.h, linux/regmap.h, linux/slab.h,
// sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// sound/initval.h, and "ak4535.h".

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct ak4535_priv {
    pub regmap: *mut regmap,
    pub sysclk: c_uint,
}

/*
 * ak4535 register cache
 */
static ak4535_reg_defaults: [reg_default; 15] = [
    reg_default { reg: 0, def: 0x00 },
    reg_default { reg: 1, def: 0x80 },
    reg_default { reg: 2, def: 0x00 },
    reg_default { reg: 3, def: 0x03 },
    reg_default { reg: 4, def: 0x02 },
    reg_default { reg: 5, def: 0x00 },
    reg_default { reg: 6, def: 0x11 },
    reg_default { reg: 7, def: 0x01 },
    reg_default { reg: 8, def: 0x00 },
    reg_default { reg: 9, def: 0x40 },
    reg_default { reg: 10, def: 0x36 },
    reg_default { reg: 11, def: 0x10 },
    reg_default { reg: 12, def: 0x00 },
    reg_default { reg: 13, def: 0x00 },
    reg_default { reg: 14, def: 0x57 },
];

unsafe extern "C" fn ak4535_volatile(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        AK4535_STATUS => true,
        _ => false,
    }
}

static ak4535_mono_gain: [*const c_char; 2] = [c"+6dB".as_ptr(), c"-17dB".as_ptr()];
static ak4535_mono_out: [*const c_char; 2] = [c"(L + R)/2".as_ptr(), c"Hi-Z".as_ptr()];
static ak4535_hp_out: [*const c_char; 2] = [c"Stereo".as_ptr(), c"Mono".as_ptr()];
static ak4535_deemp: [*const c_char; 4] = [
    c"44.1kHz".as_ptr(),
    c"Off".as_ptr(),
    c"48kHz".as_ptr(),
    c"32kHz".as_ptr(),
];
static ak4535_mic_select: [*const c_char; 2] = [c"Internal".as_ptr(), c"External".as_ptr()];

static ak4535_enum: [soc_enum; 5] = [
    SOC_ENUM_SINGLE!(AK4535_SIG1, 7, 2, ak4535_mono_gain),
    SOC_ENUM_SINGLE!(AK4535_SIG1, 6, 2, ak4535_mono_out),
    SOC_ENUM_SINGLE!(AK4535_MODE2, 2, 2, ak4535_hp_out),
    SOC_ENUM_SINGLE!(AK4535_DAC, 0, 4, ak4535_deemp),
    SOC_ENUM_SINGLE!(AK4535_MIC, 1, 2, ak4535_mic_select),
];

static ak4535_snd_controls: [snd_kcontrol_new; 19] = [
    SOC_SINGLE!(c"ALC2 Switch".as_ptr(), AK4535_SIG1, 1, 1, 0),
    SOC_ENUM!(c"Mono 1 Output".as_ptr(), ak4535_enum[1]),
    SOC_ENUM!(c"Mono 1 Gain".as_ptr(), ak4535_enum[0]),
    SOC_ENUM!(c"Headphone Output".as_ptr(), ak4535_enum[2]),
    SOC_ENUM!(c"Playback Deemphasis".as_ptr(), ak4535_enum[3]),
    SOC_SINGLE!(c"Bass Volume".as_ptr(), AK4535_DAC, 2, 3, 0),
    SOC_SINGLE!(c"Mic Boost (+20dB) Switch".as_ptr(), AK4535_MIC, 0, 1, 0),
    SOC_ENUM!(c"Mic Select".as_ptr(), ak4535_enum[4]),
    SOC_SINGLE!(c"ALC Operation Time".as_ptr(), AK4535_TIMER, 0, 3, 0),
    SOC_SINGLE!(c"ALC Recovery Time".as_ptr(), AK4535_TIMER, 2, 3, 0),
    SOC_SINGLE!(c"ALC ZC Time".as_ptr(), AK4535_TIMER, 4, 3, 0),
    SOC_SINGLE!(c"ALC 1 Switch".as_ptr(), AK4535_ALC1, 5, 1, 0),
    SOC_SINGLE!(c"ALC 2 Switch".as_ptr(), AK4535_ALC1, 6, 1, 0),
    SOC_SINGLE!(c"ALC Volume".as_ptr(), AK4535_ALC2, 0, 127, 0),
    SOC_SINGLE!(c"Capture Volume".as_ptr(), AK4535_PGA, 0, 127, 0),
    SOC_SINGLE!(c"Left Playback Volume".as_ptr(), AK4535_LATT, 0, 127, 1),
    SOC_SINGLE!(c"Right Playback Volume".as_ptr(), AK4535_RATT, 0, 127, 1),
    SOC_SINGLE!(c"AUX Bypass Volume".as_ptr(), AK4535_VOL, 0, 15, 0),
    SOC_SINGLE!(c"Mic Sidetone Volume".as_ptr(), AK4535_VOL, 4, 7, 0),
];

/* Mono 1 Mixer */
static ak4535_mono1_mixer_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE!(c"Mic Sidetone Switch".as_ptr(), AK4535_SIG1, 4, 1, 0),
    SOC_DAPM_SINGLE!(c"Mono Playback Switch".as_ptr(), AK4535_SIG1, 5, 1, 0),
];

/* Stereo Mixer */
static ak4535_stereo_mixer_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!(c"Mic Sidetone Switch".as_ptr(), AK4535_SIG2, 4, 1, 0),
    SOC_DAPM_SINGLE!(c"Playback Switch".as_ptr(), AK4535_SIG2, 7, 1, 0),
    SOC_DAPM_SINGLE!(c"Aux Bypass Switch".as_ptr(), AK4535_SIG2, 5, 1, 0),
];

/* Input Mixer */
static ak4535_input_mixer_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE!(c"Mic Capture Switch".as_ptr(), AK4535_MIC, 2, 1, 0),
    SOC_DAPM_SINGLE!(c"Aux Capture Switch".as_ptr(), AK4535_MIC, 5, 1, 0),
];

/* Input mux */
static ak4535_input_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!(c"Input Select".as_ptr(), ak4535_enum[4]);

/* HP L switch */
static ak4535_hpl_control: snd_kcontrol_new =
    SOC_DAPM_SINGLE!(c"Switch".as_ptr(), AK4535_SIG2, 1, 1, 1);

/* HP R switch */
static ak4535_hpr_control: snd_kcontrol_new =
    SOC_DAPM_SINGLE!(c"Switch".as_ptr(), AK4535_SIG2, 0, 1, 1);

/* mono 2 switch */
static ak4535_mono2_control: snd_kcontrol_new =
    SOC_DAPM_SINGLE!(c"Switch".as_ptr(), AK4535_SIG1, 0, 1, 0);

/* Line out switch */
static ak4535_line_control: snd_kcontrol_new =
    SOC_DAPM_SINGLE!(c"Switch".as_ptr(), AK4535_SIG2, 6, 1, 0);

/* ak4535 dapm widgets */
static ak4535_dapm_widgets: [snd_soc_dapm_widget; 34] = [
    SND_SOC_DAPM_MIXER!(c"Stereo Mixer".as_ptr(), SND_SOC_NOPM, 0, 0, &ak4535_stereo_mixer_controls[0], ARRAY_SIZE!(ak4535_stereo_mixer_controls)),
    SND_SOC_DAPM_MIXER!(c"Mono1 Mixer".as_ptr(), SND_SOC_NOPM, 0, 0, &ak4535_mono1_mixer_controls[0], ARRAY_SIZE!(ak4535_mono1_mixer_controls)),
    SND_SOC_DAPM_MIXER!(c"Input Mixer".as_ptr(), SND_SOC_NOPM, 0, 0, &ak4535_input_mixer_controls[0], ARRAY_SIZE!(ak4535_input_mixer_controls)),
    SND_SOC_DAPM_MUX!(c"Input Mux".as_ptr(), SND_SOC_NOPM, 0, 0, &ak4535_input_mux_control),
    SND_SOC_DAPM_DAC!(c"DAC".as_ptr(), c"Playback".as_ptr(), AK4535_PM2, 0, 0),
    SND_SOC_DAPM_SWITCH!(c"Mono 2 Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &ak4535_mono2_control),
    /* speaker powersave bit */
    SND_SOC_DAPM_PGA!(c"Speaker Enable".as_ptr(), AK4535_MODE2, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SWITCH!(c"Line Out Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &ak4535_line_control),
    SND_SOC_DAPM_SWITCH!(c"Left HP Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &ak4535_hpl_control),
    SND_SOC_DAPM_SWITCH!(c"Right HP Enable".as_ptr(), SND_SOC_NOPM, 0, 0, &ak4535_hpr_control),
    SND_SOC_DAPM_OUTPUT!(c"LOUT".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"HPL".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"ROUT".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"HPR".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"SPP".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"SPN".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"MOUT1".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"MOUT2".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"MICOUT".as_ptr()),
    SND_SOC_DAPM_ADC!(c"ADC".as_ptr(), c"Capture".as_ptr(), AK4535_PM1, 0, 0),
    SND_SOC_DAPM_PGA!(c"Spk Amp".as_ptr(), AK4535_PM2, 3, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!(c"HP R Amp".as_ptr(), AK4535_PM2, 1, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!(c"HP L Amp".as_ptr(), AK4535_PM2, 2, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!(c"Mic".as_ptr(), AK4535_PM1, 1, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!(c"Line Out".as_ptr(), AK4535_PM1, 4, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!(c"Mono Out".as_ptr(), AK4535_PM1, 3, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA!(c"AUX In".as_ptr(), AK4535_PM1, 2, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MICBIAS!(c"Mic Int Bias".as_ptr(), AK4535_MIC, 3, 0),
    SND_SOC_DAPM_MICBIAS!(c"Mic Ext Bias".as_ptr(), AK4535_MIC, 4, 0),
    SND_SOC_DAPM_INPUT!(c"MICIN".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"MICEXT".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"AUX".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"MIN".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"AIN".as_ptr()),
];

static ak4535_audio_map: [snd_soc_dapm_route; 43] = [
    /*stereo mixer */
    snd_soc_dapm_route { sink: c"Stereo Mixer".as_ptr(), control: c"Playback Switch".as_ptr(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"Stereo Mixer".as_ptr(), control: c"Mic Sidetone Switch".as_ptr(), source: c"Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Stereo Mixer".as_ptr(), control: c"Aux Bypass Switch".as_ptr(), source: c"AUX In".as_ptr() },

    /* mono1 mixer */
    snd_soc_dapm_route { sink: c"Mono1 Mixer".as_ptr(), control: c"Mic Sidetone Switch".as_ptr(), source: c"Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono1 Mixer".as_ptr(), control: c"Mono Playback Switch".as_ptr(), source: c"DAC".as_ptr() },

    /* Mic */
    snd_soc_dapm_route { sink: c"Mic".as_ptr(), control: core::ptr::null(), source: c"AIN".as_ptr() },
    snd_soc_dapm_route { sink: c"Input Mux".as_ptr(), control: c"Internal".as_ptr(), source: c"Mic Int Bias".as_ptr() },
    snd_soc_dapm_route { sink: c"Input Mux".as_ptr(), control: c"External".as_ptr(), source: c"Mic Ext Bias".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic Int Bias".as_ptr(), control: core::ptr::null(), source: c"MICIN".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic Ext Bias".as_ptr(), control: core::ptr::null(), source: c"MICEXT".as_ptr() },
    snd_soc_dapm_route { sink: c"MICOUT".as_ptr(), control: core::ptr::null(), source: c"Input Mux".as_ptr() },

    /* line out */
    snd_soc_dapm_route { sink: c"LOUT".as_ptr(), control: core::ptr::null(), source: c"Line Out Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"ROUT".as_ptr(), control: core::ptr::null(), source: c"Line Out Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"Line Out Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"Line Out".as_ptr() },
    snd_soc_dapm_route { sink: c"Line Out".as_ptr(), control: core::ptr::null(), source: c"Stereo Mixer".as_ptr() },

    /* mono1 out */
    snd_soc_dapm_route { sink: c"MOUT1".as_ptr(), control: core::ptr::null(), source: c"Mono Out".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono Out".as_ptr(), control: core::ptr::null(), source: c"Mono1 Mixer".as_ptr() },

    /* left HP */
    snd_soc_dapm_route { sink: c"HPL".as_ptr(), control: core::ptr::null(), source: c"Left HP Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"Left HP Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"HP L Amp".as_ptr() },
    snd_soc_dapm_route { sink: c"HP L Amp".as_ptr(), control: core::ptr::null(), source: c"Stereo Mixer".as_ptr() },

    /* right HP */
    snd_soc_dapm_route { sink: c"HPR".as_ptr(), control: core::ptr::null(), source: c"Right HP Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"Right HP Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"HP R Amp".as_ptr() },
    snd_soc_dapm_route { sink: c"HP R Amp".as_ptr(), control: core::ptr::null(), source: c"Stereo Mixer".as_ptr() },

    /* speaker */
    snd_soc_dapm_route { sink: c"SPP".as_ptr(), control: core::ptr::null(), source: c"Speaker Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"SPN".as_ptr(), control: core::ptr::null(), source: c"Speaker Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"Speaker Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"Spk Amp".as_ptr() },
    snd_soc_dapm_route { sink: c"Spk Amp".as_ptr(), control: core::ptr::null(), source: c"MIN".as_ptr() },

    /* mono 2 */
    snd_soc_dapm_route { sink: c"MOUT2".as_ptr(), control: core::ptr::null(), source: c"Mono 2 Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"Mono 2 Enable".as_ptr(), control: c"Switch".as_ptr(), source: c"Stereo Mixer".as_ptr() },

    /* Aux In */
    snd_soc_dapm_route { sink: c"Aux In".as_ptr(), control: core::ptr::null(), source: c"AUX".as_ptr() },

    /* ADC */
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"Input Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Input Mixer".as_ptr(), control: c"Mic Capture Switch".as_ptr(), source: c"Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Input Mixer".as_ptr(), control: c"Aux Capture Switch".as_ptr(), source: c"Aux In".as_ptr() },
];

unsafe extern "C" fn ak4535_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let ak4535: *mut ak4535_priv = snd_soc_component_get_drvdata(component) as *mut ak4535_priv;

    (*ak4535).sysclk = freq;
    0
}

unsafe extern "C" fn ak4535_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let ak4535: *mut ak4535_priv = snd_soc_component_get_drvdata(component) as *mut ak4535_priv;
    let mut mode2: u8 = (snd_soc_component_read(component, AK4535_MODE2) & !(0x3 << 5)) as u8;
    let rate: c_int = params_rate(params);
    let mut fs: c_int = 256;

    if rate != 0 {
        fs = ((*ak4535).sysclk / rate as c_uint) as c_int;
    }

    /* set fs */
    match fs {
        1024 => {
            mode2 |= (0x2 << 5) as u8;
        }
        512 => {
            mode2 |= (0x1 << 5) as u8;
        }
        256 => {}
        _ => {}
    }

    /* set rate */
    snd_soc_component_write(component, AK4535_MODE2, mode2 as c_uint);
    0
}

unsafe extern "C" fn ak4535_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let mut mode1: u8 = 0;

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            mode1 = 0x0002;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            mode1 = 0x0001;
        }
        _ => {
            return -EINVAL;
        }
    }

    /* use 32 fs for BCLK to save power */
    mode1 |= 0x4;

    snd_soc_component_write(component, AK4535_MODE1, mode1 as c_uint);
    0
}

unsafe extern "C" fn ak4535_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mute_reg: u16 = snd_soc_component_read(component, AK4535_DAC) as u16;

    if mute == 0 {
        snd_soc_component_write(component, AK4535_DAC, (mute_reg & !0x20) as c_uint);
    } else {
        snd_soc_component_write(component, AK4535_DAC, (mute_reg | 0x20) as c_uint);
    }
    0
}

unsafe extern "C" fn ak4535_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    match level {
        SND_SOC_BIAS_ON => {
            snd_soc_component_update_bits(component, AK4535_DAC, 0x20, 0);
        }
        SND_SOC_BIAS_PREPARE => {
            snd_soc_component_update_bits(component, AK4535_DAC, 0x20, 0x20);
        }
        SND_SOC_BIAS_STANDBY => {
            snd_soc_component_update_bits(component, AK4535_PM1, 0x80, 0x80);
            snd_soc_component_update_bits(component, AK4535_PM2, 0x80, 0);
        }
        SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(component, AK4535_PM1, 0x80, 0);
        }
    }
    0
}

const AK4535_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_11025
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000;

static ak4535_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(ak4535_hw_params),
    set_fmt: Some(ak4535_set_dai_fmt),
    mute_stream: Some(ak4535_mute),
    set_sysclk: Some(ak4535_set_dai_sysclk),
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

static mut ak4535_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"ak4535-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: AK4535_RATES,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
        ..unsafe { core::mem::zeroed() }
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: AK4535_RATES,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &ak4535_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn ak4535_resume(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_cache_sync(component);
    0
}

static ak4535_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,

    max_register: AK4535_STATUS,
    volatile_reg: Some(ak4535_volatile),

    cache_type: REGCACHE_RBTREE,
    reg_defaults: ak4535_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(ak4535_reg_defaults),
    ..unsafe { core::mem::zeroed() }
};

static soc_component_dev_ak4535: snd_soc_component_driver = snd_soc_component_driver {
    resume: Some(ak4535_resume),
    set_bias_level: Some(ak4535_set_bias_level),
    controls: ak4535_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(ak4535_snd_controls),
    dapm_widgets: ak4535_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(ak4535_dapm_widgets),
    dapm_routes: ak4535_audio_map.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(ak4535_audio_map),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn ak4535_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut ak4535: *mut ak4535_priv;
    let ret: c_int;

    ak4535 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<ak4535_priv>(),
        GFP_KERNEL,
    ) as *mut ak4535_priv;
    if ak4535.is_null() {
        return -ENOMEM;
    }

    (*ak4535).regmap = devm_regmap_init_i2c(i2c, &ak4535_regmap);
    if IS_ERR((*ak4535).regmap) {
        ret = PTR_ERR((*ak4535).regmap) as c_int;
        dev_err(
            &mut (*i2c).dev,
            c"Failed to init regmap: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    i2c_set_clientdata(i2c, ak4535 as *mut core::ffi::c_void);

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_ak4535,
        &mut ak4535_dai,
        1,
    );

    ret
}

static ak4535_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: *b"ak4535\0",
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
MODULE_DEVICE_TABLE!(i2c, ak4535_i2c_id);

static mut ak4535_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"ak4535".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(ak4535_i2c_probe),
    id_table: ak4535_i2c_id.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

module_i2c_driver!(ak4535_i2c_driver);

MODULE_DESCRIPTION!(c"Soc AK4535 driver".as_ptr());
MODULE_AUTHOR!(c"Richard Purdie".as_ptr());
MODULE_LICENSE!(c"GPL".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
