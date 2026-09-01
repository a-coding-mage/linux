// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8750.c -- WM8750 ALSA SoC audio driver
 *
 * Copyright 2005 Openedhand Ltd.
 *
 * Author: Richard Purdie <richard@openedhand.com>
 *
 * Based on WM8753.c
 */

/* Depends on Linux kernel, ALSA SoC, regmap, I2C, SPI, and wm8750 register definitions. */

#[repr(C)]
pub struct wm8750_priv {
    pub sysclk: c_uint,
}

#[repr(C)]
pub struct _coeff_div {
    pub mclk: u32,
    pub rate: u32,
    pub fs: u16,
    pub sr_usb: u8,
}

impl _coeff_div {
    pub const fn new(mclk: u32, rate: u32, fs: u16, sr: u8, usb: u8) -> Self {
        Self {
            mclk,
            rate,
            fs,
            sr_usb: (sr & 0x1f) | ((usb & 0x01) << 5),
        }
    }

    pub const fn sr(&self) -> u8 {
        self.sr_usb & 0x1f
    }

    pub const fn usb(&self) -> u8 {
        (self.sr_usb >> 5) & 0x01
    }
}

macro_rules! wm8750_reset {
    ($c:expr) => {
        snd_soc_component_write($c, WM8750_RESET, 0)
    };
}

/*
 * wm8750 register cache
 * We can't read the WM8750 register space when we
 * are using 2 wire for device control, so we cache them instead.
 */
