// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for ADAU1361/ADAU1461/ADAU1761/ADAU1961 codec
 *
 * Copyright 2011-2013 Analog Devices Inc.
 * Author: Lars-Peter Clausen <lars@metafoo.de>
 */

// Dependencies from Linux, ALSA SoC, adau17x1.h, and adau1761.h are external.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;

const EINVAL: c_int = 22;

const ADAU1761_DIGMIC_JACKDETECT: c_uint = 0x4008;
const ADAU1761_REC_MIXER_LEFT0: c_uint = 0x400a;
const ADAU1761_REC_MIXER_LEFT1: c_uint = 0x400b;
const ADAU1761_REC_MIXER_RIGHT0: c_uint = 0x400c;
const ADAU1761_REC_MIXER_RIGHT1: c_uint = 0x400d;
const ADAU1761_LEFT_DIFF_INPUT_VOL: c_uint = 0x400e;
const ADAU1761_RIGHT_DIFF_INPUT_VOL: c_uint = 0x400f;
const ADAU1761_ALC_CTRL0: c_uint = 0x4011;
const ADAU1761_ALC_CTRL1: c_uint = 0x4012;
const ADAU1761_ALC_CTRL2: c_uint = 0x4013;
const ADAU1761_ALC_CTRL3: c_uint = 0x4014;
const ADAU1761_PLAY_LR_MIXER_LEFT: c_uint = 0x4020;
const ADAU1761_PLAY_MIXER_LEFT0: c_uint = 0x401c;
const ADAU1761_PLAY_MIXER_LEFT1: c_uint = 0x401d;
const ADAU1761_PLAY_MIXER_RIGHT0: c_uint = 0x401e;
const ADAU1761_PLAY_MIXER_RIGHT1: c_uint = 0x401f;
const ADAU1761_PLAY_LR_MIXER_RIGHT: c_uint = 0x4021;
const ADAU1761_PLAY_MIXER_MONO: c_uint = 0x4022;
const ADAU1761_PLAY_HP_LEFT_VOL: c_uint = 0x4023;
const ADAU1761_PLAY_HP_RIGHT_VOL: c_uint = 0x4024;
const ADAU1761_PLAY_LINE_LEFT_VOL: c_uint = 0x4025;
const ADAU1761_PLAY_LINE_RIGHT_VOL: c_uint = 0x4026;
const ADAU1761_PLAY_MONO_OUTPUT_VOL: c_uint = 0x4027;
const ADAU1761_POP_CLICK_SUPPRESS: c_uint = 0x4028;
const ADAU1761_JACK_DETECT_PIN: c_uint = 0x4031;
const ADAU1761_DEJITTER: c_uint = 0x4036;
const ADAU1761_CLK_ENABLE0: c_uint = 0x40f9;
const ADAU1761_CLK_ENABLE1: c_uint = 0x40fa;

const ADAU1761_DIGMIC_JACKDETECT_ACTIVE_LOW: c_uint = 1 << 0;
const ADAU1761_DIGMIC_JACKDETECT_DIGMIC: c_uint = 1 << 5;

const ADAU1761_DIFF_INPUT_VOL_LDEN: c_uint = 1 << 0;

const ADAU1761_PLAY_MONO_OUTPUT_VOL_MODE_HP: c_uint = 1 << 0;
const ADAU1761_PLAY_MONO_OUTPUT_VOL_UNMUTE: c_uint = 1 << 1;

const ADAU1761_PLAY_HP_RIGHT_VOL_MODE_HP: c_uint = 1 << 0;

const ADAU1761_PLAY_LINE_LEFT_VOL_MODE_HP: c_uint = 1 << 0;

const ADAU1761_PLAY_LINE_RIGHT_VOL_MODE_HP: c_uint = 1 << 0;

const ADAU1761_FIRMWARE: *const c_char = b"adau1761.bin\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level:
        Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub suspend_bias_off: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const c_void,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub val_bits: c_uint,
    pub reg_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct adau {
    pub regmap: *mut regmap,
    pub master: bool_,
    pub type_: adau17x1_type,
}

#[repr(C)]
pub struct adau1761_platform_data {
    pub input_differential: bool_,
    pub lineout_mode: adau1761_output_mode,
    pub headphone_mode: adau1761_output_mode,
    pub digmic_jackdetect_pin_mode: adau1761_digmic_jackdet_pin_mode,
    pub jackdetect_debounce_time: adau1761_jackdetect_debounce_time,
    pub jackdetect_active_low: bool_,
}

pub type adau17x1_type = c_uint;
pub type adau1761_output_mode = c_uint;
pub type adau1761_digmic_jackdet_pin_mode = c_uint;
pub type adau1761_jackdetect_debounce_time = c_uint;
pub type snd_soc_bias_level = c_uint;

extern "C" {
    static adau17x1_dai_ops: c_void;

    static ADAU17X1_CLOCK_CONTROL: c_uint;
    static ADAU17X1_PLL_CONTROL: c_uint;
    static ADAU17X1_REC_POWER_MGMT: c_uint;
    static ADAU17X1_MICBIAS: c_uint;
    static ADAU17X1_SERIAL_PORT0: c_uint;
    static ADAU17X1_SERIAL_PORT1: c_uint;
    static ADAU17X1_CONVERTER0: c_uint;
    static ADAU17X1_CONVERTER1: c_uint;
    static ADAU17X1_ADC_CONTROL: c_uint;
    static ADAU17X1_LEFT_INPUT_DIGITAL_VOL: c_uint;
    static ADAU17X1_RIGHT_INPUT_DIGITAL_VOL: c_uint;
    static ADAU17X1_PLAY_POWER_MGMT: c_uint;
    static ADAU17X1_DAC_CONTROL0: c_uint;
    static ADAU17X1_DAC_CONTROL1: c_uint;
    static ADAU17X1_DAC_CONTROL2: c_uint;
    static ADAU17X1_SERIAL_PORT_PAD: c_uint;
    static ADAU17X1_CONTROL_PORT_PAD0: c_uint;
    static ADAU17X1_CONTROL_PORT_PAD1: c_uint;
    static ADAU17X1_DSP_SAMPLING_RATE: c_uint;
    static ADAU17X1_SERIAL_INPUT_ROUTE: c_uint;
    static ADAU17X1_SERIAL_OUTPUT_ROUTE: c_uint;
    static ADAU17X1_DSP_ENABLE: c_uint;
    static ADAU17X1_DSP_RUN: c_uint;
    static ADAU17X1_SERIAL_SAMPLING_RATE: c_uint;
    static ADAU17X1_CLOCK_CONTROL_SYSCLK_EN: c_uint;

    static ADAU1361: adau17x1_type;
    static ADAU1761: adau17x1_type;
    static ADAU1761_AS_1361: adau17x1_type;
    static ADAU1761_OUTPUT_MODE_LINE: adau1761_output_mode;
    static ADAU1761_OUTPUT_MODE_HEADPHONE: adau1761_output_mode;
    static ADAU1761_OUTPUT_MODE_HEADPHONE_CAPLESS: adau1761_output_mode;
    static ADAU1761_DIGMIC_JACKDET_PIN_MODE_NONE: adau1761_digmic_jackdet_pin_mode;
    static ADAU1761_DIGMIC_JACKDET_PIN_MODE_JACKDETECT: adau1761_digmic_jackdet_pin_mode;
    static ADAU1761_DIGMIC_JACKDET_PIN_MODE_DIGMIC: adau1761_digmic_jackdet_pin_mode;
    static ADAU1761_JACKDETECT_DEBOUNCE_5MS: adau1761_jackdetect_debounce_time;
    static ADAU1761_JACKDETECT_DEBOUNCE_10MS: adau1761_jackdetect_debounce_time;
    static ADAU1761_JACKDETECT_DEBOUNCE_20MS: adau1761_jackdetect_debounce_time;
    static ADAU1761_JACKDETECT_DEBOUNCE_40MS: adau1761_jackdetect_debounce_time;

    static SND_SOC_BIAS_ON: snd_soc_bias_level;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;
    static SND_SOC_NOPM: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static REGCACHE_MAPLE: c_uint;

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_bypass(map: *mut regmap, enable: bool_);
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        routes: *const snd_soc_dapm_route,
        num_routes: c_uint,
    ) -> c_int;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widgets: *const snd_soc_dapm_widget,
        num_widgets: c_uint,
    ) -> c_int;
    fn adau17x1_readable_register(dev: *mut device, reg: c_uint) -> bool_;
    fn adau17x1_volatile_register(dev: *mut device, reg: c_uint) -> bool_;
    fn adau17x1_precious_register(dev: *mut device, reg: c_uint) -> bool_;
    fn adau17x1_add_widgets(component: *mut snd_soc_component) -> c_int;
    fn adau17x1_add_routes(component: *mut snd_soc_component) -> c_int;
    fn adau17x1_resume(component: *mut snd_soc_component) -> c_int;
    fn adau17x1_probe(
        dev: *mut device,
        regmap: *mut regmap,
        type_: adau17x1_type,
        switch_mode: Option<unsafe extern "C" fn(*mut device)>,
        firmware_name: *const c_char,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        ($array.len() as c_uint)
    };
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

