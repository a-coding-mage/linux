// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PCM1681 ASoC codec driver
 *
 * Copyright (c) StreamUnlimited GmbH 2013
 *	Marek Belisko <marek.belisko@streamunlimited.com>
 */

/* C includes translated as external dependencies:
 * linux/module.h, linux/slab.h, linux/delay.h, linux/i2c.h,
 * linux/regmap.h, linux/of.h, sound/pcm.h, sound/pcm_params.h,
 * sound/soc.h, sound/tlv.h
 */

use crate::*;

const PCM1681_PCM_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;

const PCM1681_PCM_RATES: u32 = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_192000;

const PCM1681_SOFT_MUTE_ALL: u32 = 0xff;
const PCM1681_DEEMPH_RATE_MASK: u32 = 0x18;
const PCM1681_DEEMPH_MASK: u32 = 0x01;

/* Attenuation level */
const fn PCM1681_ATT_CONTROL(x: u32) -> u32 {
    if x <= 6 { x } else { x + 9 }
}

/* Soft mute control register */
const PCM1681_SOFT_MUTE: u32 = 0x07;
/* DAC operation control */
const PCM1681_DAC_CONTROL: u32 = 0x08;
/* Audio interface data format */
const PCM1681_FMT_CONTROL: u32 = 0x09;
/* De-emphasis control */
const PCM1681_DEEMPH_CONTROL: u32 = 0x0a;
/* Zero detect status reg */
const PCM1681_ZERO_DETECT_STATUS: u32 = 0x0e;

static pcm1681_reg_defaults: [reg_default; 17] = [
    reg_default { reg: 0x01, def: 0xff },
    reg_default { reg: 0x02, def: 0xff },
    reg_default { reg: 0x03, def: 0xff },
    reg_default { reg: 0x04, def: 0xff },
    reg_default { reg: 0x05, def: 0xff },
    reg_default { reg: 0x06, def: 0xff },
    reg_default { reg: 0x07, def: 0x00 },
    reg_default { reg: 0x08, def: 0x00 },
    reg_default { reg: 0x09, def: 0x06 },
    reg_default { reg: 0x0A, def: 0x00 },
    reg_default { reg: 0x0B, def: 0xff },
    reg_default { reg: 0x0C, def: 0x0f },
    reg_default { reg: 0x0D, def: 0x00 },
    reg_default { reg: 0x10, def: 0xff },
    reg_default { reg: 0x11, def: 0xff },
    reg_default { reg: 0x12, def: 0x00 },
    reg_default { reg: 0x13, def: 0x00 },
];

unsafe extern "C" fn pcm1681_accessible_reg(_dev: *mut device, reg: c_uint) -> bool {
    !((reg == 0x00) || (reg == 0x0f))
}

unsafe extern "C" fn pcm1681_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    unsafe { pcm1681_accessible_reg(dev, reg) && (reg != PCM1681_ZERO_DETECT_STATUS) }
}

#[repr(C)]
struct pcm1681_private {
    regmap: *mut regmap,
    format: c_uint,
    /* Current deemphasis status */
    deemph: c_uint,
    /* Current rate for deemphasis control */
    rate: c_uint,
}

static pcm1681_deemph: [c_int; 3] = [44100, 48000, 32000];

unsafe extern "C" fn pcm1681_set_deemph(component: *mut snd_soc_component) -> c_int {
    let priv_: *mut pcm1681_private =
        unsafe { snd_soc_component_get_drvdata(component) as *mut pcm1681_private };
    let mut val: c_int = -1;
    let mut enable: c_int = 0;

    if unsafe { (*priv_).deemph } != 0 {
        let mut i: usize = 0;
        while i < pcm1681_deemph.len() {
            if pcm1681_deemph[i] as c_uint == unsafe { (*priv_).rate } {
                val = i as c_int;
                break;
            }
            i += 1;
        }
    }

    if val != -1 {
        unsafe {
            regmap_update_bits(
                (*priv_).regmap,
                PCM1681_DEEMPH_CONTROL,
                PCM1681_DEEMPH_RATE_MASK,
                (val << 3) as c_uint,
            );
        }
        enable = 1;
    } else {
        enable = 0;
    }

    /* enable/disable deemphasis functionality */
    unsafe {
        regmap_update_bits(
            (*priv_).regmap,
            PCM1681_DEEMPH_CONTROL,
            PCM1681_DEEMPH_MASK,
            enable as c_uint,
        )
    }
}

unsafe extern "C" fn pcm1681_get_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { snd_kcontrol_chip(kcontrol) };
    let priv_: *mut pcm1681_private =
        unsafe { snd_soc_component_get_drvdata(component) as *mut pcm1681_private };

    unsafe {
        (*ucontrol).value.integer.value[0] = (*priv_).deemph as c_long;
    }

    0
}

