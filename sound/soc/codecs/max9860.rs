// SPDX-License-Identifier: GPL-2.0
//
// Driver for the MAX9860 Mono Audio Voice Codec
//
// https://datasheets.maximintegrated.com/en/ds/MAX9860.pdf
//
// The driver does not support sidetone since the DVST register field is
// backwards with the mute near the maximum level instead of the minimum.
//
// Author: Peter Rosin <peda@axentia.s>
//         Copyright 2016 Axentia Technologies

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null, null_mut};

type u8 = core::ffi::c_uchar;
type bool_ = bool;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
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
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call:
        Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct max9860_priv {
    pub regmap: *mut regmap,
    pub dvddio: *mut regulator,
    pub dvddio_nb: notifier_block,
    pub psclk: u8,
    pub pclk_rate: c_ulong,
    pub fmt: c_int,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}

pub type snd_soc_bias_level = c_uint;

#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_bias_level:
        Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct i2c_driver {
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
    pub driver: device_driver,
}

unsafe extern "C" {
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool_);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn devm_regulator_register_notifier(
        regulator: *mut regulator,
        nb: *mut notifier_block,
    ) -> c_int;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_put(clk: *mut clk);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_idle(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

const REGULATOR_EVENT_DISABLE: c_ulong = 0;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const REGCACHE_RBTREE: c_uint = 0;
const SND_SOC_NOPM: c_int = -1;
const SND_SOC_DAPM_PRE_PMU: c_int = 0;
const SND_SOC_DAPM_POST_PMD: c_int = 0;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 0;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 0;
const SND_SOC_BIAS_ON: snd_soc_bias_level = 0;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 1;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 2;
const SND_SOC_BIAS_OFF: snd_soc_bias_level = 3;

// Constants from max9860.h and ASoC headers are future dependencies.
const MAX9860_INTEN: c_uint = 0;
const MAX9860_SYSCLK: c_uint = 0;
const MAX9860_AUDIOCLKHIGH: c_uint = 0;
const MAX9860_AUDIOCLKLOW: c_uint = 0;
const MAX9860_IFC1A: c_uint = 0;
const MAX9860_IFC1B: c_uint = 0;
const MAX9860_VOICEFLTR: c_uint = 0;
const MAX9860_DACATTN: c_uint = 0;
const MAX9860_ADCLEVEL: c_uint = 0;
const MAX9860_DACGAIN: c_uint = 0;
const MAX9860_MICGAIN: c_uint = 0;
const MAX9860_MICADC: c_uint = 0;
const MAX9860_NOISEGATE: c_uint = 0;
const MAX9860_PWRMAN: c_uint = 0;
const MAX9860_INTRSTATUS: c_uint = 0;
const MAX9860_REVISION: c_uint = 0;
const MAX9860_MICREADBACK: c_uint = 0;
const MAX9860_MAX_REGISTER: c_uint = 0;
const MAX9860_PAM_MAX: c_uint = 0;
const MAX9860_AGCHLD_SHIFT: c_uint = 0;
const MAX9860_AGCSRC_SHIFT: c_uint = 0;
const MAX9860_AGCATK_SHIFT: c_uint = 0;
const MAX9860_AGCRLS_SHIFT: c_uint = 0;
const MAX9860_AVFLT_SHIFT: c_uint = 0;
const MAX9860_DVFLT_SHIFT: c_uint = 0;
const MAX9860_DVA_SHIFT: c_uint = 0;
const MAX9860_DVA_MUTE: c_uint = 0;
const MAX9860_DVG_SHIFT: c_uint = 0;
const MAX9860_DVG_MAX: c_uint = 0;
const MAX9860_ADCLL_SHIFT: c_uint = 0;
const MAX9860_ADCRL_SHIFT: c_uint = 0;
const MAX9860_ADCxL_MIN: c_uint = 0;
const MAX9860_ANTH_SHIFT: c_uint = 0;
const MAX9860_ANTH_MAX: c_uint = 0;
const MAX9860_AGCTH_SHIFT: c_uint = 0;
const MAX9860_AGCTH_MIN: c_uint = 0;
const MAX9860_PGAM_SHIFT: c_uint = 0;
const MAX9860_PGAM_MIN: c_uint = 0;
const MAX9860_PAM_SHIFT: c_uint = 0;
const MAX9860_ADCLEN_SHIFT: c_uint = 0;
const MAX9860_ADCREN_SHIFT: c_uint = 0;
const MAX9860_DACEN_SHIFT: c_uint = 0;
const MAX9860_ST: u8 = 0;
const MAX9860_MASTER: u8 = 0;
const MAX9860_BSEL_64X: u8 = 0;
const MAX9860_BSEL_48X: u8 = 0;
const MAX9860_DDLY: u8 = 0;
const MAX9860_ADLY: u8 = 0;
const MAX9860_WCI: u8 = 0;
const MAX9860_HIZ: u8 = 0;
const MAX9860_TDM: u8 = 0;
const MAX9860_DBCI: u8 = 0;
const MAX9860_ABCI: u8 = 0;
const MAX9860_FREQ_12MHZ: u8 = 0;
const MAX9860_FREQ_13MHZ: u8 = 0;
const MAX9860_FREQ_19_2MHZ: u8 = 0;
const MAX9860_16KHZ: u8 = 0;
const MAX9860_PSCLK: c_uint = 0;
const MAX9860_PSCLK_OFF: c_uint = 0;
const MAX9860_PLL: c_uint = 0;
const MAX9860_PSCLK_SHIFT: c_uint = 0;
const MAX9860_SHDN: c_uint = 0;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;

const fn cstr<const N: usize>(bytes: &[u8; N]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        $array.len() as c_uint
    };
}

