// SPDX-License-Identifier: GPL-2.0-only
/*
 * Nuvoton NAU8825 audio codec driver
 *
 * Copyright 2015 Google Chromium project.
 *  Author: Anatol Pomozov <anatol@chromium.org>
 * Copyright 2015 Nuvoton Technology Corp.
 *  Co-author: Meng-Huang Kuo <mhkuo@nuvoton.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

/* C dependencies originally included from Linux, ASoC, and "nau8825.h". */

type bool_ = bool;
type u32 = u32;
type u64_ = u64;
type irqreturn_t = c_int;

#[repr(C)]
pub struct device {
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
pub struct semaphore {
    pub count: c_int,
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub regmap: *mut regmap,
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
    pub data: *mut c_void,
}
#[repr(C)]
pub struct soc_bytes_ext {
    pub max: c_uint,
}
#[repr(C)]
pub struct soc_enum {
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_soc_jack {
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
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub suspend_bias_off: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
pub type snd_soc_bias_level = c_int;
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub val_bits: c_uint,
    pub reg_bits: c_uint,
    pub max_register: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
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
pub struct acpi_device_id {
    pub id: *const c_char,
    pub driver_data: c_ulong,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}
#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct nau8825 {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub mclk: *mut clk,
    pub mclk_freq: c_uint,
    pub irq: c_int,
    pub dapm: *mut snd_soc_dapm_context,
    pub jack: *mut snd_soc_jack,
    pub xtalk_sem: semaphore,
    pub xtalk_work: work_struct,
    pub xtalk_state: c_int,
    pub xtalk_event: c_int,
    pub xtalk_event_mask: c_int,
    pub xtalk_enable: bool,
    pub xtalk_protect: bool,
    pub xtalk_baktab_initialized: bool,
    pub imp_rms: [c_uint; 4],
    pub button_pressed: c_int,
    pub high_imped: bool,
    pub sw_id: c_int,
    pub jkdet_enable: bool,
    pub jkdet_pull_enable: bool,
    pub jkdet_pull_up: bool,
    pub jkdet_polarity: c_uint,
    pub micbias_voltage: c_uint,
    pub vref_impedance: c_uint,
    pub sar_threshold_num: c_uint,
    pub sar_threshold: [c_uint; 8],
    pub sar_hysteresis: c_uint,
    pub sar_voltage: c_uint,
    pub sar_compare_time: c_uint,
    pub sar_sampling_time: c_uint,
    pub key_debounce: c_uint,
    pub jack_insert_debounce: c_uint,
    pub jack_eject_debounce: c_uint,
    pub adcout_ds: c_uint,
    pub adc_delay: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct nau8825_fll {
    mclk_src: c_int,
    ratio: c_int,
    fll_frac: c_int,
    fll_frac_num: c_int,
    fll_int: c_int,
    clk_ref_div: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct nau8825_fll_attr {
    param: c_uint,
    val: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct nau8825_osr_attr {
    osr: c_uint,
    clk_src: c_uint,
}

const NUVOTON_CODEC_DAI: &[u8] = b"nau8825-hifi\0";
const NAU_FREF_MAX: c_uint = 13_500_000;
const NAU_FVCO_MAX: u64 = 124_000_000;
const NAU_FVCO_MIN: u64 = 90_000_000;
const GAIN_AUGMENT: u32 = 22500;
const SIDETONE_BASE: u32 = 207000;
const CLK_DA_AD_MAX: c_uint = 6_144_000;
const NAU8825_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const NAU8825_FORMATS: c_ulong =
    (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |
     SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE) as c_ulong;
const NAU8825_BUTTONS: c_int =
    SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3;

extern "C" {
    static HZ: c_long;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;
    static GFP_KERNEL: c_uint;
    static GFP_DMA: c_uint;
    static IRQF_TRIGGER_LOW: c_uint;
    static IRQF_ONESHOT: c_uint;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    static REGCACHE_RBTREE: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S24_3LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S32_LE: c_ulong;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_NOPM: c_uint;
    static SND_SOC_BIAS_ON: snd_soc_bias_level;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_HEADSET: c_int;
    static SND_JACK_BTN_0: c_int;
    static SND_JACK_BTN_1: c_int;
    static SND_JACK_BTN_2: c_int;
    static SND_JACK_BTN_3: c_int;
    static SND_JACK_BTN_4: c_int;
    static SND_JACK_BTN_5: c_int;

    fn down_timeout(sem: *mut semaphore, timeout: c_long) -> c_int;
    fn down_trylock(sem: *mut semaphore) -> c_int;
    fn up(sem: *mut semaphore);
    fn usleep_range(min: c_uint, max: c_uint);
    fn msleep(ms: c_uint);
    fn mdelay(ms: c_uint);
    fn intlog10(value: u32) -> u32;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_raw_read(map: *mut regmap, reg: c_uint, val: *mut c_void, len: c_uint) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: c_uint, val: *const c_void, len: c_uint) -> c_int;
    fn regmap_register_patch(map: *mut regmap, patch: *const reg_sequence, len: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn kmemdup(src: *const c_void, len: c_uint, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut nau8825;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut c_void, what: c_int, min: c_uint, max: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_uint;
    fn hweight_long(w: c_ulong) -> c_uint;
    fn fls(x: c_int) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn schedule_work(work: *mut work_struct) -> bool;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_round_rate(clk: *mut clk, rate: c_uint) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn device_property_read_bool(dev: *mut device, propname: *const c_char) -> bool;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint) -> c_int;
    fn device_property_read_u32_array(dev: *mut device, propname: *const c_char, val: *mut c_uint, nval: c_uint) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *mut nau8825;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn disable_irq(irq: c_int);
    fn enable_irq(irq: c_int);
}

extern "C" {
    static NAU8825_REG_ENA_CTRL: c_uint; static NAU8825_REG_IIC_ADDR_SET: c_uint;
    static NAU8825_REG_CLK_DIVIDER: c_uint; static NAU8825_REG_FLL1: c_uint;
    static NAU8825_REG_FLL2: c_uint; static NAU8825_REG_FLL3: c_uint;
    static NAU8825_REG_FLL4: c_uint; static NAU8825_REG_FLL5: c_uint;
    static NAU8825_REG_FLL6: c_uint; static NAU8825_REG_FLL_VCO_RSV: c_uint;
    static NAU8825_REG_HSD_CTRL: c_uint; static NAU8825_REG_JACK_DET_CTRL: c_uint;
    static NAU8825_REG_INTERRUPT_MASK: c_uint; static NAU8825_REG_INTERRUPT_DIS_CTRL: c_uint;
    static NAU8825_REG_SAR_CTRL: c_uint; static NAU8825_REG_KEYDET_CTRL: c_uint;
    static NAU8825_REG_VDET_THRESHOLD_1: c_uint; static NAU8825_REG_VDET_THRESHOLD_2: c_uint;
    static NAU8825_REG_VDET_THRESHOLD_3: c_uint; static NAU8825_REG_VDET_THRESHOLD_4: c_uint;
    static NAU8825_REG_GPIO34_CTRL: c_uint; static NAU8825_REG_GPIO12_CTRL: c_uint;
    static NAU8825_REG_TDM_CTRL: c_uint; static NAU8825_REG_I2S_PCM_CTRL1: c_uint;
    static NAU8825_REG_I2S_PCM_CTRL2: c_uint; static NAU8825_REG_LEFT_TIME_SLOT: c_uint;
    static NAU8825_REG_RIGHT_TIME_SLOT: c_uint; static NAU8825_REG_BIQ_CTRL: c_uint;
    static NAU8825_REG_BIQ_COF1: c_uint; static NAU8825_REG_BIQ_COF2: c_uint;
    static NAU8825_REG_BIQ_COF3: c_uint; static NAU8825_REG_BIQ_COF4: c_uint;
    static NAU8825_REG_BIQ_COF5: c_uint; static NAU8825_REG_BIQ_COF6: c_uint;
    static NAU8825_REG_BIQ_COF7: c_uint; static NAU8825_REG_BIQ_COF8: c_uint;
    static NAU8825_REG_BIQ_COF9: c_uint; static NAU8825_REG_BIQ_COF10: c_uint;
    static NAU8825_REG_ADC_RATE: c_uint; static NAU8825_REG_DAC_CTRL1: c_uint;
    static NAU8825_REG_DAC_CTRL2: c_uint; static NAU8825_REG_DAC_DGAIN_CTRL: c_uint;
    static NAU8825_REG_ADC_DGAIN_CTRL: c_uint; static NAU8825_REG_MUTE_CTRL: c_uint;
    static NAU8825_REG_HSVOL_CTRL: c_uint; static NAU8825_REG_DACL_CTRL: c_uint;
    static NAU8825_REG_DACR_CTRL: c_uint; static NAU8825_REG_ADC_DRC_KNEE_IP12: c_uint;
    static NAU8825_REG_ADC_DRC_KNEE_IP34: c_uint; static NAU8825_REG_ADC_DRC_SLOPES: c_uint;
    static NAU8825_REG_ADC_DRC_ATKDCY: c_uint; static NAU8825_REG_DAC_DRC_KNEE_IP12: c_uint;
    static NAU8825_REG_DAC_DRC_KNEE_IP34: c_uint; static NAU8825_REG_DAC_DRC_SLOPES: c_uint;
    static NAU8825_REG_DAC_DRC_ATKDCY: c_uint; static NAU8825_REG_IMM_MODE_CTRL: c_uint;
    static NAU8825_REG_IMM_RMS_L: c_uint; static NAU8825_REG_IMM_RMS_R: c_uint;
    static NAU8825_REG_CLASSG_CTRL: c_uint; static NAU8825_REG_OPT_EFUSE_CTRL: c_uint;
    static NAU8825_REG_MISC_CTRL: c_uint; static NAU8825_REG_I2C_DEVICE_ID: c_uint;
    static NAU8825_REG_FLL2_LOWER: c_uint; static NAU8825_REG_FLL2_UPPER: c_uint;
    static NAU8825_REG_BIAS_ADJ: c_uint; static NAU8825_REG_TRIM_SETTINGS: c_uint;
    static NAU8825_REG_ANALOG_CONTROL_1: c_uint; static NAU8825_REG_ANALOG_CONTROL_2: c_uint;
    static NAU8825_REG_ANALOG_ADC_1: c_uint; static NAU8825_REG_ANALOG_ADC_2: c_uint;
    static NAU8825_REG_RDAC: c_uint; static NAU8825_REG_MIC_BIAS: c_uint;
    static NAU8825_REG_BOOST: c_uint; static NAU8825_REG_FEPGA: c_uint;
    static NAU8825_REG_POWER_UP_CONTROL: c_uint; static NAU8825_REG_CHARGE_PUMP: c_uint;
    static NAU8825_REG_RESET: c_uint; static NAU8825_REG_IRQ_STATUS: c_uint;
    static NAU8825_REG_INT_CLR_KEY_STATUS: c_uint; static NAU8825_REG_SARDOUT_RAM_STATUS: c_uint;
    static NAU8825_REG_CHARGE_PUMP_INPUT_READ: c_uint; static NAU8825_REG_GENERAL_STATUS: c_uint;
    static NAU8825_REG_MAX: c_uint; static NAU8825_REG_DATA_LEN: c_uint; static NAU8825_REG_ADDR_LEN: c_uint;
}

extern "C" {
    static NAU8825_HP_VOL_MIN: c_uint;
    static NAU8825_HPL_VOL_MASK: c_uint; static NAU8825_HPR_VOL_MASK: c_uint; static NAU8825_HPL_VOL_SFT: c_uint;
    static NAU8825_ENABLE_DACR: c_uint; static NAU8825_ENABLE_DACL: c_uint; static NAU8825_ENABLE_ADC: c_uint;
    static NAU8825_ENABLE_ADC_CLK: c_uint; static NAU8825_ENABLE_DAC_CLK: c_uint;
    static NAU8825_JAMNODCLOW: c_uint; static NAU8825_CHANRGE_PUMP_EN: c_uint;
    static NAU8825_RDAC_EN: c_uint; static NAU8825_RDAC_CLK_EN: c_uint; static NAU8825_RDAC_FS_BCLK_ENB: c_uint;
    static NAU8825_POWERUP_INTEGR_R: c_uint; static NAU8825_POWERUP_INTEGR_L: c_uint;
    static NAU8825_POWERUP_DRV_IN_R: c_uint; static NAU8825_POWERUP_DRV_IN_L: c_uint;
    static NAU8825_POWERUP_HP_DRV_R: c_uint; static NAU8825_POWERUP_HP_DRV_L: c_uint;
    static NAU8825_SPKR_DWN1R: c_uint; static NAU8825_SPKR_DWN1L: c_uint; static NAU8825_HP_BOOST_DIS: c_uint;
    static NAU8825_CLASSG_LDAC_EN: c_uint; static NAU8825_CLASSG_RDAC_EN: c_uint;
    static NAU8825_POWERUP_ADCL: c_uint; static NAU8825_ADC_VREFSEL_MASK: c_uint; static NAU8825_ADC_VREFSEL_VMID_PLUS_0_5DB: c_uint;
    static NAU8825_CLK_SRC_MASK: c_uint; static NAU8825_CLK_SRC_VCO: c_uint; static NAU8825_CLK_MCLK_SRC_MASK: c_uint;
    static NAU8825_DCO_EN: c_uint; static NAU8825_FLL_RATIO_MASK: c_uint;
    static NAU8825_I2S_MS_MASK: c_uint; static NAU8825_I2S_LRC_DIV_MASK: c_uint; static NAU8825_I2S_BLK_DIV_MASK: c_uint;
    static NAU8825_I2S_MS_MASTER: c_uint; static NAU8825_I2S_MS_SLAVE: c_uint; static NAU8825_I2S_LRC_DIV_SFT: c_uint;
    static NAU8825_DACL_CH_SEL_MASK: c_uint; static NAU8825_DACL_CH_VOL_MASK: c_uint; static NAU8825_DACL_CH_SEL_L: c_uint; static NAU8825_DACL_CH_SEL_R: c_uint;
    static NAU8825_DACR_CH_SEL_MASK: c_uint; static NAU8825_DACR_CH_VOL_MASK: c_uint; static NAU8825_DACR_CH_SEL_R: c_uint;
    static NAU8825_IMM_THD_MASK: c_uint; static NAU8825_IMM_GEN_VOL_MASK: c_uint; static NAU8825_IMM_CYC_MASK: c_uint; static NAU8825_IMM_DAC_SRC_MASK: c_uint;
    static NAU8825_IMM_THD_SFT: c_uint; static NAU8825_IMM_GEN_VOL_1_16th: c_uint; static NAU8825_IMM_CYC_8192: c_uint; static NAU8825_IMM_DAC_SRC_SIN: c_uint; static NAU8825_IMM_EN: c_uint;
    static NAU8825_IRQ_RMS_EN: c_uint; static NAU8825_POWER_DOWN_DACR: c_uint; static NAU8825_POWER_DOWN_DACL: c_uint;
    static NAU8825_BIAS_HPR_IMP: c_uint; static NAU8825_BIAS_HPL_IMP: c_uint; static NAU8825_BIAS_TESTDAC_EN: c_uint; static NAU8825_BIAS_TESTDACR_EN: c_uint; static NAU8825_BIAS_TESTDACL_EN: c_uint;
    static NAU8825_ADC_DIG_VOL_MASK: c_uint;
    static NAU8825_XTALK_PREPARE: c_int; static NAU8825_XTALK_HPR_R2L: c_int; static NAU8825_XTALK_HPL_R2L: c_int; static NAU8825_XTALK_IMM: c_int; static NAU8825_XTALK_DONE: c_int;
    static NAU8825_SOFTWARE_ID_NAU8825: c_int; static NAU8825_SOFTWARE_ID_NAU8825C: c_int; static NAU8825_SOFTWARE_ID_MASK: c_int;
    static NAU8825_ACDC_CTRL_MASK: c_uint; static NAU8825_ACDC_VREF_MICP: c_uint; static NAU8825_ACDC_VREF_MICN: c_uint; static NAU8825_DISCHRG_EN: c_uint;
    static NAU8825_BIQ_WRT_EN: c_uint; static NAU8825_BIQ_PATH_SFT: c_uint; static NAU8825_ADC_SYNC_DOWN_SFT: c_uint; static NAU8825_DAC_OVERSAMPLE_SFT: c_uint;
    static NAU8825_DAC_OVERSAMPLE_MASK: c_uint; static NAU8825_ADC_SYNC_DOWN_MASK: c_uint;
    static NAU8825_CLK_DAC_SRC_MASK: c_uint; static NAU8825_CLK_DAC_SRC_SFT: c_uint; static NAU8825_CLK_ADC_SRC_MASK: c_uint; static NAU8825_CLK_ADC_SRC_SFT: c_uint;
    static NAU8825_I2S_DL_16: c_uint; static NAU8825_I2S_DL_20: c_uint; static NAU8825_I2S_DL_24: c_uint; static NAU8825_I2S_DL_32: c_uint; static NAU8825_I2S_DL_MASK: c_uint;
    static NAU8825_I2S_BP_INV: c_uint; static NAU8825_I2S_DF_I2S: c_uint; static NAU8825_I2S_DF_LEFT: c_uint; static NAU8825_I2S_DF_RIGTH: c_uint; static NAU8825_I2S_DF_PCM_AB: c_uint; static NAU8825_I2S_PCMB_EN: c_uint;
    static NAU8825_I2S_DF_MASK: c_uint; static NAU8825_I2S_BP_MASK: c_uint; static NAU8825_I2S_PCMB_MASK: c_uint;
    static NAU8825_I2S_PCM_TS_EN_MASK: c_uint; static NAU8825_I2S_PCM_TS_EN: c_uint; static NAU8825_TDM_OFFSET_EN: c_uint; static NAU8825_TDM_MODE: c_uint;
    static NAU8825_TDM_DACR_RX_SFT: c_uint; static NAU8825_TDM_DACL_RX_SFT: c_uint; static NAU8825_TDM_DACL_RX_MASK: c_uint; static NAU8825_TDM_DACR_RX_MASK: c_uint; static NAU8825_TDM_TX_MASK: c_uint; static NAU8825_TSLOT_L0_MASK: c_uint;
    static NAU8825_HSD_AUTO_MODE: c_uint; static NAU8825_JACK_POLARITY: c_uint; static NAU8825_GPIO2JD1: c_uint; static NAU8825_JACK_DET_RESTART: c_uint; static NAU8825_JACK_DET_DB_BYPASS: c_uint;
    static NAU8825_IRQ_EJECT_DIS: c_uint; static NAU8825_IRQ_INSERT_DIS: c_uint; static NAU8825_IRQ_OUTPUT_EN: c_uint; static NAU8825_IRQ_EJECT_EN: c_uint; static NAU8825_IRQ_HEADSET_COMPLETE_EN: c_uint; static NAU8825_IRQ_INSERT_EN: c_uint;
    static NAU8825_SAR_ADC_EN_SFT: c_uint; static NAU8825_MICBIAS_JKSLV: c_uint; static NAU8825_MICBIAS_JKR2: c_uint;
    static NAU8825_KEY_SHORT_PRESS_IRQ: c_int; static NAU8825_KEY_RELEASE_IRQ: c_int; static NAU8825_HEADSET_COMPLETION_IRQ: c_int; static NAU8825_IMPEDANCE_MEAS_IRQ: c_int;
    static NAU8825_JACK_EJECTION_IRQ_MASK: c_int; static NAU8825_JACK_EJECTION_DETECTED: c_int; static NAU8825_JACK_INSERTION_IRQ_MASK: c_int; static NAU8825_JACK_INSERTION_DETECTED: c_int;
    static NAU8825_SPKR_ENGND1: c_uint; static NAU8825_SPKR_ENGND2: c_uint;
    static NAU8825_TESTDACIN_MASK: c_uint; static NAU8825_TESTDACIN_GND: c_uint; static NAU8825_MICBIAS_LOWNOISE_MASK: c_uint; static NAU8825_MICBIAS_VOLTAGE_MASK: c_uint; static NAU8825_MICBIAS_LOWNOISE_EN: c_uint;
    static NAU8825_SAR_INPUT_MASK: c_uint; static NAU8825_SAR_TRACKING_GAIN_MASK: c_uint; static NAU8825_SAR_HV_SEL_MASK: c_uint; static NAU8825_SAR_RES_SEL_MASK: c_uint; static NAU8825_SAR_COMPARE_TIME_MASK: c_uint; static NAU8825_SAR_SAMPLING_TIME_MASK: c_uint;
    static NAU8825_SAR_HV_SEL_VDDMIC: c_uint; static NAU8825_SAR_RES_SEL_70K: c_uint; static NAU8825_SAR_INPUT_JKSLV: c_uint; static NAU8825_SAR_INPUT_JKR2: c_uint;
    static NAU8825_SAR_TRACKING_GAIN_SFT: c_uint; static NAU8825_SAR_COMPARE_TIME_SFT: c_uint; static NAU8825_SAR_SAMPLING_TIME_SFT: c_uint;
    static NAU8825_KEYDET_LEVELS_NR_MASK: c_uint; static NAU8825_KEYDET_LEVELS_NR_SFT: c_uint; static NAU8825_KEYDET_HYSTERESIS_MASK: c_uint; static NAU8825_KEYDET_HYSTERESIS_SFT: c_uint; static NAU8825_KEYDET_SHORTKEY_DEBOUNCE_MASK: c_uint; static NAU8825_KEYDET_SHORTKEY_DEBOUNCE_SFT: c_uint;
    static NAU8825_IRQ_KEY_SHORT_PRESS_EN: c_uint; static NAU8825_IRQ_KEY_RELEASE_EN: c_uint;
    static NAU8825_BIAS_VMID: c_uint; static NAU8825_BIAS_VMID_SEL_MASK: c_uint; static NAU8825_BIAS_VMID_SEL_SFT: c_uint; static NAU8825_PRECHARGE_DIS: c_uint; static NAU8825_HP_BOOST_G_DIS: c_uint; static NAU8825_SHORT_SHUTDOWN_EN: c_uint; static NAU8825_GLOBAL_BIAS_EN: c_uint;
    static NAU8825_JKDET_OUTPUT_EN: c_uint; static NAU8825_JKDET_PULL_EN: c_uint; static NAU8825_JKDET_PULL_UP: c_uint; static NAU8825_JACK_INSERT_DEBOUNCE_MASK: c_uint; static NAU8825_JACK_INSERT_DEBOUNCE_SFT: c_uint; static NAU8825_JACK_EJECT_DEBOUNCE_MASK: c_uint; static NAU8825_JACK_EJECT_DEBOUNCE_SFT: c_uint;
    static NAU8825_IRQ_PIN_PULLUP: c_uint; static NAU8825_IRQ_PIN_PULL_EN: c_uint; static NAU8825_ADC_SYNC_DOWN_64: c_uint; static NAU8825_ADC_SINC4_EN: c_uint; static NAU8825_DAC_OVERSAMPLE_64: c_uint; static NAU8825_DAC_CLIP_OFF: c_uint;
    static NAU8825_HP_NON_CLASSG_CURRENT_2xADJ: c_uint; static NAU8825_DAC_CAPACITOR_MSB: c_uint; static NAU8825_DAC_CAPACITOR_LSB: c_uint; static NAU8825_CLASSG_TIMER_MASK: c_uint; static NAU8825_CLASSG_TIMER_SFT: c_uint;
    static NAU8825_RDAC_CLK_DELAY_MASK: c_uint; static NAU8825_RDAC_VREF_MASK: c_uint; static NAU8825_RDAC_CLK_DELAY_SFT: c_uint; static NAU8825_RDAC_VREF_SFT: c_uint; static NAU8825_DIS_FS_SHORT_DET: c_uint; static NAU8825_ADCOUT_DS_MASK: c_uint; static NAU8825_ADCOUT_DS_SFT: c_uint;
    static NAU8825_FLL_INTEGER_MASK: c_uint; static NAU8825_FLL_REF_DIV_MASK: c_uint; static NAU8825_FLL_REF_DIV_SFT: c_uint; static NAU8825_FLL_CLK_SW_MASK: c_uint; static NAU8825_FLL_CLK_SW_REF: c_uint; static NAU8825_ICTRL_LATCH_MASK: c_uint; static NAU8825_ICTRL_LATCH_SFT: c_uint;
    static NAU8825_FLL_PDB_DAC_EN: c_uint; static NAU8825_FLL_LOOP_FTR_EN: c_uint; static NAU8825_FLL_FTR_SW_MASK: c_uint; static NAU8825_FLL_FTR_SW_FILTER: c_uint; static NAU8825_SDM_EN: c_uint; static NAU8825_CUTOFF500: c_uint; static NAU8825_FLL_FTR_SW_ACCU: c_uint;
    static NAU8825_CLK_SRC_MCLK: c_uint; static NAU8825_CLK_DIS: c_int; static NAU8825_CLK_MCLK: c_int; static NAU8825_CLK_INTERNAL: c_int; static NAU8825_CLK_FLL_MCLK: c_int; static NAU8825_CLK_FLL_BLK: c_int; static NAU8825_CLK_FLL_FS: c_int;
    static NAU8825_FLL_CLK_SRC_MASK: c_uint; static NAU8825_GAIN_ERR_MASK: c_uint; static NAU8825_FLL_CLK_SRC_MCLK: c_uint; static NAU8825_FLL_CLK_SRC_BLK: c_uint; static NAU8825_FLL_CLK_SRC_FS: c_uint; static NAU8825_GAIN_ERR_SFT: c_uint;
}

const fn BIT(n: c_uint) -> c_int {
    (1i32) << n
}
const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}
unsafe fn WARN_ON(cond: bool) -> bool {
    cond
}
unsafe fn IS_ERR<T>(p: *mut T) -> bool {
    (p as isize) < 0
}
unsafe fn PTR_ERR<T>(p: *mut T) -> c_int {
    p as isize as c_int
}
unsafe fn SND_SOC_DAPM_EVENT_OFF(event: c_int) -> bool {
    (event & (SND_SOC_DAPM_POST_PMD | SND_SOC_DAPM_PRE_PMD)) != 0
}
unsafe fn div_u64(n: u64, d: u64) -> u64 {
    n / d
}

static mclk_src_scaling: [nau8825_fll_attr; 13] = [
    nau8825_fll_attr { param: 1, val: 0x0 },
    nau8825_fll_attr { param: 2, val: 0x2 },
    nau8825_fll_attr { param: 4, val: 0x3 },
    nau8825_fll_attr { param: 8, val: 0x4 },
    nau8825_fll_attr { param: 16, val: 0x5 },
    nau8825_fll_attr { param: 32, val: 0x6 },
    nau8825_fll_attr { param: 3, val: 0x7 },
    nau8825_fll_attr { param: 6, val: 0xa },
    nau8825_fll_attr { param: 12, val: 0xb },
    nau8825_fll_attr { param: 24, val: 0xc },
    nau8825_fll_attr { param: 48, val: 0xd },
    nau8825_fll_attr { param: 96, val: 0xe },
    nau8825_fll_attr { param: 5, val: 0xf },
];

static fll_ratio: [nau8825_fll_attr; 7] = [
    nau8825_fll_attr { param: 512000, val: 0x01 },
    nau8825_fll_attr { param: 256000, val: 0x02 },
    nau8825_fll_attr { param: 128000, val: 0x04 },
    nau8825_fll_attr { param: 64000, val: 0x08 },
    nau8825_fll_attr { param: 32000, val: 0x10 },
    nau8825_fll_attr { param: 8000, val: 0x20 },
    nau8825_fll_attr { param: 4000, val: 0x40 },
];

static fll_pre_scalar: [nau8825_fll_attr; 4] = [
    nau8825_fll_attr { param: 1, val: 0x0 },
    nau8825_fll_attr { param: 2, val: 0x1 },
    nau8825_fll_attr { param: 4, val: 0x2 },
    nau8825_fll_attr { param: 8, val: 0x3 },
];

static osr_dac_sel: [nau8825_osr_attr; 5] = [
    nau8825_osr_attr { osr: 64, clk_src: 2 },  /* OSR 64, SRC 1/4 */
    nau8825_osr_attr { osr: 256, clk_src: 0 }, /* OSR 256, SRC 1 */
    nau8825_osr_attr { osr: 128, clk_src: 1 }, /* OSR 128, SRC 1/2 */
    nau8825_osr_attr { osr: 0, clk_src: 0 },
    nau8825_osr_attr { osr: 32, clk_src: 3 },  /* OSR 32, SRC 1/8 */
];

static osr_adc_sel: [nau8825_osr_attr; 4] = [
    nau8825_osr_attr { osr: 32, clk_src: 3 },  /* OSR 32, SRC 1/8 */
    nau8825_osr_attr { osr: 64, clk_src: 2 },  /* OSR 64, SRC 1/4 */
    nau8825_osr_attr { osr: 128, clk_src: 1 }, /* OSR 128, SRC 1/2 */
    nau8825_osr_attr { osr: 256, clk_src: 0 }, /* OSR 256, SRC 1 */
];

/* Register default and patch tables are constructed at runtime because their
 * register names originate in the external nau8825.h dependency.
 */
unsafe fn nau8825_reg_defaults() -> [reg_default; 74] {
    [
        reg_default { reg: NAU8825_REG_ENA_CTRL, def: 0x00ff }, reg_default { reg: NAU8825_REG_IIC_ADDR_SET, def: 0x0 },
        reg_default { reg: NAU8825_REG_CLK_DIVIDER, def: 0x0050 }, reg_default { reg: NAU8825_REG_FLL1, def: 0x0 },
        reg_default { reg: NAU8825_REG_FLL2, def: 0x3126 }, reg_default { reg: NAU8825_REG_FLL3, def: 0x0008 },
        reg_default { reg: NAU8825_REG_FLL4, def: 0x0010 }, reg_default { reg: NAU8825_REG_FLL5, def: 0x0 },
        reg_default { reg: NAU8825_REG_FLL6, def: 0x6000 }, reg_default { reg: NAU8825_REG_FLL_VCO_RSV, def: 0xf13c },
        reg_default { reg: NAU8825_REG_HSD_CTRL, def: 0x000c }, reg_default { reg: NAU8825_REG_JACK_DET_CTRL, def: 0x0 },
        reg_default { reg: NAU8825_REG_INTERRUPT_MASK, def: 0x0 }, reg_default { reg: NAU8825_REG_INTERRUPT_DIS_CTRL, def: 0xffff },
        reg_default { reg: NAU8825_REG_SAR_CTRL, def: 0x0015 }, reg_default { reg: NAU8825_REG_KEYDET_CTRL, def: 0x0110 },
        reg_default { reg: NAU8825_REG_VDET_THRESHOLD_1, def: 0x0 }, reg_default { reg: NAU8825_REG_VDET_THRESHOLD_2, def: 0x0 },
        reg_default { reg: NAU8825_REG_VDET_THRESHOLD_3, def: 0x0 }, reg_default { reg: NAU8825_REG_VDET_THRESHOLD_4, def: 0x0 },
        reg_default { reg: NAU8825_REG_GPIO34_CTRL, def: 0x0 }, reg_default { reg: NAU8825_REG_GPIO12_CTRL, def: 0x0 },
        reg_default { reg: NAU8825_REG_TDM_CTRL, def: 0x0 }, reg_default { reg: NAU8825_REG_I2S_PCM_CTRL1, def: 0x000b },
        reg_default { reg: NAU8825_REG_I2S_PCM_CTRL2, def: 0x8010 }, reg_default { reg: NAU8825_REG_LEFT_TIME_SLOT, def: 0x0 },
        reg_default { reg: NAU8825_REG_RIGHT_TIME_SLOT, def: 0x0 }, reg_default { reg: NAU8825_REG_BIQ_CTRL, def: 0x0 },
        reg_default { reg: NAU8825_REG_BIQ_COF1, def: 0x0 }, reg_default { reg: NAU8825_REG_BIQ_COF2, def: 0x0 },
        reg_default { reg: NAU8825_REG_BIQ_COF3, def: 0x0 }, reg_default { reg: NAU8825_REG_BIQ_COF4, def: 0x0 },
        reg_default { reg: NAU8825_REG_BIQ_COF5, def: 0x0 }, reg_default { reg: NAU8825_REG_BIQ_COF6, def: 0x0 },
        reg_default { reg: NAU8825_REG_BIQ_COF7, def: 0x0 }, reg_default { reg: NAU8825_REG_BIQ_COF8, def: 0x0 },
        reg_default { reg: NAU8825_REG_BIQ_COF9, def: 0x0 }, reg_default { reg: NAU8825_REG_BIQ_COF10, def: 0x0 },
        reg_default { reg: NAU8825_REG_ADC_RATE, def: 0x0010 }, reg_default { reg: NAU8825_REG_DAC_CTRL1, def: 0x0001 },
        reg_default { reg: NAU8825_REG_DAC_CTRL2, def: 0x0 }, reg_default { reg: NAU8825_REG_DAC_DGAIN_CTRL, def: 0x0 },
        reg_default { reg: NAU8825_REG_ADC_DGAIN_CTRL, def: 0x00cf }, reg_default { reg: NAU8825_REG_MUTE_CTRL, def: 0x0 },
        reg_default { reg: NAU8825_REG_HSVOL_CTRL, def: 0x0 }, reg_default { reg: NAU8825_REG_DACL_CTRL, def: 0x02cf },
        reg_default { reg: NAU8825_REG_DACR_CTRL, def: 0x00cf }, reg_default { reg: NAU8825_REG_ADC_DRC_KNEE_IP12, def: 0x1486 },
        reg_default { reg: NAU8825_REG_ADC_DRC_KNEE_IP34, def: 0x0f12 }, reg_default { reg: NAU8825_REG_ADC_DRC_SLOPES, def: 0x25ff },
        reg_default { reg: NAU8825_REG_ADC_DRC_ATKDCY, def: 0x3457 }, reg_default { reg: NAU8825_REG_DAC_DRC_KNEE_IP12, def: 0x1486 },
        reg_default { reg: NAU8825_REG_DAC_DRC_KNEE_IP34, def: 0x0f12 }, reg_default { reg: NAU8825_REG_DAC_DRC_SLOPES, def: 0x25f9 },
        reg_default { reg: NAU8825_REG_DAC_DRC_ATKDCY, def: 0x3457 }, reg_default { reg: NAU8825_REG_IMM_MODE_CTRL, def: 0x0 },
        reg_default { reg: NAU8825_REG_CLASSG_CTRL, def: 0x0 }, reg_default { reg: NAU8825_REG_OPT_EFUSE_CTRL, def: 0x0 },
        reg_default { reg: NAU8825_REG_MISC_CTRL, def: 0x0 }, reg_default { reg: NAU8825_REG_FLL2_LOWER, def: 0x0 },
        reg_default { reg: NAU8825_REG_FLL2_UPPER, def: 0x0 }, reg_default { reg: NAU8825_REG_BIAS_ADJ, def: 0x0 },
        reg_default { reg: NAU8825_REG_TRIM_SETTINGS, def: 0x0 }, reg_default { reg: NAU8825_REG_ANALOG_CONTROL_1, def: 0x0 },
        reg_default { reg: NAU8825_REG_ANALOG_CONTROL_2, def: 0x0 }, reg_default { reg: NAU8825_REG_ANALOG_ADC_1, def: 0x0011 },
        reg_default { reg: NAU8825_REG_ANALOG_ADC_2, def: 0x0020 }, reg_default { reg: NAU8825_REG_RDAC, def: 0x0008 },
        reg_default { reg: NAU8825_REG_MIC_BIAS, def: 0x0006 }, reg_default { reg: NAU8825_REG_BOOST, def: 0x0 },
        reg_default { reg: NAU8825_REG_FEPGA, def: 0x0 }, reg_default { reg: NAU8825_REG_POWER_UP_CONTROL, def: 0x0 },
        reg_default { reg: NAU8825_REG_CHARGE_PUMP, def: 0x0 },
    ]
}

static mut nau8825_xtalk_baktab: [reg_default; 4] = [
    reg_default { reg: 0, def: 0x00cf },
    reg_default { reg: 0, def: 0 },
    reg_default { reg: 0, def: 0x00cf },
    reg_default { reg: 0, def: 0x02cf },
];

unsafe fn init_xtalk_baktab_regs() {
    nau8825_xtalk_baktab[0].reg = NAU8825_REG_ADC_DGAIN_CTRL;
    nau8825_xtalk_baktab[1].reg = NAU8825_REG_HSVOL_CTRL;
    nau8825_xtalk_baktab[2].reg = NAU8825_REG_DACL_CTRL;
    nau8825_xtalk_baktab[3].reg = NAU8825_REG_DACR_CTRL;
}

unsafe fn nau8825_regmap_patch() -> [reg_sequence; 12] {
    [
        reg_sequence { reg: NAU8825_REG_FLL2, def: 0x0000 },
        reg_sequence { reg: NAU8825_REG_FLL4, def: 0x8010 },
        reg_sequence { reg: NAU8825_REG_FLL_VCO_RSV, def: 0x0bc0 },
        reg_sequence { reg: NAU8825_REG_INTERRUPT_MASK, def: 0x0800 },
        reg_sequence { reg: NAU8825_REG_DACL_CTRL, def: 0x00cf },
        reg_sequence { reg: NAU8825_REG_DACR_CTRL, def: 0x02cf },
        reg_sequence { reg: NAU8825_REG_OPT_EFUSE_CTRL, def: 0x0400 },
        reg_sequence { reg: NAU8825_REG_FLL2_LOWER, def: 0x26e9 },
        reg_sequence { reg: NAU8825_REG_FLL2_UPPER, def: 0x0031 },
        reg_sequence { reg: NAU8825_REG_ANALOG_CONTROL_2, def: 0x0020 },
        reg_sequence { reg: NAU8825_REG_ANALOG_ADC_2, def: 0x0220 },
        reg_sequence { reg: NAU8825_REG_MIC_BIAS, def: 0x0046 },
    ]
}

unsafe extern "C" fn nau8825_sema_acquire(nau8825: *mut nau8825, timeout: c_long) -> c_int {
    let ret: c_int;
    if timeout != 0 {
        ret = down_timeout(&mut (*nau8825).xtalk_sem, timeout);
        if ret < 0 {
            dev_warn((*nau8825).dev, c"Acquire semaphore timeout\n".as_ptr());
        }
    } else {
        ret = down_trylock(&mut (*nau8825).xtalk_sem);
        if ret != 0 {
            dev_warn((*nau8825).dev, c"Acquire semaphore fail\n".as_ptr());
        }
    }
    ret
}

unsafe fn nau8825_sema_release(nau8825: *mut nau8825) {
    up(&mut (*nau8825).xtalk_sem);
}

unsafe fn nau8825_sema_reset(nau8825: *mut nau8825) {
    (*nau8825).xtalk_sem.count = 1;
}

unsafe fn nau8825_hpvol_ramp(nau8825: *mut nau8825, vol_from: c_uint, vol_to: c_uint, step: c_uint) {
    let mut value: c_uint;
    let mut volume: c_uint;
    let ramp_up: bool;
    let from: c_uint;
    let mut to: c_uint;

    if vol_from == vol_to || step == 0 {
        return;
    } else if vol_from < vol_to {
        ramp_up = true;
        from = vol_from;
        to = vol_to;
    } else {
        ramp_up = false;
        from = vol_to;
        to = vol_from;
    }
    if to > NAU8825_HP_VOL_MIN {
        to = NAU8825_HP_VOL_MIN;
    }

    volume = from;
    while volume < to {
        if ramp_up {
            value = volume;
        } else {
            value = to - volume + from;
        }
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_HSVOL_CTRL,
                           NAU8825_HPL_VOL_MASK | NAU8825_HPR_VOL_MASK,
                           (value << NAU8825_HPL_VOL_SFT) | value);
        usleep_range(10000, 10500);
        volume = volume.wrapping_add(step);
    }
    if ramp_up {
        value = to;
    } else {
        value = from;
    }
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_HSVOL_CTRL,
                       NAU8825_HPL_VOL_MASK | NAU8825_HPR_VOL_MASK,
                       (value << NAU8825_HPL_VOL_SFT) | value);
}

unsafe fn nau8825_intlog10_dec3(value: u32) -> u32 {
    intlog10(value) / (((1u32) << 24) / 1000)
}

unsafe fn nau8825_xtalk_sidetone(mut sig_org: u32, mut sig_cros: u32) -> u32 {
    let gain: u32;
    let mut sidetone: u32;

    if WARN_ON(sig_org == 0 || sig_cros == 0) {
        return 0;
    }

    sig_org = nau8825_intlog10_dec3(sig_org);
    sig_cros = nau8825_intlog10_dec3(sig_cros);
    if sig_org >= sig_cros {
        gain = (sig_org - sig_cros) * 20 + GAIN_AUGMENT;
    } else {
        gain = (sig_cros - sig_org) * 20 + GAIN_AUGMENT;
    }
    sidetone = SIDETONE_BASE - gain * 2;
    sidetone /= 1000;
    sidetone
}

unsafe fn nau8825_xtalk_baktab_index_by_reg(reg: c_uint) -> c_int {
    init_xtalk_baktab_regs();
    for index in 0..nau8825_xtalk_baktab.len() {
        if nau8825_xtalk_baktab[index].reg == reg {
            return index as c_int;
        }
    }
    -EINVAL
}

unsafe fn nau8825_xtalk_backup(nau8825: *mut nau8825) {
    init_xtalk_baktab_regs();
    if (*nau8825).xtalk_baktab_initialized {
        return;
    }
    for i in 0..nau8825_xtalk_baktab.len() {
        regmap_read((*nau8825).regmap, nau8825_xtalk_baktab[i].reg, &mut nau8825_xtalk_baktab[i].def);
    }
    (*nau8825).xtalk_baktab_initialized = true;
}

unsafe fn nau8825_xtalk_restore(nau8825: *mut nau8825, cause_cancel: bool) {
    init_xtalk_baktab_regs();
    if !(*nau8825).xtalk_baktab_initialized {
        return;
    }
    for i in 0..nau8825_xtalk_baktab.len() {
        if !cause_cancel && nau8825_xtalk_baktab[i].reg == NAU8825_REG_HSVOL_CTRL {
            let volume = nau8825_xtalk_baktab[i].def & NAU8825_HPR_VOL_MASK;
            nau8825_hpvol_ramp(nau8825, 0, volume, 3);
            continue;
        }
        regmap_write((*nau8825).regmap, nau8825_xtalk_baktab[i].reg, nau8825_xtalk_baktab[i].def);
    }
    (*nau8825).xtalk_baktab_initialized = false;
}

unsafe fn nau8825_xtalk_prepare_dac(nau8825: *mut nau8825) {
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_ENA_CTRL,
        NAU8825_ENABLE_DACR | NAU8825_ENABLE_DACL | NAU8825_ENABLE_ADC | NAU8825_ENABLE_ADC_CLK | NAU8825_ENABLE_DAC_CLK,
        NAU8825_ENABLE_DACR | NAU8825_ENABLE_DACL | NAU8825_ENABLE_ADC | NAU8825_ENABLE_ADC_CLK | NAU8825_ENABLE_DAC_CLK);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_CHARGE_PUMP,
        NAU8825_JAMNODCLOW | NAU8825_CHANRGE_PUMP_EN, NAU8825_JAMNODCLOW | NAU8825_CHANRGE_PUMP_EN);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_RDAC,
        NAU8825_RDAC_EN | NAU8825_RDAC_CLK_EN | NAU8825_RDAC_FS_BCLK_ENB, NAU8825_RDAC_EN | NAU8825_RDAC_CLK_EN);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_POWER_UP_CONTROL,
        NAU8825_POWERUP_INTEGR_R | NAU8825_POWERUP_INTEGR_L | NAU8825_POWERUP_DRV_IN_R | NAU8825_POWERUP_DRV_IN_L,
        NAU8825_POWERUP_INTEGR_R | NAU8825_POWERUP_INTEGR_L | NAU8825_POWERUP_DRV_IN_R | NAU8825_POWERUP_DRV_IN_L);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_POWER_UP_CONTROL,
        NAU8825_POWERUP_HP_DRV_R | NAU8825_POWERUP_HP_DRV_L, NAU8825_POWERUP_HP_DRV_R | NAU8825_POWERUP_HP_DRV_L);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_HSD_CTRL, NAU8825_SPKR_DWN1R | NAU8825_SPKR_DWN1L, 0);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_BOOST, NAU8825_HP_BOOST_DIS, NAU8825_HP_BOOST_DIS);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_CLASSG_CTRL,
        NAU8825_CLASSG_LDAC_EN | NAU8825_CLASSG_RDAC_EN, NAU8825_CLASSG_LDAC_EN | NAU8825_CLASSG_RDAC_EN);
}

