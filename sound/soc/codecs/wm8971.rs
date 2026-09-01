// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * wm8971.c  --  WM8971 ALSA SoC Audio driver
 *
 * Copyright 2005 Lab126, Inc.
 *
 * Author: Kenneth Kiraly <kiraly@lab126.com>
 *
 * Based on wm8753.c by Liam Girdwood
 */

// C dependencies translated as external Rust dependencies:
// linux/module.h, moduleparam.h, init.h, delay.h, pm.h, i2c.h, regmap.h,
// slab.h, sound/core.h, pcm.h, pcm_params.h, soc.h, initval.h, and "wm8971.h".

const WM8971_REG_COUNT: usize = 43;

/* codec private data */
#[repr(C)]
struct wm8971_priv {
    sysclk: c_uint,
    charge_work: delayed_work,
    regmap: *mut regmap,
}

/*
 * wm8971 register cache
 * We can't read the WM8971 register space when we
 * are using 2 wire for device control, so we cache them instead.
 */
static wm8971_reg_defaults: [reg_default; 43] = [
    reg_default { reg: 0, def: 0x0097 },
    reg_default { reg: 1, def: 0x0097 },
    reg_default { reg: 2, def: 0x0079 },
    reg_default { reg: 3, def: 0x0079 },
    reg_default { reg: 4, def: 0x0000 },
    reg_default { reg: 5, def: 0x0008 },
    reg_default { reg: 6, def: 0x0000 },
    reg_default { reg: 7, def: 0x000a },
    reg_default { reg: 8, def: 0x0000 },
    reg_default { reg: 9, def: 0x0000 },
    reg_default { reg: 10, def: 0x00ff },
    reg_default { reg: 11, def: 0x00ff },
    reg_default { reg: 12, def: 0x000f },
    reg_default { reg: 13, def: 0x000f },
    reg_default { reg: 14, def: 0x0000 },
    reg_default { reg: 15, def: 0x0000 },
    reg_default { reg: 16, def: 0x0000 },
    reg_default { reg: 17, def: 0x007b },
    reg_default { reg: 18, def: 0x0000 },
    reg_default { reg: 19, def: 0x0032 },
    reg_default { reg: 20, def: 0x0000 },
    reg_default { reg: 21, def: 0x00c3 },
    reg_default { reg: 22, def: 0x00c3 },
    reg_default { reg: 23, def: 0x00c0 },
    reg_default { reg: 24, def: 0x0000 },
    reg_default { reg: 25, def: 0x0000 },
    reg_default { reg: 26, def: 0x0000 },
    reg_default { reg: 27, def: 0x0000 },
    reg_default { reg: 28, def: 0x0000 },
    reg_default { reg: 29, def: 0x0000 },
    reg_default { reg: 30, def: 0x0000 },
    reg_default { reg: 31, def: 0x0000 },
    reg_default { reg: 32, def: 0x0000 },
    reg_default { reg: 33, def: 0x0000 },
    reg_default { reg: 34, def: 0x0050 },
    reg_default { reg: 35, def: 0x0050 },
    reg_default { reg: 36, def: 0x0050 },
    reg_default { reg: 37, def: 0x0050 },
    reg_default { reg: 38, def: 0x0050 },
    reg_default { reg: 39, def: 0x0050 },
    reg_default { reg: 40, def: 0x0079 },
    reg_default { reg: 41, def: 0x0079 },
    reg_default { reg: 42, def: 0x0079 },
];

macro_rules! wm8971_reset {
    ($c:expr) => {
        snd_soc_component_write($c, WM8971_RESET, 0)
    };
}

