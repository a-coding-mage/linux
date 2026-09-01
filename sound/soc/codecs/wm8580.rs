// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * wm8580.c  --  WM8580 and WM8581 ALSA Soc Audio driver
 *
 * Copyright 2008-12 Wolfson Microelectronics PLC.
 *
 * Notes:
 *  The WM8580 is a multichannel codec with S/PDIF support, featuring six
 *  DAC channels and two ADC channels.
 *
 *  The WM8581 is a multichannel codec with S/PDIF support, featuring eight
 *  DAC channels and two ADC channels.
 *
 *  Currently only the primary audio interface is supported - S/PDIF and
 *  the secondary audio interfaces are not.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type kernel_ulong_t = c_ulong;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
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
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub driver: *mut snd_soc_dai_driver,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
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
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_clkdiv: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
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
pub struct soc_mixer_control {
    pub reg: c_uint,
    pub rreg: c_uint,
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
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}
#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: kernel_ulong_t,
}
#[repr(C)]
pub struct i2c_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: i2c_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

type snd_soc_bias_level = c_uint;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 3;
const SND_SOC_BIAS_ON: snd_soc_bias_level = 0;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 1;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 2;
const SND_SOC_BIAS_OFF: snd_soc_bias_level = 3;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 1;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_DSP_A: c_uint = 3;
const SND_SOC_DAIFMT_DSP_B: c_uint = 4;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 1;
const SND_SOC_DAIFMT_IB_NF: c_uint = 2;
const SND_SOC_DAIFMT_NB_IF: c_uint = 3;
const WM8580_PLLA: c_int = 0;
const WM8580_PLLB: c_int = 1;
const WM8580_MCLK: c_int = 0;
const WM8580_CLKOUTSRC: c_int = 1;
const WM8580_CLKSRC_NONE: c_int = 0;
const WM8580_CLKSRC_MCLK: c_int = 1;
const WM8580_CLKSRC_PLLA: c_int = 2;
const WM8580_CLKSRC_PLLB: c_int = 3;
const WM8580_CLKSRC_OSC: c_int = 4;
const WM8580_CLKSRC_ADCMCLK: c_int = 5;
const WM8580_DAI_PAIFRX: c_int = 0;
const WM8580_DAI_PAIFTX: c_int = 1;

unsafe extern "C" {
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut c_void, var: c_int, min: c_uint, max: c_uint) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num_controls: c_uint) -> c_int;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widgets: *const snd_soc_dapm_widget, num: c_uint) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, routes: *const snd_soc_dapm_route, num: c_uint) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
    fn WARN(condition: c_int, fmt: *const c_char, ...) -> c_int;
}

/* WM8580 register space */
const WM8580_PLLA1: c_uint = 0x00;
const WM8580_PLLA2: c_uint = 0x01;
const WM8580_PLLA3: c_uint = 0x02;
const WM8580_PLLA4: c_uint = 0x03;
const WM8580_PLLB1: c_uint = 0x04;
const WM8580_PLLB2: c_uint = 0x05;
const WM8580_PLLB3: c_uint = 0x06;
const WM8580_PLLB4: c_uint = 0x07;
const WM8580_CLKSEL: c_uint = 0x08;
const WM8580_PAIF1: c_uint = 0x09;
const WM8580_PAIF2: c_uint = 0x0A;
const WM8580_SAIF1: c_uint = 0x0B;
const WM8580_PAIF3: c_uint = 0x0C;
const WM8580_PAIF4: c_uint = 0x0D;
const WM8580_SAIF2: c_uint = 0x0E;
const WM8580_DAC_CONTROL1: c_uint = 0x0F;
const WM8580_DAC_CONTROL2: c_uint = 0x10;
const WM8580_DAC_CONTROL3: c_uint = 0x11;
const WM8580_DAC_CONTROL4: c_uint = 0x12;
const WM8580_DAC_CONTROL5: c_uint = 0x13;
const WM8580_DIGITAL_ATTENUATION_DACL1: c_uint = 0x14;
const WM8580_DIGITAL_ATTENUATION_DACR1: c_uint = 0x15;
const WM8580_DIGITAL_ATTENUATION_DACL2: c_uint = 0x16;
const WM8580_DIGITAL_ATTENUATION_DACR2: c_uint = 0x17;
const WM8580_DIGITAL_ATTENUATION_DACL3: c_uint = 0x18;
const WM8580_DIGITAL_ATTENUATION_DACR3: c_uint = 0x19;
const WM8581_DIGITAL_ATTENUATION_DACL4: c_uint = 0x1A;
const WM8581_DIGITAL_ATTENUATION_DACR4: c_uint = 0x1B;
const WM8580_MASTER_DIGITAL_ATTENUATION: c_uint = 0x1C;
const WM8580_ADC_CONTROL1: c_uint = 0x1D;
const WM8580_SPDTXCHAN0: c_uint = 0x1E;
const WM8580_SPDTXCHAN1: c_uint = 0x1F;
const WM8580_SPDTXCHAN2: c_uint = 0x20;
const WM8580_SPDTXCHAN3: c_uint = 0x21;
const WM8580_SPDTXCHAN4: c_uint = 0x22;
const WM8580_SPDTXCHAN5: c_uint = 0x23;
const WM8580_SPDMODE: c_uint = 0x24;
const WM8580_INTMASK: c_uint = 0x25;
const WM8580_GPO1: c_uint = 0x26;
const WM8580_GPO2: c_uint = 0x27;
const WM8580_GPO3: c_uint = 0x28;
const WM8580_GPO4: c_uint = 0x29;
const WM8580_GPO5: c_uint = 0x2A;
const WM8580_INTSTAT: c_uint = 0x2B;
const WM8580_SPDRXCHAN1: c_uint = 0x2C;
const WM8580_SPDRXCHAN2: c_uint = 0x2D;
const WM8580_SPDRXCHAN3: c_uint = 0x2E;
const WM8580_SPDRXCHAN4: c_uint = 0x2F;
const WM8580_SPDRXCHAN5: c_uint = 0x30;
const WM8580_SPDSTAT: c_uint = 0x31;
const WM8580_PWRDN1: c_uint = 0x32;
const WM8580_PWRDN2: c_uint = 0x33;
const WM8580_READBACK: c_uint = 0x34;
const WM8580_RESET: c_uint = 0x35;

