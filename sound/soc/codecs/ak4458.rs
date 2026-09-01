// SPDX-License-Identifier: GPL-2.0
//
// Audio driver for AK4458 DAC
//
// Copyright (C) 2016 Asahi Kasei Microdevices Corporation
// Copyright 2018 NXP

// C dependencies removed from executable Rust:
// linux/delay.h, linux/gpio/consumer.h, linux/i2c.h, linux/module.h,
// linux/of.h, linux/pm_runtime.h, linux/regulator/consumer.h,
// linux/reset.h, linux/slab.h, sound/initval.h, sound/pcm_params.h,
// sound/soc.h, sound/soc-dapm.h, sound/tlv.h, and "ak4458.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const AK4458_NUM_SUPPLIES: usize = 2;

static ak4458_supply_names: [*const c_char; AK4458_NUM_SUPPLIES] = [
    b"DVDD\0".as_ptr() as *const c_char,
    b"AVDD\0".as_ptr() as *const c_char,
];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum ak4458_type {
    AK4458 = 0,
    AK4497 = 1,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    no_capture_mute: c_uint,
}

#[repr(C)]
struct snd_soc_component_driver {
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    idle_bias_on: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct ak4458_drvdata {
    dai_drv: *mut snd_soc_dai_driver,
    comp_drv: *const snd_soc_component_driver,
    type_: ak4458_type,
}

/* AK4458 Codec Private Data */
#[repr(C)]
struct ak4458_priv {
    supplies: [regulator_bulk_data; AK4458_NUM_SUPPLIES],
    drvdata: *const ak4458_drvdata,
    dev: *mut device,
    regmap: *mut regmap,
    reset: *mut reset_control,
    mute_gpiod: *mut gpio_desc,
    digfil: c_int,     /* SSLOW, SD, SLOW bits */
    fs: c_int,         /* sampling rate */
    fmt: c_int,
    slots: c_int,
    slot_width: c_int,
    dsd_path: u32, /* For ak4497 */
}

#[repr(C)]
#[derive(Copy, Clone)]
struct reg_default {
    reg: c_uint,
    def: c_uint,
}

static ak4458_reg_defaults: [reg_default; 21] = [
    reg_default { reg: 0x00, def: 0x0C }, /* 0x00 AK4458_00_CONTROL1 */
    reg_default { reg: 0x01, def: 0x22 }, /* 0x01 AK4458_01_CONTROL2 */
    reg_default { reg: 0x02, def: 0x00 }, /* 0x02 AK4458_02_CONTROL3 */
    reg_default { reg: 0x03, def: 0xFF }, /* 0x03 AK4458_03_LCHATT */
    reg_default { reg: 0x04, def: 0xFF }, /* 0x04 AK4458_04_RCHATT */
    reg_default { reg: 0x05, def: 0x00 }, /* 0x05 AK4458_05_CONTROL4 */
    reg_default { reg: 0x06, def: 0x00 }, /* 0x06 AK4458_06_DSD1 */
    reg_default { reg: 0x07, def: 0x03 }, /* 0x07 AK4458_07_CONTROL5 */
    reg_default { reg: 0x08, def: 0x00 }, /* 0x08 AK4458_08_SOUND_CONTROL */
    reg_default { reg: 0x09, def: 0x00 }, /* 0x09 AK4458_09_DSD2 */
    reg_default { reg: 0x0A, def: 0x0D }, /* 0x0A AK4458_0A_CONTROL6 */
    reg_default { reg: 0x0B, def: 0x0C }, /* 0x0B AK4458_0B_CONTROL7 */
    reg_default { reg: 0x0C, def: 0x00 }, /* 0x0C AK4458_0C_CONTROL8 */
    reg_default { reg: 0x0D, def: 0x00 }, /* 0x0D AK4458_0D_CONTROL9 */
    reg_default { reg: 0x0E, def: 0x50 }, /* 0x0E AK4458_0E_CONTROL10 */
    reg_default { reg: 0x0F, def: 0xFF }, /* 0x0F AK4458_0F_L2CHATT */
    reg_default { reg: 0x10, def: 0xFF }, /* 0x10 AK4458_10_R2CHATT */
    reg_default { reg: 0x11, def: 0xFF }, /* 0x11 AK4458_11_L3CHATT */
    reg_default { reg: 0x12, def: 0xFF }, /* 0x12 AK4458_12_R3CHATT */
    reg_default { reg: 0x13, def: 0xFF }, /* 0x13 AK4458_13_L4CHATT */
    reg_default { reg: 0x14, def: 0xFF }, /* 0x14 AK4458_14_R4CHATT */
];

/*
 * Volume control:
 * from -127 to 0 dB in 0.5 dB steps (mute instead of -127.5 dB)
 */
static dac_tlv: [c_uint; 4] = [0, (-12750i32) as c_uint, 50, 1];

static ak4458_dem_select_texts: [*const c_char; 4] = [
    b"44.1kHz\0".as_ptr() as *const c_char,
    b"OFF\0".as_ptr() as *const c_char,
    b"48kHz\0".as_ptr() as *const c_char,
    b"32kHz\0".as_ptr() as *const c_char,
];

static ak4458_digfil_select_texts: [*const c_char; 5] = [
    b"Sharp Roll-Off Filter\0".as_ptr() as *const c_char,
    b"Slow Roll-Off Filter\0".as_ptr() as *const c_char,
    b"Short delay Sharp Roll-Off Filter\0".as_ptr() as *const c_char,
    b"Short delay Slow Roll-Off Filter\0".as_ptr() as *const c_char,
    b"Super Slow Roll-Off Filter\0".as_ptr() as *const c_char,
];

static ak4458_dzfb_select_texts: [*const c_char; 2] = [
    b"H\0".as_ptr() as *const c_char,
    b"L\0".as_ptr() as *const c_char,
];

static ak4458_sc_select_texts: [*const c_char; 3] = [
    b"Sound Mode 0\0".as_ptr() as *const c_char,
    b"Sound Mode 1\0".as_ptr() as *const c_char,
    b"Sound Mode 2\0".as_ptr() as *const c_char,
];

static ak4458_fir_select_texts: [*const c_char; 8] = [
    b"Mode 0\0".as_ptr() as *const c_char,
    b"Mode 1\0".as_ptr() as *const c_char,
    b"Mode 2\0".as_ptr() as *const c_char,
    b"Mode 3\0".as_ptr() as *const c_char,
    b"Mode 4\0".as_ptr() as *const c_char,
    b"Mode 5\0".as_ptr() as *const c_char,
    b"Mode 6\0".as_ptr() as *const c_char,
    b"Mode 7\0".as_ptr() as *const c_char,
];

static ak4458_ats_select_texts: [*const c_char; 4] = [
    b"4080/fs\0".as_ptr() as *const c_char,
    b"2040/fs\0".as_ptr() as *const c_char,
    b"510/fs\0".as_ptr() as *const c_char,
    b"255/fs\0".as_ptr() as *const c_char,
];

static ak4458_dif_select_texts: [*const c_char; 2] = [
    b"32fs,48fs\0".as_ptr() as *const c_char,
    b"64fs\0".as_ptr() as *const c_char,
];

// The SOC_ENUM_* and SOC_* macro-expanded objects depend on ASoC C layouts.
// They are preserved as declarations and referenced by the translated tables.
extern "C" {
    static ak4458_dac1_dem_enum: soc_enum;
    static ak4458_dac2_dem_enum: soc_enum;
    static ak4458_dac3_dem_enum: soc_enum;
    static ak4458_dac4_dem_enum: soc_enum;
    static ak4458_digfil_enum: soc_enum;
    static ak4458_dzfb_enum: soc_enum;
    static ak4458_sm_enum: soc_enum;
    static ak4458_fir_enum: soc_enum;
    static ak4458_ats_enum: soc_enum;
    static ak4458_dif_enum: soc_enum;
}

unsafe extern "C" fn get_digfil(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let ak4458 = snd_soc_component_get_drvdata(component) as *mut ak4458_priv;

    (*ucontrol).value.enumerated.item[0] = (*ak4458).digfil as c_uint;

    0
}

unsafe extern "C" fn set_digfil(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let ak4458 = snd_soc_component_get_drvdata(component) as *mut ak4458_priv;
    let num = (*ucontrol).value.enumerated.item[0] as c_int;

    if num > 4 {
        return -EINVAL;
    }

    (*ak4458).digfil = num;

    /* write SD bit */
    snd_soc_component_update_bits(
        component,
        AK4458_01_CONTROL2,
        AK4458_SD_MASK,
        (((*ak4458).digfil & 0x02) << 4) as c_uint,
    );

    /* write SLOW bit */
    snd_soc_component_update_bits(
        component,
        AK4458_02_CONTROL3,
        AK4458_SLOW_MASK,
        ((*ak4458).digfil & 0x01) as c_uint,
    );

    /* write SSLOW bit */
    snd_soc_component_update_bits(
        component,
        AK4458_05_CONTROL4,
        AK4458_SSLOW_MASK,
        (((*ak4458).digfil & 0x04) >> 2) as c_uint,
    );

    0
}

// ak4458_snd_controls, ak4458_dapm_widgets, ak4458_intercon,
// ak4497_snd_controls, ak4497_dapm_widgets, and ak4497_intercon are direct
// translations of the C macro tables in ak4458.c. Their concrete initializers
// are supplied by ASoC macros in the original C environment.
extern "C" {
    static ak4458_snd_controls: [snd_kcontrol_new; 13];
    static ak4458_dapm_widgets: [snd_soc_dapm_widget; 9];
    static ak4458_intercon: [snd_soc_dapm_route; 8];
    static ak4497_snd_controls: [snd_kcontrol_new; 6];
    static ak4497_dapm_widgets: [snd_soc_dapm_widget; 3];
    static ak4497_intercon: [snd_soc_dapm_route; 2];
}

unsafe extern "C" fn ak4458_get_tdm_mode(ak4458: *mut ak4458_priv) -> c_int {
    match (*ak4458).slots * (*ak4458).slot_width {
        128 => 1,
        256 => 2,
        512 => 3,
        _ => 0,
    }
}

unsafe extern "C" fn ak4458_rstn_control(
    component: *mut snd_soc_component,
    bit: c_int,
) -> c_int {
    let ret;

    if bit != 0 {
        ret = snd_soc_component_update_bits(component, AK4458_00_CONTROL1, AK4458_RSTN_MASK, 0x1);
    } else {
        ret = snd_soc_component_update_bits(component, AK4458_00_CONTROL1, AK4458_RSTN_MASK, 0x0);
    }
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn ak4458_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let ak4458 = snd_soc_component_get_drvdata(component) as *mut ak4458_priv;
    let pcm_width = max(params_physical_width(params), (*ak4458).slot_width);
    let mut format: u8 = 0;
    let mut dsdsel0: u8;
    let mut dsdsel1: u8;
    let dchn: u8;
    let nfs1: c_int;
    let dsd_bclk: c_int;
    let mut ret: c_int;
    let channels: c_int;
    let channels_max: c_int;

    nfs1 = params_rate(params);
    (*ak4458).fs = nfs1;

    /* calculate bit clock */
    channels = params_channels(params);
    channels_max = (*(*dai).driver).playback.channels_max as c_int;

    let params_fmt = params_format(params);
    if params_fmt == SNDRV_PCM_FORMAT_DSD_U8
        || params_fmt == SNDRV_PCM_FORMAT_DSD_U16_LE
        || params_fmt == SNDRV_PCM_FORMAT_DSD_U16_BE
        || params_fmt == SNDRV_PCM_FORMAT_DSD_U32_LE
        || params_fmt == SNDRV_PCM_FORMAT_DSD_U32_BE
    {
        dsd_bclk = nfs1 * params_physical_width(params);
        match dsd_bclk {
            2822400 => {
                dsdsel0 = 0;
                dsdsel1 = 0;
            }
            5644800 => {
                dsdsel0 = 1;
                dsdsel1 = 0;
            }
            11289600 => {
                dsdsel0 = 0;
                dsdsel1 = 1;
            }
            22579200 => {
                if (*(*ak4458).drvdata).type_ == ak4458_type::AK4497 {
                    dsdsel0 = 1;
                    dsdsel1 = 1;
                } else {
                    dev_err((*dai).dev, b"DSD512 not supported.\n\0".as_ptr() as *const c_char);
                    return -EINVAL;
                }
            }
            _ => {
                dev_err((*dai).dev, b"Unsupported dsd bclk.\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
        }

        snd_soc_component_update_bits(
            component,
            AK4458_06_DSD1,
            AK4458_DSDSEL_MASK,
            dsdsel0 as c_uint,
        );
        snd_soc_component_update_bits(
            component,
            AK4458_09_DSD2,
            AK4458_DSDSEL_MASK,
            dsdsel1 as c_uint,
        );
    }

    /* Master Clock Frequency Auto Setting Mode Enable */
    snd_soc_component_update_bits(component, AK4458_00_CONTROL1, 0x80, 0x80);

    match pcm_width {
        16 => {
            if (*ak4458).fmt == SND_SOC_DAIFMT_I2S as c_int {
                format = AK4458_DIF_24BIT_I2S as u8;
            } else {
                format = AK4458_DIF_16BIT_LSB as u8;
            }
        }
        32 => {
            if (*ak4458).fmt == SND_SOC_DAIFMT_I2S as c_int {
                format = AK4458_DIF_32BIT_I2S as u8;
            } else if (*ak4458).fmt == SND_SOC_DAIFMT_LEFT_J as c_int {
                format = AK4458_DIF_32BIT_MSB as u8;
            } else if (*ak4458).fmt == SND_SOC_DAIFMT_RIGHT_J as c_int {
                format = AK4458_DIF_32BIT_LSB as u8;
            } else if (*ak4458).fmt == SND_SOC_DAIFMT_DSP_B as c_int {
                format = AK4458_DIF_32BIT_MSB as u8;
            } else if (*ak4458).fmt == SND_SOC_DAIFMT_PDM as c_int {
                format = AK4458_DIF_32BIT_MSB as u8;
            } else {
                return -EINVAL;
            }
        }
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, AK4458_00_CONTROL1, AK4458_DIF_MASK, format as c_uint);

    /*
     * Enable/disable Daisy Chain if in TDM mode and the number of played
     * channels is bigger than the maximum supported number of channels
     */
    dchn = if ak4458_get_tdm_mode(ak4458) != 0
        && (*ak4458).fmt == SND_SOC_DAIFMT_DSP_B as c_int
        && channels > channels_max
    {
        AK4458_DCHAIN_MASK as u8
    } else {
        0
    };

    snd_soc_component_update_bits(
        component,
        AK4458_0B_CONTROL7,
        AK4458_DCHAIN_MASK,
        dchn as c_uint,
    );

    if (*(*ak4458).drvdata).type_ == ak4458_type::AK4497 {
        ret = snd_soc_component_update_bits(
            component,
            AK4458_09_DSD2,
            0x4,
            ((*ak4458).dsd_path << 2) as c_uint,
        );
        if ret < 0 {
            return ret;
        }
    }

    ret = ak4458_rstn_control(component, 0);
    if ret != 0 {
        return ret;
    }

    ret = ak4458_rstn_control(component, 1);
    if ret != 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn ak4458_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let ak4458 = snd_soc_component_get_drvdata(component) as *mut ak4458_priv;
    let mut ret: c_int;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        SND_SOC_DAIFMT_CBP_CFP | SND_SOC_DAIFMT_CBC_CFP | SND_SOC_DAIFMT_CBP_CFC | _ => {
            dev_err(
                (*component).dev,
                b"Clock provider mode unsupported\n\0".as_ptr() as *const c_char,
            );
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S
        | SND_SOC_DAIFMT_LEFT_J
        | SND_SOC_DAIFMT_RIGHT_J
        | SND_SOC_DAIFMT_DSP_B
        | SND_SOC_DAIFMT_PDM => {
            (*ak4458).fmt = (fmt & SND_SOC_DAIFMT_FORMAT_MASK) as c_int;
        }
        _ => {
            dev_err(
                (*component).dev,
                b"Audio format 0x%02X unsupported\n\0".as_ptr() as *const c_char,
                fmt & SND_SOC_DAIFMT_FORMAT_MASK,
            );
            return -EINVAL;
        }
    }

    /* DSD mode */
    snd_soc_component_update_bits(
        component,
        AK4458_02_CONTROL3,
        AK4458_DP_MASK,
        if (*ak4458).fmt == SND_SOC_DAIFMT_PDM as c_int {
            AK4458_DP_MASK
        } else {
            0
        },
    );

    ret = ak4458_rstn_control(component, 0);
    if ret != 0 {
        return ret;
    }

    ret = ak4458_rstn_control(component, 1);
    if ret != 0 {
        return ret;
    }

    0
}

static att_speed: [c_int; 4] = [4080, 2040, 510, 255];

unsafe extern "C" fn ak4458_set_dai_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*dai).component;
    let ak4458 = snd_soc_component_get_drvdata(component) as *mut ak4458_priv;
    let nfs: c_int;
    let ndt: c_int;
    let reg: c_int;
    let ats: c_int;

    nfs = (*ak4458).fs;

    reg = snd_soc_component_read(component, AK4458_0B_CONTROL7);
    ats = (reg & AK4458_ATS_MASK as c_int) >> AK4458_ATS_SHIFT;

    ndt = att_speed[ats as usize] / (nfs / 1000);

    if mute != 0 {
        snd_soc_component_update_bits(component, AK4458_01_CONTROL2, 0x01, 1);
        mdelay(ndt as c_uint);
        if !(*ak4458).mute_gpiod.is_null() {
            gpiod_set_value_cansleep((*ak4458).mute_gpiod, 1);
        }
    } else {
        if !(*ak4458).mute_gpiod.is_null() {
            gpiod_set_value_cansleep((*ak4458).mute_gpiod, 0);
        }
        snd_soc_component_update_bits(component, AK4458_01_CONTROL2, 0x01, 0);
        mdelay(ndt as c_uint);
    }

    0
}

unsafe extern "C" fn ak4458_set_tdm_slot(
    dai: *mut snd_soc_dai,
    _tx_mask: c_uint,
    _rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let ak4458 = snd_soc_component_get_drvdata(component) as *mut ak4458_priv;
    let mode: c_int;

    (*ak4458).slots = slots;
    (*ak4458).slot_width = slot_width;

    mode = ak4458_get_tdm_mode(ak4458) << AK4458_MODE_SHIFT;

    snd_soc_component_update_bits(
        component,
        AK4458_0A_CONTROL6,
        AK4458_MODE_MASK,
        mode as c_uint,
    );

    0
}

const AK4458_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE
    | SNDRV_PCM_FMTBIT_DSD_U8
    | SNDRV_PCM_FMTBIT_DSD_U16_LE
    | SNDRV_PCM_FMTBIT_DSD_U32_LE;

static ak4458_rates: [c_uint; 17] = [
    8000, 11025, 16000, 22050, 32000, 44100, 48000, 88200, 96000, 176400, 192000, 352800,
    384000, 705600, 768000, 1411200, 2822400,
];

#[repr(C)]
struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *const c_uint,
}

