// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8985.c  --  WM8985 / WM8758 ALSA SoC Audio driver
 *
 * Copyright 2010 Wolfson Microelectronics plc
 * Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
 *
 * WM8758 support:
 * Copyright: 2016 Barix AG
 * Author: Petr Kulhavy <petr@barix.com>
 *
 * TODO:
 *  o Add OUT3/OUT4 mixer controls.
 */

/* Dependencies from Linux, ALSA SoC, and "wm8985.h" are external. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const WM8985_NUM_SUPPLIES: usize = 4;
static wm8985_supply_names: [*const c_char; WM8985_NUM_SUPPLIES] = [
    b"DCVDD\0".as_ptr() as *const c_char,
    b"DBVDD\0".as_ptr() as *const c_char,
    b"AVDD1\0".as_ptr() as *const c_char,
    b"AVDD2\0".as_ptr() as *const c_char,
];

#[repr(C)]
#[derive(Copy, Clone)]
enum wm8985_type {
    WM8985,
    WM8758,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}
#[repr(C)]
struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
struct regulator_bulk_data {
    supply: *const c_char,
}
#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
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
struct snd_soc_component {
    dev: *mut device,
}
#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
    dev: *mut device,
}
#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
struct spi_device {
    dev: device,
}
#[repr(C)]
struct i2c_client {
    dev: device,
}

static wm8985_reg_defaults: [reg_default; 57] = [
    reg_default { reg: 1, def: 0x0000 },     /* R1  - Power management 1 */
    reg_default { reg: 2, def: 0x0000 },     /* R2  - Power management 2 */
    reg_default { reg: 3, def: 0x0000 },     /* R3  - Power management 3 */
    reg_default { reg: 4, def: 0x0050 },     /* R4  - Audio Interface */
    reg_default { reg: 5, def: 0x0000 },     /* R5  - Companding control */
    reg_default { reg: 6, def: 0x0140 },     /* R6  - Clock Gen control */
    reg_default { reg: 7, def: 0x0000 },     /* R7  - Additional control */
    reg_default { reg: 8, def: 0x0000 },     /* R8  - GPIO Control */
    reg_default { reg: 9, def: 0x0000 },     /* R9  - Jack Detect Control 1 */
    reg_default { reg: 10, def: 0x0000 },    /* R10 - DAC Control */
    reg_default { reg: 11, def: 0x00ff },    /* R11 - Left DAC digital Vol */
    reg_default { reg: 12, def: 0x00ff },    /* R12 - Right DAC digital vol */
    reg_default { reg: 13, def: 0x0000 },    /* R13 - Jack Detect Control 2 */
    reg_default { reg: 14, def: 0x0100 },    /* R14 - ADC Control */
    reg_default { reg: 15, def: 0x00ff },    /* R15 - Left ADC Digital Vol */
    reg_default { reg: 16, def: 0x00ff },    /* R16 - Right ADC Digital Vol */
    reg_default { reg: 18, def: 0x012c },    /* R18 - EQ1 - low shelf */
    reg_default { reg: 19, def: 0x002c },    /* R19 - EQ2 - peak 1 */
    reg_default { reg: 20, def: 0x002c },    /* R20 - EQ3 - peak 2 */
    reg_default { reg: 21, def: 0x002c },    /* R21 - EQ4 - peak 3 */
    reg_default { reg: 22, def: 0x002c },    /* R22 - EQ5 - high shelf */
    reg_default { reg: 24, def: 0x0032 },    /* R24 - DAC Limiter 1 */
    reg_default { reg: 25, def: 0x0000 },    /* R25 - DAC Limiter 2 */
    reg_default { reg: 27, def: 0x0000 },    /* R27 - Notch Filter 1 */
    reg_default { reg: 28, def: 0x0000 },    /* R28 - Notch Filter 2 */
    reg_default { reg: 29, def: 0x0000 },    /* R29 - Notch Filter 3 */
    reg_default { reg: 30, def: 0x0000 },    /* R30 - Notch Filter 4 */
    reg_default { reg: 32, def: 0x0038 },    /* R32 - ALC control 1 */
    reg_default { reg: 33, def: 0x000b },    /* R33 - ALC control 2 */
    reg_default { reg: 34, def: 0x0032 },    /* R34 - ALC control 3 */
    reg_default { reg: 35, def: 0x0000 },    /* R35 - Noise Gate */
    reg_default { reg: 36, def: 0x0008 },    /* R36 - PLL N */
    reg_default { reg: 37, def: 0x000c },    /* R37 - PLL K 1 */
    reg_default { reg: 38, def: 0x0093 },    /* R38 - PLL K 2 */
    reg_default { reg: 39, def: 0x00e9 },    /* R39 - PLL K 3 */
    reg_default { reg: 41, def: 0x0000 },    /* R41 - 3D control */
    reg_default { reg: 42, def: 0x0000 },    /* R42 - OUT4 to ADC */
    reg_default { reg: 43, def: 0x0000 },    /* R43 - Beep control */
    reg_default { reg: 44, def: 0x0033 },    /* R44 - Input ctrl */
    reg_default { reg: 45, def: 0x0010 },    /* R45 - Left INP PGA gain ctrl */
    reg_default { reg: 46, def: 0x0010 },    /* R46 - Right INP PGA gain ctrl */
    reg_default { reg: 47, def: 0x0100 },    /* R47 - Left ADC BOOST ctrl */
    reg_default { reg: 48, def: 0x0100 },    /* R48 - Right ADC BOOST ctrl */
    reg_default { reg: 49, def: 0x0002 },    /* R49 - Output ctrl */
    reg_default { reg: 50, def: 0x0001 },    /* R50 - Left mixer ctrl */
    reg_default { reg: 51, def: 0x0001 },    /* R51 - Right mixer ctrl */
    reg_default { reg: 52, def: 0x0039 },    /* R52 - LOUT1 (HP) volume ctrl */
    reg_default { reg: 53, def: 0x0039 },    /* R53 - ROUT1 (HP) volume ctrl */
    reg_default { reg: 54, def: 0x0039 },    /* R54 - LOUT2 (SPK) volume ctrl */
    reg_default { reg: 55, def: 0x0039 },    /* R55 - ROUT2 (SPK) volume ctrl */
    reg_default { reg: 56, def: 0x0001 },    /* R56 - OUT3 mixer ctrl */
    reg_default { reg: 57, def: 0x0001 },    /* R57 - OUT4 (MONO) mix ctrl */
    reg_default { reg: 60, def: 0x0004 },    /* R60 - OUTPUT ctrl */
    reg_default { reg: 61, def: 0x0000 },    /* R61 - BIAS CTRL */
];

