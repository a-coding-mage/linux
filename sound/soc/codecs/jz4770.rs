// SPDX-License-Identifier: GPL-2.0
//
// Ingenic JZ4770 CODEC driver
//
// Copyright (C) 2012, Maarten ter Huurne <maarten@treewalker.org>
// Copyright (C) 2019, Paul Cercueil <paul@crapouillou.net>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u32 = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
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
pub struct snd_soc_dapm_widget_desc {
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
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub suspend_bias_off: c_uint,
    pub use_pmdown_time: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
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
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    pub reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
    pub reg_defaults_raw: *const u8,
    pub num_reg_defaults_raw: c_uint,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_driver,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_OFF = 0,
    SND_SOC_BIAS_STANDBY = 1,
    SND_SOC_BIAS_PREPARE = 2,
    SND_SOC_BIAS_ON = 3,
}

#[repr(C)]
pub struct jz_codec {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub base: *mut c_void,
}

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

const ICDC_RGADW_OFFSET: usize = 0x00;
const ICDC_RGDATA_OFFSET: usize = 0x04;

/* ICDC internal register access control register(RGADW) */
const ICDC_RGADW_RGWR: c_uint = BIT(16);

const ICDC_RGADW_RGADDR_OFFSET: c_uint = 8;
const ICDC_RGADW_RGADDR_MASK: c_uint = GENMASK(14, ICDC_RGADW_RGADDR_OFFSET);

const ICDC_RGADW_RGDIN_OFFSET: c_uint = 0;
const ICDC_RGADW_RGDIN_MASK: c_uint = GENMASK(7, ICDC_RGADW_RGDIN_OFFSET);

/* ICDC internal register data output register (RGDATA)*/
const ICDC_RGDATA_IRQ: c_uint = BIT(8);

const ICDC_RGDATA_RGDOUT_OFFSET: c_uint = 0;
const ICDC_RGDATA_RGDOUT_MASK: c_uint = GENMASK(7, ICDC_RGDATA_RGDOUT_OFFSET);

/* Internal register space, accessed through regmap */
const JZ4770_CODEC_REG_SR: c_uint = 0;
const JZ4770_CODEC_REG_AICR_DAC: c_uint = 1;
const JZ4770_CODEC_REG_AICR_ADC: c_uint = 2;
const JZ4770_CODEC_REG_CR_LO: c_uint = 3;
const JZ4770_CODEC_REG_CR_HP: c_uint = 4;
const JZ4770_CODEC_REG_MISSING_REG1: c_uint = 5;
const JZ4770_CODEC_REG_CR_DAC: c_uint = 6;
const JZ4770_CODEC_REG_CR_MIC: c_uint = 7;
const JZ4770_CODEC_REG_CR_LI: c_uint = 8;
const JZ4770_CODEC_REG_CR_ADC: c_uint = 9;
const JZ4770_CODEC_REG_CR_MIX: c_uint = 10;
const JZ4770_CODEC_REG_CR_VIC: c_uint = 11;
const JZ4770_CODEC_REG_CCR: c_uint = 12;
const JZ4770_CODEC_REG_FCR_DAC: c_uint = 13;
const JZ4770_CODEC_REG_FCR_ADC: c_uint = 14;
const JZ4770_CODEC_REG_ICR: c_uint = 15;
const JZ4770_CODEC_REG_IMR: c_uint = 16;
const JZ4770_CODEC_REG_IFR: c_uint = 17;
const JZ4770_CODEC_REG_GCR_HPL: c_uint = 18;
const JZ4770_CODEC_REG_GCR_HPR: c_uint = 19;
const JZ4770_CODEC_REG_GCR_LIBYL: c_uint = 20;
const JZ4770_CODEC_REG_GCR_LIBYR: c_uint = 21;
const JZ4770_CODEC_REG_GCR_DACL: c_uint = 22;
const JZ4770_CODEC_REG_GCR_DACR: c_uint = 23;
const JZ4770_CODEC_REG_GCR_MIC1: c_uint = 24;
const JZ4770_CODEC_REG_GCR_MIC2: c_uint = 25;
const JZ4770_CODEC_REG_GCR_ADCL: c_uint = 26;
const JZ4770_CODEC_REG_GCR_ADCR: c_uint = 27;
const JZ4770_CODEC_REG_MISSING_REG2: c_uint = 28;
const JZ4770_CODEC_REG_GCR_MIXADC: c_uint = 29;
const JZ4770_CODEC_REG_GCR_MIXDAC: c_uint = 30;
const JZ4770_CODEC_REG_AGC1: c_uint = 31;
const JZ4770_CODEC_REG_AGC2: c_uint = 32;
const JZ4770_CODEC_REG_AGC3: c_uint = 33;
const JZ4770_CODEC_REG_AGC4: c_uint = 34;
const JZ4770_CODEC_REG_AGC5: c_uint = 35;

const REG_AICR_DAC_ADWL_OFFSET: c_uint = 6;
const REG_AICR_DAC_ADWL_MASK: c_uint = 0x3 << REG_AICR_DAC_ADWL_OFFSET;
const REG_AICR_DAC_SERIAL: c_uint = BIT(1);
const REG_AICR_DAC_I2S: c_uint = BIT(0);

const REG_AICR_ADC_ADWL_OFFSET: c_uint = 6;
const REG_AICR_ADC_ADWL_MASK: c_uint = 0x3 << REG_AICR_ADC_ADWL_OFFSET;
const REG_AICR_ADC_SERIAL: c_uint = BIT(1);
const REG_AICR_ADC_I2S: c_uint = BIT(0);

