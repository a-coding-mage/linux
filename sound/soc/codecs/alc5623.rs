// SPDX-License-Identifier: GPL-2.0-only
/*
 * alc5623.c  --  alc562[123] ALSA Soc Audio driver
 *
 * Copyright 2008 Realtek Microelectronics
 * Author: flove <flove@realtek.com> Ethan <eku@marvell.com>
 *
 * Copyright 2010 Arnaud Patard <arnaud.patard@rtp-net.org>
 *
 * Based on WM8753.c
 */

/* Rust translation of the original Linux ASoC implementation source.
 * Kernel, ASoC, regmap, I2C, OF, TLV, and codec register symbols are external
 * dependencies supplied by the surrounding repository.
 */

static mut caps_charge: c_int = 2000;
module_param!(caps_charge, int, 0);
MODULE_PARM_DESC!(caps_charge, "ALC5623 cap charge time (msecs)");

/* codec private data */
#[repr(C)]
struct alc5623_priv {
    regmap: *mut regmap,
    id: u8,
    sysclk: c_uint,
    add_ctrl: c_uint,
    jack_det_ctrl: c_uint,
}

#[inline]
unsafe fn alc5623_reset(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_write(component, ALC5623_RESET, 0)
}

unsafe fn amp_mixer_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    /* to power-on/off class-d amp generators/speaker */
    /* need to write to 'index-46h' register :        */
    /* so write index num (here 0x46) to reg 0x6a     */
    /* and then 0xffff/0 to reg 0x6c                  */
    snd_soc_component_write(component, ALC5623_HID_CTRL_INDEX, 0x46);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_write(component, ALC5623_HID_CTRL_DATA, 0xffff);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_write(component, ALC5623_HID_CTRL_DATA, 0);
        }
        _ => {}
    }

    0
}

/*
 * ALC5623 Controls
 */

static vol_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(-3450, 150, 0);
static hp_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(-4650, 150, 0);
static adc_rec_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(-1650, 150, 0);
static boost_tlv: &[c_uint] = DECLARE_TLV_DB_RANGE!(
    0, 0, TLV_DB_SCALE_ITEM!(0, 0, 0),
    1, 1, TLV_DB_SCALE_ITEM!(2000, 0, 0),
    2, 2, TLV_DB_SCALE_ITEM!(3000, 0, 0)
);
static dig_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(0, 600, 0);

static alc5621_vol_snd_controls: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_TLV!("Speaker Playback Volume", ALC5623_SPK_OUT_VOL, 8, 0, 31, 1, hp_tlv),
    SOC_DOUBLE!("Speaker Playback Switch", ALC5623_SPK_OUT_VOL, 15, 7, 1, 1),
    SOC_DOUBLE_TLV!("Headphone Playback Volume", ALC5623_HP_OUT_VOL, 8, 0, 31, 1, hp_tlv),
    SOC_DOUBLE!("Headphone Playback Switch", ALC5623_HP_OUT_VOL, 15, 7, 1, 1),
];

static alc5622_vol_snd_controls: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_TLV!("Speaker Playback Volume", ALC5623_SPK_OUT_VOL, 8, 0, 31, 1, hp_tlv),
    SOC_DOUBLE!("Speaker Playback Switch", ALC5623_SPK_OUT_VOL, 15, 7, 1, 1),
    SOC_DOUBLE_TLV!("Line Playback Volume", ALC5623_HP_OUT_VOL, 8, 0, 31, 1, hp_tlv),
    SOC_DOUBLE!("Line Playback Switch", ALC5623_HP_OUT_VOL, 15, 7, 1, 1),
];

static alc5623_vol_snd_controls: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_TLV!("Line Playback Volume", ALC5623_SPK_OUT_VOL, 8, 0, 31, 1, hp_tlv),
    SOC_DOUBLE!("Line Playback Switch", ALC5623_SPK_OUT_VOL, 15, 7, 1, 1),
    SOC_DOUBLE_TLV!("Headphone Playback Volume", ALC5623_HP_OUT_VOL, 8, 0, 31, 1, hp_tlv),
    SOC_DOUBLE!("Headphone Playback Switch", ALC5623_HP_OUT_VOL, 15, 7, 1, 1),
];

static alc5623_snd_controls: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_TLV!("Auxout Playback Volume", ALC5623_MONO_AUX_OUT_VOL, 8, 0, 31, 1, hp_tlv),
    SOC_DOUBLE!("Auxout Playback Switch", ALC5623_MONO_AUX_OUT_VOL, 15, 7, 1, 1),
    SOC_DOUBLE_TLV!("PCM Playback Volume", ALC5623_STEREO_DAC_VOL, 8, 0, 31, 1, vol_tlv),
    SOC_DOUBLE_TLV!("AuxI Capture Volume", ALC5623_AUXIN_VOL, 8, 0, 31, 1, vol_tlv),
    SOC_DOUBLE_TLV!("LineIn Capture Volume", ALC5623_LINE_IN_VOL, 8, 0, 31, 1, vol_tlv),
    SOC_SINGLE_TLV!("Mic1 Capture Volume", ALC5623_MIC_VOL, 8, 31, 1, vol_tlv),
    SOC_SINGLE_TLV!("Mic2 Capture Volume", ALC5623_MIC_VOL, 0, 31, 1, vol_tlv),
    SOC_DOUBLE_TLV!("Rec Capture Volume", ALC5623_ADC_REC_GAIN, 7, 0, 31, 0, adc_rec_tlv),
    SOC_SINGLE_TLV!("Mic 1 Boost Volume", ALC5623_MIC_CTRL, 10, 2, 0, boost_tlv),
    SOC_SINGLE_TLV!("Mic 2 Boost Volume", ALC5623_MIC_CTRL, 8, 2, 0, boost_tlv),
    SOC_SINGLE_TLV!("Digital Boost Volume", ALC5623_ADD_CTRL_REG, 4, 3, 0, dig_tlv),
];

/*
 * DAPM Controls
 */