unsafe extern "C" fn wm8985_writeable(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        WM8985_SOFTWARE_RESET | WM8985_POWER_MANAGEMENT_1 | WM8985_POWER_MANAGEMENT_2 |
        WM8985_POWER_MANAGEMENT_3 | WM8985_AUDIO_INTERFACE | WM8985_COMPANDING_CONTROL |
        WM8985_CLOCK_GEN_CONTROL | WM8985_ADDITIONAL_CONTROL | WM8985_GPIO_CONTROL |
        WM8985_JACK_DETECT_CONTROL_1 | WM8985_DAC_CONTROL | WM8985_LEFT_DAC_DIGITAL_VOL |
        WM8985_RIGHT_DAC_DIGITAL_VOL | WM8985_JACK_DETECT_CONTROL_2 | WM8985_ADC_CONTROL |
        WM8985_LEFT_ADC_DIGITAL_VOL | WM8985_RIGHT_ADC_DIGITAL_VOL | WM8985_EQ1_LOW_SHELF |
        WM8985_EQ2_PEAK_1 | WM8985_EQ3_PEAK_2 | WM8985_EQ4_PEAK_3 | WM8985_EQ5_HIGH_SHELF |
        WM8985_DAC_LIMITER_1 | WM8985_DAC_LIMITER_2 | WM8985_NOTCH_FILTER_1 |
        WM8985_NOTCH_FILTER_2 | WM8985_NOTCH_FILTER_3 | WM8985_NOTCH_FILTER_4 |
        WM8985_ALC_CONTROL_1 | WM8985_ALC_CONTROL_2 | WM8985_ALC_CONTROL_3 |
        WM8985_NOISE_GATE | WM8985_PLL_N | WM8985_PLL_K_1 | WM8985_PLL_K_2 |
        WM8985_PLL_K_3 | WM8985_3D_CONTROL | WM8985_OUT4_TO_ADC | WM8985_BEEP_CONTROL |
        WM8985_INPUT_CTRL | WM8985_LEFT_INP_PGA_GAIN_CTRL | WM8985_RIGHT_INP_PGA_GAIN_CTRL |
        WM8985_LEFT_ADC_BOOST_CTRL | WM8985_RIGHT_ADC_BOOST_CTRL | WM8985_OUTPUT_CTRL0 |
        WM8985_LEFT_MIXER_CTRL | WM8985_RIGHT_MIXER_CTRL | WM8985_LOUT1_HP_VOLUME_CTRL |
        WM8985_ROUT1_HP_VOLUME_CTRL | WM8985_LOUT2_SPK_VOLUME_CTRL |
        WM8985_ROUT2_SPK_VOLUME_CTRL | WM8985_OUT3_MIXER_CTRL | WM8985_OUT4_MONO_MIX_CTRL |
        WM8985_OUTPUT_CTRL1 | WM8985_BIAS_CTRL => true,
        _ => false,
    }
}

/*
 * latch bit 8 of these registers to ensure instant
 * volume updates
 */
static volume_update_regs: [c_int; 10] = [
    WM8985_LEFT_DAC_DIGITAL_VOL as c_int,
    WM8985_RIGHT_DAC_DIGITAL_VOL as c_int,
    WM8985_LEFT_ADC_DIGITAL_VOL as c_int,
    WM8985_RIGHT_ADC_DIGITAL_VOL as c_int,
    WM8985_LOUT2_SPK_VOLUME_CTRL as c_int,
    WM8985_ROUT2_SPK_VOLUME_CTRL as c_int,
    WM8985_LOUT1_HP_VOLUME_CTRL as c_int,
    WM8985_ROUT1_HP_VOLUME_CTRL as c_int,
    WM8985_LEFT_INP_PGA_GAIN_CTRL as c_int,
    WM8985_RIGHT_INP_PGA_GAIN_CTRL as c_int,
];

#[repr(C)]
struct wm8985_priv {
    regmap: *mut regmap,
    supplies: [regulator_bulk_data; WM8985_NUM_SUPPLIES],
    dev_type: wm8985_type,
    sysclk: c_uint,
    bclk: c_uint,
}

#[repr(C)]
struct fs_ratio {
    div: c_int,
    ratio: c_int,
}

static fs_ratios: [fs_ratio; 8] = [
    fs_ratio { div: 10, ratio: 128 },
    fs_ratio { div: 15, ratio: 192 },
    fs_ratio { div: 20, ratio: 256 },
    fs_ratio { div: 30, ratio: 384 },
    fs_ratio { div: 40, ratio: 512 },
    fs_ratio { div: 60, ratio: 768 },
    fs_ratio { div: 80, ratio: 1024 },
    fs_ratio { div: 120, ratio: 1536 },
];

static srates: [c_int; 6] = [48000, 32000, 24000, 16000, 12000, 8000];
static bclk_divs: [c_int; 6] = [1, 2, 4, 8, 16, 32];

static_tlv_db_scale!(dac_tlv, -12700, 50, 1);
static_tlv_db_scale!(adc_tlv, -12700, 50, 1);
static_tlv_db_scale!(out_tlv, -5700, 100, 0);
static_tlv_db_scale!(lim_thresh_tlv, -600, 100, 0);
static_tlv_db_scale!(lim_boost_tlv, 0, 100, 0);
static_tlv_db_scale!(alc_min_tlv, -1200, 600, 0);
static_tlv_db_scale!(alc_max_tlv, -675, 600, 0);
static_tlv_db_scale!(alc_tar_tlv, -2250, 150, 0);
static_tlv_db_scale!(pga_vol_tlv, -1200, 75, 0);
static_tlv_db_scale!(boost_tlv, -1200, 300, 1);
static_tlv_db_scale!(eq_tlv, -1200, 100, 0);
static_tlv_db_scale!(aux_tlv, -1500, 300, 0);
static_tlv_db_scale!(bypass_tlv, -1500, 300, 0);
static_tlv_db_scale!(pga_boost_tlv, 0, 2000, 0);

