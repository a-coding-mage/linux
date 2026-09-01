// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8510.c  --  WM8510 ALSA Soc Audio driver
 *
 * Copyright 2006 Wolfson Microelectronics PLC.
 *
 * Author: Liam Girdwood <lrg@slimlogic.co.uk>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::ptr;

// C include dependencies translated as external dependencies:
// linux/module.h, linux/moduleparam.h, linux/kernel.h, linux/init.h,
// linux/delay.h, linux/pm.h, linux/i2c.h, linux/spi/spi.h, linux/slab.h,
// linux/regmap.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
// sound/soc.h, sound/initval.h, and "wm8510.h".

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
    _private: [u8; 0],
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
pub struct spi_device {
    pub dev: device,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

pub type bool_ = bool;
pub type u16 = u16;
pub type snd_soc_bias_level = c_uint;

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
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
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_clkdiv: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub no_capture_mute: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulonglong,
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
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
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
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
}

#[repr(C)]
pub struct driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

extern "C" {
    static WM8510_RESET: c_uint;
    static WM8510_COMP: c_uint;
    static WM8510_DAC: c_uint;
    static WM8510_ALC3: c_uint;
    static WM8510_DACVOL: c_uint;
    static WM8510_ADC: c_uint;
    static WM8510_ADCVOL: c_uint;
    static WM8510_DACLIM1: c_uint;
    static WM8510_DACLIM2: c_uint;
    static WM8510_ALC1: c_uint;
    static WM8510_ALC2: c_uint;
    static WM8510_NGATE: c_uint;
    static WM8510_INPPGA: c_uint;
    static WM8510_SPKVOL: c_uint;
    static WM8510_OUTPUT: c_uint;
    static WM8510_ADCBOOST: c_uint;
    static WM8510_MONOMIX: c_uint;
    static WM8510_SPKMIX: c_uint;
    static WM8510_POWER1: c_uint;
    static WM8510_POWER2: c_uint;
    static WM8510_POWER3: c_uint;
    static WM8510_INPUT: c_uint;
    static WM8510_CLOCK: c_uint;
    static WM8510_PLLN: c_uint;
    static WM8510_PLLK1: c_uint;
    static WM8510_PLLK2: c_uint;
    static WM8510_PLLK3: c_uint;
    static WM8510_GPIO: c_uint;
    static WM8510_IFACE: c_uint;
    static WM8510_ADD: c_uint;
    static WM8510_OPCLKDIV: c_int;
    static WM8510_MCLKDIV: c_int;
    static WM8510_ADCCLK: c_int;
    static WM8510_DACCLK: c_int;
    static WM8510_BCLKDIV: c_int;
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
    static SND_SOC_BIAS_ON: snd_soc_bias_level;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_11025: c_uint;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_RATE_22050: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_ulonglong;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_ulonglong;
    static SNDRV_PCM_FMTBIT_S24_LE: c_ulonglong;
    static SNDRV_PCM_FMTBIT_S32_LE: c_ulonglong;
    static REGCACHE_MAPLE: c_uint;
    static GFP_KERNEL: c_uint;

    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn mdelay(msecs: c_uint);
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn i2c_add_driver(driver: *mut i2c_driver) -> c_int;
    fn i2c_del_driver(driver: *mut i2c_driver);
    fn spi_register_driver(driver: *mut spi_driver) -> c_int;
    fn spi_unregister_driver(driver: *mut spi_driver);
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const KERN_WARNING: &[u8] = b"\0";
const KERN_ERR: &[u8] = b"\0";
const WM8510_POWER1_BIASEN: u16 = 0x08;
const WM8510_POWER1_BUFIOEN: u16 = 0x10;
const FIXED_PLL_SIZE: c_ulonglong = ((1u64 << 24) * 10) as c_ulonglong;

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ARRAY_SIZE {
    ($a:expr) => {
        $a.len() as c_uint
    };
}

unsafe fn wm8510_reset(c: *mut snd_soc_component) -> c_int {
    snd_soc_component_write(c, WM8510_RESET, 0)
}

/*
 * wm8510 register cache
 * We can't read the WM8510 register space when we are
 * using 2 wire for device control, so we cache them instead.
 */
static wm8510_reg_defaults: [reg_default; 56] = [
    reg_default { reg: 1, def: 0x0000 }, reg_default { reg: 2, def: 0x0000 },
    reg_default { reg: 3, def: 0x0000 }, reg_default { reg: 4, def: 0x0050 },
    reg_default { reg: 5, def: 0x0000 }, reg_default { reg: 6, def: 0x0140 },
    reg_default { reg: 7, def: 0x0000 }, reg_default { reg: 8, def: 0x0000 },
    reg_default { reg: 9, def: 0x0000 }, reg_default { reg: 10, def: 0x0000 },
    reg_default { reg: 11, def: 0x00ff }, reg_default { reg: 12, def: 0x0000 },
    reg_default { reg: 13, def: 0x0000 }, reg_default { reg: 14, def: 0x0100 },
    reg_default { reg: 15, def: 0x00ff }, reg_default { reg: 16, def: 0x0000 },
    reg_default { reg: 17, def: 0x0000 }, reg_default { reg: 18, def: 0x012c },
    reg_default { reg: 19, def: 0x002c }, reg_default { reg: 20, def: 0x002c },
    reg_default { reg: 21, def: 0x002c }, reg_default { reg: 22, def: 0x002c },
    reg_default { reg: 23, def: 0x0000 }, reg_default { reg: 24, def: 0x0032 },
    reg_default { reg: 25, def: 0x0000 }, reg_default { reg: 26, def: 0x0000 },
    reg_default { reg: 27, def: 0x0000 }, reg_default { reg: 28, def: 0x0000 },
    reg_default { reg: 29, def: 0x0000 }, reg_default { reg: 30, def: 0x0000 },
    reg_default { reg: 31, def: 0x0000 }, reg_default { reg: 32, def: 0x0038 },
    reg_default { reg: 33, def: 0x000b }, reg_default { reg: 34, def: 0x0032 },
    reg_default { reg: 35, def: 0x0000 }, reg_default { reg: 36, def: 0x0008 },
    reg_default { reg: 37, def: 0x000c }, reg_default { reg: 38, def: 0x0093 },
    reg_default { reg: 39, def: 0x00e9 }, reg_default { reg: 40, def: 0x0000 },
    reg_default { reg: 41, def: 0x0000 }, reg_default { reg: 42, def: 0x0000 },
    reg_default { reg: 43, def: 0x0000 }, reg_default { reg: 44, def: 0x0003 },
    reg_default { reg: 45, def: 0x0010 }, reg_default { reg: 46, def: 0x0000 },
    reg_default { reg: 47, def: 0x0000 }, reg_default { reg: 48, def: 0x0000 },
    reg_default { reg: 49, def: 0x0002 }, reg_default { reg: 50, def: 0x0001 },
    reg_default { reg: 51, def: 0x0000 }, reg_default { reg: 52, def: 0x0000 },
    reg_default { reg: 53, def: 0x0000 }, reg_default { reg: 54, def: 0x0039 },
    reg_default { reg: 55, def: 0x0000 }, reg_default { reg: 56, def: 0x0001 },
];

unsafe extern "C" fn wm8510_volatile(_dev: *mut device, reg: c_uint) -> bool {
    if reg == WM8510_RESET {
        true
    } else {
        false
    }
}

/* codec private data */
#[repr(C)]
pub struct wm8510_priv {
    pub regmap: *mut regmap,
}

static wm8510_companding: [*const c_char; 4] =
    [c_str!("Off"), c_str!("NC"), c_str!("u-law"), c_str!("A-law")];
static wm8510_deemp: [*const c_char; 4] =
    [c_str!("None"), c_str!("32kHz"), c_str!("44.1kHz"), c_str!("48kHz")];
static wm8510_alc: [*const c_char; 2] = [c_str!("ALC"), c_str!("Limiter")];

// SOC_ENUM_SINGLE/SOC_SINGLE/SOC_ENUM/SOC_DAPM_* and SND_SOC_DAPM_* are C
// descriptor-building macros supplied by ASoC headers. Their generated static
// descriptor contents are preserved here as macro-intent comments.
static wm8510_enum: [soc_enum; 0] = [];
static wm8510_snd_controls: [snd_kcontrol_new; 0] = [];
static wm8510_speaker_mixer_controls: [snd_kcontrol_new; 0] = [];
static wm8510_mono_mixer_controls: [snd_kcontrol_new; 0] = [];
static wm8510_boost_controls: [snd_kcontrol_new; 0] = [];
static wm8510_micpga_controls: [snd_kcontrol_new; 0] = [];
static wm8510_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static wm8510_dapm_routes: [snd_soc_dapm_route; 19] = [
    snd_soc_dapm_route { sink: c_str!("Mono Mixer"), control: c_str!("PCM Playback Switch"), source: c_str!("DAC") },
    snd_soc_dapm_route { sink: c_str!("Mono Mixer"), control: c_str!("Aux Playback Switch"), source: c_str!("Aux Input") },
    snd_soc_dapm_route { sink: c_str!("Mono Mixer"), control: c_str!("Line Bypass Switch"), source: c_str!("Boost Mixer") },
    snd_soc_dapm_route { sink: c_str!("Speaker Mixer"), control: c_str!("PCM Playback Switch"), source: c_str!("DAC") },
    snd_soc_dapm_route { sink: c_str!("Speaker Mixer"), control: c_str!("Aux Playback Switch"), source: c_str!("Aux Input") },
    snd_soc_dapm_route { sink: c_str!("Speaker Mixer"), control: c_str!("Line Bypass Switch"), source: c_str!("Boost Mixer") },
    snd_soc_dapm_route { sink: c_str!("Mono Out"), control: ptr::null(), source: c_str!("Mono Mixer") },
    snd_soc_dapm_route { sink: c_str!("MONOOUT"), control: ptr::null(), source: c_str!("Mono Out") },
    snd_soc_dapm_route { sink: c_str!("SpkN Out"), control: ptr::null(), source: c_str!("Speaker Mixer") },
    snd_soc_dapm_route { sink: c_str!("SpkP Out"), control: ptr::null(), source: c_str!("Speaker Mixer") },
    snd_soc_dapm_route { sink: c_str!("SPKOUTN"), control: ptr::null(), source: c_str!("SpkN Out") },
    snd_soc_dapm_route { sink: c_str!("SPKOUTP"), control: ptr::null(), source: c_str!("SpkP Out") },
    snd_soc_dapm_route { sink: c_str!("Mic PGA"), control: c_str!("MICN Switch"), source: c_str!("MICN") },
    snd_soc_dapm_route { sink: c_str!("Mic PGA"), control: c_str!("MICP Switch"), source: c_str!("MICP") },
    snd_soc_dapm_route { sink: c_str!("Mic PGA"), control: c_str!("AUX Switch"), source: c_str!("Aux Input") },
    snd_soc_dapm_route { sink: c_str!("Boost Mixer"), control: c_str!("Mic PGA Switch"), source: c_str!("Mic PGA") },
    snd_soc_dapm_route { sink: c_str!("Boost Mixer"), control: c_str!("Mic Volume"), source: c_str!("MICP") },
    snd_soc_dapm_route { sink: c_str!("Boost Mixer"), control: c_str!("Aux Volume"), source: c_str!("Aux Input") },
    snd_soc_dapm_route { sink: c_str!("ADC"), control: ptr::null(), source: c_str!("Boost Mixer") },
];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_ {
    pub pre_div: c_uint, /* prescale - 1; C bit-field width: 4 */
    pub n: c_uint,       /* C bit-field width: 4 */
    pub k: c_uint,
}

static mut pll_div: pll_ = pll_ { pre_div: 0, n: 0, k: 0 };

unsafe fn do_div(n: &mut c_ulonglong, base: c_uint) -> c_uint {
    let rem = (*n % base as c_ulonglong) as c_uint;
    *n /= base as c_ulonglong;
    rem
}

unsafe extern "C" fn pll_factors(target: c_uint, mut source: c_uint) {
    let mut Kpart: c_ulonglong;
    let mut K: c_uint;
    let mut Ndiv: c_uint;
    let Nmod: c_uint;

    Ndiv = target / source;
    if Ndiv < 6 {
        source >>= 1;
        pll_div.pre_div = 1;
        Ndiv = target / source;
    } else {
        pll_div.pre_div = 0;
    }

    if (Ndiv < 6) || (Ndiv > 12) {
        printk(
            c_str!("WM8510 N value %u outwith recommended range!d\n"),
            Ndiv,
        );
    }

    pll_div.n = Ndiv;
    Nmod = target % source;
    Kpart = FIXED_PLL_SIZE.wrapping_mul(Nmod as c_ulonglong);

    do_div(&mut Kpart, source);

    K = (Kpart & 0xFFFFFFFF) as c_uint;

    /* Check if we need to round */
    if (K % 10) >= 5 {
        K = K.wrapping_add(5);
    }

    /* Move down to proper range now rounding is done */
    K /= 10;

    pll_div.k = K;
}

unsafe extern "C" fn wm8510_set_dai_pll(
    codec_dai: *mut snd_soc_dai,
    _pll_id: c_int,
    _source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let component = (*codec_dai).component;
    let mut reg: u16;

    if freq_in == 0 || freq_out == 0 {
        /* Clock CODEC directly from MCLK */
        reg = snd_soc_component_read(component, WM8510_CLOCK) as u16;
        snd_soc_component_write(component, WM8510_CLOCK, (reg & 0x0ff) as c_uint);

        /* Turn off PLL */
        reg = snd_soc_component_read(component, WM8510_POWER1) as u16;
        snd_soc_component_write(component, WM8510_POWER1, (reg & 0x1df) as c_uint);
        return 0;
    }

    pll_factors(freq_out.wrapping_mul(4), freq_in);

    snd_soc_component_write(component, WM8510_PLLN, (pll_div.pre_div << 4) | pll_div.n);
    snd_soc_component_write(component, WM8510_PLLK1, pll_div.k >> 18);
    snd_soc_component_write(component, WM8510_PLLK2, (pll_div.k >> 9) & 0x1ff);
    snd_soc_component_write(component, WM8510_PLLK3, pll_div.k & 0x1ff);
    reg = snd_soc_component_read(component, WM8510_POWER1) as u16;
    snd_soc_component_write(component, WM8510_POWER1, (reg | 0x020) as c_uint);

    /* Run CODEC from PLL instead of MCLK */
    reg = snd_soc_component_read(component, WM8510_CLOCK) as u16;
    snd_soc_component_write(component, WM8510_CLOCK, (reg | 0x100) as c_uint);

    0
}

/*
 * Configure WM8510 clock dividers.
 */
unsafe extern "C" fn wm8510_set_dai_clkdiv(
    codec_dai: *mut snd_soc_dai,
    div_id: c_int,
    div: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let mut reg: u16;

    if div_id == WM8510_OPCLKDIV {
        reg = (snd_soc_component_read(component, WM8510_GPIO) & 0x1cf) as u16;
        snd_soc_component_write(component, WM8510_GPIO, (reg as c_int | div) as c_uint);
    } else if div_id == WM8510_MCLKDIV {
        reg = (snd_soc_component_read(component, WM8510_CLOCK) & 0x11f) as u16;
        snd_soc_component_write(component, WM8510_CLOCK, (reg as c_int | div) as c_uint);
    } else if div_id == WM8510_ADCCLK {
        reg = (snd_soc_component_read(component, WM8510_ADC) & 0x1f7) as u16;
        snd_soc_component_write(component, WM8510_ADC, (reg as c_int | div) as c_uint);
    } else if div_id == WM8510_DACCLK {
        reg = (snd_soc_component_read(component, WM8510_DAC) & 0x1f7) as u16;
        snd_soc_component_write(component, WM8510_DAC, (reg as c_int | div) as c_uint);
    } else if div_id == WM8510_BCLKDIV {
        reg = (snd_soc_component_read(component, WM8510_CLOCK) & 0x1e3) as u16;
        snd_soc_component_write(component, WM8510_CLOCK, (reg as c_int | div) as c_uint);
    } else {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn wm8510_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut iface: u16 = 0;
    let mut clk: u16 = (snd_soc_component_read(component, WM8510_CLOCK) & 0x1fe) as u16;

    /* set master/slave audio interface */
    let master = fmt & SND_SOC_DAIFMT_MASTER_MASK;
    if master == SND_SOC_DAIFMT_CBP_CFP {
        clk |= 0x0001;
    } else if master == SND_SOC_DAIFMT_CBC_CFC {
    } else {
        return -EINVAL;
    }

    /* interface format */
    let format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
    if format == SND_SOC_DAIFMT_I2S {
        iface |= 0x0010;
    } else if format == SND_SOC_DAIFMT_RIGHT_J {
    } else if format == SND_SOC_DAIFMT_LEFT_J {
        iface |= 0x0008;
    } else if format == SND_SOC_DAIFMT_DSP_A {
        iface |= 0x0018;
    } else {
        return -EINVAL;
    }

    /* clock inversion */
    let inv = fmt & SND_SOC_DAIFMT_INV_MASK;
    if inv == SND_SOC_DAIFMT_NB_NF {
    } else if inv == SND_SOC_DAIFMT_IB_IF {
        iface |= 0x0180;
    } else if inv == SND_SOC_DAIFMT_IB_NF {
        iface |= 0x0100;
    } else if inv == SND_SOC_DAIFMT_NB_IF {
        iface |= 0x0080;
    } else {
        return -EINVAL;
    }

    snd_soc_component_write(component, WM8510_IFACE, iface as c_uint);
    snd_soc_component_write(component, WM8510_CLOCK, clk as c_uint);
    0
}

unsafe extern "C" fn wm8510_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let mut iface: u16 = (snd_soc_component_read(component, WM8510_IFACE) & 0x19f) as u16;
    let mut adn: u16 = (snd_soc_component_read(component, WM8510_ADD) & 0x1f1) as u16;

    /* bit size */
    match params_width(params) {
        16 => {}
        20 => iface |= 0x0020,
        24 => iface |= 0x0040,
        32 => iface |= 0x0060,
        _ => {}
    }

    /* filter coefficient */
    match params_rate(params) {
        8000 => adn |= 0x5 << 1,
        11025 => adn |= 0x4 << 1,
        16000 => adn |= 0x3 << 1,
        22050 => adn |= 0x2 << 1,
        32000 => adn |= 0x1 << 1,
        44100 | 48000 => {}
        _ => {}
    }

    snd_soc_component_write(component, WM8510_IFACE, iface as c_uint);
    snd_soc_component_write(component, WM8510_ADD, adn as c_uint);
    0
}

unsafe extern "C" fn wm8510_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    let mute_reg: u16 = (snd_soc_component_read(component, WM8510_DAC) & 0xffbf) as u16;

    if mute != 0 {
        snd_soc_component_write(component, WM8510_DAC, (mute_reg | 0x40) as c_uint);
    } else {
        snd_soc_component_write(component, WM8510_DAC, mute_reg as c_uint);
    }
    0
}