static alc5623_hp_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("LI2HP Playback Switch", ALC5623_LINE_IN_VOL, 15, 1, 1),
    SOC_DAPM_SINGLE!("AUXI2HP Playback Switch", ALC5623_AUXIN_VOL, 15, 1, 1),
    SOC_DAPM_SINGLE!("MIC12HP Playback Switch", ALC5623_MIC_ROUTING_CTRL, 15, 1, 1),
    SOC_DAPM_SINGLE!("MIC22HP Playback Switch", ALC5623_MIC_ROUTING_CTRL, 7, 1, 1),
    SOC_DAPM_SINGLE!("DAC2HP Playback Switch", ALC5623_STEREO_DAC_VOL, 15, 1, 1),
];
static alc5623_hpl_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("ADC2HP_L Playback Switch", ALC5623_ADC_REC_GAIN, 15, 1, 1),
];
static alc5623_hpr_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("ADC2HP_R Playback Switch", ALC5623_ADC_REC_GAIN, 14, 1, 1),
];
static alc5623_mono_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("ADC2MONO_L Playback Switch", ALC5623_ADC_REC_GAIN, 13, 1, 1),
    SOC_DAPM_SINGLE!("ADC2MONO_R Playback Switch", ALC5623_ADC_REC_GAIN, 12, 1, 1),
    SOC_DAPM_SINGLE!("LI2MONO Playback Switch", ALC5623_LINE_IN_VOL, 13, 1, 1),
    SOC_DAPM_SINGLE!("AUXI2MONO Playback Switch", ALC5623_AUXIN_VOL, 13, 1, 1),
    SOC_DAPM_SINGLE!("MIC12MONO Playback Switch", ALC5623_MIC_ROUTING_CTRL, 13, 1, 1),
    SOC_DAPM_SINGLE!("MIC22MONO Playback Switch", ALC5623_MIC_ROUTING_CTRL, 5, 1, 1),
    SOC_DAPM_SINGLE!("DAC2MONO Playback Switch", ALC5623_STEREO_DAC_VOL, 13, 1, 1),
];
static alc5623_speaker_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("LI2SPK Playback Switch", ALC5623_LINE_IN_VOL, 14, 1, 1),
    SOC_DAPM_SINGLE!("AUXI2SPK Playback Switch", ALC5623_AUXIN_VOL, 14, 1, 1),
    SOC_DAPM_SINGLE!("MIC12SPK Playback Switch", ALC5623_MIC_ROUTING_CTRL, 14, 1, 1),
    SOC_DAPM_SINGLE!("MIC22SPK Playback Switch", ALC5623_MIC_ROUTING_CTRL, 6, 1, 1),
    SOC_DAPM_SINGLE!("DAC2SPK Playback Switch", ALC5623_STEREO_DAC_VOL, 14, 1, 1),
];

/* Left Record Mixer */
static alc5623_captureL_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Mic1 Capture Switch", ALC5623_ADC_REC_MIXER, 14, 1, 1),
    SOC_DAPM_SINGLE!("Mic2 Capture Switch", ALC5623_ADC_REC_MIXER, 13, 1, 1),
    SOC_DAPM_SINGLE!("LineInL Capture Switch", ALC5623_ADC_REC_MIXER, 12, 1, 1),
    SOC_DAPM_SINGLE!("Left AuxI Capture Switch", ALC5623_ADC_REC_MIXER, 11, 1, 1),
    SOC_DAPM_SINGLE!("HPMixerL Capture Switch", ALC5623_ADC_REC_MIXER, 10, 1, 1),
    SOC_DAPM_SINGLE!("SPKMixer Capture Switch", ALC5623_ADC_REC_MIXER, 9, 1, 1),
    SOC_DAPM_SINGLE!("MonoMixer Capture Switch", ALC5623_ADC_REC_MIXER, 8, 1, 1),
];

/* Right Record Mixer */
static alc5623_captureR_mixer_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Mic1 Capture Switch", ALC5623_ADC_REC_MIXER, 6, 1, 1),
    SOC_DAPM_SINGLE!("Mic2 Capture Switch", ALC5623_ADC_REC_MIXER, 5, 1, 1),
    SOC_DAPM_SINGLE!("LineInR Capture Switch", ALC5623_ADC_REC_MIXER, 4, 1, 1),
    SOC_DAPM_SINGLE!("Right AuxI Capture Switch", ALC5623_ADC_REC_MIXER, 3, 1, 1),
    SOC_DAPM_SINGLE!("HPMixerR Capture Switch", ALC5623_ADC_REC_MIXER, 2, 1, 1),
    SOC_DAPM_SINGLE!("SPKMixer Capture Switch", ALC5623_ADC_REC_MIXER, 1, 1, 1),
    SOC_DAPM_SINGLE!("MonoMixer Capture Switch", ALC5623_ADC_REC_MIXER, 0, 1, 1),
];

static alc5623_spk_n_sour_sel: [&str; 4] = ["RN/-R", "RP/+R", "LN/-R", "Vmid"];
static alc5623_hpl_out_input_sel: [&str; 2] = ["Vmid", "HP Left Mix"];
static alc5623_hpr_out_input_sel: [&str; 2] = ["Vmid", "HP Right Mix"];
static alc5623_spkout_input_sel: [&str; 4] = ["Vmid", "HPOut Mix", "Speaker Mix", "Mono Mix"];
static alc5623_aux_out_input_sel: [&str; 4] = ["Vmid", "HPOut Mix", "Speaker Mix", "Mono Mix"];

/* auxout output mux */
static alc5623_aux_out_input_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(ALC5623_OUTPUT_MIXER_CTRL, 6, alc5623_aux_out_input_sel);
static alc5623_auxout_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", alc5623_aux_out_input_enum);

/* speaker output mux */
static alc5623_spkout_input_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(ALC5623_OUTPUT_MIXER_CTRL, 10, alc5623_spkout_input_sel);
static alc5623_spkout_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", alc5623_spkout_input_enum);

/* headphone left output mux */
static alc5623_hpl_out_input_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(ALC5623_OUTPUT_MIXER_CTRL, 9, alc5623_hpl_out_input_sel);
static alc5623_hpl_out_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", alc5623_hpl_out_input_enum);

/* headphone right output mux */
static alc5623_hpr_out_input_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(ALC5623_OUTPUT_MIXER_CTRL, 8, alc5623_hpr_out_input_sel);
static alc5623_hpr_out_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", alc5623_hpr_out_input_enum);

/* speaker output N select */
static alc5623_spk_n_sour_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(ALC5623_OUTPUT_MIXER_CTRL, 14, alc5623_spk_n_sour_sel);
static alc5623_spkoutn_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", alc5623_spk_n_sour_enum);