/* WM8971 Controls */
static wm8971_bass: [&str; 2] = ["Linear Control", "Adaptive Boost"];
static wm8971_bass_filter: [&str; 2] = ["130Hz @ 48kHz", "200Hz @ 48kHz"];
static wm8971_treble: [&str; 2] = ["8kHz", "4kHz"];
static wm8971_alc_func: [&str; 4] = ["Off", "Right", "Left", "Stereo"];
static wm8971_ng_type: [&str; 2] = ["Constant PGA Gain", "Mute ADC Output"];
static wm8971_deemp: [&str; 4] = ["None", "32kHz", "44.1kHz", "48kHz"];
static wm8971_mono_mux: [&str; 4] = ["Stereo", "Mono (Left)", "Mono (Right)", "Digital Mono"];
static wm8971_dac_phase: [&str; 2] = ["Non Inverted", "Inverted"];
static wm8971_lline_mux: [&str; 5] = ["Line", "NC", "NC", "PGA", "Differential"];
static wm8971_rline_mux: [&str; 5] = ["Line", "Mic", "NC", "PGA", "Differential"];
static wm8971_lpga_sel: [&str; 4] = ["Line", "NC", "NC", "Differential"];
static wm8971_rpga_sel: [&str; 4] = ["Line", "Mic", "NC", "Differential"];
static wm8971_adcpol: [&str; 4] = ["Normal", "L Invert", "R Invert", "L + R Invert"];

static wm8971_enum: [soc_enum; 14] = [
    SOC_ENUM_SINGLE!(WM8971_BASS, 7, 2, wm8971_bass), /* 0 */
    SOC_ENUM_SINGLE!(WM8971_BASS, 6, 2, wm8971_bass_filter),
    SOC_ENUM_SINGLE!(WM8971_TREBLE, 6, 2, wm8971_treble),
    SOC_ENUM_SINGLE!(WM8971_ALC1, 7, 4, wm8971_alc_func),
    SOC_ENUM_SINGLE!(WM8971_NGATE, 1, 2, wm8971_ng_type), /* 4 */
    SOC_ENUM_SINGLE!(WM8971_ADCDAC, 1, 4, wm8971_deemp),
    SOC_ENUM_SINGLE!(WM8971_ADCTL1, 4, 4, wm8971_mono_mux),
    SOC_ENUM_SINGLE!(WM8971_ADCTL1, 1, 2, wm8971_dac_phase),
    SOC_ENUM_SINGLE!(WM8971_LOUTM1, 0, 5, wm8971_lline_mux), /* 8 */
    SOC_ENUM_SINGLE!(WM8971_ROUTM1, 0, 5, wm8971_rline_mux),
    SOC_ENUM_SINGLE!(WM8971_LADCIN, 6, 4, wm8971_lpga_sel),
    SOC_ENUM_SINGLE!(WM8971_RADCIN, 6, 4, wm8971_rpga_sel),
    SOC_ENUM_SINGLE!(WM8971_ADCDAC, 5, 4, wm8971_adcpol), /* 12 */
    SOC_ENUM_SINGLE!(WM8971_ADCIN, 6, 4, wm8971_mono_mux),
];