// ALSA SoC declaration macros are external C macro dependencies preserved here.
macro_rules! DECLARE_TLV_DB_SCALE { ($name:ident, $min:expr, $step:expr, $mute:expr) => { [0u32; 0] }; }
macro_rules! SOC_ENUM_SINGLE_DECL { ($($t:tt)*) => { () }; }
macro_rules! SOC_VALUE_ENUM_SINGLE_DECL { ($($t:tt)*) => { () }; }
macro_rules! SOC_SINGLE { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DOUBLE_R { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DOUBLE_R_TLV { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_SINGLE_TLV { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_ENUM { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DAPM_SINGLE_AUTODISABLE { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DAPM_SINGLE_TLV { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DAPM_ENUM { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SND_SOC_DAPM_MIXER { ($($t:tt)*) => { snd_soc_dapm_widget { dapm: ptr::null_mut() } }; }
macro_rules! SOC_MIXER_ARRAY { ($($t:tt)*) => { snd_soc_dapm_widget { dapm: ptr::null_mut() } }; }
macro_rules! SND_SOC_DAPM_SUPPLY { ($($t:tt)*) => { snd_soc_dapm_widget { dapm: ptr::null_mut() } }; }
macro_rules! SND_SOC_DAPM_SUPPLY_S { ($($t:tt)*) => { snd_soc_dapm_widget { dapm: ptr::null_mut() } }; }
macro_rules! SND_SOC_DAPM_POST { ($($t:tt)*) => { snd_soc_dapm_widget { dapm: ptr::null_mut() } }; }
macro_rules! SND_SOC_DAPM_INPUT { ($($t:tt)*) => { snd_soc_dapm_widget { dapm: ptr::null_mut() } }; }
macro_rules! SND_SOC_DAPM_OUTPUT { ($($t:tt)*) => { snd_soc_dapm_widget { dapm: ptr::null_mut() } }; }
macro_rules! SND_SOC_DAPM_MUX { ($($t:tt)*) => { snd_soc_dapm_widget { dapm: ptr::null_mut() } }; }

static adau1761_reg_defaults: &[reg_default] = &[
    reg_default { reg: unsafe { ADAU17X1_CLOCK_CONTROL }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_PLL_CONTROL }, def: 0x00 },
    reg_default { reg: ADAU1761_DIGMIC_JACKDETECT, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_REC_POWER_MGMT }, def: 0x00 },
    reg_default { reg: ADAU1761_REC_MIXER_LEFT0, def: 0x00 },
    reg_default { reg: ADAU1761_REC_MIXER_LEFT1, def: 0x00 },
    reg_default { reg: ADAU1761_REC_MIXER_RIGHT0, def: 0x00 },
    reg_default { reg: ADAU1761_REC_MIXER_RIGHT1, def: 0x00 },
    reg_default { reg: ADAU1761_LEFT_DIFF_INPUT_VOL, def: 0x00 },
    reg_default { reg: ADAU1761_RIGHT_DIFF_INPUT_VOL, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_MICBIAS }, def: 0x00 },
    reg_default { reg: ADAU1761_ALC_CTRL0, def: 0x00 },
    reg_default { reg: ADAU1761_ALC_CTRL1, def: 0x00 },
    reg_default { reg: ADAU1761_ALC_CTRL2, def: 0x00 },
    reg_default { reg: ADAU1761_ALC_CTRL3, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_SERIAL_PORT0 }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_SERIAL_PORT1 }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_CONVERTER0 }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_CONVERTER1 }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_ADC_CONTROL }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_LEFT_INPUT_DIGITAL_VOL }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_RIGHT_INPUT_DIGITAL_VOL }, def: 0x00 },
    reg_default { reg: ADAU1761_PLAY_MIXER_LEFT0, def: 0x00 },
    reg_default { reg: ADAU1761_PLAY_MIXER_LEFT1, def: 0x00 },
    reg_default { reg: ADAU1761_PLAY_MIXER_RIGHT0, def: 0x00 },
    reg_default { reg: ADAU1761_PLAY_MIXER_RIGHT1, def: 0x00 },
    reg_default { reg: ADAU1761_PLAY_LR_MIXER_LEFT, def: 0x00 },
    reg_default { reg: ADAU1761_PLAY_LR_MIXER_RIGHT, def: 0x00 },
    reg_default { reg: ADAU1761_PLAY_MIXER_MONO, def: 0x00 },
    reg_default { reg: ADAU1761_PLAY_HP_LEFT_VOL, def: 0x00 },
    reg_default { reg: ADAU1761_PLAY_HP_RIGHT_VOL, def: 0x00 },
    reg_default { reg: ADAU1761_PLAY_LINE_LEFT_VOL, def: 0x00 },
    reg_default { reg: ADAU1761_PLAY_LINE_RIGHT_VOL, def: 0x00 },
    reg_default { reg: ADAU1761_PLAY_MONO_OUTPUT_VOL, def: 0x00 },
    reg_default { reg: ADAU1761_POP_CLICK_SUPPRESS, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_PLAY_POWER_MGMT }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_DAC_CONTROL0 }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_DAC_CONTROL1 }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_DAC_CONTROL2 }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_SERIAL_PORT_PAD }, def: 0xaa },
    reg_default { reg: unsafe { ADAU17X1_CONTROL_PORT_PAD0 }, def: 0xaa },
    reg_default { reg: unsafe { ADAU17X1_CONTROL_PORT_PAD1 }, def: 0x00 },
    reg_default { reg: ADAU1761_JACK_DETECT_PIN, def: 0x00 },
    reg_default { reg: ADAU1761_DEJITTER, def: 0x03 },
    reg_default { reg: unsafe { ADAU17X1_DSP_SAMPLING_RATE }, def: 0x01 },
    reg_default { reg: unsafe { ADAU17X1_SERIAL_INPUT_ROUTE }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_SERIAL_OUTPUT_ROUTE }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_DSP_ENABLE }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_DSP_RUN }, def: 0x00 },
    reg_default { reg: unsafe { ADAU17X1_SERIAL_SAMPLING_RATE }, def: 0x00 },
    reg_default { reg: ADAU1761_CLK_ENABLE0, def: 0x00 },
    reg_default { reg: ADAU1761_CLK_ENABLE1, def: 0x00 },
];

