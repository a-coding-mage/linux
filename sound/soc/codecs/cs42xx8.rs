// SPDX-License-Identifier: GPL-2.0
/*
 * Cirrus Logic CS42448/CS42888 Audio CODEC Digital Audio Interface (DAI) driver
 *
 * Copyright (C) 2014 Freescale Semiconductor, Inc.
 *
 * Author: Nicolin Chen <Guangyu.Chen@freescale.com>
 */

// Dependencies from Linux/ALSA headers and "cs42xx8.h" are expected to be
// supplied by the surrounding crate/bindings.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

pub const CS42XX8_NUM_SUPPLIES: usize = 4;
static cs42xx8_supply_names: [*const c_char; CS42XX8_NUM_SUPPLIES] = [
    c"VA".as_ptr(),
    c"VD".as_ptr(),
    c"VLS".as_ptr(),
    c"VLC".as_ptr(),
];

pub const CS42XX8_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

/* codec private data */
#[repr(C)]
pub struct cs42xx8_priv {
    supplies: [regulator_bulk_data; CS42XX8_NUM_SUPPLIES],
    drvdata: *const cs42xx8_driver_data,
    regmap: *mut regmap,
    clk: *mut clk,

    slave_mode: bool,
    is_tdm_mode: bool,
    sysclk: c_ulong,
    tx_channels: u32,
    gpiod_reset: *mut gpio_desc,
    rate: [u32; 2],
}

/* -127.5dB to 0dB with step of 0.5dB */
static dac_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-12750, 50, 1);
/* -64dB to 24dB with step of 0.5dB */
static adc_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-6400, 50, 0);

static cs42xx8_adc_single: [*const c_char; 2] = [c"Differential".as_ptr(), c"Single-Ended".as_ptr()];
static cs42xx8_szc: [*const c_char; 4] = [
    c"Immediate Change".as_ptr(),
    c"Zero Cross".as_ptr(),
    c"Soft Ramp".as_ptr(),
    c"Soft Ramp on Zero Cross".as_ptr(),
];

static adc1_single_enum: soc_enum = SOC_ENUM_SINGLE!(CS42XX8_ADCCTL, 4, 2, cs42xx8_adc_single);
static adc2_single_enum: soc_enum = SOC_ENUM_SINGLE!(CS42XX8_ADCCTL, 3, 2, cs42xx8_adc_single);
static adc3_single_enum: soc_enum = SOC_ENUM_SINGLE!(CS42XX8_ADCCTL, 2, 2, cs42xx8_adc_single);
static dac_szc_enum: soc_enum = SOC_ENUM_SINGLE!(CS42XX8_TXCTL, 5, 4, cs42xx8_szc);
static adc_szc_enum: soc_enum = SOC_ENUM_SINGLE!(CS42XX8_TXCTL, 0, 4, cs42xx8_szc);

