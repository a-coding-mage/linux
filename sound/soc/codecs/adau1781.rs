// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for ADAU1381/ADAU1781 codec
 *
 * Copyright 2011-2013 Analog Devices Inc.
 * Author: Lars-Peter Clausen <lars@metafoo.de>
 */

use core::ffi::{c_char, c_int, c_uint};

/* Dependencies from the original C includes:
 * linux/module.h, linux/init.h, linux/i2c.h, linux/spi/spi.h,
 * linux/slab.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
 * sound/soc.h, sound/tlv.h, linux/platform_data/adau17x1.h,
 * "adau17x1.h", and "adau1781.h".
 */

const ADAU1781_DMIC_BEEP_CTRL: c_uint = 0x4008;
const ADAU1781_LEFT_PGA: c_uint = 0x400e;
const ADAU1781_RIGHT_PGA: c_uint = 0x400f;
const ADAU1781_LEFT_PLAYBACK_MIXER: c_uint = 0x401c;
const ADAU1781_RIGHT_PLAYBACK_MIXER: c_uint = 0x401e;
const ADAU1781_MONO_PLAYBACK_MIXER: c_uint = 0x401f;
const ADAU1781_LEFT_LINEOUT: c_uint = 0x4025;
const ADAU1781_RIGHT_LINEOUT: c_uint = 0x4026;
const ADAU1781_SPEAKER: c_uint = 0x4027;
const ADAU1781_BEEP_ZC: c_uint = 0x4028;
const ADAU1781_DEJITTER: c_uint = 0x4032;
const ADAU1781_DIG_PWDN0: c_uint = 0x4080;
const ADAU1781_DIG_PWDN1: c_uint = 0x4081;

const ADAU1781_INPUT_DIFFERNTIAL: c_uint = BIT!(3);

const ADAU1381_FIRMWARE: *const c_char = b"adau1381.bin\0".as_ptr() as *const c_char;
const ADAU1781_FIRMWARE: *const c_char = b"adau1781.bin\0".as_ptr() as *const c_char;

static adau1781_reg_defaults: [reg_default; 37] = [
    reg_default { reg: ADAU1781_DMIC_BEEP_CTRL, def: 0x00 },
    reg_default { reg: ADAU1781_LEFT_PGA, def: 0xc7 },
    reg_default { reg: ADAU1781_RIGHT_PGA, def: 0xc7 },
    reg_default { reg: ADAU1781_LEFT_PLAYBACK_MIXER, def: 0x00 },
    reg_default { reg: ADAU1781_RIGHT_PLAYBACK_MIXER, def: 0x00 },
    reg_default { reg: ADAU1781_MONO_PLAYBACK_MIXER, def: 0x00 },
    reg_default { reg: ADAU1781_LEFT_LINEOUT, def: 0x00 },
    reg_default { reg: ADAU1781_RIGHT_LINEOUT, def: 0x00 },
    reg_default { reg: ADAU1781_SPEAKER, def: 0x00 },
    reg_default { reg: ADAU1781_BEEP_ZC, def: 0x19 },
    reg_default { reg: ADAU1781_DEJITTER, def: 0x60 },
    reg_default { reg: ADAU1781_DIG_PWDN1, def: 0x0c },
    reg_default { reg: ADAU1781_DIG_PWDN1, def: 0x00 },
    reg_default { reg: ADAU17X1_CLOCK_CONTROL, def: 0x00 },
    reg_default { reg: ADAU17X1_PLL_CONTROL, def: 0x00 },
    reg_default { reg: ADAU17X1_REC_POWER_MGMT, def: 0x00 },
    reg_default { reg: ADAU17X1_MICBIAS, def: 0x04 },
    reg_default { reg: ADAU17X1_SERIAL_PORT0, def: 0x00 },
    reg_default { reg: ADAU17X1_SERIAL_PORT1, def: 0x00 },
    reg_default { reg: ADAU17X1_CONVERTER0, def: 0x00 },
    reg_default { reg: ADAU17X1_CONVERTER1, def: 0x00 },
    reg_default { reg: ADAU17X1_LEFT_INPUT_DIGITAL_VOL, def: 0x00 },
    reg_default { reg: ADAU17X1_RIGHT_INPUT_DIGITAL_VOL, def: 0x00 },
    reg_default { reg: ADAU17X1_ADC_CONTROL, def: 0x00 },
    reg_default { reg: ADAU17X1_PLAY_POWER_MGMT, def: 0x00 },
    reg_default { reg: ADAU17X1_DAC_CONTROL0, def: 0x00 },
    reg_default { reg: ADAU17X1_DAC_CONTROL1, def: 0x00 },
    reg_default { reg: ADAU17X1_DAC_CONTROL2, def: 0x00 },
    reg_default { reg: ADAU17X1_SERIAL_PORT_PAD, def: 0x00 },
    reg_default { reg: ADAU17X1_CONTROL_PORT_PAD0, def: 0x00 },
    reg_default { reg: ADAU17X1_CONTROL_PORT_PAD1, def: 0x00 },
    reg_default { reg: ADAU17X1_DSP_SAMPLING_RATE, def: 0x01 },
    reg_default { reg: ADAU17X1_SERIAL_INPUT_ROUTE, def: 0x00 },
    reg_default { reg: ADAU17X1_SERIAL_OUTPUT_ROUTE, def: 0x00 },
    reg_default { reg: ADAU17X1_DSP_ENABLE, def: 0x00 },
    reg_default { reg: ADAU17X1_DSP_RUN, def: 0x00 },
    reg_default { reg: ADAU17X1_SERIAL_SAMPLING_RATE, def: 0x00 },
];

