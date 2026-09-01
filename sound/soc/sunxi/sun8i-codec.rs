// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This driver supports the digital controls for the internal codec
 * found in Allwinner's A33 SoCs.
 *
 * (C) Copyright 2010-2016
 * Reuuimlla Technology Co., Ltd. <www.reuuimllatech.com>
 * huangxin <huangxin@Reuuimllatech.com>
 * Mylène Josserand <mylene.josserand@free-electrons.com>
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u8 = core::ffi::c_uchar;
type u32 = c_uint;
type s64 = i64;
type ktime_t = i64;
type irqreturn_t = c_int;

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_ONESHOT: c_uint = 0x00002000;
const GFP_KERNEL: c_uint = 0;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_card { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub name: *const c_char,
    pub id: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
    pub sig_bits: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub ops: *const snd_soc_dai_ops,
    pub capture: snd_soc_pcm_stream,
    pub playback: snd_soc_pcm_stream,
    pub symmetric_rate: bool_,
    pub symmetric_channels: bool_,
    pub symmetric_sample_bits: bool_,
}

#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub sname: *const c_char,
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)] pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)] pub struct snd_pcm_hw_constraint_list {
    pub list: *const c_uint,
    pub count: c_uint,
    pub mask: c_uint,
}
#[repr(C)] pub struct snd_soc_component_driver {
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub idle_bias_on: c_uint,
    pub suspend_bias_off: c_uint,
    pub endianness: c_uint,
}
#[repr(C)] pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub max_register: c_uint,
    pub cache_type: c_uint,
}
#[repr(C)] pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}
#[repr(C)] pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}
#[repr(C)] pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

extern "C" {
    static mut system_power_efficient_wq: *mut workqueue_struct;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_set_rate_exclusive(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_rate_exclusive_put(clk: *mut clk);
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widgets: *const snd_soc_dapm_widget, num: c_uint) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, routes: *const snd_soc_dapm_route, num: c_uint) -> c_int;
    fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn ktime_get() -> ktime_t;
    fn ktime_add_ms(kt: ktime_t, ms: s64) -> ktime_t;
    fn ktime_after(cmp1: ktime_t, cmp2: ktime_t) -> bool_;
    fn ktime_ms_delta(later: ktime_t, earlier: ktime_t) -> s64;
    fn msecs_to_jiffies(m: s64) -> c_uint;
    fn queue_delayed_work(wq: *mut workqueue_struct, work: *mut delayed_work, delay: c_uint) -> bool_;
    fn mod_delayed_work(wq: *mut workqueue_struct, work: *mut delayed_work, delay: c_uint) -> bool_;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool_;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn devm_free_irq(dev: *mut device, irq: c_int, dev_id: *mut c_void);
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool_;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool_;
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn mutex_init(lock: *mut mutex);
}

macro_rules! BIT { ($n:expr) => { 1u32 << ($n) }; }
macro_rules! GENMASK { ($h:expr, $l:expr) => { (((!0u32) << ($l)) & ((!0u32) >> (31 - ($h)))) }; }
macro_rules! ARRAY_SIZE { ($a:expr) => { ($a.len() as c_uint) }; }
macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }

const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 1;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 1;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 2;
const SND_SOC_DAIFMT_DSP_A: c_uint = 3;
const SND_SOC_DAIFMT_DSP_B: c_uint = 4;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 1;
const SND_SOC_DAIFMT_IB_NF: c_uint = 2;
const SND_SOC_DAIFMT_IB_IF: c_uint = 3;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S20_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << 4;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << 5;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 1;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_176400: c_uint = 1 << 3;
const SNDRV_PCM_RATE_192000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 5;
const SND_JACK_HEADPHONE: c_int = 1 << 0;
const SND_JACK_MICROPHONE: c_int = 1 << 1;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_int = 1 << 2;
const SND_JACK_BTN_1: c_int = 1 << 3;
const SND_JACK_BTN_2: c_int = 1 << 4;
const SND_JACK_BTN_3: c_int = 1 << 5;
const SND_SOC_NOPM: c_uint = 0;
const snd_soc_dapm_aif_out: c_int = 0;
const SND_SOC_DAPM_PRE_PMU: c_int = 1 << 0;
const SND_SOC_DAPM_POST_PMD: c_int = 1 << 1;
const REGCACHE_FLAT: c_uint = 0;

