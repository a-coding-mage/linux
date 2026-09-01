// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs42l52.c -- CS42L52 ALSA SoC audio driver
 *
 * Copyright 2012 CirrusLogic, Inc.
 *
 * Author: Georgi Vlaev <joe@nucleusys.com>
 * Author: Brian Austin <brian.austin@cirrus.com>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type u8 = u8;
type u32 = u32;
type size_t = usize;
type ssize_t = isize;

#[repr(C)]
pub struct sp_config {
    pub spc: u8,
    pub format: u8,
    pub spfs: u8,
    pub srate: u32,
}

#[repr(C)]
pub struct cs42l52_platform_data {
    /* MICBIAS Level. Check datasheet Pg48 */
    pub micbias_lvl: c_uint,

    /* MICA mode selection Differential or Single-ended */
    pub mica_diff_cfg: bool,

    /* MICB mode selection Differential or Single-ended */
    pub micb_diff_cfg: bool,

    /* Charge Pump Freq. Check datasheet Pg73 */
    pub chgfreq: c_uint,

    /* Reset GPIO */
    pub reset_gpio: *mut gpio_desc,
}

#[repr(C)]
pub struct cs42l52_private {
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
    pub config: sp_config,
    pub pdata: cs42l52_platform_data,
    pub sysclk: u32,
    pub mclksel: u8,
    pub mclk: u32,
    pub flags: u8,
    pub beep: *mut input_dev,
    pub beep_work: work_struct,
    pub beep_rate: c_int,
}

static cs42l52_reg_defaults: &[reg_default] = &[
    reg_default { reg: CS42L52_PWRCTL1, def: 0x9F }, /* r02 PWRCTL 1 */
    reg_default { reg: CS42L52_PWRCTL2, def: 0x07 }, /* r03 PWRCTL 2 */
    reg_default { reg: CS42L52_PWRCTL3, def: 0xFF }, /* r04 PWRCTL 3 */
    reg_default { reg: CS42L52_CLK_CTL, def: 0xA0 }, /* r05 Clocking Ctl */
    reg_default { reg: CS42L52_IFACE_CTL1, def: 0x00 }, /* r06 Interface Ctl 1 */
    reg_default { reg: CS42L52_ADC_PGA_A, def: 0x80 }, /* r08 Input A Select */
    reg_default { reg: CS42L52_ADC_PGA_B, def: 0x80 }, /* r09 Input B Select */
    reg_default { reg: CS42L52_ANALOG_HPF_CTL, def: 0xA5 }, /* r0A Analog HPF Ctl */
    reg_default { reg: CS42L52_ADC_HPF_FREQ, def: 0x00 }, /* r0B ADC HPF Corner Freq */
    reg_default { reg: CS42L52_ADC_MISC_CTL, def: 0x00 }, /* r0C Misc. ADC Ctl */
    reg_default { reg: CS42L52_PB_CTL1, def: 0x60 }, /* r0D Playback Ctl 1 */
    reg_default { reg: CS42L52_MISC_CTL, def: 0x02 }, /* r0E Misc. Ctl */
    reg_default { reg: CS42L52_PB_CTL2, def: 0x00 }, /* r0F Playback Ctl 2 */
    reg_default { reg: CS42L52_MICA_CTL, def: 0x00 }, /* r10 MICA Amp Ctl */
    reg_default { reg: CS42L52_MICB_CTL, def: 0x00 }, /* r11 MICB Amp Ctl */
    reg_default { reg: CS42L52_PGAA_CTL, def: 0x00 }, /* r12 PGAA Vol, Misc. */
    reg_default { reg: CS42L52_PGAB_CTL, def: 0x00 }, /* r13 PGAB Vol, Misc. */
    reg_default { reg: CS42L52_PASSTHRUA_VOL, def: 0x00 }, /* r14 Bypass A Vol */
    reg_default { reg: CS42L52_PASSTHRUB_VOL, def: 0x00 }, /* r15 Bypass B Vol */
    reg_default { reg: CS42L52_ADCA_VOL, def: 0x00 }, /* r16 ADCA Volume */
    reg_default { reg: CS42L52_ADCB_VOL, def: 0x00 }, /* r17 ADCB Volume */
    reg_default { reg: CS42L52_ADCA_MIXER_VOL, def: 0x80 }, /* r18 ADCA Mixer Volume */
    reg_default { reg: CS42L52_ADCB_MIXER_VOL, def: 0x80 }, /* r19 ADCB Mixer Volume */
    reg_default { reg: CS42L52_PCMA_MIXER_VOL, def: 0x00 }, /* r1A PCMA Mixer Volume */
    reg_default { reg: CS42L52_PCMB_MIXER_VOL, def: 0x00 }, /* r1B PCMB Mixer Volume */
    reg_default { reg: CS42L52_BEEP_FREQ, def: 0x00 }, /* r1C Beep Freq on Time */
    reg_default { reg: CS42L52_BEEP_VOL, def: 0x00 }, /* r1D Beep Volume off Time */
    reg_default { reg: CS42L52_BEEP_TONE_CTL, def: 0x00 }, /* r1E Beep Tone Cfg. */
    reg_default { reg: CS42L52_TONE_CTL, def: 0x00 }, /* r1F Tone Ctl */
    reg_default { reg: CS42L52_MASTERA_VOL, def: 0x00 }, /* r20 Master A Volume */
    reg_default { reg: CS42L52_MASTERB_VOL, def: 0x00 }, /* r21 Master B Volume */
    reg_default { reg: CS42L52_HPA_VOL, def: 0x00 }, /* r22 Headphone A Volume */
    reg_default { reg: CS42L52_HPB_VOL, def: 0x00 }, /* r23 Headphone B Volume */
    reg_default { reg: CS42L52_SPKA_VOL, def: 0x00 }, /* r24 Speaker A Volume */
    reg_default { reg: CS42L52_SPKB_VOL, def: 0x00 }, /* r25 Speaker B Volume */
    reg_default { reg: CS42L52_ADC_PCM_MIXER, def: 0x00 }, /* r26 Channel Mixer and Swap */
    reg_default { reg: CS42L52_LIMITER_CTL1, def: 0x00 }, /* r27 Limit Ctl 1 Thresholds */
    reg_default { reg: CS42L52_LIMITER_CTL2, def: 0x7F }, /* r28 Limit Ctl 2 Release Rate */
    reg_default { reg: CS42L52_LIMITER_AT_RATE, def: 0xC0 }, /* r29 Limiter Attack Rate */
    reg_default { reg: CS42L52_ALC_CTL, def: 0x00 }, /* r2A ALC Ctl 1 Attack Rate */
    reg_default { reg: CS42L52_ALC_RATE, def: 0x3F }, /* r2B ALC Release Rate */
    reg_default { reg: CS42L52_ALC_THRESHOLD, def: 0x3f }, /* r2C ALC Thresholds */
    reg_default { reg: CS42L52_NOISE_GATE_CTL, def: 0x00 }, /* r2D Noise Gate Ctl */
    reg_default { reg: CS42L52_CLK_STATUS, def: 0x00 }, /* r2E Overflow and Clock Status */
    reg_default { reg: CS42L52_BATT_COMPEN, def: 0x00 }, /* r2F battery Compensation */
    reg_default { reg: CS42L52_BATT_LEVEL, def: 0x00 }, /* r30 VP Battery Level */
    reg_default { reg: CS42L52_SPK_STATUS, def: 0x00 }, /* r31 Speaker Status */
    reg_default { reg: CS42L52_TEM_CTL, def: 0x3B }, /* r32 Temp Ctl */
    reg_default { reg: CS42L52_THE_FOLDBACK, def: 0x00 }, /* r33 Foldback */
];