const WM8580_MAX_REGISTER: c_uint = 0x35;
const WM8580_DACOSR: c_uint = 0x40;

/* PLLB4 (register 7h) */
const WM8580_PLLB4_MCLKOUTSRC_MASK: c_uint = 0x60;
const WM8580_PLLB4_MCLKOUTSRC_PLLA: c_uint = 0x20;
const WM8580_PLLB4_MCLKOUTSRC_PLLB: c_uint = 0x40;
const WM8580_PLLB4_MCLKOUTSRC_OSC: c_uint = 0x60;
const WM8580_PLLB4_CLKOUTSRC_MASK: c_uint = 0x180;
const WM8580_PLLB4_CLKOUTSRC_PLLACLK: c_uint = 0x080;
const WM8580_PLLB4_CLKOUTSRC_PLLBCLK: c_uint = 0x100;
const WM8580_PLLB4_CLKOUTSRC_OSCCLK: c_uint = 0x180;

/* CLKSEL (register 8h) */
const WM8580_CLKSEL_DAC_CLKSEL_MASK: c_uint = 0x03;
const WM8580_CLKSEL_DAC_CLKSEL_PLLA: c_uint = 0x01;
const WM8580_CLKSEL_DAC_CLKSEL_PLLB: c_uint = 0x02;

/* AIF control 1 (registers 9h-bh) */
const WM8580_AIF_RATE_MASK: c_uint = 0x7;
const WM8580_AIF_BCLKSEL_MASK: c_uint = 0x18;
const WM8580_AIF_MS: c_uint = 0x20;
const WM8580_AIF_CLKSRC_MASK: c_uint = 0xc0;
const WM8580_AIF_CLKSRC_PLLA: c_uint = 0x40;
const WM8580_AIF_CLKSRC_PLLB: c_uint = 0x40;
const WM8580_AIF_CLKSRC_MCLK: c_uint = 0xc0;

/* AIF control 2 (registers ch-eh) */
const WM8580_AIF_FMT_MASK: c_uint = 0x03;
const WM8580_AIF_FMT_RIGHTJ: c_uint = 0x00;
const WM8580_AIF_FMT_LEFTJ: c_uint = 0x01;
const WM8580_AIF_FMT_I2S: c_uint = 0x02;
const WM8580_AIF_FMT_DSP: c_uint = 0x03;
const WM8580_AIF_LENGTH_MASK: c_uint = 0x0c;
const WM8580_AIF_LENGTH_16: c_uint = 0x00;
const WM8580_AIF_LENGTH_20: c_uint = 0x04;
const WM8580_AIF_LENGTH_24: c_uint = 0x08;
const WM8580_AIF_LENGTH_32: c_uint = 0x0c;
const WM8580_AIF_LRP: c_uint = 0x10;
const WM8580_AIF_BCP: c_uint = 0x20;

/* Powerdown Register 1 (register 32h) */
const WM8580_PWRDN1_PWDN: c_uint = 0x001;
const WM8580_PWRDN1_ALLDACPD: c_uint = 0x040;

