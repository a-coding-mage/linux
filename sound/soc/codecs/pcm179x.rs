// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PCM179X ASoC codec driver
 *
 * Copyright (c) Amarula Solutions B.V. 2013
 *
 *     Michael Trimarchi <michael@amarulasolutions.com>
 */

/* Rust translation of the implementation source.  Linux/ASoC types, helper
 * macros, and registration functions are supplied by surrounding bindings. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const PCM179X_DAC_VOL_LEFT: c_uint = 0x10;
const PCM179X_DAC_VOL_RIGHT: c_uint = 0x11;
const PCM179X_FMT_CONTROL: c_uint = 0x12;
const PCM179X_MODE_CONTROL: c_uint = 0x13;
const PCM179X_SOFT_MUTE: c_uint = PCM179X_FMT_CONTROL;

const PCM179X_FMT_MASK: c_uint = 0x70;
const PCM179X_FMT_SHIFT: c_uint = 4;
const PCM179X_MUTE_MASK: c_uint = 0x01;
const PCM179X_MUTE_SHIFT: c_uint = 0;
const PCM179X_ATLD_ENABLE: c_uint = 1 << 7;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

extern "C" {
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_POSSIBLE_DAIFMT_I2S: u64;
    static SND_SOC_POSSIBLE_DAIFMT_RIGHT_J: u64;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static PCM1792A_FORMATS: u64;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
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
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
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
pub struct pcm179x_private {
    pub regmap: *mut regmap,
    pub format: c_uint,
    pub rate: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_uint,
    pub no_capture_mute: c_uint,
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
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
}

#[repr(C)]
pub struct snd_soc_component_driver {
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

static PCM179X_REG_DEFAULTS: [reg_default; 8] = [
    reg_default { reg: 0x10, def: 0xff },
    reg_default { reg: 0x11, def: 0xff },
    reg_default { reg: 0x12, def: 0x50 },
    reg_default { reg: 0x13, def: 0x00 },
    reg_default { reg: 0x14, def: 0x00 },
    reg_default { reg: 0x15, def: 0x01 },
    reg_default { reg: 0x16, def: 0x00 },
    reg_default { reg: 0x17, def: 0x00 },
];

unsafe extern "C" fn pcm179x_accessible_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg >= 0x10 && reg <= 0x17
}

unsafe extern "C" fn pcm179x_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    let accessible: bool;

    accessible = pcm179x_accessible_reg(dev, reg);

    accessible && reg != 0x16 && reg != 0x17
}

unsafe extern "C" fn pcm179x_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    format: c_uint,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let priv_: *mut pcm179x_private =
        snd_soc_component_get_drvdata(component) as *mut pcm179x_private;

    (*priv_).format = format;

    0
}

unsafe extern "C" fn pcm179x_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let priv_: *mut pcm179x_private =
        snd_soc_component_get_drvdata(component) as *mut pcm179x_private;
    let ret: c_int;

    ret = regmap_update_bits(
        (*priv_).regmap,
        PCM179X_SOFT_MUTE,
        PCM179X_MUTE_MASK,
        (mute != 0) as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn pcm179x_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let priv_: *mut pcm179x_private =
        snd_soc_component_get_drvdata(component) as *mut pcm179x_private;
    let mut val: c_int = 0;
    let ret: c_int;

    (*priv_).rate = params_rate(params);

    match (*priv_).format & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_RIGHT_J => {
            match params_width(params) {
                24 | 32 => {
                    val = 2;
                }
                16 => {
                    val = 0;
                }
                _ => {
                    return -EINVAL;
                }
            }
        }
        x if x == SND_SOC_DAIFMT_I2S => {
            match params_width(params) {
                24 | 32 => {
                    val = 5;
                }
                16 => {
                    val = 4;
                }
                _ => {
                    return -EINVAL;
                }
            }
        }
        _ => {
            dev_err((*component).dev, b"Invalid DAI format\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    val = ((val as c_uint) << PCM179X_FMT_SHIFT | PCM179X_ATLD_ENABLE) as c_int;

    ret = regmap_update_bits(
        (*priv_).regmap,
        PCM179X_FMT_CONTROL,
        PCM179X_FMT_MASK | PCM179X_ATLD_ENABLE,
        val as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    0
}

static mut PCM179X_SELECTABLE_FORMATS: u64 = 0;

static PCM179X_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(pcm179x_set_dai_fmt),
    hw_params: Some(pcm179x_hw_params),
    mute_stream: Some(pcm179x_mute),
    auto_selectable_formats: unsafe { &PCM179X_SELECTABLE_FORMATS as *const u64 },
    num_auto_selectable_formats: 1,
    no_capture_mute: 1,
};

/* DECLARE_TLV_DB_SCALE(pcm179x_dac_tlv, -12000, 50, 1) */
static PCM179X_DAC_TLV: [c_uint; 4] = [
    0,
    2 * core::mem::size_of::<c_uint>() as c_uint,
    (-12000i32) as c_uint,
    50 | (1 << 16),
];

