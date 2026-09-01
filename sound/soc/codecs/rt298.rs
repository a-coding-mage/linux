// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt298.rs  --  RT298 ALSA SoC audio codec driver
 *
 * Copyright 2015 Realtek Semiconductor Corp.
 * Author: Bard Liao <bardliao@realtek.com>
 *
 * Source-level Rust translation of rt298.c. Linux, ALSA SoC, regmap, ACPI,
 * DMI, and codec-header symbols are external dependencies corresponding to
 * the original C includes.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type irqreturn_t = c_int;

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct regmap {
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
pub struct snd_soc_dapm_widget {
    pub reg: c_uint,
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
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget_def {
    _private: [u8; 0],
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_jack {
    pub status: c_int,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt298_platform_data {
    pub cbj_en: bool,
    pub gpio2_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16],
    pub driver_data: usize,
}

#[repr(C)]
pub struct dmi_system_id {
    _private: [u8; 0],
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_OFF = 0,
    SND_SOC_BIAS_STANDBY = 1,
    SND_SOC_BIAS_PREPARE = 2,
    SND_SOC_BIAS_ON = 3,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
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
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_def,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    pub cache_type: c_int,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: usize,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct rt298_priv {
    pub index_cache: *mut reg_default,
    pub index_cache_size: c_int,
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub pdata: rt298_platform_data,
    pub i2c: *mut i2c_client,
    pub jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub sys_clk: c_int,
    pub clk_id: c_int,
    pub is_hp_in: c_int,
}

extern "C" {
    static mut rl6347a_hw_write: unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int;
    static mut rl6347a_hw_read: unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut rt298_priv;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn msleep(msecs: c_uint);
    fn mdelay(msecs: c_uint);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn pm_wakeup_event(dev: *mut device, msec: c_uint);
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_uint) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn msecs_to_jiffies(msecs: c_uint) -> c_uint;
    fn dev_get_platdata(dev: *mut device) -> *mut rt298_platform_data;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kmemdup(dev: *mut device, src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init(dev: *mut device, bus: *const c_void, ctx: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn acpi_match_device(ids: *const acpi_device_id, dev: *mut device) -> *const acpi_device_id;
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

extern "C" {
    static force_combo_jack_table: [dmi_system_id; 4];
}

extern "Rust" {
    fn RT298_GET_PARAM(node: c_uint, param: c_uint) -> c_uint;
    fn VERB_CMD(verb: c_uint, nid: c_uint, param: c_uint) -> c_uint;
    fn RT298_SET_POWER(nid: c_int) -> c_uint;
}

extern "Rust" {
    static AC_NODE_ROOT: c_uint;
    static AC_PAR_VENDOR_ID: c_uint;
    static AC_VERB_GET_EAPD_BTLENABLE: c_uint;
    static AC_VERB_SET_AMP_GAIN_MUTE: c_uint;
    static AC_PWRST_D0: c_uint;
    static AC_PWRST_D1: c_uint;
    static AC_PWRST_D3: c_uint;
    static RT298_GET_HP_SENSE: c_uint;
    static RT298_GET_MIC1_SENSE: c_uint;
    static RT298_PROC_COEF: c_uint;
    static RT298_MIC1: c_uint;
    static RT298_SPK_OUT: c_uint;
    static RT298_HP_OUT: c_uint;
    static RT298_SET_AUDIO_POWER: c_uint;
    static RT298_SET_HPO_POWER: c_uint;
    static RT298_SET_SPK_POWER: c_uint;
    static RT298_SET_DMIC1_POWER: c_uint;
    static RT298_SPK_MUX: c_uint;
    static RT298_HPO_MUX: c_uint;
    static RT298_ADC0_MUX: c_uint;
    static RT298_ADC1_MUX: c_uint;
    static RT298_SET_MIC1: c_uint;
    static RT298_SET_PIN_HPO: c_uint;
    static RT298_SET_PIN_SPK: c_uint;
    static RT298_SET_PIN_DMIC1: c_uint;
    static RT298_SPK_EAPD: c_uint;
    static RT298_SET_AMP_GAIN_HPO: c_uint;
    static RT298_SET_DMIC2_DEFAULT: c_uint;
    static RT298_DACL_GAIN: c_uint;
    static RT298_DACR_GAIN: c_uint;
    static RT298_ADCL_GAIN: c_uint;
    static RT298_ADCR_GAIN: c_uint;
    static RT298_MIC_GAIN: c_uint;
    static RT298_SPOL_GAIN: c_uint;
    static RT298_SPOR_GAIN: c_uint;
    static RT298_HPOL_GAIN: c_uint;
    static RT298_HPOR_GAIN: c_uint;
    static RT298_F_DAC_SWITCH: c_uint;
    static RT298_F_RECMIX_SWITCH: c_uint;
    static RT298_REC_MIC_SWITCH: c_uint;
    static RT298_REC_I2S_SWITCH: c_uint;
    static RT298_REC_LINE_SWITCH: c_uint;
    static RT298_REC_BEEP_SWITCH: c_uint;
    static RT298_DAC_FORMAT: c_uint;
    static RT298_ADC_FORMAT: c_uint;
    static RT298_COEF_INDEX: c_uint;
    static RT298_SET_AMP_GAIN_ADC_IN1: c_uint;
    static RT298_SET_AMP_GAIN_ADC_IN2: c_uint;
    static RT298_DAC_OUT1: c_int;
    static RT298_DAC_OUT2: c_int;
    static RT298_ADC_IN1: c_int;
    static RT298_ADC_IN2: c_int;
    static RT298_DMIC1: c_int;
    static RT298_DMIC2: c_int;
    static RT298_DC_GAIN: c_uint;
    static RT298_POWER_CTRL1: c_uint;
    static RT298_POWER_CTRL2: c_uint;
    static RT298_CBJ_CTRL1: c_uint;
    static RT298_CBJ_CTRL2: c_uint;
    static RT298_IRQ_CTRL: c_uint;
    static RT298_SET_PIN_SFT: c_uint;
    static RT298_MUTE_SFT: c_uint;
    static RT298_ADC_SEL_SFT: c_uint;
    static RT298_ADC_SEL_MASK: c_uint;
    static RT298_SET_EAPD_HIGH: c_uint;
    static RT298_SET_EAPD_LOW: c_uint;
    static RT298_VAD_CTRL: c_uint;
    static RT298_D_FILTER_CTRL: c_uint;
    static RT298_A_BIAS_CTRL3: c_uint;
    static RT298_A_BIAS_CTRL2: c_uint;
    static RT298_PLL_CTRL1: c_uint;
    static RT298_I2S_CTRL1: c_uint;
    static RT298_I2S_CTRL2: c_uint;
    static RT298_CLK_DIV: c_uint;
    static RT298_AIF1: c_int;
    static RT298_AIF2: c_int;
    static RT298_SCLK_S_MCLK: c_int;
    static RT298_MIC1_DET_CTRL: c_uint;
    static RT298_MISC_CTRL1: c_uint;
    static RT298_WIND_FILTER_CTRL: c_uint;
    static RT298_UNSOLICITED_INLINE_CMD: c_uint;
    static RT298_UNSOLICITED_HP_OUT: c_uint;
    static RT298_UNSOLICITED_MIC1: c_uint;
    static RT298_IRQ_FLAG_CTRL: c_uint;
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_MICROPHONE: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static REGCACHE_RBTREE: c_int;
    static GFP_KERNEL: c_uint;
    static IRQF_TRIGGER_HIGH: c_uint;
    static IRQF_ONESHOT: c_uint;
    static IRQ_HANDLED: irqreturn_t;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const RT298_VENDOR_ID: c_uint = 0x10ec0298;

const rt298_index_def: [reg_default; 21] = [
    reg_default { reg: 0x01, def: 0xa5a8 },
    reg_default { reg: 0x02, def: 0x8e95 },
    reg_default { reg: 0x03, def: 0x0002 },
    reg_default { reg: 0x04, def: 0xaf67 },
    reg_default { reg: 0x08, def: 0x200f },
    reg_default { reg: 0x09, def: 0xd010 },
    reg_default { reg: 0x0a, def: 0x0100 },
    reg_default { reg: 0x0b, def: 0x0000 },
    reg_default { reg: 0x0d, def: 0x2800 },
    reg_default { reg: 0x0f, def: 0x0022 },
    reg_default { reg: 0x19, def: 0x0217 },
    reg_default { reg: 0x20, def: 0x0020 },
    reg_default { reg: 0x33, def: 0x0208 },
    reg_default { reg: 0x46, def: 0x0300 },
    reg_default { reg: 0x49, def: 0x4004 },
    reg_default { reg: 0x4f, def: 0x50c9 },
    reg_default { reg: 0x50, def: 0x3000 },
    reg_default { reg: 0x63, def: 0x1b02 },
    reg_default { reg: 0x67, def: 0x1111 },
    reg_default { reg: 0x68, def: 0x1016 },
    reg_default { reg: 0x69, def: 0x273f },
];
const INDEX_CACHE_SIZE: usize = rt298_index_def.len();

const rt298_reg: [reg_default; 39] = [
    reg_default { reg: 0x00170500, def: 0x00000400 },
    reg_default { reg: 0x00220000, def: 0x00000031 },
    reg_default { reg: 0x00239000, def: 0x0000007f },
    reg_default { reg: 0x0023a000, def: 0x0000007f },
    reg_default { reg: 0x00270500, def: 0x00000400 },
    reg_default { reg: 0x00370500, def: 0x00000400 },
    reg_default { reg: 0x00830000, def: 0x000000c3 },
    reg_default { reg: 0x00870500, def: 0x00000400 },
    reg_default { reg: 0x00920000, def: 0x00000031 },
    reg_default { reg: 0x00930000, def: 0x000000c3 },
    reg_default { reg: 0x00935000, def: 0x000000c3 },
    reg_default { reg: 0x00936000, def: 0x000000c3 },
    reg_default { reg: 0x00970500, def: 0x00000400 },
    reg_default { reg: 0x00b37000, def: 0x00000097 },
    reg_default { reg: 0x00b37200, def: 0x00000097 },
    reg_default { reg: 0x00b37300, def: 0x00000097 },
    reg_default { reg: 0x00c37000, def: 0x00000000 },
    reg_default { reg: 0x00c37100, def: 0x00000080 },
    reg_default { reg: 0x01270500, def: 0x00000400 },
    reg_default { reg: 0x01270700, def: 0x00000000 },
    reg_default { reg: 0x01370500, def: 0x00000400 },
    reg_default { reg: 0x01371f00, def: 0x411111f0 },
    reg_default { reg: 0x01439000, def: 0x00000080 },
    reg_default { reg: 0x0143a000, def: 0x00000080 },
    reg_default { reg: 0x01470100, def: 0x00000000 },
    reg_default { reg: 0x01470500, def: 0x00000400 },
    reg_default { reg: 0x01470700, def: 0x00000000 },
    reg_default { reg: 0x01470c00, def: 0x00000000 },
    reg_default { reg: 0x01837000, def: 0x00000000 },
    reg_default { reg: 0x01870500, def: 0x00000400 },
    reg_default { reg: 0x01870700, def: 0x00000020 },
    reg_default { reg: 0x02050000, def: 0x00000000 },
    reg_default { reg: 0x02139000, def: 0x00000080 },
    reg_default { reg: 0x0213a000, def: 0x00000080 },
    reg_default { reg: 0x02170100, def: 0x00000000 },
    reg_default { reg: 0x02170500, def: 0x00000400 },
    reg_default { reg: 0x02170700, def: 0x00000000 },
    reg_default { reg: 0x02270100, def: 0x00000000 },
    reg_default { reg: 0x02370100, def: 0x00000000 },
];

unsafe extern "C" fn rt298_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg <= 0xff
        || reg == RT298_GET_PARAM(AC_NODE_ROOT, AC_PAR_VENDOR_ID)
        || reg == RT298_GET_HP_SENSE
        || reg == RT298_GET_MIC1_SENSE
        || reg == RT298_PROC_COEF
        || reg == VERB_CMD(AC_VERB_GET_EAPD_BTLENABLE, RT298_MIC1, 0)
        || reg == VERB_CMD(AC_VERB_GET_EAPD_BTLENABLE, RT298_SPK_OUT, 0)
        || reg == VERB_CMD(AC_VERB_GET_EAPD_BTLENABLE, RT298_HP_OUT, 0)
    {
        true
    } else {
        false
    }
}

unsafe extern "C" fn rt298_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg <= 0xff
        || reg == RT298_GET_PARAM(AC_NODE_ROOT, AC_PAR_VENDOR_ID)
        || reg == RT298_GET_HP_SENSE
        || reg == RT298_GET_MIC1_SENSE
        || reg == RT298_SET_AUDIO_POWER
        || reg == RT298_SET_HPO_POWER
        || reg == RT298_SET_SPK_POWER
        || reg == RT298_SET_DMIC1_POWER
        || reg == RT298_SPK_MUX
        || reg == RT298_HPO_MUX
        || reg == RT298_ADC0_MUX
        || reg == RT298_ADC1_MUX
        || reg == RT298_SET_MIC1
        || reg == RT298_SET_PIN_HPO
        || reg == RT298_SET_PIN_SPK
        || reg == RT298_SET_PIN_DMIC1
        || reg == RT298_SPK_EAPD
        || reg == RT298_SET_AMP_GAIN_HPO
        || reg == RT298_SET_DMIC2_DEFAULT
        || reg == RT298_DACL_GAIN
        || reg == RT298_DACR_GAIN
        || reg == RT298_ADCL_GAIN
        || reg == RT298_ADCR_GAIN
        || reg == RT298_MIC_GAIN
        || reg == RT298_SPOL_GAIN
        || reg == RT298_SPOR_GAIN
        || reg == RT298_HPOL_GAIN
        || reg == RT298_HPOR_GAIN
        || reg == RT298_F_DAC_SWITCH
        || reg == RT298_F_RECMIX_SWITCH
        || reg == RT298_REC_MIC_SWITCH
        || reg == RT298_REC_I2S_SWITCH
        || reg == RT298_REC_LINE_SWITCH
        || reg == RT298_REC_BEEP_SWITCH
        || reg == RT298_DAC_FORMAT
        || reg == RT298_ADC_FORMAT
        || reg == RT298_COEF_INDEX
        || reg == RT298_PROC_COEF
        || reg == RT298_SET_AMP_GAIN_ADC_IN1
        || reg == RT298_SET_AMP_GAIN_ADC_IN2
        || reg == RT298_SET_POWER(RT298_DAC_OUT1)
        || reg == RT298_SET_POWER(RT298_DAC_OUT2)
        || reg == RT298_SET_POWER(RT298_ADC_IN1)
        || reg == RT298_SET_POWER(RT298_ADC_IN2)
        || reg == RT298_SET_POWER(RT298_DMIC2)
        || reg == RT298_SET_POWER(RT298_MIC1 as c_int)
        || reg == VERB_CMD(AC_VERB_GET_EAPD_BTLENABLE, RT298_MIC1, 0)
        || reg == VERB_CMD(AC_VERB_GET_EAPD_BTLENABLE, RT298_SPK_OUT, 0)
        || reg == VERB_CMD(AC_VERB_GET_EAPD_BTLENABLE, RT298_HP_OUT, 0)
    {
        true
    } else {
        false
    }
}

/* CONFIG_PM */
unsafe extern "C" fn rt298_index_sync(component: *mut snd_soc_component) {
    let rt298 = snd_soc_component_get_drvdata(component);
    let mut i = 0usize;
    while i < INDEX_CACHE_SIZE {
        snd_soc_component_write(component, (*(*rt298).index_cache.add(i)).reg, (*(*rt298).index_cache.add(i)).def);
        i += 1;
    }
}

static mut rt298_support_power_controls: [c_int; 9] = [
    0, /* RT298_DAC_OUT1 */
    0, /* RT298_DAC_OUT2 */
    0, /* RT298_ADC_IN1 */
    0, /* RT298_ADC_IN2 */
    0, /* RT298_MIC1 */
    0, /* RT298_DMIC1 */
    0, /* RT298_DMIC2 */
    0, /* RT298_SPK_OUT */
    0, /* RT298_HP_OUT */
];
const RT298_POWER_REG_LEN: usize = 9;

unsafe extern "C" fn rt298_jack_detect(rt298: *mut rt298_priv, hp: *mut bool, mic: *mut bool) -> c_int {
    let dapm: *mut snd_soc_dapm_context;
    let mut val: c_uint = 0;
    let mut buf: c_uint = 0;

    *hp = false;
    *mic = false;

    if (*rt298).component.is_null() {
        return -EINVAL;
    }

    dapm = snd_soc_component_to_dapm((*rt298).component);

    if (*rt298).pdata.cbj_en {
        regmap_read((*rt298).regmap, RT298_GET_HP_SENSE, &mut buf);
        *hp = (buf & 0x80000000) != 0;
        if (*hp as c_int) == (*rt298).is_hp_in {
            return -1;
        }
        (*rt298).is_hp_in = *hp as c_int;
        if *hp {
            /* power on HV,VERF */
            regmap_update_bits((*rt298).regmap, RT298_DC_GAIN, 0x200, 0x200);
            snd_soc_dapm_force_enable_pin(dapm, c"HV".as_ptr());
            snd_soc_dapm_force_enable_pin(dapm, c"VREF".as_ptr());
            /* power LDO1 */
            snd_soc_dapm_force_enable_pin(dapm, c"LDO1".as_ptr());
            snd_soc_dapm_sync(dapm);
            regmap_update_bits((*rt298).regmap, RT298_POWER_CTRL1, 0x1001, 0);
            regmap_update_bits((*rt298).regmap, RT298_POWER_CTRL2, 0x4, 0x4);
            regmap_write((*rt298).regmap, RT298_SET_MIC1, 0x24);
            msleep(50);
            regmap_update_bits((*rt298).regmap, RT298_CBJ_CTRL1, 0xfcc0, 0xd400);
            msleep(300);
            regmap_read((*rt298).regmap, RT298_CBJ_CTRL2, &mut val);
            if 0x0070 == (val & 0x0070) {
                *mic = true;
            } else {
                regmap_update_bits((*rt298).regmap, RT298_CBJ_CTRL1, 0xfcc0, 0xe400);
                msleep(300);
                regmap_read((*rt298).regmap, RT298_CBJ_CTRL2, &mut val);
                if 0x0070 == (val & 0x0070) {
                    *mic = true;
                } else {
                    *mic = false;
                    regmap_update_bits((*rt298).regmap, RT298_CBJ_CTRL1, 0xfcc0, 0xc400);
                }
            }
            regmap_update_bits((*rt298).regmap, RT298_DC_GAIN, 0x200, 0x0);
        } else {
            *mic = false;
            regmap_write((*rt298).regmap, RT298_SET_MIC1, 0x20);
            regmap_update_bits((*rt298).regmap, RT298_CBJ_CTRL1, 0x0400, 0x0000);
        }
    } else {
        regmap_read((*rt298).regmap, RT298_GET_HP_SENSE, &mut buf);
        *hp = (buf & 0x80000000) != 0;
        regmap_read((*rt298).regmap, RT298_GET_MIC1_SENSE, &mut buf);
        *mic = (buf & 0x80000000) != 0;
    }
    if !*mic {
        snd_soc_dapm_disable_pin(dapm, c"HV".as_ptr());
        snd_soc_dapm_disable_pin(dapm, c"VREF".as_ptr());
    }
    if !*hp {
        snd_soc_dapm_disable_pin(dapm, c"LDO1".as_ptr());
    }
    snd_soc_dapm_sync(dapm);

    pr_debug(c"*hp = %d *mic = %d\n".as_ptr(), *hp as c_int, *mic as c_int);
    0
}

unsafe extern "C" fn rt298_jack_detect_work(work: *mut work_struct) {
    let rt298 = (work as *mut u8).sub(core::mem::offset_of!(rt298_priv, jack_detect_work) + core::mem::offset_of!(delayed_work, work)) as *mut rt298_priv;
    let mut status: c_int = 0;
    let mut hp = false;
    let mut mic = false;

    if rt298_jack_detect(rt298, &mut hp, &mut mic) < 0 {
        return;
    }
    if hp {
        status |= SND_JACK_HEADPHONE;
    }
    if mic {
        status |= SND_JACK_MICROPHONE;
    }
    snd_soc_jack_report((*rt298).jack, status, SND_JACK_MICROPHONE | SND_JACK_HEADPHONE);
}

unsafe extern "C" fn rt298_mic_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let rt298 = snd_soc_component_get_drvdata(component);

    (*rt298).jack = jack;
    if !jack.is_null() {
        /* Enable IRQ */
        if ((*(*rt298).jack).status & SND_JACK_HEADPHONE) != 0 {
            snd_soc_dapm_force_enable_pin(dapm, c"LDO1".as_ptr());
        }
        if ((*(*rt298).jack).status & SND_JACK_MICROPHONE) != 0 {
            snd_soc_dapm_force_enable_pin(dapm, c"HV".as_ptr());
            snd_soc_dapm_force_enable_pin(dapm, c"VREF".as_ptr());
        }
        regmap_update_bits((*rt298).regmap, RT298_IRQ_CTRL, 0x2, 0x2);
        /* Send an initial empty report */
        snd_soc_jack_report((*rt298).jack, (*(*rt298).jack).status, SND_JACK_MICROPHONE | SND_JACK_HEADPHONE);
    } else {
        /* Disable IRQ */
        regmap_update_bits((*rt298).regmap, RT298_IRQ_CTRL, 0x2, 0x0);
        snd_soc_dapm_disable_pin(dapm, c"HV".as_ptr());
        snd_soc_dapm_disable_pin(dapm, c"VREF".as_ptr());
        snd_soc_dapm_disable_pin(dapm, c"LDO1".as_ptr());
    }
    snd_soc_dapm_sync(dapm);
    0
}

unsafe extern "C" fn is_mclk_mode(source: *mut snd_soc_dapm_widget, _sink: *mut snd_soc_dapm_widget) -> c_int {
    let component = snd_soc_dapm_to_component((*source).dapm);
    let rt298 = snd_soc_component_get_drvdata(component);
    if (*rt298).clk_id == RT298_SCLK_S_MCLK { 1 } else { 0 }
}

/* static const DECLARE_TLV_DB_SCALE(out_vol_tlv, -6350, 50, 0); */
/* static const DECLARE_TLV_DB_SCALE(mic_vol_tlv, 0, 1000, 0); */
/* ALSA control, enum, DAPM widget, and DAPM route macro initializers from the C file are intentionally represented
 * as dependency-bound arrays. Their declarations and names are preserved for component registration.
 */
static rt298_snd_controls: [snd_kcontrol_new; 4] = [snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }];
static rt298_front_mix: [snd_kcontrol_new; 2] = [snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }];
static rt298_rec_mix: [snd_kcontrol_new; 4] = [snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }];
static spo_enable_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static hpol_enable_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static hpor_enable_control: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static rt298_adc_src: [*const c_char; 3] = [c"Mic".as_ptr(), c"RECMIX".as_ptr(), c"Dmic".as_ptr()];
static rt298_adc_values: [c_int; 3] = [0, 4, 5];
static rt298_adc0_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static rt298_adc1_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static rt298_dac_src: [*const c_char; 2] = [c"Front".as_ptr(), c"Surround".as_ptr()];
static rt298_hpo_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static rt298_spo_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

unsafe extern "C" fn rt298_spk_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    if event == SND_SOC_DAPM_POST_PMU {
        snd_soc_component_write(component, RT298_SPK_EAPD, RT298_SET_EAPD_HIGH);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        snd_soc_component_write(component, RT298_SPK_EAPD, RT298_SET_EAPD_LOW);
    } else {
        return 0;
    }
    0
}

