// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * da7219.rs - DA7219 ALSA SoC Codec Driver
 *
 * Rust source-level translation of da7219.c.
 *
 * Copyright (c) 2015 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type bool_ = bool;
type __le16 = u16;

extern "C" {
    static mut da7219_of_match: [of_device_id; 0];
    static mut da7219_acpi_match: [acpi_device_id; 0];

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut da7219_priv;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
    fn regmap_raw_read(map: *mut regmap, reg: c_uint, val: *mut c_void, len: usize) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: c_uint, val: *const c_void, len: usize) -> c_int;
    fn regmap_bulk_write(map: *mut regmap, reg: c_uint, val: *const c_void, len: usize) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_write_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_bypass(map: *mut regmap, enable: bool_);
    fn regmap_reinit_cache(map: *mut regmap, config: *const regmap_config) -> c_int;
    fn regmap_register_patch(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_round_rate(clk: *mut clk, rate: c_ulong) -> c_ulong;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn __clk_get_name(clk: *mut clk) -> *const c_char;
    fn clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> c_int;
    fn clk_hw_unregister(hw: *mut clk_hw);
    fn clkdev_hw_create(hw: *mut clk_hw, con_id: *const c_char, dev_fmt: *const c_char, ...) -> *mut clk_lookup;
    fn clkdev_drop(cl: *mut clk_lookup);
    fn of_clk_add_hw_provider(np: *mut device_node, get: *const c_void, data: *mut c_void) -> c_int;
    fn of_clk_del_provider(np: *mut device_node);
    static of_clk_hw_onecell_get: c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc_flex_onecell(num: usize) -> *mut clk_hw_onecell_data;
    fn kfree(ptr: *mut c_void);
    fn device_property_read_bool(dev: *mut device, propname: *const c_char) -> bool_;
    fn device_property_read_string_array(dev: *mut device, propname: *const c_char, val: *mut *const c_char, nval: usize) -> c_int;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut u32) -> c_int;
    fn device_property_read_string(dev: *mut device, propname: *const c_char, val: *mut *const c_char) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn regulator_bulk_get(dev: *mut device, num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_free(num_consumers: c_int, consumers: *mut regulator_bulk_data);
    fn regulator_get_voltage(regulator: *mut regulator) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn msleep(msecs: c_uint);
    fn ffs(x: c_int) -> c_int;
    fn fls(x: c_int) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_ulong;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_name(dev: *mut device) -> *const c_char;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *mut da7219_pdata;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn da7219_aad_init(component: *mut snd_soc_component) -> c_int;
    fn da7219_aad_exit(component: *mut snd_soc_component);
    fn da7219_aad_suspend(component: *mut snd_soc_component);
    fn da7219_aad_resume(component: *mut snd_soc_component);
    fn da7219_aad_jack_det(component: *mut snd_soc_component, jack: *mut snd_soc_jack);
    fn da7219_aad_probe(i2c: *mut i2c_client) -> c_int;
}

#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { _unused: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: usize }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub struct snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dapm_context { _unused: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context, pub reg: c_uint }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub dev: *mut device }
#[repr(C)] pub struct snd_pcm_substream { _unused: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _unused: [u8; 0] }
#[repr(C)] pub struct snd_soc_jack { _unused: [u8; 0] }
#[repr(C)] pub struct regmap { _unused: [u8; 0] }
#[repr(C)] pub struct clk { _unused: [u8; 0] }
#[repr(C)] pub struct clk_lookup { _unused: [u8; 0] }
#[repr(C)] pub struct regulator { _unused: [u8; 0] }
#[repr(C)] pub struct mutex { _unused: [u8; 0] }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char }
#[repr(C)] pub struct acpi_device_id { pub id: [c_char; 16] }
#[repr(C)] pub struct i2c_device_id { pub name: [c_char; 20] }
#[repr(C)] pub struct i2c_client { pub dev: device }
#[repr(C)] pub struct soc_mixer_control { pub reg: c_uint }
#[repr(C)] pub struct soc_enum { _unused: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { _unused: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _unused: [u8; 0] }
type snd_soc_dapm_widget_item = snd_soc_dapm_widget_desc;
#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data, pub clk: *mut clk }
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub ops: *const clk_ops, pub flags: c_uint, pub parent_names: *const *const c_char, pub num_parents: u8 }
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong, pub best_parent_rate: c_ulong }
#[repr(C)] pub struct clk_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_prepared: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
}
#[repr(C)] pub struct clk_hw_onecell_data { pub num: c_uint, pub hws: [*mut clk_hw; DA7219_DAI_NUM_CLKS as usize] }
#[repr(C)] pub struct regulator_bulk_data { pub supply: *const c_char, pub consumer: *mut regulator }
#[repr(C)] pub struct reg_default { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct reg_sequence { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_uint,
}
#[repr(C)] pub struct snd_soc_pcm_stream { pub stream_name: *const c_char, pub channels_min: c_uint, pub channels_max: c_uint, pub rates: c_uint, pub formats: u64 }
#[repr(C)] pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
}
#[repr(C)] pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
    pub symmetric_channels: c_uint,
    pub symmetric_sample_bits: c_uint,
}
#[repr(C)] pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_item,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)] pub struct da7219_pdata {
    pub wakeup_source: bool_,
    pub dai_clk_names: [*const c_char; DA7219_DAI_NUM_CLKS as usize],
    pub micbias_lvl: da7219_micbias_voltage,
    pub mic_amp_in_sel: da7219_mic_amp_in_sel,
}
#[repr(C)] pub struct da7219_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub pdata: *mut da7219_pdata,
    pub ctrl_lock: mutex,
    pub pll_lock: mutex,
    pub alc_en: bool_,
    pub micbias_on_event: bool_,
    pub mic_pga_delay: c_uint,
    pub gain_ramp_ctrl: u8,
    pub master: bool_,
    pub clk_src: c_int,
    pub mclk_rate: c_uint,
    pub mclk: *mut clk,
    pub dai_clks: [*mut clk; DA7219_DAI_NUM_CLKS as usize],
    pub dai_clks_hw: [clk_hw; DA7219_DAI_NUM_CLKS as usize],
    pub dai_clks_lookup: [*mut clk_lookup; DA7219_DAI_NUM_CLKS as usize],
    pub clk_hw_data: *mut clk_hw_onecell_data,
    pub supplies: [regulator_bulk_data; DA7219_NUM_SUPPLIES as usize],
    pub wakeup_source: bool_,
    pub tdm_en: bool_,
}

type snd_soc_bias_level = c_uint;
type da7219_micbias_voltage = c_uint;
type da7219_mic_amp_in_sel = c_uint;

const NULL: *const c_void = ptr::null();
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 1;
const CLK_GET_RATE_NOCACHE: c_uint = 1 << 0;
const CLK_SET_RATE_GATE: c_uint = 1 << 1;

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
macro_rules! ARRAY_SIZE { ($a:expr) => { ($a.len() as c_uint) }; }
macro_rules! DECLARE_TLV_DB_SCALE { ($name:ident, $min:expr, $step:expr, $mute:expr) => { static $name: [c_uint; 4] = [$min as c_uint, $step as c_uint, $mute as c_uint, 0]; }; }
macro_rules! DECLARE_TLV_DB_RANGE { ($name:ident, $($tt:tt)*) => { static $name: [c_uint; 1] = [0]; }; }
macro_rules! TLV_DB_SCALE_ITEM { ($($tt:tt)*) => { 0 }; }
macro_rules! opaque_control { ($($tt:tt)*) => { snd_kcontrol_new { _unused: [] } }; }
macro_rules! opaque_widget { ($($tt:tt)*) => { snd_soc_dapm_widget_desc { _unused: [] } }; }

