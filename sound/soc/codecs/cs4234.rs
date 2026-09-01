// SPDX-License-Identifier: GPL-2.0-only
// cs4234.c -- ALSA SoC CS4234 driver
//
// Copyright (C) 2020 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}
#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}
#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    _private: [u8; 0],
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
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut c_void,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_rule {
    pub private: *mut c_void,
    pub var: c_int,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
    pub consumer: *mut regulator,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ratnum {
    pub num: c_uint,
    pub den_min: c_uint,
    pub den_max: c_uint,
    pub den_step: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_ratnums {
    pub nrats: c_uint,
    pub rats: *mut snd_ratnum,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
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
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
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
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub idle_bias_on: c_uint,
    pub suspend_bias_off: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
}
#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

pub type snd_soc_bias_level = c_uint;

#[repr(C)]
pub struct cs4234 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub core_supplies: [regulator_bulk_data; 2],
    pub num_core_supplies: c_int,
    pub vq_ramp_complete: completion,
    pub vq_ramp_delay: delayed_work,
    pub mclk: *mut clk,
    pub mclk_rate: c_ulong,
    pub lrclk_rate: c_ulong,
    pub format: c_uint,
    pub rate_dividers: [snd_ratnum; 2],
    pub rate_constraint: snd_pcm_hw_constraint_ratnums,
}

unsafe extern "C" {
    static mut system_power_efficient_wq: *mut c_void;

    fn snd_kcontrol_chip(kctrl: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_put_enum_double(kctrl: *mut snd_kcontrol, uctrl: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_enum_double(kctrl: *mut snd_kcontrol, uctrl: *mut snd_ctl_elem_value) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_bulk_write(map: *mut regmap, reg: c_uint, val: *const u8, val_count: usize) -> c_int;
    fn regmap_bulk_read(map: *mut regmap, reg: c_uint, val: *mut u8, val_count: usize) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn complete_all(x: *mut completion);
    fn wait_for_completion(x: *mut completion);
    fn reinit_completion(x: *mut completion);
    fn init_completion(x: *mut completion);
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn queue_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: c_ulong) -> bool;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_get_voltage(regulator: *mut regulator) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_int,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_hw_constraint_mask64(runtime: *mut c_void, var: c_int, mask: u64) -> c_int;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut c_void, var: c_int, min: c_uint, max: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_ratnums(
        runtime: *mut c_void,
        cond: c_uint,
        var: c_int,
        r: *mut snd_pcm_hw_constraint_ratnums,
    ) -> c_int;
    fn snd_pcm_hw_rule_add(
        runtime: *mut c_void,
        cond: c_uint,
        var: c_int,
        func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
        private: *mut c_void,
        dep: c_int,
    ) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut c_void;
    fn snd_interval_ranges(i: *mut c_void, count: c_uint, ranges: *mut snd_interval, mask: c_uint) -> c_int;
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

extern "Rust" {
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn ffs(x: c_int) -> c_int;
    fn pm_ptr(ptr: *const dev_pm_ops) -> *const dev_pm_ops;
}

const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_NOPM: c_uint = 0;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 2;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 1;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 1;
const SND_SOC_DAIFMT_I2S: c_uint = 2;
const SND_SOC_DAIFMT_DSP_A: c_uint = 3;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0x0000;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x0010;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0100;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 2;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << 1;

// Constants from cs4234.h and Linux/ASoC helper macros are expected dependencies.
extern "Rust" {
    static CS4234_TPS_CTRL: c_uint;
    static CS4234_GRP_DELAY_SHIFT: c_uint;
    static CS4234_LOW_LAT_CTRL1: c_uint;
    static CS4234_LL_NG_SHIFT: c_uint;
    static CS4234_DAC_CTRL1: c_uint;
    static CS4234_DAC14_NG_SHIFT: c_uint;
    static CS4234_DAC_CTRL2: c_uint;
    static CS4234_DAC5_NG_SHIFT: c_uint;
    static CS4234_DAC5_CFG_FLTR_SHIFT: c_uint;
    static CS4234_VOLUME_MODE: c_uint;
    static CS4234_MUTE_DELAY_SHIFT: c_uint;
    static CS4234_MIN_DELAY_SHIFT: c_uint;
    static CS4234_MAX_DELAY_SHIFT: c_uint;
    static CS4234_ADC_CTRL2: c_uint;
    static CS4234_DAC_CTRL4: c_uint;
    static CS4234_DAC_CTRL3: c_uint;
    static CS4234_DAC5_ATT_SHIFT: c_uint;
    static CS4234_DAC14_ATT_SHIFT: c_uint;
    static CS4234_ADC_CTRL1: c_uint;
    static CS4234_ENA_HPF_SHIFT: c_uint;
    static CS4234_INV_ADC1_SHIFT: c_uint;
    static CS4234_INV_ADC2_SHIFT: c_uint;
    static CS4234_INV_ADC3_SHIFT: c_uint;
    static CS4234_INV_ADC4_SHIFT: c_uint;
    static CS4234_INV_DAC1_SHIFT: c_uint;
    static CS4234_INV_DAC2_SHIFT: c_uint;
    static CS4234_INV_DAC3_SHIFT: c_uint;
    static CS4234_INV_DAC4_SHIFT: c_uint;
    static CS4234_INV_DAC5_SHIFT: c_uint;
    static CS4234_MUTE_ADC1_SHIFT: c_uint;
    static CS4234_MUTE_ADC2_SHIFT: c_uint;
    static CS4234_MUTE_ADC3_SHIFT: c_uint;
    static CS4234_MUTE_ADC4_SHIFT: c_uint;
    static CS4234_MUTE_DAC1_SHIFT: c_uint;
    static CS4234_MUTE_DAC2_SHIFT: c_uint;
    static CS4234_MUTE_DAC3_SHIFT: c_uint;
    static CS4234_MUTE_DAC4_SHIFT: c_uint;
    static CS4234_MUTE_DAC5_SHIFT: c_uint;
    static CS4234_MUTE_LL_SHIFT: c_uint;
    static CS4234_INV_LL1_SHIFT: c_uint;
    static CS4234_INV_LL2_SHIFT: c_uint;
    static CS4234_INV_LL3_SHIFT: c_uint;
    static CS4234_INV_LL4_SHIFT: c_uint;
    static CS4234_DAC14_DE_SHIFT: c_uint;
    static CS4234_DAC5_DE_SHIFT: c_uint;
    static CS4234_DAC5_MVC_SHIFT: c_uint;
    static CS4234_SP_CTRL: c_uint;
    static CS4234_LEFT_J: c_uint;
    static CS4234_I2S: c_uint;
    static CS4234_TDM: c_uint;
    static CS4234_SP_FORMAT_SHIFT: c_uint;
    static CS4234_MST_SLV_MASK: c_uint;
    static CS4234_INVT_SCLK_MASK: c_uint;
    static CS4234_SP_FORMAT_MASK: c_uint;
    static CS4234_CLOCK_SP: c_uint;
    static CS4234_SPEED_MODE_MASK: c_uint;
    static CS4234_SPEED_MODE_SHIFT: c_uint;
    static CS4234_MCLK_RATE_MASK: c_uint;
    static CS4234_MCLK_RATE_SHIFT: c_uint;
    static CS4234_48K: c_uint;
    static CS4234_44K1: c_uint;
    static CS4234_32K: c_uint;
    static CS4234_BASE_RATE_MASK: c_uint;
    static CS4234_BASE_RATE_SHIFT: c_uint;
    static CS4234_SAMPLE_WIDTH: c_uint;
    static CS4234_SDOUTX_SW_MASK: c_uint;
    static CS4234_SDOUTX_SW_SHIFT: c_uint;
    static CS4234_INPUT_SW_MASK: c_uint;
    static CS4234_LOW_LAT_SW_MASK: c_uint;
    static CS4234_DAC5_SW_MASK: c_uint;
    static CS4234_INPUT_SW_SHIFT: c_uint;
    static CS4234_LOW_LAT_SW_SHIFT: c_uint;
    static CS4234_DAC5_SW_SHIFT: c_uint;
    static CS4234_SP_DATA_SEL: c_uint;
    static CS4234_DAC14_SRC_MASK: c_uint;
    static CS4234_DAC14_SRC_SHIFT: c_uint;
    static CS4234_LL_SRC_MASK: c_uint;
    static CS4234_LL_SRC_SHIFT: c_uint;
    static CS4234_SDIN1_MASK1: c_uint;
    static CS4234_SDIN1_MASK2: c_uint;
    static CS4234_SDIN2_MASK1: c_uint;
    static CS4234_SDIN2_MASK2: c_uint;
    static CS4234_DEVID_AB: c_uint;
    static CS4234_DEVID_EF: c_uint;
    static CS4234_REVID: c_uint;
    static CS4234_DAC5_VOL: c_uint;
    static CS4234_INT_CTRL: c_uint;
    static CS4234_MAX_REGISTER: c_uint;
    static CS4234_INT_NOTIFY1: c_uint;
    static CS4234_INT_NOTIFY2: c_uint;
    static CS4234_MASTER_VOL: c_uint;
    static CS4234_DAC1_VOL: c_uint;
    static CS4234_DAC2_VOL: c_uint;
    static CS4234_DAC3_VOL: c_uint;
    static CS4234_DAC4_VOL: c_uint;
    static CS4234_INT_MASK1: c_uint;
    static CS4234_INT_MASK2: c_uint;
    static CS4234_VQ_RAMP_MASK: c_uint;
    static CS4234_HOLD_RESET_TIME_US: c_ulong;
    static CS4234_BOOT_TIME_US: c_ulong;
    static CS4234_VQ_CHARGE_MS: c_uint;
    static CS4234_SUPPORTED_ID: u32;
    static CS4234_SUPPLY_VA: usize;
    static CS4234_VA_SEL_MASK: c_uint;
    static CS4234_3V3: c_uint;
    static CS4234_5V: c_uint;
    static CS4234_VA_SEL_SHIFT: c_uint;
    static CS4234_PCM_RATES: c_uint;
    static CS4234_FORMATS: u64;
    static CS4234_PDN_DAC1_SHIFT: c_uint;
    static CS4234_PDN_DAC2_SHIFT: c_uint;
    static CS4234_PDN_DAC3_SHIFT: c_uint;
    static CS4234_PDN_DAC4_SHIFT: c_uint;
    static CS4234_PDN_DAC5_SHIFT: c_uint;
    static CS4234_PDN_ADC1_SHIFT: c_uint;
    static CS4234_PDN_ADC2_SHIFT: c_uint;
    static CS4234_PDN_ADC3_SHIFT: c_uint;
    static CS4234_PDN_ADC4_SHIFT: c_uint;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
macro_rules! ARRAY_SIZE {
    ($a:expr) => {
        ($a.len() as c_uint)
    };
}
macro_rules! BIT {
    ($n:expr) => {
        (1u32 << ($n))
    };
}
macro_rules! clamp {
    ($val:expr, $min:expr, $max:expr) => {{
        let v = $val;
        if v < $min {
            $min
        } else if v > $max {
            $max
        } else {
            v
        }
    }};
}

/* -89.92dB to +6.02dB with step of 0.38dB */
static dac_tlv: [c_uint; 4] = [0, (-8992i32) as c_uint, 38, 0];

static cs4234_dac14_delay_text: [*const c_char; 16] = [
    cstr!("0us"), cstr!("100us"), cstr!("150us"), cstr!("200us"), cstr!("225us"), cstr!("250us"),
    cstr!("275us"), cstr!("300us"), cstr!("325us"), cstr!("350us"), cstr!("375us"), cstr!("400us"),
    cstr!("425us"), cstr!("450us"), cstr!("475us"), cstr!("500us"),
];
// SOC_ENUM_SINGLE_DECL(cs4234_dac14_group_delay, CS4234_TPS_CTRL,
//                      CS4234_GRP_DELAY_SHIFT, cs4234_dac14_delay_text);

static cs4234_noise_gate_text: [*const c_char; 8] = [
    cstr!("72dB"), cstr!("78dB"), cstr!("84dB"), cstr!("90dB"), cstr!("96dB"), cstr!("102dB"),
    cstr!("138dB"), cstr!("Disabled"),
];
// SOC_ENUM_SINGLE_DECL(cs4234_ll_noise_gate, CS4234_LOW_LAT_CTRL1,
//                      CS4234_LL_NG_SHIFT, cs4234_noise_gate_text);
// SOC_ENUM_SINGLE_DECL(cs4234_dac14_noise_gate, CS4234_DAC_CTRL1,
//                      CS4234_DAC14_NG_SHIFT, cs4234_noise_gate_text);
// SOC_ENUM_SINGLE_DECL(cs4234_dac5_noise_gate, CS4234_DAC_CTRL2,
//                      CS4234_DAC5_NG_SHIFT, cs4234_noise_gate_text);

static cs4234_dac5_config_fltr_sel_text: [*const c_char; 2] = [
    cstr!("Interpolation Filter"), cstr!("Sample and Hold"),
];
// SOC_ENUM_SINGLE_DECL(cs4234_dac5_config_fltr_sel, CS4234_DAC_CTRL1,
//                      CS4234_DAC5_CFG_FLTR_SHIFT,
//                      cs4234_dac5_config_fltr_sel_text);

static cs4234_mute_delay_text: [*const c_char; 4] = [
    cstr!("1x"), cstr!("4x"), cstr!("16x"), cstr!("64x"),
];
// SOC_ENUM_SINGLE_DECL(cs4234_mute_delay, CS4234_VOLUME_MODE,
//                      CS4234_MUTE_DELAY_SHIFT, cs4234_mute_delay_text);

static cs4234_minmax_delay_text: [*const c_char; 8] = [
    cstr!("1x"), cstr!("2x"), cstr!("4x"), cstr!("8x"), cstr!("16x"), cstr!("32x"),
    cstr!("64x"), cstr!("128x"),
];
// SOC_ENUM_SINGLE_DECL(cs4234_min_delay, CS4234_VOLUME_MODE,
//                      CS4234_MIN_DELAY_SHIFT, cs4234_minmax_delay_text);
// SOC_ENUM_SINGLE_DECL(cs4234_max_delay, CS4234_VOLUME_MODE,
//                      CS4234_MAX_DELAY_SHIFT, cs4234_minmax_delay_text);

unsafe extern "C" fn cs4234_dac14_grp_delay_put(
    kctrl: *mut snd_kcontrol,
    uctrl: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kctrl);
    let cs4234 = snd_soc_component_get_drvdata(component) as *mut cs4234;
    let dapm = snd_soc_component_to_dapm(component);
    let mut val: c_uint = 0;
    let mut ret: c_int = 0;

    snd_soc_dapm_mutex_lock(dapm);

    regmap_read((*cs4234).regmap, CS4234_ADC_CTRL2, &mut val);
    if (val & 0x0F) != 0x0F {
        // are all the ADCs powerdown
        ret = -EBUSY;
        dev_err((*component).dev, cstr!("Can't change group delay while ADC are ON\n"));
        snd_soc_dapm_mutex_unlock(dapm);
        return ret;
    }

    regmap_read((*cs4234).regmap, CS4234_DAC_CTRL4, &mut val);
    if (val & 0x1F) != 0x1F {
        // are all the DACs powerdown
        ret = -EBUSY;
        dev_err((*component).dev, cstr!("Can't change group delay while DAC are ON\n"));
        snd_soc_dapm_mutex_unlock(dapm);
        return ret;
    }

    ret = snd_soc_put_enum_double(kctrl, uctrl);
    snd_soc_dapm_mutex_unlock(dapm);

    ret
}

unsafe extern "C" fn cs4234_vq_ramp_done(work: *mut work_struct) {
    let dw = work as *mut delayed_work;
    let cs4234 = (dw as *mut u8).sub(core::mem::offset_of!(cs4234, vq_ramp_delay)) as *mut cs4234;

    complete_all(&mut (*cs4234).vq_ramp_complete);
}

unsafe extern "C" fn cs4234_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let cs4234 = snd_soc_component_get_drvdata(component) as *mut cs4234;
    let dapm = snd_soc_component_to_dapm(component);

    match level {
        SND_SOC_BIAS_PREPARE => match snd_soc_dapm_get_bias_level(dapm) {
            SND_SOC_BIAS_STANDBY => {
                wait_for_completion(&mut (*cs4234).vq_ramp_complete);
            }
            _ => {}
        },
        _ => {}
    }

    0
}

// SND_SOC_DAPM_* and SOC_* macro initializers are preserved as dependency intent.
static cs4234_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static cs4234_dapm_routes: [snd_soc_dapm_route; 28] = [
    /* Playback */
    snd_soc_dapm_route { sink: cstr!("AOUT1"), control: core::ptr::null(), source: cstr!("DAC1") },
    snd_soc_dapm_route { sink: cstr!("AOUT2"), control: core::ptr::null(), source: cstr!("DAC2") },
    snd_soc_dapm_route { sink: cstr!("AOUT3"), control: core::ptr::null(), source: cstr!("DAC3") },
    snd_soc_dapm_route { sink: cstr!("AOUT4"), control: core::ptr::null(), source: cstr!("DAC4") },
    snd_soc_dapm_route { sink: cstr!("AOUT5"), control: core::ptr::null(), source: cstr!("DAC5") },
    snd_soc_dapm_route { sink: cstr!("DAC1"), control: core::ptr::null(), source: cstr!("SDRX1") },
    snd_soc_dapm_route { sink: cstr!("DAC2"), control: core::ptr::null(), source: cstr!("SDRX2") },
    snd_soc_dapm_route { sink: cstr!("DAC3"), control: core::ptr::null(), source: cstr!("SDRX3") },
    snd_soc_dapm_route { sink: cstr!("DAC4"), control: core::ptr::null(), source: cstr!("SDRX4") },
    snd_soc_dapm_route { sink: cstr!("DAC5"), control: core::ptr::null(), source: cstr!("SDRX5") },
    snd_soc_dapm_route { sink: cstr!("SDRX1"), control: core::ptr::null(), source: cstr!("Playback") },
    snd_soc_dapm_route { sink: cstr!("SDRX2"), control: core::ptr::null(), source: cstr!("Playback") },
    snd_soc_dapm_route { sink: cstr!("SDRX3"), control: core::ptr::null(), source: cstr!("Playback") },
    snd_soc_dapm_route { sink: cstr!("SDRX4"), control: core::ptr::null(), source: cstr!("Playback") },
    snd_soc_dapm_route { sink: cstr!("SDRX5"), control: core::ptr::null(), source: cstr!("Playback") },
    /* Capture */
    snd_soc_dapm_route { sink: cstr!("ADC1"), control: core::ptr::null(), source: cstr!("AIN1") },
    snd_soc_dapm_route { sink: cstr!("ADC2"), control: core::ptr::null(), source: cstr!("AIN2") },
    snd_soc_dapm_route { sink: cstr!("ADC3"), control: core::ptr::null(), source: cstr!("AIN3") },
    snd_soc_dapm_route { sink: cstr!("ADC4"), control: core::ptr::null(), source: cstr!("AIN4") },
    snd_soc_dapm_route { sink: cstr!("SDTX1"), control: core::ptr::null(), source: cstr!("ADC1") },
    snd_soc_dapm_route { sink: cstr!("SDTX2"), control: core::ptr::null(), source: cstr!("ADC2") },
    snd_soc_dapm_route { sink: cstr!("SDTX3"), control: core::ptr::null(), source: cstr!("ADC3") },
    snd_soc_dapm_route { sink: cstr!("SDTX4"), control: core::ptr::null(), source: cstr!("ADC4") },
    snd_soc_dapm_route { sink: cstr!("Capture"), control: core::ptr::null(), source: cstr!("SDTX1") },
    snd_soc_dapm_route { sink: cstr!("Capture"), control: core::ptr::null(), source: cstr!("SDTX2") },
    snd_soc_dapm_route { sink: cstr!("Capture"), control: core::ptr::null(), source: cstr!("SDTX3") },
    snd_soc_dapm_route { sink: cstr!("Capture"), control: core::ptr::null(), source: cstr!("SDTX4") },
    snd_soc_dapm_route { sink: core::ptr::null(), control: core::ptr::null(), source: core::ptr::null() },
];

static cs4234_snd_controls: [snd_kcontrol_new; 0] = [];

unsafe extern "C" fn cs4234_dai_set_fmt(codec_dai: *mut snd_soc_dai, format: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let cs4234 = snd_soc_component_get_drvdata(component) as *mut cs4234;
    let mut sp_ctrl: c_uint = 0;

    (*cs4234).format = format & SND_SOC_DAIFMT_FORMAT_MASK;
    match (*cs4234).format {
        SND_SOC_DAIFMT_LEFT_J => {
            sp_ctrl |= CS4234_LEFT_J << CS4234_SP_FORMAT_SHIFT;
        }
        SND_SOC_DAIFMT_I2S => {
            sp_ctrl |= CS4234_I2S << CS4234_SP_FORMAT_SHIFT;
        }
        SND_SOC_DAIFMT_DSP_A => {
            /* TDM mode in datasheet */
            sp_ctrl |= CS4234_TDM << CS4234_SP_FORMAT_SHIFT;
        }
        _ => {
            dev_err((*component).dev, cstr!("Unsupported dai format\n"));
            return -EINVAL;
        }
    }

    match format & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        SND_SOC_DAIFMT_CBP_CFP => {
            if (*cs4234).format == SND_SOC_DAIFMT_DSP_A {
                dev_err((*component).dev, cstr!("Unsupported DSP A format in master mode\n"));
                return -EINVAL;
            }
            sp_ctrl |= CS4234_MST_SLV_MASK;
        }
        _ => {
            dev_err((*component).dev, cstr!("Unsupported master/slave mode\n"));
            return -EINVAL;
        }
    }

    match format & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => {
            sp_ctrl |= CS4234_INVT_SCLK_MASK;
        }
        _ => {
            dev_err((*component).dev, cstr!("Unsupported inverted clock setting\n"));
            return -EINVAL;
        }
    }

    regmap_update_bits(
        (*cs4234).regmap,
        CS4234_SP_CTRL,
        CS4234_SP_FORMAT_MASK | CS4234_MST_SLV_MASK | CS4234_INVT_SCLK_MASK,
        sp_ctrl,
    );

    0
}