static wm8750_reg_defaults: [reg_default; 43] = [
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

/*
 * WM8750 Controls
 */
static wm8750_bass: [&'static CStr; 2] = cstr_array!["Linear Control", "Adaptive Boost"];
static wm8750_bass_filter: [&'static CStr; 2] = cstr_array!["130Hz @ 48kHz", "200Hz @ 48kHz"];
static wm8750_treble: [&'static CStr; 2] = cstr_array!["8kHz", "4kHz"];
static wm8750_3d_lc: [&'static CStr; 2] = cstr_array!["200Hz", "500Hz"];
static wm8750_3d_uc: [&'static CStr; 2] = cstr_array!["2.2kHz", "1.5kHz"];
static wm8750_3d_func: [&'static CStr; 2] = cstr_array!["Capture", "Playback"];
static wm8750_alc_func: [&'static CStr; 4] = cstr_array!["Off", "Right", "Left", "Stereo"];
static wm8750_ng_type: [&'static CStr; 2] = cstr_array!["Constant PGA Gain", "Mute ADC Output"];
static wm8750_line_mux: [&'static CStr; 5] =
    cstr_array!["Line 1", "Line 2", "Line 3", "PGA", "Differential"];
static wm8750_pga_sel: [&'static CStr; 4] =
    cstr_array!["Line 1", "Line 2", "Line 3", "Differential"];
static wm8750_out3: [&'static CStr; 4] =
    cstr_array!["VREF", "ROUT1 + Vol", "MonoOut", "ROUT1"];
static wm8750_diff_sel: [&'static CStr; 2] = cstr_array!["Line 1", "Line 2"];
static wm8750_adcpol: [&'static CStr; 4] =
    cstr_array!["Normal", "L Invert", "R Invert", "L + R Invert"];
static wm8750_deemph: [&'static CStr; 4] = cstr_array!["None", "32Khz", "44.1Khz", "48Khz"];
static wm8750_mono_mux: [&'static CStr; 4] =
    cstr_array!["Stereo", "Mono (Left)", "Mono (Right)", "Digital Mono"];

static wm8750_enum: [soc_enum; 17] = [
    SOC_ENUM_SINGLE!(WM8750_BASS, 7, 2, wm8750_bass),
    SOC_ENUM_SINGLE!(WM8750_BASS, 6, 2, wm8750_bass_filter),
    SOC_ENUM_SINGLE!(WM8750_TREBLE, 6, 2, wm8750_treble),
    SOC_ENUM_SINGLE!(WM8750_3D, 5, 2, wm8750_3d_lc),
    SOC_ENUM_SINGLE!(WM8750_3D, 6, 2, wm8750_3d_uc),
    SOC_ENUM_SINGLE!(WM8750_3D, 7, 2, wm8750_3d_func),
    SOC_ENUM_SINGLE!(WM8750_ALC1, 7, 4, wm8750_alc_func),
    SOC_ENUM_SINGLE!(WM8750_NGATE, 1, 2, wm8750_ng_type),
    SOC_ENUM_SINGLE!(WM8750_LOUTM1, 0, 5, wm8750_line_mux),
    SOC_ENUM_SINGLE!(WM8750_ROUTM1, 0, 5, wm8750_line_mux),
    SOC_ENUM_SINGLE!(WM8750_LADCIN, 6, 4, wm8750_pga_sel), /* 10 */
    SOC_ENUM_SINGLE!(WM8750_RADCIN, 6, 4, wm8750_pga_sel),
    SOC_ENUM_SINGLE!(WM8750_ADCTL2, 7, 4, wm8750_out3),
    SOC_ENUM_SINGLE!(WM8750_ADCIN, 8, 2, wm8750_diff_sel),
    SOC_ENUM_SINGLE!(WM8750_ADCDAC, 5, 4, wm8750_adcpol),
    SOC_ENUM_SINGLE!(WM8750_ADCDAC, 1, 4, wm8750_deemph),
    SOC_ENUM_SINGLE!(WM8750_ADCIN, 6, 4, wm8750_mono_mux), /* 16 */
];

static wm8750_snd_controls: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_R!("Capture Volume", WM8750_LINVOL, WM8750_RINVOL, 0, 63, 0),
    SOC_DOUBLE_R!("Capture ZC Switch", WM8750_LINVOL, WM8750_RINVOL, 6, 1, 0),
    SOC_DOUBLE_R!("Capture Switch", WM8750_LINVOL, WM8750_RINVOL, 7, 1, 1),
    SOC_DOUBLE_R!("Headphone Playback ZC Switch", WM8750_LOUT1V, WM8750_ROUT1V, 7, 1, 0),
    SOC_DOUBLE_R!("Speaker Playback ZC Switch", WM8750_LOUT2V, WM8750_ROUT2V, 7, 1, 0),
    SOC_ENUM!("Playback De-emphasis", wm8750_enum[15]),
    SOC_ENUM!("Capture Polarity", wm8750_enum[14]),
    SOC_SINGLE!("Playback 6dB Attenuate", WM8750_ADCDAC, 7, 1, 0),
    SOC_SINGLE!("Capture 6dB Attenuate", WM8750_ADCDAC, 8, 1, 0),
    SOC_DOUBLE_R!("PCM Volume", WM8750_LDAC, WM8750_RDAC, 0, 255, 0),
    SOC_ENUM!("Bass Boost", wm8750_enum[0]),
    SOC_ENUM!("Bass Filter", wm8750_enum[1]),
    SOC_SINGLE!("Bass Volume", WM8750_BASS, 0, 15, 1),
    SOC_SINGLE!("Treble Volume", WM8750_TREBLE, 0, 15, 1),
    SOC_ENUM!("Treble Cut-off", wm8750_enum[2]),
    SOC_SINGLE!("3D Switch", WM8750_3D, 0, 1, 0),
    SOC_SINGLE!("3D Volume", WM8750_3D, 1, 15, 0),
    SOC_ENUM!("3D Lower Cut-off", wm8750_enum[3]),
    SOC_ENUM!("3D Upper Cut-off", wm8750_enum[4]),
    SOC_ENUM!("3D Mode", wm8750_enum[5]),
    SOC_SINGLE!("ALC Capture Target Volume", WM8750_ALC1, 0, 7, 0),
    SOC_SINGLE!("ALC Capture Max Volume", WM8750_ALC1, 4, 7, 0),
    SOC_ENUM!("ALC Capture Function", wm8750_enum[6]),
    SOC_SINGLE!("ALC Capture ZC Switch", WM8750_ALC2, 7, 1, 0),
    SOC_SINGLE!("ALC Capture Hold Time", WM8750_ALC2, 0, 15, 0),
    SOC_SINGLE!("ALC Capture Decay Time", WM8750_ALC3, 4, 15, 0),
    SOC_SINGLE!("ALC Capture Attack Time", WM8750_ALC3, 0, 15, 0),
    SOC_SINGLE!("ALC Capture NG Threshold", WM8750_NGATE, 3, 31, 0),
    SOC_ENUM!("ALC Capture NG Type", wm8750_enum[4]),
    SOC_SINGLE!("ALC Capture NG Switch", WM8750_NGATE, 0, 1, 0),
    SOC_SINGLE!("Left ADC Capture Volume", WM8750_LADC, 0, 255, 0),
    SOC_SINGLE!("Right ADC Capture Volume", WM8750_RADC, 0, 255, 0),
    SOC_SINGLE!("ZC Timeout Switch", WM8750_ADCTL1, 0, 1, 0),
    SOC_SINGLE!("Playback Invert Switch", WM8750_ADCTL1, 1, 1, 0),
    SOC_SINGLE!("Right Speaker Playback Invert Switch", WM8750_ADCTL2, 4, 1, 0),
    /* Unimplemented */
    /* ADCDAC Bit 0 - ADCHPD */
    /* ADCDAC Bit 4 - HPOR */
    /* ADCTL1 Bit 2,3 - DATSEL */
    /* ADCTL1 Bit 4,5 - DMONOMIX */
    /* ADCTL1 Bit 6,7 - VSEL */
    /* ADCTL2 Bit 2 - LRCM */
    /* ADCTL2 Bit 3 - TRI */
    /* ADCTL3 Bit 5 - HPFLREN */
    /* ADCTL3 Bit 6 - VROI */
    /* ADCTL3 Bit 7,8 - ADCLRM */
    /* ADCIN Bit 4 - LDCM */
    /* ADCIN Bit 5 - RDCM */
    SOC_DOUBLE_R!("Mic Boost", WM8750_LADCIN, WM8750_RADCIN, 4, 3, 0),
    SOC_DOUBLE_R!("Bypass Left Playback Volume", WM8750_LOUTM1, WM8750_LOUTM2, 4, 7, 1),
    SOC_DOUBLE_R!("Bypass Right Playback Volume", WM8750_ROUTM1, WM8750_ROUTM2, 4, 7, 1),
    SOC_DOUBLE_R!("Bypass Mono Playback Volume", WM8750_MOUTM1, WM8750_MOUTM2, 4, 7, 1),
    SOC_SINGLE!("Mono Playback ZC Switch", WM8750_MOUTV, 7, 1, 0),
    SOC_DOUBLE_R!("Headphone Playback Volume", WM8750_LOUT1V, WM8750_ROUT1V, 0, 127, 0),
    SOC_DOUBLE_R!("Speaker Playback Volume", WM8750_LOUT2V, WM8750_ROUT2V, 0, 127, 0),
    SOC_SINGLE!("Mono Playback Volume", WM8750_MOUTV, 0, 127, 0),
];

/*
 * DAPM Controls
 */
static wm8750_left_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Playback Switch", WM8750_LOUTM1, 8, 1, 0),
    SOC_DAPM_SINGLE!("Left Bypass Switch", WM8750_LOUTM1, 7, 1, 0),
    SOC_DAPM_SINGLE!("Right Playback Switch", WM8750_LOUTM2, 8, 1, 0),
    SOC_DAPM_SINGLE!("Right Bypass Switch", WM8750_LOUTM2, 7, 1, 0),
];
static wm8750_right_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Left Playback Switch", WM8750_ROUTM1, 8, 1, 0),
    SOC_DAPM_SINGLE!("Left Bypass Switch", WM8750_ROUTM1, 7, 1, 0),
    SOC_DAPM_SINGLE!("Playback Switch", WM8750_ROUTM2, 8, 1, 0),
    SOC_DAPM_SINGLE!("Right Bypass Switch", WM8750_ROUTM2, 7, 1, 0),
];
static wm8750_mono_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Left Playback Switch", WM8750_MOUTM1, 8, 1, 0),
    SOC_DAPM_SINGLE!("Left Bypass Switch", WM8750_MOUTM1, 7, 1, 0),
    SOC_DAPM_SINGLE!("Right Playback Switch", WM8750_MOUTM2, 8, 1, 0),
    SOC_DAPM_SINGLE!("Right Bypass Switch", WM8750_MOUTM2, 7, 1, 0),
];

static wm8750_left_line_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8750_enum[8]);
static wm8750_right_line_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8750_enum[9]);
static wm8750_left_pga_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8750_enum[10]);
static wm8750_right_pga_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8750_enum[11]);
static wm8750_out3_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8750_enum[12]);
static wm8750_diffmux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8750_enum[13]);
static wm8750_monomux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8750_enum[16]);

static wm8750_dapm_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_MIXER!("Left Mixer", SND_SOC_NOPM, 0, 0, &wm8750_left_mixer_controls[0], ARRAY_SIZE!(wm8750_left_mixer_controls)),
    SND_SOC_DAPM_MIXER!("Right Mixer", SND_SOC_NOPM, 0, 0, &wm8750_right_mixer_controls[0], ARRAY_SIZE!(wm8750_right_mixer_controls)),
    SND_SOC_DAPM_MIXER!("Mono Mixer", WM8750_PWR2, 2, 0, &wm8750_mono_mixer_controls[0], ARRAY_SIZE!(wm8750_mono_mixer_controls)),
    SND_SOC_DAPM_PGA!("Right Out 2", WM8750_PWR2, 3, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Left Out 2", WM8750_PWR2, 4, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Right Out 1", WM8750_PWR2, 5, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Left Out 1", WM8750_PWR2, 6, 0, NULL, 0),
    SND_SOC_DAPM_DAC!("Right DAC", "Right Playback", WM8750_PWR2, 7, 0),
    SND_SOC_DAPM_DAC!("Left DAC", "Left Playback", WM8750_PWR2, 8, 0),
    SND_SOC_DAPM_MICBIAS!("Mic Bias", WM8750_PWR1, 1, 0),
    SND_SOC_DAPM_ADC!("Right ADC", "Right Capture", WM8750_PWR1, 2, 0),
    SND_SOC_DAPM_ADC!("Left ADC", "Left Capture", WM8750_PWR1, 3, 0),
    SND_SOC_DAPM_MUX!("Left PGA Mux", WM8750_PWR1, 5, 0, &wm8750_left_pga_controls),
    SND_SOC_DAPM_MUX!("Right PGA Mux", WM8750_PWR1, 4, 0, &wm8750_right_pga_controls),
    SND_SOC_DAPM_MUX!("Left Line Mux", SND_SOC_NOPM, 0, 0, &wm8750_left_line_controls),
    SND_SOC_DAPM_MUX!("Right Line Mux", SND_SOC_NOPM, 0, 0, &wm8750_right_line_controls),
    SND_SOC_DAPM_MUX!("Out3 Mux", SND_SOC_NOPM, 0, 0, &wm8750_out3_controls),
    SND_SOC_DAPM_PGA!("Out 3", WM8750_PWR2, 1, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Mono Out 1", WM8750_PWR2, 2, 0, NULL, 0),
    SND_SOC_DAPM_MUX!("Differential Mux", SND_SOC_NOPM, 0, 0, &wm8750_diffmux_controls),
    SND_SOC_DAPM_MUX!("Left ADC Mux", SND_SOC_NOPM, 0, 0, &wm8750_monomux_controls),
    SND_SOC_DAPM_MUX!("Right ADC Mux", SND_SOC_NOPM, 0, 0, &wm8750_monomux_controls),
    SND_SOC_DAPM_OUTPUT!("LOUT1"),
    SND_SOC_DAPM_OUTPUT!("ROUT1"),
    SND_SOC_DAPM_OUTPUT!("LOUT2"),
    SND_SOC_DAPM_OUTPUT!("ROUT2"),
    SND_SOC_DAPM_OUTPUT!("MONO1"),
    SND_SOC_DAPM_OUTPUT!("OUT3"),
    SND_SOC_DAPM_VMID!("VREF"),
    SND_SOC_DAPM_INPUT!("LINPUT1"),
    SND_SOC_DAPM_INPUT!("LINPUT2"),
    SND_SOC_DAPM_INPUT!("LINPUT3"),
    SND_SOC_DAPM_INPUT!("RINPUT1"),
    SND_SOC_DAPM_INPUT!("RINPUT2"),
    SND_SOC_DAPM_INPUT!("RINPUT3"),
];

static wm8750_dapm_routes: &[snd_soc_dapm_route] = &[
    dapm_route!("Left Mixer", "Playback Switch", "Left DAC"),
    dapm_route!("Left Mixer", "Left Bypass Switch", "Left Line Mux"),
    dapm_route!("Left Mixer", "Right Playback Switch", "Right DAC"),
    dapm_route!("Left Mixer", "Right Bypass Switch", "Right Line Mux"),
    dapm_route!("Right Mixer", "Left Playback Switch", "Left DAC"),
    dapm_route!("Right Mixer", "Left Bypass Switch", "Left Line Mux"),
    dapm_route!("Right Mixer", "Playback Switch", "Right DAC"),
    dapm_route!("Right Mixer", "Right Bypass Switch", "Right Line Mux"),
    dapm_route!("Left Out 1", NULL, "Left Mixer"),
    dapm_route!("LOUT1", NULL, "Left Out 1"),
    dapm_route!("Left Out 2", NULL, "Left Mixer"),
    dapm_route!("LOUT2", NULL, "Left Out 2"),
    dapm_route!("Right Out 1", NULL, "Right Mixer"),
    dapm_route!("ROUT1", NULL, "Right Out 1"),
    dapm_route!("Right Out 2", NULL, "Right Mixer"),
    dapm_route!("ROUT2", NULL, "Right Out 2"),
    dapm_route!("Mono Mixer", "Left Playback Switch", "Left DAC"),
    dapm_route!("Mono Mixer", "Left Bypass Switch", "Left Line Mux"),
    dapm_route!("Mono Mixer", "Right Playback Switch", "Right DAC"),
    dapm_route!("Mono Mixer", "Right Bypass Switch", "Right Line Mux"),
    dapm_route!("Mono Out 1", NULL, "Mono Mixer"),
    dapm_route!("MONO1", NULL, "Mono Out 1"),
    dapm_route!("Out3 Mux", "VREF", "VREF"),
    dapm_route!("Out3 Mux", "ROUT1 + Vol", "ROUT1"),
    dapm_route!("Out3 Mux", "ROUT1", "Right Mixer"),
    dapm_route!("Out3 Mux", "MonoOut", "MONO1"),
    dapm_route!("Out 3", NULL, "Out3 Mux"),
    dapm_route!("OUT3", NULL, "Out 3"),
    dapm_route!("Left Line Mux", "Line 1", "LINPUT1"),
    dapm_route!("Left Line Mux", "Line 2", "LINPUT2"),
    dapm_route!("Left Line Mux", "Line 3", "LINPUT3"),
    dapm_route!("Left Line Mux", "PGA", "Left PGA Mux"),
    dapm_route!("Left Line Mux", "Differential", "Differential Mux"),
    dapm_route!("Right Line Mux", "Line 1", "RINPUT1"),
    dapm_route!("Right Line Mux", "Line 2", "RINPUT2"),
    dapm_route!("Right Line Mux", "Line 3", "RINPUT3"),
    dapm_route!("Right Line Mux", "PGA", "Right PGA Mux"),
    dapm_route!("Right Line Mux", "Differential", "Differential Mux"),
    dapm_route!("Left PGA Mux", "Line 1", "LINPUT1"),
    dapm_route!("Left PGA Mux", "Line 2", "LINPUT2"),
    dapm_route!("Left PGA Mux", "Line 3", "LINPUT3"),
    dapm_route!("Left PGA Mux", "Differential", "Differential Mux"),
    dapm_route!("Right PGA Mux", "Line 1", "RINPUT1"),
    dapm_route!("Right PGA Mux", "Line 2", "RINPUT2"),
    dapm_route!("Right PGA Mux", "Line 3", "RINPUT3"),
    dapm_route!("Right PGA Mux", "Differential", "Differential Mux"),
    dapm_route!("Differential Mux", "Line 1", "LINPUT1"),
    dapm_route!("Differential Mux", "Line 1", "RINPUT1"),
    dapm_route!("Differential Mux", "Line 2", "LINPUT2"),
    dapm_route!("Differential Mux", "Line 2", "RINPUT2"),
    dapm_route!("Left ADC Mux", "Stereo", "Left PGA Mux"),
    dapm_route!("Left ADC Mux", "Mono (Left)", "Left PGA Mux"),
    dapm_route!("Left ADC Mux", "Digital Mono", "Left PGA Mux"),
    dapm_route!("Right ADC Mux", "Stereo", "Right PGA Mux"),
    dapm_route!("Right ADC Mux", "Mono (Right)", "Right PGA Mux"),
    dapm_route!("Right ADC Mux", "Digital Mono", "Right PGA Mux"),
    dapm_route!("Left ADC", NULL, "Left ADC Mux"),
    dapm_route!("Right ADC", NULL, "Right ADC Mux"),
];

/* codec hifi mclk clock divider coefficients */
static coeff_div: [_coeff_div; 31] = [
    _coeff_div::new(12288000, 8000, 1536, 0x6, 0x0),
    _coeff_div::new(11289600, 8000, 1408, 0x16, 0x0),
    _coeff_div::new(18432000, 8000, 2304, 0x7, 0x0),
    _coeff_div::new(16934400, 8000, 2112, 0x17, 0x0),
    _coeff_div::new(12000000, 8000, 1500, 0x6, 0x1),
    _coeff_div::new(11289600, 11025, 1024, 0x18, 0x0),
    _coeff_div::new(16934400, 11025, 1536, 0x19, 0x0),
    _coeff_div::new(12000000, 11025, 1088, 0x19, 0x1),
    _coeff_div::new(12288000, 16000, 768, 0xa, 0x0),
    _coeff_div::new(18432000, 16000, 1152, 0xb, 0x0),
    _coeff_div::new(12000000, 16000, 750, 0xa, 0x1),
    _coeff_div::new(11289600, 22050, 512, 0x1a, 0x0),
    _coeff_div::new(16934400, 22050, 768, 0x1b, 0x0),
    _coeff_div::new(12000000, 22050, 544, 0x1b, 0x1),
    _coeff_div::new(12288000, 32000, 384, 0xc, 0x0),
    _coeff_div::new(18432000, 32000, 576, 0xd, 0x0),
    _coeff_div::new(12000000, 32000, 375, 0xa, 0x1),
    _coeff_div::new(11289600, 44100, 256, 0x10, 0x0),
    _coeff_div::new(16934400, 44100, 384, 0x11, 0x0),
    _coeff_div::new(12000000, 44100, 272, 0x11, 0x1),
    _coeff_div::new(12288000, 48000, 256, 0x0, 0x0),
    _coeff_div::new(18432000, 48000, 384, 0x1, 0x0),
    _coeff_div::new(12000000, 48000, 250, 0x0, 0x1),
    _coeff_div::new(11289600, 88200, 128, 0x1e, 0x0),
    _coeff_div::new(16934400, 88200, 192, 0x1f, 0x0),
    _coeff_div::new(12000000, 88200, 136, 0x1f, 0x1),
    _coeff_div::new(12288000, 96000, 128, 0xe, 0x0),
    _coeff_div::new(18432000, 96000, 192, 0xf, 0x0),
    _coeff_div::new(12000000, 96000, 125, 0xe, 0x1),
];

unsafe fn get_coeff(mclk: c_int, rate: c_int) -> c_int {
    let mut i: usize = 0;
    while i < coeff_div.len() {
        if coeff_div[i].rate == rate as u32 && coeff_div[i].mclk == mclk as u32 {
            return i as c_int;
        }
        i += 1;
    }

    printk!(KERN_ERR, "wm8750: could not get coeff for mclk %d @ rate %d\n", mclk, rate);
    -EINVAL
}

unsafe fn wm8750_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let wm8750: *mut wm8750_priv = snd_soc_component_get_drvdata(component) as *mut wm8750_priv;

    match freq {
        11289600 | 12000000 | 12288000 | 16934400 | 18432000 => {
            (*wm8750).sysclk = freq;
            0
        }
        _ => -EINVAL,
    }
}

unsafe fn wm8750_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
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

    snd_soc_component_write(component, WM8750_IFACE, iface as c_uint);
    0
}

unsafe fn wm8750_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let wm8750: *mut wm8750_priv = snd_soc_component_get_drvdata(component) as *mut wm8750_priv;
    let mut iface: u16 = (snd_soc_component_read(component, WM8750_IFACE) & 0x1f3) as u16;
    let srate: u16 = (snd_soc_component_read(component, WM8750_SRATE) & 0x1c0) as u16;
    let coeff: c_int = get_coeff((*wm8750).sysclk as c_int, params_rate(params));

    /* bit size */
    match params_width(params) {
        16 => {}
        20 => iface |= 0x0004,
        24 => iface |= 0x0008,
        32 => iface |= 0x000c,
        _ => {}
    }

    /* set iface & srate */
    snd_soc_component_write(component, WM8750_IFACE, iface as c_uint);
    if coeff >= 0 {
        let c = coeff as usize;
        snd_soc_component_write(
            component,
            WM8750_SRATE,
            (srate | ((coeff_div[c].sr() as u16) << 1) | coeff_div[c].usb() as u16) as c_uint,
        );
    }

    0
}

unsafe fn wm8750_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mute_reg: u16 = (snd_soc_component_read(component, WM8750_ADCDAC) & 0xfff7) as u16;

    if mute != 0 {
        snd_soc_component_write(component, WM8750_ADCDAC, (mute_reg | 0x8) as c_uint);
    } else {
        snd_soc_component_write(component, WM8750_ADCDAC, mute_reg as c_uint);
    }
    0
}

unsafe fn wm8750_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let pwr_reg: u16 = (snd_soc_component_read(component, WM8750_PWR1) & 0xfe3e) as u16;

    match level {
        SND_SOC_BIAS_ON => {
            /* set vmid to 50k and unmute dac */
            snd_soc_component_write(component, WM8750_PWR1, (pwr_reg | 0x00c0) as c_uint);
        }
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                snd_soc_component_cache_sync(component);

                /* Set VMID to 5k */
                snd_soc_component_write(component, WM8750_PWR1, (pwr_reg | 0x01c1) as c_uint);

                /* ...and ramp */
                msleep(1000);
            }

            /* mute dac and set vmid to 500k, enable VREF */
            snd_soc_component_write(component, WM8750_PWR1, (pwr_reg | 0x0141) as c_uint);
        }
        SND_SOC_BIAS_OFF => {
            snd_soc_component_write(component, WM8750_PWR1, 0x0001);
        }
    }
    0
}