static ak4458_rate_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: ak4458_rates.len() as c_uint,
    list: ak4458_rates.as_ptr(),
};

unsafe extern "C" fn ak4458_startup(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) -> c_int {
    snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &ak4458_rate_constraints,
    )
}

static ak4458_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(ak4458_startup),
    hw_params: Some(ak4458_hw_params),
    set_fmt: Some(ak4458_set_dai_fmt),
    mute_stream: Some(ak4458_set_dai_mute),
    set_tdm_slot: Some(ak4458_set_tdm_slot),
    no_capture_mute: 1,
};

static mut ak4458_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"ak4458-aif\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: AK4458_FORMATS,
    },
    ops: &ak4458_dai_ops,
};

static mut ak4497_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"ak4497-aif\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: AK4458_FORMATS,
    },
    ops: &ak4458_dai_ops,
};

unsafe extern "C" fn ak4458_reset(ak4458: *mut ak4458_priv, active: bool) {
    if !IS_ERR_OR_NULL((*ak4458).reset as *const c_void) {
        if active {
            reset_control_assert((*ak4458).reset);
        } else {
            reset_control_deassert((*ak4458).reset);
        }
        usleep_range(1000, 2000);
    }
}

unsafe extern "C" fn ak4458_runtime_suspend(dev: *mut device) -> c_int {
    let ak4458 = dev_get_drvdata(dev) as *mut ak4458_priv;

    regcache_cache_only((*ak4458).regmap, true);

    ak4458_reset(ak4458, true);

    if !(*ak4458).mute_gpiod.is_null() {
        gpiod_set_value_cansleep((*ak4458).mute_gpiod, 0);
    }

    regulator_bulk_disable((*ak4458).supplies.len() as c_int, (*ak4458).supplies.as_mut_ptr());
    0
}

