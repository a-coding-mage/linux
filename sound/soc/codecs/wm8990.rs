// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * wm8990.c  --  WM8990 ALSA Soc Audio driver
 *
 * Copyright 2008 Wolfson Microelectronics PLC.
 * Author: Liam Girdwood <lrg@slimlogic.co.uk>
 */

/* Translated from Linux C source.  Kernel, ASoC, regmap, I2C and WM8990
 * register definitions are external dependencies from the original includes:
 * linux/module.h, linux/moduleparam.h, linux/kernel.h, linux/init.h,
 * linux/delay.h, linux/pm.h, linux/i2c.h, linux/regmap.h, linux/slab.h,
 * sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
 * sound/initval.h, sound/tlv.h, asm/div64.h, and "wm8990.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
pub struct soc_mixer_control {
    pub reg: c_uint,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
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
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

#[repr(C)]
struct wm8990_priv {
    regmap: *mut regmap,
    sysclk: c_uint,
    pcmclk: c_uint,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn msleep(msecs: c_uint);
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

macro_rules! wm8990_reset {
    ($c:expr) => {
        snd_soc_component_write($c, WM8990_RESET, 0)
    };
}

macro_rules! SOC_WM899X_OUTPGA_SINGLE_R_TLV {
    ($xname:expr, $reg:expr, $shift:expr, $max:expr, $invert:expr, $tlv_array:expr) => {
        SOC_SINGLE_EXT_TLV!(
            $xname,
            $reg,
            $shift,
            $max,
            $invert,
            snd_soc_get_volsw,
            wm899x_outpga_put_volsw_vu,
            $tlv_array
        )
    };
}

DECLARE_TLV_DB_SCALE!(in_pga_tlv, -1650, 3000, 0);
DECLARE_TLV_DB_SCALE!(out_mix_tlv, 0, -2100, 0);
DECLARE_TLV_DB_SCALE!(out_pga_tlv, -7300, 600, 0);
DECLARE_TLV_DB_SCALE!(out_dac_tlv, -7163, 0, 0);
DECLARE_TLV_DB_SCALE!(in_adc_tlv, -7163, 1763, 0);
DECLARE_TLV_DB_SCALE!(out_sidetone_tlv, -3600, 0, 0);

unsafe extern "C" fn wm899x_outpga_put_volsw_vu(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let reg = (*mc).reg;
    let ret: c_int;
    let val: u16;

    ret = snd_soc_put_volsw(kcontrol, ucontrol);
    if ret < 0 {
        return ret;
    }

    /* now hit the volume update bits (always bit 8) */
    val = snd_soc_component_read(component, reg) as u16;
    snd_soc_component_write(component, reg, (val | 0x0100) as c_uint)
}

static wm8990_digital_sidetone: [&[u8]; 4] = [b"None\0", b"Left ADC\0", b"Right ADC\0", b"Reserved\0"];
SOC_ENUM_SINGLE_DECL!(wm8990_left_digital_sidetone_enum, WM8990_DIGITAL_SIDE_TONE, WM8990_ADC_TO_DACL_SHIFT, wm8990_digital_sidetone);
SOC_ENUM_SINGLE_DECL!(wm8990_right_digital_sidetone_enum, WM8990_DIGITAL_SIDE_TONE, WM8990_ADC_TO_DACR_SHIFT, wm8990_digital_sidetone);

static wm8990_adcmode: [&[u8]; 4] = [b"Hi-fi mode\0", b"Voice mode 1\0", b"Voice mode 2\0", b"Voice mode 3\0"];
SOC_ENUM_SINGLE_DECL!(wm8990_right_adcmode_enum, WM8990_ADC_CTRL, WM8990_ADC_HPF_CUT_SHIFT, wm8990_adcmode);

/* The following static control, widget, route, DAI, component, I2C and module
 * declarations are direct Rust macro-shaped translations of the ASoC C
 * initializer macros in this file.  The concrete structures/macros are supplied
 * by external kernel bindings.
 */

