// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8983.c  --  WM8983 ALSA SoC Audio driver
 *
 * Copyright 2011 Wolfson Microelectronics plc
 *
 * Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
 */

/* Rust translation of the implementation source. Kernel/ASoC dependencies from
 * linux/module.h, linux/moduleparam.h, linux/init.h, linux/delay.h, linux/pm.h,
 * linux/i2c.h, linux/regmap.h, linux/spi/spi.h, linux/slab.h, sound/core.h,
 * sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/initval.h, sound/tlv.h,
 * sound/tlv.h, and "wm8983.h" are expected to be provided externally.
 */

static wm8983_defaults: &[reg_default] = &[
    reg_default { reg: 0x01, def: 0x0000 },     /* R1  - Power management 1 */
    reg_default { reg: 0x02, def: 0x0000 },     /* R2  - Power management 2 */
    reg_default { reg: 0x03, def: 0x0000 },     /* R3  - Power management 3 */
    reg_default { reg: 0x04, def: 0x0050 },     /* R4  - Audio Interface */
    reg_default { reg: 0x05, def: 0x0000 },     /* R5  - Companding control */
    reg_default { reg: 0x06, def: 0x0140 },     /* R6  - Clock Gen control */
    reg_default { reg: 0x07, def: 0x0000 },     /* R7  - Additional control */
    reg_default { reg: 0x08, def: 0x0000 },     /* R8  - GPIO Control */
    reg_default { reg: 0x09, def: 0x0000 },     /* R9  - Jack Detect Control 1 */
    reg_default { reg: 0x0A, def: 0x0000 },     /* R10 - DAC Control */
    reg_default { reg: 0x0B, def: 0x00FF },     /* R11 - Left DAC digital Vol */
    reg_default { reg: 0x0C, def: 0x00FF },     /* R12 - Right DAC digital vol */
    reg_default { reg: 0x0D, def: 0x0000 },     /* R13 - Jack Detect Control 2 */
    reg_default { reg: 0x0E, def: 0x0100 },     /* R14 - ADC Control */
    reg_default { reg: 0x0F, def: 0x00FF },     /* R15 - Left ADC Digital Vol */
    reg_default { reg: 0x10, def: 0x00FF },     /* R16 - Right ADC Digital Vol */
    reg_default { reg: 0x12, def: 0x012C },     /* R18 - EQ1 - low shelf */
    reg_default { reg: 0x13, def: 0x002C },     /* R19 - EQ2 - peak 1 */
    reg_default { reg: 0x14, def: 0x002C },     /* R20 - EQ3 - peak 2 */
    reg_default { reg: 0x15, def: 0x002C },     /* R21 - EQ4 - peak 3 */
    reg_default { reg: 0x16, def: 0x002C },     /* R22 - EQ5 - high shelf */
    reg_default { reg: 0x18, def: 0x0032 },     /* R24 - DAC Limiter 1 */
    reg_default { reg: 0x19, def: 0x0000 },     /* R25 - DAC Limiter 2 */
    reg_default { reg: 0x1B, def: 0x0000 },     /* R27 - Notch Filter 1 */
    reg_default { reg: 0x1C, def: 0x0000 },     /* R28 - Notch Filter 2 */
    reg_default { reg: 0x1D, def: 0x0000 },     /* R29 - Notch Filter 3 */
    reg_default { reg: 0x1E, def: 0x0000 },     /* R30 - Notch Filter 4 */
    reg_default { reg: 0x20, def: 0x0038 },     /* R32 - ALC control 1 */
    reg_default { reg: 0x21, def: 0x000B },     /* R33 - ALC control 2 */
    reg_default { reg: 0x22, def: 0x0032 },     /* R34 - ALC control 3 */
    reg_default { reg: 0x23, def: 0x0000 },     /* R35 - Noise Gate */
    reg_default { reg: 0x24, def: 0x0008 },     /* R36 - PLL N */
    reg_default { reg: 0x25, def: 0x000C },     /* R37 - PLL K 1 */
    reg_default { reg: 0x26, def: 0x0093 },     /* R38 - PLL K 2 */
    reg_default { reg: 0x27, def: 0x00E9 },     /* R39 - PLL K 3 */
    reg_default { reg: 0x29, def: 0x0000 },     /* R41 - 3D control */
    reg_default { reg: 0x2A, def: 0x0000 },     /* R42 - OUT4 to ADC */
    reg_default { reg: 0x2B, def: 0x0000 },     /* R43 - Beep control */
    reg_default { reg: 0x2C, def: 0x0033 },     /* R44 - Input ctrl */
    reg_default { reg: 0x2D, def: 0x0010 },     /* R45 - Left INP PGA gain ctrl */
    reg_default { reg: 0x2E, def: 0x0010 },     /* R46 - Right INP PGA gain ctrl */
    reg_default { reg: 0x2F, def: 0x0100 },     /* R47 - Left ADC BOOST ctrl */
    reg_default { reg: 0x30, def: 0x0100 },     /* R48 - Right ADC BOOST ctrl */
    reg_default { reg: 0x31, def: 0x0002 },     /* R49 - Output ctrl */
    reg_default { reg: 0x32, def: 0x0001 },     /* R50 - Left mixer ctrl */
    reg_default { reg: 0x33, def: 0x0001 },     /* R51 - Right mixer ctrl */
    reg_default { reg: 0x34, def: 0x0039 },     /* R52 - LOUT1 (HP) volume ctrl */
    reg_default { reg: 0x35, def: 0x0039 },     /* R53 - ROUT1 (HP) volume ctrl */
    reg_default { reg: 0x36, def: 0x0039 },     /* R54 - LOUT2 (SPK) volume ctrl */
    reg_default { reg: 0x37, def: 0x0039 },     /* R55 - ROUT2 (SPK) volume ctrl */
    reg_default { reg: 0x38, def: 0x0001 },     /* R56 - OUT3 mixer ctrl */
    reg_default { reg: 0x39, def: 0x0001 },     /* R57 - OUT4 (MONO) mix ctrl */
    reg_default { reg: 0x3D, def: 0x0000 },      /* R61 - BIAS CTRL */
];

/* vol/gain update regs */
static vol_update_regs: &[c_int] = &[
    WM8983_LEFT_DAC_DIGITAL_VOL,
    WM8983_RIGHT_DAC_DIGITAL_VOL,
    WM8983_LEFT_ADC_DIGITAL_VOL,
    WM8983_RIGHT_ADC_DIGITAL_VOL,
    WM8983_LOUT1_HP_VOLUME_CTRL,
    WM8983_ROUT1_HP_VOLUME_CTRL,
    WM8983_LOUT2_SPK_VOLUME_CTRL,
    WM8983_ROUT2_SPK_VOLUME_CTRL,
    WM8983_LEFT_INP_PGA_GAIN_CTRL,
    WM8983_RIGHT_INP_PGA_GAIN_CTRL,
];

