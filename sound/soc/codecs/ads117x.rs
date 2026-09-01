// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ads117x.c  --  Driver for ads1174/8 ADC chips
 *
 * Copyright 2009 ShotSpotter Inc.
 * Author: Graeme Gregory <gg@slimlogic.co.uk>
 */

// C dependencies removed from executable Rust:
// linux/kernel.h, linux/slab.h, linux/init.h, linux/device.h,
// linux/module.h, sound/core.h, sound/pcm.h, sound/initval.h,
// sound/soc.h, linux/of.h.

extern "C" {
    static SNDRV_PCM_RATE_8000_48000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static ads117x_dt_ids: [of_device_id; 3];

    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: i32,
    ) -> i32;
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
pub struct snd_soc_dapm_widget {
    pub id: i32,
    pub name: *const i8,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const i8,
    pub control: *const i8,
    pub source: *const i8,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const i8,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const i8,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
    pub idle_bias_on: u8,
    pub use_pmdown_time: u8,
    pub endianness: u8,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const i8,
}

#[repr(C)]
pub struct driver_private {
    pub name: *const i8,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

const SND_SOC_DAPM_INPUT_ID: i32 = 0;

unsafe fn of_match_ptr<T>(ptr: *const T) -> *const T {
    ptr
}

unsafe fn snd_soc_dapm_input(name: *const i8) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget {
        id: SND_SOC_DAPM_INPUT_ID,
        name,
    }
}

unsafe fn ads117x_rates() -> u32 {
    SNDRV_PCM_RATE_8000_48000
}

unsafe fn ads117x_formats() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE
}

static mut ads117x_dapm_widgets: [snd_soc_dapm_widget; 8] = unsafe {
    [
        snd_soc_dapm_input(b"Input1\0".as_ptr() as *const i8),
        snd_soc_dapm_input(b"Input2\0".as_ptr() as *const i8),
        snd_soc_dapm_input(b"Input3\0".as_ptr() as *const i8),
        snd_soc_dapm_input(b"Input4\0".as_ptr() as *const i8),
        snd_soc_dapm_input(b"Input5\0".as_ptr() as *const i8),
        snd_soc_dapm_input(b"Input6\0".as_ptr() as *const i8),
        snd_soc_dapm_input(b"Input7\0".as_ptr() as *const i8),
        snd_soc_dapm_input(b"Input8\0".as_ptr() as *const i8),
    ]
};

static mut ads117x_dapm_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route {
        sink: b"Capture\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"Input1\0".as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: b"Capture\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"Input2\0".as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: b"Capture\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"Input3\0".as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: b"Capture\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"Input4\0".as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: b"Capture\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"Input5\0".as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: b"Capture\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"Input6\0".as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: b"Capture\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"Input7\0".as_ptr() as *const i8,
    },
    snd_soc_dapm_route {
        sink: b"Capture\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"Input8\0".as_ptr() as *const i8,
    },
];

static mut ads117x_dai: snd_soc_dai_driver = unsafe {
    snd_soc_dai_driver {
        /* ADC */
        name: b"ads117x-hifi\0".as_ptr() as *const i8,
        capture: snd_soc_pcm_stream {
            stream_name: b"Capture\0".as_ptr() as *const i8,
            channels_min: 1,
            channels_max: 32,
            rates: ads117x_rates(),
            formats: ads117x_formats(),
        },
    }
};

static mut soc_component_dev_ads117x: snd_soc_component_driver = unsafe {
    snd_soc_component_driver {
        dapm_widgets: ads117x_dapm_widgets.as_ptr(),
        num_dapm_widgets: ads117x_dapm_widgets.len(),
        dapm_routes: ads117x_dapm_routes.as_ptr(),
        num_dapm_routes: ads117x_dapm_routes.len(),
        idle_bias_on: 1,
        use_pmdown_time: 1,
        endianness: 1,
    }
};

unsafe extern "C" fn ads117x_probe(pdev: *mut platform_device) -> i32 {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_ads117x,
        &mut ads117x_dai,
        1,
    )
}

// Original C condition: #if defined(CONFIG_OF)
#[cfg(CONFIG_OF)]
static ads117x_dt_ids_cfg: [of_device_id; 3] = [
    of_device_id {
        compatible: b"ti,ads1174\0".as_ptr() as *const i8,
    },
    of_device_id {
        compatible: b"ti,ads1178\0".as_ptr() as *const i8,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, ads117x_dt_ids);
// #endif

static mut ads117x_codec_driver: platform_driver = unsafe {
    platform_driver {
        driver: driver_private {
            name: b"ads117x-codec\0".as_ptr() as *const i8,
            of_match_table: of_match_ptr(ads117x_dt_ids.as_ptr()),
        },
        probe: Some(ads117x_probe),
    }
};

// module_platform_driver(ads117x_codec_driver);

// MODULE_DESCRIPTION("ASoC ads117x driver");
// MODULE_AUTHOR("Graeme Gregory");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