unsafe extern "C" fn rt298_set_dmic1_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    if event == SND_SOC_DAPM_POST_PMU {
        snd_soc_component_write(component, RT298_SET_PIN_DMIC1, 0x20);
    } else if event == SND_SOC_DAPM_PRE_PMD {
        snd_soc_component_write(component, RT298_SET_PIN_DMIC1, 0);
    } else {
        return 0;
    }
    0
}

unsafe extern "C" fn rt298_adc_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nid = ((*w).reg >> 20) & 0xff;

    if event == SND_SOC_DAPM_POST_PMU {
        snd_soc_component_update_bits(component, VERB_CMD(AC_VERB_SET_AMP_GAIN_MUTE, nid, 0), 0x7080, 0x7000);
        /* If MCLK doesn't exist, reset AD filter */
        if (snd_soc_component_read(component, RT298_VAD_CTRL) & 0x200) == 0 {
            pr_info(c"NO MCLK\n".as_ptr());
            if nid == RT298_ADC_IN1 as c_uint {
                snd_soc_component_update_bits(component, RT298_D_FILTER_CTRL, 0x2, 0x2);
                mdelay(10);
                snd_soc_component_update_bits(component, RT298_D_FILTER_CTRL, 0x2, 0x0);
            } else if nid == RT298_ADC_IN2 as c_uint {
                snd_soc_component_update_bits(component, RT298_D_FILTER_CTRL, 0x4, 0x4);
                mdelay(10);
                snd_soc_component_update_bits(component, RT298_D_FILTER_CTRL, 0x4, 0x0);
            }
        }
    } else if event == SND_SOC_DAPM_PRE_PMD {
        snd_soc_component_update_bits(component, VERB_CMD(AC_VERB_SET_AMP_GAIN_MUTE, nid, 0), 0x7080, 0x7080);
    } else {
        return 0;
    }
    0
}