macro_rules! SOC_SINGLE_TLV { ($($tt:tt)*) => { opaque_control!($($tt)*) }; }
macro_rules! SOC_SINGLE { ($($tt:tt)*) => { opaque_control!($($tt)*) }; }
macro_rules! SOC_SINGLE_EXT_TLV { ($($tt:tt)*) => { opaque_control!($($tt)*) }; }
macro_rules! SOC_SINGLE_RANGE_TLV { ($($tt:tt)*) => { opaque_control!($($tt)*) }; }
macro_rules! SOC_ENUM { ($($tt:tt)*) => { opaque_control!($($tt)*) }; }
macro_rules! SOC_SINGLE_EXT { ($($tt:tt)*) => { opaque_control!($($tt)*) }; }
macro_rules! SOC_ENUM_EXT { ($($tt:tt)*) => { opaque_control!($($tt)*) }; }
macro_rules! SOC_DOUBLE_R_EXT_TLV { ($($tt:tt)*) => { opaque_control!($($tt)*) }; }
macro_rules! SOC_DOUBLE_R_EXT { ($($tt:tt)*) => { opaque_control!($($tt)*) }; }
macro_rules! SOC_DOUBLE_R { ($($tt:tt)*) => { opaque_control!($($tt)*) }; }
macro_rules! SOC_DAPM_SINGLE { ($($tt:tt)*) => { opaque_control!($($tt)*) }; }
macro_rules! SOC_DAPM_ENUM { ($($tt:tt)*) => { opaque_control!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_SUPPLY { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_INPUT { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_PGA_E { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_ADC { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_SIGGEN { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_PGA { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_MIXER { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_MUX { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_AIF_OUT { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_AIF_IN { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_DAC_E { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_SUPPLY_S { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_OUTPUT { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_PRE { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }
macro_rules! SND_SOC_DAPM_POST { ($($tt:tt)*) => { opaque_widget!($($tt)*) }; }

// Constants normally supplied by <sound/da7219.h>, "da7219.h", and Linux/ASoC headers.
extern "C" {
    static DA7219_MIC_1_CTRL: c_uint; static DA7219_MIXIN_L_CTRL: c_uint; static DA7219_ADC_L_CTRL: c_uint;
    static DA7219_ALC_CTRL1: c_uint; static DA7219_TONE_GEN_FREQ1_L: c_uint; static DA7219_TONE_GEN_FREQ2_L: c_uint;
}

DECLARE_TLV_DB_SCALE!(da7219_mic_gain_tlv, -600, 600, 0);
DECLARE_TLV_DB_SCALE!(da7219_mixin_gain_tlv, -450, 150, 0);
DECLARE_TLV_DB_SCALE!(da7219_adc_dig_gain_tlv, -8325, 75, 0);
DECLARE_TLV_DB_SCALE!(da7219_alc_threshold_tlv, -9450, 150, 0);
DECLARE_TLV_DB_SCALE!(da7219_alc_gain_tlv, 0, 600, 0);
DECLARE_TLV_DB_SCALE!(da7219_alc_ana_gain_tlv, 0, 600, 0);
DECLARE_TLV_DB_SCALE!(da7219_sidetone_gain_tlv, -4200, 300, 0);
DECLARE_TLV_DB_SCALE!(da7219_tonegen_gain_tlv, -4500, 300, 0);
DECLARE_TLV_DB_SCALE!(da7219_dac_eq_band_tlv, -1050, 150, 0);
DECLARE_TLV_DB_RANGE!(da7219_dac_dig_gain_tlv,
    0x0, 0x07, TLV_DB_SCALE_ITEM!(TLV_DB_GAIN_MUTE, 0, 1),
    0x08, 0x7f, TLV_DB_SCALE_ITEM!(-7725, 75, 0)
);
DECLARE_TLV_DB_SCALE!(da7219_dac_ng_threshold_tlv, -10200, 600, 0);
DECLARE_TLV_DB_SCALE!(da7219_hp_gain_tlv, -5700, 100, 0);

static da7219_alc_attack_rate_txt: [*const c_char; 13] = [
    cstr!("7.33/fs"), cstr!("14.66/fs"), cstr!("29.32/fs"), cstr!("58.64/fs"), cstr!("117.3/fs"), cstr!("234.6/fs"),
    cstr!("469.1/fs"), cstr!("938.2/fs"), cstr!("1876/fs"), cstr!("3753/fs"), cstr!("7506/fs"), cstr!("15012/fs"),
    cstr!("30024/fs"),
];
static da7219_alc_release_rate_txt: [*const c_char; 11] = [
    cstr!("28.66/fs"), cstr!("57.33/fs"), cstr!("114.6/fs"), cstr!("229.3/fs"), cstr!("458.6/fs"), cstr!("917.1/fs"),
    cstr!("1834/fs"), cstr!("3668/fs"), cstr!("7337/fs"), cstr!("14674/fs"), cstr!("29348/fs"),
];
static da7219_alc_hold_time_txt: [*const c_char; 16] = [
    cstr!("62/fs"), cstr!("124/fs"), cstr!("248/fs"), cstr!("496/fs"), cstr!("992/fs"), cstr!("1984/fs"), cstr!("3968/fs"),
    cstr!("7936/fs"), cstr!("15872/fs"), cstr!("31744/fs"), cstr!("63488/fs"), cstr!("126976/fs"),
    cstr!("253952/fs"), cstr!("507904/fs"), cstr!("1015808/fs"), cstr!("2031616/fs"),
];
static da7219_alc_env_rate_txt: [*const c_char; 4] = [cstr!("1/4"), cstr!("1/16"), cstr!("1/256"), cstr!("1/65536")];
static da7219_alc_anticlip_step_txt: [*const c_char; 4] = [cstr!("0.034dB/fs"), cstr!("0.068dB/fs"), cstr!("0.136dB/fs"), cstr!("0.272dB/fs")];
static da7219_gain_ramp_rate_txt: [*const c_char; 4] = [cstr!("Nominal Rate * 8"), cstr!("Nominal Rate"), cstr!("Nominal Rate / 8"), cstr!("Nominal Rate / 16")];
static da7219_hpf_mode_txt: [*const c_char; 3] = [cstr!("Disabled"), cstr!("Audio"), cstr!("Voice")];
static da7219_hpf_mode_val: [c_uint; 3] = [DA7219_HPF_DISABLED, DA7219_HPF_AUDIO_EN, DA7219_HPF_VOICE_EN];
static da7219_audio_hpf_corner_txt: [*const c_char; 4] = [cstr!("2Hz"), cstr!("4Hz"), cstr!("8Hz"), cstr!("16Hz")];
static da7219_voice_hpf_corner_txt: [*const c_char; 8] = [cstr!("2.5Hz"), cstr!("25Hz"), cstr!("50Hz"), cstr!("100Hz"), cstr!("150Hz"), cstr!("200Hz"), cstr!("300Hz"), cstr!("400Hz")];
static da7219_tonegen_dtmf_key_txt: [*const c_char; 16] = [cstr!("0"), cstr!("1"), cstr!("2"), cstr!("3"), cstr!("4"), cstr!("5"), cstr!("6"), cstr!("7"), cstr!("8"), cstr!("9"), cstr!("A"), cstr!("B"), cstr!("C"), cstr!("D"), cstr!("*"), cstr!("#")];
static da7219_tonegen_swg_sel_txt: [*const c_char; 4] = [cstr!("Sum"), cstr!("SWG1"), cstr!("SWG2"), cstr!("SWG1_1-Cos")];
static da7219_dac_softmute_rate_txt: [*const c_char; 7] = [cstr!("1 Sample"), cstr!("2 Samples"), cstr!("4 Samples"), cstr!("8 Samples"), cstr!("16 Samples"), cstr!("32 Samples"), cstr!("64 Samples")];
static da7219_dac_ng_setup_time_txt: [*const c_char; 4] = [cstr!("256 Samples"), cstr!("512 Samples"), cstr!("1024 Samples"), cstr!("2048 Samples")];
static da7219_dac_ng_rampup_txt: [*const c_char; 2] = [cstr!("0.22ms/dB"), cstr!("0.0138ms/dB")];
static da7219_dac_ng_rampdown_txt: [*const c_char; 2] = [cstr!("0.88ms/dB"), cstr!("14.08ms/dB")];
static da7219_cp_track_mode_txt: [*const c_char; 3] = [cstr!("Largest Volume"), cstr!("DAC Volume"), cstr!("Signal Magnitude")];
static da7219_cp_track_mode_val: [c_uint; 3] = [DA7219_CP_MCHANGE_LARGEST_VOL, DA7219_CP_MCHANGE_DAC_VOL, DA7219_CP_MCHANGE_SIG_MAG];

static da7219_alc_attack_rate: soc_enum = soc_enum { _unused: [] };
static da7219_alc_release_rate: soc_enum = soc_enum { _unused: [] };
static da7219_alc_hold_time: soc_enum = soc_enum { _unused: [] };
static da7219_alc_env_attack_rate: soc_enum = soc_enum { _unused: [] };
static da7219_alc_env_release_rate: soc_enum = soc_enum { _unused: [] };
static da7219_alc_anticlip_step: soc_enum = soc_enum { _unused: [] };
static da7219_gain_ramp_rate: soc_enum = soc_enum { _unused: [] };
static da7219_adc_hpf_mode: soc_enum = soc_enum { _unused: [] };
static da7219_dac_hpf_mode: soc_enum = soc_enum { _unused: [] };
static da7219_adc_audio_hpf_corner: soc_enum = soc_enum { _unused: [] };
static da7219_dac_audio_hpf_corner: soc_enum = soc_enum { _unused: [] };
static da7219_adc_voice_hpf_corner: soc_enum = soc_enum { _unused: [] };
static da7219_dac_voice_hpf_corner: soc_enum = soc_enum { _unused: [] };
static da7219_tonegen_dtmf_key: soc_enum = soc_enum { _unused: [] };
static da7219_tonegen_swg_sel: soc_enum = soc_enum { _unused: [] };
static da7219_dac_softmute_rate: soc_enum = soc_enum { _unused: [] };
static da7219_dac_ng_setup_time: soc_enum = soc_enum { _unused: [] };
static da7219_dac_ng_rampup_rate: soc_enum = soc_enum { _unused: [] };
static da7219_dac_ng_rampdown_rate: soc_enum = soc_enum { _unused: [] };
static da7219_cp_track_mode: soc_enum = soc_enum { _unused: [] };

unsafe extern "C" fn da7219_volsw_locked_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7219 = snd_soc_component_get_drvdata(component);
    mutex_lock(&mut (*da7219).ctrl_lock);
    let ret = snd_soc_get_volsw(kcontrol, ucontrol);
    mutex_unlock(&mut (*da7219).ctrl_lock);
    ret
}

unsafe extern "C" fn da7219_volsw_locked_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7219 = snd_soc_component_get_drvdata(component);
    mutex_lock(&mut (*da7219).ctrl_lock);
    let ret = snd_soc_put_volsw(kcontrol, ucontrol);
    mutex_unlock(&mut (*da7219).ctrl_lock);
    ret
}

unsafe extern "C" fn da7219_enum_locked_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7219 = snd_soc_component_get_drvdata(component);
    mutex_lock(&mut (*da7219).ctrl_lock);
    let ret = snd_soc_get_enum_double(kcontrol, ucontrol);
    mutex_unlock(&mut (*da7219).ctrl_lock);
    ret
}

unsafe extern "C" fn da7219_enum_locked_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7219 = snd_soc_component_get_drvdata(component);
    mutex_lock(&mut (*da7219).ctrl_lock);
    let ret = snd_soc_put_enum_double(kcontrol, ucontrol);
    mutex_unlock(&mut (*da7219).ctrl_lock);
    ret
}

unsafe extern "C" fn da7219_alc_calib(component: *mut snd_soc_component) {
    let mic_ctrl = snd_soc_component_read(component, DA7219_MIC_1_CTRL) as u8;
    let mixin_ctrl = snd_soc_component_read(component, DA7219_MIXIN_L_CTRL) as u8;
    let adc_ctrl = snd_soc_component_read(component, DA7219_ADC_L_CTRL) as u8;

    snd_soc_component_update_bits(component, DA7219_MIC_1_CTRL, DA7219_MIC_1_AMP_EN_MASK, DA7219_MIC_1_AMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_MIC_1_CTRL, DA7219_MIC_1_AMP_MUTE_EN_MASK, DA7219_MIC_1_AMP_MUTE_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_MIXIN_L_CTRL, DA7219_MIXIN_L_AMP_EN_MASK | DA7219_MIXIN_L_AMP_MUTE_EN_MASK, DA7219_MIXIN_L_AMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_ADC_L_CTRL, DA7219_ADC_L_MUTE_EN_MASK | DA7219_ADC_L_EN_MASK, DA7219_ADC_L_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_ALC_CTRL1, DA7219_ALC_AUTO_CALIB_EN_MASK, DA7219_ALC_AUTO_CALIB_EN_MASK);

    let mut calib_ctrl: u8;
    loop {
        calib_ctrl = snd_soc_component_read(component, DA7219_ALC_CTRL1) as u8;
        if (calib_ctrl as c_uint & DA7219_ALC_AUTO_CALIB_EN_MASK) == 0 { break; }
    }

    if (calib_ctrl as c_uint & DA7219_ALC_CALIB_OVERFLOW_MASK) != 0 {
        dev_warn((*component).dev, cstr!("ALC auto calibration failed with overflow\n"));
        snd_soc_component_update_bits(component, DA7219_ALC_CTRL1, DA7219_ALC_OFFSET_EN_MASK | DA7219_ALC_SYNC_MODE_MASK, 0);
    } else {
        snd_soc_component_update_bits(component, DA7219_ALC_CTRL1, DA7219_ALC_OFFSET_EN_MASK | DA7219_ALC_SYNC_MODE_MASK, DA7219_ALC_OFFSET_EN_MASK | DA7219_ALC_SYNC_MODE_MASK);
    }

    snd_soc_component_write(component, DA7219_ADC_L_CTRL, adc_ctrl as c_uint);
    snd_soc_component_write(component, DA7219_MIXIN_L_CTRL, mixin_ctrl as c_uint);
    snd_soc_component_write(component, DA7219_MIC_1_CTRL, mic_ctrl as c_uint);
}

unsafe extern "C" fn da7219_mixin_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7219 = snd_soc_component_get_drvdata(component);
    let ret = snd_soc_put_volsw(kcontrol, ucontrol);
    if ret == 1 && (*da7219).alc_en { da7219_alc_calib(component); }
    ret
}

unsafe extern "C" fn da7219_alc_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7219 = snd_soc_component_get_drvdata(component);
    if (*ucontrol).value.integer.value[0] != 0 && !(*da7219).alc_en {
        da7219_alc_calib(component);
        (*da7219).alc_en = true;
    } else {
        (*da7219).alc_en = false;
    }
    snd_soc_put_volsw(kcontrol, ucontrol)
}

unsafe extern "C" fn da7219_tonegen_freq_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7219 = snd_soc_component_get_drvdata(component);
    let mixer_ctrl = (*kcontrol).private_value as *mut soc_mixer_control;
    let reg = (*mixer_ctrl).reg;
    let mut val: __le16 = 0;
    mutex_lock(&mut (*da7219).ctrl_lock);
    let ret = regmap_raw_read((*da7219).regmap, reg, &mut val as *mut _ as *mut c_void, size_of::<__le16>());
    mutex_unlock(&mut (*da7219).ctrl_lock);
    if ret != 0 { return ret; }
    (*ucontrol).value.integer.value[0] = u16::from_le(val) as i64;
    0
}

unsafe extern "C" fn da7219_tonegen_freq_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let da7219 = snd_soc_component_get_drvdata(component);
    let mixer_ctrl = (*kcontrol).private_value as *mut soc_mixer_control;
    let reg = (*mixer_ctrl).reg;
    let val_new: __le16 = ((*ucontrol).value.integer.value[0] as u16).to_le();
    let mut val_old: __le16 = 0;

    mutex_lock(&mut (*da7219).ctrl_lock);
    let mut ret = regmap_raw_read((*da7219).regmap, reg, &mut val_old as *mut _ as *mut c_void, size_of::<__le16>());
    if ret == 0 && val_old != val_new {
        ret = regmap_raw_write((*da7219).regmap, reg, &val_new as *const _ as *const c_void, size_of::<__le16>());
    }
    mutex_unlock(&mut (*da7219).ctrl_lock);

    if ret < 0 { return ret; }
    (val_old != val_new) as c_int
}

static da7219_snd_controls: [snd_kcontrol_new; 78] = [
    SOC_SINGLE_TLV!("Mic Volume", DA7219_MIC_1_GAIN, DA7219_MIC_1_AMP_GAIN_SHIFT, DA7219_MIC_1_AMP_GAIN_MAX, DA7219_NO_INVERT, da7219_mic_gain_tlv),
    SOC_SINGLE!("Mic Switch", DA7219_MIC_1_CTRL, DA7219_MIC_1_AMP_MUTE_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_INVERT),
    SOC_SINGLE_EXT_TLV!("Mixin Volume", DA7219_MIXIN_L_GAIN, DA7219_MIXIN_L_AMP_GAIN_SHIFT, DA7219_MIXIN_L_AMP_GAIN_MAX, DA7219_NO_INVERT, snd_soc_get_volsw, da7219_mixin_gain_put, da7219_mixin_gain_tlv),
    SOC_SINGLE!("Mixin Switch", DA7219_MIXIN_L_CTRL, DA7219_MIXIN_L_AMP_MUTE_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_INVERT),
    SOC_SINGLE!("Mixin Gain Ramp Switch", DA7219_MIXIN_L_CTRL, DA7219_MIXIN_L_AMP_RAMP_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT),
    SOC_SINGLE!("Mixin ZC Gain Switch", DA7219_MIXIN_L_CTRL, DA7219_MIXIN_L_AMP_ZC_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT),
    SOC_SINGLE_TLV!("Capture Digital Volume", DA7219_ADC_L_GAIN, DA7219_ADC_L_DIGITAL_GAIN_SHIFT, DA7219_ADC_L_DIGITAL_GAIN_MAX, DA7219_NO_INVERT, da7219_adc_dig_gain_tlv),
    SOC_SINGLE!("Capture Digital Switch", DA7219_ADC_L_CTRL, DA7219_ADC_L_MUTE_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_INVERT),
    SOC_SINGLE!("Capture Digital Gain Ramp Switch", DA7219_ADC_L_CTRL, DA7219_ADC_L_RAMP_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT),
    SOC_ENUM!("ALC Attack Rate", da7219_alc_attack_rate), SOC_ENUM!("ALC Release Rate", da7219_alc_release_rate),
    SOC_ENUM!("ALC Hold Time", da7219_alc_hold_time), SOC_ENUM!("ALC Envelope Attack Rate", da7219_alc_env_attack_rate),
    SOC_ENUM!("ALC Envelope Release Rate", da7219_alc_env_release_rate),
    SOC_SINGLE_TLV!("ALC Noise Threshold", DA7219_ALC_NOISE, DA7219_ALC_NOISE_SHIFT, DA7219_ALC_THRESHOLD_MAX, DA7219_INVERT, da7219_alc_threshold_tlv),
    SOC_SINGLE_TLV!("ALC Min Threshold", DA7219_ALC_TARGET_MIN, DA7219_ALC_THRESHOLD_MIN_SHIFT, DA7219_ALC_THRESHOLD_MAX, DA7219_INVERT, da7219_alc_threshold_tlv),
    SOC_SINGLE_TLV!("ALC Max Threshold", DA7219_ALC_TARGET_MAX, DA7219_ALC_THRESHOLD_MAX_SHIFT, DA7219_ALC_THRESHOLD_MAX, DA7219_INVERT, da7219_alc_threshold_tlv),
    SOC_SINGLE_TLV!("ALC Max Attenuation", DA7219_ALC_GAIN_LIMITS, DA7219_ALC_ATTEN_MAX_SHIFT, DA7219_ALC_ATTEN_GAIN_MAX, DA7219_NO_INVERT, da7219_alc_gain_tlv),
    SOC_SINGLE_TLV!("ALC Max Volume", DA7219_ALC_GAIN_LIMITS, DA7219_ALC_GAIN_MAX_SHIFT, DA7219_ALC_ATTEN_GAIN_MAX, DA7219_NO_INVERT, da7219_alc_gain_tlv),
    SOC_SINGLE_RANGE_TLV!("ALC Min Analog Volume", DA7219_ALC_ANA_GAIN_LIMITS, DA7219_ALC_ANA_GAIN_MIN_SHIFT, DA7219_ALC_ANA_GAIN_MIN, DA7219_ALC_ANA_GAIN_MAX, DA7219_NO_INVERT, da7219_alc_ana_gain_tlv),
    SOC_SINGLE_RANGE_TLV!("ALC Max Analog Volume", DA7219_ALC_ANA_GAIN_LIMITS, DA7219_ALC_ANA_GAIN_MAX_SHIFT, DA7219_ALC_ANA_GAIN_MIN, DA7219_ALC_ANA_GAIN_MAX, DA7219_NO_INVERT, da7219_alc_ana_gain_tlv),
    SOC_ENUM!("ALC Anticlip Step", da7219_alc_anticlip_step),
    SOC_SINGLE!("ALC Anticlip Switch", DA7219_ALC_ANTICLIP_CTRL, DA7219_ALC_ANTIPCLIP_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT),
    SOC_SINGLE_EXT!("ALC Switch", DA7219_ALC_CTRL1, DA7219_ALC_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT, snd_soc_get_volsw, da7219_alc_sw_put),
    SOC_ENUM!("ADC HPF Mode", da7219_adc_hpf_mode), SOC_ENUM!("ADC HPF Corner Audio", da7219_adc_audio_hpf_corner), SOC_ENUM!("ADC HPF Corner Voice", da7219_adc_voice_hpf_corner),
    SOC_SINGLE_TLV!("Sidetone Volume", DA7219_SIDETONE_GAIN, DA7219_SIDETONE_GAIN_SHIFT, DA7219_SIDETONE_GAIN_MAX, DA7219_NO_INVERT, da7219_sidetone_gain_tlv),
    SOC_SINGLE!("Sidetone Switch", DA7219_SIDETONE_CTRL, DA7219_SIDETONE_MUTE_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_INVERT),
    SOC_SINGLE_EXT_TLV!("ToneGen Volume", DA7219_TONE_GEN_CFG2, DA7219_TONE_GEN_GAIN_SHIFT, DA7219_TONE_GEN_GAIN_MAX, DA7219_NO_INVERT, da7219_volsw_locked_get, da7219_volsw_locked_put, da7219_tonegen_gain_tlv),
    SOC_ENUM_EXT!("ToneGen DTMF Key", da7219_tonegen_dtmf_key, da7219_enum_locked_get, da7219_enum_locked_put),
    SOC_SINGLE_EXT!("ToneGen DTMF Switch", DA7219_TONE_GEN_CFG1, DA7219_DTMF_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT, da7219_volsw_locked_get, da7219_volsw_locked_put),
    SOC_ENUM_EXT!("ToneGen Sinewave Gen Type", da7219_tonegen_swg_sel, da7219_enum_locked_get, da7219_enum_locked_put),
    SOC_SINGLE_EXT!("ToneGen Sinewave1 Freq", DA7219_TONE_GEN_FREQ1_L, DA7219_FREQ1_L_SHIFT, DA7219_FREQ_MAX, DA7219_NO_INVERT, da7219_tonegen_freq_get, da7219_tonegen_freq_put),
    SOC_SINGLE_EXT!("ToneGen Sinewave2 Freq", DA7219_TONE_GEN_FREQ2_L, DA7219_FREQ2_L_SHIFT, DA7219_FREQ_MAX, DA7219_NO_INVERT, da7219_tonegen_freq_get, da7219_tonegen_freq_put),
    SOC_SINGLE_EXT!("ToneGen On Time", DA7219_TONE_GEN_ON_PER, DA7219_BEEP_ON_PER_SHIFT, DA7219_BEEP_ON_OFF_MAX, DA7219_NO_INVERT, da7219_volsw_locked_get, da7219_volsw_locked_put),
    SOC_SINGLE!("ToneGen Off Time", DA7219_TONE_GEN_OFF_PER, DA7219_BEEP_OFF_PER_SHIFT, DA7219_BEEP_ON_OFF_MAX, DA7219_NO_INVERT),
    SOC_ENUM!("Gain Ramp Rate", da7219_gain_ramp_rate),
    SOC_ENUM_EXT!("DAC HPF Mode", da7219_dac_hpf_mode, da7219_enum_locked_get, da7219_enum_locked_put),
    SOC_ENUM!("DAC HPF Corner Audio", da7219_dac_audio_hpf_corner), SOC_ENUM!("DAC HPF Corner Voice", da7219_dac_voice_hpf_corner),
    SOC_SINGLE_TLV!("DAC EQ Band1 Volume", DA7219_DAC_FILTERS2, DA7219_DAC_EQ_BAND1_SHIFT, DA7219_DAC_EQ_BAND_MAX, DA7219_NO_INVERT, da7219_dac_eq_band_tlv),
    SOC_SINGLE_TLV!("DAC EQ Band2 Volume", DA7219_DAC_FILTERS2, DA7219_DAC_EQ_BAND2_SHIFT, DA7219_DAC_EQ_BAND_MAX, DA7219_NO_INVERT, da7219_dac_eq_band_tlv),
    SOC_SINGLE_TLV!("DAC EQ Band3 Volume", DA7219_DAC_FILTERS3, DA7219_DAC_EQ_BAND3_SHIFT, DA7219_DAC_EQ_BAND_MAX, DA7219_NO_INVERT, da7219_dac_eq_band_tlv),
    SOC_SINGLE_TLV!("DAC EQ Band4 Volume", DA7219_DAC_FILTERS3, DA7219_DAC_EQ_BAND4_SHIFT, DA7219_DAC_EQ_BAND_MAX, DA7219_NO_INVERT, da7219_dac_eq_band_tlv),
    SOC_SINGLE_TLV!("DAC EQ Band5 Volume", DA7219_DAC_FILTERS4, DA7219_DAC_EQ_BAND5_SHIFT, DA7219_DAC_EQ_BAND_MAX, DA7219_NO_INVERT, da7219_dac_eq_band_tlv),
    SOC_SINGLE_EXT!("DAC EQ Switch", DA7219_DAC_FILTERS4, DA7219_DAC_EQ_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT, da7219_volsw_locked_get, da7219_volsw_locked_put),
    SOC_ENUM!("DAC Soft Mute Rate", da7219_dac_softmute_rate),
    SOC_SINGLE_EXT!("DAC Soft Mute Switch", DA7219_DAC_FILTERS5, DA7219_DAC_SOFTMUTE_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT, da7219_volsw_locked_get, da7219_volsw_locked_put),
    SOC_ENUM!("DAC NG Setup Time", da7219_dac_ng_setup_time), SOC_ENUM!("DAC NG Rampup Rate", da7219_dac_ng_rampup_rate), SOC_ENUM!("DAC NG Rampdown Rate", da7219_dac_ng_rampdown_rate),
    SOC_SINGLE_TLV!("DAC NG Off Threshold", DA7219_DAC_NG_OFF_THRESH, DA7219_DAC_NG_OFF_THRESHOLD_SHIFT, DA7219_DAC_NG_THRESHOLD_MAX, DA7219_NO_INVERT, da7219_dac_ng_threshold_tlv),
    SOC_SINGLE_TLV!("DAC NG On Threshold", DA7219_DAC_NG_ON_THRESH, DA7219_DAC_NG_ON_THRESHOLD_SHIFT, DA7219_DAC_NG_THRESHOLD_MAX, DA7219_NO_INVERT, da7219_dac_ng_threshold_tlv),
    SOC_SINGLE!("DAC NG Switch", DA7219_DAC_NG_CTRL, DA7219_DAC_NG_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT),
    SOC_DOUBLE_R_EXT_TLV!("Playback Digital Volume", DA7219_DAC_L_GAIN, DA7219_DAC_R_GAIN, DA7219_DAC_L_DIGITAL_GAIN_SHIFT, DA7219_DAC_DIGITAL_GAIN_MAX, DA7219_NO_INVERT, da7219_volsw_locked_get, da7219_volsw_locked_put, da7219_dac_dig_gain_tlv),
    SOC_DOUBLE_R_EXT!("Playback Digital Switch", DA7219_DAC_L_CTRL, DA7219_DAC_R_CTRL, DA7219_DAC_L_MUTE_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_INVERT, da7219_volsw_locked_get, da7219_volsw_locked_put),
    SOC_DOUBLE_R!("Playback Digital Gain Ramp Switch", DA7219_DAC_L_CTRL, DA7219_DAC_R_CTRL, DA7219_DAC_L_RAMP_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT),
    SOC_ENUM!("Charge Pump Track Mode", da7219_cp_track_mode),
    SOC_SINGLE!("Charge Pump Threshold", DA7219_CP_VOL_THRESHOLD1, DA7219_CP_THRESH_VDD2_SHIFT, DA7219_CP_THRESH_VDD2_MAX, DA7219_NO_INVERT),
    SOC_DOUBLE_R_EXT_TLV!("Headphone Volume", DA7219_HP_L_GAIN, DA7219_HP_R_GAIN, DA7219_HP_L_AMP_GAIN_SHIFT, DA7219_HP_AMP_GAIN_MAX, DA7219_NO_INVERT, da7219_volsw_locked_get, da7219_volsw_locked_put, da7219_hp_gain_tlv),
    SOC_DOUBLE_R_EXT!("Headphone Switch", DA7219_HP_L_CTRL, DA7219_HP_R_CTRL, DA7219_HP_L_AMP_MUTE_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_INVERT, da7219_volsw_locked_get, da7219_volsw_locked_put),
    SOC_DOUBLE_R!("Headphone Gain Ramp Switch", DA7219_HP_L_CTRL, DA7219_HP_R_CTRL, DA7219_HP_L_AMP_RAMP_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT),
    SOC_DOUBLE_R!("Headphone ZC Gain Switch", DA7219_HP_L_CTRL, DA7219_HP_R_CTRL, DA7219_HP_L_AMP_ZC_EN_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT),
];

static da7219_out_sel_txt: [*const c_char; 4] = [cstr!("ADC"), cstr!("Tone Generator"), cstr!("DAIL"), cstr!("DAIR")];
static da7219_out_dail_sel: soc_enum = soc_enum { _unused: [] };
static da7219_out_dail_sel_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Out DAIL Mux", da7219_out_dail_sel);
static da7219_out_dair_sel: soc_enum = soc_enum { _unused: [] };
static da7219_out_dair_sel_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Out DAIR Mux", da7219_out_dair_sel);
static da7219_out_dacl_sel: soc_enum = soc_enum { _unused: [] };
static da7219_out_dacl_sel_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Out DACL Mux", da7219_out_dacl_sel);
static da7219_out_dacr_sel: soc_enum = soc_enum { _unused: [] };
static da7219_out_dacr_sel_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Out DACR Mux", da7219_out_dacr_sel);

static da7219_mixin_controls: [snd_kcontrol_new; 1] = [SOC_DAPM_SINGLE!("Mic Switch", DA7219_MIXIN_L_SELECT, DA7219_MIXIN_L_MIX_SELECT_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT)];
static da7219_mixout_l_controls: [snd_kcontrol_new; 1] = [SOC_DAPM_SINGLE!("DACL Switch", DA7219_MIXOUT_L_SELECT, DA7219_MIXOUT_L_MIX_SELECT_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT)];
static da7219_mixout_r_controls: [snd_kcontrol_new; 1] = [SOC_DAPM_SINGLE!("DACR Switch", DA7219_MIXOUT_R_SELECT, DA7219_MIXOUT_R_MIX_SELECT_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT)];
macro_rules! DA7219_DMIX_ST_CTRLS { ($reg:expr) => {
    SOC_DAPM_SINGLE!("Out FilterL Switch", $reg, DA7219_DMIX_ST_SRC_OUTFILT1L_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT),
    SOC_DAPM_SINGLE!("Out FilterR Switch", $reg, DA7219_DMIX_ST_SRC_OUTFILT1R_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT),
    SOC_DAPM_SINGLE!("Sidetone Switch", $reg, DA7219_DMIX_ST_SRC_SIDETONE_SHIFT, DA7219_SWITCH_EN_MAX, DA7219_NO_INVERT)
}; }
static da7219_st_out_filtl_mix_controls: [snd_kcontrol_new; 3] = [DA7219_DMIX_ST_CTRLS!(DA7219_DROUTING_ST_OUTFILT_1L)];
static da7219_st_out_filtr_mix_controls: [snd_kcontrol_new; 3] = [DA7219_DMIX_ST_CTRLS!(DA7219_DROUTING_ST_OUTFILT_1R)];

unsafe extern "C" fn da7219_mic_pga_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let da7219 = snd_soc_component_get_drvdata(component);
    match event as c_uint {
        SND_SOC_DAPM_POST_PMU => {
            if (*da7219).micbias_on_event {
                (*da7219).micbias_on_event = false;
                msleep((*da7219).mic_pga_delay);
            }
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn da7219_dai_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let da7219 = snd_soc_component_get_drvdata(component);
    let bclk = (*da7219).dai_clks[DA7219_DAI_BCLK_IDX as usize];
    let mut i = 0;
    let mut srm_lock = false;
    match event as c_uint {
        SND_SOC_DAPM_PRE_PMU => {
            if (*da7219).master {
                if !bclk.is_null() {
                    let ret = clk_prepare_enable(bclk);
                    if ret != 0 {
                        dev_err((*component).dev, cstr!("Failed to enable DAI clks\n"));
                        return ret;
                    }
                } else {
                    snd_soc_component_update_bits(component, DA7219_DAI_CLK_MODE, DA7219_DAI_CLK_EN_MASK, DA7219_DAI_CLK_EN_MASK);
                }
            }
            snd_soc_component_update_bits(component, DA7219_PC_COUNT, DA7219_PC_FREERUN_MASK, 0);
            let pll_ctrl = snd_soc_component_read(component, DA7219_PLL_CTRL) as u8;
            if (pll_ctrl as c_uint & DA7219_PLL_MODE_MASK) != DA7219_PLL_MODE_SRM { return 0; }
            while i < DA7219_SRM_CHECK_RETRIES && !srm_lock {
                let pll_status = snd_soc_component_read(component, DA7219_PLL_SRM_STS) as u8;
                if (pll_status as c_uint & DA7219_PLL_SRM_STS_SRM_LOCK) != 0 { srm_lock = true; } else { i += 1; msleep(50); }
            }
            if !srm_lock { dev_warn((*component).dev, cstr!("SRM failed to lock\n")); }
            0
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, DA7219_PC_COUNT, DA7219_PC_FREERUN_MASK, DA7219_PC_FREERUN_MASK);
            if (*da7219).master {
                if !bclk.is_null() { clk_disable_unprepare(bclk); }
                else { snd_soc_component_update_bits(component, DA7219_DAI_CLK_MODE, DA7219_DAI_CLK_EN_MASK, 0); }
            }
            0
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn da7219_settling_event(_w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    match event as c_uint {
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD => msleep(DA7219_SETTLING_DELAY),
        _ => {}
    }
    0
}

unsafe extern "C" fn da7219_mixout_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let (hp_ctrl, min_gain_mask) = match (*w).reg {
        DA7219_MIXOUT_L_CTRL => (DA7219_HP_L_CTRL, DA7219_HP_L_AMP_MIN_GAIN_EN_MASK),
        DA7219_MIXOUT_R_CTRL => (DA7219_HP_R_CTRL, DA7219_HP_R_AMP_MIN_GAIN_EN_MASK),
        _ => return -EINVAL,
    };
    match event as c_uint {
        SND_SOC_DAPM_PRE_PMD => { snd_soc_component_update_bits(component, hp_ctrl, min_gain_mask, min_gain_mask); msleep(DA7219_MIN_GAIN_DELAY); }
        SND_SOC_DAPM_POST_PMU => { snd_soc_component_update_bits(component, hp_ctrl, min_gain_mask, 0); }
        _ => {}
    }
    0
}

unsafe extern "C" fn da7219_gain_ramp_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let da7219 = snd_soc_component_get_drvdata(component);
    match event as c_uint {
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD => {
            (*da7219).gain_ramp_ctrl = snd_soc_component_read(component, DA7219_GAIN_RAMP_CTRL) as u8;
            snd_soc_component_write(component, DA7219_GAIN_RAMP_CTRL, DA7219_GAIN_RAMP_RATE_NOMINAL);
        }
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_write(component, DA7219_GAIN_RAMP_CTRL, (*da7219).gain_ramp_ctrl as c_uint);
        }
        _ => {}
    }
    0
}

static da7219_dapm_widgets: [snd_soc_dapm_widget_item; 35] = [
    SND_SOC_DAPM_SUPPLY!("Mic Bias", DA7219_MICBIAS_CTRL, DA7219_MICBIAS1_EN_SHIFT, DA7219_NO_INVERT, NULL, 0),
    SND_SOC_DAPM_INPUT!("MIC"),
    SND_SOC_DAPM_PGA_E!("Mic PGA", DA7219_MIC_1_CTRL, DA7219_MIC_1_AMP_EN_SHIFT, DA7219_NO_INVERT, NULL, 0, da7219_mic_pga_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_PGA_E!("Mixin PGA", DA7219_MIXIN_L_CTRL, DA7219_MIXIN_L_AMP_EN_SHIFT, DA7219_NO_INVERT, NULL, 0, da7219_settling_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_ADC!("ADC", NULL, DA7219_ADC_L_CTRL, DA7219_ADC_L_EN_SHIFT, DA7219_NO_INVERT),
    SND_SOC_DAPM_SIGGEN!("TONE"),
    SND_SOC_DAPM_PGA!("Tone Generator", DA7219_TONE_GEN_CFG1, DA7219_START_STOPN_SHIFT, DA7219_NO_INVERT, NULL, 0),
    SND_SOC_DAPM_ADC!("Sidetone Filter", NULL, DA7219_SIDETONE_CTRL, DA7219_SIDETONE_EN_SHIFT, DA7219_NO_INVERT),
    SND_SOC_DAPM_SUPPLY!("Mixer In Supply", DA7219_MIXIN_L_CTRL, DA7219_MIXIN_L_MIX_EN_SHIFT, DA7219_NO_INVERT, NULL, 0),
    SND_SOC_DAPM_MIXER!("Mixer In", SND_SOC_NOPM, 0, 0, da7219_mixin_controls, ARRAY_SIZE!(da7219_mixin_controls)),
    SND_SOC_DAPM_MUX!("Out DAIL Mux", SND_SOC_NOPM, 0, 0, &da7219_out_dail_sel_mux),
    SND_SOC_DAPM_MUX!("Out DAIR Mux", SND_SOC_NOPM, 0, 0, &da7219_out_dair_sel_mux),
    SND_SOC_DAPM_SUPPLY!("DAI", DA7219_DAI_CTRL, DA7219_DAI_EN_SHIFT, DA7219_NO_INVERT, da7219_dai_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_AIF_OUT!("DAIOUT", "Capture", 0, DA7219_DAI_TDM_CTRL, DA7219_DAI_OE_SHIFT, DA7219_NO_INVERT),
    SND_SOC_DAPM_AIF_IN!("DAIIN", "Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX!("Out DACL Mux", SND_SOC_NOPM, 0, 0, &da7219_out_dacl_sel_mux),
    SND_SOC_DAPM_MUX!("Out DACR Mux", SND_SOC_NOPM, 0, 0, &da7219_out_dacr_sel_mux),
    SND_SOC_DAPM_MIXER!("Mixer Out FilterL", SND_SOC_NOPM, 0, 0, da7219_mixout_l_controls, ARRAY_SIZE!(da7219_mixout_l_controls)),
    SND_SOC_DAPM_MIXER!("Mixer Out FilterR", SND_SOC_NOPM, 0, 0, da7219_mixout_r_controls, ARRAY_SIZE!(da7219_mixout_r_controls)),
    SND_SOC_DAPM_MIXER!("ST Mixer Out FilterL", SND_SOC_NOPM, 0, 0, da7219_st_out_filtl_mix_controls, ARRAY_SIZE!(da7219_st_out_filtl_mix_controls)),
    SND_SOC_DAPM_MIXER!("ST Mixer Out FilterR", SND_SOC_NOPM, 0, 0, da7219_st_out_filtr_mix_controls, ARRAY_SIZE!(da7219_st_out_filtr_mix_controls)),
    SND_SOC_DAPM_DAC_E!("DACL", NULL, DA7219_DAC_L_CTRL, DA7219_DAC_L_EN_SHIFT, DA7219_NO_INVERT, da7219_settling_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E!("DACR", NULL, DA7219_DAC_R_CTRL, DA7219_DAC_R_EN_SHIFT, DA7219_NO_INVERT, da7219_settling_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_PGA_E!("Mixout Left PGA", DA7219_MIXOUT_L_CTRL, DA7219_MIXOUT_L_AMP_EN_SHIFT, DA7219_NO_INVERT, NULL, 0, da7219_mixout_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_PGA_E!("Mixout Right PGA", DA7219_MIXOUT_R_CTRL, DA7219_MIXOUT_R_AMP_EN_SHIFT, DA7219_NO_INVERT, NULL, 0, da7219_mixout_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY_S!("Headphone Left PGA", 1, DA7219_HP_L_CTRL, DA7219_HP_L_AMP_EN_SHIFT, DA7219_NO_INVERT, da7219_settling_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("Headphone Right PGA", 1, DA7219_HP_R_CTRL, DA7219_HP_R_AMP_EN_SHIFT, DA7219_NO_INVERT, da7219_settling_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("Charge Pump", 0, DA7219_CP_CTRL, DA7219_CP_EN_SHIFT, DA7219_NO_INVERT, da7219_settling_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_OUTPUT!("HPL"), SND_SOC_DAPM_OUTPUT!("HPR"),
    SND_SOC_DAPM_PRE!("Pre Power Gain Ramp", da7219_gain_ramp_event),
    SND_SOC_DAPM_POST!("Post Power Gain Ramp", da7219_gain_ramp_event),
    opaque_widget!(), opaque_widget!(), opaque_widget!(),
];

macro_rules! route { ($sink:literal, NULL, $source:literal) => { snd_soc_dapm_route { sink: cstr!($sink), control: ptr::null(), source: cstr!($source) } }; ($sink:literal, $control:literal, $source:literal) => { snd_soc_dapm_route { sink: cstr!($sink), control: cstr!($control), source: cstr!($source) } }; }
static da7219_audio_map: [snd_soc_dapm_route; 34] = [
    route!("MIC", NULL, "Mic Bias"), route!("Mic PGA", NULL, "MIC"), route!("Mixin PGA", NULL, "Mic PGA"), route!("ADC", NULL, "Mixin PGA"),
    route!("Mixer In", NULL, "Mixer In Supply"), route!("Mixer In", "Mic Switch", "ADC"), route!("Sidetone Filter", NULL, "Mixer In"), route!("Tone Generator", NULL, "TONE"),
    route!("Out DAIL Mux", "ADC", "Mixer In"), route!("Out DAIL Mux", "Tone Generator", "Tone Generator"), route!("Out DAIL Mux", "DAIL", "DAIOUT"), route!("Out DAIL Mux", "DAIR", "DAIOUT"),
    route!("Out DAIR Mux", "ADC", "Mixer In"), route!("Out DAIR Mux", "Tone Generator", "Tone Generator"), route!("Out DAIR Mux", "DAIL", "DAIOUT"), route!("Out DAIR Mux", "DAIR", "DAIOUT"),
    route!("DAIOUT", NULL, "Out DAIL Mux"), route!("DAIOUT", NULL, "Out DAIR Mux"), route!("DAIOUT", NULL, "DAI"), route!("DAIIN", NULL, "DAI"),
    route!("Out DACL Mux", "ADC", "Mixer In"), route!("Out DACL Mux", "Tone Generator", "Tone Generator"), route!("Out DACL Mux", "DAIL", "DAIIN"), route!("Out DACL Mux", "DAIR", "DAIIN"),
    route!("Out DACR Mux", "ADC", "Mixer In"), route!("Out DACR Mux", "Tone Generator", "Tone Generator"), route!("Out DACR Mux", "DAIL", "DAIIN"), route!("Out DACR Mux", "DAIR", "DAIIN"),
    route!("Mixer Out FilterL", "DACL Switch", "Out DACL Mux"), route!("Mixer Out FilterR", "DACR Switch", "Out DACR Mux"),
    route!("ST Mixer Out FilterL", "Out FilterL Switch", "Mixer Out FilterL"), route!("ST Mixer Out FilterL", "Out FilterR Switch", "Mixer Out FilterR"), route!("ST Mixer Out FilterL", "Sidetone Switch", "Sidetone Filter"), route!("ST Mixer Out FilterR", "Out FilterL Switch", "Mixer Out FilterL"),
];

unsafe extern "C" fn da7219_set_dai_sysclk(codec_dai: *mut snd_soc_dai, clk_id: c_int, mut freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let da7219 = snd_soc_component_get_drvdata(component);
    mutex_lock(&mut (*da7219).pll_lock);
    if (*da7219).clk_src == clk_id && (*da7219).mclk_rate == freq { mutex_unlock(&mut (*da7219).pll_lock); return 0; }
    if freq < 2000000 || freq > 54000000 {
        dev_err((*codec_dai).dev, cstr!("Unsupported MCLK value %d\n"), freq);
        mutex_unlock(&mut (*da7219).pll_lock);
        return -EINVAL;
    }
    match clk_id as c_uint {
        DA7219_CLKSRC_MCLK_SQR => { snd_soc_component_update_bits(component, DA7219_PLL_CTRL, DA7219_PLL_MCLK_SQR_EN_MASK, DA7219_PLL_MCLK_SQR_EN_MASK); }
        DA7219_CLKSRC_MCLK => { snd_soc_component_update_bits(component, DA7219_PLL_CTRL, DA7219_PLL_MCLK_SQR_EN_MASK, 0); }
        _ => { dev_err((*codec_dai).dev, cstr!("Unknown clock source %d\n"), clk_id); mutex_unlock(&mut (*da7219).pll_lock); return -EINVAL; }
    }
    (*da7219).clk_src = clk_id;
    if !(*da7219).mclk.is_null() {
        freq = clk_round_rate((*da7219).mclk, freq as c_ulong) as c_uint;
        let ret = clk_set_rate((*da7219).mclk, freq as c_ulong);
        if ret != 0 { dev_err((*codec_dai).dev, cstr!("Failed to set clock rate %d\n"), freq); mutex_unlock(&mut (*da7219).pll_lock); return ret; }
    }
    (*da7219).mclk_rate = freq;
    mutex_unlock(&mut (*da7219).pll_lock);
    0
}

#[no_mangle]
pub unsafe extern "C" fn da7219_set_pll(component: *mut snd_soc_component, source: c_int, fout: c_uint) -> c_int {
    let da7219 = snd_soc_component_get_drvdata(component);
    let (indiv_bits, indiv): (u8, u8);
    if (*da7219).mclk_rate < 2000000 {
        dev_err((*component).dev, cstr!("PLL input clock %d below valid range\n"), (*da7219).mclk_rate);
        return -EINVAL;
    } else if (*da7219).mclk_rate <= 4500000 { indiv_bits = DA7219_PLL_INDIV_2_TO_4_5_MHZ as u8; indiv = DA7219_PLL_INDIV_2_TO_4_5_MHZ_VAL as u8; }
    else if (*da7219).mclk_rate <= 9000000 { indiv_bits = DA7219_PLL_INDIV_4_5_TO_9_MHZ as u8; indiv = DA7219_PLL_INDIV_4_5_TO_9_MHZ_VAL as u8; }
    else if (*da7219).mclk_rate <= 18000000 { indiv_bits = DA7219_PLL_INDIV_9_TO_18_MHZ as u8; indiv = DA7219_PLL_INDIV_9_TO_18_MHZ_VAL as u8; }
    else if (*da7219).mclk_rate <= 36000000 { indiv_bits = DA7219_PLL_INDIV_18_TO_36_MHZ as u8; indiv = DA7219_PLL_INDIV_18_TO_36_MHZ_VAL as u8; }
    else if (*da7219).mclk_rate <= 54000000 { indiv_bits = DA7219_PLL_INDIV_36_TO_54_MHZ as u8; indiv = DA7219_PLL_INDIV_36_TO_54_MHZ_VAL as u8; }
    else { dev_err((*component).dev, cstr!("PLL input clock %d above valid range\n"), (*da7219).mclk_rate); return -EINVAL; }
    let freq_ref = (*da7219).mclk_rate / indiv as c_uint;
    let mut pll_ctrl = indiv_bits as c_uint;
    match source as c_uint {
        DA7219_SYSCLK_MCLK => { pll_ctrl |= DA7219_PLL_MODE_BYPASS; snd_soc_component_update_bits(component, DA7219_PLL_CTRL, DA7219_PLL_INDIV_MASK | DA7219_PLL_MODE_MASK, pll_ctrl); return 0; }
        DA7219_SYSCLK_PLL => pll_ctrl |= DA7219_PLL_MODE_NORMAL,
        DA7219_SYSCLK_PLL_SRM => pll_ctrl |= DA7219_PLL_MODE_SRM,
        _ => { dev_err((*component).dev, cstr!("Invalid PLL config\n")); return -EINVAL; }
    }
    let pll_integer = fout / freq_ref;
    let mut frac_div = ((fout % freq_ref) as u64).wrapping_mul(8192);
    frac_div /= freq_ref as u64;
    let pll_frac_top = ((frac_div >> DA7219_BYTE_SHIFT) & DA7219_BYTE_MASK as u64) as c_uint;
    let pll_frac_bot = (frac_div & DA7219_BYTE_MASK as u64) as c_uint;
    snd_soc_component_write(component, DA7219_PLL_FRAC_TOP, pll_frac_top);
    snd_soc_component_write(component, DA7219_PLL_FRAC_BOT, pll_frac_bot);
    snd_soc_component_write(component, DA7219_PLL_INTEGER, pll_integer);
    snd_soc_component_update_bits(component, DA7219_PLL_CTRL, DA7219_PLL_INDIV_MASK | DA7219_PLL_MODE_MASK, pll_ctrl);
    0
}

unsafe extern "C" fn da7219_set_dai_pll(codec_dai: *mut snd_soc_dai, _pll_id: c_int, source: c_int, _fref: c_uint, fout: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let da7219 = snd_soc_component_get_drvdata(component);
    mutex_lock(&mut (*da7219).pll_lock);
    let ret = da7219_set_pll(component, source, fout);
    mutex_unlock(&mut (*da7219).pll_lock);
    ret
}

unsafe extern "C" fn da7219_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let da7219 = snd_soc_component_get_drvdata(component);
    let mut dai_clk_mode: c_uint = 0;
    let mut dai_ctrl: c_uint = 0;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => (*da7219).master = true,
        SND_SOC_DAIFMT_CBC_CFC => (*da7219).master = false,
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J => match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => {}
            SND_SOC_DAIFMT_NB_IF => dai_clk_mode |= DA7219_DAI_WCLK_POL_INV,
            SND_SOC_DAIFMT_IB_NF => dai_clk_mode |= DA7219_DAI_CLK_POL_INV,
            SND_SOC_DAIFMT_IB_IF => dai_clk_mode |= DA7219_DAI_WCLK_POL_INV | DA7219_DAI_CLK_POL_INV,
            _ => return -EINVAL,
        },
        SND_SOC_DAIFMT_DSP_B => match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => dai_clk_mode |= DA7219_DAI_CLK_POL_INV,
            SND_SOC_DAIFMT_NB_IF => dai_clk_mode |= DA7219_DAI_WCLK_POL_INV | DA7219_DAI_CLK_POL_INV,
            SND_SOC_DAIFMT_IB_NF => {}
            SND_SOC_DAIFMT_IB_IF => dai_clk_mode |= DA7219_DAI_WCLK_POL_INV,
            _ => return -EINVAL,
        },
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => dai_ctrl |= DA7219_DAI_FORMAT_I2S,
        SND_SOC_DAIFMT_LEFT_J => dai_ctrl |= DA7219_DAI_FORMAT_LEFT_J,
        SND_SOC_DAIFMT_RIGHT_J => dai_ctrl |= DA7219_DAI_FORMAT_RIGHT_J,
        SND_SOC_DAIFMT_DSP_B => dai_ctrl |= DA7219_DAI_FORMAT_DSP,
        _ => return -EINVAL,
    }
    snd_soc_component_update_bits(component, DA7219_DAI_CLK_MODE, DA7219_DAI_CLK_POL_MASK | DA7219_DAI_WCLK_POL_MASK, dai_clk_mode);
    snd_soc_component_update_bits(component, DA7219_DAI_CTRL, DA7219_DAI_FORMAT_MASK, dai_ctrl);
    0
}

unsafe extern "C" fn da7219_set_bclks_per_wclk(component: *mut snd_soc_component, factor: c_ulong) -> c_int {
    let bclks_per_wclk = match factor {
        32 => DA7219_DAI_BCLKS_PER_WCLK_32,
        64 => DA7219_DAI_BCLKS_PER_WCLK_64,
        128 => DA7219_DAI_BCLKS_PER_WCLK_128,
        256 => DA7219_DAI_BCLKS_PER_WCLK_256,
        _ => return -EINVAL,
    };
    snd_soc_component_update_bits(component, DA7219_DAI_CLK_MODE, DA7219_DAI_BCLKS_PER_WCLK_MASK, bclks_per_wclk);
    0
}

unsafe extern "C" fn da7219_set_dai_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, _rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let component = (*dai).component;
    let da7219 = snd_soc_component_get_drvdata(component);
    let wclk = (*da7219).dai_clks[DA7219_DAI_WCLK_IDX as usize];
    let bclk = (*da7219).dai_clks[DA7219_DAI_BCLK_IDX as usize];
    if tx_mask == 0 {
        snd_soc_component_update_bits(component, DA7219_DAI_TDM_CTRL, DA7219_DAI_TDM_CH_EN_MASK | DA7219_DAI_TDM_MODE_EN_MASK, 0);
        (*da7219).tdm_en = false;
        return 0;
    }
    let slot_offset = (ffs(tx_mask as c_int) - 1) as c_uint;
    let ch_mask = tx_mask >> slot_offset;
    if fls(ch_mask as c_int) as c_uint > DA7219_DAI_TDM_MAX_SLOTS {
        dev_err((*component).dev, cstr!("Invalid number of slots, max = %d\n"), DA7219_DAI_TDM_MAX_SLOTS);
        return -EINVAL;
    }
    let offset = slot_offset * slot_width as c_uint;
    if offset > DA7219_DAI_OFFSET_MAX {
        dev_err((*component).dev, cstr!("Invalid frame offset %d\n"), offset);
        return -EINVAL;
    }
    if (*da7219).master {
        let frame_size = slots * slot_width;
        if !bclk.is_null() {
            let sr = clk_get_rate(wclk);
            let bclk_rate = sr * frame_size as c_ulong;
            let ret = clk_set_rate(bclk, bclk_rate);
            if ret != 0 { dev_err((*component).dev, cstr!("Failed to set TDM BCLK rate %lu: %d\n"), bclk_rate, ret); return ret; }
        } else {
            let ret = da7219_set_bclks_per_wclk(component, frame_size as c_ulong);
            if ret != 0 { dev_err((*component).dev, cstr!("Failed to set TDM BCLKs per WCLK %d: %d\n"), frame_size, ret); return ret; }
        }
    }
    let dai_offset: __le16 = (offset as u16).to_le();
    regmap_bulk_write((*da7219).regmap, DA7219_DAI_OFFSET_LOWER, &dai_offset as *const _ as *const c_void, size_of::<__le16>());
    snd_soc_component_update_bits(component, DA7219_DAI_TDM_CTRL, DA7219_DAI_TDM_CH_EN_MASK | DA7219_DAI_TDM_MODE_EN_MASK, (ch_mask << DA7219_DAI_TDM_CH_EN_SHIFT) | DA7219_DAI_TDM_MODE_EN_MASK);
    (*da7219).tdm_en = true;
    0
}

unsafe extern "C" fn da7219_set_sr(component: *mut snd_soc_component, rate: c_ulong) -> c_int {
    let fs = match rate {
        8000 => DA7219_SR_8000, 11025 => DA7219_SR_11025, 12000 => DA7219_SR_12000, 16000 => DA7219_SR_16000,
        22050 => DA7219_SR_22050, 24000 => DA7219_SR_24000, 32000 => DA7219_SR_32000, 44100 => DA7219_SR_44100,
        48000 => DA7219_SR_48000, 88200 => DA7219_SR_88200, 96000 => DA7219_SR_96000,
        _ => return -EINVAL,
    };
    snd_soc_component_write(component, DA7219_SR, fs);
    0
}

unsafe extern "C" fn da7219_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let da7219 = snd_soc_component_get_drvdata(component);
    let wclk = (*da7219).dai_clks[DA7219_DAI_WCLK_IDX as usize];
    let bclk = (*da7219).dai_clks[DA7219_DAI_BCLK_IDX as usize];
    let word_len = params_width(params);
    let mut dai_ctrl = match word_len {
        16 => DA7219_DAI_WORD_LENGTH_S16_LE, 20 => DA7219_DAI_WORD_LENGTH_S20_LE, 24 => DA7219_DAI_WORD_LENGTH_S24_LE, 32 => DA7219_DAI_WORD_LENGTH_S32_LE,
        _ => return -EINVAL,
    };
    let channels = params_channels(params);
    if channels < 1 || channels > DA7219_DAI_CH_NUM_MAX {
        dev_err((*component).dev, cstr!("Invalid number of channels, only 1 to %d supported\n"), DA7219_DAI_CH_NUM_MAX);
        return -EINVAL;
    }
    dai_ctrl |= channels << DA7219_DAI_CH_NUM_SHIFT;
    let sr = params_rate(params);
    let mut ret;
    if (*da7219).master && !wclk.is_null() {
        ret = clk_set_rate(wclk, sr);
        if ret != 0 { dev_err((*component).dev, cstr!("Failed to set WCLK SR %lu: %d\n"), sr, ret); return ret; }
    } else {
        ret = da7219_set_sr(component, sr);
        if ret != 0 { dev_err((*component).dev, cstr!("Failed to set SR %lu: %d\n"), sr, ret); return ret; }
    }
    if (*da7219).master && !(*da7219).tdm_en {
        let frame_size = if word_len * DA7219_DAI_CH_NUM_MAX as c_int <= 32 { 32 } else { 64 };
        if !bclk.is_null() {
            let mut bclk_rate = (frame_size as c_ulong) * sr;
            bclk_rate = clk_round_rate(bclk, bclk_rate);
            if bclk_rate / sr < frame_size as c_ulong {
                dev_err((*component).dev, cstr!("BCLK rate mismatch against frame size"));
                return -EINVAL;
            }
            ret = clk_set_rate(bclk, bclk_rate);
            if ret != 0 { dev_err((*component).dev, cstr!("Failed to set BCLK rate %lu: %d\n"), bclk_rate, ret); return ret; }
        } else {
            ret = da7219_set_bclks_per_wclk(component, frame_size as c_ulong);
            if ret != 0 { dev_err((*component).dev, cstr!("Failed to set BCLKs per WCLK %d: %d\n"), frame_size, ret); return ret; }
        }
    }
    snd_soc_component_update_bits(component, DA7219_DAI_CTRL, DA7219_DAI_WORD_LENGTH_MASK | DA7219_DAI_CH_NUM_MASK, dai_ctrl);
    0
}

static da7219_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(da7219_hw_params), set_sysclk: Some(da7219_set_dai_sysclk), set_pll: Some(da7219_set_dai_pll),
    set_fmt: Some(da7219_set_dai_fmt), set_tdm_slot: Some(da7219_set_dai_tdm_slot),
};

const DA7219_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;
const DA7219_RATES: c_uint = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_11025 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_22050 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000;

static mut da7219_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("da7219-hifi"),
    playback: snd_soc_pcm_stream { stream_name: cstr!("Playback"), channels_min: 1, channels_max: DA7219_DAI_CH_NUM_MAX, rates: DA7219_RATES, formats: DA7219_FORMATS },
    capture: snd_soc_pcm_stream { stream_name: cstr!("Capture"), channels_min: 1, channels_max: DA7219_DAI_CH_NUM_MAX, rates: DA7219_RATES, formats: DA7219_FORMATS },
    ops: &da7219_dai_ops, symmetric_rate: 1, symmetric_channels: 1, symmetric_sample_bits: 1,
};

unsafe extern "C" fn da7219_fw_micbias_lvl(dev: *mut device, val: u32) -> da7219_micbias_voltage {
    match val {
        1600 => DA7219_MICBIAS_1_6V, 1800 => DA7219_MICBIAS_1_8V, 2000 => DA7219_MICBIAS_2_0V,
        2200 => DA7219_MICBIAS_2_2V, 2400 => DA7219_MICBIAS_2_4V, 2600 => DA7219_MICBIAS_2_6V,
        _ => { dev_warn(dev, cstr!("Invalid micbias level")); DA7219_MICBIAS_2_2V }
    }
}

unsafe extern "C" fn da7219_fw_mic_amp_in_sel(dev: *mut device, str_: *const c_char) -> da7219_mic_amp_in_sel {
    if strcmp(str_, cstr!("diff")) == 0 { DA7219_MIC_AMP_IN_SEL_DIFF }
    else if strcmp(str_, cstr!("se_p")) == 0 { DA7219_MIC_AMP_IN_SEL_SE_P }
    else if strcmp(str_, cstr!("se_n")) == 0 { DA7219_MIC_AMP_IN_SEL_SE_N }
    else { dev_warn(dev, cstr!("Invalid mic input type selection")); DA7219_MIC_AMP_IN_SEL_DIFF }
}

unsafe extern "C" fn da7219_fw_to_pdata(dev: *mut device) -> *mut da7219_pdata {
    let pdata = devm_kzalloc(dev, size_of::<da7219_pdata>(), GFP_KERNEL) as *mut da7219_pdata;
    if pdata.is_null() { return ptr::null_mut(); }
    (*pdata).wakeup_source = device_property_read_bool(dev, cstr!("wakeup-source"));
    (*pdata).dai_clk_names[DA7219_DAI_WCLK_IDX as usize] = cstr!("da7219-dai-wclk");
    (*pdata).dai_clk_names[DA7219_DAI_BCLK_IDX as usize] = cstr!("da7219-dai-bclk");
    if device_property_read_string_array(dev, cstr!("clock-output-names"), (*pdata).dai_clk_names.as_mut_ptr(), DA7219_DAI_NUM_CLKS as usize) < 0 {
        dev_warn(dev, cstr!("Using default DAI clk names: %s, %s\n"), (*pdata).dai_clk_names[DA7219_DAI_WCLK_IDX as usize], (*pdata).dai_clk_names[DA7219_DAI_BCLK_IDX as usize]);
    }
    let mut of_val32: u32 = 0;
    if device_property_read_u32(dev, cstr!("dlg,micbias-lvl"), &mut of_val32) >= 0 { (*pdata).micbias_lvl = da7219_fw_micbias_lvl(dev, of_val32); }
    else { (*pdata).micbias_lvl = DA7219_MICBIAS_2_2V; }
    let mut of_str: *const c_char = ptr::null();
    if device_property_read_string(dev, cstr!("dlg,mic-amp-in-sel"), &mut of_str) == 0 { (*pdata).mic_amp_in_sel = da7219_fw_mic_amp_in_sel(dev, of_str); }
    else { (*pdata).mic_amp_in_sel = DA7219_MIC_AMP_IN_SEL_DIFF; }
    pdata
}

unsafe extern "C" fn da7219_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let da7219 = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);
    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_STANDBY && !(*da7219).mclk.is_null() {
                let ret = clk_prepare_enable((*da7219).mclk);
                if ret != 0 { dev_err((*component).dev, cstr!("Failed to enable mclk\n")); return ret; }
            }
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                snd_soc_component_update_bits(component, DA7219_REFERENCES, DA7219_BIAS_EN_MASK, DA7219_BIAS_EN_MASK);
            }
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_PREPARE && !(*da7219).mclk.is_null() { clk_disable_unprepare((*da7219).mclk); }
        }
        SND_SOC_BIAS_OFF => {
            if !(*da7219).wakeup_source { snd_soc_component_update_bits(component, DA7219_REFERENCES, DA7219_BIAS_EN_MASK, 0); }
        }
        _ => {}
    }
    0
}

static da7219_supply_names: [*const c_char; DA7219_NUM_SUPPLIES as usize] = [cstr!("VDD"), cstr!("VDDMIC"), cstr!("VDDIO")];

unsafe extern "C" fn da7219_handle_supplies(component: *mut snd_soc_component, io_voltage_lvl: *mut u8) -> c_int {
    let da7219 = snd_soc_component_get_drvdata(component);
    for i in 0..DA7219_NUM_SUPPLIES as usize { (*da7219).supplies[i].supply = da7219_supply_names[i]; }
    let mut ret = regulator_bulk_get((*component).dev, DA7219_NUM_SUPPLIES as c_int, (*da7219).supplies.as_mut_ptr());
    if ret != 0 { dev_err((*component).dev, cstr!("Failed to get supplies")); return ret; }
    *io_voltage_lvl = DA7219_IO_VOLTAGE_LEVEL_2_5V_3_6V as u8;
    let vddio = (*da7219).supplies[DA7219_SUPPLY_VDDIO as usize].consumer;
    ret = regulator_get_voltage(vddio);
    if ret < 1200000 { dev_warn((*component).dev, cstr!("Invalid VDDIO voltage\n")); }
    else if ret < 2800000 { *io_voltage_lvl = DA7219_IO_VOLTAGE_LEVEL_1_2V_2_8V as u8; }
    ret = regulator_bulk_enable(DA7219_NUM_SUPPLIES as c_int, (*da7219).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, cstr!("Failed to enable supplies"));
        regulator_bulk_free(DA7219_NUM_SUPPLIES as c_int, (*da7219).supplies.as_mut_ptr());
        return ret;
    }
    0
}

// CONFIG_COMMON_CLK block translated; container_of requires the surrounding kernel layout.
unsafe extern "C" fn da7219_wclk_prepare(_hw: *mut clk_hw) -> c_int { 0 }
unsafe extern "C" fn da7219_wclk_unprepare(_hw: *mut clk_hw) {}
unsafe extern "C" fn da7219_wclk_is_prepared(_hw: *mut clk_hw) -> c_int { 0 }
unsafe extern "C" fn da7219_wclk_recalc_rate(_hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong { 0 }
unsafe extern "C" fn da7219_wclk_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    if (*req).rate < 11025 { (*req).rate = 8000; } else if (*req).rate < 12000 { (*req).rate = 11025; } else if (*req).rate < 16000 { (*req).rate = 12000; } else if (*req).rate < 22050 { (*req).rate = 16000; } else if (*req).rate < 24000 { (*req).rate = 22050; } else if (*req).rate < 32000 { (*req).rate = 24000; } else if (*req).rate < 44100 { (*req).rate = 32000; } else if (*req).rate < 48000 { (*req).rate = 44100; } else if (*req).rate < 88200 { (*req).rate = 48000; } else if (*req).rate < 96000 { (*req).rate = 88200; } else { (*req).rate = 96000; }
    0
}
unsafe extern "C" fn da7219_wclk_set_rate(_hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong) -> c_int { da7219_set_sr(ptr::null_mut(), rate) }
unsafe extern "C" fn da7219_bclk_recalc_rate(_hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong { parent_rate * 32 }
unsafe extern "C" fn da7219_bclk_get_factor(rate: c_ulong, parent_rate: c_ulong) -> c_ulong {
    let factor = rate / parent_rate;
    if factor < 64 { 32 } else if factor < 128 { 64 } else if factor < 256 { 128 } else { 256 }
}
unsafe extern "C" fn da7219_bclk_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    if (*req).best_parent_rate == 0 { return -EINVAL; }
    let factor = da7219_bclk_get_factor((*req).rate, (*req).best_parent_rate);
    (*req).rate = (*req).best_parent_rate * factor;
    0
}
unsafe extern "C" fn da7219_bclk_set_rate(_hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let factor = da7219_bclk_get_factor(rate, parent_rate);
    da7219_set_bclks_per_wclk(ptr::null_mut(), factor)
}

static da7219_dai_clk_ops: [clk_ops; DA7219_DAI_NUM_CLKS as usize] = [
    clk_ops { prepare: Some(da7219_wclk_prepare), unprepare: Some(da7219_wclk_unprepare), is_prepared: Some(da7219_wclk_is_prepared), recalc_rate: Some(da7219_wclk_recalc_rate), determine_rate: Some(da7219_wclk_determine_rate), set_rate: Some(da7219_wclk_set_rate) },
    clk_ops { prepare: None, unprepare: None, is_prepared: None, recalc_rate: Some(da7219_bclk_recalc_rate), determine_rate: Some(da7219_bclk_determine_rate), set_rate: Some(da7219_bclk_set_rate) },
];

unsafe extern "C" fn da7219_register_dai_clks(component: *mut snd_soc_component) -> c_int {
    let dev = (*component).dev;
    let np = (*dev).of_node;
    let da7219 = snd_soc_component_get_drvdata(component);
    let pdata = (*da7219).pdata;
    let mut i: c_int = 0;
    if !np.is_null() {
        (*da7219).clk_hw_data = kzalloc_flex_onecell(DA7219_DAI_NUM_CLKS as usize);
        if (*da7219).clk_hw_data.is_null() { return -ENOMEM; }
        (*(*da7219).clk_hw_data).num = DA7219_DAI_NUM_CLKS;
    }
    while i < DA7219_DAI_NUM_CLKS as c_int {
        let mut init = clk_init_data { name: ptr::null(), ops: ptr::null(), flags: 0, parent_names: ptr::null(), num_parents: 0 };
        let dai_clk_hw = &mut (*da7219).dai_clks_hw[i as usize] as *mut clk_hw;
        let mut parent_name: *const c_char = ptr::null();
        match i as c_uint {
            DA7219_DAI_WCLK_IDX => {
                if !(*da7219).mclk.is_null() { parent_name = __clk_get_name((*da7219).mclk); init.parent_names = &parent_name; init.num_parents = 1; }
            }
            DA7219_DAI_BCLK_IDX => {
                parent_name = __clk_get_name((*da7219).dai_clks[DA7219_DAI_WCLK_IDX as usize]);
                init.parent_names = &parent_name; init.num_parents = 1;
            }
            _ => { dev_err(dev, cstr!("Invalid clock index\n")); break; }
        }
        init.name = (*pdata).dai_clk_names[i as usize];
        init.ops = &da7219_dai_clk_ops[i as usize];
        init.flags = CLK_GET_RATE_NOCACHE | CLK_SET_RATE_GATE;
        (*dai_clk_hw).init = &init;
        let ret = clk_hw_register(dev, dai_clk_hw);
        if ret != 0 { dev_warn(dev, cstr!("Failed to register %s: %d\n"), init.name, ret); return ret; }
        (*da7219).dai_clks[i as usize] = (*dai_clk_hw).clk;
        if !np.is_null() { (*(*da7219).clk_hw_data).hws[i as usize] = dai_clk_hw; }
        else {
            let dai_clk_lookup = clkdev_hw_create(dai_clk_hw, init.name, cstr!("%s"), dev_name(dev));
            if dai_clk_lookup.is_null() { clk_hw_unregister(dai_clk_hw); return -ENOMEM; }
            (*da7219).dai_clks_lookup[i as usize] = dai_clk_lookup;
        }
        i += 1;
    }
    if !np.is_null() {
        let ret = of_clk_add_hw_provider((*dev).of_node, &of_clk_hw_onecell_get, (*da7219).clk_hw_data as *mut c_void);
        if ret != 0 { dev_err(dev, cstr!("Failed to register clock provider\n")); return ret; }
    }
    0
}

unsafe extern "C" fn da7219_free_dai_clks(component: *mut snd_soc_component) {
    let da7219 = snd_soc_component_get_drvdata(component);
    let np = (*(*component).dev).of_node;
    if !np.is_null() { of_clk_del_provider(np); }
    let mut i = DA7219_DAI_NUM_CLKS as c_int - 1;
    while i >= 0 {
        if !(*da7219).dai_clks_lookup[i as usize].is_null() { clkdev_drop((*da7219).dai_clks_lookup[i as usize]); }
        clk_hw_unregister(&mut (*da7219).dai_clks_hw[i as usize]);
        i -= 1;
    }
    if !np.is_null() { kfree((*da7219).clk_hw_data as *mut c_void); }
}

unsafe extern "C" fn da7219_handle_pdata(component: *mut snd_soc_component) {
    let da7219 = snd_soc_component_get_drvdata(component);
    let pdata = (*da7219).pdata;
    if !pdata.is_null() {
        let mut micbias_lvl: u8 = 0;
        (*da7219).wakeup_source = (*pdata).wakeup_source;
        match (*pdata).micbias_lvl {
            DA7219_MICBIAS_1_6V | DA7219_MICBIAS_1_8V | DA7219_MICBIAS_2_0V | DA7219_MICBIAS_2_2V | DA7219_MICBIAS_2_4V | DA7219_MICBIAS_2_6V => {
                micbias_lvl |= ((*pdata).micbias_lvl << DA7219_MICBIAS1_LEVEL_SHIFT) as u8;
            }
            _ => {}
        }
        snd_soc_component_write(component, DA7219_MICBIAS_CTRL, micbias_lvl as c_uint);
        (*da7219).mic_pga_delay = DA7219_MIC_PGA_BASE_DELAY + ((*pdata).micbias_lvl * DA7219_MIC_PGA_OFFSET_DELAY);
        match (*pdata).mic_amp_in_sel {
            DA7219_MIC_AMP_IN_SEL_DIFF | DA7219_MIC_AMP_IN_SEL_SE_P | DA7219_MIC_AMP_IN_SEL_SE_N => {
                snd_soc_component_write(component, DA7219_MIC_1_SELECT, (*pdata).mic_amp_in_sel);
            }
            _ => {}
        }
    }
}

static da7219_reg_defaults: [reg_default; 84] = [
    reg_default { reg: DA7219_MIC_1_SELECT, def: 0x00 }, reg_default { reg: DA7219_CIF_TIMEOUT_CTRL, def: 0x01 }, reg_default { reg: DA7219_SR_24_48, def: 0x00 }, reg_default { reg: DA7219_SR, def: 0x0A },
    reg_default { reg: DA7219_CIF_I2C_ADDR_CFG, def: 0x02 }, reg_default { reg: DA7219_PLL_CTRL, def: 0x10 }, reg_default { reg: DA7219_PLL_FRAC_TOP, def: 0x00 }, reg_default { reg: DA7219_PLL_FRAC_BOT, def: 0x00 },
    reg_default { reg: DA7219_PLL_INTEGER, def: 0x20 }, reg_default { reg: DA7219_DIG_ROUTING_DAI, def: 0x10 }, reg_default { reg: DA7219_DAI_CLK_MODE, def: 0x01 }, reg_default { reg: DA7219_DAI_CTRL, def: 0x28 },
    reg_default { reg: DA7219_DAI_TDM_CTRL, def: 0x40 }, reg_default { reg: DA7219_DIG_ROUTING_DAC, def: 0x32 }, reg_default { reg: DA7219_DAI_OFFSET_LOWER, def: 0x00 }, reg_default { reg: DA7219_DAI_OFFSET_UPPER, def: 0x00 },
    reg_default { reg: DA7219_REFERENCES, def: 0x08 }, reg_default { reg: DA7219_MIXIN_L_SELECT, def: 0x00 }, reg_default { reg: DA7219_MIXIN_L_GAIN, def: 0x03 }, reg_default { reg: DA7219_ADC_L_GAIN, def: 0x6F },
    reg_default { reg: DA7219_ADC_FILTERS1, def: 0x80 }, reg_default { reg: DA7219_MIC_1_GAIN, def: 0x01 }, reg_default { reg: DA7219_SIDETONE_CTRL, def: 0x40 }, reg_default { reg: DA7219_SIDETONE_GAIN, def: 0x0E },
    reg_default { reg: DA7219_DROUTING_ST_OUTFILT_1L, def: 0x01 }, reg_default { reg: DA7219_DROUTING_ST_OUTFILT_1R, def: 0x02 }, reg_default { reg: DA7219_DAC_FILTERS5, def: 0x00 }, reg_default { reg: DA7219_DAC_FILTERS2, def: 0x88 },
    reg_default { reg: DA7219_DAC_FILTERS3, def: 0x88 }, reg_default { reg: DA7219_DAC_FILTERS4, def: 0x08 }, reg_default { reg: DA7219_DAC_FILTERS1, def: 0x80 }, reg_default { reg: DA7219_DAC_L_GAIN, def: 0x6F },
    reg_default { reg: DA7219_DAC_R_GAIN, def: 0x6F }, reg_default { reg: DA7219_CP_CTRL, def: 0x20 }, reg_default { reg: DA7219_HP_L_GAIN, def: 0x39 }, reg_default { reg: DA7219_HP_R_GAIN, def: 0x39 },
    reg_default { reg: DA7219_MIXOUT_L_SELECT, def: 0x00 }, reg_default { reg: DA7219_MIXOUT_R_SELECT, def: 0x00 }, reg_default { reg: DA7219_MICBIAS_CTRL, def: 0x03 }, reg_default { reg: DA7219_MIC_1_CTRL, def: 0x40 },
    reg_default { reg: DA7219_MIXIN_L_CTRL, def: 0x40 }, reg_default { reg: DA7219_ADC_L_CTRL, def: 0x40 }, reg_default { reg: DA7219_DAC_L_CTRL, def: 0x40 }, reg_default { reg: DA7219_DAC_R_CTRL, def: 0x40 },
    reg_default { reg: DA7219_HP_L_CTRL, def: 0x40 }, reg_default { reg: DA7219_HP_R_CTRL, def: 0x40 }, reg_default { reg: DA7219_MIXOUT_L_CTRL, def: 0x10 }, reg_default { reg: DA7219_MIXOUT_R_CTRL, def: 0x10 },
    reg_default { reg: DA7219_CHIP_ID1, def: 0x23 }, reg_default { reg: DA7219_CHIP_ID2, def: 0x93 }, reg_default { reg: DA7219_IO_CTRL, def: 0x00 }, reg_default { reg: DA7219_GAIN_RAMP_CTRL, def: 0x00 },
    reg_default { reg: DA7219_PC_COUNT, def: 0x02 }, reg_default { reg: DA7219_CP_VOL_THRESHOLD1, def: 0x0E }, reg_default { reg: DA7219_DIG_CTRL, def: 0x00 }, reg_default { reg: DA7219_ALC_CTRL2, def: 0x00 },
    reg_default { reg: DA7219_ALC_CTRL3, def: 0x00 }, reg_default { reg: DA7219_ALC_NOISE, def: 0x3F }, reg_default { reg: DA7219_ALC_TARGET_MIN, def: 0x3F }, reg_default { reg: DA7219_ALC_TARGET_MAX, def: 0x00 },
    reg_default { reg: DA7219_ALC_GAIN_LIMITS, def: 0xFF }, reg_default { reg: DA7219_ALC_ANA_GAIN_LIMITS, def: 0x71 }, reg_default { reg: DA7219_ALC_ANTICLIP_CTRL, def: 0x00 }, reg_default { reg: DA7219_ALC_ANTICLIP_LEVEL, def: 0x00 },
    reg_default { reg: DA7219_DAC_NG_SETUP_TIME, def: 0x00 }, reg_default { reg: DA7219_DAC_NG_OFF_THRESH, def: 0x00 }, reg_default { reg: DA7219_DAC_NG_ON_THRESH, def: 0x00 }, reg_default { reg: DA7219_DAC_NG_CTRL, def: 0x00 },
    reg_default { reg: DA7219_TONE_GEN_CFG1, def: 0x00 }, reg_default { reg: DA7219_TONE_GEN_CFG2, def: 0x00 }, reg_default { reg: DA7219_TONE_GEN_CYCLES, def: 0x00 }, reg_default { reg: DA7219_TONE_GEN_FREQ1_L, def: 0x55 },
    reg_default { reg: DA7219_TONE_GEN_FREQ1_U, def: 0x15 }, reg_default { reg: DA7219_TONE_GEN_FREQ2_L, def: 0x00 }, reg_default { reg: DA7219_TONE_GEN_FREQ2_U, def: 0x40 }, reg_default { reg: DA7219_TONE_GEN_ON_PER, def: 0x02 },
    reg_default { reg: DA7219_TONE_GEN_OFF_PER, def: 0x01 }, reg_default { reg: DA7219_ACCDET_IRQ_MASK_A, def: 0x00 }, reg_default { reg: DA7219_ACCDET_IRQ_MASK_B, def: 0x00 }, reg_default { reg: DA7219_ACCDET_CONFIG_1, def: 0xD6 },
    reg_default { reg: DA7219_ACCDET_CONFIG_2, def: 0x34 }, reg_default { reg: DA7219_ACCDET_CONFIG_3, def: 0x0A }, reg_default { reg: DA7219_ACCDET_CONFIG_4, def: 0x16 }, reg_default { reg: DA7219_ACCDET_CONFIG_5, def: 0x21 },
    reg_default { reg: DA7219_ACCDET_CONFIG_6, def: 0x3E }, reg_default { reg: DA7219_ACCDET_CONFIG_7, def: 0x01 }, reg_default { reg: DA7219_SYSTEM_ACTIVE, def: 0x00 }, reg_default { reg: 0, def: 0 },
];

unsafe extern "C" fn da7219_volatile_register(_dev: *mut device, reg: c_uint) -> bool_ {
    matches!(reg, DA7219_MIC_1_GAIN_STATUS | DA7219_MIXIN_L_GAIN_STATUS | DA7219_ADC_L_GAIN_STATUS | DA7219_DAC_L_GAIN_STATUS | DA7219_DAC_R_GAIN_STATUS | DA7219_HP_L_GAIN_STATUS | DA7219_HP_R_GAIN_STATUS | DA7219_CIF_CTRL | DA7219_PLL_SRM_STS | DA7219_ALC_CTRL1 | DA7219_SYSTEM_MODES_INPUT | DA7219_SYSTEM_MODES_OUTPUT | DA7219_ALC_OFFSET_AUTO_M_L | DA7219_ALC_OFFSET_AUTO_U_L | DA7219_TONE_GEN_CFG1 | DA7219_ACCDET_STATUS_A | DA7219_ACCDET_STATUS_B | DA7219_ACCDET_IRQ_EVENT_A | DA7219_ACCDET_IRQ_EVENT_B | DA7219_ACCDET_CONFIG_8 | DA7219_SYSTEM_STATUS)
}

static da7219_regmap_config: regmap_config = regmap_config { reg_bits: 8, val_bits: 8, max_register: DA7219_SYSTEM_ACTIVE, reg_defaults: da7219_reg_defaults.as_ptr(), num_reg_defaults: ARRAY_SIZE!(da7219_reg_defaults), volatile_reg: Some(da7219_volatile_register), cache_type: REGCACHE_RBTREE };
static da7219_rev_aa_patch: [reg_sequence; 1] = [reg_sequence { reg: DA7219_REFERENCES, def: 0x08 }];

unsafe extern "C" fn da7219_probe(component: *mut snd_soc_component) -> c_int {
    let da7219 = snd_soc_component_get_drvdata(component);
    let mut system_active: c_uint = 0;
    let mut system_status: c_uint = 0;
    let mut rev: c_uint = 0;
    let mut io_voltage_lvl: u8 = 0;
    (*da7219).component = component;
    mutex_init(&mut (*da7219).ctrl_lock); mutex_init(&mut (*da7219).pll_lock);
    let mut ret = da7219_handle_supplies(component, &mut io_voltage_lvl);
    if ret != 0 { return ret; }
    regcache_cache_bypass((*da7219).regmap, true);
    regmap_read((*da7219).regmap, DA7219_SYSTEM_ACTIVE, &mut system_active);
    if system_active != 0 {
        regmap_write((*da7219).regmap, DA7219_GAIN_RAMP_CTRL, DA7219_GAIN_RAMP_RATE_NOMINAL);
        regmap_write((*da7219).regmap, DA7219_SYSTEM_MODES_INPUT, 0x00);
        regmap_write((*da7219).regmap, DA7219_SYSTEM_MODES_OUTPUT, 0x01);
        for _i in 0..DA7219_SYS_STAT_CHECK_RETRIES {
            regmap_read((*da7219).regmap, DA7219_SYSTEM_STATUS, &mut system_status);
            if system_status == 0 { break; }
            msleep(DA7219_SYS_STAT_CHECK_DELAY);
        }
    }
    regmap_write_bits((*da7219).regmap, DA7219_ACCDET_CONFIG_1, DA7219_ACCDET_EN_MASK, 0);
    regmap_write_bits((*da7219).regmap, DA7219_CIF_CTRL, DA7219_CIF_REG_SOFT_RESET_MASK, DA7219_CIF_REG_SOFT_RESET_MASK);
    regmap_write_bits((*da7219).regmap, DA7219_SYSTEM_ACTIVE, DA7219_SYSTEM_ACTIVE_MASK, 0);
    regmap_write_bits((*da7219).regmap, DA7219_SYSTEM_ACTIVE, DA7219_SYSTEM_ACTIVE_MASK, 1);
    regcache_cache_bypass((*da7219).regmap, false);
    regmap_reinit_cache((*da7219).regmap, &da7219_regmap_config);
    snd_soc_component_write(component, DA7219_IO_CTRL, io_voltage_lvl as c_uint);
    ret = regmap_read((*da7219).regmap, DA7219_CHIP_REVISION, &mut rev);
    if ret != 0 { dev_err((*component).dev, cstr!("Failed to read chip revision: %d\n"), ret); goto_err_disable_reg(component, da7219); return ret; }
    if (rev & DA7219_CHIP_MINOR_MASK) == 0 {
        ret = regmap_register_patch((*da7219).regmap, da7219_rev_aa_patch.as_ptr(), ARRAY_SIZE!(da7219_rev_aa_patch) as c_int);
        if ret != 0 { dev_err((*component).dev, cstr!("Failed to register AA patch: %d\n"), ret); goto_err_disable_reg(component, da7219); return ret; }
    }
    da7219_handle_pdata(component);
    (*da7219).mclk = clk_get((*component).dev, cstr!("mclk"));
    if IS_ERR((*da7219).mclk as *const c_void) {
        if PTR_ERR((*da7219).mclk as *const c_void) != -ENOENT { ret = PTR_ERR((*da7219).mclk as *const c_void); goto_err_disable_reg(component, da7219); return ret; }
        else { (*da7219).mclk = ptr::null_mut(); }
    }
    ret = da7219_register_dai_clks(component);
    if ret != 0 { clk_put((*da7219).mclk); goto_err_disable_reg(component, da7219); return ret; }
    snd_soc_component_update_bits(component, DA7219_PC_COUNT, DA7219_PC_FREERUN_MASK, DA7219_PC_FREERUN_MASK);
    snd_soc_component_update_bits(component, DA7219_MIXIN_L_CTRL, DA7219_MIXIN_L_AMP_RAMP_EN_MASK, DA7219_MIXIN_L_AMP_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_ADC_L_CTRL, DA7219_ADC_L_RAMP_EN_MASK, DA7219_ADC_L_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_DAC_L_CTRL, DA7219_DAC_L_RAMP_EN_MASK, DA7219_DAC_L_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_DAC_R_CTRL, DA7219_DAC_R_RAMP_EN_MASK, DA7219_DAC_R_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_HP_L_CTRL, DA7219_HP_L_AMP_RAMP_EN_MASK, DA7219_HP_L_AMP_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_HP_R_CTRL, DA7219_HP_R_AMP_RAMP_EN_MASK, DA7219_HP_R_AMP_RAMP_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_HP_L_CTRL, DA7219_HP_L_AMP_MIN_GAIN_EN_MASK, DA7219_HP_L_AMP_MIN_GAIN_EN_MASK);
    snd_soc_component_update_bits(component, DA7219_HP_R_CTRL, DA7219_HP_R_AMP_MIN_GAIN_EN_MASK, DA7219_HP_R_AMP_MIN_GAIN_EN_MASK);
    snd_soc_component_write(component, DA7219_TONE_GEN_CYCLES, DA7219_BEEP_CYCLES_MASK);
    ret = da7219_aad_init(component);
    if ret != 0 { da7219_free_dai_clks(component); clk_put((*da7219).mclk); goto_err_disable_reg(component, da7219); return ret; }
    0
}

unsafe fn goto_err_disable_reg(_component: *mut snd_soc_component, da7219: *mut da7219_priv) {
    regulator_bulk_disable(DA7219_NUM_SUPPLIES as c_int, (*da7219).supplies.as_mut_ptr());
    regulator_bulk_free(DA7219_NUM_SUPPLIES as c_int, (*da7219).supplies.as_mut_ptr());
}

unsafe extern "C" fn da7219_remove(component: *mut snd_soc_component) {
    let da7219 = snd_soc_component_get_drvdata(component);
    da7219_aad_exit(component);
    da7219_free_dai_clks(component);
    clk_put((*da7219).mclk);
    regulator_bulk_disable(DA7219_NUM_SUPPLIES as c_int, (*da7219).supplies.as_mut_ptr());
    regulator_bulk_free(DA7219_NUM_SUPPLIES as c_int, (*da7219).supplies.as_mut_ptr());
}

unsafe extern "C" fn da7219_suspend(component: *mut snd_soc_component) -> c_int {
    let da7219 = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);
    if !(*da7219).wakeup_source { da7219_aad_suspend(component); }
    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_OFF);
    0
}

unsafe extern "C" fn da7219_resume(component: *mut snd_soc_component) -> c_int {
    let da7219 = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);
    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_STANDBY);
    if !(*da7219).wakeup_source { da7219_aad_resume(component); }
    0
}