const WM8750_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_11025
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000;

const WM8750_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static wm8750_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8750_pcm_hw_params),
    mute_stream: Some(wm8750_mute),
    set_fmt: Some(wm8750_set_dai_fmt),
    set_sysclk: Some(wm8750_set_dai_sysclk),
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

static mut wm8750_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("wm8750-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("Playback"),
        channels_min: 1,
        channels_max: 2,
        rates: WM8750_RATES,
        formats: WM8750_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    capture: snd_soc_pcm_stream {
        stream_name: cstr!("Capture"),
        channels_min: 1,
        channels_max: 2,
        rates: WM8750_RATES,
        formats: WM8750_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &wm8750_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn wm8750_probe(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;

    ret = wm8750_reset!(component);
    if ret < 0 {
        printk!(KERN_ERR, "wm8750: failed to reset: %d\n", ret);
        return ret;
    }

    /* set the update bits */
    snd_soc_component_update_bits(component, WM8750_LDAC, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8750_RDAC, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8750_LOUT1V, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8750_ROUT1V, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8750_LOUT2V, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8750_ROUT2V, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8750_LINVOL, 0x0100, 0x0100);
    snd_soc_component_update_bits(component, WM8750_RINVOL, 0x0100, 0x0100);

    ret
}

static soc_component_dev_wm8750: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8750_probe),
    set_bias_level: Some(wm8750_set_bias_level),
    controls: wm8750_snd_controls.as_ptr(),
    num_controls: wm8750_snd_controls.len() as c_uint,
    dapm_widgets: wm8750_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8750_dapm_widgets.len() as c_uint,
    dapm_routes: wm8750_dapm_routes.as_ptr(),
    num_dapm_routes: wm8750_dapm_routes.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

static wm8750_of_match: [of_device_id; 3] = [
    of_device_id { compatible: cstr!("wlf,wm8750"), ..unsafe { core::mem::zeroed() } },
    of_device_id { compatible: cstr!("wlf,wm8987"), ..unsafe { core::mem::zeroed() } },
    of_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(of, wm8750_of_match);

static wm8750_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: WM8750_MOUTV,
    reg_defaults: wm8750_reg_defaults.as_ptr(),
    num_reg_defaults: wm8750_reg_defaults.len() as c_uint,
    cache_type: REGCACHE_MAPLE,
    ..unsafe { core::mem::zeroed() }
};

/* Original C condition: #if defined(CONFIG_SPI_MASTER) */
unsafe fn wm8750_spi_probe(spi: *mut spi_device) -> c_int {
    let wm8750: *mut wm8750_priv;
    let regmap: *mut regmap;
    let ret: c_int;

    wm8750 = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<wm8750_priv>(), GFP_KERNEL)
        as *mut wm8750_priv;
    if wm8750.is_null() {
        return -ENOMEM;
    }

    regmap = devm_regmap_init_spi(spi, &wm8750_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    spi_set_drvdata(spi, wm8750 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &soc_component_dev_wm8750,
        &raw mut wm8750_dai,
        1,
    );
    ret
}

static wm8750_spi_ids: [spi_device_id; 3] = [
    spi_device_id { name: cstr!("wm8750"), driver_data: 0 },
    spi_device_id { name: cstr!("wm8987"), driver_data: 0 },
    spi_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(spi, wm8750_spi_ids);

static mut wm8750_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: cstr!("wm8750"),
        of_match_table: wm8750_of_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    id_table: wm8750_spi_ids.as_ptr(),
    probe: Some(wm8750_spi_probe),
    ..unsafe { core::mem::zeroed() }
};

/* Original C condition: #if IS_ENABLED(CONFIG_I2C) */
unsafe fn wm8750_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8750: *mut wm8750_priv;
    let regmap: *mut regmap;
    let ret: c_int;

    wm8750 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8750_priv>(), GFP_KERNEL)
        as *mut wm8750_priv;
    if wm8750.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, wm8750 as *mut c_void);

    regmap = devm_regmap_init_i2c(i2c, &wm8750_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm8750,
        &raw mut wm8750_dai,
        1,
    );
    ret
}

