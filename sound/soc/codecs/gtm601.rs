// SPDX-License-Identifier: GPL-2.0-only
/*
 * This is a simple driver for the GTM601 Voice PCM interface
 *
 * Copyright (C) 2015 Goldelico GmbH
 *
 * Author: Marek Belisko <marek@goldelico.com>
 *
 * Based on wm8727.c driver
 */

// C includes translated as dependency intent:
// <linux/init.h>, <linux/slab.h>, <linux/module.h>, <linux/kernel.h>,
// <linux/of.h>, <sound/core.h>, <sound/pcm.h>, <sound/initval.h>,
// <sound/soc.h>

use core::ffi::{c_char, c_int, c_void};

const SNDRV_PCM_RATE_8000: u32 = 1 << 0;
const SNDRV_PCM_RATE_48000: u32 = 1 << 10;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;

const SND_SOC_DAPM_OUTPUT: c_int = 0;
const SND_SOC_DAPM_INPUT: c_int = 1;

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    fn of_device_get_match_data(dev: *const device) -> *const c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

static GTM601_DAPM_WIDGETS: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget {
        id: SND_SOC_DAPM_OUTPUT,
        name: c"AOUT".as_ptr(),
    },
    snd_soc_dapm_widget {
        id: SND_SOC_DAPM_INPUT,
        name: c"AIN".as_ptr(),
    },
];

static GTM601_DAPM_ROUTES: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"AOUT".as_ptr(),
        control: core::ptr::null(),
        source: c"Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"AIN".as_ptr(),
    },
];

static mut GTM601_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"gtm601".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 1,
        rates: SNDRV_PCM_RATE_8000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 1,
        rates: SNDRV_PCM_RATE_8000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
};

static mut BM818_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"bm818".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
};

static SOC_COMPONENT_DEV_GTM601: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: GTM601_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: GTM601_DAPM_WIDGETS.len(),
    dapm_routes: GTM601_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: GTM601_DAPM_ROUTES.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn gtm601_platform_probe(pdev: *mut platform_device) -> c_int {
    let dai_driver: *const snd_soc_dai_driver;

    dai_driver = unsafe { of_device_get_match_data(core::ptr::addr_of!((*pdev).dev)) }
        as *const snd_soc_dai_driver;

    unsafe {
        devm_snd_soc_register_component(
            core::ptr::addr_of_mut!((*pdev).dev),
            &SOC_COMPONENT_DEV_GTM601,
            dai_driver as *mut snd_soc_dai_driver,
            1,
        )
    }
}

// __maybe_unused in C.
static GTM601_CODEC_OF_MATCH: [of_device_id; 3] = [
    of_device_id {
        compatible: c"option,gtm601".as_ptr(),
        data: core::ptr::addr_of!(GTM601_DAI) as *const c_void,
    },
    of_device_id {
        compatible: c"broadmobi,bm818".as_ptr(),
        data: core::ptr::addr_of!(BM818_DAI) as *const c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, gtm601_codec_of_match);

static mut GTM601_CODEC_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: c"gtm601".as_ptr(),
        of_match_table: GTM601_CODEC_OF_MATCH.as_ptr(),
    },
    probe: Some(gtm601_platform_probe),
};

// module_platform_driver(gtm601_codec_driver);

// MODULE_DESCRIPTION("ASoC gtm601 driver");
// MODULE_AUTHOR("Marek Belisko <marek@goldelico.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:gtm601");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