/* liam need to make this lower power with dapm */
unsafe extern "C" fn wm8510_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let wm8510 = snd_soc_component_get_drvdata(component) as *mut wm8510_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut power1: u16 = (snd_soc_component_read(component, WM8510_POWER1) & !0x3) as u16;

    if level == SND_SOC_BIAS_ON || level == SND_SOC_BIAS_PREPARE {
        power1 |= 0x1; /* VMID 50k */
        snd_soc_component_write(component, WM8510_POWER1, power1 as c_uint);
    } else if level == SND_SOC_BIAS_STANDBY {
        power1 |= WM8510_POWER1_BIASEN | WM8510_POWER1_BUFIOEN;

        if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
            regcache_sync((*wm8510).regmap);

            /* Initial cap charge at VMID 5k */
            snd_soc_component_write(component, WM8510_POWER1, (power1 | 0x3) as c_uint);
            mdelay(100);
        }

        power1 |= 0x2; /* VMID 500k */
        snd_soc_component_write(component, WM8510_POWER1, power1 as c_uint);
    } else if level == SND_SOC_BIAS_OFF {
        snd_soc_component_write(component, WM8510_POWER1, 0);
        snd_soc_component_write(component, WM8510_POWER2, 0);
        snd_soc_component_write(component, WM8510_POWER3, 0);
    }

    0
}