static wm8990_snd_controls: &[snd_kcontrol_new] = &[
SOC_SINGLE!("LIN12 PGA Boost", WM8990_INPUT_MIXER3, WM8990_L12MNBST_BIT, 1, 0),
SOC_SINGLE!("LIN34 PGA Boost", WM8990_INPUT_MIXER3, WM8990_L34MNBST_BIT, 1, 0),
SOC_SINGLE!("RIN12 PGA Boost", WM8990_INPUT_MIXER3, WM8990_R12MNBST_BIT, 1, 0),
SOC_SINGLE!("RIN34 PGA Boost", WM8990_INPUT_MIXER3, WM8990_R34MNBST_BIT, 1, 0),
SOC_SINGLE_TLV!("LOMIX LIN3 Bypass Volume", WM8990_OUTPUT_MIXER3, WM8990_LLI3LOVOL_SHIFT, WM8990_LLI3LOVOL_MASK, 1, out_mix_tlv),
SOC_SINGLE_TLV!("LOMIX RIN12 PGA Bypass Volume", WM8990_OUTPUT_MIXER3, WM8990_LR12LOVOL_SHIFT, WM8990_LR12LOVOL_MASK, 1, out_mix_tlv),
SOC_SINGLE_TLV!("LOMIX LIN12 PGA Bypass Volume", WM8990_OUTPUT_MIXER3, WM8990_LL12LOVOL_SHIFT, WM8990_LL12LOVOL_MASK, 1, out_mix_tlv),
SOC_SINGLE_TLV!("LOMIX RIN3 Bypass Volume", WM8990_OUTPUT_MIXER5, WM8990_LRI3LOVOL_SHIFT, WM8990_LRI3LOVOL_MASK, 1, out_mix_tlv),
SOC_SINGLE_TLV!("LOMIX AINRMUX Bypass Volume", WM8990_OUTPUT_MIXER5, WM8990_LRBLOVOL_SHIFT, WM8990_LRBLOVOL_MASK, 1, out_mix_tlv),
SOC_SINGLE_TLV!("LOMIX AINLMUX Bypass Volume", WM8990_OUTPUT_MIXER5, WM8990_LRBLOVOL_SHIFT, WM8990_LRBLOVOL_MASK, 1, out_mix_tlv),
SOC_SINGLE_TLV!("ROMIX RIN3 Bypass Volume", WM8990_OUTPUT_MIXER4, WM8990_RRI3ROVOL_SHIFT, WM8990_RRI3ROVOL_MASK, 1, out_mix_tlv),
SOC_SINGLE_TLV!("ROMIX LIN12 PGA Bypass Volume", WM8990_OUTPUT_MIXER4, WM8990_RL12ROVOL_SHIFT, WM8990_RL12ROVOL_MASK, 1, out_mix_tlv),
SOC_SINGLE_TLV!("ROMIX RIN12 PGA Bypass Volume", WM8990_OUTPUT_MIXER4, WM8990_RR12ROVOL_SHIFT, WM8990_RR12ROVOL_MASK, 1, out_mix_tlv),
SOC_SINGLE_TLV!("ROMIX LIN3 Bypass Volume", WM8990_OUTPUT_MIXER6, WM8990_RLI3ROVOL_SHIFT, WM8990_RLI3ROVOL_MASK, 1, out_mix_tlv),
SOC_SINGLE_TLV!("ROMIX AINLMUX Bypass Volume", WM8990_OUTPUT_MIXER6, WM8990_RLBROVOL_SHIFT, WM8990_RLBROVOL_MASK, 1, out_mix_tlv),
SOC_SINGLE_TLV!("ROMIX AINRMUX Bypass Volume", WM8990_OUTPUT_MIXER6, WM8990_RRBROVOL_SHIFT, WM8990_RRBROVOL_MASK, 1, out_mix_tlv),
SOC_WM899X_OUTPGA_SINGLE_R_TLV!("LOUT Volume", WM8990_LEFT_OUTPUT_VOLUME, WM8990_LOUTVOL_SHIFT, WM8990_LOUTVOL_MASK, 0, out_pga_tlv),
SOC_SINGLE!("LOUT ZC", WM8990_LEFT_OUTPUT_VOLUME, WM8990_LOZC_BIT, 1, 0),
SOC_WM899X_OUTPGA_SINGLE_R_TLV!("ROUT Volume", WM8990_RIGHT_OUTPUT_VOLUME, WM8990_ROUTVOL_SHIFT, WM8990_ROUTVOL_MASK, 0, out_pga_tlv),
SOC_SINGLE!("ROUT ZC", WM8990_RIGHT_OUTPUT_VOLUME, WM8990_ROZC_BIT, 1, 0),
SOC_WM899X_OUTPGA_SINGLE_R_TLV!("LOPGA Volume", WM8990_LEFT_OPGA_VOLUME, WM8990_LOPGAVOL_SHIFT, WM8990_LOPGAVOL_MASK, 0, out_pga_tlv),
SOC_SINGLE!("LOPGA ZC Switch", WM8990_LEFT_OPGA_VOLUME, WM8990_LOPGAZC_BIT, 1, 0),
SOC_WM899X_OUTPGA_SINGLE_R_TLV!("ROPGA Volume", WM8990_RIGHT_OPGA_VOLUME, WM8990_ROPGAVOL_SHIFT, WM8990_ROPGAVOL_MASK, 0, out_pga_tlv),
SOC_SINGLE!("ROPGA ZC Switch", WM8990_RIGHT_OPGA_VOLUME, WM8990_ROPGAZC_BIT, 1, 0),
SOC_SINGLE!("LON Mute Switch", WM8990_LINE_OUTPUTS_VOLUME, WM8990_LONMUTE_BIT, 1, 0),
SOC_SINGLE!("LOP Mute Switch", WM8990_LINE_OUTPUTS_VOLUME, WM8990_LOPMUTE_BIT, 1, 0),
SOC_SINGLE!("LOP Attenuation Switch", WM8990_LINE_OUTPUTS_VOLUME, WM8990_LOATTN_BIT, 1, 0),
SOC_SINGLE!("RON Mute Switch", WM8990_LINE_OUTPUTS_VOLUME, WM8990_RONMUTE_BIT, 1, 0),
SOC_SINGLE!("ROP Mute Switch", WM8990_LINE_OUTPUTS_VOLUME, WM8990_ROPMUTE_BIT, 1, 0),
SOC_SINGLE!("ROP Attenuation Switch", WM8990_LINE_OUTPUTS_VOLUME, WM8990_ROATTN_BIT, 1, 0),
SOC_SINGLE!("OUT3 Mute Switch", WM8990_OUT3_4_VOLUME, WM8990_OUT3MUTE_BIT, 1, 0),
SOC_SINGLE!("OUT3 Attenuation Switch", WM8990_OUT3_4_VOLUME, WM8990_OUT3ATTN_BIT, 1, 0),
SOC_SINGLE!("OUT4 Mute Switch", WM8990_OUT3_4_VOLUME, WM8990_OUT4MUTE_BIT, 1, 0),
SOC_SINGLE!("OUT4 Attenuation Switch", WM8990_OUT3_4_VOLUME, WM8990_OUT4ATTN_BIT, 1, 0),
SOC_SINGLE!("Speaker Mode Switch", WM8990_CLASSD1, WM8990_CDMODE_BIT, 1, 0),
SOC_SINGLE!("Speaker Output Attenuation Volume", WM8990_SPEAKER_VOLUME, WM8990_SPKATTN_SHIFT, WM8990_SPKATTN_MASK, 0),
SOC_SINGLE!("Speaker DC Boost Volume", WM8990_CLASSD3, WM8990_DCGAIN_SHIFT, WM8990_DCGAIN_MASK, 0),
SOC_SINGLE!("Speaker AC Boost Volume", WM8990_CLASSD3, WM8990_ACGAIN_SHIFT, WM8990_ACGAIN_MASK, 0),
SOC_SINGLE_TLV!("Speaker Volume", WM8990_CLASSD4, WM8990_SPKVOL_SHIFT, WM8990_SPKVOL_MASK, 0, out_pga_tlv),
SOC_SINGLE!("Speaker ZC Switch", WM8990_CLASSD4, WM8990_SPKZC_SHIFT, WM8990_SPKZC_MASK, 0),
SOC_WM899X_OUTPGA_SINGLE_R_TLV!("Left DAC Digital Volume", WM8990_LEFT_DAC_DIGITAL_VOLUME, WM8990_DACL_VOL_SHIFT, WM8990_DACL_VOL_MASK, 0, out_dac_tlv),
SOC_WM899X_OUTPGA_SINGLE_R_TLV!("Right DAC Digital Volume", WM8990_RIGHT_DAC_DIGITAL_VOLUME, WM8990_DACR_VOL_SHIFT, WM8990_DACR_VOL_MASK, 0, out_dac_tlv),
SOC_ENUM!("Left Digital Sidetone", wm8990_left_digital_sidetone_enum),
SOC_ENUM!("Right Digital Sidetone", wm8990_right_digital_sidetone_enum),
SOC_SINGLE_TLV!("Left Digital Sidetone Volume", WM8990_DIGITAL_SIDE_TONE, WM8990_ADCL_DAC_SVOL_SHIFT, WM8990_ADCL_DAC_SVOL_MASK, 0, out_sidetone_tlv),
SOC_SINGLE_TLV!("Right Digital Sidetone Volume", WM8990_DIGITAL_SIDE_TONE, WM8990_ADCR_DAC_SVOL_SHIFT, WM8990_ADCR_DAC_SVOL_MASK, 0, out_sidetone_tlv),
SOC_SINGLE!("ADC Digital High Pass Filter Switch", WM8990_ADC_CTRL, WM8990_ADC_HPF_ENA_BIT, 1, 0),
SOC_ENUM!("ADC HPF Mode", wm8990_right_adcmode_enum),
SOC_WM899X_OUTPGA_SINGLE_R_TLV!("Left ADC Digital Volume", WM8990_LEFT_ADC_DIGITAL_VOLUME, WM8990_ADCL_VOL_SHIFT, WM8990_ADCL_VOL_MASK, 0, in_adc_tlv),
SOC_WM899X_OUTPGA_SINGLE_R_TLV!("Right ADC Digital Volume", WM8990_RIGHT_ADC_DIGITAL_VOLUME, WM8990_ADCR_VOL_SHIFT, WM8990_ADCR_VOL_MASK, 0, in_adc_tlv),
SOC_WM899X_OUTPGA_SINGLE_R_TLV!("LIN12 Volume", WM8990_LEFT_LINE_INPUT_1_2_VOLUME, WM8990_LIN12VOL_SHIFT, WM8990_LIN12VOL_MASK, 0, in_pga_tlv),
SOC_SINGLE!("LIN12 ZC Switch", WM8990_LEFT_LINE_INPUT_1_2_VOLUME, WM8990_LI12ZC_BIT, 1, 0),
SOC_SINGLE!("LIN12 Mute Switch", WM8990_LEFT_LINE_INPUT_1_2_VOLUME, WM8990_LI12MUTE_BIT, 1, 0),
SOC_WM899X_OUTPGA_SINGLE_R_TLV!("LIN34 Volume", WM8990_LEFT_LINE_INPUT_3_4_VOLUME, WM8990_LIN34VOL_SHIFT, WM8990_LIN34VOL_MASK, 0, in_pga_tlv),
SOC_SINGLE!("LIN34 ZC Switch", WM8990_LEFT_LINE_INPUT_3_4_VOLUME, WM8990_LI34ZC_BIT, 1, 0),
SOC_SINGLE!("LIN34 Mute Switch", WM8990_LEFT_LINE_INPUT_3_4_VOLUME, WM8990_LI34MUTE_BIT, 1, 0),
SOC_WM899X_OUTPGA_SINGLE_R_TLV!("RIN12 Volume", WM8990_RIGHT_LINE_INPUT_1_2_VOLUME, WM8990_RIN12VOL_SHIFT, WM8990_RIN12VOL_MASK, 0, in_pga_tlv),
SOC_SINGLE!("RIN12 ZC Switch", WM8990_RIGHT_LINE_INPUT_1_2_VOLUME, WM8990_RI12ZC_BIT, 1, 0),
SOC_SINGLE!("RIN12 Mute Switch", WM8990_RIGHT_LINE_INPUT_1_2_VOLUME, WM8990_RI12MUTE_BIT, 1, 0),
SOC_WM899X_OUTPGA_SINGLE_R_TLV!("RIN34 Volume", WM8990_RIGHT_LINE_INPUT_3_4_VOLUME, WM8990_RIN34VOL_SHIFT, WM8990_RIN34VOL_MASK, 0, in_pga_tlv),
SOC_SINGLE!("RIN34 ZC Switch", WM8990_RIGHT_LINE_INPUT_3_4_VOLUME, WM8990_RI34ZC_BIT, 1, 0),
SOC_SINGLE!("RIN34 Mute Switch", WM8990_RIGHT_LINE_INPUT_3_4_VOLUME, WM8990_RI34MUTE_BIT, 1, 0),
];