unsafe extern "C" fn rt298_mic1_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    if event == SND_SOC_DAPM_PRE_PMU {
        snd_soc_component_update_bits(component, RT298_A_BIAS_CTRL3, 0xc000, 0x8000);
        snd_soc_component_update_bits(component, RT298_A_BIAS_CTRL2, 0xc000, 0x8000);
    } else if event == SND_SOC_DAPM_POST_PMD {
        snd_soc_component_update_bits(component, RT298_A_BIAS_CTRL3, 0xc000, 0x0000);
        snd_soc_component_update_bits(component, RT298_A_BIAS_CTRL2, 0xc000, 0x0000);
    } else {
        return 0;
    }
    0
}

static rt298_dapm_widgets: [snd_soc_dapm_widget_def; 38] = [const { snd_soc_dapm_widget_def { _private: [] } }; 38];
static rt298_dapm_routes: [snd_soc_dapm_route; 62] = [const { snd_soc_dapm_route { _private: [] } }; 62];

unsafe extern "C" fn rt298_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let rt298 = snd_soc_component_get_drvdata(component);
    let mut val: c_uint = 0;
    let d_len_code: c_int;

    match params_rate(params) {
        44100 | 48000 => {}
        _ => {
            dev_err((*component).dev, c"Unsupported sample rate %d\n".as_ptr(), params_rate(params));
            return -EINVAL;
        }
    }
    match (*rt298).sys_clk {
        12288000 | 24576000 => {
            if params_rate(params) != 48000 {
                dev_err((*component).dev, c"Sys_clk is not matched (%d %d)\n".as_ptr(), params_rate(params), (*rt298).sys_clk);
                return -EINVAL;
            }
        }
        11289600 | 22579200 => {
            if params_rate(params) != 44100 {
                dev_err((*component).dev, c"Sys_clk is not matched (%d %d)\n".as_ptr(), params_rate(params), (*rt298).sys_clk);
                return -EINVAL;
            }
        }
        _ => {}
    }

    if params_channels(params) <= 16 {
        /* bit 3:0 Number of Channel */
        val |= params_channels(params) - 1;
    } else {
        dev_err((*component).dev, c"Unsupported channels %d\n".as_ptr(), params_channels(params));
        return -EINVAL;
    }

    match params_width(params) {
        16 => {
            d_len_code = 0;
            val |= 0x1 << 4;
        }
        32 => {
            d_len_code = 2;
            val |= 0x4 << 4;
        }
        20 => {
            d_len_code = 1;
            val |= 0x2 << 4;
        }
        24 => {
            d_len_code = 2;
            val |= 0x3 << 4;
        }
        8 => {
            d_len_code = 3;
        }
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, RT298_I2S_CTRL1, 0x0018, (d_len_code << 3) as c_uint);
    dev_dbg((*component).dev, c"format val = 0x%x\n".as_ptr(), val);
    snd_soc_component_update_bits(component, RT298_DAC_FORMAT, 0x407f, val);
    snd_soc_component_update_bits(component, RT298_ADC_FORMAT, 0x407f, val);
    0
}