unsafe extern "C" fn max9860_dvddio_event(
    nb: *mut notifier_block,
    event: c_ulong,
    _data: *mut c_void,
) -> c_int {
    let max9860 = (nb as *mut u8).sub(core::mem::offset_of!(max9860_priv, dvddio_nb))
        as *mut max9860_priv;
    if event & REGULATOR_EVENT_DISABLE != 0 {
        regcache_mark_dirty((*max9860).regmap);
        regcache_cache_only((*max9860).regmap, true);
    }

    0
}

static max9860_reg_defaults: [reg_default; 14] = [
    reg_default { reg: MAX9860_INTEN, def: 0x00 },
    reg_default { reg: MAX9860_SYSCLK, def: 0x00 },
    reg_default { reg: MAX9860_AUDIOCLKHIGH, def: 0x00 },
    reg_default { reg: MAX9860_AUDIOCLKLOW, def: 0x00 },
    reg_default { reg: MAX9860_IFC1A, def: 0x00 },
    reg_default { reg: MAX9860_IFC1B, def: 0x00 },
    reg_default { reg: MAX9860_VOICEFLTR, def: 0x00 },
    reg_default { reg: MAX9860_DACATTN, def: 0x00 },
    reg_default { reg: MAX9860_ADCLEVEL, def: 0x00 },
    reg_default { reg: MAX9860_DACGAIN, def: 0x00 },
    reg_default { reg: MAX9860_MICGAIN, def: 0x00 },
    reg_default { reg: MAX9860_MICADC, def: 0x00 },
    reg_default { reg: MAX9860_NOISEGATE, def: 0x00 },
    reg_default { reg: MAX9860_PWRMAN, def: 0x00 },
];

unsafe extern "C" fn max9860_readable(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        MAX9860_INTRSTATUS..=MAX9860_MICGAIN
        | MAX9860_MICADC..=MAX9860_PWRMAN
        | MAX9860_REVISION => true,
        _ => false,
    }
}

unsafe extern "C" fn max9860_writeable(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        MAX9860_INTEN..=MAX9860_MICGAIN | MAX9860_MICADC..=MAX9860_PWRMAN => true,
        _ => false,
    }
}

unsafe extern "C" fn max9860_volatile(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        MAX9860_INTRSTATUS | MAX9860_MICREADBACK => true,
        _ => false,
    }
}

unsafe extern "C" fn max9860_precious(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        MAX9860_INTRSTATUS => true,
        _ => false,
    }
}