static wm8971_snd_controls: [snd_kcontrol_new; 38] = [
    SOC_DOUBLE_R!("Capture Volume", WM8971_LINVOL, WM8971_RINVOL, 0, 63, 0),
    SOC_DOUBLE_R!("Capture ZC Switch", WM8971_LINVOL, WM8971_RINVOL, 6, 1, 0),
    SOC_DOUBLE_R!("Capture Switch", WM8971_LINVOL, WM8971_RINVOL, 7, 1, 1),
    SOC_DOUBLE_R!("Headphone Playback ZC Switch", WM8971_LOUT1V, WM8971_ROUT1V, 7, 1, 0),
    SOC_DOUBLE_R!("Speaker Playback ZC Switch", WM8971_LOUT2V, WM8971_ROUT2V, 7, 1, 0),
    SOC_SINGLE!("Mono Playback ZC Switch", WM8971_MOUTV, 7, 1, 0),
    SOC_DOUBLE_R!("PCM Volume", WM8971_LDAC, WM8971_RDAC, 0, 255, 0),
    SOC_DOUBLE_R!("Bypass Left Playback Volume", WM8971_LOUTM1, WM8971_LOUTM2, 4, 7, 1),
    SOC_DOUBLE_R!("Bypass Right Playback Volume", WM8971_ROUTM1, WM8971_ROUTM2, 4, 7, 1),
    SOC_DOUBLE_R!("Bypass Mono Playback Volume", WM8971_MOUTM1, WM8971_MOUTM2, 4, 7, 1),
    SOC_DOUBLE_R!("Headphone Playback Volume", WM8971_LOUT1V, WM8971_ROUT1V, 0, 127, 0),
    SOC_DOUBLE_R!("Speaker Playback Volume", WM8971_LOUT2V, WM8971_ROUT2V, 0, 127, 0),
    SOC_ENUM!("Bass Boost", wm8971_enum[0]),
    SOC_ENUM!("Bass Filter", wm8971_enum[1]),
    SOC_SINGLE!("Bass Volume", WM8971_BASS, 0, 7, 1),
    SOC_SINGLE!("Treble Volume", WM8971_TREBLE, 0, 7, 0),
    SOC_ENUM!("Treble Cut-off", wm8971_enum[2]),
    SOC_SINGLE!("Capture Filter Switch", WM8971_ADCDAC, 0, 1, 1),
    SOC_SINGLE!("ALC Target Volume", WM8971_ALC1, 0, 7, 0),
    SOC_SINGLE!("ALC Max Volume", WM8971_ALC1, 4, 7, 0),
    SOC_SINGLE!("ALC Capture Target Volume", WM8971_ALC1, 0, 7, 0),
    SOC_SINGLE!("ALC Capture Max Volume", WM8971_ALC1, 4, 7, 0),
    SOC_ENUM!("ALC Capture Function", wm8971_enum[3]),
    SOC_SINGLE!("ALC Capture ZC Switch", WM8971_ALC2, 7, 1, 0),
    SOC_SINGLE!("ALC Capture Hold Time", WM8971_ALC2, 0, 15, 0),
    SOC_SINGLE!("ALC Capture Decay Time", WM8971_ALC3, 4, 15, 0),
    SOC_SINGLE!("ALC Capture Attack Time", WM8971_ALC3, 0, 15, 0),
    SOC_SINGLE!("ALC Capture NG Threshold", WM8971_NGATE, 3, 31, 0),
    SOC_ENUM!("ALC Capture NG Type", wm8971_enum[4]),
    SOC_SINGLE!("ALC Capture NG Switch", WM8971_NGATE, 0, 1, 0),
    SOC_SINGLE!("Capture 6dB Attenuate", WM8971_ADCDAC, 8, 1, 0),
    SOC_SINGLE!("Playback 6dB Attenuate", WM8971_ADCDAC, 7, 1, 0),
    SOC_ENUM!("Playback De-emphasis", wm8971_enum[5]),
    SOC_ENUM!("Playback Function", wm8971_enum[6]),
    SOC_ENUM!("Playback Phase", wm8971_enum[7]),
    SOC_DOUBLE_R!("Mic Boost", WM8971_LADCIN, WM8971_RADCIN, 4, 3, 0),
];

/*
 * DAPM Controls
 */

/* Left Mixer */
static wm8971_left_mixer_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE!("Playback Switch", WM8971_LOUTM1, 8, 1, 0),
    SOC_DAPM_SINGLE!("Left Bypass Switch", WM8971_LOUTM1, 7, 1, 0),
    SOC_DAPM_SINGLE!("Right Playback Switch", WM8971_LOUTM2, 8, 1, 0),
    SOC_DAPM_SINGLE!("Right Bypass Switch", WM8971_LOUTM2, 7, 1, 0),
];

/* Right Mixer */
static wm8971_right_mixer_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE!("Left Playback Switch", WM8971_ROUTM1, 8, 1, 0),
    SOC_DAPM_SINGLE!("Left Bypass Switch", WM8971_ROUTM1, 7, 1, 0),
    SOC_DAPM_SINGLE!("Playback Switch", WM8971_ROUTM2, 8, 1, 0),
    SOC_DAPM_SINGLE!("Right Bypass Switch", WM8971_ROUTM2, 7, 1, 0),
];