static cs42xx8_snd_controls: [snd_kcontrol_new; 22] = [
    SOC_DOUBLE_R_TLV!(c"DAC1 Playback Volume".as_ptr(), CS42XX8_VOLAOUT1, CS42XX8_VOLAOUT2, 0, 0xff, 1, dac_tlv),
    SOC_DOUBLE_R_TLV!(c"DAC2 Playback Volume".as_ptr(), CS42XX8_VOLAOUT3, CS42XX8_VOLAOUT4, 0, 0xff, 1, dac_tlv),
    SOC_DOUBLE_R_TLV!(c"DAC3 Playback Volume".as_ptr(), CS42XX8_VOLAOUT5, CS42XX8_VOLAOUT6, 0, 0xff, 1, dac_tlv),
    SOC_DOUBLE_R_TLV!(c"DAC4 Playback Volume".as_ptr(), CS42XX8_VOLAOUT7, CS42XX8_VOLAOUT8, 0, 0xff, 1, dac_tlv),
    SOC_DOUBLE_R_S_TLV!(c"ADC1 Capture Volume".as_ptr(), CS42XX8_VOLAIN1, CS42XX8_VOLAIN2, 0, -0x80, 0x30, 7, 0, adc_tlv),
    SOC_DOUBLE_R_S_TLV!(c"ADC2 Capture Volume".as_ptr(), CS42XX8_VOLAIN3, CS42XX8_VOLAIN4, 0, -0x80, 0x30, 7, 0, adc_tlv),
    SOC_DOUBLE!(c"DAC1 Invert Switch".as_ptr(), CS42XX8_DACINV, 0, 1, 1, 0),
    SOC_DOUBLE!(c"DAC2 Invert Switch".as_ptr(), CS42XX8_DACINV, 2, 3, 1, 0),
    SOC_DOUBLE!(c"DAC3 Invert Switch".as_ptr(), CS42XX8_DACINV, 4, 5, 1, 0),
    SOC_DOUBLE!(c"DAC4 Invert Switch".as_ptr(), CS42XX8_DACINV, 6, 7, 1, 0),
    SOC_DOUBLE!(c"ADC1 Invert Switch".as_ptr(), CS42XX8_ADCINV, 0, 1, 1, 0),
    SOC_DOUBLE!(c"ADC2 Invert Switch".as_ptr(), CS42XX8_ADCINV, 2, 3, 1, 0),
    SOC_SINGLE!(c"ADC High-Pass Filter Switch".as_ptr(), CS42XX8_ADCCTL, 7, 1, 1),
    SOC_SINGLE!(c"DAC De-emphasis Switch".as_ptr(), CS42XX8_ADCCTL, 5, 1, 0),
    SOC_ENUM!(c"ADC1 Single Ended Mode Switch".as_ptr(), adc1_single_enum),
    SOC_ENUM!(c"ADC2 Single Ended Mode Switch".as_ptr(), adc2_single_enum),
    SOC_SINGLE!(c"DAC Single Volume Control Switch".as_ptr(), CS42XX8_TXCTL, 7, 1, 0),
    SOC_ENUM!(c"DAC Soft Ramp & Zero Cross Control Switch".as_ptr(), dac_szc_enum),
    SOC_SINGLE!(c"DAC Auto Mute Switch".as_ptr(), CS42XX8_TXCTL, 4, 1, 0),
    SOC_SINGLE!(c"Mute ADC Serial Port Switch".as_ptr(), CS42XX8_TXCTL, 3, 1, 0),
    SOC_SINGLE!(c"ADC Single Volume Control Switch".as_ptr(), CS42XX8_TXCTL, 2, 1, 0),
    SOC_ENUM!(c"ADC Soft Ramp & Zero Cross Control Switch".as_ptr(), adc_szc_enum),
];

static cs42xx8_adc3_snd_controls: [snd_kcontrol_new; 3] = [
    SOC_DOUBLE_R_S_TLV!(c"ADC3 Capture Volume".as_ptr(), CS42XX8_VOLAIN5, CS42XX8_VOLAIN6, 0, -0x80, 0x30, 7, 0, adc_tlv),
    SOC_DOUBLE!(c"ADC3 Invert Switch".as_ptr(), CS42XX8_ADCINV, 4, 5, 1, 0),
    SOC_ENUM!(c"ADC3 Single Ended Mode Switch".as_ptr(), adc3_single_enum),
];

static cs42xx8_dapm_widgets: [snd_soc_dapm_widget; 21] = [
    SND_SOC_DAPM_DAC!(c"DAC1".as_ptr(), c"Playback".as_ptr(), CS42XX8_PWRCTL, 1, 1),
    SND_SOC_DAPM_DAC!(c"DAC2".as_ptr(), c"Playback".as_ptr(), CS42XX8_PWRCTL, 2, 1),
    SND_SOC_DAPM_DAC!(c"DAC3".as_ptr(), c"Playback".as_ptr(), CS42XX8_PWRCTL, 3, 1),
    SND_SOC_DAPM_DAC!(c"DAC4".as_ptr(), c"Playback".as_ptr(), CS42XX8_PWRCTL, 4, 1),
    SND_SOC_DAPM_OUTPUT!(c"AOUT1L".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"AOUT1R".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"AOUT2L".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"AOUT2R".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"AOUT3L".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"AOUT3R".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"AOUT4L".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"AOUT4R".as_ptr()),
    SND_SOC_DAPM_ADC!(c"ADC1".as_ptr(), c"Capture".as_ptr(), CS42XX8_PWRCTL, 5, 1),
    SND_SOC_DAPM_ADC!(c"ADC2".as_ptr(), c"Capture".as_ptr(), CS42XX8_PWRCTL, 6, 1),
    SND_SOC_DAPM_INPUT!(c"AIN1L".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"AIN1R".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"AIN2L".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"AIN2R".as_ptr()),
    SND_SOC_DAPM_SUPPLY!(c"PWR".as_ptr(), CS42XX8_PWRCTL, 0, 1, ptr::null_mut(), 0),
];