unsafe extern "C" fn outmixer_event(
    w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let reg_shift: u32 = ((*kcontrol).private_value & 0xfff) as u32;
    let mut ret: c_int = 0;
    let reg: u16;

    match reg_shift {
        x if x == (WM8990_SPEAKER_MIXER | (WM8990_LDSPK_BIT << 8)) => {
            reg = snd_soc_component_read(component, WM8990_OUTPUT_MIXER1) as u16;
            if (reg as c_uint & WM8990_LDLO) != 0 {
                printk(c"Cannot set as Output Mixer 1 LDLO Set\n".as_ptr());
                ret = -1;
            }
        }
        x if x == (WM8990_SPEAKER_MIXER | (WM8990_RDSPK_BIT << 8)) => {
            reg = snd_soc_component_read(component, WM8990_OUTPUT_MIXER2) as u16;
            if (reg as c_uint & WM8990_RDRO) != 0 {
                printk(c"Cannot set as Output Mixer 2 RDRO Set\n".as_ptr());
                ret = -1;
            }
        }
        x if x == (WM8990_OUTPUT_MIXER1 | (WM8990_LDLO_BIT << 8)) => {
            reg = snd_soc_component_read(component, WM8990_SPEAKER_MIXER) as u16;
            if (reg as c_uint & WM8990_LDSPK) != 0 {
                printk(c"Cannot set as Speaker Mixer LDSPK Set\n".as_ptr());
                ret = -1;
            }
        }
        x if x == (WM8990_OUTPUT_MIXER2 | (WM8990_RDRO_BIT << 8)) => {
            reg = snd_soc_component_read(component, WM8990_SPEAKER_MIXER) as u16;
            if (reg as c_uint & WM8990_RDSPK) != 0 {
                printk(c"Cannot set as Speaker Mixer RDSPK Set\n".as_ptr());
                ret = -1;
            }
        }
        _ => {}
    }

    ret
}

DECLARE_TLV_DB_SCALE!(in_mix_tlv, -1200, 600, 0);

static wm8990_dapm_lin12_pga_controls: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("LIN1 Switch", WM8990_INPUT_MIXER2, WM8990_LMN1_BIT, 1, 0), SOC_DAPM_SINGLE!("LIN2 Switch", WM8990_INPUT_MIXER2, WM8990_LMP2_BIT, 1, 0)];
static wm8990_dapm_lin34_pga_controls: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("LIN3 Switch", WM8990_INPUT_MIXER2, WM8990_LMN3_BIT, 1, 0), SOC_DAPM_SINGLE!("LIN4 Switch", WM8990_INPUT_MIXER2, WM8990_LMP4_BIT, 1, 0)];
static wm8990_dapm_rin12_pga_controls: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("RIN1 Switch", WM8990_INPUT_MIXER2, WM8990_RMN1_BIT, 1, 0), SOC_DAPM_SINGLE!("RIN2 Switch", WM8990_INPUT_MIXER2, WM8990_RMP2_BIT, 1, 0)];
static wm8990_dapm_rin34_pga_controls: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("RIN3 Switch", WM8990_INPUT_MIXER2, WM8990_RMN3_BIT, 1, 0), SOC_DAPM_SINGLE!("RIN4 Switch", WM8990_INPUT_MIXER2, WM8990_RMP4_BIT, 1, 0)];

static wm8990_dapm_inmixl_controls: &[snd_kcontrol_new] = &[
SOC_DAPM_SINGLE_TLV!("Record Left Volume", WM8990_INPUT_MIXER3, WM8990_LDBVOL_SHIFT, WM8990_LDBVOL_MASK, 0, in_mix_tlv),
SOC_DAPM_SINGLE_TLV!("LIN2 Volume", WM8990_INPUT_MIXER5, WM8990_LI2BVOL_SHIFT, 7, 0, in_mix_tlv),
SOC_DAPM_SINGLE!("LINPGA12 Switch", WM8990_INPUT_MIXER3, WM8990_L12MNB_BIT, 1, 0),
SOC_DAPM_SINGLE!("LINPGA34 Switch", WM8990_INPUT_MIXER3, WM8990_L34MNB_BIT, 1, 0),
];
static wm8990_dapm_inmixr_controls: &[snd_kcontrol_new] = &[
SOC_DAPM_SINGLE_TLV!("Record Right Volume", WM8990_INPUT_MIXER4, WM8990_RDBVOL_SHIFT, WM8990_RDBVOL_MASK, 0, in_mix_tlv),
SOC_DAPM_SINGLE_TLV!("RIN2 Volume", WM8990_INPUT_MIXER6, WM8990_RI2BVOL_SHIFT, 7, 0, in_mix_tlv),
SOC_DAPM_SINGLE!("RINPGA12 Switch", WM8990_INPUT_MIXER3, WM8990_L12MNB_BIT, 1, 0),
SOC_DAPM_SINGLE!("RINPGA34 Switch", WM8990_INPUT_MIXER3, WM8990_L34MNB_BIT, 1, 0),
];
static wm8990_ainlmux: [&[u8]; 3] = [b"INMIXL Mix\0", b"RXVOICE Mix\0", b"DIFFINL Mix\0"];
SOC_ENUM_SINGLE_DECL!(wm8990_ainlmux_enum, WM8990_INPUT_MIXER1, WM8990_AINLMODE_SHIFT, wm8990_ainlmux);
static wm8990_dapm_ainlmux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8990_ainlmux_enum);
static wm8990_ainrmux: [&[u8]; 3] = [b"INMIXR Mix\0", b"RXVOICE Mix\0", b"DIFFINR Mix\0"];
SOC_ENUM_SINGLE_DECL!(wm8990_ainrmux_enum, WM8990_INPUT_MIXER1, WM8990_AINRMODE_SHIFT, wm8990_ainrmux);
static wm8990_dapm_ainrmux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm8990_ainrmux_enum);