static adau1761_sing_in_tlv: [u32; 0] = DECLARE_TLV_DB_SCALE!(adau1761_sing_in_tlv, -1500, 300, 1);
static adau1761_diff_in_tlv: [u32; 0] = DECLARE_TLV_DB_SCALE!(adau1761_diff_in_tlv, -1200, 75, 0);
static adau1761_out_tlv: [u32; 0] = DECLARE_TLV_DB_SCALE!(adau1761_out_tlv, -5700, 100, 0);
static adau1761_sidetone_tlv: [u32; 0] = DECLARE_TLV_DB_SCALE!(adau1761_sidetone_tlv, -1800, 300, 1);
static adau1761_boost_tlv: [u32; 0] = DECLARE_TLV_DB_SCALE!(adau1761_boost_tlv, -600, 600, 1);
static adau1761_pga_boost_tlv: [u32; 0] = DECLARE_TLV_DB_SCALE!(adau1761_pga_boost_tlv, -2000, 2000, 1);
static adau1761_alc_max_gain_tlv: [u32; 0] = DECLARE_TLV_DB_SCALE!(adau1761_alc_max_gain_tlv, -1200, 600, 0);
static adau1761_alc_target_tlv: [u32; 0] = DECLARE_TLV_DB_SCALE!(adau1761_alc_target_tlv, -2850, 150, 0);
static adau1761_alc_ng_threshold_tlv: [u32; 0] = DECLARE_TLV_DB_SCALE!(adau1761_alc_ng_threshold_tlv, -7650, 150, 0);

static adau1761_bias_select_values: [c_uint; 3] = [0, 2, 3];
static adau1761_bias_select_text: [*const c_char; 3] = [
    cstr!("Normal operation"),
    cstr!("Enhanced performance"),
    cstr!("Power saving"),
];
static adau1761_bias_select_extreme_text: [*const c_char; 4] = [
    cstr!("Normal operation"),
    cstr!("Extreme power saving"),
    cstr!("Enhanced performance"),
    cstr!("Power saving"),
];

static adau1761_adc_bias_enum: () = SOC_ENUM_SINGLE_DECL!(adau1761_adc_bias_enum, ADAU17X1_REC_POWER_MGMT, 3, adau1761_bias_select_extreme_text);
static adau1761_hp_bias_enum: () = SOC_ENUM_SINGLE_DECL!(adau1761_hp_bias_enum, ADAU17X1_PLAY_POWER_MGMT, 6, adau1761_bias_select_extreme_text);
static adau1761_dac_bias_enum: () = SOC_ENUM_SINGLE_DECL!(adau1761_dac_bias_enum, ADAU17X1_PLAY_POWER_MGMT, 4, adau1761_bias_select_extreme_text);
static adau1761_playback_bias_enum: () = SOC_VALUE_ENUM_SINGLE_DECL!(adau1761_playback_bias_enum, ADAU17X1_PLAY_POWER_MGMT, 2, 0x3, adau1761_bias_select_text, adau1761_bias_select_values);
static adau1761_capture_bias_enum: () = SOC_VALUE_ENUM_SINGLE_DECL!(adau1761_capture_bias_enum, ADAU17X1_REC_POWER_MGMT, 1, 0x3, adau1761_bias_select_text, adau1761_bias_select_values);

static adau1761_pga_slew_time_values: [c_uint; 4] = [3, 0, 1, 2];
static adau1761_pga_slew_time_text: [*const c_char; 4] = [cstr!("Off"), cstr!("24 ms"), cstr!("48 ms"), cstr!("96 ms")];
static adau1761_alc_function_text: [*const c_char; 5] = [cstr!("Off"), cstr!("Right"), cstr!("Left"), cstr!("Stereo"), cstr!("DSP control")];
static adau1761_alc_hold_time_text: [*const c_char; 16] = [
    cstr!("2.67 ms"), cstr!("5.34 ms"), cstr!("10.68 ms"), cstr!("21.36 ms"),
    cstr!("42.72 ms"), cstr!("85.44 ms"), cstr!("170.88 ms"), cstr!("341.76 ms"),
    cstr!("683.52 ms"), cstr!("1367 ms"), cstr!("2734.1 ms"), cstr!("5468.2 ms"),
    cstr!("10936 ms"), cstr!("21873 ms"), cstr!("43745 ms"), cstr!("87491 ms"),
];
static adau1761_alc_attack_time_text: [*const c_char; 16] = [
    cstr!("6 ms"), cstr!("12 ms"), cstr!("24 ms"), cstr!("48 ms"),
    cstr!("96 ms"), cstr!("192 ms"), cstr!("384 ms"), cstr!("768 ms"),
    cstr!("1540 ms"), cstr!("3070 ms"), cstr!("6140 ms"), cstr!("12290 ms"),
    cstr!("24580 ms"), cstr!("49150 ms"), cstr!("98300 ms"), cstr!("196610 ms"),
];
static adau1761_alc_decay_time_text: [*const c_char; 16] = [
    cstr!("24 ms"), cstr!("48 ms"), cstr!("96 ms"), cstr!("192 ms"),
    cstr!("384 ms"), cstr!("768 ms"), cstr!("15400 ms"), cstr!("30700 ms"),
    cstr!("61400 ms"), cstr!("12290 ms"), cstr!("24580 ms"), cstr!("49150 ms"),
    cstr!("98300 ms"), cstr!("196610 ms"), cstr!("393220 ms"), cstr!("786430 ms"),
];
static adau1761_alc_ng_type_text: [*const c_char; 4] = [cstr!("Hold"), cstr!("Mute"), cstr!("Fade"), cstr!("Fade + Mute")];

static adau1761_pga_slew_time_enum: () = SOC_VALUE_ENUM_SINGLE_DECL!(adau1761_pga_slew_time_enum, ADAU1761_ALC_CTRL0, 6, 0x3, adau1761_pga_slew_time_text, adau1761_pga_slew_time_values);
static adau1761_alc_function_enum: () = SOC_ENUM_SINGLE_DECL!(adau1761_alc_function_enum, ADAU1761_ALC_CTRL0, 0, adau1761_alc_function_text);
static adau1761_alc_hold_time_enum: () = SOC_ENUM_SINGLE_DECL!(adau1761_alc_hold_time_enum, ADAU1761_ALC_CTRL1, 4, adau1761_alc_hold_time_text);
static adau1761_alc_attack_time_enum: () = SOC_ENUM_SINGLE_DECL!(adau1761_alc_attack_time_enum, ADAU1761_ALC_CTRL2, 4, adau1761_alc_attack_time_text);
static adau1761_alc_decay_time_enum: () = SOC_ENUM_SINGLE_DECL!(adau1761_alc_decay_time_enum, ADAU1761_ALC_CTRL2, 0, adau1761_alc_decay_time_text);
static adau1761_alc_ng_type_enum: () = SOC_ENUM_SINGLE_DECL!(adau1761_alc_ng_type_enum, ADAU1761_ALC_CTRL3, 6, adau1761_alc_ng_type_text);

static adau1761_jack_detect_controls: [snd_kcontrol_new; 1] = [
    SOC_SINGLE!("Speaker Auto-mute Switch", ADAU1761_DIGMIC_JACKDETECT, 4, 1, 0),
];

static adau1761_differential_mode_controls: [snd_kcontrol_new; 13] = [
    SOC_DOUBLE_R_TLV!("Capture Volume", ADAU1761_LEFT_DIFF_INPUT_VOL, ADAU1761_RIGHT_DIFF_INPUT_VOL, 2, 0x3f, 0, adau1761_diff_in_tlv),
    SOC_DOUBLE_R!("Capture Switch", ADAU1761_LEFT_DIFF_INPUT_VOL, ADAU1761_RIGHT_DIFF_INPUT_VOL, 1, 1, 0),
    SOC_DOUBLE_R_TLV!("PGA Boost Capture Volume", ADAU1761_REC_MIXER_LEFT1, ADAU1761_REC_MIXER_RIGHT1, 3, 2, 0, adau1761_pga_boost_tlv),
    SOC_ENUM!("PGA Capture Slew Time", adau1761_pga_slew_time_enum),
    SOC_SINGLE_TLV!("ALC Capture Max Gain Volume", ADAU1761_ALC_CTRL0, 3, 7, 0, adau1761_alc_max_gain_tlv),
    SOC_ENUM!("ALC Capture Function", adau1761_alc_function_enum),
    SOC_ENUM!("ALC Capture Hold Time", adau1761_alc_hold_time_enum),
    SOC_SINGLE_TLV!("ALC Capture Target Volume", ADAU1761_ALC_CTRL1, 0, 15, 0, adau1761_alc_target_tlv),
    SOC_ENUM!("ALC Capture Attack Time", adau1761_alc_decay_time_enum),
    SOC_ENUM!("ALC Capture Decay Time", adau1761_alc_attack_time_enum),
    SOC_ENUM!("ALC Capture Noise Gate Type", adau1761_alc_ng_type_enum),
    SOC_SINGLE!("ALC Capture Noise Gate Switch", ADAU1761_ALC_CTRL3, 5, 1, 0),
    SOC_SINGLE_TLV!("ALC Capture Noise Gate Threshold Volume", ADAU1761_ALC_CTRL3, 0, 31, 0, adau1761_alc_ng_threshold_tlv),
];