DECLARE_TLV_DB_SCALE!(adau1781_speaker_tlv, 0, 200, 0);

DECLARE_TLV_DB_RANGE!(
    adau1781_pga_tlv,
    0, 1, TLV_DB_SCALE_ITEM!(0, 600, 0),
    2, 3, TLV_DB_SCALE_ITEM!(1000, 400, 0),
    4, 4, TLV_DB_SCALE_ITEM!(1700, 0, 0),
    5, 7, TLV_DB_SCALE_ITEM!(2000, 600, 0)
);

DECLARE_TLV_DB_RANGE!(
    adau1781_beep_tlv,
    0, 1, TLV_DB_SCALE_ITEM!(0, 600, 0),
    2, 3, TLV_DB_SCALE_ITEM!(1000, 400, 0),
    4, 4, TLV_DB_SCALE_ITEM!(-2300, 0, 0),
    5, 7, TLV_DB_SCALE_ITEM!(2000, 600, 0)
);

DECLARE_TLV_DB_SCALE!(adau1781_sidetone_tlv, -1800, 300, 1);

static adau1781_speaker_bias_select_text: [*const c_char; 3] = [
    b"Normal operation\0".as_ptr() as *const c_char,
    b"Power saving\0".as_ptr() as *const c_char,
    b"Enhanced performance\0".as_ptr() as *const c_char,
];

static adau1781_bias_select_text: [*const c_char; 4] = [
    b"Normal operation\0".as_ptr() as *const c_char,
    b"Extreme power saving\0".as_ptr() as *const c_char,
    b"Power saving\0".as_ptr() as *const c_char,
    b"Enhanced performance\0".as_ptr() as *const c_char,
];

SOC_ENUM_SINGLE_DECL!(adau1781_adc_bias_enum, ADAU17X1_REC_POWER_MGMT, 3, adau1781_bias_select_text);
SOC_ENUM_SINGLE_DECL!(adau1781_speaker_bias_enum, ADAU17X1_PLAY_POWER_MGMT, 6, adau1781_speaker_bias_select_text);
SOC_ENUM_SINGLE_DECL!(adau1781_dac_bias_enum, ADAU17X1_PLAY_POWER_MGMT, 4, adau1781_bias_select_text);
SOC_ENUM_SINGLE_DECL!(adau1781_playback_bias_enum, ADAU17X1_PLAY_POWER_MGMT, 2, adau1781_bias_select_text);
SOC_ENUM_SINGLE_DECL!(adau1781_capture_bias_enum, ADAU17X1_REC_POWER_MGMT, 1, adau1781_bias_select_text);