const SUN8I_SYSCLK_CTL: c_uint = 0x00c;
const SUN8I_SYSCLK_CTL_AIF1CLK_ENA: c_uint = 11;
const SUN8I_SYSCLK_CTL_AIF1CLK_SRC_PLL: c_uint = 0x2 << 8;
const SUN8I_SYSCLK_CTL_AIF2CLK_ENA: c_uint = 7;
const SUN8I_SYSCLK_CTL_AIF2CLK_SRC_PLL: c_uint = 0x2 << 4;
const SUN8I_SYSCLK_CTL_SYSCLK_ENA: c_uint = 3;
const SUN8I_SYSCLK_CTL_SYSCLK_SRC: c_uint = 0;
const SUN8I_SYSCLK_CTL_SYSCLK_SRC_AIF1CLK: c_uint = 0x0 << 0;
const SUN8I_SYSCLK_CTL_SYSCLK_SRC_AIF2CLK: c_uint = 0x1 << 0;
const SUN8I_MOD_CLK_ENA: c_uint = 0x010;
const SUN8I_MOD_CLK_ENA_AIF1: c_uint = 15;
const SUN8I_MOD_CLK_ENA_AIF2: c_uint = 14;
const SUN8I_MOD_CLK_ENA_AIF3: c_uint = 13;
const SUN8I_MOD_CLK_ENA_ADC: c_uint = 3;
const SUN8I_MOD_CLK_ENA_DAC: c_uint = 2;
const SUN8I_MOD_RST_CTL: c_uint = 0x014;
const SUN8I_MOD_RST_CTL_AIF1: c_uint = 15;
const SUN8I_MOD_RST_CTL_AIF2: c_uint = 14;
const SUN8I_MOD_RST_CTL_AIF3: c_uint = 13;
const SUN8I_MOD_RST_CTL_ADC: c_uint = 3;
const SUN8I_MOD_RST_CTL_DAC: c_uint = 2;
const SUN8I_SYS_SR_CTRL: c_uint = 0x018;
const SUN8I_SYS_SR_CTRL_AIF1_FS: c_uint = 12;
const SUN8I_SYS_SR_CTRL_AIF2_FS: c_uint = 8;
const fn SUN8I_AIF_CLK_CTRL(n: c_int) -> c_uint { 0x040 * (1 + n as c_uint) }
const SUN8I_AIF_CLK_CTRL_MSTR_MOD: c_uint = 15;
const SUN8I_AIF_CLK_CTRL_CLK_INV: c_uint = 13;
const SUN8I_AIF_CLK_CTRL_BCLK_DIV: c_uint = 9;
const SUN8I_AIF_CLK_CTRL_LRCK_DIV: c_uint = 6;
const SUN8I_AIF_CLK_CTRL_WORD_SIZ: c_uint = 4;
const SUN8I_AIF_CLK_CTRL_DATA_FMT: c_uint = 2;
const SUN8I_AIF1_ADCDAT_CTRL: c_uint = 0x044;
const SUN8I_AIF1_ADCDAT_CTRL_AIF1_AD0L_ENA: c_uint = 15;
const SUN8I_AIF1_ADCDAT_CTRL_AIF1_AD0R_ENA: c_uint = 14;
const SUN8I_AIF1_ADCDAT_CTRL_AIF1_AD0L_SRC: c_uint = 10;
const SUN8I_AIF1_ADCDAT_CTRL_AIF1_AD0R_SRC: c_uint = 8;
const SUN8I_AIF1_DACDAT_CTRL: c_uint = 0x048;
const SUN8I_AIF1_DACDAT_CTRL_AIF1_DA0L_ENA: c_uint = 15;
const SUN8I_AIF1_DACDAT_CTRL_AIF1_DA0R_ENA: c_uint = 14;
const SUN8I_AIF1_DACDAT_CTRL_AIF1_DA0L_SRC: c_uint = 10;
const SUN8I_AIF1_DACDAT_CTRL_AIF1_DA0R_SRC: c_uint = 8;
const SUN8I_AIF1_MXR_SRC: c_uint = 0x04c;
const SUN8I_AIF1_MXR_SRC_AD0L_MXR_SRC_AIF1DA0L: c_uint = 15;
const SUN8I_AIF1_MXR_SRC_AD0L_MXR_SRC_AIF2DACL: c_uint = 14;
const SUN8I_AIF1_MXR_SRC_AD0L_MXR_SRC_ADCL: c_uint = 13;
const SUN8I_AIF1_MXR_SRC_AD0L_MXR_SRC_AIF2DACR: c_uint = 12;
const SUN8I_AIF1_MXR_SRC_AD0R_MXR_SRC_AIF1DA0R: c_uint = 11;
const SUN8I_AIF1_MXR_SRC_AD0R_MXR_SRC_AIF2DACR: c_uint = 10;
const SUN8I_AIF1_MXR_SRC_AD0R_MXR_SRC_ADCR: c_uint = 9;
const SUN8I_AIF1_MXR_SRC_AD0R_MXR_SRC_AIF2DACL: c_uint = 8;
const SUN8I_AIF1_VOL_CTRL1: c_uint = 0x050;
const SUN8I_AIF1_VOL_CTRL1_AD0L_VOL: c_uint = 8;
const SUN8I_AIF1_VOL_CTRL1_AD0R_VOL: c_uint = 0;
const SUN8I_AIF1_VOL_CTRL3: c_uint = 0x058;
const SUN8I_AIF1_VOL_CTRL3_DA0L_VOL: c_uint = 8;
const SUN8I_AIF1_VOL_CTRL3_DA0R_VOL: c_uint = 0;
const SUN8I_AIF2_ADCDAT_CTRL: c_uint = 0x084;
const SUN8I_AIF2_ADCDAT_CTRL_AIF2_ADCL_ENA: c_uint = 15;
const SUN8I_AIF2_ADCDAT_CTRL_AIF2_ADCR_ENA: c_uint = 14;
const SUN8I_AIF2_ADCDAT_CTRL_AIF2_ADCL_SRC: c_uint = 10;
const SUN8I_AIF2_ADCDAT_CTRL_AIF2_ADCR_SRC: c_uint = 8;
const SUN8I_AIF2_DACDAT_CTRL: c_uint = 0x088;
const SUN8I_AIF2_DACDAT_CTRL_AIF2_DACL_ENA: c_uint = 15;
const SUN8I_AIF2_DACDAT_CTRL_AIF2_DACR_ENA: c_uint = 14;
const SUN8I_AIF2_DACDAT_CTRL_AIF2_DACL_SRC: c_uint = 10;
const SUN8I_AIF2_DACDAT_CTRL_AIF2_DACR_SRC: c_uint = 8;
const SUN8I_AIF2_MXR_SRC: c_uint = 0x08c;
const SUN8I_AIF2_MXR_SRC_ADCL_MXR_SRC_AIF1DA0L: c_uint = 15;
const SUN8I_AIF2_MXR_SRC_ADCL_MXR_SRC_AIF1DA1L: c_uint = 14;
const SUN8I_AIF2_MXR_SRC_ADCL_MXR_SRC_AIF2DACR: c_uint = 13;
const SUN8I_AIF2_MXR_SRC_ADCL_MXR_SRC_ADCL: c_uint = 12;
const SUN8I_AIF2_MXR_SRC_ADCR_MXR_SRC_AIF1DA0R: c_uint = 11;
const SUN8I_AIF2_MXR_SRC_ADCR_MXR_SRC_AIF1DA1R: c_uint = 10;
const SUN8I_AIF2_MXR_SRC_ADCR_MXR_SRC_AIF2DACL: c_uint = 9;
const SUN8I_AIF2_MXR_SRC_ADCR_MXR_SRC_ADCR: c_uint = 8;
const SUN8I_AIF2_VOL_CTRL1: c_uint = 0x090;
const SUN8I_AIF2_VOL_CTRL1_ADCL_VOL: c_uint = 8;
const SUN8I_AIF2_VOL_CTRL1_ADCR_VOL: c_uint = 0;
const SUN8I_AIF2_VOL_CTRL2: c_uint = 0x098;
const SUN8I_AIF2_VOL_CTRL2_DACL_VOL: c_uint = 8;
const SUN8I_AIF2_VOL_CTRL2_DACR_VOL: c_uint = 0;
const SUN8I_AIF3_CLK_CTRL_AIF3_CLK_SRC_AIF1: c_uint = 0x0 << 0;
const SUN8I_AIF3_CLK_CTRL_AIF3_CLK_SRC_AIF2: c_uint = 0x1 << 0;
const SUN8I_AIF3_CLK_CTRL_AIF3_CLK_SRC_AIF1CLK: c_uint = 0x2 << 0;
const SUN8I_AIF3_PATH_CTRL: c_uint = 0x0cc;
const SUN8I_AIF3_PATH_CTRL_AIF3_ADC_SRC: c_uint = 10;
const SUN8I_AIF3_PATH_CTRL_AIF2_DAC_SRC: c_uint = 8;
const SUN8I_AIF3_PATH_CTRL_AIF3_PINS_TRI: c_uint = 7;
const SUN8I_ADC_DIG_CTRL: c_uint = 0x100;
const SUN8I_ADC_DIG_CTRL_ENAD: c_uint = 15;
const SUN8I_ADC_DIG_CTRL_ADOUT_DTS: c_uint = 2;
const SUN8I_ADC_DIG_CTRL_ADOUT_DLY: c_uint = 1;
const SUN8I_ADC_VOL_CTRL: c_uint = 0x104;
const SUN8I_ADC_VOL_CTRL_ADCL_VOL: c_uint = 8;
const SUN8I_ADC_VOL_CTRL_ADCR_VOL: c_uint = 0;
const SUN8I_HMIC_CTRL1: c_uint = 0x110;
const SUN8I_HMIC_CTRL1_HMIC_M: c_uint = 12;
const SUN8I_HMIC_CTRL1_HMIC_N: c_uint = 8;
const SUN8I_HMIC_CTRL1_MDATA_THRESHOLD_DB: c_uint = 5;
const SUN8I_HMIC_CTRL1_JACK_OUT_IRQ_EN: c_uint = 4;
const SUN8I_HMIC_CTRL1_JACK_IN_IRQ_EN: c_uint = 3;
const SUN8I_HMIC_CTRL1_HMIC_DATA_IRQ_EN: c_uint = 0;
const SUN8I_HMIC_CTRL2: c_uint = 0x114;
const SUN8I_HMIC_CTRL2_HMIC_SAMPLE: c_uint = 14;
const SUN8I_HMIC_CTRL2_HMIC_MDATA_THRESHOLD: c_uint = 8;
const SUN8I_HMIC_CTRL2_HMIC_SF: c_uint = 6;
const SUN8I_HMIC_STS: c_uint = 0x118;
const SUN8I_HMIC_STS_MDATA_DISCARD: c_uint = 13;
const SUN8I_HMIC_STS_HMIC_DATA: c_uint = 8;
const SUN8I_HMIC_STS_JACK_OUT_IRQ_ST: c_uint = 4;
const SUN8I_HMIC_STS_JACK_IN_IRQ_ST: c_uint = 3;
const SUN8I_HMIC_STS_HMIC_DATA_IRQ_ST: c_uint = 0;
const SUN8I_DAC_DIG_CTRL: c_uint = 0x120;
const SUN8I_DAC_DIG_CTRL_ENDA: c_uint = 15;
const SUN8I_DAC_VOL_CTRL: c_uint = 0x124;
const SUN8I_DAC_VOL_CTRL_DACL_VOL: c_uint = 8;
const SUN8I_DAC_VOL_CTRL_DACR_VOL: c_uint = 0;
const SUN8I_DAC_MXR_SRC: c_uint = 0x130;
const SUN8I_DAC_MXR_SRC_DACL_MXR_SRC_AIF1DA0L: c_uint = 15;
const SUN8I_DAC_MXR_SRC_DACL_MXR_SRC_AIF1DA1L: c_uint = 14;
const SUN8I_DAC_MXR_SRC_DACL_MXR_SRC_AIF2DACL: c_uint = 13;
const SUN8I_DAC_MXR_SRC_DACL_MXR_SRC_ADCL: c_uint = 12;
const SUN8I_DAC_MXR_SRC_DACR_MXR_SRC_AIF1DA0R: c_uint = 11;
const SUN8I_DAC_MXR_SRC_DACR_MXR_SRC_AIF1DA1R: c_uint = 10;
const SUN8I_DAC_MXR_SRC_DACR_MXR_SRC_AIF2DACR: c_uint = 9;
const SUN8I_DAC_MXR_SRC_DACR_MXR_SRC_ADCR: c_uint = 8;
const SUN8I_SYSCLK_CTL_AIF1CLK_SRC_MASK: c_uint = GENMASK!(9, 8);
const SUN8I_SYSCLK_CTL_AIF2CLK_SRC_MASK: c_uint = GENMASK!(5, 4);
const SUN8I_SYS_SR_CTRL_AIF1_FS_MASK: c_uint = GENMASK!(15, 12);
const SUN8I_SYS_SR_CTRL_AIF2_FS_MASK: c_uint = GENMASK!(11, 8);
const SUN8I_AIF_CLK_CTRL_CLK_INV_MASK: c_uint = GENMASK!(14, 13);
const SUN8I_AIF_CLK_CTRL_BCLK_DIV_MASK: c_uint = GENMASK!(12, 9);
const SUN8I_AIF_CLK_CTRL_LRCK_DIV_MASK: c_uint = GENMASK!(8, 6);
const SUN8I_AIF_CLK_CTRL_WORD_SIZ_MASK: c_uint = GENMASK!(5, 4);
const SUN8I_AIF_CLK_CTRL_DATA_FMT_MASK: c_uint = GENMASK!(3, 2);
const SUN8I_AIF3_CLK_CTRL_AIF3_CLK_SRC_MASK: c_uint = GENMASK!(1, 0);
const SUN8I_HMIC_CTRL1_HMIC_M_MASK: c_uint = GENMASK!(15, 12);
const SUN8I_HMIC_CTRL1_HMIC_N_MASK: c_uint = GENMASK!(11, 8);
const SUN8I_HMIC_CTRL1_MDATA_THRESHOLD_DB_MASK: c_uint = GENMASK!(6, 5);
const SUN8I_HMIC_CTRL2_HMIC_SAMPLE_MASK: c_uint = GENMASK!(15, 14);
const SUN8I_HMIC_CTRL2_HMIC_SF_MASK: c_uint = GENMASK!(7, 6);
const SUN8I_HMIC_STS_HMIC_DATA_MASK: c_uint = GENMASK!(12, 8);
const SUN8I_CODEC_BUTTONS: c_int = SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3;
const SUN8I_CODEC_PASSTHROUGH_SAMPLE_RATE: c_uint = 48000;
const SUN8I_CODEC_PCM_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_3LE;
const SUN8I_CODEC_PCM_RATES: c_uint = SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_192000 | SNDRV_PCM_RATE_KNOT;