static alc5623_dapm_widgets: &[snd_soc_dapm_widget] = &[
    /* Muxes */
    SND_SOC_DAPM_MUX!("AuxOut Mux", SND_SOC_NOPM, 0, 0, &alc5623_auxout_mux_controls),
    SND_SOC_DAPM_MUX!("SpeakerOut Mux", SND_SOC_NOPM, 0, 0, &alc5623_spkout_mux_controls),
    SND_SOC_DAPM_MUX!("Left Headphone Mux", SND_SOC_NOPM, 0, 0, &alc5623_hpl_out_mux_controls),
    SND_SOC_DAPM_MUX!("Right Headphone Mux", SND_SOC_NOPM, 0, 0, &alc5623_hpr_out_mux_controls),
    SND_SOC_DAPM_MUX!("SpeakerOut N Mux", SND_SOC_NOPM, 0, 0, &alc5623_spkoutn_mux_controls),
    /* output mixers */
    SND_SOC_DAPM_MIXER!("HP Mix", SND_SOC_NOPM, 0, 0, &alc5623_hp_mixer_controls[0], ARRAY_SIZE!(alc5623_hp_mixer_controls)),
    SND_SOC_DAPM_MIXER!("HPR Mix", ALC5623_PWR_MANAG_ADD2, 4, 0, &alc5623_hpr_mixer_controls[0], ARRAY_SIZE!(alc5623_hpr_mixer_controls)),
    SND_SOC_DAPM_MIXER!("HPL Mix", ALC5623_PWR_MANAG_ADD2, 5, 0, &alc5623_hpl_mixer_controls[0], ARRAY_SIZE!(alc5623_hpl_mixer_controls)),
    SND_SOC_DAPM_MIXER!("HPOut Mix", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Mono Mix", ALC5623_PWR_MANAG_ADD2, 2, 0, &alc5623_mono_mixer_controls[0], ARRAY_SIZE!(alc5623_mono_mixer_controls)),
    SND_SOC_DAPM_MIXER!("Speaker Mix", ALC5623_PWR_MANAG_ADD2, 3, 0, &alc5623_speaker_mixer_controls[0], ARRAY_SIZE!(alc5623_speaker_mixer_controls)),
    /* input mixers */
    SND_SOC_DAPM_MIXER!("Left Capture Mix", ALC5623_PWR_MANAG_ADD2, 1, 0, &alc5623_captureL_mixer_controls[0], ARRAY_SIZE!(alc5623_captureL_mixer_controls)),
    SND_SOC_DAPM_MIXER!("Right Capture Mix", ALC5623_PWR_MANAG_ADD2, 0, 0, &alc5623_captureR_mixer_controls[0], ARRAY_SIZE!(alc5623_captureR_mixer_controls)),
    SND_SOC_DAPM_DAC!("Left DAC", "Left HiFi Playback", ALC5623_PWR_MANAG_ADD2, 9, 0),
    SND_SOC_DAPM_DAC!("Right DAC", "Right HiFi Playback", ALC5623_PWR_MANAG_ADD2, 8, 0),
    SND_SOC_DAPM_MIXER!("I2S Mix", ALC5623_PWR_MANAG_ADD1, 15, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("AuxI Mix", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Line Mix", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_ADC!("Left ADC", "Left HiFi Capture", ALC5623_PWR_MANAG_ADD2, 7, 0),
    SND_SOC_DAPM_ADC!("Right ADC", "Right HiFi Capture", ALC5623_PWR_MANAG_ADD2, 6, 0),
    SND_SOC_DAPM_PGA!("Left Headphone", ALC5623_PWR_MANAG_ADD3, 10, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right Headphone", ALC5623_PWR_MANAG_ADD3, 9, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("SpeakerOut", ALC5623_PWR_MANAG_ADD3, 12, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left AuxOut", ALC5623_PWR_MANAG_ADD3, 14, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right AuxOut", ALC5623_PWR_MANAG_ADD3, 13, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left LineIn", ALC5623_PWR_MANAG_ADD3, 7, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right LineIn", ALC5623_PWR_MANAG_ADD3, 6, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left AuxI", ALC5623_PWR_MANAG_ADD3, 5, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right AuxI", ALC5623_PWR_MANAG_ADD3, 4, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("MIC1 PGA", ALC5623_PWR_MANAG_ADD3, 3, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("MIC2 PGA", ALC5623_PWR_MANAG_ADD3, 2, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("MIC1 Pre Amp", ALC5623_PWR_MANAG_ADD3, 1, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("MIC2 Pre Amp", ALC5623_PWR_MANAG_ADD3, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MICBIAS!("Mic Bias1", ALC5623_PWR_MANAG_ADD1, 11, 0),
    SND_SOC_DAPM_OUTPUT!("AUXOUTL"),
    SND_SOC_DAPM_OUTPUT!("AUXOUTR"),
    SND_SOC_DAPM_OUTPUT!("HPL"),
    SND_SOC_DAPM_OUTPUT!("HPR"),
    SND_SOC_DAPM_OUTPUT!("SPKOUT"),
    SND_SOC_DAPM_OUTPUT!("SPKOUTN"),
    SND_SOC_DAPM_INPUT!("LINEINL"),
    SND_SOC_DAPM_INPUT!("LINEINR"),
    SND_SOC_DAPM_INPUT!("AUXINL"),
    SND_SOC_DAPM_INPUT!("AUXINR"),
    SND_SOC_DAPM_INPUT!("MIC1"),
    SND_SOC_DAPM_INPUT!("MIC2"),
    SND_SOC_DAPM_VMID!("Vmid"),
];

static alc5623_amp_names: [&str; 2] = ["AB Amp", "D Amp"];
static alc5623_amp_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(ALC5623_OUTPUT_MIXER_CTRL, 13, alc5623_amp_names);
static alc5623_amp_mux_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", alc5623_amp_enum);

static alc5623_dapm_amp_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_PGA_E!("D Amp", ALC5623_PWR_MANAG_ADD2, 14, 0, core::ptr::null(), 0,
        amp_mixer_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_PGA!("AB Amp", ALC5623_PWR_MANAG_ADD2, 15, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MUX!("AB-D Amp Mux", SND_SOC_NOPM, 0, 0, &alc5623_amp_mux_controls),
];

static intercon: &[snd_soc_dapm_route] = &[
    /* virtual mixer - mixes left & right channels */
    snd_soc_dapm_route { sink: "I2S Mix", control: core::ptr::null(), source: "Left DAC" },
    snd_soc_dapm_route { sink: "I2S Mix", control: core::ptr::null(), source: "Right DAC" },
    snd_soc_dapm_route { sink: "Line Mix", control: core::ptr::null(), source: "Right LineIn" },
    snd_soc_dapm_route { sink: "Line Mix", control: core::ptr::null(), source: "Left LineIn" },
    snd_soc_dapm_route { sink: "AuxI Mix", control: core::ptr::null(), source: "Left AuxI" },
    snd_soc_dapm_route { sink: "AuxI Mix", control: core::ptr::null(), source: "Right AuxI" },
    snd_soc_dapm_route { sink: "AUXOUTL", control: core::ptr::null(), source: "Left AuxOut" },
    snd_soc_dapm_route { sink: "AUXOUTR", control: core::ptr::null(), source: "Right AuxOut" },
    snd_soc_dapm_route { sink: "HPL Mix", control: "ADC2HP_L Playback Switch", source: "Left Capture Mix" },
    snd_soc_dapm_route { sink: "HPL Mix", control: core::ptr::null(), source: "HP Mix" },
    snd_soc_dapm_route { sink: "HPR Mix", control: "ADC2HP_R Playback Switch", source: "Right Capture Mix" },
    snd_soc_dapm_route { sink: "HPR Mix", control: core::ptr::null(), source: "HP Mix" },
    snd_soc_dapm_route { sink: "HP Mix", control: "LI2HP Playback Switch", source: "Line Mix" },
    snd_soc_dapm_route { sink: "HP Mix", control: "AUXI2HP Playback Switch", source: "AuxI Mix" },
    snd_soc_dapm_route { sink: "HP Mix", control: "MIC12HP Playback Switch", source: "MIC1 PGA" },
    snd_soc_dapm_route { sink: "HP Mix", control: "MIC22HP Playback Switch", source: "MIC2 PGA" },
    snd_soc_dapm_route { sink: "HP Mix", control: "DAC2HP Playback Switch", source: "I2S Mix" },
    snd_soc_dapm_route { sink: "Speaker Mix", control: "LI2SPK Playback Switch", source: "Line Mix" },
    snd_soc_dapm_route { sink: "Speaker Mix", control: "AUXI2SPK Playback Switch", source: "AuxI Mix" },
    snd_soc_dapm_route { sink: "Speaker Mix", control: "MIC12SPK Playback Switch", source: "MIC1 PGA" },
    snd_soc_dapm_route { sink: "Speaker Mix", control: "MIC22SPK Playback Switch", source: "MIC2 PGA" },
    snd_soc_dapm_route { sink: "Speaker Mix", control: "DAC2SPK Playback Switch", source: "I2S Mix" },
    snd_soc_dapm_route { sink: "Mono Mix", control: "ADC2MONO_L Playback Switch", source: "Left Capture Mix" },
    snd_soc_dapm_route { sink: "Mono Mix", control: "ADC2MONO_R Playback Switch", source: "Right Capture Mix" },
    snd_soc_dapm_route { sink: "Mono Mix", control: "LI2MONO Playback Switch", source: "Line Mix" },
    snd_soc_dapm_route { sink: "Mono Mix", control: "AUXI2MONO Playback Switch", source: "AuxI Mix" },
    snd_soc_dapm_route { sink: "Mono Mix", control: "MIC12MONO Playback Switch", source: "MIC1 PGA" },
    snd_soc_dapm_route { sink: "Mono Mix", control: "MIC22MONO Playback Switch", source: "MIC2 PGA" },
    snd_soc_dapm_route { sink: "Mono Mix", control: "DAC2MONO Playback Switch", source: "I2S Mix" },
    snd_soc_dapm_route { sink: "Left Capture Mix", control: "LineInL Capture Switch", source: "LINEINL" },
    snd_soc_dapm_route { sink: "Left Capture Mix", control: "Left AuxI Capture Switch", source: "AUXINL" },
    snd_soc_dapm_route { sink: "Left Capture Mix", control: "Mic1 Capture Switch", source: "MIC1 Pre Amp" },
    snd_soc_dapm_route { sink: "Left Capture Mix", control: "Mic2 Capture Switch", source: "MIC2 Pre Amp" },
    snd_soc_dapm_route { sink: "Left Capture Mix", control: "HPMixerL Capture Switch", source: "HPL Mix" },
    snd_soc_dapm_route { sink: "Left Capture Mix", control: "SPKMixer Capture Switch", source: "Speaker Mix" },
    snd_soc_dapm_route { sink: "Left Capture Mix", control: "MonoMixer Capture Switch", source: "Mono Mix" },
    snd_soc_dapm_route { sink: "Right Capture Mix", control: "LineInR Capture Switch", source: "LINEINR" },
    snd_soc_dapm_route { sink: "Right Capture Mix", control: "Right AuxI Capture Switch", source: "AUXINR" },
    snd_soc_dapm_route { sink: "Right Capture Mix", control: "Mic1 Capture Switch", source: "MIC1 Pre Amp" },
    snd_soc_dapm_route { sink: "Right Capture Mix", control: "Mic2 Capture Switch", source: "MIC2 Pre Amp" },
    snd_soc_dapm_route { sink: "Right Capture Mix", control: "HPMixerR Capture Switch", source: "HPR Mix" },
    snd_soc_dapm_route { sink: "Right Capture Mix", control: "SPKMixer Capture Switch", source: "Speaker Mix" },
    snd_soc_dapm_route { sink: "Right Capture Mix", control: "MonoMixer Capture Switch", source: "Mono Mix" },
    snd_soc_dapm_route { sink: "Left Headphone Mux", control: "HP Left Mix", source: "HPL Mix" },
    snd_soc_dapm_route { sink: "Left Headphone Mux", control: "Vmid", source: "Vmid" },
    snd_soc_dapm_route { sink: "Right Headphone Mux", control: "HP Right Mix", source: "HPR Mix" },
    snd_soc_dapm_route { sink: "Right Headphone Mux", control: "Vmid", source: "Vmid" },
    snd_soc_dapm_route { sink: "SpeakerOut Mux", control: "Vmid", source: "Vmid" },
    snd_soc_dapm_route { sink: "SpeakerOut Mux", control: "HPOut Mix", source: "HPOut Mix" },
    snd_soc_dapm_route { sink: "SpeakerOut Mux", control: "Speaker Mix", source: "Speaker Mix" },
    snd_soc_dapm_route { sink: "SpeakerOut Mux", control: "Mono Mix", source: "Mono Mix" },
    snd_soc_dapm_route { sink: "AuxOut Mux", control: "Vmid", source: "Vmid" },
    snd_soc_dapm_route { sink: "AuxOut Mux", control: "HPOut Mix", source: "HPOut Mix" },
    snd_soc_dapm_route { sink: "AuxOut Mux", control: "Speaker Mix", source: "Speaker Mix" },
    snd_soc_dapm_route { sink: "AuxOut Mux", control: "Mono Mix", source: "Mono Mix" },
    snd_soc_dapm_route { sink: "HPL", control: core::ptr::null(), source: "Left Headphone" },
    snd_soc_dapm_route { sink: "Left Headphone", control: core::ptr::null(), source: "Left Headphone Mux" },
    snd_soc_dapm_route { sink: "HPR", control: core::ptr::null(), source: "Right Headphone" },
    snd_soc_dapm_route { sink: "Right Headphone", control: core::ptr::null(), source: "Right Headphone Mux" },
    snd_soc_dapm_route { sink: "Left AuxOut", control: core::ptr::null(), source: "AuxOut Mux" },
    snd_soc_dapm_route { sink: "Right AuxOut", control: core::ptr::null(), source: "AuxOut Mux" },
    snd_soc_dapm_route { sink: "Left LineIn", control: core::ptr::null(), source: "LINEINL" },
    snd_soc_dapm_route { sink: "Right LineIn", control: core::ptr::null(), source: "LINEINR" },
    snd_soc_dapm_route { sink: "Left AuxI", control: core::ptr::null(), source: "AUXINL" },
    snd_soc_dapm_route { sink: "Right AuxI", control: core::ptr::null(), source: "AUXINR" },
    snd_soc_dapm_route { sink: "MIC1 Pre Amp", control: core::ptr::null(), source: "MIC1" },
    snd_soc_dapm_route { sink: "MIC2 Pre Amp", control: core::ptr::null(), source: "MIC2" },
    snd_soc_dapm_route { sink: "MIC1 PGA", control: core::ptr::null(), source: "MIC1 Pre Amp" },
    snd_soc_dapm_route { sink: "MIC2 PGA", control: core::ptr::null(), source: "MIC2 Pre Amp" },
    snd_soc_dapm_route { sink: "Left ADC", control: core::ptr::null(), source: "Left Capture Mix" },
    snd_soc_dapm_route { sink: "Right ADC", control: core::ptr::null(), source: "Right Capture Mix" },
    snd_soc_dapm_route { sink: "SpeakerOut N Mux", control: "RN/-R", source: "SpeakerOut" },
    snd_soc_dapm_route { sink: "SpeakerOut N Mux", control: "RP/+R", source: "SpeakerOut" },
    snd_soc_dapm_route { sink: "SpeakerOut N Mux", control: "LN/-R", source: "SpeakerOut" },
    snd_soc_dapm_route { sink: "SpeakerOut N Mux", control: "Vmid", source: "Vmid" },
    snd_soc_dapm_route { sink: "SPKOUT", control: core::ptr::null(), source: "SpeakerOut" },
    snd_soc_dapm_route { sink: "SPKOUTN", control: core::ptr::null(), source: "SpeakerOut N Mux" },
];

static intercon_spk: &[snd_soc_dapm_route] = &[
    snd_soc_dapm_route { sink: "SpeakerOut", control: core::ptr::null(), source: "SpeakerOut Mux" },
];

static intercon_amp_spk: &[snd_soc_dapm_route] = &[
    snd_soc_dapm_route { sink: "AB Amp", control: core::ptr::null(), source: "SpeakerOut Mux" },
    snd_soc_dapm_route { sink: "D Amp", control: core::ptr::null(), source: "SpeakerOut Mux" },
    snd_soc_dapm_route { sink: "AB-D Amp Mux", control: "AB Amp", source: "AB Amp" },
    snd_soc_dapm_route { sink: "AB-D Amp Mux", control: "D Amp", source: "D Amp" },
    snd_soc_dapm_route { sink: "SpeakerOut", control: core::ptr::null(), source: "AB-D Amp Mux" },
];

/* PLL divisors */
#[repr(C)]
struct _pll_div {
    pll_in: u32,
    pll_out: u32,
    regvalue: u16,
}

/* Note : pll code from original alc5623 driver. Not sure of how good it is */
/* useful only for master mode */
static codec_master_pll_div: [_pll_div; 30] = [
    _pll_div { pll_in: 2048000, pll_out: 8192000, regvalue: 0x0ea0 },
    _pll_div { pll_in: 3686400, pll_out: 8192000, regvalue: 0x4e27 },
    _pll_div { pll_in: 12000000, pll_out: 8192000, regvalue: 0x456b },
    _pll_div { pll_in: 13000000, pll_out: 8192000, regvalue: 0x495f },
    _pll_div { pll_in: 13100000, pll_out: 8192000, regvalue: 0x0320 },
    _pll_div { pll_in: 2048000, pll_out: 11289600, regvalue: 0xf637 },
    _pll_div { pll_in: 3686400, pll_out: 11289600, regvalue: 0x2f22 },
    _pll_div { pll_in: 12000000, pll_out: 11289600, regvalue: 0x3e2f },
    _pll_div { pll_in: 13000000, pll_out: 11289600, regvalue: 0x4d5b },
    _pll_div { pll_in: 13100000, pll_out: 11289600, regvalue: 0x363b },
    _pll_div { pll_in: 2048000, pll_out: 16384000, regvalue: 0x1ea0 },
    _pll_div { pll_in: 3686400, pll_out: 16384000, regvalue: 0x9e27 },
    _pll_div { pll_in: 12000000, pll_out: 16384000, regvalue: 0x452b },
    _pll_div { pll_in: 13000000, pll_out: 16384000, regvalue: 0x542f },
    _pll_div { pll_in: 13100000, pll_out: 16384000, regvalue: 0x03a0 },
    _pll_div { pll_in: 2048000, pll_out: 16934400, regvalue: 0xe625 },
    _pll_div { pll_in: 3686400, pll_out: 16934400, regvalue: 0x9126 },
    _pll_div { pll_in: 12000000, pll_out: 16934400, regvalue: 0x4d2c },
    _pll_div { pll_in: 13000000, pll_out: 16934400, regvalue: 0x742f },
    _pll_div { pll_in: 13100000, pll_out: 16934400, regvalue: 0x3c27 },
    _pll_div { pll_in: 2048000, pll_out: 22579200, regvalue: 0x2aa0 },
    _pll_div { pll_in: 3686400, pll_out: 22579200, regvalue: 0x2f20 },
    _pll_div { pll_in: 12000000, pll_out: 22579200, regvalue: 0x7e2f },
    _pll_div { pll_in: 13000000, pll_out: 22579200, regvalue: 0x742f },
    _pll_div { pll_in: 13100000, pll_out: 22579200, regvalue: 0x3c27 },
    _pll_div { pll_in: 2048000, pll_out: 24576000, regvalue: 0x2ea0 },
    _pll_div { pll_in: 3686400, pll_out: 24576000, regvalue: 0xee27 },
    _pll_div { pll_in: 12000000, pll_out: 24576000, regvalue: 0x2915 },
    _pll_div { pll_in: 13000000, pll_out: 24576000, regvalue: 0x772e },
    _pll_div { pll_in: 13100000, pll_out: 24576000, regvalue: 0x0d20 },
];

static codec_slave_pll_div: [_pll_div; 6] = [
    _pll_div { pll_in: 1024000, pll_out: 16384000, regvalue: 0x3ea0 },
    _pll_div { pll_in: 1411200, pll_out: 22579200, regvalue: 0x3ea0 },
    _pll_div { pll_in: 1536000, pll_out: 24576000, regvalue: 0x3ea0 },
    _pll_div { pll_in: 2048000, pll_out: 16384000, regvalue: 0x1ea0 },
    _pll_div { pll_in: 2822400, pll_out: 22579200, regvalue: 0x1ea0 },
    _pll_div { pll_in: 3072000, pll_out: 24576000, regvalue: 0x1ea0 },
];

unsafe fn alc5623_set_dai_pll(
    codec_dai: *mut snd_soc_dai,
    pll_id: c_int,
    _source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let component = (*codec_dai).component;
    let mut gbl_clk: c_int = 0;
    let mut pll_div: c_int = 0;
    let mut reg: u16;

    if pll_id < ALC5623_PLL_FR_MCLK || pll_id > ALC5623_PLL_FR_BCK {
        return -ENODEV;
    }

    /* Disable PLL power */
    snd_soc_component_update_bits(component, ALC5623_PWR_MANAG_ADD2, ALC5623_PWR_ADD2_PLL, 0);

    /* pll is not used in slave mode */
    reg = snd_soc_component_read(component, ALC5623_DAI_CONTROL) as u16;
    if (reg & ALC5623_DAI_SDP_SLAVE_MODE as u16) != 0 {
        return 0;
    }

    if freq_in == 0 || freq_out == 0 {
        return 0;
    }

    match pll_id {
        ALC5623_PLL_FR_MCLK => {
            for div in codec_master_pll_div.iter() {
                if div.pll_in == freq_in && div.pll_out == freq_out {
                    /* PLL source from MCLK */
                    pll_div = div.regvalue as c_int;
                    break;
                }
            }
        }
        ALC5623_PLL_FR_BCK => {
            for div in codec_slave_pll_div.iter() {
                if div.pll_in == freq_in && div.pll_out == freq_out {
                    /* PLL source from Bitclk */
                    gbl_clk = ALC5623_GBL_CLK_PLL_SOUR_SEL_BITCLK;
                    pll_div = div.regvalue as c_int;
                    break;
                }
            }
        }
        _ => return -EINVAL,
    }

    if pll_div == 0 {
        return -EINVAL;
    }

    snd_soc_component_write(component, ALC5623_GLOBAL_CLK_CTRL_REG, gbl_clk);
    snd_soc_component_write(component, ALC5623_PLL_CTRL, pll_div);
    snd_soc_component_update_bits(component, ALC5623_PWR_MANAG_ADD2, ALC5623_PWR_ADD2_PLL, ALC5623_PWR_ADD2_PLL);
    gbl_clk |= ALC5623_GBL_CLK_SYS_SOUR_SEL_PLL;
    snd_soc_component_write(component, ALC5623_GLOBAL_CLK_CTRL_REG, gbl_clk);

    0
}

#[repr(C)]
struct _coeff_div {
    fs: u16,
    regvalue: u16,
}

/* codec hifi mclk (after PLL) clock divider coefficients */
/* values inspired from column BCLK=32Fs of Appendix A table */
static coeff_div: [_coeff_div; 8] = [
    _coeff_div { fs: 256 * 8, regvalue: 0x3a69 },
    _coeff_div { fs: 384 * 8, regvalue: 0x3c6b },
    _coeff_div { fs: 256 * 4, regvalue: 0x2a69 },
    _coeff_div { fs: 384 * 4, regvalue: 0x2c6b },
    _coeff_div { fs: 256 * 2, regvalue: 0x1a69 },
    _coeff_div { fs: 384 * 2, regvalue: 0x1c6b },
    _coeff_div { fs: 256 * 1, regvalue: 0x0a69 },
    _coeff_div { fs: 384 * 1, regvalue: 0x0c6b },
];

unsafe fn get_coeff(component: *mut snd_soc_component, rate: c_int) -> c_int {
    let alc5623 = snd_soc_component_get_drvdata(component) as *mut alc5623_priv;

    for (i, div) in coeff_div.iter().enumerate() {
        if div.fs as c_int * rate == (*alc5623).sysclk as c_int {
            return i as c_int;
        }
    }
    -EINVAL
}

/*
 * Clock after PLL and dividers
 */
unsafe fn alc5623_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let alc5623 = snd_soc_component_get_drvdata(component) as *mut alc5623_priv;

    match freq {
        8192000 | 11289600 | 12288000 | 16384000 | 16934400 | 18432000 | 22579200 | 24576000 => {
            (*alc5623).sysclk = freq;
            0
        }
        _ => -EINVAL,
    }
}

unsafe fn alc5623_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut iface: u16 = 0;

    /* set audio interface clocking */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => iface = ALC5623_DAI_SDP_MASTER_MODE as u16,
        SND_SOC_DAIFMT_CBC_CFC => iface = ALC5623_DAI_SDP_SLAVE_MODE as u16,
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => iface |= ALC5623_DAI_I2S_DF_I2S as u16,
        SND_SOC_DAIFMT_RIGHT_J => iface |= ALC5623_DAI_I2S_DF_RIGHT as u16,
        SND_SOC_DAIFMT_LEFT_J => iface |= ALC5623_DAI_I2S_DF_LEFT as u16,
        SND_SOC_DAIFMT_DSP_A => iface |= ALC5623_DAI_I2S_DF_PCM as u16,
        SND_SOC_DAIFMT_DSP_B => iface |= (ALC5623_DAI_I2S_DF_PCM | ALC5623_DAI_I2S_PCM_MODE) as u16,
        _ => return -EINVAL,
    }

    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => iface |= ALC5623_DAI_MAIN_I2S_BCLK_POL_CTRL as u16,
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, ALC5623_DAI_CONTROL, iface as c_int)
}

unsafe fn alc5623_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let alc5623 = snd_soc_component_get_drvdata(component) as *mut alc5623_priv;
    let mut coeff: c_int;
    let rate: c_int;
    let mut iface: u16;

    iface = snd_soc_component_read(component, ALC5623_DAI_CONTROL) as u16;
    iface &= !(ALC5623_DAI_I2S_DL_MASK as u16);

    /* bit size */
    match params_width(params) {
        16 => iface |= ALC5623_DAI_I2S_DL_16 as u16,
        20 => iface |= ALC5623_DAI_I2S_DL_20 as u16,
        24 => iface |= ALC5623_DAI_I2S_DL_24 as u16,
        32 => iface |= ALC5623_DAI_I2S_DL_32 as u16,
        _ => return -EINVAL,
    }

    /* set iface & srate */
    snd_soc_component_write(component, ALC5623_DAI_CONTROL, iface as c_int);
    rate = params_rate(params);
    coeff = get_coeff(component, rate);
    if coeff < 0 {
        return -EINVAL;
    }

    coeff = coeff_div[coeff as usize].regvalue as c_int;
    dev_dbg!((*component).dev, "%s: sysclk=%d,rate=%d,coeff=0x%04x\n",
        __func__, (*alc5623).sysclk, rate, coeff);
    snd_soc_component_write(component, ALC5623_STEREO_AD_DA_CLK_CTRL, coeff);

    0
}

