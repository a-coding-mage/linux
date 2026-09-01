// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * wm8727.c
 *
 *  Created on: 15-Oct-2009
 *      Author: neil.jones@imgtec.com
 *
 * Copyright (C) 2009 Imagination Technologies Ltd.
 */

// C includes translated as dependency intent:
// linux/init.h, linux/slab.h, linux/module.h, linux/kernel.h, linux/device.h
// sound/core.h, sound/pcm.h, sound/initval.h, sound/soc.h

extern "C" {
    static SNDRV_PCM_RATE_32000: u32;
    static SNDRV_PCM_RATE_44100: u32;
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_RATE_96000: u32;
    static SNDRV_PCM_RATE_192000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;

    fn SND_SOC_DAPM_OUTPUT(name: *const ::core::ffi::c_char) -> snd_soc_dapm_widget;

    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const ::core::ffi::c_char,
    pub control: *const ::core::ffi::c_char,
    pub source: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const ::core::ffi::c_char,
    pub channels_min: ::core::ffi::c_uint,
    pub channels_max: ::core::ffi::c_uint,
    pub rates: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const ::core::ffi::c_char,
    pub playback: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: ::core::ffi::c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: ::core::ffi::c_uint,
    pub idle_bias_on: ::core::ffi::c_uint,
    pub use_pmdown_time: ::core::ffi::c_uint,
    pub endianness: ::core::ffi::c_uint,
}

static wm8727_dapm_widgets: [snd_soc_dapm_widget; 2] = unsafe {
    [
        SND_SOC_DAPM_OUTPUT(c"VOUTL".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"VOUTR".as_ptr()),
    ]
};

static wm8727_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"VOUTL".as_ptr(),
        control: ::core::ptr::null(),
        source: c"Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"VOUTR".as_ptr(),
        control: ::core::ptr::null(),
        source: c"Playback".as_ptr(),
    },
];

/*
 * Note this is a simple chip with no configuration interface, sample rate is
 * determined automatically by examining the Master clock and Bit clock ratios
 */
unsafe fn WM8727_RATES() -> u32 {
    SNDRV_PCM_RATE_32000
        | SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_96000
        | SNDRV_PCM_RATE_192000
}

static mut wm8727_dai: snd_soc_dai_driver = unsafe {
    snd_soc_dai_driver {
        name: c"wm8727-hifi".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"Playback".as_ptr(),
            channels_min: 2,
            channels_max: 2,
            rates: WM8727_RATES(),
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
        },
    }
};

static soc_component_dev_wm8727: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: wm8727_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8727_dapm_widgets.len() as ::core::ffi::c_uint,
    dapm_routes: wm8727_dapm_routes.as_ptr(),
    num_dapm_routes: wm8727_dapm_routes.len() as ::core::ffi::c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn wm8727_probe(pdev: *mut platform_device) -> ::core::ffi::c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_wm8727,
        &mut wm8727_dai,
        1,
    )
}

static mut wm8727_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"wm8727".as_ptr(),
    },

    probe: Some(wm8727_probe),
};

// module_platform_driver(wm8727_codec_driver);
// MODULE_DESCRIPTION("ASoC wm8727 driver");
// MODULE_AUTHOR("Neil Jones");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
