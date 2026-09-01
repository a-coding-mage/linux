// SPDX-License-Identifier: GPL-2.0-or-later
//
// File:         sound/soc/codecs/ssm2602.c
// Author:       Cliff Cai <Cliff.Cai@analog.com>
//
// Created:      Tue June 06 2008
// Description:  Driver for ssm2602 sound chip
//
// Modified:
//               Copyright 2008 Analog Devices Inc.
//
// Bugs:         Enter bugs at http://blackfin.uclinux.org/

// C dependencies:
// linux/delay.h, linux/module.h, linux/regmap.h, linux/slab.h
// sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/tlv.h
// "ssm2602.h"
use crate::*;

/* codec private data */
#[repr(C)]
pub struct ssm2602_priv {
    pub sysclk: c_uint,
    pub sysclk_constraints: *const snd_pcm_hw_constraint_list,
    pub regmap: *mut regmap,
    pub type_: ssm2602_type,
    pub clk_out_pwr: c_uint,
}

/*
 * ssm2602 register cache
 * We can't read the ssm2602 register space when we are
 * using 2 wire for device control, so we cache them instead.
 * There is no point in caching the reset register
 */
static ssm2602_reg: [reg_default; SSM2602_CACHEREGNUM as usize] = [
    reg_default { reg: 0x00, def: 0x0097 },
    reg_default { reg: 0x01, def: 0x0097 },
    reg_default { reg: 0x02, def: 0x0079 },
    reg_default { reg: 0x03, def: 0x0079 },
    reg_default { reg: 0x04, def: 0x000a },
    reg_default { reg: 0x05, def: 0x0008 },
    reg_default { reg: 0x06, def: 0x009f },
    reg_default { reg: 0x07, def: 0x000a },
    reg_default { reg: 0x08, def: 0x0000 },
    reg_default { reg: 0x09, def: 0x0000 },
];

/*
 * ssm2602 register patch
 * Workaround for playback distortions after power up: activates digital
 * core, and then powers on output, DAC, and whole chip at the same time
 */
static ssm2602_patch: [reg_sequence; 3] = [
    reg_sequence { reg: SSM2602_ACTIVE, def: 0x01 },
    reg_sequence { reg: SSM2602_PWR, def: 0x07 },
    reg_sequence { reg: SSM2602_RESET, def: 0x00 },
];

/*Appending several "None"s just for OSS mixer use*/
static ssm2602_input_select: [*const c_char; 2] = [c_str!("Line"), c_str!("Mic")];

static ssm2602_deemph: [*const c_char; 4] = [
    c_str!("None"),
    c_str!("32Khz"),
    c_str!("44.1Khz"),
    c_str!("48Khz"),
];

static ssm2602_enum: [soc_enum; 2] = [
    SOC_ENUM_SINGLE!(
        SSM2602_APANA,
        2,
        ARRAY_SIZE!(ssm2602_input_select),
        ssm2602_input_select
    ),
    SOC_ENUM_SINGLE!(
        SSM2602_APDIGI,
        1,
        ARRAY_SIZE!(ssm2602_deemph),
        ssm2602_deemph
    ),
];

static ssm260x_outmix_tlv: [c_uint; 0] = DECLARE_TLV_DB_RANGE!(
    0,
    47,
    TLV_DB_SCALE_ITEM!(TLV_DB_GAIN_MUTE, 0, 0),
    48,
    127,
    TLV_DB_SCALE_ITEM!(-7400, 100, 0)
);

static ssm260x_inpga_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-3450, 150, 0);
static ssm260x_sidetone_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-1500, 300, 0);

static ssm260x_snd_controls: [snd_kcontrol_new; 5] = [
    SOC_DOUBLE_R_TLV!(
        c_str!("Capture Volume"),
        SSM2602_LINVOL,
        SSM2602_RINVOL,
        0,
        45,
        0,
        ssm260x_inpga_tlv
    ),
    SOC_DOUBLE_R!(
        c_str!("Capture Switch"),
        SSM2602_LINVOL,
        SSM2602_RINVOL,
        7,
        1,
        1
    ),
    SOC_SINGLE!(c_str!("ADC High Pass Filter Switch"), SSM2602_APDIGI, 0, 1, 1),
    SOC_SINGLE!(c_str!("Store DC Offset Switch"), SSM2602_APDIGI, 4, 1, 0),
    SOC_ENUM!(c_str!("Playback De-emphasis"), ssm2602_enum[1]),
];

