// SPDX-License-Identifier: GPL-2.0-only
//
// nau8821.c -- Nuvoton NAU88L21 audio codec driver
//
// Copyright 2021 Nuvoton Technology Corp.
// Author: John Hsu <kchsu0@nuvoton.com>
// Co-author: Seven Lee <wtli@nuvoton.com>
//
// Translated from C source ./soc/codecs/nau8821.c.
// External Linux/ASoC/regmap symbols and macros are referenced as future
// dependencies supplied by the surrounding driver tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type bool_ = bool;
type u64_ = u64;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub union snd_ctl_elem_value_value { pub bytes: snd_ctl_elem_value_bytes }
#[repr(C)] pub struct snd_ctl_elem_value_bytes { pub data: *mut c_void }
#[repr(C)] pub struct snd_soc_component { pub regmap: *mut regmap }
#[repr(C)] pub struct soc_bytes_ext { pub max: c_uint }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int, pub runtime: *mut c_void }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct i2c_client { pub dev: device, pub irq: c_int }
#[repr(C)] pub struct dmi_system_id { pub matches: [c_ulong; 4], pub driver_data: *mut c_void }
#[repr(C)] pub struct i2c_device_id { pub name: [c_char; 20], pub driver_data: c_ulong }
#[repr(C)] pub struct of_device_id { pub compatible: *const c_char }
#[repr(C)] pub struct acpi_device_id { pub id: [c_char; 16], pub driver_data: c_ulong }

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)] pub struct soc_enum { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct regmap_config { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component_driver { _private: [u8; 0] }
#[repr(C)] pub struct i2c_driver { _private: [u8; 0] }

#[repr(C)]
pub struct nau8821 {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub dapm: *mut snd_soc_dapm_context,
    pub jack: *mut snd_soc_jack,
    pub jdet_work: delayed_work,
    pub irq: c_int,
    pub fs: c_int,
    pub clk_id: c_int,
    pub jkdet_enable: bool,
    pub jkdet_pull_enable: bool,
    pub jkdet_pull_up: bool,
    pub key_enable: bool,
    pub left_input_single_end: bool,
    pub jdet_active: bool,
    pub jkdet_polarity: c_uint,
    pub micbias_voltage: c_uint,
    pub vref_impedance: c_uint,
    pub jack_insert_debounce: c_uint,
    pub jack_eject_debounce: c_uint,
    pub dmic_clk_threshold: c_uint,
    pub dmic_slew_rate: c_uint,
    pub adc_delay: c_uint,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut nau8821;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn regmap_raw_read(map: *mut regmap, reg: c_uint, val: *mut c_void, len: c_uint) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: c_uint, val: *const c_void, len: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn kmemdup(src: *const c_void, len: c_uint, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn msleep(msecs: c_uint);
    fn mdelay(msecs: c_uint);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_hw_constraint_minmax(runtime: *mut c_void, param: c_int, min: c_uint, max: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: c_int) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> bool;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *const c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn disable_irq(irq: c_int);
    fn enable_irq(irq: c_int);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn device_property_read_bool(dev: *mut device, prop: *const c_char) -> bool;
    fn device_property_read_u32(dev: *mut device, prop: *const c_char, val: *mut c_uint) -> c_int;
    fn dmi_first_match(ids: *const dmi_system_id) -> *const dmi_system_id;
    fn dev_get_platdata(dev: *mut device) -> *mut nau8821;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
}

type irqreturn_t = c_int;

const fn BIT(n: c_uint) -> c_uint { 1u32 << n }
const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize { N }
fn div_u64(n: u64, d: u64) -> u64 { n / d }

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GFP_DMA: c_uint = 0;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQF_TRIGGER_LOW: c_uint = 0;
const IRQF_ONESHOT: c_uint = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x4;
const SND_SOC_DAPM_PRE_PMD: c_int = 0x8;
const SND_SOC_BIAS_OFF: c_int = 0;
const SND_SOC_BIAS_STANDBY: c_int = 1;
const SND_SOC_BIAS_PREPARE: c_int = 2;
const SND_SOC_BIAS_ON: c_int = 3;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;

// Constants from nau8821.h and Linux/ASoC headers are expected dependencies.
// They are referenced below exactly by their C names in Rust expression form.

const NAU8821_QUIRK_JD_ACTIVE_HIGH: c_int = BIT(0) as c_int;
const NAU8821_QUIRK_JD_DB_BYPASS: c_int = BIT(1) as c_int;

static mut nau8821_quirk: c_int = 0;
static mut quirk_override: c_int = -1;
// module_param_named(quirk, quirk_override, uint, 0444);
// MODULE_PARM_DESC(quirk, "Board-specific quirk override");

const NAU_FREF_MAX: c_uint = 13500000;
const NAU_FVCO_MAX: u64 = 100000000;
const NAU_FVCO_MIN: u64 = 90000000;
const NAU8821_BUTTON: c_int = SND_JACK_BTN_0;

/* the maximum frequency of CLK_ADC and CLK_DAC */
const CLK_DA_AD_MAX: c_int = 6144000;

#[repr(C)]
struct nau8821_fll {
    mclk_src: c_int,
    ratio: c_int,
    fll_frac: c_int,
    fll_int: c_int,
    clk_ref_div: c_int,
}

#[repr(C)]
struct nau8821_fll_attr {
    param: c_uint,
    val: c_uint,
}

/* scaling for mclk from sysclk_src output */
static mclk_src_scaling: [nau8821_fll_attr; 13] = [
    nau8821_fll_attr { param: 1, val: 0x0 },
    nau8821_fll_attr { param: 2, val: 0x2 },
    nau8821_fll_attr { param: 4, val: 0x3 },
    nau8821_fll_attr { param: 8, val: 0x4 },
    nau8821_fll_attr { param: 16, val: 0x5 },
    nau8821_fll_attr { param: 32, val: 0x6 },
    nau8821_fll_attr { param: 3, val: 0x7 },
    nau8821_fll_attr { param: 6, val: 0xa },
    nau8821_fll_attr { param: 12, val: 0xb },
    nau8821_fll_attr { param: 24, val: 0xc },
    nau8821_fll_attr { param: 48, val: 0xd },
    nau8821_fll_attr { param: 96, val: 0xe },
    nau8821_fll_attr { param: 5, val: 0xf },
];

/* ratio for input clk freq */
static fll_ratio: [nau8821_fll_attr; 7] = [
    nau8821_fll_attr { param: 512000, val: 0x01 },
    nau8821_fll_attr { param: 256000, val: 0x02 },
    nau8821_fll_attr { param: 128000, val: 0x04 },
    nau8821_fll_attr { param: 64000, val: 0x08 },
    nau8821_fll_attr { param: 32000, val: 0x10 },
    nau8821_fll_attr { param: 8000, val: 0x20 },
    nau8821_fll_attr { param: 4000, val: 0x40 },
];

static fll_pre_scalar: [nau8821_fll_attr; 4] = [
    nau8821_fll_attr { param: 0, val: 0x0 },
    nau8821_fll_attr { param: 1, val: 0x1 },
    nau8821_fll_attr { param: 2, val: 0x2 },
    nau8821_fll_attr { param: 3, val: 0x3 },
];

/* over sampling rate */
#[repr(C)]
struct nau8821_osr_attr {
    osr: c_uint,
    clk_src: c_uint,
}

static osr_dac_sel: [nau8821_osr_attr; 5] = [
    nau8821_osr_attr { osr: 64, clk_src: 2 },  /* OSR 64, SRC 1/4 */
    nau8821_osr_attr { osr: 256, clk_src: 0 }, /* OSR 256, SRC 1 */
    nau8821_osr_attr { osr: 128, clk_src: 1 }, /* OSR 128, SRC 1/2 */
    nau8821_osr_attr { osr: 0, clk_src: 0 },
    nau8821_osr_attr { osr: 32, clk_src: 3 },  /* OSR 32, SRC 1/8 */
];

static osr_adc_sel: [nau8821_osr_attr; 4] = [
    nau8821_osr_attr { osr: 32, clk_src: 3 },  /* OSR 32, SRC 1/8 */
    nau8821_osr_attr { osr: 64, clk_src: 2 },  /* OSR 64, SRC 1/4 */
    nau8821_osr_attr { osr: 128, clk_src: 1 }, /* OSR 128, SRC 1/2 */
    nau8821_osr_attr { osr: 256, clk_src: 0 }, /* OSR 256, SRC 1 */
];

#[repr(C)]
struct nau8821_dmic_speed {
    param: c_uint,
    val: c_uint,
}

static dmic_speed_sel: [nau8821_dmic_speed; 4] = [
    nau8821_dmic_speed { param: 0, val: 0x0 }, /*SPEED 1, SRC 1 */
    nau8821_dmic_speed { param: 1, val: 0x1 }, /*SPEED 2, SRC 1/2 */
    nau8821_dmic_speed { param: 2, val: 0x2 }, /*SPEED 4, SRC 1/4 */
    nau8821_dmic_speed { param: 3, val: 0x3 }, /*SPEED 8, SRC 1/8 */
];

static nau8821_reg_defaults: [reg_default; 77] = [
    reg_default { reg: NAU8821_R01_ENA_CTRL, def: 0x00ff },
    reg_default { reg: NAU8821_R03_CLK_DIVIDER, def: 0x0050 },
    reg_default { reg: NAU8821_R04_FLL1, def: 0x0 },
    reg_default { reg: NAU8821_R05_FLL2, def: 0x00bc },
    reg_default { reg: NAU8821_R06_FLL3, def: 0x0008 },
    reg_default { reg: NAU8821_R07_FLL4, def: 0x0010 },
    reg_default { reg: NAU8821_R08_FLL5, def: 0x4000 },
    reg_default { reg: NAU8821_R09_FLL6, def: 0x6900 },
    reg_default { reg: NAU8821_R0A_FLL7, def: 0x0031 },
    reg_default { reg: NAU8821_R0B_FLL8, def: 0x26e9 },
    reg_default { reg: NAU8821_R0D_JACK_DET_CTRL, def: 0x0 },
    reg_default { reg: NAU8821_R0F_INTERRUPT_MASK, def: 0x0 },
    reg_default { reg: NAU8821_R12_INTERRUPT_DIS_CTRL, def: 0xffff },
    reg_default { reg: NAU8821_R13_DMIC_CTRL, def: 0x0 },
    reg_default { reg: NAU8821_R1A_GPIO12_CTRL, def: 0x0 },
    reg_default { reg: NAU8821_R1B_TDM_CTRL, def: 0x0 },
    reg_default { reg: NAU8821_R1C_I2S_PCM_CTRL1, def: 0x000a },
    reg_default { reg: NAU8821_R1D_I2S_PCM_CTRL2, def: 0x8010 },
    reg_default { reg: NAU8821_R1E_LEFT_TIME_SLOT, def: 0x0 },
    reg_default { reg: NAU8821_R1F_RIGHT_TIME_SLOT, def: 0x0 },
    reg_default { reg: NAU8821_R21_BIQ0_COF1, def: 0x0 },
    reg_default { reg: NAU8821_R22_BIQ0_COF2, def: 0x0 },
    reg_default { reg: NAU8821_R23_BIQ0_COF3, def: 0x0 },
    reg_default { reg: NAU8821_R24_BIQ0_COF4, def: 0x0 },
    reg_default { reg: NAU8821_R25_BIQ0_COF5, def: 0x0 },
    reg_default { reg: NAU8821_R26_BIQ0_COF6, def: 0x0 },
    reg_default { reg: NAU8821_R27_BIQ0_COF7, def: 0x0 },
    reg_default { reg: NAU8821_R28_BIQ0_COF8, def: 0x0 },
    reg_default { reg: NAU8821_R29_BIQ0_COF9, def: 0x0 },
    reg_default { reg: NAU8821_R2A_BIQ0_COF10, def: 0x0 },
    reg_default { reg: NAU8821_R2B_ADC_RATE, def: 0x0002 },
    reg_default { reg: NAU8821_R2C_DAC_CTRL1, def: 0x0082 },
    reg_default { reg: NAU8821_R2D_DAC_CTRL2, def: 0x0 },
    reg_default { reg: NAU8821_R2F_DAC_DGAIN_CTRL, def: 0x0 },
    reg_default { reg: NAU8821_R30_ADC_DGAIN_CTRL, def: 0x0 },
    reg_default { reg: NAU8821_R31_MUTE_CTRL, def: 0x0 },
    reg_default { reg: NAU8821_R32_HSVOL_CTRL, def: 0x0 },
    reg_default { reg: NAU8821_R34_DACR_CTRL, def: 0xcfcf },
    reg_default { reg: NAU8821_R35_ADC_DGAIN_CTRL1, def: 0xcfcf },
    reg_default { reg: NAU8821_R36_ADC_DRC_KNEE_IP12, def: 0x1486 },
    reg_default { reg: NAU8821_R37_ADC_DRC_KNEE_IP34, def: 0x0f12 },
    reg_default { reg: NAU8821_R38_ADC_DRC_SLOPES, def: 0x25ff },
    reg_default { reg: NAU8821_R39_ADC_DRC_ATKDCY, def: 0x3457 },
    reg_default { reg: NAU8821_R3A_DAC_DRC_KNEE_IP12, def: 0x1486 },
    reg_default { reg: NAU8821_R3B_DAC_DRC_KNEE_IP34, def: 0x0f12 },
    reg_default { reg: NAU8821_R3C_DAC_DRC_SLOPES, def: 0x25f9 },
    reg_default { reg: NAU8821_R3D_DAC_DRC_ATKDCY, def: 0x3457 },
    reg_default { reg: NAU8821_R41_BIQ1_COF1, def: 0x0 },
    reg_default { reg: NAU8821_R42_BIQ1_COF2, def: 0x0 },
    reg_default { reg: NAU8821_R43_BIQ1_COF3, def: 0x0 },
    reg_default { reg: NAU8821_R44_BIQ1_COF4, def: 0x0 },
    reg_default { reg: NAU8821_R45_BIQ1_COF5, def: 0x0 },
    reg_default { reg: NAU8821_R46_BIQ1_COF6, def: 0x0 },
    reg_default { reg: NAU8821_R47_BIQ1_COF7, def: 0x0 },
    reg_default { reg: NAU8821_R48_BIQ1_COF8, def: 0x0 },
    reg_default { reg: NAU8821_R49_BIQ1_COF9, def: 0x0 },
    reg_default { reg: NAU8821_R4A_BIQ1_COF10, def: 0x0 },
    reg_default { reg: NAU8821_R4B_CLASSG_CTRL, def: 0x0 },
    reg_default { reg: NAU8821_R4C_IMM_MODE_CTRL, def: 0x0 },
    reg_default { reg: NAU8821_R4D_IMM_RMS_L, def: 0x0 },
    reg_default { reg: NAU8821_R53_OTPDOUT_1, def: 0xaad8 },
    reg_default { reg: NAU8821_R54_OTPDOUT_2, def: 0x0002 },
    reg_default { reg: NAU8821_R55_MISC_CTRL, def: 0x0 },
    reg_default { reg: NAU8821_R66_BIAS_ADJ, def: 0x0 },
    reg_default { reg: NAU8821_R68_TRIM_SETTINGS, def: 0x0 },
    reg_default { reg: NAU8821_R69_ANALOG_CONTROL_1, def: 0x0 },
    reg_default { reg: NAU8821_R6A_ANALOG_CONTROL_2, def: 0x0 },
    reg_default { reg: NAU8821_R6B_PGA_MUTE, def: 0x0 },
    reg_default { reg: NAU8821_R71_ANALOG_ADC_1, def: 0x0011 },
    reg_default { reg: NAU8821_R72_ANALOG_ADC_2, def: 0x0020 },
    reg_default { reg: NAU8821_R73_RDAC, def: 0x0008 },
    reg_default { reg: NAU8821_R74_MIC_BIAS, def: 0x0006 },
    reg_default { reg: NAU8821_R76_BOOST, def: 0x0 },
    reg_default { reg: NAU8821_R77_FEPGA, def: 0x0 },
    reg_default { reg: NAU8821_R7E_PGA_GAIN, def: 0x0 },
    reg_default { reg: NAU8821_R7F_POWER_UP_CONTROL, def: 0x0 },
    reg_default { reg: NAU8821_R80_CHARGE_PUMP, def: 0x0 },
];

unsafe fn nau8821_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        NAU8821_R00_RESET..=NAU8821_R01_ENA_CTRL
        | NAU8821_R03_CLK_DIVIDER..=NAU8821_R0B_FLL8
        | NAU8821_R0D_JACK_DET_CTRL
        | NAU8821_R0F_INTERRUPT_MASK..=NAU8821_R13_DMIC_CTRL
        | NAU8821_R1A_GPIO12_CTRL..=NAU8821_R1F_RIGHT_TIME_SLOT
        | NAU8821_R21_BIQ0_COF1..=NAU8821_R2D_DAC_CTRL2
        | NAU8821_R2F_DAC_DGAIN_CTRL..=NAU8821_R32_HSVOL_CTRL
        | NAU8821_R34_DACR_CTRL..=NAU8821_R3D_DAC_DRC_ATKDCY
        | NAU8821_R41_BIQ1_COF1..=NAU8821_R4F_FUSE_CTRL3
        | NAU8821_R51_FUSE_CTRL1
        | NAU8821_R53_OTPDOUT_1..=NAU8821_R55_MISC_CTRL
        | NAU8821_R58_I2C_DEVICE_ID..=NAU8821_R5A_SOFTWARE_RST
        | NAU8821_R66_BIAS_ADJ
        | NAU8821_R68_TRIM_SETTINGS..=NAU8821_R6B_PGA_MUTE
        | NAU8821_R71_ANALOG_ADC_1..=NAU8821_R74_MIC_BIAS
        | NAU8821_R76_BOOST..=NAU8821_R77_FEPGA
        | NAU8821_R7E_PGA_GAIN..=NAU8821_R82_GENERAL_STATUS => true,
        _ => false,
    }
}

unsafe fn nau8821_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        NAU8821_R00_RESET..=NAU8821_R01_ENA_CTRL
        | NAU8821_R03_CLK_DIVIDER..=NAU8821_R0B_FLL8
        | NAU8821_R0D_JACK_DET_CTRL
        | NAU8821_R0F_INTERRUPT_MASK
        | NAU8821_R11_INT_CLR_KEY_STATUS..=NAU8821_R13_DMIC_CTRL
        | NAU8821_R1A_GPIO12_CTRL..=NAU8821_R1F_RIGHT_TIME_SLOT
        | NAU8821_R21_BIQ0_COF1..=NAU8821_R2D_DAC_CTRL2
        | NAU8821_R2F_DAC_DGAIN_CTRL..=NAU8821_R32_HSVOL_CTRL
        | NAU8821_R34_DACR_CTRL..=NAU8821_R3D_DAC_DRC_ATKDCY
        | NAU8821_R41_BIQ1_COF1..=NAU8821_R4C_IMM_MODE_CTRL
        | NAU8821_R4E_FUSE_CTRL2..=NAU8821_R4F_FUSE_CTRL3
        | NAU8821_R51_FUSE_CTRL1
        | NAU8821_R55_MISC_CTRL
        | NAU8821_R5A_SOFTWARE_RST
        | NAU8821_R66_BIAS_ADJ
        | NAU8821_R68_TRIM_SETTINGS..=NAU8821_R6B_PGA_MUTE
        | NAU8821_R71_ANALOG_ADC_1..=NAU8821_R74_MIC_BIAS
        | NAU8821_R76_BOOST..=NAU8821_R77_FEPGA
        | NAU8821_R7E_PGA_GAIN..=NAU8821_R80_CHARGE_PUMP => true,
        _ => false,
    }
}

