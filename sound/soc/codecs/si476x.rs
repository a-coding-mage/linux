// SPDX-License-Identifier: GPL-2.0-only
/*
 * sound/soc/codecs/si476x.c -- Codec driver for SI476X chips
 *
 * Copyright (C) 2012 Innovative Converged Devices(ICD)
 * Copyright (C) 2013 Andrey Smirnov
 *
 * Author: Andrey Smirnov <andrew.smirnov@gmail.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u16 = u16;

const EINVAL: c_int = 22;

const SI476X_DIGITAL_IO_OUTPUT_FORMAT: c_uint = 0x0203;
const SI476X_DIGITAL_IO_OUTPUT_SAMPLE_RATE: c_uint = 0x0202;

const SI476X_DIGITAL_IO_SLOT_SIZE_SHIFT: c_uint = 11;
const SI476X_DIGITAL_IO_SAMPLE_SIZE_SHIFT: c_uint = 8;

const SI476X_DIGITAL_IO_OUTPUT_WIDTH_MASK: c_uint =
    (0x7 << SI476X_DIGITAL_IO_SLOT_SIZE_SHIFT) | (0x7 << SI476X_DIGITAL_IO_SAMPLE_SIZE_SHIFT);
const SI476X_DIGITAL_IO_OUTPUT_FORMAT_MASK: c_uint = 0x7e;

const SI476X_DAUDIO_MODE_I2S: c_uint = 0x0 << 1;
const SI476X_DAUDIO_MODE_DSP_A: c_uint = 0x6 << 1;
const SI476X_DAUDIO_MODE_DSP_B: c_uint = 0x7 << 1;
const SI476X_DAUDIO_MODE_LEFT_J: c_uint = 0x8 << 1;
const SI476X_DAUDIO_MODE_RIGHT_J: c_uint = 0x9 << 1;

const SI476X_DAUDIO_MODE_IB: c_uint = 1 << 5;
const SI476X_DAUDIO_MODE_IF: c_uint = 1 << 6;

const SI476X_PCM_FORMAT_S8: c_int = 2;
const SI476X_PCM_FORMAT_S16_LE: c_int = 4;
const SI476X_PCM_FORMAT_S20_3LE: c_int = 5;
const SI476X_PCM_FORMAT_S24_LE: c_int = 6;

extern "C" {
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;

    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S8: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;

    fn i2c_mfd_cell_to_core(dev: *mut device) -> *mut si476x_core;
    fn si476x_core_lock(core: *mut si476x_core);
    fn si476x_core_unlock(core: *mut si476x_core);
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_write(
        component: *mut snd_soc_component,
        reg: c_uint,
        val: c_int,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

#[repr(C)]
struct device {
    parent: *mut device,
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct si476x_core {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
    component: *mut snd_soc_component,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    id: c_int,
    name: *const c_char,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
    set_fmt: Option<unsafe extern "C" fn(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> c_int>,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    idle_bias_on: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct driver {
    name: *const c_char,
}

#[repr(C)]
struct platform_driver {
    driver: driver,
    probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
}

const fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget { id: 0, name }
}

static si476x_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_OUTPUT(c"LOUT".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"ROUT".as_ptr()),
];

static si476x_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"LOUT".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"ROUT".as_ptr(),
    },
];

unsafe extern "C" fn si476x_codec_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let core = i2c_mfd_cell_to_core((*codec_dai).dev);
    let mut err: c_int;
    let mut format: u16 = 0;

    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC {
        return -EINVAL;
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_DSP_A => {
            format |= SI476X_DAUDIO_MODE_DSP_A as u16;
        }
        x if x == SND_SOC_DAIFMT_DSP_B => {
            format |= SI476X_DAUDIO_MODE_DSP_B as u16;
        }
        x if x == SND_SOC_DAIFMT_I2S => {
            format |= SI476X_DAUDIO_MODE_I2S as u16;
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {
            format |= SI476X_DAUDIO_MODE_RIGHT_J as u16;
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            format |= SI476X_DAUDIO_MODE_LEFT_J as u16;
        }
        _ => {
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_DSP_A || x == SND_SOC_DAIFMT_DSP_B => {
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                x if x == SND_SOC_DAIFMT_NB_NF => {}
                x if x == SND_SOC_DAIFMT_IB_NF => {
                    format |= SI476X_DAUDIO_MODE_IB as u16;
                }
                _ => {
                    return -EINVAL;
                }
            }
        }
        x if x == SND_SOC_DAIFMT_I2S
            || x == SND_SOC_DAIFMT_RIGHT_J
            || x == SND_SOC_DAIFMT_LEFT_J =>
        {
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                x if x == SND_SOC_DAIFMT_NB_NF => {}
                x if x == SND_SOC_DAIFMT_IB_IF => {
                    format |= (SI476X_DAUDIO_MODE_IB | SI476X_DAUDIO_MODE_IF) as u16;
                }
                x if x == SND_SOC_DAIFMT_IB_NF => {
                    format |= SI476X_DAUDIO_MODE_IB as u16;
                }
                x if x == SND_SOC_DAIFMT_NB_IF => {
                    format |= SI476X_DAUDIO_MODE_IF as u16;
                }
                _ => {
                    return -EINVAL;
                }
            }
        }
        _ => {
            return -EINVAL;
        }
    }

    si476x_core_lock(core);

    err = snd_soc_component_update_bits(
        (*codec_dai).component,
        SI476X_DIGITAL_IO_OUTPUT_FORMAT,
        SI476X_DIGITAL_IO_OUTPUT_FORMAT_MASK,
        format as c_uint,
    );

    si476x_core_unlock(core);

    if err < 0 {
        dev_err(
            (*(*codec_dai).component).dev,
            c"Failed to set output format\n".as_ptr(),
        );
        return err;
    }

    0
}

unsafe extern "C" fn si476x_codec_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let core = i2c_mfd_cell_to_core((*dai).dev);
    let rate: c_int;
    let width: c_int;
    let mut err: c_int;

    rate = params_rate(params);
    if rate < 32000 || rate > 48000 {
        dev_err(
            (*(*dai).component).dev,
            c"Rate: %d is not supported\n".as_ptr(),
            rate,
        );
        return -EINVAL;
    }

    match params_width(params) {
        8 => {
            width = SI476X_PCM_FORMAT_S8;
        }
        16 => {
            width = SI476X_PCM_FORMAT_S16_LE;
        }
        20 => {
            width = SI476X_PCM_FORMAT_S20_3LE;
        }
        24 => {
            width = SI476X_PCM_FORMAT_S24_LE;
        }
        _ => {
            return -EINVAL;
        }
    }

    si476x_core_lock(core);

    err = snd_soc_component_write((*dai).component, SI476X_DIGITAL_IO_OUTPUT_SAMPLE_RATE, rate);
    if err < 0 {
        dev_err(
            (*(*dai).component).dev,
            c"Failed to set sample rate\n".as_ptr(),
        );
        si476x_core_unlock(core);
        return err;
    }

    err = snd_soc_component_update_bits(
        (*dai).component,
        SI476X_DIGITAL_IO_OUTPUT_FORMAT,
        SI476X_DIGITAL_IO_OUTPUT_WIDTH_MASK,
        ((width as c_uint) << SI476X_DIGITAL_IO_SLOT_SIZE_SHIFT)
            | ((width as c_uint) << SI476X_DIGITAL_IO_SAMPLE_SIZE_SHIFT),
    );
    if err < 0 {
        dev_err(
            (*(*dai).component).dev,
            c"Failed to set output width\n".as_ptr(),
        );
        si476x_core_unlock(core);
        return err;
    }

    si476x_core_unlock(core);

    err
}

static si476x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(si476x_codec_hw_params),
    set_fmt: Some(si476x_codec_set_dai_fmt),
};

static mut si476x_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"si476x-codec".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,

        rates: unsafe { SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 },
        formats: unsafe {
            SNDRV_PCM_FMTBIT_S8
                | SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S20_3LE
                | SNDRV_PCM_FMTBIT_S24_LE
        },
    },
    ops: &si476x_dai_ops,
};

unsafe extern "C" fn si476x_probe(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_init_regmap(
        component,
        dev_get_regmap((*(*component).dev).parent, core::ptr::null()),
    );

    0
}

static soc_component_dev_si476x: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(si476x_probe),
    dapm_widgets: si476x_dapm_widgets.as_ptr(),
    num_dapm_widgets: si476x_dapm_widgets.len() as c_uint,
    dapm_routes: si476x_dapm_routes.as_ptr(),
    num_dapm_routes: si476x_dapm_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn si476x_platform_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_si476x,
        &mut si476x_dai,
        1,
    )
}

// MODULE_ALIAS("platform:si476x-codec");

static mut si476x_platform_driver: platform_driver = platform_driver {
    driver: driver {
        name: c"si476x-codec".as_ptr(),
    },
    probe: Some(si476x_platform_probe),
};

// module_platform_driver(si476x_platform_driver);

// MODULE_AUTHOR("Andrey Smirnov <andrew.smirnov@gmail.com>");
// MODULE_DESCRIPTION("ASoC Si4761/64 codec driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