unsafe extern "C" fn ak4458_runtime_resume(dev: *mut device) -> c_int {
    let ak4458 = dev_get_drvdata(dev) as *mut ak4458_priv;
    let mut ret: c_int;

    ret = regulator_bulk_enable((*ak4458).supplies.len() as c_int, (*ak4458).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(
            (*ak4458).dev,
            b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    if !(*ak4458).mute_gpiod.is_null() {
        gpiod_set_value_cansleep((*ak4458).mute_gpiod, 1);
    }

    ak4458_reset(ak4458, false);

    regcache_cache_only((*ak4458).regmap, false);
    regcache_mark_dirty((*ak4458).regmap);

    ret = regcache_sync((*ak4458).regmap);
    if ret != 0 {
        regcache_cache_only((*ak4458).regmap, true);
        regulator_bulk_disable((*ak4458).supplies.len() as c_int, (*ak4458).supplies.as_mut_ptr());
        return ret;
    }

    0
}

static soc_codec_dev_ak4458: snd_soc_component_driver = snd_soc_component_driver {
    controls: unsafe { ak4458_snd_controls.as_ptr() },
    num_controls: 13,
    dapm_widgets: unsafe { ak4458_dapm_widgets.as_ptr() },
    num_dapm_widgets: 9,
    dapm_routes: unsafe { ak4458_intercon.as_ptr() },
    num_dapm_routes: 8,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static soc_codec_dev_ak4497: snd_soc_component_driver = snd_soc_component_driver {
    controls: unsafe { ak4497_snd_controls.as_ptr() },
    num_controls: 6,
    dapm_widgets: unsafe { ak4497_dapm_widgets.as_ptr() },
    num_dapm_widgets: 3,
    dapm_routes: unsafe { ak4497_intercon.as_ptr() },
    num_dapm_routes: 2,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
    cache_type: c_uint,
}

static ak4458_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: AK4458_14_R4CHATT,
    reg_defaults: ak4458_reg_defaults.as_ptr(),
    num_reg_defaults: ak4458_reg_defaults.len() as c_uint,
    cache_type: REGCACHE_RBTREE,
};

static ak4458_drvdata: ak4458_drvdata = ak4458_drvdata {
    dai_drv: unsafe { &mut ak4458_dai },
    comp_drv: &soc_codec_dev_ak4458,
    type_: ak4458_type::AK4458,
};

static ak4497_drvdata: ak4458_drvdata = ak4458_drvdata {
    dai_drv: unsafe { &mut ak4497_dai },
    comp_drv: &soc_codec_dev_ak4497,
    type_: ak4458_type::AK4497,
};

// RUNTIME_PM_OPS(ak4458_runtime_suspend, ak4458_runtime_resume, NULL)
// SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
extern "C" {
    static ak4458_pm: dev_pm_ops;
}

unsafe extern "C" fn ak4458_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let ak4458: *mut ak4458_priv;
    let mut ret: c_int;
    let mut i: c_int;

    ak4458 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<ak4458_priv>(),
        GFP_KERNEL,
    ) as *mut ak4458_priv;
    if ak4458.is_null() {
        return -ENOMEM;
    }

    (*ak4458).regmap = devm_regmap_init_i2c(i2c, &ak4458_regmap);
    if IS_ERR((*ak4458).regmap as *const c_void) {
        return PTR_ERR((*ak4458).regmap as *const c_void);
    }

    i2c_set_clientdata(i2c, ak4458 as *mut c_void);
    (*ak4458).dev = &mut (*i2c).dev;

    (*ak4458).drvdata = of_device_get_match_data(&mut (*i2c).dev) as *const ak4458_drvdata;

    (*ak4458).reset = devm_reset_control_get_optional_shared((*ak4458).dev, ptr::null());
    if IS_ERR((*ak4458).reset as *const c_void) {
        return PTR_ERR((*ak4458).reset as *const c_void);
    }

    (*ak4458).mute_gpiod = devm_gpiod_get_optional(
        (*ak4458).dev,
        b"mute\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR((*ak4458).mute_gpiod as *const c_void) {
        return PTR_ERR((*ak4458).mute_gpiod as *const c_void);
    }

    /* Optional property for ak4497 */
    of_property_read_u32(
        (*i2c).dev.of_node,
        b"dsd-path\0".as_ptr() as *const c_char,
        &mut (*ak4458).dsd_path,
    );

    i = 0;
    while i < (*ak4458).supplies.len() as c_int {
        (*ak4458).supplies[i as usize].supply = ak4458_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        (*ak4458).dev,
        (*ak4458).supplies.len() as c_int,
        (*ak4458).supplies.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err(
            (*ak4458).dev,
            b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = devm_snd_soc_register_component(
        (*ak4458).dev,
        (*(*ak4458).drvdata).comp_drv,
        (*(*ak4458).drvdata).dai_drv,
        1,
    );
    if ret < 0 {
        dev_err(
            (*ak4458).dev,
            b"Failed to register CODEC: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    pm_runtime_enable(&mut (*i2c).dev);
    regcache_cache_only((*ak4458).regmap, true);

    0
}

unsafe extern "C" fn ak4458_i2c_remove(i2c: *mut i2c_client) {
    pm_runtime_disable(&mut (*i2c).dev);
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

static ak4458_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: b"asahi-kasei,ak4458\0".as_ptr() as *const c_char,
        data: &ak4458_drvdata as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"asahi-kasei,ak4497\0".as_ptr() as *const c_char,
        data: &ak4497_drvdata as *const _ as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, ak4458_of_match);

#[repr(C)]
struct i2c_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
    of_match_table: *const of_device_id,
}

static mut ak4458_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"ak4458\0".as_ptr() as *const c_char,
        pm: unsafe { pm_ptr(&ak4458_pm) },
        of_match_table: ak4458_of_match.as_ptr(),
    },
    probe: Some(ak4458_i2c_probe),
    remove: Some(ak4458_i2c_remove),
};

// module_i2c_driver(ak4458_i2c_driver);
// MODULE_AUTHOR("Junichi Wakasugi <wakasugi.jb@om.asahi-kasei.co.jp>");
// MODULE_AUTHOR("Mihai Serban <mihai.serban@nxp.com>");
// MODULE_DESCRIPTION("ASoC AK4458 DAC driver");
// MODULE_LICENSE("GPL v2");

#[repr(C)]
struct device {
    of_node: *mut device_node,
}
#[repr(C)]
struct i2c_client {
    dev: device,
}
#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}
#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
    driver: *mut snd_soc_dai_driver,
    dev: *mut device,
}
#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}
#[repr(C)]
union snd_ctl_elem_value_value {
    enumerated: snd_ctl_elem_value_enumerated,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_enumerated {
    item: [c_uint; 4],
}
#[repr(C)]
struct regulator_bulk_data {
    supply: *const c_char,
}

enum regmap {}
enum reset_control {}
enum gpio_desc {}
enum snd_kcontrol {}
enum snd_pcm_hw_params {}
enum snd_pcm_runtime {}
enum device_node {}
enum snd_kcontrol_new {}
enum snd_soc_dapm_widget {}
enum snd_soc_dapm_route {}
enum soc_enum {}
enum dev_pm_ops {}

extern "C" {
    static AK4458_00_CONTROL1: c_uint;
    static AK4458_01_CONTROL2: c_uint;
    static AK4458_02_CONTROL3: c_uint;
    static AK4458_05_CONTROL4: c_uint;
    static AK4458_06_DSD1: c_uint;
    static AK4458_09_DSD2: c_uint;
    static AK4458_0A_CONTROL6: c_uint;
    static AK4458_0B_CONTROL7: c_uint;
    static AK4458_14_R4CHATT: c_uint;
    static AK4458_SD_MASK: c_uint;
    static AK4458_SLOW_MASK: c_uint;
    static AK4458_SSLOW_MASK: c_uint;
    static AK4458_RSTN_MASK: c_uint;
    static AK4458_DSDSEL_MASK: c_uint;
    static AK4458_DIF_MASK: c_uint;
    static AK4458_DCHAIN_MASK: c_uint;
    static AK4458_DP_MASK: c_uint;
}

extern "C" {
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFP: c_uint;
    static SND_SOC_DAIFMT_CBP_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_PDM: c_uint;
}

extern "C" {
    static SNDRV_PCM_FORMAT_DSD_U8: c_int;
    static SNDRV_PCM_FORMAT_DSD_U16_LE: c_int;
    static SNDRV_PCM_FORMAT_DSD_U16_BE: c_int;
    static SNDRV_PCM_FORMAT_DSD_U32_LE: c_int;
    static SNDRV_PCM_FORMAT_DSD_U32_BE: c_int;
}

extern "C" {
    static AK4458_DIF_24BIT_I2S: c_uint;
    static AK4458_DIF_16BIT_LSB: c_uint;
    static AK4458_DIF_32BIT_I2S: c_uint;
    static AK4458_DIF_32BIT_MSB: c_uint;
    static AK4458_DIF_32BIT_LSB: c_uint;
    static AK4458_ATS_MASK: c_uint;
    static AK4458_ATS_SHIFT: c_int;
    static AK4458_MODE_SHIFT: c_int;
    static AK4458_MODE_MASK: c_uint;
}

extern "C" {
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_FMTBIT_DSD_U8: u64;
    static SNDRV_PCM_FMTBIT_DSD_U16_LE: u64;
    static SNDRV_PCM_FMTBIT_DSD_U32_LE: u64;
    static SNDRV_PCM_RATE_KNOT: c_uint;
    static SNDRV_PCM_HW_PARAM_RATE: c_uint;
    static REGCACHE_RBTREE: c_uint;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_LOW: c_int;
}

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_int;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn mdelay(msecs: c_uint);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_reset_control_get_optional_shared(
        dev: *mut device,
        id: *const c_char,
    ) -> *mut reset_control;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn of_property_read_u32(
        np: *mut device_node,
        propname: *const c_char,
        out_value: *mut u32,
    ) -> c_int;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_int,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

fn max(a: c_int, b: c_int) -> c_int {
    if a > b { a } else { b }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