unsafe fn nau8821_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        NAU8821_R00_RESET
        | NAU8821_R10_IRQ_STATUS..=NAU8821_R11_INT_CLR_KEY_STATUS
        | NAU8821_R21_BIQ0_COF1..=NAU8821_R2A_BIQ0_COF10
        | NAU8821_R41_BIQ1_COF1..=NAU8821_R4A_BIQ1_COF10
        | NAU8821_R4D_IMM_RMS_L
        | NAU8821_R53_OTPDOUT_1..=NAU8821_R54_OTPDOUT_2
        | NAU8821_R58_I2C_DEVICE_ID..=NAU8821_R5A_SOFTWARE_RST
        | NAU8821_R81_CHARGE_PUMP_INPUT_READ..=NAU8821_R82_GENERAL_STATUS => true,
        _ => false,
    }
}

unsafe fn nau8821_biq_coeff_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let params = (*kcontrol).private_value as *mut soc_bytes_ext;
    if (*component).regmap.is_null() {
        return -EINVAL;
    }
    regmap_raw_read((*component).regmap, NAU8821_R21_BIQ0_COF1, (*ucontrol).value.bytes.data, (*params).max)
}

unsafe fn nau8821_biq_coeff_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let params = (*kcontrol).private_value as *mut soc_bytes_ext;
    let data: *mut c_void;
    let ret: c_int;

    if (*component).regmap.is_null() {
        return -EINVAL;
    }
    data = kmemdup((*ucontrol).value.bytes.data, (*params).max, GFP_KERNEL | GFP_DMA);
    if data.is_null() {
        return -ENOMEM;
    }
    ret = regmap_raw_write((*component).regmap, NAU8821_R21_BIQ0_COF1, data, (*params).max);
    kfree(data);
    ret
}