static adau1761_single_mode_controls: [snd_kcontrol_new; 4] = [
    SOC_SINGLE_TLV!("Input 1 Capture Volume", ADAU1761_REC_MIXER_LEFT0, 4, 7, 0, adau1761_sing_in_tlv),
    SOC_SINGLE_TLV!("Input 2 Capture Volume", ADAU1761_REC_MIXER_LEFT0, 1, 7, 0, adau1761_sing_in_tlv),
    SOC_SINGLE_TLV!("Input 3 Capture Volume", ADAU1761_REC_MIXER_RIGHT0, 4, 7, 0, adau1761_sing_in_tlv),
    SOC_SINGLE_TLV!("Input 4 Capture Volume", ADAU1761_REC_MIXER_RIGHT0, 1, 7, 0, adau1761_sing_in_tlv),
];

static adau1761_controls: [snd_kcontrol_new; 10] = [
    SOC_DOUBLE_R_TLV!("Aux Capture Volume", ADAU1761_REC_MIXER_LEFT1, ADAU1761_REC_MIXER_RIGHT1, 0, 7, 0, adau1761_sing_in_tlv),
    SOC_DOUBLE_R_TLV!("Headphone Playback Volume", ADAU1761_PLAY_HP_LEFT_VOL, ADAU1761_PLAY_HP_RIGHT_VOL, 2, 0x3f, 0, adau1761_out_tlv),
    SOC_DOUBLE_R!("Headphone Playback Switch", ADAU1761_PLAY_HP_LEFT_VOL, ADAU1761_PLAY_HP_RIGHT_VOL, 1, 1, 0),
    SOC_DOUBLE_R_TLV!("Lineout Playback Volume", ADAU1761_PLAY_LINE_LEFT_VOL, ADAU1761_PLAY_LINE_RIGHT_VOL, 2, 0x3f, 0, adau1761_out_tlv),
    SOC_DOUBLE_R!("Lineout Playback Switch", ADAU1761_PLAY_LINE_LEFT_VOL, ADAU1761_PLAY_LINE_RIGHT_VOL, 1, 1, 0),
    SOC_ENUM!("ADC Bias", adau1761_adc_bias_enum),
    SOC_ENUM!("DAC Bias", adau1761_dac_bias_enum),
    SOC_ENUM!("Capture Bias", adau1761_capture_bias_enum),
    SOC_ENUM!("Playback Bias", adau1761_playback_bias_enum),
    SOC_ENUM!("Headphone Bias", adau1761_hp_bias_enum),
];

static adau1761_mono_controls: [snd_kcontrol_new; 2] = [
    SOC_SINGLE_TLV!("Mono Playback Volume", ADAU1761_PLAY_MONO_OUTPUT_VOL, 2, 0x3f, 0, adau1761_out_tlv),
    SOC_SINGLE!("Mono Playback Switch", ADAU1761_PLAY_MONO_OUTPUT_VOL, 1, 1, 0),
];

static adau1761_left_mixer_controls: [snd_kcontrol_new; 5] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("Left DAC Switch", ADAU1761_PLAY_MIXER_LEFT0, 5, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("Right DAC Switch", ADAU1761_PLAY_MIXER_LEFT0, 6, 1, 0),
    SOC_DAPM_SINGLE_TLV!("Aux Bypass Volume", ADAU1761_PLAY_MIXER_LEFT0, 1, 8, 0, adau1761_sidetone_tlv),
    SOC_DAPM_SINGLE_TLV!("Right Bypass Volume", ADAU1761_PLAY_MIXER_LEFT1, 4, 8, 0, adau1761_sidetone_tlv),
    SOC_DAPM_SINGLE_TLV!("Left Bypass Volume", ADAU1761_PLAY_MIXER_LEFT1, 0, 8, 0, adau1761_sidetone_tlv),
];

static adau1761_right_mixer_controls: [snd_kcontrol_new; 5] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("Left DAC Switch", ADAU1761_PLAY_MIXER_RIGHT0, 5, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("Right DAC Switch", ADAU1761_PLAY_MIXER_RIGHT0, 6, 1, 0),
    SOC_DAPM_SINGLE_TLV!("Aux Bypass Volume", ADAU1761_PLAY_MIXER_RIGHT0, 1, 8, 0, adau1761_sidetone_tlv),
    SOC_DAPM_SINGLE_TLV!("Right Bypass Volume", ADAU1761_PLAY_MIXER_RIGHT1, 4, 8, 0, adau1761_sidetone_tlv),
    SOC_DAPM_SINGLE_TLV!("Left Bypass Volume", ADAU1761_PLAY_MIXER_RIGHT1, 0, 8, 0, adau1761_sidetone_tlv),
];

static adau1761_left_lr_mixer_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_TLV!("Left Volume", ADAU1761_PLAY_LR_MIXER_LEFT, 1, 2, 0, adau1761_boost_tlv),
    SOC_DAPM_SINGLE_TLV!("Right Volume", ADAU1761_PLAY_LR_MIXER_LEFT, 3, 2, 0, adau1761_boost_tlv),
];

static adau1761_right_lr_mixer_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_TLV!("Left Volume", ADAU1761_PLAY_LR_MIXER_RIGHT, 1, 2, 0, adau1761_boost_tlv),
    SOC_DAPM_SINGLE_TLV!("Right Volume", ADAU1761_PLAY_LR_MIXER_RIGHT, 3, 2, 0, adau1761_boost_tlv),
];

static adau1761_input_mux_text: [*const c_char; 2] = [cstr!("ADC"), cstr!("DMIC")];
static adau1761_input_mux_enum: () = SOC_ENUM_SINGLE_DECL!(adau1761_input_mux_enum, ADAU17X1_ADC_CONTROL, 2, adau1761_input_mux_text);
static adau1761_input_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("Input Select", adau1761_input_mux_enum);

unsafe extern "C" fn adau1761_dejitter_fixup(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    _event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;

    /*
     * After any power changes have been made the dejitter circuit
     * has to be reinitialized.
     */
    regmap_write((*adau).regmap, ADAU1761_DEJITTER, 0);
    if !(*adau).master {
        regmap_write((*adau).regmap, ADAU1761_DEJITTER, 3);
    }

    0
}