const REG_CR_LO_MUTE_OFFSET: c_uint = 7;
const REG_CR_LO_SB_OFFSET: c_uint = 4;
const REG_CR_LO_SEL_OFFSET: c_uint = 0;
const REG_CR_LO_SEL_MASK: c_uint = 0x3 << REG_CR_LO_SEL_OFFSET;

const REG_CR_HP_MUTE: c_uint = BIT(7);
const REG_CR_HP_LOAD: c_uint = BIT(6);
const REG_CR_HP_SB_OFFSET: c_uint = 4;
const REG_CR_HP_SB_HPCM_OFFSET: c_uint = 3;
const REG_CR_HP_SEL_OFFSET: c_uint = 0;
const REG_CR_HP_SEL_MASK: c_uint = 0x3 << REG_CR_HP_SEL_OFFSET;

const REG_CR_DAC_MUTE: c_uint = BIT(7);
const REG_CR_DAC_MONO: c_uint = BIT(6);
const REG_CR_DAC_LEFT_ONLY: c_uint = BIT(5);
const REG_CR_DAC_SB_OFFSET: c_uint = 4;
const REG_CR_DAC_LRSWAP: c_uint = BIT(3);

const REG_CR_MIC_STEREO_OFFSET: c_uint = 7;
const REG_CR_MIC_IDIFF_OFFSET: c_uint = 6;
const REG_CR_MIC_SB_MIC2_OFFSET: c_uint = 5;
const REG_CR_MIC_SB_MIC1_OFFSET: c_uint = 4;
const REG_CR_MIC_BIAS_V0_OFFSET: c_uint = 1;
const REG_CR_MIC_BIAS_SB_OFFSET: c_uint = 0;

const REG_CR_LI_LIBY_OFFSET: c_uint = 4;
const REG_CR_LI_SB_OFFSET: c_uint = 0;

const REG_CR_ADC_DMIC_SEL: c_uint = BIT(7);
const REG_CR_ADC_MONO: c_uint = BIT(6);
const REG_CR_ADC_LEFT_ONLY: c_uint = BIT(5);
const REG_CR_ADC_SB_OFFSET: c_uint = 4;
const REG_CR_ADC_LRSWAP: c_uint = BIT(3);
const REG_CR_ADC_IN_SEL_OFFSET: c_uint = 0;
const REG_CR_ADC_IN_SEL_MASK: c_uint = 0x3 << REG_CR_ADC_IN_SEL_OFFSET;

const REG_CR_VIC_SB_SLEEP: c_uint = BIT(1);
const REG_CR_VIC_SB: c_uint = BIT(0);

const REG_CCR_CRYSTAL_OFFSET: c_uint = 0;
const REG_CCR_CRYSTAL_MASK: c_uint = 0xf << REG_CCR_CRYSTAL_OFFSET;

const REG_FCR_DAC_FREQ_OFFSET: c_uint = 0;
const REG_FCR_DAC_FREQ_MASK: c_uint = 0xf << REG_FCR_DAC_FREQ_OFFSET;

const REG_FCR_ADC_FREQ_OFFSET: c_uint = 0;
const REG_FCR_ADC_FREQ_MASK: c_uint = 0xf << REG_FCR_ADC_FREQ_OFFSET;

const REG_ICR_INT_FORM_OFFSET: c_uint = 6;
const REG_ICR_INT_FORM_MASK: c_uint = 0x3 << REG_ICR_INT_FORM_OFFSET;

const REG_IMR_ALL_MASK: c_uint = 0x7f;
const REG_IMR_SCLR_MASK: c_uint = BIT(6);
const REG_IMR_JACK_MASK: c_uint = BIT(5);
const REG_IMR_SCMC_MASK: c_uint = BIT(4);
const REG_IMR_RUP_MASK: c_uint = BIT(3);
const REG_IMR_RDO_MASK: c_uint = BIT(2);
const REG_IMR_GUP_MASK: c_uint = BIT(1);
const REG_IMR_GDO_MASK: c_uint = BIT(0);

const REG_IFR_ALL_MASK: c_uint = 0x7f;
const REG_IFR_SCLR: c_uint = BIT(6);
const REG_IFR_JACK: c_uint = BIT(5);
const REG_IFR_SCMC: c_uint = BIT(4);
const REG_IFR_RUP: c_uint = BIT(3);
const REG_IFR_RDO: c_uint = BIT(2);
const REG_IFR_GUP: c_uint = BIT(1);
const REG_IFR_GDO: c_uint = BIT(0);

const REG_GCR_HPL_LRGO: c_uint = BIT(7);
const REG_GCR_DACL_RLGOD: c_uint = BIT(7);
const REG_GCR_GAIN_OFFSET: c_uint = 0;
const REG_GCR_GAIN_MAX: c_uint = 0x1f;
const REG_GCR_MIC_GAIN_OFFSET: c_uint = 0;
const REG_GCR_MIC_GAIN_MAX: c_uint = 5;
const REG_GCR_ADC_GAIN_OFFSET: c_uint = 0;
const REG_GCR_ADC_GAIN_MAX: c_uint = 23;
const REG_AGC1_EN: c_uint = BIT(7);

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const USEC_PER_SEC: c_uint = 1_000_000;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S18_3LE: c_int = 7;
const SNDRV_PCM_FORMAT_S20_3LE: c_int = 8;
const SNDRV_PCM_FORMAT_S24_3LE: c_int = 9;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S18_3LE: u64 = 1 << SNDRV_PCM_FORMAT_S18_3LE;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << SNDRV_PCM_FORMAT_S20_3LE;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << SNDRV_PCM_FORMAT_S24_3LE;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMU: c_int = 0x2;
const SND_SOC_DAPM_POST_PMD: c_int = 0x8;
const SND_SOC_NOPM: c_uint = 0;
const REGCACHE_FLAT: c_uint = 0;