static wm8990_dapm_lomix_controls: &[snd_kcontrol_new] = &[
SOC_DAPM_SINGLE!("LOMIX Right ADC Bypass Switch", WM8990_OUTPUT_MIXER1, WM8990_LRBLO_BIT, 1, 0),
SOC_DAPM_SINGLE!("LOMIX Left ADC Bypass Switch", WM8990_OUTPUT_MIXER1, WM8990_LLBLO_BIT, 1, 0),
SOC_DAPM_SINGLE!("LOMIX RIN3 Bypass Switch", WM8990_OUTPUT_MIXER1, WM8990_LRI3LO_BIT, 1, 0),
SOC_DAPM_SINGLE!("LOMIX LIN3 Bypass Switch", WM8990_OUTPUT_MIXER1, WM8990_LLI3LO_BIT, 1, 0),
SOC_DAPM_SINGLE!("LOMIX RIN12 PGA Bypass Switch", WM8990_OUTPUT_MIXER1, WM8990_LR12LO_BIT, 1, 0),
SOC_DAPM_SINGLE!("LOMIX LIN12 PGA Bypass Switch", WM8990_OUTPUT_MIXER1, WM8990_LL12LO_BIT, 1, 0),
SOC_DAPM_SINGLE!("LOMIX Left DAC Switch", WM8990_OUTPUT_MIXER1, WM8990_LDLO_BIT, 1, 0),
];
static wm8990_dapm_romix_controls: &[snd_kcontrol_new] = &[
SOC_DAPM_SINGLE!("ROMIX Left ADC Bypass Switch", WM8990_OUTPUT_MIXER2, WM8990_RLBRO_BIT, 1, 0),
SOC_DAPM_SINGLE!("ROMIX Right ADC Bypass Switch", WM8990_OUTPUT_MIXER2, WM8990_RRBRO_BIT, 1, 0),
SOC_DAPM_SINGLE!("ROMIX LIN3 Bypass Switch", WM8990_OUTPUT_MIXER2, WM8990_RLI3RO_BIT, 1, 0),
SOC_DAPM_SINGLE!("ROMIX RIN3 Bypass Switch", WM8990_OUTPUT_MIXER2, WM8990_RRI3RO_BIT, 1, 0),
SOC_DAPM_SINGLE!("ROMIX LIN12 PGA Bypass Switch", WM8990_OUTPUT_MIXER2, WM8990_RL12RO_BIT, 1, 0),
SOC_DAPM_SINGLE!("ROMIX RIN12 PGA Bypass Switch", WM8990_OUTPUT_MIXER2, WM8990_RR12RO_BIT, 1, 0),
SOC_DAPM_SINGLE!("ROMIX Right DAC Switch", WM8990_OUTPUT_MIXER2, WM8990_RDRO_BIT, 1, 0),
];
static wm8990_dapm_lonmix_controls: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("LONMIX Left Mixer PGA Switch", WM8990_LINE_MIXER1, WM8990_LLOPGALON_BIT, 1, 0), SOC_DAPM_SINGLE!("LONMIX Right Mixer PGA Switch", WM8990_LINE_MIXER1, WM8990_LROPGALON_BIT, 1, 0), SOC_DAPM_SINGLE!("LONMIX Inverted LOP Switch", WM8990_LINE_MIXER1, WM8990_LOPLON_BIT, 1, 0)];
static wm8990_dapm_lopmix_controls: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("LOPMIX Right Mic Bypass Switch", WM8990_LINE_MIXER1, WM8990_LR12LOP_BIT, 1, 0), SOC_DAPM_SINGLE!("LOPMIX Left Mic Bypass Switch", WM8990_LINE_MIXER1, WM8990_LL12LOP_BIT, 1, 0), SOC_DAPM_SINGLE!("LOPMIX Left Mixer PGA Switch", WM8990_LINE_MIXER1, WM8990_LLOPGALOP_BIT, 1, 0)];
static wm8990_dapm_ronmix_controls: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("RONMIX Right Mixer PGA Switch", WM8990_LINE_MIXER2, WM8990_RROPGARON_BIT, 1, 0), SOC_DAPM_SINGLE!("RONMIX Left Mixer PGA Switch", WM8990_LINE_MIXER2, WM8990_RLOPGARON_BIT, 1, 0), SOC_DAPM_SINGLE!("RONMIX Inverted ROP Switch", WM8990_LINE_MIXER2, WM8990_ROPRON_BIT, 1, 0)];
static wm8990_dapm_ropmix_controls: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("ROPMIX Left Mic Bypass Switch", WM8990_LINE_MIXER2, WM8990_RL12ROP_BIT, 1, 0), SOC_DAPM_SINGLE!("ROPMIX Right Mic Bypass Switch", WM8990_LINE_MIXER2, WM8990_RR12ROP_BIT, 1, 0), SOC_DAPM_SINGLE!("ROPMIX Right Mixer PGA Switch", WM8990_LINE_MIXER2, WM8990_RROPGAROP_BIT, 1, 0)];
static wm8990_dapm_out3mix_controls: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("OUT3MIX LIN4/RXP Bypass Switch", WM8990_OUT3_4_MIXER, WM8990_LI4O3_BIT, 1, 0), SOC_DAPM_SINGLE!("OUT3MIX Left Out PGA Switch", WM8990_OUT3_4_MIXER, WM8990_LPGAO3_BIT, 1, 0)];
static wm8990_dapm_out4mix_controls: &[snd_kcontrol_new] = &[SOC_DAPM_SINGLE!("OUT4MIX Right Out PGA Switch", WM8990_OUT3_4_MIXER, WM8990_RPGAO4_BIT, 1, 0), SOC_DAPM_SINGLE!("OUT4MIX RIN4/RXP Bypass Switch", WM8990_OUT3_4_MIXER, WM8990_RI4O4_BIT, 1, 0)];
static wm8990_dapm_spkmix_controls: &[snd_kcontrol_new] = &[
SOC_DAPM_SINGLE!("SPKMIX LIN2 Bypass Switch", WM8990_SPEAKER_MIXER, WM8990_LI2SPK_BIT, 1, 0),
SOC_DAPM_SINGLE!("SPKMIX LADC Bypass Switch", WM8990_SPEAKER_MIXER, WM8990_LB2SPK_BIT, 1, 0),
SOC_DAPM_SINGLE!("SPKMIX Left Mixer PGA Switch", WM8990_SPEAKER_MIXER, WM8990_LOPGASPK_BIT, 1, 0),
SOC_DAPM_SINGLE!("SPKMIX Left DAC Switch", WM8990_SPEAKER_MIXER, WM8990_LDSPK_BIT, 1, 0),
SOC_DAPM_SINGLE!("SPKMIX Right DAC Switch", WM8990_SPEAKER_MIXER, WM8990_RDSPK_BIT, 1, 0),
SOC_DAPM_SINGLE!("SPKMIX Right Mixer PGA Switch", WM8990_SPEAKER_MIXER, WM8990_ROPGASPK_BIT, 1, 0),
SOC_DAPM_SINGLE!("SPKMIX RADC Bypass Switch", WM8990_SPEAKER_MIXER, WM8990_RL12ROP_BIT, 1, 0),
SOC_DAPM_SINGLE!("SPKMIX RIN2 Bypass Switch", WM8990_SPEAKER_MIXER, WM8990_RI2SPK_BIT, 1, 0),
];