static ssm2602_snd_controls: [snd_kcontrol_new; 5] = [
    SOC_DOUBLE_R_TLV!(
        c_str!("Master Playback Volume"),
        SSM2602_LOUT1V,
        SSM2602_ROUT1V,
        0,
        127,
        0,
        ssm260x_outmix_tlv
    ),
    SOC_DOUBLE_R!(
        c_str!("Master Playback ZC Switch"),
        SSM2602_LOUT1V,
        SSM2602_ROUT1V,
        7,
        1,
        0
    ),
    SOC_SINGLE_TLV!(
        c_str!("Sidetone Playback Volume"),
        SSM2602_APANA,
        6,
        3,
        1,
        ssm260x_sidetone_tlv
    ),
    SOC_SINGLE!(c_str!("Mic Boost (+20dB)"), SSM2602_APANA, 0, 1, 0),
    SOC_SINGLE!(c_str!("Mic Boost2 (+20dB)"), SSM2602_APANA, 8, 1, 0),
];

/* Output Mixer */
static ssm260x_output_mixer_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!(c_str!("Line Bypass Switch"), SSM2602_APANA, 3, 1, 0),
    SOC_DAPM_SINGLE!(c_str!("HiFi Playback Switch"), SSM2602_APANA, 4, 1, 0),
    SOC_DAPM_SINGLE!(c_str!("Mic Sidetone Switch"), SSM2602_APANA, 5, 1, 0),
];

static mic_ctl: snd_kcontrol_new = SOC_DAPM_SINGLE!(c_str!("Switch"), SSM2602_APANA, 1, 1, 1);

/* Input mux */
static ssm2602_input_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!(c_str!("Input Select"), ssm2602_enum[0]);

unsafe extern "C" fn ssm2602_mic_switch_event(
    w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    /*
     * According to the ssm2603 data sheet (control register sequencing),
     * the digital core should be activated only after all necessary bits
     * in the power register are enabled, and a delay determined by the
     * decoupling capacitor on the VMID pin has passed. If the digital core
     * is activated too early, or even before the ADC is powered up, audible
     * artifacts appear at the beginning and end of the recorded signal.
     *
     * In practice, audible artifacts disappear well over 500 ms.
     */
    let _ = (w, kcontrol, event);
    unsafe { msleep(500) };

    0
}

static ssm260x_dapm_widgets: [snd_soc_dapm_widget; 8] = [
    SND_SOC_DAPM_DAC!(c_str!("DAC"), c_str!("HiFi Playback"), SSM2602_PWR, 3, 1),
    SND_SOC_DAPM_ADC!(c_str!("ADC"), c_str!("HiFi Capture"), SSM2602_PWR, 2, 1),
    SND_SOC_DAPM_PGA!(c_str!("Line Input"), SSM2602_PWR, 0, 1, NULL, 0),
    SND_SOC_DAPM_SUPPLY!(c_str!("Digital Core Power"), SSM2602_ACTIVE, 0, 0, NULL, 0),
    SND_SOC_DAPM_OUTPUT!(c_str!("LOUT")),
    SND_SOC_DAPM_OUTPUT!(c_str!("ROUT")),
    SND_SOC_DAPM_INPUT!(c_str!("RLINEIN")),
    SND_SOC_DAPM_INPUT!(c_str!("LLINEIN")),
];

static ssm2602_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    SND_SOC_DAPM_MIXER!(
        c_str!("Output Mixer"),
        SSM2602_PWR,
        4,
        1,
        ssm260x_output_mixer_controls,
        ARRAY_SIZE!(ssm260x_output_mixer_controls)
    ),
    SND_SOC_DAPM_MUX!(c_str!("Input Mux"), SND_SOC_NOPM, 0, 0, &ssm2602_input_mux_controls),
    SND_SOC_DAPM_MICBIAS!(c_str!("Mic Bias"), SSM2602_PWR, 1, 1),
    SND_SOC_DAPM_SWITCH_E!(
        c_str!("Mic Switch"),
        SSM2602_APANA,
        1,
        1,
        &mic_ctl,
        ssm2602_mic_switch_event,
        SND_SOC_DAPM_PRE_PMU
    ),
    SND_SOC_DAPM_OUTPUT!(c_str!("LHPOUT")),
    SND_SOC_DAPM_OUTPUT!(c_str!("RHPOUT")),
    SND_SOC_DAPM_INPUT!(c_str!("MICIN")),
];