unsafe fn nau8825_xtalk_prepare_adc(nau8825: *mut nau8825) {
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_ANALOG_ADC_2,
        NAU8825_POWERUP_ADCL | NAU8825_ADC_VREFSEL_MASK, NAU8825_POWERUP_ADCL | NAU8825_ADC_VREFSEL_VMID_PLUS_0_5DB);
}

unsafe fn nau8825_xtalk_clock(nau8825: *mut nau8825) {
    regmap_write((*nau8825).regmap, NAU8825_REG_FLL1, 0x0);
    regmap_write((*nau8825).regmap, NAU8825_REG_FLL2, 0x3126);
    regmap_write((*nau8825).regmap, NAU8825_REG_FLL3, 0x0008);
    regmap_write((*nau8825).regmap, NAU8825_REG_FLL4, 0x0010);
    regmap_write((*nau8825).regmap, NAU8825_REG_FLL5, 0x0);
    regmap_write((*nau8825).regmap, NAU8825_REG_FLL6, 0x6000);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_CLK_DIVIDER, NAU8825_CLK_SRC_MASK, NAU8825_CLK_SRC_VCO);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_FLL6, NAU8825_DCO_EN, NAU8825_DCO_EN);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_CLK_DIVIDER, NAU8825_CLK_MCLK_SRC_MASK, 0xf);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_FLL1, NAU8825_FLL_RATIO_MASK, 0x10);
}