unsafe fn WM8510_RATES() -> c_uint {
    SNDRV_PCM_RATE_8000
        | SNDRV_PCM_RATE_11025
        | SNDRV_PCM_RATE_16000
        | SNDRV_PCM_RATE_22050
        | SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
}

unsafe fn WM8510_FORMATS() -> c_ulonglong {
    SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S20_3LE
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_S32_LE
}

static wm8510_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8510_pcm_hw_params),
    mute_stream: Some(wm8510_mute),
    set_fmt: Some(wm8510_set_dai_fmt),
    set_clkdiv: Some(wm8510_set_dai_clkdiv),
    set_pll: Some(wm8510_set_dai_pll),
    no_capture_mute: 1,
};

static mut wm8510_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c_str!("wm8510-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("Playback"),
        channels_min: 2,
        channels_max: 2,
        rates: 0,   // WM8510_RATES(), uses external constants from headers.
        formats: 0, // WM8510_FORMATS(), uses external constants from headers.
    },
    capture: snd_soc_pcm_stream {
        stream_name: c_str!("Capture"),
        channels_min: 2,
        channels_max: 2,
        rates: 0,   // WM8510_RATES(), uses external constants from headers.
        formats: 0, // WM8510_FORMATS(), uses external constants from headers.
    },
    ops: &wm8510_dai_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn wm8510_probe(component: *mut snd_soc_component) -> c_int {
    wm8510_reset(component);

    0
}