static max9860_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    readable_reg: Some(max9860_readable),
    writeable_reg: Some(max9860_writeable),
    volatile_reg: Some(max9860_volatile),
    precious_reg: Some(max9860_precious),
    max_register: MAX9860_MAX_REGISTER,
    reg_defaults: max9860_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(max9860_reg_defaults),
    cache_type: REGCACHE_RBTREE,
};

// TLV, SOC_ENUM, SOC_* control, and DAPM widget macros are provided by ASoC.
static dva_tlv: [c_uint; 4] = [0, (-9100i32) as c_uint, 100, 1];
static dvg_tlv: [c_uint; 4] = [0, 0, 600, 0];
static adc_tlv: [c_uint; 4] = [0, (-1200i32) as c_uint, 100, 0];
static pam_tlv: [c_uint; 8] = [
    0,
    MAX9860_PAM_MAX - 1,
    (-2000i32) as c_uint,
    2000,
    MAX9860_PAM_MAX,
    MAX9860_PAM_MAX,
    3000,
    0,
];
static pgam_tlv: [c_uint; 4] = [0, 0, 100, 0];
static anth_tlv: [c_uint; 4] = [0, (-7600i32) as c_uint, 400, 1];
static agcth_tlv: [c_uint; 4] = [0, (-1800i32) as c_uint, 100, 0];

static agchld_text: [*const c_char; 4] = [
    cstr(b"AGC Disabled\0"),
    cstr(b"50ms\0"),
    cstr(b"100ms\0"),
    cstr(b"400ms\0"),
];
static agcsrc_text: [*const c_char; 2] = [cstr(b"Left ADC\0"), cstr(b"Left/Right ADC\0")];
static agcatk_text: [*const c_char; 4] = [
    cstr(b"3ms\0"),
    cstr(b"12ms\0"),
    cstr(b"50ms\0"),
    cstr(b"200ms\0"),
];
static agcrls_text: [*const c_char; 8] = [
    cstr(b"78ms\0"),
    cstr(b"156ms\0"),
    cstr(b"312ms\0"),
    cstr(b"625ms\0"),
    cstr(b"1.25s\0"),
    cstr(b"2.5s\0"),
    cstr(b"5s\0"),
    cstr(b"10s\0"),
];
static filter_text: [*const c_char; 6] = [
    cstr(b"Disabled\0"),
    cstr(b"Elliptical HP 217Hz notch (16kHz)\0"),
    cstr(b"Butterworth HP 500Hz (16kHz)\0"),
    cstr(b"Elliptical HP 217Hz notch (8kHz)\0"),
    cstr(b"Butterworth HP 500Hz (8kHz)\0"),
    cstr(b"Butterworth HP 200Hz (48kHz)\0"),
];

static max9860_controls: [snd_kcontrol_new; 0] = [];
/*
 * Original ASoC controls:
 * SOC_SINGLE_TLV("Master Playback Volume", MAX9860_DACATTN, MAX9860_DVA_SHIFT, MAX9860_DVA_MUTE, 1, dva_tlv)
 * SOC_SINGLE_TLV("DAC Gain Volume", MAX9860_DACGAIN, MAX9860_DVG_SHIFT, MAX9860_DVG_MAX, 0, dvg_tlv)
 * SOC_DOUBLE_TLV("Line Capture Volume", MAX9860_ADCLEVEL, MAX9860_ADCLL_SHIFT, MAX9860_ADCRL_SHIFT, MAX9860_ADCxL_MIN, 1, adc_tlv)
 * SOC_ENUM("AGC Hold Time", agchld_enum)
 * SOC_ENUM("AGC/Noise Gate Source", agcsrc_enum)
 * SOC_ENUM("AGC Attack Time", agcatk_enum)
 * SOC_ENUM("AGC Release Time", agcrls_enum)
 * SOC_SINGLE_TLV("Noise Gate Threshold Volume", MAX9860_NOISEGATE, MAX9860_ANTH_SHIFT, MAX9860_ANTH_MAX, 0, anth_tlv)
 * SOC_SINGLE_TLV("AGC Signal Threshold Volume", MAX9860_NOISEGATE, MAX9860_AGCTH_SHIFT, MAX9860_AGCTH_MIN, 1, agcth_tlv)
 * SOC_SINGLE_TLV("Mic PGA Volume", MAX9860_MICGAIN, MAX9860_PGAM_SHIFT, MAX9860_PGAM_MIN, 1, pgam_tlv)
 * SOC_SINGLE_TLV("Mic Preamp Volume", MAX9860_MICGAIN, MAX9860_PAM_SHIFT, MAX9860_PAM_MAX, 0, pam_tlv)
 * SOC_ENUM("ADC Filter", avflt_enum)
 * SOC_ENUM("DAC Filter", dvflt_enum)
 */