/* Powerdown Register 2 (register 33h) */
const WM8580_PWRDN2_OSSCPD: c_uint = 0x001;
const WM8580_PWRDN2_PLLAPD: c_uint = 0x002;
const WM8580_PWRDN2_PLLBPD: c_uint = 0x004;
const WM8580_PWRDN2_SPDIFPD: c_uint = 0x008;
const WM8580_PWRDN2_SPDIFTXD: c_uint = 0x010;
const WM8580_PWRDN2_SPDIFRXD: c_uint = 0x020;
const WM8580_DAC_CONTROL5_MUTEALL: c_uint = 0x10;

/*
 * wm8580 register cache
 * We can't read the WM8580 register space when we
 * are using 2 wire for device control, so we cache them instead.
 */
static wm8580_reg_defaults: [reg_default; 53] = [
    reg_default { reg: 0, def: 0x0121 }, reg_default { reg: 1, def: 0x017e },
    reg_default { reg: 2, def: 0x007d }, reg_default { reg: 3, def: 0x0014 },
    reg_default { reg: 4, def: 0x0121 }, reg_default { reg: 5, def: 0x017e },
    reg_default { reg: 6, def: 0x007d }, reg_default { reg: 7, def: 0x0194 },
    reg_default { reg: 8, def: 0x0010 }, reg_default { reg: 9, def: 0x0002 },
    reg_default { reg: 10, def: 0x0002 }, reg_default { reg: 11, def: 0x00c2 },
    reg_default { reg: 12, def: 0x0182 }, reg_default { reg: 13, def: 0x0082 },
    reg_default { reg: 14, def: 0x000a }, reg_default { reg: 15, def: 0x0024 },
    reg_default { reg: 16, def: 0x0009 }, reg_default { reg: 17, def: 0x0000 },
    reg_default { reg: 18, def: 0x00ff }, reg_default { reg: 19, def: 0x0000 },
    reg_default { reg: 20, def: 0x00ff }, reg_default { reg: 21, def: 0x00ff },
    reg_default { reg: 22, def: 0x00ff }, reg_default { reg: 23, def: 0x00ff },
    reg_default { reg: 24, def: 0x00ff }, reg_default { reg: 25, def: 0x00ff },
    reg_default { reg: 26, def: 0x00ff }, reg_default { reg: 27, def: 0x00ff },
    reg_default { reg: 28, def: 0x01f0 }, reg_default { reg: 29, def: 0x0040 },
    reg_default { reg: 30, def: 0x0000 }, reg_default { reg: 31, def: 0x0000 },
    reg_default { reg: 32, def: 0x0000 }, reg_default { reg: 33, def: 0x0000 },
    reg_default { reg: 34, def: 0x0031 }, reg_default { reg: 35, def: 0x000b },
    reg_default { reg: 36, def: 0x0039 }, reg_default { reg: 37, def: 0x0000 },
    reg_default { reg: 38, def: 0x0010 }, reg_default { reg: 39, def: 0x0032 },
    reg_default { reg: 40, def: 0x0054 }, reg_default { reg: 41, def: 0x0076 },
    reg_default { reg: 42, def: 0x0098 }, reg_default { reg: 43, def: 0x0000 },
    reg_default { reg: 44, def: 0x0000 }, reg_default { reg: 45, def: 0x0000 },
    reg_default { reg: 46, def: 0x0000 }, reg_default { reg: 47, def: 0x0000 },
    reg_default { reg: 48, def: 0x0000 }, reg_default { reg: 49, def: 0x0000 },
    reg_default { reg: 50, def: 0x005e }, reg_default { reg: 51, def: 0x003e },
    reg_default { reg: 52, def: 0x0000 },
];

unsafe extern "C" fn wm8580_volatile(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        WM8580_RESET => true,
        _ => false,
    }
}

#[repr(C)]
struct pll_state {
    input: c_uint,
    out: c_uint,
}

const WM8580_NUM_SUPPLIES: usize = 3;
static wm8580_supply_names: [*const c_char; WM8580_NUM_SUPPLIES] = [
    b"AVDD\0".as_ptr() as *const c_char,
    b"DVDD\0".as_ptr() as *const c_char,
    b"PVDD\0".as_ptr() as *const c_char,
];

#[repr(C)]
struct wm8580_driver_data {
    num_dacs: c_int,
}

/* codec private data */
#[repr(C)]
struct wm8580_priv {
    regmap: *mut regmap,
    supplies: [regulator_bulk_data; WM8580_NUM_SUPPLIES],
    a: pll_state,
    b: pll_state,
    drvdata: *const wm8580_driver_data,
    sysclk: [c_int; 2],
}

/* static const DECLARE_TLV_DB_SCALE(dac_tlv, -12750, 50, 1); */
static dac_tlv: [c_uint; 4] = [0, (-12750i32) as c_uint, 50, 1];

