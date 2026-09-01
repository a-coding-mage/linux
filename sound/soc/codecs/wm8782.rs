// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sound/soc/codecs/wm8782.c
 * simple, strap-pin configured 24bit 2ch ADC
 *
 * Copyright: 2011 Raumfeld GmbH
 * Author: Johannes Stezenbach <js@sig21.net>
 *
 * based on ad73311.c
 * Copyright:	Analog Devices Inc.
 * Author:	Cliff Cai <cliff.cai@analog.com>
 */

// C includes translated as external dependency intent:
// linux/init.h, linux/slab.h, linux/module.h, linux/kernel.h,
// linux/device.h, linux/regulator/consumer.h, sound/core.h, sound/pcm.h,
// sound/ac97_codec.h, sound/initval.h, sound/soc.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

extern "C" {
    static wm8782_of_match: [of_device_id; 0];

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_uint,
        min: c_uint,
        max: c_uint,
    ) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_int,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn of_property_read_u32(
        np: *mut device_node,
        propname: *const c_char,
        out_value: *mut c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_uint = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 1 << 2;

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
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
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
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
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

const fn snd_soc_dapm_input(name: *const c_char) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget { id: 0, name }
}

const fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id {
    ptr
}

/* regulator power supply names */
static supply_names: [*const c_char; 2] = [
    b"Vdda\0".as_ptr() as *const c_char, /* analog supply, 2.7V - 3.6V */
    b"Vdd\0".as_ptr() as *const c_char,  /* digital supply, 2.7V - 5.5V */
];

#[repr(C)]
struct wm8782_priv {
    supplies: [regulator_bulk_data; supply_names.len()],
    max_rate: c_int,
}

unsafe extern "C" fn wm8782_dai_startup(
    sub: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*sub).runtime;
    let priv_: *mut wm8782_priv =
        snd_soc_component_get_drvdata((*dai).component) as *mut wm8782_priv;

    snd_pcm_hw_constraint_minmax(
        runtime,
        SNDRV_PCM_HW_PARAM_RATE,
        8000,
        (*priv_).max_rate as c_uint,
    )
}

static wm8782_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_input(b"AINL\0".as_ptr() as *const c_char),
    snd_soc_dapm_input(b"AINR\0".as_ptr() as *const c_char),
];

static wm8782_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"Capture\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"AINL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Capture\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"AINR\0".as_ptr() as *const c_char,
    },
];

static wm8782_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(wm8782_dai_startup),
};

static mut wm8782_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"wm8782\0".as_ptr() as *const c_char,
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &wm8782_dai_ops,
};

unsafe extern "C" fn wm8782_soc_probe(component: *mut snd_soc_component) -> c_int {
    let priv_: *mut wm8782_priv = snd_soc_component_get_drvdata(component) as *mut wm8782_priv;
    regulator_bulk_enable((*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr())
}

unsafe extern "C" fn wm8782_soc_remove(component: *mut snd_soc_component) {
    let priv_: *mut wm8782_priv = snd_soc_component_get_drvdata(component) as *mut wm8782_priv;
    regulator_bulk_disable((*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr());
}

// CONFIG_PM: when enabled, these callbacks suspend and resume the regulators;
// otherwise the C preprocessor defines both callback symbols as NULL.
#[cfg(CONFIG_PM)]
unsafe extern "C" fn wm8782_soc_suspend(component: *mut snd_soc_component) -> c_int {
    let priv_: *mut wm8782_priv = snd_soc_component_get_drvdata(component) as *mut wm8782_priv;
    regulator_bulk_disable((*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr());
    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn wm8782_soc_resume(component: *mut snd_soc_component) -> c_int {
    let priv_: *mut wm8782_priv = snd_soc_component_get_drvdata(component) as *mut wm8782_priv;
    regulator_bulk_enable((*priv_).supplies.len() as c_int, (*priv_).supplies.as_mut_ptr())
}

static soc_component_dev_wm8782: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8782_soc_probe),
    remove: Some(wm8782_soc_remove),
    suspend: {
        #[cfg(CONFIG_PM)]
        {
            Some(wm8782_soc_suspend)
        }
        #[cfg(not(CONFIG_PM))]
        {
            None
        }
    },
    resume: {
        #[cfg(CONFIG_PM)]
        {
            Some(wm8782_soc_resume)
        }
        #[cfg(not(CONFIG_PM))]
        {
            None
        }
    },
    dapm_widgets: wm8782_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8782_dapm_widgets.len() as c_uint,
    dapm_routes: wm8782_dapm_routes.as_ptr(),
    num_dapm_routes: wm8782_dapm_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn wm8782_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let np: *mut device_node = (*dev).of_node;
    let mut ret: c_int;
    let mut i: c_int;
    let mut fsampen: c_int;

    let priv_: *mut wm8782_priv =
        devm_kzalloc(dev, core::mem::size_of::<wm8782_priv>(), GFP_KERNEL) as *mut wm8782_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, priv_ as *mut c_void);

    i = 0;
    while i < supply_names.len() as c_int {
        (*priv_).supplies[i as usize].supply = supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        dev,
        (*priv_).supplies.len() as c_int,
        (*priv_).supplies.as_mut_ptr(),
    );
    if ret < 0 {
        return ret;
    }

    // Assume lowest value by default to avoid inadvertent overclocking
    fsampen = 0;

    if !np.is_null() {
        of_property_read_u32(np, b"wlf,fsampen\0".as_ptr() as *const c_char, &mut fsampen);
    }

    match fsampen {
        0 => {
            (*priv_).max_rate = 48000;
        }
        1 => {
            (*priv_).max_rate = 96000;
        }
        2 => {
            (*priv_).max_rate = 192000;
        }
        _ => {
            dev_err(dev, b"Invalid wlf,fsampen value\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_wm8782,
        &mut wm8782_dai,
        1,
    )
}

// CONFIG_OF: C defines wm8782_of_match only when CONFIG_OF is enabled and
// exports MODULE_DEVICE_TABLE(of, wm8782_of_match).
#[cfg(CONFIG_OF)]
static wm8782_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"wlf,wm8782\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

static mut wm8782_codec_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"wm8782\0".as_ptr() as *const c_char,
        of_match_table: unsafe { of_match_ptr(wm8782_of_match.as_ptr()) },
    },
    probe: Some(wm8782_probe),
};

// module_platform_driver(wm8782_codec_driver);
// MODULE_DESCRIPTION("ASoC WM8782 driver");
// MODULE_AUTHOR("Johannes Stezenbach <js@sig21.net>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