static nau8821_adc_decimation: [&[u8]; 4] = [b"32\0", b"64\0", b"128\0", b"256\0"];
static nau8821_dac_oversampl: [&[u8]; 5] = [b"64\0", b"256\0", b"128\0", b"\0", b"32\0"];
static nau8821_adc_drc_noise_gate: [&[u8]; 4] = [b"1:1\0", b"2:1\0", b"4:1\0", b"8:1\0"];
static nau8821_adc_drc_expansion_slope: [&[u8]; 3] = [b"1:1\0", b"2:1\0", b"4:1\0"];
static nau8821_adc_drc_lower_region: [&[u8]; 8] = [b"0\0", b"1:2\0", b"1:4\0", b"1:8\0", b"1:16\0", b"\0", b"\0", b"1:1\0"];
static nau8821_higher_region: [&[u8]; 8] = [b"0\0", b"1:2\0", b"1:4\0", b"1:8\0", b"1:16\0", b"\0", b"\0", b"1:1\0"];
static nau8821_limiter_slope: [&[u8]; 8] = [b"0\0", b"1:2\0", b"1:4\0", b"1:8\0", b"1:16\0", b"1:32\0", b"1:64\0", b"1:1\0"];
static nau8821_detection_attack_time: [&[u8]; 10] = [b"Ts\0", b"3Ts\0", b"7Ts\0", b"15Ts\0", b"31Ts\0", b"63Ts\0", b"127Ts\0", b"255Ts\0", b"\0", b"511Ts\0"];
static nau8821_detection_release_time: [&[u8]; 10] = [b"63Ts\0", b"127Ts\0", b"255Ts\0", b"511Ts\0", b"1023Ts\0", b"2047Ts\0", b"4095Ts\0", b"8191Ts\0", b"\0", b"16383Ts\0"];
static nau8821_attack_time: [&[u8]; 13] = [b"Ts\0", b"3Ts\0", b"7Ts\0", b"15Ts\0", b"31Ts\0", b"63Ts\0", b"127Ts\0", b"255Ts\0", b"511Ts\0", b"1023Ts\0", b"2047Ts\0", b"4095Ts\0", b"8191Ts\0"];
static nau8821_decay_time: [&[u8]; 11] = [b"63Ts\0", b"127Ts\0", b"255Ts\0", b"511Ts\0", b"1023Ts\0", b"2047Ts\0", b"4095Ts\0", b"8191Ts\0", b"16383Ts\0", b"32757Ts\0", b"65535Ts\0"];

// SOC_ENUM_SINGLE, DECLARE_TLV_DB_*, SOC_* and SND_SOC_DAPM_* static tables
// from the C source are external macro-generated ASoC data. Their translated
// Rust declarations intentionally preserve the source-level construction names.
macro_rules! external_asoc_table { ($($tt:tt)*) => {}; }
external_asoc_table! {
static const nau8821_adc_decimation_enum: soc_enum =
    SOC_ENUM_SINGLE(NAU8821_R2B_ADC_RATE, NAU8821_ADC_SYNC_DOWN_SFT,
        ARRAY_SIZE(nau8821_adc_decimation), nau8821_adc_decimation);
static const nau8821_dac_oversampl_enum: soc_enum =
    SOC_ENUM_SINGLE(NAU8821_R2C_DAC_CTRL1, NAU8821_DAC_OVERSAMPLE_SFT,
        ARRAY_SIZE(nau8821_dac_oversampl), nau8821_dac_oversampl);
static const nau8821_adc_drc_noise_gate_enum: soc_enum =
    SOC_ENUM_SINGLE(NAU8821_R38_ADC_DRC_SLOPES, NAU8821_DRC_NG_SLP_ADC_SFT,
        ARRAY_SIZE(nau8821_adc_drc_noise_gate), nau8821_adc_drc_noise_gate);
static const nau8821_adc_drc_expansion_slope_enum: soc_enum =
    SOC_ENUM_SINGLE(NAU8821_R38_ADC_DRC_SLOPES, NAU8821_DRC_EXP_SLP_ADC_SFT,
        ARRAY_SIZE(nau8821_adc_drc_expansion_slope), nau8821_adc_drc_expansion_slope);
static const nau8821_adc_drc_lower_region_enum: soc_enum =
    SOC_ENUM_SINGLE(NAU8821_R38_ADC_DRC_SLOPES, NAU8821_DRC_CMP2_SLP_ADC_SFT,
        ARRAY_SIZE(nau8821_adc_drc_lower_region), nau8821_adc_drc_lower_region);
static const nau8821_higher_region_enum: soc_enum =
    SOC_ENUM_SINGLE(NAU8821_R38_ADC_DRC_SLOPES, NAU8821_DRC_CMP1_SLP_ADC_SFT,
        ARRAY_SIZE(nau8821_higher_region), nau8821_higher_region);
static const nau8821_limiter_slope_enum: soc_enum =
    SOC_ENUM_SINGLE(NAU8821_R38_ADC_DRC_SLOPES, NAU8821_DRC_LMT_SLP_ADC_SFT,
        ARRAY_SIZE(nau8821_limiter_slope), nau8821_limiter_slope);
static const nau8821_detection_attack_time_enum: soc_enum =
    SOC_ENUM_SINGLE(NAU8821_R39_ADC_DRC_ATKDCY, NAU8821_DRC_PK_COEF1_ADC_SFT,
        ARRAY_SIZE(nau8821_detection_attack_time), nau8821_detection_attack_time);
static const nau8821_detection_release_time_enum: soc_enum =
    SOC_ENUM_SINGLE(NAU8821_R39_ADC_DRC_ATKDCY, NAU8821_DRC_PK_COEF2_ADC_SFT,
        ARRAY_SIZE(nau8821_detection_release_time), nau8821_detection_release_time);
static const nau8821_attack_time_enum: soc_enum =
    SOC_ENUM_SINGLE(NAU8821_R39_ADC_DRC_ATKDCY, NAU8821_DRC_ATK_ADC_SFT,
        ARRAY_SIZE(nau8821_attack_time), nau8821_attack_time);
static const nau8821_decay_time_enum: soc_enum =
    SOC_ENUM_SINGLE(NAU8821_R39_ADC_DRC_ATKDCY, NAU8821_DRC_DCY_ADC_SFT,
        ARRAY_SIZE(nau8821_decay_time), nau8821_decay_time);
static const DECLARE_TLV_DB_MINMAX_MUTE(adc_vol_tlv, -6600, 2400);
static const DECLARE_TLV_DB_MINMAX_MUTE(sidetone_vol_tlv, -4200, 0);
static const DECLARE_TLV_DB_MINMAX(hp_vol_tlv, -900, 0);
static const DECLARE_TLV_DB_SCALE(playback_vol_tlv, -6600, 50, 1);
static const DECLARE_TLV_DB_MINMAX(fepga_gain_tlv, -100, 3600);
static const DECLARE_TLV_DB_MINMAX_MUTE(crosstalk_vol_tlv, -7000, 2400);
static const DECLARE_TLV_DB_MINMAX(drc_knee4_tlv, -9800, -3500);
static const DECLARE_TLV_DB_MINMAX(drc_knee3_tlv, -8100, -1800);
static const nau8821_controls: [snd_kcontrol_new; 22] = { /* SOC_* entries from C source */ };
static const nau8821_dmic_mode_switch: snd_kcontrol_new =
    SOC_DAPM_SINGLE("Switch", NAU8821_R13_DMIC_CTRL, NAU8821_DMIC_EN_SFT, 1, 0);
static const nau8821_dapm_widgets: [snd_soc_dapm_widget; 39] = { /* SND_SOC_DAPM_* entries from C source */ };
static const nau8821_dapm_routes: [snd_soc_dapm_route; 45] = { /* routes from C source */ };
static const nau8821_dai_ops: snd_soc_dai_ops = {
    .startup = nau8821_dai_startup,
    .hw_params = nau8821_hw_params,
    .set_fmt = nau8821_set_dai_fmt,
    .mute_stream = nau8821_digital_mute,
    .no_capture_mute = 1,
};
static mut nau8821_dai: snd_soc_dai_driver = { /* C initializer preserved by dependency macros */ };
static const nau8821_regmap_config: regmap_config = { /* C regmap_config initializer */ };
static const nau8821_component_driver: snd_soc_component_driver = { /* C component driver initializer */ };
static const nau8821_quirk_table: [dmi_system_id; 4] = { /* DMI_MATCH table from C source */ };
static const nau8821_i2c_ids: [i2c_device_id; 2] = { /* MODULE_DEVICE_TABLE(i2c, nau8821_i2c_ids) */ };
/* CONFIG_OF: static const nau8821_of_ids: [of_device_id; 2] = ...; MODULE_DEVICE_TABLE(of, nau8821_of_ids); */
/* CONFIG_ACPI: static const nau8821_acpi_match: [acpi_device_id; 2] = ...; MODULE_DEVICE_TABLE(acpi, nau8821_acpi_match); */
static mut nau8821_driver: i2c_driver = { /* module_i2c_driver(nau8821_driver) */ };
}

unsafe fn dmic_clock_control(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, _event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8821 = snd_soc_component_get_drvdata(component);
    let mut speed_selection: c_int = -1;
    let mut clk_divider_r03: c_int = 0;

    /* The DMIC clock is gotten from adc clock divided by
     * CLK_DMIC_SRC (1, 2, 4, 8). The clock has to be equal or
     * less than nau8821->dmic_clk_threshold.
     */
    regmap_read((*nau8821).regmap, NAU8821_R03_CLK_DIVIDER, &mut clk_divider_r03);
    let clk_adc_src = ((clk_divider_r03 as c_uint & NAU8821_CLK_ADC_SRC_MASK) >> NAU8821_CLK_ADC_SRC_SFT) as c_int;
    let clk_adc = ((*nau8821).fs * 256) >> clk_adc_src;

    let mut i = 0usize;
    while i < 4 {
        if (clk_adc >> dmic_speed_sel[i].param) <= (*nau8821).dmic_clk_threshold as c_int {
            speed_selection = dmic_speed_sel[i].val as c_int;
            break;
        }
        i += 1;
    }
    if i == 4 {
        return -EINVAL;
    }

    dev_dbg((*nau8821).dev, b"clk_adc=%d, dmic_clk_threshold = %d, param=%d, val = %d\n\0".as_ptr() as *const c_char,
        clk_adc, (*nau8821).dmic_clk_threshold, dmic_speed_sel[i].param, dmic_speed_sel[i].val);
    regmap_update_bits((*nau8821).regmap, NAU8821_R13_DMIC_CTRL,
        NAU8821_DMIC_SRC_MASK, (speed_selection as c_uint) << NAU8821_DMIC_SRC_SFT);
    0
}

unsafe fn nau8821_left_adc_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8821 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => msleep((*nau8821).adc_delay),
        SND_SOC_DAPM_POST_PMD => {}
        _ => return -EINVAL,
    }
    0
}

unsafe fn nau8821_right_adc_event(w: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    nau8821_left_adc_event(w, kcontrol, event)
}