#[repr(C)]
struct wm8983_priv {
    regmap: *mut regmap,
    sysclk: u32,
    bclk: u32,
}

#[repr(C)]
struct fs_ratio {
    div: c_int,
    ratio: c_int,
}

static fs_ratios: &[fs_ratio] = &[
    fs_ratio { div: 10, ratio: 128 },
    fs_ratio { div: 15, ratio: 192 },
    fs_ratio { div: 20, ratio: 256 },
    fs_ratio { div: 30, ratio: 384 },
    fs_ratio { div: 40, ratio: 512 },
    fs_ratio { div: 60, ratio: 768 },
    fs_ratio { div: 80, ratio: 1024 },
    fs_ratio { div: 120, ratio: 1536 },
];

static srates: &[c_int] = &[48000, 32000, 24000, 16000, 12000, 8000];
static bclk_divs: &[c_int] = &[1, 2, 4, 8, 16, 32];

DECLARE_TLV_DB_SCALE!(dac_tlv, -12700, 50, 1);
DECLARE_TLV_DB_SCALE!(adc_tlv, -12700, 50, 1);
DECLARE_TLV_DB_SCALE!(out_tlv, -5700, 100, 0);
DECLARE_TLV_DB_SCALE!(lim_thresh_tlv, -600, 100, 0);
DECLARE_TLV_DB_SCALE!(lim_boost_tlv, 0, 100, 0);
DECLARE_TLV_DB_SCALE!(alc_min_tlv, -1200, 600, 0);
DECLARE_TLV_DB_SCALE!(alc_max_tlv, -675, 600, 0);
DECLARE_TLV_DB_SCALE!(alc_tar_tlv, -2250, 150, 0);
DECLARE_TLV_DB_SCALE!(pga_vol_tlv, -1200, 75, 0);
DECLARE_TLV_DB_SCALE!(boost_tlv, -1200, 300, 1);
DECLARE_TLV_DB_SCALE!(eq_tlv, -1200, 100, 0);
DECLARE_TLV_DB_SCALE!(aux_tlv, -1500, 300, 0);
DECLARE_TLV_DB_SCALE!(bypass_tlv, -1500, 300, 0);
DECLARE_TLV_DB_SCALE!(pga_boost_tlv, 0, 2000, 0);

static alc_sel_text: &[*const c_char] = &[c_str!("Off"), c_str!("Right"), c_str!("Left"), c_str!("Stereo")];
SOC_ENUM_SINGLE_DECL!(alc_sel, WM8983_ALC_CONTROL_1, 7, alc_sel_text);
static alc_mode_text: &[*const c_char] = &[c_str!("ALC"), c_str!("Limiter")];
SOC_ENUM_SINGLE_DECL!(alc_mode, WM8983_ALC_CONTROL_3, 8, alc_mode_text);
static filter_mode_text: &[*const c_char] = &[c_str!("Audio"), c_str!("Application")];
SOC_ENUM_SINGLE_DECL!(filter_mode, WM8983_ADC_CONTROL, 7, filter_mode_text);
static eq_bw_text: &[*const c_char] = &[c_str!("Narrow"), c_str!("Wide")];
static eqmode_text: &[*const c_char] = &[c_str!("Capture"), c_str!("Playback")];
SOC_ENUM_SINGLE_EXT_DECL!(eqmode, eqmode_text);
static eq1_cutoff_text: &[*const c_char] = &[c_str!("80Hz"), c_str!("105Hz"), c_str!("135Hz"), c_str!("175Hz")];
SOC_ENUM_SINGLE_DECL!(eq1_cutoff, WM8983_EQ1_LOW_SHELF, 5, eq1_cutoff_text);
static eq2_cutoff_text: &[*const c_char] = &[c_str!("230Hz"), c_str!("300Hz"), c_str!("385Hz"), c_str!("500Hz")];
SOC_ENUM_SINGLE_DECL!(eq2_bw, WM8983_EQ2_PEAK_1, 8, eq_bw_text);
SOC_ENUM_SINGLE_DECL!(eq2_cutoff, WM8983_EQ2_PEAK_1, 5, eq2_cutoff_text);
static eq3_cutoff_text: &[*const c_char] = &[c_str!("650Hz"), c_str!("850Hz"), c_str!("1.1kHz"), c_str!("1.4kHz")];
SOC_ENUM_SINGLE_DECL!(eq3_bw, WM8983_EQ3_PEAK_2, 8, eq_bw_text);
SOC_ENUM_SINGLE_DECL!(eq3_cutoff, WM8983_EQ3_PEAK_2, 5, eq3_cutoff_text);
static eq4_cutoff_text: &[*const c_char] = &[c_str!("1.8kHz"), c_str!("2.4kHz"), c_str!("3.2kHz"), c_str!("4.1kHz")];
SOC_ENUM_SINGLE_DECL!(eq4_bw, WM8983_EQ4_PEAK_3, 8, eq_bw_text);
SOC_ENUM_SINGLE_DECL!(eq4_cutoff, WM8983_EQ4_PEAK_3, 5, eq4_cutoff_text);
static eq5_cutoff_text: &[*const c_char] = &[c_str!("5.3kHz"), c_str!("6.9kHz"), c_str!("9kHz"), c_str!("11.7kHz")];
SOC_ENUM_SINGLE_DECL!(eq5_cutoff, WM8983_EQ5_HIGH_SHELF, 5, eq5_cutoff_text);
static depth_3d_text: &[*const c_char] = &[
    c_str!("Off"), c_str!("6.67%"), c_str!("13.3%"), c_str!("20%"),
    c_str!("26.7%"), c_str!("33.3%"), c_str!("40%"), c_str!("46.6%"),
    c_str!("53.3%"), c_str!("60%"), c_str!("66.7%"), c_str!("73.3%"),
    c_str!("80%"), c_str!("86.7%"), c_str!("93.3%"), c_str!("100%"),
];
SOC_ENUM_SINGLE_DECL!(depth_3d, WM8983_3D_CONTROL, 0, depth_3d_text);

