// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8978.c  --  WM8978 ALSA SoC Audio Codec driver
 *
 * Copyright (C) 2009-2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 * Copyright (C) 2007 Carlos Munoz <carlos@kenati.com>
 * Copyright 2006-2009 Wolfson Microelectronics PLC.
 * Based on wm8974 and wm8990 by Liam Girdwood <lrg@slimlogic.co.uk>
 */

/* Translated from the C implementation source. Kernel, ASoC, regmap, I2C,
 * and WM8978 header symbols are external dependencies supplied elsewhere. */

const FIXED_PLL_SIZE: u64 = 1 << 24;

const WM8978_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

static wm8978_reg_defaults: &[reg_default] = &[
    reg_default { reg: 1, def: 0x0000 },
    reg_default { reg: 2, def: 0x0000 },
    reg_default { reg: 3, def: 0x0000 },
    reg_default { reg: 4, def: 0x0050 },
    reg_default { reg: 5, def: 0x0000 },
    reg_default { reg: 6, def: 0x0140 },
    reg_default { reg: 7, def: 0x0000 },
    reg_default { reg: 8, def: 0x0000 },
    reg_default { reg: 9, def: 0x0000 },
    reg_default { reg: 10, def: 0x0000 },
    reg_default { reg: 11, def: 0x00ff },
    reg_default { reg: 12, def: 0x00ff },
    reg_default { reg: 13, def: 0x0000 },
    reg_default { reg: 14, def: 0x0100 },
    reg_default { reg: 15, def: 0x00ff },
    reg_default { reg: 16, def: 0x00ff },
    reg_default { reg: 17, def: 0x0000 },
    reg_default { reg: 18, def: 0x012c },
    reg_default { reg: 19, def: 0x002c },
    reg_default { reg: 20, def: 0x002c },
    reg_default { reg: 21, def: 0x002c },
    reg_default { reg: 22, def: 0x002c },
    reg_default { reg: 23, def: 0x0000 },
    reg_default { reg: 24, def: 0x0032 },
    reg_default { reg: 25, def: 0x0000 },
    reg_default { reg: 26, def: 0x0000 },
    reg_default { reg: 27, def: 0x0000 },
    reg_default { reg: 28, def: 0x0000 },
    reg_default { reg: 29, def: 0x0000 },
    reg_default { reg: 30, def: 0x0000 },
    reg_default { reg: 31, def: 0x0000 },
    reg_default { reg: 32, def: 0x0038 },
    reg_default { reg: 33, def: 0x000b },
    reg_default { reg: 34, def: 0x0032 },
    reg_default { reg: 35, def: 0x0000 },
    reg_default { reg: 36, def: 0x0008 },
    reg_default { reg: 37, def: 0x000c },
    reg_default { reg: 38, def: 0x0093 },
    reg_default { reg: 39, def: 0x00e9 },
    reg_default { reg: 40, def: 0x0000 },
    reg_default { reg: 41, def: 0x0000 },
    reg_default { reg: 42, def: 0x0000 },
    reg_default { reg: 43, def: 0x0000 },
    reg_default { reg: 44, def: 0x0033 },
    reg_default { reg: 45, def: 0x0010 },
    reg_default { reg: 46, def: 0x0010 },
    reg_default { reg: 47, def: 0x0100 },
    reg_default { reg: 48, def: 0x0100 },
    reg_default { reg: 49, def: 0x0002 },
    reg_default { reg: 50, def: 0x0001 },
    reg_default { reg: 51, def: 0x0001 },
    reg_default { reg: 52, def: 0x0039 },
    reg_default { reg: 53, def: 0x0039 },
    reg_default { reg: 54, def: 0x0039 },
    reg_default { reg: 55, def: 0x0039 },
    reg_default { reg: 56, def: 0x0001 },
    reg_default { reg: 57, def: 0x0001 },
];

unsafe extern "C" fn wm8978_volatile(_dev: *mut device, reg: c_uint) -> bool {
    reg == WM8978_RESET
}

/* codec private data */
#[repr(C)]
struct wm8978_priv {
    regmap: *mut regmap,
    f_pllout: c_uint,
    f_mclk: c_uint,
    f_256fs: c_uint,
    f_opclk: c_uint,
    mclk_idx: c_int,
    sysclk: wm8978_sysclk_src,
}

static wm8978_companding: &[&str] = &["Off", "NC", "u-law", "A-law"];
static wm8978_eqmode: &[&str] = &["Capture", "Playback"];
static wm8978_bw: &[&str] = &["Narrow", "Wide"];
static wm8978_eq1: &[&str] = &["80Hz", "105Hz", "135Hz", "175Hz"];
static wm8978_eq2: &[&str] = &["230Hz", "300Hz", "385Hz", "500Hz"];
static wm8978_eq3: &[&str] = &["650Hz", "850Hz", "1.1kHz", "1.4kHz"];
static wm8978_eq4: &[&str] = &["1.8kHz", "2.4kHz", "3.2kHz", "4.1kHz"];
static wm8978_eq5: &[&str] = &["5.3kHz", "6.9kHz", "9kHz", "11.7kHz"];
static wm8978_alc3: &[&str] = &["ALC", "Limiter"];
static wm8978_alc1: &[&str] = &["Off", "Right", "Left", "Both"];

