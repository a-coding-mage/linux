// SPDX-License-Identifier: GPL-2.0
//
// nau8822.c  --  NAU8822 ALSA Soc Audio driver
//
// Copyright 2017 Nuvoton Technology Crop.
//
// Author: David Lin <ctlin0@nuvoton.com>
// Co-author: John Hsu <kchsu0@nuvoton.com>
// Co-author: Seven Li <wtli@nuvoton.com>
//
// Based on WM8974.c

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const NAU_PLL_FREQ_MAX: c_uint = 100000000;
const NAU_PLL_FREQ_MIN: c_uint = 90000000;
const NAU_PLL_REF_MAX: c_uint = 33000000;
const NAU_PLL_REF_MIN: c_uint = 8000000;
const NAU_PLL_OPTOP_MIN: c_uint = 6;

const GFP_KERNEL: c_uint = 0;
const GFP_DMA: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const REGCACHE_RBTREE: c_uint = 0;
const SND_SOC_NOPM: c_uint = 0;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub bytes: snd_ctl_elem_value_bytes,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_bytes {
    pub data: [u8; 512],
}
#[repr(C)]
pub struct soc_bytes_ext {
    pub max: c_uint,
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
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
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
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
    pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub no_capture_mute: c_uint,
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
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
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
pub struct driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}
#[repr(C)]
pub struct nau8822_pll {
    pub freq_in: c_uint,
    pub freq_out: c_uint,
    pub mclk_scaler: c_int,
    pub pre_factor: c_int,
    pub pll_int: c_uint,
    pub pll_frac: c_uint,
}
#[repr(C)]
pub struct nau8822 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub supplies: [regulator_bulk_data; NAU8822_NUM_SUPPLIES as usize],
    pub pll: nau8822_pll,
    pub div_id: c_int,
    pub sysclk: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

