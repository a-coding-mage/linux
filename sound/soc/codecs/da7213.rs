// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DA7213 ALSA SoC Codec Driver
 *
 * Copyright (c) 2013 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 * Based on DA9055 ALSA SoC codec driver.
 */

/* Translated from soc/codecs/da7213.c. Kernel, ASoC, regmap, PM, OF and ACPI
 * declarations/macros are supplied by external bindings.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, improper_ctypes, overflowing_literals)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u32 = u32;
type u64 = u64;
type bool_t = bool;
type __le16 = u16;

extern "C" {
    static DA7213_DAC_FILTERS1: c_uint;
    static DA7213_VOICE_HPF_CORNER_SHIFT: c_uint;
    static DA7213_ADC_FILTERS1: c_uint;
    static DA7213_AUDIO_HPF_CORNER_SHIFT: c_uint;
    static DA7213_TONE_GEN_CFG1: c_uint;
    static DA7213_DTMF_REG_SHIFT: c_uint;
    static DA7213_DTMF_REG_MAX: c_uint;
    static DA7213_TONE_GEN_CFG2: c_uint;
    static DA7213_SWG_SEL_SHIFT: c_uint;
    static DA7213_SWG_SEL_MAX: c_uint;
    static DA7213_GAIN_RAMP_CTRL: c_uint;
    static DA7213_GAIN_RAMP_RATE_SHIFT: c_uint;
    static DA7213_DAC_NG_SETUP_TIME: c_uint;
    static DA7213_DAC_NG_SETUP_TIME_SHIFT: c_uint;
    static DA7213_DAC_NG_RAMPUP_RATE_SHIFT: c_uint;
    static DA7213_DAC_NG_RAMPDN_RATE_SHIFT: c_uint;
    static DA7213_DAC_FILTERS5: c_uint;
    static DA7213_DAC_SOFTMUTE_RATE_SHIFT: c_uint;
    static DA7213_ALC_CTRL2: c_uint;
    static DA7213_ALC_ATTACK_SHIFT: c_uint;
    static DA7213_ALC_RELEASE_SHIFT: c_uint;
    static DA7213_ALC_CTRL3: c_uint;
    static DA7213_ALC_HOLD_SHIFT: c_uint;
    static DA7213_ALC_INTEG_ATTACK_SHIFT: c_uint;
    static DA7213_ALC_INTEG_RELEASE_SHIFT: c_uint;
}

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut da7213_priv;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_volsw_2r(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_put_volsw_2r(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_raw_read(map: *mut regmap, reg: c_uint, val: *mut c_void, val_len: usize) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: c_uint, val: *const c_void, val_len: usize) -> c_int;
    fn msleep(msecs: c_uint);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn clk_round_rate(clk: *mut clk, rate: c_uint) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_uint;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut u32) -> c_int;
    fn device_property_read_string(dev: *mut device, propname: *const c_char, val: *mut *const c_char) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_get_platdata(dev: *mut device) -> *mut da7213_platform_data;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut da7213_priv;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regulator_bulk_disable(num: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_regulator_bulk_get(dev: *mut device, num: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_add_action_or_reset(dev: *mut device, action: Option<unsafe extern "C" fn(*mut c_void)>, data: *mut c_void) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
}

#[repr(C)] struct snd_kcontrol { private_value: usize }
#[repr(C)] struct snd_soc_component { dev: *mut device }
#[repr(C)] struct da7213_priv {
    ctrl_lock: mutex,
    regmap: *mut regmap,
    alc_calib_auto: bool,
    alc_en: bool,
    master: bool,
    fmt: u8,
    clk_src: c_int,
    mclk_rate: c_uint,
    fin_min_rate: usize,
    mclk: *mut clk,
    out_rate: c_uint,
    fixed_clk_auto_pll: bool,
    pdata: *mut da7213_platform_data,
    dev: *mut device,
    supplies: [regulator_bulk_data; DA7213_NUM_SUPPLIES as usize],
}
#[repr(C)] struct snd_ctl_elem_value { value: snd_ctl_elem_value_union }
#[repr(C)] union snd_ctl_elem_value_union { integer: snd_ctl_elem_value_integer }
#[repr(C)] #[derive(Copy, Clone)] struct snd_ctl_elem_value_integer { value: [i64; 128] }
#[repr(C)] struct soc_mixer_control { reg: c_uint }
#[repr(C)] struct snd_soc_dapm_widget { dapm: *mut snd_soc_dapm_context }
#[repr(C)] struct snd_soc_dapm_context;
#[repr(C)] struct snd_pcm_substream;
#[repr(C)] struct snd_pcm_hw_params;
#[repr(C)] struct snd_soc_dai { component: *mut snd_soc_component }
#[repr(C)] struct device;
#[repr(C)] struct regmap;
#[repr(C)] struct clk;
#[repr(C)] struct mutex;
#[repr(C)] struct regulator_bulk_data { supply: *const c_char }
#[repr(C)] struct i2c_client { dev: device }
#[repr(C)] struct snd_kcontrol_new;
#[repr(C)] struct snd_soc_dapm_widget_decl;
#[repr(C)] struct snd_soc_dapm_route { sink: *const c_char, control: *const c_char, source: *const c_char }
#[repr(C)] struct reg_default { reg: c_uint, def: c_uint }
#[repr(C)] struct soc_enum;
#[repr(C)] struct of_device_id { compatible: *const c_char, data: *const c_void }
#[repr(C)] struct acpi_device_id { id: [c_char; 16], driver_data: usize }
#[repr(C)] struct i2c_device_id { name: [c_char; 20], driver_data: usize }
#[repr(C)] struct dev_pm_ops;
#[repr(C)] struct regmap_config;
#[repr(C)] struct snd_soc_component_driver;
#[repr(C)] struct snd_soc_dai_ops;
#[repr(C)] struct snd_soc_dai_driver;
type snd_soc_bias_level = c_uint;

#[repr(C)] struct da7213_platform_data {
    micbias1_lvl: da7213_micbias_voltage,
    micbias2_lvl: da7213_micbias_voltage,
    dmic_data_sel: da7213_dmic_data_sel,
    dmic_samplephase: da7213_dmic_samplephase,
    dmic_clk_rate: da7213_dmic_clk_rate,
}
type da7213_micbias_voltage = c_uint;
type da7213_dmic_data_sel = c_uint;
type da7213_dmic_samplephase = c_uint;
type da7213_dmic_clk_rate = c_uint;

extern "Rust" {
    static DA7213_ALC_AVG_ITERATIONS: u8;
    static DA7213_ALC_CIC_OP_LVL_CTRL: c_uint;
    static DA7213_ALC_DATA_MIDDLE: u8;
    static DA7213_ALC_CIC_OP_LVL_DATA: c_uint;
    static DA7213_ALC_DATA_TOP: u8;
    static DA7213_ALC_CIC_OP_CHANNEL_LEFT: u8;
    static DA7213_ALC_CIC_OP_CHANNEL_RIGHT: u8;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const MEGA: c_uint = 1_000_000;
const DA7213_FIN_MIN_RATE: c_uint = 5 * MEGA;
const DA7212_FIN_MIN_RATE: c_uint = 2 * MEGA;
const DA7213_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

/* Gain and Volume */
static aux_vol_tlv: &[c_uint] = DECLARE_TLV_DB_RANGE!(
    0x0, 0x11, TLV_DB_SCALE_ITEM!(-5400, 0, 0),
    0x12, 0x3f, TLV_DB_SCALE_ITEM!(-5250, 150, 0)
);
static digital_gain_tlv: &[c_uint] = DECLARE_TLV_DB_RANGE!(
    0x0, 0x07, TLV_DB_SCALE_ITEM!(TLV_DB_GAIN_MUTE, 0, 1),
    0x08, 0x7f, TLV_DB_SCALE_ITEM!(-7800, 75, 0)
);
static alc_analog_gain_tlv: &[c_uint] = DECLARE_TLV_DB_RANGE!(
    0x0, 0x0, TLV_DB_SCALE_ITEM!(TLV_DB_GAIN_MUTE, 0, 1),
    0x01, 0x07, TLV_DB_SCALE_ITEM!(0, 600, 0)
);
static mic_vol_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(mic_vol_tlv, -600, 600, 0);
static mixin_gain_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(mixin_gain_tlv, -450, 150, 0);
static eq_gain_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(eq_gain_tlv, -1050, 150, 0);
static hp_vol_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(hp_vol_tlv, -5700, 100, 0);
static lineout_vol_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(lineout_vol_tlv, -4800, 100, 0);
static alc_threshold_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(alc_threshold_tlv, -9450, 150, 0);
static alc_gain_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(alc_gain_tlv, 0, 600, 0);
static da7213_tonegen_gain_tlv: &[c_uint] = DECLARE_TLV_DB_SCALE!(da7213_tonegen_gain_tlv, -4500, 300, 0);

static da7213_voice_hpf_corner_txt: [*const c_char; 8] = [
    c"2.5Hz".as_ptr(), c"25Hz".as_ptr(), c"50Hz".as_ptr(), c"100Hz".as_ptr(),
    c"150Hz".as_ptr(), c"200Hz".as_ptr(), c"300Hz".as_ptr(), c"400Hz".as_ptr(),
];
static da7213_dac_voice_hpf_corner: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_DAC_FILTERS1, DA7213_VOICE_HPF_CORNER_SHIFT, da7213_voice_hpf_corner_txt);
static da7213_adc_voice_hpf_corner: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_ADC_FILTERS1, DA7213_VOICE_HPF_CORNER_SHIFT, da7213_voice_hpf_corner_txt);

static da7213_audio_hpf_corner_txt: [*const c_char; 4] = [
    c"Fs/24000".as_ptr(), c"Fs/12000".as_ptr(), c"Fs/6000".as_ptr(), c"Fs/3000".as_ptr(),
];
static da7213_dac_audio_hpf_corner: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_DAC_FILTERS1, DA7213_AUDIO_HPF_CORNER_SHIFT, da7213_audio_hpf_corner_txt);
static da7213_adc_audio_hpf_corner: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_ADC_FILTERS1, DA7213_AUDIO_HPF_CORNER_SHIFT, da7213_audio_hpf_corner_txt);

static da7213_tonegen_dtmf_key_txt: [*const c_char; 16] = [
    c"0".as_ptr(), c"1".as_ptr(), c"2".as_ptr(), c"3".as_ptr(), c"4".as_ptr(), c"5".as_ptr(),
    c"6".as_ptr(), c"7".as_ptr(), c"8".as_ptr(), c"9".as_ptr(), c"A".as_ptr(), c"B".as_ptr(),
    c"C".as_ptr(), c"D".as_ptr(), c"*".as_ptr(), c"#".as_ptr(),
];
static da7213_tonegen_dtmf_key: soc_enum =
    SOC_ENUM_SINGLE!(DA7213_TONE_GEN_CFG1, DA7213_DTMF_REG_SHIFT, DA7213_DTMF_REG_MAX, da7213_tonegen_dtmf_key_txt);

static da7213_tonegen_swg_sel_txt: [*const c_char; 4] = [
    c"Sum".as_ptr(), c"SWG1".as_ptr(), c"SWG2".as_ptr(), c"Sum".as_ptr(),
];
static da7213_tonegen_swg_sel: soc_enum =
    SOC_ENUM_SINGLE!(DA7213_TONE_GEN_CFG2, DA7213_SWG_SEL_SHIFT, DA7213_SWG_SEL_MAX, da7213_tonegen_swg_sel_txt);

static da7213_gain_ramp_rate_txt: [*const c_char; 4] = [
    c"nominal rate * 8".as_ptr(), c"nominal rate * 16".as_ptr(),
    c"nominal rate / 16".as_ptr(), c"nominal rate / 32".as_ptr(),
];
static da7213_gain_ramp_rate: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_GAIN_RAMP_CTRL, DA7213_GAIN_RAMP_RATE_SHIFT, da7213_gain_ramp_rate_txt);

