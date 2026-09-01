// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8776.c  --  WM8776 ALSA SoC Audio driver
 *
 * Copyright 2009-12 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 *
 * TODO: Input ALC/limiter support
 */

// Rust translation of the C implementation source. C include dependencies are
// expected to be supplied by the surrounding kernel/ASoC bindings.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

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
pub struct snd_soc_dai_driver;

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub driver: *mut snd_soc_dai_driver_id_view,
}

#[repr(C)]
pub struct snd_soc_dai_driver_id_view {
    pub id: c_int,
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

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
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
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub no_capture_mute: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver_full {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: usize,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
    pub suspend_bias_off: c_int,
    pub idle_bias_on: c_int,
    pub use_pmdown_time: c_int,
    pub endianness: c_int,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: usize,
    pub cache_type: c_int,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: wm8776_chip_type,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum wm8776_chip_type {
    WM8775 = 1,
    WM8776,
}

/* codec private data */
#[repr(C)]
pub struct wm8776_priv {
    pub regmap: *mut regmap,
    pub sysclk: [c_int; 2],
}

extern "C" {
    static WM8776_RESET: c_uint;
    static WM8776_HPLVOL: c_uint;
    static WM8776_HPRVOL: c_uint;
    static WM8776_DACLVOL: c_uint;
    static WM8776_DACRVOL: c_uint;
    static WM8776_DACCTRL1: c_uint;
    static WM8776_DACCTRL2: c_uint;
    static WM8776_ADCLVOL: c_uint;
    static WM8776_ADCRVOL: c_uint;
    static WM8776_ADCMUX: c_uint;
    static WM8776_ADCIFCTRL: c_uint;
    static WM8776_OUTMUX: c_uint;
    static WM8776_PWRDOWN: c_uint;
    static WM8776_DACIFCTRL: c_uint;
    static WM8776_MSTRCTRL: c_uint;
    static WM8776_DACMUTE: c_uint;

    static WM8776_DAI_DAC: c_int;
    static WM8776_DAI_ADC: c_int;

    static SND_SOC_NOPM: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static REGCACHE_MAPLE: c_int;
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;
    static EINVAL: c_int;

    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn spi_set_drvdata(spi: *mut spi_device, data: *mut c_void);
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver_full,
        num_dai: c_int,
    ) -> c_int;
    fn i2c_add_driver(driver: *mut i2c_driver) -> c_int;
    fn i2c_del_driver(driver: *mut i2c_driver);
    fn spi_register_driver(driver: *mut spi_driver) -> c_int;
    fn spi_unregister_driver(driver: *mut spi_driver);
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn WARN_ON(condition: bool) -> bool;
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

static wm8776_reg_defaults: [reg_default; 23] = [
    reg_default { reg: 0, def: 0x79 },
    reg_default { reg: 1, def: 0x79 },
    reg_default { reg: 2, def: 0x79 },
    reg_default { reg: 3, def: 0xff },
    reg_default { reg: 4, def: 0xff },
    reg_default { reg: 5, def: 0xff },
    reg_default { reg: 6, def: 0x00 },
    reg_default { reg: 7, def: 0x90 },
    reg_default { reg: 8, def: 0x00 },
    reg_default { reg: 9, def: 0x00 },
    reg_default { reg: 10, def: 0x22 },
    reg_default { reg: 11, def: 0x22 },
    reg_default { reg: 12, def: 0x22 },
    reg_default { reg: 13, def: 0x08 },
    reg_default { reg: 14, def: 0xcf },
    reg_default { reg: 15, def: 0xcf },
    reg_default { reg: 16, def: 0x7b },
    reg_default { reg: 17, def: 0x00 },
    reg_default { reg: 18, def: 0x32 },
    reg_default { reg: 19, def: 0x00 },
    reg_default { reg: 20, def: 0xa6 },
    reg_default { reg: 21, def: 0x01 },
    reg_default { reg: 22, def: 0x01 },
];

unsafe extern "C" fn wm8776_volatile(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == WM8776_RESET => true,
        _ => false,
    }
}

unsafe extern "C" fn wm8776_reset(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_write(component, WM8776_RESET, 0)
}

static hp_tlv: [c_uint; 4] = [0, (-12100i32) as c_uint, 100, 1];
static dac_tlv: [c_uint; 4] = [0, (-12750i32) as c_uint, 50, 1];
static adc_tlv: [c_uint; 4] = [0, (-10350i32) as c_uint, 50, 1];

/* The following control/widget arrays are direct translations of ASoC macro
 * initializers; concrete field layouts are supplied by external bindings. */
extern "C" {
    static wm8776_snd_controls: [snd_kcontrol_new; 8];
    static inmix_controls: [snd_kcontrol_new; 5];
    static outmix_controls: [snd_kcontrol_new; 3];
    static wm8776_dapm_widgets: [snd_soc_dapm_widget; 14];
}

static routes: [snd_soc_dapm_route; 14] = [
    snd_soc_dapm_route { sink: c_str!("Input Mixer"), control: c_str!("AIN1 Switch"), source: c_str!("AIN1") },
    snd_soc_dapm_route { sink: c_str!("Input Mixer"), control: c_str!("AIN2 Switch"), source: c_str!("AIN2") },
    snd_soc_dapm_route { sink: c_str!("Input Mixer"), control: c_str!("AIN3 Switch"), source: c_str!("AIN3") },
    snd_soc_dapm_route { sink: c_str!("Input Mixer"), control: c_str!("AIN4 Switch"), source: c_str!("AIN4") },
    snd_soc_dapm_route { sink: c_str!("Input Mixer"), control: c_str!("AIN5 Switch"), source: c_str!("AIN5") },
    snd_soc_dapm_route { sink: c_str!("ADC"), control: ptr::null(), source: c_str!("Input Mixer") },
    snd_soc_dapm_route { sink: c_str!("Output Mixer"), control: c_str!("DAC Switch"), source: c_str!("DAC") },
    snd_soc_dapm_route { sink: c_str!("Output Mixer"), control: c_str!("AUX Switch"), source: c_str!("AUX") },
    snd_soc_dapm_route { sink: c_str!("Output Mixer"), control: c_str!("Bypass Switch"), source: c_str!("Input Mixer") },
    snd_soc_dapm_route { sink: c_str!("VOUT"), control: ptr::null(), source: c_str!("Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("Headphone PGA"), control: ptr::null(), source: c_str!("Output Mixer") },
    snd_soc_dapm_route { sink: c_str!("HPOUTL"), control: ptr::null(), source: c_str!("Headphone PGA") },
    snd_soc_dapm_route { sink: c_str!("HPOUTR"), control: ptr::null(), source: c_str!("Headphone PGA") },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
];

unsafe extern "C" fn wm8776_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let reg: c_uint;
    let mut iface: c_int;
    let mut master: c_uint;

    match (*(*dai).driver).id {
        x if x == WM8776_DAI_DAC => {
            reg = WM8776_DACIFCTRL;
            master = 0x80;
        }
        x if x == WM8776_DAI_ADC => {
            reg = WM8776_ADCIFCTRL;
            master = 0x100;
        }
        _ => return -EINVAL,
    }

    iface = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {}
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            master = 0;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            iface |= 0x0002;
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {}
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            iface |= 0x0001;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_IB_IF => {
            iface |= 0x00c;
        }
        x if x == SND_SOC_DAIFMT_IB_NF => {
            iface |= 0x008;
        }
        x if x == SND_SOC_DAIFMT_NB_IF => {
            iface |= 0x004;
        }
        _ => return -EINVAL,
    }

    /* Finally, write out the values */
    snd_soc_component_update_bits(component, reg, 0xf, iface as c_uint);
    snd_soc_component_update_bits(component, WM8776_MSTRCTRL, 0x180, master);

    0
}

static mclk_ratios: [c_int; 6] = [128, 192, 256, 384, 512, 768];

unsafe extern "C" fn wm8776_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let wm8776 = snd_soc_component_get_drvdata(component) as *mut wm8776_priv;
    let iface_reg: c_uint;
    let iface: c_int;
    let ratio_shift: c_int;
    let master: c_uint;
    let mut i: usize;