unsafe extern "C" fn cs4234_dai_hw_params(
    sub: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let cs4234 = snd_soc_component_get_drvdata(component) as *mut cs4234;
    let mut double_speed: c_uint = 0;
    let ret: c_int = 0;
    let mut rate_ad: c_int;
    let mut sample_width: c_int;

    (*cs4234).lrclk_rate = params_rate(params) as c_ulong;
    let mut mclk_mult: c_uint = ((*cs4234).mclk_rate / (*cs4234).lrclk_rate) as c_uint;

    if (*cs4234).lrclk_rate > 48000 {
        double_speed = 1;
        mclk_mult = mclk_mult.wrapping_mul(2);
    }

    match mclk_mult {
        256 | 384 | 512 => {
            regmap_update_bits(
                (*cs4234).regmap,
                CS4234_CLOCK_SP,
                CS4234_SPEED_MODE_MASK,
                double_speed << CS4234_SPEED_MODE_SHIFT,
            );
            regmap_update_bits(
                (*cs4234).regmap,
                CS4234_CLOCK_SP,
                CS4234_MCLK_RATE_MASK,
                ((mclk_mult / 128) - 2) << CS4234_MCLK_RATE_SHIFT,
            );
        }
        _ => {
            dev_err((*component).dev, cstr!("Unsupported mclk/lrclk rate\n"));
            return -EINVAL;
        }
    }

    match (*cs4234).lrclk_rate {
        48000 | 96000 => rate_ad = CS4234_48K as c_int,
        44100 | 88200 => rate_ad = CS4234_44K1 as c_int,
        32000 | 64000 => rate_ad = CS4234_32K as c_int,
        _ => {
            dev_err((*component).dev, cstr!("Unsupported LR clock\n"));
            return -EINVAL;
        }
    }
    regmap_update_bits(
        (*cs4234).regmap,
        CS4234_CLOCK_SP,
        CS4234_BASE_RATE_MASK,
        (rate_ad as c_uint) << CS4234_BASE_RATE_SHIFT,
    );

    sample_width = params_width(params);
    match sample_width {
        16 => sample_width = 0,
        18 => sample_width = 1,
        20 => sample_width = 2,
        24 => sample_width = 3,
        _ => {
            dev_err((*component).dev, cstr!("Unsupported sample width\n"));
            return -EINVAL;
        }
    }
    if (*sub).stream == SNDRV_PCM_STREAM_CAPTURE {
        regmap_update_bits(
            (*cs4234).regmap,
            CS4234_SAMPLE_WIDTH,
            CS4234_SDOUTX_SW_MASK,
            (sample_width as c_uint) << CS4234_SDOUTX_SW_SHIFT,
        );
    } else {
        regmap_update_bits(
            (*cs4234).regmap,
            CS4234_SAMPLE_WIDTH,
            CS4234_INPUT_SW_MASK | CS4234_LOW_LAT_SW_MASK | CS4234_DAC5_SW_MASK,
            ((sample_width as c_uint) << CS4234_INPUT_SW_SHIFT)
                | ((sample_width as c_uint) << CS4234_LOW_LAT_SW_SHIFT)
                | ((sample_width as c_uint) << CS4234_DAC5_SW_SHIFT),
        );
    }

    ret
}

