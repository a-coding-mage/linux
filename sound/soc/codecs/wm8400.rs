// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * wm8400.rs  --  WM8400 ALSA Soc Audio driver
 *
 * Rust translation of wm8400.c.
 *
 * Copyright 2008-11 Wolfson Microelectronics PLC.
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type u16 = u16;
type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wm8400 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_int,
    pub rreg: c_int,
    pub shift: c_uint,
    pub rshift: c_uint,
    pub max: c_uint,
    pub platform_max: c_uint,
    pub invert: c_uint,
    pub autodisable: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct fll_factors {
    pub n: u16,
    pub k: u16,
    pub outdiv: u16,
    pub fratio: u16,
    pub freq_ref: u16,
}

/* codec private data */
#[repr(C)]
pub struct wm8400_priv {
    pub wm8400: *mut wm8400,
    pub fake_register: u16,
    pub sysclk: c_uint,
    pub pcmclk: c_uint,
    pub fll_in: c_int,
    pub fll_out: c_int,
}

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut c_void);
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_int) -> u16;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_int, val: u16) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn wm8400_reset_codec_reg_cache(wm8400: *mut wm8400);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *mut wm8400;
    fn msleep(msecs: c_uint);
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
}

extern "C" {
    static mut power: [regulator_bulk_data; 7];
}

unsafe fn wm8400_component_reset(component: *mut snd_soc_component) {
    let wm8400 = snd_soc_component_get_drvdata(component) as *mut wm8400_priv;
    wm8400_reset_codec_reg_cache((*wm8400).wm8400);
}

unsafe fn wm8400_outpga_put_volsw_vu(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let reg = (*mc).reg;
    let ret = snd_soc_put_volsw(kcontrol, ucontrol);
    if ret < 0 {
        return ret;
    }

    /* now hit the volume update bits (always bit 8) */
    let val = snd_soc_component_read(component, reg);
    snd_soc_component_write(component, reg, val | 0x0100)
}