const JZ_CODEC_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE;

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn msleep(msecs: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn readl(addr: *const c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init(dev: *mut device, bus: *const c_void, context: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! DECLARE_TLV_DB_MINMAX_MUTE {
    ($name:ident, $min:expr, $max:expr) => {
        static $name: [c_uint; 4] = [$min as c_uint, $max as c_uint, 0, 0];
    };
}

macro_rules! DECLARE_TLV_DB_SCALE {
    ($name:ident, $min:expr, $step:expr, $mute:expr) => {
        static $name: [c_uint; 4] = [$min as c_uint, $step as c_uint, $mute as c_uint, 0];
    };
}

macro_rules! DECLARE_TLV_DB_MINMAX {
    ($name:ident, $min:expr, $max:expr) => {
        static $name: [c_uint; 4] = [$min as c_uint, $max as c_uint, 0, 0];
    };
}

/* External ALSA SoC construction macros are represented as Rust macro calls. */
macro_rules! SOC_DOUBLE_R_TLV { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_SINGLE_TLV { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DAPM_DOUBLE_R_TLV { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_VALUE_ENUM_SINGLE_DECL { ($($tt:tt)*) => {}; }
macro_rules! SOC_DAPM_ENUM { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DAPM_SINGLE { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SND_SOC_DAPM_PGA_E { ($($tt:tt)*) => { snd_soc_dapm_widget_desc { _private: [] } }; }
macro_rules! SND_SOC_DAPM_PGA { ($($tt:tt)*) => { snd_soc_dapm_widget_desc { _private: [] } }; }
macro_rules! SND_SOC_DAPM_MUX { ($($tt:tt)*) => { snd_soc_dapm_widget_desc { _private: [] } }; }
macro_rules! SND_SOC_DAPM_MIXER { ($($tt:tt)*) => { snd_soc_dapm_widget_desc { _private: [] } }; }
macro_rules! SND_SOC_DAPM_ADC_E { ($($tt:tt)*) => { snd_soc_dapm_widget_desc { _private: [] } }; }
macro_rules! SND_SOC_DAPM_DAC { ($($tt:tt)*) => { snd_soc_dapm_widget_desc { _private: [] } }; }
macro_rules! SND_SOC_DAPM_SUPPLY { ($($tt:tt)*) => { snd_soc_dapm_widget_desc { _private: [] } }; }
macro_rules! SND_SOC_DAPM_INPUT { ($($tt:tt)*) => { snd_soc_dapm_widget_desc { _private: [] } }; }
macro_rules! SND_SOC_DAPM_OUTPUT { ($($tt:tt)*) => { snd_soc_dapm_widget_desc { _private: [] } }; }

unsafe extern "C" fn jz4770_codec_set_bias_level(
    codec: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let jz_codec = snd_soc_component_get_drvdata(codec) as *mut jz_codec;
    let regmap = (*jz_codec).regmap;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            /* Reset all interrupt flags. */
            regmap_write(regmap, JZ4770_CODEC_REG_IFR, REG_IFR_ALL_MASK);

            regmap_clear_bits(regmap, JZ4770_CODEC_REG_CR_VIC, REG_CR_VIC_SB);
            msleep(250);
            regmap_clear_bits(regmap, JZ4770_CODEC_REG_CR_VIC, REG_CR_VIC_SB_SLEEP);
            msleep(400);
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            regmap_set_bits(regmap, JZ4770_CODEC_REG_CR_VIC, REG_CR_VIC_SB_SLEEP);
            regmap_set_bits(regmap, JZ4770_CODEC_REG_CR_VIC, REG_CR_VIC_SB);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn jz4770_codec_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let codec = (*dai).component;
    let dapm = snd_soc_component_to_dapm(codec);

    /*
     * SYSCLK output from the codec to the AIC is required to keep the
     * DMA transfer going during playback when all audible outputs have
     * been disabled.
     */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        snd_soc_dapm_force_enable_pin(dapm, cstr!("SYSCLK"));
    }

    0
}

unsafe extern "C" fn jz4770_codec_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let codec = (*dai).component;
    let dapm = snd_soc_component_to_dapm(codec);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        snd_soc_dapm_disable_pin(dapm, cstr!("SYSCLK"));
    }
}

unsafe extern "C" fn jz4770_codec_pcm_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let codec = (*dai).component;
    let dapm = snd_soc_component_to_dapm(codec);
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK {
                snd_soc_dapm_force_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_ON);
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            /* do nothing */
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn regmap_read_poll_timeout(
    map: *mut regmap,
    reg: c_uint,
    val: *mut c_uint,
    check: fn(c_uint) -> bool,
    _sleep_us: c_uint,
    _timeout_us: c_uint,
) -> c_int {
    let ret = regmap_read(map, reg, val);
    if ret != 0 {
        return ret;
    }
    if check(*val) {
        0
    } else {
        -EINVAL
    }
}

unsafe extern "C" fn readl_poll_timeout(
    addr: *mut c_void,
    val: *mut u32,
    check: fn(u32) -> bool,
    _sleep_us: c_uint,
    _timeout_us: c_uint,
) -> c_int {
    *val = readl(addr);
    if check(*val) {
        0
    } else {
        -EINVAL
    }
}

unsafe extern "C" fn jz4770_codec_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let codec = (*dai).component;
    let jz_codec = snd_soc_component_get_drvdata(codec) as *mut jz_codec;
    let gain_bit: c_uint = if mute != 0 { REG_IFR_GDO } else { REG_IFR_GUP };
    let mut val: c_uint = 0;
    let change: c_int;
    let mut err: c_int;

    change = snd_soc_component_update_bits(
        codec,
        JZ4770_CODEC_REG_CR_DAC,
        REG_CR_DAC_MUTE,
        if mute != 0 { REG_CR_DAC_MUTE } else { 0 },
    );
    if change == 1 {
        regmap_read((*jz_codec).regmap, JZ4770_CODEC_REG_CR_DAC, &mut val);

        if (val & BIT(REG_CR_DAC_SB_OFFSET)) != 0 {
            return 1;
        }

        err = regmap_read_poll_timeout(
            (*jz_codec).regmap,
            JZ4770_CODEC_REG_IFR,
            &mut val,
            |v| (v & gain_bit) != 0,
            1000,
            1 * USEC_PER_SEC,
        );
        if err != 0 {
            dev_err((*jz_codec).dev, cstr!("Timeout while setting digital mute: %d"), err);
            return err;
        }

        /* clear GUP/GDO flag */
        regmap_set_bits((*jz_codec).regmap, JZ4770_CODEC_REG_IFR, gain_bit);
    }

    0
}

/* unit: 0.01dB */
DECLARE_TLV_DB_MINMAX_MUTE!(dac_tlv, -3100, 0);
DECLARE_TLV_DB_SCALE!(adc_tlv, 0, 100, 0);
DECLARE_TLV_DB_MINMAX!(out_tlv, -2500, 600);
DECLARE_TLV_DB_SCALE!(linein_tlv, -2500, 100, 0);
DECLARE_TLV_DB_MINMAX!(mixer_tlv, -3100, 0);

/* Unconditional controls. */
static jz4770_codec_snd_controls: [snd_kcontrol_new; 4] = [
    /* record gain control */
    SOC_DOUBLE_R_TLV!("PCM Capture Volume",
        JZ4770_CODEC_REG_GCR_ADCL, JZ4770_CODEC_REG_GCR_ADCR,
        REG_GCR_ADC_GAIN_OFFSET, REG_GCR_ADC_GAIN_MAX,
        0, adc_tlv),
    SOC_DOUBLE_R_TLV!("Line In Bypass Playback Volume",
        JZ4770_CODEC_REG_GCR_LIBYL, JZ4770_CODEC_REG_GCR_LIBYR,
        REG_GCR_GAIN_OFFSET, REG_GCR_GAIN_MAX, 1, linein_tlv),
    SOC_SINGLE_TLV!("Mixer Capture Volume",
        JZ4770_CODEC_REG_GCR_MIXADC,
        REG_GCR_GAIN_OFFSET, REG_GCR_GAIN_MAX, 1, mixer_tlv),
    SOC_SINGLE_TLV!("Mixer Playback Volume",
        JZ4770_CODEC_REG_GCR_MIXDAC,
        REG_GCR_GAIN_OFFSET, REG_GCR_GAIN_MAX, 1, mixer_tlv),
];

static jz4770_codec_pcm_playback_controls: [snd_kcontrol_new; 1] = [
    SOC_DAPM_DOUBLE_R_TLV!("Volume", JZ4770_CODEC_REG_GCR_DACR,
        JZ4770_CODEC_REG_GCR_DACL, REG_GCR_GAIN_OFFSET,
        REG_GCR_GAIN_MAX, 1, dac_tlv),
];

static jz4770_codec_hp_playback_controls: [snd_kcontrol_new; 1] = [
    SOC_DAPM_DOUBLE_R_TLV!("Volume", JZ4770_CODEC_REG_GCR_HPR,
        JZ4770_CODEC_REG_GCR_HPL, REG_GCR_GAIN_OFFSET,
        REG_GCR_GAIN_MAX, 1, out_tlv),
];

unsafe extern "C" fn hpout_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let codec = snd_soc_dapm_to_component((*w).dapm);
    let jz_codec = snd_soc_component_get_drvdata(codec) as *mut jz_codec;
    let mut val: c_uint = 0;
    let mut err: c_int;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* unmute HP */
            regmap_clear_bits((*jz_codec).regmap, JZ4770_CODEC_REG_CR_HP, REG_CR_HP_MUTE);
        }
        SND_SOC_DAPM_POST_PMU => {
            /* wait for ramp-up complete (RUP) */
            err = regmap_read_poll_timeout(
                (*jz_codec).regmap,
                JZ4770_CODEC_REG_IFR,
                &mut val,
                |v| (v & REG_IFR_RUP) != 0,
                1000,
                1 * USEC_PER_SEC,
            );
            if err != 0 {
                dev_err((*jz_codec).dev, cstr!("RUP timeout: %d"), err);
                return err;
            }

            /* clear RUP flag */
            regmap_set_bits((*jz_codec).regmap, JZ4770_CODEC_REG_IFR, REG_IFR_RUP);
        }
        SND_SOC_DAPM_POST_PMD => {
            /* mute HP */
            regmap_set_bits((*jz_codec).regmap, JZ4770_CODEC_REG_CR_HP, REG_CR_HP_MUTE);

            err = regmap_read_poll_timeout(
                (*jz_codec).regmap,
                JZ4770_CODEC_REG_IFR,
                &mut val,
                |v| (v & REG_IFR_RDO) != 0,
                1000,
                1 * USEC_PER_SEC,
            );
            if err != 0 {
                dev_err((*jz_codec).dev, cstr!("RDO timeout: %d"), err);
                return err;
            }

            /* clear RDO flag */
            regmap_set_bits((*jz_codec).regmap, JZ4770_CODEC_REG_IFR, REG_IFR_RDO);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn adc_poweron_event(
    _w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    if event == SND_SOC_DAPM_POST_PMU {
        msleep(1000);
    }

    0
}

static jz4770_codec_hp_texts: [*const c_char; 4] = [
    cstr!("PCM"), cstr!("Line In"), cstr!("Mic 1"), cstr!("Mic 2"),
];
static jz4770_codec_hp_values: [c_uint; 4] = [3, 2, 0, 1];
SOC_VALUE_ENUM_SINGLE_DECL!(jz4770_codec_hp_enum,
    JZ4770_CODEC_REG_CR_HP,
    REG_CR_HP_SEL_OFFSET,
    REG_CR_HP_SEL_MASK,
    jz4770_codec_hp_texts,
    jz4770_codec_hp_values);
static jz4770_codec_hp_source: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", jz4770_codec_hp_enum);

SOC_VALUE_ENUM_SINGLE_DECL!(jz4770_codec_lo_enum,
    JZ4770_CODEC_REG_CR_LO,
    REG_CR_LO_SEL_OFFSET,
    REG_CR_LO_SEL_MASK,
    jz4770_codec_hp_texts,
    jz4770_codec_hp_values);
static jz4770_codec_lo_source: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", jz4770_codec_lo_enum);

static jz4770_codec_cap_texts: [*const c_char; 3] = [
    cstr!("Line In"), cstr!("Mic 1"), cstr!("Mic 2"),
];
static jz4770_codec_cap_values: [c_uint; 3] = [2, 0, 1];
SOC_VALUE_ENUM_SINGLE_DECL!(jz4770_codec_cap_enum,
    JZ4770_CODEC_REG_CR_ADC,
    REG_CR_ADC_IN_SEL_OFFSET,
    REG_CR_ADC_IN_SEL_MASK,
    jz4770_codec_cap_texts,
    jz4770_codec_cap_values);
static jz4770_codec_cap_source: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", jz4770_codec_cap_enum);

static jz4770_codec_mic_controls: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE!("Stereo Capture Switch", JZ4770_CODEC_REG_CR_MIC,
        REG_CR_MIC_STEREO_OFFSET, 1, 0),
];

static jz4770_codec_dapm_widgets: [snd_soc_dapm_widget_desc; 31] = [
    SND_SOC_DAPM_PGA_E!("HP Out", JZ4770_CODEC_REG_CR_HP,
        REG_CR_HP_SB_OFFSET, 1, ptr::null(), 0, hpout_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_PGA!("Line Out", JZ4770_CODEC_REG_CR_LO,
        REG_CR_LO_SB_OFFSET, 1, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Line Out Switch 2", JZ4770_CODEC_REG_CR_LO,
        REG_CR_LO_MUTE_OFFSET, 1, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Line In", JZ4770_CODEC_REG_CR_LI,
        REG_CR_LI_SB_OFFSET, 1, ptr::null(), 0),
    SND_SOC_DAPM_MUX!("Headphones Source", SND_SOC_NOPM, 0, 0, &jz4770_codec_hp_source),
    SND_SOC_DAPM_MUX!("Capture Source", SND_SOC_NOPM, 0, 0, &jz4770_codec_cap_source),
    SND_SOC_DAPM_MUX!("Line Out Source", SND_SOC_NOPM, 0, 0, &jz4770_codec_lo_source),
    SND_SOC_DAPM_PGA!("Mic 1", JZ4770_CODEC_REG_CR_MIC,
        REG_CR_MIC_SB_MIC1_OFFSET, 1, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Mic 2", JZ4770_CODEC_REG_CR_MIC,
        REG_CR_MIC_SB_MIC2_OFFSET, 1, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Mic Diff", JZ4770_CODEC_REG_CR_MIC,
        REG_CR_MIC_IDIFF_OFFSET, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Mic", SND_SOC_NOPM, 0, 0,
        jz4770_codec_mic_controls, ARRAY_SIZE(&jz4770_codec_mic_controls)),
    SND_SOC_DAPM_PGA!("Line In Bypass", JZ4770_CODEC_REG_CR_LI,
        REG_CR_LI_LIBY_OFFSET, 1, ptr::null(), 0),
    SND_SOC_DAPM_ADC_E!("ADC", "HiFi Capture", JZ4770_CODEC_REG_CR_ADC,
        REG_CR_ADC_SB_OFFSET, 1, adc_poweron_event, SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_DAC!("DAC", "HiFi Playback", JZ4770_CODEC_REG_CR_DAC,
        REG_CR_DAC_SB_OFFSET, 1),
    SND_SOC_DAPM_MIXER!("PCM Playback", SND_SOC_NOPM, 0, 0,
        jz4770_codec_pcm_playback_controls, ARRAY_SIZE(&jz4770_codec_pcm_playback_controls)),
    SND_SOC_DAPM_MIXER!("Headphones Playback", SND_SOC_NOPM, 0, 0,
        jz4770_codec_hp_playback_controls, ARRAY_SIZE(&jz4770_codec_hp_playback_controls)),
    SND_SOC_DAPM_SUPPLY!("MICBIAS", JZ4770_CODEC_REG_CR_MIC,
        REG_CR_MIC_BIAS_SB_OFFSET, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Cap-less", JZ4770_CODEC_REG_CR_HP,
        REG_CR_HP_SB_HPCM_OFFSET, 1, ptr::null(), 0),
    SND_SOC_DAPM_INPUT!("MIC1P"),
    SND_SOC_DAPM_INPUT!("MIC1N"),
    SND_SOC_DAPM_INPUT!("MIC2P"),
    SND_SOC_DAPM_INPUT!("MIC2N"),
    SND_SOC_DAPM_OUTPUT!("LOUT"),
    SND_SOC_DAPM_OUTPUT!("ROUT"),
    SND_SOC_DAPM_OUTPUT!("LHPOUT"),
    SND_SOC_DAPM_OUTPUT!("RHPOUT"),
    SND_SOC_DAPM_INPUT!("LLINEIN"),
    SND_SOC_DAPM_INPUT!("RLINEIN"),
    SND_SOC_DAPM_OUTPUT!("SYSCLK"),
];

/* Unconditional routes. */
static jz4770_codec_dapm_routes: [snd_soc_dapm_route; 39] = [
    snd_soc_dapm_route { sink: cstr!("Mic 1"), control: ptr::null(), source: cstr!("MIC1P") },
    snd_soc_dapm_route { sink: cstr!("Mic Diff"), control: ptr::null(), source: cstr!("MIC1N") },
    snd_soc_dapm_route { sink: cstr!("Mic 1"), control: ptr::null(), source: cstr!("Mic Diff") },
    snd_soc_dapm_route { sink: cstr!("Mic 2"), control: ptr::null(), source: cstr!("MIC2P") },
    snd_soc_dapm_route { sink: cstr!("Mic Diff"), control: ptr::null(), source: cstr!("MIC2N") },
    snd_soc_dapm_route { sink: cstr!("Mic 2"), control: ptr::null(), source: cstr!("Mic Diff") },
    snd_soc_dapm_route { sink: cstr!("Line In"), control: ptr::null(), source: cstr!("LLINEIN") },
    snd_soc_dapm_route { sink: cstr!("Line In"), control: ptr::null(), source: cstr!("RLINEIN") },
    snd_soc_dapm_route { sink: cstr!("Mic"), control: cstr!("Stereo Capture Switch"), source: cstr!("Mic 1") },
    snd_soc_dapm_route { sink: cstr!("Mic"), control: cstr!("Stereo Capture Switch"), source: cstr!("Mic 2") },
    snd_soc_dapm_route { sink: cstr!("Headphones Source"), control: cstr!("Mic 1"), source: cstr!("Mic") },
    snd_soc_dapm_route { sink: cstr!("Headphones Source"), control: cstr!("Mic 2"), source: cstr!("Mic") },
    snd_soc_dapm_route { sink: cstr!("Capture Source"), control: cstr!("Mic 1"), source: cstr!("Mic") },
    snd_soc_dapm_route { sink: cstr!("Capture Source"), control: cstr!("Mic 2"), source: cstr!("Mic") },
    snd_soc_dapm_route { sink: cstr!("Headphones Source"), control: cstr!("Mic 1"), source: cstr!("Mic 1") },
    snd_soc_dapm_route { sink: cstr!("Headphones Source"), control: cstr!("Mic 2"), source: cstr!("Mic 2") },
    snd_soc_dapm_route { sink: cstr!("Headphones Source"), control: cstr!("Line In"), source: cstr!("Line In Bypass") },
    snd_soc_dapm_route { sink: cstr!("Headphones Source"), control: cstr!("PCM"), source: cstr!("Headphones Playback") },
    snd_soc_dapm_route { sink: cstr!("HP Out"), control: ptr::null(), source: cstr!("Headphones Source") },
    snd_soc_dapm_route { sink: cstr!("Capture Source"), control: cstr!("Line In"), source: cstr!("Line In") },
    snd_soc_dapm_route { sink: cstr!("Capture Source"), control: cstr!("Mic 1"), source: cstr!("Mic 1") },
    snd_soc_dapm_route { sink: cstr!("Capture Source"), control: cstr!("Mic 2"), source: cstr!("Mic 2") },
    snd_soc_dapm_route { sink: cstr!("ADC"), control: ptr::null(), source: cstr!("Capture Source") },
    snd_soc_dapm_route { sink: cstr!("Line In Bypass"), control: ptr::null(), source: cstr!("Line In") },
    snd_soc_dapm_route { sink: cstr!("Line Out Source"), control: cstr!("Line In"), source: cstr!("Line In Bypass") },
    snd_soc_dapm_route { sink: cstr!("Line Out Source"), control: cstr!("PCM"), source: cstr!("PCM Playback") },
    snd_soc_dapm_route { sink: cstr!("LHPOUT"), control: ptr::null(), source: cstr!("HP Out") },
    snd_soc_dapm_route { sink: cstr!("RHPOUT"), control: ptr::null(), source: cstr!("HP Out") },
    snd_soc_dapm_route { sink: cstr!("Line Out"), control: ptr::null(), source: cstr!("Line Out Source") },
    snd_soc_dapm_route { sink: cstr!("Line Out Switch 2"), control: ptr::null(), source: cstr!("Line Out") },
    snd_soc_dapm_route { sink: cstr!("LOUT"), control: ptr::null(), source: cstr!("Line Out Switch 2") },
    snd_soc_dapm_route { sink: cstr!("ROUT"), control: ptr::null(), source: cstr!("Line Out Switch 2") },
    snd_soc_dapm_route { sink: cstr!("PCM Playback"), control: cstr!("Volume"), source: cstr!("DAC") },
    snd_soc_dapm_route { sink: cstr!("Headphones Playback"), control: cstr!("Volume"), source: cstr!("PCM Playback") },
    snd_soc_dapm_route { sink: cstr!("SYSCLK"), control: ptr::null(), source: cstr!("DAC") },
];

unsafe fn jz4770_codec_codec_init_regs(codec: *mut snd_soc_component) {
    let jz_codec = snd_soc_component_get_drvdata(codec) as *mut jz_codec;
    let regmap = (*jz_codec).regmap;

    /* Collect updates for later sending. */
    regcache_cache_only(regmap, true);

    /* default HP output to PCM */
    regmap_set_bits(regmap, JZ4770_CODEC_REG_CR_HP, REG_CR_HP_SEL_MASK);

    /* default line output to PCM */
    regmap_set_bits(regmap, JZ4770_CODEC_REG_CR_LO, REG_CR_LO_SEL_MASK);

    /* Disable stereo mic */
    regmap_clear_bits(regmap, JZ4770_CODEC_REG_CR_MIC, BIT(REG_CR_MIC_STEREO_OFFSET));

    /* Set mic 1 as default source for ADC */
    regmap_clear_bits(regmap, JZ4770_CODEC_REG_CR_ADC, REG_CR_ADC_IN_SEL_MASK);

    /* ADC/DAC: serial + i2s */
    regmap_set_bits(regmap, JZ4770_CODEC_REG_AICR_ADC, REG_AICR_ADC_SERIAL | REG_AICR_ADC_I2S);
    regmap_set_bits(regmap, JZ4770_CODEC_REG_AICR_DAC, REG_AICR_DAC_SERIAL | REG_AICR_DAC_I2S);

    /* The generated IRQ is a high level */
    regmap_clear_bits(regmap, JZ4770_CODEC_REG_ICR, REG_ICR_INT_FORM_MASK);
    regmap_update_bits(
        regmap,
        JZ4770_CODEC_REG_IMR,
        REG_IMR_ALL_MASK,
        REG_IMR_JACK_MASK | REG_IMR_RUP_MASK | REG_IMR_RDO_MASK | REG_IMR_GUP_MASK | REG_IMR_GDO_MASK,
    );

    /* 12M oscillator */
    regmap_clear_bits(regmap, JZ4770_CODEC_REG_CCR, REG_CCR_CRYSTAL_MASK);

    /* 0: 16ohm/220uF, 1: 10kohm/1uF */
    regmap_clear_bits(regmap, JZ4770_CODEC_REG_CR_HP, REG_CR_HP_LOAD);

    /* disable automatic gain */
    regmap_clear_bits(regmap, JZ4770_CODEC_REG_AGC1, REG_AGC1_EN);

    /* Disable DAC lrswap */
    regmap_set_bits(regmap, JZ4770_CODEC_REG_CR_DAC, REG_CR_DAC_LRSWAP);

    /* Independent L/R DAC gain control */
    regmap_clear_bits(regmap, JZ4770_CODEC_REG_GCR_DACL, REG_GCR_DACL_RLGOD);

    /* Disable ADC lrswap */
    regmap_set_bits(regmap, JZ4770_CODEC_REG_CR_ADC, REG_CR_ADC_LRSWAP);

    /* default to cap-less mode(0) */
    regmap_clear_bits(regmap, JZ4770_CODEC_REG_CR_HP, BIT(REG_CR_HP_SB_HPCM_OFFSET));

    /* Send collected updates. */
    regcache_cache_only(regmap, false);
    regcache_sync(regmap);
}

unsafe extern "C" fn jz4770_codec_codec_probe(codec: *mut snd_soc_component) -> c_int {
    jz4770_codec_codec_init_regs(codec);
    0
}

static jz4770_codec_soc_codec_dev: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(jz4770_codec_codec_probe),
    set_bias_level: Some(jz4770_codec_set_bias_level),
    controls: jz4770_codec_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&jz4770_codec_snd_controls),
    dapm_widgets: jz4770_codec_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&jz4770_codec_dapm_widgets),
    dapm_routes: jz4770_codec_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&jz4770_codec_dapm_routes),
    suspend_bias_off: 1,
    use_pmdown_time: 1,
};

static jz4770_codec_sample_rates: [c_uint; 11] = [
    96000, 48000, 44100, 32000,
    24000, 22050, 16000, 12000,
    11025, 9600, 8000,
];

unsafe extern "C" fn jz4770_codec_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let codec = snd_soc_component_get_drvdata((*dai).component) as *mut jz_codec;
    let bit_width: c_uint;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => bit_width = 0,
        SNDRV_PCM_FORMAT_S18_3LE => bit_width = 1,
        SNDRV_PCM_FORMAT_S20_3LE => bit_width = 2,
        SNDRV_PCM_FORMAT_S24_3LE => bit_width = 3,
        _ => return -EINVAL,
    }

    let mut rate: c_uint = 0;
    while rate < ARRAY_SIZE(&jz4770_codec_sample_rates) {
        if jz4770_codec_sample_rates[rate as usize] == params_rate(params) {
            break;
        }
        rate += 1;
    }

    if rate == ARRAY_SIZE(&jz4770_codec_sample_rates) {
        return -EINVAL;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits(
            (*codec).regmap,
            JZ4770_CODEC_REG_AICR_DAC,
            REG_AICR_DAC_ADWL_MASK,
            bit_width << REG_AICR_DAC_ADWL_OFFSET,
        );
        regmap_update_bits(
            (*codec).regmap,
            JZ4770_CODEC_REG_FCR_DAC,
            REG_FCR_DAC_FREQ_MASK,
            rate << REG_FCR_DAC_FREQ_OFFSET,
        );
    } else {
        regmap_update_bits(
            (*codec).regmap,
            JZ4770_CODEC_REG_AICR_ADC,
            REG_AICR_ADC_ADWL_MASK,
            bit_width << REG_AICR_ADC_ADWL_OFFSET,
        );
        regmap_update_bits(
            (*codec).regmap,
            JZ4770_CODEC_REG_FCR_ADC,
            REG_FCR_ADC_FREQ_MASK,
            rate << REG_FCR_ADC_FREQ_OFFSET,
        );
    }

    0
}