static wm8990_dapm_widgets: &[snd_soc_dapm_widget] = &[
SND_SOC_DAPM_INPUT!("LIN1"), SND_SOC_DAPM_INPUT!("LIN2"), SND_SOC_DAPM_INPUT!("LIN3"),
SND_SOC_DAPM_INPUT!("LIN4/RXN"), SND_SOC_DAPM_INPUT!("RIN3"), SND_SOC_DAPM_INPUT!("RIN4/RXP"),
SND_SOC_DAPM_INPUT!("RIN1"), SND_SOC_DAPM_INPUT!("RIN2"), SND_SOC_DAPM_INPUT!("Internal ADC Source"),
SND_SOC_DAPM_SUPPLY!("INL", WM8990_POWER_MANAGEMENT_2, WM8990_AINL_ENA_BIT, 0, None, 0),
SND_SOC_DAPM_SUPPLY!("INR", WM8990_POWER_MANAGEMENT_2, WM8990_AINR_ENA_BIT, 0, None, 0),
SND_SOC_DAPM_ADC!("Left ADC", "Left Capture", WM8990_POWER_MANAGEMENT_2, WM8990_ADCL_ENA_BIT, 0),
SND_SOC_DAPM_ADC!("Right ADC", "Right Capture", WM8990_POWER_MANAGEMENT_2, WM8990_ADCR_ENA_BIT, 0),
SND_SOC_DAPM_MIXER!("LIN12 PGA", WM8990_POWER_MANAGEMENT_2, WM8990_LIN12_ENA_BIT, 0, &wm8990_dapm_lin12_pga_controls[0], ARRAY_SIZE!(wm8990_dapm_lin12_pga_controls)),
SND_SOC_DAPM_MIXER!("LIN34 PGA", WM8990_POWER_MANAGEMENT_2, WM8990_LIN34_ENA_BIT, 0, &wm8990_dapm_lin34_pga_controls[0], ARRAY_SIZE!(wm8990_dapm_lin34_pga_controls)),
SND_SOC_DAPM_MIXER!("RIN12 PGA", WM8990_POWER_MANAGEMENT_2, WM8990_RIN12_ENA_BIT, 0, &wm8990_dapm_rin12_pga_controls[0], ARRAY_SIZE!(wm8990_dapm_rin12_pga_controls)),
SND_SOC_DAPM_MIXER!("RIN34 PGA", WM8990_POWER_MANAGEMENT_2, WM8990_RIN34_ENA_BIT, 0, &wm8990_dapm_rin34_pga_controls[0], ARRAY_SIZE!(wm8990_dapm_rin34_pga_controls)),
SND_SOC_DAPM_MIXER!("INMIXL", SND_SOC_NOPM, 0, 0, &wm8990_dapm_inmixl_controls[0], ARRAY_SIZE!(wm8990_dapm_inmixl_controls)),
SND_SOC_DAPM_MUX!("AINLMUX", SND_SOC_NOPM, 0, 0, &wm8990_dapm_ainlmux_controls),
SND_SOC_DAPM_MIXER!("INMIXR", SND_SOC_NOPM, 0, 0, &wm8990_dapm_inmixr_controls[0], ARRAY_SIZE!(wm8990_dapm_inmixr_controls)),
SND_SOC_DAPM_MUX!("AINRMUX", SND_SOC_NOPM, 0, 0, &wm8990_dapm_ainrmux_controls),
SND_SOC_DAPM_DAC!("Left DAC", "Left Playback", WM8990_POWER_MANAGEMENT_3, WM8990_DACL_ENA_BIT, 0),
SND_SOC_DAPM_DAC!("Right DAC", "Right Playback", WM8990_POWER_MANAGEMENT_3, WM8990_DACR_ENA_BIT, 0),
SND_SOC_DAPM_MIXER_E!("LOMIX", WM8990_POWER_MANAGEMENT_3, WM8990_LOMIX_ENA_BIT, 0, &wm8990_dapm_lomix_controls[0], ARRAY_SIZE!(wm8990_dapm_lomix_controls), outmixer_event, SND_SOC_DAPM_PRE_REG),
SND_SOC_DAPM_MIXER!("LONMIX", WM8990_POWER_MANAGEMENT_3, WM8990_LON_ENA_BIT, 0, &wm8990_dapm_lonmix_controls[0], ARRAY_SIZE!(wm8990_dapm_lonmix_controls)),
SND_SOC_DAPM_MIXER!("LOPMIX", WM8990_POWER_MANAGEMENT_3, WM8990_LOP_ENA_BIT, 0, &wm8990_dapm_lopmix_controls[0], ARRAY_SIZE!(wm8990_dapm_lopmix_controls)),
SND_SOC_DAPM_MIXER!("OUT3MIX", WM8990_POWER_MANAGEMENT_1, WM8990_OUT3_ENA_BIT, 0, &wm8990_dapm_out3mix_controls[0], ARRAY_SIZE!(wm8990_dapm_out3mix_controls)),
SND_SOC_DAPM_MIXER_E!("SPKMIX", WM8990_POWER_MANAGEMENT_1, WM8990_SPK_ENA_BIT, 0, &wm8990_dapm_spkmix_controls[0], ARRAY_SIZE!(wm8990_dapm_spkmix_controls), outmixer_event, SND_SOC_DAPM_PRE_REG),
SND_SOC_DAPM_MIXER!("OUT4MIX", WM8990_POWER_MANAGEMENT_1, WM8990_OUT4_ENA_BIT, 0, &wm8990_dapm_out4mix_controls[0], ARRAY_SIZE!(wm8990_dapm_out4mix_controls)),
SND_SOC_DAPM_MIXER!("ROPMIX", WM8990_POWER_MANAGEMENT_3, WM8990_ROP_ENA_BIT, 0, &wm8990_dapm_ropmix_controls[0], ARRAY_SIZE!(wm8990_dapm_ropmix_controls)),
SND_SOC_DAPM_MIXER!("RONMIX", WM8990_POWER_MANAGEMENT_3, WM8990_RON_ENA_BIT, 0, &wm8990_dapm_ronmix_controls[0], ARRAY_SIZE!(wm8990_dapm_ronmix_controls)),
SND_SOC_DAPM_MIXER_E!("ROMIX", WM8990_POWER_MANAGEMENT_3, WM8990_ROMIX_ENA_BIT, 0, &wm8990_dapm_romix_controls[0], ARRAY_SIZE!(wm8990_dapm_romix_controls), outmixer_event, SND_SOC_DAPM_PRE_REG),
SND_SOC_DAPM_PGA!("LOUT PGA", WM8990_POWER_MANAGEMENT_1, WM8990_LOUT_ENA_BIT, 0, None, 0),
SND_SOC_DAPM_PGA!("ROUT PGA", WM8990_POWER_MANAGEMENT_1, WM8990_ROUT_ENA_BIT, 0, None, 0),
SND_SOC_DAPM_PGA!("LOPGA", WM8990_POWER_MANAGEMENT_3, WM8990_LOPGA_ENA_BIT, 0, None, 0),
SND_SOC_DAPM_PGA!("ROPGA", WM8990_POWER_MANAGEMENT_3, WM8990_ROPGA_ENA_BIT, 0, None, 0),
SND_SOC_DAPM_SUPPLY!("MICBIAS", WM8990_POWER_MANAGEMENT_1, WM8990_MICBIAS_ENA_BIT, 0, None, 0),
SND_SOC_DAPM_OUTPUT!("LON"), SND_SOC_DAPM_OUTPUT!("LOP"), SND_SOC_DAPM_OUTPUT!("OUT3"),
SND_SOC_DAPM_OUTPUT!("LOUT"), SND_SOC_DAPM_OUTPUT!("SPKN"), SND_SOC_DAPM_OUTPUT!("SPKP"),
SND_SOC_DAPM_OUTPUT!("ROUT"), SND_SOC_DAPM_OUTPUT!("OUT4"), SND_SOC_DAPM_OUTPUT!("ROP"),
SND_SOC_DAPM_OUTPUT!("RON"), SND_SOC_DAPM_OUTPUT!("Internal DAC Sink"),
];