static ssm2604_dapm_widgets: [snd_soc_dapm_widget; 1] = [SND_SOC_DAPM_MIXER!(
    c_str!("Output Mixer"),
    SND_SOC_NOPM,
    0,
    0,
    ssm260x_output_mixer_controls,
    ARRAY_SIZE!(ssm260x_output_mixer_controls) - 1
) /* Last element is the mic */];

static ssm260x_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: c_str!("DAC"), control: NULL, source: c_str!("Digital Core Power") },
    snd_soc_dapm_route { sink: c_str!("ADC"), control: NULL, source: c_str!("Digital Core Power") },
    snd_soc_dapm_route { sink: c_str!("Output Mixer"), control: c_str!("Line Bypass Switch"), source: c_str!("Line Input") },
    snd_soc_dapm_route { sink: c_str!("Output Mixer"), control: c_str!("HiFi Playback Switch"), source: c_str!("DAC") },
    snd_soc_dapm_route { sink: c_str!("ROUT"), control: NULL, source: c_str!("Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("LOUT"), control: NULL, source: c_str!("Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("Line Input"), control: NULL, source: c_str!("LLINEIN") },
    snd_soc_dapm_route { sink: c_str!("Line Input"), control: NULL, source: c_str!("RLINEIN") },
];

static ssm2602_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: c_str!("Output Mixer"), control: c_str!("Mic Sidetone Switch"), source: c_str!("Mic Bias") },
    snd_soc_dapm_route { sink: c_str!("RHPOUT"), control: NULL, source: c_str!("Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("LHPOUT"), control: NULL, source: c_str!("Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("Input Mux"), control: c_str!("Line"), source: c_str!("Line Input") },
    snd_soc_dapm_route { sink: c_str!("Input Mux"), control: c_str!("Mic"), source: c_str!("Mic Switch") },
    snd_soc_dapm_route { sink: c_str!("ADC"), control: NULL, source: c_str!("Input Mux") },
    snd_soc_dapm_route { sink: c_str!("Mic Switch"), control: NULL, source: c_str!("Mic Bias") },
    snd_soc_dapm_route { sink: c_str!("Mic Bias"), control: NULL, source: c_str!("MICIN") },
];

static ssm2604_routes: [snd_soc_dapm_route; 1] = [
    snd_soc_dapm_route { sink: c_str!("ADC"), control: NULL, source: c_str!("Line Input") },
];

static ssm2602_rates_12288000: [c_uint; 5] = [8000, 16000, 32000, 48000, 96000];

static ssm2602_constraints_12288000: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: ssm2602_rates_12288000.as_ptr(),
    count: ARRAY_SIZE!(ssm2602_rates_12288000),
};

static ssm2602_rates_11289600: [c_uint; 5] = [8000, 11025, 22050, 44100, 88200];

static ssm2602_constraints_11289600: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: ssm2602_rates_11289600.as_ptr(),
    count: ARRAY_SIZE!(ssm2602_rates_11289600),
};

#[repr(C)]
pub struct ssm2602_coeff {
    pub mclk: u32,
    pub rate: u32,
    pub srate: u8,
}

const fn SSM2602_COEFF_SRATE(sr: u8, bosr: u8, usb: u8) -> u8 {
    (sr << 2) | (bosr << 1) | usb
}