unsafe extern "C" fn cs42l52_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS42L52_CHIP..=CS42L52_CHARGE_PUMP => true,
        _ => false,
    }
}

unsafe extern "C" fn cs42l52_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS42L52_IFACE_CTL2 | CS42L52_CLK_STATUS | CS42L52_BATT_LEVEL
        | CS42L52_SPK_STATUS | CS42L52_CHARGE_PUMP => true,
        _ => false,
    }
}

static hl_tlv: TLV = DECLARE_TLV_DB_SCALE!(-10200, 50, 0);
static hpd_tlv: TLV = DECLARE_TLV_DB_SCALE!(-9600, 50, 1);
static ipd_tlv: TLV = DECLARE_TLV_DB_SCALE!(-9600, 100, 0);
static mic_tlv: TLV = DECLARE_TLV_DB_SCALE!(1600, 100, 0);
static pga_tlv: TLV = DECLARE_TLV_DB_SCALE!(-600, 50, 0);
static pass_tlv: TLV = DECLARE_TLV_DB_SCALE!(-6000, 50, 0);
static mix_tlv: TLV = DECLARE_TLV_DB_SCALE!(-5150, 50, 0);
static beep_tlv: TLV = DECLARE_TLV_DB_SCALE!(-56, 200, 0);
static limiter_tlv: TLV = DECLARE_TLV_DB_RANGE!(
    0, 2, TLV_DB_SCALE_ITEM!(-3000, 600, 0),
    3, 7, TLV_DB_SCALE_ITEM!(-1200, 300, 0)
);

static cs42l52_adca_text: [&[u8]; 5] = [b"Input1A\0", b"Input2A\0", b"Input3A\0", b"Input4A\0", b"PGA Input Left\0"];
static cs42l52_adcb_text: [&[u8]; 5] = [b"Input1B\0", b"Input2B\0", b"Input3B\0", b"Input4B\0", b"PGA Input Right\0"];
static adca_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_ADC_PGA_A, 5, cs42l52_adca_text);
static adcb_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_ADC_PGA_B, 5, cs42l52_adcb_text);
static adca_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Left ADC Input Capture Mux", adca_enum);
static adcb_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Right ADC Input Capture Mux", adcb_enum);

static mic_bias_level_text: [&[u8]; 6] = [b"0.5 +VA\0", b"0.6 +VA\0", b"0.7 +VA\0", b"0.8 +VA\0", b"0.83 +VA\0", b"0.91 +VA\0"];
static mic_bias_level_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_IFACE_CTL2, 0, mic_bias_level_text);
static cs42l52_mic_text: [&[u8]; 2] = [b"MIC1\0", b"MIC2\0"];
static mica_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_MICA_CTL, 5, cs42l52_mic_text);
static micb_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_MICB_CTL, 5, cs42l52_mic_text);
static digital_output_mux_text: [&[u8]; 2] = [b"ADC\0", b"DSP\0"];
static digital_output_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_ADC_MISC_CTL, 6, digital_output_mux_text);
static digital_output_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Digital Output Mux", digital_output_mux_enum);
static hp_gain_num_text: [&[u8]; 8] = [b"0.3959\0", b"0.4571\0", b"0.5111\0", b"0.6047\0", b"0.7099\0", b"0.8399\0", b"1.000\0", b"1.1430\0"];
static hp_gain_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_PB_CTL1, 5, hp_gain_num_text);
static beep_pitch_text: [&[u8]; 16] = [b"C4\0", b"C5\0", b"D5\0", b"E5\0", b"F5\0", b"G5\0", b"A5\0", b"B5\0", b"C6\0", b"D6\0", b"E6\0", b"F6\0", b"G6\0", b"A6\0", b"B6\0", b"C7\0"];
static beep_pitch_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_BEEP_FREQ, 4, beep_pitch_text);
static beep_ontime_text: [&[u8]; 16] = [b"86 ms\0", b"430 ms\0", b"780 ms\0", b"1.20 s\0", b"1.50 s\0", b"1.80 s\0", b"2.20 s\0", b"2.50 s\0", b"2.80 s\0", b"3.20 s\0", b"3.50 s\0", b"3.80 s\0", b"4.20 s\0", b"4.50 s\0", b"4.80 s\0", b"5.20 s\0"];
static beep_ontime_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_BEEP_FREQ, 0, beep_ontime_text);
static beep_offtime_text: [&[u8]; 8] = [b"1.23 s\0", b"2.58 s\0", b"3.90 s\0", b"5.20 s\0", b"6.60 s\0", b"8.05 s\0", b"9.35 s\0", b"10.80 s\0"];
static beep_offtime_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_BEEP_VOL, 5, beep_offtime_text);
static beep_config_text: [&[u8]; 4] = [b"Off\0", b"Single\0", b"Multiple\0", b"Continuous\0"];
static beep_config_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_BEEP_TONE_CTL, 6, beep_config_text);
static beep_bass_text: [&[u8]; 4] = [b"50 Hz\0", b"100 Hz\0", b"200 Hz\0", b"250 Hz\0"];
static beep_bass_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_BEEP_TONE_CTL, 1, beep_bass_text);
static beep_treble_text: [&[u8]; 4] = [b"5 kHz\0", b"7 kHz\0", b"10 kHz\0", b" 15 kHz\0"];
static beep_treble_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_BEEP_TONE_CTL, 3, beep_treble_text);
static ng_threshold_text: [&[u8]; 8] = [b"-34dB\0", b"-37dB\0", b"-40dB\0", b"-43dB\0", b"-46dB\0", b"-52dB\0", b"-58dB\0", b"-64dB\0"];
static ng_threshold_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_NOISE_GATE_CTL, 2, ng_threshold_text);
static cs42l52_ng_delay_text: [&[u8]; 4] = [b"50ms\0", b"100ms\0", b"150ms\0", b"200ms\0"];
static ng_delay_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_NOISE_GATE_CTL, 0, cs42l52_ng_delay_text);
static cs42l52_ng_type_text: [&[u8]; 2] = [b"Apply Specific\0", b"Apply All\0"];
static ng_type_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(CS42L52_NOISE_GATE_CTL, 6, cs42l52_ng_type_text);
static left_swap_text: [&[u8]; 3] = [b"Left\0", b"LR 2\0", b"Right\0"];
static right_swap_text: [&[u8]; 3] = [b"Right\0", b"LR 2\0", b"Left\0"];
static swap_values: [c_uint; 3] = [0, 1, 3];
static adca_swap_enum: soc_enum = SOC_VALUE_ENUM_SINGLE!(CS42L52_ADC_PCM_MIXER, 2, 3, left_swap_text.len(), left_swap_text, swap_values);
static adca_mixer: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", adca_swap_enum);
static pcma_swap_enum: soc_enum = SOC_VALUE_ENUM_SINGLE!(CS42L52_ADC_PCM_MIXER, 6, 3, left_swap_text.len(), left_swap_text, swap_values);
static pcma_mixer: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", pcma_swap_enum);
static adcb_swap_enum: soc_enum = SOC_VALUE_ENUM_SINGLE!(CS42L52_ADC_PCM_MIXER, 0, 3, right_swap_text.len(), right_swap_text, swap_values);
static adcb_mixer: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", adcb_swap_enum);
static pcmb_swap_enum: soc_enum = SOC_VALUE_ENUM_SINGLE!(CS42L52_ADC_PCM_MIXER, 4, 3, right_swap_text.len(), right_swap_text, swap_values);
static pcmb_mixer: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", pcmb_swap_enum);