static max9860_dapm_widgets: [snd_soc_dapm_widget; 0] = [];
/*
 * Original ASoC DAPM widgets:
 * SND_SOC_DAPM_INPUT("MICL")
 * SND_SOC_DAPM_INPUT("MICR")
 * SND_SOC_DAPM_ADC("ADCL", NULL, MAX9860_PWRMAN, MAX9860_ADCLEN_SHIFT, 0)
 * SND_SOC_DAPM_ADC("ADCR", NULL, MAX9860_PWRMAN, MAX9860_ADCREN_SHIFT, 0)
 * SND_SOC_DAPM_AIF_OUT("AIFOUTL", "Capture", 0, SND_SOC_NOPM, 0, 0)
 * SND_SOC_DAPM_AIF_OUT("AIFOUTR", "Capture", 1, SND_SOC_NOPM, 0, 0)
 * SND_SOC_DAPM_AIF_IN("AIFINL", "Playback", 0, SND_SOC_NOPM, 0, 0)
 * SND_SOC_DAPM_AIF_IN("AIFINR", "Playback", 1, SND_SOC_NOPM, 0, 0)
 * SND_SOC_DAPM_DAC("DAC", NULL, MAX9860_PWRMAN, MAX9860_DACEN_SHIFT, 0)
 * SND_SOC_DAPM_OUTPUT("OUT")
 * SND_SOC_DAPM_SUPPLY("Supply", SND_SOC_NOPM, 0, 0, NULL, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD)
 * SND_SOC_DAPM_REGULATOR_SUPPLY("AVDD", 0, 0)
 * SND_SOC_DAPM_REGULATOR_SUPPLY("DVDD", 0, 0)
 * SND_SOC_DAPM_CLOCK_SUPPLY("mclk")
 */
static max9860_dapm_routes: [snd_soc_dapm_route; 13] = [
    snd_soc_dapm_route { sink: cstr(b"ADCL\0"), control: null(), source: cstr(b"MICL\0") },
    snd_soc_dapm_route { sink: cstr(b"ADCR\0"), control: null(), source: cstr(b"MICR\0") },
    snd_soc_dapm_route { sink: cstr(b"AIFOUTL\0"), control: null(), source: cstr(b"ADCL\0") },
    snd_soc_dapm_route { sink: cstr(b"AIFOUTR\0"), control: null(), source: cstr(b"ADCR\0") },
    snd_soc_dapm_route { sink: cstr(b"DAC\0"), control: null(), source: cstr(b"AIFINL\0") },
    snd_soc_dapm_route { sink: cstr(b"DAC\0"), control: null(), source: cstr(b"AIFINR\0") },
    snd_soc_dapm_route { sink: cstr(b"OUT\0"), control: null(), source: cstr(b"DAC\0") },
    snd_soc_dapm_route { sink: cstr(b"Supply\0"), control: null(), source: cstr(b"AVDD\0") },
    snd_soc_dapm_route { sink: cstr(b"Supply\0"), control: null(), source: cstr(b"DVDD\0") },
    snd_soc_dapm_route { sink: cstr(b"Supply\0"), control: null(), source: cstr(b"mclk\0") },
    snd_soc_dapm_route { sink: cstr(b"DAC\0"), control: null(), source: cstr(b"Supply\0") },
    snd_soc_dapm_route { sink: cstr(b"ADCL\0"), control: null(), source: cstr(b"Supply\0") },
    snd_soc_dapm_route { sink: cstr(b"ADCR\0"), control: null(), source: cstr(b"Supply\0") },
];