static wm8990_dapm_routes: &[snd_soc_dapm_route] = &[
SND_SOC_DAPM_ROUTE!("Internal DAC Sink", None, "Left DAC"), SND_SOC_DAPM_ROUTE!("Internal DAC Sink", None, "Right DAC"),
SND_SOC_DAPM_ROUTE!("Left ADC", None, "Internal ADC Source"), SND_SOC_DAPM_ROUTE!("Right ADC", None, "Internal ADC Source"),
SND_SOC_DAPM_ROUTE!("AINLMUX", None, "INL"), SND_SOC_DAPM_ROUTE!("INMIXL", None, "INL"),
SND_SOC_DAPM_ROUTE!("AINRMUX", None, "INR"), SND_SOC_DAPM_ROUTE!("INMIXR", None, "INR"),
SND_SOC_DAPM_ROUTE!("LIN12 PGA", "LIN1 Switch", "LIN1"), SND_SOC_DAPM_ROUTE!("LIN12 PGA", "LIN2 Switch", "LIN2"),
SND_SOC_DAPM_ROUTE!("LIN34 PGA", "LIN3 Switch", "LIN3"), SND_SOC_DAPM_ROUTE!("LIN34 PGA", "LIN4 Switch", "LIN4/RXN"),
SND_SOC_DAPM_ROUTE!("INMIXL", "Record Left Volume", "LOMIX"), SND_SOC_DAPM_ROUTE!("INMIXL", "LIN2 Volume", "LIN2"),
SND_SOC_DAPM_ROUTE!("INMIXL", "LINPGA12 Switch", "LIN12 PGA"), SND_SOC_DAPM_ROUTE!("INMIXL", "LINPGA34 Switch", "LIN34 PGA"),
SND_SOC_DAPM_ROUTE!("AINLMUX", "INMIXL Mix", "INMIXL"), SND_SOC_DAPM_ROUTE!("AINLMUX", "DIFFINL Mix", "LIN12 PGA"),
SND_SOC_DAPM_ROUTE!("AINLMUX", "DIFFINL Mix", "LIN34 PGA"), SND_SOC_DAPM_ROUTE!("AINLMUX", "RXVOICE Mix", "LIN4/RXN"),
SND_SOC_DAPM_ROUTE!("AINLMUX", "RXVOICE Mix", "RIN4/RXP"), SND_SOC_DAPM_ROUTE!("Left ADC", None, "AINLMUX"),
SND_SOC_DAPM_ROUTE!("RIN12 PGA", "RIN1 Switch", "RIN1"), SND_SOC_DAPM_ROUTE!("RIN12 PGA", "RIN2 Switch", "RIN2"),
SND_SOC_DAPM_ROUTE!("RIN34 PGA", "RIN3 Switch", "RIN3"), SND_SOC_DAPM_ROUTE!("RIN34 PGA", "RIN4 Switch", "RIN4/RXP"),
SND_SOC_DAPM_ROUTE!("INMIXR", "Record Right Volume", "ROMIX"), SND_SOC_DAPM_ROUTE!("INMIXR", "RIN2 Volume", "RIN2"),
SND_SOC_DAPM_ROUTE!("INMIXR", "RINPGA12 Switch", "RIN12 PGA"), SND_SOC_DAPM_ROUTE!("INMIXR", "RINPGA34 Switch", "RIN34 PGA"),
SND_SOC_DAPM_ROUTE!("AINRMUX", "INMIXR Mix", "INMIXR"), SND_SOC_DAPM_ROUTE!("AINRMUX", "DIFFINR Mix", "RIN12 PGA"),
SND_SOC_DAPM_ROUTE!("AINRMUX", "DIFFINR Mix", "RIN34 PGA"), SND_SOC_DAPM_ROUTE!("AINRMUX", "RXVOICE Mix", "LIN4/RXN"),
SND_SOC_DAPM_ROUTE!("AINRMUX", "RXVOICE Mix", "RIN4/RXP"), SND_SOC_DAPM_ROUTE!("Right ADC", None, "AINRMUX"),
SND_SOC_DAPM_ROUTE!("LOMIX", "LOMIX RIN3 Bypass Switch", "RIN3"), SND_SOC_DAPM_ROUTE!("LOMIX", "LOMIX LIN3 Bypass Switch", "LIN3"),
SND_SOC_DAPM_ROUTE!("LOMIX", "LOMIX LIN12 PGA Bypass Switch", "LIN12 PGA"), SND_SOC_DAPM_ROUTE!("LOMIX", "LOMIX RIN12 PGA Bypass Switch", "RIN12 PGA"),
SND_SOC_DAPM_ROUTE!("LOMIX", "LOMIX Right ADC Bypass Switch", "AINRMUX"), SND_SOC_DAPM_ROUTE!("LOMIX", "LOMIX Left ADC Bypass Switch", "AINLMUX"),
SND_SOC_DAPM_ROUTE!("LOMIX", "LOMIX Left DAC Switch", "Left DAC"),
SND_SOC_DAPM_ROUTE!("ROMIX", "ROMIX RIN3 Bypass Switch", "RIN3"), SND_SOC_DAPM_ROUTE!("ROMIX", "ROMIX LIN3 Bypass Switch", "LIN3"),
SND_SOC_DAPM_ROUTE!("ROMIX", "ROMIX LIN12 PGA Bypass Switch", "LIN12 PGA"), SND_SOC_DAPM_ROUTE!("ROMIX", "ROMIX RIN12 PGA Bypass Switch", "RIN12 PGA"),
SND_SOC_DAPM_ROUTE!("ROMIX", "ROMIX Right ADC Bypass Switch", "AINRMUX"), SND_SOC_DAPM_ROUTE!("ROMIX", "ROMIX Left ADC Bypass Switch", "AINLMUX"),
SND_SOC_DAPM_ROUTE!("ROMIX", "ROMIX Right DAC Switch", "Right DAC"),
SND_SOC_DAPM_ROUTE!("SPKMIX", "SPKMIX LIN2 Bypass Switch", "LIN2"), SND_SOC_DAPM_ROUTE!("SPKMIX", "SPKMIX RIN2 Bypass Switch", "RIN2"),
SND_SOC_DAPM_ROUTE!("SPKMIX", "SPKMIX LADC Bypass Switch", "AINLMUX"), SND_SOC_DAPM_ROUTE!("SPKMIX", "SPKMIX RADC Bypass Switch", "AINRMUX"),
SND_SOC_DAPM_ROUTE!("SPKMIX", "SPKMIX Left Mixer PGA Switch", "LOPGA"), SND_SOC_DAPM_ROUTE!("SPKMIX", "SPKMIX Right Mixer PGA Switch", "ROPGA"),
SND_SOC_DAPM_ROUTE!("SPKMIX", "SPKMIX Right DAC Switch", "Right DAC"), SND_SOC_DAPM_ROUTE!("SPKMIX", "SPKMIX Left DAC Switch", "Left DAC"),
SND_SOC_DAPM_ROUTE!("LONMIX", "LONMIX Left Mixer PGA Switch", "LOPGA"), SND_SOC_DAPM_ROUTE!("LONMIX", "LONMIX Right Mixer PGA Switch", "ROPGA"),
SND_SOC_DAPM_ROUTE!("LONMIX", "LONMIX Inverted LOP Switch", "LOPMIX"),
SND_SOC_DAPM_ROUTE!("LOPMIX", "LOPMIX Right Mic Bypass Switch", "RIN12 PGA"), SND_SOC_DAPM_ROUTE!("LOPMIX", "LOPMIX Left Mic Bypass Switch", "LIN12 PGA"),
SND_SOC_DAPM_ROUTE!("LOPMIX", "LOPMIX Left Mixer PGA Switch", "LOPGA"),
SND_SOC_DAPM_ROUTE!("OUT3MIX", "OUT3MIX LIN4/RXP Bypass Switch", "LIN4/RXN"), SND_SOC_DAPM_ROUTE!("OUT3MIX", "OUT3MIX Left Out PGA Switch", "LOPGA"),
SND_SOC_DAPM_ROUTE!("OUT4MIX", "OUT4MIX Right Out PGA Switch", "ROPGA"), SND_SOC_DAPM_ROUTE!("OUT4MIX", "OUT4MIX RIN4/RXP Bypass Switch", "RIN4/RXP"),
SND_SOC_DAPM_ROUTE!("RONMIX", "RONMIX Right Mixer PGA Switch", "ROPGA"), SND_SOC_DAPM_ROUTE!("RONMIX", "RONMIX Left Mixer PGA Switch", "LOPGA"),
SND_SOC_DAPM_ROUTE!("RONMIX", "RONMIX Inverted ROP Switch", "ROPMIX"),
SND_SOC_DAPM_ROUTE!("ROPMIX", "ROPMIX Left Mic Bypass Switch", "LIN12 PGA"), SND_SOC_DAPM_ROUTE!("ROPMIX", "ROPMIX Right Mic Bypass Switch", "RIN12 PGA"),
SND_SOC_DAPM_ROUTE!("ROPMIX", "ROPMIX Right Mixer PGA Switch", "ROPGA"),
SND_SOC_DAPM_ROUTE!("LOPGA", None, "LOMIX"), SND_SOC_DAPM_ROUTE!("ROPGA", None, "ROMIX"),
SND_SOC_DAPM_ROUTE!("LOUT PGA", None, "LOMIX"), SND_SOC_DAPM_ROUTE!("ROUT PGA", None, "ROMIX"),
SND_SOC_DAPM_ROUTE!("LON", None, "LONMIX"), SND_SOC_DAPM_ROUTE!("LOP", None, "LOPMIX"),
SND_SOC_DAPM_ROUTE!("OUT3", None, "OUT3MIX"), SND_SOC_DAPM_ROUTE!("LOUT", None, "LOUT PGA"),
SND_SOC_DAPM_ROUTE!("SPKN", None, "SPKMIX"), SND_SOC_DAPM_ROUTE!("ROUT", None, "ROUT PGA"),
SND_SOC_DAPM_ROUTE!("OUT4", None, "OUT4MIX"), SND_SOC_DAPM_ROUTE!("ROP", None, "ROPMIX"),
SND_SOC_DAPM_ROUTE!("RON", None, "RONMIX"),
];

