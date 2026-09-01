// SPDX-License-Identifier: GPL-2.0-only
//
// rt9123p.c -- RT9123 (HW Mode) ALSA SoC Codec driver
//
// Author: ChiYuan Huang <cy_huang@richtek.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

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
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

pub type snd_soc_dapm_event =
    unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int;

#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

pub type snd_soc_dai_trigger =
    unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int;

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub trigger: Option<snd_soc_dai_trigger>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub formats: c_ulong,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
struct rt9123p_priv {
    enable: *mut gpio_desc,
    enable_delay: c_uint,
    enable_switch: c_int,
}

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_int = 0;

const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;

const SND_SOC_DAPM_POST_PMU: c_int = 0x2;
const SND_SOC_DAPM_POST_PMD: c_int = 0x20;

const SNDRV_PCM_FMTBIT_S16: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_S24: c_ulong = 1 << 6;
const SNDRV_PCM_FMTBIT_S32: c_ulong = 1 << 10;

const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_22050: c_uint = 1 << 5;
const SNDRV_PCM_RATE_24000: c_uint = 1 << 6;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 7;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 8;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 9;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 10;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 11;

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn mdelay(msecs: c_uint);
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint)
        -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn module_platform_driver(driver: *mut platform_driver);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" fn rt9123p_daiops_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let comp: *mut snd_soc_component = (*dai).component;
    let rt9123p: *mut rt9123p_priv =
        snd_soc_component_get_drvdata(comp) as *mut rt9123p_priv;

    if (*rt9123p).enable.is_null() {
        return 0;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            mdelay((*rt9123p).enable_delay);
            if (*rt9123p).enable_switch != 0 {
                gpiod_set_value((*rt9123p).enable, 1);
                dev_dbg((*comp).dev, c"set enable to 1".as_ptr());
            }
        }
        SNDRV_PCM_TRIGGER_STOP
        | SNDRV_PCM_TRIGGER_SUSPEND
        | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            gpiod_set_value((*rt9123p).enable, 0);
            dev_dbg((*comp).dev, c"set enable to 0".as_ptr());
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn rt9123p_enable_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let comp: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let rt9123p: *mut rt9123p_priv =
        snd_soc_component_get_drvdata(comp) as *mut rt9123p_priv;

    if event & SND_SOC_DAPM_POST_PMU != 0 {
        (*rt9123p).enable_switch = 1;
    } else if event & SND_SOC_DAPM_POST_PMD != 0 {
        (*rt9123p).enable_switch = 0;
    }

    0
}

// C macro initializers preserved from:
// SND_SOC_DAPM_OUTPUT("SPK")
// SND_SOC_DAPM_OUT_DRV_E("Amp Drv", SND_SOC_NOPM, 0, 0, NULL, 0,
//                        rt9123p_enable_event,
//                        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD)
unsafe extern "C" {
    static rt9123p_dapm_widgets: [snd_soc_dapm_widget_desc; 2];
}

static rt9123p_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"Amp Drv".as_ptr(),
        control: ptr::null(),
        source: c"HiFi Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"SPK".as_ptr(),
        control: ptr::null(),
        source: c"Amp Drv".as_ptr(),
    },
];

static rt9123p_comp_driver: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: unsafe { rt9123p_dapm_widgets.as_ptr() },
    num_dapm_widgets: 2,
    dapm_routes: rt9123p_dapm_routes.as_ptr(),
    num_dapm_routes: 2,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static rt9123p_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    trigger: Some(rt9123p_daiops_trigger),
};

static mut rt9123p_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"HiFi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"HiFi Playback".as_ptr(),
        formats: SNDRV_PCM_FMTBIT_S16 | SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
        rates: SNDRV_PCM_RATE_8000
            | SNDRV_PCM_RATE_16000
            | SNDRV_PCM_RATE_22050
            | SNDRV_PCM_RATE_24000
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
    ops: &rt9123p_dai_ops,
};

unsafe extern "C" fn rt9123p_platform_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let mut rt9123p: *mut rt9123p_priv;
    let mut ret: c_int;

    rt9123p = devm_kzalloc(dev, size_of::<rt9123p_priv>(), GFP_KERNEL) as *mut rt9123p_priv;
    if rt9123p.is_null() {
        return -ENOMEM;
    }

    (*rt9123p).enable = devm_gpiod_get_optional(dev, c"enable".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*rt9123p).enable as *const c_void) {
        return PTR_ERR((*rt9123p).enable as *const c_void);
    }

    ret = device_property_read_u32(
        dev,
        c"enable-delay-ms".as_ptr(),
        &mut (*rt9123p).enable_delay,
    );
    if ret != 0 {
        (*rt9123p).enable_delay = 0;
        dev_dbg(
            dev,
            c"no optional property 'enable-delay-ms' found, default: no delay\n".as_ptr(),
        );
    }

    platform_set_drvdata(pdev, rt9123p as *mut c_void);

    devm_snd_soc_register_component(
        dev,
        &rt9123p_comp_driver,
        &raw mut rt9123p_dai_driver,
        1,
    )
}

// CONFIG_OF conditional device table from the C source.
static rt9123p_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: c"richtek,rt9123p".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, rt9123p_device_id);

// CONFIG_ACPI conditional device table from the C source.
static rt9123p_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: [
            b'R' as c_char,
            b'T' as c_char,
            b'9' as c_char,
            b'1' as c_char,
            b'2' as c_char,
            b'3' as c_char,
            b'P' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        driver_data: 0,
    },
    acpi_device_id {
        id: [0; 16],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, rt9123p_acpi_match);

static mut rt9123p_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"rt9123p".as_ptr(),
        // of_match_ptr(rt9123p_device_id)
        of_match_table: rt9123p_device_id.as_ptr(),
        // ACPI_PTR(rt9123p_acpi_match)
        acpi_match_table: rt9123p_acpi_match.as_ptr(),
    },
    probe: Some(rt9123p_platform_probe),
};

unsafe fn __register_rt9123p_platform_driver() {
    module_platform_driver(&raw mut rt9123p_platform_driver);
}

// MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
// MODULE_DESCRIPTION("ASoC rt9123p Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