const SUN8I_CODEC_AIF1: c_int = 0;
const SUN8I_CODEC_AIF2: c_int = 1;
const SUN8I_CODEC_AIF3: c_int = 2;
const SUN8I_CODEC_NAIFS: usize = 3;
const SUN8I_JACK_STATUS_DISCONNECTED: c_int = 0;
const SUN8I_JACK_STATUS_WAITING_HBIAS: c_int = 1;
const SUN8I_JACK_STATUS_CONNECTED: c_int = 2;

#[repr(C)]
pub struct sun8i_codec_aif {
    pub lrck_div_order: c_uint,
    pub sample_rate: c_uint,
    pub slots: c_uint,
    pub slot_width: c_uint,
    pub active_streams: c_uint,
    pub open_streams: c_uint,
}

#[repr(C)]
pub struct sun8i_codec_quirks {
    pub bus_clock: bool_,
    pub jack_detection: bool_,
    pub legacy_widgets: bool_,
    pub lrck_inversion: bool_,
}

#[repr(C)]
pub struct sun8i_codec {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub clk_bus: *mut clk,
    pub clk_module: *mut clk,
    pub quirks: *const sun8i_codec_quirks,
    pub aifs: [sun8i_codec_aif; SUN8I_CODEC_NAIFS],
    pub jack: *mut snd_soc_jack,
    pub jack_work: delayed_work,
    pub jack_irq: c_int,
    pub jack_status: c_int,
    pub jack_type: c_int,
    pub jack_last_sample: c_int,
    pub jack_hbias_ready: ktime_t,
    pub jack_mutex: mutex,
    pub last_hmic_irq: c_int,
    pub sysclk_rate: c_uint,
    pub sysclk_refcnt: c_int,
}

static mut sun8i_codec_dais: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver { name: cstr!("sun8i-codec-aif1"), id: SUN8I_CODEC_AIF1, ops: unsafe { &sun8i_codec_dai_ops }, capture: snd_soc_pcm_stream { stream_name: cstr!("AIF1 Capture"), channels_min: 1, channels_max: 2, rates: SUN8I_CODEC_PCM_RATES, formats: SUN8I_CODEC_PCM_FORMATS, sig_bits: 24 }, playback: snd_soc_pcm_stream { stream_name: cstr!("AIF1 Playback"), channels_min: 1, channels_max: 2, rates: SUN8I_CODEC_PCM_RATES, formats: SUN8I_CODEC_PCM_FORMATS, sig_bits: 0 }, symmetric_rate: true, symmetric_channels: true, symmetric_sample_bits: true },
    snd_soc_dai_driver { name: cstr!("sun8i-codec-aif2"), id: SUN8I_CODEC_AIF2, ops: unsafe { &sun8i_codec_dai_ops }, capture: snd_soc_pcm_stream { stream_name: cstr!("AIF2 Capture"), channels_min: 1, channels_max: 2, rates: SUN8I_CODEC_PCM_RATES, formats: SUN8I_CODEC_PCM_FORMATS, sig_bits: 24 }, playback: snd_soc_pcm_stream { stream_name: cstr!("AIF2 Playback"), channels_min: 1, channels_max: 2, rates: SUN8I_CODEC_PCM_RATES, formats: SUN8I_CODEC_PCM_FORMATS, sig_bits: 0 }, symmetric_rate: true, symmetric_channels: true, symmetric_sample_bits: true },
    snd_soc_dai_driver { name: cstr!("sun8i-codec-aif3"), id: SUN8I_CODEC_AIF3, ops: unsafe { &sun8i_codec_dai_ops }, capture: snd_soc_pcm_stream { stream_name: cstr!("AIF3 Capture"), channels_min: 1, channels_max: 1, rates: SUN8I_CODEC_PCM_RATES, formats: SUN8I_CODEC_PCM_FORMATS, sig_bits: 24 }, playback: snd_soc_pcm_stream { stream_name: cstr!("AIF3 Playback"), channels_min: 1, channels_max: 1, rates: SUN8I_CODEC_PCM_RATES, formats: SUN8I_CODEC_PCM_FORMATS, sig_bits: 0 }, symmetric_rate: true, symmetric_channels: true, symmetric_sample_bits: true },
];

unsafe extern "C" fn sun8i_codec_runtime_resume(dev: *mut device) -> c_int {
    let scodec = dev_get_drvdata(dev) as *mut sun8i_codec;
    let mut ret = clk_prepare_enable((*scodec).clk_bus);
    if ret != 0 {
        dev_err(dev, cstr!("Failed to enable the bus clock\n"));
        return ret;
    }
    regcache_cache_only((*scodec).regmap, false);
    ret = regcache_sync((*scodec).regmap);
    if ret != 0 {
        dev_err(dev, cstr!("Failed to sync regmap cache\n"));
        return ret;
    }
    0
}

unsafe extern "C" fn sun8i_codec_runtime_suspend(dev: *mut device) -> c_int {
    let scodec = dev_get_drvdata(dev) as *mut sun8i_codec;
    regcache_cache_only((*scodec).regmap, true);
    regcache_mark_dirty((*scodec).regmap);
    clk_disable_unprepare((*scodec).clk_bus);
    0
}