fn div_round_closest_ull(n: u64, d: c_ulong) -> c_ulong {
    ((n + (d as u64 / 2)) / d as u64) as c_ulong
}

unsafe extern "C" fn max9860_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let max9860 = snd_soc_component_get_drvdata(component) as *mut max9860_priv;
    let mut master: u8;
    let mut ifc1a: u8 = 0;
    let mut ifc1b: u8 = 0;
    let mut sysclk: u8 = 0;
    let mut n: c_ulong;
    let mut ret: c_int;

    dev_dbg(
        (*component).dev,
        cstr(b"hw_params %u Hz, %u channels\n\0"),
        params_rate(params),
        params_channels(params),
    );

    if params_channels(params) == 2 {
        ifc1b |= MAX9860_ST;
    }

    match ((*max9860).fmt as c_uint) & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => master = 0,
        SND_SOC_DAIFMT_CBP_CFP => master = MAX9860_MASTER,
        _ => return -EINVAL,
    }
    ifc1a |= master;

    if master != 0 {
        if params_width(params) * params_channels(params) > 48 {
            ifc1b |= MAX9860_BSEL_64X;
        } else {
            ifc1b |= MAX9860_BSEL_48X;
        }
    }

    match ((*max9860).fmt as c_uint) & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            ifc1a |= MAX9860_DDLY;
            ifc1b |= MAX9860_ADLY;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            ifc1a |= MAX9860_WCI;
        }
        SND_SOC_DAIFMT_DSP_A => {
            if params_width(params) != 16 {
                dev_err((*component).dev, cstr(b"DSP_A works for 16 bits per sample only.\n\0"));
                return -EINVAL;
            }
            ifc1a |= MAX9860_DDLY | MAX9860_WCI | MAX9860_HIZ | MAX9860_TDM;
            ifc1b |= MAX9860_ADLY;
        }
        SND_SOC_DAIFMT_DSP_B => {
            if params_width(params) != 16 {
                dev_err((*component).dev, cstr(b"DSP_B works for 16 bits per sample only.\n\0"));
                return -EINVAL;
            }
            ifc1a |= MAX9860_WCI | MAX9860_HIZ | MAX9860_TDM;
        }
        _ => return -EINVAL,
    }

    match ((*max9860).fmt as c_uint) & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_NB_IF => {
            match ((*max9860).fmt as c_uint) & SND_SOC_DAIFMT_FORMAT_MASK {
                SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => return -EINVAL,
                _ => {}
            }
            ifc1a ^= MAX9860_WCI;
        }
        SND_SOC_DAIFMT_IB_IF => {
            match ((*max9860).fmt as c_uint) & SND_SOC_DAIFMT_FORMAT_MASK {
                SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => return -EINVAL,
                _ => {}
            }
            ifc1a ^= MAX9860_WCI;
            ifc1a ^= MAX9860_DBCI;
            ifc1b ^= MAX9860_ABCI;
        }
        SND_SOC_DAIFMT_IB_NF => {
            ifc1a ^= MAX9860_DBCI;
            ifc1b ^= MAX9860_ABCI;
        }
        _ => return -EINVAL,
    }

    dev_dbg((*component).dev, cstr(b"IFC1A  %02x\n\0"), ifc1a as c_uint);
    ret = regmap_write((*max9860).regmap, MAX9860_IFC1A, ifc1a as c_uint);
    if ret != 0 {
        dev_err((*component).dev, cstr(b"Failed to set IFC1A: %d\n\0"), ret);
        return ret;
    }
    dev_dbg((*component).dev, cstr(b"IFC1B  %02x\n\0"), ifc1b as c_uint);
    ret = regmap_write((*max9860).regmap, MAX9860_IFC1B, ifc1b as c_uint);
    if ret != 0 {
        dev_err((*component).dev, cstr(b"Failed to set IFC1B: %d\n\0"), ret);
        return ret;
    }

    /*
     * Check if Integer Clock Mode is possible, but avoid it in slave mode
     * since we then do not know if lrclk is derived from pclk and the
     * datasheet mentions that the frequencies have to match exactly in
     * order for this to work.
     */
    if params_rate(params) == 8000 || params_rate(params) == 16000 {
        if master != 0 {
            match (*max9860).pclk_rate {
                12000000 => sysclk = MAX9860_FREQ_12MHZ,
                13000000 => sysclk = MAX9860_FREQ_13MHZ,
                19200000 => sysclk = MAX9860_FREQ_19_2MHZ,
                _ => {
                    /*
                     * Integer Clock Mode not possible. Leave
                     * sysclk at zero and fall through to the
                     * code below for PLL mode.
                     */
                }
            }

            if sysclk != 0 && params_rate(params) == 16000 {
                sysclk |= MAX9860_16KHZ;
            }
        }
    }

    /*
     * Largest possible n:
     *    65536 * 96 * 48kHz / 10MHz -> 30199
     * Smallest possible n:
     *    65536 * 96 *  8kHz / 20MHz -> 2517
     * Both fit nicely in the available 15 bits, no need to apply any mask.
     */
    n = div_round_closest_ull(65536u64 * 96 * params_rate(params) as u64, (*max9860).pclk_rate);

    if sysclk == 0 {
        /* PLL mode */
        if params_rate(params) > 24000 {
            sysclk |= MAX9860_16KHZ;
        }

        if master == 0 {
            n |= 1; /* trigger rapid pll lock mode */
        }
    }

    sysclk |= (*max9860).psclk;
    dev_dbg((*component).dev, cstr(b"SYSCLK %02x\n\0"), sysclk as c_uint);
    ret = regmap_write((*max9860).regmap, MAX9860_SYSCLK, sysclk as c_uint);
    if ret != 0 {
        dev_err((*component).dev, cstr(b"Failed to set SYSCLK: %d\n\0"), ret);
        return ret;
    }
    dev_dbg((*component).dev, cstr(b"N %lu\n\0"), n);
    ret = regmap_write((*max9860).regmap, MAX9860_AUDIOCLKHIGH, (n >> 8) as c_uint);
    if ret != 0 {
        dev_err((*component).dev, cstr(b"Failed to set NHI: %d\n\0"), ret);
        return ret;
    }
    ret = regmap_write((*max9860).regmap, MAX9860_AUDIOCLKLOW, (n & 0xff) as c_uint);
    if ret != 0 {
        dev_err((*component).dev, cstr(b"Failed to set NLO: %d\n\0"), ret);
        return ret;
    }

    if master == 0 {
        dev_dbg((*component).dev, cstr(b"Enable PLL\n\0"));
        ret = regmap_update_bits(
            (*max9860).regmap,
            MAX9860_AUDIOCLKHIGH,
            MAX9860_PLL,
            MAX9860_PLL,
        );
        if ret != 0 {
            dev_err((*component).dev, cstr(b"Failed to enable PLL: %d\n\0"), ret);
            return ret;
        }
    }

    0
}