    match (*(*dai).driver).id {
        x if x == WM8776_DAI_DAC => {
            iface_reg = WM8776_DACIFCTRL;
            master = 0x80;
            ratio_shift = 4;
        }
        x if x == WM8776_DAI_ADC => {
            iface_reg = WM8776_ADCIFCTRL;
            master = 0x100;
            ratio_shift = 0;
        }
        _ => return -EINVAL,
    }

    /* Set word length */
    match params_width(params) {
        16 => iface = 0,
        20 => iface = 0x10,
        24 => iface = 0x20,
        32 => iface = 0x30,
        _ => {
            dev_err(
                (*component).dev,
                c_str!("Unsupported sample size: %i\n"),
                params_width(params),
            );
            return -EINVAL;
        }
    }

    /* Only need to set MCLK/LRCLK ratio if we're master */
    if (snd_soc_component_read(component, WM8776_MSTRCTRL) & master) != 0 {
        i = 0;
        while i < array_size(&mclk_ratios) {
            if (*wm8776).sysclk[(*(*dai).driver).id as usize] / params_rate(params) == mclk_ratios[i] {
                break;
            }
            i += 1;
        }

        if i == array_size(&mclk_ratios) {
            dev_err(
                (*component).dev,
                c_str!("Unable to configure MCLK ratio %d/%d\n"),
                (*wm8776).sysclk[(*(*dai).driver).id as usize],
                params_rate(params),
            );
            return -EINVAL;
        }

        dev_dbg((*component).dev, c_str!("MCLK is %dfs\n"), mclk_ratios[i]);

        snd_soc_component_update_bits(
            component,
            WM8776_MSTRCTRL,
            (0x7 << ratio_shift) as c_uint,
            ((i as c_int) << ratio_shift) as c_uint,
        );
    } else {
        dev_dbg((*component).dev, c_str!("DAI in slave mode\n"));
    }