unsafe fn alc5623_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let hp_mute: u16 = (ALC5623_MISC_M_DAC_L_INPUT | ALC5623_MISC_M_DAC_R_INPUT) as u16;
    let mut mute_reg: u16 =
        (snd_soc_component_read(component, ALC5623_MISC_CTRL) as u16) & !hp_mute;

    if mute != 0 {
        mute_reg |= hp_mute;
    }

    snd_soc_component_write(component, ALC5623_MISC_CTRL, mute_reg as c_int)
}

const ALC5623_ADD2_POWER_EN: c_int = ALC5623_PWR_ADD2_VREF | ALC5623_PWR_ADD2_DAC_REF_CIR;
const ALC5623_ADD3_POWER_EN: c_int = ALC5623_PWR_ADD3_MAIN_BIAS | ALC5623_PWR_ADD3_MIC1_BOOST_AD;
const ALC5623_ADD1_POWER_EN: c_int = ALC5623_PWR_ADD1_SHORT_CURR_DET_EN
    | ALC5623_PWR_ADD1_SOFTGEN_EN
    | ALC5623_PWR_ADD1_DEPOP_BUF_HP
    | ALC5623_PWR_ADD1_HP_OUT_AMP
    | ALC5623_PWR_ADD1_HP_OUT_ENH_AMP;