static adau1x61_dapm_widgets: [snd_soc_dapm_widget; 19] = [
    SND_SOC_DAPM_MIXER!("Left Input Mixer", ADAU1761_REC_MIXER_LEFT0, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Right Input Mixer", ADAU1761_REC_MIXER_RIGHT0, 0, 0, ptr::null(), 0),
    SOC_MIXER_ARRAY!("Left Playback Mixer", ADAU1761_PLAY_MIXER_LEFT0, 0, 0, adau1761_left_mixer_controls),
    SOC_MIXER_ARRAY!("Right Playback Mixer", ADAU1761_PLAY_MIXER_RIGHT0, 0, 0, adau1761_right_mixer_controls),
    SOC_MIXER_ARRAY!("Left LR Playback Mixer", ADAU1761_PLAY_LR_MIXER_LEFT, 0, 0, adau1761_left_lr_mixer_controls),
    SOC_MIXER_ARRAY!("Right LR Playback Mixer", ADAU1761_PLAY_LR_MIXER_RIGHT, 0, 0, adau1761_right_lr_mixer_controls),
    SND_SOC_DAPM_SUPPLY!("Headphone", ADAU1761_PLAY_HP_LEFT_VOL, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!("SYSCLK", 2, SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_POST!("Dejitter fixup", adau1761_dejitter_fixup),
    SND_SOC_DAPM_INPUT!("LAUX"),
    SND_SOC_DAPM_INPUT!("RAUX"),
    SND_SOC_DAPM_INPUT!("LINP"),
    SND_SOC_DAPM_INPUT!("LINN"),
    SND_SOC_DAPM_INPUT!("RINP"),
    SND_SOC_DAPM_INPUT!("RINN"),
    SND_SOC_DAPM_OUTPUT!("LOUT"),
    SND_SOC_DAPM_OUTPUT!("ROUT"),
    SND_SOC_DAPM_OUTPUT!("LHP"),
    SND_SOC_DAPM_OUTPUT!("RHP"),
];

static adau1761_mono_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_MIXER!("Mono Playback Mixer", ADAU1761_PLAY_MIXER_MONO, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_OUTPUT!("MONOOUT"),
];

static adau1761_capless_dapm_widgets: [snd_soc_dapm_widget; 1] = [
    SND_SOC_DAPM_SUPPLY_S!("Headphone VGND", 1, ADAU1761_PLAY_MIXER_MONO, 0, 0, ptr::null(), 0),
];

static adau1x61_dapm_routes: [snd_soc_dapm_route; 30] = [
    snd_soc_dapm_route { sink: cstr!("Left Input Mixer"), control: ptr::null(), source: cstr!("LINP") },
    snd_soc_dapm_route { sink: cstr!("Left Input Mixer"), control: ptr::null(), source: cstr!("LINN") },
    snd_soc_dapm_route { sink: cstr!("Left Input Mixer"), control: ptr::null(), source: cstr!("LAUX") },
    snd_soc_dapm_route { sink: cstr!("Right Input Mixer"), control: ptr::null(), source: cstr!("RINP") },
    snd_soc_dapm_route { sink: cstr!("Right Input Mixer"), control: ptr::null(), source: cstr!("RINN") },
    snd_soc_dapm_route { sink: cstr!("Right Input Mixer"), control: ptr::null(), source: cstr!("RAUX") },
    snd_soc_dapm_route { sink: cstr!("Left Playback Mixer"), control: ptr::null(), source: cstr!("Left Playback Enable") },
    snd_soc_dapm_route { sink: cstr!("Right Playback Mixer"), control: ptr::null(), source: cstr!("Right Playback Enable") },
    snd_soc_dapm_route { sink: cstr!("Left LR Playback Mixer"), control: ptr::null(), source: cstr!("Left Playback Enable") },
    snd_soc_dapm_route { sink: cstr!("Right LR Playback Mixer"), control: ptr::null(), source: cstr!("Right Playback Enable") },
    snd_soc_dapm_route { sink: cstr!("Left Playback Mixer"), control: cstr!("Left DAC Switch"), source: cstr!("Left DAC") },
    snd_soc_dapm_route { sink: cstr!("Left Playback Mixer"), control: cstr!("Right DAC Switch"), source: cstr!("Right DAC") },
    snd_soc_dapm_route { sink: cstr!("Right Playback Mixer"), control: cstr!("Left DAC Switch"), source: cstr!("Left DAC") },
    snd_soc_dapm_route { sink: cstr!("Right Playback Mixer"), control: cstr!("Right DAC Switch"), source: cstr!("Right DAC") },
    snd_soc_dapm_route { sink: cstr!("Left LR Playback Mixer"), control: cstr!("Left Volume"), source: cstr!("Left Playback Mixer") },
    snd_soc_dapm_route { sink: cstr!("Left LR Playback Mixer"), control: cstr!("Right Volume"), source: cstr!("Right Playback Mixer") },
    snd_soc_dapm_route { sink: cstr!("Right LR Playback Mixer"), control: cstr!("Left Volume"), source: cstr!("Left Playback Mixer") },
    snd_soc_dapm_route { sink: cstr!("Right LR Playback Mixer"), control: cstr!("Right Volume"), source: cstr!("Right Playback Mixer") },
    snd_soc_dapm_route { sink: cstr!("LHP"), control: ptr::null(), source: cstr!("Left Playback Mixer") },
    snd_soc_dapm_route { sink: cstr!("RHP"), control: ptr::null(), source: cstr!("Right Playback Mixer") },
    snd_soc_dapm_route { sink: cstr!("LHP"), control: ptr::null(), source: cstr!("Headphone") },
    snd_soc_dapm_route { sink: cstr!("RHP"), control: ptr::null(), source: cstr!("Headphone") },
    snd_soc_dapm_route { sink: cstr!("LOUT"), control: ptr::null(), source: cstr!("Left LR Playback Mixer") },
    snd_soc_dapm_route { sink: cstr!("ROUT"), control: ptr::null(), source: cstr!("Right LR Playback Mixer") },
    snd_soc_dapm_route { sink: cstr!("Left Playback Mixer"), control: cstr!("Aux Bypass Volume"), source: cstr!("LAUX") },
    snd_soc_dapm_route { sink: cstr!("Left Playback Mixer"), control: cstr!("Left Bypass Volume"), source: cstr!("Left Input Mixer") },
    snd_soc_dapm_route { sink: cstr!("Left Playback Mixer"), control: cstr!("Right Bypass Volume"), source: cstr!("Right Input Mixer") },
    snd_soc_dapm_route { sink: cstr!("Right Playback Mixer"), control: cstr!("Aux Bypass Volume"), source: cstr!("RAUX") },
    snd_soc_dapm_route { sink: cstr!("Right Playback Mixer"), control: cstr!("Left Bypass Volume"), source: cstr!("Left Input Mixer") },
    snd_soc_dapm_route { sink: cstr!("Right Playback Mixer"), control: cstr!("Right Bypass Volume"), source: cstr!("Right Input Mixer") },
];

static adau1761_mono_dapm_routes: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route { sink: cstr!("Mono Playback Mixer"), control: ptr::null(), source: cstr!("Left Playback Mixer") },
    snd_soc_dapm_route { sink: cstr!("Mono Playback Mixer"), control: ptr::null(), source: cstr!("Right Playback Mixer") },
    snd_soc_dapm_route { sink: cstr!("MONOOUT"), control: ptr::null(), source: cstr!("Mono Playback Mixer") },
];

static adau1761_capless_dapm_routes: [snd_soc_dapm_route; 1] = [
    snd_soc_dapm_route { sink: cstr!("Headphone"), control: ptr::null(), source: cstr!("Headphone VGND") },
];

static adau1761_dmic_widgets: [snd_soc_dapm_widget; 3] = [
    SND_SOC_DAPM_MUX!("Left Decimator Mux", SND_SOC_NOPM, 0, 0, &adau1761_input_mux_control),
    SND_SOC_DAPM_MUX!("Right Decimator Mux", SND_SOC_NOPM, 0, 0, &adau1761_input_mux_control),
    SND_SOC_DAPM_INPUT!("DMIC"),
];

static adau1761_dmic_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: cstr!("Left Decimator Mux"), control: cstr!("ADC"), source: cstr!("Left Input Mixer") },
    snd_soc_dapm_route { sink: cstr!("Left Decimator Mux"), control: cstr!("DMIC"), source: cstr!("DMIC") },
    snd_soc_dapm_route { sink: cstr!("Right Decimator Mux"), control: cstr!("ADC"), source: cstr!("Right Input Mixer") },
    snd_soc_dapm_route { sink: cstr!("Right Decimator Mux"), control: cstr!("DMIC"), source: cstr!("DMIC") },
    snd_soc_dapm_route { sink: cstr!("Left Decimator"), control: ptr::null(), source: cstr!("Left Decimator Mux") },
    snd_soc_dapm_route { sink: cstr!("Right Decimator"), control: ptr::null(), source: cstr!("Right Decimator Mux") },
];

