// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2010-2011,2013-2015 The Linux Foundation. All rights reserved.
 *
 * max98357a.c -- MAX98357A ALSA SoC Codec driver
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{addr_of, null};

type u32 = c_uint;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
    pub id: c_int,
    pub name: *const c_char,
    pub sname: *const c_char,
    pub reg: c_int,
    pub shift: c_uchar,
    pub mask: c_uint,
    pub on_val: c_uint,
    pub off_val: c_uint,
    pub event: Option<
        unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int,
    >,
    pub event_flags: c_uchar,
}

type c_uchar = u8;

#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: *const c_char,
    pub driver_data: c_ulong,
}

#[repr(C)]
struct max98357a_priv {
    sdmode: *mut gpio_desc,
    sdmode_delay: c_uint,
    sdmode_switch: c_int,
}

const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;

const SND_SOC_DAPM_POST_PMU: c_int = 0x10;
const SND_SOC_DAPM_POST_PMD: c_int = 0x20;
const SND_SOC_NOPM: c_int = -1;

const SNDRV_PCM_FMTBIT_S16: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S32: u64 = 1 << 10;

const SNDRV_PCM_RATE_8000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 7;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 8;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 10;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 11;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 12;

const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_int = 0;
const ENOMEM: c_int = 12;

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn mdelay(msecs: c_uint);
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut u32)
        -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

unsafe extern "C" fn max98357a_daiops_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let component: *mut snd_soc_component = (*dai).component;
        let max98357a: *mut max98357a_priv =
            snd_soc_component_get_drvdata(component) as *mut max98357a_priv;

        if (*max98357a).sdmode.is_null() {
            return 0;
        }

        match cmd {
            SNDRV_PCM_TRIGGER_START
            | SNDRV_PCM_TRIGGER_RESUME
            | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                mdelay((*max98357a).sdmode_delay);
                if (*max98357a).sdmode_switch != 0 {
                    gpiod_set_value((*max98357a).sdmode, 1);
                    dev_dbg((*component).dev, c"set sdmode to 1".as_ptr());
                }
            }
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                gpiod_set_value((*max98357a).sdmode, 0);
                dev_dbg((*component).dev, c"set sdmode to 0".as_ptr());
            }
            _ => {}
        }

        0
    }
}

unsafe extern "C" fn max98357a_sdmode_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    unsafe {
        let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
        let max98357a: *mut max98357a_priv =
            snd_soc_component_get_drvdata(component) as *mut max98357a_priv;

        if event & SND_SOC_DAPM_POST_PMU != 0 {
            (*max98357a).sdmode_switch = 1;
        } else if event & SND_SOC_DAPM_POST_PMD != 0 {
            (*max98357a).sdmode_switch = 0;
        }

        0
    }
}

static max98357a_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget {
        dapm: core::ptr::null_mut(),
        id: 0,
        name: c"Speaker".as_ptr(),
        sname: null(),
        reg: SND_SOC_NOPM,
        shift: 0,
        mask: 0,
        on_val: 0,
        off_val: 0,
        event: None,
        event_flags: 0,
    },
    snd_soc_dapm_widget {
        dapm: core::ptr::null_mut(),
        id: 0,
        name: c"SD_MODE".as_ptr(),
        sname: null(),
        reg: SND_SOC_NOPM,
        shift: 0,
        mask: 0,
        on_val: 0,
        off_val: 0,
        event: Some(max98357a_sdmode_event),
        event_flags: (SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD) as c_uchar,
    },
];

static max98357a_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"SD_MODE".as_ptr(),
        control: null(),
        source: c"HiFi Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: null(),
        source: c"SD_MODE".as_ptr(),
    },
];

static max98357a_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: max98357a_dapm_widgets.as_ptr(),
    num_dapm_widgets: max98357a_dapm_widgets.len() as c_uint,
    dapm_routes: max98357a_dapm_routes.as_ptr(),
    num_dapm_routes: max98357a_dapm_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static max98357a_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    trigger: Some(max98357a_daiops_trigger),
};

static mut max98357a_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"HiFi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"HiFi Playback".as_ptr(),
        formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
        rates: SNDRV_PCM_RATE_8000
            | SNDRV_PCM_RATE_16000
            | SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_88200
            | SNDRV_PCM_RATE_96000,
        rate_min: 8000,
        rate_max: 96000,
        channels_min: 1,
        channels_max: 2,
    },
    ops: addr_of!(max98357a_dai_ops),
};

unsafe extern "C" fn max98357a_platform_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        let max98357a: *mut max98357a_priv;
        let ret: c_int;

        max98357a = devm_kzalloc(
            addr_of!((*pdev).dev) as *mut device,
            size_of::<max98357a_priv>(),
            GFP_KERNEL,
        ) as *mut max98357a_priv;
        if max98357a.is_null() {
            return -ENOMEM;
        }

        (*max98357a).sdmode = devm_gpiod_get_optional(
            addr_of!((*pdev).dev) as *mut device,
            c"sdmode".as_ptr(),
            GPIOD_OUT_LOW,
        );
        if IS_ERR((*max98357a).sdmode as *const c_void) != 0 {
            return PTR_ERR((*max98357a).sdmode as *const c_void);
        }

        ret = device_property_read_u32(
            addr_of!((*pdev).dev) as *mut device,
            c"sdmode-delay".as_ptr(),
            addr_of!((*max98357a).sdmode_delay) as *mut u32,
        );
        if ret != 0 {
            (*max98357a).sdmode_delay = 0;
            dev_dbg(
                addr_of!((*pdev).dev) as *mut device,
                c"no optional property 'sdmode-delay' found, default: no delay\n".as_ptr(),
            );
        }

        dev_set_drvdata(
            addr_of!((*pdev).dev) as *mut device,
            max98357a as *mut c_void,
        );

        devm_snd_soc_register_component(
            addr_of!((*pdev).dev) as *mut device,
            addr_of!(max98357a_component_driver),
            addr_of!(max98357a_dai_driver),
            1,
        )
    }
}

// Original C condition: #ifdef CONFIG_OF
static max98357a_device_id: [of_device_id; 3] = [
    of_device_id {
        compatible: c"maxim,max98357a".as_ptr(),
    },
    of_device_id {
        compatible: c"maxim,max98360a".as_ptr(),
    },
    of_device_id { compatible: null() },
];
// MODULE_DEVICE_TABLE(of, max98357a_device_id);

// Original C condition: #ifdef CONFIG_ACPI
static max98357a_acpi_match: [acpi_device_id; 3] = [
    acpi_device_id {
        id: c"MX98357A".as_ptr(),
        driver_data: 0,
    },
    acpi_device_id {
        id: c"MX98360A".as_ptr(),
        driver_data: 0,
    },
    acpi_device_id {
        id: null(),
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, max98357a_acpi_match);

static mut max98357a_platform_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: c"max98357a".as_ptr(),
        of_match_table: max98357a_device_id.as_ptr(),
        acpi_match_table: max98357a_acpi_match.as_ptr(),
    },
    probe: Some(max98357a_platform_probe),
};
// module_platform_driver(max98357a_platform_driver);

// MODULE_DESCRIPTION("Maxim MAX98357A Codec Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