/* Mono Mixer */
static wm8971_mono_mixer_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE!("Left Playback Switch", WM8971_MOUTM1, 8, 1, 0),
    SOC_DAPM_SINGLE!("Left Bypass Switch", WM8971_MOUTM1, 7, 1, 0),
    SOC_DAPM_SINGLE!("Right Playback Switch", WM8971_MOUTM2, 8, 1, 0),
    SOC_DAPM_SINGLE!("Right Bypass Switch", WM8971_MOUTM2, 7, 1, 0),
];

/* Left Line Mux */
static wm8971_left_line_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8971_enum[8]);

/* Right Line Mux */
static wm8971_right_line_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8971_enum[9]);

/* Left PGA Mux */
static wm8971_left_pga_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8971_enum[10]);

/* Right PGA Mux */
static wm8971_right_pga_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8971_enum[11]);

/* Mono ADC Mux */
static wm8971_monomux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8971_enum[13]);

static wm8971_dapm_widgets: [snd_soc_dapm_widget; 24] = [
    SND_SOC_DAPM_MIXER!("Left Mixer", SND_SOC_NOPM, 0, 0, &wm8971_left_mixer_controls[0], wm8971_left_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Right Mixer", SND_SOC_NOPM, 0, 0, &wm8971_right_mixer_controls[0], wm8971_right_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Mono Mixer", WM8971_PWR2, 2, 0, &wm8971_mono_mixer_controls[0], wm8971_mono_mixer_controls.len()),
    SND_SOC_DAPM_PGA!("Right Out 2", WM8971_PWR2, 3, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left Out 2", WM8971_PWR2, 4, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right Out 1", WM8971_PWR2, 5, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left Out 1", WM8971_PWR2, 6, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_DAC!("Right DAC", "Right Playback", WM8971_PWR2, 7, 0),
    SND_SOC_DAPM_DAC!("Left DAC", "Left Playback", WM8971_PWR2, 8, 0),
    SND_SOC_DAPM_PGA!("Mono Out 1", WM8971_PWR2, 2, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Bias", WM8971_PWR1, 1, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_ADC!("Right ADC", "Right Capture", WM8971_PWR1, 2, 0),
    SND_SOC_DAPM_ADC!("Left ADC", "Left Capture", WM8971_PWR1, 3, 0),
    SND_SOC_DAPM_MUX!("Left PGA Mux", WM8971_PWR1, 5, 0, &wm8971_left_pga_controls),
    SND_SOC_DAPM_MUX!("Right PGA Mux", WM8971_PWR1, 4, 0, &wm8971_right_pga_controls),
    SND_SOC_DAPM_MUX!("Left Line Mux", SND_SOC_NOPM, 0, 0, &wm8971_left_line_controls),
    SND_SOC_DAPM_MUX!("Right Line Mux", SND_SOC_NOPM, 0, 0, &wm8971_right_line_controls),
    SND_SOC_DAPM_MUX!("Left ADC Mux", SND_SOC_NOPM, 0, 0, &wm8971_monomux_controls),
    SND_SOC_DAPM_MUX!("Right ADC Mux", SND_SOC_NOPM, 0, 0, &wm8971_monomux_controls),
    SND_SOC_DAPM_OUTPUT!("LOUT1"),
    SND_SOC_DAPM_OUTPUT!("ROUT1"),
    SND_SOC_DAPM_OUTPUT!("LOUT2"),
    SND_SOC_DAPM_OUTPUT!("ROUT2"),
    SND_SOC_DAPM_OUTPUT!("MONO"),
    SND_SOC_DAPM_INPUT!("LINPUT1"),
    SND_SOC_DAPM_INPUT!("RINPUT1"),
    SND_SOC_DAPM_INPUT!("MIC"),
];

static wm8971_dapm_routes: [snd_soc_dapm_route; 43] = [
    /* left mixer */
    snd_soc_dapm_route { sink: "Left Mixer", control: "Playback Switch", source: "Left DAC" },
    snd_soc_dapm_route { sink: "Left Mixer", control: "Left Bypass Switch", source: "Left Line Mux" },
    snd_soc_dapm_route { sink: "Left Mixer", control: "Right Playback Switch", source: "Right DAC" },
    snd_soc_dapm_route { sink: "Left Mixer", control: "Right Bypass Switch", source: "Right Line Mux" },
    /* right mixer */
    snd_soc_dapm_route { sink: "Right Mixer", control: "Left Playback Switch", source: "Left DAC" },
    snd_soc_dapm_route { sink: "Right Mixer", control: "Left Bypass Switch", source: "Left Line Mux" },
    snd_soc_dapm_route { sink: "Right Mixer", control: "Playback Switch", source: "Right DAC" },
    snd_soc_dapm_route { sink: "Right Mixer", control: "Right Bypass Switch", source: "Right Line Mux" },
    /* left out 1 */
    snd_soc_dapm_route { sink: "Left Out 1", control: core::ptr::null(), source: "Left Mixer" },
    snd_soc_dapm_route { sink: "LOUT1", control: core::ptr::null(), source: "Left Out 1" },
    /* left out 2 */
    snd_soc_dapm_route { sink: "Left Out 2", control: core::ptr::null(), source: "Left Mixer" },
    snd_soc_dapm_route { sink: "LOUT2", control: core::ptr::null(), source: "Left Out 2" },
    /* right out 1 */
    snd_soc_dapm_route { sink: "Right Out 1", control: core::ptr::null(), source: "Right Mixer" },
    snd_soc_dapm_route { sink: "ROUT1", control: core::ptr::null(), source: "Right Out 1" },
    /* right out 2 */
    snd_soc_dapm_route { sink: "Right Out 2", control: core::ptr::null(), source: "Right Mixer" },
    snd_soc_dapm_route { sink: "ROUT2", control: core::ptr::null(), source: "Right Out 2" },
    /* mono mixer */
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Left Playback Switch", source: "Left DAC" },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Left Bypass Switch", source: "Left Line Mux" },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Right Playback Switch", source: "Right DAC" },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Right Bypass Switch", source: "Right Line Mux" },
    /* mono out */
    snd_soc_dapm_route { sink: "Mono Out", control: core::ptr::null(), source: "Mono Mixer" },
    snd_soc_dapm_route { sink: "MONO1", control: core::ptr::null(), source: "Mono Out" },
    /* Left Line Mux */
    snd_soc_dapm_route { sink: "Left Line Mux", control: "Line", source: "LINPUT1" },
    snd_soc_dapm_route { sink: "Left Line Mux", control: "PGA", source: "Left PGA Mux" },
    snd_soc_dapm_route { sink: "Left Line Mux", control: "Differential", source: "Differential Mux" },
    /* Right Line Mux */
    snd_soc_dapm_route { sink: "Right Line Mux", control: "Line", source: "RINPUT1" },
    snd_soc_dapm_route { sink: "Right Line Mux", control: "Mic", source: "MIC" },
    snd_soc_dapm_route { sink: "Right Line Mux", control: "PGA", source: "Right PGA Mux" },
    snd_soc_dapm_route { sink: "Right Line Mux", control: "Differential", source: "Differential Mux" },
    /* Left PGA Mux */
    snd_soc_dapm_route { sink: "Left PGA Mux", control: "Line", source: "LINPUT1" },
    snd_soc_dapm_route { sink: "Left PGA Mux", control: "Differential", source: "Differential Mux" },
    /* Right PGA Mux */
    snd_soc_dapm_route { sink: "Right PGA Mux", control: "Line", source: "RINPUT1" },
    snd_soc_dapm_route { sink: "Right PGA Mux", control: "Differential", source: "Differential Mux" },
    /* Differential Mux */
    snd_soc_dapm_route { sink: "Differential Mux", control: "Line", source: "LINPUT1" },
    snd_soc_dapm_route { sink: "Differential Mux", control: "Line", source: "RINPUT1" },
    /* Left ADC Mux */
    snd_soc_dapm_route { sink: "Left ADC Mux", control: "Stereo", source: "Left PGA Mux" },
    snd_soc_dapm_route { sink: "Left ADC Mux", control: "Mono (Left)", source: "Left PGA Mux" },
    snd_soc_dapm_route { sink: "Left ADC Mux", control: "Digital Mono", source: "Left PGA Mux" },
    /* Right ADC Mux */
    snd_soc_dapm_route { sink: "Right ADC Mux", control: "Stereo", source: "Right PGA Mux" },
    snd_soc_dapm_route { sink: "Right ADC Mux", control: "Mono (Right)", source: "Right PGA Mux" },
    snd_soc_dapm_route { sink: "Right ADC Mux", control: "Digital Mono", source: "Right PGA Mux" },
    /* ADC */
    snd_soc_dapm_route { sink: "Left ADC", control: core::ptr::null(), source: "Left ADC Mux" },
    snd_soc_dapm_route { sink: "Right ADC", control: core::ptr::null(), source: "Right ADC Mux" },
];