static passthrul_ctl: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CS42L52_MISC_CTL, 6, 1, 0);
static passthrur_ctl: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CS42L52_MISC_CTL, 7, 1, 0);
static spkl_ctl: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CS42L52_PWRCTL3, 0, 1, 1);
static spkr_ctl: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CS42L52_PWRCTL3, 2, 1, 1);
static hpl_ctl: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CS42L52_PWRCTL3, 4, 1, 1);
static hpr_ctl: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CS42L52_PWRCTL3, 6, 1, 1);

static cs42l52_snd_controls: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_R_SX_TLV!("Master Volume", CS42L52_MASTERA_VOL, CS42L52_MASTERB_VOL, 0, 0x34, 0xE4, hl_tlv),
    SOC_DOUBLE_R_SX_TLV!("Headphone Volume", CS42L52_HPA_VOL, CS42L52_HPB_VOL, 0, 0x34, 0xC0, hpd_tlv),
    SOC_ENUM!("Headphone Analog Gain", hp_gain_enum),
    SOC_DOUBLE_R_SX_TLV!("Speaker Volume", CS42L52_SPKA_VOL, CS42L52_SPKB_VOL, 0, 0x40, 0xC0, hl_tlv),
    SOC_DOUBLE_R_SX_TLV!("Bypass Volume", CS42L52_PASSTHRUA_VOL, CS42L52_PASSTHRUB_VOL, 0, 0x88, 0x90, pass_tlv),
    SOC_DOUBLE!("Bypass Mute", CS42L52_MISC_CTL, 4, 5, 1, 0),
    SOC_DOUBLE_R_TLV!("MIC Gain Volume", CS42L52_MICA_CTL, CS42L52_MICB_CTL, 0, 0x10, 0, mic_tlv),
    SOC_ENUM!("MIC Bias Level", mic_bias_level_enum),
    SOC_DOUBLE_R_SX_TLV!("ADC Volume", CS42L52_ADCA_VOL, CS42L52_ADCB_VOL, 0, 0xA0, 0x78, ipd_tlv),
    SOC_DOUBLE_R_SX_TLV!("ADC Mixer Volume", CS42L52_ADCA_MIXER_VOL, CS42L52_ADCB_MIXER_VOL, 0, 0x19, 0x7F, mix_tlv),
    SOC_DOUBLE!("ADC Switch", CS42L52_ADC_MISC_CTL, 0, 1, 1, 0),
    SOC_DOUBLE_R!("ADC Mixer Switch", CS42L52_ADCA_MIXER_VOL, CS42L52_ADCB_MIXER_VOL, 7, 1, 1),
    SOC_DOUBLE_R_SX_TLV!("PGA Volume", CS42L52_PGAA_CTL, CS42L52_PGAB_CTL, 0, 0x28, 0x24, pga_tlv),
    SOC_DOUBLE_R_SX_TLV!("PCM Mixer Volume", CS42L52_PCMA_MIXER_VOL, CS42L52_PCMB_MIXER_VOL, 0, 0x19, 0x7f, mix_tlv),
    SOC_DOUBLE_R!("PCM Mixer Switch", CS42L52_PCMA_MIXER_VOL, CS42L52_PCMB_MIXER_VOL, 7, 1, 1),
    SOC_ENUM!("Beep Config", beep_config_enum),
    SOC_ENUM!("Beep Pitch", beep_pitch_enum),
    SOC_ENUM!("Beep on Time", beep_ontime_enum),
    SOC_ENUM!("Beep off Time", beep_offtime_enum),
    SOC_SINGLE_SX_TLV!("Beep Volume", CS42L52_BEEP_VOL, 0, 0x07, 0x1f, beep_tlv),
    SOC_SINGLE!("Beep Mixer Switch", CS42L52_BEEP_TONE_CTL, 5, 1, 1),
    SOC_ENUM!("Beep Treble Corner Freq", beep_treble_enum),
    SOC_ENUM!("Beep Bass Corner Freq", beep_bass_enum),
    SOC_SINGLE!("Tone Control Switch", CS42L52_BEEP_TONE_CTL, 0, 1, 1),
    SOC_SINGLE_TLV!("Treble Gain Volume", CS42L52_TONE_CTL, 4, 15, 1, hl_tlv),
    SOC_SINGLE_TLV!("Bass Gain Volume", CS42L52_TONE_CTL, 0, 15, 1, hl_tlv),
    /* Limiter */
    SOC_SINGLE_TLV!("Limiter Max Threshold Volume", CS42L52_LIMITER_CTL1, 5, 7, 0, limiter_tlv),
    SOC_SINGLE_TLV!("Limiter Cushion Threshold Volume", CS42L52_LIMITER_CTL1, 2, 7, 0, limiter_tlv),
    SOC_SINGLE_TLV!("Limiter Release Rate Volume", CS42L52_LIMITER_CTL2, 0, 63, 0, limiter_tlv),
    SOC_SINGLE_TLV!("Limiter Attack Rate Volume", CS42L52_LIMITER_AT_RATE, 0, 63, 0, limiter_tlv),
    SOC_SINGLE!("Limiter SR Switch", CS42L52_LIMITER_CTL1, 1, 1, 0),
    SOC_SINGLE!("Limiter ZC Switch", CS42L52_LIMITER_CTL1, 0, 1, 0),
    SOC_SINGLE!("Limiter Switch", CS42L52_LIMITER_CTL2, 7, 1, 0),
    /* ALC */
    SOC_SINGLE_TLV!("ALC Attack Rate Volume", CS42L52_ALC_CTL, 0, 63, 0, limiter_tlv),
    SOC_SINGLE_TLV!("ALC Release Rate Volume", CS42L52_ALC_RATE, 0, 63, 0, limiter_tlv),
    SOC_SINGLE_TLV!("ALC Max Threshold Volume", CS42L52_ALC_THRESHOLD, 5, 7, 0, limiter_tlv),
    SOC_SINGLE_TLV!("ALC Min Threshold Volume", CS42L52_ALC_THRESHOLD, 2, 7, 0, limiter_tlv),
    SOC_DOUBLE_R!("ALC SR Capture Switch", CS42L52_PGAA_CTL, CS42L52_PGAB_CTL, 7, 1, 1),
    SOC_DOUBLE_R!("ALC ZC Capture Switch", CS42L52_PGAA_CTL, CS42L52_PGAB_CTL, 6, 1, 1),
    SOC_DOUBLE!("ALC Capture Switch", CS42L52_ALC_CTL, 6, 7, 1, 0),
    /* Noise gate */
    SOC_ENUM!("NG Type Switch", ng_type_enum),
    SOC_SINGLE!("NG Enable Switch", CS42L52_NOISE_GATE_CTL, 6, 1, 0),
    SOC_SINGLE!("NG Boost Switch", CS42L52_NOISE_GATE_CTL, 5, 1, 1),
    SOC_ENUM!("NG Threshold", ng_threshold_enum),
    SOC_ENUM!("NG Delay", ng_delay_enum),
    SOC_DOUBLE!("HPF Switch", CS42L52_ANALOG_HPF_CTL, 5, 7, 1, 0),
    SOC_DOUBLE!("Analog SR Switch", CS42L52_ANALOG_HPF_CTL, 1, 3, 1, 1),
    SOC_DOUBLE!("Analog ZC Switch", CS42L52_ANALOG_HPF_CTL, 0, 2, 1, 1),
    SOC_SINGLE!("Digital SR Switch", CS42L52_MISC_CTL, 1, 1, 0),
    SOC_SINGLE!("Digital ZC Switch", CS42L52_MISC_CTL, 0, 1, 0),
    SOC_SINGLE!("Deemphasis Switch", CS42L52_MISC_CTL, 2, 1, 0),
    SOC_SINGLE!("Batt Compensation Switch", CS42L52_BATT_COMPEN, 7, 1, 0),
    SOC_SINGLE!("Batt VP Monitor Switch", CS42L52_BATT_COMPEN, 6, 1, 0),
    SOC_SINGLE!("Batt VP ref", CS42L52_BATT_COMPEN, 0, 0x0f, 0),
    SOC_SINGLE!("PGA AIN1L Switch", CS42L52_ADC_PGA_A, 0, 1, 0),
    SOC_SINGLE!("PGA AIN1R Switch", CS42L52_ADC_PGA_B, 0, 1, 0),
    SOC_SINGLE!("PGA AIN2L Switch", CS42L52_ADC_PGA_A, 1, 1, 0),
    SOC_SINGLE!("PGA AIN2R Switch", CS42L52_ADC_PGA_B, 1, 1, 0),
    SOC_SINGLE!("PGA AIN3L Switch", CS42L52_ADC_PGA_A, 2, 1, 0),
    SOC_SINGLE!("PGA AIN3R Switch", CS42L52_ADC_PGA_B, 2, 1, 0),
    SOC_SINGLE!("PGA AIN4L Switch", CS42L52_ADC_PGA_A, 3, 1, 0),
    SOC_SINGLE!("PGA AIN4R Switch", CS42L52_ADC_PGA_B, 3, 1, 0),
    SOC_SINGLE!("PGA MICA Switch", CS42L52_ADC_PGA_A, 4, 1, 0),
    SOC_SINGLE!("PGA MICB Switch", CS42L52_ADC_PGA_B, 4, 1, 0),
];