static alc_sel_text: [*const c_char; 4] = [cstr!("Off"), cstr!("Right"), cstr!("Left"), cstr!("Stereo")];
soc_enum_single_decl!(alc_sel, WM8985_ALC_CONTROL_1, 7, alc_sel_text);
static alc_mode_text: [*const c_char; 2] = [cstr!("ALC"), cstr!("Limiter")];
soc_enum_single_decl!(alc_mode, WM8985_ALC_CONTROL_3, 8, alc_mode_text);
static filter_mode_text: [*const c_char; 2] = [cstr!("Audio"), cstr!("Application")];
soc_enum_single_decl!(filter_mode, WM8985_ADC_CONTROL, 7, filter_mode_text);
static eq_bw_text: [*const c_char; 2] = [cstr!("Narrow"), cstr!("Wide")];
static eqmode_text: [*const c_char; 2] = [cstr!("Capture"), cstr!("Playback")];
soc_enum_single_ext_decl!(eqmode, eqmode_text);
static eq1_cutoff_text: [*const c_char; 4] = [cstr!("80Hz"), cstr!("105Hz"), cstr!("135Hz"), cstr!("175Hz")];
soc_enum_single_decl!(eq1_cutoff, WM8985_EQ1_LOW_SHELF, 5, eq1_cutoff_text);
static eq2_cutoff_text: [*const c_char; 4] = [cstr!("230Hz"), cstr!("300Hz"), cstr!("385Hz"), cstr!("500Hz")];
soc_enum_single_decl!(eq2_bw, WM8985_EQ2_PEAK_1, 8, eq_bw_text);
soc_enum_single_decl!(eq2_cutoff, WM8985_EQ2_PEAK_1, 5, eq2_cutoff_text);
static eq3_cutoff_text: [*const c_char; 4] = [cstr!("650Hz"), cstr!("850Hz"), cstr!("1.1kHz"), cstr!("1.4kHz")];
soc_enum_single_decl!(eq3_bw, WM8985_EQ3_PEAK_2, 8, eq_bw_text);
soc_enum_single_decl!(eq3_cutoff, WM8985_EQ3_PEAK_2, 5, eq3_cutoff_text);
static eq4_cutoff_text: [*const c_char; 4] = [cstr!("1.8kHz"), cstr!("2.4kHz"), cstr!("3.2kHz"), cstr!("4.1kHz")];
soc_enum_single_decl!(eq4_bw, WM8985_EQ4_PEAK_3, 8, eq_bw_text);
soc_enum_single_decl!(eq4_cutoff, WM8985_EQ4_PEAK_3, 5, eq4_cutoff_text);
static eq5_cutoff_text: [*const c_char; 4] = [cstr!("5.3kHz"), cstr!("6.9kHz"), cstr!("9kHz"), cstr!("11.7kHz")];
soc_enum_single_decl!(eq5_cutoff, WM8985_EQ5_HIGH_SHELF, 5, eq5_cutoff_text);
static speaker_mode_text: [*const c_char; 2] = [cstr!("Class A/B"), cstr!("Class D")];
soc_enum_single_decl!(speaker_mode, 0x17, 8, speaker_mode_text);
static depth_3d_text: [*const c_char; 16] = [
    cstr!("Off"), cstr!("6.67%"), cstr!("13.3%"), cstr!("20%"),
    cstr!("26.7%"), cstr!("33.3%"), cstr!("40%"), cstr!("46.6%"),
    cstr!("53.3%"), cstr!("60%"), cstr!("66.7%"), cstr!("73.3%"),
    cstr!("80%"), cstr!("86.7%"), cstr!("93.3%"), cstr!("100%"),
];
soc_enum_single_decl!(depth_3d, WM8985_3D_CONTROL, 0, depth_3d_text);

static wm8985_common_snd_controls: [snd_kcontrol_new; 49] = snd_controls![
    SOC_SINGLE("Digital Loopback Switch", WM8985_COMPANDING_CONTROL, 0, 1, 0),
    SOC_ENUM("ALC Capture Function", alc_sel),
    SOC_SINGLE_TLV("ALC Capture Max Volume", WM8985_ALC_CONTROL_1, 3, 7, 0, alc_max_tlv),
    SOC_SINGLE_TLV("ALC Capture Min Volume", WM8985_ALC_CONTROL_1, 0, 7, 0, alc_min_tlv),
    SOC_SINGLE_TLV("ALC Capture Target Volume", WM8985_ALC_CONTROL_2, 0, 15, 0, alc_tar_tlv),
    SOC_SINGLE("ALC Capture Attack", WM8985_ALC_CONTROL_3, 0, 10, 0),
    SOC_SINGLE("ALC Capture Hold", WM8985_ALC_CONTROL_2, 4, 10, 0),
    SOC_SINGLE("ALC Capture Decay", WM8985_ALC_CONTROL_3, 4, 10, 0),
    SOC_ENUM("ALC Mode", alc_mode),
    SOC_SINGLE("ALC Capture NG Switch", WM8985_NOISE_GATE, 3, 1, 0),
    SOC_SINGLE("ALC Capture NG Threshold", WM8985_NOISE_GATE, 0, 7, 1),
    SOC_DOUBLE_R_TLV("Capture Volume", WM8985_LEFT_ADC_DIGITAL_VOL, WM8985_RIGHT_ADC_DIGITAL_VOL, 0, 255, 0, adc_tlv),
    SOC_DOUBLE_R("Capture PGA ZC Switch", WM8985_LEFT_INP_PGA_GAIN_CTRL, WM8985_RIGHT_INP_PGA_GAIN_CTRL, 7, 1, 0),
    SOC_DOUBLE_R_TLV("Capture PGA Volume", WM8985_LEFT_INP_PGA_GAIN_CTRL, WM8985_RIGHT_INP_PGA_GAIN_CTRL, 0, 63, 0, pga_vol_tlv),
    SOC_DOUBLE_R_TLV("Capture PGA Boost Volume", WM8985_LEFT_ADC_BOOST_CTRL, WM8985_RIGHT_ADC_BOOST_CTRL, 8, 1, 0, pga_boost_tlv),
    SOC_DOUBLE("ADC Inversion Switch", WM8985_ADC_CONTROL, 0, 1, 1, 0),
    SOC_SINGLE("ADC 128x Oversampling Switch", WM8985_ADC_CONTROL, 8, 1, 0),
    SOC_DOUBLE_R_TLV("Playback Volume", WM8985_LEFT_DAC_DIGITAL_VOL, WM8985_RIGHT_DAC_DIGITAL_VOL, 0, 255, 0, dac_tlv),
    SOC_SINGLE("DAC Playback Limiter Switch", WM8985_DAC_LIMITER_1, 8, 1, 0),
    SOC_SINGLE("DAC Playback Limiter Decay", WM8985_DAC_LIMITER_1, 4, 10, 0),
    SOC_SINGLE("DAC Playback Limiter Attack", WM8985_DAC_LIMITER_1, 0, 11, 0),
    SOC_SINGLE_TLV("DAC Playback Limiter Threshold", WM8985_DAC_LIMITER_2, 4, 7, 1, lim_thresh_tlv),
    SOC_SINGLE_TLV("DAC Playback Limiter Boost Volume", WM8985_DAC_LIMITER_2, 0, 12, 0, lim_boost_tlv),
    SOC_DOUBLE("DAC Inversion Switch", WM8985_DAC_CONTROL, 0, 1, 1, 0),
    SOC_SINGLE("DAC Auto Mute Switch", WM8985_DAC_CONTROL, 2, 1, 0),
    SOC_SINGLE("DAC 128x Oversampling Switch", WM8985_DAC_CONTROL, 3, 1, 0),
    SOC_DOUBLE_R_TLV("Headphone Playback Volume", WM8985_LOUT1_HP_VOLUME_CTRL, WM8985_ROUT1_HP_VOLUME_CTRL, 0, 63, 0, out_tlv),
    SOC_DOUBLE_R("Headphone Playback ZC Switch", WM8985_LOUT1_HP_VOLUME_CTRL, WM8985_ROUT1_HP_VOLUME_CTRL, 7, 1, 0),
    SOC_DOUBLE_R("Headphone Switch", WM8985_LOUT1_HP_VOLUME_CTRL, WM8985_ROUT1_HP_VOLUME_CTRL, 6, 1, 1),
    SOC_DOUBLE_R_TLV("Speaker Playback Volume", WM8985_LOUT2_SPK_VOLUME_CTRL, WM8985_ROUT2_SPK_VOLUME_CTRL, 0, 63, 0, out_tlv),
    SOC_DOUBLE_R("Speaker Playback ZC Switch", WM8985_LOUT2_SPK_VOLUME_CTRL, WM8985_ROUT2_SPK_VOLUME_CTRL, 7, 1, 0),
    SOC_DOUBLE_R("Speaker Switch", WM8985_LOUT2_SPK_VOLUME_CTRL, WM8985_ROUT2_SPK_VOLUME_CTRL, 6, 1, 1),
    SOC_SINGLE("High Pass Filter Switch", WM8985_ADC_CONTROL, 8, 1, 0),
    SOC_ENUM("High Pass Filter Mode", filter_mode),
    SOC_SINGLE("High Pass Filter Cutoff", WM8985_ADC_CONTROL, 4, 7, 0),
    SOC_DOUBLE_R_TLV("Input PGA Bypass Volume", WM8985_LEFT_MIXER_CTRL, WM8985_RIGHT_MIXER_CTRL, 2, 7, 0, bypass_tlv),
    SOC_ENUM_EXT("Equalizer Function", eqmode, eqmode_get, eqmode_put),
    SOC_ENUM("EQ1 Cutoff", eq1_cutoff),
    SOC_SINGLE_TLV("EQ1 Volume", WM8985_EQ1_LOW_SHELF, 0, 24, 1, eq_tlv),
    SOC_ENUM("EQ2 Bandwidth", eq2_bw),
    SOC_ENUM("EQ2 Cutoff", eq2_cutoff),
    SOC_SINGLE_TLV("EQ2 Volume", WM8985_EQ2_PEAK_1, 0, 24, 1, eq_tlv),
    SOC_ENUM("EQ3 Bandwidth", eq3_bw),
    SOC_ENUM("EQ3 Cutoff", eq3_cutoff),
    SOC_SINGLE_TLV("EQ3 Volume", WM8985_EQ3_PEAK_2, 0, 24, 1, eq_tlv),
    SOC_ENUM("EQ4 Bandwidth", eq4_bw),
    SOC_ENUM("EQ4 Cutoff", eq4_cutoff),
    SOC_SINGLE_TLV("EQ4 Volume", WM8985_EQ4_PEAK_3, 0, 24, 1, eq_tlv),
    SOC_ENUM("EQ5 Cutoff", eq5_cutoff),
    SOC_SINGLE_TLV("EQ5 Volume", WM8985_EQ5_HIGH_SHELF, 0, 24, 1, eq_tlv),
    SOC_ENUM("3D Depth", depth_3d),
];