unsafe fn nau8825_xtalk_prepare(nau8825: *mut nau8825) {
    nau8825_xtalk_backup(nau8825);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_I2S_PCM_CTRL2,
        NAU8825_I2S_MS_MASK | NAU8825_I2S_LRC_DIV_MASK | NAU8825_I2S_BLK_DIV_MASK,
        NAU8825_I2S_MS_MASTER | (0x2 << NAU8825_I2S_LRC_DIV_SFT) | 0x1);
    let index = nau8825_xtalk_baktab_index_by_reg(NAU8825_REG_HSVOL_CTRL);
    if index != -EINVAL {
        let volume = nau8825_xtalk_baktab[index as usize].def & NAU8825_HPR_VOL_MASK;
        nau8825_hpvol_ramp(nau8825, volume, 0, 3);
    }
    nau8825_xtalk_clock(nau8825);
    nau8825_xtalk_prepare_dac(nau8825);
    nau8825_xtalk_prepare_adc(nau8825);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_DACL_CTRL, NAU8825_DACL_CH_SEL_MASK | NAU8825_DACL_CH_VOL_MASK, NAU8825_DACL_CH_SEL_L | 0xab);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_DACR_CTRL, NAU8825_DACR_CH_SEL_MASK | NAU8825_DACR_CH_VOL_MASK, NAU8825_DACR_CH_SEL_R | 0xab);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_IMM_MODE_CTRL,
        NAU8825_IMM_THD_MASK | NAU8825_IMM_GEN_VOL_MASK | NAU8825_IMM_CYC_MASK | NAU8825_IMM_DAC_SRC_MASK,
        (0x9 << NAU8825_IMM_THD_SFT) | NAU8825_IMM_GEN_VOL_1_16th | NAU8825_IMM_CYC_8192 | NAU8825_IMM_DAC_SRC_SIN);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_INTERRUPT_MASK, NAU8825_IRQ_RMS_EN, 0);
    if (*nau8825).sw_id == NAU8825_SOFTWARE_ID_NAU8825 {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL, 0);
    } else {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL);
    }
}

unsafe fn nau8825_xtalk_clean_dac(nau8825: *mut nau8825) {
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_BOOST, NAU8825_HP_BOOST_DIS, 0);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_HSD_CTRL, NAU8825_SPKR_DWN1R | NAU8825_SPKR_DWN1L, NAU8825_SPKR_DWN1R | NAU8825_SPKR_DWN1L);
    if (*nau8825).sw_id == NAU8825_SOFTWARE_ID_NAU8825 {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL);
    } else {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL, 0);
    }
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_BIAS_ADJ, NAU8825_BIAS_HPR_IMP | NAU8825_BIAS_HPL_IMP | NAU8825_BIAS_TESTDAC_EN, NAU8825_BIAS_TESTDAC_EN);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_POWER_UP_CONTROL, NAU8825_POWERUP_HP_DRV_R | NAU8825_POWERUP_HP_DRV_L, 0);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_POWER_UP_CONTROL, NAU8825_POWERUP_INTEGR_R | NAU8825_POWERUP_INTEGR_L | NAU8825_POWERUP_DRV_IN_R | NAU8825_POWERUP_DRV_IN_L, 0);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_RDAC, NAU8825_RDAC_EN | NAU8825_RDAC_CLK_EN, 0);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_JAMNODCLOW | NAU8825_CHANRGE_PUMP_EN, 0);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_ENA_CTRL, NAU8825_ENABLE_DACR | NAU8825_ENABLE_DACL | NAU8825_ENABLE_ADC_CLK | NAU8825_ENABLE_DAC_CLK, 0);
    if (*nau8825).irq == 0 {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_ENA_CTRL, NAU8825_ENABLE_ADC, 0);
    }
}

unsafe fn nau8825_xtalk_clean_adc(nau8825: *mut nau8825) {
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_ANALOG_ADC_2, NAU8825_POWERUP_ADCL | NAU8825_ADC_VREFSEL_MASK, 0);
}

unsafe extern "C" fn nau8825_configure_sysclk(nau8825: *mut nau8825, clk_id: c_int, freq: c_uint) -> c_int;

unsafe fn nau8825_xtalk_clean(nau8825: *mut nau8825, cause_cancel: bool) {
    nau8825_configure_sysclk(nau8825, NAU8825_CLK_INTERNAL, 0);
    nau8825_xtalk_clean_dac(nau8825);
    nau8825_xtalk_clean_adc(nau8825);
    regmap_write((*nau8825).regmap, NAU8825_REG_IMM_MODE_CTRL, 0);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_INTERRUPT_MASK, NAU8825_IRQ_RMS_EN, NAU8825_IRQ_RMS_EN);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_I2S_PCM_CTRL2, NAU8825_I2S_MS_MASK | NAU8825_I2S_LRC_DIV_MASK | NAU8825_I2S_BLK_DIV_MASK, NAU8825_I2S_MS_SLAVE);
    nau8825_xtalk_restore(nau8825, cause_cancel);
}