static jz4770_codec_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(jz4770_codec_startup),
    shutdown: Some(jz4770_codec_shutdown),
    hw_params: Some(jz4770_codec_hw_params),
    trigger: Some(jz4770_codec_pcm_trigger),
    mute_stream: Some(jz4770_codec_mute_stream),
    no_capture_mute: 1,
};

static mut jz4770_codec_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("jz4770-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("Playback"),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: JZ_CODEC_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: cstr!("Capture"),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: JZ_CODEC_FORMATS,
    },
    ops: &jz4770_codec_dai_ops,
};

unsafe extern "C" fn jz4770_codec_volatile(_dev: *mut device, reg: c_uint) -> bool_ {
    reg == JZ4770_CODEC_REG_SR || reg == JZ4770_CODEC_REG_IFR
}

unsafe extern "C" fn jz4770_codec_readable(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        JZ4770_CODEC_REG_MISSING_REG1 | JZ4770_CODEC_REG_MISSING_REG2 => false,
        _ => true,
    }
}

unsafe extern "C" fn jz4770_codec_writeable(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        JZ4770_CODEC_REG_SR | JZ4770_CODEC_REG_MISSING_REG1 | JZ4770_CODEC_REG_MISSING_REG2 => false,
        _ => true,
    }
}

unsafe fn jz4770_codec_io_wait(codec: *mut jz_codec) -> c_int {
    let mut reg: u32 = 0;

    readl_poll_timeout(
        ((*codec).base as *mut u8).add(ICDC_RGADW_OFFSET) as *mut c_void,
        &mut reg,
        |r| (r & ICDC_RGADW_RGWR) == 0,
        1000,
        1 * USEC_PER_SEC,
    )
}