static wm8985_specific_snd_controls: [snd_kcontrol_new; 2] = snd_controls![
    SOC_DOUBLE_R_TLV("Aux Bypass Volume", WM8985_LEFT_MIXER_CTRL, WM8985_RIGHT_MIXER_CTRL, 6, 7, 0, aux_tlv),
    SOC_ENUM("Speaker Mode", speaker_mode),
];

static left_out_mixer: [snd_kcontrol_new; 3] = snd_controls![
    SOC_DAPM_SINGLE("Line Switch", WM8985_LEFT_MIXER_CTRL, 1, 1, 0),
    SOC_DAPM_SINGLE("PCM Switch", WM8985_LEFT_MIXER_CTRL, 0, 1, 0),
    /* --- WM8985 only --- */
    SOC_DAPM_SINGLE("Aux Switch", WM8985_LEFT_MIXER_CTRL, 5, 1, 0),
];
static right_out_mixer: [snd_kcontrol_new; 3] = snd_controls![
    SOC_DAPM_SINGLE("Line Switch", WM8985_RIGHT_MIXER_CTRL, 1, 1, 0),
    SOC_DAPM_SINGLE("PCM Switch", WM8985_RIGHT_MIXER_CTRL, 0, 1, 0),
    /* --- WM8985 only --- */
    SOC_DAPM_SINGLE("Aux Switch", WM8985_RIGHT_MIXER_CTRL, 5, 1, 0),
];
static left_input_mixer: [snd_kcontrol_new; 3] = snd_controls![
    SOC_DAPM_SINGLE("L2 Switch", WM8985_INPUT_CTRL, 2, 1, 0),
    SOC_DAPM_SINGLE("MicN Switch", WM8985_INPUT_CTRL, 1, 1, 0),
    SOC_DAPM_SINGLE("MicP Switch", WM8985_INPUT_CTRL, 0, 1, 0),
];
static right_input_mixer: [snd_kcontrol_new; 3] = snd_controls![
    SOC_DAPM_SINGLE("R2 Switch", WM8985_INPUT_CTRL, 6, 1, 0),
    SOC_DAPM_SINGLE("MicN Switch", WM8985_INPUT_CTRL, 5, 1, 0),
    SOC_DAPM_SINGLE("MicP Switch", WM8985_INPUT_CTRL, 4, 1, 0),
];
static left_boost_mixer: [snd_kcontrol_new; 2] = snd_controls![
    SOC_DAPM_SINGLE_TLV("L2 Volume", WM8985_LEFT_ADC_BOOST_CTRL, 4, 7, 0, boost_tlv),
    /* --- WM8985 only --- */
    SOC_DAPM_SINGLE_TLV("AUXL Volume", WM8985_LEFT_ADC_BOOST_CTRL, 0, 7, 0, boost_tlv),
];
static right_boost_mixer: [snd_kcontrol_new; 2] = snd_controls![
    SOC_DAPM_SINGLE_TLV("R2 Volume", WM8985_RIGHT_ADC_BOOST_CTRL, 4, 7, 0, boost_tlv),
    /* --- WM8985 only --- */
    SOC_DAPM_SINGLE_TLV("AUXR Volume", WM8985_RIGHT_ADC_BOOST_CTRL, 0, 7, 0, boost_tlv),
];

