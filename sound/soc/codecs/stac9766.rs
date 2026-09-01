// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * stac9766.c  --  ALSA SoC STAC9766 codec support
 *
 * Copyright 2009 Jon Smirl, Digispeaker
 * Author: Jon Smirl <jonsmirl@gmail.com>
 *
 *  Features:-
 *
 *   o Support for AC97 Codec, S/PDIF
 */

// C includes removed; external Linux/ALSA/regmap symbols are expected from bindings.

const STAC9766_VENDOR_ID: u32 = 0x83847666;
const STAC9766_VENDOR_ID_MASK: u32 = 0xffffffff;

const AC97_STAC_DA_CONTROL: u16 = 0x6A;
const AC97_STAC_ANALOG_SPECIAL: u16 = 0x6E;
const AC97_STAC_STEREO_MIC: u16 = 0x78;

static stac9766_reg_defaults: [reg_default; 30] = [
    reg_default { reg: 0x02, def: 0x8000 },
    reg_default { reg: 0x04, def: 0x8000 },
    reg_default { reg: 0x06, def: 0x8000 },
    reg_default { reg: 0x0a, def: 0x0000 },
    reg_default { reg: 0x0c, def: 0x8008 },
    reg_default { reg: 0x0e, def: 0x8008 },
    reg_default { reg: 0x10, def: 0x8808 },
    reg_default { reg: 0x12, def: 0x8808 },
    reg_default { reg: 0x14, def: 0x8808 },
    reg_default { reg: 0x16, def: 0x8808 },
    reg_default { reg: 0x18, def: 0x8808 },
    reg_default { reg: 0x1a, def: 0x0000 },
    reg_default { reg: 0x1c, def: 0x8000 },
    reg_default { reg: 0x20, def: 0x0000 },
    reg_default { reg: 0x22, def: 0x0000 },
    reg_default { reg: 0x28, def: 0x0a05 },
    reg_default { reg: 0x2c, def: 0xbb80 },
    reg_default { reg: 0x32, def: 0xbb80 },
    reg_default { reg: 0x3a, def: 0x2000 },
    reg_default { reg: 0x3e, def: 0x0100 },
    reg_default { reg: 0x4c, def: 0x0300 },
    reg_default { reg: 0x4e, def: 0xffff },
    reg_default { reg: 0x50, def: 0x0000 },
    reg_default { reg: 0x52, def: 0x0000 },
    reg_default { reg: 0x54, def: 0x0000 },
    reg_default { reg: 0x6a, def: 0x0000 },
    reg_default { reg: 0x6e, def: 0x1000 },
    reg_default { reg: 0x72, def: 0x0000 },
    reg_default { reg: 0x78, def: 0x0000 },
];

static stac9766_regmap_config: regmap_config = regmap_config {
    reg_bits: 16,
    reg_stride: 2,
    val_bits: 16,
    max_register: 0x78,
    cache_type: REGCACHE_MAPLE,

    volatile_reg: Some(regmap_ac97_default_volatile),

    reg_defaults: stac9766_reg_defaults.as_ptr(),
    num_reg_defaults: stac9766_reg_defaults.len() as _,
};

static stac9766_record_mux: [&str; 8] = [
    "Mic",
    "CD",
    "Video",
    "AUX",
    "Line",
    "Stereo Mix",
    "Mono Mix",
    "Phone",
];
static stac9766_mono_mux: [&str; 2] = ["Mix", "Mic"];
static stac9766_mic_mux: [&str; 2] = ["Mic1", "Mic2"];
static stac9766_SPDIF_mux: [&str; 2] = ["PCM", "ADC Record"];
static stac9766_popbypass_mux: [&str; 2] = ["Normal", "Bypass Mixer"];
static stac9766_record_all_mux: [&str; 2] = ["All analog", "Analog plus DAC"];
static stac9766_boost1: [&str; 2] = ["0dB", "10dB"];
static stac9766_boost2: [&str; 2] = ["0dB", "20dB"];
static stac9766_stereo_mic: [&str; 2] = ["Off", "On"];

static stac9766_record_enum: soc_enum =
    SOC_ENUM_DOUBLE_DECL!(AC97_REC_SEL, 8, 0, stac9766_record_mux);
static stac9766_mono_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(AC97_GENERAL_PURPOSE, 9, stac9766_mono_mux);
static stac9766_mic_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(AC97_GENERAL_PURPOSE, 8, stac9766_mic_mux);
static stac9766_SPDIF_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(AC97_STAC_DA_CONTROL, 1, stac9766_SPDIF_mux);
static stac9766_popbypass_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(AC97_GENERAL_PURPOSE, 15, stac9766_popbypass_mux);
static stac9766_record_all_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(AC97_STAC_ANALOG_SPECIAL, 12, stac9766_record_all_mux);
static stac9766_boost1_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(AC97_MIC, 6, stac9766_boost1); /* 0/10dB */
static stac9766_boost2_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(AC97_STAC_ANALOG_SPECIAL, 2, stac9766_boost2); /* 0/20dB */
static stac9766_stereo_mic_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(AC97_STAC_STEREO_MIC, 2, stac9766_stereo_mic);