unsafe extern "C" fn wm8580_out_vu(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*(kcontrol)).private_value as *mut soc_mixer_control;
    let component = snd_kcontrol_chip(kcontrol);
    let wm8580 = snd_soc_component_get_drvdata(component) as *mut wm8580_priv;
    let reg = (*mc).reg;
    let reg2 = (*mc).rreg;
    let ret: c_int;

    /* Clear the register cache VU so we write without VU set */
    regcache_cache_only((*wm8580).regmap, true);
    regmap_update_bits((*wm8580).regmap, reg, 0x100, 0x000);
    regmap_update_bits((*wm8580).regmap, reg2, 0x100, 0x000);
    regcache_cache_only((*wm8580).regmap, false);

    ret = snd_soc_put_volsw(kcontrol, ucontrol);
    if ret < 0 {
        return ret;
    }

    /* Now write again with the volume update bit set */
    snd_soc_component_update_bits(component, reg, 0x100, 0x100);
    snd_soc_component_update_bits(component, reg2, 0x100, 0x100);

    0
}

/* SOC_* and SND_SOC_DAPM_* macro initializers require external ALSA macro expansion. */
static wm8580_snd_controls: [snd_kcontrol_new; 0] = [];
static wm8581_snd_controls: [snd_kcontrol_new; 0] = [];
static wm8580_dapm_widgets: [snd_soc_dapm_widget; 0] = [];
static wm8581_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static wm8580_dapm_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: b"VOUT1L\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VOUT1R\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VOUT2L\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VOUT2R\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VOUT3L\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC3\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VOUT3R\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC3\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AINL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AINR\0".as_ptr() as *const c_char },
];

static wm8581_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: b"VOUT4L\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC4\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VOUT4R\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC4\0".as_ptr() as *const c_char },
];

/* PLL divisors */
#[repr(C)]
struct _pll_div {
    prescale: u32,
    postscale: u32,
    freqmode: u32,
    n: u32,
    k: u32,
}

/* The size in bits of the pll divide */
const FIXED_PLL_SIZE: u64 = 1 << 22;

/* PLL rate to output rate divisions */
#[repr(C)]
struct post_table_entry {
    div: c_uint,
    freqmode: c_uint,
    postscale: c_uint,
}

static mut post_table: [post_table_entry; 8] = [
    post_table_entry { div: 2, freqmode: 0, postscale: 0 },
    post_table_entry { div: 4, freqmode: 0, postscale: 1 },
    post_table_entry { div: 4, freqmode: 1, postscale: 0 },
    post_table_entry { div: 8, freqmode: 1, postscale: 1 },
    post_table_entry { div: 8, freqmode: 2, postscale: 0 },
    post_table_entry { div: 16, freqmode: 2, postscale: 1 },
    post_table_entry { div: 12, freqmode: 3, postscale: 0 },
    post_table_entry { div: 24, freqmode: 3, postscale: 1 },
];

unsafe extern "C" fn pll_factors(pll_div: *mut _pll_div, mut target: c_uint, mut source: c_uint) -> c_int {
    let mut Kpart: u64;
    let K: c_uint;
    let mut Ndiv: c_uint;
    let Nmod: c_uint;
    let mut i: usize;

    pr_debug(b"wm8580: PLL %uHz->%uHz\n\0".as_ptr() as *const c_char, source, target);

    /*
     * Scale the output frequency up; the PLL should run in the
     * region of 90-100MHz.
     */
    i = 0;
    while i < post_table.len() {
        if target.wrapping_mul(post_table[i].div) >= 90000000
            && target.wrapping_mul(post_table[i].div) <= 100000000
        {
            (*pll_div).freqmode = post_table[i].freqmode;
            (*pll_div).postscale = post_table[i].postscale;
            target = target.wrapping_mul(post_table[i].div);
            break;
        }
        i += 1;
    }

    if i == post_table.len() {
        printk(b"wm8580: Unable to scale output frequency %u\n\0".as_ptr() as *const c_char, target);
        return -EINVAL;
    }

    Ndiv = target / source;

    if Ndiv < 5 {
        source /= 2;
        (*pll_div).prescale = 1;
        Ndiv = target / source;
    } else {
        (*pll_div).prescale = 0;
    }

    if Ndiv < 5 || Ndiv > 13 {
        printk(b"WM8580 N=%u outside supported range\n\0".as_ptr() as *const c_char, Ndiv);
        return -EINVAL;
    }

    (*pll_div).n = Ndiv;
    Nmod = target % source;
    Kpart = FIXED_PLL_SIZE.wrapping_mul(Nmod as u64);
    Kpart /= source as u64;
    K = (Kpart & 0xFFFFFFFF) as c_uint;
    (*pll_div).k = K;

    pr_debug(
        b"PLL %x.%x prescale %d freqmode %d postscale %d\n\0".as_ptr() as *const c_char,
        (*pll_div).n,
        (*pll_div).k,
        (*pll_div).prescale,
        (*pll_div).freqmode,
        (*pll_div).postscale,
    );

    0
}