SOC_ENUM_SINGLE_DECL!(adc_compand, WM8978_COMPANDING_CONTROL, 1, wm8978_companding);
SOC_ENUM_SINGLE_DECL!(dac_compand, WM8978_COMPANDING_CONTROL, 3, wm8978_companding);
SOC_ENUM_SINGLE_DECL!(eqmode, WM8978_EQ1, 8, wm8978_eqmode);
SOC_ENUM_SINGLE_DECL!(eq1, WM8978_EQ1, 5, wm8978_eq1);
SOC_ENUM_SINGLE_DECL!(eq2bw, WM8978_EQ2, 8, wm8978_bw);
SOC_ENUM_SINGLE_DECL!(eq2, WM8978_EQ2, 5, wm8978_eq2);
SOC_ENUM_SINGLE_DECL!(eq3bw, WM8978_EQ3, 8, wm8978_bw);
SOC_ENUM_SINGLE_DECL!(eq3, WM8978_EQ3, 5, wm8978_eq3);
SOC_ENUM_SINGLE_DECL!(eq4bw, WM8978_EQ4, 8, wm8978_bw);
SOC_ENUM_SINGLE_DECL!(eq4, WM8978_EQ4, 5, wm8978_eq4);
SOC_ENUM_SINGLE_DECL!(eq5, WM8978_EQ5, 5, wm8978_eq5);
SOC_ENUM_SINGLE_DECL!(alc3, WM8978_ALC_CONTROL_3, 8, wm8978_alc3);
SOC_ENUM_SINGLE_DECL!(alc1, WM8978_ALC_CONTROL_1, 7, wm8978_alc1);

DECLARE_TLV_DB_SCALE!(digital_tlv, -12750, 50, 1);
DECLARE_TLV_DB_SCALE!(eq_tlv, -1200, 100, 0);
DECLARE_TLV_DB_SCALE!(inpga_tlv, -1200, 75, 0);
DECLARE_TLV_DB_SCALE!(spk_tlv, -5700, 100, 0);
DECLARE_TLV_DB_SCALE!(boost_tlv, -1500, 300, 1);
DECLARE_TLV_DB_SCALE!(limiter_tlv, 0, 100, 0);

static wm8978_snd_controls: &[snd_kcontrol_new] = &[
    SOC_SINGLE!("Digital Loopback Switch", WM8978_COMPANDING_CONTROL, 0, 1, 0),
    SOC_ENUM!("ADC Companding", adc_compand),
    SOC_ENUM!("DAC Companding", dac_compand),
    SOC_DOUBLE!("DAC Inversion Switch", WM8978_DAC_CONTROL, 0, 1, 1, 0),
    SOC_DOUBLE_R_TLV!("PCM Volume", WM8978_LEFT_DAC_DIGITAL_VOLUME, WM8978_RIGHT_DAC_DIGITAL_VOLUME, 0, 255, 0, digital_tlv),
    SOC_SINGLE!("High Pass Filter Switch", WM8978_ADC_CONTROL, 8, 1, 0),
    SOC_SINGLE!("High Pass Cut Off", WM8978_ADC_CONTROL, 4, 7, 0),
    SOC_DOUBLE!("ADC Inversion Switch", WM8978_ADC_CONTROL, 0, 1, 1, 0),
    SOC_DOUBLE_R_TLV!("ADC Volume", WM8978_LEFT_ADC_DIGITAL_VOLUME, WM8978_RIGHT_ADC_DIGITAL_VOLUME, 0, 255, 0, digital_tlv),
    SOC_ENUM!("Equaliser Function", eqmode),
    SOC_ENUM!("EQ1 Cut Off", eq1),
    SOC_SINGLE_TLV!("EQ1 Volume", WM8978_EQ1, 0, 24, 1, eq_tlv),
    SOC_ENUM!("Equaliser EQ2 Bandwidth", eq2bw),
    SOC_ENUM!("EQ2 Cut Off", eq2),
    SOC_SINGLE_TLV!("EQ2 Volume", WM8978_EQ2, 0, 24, 1, eq_tlv),
    SOC_ENUM!("Equaliser EQ3 Bandwidth", eq3bw),
    SOC_ENUM!("EQ3 Cut Off", eq3),
    SOC_SINGLE_TLV!("EQ3 Volume", WM8978_EQ3, 0, 24, 1, eq_tlv),
    SOC_ENUM!("Equaliser EQ4 Bandwidth", eq4bw),
    SOC_ENUM!("EQ4 Cut Off", eq4),
    SOC_SINGLE_TLV!("EQ4 Volume", WM8978_EQ4, 0, 24, 1, eq_tlv),
    SOC_ENUM!("EQ5 Cut Off", eq5),
    SOC_SINGLE_TLV!("EQ5 Volume", WM8978_EQ5, 0, 24, 1, eq_tlv),
    SOC_SINGLE!("DAC Playback Limiter Switch", WM8978_DAC_LIMITER_1, 8, 1, 0),
    SOC_SINGLE!("DAC Playback Limiter Decay", WM8978_DAC_LIMITER_1, 4, 15, 0),
    SOC_SINGLE!("DAC Playback Limiter Attack", WM8978_DAC_LIMITER_1, 0, 15, 0),
    SOC_SINGLE!("DAC Playback Limiter Threshold", WM8978_DAC_LIMITER_2, 4, 7, 0),
    SOC_SINGLE_TLV!("DAC Playback Limiter Volume", WM8978_DAC_LIMITER_2, 0, 12, 0, limiter_tlv),
    SOC_ENUM!("ALC Enable Switch", alc1),
    SOC_SINGLE!("ALC Capture Min Gain", WM8978_ALC_CONTROL_1, 0, 7, 0),
    SOC_SINGLE!("ALC Capture Max Gain", WM8978_ALC_CONTROL_1, 3, 7, 0),
    SOC_SINGLE!("ALC Capture Hold", WM8978_ALC_CONTROL_2, 4, 10, 0),
    SOC_SINGLE!("ALC Capture Target", WM8978_ALC_CONTROL_2, 0, 15, 0),
    SOC_ENUM!("ALC Capture Mode", alc3),
    SOC_SINGLE!("ALC Capture Decay", WM8978_ALC_CONTROL_3, 4, 10, 0),
    SOC_SINGLE!("ALC Capture Attack", WM8978_ALC_CONTROL_3, 0, 10, 0),
    SOC_SINGLE!("ALC Capture Noise Gate Switch", WM8978_NOISE_GATE, 3, 1, 0),
    SOC_SINGLE!("ALC Capture Noise Gate Threshold", WM8978_NOISE_GATE, 0, 7, 0),
    SOC_DOUBLE_R!("Capture PGA ZC Switch", WM8978_LEFT_INP_PGA_CONTROL, WM8978_RIGHT_INP_PGA_CONTROL, 7, 1, 0),
    /* OUT1 - Headphones */
    SOC_DOUBLE_R!("Headphone Playback ZC Switch", WM8978_LOUT1_HP_CONTROL, WM8978_ROUT1_HP_CONTROL, 7, 1, 0),
    SOC_DOUBLE_R_TLV!("Headphone Playback Volume", WM8978_LOUT1_HP_CONTROL, WM8978_ROUT1_HP_CONTROL, 0, 63, 0, spk_tlv),
    /* OUT2 - Speakers */
    SOC_DOUBLE_R!("Speaker Playback ZC Switch", WM8978_LOUT2_SPK_CONTROL, WM8978_ROUT2_SPK_CONTROL, 7, 1, 0),
    SOC_DOUBLE_R_TLV!("Speaker Playback Volume", WM8978_LOUT2_SPK_CONTROL, WM8978_ROUT2_SPK_CONTROL, 0, 63, 0, spk_tlv),
    /* OUT3/4 - Line Output */
    SOC_DOUBLE_R!("Line Playback Switch", WM8978_OUT3_MIXER_CONTROL, WM8978_OUT4_MIXER_CONTROL, 6, 1, 1),
    /* Mixer #3: Boost (Input) mixer */
    SOC_DOUBLE_R!("PGA Boost (+20dB)", WM8978_LEFT_ADC_BOOST_CONTROL, WM8978_RIGHT_ADC_BOOST_CONTROL, 8, 1, 0),
    SOC_DOUBLE_R_TLV!("L2/R2 Boost Volume", WM8978_LEFT_ADC_BOOST_CONTROL, WM8978_RIGHT_ADC_BOOST_CONTROL, 4, 7, 0, boost_tlv),
    SOC_DOUBLE_R_TLV!("Aux Boost Volume", WM8978_LEFT_ADC_BOOST_CONTROL, WM8978_RIGHT_ADC_BOOST_CONTROL, 0, 7, 0, boost_tlv),
    /* Input PGA volume */
    SOC_DOUBLE_R_TLV!("Input PGA Volume", WM8978_LEFT_INP_PGA_CONTROL, WM8978_RIGHT_INP_PGA_CONTROL, 0, 63, 0, inpga_tlv),
    /* Headphone */
    SOC_DOUBLE_R!("Headphone Switch", WM8978_LOUT1_HP_CONTROL, WM8978_ROUT1_HP_CONTROL, 6, 1, 1),
    /* Speaker */
    SOC_DOUBLE_R!("Speaker Switch", WM8978_LOUT2_SPK_CONTROL, WM8978_ROUT2_SPK_CONTROL, 6, 1, 1),
    /* DAC / ADC oversampling */
    SOC_SINGLE!("DAC 128x Oversampling Switch", WM8978_DAC_CONTROL, 5, 1, 0),
    SOC_SINGLE!("ADC 128x Oversampling Switch", WM8978_ADC_CONTROL, 5, 1, 0),
];