static adau1761_no_dmic_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: cstr!("Left Decimator"), control: ptr::null(), source: cstr!("Left Input Mixer") },
    snd_soc_dapm_route { sink: cstr!("Right Decimator"), control: ptr::null(), source: cstr!("Right Input Mixer") },
];

static adau1761_dapm_widgets: [snd_soc_dapm_widget; 9] = [
    SND_SOC_DAPM_SUPPLY!("Serial Port Clock", ADAU1761_CLK_ENABLE0, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Serial Input Routing Clock", ADAU1761_CLK_ENABLE0, 1, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Serial Output Routing Clock", ADAU1761_CLK_ENABLE0, 3, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Decimator Resync Clock", ADAU1761_CLK_ENABLE0, 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Interpolator Resync Clock", ADAU1761_CLK_ENABLE0, 2, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Slew Clock", ADAU1761_CLK_ENABLE0, 6, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ALC Clock", ADAU1761_CLK_ENABLE0, 5, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!("Digital Clock 0", 1, ADAU1761_CLK_ENABLE1, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!("Digital Clock 1", 1, ADAU1761_CLK_ENABLE1, 1, 0, ptr::null(), 0),
];

static adau1761_dapm_routes: [snd_soc_dapm_route; 20] = [
    snd_soc_dapm_route { sink: cstr!("Left Decimator"), control: ptr::null(), source: cstr!("Digital Clock 0") },
    snd_soc_dapm_route { sink: cstr!("Right Decimator"), control: ptr::null(), source: cstr!("Digital Clock 0") },
    snd_soc_dapm_route { sink: cstr!("Left DAC"), control: ptr::null(), source: cstr!("Digital Clock 0") },
    snd_soc_dapm_route { sink: cstr!("Right DAC"), control: ptr::null(), source: cstr!("Digital Clock 0") },
    snd_soc_dapm_route { sink: cstr!("AIFCLK"), control: ptr::null(), source: cstr!("Digital Clock 1") },
    snd_soc_dapm_route { sink: cstr!("Playback"), control: ptr::null(), source: cstr!("Serial Port Clock") },
    snd_soc_dapm_route { sink: cstr!("Capture"), control: ptr::null(), source: cstr!("Serial Port Clock") },
    snd_soc_dapm_route { sink: cstr!("Playback"), control: ptr::null(), source: cstr!("Serial Input Routing Clock") },
    snd_soc_dapm_route { sink: cstr!("Capture"), control: ptr::null(), source: cstr!("Serial Output Routing Clock") },
    snd_soc_dapm_route { sink: cstr!("Left Decimator"), control: ptr::null(), source: cstr!("Decimator Resync Clock") },
    snd_soc_dapm_route { sink: cstr!("Right Decimator"), control: ptr::null(), source: cstr!("Decimator Resync Clock") },
    snd_soc_dapm_route { sink: cstr!("Left DAC"), control: ptr::null(), source: cstr!("Interpolator Resync Clock") },
    snd_soc_dapm_route { sink: cstr!("Right DAC"), control: ptr::null(), source: cstr!("Interpolator Resync Clock") },
    snd_soc_dapm_route { sink: cstr!("Slew Clock"), control: ptr::null(), source: cstr!("Digital Clock 0") },
    snd_soc_dapm_route { sink: cstr!("Right Playback Mixer"), control: ptr::null(), source: cstr!("Slew Clock") },
    snd_soc_dapm_route { sink: cstr!("Left Playback Mixer"), control: ptr::null(), source: cstr!("Slew Clock") },
    snd_soc_dapm_route { sink: cstr!("Left Input Mixer"), control: ptr::null(), source: cstr!("ALC Clock") },
    snd_soc_dapm_route { sink: cstr!("Right Input Mixer"), control: ptr::null(), source: cstr!("ALC Clock") },
    snd_soc_dapm_route { sink: cstr!("Digital Clock 0"), control: ptr::null(), source: cstr!("SYSCLK") },
    snd_soc_dapm_route { sink: cstr!("Digital Clock 1"), control: ptr::null(), source: cstr!("SYSCLK") },
];

static adau1761_dapm_dsp_routes: [snd_soc_dapm_route; 1] = [
    snd_soc_dapm_route { sink: cstr!("DSP"), control: ptr::null(), source: cstr!("Digital Clock 0") },
];

unsafe extern "C" fn adau1761_compatibility_probe(dev: *mut device) -> c_int {
    let adau = dev_get_drvdata(dev) as *mut adau;
    let regmap = (*adau).regmap;
    let mut val: c_int = 0;
    let mut ret: c_int = 0;

    /* Only consider compatibility mode when ADAU1361 was specified. */
    if (*adau).type_ != ADAU1361 {
        return 0;
    }

    regcache_cache_bypass(regmap, true);

    /*
     * This will enable the core clock and bypass the PLL,
     * so that we can access the registers for probing purposes
     * (without having to set up the PLL).
     */
    regmap_write(regmap, ADAU17X1_CLOCK_CONTROL, ADAU17X1_CLOCK_CONTROL_SYSCLK_EN);

    /*
     * ADAU17X1_SERIAL_SAMPLING_RATE doesn't exist in non-DSP chips;
     * reading it results in zero at all times, and write is a no-op.
     * Use this register to probe for ADAU1761.
     */
    regmap_write(regmap, ADAU17X1_SERIAL_SAMPLING_RATE, 1);
    ret = regmap_read(regmap, ADAU17X1_SERIAL_SAMPLING_RATE, &mut val);
    if ret != 0 {
        regmap_write(regmap, ADAU17X1_CLOCK_CONTROL, 0);
        regcache_cache_bypass(regmap, false);
        return ret;
    }
    if val != 1 {
        regmap_write(regmap, ADAU17X1_CLOCK_CONTROL, 0);
        regcache_cache_bypass(regmap, false);
        return ret;
    }
    regmap_write(regmap, ADAU17X1_SERIAL_SAMPLING_RATE, 0);
    ret = regmap_read(regmap, ADAU17X1_SERIAL_SAMPLING_RATE, &mut val);
    if ret != 0 {
        regmap_write(regmap, ADAU17X1_CLOCK_CONTROL, 0);
        regcache_cache_bypass(regmap, false);
        return ret;
    }
    if val != 0 {
        regmap_write(regmap, ADAU17X1_CLOCK_CONTROL, 0);
        regcache_cache_bypass(regmap, false);
        return ret;
    }

    (*adau).type_ = ADAU1761_AS_1361;
    /* Disable core clock after probing. */
    regmap_write(regmap, ADAU17X1_CLOCK_CONTROL, 0);
    regcache_cache_bypass(regmap, false);
    ret
}

unsafe extern "C" fn adau1761_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;
    let dapm = snd_soc_component_to_dapm(component);

    if level == SND_SOC_BIAS_ON {
    } else if level == SND_SOC_BIAS_PREPARE {
    } else if level == SND_SOC_BIAS_STANDBY {
        regcache_cache_only((*adau).regmap, false);
        regmap_update_bits(
            (*adau).regmap,
            ADAU17X1_CLOCK_CONTROL,
            ADAU17X1_CLOCK_CONTROL_SYSCLK_EN,
            ADAU17X1_CLOCK_CONTROL_SYSCLK_EN,
        );
        if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
            regcache_sync((*adau).regmap);
        }
    } else if level == SND_SOC_BIAS_OFF {
        regmap_update_bits(
            (*adau).regmap,
            ADAU17X1_CLOCK_CONTROL,
            ADAU17X1_CLOCK_CONTROL_SYSCLK_EN,
            0,
        );
        regcache_cache_only((*adau).regmap, true);
    }
    0
}

unsafe extern "C" fn adau1761_get_lineout_mode(
    component: *mut snd_soc_component,
) -> adau1761_output_mode {
    let pdata = (*(*component).dev).platform_data as *mut adau1761_platform_data;

    if !pdata.is_null() {
        return (*pdata).lineout_mode;
    }

    ADAU1761_OUTPUT_MODE_LINE
}