static da7213_dac_ng_setup_time_txt: [*const c_char; 4] = [
    c"256 samples".as_ptr(), c"512 samples".as_ptr(), c"1024 samples".as_ptr(), c"2048 samples".as_ptr(),
];
static da7213_dac_ng_setup_time: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_DAC_NG_SETUP_TIME, DA7213_DAC_NG_SETUP_TIME_SHIFT, da7213_dac_ng_setup_time_txt);
static da7213_dac_ng_rampup_txt: [*const c_char; 2] = [c"0.02 ms/dB".as_ptr(), c"0.16 ms/dB".as_ptr()];
static da7213_dac_ng_rampup_rate: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_DAC_NG_SETUP_TIME, DA7213_DAC_NG_RAMPUP_RATE_SHIFT, da7213_dac_ng_rampup_txt);
static da7213_dac_ng_rampdown_txt: [*const c_char; 2] = [c"0.64 ms/dB".as_ptr(), c"20.48 ms/dB".as_ptr()];
static da7213_dac_ng_rampdown_rate: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_DAC_NG_SETUP_TIME, DA7213_DAC_NG_RAMPDN_RATE_SHIFT, da7213_dac_ng_rampdown_txt);
static da7213_dac_soft_mute_rate_txt: [*const c_char; 7] = [
    c"1".as_ptr(), c"2".as_ptr(), c"4".as_ptr(), c"8".as_ptr(), c"16".as_ptr(), c"32".as_ptr(), c"64".as_ptr(),
];
static da7213_dac_soft_mute_rate: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_DAC_FILTERS5, DA7213_DAC_SOFTMUTE_RATE_SHIFT, da7213_dac_soft_mute_rate_txt);
static da7213_alc_attack_rate_txt: [*const c_char; 13] = [
    c"44/fs".as_ptr(), c"88/fs".as_ptr(), c"176/fs".as_ptr(), c"352/fs".as_ptr(), c"704/fs".as_ptr(),
    c"1408/fs".as_ptr(), c"2816/fs".as_ptr(), c"5632/fs".as_ptr(), c"11264/fs".as_ptr(),
    c"22528/fs".as_ptr(), c"45056/fs".as_ptr(), c"90112/fs".as_ptr(), c"180224/fs".as_ptr(),
];
static da7213_alc_attack_rate: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_ALC_CTRL2, DA7213_ALC_ATTACK_SHIFT, da7213_alc_attack_rate_txt);
static da7213_alc_release_rate_txt: [*const c_char; 11] = [
    c"176/fs".as_ptr(), c"352/fs".as_ptr(), c"704/fs".as_ptr(), c"1408/fs".as_ptr(), c"2816/fs".as_ptr(),
    c"5632/fs".as_ptr(), c"11264/fs".as_ptr(), c"22528/fs".as_ptr(), c"45056/fs".as_ptr(),
    c"90112/fs".as_ptr(), c"180224/fs".as_ptr(),
];
static da7213_alc_release_rate: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_ALC_CTRL2, DA7213_ALC_RELEASE_SHIFT, da7213_alc_release_rate_txt);
static da7213_alc_hold_time_txt: [*const c_char; 16] = [
    c"62/fs".as_ptr(), c"124/fs".as_ptr(), c"248/fs".as_ptr(), c"496/fs".as_ptr(), c"992/fs".as_ptr(),
    c"1984/fs".as_ptr(), c"3968/fs".as_ptr(), c"7936/fs".as_ptr(), c"15872/fs".as_ptr(),
    c"31744/fs".as_ptr(), c"63488/fs".as_ptr(), c"126976/fs".as_ptr(), c"253952/fs".as_ptr(),
    c"507904/fs".as_ptr(), c"1015808/fs".as_ptr(), c"2031616/fs".as_ptr(),
];
static da7213_alc_hold_time: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_ALC_CTRL3, DA7213_ALC_HOLD_SHIFT, da7213_alc_hold_time_txt);
static da7213_alc_integ_rate_txt: [*const c_char; 4] = [
    c"1/4".as_ptr(), c"1/16".as_ptr(), c"1/256".as_ptr(), c"1/65536".as_ptr(),
];
static da7213_alc_integ_attack_rate: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_ALC_CTRL3, DA7213_ALC_INTEG_ATTACK_SHIFT, da7213_alc_integ_rate_txt);
static da7213_alc_integ_release_rate: soc_enum =
    SOC_ENUM_SINGLE_DECL!(DA7213_ALC_CTRL3, DA7213_ALC_INTEG_RELEASE_SHIFT, da7213_alc_integ_rate_txt);