    snd_soc_component_update_bits(component, iface_reg, 0x30, iface as c_uint);

    0
}

unsafe extern "C" fn wm8776_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*dai).component;

    snd_soc_component_write(component, WM8776_DACMUTE, (mute != 0) as c_uint)
}

unsafe extern "C" fn wm8776_set_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*dai).component;
    let wm8776 = snd_soc_component_get_drvdata(component) as *mut wm8776_priv;

    if WARN_ON((*(*dai).driver).id as usize >= array_size(&(*wm8776).sysclk)) {
        return -EINVAL;
    }

    (*wm8776).sysclk[(*(*dai).driver).id as usize] = freq as c_int;

    0
}

unsafe extern "C" fn wm8776_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let wm8776 = snd_soc_component_get_drvdata(component) as *mut wm8776_priv;
    let dapm = snd_soc_component_to_dapm(component);

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_OFF {
                regcache_sync((*wm8776).regmap);

                /* Disable the global powerdown; DAPM does the rest */
                snd_soc_component_update_bits(component, WM8776_PWRDOWN, 1, 0);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(component, WM8776_PWRDOWN, 1, 1);
        }
    }

    0
}

fn WM8776_FORMATS() -> u64 {
    unsafe {
        SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE
    }
}

static wm8776_dac_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    mute_stream: Some(wm8776_mute),
    hw_params: Some(wm8776_hw_params),
    set_fmt: Some(wm8776_set_fmt),
    set_sysclk: Some(wm8776_set_sysclk),
    no_capture_mute: 1,
};

static wm8776_adc_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    mute_stream: None,
    hw_params: Some(wm8776_hw_params),
    set_fmt: Some(wm8776_set_fmt),
    set_sysclk: Some(wm8776_set_sysclk),
    no_capture_mute: 0,
};

static mut wm8776_dai: [snd_soc_dai_driver_full; 2] = [
    snd_soc_dai_driver_full {
        name: c_str!("wm8776-hifi-playback"),
        id: unsafe { WM8776_DAI_DAC },
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("Playback"),
            channels_min: 2,
            channels_max: 2,
            rates: unsafe { SNDRV_PCM_RATE_CONTINUOUS },
            rate_min: 32000,
            rate_max: 192000,
            formats: 0, /* WM8776_FORMATS() */
        },
        capture: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            rate_min: 0,
            rate_max: 0,
            formats: 0,
        },
        ops: &wm8776_dac_ops,
    },
    snd_soc_dai_driver_full {
        name: c_str!("wm8776-hifi-capture"),
        id: unsafe { WM8776_DAI_ADC },
        playback: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            rate_min: 0,
            rate_max: 0,
            formats: 0,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("Capture"),
            channels_min: 2,
            channels_max: 2,
            rates: unsafe { SNDRV_PCM_RATE_CONTINUOUS },
            rate_min: 32000,
            rate_max: 96000,
            formats: 0, /* WM8776_FORMATS() */
        },
        ops: &wm8776_adc_ops,
    },
];

unsafe extern "C" fn wm8776_probe(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int = 0;

    ret = wm8776_reset(component);
    if ret < 0 {
        dev_err((*component).dev, c_str!("Failed to issue reset: %d\n"), ret);
        return ret;
    }

    /* Latch the update bits; right channel only since we always
     * update both. */
    snd_soc_component_update_bits(component, WM8776_HPRVOL, 0x100, 0x100);
    snd_soc_component_update_bits(component, WM8776_DACRVOL, 0x100, 0x100);

    ret
}