fn sun8i_codec_get_hw_rate(sample_rate: c_uint) -> c_int {
    match sample_rate {
        7350 | 8000 => 0x0,
        11025 => 0x1,
        12000 => 0x2,
        14700 | 16000 => 0x3,
        22050 => 0x4,
        24000 => 0x5,
        29400 | 32000 => 0x6,
        44100 => 0x7,
        48000 => 0x8,
        88200 | 96000 => 0x9,
        176400 | 192000 => 0xa,
        _ => -EINVAL,
    }
}

unsafe fn sun8i_codec_update_sample_rate(scodec: *mut sun8i_codec) -> c_int {
    let mut max_rate: c_uint = 0;
    for i in SUN8I_CODEC_AIF1 as usize..SUN8I_CODEC_NAIFS {
        let aif = &mut (*scodec).aifs[i];
        if aif.active_streams != 0 {
            max_rate = core::cmp::max(max_rate, aif.sample_rate);
        }
    }
    if max_rate == 0 {
        max_rate = SUN8I_CODEC_PASSTHROUGH_SAMPLE_RATE;
    }
    let hw_rate = sun8i_codec_get_hw_rate(max_rate);
    if hw_rate < 0 {
        return hw_rate;
    }
    regmap_update_bits((*scodec).regmap, SUN8I_SYS_SR_CTRL, SUN8I_SYS_SR_CTRL_AIF1_FS_MASK, (hw_rate as c_uint) << SUN8I_SYS_SR_CTRL_AIF1_FS);
    0
}

unsafe extern "C" fn sun8i_codec_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let scodec = snd_soc_dai_get_drvdata(dai) as *mut sun8i_codec;
    let dsp_format: c_uint;
    let format: c_uint;
    let mut invert: c_uint;
    let value: c_uint;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => value = 0x1,
        SND_SOC_DAIFMT_CBP_CFP => value = 0x0,
        _ => return -EINVAL,
    }
    if (*dai).id == SUN8I_CODEC_AIF3 {
        if value != 0 { return -EINVAL; }
        regmap_update_bits((*scodec).regmap, SUN8I_AIF_CLK_CTRL((*dai).id), SUN8I_AIF3_CLK_CTRL_AIF3_CLK_SRC_MASK, SUN8I_AIF3_CLK_CTRL_AIF3_CLK_SRC_AIF2);
    } else {
        regmap_update_bits((*scodec).regmap, SUN8I_AIF_CLK_CTRL((*dai).id), BIT!(SUN8I_AIF_CLK_CTRL_MSTR_MOD), value << SUN8I_AIF_CLK_CTRL_MSTR_MOD);
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => { format = 0x0; dsp_format = 0; }
        SND_SOC_DAIFMT_LEFT_J => { format = 0x1; dsp_format = 0; }
        SND_SOC_DAIFMT_RIGHT_J => { format = 0x2; dsp_format = 0; }
        SND_SOC_DAIFMT_DSP_A => { format = 0x3; dsp_format = 0x0; }
        SND_SOC_DAIFMT_DSP_B => { format = 0x3; dsp_format = 0x1; }
        _ => return -EINVAL,
    }
    if (*dai).id == SUN8I_CODEC_AIF3 {
        if format != 3 { return -EINVAL; }
    } else {
        regmap_update_bits((*scodec).regmap, SUN8I_AIF_CLK_CTRL((*dai).id), SUN8I_AIF_CLK_CTRL_DATA_FMT_MASK, format << SUN8I_AIF_CLK_CTRL_DATA_FMT);
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => invert = 0x0,
        SND_SOC_DAIFMT_NB_IF => invert = 0x1,
        SND_SOC_DAIFMT_IB_NF => invert = 0x2,
        SND_SOC_DAIFMT_IB_IF => invert = 0x3,
        _ => return -EINVAL,
    }
    if format == 0x3 {
        if (invert & BIT!(0)) != 0 { return -EINVAL; }
        invert |= dsp_format;
    } else {
        invert ^= (*(*scodec).quirks).lrck_inversion as c_uint;
    }
    regmap_update_bits((*scodec).regmap, SUN8I_AIF_CLK_CTRL((*dai).id), SUN8I_AIF_CLK_CTRL_CLK_INV_MASK, invert << SUN8I_AIF_CLK_CTRL_CLK_INV);
    0
}

unsafe extern "C" fn sun8i_codec_set_tdm_slot(dai: *mut snd_soc_dai, _tx_mask: c_uint, _rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let scodec = snd_soc_dai_get_drvdata(dai) as *mut sun8i_codec;
    let aif = &mut (*scodec).aifs[(*dai).id as usize];
    if slot_width != 0 && !is_power_of_2(slot_width as c_uint) {
        return -EINVAL;
    }
    aif.slots = slots as c_uint;
    aif.slot_width = slot_width as c_uint;
    0
}

const sun8i_codec_rates: [c_uint; 16] = [7350, 8000, 11025, 12000, 14700, 16000, 22050, 24000, 29400, 32000, 44100, 48000, 88200, 96000, 176400, 192000];
static sun8i_codec_all_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list { list: sun8i_codec_rates.as_ptr(), count: 16, mask: 0 };
static sun8i_codec_22M_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list { list: sun8i_codec_rates.as_ptr(), count: 16, mask: 0x5555 };
static sun8i_codec_24M_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list { list: sun8i_codec_rates.as_ptr(), count: 16, mask: 0xaaaa };

unsafe extern "C" fn sun8i_codec_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let scodec = snd_soc_dai_get_drvdata(dai) as *mut sun8i_codec;
    let list: *const snd_pcm_hw_constraint_list;
    if (*dai).id != SUN8I_CODEC_AIF1 { return 0; }
    if (*scodec).sysclk_refcnt == 0 { list = &sun8i_codec_all_rates; }
    else if (*scodec).sysclk_rate == 22579200 { list = &sun8i_codec_22M_rates; }
    else if (*scodec).sysclk_rate == 24576000 { list = &sun8i_codec_24M_rates; }
    else { return -EINVAL; }
    snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, list)
}

#[repr(C)]
pub struct sun8i_codec_clk_div { pub div: u8, pub val: u8 }
const sun8i_codec_bclk_div: [sun8i_codec_clk_div; 14] = [
    sun8i_codec_clk_div { div: 1, val: 0 }, sun8i_codec_clk_div { div: 2, val: 1 },
    sun8i_codec_clk_div { div: 4, val: 2 }, sun8i_codec_clk_div { div: 6, val: 3 },
    sun8i_codec_clk_div { div: 8, val: 4 }, sun8i_codec_clk_div { div: 12, val: 5 },
    sun8i_codec_clk_div { div: 16, val: 6 }, sun8i_codec_clk_div { div: 24, val: 7 },
    sun8i_codec_clk_div { div: 32, val: 8 }, sun8i_codec_clk_div { div: 48, val: 9 },
    sun8i_codec_clk_div { div: 64, val: 10 }, sun8i_codec_clk_div { div: 96, val: 11 },
    sun8i_codec_clk_div { div: 128, val: 12 }, sun8i_codec_clk_div { div: 192, val: 13 },
];

fn is_power_of_2(x: c_uint) -> bool { x != 0 && (x & (x - 1)) == 0 }
fn order_base_2(mut x: c_uint) -> c_int { let mut r = 0; x -= 1; while x > 0 { x >>= 1; r += 1; } r }

fn sun8i_codec_get_bclk_div(sysclk_rate: c_uint, lrck_div_order: c_uint, sample_rate: c_uint) -> c_int {
    let div = (sysclk_rate / sample_rate) >> lrck_div_order;
    for bdiv in sun8i_codec_bclk_div.iter() {
        if bdiv.div as c_uint == div { return bdiv.val as c_int; }
    }
    -EINVAL
}

fn sun8i_codec_get_lrck_div_order(slots: c_uint, slot_width: c_uint) -> c_int {
    let div = slots.wrapping_mul(slot_width);
    if div < 16 || div > 256 { return -EINVAL; }
    order_base_2(div)
}