#[repr(C)]
struct _coeff_div {
    mclk: u32,
    rate: u32,
    fs: u16,
    /* C bitfields: u8 sr:5; u8 usb:1; */
    sr: u8,
    usb: u8,
}

/* codec hifi mclk clock divider coefficients */
static coeff_div: [_coeff_div; 30] = [
    /* 8k */
    _coeff_div { mclk: 12288000, rate: 8000, fs: 1536, sr: 0x6, usb: 0x0 },
    _coeff_div { mclk: 11289600, rate: 8000, fs: 1408, sr: 0x16, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 8000, fs: 2304, sr: 0x7, usb: 0x0 },
    _coeff_div { mclk: 16934400, rate: 8000, fs: 2112, sr: 0x17, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 8000, fs: 1500, sr: 0x6, usb: 0x1 },
    /* 11.025k */
    _coeff_div { mclk: 11289600, rate: 11025, fs: 1024, sr: 0x18, usb: 0x0 },
    _coeff_div { mclk: 16934400, rate: 11025, fs: 1536, sr: 0x19, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 11025, fs: 1088, sr: 0x19, usb: 0x1 },
    /* 16k */
    _coeff_div { mclk: 12288000, rate: 16000, fs: 768, sr: 0xa, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 16000, fs: 1152, sr: 0xb, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 16000, fs: 750, sr: 0xa, usb: 0x1 },
    /* 22.05k */
    _coeff_div { mclk: 11289600, rate: 22050, fs: 512, sr: 0x1a, usb: 0x0 },
    _coeff_div { mclk: 16934400, rate: 22050, fs: 768, sr: 0x1b, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 22050, fs: 544, sr: 0x1b, usb: 0x1 },
    /* 32k */
    _coeff_div { mclk: 12288000, rate: 32000, fs: 384, sr: 0xc, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 32000, fs: 576, sr: 0xd, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 32000, fs: 375, sr: 0xa, usb: 0x1 },
    /* 44.1k */
    _coeff_div { mclk: 11289600, rate: 44100, fs: 256, sr: 0x10, usb: 0x0 },
    _coeff_div { mclk: 16934400, rate: 44100, fs: 384, sr: 0x11, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 44100, fs: 272, sr: 0x11, usb: 0x1 },
    /* 48k */
    _coeff_div { mclk: 12288000, rate: 48000, fs: 256, sr: 0x0, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 48000, fs: 384, sr: 0x1, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 48000, fs: 250, sr: 0x0, usb: 0x1 },
    /* 88.2k */
    _coeff_div { mclk: 11289600, rate: 88200, fs: 128, sr: 0x1e, usb: 0x0 },
    _coeff_div { mclk: 16934400, rate: 88200, fs: 192, sr: 0x1f, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 88200, fs: 136, sr: 0x1f, usb: 0x1 },
    /* 96k */
    _coeff_div { mclk: 12288000, rate: 96000, fs: 128, sr: 0xe, usb: 0x0 },
    _coeff_div { mclk: 18432000, rate: 96000, fs: 192, sr: 0xf, usb: 0x0 },
    _coeff_div { mclk: 12000000, rate: 96000, fs: 125, sr: 0xe, usb: 0x1 },
];