unsafe fn nau8825_xtalk_imm_start(nau8825: *mut nau8825, vol: c_int) {
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_ADC_DGAIN_CTRL, NAU8825_ADC_DIG_VOL_MASK, vol as c_uint);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_BIAS_ADJ, NAU8825_BIAS_TESTDACR_EN | NAU8825_BIAS_TESTDACL_EN, NAU8825_BIAS_TESTDACL_EN);
    if (*nau8825).xtalk_state == NAU8825_XTALK_HPR_R2L {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_BIAS_ADJ, NAU8825_BIAS_HPR_IMP | NAU8825_BIAS_HPL_IMP, NAU8825_BIAS_HPR_IMP);
    } else if (*nau8825).xtalk_state == NAU8825_XTALK_HPL_R2L {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_BIAS_ADJ, NAU8825_BIAS_HPR_IMP | NAU8825_BIAS_HPL_IMP, NAU8825_BIAS_HPL_IMP);
    }
    msleep(100);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_IMM_MODE_CTRL, NAU8825_IMM_EN, NAU8825_IMM_EN);
}

unsafe fn nau8825_xtalk_imm_stop(nau8825: *mut nau8825) {
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_IMM_MODE_CTRL, NAU8825_IMM_EN, 0);
}

unsafe fn nau8825_xtalk_measure(nau8825: *mut nau8825) {
    if (*nau8825).xtalk_state == NAU8825_XTALK_PREPARE {
        nau8825_xtalk_prepare(nau8825);
        msleep(280);
        (*nau8825).xtalk_state = NAU8825_XTALK_HPR_R2L;
        nau8825_xtalk_imm_start(nau8825, 0x00d2);
    } else if (*nau8825).xtalk_state == NAU8825_XTALK_HPR_R2L {
        regmap_read((*nau8825).regmap, NAU8825_REG_IMM_RMS_L, &mut (*nau8825).imp_rms[NAU8825_XTALK_HPR_R2L as usize]);
        dev_dbg((*nau8825).dev, c"HPR_R2L imm: %x\n".as_ptr(), (*nau8825).imp_rms[NAU8825_XTALK_HPR_R2L as usize]);
        nau8825_xtalk_imm_stop(nau8825);
        (*nau8825).xtalk_state = NAU8825_XTALK_HPL_R2L;
        nau8825_xtalk_imm_start(nau8825, 0x00ff);
    } else if (*nau8825).xtalk_state == NAU8825_XTALK_HPL_R2L {
        regmap_read((*nau8825).regmap, NAU8825_REG_IMM_RMS_L, &mut (*nau8825).imp_rms[NAU8825_XTALK_HPL_R2L as usize]);
        dev_dbg((*nau8825).dev, c"HPL_R2L imm: %x\n".as_ptr(), (*nau8825).imp_rms[NAU8825_XTALK_HPL_R2L as usize]);
        nau8825_xtalk_imm_stop(nau8825);
        msleep(150);
        (*nau8825).xtalk_state = NAU8825_XTALK_IMM;
    } else if (*nau8825).xtalk_state == NAU8825_XTALK_IMM {
        let sidetone = nau8825_xtalk_sidetone((*nau8825).imp_rms[NAU8825_XTALK_HPR_R2L as usize], (*nau8825).imp_rms[NAU8825_XTALK_HPL_R2L as usize]);
        dev_dbg((*nau8825).dev, c"cross talk sidetone: %x\n".as_ptr(), sidetone);
        regmap_write((*nau8825).regmap, NAU8825_REG_DAC_DGAIN_CTRL, (sidetone << 8) | sidetone);
        nau8825_xtalk_clean(nau8825, false);
        (*nau8825).xtalk_state = NAU8825_XTALK_DONE;
    }
}

unsafe extern "C" fn nau8825_xtalk_work(work: *mut work_struct) {
    let nau8825 = (work as *mut u8).sub(core::mem::offset_of!(nau8825, xtalk_work)) as *mut nau8825;
    nau8825_xtalk_measure(nau8825);
    if (*nau8825).xtalk_state == NAU8825_XTALK_IMM {
        nau8825_xtalk_measure(nau8825);
    }
    if (*nau8825).xtalk_state == NAU8825_XTALK_DONE {
        snd_soc_jack_report((*nau8825).jack, (*nau8825).xtalk_event, (*nau8825).xtalk_event_mask);
        nau8825_sema_release(nau8825);
        (*nau8825).xtalk_protect = false;
    }
}

unsafe fn nau8825_xtalk_cancel(nau8825: *mut nau8825) {
    if (*nau8825).xtalk_enable && (*nau8825).xtalk_state != NAU8825_XTALK_DONE {
        cancel_work_sync(&mut (*nau8825).xtalk_work);
        nau8825_xtalk_clean(nau8825, true);
    }
    nau8825_sema_reset(nau8825);
    (*nau8825).xtalk_state = NAU8825_XTALK_DONE;
    (*nau8825).xtalk_protect = false;
}

unsafe extern "C" fn nau8825_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    (reg >= NAU8825_REG_ENA_CTRL && reg <= NAU8825_REG_FLL_VCO_RSV) ||
    (reg >= NAU8825_REG_HSD_CTRL && reg <= NAU8825_REG_JACK_DET_CTRL) ||
    (reg >= NAU8825_REG_INTERRUPT_MASK && reg <= NAU8825_REG_KEYDET_CTRL) ||
    (reg >= NAU8825_REG_VDET_THRESHOLD_1 && reg <= NAU8825_REG_DACR_CTRL) ||
    (reg >= NAU8825_REG_ADC_DRC_KNEE_IP12 && reg <= NAU8825_REG_ADC_DRC_ATKDCY) ||
    (reg >= NAU8825_REG_DAC_DRC_KNEE_IP12 && reg <= NAU8825_REG_DAC_DRC_ATKDCY) ||
    (reg >= NAU8825_REG_IMM_MODE_CTRL && reg <= NAU8825_REG_IMM_RMS_R) ||
    (reg >= NAU8825_REG_CLASSG_CTRL && reg <= NAU8825_REG_OPT_EFUSE_CTRL) ||
    reg == NAU8825_REG_MISC_CTRL ||
    (reg >= NAU8825_REG_I2C_DEVICE_ID && reg <= NAU8825_REG_FLL2_UPPER) ||
    reg == NAU8825_REG_BIAS_ADJ ||
    (reg >= NAU8825_REG_TRIM_SETTINGS && reg <= NAU8825_REG_ANALOG_CONTROL_2) ||
    (reg >= NAU8825_REG_ANALOG_ADC_1 && reg <= NAU8825_REG_MIC_BIAS) ||
    (reg >= NAU8825_REG_BOOST && reg <= NAU8825_REG_FEPGA) ||
    (reg >= NAU8825_REG_POWER_UP_CONTROL && reg <= NAU8825_REG_GENERAL_STATUS)
}

unsafe extern "C" fn nau8825_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    (reg >= NAU8825_REG_RESET && reg <= NAU8825_REG_FLL_VCO_RSV) ||
    (reg >= NAU8825_REG_HSD_CTRL && reg <= NAU8825_REG_JACK_DET_CTRL) ||
    reg == NAU8825_REG_INTERRUPT_MASK ||
    (reg >= NAU8825_REG_INT_CLR_KEY_STATUS && reg <= NAU8825_REG_KEYDET_CTRL) ||
    (reg >= NAU8825_REG_VDET_THRESHOLD_1 && reg <= NAU8825_REG_DACR_CTRL) ||
    (reg >= NAU8825_REG_ADC_DRC_KNEE_IP12 && reg <= NAU8825_REG_ADC_DRC_ATKDCY) ||
    (reg >= NAU8825_REG_DAC_DRC_KNEE_IP12 && reg <= NAU8825_REG_DAC_DRC_ATKDCY) ||
    reg == NAU8825_REG_IMM_MODE_CTRL ||
    (reg >= NAU8825_REG_CLASSG_CTRL && reg <= NAU8825_REG_OPT_EFUSE_CTRL) ||
    reg == NAU8825_REG_MISC_CTRL ||
    (reg >= NAU8825_REG_FLL2_LOWER && reg <= NAU8825_REG_FLL2_UPPER) ||
    reg == NAU8825_REG_BIAS_ADJ ||
    (reg >= NAU8825_REG_TRIM_SETTINGS && reg <= NAU8825_REG_ANALOG_CONTROL_2) ||
    (reg >= NAU8825_REG_ANALOG_ADC_1 && reg <= NAU8825_REG_MIC_BIAS) ||
    (reg >= NAU8825_REG_BOOST && reg <= NAU8825_REG_FEPGA) ||
    (reg >= NAU8825_REG_POWER_UP_CONTROL && reg <= NAU8825_REG_CHARGE_PUMP)
}

unsafe extern "C" fn nau8825_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == NAU8825_REG_RESET || reg == NAU8825_REG_IRQ_STATUS ||
    reg == NAU8825_REG_INT_CLR_KEY_STATUS || reg == NAU8825_REG_IMM_RMS_L ||
    reg == NAU8825_REG_IMM_RMS_R || reg == NAU8825_REG_I2C_DEVICE_ID ||
    reg == NAU8825_REG_SARDOUT_RAM_STATUS || reg == NAU8825_REG_CHARGE_PUMP_INPUT_READ ||
    reg == NAU8825_REG_GENERAL_STATUS || (reg >= NAU8825_REG_BIQ_CTRL && reg <= NAU8825_REG_BIQ_COF10)
}

unsafe extern "C" fn nau8825_fepga_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8825 = snd_soc_component_get_drvdata(component);
    if event == SND_SOC_DAPM_POST_PMU {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_FEPGA, NAU8825_ACDC_CTRL_MASK, NAU8825_ACDC_VREF_MICP | NAU8825_ACDC_VREF_MICN);
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_BOOST, NAU8825_DISCHRG_EN, NAU8825_DISCHRG_EN);
        msleep(40);
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_BOOST, NAU8825_DISCHRG_EN, 0);
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_FEPGA, NAU8825_ACDC_CTRL_MASK, 0);
    }
    0
}

unsafe extern "C" fn nau8825_adc_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8825 = snd_soc_component_get_drvdata(component);
    if event == SND_SOC_DAPM_POST_PMU {
        msleep((*nau8825).adc_delay);
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_ENA_CTRL, NAU8825_ENABLE_ADC, NAU8825_ENABLE_ADC);
    } else if event == SND_SOC_DAPM_POST_PMD {
        if (*nau8825).irq == 0 {
            regmap_update_bits((*nau8825).regmap, NAU8825_REG_ENA_CTRL, NAU8825_ENABLE_ADC, 0);
        }
    } else {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn nau8825_pump_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8825 = snd_soc_component_get_drvdata(component);
    if event == SND_SOC_DAPM_POST_PMU {
        msleep(10);
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_JAMNODCLOW, NAU8825_JAMNODCLOW);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_JAMNODCLOW, 0);
    } else {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn nau8825_output_dac_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8825 = snd_soc_component_get_drvdata(component);
    if event == SND_SOC_DAPM_PRE_PMU {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_BIAS_ADJ, NAU8825_BIAS_TESTDAC_EN, 0);
        if (*nau8825).sw_id == NAU8825_SOFTWARE_ID_NAU8825 {
            regmap_update_bits((*nau8825).regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL, 0);
        } else {
            regmap_update_bits((*nau8825).regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL);
        }
    } else if event == SND_SOC_DAPM_POST_PMD {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_BIAS_ADJ, NAU8825_BIAS_TESTDAC_EN, NAU8825_BIAS_TESTDAC_EN);
        if (*nau8825).sw_id == NAU8825_SOFTWARE_ID_NAU8825 {
            regmap_update_bits((*nau8825).regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL);
        } else {
            regmap_update_bits((*nau8825).regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL, 0);
        }
    } else {
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn system_clock_control(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8825 = snd_soc_component_get_drvdata(component);
    let regmap = (*nau8825).regmap;
    if SND_SOC_DAPM_EVENT_OFF(event) {
        dev_dbg((*nau8825).dev, c"system clock control : POWER OFF\n".as_ptr());
        if nau8825_is_jack_inserted(regmap) {
            nau8825_configure_sysclk(nau8825, NAU8825_CLK_INTERNAL, 0);
        } else {
            nau8825_configure_sysclk(nau8825, NAU8825_CLK_DIS, 0);
        }
    }
    0
}

unsafe extern "C" fn nau8825_biq_coeff_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let params = (*kcontrol).private_value as *mut soc_bytes_ext;
    if (*component).regmap.is_null() {
        return -EINVAL;
    }
    regmap_raw_read((*component).regmap, NAU8825_REG_BIQ_COF1, (*ucontrol).value.bytes.data, (*params).max);
    0
}

unsafe extern "C" fn nau8825_biq_coeff_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let params = (*kcontrol).private_value as *mut soc_bytes_ext;
    if (*component).regmap.is_null() {
        return -EINVAL;
    }
    let data = kmemdup((*ucontrol).value.bytes.data, (*params).max, GFP_KERNEL | GFP_DMA);
    if data.is_null() {
        return -ENOMEM;
    }
    regmap_update_bits((*component).regmap, NAU8825_REG_BIQ_CTRL, NAU8825_BIQ_WRT_EN, 0);
    regmap_raw_write((*component).regmap, NAU8825_REG_BIQ_COF1, data, (*params).max);
    regmap_update_bits((*component).regmap, NAU8825_REG_BIQ_CTRL, NAU8825_BIQ_WRT_EN, NAU8825_BIQ_WRT_EN);
    kfree(data);
    0
}

/* The following ASoC control/widget declarations translate C macro-generated
 * static initializers. Their concrete values are supplied by the ASoC binding
 * layer, so the isolated translation keeps the source-level declarations as
 * zero-sized placeholders.
 */
static nau8825_biq_path: [*const c_char; 2] = [c"ADC".as_ptr(), c"DAC".as_ptr()];
static nau8825_adc_decimation: [*const c_char; 4] = [c"32".as_ptr(), c"64".as_ptr(), c"128".as_ptr(), c"256".as_ptr()];
static nau8825_dac_oversampl: [*const c_char; 5] = [c"64".as_ptr(), c"256".as_ptr(), c"128".as_ptr(), c"".as_ptr(), c"32".as_ptr()];
static nau8825_dac_src: [*const c_char; 2] = [c"DACL".as_ptr(), c"DACR".as_ptr()];
static nau8825_controls: [snd_kcontrol_new; 9] = [snd_kcontrol_new { _private: [] }; 9];
static nau8825_dacl_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static nau8825_dacr_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static nau8825_dapm_widgets: [snd_soc_dapm_widget; 43] = [snd_soc_dapm_widget { dapm: ptr::null_mut() }; 43];
static nau8825_dapm_routes: [snd_soc_dapm_route; 39] = [
    snd_soc_dapm_route { sink: c"Frontend PGA".as_ptr(), control: ptr::null(), source: c"MIC".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: ptr::null(), source: c"Frontend PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: ptr::null(), source: c"ADC Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: ptr::null(), source: c"ADC Power".as_ptr() },
    snd_soc_dapm_route { sink: c"AIFTX".as_ptr(), control: ptr::null(), source: c"ADC".as_ptr() },
    snd_soc_dapm_route { sink: c"AIFTX".as_ptr(), control: ptr::null(), source: c"System Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"AIFRX".as_ptr(), control: ptr::null(), source: c"System Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"DDACL".as_ptr(), control: ptr::null(), source: c"AIFRX".as_ptr() },
    snd_soc_dapm_route { sink: c"DDACR".as_ptr(), control: ptr::null(), source: c"AIFRX".as_ptr() },
    snd_soc_dapm_route { sink: c"DDACL".as_ptr(), control: ptr::null(), source: c"DDAC Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"DDACR".as_ptr(), control: ptr::null(), source: c"DDAC Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"DACL Mux".as_ptr(), control: c"DACL".as_ptr(), source: c"DDACL".as_ptr() },
    snd_soc_dapm_route { sink: c"DACL Mux".as_ptr(), control: c"DACR".as_ptr(), source: c"DDACR".as_ptr() },
    snd_soc_dapm_route { sink: c"DACR Mux".as_ptr(), control: c"DACL".as_ptr(), source: c"DDACL".as_ptr() },
    snd_soc_dapm_route { sink: c"DACR Mux".as_ptr(), control: c"DACR".as_ptr(), source: c"DDACR".as_ptr() },
    snd_soc_dapm_route { sink: c"HP amp L".as_ptr(), control: ptr::null(), source: c"DACL Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"HP amp R".as_ptr(), control: ptr::null(), source: c"DACR Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"Charge Pump".as_ptr(), control: ptr::null(), source: c"HP amp L".as_ptr() },
    snd_soc_dapm_route { sink: c"Charge Pump".as_ptr(), control: ptr::null(), source: c"HP amp R".as_ptr() },
    snd_soc_dapm_route { sink: c"ADACL".as_ptr(), control: ptr::null(), source: c"Charge Pump".as_ptr() },
    snd_soc_dapm_route { sink: c"ADACR".as_ptr(), control: ptr::null(), source: c"Charge Pump".as_ptr() },
    snd_soc_dapm_route { sink: c"ADACL Clock".as_ptr(), control: ptr::null(), source: c"ADACL".as_ptr() },
    snd_soc_dapm_route { sink: c"ADACR Clock".as_ptr(), control: ptr::null(), source: c"ADACR".as_ptr() },
    snd_soc_dapm_route { sink: c"Output Driver L Stage 1".as_ptr(), control: ptr::null(), source: c"ADACL Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"Output Driver R Stage 1".as_ptr(), control: ptr::null(), source: c"ADACR Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"Output Driver L Stage 2".as_ptr(), control: ptr::null(), source: c"Output Driver L Stage 1".as_ptr() },
    snd_soc_dapm_route { sink: c"Output Driver R Stage 2".as_ptr(), control: ptr::null(), source: c"Output Driver R Stage 1".as_ptr() },
    snd_soc_dapm_route { sink: c"Output Driver L Stage 3".as_ptr(), control: ptr::null(), source: c"Output Driver L Stage 2".as_ptr() },
    snd_soc_dapm_route { sink: c"Output Driver R Stage 3".as_ptr(), control: ptr::null(), source: c"Output Driver R Stage 2".as_ptr() },
    snd_soc_dapm_route { sink: c"Output DACL".as_ptr(), control: ptr::null(), source: c"Output Driver L Stage 3".as_ptr() },
    snd_soc_dapm_route { sink: c"Output DACR".as_ptr(), control: ptr::null(), source: c"Output Driver R Stage 3".as_ptr() },
    snd_soc_dapm_route { sink: c"HPOL Pulldown".as_ptr(), control: ptr::null(), source: c"Output DACL".as_ptr() },
    snd_soc_dapm_route { sink: c"HPOR Pulldown".as_ptr(), control: ptr::null(), source: c"Output DACR".as_ptr() },
    snd_soc_dapm_route { sink: c"HP Boost Driver".as_ptr(), control: ptr::null(), source: c"HPOL Pulldown".as_ptr() },
    snd_soc_dapm_route { sink: c"HP Boost Driver".as_ptr(), control: ptr::null(), source: c"HPOR Pulldown".as_ptr() },
    snd_soc_dapm_route { sink: c"Class G".as_ptr(), control: ptr::null(), source: c"HP Boost Driver".as_ptr() },
    snd_soc_dapm_route { sink: c"HPOL".as_ptr(), control: ptr::null(), source: c"Class G".as_ptr() },
    snd_soc_dapm_route { sink: c"HPOR".as_ptr(), control: ptr::null(), source: c"Class G".as_ptr() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
];

unsafe fn nau8825_get_osr(nau8825: *mut nau8825, stream: c_int) -> *const nau8825_osr_attr {
    let mut osr: c_uint = 0;
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_read((*nau8825).regmap, NAU8825_REG_DAC_CTRL1, &mut osr);
        osr &= NAU8825_DAC_OVERSAMPLE_MASK;
        if osr >= osr_dac_sel.len() as c_uint { return ptr::null(); }
        &osr_dac_sel[osr as usize]
    } else {
        regmap_read((*nau8825).regmap, NAU8825_REG_ADC_RATE, &mut osr);
        osr &= NAU8825_ADC_SYNC_DOWN_MASK;
        if osr >= osr_adc_sel.len() as c_uint { return ptr::null(); }
        &osr_adc_sel[osr as usize]
    }
}