static soc_component_dev_wm8510: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8510_probe),
    set_bias_level: Some(wm8510_set_bias_level),
    controls: wm8510_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(wm8510_snd_controls),
    dapm_widgets: wm8510_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(wm8510_dapm_widgets),
    dapm_routes: wm8510_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(wm8510_dapm_routes),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8510_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c_str!("wlf,wm8510") },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, wm8510_of_match);

static wm8510_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: 0, // WM8510_MONOMIX, supplied by "wm8510.h".
    reg_defaults: wm8510_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(wm8510_reg_defaults),
    cache_type: 0, // REGCACHE_MAPLE, supplied by linux/regmap.h.
    volatile_reg: Some(wm8510_volatile),
};

// Original C condition: #if defined(CONFIG_SPI_MASTER)
unsafe extern "C" fn wm8510_spi_probe(spi: *mut spi_device) -> c_int {
    let mut wm8510: *mut wm8510_priv;
    let ret: c_int;

    wm8510 = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<wm8510_priv>(), GFP_KERNEL)
        as *mut wm8510_priv;
    if wm8510.is_null() {
        return -ENOMEM;
    }

    (*wm8510).regmap = devm_regmap_init_spi(spi, &wm8510_regmap);
    if IS_ERR((*wm8510).regmap as *const c_void) {
        return PTR_ERR((*wm8510).regmap as *const c_void);
    }

    spi_set_drvdata(spi, wm8510 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &soc_component_dev_wm8510,
        &raw mut wm8510_dai,
        1,
    );

    ret
}