kernel_c_items! {
static struct regulator_bulk_data power[] = {
	{ .supply = "I2S1VDD", },
	{ .supply = "I2S2VDD", },
	{ .supply = "DCVDD", },
	{ .supply = "AVDD", },
	{ .supply = "FLLVDD", },
	{ .supply = "HPVDD", },
	{ .supply = "SPKVDD", },
};

static const DECLARE_TLV_DB_SCALE(in_pga_tlv, -1650, 3000, 0);
static const DECLARE_TLV_DB_SCALE(out_mix_tlv, -2100, 0, 0);
static const DECLARE_TLV_DB_SCALE(out_pga_tlv, -7300, 600, 0);
static const DECLARE_TLV_DB_SCALE(out_dac_tlv, -7163, 0, 0);
static const DECLARE_TLV_DB_SCALE(in_adc_tlv, -7163, 1763, 0);
static const DECLARE_TLV_DB_SCALE(out_sidetone_tlv, -3600, 0, 0);

#define WM8400_OUTPGA_SINGLE_R_TLV(xname, reg, shift, max, invert, tlv_array) \
	SOC_SINGLE_EXT_TLV(xname, reg, shift, max, invert, \
		snd_soc_get_volsw, wm8400_outpga_put_volsw_vu, tlv_array)

static const char *wm8400_digital_sidetone[] =
	{"None", "Left ADC", "Right ADC", "Reserved"};
static SOC_ENUM_SINGLE_DECL(wm8400_left_digital_sidetone_enum,
			    WM8400_DIGITAL_SIDE_TONE,
			    WM8400_ADC_TO_DACL_SHIFT,
			    wm8400_digital_sidetone);
static SOC_ENUM_SINGLE_DECL(wm8400_right_digital_sidetone_enum,
			    WM8400_DIGITAL_SIDE_TONE,
			    WM8400_ADC_TO_DACR_SHIFT,
			    wm8400_digital_sidetone);

static const char *wm8400_adcmode[] =
	{"Hi-fi mode", "Voice mode 1", "Voice mode 2", "Voice mode 3"};
static SOC_ENUM_SINGLE_DECL(wm8400_right_adcmode_enum,
			    WM8400_ADC_CTRL,
			    WM8400_ADC_HPF_CUT_SHIFT,
			    wm8400_adcmode);

static const struct snd_kcontrol_new wm8400_snd_controls[] = {
/* INMIXL */
SOC_SINGLE("LIN12 PGA Boost", WM8400_INPUT_MIXER3, WM8400_L12MNBST_SHIFT, 1, 0),
SOC_SINGLE("LIN34 PGA Boost", WM8400_INPUT_MIXER3, WM8400_L34MNBST_SHIFT, 1, 0),
/* INMIXR */
SOC_SINGLE("RIN12 PGA Boost", WM8400_INPUT_MIXER3, WM8400_R12MNBST_SHIFT, 1, 0),
SOC_SINGLE("RIN34 PGA Boost", WM8400_INPUT_MIXER3, WM8400_R34MNBST_SHIFT, 1, 0),
/* LOMIX */
SOC_SINGLE_TLV("LOMIX LIN3 Bypass Volume", WM8400_OUTPUT_MIXER3, WM8400_LLI3LOVOL_SHIFT, 7, 0, out_mix_tlv),
SOC_SINGLE_TLV("LOMIX RIN12 PGA Bypass Volume", WM8400_OUTPUT_MIXER3, WM8400_LR12LOVOL_SHIFT, 7, 0, out_mix_tlv),
SOC_SINGLE_TLV("LOMIX LIN12 PGA Bypass Volume", WM8400_OUTPUT_MIXER3, WM8400_LL12LOVOL_SHIFT, 7, 0, out_mix_tlv),
SOC_SINGLE_TLV("LOMIX RIN3 Bypass Volume", WM8400_OUTPUT_MIXER5, WM8400_LRI3LOVOL_SHIFT, 7, 0, out_mix_tlv),
SOC_SINGLE_TLV("LOMIX AINRMUX Bypass Volume", WM8400_OUTPUT_MIXER5, WM8400_LRBLOVOL_SHIFT, 7, 0, out_mix_tlv),
SOC_SINGLE_TLV("LOMIX AINLMUX Bypass Volume", WM8400_OUTPUT_MIXER5, WM8400_LRBLOVOL_SHIFT, 7, 0, out_mix_tlv),
/* ROMIX */
SOC_SINGLE_TLV("ROMIX RIN3 Bypass Volume", WM8400_OUTPUT_MIXER4, WM8400_RRI3ROVOL_SHIFT, 7, 0, out_mix_tlv),
SOC_SINGLE_TLV("ROMIX LIN12 PGA Bypass Volume", WM8400_OUTPUT_MIXER4, WM8400_RL12ROVOL_SHIFT, 7, 0, out_mix_tlv),
SOC_SINGLE_TLV("ROMIX RIN12 PGA Bypass Volume", WM8400_OUTPUT_MIXER4, WM8400_RR12ROVOL_SHIFT, 7, 0, out_mix_tlv),
SOC_SINGLE_TLV("ROMIX LIN3 Bypass Volume", WM8400_OUTPUT_MIXER6, WM8400_RLI3ROVOL_SHIFT, 7, 0, out_mix_tlv),
SOC_SINGLE_TLV("ROMIX AINLMUX Bypass Volume", WM8400_OUTPUT_MIXER6, WM8400_RLBROVOL_SHIFT, 7, 0, out_mix_tlv),
SOC_SINGLE_TLV("ROMIX AINRMUX Bypass Volume", WM8400_OUTPUT_MIXER6, WM8400_RRBROVOL_SHIFT, 7, 0, out_mix_tlv),
WM8400_OUTPGA_SINGLE_R_TLV("LOUT Volume", WM8400_LEFT_OUTPUT_VOLUME, WM8400_LOUTVOL_SHIFT, WM8400_LOUTVOL_MASK, 0, out_pga_tlv),
SOC_SINGLE("LOUT ZC", WM8400_LEFT_OUTPUT_VOLUME, WM8400_LOZC_SHIFT, 1, 0),
WM8400_OUTPGA_SINGLE_R_TLV("ROUT Volume", WM8400_RIGHT_OUTPUT_VOLUME, WM8400_ROUTVOL_SHIFT, WM8400_ROUTVOL_MASK, 0, out_pga_tlv),
SOC_SINGLE("ROUT ZC", WM8400_RIGHT_OUTPUT_VOLUME, WM8400_ROZC_SHIFT, 1, 0),
WM8400_OUTPGA_SINGLE_R_TLV("LOPGA Volume", WM8400_LEFT_OPGA_VOLUME, WM8400_LOPGAVOL_SHIFT, WM8400_LOPGAVOL_MASK, 0, out_pga_tlv),
SOC_SINGLE("LOPGA ZC Switch", WM8400_LEFT_OPGA_VOLUME, WM8400_LOPGAZC_SHIFT, 1, 0),
WM8400_OUTPGA_SINGLE_R_TLV("ROPGA Volume", WM8400_RIGHT_OPGA_VOLUME, WM8400_ROPGAVOL_SHIFT, WM8400_ROPGAVOL_MASK, 0, out_pga_tlv),
SOC_SINGLE("ROPGA ZC Switch", WM8400_RIGHT_OPGA_VOLUME, WM8400_ROPGAZC_SHIFT, 1, 0),
SOC_SINGLE("LON Mute Switch", WM8400_LINE_OUTPUTS_VOLUME, WM8400_LONMUTE_SHIFT, 1, 0),
SOC_SINGLE("LOP Mute Switch", WM8400_LINE_OUTPUTS_VOLUME, WM8400_LOPMUTE_SHIFT, 1, 0),
SOC_SINGLE("LOP Attenuation Switch", WM8400_LINE_OUTPUTS_VOLUME, WM8400_LOATTN_SHIFT, 1, 0),
SOC_SINGLE("RON Mute Switch", WM8400_LINE_OUTPUTS_VOLUME, WM8400_RONMUTE_SHIFT, 1, 0),
SOC_SINGLE("ROP Mute Switch", WM8400_LINE_OUTPUTS_VOLUME, WM8400_ROPMUTE_SHIFT, 1, 0),
SOC_SINGLE("ROP Attenuation Switch", WM8400_LINE_OUTPUTS_VOLUME, WM8400_ROATTN_SHIFT, 1, 0),
SOC_SINGLE("OUT3 Mute Switch", WM8400_OUT3_4_VOLUME, WM8400_OUT3MUTE_SHIFT, 1, 0),
SOC_SINGLE("OUT3 Attenuation Switch", WM8400_OUT3_4_VOLUME, WM8400_OUT3ATTN_SHIFT, 1, 0),
SOC_SINGLE("OUT4 Mute Switch", WM8400_OUT3_4_VOLUME, WM8400_OUT4MUTE_SHIFT, 1, 0),
SOC_SINGLE("OUT4 Attenuation Switch", WM8400_OUT3_4_VOLUME, WM8400_OUT4ATTN_SHIFT, 1, 0),
SOC_SINGLE("Speaker Mode Switch", WM8400_CLASSD1, WM8400_CDMODE_SHIFT, 1, 0),
SOC_SINGLE("Speaker Output Attenuation Volume", WM8400_SPEAKER_VOLUME, WM8400_SPKATTN_SHIFT, WM8400_SPKATTN_MASK, 0),
SOC_SINGLE("Speaker DC Boost Volume", WM8400_CLASSD3, WM8400_DCGAIN_SHIFT, 6, 0),
SOC_SINGLE("Speaker AC Boost Volume", WM8400_CLASSD3, WM8400_ACGAIN_SHIFT, 6, 0),
WM8400_OUTPGA_SINGLE_R_TLV("Left DAC Digital Volume", WM8400_LEFT_DAC_DIGITAL_VOLUME, WM8400_DACL_VOL_SHIFT, 127, 0, out_dac_tlv),
WM8400_OUTPGA_SINGLE_R_TLV("Right DAC Digital Volume", WM8400_RIGHT_DAC_DIGITAL_VOLUME, WM8400_DACR_VOL_SHIFT, 127, 0, out_dac_tlv),
SOC_ENUM("Left Digital Sidetone", wm8400_left_digital_sidetone_enum),
SOC_ENUM("Right Digital Sidetone", wm8400_right_digital_sidetone_enum),
SOC_SINGLE_TLV("Left Digital Sidetone Volume", WM8400_DIGITAL_SIDE_TONE, WM8400_ADCL_DAC_SVOL_SHIFT, 15, 0, out_sidetone_tlv),
SOC_SINGLE_TLV("Right Digital Sidetone Volume", WM8400_DIGITAL_SIDE_TONE, WM8400_ADCR_DAC_SVOL_SHIFT, 15, 0, out_sidetone_tlv),
SOC_SINGLE("ADC Digital High Pass Filter Switch", WM8400_ADC_CTRL, WM8400_ADC_HPF_ENA_SHIFT, 1, 0),
SOC_ENUM("ADC HPF Mode", wm8400_right_adcmode_enum),
WM8400_OUTPGA_SINGLE_R_TLV("Left ADC Digital Volume", WM8400_LEFT_ADC_DIGITAL_VOLUME, WM8400_ADCL_VOL_SHIFT, WM8400_ADCL_VOL_MASK, 0, in_adc_tlv),
WM8400_OUTPGA_SINGLE_R_TLV("Right ADC Digital Volume", WM8400_RIGHT_ADC_DIGITAL_VOLUME, WM8400_ADCR_VOL_SHIFT, WM8400_ADCR_VOL_MASK, 0, in_adc_tlv),
WM8400_OUTPGA_SINGLE_R_TLV("LIN12 Volume", WM8400_LEFT_LINE_INPUT_1_2_VOLUME, WM8400_LIN12VOL_SHIFT, WM8400_LIN12VOL_MASK, 0, in_pga_tlv),
SOC_SINGLE("LIN12 ZC Switch", WM8400_LEFT_LINE_INPUT_1_2_VOLUME, WM8400_LI12ZC_SHIFT, 1, 0),
SOC_SINGLE("LIN12 Mute Switch", WM8400_LEFT_LINE_INPUT_1_2_VOLUME, WM8400_LI12MUTE_SHIFT, 1, 0),
WM8400_OUTPGA_SINGLE_R_TLV("LIN34 Volume", WM8400_LEFT_LINE_INPUT_3_4_VOLUME, WM8400_LIN34VOL_SHIFT, WM8400_LIN34VOL_MASK, 0, in_pga_tlv),
SOC_SINGLE("LIN34 ZC Switch", WM8400_LEFT_LINE_INPUT_3_4_VOLUME, WM8400_LI34ZC_SHIFT, 1, 0),
SOC_SINGLE("LIN34 Mute Switch", WM8400_LEFT_LINE_INPUT_3_4_VOLUME, WM8400_LI34MUTE_SHIFT, 1, 0),
WM8400_OUTPGA_SINGLE_R_TLV("RIN12 Volume", WM8400_RIGHT_LINE_INPUT_1_2_VOLUME, WM8400_RIN12VOL_SHIFT, WM8400_RIN12VOL_MASK, 0, in_pga_tlv),
SOC_SINGLE("RIN12 ZC Switch", WM8400_RIGHT_LINE_INPUT_1_2_VOLUME, WM8400_RI12ZC_SHIFT, 1, 0),
SOC_SINGLE("RIN12 Mute Switch", WM8400_RIGHT_LINE_INPUT_1_2_VOLUME, WM8400_RI12MUTE_SHIFT, 1, 0),
WM8400_OUTPGA_SINGLE_R_TLV("RIN34 Volume", WM8400_RIGHT_LINE_INPUT_3_4_VOLUME, WM8400_RIN34VOL_SHIFT, WM8400_RIN34VOL_MASK, 0, in_pga_tlv),
SOC_SINGLE("RIN34 ZC Switch", WM8400_RIGHT_LINE_INPUT_3_4_VOLUME, WM8400_RI34ZC_SHIFT, 1, 0),
SOC_SINGLE("RIN34 Mute Switch", WM8400_RIGHT_LINE_INPUT_3_4_VOLUME, WM8400_RI34MUTE_SHIFT, 1, 0),
};
}