unsafe extern "C" fn adau1761_setup_digmic_jackdetect(
    component: *mut snd_soc_component,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let pdata = (*(*component).dev).platform_data as *mut adau1761_platform_data;
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;
    let mode: adau1761_digmic_jackdet_pin_mode;
    let mut val: c_uint = 0;
    let mut ret: c_int;

    if !pdata.is_null() {
        mode = (*pdata).digmic_jackdetect_pin_mode;
    } else {
        mode = ADAU1761_DIGMIC_JACKDET_PIN_MODE_NONE;
    }

    if mode == ADAU1761_DIGMIC_JACKDET_PIN_MODE_JACKDETECT {
        if (*pdata).jackdetect_debounce_time == ADAU1761_JACKDETECT_DEBOUNCE_5MS
            || (*pdata).jackdetect_debounce_time == ADAU1761_JACKDETECT_DEBOUNCE_10MS
            || (*pdata).jackdetect_debounce_time == ADAU1761_JACKDETECT_DEBOUNCE_20MS
            || (*pdata).jackdetect_debounce_time == ADAU1761_JACKDETECT_DEBOUNCE_40MS
        {
            val |= ((*pdata).jackdetect_debounce_time as c_uint) << 6;
        } else {
            return -EINVAL;
        }
        if (*pdata).jackdetect_active_low {
            val |= ADAU1761_DIGMIC_JACKDETECT_ACTIVE_LOW;
        }

        ret = snd_soc_add_component_controls(
            component,
            adau1761_jack_detect_controls.as_ptr(),
            ARRAY_SIZE!(adau1761_jack_detect_controls),
        );
        if ret != 0 {
            return ret;
        }
        ret = snd_soc_dapm_add_routes(
            dapm,
            adau1761_no_dmic_routes.as_ptr(),
            ARRAY_SIZE!(adau1761_no_dmic_routes),
        );
        if ret != 0 {
            return ret;
        }
    } else if mode == ADAU1761_DIGMIC_JACKDET_PIN_MODE_NONE {
        ret = snd_soc_dapm_add_routes(
            dapm,
            adau1761_no_dmic_routes.as_ptr(),
            ARRAY_SIZE!(adau1761_no_dmic_routes),
        );
        if ret != 0 {
            return ret;
        }
    } else if mode == ADAU1761_DIGMIC_JACKDET_PIN_MODE_DIGMIC {
        ret = snd_soc_dapm_new_controls(
            dapm,
            adau1761_dmic_widgets.as_ptr(),
            ARRAY_SIZE!(adau1761_dmic_widgets),
        );
        if ret != 0 {
            return ret;
        }

        ret = snd_soc_dapm_add_routes(
            dapm,
            adau1761_dmic_routes.as_ptr(),
            ARRAY_SIZE!(adau1761_dmic_routes),
        );
        if ret != 0 {
            return ret;
        }

        val |= ADAU1761_DIGMIC_JACKDETECT_DIGMIC;
    } else {
        return -EINVAL;
    }

    regmap_write((*adau).regmap, ADAU1761_DIGMIC_JACKDETECT, val);

    0
}

unsafe extern "C" fn adau1761_setup_headphone_mode(
    component: *mut snd_soc_component,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;
    let pdata = (*(*component).dev).platform_data as *mut adau1761_platform_data;
    let mode: adau1761_output_mode;
    let mut ret: c_int;

    if !pdata.is_null() {
        mode = (*pdata).headphone_mode;
    } else {
        mode = ADAU1761_OUTPUT_MODE_HEADPHONE;
    }

    if mode == ADAU1761_OUTPUT_MODE_LINE {
    } else if mode == ADAU1761_OUTPUT_MODE_HEADPHONE_CAPLESS {
        regmap_update_bits(
            (*adau).regmap,
            ADAU1761_PLAY_MONO_OUTPUT_VOL,
            ADAU1761_PLAY_MONO_OUTPUT_VOL_MODE_HP | ADAU1761_PLAY_MONO_OUTPUT_VOL_UNMUTE,
            ADAU1761_PLAY_MONO_OUTPUT_VOL_MODE_HP | ADAU1761_PLAY_MONO_OUTPUT_VOL_UNMUTE,
        );
        regmap_update_bits(
            (*adau).regmap,
            ADAU1761_PLAY_HP_RIGHT_VOL,
            ADAU1761_PLAY_HP_RIGHT_VOL_MODE_HP,
            ADAU1761_PLAY_HP_RIGHT_VOL_MODE_HP,
        );
    } else if mode == ADAU1761_OUTPUT_MODE_HEADPHONE {
        regmap_update_bits(
            (*adau).regmap,
            ADAU1761_PLAY_HP_RIGHT_VOL,
            ADAU1761_PLAY_HP_RIGHT_VOL_MODE_HP,
            ADAU1761_PLAY_HP_RIGHT_VOL_MODE_HP,
        );
    } else {
        return -EINVAL;
    }

    if mode == ADAU1761_OUTPUT_MODE_HEADPHONE_CAPLESS {
        ret = snd_soc_dapm_new_controls(
            dapm,
            adau1761_capless_dapm_widgets.as_ptr(),
            ARRAY_SIZE!(adau1761_capless_dapm_widgets),
        );
        if ret != 0 {
            return ret;
        }
        ret = snd_soc_dapm_add_routes(
            dapm,
            adau1761_capless_dapm_routes.as_ptr(),
            ARRAY_SIZE!(adau1761_capless_dapm_routes),
        );
    } else {
        ret = snd_soc_add_component_controls(
            component,
            adau1761_mono_controls.as_ptr(),
            ARRAY_SIZE!(adau1761_mono_controls),
        );
        if ret != 0 {
            return ret;
        }
        ret = snd_soc_dapm_new_controls(
            dapm,
            adau1761_mono_dapm_widgets.as_ptr(),
            ARRAY_SIZE!(adau1761_mono_dapm_widgets),
        );
        if ret != 0 {
            return ret;
        }
        ret = snd_soc_dapm_add_routes(
            dapm,
            adau1761_mono_dapm_routes.as_ptr(),
            ARRAY_SIZE!(adau1761_mono_dapm_routes),
        );
    }

    ret
}

unsafe extern "C" fn adau1761_readable_register(
    dev: *mut device,
    reg: c_uint,
) -> bool_ {
    match reg {
        ADAU1761_DIGMIC_JACKDETECT
        | ADAU1761_REC_MIXER_LEFT0
        | ADAU1761_REC_MIXER_LEFT1
        | ADAU1761_REC_MIXER_RIGHT0
        | ADAU1761_REC_MIXER_RIGHT1
        | ADAU1761_LEFT_DIFF_INPUT_VOL
        | ADAU1761_RIGHT_DIFF_INPUT_VOL
        | ADAU1761_PLAY_LR_MIXER_LEFT
        | ADAU1761_PLAY_MIXER_LEFT0
        | ADAU1761_PLAY_MIXER_LEFT1
        | ADAU1761_PLAY_MIXER_RIGHT0
        | ADAU1761_PLAY_MIXER_RIGHT1
        | ADAU1761_PLAY_LR_MIXER_RIGHT
        | ADAU1761_PLAY_MIXER_MONO
        | ADAU1761_PLAY_HP_LEFT_VOL
        | ADAU1761_PLAY_HP_RIGHT_VOL
        | ADAU1761_PLAY_LINE_LEFT_VOL
        | ADAU1761_PLAY_LINE_RIGHT_VOL
        | ADAU1761_PLAY_MONO_OUTPUT_VOL
        | ADAU1761_POP_CLICK_SUPPRESS
        | ADAU1761_JACK_DETECT_PIN
        | ADAU1761_DEJITTER
        | ADAU1761_CLK_ENABLE0
        | ADAU1761_CLK_ENABLE1
        | ADAU1761_ALC_CTRL0
        | ADAU1761_ALC_CTRL1
        | ADAU1761_ALC_CTRL2
        | ADAU1761_ALC_CTRL3 => true,
        _ => adau17x1_readable_register(dev, reg),
    }
}