unsafe fn nau8821_pump_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8821 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_POST_PMU => {
            /* Prevent startup click by letting charge pump to ramp up */
            msleep(20);
            regmap_update_bits((*nau8821).regmap, NAU8821_R80_CHARGE_PUMP, NAU8821_JAMNODCLOW, NAU8821_JAMNODCLOW);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_update_bits((*nau8821).regmap, NAU8821_R80_CHARGE_PUMP, NAU8821_JAMNODCLOW, 0);
        }
        _ => return -EINVAL,
    }
    0
}

unsafe fn nau8821_output_dac_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8821 = snd_soc_component_get_drvdata(component);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* Disables the TESTDAC to let DAC signal pass through. */
            regmap_update_bits((*nau8821).regmap, NAU8821_R66_BIAS_ADJ, NAU8821_BIAS_TESTDAC_EN, 0);
        }
        SND_SOC_DAPM_POST_PMD => {
            regmap_update_bits((*nau8821).regmap, NAU8821_R66_BIAS_ADJ, NAU8821_BIAS_TESTDAC_EN, NAU8821_BIAS_TESTDAC_EN);
        }
        _ => return -EINVAL,
    }
    0
}

unsafe fn SND_SOC_DAPM_EVENT_OFF(event: c_int) -> bool { event != 0 }

unsafe fn system_clock_control(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8821 = snd_soc_component_get_drvdata(component);

    if SND_SOC_DAPM_EVENT_OFF(event) {
        dev_dbg((*nau8821).dev, b"system clock control : POWER OFF\n\0".as_ptr() as *const c_char);
        /* Set clock source to disable or internal clock before the
         * playback or capture end. Codec needs clock for Jack
         * detection and button press if jack inserted; otherwise,
         * the clock should be closed.
         */
        if nau8821_is_jack_inserted((*nau8821).regmap) {
            nau8821_configure_sysclk(nau8821, NAU8821_CLK_INTERNAL, 0);
        } else {
            nau8821_configure_sysclk(nau8821, NAU8821_CLK_DIS, 0);
        }
    }
    0
}

unsafe fn nau8821_left_fepga_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8821 = snd_soc_component_get_drvdata(component);

    if !(*nau8821).left_input_single_end {
        return 0;
    }
    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_update_bits((*nau8821).regmap, NAU8821_R77_FEPGA,
                NAU8821_ACDC_CTRL_MASK | NAU8821_FEPGA_MODEL_MASK,
                NAU8821_ACDC_VREF_MICN | NAU8821_FEPGA_MODEL_AAF);
            regmap_update_bits((*nau8821).regmap, NAU8821_R76_BOOST,
                NAU8821_HP_BOOST_DISCHRG_EN, NAU8821_HP_BOOST_DISCHRG_EN);
        }
        SND_SOC_DAPM_POST_PMD => {
            regmap_update_bits((*nau8821).regmap, NAU8821_R77_FEPGA,
                NAU8821_ACDC_CTRL_MASK | NAU8821_FEPGA_MODEL_MASK, 0);
            regmap_update_bits((*nau8821).regmap, NAU8821_R76_BOOST,
                NAU8821_HP_BOOST_DISCHRG_EN, 0);
        }
        _ => {}
    }
    0
}

unsafe fn nau8821_get_osr(nau8821: *mut nau8821, stream: c_int) -> *const nau8821_osr_attr {
    let mut osr: c_int = 0;
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_read((*nau8821).regmap, NAU8821_R2C_DAC_CTRL1, &mut osr);
        osr &= NAU8821_DAC_OVERSAMPLE_MASK as c_int;
        if osr as usize >= osr_dac_sel.len() {
            return ptr::null();
        }
        return &osr_dac_sel[osr as usize] as *const _;
    }
    regmap_read((*nau8821).regmap, NAU8821_R2B_ADC_RATE, &mut osr);
    osr &= NAU8821_ADC_SYNC_DOWN_MASK as c_int;
    if osr as usize >= osr_adc_sel.len() {
        return ptr::null();
    }
    &osr_adc_sel[osr as usize] as *const _
}

unsafe fn nau8821_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let nau8821 = snd_soc_component_get_drvdata(component);
    let osr = nau8821_get_osr(nau8821, (*substream).stream);
    if osr.is_null() || (*osr).osr == 0 {
        return -EINVAL;
    }
    snd_pcm_hw_constraint_minmax((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 0, (CLK_DA_AD_MAX as c_uint) / (*osr).osr)
}

unsafe fn nau8821_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let nau8821 = snd_soc_component_get_drvdata(component);
    let mut val_len: c_uint = 0;
    let mut ctrl_val: c_int = 0;
    let bclk_fs: c_uint;
    let clk_div: c_uint;
    let osr: *const nau8821_osr_attr;

    (*nau8821).fs = params_rate(params);
    /* CLK_DAC or CLK_ADC = OSR * FS
     * DAC or ADC clock frequency is defined as Over Sampling Rate (OSR)
     * multiplied by the audio sample rate (Fs). Note that the OSR and Fs
     * values must be selected such that the maximum frequency is less
     * than 6.144 MHz.
     */
    osr = nau8821_get_osr(nau8821, (*substream).stream);
    if osr.is_null() || (*osr).osr == 0 {
        return -EINVAL;
    }
    if (*nau8821).fs * (*osr).osr as c_int > CLK_DA_AD_MAX {
        return -EINVAL;
    }
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits((*nau8821).regmap, NAU8821_R03_CLK_DIVIDER,
            NAU8821_CLK_DAC_SRC_MASK, (*osr).clk_src << NAU8821_CLK_DAC_SRC_SFT);
    } else {
        regmap_update_bits((*nau8821).regmap, NAU8821_R03_CLK_DIVIDER,
            NAU8821_CLK_ADC_SRC_MASK, (*osr).clk_src << NAU8821_CLK_ADC_SRC_SFT);
    }

    /* make BCLK and LRC divde configuration if the codec as master. */
    regmap_read((*nau8821).regmap, NAU8821_R1D_I2S_PCM_CTRL2, &mut ctrl_val);
    if (ctrl_val as c_uint & NAU8821_I2S_MS_MASTER) != 0 {
        /* get the bclk and fs ratio */
        bclk_fs = snd_soc_params_to_bclk(params) / (*nau8821).fs as c_uint;
        if bclk_fs <= 32 {
            clk_div = 3;
        } else if bclk_fs <= 64 {
            clk_div = 2;
        } else if bclk_fs <= 128 {
            clk_div = 1;
        } else {
            return -EINVAL;
        }
        regmap_update_bits((*nau8821).regmap, NAU8821_R1D_I2S_PCM_CTRL2,
            NAU8821_I2S_LRC_DIV_MASK | NAU8821_I2S_BLK_DIV_MASK,
            (clk_div << NAU8821_I2S_LRC_DIV_SFT) | clk_div);
    }

    match params_width(params) {
        16 => val_len |= NAU8821_I2S_DL_16,
        20 => val_len |= NAU8821_I2S_DL_20,
        24 => val_len |= NAU8821_I2S_DL_24,
        32 => val_len |= NAU8821_I2S_DL_32,
        _ => return -EINVAL,
    }
    regmap_update_bits((*nau8821).regmap, NAU8821_R1C_I2S_PCM_CTRL1, NAU8821_I2S_DL_MASK, val_len);
    0
}

unsafe fn nau8821_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let nau8821 = snd_soc_component_get_drvdata(component);
    let mut ctrl1_val: c_uint = 0;
    let mut ctrl2_val: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => ctrl2_val |= NAU8821_I2S_MS_MASTER,
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => ctrl1_val |= NAU8821_I2S_BP_INV,
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => ctrl1_val |= NAU8821_I2S_DF_I2S,
        SND_SOC_DAIFMT_LEFT_J => ctrl1_val |= NAU8821_I2S_DF_LEFT,
        SND_SOC_DAIFMT_RIGHT_J => ctrl1_val |= NAU8821_I2S_DF_RIGTH,
        SND_SOC_DAIFMT_DSP_A => ctrl1_val |= NAU8821_I2S_DF_PCM_AB,
        SND_SOC_DAIFMT_DSP_B => {
            ctrl1_val |= NAU8821_I2S_DF_PCM_AB;
            ctrl1_val |= NAU8821_I2S_PCMB_EN;
        }
        _ => return -EINVAL,
    }
    regmap_update_bits((*nau8821).regmap, NAU8821_R1C_I2S_PCM_CTRL1,
        NAU8821_I2S_DL_MASK | NAU8821_I2S_DF_MASK | NAU8821_I2S_BP_MASK | NAU8821_I2S_PCMB_MASK, ctrl1_val);
    regmap_update_bits((*nau8821).regmap, NAU8821_R1D_I2S_PCM_CTRL2, NAU8821_I2S_MS_MASK, ctrl2_val);
    0
}

unsafe fn nau8821_digital_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let nau8821 = snd_soc_component_get_drvdata(component);
    let mut val: c_uint = 0;
    if mute != 0 {
        val = NAU8821_DAC_SOFT_MUTE;
    }
    regmap_update_bits((*nau8821).regmap, NAU8821_R31_MUTE_CTRL, NAU8821_DAC_SOFT_MUTE, val)
}

const NAU8821_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const NAU8821_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE;

unsafe fn nau8821_is_jack_inserted(regmap: *mut regmap) -> bool {
    let mut status: c_int = 0;
    let mut jkdet: c_int = 0;
    regmap_read(regmap, NAU8821_R0D_JACK_DET_CTRL, &mut jkdet);
    let active_high = (jkdet as c_uint & NAU8821_JACK_POLARITY) != 0;
    regmap_read(regmap, NAU8821_R82_GENERAL_STATUS, &mut status);
    let is_high = (status as c_uint & NAU8821_GPIO2_IN) != 0;
    /* return jack connection status according to jack insertion logic
     * active high or active low.
     */
    active_high == is_high
}

unsafe fn nau8821_irq_status_clear(regmap: *mut regmap, mut active_irq: c_int) {
    if active_irq != 0 {
        regmap_write(regmap, NAU8821_R11_INT_CLR_KEY_STATUS, active_irq as c_uint);
        return;
    }
    /* Reset the interruption status from rightmost bit if the
     * corresponding irq event occurs.
     */
    regmap_read(regmap, NAU8821_R10_IRQ_STATUS, &mut active_irq);
    for i in 0..NAU8821_REG_DATA_LEN {
        let clear_irq = 0x1 << i;
        if (active_irq & clear_irq as c_int) != 0 {
            regmap_write(regmap, NAU8821_R11_INT_CLR_KEY_STATUS, clear_irq);
        }
    }
}