unsafe extern "C" fn max9860_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let max9860 = snd_soc_component_get_drvdata(component) as *mut max9860_priv;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP | SND_SOC_DAIFMT_CBC_CFC => {
            (*max9860).fmt = fmt as c_int;
            0
        }
        _ => -EINVAL,
    }
}

static max9860_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(max9860_hw_params),
    set_fmt: Some(max9860_set_fmt),
};

static mut max9860_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr(b"max9860-hifi\0"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr(b"Playback\0"),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 8000,
        rate_max: 48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    },
    capture: snd_soc_pcm_stream {
        stream_name: cstr(b"Capture\0"),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 8000,
        rate_max: 48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    },
    ops: &max9860_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn max9860_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let max9860 = dev_get_drvdata((*component).dev) as *mut max9860_priv;
    let mut ret: c_int;

    match level {
        SND_SOC_BIAS_ON | SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            ret = regmap_update_bits((*max9860).regmap, MAX9860_PWRMAN, MAX9860_SHDN, MAX9860_SHDN);
            if ret != 0 {
                dev_err((*component).dev, cstr(b"Failed to remove SHDN: %d\n\0"), ret);
                return ret;
            }
        }
        SND_SOC_BIAS_OFF => {
            ret = regmap_update_bits((*max9860).regmap, MAX9860_PWRMAN, MAX9860_SHDN, 0);
            if ret != 0 {
                dev_err((*component).dev, cstr(b"Failed to request SHDN: %d\n\0"), ret);
                return ret;
            }
        }
        _ => {}
    }

    0
}