static cs42xx8_adc3_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    SND_SOC_DAPM_ADC!(c"ADC3".as_ptr(), c"Capture".as_ptr(), CS42XX8_PWRCTL, 7, 1),
    SND_SOC_DAPM_INPUT!(c"AIN3L".as_ptr()),
    SND_SOC_DAPM_INPUT!(c"AIN3R".as_ptr()),
];

static cs42xx8_dapm_routes: [snd_soc_dapm_route; 18] = [
    /* Playback */
    snd_soc_dapm_route { sink: c"AOUT1L".as_ptr(), control: ptr::null(), source: c"DAC1".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUT1R".as_ptr(), control: ptr::null(), source: c"DAC1".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC1".as_ptr(), control: ptr::null(), source: c"PWR".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUT2L".as_ptr(), control: ptr::null(), source: c"DAC2".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUT2R".as_ptr(), control: ptr::null(), source: c"DAC2".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC2".as_ptr(), control: ptr::null(), source: c"PWR".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUT3L".as_ptr(), control: ptr::null(), source: c"DAC3".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUT3R".as_ptr(), control: ptr::null(), source: c"DAC3".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC3".as_ptr(), control: ptr::null(), source: c"PWR".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUT4L".as_ptr(), control: ptr::null(), source: c"DAC4".as_ptr() },
    snd_soc_dapm_route { sink: c"AOUT4R".as_ptr(), control: ptr::null(), source: c"DAC4".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC4".as_ptr(), control: ptr::null(), source: c"PWR".as_ptr() },
    /* Capture */
    snd_soc_dapm_route { sink: c"ADC1".as_ptr(), control: ptr::null(), source: c"AIN1L".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC1".as_ptr(), control: ptr::null(), source: c"AIN1R".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC1".as_ptr(), control: ptr::null(), source: c"PWR".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC2".as_ptr(), control: ptr::null(), source: c"AIN2L".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC2".as_ptr(), control: ptr::null(), source: c"AIN2R".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC2".as_ptr(), control: ptr::null(), source: c"PWR".as_ptr() },
];

static cs42xx8_adc3_dapm_routes: [snd_soc_dapm_route; 3] = [
    /* Capture */
    snd_soc_dapm_route { sink: c"ADC3".as_ptr(), control: ptr::null(), source: c"AIN3L".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC3".as_ptr(), control: ptr::null(), source: c"AIN3R".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC3".as_ptr(), control: ptr::null(), source: c"PWR".as_ptr() },
];

#[repr(C)]
pub struct cs42xx8_ratios {
    mfreq: c_uint,
    min_mclk: c_uint,
    max_mclk: c_uint,
    ratio: [c_uint; 3],
}

/*
 * According to reference mannual, define the cs42xx8_ratio struct
 * MFreq2 | MFreq1 | MFreq0 |     Description     | SSM | DSM | QSM |
 * 0      | 0      | 0      |1.029MHz to 12.8MHz  | 256 | 128 |  64 |
 * 0      | 0      | 1      |1.536MHz to 19.2MHz  | 384 | 192 |  96 |
 * 0      | 1      | 0      |2.048MHz to 25.6MHz  | 512 | 256 | 128 |
 * 0      | 1      | 1      |3.072MHz to 38.4MHz  | 768 | 384 | 192 |
 * 1      | x      | x      |4.096MHz to 51.2MHz  |1024 | 512 | 256 |
 */
static cs42xx8_ratios: [cs42xx8_ratios; 5] = [
    cs42xx8_ratios { mfreq: 0, min_mclk: 1029000, max_mclk: 12800000, ratio: [256, 128, 64] },
    cs42xx8_ratios { mfreq: 2, min_mclk: 1536000, max_mclk: 19200000, ratio: [384, 192, 96] },
    cs42xx8_ratios { mfreq: 4, min_mclk: 2048000, max_mclk: 25600000, ratio: [512, 256, 128] },
    cs42xx8_ratios { mfreq: 6, min_mclk: 3072000, max_mclk: 38400000, ratio: [768, 384, 192] },
    cs42xx8_ratios { mfreq: 8, min_mclk: 4096000, max_mclk: 51200000, ratio: [1024, 512, 256] },
];

unsafe extern "C" fn cs42xx8_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let cs42xx8 = snd_soc_component_get_drvdata(component) as *mut cs42xx8_priv;

    (*cs42xx8).sysclk = freq as c_ulong;

    0
}

unsafe extern "C" fn cs42xx8_set_dai_fmt(codec_dai: *mut snd_soc_dai, format: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let cs42xx8 = snd_soc_component_get_drvdata(component) as *mut cs42xx8_priv;
    let val: u32;

    (*cs42xx8).is_tdm_mode = false;

    /* Set DAI format */
    match format & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_LEFT_J => {
            val = CS42XX8_INTF_DAC_DIF_LEFTJ | CS42XX8_INTF_ADC_DIF_LEFTJ;
        }
        SND_SOC_DAIFMT_I2S => {
            val = CS42XX8_INTF_DAC_DIF_I2S | CS42XX8_INTF_ADC_DIF_I2S;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            val = CS42XX8_INTF_DAC_DIF_RIGHTJ | CS42XX8_INTF_ADC_DIF_RIGHTJ;
        }
        SND_SOC_DAIFMT_DSP_A => {
            val = CS42XX8_INTF_DAC_DIF_TDM | CS42XX8_INTF_ADC_DIF_TDM;
            (*cs42xx8).is_tdm_mode = true;
        }
        _ => {
            dev_err((*component).dev, c"unsupported dai format\n".as_ptr());
            return -EINVAL;
        }
    }

    regmap_update_bits(
        (*cs42xx8).regmap,
        CS42XX8_INTF,
        CS42XX8_INTF_DAC_DIF_MASK | CS42XX8_INTF_ADC_DIF_MASK,
        val,
    );

    /* Set master/slave audio interface */
    match format & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {
            (*cs42xx8).slave_mode = true;
        }
        SND_SOC_DAIFMT_CBP_CFP => {
            (*cs42xx8).slave_mode = false;
        }
        _ => {
            dev_err((*component).dev, c"unsupported master/slave mode\n".as_ptr());
            return -EINVAL;
        }
    }

    if (*cs42xx8).is_tdm_mode && !(*cs42xx8).slave_mode {
        dev_err((*component).dev, c"TDM mode is supported only in slave mode\n".as_ptr());
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn cs42xx8_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let cs42xx8 = snd_soc_component_get_drvdata(component) as *mut cs42xx8_priv;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let txi = tx as usize;
    let rxi = (!tx) as usize;
    let mut ratio: [u32; 2] = [0; 2];
    let mut rate: [u32; 2] = [0; 2];
    let mut fm: [u32; 2] = [0; 2];
    let mut i: usize;
    let val: u32;
    let mask: u32;
    let mut condition1: bool;
    let mut condition2: bool;

    if tx {
        (*cs42xx8).tx_channels = params_channels(params);
    }

    rate[txi] = params_rate(params);
    rate[rxi] = (*cs42xx8).rate[rxi];

    ratio[txi] = if rate[txi] > 0 { ((*cs42xx8).sysclk as u32) / rate[txi] } else { 0 };
    ratio[rxi] = if rate[rxi] > 0 { ((*cs42xx8).sysclk as u32) / rate[rxi] } else { 0 };

    /* Get functional mode for tx and rx according to rate */
    i = 0;
    while i < 2 {
        if (*cs42xx8).slave_mode {
            fm[i] = CS42XX8_FM_AUTO;
        } else if rate[i] < 50000 {
            fm[i] = CS42XX8_FM_SINGLE;
        } else if rate[i] > 50000 && rate[i] < 100000 {
            fm[i] = CS42XX8_FM_DOUBLE;
        } else if rate[i] > 100000 && rate[i] < 200000 {
            fm[i] = CS42XX8_FM_QUAD;
        } else {
            dev_err((*component).dev, c"unsupported sample rate\n".as_ptr());
            return -EINVAL;
        }
        i += 1;
    }

    i = 0;
    while i < cs42xx8_ratios.len() {
        /* Is the ratio[tx] valid ? */
        condition1 = (if fm[txi] == CS42XX8_FM_AUTO {
            cs42xx8_ratios[i].ratio[0] == ratio[txi]
                || cs42xx8_ratios[i].ratio[1] == ratio[txi]
                || cs42xx8_ratios[i].ratio[2] == ratio[txi]
        } else {
            cs42xx8_ratios[i].ratio[fm[txi] as usize] == ratio[txi]
        }) && (*cs42xx8).sysclk >= cs42xx8_ratios[i].min_mclk as c_ulong
            && (*cs42xx8).sysclk <= cs42xx8_ratios[i].max_mclk as c_ulong;

        if ratio[txi] == 0 {
            condition1 = true;
        }

        /* Is the ratio[!tx] valid ? */
        condition2 = if fm[rxi] == CS42XX8_FM_AUTO {
            cs42xx8_ratios[i].ratio[0] == ratio[rxi]
                || cs42xx8_ratios[i].ratio[1] == ratio[rxi]
                || cs42xx8_ratios[i].ratio[2] == ratio[rxi]
        } else {
            cs42xx8_ratios[i].ratio[fm[rxi] as usize] == ratio[rxi]
        };

        if ratio[rxi] == 0 {
            condition2 = true;
        }

        /*
         * Both ratio[tx] and ratio[!tx] is valid, then we get
         * a proper MFreq.
         */
        if condition1 && condition2 {
            break;
        }
        i += 1;
    }

    if i == cs42xx8_ratios.len() {
        dev_err((*component).dev, c"unsupported sysclk ratio\n".as_ptr());
        return -EINVAL;
    }

    (*cs42xx8).rate[txi] = params_rate(params);

    if (*cs42xx8).is_tdm_mode {
        if (*cs42xx8).sysclk < (256u32.wrapping_mul((*cs42xx8).rate[txi])) as c_ulong {
            dev_err((*component).dev, c"Unsupported sysclk in TDM mode\n".as_ptr());
            return -EINVAL;
        }

        if !tx && (*cs42xx8).rate[txi] > 100000 {
            dev_err(
                (*component).dev,
                c"ADC does not support Quad-Speed Mode in TDM mode\n".as_ptr(),
            );
            return -EINVAL;
        }
    }

    mask = CS42XX8_FUNCMOD_MFREQ_MASK;
    val = cs42xx8_ratios[i].mfreq;

    regmap_update_bits(
        (*cs42xx8).regmap,
        CS42XX8_FUNCMOD,
        CS42XX8_FUNCMOD_xC_FM_MASK!(tx) | mask,
        CS42XX8_FUNCMOD_xC_FM!(tx, fm[txi]) | val,
    );

    0
}

unsafe extern "C" fn cs42xx8_hw_free(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let cs42xx8 = snd_soc_component_get_drvdata(component) as *mut cs42xx8_priv;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let txi = tx as usize;

    /* Clear stored rate */
    (*cs42xx8).rate[txi] = 0;

    regmap_update_bits(
        (*cs42xx8).regmap,
        CS42XX8_FUNCMOD,
        CS42XX8_FUNCMOD_xC_FM_MASK!(tx),
        CS42XX8_FUNCMOD_xC_FM!(tx, CS42XX8_FM_AUTO),
    );
    0
}

unsafe extern "C" fn cs42xx8_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*dai).component;
    let cs42xx8 = snd_soc_component_get_drvdata(component) as *mut cs42xx8_priv;
    let dac_unmute: u8 = if (*cs42xx8).tx_channels != 0 {
        !(((0x1u32 << (*cs42xx8).tx_channels) - 1) as u8)
    } else {
        0
    };

    regmap_write(
        (*cs42xx8).regmap,
        CS42XX8_DACMUTE,
        if mute != 0 { CS42XX8_DACMUTE_ALL } else { dac_unmute as u32 },
    );

    0
}