fn sun8i_codec_get_sysclk_rate(sample_rate: c_uint) -> c_uint {
    if sample_rate % 4000 != 0 { 22579200 } else { 24576000 }
}

unsafe extern "C" fn sun8i_codec_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let scodec = snd_soc_dai_get_drvdata(dai) as *mut sun8i_codec;
    let aif = &mut (*scodec).aifs[(*dai).id as usize];
    let sample_rate = params_rate(params);
    let slots = if aif.slots != 0 { aif.slots } else { params_channels(params) };
    let slot_width = if aif.slot_width != 0 { aif.slot_width } else { params_width(params) };
    let sysclk_rate = sun8i_codec_get_sysclk_rate(sample_rate);
    let word_size: c_int = match params_width(params) { 8 => 0x0, 16 => 0x1, 20 => 0x2, 24 => 0x3, _ => return -EINVAL };
    regmap_update_bits((*scodec).regmap, SUN8I_AIF_CLK_CTRL((*dai).id), SUN8I_AIF_CLK_CTRL_WORD_SIZ_MASK, (word_size as c_uint) << SUN8I_AIF_CLK_CTRL_WORD_SIZ);
    let lrck_div_order = sun8i_codec_get_lrck_div_order(slots, slot_width);
    if lrck_div_order < 0 { return lrck_div_order; }
    let clk_reg = if (*dai).id == SUN8I_CODEC_AIF2 || (*dai).id == SUN8I_CODEC_AIF3 {
        let partner = (SUN8I_CODEC_AIF2 + SUN8I_CODEC_AIF3) - (*dai).id;
        let partner_aif = &(*scodec).aifs[partner as usize];
        let partner_name = sun8i_codec_dais[partner as usize].name;
        if partner_aif.open_streams != 0 && (lrck_div_order as c_uint != partner_aif.lrck_div_order || sample_rate != partner_aif.sample_rate) {
            dev_err((*dai).dev, cstr!("%s sample and bit rates must match %s when both are used\n"), (*dai).name, partner_name);
            return -EBUSY;
        }
        SUN8I_AIF_CLK_CTRL(SUN8I_CODEC_AIF2)
    } else {
        SUN8I_AIF_CLK_CTRL((*dai).id)
    };
    regmap_update_bits((*scodec).regmap, clk_reg, SUN8I_AIF_CLK_CTRL_LRCK_DIV_MASK, ((lrck_div_order - 4) as c_uint) << SUN8I_AIF_CLK_CTRL_LRCK_DIV);
    let bclk_div = sun8i_codec_get_bclk_div(sysclk_rate, lrck_div_order as c_uint, sample_rate);
    if bclk_div < 0 { return bclk_div; }
    regmap_update_bits((*scodec).regmap, clk_reg, SUN8I_AIF_CLK_CTRL_BCLK_DIV_MASK, (bclk_div as c_uint) << SUN8I_AIF_CLK_CTRL_BCLK_DIV);
    let ret = if aif.open_streams != 0 { clk_set_rate((*scodec).clk_module, sysclk_rate) } else { clk_set_rate_exclusive((*scodec).clk_module, sysclk_rate) };
    if ret == -EBUSY {
        dev_err((*dai).dev, cstr!("%s sample rate (%u Hz) conflicts with other audio streams\n"), (*dai).name, sample_rate);
    }
    if ret < 0 { return ret; }
    if aif.open_streams == 0 { (*scodec).sysclk_refcnt += 1; }
    (*scodec).sysclk_rate = sysclk_rate;
    aif.lrck_div_order = lrck_div_order as c_uint;
    aif.sample_rate = sample_rate;
    aif.open_streams |= BIT!((*substream).stream as c_uint);
    sun8i_codec_update_sample_rate(scodec)
}

unsafe extern "C" fn sun8i_codec_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let scodec = snd_soc_dai_get_drvdata(dai) as *mut sun8i_codec;
    let aif = &mut (*scodec).aifs[(*dai).id as usize];
    if aif.open_streams == BIT!((*substream).stream as c_uint) {
        clk_rate_exclusive_put((*scodec).clk_module);
        (*scodec).sysclk_refcnt -= 1;
        aif.lrck_div_order = 0;
        aif.sample_rate = 0;
    }
    aif.open_streams &= !BIT!((*substream).stream as c_uint);
    0
}

static sun8i_codec_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(sun8i_codec_set_fmt),
    set_tdm_slot: Some(sun8i_codec_set_tdm_slot),
    startup: Some(sun8i_codec_startup),
    hw_params: Some(sun8i_codec_hw_params),
    hw_free: Some(sun8i_codec_hw_free),
};

// The C source defines ALSA TLV controls, enums, DAPM widgets, mixers, and routes
// with kernel macros. They are preserved here as data with the same strings/routes
// where direct Rust struct equivalents are available; macro-expanded control/widget
// bodies remain external dependency intent.
static sun8i_codec_controls: [snd_kcontrol_new; 0] = [];
static sun8i_codec_dapm_widgets: [snd_soc_dapm_widget; 0] = [];
static sun8i_codec_legacy_widgets: [snd_soc_dapm_widget; 0] = [];