static wm8983_snd_controls: &[snd_kcontrol_new] = &[
    SOC_SINGLE!("Digital Loopback Switch", WM8983_COMPANDING_CONTROL, 0, 1, 0),
    SOC_ENUM!("ALC Capture Function", alc_sel),
    SOC_SINGLE_TLV!("ALC Capture Max Volume", WM8983_ALC_CONTROL_1, 3, 7, 0, alc_max_tlv),
    SOC_SINGLE_TLV!("ALC Capture Min Volume", WM8983_ALC_CONTROL_1, 0, 7, 0, alc_min_tlv),
    SOC_SINGLE_TLV!("ALC Capture Target Volume", WM8983_ALC_CONTROL_2, 0, 15, 0, alc_tar_tlv),
    SOC_SINGLE!("ALC Capture Attack", WM8983_ALC_CONTROL_3, 0, 10, 0),
    SOC_SINGLE!("ALC Capture Hold", WM8983_ALC_CONTROL_2, 4, 10, 0),
    SOC_SINGLE!("ALC Capture Decay", WM8983_ALC_CONTROL_3, 4, 10, 0),
    SOC_ENUM!("ALC Mode", alc_mode),
    SOC_SINGLE!("ALC Capture NG Switch", WM8983_NOISE_GATE, 3, 1, 0),
    SOC_SINGLE!("ALC Capture NG Threshold", WM8983_NOISE_GATE, 0, 7, 1),
    SOC_DOUBLE_R_TLV!("Capture Volume", WM8983_LEFT_ADC_DIGITAL_VOL, WM8983_RIGHT_ADC_DIGITAL_VOL, 0, 255, 0, adc_tlv),
    SOC_DOUBLE_R!("Capture PGA ZC Switch", WM8983_LEFT_INP_PGA_GAIN_CTRL, WM8983_RIGHT_INP_PGA_GAIN_CTRL, 7, 1, 0),
    SOC_DOUBLE_R_TLV!("Capture PGA Volume", WM8983_LEFT_INP_PGA_GAIN_CTRL, WM8983_RIGHT_INP_PGA_GAIN_CTRL, 0, 63, 0, pga_vol_tlv),
    SOC_DOUBLE_R_TLV!("Capture PGA Boost Volume", WM8983_LEFT_ADC_BOOST_CTRL, WM8983_RIGHT_ADC_BOOST_CTRL, 8, 1, 0, pga_boost_tlv),
    SOC_DOUBLE!("ADC Inversion Switch", WM8983_ADC_CONTROL, 0, 1, 1, 0),
    SOC_SINGLE!("ADC 128x Oversampling Switch", WM8983_ADC_CONTROL, 8, 1, 0),
    SOC_DOUBLE_R_TLV!("Playback Volume", WM8983_LEFT_DAC_DIGITAL_VOL, WM8983_RIGHT_DAC_DIGITAL_VOL, 0, 255, 0, dac_tlv),
    SOC_SINGLE!("DAC Playback Limiter Switch", WM8983_DAC_LIMITER_1, 8, 1, 0),
    SOC_SINGLE!("DAC Playback Limiter Decay", WM8983_DAC_LIMITER_1, 4, 10, 0),
    SOC_SINGLE!("DAC Playback Limiter Attack", WM8983_DAC_LIMITER_1, 0, 11, 0),
    SOC_SINGLE_TLV!("DAC Playback Limiter Threshold", WM8983_DAC_LIMITER_2, 4, 7, 1, lim_thresh_tlv),
    SOC_SINGLE_TLV!("DAC Playback Limiter Boost Volume", WM8983_DAC_LIMITER_2, 0, 12, 0, lim_boost_tlv),
    SOC_DOUBLE!("DAC Inversion Switch", WM8983_DAC_CONTROL, 0, 1, 1, 0),
    SOC_SINGLE!("DAC Auto Mute Switch", WM8983_DAC_CONTROL, 2, 1, 0),
    SOC_SINGLE!("DAC 128x Oversampling Switch", WM8983_DAC_CONTROL, 3, 1, 0),
    SOC_DOUBLE_R_TLV!("Headphone Playback Volume", WM8983_LOUT1_HP_VOLUME_CTRL, WM8983_ROUT1_HP_VOLUME_CTRL, 0, 63, 0, out_tlv),
    SOC_DOUBLE_R!("Headphone Playback ZC Switch", WM8983_LOUT1_HP_VOLUME_CTRL, WM8983_ROUT1_HP_VOLUME_CTRL, 7, 1, 0),
    SOC_DOUBLE_R!("Headphone Switch", WM8983_LOUT1_HP_VOLUME_CTRL, WM8983_ROUT1_HP_VOLUME_CTRL, 6, 1, 1),
    SOC_DOUBLE_R_TLV!("Speaker Playback Volume", WM8983_LOUT2_SPK_VOLUME_CTRL, WM8983_ROUT2_SPK_VOLUME_CTRL, 0, 63, 0, out_tlv),
    SOC_DOUBLE_R!("Speaker Playback ZC Switch", WM8983_LOUT2_SPK_VOLUME_CTRL, WM8983_ROUT2_SPK_VOLUME_CTRL, 7, 1, 0),
    SOC_DOUBLE_R!("Speaker Switch", WM8983_LOUT2_SPK_VOLUME_CTRL, WM8983_ROUT2_SPK_VOLUME_CTRL, 6, 1, 1),
    SOC_SINGLE!("OUT3 Switch", WM8983_OUT3_MIXER_CTRL, 6, 1, 1),
    SOC_SINGLE!("OUT4 Switch", WM8983_OUT4_MONO_MIX_CTRL, 6, 1, 1),
    SOC_SINGLE!("High Pass Filter Switch", WM8983_ADC_CONTROL, 8, 1, 0),
    SOC_ENUM!("High Pass Filter Mode", filter_mode),
    SOC_SINGLE!("High Pass Filter Cutoff", WM8983_ADC_CONTROL, 4, 7, 0),
    SOC_DOUBLE_R_TLV!("Aux Bypass Volume", WM8983_LEFT_MIXER_CTRL, WM8983_RIGHT_MIXER_CTRL, 6, 7, 0, aux_tlv),
    SOC_DOUBLE_R_TLV!("Input PGA Bypass Volume", WM8983_LEFT_MIXER_CTRL, WM8983_RIGHT_MIXER_CTRL, 2, 7, 0, bypass_tlv),
    SOC_ENUM_EXT!("Equalizer Function", eqmode, eqmode_get, eqmode_put),
    SOC_ENUM!("EQ1 Cutoff", eq1_cutoff),
    SOC_SINGLE_TLV!("EQ1 Volume", WM8983_EQ1_LOW_SHELF, 0, 24, 1, eq_tlv),
    SOC_ENUM!("EQ2 Bandwidth", eq2_bw),
    SOC_ENUM!("EQ2 Cutoff", eq2_cutoff),
    SOC_SINGLE_TLV!("EQ2 Volume", WM8983_EQ2_PEAK_1, 0, 24, 1, eq_tlv),
    SOC_ENUM!("EQ3 Bandwidth", eq3_bw),
    SOC_ENUM!("EQ3 Cutoff", eq3_cutoff),
    SOC_SINGLE_TLV!("EQ3 Volume", WM8983_EQ3_PEAK_2, 0, 24, 1, eq_tlv),
    SOC_ENUM!("EQ4 Bandwidth", eq4_bw),
    SOC_ENUM!("EQ4 Cutoff", eq4_cutoff),
    SOC_SINGLE_TLV!("EQ4 Volume", WM8983_EQ4_PEAK_3, 0, 24, 1, eq_tlv),
    SOC_ENUM!("EQ5 Cutoff", eq5_cutoff),
    SOC_SINGLE_TLV!("EQ5 Volume", WM8983_EQ5_HIGH_SHELF, 0, 24, 1, eq_tlv),
    SOC_ENUM!("3D Depth", depth_3d),
];