static cs42l52_mica_controls: &[snd_kcontrol_new] = &[SOC_ENUM!("MICA Select", mica_enum)];
static cs42l52_micb_controls: &[snd_kcontrol_new] = &[SOC_ENUM!("MICB Select", micb_enum)];

unsafe extern "C" fn cs42l52_add_mic_controls(component: *mut snd_soc_component) -> c_int {
    let cs42l52 = snd_soc_component_get_drvdata(component) as *mut cs42l52_private;
    let pdata = &mut (*cs42l52).pdata as *mut cs42l52_platform_data;
    if !(*pdata).mica_diff_cfg {
        snd_soc_add_component_controls(component, cs42l52_mica_controls.as_ptr(), cs42l52_mica_controls.len() as c_uint);
    }
    if !(*pdata).micb_diff_cfg {
        snd_soc_add_component_controls(component, cs42l52_micb_controls.as_ptr(), cs42l52_micb_controls.len() as c_uint);
    }
    0
}

static cs42l52_dapm_widgets: &[snd_soc_dapm_widget] = SND_SOC_DAPM_WIDGETS! {
    SND_SOC_DAPM_INPUT!("AIN1L"), SND_SOC_DAPM_INPUT!("AIN1R"),
    SND_SOC_DAPM_INPUT!("AIN2L"), SND_SOC_DAPM_INPUT!("AIN2R"),
    SND_SOC_DAPM_INPUT!("AIN3L"), SND_SOC_DAPM_INPUT!("AIN3R"),
    SND_SOC_DAPM_INPUT!("AIN4L"), SND_SOC_DAPM_INPUT!("AIN4R"),
    SND_SOC_DAPM_INPUT!("MICA"), SND_SOC_DAPM_INPUT!("MICB"),
    SND_SOC_DAPM_SIGGEN!("Beep"),
    SND_SOC_DAPM_AIF_OUT!("AIFOUTL", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("AIFOUTR", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!("ADC Left", core::ptr::null(), CS42L52_PWRCTL1, 1, 1),
    SND_SOC_DAPM_ADC!("ADC Right", core::ptr::null(), CS42L52_PWRCTL1, 2, 1),
    SND_SOC_DAPM_PGA!("PGA Left", CS42L52_PWRCTL1, 3, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("PGA Right", CS42L52_PWRCTL1, 4, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_MUX!("ADC Left Mux", SND_SOC_NOPM, 0, 0, &adca_mux),
    SND_SOC_DAPM_MUX!("ADC Right Mux", SND_SOC_NOPM, 0, 0, &adcb_mux),
    SND_SOC_DAPM_MUX!("ADC Left Swap", SND_SOC_NOPM, 0, 0, &adca_mixer),
    SND_SOC_DAPM_MUX!("ADC Right Swap", SND_SOC_NOPM, 0, 0, &adcb_mixer),
    SND_SOC_DAPM_MUX!("Output Mux", SND_SOC_NOPM, 0, 0, &digital_output_mux),
    SND_SOC_DAPM_PGA!("PGA MICA", CS42L52_PWRCTL2, 1, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("PGA MICB", CS42L52_PWRCTL2, 2, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Bias", CS42L52_PWRCTL2, 0, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Charge Pump", CS42L52_PWRCTL1, 7, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_AIF_IN!("AIFINL", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("AIFINR", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC!("DAC Left", core::ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC!("DAC Right", core::ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_SWITCH!("Bypass Left", CS42L52_MISC_CTL, 6, 0, &passthrul_ctl),
    SND_SOC_DAPM_SWITCH!("Bypass Right", CS42L52_MISC_CTL, 7, 0, &passthrur_ctl),
    SND_SOC_DAPM_MUX!("PCM Left Swap", SND_SOC_NOPM, 0, 0, &pcma_mixer),
    SND_SOC_DAPM_MUX!("PCM Right Swap", SND_SOC_NOPM, 0, 0, &pcmb_mixer),
    SND_SOC_DAPM_SWITCH!("HP Left Amp", SND_SOC_NOPM, 0, 0, &hpl_ctl),
    SND_SOC_DAPM_SWITCH!("HP Right Amp", SND_SOC_NOPM, 0, 0, &hpr_ctl),
    SND_SOC_DAPM_SWITCH!("SPK Left Amp", SND_SOC_NOPM, 0, 0, &spkl_ctl),
    SND_SOC_DAPM_SWITCH!("SPK Right Amp", SND_SOC_NOPM, 0, 0, &spkr_ctl),
    SND_SOC_DAPM_OUTPUT!("HPOUTA"), SND_SOC_DAPM_OUTPUT!("HPOUTB"),
    SND_SOC_DAPM_OUTPUT!("SPKOUTA"), SND_SOC_DAPM_OUTPUT!("SPKOUTB"),
};

static cs42l52_audio_map: &[snd_soc_dapm_route] = &[
    route!("Capture", None, "AIFOUTL"), route!("Capture", None, "AIFOUTL"),
    route!("AIFOUTL", None, "Output Mux"), route!("AIFOUTR", None, "Output Mux"),
    route!("Output Mux", Some("ADC"), "ADC Left"), route!("Output Mux", Some("ADC"), "ADC Right"),
    route!("ADC Left", None, "Charge Pump"), route!("ADC Right", None, "Charge Pump"),
    route!("Charge Pump", None, "ADC Left Mux"), route!("Charge Pump", None, "ADC Right Mux"),
    route!("ADC Left Mux", Some("Input1A"), "AIN1L"), route!("ADC Right Mux", Some("Input1B"), "AIN1R"),
    route!("ADC Left Mux", Some("Input2A"), "AIN2L"), route!("ADC Right Mux", Some("Input2B"), "AIN2R"),
    route!("ADC Left Mux", Some("Input3A"), "AIN3L"), route!("ADC Right Mux", Some("Input3B"), "AIN3R"),
    route!("ADC Left Mux", Some("Input4A"), "AIN4L"), route!("ADC Right Mux", Some("Input4B"), "AIN4R"),
    route!("ADC Left Mux", Some("PGA Input Left"), "PGA Left"), route!("ADC Right Mux", Some("PGA Input Right"), "PGA Right"),
    route!("PGA Left", Some("Switch"), "AIN1L"), route!("PGA Right", Some("Switch"), "AIN1R"),
    route!("PGA Left", Some("Switch"), "AIN2L"), route!("PGA Right", Some("Switch"), "AIN2R"),
    route!("PGA Left", Some("Switch"), "AIN3L"), route!("PGA Right", Some("Switch"), "AIN3R"),
    route!("PGA Left", Some("Switch"), "AIN4L"), route!("PGA Right", Some("Switch"), "AIN4R"),
    route!("PGA Left", Some("Switch"), "PGA MICA"), route!("PGA MICA", None, "MICA"),
    route!("PGA Right", Some("Switch"), "PGA MICB"), route!("PGA MICB", None, "MICB"),
    route!("HPOUTA", None, "HP Left Amp"), route!("HPOUTB", None, "HP Right Amp"),
    route!("HP Left Amp", None, "Bypass Left"), route!("HP Right Amp", None, "Bypass Right"),
    route!("Bypass Left", Some("Switch"), "PGA Left"), route!("Bypass Right", Some("Switch"), "PGA Right"),
    route!("HP Left Amp", Some("Switch"), "DAC Left"), route!("HP Right Amp", Some("Switch"), "DAC Right"),
    route!("SPKOUTA", None, "SPK Left Amp"), route!("SPKOUTB", None, "SPK Right Amp"),
    route!("SPK Left Amp", None, "Beep"), route!("SPK Right Amp", None, "Beep"),
    route!("SPK Left Amp", Some("Switch"), "Playback"), route!("SPK Right Amp", Some("Switch"), "Playback"),
    route!("DAC Left", None, "Beep"), route!("DAC Right", None, "Beep"),
    route!("DAC Left", None, "Playback"), route!("DAC Right", None, "Playback"),
    route!("Output Mux", Some("DSP"), "Playback"), route!("Output Mux", Some("DSP"), "Playback"),
    route!("AIFINL", None, "Playback"), route!("AIFINR", None, "Playback"),
];

#[repr(C)]
pub struct cs42l52_clk_para {
    pub mclk: u32,
    pub rate: u32,
    pub speed: u8,
    pub group: u8,
    pub videoclk: u8,
    pub ratio: u8,
    pub mclkdiv2: u8,
}

static clk_map_table: &[cs42l52_clk_para] = &[
    /*8k*/
    cs42l52_clk_para { mclk: 12288000, rate: 8000, speed: CLK_QS_MODE, group: CLK_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 18432000, rate: 8000, speed: CLK_QS_MODE, group: CLK_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 12000000, rate: 8000, speed: CLK_QS_MODE, group: CLK_32K, videoclk: CLK_NO_27M, ratio: CLK_R_125, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 24000000, rate: 8000, speed: CLK_QS_MODE, group: CLK_32K, videoclk: CLK_NO_27M, ratio: CLK_R_125, mclkdiv2: 1 },
    cs42l52_clk_para { mclk: 27000000, rate: 8000, speed: CLK_QS_MODE, group: CLK_32K, videoclk: CLK_27M_MCLK, ratio: CLK_R_125, mclkdiv2: 0 },
    /*11.025k*/
    cs42l52_clk_para { mclk: 11289600, rate: 11025, speed: CLK_QS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 16934400, rate: 11025, speed: CLK_QS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    /*16k*/
    cs42l52_clk_para { mclk: 12288000, rate: 16000, speed: CLK_HS_MODE, group: CLK_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 18432000, rate: 16000, speed: CLK_HS_MODE, group: CLK_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 12000000, rate: 16000, speed: CLK_HS_MODE, group: CLK_32K, videoclk: CLK_NO_27M, ratio: CLK_R_125, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 24000000, rate: 16000, speed: CLK_HS_MODE, group: CLK_32K, videoclk: CLK_NO_27M, ratio: CLK_R_125, mclkdiv2: 1 },
    cs42l52_clk_para { mclk: 27000000, rate: 16000, speed: CLK_HS_MODE, group: CLK_32K, videoclk: CLK_27M_MCLK, ratio: CLK_R_125, mclkdiv2: 1 },
    /*22.05k*/
    cs42l52_clk_para { mclk: 11289600, rate: 22050, speed: CLK_HS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 16934400, rate: 22050, speed: CLK_HS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    /* 32k */
    cs42l52_clk_para { mclk: 12288000, rate: 32000, speed: CLK_SS_MODE, group: CLK_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 18432000, rate: 32000, speed: CLK_SS_MODE, group: CLK_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 12000000, rate: 32000, speed: CLK_SS_MODE, group: CLK_32K, videoclk: CLK_NO_27M, ratio: CLK_R_125, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 24000000, rate: 32000, speed: CLK_SS_MODE, group: CLK_32K, videoclk: CLK_NO_27M, ratio: CLK_R_125, mclkdiv2: 1 },
    cs42l52_clk_para { mclk: 27000000, rate: 32000, speed: CLK_SS_MODE, group: CLK_32K, videoclk: CLK_27M_MCLK, ratio: CLK_R_125, mclkdiv2: 0 },
    /* 44.1k */
    cs42l52_clk_para { mclk: 11289600, rate: 44100, speed: CLK_SS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 16934400, rate: 44100, speed: CLK_SS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    /* 48k */
    cs42l52_clk_para { mclk: 12288000, rate: 48000, speed: CLK_SS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 18432000, rate: 48000, speed: CLK_SS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 12000000, rate: 48000, speed: CLK_SS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_125, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 24000000, rate: 48000, speed: CLK_SS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_125, mclkdiv2: 1 },
    cs42l52_clk_para { mclk: 27000000, rate: 48000, speed: CLK_SS_MODE, group: CLK_NO_32K, videoclk: CLK_27M_MCLK, ratio: CLK_R_125, mclkdiv2: 1 },
    /* 88.2k */
    cs42l52_clk_para { mclk: 11289600, rate: 88200, speed: CLK_DS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 16934400, rate: 88200, speed: CLK_DS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    /* 96k */
    cs42l52_clk_para { mclk: 12288000, rate: 96000, speed: CLK_DS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 18432000, rate: 96000, speed: CLK_DS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_128, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 12000000, rate: 96000, speed: CLK_DS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_125, mclkdiv2: 0 },
    cs42l52_clk_para { mclk: 24000000, rate: 96000, speed: CLK_DS_MODE, group: CLK_NO_32K, videoclk: CLK_NO_27M, ratio: CLK_R_125, mclkdiv2: 1 },
];

fn abs_i32(v: c_int) -> c_int {
    if v < 0 { -v } else { v }
}

fn cs42l52_get_clk(mclk: c_int, rate: c_int) -> c_int {
    let mut ret: c_int = -EINVAL;
    let mut mclk2: c_uint = 0;
    for (i, entry) in clk_map_table.iter().enumerate() {
        if entry.rate == rate as u32 {
            let mclk1 = entry.mclk;
            if abs_i32(mclk - mclk1 as c_int) < abs_i32(mclk - mclk2 as c_int) {
                mclk2 = mclk1;
                ret = i as c_int;
            }
        }
    }
    ret
}

unsafe extern "C" fn cs42l52_set_sysclk(codec_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let cs42l52 = snd_soc_component_get_drvdata(component) as *mut cs42l52_private;
    if freq >= CS42L52_MIN_CLK && freq <= CS42L52_MAX_CLK {
        (*cs42l52).sysclk = freq;
    } else {
        dev_err((*component).dev, b"Invalid freq parameter\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn cs42l52_set_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let cs42l52 = snd_soc_component_get_drvdata(component) as *mut cs42l52_private;
    let mut iface: u8 = 0;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => iface = CS42L52_IFACE_CTL1_MASTER,
        SND_SOC_DAIFMT_CBC_CFC => iface = CS42L52_IFACE_CTL1_SLAVE,
        _ => return -EINVAL,
    }
    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => iface |= CS42L52_IFACE_CTL1_ADC_FMT_I2S | CS42L52_IFACE_CTL1_DAC_FMT_I2S,
        SND_SOC_DAIFMT_RIGHT_J => iface |= CS42L52_IFACE_CTL1_DAC_FMT_RIGHT_J,
        SND_SOC_DAIFMT_LEFT_J => iface |= CS42L52_IFACE_CTL1_ADC_FMT_LEFT_J | CS42L52_IFACE_CTL1_DAC_FMT_LEFT_J,
        SND_SOC_DAIFMT_DSP_A => iface |= CS42L52_IFACE_CTL1_DSP_MODE_EN,
        SND_SOC_DAIFMT_DSP_B => {}
        _ => return -EINVAL,
    }
    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF | SND_SOC_DAIFMT_IB_NF => iface |= CS42L52_IFACE_CTL1_INV_SCLK,
        SND_SOC_DAIFMT_NB_IF => {}
        _ => return -EINVAL,
    }
    (*cs42l52).config.format = iface;
    snd_soc_component_write(component, CS42L52_IFACE_CTL1, (*cs42l52).config.format as c_uint);
    0
}

unsafe extern "C" fn cs42l52_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    if mute != 0 {
        snd_soc_component_update_bits(component, CS42L52_PB_CTL1, CS42L52_PB_CTL1_MUTE_MASK, CS42L52_PB_CTL1_MUTE);
    } else {
        snd_soc_component_update_bits(component, CS42L52_PB_CTL1, CS42L52_PB_CTL1_MUTE_MASK, CS42L52_PB_CTL1_UNMUTE);
    }
    0
}

unsafe extern "C" fn cs42l52_pcm_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let cs42l52 = snd_soc_component_get_drvdata(component) as *mut cs42l52_private;
    let mut clk: u32 = 0;
    let index = cs42l52_get_clk((*cs42l52).sysclk as c_int, params_rate(params) as c_int);
    if index >= 0 {
        let entry = &clk_map_table[index as usize];
        (*cs42l52).sysclk = entry.mclk;
        clk |= ((entry.speed as u32) << CLK_SPEED_SHIFT)
            | ((entry.group as u32) << CLK_32K_SR_SHIFT)
            | ((entry.videoclk as u32) << CLK_27M_MCLK_SHIFT)
            | ((entry.ratio as u32) << CLK_RATIO_SHIFT)
            | entry.mclkdiv2 as u32;
        snd_soc_component_write(component, CS42L52_CLK_CTL, clk);
    } else {
        dev_err((*component).dev, b"can't get correct mclk\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn cs42l52_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let cs42l52 = snd_soc_component_get_drvdata(component) as *mut cs42l52_private;
    let dapm = snd_soc_component_to_dapm(component);
    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            snd_soc_component_update_bits(component, CS42L52_PWRCTL1, CS42L52_PWRCTL1_PDN_CODEC, 0);
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                regcache_cache_only((*cs42l52).regmap, false);
                regcache_sync((*cs42l52).regmap);
            }
            snd_soc_component_write(component, CS42L52_PWRCTL1, CS42L52_PWRCTL1_PDN_ALL);
        }
        SND_SOC_BIAS_OFF => {
            snd_soc_component_write(component, CS42L52_PWRCTL1, CS42L52_PWRCTL1_PDN_ALL);
            regcache_cache_only((*cs42l52).regmap, true);
        }
    }
    0
}

const CS42L52_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const CS42L52_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_U16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE | SNDRV_PCM_FMTBIT_U18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_U20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_U24_LE;

static cs42l52_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(cs42l52_pcm_hw_params),
    mute_stream: Some(cs42l52_mute),
    set_fmt: Some(cs42l52_set_fmt),
    set_sysclk: Some(cs42l52_set_sysclk),
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

static mut cs42l52_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"cs42l52\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream { stream_name: b"Playback\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: CS42L52_RATES, formats: CS42L52_FORMATS, ..unsafe { core::mem::zeroed() } },
    capture: snd_soc_pcm_stream { stream_name: b"Capture\0".as_ptr() as *const c_char, channels_min: 1, channels_max: 2, rates: CS42L52_RATES, formats: CS42L52_FORMATS, ..unsafe { core::mem::zeroed() } },
    ops: &cs42l52_ops,
    ..unsafe { core::mem::zeroed() }
};

static beep_rates: [c_int; 16] = [261, 522, 585, 667, 706, 774, 889, 1000, 1043, 1200, 1333, 1412, 1600, 1714, 2000, 2182];

unsafe extern "C" fn cs42l52_beep_work(work: *mut work_struct) {
    let cs42l52 = container_of!(work, cs42l52_private, beep_work);
    let component = (*cs42l52).component;
    let dapm = snd_soc_component_to_dapm(component);
    let mut val: c_int = 0;
    let mut best: c_int = 0;
    if (*cs42l52).beep_rate != 0 {
        for i in 0..beep_rates.len() {
            if abs_i32((*cs42l52).beep_rate - beep_rates[i]) < abs_i32((*cs42l52).beep_rate - beep_rates[best as usize]) {
                best = i as c_int;
            }
        }
        dev_dbg((*component).dev, b"Set beep rate %dHz for requested %dHz\n\0".as_ptr() as *const c_char, beep_rates[best as usize], (*cs42l52).beep_rate);
        val = best << CS42L52_BEEP_RATE_SHIFT;
        snd_soc_dapm_enable_pin(dapm, b"Beep\0".as_ptr() as *const c_char);
    } else {
        dev_dbg((*component).dev, b"Disabling beep\n\0".as_ptr() as *const c_char);
        snd_soc_dapm_disable_pin(dapm, b"Beep\0".as_ptr() as *const c_char);
    }
    snd_soc_component_update_bits(component, CS42L52_BEEP_FREQ, CS42L52_BEEP_RATE_MASK, val as c_uint);
    snd_soc_dapm_sync(dapm);
}

/* For usability define a way of injecting beep events for the device -
 * many systems will not have a keyboard.
 */
unsafe extern "C" fn cs42l52_beep_event(dev: *mut input_dev, _type: c_uint, code: c_uint, mut hz: c_int) -> c_int {
    let component = input_get_drvdata(dev) as *mut snd_soc_component;
    let cs42l52 = snd_soc_component_get_drvdata(component) as *mut cs42l52_private;
    dev_dbg((*component).dev, b"Beep event %x %x\n\0".as_ptr() as *const c_char, code, hz);
    match code {
        SND_BELL => {
            if hz != 0 {
                hz = 261;
            }
        }
        SND_TONE => {}
        _ => return -1,
    }
    /* Kick the beep from a workqueue */
    (*cs42l52).beep_rate = hz;
    schedule_work(&mut (*cs42l52).beep_work);
    0
}

unsafe extern "C" fn beep_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let cs42l52 = dev_get_drvdata(dev) as *mut cs42l52_private;
    let mut time: c_long = 0;
    let ret = kstrtol(buf, 10, &mut time);
    if ret != 0 {
        return ret as ssize_t;
    }
    input_event((*cs42l52).beep, EV_SND, SND_TONE, time as c_int);
    count as ssize_t
}

static dev_attr_beep: device_attribute = DEVICE_ATTR_WO!(beep);

unsafe extern "C" fn cs42l52_init_beep(component: *mut snd_soc_component) {
    let cs42l52 = snd_soc_component_get_drvdata(component) as *mut cs42l52_private;
    (*cs42l52).beep = devm_input_allocate_device((*component).dev);
    if (*cs42l52).beep.is_null() {
        dev_err((*component).dev, b"Failed to allocate beep device\n\0".as_ptr() as *const c_char);
        return;
    }
    INIT_WORK(&mut (*cs42l52).beep_work, Some(cs42l52_beep_work));
    (*cs42l52).beep_rate = 0;
    (*(*cs42l52).beep).name = b"CS42L52 Beep Generator\0".as_ptr() as *const c_char;
    (*(*cs42l52).beep).phys = dev_name((*component).dev);
    (*(*cs42l52).beep).id.bustype = BUS_I2C;
    (*(*cs42l52).beep).evbit[0] = BIT_MASK(EV_SND);
    (*(*cs42l52).beep).sndbit[0] = BIT_MASK(SND_BELL) | BIT_MASK(SND_TONE);
    (*(*cs42l52).beep).event = Some(cs42l52_beep_event);
    (*(*cs42l52).beep).dev.parent = (*component).dev;
    input_set_drvdata((*cs42l52).beep, component as *mut c_void);
    let mut ret = input_register_device((*cs42l52).beep);
    if ret != 0 {
        (*cs42l52).beep = core::ptr::null_mut();
        dev_err((*component).dev, b"Failed to register beep device\n\0".as_ptr() as *const c_char);
    }
    ret = device_create_file((*component).dev, &dev_attr_beep);
    if ret != 0 {
        dev_err((*component).dev, b"Failed to create keyclick file: %d\n\0".as_ptr() as *const c_char, ret);
    }
}

unsafe extern "C" fn cs42l52_free_beep(component: *mut snd_soc_component) {
    let cs42l52 = snd_soc_component_get_drvdata(component) as *mut cs42l52_private;
    device_remove_file((*component).dev, &dev_attr_beep);
    cancel_work_sync(&mut (*cs42l52).beep_work);
    (*cs42l52).beep = core::ptr::null_mut();
    snd_soc_component_update_bits(component, CS42L52_BEEP_TONE_CTL, CS42L52_BEEP_EN_MASK, 0);
}

unsafe extern "C" fn cs42l52_probe(component: *mut snd_soc_component) -> c_int {
    let cs42l52 = snd_soc_component_get_drvdata(component) as *mut cs42l52_private;
    regcache_cache_only((*cs42l52).regmap, true);
    cs42l52_add_mic_controls(component);
    cs42l52_init_beep(component);
    (*cs42l52).sysclk = CS42L52_DEFAULT_CLK;
    (*cs42l52).config.format = CS42L52_DEFAULT_FORMAT;
    0
}

unsafe extern "C" fn cs42l52_remove(component: *mut snd_soc_component) {
    cs42l52_free_beep(component);
}

static soc_component_dev_cs42l52: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs42l52_probe),
    remove: Some(cs42l52_remove),
    set_bias_level: Some(cs42l52_set_bias_level),
    controls: cs42l52_snd_controls.as_ptr(),
    num_controls: cs42l52_snd_controls.len() as c_uint,
    dapm_widgets: cs42l52_dapm_widgets.as_ptr(),
    num_dapm_widgets: cs42l52_dapm_widgets.len() as c_uint,
    dapm_routes: cs42l52_audio_map.as_ptr(),
    num_dapm_routes: cs42l52_audio_map.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

/* Current and threshold powerup sequence Pg37 */
static cs42l52_threshold_patch: &[reg_sequence] = &[
    reg_sequence { reg: 0x00, def: 0x99, delay_us: 0 },
    reg_sequence { reg: 0x3E, def: 0xBA, delay_us: 0 },
    reg_sequence { reg: 0x47, def: 0x80, delay_us: 0 },
    reg_sequence { reg: 0x32, def: 0xBB, delay_us: 0 },
    reg_sequence { reg: 0x32, def: 0x3B, delay_us: 0 },
    reg_sequence { reg: 0x00, def: 0x00, delay_us: 0 },
];

static cs42l52_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: CS42L52_MAX_REGISTER,
    reg_defaults: cs42l52_reg_defaults.as_ptr(),
    num_reg_defaults: cs42l52_reg_defaults.len() as c_uint,
    readable_reg: Some(cs42l52_readable_register),
    volatile_reg: Some(cs42l52_volatile_register),
    cache_type: REGCACHE_MAPLE,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn cs42l52_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let mut ret: c_int;
    let mut devid: c_uint;
    let mut reg: c_uint = 0;
    let mut val32: u32 = 0;
    let cs42l52 = devm_kzalloc(&mut (*i2c_client).dev, core::mem::size_of::<cs42l52_private>(), GFP_KERNEL) as *mut cs42l52_private;
    if cs42l52.is_null() {
        return -ENOMEM;
    }
    (*cs42l52).dev = &mut (*i2c_client).dev;
    (*cs42l52).regmap = devm_regmap_init_i2c(i2c_client, &cs42l52_regmap);
    if IS_ERR((*cs42l52).regmap as *const c_void) {
        ret = PTR_ERR((*cs42l52).regmap as *const c_void) as c_int;
        dev_err(&mut (*i2c_client).dev, b"regmap_init() failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    let pdata = devm_kzalloc(&mut (*i2c_client).dev, core::mem::size_of::<cs42l52_platform_data>(), GFP_KERNEL) as *mut cs42l52_platform_data;
    if pdata.is_null() {
        return -ENOMEM;
    }
    if !(*i2c_client).dev.of_node.is_null() {
        if of_property_read_bool((*i2c_client).dev.of_node, b"cirrus,mica-differential-cfg\0".as_ptr() as *const c_char) {
            (*pdata).mica_diff_cfg = true;
        }
        if of_property_read_bool((*i2c_client).dev.of_node, b"cirrus,micb-differential-cfg\0".as_ptr() as *const c_char) {
            (*pdata).micb_diff_cfg = true;
        }
        if of_property_read_u32((*i2c_client).dev.of_node, b"cirrus,micbias-lvl\0".as_ptr() as *const c_char, &mut val32) >= 0 {
            (*pdata).micbias_lvl = val32;
        }
        if of_property_read_u32((*i2c_client).dev.of_node, b"cirrus,chgfreq-divisor\0".as_ptr() as *const c_char, &mut val32) >= 0 {
            (*pdata).chgfreq = val32;
        }
        (*pdata).reset_gpio = devm_gpiod_get_optional(&mut (*i2c_client).dev, b"cirrus,reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
        if IS_ERR((*pdata).reset_gpio as *const c_void) {
            return PTR_ERR((*pdata).reset_gpio as *const c_void) as c_int;
        }
        gpiod_set_consumer_name((*pdata).reset_gpio, b"CS42L52 /RST\0".as_ptr() as *const c_char);
    }
    (*cs42l52).pdata = core::ptr::read(pdata);
    if !(*cs42l52).pdata.reset_gpio.is_null() {
        gpiod_set_value_cansleep((*cs42l52).pdata.reset_gpio, 1);
        gpiod_set_value_cansleep((*cs42l52).pdata.reset_gpio, 0);
    }
    i2c_set_clientdata(i2c_client, cs42l52 as *mut c_void);
    ret = regmap_register_patch((*cs42l52).regmap, cs42l52_threshold_patch.as_ptr(), cs42l52_threshold_patch.len() as c_int);
    if ret != 0 {
        dev_warn((*cs42l52).dev, b"Failed to apply regmap patch: %d\n\0".as_ptr() as *const c_char, ret);
    }
    ret = regmap_read((*cs42l52).regmap, CS42L52_CHIP, &mut reg);
    if ret != 0 {
        dev_err(&mut (*i2c_client).dev, b"Failed to read chip ID: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    devid = reg & CS42L52_CHIP_ID_MASK;
    if devid != CS42L52_CHIP_ID {
        ret = -ENODEV;
        dev_err(&mut (*i2c_client).dev, b"CS42L52 Device ID (%X). Expected %X\n\0".as_ptr() as *const c_char, devid, CS42L52_CHIP_ID);
        return ret;
    }
    dev_info(&mut (*i2c_client).dev, b"Cirrus Logic CS42L52, Revision: %02X\n\0".as_ptr() as *const c_char, reg & CS42L52_CHIP_REV_MASK);
    /* Set Platform Data */
    if (*cs42l52).pdata.mica_diff_cfg {
        regmap_update_bits((*cs42l52).regmap, CS42L52_MICA_CTL, CS42L52_MIC_CTL_TYPE_MASK, ((*cs42l52).pdata.mica_diff_cfg as c_uint) << CS42L52_MIC_CTL_TYPE_SHIFT);
    }
    if (*cs42l52).pdata.micb_diff_cfg {
        regmap_update_bits((*cs42l52).regmap, CS42L52_MICB_CTL, CS42L52_MIC_CTL_TYPE_MASK, ((*cs42l52).pdata.micb_diff_cfg as c_uint) << CS42L52_MIC_CTL_TYPE_SHIFT);
    }
    if (*cs42l52).pdata.chgfreq != 0 {
        regmap_update_bits((*cs42l52).regmap, CS42L52_CHARGE_PUMP, CS42L52_CHARGE_PUMP_MASK, (*cs42l52).pdata.chgfreq << CS42L52_CHARGE_PUMP_SHIFT);
    }
    if (*cs42l52).pdata.micbias_lvl != 0 {
        regmap_update_bits((*cs42l52).regmap, CS42L52_IFACE_CTL2, CS42L52_IFACE_CTL2_BIAS_LVL, (*cs42l52).pdata.micbias_lvl);
    }
    devm_snd_soc_register_component(&mut (*i2c_client).dev, &soc_component_dev_cs42l52, &mut cs42l52_dai, 1)
}

static cs42l52_of_match: &[of_device_id] = &[
    of_device_id { compatible: b"cirrus,cs42l52\0".as_ptr() as *const c_char, ..unsafe { core::mem::zeroed() } },
    of_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(of, cs42l52_of_match);

static cs42l52_id: &[i2c_device_id] = &[
    i2c_device_id { name: *b"cs42l52\0", ..unsafe { core::mem::zeroed() } },
    i2c_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(i2c, cs42l52_id);

static mut cs42l52_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"cs42l52\0".as_ptr() as *const c_char,
        of_match_table: cs42l52_of_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    id_table: cs42l52_id.as_ptr(),
    probe: Some(cs42l52_i2c_probe),
    ..unsafe { core::mem::zeroed() }
};

module_i2c_driver!(cs42l52_i2c_driver);

MODULE_DESCRIPTION!("ASoC CS42L52 driver");
MODULE_AUTHOR!("Georgi Vlaev, Nucleus Systems Ltd, <joe@nucleusys.com>");
MODULE_AUTHOR!("Brian Austin, Cirrus Logic Inc, <brian.austin@cirrus.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