unsafe extern "C" fn jz4770_codec_reg_read(
    context: *mut c_void,
    reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    let codec = context as *mut jz_codec;
    let mut i: c_uint;
    let mut tmp: u32;
    let mut ret: c_int;

    ret = jz4770_codec_io_wait(codec);
    if ret != 0 {
        return ret;
    }

    tmp = readl(((*codec).base as *mut u8).add(ICDC_RGADW_OFFSET) as *const c_void);
    tmp = (tmp & !ICDC_RGADW_RGADDR_MASK) | (reg << ICDC_RGADW_RGADDR_OFFSET);
    writel(tmp, ((*codec).base as *mut u8).add(ICDC_RGADW_OFFSET) as *mut c_void);

    /* wait 6+ cycles */
    i = 0;
    while i < 6 {
        *val = readl(((*codec).base as *mut u8).add(ICDC_RGDATA_OFFSET) as *const c_void)
            & ICDC_RGDATA_RGDOUT_MASK;
        i += 1;
    }

    0
}

unsafe extern "C" fn jz4770_codec_reg_write(
    context: *mut c_void,
    reg: c_uint,
    val: c_uint,
) -> c_int {
    let codec = context as *mut jz_codec;
    let mut ret: c_int;

    ret = jz4770_codec_io_wait(codec);
    if ret != 0 {
        return ret;
    }

    writel(
        ICDC_RGADW_RGWR | (reg << ICDC_RGADW_RGADDR_OFFSET) | val,
        ((*codec).base as *mut u8).add(ICDC_RGADW_OFFSET) as *mut c_void,
    );

    ret = jz4770_codec_io_wait(codec);
    if ret != 0 {
        return ret;
    }

    0
}

