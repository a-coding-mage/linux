// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for the PCM5102A codec
 *
 * Author:	Florian Meier <florian.meier@koalo.de>
 *		Copyright 2013
 */

// C dependencies: <linux/init.h>, <linux/module.h>,
// <linux/platform_device.h>, and <sound/soc.h>.

extern "C" {
    static SNDRV_PCM_RATE_8000_384000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;

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
    pub playback: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub idle_bias_on: u32,
    pub use_pmdown_time: u32,
    pub endianness: u32,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const i8,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const i8,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: device_driver,
}

static mut pcm5102a_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"pcm5102a-hifi\0".as_ptr() as *const i8,
    playback: snd_soc_pcm_stream {
        stream_name: core::ptr::null(),
        channels_min: 2,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_384000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE },
    },
};

static soc_component_dev_pcm5102a: snd_soc_component_driver = snd_soc_component_driver {
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn pcm5102a_probe(pdev: *mut platform_device) -> i32 {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_pcm5102a,
        &mut pcm5102a_dai,
        1,
    )
}

static pcm5102a_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"ti,pcm5102a\0".as_ptr() as *const i8,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, pcm5102a_of_match);

static mut pcm5102a_codec_driver: platform_driver = platform_driver {
    probe: Some(pcm5102a_probe),
    driver: device_driver {
        name: b"pcm5102a-codec\0".as_ptr() as *const i8,
        of_match_table: pcm5102a_of_match.as_ptr(),
    },
};

// module_platform_driver(pcm5102a_codec_driver);

// MODULE_DESCRIPTION("ASoC PCM5102A codec driver");
// MODULE_AUTHOR("Florian Meier <florian.meier@koalo.de>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
