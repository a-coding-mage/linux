// SPDX-License-Identifier: GPL-2.0-only
/*
 * ADAU7002 Stereo PDM-to-I2S/TDM converter driver
 *
 * Copyright 2014-2016 Analog Devices
 *  Author: Lars-Peter Clausen <lars@metafoo.de>
 */

/* Dependencies from the original C includes:
 * linux/acpi.h, linux/delay.h, linux/init.h, linux/module.h, linux/of.h,
 * linux/platform_device.h, and sound/soc.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
    pub sig_bits: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    pub kind: c_int,
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub reg: c_int,
    pub shift: c_uint,
    pub invert: c_uint,
    pub event: Option<
        unsafe extern "C" fn(
            *mut snd_soc_dapm_widget,
            *mut snd_kcontrol,
            c_int,
        ) -> c_int,
    >,
    pub event_flags: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 9],
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
struct adau7002_priv {
    wakeup_delay: c_int,
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const SND_SOC_DAPM_POST_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const SND_SOC_NOPM: c_int = -1;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S18_3LE: c_ulong = 1 << 1;
const SNDRV_PCM_FMTBIT_S20_3LE: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 1 << 3;
const SNDRV_PCM_FMTBIT_S24_3LE: c_ulong = 1 << 4;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 1 << 5;

const SND_SOC_DAPM_AIF_OUT_E_KIND: c_int = 0;
const SND_SOC_DAPM_INPUT_KIND: c_int = 1;
const SND_SOC_DAPM_REGULATOR_SUPPLY_KIND: c_int = 2;

unsafe extern "C" {
    fn snd_soc_dapm_to_component(
        dapm: *mut snd_soc_dapm_context,
    ) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut c_void);
    fn msleep(msecs: c_uint);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn device_property_read_u32(
        dev: *mut device,
        propname: *const c_char,
        val: *mut c_int,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

unsafe extern "C" fn adau7002_aif_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let adau7002: *mut adau7002_priv =
        snd_soc_component_get_drvdata(component) as *mut adau7002_priv;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            if (*adau7002).wakeup_delay != 0 {
                msleep((*adau7002).wakeup_delay as c_uint);
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn adau7002_component_probe(
    component: *mut snd_soc_component,
) -> c_int {
    let adau7002: *mut adau7002_priv;

    adau7002 = devm_kzalloc(
        (*component).dev,
        size_of::<adau7002_priv>(),
        GFP_KERNEL,
    ) as *mut adau7002_priv;
    if adau7002.is_null() {
        return -ENOMEM;
    }

    device_property_read_u32(
        (*component).dev,
        c"wakeup-delay-ms".as_ptr(),
        &mut (*adau7002).wakeup_delay,
    );

    snd_soc_component_set_drvdata(component, adau7002 as *mut c_void);

    0
}

static ADAU7002_WIDGETS: [snd_soc_dapm_widget_desc; 3] = [
    snd_soc_dapm_widget_desc {
        kind: SND_SOC_DAPM_AIF_OUT_E_KIND,
        name: c"ADAU AIF".as_ptr(),
        stream_name: c"Capture".as_ptr(),
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        event: Some(adau7002_aif_event),
        event_flags: SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD,
    },
    snd_soc_dapm_widget_desc {
        kind: SND_SOC_DAPM_INPUT_KIND,
        name: c"PDM_DAT".as_ptr(),
        stream_name: ptr::null(),
        reg: 0,
        shift: 0,
        invert: 0,
        event: None,
        event_flags: 0,
    },
    snd_soc_dapm_widget_desc {
        kind: SND_SOC_DAPM_REGULATOR_SUPPLY_KIND,
        name: c"IOVDD".as_ptr(),
        stream_name: ptr::null(),
        reg: 0,
        shift: 0,
        invert: 0,
        event: None,
        event_flags: 0,
    },
];

static ADAU7002_ROUTES: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route {
        sink: c"ADAU AIF".as_ptr(),
        control: ptr::null(),
        source: c"PDM_DAT".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Capture".as_ptr(),
        control: ptr::null(),
        source: c"PDM_DAT".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Capture".as_ptr(),
        control: ptr::null(),
        source: c"IOVDD".as_ptr(),
    },
];

static mut ADAU7002_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"adau7002-hifi".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S18_3LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S24_3LE
            | SNDRV_PCM_FMTBIT_S32_LE,
        sig_bits: 20,
    },
};

static ADAU7002_COMPONENT_DRIVER: snd_soc_component_driver =
    snd_soc_component_driver {
        probe: Some(adau7002_component_probe),
        dapm_widgets: ADAU7002_WIDGETS.as_ptr(),
        num_dapm_widgets: ADAU7002_WIDGETS.len() as c_uint,
        dapm_routes: ADAU7002_ROUTES.as_ptr(),
        num_dapm_routes: ADAU7002_ROUTES.len() as c_uint,
        idle_bias_on: 1,
        use_pmdown_time: 1,
        endianness: 1,
    };

unsafe extern "C" fn adau7002_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &ADAU7002_COMPONENT_DRIVER,
        &mut ADAU7002_DAI,
        1,
    )
}

/* Original C condition: #ifdef CONFIG_OF */
static ADAU7002_DT_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: c"adi,adau7002".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, adau7002_dt_ids); */

/* Original C condition: #ifdef CONFIG_ACPI */
static ADAU7002_ACPI_MATCH: [acpi_device_id; 2] = [
    acpi_device_id {
        id: [
            b'A' as c_char,
            b'D' as c_char,
            b'A' as c_char,
            b'U' as c_char,
            b'7' as c_char,
            b'0' as c_char,
            b'0' as c_char,
            b'2' as c_char,
            0,
        ],
        driver_data: 0,
    },
    acpi_device_id {
        id: [0; 9],
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(acpi, adau7002_acpi_match); */

static mut ADAU7002_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: c"adau7002".as_ptr(),
        /* of_match_ptr(adau7002_dt_ids) */
        of_match_table: ADAU7002_DT_IDS.as_ptr(),
        /* ACPI_PTR(adau7002_acpi_match) */
        acpi_match_table: ADAU7002_ACPI_MATCH.as_ptr(),
    },
    probe: Some(adau7002_probe),
};
/* module_platform_driver(adau7002_driver); */

/* MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>"); */
/* MODULE_DESCRIPTION("ADAU7002 Stereo PDM-to-I2S/TDM Converter driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