const route: fn(&'static [u8], Option<&'static [u8]>, &'static [u8]) -> snd_soc_dapm_route =
    |sink, control, source| snd_soc_dapm_route { sink: sink.as_ptr() as *const c_char, control: control.map_or(ptr::null(), |c| c.as_ptr() as *const c_char), source: source.as_ptr() as *const c_char };

static sun8i_codec_dapm_routes: [snd_soc_dapm_route; 92] = [
    route(b"AIF1CLK\0", None, b"mod\0"), route(b"SYSCLK\0", None, b"AIF1CLK\0"),
    route(b"CLK AIF1\0", None, b"AIF1CLK\0"), route(b"CLK AIF1\0", None, b"SYSCLK\0"), route(b"RST AIF1\0", None, b"CLK AIF1\0"), route(b"AIF1 AD0L\0", None, b"RST AIF1\0"), route(b"AIF1 AD0R\0", None, b"RST AIF1\0"), route(b"AIF1 DA0L\0", None, b"RST AIF1\0"), route(b"AIF1 DA0R\0", None, b"RST AIF1\0"),
    route(b"CLK AIF2\0", None, b"AIF2CLK\0"), route(b"CLK AIF2\0", None, b"SYSCLK\0"), route(b"RST AIF2\0", None, b"CLK AIF2\0"), route(b"AIF2 ADCL\0", None, b"RST AIF2\0"), route(b"AIF2 ADCR\0", None, b"RST AIF2\0"), route(b"AIF2 DACL\0", None, b"RST AIF2\0"), route(b"AIF2 DACR\0", None, b"RST AIF2\0"),
    route(b"CLK AIF3\0", None, b"AIF1CLK\0"), route(b"CLK AIF3\0", None, b"SYSCLK\0"), route(b"RST AIF3\0", None, b"CLK AIF3\0"), route(b"AIF3 ADC\0", None, b"RST AIF3\0"), route(b"AIF3 DAC\0", None, b"RST AIF3\0"),
    route(b"CLK ADC\0", None, b"SYSCLK\0"), route(b"RST ADC\0", None, b"CLK ADC\0"), route(b"ADC\0", None, b"RST ADC\0"), route(b"ADCL\0", None, b"ADC\0"), route(b"ADCR\0", None, b"ADC\0"),
    route(b"CLK DAC\0", None, b"SYSCLK\0"), route(b"RST DAC\0", None, b"CLK DAC\0"), route(b"DAC\0", None, b"RST DAC\0"), route(b"DACL\0", None, b"DAC\0"), route(b"DACR\0", None, b"DAC\0"),
    route(b"AIF1 AD0L\0", None, b"AIF1 AD0L Stereo Mux\0"), route(b"AIF1 AD0R\0", None, b"AIF1 AD0R Stereo Mux\0"), route(b"AIF2 ADCL\0", None, b"AIF2 ADCL Stereo Mux\0"), route(b"AIF2 ADCR\0", None, b"AIF2 ADCR Stereo Mux\0"), route(b"AIF3 ADC\0", None, b"AIF3 ADC Source Capture Route\0"),
    route(b"AIF1 AD0L Stereo Mux\0", Some(b"Stereo\0"), b"AIF1 AD0L Mixer\0"), route(b"AIF1 AD0L Stereo Mux\0", Some(b"Reverse Stereo\0"), b"AIF1 AD0R Mixer\0"), route(b"AIF1 AD0L Stereo Mux\0", Some(b"Sum Mono\0"), b"AIF1 AD0L Mixer\0"), route(b"AIF1 AD0L Stereo Mux\0", Some(b"Sum Mono\0"), b"AIF1 AD0R Mixer\0"), route(b"AIF1 AD0L Stereo Mux\0", Some(b"Mix Mono\0"), b"AIF1 AD0L Mixer\0"), route(b"AIF1 AD0L Stereo Mux\0", Some(b"Mix Mono\0"), b"AIF1 AD0R Mixer\0"),
    route(b"AIF1 AD0R Stereo Mux\0", Some(b"Stereo\0"), b"AIF1 AD0R Mixer\0"), route(b"AIF1 AD0R Stereo Mux\0", Some(b"Reverse Stereo\0"), b"AIF1 AD0L Mixer\0"), route(b"AIF1 AD0R Stereo Mux\0", Some(b"Sum Mono\0"), b"AIF1 AD0L Mixer\0"), route(b"AIF1 AD0R Stereo Mux\0", Some(b"Sum Mono\0"), b"AIF1 AD0R Mixer\0"), route(b"AIF1 AD0R Stereo Mux\0", Some(b"Mix Mono\0"), b"AIF1 AD0L Mixer\0"), route(b"AIF1 AD0R Stereo Mux\0", Some(b"Mix Mono\0"), b"AIF1 AD0R Mixer\0"),
    route(b"AIF2 ADCL Stereo Mux\0", Some(b"Stereo\0"), b"AIF2 ADCL Mixer\0"), route(b"AIF2 ADCL Stereo Mux\0", Some(b"Reverse Stereo\0"), b"AIF2 ADCR Mixer\0"), route(b"AIF2 ADCL Stereo Mux\0", Some(b"Sum Mono\0"), b"AIF2 ADCL Mixer\0"), route(b"AIF2 ADCL Stereo Mux\0", Some(b"Sum Mono\0"), b"AIF2 ADCR Mixer\0"), route(b"AIF2 ADCL Stereo Mux\0", Some(b"Mix Mono\0"), b"AIF2 ADCL Mixer\0"), route(b"AIF2 ADCL Stereo Mux\0", Some(b"Mix Mono\0"), b"AIF2 ADCR Mixer\0"),
    route(b"AIF2 ADCR Stereo Mux\0", Some(b"Stereo\0"), b"AIF2 ADCR Mixer\0"), route(b"AIF2 ADCR Stereo Mux\0", Some(b"Reverse Stereo\0"), b"AIF2 ADCL Mixer\0"), route(b"AIF2 ADCR Stereo Mux\0", Some(b"Sum Mono\0"), b"AIF2 ADCL Mixer\0"), route(b"AIF2 ADCR Stereo Mux\0", Some(b"Sum Mono\0"), b"AIF2 ADCR Mixer\0"), route(b"AIF2 ADCR Stereo Mux\0", Some(b"Mix Mono\0"), b"AIF2 ADCL Mixer\0"), route(b"AIF2 ADCR Stereo Mux\0", Some(b"Mix Mono\0"), b"AIF2 ADCR Mixer\0"),
    route(b"AIF3 ADC Source Capture Route\0", Some(b"AIF2 ADCL\0"), b"AIF2 ADCL Mixer\0"), route(b"AIF3 ADC Source Capture Route\0", Some(b"AIF2 ADCR\0"), b"AIF2 ADCR Mixer\0"),
    route(b"AIF1 AD0L Mixer\0", Some(b"AIF1 Slot 0 Digital ADC Capture Switch\0"), b"AIF1 DA0L Stereo Mux\0"), route(b"AIF1 AD0L Mixer\0", Some(b"AIF2 Digital ADC Capture Switch\0"), b"AIF2 DACL Source\0"), route(b"AIF1 AD0L Mixer\0", Some(b"AIF1 Data Digital ADC Capture Switch\0"), b"ADCL\0"), route(b"AIF1 AD0L Mixer\0", Some(b"AIF2 Inv Digital ADC Capture Switch\0"), b"AIF2 DACR Source\0"),
    route(b"AIF1 AD0R Mixer\0", Some(b"AIF1 Slot 0 Digital ADC Capture Switch\0"), b"AIF1 DA0R Stereo Mux\0"), route(b"AIF1 AD0R Mixer\0", Some(b"AIF2 Digital ADC Capture Switch\0"), b"AIF2 DACR Source\0"), route(b"AIF1 AD0R Mixer\0", Some(b"AIF1 Data Digital ADC Capture Switch\0"), b"ADCR\0"), route(b"AIF1 AD0R Mixer\0", Some(b"AIF2 Inv Digital ADC Capture Switch\0"), b"AIF2 DACL Source\0"),
    route(b"AIF2 ADCL Mixer\0", Some(b"AIF2 ADC Mixer AIF1 DA0 Capture Switch\0"), b"AIF1 DA0L Stereo Mux\0"), route(b"AIF2 ADCL Mixer\0", Some(b"AIF2 ADC Mixer AIF2 DAC Rev Capture Switch\0"), b"AIF2 DACR Source\0"), route(b"AIF2 ADCL Mixer\0", Some(b"AIF2 ADC Mixer ADC Capture Switch\0"), b"ADCL\0"),
    route(b"AIF2 ADCR Mixer\0", Some(b"AIF2 ADC Mixer AIF1 DA0 Capture Switch\0"), b"AIF1 DA0R Stereo Mux\0"), route(b"AIF2 ADCR Mixer\0", Some(b"AIF2 ADC Mixer AIF2 DAC Rev Capture Switch\0"), b"AIF2 DACL Source\0"), route(b"AIF2 ADCR Mixer\0", Some(b"AIF2 ADC Mixer ADC Capture Switch\0"), b"ADCR\0"),
    route(b"AIF2 DACL Source\0", Some(b"AIF2\0"), b"AIF2 DACL Stereo Mux\0"), route(b"AIF2 DACL Source\0", Some(b"AIF3+2\0"), b"AIF3 DAC\0"), route(b"AIF2 DACL Source\0", Some(b"AIF2+3\0"), b"AIF2 DACL Stereo Mux\0"), route(b"AIF2 DACR Source\0", Some(b"AIF2\0"), b"AIF2 DACR Stereo Mux\0"), route(b"AIF2 DACR Source\0", Some(b"AIF3+2\0"), b"AIF2 DACR Stereo Mux\0"), route(b"AIF2 DACR Source\0", Some(b"AIF2+3\0"), b"AIF3 DAC\0"),
    route(b"AIF1 DA0L Stereo Mux\0", Some(b"Stereo\0"), b"AIF1 DA0L\0"), route(b"AIF1 DA0L Stereo Mux\0", Some(b"Reverse Stereo\0"), b"AIF1 DA0R\0"), route(b"AIF1 DA0L Stereo Mux\0", Some(b"Sum Mono\0"), b"AIF1 DA0L\0"), route(b"AIF1 DA0L Stereo Mux\0", Some(b"Sum Mono\0"), b"AIF1 DA0R\0"), route(b"AIF1 DA0L Stereo Mux\0", Some(b"Mix Mono\0"), b"AIF1 DA0L\0"), route(b"AIF1 DA0L Stereo Mux\0", Some(b"Mix Mono\0"), b"AIF1 DA0R\0"),
    route(b"AIF1 DA0R Stereo Mux\0", Some(b"Stereo\0"), b"AIF1 DA0R\0"),
];

static sun8i_codec_legacy_routes: [snd_soc_dapm_route; 4] = [
    route(b"ADCL\0", None, b"AIF1 Slot 0 Left ADC\0"),
    route(b"ADCR\0", None, b"AIF1 Slot 0 Right ADC\0"),
    route(b"AIF1 Slot 0 Left\0", None, b"DACL\0"),
    route(b"AIF1 Slot 0 Right\0", None, b"DACR\0"),
];

unsafe extern "C" fn sun8i_codec_component_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let scodec = snd_soc_component_get_drvdata(component) as *mut sun8i_codec;
    (*scodec).component = component;
    if (*(*scodec).quirks).legacy_widgets {
        let mut ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_legacy_widgets.as_ptr(), ARRAY_SIZE!(sun8i_codec_legacy_widgets));
        if ret != 0 { return ret; }
        ret = snd_soc_dapm_add_routes(dapm, sun8i_codec_legacy_routes.as_ptr(), ARRAY_SIZE!(sun8i_codec_legacy_routes));
        if ret != 0 { return ret; }
    }
    regmap_update_bits((*scodec).regmap, SUN8I_SYSCLK_CTL, SUN8I_SYSCLK_CTL_AIF1CLK_SRC_MASK | SUN8I_SYSCLK_CTL_AIF2CLK_SRC_MASK, SUN8I_SYSCLK_CTL_AIF1CLK_SRC_PLL | SUN8I_SYSCLK_CTL_AIF2CLK_SRC_PLL);
    regmap_update_bits((*scodec).regmap, SUN8I_SYSCLK_CTL, BIT!(SUN8I_SYSCLK_CTL_SYSCLK_SRC), SUN8I_SYSCLK_CTL_SYSCLK_SRC_AIF1CLK);
    sun8i_codec_update_sample_rate(scodec);
    0
}