static wm8750_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id { name: cstr!("wm8750"), ..unsafe { core::mem::zeroed() } },
    i2c_device_id { name: cstr!("wm8987"), ..unsafe { core::mem::zeroed() } },
    i2c_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(i2c, wm8750_i2c_id);

static mut wm8750_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: cstr!("wm8750"),
        of_match_table: wm8750_of_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(wm8750_i2c_probe),
    id_table: wm8750_i2c_id.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn wm8750_modinit() -> c_int {
    let mut ret: c_int = 0;

    /* Original C condition: #if IS_ENABLED(CONFIG_I2C) */
    ret = i2c_add_driver(&raw mut wm8750_i2c_driver);
    if ret != 0 {
        printk!(KERN_ERR, "Failed to register wm8750 I2C driver: %d\n", ret);
    }

    /* Original C condition: #if defined(CONFIG_SPI_MASTER) */
    ret = spi_register_driver(&raw mut wm8750_spi_driver);
    if ret != 0 {
        printk!(KERN_ERR, "Failed to register wm8750 SPI driver: %d\n", ret);
    }

    ret
}
module_init!(wm8750_modinit);

unsafe fn wm8750_exit() {
    /* Original C condition: #if IS_ENABLED(CONFIG_I2C) */
    i2c_del_driver(&raw mut wm8750_i2c_driver);

    /* Original C condition: #if defined(CONFIG_SPI_MASTER) */
    spi_unregister_driver(&raw mut wm8750_spi_driver);
}
module_exit!(wm8750_exit);

MODULE_DESCRIPTION!("ASoC WM8750 driver");
MODULE_AUTHOR!("Liam Girdwood");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