unsafe fn outmixer_event(
    w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    _event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let wm8400 = snd_soc_component_get_drvdata(component) as *mut wm8400_priv;
    let reg_shift: u32 = (*mc).shift;
    let mut ret: c_int = 0;
    let reg: u16;

    match reg_shift {
        x if x == (WM8400_SPEAKER_MIXER | (WM8400_LDSPK << 8)) as u32 => {
            reg = snd_soc_component_read(component, WM8400_OUTPUT_MIXER1);
            if (reg & WM8400_LDLO as u16) != 0 {
                dev_warn((*(*wm8400).wm8400).dev, c"Cannot set as Output Mixer 1 LDLO Set\n".as_ptr());
                ret = -1;
            }
        }
        x if x == (WM8400_SPEAKER_MIXER | (WM8400_RDSPK << 8)) as u32 => {
            reg = snd_soc_component_read(component, WM8400_OUTPUT_MIXER2);
            if (reg & WM8400_RDRO as u16) != 0 {
                dev_warn((*(*wm8400).wm8400).dev, c"Cannot set as Output Mixer 2 RDRO Set\n".as_ptr());
                ret = -1;
            }
        }
        x if x == (WM8400_OUTPUT_MIXER1 | (WM8400_LDLO << 8)) as u32 => {
            reg = snd_soc_component_read(component, WM8400_SPEAKER_MIXER);
            if (reg & WM8400_LDSPK as u16) != 0 {
                dev_warn((*(*wm8400).wm8400).dev, c"Cannot set as Speaker Mixer LDSPK Set\n".as_ptr());
                ret = -1;
            }
        }
        x if x == (WM8400_OUTPUT_MIXER2 | (WM8400_RDRO << 8)) as u32 => {
            reg = snd_soc_component_read(component, WM8400_SPEAKER_MIXER);
            if (reg & WM8400_RDSPK as u16) != 0 {
                dev_warn((*(*wm8400).wm8400).dev, c"Cannot set as Speaker Mixer RDSPK Set\n".as_ptr());
                ret = -1;
            }
        }
        _ => {}
    }

    ret
}