/* Mixer #1: Output (OUT1, OUT2) Mixer: mix AUX, Input mixer output and DAC */
static wm8978_left_out_mixer: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Line Bypass Switch", WM8978_LEFT_MIXER_CONTROL, 1, 1, 0),
    SOC_DAPM_SINGLE!("Aux Playback Switch", WM8978_LEFT_MIXER_CONTROL, 5, 1, 0),
    SOC_DAPM_SINGLE!("PCM Playback Switch", WM8978_LEFT_MIXER_CONTROL, 0, 1, 0),
];

static wm8978_right_out_mixer: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Line Bypass Switch", WM8978_RIGHT_MIXER_CONTROL, 1, 1, 0),
    SOC_DAPM_SINGLE!("Aux Playback Switch", WM8978_RIGHT_MIXER_CONTROL, 5, 1, 0),
    SOC_DAPM_SINGLE!("PCM Playback Switch", WM8978_RIGHT_MIXER_CONTROL, 0, 1, 0),
];

/* OUT3/OUT4 Mixer not implemented */

/* Mixer #2: Input PGA Mute */
static wm8978_left_input_mixer: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("L2 Switch", WM8978_INPUT_CONTROL, 2, 1, 0),
    SOC_DAPM_SINGLE!("MicN Switch", WM8978_INPUT_CONTROL, 1, 1, 0),
    SOC_DAPM_SINGLE!("MicP Switch", WM8978_INPUT_CONTROL, 0, 1, 0),
];

static wm8978_right_input_mixer: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("R2 Switch", WM8978_INPUT_CONTROL, 6, 1, 0),
    SOC_DAPM_SINGLE!("MicN Switch", WM8978_INPUT_CONTROL, 5, 1, 0),
    SOC_DAPM_SINGLE!("MicP Switch", WM8978_INPUT_CONTROL, 4, 1, 0),
];