static adau1781_controls: [snd_kcontrol_new; 12] = [
    SOC_SINGLE_TLV!(b"Beep Capture Volume\0", ADAU1781_DMIC_BEEP_CTRL, 0, 7, 0, adau1781_beep_tlv),
    SOC_DOUBLE_R_TLV!(b"PGA Capture Volume\0", ADAU1781_LEFT_PGA, ADAU1781_RIGHT_PGA, 5, 7, 0, adau1781_pga_tlv),
    SOC_DOUBLE_R!(b"PGA Capture Switch\0", ADAU1781_LEFT_PGA, ADAU1781_RIGHT_PGA, 1, 1, 0),
    SOC_DOUBLE_R!(b"Lineout Playback Switch\0", ADAU1781_LEFT_LINEOUT, ADAU1781_RIGHT_LINEOUT, 1, 1, 0),
    SOC_SINGLE!(b"Beep ZC Switch\0", ADAU1781_BEEP_ZC, 0, 1, 0),
    SOC_SINGLE!(b"Mono Playback Switch\0", ADAU1781_MONO_PLAYBACK_MIXER, 0, 1, 0),
    SOC_SINGLE_TLV!(b"Mono Playback Volume\0", ADAU1781_SPEAKER, 6, 3, 0, adau1781_speaker_tlv),
    SOC_ENUM!(b"ADC Bias\0", adau1781_adc_bias_enum),
    SOC_ENUM!(b"DAC Bias\0", adau1781_dac_bias_enum),
    SOC_ENUM!(b"Capture Bias\0", adau1781_capture_bias_enum),
    SOC_ENUM!(b"Playback Bias\0", adau1781_playback_bias_enum),
    SOC_ENUM!(b"Speaker Bias\0", adau1781_speaker_bias_enum),
];

static adau1781_beep_mixer_controls: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE!(b"Beep Capture Switch\0", ADAU1781_DMIC_BEEP_CTRL, 3, 1, 0),
];

static adau1781_left_mixer_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!(b"Switch\0", ADAU1781_LEFT_PLAYBACK_MIXER, 5, 1, 0),
    SOC_DAPM_SINGLE_TLV!(b"Beep Playback Volume\0", ADAU1781_LEFT_PLAYBACK_MIXER, 1, 8, 0, adau1781_sidetone_tlv),
];

static adau1781_right_mixer_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!(b"Switch\0", ADAU1781_RIGHT_PLAYBACK_MIXER, 6, 1, 0),
    SOC_DAPM_SINGLE_TLV!(b"Beep Playback Volume\0", ADAU1781_LEFT_PLAYBACK_MIXER, 1, 8, 0, adau1781_sidetone_tlv),
];

static adau1781_mono_mixer_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE_AUTODISABLE!(b"Left Switch\0", ADAU1781_MONO_PLAYBACK_MIXER, 7, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!(b"Right Switch\0", ADAU1781_MONO_PLAYBACK_MIXER, 6, 1, 0),
    SOC_DAPM_SINGLE_TLV!(b"Beep Playback Volume\0", ADAU1781_MONO_PLAYBACK_MIXER, 2, 8, 0, adau1781_sidetone_tlv),
];

unsafe extern "C" fn adau1781_dejitter_fixup(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    _event: c_int,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let adau: *mut adau = snd_soc_component_get_drvdata(component) as *mut adau;

    /* After any power changes have been made the dejitter circuit
     * has to be reinitialized. */
    regmap_write((*adau).regmap, ADAU1781_DEJITTER, 0);
    if !(*adau).master {
        regmap_write((*adau).regmap, ADAU1781_DEJITTER, 5);
    }

    0
}