const ALC5623_ADD1_POWER_EN_5622: c_int =
    ALC5623_PWR_ADD1_SHORT_CURR_DET_EN | ALC5623_PWR_ADD1_HP_OUT_AMP;

unsafe fn enable_power_depop(component: *mut snd_soc_component) {
    let alc5623 = snd_soc_component_get_drvdata(component) as *mut alc5623_priv;

    snd_soc_component_update_bits(component, ALC5623_PWR_MANAG_ADD1,
        ALC5623_PWR_ADD1_SOFTGEN_EN, ALC5623_PWR_ADD1_SOFTGEN_EN);
    snd_soc_component_write(component, ALC5623_PWR_MANAG_ADD3, ALC5623_ADD3_POWER_EN);
    snd_soc_component_update_bits(component, ALC5623_MISC_CTRL,
        ALC5623_MISC_HP_DEPOP_MODE2_EN, ALC5623_MISC_HP_DEPOP_MODE2_EN);

    msleep(500);

    snd_soc_component_write(component, ALC5623_PWR_MANAG_ADD2, ALC5623_ADD2_POWER_EN);

    /* avoid writing '1' into 5622 reserved bits */
    if (*alc5623).id == 0x22 {
        snd_soc_component_write(component, ALC5623_PWR_MANAG_ADD1, ALC5623_ADD1_POWER_EN_5622);
    } else {
        snd_soc_component_write(component, ALC5623_PWR_MANAG_ADD1, ALC5623_ADD1_POWER_EN);
    }

    /* disable HP Depop2 */
    snd_soc_component_update_bits(component, ALC5623_MISC_CTRL, ALC5623_MISC_HP_DEPOP_MODE2_EN, 0);
}