static cs42xx8_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(cs42xx8_set_dai_fmt),
    set_sysclk: Some(cs42xx8_set_dai_sysclk),
    hw_params: Some(cs42xx8_hw_params),
    hw_free: Some(cs42xx8_hw_free),
    mute_stream: Some(cs42xx8_mute),
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

static mut cs42xx8_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: CS42XX8_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: CS42XX8_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &cs42xx8_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

static cs42xx8_reg: [reg_default; 25] = [
    reg_default { reg: 0x02, def: 0x00 },   /* Power Control */
    reg_default { reg: 0x03, def: 0xF0 },   /* Functional Mode */
    reg_default { reg: 0x04, def: 0x46 },   /* Interface Formats */
    reg_default { reg: 0x05, def: 0x00 },   /* ADC Control & DAC De-Emphasis */
    reg_default { reg: 0x06, def: 0x10 },   /* Transition Control */
    reg_default { reg: 0x07, def: 0x00 },   /* DAC Channel Mute */
    reg_default { reg: 0x08, def: 0x00 },   /* Volume Control AOUT1 */
    reg_default { reg: 0x09, def: 0x00 },   /* Volume Control AOUT2 */
    reg_default { reg: 0x0a, def: 0x00 },   /* Volume Control AOUT3 */
    reg_default { reg: 0x0b, def: 0x00 },   /* Volume Control AOUT4 */
    reg_default { reg: 0x0c, def: 0x00 },   /* Volume Control AOUT5 */
    reg_default { reg: 0x0d, def: 0x00 },   /* Volume Control AOUT6 */
    reg_default { reg: 0x0e, def: 0x00 },   /* Volume Control AOUT7 */
    reg_default { reg: 0x0f, def: 0x00 },   /* Volume Control AOUT8 */
    reg_default { reg: 0x10, def: 0x00 },   /* DAC Channel Invert */
    reg_default { reg: 0x11, def: 0x00 },   /* Volume Control AIN1 */
    reg_default { reg: 0x12, def: 0x00 },   /* Volume Control AIN2 */
    reg_default { reg: 0x13, def: 0x00 },   /* Volume Control AIN3 */
    reg_default { reg: 0x14, def: 0x00 },   /* Volume Control AIN4 */
    reg_default { reg: 0x15, def: 0x00 },   /* Volume Control AIN5 */
    reg_default { reg: 0x16, def: 0x00 },   /* Volume Control AIN6 */
    reg_default { reg: 0x17, def: 0x00 },   /* ADC Channel Invert */
    reg_default { reg: 0x18, def: 0x00 },   /* Status Control */
    reg_default { reg: 0x1a, def: 0x00 },   /* Status Mask */
    reg_default { reg: 0x1b, def: 0x00 },   /* MUTEC Pin Control */
];