/* codec mclk clock coefficients */
static ssm2602_coeff_table: [ssm2602_coeff; 30] = [
    /* 48k */
    ssm2602_coeff { mclk: 12288000, rate: 48000, srate: SSM2602_COEFF_SRATE(0x0, 0x0, 0x0) },
    ssm2602_coeff { mclk: 18432000, rate: 48000, srate: SSM2602_COEFF_SRATE(0x0, 0x1, 0x0) },
    ssm2602_coeff { mclk: 12000000, rate: 48000, srate: SSM2602_COEFF_SRATE(0x0, 0x0, 0x1) },
    /* 32k */
    ssm2602_coeff { mclk: 12288000, rate: 32000, srate: SSM2602_COEFF_SRATE(0x6, 0x0, 0x0) },
    ssm2602_coeff { mclk: 18432000, rate: 32000, srate: SSM2602_COEFF_SRATE(0x6, 0x1, 0x0) },
    ssm2602_coeff { mclk: 12000000, rate: 32000, srate: SSM2602_COEFF_SRATE(0x6, 0x0, 0x1) },
    /* 16k */
    ssm2602_coeff { mclk: 12288000, rate: 16000, srate: SSM2602_COEFF_SRATE(0x5, 0x0, 0x0) },
    ssm2602_coeff { mclk: 18432000, rate: 16000, srate: SSM2602_COEFF_SRATE(0x5, 0x1, 0x0) },
    ssm2602_coeff { mclk: 12000000, rate: 16000, srate: SSM2602_COEFF_SRATE(0xa, 0x0, 0x1) },
    /* 8k */
    ssm2602_coeff { mclk: 12288000, rate: 8000, srate: SSM2602_COEFF_SRATE(0x3, 0x0, 0x0) },
    ssm2602_coeff { mclk: 18432000, rate: 8000, srate: SSM2602_COEFF_SRATE(0x3, 0x1, 0x0) },
    ssm2602_coeff { mclk: 11289600, rate: 8000, srate: SSM2602_COEFF_SRATE(0xb, 0x0, 0x0) },
    ssm2602_coeff { mclk: 16934400, rate: 8000, srate: SSM2602_COEFF_SRATE(0xb, 0x1, 0x0) },
    ssm2602_coeff { mclk: 12000000, rate: 8000, srate: SSM2602_COEFF_SRATE(0x3, 0x0, 0x1) },
    /* 96k */
    ssm2602_coeff { mclk: 12288000, rate: 96000, srate: SSM2602_COEFF_SRATE(0x7, 0x0, 0x0) },
    ssm2602_coeff { mclk: 18432000, rate: 96000, srate: SSM2602_COEFF_SRATE(0x7, 0x1, 0x0) },
    ssm2602_coeff { mclk: 12000000, rate: 96000, srate: SSM2602_COEFF_SRATE(0x7, 0x0, 0x1) },
    /* 11.025k */
    ssm2602_coeff { mclk: 11289600, rate: 11025, srate: SSM2602_COEFF_SRATE(0xc, 0x0, 0x0) },
    ssm2602_coeff { mclk: 16934400, rate: 11025, srate: SSM2602_COEFF_SRATE(0xc, 0x1, 0x0) },
    ssm2602_coeff { mclk: 12000000, rate: 11025, srate: SSM2602_COEFF_SRATE(0xc, 0x1, 0x1) },
    /* 22.05k */
    ssm2602_coeff { mclk: 11289600, rate: 22050, srate: SSM2602_COEFF_SRATE(0xd, 0x0, 0x0) },
    ssm2602_coeff { mclk: 16934400, rate: 22050, srate: SSM2602_COEFF_SRATE(0xd, 0x1, 0x0) },
    ssm2602_coeff { mclk: 12000000, rate: 22050, srate: SSM2602_COEFF_SRATE(0xd, 0x1, 0x1) },
    /* 44.1k */
    ssm2602_coeff { mclk: 11289600, rate: 44100, srate: SSM2602_COEFF_SRATE(0x8, 0x0, 0x0) },
    ssm2602_coeff { mclk: 16934400, rate: 44100, srate: SSM2602_COEFF_SRATE(0x8, 0x1, 0x0) },
    ssm2602_coeff { mclk: 12000000, rate: 44100, srate: SSM2602_COEFF_SRATE(0x8, 0x1, 0x1) },
    /* 88.2k */
    ssm2602_coeff { mclk: 11289600, rate: 88200, srate: SSM2602_COEFF_SRATE(0xf, 0x0, 0x0) },
    ssm2602_coeff { mclk: 16934400, rate: 88200, srate: SSM2602_COEFF_SRATE(0xf, 0x1, 0x0) },
    ssm2602_coeff { mclk: 12000000, rate: 88200, srate: SSM2602_COEFF_SRATE(0xf, 0x1, 0x1) },
];