unsafe extern "C" fn da7213_volsw_locked_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7213 = snd_soc_component_get_drvdata(component);
    mutex_lock(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    let ret = snd_soc_get_volsw(kcontrol, ucontrol);
    mutex_unlock(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    ret
}

unsafe extern "C" fn da7213_volsw_locked_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7213 = snd_soc_component_get_drvdata(component);
    mutex_lock(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    let ret = snd_soc_put_volsw(kcontrol, ucontrol);
    mutex_unlock(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    ret
}

unsafe extern "C" fn da7213_enum_locked_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7213 = snd_soc_component_get_drvdata(component);
    mutex_lock(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    let ret = snd_soc_get_enum_double(kcontrol, ucontrol);
    mutex_unlock(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    ret
}

unsafe extern "C" fn da7213_enum_locked_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7213 = snd_soc_component_get_drvdata(component);
    mutex_lock(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    let ret = snd_soc_put_enum_double(kcontrol, ucontrol);
    mutex_unlock(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    ret
}

unsafe extern "C" fn da7213_get_alc_data(component: *mut snd_soc_component, reg_val: u8) -> c_int {
    let mut sum: c_int = 0;
    let mut iteration: u8 = 0;
    while iteration < DA7213_ALC_AVG_ITERATIONS {
        snd_soc_component_write(component, DA7213_ALC_CIC_OP_LVL_CTRL, reg_val as c_uint);
        snd_soc_component_write(component, DA7213_ALC_CIC_OP_LVL_CTRL, (reg_val | DA7213_ALC_DATA_MIDDLE) as c_uint);
        let mid_data = snd_soc_component_read(component, DA7213_ALC_CIC_OP_LVL_DATA) as c_int;
        snd_soc_component_write(component, DA7213_ALC_CIC_OP_LVL_CTRL, (reg_val | DA7213_ALC_DATA_TOP) as c_uint);
        let top_data = snd_soc_component_read(component, DA7213_ALC_CIC_OP_LVL_DATA) as c_int;
        sum += (mid_data << 8) | (top_data << 16);
        iteration = iteration.wrapping_add(1);
    }
    sum / DA7213_ALC_AVG_ITERATIONS as c_int
}

unsafe extern "C" fn da7213_alc_calib_man(component: *mut snd_soc_component) {
    let avg_left_data = da7213_get_alc_data(component, DA7213_ALC_CIC_OP_CHANNEL_LEFT);
    let avg_right_data = da7213_get_alc_data(component, DA7213_ALC_CIC_OP_CHANNEL_RIGHT);
    let offset_l = -avg_left_data;
    let offset_r = -avg_right_data;
    let mut reg_val: u8 = ((offset_l & DA7213_ALC_OFFSET_15_8 as c_int) >> 8) as u8;
    snd_soc_component_write(component, DA7213_ALC_OFFSET_MAN_M_L, reg_val as c_uint);
    reg_val = ((offset_l & DA7213_ALC_OFFSET_19_16 as c_int) >> 16) as u8;
    snd_soc_component_write(component, DA7213_ALC_OFFSET_MAN_U_L, reg_val as c_uint);
    reg_val = ((offset_r & DA7213_ALC_OFFSET_15_8 as c_int) >> 8) as u8;
    snd_soc_component_write(component, DA7213_ALC_OFFSET_MAN_M_R, reg_val as c_uint);
    reg_val = ((offset_r & DA7213_ALC_OFFSET_19_16 as c_int) >> 16) as u8;
    snd_soc_component_write(component, DA7213_ALC_OFFSET_MAN_U_R, reg_val as c_uint);
    snd_soc_component_update_bits(component, DA7213_ALC_CTRL1,
        DA7213_ALC_OFFSET_EN | DA7213_ALC_SYNC_MODE,
        DA7213_ALC_OFFSET_EN | DA7213_ALC_SYNC_MODE);
}

unsafe extern "C" fn da7213_alc_calib_auto(component: *mut snd_soc_component) {
    snd_soc_component_update_bits(component, DA7213_ALC_CTRL1, DA7213_ALC_AUTO_CALIB_EN, DA7213_ALC_AUTO_CALIB_EN);
    let mut alc_ctrl1: u8;
    loop {
        alc_ctrl1 = snd_soc_component_read(component, DA7213_ALC_CTRL1) as u8;
        if (alc_ctrl1 as c_uint & DA7213_ALC_AUTO_CALIB_EN) == 0 { break; }
    }
    if (alc_ctrl1 as c_uint & DA7213_ALC_CALIB_OVERFLOW) != 0 {
        dev_warn((*component).dev, c"ALC auto calibration failed with overflow\n".as_ptr());
        snd_soc_component_update_bits(component, DA7213_ALC_CTRL1,
            DA7213_ALC_OFFSET_EN | DA7213_ALC_SYNC_MODE, 0);
    } else {
        snd_soc_component_update_bits(component, DA7213_ALC_CTRL1,
            DA7213_ALC_OFFSET_EN | DA7213_ALC_SYNC_MODE,
            DA7213_ALC_OFFSET_EN | DA7213_ALC_SYNC_MODE);
    }
}

unsafe extern "C" fn da7213_alc_calib(component: *mut snd_soc_component) {
    let da7213 = snd_soc_component_get_drvdata(component);
    let adc_l_ctrl = snd_soc_component_read(component, DA7213_ADC_L_CTRL) as u8;
    let adc_r_ctrl = snd_soc_component_read(component, DA7213_ADC_R_CTRL) as u8;
    let mixin_l_sel = snd_soc_component_read(component, DA7213_MIXIN_L_SELECT) as u8;
    let mixin_r_sel = snd_soc_component_read(component, DA7213_MIXIN_R_SELECT) as u8;
    let mic_1_ctrl = snd_soc_component_read(component, DA7213_MIC_1_CTRL) as u8;
    let mic_2_ctrl = snd_soc_component_read(component, DA7213_MIC_2_CTRL) as u8;

    snd_soc_component_update_bits(component, DA7213_ADC_L_CTRL, DA7213_ADC_EN, DA7213_ADC_EN);
    snd_soc_component_update_bits(component, DA7213_ADC_R_CTRL, DA7213_ADC_EN, DA7213_ADC_EN);
    snd_soc_component_update_bits(component, DA7213_MIXIN_L_SELECT,
        DA7213_MIXIN_L_MIX_SELECT_MIC_1 | DA7213_MIXIN_L_MIX_SELECT_MIC_2,
        DA7213_MIXIN_L_MIX_SELECT_MIC_1 | DA7213_MIXIN_L_MIX_SELECT_MIC_2);
    snd_soc_component_update_bits(component, DA7213_MIXIN_R_SELECT,
        DA7213_MIXIN_R_MIX_SELECT_MIC_2 | DA7213_MIXIN_R_MIX_SELECT_MIC_1,
        DA7213_MIXIN_R_MIX_SELECT_MIC_2 | DA7213_MIXIN_R_MIX_SELECT_MIC_1);
    snd_soc_component_update_bits(component, DA7213_MIC_1_CTRL, DA7213_MUTE_EN, DA7213_MUTE_EN);
    snd_soc_component_update_bits(component, DA7213_MIC_2_CTRL, DA7213_MUTE_EN, DA7213_MUTE_EN);
    if (*da7213).alc_calib_auto { da7213_alc_calib_auto(component); } else { da7213_alc_calib_man(component); }
    snd_soc_component_write(component, DA7213_MIXIN_L_SELECT, mixin_l_sel as c_uint);
    snd_soc_component_write(component, DA7213_MIXIN_R_SELECT, mixin_r_sel as c_uint);
    snd_soc_component_write(component, DA7213_ADC_L_CTRL, adc_l_ctrl as c_uint);
    snd_soc_component_write(component, DA7213_ADC_R_CTRL, adc_r_ctrl as c_uint);
    snd_soc_component_write(component, DA7213_MIC_1_CTRL, mic_1_ctrl as c_uint);
    snd_soc_component_write(component, DA7213_MIC_2_CTRL, mic_2_ctrl as c_uint);
}

unsafe extern "C" fn da7213_put_mixin_gain(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7213 = snd_soc_component_get_drvdata(component);
    let ret = snd_soc_put_volsw_2r(kcontrol, ucontrol);
    if ret == 0 && (*da7213).alc_en { da7213_alc_calib(component); }
    ret
}

unsafe extern "C" fn da7213_put_alc_sw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7213 = snd_soc_component_get_drvdata(component);
    if (*ucontrol).value.integer.value[0] != 0 || (*ucontrol).value.integer.value[1] != 0 {
        if !(*da7213).alc_en {
            da7213_alc_calib(component);
            (*da7213).alc_en = true;
        }
    } else {
        (*da7213).alc_en = false;
    }
    snd_soc_put_volsw(kcontrol, ucontrol)
}

unsafe extern "C" fn da7213_tonegen_freq_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7213 = snd_soc_component_get_drvdata(component);
    let mixer_ctrl = (*kcontrol).private_value as *mut soc_mixer_control;
    let reg = (*mixer_ctrl).reg;
    let mut val: __le16 = 0;
    mutex_lock(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    let ret = regmap_raw_read((*da7213).regmap, reg, &mut val as *mut _ as *mut c_void, size_of::<__le16>());
    mutex_unlock(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    if ret != 0 { return ret; }
    (*ucontrol).value.integer.value[0] = u16::from_le(val) as i64;
    0
}

unsafe extern "C" fn da7213_tonegen_freq_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7213 = snd_soc_component_get_drvdata(component);
    let mixer_ctrl = (*kcontrol).private_value as *mut soc_mixer_control;
    let reg = (*mixer_ctrl).reg;
    let val_new: __le16 = ((*ucontrol).value.integer.value[0] as u16).to_le();
    let mut val_old: __le16 = 0;
    mutex_lock(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    let mut ret = regmap_raw_read((*da7213).regmap, reg, &mut val_old as *mut _ as *mut c_void, size_of::<__le16>());
    if ret == 0 && val_old != val_new {
        ret = regmap_raw_write((*da7213).regmap, reg, &val_new as *const _ as *const c_void, size_of::<__le16>());
    }
    mutex_unlock(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    if ret < 0 { return ret; }
    (val_old != val_new) as c_int
}

static da7213_snd_controls: &[snd_kcontrol_new] = &[
    SOC_SINGLE_TLV!("Mic 1 Volume", DA7213_MIC_1_GAIN, DA7213_MIC_AMP_GAIN_SHIFT, DA7213_MIC_AMP_GAIN_MAX, DA7213_NO_INVERT, mic_vol_tlv),
    SOC_SINGLE_TLV!("Mic 2 Volume", DA7213_MIC_2_GAIN, DA7213_MIC_AMP_GAIN_SHIFT, DA7213_MIC_AMP_GAIN_MAX, DA7213_NO_INVERT, mic_vol_tlv),
    SOC_DOUBLE_R_TLV!("Aux Volume", DA7213_AUX_L_GAIN, DA7213_AUX_R_GAIN, DA7213_AUX_AMP_GAIN_SHIFT, DA7213_AUX_AMP_GAIN_MAX, DA7213_NO_INVERT, aux_vol_tlv),
    SOC_DOUBLE_R_EXT_TLV!("Mixin PGA Volume", DA7213_MIXIN_L_GAIN, DA7213_MIXIN_R_GAIN, DA7213_MIXIN_AMP_GAIN_SHIFT, DA7213_MIXIN_AMP_GAIN_MAX, DA7213_NO_INVERT, snd_soc_get_volsw_2r, da7213_put_mixin_gain, mixin_gain_tlv),
    SOC_DOUBLE_R_TLV!("ADC Volume", DA7213_ADC_L_GAIN, DA7213_ADC_R_GAIN, DA7213_ADC_AMP_GAIN_SHIFT, DA7213_ADC_AMP_GAIN_MAX, DA7213_NO_INVERT, digital_gain_tlv),
    SOC_DOUBLE_R_TLV!("DAC Volume", DA7213_DAC_L_GAIN, DA7213_DAC_R_GAIN, DA7213_DAC_AMP_GAIN_SHIFT, DA7213_DAC_AMP_GAIN_MAX, DA7213_NO_INVERT, digital_gain_tlv),
    SOC_DOUBLE_R_TLV!("Headphone Volume", DA7213_HP_L_GAIN, DA7213_HP_R_GAIN, DA7213_HP_AMP_GAIN_SHIFT, DA7213_HP_AMP_GAIN_MAX, DA7213_NO_INVERT, hp_vol_tlv),
    SOC_SINGLE_TLV!("Lineout Volume", DA7213_LINE_GAIN, DA7213_LINE_AMP_GAIN_SHIFT, DA7213_LINE_AMP_GAIN_MAX, DA7213_NO_INVERT, lineout_vol_tlv),
    SOC_SINGLE!("DAC EQ Switch", DA7213_DAC_FILTERS4, DA7213_DAC_EQ_EN_SHIFT, DA7213_DAC_EQ_EN_MAX, DA7213_NO_INVERT),
    SOC_SINGLE_TLV!("DAC EQ1 Volume", DA7213_DAC_FILTERS2, DA7213_DAC_EQ_BAND1_SHIFT, DA7213_DAC_EQ_BAND_MAX, DA7213_NO_INVERT, eq_gain_tlv),
    SOC_SINGLE_TLV!("DAC EQ2 Volume", DA7213_DAC_FILTERS2, DA7213_DAC_EQ_BAND2_SHIFT, DA7213_DAC_EQ_BAND_MAX, DA7213_NO_INVERT, eq_gain_tlv),
    SOC_SINGLE_TLV!("DAC EQ3 Volume", DA7213_DAC_FILTERS3, DA7213_DAC_EQ_BAND3_SHIFT, DA7213_DAC_EQ_BAND_MAX, DA7213_NO_INVERT, eq_gain_tlv),
    SOC_SINGLE_TLV!("DAC EQ4 Volume", DA7213_DAC_FILTERS3, DA7213_DAC_EQ_BAND4_SHIFT, DA7213_DAC_EQ_BAND_MAX, DA7213_NO_INVERT, eq_gain_tlv),
    SOC_SINGLE_TLV!("DAC EQ5 Volume", DA7213_DAC_FILTERS4, DA7213_DAC_EQ_BAND5_SHIFT, DA7213_DAC_EQ_BAND_MAX, DA7213_NO_INVERT, eq_gain_tlv),
    SOC_SINGLE!("ADC HPF Switch", DA7213_ADC_FILTERS1, DA7213_HPF_EN_SHIFT, DA7213_HPF_EN_MAX, DA7213_NO_INVERT),
    SOC_ENUM!("ADC HPF Cutoff", da7213_adc_audio_hpf_corner),
    SOC_SINGLE!("ADC Voice Mode Switch", DA7213_ADC_FILTERS1, DA7213_VOICE_EN_SHIFT, DA7213_VOICE_EN_MAX, DA7213_NO_INVERT),
    SOC_ENUM!("ADC Voice Cutoff", da7213_adc_voice_hpf_corner),
    SOC_SINGLE!("DAC HPF Switch", DA7213_DAC_FILTERS1, DA7213_HPF_EN_SHIFT, DA7213_HPF_EN_MAX, DA7213_NO_INVERT),
    SOC_ENUM!("DAC HPF Cutoff", da7213_dac_audio_hpf_corner),
    SOC_SINGLE!("DAC Voice Mode Switch", DA7213_DAC_FILTERS1, DA7213_VOICE_EN_SHIFT, DA7213_VOICE_EN_MAX, DA7213_NO_INVERT),
    SOC_ENUM!("DAC Voice Cutoff", da7213_dac_voice_hpf_corner),
    SOC_SINGLE!("Mic 1 Switch", DA7213_MIC_1_CTRL, DA7213_MUTE_EN_SHIFT, DA7213_MUTE_EN_MAX, DA7213_INVERT),
    SOC_SINGLE!("Mic 2 Switch", DA7213_MIC_2_CTRL, DA7213_MUTE_EN_SHIFT, DA7213_MUTE_EN_MAX, DA7213_INVERT),
    SOC_DOUBLE_R!("Aux Switch", DA7213_AUX_L_CTRL, DA7213_AUX_R_CTRL, DA7213_MUTE_EN_SHIFT, DA7213_MUTE_EN_MAX, DA7213_INVERT),
    SOC_DOUBLE_R!("Mixin PGA Switch", DA7213_MIXIN_L_CTRL, DA7213_MIXIN_R_CTRL, DA7213_MUTE_EN_SHIFT, DA7213_MUTE_EN_MAX, DA7213_INVERT),
    SOC_DOUBLE_R!("ADC Switch", DA7213_ADC_L_CTRL, DA7213_ADC_R_CTRL, DA7213_MUTE_EN_SHIFT, DA7213_MUTE_EN_MAX, DA7213_INVERT),
    SOC_DOUBLE_R!("Headphone Switch", DA7213_HP_L_CTRL, DA7213_HP_R_CTRL, DA7213_MUTE_EN_SHIFT, DA7213_MUTE_EN_MAX, DA7213_INVERT),
    SOC_SINGLE!("Lineout Switch", DA7213_LINE_CTRL, DA7213_MUTE_EN_SHIFT, DA7213_MUTE_EN_MAX, DA7213_INVERT),
    SOC_SINGLE!("DAC Soft Mute Switch", DA7213_DAC_FILTERS5, DA7213_DAC_SOFTMUTE_EN_SHIFT, DA7213_DAC_SOFTMUTE_EN_MAX, DA7213_NO_INVERT),
    SOC_ENUM!("DAC Soft Mute Rate", da7213_dac_soft_mute_rate),
    SOC_DOUBLE_R!("Aux ZC Switch", DA7213_AUX_L_CTRL, DA7213_AUX_R_CTRL, DA7213_ZC_EN_SHIFT, DA7213_ZC_EN_MAX, DA7213_NO_INVERT),
    SOC_DOUBLE_R!("Mixin PGA ZC Switch", DA7213_MIXIN_L_CTRL, DA7213_MIXIN_R_CTRL, DA7213_ZC_EN_SHIFT, DA7213_ZC_EN_MAX, DA7213_NO_INVERT),
    SOC_DOUBLE_R!("Headphone ZC Switch", DA7213_HP_L_CTRL, DA7213_HP_R_CTRL, DA7213_ZC_EN_SHIFT, DA7213_ZC_EN_MAX, DA7213_NO_INVERT),
    SOC_SINGLE_EXT_TLV!("ToneGen Volume", DA7213_TONE_GEN_CFG2, DA7213_TONE_GEN_GAIN_SHIFT, DA7213_TONE_GEN_GAIN_MAX, DA7213_NO_INVERT, da7213_volsw_locked_get, da7213_volsw_locked_put, da7213_tonegen_gain_tlv),
    SOC_ENUM_EXT!("ToneGen DTMF Key", da7213_tonegen_dtmf_key, da7213_enum_locked_get, da7213_enum_locked_put),
    SOC_SINGLE_EXT!("ToneGen DTMF Switch", DA7213_TONE_GEN_CFG1, DA7213_DTMF_EN_SHIFT, DA7213_SWITCH_EN_MAX, DA7213_NO_INVERT, da7213_volsw_locked_get, da7213_volsw_locked_put),
    SOC_SINGLE_EXT!("ToneGen Start", DA7213_TONE_GEN_CFG1, DA7213_START_STOPN_SHIFT, DA7213_SWITCH_EN_MAX, DA7213_NO_INVERT, da7213_volsw_locked_get, da7213_volsw_locked_put),
    SOC_ENUM_EXT!("ToneGen Sinewave Gen Type", da7213_tonegen_swg_sel, da7213_enum_locked_get, da7213_enum_locked_put),
    SOC_SINGLE_EXT!("ToneGen Sinewave1 Freq", DA7213_TONE_GEN_FREQ1_L, DA7213_FREQ1_L_SHIFT, DA7213_FREQ_MAX, DA7213_NO_INVERT, da7213_tonegen_freq_get, da7213_tonegen_freq_put),
    SOC_SINGLE_EXT!("ToneGen Sinewave2 Freq", DA7213_TONE_GEN_FREQ2_L, DA7213_FREQ2_L_SHIFT, DA7213_FREQ_MAX, DA7213_NO_INVERT, da7213_tonegen_freq_get, da7213_tonegen_freq_put),
    SOC_SINGLE_EXT!("ToneGen On Time", DA7213_TONE_GEN_ON_PER, DA7213_BEEP_ON_PER_SHIFT, DA7213_BEEP_ON_OFF_MAX, DA7213_NO_INVERT, da7213_volsw_locked_get, da7213_volsw_locked_put),
    SOC_SINGLE!("ToneGen Off Time", DA7213_TONE_GEN_OFF_PER, DA7213_BEEP_OFF_PER_SHIFT, DA7213_BEEP_ON_OFF_MAX, DA7213_NO_INVERT),
    SOC_DOUBLE_R!("Aux Gain Ramping Switch", DA7213_AUX_L_CTRL, DA7213_AUX_R_CTRL, DA7213_GAIN_RAMP_EN_SHIFT, DA7213_GAIN_RAMP_EN_MAX, DA7213_NO_INVERT),
    SOC_DOUBLE_R!("Mixin Gain Ramping Switch", DA7213_MIXIN_L_CTRL, DA7213_MIXIN_R_CTRL, DA7213_GAIN_RAMP_EN_SHIFT, DA7213_GAIN_RAMP_EN_MAX, DA7213_NO_INVERT),
    SOC_DOUBLE_R!("ADC Gain Ramping Switch", DA7213_ADC_L_CTRL, DA7213_ADC_R_CTRL, DA7213_GAIN_RAMP_EN_SHIFT, DA7213_GAIN_RAMP_EN_MAX, DA7213_NO_INVERT),
    SOC_DOUBLE_R!("DAC Gain Ramping Switch", DA7213_DAC_L_CTRL, DA7213_DAC_R_CTRL, DA7213_GAIN_RAMP_EN_SHIFT, DA7213_GAIN_RAMP_EN_MAX, DA7213_NO_INVERT),
    SOC_DOUBLE_R!("Headphone Gain Ramping Switch", DA7213_HP_L_CTRL, DA7213_HP_R_CTRL, DA7213_GAIN_RAMP_EN_SHIFT, DA7213_GAIN_RAMP_EN_MAX, DA7213_NO_INVERT),
    SOC_SINGLE!("Lineout Gain Ramping Switch", DA7213_LINE_CTRL, DA7213_GAIN_RAMP_EN_SHIFT, DA7213_GAIN_RAMP_EN_MAX, DA7213_NO_INVERT),
    SOC_ENUM!("Gain Ramping Rate", da7213_gain_ramp_rate),
    SOC_SINGLE!("DAC NG Switch", DA7213_DAC_NG_CTRL, DA7213_DAC_NG_EN_SHIFT, DA7213_DAC_NG_EN_MAX, DA7213_NO_INVERT),
    SOC_ENUM!("DAC NG Setup Time", da7213_dac_ng_setup_time),
    SOC_ENUM!("DAC NG Rampup Rate", da7213_dac_ng_rampup_rate),
    SOC_ENUM!("DAC NG Rampdown Rate", da7213_dac_ng_rampdown_rate),
    SOC_SINGLE!("DAC NG OFF Threshold", DA7213_DAC_NG_OFF_THRESHOLD, DA7213_DAC_NG_THRESHOLD_SHIFT, DA7213_DAC_NG_THRESHOLD_MAX, DA7213_NO_INVERT),
    SOC_SINGLE!("DAC NG ON Threshold", DA7213_DAC_NG_ON_THRESHOLD, DA7213_DAC_NG_THRESHOLD_SHIFT, DA7213_DAC_NG_THRESHOLD_MAX, DA7213_NO_INVERT),
    SOC_DOUBLE!("DAC Mono Switch", DA7213_DIG_ROUTING_DAC, DA7213_DAC_L_MONO_SHIFT, DA7213_DAC_R_MONO_SHIFT, DA7213_DAC_MONO_MAX, DA7213_NO_INVERT),
    SOC_DOUBLE!("DAC Invert Switch", DA7213_DIG_CTRL, DA7213_DAC_L_INV_SHIFT, DA7213_DAC_R_INV_SHIFT, DA7213_DAC_INV_MAX, DA7213_NO_INVERT),
    SOC_DOUBLE_R!("DMIC Switch", DA7213_MIXIN_L_SELECT, DA7213_MIXIN_R_SELECT, DA7213_DMIC_EN_SHIFT, DA7213_DMIC_EN_MAX, DA7213_NO_INVERT),
    SOC_DOUBLE_EXT!("ALC Switch", DA7213_ALC_CTRL1, DA7213_ALC_L_EN_SHIFT, DA7213_ALC_R_EN_SHIFT, DA7213_ALC_EN_MAX, DA7213_NO_INVERT, snd_soc_get_volsw, da7213_put_alc_sw),
    SOC_ENUM!("ALC Attack Rate", da7213_alc_attack_rate),
    SOC_ENUM!("ALC Release Rate", da7213_alc_release_rate),
    SOC_ENUM!("ALC Hold Time", da7213_alc_hold_time),
    SOC_ENUM!("ALC Integ Attack Rate", da7213_alc_integ_attack_rate),
    SOC_ENUM!("ALC Integ Release Rate", da7213_alc_integ_release_rate),
    SOC_SINGLE_TLV!("ALC Noise Threshold Volume", DA7213_ALC_NOISE, DA7213_ALC_THRESHOLD_SHIFT, DA7213_ALC_THRESHOLD_MAX, DA7213_INVERT, alc_threshold_tlv),
    SOC_SINGLE_TLV!("ALC Min Threshold Volume", DA7213_ALC_TARGET_MIN, DA7213_ALC_THRESHOLD_SHIFT, DA7213_ALC_THRESHOLD_MAX, DA7213_INVERT, alc_threshold_tlv),
    SOC_SINGLE_TLV!("ALC Max Threshold Volume", DA7213_ALC_TARGET_MAX, DA7213_ALC_THRESHOLD_SHIFT, DA7213_ALC_THRESHOLD_MAX, DA7213_INVERT, alc_threshold_tlv),
    SOC_SINGLE_TLV!("ALC Max Attenuation Volume", DA7213_ALC_GAIN_LIMITS, DA7213_ALC_ATTEN_MAX_SHIFT, DA7213_ALC_ATTEN_GAIN_MAX_MAX, DA7213_NO_INVERT, alc_gain_tlv),
    SOC_SINGLE_TLV!("ALC Max Gain Volume", DA7213_ALC_GAIN_LIMITS, DA7213_ALC_GAIN_MAX_SHIFT, DA7213_ALC_ATTEN_GAIN_MAX_MAX, DA7213_NO_INVERT, alc_gain_tlv),
    SOC_SINGLE_TLV!("ALC Min Analog Gain Volume", DA7213_ALC_ANA_GAIN_LIMITS, DA7213_ALC_ANA_GAIN_MIN_SHIFT, DA7213_ALC_ANA_GAIN_MAX, DA7213_NO_INVERT, alc_analog_gain_tlv),
    SOC_SINGLE_TLV!("ALC Max Analog Gain Volume", DA7213_ALC_ANA_GAIN_LIMITS, DA7213_ALC_ANA_GAIN_MAX_SHIFT, DA7213_ALC_ANA_GAIN_MAX, DA7213_NO_INVERT, alc_analog_gain_tlv),
    SOC_SINGLE!("ALC Anticlip Mode Switch", DA7213_ALC_ANTICLIP_CTRL, DA7213_ALC_ANTICLIP_EN_SHIFT, DA7213_ALC_ANTICLIP_EN_MAX, DA7213_NO_INVERT),
    SOC_SINGLE!("ALC Anticlip Level", DA7213_ALC_ANTICLIP_LEVEL, DA7213_ALC_ANTICLIP_LEVEL_SHIFT, DA7213_ALC_ANTICLIP_LEVEL_MAX, DA7213_NO_INVERT),
];

static da7213_mic_amp_in_sel_txt: [*const c_char; 3] = [c"Differential".as_ptr(), c"MIC_P".as_ptr(), c"MIC_N".as_ptr()];
static da7213_mic_1_amp_in_sel: soc_enum = SOC_ENUM_SINGLE_DECL!(DA7213_MIC_1_CTRL, DA7213_MIC_AMP_IN_SEL_SHIFT, da7213_mic_amp_in_sel_txt);
static da7213_mic_1_amp_in_sel_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Mic 1 Amp Source MUX", da7213_mic_1_amp_in_sel);
static da7213_mic_2_amp_in_sel: soc_enum = SOC_ENUM_SINGLE_DECL!(DA7213_MIC_2_CTRL, DA7213_MIC_AMP_IN_SEL_SHIFT, da7213_mic_amp_in_sel_txt);
static da7213_mic_2_amp_in_sel_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Mic 2 Amp Source MUX", da7213_mic_2_amp_in_sel);
static da7213_dai_src_txt: [*const c_char; 4] = [c"ADC Left".as_ptr(), c"ADC Right".as_ptr(), c"DAI Input Left".as_ptr(), c"DAI Input Right".as_ptr()];
static da7213_dai_l_src: soc_enum = SOC_ENUM_SINGLE_DECL!(DA7213_DIG_ROUTING_DAI, DA7213_DAI_L_SRC_SHIFT, da7213_dai_src_txt);
static da7213_dai_l_src_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("DAI Left Source MUX", da7213_dai_l_src);
static da7213_dai_r_src: soc_enum = SOC_ENUM_SINGLE_DECL!(DA7213_DIG_ROUTING_DAI, DA7213_DAI_R_SRC_SHIFT, da7213_dai_src_txt);
static da7213_dai_r_src_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("DAI Right Source MUX", da7213_dai_r_src);
static da7213_dac_src_txt: [*const c_char; 4] = [c"ADC Output Left".as_ptr(), c"ADC Output Right".as_ptr(), c"DAI Input Left".as_ptr(), c"DAI Input Right".as_ptr()];
static da7213_dac_l_src: soc_enum = SOC_ENUM_SINGLE_DECL!(DA7213_DIG_ROUTING_DAC, DA7213_DAC_L_SRC_SHIFT, da7213_dac_src_txt);
static da7213_dac_l_src_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("DAC Left Source MUX", da7213_dac_l_src);
static da7213_dac_r_src: soc_enum = SOC_ENUM_SINGLE_DECL!(DA7213_DIG_ROUTING_DAC, DA7213_DAC_R_SRC_SHIFT, da7213_dac_src_txt);
static da7213_dac_r_src_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("DAC Right Source MUX", da7213_dac_r_src);

static da7213_dapm_mixinl_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Aux Left Switch", DA7213_MIXIN_L_SELECT, DA7213_MIXIN_L_MIX_SELECT_AUX_L_SHIFT, DA7213_MIXIN_L_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mic 1 Switch", DA7213_MIXIN_L_SELECT, DA7213_MIXIN_L_MIX_SELECT_MIC_1_SHIFT, DA7213_MIXIN_L_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mic 2 Switch", DA7213_MIXIN_L_SELECT, DA7213_MIXIN_L_MIX_SELECT_MIC_2_SHIFT, DA7213_MIXIN_L_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mixin Right Switch", DA7213_MIXIN_L_SELECT, DA7213_MIXIN_L_MIX_SELECT_MIXIN_R_SHIFT, DA7213_MIXIN_L_MIX_SELECT_MAX, DA7213_NO_INVERT),
];
static da7213_dapm_mixinr_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Aux Right Switch", DA7213_MIXIN_R_SELECT, DA7213_MIXIN_R_MIX_SELECT_AUX_R_SHIFT, DA7213_MIXIN_R_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mic 2 Switch", DA7213_MIXIN_R_SELECT, DA7213_MIXIN_R_MIX_SELECT_MIC_2_SHIFT, DA7213_MIXIN_R_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mic 1 Switch", DA7213_MIXIN_R_SELECT, DA7213_MIXIN_R_MIX_SELECT_MIC_1_SHIFT, DA7213_MIXIN_R_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mixin Left Switch", DA7213_MIXIN_R_SELECT, DA7213_MIXIN_R_MIX_SELECT_MIXIN_L_SHIFT, DA7213_MIXIN_R_MIX_SELECT_MAX, DA7213_NO_INVERT),
];
static da7213_dapm_mixoutl_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Aux Left Switch", DA7213_MIXOUT_L_SELECT, DA7213_MIXOUT_L_MIX_SELECT_AUX_L_SHIFT, DA7213_MIXOUT_L_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mixin Left Switch", DA7213_MIXOUT_L_SELECT, DA7213_MIXOUT_L_MIX_SELECT_MIXIN_L_SHIFT, DA7213_MIXOUT_L_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mixin Right Switch", DA7213_MIXOUT_L_SELECT, DA7213_MIXOUT_L_MIX_SELECT_MIXIN_R_SHIFT, DA7213_MIXOUT_L_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("DAC Left Switch", DA7213_MIXOUT_L_SELECT, DA7213_MIXOUT_L_MIX_SELECT_DAC_L_SHIFT, DA7213_MIXOUT_L_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Aux Left Invert Switch", DA7213_MIXOUT_L_SELECT, DA7213_MIXOUT_L_MIX_SELECT_AUX_L_INVERTED_SHIFT, DA7213_MIXOUT_L_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mixin Left Invert Switch", DA7213_MIXOUT_L_SELECT, DA7213_MIXOUT_L_MIX_SELECT_MIXIN_L_INVERTED_SHIFT, DA7213_MIXOUT_L_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mixin Right Invert Switch", DA7213_MIXOUT_L_SELECT, DA7213_MIXOUT_L_MIX_SELECT_MIXIN_R_INVERTED_SHIFT, DA7213_MIXOUT_L_MIX_SELECT_MAX, DA7213_NO_INVERT),
];
static da7213_dapm_mixoutr_controls: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Aux Right Switch", DA7213_MIXOUT_R_SELECT, DA7213_MIXOUT_R_MIX_SELECT_AUX_R_SHIFT, DA7213_MIXOUT_R_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mixin Right Switch", DA7213_MIXOUT_R_SELECT, DA7213_MIXOUT_R_MIX_SELECT_MIXIN_R_SHIFT, DA7213_MIXOUT_R_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mixin Left Switch", DA7213_MIXOUT_R_SELECT, DA7213_MIXOUT_R_MIX_SELECT_MIXIN_L_SHIFT, DA7213_MIXOUT_R_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("DAC Right Switch", DA7213_MIXOUT_R_SELECT, DA7213_MIXOUT_R_MIX_SELECT_DAC_R_SHIFT, DA7213_MIXOUT_R_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Aux Right Invert Switch", DA7213_MIXOUT_R_SELECT, DA7213_MIXOUT_R_MIX_SELECT_AUX_R_INVERTED_SHIFT, DA7213_MIXOUT_R_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mixin Right Invert Switch", DA7213_MIXOUT_R_SELECT, DA7213_MIXOUT_R_MIX_SELECT_MIXIN_R_INVERTED_SHIFT, DA7213_MIXOUT_R_MIX_SELECT_MAX, DA7213_NO_INVERT),
    SOC_DAPM_SINGLE!("Mixin Left Invert Switch", DA7213_MIXOUT_R_SELECT, DA7213_MIXOUT_R_MIX_SELECT_MIXIN_L_INVERTED_SHIFT, DA7213_MIXOUT_R_MIX_SELECT_MAX, DA7213_NO_INVERT),
];