unsafe extern "C" fn cs42xx8_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS42XX8_STATUS => true,
        _ => false,
    }
}

unsafe extern "C" fn cs42xx8_writeable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS42XX8_CHIPID | CS42XX8_STATUS => false,
        _ => true,
    }
}

#[no_mangle]
pub static cs42xx8_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,

    max_register: CS42XX8_LASTREG,
    reg_defaults: cs42xx8_reg.as_ptr(),
    num_reg_defaults: cs42xx8_reg.len() as c_uint,
    volatile_reg: Some(cs42xx8_volatile_register),
    writeable_reg: Some(cs42xx8_writeable_register),
    cache_type: REGCACHE_MAPLE,
    reg_format_endian: REGMAP_ENDIAN_BIG,
    use_single_read: true,
    use_single_write: true,
    ..unsafe { core::mem::zeroed() }
};
// EXPORT_SYMBOL_GPL(cs42xx8_regmap_config);

unsafe extern "C" fn cs42xx8_component_probe(component: *mut snd_soc_component) -> c_int {
    let cs42xx8 = snd_soc_component_get_drvdata(component) as *mut cs42xx8_priv;
    let dapm = snd_soc_component_to_dapm(component);

    match (*(*cs42xx8).drvdata).num_adcs {
        3 => {
            snd_soc_add_component_controls(
                component,
                cs42xx8_adc3_snd_controls.as_ptr(),
                cs42xx8_adc3_snd_controls.len() as c_uint,
            );
            snd_soc_dapm_new_controls(
                dapm,
                cs42xx8_adc3_dapm_widgets.as_ptr(),
                cs42xx8_adc3_dapm_widgets.len() as c_uint,
            );
            snd_soc_dapm_add_routes(
                dapm,
                cs42xx8_adc3_dapm_routes.as_ptr(),
                cs42xx8_adc3_dapm_routes.len() as c_uint,
            );
        }
        _ => {}
    }

    /* Mute all DAC channels */
    regmap_write((*cs42xx8).regmap, CS42XX8_DACMUTE, CS42XX8_DACMUTE_ALL);

    0
}