unsafe fn alc5623_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    match level {
        SND_SOC_BIAS_ON => enable_power_depop(component),
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            /* everything off except vref/vmid, */
            snd_soc_component_write(component, ALC5623_PWR_MANAG_ADD2, ALC5623_PWR_ADD2_VREF);
            snd_soc_component_write(component, ALC5623_PWR_MANAG_ADD3, ALC5623_PWR_ADD3_MAIN_BIAS);
        }
        SND_SOC_BIAS_OFF => {
            /* everything off, dac mute, inactive */
            snd_soc_component_write(component, ALC5623_PWR_MANAG_ADD2, 0);
            snd_soc_component_write(component, ALC5623_PWR_MANAG_ADD3, 0);
            snd_soc_component_write(component, ALC5623_PWR_MANAG_ADD1, 0);
        }
        _ => {}
    }
    0
}

const ALC5623_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static alc5623_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(alc5623_pcm_hw_params),
    mute_stream: Some(alc5623_mute),
    set_fmt: Some(alc5623_set_dai_fmt),
    set_sysclk: Some(alc5623_set_dai_sysclk),
    set_pll: Some(alc5623_set_dai_pll),
    no_capture_mute: 1,
    ..Default::default()
};

static mut alc5623_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: "alc5623-hifi",
    playback: snd_soc_pcm_stream {
        stream_name: "Playback",
        channels_min: 1,
        channels_max: 2,
        rate_min: 8000,
        rate_max: 48000,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: ALC5623_FORMATS,
        ..Default::default()
    },
    capture: snd_soc_pcm_stream {
        stream_name: "Capture",
        channels_min: 1,
        channels_max: 2,
        rate_min: 8000,
        rate_max: 48000,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: ALC5623_FORMATS,
        ..Default::default()
    },
    ops: &alc5623_dai_ops,
    ..Default::default()
};

