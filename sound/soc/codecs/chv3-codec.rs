// SPDX-License-Identifier: GPL-2.0-only
// Dependencies from C includes:
// #include <linux/module.h>
// #include <sound/soc.h>

use core::ffi::{c_char, c_int};

const SNDRV_PCM_RATE_CONTINUOUS: u32 = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 0;

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
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
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
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

static mut chv3_codec_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"chv3-codec-hifi".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 8,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        formats: SNDRV_PCM_FMTBIT_S32_LE,
    },
};

static soc_component_dev_chv3_codec: snd_soc_component_driver = snd_soc_component_driver {
    _private: [],
};

unsafe extern "C" fn chv3_codec_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &soc_component_dev_chv3_codec,
            &raw mut chv3_codec_dai,
            1,
        )
    }
}

static chv3_codec_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"google,chv3-codec".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, chv3_codec_of_match);

static mut chv3_codec_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"chv3-codec".as_ptr(),
        of_match_table: chv3_codec_of_match.as_ptr(),
    },
    probe: Some(chv3_codec_probe),
};

// module_platform_driver(chv3_codec_platform_driver);

// MODULE_DESCRIPTION("ASoC Chameleon v3 codec driver");
// MODULE_AUTHOR("Pawel Anikiel <pan@semihalf.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