static cs42xx8_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs42xx8_component_probe),
    controls: cs42xx8_snd_controls.as_ptr(),
    num_controls: cs42xx8_snd_controls.len() as c_uint,
    dapm_widgets: cs42xx8_dapm_widgets.as_ptr(),
    num_dapm_widgets: cs42xx8_dapm_widgets.len() as c_uint,
    dapm_routes: cs42xx8_dapm_routes.as_ptr(),
    num_dapm_routes: cs42xx8_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

#[no_mangle]
pub static cs42448_data: cs42xx8_driver_data = cs42xx8_driver_data {
    name: c"cs42448".as_ptr(),
    num_adcs: 3,
};
// EXPORT_SYMBOL_GPL(cs42448_data);

#[no_mangle]
pub static cs42888_data: cs42xx8_driver_data = cs42xx8_driver_data {
    name: c"cs42888".as_ptr(),
    num_adcs: 2,
};
// EXPORT_SYMBOL_GPL(cs42888_data);

#[no_mangle]
pub unsafe extern "C" fn cs42xx8_probe(
    dev: *mut device,
    regmap: *mut regmap,
    drvdata: *mut cs42xx8_driver_data,
) -> c_int {
    let cs42xx8: *mut cs42xx8_priv;
    let mut ret: c_int;
    let mut val: c_int = 0;
    let mut i: usize;

    if IS_ERR(regmap as *const c_void) {
        ret = PTR_ERR(regmap as *const c_void) as c_int;
        dev_err(dev, c"failed to allocate regmap: %d\n".as_ptr(), ret);
        return ret;
    }

    cs42xx8 = devm_kzalloc(dev, core::mem::size_of::<cs42xx8_priv>(), GFP_KERNEL) as *mut cs42xx8_priv;
    if cs42xx8.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, cs42xx8 as *mut c_void);

    (*cs42xx8).regmap = regmap;

    (*cs42xx8).drvdata = drvdata;

    (*cs42xx8).gpiod_reset = devm_gpiod_get_optional(dev, c"reset".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*cs42xx8).gpiod_reset as *const c_void) {
        return PTR_ERR((*cs42xx8).gpiod_reset as *const c_void) as c_int;
    }

    gpiod_set_value_cansleep((*cs42xx8).gpiod_reset, 0);

    (*cs42xx8).clk = devm_clk_get(dev, c"mclk".as_ptr());
    if IS_ERR((*cs42xx8).clk as *const c_void) {
        dev_err(
            dev,
            c"failed to get the clock: %ld\n".as_ptr(),
            PTR_ERR((*cs42xx8).clk as *const c_void),
        );
        return -EINVAL;
    }

    (*cs42xx8).sysclk = clk_get_rate((*cs42xx8).clk);

    i = 0;
    while i < (*cs42xx8).supplies.len() {
        (*cs42xx8).supplies[i].supply = cs42xx8_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        dev,
        (*cs42xx8).supplies.len() as c_int,
        (*cs42xx8).supplies.as_mut_ptr(),
    );
    if ret != 0 {
        dev_err(dev, c"failed to request supplies: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = regulator_bulk_enable((*cs42xx8).supplies.len() as c_int, (*cs42xx8).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, c"failed to enable supplies: %d\n".as_ptr(), ret);
        return ret;
    }

    /* Make sure hardware reset done */
    msleep(5);

    /* Validate the chip ID */
    ret = regmap_read((*cs42xx8).regmap, CS42XX8_CHIPID, &mut val);
    if ret < 0 {
        dev_err(dev, c"failed to get device ID, ret = %d".as_ptr(), ret);
        regulator_bulk_disable((*cs42xx8).supplies.len() as c_int, (*cs42xx8).supplies.as_mut_ptr());
        return ret;
    }

    /* The top four bits of the chip ID should be 0000 */
    if ((val as u32 & CS42XX8_CHIPID_CHIP_ID_MASK) >> 4) != 0x00 {
        dev_err(
            dev,
            c"unmatched chip ID: %d\n".as_ptr(),
            (val as u32 & CS42XX8_CHIPID_CHIP_ID_MASK) >> 4,
        );
        ret = -EINVAL;
        regulator_bulk_disable((*cs42xx8).supplies.len() as c_int, (*cs42xx8).supplies.as_mut_ptr());
        return ret;
    }

    dev_info(
        dev,
        c"found device, revision %X\n".as_ptr(),
        val as u32 & CS42XX8_CHIPID_REV_ID_MASK,
    );

    cs42xx8_dai.name = (*(*cs42xx8).drvdata).name;

    /* Each adc supports stereo input */
    cs42xx8_dai.capture.channels_max = (*(*cs42xx8).drvdata).num_adcs * 2;

    ret = devm_snd_soc_register_component(dev, &cs42xx8_driver, &mut cs42xx8_dai, 1);
    if ret != 0 {
        dev_err(dev, c"failed to register component:%d\n".as_ptr(), ret);
        regulator_bulk_disable((*cs42xx8).supplies.len() as c_int, (*cs42xx8).supplies.as_mut_ptr());
        return ret;
    }

    regcache_cache_only((*cs42xx8).regmap, true);

    regulator_bulk_disable((*cs42xx8).supplies.len() as c_int, (*cs42xx8).supplies.as_mut_ptr());

    ret
}
// EXPORT_SYMBOL_GPL(cs42xx8_probe);