unsafe extern "C" fn pcm1681_put_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { snd_kcontrol_chip(kcontrol) };
    let priv_: *mut pcm1681_private =
        unsafe { snd_soc_component_get_drvdata(component) as *mut pcm1681_private };

    unsafe {
        (*priv_).deemph = (*ucontrol).value.integer.value[0] as c_uint;
    }

    unsafe { pcm1681_set_deemph(component) }
}

unsafe extern "C" fn pcm1681_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    format: c_uint,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { (*codec_dai).component };
    let priv_: *mut pcm1681_private =
        unsafe { snd_soc_component_get_drvdata(component) as *mut pcm1681_private };

    /* The PCM1681 can only be consumer to all clocks */
    if (format & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC {
        unsafe {
            dev_err((*component).dev, c"Invalid clocking mode\n".as_ptr());
        }
        return -EINVAL;
    }

    unsafe {
        (*priv_).format = format;
    }

    0
}

unsafe extern "C" fn pcm1681_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { (*dai).component };
    let priv_: *mut pcm1681_private =
        unsafe { snd_soc_component_get_drvdata(component) as *mut pcm1681_private };
    let val: c_int;

    if mute != 0 {
        val = PCM1681_SOFT_MUTE_ALL as c_int;
    } else {
        val = 0;
    }

    unsafe { regmap_write((*priv_).regmap, PCM1681_SOFT_MUTE, val as c_uint) }
}

unsafe extern "C" fn pcm1681_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { (*dai).component };
    let priv_: *mut pcm1681_private =
        unsafe { snd_soc_component_get_drvdata(component) as *mut pcm1681_private };
    let mut val: c_int = 0;
    let ret: c_int;

    unsafe {
        (*priv_).rate = params_rate(params) as c_uint;
    }

    match unsafe { (*priv_).format & SND_SOC_DAIFMT_FORMAT_MASK } {
        SND_SOC_DAIFMT_RIGHT_J => {
            match unsafe { params_width(params) } {
                24 => {
                    val = 0;
                }
                16 => {
                    val = 3;
                }
                _ => {
                    return -EINVAL;
                }
            }
        }
        SND_SOC_DAIFMT_I2S => {
            val = 0x04;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            val = 0x05;
        }
        _ => {
            unsafe {
                dev_err((*component).dev, c"Invalid DAI format\n".as_ptr());
            }
            return -EINVAL;
        }
    }

    ret = unsafe { regmap_update_bits((*priv_).regmap, PCM1681_FMT_CONTROL, 0x0f, val as c_uint) };
    if ret < 0 {
        return ret;
    }

    unsafe { pcm1681_set_deemph(component) }
}

static pcm1681_selectable_formats: u64 = SND_SOC_POSSIBLE_DAIFMT_I2S
    | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J
    | SND_SOC_POSSIBLE_DAIFMT_LEFT_J;

static pcm1681_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(pcm1681_set_dai_fmt),
    hw_params: Some(pcm1681_hw_params),
    mute_stream: Some(pcm1681_mute),
    auto_selectable_formats: &pcm1681_selectable_formats,
    num_auto_selectable_formats: 1,
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

static pcm1681_dapm_widgets: [snd_soc_dapm_widget; 8] = [
    SND_SOC_DAPM_OUTPUT(c"VOUT1"),
    SND_SOC_DAPM_OUTPUT(c"VOUT2"),
    SND_SOC_DAPM_OUTPUT(c"VOUT3"),
    SND_SOC_DAPM_OUTPUT(c"VOUT4"),
    SND_SOC_DAPM_OUTPUT(c"VOUT5"),
    SND_SOC_DAPM_OUTPUT(c"VOUT6"),
    SND_SOC_DAPM_OUTPUT(c"VOUT7"),
    SND_SOC_DAPM_OUTPUT(c"VOUT8"),
];

static pcm1681_dapm_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: c"VOUT1".as_ptr(), control: core::ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT2".as_ptr(), control: core::ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT3".as_ptr(), control: core::ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT4".as_ptr(), control: core::ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT5".as_ptr(), control: core::ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT6".as_ptr(), control: core::ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT7".as_ptr(), control: core::ptr::null(), source: c"Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"VOUT8".as_ptr(), control: core::ptr::null(), source: c"Playback".as_ptr() },
];

static pcm1681_dac_tlv: [c_uint; TLV_DB_SCALE_ITEM_LEN] = DECLARE_TLV_DB_SCALE(-6350, 50, 1);