/* Scale MCLK rate by 64 to avoid overflow in the ratnum calculation */
const CS4234_MCLK_SCALE: c_uint = 64;

static cs4234_dividers: [snd_ratnum; 2] = [
    snd_ratnum {
        num: 0,
        den_min: 256 / CS4234_MCLK_SCALE,
        den_max: 512 / CS4234_MCLK_SCALE,
        den_step: 128 / CS4234_MCLK_SCALE,
    },
    snd_ratnum {
        num: 0,
        den_min: 128 / CS4234_MCLK_SCALE,
        den_max: 192 / CS4234_MCLK_SCALE,
        den_step: 64 / CS4234_MCLK_SCALE,
    },
];

unsafe extern "C" fn cs4234_dai_rule_rate(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let cs4234 = (*rule).private as *mut cs4234;
    let mclk: c_int = (*cs4234).mclk_rate as c_int;
    let mut ranges: [snd_interval; 2] = [
        snd_interval {
            /* Single Speed Mode */
            min: (mclk / clamp!(mclk / 30000, 256, 512)) as c_uint,
            max: (mclk / clamp!(mclk / 50000, 256, 512)) as c_uint,
        },
        snd_interval {
            /* Double Speed Mode */
            min: (mclk / clamp!(mclk / 60000, 128, 256)) as c_uint,
            max: (mclk / clamp!(mclk / 100000, 128, 256)) as c_uint,
        },
    ];

    snd_interval_ranges(
        hw_param_interval(params, (*rule).var),
        ARRAY_SIZE!(ranges),
        ranges.as_mut_ptr(),
        0,
    )
}