#[inline]
unsafe fn ssm2602_get_coeff(mclk: c_int, rate: c_int) -> c_int {
    let mut i: usize = 0;

    while i < ARRAY_SIZE!(ssm2602_coeff_table) as usize {
        if ssm2602_coeff_table[i].rate == rate as u32 {
            if ssm2602_coeff_table[i].mclk == mclk as u32 {
                return ssm2602_coeff_table[i].srate as c_int;
            }
            if ssm2602_coeff_table[i].mclk == (mclk / 2) as u32 {
                return (ssm2602_coeff_table[i].srate as c_int) | SRATE_CORECLK_DIV2;
            }
        }
        i += 1;
    }
    -EINVAL
}

unsafe extern "C" fn ssm2602_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { (*dai).component };
    let ssm2602: *mut ssm2602_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ssm2602_priv };
    let srate: c_int = unsafe { ssm2602_get_coeff((*ssm2602).sysclk as c_int, params_rate(params)) };
    let iface: c_uint;
    let _ = substream;

    if srate < 0 {
        return srate;
    }

    unsafe { regmap_write((*ssm2602).regmap, SSM2602_SRATE, srate as c_uint) };

    /* bit size */
    match unsafe { params_width(params) } {
        16 => iface = 0x0,
        20 => iface = 0x4,
        24 => iface = 0x8,
        32 => iface = 0xc,
        _ => return -EINVAL,
    }
    unsafe {
        regmap_update_bits((*ssm2602).regmap, SSM2602_IFACE, IFACE_AUDIO_DATA_LEN, iface);
    }
    0
}

unsafe extern "C" fn ssm2602_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { (*dai).component };
    let ssm2602: *mut ssm2602_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ssm2602_priv };

    if unsafe { !(*ssm2602).sysclk_constraints.is_null() } {
        unsafe {
            snd_pcm_hw_constraint_list(
                (*substream).runtime,
                0,
                SNDRV_PCM_HW_PARAM_RATE,
                (*ssm2602).sysclk_constraints,
            );
        }
    }

    0
}

unsafe extern "C" fn ssm2602_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    direction: c_int,
) -> c_int {
    let ssm2602: *mut ssm2602_priv =
        unsafe { snd_soc_component_get_drvdata((*dai).component) as *mut ssm2602_priv };
    let _ = direction;

    if mute != 0 {
        unsafe {
            regmap_update_bits(
                (*ssm2602).regmap,
                SSM2602_APDIGI,
                APDIGI_ENABLE_DAC_MUTE,
                APDIGI_ENABLE_DAC_MUTE,
            );
        }
    } else {
        unsafe {
            regmap_update_bits((*ssm2602).regmap, SSM2602_APDIGI, APDIGI_ENABLE_DAC_MUTE, 0);
        }
    }
    0
}

unsafe extern "C" fn ssm2602_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component = unsafe { (*codec_dai).component };
    let ssm2602: *mut ssm2602_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ssm2602_priv };

    if dir == SND_SOC_CLOCK_IN {
        if clk_id != SSM2602_SYSCLK {
            return -EINVAL;
        }

        match freq {
            12288000 | 18432000 | 24576000 | 36864000 => unsafe {
                (*ssm2602).sysclk_constraints = &ssm2602_constraints_12288000;
            },
            11289600 | 16934400 | 22579200 | 33868800 => unsafe {
                (*ssm2602).sysclk_constraints = &ssm2602_constraints_11289600;
            },
            12000000 | 24000000 => unsafe {
                (*ssm2602).sysclk_constraints = NULL as *const snd_pcm_hw_constraint_list;
            },
            _ => return -EINVAL,
        }

        unsafe {
            (*ssm2602).sysclk = freq;
        }
    } else {
        let mask: c_uint;

        match clk_id {
            SSM2602_CLK_CLKOUT => mask = PWR_CLK_OUT_PDN,
            SSM2602_CLK_XTO => mask = PWR_OSC_PDN,
            _ => return -EINVAL,
        }

        if freq == 0 {
            unsafe {
                (*ssm2602).clk_out_pwr |= mask;
            }
        } else {
            unsafe {
                (*ssm2602).clk_out_pwr &= !mask;
            }
        }

        unsafe {
            regmap_update_bits(
                (*ssm2602).regmap,
                SSM2602_PWR,
                PWR_CLK_OUT_PDN | PWR_OSC_PDN,
                (*ssm2602).clk_out_pwr,
            );
        }
    }

    0
}