unsafe extern "C" fn cs42xx8_runtime_resume(dev: *mut device) -> c_int {
    let cs42xx8 = dev_get_drvdata(dev) as *mut cs42xx8_priv;
    let mut ret: c_int;

    ret = clk_prepare_enable((*cs42xx8).clk);
    if ret != 0 {
        dev_err(dev, c"failed to enable mclk: %d\n".as_ptr(), ret);
        return ret;
    }

    gpiod_set_value_cansleep((*cs42xx8).gpiod_reset, 0);

    ret = regulator_bulk_enable((*cs42xx8).supplies.len() as c_int, (*cs42xx8).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, c"failed to enable supplies: %d\n".as_ptr(), ret);
        clk_disable_unprepare((*cs42xx8).clk);
        return ret;
    }

    /* Make sure hardware reset done */
    msleep(5);

    regcache_cache_only((*cs42xx8).regmap, false);
    regcache_mark_dirty((*cs42xx8).regmap);

    ret = regcache_sync((*cs42xx8).regmap);
    if ret != 0 {
        dev_err(dev, c"failed to sync regmap: %d\n".as_ptr(), ret);
        regulator_bulk_disable((*cs42xx8).supplies.len() as c_int, (*cs42xx8).supplies.as_mut_ptr());
        clk_disable_unprepare((*cs42xx8).clk);
        return ret;
    }

    0
}

unsafe extern "C" fn cs42xx8_runtime_suspend(dev: *mut device) -> c_int {
    let cs42xx8 = dev_get_drvdata(dev) as *mut cs42xx8_priv;

    regcache_cache_only((*cs42xx8).regmap, true);

    regulator_bulk_disable((*cs42xx8).supplies.len() as c_int, (*cs42xx8).supplies.as_mut_ptr());

    gpiod_set_value_cansleep((*cs42xx8).gpiod_reset, 1);

    clk_disable_unprepare((*cs42xx8).clk);

    0
}

EXPORT_GPL_DEV_PM_OPS!(cs42xx8_pm, {
    SYSTEM_SLEEP_PM_OPS!(pm_runtime_force_suspend, pm_runtime_force_resume);
    RUNTIME_PM_OPS!(cs42xx8_runtime_suspend, cs42xx8_runtime_resume, ptr::null_mut());
});

MODULE_DESCRIPTION!(c"Cirrus Logic CS42448/CS42888 ALSA SoC Codec Driver".as_ptr());
MODULE_AUTHOR!(c"Freescale Semiconductor, Inc.".as_ptr());
MODULE_LICENSE!(c"GPL".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