static wm8978_dapm_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_DAC!("Left DAC", "Left HiFi Playback", WM8978_POWER_MANAGEMENT_3, 0, 0),
    SND_SOC_DAPM_DAC!("Right DAC", "Right HiFi Playback", WM8978_POWER_MANAGEMENT_3, 1, 0),
    SND_SOC_DAPM_ADC!("Left ADC", "Left HiFi Capture", WM8978_POWER_MANAGEMENT_2, 0, 0),
    SND_SOC_DAPM_ADC!("Right ADC", "Right HiFi Capture", WM8978_POWER_MANAGEMENT_2, 1, 0),
    /* Mixer #1: OUT1,2 */
    SOC_MIXER_ARRAY!("Left Output Mixer", WM8978_POWER_MANAGEMENT_3, 2, 0, wm8978_left_out_mixer),
    SOC_MIXER_ARRAY!("Right Output Mixer", WM8978_POWER_MANAGEMENT_3, 3, 0, wm8978_right_out_mixer),
    SOC_MIXER_ARRAY!("Left Input Mixer", WM8978_POWER_MANAGEMENT_2, 2, 0, wm8978_left_input_mixer),
    SOC_MIXER_ARRAY!("Right Input Mixer", WM8978_POWER_MANAGEMENT_2, 3, 0, wm8978_right_input_mixer),
    SND_SOC_DAPM_PGA!("Left Boost Mixer", WM8978_POWER_MANAGEMENT_2, 4, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Right Boost Mixer", WM8978_POWER_MANAGEMENT_2, 5, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Left Capture PGA", WM8978_LEFT_INP_PGA_CONTROL, 6, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Right Capture PGA", WM8978_RIGHT_INP_PGA_CONTROL, 6, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Left Headphone Out", WM8978_POWER_MANAGEMENT_2, 7, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Right Headphone Out", WM8978_POWER_MANAGEMENT_2, 8, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Left Speaker Out", WM8978_POWER_MANAGEMENT_3, 6, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Right Speaker Out", WM8978_POWER_MANAGEMENT_3, 5, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("OUT4 VMID", WM8978_POWER_MANAGEMENT_3, 8, 0, NULL, 0),
    SND_SOC_DAPM_MICBIAS!("Mic Bias", WM8978_POWER_MANAGEMENT_1, 4, 0),
    SND_SOC_DAPM_INPUT!("LMICN"),
    SND_SOC_DAPM_INPUT!("LMICP"),
    SND_SOC_DAPM_INPUT!("RMICN"),
    SND_SOC_DAPM_INPUT!("RMICP"),
    SND_SOC_DAPM_INPUT!("LAUX"),
    SND_SOC_DAPM_INPUT!("RAUX"),
    SND_SOC_DAPM_INPUT!("L2"),
    SND_SOC_DAPM_INPUT!("R2"),
    SND_SOC_DAPM_OUTPUT!("LHP"),
    SND_SOC_DAPM_OUTPUT!("RHP"),
    SND_SOC_DAPM_OUTPUT!("LSPK"),
    SND_SOC_DAPM_OUTPUT!("RSPK"),
];

static wm8978_dapm_routes: &[snd_soc_dapm_route] = &[
    /* Output mixer */
    snd_soc_dapm_route { sink: "Right Output Mixer", control: "PCM Playback Switch", source: "Right DAC" },
    snd_soc_dapm_route { sink: "Right Output Mixer", control: "Aux Playback Switch", source: "RAUX" },
    snd_soc_dapm_route { sink: "Right Output Mixer", control: "Line Bypass Switch", source: "Right Boost Mixer" },
    snd_soc_dapm_route { sink: "Left Output Mixer", control: "PCM Playback Switch", source: "Left DAC" },
    snd_soc_dapm_route { sink: "Left Output Mixer", control: "Aux Playback Switch", source: "LAUX" },
    snd_soc_dapm_route { sink: "Left Output Mixer", control: "Line Bypass Switch", source: "Left Boost Mixer" },
    /* Outputs */
    snd_soc_dapm_route { sink: "Right Headphone Out", control: NULL, source: "Right Output Mixer" },
    snd_soc_dapm_route { sink: "RHP", control: NULL, source: "Right Headphone Out" },
    snd_soc_dapm_route { sink: "Left Headphone Out", control: NULL, source: "Left Output Mixer" },
    snd_soc_dapm_route { sink: "LHP", control: NULL, source: "Left Headphone Out" },
    snd_soc_dapm_route { sink: "Right Speaker Out", control: NULL, source: "Right Output Mixer" },
    snd_soc_dapm_route { sink: "RSPK", control: NULL, source: "Right Speaker Out" },
    snd_soc_dapm_route { sink: "Left Speaker Out", control: NULL, source: "Left Output Mixer" },
    snd_soc_dapm_route { sink: "LSPK", control: NULL, source: "Left Speaker Out" },
    /* Boost Mixer */
    snd_soc_dapm_route { sink: "Right ADC", control: NULL, source: "Right Boost Mixer" },
    snd_soc_dapm_route { sink: "Right Boost Mixer", control: NULL, source: "RAUX" },
    snd_soc_dapm_route { sink: "Right Boost Mixer", control: NULL, source: "Right Capture PGA" },
    snd_soc_dapm_route { sink: "Right Boost Mixer", control: NULL, source: "R2" },
    snd_soc_dapm_route { sink: "Left ADC", control: NULL, source: "Left Boost Mixer" },
    snd_soc_dapm_route { sink: "Left Boost Mixer", control: NULL, source: "LAUX" },
    snd_soc_dapm_route { sink: "Left Boost Mixer", control: NULL, source: "Left Capture PGA" },
    snd_soc_dapm_route { sink: "Left Boost Mixer", control: NULL, source: "L2" },
    /* Input PGA */
    snd_soc_dapm_route { sink: "Right Capture PGA", control: NULL, source: "Right Input Mixer" },
    snd_soc_dapm_route { sink: "Left Capture PGA", control: NULL, source: "Left Input Mixer" },
    snd_soc_dapm_route { sink: "Right Input Mixer", control: "R2 Switch", source: "R2" },
    snd_soc_dapm_route { sink: "Right Input Mixer", control: "MicN Switch", source: "RMICN" },
    snd_soc_dapm_route { sink: "Right Input Mixer", control: "MicP Switch", source: "RMICP" },
    snd_soc_dapm_route { sink: "Left Input Mixer", control: "L2 Switch", source: "L2" },
    snd_soc_dapm_route { sink: "Left Input Mixer", control: "MicN Switch", source: "LMICN" },
    snd_soc_dapm_route { sink: "Left Input Mixer", control: "MicP Switch", source: "LMICP" },
];

/* PLL divisors */
#[repr(C)]
struct wm8978_pll_div {
    k: u32,
    n: u8,
    div2: u8,
}