unsafe extern "C" fn wm8580_set_dai_pll(
    codec_dai: *mut snd_soc_dai,
    pll_id: c_int,
    _source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let offset: c_int;
    let component = (*codec_dai).component;
    let wm8580 = snd_soc_component_get_drvdata(component) as *mut wm8580_priv;
    let state: *mut pll_state;
    let mut pll_div = _pll_div { prescale: 0, postscale: 0, freqmode: 0, n: 0, k: 0 };
    let mut reg: c_uint;
    let pwr_mask: c_uint;
    let ret: c_int;

    match pll_id {
        WM8580_PLLA => {
            state = &mut (*wm8580).a;
            offset = 0;
            pwr_mask = WM8580_PWRDN2_PLLAPD;
        }
        WM8580_PLLB => {
            state = &mut (*wm8580).b;
            offset = 4;
            pwr_mask = WM8580_PWRDN2_PLLBPD;
        }
        _ => return -ENODEV,
    }

    if freq_in != 0 && freq_out != 0 {
        ret = pll_factors(&mut pll_div, freq_out, freq_in);
        if ret != 0 {
            return ret;
        }
    }

    (*state).input = freq_in;
    (*state).out = freq_out;

    /*
     * Always disable the PLL - it is not safe to leave it running
     * while reprogramming it.
     */
    snd_soc_component_update_bits(component, WM8580_PWRDN2, pwr_mask, pwr_mask);

    if freq_in == 0 || freq_out == 0 {
        return 0;
    }

    snd_soc_component_write(component, (WM8580_PLLA1 as c_int + offset) as c_uint, pll_div.k & 0x1ff);
    snd_soc_component_write(component, (WM8580_PLLA2 as c_int + offset) as c_uint, (pll_div.k >> 9) & 0x1ff);
    snd_soc_component_write(component, (WM8580_PLLA3 as c_int + offset) as c_uint, ((pll_div.k >> 18) & 0xf) | (pll_div.n << 4));

    reg = snd_soc_component_read(component, (WM8580_PLLA4 as c_int + offset) as c_uint);
    reg &= !0x1b;
    reg |= pll_div.prescale | (pll_div.postscale << 1) | (pll_div.freqmode << 3);

    snd_soc_component_write(component, (WM8580_PLLA4 as c_int + offset) as c_uint, reg);

    /* All done, turn it on */
    snd_soc_component_update_bits(component, WM8580_PWRDN2, pwr_mask, 0);

    0
}

static wm8580_sysclk_ratios: [c_int; 7] = [128, 192, 256, 384, 512, 768, 1152];

/*
 * Set PCM DAI bit size and sample rate.
 */
unsafe extern "C" fn wm8580_paif_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let wm8580 = snd_soc_component_get_drvdata(component) as *mut wm8580_priv;
    let mut paifa: u16 = 0;
    let mut paifb: u16 = 0;
    let mut i: usize;
    let ratio: c_int;
    let osr: c_int;

    /* bit size */
    match params_width(params) {
        16 => paifa |= 0x8,
        20 => {
            paifa |= 0x0;
            paifb |= WM8580_AIF_LENGTH_20 as u16;
        }
        24 => {
            paifa |= 0x0;
            paifb |= WM8580_AIF_LENGTH_24 as u16;
        }
        32 => {
            paifa |= 0x0;
            paifb |= WM8580_AIF_LENGTH_32 as u16;
        }
        _ => return -EINVAL,
    }

    /* Look up the SYSCLK ratio; accept only exact matches */
    ratio = (*wm8580).sysclk[(*(*dai).driver).id as usize] / params_rate(params) as c_int;
    i = 0;
    while i < wm8580_sysclk_ratios.len() {
        if ratio == wm8580_sysclk_ratios[i] {
            break;
        }
        i += 1;
    }
    if i == wm8580_sysclk_ratios.len() {
        dev_err(
            (*component).dev,
            b"Invalid clock ratio %d/%d\n\0".as_ptr() as *const c_char,
            (*wm8580).sysclk[(*(*dai).driver).id as usize],
            params_rate(params),
        );
        return -EINVAL;
    }
    paifa |= i as u16;
    dev_dbg(
        (*component).dev,
        b"Running at %dfs with %dHz clock\n\0".as_ptr() as *const c_char,
        wm8580_sysclk_ratios[i],
        (*wm8580).sysclk[(*(*dai).driver).id as usize],
    );

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        match ratio {
            128 | 192 => {
                osr = WM8580_DACOSR as c_int;
                dev_dbg((*component).dev, b"Selecting 64x OSR\n\0".as_ptr() as *const c_char);
            }
            _ => {
                osr = 0;
                dev_dbg((*component).dev, b"Selecting 128x OSR\n\0".as_ptr() as *const c_char);
            }
        }

        snd_soc_component_update_bits(component, WM8580_PAIF3, WM8580_DACOSR, osr as c_uint);
    }

    snd_soc_component_update_bits(
        component,
        WM8580_PAIF1 + (*(*dai).driver).id as c_uint,
        WM8580_AIF_RATE_MASK | WM8580_AIF_BCLKSEL_MASK,
        paifa as c_uint,
    );
    snd_soc_component_update_bits(
        component,
        WM8580_PAIF3 + (*(*dai).driver).id as c_uint,
        WM8580_AIF_LENGTH_MASK,
        paifb as c_uint,
    );
    0
}

