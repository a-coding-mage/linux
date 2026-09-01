// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PCM1754 DAC ASoC codec driver
 *
 * Copyright (c) 2022 Alvin Sipraga <alsi@bang-olufsen.dk>
 * Copyright (c) 2025 Stefan Kerkmann <s.kerkmann@pengutronix.de>
 */

// Derived from:
// #include <linux/gpio/consumer.h>
// #include <linux/module.h>
// #include <linux/regulator/consumer.h>
// #include <sound/pcm_params.h>
// #include <sound/soc.h>

use core::ffi::{c_char, c_int, c_uint, c_void};

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_int = 1;
const GPIOD_OUT_LOW: c_int = 0;

extern "C" {
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_POSSIBLE_DAIFMT_I2S: u64;
    static SND_SOC_POSSIBLE_DAIFMT_RIGHT_J: u64;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SND_SOC_NOPM: c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
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
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
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
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
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
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

#[repr(C)]
pub struct pcm1754_priv {
    pub format: c_uint,
    pub gpiod_mute: *mut gpio_desc,
    pub gpiod_format: *mut gpio_desc,
}

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn devm_kmemdup(
        dev: *mut device,
        src: *const c_void,
        len: usize,
        gfp: c_uint,
    ) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char, ...) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn of_match_ptr(ids: *const of_device_id) -> *const of_device_id;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" fn pcm1754_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    format: c_uint,
) -> c_int {
    let component = (*codec_dai).component;
    let priv_0 = snd_soc_component_get_drvdata(component) as *mut pcm1754_priv;

    (*priv_0).format = format;

    0
}

unsafe extern "C" fn pcm1754_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    codec_dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*codec_dai).component;
    let priv_0 = snd_soc_component_get_drvdata(component) as *mut pcm1754_priv;
    let format: c_int;

    match (*priv_0).format & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_RIGHT_J => {
            match params_width(params) {
                16 => {
                    format = 1;
                }
                _ => {
                    return -EINVAL;
                }
            }
        }
        x if x == SND_SOC_DAIFMT_I2S => {
            match params_width(params) {
                16 | 24 => {
                    format = 0;
                }
                _ => {
                    return -EINVAL;
                }
            }
        }
        _ => {
            dev_err((*component).dev, c"Invalid DAI format\n".as_ptr());
            return -EINVAL;
        }
    }

    gpiod_set_value_cansleep((*priv_0).gpiod_format, format);

    0
}

unsafe extern "C" fn pcm1754_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _stream: c_int,
) -> c_int {
    let priv_0 = snd_soc_component_get_drvdata((*dai).component) as *mut pcm1754_priv;

    gpiod_set_value_cansleep((*priv_0).gpiod_mute, mute);

    0
}

static mut PCM1754_SELECTABLE_FORMATS: u64 = unsafe {
    SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J
};

static PCM1754_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(pcm1754_set_dai_fmt),
    hw_params: Some(pcm1754_hw_params),
    mute_stream: Some(pcm1754_mute_stream),
    auto_selectable_formats: unsafe { &raw const PCM1754_SELECTABLE_FORMATS },
    num_auto_selectable_formats: 1,
};

static PCM1754_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"pcm1754".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_CONTINUOUS },
        rate_min: 5000,
        rate_max: 200000,
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE },
    },
    ops: &PCM1754_DAI_OPS,
};

// static const struct snd_soc_dapm_widget pcm1754_dapm_widgets[] = {
//     SND_SOC_DAPM_REGULATOR_SUPPLY("VCC", 0, 0),
//     SND_SOC_DAPM_DAC("DAC1", "Channel 1 Playback", SND_SOC_NOPM, 0, 0),
//     SND_SOC_DAPM_DAC("DAC2", "Channel 2 Playback", SND_SOC_NOPM, 0, 0),
//     SND_SOC_DAPM_OUTPUT("VOUTL"),
//     SND_SOC_DAPM_OUTPUT("VOUTR"),
// };
extern "C" {
    static pcm1754_dapm_widgets: [snd_soc_dapm_widget; 5];
}

static PCM1754_DAPM_ROUTES: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route {
        sink: c"DAC1".as_ptr(),
        control: core::ptr::null(),
        source: c"Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DAC2".as_ptr(),
        control: core::ptr::null(),
        source: c"Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DAC1".as_ptr(),
        control: core::ptr::null(),
        source: c"VCC".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DAC2".as_ptr(),
        control: core::ptr::null(),
        source: c"VCC".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"VOUTL".as_ptr(),
        control: core::ptr::null(),
        source: c"DAC1".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"VOUTR".as_ptr(),
        control: core::ptr::null(),
        source: c"DAC2".as_ptr(),
    },
];

static SOC_COMPONENT_DEV_PCM1754: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: unsafe { pcm1754_dapm_widgets.as_ptr() },
    num_dapm_widgets: 5,
    dapm_routes: PCM1754_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: PCM1754_DAPM_ROUTES.len() as c_uint,
};

unsafe extern "C" fn pcm1754_probe(pdev: *mut platform_device) -> c_int {
    let priv_0: *mut pcm1754_priv;
    let dev = &mut (*pdev).dev as *mut device;
    let dai_drv: *mut snd_soc_dai_driver;
    let ret: c_int;

    dai_drv = devm_kmemdup(
        dev,
        &PCM1754_DAI as *const snd_soc_dai_driver as *const c_void,
        core::mem::size_of::<snd_soc_dai_driver>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_driver;
    if dai_drv.is_null() {
        return -ENOMEM;
    }

    priv_0 = devm_kzalloc(dev, core::mem::size_of::<pcm1754_priv>(), GFP_KERNEL)
        as *mut pcm1754_priv;
    if priv_0.is_null() {
        return -ENOMEM;
    }

    (*priv_0).gpiod_mute = devm_gpiod_get_optional(dev, c"mute".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*priv_0).gpiod_mute as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_0).gpiod_mute as *const c_void),
            c"failed to get mute gpio".as_ptr(),
        );
    }

    (*priv_0).gpiod_format = devm_gpiod_get_optional(dev, c"format".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*priv_0).gpiod_format as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_0).gpiod_format as *const c_void),
            c"failed to get format gpio".as_ptr(),
        );
    }

    dev_set_drvdata(dev, priv_0 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev as *mut device,
        &SOC_COMPONENT_DEV_PCM1754,
        dai_drv,
        1,
    );
    if ret != 0 {
        return dev_err_probe(dev, ret as isize, c"failed to register".as_ptr());
    }

    0
}

// #ifdef CONFIG_OF
static PCM1754_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: c"ti,pcm1754".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pcm1754_of_match);
// #endif

static mut PCM1754_CODEC_DRIVER: platform_driver = platform_driver {
    driver: driver {
        name: c"pcm1754-codec".as_ptr(),
        of_match_table: unsafe { of_match_ptr(PCM1754_OF_MATCH.as_ptr()) },
    },
    probe: Some(pcm1754_probe),
};

// module_platform_driver(pcm1754_codec_driver);

// MODULE_DESCRIPTION("ASoC PCM1754 driver");
// MODULE_AUTHOR("Alvin Sipraga <alsi@bang-olufsen.dk>");
// MODULE_AUTHOR("Stefan Kerkmann <s.kerkmann@pengutronix.de>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