unsafe extern "C" fn adau1761_component_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let pdata = (*(*component).dev).platform_data as *mut adau1761_platform_data;
    let adau = snd_soc_component_get_drvdata(component) as *mut adau;
    let mut ret: c_int;

    ret = adau17x1_add_widgets(component);
    if ret < 0 {
        return ret;
    }

    if !pdata.is_null() && (*pdata).input_differential {
        regmap_update_bits(
            (*adau).regmap,
            ADAU1761_LEFT_DIFF_INPUT_VOL,
            ADAU1761_DIFF_INPUT_VOL_LDEN,
            ADAU1761_DIFF_INPUT_VOL_LDEN,
        );
        regmap_update_bits(
            (*adau).regmap,
            ADAU1761_RIGHT_DIFF_INPUT_VOL,
            ADAU1761_DIFF_INPUT_VOL_LDEN,
            ADAU1761_DIFF_INPUT_VOL_LDEN,
        );
        ret = snd_soc_add_component_controls(
            component,
            adau1761_differential_mode_controls.as_ptr(),
            ARRAY_SIZE!(adau1761_differential_mode_controls),
        );
        if ret != 0 {
            return ret;
        }
    } else {
        ret = snd_soc_add_component_controls(
            component,
            adau1761_single_mode_controls.as_ptr(),
            ARRAY_SIZE!(adau1761_single_mode_controls),
        );
        if ret != 0 {
            return ret;
        }
    }

    let lineout_mode = adau1761_get_lineout_mode(component);
    if lineout_mode == ADAU1761_OUTPUT_MODE_LINE {
    } else if lineout_mode == ADAU1761_OUTPUT_MODE_HEADPHONE {
        regmap_update_bits(
            (*adau).regmap,
            ADAU1761_PLAY_LINE_LEFT_VOL,
            ADAU1761_PLAY_LINE_LEFT_VOL_MODE_HP,
            ADAU1761_PLAY_LINE_LEFT_VOL_MODE_HP,
        );
        regmap_update_bits(
            (*adau).regmap,
            ADAU1761_PLAY_LINE_RIGHT_VOL,
            ADAU1761_PLAY_LINE_RIGHT_VOL_MODE_HP,
            ADAU1761_PLAY_LINE_RIGHT_VOL_MODE_HP,
        );
    } else {
        return -EINVAL;
    }

    ret = adau1761_setup_headphone_mode(component);
    if ret != 0 {
        return ret;
    }

    ret = adau1761_setup_digmic_jackdetect(component);
    if ret != 0 {
        return ret;
    }

    /*
     * If we've got an ADAU1761, or an ADAU1761 operating as an
     * ADAU1361, we need these non-DSP related DAPM widgets and routes.
     */
    if (*adau).type_ == ADAU1761 || (*adau).type_ == ADAU1761_AS_1361 {
        ret = snd_soc_dapm_new_controls(
            dapm,
            adau1761_dapm_widgets.as_ptr(),
            ARRAY_SIZE!(adau1761_dapm_widgets),
        );
        if ret != 0 {
            return ret;
        }

        ret = snd_soc_dapm_add_routes(
            dapm,
            adau1761_dapm_routes.as_ptr(),
            ARRAY_SIZE!(adau1761_dapm_routes),
        );
        if ret != 0 {
            return ret;
        }
    }
    /*
     * These routes are DSP related and only used when we have a
     * bona fide ADAU1761.
     */
    if (*adau).type_ == ADAU1761 {
        ret = snd_soc_dapm_add_routes(
            dapm,
            adau1761_dapm_dsp_routes.as_ptr(),
            ARRAY_SIZE!(adau1761_dapm_dsp_routes),
        );
        if ret != 0 {
            return ret;
        }
    }
    /*
     * In the ADAU1761, by default, the AIF is routed to the DSP, whereas
     * for the ADAU1361, the AIF is permanently routed to the ADC and DAC.
     * Thus, if we have an ADAU1761 masquerading as an ADAU1361,
     * we need to explicitly route the AIF to the ADC and DAC.
     * For the ADAU1761, this is normally done by set_tdm_slot, but this
     * function is not necessarily called during stream setup, so set up
     * the compatible AIF routings here from the start.
     */
    if (*adau).type_ == ADAU1761_AS_1361 {
        regmap_write((*adau).regmap, ADAU17X1_SERIAL_INPUT_ROUTE, 0x01);
        regmap_write((*adau).regmap, ADAU17X1_SERIAL_OUTPUT_ROUTE, 0x01);
    }
    ret = adau17x1_add_routes(component);
    if ret < 0 {
        return ret;
    }

    0
}

static adau1761_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(adau1761_component_probe),
    resume: Some(adau17x1_resume),
    set_bias_level: Some(adau1761_set_bias_level),
    controls: adau1761_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(adau1761_controls),
    dapm_widgets: adau1x61_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(adau1x61_dapm_widgets),
    dapm_routes: adau1x61_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(adau1x61_dapm_routes),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe fn ADAU1761_FORMATS() -> c_uint {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

static mut adau1361_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("adau-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("Playback"),
        channels_min: 2,
        channels_max: 4,
        rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
        formats: unsafe { ADAU1761_FORMATS() },
    },
    capture: snd_soc_pcm_stream {
        stream_name: cstr!("Capture"),
        channels_min: 2,
        channels_max: 4,
        rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
        formats: unsafe { ADAU1761_FORMATS() },
    },
    ops: unsafe { &adau17x1_dai_ops as *const c_void },
};

static mut adau1761_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("adau-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("Playback"),
        channels_min: 2,
        channels_max: 8,
        rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
        formats: unsafe { ADAU1761_FORMATS() },
    },
    capture: snd_soc_pcm_stream {
        stream_name: cstr!("Capture"),
        channels_min: 2,
        channels_max: 8,
        rates: unsafe { SNDRV_PCM_RATE_8000_96000 },
        formats: unsafe { ADAU1761_FORMATS() },
    },
    ops: unsafe { &adau17x1_dai_ops as *const c_void },
};

#[no_mangle]
pub unsafe extern "C" fn adau1761_probe(
    dev: *mut device,
    regmap: *mut regmap,
    type_: adau17x1_type,
    switch_mode: Option<unsafe extern "C" fn(*mut device)>,
) -> c_int {
    let dai_drv: *mut snd_soc_dai_driver;
    let firmware_name: *const c_char;
    let mut ret: c_int;

    if type_ == ADAU1361 {
        dai_drv = &mut adau1361_dai_driver;
        firmware_name = ptr::null();
    } else {
        dai_drv = &mut adau1761_dai_driver;
        firmware_name = ADAU1761_FIRMWARE;
    }

    ret = adau17x1_probe(dev, regmap, type_, switch_mode, firmware_name);
    if ret != 0 {
        return ret;
    }

    ret = adau1761_compatibility_probe(dev);
    if ret != 0 {
        return ret;
    }

    /*
     * Enable cache only mode as we could miss writes before bias level
     * reaches standby and the core clock is enabled
     */
    regcache_cache_only(regmap, true);

    devm_snd_soc_register_component(dev, &adau1761_component_driver, dai_drv, 1)
}

#[no_mangle]
pub static adau1761_regmap_config: regmap_config = regmap_config {
    val_bits: 8,
    reg_bits: 16,
    max_register: 0x40fa,
    reg_defaults: adau1761_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(adau1761_reg_defaults),
    readable_reg: Some(adau1761_readable_register),
    volatile_reg: Some(adau17x1_volatile_register),
    precious_reg: Some(adau17x1_precious_register),
    cache_type: unsafe { REGCACHE_MAPLE },
};

// EXPORT_SYMBOL_GPL(adau1761_probe);
// EXPORT_SYMBOL_GPL(adau1761_regmap_config);
// MODULE_DESCRIPTION("ASoC ADAU1361/ADAU1461/ADAU1761/ADAU1961 CODEC driver");
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