unsafe extern "C" fn ssm2602_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let ssm2602: *mut ssm2602_priv =
        unsafe { snd_soc_component_get_drvdata((*codec_dai).component) as *mut ssm2602_priv };
    let mut iface: c_uint = 0;

    /* set master/slave audio interface */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => iface |= 0x0040,
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => iface |= 0x0002,
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => iface |= 0x0001,
        SND_SOC_DAIFMT_DSP_A => iface |= 0x0013,
        SND_SOC_DAIFMT_DSP_B => iface |= 0x0003,
        _ => return -EINVAL,
    }

    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => iface |= 0x0090,
        SND_SOC_DAIFMT_IB_NF => iface |= 0x0080,
        SND_SOC_DAIFMT_NB_IF => iface |= 0x0010,
        _ => return -EINVAL,
    }

    /* set iface */
    unsafe { regmap_write((*ssm2602).regmap, SSM2602_IFACE, iface) };
    0
}

unsafe extern "C" fn ssm2602_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let ssm2602: *mut ssm2602_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ssm2602_priv };

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {
            /* vref/mid on, osc and clkout on if enabled */
            unsafe {
                regmap_update_bits(
                    (*ssm2602).regmap,
                    SSM2602_PWR,
                    PWR_POWER_OFF | PWR_CLK_OUT_PDN | PWR_OSC_PDN,
                    (*ssm2602).clk_out_pwr,
                );
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            /* everything off except vref/vmid, */
            unsafe {
                regmap_update_bits(
                    (*ssm2602).regmap,
                    SSM2602_PWR,
                    PWR_POWER_OFF | PWR_CLK_OUT_PDN | PWR_OSC_PDN,
                    PWR_CLK_OUT_PDN | PWR_OSC_PDN,
                );
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            /* everything off */
            unsafe {
                regmap_update_bits((*ssm2602).regmap, SSM2602_PWR, PWR_POWER_OFF, PWR_POWER_OFF);
            }
        }
    }
    0
}

const SSM2602_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_11025
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000;

const SSM2602_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

static ssm2602_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(ssm2602_startup),
    hw_params: Some(ssm2602_hw_params),
    mute_stream: Some(ssm2602_mute),
    set_sysclk: Some(ssm2602_set_dai_sysclk),
    set_fmt: Some(ssm2602_set_dai_fmt),
    no_capture_mute: 1,
};

static mut ssm2602_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c_str!("ssm2602-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("Playback"),
        channels_min: 2,
        channels_max: 2,
        rates: SSM2602_RATES,
        formats: SSM2602_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c_str!("Capture"),
        channels_min: 2,
        channels_max: 2,
        rates: SSM2602_RATES,
        formats: SSM2602_FORMATS,
    },
    ops: &ssm2602_dai_ops,
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
};

unsafe extern "C" fn ssm2602_resume(component: *mut snd_soc_component) -> c_int {
    let ssm2602: *mut ssm2602_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ssm2602_priv };

    unsafe { regcache_sync((*ssm2602).regmap) };

    0
}

unsafe extern "C" fn ssm2602_component_probe(component: *mut snd_soc_component) -> c_int {
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_component_to_dapm(component) };
    let ssm2602: *mut ssm2602_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ssm2602_priv };
    let mut ret: c_int;

    unsafe {
        regmap_update_bits((*ssm2602).regmap, SSM2602_LOUT1V, LOUT1V_LRHP_BOTH, LOUT1V_LRHP_BOTH);
        regmap_update_bits((*ssm2602).regmap, SSM2602_ROUT1V, ROUT1V_RLHP_BOTH, ROUT1V_RLHP_BOTH);
    }

    ret = unsafe {
        snd_soc_add_component_controls(
            component,
            ssm2602_snd_controls.as_ptr(),
            ARRAY_SIZE!(ssm2602_snd_controls),
        )
    };
    if ret != 0 {
        return ret;
    }

    ret = unsafe {
        snd_soc_dapm_new_controls(
            dapm,
            ssm2602_dapm_widgets.as_ptr(),
            ARRAY_SIZE!(ssm2602_dapm_widgets),
        )
    };
    if ret != 0 {
        return ret;
    }

    unsafe { snd_soc_dapm_add_routes(dapm, ssm2602_routes.as_ptr(), ARRAY_SIZE!(ssm2602_routes)) }
}