static wm8985_common_dapm_widgets: [snd_soc_dapm_widget; 20] = dapm_widgets![
    SND_SOC_DAPM_DAC("Left DAC", "Left Playback", WM8985_POWER_MANAGEMENT_3, 0, 0),
    SND_SOC_DAPM_DAC("Right DAC", "Right Playback", WM8985_POWER_MANAGEMENT_3, 1, 0),
    SND_SOC_DAPM_ADC("Left ADC", "Left Capture", WM8985_POWER_MANAGEMENT_2, 0, 0),
    SND_SOC_DAPM_ADC("Right ADC", "Right Capture", WM8985_POWER_MANAGEMENT_2, 1, 0),
    SND_SOC_DAPM_MIXER("Left Input Mixer", WM8985_POWER_MANAGEMENT_2, 2, 0, left_input_mixer, left_input_mixer.len()),
    SND_SOC_DAPM_MIXER("Right Input Mixer", WM8985_POWER_MANAGEMENT_2, 3, 0, right_input_mixer, right_input_mixer.len()),
    SND_SOC_DAPM_PGA("Left Capture PGA", WM8985_LEFT_INP_PGA_GAIN_CTRL, 6, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA("Right Capture PGA", WM8985_RIGHT_INP_PGA_GAIN_CTRL, 6, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA("Left Headphone Out", WM8985_POWER_MANAGEMENT_2, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA("Right Headphone Out", WM8985_POWER_MANAGEMENT_2, 8, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA("Left Speaker Out", WM8985_POWER_MANAGEMENT_3, 5, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA("Right Speaker Out", WM8985_POWER_MANAGEMENT_3, 6, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY("Mic Bias", WM8985_POWER_MANAGEMENT_1, 4, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_INPUT("LIN"), SND_SOC_DAPM_INPUT("LIP"), SND_SOC_DAPM_INPUT("RIN"),
    SND_SOC_DAPM_INPUT("RIP"), SND_SOC_DAPM_INPUT("L2"), SND_SOC_DAPM_INPUT("R2"),
    SND_SOC_DAPM_OUTPUT("HPL"), SND_SOC_DAPM_OUTPUT("HPR"), SND_SOC_DAPM_OUTPUT("SPKL"),
    SND_SOC_DAPM_OUTPUT("SPKR"),
];

static wm8985_dapm_widgets: [snd_soc_dapm_widget; 6] = dapm_widgets![
    SND_SOC_DAPM_MIXER("Left Output Mixer", WM8985_POWER_MANAGEMENT_3, 2, 0, left_out_mixer, left_out_mixer.len()),
    SND_SOC_DAPM_MIXER("Right Output Mixer", WM8985_POWER_MANAGEMENT_3, 3, 0, right_out_mixer, right_out_mixer.len()),
    SND_SOC_DAPM_MIXER("Left Boost Mixer", WM8985_POWER_MANAGEMENT_2, 4, 0, left_boost_mixer, left_boost_mixer.len()),
    SND_SOC_DAPM_MIXER("Right Boost Mixer", WM8985_POWER_MANAGEMENT_2, 5, 0, right_boost_mixer, right_boost_mixer.len()),
    SND_SOC_DAPM_INPUT("AUXL"), SND_SOC_DAPM_INPUT("AUXR"),
];
static wm8758_dapm_widgets: [snd_soc_dapm_widget; 4] = dapm_widgets![
    SND_SOC_DAPM_MIXER("Left Output Mixer", WM8985_POWER_MANAGEMENT_3, 2, 0, left_out_mixer, left_out_mixer.len() - 1),
    SND_SOC_DAPM_MIXER("Right Output Mixer", WM8985_POWER_MANAGEMENT_3, 3, 0, right_out_mixer, right_out_mixer.len() - 1),
    SND_SOC_DAPM_MIXER("Left Boost Mixer", WM8985_POWER_MANAGEMENT_2, 4, 0, left_boost_mixer, left_boost_mixer.len() - 1),
    SND_SOC_DAPM_MIXER("Right Boost Mixer", WM8985_POWER_MANAGEMENT_2, 5, 0, right_boost_mixer, right_boost_mixer.len() - 1),
];

static wm8985_common_dapm_routes: [snd_soc_dapm_route; 28] = dapm_routes![
    { "Right Output Mixer", "PCM Switch", "Right DAC" },
    { "Right Output Mixer", "Line Switch", "Right Boost Mixer" },
    { "Left Output Mixer", "PCM Switch", "Left DAC" },
    { "Left Output Mixer", "Line Switch", "Left Boost Mixer" },
    { "Right Headphone Out", NULL, "Right Output Mixer" },
    { "HPR", NULL, "Right Headphone Out" },
    { "Left Headphone Out", NULL, "Left Output Mixer" },
    { "HPL", NULL, "Left Headphone Out" },
    { "Right Speaker Out", NULL, "Right Output Mixer" },
    { "SPKR", NULL, "Right Speaker Out" },
    { "Left Speaker Out", NULL, "Left Output Mixer" },
    { "SPKL", NULL, "Left Speaker Out" },
    { "Right ADC", NULL, "Right Boost Mixer" },
    { "Right Boost Mixer", NULL, "Right Capture PGA" },
    { "Right Boost Mixer", "R2 Volume", "R2" },
    { "Left ADC", NULL, "Left Boost Mixer" },
    { "Left Boost Mixer", NULL, "Left Capture PGA" },
    { "Left Boost Mixer", "L2 Volume", "L2" },
    { "Right Capture PGA", NULL, "Right Input Mixer" },
    { "Left Capture PGA", NULL, "Left Input Mixer" },
    { "Right Input Mixer", "R2 Switch", "R2" },
    { "Right Input Mixer", "MicN Switch", "RIN" },
    { "Right Input Mixer", "MicP Switch", "RIP" },
    { "Left Input Mixer", "L2 Switch", "L2" },
    { "Left Input Mixer", "MicN Switch", "LIN" },
    { "Left Input Mixer", "MicP Switch", "LIP" },
];
static wm8985_aux_dapm_routes: [snd_soc_dapm_route; 4] = dapm_routes![
    { "Right Output Mixer", "Aux Switch", "AUXR" },
    { "Left Output Mixer", "Aux Switch", "AUXL" },
    { "Right Boost Mixer", "AUXR Volume", "AUXR" },
    { "Left Boost Mixer", "AUXL Volume", "AUXL" },
];

unsafe extern "C" fn wm8985_add_widgets(component: *mut snd_soc_component) -> c_int {
    let wm8985 = snd_soc_component_get_drvdata(component) as *mut wm8985_priv;
    let dapm = snd_soc_component_to_dapm(component);
    match (*wm8985).dev_type {
        wm8985_type::WM8758 => {
            snd_soc_dapm_new_controls(dapm, wm8758_dapm_widgets.as_ptr(), wm8758_dapm_widgets.len() as c_uint);
        }
        wm8985_type::WM8985 => {
            snd_soc_add_component_controls(component, wm8985_specific_snd_controls.as_ptr(), wm8985_specific_snd_controls.len() as c_uint);
            snd_soc_dapm_new_controls(dapm, wm8985_dapm_widgets.as_ptr(), wm8985_dapm_widgets.len() as c_uint);
            snd_soc_dapm_add_routes(dapm, wm8985_aux_dapm_routes.as_ptr(), wm8985_aux_dapm_routes.len() as c_uint);
        }
    }
    0
}

unsafe extern "C" fn eqmode_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let reg = snd_soc_component_read(component, WM8985_EQ1_LOW_SHELF);
    if (reg & WM8985_EQ3DMODE) != 0 {
        (*ucontrol).value.enumerated.item[0] = 1;
    } else {
        (*ucontrol).value.enumerated.item[0] = 0;
    }
    0
}

unsafe extern "C" fn eqmode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let regpwr2: c_uint;
    let regpwr3: c_uint;
    let reg_eq: c_uint;
    let item0 = (*ucontrol).value.enumerated.item[0];

    if item0 != 0 && item0 != 1 {
        return -EINVAL;
    }

    reg_eq = snd_soc_component_read(component, WM8985_EQ1_LOW_SHELF);
    match (reg_eq & WM8985_EQ3DMODE) >> WM8985_EQ3DMODE_SHIFT {
        0 => {
            if item0 == 0 {
                return 0;
            }
        }
        1 => {
            if item0 != 0 {
                return 0;
            }
        }
        _ => {}
    }

    regpwr2 = snd_soc_component_read(component, WM8985_POWER_MANAGEMENT_2);
    regpwr3 = snd_soc_component_read(component, WM8985_POWER_MANAGEMENT_3);
    /* disable the DACs and ADCs */
    snd_soc_component_update_bits(component, WM8985_POWER_MANAGEMENT_2, WM8985_ADCENR_MASK | WM8985_ADCENL_MASK, 0);
    snd_soc_component_update_bits(component, WM8985_POWER_MANAGEMENT_3, WM8985_DACENR_MASK | WM8985_DACENL_MASK, 0);
    snd_soc_component_update_bits(component, WM8985_ADDITIONAL_CONTROL, WM8985_M128ENB_MASK, WM8985_M128ENB);
    /* set the desired eqmode */
    snd_soc_component_update_bits(component, WM8985_EQ1_LOW_SHELF, WM8985_EQ3DMODE_MASK, item0 << WM8985_EQ3DMODE_SHIFT);
    /* restore DAC/ADC configuration */
    snd_soc_component_write(component, WM8985_POWER_MANAGEMENT_2, regpwr2);
    snd_soc_component_write(component, WM8985_POWER_MANAGEMENT_3, regpwr3);
    0
}

unsafe extern "C" fn wm8985_reset(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_write(component, WM8985_SOFTWARE_RESET, 0x0)
}

unsafe extern "C" fn wm8985_dac_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    snd_soc_component_update_bits(component, WM8985_DAC_CONTROL, WM8985_SOFTMUTE_MASK, ((mute != 0) as c_uint) << WM8985_SOFTMUTE_SHIFT)
}

unsafe extern "C" fn wm8985_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
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
            dev_err((*dai).dev, cstr!("Unknown dai format\n"));
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, WM8985_AUDIO_INTERFACE, WM8985_FMT_MASK, (format as c_uint) << WM8985_FMT_SHIFT);

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => master = 1,
        SND_SOC_DAIFMT_CBC_CFC => master = 0,
        _ => {
            dev_err((*dai).dev, cstr!("Unknown master/slave configuration\n"));
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, WM8985_CLOCK_GEN_CONTROL, WM8985_MS_MASK, (master as c_uint) << WM8985_MS_SHIFT);

    /* frame inversion is not valid for dsp modes */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => {
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                SND_SOC_DAIFMT_IB_IF | SND_SOC_DAIFMT_NB_IF => return -EINVAL,
                _ => {}
            }
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
            dev_err((*dai).dev, cstr!("Unknown polarity configuration\n"));
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, WM8985_AUDIO_INTERFACE, WM8985_LRP_MASK, (lrp as c_uint) << WM8985_LRP_SHIFT);
    snd_soc_component_update_bits(component, WM8985_AUDIO_INTERFACE, WM8985_BCP_MASK, (bcp as c_uint) << WM8985_BCP_SHIFT);
    0
}

unsafe extern "C" fn wm8985_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut i: usize;
    let component = (*dai).component;
    let wm8985 = snd_soc_component_get_drvdata(component) as *mut wm8985_priv;
    let blen: u16;
    let mut srate_idx: u16;
    let mut tmp: c_uint;
    let mut srate_best: c_int;

    (*wm8985).bclk = snd_soc_params_to_bclk(params);
    if ((*wm8985).bclk as c_int) < 0 {
        return (*wm8985).bclk as c_int;
    }

    match params_width(params) {
        16 => blen = 0x0,
        20 => blen = 0x1,
        24 => blen = 0x2,
        32 => blen = 0x3,
        _ => {
            dev_err((*dai).dev, cstr!("Unsupported word length %u\n"), params_width(params));
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, WM8985_AUDIO_INTERFACE, WM8985_WL_MASK, (blen as c_uint) << WM8985_WL_SHIFT);

    /*
     * match to the nearest possible sample rate and rely
     * on the array index to configure the SR register
     */
    srate_idx = 0;
    srate_best = (srates[0] - params_rate(params)).abs();
    i = 1;
    while i < srates.len() {
        if (srates[i] - params_rate(params)).abs() < srate_best {
            srate_idx = i as u16;
            srate_best = (srates[i] - params_rate(params)).abs();
        }
        i += 1;
    }

    dev_dbg((*dai).dev, cstr!("Selected SRATE = %d\n"), srates[srate_idx as usize]);
    snd_soc_component_update_bits(component, WM8985_ADDITIONAL_CONTROL, WM8985_SR_MASK, (srate_idx as c_uint) << WM8985_SR_SHIFT);
    dev_dbg((*dai).dev, cstr!("Target BCLK = %uHz\n"), (*wm8985).bclk);
    dev_dbg((*dai).dev, cstr!("SYSCLK = %uHz\n"), (*wm8985).sysclk);

    i = 0;
    while i < fs_ratios.len() {
        if (*wm8985).sysclk / (params_rate(params) as c_uint) == fs_ratios[i].ratio as c_uint {
            break;
        }
        i += 1;
    }
    if i == fs_ratios.len() {
        dev_err((*dai).dev, cstr!("Unable to configure MCLK ratio %u/%u\n"), (*wm8985).sysclk, params_rate(params));
        return -EINVAL;
    }

    dev_dbg((*dai).dev, cstr!("MCLK ratio = %dfs\n"), fs_ratios[i].ratio);
    snd_soc_component_update_bits(component, WM8985_CLOCK_GEN_CONTROL, WM8985_MCLKDIV_MASK, (i as c_uint) << WM8985_MCLKDIV_SHIFT);

    /* select the appropriate bclk divider */
    tmp = ((*wm8985).sysclk / fs_ratios[i].div as c_uint) * 10;
    i = 0;
    while i < bclk_divs.len() {
        if (*wm8985).bclk == tmp / bclk_divs[i] as c_uint {
            break;
        }
        i += 1;
    }
    if i == bclk_divs.len() {
        dev_err((*dai).dev, cstr!("No matching BCLK divider found\n"));
        return -EINVAL;
    }

    dev_dbg((*dai).dev, cstr!("BCLK div = %d\n"), i as c_int);
    snd_soc_component_update_bits(component, WM8985_CLOCK_GEN_CONTROL, WM8985_BCLKDIV_MASK, (i as c_uint) << WM8985_BCLKDIV_SHIFT);
    0
}

#[repr(C)]
struct pll_div {
    div2: u32,
    n: u32,
    k: u32,
}

const FIXED_PLL_SIZE: u64 = (1u64 << 24) * 10;
unsafe extern "C" fn pll_factors(pll_div: *mut pll_div, target: c_uint, mut source: c_uint) -> c_int {
    let mut Kpart: u64;
    let mut K: c_ulong;
    let mut Ndiv: c_ulong;
    let Nmod: c_ulong;

    (*pll_div).div2 = 0;
    Ndiv = (target / source) as c_ulong;
    if Ndiv < 6 {
        source >>= 1;
        (*pll_div).div2 = 1;
        Ndiv = (target / source) as c_ulong;
    }

    if Ndiv < 6 || Ndiv > 12 {
        printk(KERN_ERR, cstr!("%s: WM8985 N value is not within the recommended range: %lu\n"), __func__, Ndiv);
        return -EINVAL;
    }
    (*pll_div).n = Ndiv as u32;

    Nmod = (target % source) as c_ulong;
    Kpart = FIXED_PLL_SIZE * Nmod as u64;
    do_div(&mut Kpart, source);

    K = (Kpart & 0xffffffff) as c_ulong;
    if (K % 10) >= 5 {
        K += 5;
    }
    K /= 10;
    (*pll_div).k = K as u32;
    0
}

unsafe extern "C" fn wm8985_set_pll(
    dai: *mut snd_soc_dai,
    _pll_id: c_int,
    _source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let mut ret: c_int;
    let component = (*dai).component;
    let mut pll_div = core::mem::MaybeUninit::<pll_div>::uninit();

    if freq_in == 0 || freq_out == 0 {
        /* disable the PLL */
        snd_soc_component_update_bits(component, WM8985_POWER_MANAGEMENT_1, WM8985_PLLEN_MASK, 0);
    } else {
        ret = pll_factors(pll_div.as_mut_ptr(), freq_out * 4 * 2, freq_in);
        if ret != 0 {
            return ret;
        }
        let pll_div = pll_div.assume_init();
        /* set PLLN and PRESCALE */
        snd_soc_component_write(component, WM8985_PLL_N, (pll_div.div2 << WM8985_PLL_PRESCALE_SHIFT) | pll_div.n);
        /* set PLLK */
        snd_soc_component_write(component, WM8985_PLL_K_3, pll_div.k & 0x1ff);
        snd_soc_component_write(component, WM8985_PLL_K_2, (pll_div.k >> 9) & 0x1ff);
        snd_soc_component_write(component, WM8985_PLL_K_1, pll_div.k >> 18);
        /* set the source of the clock to be the PLL */
        snd_soc_component_update_bits(component, WM8985_CLOCK_GEN_CONTROL, WM8985_CLKSEL_MASK, WM8985_CLKSEL);
        /* enable the PLL */
        snd_soc_component_update_bits(component, WM8985_POWER_MANAGEMENT_1, WM8985_PLLEN_MASK, WM8985_PLLEN);
    }
    0
}

unsafe extern "C" fn wm8985_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let wm8985 = snd_soc_component_get_drvdata(component) as *mut wm8985_priv;

    match clk_id {
        WM8985_CLKSRC_MCLK => {
            snd_soc_component_update_bits(component, WM8985_CLOCK_GEN_CONTROL, WM8985_CLKSEL_MASK, 0);
            snd_soc_component_update_bits(component, WM8985_POWER_MANAGEMENT_1, WM8985_PLLEN_MASK, 0);
        }
        WM8985_CLKSRC_PLL => {
            snd_soc_component_update_bits(component, WM8985_CLOCK_GEN_CONTROL, WM8985_CLKSEL_MASK, WM8985_CLKSEL);
        }
        _ => {
            dev_err((*dai).dev, cstr!("Unknown clock source %d\n"), clk_id);
            return -EINVAL;
        }
    }
    (*wm8985).sysclk = freq;
    0
}