fn get_coeff(mclk: c_int, rate: c_int) -> c_int {
    let mut i: usize = 0;

    while i < coeff_div.len() {
        if coeff_div[i].rate == rate as u32 && coeff_div[i].mclk == mclk as u32 {
            return i as c_int;
        }
        i += 1;
    }
    -EINVAL
}

unsafe extern "C" fn wm8971_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let wm8971: *mut wm8971_priv = snd_soc_component_get_drvdata(component) as *mut wm8971_priv;

    match freq {
        11289600 | 12000000 | 12288000 | 16934400 | 18432000 => {
            (*wm8971).sysclk = freq;
            0
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn wm8971_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let mut iface: u16 = 0;

    /* set master/slave audio interface */
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => iface = 0x0040,
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => iface |= 0x0002,
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => iface |= 0x0001,
        SND_SOC_DAIFMT_DSP_A => iface |= 0x0003,
        SND_SOC_DAIFMT_DSP_B => iface |= 0x0013,
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

    snd_soc_component_write(component, WM8971_IFACE, iface as c_uint);
    0
}

unsafe extern "C" fn wm8971_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let wm8971: *mut wm8971_priv = snd_soc_component_get_drvdata(component) as *mut wm8971_priv;
    let mut iface: u16 = (snd_soc_component_read(component, WM8971_IFACE) & 0x1f3) as u16;
    let srate: u16 = (snd_soc_component_read(component, WM8971_SRATE) & 0x1c0) as u16;
    let coeff: c_int = get_coeff((*wm8971).sysclk as c_int, params_rate(params) as c_int);

    /* bit size */
    match params_width(params) {
        16 => {}
        20 => iface |= 0x0004,
        24 => iface |= 0x0008,
        32 => iface |= 0x000c,
        _ => {}
    }

    /* set iface & srate */
    snd_soc_component_write(component, WM8971_IFACE, iface as c_uint);
    if coeff >= 0 {
        snd_soc_component_write(
            component,
            WM8971_SRATE,
            (srate | ((coeff_div[coeff as usize].sr as u16) << 1) | coeff_div[coeff as usize].usb as u16) as c_uint,
        );
    }

    0
}