unsafe extern "C" {
    static NAU8822_REG_POWER_MANAGEMENT_1: c_uint;
    static NAU8822_REG_POWER_MANAGEMENT_2: c_uint;
    static NAU8822_REG_POWER_MANAGEMENT_3: c_uint;
    static NAU8822_REG_AUDIO_INTERFACE: c_uint;
    static NAU8822_REG_COMPANDING_CONTROL: c_uint;
    static NAU8822_REG_CLOCKING: c_uint;
    static NAU8822_REG_ADDITIONAL_CONTROL: c_uint;
    static NAU8822_REG_GPIO_CONTROL: c_uint;
    static NAU8822_REG_JACK_DETECT_CONTROL_1: c_uint;
    static NAU8822_REG_DAC_CONTROL: c_uint;
    static NAU8822_REG_LEFT_DAC_DIGITAL_VOLUME: c_uint;
    static NAU8822_REG_RIGHT_DAC_DIGITAL_VOLUME: c_uint;
    static NAU8822_REG_JACK_DETECT_CONTROL_2: c_uint;
    static NAU8822_REG_ADC_CONTROL: c_uint;
    static NAU8822_REG_LEFT_ADC_DIGITAL_VOLUME: c_uint;
    static NAU8822_REG_RIGHT_ADC_DIGITAL_VOLUME: c_uint;
    static NAU8822_REG_EQ1: c_uint;
    static NAU8822_REG_EQ2: c_uint;
    static NAU8822_REG_EQ3: c_uint;
    static NAU8822_REG_EQ4: c_uint;
    static NAU8822_REG_EQ5: c_uint;
    static NAU8822_REG_DAC_LIMITER_1: c_uint;
    static NAU8822_REG_DAC_LIMITER_2: c_uint;
    static NAU8822_REG_NOTCH_FILTER_1: c_uint;
    static NAU8822_REG_NOTCH_FILTER_2: c_uint;
    static NAU8822_REG_NOTCH_FILTER_3: c_uint;
    static NAU8822_REG_NOTCH_FILTER_4: c_uint;
    static NAU8822_REG_ALC_CONTROL_1: c_uint;
    static NAU8822_REG_ALC_CONTROL_2: c_uint;
    static NAU8822_REG_ALC_CONTROL_3: c_uint;
    static NAU8822_REG_NOISE_GATE: c_uint;
    static NAU8822_REG_PLL_N: c_uint;
    static NAU8822_REG_PLL_K1: c_uint;
    static NAU8822_REG_PLL_K2: c_uint;
    static NAU8822_REG_PLL_K3: c_uint;
    static NAU8822_REG_3D_CONTROL: c_uint;
    static NAU8822_REG_RIGHT_SPEAKER_CONTROL: c_uint;
    static NAU8822_REG_INPUT_CONTROL: c_uint;
    static NAU8822_REG_LEFT_INP_PGA_CONTROL: c_uint;
    static NAU8822_REG_RIGHT_INP_PGA_CONTROL: c_uint;
    static NAU8822_REG_LEFT_ADC_BOOST_CONTROL: c_uint;
    static NAU8822_REG_RIGHT_ADC_BOOST_CONTROL: c_uint;
    static NAU8822_REG_OUTPUT_CONTROL: c_uint;
    static NAU8822_REG_LEFT_MIXER_CONTROL: c_uint;
    static NAU8822_REG_RIGHT_MIXER_CONTROL: c_uint;
    static NAU8822_REG_LHP_VOLUME: c_uint;
    static NAU8822_REG_RHP_VOLUME: c_uint;
    static NAU8822_REG_LSPKOUT_VOLUME: c_uint;
    static NAU8822_REG_RSPKOUT_VOLUME: c_uint;
    static NAU8822_REG_AUX2_MIXER: c_uint;
    static NAU8822_REG_AUX1_MIXER: c_uint;
    static NAU8822_REG_POWER_MANAGEMENT_4: c_uint;
    static NAU8822_REG_LEFT_TIME_SLOT: c_uint;
    static NAU8822_REG_MISC: c_uint;
    static NAU8822_REG_RIGHT_TIME_SLOT: c_uint;
    static NAU8822_REG_DEVICE_REVISION: c_uint;
    static NAU8822_REG_DEVICE_ID: c_uint;
    static NAU8822_REG_DAC_DITHER: c_uint;
    static NAU8822_REG_ALC_ENHANCE_1: c_uint;
    static NAU8822_REG_ALC_ENHANCE_2: c_uint;
    static NAU8822_REG_192KHZ_SAMPLING: c_uint;
    static NAU8822_REG_MISC_CONTROL: c_uint;
    static NAU8822_REG_INPUT_TIEOFF: c_uint;
    static NAU8822_REG_POWER_REDUCTION: c_uint;
    static NAU8822_REG_AGC_PEAK2PEAK: c_uint;
    static NAU8822_REG_AGC_PEAK_DETECT: c_uint;
    static NAU8822_REG_AUTOMUTE_CONTROL: c_uint;
    static NAU8822_REG_OUTPUT_TIEOFF: c_uint;
    static NAU8822_REG_RESET: c_uint;
    static NAU8822_REG_MAX_REGISTER: c_uint;
    static NAU8822_NUM_SUPPLIES: c_uint;
    static NAU8822_ADCCM_SFT: c_uint;
    static NAU8822_DACCM_SFT: c_uint;
    static NAU8822_EQM_SFT: c_uint;
    static NAU8822_ALCEN_SFT: c_uint;
    static NAU8822_ALCM_SFT: c_uint;
    static NAU8822_ADDAP_SFT: c_uint;
    static NAU8822_CLKM_MASK: c_uint;
    static NAU8822_CLK_MCLK: c_int;
    static NAU8822_CLK_PLL: c_int;
    static NAU8822_MCLKSEL_MASK: c_uint;
    static NAU8822_MCLKSEL_SFT: c_uint;
    static NAU8822_CLKM_MCLK: c_uint;
    static NAU8822_CLKM_PLL: c_uint;
    static NAU8822_PLL_EN_MASK: c_uint;
    static NAU8822_PLL_OFF: c_uint;
    static NAU8822_PLL_ON: c_uint;
    static NAU8822_PLLMCLK_DIV2: c_uint;
    static NAU8822_PLLN_MASK: c_uint;
    static NAU8822_PLLK1_SFT: c_uint;
    static NAU8822_PLLK1_MASK: c_uint;
    static NAU8822_PLLK2_SFT: c_uint;
    static NAU8822_PLLK2_MASK: c_uint;
    static NAU8822_PLLK3_MASK: c_uint;
    static NAU8822_CLKIOEN_MASK: c_uint;
    static NAU8822_AIFMT_MASK: c_uint;
    static NAU8822_LRP_MASK: c_uint;
    static NAU8822_BCLKP_MASK: c_uint;
    static NAU8822_CLK_MASTER: c_uint;
    static NAU8822_BCLKDIV_8: c_uint;
    static NAU8822_BCLKDIV_4: c_uint;
    static NAU8822_BCLKDIV_2: c_uint;
    static NAU8822_BCLKSEL_MASK: c_uint;
    static NAU8822_WLEN_20: c_int;
    static NAU8822_WLEN_24: c_int;
    static NAU8822_WLEN_32: c_int;
    static NAU8822_WLEN_MASK: c_uint;
    static NAU8822_SMPLR_8K: c_int;
    static NAU8822_SMPLR_12K: c_int;
    static NAU8822_SMPLR_16K: c_int;
    static NAU8822_SMPLR_24K: c_int;
    static NAU8822_SMPLR_32K: c_int;
    static NAU8822_SMPLR_MASK: c_uint;
    static NAU8822_REFIMP_MASK: c_uint;
    static NAU8822_REFIMP_80K: c_uint;
    static NAU8822_REFIMP_3K: c_uint;
    static NAU8822_REFIMP_300K: c_uint;
    static NAU8822_IOBUF_EN: c_uint;
    static NAU8822_ABIAS_EN: c_uint;
    static NAU8822_RSUBBYP: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: c_uint;
    static SNDRV_PCM_FORMAT_S20_3LE: c_uint;
    static SNDRV_PCM_FORMAT_S24_LE: c_uint;
    static SNDRV_PCM_FORMAT_S32_LE: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_uint;
    fn kmemdup(src: *const c_void, len: c_uint, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(i2c: *mut i2c_client) -> *mut c_void;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_regulator_bulk_get(dev: *mut device, num: c_uint, supplies: *mut regulator_bulk_data) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn regulator_bulk_enable(num: c_uint, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_uint, supplies: *mut regulator_bulk_data) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn mdelay(msecs: c_uint);
    fn fsleep(usecs: c_uint);
    fn of_match_ptr(matches: *const of_device_id) -> *const of_device_id;
}

const NAU8822_MCLK_SCALER: [c_int; 8] = [10, 15, 20, 30, 40, 60, 80, 120];

static NAU8822_REG_DEFAULTS: &[reg_default] = &[
    reg_default { reg: unsafe { NAU8822_REG_POWER_MANAGEMENT_1 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_POWER_MANAGEMENT_2 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_POWER_MANAGEMENT_3 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_AUDIO_INTERFACE }, def: 0x0050 },
    reg_default { reg: unsafe { NAU8822_REG_COMPANDING_CONTROL }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_CLOCKING }, def: 0x0140 },
    reg_default { reg: unsafe { NAU8822_REG_ADDITIONAL_CONTROL }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_GPIO_CONTROL }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_JACK_DETECT_CONTROL_1 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_DAC_CONTROL }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_LEFT_DAC_DIGITAL_VOLUME }, def: 0x00ff },
    reg_default { reg: unsafe { NAU8822_REG_RIGHT_DAC_DIGITAL_VOLUME }, def: 0x00ff },
    reg_default { reg: unsafe { NAU8822_REG_JACK_DETECT_CONTROL_2 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_ADC_CONTROL }, def: 0x0100 },
    reg_default { reg: unsafe { NAU8822_REG_LEFT_ADC_DIGITAL_VOLUME }, def: 0x00ff },
    reg_default { reg: unsafe { NAU8822_REG_RIGHT_ADC_DIGITAL_VOLUME }, def: 0x00ff },
    reg_default { reg: unsafe { NAU8822_REG_EQ1 }, def: 0x012c },
    reg_default { reg: unsafe { NAU8822_REG_EQ2 }, def: 0x002c },
    reg_default { reg: unsafe { NAU8822_REG_EQ3 }, def: 0x002c },
    reg_default { reg: unsafe { NAU8822_REG_EQ4 }, def: 0x002c },
    reg_default { reg: unsafe { NAU8822_REG_EQ5 }, def: 0x002c },
    reg_default { reg: unsafe { NAU8822_REG_DAC_LIMITER_1 }, def: 0x0032 },
    reg_default { reg: unsafe { NAU8822_REG_DAC_LIMITER_2 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_NOTCH_FILTER_1 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_NOTCH_FILTER_2 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_NOTCH_FILTER_3 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_NOTCH_FILTER_4 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_ALC_CONTROL_1 }, def: 0x0038 },
    reg_default { reg: unsafe { NAU8822_REG_ALC_CONTROL_2 }, def: 0x000b },
    reg_default { reg: unsafe { NAU8822_REG_ALC_CONTROL_3 }, def: 0x0032 },
    reg_default { reg: unsafe { NAU8822_REG_NOISE_GATE }, def: 0x0010 },
    reg_default { reg: unsafe { NAU8822_REG_PLL_N }, def: 0x0008 },
    reg_default { reg: unsafe { NAU8822_REG_PLL_K1 }, def: 0x000c },
    reg_default { reg: unsafe { NAU8822_REG_PLL_K2 }, def: 0x0093 },
    reg_default { reg: unsafe { NAU8822_REG_PLL_K3 }, def: 0x00e9 },
    reg_default { reg: unsafe { NAU8822_REG_3D_CONTROL }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_RIGHT_SPEAKER_CONTROL }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_INPUT_CONTROL }, def: 0x0033 },
    reg_default { reg: unsafe { NAU8822_REG_LEFT_INP_PGA_CONTROL }, def: 0x0010 },
    reg_default { reg: unsafe { NAU8822_REG_RIGHT_INP_PGA_CONTROL }, def: 0x0010 },
    reg_default { reg: unsafe { NAU8822_REG_LEFT_ADC_BOOST_CONTROL }, def: 0x0100 },
    reg_default { reg: unsafe { NAU8822_REG_RIGHT_ADC_BOOST_CONTROL }, def: 0x0100 },
    reg_default { reg: unsafe { NAU8822_REG_OUTPUT_CONTROL }, def: 0x0002 },
    reg_default { reg: unsafe { NAU8822_REG_LEFT_MIXER_CONTROL }, def: 0x0001 },
    reg_default { reg: unsafe { NAU8822_REG_RIGHT_MIXER_CONTROL }, def: 0x0001 },
    reg_default { reg: unsafe { NAU8822_REG_LHP_VOLUME }, def: 0x0039 },
    reg_default { reg: unsafe { NAU8822_REG_RHP_VOLUME }, def: 0x0039 },
    reg_default { reg: unsafe { NAU8822_REG_LSPKOUT_VOLUME }, def: 0x0039 },
    reg_default { reg: unsafe { NAU8822_REG_RSPKOUT_VOLUME }, def: 0x0039 },
    reg_default { reg: unsafe { NAU8822_REG_AUX2_MIXER }, def: 0x0001 },
    reg_default { reg: unsafe { NAU8822_REG_AUX1_MIXER }, def: 0x0001 },
    reg_default { reg: unsafe { NAU8822_REG_POWER_MANAGEMENT_4 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_LEFT_TIME_SLOT }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_MISC }, def: 0x0020 },
    reg_default { reg: unsafe { NAU8822_REG_RIGHT_TIME_SLOT }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_DEVICE_REVISION }, def: 0x007f },
    reg_default { reg: unsafe { NAU8822_REG_DEVICE_ID }, def: 0x001a },
    reg_default { reg: unsafe { NAU8822_REG_DAC_DITHER }, def: 0x0114 },
    reg_default { reg: unsafe { NAU8822_REG_ALC_ENHANCE_1 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_ALC_ENHANCE_2 }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_192KHZ_SAMPLING }, def: 0x0008 },
    reg_default { reg: unsafe { NAU8822_REG_MISC_CONTROL }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_INPUT_TIEOFF }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_POWER_REDUCTION }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_AGC_PEAK2PEAK }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_AGC_PEAK_DETECT }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_AUTOMUTE_CONTROL }, def: 0x0000 },
    reg_default { reg: unsafe { NAU8822_REG_OUTPUT_TIEOFF }, def: 0x0000 },
];