static master_tlv: [u32; 4] = SNDRV_CTL_TLVD_DECLARE_DB_SCALE!(-4650, 150, 0);
static record_tlv: [u32; 4] = SNDRV_CTL_TLVD_DECLARE_DB_SCALE!(0, 150, 0);
static beep_tlv: [u32; 4] = SNDRV_CTL_TLVD_DECLARE_DB_SCALE!(-4500, 300, 0);
static mix_tlv: [u32; 4] = SNDRV_CTL_TLVD_DECLARE_DB_SCALE!(-3450, 150, 0);

static stac9766_snd_ac97_controls: [snd_kcontrol_new; 37] = [
    SOC_DOUBLE_TLV!("Speaker Volume", AC97_MASTER, 8, 0, 31, 1, master_tlv),
    SOC_SINGLE!("Speaker Switch", AC97_MASTER, 15, 1, 1),
    SOC_DOUBLE_TLV!("Headphone Volume", AC97_HEADPHONE, 8, 0, 31, 1, master_tlv),
    SOC_SINGLE!("Headphone Switch", AC97_HEADPHONE, 15, 1, 1),
    SOC_SINGLE_TLV!("Mono Out Volume", AC97_MASTER_MONO, 0, 31, 1, master_tlv),
    SOC_SINGLE!("Mono Out Switch", AC97_MASTER_MONO, 15, 1, 1),

    SOC_DOUBLE_TLV!("Record Volume", AC97_REC_GAIN, 8, 0, 15, 0, record_tlv),
    SOC_SINGLE!("Record Switch", AC97_REC_GAIN, 15, 1, 1),

    SOC_SINGLE_TLV!("Beep Volume", AC97_PC_BEEP, 1, 15, 1, beep_tlv),
    SOC_SINGLE!("Beep Switch", AC97_PC_BEEP, 15, 1, 1),
    SOC_SINGLE!("Beep Frequency", AC97_PC_BEEP, 5, 127, 1),
    SOC_SINGLE_TLV!("Phone Volume", AC97_PHONE, 0, 31, 1, mix_tlv),
    SOC_SINGLE!("Phone Switch", AC97_PHONE, 15, 1, 1),

    SOC_ENUM!("Mic Boost1", stac9766_boost1_enum),
    SOC_ENUM!("Mic Boost2", stac9766_boost2_enum),
    SOC_SINGLE_TLV!("Mic Volume", AC97_MIC, 0, 31, 1, mix_tlv),
    SOC_SINGLE!("Mic Switch", AC97_MIC, 15, 1, 1),
    SOC_ENUM!("Stereo Mic", stac9766_stereo_mic_enum),

    SOC_DOUBLE_TLV!("Line Volume", AC97_LINE, 8, 0, 31, 1, mix_tlv),
    SOC_SINGLE!("Line Switch", AC97_LINE, 15, 1, 1),
    SOC_DOUBLE_TLV!("CD Volume", AC97_CD, 8, 0, 31, 1, mix_tlv),
    SOC_SINGLE!("CD Switch", AC97_CD, 15, 1, 1),
    SOC_DOUBLE_TLV!("AUX Volume", AC97_AUX, 8, 0, 31, 1, mix_tlv),
    SOC_SINGLE!("AUX Switch", AC97_AUX, 15, 1, 1),
    SOC_DOUBLE_TLV!("Video Volume", AC97_VIDEO, 8, 0, 31, 1, mix_tlv),
    SOC_SINGLE!("Video Switch", AC97_VIDEO, 15, 1, 1),

    SOC_DOUBLE_TLV!("DAC Volume", AC97_PCM, 8, 0, 31, 1, mix_tlv),
    SOC_SINGLE!("DAC Switch", AC97_PCM, 15, 1, 1),
    SOC_SINGLE!("Loopback Test Switch", AC97_GENERAL_PURPOSE, 7, 1, 0),
    SOC_SINGLE!("3D Volume", AC97_3D_CONTROL, 3, 2, 1),
    SOC_SINGLE!("3D Switch", AC97_GENERAL_PURPOSE, 13, 1, 0),

    SOC_ENUM!("SPDIF Mux", stac9766_SPDIF_enum),
    SOC_ENUM!("Mic1/2 Mux", stac9766_mic_enum),
    SOC_ENUM!("Record All Mux", stac9766_record_all_enum),
    SOC_ENUM!("Record Mux", stac9766_record_enum),
    SOC_ENUM!("Mono Mux", stac9766_mono_enum),
    SOC_ENUM!("Pop Bypass Mux", stac9766_popbypass_enum),
];