unsafe extern "C" fn nau8825_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let nau8825 = snd_soc_component_get_drvdata((*dai).component);
    let osr = nau8825_get_osr(nau8825, (*substream).stream);
    if osr.is_null() || (*osr).osr == 0 { return -EINVAL; }
    snd_pcm_hw_constraint_minmax((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 0, CLK_DA_AD_MAX / (*osr).osr)
}

unsafe extern "C" fn nau8825_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let nau8825 = snd_soc_component_get_drvdata((*dai).component);
    let mut val_len: c_uint = 0;
    let mut ctrl_val: c_uint = 0;
    let mut err: c_int = -EINVAL;
    nau8825_sema_acquire(nau8825, 3 * HZ);
    let osr = nau8825_get_osr(nau8825, (*substream).stream);
    if !osr.is_null() && (*osr).osr != 0 && params_rate(params) * (*osr).osr <= CLK_DA_AD_MAX {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            regmap_update_bits((*nau8825).regmap, NAU8825_REG_CLK_DIVIDER, NAU8825_CLK_DAC_SRC_MASK, (*osr).clk_src << NAU8825_CLK_DAC_SRC_SFT);
        } else {
            regmap_update_bits((*nau8825).regmap, NAU8825_REG_CLK_DIVIDER, NAU8825_CLK_ADC_SRC_MASK, (*osr).clk_src << NAU8825_CLK_ADC_SRC_SFT);
        }
        regmap_read((*nau8825).regmap, NAU8825_REG_I2S_PCM_CTRL2, &mut ctrl_val);
        if (ctrl_val & NAU8825_I2S_MS_MASTER) != 0 {
            let bclk_fs = snd_soc_params_to_bclk(params) / params_rate(params);
            let bclk_div = if bclk_fs <= 32 { 2 } else if bclk_fs <= 64 { 1 } else if bclk_fs <= 128 { 0 } else { nau8825_sema_release(nau8825); return -EINVAL; };
            regmap_update_bits((*nau8825).regmap, NAU8825_REG_I2S_PCM_CTRL2, NAU8825_I2S_LRC_DIV_MASK | NAU8825_I2S_BLK_DIV_MASK, ((bclk_div + 1) << NAU8825_I2S_LRC_DIV_SFT) | bclk_div);
        }
        match params_width(params) {
            16 => val_len |= NAU8825_I2S_DL_16,
            20 => val_len |= NAU8825_I2S_DL_20,
            24 => val_len |= NAU8825_I2S_DL_24,
            32 => val_len |= NAU8825_I2S_DL_32,
            _ => {}
        }
        if val_len != 0 {
            regmap_update_bits((*nau8825).regmap, NAU8825_REG_I2S_PCM_CTRL1, NAU8825_I2S_DL_MASK, val_len);
            err = 0;
        }
    }
    nau8825_sema_release(nau8825);
    err
}

unsafe extern "C" fn nau8825_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let nau8825 = snd_soc_component_get_drvdata((*codec_dai).component);
    let mut ctrl1_val: c_uint = 0;
    let mut ctrl2_val: c_uint = 0;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => ctrl2_val |= NAU8825_I2S_MS_MASTER,
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_IB_NF => ctrl1_val |= NAU8825_I2S_BP_INV,
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => ctrl1_val |= NAU8825_I2S_DF_I2S,
        x if x == SND_SOC_DAIFMT_LEFT_J => ctrl1_val |= NAU8825_I2S_DF_LEFT,
        x if x == SND_SOC_DAIFMT_RIGHT_J => ctrl1_val |= NAU8825_I2S_DF_RIGTH,
        x if x == SND_SOC_DAIFMT_DSP_A => ctrl1_val |= NAU8825_I2S_DF_PCM_AB,
        x if x == SND_SOC_DAIFMT_DSP_B => { ctrl1_val |= NAU8825_I2S_DF_PCM_AB; ctrl1_val |= NAU8825_I2S_PCMB_EN; }
        _ => return -EINVAL,
    }
    nau8825_sema_acquire(nau8825, 3 * HZ);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_I2S_PCM_CTRL1, NAU8825_I2S_DL_MASK | NAU8825_I2S_DF_MASK | NAU8825_I2S_BP_MASK | NAU8825_I2S_PCMB_MASK, ctrl1_val);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_I2S_PCM_CTRL2, NAU8825_I2S_MS_MASK, ctrl2_val);
    nau8825_sema_release(nau8825);
    0
}

unsafe extern "C" fn nau8825_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let nau8825 = snd_soc_component_get_drvdata((*dai).component);
    let mut ctrl_val: c_uint = 0;
    let mut ctrl_offset: c_uint = 0;
    let mut value: c_uint = 0;
    let dac_s: c_uint;
    let adc_s: c_int;
    if slots != 4 && slots != 8 {
        dev_err((*nau8825).dev, c"Only support 4 or 8 slots!\n".as_ptr());
        return -EINVAL;
    }
    if hweight_long(tx_mask as c_ulong) != 1 || hweight_long(rx_mask as c_ulong) != 2 {
        dev_err((*nau8825).dev, c"The limitation is 1-channel for ADC, and 2-channel for DAC on TDM mode.\n".as_ptr());
        return -EINVAL;
    }
    if ((tx_mask & 0xf) != 0 && (tx_mask & 0xf0) != 0) ||
       ((rx_mask & 0xf) != 0 && (rx_mask & 0xf0) != 0) ||
       ((tx_mask & 0xf) != 0 && (rx_mask & 0xf0) != 0) ||
       ((rx_mask & 0xf) != 0 && (tx_mask & 0xf0) != 0) {
        dev_err((*nau8825).dev, c"Slot assignment of DAC and ADC need to set same interval.\n".as_ptr());
        return -EINVAL;
    }
    if (rx_mask & 0xf0) != 0 {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_I2S_PCM_CTRL2, NAU8825_I2S_PCM_TS_EN_MASK, NAU8825_I2S_PCM_TS_EN);
        regmap_read((*nau8825).regmap, NAU8825_REG_I2S_PCM_CTRL1, &mut value);
        ctrl_val |= NAU8825_TDM_OFFSET_EN;
        ctrl_offset = (4 * slot_width) as c_uint;
        if (value & NAU8825_I2S_PCMB_MASK) == 0 { ctrl_offset += 1; }
        dac_s = (rx_mask & 0xf0) >> 4;
        adc_s = fls(((tx_mask & 0xf0) >> 4) as c_int);
    } else {
        dac_s = rx_mask & 0xf;
        adc_s = fls((tx_mask & 0xf) as c_int);
    }
    ctrl_val |= NAU8825_TDM_MODE;
    match dac_s {
        0x3 => ctrl_val |= 1 << NAU8825_TDM_DACR_RX_SFT,
        0x5 => ctrl_val |= 2 << NAU8825_TDM_DACR_RX_SFT,
        0x6 => { ctrl_val |= 1 << NAU8825_TDM_DACL_RX_SFT; ctrl_val |= 2 << NAU8825_TDM_DACR_RX_SFT; }
        0x9 => ctrl_val |= 3 << NAU8825_TDM_DACR_RX_SFT,
        0xa => { ctrl_val |= 1 << NAU8825_TDM_DACL_RX_SFT; ctrl_val |= 3 << NAU8825_TDM_DACR_RX_SFT; }
        0xc => { ctrl_val |= 2 << NAU8825_TDM_DACL_RX_SFT; ctrl_val |= 3 << NAU8825_TDM_DACR_RX_SFT; }
        _ => return -EINVAL,
    }
    ctrl_val |= (adc_s - 1) as c_uint;
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_TDM_CTRL, NAU8825_TDM_MODE | NAU8825_TDM_OFFSET_EN | NAU8825_TDM_DACL_RX_MASK | NAU8825_TDM_DACR_RX_MASK | NAU8825_TDM_TX_MASK, ctrl_val);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_LEFT_TIME_SLOT, NAU8825_TSLOT_L0_MASK, ctrl_offset);
    0
}

static nau8825_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(nau8825_dai_startup),
    hw_params: Some(nau8825_hw_params),
    set_fmt: Some(nau8825_set_dai_fmt),
    set_tdm_slot: Some(nau8825_set_tdm_slot),
};

static mut nau8825_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: NUVOTON_CODEC_DAI.as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream { stream_name: c"Playback".as_ptr(), channels_min: 1, channels_max: 2, rates: NAU8825_RATES, formats: NAU8825_FORMATS },
    capture: snd_soc_pcm_stream { stream_name: c"Capture".as_ptr(), channels_min: 1, channels_max: 2, rates: NAU8825_RATES, formats: NAU8825_FORMATS },
    ops: &nau8825_dai_ops,
};

#[no_mangle]
pub unsafe extern "C" fn nau8825_enable_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack) -> c_int {
    let nau8825 = snd_soc_component_get_drvdata(component);
    let regmap = (*nau8825).regmap;
    (*nau8825).jack = jack;
    if (*nau8825).jack.is_null() {
        regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL, NAU8825_HSD_AUTO_MODE | NAU8825_SPKR_DWN1R | NAU8825_SPKR_DWN1L, 0);
        return 0;
    }
    regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL,
        NAU8825_HSD_AUTO_MODE | NAU8825_SPKR_DWN1R | NAU8825_SPKR_DWN1L,
        NAU8825_HSD_AUTO_MODE | NAU8825_SPKR_DWN1R | NAU8825_SPKR_DWN1L);
    0
}

unsafe fn nau8825_is_jack_inserted(regmap: *mut regmap) -> bool {
    let mut status: c_uint = 0;
    let mut jkdet: c_uint = 0;
    regmap_read(regmap, NAU8825_REG_JACK_DET_CTRL, &mut jkdet);
    let active_high = (jkdet & NAU8825_JACK_POLARITY) != 0;
    regmap_read(regmap, NAU8825_REG_I2C_DEVICE_ID, &mut status);
    let is_high = (status & NAU8825_GPIO2JD1) != 0;
    active_high == is_high
}

unsafe fn nau8825_restart_jack_detection(regmap: *mut regmap) {
    regmap_update_bits(regmap, NAU8825_REG_JACK_DET_CTRL, NAU8825_JACK_DET_RESTART, NAU8825_JACK_DET_RESTART);
    regmap_update_bits(regmap, NAU8825_REG_JACK_DET_CTRL, NAU8825_JACK_DET_RESTART, 0);
}

unsafe fn nau8825_int_status_clear_all(regmap: *mut regmap) {
    let mut active_irq: c_uint = 0;
    regmap_read(regmap, NAU8825_REG_IRQ_STATUS, &mut active_irq);
    let mut i: c_uint = 0;
    while i < NAU8825_REG_DATA_LEN {
        let clear_irq = 0x1 << i;
        if (active_irq & clear_irq) != 0 {
            regmap_write(regmap, NAU8825_REG_INT_CLR_KEY_STATUS, clear_irq);
        }
        i += 1;
    }
}

unsafe fn nau8825_eject_jack(nau8825: *mut nau8825) {
    let dapm = (*nau8825).dapm;
    let regmap = (*nau8825).regmap;
    nau8825_xtalk_cancel(nau8825);
    snd_soc_dapm_disable_pin(dapm, c"SAR".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"MICBIAS".as_ptr());
    regmap_update_bits(regmap, NAU8825_REG_MIC_BIAS, NAU8825_MICBIAS_JKSLV | NAU8825_MICBIAS_JKR2, 0);
    regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL, 0xf, 0xf);
    snd_soc_dapm_sync(dapm);
    nau8825_int_status_clear_all(regmap);
    regmap_update_bits(regmap, NAU8825_REG_INTERRUPT_DIS_CTRL, NAU8825_IRQ_EJECT_DIS | NAU8825_IRQ_INSERT_DIS, NAU8825_IRQ_EJECT_DIS);
    regmap_update_bits(regmap, NAU8825_REG_INTERRUPT_MASK, NAU8825_IRQ_OUTPUT_EN | NAU8825_IRQ_EJECT_EN | NAU8825_IRQ_HEADSET_COMPLETE_EN | NAU8825_IRQ_INSERT_EN, NAU8825_IRQ_OUTPUT_EN | NAU8825_IRQ_EJECT_EN | NAU8825_IRQ_HEADSET_COMPLETE_EN);
    regmap_update_bits(regmap, NAU8825_REG_JACK_DET_CTRL, NAU8825_JACK_DET_DB_BYPASS, NAU8825_JACK_DET_DB_BYPASS);
    regmap_update_bits(regmap, NAU8825_REG_ENA_CTRL, NAU8825_ENABLE_ADC, 0);
    nau8825_configure_sysclk(nau8825, NAU8825_CLK_DIS, 0);
}

unsafe fn nau8825_setup_auto_irq(nau8825: *mut nau8825) {
    let regmap = (*nau8825).regmap;
    regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL, NAU8825_HSD_AUTO_MODE, NAU8825_HSD_AUTO_MODE);
    regmap_update_bits(regmap, NAU8825_REG_INTERRUPT_MASK, NAU8825_IRQ_HEADSET_COMPLETE_EN | NAU8825_IRQ_EJECT_EN, 0);
    nau8825_configure_sysclk(nau8825, NAU8825_CLK_INTERNAL, 0);
    regmap_update_bits(regmap, NAU8825_REG_CLK_DIVIDER, NAU8825_CLK_MCLK_SRC_MASK, 0);
    regmap_update_bits(regmap, NAU8825_REG_ENA_CTRL, NAU8825_ENABLE_ADC, NAU8825_ENABLE_ADC);
    regmap_update_bits(regmap, NAU8825_REG_I2S_PCM_CTRL2, NAU8825_I2S_MS_MASK, NAU8825_I2S_MS_MASTER);
    regmap_update_bits(regmap, NAU8825_REG_I2S_PCM_CTRL2, NAU8825_I2S_MS_MASK, NAU8825_I2S_MS_SLAVE);
    regmap_update_bits(regmap, NAU8825_REG_JACK_DET_CTRL, NAU8825_JACK_DET_DB_BYPASS, 0);
    regmap_write(regmap, NAU8825_REG_INTERRUPT_DIS_CTRL, 0);
    nau8825_restart_jack_detection(regmap);
}

unsafe fn nau8825_button_decode(value: c_int) -> c_int {
    let mut buttons = 0;
    if (value & BIT(0)) != 0 { buttons |= SND_JACK_BTN_0; }
    if (value & BIT(1)) != 0 { buttons |= SND_JACK_BTN_1; }
    if (value & BIT(2)) != 0 { buttons |= SND_JACK_BTN_2; }
    if (value & BIT(3)) != 0 { buttons |= SND_JACK_BTN_3; }
    if (value & BIT(4)) != 0 { buttons |= SND_JACK_BTN_4; }
    if (value & BIT(5)) != 0 { buttons |= SND_JACK_BTN_5; }
    buttons
}