static NAU8822_SUPPLY_NAMES: [*const c_char; 4] = [
    b"vdda\0".as_ptr() as *const c_char,
    b"vddb\0".as_ptr() as *const c_char,
    b"vddc\0".as_ptr() as *const c_char,
    b"vddspk\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn nau8822_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    (reg >= NAU8822_REG_RESET && reg <= NAU8822_REG_JACK_DETECT_CONTROL_1)
        || (reg >= NAU8822_REG_DAC_CONTROL && reg <= NAU8822_REG_LEFT_ADC_DIGITAL_VOLUME)
        || reg == NAU8822_REG_RIGHT_ADC_DIGITAL_VOLUME
        || (reg >= NAU8822_REG_EQ1 && reg <= NAU8822_REG_EQ5)
        || (reg >= NAU8822_REG_DAC_LIMITER_1 && reg <= NAU8822_REG_DAC_LIMITER_2)
        || (reg >= NAU8822_REG_NOTCH_FILTER_1 && reg <= NAU8822_REG_NOTCH_FILTER_4)
        || (reg >= NAU8822_REG_ALC_CONTROL_1 && reg <= NAU8822_REG_PLL_K3)
        || reg == NAU8822_REG_3D_CONTROL
        || reg == NAU8822_REG_RIGHT_SPEAKER_CONTROL
        || (reg >= NAU8822_REG_INPUT_CONTROL && reg <= NAU8822_REG_LEFT_ADC_BOOST_CONTROL)
        || (reg >= NAU8822_REG_RIGHT_ADC_BOOST_CONTROL && reg <= NAU8822_REG_AUX1_MIXER)
        || (reg >= NAU8822_REG_POWER_MANAGEMENT_4 && reg <= NAU8822_REG_DEVICE_ID)
        || reg == NAU8822_REG_DAC_DITHER
        || (reg >= NAU8822_REG_ALC_ENHANCE_1 && reg <= NAU8822_REG_MISC_CONTROL)
        || (reg >= NAU8822_REG_INPUT_TIEOFF && reg <= NAU8822_REG_OUTPUT_TIEOFF)
}

unsafe extern "C" fn nau8822_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    nau8822_readable_reg(dev, reg)
}

unsafe extern "C" fn nau8822_volatile(_dev: *mut device, reg: c_uint) -> bool {
    reg == NAU8822_REG_RESET
        || reg == NAU8822_REG_DEVICE_REVISION
        || reg == NAU8822_REG_DEVICE_ID
        || reg == NAU8822_REG_AGC_PEAK2PEAK
        || reg == NAU8822_REG_AGC_PEAK_DETECT
        || reg == NAU8822_REG_AUTOMUTE_CONTROL
}

/* The EQ parameters get function is to get the 5 band equalizer control.
 * The regmap raw read can't work here because regmap doesn't provide
 * value format for value width of 9 bits. Therefore, the driver reads data
 * from cache and makes value format according to the endianness of
 * bytes type control element.
 */
unsafe extern "C" fn nau8822_eq_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let params = (*kcontrol).private_value as *mut soc_bytes_ext;
    let val = (*ucontrol).value.bytes.data.as_mut_ptr() as *mut u16;
    let reg = NAU8822_REG_EQ1;

    for i in 0..((*params).max as usize / size_of::<u16>()) {
        let reg_val = snd_soc_component_read(component, reg + i as c_uint) as u16;
        /* conversion of 16-bit integers between native CPU format
         * and big endian format
         */
        let tmp = reg_val.to_be();
        memcpy(val.add(i) as *mut c_void, &tmp as *const u16 as *const c_void, size_of::<u16>());
    }

    0
}

/* The EQ parameters put function is to make configuration of 5 band equalizer
 * control. These configuration includes central frequency, equalizer gain,
 * cut-off frequency, bandwidth control, and equalizer path.
 * The regmap raw write can't work here because regmap doesn't provide
 * register and value format for register with address 7 bits and value 9 bits.
 * Therefore, the driver makes value format according to the endianness of
 * bytes type control element and writes data to codec.
 */
unsafe extern "C" fn nau8822_eq_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let params = (*kcontrol).private_value as *mut soc_bytes_ext;
    let data = kmemdup(
        (*ucontrol).value.bytes.data.as_ptr() as *const c_void,
        (*params).max,
        GFP_KERNEL | GFP_DMA,
    );
    if data.is_null() {
        return -ENOMEM;
    }

    let val = data as *mut u16;
    let reg = NAU8822_REG_EQ1;
    for i in 0..((*params).max as usize / size_of::<u16>()) {
        /* conversion of 16-bit integers between native CPU format
         * and big endian format
         */
        let tmp = val.add(i) as *const u16;
        let value = u16::from_be(ptr::read_unaligned(tmp));
        let ret = snd_soc_component_write(component, reg + i as c_uint, value as c_uint);
        if ret != 0 {
            dev_err((*component).dev, b"EQ configuration fail, register: %x ret: %d\n\0".as_ptr() as *const c_char, reg + i as c_uint, ret);
            kfree(data);
            return ret;
        }
    }
    kfree(data);

    0
}

static NAU8822_COMPANDING: [*const c_char; 4] = [
    b"Off\0".as_ptr() as *const c_char,
    b"NC\0".as_ptr() as *const c_char,
    b"u-law\0".as_ptr() as *const c_char,
    b"A-law\0".as_ptr() as *const c_char,
];

static NAU8822_COMANDING_ADC_ENUM: soc_enum =
    SOC_ENUM_SINGLE!(NAU8822_REG_COMPANDING_CONTROL, NAU8822_ADCCM_SFT, NAU8822_COMPANDING.len(), NAU8822_COMPANDING);
static NAU8822_COMPANDING_DAC_ENUM: soc_enum =
    SOC_ENUM_SINGLE!(NAU8822_REG_COMPANDING_CONTROL, NAU8822_DACCM_SFT, NAU8822_COMPANDING.len(), NAU8822_COMPANDING);

static NAU8822_EQMODE: [*const c_char; 2] = [b"Capture\0".as_ptr() as *const c_char, b"Playback\0".as_ptr() as *const c_char];
static NAU8822_EQMODE_ENUM: soc_enum =
    SOC_ENUM_SINGLE!(NAU8822_REG_EQ1, NAU8822_EQM_SFT, NAU8822_EQMODE.len(), NAU8822_EQMODE);

static NAU8822_ALC1: [*const c_char; 4] = [
    b"Off\0".as_ptr() as *const c_char,
    b"Right\0".as_ptr() as *const c_char,
    b"Left\0".as_ptr() as *const c_char,
    b"Both\0".as_ptr() as *const c_char,
];
static NAU8822_ALC3: [*const c_char; 2] = [b"Normal\0".as_ptr() as *const c_char, b"Limiter\0".as_ptr() as *const c_char];

static NAU8822_ALC_ENABLE_ENUM: soc_enum =
    SOC_ENUM_SINGLE!(NAU8822_REG_ALC_CONTROL_1, NAU8822_ALCEN_SFT, NAU8822_ALC1.len(), NAU8822_ALC1);
static NAU8822_ALC_MODE_ENUM: soc_enum =
    SOC_ENUM_SINGLE!(NAU8822_REG_ALC_CONTROL_3, NAU8822_ALCM_SFT, NAU8822_ALC3.len(), NAU8822_ALC3);

static DIGITAL_TLV: &[c_uint] = &DECLARE_TLV_DB_SCALE!(digital_tlv, -12750, 50, 1);
static INPGA_TLV: &[c_uint] = &DECLARE_TLV_DB_SCALE!(inpga_tlv, -1200, 75, 0);
static SPK_TLV: &[c_uint] = &DECLARE_TLV_DB_SCALE!(spk_tlv, -5700, 100, 0);
static PGA_BOOST_TLV: &[c_uint] = &DECLARE_TLV_DB_SCALE!(pga_boost_tlv, 0, 2000, 0);
static BOOST_TLV: &[c_uint] = &DECLARE_TLV_DB_SCALE!(boost_tlv, -1500, 300, 1);
static LIMITER_TLV: &[c_uint] = &DECLARE_TLV_DB_SCALE!(limiter_tlv, 0, 100, 0);