static soc_component_dev_wm8776: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8776_probe),
    set_bias_level: Some(wm8776_set_bias_level),
    controls: unsafe { wm8776_snd_controls.as_ptr() },
    num_controls: 8,
    dapm_widgets: unsafe { wm8776_dapm_widgets.as_ptr() },
    num_dapm_widgets: 14,
    dapm_routes: routes.as_ptr(),
    num_dapm_routes: 14,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static wm8776_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c_str!("wlf,wm8776") },
    of_device_id { compatible: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, wm8776_of_match); */

static wm8776_regmap: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: unsafe { WM8776_RESET },
    reg_defaults: wm8776_reg_defaults.as_ptr(),
    num_reg_defaults: 23,
    cache_type: unsafe { REGCACHE_MAPLE },
    volatile_reg: Some(wm8776_volatile),
};

/* #if defined(CONFIG_SPI_MASTER) */
unsafe extern "C" fn wm8776_spi_probe(spi: *mut spi_device) -> c_int {
    let wm8776: *mut wm8776_priv;
    let ret: c_int;

    wm8776 = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<wm8776_priv>(), GFP_KERNEL)
        as *mut wm8776_priv;
    if wm8776.is_null() {
        return -ENOMEM;
    }

    (*wm8776).regmap = devm_regmap_init_spi(spi, &wm8776_regmap);
    if IS_ERR((*wm8776).regmap as *const c_void) {
        return PTR_ERR((*wm8776).regmap as *const c_void);
    }

    spi_set_drvdata(spi, wm8776 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &soc_component_dev_wm8776,
        wm8776_dai.as_mut_ptr(),
        array_size(&wm8776_dai) as c_int,
    );

    ret
}

static mut wm8776_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c_str!("wm8776"),
        of_match_table: wm8776_of_match.as_ptr(),
    },
    probe: Some(wm8776_spi_probe),
};
/* #endif CONFIG_SPI_MASTER */

/* #if IS_ENABLED(CONFIG_I2C) */
unsafe extern "C" fn wm8776_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm8776: *mut wm8776_priv;
    let ret: c_int;

    wm8776 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8776_priv>(), GFP_KERNEL)
        as *mut wm8776_priv;
    if wm8776.is_null() {
        return -ENOMEM;
    }

    (*wm8776).regmap = devm_regmap_init_i2c(i2c, &wm8776_regmap);
    if IS_ERR((*wm8776).regmap as *const c_void) {
        return PTR_ERR((*wm8776).regmap as *const c_void);
    }

    i2c_set_clientdata(i2c, wm8776 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm8776,
        wm8776_dai.as_mut_ptr(),
        array_size(&wm8776_dai) as c_int,
    );

    ret
}

static wm8776_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id { name: c_str!("wm8775"), driver_data: wm8776_chip_type::WM8775 },
    i2c_device_id { name: c_str!("wm8776"), driver_data: wm8776_chip_type::WM8776 },
    i2c_device_id { name: ptr::null(), driver_data: wm8776_chip_type::WM8775 },
];
/* MODULE_DEVICE_TABLE(i2c, wm8776_i2c_id); */

static mut wm8776_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c_str!("wm8776"),
        of_match_table: wm8776_of_match.as_ptr(),
    },
    probe: Some(wm8776_i2c_probe),
    id_table: wm8776_i2c_id.as_ptr(),
};
/* #endif */

unsafe extern "C" fn wm8776_modinit() -> c_int {
    let mut ret: c_int = 0;

    /* #if IS_ENABLED(CONFIG_I2C) */
    ret = i2c_add_driver(&mut wm8776_i2c_driver);
    if ret != 0 {
        printk(
            c_str!("Failed to register wm8776 I2C driver: %d\n"),
            ret,
        );
    }
    /* #endif */

    /* #if defined(CONFIG_SPI_MASTER) */
    ret = spi_register_driver(&mut wm8776_spi_driver);
    if ret != 0 {
        printk(
            c_str!("Failed to register wm8776 SPI driver: %d\n"),
            ret,
        );
    }
    /* #endif */

    ret
}
/* module_init(wm8776_modinit); */

unsafe extern "C" fn wm8776_exit() {
    /* #if IS_ENABLED(CONFIG_I2C) */
    i2c_del_driver(&mut wm8776_i2c_driver);
    /* #endif */

    /* #if defined(CONFIG_SPI_MASTER) */
    spi_unregister_driver(&mut wm8776_spi_driver);
    /* #endif */
}
/* module_exit(wm8776_exit); */

/* MODULE_DESCRIPTION("ASoC WM8776 driver"); */
/* MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