unsafe fn nau8825_high_imped_detection(nau8825: *mut nau8825) -> c_int {
    let regmap = (*nau8825).regmap;
    let dapm = (*nau8825).dapm;
    let mut adc_mg1: c_uint = 0;
    let mut adc_mg2: c_uint = 0;
    regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL, NAU8825_SPKR_ENGND1 | NAU8825_SPKR_ENGND2 | NAU8825_SPKR_DWN1R | NAU8825_SPKR_DWN1L, NAU8825_SPKR_ENGND1 | NAU8825_SPKR_ENGND2);
    regmap_update_bits(regmap, NAU8825_REG_ANALOG_CONTROL_1, NAU8825_TESTDACIN_MASK, NAU8825_TESTDACIN_GND);
    regmap_write(regmap, NAU8825_REG_TRIM_SETTINGS, 0x6);
    regmap_update_bits(regmap, NAU8825_REG_MIC_BIAS, NAU8825_MICBIAS_LOWNOISE_MASK | NAU8825_MICBIAS_VOLTAGE_MASK, NAU8825_MICBIAS_LOWNOISE_EN);
    regmap_update_bits(regmap, NAU8825_REG_SAR_CTRL, NAU8825_SAR_INPUT_MASK | NAU8825_SAR_TRACKING_GAIN_MASK | NAU8825_SAR_HV_SEL_MASK | NAU8825_SAR_RES_SEL_MASK | NAU8825_SAR_COMPARE_TIME_MASK | NAU8825_SAR_SAMPLING_TIME_MASK, NAU8825_SAR_HV_SEL_VDDMIC | NAU8825_SAR_RES_SEL_70K);
    snd_soc_dapm_force_enable_pin(dapm, c"MICBIAS".as_ptr());
    snd_soc_dapm_force_enable_pin(dapm, c"SAR".as_ptr());
    snd_soc_dapm_sync(dapm);
    regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL, NAU8825_SPKR_ENGND1 | NAU8825_SPKR_ENGND2 | NAU8825_SPKR_DWN1R | NAU8825_SPKR_DWN1L, NAU8825_SPKR_ENGND2);
    regmap_update_bits(regmap, NAU8825_REG_MIC_BIAS, NAU8825_MICBIAS_JKSLV | NAU8825_MICBIAS_JKR2, NAU8825_MICBIAS_JKR2);
    regmap_read(regmap, NAU8825_REG_SARDOUT_RAM_STATUS, &mut adc_mg1);
    regmap_update_bits(regmap, NAU8825_REG_MIC_BIAS, NAU8825_MICBIAS_JKSLV | NAU8825_MICBIAS_JKR2, 0);
    regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL, NAU8825_SPKR_ENGND1 | NAU8825_SPKR_ENGND2 | NAU8825_SPKR_DWN1R | NAU8825_SPKR_DWN1L, NAU8825_SPKR_ENGND1);
    regmap_update_bits(regmap, NAU8825_REG_MIC_BIAS, NAU8825_MICBIAS_JKSLV | NAU8825_MICBIAS_JKR2, NAU8825_MICBIAS_JKSLV);
    regmap_update_bits(regmap, NAU8825_REG_SAR_CTRL, NAU8825_SAR_INPUT_MASK, NAU8825_SAR_INPUT_JKSLV);
    regmap_read(regmap, NAU8825_REG_SARDOUT_RAM_STATUS, &mut adc_mg2);
    snd_soc_dapm_disable_pin(dapm, c"SAR".as_ptr());
    snd_soc_dapm_disable_pin(dapm, c"MICBIAS".as_ptr());
    snd_soc_dapm_sync(dapm);
    regmap_update_bits(regmap, NAU8825_REG_MIC_BIAS, NAU8825_MICBIAS_JKSLV | NAU8825_MICBIAS_LOWNOISE_MASK | NAU8825_MICBIAS_VOLTAGE_MASK, (*nau8825).micbias_voltage);
    regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL, NAU8825_SPKR_ENGND1 | NAU8825_SPKR_ENGND2 | NAU8825_SPKR_DWN1R | NAU8825_SPKR_DWN1L, NAU8825_SPKR_ENGND1 | NAU8825_SPKR_ENGND2 | NAU8825_SPKR_DWN1R | NAU8825_SPKR_DWN1L);
    regmap_update_bits(regmap, NAU8825_REG_ANALOG_CONTROL_1, NAU8825_TESTDACIN_MASK, NAU8825_TESTDACIN_GND);
    regmap_write(regmap, NAU8825_REG_TRIM_SETTINGS, 0);
    regmap_update_bits(regmap, NAU8825_REG_SAR_CTRL, NAU8825_SAR_TRACKING_GAIN_MASK | NAU8825_SAR_HV_SEL_MASK, (*nau8825).sar_voltage << NAU8825_SAR_TRACKING_GAIN_SFT);
    regmap_update_bits(regmap, NAU8825_REG_SAR_CTRL, NAU8825_SAR_COMPARE_TIME_MASK | NAU8825_SAR_SAMPLING_TIME_MASK, ((*nau8825).sar_compare_time << NAU8825_SAR_COMPARE_TIME_SFT) | ((*nau8825).sar_sampling_time << NAU8825_SAR_SAMPLING_TIME_SFT));
    dev_dbg((*nau8825).dev, c"adc_mg1:%x, adc_mg2:%x\n".as_ptr(), adc_mg1, adc_mg2);
    if adc_mg1 > adc_mg2 {
        dev_dbg((*nau8825).dev, c"OMTP (micgnd1) mic connected\n".as_ptr());
        regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL, NAU8825_SPKR_ENGND1 | NAU8825_SPKR_ENGND2, NAU8825_SPKR_ENGND2);
        regmap_update_bits(regmap, NAU8825_REG_MIC_BIAS, NAU8825_MICBIAS_JKSLV | NAU8825_MICBIAS_JKR2, NAU8825_MICBIAS_JKR2);
        regmap_update_bits(regmap, NAU8825_REG_SAR_CTRL, NAU8825_SAR_INPUT_MASK, NAU8825_SAR_INPUT_JKR2);
    } else if adc_mg1 < adc_mg2 {
        dev_dbg((*nau8825).dev, c"CTIA (micgnd2) mic connected\n".as_ptr());
        regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL, NAU8825_SPKR_ENGND1 | NAU8825_SPKR_ENGND2, NAU8825_SPKR_ENGND1);
        regmap_update_bits(regmap, NAU8825_REG_MIC_BIAS, NAU8825_MICBIAS_JKSLV | NAU8825_MICBIAS_JKR2, NAU8825_MICBIAS_JKSLV);
        regmap_update_bits(regmap, NAU8825_REG_SAR_CTRL, NAU8825_SAR_INPUT_MASK, NAU8825_SAR_INPUT_JKSLV);
    } else {
        dev_err((*nau8825).dev, c"Jack broken.\n".as_ptr());
        return -EINVAL;
    }
    0
}

unsafe fn nau8825_jack_insert(nau8825: *mut nau8825) -> c_int {
    let regmap = (*nau8825).regmap;
    let dapm = (*nau8825).dapm;
    let mut jack_status_reg: c_uint = 0;
    let mut type_: c_int = 0;
    regmap_read(regmap, NAU8825_REG_GENERAL_STATUS, &mut jack_status_reg);
    let mic_detected = ((jack_status_reg >> 10) & 3) as c_int;
    (*nau8825).high_imped = mic_detected == 0x3;
    match mic_detected {
        0 => type_ = SND_JACK_HEADPHONE,
        1 => {
            dev_dbg((*nau8825).dev, c"OMTP (micgnd1) mic connected\n".as_ptr());
            type_ = SND_JACK_HEADSET;
            regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL, 3 << 2, 1 << 2);
            regmap_update_bits(regmap, NAU8825_REG_MIC_BIAS, NAU8825_MICBIAS_JKSLV | NAU8825_MICBIAS_JKR2, NAU8825_MICBIAS_JKR2);
            regmap_update_bits(regmap, NAU8825_REG_SAR_CTRL, NAU8825_SAR_INPUT_MASK, NAU8825_SAR_INPUT_JKR2);
            snd_soc_dapm_force_enable_pin(dapm, c"MICBIAS".as_ptr()); snd_soc_dapm_force_enable_pin(dapm, c"SAR".as_ptr()); snd_soc_dapm_sync(dapm);
        }
        2 => {
            dev_dbg((*nau8825).dev, c"CTIA (micgnd2) mic connected\n".as_ptr());
            type_ = SND_JACK_HEADSET;
            regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL, 3 << 2, 2 << 2);
            regmap_update_bits(regmap, NAU8825_REG_MIC_BIAS, NAU8825_MICBIAS_JKSLV | NAU8825_MICBIAS_JKR2, NAU8825_MICBIAS_JKSLV);
            regmap_update_bits(regmap, NAU8825_REG_SAR_CTRL, NAU8825_SAR_INPUT_MASK, NAU8825_SAR_INPUT_JKSLV);
            snd_soc_dapm_force_enable_pin(dapm, c"MICBIAS".as_ptr()); snd_soc_dapm_force_enable_pin(dapm, c"SAR".as_ptr()); snd_soc_dapm_sync(dapm);
        }
        3 => {
            dev_warn((*nau8825).dev, c"Detection failure. Try the manually mechanism for jack type checking.\n".as_ptr());
            if nau8825_high_imped_detection(nau8825) == 0 {
                type_ = SND_JACK_HEADSET;
                snd_soc_dapm_force_enable_pin(dapm, c"MICBIAS".as_ptr()); snd_soc_dapm_force_enable_pin(dapm, c"SAR".as_ptr()); snd_soc_dapm_sync(dapm);
            } else {
                type_ = SND_JACK_HEADPHONE;
            }
        }
        _ => {}
    }
    regmap_update_bits(regmap, NAU8825_REG_CLK_DIVIDER, NAU8825_CLK_MCLK_SRC_MASK, 0xf);
    regmap_update_bits(regmap, NAU8825_REG_HSD_CTRL, NAU8825_HSD_AUTO_MODE, 0);
    type_
}

unsafe extern "C" fn nau8825_interrupt(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let nau8825 = data as *mut nau8825;
    let regmap = (*nau8825).regmap;
    let mut active_irq: c_uint = 0;
    let mut clear_irq: c_uint = 0;
    let mut event: c_int = 0;
    let mut event_mask: c_int = 0;
    if regmap_read(regmap, NAU8825_REG_IRQ_STATUS, &mut active_irq) != 0 {
        dev_err((*nau8825).dev, c"failed to read irq status\n".as_ptr());
        return IRQ_NONE;
    }
    if ((active_irq as c_int) & NAU8825_JACK_EJECTION_IRQ_MASK) == NAU8825_JACK_EJECTION_DETECTED {
        nau8825_eject_jack(nau8825);
        event_mask |= SND_JACK_HEADSET;
        clear_irq = NAU8825_JACK_EJECTION_IRQ_MASK as c_uint;
    } else if ((active_irq as c_int) & NAU8825_KEY_SHORT_PRESS_IRQ) != 0 {
        let mut key_status: c_uint = 0;
        regmap_read(regmap, NAU8825_REG_INT_CLR_KEY_STATUS, &mut key_status);
        (*nau8825).button_pressed = nau8825_button_decode((key_status >> 8) as c_int);
        event |= (*nau8825).button_pressed;
        event_mask |= NAU8825_BUTTONS;
        clear_irq = NAU8825_KEY_SHORT_PRESS_IRQ as c_uint;
    } else if ((active_irq as c_int) & NAU8825_KEY_RELEASE_IRQ) != 0 {
        event_mask = NAU8825_BUTTONS;
        clear_irq = NAU8825_KEY_RELEASE_IRQ as c_uint;
    } else if ((active_irq as c_int) & NAU8825_HEADSET_COMPLETION_IRQ) != 0 {
        if nau8825_is_jack_inserted(regmap) {
            event |= nau8825_jack_insert(nau8825);
            if (*nau8825).xtalk_enable && !(*nau8825).high_imped {
                if !(*nau8825).xtalk_protect {
                    (*nau8825).xtalk_protect = true;
                    let ret = nau8825_sema_acquire(nau8825, 0);
                    if ret != 0 { (*nau8825).xtalk_protect = false; }
                }
                if (*nau8825).xtalk_protect {
                    (*nau8825).xtalk_state = NAU8825_XTALK_PREPARE;
                    schedule_work(&mut (*nau8825).xtalk_work);
                }
            } else if (*nau8825).xtalk_protect {
                nau8825_sema_release(nau8825);
                (*nau8825).xtalk_protect = false;
            }
        } else {
            dev_warn((*nau8825).dev, c"Headset completion IRQ fired but no headset connected\n".as_ptr());
            nau8825_eject_jack(nau8825);
        }
        event_mask |= SND_JACK_HEADSET;
        clear_irq = NAU8825_HEADSET_COMPLETION_IRQ as c_uint;
        if (*nau8825).xtalk_state == NAU8825_XTALK_PREPARE {
            (*nau8825).xtalk_event = event;
            (*nau8825).xtalk_event_mask = event_mask;
        }
    } else if ((active_irq as c_int) & NAU8825_IMPEDANCE_MEAS_IRQ) != 0 {
        if (*nau8825).xtalk_enable && (*nau8825).xtalk_protect {
            schedule_work(&mut (*nau8825).xtalk_work);
        }
        clear_irq = NAU8825_IMPEDANCE_MEAS_IRQ as c_uint;
    } else if ((active_irq as c_int) & NAU8825_JACK_INSERTION_IRQ_MASK) == NAU8825_JACK_INSERTION_DETECTED {
        if nau8825_is_jack_inserted(regmap) {
            regmap_update_bits(regmap, NAU8825_REG_INTERRUPT_DIS_CTRL, NAU8825_IRQ_INSERT_DIS, NAU8825_IRQ_INSERT_DIS);
            regmap_update_bits(regmap, NAU8825_REG_INTERRUPT_MASK, NAU8825_IRQ_INSERT_EN, NAU8825_IRQ_INSERT_EN);
            nau8825_setup_auto_irq(nau8825);
        }
    }
    if clear_irq == 0 { clear_irq = active_irq; }
    regmap_write(regmap, NAU8825_REG_INT_CLR_KEY_STATUS, clear_irq);
    if event_mask != 0 && (*nau8825).xtalk_state == NAU8825_XTALK_DONE {
        snd_soc_jack_report((*nau8825).jack, event, event_mask);
    }
    IRQ_HANDLED
}

unsafe fn nau8825_setup_buttons(nau8825: *mut nau8825) {
    let regmap = (*nau8825).regmap;
    regmap_update_bits(regmap, NAU8825_REG_SAR_CTRL, NAU8825_SAR_TRACKING_GAIN_MASK, (*nau8825).sar_voltage << NAU8825_SAR_TRACKING_GAIN_SFT);
    regmap_update_bits(regmap, NAU8825_REG_SAR_CTRL, NAU8825_SAR_COMPARE_TIME_MASK, (*nau8825).sar_compare_time << NAU8825_SAR_COMPARE_TIME_SFT);
    regmap_update_bits(regmap, NAU8825_REG_SAR_CTRL, NAU8825_SAR_SAMPLING_TIME_MASK, (*nau8825).sar_sampling_time << NAU8825_SAR_SAMPLING_TIME_SFT);
    regmap_update_bits(regmap, NAU8825_REG_KEYDET_CTRL, NAU8825_KEYDET_LEVELS_NR_MASK, ((*nau8825).sar_threshold_num - 1) << NAU8825_KEYDET_LEVELS_NR_SFT);
    regmap_update_bits(regmap, NAU8825_REG_KEYDET_CTRL, NAU8825_KEYDET_HYSTERESIS_MASK, (*nau8825).sar_hysteresis << NAU8825_KEYDET_HYSTERESIS_SFT);
    regmap_update_bits(regmap, NAU8825_REG_KEYDET_CTRL, NAU8825_KEYDET_SHORTKEY_DEBOUNCE_MASK, (*nau8825).key_debounce << NAU8825_KEYDET_SHORTKEY_DEBOUNCE_SFT);
    regmap_write(regmap, NAU8825_REG_VDET_THRESHOLD_1, ((*nau8825).sar_threshold[0] << 8) | (*nau8825).sar_threshold[1]);
    regmap_write(regmap, NAU8825_REG_VDET_THRESHOLD_2, ((*nau8825).sar_threshold[2] << 8) | (*nau8825).sar_threshold[3]);
    regmap_write(regmap, NAU8825_REG_VDET_THRESHOLD_3, ((*nau8825).sar_threshold[4] << 8) | (*nau8825).sar_threshold[5]);
    regmap_write(regmap, NAU8825_REG_VDET_THRESHOLD_4, ((*nau8825).sar_threshold[6] << 8) | (*nau8825).sar_threshold[7]);
    regmap_update_bits(regmap, NAU8825_REG_INTERRUPT_MASK, NAU8825_IRQ_KEY_SHORT_PRESS_EN | NAU8825_IRQ_KEY_RELEASE_EN, 0);
}