unsafe fn nau8821_eject_jack(nau8821: *mut nau8821) {
    let dapm = (*nau8821).dapm;
    let regmap = (*nau8821).regmap;
    /* Detach 2kOhm Resistors from MICBIAS to MICGND */
    regmap_update_bits(regmap, NAU8821_R74_MIC_BIAS, NAU8821_MICBIAS_JKR2, 0);
    /* HPL/HPR short to ground */
    regmap_update_bits(regmap, NAU8821_R0D_JACK_DET_CTRL, NAU8821_SPKR_DWN1R | NAU8821_SPKR_DWN1L, 0);
    snd_soc_dapm_disable_pin(dapm, b"MICBIAS\0".as_ptr() as *const c_char);
    snd_soc_dapm_sync(dapm);
    /* Disable & mask both insertion & ejection IRQs */
    regmap_update_bits(regmap, NAU8821_R12_INTERRUPT_DIS_CTRL,
        NAU8821_IRQ_INSERT_DIS | NAU8821_IRQ_EJECT_DIS, NAU8821_IRQ_INSERT_DIS | NAU8821_IRQ_EJECT_DIS);
    regmap_update_bits(regmap, NAU8821_R0F_INTERRUPT_MASK,
        NAU8821_IRQ_INSERT_EN | NAU8821_IRQ_EJECT_EN, NAU8821_IRQ_INSERT_EN | NAU8821_IRQ_EJECT_EN);
    /* Clear all interruption status */
    nau8821_irq_status_clear(regmap, 0);
    /* Enable & unmask the insertion IRQ */
    regmap_update_bits(regmap, NAU8821_R12_INTERRUPT_DIS_CTRL, NAU8821_IRQ_INSERT_DIS, 0);
    regmap_update_bits(regmap, NAU8821_R0F_INTERRUPT_MASK, NAU8821_IRQ_INSERT_EN, 0);
    /* Bypass de-bounce circuit */
    regmap_update_bits(regmap, NAU8821_R0D_JACK_DET_CTRL, NAU8821_JACK_DET_DB_BYPASS, NAU8821_JACK_DET_DB_BYPASS);
    /* Close clock for jack type detection at manual mode */
    if snd_soc_dapm_get_bias_level(dapm) < SND_SOC_BIAS_PREPARE {
        nau8821_configure_sysclk(nau8821, NAU8821_CLK_DIS, 0);
    }
    /* Recover to normal channel input */
    regmap_update_bits(regmap, NAU8821_R2B_ADC_RATE, NAU8821_ADC_R_SRC_EN, 0);
    if (*nau8821).key_enable {
        regmap_update_bits(regmap, NAU8821_R0F_INTERRUPT_MASK,
            NAU8821_IRQ_KEY_RELEASE_EN | NAU8821_IRQ_KEY_PRESS_EN,
            NAU8821_IRQ_KEY_RELEASE_EN | NAU8821_IRQ_KEY_PRESS_EN);
        regmap_update_bits(regmap, NAU8821_R12_INTERRUPT_DIS_CTRL,
            NAU8821_IRQ_KEY_RELEASE_DIS | NAU8821_IRQ_KEY_PRESS_DIS,
            NAU8821_IRQ_KEY_RELEASE_DIS | NAU8821_IRQ_KEY_PRESS_DIS);
    }
}

unsafe fn nau8821_jdet_work(work: *mut work_struct) {
    let nau8821 = container_of!(work, nau8821, jdet_work.work);
    let dapm = (*nau8821).dapm;
    let regmap = (*nau8821).regmap;
    let mut jack_status_reg: c_int = 0;
    let mut event: c_int = 0;
    let mut event_mask: c_int = 0;

    regmap_read(regmap, NAU8821_R58_I2C_DEVICE_ID, &mut jack_status_reg);
    let mic_detected = (jack_status_reg as c_uint & NAU8821_KEYDET) == 0;
    if mic_detected {
        dev_dbg((*nau8821).dev, b"Headset connected\n\0".as_ptr() as *const c_char);
        event |= SND_JACK_HEADSET;
        /* 2kOhm Resistor from MICBIAS to MICGND1 */
        regmap_update_bits(regmap, NAU8821_R74_MIC_BIAS, NAU8821_MICBIAS_JKR2, NAU8821_MICBIAS_JKR2);
        /* Latch Right Channel Analog data
         * input into the Right Channel Filter
         */
        regmap_update_bits(regmap, NAU8821_R2B_ADC_RATE, NAU8821_ADC_R_SRC_EN, NAU8821_ADC_R_SRC_EN);
        if (*nau8821).key_enable {
            regmap_update_bits(regmap, NAU8821_R0F_INTERRUPT_MASK, NAU8821_IRQ_KEY_RELEASE_EN | NAU8821_IRQ_KEY_PRESS_EN, 0);
            regmap_update_bits(regmap, NAU8821_R12_INTERRUPT_DIS_CTRL, NAU8821_IRQ_KEY_RELEASE_DIS | NAU8821_IRQ_KEY_PRESS_DIS, 0);
        } else {
            snd_soc_dapm_disable_pin(dapm, b"MICBIAS\0".as_ptr() as *const c_char);
            snd_soc_dapm_sync(dapm);
        }
    } else {
        dev_dbg((*nau8821).dev, b"Headphone connected\n\0".as_ptr() as *const c_char);
        event |= SND_JACK_HEADPHONE;
        snd_soc_dapm_disable_pin(dapm, b"MICBIAS\0".as_ptr() as *const c_char);
        snd_soc_dapm_sync(dapm);
    }
    event_mask |= SND_JACK_HEADSET;
    snd_soc_jack_report((*nau8821).jack, event, event_mask);
}

/* Enable interruptions with internal clock. */
unsafe fn nau8821_setup_inserted_irq(nau8821: *mut nau8821) {
    let regmap = (*nau8821).regmap;
    /* Disable & mask insertion IRQ */
    regmap_update_bits(regmap, NAU8821_R12_INTERRUPT_DIS_CTRL, NAU8821_IRQ_INSERT_DIS, NAU8821_IRQ_INSERT_DIS);
    regmap_update_bits(regmap, NAU8821_R0F_INTERRUPT_MASK, NAU8821_IRQ_INSERT_EN, NAU8821_IRQ_INSERT_EN);
    /* Clear insert IRQ status */
    nau8821_irq_status_clear(regmap, NAU8821_JACK_INSERT_DETECTED as c_int);
    /* Enable internal VCO needed for interruptions */
    if snd_soc_dapm_get_bias_level((*nau8821).dapm) < SND_SOC_BIAS_PREPARE {
        nau8821_configure_sysclk(nau8821, NAU8821_CLK_INTERNAL, 0);
    }
    /* Chip needs one FSCLK cycle in order to generate interruptions,
     * as we cannot guarantee one will be provided by the system. Turning
     * master mode on then off enables us to generate that FSCLK cycle
     * with a minimum of contention on the clock bus.
     */
    regmap_update_bits(regmap, NAU8821_R1D_I2S_PCM_CTRL2, NAU8821_I2S_MS_MASK, NAU8821_I2S_MS_MASTER);
    regmap_update_bits(regmap, NAU8821_R1D_I2S_PCM_CTRL2, NAU8821_I2S_MS_MASK, NAU8821_I2S_MS_SLAVE);
    /* Do not bypass de-bounce circuit */
    if (nau8821_quirk & NAU8821_QUIRK_JD_DB_BYPASS) == 0 {
        regmap_update_bits(regmap, NAU8821_R0D_JACK_DET_CTRL, NAU8821_JACK_DET_DB_BYPASS, 0);
    }
    /* Unmask & enable the ejection IRQs */
    regmap_update_bits(regmap, NAU8821_R0F_INTERRUPT_MASK, NAU8821_IRQ_EJECT_EN, 0);
    regmap_update_bits(regmap, NAU8821_R12_INTERRUPT_DIS_CTRL, NAU8821_IRQ_EJECT_DIS, 0);
}

unsafe extern "C" fn nau8821_interrupt(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let nau8821 = data as *mut nau8821;
    let regmap = (*nau8821).regmap;
    let mut active_irq: c_int = 0;
    let mut event: c_int = 0;
    let mut event_mask: c_int = 0;

    if regmap_read(regmap, NAU8821_R10_IRQ_STATUS, &mut active_irq) != 0 {
        dev_err((*nau8821).dev, b"failed to read irq status\n\0".as_ptr() as *const c_char);
        return IRQ_NONE;
    }
    dev_dbg((*nau8821).dev, b"IRQ %d\n\0".as_ptr() as *const c_char, active_irq);
    if ((active_irq as c_uint & NAU8821_JACK_EJECT_IRQ_MASK) == NAU8821_JACK_EJECT_DETECTED) {
        cancel_delayed_work_sync(&mut (*nau8821).jdet_work);
        regmap_update_bits(regmap, NAU8821_R71_ANALOG_ADC_1, NAU8821_MICDET_MASK, NAU8821_MICDET_DIS);
        nau8821_eject_jack(nau8821);
        event_mask |= SND_JACK_HEADSET;
    } else if (active_irq as c_uint & NAU8821_KEY_SHORT_PRESS_IRQ) != 0 {
        event |= NAU8821_BUTTON;
        event_mask |= NAU8821_BUTTON;
        nau8821_irq_status_clear(regmap, NAU8821_KEY_SHORT_PRESS_IRQ as c_int);
    } else if (active_irq as c_uint & NAU8821_KEY_RELEASE_IRQ) != 0 {
        event_mask = NAU8821_BUTTON;
        nau8821_irq_status_clear(regmap, NAU8821_KEY_RELEASE_IRQ as c_int);
    } else if ((active_irq as c_uint & NAU8821_JACK_INSERT_IRQ_MASK) == NAU8821_JACK_INSERT_DETECTED) {
        cancel_delayed_work_sync(&mut (*nau8821).jdet_work);
        regmap_update_bits(regmap, NAU8821_R71_ANALOG_ADC_1, NAU8821_MICDET_MASK, NAU8821_MICDET_EN);
        if nau8821_is_jack_inserted(regmap) {
            /* Detect microphone and jack type */
            snd_soc_dapm_force_enable_pin((*nau8821).dapm, b"MICBIAS\0".as_ptr() as *const c_char);
            snd_soc_dapm_sync((*nau8821).dapm);
            schedule_delayed_work(&mut (*nau8821).jdet_work, msecs_to_jiffies(20));
            /* Turn off insertion interruption at manual mode */
            nau8821_setup_inserted_irq(nau8821);
        } else {
            dev_warn((*nau8821).dev, b"Inserted IRQ fired but not connected\n\0".as_ptr() as *const c_char);
            nau8821_eject_jack(nau8821);
        }
    } else {
        /* Clear the rightmost interrupt */
        nau8821_irq_status_clear(regmap, active_irq);
    }
    if event_mask != 0 {
        snd_soc_jack_report((*nau8821).jack, event, event_mask);
    }
    IRQ_HANDLED
}

unsafe fn nau8821_component_probe(component: *mut snd_soc_component) -> c_int {
    let nau8821 = snd_soc_component_get_drvdata(component);
    let dapm = snd_soc_component_to_dapm(component);
    (*nau8821).dapm = dapm;
    0
}