unsafe extern "C" fn wm8985_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let mut ret: c_int;
    let dapm = snd_soc_component_to_dapm(component);
    let wm8985 = snd_soc_component_get_drvdata(component) as *mut wm8985_priv;

    match level {
        SND_SOC_BIAS_ON | SND_SOC_BIAS_PREPARE => {
            /* VMID at 75k */
            snd_soc_component_update_bits(component, WM8985_POWER_MANAGEMENT_1, WM8985_VMIDSEL_MASK, 1 << WM8985_VMIDSEL_SHIFT);
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                ret = regulator_bulk_enable((*wm8985).supplies.len() as c_int, (*wm8985).supplies.as_mut_ptr());
                if ret != 0 {
                    dev_err((*component).dev, cstr!("Failed to enable supplies: %d\n"), ret);
                    return ret;
                }
                regcache_sync((*wm8985).regmap);
                /* enable anti-pop features */
                snd_soc_component_update_bits(component, WM8985_OUT4_TO_ADC, WM8985_POBCTRL_MASK, WM8985_POBCTRL);
                /* enable thermal shutdown */
                snd_soc_component_update_bits(component, WM8985_OUTPUT_CTRL0, WM8985_TSDEN_MASK, WM8985_TSDEN);
                snd_soc_component_update_bits(component, WM8985_OUTPUT_CTRL0, WM8985_TSOPCTRL_MASK, WM8985_TSOPCTRL);
                /* enable BIASEN */
                snd_soc_component_update_bits(component, WM8985_POWER_MANAGEMENT_1, WM8985_BIASEN_MASK, WM8985_BIASEN);
                /* VMID at 75k */
                snd_soc_component_update_bits(component, WM8985_POWER_MANAGEMENT_1, WM8985_VMIDSEL_MASK, 1 << WM8985_VMIDSEL_SHIFT);
                msleep(500);
                /* disable anti-pop features */
                snd_soc_component_update_bits(component, WM8985_OUT4_TO_ADC, WM8985_POBCTRL_MASK, 0);
            }
            /* VMID at 300k */
            snd_soc_component_update_bits(component, WM8985_POWER_MANAGEMENT_1, WM8985_VMIDSEL_MASK, 2 << WM8985_VMIDSEL_SHIFT);
        }
        SND_SOC_BIAS_OFF => {
            /* disable thermal shutdown */
            snd_soc_component_update_bits(component, WM8985_OUTPUT_CTRL0, WM8985_TSOPCTRL_MASK, 0);
            snd_soc_component_update_bits(component, WM8985_OUTPUT_CTRL0, WM8985_TSDEN_MASK, 0);
            /* disable VMIDSEL and BIASEN */
            snd_soc_component_update_bits(component, WM8985_POWER_MANAGEMENT_1, WM8985_VMIDSEL_MASK | WM8985_BIASEN_MASK, 0);
            snd_soc_component_write(component, WM8985_POWER_MANAGEMENT_1, 0);
            snd_soc_component_write(component, WM8985_POWER_MANAGEMENT_2, 0);
            snd_soc_component_write(component, WM8985_POWER_MANAGEMENT_3, 0);
            regcache_mark_dirty((*wm8985).regmap);
            regulator_bulk_disable((*wm8985).supplies.len() as c_int, (*wm8985).supplies.as_mut_ptr());
        }
    }
    0
}