unsafe extern "C" fn wm8971_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mute_reg: u16 = (snd_soc_component_read(component, WM8971_ADCDAC) & 0xfff7) as u16;

    if mute != 0 {
        snd_soc_component_write(component, WM8971_ADCDAC, (mute_reg | 0x8) as c_uint);
    } else {
        snd_soc_component_write(component, WM8971_ADCDAC, mute_reg as c_uint);
    }
    0
}

unsafe extern "C" fn wm8971_charge_work(work: *mut work_struct) {
    let wm8971: *mut wm8971_priv = container_of!(work, wm8971_priv, charge_work.work);

    /* Set to 500k */
    regmap_update_bits((*wm8971).regmap, WM8971_PWR1, 0x0180, 0x0100);
}

unsafe extern "C" fn wm8971_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let wm8971: *mut wm8971_priv = snd_soc_component_get_drvdata(component) as *mut wm8971_priv;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let pwr_reg: u16 = (snd_soc_component_read(component, WM8971_PWR1) & 0xfe3e) as u16;

    match level {
        SND_SOC_BIAS_ON => {
            /* set vmid to 50k and unmute dac */
            snd_soc_component_write(component, WM8971_PWR1, (pwr_reg | 0x00c1) as c_uint);
        }
        SND_SOC_BIAS_PREPARE => {
            /* Wait until fully charged */
            flush_delayed_work(&mut (*wm8971).charge_work);
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                snd_soc_component_cache_sync(component);
                /* charge output caps - set vmid to 5k for quick power up */
                snd_soc_component_write(component, WM8971_PWR1, (pwr_reg | 0x01c0) as c_uint);
                queue_delayed_work(
                    system_power_efficient_wq,
                    &mut (*wm8971).charge_work,
                    msecs_to_jiffies(1000),
                );
            } else {
                /* mute dac and set vmid to 500k, enable VREF */
                snd_soc_component_write(component, WM8971_PWR1, (pwr_reg | 0x0140) as c_uint);
            }
        }
        SND_SOC_BIAS_OFF => {
            cancel_delayed_work_sync(&mut (*wm8971).charge_work);
            snd_soc_component_write(component, WM8971_PWR1, 0x0001);
        }
    }
    0
}