unsafe fn nau8821_component_remove(component: *mut snd_soc_component) {
    let nau8821 = snd_soc_component_get_drvdata(component);
    if (*nau8821).jdet_active {
        cancel_delayed_work_sync(&mut (*nau8821).jdet_work);
    }
}

/**
 * nau8821_calc_fll_param - Calculate FLL parameters.
 * @fll_in: external clock provided to codec.
 * @fs: sampling rate.
 * @fll_param: Pointer to structure of FLL parameters.
 *
 * Calculate FLL parameters to configure codec.
 *
 * Returns 0 for success or negative error code.
 */
unsafe fn nau8821_calc_fll_param(fll_in: c_uint, fs: c_uint, fll_param: *mut nau8821_fll) -> c_int {
    let mut fref: c_uint = 0;
    let mut fvco_sel: usize;

    /* Ensure the reference clock frequency (FREF) is <= 13.5MHz by
     * dividing freq_in by 1, 2, 4, or 8 using FLL pre-scalar.
     * FREF = freq_in / NAU8821_FLL_REF_DIV_MASK
     */
    let mut i = 0usize;
    while i < fll_pre_scalar.len() {
        fref = fll_in >> fll_pre_scalar[i].param;
        if fref <= NAU_FREF_MAX {
            break;
        }
        i += 1;
    }
    if i == fll_pre_scalar.len() {
        return -EINVAL;
    }
    (*fll_param).clk_ref_div = fll_pre_scalar[i].val as c_int;

    /* Choose the FLL ratio based on FREF */
    i = 0;
    while i < fll_ratio.len() {
        if fref >= fll_ratio[i].param {
            break;
        }
        i += 1;
    }
    if i == fll_ratio.len() {
        return -EINVAL;
    }
    (*fll_param).ratio = fll_ratio[i].val as c_int;

    /* Calculate the frequency of DCO (FDCO) given freq_out = 256 * Fs.
     * FDCO must be within the 90MHz - 100MHz or the FFL cannot be
     * guaranteed across the full range of operation.
     * FDCO = freq_out * 2 * mclk_src_scaling
     */
    let mut fvco_max: u64 = 0;
    fvco_sel = mclk_src_scaling.len();
    i = 0;
    while i < mclk_src_scaling.len() {
        let fvco = 256u64 * fs as u64 * 2 * mclk_src_scaling[i].param as u64;
        if fvco > NAU_FVCO_MIN && fvco < NAU_FVCO_MAX && fvco_max < fvco {
            fvco_max = fvco;
            fvco_sel = i;
        }
        i += 1;
    }
    if mclk_src_scaling.len() == fvco_sel {
        return -EINVAL;
    }
    (*fll_param).mclk_src = mclk_src_scaling[fvco_sel].val as c_int;

    /* Calculate the FLL 10-bit integer input and the FLL 24-bit fractional
     * input based on FDCO, FREF and FLL ratio.
     */
    let fvco = div_u64(fvco_max << 24, fref as u64 * (*fll_param).ratio as u64);
    (*fll_param).fll_int = ((fvco >> 24) & 0x3ff) as c_int;
    (*fll_param).fll_frac = (fvco & 0xffffff) as c_int;
    0
}

unsafe fn nau8821_fll_apply(nau8821: *mut nau8821, fll_param: *mut nau8821_fll) {
    let regmap = (*nau8821).regmap;
    regmap_update_bits(regmap, NAU8821_R03_CLK_DIVIDER,
        NAU8821_CLK_SRC_MASK | NAU8821_CLK_MCLK_SRC_MASK,
        NAU8821_CLK_SRC_MCLK | (*fll_param).mclk_src as c_uint);
    /* Make DSP operate at high speed for better performance. */
    regmap_update_bits(regmap, NAU8821_R04_FLL1,
        NAU8821_FLL_RATIO_MASK | NAU8821_ICTRL_LATCH_MASK,
        (*fll_param).ratio as c_uint | (0x6 << NAU8821_ICTRL_LATCH_SFT));
    /* FLL 24-bit fractional input */
    regmap_write(regmap, NAU8821_R0A_FLL7, (((*fll_param).fll_frac >> 16) & 0xff) as c_uint);
    regmap_write(regmap, NAU8821_R0B_FLL8, ((*fll_param).fll_frac & 0xffff) as c_uint);
    /* FLL 10-bit integer input */
    regmap_update_bits(regmap, NAU8821_R06_FLL3, NAU8821_FLL_INTEGER_MASK, (*fll_param).fll_int as c_uint);
    /* FLL pre-scaler */
    regmap_update_bits(regmap, NAU8821_R07_FLL4,
        NAU8821_HIGHBW_EN | NAU8821_FLL_REF_DIV_MASK,
        NAU8821_HIGHBW_EN | ((*fll_param).clk_ref_div as c_uint << NAU8821_FLL_REF_DIV_SFT));
    /* select divided VCO input */
    regmap_update_bits(regmap, NAU8821_R08_FLL5, NAU8821_FLL_CLK_SW_MASK, NAU8821_FLL_CLK_SW_REF);
    /* Disable free-running mode */
    regmap_update_bits(regmap, NAU8821_R09_FLL6, NAU8821_DCO_EN, 0);
    if (*fll_param).fll_frac != 0 {
        /* set FLL loop filter enable and cutoff frequency at 500Khz */
        regmap_update_bits(regmap, NAU8821_R08_FLL5,
            NAU8821_FLL_PDB_DAC_EN | NAU8821_FLL_LOOP_FTR_EN | NAU8821_FLL_FTR_SW_MASK,
            NAU8821_FLL_PDB_DAC_EN | NAU8821_FLL_LOOP_FTR_EN | NAU8821_FLL_FTR_SW_FILTER);
        regmap_update_bits(regmap, NAU8821_R09_FLL6, NAU8821_SDM_EN | NAU8821_CUTOFF500, NAU8821_SDM_EN | NAU8821_CUTOFF500);
    } else {
        /* disable FLL loop filter and cutoff frequency */
        regmap_update_bits(regmap, NAU8821_R08_FLL5,
            NAU8821_FLL_PDB_DAC_EN | NAU8821_FLL_LOOP_FTR_EN | NAU8821_FLL_FTR_SW_MASK, NAU8821_FLL_FTR_SW_ACCU);
        regmap_update_bits(regmap, NAU8821_R09_FLL6, NAU8821_SDM_EN | NAU8821_CUTOFF500, 0);
    }
}

/**
 * nau8821_set_fll - FLL configuration of nau8821
 * @component:  codec component
 * @pll_id:  PLL requested
 * @source:  clock source
 * @freq_in:  frequency of input clock source
 * @freq_out:  must be 256*Fs in order to achieve the best performance
 *
 * The FLL function can select BCLK or MCLK as the input clock source.
 *
 * Returns 0 if the parameters have been applied successfully
 * or negative error code.
 */
unsafe fn nau8821_set_fll(component: *mut snd_soc_component, _pll_id: c_int, _source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int {
    let nau8821 = snd_soc_component_get_drvdata(component);
    let mut fll_set_param = nau8821_fll { mclk_src: 0, ratio: 0, fll_frac: 0, fll_int: 0, clk_ref_div: 0 };
    let fll_param = &mut fll_set_param as *mut nau8821_fll;
    let fs = freq_out >> 8;
    let ret = nau8821_calc_fll_param(freq_in, fs, fll_param);
    if ret != 0 {
        dev_err((*nau8821).dev, b"Unsupported input clock %d to output clock %d\n\0".as_ptr() as *const c_char, freq_in, freq_out);
        return ret;
    }
    dev_dbg((*nau8821).dev, b"mclk_src=%x ratio=%x fll_frac=%x fll_int=%x clk_ref_div=%x\n\0".as_ptr() as *const c_char,
        (*fll_param).mclk_src, (*fll_param).ratio, (*fll_param).fll_frac, (*fll_param).fll_int, (*fll_param).clk_ref_div);
    nau8821_fll_apply(nau8821, fll_param);
    mdelay(2);
    regmap_update_bits((*nau8821).regmap, NAU8821_R03_CLK_DIVIDER, NAU8821_CLK_SRC_MASK, NAU8821_CLK_SRC_VCO);
    0
}

unsafe fn nau8821_configure_mclk_as_sysclk(regmap: *mut regmap) {
    regmap_update_bits(regmap, NAU8821_R03_CLK_DIVIDER, NAU8821_CLK_SRC_MASK, NAU8821_CLK_SRC_MCLK);
    regmap_update_bits(regmap, NAU8821_R09_FLL6, NAU8821_DCO_EN, 0);
    /* Make DSP operate as default setting for power saving. */
    regmap_update_bits(regmap, NAU8821_R04_FLL1, NAU8821_ICTRL_LATCH_MASK, 0);
}

unsafe fn nau8821_configure_sysclk(nau8821: *mut nau8821, clk_id: c_int, freq: c_uint) -> c_int {
    let regmap = (*nau8821).regmap;
    match clk_id {
        NAU8821_CLK_DIS => {
            /* Clock provided externally and disable internal VCO clock */
            nau8821_configure_mclk_as_sysclk(regmap);
        }
        NAU8821_CLK_MCLK => {
            nau8821_configure_mclk_as_sysclk(regmap);
            /* MCLK not changed by clock tree */
            regmap_update_bits(regmap, NAU8821_R03_CLK_DIVIDER, NAU8821_CLK_MCLK_SRC_MASK, 0);
        }
        NAU8821_CLK_INTERNAL => {
            if nau8821_is_jack_inserted(regmap) {
                regmap_update_bits(regmap, NAU8821_R09_FLL6, NAU8821_DCO_EN, NAU8821_DCO_EN);
                regmap_update_bits(regmap, NAU8821_R03_CLK_DIVIDER, NAU8821_CLK_SRC_MASK, NAU8821_CLK_SRC_VCO);
                /* Decrease the VCO frequency and make DSP operate
                 * as default setting for power saving.
                 */
                regmap_update_bits(regmap, NAU8821_R03_CLK_DIVIDER, NAU8821_CLK_MCLK_SRC_MASK, 0xf);
                regmap_update_bits(regmap, NAU8821_R04_FLL1, NAU8821_ICTRL_LATCH_MASK | NAU8821_FLL_RATIO_MASK, 0x10);
                regmap_update_bits(regmap, NAU8821_R09_FLL6, NAU8821_SDM_EN, NAU8821_SDM_EN);
            }
        }
        NAU8821_CLK_FLL_MCLK => {
            /* Higher FLL reference input frequency can only set lower
             * gain error, such as 0000 for input reference from MCLK
             * 12.288Mhz.
             */
            regmap_update_bits(regmap, NAU8821_R06_FLL3, NAU8821_FLL_CLK_SRC_MASK | NAU8821_GAIN_ERR_MASK, NAU8821_FLL_CLK_SRC_MCLK | 0);
        }
        NAU8821_CLK_FLL_BLK => {
            /* If FLL reference input is from low frequency source,
             * higher error gain can apply such as 0xf which has
             * the most sensitive gain error correction threshold,
             * Therefore, FLL has the most accurate DCO to
             * target frequency.
             */
            regmap_update_bits(regmap, NAU8821_R06_FLL3, NAU8821_FLL_CLK_SRC_MASK | NAU8821_GAIN_ERR_MASK,
                NAU8821_FLL_CLK_SRC_BLK | (0xf << NAU8821_GAIN_ERR_SFT));
        }
        NAU8821_CLK_FLL_FS => {
            /* If FLL reference input is from low frequency source,
             * higher error gain can apply such as 0xf which has
             * the most sensitive gain error correction threshold,
             * Therefore, FLL has the most accurate DCO to
             * target frequency.
             */
            regmap_update_bits(regmap, NAU8821_R06_FLL3, NAU8821_FLL_CLK_SRC_MASK | NAU8821_GAIN_ERR_MASK,
                NAU8821_FLL_CLK_SRC_FS | (0xf << NAU8821_GAIN_ERR_SFT));
        }
        _ => {
            dev_err((*nau8821).dev, b"Invalid clock id (%d)\n\0".as_ptr() as *const c_char, clk_id);
            return -EINVAL;
        }
    }
    (*nau8821).clk_id = clk_id;
    dev_dbg((*nau8821).dev, b"Sysclk is %dHz and clock id is %d\n\0".as_ptr() as *const c_char, freq, (*nau8821).clk_id);
    0
}

unsafe fn nau8821_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, _source: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let nau8821 = snd_soc_component_get_drvdata(component);
    nau8821_configure_sysclk(nau8821, clk_id, freq)
}