kernel_c_items! {
/* INMIX dB values */
static const DECLARE_TLV_DB_SCALE(in_mix_tlv, -1200, 600, 0);

/* DAPM controls, widgets and routes are translated as the original macro constructors. */
static const struct snd_kcontrol_new wm8400_dapm_lin12_pga_controls[] = {
SOC_DAPM_SINGLE("LIN1 Switch", WM8400_INPUT_MIXER2, WM8400_LMN1_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LIN2 Switch", WM8400_INPUT_MIXER2, WM8400_LMP2_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_lin34_pga_controls[] = {
SOC_DAPM_SINGLE("LIN3 Switch", WM8400_INPUT_MIXER2, WM8400_LMN3_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LIN4 Switch", WM8400_INPUT_MIXER2, WM8400_LMP4_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_rin12_pga_controls[] = {
SOC_DAPM_SINGLE("RIN1 Switch", WM8400_INPUT_MIXER2, WM8400_RMN1_SHIFT, 1, 0),
SOC_DAPM_SINGLE("RIN2 Switch", WM8400_INPUT_MIXER2, WM8400_RMP2_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_rin34_pga_controls[] = {
SOC_DAPM_SINGLE("RIN3 Switch", WM8400_INPUT_MIXER2, WM8400_RMN3_SHIFT, 1, 0),
SOC_DAPM_SINGLE("RIN4 Switch", WM8400_INPUT_MIXER2, WM8400_RMP4_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_inmixl_controls[] = {
SOC_DAPM_SINGLE_TLV("Record Left Volume", WM8400_INPUT_MIXER3, WM8400_LDBVOL_SHIFT, WM8400_LDBVOL_MASK, 0, in_mix_tlv),
SOC_DAPM_SINGLE_TLV("LIN2 Volume", WM8400_INPUT_MIXER5, WM8400_LI2BVOL_SHIFT, 7, 0, in_mix_tlv),
SOC_DAPM_SINGLE("LINPGA12 Switch", WM8400_INPUT_MIXER3, WM8400_L12MNB_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LINPGA34 Switch", WM8400_INPUT_MIXER3, WM8400_L34MNB_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_inmixr_controls[] = {
SOC_DAPM_SINGLE_TLV("Record Right Volume", WM8400_INPUT_MIXER4, WM8400_RDBVOL_SHIFT, WM8400_RDBVOL_MASK, 0, in_mix_tlv),
SOC_DAPM_SINGLE_TLV("RIN2 Volume", WM8400_INPUT_MIXER6, WM8400_RI2BVOL_SHIFT, 7, 0, in_mix_tlv),
SOC_DAPM_SINGLE("RINPGA12 Switch", WM8400_INPUT_MIXER3, WM8400_L12MNB_SHIFT, 1, 0),
SOC_DAPM_SINGLE("RINPGA34 Switch", WM8400_INPUT_MIXER3, WM8400_L34MNB_SHIFT, 1, 0),
};
static const char *wm8400_ainlmux[] = {"INMIXL Mix", "RXVOICE Mix", "DIFFINL Mix"};
static SOC_ENUM_SINGLE_DECL(wm8400_ainlmux_enum, WM8400_INPUT_MIXER1, WM8400_AINLMODE_SHIFT, wm8400_ainlmux);
static const struct snd_kcontrol_new wm8400_dapm_ainlmux_controls = SOC_DAPM_ENUM("Route", wm8400_ainlmux_enum);
static const char *wm8400_ainrmux[] = {"INMIXR Mix", "RXVOICE Mix", "DIFFINR Mix"};
static SOC_ENUM_SINGLE_DECL(wm8400_ainrmux_enum, WM8400_INPUT_MIXER1, WM8400_AINRMODE_SHIFT, wm8400_ainrmux);
static const struct snd_kcontrol_new wm8400_dapm_ainrmux_controls = SOC_DAPM_ENUM("Route", wm8400_ainrmux_enum);
static const struct snd_kcontrol_new wm8400_dapm_lomix_controls[] = {
SOC_DAPM_SINGLE("LOMIX Right ADC Bypass Switch", WM8400_OUTPUT_MIXER1, WM8400_LRBLO_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LOMIX Left ADC Bypass Switch", WM8400_OUTPUT_MIXER1, WM8400_LLBLO_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LOMIX RIN3 Bypass Switch", WM8400_OUTPUT_MIXER1, WM8400_LRI3LO_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LOMIX LIN3 Bypass Switch", WM8400_OUTPUT_MIXER1, WM8400_LLI3LO_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LOMIX RIN12 PGA Bypass Switch", WM8400_OUTPUT_MIXER1, WM8400_LR12LO_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LOMIX LIN12 PGA Bypass Switch", WM8400_OUTPUT_MIXER1, WM8400_LL12LO_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LOMIX Left DAC Switch", WM8400_OUTPUT_MIXER1, WM8400_LDLO_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_romix_controls[] = {
SOC_DAPM_SINGLE("ROMIX Left ADC Bypass Switch", WM8400_OUTPUT_MIXER2, WM8400_RLBRO_SHIFT, 1, 0),
SOC_DAPM_SINGLE("ROMIX Right ADC Bypass Switch", WM8400_OUTPUT_MIXER2, WM8400_RRBRO_SHIFT, 1, 0),
SOC_DAPM_SINGLE("ROMIX LIN3 Bypass Switch", WM8400_OUTPUT_MIXER2, WM8400_RLI3RO_SHIFT, 1, 0),
SOC_DAPM_SINGLE("ROMIX RIN3 Bypass Switch", WM8400_OUTPUT_MIXER2, WM8400_RRI3RO_SHIFT, 1, 0),
SOC_DAPM_SINGLE("ROMIX LIN12 PGA Bypass Switch", WM8400_OUTPUT_MIXER2, WM8400_RL12RO_SHIFT, 1, 0),
SOC_DAPM_SINGLE("ROMIX RIN12 PGA Bypass Switch", WM8400_OUTPUT_MIXER2, WM8400_RR12RO_SHIFT, 1, 0),
SOC_DAPM_SINGLE("ROMIX Right DAC Switch", WM8400_OUTPUT_MIXER2, WM8400_RDRO_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_lonmix_controls[] = {
SOC_DAPM_SINGLE("LONMIX Left Mixer PGA Switch", WM8400_LINE_MIXER1, WM8400_LLOPGALON_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LONMIX Right Mixer PGA Switch", WM8400_LINE_MIXER1, WM8400_LROPGALON_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LONMIX Inverted LOP Switch", WM8400_LINE_MIXER1, WM8400_LOPLON_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_lopmix_controls[] = {
SOC_DAPM_SINGLE("LOPMIX Right Mic Bypass Switch", WM8400_LINE_MIXER1, WM8400_LR12LOP_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LOPMIX Left Mic Bypass Switch", WM8400_LINE_MIXER1, WM8400_LL12LOP_SHIFT, 1, 0),
SOC_DAPM_SINGLE("LOPMIX Left Mixer PGA Switch", WM8400_LINE_MIXER1, WM8400_LLOPGALOP_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_ronmix_controls[] = {
SOC_DAPM_SINGLE("RONMIX Right Mixer PGA Switch", WM8400_LINE_MIXER2, WM8400_RROPGARON_SHIFT, 1, 0),
SOC_DAPM_SINGLE("RONMIX Left Mixer PGA Switch", WM8400_LINE_MIXER2, WM8400_RLOPGARON_SHIFT, 1, 0),
SOC_DAPM_SINGLE("RONMIX Inverted ROP Switch", WM8400_LINE_MIXER2, WM8400_ROPRON_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_ropmix_controls[] = {
SOC_DAPM_SINGLE("ROPMIX Left Mic Bypass Switch", WM8400_LINE_MIXER2, WM8400_RL12ROP_SHIFT, 1, 0),
SOC_DAPM_SINGLE("ROPMIX Right Mic Bypass Switch", WM8400_LINE_MIXER2, WM8400_RR12ROP_SHIFT, 1, 0),
SOC_DAPM_SINGLE("ROPMIX Right Mixer PGA Switch", WM8400_LINE_MIXER2, WM8400_RROPGAROP_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_out3mix_controls[] = {
SOC_DAPM_SINGLE("OUT3MIX LIN4/RXP Bypass Switch", WM8400_OUT3_4_MIXER, WM8400_LI4O3_SHIFT, 1, 0),
SOC_DAPM_SINGLE("OUT3MIX Left Out PGA Switch", WM8400_OUT3_4_MIXER, WM8400_LPGAO3_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_out4mix_controls[] = {
SOC_DAPM_SINGLE("OUT4MIX Right Out PGA Switch", WM8400_OUT3_4_MIXER, WM8400_RPGAO4_SHIFT, 1, 0),
SOC_DAPM_SINGLE("OUT4MIX RIN4/RXP Bypass Switch", WM8400_OUT3_4_MIXER, WM8400_RI4O4_SHIFT, 1, 0),
};
static const struct snd_kcontrol_new wm8400_dapm_spkmix_controls[] = {
SOC_DAPM_SINGLE("SPKMIX LIN2 Bypass Switch", WM8400_SPEAKER_MIXER, WM8400_LI2SPK_SHIFT, 1, 0),
SOC_DAPM_SINGLE("SPKMIX LADC Bypass Switch", WM8400_SPEAKER_MIXER, WM8400_LB2SPK_SHIFT, 1, 0),
SOC_DAPM_SINGLE("SPKMIX Left Mixer PGA Switch", WM8400_SPEAKER_MIXER, WM8400_LOPGASPK_SHIFT, 1, 0),
SOC_DAPM_SINGLE("SPKMIX Left DAC Switch", WM8400_SPEAKER_MIXER, WM8400_LDSPK_SHIFT, 1, 0),
SOC_DAPM_SINGLE("SPKMIX Right DAC Switch", WM8400_SPEAKER_MIXER, WM8400_RDSPK_SHIFT, 1, 0),
SOC_DAPM_SINGLE("SPKMIX Right Mixer PGA Switch", WM8400_SPEAKER_MIXER, WM8400_ROPGASPK_SHIFT, 1, 0),
SOC_DAPM_SINGLE("SPKMIX RADC Bypass Switch", WM8400_SPEAKER_MIXER, WM8400_RL12ROP_SHIFT, 1, 0),
SOC_DAPM_SINGLE("SPKMIX RIN2 Bypass Switch", WM8400_SPEAKER_MIXER, WM8400_RI2SPK_SHIFT, 1, 0),
};

static const struct snd_soc_dapm_widget wm8400_dapm_widgets[] = {
SND_SOC_DAPM_INPUT("LIN1"), SND_SOC_DAPM_INPUT("LIN2"), SND_SOC_DAPM_INPUT("LIN3"), SND_SOC_DAPM_INPUT("LIN4/RXN"),
SND_SOC_DAPM_INPUT("RIN3"), SND_SOC_DAPM_INPUT("RIN4/RXP"), SND_SOC_DAPM_INPUT("RIN1"), SND_SOC_DAPM_INPUT("RIN2"),
SND_SOC_DAPM_INPUT("Internal ADC Source"),
SND_SOC_DAPM_ADC("Left ADC", "Left Capture", WM8400_POWER_MANAGEMENT_2, WM8400_ADCL_ENA_SHIFT, 0),
SND_SOC_DAPM_ADC("Right ADC", "Right Capture", WM8400_POWER_MANAGEMENT_2, WM8400_ADCR_ENA_SHIFT, 0),
SND_SOC_DAPM_MIXER("LIN12 PGA", WM8400_POWER_MANAGEMENT_2, WM8400_LIN12_ENA_SHIFT, 0, &wm8400_dapm_lin12_pga_controls[0], ARRAY_SIZE(wm8400_dapm_lin12_pga_controls)),
SND_SOC_DAPM_MIXER("LIN34 PGA", WM8400_POWER_MANAGEMENT_2, WM8400_LIN34_ENA_SHIFT, 0, &wm8400_dapm_lin34_pga_controls[0], ARRAY_SIZE(wm8400_dapm_lin34_pga_controls)),
SND_SOC_DAPM_MIXER("RIN12 PGA", WM8400_POWER_MANAGEMENT_2, WM8400_RIN12_ENA_SHIFT, 0, &wm8400_dapm_rin12_pga_controls[0], ARRAY_SIZE(wm8400_dapm_rin12_pga_controls)),
SND_SOC_DAPM_MIXER("RIN34 PGA", WM8400_POWER_MANAGEMENT_2, WM8400_RIN34_ENA_SHIFT, 0, &wm8400_dapm_rin34_pga_controls[0], ARRAY_SIZE(wm8400_dapm_rin34_pga_controls)),
SND_SOC_DAPM_SUPPLY("INL", WM8400_POWER_MANAGEMENT_2, WM8400_AINL_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_SUPPLY("INR", WM8400_POWER_MANAGEMENT_2, WM8400_AINR_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_MIXER("INMIXL", SND_SOC_NOPM, 0, 0, &wm8400_dapm_inmixl_controls[0], ARRAY_SIZE(wm8400_dapm_inmixl_controls)),
SND_SOC_DAPM_MUX("AILNMUX", SND_SOC_NOPM, 0, 0, &wm8400_dapm_ainlmux_controls),
SND_SOC_DAPM_MIXER("INMIXR", SND_SOC_NOPM, 0, 0, &wm8400_dapm_inmixr_controls[0], ARRAY_SIZE(wm8400_dapm_inmixr_controls)),
SND_SOC_DAPM_MUX("AIRNMUX", SND_SOC_NOPM, 0, 0, &wm8400_dapm_ainrmux_controls),
SND_SOC_DAPM_DAC("Left DAC", "Left Playback", WM8400_POWER_MANAGEMENT_3, WM8400_DACL_ENA_SHIFT, 0),
SND_SOC_DAPM_DAC("Right DAC", "Right Playback", WM8400_POWER_MANAGEMENT_3, WM8400_DACR_ENA_SHIFT, 0),
SND_SOC_DAPM_MIXER_E("LOMIX", WM8400_POWER_MANAGEMENT_3, WM8400_LOMIX_ENA_SHIFT, 0, &wm8400_dapm_lomix_controls[0], ARRAY_SIZE(wm8400_dapm_lomix_controls), outmixer_event, SND_SOC_DAPM_PRE_REG),
SND_SOC_DAPM_MIXER("LONMIX", WM8400_POWER_MANAGEMENT_3, WM8400_LON_ENA_SHIFT, 0, &wm8400_dapm_lonmix_controls[0], ARRAY_SIZE(wm8400_dapm_lonmix_controls)),
SND_SOC_DAPM_MIXER("LOPMIX", WM8400_POWER_MANAGEMENT_3, WM8400_LOP_ENA_SHIFT, 0, &wm8400_dapm_lopmix_controls[0], ARRAY_SIZE(wm8400_dapm_lopmix_controls)),
SND_SOC_DAPM_MIXER("OUT3MIX", WM8400_POWER_MANAGEMENT_1, WM8400_OUT3_ENA_SHIFT, 0, &wm8400_dapm_out3mix_controls[0], ARRAY_SIZE(wm8400_dapm_out3mix_controls)),
SND_SOC_DAPM_MIXER_E("SPKMIX", WM8400_POWER_MANAGEMENT_1, WM8400_SPK_ENA_SHIFT, 0, &wm8400_dapm_spkmix_controls[0], ARRAY_SIZE(wm8400_dapm_spkmix_controls), outmixer_event, SND_SOC_DAPM_PRE_REG),
SND_SOC_DAPM_MIXER("OUT4MIX", WM8400_POWER_MANAGEMENT_1, WM8400_OUT4_ENA_SHIFT, 0, &wm8400_dapm_out4mix_controls[0], ARRAY_SIZE(wm8400_dapm_out4mix_controls)),
SND_SOC_DAPM_MIXER("ROPMIX", WM8400_POWER_MANAGEMENT_3, WM8400_ROP_ENA_SHIFT, 0, &wm8400_dapm_ropmix_controls[0], ARRAY_SIZE(wm8400_dapm_ropmix_controls)),
SND_SOC_DAPM_MIXER("RONMIX", WM8400_POWER_MANAGEMENT_3, WM8400_RON_ENA_SHIFT, 0, &wm8400_dapm_ronmix_controls[0], ARRAY_SIZE(wm8400_dapm_ronmix_controls)),
SND_SOC_DAPM_MIXER_E("ROMIX", WM8400_POWER_MANAGEMENT_3, WM8400_ROMIX_ENA_SHIFT, 0, &wm8400_dapm_romix_controls[0], ARRAY_SIZE(wm8400_dapm_romix_controls), outmixer_event, SND_SOC_DAPM_PRE_REG),
SND_SOC_DAPM_PGA("LOUT PGA", WM8400_POWER_MANAGEMENT_1, WM8400_LOUT_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ROUT PGA", WM8400_POWER_MANAGEMENT_1, WM8400_ROUT_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("LOPGA", WM8400_POWER_MANAGEMENT_3, WM8400_LOPGA_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_PGA("ROPGA", WM8400_POWER_MANAGEMENT_3, WM8400_ROPGA_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_SUPPLY("MICBIAS", WM8400_POWER_MANAGEMENT_1, WM8400_MIC1BIAS_ENA_SHIFT, 0, NULL, 0),
SND_SOC_DAPM_OUTPUT("LON"), SND_SOC_DAPM_OUTPUT("LOP"), SND_SOC_DAPM_OUTPUT("OUT3"), SND_SOC_DAPM_OUTPUT("LOUT"),
SND_SOC_DAPM_OUTPUT("SPKN"), SND_SOC_DAPM_OUTPUT("SPKP"), SND_SOC_DAPM_OUTPUT("ROUT"), SND_SOC_DAPM_OUTPUT("OUT4"),
SND_SOC_DAPM_OUTPUT("ROP"), SND_SOC_DAPM_OUTPUT("RON"), SND_SOC_DAPM_OUTPUT("Internal DAC Sink"),
};
}

kernel_c_items! {
static const struct snd_soc_dapm_route wm8400_dapm_routes[] = {
{"Internal DAC Sink", NULL, "Left DAC"}, {"Internal DAC Sink", NULL, "Right DAC"},
{"Left ADC", NULL, "Internal ADC Source"}, {"Right ADC", NULL, "Internal ADC Source"},
{"LIN12 PGA", "LIN1 Switch", "LIN1"}, {"LIN12 PGA", "LIN2 Switch", "LIN2"},
{"LIN34 PGA", "LIN3 Switch", "LIN3"}, {"LIN34 PGA", "LIN4 Switch", "LIN4/RXN"},
{"INMIXL", NULL, "INL"}, {"INMIXL", "Record Left Volume", "LOMIX"}, {"INMIXL", "LIN2 Volume", "LIN2"},
{"INMIXL", "LINPGA12 Switch", "LIN12 PGA"}, {"INMIXL", "LINPGA34 Switch", "LIN34 PGA"},
{"AILNMUX", NULL, "INL"}, {"AILNMUX", "INMIXL Mix", "INMIXL"}, {"AILNMUX", "DIFFINL Mix", "LIN12 PGA"},
{"AILNMUX", "DIFFINL Mix", "LIN34 PGA"}, {"AILNMUX", "RXVOICE Mix", "LIN4/RXN"}, {"AILNMUX", "RXVOICE Mix", "RIN4/RXP"},
{"Left ADC", NULL, "AILNMUX"}, {"RIN12 PGA", "RIN1 Switch", "RIN1"}, {"RIN12 PGA", "RIN2 Switch", "RIN2"},
{"RIN34 PGA", "RIN3 Switch", "RIN3"}, {"RIN34 PGA", "RIN4 Switch", "RIN4/RXP"},
{"INMIXR", NULL, "INR"}, {"INMIXR", "Record Right Volume", "ROMIX"}, {"INMIXR", "RIN2 Volume", "RIN2"},
{"INMIXR", "RINPGA12 Switch", "RIN12 PGA"}, {"INMIXR", "RINPGA34 Switch", "RIN34 PGA"},
{"AIRNMUX", NULL, "INR"}, {"AIRNMUX", "INMIXR Mix", "INMIXR"}, {"AIRNMUX", "DIFFINR Mix", "RIN12 PGA"},
{"AIRNMUX", "DIFFINR Mix", "RIN34 PGA"}, {"AIRNMUX", "RXVOICE Mix", "LIN4/RXN"}, {"AIRNMUX", "RXVOICE Mix", "RIN4/RXP"},
{"Right ADC", NULL, "AIRNMUX"}, {"LOMIX", "LOMIX RIN3 Bypass Switch", "RIN3"}, {"LOMIX", "LOMIX LIN3 Bypass Switch", "LIN3"},
{"LOMIX", "LOMIX LIN12 PGA Bypass Switch", "LIN12 PGA"}, {"LOMIX", "LOMIX RIN12 PGA Bypass Switch", "RIN12 PGA"},
{"LOMIX", "LOMIX Right ADC Bypass Switch", "AIRNMUX"}, {"LOMIX", "LOMIX Left ADC Bypass Switch", "AILNMUX"},
{"LOMIX", "LOMIX Left DAC Switch", "Left DAC"}, {"ROMIX", "ROMIX RIN3 Bypass Switch", "RIN3"},
{"ROMIX", "ROMIX LIN3 Bypass Switch", "LIN3"}, {"ROMIX", "ROMIX LIN12 PGA Bypass Switch", "LIN12 PGA"},
{"ROMIX", "ROMIX RIN12 PGA Bypass Switch", "RIN12 PGA"}, {"ROMIX", "ROMIX Right ADC Bypass Switch", "AIRNMUX"},
{"ROMIX", "ROMIX Left ADC Bypass Switch", "AILNMUX"}, {"ROMIX", "ROMIX Right DAC Switch", "Right DAC"},
{"SPKMIX", "SPKMIX LIN2 Bypass Switch", "LIN2"}, {"SPKMIX", "SPKMIX RIN2 Bypass Switch", "RIN2"},
{"SPKMIX", "SPKMIX LADC Bypass Switch", "AILNMUX"}, {"SPKMIX", "SPKMIX RADC Bypass Switch", "AIRNMUX"},
{"SPKMIX", "SPKMIX Left Mixer PGA Switch", "LOPGA"}, {"SPKMIX", "SPKMIX Right Mixer PGA Switch", "ROPGA"},
{"SPKMIX", "SPKMIX Right DAC Switch", "Right DAC"}, {"SPKMIX", "SPKMIX Left DAC Switch", "Right DAC"},
{"LONMIX", "LONMIX Left Mixer PGA Switch", "LOPGA"}, {"LONMIX", "LONMIX Right Mixer PGA Switch", "ROPGA"},
{"LONMIX", "LONMIX Inverted LOP Switch", "LOPMIX"}, {"LOPMIX", "LOPMIX Right Mic Bypass Switch", "RIN12 PGA"},
{"LOPMIX", "LOPMIX Left Mic Bypass Switch", "LIN12 PGA"}, {"LOPMIX", "LOPMIX Left Mixer PGA Switch", "LOPGA"},
{"OUT3MIX", "OUT3MIX LIN4/RXP Bypass Switch", "LIN4/RXN"}, {"OUT3MIX", "OUT3MIX Left Out PGA Switch", "LOPGA"},
{"OUT4MIX", "OUT4MIX Right Out PGA Switch", "ROPGA"}, {"OUT4MIX", "OUT4MIX RIN4/RXP Bypass Switch", "RIN4/RXP"},
{"RONMIX", "RONMIX Right Mixer PGA Switch", "ROPGA"}, {"RONMIX", "RONMIX Left Mixer PGA Switch", "LOPGA"},
{"RONMIX", "RONMIX Inverted ROP Switch", "ROPMIX"}, {"ROPMIX", "ROPMIX Left Mic Bypass Switch", "LIN12 PGA"},
{"ROPMIX", "ROPMIX Right Mic Bypass Switch", "RIN12 PGA"}, {"ROPMIX", "ROPMIX Right Mixer PGA Switch", "ROPGA"},
{"LOPGA", NULL, "LOMIX"}, {"ROPGA", NULL, "ROMIX"}, {"LOUT PGA", NULL, "LOMIX"}, {"ROUT PGA", NULL, "ROMIX"},
{"LON", NULL, "LONMIX"}, {"LOP", NULL, "LOPMIX"}, {"OUT3", NULL, "OUT3MIX"}, {"LOUT", NULL, "LOUT PGA"},
{"SPKN", NULL, "SPKMIX"}, {"ROUT", NULL, "ROUT PGA"}, {"OUT4", NULL, "OUT4MIX"}, {"ROP", NULL, "ROPMIX"}, {"RON", NULL, "RONMIX"},
};
}

const FIXED_FLL_SIZE: u64 = ((1u64 << 16) * 10);

unsafe fn wm8400_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let wm8400 = snd_soc_component_get_drvdata(component) as *mut wm8400_priv;
    (*wm8400).sysclk = freq;
    0
}

unsafe fn fll_factors(
    wm8400: *mut wm8400_priv,
    factors: *mut fll_factors,
    Fref: c_uint,
    Fout: c_uint,
) -> c_int {
    let mut Kpart: u64;
    let mut K: c_uint;
    let mut Nmod: c_uint;
    let mut target: c_uint;

    (*factors).outdiv = 2;
    while Fout.wrapping_mul((*factors).outdiv as c_uint) < 90_000_000
        || Fout.wrapping_mul((*factors).outdiv as c_uint) > 100_000_000
    {
        (*factors).outdiv = (*factors).outdiv.wrapping_mul(2);
        if (*factors).outdiv > 32 {
            dev_err((*(*wm8400).wm8400).dev, c"Unsupported FLL output frequency %uHz\n".as_ptr(), Fout);
            return -EINVAL;
        }
    }
    target = Fout.wrapping_mul((*factors).outdiv as c_uint);
    (*factors).outdiv >>= 2;

    if Fref < 48000 {
        (*factors).freq_ref = 1;
    } else {
        (*factors).freq_ref = 0;
    }

    if Fref < 1_000_000 {
        (*factors).fratio = 9;
    } else {
        (*factors).fratio = 0;
    }

    /* Ensure we have a fractional part */
    loop {
        if Fref < 1_000_000 {
            (*factors).fratio = (*factors).fratio.wrapping_sub(1);
        } else {
            (*factors).fratio = (*factors).fratio.wrapping_add(1);
        }

        if (*factors).fratio < 1 || (*factors).fratio > 8 {
            dev_err((*(*wm8400).wm8400).dev, c"Unable to calculate FRATIO\n".as_ptr());
            return -EINVAL;
        }

        (*factors).n = (target / Fref.wrapping_mul((*factors).fratio as c_uint)) as u16;
        Nmod = target % Fref.wrapping_mul((*factors).fratio as c_uint);
        if Nmod != 0 {
            break;
        }
    }

    /* Calculate fractional part - scale up so we can round. */
    Kpart = FIXED_FLL_SIZE.wrapping_mul(Nmod as u64);
    Kpart /= Fref.wrapping_mul((*factors).fratio as c_uint) as u64;
    K = (Kpart & 0xFFFF_FFFF) as c_uint;

    if (K % 10) >= 5 {
        K = K.wrapping_add(5);
    }

    /* Move down to proper range now rounding is done */
    (*factors).k = (K / 10) as u16;

    dev_dbg(
        (*(*wm8400).wm8400).dev,
        c"FLL: Fref=%u Fout=%u N=%x K=%x, FRATIO=%x OUTDIV=%x\n".as_ptr(),
        Fref,
        Fout,
        (*factors).n as c_int,
        (*factors).k as c_int,
        (*factors).fratio as c_int,
        (*factors).outdiv as c_int,
    );

    0
}

unsafe fn wm8400_set_dai_pll(
    codec_dai: *mut snd_soc_dai,
    _pll_id: c_int,
    _source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let component = (*codec_dai).component;
    let wm8400 = snd_soc_component_get_drvdata(component) as *mut wm8400_priv;
    let mut factors: fll_factors = mem::zeroed();
    let ret: c_int;
    let mut reg: u16;

    if freq_in as c_int == (*wm8400).fll_in && freq_out as c_int == (*wm8400).fll_out {
        return 0;
    }

    if freq_out != 0 {
        ret = fll_factors(wm8400, &mut factors, freq_in, freq_out);
        if ret != 0 {
            return ret;
        }
    }

    (*wm8400).fll_out = freq_out as c_int;
    (*wm8400).fll_in = freq_in as c_int;

    /* We *must* disable the FLL before any changes */
    reg = snd_soc_component_read(component, WM8400_POWER_MANAGEMENT_2);
    reg &= !(WM8400_FLL_ENA as u16);
    snd_soc_component_write(component, WM8400_POWER_MANAGEMENT_2, reg);

    reg = snd_soc_component_read(component, WM8400_FLL_CONTROL_1);
    reg &= !(WM8400_FLL_OSC_ENA as u16);
    snd_soc_component_write(component, WM8400_FLL_CONTROL_1, reg);

    if freq_out == 0 {
        return 0;
    }

    reg &= !((WM8400_FLL_REF_FREQ | WM8400_FLL_FRATIO_MASK) as u16);
    reg |= (WM8400_FLL_FRAC as u16) | factors.fratio;
    reg |= factors.freq_ref << WM8400_FLL_REF_FREQ_SHIFT;
    snd_soc_component_write(component, WM8400_FLL_CONTROL_1, reg);
    snd_soc_component_write(component, WM8400_FLL_CONTROL_2, factors.k);
    snd_soc_component_write(component, WM8400_FLL_CONTROL_3, factors.n);

    reg = snd_soc_component_read(component, WM8400_FLL_CONTROL_4);
    reg &= !(WM8400_FLL_OUTDIV_MASK as u16);
    reg |= factors.outdiv;
    snd_soc_component_write(component, WM8400_FLL_CONTROL_4, reg);

    0
}

unsafe fn wm8400_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut audio1 = snd_soc_component_read(component, WM8400_AUDIO_INTERFACE_1);
    let mut audio3 = snd_soc_component_read(component, WM8400_AUDIO_INTERFACE_3);

    /* set master/slave audio interface */
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => audio3 &= !(WM8400_AIF_MSTR1 as u16),
        SND_SOC_DAIFMT_CBP_CFP => audio3 |= WM8400_AIF_MSTR1 as u16,
        _ => return -EINVAL,
    }

    audio1 &= !(WM8400_AIF_FMT_MASK as u16);

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            audio1 |= WM8400_AIF_FMT_I2S as u16;
            audio1 &= !(WM8400_AIF_LRCLK_INV as u16);
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            audio1 |= WM8400_AIF_FMT_RIGHTJ as u16;
            audio1 &= !(WM8400_AIF_LRCLK_INV as u16);
        }
        SND_SOC_DAIFMT_LEFT_J => {
            audio1 |= WM8400_AIF_FMT_LEFTJ as u16;
            audio1 &= !(WM8400_AIF_LRCLK_INV as u16);
        }
        SND_SOC_DAIFMT_DSP_A => {
            audio1 |= WM8400_AIF_FMT_DSP as u16;
            audio1 &= !(WM8400_AIF_LRCLK_INV as u16);
        }
        SND_SOC_DAIFMT_DSP_B => {
            audio1 |= (WM8400_AIF_FMT_DSP | WM8400_AIF_LRCLK_INV) as u16;
        }
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, WM8400_AUDIO_INTERFACE_1, audio1);
    snd_soc_component_write(component, WM8400_AUDIO_INTERFACE_3, audio3);
    0
}

unsafe fn wm8400_set_dai_clkdiv(
    codec_dai: *mut snd_soc_dai,
    div_id: c_int,
    div: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let mut reg: u16;

    match div_id {
        WM8400_MCLK_DIV => {
            reg = snd_soc_component_read(component, WM8400_CLOCKING_2) & !(WM8400_MCLK_DIV_MASK as u16);
            snd_soc_component_write(component, WM8400_CLOCKING_2, reg | div as u16);
        }
        WM8400_DACCLK_DIV => {
            reg = snd_soc_component_read(component, WM8400_CLOCKING_2) & !(WM8400_DAC_CLKDIV_MASK as u16);
            snd_soc_component_write(component, WM8400_CLOCKING_2, reg | div as u16);
        }
        WM8400_ADCCLK_DIV => {
            reg = snd_soc_component_read(component, WM8400_CLOCKING_2) & !(WM8400_ADC_CLKDIV_MASK as u16);
            snd_soc_component_write(component, WM8400_CLOCKING_2, reg | div as u16);
        }
        WM8400_BCLK_DIV => {
            reg = snd_soc_component_read(component, WM8400_CLOCKING_1) & !(WM8400_BCLK_DIV_MASK as u16);
            snd_soc_component_write(component, WM8400_CLOCKING_1, reg | div as u16);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe fn wm8400_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let mut audio1 = snd_soc_component_read(component, WM8400_AUDIO_INTERFACE_1);

    audio1 &= !(WM8400_AIF_WL_MASK as u16);
    /* bit size */
    match params_width(params) {
        16 => {}
        20 => audio1 |= WM8400_AIF_WL_20BITS as u16,
        24 => audio1 |= WM8400_AIF_WL_24BITS as u16,
        32 => audio1 |= WM8400_AIF_WL_32BITS as u16,
        _ => {}
    }

    snd_soc_component_write(component, WM8400_AUDIO_INTERFACE_1, audio1);
    0
}

unsafe fn wm8400_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let val = snd_soc_component_read(component, WM8400_DAC_CTRL) & !(WM8400_DAC_MUTE as u16);

    if mute != 0 {
        snd_soc_component_write(component, WM8400_DAC_CTRL, val | WM8400_DAC_MUTE as u16);
    } else {
        snd_soc_component_write(component, WM8400_DAC_CTRL, val);
    }

    0
}

/* TODO: set bias for best performance at standby */
unsafe fn wm8400_set_bias_level(component: *mut snd_soc_component, level: c_int) -> c_int {
    let wm8400 = snd_soc_component_get_drvdata(component) as *mut wm8400_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut val: u16;
    let mut ret: c_int;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            /* VMID=2*50k */
            val = snd_soc_component_read(component, WM8400_POWER_MANAGEMENT_1) & !(WM8400_VMID_MODE_MASK as u16);
            snd_soc_component_write(component, WM8400_POWER_MANAGEMENT_1, val | 0x2);
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                ret = regulator_bulk_enable(ARRAY_SIZE_power, power.as_mut_ptr());
                if ret != 0 {
                    dev_err((*(*wm8400).wm8400).dev, c"Failed to enable regulators: %d\n".as_ptr(), ret);
                    return ret;
                }

                snd_soc_component_write(component, WM8400_POWER_MANAGEMENT_1, (WM8400_CODEC_ENA | WM8400_SYSCLK_ENA) as u16);
                /* Enable POBCTRL, SOFT_ST, VMIDTOG and BUFDCOPEN */
                snd_soc_component_write(component, WM8400_ANTIPOP2, (WM8400_SOFTST | WM8400_BUFDCOPEN | WM8400_POBCTRL) as u16);
                msleep(50);
                /* Enable VREF & VMID at 2x50k */
                val = snd_soc_component_read(component, WM8400_POWER_MANAGEMENT_1);
                val |= 0x2 | WM8400_VREF_ENA as u16;
                snd_soc_component_write(component, WM8400_POWER_MANAGEMENT_1, val);
                /* Enable BUFIOEN */
                snd_soc_component_write(component, WM8400_ANTIPOP2, (WM8400_SOFTST | WM8400_BUFDCOPEN | WM8400_POBCTRL | WM8400_BUFIOEN) as u16);
                /* disable POBCTRL, SOFT_ST and BUFDCOPEN */
                snd_soc_component_write(component, WM8400_ANTIPOP2, WM8400_BUFIOEN as u16);
            }

            /* VMID=2*300k */
            val = snd_soc_component_read(component, WM8400_POWER_MANAGEMENT_1) & !(WM8400_VMID_MODE_MASK as u16);
            snd_soc_component_write(component, WM8400_POWER_MANAGEMENT_1, val | 0x4);
        }
        SND_SOC_BIAS_OFF => {
            /* Enable POBCTRL and SOFT_ST */
            snd_soc_component_write(component, WM8400_ANTIPOP2, (WM8400_SOFTST | WM8400_POBCTRL | WM8400_BUFIOEN) as u16);
            /* Enable POBCTRL, SOFT_ST and BUFDCOPEN */
            snd_soc_component_write(component, WM8400_ANTIPOP2, (WM8400_SOFTST | WM8400_BUFDCOPEN | WM8400_POBCTRL | WM8400_BUFIOEN) as u16);
            /* mute DAC */
            val = snd_soc_component_read(component, WM8400_DAC_CTRL);
            snd_soc_component_write(component, WM8400_DAC_CTRL, val | WM8400_DAC_MUTE as u16);
            /* Enable any disabled outputs */
            val = snd_soc_component_read(component, WM8400_POWER_MANAGEMENT_1);
            val |= (WM8400_SPK_ENA | WM8400_OUT3_ENA | WM8400_OUT4_ENA | WM8400_LOUT_ENA | WM8400_ROUT_ENA) as u16;
            snd_soc_component_write(component, WM8400_POWER_MANAGEMENT_1, val);
            /* Disable VMID */
            val &= !(WM8400_VMID_MODE_MASK as u16);
            snd_soc_component_write(component, WM8400_POWER_MANAGEMENT_1, val);
            msleep(300);
            /* Enable all output discharge bits */
            snd_soc_component_write(component, WM8400_ANTIPOP1, (WM8400_DIS_LLINE | WM8400_DIS_RLINE | WM8400_DIS_OUT3 | WM8400_DIS_OUT4 | WM8400_DIS_LOUT | WM8400_DIS_ROUT) as u16);
            /* Disable VREF */
            val &= !(WM8400_VREF_ENA as u16);
            snd_soc_component_write(component, WM8400_POWER_MANAGEMENT_1, val);
            /* disable POBCTRL, SOFT_ST and BUFDCOPEN */
            snd_soc_component_write(component, WM8400_ANTIPOP2, 0x0);

            ret = regulator_bulk_disable(ARRAY_SIZE_power, power.as_mut_ptr());
            if ret != 0 {
                return ret;
            }
        }
        _ => {}
    }

    0
}

unsafe fn wm8400_component_probe(component: *mut snd_soc_component) -> c_int {
    let wm8400 = dev_get_platdata((*component).dev);
    let priv_data: *mut wm8400_priv;
    let mut ret: c_int;
    let mut reg: u16;

    priv_data = devm_kzalloc((*component).dev, mem::size_of::<wm8400_priv>(), GFP_KERNEL) as *mut wm8400_priv;
    if priv_data.is_null() {
        return -ENOMEM;
    }

    snd_soc_component_init_regmap(component, (*wm8400).regmap);
    snd_soc_component_set_drvdata(component, priv_data as *mut c_void);
    (*priv_data).wm8400 = wm8400;

    ret = devm_regulator_bulk_get((*wm8400).dev, ARRAY_SIZE_power, power.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, c"Failed to get regulators: %d\n".as_ptr(), ret);
        return ret;
    }

    wm8400_component_reset(component);

    reg = snd_soc_component_read(component, WM8400_POWER_MANAGEMENT_1);
    snd_soc_component_write(component, WM8400_POWER_MANAGEMENT_1, reg | WM8400_CODEC_ENA as u16);

    /* Latch volume update bits */
    reg = snd_soc_component_read(component, WM8400_LEFT_LINE_INPUT_1_2_VOLUME);
    snd_soc_component_write(component, WM8400_LEFT_LINE_INPUT_1_2_VOLUME, reg & WM8400_IPVU as u16);
    reg = snd_soc_component_read(component, WM8400_RIGHT_LINE_INPUT_1_2_VOLUME);
    snd_soc_component_write(component, WM8400_RIGHT_LINE_INPUT_1_2_VOLUME, reg & WM8400_IPVU as u16);

    snd_soc_component_write(component, WM8400_LEFT_OUTPUT_VOLUME, 0x50 | (1 << 8));
    snd_soc_component_write(component, WM8400_RIGHT_OUTPUT_VOLUME, 0x50 | (1 << 8));

    0
}

unsafe fn wm8400_component_remove(component: *mut snd_soc_component) {
    let mut reg: u16;

    reg = snd_soc_component_read(component, WM8400_POWER_MANAGEMENT_1);
    snd_soc_component_write(component, WM8400_POWER_MANAGEMENT_1, reg & !(WM8400_CODEC_ENA as u16));
}

kernel_c_items! {
#define WM8400_RATES SNDRV_PCM_RATE_8000_96000
#define WM8400_FORMATS (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE)

static const struct snd_soc_dai_ops wm8400_dai_ops = {
	.hw_params = wm8400_hw_params,
	.mute_stream = wm8400_mute,
	.set_fmt = wm8400_set_dai_fmt,
	.set_clkdiv = wm8400_set_dai_clkdiv,
	.set_sysclk = wm8400_set_dai_sysclk,
	.set_pll = wm8400_set_dai_pll,
	.no_capture_mute = 1,
};

/*
 * The WM8400 supports 2 different and mutually exclusive DAI
 * configurations.
 *
 * 1. ADC/DAC on Primary Interface
 * 2. ADC on Primary Interface/DAC on secondary
 */
static struct snd_soc_dai_driver wm8400_dai = {
	.name = "wm8400-hifi",
	.playback = {
		.stream_name = "Playback",
		.channels_min = 1,
		.channels_max = 2,
		.rates = WM8400_RATES,
		.formats = WM8400_FORMATS,
	},
	.capture = {
		.stream_name = "Capture",
		.channels_min = 1,
		.channels_max = 2,
		.rates = WM8400_RATES,
		.formats = WM8400_FORMATS,
	},
	.ops = &wm8400_dai_ops,
};

static const struct snd_soc_component_driver soc_component_dev_wm8400 = {
	.probe			= wm8400_component_probe,
	.remove			= wm8400_component_remove,
	.set_bias_level		= wm8400_set_bias_level,
	.controls		= wm8400_snd_controls,
	.num_controls		= ARRAY_SIZE(wm8400_snd_controls),
	.dapm_widgets		= wm8400_dapm_widgets,
	.num_dapm_widgets	= ARRAY_SIZE(wm8400_dapm_widgets),
	.dapm_routes		= wm8400_dapm_routes,
	.num_dapm_routes	= ARRAY_SIZE(wm8400_dapm_routes),
	.suspend_bias_off	= 1,
	.idle_bias_on		= 1,
	.use_pmdown_time	= 1,
	.endianness		= 1,
};
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

extern "C" {
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const c_void,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
}

unsafe fn wm8400_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_wm8400 as *const _ as *const c_void,
        &mut wm8400_dai as *mut _ as *mut c_void,
        1,
    )
}

kernel_c_items! {
static struct platform_driver wm8400_codec_driver = {
	.driver = {
		   .name = "wm8400-codec",
		   },
	.probe = wm8400_probe,
};

module_platform_driver(wm8400_codec_driver);

MODULE_DESCRIPTION("ASoC WM8400 driver");
MODULE_AUTHOR("Mark Brown");
MODULE_LICENSE("GPL");
MODULE_ALIAS("platform:wm8400-codec");
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