unsafe extern "C" fn wm8580_set_paif_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut aifa: c_uint;
    let mut aifb: c_uint;
    let can_invert_lrclk: c_int;

    aifa = snd_soc_component_read(component, WM8580_PAIF1 + (*(*codec_dai).driver).id as c_uint);
    aifb = snd_soc_component_read(component, WM8580_PAIF3 + (*(*codec_dai).driver).id as c_uint);
    aifb &= !(WM8580_AIF_FMT_MASK | WM8580_AIF_LRP | WM8580_AIF_BCP);

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => aifa &= !WM8580_AIF_MS,
        SND_SOC_DAIFMT_CBP_CFP => aifa |= WM8580_AIF_MS,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            can_invert_lrclk = 1;
            aifb |= WM8580_AIF_FMT_I2S;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            can_invert_lrclk = 1;
            aifb |= WM8580_AIF_FMT_RIGHTJ;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            can_invert_lrclk = 1;
            aifb |= WM8580_AIF_FMT_LEFTJ;
        }
        SND_SOC_DAIFMT_DSP_A => {
            can_invert_lrclk = 0;
            aifb |= WM8580_AIF_FMT_DSP;
        }
        SND_SOC_DAIFMT_DSP_B => {
            can_invert_lrclk = 0;
            aifb |= WM8580_AIF_FMT_DSP;
            aifb |= WM8580_AIF_LRP;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => {
            if can_invert_lrclk == 0 {
                return -EINVAL;
            }
            aifb |= WM8580_AIF_BCP;
            aifb |= WM8580_AIF_LRP;
        }
        SND_SOC_DAIFMT_IB_NF => aifb |= WM8580_AIF_BCP,
        SND_SOC_DAIFMT_NB_IF => {
            if can_invert_lrclk == 0 {
                return -EINVAL;
            }
            aifb |= WM8580_AIF_LRP;
        }
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, WM8580_PAIF1 + (*(*codec_dai).driver).id as c_uint, aifa);
    snd_soc_component_write(component, WM8580_PAIF3 + (*(*codec_dai).driver).id as c_uint, aifb);
    0
}