unsafe fn nau8821_resume_setup(nau8821: *mut nau8821) -> c_int {
    let regmap = (*nau8821).regmap;
    /* Close clock when jack type detection at manual mode */
    nau8821_configure_sysclk(nau8821, NAU8821_CLK_DIS, 0);
    if (*nau8821).irq != 0 {
        /* Clear all interruption status */
        nau8821_irq_status_clear(regmap, 0);
        /* Enable both insertion and ejection interruptions, and then
         * bypass de-bounce circuit.
         */
        regmap_update_bits(regmap, NAU8821_R0F_INTERRUPT_MASK, NAU8821_IRQ_EJECT_EN | NAU8821_IRQ_INSERT_EN, 0);
        regmap_update_bits(regmap, NAU8821_R0D_JACK_DET_CTRL, NAU8821_JACK_DET_DB_BYPASS, NAU8821_JACK_DET_DB_BYPASS);
        regmap_update_bits(regmap, NAU8821_R12_INTERRUPT_DIS_CTRL, NAU8821_IRQ_INSERT_DIS | NAU8821_IRQ_EJECT_DIS, 0);
    }
    0
}

unsafe fn nau8821_set_bias_level(component: *mut snd_soc_component, level: c_int) -> c_int {
    let nau8821 = snd_soc_component_get_drvdata(component);
    let regmap = (*nau8821).regmap;
    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            /* Setup codec configuration after resume */
            if snd_soc_dapm_get_bias_level((*nau8821).dapm) == SND_SOC_BIAS_OFF {
                nau8821_resume_setup(nau8821);
            }
        }
        SND_SOC_BIAS_OFF => {
            /* HPL/HPR short to ground */
            regmap_update_bits(regmap, NAU8821_R0D_JACK_DET_CTRL, NAU8821_SPKR_DWN1R | NAU8821_SPKR_DWN1L, 0);
            if (*nau8821).irq != 0 {
                /* Reset the configuration of jack type for detection.
                 * Detach 2kOhm Resistors from MICBIAS to MICGND1/2.
                 */
                regmap_update_bits(regmap, NAU8821_R74_MIC_BIAS, NAU8821_MICBIAS_JKR2, 0);
                /* Turn off all interruptions before system shutdown.
                 * Keep theinterruption quiet before resume
                 * setup completes.
                 */
                regmap_write(regmap, NAU8821_R12_INTERRUPT_DIS_CTRL, 0xffff);
                regmap_update_bits(regmap, NAU8821_R0F_INTERRUPT_MASK,
                    NAU8821_IRQ_EJECT_EN | NAU8821_IRQ_INSERT_EN, NAU8821_IRQ_EJECT_EN | NAU8821_IRQ_INSERT_EN);
            }
        }
        _ => {}
    }
    0
}

unsafe fn nau8821_suspend(component: *mut snd_soc_component) -> c_int {
    let nau8821 = snd_soc_component_get_drvdata(component);
    if (*nau8821).irq != 0 {
        disable_irq((*nau8821).irq);
    }
    if (*nau8821).jdet_active {
        cancel_delayed_work_sync(&mut (*nau8821).jdet_work);
    }
    snd_soc_dapm_force_bias_level((*nau8821).dapm, SND_SOC_BIAS_OFF);
    /* Power down codec power; don't support button wakeup */
    snd_soc_dapm_disable_pin((*nau8821).dapm, b"MICBIAS\0".as_ptr() as *const c_char);
    snd_soc_dapm_sync((*nau8821).dapm);
    regcache_cache_only((*nau8821).regmap, true);
    regcache_mark_dirty((*nau8821).regmap);
    0
}

unsafe fn nau8821_resume(component: *mut snd_soc_component) -> c_int {
    let nau8821 = snd_soc_component_get_drvdata(component);
    regcache_cache_only((*nau8821).regmap, false);
    regcache_sync((*nau8821).regmap);
    if (*nau8821).irq != 0 {
        enable_irq((*nau8821).irq);
    }
    0
}

/**
 * nau8821_enable_jack_detect - Specify a jack for event reporting
 *
 * @component:  component to register the jack with
 * @jack: jack to use to report headset and button events on
 *
 * After this function has been called the headset insert/remove and button
 * events will be routed to the given jack.  Jack can be null to stop
 * reporting.
 */
#[no_mangle]
pub unsafe extern "C" fn nau8821_enable_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack) -> c_int {
    let nau8821 = snd_soc_component_get_drvdata(component);
    (*nau8821).jack = jack;
    if (*nau8821).jdet_active {
        return 0;
    }
    /* Initiate jack detection work queue */
    INIT_DELAYED_WORK!(&mut (*nau8821).jdet_work, nau8821_jdet_work);
    (*nau8821).jdet_active = true;
    let ret = devm_request_threaded_irq((*nau8821).dev, (*nau8821).irq, ptr::null(),
        nau8821_interrupt, IRQF_TRIGGER_LOW | IRQF_ONESHOT, b"nau8821\0".as_ptr() as *const c_char, nau8821 as *mut c_void);
    if ret != 0 {
        dev_err((*nau8821).dev, b"Cannot request irq %d (%d)\n\0".as_ptr() as *const c_char, (*nau8821).irq, ret);
    }
    ret
}
// EXPORT_SYMBOL_GPL(nau8821_enable_jack_detect);

unsafe fn nau8821_reset_chip(regmap: *mut regmap) {
    regmap_write(regmap, NAU8821_R00_RESET, 0xffff);
    regmap_write(regmap, NAU8821_R00_RESET, 0xffff);
}

unsafe fn nau8821_print_device_properties(nau8821: *mut nau8821) {
    let dev = (*nau8821).dev;
    dev_dbg(dev, b"jkdet-enable:         %d\n\0".as_ptr() as *const c_char, (*nau8821).jkdet_enable as c_int);
    dev_dbg(dev, b"jkdet-pull-enable:    %d\n\0".as_ptr() as *const c_char, (*nau8821).jkdet_pull_enable as c_int);
    dev_dbg(dev, b"jkdet-pull-up:        %d\n\0".as_ptr() as *const c_char, (*nau8821).jkdet_pull_up as c_int);
    dev_dbg(dev, b"jkdet-polarity:       %d\n\0".as_ptr() as *const c_char, (*nau8821).jkdet_polarity);
    dev_dbg(dev, b"micbias-voltage:      %d\n\0".as_ptr() as *const c_char, (*nau8821).micbias_voltage);
    dev_dbg(dev, b"vref-impedance:       %d\n\0".as_ptr() as *const c_char, (*nau8821).vref_impedance);
    dev_dbg(dev, b"jack-insert-debounce: %d\n\0".as_ptr() as *const c_char, (*nau8821).jack_insert_debounce);
    dev_dbg(dev, b"jack-eject-debounce:  %d\n\0".as_ptr() as *const c_char, (*nau8821).jack_eject_debounce);
    dev_dbg(dev, b"dmic-clk-threshold:       %d\n\0".as_ptr() as *const c_char, (*nau8821).dmic_clk_threshold);
    dev_dbg(dev, b"key_enable:       %d\n\0".as_ptr() as *const c_char, (*nau8821).key_enable as c_int);
    dev_dbg(dev, b"adc-delay-ms:\t\t%d\n\0".as_ptr() as *const c_char, (*nau8821).adc_delay);
}

unsafe fn nau8821_read_device_properties(dev: *mut device, nau8821: *mut nau8821) -> c_int {
    let mut ret: c_int;
    (*nau8821).jkdet_enable = device_property_read_bool(dev, b"nuvoton,jkdet-enable\0".as_ptr() as *const c_char);
    (*nau8821).jkdet_pull_enable = device_property_read_bool(dev, b"nuvoton,jkdet-pull-enable\0".as_ptr() as *const c_char);
    (*nau8821).jkdet_pull_up = device_property_read_bool(dev, b"nuvoton,jkdet-pull-up\0".as_ptr() as *const c_char);
    (*nau8821).key_enable = device_property_read_bool(dev, b"nuvoton,key-enable\0".as_ptr() as *const c_char);
    (*nau8821).left_input_single_end = device_property_read_bool(dev, b"nuvoton,left-input-single-end\0".as_ptr() as *const c_char);
    ret = device_property_read_u32(dev, b"nuvoton,jkdet-polarity\0".as_ptr() as *const c_char, &mut (*nau8821).jkdet_polarity);
    if ret != 0 { (*nau8821).jkdet_polarity = 1; }
    ret = device_property_read_u32(dev, b"nuvoton,micbias-voltage\0".as_ptr() as *const c_char, &mut (*nau8821).micbias_voltage);
    if ret != 0 { (*nau8821).micbias_voltage = 6; }
    ret = device_property_read_u32(dev, b"nuvoton,vref-impedance\0".as_ptr() as *const c_char, &mut (*nau8821).vref_impedance);
    if ret != 0 { (*nau8821).vref_impedance = 2; }
    ret = device_property_read_u32(dev, b"nuvoton,jack-insert-debounce\0".as_ptr() as *const c_char, &mut (*nau8821).jack_insert_debounce);
    if ret != 0 { (*nau8821).jack_insert_debounce = 7; }
    ret = device_property_read_u32(dev, b"nuvoton,jack-eject-debounce\0".as_ptr() as *const c_char, &mut (*nau8821).jack_eject_debounce);
    if ret != 0 { (*nau8821).jack_eject_debounce = 0; }
    ret = device_property_read_u32(dev, b"nuvoton,dmic-clk-threshold\0".as_ptr() as *const c_char, &mut (*nau8821).dmic_clk_threshold);
    if ret != 0 { (*nau8821).dmic_clk_threshold = 3072000; }
    ret = device_property_read_u32(dev, b"nuvoton,dmic-slew-rate\0".as_ptr() as *const c_char, &mut (*nau8821).dmic_slew_rate);
    if ret != 0 { (*nau8821).dmic_slew_rate = 0; }
    ret = device_property_read_u32(dev, b"nuvoton,adc-delay-ms\0".as_ptr() as *const c_char, &mut (*nau8821).adc_delay);
    if ret != 0 { (*nau8821).adc_delay = 125; }
    if (*nau8821).adc_delay < 125 || (*nau8821).adc_delay > 500 {
        dev_warn(dev, b"Please set the suitable delay time!\n\0".as_ptr() as *const c_char);
    }
    0
}