unsafe extern "C" fn ssm2604_component_probe(component: *mut snd_soc_component) -> c_int {
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_component_to_dapm(component) };
    let mut ret: c_int;

    ret = unsafe {
        snd_soc_dapm_new_controls(
            dapm,
            ssm2604_dapm_widgets.as_ptr(),
            ARRAY_SIZE!(ssm2604_dapm_widgets),
        )
    };
    if ret != 0 {
        return ret;
    }

    unsafe { snd_soc_dapm_add_routes(dapm, ssm2604_routes.as_ptr(), ARRAY_SIZE!(ssm2604_routes)) }
}

unsafe extern "C" fn ssm260x_component_probe(component: *mut snd_soc_component) -> c_int {
    let ssm2602: *mut ssm2602_priv =
        unsafe { snd_soc_component_get_drvdata(component) as *mut ssm2602_priv };
    let mut ret: c_int;

    ret = unsafe { regmap_write((*ssm2602).regmap, SSM2602_RESET, 0) };
    if ret < 0 {
        unsafe { dev_err((*component).dev, c_str!("Failed to issue reset: %d\n"), ret) };
        return ret;
    }

    unsafe {
        regmap_register_patch(
            (*ssm2602).regmap,
            ssm2602_patch.as_ptr(),
            ARRAY_SIZE!(ssm2602_patch),
        );
    }

    /* set the update bits */
    unsafe {
        regmap_update_bits((*ssm2602).regmap, SSM2602_LINVOL, LINVOL_LRIN_BOTH, LINVOL_LRIN_BOTH);
        regmap_update_bits((*ssm2602).regmap, SSM2602_RINVOL, RINVOL_RLIN_BOTH, RINVOL_RLIN_BOTH);
        /*select Line in as default input*/
        regmap_write(
            (*ssm2602).regmap,
            SSM2602_APANA,
            APANA_SELECT_DAC | APANA_ENABLE_MIC_BOOST,
        );
    }

    match unsafe { (*ssm2602).type_ } {
        ssm2602_type::SSM2602 => ret = unsafe { ssm2602_component_probe(component) },
        ssm2602_type::SSM2604 => ret = unsafe { ssm2604_component_probe(component) },
    }

    ret
}

static soc_component_dev_ssm2602: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ssm260x_component_probe),
    resume: Some(ssm2602_resume),
    set_bias_level: Some(ssm2602_set_bias_level),
    controls: ssm260x_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(ssm260x_snd_controls),
    dapm_widgets: ssm260x_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(ssm260x_dapm_widgets),
    dapm_routes: ssm260x_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(ssm260x_routes),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn ssm2602_register_volatile(dev: *mut device, reg: c_uint) -> bool {
    let _ = dev;
    reg == SSM2602_RESET
}

pub static ssm2602_regmap_config: regmap_config = regmap_config {
    val_bits: 9,
    reg_bits: 7,
    max_register: SSM2602_RESET,
    volatile_reg: Some(ssm2602_register_volatile),
    cache_type: REGCACHE_RBTREE,
    reg_defaults: ssm2602_reg.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(ssm2602_reg),
};
EXPORT_SYMBOL_GPL!(ssm2602_regmap_config);

pub unsafe extern "C" fn ssm2602_probe(
    dev: *mut device,
    type_: ssm2602_type,
    regmap: *mut regmap,
) -> c_int {
    let ssm2602: *mut ssm2602_priv;

    if unsafe { IS_ERR(regmap as *const c_void) } {
        return unsafe { PTR_ERR(regmap as *const c_void) as c_int };
    }

    ssm2602 = unsafe {
        devm_kzalloc(
            dev,
            core::mem::size_of::<ssm2602_priv>(),
            GFP_KERNEL,
        ) as *mut ssm2602_priv
    };
    if ssm2602.is_null() {
        return -ENOMEM;
    }

    unsafe {
        dev_set_drvdata(dev, ssm2602 as *mut c_void);
        (*ssm2602).type_ = type_;
        (*ssm2602).regmap = regmap;
    }

    unsafe {
        devm_snd_soc_register_component(
            dev,
            &soc_component_dev_ssm2602,
            &mut ssm2602_dai,
            1,
        )
    }
}
EXPORT_SYMBOL_GPL!(ssm2602_probe);

MODULE_DESCRIPTION!(c_str!("ASoC SSM2602/SSM2603/SSM2604 driver"));
MODULE_AUTHOR!(c_str!("Cliff Cai"));
MODULE_LICENSE!(c_str!("GPL"));

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