unsafe extern "C" fn da7213_dai_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let da7213 = snd_soc_component_get_drvdata(component);
    let mut i = 0;
    let mut srm_lock = false;
    match event as c_uint {
        SND_SOC_DAPM_PRE_PMU => {
            if (*da7213).master {
                snd_soc_component_update_bits(component, DA7213_DAI_CLK_MODE, DA7213_DAI_CLK_EN_MASK, DA7213_DAI_CLK_EN_MASK);
            }
            snd_soc_component_update_bits(component, DA7213_PC_COUNT, DA7213_PC_FREERUN_MASK, 0);
            let pll_ctrl = snd_soc_component_read(component, DA7213_PLL_CTRL) as u8;
            if (pll_ctrl as c_uint & DA7213_PLL_SRM_EN) == 0 { return 0; }
            if (pll_ctrl as c_uint & DA7213_PLL_32K_MODE) != 0 {
                snd_soc_component_write(component, 0xF0, 0x8B);
                snd_soc_component_write(component, 0xF2, 0x03);
                snd_soc_component_write(component, 0xF0, 0x00);
            }
            loop {
                let pll_status = snd_soc_component_read(component, DA7213_PLL_STATUS) as u8;
                if (pll_status as c_uint & DA7213_PLL_SRM_LOCK) != 0 {
                    srm_lock = true;
                } else {
                    i += 1;
                    msleep(50);
                }
                if !(i < DA7213_SRM_CHECK_RETRIES && !srm_lock) { break; }
            }
            if !srm_lock { dev_warn((*component).dev, c"SRM failed to lock\n".as_ptr()); }
            0
        }
        SND_SOC_DAPM_POST_PMD => {
            let pll_ctrl = snd_soc_component_read(component, DA7213_PLL_CTRL) as u8;
            if (pll_ctrl as c_uint & DA7213_PLL_32K_MODE) != 0 {
                snd_soc_component_write(component, 0xF0, 0x8B);
                snd_soc_component_write(component, 0xF2, 0x01);
                snd_soc_component_write(component, 0xF0, 0x00);
            }
            snd_soc_component_update_bits(component, DA7213_PC_COUNT, DA7213_PC_FREERUN_MASK, DA7213_PC_FREERUN_MASK);
            if (*da7213).master {
                snd_soc_component_update_bits(component, DA7213_DAI_CLK_MODE, DA7213_DAI_CLK_EN_MASK, 0);
            }
            0
        }
        _ => -EINVAL,
    }
}