unsafe extern "C" fn wm8580_set_dai_clkdiv(codec_dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int {
    let component = (*codec_dai).component;
    let mut reg: c_uint;

    match div_id {
        WM8580_MCLK => {
            reg = snd_soc_component_read(component, WM8580_PLLB4);
            reg &= !WM8580_PLLB4_MCLKOUTSRC_MASK;
            match div {
                WM8580_CLKSRC_MCLK => {}
                WM8580_CLKSRC_PLLA => reg |= WM8580_PLLB4_MCLKOUTSRC_PLLA,
                WM8580_CLKSRC_PLLB => reg |= WM8580_PLLB4_MCLKOUTSRC_PLLB,
                WM8580_CLKSRC_OSC => reg |= WM8580_PLLB4_MCLKOUTSRC_OSC,
                _ => return -EINVAL,
            }
            snd_soc_component_write(component, WM8580_PLLB4, reg);
        }
        WM8580_CLKOUTSRC => {
            reg = snd_soc_component_read(component, WM8580_PLLB4);
            reg &= !WM8580_PLLB4_CLKOUTSRC_MASK;
            match div {
                WM8580_CLKSRC_NONE => {}
                WM8580_CLKSRC_PLLA => reg |= WM8580_PLLB4_CLKOUTSRC_PLLACLK,
                WM8580_CLKSRC_PLLB => reg |= WM8580_PLLB4_CLKOUTSRC_PLLBCLK,
                WM8580_CLKSRC_OSC => reg |= WM8580_PLLB4_CLKOUTSRC_OSCCLK,
                _ => return -EINVAL,
            }
            snd_soc_component_write(component, WM8580_PLLB4, reg);
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn wm8580_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let wm8580 = snd_soc_component_get_drvdata(component) as *mut wm8580_priv;
    let ret: c_int;
    let sel: c_int;
    let sel_mask: c_int;
    let sel_shift: c_int;

    match (*(*dai).driver).id {
        WM8580_DAI_PAIFRX => {
            sel_mask = 0x3;
            sel_shift = 0;
        }
        WM8580_DAI_PAIFTX => {
            sel_mask = 0xc;
            sel_shift = 2;
        }
        _ => {
            WARN(1, b"Unknown DAI driver ID\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    match clk_id {
        WM8580_CLKSRC_ADCMCLK => {
            if (*(*dai).driver).id != WM8580_DAI_PAIFTX {
                return -EINVAL;
            }
            sel = 0 << sel_shift;
        }
        WM8580_CLKSRC_PLLA => sel = 1 << sel_shift,
        WM8580_CLKSRC_PLLB => sel = 2 << sel_shift,
        WM8580_CLKSRC_MCLK => sel = 3 << sel_shift,
        _ => {
            dev_err((*component).dev, b"Unknown clock %d\n\0".as_ptr() as *const c_char, clk_id);
            return -EINVAL;
        }
    }

    /* We really should validate PLL settings but not yet */
    (*wm8580).sysclk[(*(*dai).driver).id as usize] = freq as c_int;

    ret = snd_soc_component_update_bits(component, WM8580_CLKSEL, sel_mask as c_uint, sel as c_uint);
    if ret < 0 {
        return ret;
    }
    0
}

unsafe extern "C" fn wm8580_mute(codec_dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*codec_dai).component;
    let mut reg: c_uint;

    reg = snd_soc_component_read(component, WM8580_DAC_CONTROL5);
    if mute != 0 {
        reg |= WM8580_DAC_CONTROL5_MUTEALL;
    } else {
        reg &= !WM8580_DAC_CONTROL5_MUTEALL;
    }
    snd_soc_component_write(component, WM8580_DAC_CONTROL5, reg);
    0
}

unsafe extern "C" fn wm8580_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);

    match level {
        SND_SOC_BIAS_ON | SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                /* Power up and get individual control of the DACs */
                snd_soc_component_update_bits(component, WM8580_PWRDN1, WM8580_PWRDN1_PWDN | WM8580_PWRDN1_ALLDACPD, 0);
                /* Make VMID high impedance */
                snd_soc_component_update_bits(component, WM8580_ADC_CONTROL1, 0x100, 0);
            }
        }
        SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(component, WM8580_PWRDN1, WM8580_PWRDN1_PWDN, WM8580_PWRDN1_PWDN);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn wm8580_playback_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let wm8580 = snd_soc_component_get_drvdata(component) as *mut wm8580_priv;

    snd_pcm_hw_constraint_minmax(
        (*substream).runtime,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        1,
        ((*(*wm8580).drvdata).num_dacs * 2) as c_uint,
    )
}

const WM8580_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static wm8580_dai_ops_playback: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(wm8580_playback_startup),
    set_sysclk: Some(wm8580_set_sysclk),
    hw_params: Some(wm8580_paif_hw_params),
    set_fmt: Some(wm8580_set_paif_dai_fmt),
    set_clkdiv: Some(wm8580_set_dai_clkdiv),
    set_pll: Some(wm8580_set_dai_pll),
    mute_stream: Some(wm8580_mute),
    no_capture_mute: 1,
};

static wm8580_dai_ops_capture: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: None,
    set_sysclk: Some(wm8580_set_sysclk),
    hw_params: Some(wm8580_paif_hw_params),
    set_fmt: Some(wm8580_set_paif_dai_fmt),
    set_clkdiv: Some(wm8580_set_dai_clkdiv),
    set_pll: Some(wm8580_set_dai_pll),
    mute_stream: None,
    no_capture_mute: 0,
};

static mut wm8580_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: b"wm8580-hifi-playback\0".as_ptr() as *const c_char,
        id: WM8580_DAI_PAIFRX,
        playback: snd_soc_pcm_stream {
            stream_name: b"Playback\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 0,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: WM8580_FORMATS,
        },
        capture: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        ops: &wm8580_dai_ops_playback,
    },
    snd_soc_dai_driver {
        name: b"wm8580-hifi-capture\0".as_ptr() as *const c_char,
        id: WM8580_DAI_PAIFTX,
        playback: snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        capture: snd_soc_pcm_stream {
            stream_name: b"Capture\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: WM8580_FORMATS,
        },
        ops: &wm8580_dai_ops_capture,
    },
];