static left_out_mixer: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Line Switch", WM8983_LEFT_MIXER_CTRL, 1, 1, 0),
    SOC_DAPM_SINGLE!("Aux Switch", WM8983_LEFT_MIXER_CTRL, 5, 1, 0),
    SOC_DAPM_SINGLE!("PCM Switch", WM8983_LEFT_MIXER_CTRL, 0, 1, 0),
];
static right_out_mixer: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Line Switch", WM8983_RIGHT_MIXER_CTRL, 1, 1, 0),
    SOC_DAPM_SINGLE!("Aux Switch", WM8983_RIGHT_MIXER_CTRL, 5, 1, 0),
    SOC_DAPM_SINGLE!("PCM Switch", WM8983_RIGHT_MIXER_CTRL, 0, 1, 0),
];
static left_input_mixer: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("L2 Switch", WM8983_INPUT_CTRL, 2, 1, 0),
    SOC_DAPM_SINGLE!("MicN Switch", WM8983_INPUT_CTRL, 1, 1, 0),
    SOC_DAPM_SINGLE!("MicP Switch", WM8983_INPUT_CTRL, 0, 1, 0),
];
static right_input_mixer: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("R2 Switch", WM8983_INPUT_CTRL, 6, 1, 0),
    SOC_DAPM_SINGLE!("MicN Switch", WM8983_INPUT_CTRL, 5, 1, 0),
    SOC_DAPM_SINGLE!("MicP Switch", WM8983_INPUT_CTRL, 4, 1, 0),
];
static left_boost_mixer: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_TLV!("L2 Volume", WM8983_LEFT_ADC_BOOST_CTRL, 4, 7, 0, boost_tlv),
    SOC_DAPM_SINGLE_TLV!("AUXL Volume", WM8983_LEFT_ADC_BOOST_CTRL, 0, 7, 0, boost_tlv),
];
static out3_mixer: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("LMIX2OUT3 Switch", WM8983_OUT3_MIXER_CTRL, 1, 1, 0),
    SOC_DAPM_SINGLE!("LDAC2OUT3 Switch", WM8983_OUT3_MIXER_CTRL, 0, 1, 0),
];
static out4_mixer: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("LMIX2OUT4 Switch", WM8983_OUT4_MONO_MIX_CTRL, 4, 1, 0),
    SOC_DAPM_SINGLE!("RMIX2OUT4 Switch", WM8983_OUT4_MONO_MIX_CTRL, 1, 1, 0),
    SOC_DAPM_SINGLE!("LDAC2OUT4 Switch", WM8983_OUT4_MONO_MIX_CTRL, 3, 1, 0),
    SOC_DAPM_SINGLE!("RDAC2OUT4 Switch", WM8983_OUT4_MONO_MIX_CTRL, 0, 1, 0),
];
static right_boost_mixer: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_TLV!("R2 Volume", WM8983_RIGHT_ADC_BOOST_CTRL, 4, 7, 0, boost_tlv),
    SOC_DAPM_SINGLE_TLV!("AUXR Volume", WM8983_RIGHT_ADC_BOOST_CTRL, 0, 7, 0, boost_tlv),
];

static wm8983_dapm_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_DAC!("Left DAC", "Left Playback", WM8983_POWER_MANAGEMENT_3, 0, 0),
    SND_SOC_DAPM_DAC!("Right DAC", "Right Playback", WM8983_POWER_MANAGEMENT_3, 1, 0),
    SND_SOC_DAPM_ADC!("Left ADC", "Left Capture", WM8983_POWER_MANAGEMENT_2, 0, 0),
    SND_SOC_DAPM_ADC!("Right ADC", "Right Capture", WM8983_POWER_MANAGEMENT_2, 1, 0),
    SND_SOC_DAPM_MIXER!("Left Output Mixer", WM8983_POWER_MANAGEMENT_3, 2, 0, left_out_mixer, left_out_mixer.len()),
    SND_SOC_DAPM_MIXER!("Right Output Mixer", WM8983_POWER_MANAGEMENT_3, 3, 0, right_out_mixer, right_out_mixer.len()),
    SND_SOC_DAPM_MIXER!("Left Input Mixer", WM8983_POWER_MANAGEMENT_2, 2, 0, left_input_mixer, left_input_mixer.len()),
    SND_SOC_DAPM_MIXER!("Right Input Mixer", WM8983_POWER_MANAGEMENT_2, 3, 0, right_input_mixer, right_input_mixer.len()),
    SND_SOC_DAPM_MIXER!("Left Boost Mixer", WM8983_POWER_MANAGEMENT_2, 4, 0, left_boost_mixer, left_boost_mixer.len()),
    SND_SOC_DAPM_MIXER!("Right Boost Mixer", WM8983_POWER_MANAGEMENT_2, 5, 0, right_boost_mixer, right_boost_mixer.len()),
    SND_SOC_DAPM_MIXER!("OUT3 Mixer", WM8983_POWER_MANAGEMENT_1, 6, 0, out3_mixer, out3_mixer.len()),
    SND_SOC_DAPM_MIXER!("OUT4 Mixer", WM8983_POWER_MANAGEMENT_1, 7, 0, out4_mixer, out4_mixer.len()),
    SND_SOC_DAPM_PGA!("Left Capture PGA", WM8983_LEFT_INP_PGA_GAIN_CTRL, 6, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right Capture PGA", WM8983_RIGHT_INP_PGA_GAIN_CTRL, 6, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left Headphone Out", WM8983_POWER_MANAGEMENT_2, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right Headphone Out", WM8983_POWER_MANAGEMENT_2, 8, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left Speaker Out", WM8983_POWER_MANAGEMENT_3, 5, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right Speaker Out", WM8983_POWER_MANAGEMENT_3, 6, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("OUT3 Out", WM8983_POWER_MANAGEMENT_3, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("OUT4 Out", WM8983_POWER_MANAGEMENT_3, 8, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Bias", WM8983_POWER_MANAGEMENT_1, 4, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_INPUT!("LIN"), SND_SOC_DAPM_INPUT!("LIP"),
    SND_SOC_DAPM_INPUT!("RIN"), SND_SOC_DAPM_INPUT!("RIP"),
    SND_SOC_DAPM_INPUT!("AUXL"), SND_SOC_DAPM_INPUT!("AUXR"),
    SND_SOC_DAPM_INPUT!("L2"), SND_SOC_DAPM_INPUT!("R2"),
    SND_SOC_DAPM_OUTPUT!("HPL"), SND_SOC_DAPM_OUTPUT!("HPR"),
    SND_SOC_DAPM_OUTPUT!("SPKL"), SND_SOC_DAPM_OUTPUT!("SPKR"),
    SND_SOC_DAPM_OUTPUT!("OUT3"), SND_SOC_DAPM_OUTPUT!("OUT4"),
];