unsafe fn alc5623_suspend(component: *mut snd_soc_component) -> c_int {
    let alc5623 = snd_soc_component_get_drvdata(component) as *mut alc5623_priv;
    regcache_cache_only((*alc5623).regmap, true);
    0
}

unsafe fn alc5623_resume(component: *mut snd_soc_component) -> c_int {
    let alc5623 = snd_soc_component_get_drvdata(component) as *mut alc5623_priv;
    let ret: c_int;

    /* Sync reg_cache with the hardware */
    regcache_cache_only((*alc5623).regmap, false);
    ret = regcache_sync((*alc5623).regmap);
    if ret != 0 {
        dev_err!((*component).dev, "Failed to sync register cache: %d\n", ret);
        regcache_cache_only((*alc5623).regmap, true);
        return ret;
    }

    0
}

unsafe fn alc5623_probe(component: *mut snd_soc_component) -> c_int {
    let alc5623 = snd_soc_component_get_drvdata(component) as *mut alc5623_priv;
    let dapm = snd_soc_component_to_dapm(component);

    alc5623_reset(component);

    if (*alc5623).add_ctrl != 0 {
        snd_soc_component_write(component, ALC5623_ADD_CTRL_REG, (*alc5623).add_ctrl as c_int);
    }

    if (*alc5623).jack_det_ctrl != 0 {
        snd_soc_component_write(component, ALC5623_JACK_DET_CTRL, (*alc5623).jack_det_ctrl as c_int);
    }

    match (*alc5623).id {
        0x21 => snd_soc_add_component_controls(component, alc5621_vol_snd_controls.as_ptr(), alc5621_vol_snd_controls.len() as c_uint),
        0x22 => snd_soc_add_component_controls(component, alc5622_vol_snd_controls.as_ptr(), alc5622_vol_snd_controls.len() as c_uint),
        0x23 => snd_soc_add_component_controls(component, alc5623_vol_snd_controls.as_ptr(), alc5623_vol_snd_controls.len() as c_uint),
        _ => return -EINVAL,
    };

    snd_soc_add_component_controls(component, alc5623_snd_controls.as_ptr(), alc5623_snd_controls.len() as c_uint);
    snd_soc_dapm_new_controls(dapm, alc5623_dapm_widgets.as_ptr(), alc5623_dapm_widgets.len() as c_uint);

    /* set up audio path interconnects */
    snd_soc_dapm_add_routes(dapm, intercon.as_ptr(), intercon.len() as c_uint);

    match (*alc5623).id {
        0x21 | 0x22 => {
            snd_soc_dapm_new_controls(dapm, alc5623_dapm_amp_widgets.as_ptr(), alc5623_dapm_amp_widgets.len() as c_uint);
            snd_soc_dapm_add_routes(dapm, intercon_amp_spk.as_ptr(), intercon_amp_spk.len() as c_uint);
        }
        0x23 => {
            snd_soc_dapm_add_routes(dapm, intercon_spk.as_ptr(), intercon_spk.len() as c_uint);
        }
        _ => return -EINVAL,
    }

    0
}