/* The following initializers are direct translations of ASoC control/widget
 * macros whose concrete struct layouts are provided externally in C. */
extern "C" {
    static PCM179X_CONTROLS: [snd_kcontrol_new; 3];
    static PCM179X_DAPM_WIDGETS: [snd_soc_dapm_widget; 4];
}

static PCM179X_DAPM_ROUTES: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: b"IOUTL+\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"IOUTL-\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"IOUTR+\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"IOUTR-\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Playback\0".as_ptr() as *const c_char,
    },
];

static mut PCM179X_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"pcm179x-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: 0,
        rate_min: 10000,
        rate_max: 200000,
        formats: 0,
    },
    ops: &PCM179X_DAI_OPS as *const snd_soc_dai_ops,
};

#[no_mangle]
pub static PCM179X_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: 23,
    reg_defaults: PCM179X_REG_DEFAULTS.as_ptr(),
    num_reg_defaults: PCM179X_REG_DEFAULTS.len() as c_uint,
    writeable_reg: Some(pcm179x_writeable_reg),
    readable_reg: Some(pcm179x_accessible_reg),
};

unsafe fn pcm179x_init_static_values() {
    PCM179X_SELECTABLE_FORMATS = SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J;
    PCM179X_DAI.playback.rates = SNDRV_PCM_RATE_CONTINUOUS;
    PCM179X_DAI.playback.formats = PCM1792A_FORMATS;
}

static SOC_COMPONENT_DEV_PCM179X: snd_soc_component_driver = snd_soc_component_driver {
    controls: unsafe { PCM179X_CONTROLS.as_ptr() },
    num_controls: 3,
    dapm_widgets: unsafe { PCM179X_DAPM_WIDGETS.as_ptr() },
    num_dapm_widgets: 4,
    dapm_routes: PCM179X_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: PCM179X_DAPM_ROUTES.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

#[no_mangle]
pub unsafe extern "C" fn pcm179x_common_init(
    dev: *mut device,
    regmap: *mut regmap,
) -> c_int {
    let pcm179x: *mut pcm179x_private;

    pcm179x_init_static_values();

    pcm179x = devm_kzalloc(dev, core::mem::size_of::<pcm179x_private>(), GFP_KERNEL)
        as *mut pcm179x_private;
    if pcm179x.is_null() {
        return -ENOMEM;
    }

    (*pcm179x).regmap = regmap;
    dev_set_drvdata(dev, pcm179x as *mut c_void);

    devm_snd_soc_register_component(
        dev,
        &SOC_COMPONENT_DEV_PCM179X as *const snd_soc_component_driver,
        &mut PCM179X_DAI as *mut snd_soc_dai_driver,
        1,
    )
}

/* EXPORT_SYMBOL_GPL(pcm179x_regmap_config); */
/* EXPORT_SYMBOL_GPL(pcm179x_common_init); */
/* MODULE_DESCRIPTION("ASoC PCM179X driver"); */
/* MODULE_AUTHOR("Michael Trimarchi <michael@amarulasolutions.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