static wm8983_audio_map: &[snd_soc_dapm_route] = &[
    snd_soc_dapm_route { sink: c_str!("OUT3 Mixer"), control: c_str!("LMIX2OUT3 Switch"), source: c_str!("Left Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("OUT3 Mixer"), control: c_str!("LDAC2OUT3 Switch"), source: c_str!("Left DAC") },
    snd_soc_dapm_route { sink: c_str!("OUT3 Out"), control: core::ptr::null(), source: c_str!("OUT3 Mixer") },
    snd_soc_dapm_route { sink: c_str!("OUT3"), control: core::ptr::null(), source: c_str!("OUT3 Out") },
    snd_soc_dapm_route { sink: c_str!("OUT4 Mixer"), control: c_str!("LMIX2OUT4 Switch"), source: c_str!("Left Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("OUT4 Mixer"), control: c_str!("RMIX2OUT4 Switch"), source: c_str!("Right Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("OUT4 Mixer"), control: c_str!("LDAC2OUT4 Switch"), source: c_str!("Left DAC") },
    snd_soc_dapm_route { sink: c_str!("OUT4 Mixer"), control: c_str!("RDAC2OUT4 Switch"), source: c_str!("Right DAC") },
    snd_soc_dapm_route { sink: c_str!("OUT4 Out"), control: core::ptr::null(), source: c_str!("OUT4 Mixer") },
    snd_soc_dapm_route { sink: c_str!("OUT4"), control: core::ptr::null(), source: c_str!("OUT4 Out") },
    snd_soc_dapm_route { sink: c_str!("Right Output Mixer"), control: c_str!("PCM Switch"), source: c_str!("Right DAC") },
    snd_soc_dapm_route { sink: c_str!("Right Output Mixer"), control: c_str!("Aux Switch"), source: c_str!("AUXR") },
    snd_soc_dapm_route { sink: c_str!("Right Output Mixer"), control: c_str!("Line Switch"), source: c_str!("Right Boost Mixer") },
    snd_soc_dapm_route { sink: c_str!("Left Output Mixer"), control: c_str!("PCM Switch"), source: c_str!("Left DAC") },
    snd_soc_dapm_route { sink: c_str!("Left Output Mixer"), control: c_str!("Aux Switch"), source: c_str!("AUXL") },
    snd_soc_dapm_route { sink: c_str!("Left Output Mixer"), control: c_str!("Line Switch"), source: c_str!("Left Boost Mixer") },
    snd_soc_dapm_route { sink: c_str!("Right Headphone Out"), control: core::ptr::null(), source: c_str!("Right Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("HPR"), control: core::ptr::null(), source: c_str!("Right Headphone Out") },
    snd_soc_dapm_route { sink: c_str!("Left Headphone Out"), control: core::ptr::null(), source: c_str!("Left Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("HPL"), control: core::ptr::null(), source: c_str!("Left Headphone Out") },
    snd_soc_dapm_route { sink: c_str!("Right Speaker Out"), control: core::ptr::null(), source: c_str!("Right Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("SPKR"), control: core::ptr::null(), source: c_str!("Right Speaker Out") },
    snd_soc_dapm_route { sink: c_str!("Left Speaker Out"), control: core::ptr::null(), source: c_str!("Left Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("SPKL"), control: core::ptr::null(), source: c_str!("Left Speaker Out") },
    snd_soc_dapm_route { sink: c_str!("Right ADC"), control: core::ptr::null(), source: c_str!("Right Boost Mixer") },
    snd_soc_dapm_route { sink: c_str!("Right Boost Mixer"), control: c_str!("AUXR Volume"), source: c_str!("AUXR") },
    snd_soc_dapm_route { sink: c_str!("Right Boost Mixer"), control: core::ptr::null(), source: c_str!("Right Capture PGA") },
    snd_soc_dapm_route { sink: c_str!("Right Boost Mixer"), control: c_str!("R2 Volume"), source: c_str!("R2") },
    snd_soc_dapm_route { sink: c_str!("Left ADC"), control: core::ptr::null(), source: c_str!("Left Boost Mixer") },
    snd_soc_dapm_route { sink: c_str!("Left Boost Mixer"), control: c_str!("AUXL Volume"), source: c_str!("AUXL") },
    snd_soc_dapm_route { sink: c_str!("Left Boost Mixer"), control: core::ptr::null(), source: c_str!("Left Capture PGA") },
    snd_soc_dapm_route { sink: c_str!("Left Boost Mixer"), control: c_str!("L2 Volume"), source: c_str!("L2") },
    snd_soc_dapm_route { sink: c_str!("Right Capture PGA"), control: core::ptr::null(), source: c_str!("Right Input Mixer") },
    snd_soc_dapm_route { sink: c_str!("Left Capture PGA"), control: core::ptr::null(), source: c_str!("Left Input Mixer") },
    snd_soc_dapm_route { sink: c_str!("Right Input Mixer"), control: c_str!("R2 Switch"), source: c_str!("R2") },
    snd_soc_dapm_route { sink: c_str!("Right Input Mixer"), control: c_str!("MicN Switch"), source: c_str!("RIN") },
    snd_soc_dapm_route { sink: c_str!("Right Input Mixer"), control: c_str!("MicP Switch"), source: c_str!("RIP") },
    snd_soc_dapm_route { sink: c_str!("Left Input Mixer"), control: c_str!("L2 Switch"), source: c_str!("L2") },
    snd_soc_dapm_route { sink: c_str!("Left Input Mixer"), control: c_str!("MicN Switch"), source: c_str!("LIN") },
    snd_soc_dapm_route { sink: c_str!("Left Input Mixer"), control: c_str!("MicP Switch"), source: c_str!("LIP") },
];

unsafe fn eqmode_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let reg: c_uint = snd_soc_component_read(component, WM8983_EQ1_LOW_SHELF);
    if (reg & WM8983_EQ3DMODE) != 0 {
        (*ucontrol).value.enumerated.item[0] = 1;
    } else {
        (*ucontrol).value.enumerated.item[0] = 0;
    }
    0
}

unsafe fn eqmode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let regpwr2: c_uint;
    let regpwr3: c_uint;
    let reg_eq: c_uint;

    if (*ucontrol).value.enumerated.item[0] != 0 && (*ucontrol).value.enumerated.item[0] != 1 {
        return -EINVAL;
    }

    reg_eq = snd_soc_component_read(component, WM8983_EQ1_LOW_SHELF);
    match (reg_eq & WM8983_EQ3DMODE) >> WM8983_EQ3DMODE_SHIFT {
        0 => {
            if (*ucontrol).value.enumerated.item[0] == 0 {
                return 0;
            }
        }
        1 => {
            if (*ucontrol).value.enumerated.item[0] != 0 {
                return 0;
            }
        }
        _ => {}
    }

    regpwr2 = snd_soc_component_read(component, WM8983_POWER_MANAGEMENT_2);
    regpwr3 = snd_soc_component_read(component, WM8983_POWER_MANAGEMENT_3);
    /* disable the DACs and ADCs */
    snd_soc_component_update_bits(component, WM8983_POWER_MANAGEMENT_2, WM8983_ADCENR_MASK | WM8983_ADCENL_MASK, 0);
    snd_soc_component_update_bits(component, WM8983_POWER_MANAGEMENT_3, WM8983_DACENR_MASK | WM8983_DACENL_MASK, 0);
    /* set the desired eqmode */
    snd_soc_component_update_bits(component, WM8983_EQ1_LOW_SHELF, WM8983_EQ3DMODE_MASK, (*ucontrol).value.enumerated.item[0] << WM8983_EQ3DMODE_SHIFT);
    /* restore DAC/ADC configuration */
    snd_soc_component_write(component, WM8983_POWER_MANAGEMENT_2, regpwr2);
    snd_soc_component_write(component, WM8983_POWER_MANAGEMENT_3, regpwr3);
    0
}

unsafe fn wm8983_writeable(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        WM8983_SOFTWARE_RESET..=WM8983_RIGHT_ADC_DIGITAL_VOL
        | WM8983_EQ1_LOW_SHELF..=WM8983_DAC_LIMITER_2
        | WM8983_NOTCH_FILTER_1..=WM8983_NOTCH_FILTER_4
        | WM8983_ALC_CONTROL_1..=WM8983_PLL_K_3
        | WM8983_3D_CONTROL..=WM8983_OUT4_MONO_MIX_CTRL
        | WM8983_BIAS_CTRL => true,
        _ => false,
    }
}

unsafe fn wm8983_dac_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    snd_soc_component_update_bits(component, WM8983_DAC_CONTROL, WM8983_SOFTMUTE_MASK, ((mute != 0) as c_uint) << WM8983_SOFTMUTE_SHIFT)
}

unsafe fn wm8983_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let format: u16;
    let master: u16;
    let mut bcp: u16;
    let mut lrp: u16;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => format = 0x2,
        SND_SOC_DAIFMT_RIGHT_J => format = 0x0,
        SND_SOC_DAIFMT_LEFT_J => format = 0x1,
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => format = 0x3,
        _ => {
            dev_err((*dai).dev, c_str!("Unknown dai format\n"));
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, WM8983_AUDIO_INTERFACE, WM8983_FMT_MASK, (format as c_uint) << WM8983_FMT_SHIFT);

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => master = 1,
        SND_SOC_DAIFMT_CBC_CFC => master = 0,
        _ => {
            dev_err((*dai).dev, c_str!("Unknown master/slave configuration\n"));
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, WM8983_CLOCK_GEN_CONTROL, WM8983_MS_MASK, (master as c_uint) << WM8983_MS_SHIFT);

    /* FIXME: We don't currently support DSP A/B modes */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => {
            dev_err((*dai).dev, c_str!("DSP A/B modes are not supported\n"));
            return -EINVAL;
        }
        _ => {}
    }

    bcp = 0;
    lrp = 0;
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => {
            bcp = 1;
            lrp = 1;
        }
        SND_SOC_DAIFMT_IB_NF => bcp = 1,
        SND_SOC_DAIFMT_NB_IF => lrp = 1,
        _ => {
            dev_err((*dai).dev, c_str!("Unknown polarity configuration\n"));
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, WM8983_AUDIO_INTERFACE, WM8983_LRCP_MASK, (lrp as c_uint) << WM8983_LRCP_SHIFT);
    snd_soc_component_update_bits(component, WM8983_AUDIO_INTERFACE, WM8983_BCP_MASK, (bcp as c_uint) << WM8983_BCP_SHIFT);
    0
}

unsafe fn wm8983_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let _ = substream;
    let component: *mut snd_soc_component = (*dai).component;
    let wm8983: *mut wm8983_priv = snd_soc_component_get_drvdata(component) as *mut wm8983_priv;
    let blen: u16;
    let mut srate_idx: u16;
    let tmp: u32;
    let mut srate_best: c_int;
    let mut ret: c_int;

    ret = snd_soc_params_to_bclk(params);
    if ret < 0 {
        dev_err((*component).dev, c_str!("Failed to convert params to bclk: %d\n"), ret);
        return ret;
    }
    (*wm8983).bclk = ret as u32;

    match params_width(params) {
        16 => blen = 0x0,
        20 => blen = 0x1,
        24 => blen = 0x2,
        32 => blen = 0x3,
        _ => {
            dev_err((*dai).dev, c_str!("Unsupported word length %u\n"), params_width(params));
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, WM8983_AUDIO_INTERFACE, WM8983_WL_MASK, (blen as c_uint) << WM8983_WL_SHIFT);

    /*
     * match to the nearest possible sample rate and rely
     * on the array index to configure the SR register
     */
    srate_idx = 0;
    srate_best = (srates[0] - params_rate(params)).abs();
    let mut i: usize = 1;
    while i < srates.len() {
        if (srates[i] - params_rate(params)).abs() < srate_best {
            srate_idx = i as u16;
            srate_best = (srates[i] - params_rate(params)).abs();
        }
        i += 1;
    }

    dev_dbg((*dai).dev, c_str!("Selected SRATE = %d\n"), srates[srate_idx as usize]);
    snd_soc_component_update_bits(component, WM8983_ADDITIONAL_CONTROL, WM8983_SR_MASK, (srate_idx as c_uint) << WM8983_SR_SHIFT);

    dev_dbg((*dai).dev, c_str!("Target BCLK = %uHz\n"), (*wm8983).bclk);
    dev_dbg((*dai).dev, c_str!("SYSCLK = %uHz\n"), (*wm8983).sysclk);

    i = 0;
    while i < fs_ratios.len() {
        if (*wm8983).sysclk / params_rate(params) as u32 == fs_ratios[i].ratio as u32 {
            break;
        }
        i += 1;
    }
    if i == fs_ratios.len() {
        dev_err((*dai).dev, c_str!("Unable to configure MCLK ratio %u/%u\n"), (*wm8983).sysclk, params_rate(params));
        return -EINVAL;
    }

    dev_dbg((*dai).dev, c_str!("MCLK ratio = %dfs\n"), fs_ratios[i].ratio);
    snd_soc_component_update_bits(component, WM8983_CLOCK_GEN_CONTROL, WM8983_MCLKDIV_MASK, (i as c_uint) << WM8983_MCLKDIV_SHIFT);

    /* select the appropriate bclk divider */
    tmp = ((*wm8983).sysclk / fs_ratios[i].div as u32) * 10;
    i = 0;
    while i < bclk_divs.len() {
        if (*wm8983).bclk == tmp / bclk_divs[i] as u32 {
            break;
        }
        i += 1;
    }
    if i == bclk_divs.len() {
        dev_err((*dai).dev, c_str!("No matching BCLK divider found\n"));
        return -EINVAL;
    }

    dev_dbg((*dai).dev, c_str!("BCLK div = %d\n"), i as c_int);
    snd_soc_component_update_bits(component, WM8983_CLOCK_GEN_CONTROL, WM8983_BCLKDIV_MASK, (i as c_uint) << WM8983_BCLKDIV_SHIFT);
    0
}

#[repr(C)]
struct pll_div {
    div2: u32,
    n: u32,
    k: u32,
}

const FIXED_PLL_SIZE: u64 = (1u64 << 24) * 10;

unsafe fn pll_factors(pll_div: *mut pll_div, target: c_uint, mut source: c_uint) -> c_int {
    let mut kpart: u64;
    let mut k: c_ulong;
    let mut ndiv: c_ulong;
    let nmod: c_ulong;

    (*pll_div).div2 = 0;
    ndiv = (target / source) as c_ulong;
    if ndiv < 6 {
        source >>= 1;
        (*pll_div).div2 = 1;
        ndiv = (target / source) as c_ulong;
    }

    if ndiv < 6 || ndiv > 12 {
        printk(KERN_ERR, c_str!("%s: WM8983 N value is not within the recommended range: %lu\n"), __func__, ndiv);
        return -EINVAL;
    }
    (*pll_div).n = ndiv as u32;

    nmod = (target % source) as c_ulong;
    kpart = FIXED_PLL_SIZE * nmod as u64;
    do_div(&mut kpart, source);

    k = (kpart & 0xffffffff) as c_ulong;
    if (k % 10) >= 5 {
        k += 5;
    }
    k /= 10;
    (*pll_div).k = k as u32;
    0
}

unsafe fn wm8983_set_pll(dai: *mut snd_soc_dai, _pll_id: c_int, _source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int {
    let ret: c_int;
    let component: *mut snd_soc_component = (*dai).component;
    let mut pll_div = pll_div { div2: 0, n: 0, k: 0 };

    if freq_in == 0 || freq_out == 0 {
        /* disable the PLL */
        snd_soc_component_update_bits(component, WM8983_POWER_MANAGEMENT_1, WM8983_PLLEN_MASK, 0);
        return 0;
    } else {
        ret = pll_factors(&mut pll_div, freq_out * 4 * 2, freq_in);
        if ret != 0 {
            return ret;
        }
        /* disable the PLL before re-programming it */
        snd_soc_component_update_bits(component, WM8983_POWER_MANAGEMENT_1, WM8983_PLLEN_MASK, 0);
        /* set PLLN and PRESCALE */
        snd_soc_component_write(component, WM8983_PLL_N, (pll_div.div2 << WM8983_PLL_PRESCALE_SHIFT) | pll_div.n);
        /* set PLLK */
        snd_soc_component_write(component, WM8983_PLL_K_3, pll_div.k & 0x1ff);
        snd_soc_component_write(component, WM8983_PLL_K_2, (pll_div.k >> 9) & 0x1ff);
        snd_soc_component_write(component, WM8983_PLL_K_1, pll_div.k >> 18);
        /* enable the PLL */
        snd_soc_component_update_bits(component, WM8983_POWER_MANAGEMENT_1, WM8983_PLLEN_MASK, WM8983_PLLEN);
    }
    0
}

unsafe fn wm8983_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let wm8983: *mut wm8983_priv = snd_soc_component_get_drvdata(component) as *mut wm8983_priv;

    match clk_id {
        WM8983_CLKSRC_MCLK => {
            snd_soc_component_update_bits(component, WM8983_CLOCK_GEN_CONTROL, WM8983_CLKSEL_MASK, 0);
        }
        WM8983_CLKSRC_PLL => {
            snd_soc_component_update_bits(component, WM8983_CLOCK_GEN_CONTROL, WM8983_CLKSEL_MASK, WM8983_CLKSEL);
        }
        _ => {
            dev_err((*dai).dev, c_str!("Unknown clock source: %d\n"), clk_id);
            return -EINVAL;
        }
    }
    (*wm8983).sysclk = freq;
    0
}

unsafe fn wm8983_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let wm8983: *mut wm8983_priv = snd_soc_component_get_drvdata(component) as *mut wm8983_priv;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    match level {
        SND_SOC_BIAS_ON | SND_SOC_BIAS_PREPARE => {
            /* VMID at 100k */
            snd_soc_component_update_bits(component, WM8983_POWER_MANAGEMENT_1, WM8983_VMIDSEL_MASK, 1 << WM8983_VMIDSEL_SHIFT);
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                ret = regcache_sync((*wm8983).regmap);
                if ret < 0 {
                    dev_err((*component).dev, c_str!("Failed to sync cache: %d\n"), ret);
                    return ret;
                }
                /* enable anti-pop features */
                snd_soc_component_update_bits(component, WM8983_OUT4_TO_ADC, WM8983_POBCTRL_MASK | WM8983_DELEN_MASK, WM8983_POBCTRL | WM8983_DELEN);
                /* enable thermal shutdown */
                snd_soc_component_update_bits(component, WM8983_OUTPUT_CTRL, WM8983_TSDEN_MASK, WM8983_TSDEN);
                /* enable BIASEN */
                snd_soc_component_update_bits(component, WM8983_POWER_MANAGEMENT_1, WM8983_BIASEN_MASK, WM8983_BIASEN);
                /* VMID at 100k */
                snd_soc_component_update_bits(component, WM8983_POWER_MANAGEMENT_1, WM8983_VMIDSEL_MASK, 1 << WM8983_VMIDSEL_SHIFT);
                msleep(250);
                /* disable anti-pop features */
                snd_soc_component_update_bits(component, WM8983_OUT4_TO_ADC, WM8983_POBCTRL_MASK | WM8983_DELEN_MASK, 0);
            }
            /* VMID at 500k */
            snd_soc_component_update_bits(component, WM8983_POWER_MANAGEMENT_1, WM8983_VMIDSEL_MASK, 2 << WM8983_VMIDSEL_SHIFT);
        }
        SND_SOC_BIAS_OFF => {
            /* disable thermal shutdown */
            snd_soc_component_update_bits(component, WM8983_OUTPUT_CTRL, WM8983_TSDEN_MASK, 0);
            /* disable VMIDSEL and BIASEN */
            snd_soc_component_update_bits(component, WM8983_POWER_MANAGEMENT_1, WM8983_VMIDSEL_MASK | WM8983_BIASEN_MASK, 0);
            /* wait for VMID to discharge */
            msleep(100);
            snd_soc_component_write(component, WM8983_POWER_MANAGEMENT_1, 0);
            snd_soc_component_write(component, WM8983_POWER_MANAGEMENT_2, 0);
            snd_soc_component_write(component, WM8983_POWER_MANAGEMENT_3, 0);
        }
    }
    0
}

unsafe fn wm8983_probe(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;
    ret = snd_soc_component_write(component, WM8983_SOFTWARE_RESET, 0);
    if ret < 0 {
        dev_err((*component).dev, c_str!("Failed to issue reset: %d\n"), ret);
        return ret;
    }

    /* set the vol/gain update bits */
    let mut i: usize = 0;
    while i < vol_update_regs.len() {
        snd_soc_component_update_bits(component, vol_update_regs[i] as c_uint, 0x100, 0x100);
        i += 1;
    }

    /* mute all outputs and set PGAs to minimum gain */
    let mut reg = WM8983_LOUT1_HP_VOLUME_CTRL;
    while reg <= WM8983_OUT4_MONO_MIX_CTRL {
        snd_soc_component_update_bits(component, reg, 0x40, 0x40);
        reg += 1;
    }

    /* enable soft mute */
    snd_soc_component_update_bits(component, WM8983_DAC_CONTROL, WM8983_SOFTMUTE_MASK, WM8983_SOFTMUTE);
    /* enable BIASCUT */
    snd_soc_component_update_bits(component, WM8983_BIAS_CTRL, WM8983_BIASCUT, WM8983_BIASCUT);
    0
}

static wm8983_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    mute_stream: Some(wm8983_dac_mute),
    hw_params: Some(wm8983_hw_params),
    set_fmt: Some(wm8983_set_fmt),
    set_sysclk: Some(wm8983_set_sysclk),
    set_pll: Some(wm8983_set_pll),
    no_capture_mute: 1,
    ..snd_soc_dai_ops::default()
};

const WM8983_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut wm8983_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c_str!("wm8983-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("Playback"),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: WM8983_FORMATS,
        ..snd_soc_pcm_stream::default()
    },
    capture: snd_soc_pcm_stream {
        stream_name: c_str!("Capture"),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: WM8983_FORMATS,
        ..snd_soc_pcm_stream::default()
    },
    ops: &wm8983_dai_ops,
    symmetric_rate: 1,
    ..snd_soc_dai_driver::default()
};

static soc_component_dev_wm8983: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8983_probe),
    set_bias_level: Some(wm8983_set_bias_level),
    controls: wm8983_snd_controls.as_ptr(),
    num_controls: wm8983_snd_controls.len(),
    dapm_widgets: wm8983_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8983_dapm_widgets.len(),
    dapm_routes: wm8983_audio_map.as_ptr(),
    num_dapm_routes: wm8983_audio_map.len(),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..snd_soc_component_driver::default()
};