static soc_component_device_alc5623: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(alc5623_probe),
    suspend: Some(alc5623_suspend),
    resume: Some(alc5623_resume),
    set_bias_level: Some(alc5623_set_bias_level),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..Default::default()
};

static alc5623_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    reg_stride: 2,
    max_register: ALC5623_VENDOR_ID2,
    cache_type: REGCACHE_RBTREE,
    ..Default::default()
};

static alc5623_i2c_table: &[i2c_device_id] = &[
    i2c_device_id { name: "alc5621", driver_data: 0x21 },
    i2c_device_id { name: "alc5622", driver_data: 0x22 },
    i2c_device_id { name: "alc5623", driver_data: 0x23 },
    i2c_device_id::default(),
];
MODULE_DEVICE_TABLE!(i2c, alc5623_i2c_table);

/*
 * ALC5623 2 wire address is determined by A1 pin
 * state during powerup.
 *    low  = 0x1a
 *    high = 0x1b
 */
unsafe fn alc5623_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut pdata: *mut alc5623_platform_data;
    let alc5623: *mut alc5623_priv;
    let mut np: *mut device_node;
    let mut vid1: c_uint = 0;
    let mut vid2: c_uint = 0;
    let matched_id: c_uint;
    let mut ret: c_int;
    let mut val32: u32 = 0;

    alc5623 = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<alc5623_priv>(), GFP_KERNEL)
        as *mut alc5623_priv;
    if alc5623.is_null() {
        return -ENOMEM;
    }

    (*alc5623).regmap = devm_regmap_init_i2c(client, &alc5623_regmap);
    if IS_ERR((*alc5623).regmap) {
        ret = PTR_ERR((*alc5623).regmap);
        dev_err!(&mut (*client).dev, "Failed to initialise I/O: %d\n", ret);
        return ret;
    }

    ret = regmap_read((*alc5623).regmap, ALC5623_VENDOR_ID1, &mut vid1);
    if ret < 0 {
        dev_err!(&mut (*client).dev, "failed to read vendor ID1: %d\n", ret);
        return ret;
    }

    ret = regmap_read((*alc5623).regmap, ALC5623_VENDOR_ID2, &mut vid2);
    if ret < 0 {
        dev_err!(&mut (*client).dev, "failed to read vendor ID2: %d\n", ret);
        return ret;
    }
    vid2 >>= 8;

    matched_id = i2c_get_match_data(client) as uintptr_t as c_uint;

    if vid1 != 0x10ec || vid2 != matched_id {
        dev_err!(&mut (*client).dev, "unknown or wrong codec\n");
        dev_err!(&mut (*client).dev, "Expected %x:%x, got %x:%x\n",
            0x10ec, matched_id, vid1, vid2);
        return -ENODEV;
    }

    dev_dbg!(&mut (*client).dev, "Found codec id : alc56%02x\n", vid2);

    pdata = (*client).dev.platform_data as *mut alc5623_platform_data;
    if !pdata.is_null() {
        (*alc5623).add_ctrl = (*pdata).add_ctrl;
        (*alc5623).jack_det_ctrl = (*pdata).jack_det_ctrl;
    } else if !(*client).dev.of_node.is_null() {
        np = (*client).dev.of_node;
        ret = of_property_read_u32(np, "add-ctrl", &mut val32);
        if ret == 0 {
            (*alc5623).add_ctrl = val32;
        }
        ret = of_property_read_u32(np, "jack-det-ctrl", &mut val32);
        if ret == 0 {
            (*alc5623).jack_det_ctrl = val32;
        }
    }

    (*alc5623).id = vid2 as u8;
    match (*alc5623).id {
        0x21 => alc5623_dai.name = "alc5621-hifi",
        0x22 => alc5623_dai.name = "alc5622-hifi",
        0x23 => alc5623_dai.name = "alc5623-hifi",
        _ => return -EINVAL,
    }

    i2c_set_clientdata(client, alc5623 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*client).dev,
        &soc_component_device_alc5623,
        &mut alc5623_dai,
        1,
    );
    if ret != 0 {
        dev_err!(&mut (*client).dev, "Failed to register codec: %d\n", ret);
    }

    ret
}

/* CONFIG_OF: Open Firmware match table is present when enabled by the kernel build. */
static alc5623_of_match: &[of_device_id] = &[
    of_device_id { compatible: "realtek,alc5623", ..Default::default() },
    of_device_id::default(),
];
MODULE_DEVICE_TABLE!(of, alc5623_of_match);

/*  i2c codec control layer */
static mut alc5623_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: "alc562x-codec",
        of_match_table: of_match_ptr!(alc5623_of_match),
        ..Default::default()
    },
    probe: Some(alc5623_i2c_probe),
    id_table: alc5623_i2c_table.as_ptr(),
    ..Default::default()
};

module_i2c_driver!(alc5623_i2c_driver);

MODULE_DESCRIPTION!("ASoC alc5621/2/3 driver");
MODULE_AUTHOR!("Arnaud Patard <arnaud.patard@rtp-net.org>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