unsafe extern "C" fn da7219_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    da7219_aad_jack_det(component, jack);
    0
}

static soc_component_dev_da7219: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(da7219_probe), remove: Some(da7219_remove), suspend: Some(da7219_suspend), resume: Some(da7219_resume),
    set_jack: Some(da7219_set_jack), set_bias_level: Some(da7219_set_bias_level),
    controls: da7219_snd_controls.as_ptr(), num_controls: ARRAY_SIZE!(da7219_snd_controls),
    dapm_widgets: da7219_dapm_widgets.as_ptr(), num_dapm_widgets: ARRAY_SIZE!(da7219_dapm_widgets),
    dapm_routes: da7219_audio_map.as_ptr(), num_dapm_routes: ARRAY_SIZE!(da7219_audio_map),
    idle_bias_on: 1, use_pmdown_time: 1, endianness: 1,
};

unsafe extern "C" fn da7219_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let da7219 = devm_kzalloc(dev, size_of::<da7219_priv>(), GFP_KERNEL) as *mut da7219_priv;
    if da7219.is_null() { return -ENOMEM; }
    i2c_set_clientdata(i2c, da7219 as *mut c_void);
    (*da7219).regmap = devm_regmap_init_i2c(i2c, &da7219_regmap_config);
    if IS_ERR((*da7219).regmap as *const c_void) {
        let ret = PTR_ERR((*da7219).regmap as *const c_void);
        dev_err(dev, cstr!("regmap_init() failed: %d\n"), ret);
        return ret;
    }
    (*da7219).pdata = dev_get_platdata(dev);
    if (*da7219).pdata.is_null() { (*da7219).pdata = da7219_fw_to_pdata(dev); }
    let mut ret = da7219_aad_probe(i2c);
    if ret != 0 { return ret; }
    ret = devm_snd_soc_register_component(dev, &soc_component_dev_da7219, &mut da7219_dai, 1);
    if ret < 0 { dev_err(dev, cstr!("Failed to register da7219 component: %d\n"), ret); }
    ret
}

static da7219_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [b'd' as c_char, b'a' as c_char, b'7' as c_char, b'2' as c_char, b'1' as c_char, b'9' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    i2c_device_id { name: [0; 20] },
];

#[repr(C)] pub struct i2c_driver { pub driver: device_driver, pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>, pub id_table: *const i2c_device_id }
#[repr(C)] pub struct device_driver { pub name: *const c_char, pub of_match_table: *const of_device_id, pub acpi_match_table: *const acpi_device_id }

static mut da7219_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver { name: cstr!("da7219"), of_match_table: ptr::null(), acpi_match_table: ptr::null() },
    probe: Some(da7219_i2c_probe),
    id_table: da7219_i2c_id.as_ptr(),
};

// module_i2c_driver!(da7219_i2c_driver);
// MODULE_DESCRIPTION("ASoC DA7219 Codec Driver");
// MODULE_AUTHOR("Adam Thomson <Adam.Thomson.Opensource@diasemi.com>");
// MODULE_LICENSE("GPL");


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