static mut wm8510_spi_driver: spi_driver = spi_driver {
    driver: driver_inner {
        name: c_str!("wm8510"),
        of_match_table: wm8510_of_match.as_ptr(),
    },
    probe: Some(wm8510_spi_probe),
};

// Original C condition: #if IS_ENABLED(CONFIG_I2C)
unsafe extern "C" fn wm8510_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut wm8510: *mut wm8510_priv;
    let ret: c_int;

    wm8510 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8510_priv>(), GFP_KERNEL)
        as *mut wm8510_priv;
    if wm8510.is_null() {
        return -ENOMEM;
    }

    (*wm8510).regmap = devm_regmap_init_i2c(i2c, &wm8510_regmap);
    if IS_ERR((*wm8510).regmap as *const c_void) {
        return PTR_ERR((*wm8510).regmap as *const c_void);
    }

    i2c_set_clientdata(i2c, wm8510 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm8510,
        &raw mut wm8510_dai,
        1,
    );

    ret
}

static wm8510_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: c_str!("wm8510") },
    i2c_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, wm8510_i2c_id);

static mut wm8510_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_inner {
        name: c_str!("wm8510"),
        of_match_table: wm8510_of_match.as_ptr(),
    },
    probe: Some(wm8510_i2c_probe),
    id_table: wm8510_i2c_id.as_ptr(),
};

unsafe extern "C" fn wm8510_modinit() -> c_int {
    let mut ret: c_int = 0;

    // Original C condition: #if IS_ENABLED(CONFIG_I2C)
    ret = i2c_add_driver(&raw mut wm8510_i2c_driver);
    if ret != 0 {
        printk(c_str!("Failed to register WM8510 I2C driver: %d\n"), ret);
    }

    // Original C condition: #if defined(CONFIG_SPI_MASTER)
    ret = spi_register_driver(&raw mut wm8510_spi_driver);
    if ret != 0 {
        printk(c_str!("Failed to register WM8510 SPI driver: %d\n"), ret);
    }

    ret
}
// module_init(wm8510_modinit);

unsafe extern "C" fn wm8510_exit() {
    // Original C condition: #if IS_ENABLED(CONFIG_I2C)
    i2c_del_driver(&raw mut wm8510_i2c_driver);
    // Original C condition: #if defined(CONFIG_SPI_MASTER)
    spi_unregister_driver(&raw mut wm8510_spi_driver);
}
// module_exit(wm8510_exit);

// MODULE_DESCRIPTION("ASoC WM8510 driver");
// MODULE_AUTHOR("Liam Girdwood");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