static da7213_dapm_widgets: &[snd_soc_dapm_widget_decl] = &[
    SND_SOC_DAPM_REGULATOR_SUPPLY!("VDDMIC", 0, 0),
    SND_SOC_DAPM_SUPPLY!("DAI", DA7213_DAI_CTRL, DA7213_DAI_EN_SHIFT, DA7213_NO_INVERT, da7213_dai_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_INPUT!("MIC1"), SND_SOC_DAPM_INPUT!("MIC2"), SND_SOC_DAPM_INPUT!("AUXL"), SND_SOC_DAPM_INPUT!("AUXR"),
    SND_SOC_DAPM_MUX!("Mic 1 Amp Source MUX", SND_SOC_NOPM, 0, 0, &da7213_mic_1_amp_in_sel_mux),
    SND_SOC_DAPM_MUX!("Mic 2 Amp Source MUX", SND_SOC_NOPM, 0, 0, &da7213_mic_2_amp_in_sel_mux),
    SND_SOC_DAPM_PGA!("Mic 1 PGA", DA7213_MIC_1_CTRL, DA7213_AMP_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Mic 2 PGA", DA7213_MIC_2_CTRL, DA7213_AMP_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Aux Left PGA", DA7213_AUX_L_CTRL, DA7213_AMP_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Aux Right PGA", DA7213_AUX_R_CTRL, DA7213_AMP_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Mixin Left PGA", DA7213_MIXIN_L_CTRL, DA7213_AMP_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Mixin Right PGA", DA7213_MIXIN_R_CTRL, DA7213_AMP_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Bias 1", DA7213_MICBIAS_CTRL, DA7213_MICBIAS1_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Bias 2", DA7213_MICBIAS_CTRL, DA7213_MICBIAS2_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Mixin Left", SND_SOC_NOPM, 0, 0, &da7213_dapm_mixinl_controls[0], ARRAY_SIZE!(da7213_dapm_mixinl_controls)),
    SND_SOC_DAPM_MIXER!("Mixin Right", SND_SOC_NOPM, 0, 0, &da7213_dapm_mixinr_controls[0], ARRAY_SIZE!(da7213_dapm_mixinr_controls)),
    SND_SOC_DAPM_ADC!("ADC Left", ptr::null(), DA7213_ADC_L_CTRL, DA7213_ADC_EN_SHIFT, DA7213_NO_INVERT),
    SND_SOC_DAPM_ADC!("ADC Right", ptr::null(), DA7213_ADC_R_CTRL, DA7213_ADC_EN_SHIFT, DA7213_NO_INVERT),
    SND_SOC_DAPM_MUX!("DAI Left Source MUX", SND_SOC_NOPM, 0, 0, &da7213_dai_l_src_mux),
    SND_SOC_DAPM_MUX!("DAI Right Source MUX", SND_SOC_NOPM, 0, 0, &da7213_dai_r_src_mux),
    SND_SOC_DAPM_AIF_OUT!("DAIOUTL", "Capture", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("DAIOUTR", "Capture", 1, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("DAIINL", "Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("DAIINR", "Playback", 1, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX!("DAC Left Source MUX", SND_SOC_NOPM, 0, 0, &da7213_dac_l_src_mux),
    SND_SOC_DAPM_MUX!("DAC Right Source MUX", SND_SOC_NOPM, 0, 0, &da7213_dac_r_src_mux),
    SND_SOC_DAPM_DAC!("DAC Left", ptr::null(), DA7213_DAC_L_CTRL, DA7213_DAC_EN_SHIFT, DA7213_NO_INVERT),
    SND_SOC_DAPM_DAC!("DAC Right", ptr::null(), DA7213_DAC_R_CTRL, DA7213_DAC_EN_SHIFT, DA7213_NO_INVERT),
    SND_SOC_DAPM_MIXER!("Mixout Left", SND_SOC_NOPM, 0, 0, &da7213_dapm_mixoutl_controls[0], ARRAY_SIZE!(da7213_dapm_mixoutl_controls)),
    SND_SOC_DAPM_MIXER!("Mixout Right", SND_SOC_NOPM, 0, 0, &da7213_dapm_mixoutr_controls[0], ARRAY_SIZE!(da7213_dapm_mixoutr_controls)),
    SND_SOC_DAPM_PGA!("Mixout Left PGA", DA7213_MIXOUT_L_CTRL, DA7213_AMP_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Mixout Right PGA", DA7213_MIXOUT_R_CTRL, DA7213_AMP_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Lineout PGA", DA7213_LINE_CTRL, DA7213_AMP_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Headphone Left PGA", DA7213_HP_L_CTRL, DA7213_AMP_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Headphone Right PGA", DA7213_HP_R_CTRL, DA7213_AMP_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Charge Pump", DA7213_CP_CTRL, DA7213_CP_EN_SHIFT, DA7213_NO_INVERT, ptr::null(), 0),
    SND_SOC_DAPM_OUTPUT!("HPL"), SND_SOC_DAPM_OUTPUT!("HPR"), SND_SOC_DAPM_OUTPUT!("LINE"),
];

static da7213_audio_map: &[snd_soc_dapm_route] = &[
    route!("Mic Bias 1", NULL, "VDDMIC"), route!("Mic Bias 2", NULL, "VDDMIC"),
    route!("MIC1", NULL, "Mic Bias 1"), route!("MIC2", NULL, "Mic Bias 2"),
    route!("Mic 1 Amp Source MUX", "Differential", "MIC1"), route!("Mic 1 Amp Source MUX", "MIC_P", "MIC1"), route!("Mic 1 Amp Source MUX", "MIC_N", "MIC1"),
    route!("Mic 2 Amp Source MUX", "Differential", "MIC2"), route!("Mic 2 Amp Source MUX", "MIC_P", "MIC2"), route!("Mic 2 Amp Source MUX", "MIC_N", "MIC2"),
    route!("Mic 1 PGA", NULL, "Mic 1 Amp Source MUX"), route!("Mic 2 PGA", NULL, "Mic 2 Amp Source MUX"),
    route!("Aux Left PGA", NULL, "AUXL"), route!("Aux Right PGA", NULL, "AUXR"),
    route!("Mixin Left", "Aux Left Switch", "Aux Left PGA"), route!("Mixin Left", "Mic 1 Switch", "Mic 1 PGA"), route!("Mixin Left", "Mic 2 Switch", "Mic 2 PGA"), route!("Mixin Left", "Mixin Right Switch", "Mixin Right PGA"),
    route!("Mixin Right", "Aux Right Switch", "Aux Right PGA"), route!("Mixin Right", "Mic 2 Switch", "Mic 2 PGA"), route!("Mixin Right", "Mic 1 Switch", "Mic 1 PGA"), route!("Mixin Right", "Mixin Left Switch", "Mixin Left PGA"),
    route!("Mixin Left PGA", NULL, "Mixin Left"), route!("ADC Left", NULL, "Mixin Left PGA"), route!("Mixin Right PGA", NULL, "Mixin Right"), route!("ADC Right", NULL, "Mixin Right PGA"),
    route!("DAI Left Source MUX", "ADC Left", "ADC Left"), route!("DAI Left Source MUX", "ADC Right", "ADC Right"), route!("DAI Left Source MUX", "DAI Input Left", "DAIINL"), route!("DAI Left Source MUX", "DAI Input Right", "DAIINR"),
    route!("DAI Right Source MUX", "ADC Left", "ADC Left"), route!("DAI Right Source MUX", "ADC Right", "ADC Right"), route!("DAI Right Source MUX", "DAI Input Left", "DAIINL"), route!("DAI Right Source MUX", "DAI Input Right", "DAIINR"),
    route!("DAIOUTL", NULL, "DAI Left Source MUX"), route!("DAIOUTR", NULL, "DAI Right Source MUX"), route!("DAIOUTL", NULL, "DAI"), route!("DAIOUTR", NULL, "DAI"),
    route!("DAIINL", NULL, "DAI"), route!("DAIINR", NULL, "DAI"),
    route!("DAC Left Source MUX", "ADC Output Left", "ADC Left"), route!("DAC Left Source MUX", "ADC Output Right", "ADC Right"), route!("DAC Left Source MUX", "DAI Input Left", "DAIINL"), route!("DAC Left Source MUX", "DAI Input Right", "DAIINR"),
    route!("DAC Right Source MUX", "ADC Output Left", "ADC Left"), route!("DAC Right Source MUX", "ADC Output Right", "ADC Right"), route!("DAC Right Source MUX", "DAI Input Left", "DAIINL"), route!("DAC Right Source MUX", "DAI Input Right", "DAIINR"),
    route!("DAC Left", NULL, "DAC Left Source MUX"), route!("DAC Right", NULL, "DAC Right Source MUX"),
    route!("Mixout Left", "Aux Left Switch", "Aux Left PGA"), route!("Mixout Left", "Mixin Left Switch", "Mixin Left PGA"), route!("Mixout Left", "Mixin Right Switch", "Mixin Right PGA"), route!("Mixout Left", "DAC Left Switch", "DAC Left"), route!("Mixout Left", "Aux Left Invert Switch", "Aux Left PGA"), route!("Mixout Left", "Mixin Left Invert Switch", "Mixin Left PGA"), route!("Mixout Left", "Mixin Right Invert Switch", "Mixin Right PGA"),
    route!("Mixout Right", "Aux Right Switch", "Aux Right PGA"), route!("Mixout Right", "Mixin Right Switch", "Mixin Right PGA"), route!("Mixout Right", "Mixin Left Switch", "Mixin Left PGA"), route!("Mixout Right", "DAC Right Switch", "DAC Right"), route!("Mixout Right", "Aux Right Invert Switch", "Aux Right PGA"), route!("Mixout Right", "Mixin Right Invert Switch", "Mixin Right PGA"), route!("Mixout Right", "Mixin Left Invert Switch", "Mixin Left PGA"),
    route!("Mixout Left PGA", NULL, "Mixout Left"), route!("Mixout Right PGA", NULL, "Mixout Right"),
    route!("Headphone Left PGA", NULL, "Mixout Left PGA"), route!("Headphone Left PGA", NULL, "Charge Pump"), route!("HPL", NULL, "Headphone Left PGA"),
    route!("Headphone Right PGA", NULL, "Mixout Right PGA"), route!("Headphone Right PGA", NULL, "Charge Pump"), route!("HPR", NULL, "Headphone Right PGA"),
    route!("Lineout PGA", NULL, "Mixout Right PGA"), route!("LINE", NULL, "Lineout PGA"),
];

static da7213_reg_defaults: &[reg_default] = &[
    reg_default { reg: DA7213_DIG_ROUTING_DAI, def: 0x10 }, reg_default { reg: DA7213_SR, def: 0x0A },
    reg_default { reg: DA7213_REFERENCES, def: 0x80 }, reg_default { reg: DA7213_PLL_FRAC_TOP, def: 0x00 },
    reg_default { reg: DA7213_PLL_FRAC_BOT, def: 0x00 }, reg_default { reg: DA7213_PLL_INTEGER, def: 0x20 },
    reg_default { reg: DA7213_PLL_CTRL, def: 0x0C }, reg_default { reg: DA7213_DAI_CLK_MODE, def: 0x01 },
    reg_default { reg: DA7213_DAI_CTRL, def: 0x08 }, reg_default { reg: DA7213_DIG_ROUTING_DAC, def: 0x32 },
    reg_default { reg: DA7213_AUX_L_GAIN, def: 0x35 }, reg_default { reg: DA7213_AUX_R_GAIN, def: 0x35 },
    reg_default { reg: DA7213_MIXIN_L_SELECT, def: 0x00 }, reg_default { reg: DA7213_MIXIN_R_SELECT, def: 0x00 },
    reg_default { reg: DA7213_MIXIN_L_GAIN, def: 0x03 }, reg_default { reg: DA7213_MIXIN_R_GAIN, def: 0x03 },
    reg_default { reg: DA7213_ADC_L_GAIN, def: 0x6F }, reg_default { reg: DA7213_ADC_R_GAIN, def: 0x6F },
    reg_default { reg: DA7213_ADC_FILTERS1, def: 0x80 }, reg_default { reg: DA7213_MIC_1_GAIN, def: 0x01 },
    reg_default { reg: DA7213_MIC_2_GAIN, def: 0x01 }, reg_default { reg: DA7213_DAC_FILTERS5, def: 0x00 },
    reg_default { reg: DA7213_DAC_FILTERS2, def: 0x88 }, reg_default { reg: DA7213_DAC_FILTERS3, def: 0x88 },
    reg_default { reg: DA7213_DAC_FILTERS4, def: 0x08 }, reg_default { reg: DA7213_DAC_FILTERS1, def: 0x80 },
    reg_default { reg: DA7213_DAC_L_GAIN, def: 0x6F }, reg_default { reg: DA7213_DAC_R_GAIN, def: 0x6F },
    reg_default { reg: DA7213_CP_CTRL, def: 0x61 }, reg_default { reg: DA7213_HP_L_GAIN, def: 0x39 },
    reg_default { reg: DA7213_HP_R_GAIN, def: 0x39 }, reg_default { reg: DA7213_LINE_GAIN, def: 0x30 },
    reg_default { reg: DA7213_MIXOUT_L_SELECT, def: 0x00 }, reg_default { reg: DA7213_MIXOUT_R_SELECT, def: 0x00 },
    reg_default { reg: DA7213_SYSTEM_MODES_INPUT, def: 0x00 }, reg_default { reg: DA7213_SYSTEM_MODES_OUTPUT, def: 0x00 },
    reg_default { reg: DA7213_AUX_L_CTRL, def: 0x44 }, reg_default { reg: DA7213_AUX_R_CTRL, def: 0x44 },
    reg_default { reg: DA7213_MICBIAS_CTRL, def: 0x11 }, reg_default { reg: DA7213_MIC_1_CTRL, def: 0x40 },
    reg_default { reg: DA7213_MIC_2_CTRL, def: 0x40 }, reg_default { reg: DA7213_MIXIN_L_CTRL, def: 0x40 },
    reg_default { reg: DA7213_MIXIN_R_CTRL, def: 0x40 }, reg_default { reg: DA7213_ADC_L_CTRL, def: 0x40 },
    reg_default { reg: DA7213_ADC_R_CTRL, def: 0x40 }, reg_default { reg: DA7213_DAC_L_CTRL, def: 0x48 },
    reg_default { reg: DA7213_DAC_R_CTRL, def: 0x40 }, reg_default { reg: DA7213_HP_L_CTRL, def: 0x41 },
    reg_default { reg: DA7213_HP_R_CTRL, def: 0x40 }, reg_default { reg: DA7213_LINE_CTRL, def: 0x40 },
    reg_default { reg: DA7213_MIXOUT_L_CTRL, def: 0x10 }, reg_default { reg: DA7213_MIXOUT_R_CTRL, def: 0x10 },
    reg_default { reg: DA7213_LDO_CTRL, def: 0x00 }, reg_default { reg: DA7213_IO_CTRL, def: 0x00 },
    reg_default { reg: DA7213_GAIN_RAMP_CTRL, def: 0x00 }, reg_default { reg: DA7213_MIC_CONFIG, def: 0x00 },
    reg_default { reg: DA7213_PC_COUNT, def: 0x00 }, reg_default { reg: DA7213_CP_VOL_THRESHOLD1, def: 0x32 },
    reg_default { reg: DA7213_CP_DELAY, def: 0x95 }, reg_default { reg: DA7213_CP_DETECTOR, def: 0x00 },
    reg_default { reg: DA7213_DAI_OFFSET, def: 0x00 }, reg_default { reg: DA7213_DIG_CTRL, def: 0x00 },
    reg_default { reg: DA7213_ALC_CTRL2, def: 0x00 }, reg_default { reg: DA7213_ALC_CTRL3, def: 0x00 },
    reg_default { reg: DA7213_ALC_NOISE, def: 0x3F }, reg_default { reg: DA7213_ALC_TARGET_MIN, def: 0x3F },
    reg_default { reg: DA7213_ALC_TARGET_MAX, def: 0x00 }, reg_default { reg: DA7213_ALC_GAIN_LIMITS, def: 0xFF },
    reg_default { reg: DA7213_ALC_ANA_GAIN_LIMITS, def: 0x71 }, reg_default { reg: DA7213_ALC_ANTICLIP_CTRL, def: 0x00 },
    reg_default { reg: DA7213_ALC_ANTICLIP_LEVEL, def: 0x00 }, reg_default { reg: DA7213_ALC_OFFSET_MAN_M_L, def: 0x00 },
    reg_default { reg: DA7213_ALC_OFFSET_MAN_U_L, def: 0x00 }, reg_default { reg: DA7213_ALC_OFFSET_MAN_M_R, def: 0x00 },
    reg_default { reg: DA7213_ALC_OFFSET_MAN_U_R, def: 0x00 }, reg_default { reg: DA7213_ALC_CIC_OP_LVL_CTRL, def: 0x00 },
    reg_default { reg: DA7213_DAC_NG_SETUP_TIME, def: 0x00 }, reg_default { reg: DA7213_DAC_NG_OFF_THRESHOLD, def: 0x00 },
    reg_default { reg: DA7213_DAC_NG_ON_THRESHOLD, def: 0x00 }, reg_default { reg: DA7213_DAC_NG_CTRL, def: 0x00 },
];

unsafe extern "C" fn da7213_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    matches!(reg,
        DA7213_STATUS1 | DA7213_PLL_STATUS | DA7213_AUX_L_GAIN_STATUS | DA7213_AUX_R_GAIN_STATUS |
        DA7213_MIC_1_GAIN_STATUS | DA7213_MIC_2_GAIN_STATUS | DA7213_MIXIN_L_GAIN_STATUS |
        DA7213_MIXIN_R_GAIN_STATUS | DA7213_ADC_L_GAIN_STATUS | DA7213_ADC_R_GAIN_STATUS |
        DA7213_DAC_L_GAIN_STATUS | DA7213_DAC_R_GAIN_STATUS | DA7213_HP_L_GAIN_STATUS |
        DA7213_HP_R_GAIN_STATUS | DA7213_LINE_GAIN_STATUS | DA7213_ALC_CTRL1 |
        DA7213_ALC_OFFSET_AUTO_M_L | DA7213_ALC_OFFSET_AUTO_U_L |
        DA7213_ALC_OFFSET_AUTO_M_R | DA7213_ALC_OFFSET_AUTO_U_R | DA7213_ALC_CIC_OP_LVL_DATA)
}

unsafe extern "C" fn da7213_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let da7213 = snd_soc_component_get_drvdata(component);
    let mut dai_clk_mode: u8 = DA7213_DAI_BCLKS_PER_WCLK_64 as u8;
    let mut dai_ctrl: u8 = 0;
    match params_channels(params) {
        1 => {
            if (*da7213).fmt as c_uint != DA7213_DAI_FORMAT_DSP {
                dev_err((*component).dev, c"Mono supported only in DSP mode\n".as_ptr());
                return -EINVAL;
            }
            dai_ctrl |= DA7213_DAI_MONO_MODE_EN as u8;
        }
        2 => dai_ctrl &= !(DA7213_DAI_MONO_MODE_EN as u8),
        _ => return -EINVAL,
    }
    match params_width(params) {
        16 => { dai_ctrl |= DA7213_DAI_WORD_LENGTH_S16_LE as u8; dai_clk_mode = DA7213_DAI_BCLKS_PER_WCLK_32 as u8; }
        20 => dai_ctrl |= DA7213_DAI_WORD_LENGTH_S20_LE as u8,
        24 => dai_ctrl |= DA7213_DAI_WORD_LENGTH_S24_LE as u8,
        32 => dai_ctrl |= DA7213_DAI_WORD_LENGTH_S32_LE as u8,
        _ => return -EINVAL,
    }
    let fs: u8 = match params_rate(params) {
        8000 => { (*da7213).out_rate = DA7213_PLL_FREQ_OUT_98304000; DA7213_SR_8000 as u8 }
        11025 => { (*da7213).out_rate = DA7213_PLL_FREQ_OUT_90316800; DA7213_SR_11025 as u8 }
        12000 => { (*da7213).out_rate = DA7213_PLL_FREQ_OUT_98304000; DA7213_SR_12000 as u8 }
        16000 => { (*da7213).out_rate = DA7213_PLL_FREQ_OUT_98304000; DA7213_SR_16000 as u8 }
        22050 => { (*da7213).out_rate = DA7213_PLL_FREQ_OUT_90316800; DA7213_SR_22050 as u8 }
        32000 => { (*da7213).out_rate = DA7213_PLL_FREQ_OUT_98304000; DA7213_SR_32000 as u8 }
        44100 => { (*da7213).out_rate = DA7213_PLL_FREQ_OUT_90316800; DA7213_SR_44100 as u8 }
        48000 => { (*da7213).out_rate = DA7213_PLL_FREQ_OUT_98304000; DA7213_SR_48000 as u8 }
        88200 => { (*da7213).out_rate = DA7213_PLL_FREQ_OUT_90316800; DA7213_SR_88200 as u8 }
        96000 => { (*da7213).out_rate = DA7213_PLL_FREQ_OUT_98304000; DA7213_SR_96000 as u8 }
        _ => return -EINVAL,
    };
    snd_soc_component_update_bits(component, DA7213_DAI_CLK_MODE, DA7213_DAI_BCLKS_PER_WCLK_MASK, dai_clk_mode as c_uint);
    snd_soc_component_update_bits(component, DA7213_DAI_CTRL, DA7213_DAI_WORD_LENGTH_MASK | DA7213_DAI_MONO_MODE_MASK, dai_ctrl as c_uint);
    snd_soc_component_write(component, DA7213_SR, fs as c_uint);
    0
}

unsafe extern "C" fn da7213_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let da7213 = snd_soc_component_get_drvdata(component);
    let mut dai_clk_mode: u8 = 0;
    let mut dai_ctrl: u8 = 0;
    let mut dai_offset: u8 = 0;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => (*da7213).master = true,
        SND_SOC_DAIFMT_CBC_CFC => (*da7213).master = false,
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J => match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => {}
            SND_SOC_DAIFMT_NB_IF => dai_clk_mode |= DA7213_DAI_WCLK_POL_INV as u8,
            SND_SOC_DAIFMT_IB_NF => dai_clk_mode |= DA7213_DAI_CLK_POL_INV as u8,
            SND_SOC_DAIFMT_IB_IF => dai_clk_mode |= (DA7213_DAI_WCLK_POL_INV | DA7213_DAI_CLK_POL_INV) as u8,
            _ => return -EINVAL,
        },
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => dai_clk_mode |= DA7213_DAI_CLK_POL_INV as u8,
            SND_SOC_DAIFMT_NB_IF => dai_clk_mode |= (DA7213_DAI_WCLK_POL_INV | DA7213_DAI_CLK_POL_INV) as u8,
            SND_SOC_DAIFMT_IB_NF => {}
            SND_SOC_DAIFMT_IB_IF => dai_clk_mode |= DA7213_DAI_WCLK_POL_INV as u8,
            _ => return -EINVAL,
        },
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => { dai_ctrl |= DA7213_DAI_FORMAT_I2S_MODE as u8; (*da7213).fmt = DA7213_DAI_FORMAT_I2S_MODE as u8; }
        SND_SOC_DAIFMT_LEFT_J => { dai_ctrl |= DA7213_DAI_FORMAT_LEFT_J as u8; (*da7213).fmt = DA7213_DAI_FORMAT_LEFT_J as u8; }
        SND_SOC_DAIFMT_RIGHT_J => { dai_ctrl |= DA7213_DAI_FORMAT_RIGHT_J as u8; (*da7213).fmt = DA7213_DAI_FORMAT_RIGHT_J as u8; }
        SND_SOC_DAIFMT_DSP_A => { dai_ctrl |= DA7213_DAI_FORMAT_DSP as u8; dai_offset = 1; (*da7213).fmt = DA7213_DAI_FORMAT_DSP as u8; }
        SND_SOC_DAIFMT_DSP_B => { dai_ctrl |= DA7213_DAI_FORMAT_DSP as u8; (*da7213).fmt = DA7213_DAI_FORMAT_DSP as u8; }
        _ => return -EINVAL,
    }
    dai_clk_mode |= DA7213_DAI_BCLKS_PER_WCLK_64 as u8;
    snd_soc_component_update_bits(component, DA7213_DAI_CLK_MODE,
        DA7213_DAI_BCLKS_PER_WCLK_MASK | DA7213_DAI_CLK_POL_MASK | DA7213_DAI_WCLK_POL_MASK, dai_clk_mode as c_uint);
    snd_soc_component_update_bits(component, DA7213_DAI_CTRL, DA7213_DAI_FORMAT_MASK, dai_ctrl as c_uint);
    snd_soc_component_write(component, DA7213_DAI_OFFSET, dai_offset as c_uint);
    0
}