static max9860_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(max9860_set_bias_level),
    controls: max9860_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(max9860_controls),
    dapm_widgets: max9860_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(max9860_dapm_widgets),
    dapm_routes: max9860_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(max9860_dapm_routes),
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn max9860_suspend(dev: *mut device) -> c_int {
    let max9860 = dev_get_drvdata(dev) as *mut max9860_priv;
    let mut ret: c_int;

    ret = regmap_update_bits(
        (*max9860).regmap,
        MAX9860_SYSCLK,
        MAX9860_PSCLK,
        MAX9860_PSCLK_OFF,
    );
    if ret != 0 {
        dev_err(dev, cstr(b"Failed to disable clock: %d\n\0"), ret);
        return ret;
    }

    regulator_disable((*max9860).dvddio);

    0
}

unsafe extern "C" fn max9860_resume(dev: *mut device) -> c_int {
    let max9860 = dev_get_drvdata(dev) as *mut max9860_priv;
    let mut ret: c_int;

    ret = regulator_enable((*max9860).dvddio);
    if ret != 0 {
        dev_err(dev, cstr(b"Failed to enable DVDDIO: %d\n\0"), ret);
        return ret;
    }

    regcache_cache_only((*max9860).regmap, false);
    ret = regcache_sync((*max9860).regmap);
    if ret != 0 {
        dev_err(dev, cstr(b"Failed to sync cache: %d\n\0"), ret);
        return ret;
    }

    ret = regmap_update_bits(
        (*max9860).regmap,
        MAX9860_SYSCLK,
        MAX9860_PSCLK,
        (*max9860).psclk as c_uint,
    );
    if ret != 0 {
        dev_err(dev, cstr(b"Failed to enable clock: %d\n\0"), ret);
        return ret;
    }

    0
}

static max9860_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(max9860_suspend),
    runtime_resume: Some(max9860_resume),
    runtime_idle: None,
};