unsafe fn nau8825_init_regs(nau8825: *mut nau8825) {
    let regmap = (*nau8825).regmap;
    regmap_write(regmap, NAU8825_REG_IIC_ADDR_SET, 0x0001);
    regmap_update_bits(regmap, NAU8825_REG_BIAS_ADJ, NAU8825_BIAS_VMID, NAU8825_BIAS_VMID);
    regmap_update_bits(regmap, NAU8825_REG_BOOST, NAU8825_GLOBAL_BIAS_EN, NAU8825_GLOBAL_BIAS_EN);
    regmap_update_bits(regmap, NAU8825_REG_BIAS_ADJ, NAU8825_BIAS_VMID_SEL_MASK, (*nau8825).vref_impedance << NAU8825_BIAS_VMID_SEL_SFT);
    regmap_update_bits(regmap, NAU8825_REG_BOOST, NAU8825_PRECHARGE_DIS | NAU8825_HP_BOOST_DIS | NAU8825_HP_BOOST_G_DIS | NAU8825_SHORT_SHUTDOWN_EN, NAU8825_PRECHARGE_DIS | NAU8825_HP_BOOST_DIS | NAU8825_HP_BOOST_G_DIS | NAU8825_SHORT_SHUTDOWN_EN);
    regmap_update_bits(regmap, NAU8825_REG_GPIO12_CTRL, NAU8825_JKDET_OUTPUT_EN, if (*nau8825).jkdet_enable { 0 } else { NAU8825_JKDET_OUTPUT_EN });
    regmap_update_bits(regmap, NAU8825_REG_GPIO12_CTRL, NAU8825_JKDET_PULL_EN, if (*nau8825).jkdet_pull_enable { 0 } else { NAU8825_JKDET_PULL_EN });
    regmap_update_bits(regmap, NAU8825_REG_GPIO12_CTRL, NAU8825_JKDET_PULL_UP, if (*nau8825).jkdet_pull_up { NAU8825_JKDET_PULL_UP } else { 0 });
    regmap_update_bits(regmap, NAU8825_REG_JACK_DET_CTRL, NAU8825_JACK_POLARITY, if (*nau8825).jkdet_polarity != 0 { 0 } else { NAU8825_JACK_POLARITY });
    regmap_update_bits(regmap, NAU8825_REG_JACK_DET_CTRL, NAU8825_JACK_INSERT_DEBOUNCE_MASK, (*nau8825).jack_insert_debounce << NAU8825_JACK_INSERT_DEBOUNCE_SFT);
    regmap_update_bits(regmap, NAU8825_REG_JACK_DET_CTRL, NAU8825_JACK_EJECT_DEBOUNCE_MASK, (*nau8825).jack_eject_debounce << NAU8825_JACK_EJECT_DEBOUNCE_SFT);
    regmap_update_bits(regmap, NAU8825_REG_INTERRUPT_MASK, NAU8825_IRQ_PIN_PULLUP | NAU8825_IRQ_PIN_PULL_EN, NAU8825_IRQ_PIN_PULLUP | NAU8825_IRQ_PIN_PULL_EN);
    regmap_update_bits(regmap, NAU8825_REG_INTERRUPT_MASK, 0x7ff, 0x7ff);
    regmap_update_bits(regmap, NAU8825_REG_MIC_BIAS, NAU8825_MICBIAS_VOLTAGE_MASK, (*nau8825).micbias_voltage);
    if (*nau8825).sar_threshold_num != 0 { nau8825_setup_buttons(nau8825); }
    regmap_update_bits(regmap, NAU8825_REG_ADC_RATE, NAU8825_ADC_SYNC_DOWN_MASK | NAU8825_ADC_SINC4_EN, NAU8825_ADC_SYNC_DOWN_64);
    regmap_update_bits(regmap, NAU8825_REG_DAC_CTRL1, NAU8825_DAC_OVERSAMPLE_MASK, NAU8825_DAC_OVERSAMPLE_64);
    if (*nau8825).sw_id == NAU8825_SOFTWARE_ID_NAU8825 {
        regmap_update_bits(regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL, NAU8825_POWER_DOWN_DACR | NAU8825_POWER_DOWN_DACL);
    }
    regmap_update_bits(regmap, NAU8825_REG_BIAS_ADJ, NAU8825_BIAS_TESTDAC_EN, NAU8825_BIAS_TESTDAC_EN);
    regmap_update_bits(regmap, NAU8825_REG_DAC_CTRL1, NAU8825_DAC_CLIP_OFF, NAU8825_DAC_CLIP_OFF);
    regmap_update_bits(regmap, NAU8825_REG_ANALOG_CONTROL_2, NAU8825_HP_NON_CLASSG_CURRENT_2xADJ | NAU8825_DAC_CAPACITOR_MSB | NAU8825_DAC_CAPACITOR_LSB, NAU8825_HP_NON_CLASSG_CURRENT_2xADJ | NAU8825_DAC_CAPACITOR_MSB | NAU8825_DAC_CAPACITOR_LSB);
    regmap_update_bits(regmap, NAU8825_REG_CLASSG_CTRL, NAU8825_CLASSG_TIMER_MASK, 0x20 << NAU8825_CLASSG_TIMER_SFT);
    regmap_update_bits(regmap, NAU8825_REG_RDAC, NAU8825_RDAC_CLK_DELAY_MASK | NAU8825_RDAC_VREF_MASK, (0x2 << NAU8825_RDAC_CLK_DELAY_SFT) | (0x3 << NAU8825_RDAC_VREF_SFT));
    regmap_update_bits(regmap, NAU8825_REG_DACL_CTRL, NAU8825_DACL_CH_SEL_MASK, NAU8825_DACL_CH_SEL_L);
    regmap_update_bits(regmap, NAU8825_REG_DACR_CTRL, NAU8825_DACL_CH_SEL_MASK, NAU8825_DACL_CH_SEL_R);
    regmap_update_bits(regmap, NAU8825_REG_LEFT_TIME_SLOT, NAU8825_DIS_FS_SHORT_DET, NAU8825_DIS_FS_SHORT_DET);
    regmap_update_bits(regmap, NAU8825_REG_CHARGE_PUMP, NAU8825_ADCOUT_DS_MASK, (*nau8825).adcout_ds << NAU8825_ADCOUT_DS_SFT);
}

unsafe extern "C" fn nau8825_component_probe(component: *mut snd_soc_component) -> c_int {
    let nau8825 = snd_soc_component_get_drvdata(component);
    (*nau8825).dapm = snd_soc_component_to_dapm(component);
    0
}

unsafe extern "C" fn nau8825_component_remove(component: *mut snd_soc_component) {
    let nau8825 = snd_soc_component_get_drvdata(component);
    nau8825_xtalk_cancel(nau8825);
}

unsafe fn nau8825_calc_fll_param(fll_in: c_uint, fs: c_uint, fll_param: *mut nau8825_fll) -> c_int {
    let mut fref: c_uint = 0;
    let mut i: usize = 0;
    while i < fll_pre_scalar.len() {
        fref = fll_in / fll_pre_scalar[i].param;
        if fref <= NAU_FREF_MAX { break; }
        i += 1;
    }
    if i == fll_pre_scalar.len() { return -EINVAL; }
    (*fll_param).clk_ref_div = fll_pre_scalar[i].val as c_int;
    i = 0;
    while i < fll_ratio.len() {
        if fref >= fll_ratio[i].param { break; }
        i += 1;
    }
    if i == fll_ratio.len() { return -EINVAL; }
    (*fll_param).ratio = fll_ratio[i].val as c_int;
    let mut fvco_max: u64 = 0;
    let mut fvco_sel = mclk_src_scaling.len();
    i = 0;
    while i < mclk_src_scaling.len() {
        let fvco = 256u64 * fs as u64 * 2 * mclk_src_scaling[i].param as u64;
        if fvco > NAU_FVCO_MIN && fvco < NAU_FVCO_MAX && fvco_max < fvco {
            fvco_max = fvco;
            fvco_sel = i;
        }
        i += 1;
    }
    if mclk_src_scaling.len() == fvco_sel { return -EINVAL; }
    (*fll_param).mclk_src = mclk_src_scaling[fvco_sel].val as c_int;
    let fvco = div_u64(fvco_max << (*fll_param).fll_frac_num, fref as u64 * (*fll_param).ratio as u64);
    (*fll_param).fll_int = ((fvco >> (*fll_param).fll_frac_num) & 0x3ff) as c_int;
    if (*fll_param).fll_frac_num == 16 {
        (*fll_param).fll_frac = (fvco & 0xffff) as c_int;
    } else {
        (*fll_param).fll_frac = (fvco & 0xffffff) as c_int;
    }
    0
}

unsafe fn nau8825_fll_apply(nau8825: *mut nau8825, fll_param: *mut nau8825_fll) {
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_CLK_DIVIDER, NAU8825_CLK_SRC_MASK | NAU8825_CLK_MCLK_SRC_MASK, NAU8825_CLK_SRC_MCLK | (*fll_param).mclk_src as c_uint);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_FLL1, NAU8825_FLL_RATIO_MASK | NAU8825_ICTRL_LATCH_MASK, (*fll_param).ratio as c_uint | (0x6 << NAU8825_ICTRL_LATCH_SFT));
    if (*fll_param).fll_frac_num == 16 {
        regmap_write((*nau8825).regmap, NAU8825_REG_FLL2, (*fll_param).fll_frac as c_uint);
    } else {
        regmap_write((*nau8825).regmap, NAU8825_REG_FLL2_LOWER, ((*fll_param).fll_frac & 0xffff) as c_uint);
        regmap_write((*nau8825).regmap, NAU8825_REG_FLL2_UPPER, (((*fll_param).fll_frac >> 16) & 0xff) as c_uint);
    }
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_FLL3, NAU8825_FLL_INTEGER_MASK, (*fll_param).fll_int as c_uint);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_FLL4, NAU8825_FLL_REF_DIV_MASK, (*fll_param).clk_ref_div as c_uint << NAU8825_FLL_REF_DIV_SFT);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_FLL5, NAU8825_FLL_CLK_SW_MASK, NAU8825_FLL_CLK_SW_REF);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_FLL6, NAU8825_DCO_EN, 0);
    if (*fll_param).fll_frac != 0 {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_FLL5, NAU8825_FLL_PDB_DAC_EN | NAU8825_FLL_LOOP_FTR_EN | NAU8825_FLL_FTR_SW_MASK, NAU8825_FLL_PDB_DAC_EN | NAU8825_FLL_LOOP_FTR_EN | NAU8825_FLL_FTR_SW_FILTER);
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_FLL6, NAU8825_SDM_EN | NAU8825_CUTOFF500, NAU8825_SDM_EN | NAU8825_CUTOFF500);
    } else {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_FLL5, NAU8825_FLL_PDB_DAC_EN | NAU8825_FLL_LOOP_FTR_EN | NAU8825_FLL_FTR_SW_MASK, NAU8825_FLL_FTR_SW_ACCU);
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_FLL6, NAU8825_SDM_EN | NAU8825_CUTOFF500, 0);
    }
}

unsafe extern "C" fn nau8825_set_pll(component: *mut snd_soc_component, _pll_id: c_int, _source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int {
    let nau8825 = snd_soc_component_get_drvdata(component);
    let mut fll_param = nau8825_fll { mclk_src: 0, ratio: 0, fll_frac: 0, fll_frac_num: if (*nau8825).sw_id == NAU8825_SOFTWARE_ID_NAU8825 { 16 } else { 24 }, fll_int: 0, clk_ref_div: 0 };
    let fs = freq_out / 256;
    let ret = nau8825_calc_fll_param(freq_in, fs, &mut fll_param);
    if ret < 0 {
        dev_err((*component).dev, c"Unsupported input clock %d\n".as_ptr(), freq_in);
        return ret;
    }
    dev_dbg((*component).dev, c"mclk_src=%x ratio=%x fll_frac=%x fll_int=%x clk_ref_div=%x\n".as_ptr(), fll_param.mclk_src, fll_param.ratio, fll_param.fll_frac, fll_param.fll_int, fll_param.clk_ref_div);
    nau8825_fll_apply(nau8825, &mut fll_param);
    mdelay(2);
    regmap_update_bits((*nau8825).regmap, NAU8825_REG_CLK_DIVIDER, NAU8825_CLK_SRC_MASK, NAU8825_CLK_SRC_VCO);
    0
}

unsafe fn nau8825_mclk_prepare(nau8825: *mut nau8825, mut freq: c_uint) -> c_int {
    let mut ret: c_int;
    (*nau8825).mclk = devm_clk_get((*nau8825).dev, c"mclk".as_ptr());
    if IS_ERR((*nau8825).mclk) {
        dev_info((*nau8825).dev, c"No 'mclk' clock found, assume MCLK is managed externally".as_ptr());
        return 0;
    }
    if (*nau8825).mclk_freq == 0 {
        ret = clk_prepare_enable((*nau8825).mclk);
        if ret != 0 { dev_err((*nau8825).dev, c"Unable to prepare codec mclk\n".as_ptr()); return ret; }
    }
    if (*nau8825).mclk_freq != freq {
        freq = clk_round_rate((*nau8825).mclk, freq);
        ret = clk_set_rate((*nau8825).mclk, freq);
        if ret != 0 { dev_err((*nau8825).dev, c"Unable to set mclk rate\n".as_ptr()); return ret; }
        (*nau8825).mclk_freq = freq;
    }
    0
}

unsafe fn nau8825_configure_mclk_as_sysclk(regmap: *mut regmap) {
    regmap_update_bits(regmap, NAU8825_REG_CLK_DIVIDER, NAU8825_CLK_SRC_MASK, NAU8825_CLK_SRC_MCLK);
    regmap_update_bits(regmap, NAU8825_REG_FLL6, NAU8825_DCO_EN, 0);
    regmap_update_bits(regmap, NAU8825_REG_FLL1, NAU8825_ICTRL_LATCH_MASK, 0);
}

unsafe extern "C" fn nau8825_configure_sysclk(nau8825: *mut nau8825, clk_id: c_int, freq: c_uint) -> c_int {
    let regmap = (*nau8825).regmap;
    let mut ret: c_int;
    if clk_id == NAU8825_CLK_DIS {
        nau8825_configure_mclk_as_sysclk(regmap);
        if (*nau8825).mclk_freq != 0 { clk_disable_unprepare((*nau8825).mclk); (*nau8825).mclk_freq = 0; }
    } else if clk_id == NAU8825_CLK_MCLK {
        nau8825_sema_acquire(nau8825, 3 * HZ);
        nau8825_configure_mclk_as_sysclk(regmap);
        regmap_update_bits(regmap, NAU8825_REG_CLK_DIVIDER, NAU8825_CLK_MCLK_SRC_MASK, 0);
        nau8825_sema_release(nau8825);
        ret = nau8825_mclk_prepare(nau8825, freq);
        if ret != 0 { return ret; }
    } else if clk_id == NAU8825_CLK_INTERNAL {
        if nau8825_is_jack_inserted((*nau8825).regmap) {
            regmap_update_bits(regmap, NAU8825_REG_FLL6, NAU8825_DCO_EN, NAU8825_DCO_EN);
            regmap_update_bits(regmap, NAU8825_REG_CLK_DIVIDER, NAU8825_CLK_SRC_MASK, NAU8825_CLK_SRC_VCO);
            regmap_update_bits(regmap, NAU8825_REG_CLK_DIVIDER, NAU8825_CLK_MCLK_SRC_MASK, 0xf);
            regmap_update_bits(regmap, NAU8825_REG_FLL1, NAU8825_ICTRL_LATCH_MASK | NAU8825_FLL_RATIO_MASK, 0x10);
            regmap_update_bits(regmap, NAU8825_REG_FLL6, NAU8825_SDM_EN, NAU8825_SDM_EN);
        } else {
            nau8825_configure_mclk_as_sysclk(regmap);
            dev_warn((*nau8825).dev, c"Disable clock for power saving when no headset connected\n".as_ptr());
        }
        if (*nau8825).mclk_freq != 0 { clk_disable_unprepare((*nau8825).mclk); (*nau8825).mclk_freq = 0; }
    } else if clk_id == NAU8825_CLK_FLL_MCLK {
        nau8825_sema_acquire(nau8825, 3 * HZ);
        regmap_update_bits(regmap, NAU8825_REG_FLL3, NAU8825_FLL_CLK_SRC_MASK | NAU8825_GAIN_ERR_MASK, NAU8825_FLL_CLK_SRC_MCLK | 0);
        nau8825_sema_release(nau8825);
        ret = nau8825_mclk_prepare(nau8825, freq);
        if ret != 0 { return ret; }
    } else if clk_id == NAU8825_CLK_FLL_BLK || clk_id == NAU8825_CLK_FLL_FS {
        nau8825_sema_acquire(nau8825, 3 * HZ);
        let src = if clk_id == NAU8825_CLK_FLL_BLK { NAU8825_FLL_CLK_SRC_BLK } else { NAU8825_FLL_CLK_SRC_FS };
        regmap_update_bits(regmap, NAU8825_REG_FLL3, NAU8825_FLL_CLK_SRC_MASK | NAU8825_GAIN_ERR_MASK, src | (0xf << NAU8825_GAIN_ERR_SFT));
        nau8825_sema_release(nau8825);
        if (*nau8825).mclk_freq != 0 { clk_disable_unprepare((*nau8825).mclk); (*nau8825).mclk_freq = 0; }
    } else {
        dev_err((*nau8825).dev, c"Invalid clock id (%d)\n".as_ptr(), clk_id);
        return -EINVAL;
    }
    dev_dbg((*nau8825).dev, c"Sysclk is %dHz and clock id is %d\n".as_ptr(), freq, clk_id);
    0
}

unsafe extern "C" fn nau8825_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, _source: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let nau8825 = snd_soc_component_get_drvdata(component);
    nau8825_configure_sysclk(nau8825, clk_id, freq)
}