unsafe extern "C" fn da7213_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let val = if mute != 0 { DA7213_MUTE_EN } else { 0 };
    snd_soc_component_update_bits(component, DA7213_DAC_L_CTRL, DA7213_MUTE_EN, val);
    snd_soc_component_update_bits(component, DA7213_DAC_R_CTRL, DA7213_MUTE_EN, val);
    0
}

unsafe extern "C" fn da7213_set_component_sysclk(component: *mut snd_soc_component, clk_id: c_int, _source: c_int, mut freq: c_uint, _dir: c_int) -> c_int {
    let da7213 = snd_soc_component_get_drvdata(component);
    if (*da7213).clk_src == clk_id && (*da7213).mclk_rate == freq { return 0; }
    if freq == 0 { return 0; }
    if ((freq < (*da7213).fin_min_rate as c_uint) && freq != 32768) || freq > 54000000 {
        dev_err((*component).dev, c"Unsupported MCLK value %d\n".as_ptr(), freq);
        return -EINVAL;
    }
    match clk_id as c_uint {
        DA7213_CLKSRC_MCLK => { snd_soc_component_update_bits(component, DA7213_PLL_CTRL, DA7213_PLL_MCLK_SQR_EN, 0); }
        DA7213_CLKSRC_MCLK_SQR => { snd_soc_component_update_bits(component, DA7213_PLL_CTRL, DA7213_PLL_MCLK_SQR_EN, DA7213_PLL_MCLK_SQR_EN); }
        _ => { dev_err((*component).dev, c"Unknown clock source %d\n".as_ptr(), clk_id); return -EINVAL; }
    }
    (*da7213).clk_src = clk_id;
    if !(*da7213).mclk.is_null() {
        freq = clk_round_rate((*da7213).mclk, freq);
        let ret = clk_set_rate((*da7213).mclk, freq);
        if ret != 0 {
            dev_err((*component).dev, c"Failed to set clock rate %d\n".as_ptr(), freq);
            return ret;
        }
    }
    (*da7213).mclk_rate = freq;
    0
}