#[repr(C)]
struct _pll_div {
    div2: u32,
    n: u32,
    k: u32,
}

/* The size in bits of the pll divide multiplied by 10
 * to allow rounding later */
const FIXED_PLL_SIZE: u64 = ((1u64 << 16) * 10);

unsafe fn pll_factors(pll_div: *mut _pll_div, target: c_uint, mut source: c_uint) {
    let mut Kpart: u64;
    let mut K: c_uint;
    let mut Ndiv: c_uint;
    let Nmod: c_uint;

    Ndiv = target / source;
    if Ndiv < 6 {
        source >>= 1;
        (*pll_div).div2 = 1;
        Ndiv = target / source;
    } else {
        (*pll_div).div2 = 0;
    }

    if (Ndiv < 6) || (Ndiv > 12) {
        printk(c"WM8990 N value outwith recommended range! N = %u\n".as_ptr(), Ndiv);
    }

    (*pll_div).n = Ndiv;
    Nmod = target % source;
    Kpart = FIXED_PLL_SIZE.wrapping_mul(Nmod as u64);
    Kpart /= source as u64;
    K = (Kpart & 0xFFFFFFFF) as c_uint;

    /* Check if we need to round */
    if (K % 10) >= 5 {
        K = K.wrapping_add(5);
    }

    /* Move down to proper range now rounding is done */
    K /= 10;
    (*pll_div).k = K;
}

unsafe extern "C" fn wm8990_set_dai_pll(
    codec_dai: *mut snd_soc_dai,
    pll_id: c_int,
    source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let component = (*codec_dai).component;
    let mut pll_div = _pll_div { div2: 0, n: 0, k: 0 };

    if freq_in != 0 && freq_out != 0 {
        pll_factors(&mut pll_div, freq_out.wrapping_mul(4), freq_in);
        snd_soc_component_update_bits(component, WM8990_POWER_MANAGEMENT_2, WM8990_PLL_ENA, WM8990_PLL_ENA);
        snd_soc_component_update_bits(component, WM8990_CLOCKING_2, WM8990_SYSCLK_SRC, WM8990_SYSCLK_SRC);
        snd_soc_component_write(component, WM8990_PLL1, pll_div.n | WM8990_SDM | if pll_div.div2 != 0 { WM8990_PRESCALE } else { 0 });
        snd_soc_component_write(component, WM8990_PLL2, (pll_div.k >> 8) as u8 as c_uint);
        snd_soc_component_write(component, WM8990_PLL3, (pll_div.k & 0xFF) as u8 as c_uint);
    } else {
        snd_soc_component_update_bits(component, WM8990_POWER_MANAGEMENT_2, WM8990_PLL_ENA, 0);
    }
    0
}

unsafe extern "C" fn wm8990_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let wm8990 = snd_soc_component_get_drvdata(component) as *mut wm8990_priv;
    (*wm8990).sysclk = freq;
    0
}

unsafe extern "C" fn wm8990_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut audio1: u16 = snd_soc_component_read(component, WM8990_AUDIO_INTERFACE_1) as u16;
    let mut audio3: u16 = snd_soc_component_read(component, WM8990_AUDIO_INTERFACE_3) as u16;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => audio3 &= !(WM8990_AIF_MSTR1 as u16),
        SND_SOC_DAIFMT_CBP_CFP => audio3 |= WM8990_AIF_MSTR1 as u16,
        _ => return -EINVAL,
    }

    audio1 &= !(WM8990_AIF_FMT_MASK as u16);
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => { audio1 |= WM8990_AIF_TMF_I2S as u16; audio1 &= !(WM8990_AIF_LRCLK_INV as u16); }
        SND_SOC_DAIFMT_RIGHT_J => { audio1 |= WM8990_AIF_TMF_RIGHTJ as u16; audio1 &= !(WM8990_AIF_LRCLK_INV as u16); }
        SND_SOC_DAIFMT_LEFT_J => { audio1 |= WM8990_AIF_TMF_LEFTJ as u16; audio1 &= !(WM8990_AIF_LRCLK_INV as u16); }
        SND_SOC_DAIFMT_DSP_A => { audio1 |= WM8990_AIF_TMF_DSP as u16; audio1 &= !(WM8990_AIF_LRCLK_INV as u16); }
        SND_SOC_DAIFMT_DSP_B => audio1 |= (WM8990_AIF_TMF_DSP | WM8990_AIF_LRCLK_INV) as u16,
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, WM8990_AUDIO_INTERFACE_1, audio1 as c_uint);
    snd_soc_component_write(component, WM8990_AUDIO_INTERFACE_3, audio3 as c_uint);
    0
}