unsafe extern "C" fn wm8580_probe(component: *mut snd_soc_component) -> c_int {
    let wm8580 = snd_soc_component_get_drvdata(component) as *mut wm8580_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int = 0;

    match (*(*wm8580).drvdata).num_dacs {
        4 => {
            snd_soc_add_component_controls(component, wm8581_snd_controls.as_ptr(), wm8581_snd_controls.len() as c_uint);
            snd_soc_dapm_new_controls(dapm, wm8581_dapm_widgets.as_ptr(), wm8581_dapm_widgets.len() as c_uint);
            snd_soc_dapm_add_routes(dapm, wm8581_dapm_routes.as_ptr(), wm8581_dapm_routes.len() as c_uint);
        }
        _ => {}
    }

    ret = regulator_bulk_enable((*wm8580).supplies.len() as c_uint, (*wm8580).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    /* Get the codec into a known state */
    ret = snd_soc_component_write(component, WM8580_RESET, 0);
    if ret != 0 {
        dev_err((*component).dev, b"Failed to reset component: %d\n\0".as_ptr() as *const c_char, ret);
        regulator_bulk_disable((*wm8580).supplies.len() as c_uint, (*wm8580).supplies.as_mut_ptr());
        return ret;
    }

    0
}

/* power down chip */
unsafe extern "C" fn wm8580_remove(component: *mut snd_soc_component) {
    let wm8580 = snd_soc_component_get_drvdata(component) as *mut wm8580_priv;
    regulator_bulk_disable((*wm8580).supplies.len() as c_uint, (*wm8580).supplies.as_mut_ptr());
}

static soc_component_dev_wm8580: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8580_probe),
    remove: Some(wm8580_remove),
    set_bias_level: Some(wm8580_set_bias_level),
    controls: wm8580_snd_controls.as_ptr(),
    num_controls: wm8580_snd_controls.len() as c_uint,
    dapm_widgets: wm8580_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8580_dapm_widgets.len() as c_uint,
    dapm_routes: wm8580_dapm_routes.as_ptr(),
    num_dapm_routes: wm8580_dapm_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8580_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: WM8580_MAX_REGISTER,
    reg_defaults: wm8580_reg_defaults.as_ptr(),
    num_reg_defaults: wm8580_reg_defaults.len() as c_uint,
    cache_type: REGCACHE_MAPLE,
    volatile_reg: Some(wm8580_volatile),
};

static wm8580_data: wm8580_driver_data = wm8580_driver_data { num_dacs: 3 };
static wm8581_data: wm8580_driver_data = wm8580_driver_data { num_dacs: 4 };

unsafe extern "C" fn wm8580_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8580: *mut wm8580_priv;
    let mut ret: c_int;
    let mut i: usize;

    wm8580 = devm_kzalloc(&mut (*i2c).dev, size_of::<wm8580_priv>(), GFP_KERNEL) as *mut wm8580_priv;
    if wm8580.is_null() {
        return -ENOMEM;
    }

    (*wm8580).regmap = devm_regmap_init_i2c(i2c, &wm8580_regmap);
    if IS_ERR((*wm8580).regmap as *const c_void) {
        return PTR_ERR((*wm8580).regmap as *const c_void);
    }

    i = 0;
    while i < (*wm8580).supplies.len() {
        (*wm8580).supplies[i].supply = wm8580_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(&mut (*i2c).dev, (*wm8580).supplies.len() as c_uint, (*wm8580).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    i2c_set_clientdata(i2c, wm8580 as *mut c_void);

    (*wm8580).drvdata = i2c_get_match_data(i2c) as *const wm8580_driver_data;
    if (*wm8580).drvdata.is_null() {
        return dev_err_probe(&mut (*i2c).dev, -EINVAL, b"failed to find driver data\n\0".as_ptr() as *const c_char);
    }

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm8580,
        wm8580_dai.as_mut_ptr(),
        wm8580_dai.len() as c_int,
    );

    ret
}

static wm8580_of_match: [of_device_id; 3] = [
    of_device_id { compatible: b"wlf,wm8580\0".as_ptr() as *const c_char, data: &wm8580_data as *const _ as *const c_void },
    of_device_id { compatible: b"wlf,wm8581\0".as_ptr() as *const c_char, data: &wm8581_data as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, wm8580_of_match); */

static wm8580_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id { name: b"wm8580\0".as_ptr() as *const c_char, driver_data: &wm8580_data as *const _ as kernel_ulong_t },
    i2c_device_id { name: b"wm8581\0".as_ptr() as *const c_char, driver_data: &wm8581_data as *const _ as kernel_ulong_t },
    i2c_device_id { name: ptr::null(), driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(i2c, wm8580_i2c_id); */

static mut wm8580_i2c_driver: i2c_driver = i2c_driver {
    driver: i2c_driver_inner {
        name: b"wm8580\0".as_ptr() as *const c_char,
        of_match_table: wm8580_of_match.as_ptr(),
    },
    probe: Some(wm8580_i2c_probe),
    id_table: wm8580_i2c_id.as_ptr(),
};

/* module_i2c_driver(wm8580_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC WM8580 driver"); */
/* MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>"); */
/* MODULE_AUTHOR("Matt Flax <flatmax@flatmax.org>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