static wm8983_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    reg_defaults: wm8983_defaults.as_ptr(),
    num_reg_defaults: wm8983_defaults.len(),
    cache_type: REGCACHE_MAPLE,
    max_register: WM8983_MAX_REGISTER,
    writeable_reg: Some(wm8983_writeable),
    ..regmap_config::default()
};

/* #if defined(CONFIG_SPI_MASTER) */
unsafe fn wm8983_spi_probe(spi: *mut spi_device) -> c_int {
    let wm8983: *mut wm8983_priv;
    let ret: c_int;

    wm8983 = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<wm8983_priv>(), GFP_KERNEL) as *mut wm8983_priv;
    if wm8983.is_null() {
        return -ENOMEM;
    }

    (*wm8983).regmap = devm_regmap_init_spi(spi, &wm8983_regmap);
    if IS_ERR((*wm8983).regmap) {
        let err = PTR_ERR((*wm8983).regmap);
        dev_err(&mut (*spi).dev, c_str!("Failed to init regmap: %d\n"), err);
        return err;
    }

    spi_set_drvdata(spi, wm8983 as *mut c_void);
    ret = devm_snd_soc_register_component(&mut (*spi).dev, &soc_component_dev_wm8983, &mut wm8983_dai, 1);
    ret
}

static mut wm8983_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c_str!("wm8983"),
        ..device_driver::default()
    },
    probe: Some(wm8983_spi_probe),
    ..spi_driver::default()
};
/* #endif */