unsafe extern "C" fn ac97_analog_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let reg: c_ushort;

    /* enable variable rate audio, disable SPDIF output */
    snd_soc_component_update_bits(component, AC97_EXTENDED_STATUS, 0x5, 0x1);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        reg = AC97_PCM_FRONT_DAC_RATE;
    } else {
        reg = AC97_PCM_LR_ADC_RATE;
    }

    snd_soc_component_write(component, reg, (*runtime).rate)
}

unsafe extern "C" fn ac97_digital_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let reg: c_ushort;

    snd_soc_component_write(component, AC97_SPDIF, 0x2002);

    /* Enable VRA and SPDIF out */
    snd_soc_component_update_bits(component, AC97_EXTENDED_STATUS, 0x5, 0x5);

    reg = AC97_PCM_FRONT_DAC_RATE;

    snd_soc_component_write(component, reg, (*runtime).rate)
}

unsafe extern "C" fn stac9766_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    match level {
        SND_SOC_BIAS_ON | /* full On */
        SND_SOC_BIAS_PREPARE | /* partial On */
        SND_SOC_BIAS_STANDBY /* Off, with power */ => {
            snd_soc_component_write(component, AC97_POWERDOWN, 0x0000);
        }
        SND_SOC_BIAS_OFF /* Off, without power */ => {
            /* disable everything including AC link */
            snd_soc_component_write(component, AC97_POWERDOWN, 0xffff);
        }
    }
    0
}

unsafe extern "C" fn stac9766_component_resume(component: *mut snd_soc_component) -> c_int {
    let ac97: *mut snd_ac97 = snd_soc_component_get_drvdata(component) as *mut snd_ac97;

    snd_ac97_reset(
        ac97,
        true,
        STAC9766_VENDOR_ID,
        STAC9766_VENDOR_ID_MASK,
    )
}

static stac9766_dai_ops_analog: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(ac97_analog_prepare),
};

static stac9766_dai_ops_digital: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(ac97_digital_prepare),
};

static mut stac9766_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c_str!("stac9766-hifi-analog"),

        /* stream cababilities */
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("stac9766 analog"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SND_SOC_STD_AC97_FMTS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("stac9766 analog"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SND_SOC_STD_AC97_FMTS,
        },
        /* alsa ops */
        ops: &stac9766_dai_ops_analog,
    },
    snd_soc_dai_driver {
        name: c_str!("stac9766-hifi-IEC958"),

        /* stream cababilities */
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("stac9766 IEC958"),
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
            formats: SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_BE,
        },
        /* alsa ops */
        ops: &stac9766_dai_ops_digital,
    },
];

unsafe extern "C" fn stac9766_component_probe(component: *mut snd_soc_component) -> c_int {
    let ac97: *mut snd_ac97;
    let regmap: *mut regmap;
    let ret: c_int;

    ac97 = snd_soc_new_ac97_component(
        component,
        STAC9766_VENDOR_ID,
        STAC9766_VENDOR_ID_MASK,
    );
    if IS_ERR(ac97 as *const c_void) {
        return PTR_ERR(ac97 as *const c_void);
    }

    regmap = regmap_init_ac97(ac97, &stac9766_regmap_config);
    if IS_ERR(regmap as *const c_void) {
        ret = PTR_ERR(regmap as *const c_void);
        snd_soc_free_ac97_component(ac97);
        return ret;
    }

    snd_soc_component_init_regmap(component, regmap);
    snd_soc_component_set_drvdata(component, ac97 as *mut c_void);

    0
}

unsafe extern "C" fn stac9766_component_remove(component: *mut snd_soc_component) {
    let ac97: *mut snd_ac97 = snd_soc_component_get_drvdata(component) as *mut snd_ac97;

    snd_soc_component_exit_regmap(component);
    snd_soc_free_ac97_component(ac97);
}

static soc_component_dev_stac9766: snd_soc_component_driver = snd_soc_component_driver {
    controls: stac9766_snd_ac97_controls.as_ptr(),
    num_controls: stac9766_snd_ac97_controls.len() as _,
    set_bias_level: Some(stac9766_set_bias_level),
    probe: Some(stac9766_component_probe),
    remove: Some(stac9766_component_remove),
    resume: Some(stac9766_component_resume),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn stac9766_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_stac9766,
        stac9766_dai.as_mut_ptr(),
        stac9766_dai.len() as _,
    )
}

static mut stac9766_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("stac9766-codec"),
    },

    probe: Some(stac9766_probe),
};

module_platform_driver!(stac9766_codec_driver);

MODULE_DESCRIPTION!("ASoC stac9766 driver");
MODULE_AUTHOR!("Jon Smirl <jonsmirl@gmail.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