unsafe fn pll_factors(
    component: *mut snd_soc_component,
    pll_div: *mut wm8978_pll_div,
    target: c_uint,
    mut source: c_uint,
) {
    let mut k_part: u64;
    let k: c_uint;
    let mut n_div: c_uint;
    let n_mod: c_uint;

    n_div = target / source;
    if n_div < 6 {
        source >>= 1;
        (*pll_div).div2 = 1;
        n_div = target / source;
    } else {
        (*pll_div).div2 = 0;
    }

    if n_div < 6 || n_div > 12 {
        dev_warn!(
            (*component).dev,
            "WM8978 N value exceeds recommended range! N = %u\n",
            n_div
        );
    }

    (*pll_div).n = n_div as u8;
    n_mod = target - source * n_div;
    k_part = FIXED_PLL_SIZE * n_mod as u64 + (source / 2) as u64;

    do_div(&mut k_part, source);

    k = (k_part & 0xFFFFFFFF) as c_uint;

    (*pll_div).k = k;
}

/* MCLK dividers */
static mclk_numerator: [c_int; 8] = [1, 3, 2, 3, 4, 6, 8, 12];
static mclk_denominator: [c_int; 8] = [1, 2, 1, 1, 1, 1, 1, 1];

/*
 * find index >= idx, such that, for a given f_out,
 * 3 * f_mclk / 4 <= f_PLLOUT < 13 * f_mclk / 4
 * f_out can be f_256fs or f_opclk, currently only used for f_256fs. Can be
 * generalised for f_opclk with suitable coefficient arrays, but currently
 * the OPCLK divisor is calculated directly, not iteratively.
 */
unsafe fn wm8978_enum_mclk(f_out: c_uint, f_mclk: c_uint, f_pllout: *mut c_uint) -> c_int {
    let mut i: usize = 0;

    while i < mclk_numerator.len() {
        let f_pllout_x4: c_uint =
            4 * f_out * mclk_numerator[i] as c_uint / mclk_denominator[i] as c_uint;
        if 3 * f_mclk <= f_pllout_x4 && f_pllout_x4 < 13 * f_mclk {
            *f_pllout = f_pllout_x4 / 4;
            return i as c_int;
        }
        i += 1;
    }

    -EINVAL
}

/*
 * Calculate internal frequencies and dividers, according to Figure 40
 * "PLL and Clock Select Circuit" in WM8978 datasheet Rev. 2.6
 */
unsafe fn wm8978_configure_pll(component: *mut snd_soc_component) -> c_int {
    let wm8978 = snd_soc_component_get_drvdata(component) as *mut wm8978_priv;
    let mut pll_div = wm8978_pll_div { k: 0, n: 0, div2: 0 };
    let f_opclk = (*wm8978).f_opclk;
    let f_mclk = (*wm8978).f_mclk;
    let f_256fs = (*wm8978).f_256fs;
    let f2: c_uint;

    if f_mclk == 0 {
        return -EINVAL;
    }

    if f_opclk != 0 {
        let opclk_div: c_uint;
        /* Cannot set up MCLK divider now, do later */
        (*wm8978).mclk_idx = -1;

        /*
         * The user needs OPCLK. Choose OPCLKDIV to put
         * 6 <= R = f2 / f1 < 13, 1 <= OPCLKDIV <= 4.
         * f_opclk = f_mclk * prescale * R / 4 / OPCLKDIV, where
         * prescale = 1, or prescale = 2. Prescale is calculated inside
         * pll_factors(). We have to select f_PLLOUT, such that
         * f_mclk * 3 / 4 <= f_PLLOUT < f_mclk * 13 / 4. Must be
         * f_mclk * 3 / 16 <= f_opclk < f_mclk * 13 / 4.
         */
        if 16 * f_opclk < 3 * f_mclk || 4 * f_opclk >= 13 * f_mclk {
            return -EINVAL;
        }

        if 4 * f_opclk < 3 * f_mclk {
            /* Have to use OPCLKDIV */
            opclk_div = DIV_ROUND_UP(3 * f_mclk / 4, f_opclk);
        } else {
            opclk_div = 1;
        }

        dev_dbg!((*component).dev, "%s: OPCLKDIV=%d\n", __func__, opclk_div);

        snd_soc_component_update_bits(
            component,
            WM8978_GPIO_CONTROL,
            0x30,
            (opclk_div - 1) << 4,
        );

        (*wm8978).f_pllout = f_opclk * opclk_div;
    } else if f_256fs != 0 {
        /*
         * Not using OPCLK, but PLL is used for the codec, choose R:
         * 6 <= R = f2 / f1 < 13, to put 1 <= MCLKDIV <= 12.
         * f_256fs = f_mclk * prescale * R / 4 / MCLKDIV, where
         * prescale = 1, or prescale = 2. Prescale is calculated inside
         * pll_factors(). We have to select f_PLLOUT, such that
         * f_mclk * 3 / 4 <= f_PLLOUT < f_mclk * 13 / 4. Must be
         * f_mclk * 3 / 48 <= f_256fs < f_mclk * 13 / 4. This means MCLK
         * must be 3.781MHz <= f_MCLK <= 32.768MHz
         */
        let idx = wm8978_enum_mclk(f_256fs, f_mclk, &mut (*wm8978).f_pllout);
        if idx < 0 {
            return idx;
        }

        (*wm8978).mclk_idx = idx;
    } else {
        return -EINVAL;
    }

    f2 = (*wm8978).f_pllout * 4;

    dev_dbg!(
        (*component).dev,
        "%s: f_MCLK=%uHz, f_PLLOUT=%uHz\n",
        __func__,
        (*wm8978).f_mclk,
        (*wm8978).f_pllout
    );

    pll_factors(component, &mut pll_div, f2, (*wm8978).f_mclk);

    dev_dbg!(
        (*component).dev,
        "%s: calculated PLL N=0x%x, K=0x%x, div2=%d\n",
        __func__,
        pll_div.n,
        pll_div.k,
        pll_div.div2
    );

    /* Turn PLL off for configuration... */
    snd_soc_component_update_bits(component, WM8978_POWER_MANAGEMENT_1, 0x20, 0);

    snd_soc_component_write(component, WM8978_PLL_N, ((pll_div.div2 as c_uint) << 4) | pll_div.n as c_uint);
    snd_soc_component_write(component, WM8978_PLL_K1, pll_div.k >> 18);
    snd_soc_component_write(component, WM8978_PLL_K2, (pll_div.k >> 9) & 0x1ff);
    snd_soc_component_write(component, WM8978_PLL_K3, pll_div.k & 0x1ff);

    /* ...and on again */
    snd_soc_component_update_bits(component, WM8978_POWER_MANAGEMENT_1, 0x20, 0x20);

    if f_opclk != 0 {
        /* Output PLL (OPCLK) to GPIO1 */
        snd_soc_component_update_bits(component, WM8978_GPIO_CONTROL, 7, 4);
    }

    0
}