unsafe extern "C" fn _da7213_set_component_pll(component: *mut snd_soc_component, _pll_id: c_int, mut source: c_int, _fref: c_uint, mut fout: c_uint) -> c_int {
    let da7213 = snd_soc_component_get_drvdata(component);
    let indiv_bits: u8;
    let indiv: u8;
    let freq_ref: u32;
    if (*da7213).mclk_rate == 32768 {
        if !(*da7213).master {
            dev_err((*component).dev, c"32KHz only valid if codec is clock master\n".as_ptr());
            return -EINVAL;
        }
        indiv_bits = DA7213_PLL_INDIV_9_TO_18_MHZ as u8;
        indiv = DA7213_PLL_INDIV_9_TO_18_MHZ_VAL as u8;
        source = DA7213_SYSCLK_PLL_32KHZ as c_int;
        freq_ref = 3750000;
    } else {
        if (*da7213).mclk_rate < 5000000 {
            dev_err((*component).dev, c"PLL input clock %d below valid range\n".as_ptr(), (*da7213).mclk_rate);
            return -EINVAL;
        } else if (*da7213).mclk_rate <= 9000000 {
            indiv_bits = DA7213_PLL_INDIV_5_TO_9_MHZ as u8; indiv = DA7213_PLL_INDIV_5_TO_9_MHZ_VAL as u8;
        } else if (*da7213).mclk_rate <= 18000000 {
            indiv_bits = DA7213_PLL_INDIV_9_TO_18_MHZ as u8; indiv = DA7213_PLL_INDIV_9_TO_18_MHZ_VAL as u8;
        } else if (*da7213).mclk_rate <= 36000000 {
            indiv_bits = DA7213_PLL_INDIV_18_TO_36_MHZ as u8; indiv = DA7213_PLL_INDIV_18_TO_36_MHZ_VAL as u8;
        } else if (*da7213).mclk_rate <= 54000000 {
            indiv_bits = DA7213_PLL_INDIV_36_TO_54_MHZ as u8; indiv = DA7213_PLL_INDIV_36_TO_54_MHZ_VAL as u8;
        } else {
            dev_err((*component).dev, c"PLL input clock %d above valid range\n".as_ptr(), (*da7213).mclk_rate);
            return -EINVAL;
        }
        freq_ref = (*da7213).mclk_rate / indiv as c_uint;
    }
    let mut pll_ctrl = indiv_bits;
    match source as c_uint {
        DA7213_SYSCLK_MCLK => {
            snd_soc_component_update_bits(component, DA7213_PLL_CTRL, DA7213_PLL_INDIV_MASK | DA7213_PLL_MODE_MASK, pll_ctrl as c_uint);
            return 0;
        }
        DA7213_SYSCLK_PLL => {}
        DA7213_SYSCLK_PLL_SRM => { pll_ctrl |= DA7213_PLL_SRM_EN as u8; fout = DA7213_PLL_FREQ_OUT_94310400; }
        DA7213_SYSCLK_PLL_32KHZ => {
            if (*da7213).mclk_rate != 32768 {
                dev_err((*component).dev, c"32KHz mode only valid with 32KHz MCLK\n".as_ptr());
                return -EINVAL;
            }
            pll_ctrl |= (DA7213_PLL_32K_MODE | DA7213_PLL_SRM_EN) as u8;
            fout = DA7213_PLL_FREQ_OUT_94310400;
        }
        _ => { dev_err((*component).dev, c"Invalid PLL config\n".as_ptr()); return -EINVAL; }
    }
    let pll_integer: u8 = (fout / freq_ref) as u8;
    let frac_div: u64 = ((fout % freq_ref) as u64 * 8192u64) / freq_ref as u64;
    let pll_frac_top: u8 = ((frac_div >> DA7213_BYTE_SHIFT) & DA7213_BYTE_MASK as u64) as u8;
    let pll_frac_bot: u8 = (frac_div & DA7213_BYTE_MASK as u64) as u8;
    snd_soc_component_write(component, DA7213_PLL_FRAC_TOP, pll_frac_top as c_uint);
    snd_soc_component_write(component, DA7213_PLL_FRAC_BOT, pll_frac_bot as c_uint);
    snd_soc_component_write(component, DA7213_PLL_INTEGER, pll_integer as c_uint);
    pll_ctrl |= DA7213_PLL_EN as u8;
    snd_soc_component_update_bits(component, DA7213_PLL_CTRL, DA7213_PLL_INDIV_MASK | DA7213_PLL_MODE_MASK, pll_ctrl as c_uint);
    if source as c_uint == DA7213_SYSCLK_PLL_32KHZ {
        snd_soc_component_write(component, 0xF0, 0x8B);
        snd_soc_component_write(component, 0xF1, 0x03);
        snd_soc_component_write(component, 0xF1, 0x01);
        snd_soc_component_write(component, 0xF0, 0x00);
    }
    0
}

unsafe extern "C" fn da7213_set_component_pll(component: *mut snd_soc_component, pll_id: c_int, source: c_int, fref: c_uint, fout: c_uint) -> c_int {
    let da7213 = snd_soc_component_get_drvdata(component);
    (*da7213).fixed_clk_auto_pll = false;
    _da7213_set_component_pll(component, pll_id, source, fref, fout)
}

static da7213_dai_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_LEFT_J |
    SND_SOC_POSSIBLE_DAIFMT_RIGHT_J | SND_SOC_POSSIBLE_DAIFMT_DSP_A |
    SND_SOC_POSSIBLE_DAIFMT_DSP_B | SND_SOC_POSSIBLE_DAIFMT_NB_NF |
    SND_SOC_POSSIBLE_DAIFMT_NB_IF | SND_SOC_POSSIBLE_DAIFMT_IB_NF |
    SND_SOC_POSSIBLE_DAIFMT_IB_IF;

static da7213_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops! {
    hw_params: da7213_hw_params,
    set_fmt: da7213_set_dai_fmt,
    mute_stream: da7213_mute,
    no_capture_mute: 1,
    auto_selectable_formats: &da7213_dai_formats,
    num_auto_selectable_formats: 1,
};

static mut da7213_dai: snd_soc_dai_driver = snd_soc_dai_driver! {
    name: "da7213-hifi",
    playback: { stream_name: "Playback", channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_96000, formats: DA7213_FORMATS },
    capture: { stream_name: "Capture", channels_min: 1, channels_max: 2, rates: SNDRV_PCM_RATE_8000_96000, formats: DA7213_FORMATS },
    ops: &da7213_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn da7213_set_auto_pll(component: *mut snd_soc_component, enable: bool) -> c_int {
    let da7213 = snd_soc_component_get_drvdata(component);
    if !(*da7213).fixed_clk_auto_pll { return 0; }
    (*da7213).mclk_rate = clk_get_rate((*da7213).mclk);
    let mode: c_int;
    if enable {
        let mut m = if (*da7213).master { DA7213_SYSCLK_PLL } else { DA7213_SYSCLK_PLL_SRM } as c_int;
        match (*da7213).out_rate {
            DA7213_PLL_FREQ_OUT_90316800 => {
                if (*da7213).mclk_rate == 11289600 || (*da7213).mclk_rate == 22579200 || (*da7213).mclk_rate == 45158400 { m = DA7213_SYSCLK_MCLK as c_int; }
            }
            DA7213_PLL_FREQ_OUT_98304000 => {
                if (*da7213).mclk_rate == 12288000 || (*da7213).mclk_rate == 24576000 || (*da7213).mclk_rate == 49152000 { m = DA7213_SYSCLK_MCLK as c_int; }
            }
            _ => return -1,
        }
        mode = m;
    } else {
        mode = DA7213_SYSCLK_MCLK as c_int;
    }
    _da7213_set_component_pll(component, 0, mode, (*da7213).mclk_rate, (*da7213).out_rate)
}

unsafe extern "C" fn da7213_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let da7213 = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);
    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_STANDBY {
                if !(*da7213).mclk.is_null() {
                    let ret = clk_prepare_enable((*da7213).mclk);
                    if ret != 0 {
                        dev_err((*component).dev, c"Failed to enable mclk\n".as_ptr());
                        return ret;
                    }
                    da7213_set_auto_pll(component, true);
                }
            }
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                snd_soc_component_update_bits(component, DA7213_REFERENCES, DA7213_VMID_EN | DA7213_BIAS_EN, DA7213_VMID_EN | DA7213_BIAS_EN);
            } else if !(*da7213).mclk.is_null() {
                da7213_set_auto_pll(component, false);
                clk_disable_unprepare((*da7213).mclk);
            }
        }
        SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(component, DA7213_REFERENCES, DA7213_VMID_EN | DA7213_BIAS_EN, 0);
        }
        _ => {}
    }
    0
}