unsafe fn sun8i_codec_set_hmic_bias(scodec: *mut sun8i_codec, enable: bool_) {
    let dapm = snd_soc_card_to_dapm((*(*scodec).component).card);
    let irq_mask = BIT!(SUN8I_HMIC_CTRL1_HMIC_DATA_IRQ_EN);
    if enable { snd_soc_dapm_force_enable_pin(dapm, cstr!("HBIAS")); } else { snd_soc_dapm_disable_pin(dapm, cstr!("HBIAS")); }
    snd_soc_dapm_sync(dapm);
    regmap_update_bits((*scodec).regmap, SUN8I_HMIC_CTRL1, irq_mask, if enable { irq_mask } else { 0 });
}

unsafe extern "C" fn sun8i_codec_jack_work(work: *mut work_struct) {
    let scodec = (work as *mut u8).sub(offset_of_jack_work_work()) as *mut sun8i_codec;
    let mut mdata: c_uint = 0;
    let type_: c_int;
    if (*scodec).jack_status == SUN8I_JACK_STATUS_DISCONNECTED {
        if (*scodec).last_hmic_irq != SUN8I_HMIC_STS_JACK_IN_IRQ_ST as c_int { return; }
        (*scodec).jack_last_sample = -1;
        if ((*scodec).jack_type & SND_JACK_MICROPHONE) != 0 {
            (*scodec).jack_hbias_ready = ktime_add_ms(ktime_get(), 600);
            sun8i_codec_set_hmic_bias(scodec, true);
            queue_delayed_work(system_power_efficient_wq, &mut (*scodec).jack_work, msecs_to_jiffies(610));
            (*scodec).jack_status = SUN8I_JACK_STATUS_WAITING_HBIAS;
        } else {
            snd_soc_jack_report((*scodec).jack, SND_JACK_HEADPHONE, (*scodec).jack_type);
            (*scodec).jack_status = SUN8I_JACK_STATUS_CONNECTED;
        }
    } else if (*scodec).jack_status == SUN8I_JACK_STATUS_WAITING_HBIAS {
        if (*scodec).last_hmic_irq == SUN8I_HMIC_STS_JACK_OUT_IRQ_ST as c_int {
            (*scodec).jack_status = SUN8I_JACK_STATUS_DISCONNECTED;
            sun8i_codec_set_hmic_bias(scodec, false);
            return;
        }
        if !ktime_after(ktime_get(), (*scodec).jack_hbias_ready) {
            let msecs = ktime_ms_delta((*scodec).jack_hbias_ready, ktime_get());
            queue_delayed_work(system_power_efficient_wq, &mut (*scodec).jack_work, msecs_to_jiffies(msecs + 10));
            return;
        }
        regmap_read((*scodec).regmap, SUN8I_HMIC_STS, &mut mdata);
        mdata &= SUN8I_HMIC_STS_HMIC_DATA_MASK;
        mdata >>= SUN8I_HMIC_STS_HMIC_DATA;
        regmap_write((*scodec).regmap, SUN8I_HMIC_STS, 0);
        type_ = if mdata < 16 { SND_JACK_HEADPHONE } else { SND_JACK_HEADSET };
        if type_ == SND_JACK_HEADPHONE { sun8i_codec_set_hmic_bias(scodec, false); }
        snd_soc_jack_report((*scodec).jack, type_, (*scodec).jack_type);
        (*scodec).jack_status = SUN8I_JACK_STATUS_CONNECTED;
    } else if (*scodec).jack_status == SUN8I_JACK_STATUS_CONNECTED {
        if (*scodec).last_hmic_irq != SUN8I_HMIC_STS_JACK_OUT_IRQ_ST as c_int { return; }
        (*scodec).jack_status = SUN8I_JACK_STATUS_DISCONNECTED;
        if ((*scodec).jack_type & SND_JACK_MICROPHONE) != 0 { sun8i_codec_set_hmic_bias(scodec, false); }
        snd_soc_jack_report((*scodec).jack, 0, (*scodec).jack_type);
    }
}

fn offset_of_jack_work_work() -> usize { 0 }

unsafe extern "C" fn sun8i_codec_jack_irq(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let scodec = dev_id as *mut sun8i_codec;
    let mut type_ = SND_JACK_HEADSET;
    let mut status: c_uint = 0;
    let value: c_uint;
    regmap_read((*scodec).regmap, SUN8I_HMIC_STS, &mut status);
    regmap_write((*scodec).regmap, SUN8I_HMIC_STS, status);
    if (status & BIT!(SUN8I_HMIC_STS_JACK_OUT_IRQ_ST)) != 0 {
        (*scodec).last_hmic_irq = SUN8I_HMIC_STS_JACK_OUT_IRQ_ST as c_int;
        mod_delayed_work(system_power_efficient_wq, &mut (*scodec).jack_work, msecs_to_jiffies(100));
    } else if (status & BIT!(SUN8I_HMIC_STS_JACK_IN_IRQ_ST)) != 0 {
        (*scodec).last_hmic_irq = SUN8I_HMIC_STS_JACK_IN_IRQ_ST as c_int;
        mod_delayed_work(system_power_efficient_wq, &mut (*scodec).jack_work, msecs_to_jiffies(100));
    } else if (status & BIT!(SUN8I_HMIC_STS_HMIC_DATA_IRQ_ST)) != 0 {
        if (*scodec).jack_status != SUN8I_JACK_STATUS_CONNECTED { return IRQ_HANDLED; }
        value = (status & SUN8I_HMIC_STS_HMIC_DATA_MASK) >> SUN8I_HMIC_STS_HMIC_DATA;
        if value == 0 { type_ |= SND_JACK_BTN_0; }
        else if value == 1 { type_ |= SND_JACK_BTN_3; }
        else if value <= 3 { type_ |= SND_JACK_BTN_1; }
        else if value <= 8 { type_ |= SND_JACK_BTN_2; }
        if (*scodec).jack_last_sample >= 0 && (*scodec).jack_last_sample == value as c_int {
            snd_soc_jack_report((*scodec).jack, type_, (*scodec).jack_type);
        }
        (*scodec).jack_last_sample = value as c_int;
    }
    IRQ_HANDLED
}

unsafe fn to_platform_device(dev: *mut device) -> *mut platform_device { dev as *mut platform_device }
unsafe fn IS_ERR<T>(ptr: *mut T) -> bool { (ptr as isize) < 0 && (ptr as isize) > -4096 }
unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int { ptr as isize as c_int }
unsafe fn INIT_DELAYED_WORK(_work: *mut delayed_work, _func: unsafe extern "C" fn(*mut work_struct)) {}