/*
 * Configure WM8978 clock dividers.
 */
unsafe extern "C" fn wm8978_set_dai_clkdiv(
    codec_dai: *mut snd_soc_dai,
    div_id: c_int,
    div: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let wm8978 = snd_soc_component_get_drvdata(component) as *mut wm8978_priv;
    let mut ret: c_int = 0;

    match div_id {
        WM8978_OPCLKRATE => {
            (*wm8978).f_opclk = div as c_uint;

            if (*wm8978).f_mclk != 0 {
                /*
                 * We know the MCLK frequency, the user has requested
                 * OPCLK, configure the PLL based on that and start it
                 * and OPCLK immediately. We will configure PLL to match
                 * user-requested OPCLK frquency as good as possible.
                 * In fact, it is likely, that matching the sampling
                 * rate, when it becomes known, is more important, and
                 * we will not be reconfiguring PLL then, because we
                 * must not interrupt OPCLK. But it should be fine,
                 * because typically the user will request OPCLK to run
                 * at 256fs or 512fs, and for these cases we will also
                 * find an exact MCLK divider configuration - it will
                 * be equal to or double the OPCLK divisor.
                 */
                ret = wm8978_configure_pll(component);
            }
        }
        WM8978_BCLKDIV => {
            if div & !0x1c != 0 {
                return -EINVAL;
            }
            snd_soc_component_update_bits(component, WM8978_CLOCKING, 0x1c, div as c_uint);
        }
        _ => return -EINVAL,
    }

    dev_dbg!((*component).dev, "%s: ID %d, value %u\n", __func__, div_id, div);

    ret
}

/*
 * @freq:	when .set_pll() us not used, freq is codec MCLK input frequency
 */
unsafe extern "C" fn wm8978_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let wm8978 = snd_soc_component_get_drvdata(component) as *mut wm8978_priv;
    let mut ret: c_int = 0;

    dev_dbg!((*component).dev, "%s: ID %d, freq %u\n", __func__, clk_id, freq);

    if freq != 0 {
        (*wm8978).f_mclk = freq;

        /* Even if MCLK is used for system clock, might have to drive OPCLK */
        if (*wm8978).f_opclk != 0 {
            ret = wm8978_configure_pll(component);
        }

        /* Our sysclk is fixed to 256 * fs, will configure in .hw_params()  */

        if ret == 0 {
            (*wm8978).sysclk = clk_id as wm8978_sysclk_src;
        }
    }

    if (*wm8978).sysclk == WM8978_PLL && (freq == 0 || clk_id == WM8978_MCLK) {
        /* Clock CODEC directly from MCLK */
        snd_soc_component_update_bits(component, WM8978_CLOCKING, 0x100, 0);

        /* GPIO1 into default mode as input - before configuring PLL */
        snd_soc_component_update_bits(component, WM8978_GPIO_CONTROL, 7, 0);

        /* Turn off PLL */
        snd_soc_component_update_bits(component, WM8978_POWER_MANAGEMENT_1, 0x20, 0);
        (*wm8978).sysclk = WM8978_MCLK;
        (*wm8978).f_pllout = 0;
        (*wm8978).f_opclk = 0;
    }

    ret
}

/*
 * Set ADC and Voice DAC format.
 */
unsafe extern "C" fn wm8978_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    /*
     * BCLK polarity mask = 0x100, LRC clock polarity mask = 0x80,
     * Data Format mask = 0x18: all will be calculated anew
     */
    let mut iface: u16 = (snd_soc_component_read(component, WM8978_AUDIO_INTERFACE) & !0x198) as u16;
    let mut clk: u16 = snd_soc_component_read(component, WM8978_CLOCKING) as u16;

    dev_dbg!((*component).dev, "%s\n", __func__);

    /* set master/slave audio interface */
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => clk |= 1,
        SND_SOC_DAIFMT_CBC_CFC => clk &= !1,
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => iface |= 0x10,
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => iface |= 0x8,
        SND_SOC_DAIFMT_DSP_A => iface |= 0x18,
        _ => return -EINVAL,
    }

    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => iface |= 0x180,
        SND_SOC_DAIFMT_IB_NF => iface |= 0x100,
        SND_SOC_DAIFMT_NB_IF => iface |= 0x80,
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, WM8978_AUDIO_INTERFACE, iface as c_uint);
    snd_soc_component_write(component, WM8978_CLOCKING, clk as c_uint);

    0
}

/*
 * Set PCM DAI bit size and sample rate.
 */