static jz4770_codec_reg_defaults: [u8; 36] = [
    0x00, 0xC3, 0xC3, 0x90, 0x98, 0xFF, 0x90, 0xB1,
    0x11, 0x10, 0x00, 0x03, 0x00, 0x00, 0x40, 0x00,
    0xFF, 0x00, 0x06, 0x06, 0x06, 0x06, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x34,
    0x07, 0x44, 0x1F, 0x00,
];

static jz4770_codec_regmap_config: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 8,
    max_register: JZ4770_CODEC_REG_AGC5,
    volatile_reg: Some(jz4770_codec_volatile),
    readable_reg: Some(jz4770_codec_readable),
    writeable_reg: Some(jz4770_codec_writeable),
    reg_read: Some(jz4770_codec_reg_read),
    reg_write: Some(jz4770_codec_reg_write),
    reg_defaults_raw: jz4770_codec_reg_defaults.as_ptr(),
    num_reg_defaults_raw: ARRAY_SIZE(&jz4770_codec_reg_defaults),
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn jz4770_codec_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let codec: *mut jz_codec;
    let clk: *mut clk;
    let mut ret: c_int;

    codec = devm_kzalloc(dev, size_of::<jz_codec>(), GFP_KERNEL) as *mut jz_codec;
    if codec.is_null() {
        return -ENOMEM;
    }

    (*codec).dev = dev;

    (*codec).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*codec).base) {
        return PTR_ERR((*codec).base);
    }

    (*codec).regmap = devm_regmap_init(dev, ptr::null(), codec as *mut c_void, &jz4770_codec_regmap_config);
    if IS_ERR((*codec).regmap as *const c_void) {
        return PTR_ERR((*codec).regmap as *const c_void);
    }

    clk = devm_clk_get_enabled(dev, cstr!("aic"));
    if IS_ERR(clk as *const c_void) {
        return PTR_ERR(clk as *const c_void);
    }

    platform_set_drvdata(pdev, codec as *mut c_void);

    ret = devm_snd_soc_register_component(
        dev,
        &jz4770_codec_soc_codec_dev,
        &mut jz4770_codec_dai,
        1,
    );
    if ret != 0 {
        dev_err(dev, cstr!("Failed to register codec: %d\n"), ret);
        return ret;
    }

    0
}

static jz4770_codec_of_matches: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("ingenic,jz4770-codec") },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, jz4770_codec_of_matches); */

static mut jz4770_codec_driver: platform_driver = platform_driver {
    probe: Some(jz4770_codec_probe),
    driver: platform_driver_driver {
        name: cstr!("jz4770-codec"),
        of_match_table: jz4770_codec_of_matches.as_ptr(),
    },
};
/* module_platform_driver(jz4770_codec_driver); */

/* MODULE_DESCRIPTION("JZ4770 SoC internal codec driver"); */
/* MODULE_AUTHOR("Maarten ter Huurne <maarten@treewalker.org>"); */
/* MODULE_AUTHOR("Paul Cercueil <paul@crapouillou.net>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
