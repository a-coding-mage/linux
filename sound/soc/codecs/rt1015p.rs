// SPDX-License-Identifier: GPL-2.0-only
//
// rt1015p.c  --  RT1015P ALSA SoC audio amplifier driver
//
// Copyright 2020 The Linux Foundation. All rights reserved.

// Dependencies from the original C includes:
// linux/acpi.h, linux/delay.h, linux/device.h, linux/err.h,
// linux/gpio/consumer.h, linux/kernel.h, linux/module.h, linux/of.h,
// linux/platform_device.h, sound/pcm.h, sound/soc.h, sound/soc-dai.h,
// sound/soc-dapm.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
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
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_entry {
    pub name: *const c_char,
    pub id: c_int,
    pub reg: c_int,
    pub shift: c_int,
    pub invert: c_int,
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
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget_entry,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
    pub idle_bias_on: c_int,
    pub use_pmdown_time: c_int,
    pub endianness: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub formats: c_ulong,
    pub rates: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

pub type c_uint = u32;

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
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
pub struct acpi_device_id {
    pub id: *const c_char,
    pub driver_data: c_ulong,
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
pub struct rt1015p_priv {
    pub sdb: *mut gpio_desc,
    pub calib_done: bool,
}

unsafe extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn msleep(msecs: c_uint);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn of_match_ptr(ids: *const of_device_id) -> *const of_device_id;
    fn ACPI_PTR(ids: *const acpi_device_id) -> *const acpi_device_id;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

pub const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
pub const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
pub const SND_SOC_NOPM: c_int = -1;
pub const SND_SOC_DAPM_OUTPUT_ID: c_int = 0;
pub const SND_SOC_DAPM_OUT_DRV_E_ID: c_int = 1;
pub const SNDRV_PCM_FMTBIT_S24: c_ulong = 1 << 6;
pub const SNDRV_PCM_FMTBIT_S32: c_ulong = 1 << 10;
pub const SNDRV_PCM_RATE_48000: c_uint = 1 << 10;
pub const GFP_KERNEL: c_uint = 0;
pub const GPIOD_OUT_LOW: c_int = 0;
pub const ENOMEM: c_int = 12;

unsafe extern "C" fn rt1015p_sdb_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt1015p = snd_soc_component_get_drvdata(component) as *mut rt1015p_priv;

    if (*rt1015p).sdb.is_null() {
        return 0;
    }

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            gpiod_set_value_cansleep((*rt1015p).sdb, 1);
            dev_dbg((*component).dev, c"set sdb to 1".as_ptr());

            if !(*rt1015p).calib_done {
                msleep(300);
                (*rt1015p).calib_done = true;
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            gpiod_set_value_cansleep((*rt1015p).sdb, 0);
            dev_dbg((*component).dev, c"set sdb to 0".as_ptr());
        }
        _ => {}
    }

    0
}

static RT1015P_DAPM_WIDGETS: [snd_soc_dapm_widget_entry; 2] = [
    snd_soc_dapm_widget_entry {
        name: c"Speaker".as_ptr(),
        id: SND_SOC_DAPM_OUTPUT_ID,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        event: None,
        event_flags: 0,
    },
    snd_soc_dapm_widget_entry {
        name: c"SDB".as_ptr(),
        id: SND_SOC_DAPM_OUT_DRV_E_ID,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        event: Some(rt1015p_sdb_event),
        event_flags: SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
    },
];

static RT1015P_DAPM_ROUTES: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"SDB".as_ptr(),
        control: ptr::null(),
        source: c"HiFi Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"SDB".as_ptr(),
    },
];

// Original C condition: #ifdef CONFIG_PM
unsafe extern "C" fn rt1015p_suspend(component: *mut snd_soc_component) -> c_int {
    let rt1015p = snd_soc_component_get_drvdata(component) as *mut rt1015p_priv;

    (*rt1015p).calib_done = false;
    0
}
// Original C else branch: #define rt1015p_suspend NULL

static RT1015P_COMPONENT_DRIVER: snd_soc_component_driver = snd_soc_component_driver {
    suspend: Some(rt1015p_suspend),
    dapm_widgets: RT1015P_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: RT1015P_DAPM_WIDGETS.len(),
    dapm_routes: RT1015P_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: RT1015P_DAPM_ROUTES.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static mut RT1015P_DAI_DRIVER: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"HiFi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"HiFi Playback".as_ptr(),
        formats: SNDRV_PCM_FMTBIT_S24 | SNDRV_PCM_FMTBIT_S32,
        rates: SNDRV_PCM_RATE_48000,
        channels_min: 1,
        channels_max: 2,
    },
};

unsafe extern "C" fn rt1015p_platform_probe(pdev: *mut platform_device) -> c_int {
    let rt1015p: *mut rt1015p_priv;

    rt1015p = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<rt1015p_priv>(),
        GFP_KERNEL,
    ) as *mut rt1015p_priv;
    if rt1015p.is_null() {
        return -ENOMEM;
    }

    (*rt1015p).sdb =
        devm_gpiod_get_optional(&mut (*pdev).dev, c"sdb".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*rt1015p).sdb as *const c_void) {
        return PTR_ERR((*rt1015p).sdb as *const c_void);
    }

    dev_set_drvdata(&mut (*pdev).dev, rt1015p as *mut c_void);

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &RT1015P_COMPONENT_DRIVER,
        &raw mut RT1015P_DAI_DRIVER,
        1,
    )
}

// Original C condition: #ifdef CONFIG_OF
static RT1015P_DEVICE_ID: [of_device_id; 3] = [
    of_device_id {
        compatible: c"realtek,rt1015p".as_ptr(),
    },
    of_device_id {
        compatible: c"realtek,rt1019p".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, rt1015p_device_id);

// Original C condition: #ifdef CONFIG_ACPI
static RT1015P_ACPI_MATCH: [acpi_device_id; 3] = [
    acpi_device_id {
        id: c"RTL1015".as_ptr(),
        driver_data: 0,
    },
    acpi_device_id {
        id: c"RTL1019".as_ptr(),
        driver_data: 0,
    },
    acpi_device_id {
        id: ptr::null(),
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, rt1015p_acpi_match);

static mut RT1015P_PLATFORM_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: c"rt1015p".as_ptr(),
        of_match_table: unsafe { of_match_ptr(RT1015P_DEVICE_ID.as_ptr()) },
        acpi_match_table: unsafe { ACPI_PTR(RT1015P_ACPI_MATCH.as_ptr()) },
    },
    probe: Some(rt1015p_platform_probe),
};

// module_platform_driver(rt1015p_platform_driver);
// MODULE_DESCRIPTION("ASoC RT1015P driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