static NAU8822_SND_CONTROLS: &[snd_kcontrol_new] = &[
    SOC_ENUM!("ADC Companding", NAU8822_COMANDING_ADC_ENUM),
    SOC_ENUM!("DAC Companding", NAU8822_COMPANDING_DAC_ENUM),
    SOC_ENUM!("EQ Function", NAU8822_EQMODE_ENUM),
    SND_SOC_BYTES_EXT!("EQ Parameters", 10, nau8822_eq_get, nau8822_eq_put),
    SOC_DOUBLE!("DAC Inversion Switch", NAU8822_REG_DAC_CONTROL, 0, 1, 1, 0),
    SOC_DOUBLE_R_TLV!("PCM Volume", NAU8822_REG_LEFT_DAC_DIGITAL_VOLUME, NAU8822_REG_RIGHT_DAC_DIGITAL_VOLUME, 0, 255, 0, DIGITAL_TLV),
    SOC_SINGLE!("High Pass Filter Switch", NAU8822_REG_ADC_CONTROL, 8, 1, 0),
    SOC_SINGLE!("High Pass Cut Off", NAU8822_REG_ADC_CONTROL, 4, 7, 0),
    SOC_DOUBLE!("ADC Inversion Switch", NAU8822_REG_ADC_CONTROL, 0, 1, 1, 0),
    SOC_DOUBLE_R_TLV!("ADC Volume", NAU8822_REG_LEFT_ADC_DIGITAL_VOLUME, NAU8822_REG_RIGHT_ADC_DIGITAL_VOLUME, 0, 255, 0, DIGITAL_TLV),
    SOC_SINGLE!("DAC Limiter Switch", NAU8822_REG_DAC_LIMITER_1, 8, 1, 0),
    SOC_SINGLE!("DAC Limiter Decay", NAU8822_REG_DAC_LIMITER_1, 4, 15, 0),
    SOC_SINGLE!("DAC Limiter Attack", NAU8822_REG_DAC_LIMITER_1, 0, 15, 0),
    SOC_SINGLE!("DAC Limiter Threshold", NAU8822_REG_DAC_LIMITER_2, 4, 7, 0),
    SOC_SINGLE_TLV!("DAC Limiter Volume", NAU8822_REG_DAC_LIMITER_2, 0, 12, 0, LIMITER_TLV),
    SOC_ENUM!("ALC Mode", NAU8822_ALC_MODE_ENUM),
    SOC_ENUM!("ALC Enable Switch", NAU8822_ALC_ENABLE_ENUM),
    SOC_SINGLE!("ALC Min Gain", NAU8822_REG_ALC_CONTROL_1, 0, 7, 0),
    SOC_SINGLE!("ALC Max Gain", NAU8822_REG_ALC_CONTROL_1, 3, 7, 0),
    SOC_SINGLE!("ALC Hold", NAU8822_REG_ALC_CONTROL_2, 4, 10, 0),
    SOC_SINGLE!("ALC Target", NAU8822_REG_ALC_CONTROL_2, 0, 15, 0),
    SOC_SINGLE!("ALC Decay", NAU8822_REG_ALC_CONTROL_3, 4, 10, 0),
    SOC_SINGLE!("ALC Attack", NAU8822_REG_ALC_CONTROL_3, 0, 10, 0),
    SOC_SINGLE!("ALC Noise Gate Switch", NAU8822_REG_NOISE_GATE, 3, 1, 0),
    SOC_SINGLE!("ALC Noise Gate Threshold", NAU8822_REG_NOISE_GATE, 0, 7, 0),
    SOC_DOUBLE_R!("PGA ZC Switch", NAU8822_REG_LEFT_INP_PGA_CONTROL, NAU8822_REG_RIGHT_INP_PGA_CONTROL, 7, 1, 0),
    SOC_DOUBLE_R_TLV!("PGA Volume", NAU8822_REG_LEFT_INP_PGA_CONTROL, NAU8822_REG_RIGHT_INP_PGA_CONTROL, 0, 63, 0, INPGA_TLV),
    SOC_DOUBLE_R!("Headphone ZC Switch", NAU8822_REG_LHP_VOLUME, NAU8822_REG_RHP_VOLUME, 7, 1, 0),
    SOC_DOUBLE_R!("Headphone Playback Switch", NAU8822_REG_LHP_VOLUME, NAU8822_REG_RHP_VOLUME, 6, 1, 1),
    SOC_DOUBLE_R_TLV!("Headphone Volume", NAU8822_REG_LHP_VOLUME, NAU8822_REG_RHP_VOLUME, 0, 63, 0, SPK_TLV),
    SOC_DOUBLE_R!("Speaker ZC Switch", NAU8822_REG_LSPKOUT_VOLUME, NAU8822_REG_RSPKOUT_VOLUME, 7, 1, 0),
    SOC_DOUBLE_R!("Speaker Playback Switch", NAU8822_REG_LSPKOUT_VOLUME, NAU8822_REG_RSPKOUT_VOLUME, 6, 1, 1),
    SOC_DOUBLE_R_TLV!("Speaker Volume", NAU8822_REG_LSPKOUT_VOLUME, NAU8822_REG_RSPKOUT_VOLUME, 0, 63, 0, SPK_TLV),
    SOC_DOUBLE_R!("AUXOUT Playback Switch", NAU8822_REG_AUX2_MIXER, NAU8822_REG_AUX1_MIXER, 6, 1, 1),
    SOC_DOUBLE_R_TLV!("PGA Boost Volume", NAU8822_REG_LEFT_ADC_BOOST_CONTROL, NAU8822_REG_RIGHT_ADC_BOOST_CONTROL, 8, 1, 0, PGA_BOOST_TLV),
    SOC_DOUBLE_R_TLV!("L2/R2 Boost Volume", NAU8822_REG_LEFT_ADC_BOOST_CONTROL, NAU8822_REG_RIGHT_ADC_BOOST_CONTROL, 4, 7, 0, BOOST_TLV),
    SOC_DOUBLE_R_TLV!("Aux Boost Volume", NAU8822_REG_LEFT_ADC_BOOST_CONTROL, NAU8822_REG_RIGHT_ADC_BOOST_CONTROL, 0, 7, 0, BOOST_TLV),
    SOC_SINGLE!("DAC 128x Oversampling Switch", NAU8822_REG_DAC_CONTROL, 5, 1, 0),
    SOC_SINGLE!("ADC 128x Oversampling Switch", NAU8822_REG_ADC_CONTROL, 5, 1, 0),
];

/* LMAIN and RMAIN Mixer */
static NAU8822_LEFT_OUT_MIXER: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("LINMIX Switch", NAU8822_REG_LEFT_MIXER_CONTROL, 1, 1, 0),
    SOC_DAPM_SINGLE!("LAUX Switch", NAU8822_REG_LEFT_MIXER_CONTROL, 5, 1, 0),
    SOC_DAPM_SINGLE!("LDAC Switch", NAU8822_REG_LEFT_MIXER_CONTROL, 0, 1, 0),
    SOC_DAPM_SINGLE!("RDAC Switch", NAU8822_REG_OUTPUT_CONTROL, 5, 1, 0),
];
static NAU8822_RIGHT_OUT_MIXER: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("RINMIX Switch", NAU8822_REG_RIGHT_MIXER_CONTROL, 1, 1, 0),
    SOC_DAPM_SINGLE!("RAUX Switch", NAU8822_REG_RIGHT_MIXER_CONTROL, 5, 1, 0),
    SOC_DAPM_SINGLE!("RDAC Switch", NAU8822_REG_RIGHT_MIXER_CONTROL, 0, 1, 0),
    SOC_DAPM_SINGLE!("LDAC Switch", NAU8822_REG_OUTPUT_CONTROL, 6, 1, 0),
];

/* AUX1 and AUX2 Mixer */
static NAU8822_AUXOUT1_MIXER: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("RDAC Switch", NAU8822_REG_AUX1_MIXER, 0, 1, 0),
    SOC_DAPM_SINGLE!("RMIX Switch", NAU8822_REG_AUX1_MIXER, 1, 1, 0),
    SOC_DAPM_SINGLE!("RINMIX Switch", NAU8822_REG_AUX1_MIXER, 2, 1, 0),
    SOC_DAPM_SINGLE!("LDAC Switch", NAU8822_REG_AUX1_MIXER, 3, 1, 0),
    SOC_DAPM_SINGLE!("LMIX Switch", NAU8822_REG_AUX1_MIXER, 4, 1, 0),
];
static NAU8822_AUXOUT2_MIXER: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("LDAC Switch", NAU8822_REG_AUX2_MIXER, 0, 1, 0),
    SOC_DAPM_SINGLE!("LMIX Switch", NAU8822_REG_AUX2_MIXER, 1, 1, 0),
    SOC_DAPM_SINGLE!("LINMIX Switch", NAU8822_REG_AUX2_MIXER, 2, 1, 0),
    SOC_DAPM_SINGLE!("AUX1MIX Output Switch", NAU8822_REG_AUX2_MIXER, 3, 1, 0),
];

