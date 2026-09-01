// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ad73311.c  --  ALSA Soc AD73311 codec support
 *
 * Copyright:	Analog Devices Inc.
 * Author:	Cliff Cai <cliff.cai@analog.com>
 */

// C dependencies translated from:
// <linux/init.h>, <linux/slab.h>, <linux/module.h>, <linux/kernel.h>,
// <linux/device.h>, <sound/core.h>, <sound/pcm.h>, <sound/ac97_codec.h>,
// <sound/initval.h>, <sound/soc.h>, and "ad73311.h".

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    static SNDRV_PCM_RATE_8000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;

    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
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
pub struct platform_driver_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
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
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
    pub idle_bias_on: c_int,
    pub use_pmdown_time: c_int,
    pub endianness: c_int,
}

unsafe extern "C" {
    fn SND_SOC_DAPM_INPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget;
}

static AD73311_DAPM_WIDGETS: [snd_soc_dapm_widget; 4] = unsafe {
    [
        SND_SOC_DAPM_INPUT(c"VINP".as_ptr()),
        SND_SOC_DAPM_INPUT(c"VINN".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"VOUTN".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"VOUTP".as_ptr()),
    ]
};

static AD73311_DAPM_ROUTES: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: c"Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"VINP".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"VINN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"VOUTN".as_ptr(),
        control: core::ptr::null(),
        source: c"Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"VOUTP".as_ptr(),
        control: core::ptr::null(),
        source: c"Playback".as_ptr(),
    },
];

static mut AD73311_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"ad73311-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 1,
        rates: unsafe { SNDRV_PCM_RATE_8000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 1,
        rates: unsafe { SNDRV_PCM_RATE_8000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
    },
};

static SOC_COMPONENT_DEV_AD73311: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: AD73311_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: AD73311_DAPM_WIDGETS.len(),
    dapm_routes: AD73311_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: AD73311_DAPM_ROUTES.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn ad73311_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &SOC_COMPONENT_DEV_AD73311,
            &raw mut AD73311_DAI,
            1,
        )
    }
}

static mut AD73311_CODEC_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: c"ad73311".as_ptr(),
    },
    probe: Some(ad73311_probe),
};

// module_platform_driver(ad73311_codec_driver);
// MODULE_DESCRIPTION("ASoC ad73311 driver");
// MODULE_AUTHOR("Cliff Cai ");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