static adau1781_dapm_widgets: [snd_soc_dapm_widget; 24] = [
    SND_SOC_DAPM_PGA!(b"Left PGA\0", ADAU1781_LEFT_PGA, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!(b"Right PGA\0", ADAU1781_RIGHT_PGA, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_OUT_DRV!(b"Speaker\0", ADAU1781_SPEAKER, 0, 0, core::ptr::null(), 0),
    SOC_MIXER_NAMED_CTL_ARRAY!(b"Beep Mixer\0", ADAU17X1_MICBIAS, 4, 0, adau1781_beep_mixer_controls),
    SOC_MIXER_ARRAY!(b"Left Lineout Mixer\0", SND_SOC_NOPM, 0, 0, adau1781_left_mixer_controls),
    SOC_MIXER_ARRAY!(b"Right Lineout Mixer\0", SND_SOC_NOPM, 0, 0, adau1781_right_mixer_controls),
    SOC_MIXER_ARRAY!(b"Mono Mixer\0", SND_SOC_NOPM, 0, 0, adau1781_mono_mixer_controls),
    SND_SOC_DAPM_SUPPLY!(b"Serial Input Routing\0", ADAU1781_DIG_PWDN0, 2, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(b"Serial Output Routing\0", ADAU1781_DIG_PWDN0, 3, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(b"Clock Domain Transfer\0", ADAU1781_DIG_PWDN0, 5, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(b"Serial Ports\0", ADAU1781_DIG_PWDN0, 4, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(b"ADC Engine\0", ADAU1781_DIG_PWDN0, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(b"DAC Engine\0", ADAU1781_DIG_PWDN1, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(b"Digital Mic\0", ADAU1781_DIG_PWDN1, 1, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(b"Sound Engine\0", ADAU1781_DIG_PWDN0, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!(b"SYSCLK\0", 1, ADAU1781_DIG_PWDN0, 1, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!(b"Zero Crossing Detector\0", ADAU1781_DIG_PWDN1, 2, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_POST!(b"Dejitter fixup\0", adau1781_dejitter_fixup),
    SND_SOC_DAPM_INPUT!(b"BEEP\0"),
    SND_SOC_DAPM_OUTPUT!(b"AOUTL\0"),
    SND_SOC_DAPM_OUTPUT!(b"AOUTR\0"),
    SND_SOC_DAPM_OUTPUT!(b"SP\0"),
    SND_SOC_DAPM_INPUT!(b"LMIC\0"),
    SND_SOC_DAPM_INPUT!(b"RMIC\0"),
];

static adau1781_dapm_routes: [snd_soc_dapm_route; 31] = [
    snd_soc_dapm_route { sink: b"Left Lineout Mixer\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Left Playback Enable\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Lineout Mixer\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Right Playback Enable\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left Lineout Mixer\0".as_ptr() as *const c_char, control: b"Beep Playback Volume\0".as_ptr() as *const c_char, source: b"Beep Mixer\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left Lineout Mixer\0".as_ptr() as *const c_char, control: b"Switch\0".as_ptr() as *const c_char, source: b"Left DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Lineout Mixer\0".as_ptr() as *const c_char, control: b"Beep Playback Volume\0".as_ptr() as *const c_char, source: b"Beep Mixer\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Lineout Mixer\0".as_ptr() as *const c_char, control: b"Switch\0".as_ptr() as *const c_char, source: b"Right DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Mono Mixer\0".as_ptr() as *const c_char, control: b"Beep Playback Volume\0".as_ptr() as *const c_char, source: b"Beep Mixer\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Mono Mixer\0".as_ptr() as *const c_char, control: b"Right Switch\0".as_ptr() as *const c_char, source: b"Right DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Mono Mixer\0".as_ptr() as *const c_char, control: b"Left Switch\0".as_ptr() as *const c_char, source: b"Left DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Mono Mixer\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Mono Mixer\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"SYSCLK\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left Lineout Mixer\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"SYSCLK\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left Lineout Mixer\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"SYSCLK\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Beep Mixer\0".as_ptr() as *const c_char, control: b"Beep Capture Switch\0".as_ptr() as *const c_char, source: b"BEEP\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Beep Mixer\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Zero Crossing Detector\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left DAC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DAC Engine\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right DAC\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DAC Engine\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Sound Engine\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"SYSCLK\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DSP\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Sound Engine\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left Decimator\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"ADC Engine\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Decimator\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"ADC Engine\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIFCLK\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"SYSCLK\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Playback\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Serial Input Routing\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Playback\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Serial Ports\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Playback\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Clock Domain Transfer\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Capture\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Serial Output Routing\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Capture\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Serial Ports\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Capture\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Clock Domain Transfer\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AOUTL\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Left Lineout Mixer\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AOUTR\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Right Lineout Mixer\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SP\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Speaker\0".as_ptr() as *const c_char },
];

static adau1781_adc_dapm_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"Left PGA\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"LMIC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right PGA\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"RMIC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left Decimator\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Left PGA\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Decimator\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Right PGA\0".as_ptr() as *const c_char },
];

static adau1781_dmic_select_text: [*const c_char; 2] = [
    b"DMIC1\0".as_ptr() as *const c_char,
    b"DMIC2\0".as_ptr() as *const c_char,
];

SOC_ENUM_SINGLE_VIRT_DECL!(adau1781_dmic_select_enum, adau1781_dmic_select_text);

static adau1781_dmic_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM!(b"DMIC Select\0", adau1781_dmic_select_enum);

static adau1781_dmic_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    SND_SOC_DAPM_MUX!(b"DMIC Select\0", SND_SOC_NOPM, 0, 0, &adau1781_dmic_mux),
    SND_SOC_DAPM_ADC!(b"DMIC1\0", core::ptr::null(), ADAU1781_DMIC_BEEP_CTRL, 4, 0),
    SND_SOC_DAPM_ADC!(b"DMIC2\0", core::ptr::null(), ADAU1781_DMIC_BEEP_CTRL, 5, 0),
];

static adau1781_dmic_dapm_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: b"DMIC1\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"LMIC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC2\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"RMIC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC1\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Digital Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC2\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Digital Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC Select\0".as_ptr() as *const c_char, control: b"DMIC1\0".as_ptr() as *const c_char, source: b"DMIC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC Select\0".as_ptr() as *const c_char, control: b"DMIC2\0".as_ptr() as *const c_char, source: b"DMIC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Left Decimator\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DMIC Select\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Decimator\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DMIC Select\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn adau1781_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let adau: *mut adau = snd_soc_component_get_drvdata(component) as *mut adau;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            regmap_update_bits(
                (*adau).regmap,
                ADAU17X1_CLOCK_CONTROL,
                ADAU17X1_CLOCK_CONTROL_SYSCLK_EN,
                ADAU17X1_CLOCK_CONTROL_SYSCLK_EN,
            );

            /* Precharge */
            regmap_update_bits((*adau).regmap, ADAU1781_DIG_PWDN1, 0x8, 0x8);
        }
        SND_SOC_BIAS_OFF => {
            regmap_update_bits((*adau).regmap, ADAU1781_DIG_PWDN1, 0xc, 0x0);
            regmap_update_bits(
                (*adau).regmap,
                ADAU17X1_CLOCK_CONTROL,
                ADAU17X1_CLOCK_CONTROL_SYSCLK_EN,
                0,
            );
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn adau1781_readable_register(
    dev: *mut device,
    reg: c_uint,
) -> bool {
    match reg {
        ADAU1781_DMIC_BEEP_CTRL
        | ADAU1781_LEFT_PGA
        | ADAU1781_RIGHT_PGA
        | ADAU1781_LEFT_PLAYBACK_MIXER
        | ADAU1781_RIGHT_PLAYBACK_MIXER
        | ADAU1781_MONO_PLAYBACK_MIXER
        | ADAU1781_LEFT_LINEOUT
        | ADAU1781_RIGHT_LINEOUT
        | ADAU1781_SPEAKER
        | ADAU1781_BEEP_ZC
        | ADAU1781_DEJITTER
        | ADAU1781_DIG_PWDN0
        | ADAU1781_DIG_PWDN1 => true,
        _ => adau17x1_readable_register(dev, reg),
    }
}

unsafe extern "C" fn adau1781_set_input_mode(
    adau: *mut adau,
    reg: c_uint,
    differential: bool,
) -> c_int {
    let val: c_uint;

    if differential {
        val = ADAU1781_INPUT_DIFFERNTIAL;
    } else {
        val = 0;
    }

    regmap_update_bits((*adau).regmap, reg, ADAU1781_INPUT_DIFFERNTIAL, val)
}

unsafe extern "C" fn adau1781_component_probe(
    component: *mut snd_soc_component,
) -> c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let pdata: *mut adau1781_platform_data = dev_get_platdata((*component).dev) as *mut adau1781_platform_data;
    let adau: *mut adau = snd_soc_component_get_drvdata(component) as *mut adau;
    let mut ret: c_int;

    ret = adau17x1_add_widgets(component);
    if ret != 0 {
        return ret;
    }

    if !pdata.is_null() {
        ret = adau1781_set_input_mode(adau, ADAU1781_LEFT_PGA, (*pdata).left_input_differential);
        if ret != 0 {
            return ret;
        }
        ret = adau1781_set_input_mode(adau, ADAU1781_RIGHT_PGA, (*pdata).right_input_differential);
        if ret != 0 {
            return ret;
        }
    }

    if !pdata.is_null() && (*pdata).use_dmic {
        ret = snd_soc_dapm_new_controls(
            dapm,
            adau1781_dmic_dapm_widgets.as_ptr(),
            ARRAY_SIZE!(adau1781_dmic_dapm_widgets),
        );
        if ret != 0 {
            return ret;
        }
        ret = snd_soc_dapm_add_routes(
            dapm,
            adau1781_dmic_dapm_routes.as_ptr(),
            ARRAY_SIZE!(adau1781_dmic_dapm_routes),
        );
        if ret != 0 {
            return ret;
        }
    } else {
        ret = snd_soc_dapm_add_routes(
            dapm,
            adau1781_adc_dapm_routes.as_ptr(),
            ARRAY_SIZE!(adau1781_adc_dapm_routes),
        );
        if ret != 0 {
            return ret;
        }
    }

    ret = adau17x1_add_routes(component);
    if ret < 0 {
        return ret;
    }

    0
}

static adau1781_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(adau1781_component_probe),
    resume: Some(adau17x1_resume),
    set_bias_level: Some(adau1781_set_bias_level),
    controls: adau1781_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(adau1781_controls),
    dapm_widgets: adau1781_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(adau1781_dapm_widgets),
    dapm_routes: adau1781_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(adau1781_dapm_routes),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

const ADAU1781_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut adau1781_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"adau-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: ADAU1781_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: ADAU1781_FORMATS,
    },
    ops: &adau17x1_dai_ops,
};

#[no_mangle]
pub static adau1781_regmap_config: regmap_config = regmap_config {
    val_bits: 8,
    reg_bits: 16,
    max_register: 0x40f8,
    reg_defaults: adau1781_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(adau1781_reg_defaults),
    readable_reg: Some(adau1781_readable_register),
    volatile_reg: Some(adau17x1_volatile_register),
    precious_reg: Some(adau17x1_precious_register),
    cache_type: REGCACHE_MAPLE,
};
EXPORT_SYMBOL_GPL!(adau1781_regmap_config);

#[no_mangle]
pub unsafe extern "C" fn adau1781_probe(
    dev: *mut device,
    regmap: *mut regmap,
    type_: adau17x1_type,
    switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>,
) -> c_int {
    let firmware_name: *const c_char;
    let mut ret: c_int;

    match type_ {
        ADAU1381 => {
            firmware_name = ADAU1381_FIRMWARE;
        }
        ADAU1781 => {
            firmware_name = ADAU1781_FIRMWARE;
        }
        _ => {
            return -EINVAL;
        }
    }

    ret = adau17x1_probe(dev, regmap, type_, switch_mode, firmware_name);
    if ret != 0 {
        return ret;
    }

    devm_snd_soc_register_component(dev, &adau1781_component_driver, &raw mut adau1781_dai_driver, 1)
}
EXPORT_SYMBOL_GPL!(adau1781_probe);

MODULE_DESCRIPTION!(b"ASoC ADAU1381/ADAU1781 driver\0");
MODULE_AUTHOR!(b"Lars-Peter Clausen <lars@metafoo.de>\0");
MODULE_LICENSE!(b"GPL\0");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