/* Input PGA */
static NAU8822_LEFT_INPUT_MIXER: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("L2 Switch", NAU8822_REG_INPUT_CONTROL, 2, 1, 0),
    SOC_DAPM_SINGLE!("MicN Switch", NAU8822_REG_INPUT_CONTROL, 1, 1, 0),
    SOC_DAPM_SINGLE!("MicP Switch", NAU8822_REG_INPUT_CONTROL, 0, 1, 0),
];
static NAU8822_RIGHT_INPUT_MIXER: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("R2 Switch", NAU8822_REG_INPUT_CONTROL, 6, 1, 0),
    SOC_DAPM_SINGLE!("MicN Switch", NAU8822_REG_INPUT_CONTROL, 5, 1, 0),
    SOC_DAPM_SINGLE!("MicP Switch", NAU8822_REG_INPUT_CONTROL, 4, 1, 0),
];

/* Loopback Switch */
static NAU8822_LOOPBACK: snd_kcontrol_new =
    SOC_DAPM_SINGLE!("Switch", NAU8822_REG_COMPANDING_CONTROL, NAU8822_ADDAP_SFT, 1, 0);

unsafe extern "C" fn check_mclk_select_pll(source: *mut snd_soc_dapm_widget, _sink: *mut snd_soc_dapm_widget) -> c_int {
    let component = snd_soc_dapm_to_component((*source).dapm);
    let value = snd_soc_component_read(component, NAU8822_REG_CLOCKING);
    (value & NAU8822_CLKM_MASK) as c_int
}

static NAU8822_DAPM_WIDGETS: &[snd_soc_dapm_widget_desc] = &[
    SND_SOC_DAPM_DAC!("Left DAC", "Left HiFi Playback", NAU8822_REG_POWER_MANAGEMENT_3, 0, 0),
    SND_SOC_DAPM_DAC!("Right DAC", "Right HiFi Playback", NAU8822_REG_POWER_MANAGEMENT_3, 1, 0),
    SND_SOC_DAPM_ADC!("Left ADC", "Left HiFi Capture", NAU8822_REG_POWER_MANAGEMENT_2, 0, 0),
    SND_SOC_DAPM_ADC!("Right ADC", "Right HiFi Capture", NAU8822_REG_POWER_MANAGEMENT_2, 1, 0),
    SOC_MIXER_ARRAY!("Left Output Mixer", NAU8822_REG_POWER_MANAGEMENT_3, 2, 0, NAU8822_LEFT_OUT_MIXER),
    SOC_MIXER_ARRAY!("Right Output Mixer", NAU8822_REG_POWER_MANAGEMENT_3, 3, 0, NAU8822_RIGHT_OUT_MIXER),
    SOC_MIXER_ARRAY!("AUX1 Output Mixer", NAU8822_REG_POWER_MANAGEMENT_1, 7, 0, NAU8822_AUXOUT1_MIXER),
    SOC_MIXER_ARRAY!("AUX2 Output Mixer", NAU8822_REG_POWER_MANAGEMENT_1, 6, 0, NAU8822_AUXOUT2_MIXER),
    SOC_MIXER_ARRAY!("Left Input Mixer", NAU8822_REG_POWER_MANAGEMENT_2, 2, 0, NAU8822_LEFT_INPUT_MIXER),
    SOC_MIXER_ARRAY!("Right Input Mixer", NAU8822_REG_POWER_MANAGEMENT_2, 3, 0, NAU8822_RIGHT_INPUT_MIXER),
    SND_SOC_DAPM_PGA!("Left Boost Mixer", NAU8822_REG_POWER_MANAGEMENT_2, 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right Boost Mixer", NAU8822_REG_POWER_MANAGEMENT_2, 5, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left Capture PGA", NAU8822_REG_LEFT_INP_PGA_CONTROL, 6, 1, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right Capture PGA", NAU8822_REG_RIGHT_INP_PGA_CONTROL, 6, 1, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left Headphone Out", NAU8822_REG_POWER_MANAGEMENT_2, 7, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right Headphone Out", NAU8822_REG_POWER_MANAGEMENT_2, 8, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left Speaker Out", NAU8822_REG_POWER_MANAGEMENT_3, 6, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right Speaker Out", NAU8822_REG_POWER_MANAGEMENT_3, 5, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("AUX1 Out", NAU8822_REG_POWER_MANAGEMENT_3, 8, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("AUX2 Out", NAU8822_REG_POWER_MANAGEMENT_3, 7, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Bias", NAU8822_REG_POWER_MANAGEMENT_1, 4, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("PLL", NAU8822_REG_POWER_MANAGEMENT_1, 5, 0, ptr::null(), 0),
    SND_SOC_DAPM_SWITCH!("Digital Loopback", SND_SOC_NOPM, 0, 0, &NAU8822_LOOPBACK),
    SND_SOC_DAPM_INPUT!("LMICN"),
    SND_SOC_DAPM_INPUT!("LMICP"),
    SND_SOC_DAPM_INPUT!("RMICN"),
    SND_SOC_DAPM_INPUT!("RMICP"),
    SND_SOC_DAPM_INPUT!("LAUX"),
    SND_SOC_DAPM_INPUT!("RAUX"),
    SND_SOC_DAPM_INPUT!("L2"),
    SND_SOC_DAPM_INPUT!("R2"),
    SND_SOC_DAPM_OUTPUT!("LHP"),
    SND_SOC_DAPM_OUTPUT!("RHP"),
    SND_SOC_DAPM_OUTPUT!("LSPK"),
    SND_SOC_DAPM_OUTPUT!("RSPK"),
    SND_SOC_DAPM_OUTPUT!("AUXOUT1"),
    SND_SOC_DAPM_OUTPUT!("AUXOUT2"),
];

macro_rules! route {
    ($sink:expr, NULL, $source:expr) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: ptr::null(), source: concat!($source, "\0").as_ptr() as *const c_char, connected: None }
    };
    ($sink:expr, NULL, $source:expr, $connected:path) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: ptr::null(), source: concat!($source, "\0").as_ptr() as *const c_char, connected: Some($connected) }
    };
    ($sink:expr, $control:expr, $source:expr) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: concat!($control, "\0").as_ptr() as *const c_char, source: concat!($source, "\0").as_ptr() as *const c_char, connected: None }
    };
}

static NAU8822_DAPM_ROUTES: &[snd_soc_dapm_route] = &[
    route!("Right DAC", NULL, "PLL", check_mclk_select_pll),
    route!("Left DAC", NULL, "PLL", check_mclk_select_pll),
    /* LMAIN and RMAIN Mixer */
    route!("Right Output Mixer", "LDAC Switch", "Left DAC"),
    route!("Right Output Mixer", "RDAC Switch", "Right DAC"),
    route!("Right Output Mixer", "RAUX Switch", "RAUX"),
    route!("Right Output Mixer", "RINMIX Switch", "Right Boost Mixer"),
    route!("Left Output Mixer", "LDAC Switch", "Left DAC"),
    route!("Left Output Mixer", "RDAC Switch", "Right DAC"),
    route!("Left Output Mixer", "LAUX Switch", "LAUX"),
    route!("Left Output Mixer", "LINMIX Switch", "Left Boost Mixer"),
    /* AUX1 and AUX2 Mixer */
    route!("AUX1 Output Mixer", "RDAC Switch", "Right DAC"),
    route!("AUX1 Output Mixer", "RMIX Switch", "Right Output Mixer"),
    route!("AUX1 Output Mixer", "RINMIX Switch", "Right Boost Mixer"),
    route!("AUX1 Output Mixer", "LDAC Switch", "Left DAC"),
    route!("AUX1 Output Mixer", "LMIX Switch", "Left Output Mixer"),
    route!("AUX2 Output Mixer", "LDAC Switch", "Left DAC"),
    route!("AUX2 Output Mixer", "LMIX Switch", "Left Output Mixer"),
    route!("AUX2 Output Mixer", "LINMIX Switch", "Left Boost Mixer"),
    route!("AUX2 Output Mixer", "AUX1MIX Output Switch", "AUX1 Output Mixer"),
    /* Outputs */
    route!("Right Headphone Out", NULL, "Right Output Mixer"),
    route!("RHP", NULL, "Right Headphone Out"),
    route!("Left Headphone Out", NULL, "Left Output Mixer"),
    route!("LHP", NULL, "Left Headphone Out"),
    route!("Right Speaker Out", NULL, "Right Output Mixer"),
    route!("RSPK", NULL, "Right Speaker Out"),
    route!("Left Speaker Out", NULL, "Left Output Mixer"),
    route!("LSPK", NULL, "Left Speaker Out"),
    route!("AUX1 Out", NULL, "AUX1 Output Mixer"),
    route!("AUX2 Out", NULL, "AUX2 Output Mixer"),
    route!("AUXOUT1", NULL, "AUX1 Out"),
    route!("AUXOUT2", NULL, "AUX2 Out"),
    /* Boost Mixer */
    route!("Right ADC", NULL, "PLL", check_mclk_select_pll),
    route!("Left ADC", NULL, "PLL", check_mclk_select_pll),
    route!("Right ADC", NULL, "Right Boost Mixer"),
    route!("Right Boost Mixer", NULL, "RAUX"),
    route!("Right Boost Mixer", NULL, "Right Capture PGA"),
    route!("Right Boost Mixer", NULL, "R2"),
    route!("Left ADC", NULL, "Left Boost Mixer"),
    route!("Left Boost Mixer", NULL, "LAUX"),
    route!("Left Boost Mixer", NULL, "Left Capture PGA"),
    route!("Left Boost Mixer", NULL, "L2"),
    /* Input PGA */
    route!("Right Capture PGA", NULL, "Right Input Mixer"),
    route!("Left Capture PGA", NULL, "Left Input Mixer"),
    /* Enable Microphone Power */
    route!("Right Capture PGA", NULL, "Mic Bias"),
    route!("Left Capture PGA", NULL, "Mic Bias"),
    route!("Right Input Mixer", "R2 Switch", "R2"),
    route!("Right Input Mixer", "MicN Switch", "RMICN"),
    route!("Right Input Mixer", "MicP Switch", "RMICP"),
    route!("Left Input Mixer", "L2 Switch", "L2"),
    route!("Left Input Mixer", "MicN Switch", "LMICN"),
    route!("Left Input Mixer", "MicP Switch", "LMICP"),
    /* Digital Loopback */
    route!("Digital Loopback", "Switch", "Left ADC"),
    route!("Digital Loopback", "Switch", "Right ADC"),
    route!("Left DAC", NULL, "Digital Loopback"),
    route!("Right DAC", NULL, "Digital Loopback"),
];