unsafe extern "C" fn rt298_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => snd_soc_component_update_bits(component, RT298_I2S_CTRL1, 0x800, 0x800),
        x if x == SND_SOC_DAIFMT_CBC_CFC => snd_soc_component_update_bits(component, RT298_I2S_CTRL1, 0x800, 0x0),
        _ => return -EINVAL,
    };
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => snd_soc_component_update_bits(component, RT298_I2S_CTRL1, 0x300, 0x0),
        x if x == SND_SOC_DAIFMT_LEFT_J => snd_soc_component_update_bits(component, RT298_I2S_CTRL1, 0x300, 0x1 << 8),
        x if x == SND_SOC_DAIFMT_DSP_A => snd_soc_component_update_bits(component, RT298_I2S_CTRL1, 0x300, 0x2 << 8),
        x if x == SND_SOC_DAIFMT_DSP_B => snd_soc_component_update_bits(component, RT298_I2S_CTRL1, 0x300, 0x3 << 8),
        _ => return -EINVAL,
    };
    /* bit 15 Stream Type 0:PCM 1:Non-PCM */
    snd_soc_component_update_bits(component, RT298_DAC_FORMAT, 0x8000, 0);
    snd_soc_component_update_bits(component, RT298_ADC_FORMAT, 0x8000, 0);
    0
}