/* #if IS_ENABLED(CONFIG_I2C) */
unsafe fn wm8983_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8983: *mut wm8983_priv;
    let ret: c_int;

    wm8983 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8983_priv>(), GFP_KERNEL) as *mut wm8983_priv;
    if wm8983.is_null() {
        return -ENOMEM;
    }

    (*wm8983).regmap = devm_regmap_init_i2c(i2c, &wm8983_regmap);
    if IS_ERR((*wm8983).regmap) {
        let err = PTR_ERR((*wm8983).regmap);
        dev_err(&mut (*i2c).dev, c_str!("Failed to init regmap: %d\n"), err);
        return err;
    }

    i2c_set_clientdata(i2c, wm8983 as *mut c_void);
    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_wm8983, &mut wm8983_dai, 1);
    ret
}

static wm8983_i2c_id: &[i2c_device_id] = &[
    i2c_device_id { name: c_str!("wm8983"), ..i2c_device_id::default() },
    i2c_device_id { ..i2c_device_id::default() },
];
MODULE_DEVICE_TABLE!(i2c, wm8983_i2c_id);

static mut wm8983_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c_str!("wm8983"),
        ..device_driver::default()
    },
    probe: Some(wm8983_i2c_probe),
    id_table: wm8983_i2c_id.as_ptr(),
    ..i2c_driver::default()
};
/* #endif */