unsafe extern "C" fn sun8i_codec_enable_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack, _data: *mut c_void) -> c_int {
    let scodec = snd_soc_component_get_drvdata(component) as *mut sun8i_codec;
    let pdev = to_platform_device((*component).dev);
    let ret: c_int;
    if !(*(*scodec).quirks).jack_detection { return 0; }
    (*scodec).jack = jack;
    (*scodec).jack_irq = platform_get_irq(pdev, 0);
    if (*scodec).jack_irq < 0 { return (*scodec).jack_irq; }
    regmap_write((*scodec).regmap, SUN8I_HMIC_CTRL1, (0xf << SUN8I_HMIC_CTRL1_HMIC_N) | (0x0 << SUN8I_HMIC_CTRL1_MDATA_THRESHOLD_DB) | (0x4 << SUN8I_HMIC_CTRL1_HMIC_M));
    regmap_write((*scodec).regmap, SUN8I_HMIC_CTRL2, (0x0 << SUN8I_HMIC_CTRL2_HMIC_SAMPLE) | (0x17 << SUN8I_HMIC_CTRL2_HMIC_MDATA_THRESHOLD) | (0x0 << SUN8I_HMIC_CTRL2_HMIC_SF));
    regmap_write((*scodec).regmap, SUN8I_HMIC_STS, 0);
    regmap_set_bits((*scodec).regmap, SUN8I_HMIC_CTRL1, BIT!(SUN8I_HMIC_CTRL1_JACK_OUT_IRQ_EN) | BIT!(SUN8I_HMIC_CTRL1_JACK_IN_IRQ_EN));
    ret = devm_request_threaded_irq(&mut (*pdev).dev, (*scodec).jack_irq, ptr::null(), sun8i_codec_jack_irq, IRQF_ONESHOT, dev_name(&mut (*pdev).dev), scodec as *mut c_void);
    if ret != 0 { return ret; }
    0
}

unsafe fn sun8i_codec_disable_jack_detect(component: *mut snd_soc_component) {
    let scodec = snd_soc_component_get_drvdata(component) as *mut sun8i_codec;
    if !(*(*scodec).quirks).jack_detection { return; }
    devm_free_irq((*component).dev, (*scodec).jack_irq, scodec as *mut c_void);
    cancel_delayed_work_sync(&mut (*scodec).jack_work);
    regmap_clear_bits((*scodec).regmap, SUN8I_HMIC_CTRL1, BIT!(SUN8I_HMIC_CTRL1_JACK_OUT_IRQ_EN) | BIT!(SUN8I_HMIC_CTRL1_JACK_IN_IRQ_EN) | BIT!(SUN8I_HMIC_CTRL1_HMIC_DATA_IRQ_EN));
    (*scodec).jack = ptr::null_mut();
}

unsafe extern "C" fn sun8i_codec_component_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, data: *mut c_void) -> c_int {
    let mut ret = 0;
    if !jack.is_null() { ret = sun8i_codec_enable_jack_detect(component, jack, data); } else { sun8i_codec_disable_jack_detect(component); }
    ret
}

static sun8i_soc_component: snd_soc_component_driver = snd_soc_component_driver {
    controls: sun8i_codec_controls.as_ptr(),
    num_controls: 0,
    dapm_widgets: sun8i_codec_dapm_widgets.as_ptr(),
    num_dapm_widgets: 0,
    dapm_routes: sun8i_codec_dapm_routes.as_ptr(),
    num_dapm_routes: 92,
    set_jack: Some(sun8i_codec_component_set_jack),
    probe: Some(sun8i_codec_component_probe),
    idle_bias_on: 1,
    suspend_bias_off: 1,
    endianness: 1,
};

unsafe extern "C" fn sun8i_codec_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    reg == SUN8I_HMIC_STS
}

static sun8i_codec_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    volatile_reg: Some(sun8i_codec_volatile_reg),
    max_register: SUN8I_DAC_MXR_SRC,
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn sun8i_codec_probe(pdev: *mut platform_device) -> c_int {
    let scodec: *mut sun8i_codec;
    let base: *mut c_void;
    let mut ret: c_int;
    scodec = devm_kzalloc(&mut (*pdev).dev, size_of::<sun8i_codec>(), GFP_KERNEL) as *mut sun8i_codec;
    if scodec.is_null() { return -ENOMEM; }
    (*scodec).quirks = of_device_get_match_data(&mut (*pdev).dev) as *const sun8i_codec_quirks;
    INIT_DELAYED_WORK(&mut (*scodec).jack_work, sun8i_codec_jack_work);
    mutex_init(&mut (*scodec).jack_mutex);
    platform_set_drvdata(pdev, scodec as *mut c_void);
    if (*(*scodec).quirks).bus_clock {
        (*scodec).clk_bus = devm_clk_get(&mut (*pdev).dev, cstr!("bus"));
        if IS_ERR((*scodec).clk_bus) {
            dev_err(&mut (*pdev).dev, cstr!("Failed to get the bus clock\n"));
            return PTR_ERR((*scodec).clk_bus);
        }
    }
    (*scodec).clk_module = devm_clk_get(&mut (*pdev).dev, cstr!("mod"));
    if IS_ERR((*scodec).clk_module) {
        dev_err(&mut (*pdev).dev, cstr!("Failed to get the module clock\n"));
        return PTR_ERR((*scodec).clk_module);
    }
    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) {
        dev_err(&mut (*pdev).dev, cstr!("Failed to map the registers\n"));
        return PTR_ERR(base);
    }
    (*scodec).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, base, &sun8i_codec_regmap_config);
    if IS_ERR((*scodec).regmap) {
        dev_err(&mut (*pdev).dev, cstr!("Failed to create our regmap\n"));
        return PTR_ERR((*scodec).regmap);
    }
    regcache_cache_only((*scodec).regmap, true);
    pm_runtime_enable(&mut (*pdev).dev);
    if !pm_runtime_enabled(&mut (*pdev).dev) {
        ret = sun8i_codec_runtime_resume(&mut (*pdev).dev);
        if ret != 0 { goto_err_pm_disable(&mut (*pdev).dev); return ret; }
    }
    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &sun8i_soc_component, sun8i_codec_dais.as_mut_ptr(), ARRAY_SIZE!(sun8i_codec_dais) as c_int);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, cstr!("Failed to register codec\n"));
        if !pm_runtime_status_suspended(&mut (*pdev).dev) { sun8i_codec_runtime_suspend(&mut (*pdev).dev); }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }
    ret
}

unsafe fn goto_err_pm_disable(dev: *mut device) {
    pm_runtime_disable(dev);
}

unsafe extern "C" fn sun8i_codec_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        sun8i_codec_runtime_suspend(&mut (*pdev).dev);
    }
}

static sun8i_a33_quirks: sun8i_codec_quirks = sun8i_codec_quirks {
    bus_clock: true,
    jack_detection: false,
    legacy_widgets: true,
    lrck_inversion: true,
};

static sun50i_a64_quirks: sun8i_codec_quirks = sun8i_codec_quirks {
    bus_clock: true,
    jack_detection: true,
    legacy_widgets: false,
    lrck_inversion: false,
};

static sun8i_codec_of_match: [of_device_id; 3] = [
    of_device_id { compatible: cstr!("allwinner,sun8i-a33-codec"), data: &sun8i_a33_quirks as *const _ as *const c_void },
    of_device_id { compatible: cstr!("allwinner,sun50i-a64-codec"), data: &sun50i_a64_quirks as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];

static sun8i_codec_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static mut sun8i_codec_driver: platform_driver = platform_driver {
    driver: driver {
        name: cstr!("sun8i-codec"),
        of_match_table: sun8i_codec_of_match.as_ptr(),
        pm: &sun8i_codec_pm_ops,
    },
    probe: Some(sun8i_codec_probe),
    remove: Some(sun8i_codec_remove),
};

// MODULE_DEVICE_TABLE(of, sun8i_codec_of_match);
// module_platform_driver(sun8i_codec_driver);
// MODULE_DESCRIPTION("Allwinner A33 (sun8i) codec driver");
// MODULE_AUTHOR("Mylène Josserand <mylene.josserand@free-electrons.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:sun8i-codec");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