unsafe extern "C" fn wm8978_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let wm8978 = snd_soc_component_get_drvdata(component) as *mut wm8978_priv;
    /* Word length mask = 0x60 */
    let mut iface_ctl: u16 = (snd_soc_component_read(component, WM8978_AUDIO_INTERFACE) & !0x60) as u16;
    /* Sampling rate mask = 0xe (for filters) */
    let mut add_ctl: u16 = (snd_soc_component_read(component, WM8978_ADDITIONAL_CONTROL) & !0xe) as u16;
    let clking: u16 = snd_soc_component_read(component, WM8978_CLOCKING) as u16;
    let current_clk_id: wm8978_sysclk_src = if clking & 0x100 != 0 {
        WM8978_PLL
    } else {
        WM8978_MCLK
    };
    let f_sel: c_uint;
    let mut diff: c_uint = 0;
    let mut diff_best: c_uint = INT_MAX as c_uint;
    let mut best: c_int = 0;

    if (*wm8978).f_mclk == 0 {
        return -EINVAL;
    }

    /* bit size */
    match params_width(params) {
        16 => {}
        20 => iface_ctl |= 0x20,
        24 => iface_ctl |= 0x40,
        32 => iface_ctl |= 0x60,
        _ => {}
    }

    /* filter coefficient */
    match params_rate(params) {
        8000 => add_ctl |= 0x5 << 1,
        11025 => add_ctl |= 0x4 << 1,
        16000 => add_ctl |= 0x3 << 1,
        22050 => add_ctl |= 0x2 << 1,
        32000 => add_ctl |= 0x1 << 1,
        44100 | 48000 => {}
        _ => {}
    }

    /* Sampling rate is known now, can configure the MCLK divider */
    (*wm8978).f_256fs = params_rate(params) * 256;

    if (*wm8978).sysclk == WM8978_MCLK {
        (*wm8978).mclk_idx = -1;
        f_sel = (*wm8978).f_mclk;
    } else {
        if (*wm8978).f_opclk == 0 {
            /* We only enter here, if OPCLK is not used */
            let ret = wm8978_configure_pll(component);
            if ret < 0 {
                return ret;
            }
        }
        f_sel = (*wm8978).f_pllout;
    }

    if (*wm8978).mclk_idx < 0 {
        /* Either MCLK is used directly, or OPCLK is used */
        if f_sel < (*wm8978).f_256fs || f_sel > 12 * (*wm8978).f_256fs {
            return -EINVAL;
        }

        let mut i: usize = 0;
        while i < mclk_numerator.len() {
            diff = abs(
                ((*wm8978).f_256fs * 3) as c_int
                    - (f_sel * 3 * mclk_denominator[i] as c_uint
                        / mclk_numerator[i] as c_uint) as c_int,
            ) as c_uint;

            if diff < diff_best {
                diff_best = diff;
                best = i as c_int;
            }

            if diff == 0 {
                break;
            }
            i += 1;
        }
    } else {
        /* OPCLK not used, codec driven by PLL */
        best = (*wm8978).mclk_idx;
        diff = 0;
    }

    if diff != 0 {
        dev_warn!(
            (*component).dev,
            "Imprecise sampling rate: %uHz%s\n",
            f_sel * mclk_denominator[best as usize] as c_uint
                / mclk_numerator[best as usize] as c_uint
                / 256,
            if (*wm8978).sysclk == WM8978_MCLK {
                ", consider using PLL"
            } else {
                ""
            }
        );
    }

    dev_dbg!(
        (*component).dev,
        "%s: width %d, rate %u, MCLK divisor #%d\n",
        __func__,
        params_width(params),
        params_rate(params),
        best
    );

    /* MCLK divisor mask = 0xe0 */
    snd_soc_component_update_bits(component, WM8978_CLOCKING, 0xe0, (best as c_uint) << 5);

    snd_soc_component_write(component, WM8978_AUDIO_INTERFACE, iface_ctl as c_uint);
    snd_soc_component_write(component, WM8978_ADDITIONAL_CONTROL, add_ctl as c_uint);

    if (*wm8978).sysclk != current_clk_id {
        if (*wm8978).sysclk == WM8978_PLL {
            /* Run CODEC from PLL instead of MCLK */
            snd_soc_component_update_bits(component, WM8978_CLOCKING, 0x100, 0x100);
        } else {
            /* Clock CODEC directly from MCLK */
            snd_soc_component_update_bits(component, WM8978_CLOCKING, 0x100, 0);
        }
    }

    0
}

unsafe extern "C" fn wm8978_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*dai).component;

    dev_dbg!((*component).dev, "%s: %d\n", __func__, mute);

    if mute != 0 {
        snd_soc_component_update_bits(component, WM8978_DAC_CONTROL, 0x40, 0x40);
    } else {
        snd_soc_component_update_bits(component, WM8978_DAC_CONTROL, 0x40, 0);
    }

    0
}

unsafe extern "C" fn wm8978_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let mut power1: u16 = (snd_soc_component_read(component, WM8978_POWER_MANAGEMENT_1) & !3) as u16;

    match level {
        SND_SOC_BIAS_ON | SND_SOC_BIAS_PREPARE => {
            power1 |= 1; /* VMID 75k */
            snd_soc_component_write(component, WM8978_POWER_MANAGEMENT_1, power1 as c_uint);
        }
        SND_SOC_BIAS_STANDBY => {
            /* bit 3: enable bias, bit 2: enable I/O tie off buffer */
            power1 |= 0xc;

            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                /* Initial cap charge at VMID 5k */
                snd_soc_component_write(component, WM8978_POWER_MANAGEMENT_1, (power1 | 0x3) as c_uint);
                mdelay(100);
            }

            power1 |= 0x2; /* VMID 500k */
            snd_soc_component_write(component, WM8978_POWER_MANAGEMENT_1, power1 as c_uint);
        }
        SND_SOC_BIAS_OFF => {
            /* Preserve PLL - OPCLK may be used by someone */
            snd_soc_component_update_bits(component, WM8978_POWER_MANAGEMENT_1, !0x20, 0);
            snd_soc_component_write(component, WM8978_POWER_MANAGEMENT_2, 0);
            snd_soc_component_write(component, WM8978_POWER_MANAGEMENT_3, 0);
        }
    }

    dev_dbg!((*component).dev, "%s: %d, %x\n", __func__, level, power1);

    0
}

static wm8978_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8978_hw_params),
    mute_stream: Some(wm8978_mute),
    set_fmt: Some(wm8978_set_dai_fmt),
    set_clkdiv: Some(wm8978_set_dai_clkdiv),
    set_sysclk: Some(wm8978_set_dai_sysclk),
    no_capture_mute: 1,
};