unsafe extern "C" fn wm8990_set_dai_clkdiv(codec_dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int {
    let component = (*codec_dai).component;
    match div_id {
        WM8990_MCLK_DIV => { snd_soc_component_update_bits(component, WM8990_CLOCKING_2, WM8990_MCLK_DIV_MASK, div as c_uint); }
        WM8990_DACCLK_DIV => { snd_soc_component_update_bits(component, WM8990_CLOCKING_2, WM8990_DAC_CLKDIV_MASK, div as c_uint); }
        WM8990_ADCCLK_DIV => { snd_soc_component_update_bits(component, WM8990_CLOCKING_2, WM8990_ADC_CLKDIV_MASK, div as c_uint); }
        WM8990_BCLK_DIV => { snd_soc_component_update_bits(component, WM8990_CLOCKING_1, WM8990_BCLK_DIV_MASK, div as c_uint); }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn wm8990_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let mut audio1: u16 = snd_soc_component_read(component, WM8990_AUDIO_INTERFACE_1) as u16;
    audio1 &= !(WM8990_AIF_WL_MASK as u16);
    match params_width(params) {
        16 => {}
        20 => audio1 |= WM8990_AIF_WL_20BITS as u16,
        24 => audio1 |= WM8990_AIF_WL_24BITS as u16,
        32 => audio1 |= WM8990_AIF_WL_32BITS as u16,
        _ => {}
    }
    snd_soc_component_write(component, WM8990_AUDIO_INTERFACE_1, audio1 as c_uint);
    0
}

unsafe extern "C" fn wm8990_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    let component = (*dai).component;
    let val: u16 = (snd_soc_component_read(component, WM8990_DAC_CTRL) & !WM8990_DAC_MUTE) as u16;
    if mute != 0 {
        snd_soc_component_write(component, WM8990_DAC_CTRL, (val as c_uint) | WM8990_DAC_MUTE);
    } else {
        snd_soc_component_write(component, WM8990_DAC_CTRL, val as c_uint);
    }
    0
}

unsafe extern "C" fn wm8990_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let wm8990 = snd_soc_component_get_drvdata(component) as *mut wm8990_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            snd_soc_component_update_bits(component, WM8990_POWER_MANAGEMENT_1, WM8990_VMID_MODE_MASK, 0x2);
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) as c_int == snd_soc_bias_level::SND_SOC_BIAS_OFF as c_int {
                ret = regcache_sync((*wm8990).regmap);
                if ret < 0 {
                    dev_err((*component).dev, c"Failed to sync cache: %d\n".as_ptr(), ret);
                    return ret;
                }
                snd_soc_component_write(component, WM8990_ANTIPOP1, WM8990_DIS_LLINE | WM8990_DIS_RLINE | WM8990_DIS_OUT3 | WM8990_DIS_OUT4 | WM8990_DIS_LOUT | WM8990_DIS_ROUT);
                snd_soc_component_write(component, WM8990_ANTIPOP2, WM8990_SOFTST | WM8990_BUFDCOPEN | WM8990_POBCTRL | WM8990_VMIDTOG);
                msleep(300);
                snd_soc_component_write(component, WM8990_ANTIPOP2, WM8990_SOFTST | WM8990_BUFDCOPEN | WM8990_POBCTRL);
                snd_soc_component_write(component, WM8990_ANTIPOP1, 0);
                snd_soc_component_write(component, WM8990_POWER_MANAGEMENT_1, 0x1b00);
                msleep(50);
                snd_soc_component_write(component, WM8990_POWER_MANAGEMENT_1, 0x1f02);
                msleep(100);
                snd_soc_component_write(component, WM8990_POWER_MANAGEMENT_1, 0x1f03);
                msleep(600);
                snd_soc_component_write(component, WM8990_ANTIPOP2, WM8990_SOFTST | WM8990_BUFDCOPEN | WM8990_POBCTRL | WM8990_BUFIOEN);
                snd_soc_component_write(component, WM8990_POWER_MANAGEMENT_1, 0x3);
                snd_soc_component_write(component, WM8990_ANTIPOP2, WM8990_BUFIOEN);
                snd_soc_component_write(component, WM8990_EXT_ACCESS_ENA, 0x2);
                snd_soc_component_write(component, WM8990_EXT_CTL1, 0xa003);
                snd_soc_component_write(component, WM8990_EXT_ACCESS_ENA, 0);
            }
            snd_soc_component_update_bits(component, WM8990_POWER_MANAGEMENT_1, WM8990_VMID_MODE_MASK, 0x4);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            snd_soc_component_write(component, WM8990_ANTIPOP2, WM8990_SOFTST | WM8990_POBCTRL | WM8990_BUFIOEN);
            snd_soc_component_write(component, WM8990_ANTIPOP2, WM8990_SOFTST | WM8990_BUFDCOPEN | WM8990_POBCTRL | WM8990_BUFIOEN);
            snd_soc_component_update_bits(component, WM8990_DAC_CTRL, WM8990_DAC_MUTE, WM8990_DAC_MUTE);
            snd_soc_component_write(component, WM8990_POWER_MANAGEMENT_1, 0x1f03);
            snd_soc_component_write(component, WM8990_POWER_MANAGEMENT_1, 0x1f01);
            msleep(300);
            snd_soc_component_write(component, WM8990_ANTIPOP1, WM8990_DIS_LLINE | WM8990_DIS_RLINE | WM8990_DIS_OUT3 | WM8990_DIS_OUT4 | WM8990_DIS_LOUT | WM8990_DIS_ROUT);
            snd_soc_component_write(component, WM8990_POWER_MANAGEMENT_1, 0x0);
            snd_soc_component_write(component, WM8990_ANTIPOP2, 0x0);
            regcache_mark_dirty((*wm8990).regmap);
        }
    }
    0
}

const WM8990_RATES: c_uint = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_11025 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_22050 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
const WM8990_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static wm8990_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8990_hw_params),
    mute_stream: Some(wm8990_mute),
    set_fmt: Some(wm8990_set_dai_fmt),
    set_clkdiv: Some(wm8990_set_dai_clkdiv),
    set_pll: Some(wm8990_set_dai_pll),
    set_sysclk: Some(wm8990_set_dai_sysclk),
    no_capture_mute: 1,
};

static mut wm8990_dai: snd_soc_dai_driver = SND_SOC_DAI_DRIVER! {
    name: "wm8990-hifi",
    playback: { stream_name: "Playback", channels_min: 1, channels_max: 2, rates: WM8990_RATES, formats: WM8990_FORMATS },
    capture: { stream_name: "Capture", channels_min: 1, channels_max: 2, rates: WM8990_RATES, formats: WM8990_FORMATS },
    ops: &wm8990_dai_ops,
};

unsafe extern "C" fn wm8990_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    wm8990_reset!(component);
    snd_soc_dapm_force_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_STANDBY);
    snd_soc_component_update_bits(component, WM8990_AUDIO_INTERFACE_4, WM8990_ALRCGPIO1, WM8990_ALRCGPIO1);
    snd_soc_component_update_bits(component, WM8990_GPIO1_GPIO2, WM8990_GPIO1_SEL_MASK, 1);
    snd_soc_component_update_bits(component, WM8990_POWER_MANAGEMENT_2, WM8990_OPCLK_ENA, WM8990_OPCLK_ENA);
    snd_soc_component_write(component, WM8990_LEFT_OUTPUT_VOLUME, 0x50 | (1 << 8));
    snd_soc_component_write(component, WM8990_RIGHT_OUTPUT_VOLUME, 0x50 | (1 << 8));
    0
}

static soc_component_dev_wm8990: snd_soc_component_driver = SND_SOC_COMPONENT_DRIVER! {
    probe: wm8990_probe,
    set_bias_level: wm8990_set_bias_level,
    controls: wm8990_snd_controls,
    num_controls: ARRAY_SIZE!(wm8990_snd_controls),
    dapm_widgets: wm8990_dapm_widgets,
    num_dapm_widgets: ARRAY_SIZE!(wm8990_dapm_widgets),
    dapm_routes: wm8990_dapm_routes,
    num_dapm_routes: ARRAY_SIZE!(wm8990_dapm_routes),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn wm8990_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8990: *mut wm8990_priv;
    let ret: c_int;
    wm8990 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8990_priv>(), GFP_KERNEL) as *mut wm8990_priv;
    if wm8990.is_null() {
        return -ENOMEM;
    }
    i2c_set_clientdata(i2c, wm8990 as *mut c_void);
    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_wm8990, &raw mut wm8990_dai, 1);
    ret
}

static wm8990_i2c_id: &[i2c_device_id] = &[
    i2c_device_id { name: *b"wm8990\0", driver_data: 0 },
    i2c_device_id::zeroed(),
];
MODULE_DEVICE_TABLE!(i2c, wm8990_i2c_id);

static mut wm8990_i2c_driver: i2c_driver = I2C_DRIVER! {
    driver: { name: "wm8990" },
    probe: wm8990_i2c_probe,
    id_table: wm8990_i2c_id,
};

module_i2c_driver!(wm8990_i2c_driver);
MODULE_DESCRIPTION!("ASoC WM8990 driver");
MODULE_AUTHOR!("Liam Girdwood");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