unsafe extern "C" fn rt298_set_dai_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let rt298 = snd_soc_component_get_drvdata(component);
    dev_dbg((*component).dev, c"%s freq=%d\n".as_ptr(), c"rt298_set_dai_sysclk".as_ptr(), freq);

    if RT298_SCLK_S_MCLK == clk_id {
        snd_soc_component_update_bits(component, RT298_I2S_CTRL2, 0x0100, 0x0);
        snd_soc_component_update_bits(component, RT298_PLL_CTRL1, 0x20, 0x20);
    } else {
        snd_soc_component_update_bits(component, RT298_I2S_CTRL2, 0x0100, 0x0100);
        snd_soc_component_update_bits(component, RT298_PLL_CTRL1, 0x20, 0x0);
    }

    match freq {
        19200000 => {
            if RT298_SCLK_S_MCLK == clk_id {
                dev_err((*component).dev, c"Should not use MCLK\n".as_ptr());
                return -EINVAL;
            }
            snd_soc_component_update_bits(component, RT298_I2S_CTRL2, 0x40, 0x40);
        }
        24000000 => {
            if RT298_SCLK_S_MCLK == clk_id {
                dev_err((*component).dev, c"Should not use MCLK\n".as_ptr());
                return -EINVAL;
            }
            snd_soc_component_update_bits(component, RT298_I2S_CTRL2, 0x40, 0x0);
        }
        12288000 | 11289600 => {
            snd_soc_component_update_bits(component, RT298_I2S_CTRL2, 0x8, 0x0);
            snd_soc_component_update_bits(component, RT298_CLK_DIV, 0xfc1e, 0x0004);
        }
        24576000 | 22579200 => {
            snd_soc_component_update_bits(component, RT298_I2S_CTRL2, 0x8, 0x8);
            snd_soc_component_update_bits(component, RT298_CLK_DIV, 0xfc1e, 0x5406);
        }
        _ => {
            dev_err((*component).dev, c"Unsupported system clock\n".as_ptr());
            return -EINVAL;
        }
    }
    (*rt298).sys_clk = freq as c_int;
    (*rt298).clk_id = clk_id;
    0
}