unsafe extern "C" fn cs4234_dai_startup(
    sub: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let comp = (*dai).component;
    let cs4234 = snd_soc_component_get_drvdata(comp) as *mut cs4234;
    let mut ret: c_int;

    match (*cs4234).format {
        SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_I2S => {
            (*cs4234).rate_constraint.nrats = 2;

            /*
             * Playback only supports 24-bit samples in these modes.
             * Note: SNDRV_PCM_HW_PARAM_SAMPLE_BITS constrains the physical
             * width, which we don't care about, so constrain the format.
             */
            if (*sub).stream == SNDRV_PCM_STREAM_PLAYBACK {
                ret = snd_pcm_hw_constraint_mask64(
                    (*sub).runtime,
                    SNDRV_PCM_HW_PARAM_FORMAT,
                    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S24_3LE,
                );
                if ret < 0 {
                    return ret;
                }

                ret = snd_pcm_hw_constraint_minmax(
                    (*sub).runtime,
                    SNDRV_PCM_HW_PARAM_CHANNELS,
                    1,
                    4,
                );
                if ret < 0 {
                    return ret;
                }
            }
        }
        SND_SOC_DAIFMT_DSP_A => {
            (*cs4234).rate_constraint.nrats = 1;
        }
        _ => {
            dev_err((*comp).dev, cstr!("Startup unsupported DAI format\n"));
            return -EINVAL;
        }
    }

    let mut i: c_uint = 0;
    while i < (*cs4234).rate_constraint.nrats {
        (*cs4234).rate_dividers[i as usize].num = ((*cs4234).mclk_rate / CS4234_MCLK_SCALE as c_ulong) as c_uint;
        i += 1;
    }

    ret = snd_pcm_hw_constraint_ratnums(
        (*sub).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &mut (*cs4234).rate_constraint,
    );
    if ret < 0 {
        return ret;
    }

    /*
     * MCLK/rate may be a valid ratio but out-of-spec (e.g. 24576000/64000)
     * so this rule limits the range of sample rate for given MCLK.
     */
    snd_pcm_hw_rule_add(
        (*sub).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        cs4234_dai_rule_rate,
        cs4234 as *mut c_void,
        -1,
    )
}