unsafe extern "C" fn max9860_probe(i2c: *mut i2c_client) -> c_int {
    let dev = addr_of_mut!((*i2c).dev);
    let max9860: *mut max9860_priv;
    let mut ret: c_int;
    let mclk: *mut clk;
    let mclk_rate: c_ulong;
    let mut i: c_int;
    let mut intr: c_int = 0;

    max9860 = devm_kzalloc(dev, size_of::<max9860_priv>(), GFP_KERNEL) as *mut max9860_priv;
    if max9860.is_null() {
        return -ENOMEM;
    }

    (*max9860).dvddio = devm_regulator_get(dev, cstr(b"DVDDIO\0"));
    if IS_ERR((*max9860).dvddio as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*max9860).dvddio as *const c_void),
            cstr(b"Failed to get DVDDIO supply\n\0"),
        );
    }

    (*max9860).dvddio_nb.notifier_call = Some(max9860_dvddio_event);

    ret = devm_regulator_register_notifier((*max9860).dvddio, addr_of_mut!((*max9860).dvddio_nb));
    if ret != 0 {
        dev_err(dev, cstr(b"Failed to register DVDDIO notifier: %d\n\0"), ret);
    }

    ret = regulator_enable((*max9860).dvddio);
    if ret != 0 {
        dev_err(dev, cstr(b"Failed to enable DVDDIO: %d\n\0"), ret);
        return ret;
    }

    (*max9860).regmap = devm_regmap_init_i2c(i2c, &max9860_regmap);
    if IS_ERR((*max9860).regmap as *const c_void) {
        ret = PTR_ERR((*max9860).regmap as *const c_void);
        regulator_disable((*max9860).dvddio);
        return ret;
    }

    dev_set_drvdata(dev, max9860 as *mut c_void);

    /*
     * mclk has to be in the 10MHz to 60MHz range.
     * psclk is used to scale mclk into pclk so that
     * pclk is in the 10MHz to 20MHz range.
     */
    mclk = clk_get(dev, cstr(b"mclk\0"));

    if IS_ERR(mclk as *const c_void) {
        ret = PTR_ERR(mclk as *const c_void);
        dev_err_probe(dev, ret, cstr(b"Failed to get MCLK\n\0"));
        regulator_disable((*max9860).dvddio);
        return ret;
    }

    mclk_rate = clk_get_rate(mclk);
    clk_put(mclk);

    if mclk_rate > 60000000 || mclk_rate < 10000000 {
        dev_err(
            dev,
            cstr(b"Bad mclk %luHz (needs 10MHz - 60MHz)\n\0"),
            mclk_rate,
        );
        ret = -EINVAL;
        regulator_disable((*max9860).dvddio);
        return ret;
    }
    if mclk_rate >= 40000000 {
        (*max9860).psclk = 3;
    } else if mclk_rate >= 20000000 {
        (*max9860).psclk = 2;
    } else {
        (*max9860).psclk = 1;
    }
    (*max9860).pclk_rate = mclk_rate >> ((*max9860).psclk - 1);
    (*max9860).psclk <<= MAX9860_PSCLK_SHIFT;
    dev_dbg(
        dev,
        cstr(b"mclk %lu pclk %lu\n\0"),
        mclk_rate,
        (*max9860).pclk_rate,
    );

    regcache_cache_bypass((*max9860).regmap, true);
    i = 0;
    while i < max9860_regmap.num_reg_defaults as c_int {
        ret = regmap_write(
            (*max9860).regmap,
            (*max9860_regmap.reg_defaults.add(i as usize)).reg,
            (*max9860_regmap.reg_defaults.add(i as usize)).def,
        );
        if ret != 0 {
            dev_err(
                dev,
                cstr(b"Failed to initialize register %u: %d\n\0"),
                (*max9860_regmap.reg_defaults.add(i as usize)).reg,
                ret,
            );
            regulator_disable((*max9860).dvddio);
            return ret;
        }
        i += 1;
    }
    regcache_cache_bypass((*max9860).regmap, false);

    ret = regmap_read((*max9860).regmap, MAX9860_INTRSTATUS, &mut intr);
    if ret != 0 {
        dev_err(dev, cstr(b"Failed to clear INTRSTATUS: %d\n\0"), ret);
        regulator_disable((*max9860).dvddio);
        return ret;
    }

    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    pm_runtime_idle(dev);

    ret = devm_snd_soc_register_component(
        dev,
        &max9860_component_driver,
        addr_of_mut!(max9860_dai),
        1,
    );
    if ret != 0 {
        dev_err(dev, cstr(b"Failed to register CODEC: %d\n\0"), ret);
        pm_runtime_disable(dev);
        regulator_disable((*max9860).dvddio);
        return ret;
    }

    0
}

unsafe extern "C" fn max9860_remove(i2c: *mut i2c_client) {
    let dev = addr_of_mut!((*i2c).dev);
    let max9860 = dev_get_drvdata(dev) as *mut max9860_priv;

    pm_runtime_disable(dev);
    regulator_disable((*max9860).dvddio);
}

static max9860_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [
        b'm' as c_char, b'a' as c_char, b'x' as c_char, b'9' as c_char, b'8' as c_char,
        b'6' as c_char, b'0' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ] },
    i2c_device_id { name: [0; 20] },
];
// MODULE_DEVICE_TABLE(i2c, max9860_i2c_id);

static max9860_of_match: [of_device_id; 2] = [
    of_device_id { compatible: cstr(b"maxim,max9860\0") },
    of_device_id { compatible: null() },
];
// MODULE_DEVICE_TABLE(of, max9860_of_match);

static mut max9860_i2c_driver: i2c_driver = i2c_driver {
    probe: Some(max9860_probe),
    remove: Some(max9860_remove),
    id_table: max9860_i2c_id.as_ptr(),
    driver: device_driver {
        name: cstr(b"max9860\0"),
        of_match_table: max9860_of_match.as_ptr(),
        pm: &max9860_pm_ops,
    },
};

// module_i2c_driver(max9860_i2c_driver);
// MODULE_DESCRIPTION("ASoC MAX9860 Mono Audio Voice Codec driver");
// MODULE_AUTHOR("Peter Rosin <peda@axentia.se>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