unsafe fn nau8825_resume_setup(nau8825: *mut nau8825) -> c_int {
    let regmap = (*nau8825).regmap;
    nau8825_configure_sysclk(nau8825, NAU8825_CLK_DIS, 0);
    nau8825_int_status_clear_all(regmap);
    regmap_update_bits(regmap, NAU8825_REG_INTERRUPT_MASK, NAU8825_IRQ_OUTPUT_EN | NAU8825_IRQ_HEADSET_COMPLETE_EN | NAU8825_IRQ_EJECT_EN | NAU8825_IRQ_INSERT_EN, NAU8825_IRQ_OUTPUT_EN | NAU8825_IRQ_HEADSET_COMPLETE_EN);
    regmap_update_bits(regmap, NAU8825_REG_JACK_DET_CTRL, NAU8825_JACK_DET_DB_BYPASS, NAU8825_JACK_DET_DB_BYPASS);
    regmap_update_bits(regmap, NAU8825_REG_INTERRUPT_DIS_CTRL, NAU8825_IRQ_INSERT_DIS | NAU8825_IRQ_EJECT_DIS, 0);
    0
}

unsafe extern "C" fn nau8825_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let nau8825 = snd_soc_component_get_drvdata(component);
    if level == SND_SOC_BIAS_STANDBY {
        if snd_soc_dapm_get_bias_level((*nau8825).dapm) == SND_SOC_BIAS_OFF {
            if (*nau8825).mclk_freq != 0 {
                let ret = clk_prepare_enable((*nau8825).mclk);
                if ret != 0 { dev_err((*nau8825).dev, c"Unable to prepare component mclk\n".as_ptr()); return ret; }
            }
            nau8825_resume_setup(nau8825);
        }
    } else if level == SND_SOC_BIAS_OFF {
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_MIC_BIAS, NAU8825_MICBIAS_JKSLV | NAU8825_MICBIAS_JKR2, 0);
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_HSD_CTRL, 0xf, 0xf);
        nau8825_xtalk_cancel(nau8825);
        regmap_write((*nau8825).regmap, NAU8825_REG_INTERRUPT_DIS_CTRL, 0xffff);
        regmap_update_bits((*nau8825).regmap, NAU8825_REG_ENA_CTRL, NAU8825_ENABLE_ADC, 0);
        if (*nau8825).mclk_freq != 0 { clk_disable_unprepare((*nau8825).mclk); }
    } else if level == SND_SOC_BIAS_ON || level == SND_SOC_BIAS_PREPARE {
    }
    0
}

unsafe extern "C" fn nau8825_suspend(component: *mut snd_soc_component) -> c_int {
    let nau8825 = snd_soc_component_get_drvdata(component);
    disable_irq((*nau8825).irq);
    snd_soc_dapm_force_bias_level((*nau8825).dapm, SND_SOC_BIAS_OFF);
    snd_soc_dapm_disable_pin((*nau8825).dapm, c"SAR".as_ptr());
    snd_soc_dapm_disable_pin((*nau8825).dapm, c"MICBIAS".as_ptr());
    snd_soc_dapm_sync((*nau8825).dapm);
    regcache_cache_only((*nau8825).regmap, true);
    regcache_mark_dirty((*nau8825).regmap);
    0
}

unsafe extern "C" fn nau8825_resume(component: *mut snd_soc_component) -> c_int {
    let nau8825 = snd_soc_component_get_drvdata(component);
    regcache_cache_only((*nau8825).regmap, false);
    regcache_sync((*nau8825).regmap);
    (*nau8825).xtalk_protect = true;
    let ret = nau8825_sema_acquire(nau8825, 0);
    if ret != 0 { (*nau8825).xtalk_protect = false; }
    enable_irq((*nau8825).irq);
    0
}

unsafe extern "C" fn nau8825_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    nau8825_enable_jack_detect(component, jack)
}

static nau8825_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(nau8825_component_probe),
    remove: Some(nau8825_component_remove),
    set_sysclk: Some(nau8825_set_sysclk),
    set_pll: Some(nau8825_set_pll),
    set_bias_level: Some(nau8825_set_bias_level),
    suspend: Some(nau8825_suspend),
    resume: Some(nau8825_resume),
    controls: nau8825_controls.as_ptr(),
    num_controls: 9,
    dapm_widgets: nau8825_dapm_widgets.as_ptr(),
    num_dapm_widgets: 43,
    dapm_routes: nau8825_dapm_routes.as_ptr(),
    num_dapm_routes: 39,
    set_jack: Some(nau8825_set_jack),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe fn nau8825_reset_chip(regmap: *mut regmap) {
    regmap_write(regmap, NAU8825_REG_RESET, 0x00);
    regmap_write(regmap, NAU8825_REG_RESET, 0x00);
}

unsafe fn nau8825_print_device_properties(nau8825: *mut nau8825) {
    let dev = (*nau8825).dev;
    dev_dbg(dev, c"jkdet-enable:         %d\n".as_ptr(), (*nau8825).jkdet_enable as c_int);
    dev_dbg(dev, c"jkdet-pull-enable:    %d\n".as_ptr(), (*nau8825).jkdet_pull_enable as c_int);
    dev_dbg(dev, c"jkdet-pull-up:        %d\n".as_ptr(), (*nau8825).jkdet_pull_up as c_int);
    dev_dbg(dev, c"jkdet-polarity:       %d\n".as_ptr(), (*nau8825).jkdet_polarity);
    dev_dbg(dev, c"micbias-voltage:      %d\n".as_ptr(), (*nau8825).micbias_voltage);
    dev_dbg(dev, c"vref-impedance:       %d\n".as_ptr(), (*nau8825).vref_impedance);
    dev_dbg(dev, c"sar-threshold-num:    %d\n".as_ptr(), (*nau8825).sar_threshold_num);
    let mut i = 0usize;
    while i < (*nau8825).sar_threshold_num as usize {
        dev_dbg(dev, c"sar-threshold[%d]=%d\n".as_ptr(), i as c_int, (*nau8825).sar_threshold[i]);
        i += 1;
    }
    dev_dbg(dev, c"sar-hysteresis:       %d\n".as_ptr(), (*nau8825).sar_hysteresis);
    dev_dbg(dev, c"sar-voltage:          %d\n".as_ptr(), (*nau8825).sar_voltage);
    dev_dbg(dev, c"sar-compare-time:     %d\n".as_ptr(), (*nau8825).sar_compare_time);
    dev_dbg(dev, c"sar-sampling-time:    %d\n".as_ptr(), (*nau8825).sar_sampling_time);
    dev_dbg(dev, c"short-key-debounce:   %d\n".as_ptr(), (*nau8825).key_debounce);
    dev_dbg(dev, c"jack-insert-debounce: %d\n".as_ptr(), (*nau8825).jack_insert_debounce);
    dev_dbg(dev, c"jack-eject-debounce:  %d\n".as_ptr(), (*nau8825).jack_eject_debounce);
    dev_dbg(dev, c"crosstalk-enable:     %d\n".as_ptr(), (*nau8825).xtalk_enable as c_int);
    dev_dbg(dev, c"adcout-drive-strong:  %d\n".as_ptr(), (*nau8825).adcout_ds);
    dev_dbg(dev, c"adc-delay-ms:         %d\n".as_ptr(), (*nau8825).adc_delay);
}

unsafe fn nau8825_read_device_properties(dev: *mut device, nau8825: *mut nau8825) -> c_int {
    let mut ret: c_int;
    (*nau8825).jkdet_enable = device_property_read_bool(dev, c"nuvoton,jkdet-enable".as_ptr());
    (*nau8825).jkdet_pull_enable = device_property_read_bool(dev, c"nuvoton,jkdet-pull-enable".as_ptr());
    (*nau8825).jkdet_pull_up = device_property_read_bool(dev, c"nuvoton,jkdet-pull-up".as_ptr());
    ret = device_property_read_u32(dev, c"nuvoton,jkdet-polarity".as_ptr(), &mut (*nau8825).jkdet_polarity); if ret != 0 { (*nau8825).jkdet_polarity = 1; }
    ret = device_property_read_u32(dev, c"nuvoton,micbias-voltage".as_ptr(), &mut (*nau8825).micbias_voltage); if ret != 0 { (*nau8825).micbias_voltage = 6; }
    ret = device_property_read_u32(dev, c"nuvoton,vref-impedance".as_ptr(), &mut (*nau8825).vref_impedance); if ret != 0 { (*nau8825).vref_impedance = 2; }
    ret = device_property_read_u32(dev, c"nuvoton,sar-threshold-num".as_ptr(), &mut (*nau8825).sar_threshold_num); if ret != 0 { (*nau8825).sar_threshold_num = 4; }
    ret = device_property_read_u32_array(dev, c"nuvoton,sar-threshold".as_ptr(), (*nau8825).sar_threshold.as_mut_ptr(), (*nau8825).sar_threshold_num);
    if ret != 0 { (*nau8825).sar_threshold[0] = 0x08; (*nau8825).sar_threshold[1] = 0x12; (*nau8825).sar_threshold[2] = 0x26; (*nau8825).sar_threshold[3] = 0x73; }
    ret = device_property_read_u32(dev, c"nuvoton,sar-hysteresis".as_ptr(), &mut (*nau8825).sar_hysteresis); if ret != 0 { (*nau8825).sar_hysteresis = 0; }
    ret = device_property_read_u32(dev, c"nuvoton,sar-voltage".as_ptr(), &mut (*nau8825).sar_voltage); if ret != 0 { (*nau8825).sar_voltage = 6; }
    ret = device_property_read_u32(dev, c"nuvoton,sar-compare-time".as_ptr(), &mut (*nau8825).sar_compare_time); if ret != 0 { (*nau8825).sar_compare_time = 1; }
    ret = device_property_read_u32(dev, c"nuvoton,sar-sampling-time".as_ptr(), &mut (*nau8825).sar_sampling_time); if ret != 0 { (*nau8825).sar_sampling_time = 1; }
    ret = device_property_read_u32(dev, c"nuvoton,short-key-debounce".as_ptr(), &mut (*nau8825).key_debounce); if ret != 0 { (*nau8825).key_debounce = 3; }
    ret = device_property_read_u32(dev, c"nuvoton,jack-insert-debounce".as_ptr(), &mut (*nau8825).jack_insert_debounce); if ret != 0 { (*nau8825).jack_insert_debounce = 7; }
    ret = device_property_read_u32(dev, c"nuvoton,jack-eject-debounce".as_ptr(), &mut (*nau8825).jack_eject_debounce); if ret != 0 { (*nau8825).jack_eject_debounce = 0; }
    (*nau8825).xtalk_enable = device_property_read_bool(dev, c"nuvoton,crosstalk-enable".as_ptr());
    (*nau8825).adcout_ds = device_property_read_bool(dev, c"nuvoton,adcout-drive-strong".as_ptr()) as c_uint;
    ret = device_property_read_u32(dev, c"nuvoton,adc-delay-ms".as_ptr(), &mut (*nau8825).adc_delay); if ret != 0 { (*nau8825).adc_delay = 125; }
    if (*nau8825).adc_delay < 125 || (*nau8825).adc_delay > 500 { dev_warn(dev, c"Please set the suitable delay time!\n".as_ptr()); }
    (*nau8825).mclk = devm_clk_get_optional(dev, c"mclk".as_ptr());
    if IS_ERR((*nau8825).mclk) { return PTR_ERR((*nau8825).mclk); }
    if (*nau8825).mclk.is_null() { dev_info(dev, c"No 'mclk' clock found, assume MCLK is managed externally".as_ptr()); }
    0
}

unsafe fn nau8825_setup_irq(nau8825: *mut nau8825) -> c_int {
    let ret = devm_request_threaded_irq((*nau8825).dev, (*nau8825).irq, ptr::null(), nau8825_interrupt, IRQF_TRIGGER_LOW | IRQF_ONESHOT, c"nau8825".as_ptr(), nau8825 as *mut c_void);
    if ret != 0 {
        dev_err((*nau8825).dev, c"Cannot request irq %d (%d)\n".as_ptr(), (*nau8825).irq, ret);
        return ret;
    }
    0
}

unsafe extern "C" fn nau8825_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let mut nau8825 = dev_get_platdata(dev);
    let mut ret: c_int;
    let mut value: c_uint = 0;
    if nau8825.is_null() {
        nau8825 = devm_kzalloc(dev, core::mem::size_of::<nau8825>(), GFP_KERNEL) as *mut nau8825;
        if nau8825.is_null() { return -ENOMEM; }
        ret = nau8825_read_device_properties(dev, nau8825);
        if ret != 0 { return ret; }
    }
    i2c_set_clientdata(i2c, nau8825 as *mut c_void);
    (*nau8825).regmap = devm_regmap_init_i2c(i2c, &nau8825_regmap_config);
    if IS_ERR((*nau8825).regmap) { return PTR_ERR((*nau8825).regmap); }
    (*nau8825).dev = dev;
    (*nau8825).irq = (*i2c).irq;
    (*nau8825).xtalk_state = NAU8825_XTALK_DONE;
    (*nau8825).xtalk_protect = false;
    (*nau8825).xtalk_baktab_initialized = false;
    (*nau8825).xtalk_sem.count = 1;
    nau8825_print_device_properties(nau8825);
    nau8825_reset_chip((*nau8825).regmap);
    ret = regmap_read((*nau8825).regmap, NAU8825_REG_I2C_DEVICE_ID, &mut value);
    if ret < 0 { dev_err(dev, c"Failed to read device id from the NAU8825: %d\n".as_ptr(), ret); return ret; }
    (*nau8825).sw_id = (value as c_int) & NAU8825_SOFTWARE_ID_MASK;
    if (*nau8825).sw_id == NAU8825_SOFTWARE_ID_NAU8825 {
    } else if (*nau8825).sw_id == NAU8825_SOFTWARE_ID_NAU8825C {
        let patch = nau8825_regmap_patch();
        ret = regmap_register_patch((*nau8825).regmap, patch.as_ptr(), patch.len() as c_uint);
        if ret != 0 { dev_err(dev, c"Failed to register Rev C patch: %d\n".as_ptr(), ret); return ret; }
    } else {
        dev_err(dev, c"Not a NAU8825 chip\n".as_ptr());
        return -ENODEV;
    }
    nau8825_init_regs(nau8825);
    if (*i2c).irq != 0 { nau8825_setup_irq(nau8825); }
    devm_snd_soc_register_component(dev, &nau8825_component_driver, &mut nau8825_dai, 1)
}

static nau8825_regmap_config: regmap_config = regmap_config {
    val_bits: 16,
    reg_bits: 8,
    max_register: 0,
    readable_reg: Some(nau8825_readable_reg),
    writeable_reg: Some(nau8825_writeable_reg),
    volatile_reg: Some(nau8825_volatile_reg),
    cache_type: 0,
    reg_defaults: ptr::null(),
    num_reg_defaults: 74,
};

static nau8825_i2c_ids: [i2c_device_id; 2] = [
    i2c_device_id { name: [110, 97, 117, 56, 56, 50, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    i2c_device_id { name: [0; 20] },
];

/* CONFIG_OF: static const struct of_device_id nau8825_of_ids[] = {
 *     { .compatible = "nuvoton,nau8825", },
 *     {}
 * };
 */
static nau8825_of_ids: [of_device_id; 2] = [
    of_device_id { compatible: c"nuvoton,nau8825".as_ptr() },
    of_device_id { compatible: ptr::null() },
];

/* CONFIG_ACPI: static const struct acpi_device_id nau8825_acpi_match[] = {
 *     { "10508825", 0 },
 *     {},
 * };
 */
static nau8825_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: c"10508825".as_ptr(), driver_data: 0 },
    acpi_device_id { id: ptr::null(), driver_data: 0 },
];

static mut nau8825_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"nau8825".as_ptr(),
        of_match_table: nau8825_of_ids.as_ptr(),
        acpi_match_table: nau8825_acpi_match.as_ptr(),
    },
    probe: Some(nau8825_i2c_probe),
    id_table: nau8825_i2c_ids.as_ptr(),
};

/* module_i2c_driver(nau8825_driver);
 * MODULE_DEVICE_TABLE(i2c, nau8825_i2c_ids);
 * MODULE_DEVICE_TABLE(of, nau8825_of_ids);
 * MODULE_DEVICE_TABLE(acpi, nau8825_acpi_match);
 * MODULE_DESCRIPTION("ASoC nau8825 driver");
 * MODULE_AUTHOR("Anatol Pomozov <anatol@chromium.org>");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