unsafe extern "C" fn nau8822_calc_pll(pll_in: c_uint, fs: c_uint, pll_param: *mut nau8822_pll) -> c_int {
    let mut f2: u64;
    let mut f2_max: u64;
    let mut pll_ratio: u64;
    let mut scal_sel: c_int;

    if pll_in > NAU_PLL_REF_MAX || pll_in < NAU_PLL_REF_MIN {
        return -EINVAL;
    }
    f2_max = 0;
    scal_sel = NAU8822_MCLK_SCALER.len() as c_int;

    for i in 0..scal_sel {
        f2 = 256u64 * fs as u64 * 4 * NAU8822_MCLK_SCALER[i as usize] as u64 / 10;
        if f2 > NAU_PLL_FREQ_MIN as u64 && f2 < NAU_PLL_FREQ_MAX as u64 && f2_max < f2 {
            f2_max = f2;
            scal_sel = i;
        }
    }

    if NAU8822_MCLK_SCALER.len() as c_int == scal_sel {
        return -EINVAL;
    }
    (*pll_param).mclk_scaler = scal_sel;
    f2 = f2_max;

    /* Calculate the PLL 4-bit integer input and the PLL 24-bit fractional
     * input; round up the 24+4bit.
     */
    pll_ratio = (f2 << 28) / pll_in as u64;
    (*pll_param).pre_factor = 0;
    if ((pll_ratio >> 28) & 0xF) < NAU_PLL_OPTOP_MIN as u64 {
        pll_ratio <<= 1;
        (*pll_param).pre_factor = 1;
    }
    (*pll_param).pll_int = ((pll_ratio >> 28) & 0xF) as c_uint;
    (*pll_param).pll_frac = ((pll_ratio & 0xFFFFFFF) >> 4) as c_uint;

    0
}

