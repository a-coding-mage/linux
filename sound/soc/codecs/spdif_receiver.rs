// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC SPDIF DIR (Digital Interface Reciever) driver
 *
 * Based on ALSA SoC SPDIF DIT driver
 *
 *  This driver is used by controllers which can operate in DIR (SPDI/F) where
 *  no codec is needed.  This file provides stub codec that can be used
 *  in these configurations. SPEAr SPDIF IN Audio controller uses this driver.
 *
 * Author:      Vipin Kumar,  <vipin.kumar@st.com>
 * Copyright:   (C) 2012  ST Microelectronics
 */

use core::ffi::{c_char, c_int};
use core::ptr;

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
    pub id: c_int,
    pub name: *const c_char,
}
unsafe impl Sync for snd_soc_dapm_widget {}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
unsafe impl Sync for snd_soc_dapm_route {}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub formats: u64,
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
unsafe impl Sync for snd_soc_component_driver {}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
unsafe impl Sync for of_device_id {}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}
unsafe impl Sync for platform_driver {}

unsafe extern "C" {
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

// Required macro/constant dependencies from included kernel headers:
// SND_SOC_DAPM_INPUT, SNDRV_PCM_RATE_8000_768000, SNDRV_PCM_RATE_128000,
// SNDRV_PCM_FMTBIT_S16_LE, SNDRV_PCM_FMTBIT_S20_3LE,
// SNDRV_PCM_FMTBIT_S24_LE, SNDRV_PCM_FMTBIT_S32_LE,
// SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE.
//
// From <sound/soc.h>: SND_SOC_DAPM_INPUT("spdif-in")
static DIR_WIDGETS: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget {
    id: SND_SOC_DAPM_INPUT,
    name: c"spdif-in".as_ptr(),
}];

static DIR_ROUTES: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: c"Capture".as_ptr(),
    control: ptr::null(),
    source: c"spdif-in".as_ptr(),
}];

const STUB_RATES: u32 = SNDRV_PCM_RATE_8000_768000 | SNDRV_PCM_RATE_128000;
const STUB_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE
    | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE;

static SOC_CODEC_SPDIF_DIR: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: DIR_WIDGETS.as_ptr(),
    num_dapm_widgets: DIR_WIDGETS.len(),
    dapm_routes: DIR_ROUTES.as_ptr(),
    num_dapm_routes: DIR_ROUTES.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static mut DIR_STUB_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"dir-hifi".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 384,
        rates: STUB_RATES,
        formats: STUB_FORMATS,
    },
};

unsafe extern "C" fn spdif_dir_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &SOC_CODEC_SPDIF_DIR,
            &raw mut DIR_STUB_DAI,
            1,
        )
    }
}

// #ifdef CONFIG_OF
static SPDIF_DIR_DT_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: c"linux,spdif-dir".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, spdif_dir_dt_ids);
// #endif

static mut SPDIF_DIR_DRIVER: platform_driver = platform_driver {
    probe: Some(spdif_dir_probe),
    driver: device_driver {
        name: c"spdif-dir".as_ptr(),
        // of_match_ptr(spdif_dir_dt_ids)
        of_match_table: SPDIF_DIR_DT_IDS.as_ptr(),
    },
};

// module_platform_driver(spdif_dir_driver);
// MODULE_DESCRIPTION("ASoC SPDIF DIR driver");
// MODULE_AUTHOR("Vipin Kumar <vipin.kumar@st.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