unsafe extern "C" fn cs4234_dai_set_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: c_uint,
    _rx_mask: c_uint,
    _slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let cs4234 = snd_soc_component_get_drvdata(component) as *mut cs4234;
    let slot_offset: c_uint;
    let mut dac5_slot: c_uint;
    let dac5_mask_group: c_uint;
    let mut dac5_masks: [u8; 4] = [0; 4];

    if slot_width != 32 {
        dev_err((*component).dev, cstr!("Unsupported slot width\n"));
        return -EINVAL;
    }

    /* Either 4 or 5 consecutive bits, DAC5 is optional */
    slot_offset = (ffs(tx_mask as c_int) - 1) as c_uint;
    tx_mask >>= slot_offset;
    if (slot_offset % 4) != 0 || ((tx_mask != 0x0F) && (tx_mask != 0x1F)) {
        dev_err((*component).dev, cstr!("Unsupported tx slots allocation\n"));
        return -EINVAL;
    }

    regmap_update_bits(
        (*cs4234).regmap,
        CS4234_SP_DATA_SEL,
        CS4234_DAC14_SRC_MASK,
        (slot_offset / 4) << CS4234_DAC14_SRC_SHIFT,
    );
    regmap_update_bits(
        (*cs4234).regmap,
        CS4234_SP_DATA_SEL,
        CS4234_LL_SRC_MASK,
        (slot_offset / 4) << CS4234_LL_SRC_SHIFT,
    );

    if tx_mask == 0x1F {
        dac5_slot = slot_offset + 4;
        dac5_masks = [0xFF; 4];
        dac5_mask_group = dac5_slot / 8;
        dac5_slot %= 8;
        dac5_masks[dac5_mask_group as usize] ^= BIT!(7 - dac5_slot) as u8;
        regmap_bulk_write(
            (*cs4234).regmap,
            CS4234_SDIN1_MASK1,
            dac5_masks.as_ptr(),
            dac5_masks.len(),
        );
    }

    0
}