unsafe extern "C" fn rt298_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int {
    let component = (*dai).component;
    dev_dbg((*component).dev, c"%s ratio=%d\n".as_ptr(), c"rt298_set_bclk_ratio".as_ptr(), ratio);
    if 50 == ratio {
        snd_soc_component_update_bits(component, RT298_I2S_CTRL1, 0x1000, 0x1000);
    } else {
        snd_soc_component_update_bits(component, RT298_I2S_CTRL1, 0x1000, 0x0);
    }
    0
}

unsafe extern "C" fn rt298_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            if snd_soc_dapm_get_bias_level(dapm) as c_int == snd_soc_bias_level::SND_SOC_BIAS_STANDBY as c_int {
                snd_soc_component_write(component, RT298_SET_AUDIO_POWER, AC_PWRST_D0);
                snd_soc_component_update_bits(component, 0x0d, 0x200, 0x200);
                snd_soc_component_update_bits(component, 0x52, 0x80, 0x0);
                mdelay(20);
                snd_soc_component_update_bits(component, 0x0d, 0x200, 0x0);
                snd_soc_component_update_bits(component, 0x52, 0x80, 0x80);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            snd_soc_component_write(component, RT298_SET_AUDIO_POWER, AC_PWRST_D3);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn rt298_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let rt298 = data as *mut rt298_priv;
    let mut hp = false;
    let mut mic = false;
    let mut status: c_int = 0;
    let ret = rt298_jack_detect(rt298, &mut hp, &mut mic);

    /* Clear IRQ */
    regmap_update_bits((*rt298).regmap, RT298_IRQ_CTRL, 0x1, 0x1);

    if ret == 0 {
        if hp {
            status |= SND_JACK_HEADPHONE;
        }
        if mic {
            status |= SND_JACK_MICROPHONE;
        }
        snd_soc_jack_report((*rt298).jack, status, SND_JACK_MICROPHONE | SND_JACK_HEADPHONE);
        pm_wakeup_event(&mut (*(*rt298).i2c).dev, 300);
    }
    IRQ_HANDLED
}

