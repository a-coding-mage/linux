// SPDX-License-Identifier: GPL-2.0-only
/*
 * I2S MEMS microphone driver for InvenSense ICS-43432 and similar
 * MEMS-based microphones.
 *
 * - Non configurable.
 * - I2S interface, 64 BCLs per frame, 32 bits per channel, 24 bit data
 *
 * Copyright (c) 2015 Axis Communications AB
 */

// C dependencies:
// #include <linux/module.h>
// #include <linux/init.h>
// #include <linux/slab.h>
// #include <sound/core.h>
// #include <sound/pcm.h>
// #include <sound/pcm_params.h>
// #include <sound/soc.h>
// #include <sound/initval.h>
// #include <sound/tlv.h>

use core::ffi::{c_char, c_int};

const ICS43432_RATE_MIN: c_int = 7190; /* Hz, from data sheet */
const ICS43432_RATE_MAX: c_int = 52800; /* Hz, from data sheet */

// Supplied by <sound/pcm.h> in the original C source.
const ICS43432_FORMATS: u64 = SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub rates: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub idle_bias_on: u8,
    pub use_pmdown_time: u8,
    pub endianness: u8,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

unsafe impl Sync for of_device_id {}

unsafe extern "C" {
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

static mut ICS43432_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"ics43432-hifi\0".as_ptr() as *const c_char,
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rate_min: ICS43432_RATE_MIN as u32,
        rate_max: ICS43432_RATE_MAX as u32,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        formats: ICS43432_FORMATS,
    },
};

static ICS43432_COMPONENT_DRIVER: snd_soc_component_driver = snd_soc_component_driver {
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn ics43432_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &ICS43432_COMPONENT_DRIVER,
            &mut ICS43432_DAI,
            1,
        )
    }
}

// #ifdef CONFIG_OF
static ICS43432_IDS: [of_device_id; 3] = [
    of_device_id {
        compatible: b"invensense,ics43432\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"cui,cmm-4030d-261\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, ics43432_ids);
// #endif

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

static mut ICS43432_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: b"ics43432\0".as_ptr() as *const c_char,
        of_match_table: ICS43432_IDS.as_ptr(),
    },
    probe: Some(ics43432_probe),
};

// module_platform_driver(ics43432_driver);

// MODULE_DESCRIPTION("ASoC ICS43432 driver");
// MODULE_AUTHOR("Ricard Wanderlof <ricardw@axis.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