static cs4234_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(cs4234_dai_set_fmt),
    hw_params: Some(cs4234_dai_hw_params),
    startup: Some(cs4234_dai_startup),
    set_tdm_slot: Some(cs4234_dai_set_tdm_slot),
};

static mut cs4234_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: cstr!("cs4234-dai"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("Playback"),
        channels_min: 1,
        channels_max: 5,
        rates: unsafe { CS4234_PCM_RATES },
        formats: unsafe { CS4234_FORMATS },
    },
    capture: snd_soc_pcm_stream {
        stream_name: cstr!("Capture"),
        channels_min: 1,
        channels_max: 4,
        rates: unsafe { CS4234_PCM_RATES },
        formats: unsafe { CS4234_FORMATS },
    },
    ops: &cs4234_dai_ops,
    symmetric_rate: 1,
}];

static cs4234_default_reg: [reg_default; 26] = unsafe {
    [
        reg_default { reg: CS4234_CLOCK_SP, def: 0x04 },
        reg_default { reg: CS4234_SAMPLE_WIDTH, def: 0xFF },
        reg_default { reg: CS4234_SP_CTRL, def: 0x48 },
        reg_default { reg: CS4234_SP_DATA_SEL, def: 0x01 },
        reg_default { reg: CS4234_SDIN1_MASK1, def: 0xFF },
        reg_default { reg: CS4234_SDIN1_MASK2, def: 0xFF },
        reg_default { reg: CS4234_SDIN2_MASK1, def: 0xFF },
        reg_default { reg: CS4234_SDIN2_MASK2, def: 0xFF },
        reg_default { reg: CS4234_TPS_CTRL, def: 0x00 },
        reg_default { reg: CS4234_ADC_CTRL1, def: 0xC0 },
        reg_default { reg: CS4234_ADC_CTRL2, def: 0xFF },
        reg_default { reg: CS4234_LOW_LAT_CTRL1, def: 0xE0 },
        reg_default { reg: CS4234_DAC_CTRL1, def: 0xE0 },
        reg_default { reg: CS4234_DAC_CTRL2, def: 0xE0 },
        reg_default { reg: CS4234_DAC_CTRL3, def: 0xBF },
        reg_default { reg: CS4234_DAC_CTRL4, def: 0x1F },
        reg_default { reg: CS4234_VOLUME_MODE, def: 0x87 },
        reg_default { reg: CS4234_MASTER_VOL, def: 0x10 },
        reg_default { reg: CS4234_DAC1_VOL, def: 0x10 },
        reg_default { reg: CS4234_DAC2_VOL, def: 0x10 },
        reg_default { reg: CS4234_DAC3_VOL, def: 0x10 },
        reg_default { reg: CS4234_DAC4_VOL, def: 0x10 },
        reg_default { reg: CS4234_DAC5_VOL, def: 0x10 },
        reg_default { reg: CS4234_INT_CTRL, def: 0x40 },
        reg_default { reg: CS4234_INT_MASK1, def: 0x10 },
        reg_default { reg: CS4234_INT_MASK2, def: 0x20 },
    ]
};

unsafe extern "C" fn cs4234_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        r if r >= CS4234_DEVID_AB && r <= CS4234_DEVID_EF => true,
        r if r >= CS4234_REVID && r <= CS4234_DAC5_VOL => true,
        r if r >= CS4234_INT_CTRL && r <= CS4234_MAX_REGISTER => true,
        _ => false,
    }
}