const WM8971_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_11025
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000;

const WM8971_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static wm8971_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8971_pcm_hw_params),
    mute_stream: Some(wm8971_mute),
    set_fmt: Some(wm8971_set_dai_fmt),
    set_sysclk: Some(wm8971_set_dai_sysclk),
    no_capture_mute: 1,
};

static mut wm8971_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: "wm8971-hifi",
    playback: snd_soc_pcm_stream {
        stream_name: "Playback",
        channels_min: 1,
        channels_max: 2,
        rates: WM8971_RATES,
        formats: WM8971_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: "Capture",
        channels_min: 1,
        channels_max: 2,
        rates: WM8971_RATES,
        formats: WM8971_FORMATS,
    },
    ops: &wm8971_dai_ops,
};

unsafe extern "C" fn wm8971_probe(component: *mut snd_soc_component) -> c_int {
    let wm8971: *mut wm8971_priv = snd_soc_component_get_drvdata(component) as *mut wm8971_priv;

    INIT_DELAYED_WORK!(&mut (*wm8971).charge_work, wm8971_charge_work);

    wm8971_reset!(component);

    /* set the update bits */
    snd_soc_component_update_bits(component, WM8971_LDAC, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8971_RDAC, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8971_LOUT1V, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8971_ROUT1V, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8971_LOUT2V, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8971_ROUT2V, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8971_LINVOL, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8971_RINVOL, 0x0100, 0x0100);

    0
}

static soc_component_dev_wm8971: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8971_probe),
    set_bias_level: Some(wm8971_set_bias_level),
    controls: wm8971_snd_controls.as_ptr(),
    num_controls: wm8971_snd_controls.len(),
    dapm_widgets: wm8971_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8971_dapm_widgets.len(),
    dapm_routes: wm8971_dapm_routes.as_ptr(),
    num_dapm_routes: wm8971_dapm_routes.len(),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8971_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: WM8971_MOUTV,
    reg_defaults: wm8971_reg_defaults.as_ptr(),
    num_reg_defaults: wm8971_reg_defaults.len(),
    cache_type: REGCACHE_MAPLE,
};

unsafe extern "C" fn wm8971_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut wm8971: *mut wm8971_priv;

    wm8971 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<wm8971_priv>(),
        GFP_KERNEL,
    ) as *mut wm8971_priv;
    if wm8971.is_null() {
        return -ENOMEM;
    }

    (*wm8971).regmap = devm_regmap_init_i2c(i2c, &wm8971_regmap);
    if IS_ERR((*wm8971).regmap) {
        return PTR_ERR((*wm8971).regmap);
    }

    i2c_set_clientdata(i2c, wm8971 as *mut c_void);

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm8971,
        &mut wm8971_dai,
        1,
    )
}

static wm8971_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: "wm8971",
    },
    i2c_device_id::default(),
];
MODULE_DEVICE_TABLE!(i2c, wm8971_i2c_id);

static mut wm8971_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: "wm8971",
    },
    probe: Some(wm8971_i2c_probe),
    id_table: wm8971_i2c_id.as_ptr(),
};

module_i2c_driver!(wm8971_i2c_driver);

MODULE_DESCRIPTION!("ASoC WM8971 driver");
MODULE_AUTHOR!("Lab126");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