/* CONFIG_OF */
static da7213_of_match: &[of_device_id] = &[
    of_device_id { compatible: c"dlg,da7212".as_ptr(), data: DA7212_FIN_MIN_RATE as usize as *const c_void },
    of_device_id { compatible: c"dlg,da7213".as_ptr(), data: DA7213_FIN_MIN_RATE as usize as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
MODULE_DEVICE_TABLE!(of, da7213_of_match);

/* CONFIG_ACPI */
static da7213_acpi_match: &[acpi_device_id] = &[
    acpi_device_id { id: *b"DLGS7212\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: DA7212_FIN_MIN_RATE as usize },
    acpi_device_id { id: *b"DLGS7213\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: DA7213_FIN_MIN_RATE as usize },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];
MODULE_DEVICE_TABLE!(acpi, da7213_acpi_match);

unsafe extern "C" fn da7213_of_micbias_lvl(component: *mut snd_soc_component, val: u32) -> da7213_micbias_voltage {
    match val {
        1600 => DA7213_MICBIAS_1_6V,
        2200 => DA7213_MICBIAS_2_2V,
        2500 => DA7213_MICBIAS_2_5V,
        3000 => DA7213_MICBIAS_3_0V,
        _ => { dev_warn((*component).dev, c"Invalid micbias level\n".as_ptr()); DA7213_MICBIAS_2_2V }
    }
}

unsafe extern "C" fn da7213_of_dmic_data_sel(component: *mut snd_soc_component, str_: *const c_char) -> da7213_dmic_data_sel {
    if strcmp(str_, c"lrise_rfall".as_ptr()) == 0 {
        DA7213_DMIC_DATA_LRISE_RFALL
    } else if strcmp(str_, c"lfall_rrise".as_ptr()) == 0 {
        DA7213_DMIC_DATA_LFALL_RRISE
    } else {
        dev_warn((*component).dev, c"Invalid DMIC data select type\n".as_ptr());
        DA7213_DMIC_DATA_LRISE_RFALL
    }
}

unsafe extern "C" fn da7213_of_dmic_samplephase(component: *mut snd_soc_component, str_: *const c_char) -> da7213_dmic_samplephase {
    if strcmp(str_, c"on_clkedge".as_ptr()) == 0 {
        DA7213_DMIC_SAMPLE_ON_CLKEDGE
    } else if strcmp(str_, c"between_clkedge".as_ptr()) == 0 {
        DA7213_DMIC_SAMPLE_BETWEEN_CLKEDGE
    } else {
        dev_warn((*component).dev, c"Invalid DMIC sample phase\n".as_ptr());
        DA7213_DMIC_SAMPLE_ON_CLKEDGE
    }
}

unsafe extern "C" fn da7213_of_dmic_clkrate(component: *mut snd_soc_component, val: u32) -> da7213_dmic_clk_rate {
    match val {
        1500000 => DA7213_DMIC_CLK_1_5MHZ,
        3000000 => DA7213_DMIC_CLK_3_0MHZ,
        _ => { dev_warn((*component).dev, c"Invalid DMIC clock rate\n".as_ptr()); DA7213_DMIC_CLK_1_5MHZ }
    }
}

unsafe extern "C" fn da7213_fw_to_pdata(component: *mut snd_soc_component) -> *mut da7213_platform_data {
    let dev = (*component).dev;
    let pdata = devm_kzalloc((*component).dev, size_of::<da7213_platform_data>(), GFP_KERNEL) as *mut da7213_platform_data;
    if pdata.is_null() { return ptr::null_mut(); }
    let mut fw_str: *const c_char = ptr::null();
    let mut fw_val32: u32 = 0;
    if device_property_read_u32(dev, c"dlg,micbias1-lvl".as_ptr(), &mut fw_val32) >= 0 { (*pdata).micbias1_lvl = da7213_of_micbias_lvl(component, fw_val32); } else { (*pdata).micbias1_lvl = DA7213_MICBIAS_2_2V; }
    if device_property_read_u32(dev, c"dlg,micbias2-lvl".as_ptr(), &mut fw_val32) >= 0 { (*pdata).micbias2_lvl = da7213_of_micbias_lvl(component, fw_val32); } else { (*pdata).micbias2_lvl = DA7213_MICBIAS_2_2V; }
    if device_property_read_string(dev, c"dlg,dmic-data-sel".as_ptr(), &mut fw_str) == 0 { (*pdata).dmic_data_sel = da7213_of_dmic_data_sel(component, fw_str); } else { (*pdata).dmic_data_sel = DA7213_DMIC_DATA_LRISE_RFALL; }
    if device_property_read_string(dev, c"dlg,dmic-samplephase".as_ptr(), &mut fw_str) == 0 { (*pdata).dmic_samplephase = da7213_of_dmic_samplephase(component, fw_str); } else { (*pdata).dmic_samplephase = DA7213_DMIC_SAMPLE_ON_CLKEDGE; }
    if device_property_read_u32(dev, c"dlg,dmic-clkrate".as_ptr(), &mut fw_val32) >= 0 { (*pdata).dmic_clk_rate = da7213_of_dmic_clkrate(component, fw_val32); } else { (*pdata).dmic_clk_rate = DA7213_DMIC_CLK_3_0MHZ; }
    pdata
}

unsafe extern "C" fn da7213_probe(component: *mut snd_soc_component) -> c_int {
    let da7213 = snd_soc_component_get_drvdata(component);
    pm_runtime_get_sync((*component).dev);
    snd_soc_component_update_bits(component, DA7213_ALC_CTRL1, DA7213_ALC_CALIB_MODE_MAN, 0);
    (*da7213).alc_calib_auto = true;
    snd_soc_component_update_bits(component, DA7213_PC_COUNT, DA7213_PC_FREERUN_MASK, DA7213_PC_FREERUN_MASK);
    for reg in [DA7213_AUX_L_CTRL, DA7213_AUX_R_CTRL, DA7213_MIXIN_L_CTRL, DA7213_MIXIN_R_CTRL, DA7213_ADC_L_CTRL, DA7213_ADC_R_CTRL, DA7213_DAC_L_CTRL, DA7213_DAC_R_CTRL, DA7213_HP_L_CTRL, DA7213_HP_R_CTRL, DA7213_LINE_CTRL] {
        snd_soc_component_update_bits(component, reg, DA7213_GAIN_RAMP_EN, DA7213_GAIN_RAMP_EN);
    }
    snd_soc_component_update_bits(component, DA7213_MIXIN_L_CTRL, DA7213_MIXIN_MIX_EN, DA7213_MIXIN_MIX_EN);
    snd_soc_component_update_bits(component, DA7213_MIXIN_R_CTRL, DA7213_MIXIN_MIX_EN, DA7213_MIXIN_MIX_EN);
    snd_soc_component_update_bits(component, DA7213_MIXOUT_L_CTRL, DA7213_MIXOUT_MIX_EN, DA7213_MIXOUT_MIX_EN);
    snd_soc_component_update_bits(component, DA7213_MIXOUT_R_CTRL, DA7213_MIXOUT_MIX_EN, DA7213_MIXOUT_MIX_EN);
    snd_soc_component_update_bits(component, DA7213_HP_L_CTRL, DA7213_HP_AMP_OE, DA7213_HP_AMP_OE);
    snd_soc_component_update_bits(component, DA7213_HP_R_CTRL, DA7213_HP_AMP_OE, DA7213_HP_AMP_OE);
    snd_soc_component_update_bits(component, DA7213_LINE_CTRL, DA7213_LINE_AMP_OE, DA7213_LINE_AMP_OE);
    (*da7213).pdata = dev_get_platdata((*component).dev);
    if (*da7213).pdata.is_null() { (*da7213).pdata = da7213_fw_to_pdata(component); }
    if !(*da7213).pdata.is_null() {
        let pdata = (*da7213).pdata;
        let mut micbias_lvl: u8 = 0;
        let mut dmic_cfg: u8 = 0;
        match (*pdata).micbias1_lvl { DA7213_MICBIAS_1_6V | DA7213_MICBIAS_2_2V | DA7213_MICBIAS_2_5V | DA7213_MICBIAS_3_0V => micbias_lvl |= ((*pdata).micbias1_lvl << DA7213_MICBIAS1_LEVEL_SHIFT) as u8, _ => {} }
        match (*pdata).micbias2_lvl { DA7213_MICBIAS_1_6V | DA7213_MICBIAS_2_2V | DA7213_MICBIAS_2_5V | DA7213_MICBIAS_3_0V => micbias_lvl |= ((*pdata).micbias2_lvl << DA7213_MICBIAS2_LEVEL_SHIFT) as u8, _ => {} }
        snd_soc_component_update_bits(component, DA7213_MICBIAS_CTRL, DA7213_MICBIAS1_LEVEL_MASK | DA7213_MICBIAS2_LEVEL_MASK, micbias_lvl as c_uint);
        match (*pdata).dmic_data_sel { DA7213_DMIC_DATA_LFALL_RRISE | DA7213_DMIC_DATA_LRISE_RFALL => dmic_cfg |= ((*pdata).dmic_data_sel << DA7213_DMIC_DATA_SEL_SHIFT) as u8, _ => {} }
        match (*pdata).dmic_samplephase { DA7213_DMIC_SAMPLE_ON_CLKEDGE | DA7213_DMIC_SAMPLE_BETWEEN_CLKEDGE => dmic_cfg |= ((*pdata).dmic_samplephase << DA7213_DMIC_SAMPLEPHASE_SHIFT) as u8, _ => {} }
        match (*pdata).dmic_clk_rate { DA7213_DMIC_CLK_3_0MHZ | DA7213_DMIC_CLK_1_5MHZ => dmic_cfg |= ((*pdata).dmic_clk_rate << DA7213_DMIC_CLK_RATE_SHIFT) as u8, _ => {} }
        snd_soc_component_update_bits(component, DA7213_MIC_CONFIG, DA7213_DMIC_DATA_SEL_MASK | DA7213_DMIC_SAMPLEPHASE_MASK | DA7213_DMIC_CLK_RATE_MASK, dmic_cfg as c_uint);
    }
    pm_runtime_put_sync((*component).dev);
    (*da7213).mclk = devm_clk_get_optional((*component).dev, c"mclk".as_ptr());
    if IS_ERR((*da7213).mclk as *const c_void) { return PTR_ERR((*da7213).mclk as *const c_void); }
    if !(*da7213).mclk.is_null() { (*da7213).fixed_clk_auto_pll = true; }
    snd_soc_component_write(component, DA7213_TONE_GEN_CYCLES, DA7213_BEEP_CYCLES_MASK);
    0
}

unsafe extern "C" fn da7213_runtime_suspend(dev: *mut device) -> c_int {
    let da7213 = dev_get_drvdata(dev);
    regcache_cache_only((*da7213).regmap, true);
    regcache_mark_dirty((*da7213).regmap);
    regulator_bulk_disable(DA7213_NUM_SUPPLIES, (*da7213).supplies.as_mut_ptr());
    0
}

unsafe extern "C" fn da7213_runtime_resume(dev: *mut device) -> c_int {
    let da7213 = dev_get_drvdata(dev);
    let ret = regulator_bulk_enable(DA7213_NUM_SUPPLIES, (*da7213).supplies.as_mut_ptr());
    if ret < 0 { return ret; }
    regcache_cache_only((*da7213).regmap, false);
    regcache_sync((*da7213).regmap)
}

unsafe extern "C" fn da7213_suspend(component: *mut snd_soc_component) -> c_int {
    let da7213 = snd_soc_component_get_drvdata(component);
    da7213_runtime_suspend((*da7213).dev)
}

unsafe extern "C" fn da7213_resume(component: *mut snd_soc_component) -> c_int {
    let da7213 = snd_soc_component_get_drvdata(component);
    da7213_runtime_resume((*da7213).dev)
}

static soc_component_dev_da7213: snd_soc_component_driver = snd_soc_component_driver! {
    probe: da7213_probe,
    set_bias_level: da7213_set_bias_level,
    controls: da7213_snd_controls,
    num_controls: ARRAY_SIZE!(da7213_snd_controls),
    suspend: da7213_suspend,
    resume: da7213_resume,
    dapm_widgets: da7213_dapm_widgets,
    num_dapm_widgets: ARRAY_SIZE!(da7213_dapm_widgets),
    dapm_routes: da7213_audio_map,
    num_dapm_routes: ARRAY_SIZE!(da7213_audio_map),
    set_sysclk: da7213_set_component_sysclk,
    set_pll: da7213_set_component_pll,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static da7213_regmap_config: regmap_config = regmap_config! {
    reg_bits: 8,
    val_bits: 8,
    max_register: DA7213_TONE_GEN_OFF_PER,
    reg_defaults: da7213_reg_defaults,
    num_reg_defaults: ARRAY_SIZE!(da7213_reg_defaults),
    volatile_reg: da7213_volatile_register,
    cache_type: REGCACHE_RBTREE,
};

unsafe extern "C" fn da7213_power_off(data: *mut c_void) {
    let da7213 = data as *mut da7213_priv;
    regulator_bulk_disable(DA7213_NUM_SUPPLIES, (*da7213).supplies.as_mut_ptr());
}

static da7213_supply_names: [*const c_char; DA7213_NUM_SUPPLIES as usize] = [
    [DA7213_SUPPLY_VDDA as usize] = c"VDDA".as_ptr(),
    [DA7213_SUPPLY_VDDIO as usize] = c"VDDIO".as_ptr(),
];

unsafe extern "C" fn da7213_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let da7213 = devm_kzalloc(&mut (*i2c).dev, size_of::<da7213_priv>(), GFP_KERNEL) as *mut da7213_priv;
    if da7213.is_null() { return -ENOMEM; }
    (*da7213).fin_min_rate = i2c_get_match_data(i2c) as usize;
    if (*da7213).fin_min_rate == 0 { return -EINVAL; }
    (*da7213).dev = &mut (*i2c).dev;
    i2c_set_clientdata(i2c, da7213 as *mut c_void);
    let mut i: c_int = 0;
    while i < DA7213_NUM_SUPPLIES {
        (*da7213).supplies[i as usize].supply = da7213_supply_names[i as usize];
        i += 1;
    }
    let mut ret = devm_regulator_bulk_get(&mut (*i2c).dev, DA7213_NUM_SUPPLIES, (*da7213).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*i2c).dev, c"Failed to get supplies: %d\n".as_ptr(), ret);
        return ret;
    }
    ret = regulator_bulk_enable(DA7213_NUM_SUPPLIES, (*da7213).supplies.as_mut_ptr());
    if ret < 0 { return ret; }
    ret = devm_add_action_or_reset(&mut (*i2c).dev, Some(da7213_power_off), da7213 as *mut c_void);
    if ret < 0 { return ret; }
    (*da7213).regmap = devm_regmap_init_i2c(i2c, &da7213_regmap_config);
    if IS_ERR((*da7213).regmap as *const c_void) {
        ret = PTR_ERR((*da7213).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, c"regmap_init() failed: %d\n".as_ptr(), ret);
        return ret;
    }
    mutex_init(core::ptr::addr_of_mut!((*da7213).ctrl_lock));
    pm_runtime_set_autosuspend_delay(&mut (*i2c).dev, 100);
    pm_runtime_use_autosuspend(&mut (*i2c).dev);
    pm_runtime_set_active(&mut (*i2c).dev);
    pm_runtime_enable(&mut (*i2c).dev);
    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_da7213, &mut da7213_dai, 1);
    if ret < 0 {
        dev_err(&mut (*i2c).dev, c"Failed to register da7213 component: %d\n".as_ptr(), ret);
    }
    ret
}

unsafe extern "C" fn da7213_i2c_remove(i2c: *mut i2c_client) {
    pm_runtime_disable(&mut (*i2c).dev);
}

static da7213_pm: dev_pm_ops = dev_pm_ops! {
    RUNTIME_PM_OPS(da7213_runtime_suspend, da7213_runtime_resume, NULL)
};

static da7213_i2c_id: &[i2c_device_id] = &[
    i2c_device_id { name: *b"da7213\0\0\0\0\0\0\0\0\0\0\0\0\0\0" as [u8; 20] as [c_char; 20], driver_data: 0 },
    i2c_device_id { name: [0; 20], driver_data: 0 },
];
MODULE_DEVICE_TABLE!(i2c, da7213_i2c_id);

static mut da7213_i2c_driver: i2c_driver = i2c_driver! {
    driver: {
        name: "da7213",
        of_match_table: of_match_ptr!(da7213_of_match),
        acpi_match_table: ACPI_PTR!(da7213_acpi_match),
        pm: pm_ptr!(&da7213_pm),
    },
    probe: da7213_i2c_probe,
    remove: da7213_i2c_remove,
    id_table: da7213_i2c_id,
};

module_i2c_driver!(da7213_i2c_driver);

MODULE_DESCRIPTION!("ASoC DA7213 Codec driver");
MODULE_AUTHOR!("Adam Thomson <Adam.Thomson.Opensource@diasemi.com>");
MODULE_AUTHOR!("David Rau <David.Rau.opensource@dm.renesas.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