unsafe extern "C" fn cs4234_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        r if r == CS4234_INT_NOTIFY1 => true,
        r if r == CS4234_INT_NOTIFY2 => true,
        _ => false,
    }
}

unsafe extern "C" fn cs4234_writeable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        r if r >= CS4234_DEVID_AB && r <= CS4234_REVID => false,
        r if r >= CS4234_INT_NOTIFY1 && r <= CS4234_INT_NOTIFY2 => false,
        _ => true,
    }
}

static soc_component_cs4234: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: cs4234_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(cs4234_dapm_widgets),
    dapm_routes: cs4234_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(cs4234_dapm_routes),
    controls: cs4234_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(cs4234_snd_controls),
    set_bias_level: Some(cs4234_set_bias_level),
    idle_bias_on: 1,
    suspend_bias_off: 1,
    endianness: 1,
};

static cs4234_regmap: regmap_config = unsafe {
    regmap_config {
        reg_bits: 8,
        val_bits: 8,
        max_register: CS4234_MAX_REGISTER,
        readable_reg: Some(cs4234_readable_register),
        volatile_reg: Some(cs4234_volatile_reg),
        writeable_reg: Some(cs4234_writeable_register),
        reg_defaults: cs4234_default_reg.as_ptr(),
        num_reg_defaults: ARRAY_SIZE!(cs4234_default_reg),
        cache_type: REGCACHE_MAPLE,
        use_single_read: true,
        use_single_write: true,
    }
};

static cs4234_core_supplies: [*const c_char; 2] = [
    cstr!("VA"),
    cstr!("VL"),
];

unsafe fn cs4234_shutdown(cs4234: *mut cs4234) {
    cancel_delayed_work_sync(&mut (*cs4234).vq_ramp_delay);
    reinit_completion(&mut (*cs4234).vq_ramp_complete);

    regmap_update_bits(
        (*cs4234).regmap,
        CS4234_DAC_CTRL4,
        CS4234_VQ_RAMP_MASK,
        CS4234_VQ_RAMP_MASK,
    );
    msleep(50);
    regcache_cache_only((*cs4234).regmap, true);
    /* Clear VQ Ramp Bit in cache for the next PowerUp */
    regmap_update_bits((*cs4234).regmap, CS4234_DAC_CTRL4, CS4234_VQ_RAMP_MASK, 0);
    gpiod_set_value_cansleep((*cs4234).reset_gpio, 0);
    regulator_bulk_disable((*cs4234).num_core_supplies, (*cs4234).core_supplies.as_mut_ptr());
    clk_disable_unprepare((*cs4234).mclk);
}

unsafe fn cs4234_powerup(cs4234: *mut cs4234) -> c_int {
    let mut ret: c_int;

    ret = clk_prepare_enable((*cs4234).mclk);
    if ret != 0 {
        dev_err((*cs4234).dev, cstr!("Failed to enable mclk: %d\n"), ret);
        return ret;
    }

    ret = regulator_bulk_enable((*cs4234).num_core_supplies, (*cs4234).core_supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*cs4234).dev, cstr!("Failed to enable core supplies: %d\n"), ret);
        clk_disable_unprepare((*cs4234).mclk);
        return ret;
    }

    usleep_range(CS4234_HOLD_RESET_TIME_US, 2 * CS4234_HOLD_RESET_TIME_US);
    gpiod_set_value_cansleep((*cs4234).reset_gpio, 1);

    /* Make sure hardware reset done 2 ms + (3000/MCLK) */
    usleep_range(CS4234_BOOT_TIME_US, CS4234_BOOT_TIME_US * 2);

    queue_delayed_work(
        system_power_efficient_wq,
        &mut (*cs4234).vq_ramp_delay,
        msecs_to_jiffies(CS4234_VQ_CHARGE_MS),
    );

    0
}