unsafe extern "C" fn nau8822_config_clkdiv(dai: *mut snd_soc_dai, mut div: c_int, rate: c_int) -> c_int {
    let component = (*dai).component;
    let nau8822 = snd_soc_component_get_drvdata(component) as *mut nau8822;
    let pll = &mut (*nau8822).pll as *mut nau8822_pll;
    let mut i: c_int;
    let mut sclk: c_int;
    let imclk: c_int;

    if (*nau8822).div_id == NAU8822_CLK_MCLK {
        /* Configure the master clock prescaler div to make system
         * clock to approximate the internal master clock (IMCLK);
         * and large or equal to IMCLK.
         */
        div = 0;
        imclk = rate * 256;
        i = 1;
        while i < NAU8822_MCLK_SCALER.len() as c_int {
            sclk = ((*nau8822).sysclk as c_int * 10) / NAU8822_MCLK_SCALER[i as usize];
            if sclk < imclk {
                break;
            }
            div = i;
            i += 1;
        }
        dev_dbg((*component).dev, b"master clock prescaler %x for fs %d\n\0".as_ptr() as *const c_char, div, rate);

        /* master clock from MCLK and disable PLL */
        snd_soc_component_update_bits(component, NAU8822_REG_CLOCKING, NAU8822_MCLKSEL_MASK, (div as c_uint) << NAU8822_MCLKSEL_SFT);
        snd_soc_component_update_bits(component, NAU8822_REG_CLOCKING, NAU8822_CLKM_MASK, NAU8822_CLKM_MCLK);
    } else if (*nau8822).div_id == NAU8822_CLK_PLL {
        /* master clock from PLL and enable PLL */
        if (*pll).mclk_scaler != div {
            dev_err((*component).dev, b"master clock prescaler not meet PLL parameters\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        snd_soc_component_update_bits(component, NAU8822_REG_CLOCKING, NAU8822_MCLKSEL_MASK, (div as c_uint) << NAU8822_MCLKSEL_SFT);
        snd_soc_component_update_bits(component, NAU8822_REG_CLOCKING, NAU8822_CLKM_MASK, NAU8822_CLKM_PLL);
    } else {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn nau8822_set_pll(dai: *mut snd_soc_dai, _pll_id: c_int, _source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int {
    let component = (*dai).component;
    let nau8822 = snd_soc_component_get_drvdata(component) as *mut nau8822;
    let pll_param = &mut (*nau8822).pll as *mut nau8822_pll;
    let fs: c_int;

    if freq_in == (*pll_param).freq_in && freq_out == (*pll_param).freq_out {
        return 0;
    }

    if freq_out == 0 {
        dev_dbg((*component).dev, b"PLL disabled\n\0".as_ptr() as *const c_char);
        snd_soc_component_update_bits(component, NAU8822_REG_POWER_MANAGEMENT_1, NAU8822_PLL_EN_MASK, NAU8822_PLL_OFF);
        return 0;
    }

    fs = (freq_out / 256) as c_int;

    let ret = nau8822_calc_pll(freq_in, fs as c_uint, pll_param);
    if ret < 0 {
        dev_err((*component).dev, b"Unsupported input clock %d\n\0".as_ptr() as *const c_char, freq_in);
        return ret;
    }

    dev_dbg((*component).dev, b"pll_int=%x pll_frac=%x mclk_scaler=%x pre_factor=%x\n\0".as_ptr() as *const c_char, (*pll_param).pll_int, (*pll_param).pll_frac, (*pll_param).mclk_scaler, (*pll_param).pre_factor);

    snd_soc_component_update_bits(component, NAU8822_REG_POWER_MANAGEMENT_1, NAU8822_PLL_EN_MASK, NAU8822_PLL_OFF);
    snd_soc_component_update_bits(component, NAU8822_REG_PLL_N, NAU8822_PLLMCLK_DIV2 | NAU8822_PLLN_MASK, (if (*pll_param).pre_factor != 0 { NAU8822_PLLMCLK_DIV2 } else { 0 }) | (*pll_param).pll_int);
    snd_soc_component_write(component, NAU8822_REG_PLL_K1, ((*pll_param).pll_frac >> NAU8822_PLLK1_SFT) & NAU8822_PLLK1_MASK);
    snd_soc_component_write(component, NAU8822_REG_PLL_K2, ((*pll_param).pll_frac >> NAU8822_PLLK2_SFT) & NAU8822_PLLK2_MASK);
    snd_soc_component_write(component, NAU8822_REG_PLL_K3, (*pll_param).pll_frac & NAU8822_PLLK3_MASK);
    snd_soc_component_update_bits(component, NAU8822_REG_CLOCKING, NAU8822_MCLKSEL_MASK, ((*pll_param).mclk_scaler as c_uint) << NAU8822_MCLKSEL_SFT);
    snd_soc_component_update_bits(component, NAU8822_REG_CLOCKING, NAU8822_CLKM_MASK, NAU8822_CLKM_PLL);
    snd_soc_component_update_bits(component, NAU8822_REG_POWER_MANAGEMENT_1, NAU8822_PLL_EN_MASK, NAU8822_PLL_ON);

    (*pll_param).freq_in = freq_in;
    (*pll_param).freq_out = freq_out;

    0
}

unsafe extern "C" fn nau8822_set_dai_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let nau8822 = snd_soc_component_get_drvdata(component) as *mut nau8822;
    let mclk_freq: c_ulong;

    (*nau8822).div_id = clk_id;
    (*nau8822).sysclk = freq;

    if !(*nau8822).mclk.is_null() {
        mclk_freq = clk_get_rate((*nau8822).mclk);
        if mclk_freq != freq as c_ulong {
            let ret = nau8822_set_pll(dai, NAU8822_CLK_MCLK, NAU8822_CLK_MCLK, mclk_freq as c_uint, freq);
            if ret != 0 {
                dev_err((*component).dev, b"Failed to set PLL\n\0".as_ptr() as *const c_char);
                return ret;
            }
            (*nau8822).div_id = NAU8822_CLK_PLL;
        }
    }

    dev_dbg((*component).dev, b"master sysclk %dHz, source %s\n\0".as_ptr() as *const c_char, freq, if (*nau8822).div_id == NAU8822_CLK_PLL { b"PLL\0".as_ptr() } else { b"MCLK\0".as_ptr() });

    0
}

unsafe extern "C" fn nau8822_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let mut ctrl1_val: u16 = 0;
    let mut ctrl2_val: u16 = 0;

    dev_dbg((*component).dev, b"%s\n\0".as_ptr() as *const c_char, b"nau8822_set_dai_fmt\0".as_ptr());

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => ctrl2_val |= 1,
        x if x == SND_SOC_DAIFMT_CBC_CFC => ctrl2_val &= !1,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => ctrl1_val |= 0x10,
        x if x == SND_SOC_DAIFMT_RIGHT_J => {}
        x if x == SND_SOC_DAIFMT_LEFT_J => ctrl1_val |= 0x8,
        x if x == SND_SOC_DAIFMT_DSP_A => ctrl1_val |= 0x18,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_IB_IF => ctrl1_val |= 0x180,
        x if x == SND_SOC_DAIFMT_IB_NF => ctrl1_val |= 0x100,
        x if x == SND_SOC_DAIFMT_NB_IF => ctrl1_val |= 0x80,
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, NAU8822_REG_AUDIO_INTERFACE, NAU8822_AIFMT_MASK | NAU8822_LRP_MASK | NAU8822_BCLKP_MASK, ctrl1_val as c_uint);
    snd_soc_component_update_bits(component, NAU8822_REG_CLOCKING, NAU8822_CLKIOEN_MASK, ctrl2_val as c_uint);

    0
}

unsafe extern "C" fn nau8822_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let nau8822 = snd_soc_component_get_drvdata(component) as *mut nau8822;
    let mut div: c_int = 0;
    let mut val_len: c_int = 0;
    let mut val_rate: c_int = 0;
    let ctrl_val: c_uint;
    let bclk_fs: c_uint;
    let bclk_div: c_uint;

    /* make BCLK and LRC divide configuration if the codec as master. */
    ctrl_val = snd_soc_component_read(component, NAU8822_REG_CLOCKING);
    if (ctrl_val & NAU8822_CLK_MASTER) != 0 {
        /* get the bclk and fs ratio */
        bclk_fs = snd_soc_params_to_bclk(params) / params_rate(params);
        if bclk_fs <= 32 {
            bclk_div = NAU8822_BCLKDIV_8;
        } else if bclk_fs <= 64 {
            bclk_div = NAU8822_BCLKDIV_4;
        } else if bclk_fs <= 128 {
            bclk_div = NAU8822_BCLKDIV_2;
        } else {
            return -EINVAL;
        }
        snd_soc_component_update_bits(component, NAU8822_REG_CLOCKING, NAU8822_BCLKSEL_MASK, bclk_div);
    }

    match params_format(params) {
        x if x == SNDRV_PCM_FORMAT_S16_LE => {}
        x if x == SNDRV_PCM_FORMAT_S20_3LE => val_len |= NAU8822_WLEN_20,
        x if x == SNDRV_PCM_FORMAT_S24_LE => val_len |= NAU8822_WLEN_24,
        x if x == SNDRV_PCM_FORMAT_S32_LE => val_len |= NAU8822_WLEN_32,
        _ => return -EINVAL,
    }

    match params_rate(params) {
        8000 => val_rate |= NAU8822_SMPLR_8K,
        11025 => val_rate |= NAU8822_SMPLR_12K,
        16000 => val_rate |= NAU8822_SMPLR_16K,
        22050 => val_rate |= NAU8822_SMPLR_24K,
        32000 => val_rate |= NAU8822_SMPLR_32K,
        44100 | 48000 => {}
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, NAU8822_REG_AUDIO_INTERFACE, NAU8822_WLEN_MASK, val_len as c_uint);
    snd_soc_component_update_bits(component, NAU8822_REG_ADDITIONAL_CONTROL, NAU8822_SMPLR_MASK, val_rate as c_uint);

    /* If the master clock is from MCLK, provide the runtime FS for driver
     * to get the master clock prescaler configuration.
     */
    if (*nau8822).div_id != NAU8822_CLK_MCLK {
        div = (*nau8822).pll.mclk_scaler;
    }

    nau8822_config_clkdiv(dai, div, params_rate(params) as c_int);

    0
}

unsafe extern "C" fn nau8822_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;

    dev_dbg((*component).dev, b"%s: %d\n\0".as_ptr() as *const c_char, b"nau8822_mute\0".as_ptr(), mute);

    if mute != 0 {
        snd_soc_component_update_bits(component, NAU8822_REG_DAC_CONTROL, 0x40, 0x40);
    } else {
        snd_soc_component_update_bits(component, NAU8822_REG_DAC_CONTROL, 0x40, 0);
    }

    0
}

unsafe extern "C" fn nau8822_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let nau8822 = snd_soc_component_get_drvdata(component) as *mut nau8822;
    let dapm = snd_soc_component_to_dapm(component);

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            if !(*nau8822).mclk.is_null() && snd_soc_dapm_get_bias_level(dapm) != snd_soc_bias_level::SND_SOC_BIAS_ON {
                let ret = clk_prepare_enable((*nau8822).mclk);
                if ret != 0 {
                    dev_err((*component).dev, b"Failed to enable MCLK: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
            }
            snd_soc_component_update_bits(component, NAU8822_REG_POWER_MANAGEMENT_1, NAU8822_REFIMP_MASK, NAU8822_REFIMP_80K);
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if !(*nau8822).mclk.is_null() && snd_soc_dapm_get_bias_level(dapm) != snd_soc_bias_level::SND_SOC_BIAS_OFF {
                clk_disable_unprepare((*nau8822).mclk);
            }
            snd_soc_component_update_bits(component, NAU8822_REG_POWER_MANAGEMENT_1, NAU8822_IOBUF_EN | NAU8822_ABIAS_EN, NAU8822_IOBUF_EN | NAU8822_ABIAS_EN);
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_OFF {
                snd_soc_component_update_bits(component, NAU8822_REG_POWER_MANAGEMENT_1, NAU8822_REFIMP_MASK, NAU8822_REFIMP_3K);
                mdelay(100);
            }
            snd_soc_component_update_bits(component, NAU8822_REG_POWER_MANAGEMENT_1, NAU8822_REFIMP_MASK, NAU8822_REFIMP_300K);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            snd_soc_component_write(component, NAU8822_REG_POWER_MANAGEMENT_1, 0);
            snd_soc_component_write(component, NAU8822_REG_POWER_MANAGEMENT_2, 0);
            snd_soc_component_write(component, NAU8822_REG_POWER_MANAGEMENT_3, 0);
        }
    }

    dev_dbg((*component).dev, b"%s: %d\n\0".as_ptr() as *const c_char, b"nau8822_set_bias_level\0".as_ptr(), level as c_int);

    0
}

unsafe fn nau8822_rates() -> c_uint {
    SNDRV_PCM_RATE_8000_48000
}

unsafe fn nau8822_formats() -> c_uint {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

static NAU8822_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(nau8822_hw_params),
    mute_stream: Some(nau8822_mute),
    set_fmt: Some(nau8822_set_dai_fmt),
    set_sysclk: Some(nau8822_set_dai_sysclk),
    set_pll: Some(nau8822_set_pll),
    no_capture_mute: 1,
};

static mut NAU8822_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"nau8822-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { nau8822_rates() },
        formats: unsafe { nau8822_formats() },
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { nau8822_rates() },
        formats: unsafe { nau8822_formats() },
    },
    ops: &NAU8822_DAI_OPS,
    symmetric_rate: 1,
};