unsafe extern "C" fn wm8985_probe(component: *mut snd_soc_component) -> c_int {
    let mut i: usize;
    let wm8985 = snd_soc_component_get_drvdata(component) as *mut wm8985_priv;
    let mut ret: c_int;

    i = 0;
    while i < (*wm8985).supplies.len() {
        (*wm8985).supplies[i].supply = wm8985_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get((*component).dev, (*wm8985).supplies.len() as c_int, (*wm8985).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, cstr!("Failed to request supplies: %d\n"), ret);
        return ret;
    }

    ret = regulator_bulk_enable((*wm8985).supplies.len() as c_int, (*wm8985).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, cstr!("Failed to enable supplies: %d\n"), ret);
        return ret;
    }

    ret = wm8985_reset(component);
    if ret < 0 {
        dev_err((*component).dev, cstr!("Failed to issue reset: %d\n"), ret);
        regulator_bulk_disable((*wm8985).supplies.len() as c_int, (*wm8985).supplies.as_mut_ptr());
        return ret;
    }

    /* latch volume update bits */
    i = 0;
    while i < volume_update_regs.len() {
        snd_soc_component_update_bits(component, volume_update_regs[i] as c_uint, 0x100, 0x100);
        i += 1;
    }
    /* enable BIASCUT */
    snd_soc_component_update_bits(component, WM8985_BIAS_CTRL, WM8985_BIASCUT, WM8985_BIASCUT);
    wm8985_add_widgets(component);
    0
}