unsafe extern "C" fn cs4234_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let mut cs4234: *mut cs4234;
    let dev = &mut (*i2c_client).dev as *mut device;
    let mut revid: c_uint = 0;
    let mut devid: u32;
    let mut ids: [u8; 3] = [0; 3];
    let mut ret: c_int = 0;
    let mut i: c_int;

    cs4234 = devm_kzalloc(dev, core::mem::size_of::<cs4234>(), GFP_KERNEL) as *mut cs4234;
    if cs4234.is_null() {
        return -ENOMEM;
    }
    i2c_set_clientdata(i2c_client, cs4234 as *mut c_void);
    (*cs4234).dev = dev;
    init_completion(&mut (*cs4234).vq_ramp_complete);
    // INIT_DELAYED_WORK(&cs4234->vq_ramp_delay, cs4234_vq_ramp_done);

    (*cs4234).reset_gpio = devm_gpiod_get(dev, cstr!("reset"), GPIOD_OUT_LOW);
    if IS_ERR((*cs4234).reset_gpio as *const c_void) {
        return PTR_ERR((*cs4234).reset_gpio as *const c_void);
    }

    // BUILD_BUG_ON(ARRAY_SIZE(cs4234->core_supplies) < ARRAY_SIZE(cs4234_core_supplies));

    (*cs4234).num_core_supplies = ARRAY_SIZE!(cs4234_core_supplies) as c_int;
    i = 0;
    while i < ARRAY_SIZE!(cs4234_core_supplies) as c_int {
        (*cs4234).core_supplies[i as usize].supply = cs4234_core_supplies[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, (*cs4234).num_core_supplies, (*cs4234).core_supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, cstr!("Failed to request core supplies %d\n"), ret);
        return ret;
    }

    (*cs4234).mclk = devm_clk_get(dev, cstr!("mclk"));
    if IS_ERR((*cs4234).mclk as *const c_void) {
        ret = PTR_ERR((*cs4234).mclk as *const c_void);
        dev_err(dev, cstr!("Failed to get the mclk: %d\n"), ret);
        return ret;
    }
    (*cs4234).mclk_rate = clk_get_rate((*cs4234).mclk);

    if (*cs4234).mclk_rate < 7680000 || (*cs4234).mclk_rate > 25600000 {
        dev_err(dev, cstr!("Invalid Master Clock rate\n"));
        return -EINVAL;
    }

    (*cs4234).regmap = devm_regmap_init_i2c(i2c_client, &cs4234_regmap);
    if IS_ERR((*cs4234).regmap as *const c_void) {
        ret = PTR_ERR((*cs4234).regmap as *const c_void);
        dev_err(dev, cstr!("regmap_init() failed: %d\n"), ret);
        return ret;
    }

    ret = cs4234_powerup(cs4234);
    if ret != 0 {
        return ret;
    }

    ret = regmap_bulk_read((*cs4234).regmap, CS4234_DEVID_AB, ids.as_mut_ptr(), ids.len());
    if ret < 0 {
        dev_err(dev, cstr!("Failed to read DEVID: %d\n"), ret);
        cs4234_shutdown(cs4234);
        return ret;
    }

    devid = ((ids[0] as u32) << 16) | ((ids[1] as u32) << 8) | ids[2] as u32;
    if devid != CS4234_SUPPORTED_ID {
        dev_err(dev, cstr!("Unknown device ID: %x\n"), devid);
        ret = -EINVAL;
        cs4234_shutdown(cs4234);
        return ret;
    }

    ret = regmap_read((*cs4234).regmap, CS4234_REVID, &mut revid);
    if ret < 0 {
        dev_err(dev, cstr!("Failed to read CS4234_REVID: %d\n"), ret);
        cs4234_shutdown(cs4234);
        return ret;
    }

    dev_info(
        dev,
        cstr!("Cirrus Logic CS4234, Alpha Rev: %02X, Numeric Rev: %02X\n"),
        (revid & 0xF0) >> 4,
        revid & 0x0F,
    );

    ret = regulator_get_voltage((*cs4234).core_supplies[CS4234_SUPPLY_VA].consumer);
    match ret {
        3135000..=3650000 => {
            regmap_update_bits(
                (*cs4234).regmap,
                CS4234_ADC_CTRL1,
                CS4234_VA_SEL_MASK,
                CS4234_3V3 << CS4234_VA_SEL_SHIFT,
            );
        }
        4750000..=5250000 => {
            regmap_update_bits(
                (*cs4234).regmap,
                CS4234_ADC_CTRL1,
                CS4234_VA_SEL_MASK,
                CS4234_5V << CS4234_VA_SEL_SHIFT,
            );
        }
        _ => {
            dev_err(dev, cstr!("Invalid VA voltage\n"));
            ret = -EINVAL;
            cs4234_shutdown(cs4234);
            return ret;
        }
    }

    pm_runtime_set_active(&mut (*i2c_client).dev);
    pm_runtime_enable(&mut (*i2c_client).dev);

    (*cs4234).rate_dividers = cs4234_dividers;
    (*cs4234).rate_constraint.rats = (*cs4234).rate_dividers.as_mut_ptr();

    ret = snd_soc_register_component(
        dev,
        &soc_component_cs4234,
        cs4234_dai.as_mut_ptr(),
        ARRAY_SIZE!(cs4234_dai) as c_int,
    );
    if ret < 0 {
        dev_err(dev, cstr!("Failed to register component:%d\n"), ret);
        pm_runtime_disable(&mut (*i2c_client).dev);
        cs4234_shutdown(cs4234);
        return ret;
    }

    ret
}

unsafe extern "C" fn cs4234_i2c_remove(i2c_client: *mut i2c_client) {
    let cs4234 = i2c_get_clientdata(i2c_client) as *mut cs4234;
    let dev = &mut (*i2c_client).dev as *mut device;

    snd_soc_unregister_component(dev);
    pm_runtime_disable(dev);
    cs4234_shutdown(cs4234);
}

unsafe extern "C" fn cs4234_runtime_resume(dev: *mut device) -> c_int {
    let cs4234 = dev_get_drvdata(dev) as *mut cs4234;
    let mut ret: c_int;

    ret = cs4234_powerup(cs4234);
    if ret != 0 {
        return ret;
    }

    regcache_mark_dirty((*cs4234).regmap);
    regcache_cache_only((*cs4234).regmap, false);
    ret = regcache_sync((*cs4234).regmap);
    if ret != 0 {
        dev_err(dev, cstr!("Failed to sync regmap: %d\n"), ret);
        cs4234_shutdown(cs4234);
        return ret;
    }

    0
}

unsafe extern "C" fn cs4234_runtime_suspend(dev: *mut device) -> c_int {
    let cs4234 = dev_get_drvdata(dev) as *mut cs4234;

    cs4234_shutdown(cs4234);

    0
}

static cs4234_pm: dev_pm_ops = dev_pm_ops {
    // RUNTIME_PM_OPS(cs4234_runtime_suspend, cs4234_runtime_resume, NULL)
    _private: [],
};

static cs4234_of_match: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("cirrus,cs4234") },
    of_device_id { compatible: core::ptr::null() },
];
// MODULE_DEVICE_TABLE(of, cs4234_of_match);

static mut cs4234_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: cstr!("cs4234"),
        pm: unsafe { pm_ptr(&cs4234_pm) },
        of_match_table: cs4234_of_match.as_ptr(),
    },
    probe: Some(cs4234_i2c_probe),
    remove: Some(cs4234_i2c_remove),
};
// module_i2c_driver(cs4234_i2c_driver);

// MODULE_DESCRIPTION("ASoC Cirrus Logic CS4234 driver");
// MODULE_AUTHOR("Lucas Tanure <tanureal@opensource.cirrus.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