/* Also supports 12kHz */
static mut wm8978_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: "wm8978-hifi",
    playback: snd_soc_pcm_stream {
        stream_name: "Playback",
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: WM8978_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: "Capture",
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: WM8978_FORMATS,
    },
    ops: &wm8978_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn wm8978_suspend(component: *mut snd_soc_component) -> c_int {
    let wm8978 = snd_soc_component_get_drvdata(component) as *mut wm8978_priv;
    let dapm = snd_soc_component_to_dapm(component);

    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_OFF);
    /* Also switch PLL off */
    snd_soc_component_write(component, WM8978_POWER_MANAGEMENT_1, 0);

    regcache_mark_dirty((*wm8978).regmap);

    0
}

unsafe extern "C" fn wm8978_resume(component: *mut snd_soc_component) -> c_int {
    let wm8978 = snd_soc_component_get_drvdata(component) as *mut wm8978_priv;
    let dapm = snd_soc_component_to_dapm(component);

    /* Sync reg_cache with the hardware */
    regcache_sync((*wm8978).regmap);

    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_STANDBY);

    if (*wm8978).f_pllout != 0 {
        /* Switch PLL on */
        snd_soc_component_update_bits(component, WM8978_POWER_MANAGEMENT_1, 0x20, 0x20);
    }

    0
}

/*
 * These registers contain an "update" bit - bit 8. This means, for example,
 * that one can write new DAC digital volume for both channels, but only when
 * the update bit is set, will also the volume be updated - simultaneously for
 * both channels.
 */
static update_reg: [c_int; 10] = [
    WM8978_LEFT_DAC_DIGITAL_VOLUME,
    WM8978_RIGHT_DAC_DIGITAL_VOLUME,
    WM8978_LEFT_ADC_DIGITAL_VOLUME,
    WM8978_RIGHT_ADC_DIGITAL_VOLUME,
    WM8978_LEFT_INP_PGA_CONTROL,
    WM8978_RIGHT_INP_PGA_CONTROL,
    WM8978_LOUT1_HP_CONTROL,
    WM8978_ROUT1_HP_CONTROL,
    WM8978_LOUT2_SPK_CONTROL,
    WM8978_ROUT2_SPK_CONTROL,
];

unsafe extern "C" fn wm8978_probe(component: *mut snd_soc_component) -> c_int {
    let wm8978 = snd_soc_component_get_drvdata(component) as *mut wm8978_priv;
    let mut i: usize;

    /*
     * Set default system clock to PLL, it is more precise, this is also the
     * default hardware setting
     */
    (*wm8978).sysclk = WM8978_PLL;

    /*
     * Set the update bit in all registers, that have one. This way all
     * writes to those registers will also cause the update bit to be
     * written.
     */
    i = 0;
    while i < update_reg.len() {
        snd_soc_component_update_bits(component, update_reg[i], 0x100, 0x100);
        i += 1;
    }

    0
}

static soc_component_dev_wm8978: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8978_probe),
    suspend: Some(wm8978_suspend),
    resume: Some(wm8978_resume),
    set_bias_level: Some(wm8978_set_bias_level),
    controls: wm8978_snd_controls.as_ptr(),
    num_controls: wm8978_snd_controls.len(),
    dapm_widgets: wm8978_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8978_dapm_widgets.len(),
    dapm_routes: wm8978_dapm_routes.as_ptr(),
    num_dapm_routes: wm8978_dapm_routes.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8978_regmap_config: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,

    max_register: WM8978_MAX_REGISTER,
    volatile_reg: Some(wm8978_volatile),

    cache_type: REGCACHE_MAPLE,
    reg_defaults: wm8978_reg_defaults.as_ptr(),
    num_reg_defaults: wm8978_reg_defaults.len(),
};

unsafe extern "C" fn wm8978_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8978: *mut wm8978_priv;
    let mut ret: c_int;

    wm8978 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<wm8978_priv>(),
        GFP_KERNEL,
    ) as *mut wm8978_priv;
    if wm8978 == NULL {
        return -ENOMEM;
    }

    (*wm8978).regmap = devm_regmap_init_i2c(i2c, &wm8978_regmap_config);
    if IS_ERR((*wm8978).regmap) {
        ret = PTR_ERR((*wm8978).regmap);
        dev_err!(&mut (*i2c).dev, "Failed to allocate regmap: %d\n", ret);
        return ret;
    }

    i2c_set_clientdata(i2c, wm8978 as *mut c_void);

    /* Reset the codec */
    ret = regmap_write((*wm8978).regmap, WM8978_RESET, 0);
    if ret != 0 {
        dev_err!(&mut (*i2c).dev, "Failed to issue reset: %d\n", ret);
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm8978,
        &mut wm8978_dai,
        1,
    );
    if ret != 0 {
        dev_err!(&mut (*i2c).dev, "Failed to register CODEC: %d\n", ret);
        return ret;
    }

    0
}

static wm8978_i2c_id: &[i2c_device_id] = &[
    i2c_device_id { name: "wm8978" },
    i2c_device_id::zeroed(),
];
MODULE_DEVICE_TABLE!(i2c, wm8978_i2c_id);

static wm8978_of_match: &[of_device_id] = &[
    of_device_id { compatible: "wlf,wm8978" },
    of_device_id::zeroed(),
];
MODULE_DEVICE_TABLE!(of, wm8978_of_match);

static mut wm8978_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: "wm8978",
        of_match_table: wm8978_of_match.as_ptr(),
    },
    probe: Some(wm8978_i2c_probe),
    id_table: wm8978_i2c_id.as_ptr(),
};

module_i2c_driver!(wm8978_i2c_driver);

MODULE_DESCRIPTION!("ASoC WM8978 codec driver");
MODULE_AUTHOR!("Guennadi Liakhovetski <g.liakhovetski@gmx.de>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