static wm8985_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    mute_stream: Some(wm8985_dac_mute),
    hw_params: Some(wm8985_hw_params),
    set_fmt: Some(wm8985_set_fmt),
    set_sysclk: Some(wm8985_set_sysclk),
    set_pll: Some(wm8985_set_pll),
    no_capture_mute: 1,
};

const WM8985_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut wm8985_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("wm8985-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("Playback"),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: WM8985_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: cstr!("Capture"),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: WM8985_FORMATS,
    },
    ops: &wm8985_dai_ops,
    symmetric_rate: 1,
};

static soc_component_dev_wm8985: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8985_probe),
    set_bias_level: Some(wm8985_set_bias_level),
    controls: wm8985_common_snd_controls.as_ptr(),
    num_controls: wm8985_common_snd_controls.len() as c_uint,
    dapm_widgets: wm8985_common_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8985_common_dapm_widgets.len() as c_uint,
    dapm_routes: wm8985_common_dapm_routes.as_ptr(),
    num_dapm_routes: wm8985_common_dapm_routes.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8985_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: WM8985_MAX_REGISTER,
    writeable_reg: Some(wm8985_writeable),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: wm8985_reg_defaults.as_ptr(),
    num_reg_defaults: wm8985_reg_defaults.len() as c_uint,
};

/* Original C conditional: #if defined(CONFIG_SPI_MASTER) */
unsafe extern "C" fn wm8985_spi_probe(spi: *mut spi_device) -> c_int {
    let wm8985: *mut wm8985_priv;
    let mut ret: c_int;

    wm8985 = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<wm8985_priv>(), GFP_KERNEL) as *mut wm8985_priv;
    if wm8985.is_null() {
        return -ENOMEM;
    }

    spi_set_drvdata(spi, wm8985 as *mut c_void);
    (*wm8985).dev_type = wm8985_type::WM8985;
    (*wm8985).regmap = devm_regmap_init_spi(spi, &wm8985_regmap);
    if IS_ERR((*wm8985).regmap as *const c_void) {
        ret = PTR_ERR((*wm8985).regmap as *const c_void) as c_int;
        dev_err(&mut (*spi).dev, cstr!("Failed to allocate register map: %d\n"), ret);
        return ret;
    }

    ret = devm_snd_soc_register_component(&mut (*spi).dev, &soc_component_dev_wm8985, &mut wm8985_dai, 1);
    ret
}

static mut wm8985_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: cstr!("wm8985"),
    },
    probe: Some(wm8985_spi_probe),
};

/* Original C conditional: #if IS_ENABLED(CONFIG_I2C) */
unsafe extern "C" fn wm8985_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8985: *mut wm8985_priv;
    let mut ret: c_int;

    wm8985 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8985_priv>(), GFP_KERNEL) as *mut wm8985_priv;
    if wm8985.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, wm8985 as *mut c_void);
    (*wm8985).dev_type = core::mem::transmute::<usize, wm8985_type>(i2c_get_match_data(i2c) as usize);
    (*wm8985).regmap = devm_regmap_init_i2c(i2c, &wm8985_regmap);
    if IS_ERR((*wm8985).regmap as *const c_void) {
        ret = PTR_ERR((*wm8985).regmap as *const c_void) as c_int;
        dev_err(&mut (*i2c).dev, cstr!("Failed to allocate register map: %d\n"), ret);
        return ret;
    }

    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_wm8985, &mut wm8985_dai, 1);
    ret
}

static wm8985_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id { name: *b"wm8985\0", driver_data: wm8985_type::WM8985 as c_ulong },
    i2c_device_id { name: *b"wm8758\0", driver_data: wm8985_type::WM8758 as c_ulong },
    i2c_device_id { name: [0; I2C_NAME_SIZE], driver_data: 0 },
];
module_device_table!(i2c, wm8985_i2c_id);

static mut wm8985_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: cstr!("wm8985"),
    },
    probe: Some(wm8985_i2c_probe),
    id_table: wm8985_i2c_id.as_ptr(),
};

unsafe extern "C" fn wm8985_modinit() -> c_int {
    let mut ret: c_int = 0;

    /* Original C conditional: #if IS_ENABLED(CONFIG_I2C) */
    ret = i2c_add_driver(&mut wm8985_i2c_driver);
    if ret != 0 {
        printk(KERN_ERR, cstr!("Failed to register wm8985 I2C driver: %d\n"), ret);
    }
    /* Original C conditional: #if defined(CONFIG_SPI_MASTER) */
    ret = spi_register_driver(&mut wm8985_spi_driver);
    if ret != 0 {
        printk(KERN_ERR, cstr!("Failed to register wm8985 SPI driver: %d\n"), ret);
    }
    ret
}
module_init!(wm8985_modinit);

unsafe extern "C" fn wm8985_exit() {
    /* Original C conditional: #if IS_ENABLED(CONFIG_I2C) */
    i2c_del_driver(&mut wm8985_i2c_driver);
    /* Original C conditional: #if defined(CONFIG_SPI_MASTER) */
    spi_unregister_driver(&mut wm8985_spi_driver);
}
module_exit!(wm8985_exit);

module_description!("ASoC WM8985 / WM8758 driver");
module_author!("Dimitris Papastamos <dp@opensource.wolfsonmicro.com>");
module_license!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