unsafe fn wm8983_modinit() -> c_int {
    let mut ret: c_int = 0;

    /* #if IS_ENABLED(CONFIG_I2C) */
    ret = i2c_add_driver(&mut wm8983_i2c_driver);
    if ret != 0 {
        printk(KERN_ERR, c_str!("Failed to register wm8983 I2C driver: %d\n"), ret);
    }
    /* #endif */
    /* #if defined(CONFIG_SPI_MASTER) */
    ret = spi_register_driver(&mut wm8983_spi_driver);
    if ret != 0 {
        printk(KERN_ERR, c_str!("Failed to register wm8983 SPI driver: %d\n"), ret);
    }
    /* #endif */
    ret
}
module_init!(wm8983_modinit);

unsafe fn wm8983_exit() {
    /* #if IS_ENABLED(CONFIG_I2C) */
    i2c_del_driver(&mut wm8983_i2c_driver);
    /* #endif */
    /* #if defined(CONFIG_SPI_MASTER) */
    spi_unregister_driver(&mut wm8983_spi_driver);
    /* #endif */
}
module_exit!(wm8983_exit);

MODULE_DESCRIPTION!("ASoC WM8983 driver");
MODULE_AUTHOR!("Dimitris Papastamos <dp@opensource.wolfsonmicro.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