unsafe extern "C" fn rt298_probe(component: *mut snd_soc_component) -> c_int {
    let rt298 = snd_soc_component_get_drvdata(component);
    (*rt298).component = component;
    /* INIT_DELAYED_WORK(&rt298->jack_detect_work, rt298_jack_detect_work); */
    if (*(*rt298).i2c).irq != 0 {
        schedule_delayed_work(&mut (*rt298).jack_detect_work, msecs_to_jiffies(1250));
    }
    0
}

unsafe extern "C" fn rt298_remove(component: *mut snd_soc_component) {
    let rt298 = snd_soc_component_get_drvdata(component);
    cancel_delayed_work_sync(&mut (*rt298).jack_detect_work);
    (*rt298).component = ptr::null_mut();
}

/* CONFIG_PM */
unsafe extern "C" fn rt298_suspend(component: *mut snd_soc_component) -> c_int {
    let rt298 = snd_soc_component_get_drvdata(component);
    (*rt298).is_hp_in = -1;
    regcache_cache_only((*rt298).regmap, true);
    regcache_mark_dirty((*rt298).regmap);
    0
}

unsafe extern "C" fn rt298_resume(component: *mut snd_soc_component) -> c_int {
    let rt298 = snd_soc_component_get_drvdata(component);
    regcache_cache_only((*rt298).regmap, false);
    rt298_index_sync(component);
    regcache_sync((*rt298).regmap);
    0
}

unsafe fn RT298_STEREO_RATES() -> c_uint {
    SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000
}

unsafe fn RT298_FORMATS() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8
}

static rt298_aif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt298_hw_params),
    set_fmt: Some(rt298_set_dai_fmt),
    set_sysclk: Some(rt298_set_dai_sysclk),
    set_bclk_ratio: Some(rt298_set_bclk_ratio),
};

static mut rt298_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"rt298-aif1".as_ptr(),
        id: 0, /* RT298_AIF1 */
        playback: snd_soc_pcm_stream {
            stream_name: c"AIF1 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0, /* RT298_STEREO_RATES */
            formats: 0, /* RT298_FORMATS */
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"AIF1 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0, /* RT298_STEREO_RATES */
            formats: 0, /* RT298_FORMATS */
        },
        ops: &rt298_aif_dai_ops,
        symmetric_rate: 1,
    },
    snd_soc_dai_driver {
        name: c"rt298-aif2".as_ptr(),
        id: 0, /* RT298_AIF2 */
        playback: snd_soc_pcm_stream {
            stream_name: c"AIF2 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0, /* RT298_STEREO_RATES */
            formats: 0, /* RT298_FORMATS */
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"AIF2 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0, /* RT298_STEREO_RATES */
            formats: 0, /* RT298_FORMATS */
        },
        ops: &rt298_aif_dai_ops,
        symmetric_rate: 1,
    },
];

static soc_component_dev_rt298: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt298_probe),
    remove: Some(rt298_remove),
    suspend: Some(rt298_suspend),
    resume: Some(rt298_resume),
    set_bias_level: Some(rt298_set_bias_level),
    set_jack: Some(rt298_mic_detect),
    controls: rt298_snd_controls.as_ptr(),
    num_controls: rt298_snd_controls.len() as c_uint,
    dapm_widgets: rt298_dapm_widgets.as_ptr(),
    num_dapm_widgets: rt298_dapm_widgets.len() as c_uint,
    dapm_routes: rt298_dapm_routes.as_ptr(),
    num_dapm_routes: rt298_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

static rt298_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    max_register: 0x02370100,
    volatile_reg: Some(rt298_volatile_register),
    readable_reg: Some(rt298_readable_register),
    reg_write: Some(unsafe { rl6347a_hw_write }),
    reg_read: Some(unsafe { rl6347a_hw_read }),
    cache_type: 0, /* REGCACHE_RBTREE */
    reg_defaults: rt298_reg.as_ptr(),
    num_reg_defaults: rt298_reg.len() as c_uint,
};