unsafe extern "C" fn nau8822_suspend(component: *mut snd_soc_component) -> c_int {
    let nau8822 = snd_soc_component_get_drvdata(component) as *mut nau8822;
    let dapm = snd_soc_component_to_dapm(component);

    snd_soc_dapm_force_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_OFF);
    regulator_bulk_disable(NAU8822_NUM_SUPPLIES, (*nau8822).supplies.as_mut_ptr());
    regcache_mark_dirty((*nau8822).regmap);

    0
}

unsafe extern "C" fn nau8822_resume(component: *mut snd_soc_component) -> c_int {
    let nau8822 = snd_soc_component_get_drvdata(component) as *mut nau8822;
    let dapm = snd_soc_component_to_dapm(component);
    let ret = regulator_bulk_enable(NAU8822_NUM_SUPPLIES, (*nau8822).supplies.as_mut_ptr());

    if ret != 0 {
        dev_err((*component).dev, b"Failed to enable regulators: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    fsleep(100);
    regcache_sync((*nau8822).regmap);
    snd_soc_dapm_force_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_STANDBY);

    0
}

/*
 * These registers contain an "update" bit - bit 8. This means, for example,
 * that one can write new DAC digital volume for both channels, but only when
 * the update bit is set, will also the volume be updated - simultaneously for
 * both channels.
 */
static UPDATE_REG: &[c_uint] = &[
    unsafe { NAU8822_REG_LEFT_DAC_DIGITAL_VOLUME },
    unsafe { NAU8822_REG_RIGHT_DAC_DIGITAL_VOLUME },
    unsafe { NAU8822_REG_LEFT_ADC_DIGITAL_VOLUME },
    unsafe { NAU8822_REG_RIGHT_ADC_DIGITAL_VOLUME },
    unsafe { NAU8822_REG_LEFT_INP_PGA_CONTROL },
    unsafe { NAU8822_REG_RIGHT_INP_PGA_CONTROL },
    unsafe { NAU8822_REG_LHP_VOLUME },
    unsafe { NAU8822_REG_RHP_VOLUME },
    unsafe { NAU8822_REG_LSPKOUT_VOLUME },
    unsafe { NAU8822_REG_RSPKOUT_VOLUME },
];

unsafe extern "C" fn nau8822_probe(component: *mut snd_soc_component) -> c_int {
    let of_node = (*(*component).dev).of_node;

    /*
     * Set the update bit in all registers, that have one. This way all
     * writes to those registers will also cause the update bit to be
     * written.
     */
    for i in 0..UPDATE_REG.len() {
        snd_soc_component_update_bits(component, UPDATE_REG[i], 0x100, 0x100);
    }

    /* Check property to configure the two loudspeaker outputs as
     * a single Bridge Tied Load output
     */
    if of_property_read_bool(of_node, b"nuvoton,spk-btl\0".as_ptr() as *const c_char) {
        snd_soc_component_update_bits(component, NAU8822_REG_RIGHT_SPEAKER_CONTROL, NAU8822_RSUBBYP, NAU8822_RSUBBYP);
    }

    0
}

static SOC_COMPONENT_DEV_NAU8822: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(nau8822_probe),
    suspend: Some(nau8822_suspend),
    resume: Some(nau8822_resume),
    set_bias_level: Some(nau8822_set_bias_level),
    controls: NAU8822_SND_CONTROLS.as_ptr(),
    num_controls: NAU8822_SND_CONTROLS.len() as c_uint,
    dapm_widgets: NAU8822_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: NAU8822_DAPM_WIDGETS.len() as c_uint,
    dapm_routes: NAU8822_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: NAU8822_DAPM_ROUTES.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static NAU8822_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: unsafe { NAU8822_REG_MAX_REGISTER },
    volatile_reg: Some(nau8822_volatile),
    readable_reg: Some(nau8822_readable_reg),
    writeable_reg: Some(nau8822_writeable_reg),
    cache_type: REGCACHE_RBTREE,
    reg_defaults: NAU8822_REG_DEFAULTS.as_ptr(),
    num_reg_defaults: NAU8822_REG_DEFAULTS.len() as c_uint,
};

unsafe extern "C" fn nau8822_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let mut nau8822_ptr = dev_get_platdata(dev) as *mut nau8822;
    let mut ret: c_int;

    if nau8822_ptr.is_null() {
        nau8822_ptr = devm_kzalloc(dev, size_of::<nau8822>(), GFP_KERNEL) as *mut nau8822;
        if nau8822_ptr.is_null() {
            return -ENOMEM;
        }
    }
    i2c_set_clientdata(i2c, nau8822_ptr as *mut c_void);

    (*nau8822_ptr).mclk = devm_clk_get_optional(&mut (*i2c).dev, b"mclk\0".as_ptr() as *const c_char);
    if IS_ERR((*nau8822_ptr).mclk as *const c_void) {
        return dev_err_probe(&mut (*i2c).dev, PTR_ERR((*nau8822_ptr).mclk as *const c_void), b"Error getting mclk\n\0".as_ptr() as *const c_char);
    }

    for i in 0..NAU8822_NUM_SUPPLIES as usize {
        (*nau8822_ptr).supplies[i].supply = NAU8822_SUPPLY_NAMES[i];
    }

    ret = devm_regulator_bulk_get(dev, NAU8822_NUM_SUPPLIES, (*nau8822_ptr).supplies.as_mut_ptr());
    if ret != 0 {
        return dev_err_probe(dev, ret, b"Failed to get regulators\n\0".as_ptr() as *const c_char);
    }

    (*nau8822_ptr).regmap = devm_regmap_init_i2c(i2c, &NAU8822_REGMAP_CONFIG);
    if IS_ERR((*nau8822_ptr).regmap as *const c_void) {
        ret = PTR_ERR((*nau8822_ptr).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, b"Failed to allocate regmap: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    (*nau8822_ptr).dev = dev;

    ret = regulator_bulk_enable(NAU8822_NUM_SUPPLIES, (*nau8822_ptr).supplies.as_mut_ptr());
    if ret != 0 {
        return dev_err_probe(dev, ret, b"Failed to enable regulators\n\0".as_ptr() as *const c_char);
    }

    fsleep(100);

    /* Reset the codec */
    ret = regmap_write((*nau8822_ptr).regmap, NAU8822_REG_RESET, 0x00);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"Failed to issue reset: %d\n\0".as_ptr() as *const c_char, ret);
        regulator_bulk_disable(NAU8822_NUM_SUPPLIES, (*nau8822_ptr).supplies.as_mut_ptr());
        return ret;
    }

    ret = devm_snd_soc_register_component(dev, &SOC_COMPONENT_DEV_NAU8822, &mut NAU8822_DAI, 1);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"Failed to register CODEC: %d\n\0".as_ptr() as *const c_char, ret);
        regulator_bulk_disable(NAU8822_NUM_SUPPLIES, (*nau8822_ptr).supplies.as_mut_ptr());
        return ret;
    }

    0
}

unsafe extern "C" fn nau8822_i2c_remove(i2c: *mut i2c_client) {
    let nau8822 = i2c_get_clientdata(i2c) as *mut nau8822;
    regulator_bulk_disable(NAU8822_NUM_SUPPLIES, (*nau8822).supplies.as_mut_ptr());
}

static NAU8822_I2C_ID: &[i2c_device_id] = &[
    i2c_device_id { name: [b'n' as c_char, b'a' as c_char, b'u' as c_char, b'8' as c_char, b'8' as c_char, b'2' as c_char, b'2' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    i2c_device_id { name: [0; 20] },
];
MODULE_DEVICE_TABLE!(i2c, NAU8822_I2C_ID);

// CONFIG_OF: Open Firmware device match table.
static NAU8822_OF_MATCH: &[of_device_id] = &[
    of_device_id { compatible: b"nuvoton,nau8822\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
MODULE_DEVICE_TABLE!(of, NAU8822_OF_MATCH);

static mut NAU8822_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: driver_inner {
        name: b"nau8822\0".as_ptr() as *const c_char,
        of_match_table: unsafe { of_match_ptr(NAU8822_OF_MATCH.as_ptr()) },
    },
    probe: Some(nau8822_i2c_probe),
    remove: Some(nau8822_i2c_remove),
    id_table: NAU8822_I2C_ID.as_ptr(),
};
module_i2c_driver!(NAU8822_I2C_DRIVER);

MODULE_DESCRIPTION!("ASoC NAU8822 codec driver");
MODULE_AUTHOR!("David Lin <ctlin0@nuvoton.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