static pcm1681_controls: [snd_kcontrol_new; 5] = [
    SOC_DOUBLE_R_TLV(
        c"Channel 1/2 Playback Volume",
        PCM1681_ATT_CONTROL(1),
        PCM1681_ATT_CONTROL(2),
        0,
        0x7f,
        0,
        pcm1681_dac_tlv.as_ptr(),
    ),
    SOC_DOUBLE_R_TLV(
        c"Channel 3/4 Playback Volume",
        PCM1681_ATT_CONTROL(3),
        PCM1681_ATT_CONTROL(4),
        0,
        0x7f,
        0,
        pcm1681_dac_tlv.as_ptr(),
    ),
    SOC_DOUBLE_R_TLV(
        c"Channel 5/6 Playback Volume",
        PCM1681_ATT_CONTROL(5),
        PCM1681_ATT_CONTROL(6),
        0,
        0x7f,
        0,
        pcm1681_dac_tlv.as_ptr(),
    ),
    SOC_DOUBLE_R_TLV(
        c"Channel 7/8 Playback Volume",
        PCM1681_ATT_CONTROL(7),
        PCM1681_ATT_CONTROL(8),
        0,
        0x7f,
        0,
        pcm1681_dac_tlv.as_ptr(),
    ),
    SOC_SINGLE_BOOL_EXT(
        c"De-emphasis Switch",
        0,
        Some(pcm1681_get_deemph),
        Some(pcm1681_put_deemph),
    ),
];

static mut pcm1681_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"pcm1681-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 8,
        rates: PCM1681_PCM_RATES,
        formats: PCM1681_PCM_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &pcm1681_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

/* CONFIG_OF: Open Firmware device IDs are present when enabled by the build. */
static pcm1681_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"ti,pcm1681".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
MODULE_DEVICE_TABLE(of, pcm1681_dt_ids);

static pcm1681_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: 0x13,
    reg_defaults: pcm1681_reg_defaults.as_ptr(),
    num_reg_defaults: pcm1681_reg_defaults.len() as c_uint,
    writeable_reg: Some(pcm1681_writeable_reg),
    readable_reg: Some(pcm1681_accessible_reg),
    ..unsafe { core::mem::zeroed() }
};

static soc_component_dev_pcm1681: snd_soc_component_driver = snd_soc_component_driver {
    controls: pcm1681_controls.as_ptr(),
    num_controls: pcm1681_controls.len() as c_uint,
    dapm_widgets: pcm1681_dapm_widgets.as_ptr(),
    num_dapm_widgets: pcm1681_dapm_widgets.len() as c_uint,
    dapm_routes: pcm1681_dapm_routes.as_ptr(),
    num_dapm_routes: pcm1681_dapm_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

static pcm1681_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: *b"pcm1681\0",
        ..unsafe { core::mem::zeroed() }
    },
    i2c_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
MODULE_DEVICE_TABLE(i2c, pcm1681_i2c_id);

unsafe extern "C" fn pcm1681_i2c_probe(client: *mut i2c_client) -> c_int {
    let ret: c_int;
    let priv_: *mut pcm1681_private;

    priv_ = unsafe {
        devm_kzalloc(
            &mut (*client).dev,
            core::mem::size_of::<pcm1681_private>(),
            GFP_KERNEL,
        ) as *mut pcm1681_private
    };
    if priv_.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*priv_).regmap = devm_regmap_init_i2c(client, &pcm1681_regmap);
    }
    if unsafe { IS_ERR((*priv_).regmap) } {
        ret = unsafe { PTR_ERR((*priv_).regmap) as c_int };
        unsafe {
            dev_err(&mut (*client).dev, c"Failed to create regmap: %d\n".as_ptr(), ret);
        }
        return ret;
    }

    unsafe {
        i2c_set_clientdata(client, priv_ as *mut c_void);
    }

    unsafe {
        devm_snd_soc_register_component(
            &mut (*client).dev,
            &soc_component_dev_pcm1681,
            &mut pcm1681_dai,
            1,
        )
    }
}

static mut pcm1681_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"pcm1681".as_ptr(),
        of_match_table: of_match_ptr(pcm1681_dt_ids.as_ptr()),
        ..unsafe { core::mem::zeroed() }
    },
    id_table: pcm1681_i2c_id.as_ptr(),
    probe: Some(pcm1681_i2c_probe),
    ..unsafe { core::mem::zeroed() }
};

module_i2c_driver!(pcm1681_i2c_driver);

MODULE_DESCRIPTION(c"Texas Instruments PCM1681 ALSA SoC Codec Driver");
MODULE_AUTHOR(c"Marek Belisko <marek.belisko@streamunlimited.com>");
MODULE_LICENSE(c"GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