unsafe fn nau8821_init_regs(nau8821: *mut nau8821) {
    let regmap = (*nau8821).regmap;
    /* Enable Bias/Vmid */
    regmap_update_bits(regmap, NAU8821_R66_BIAS_ADJ, NAU8821_BIAS_VMID, NAU8821_BIAS_VMID);
    regmap_update_bits(regmap, NAU8821_R76_BOOST, NAU8821_GLOBAL_BIAS_EN, NAU8821_GLOBAL_BIAS_EN);
    /* VMID Tieoff setting and enable TESTDAC.
     * This sets the analog DAC inputs to a '0' input signal to avoid
     * any glitches due to power up transients in both the analog and
     * digital DAC circuit.
     */
    regmap_update_bits(regmap, NAU8821_R66_BIAS_ADJ, NAU8821_BIAS_VMID_SEL_MASK | NAU8821_BIAS_TESTDAC_EN,
        ((*nau8821).vref_impedance << NAU8821_BIAS_VMID_SEL_SFT) | NAU8821_BIAS_TESTDAC_EN);
    /* Disable short Frame Sync detection logic */
    regmap_update_bits(regmap, NAU8821_R1E_LEFT_TIME_SLOT, NAU8821_DIS_FS_SHORT_DET, NAU8821_DIS_FS_SHORT_DET);
    /* Disable Boost Driver, Automatic Short circuit protection enable */
    regmap_update_bits(regmap, NAU8821_R76_BOOST,
        NAU8821_PRECHARGE_DIS | NAU8821_HP_BOOST_DIS | NAU8821_HP_BOOST_G_DIS | NAU8821_SHORT_SHUTDOWN_EN,
        NAU8821_PRECHARGE_DIS | NAU8821_HP_BOOST_DIS | NAU8821_HP_BOOST_G_DIS | NAU8821_SHORT_SHUTDOWN_EN);
    /* Class G timer 64ms */
    regmap_update_bits(regmap, NAU8821_R4B_CLASSG_CTRL, NAU8821_CLASSG_TIMER_MASK, 0x20 << NAU8821_CLASSG_TIMER_SFT);
    /* Class AB bias current to 2x, DAC Capacitor enable MSB/LSB */
    regmap_update_bits(regmap, NAU8821_R6A_ANALOG_CONTROL_2,
        NAU8821_HP_NON_CLASSG_CURRENT_2xADJ | NAU8821_DAC_CAPACITOR_MSB | NAU8821_DAC_CAPACITOR_LSB,
        NAU8821_HP_NON_CLASSG_CURRENT_2xADJ | NAU8821_DAC_CAPACITOR_MSB | NAU8821_DAC_CAPACITOR_LSB);
    /* Disable DACR/L power */
    regmap_update_bits(regmap, NAU8821_R80_CHARGE_PUMP, NAU8821_POWER_DOWN_DACR | NAU8821_POWER_DOWN_DACL, 0);
    /* DAC clock delay 2ns, VREF */
    regmap_update_bits(regmap, NAU8821_R73_RDAC, NAU8821_DAC_CLK_DELAY_MASK | NAU8821_DAC_VREF_MASK,
        (0x2 << NAU8821_DAC_CLK_DELAY_SFT) | (0x3 << NAU8821_DAC_VREF_SFT));
    regmap_update_bits(regmap, NAU8821_R74_MIC_BIAS, NAU8821_MICBIAS_VOLTAGE_MASK, (*nau8821).micbias_voltage);
    /* Default oversampling/decimations settings are unusable
     * (audible hiss). Set it to something better.
     */
    regmap_update_bits(regmap, NAU8821_R2B_ADC_RATE, NAU8821_ADC_SYNC_DOWN_MASK, NAU8821_ADC_SYNC_DOWN_64);
    regmap_update_bits(regmap, NAU8821_R2C_DAC_CTRL1, NAU8821_DAC_OVERSAMPLE_MASK, NAU8821_DAC_OVERSAMPLE_64);
    regmap_update_bits(regmap, NAU8821_R13_DMIC_CTRL, NAU8821_DMIC_SLEW_MASK, (*nau8821).dmic_slew_rate << NAU8821_DMIC_SLEW_SFT);
    if (*nau8821).left_input_single_end {
        regmap_update_bits(regmap, NAU8821_R6B_PGA_MUTE, NAU8821_MUTE_MICNL_EN, NAU8821_MUTE_MICNL_EN);
        regmap_update_bits(regmap, NAU8821_R74_MIC_BIAS, NAU8821_MICBIAS_LOWNOISE_EN, NAU8821_MICBIAS_LOWNOISE_EN);
    }
}

unsafe fn nau8821_setup_irq(nau8821: *mut nau8821) -> c_int {
    let regmap = (*nau8821).regmap;
    /* Jack detection */
    regmap_update_bits(regmap, NAU8821_R1A_GPIO12_CTRL, NAU8821_JKDET_OUTPUT_EN,
        if (*nau8821).jkdet_enable { 0 } else { NAU8821_JKDET_OUTPUT_EN });
    regmap_update_bits(regmap, NAU8821_R1A_GPIO12_CTRL, NAU8821_JKDET_PULL_EN,
        if (*nau8821).jkdet_pull_enable { 0 } else { NAU8821_JKDET_PULL_EN });
    regmap_update_bits(regmap, NAU8821_R1A_GPIO12_CTRL, NAU8821_JKDET_PULL_UP,
        if (*nau8821).jkdet_pull_up { NAU8821_JKDET_PULL_UP } else { 0 });
    regmap_update_bits(regmap, NAU8821_R0D_JACK_DET_CTRL, NAU8821_JACK_POLARITY,
        /* jkdet_polarity - 1  is for active-low */
        if (*nau8821).jkdet_polarity != 0 { 0 } else { NAU8821_JACK_POLARITY });
    regmap_update_bits(regmap, NAU8821_R0D_JACK_DET_CTRL, NAU8821_JACK_INSERT_DEBOUNCE_MASK,
        (*nau8821).jack_insert_debounce << NAU8821_JACK_INSERT_DEBOUNCE_SFT);
    regmap_update_bits(regmap, NAU8821_R0D_JACK_DET_CTRL, NAU8821_JACK_EJECT_DEBOUNCE_MASK,
        (*nau8821).jack_eject_debounce << NAU8821_JACK_EJECT_DEBOUNCE_SFT);
    /* Pull up IRQ pin */
    regmap_update_bits(regmap, NAU8821_R0F_INTERRUPT_MASK,
        NAU8821_IRQ_PIN_PULL_UP | NAU8821_IRQ_PIN_PULL_EN | NAU8821_IRQ_OUTPUT_EN,
        NAU8821_IRQ_PIN_PULL_UP | NAU8821_IRQ_PIN_PULL_EN | NAU8821_IRQ_OUTPUT_EN);
    /* Disable interruption before codec initiation done */
    /* Mask unneeded IRQs: 1 - disable, 0 - enable */
    regmap_update_bits(regmap, NAU8821_R0F_INTERRUPT_MASK, 0x3f5, 0x3f5);
    0
}

/* Please keep this list alphabetically sorted */
// DMI quirk entries are preserved in the external_asoc_table! block above.

unsafe fn nau8821_check_quirks() {
    if quirk_override != -1 {
        nau8821_quirk = quirk_override;
        return;
    }
    let dmi_id = dmi_first_match(nau8821_quirk_table.as_ptr());
    if !dmi_id.is_null() {
        nau8821_quirk = (*dmi_id).driver_data as c_ulong as c_int;
    }
}

unsafe fn nau8821_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let mut nau8821 = dev_get_platdata(&mut (*i2c).dev);
    let mut value: c_int = 0;

    if nau8821.is_null() {
        nau8821 = devm_kzalloc(dev, core::mem::size_of::<nau8821>(), GFP_KERNEL) as *mut nau8821;
        if nau8821.is_null() {
            return -ENOMEM;
        }
        nau8821_read_device_properties(dev, nau8821);
    }
    i2c_set_clientdata(i2c, nau8821 as *mut c_void);

    (*nau8821).regmap = devm_regmap_init_i2c(i2c, &nau8821_regmap_config);
    if IS_ERR((*nau8821).regmap as *const c_void) {
        return PTR_ERR((*nau8821).regmap as *const c_void);
    }

    (*nau8821).dev = dev;
    (*nau8821).irq = (*i2c).irq;

    nau8821_check_quirks();
    if (nau8821_quirk & NAU8821_QUIRK_JD_ACTIVE_HIGH) != 0 {
        (*nau8821).jkdet_polarity = 0;
    }
    if (nau8821_quirk & NAU8821_QUIRK_JD_DB_BYPASS) != 0 {
        dev_dbg(dev, b"Force bypassing jack detection debounce circuit\n\0".as_ptr() as *const c_char);
    }

    nau8821_print_device_properties(nau8821);
    nau8821_reset_chip((*nau8821).regmap);
    let ret = regmap_read((*nau8821).regmap, NAU8821_R58_I2C_DEVICE_ID, &mut value);
    if ret != 0 {
        dev_err(dev, b"Failed to read device id (%d)\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    nau8821_init_regs(nau8821);
    if (*i2c).irq != 0 {
        nau8821_setup_irq(nau8821);
    }
    devm_snd_soc_register_component(&mut (*i2c).dev, &nau8821_component_driver, &mut nau8821_dai, 1)
}

// module_i2c_driver(nau8821_driver);
// MODULE_DESCRIPTION("ASoC nau8821 driver");
// MODULE_AUTHOR("John Hsu <kchsu0@nuvoton.com>");
// MODULE_AUTHOR("Seven Lee <wtli@nuvoton.com>");
// MODULE_AUTHOR("Seven Lee <wtli@nuvoton.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