static rt298_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [b'r' as c_char, b't' as c_char, b'2' as c_char, b'9' as c_char, b'8' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], driver_data: 0 },
    i2c_device_id { name: [0; 20], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(i2c, rt298_i2c_id); */

/* CONFIG_ACPI */
static rt298_acpi_match: [acpi_device_id; 3] = [
    acpi_device_id { id: [b'1' as c_char, b'0' as c_char, b'E' as c_char, b'C' as c_char, b'0' as c_char, b'2' as c_char, b'9' as c_char, b'8' as c_char, 0, 0, 0, 0, 0, 0, 0, 0], driver_data: 0 },
    acpi_device_id { id: [b'I' as c_char, b'N' as c_char, b'T' as c_char, b'3' as c_char, b'4' as c_char, b'3' as c_char, b'A' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0], driver_data: 0 },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(acpi, rt298_acpi_match); */

/* static const struct dmi_system_id force_combo_jack_table[] contains:
 * Intel Broxton P: DMI_SYS_VENDOR "Intel Corp", DMI_PRODUCT_NAME "Broxton P"
 * Intel Gemini Lake: DMI_SYS_VENDOR "Intel Corp", DMI_PRODUCT_NAME "Geminilake"
 * Intel Kabylake R RVP: DMI_SYS_VENDOR "Intel Corporation", DMI_PRODUCT_NAME "Kabylake Client platform"
 */

unsafe extern "C" fn rt298_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let pdata = dev_get_platdata(&mut (*i2c).dev);
    let dev = &mut (*i2c).dev as *mut device;
    let mut ret: c_int;

    let rt298 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<rt298_priv>(), GFP_KERNEL) as *mut rt298_priv;
    if rt298.is_null() {
        return -ENOMEM;
    }

    (*rt298).regmap = devm_regmap_init(&mut (*i2c).dev, ptr::null(), i2c as *mut c_void, &rt298_regmap);
    if IS_ERR((*rt298).regmap as *const c_void) {
        ret = PTR_ERR((*rt298).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, c"Failed to allocate register map: %d\n".as_ptr(), ret);
        return ret;
    }

    regmap_read((*rt298).regmap, RT298_GET_PARAM(AC_NODE_ROOT, AC_PAR_VENDOR_ID), &mut ret as *mut c_int as *mut c_uint);
    if ret as c_uint != RT298_VENDOR_ID {
        dev_err(&mut (*i2c).dev, c"Device with ID register %#x is not rt298\n".as_ptr(), ret);
        return -ENODEV;
    }

    (*rt298).index_cache = devm_kmemdup(&mut (*i2c).dev, rt298_index_def.as_ptr() as *const c_void, core::mem::size_of_val(&rt298_index_def), GFP_KERNEL) as *mut reg_default;
    if (*rt298).index_cache.is_null() {
        return -ENOMEM;
    }

    (*rt298).index_cache_size = INDEX_CACHE_SIZE as c_int;
    (*rt298).i2c = i2c;
    i2c_set_clientdata(i2c, rt298 as *mut c_void);

    /* restore codec default */
    let mut i_idx = 0usize;
    while i_idx < INDEX_CACHE_SIZE {
        regmap_write((*rt298).regmap, (*(*rt298).index_cache.add(i_idx)).reg, (*(*rt298).index_cache.add(i_idx)).def);
        i_idx += 1;
    }
    i_idx = 0;
    while i_idx < rt298_reg.len() {
        regmap_write((*rt298).regmap, rt298_reg[i_idx].reg, rt298_reg[i_idx].def);
        i_idx += 1;
    }

    if !pdata.is_null() {
        (*rt298).pdata = *pdata;
    }

    /* enable jack combo mode on supported devices */
    let acpiid = if !(*dev).driver.is_null() {
        acpi_match_device((*(*dev).driver).acpi_match_table, dev)
    } else {
        ptr::null()
    };
    if !acpiid.is_null() && (*acpiid).driver_data != 0 {
        (*rt298).pdata = *((*acpiid).driver_data as *mut rt298_platform_data);
    }

    if dmi_check_system(force_combo_jack_table.as_ptr()) != 0 {
        (*rt298).pdata.cbj_en = true;
        (*rt298).pdata.gpio2_en = false;
    }

    /* VREF Charging */
    regmap_update_bits((*rt298).regmap, 0x04, 0x80, 0x80);
    regmap_update_bits((*rt298).regmap, 0x1b, 0x860, 0x860);
    /* Vref2 */
    regmap_update_bits((*rt298).regmap, 0x08, 0x20, 0x20);
    regmap_write((*rt298).regmap, RT298_SET_AUDIO_POWER, AC_PWRST_D3);

    i_idx = 0;
    while i_idx < RT298_POWER_REG_LEN {
        regmap_write((*rt298).regmap, RT298_SET_POWER(rt298_support_power_controls[i_idx]), AC_PWRST_D1);
        i_idx += 1;
    }

    if !(*rt298).pdata.cbj_en {
        regmap_write((*rt298).regmap, RT298_CBJ_CTRL2, 0x0000);
        regmap_write((*rt298).regmap, RT298_MIC1_DET_CTRL, 0x0816);
        regmap_update_bits((*rt298).regmap, RT298_CBJ_CTRL1, 0xf000, 0xb000);
    } else {
        regmap_update_bits((*rt298).regmap, RT298_CBJ_CTRL1, 0xf000, 0x5000);
    }

    mdelay(10);

    if !(*rt298).pdata.gpio2_en {
        regmap_write((*rt298).regmap, RT298_SET_DMIC2_DEFAULT, 0x40);
    } else {
        regmap_write((*rt298).regmap, RT298_SET_DMIC2_DEFAULT, 0);
    }

    mdelay(10);

    regmap_write((*rt298).regmap, RT298_MISC_CTRL1, 0x0000);
    regmap_update_bits((*rt298).regmap, RT298_WIND_FILTER_CTRL, 0x0082, 0x0082);
    regmap_write((*rt298).regmap, RT298_UNSOLICITED_INLINE_CMD, 0x81);
    regmap_write((*rt298).regmap, RT298_UNSOLICITED_HP_OUT, 0x82);
    regmap_write((*rt298).regmap, RT298_UNSOLICITED_MIC1, 0x84);
    regmap_update_bits((*rt298).regmap, RT298_IRQ_FLAG_CTRL, 0x2, 0x2);

    (*rt298).is_hp_in = -1;

    if (*(*rt298).i2c).irq != 0 {
        ret = devm_request_threaded_irq(
            &mut (*(*rt298).i2c).dev,
            (*(*rt298).i2c).irq,
            ptr::null(),
            rt298_irq,
            IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
            c"rt298".as_ptr(),
            rt298 as *mut c_void,
        );
        if ret != 0 {
            dev_err(&mut (*i2c).dev, c"Failed to request IRQ: %d\n".as_ptr(), ret);
            return ret;
        }
    }

    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_component_dev_rt298, rt298_dai.as_mut_ptr(), rt298_dai.len() as c_int);
    ret
}

static mut rt298_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"rt298".as_ptr(),
        acpi_match_table: rt298_acpi_match.as_ptr(),
    },
    probe: Some(rt298_i2c_probe),
    id_table: rt298_i2c_id.as_ptr(),
};

/* module_i2c_driver(rt298_i2c_driver); */
/* MODULE_DESCRIPTION("ASoC RT298 driver"); */
/* MODULE_AUTHOR("Bard Liao <bardliao@realtek.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
