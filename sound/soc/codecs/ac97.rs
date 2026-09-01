// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ac97.c  --  ALSA Soc AC97 codec support
 *
 * Copyright 2005 Wolfson Microelectronics PLC.
 * Author: Liam Girdwood <lrg@slimlogic.co.uk>
 *
 * Generic AC97 support.
 */

// C dependencies:
// <linux/init.h>, <linux/slab.h>, <linux/kernel.h>, <linux/device.h>,
// <linux/module.h>, <linux/of.h>, <sound/core.h>, <sound/pcm.h>,
// <sound/ac97_codec.h>, <sound/initval.h>, <sound/soc.h>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

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
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ac97 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ac97_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ac97_template {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub prepare: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
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
    pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
}

unsafe extern "C" {
    static soc_ac97_ops: *const c_void;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut c_void);
    fn snd_ac97_set_rate(ac97: *mut snd_ac97, reg: c_int, rate: c_uint) -> c_int;
    fn snd_ac97_bus(
        card: *mut snd_card,
        num: c_int,
        ops: *const c_void,
        private_data: *mut c_void,
        bus: *mut *mut snd_ac97_bus,
    ) -> c_int;
    fn snd_ac97_mixer(
        bus: *mut snd_ac97_bus,
        template: *mut snd_ac97_template,
        ac97: *mut *mut snd_ac97,
    ) -> c_int;
    fn snd_ac97_suspend(ac97: *mut snd_ac97);
    fn snd_ac97_resume(ac97: *mut snd_ac97);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn snd_soc_dapm_input(name: *const c_char) -> snd_soc_dapm_widget;
    fn snd_soc_dapm_output(name: *const c_char) -> snd_soc_dapm_widget;
    fn module_platform_driver(driver: *mut platform_driver);
}

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const AC97_PCM_FRONT_DAC_RATE: c_int = 0x2c;
const AC97_PCM_LR_ADC_RATE: c_int = 0x32;
const SNDRV_PCM_RATE_KNOT: c_uint = 1 << 31;
const SND_SOC_STD_AC97_FMTS: u64 = 0;

const RX: &[u8; 3] = b"RX\0";
const TX: &[u8; 3] = b"TX\0";
const AC97_CAPTURE: &[u8; 13] = b"AC97 Capture\0";
const AC97_PLAYBACK: &[u8; 14] = b"AC97 Playback\0";
const AC97_HIFI: &[u8; 10] = b"ac97-hifi\0";
const REALTEK_ALC203: &[u8; 15] = b"realtek,alc203\0";
const AC97_CODEC: &[u8; 11] = b"ac97-codec\0";

static mut ac97_widgets: [snd_soc_dapm_widget; 2] = unsafe {
    [
        snd_soc_dapm_input(RX.as_ptr() as *const c_char),
        snd_soc_dapm_output(TX.as_ptr() as *const c_char),
    ]
};

static ac97_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: AC97_CAPTURE.as_ptr() as *const c_char,
        control: ptr::null(),
        source: RX.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: TX.as_ptr() as *const c_char,
        control: ptr::null(),
        source: AC97_PLAYBACK.as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn ac97_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let ac97: *mut snd_ac97 = snd_soc_component_get_drvdata(component) as *mut snd_ac97;

    let reg: c_int = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        AC97_PCM_FRONT_DAC_RATE
    } else {
        AC97_PCM_LR_ADC_RATE
    };
    snd_ac97_set_rate(ac97, reg, (*(*substream).runtime).rate)
}

static ac97_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(ac97_prepare),
};

static mut ac97_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: AC97_HIFI.as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: AC97_PLAYBACK.as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: SND_SOC_STD_AC97_FMTS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: AC97_CAPTURE.as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: SND_SOC_STD_AC97_FMTS,
    },
    ops: &ac97_dai_ops,
};

unsafe extern "C" fn ac97_soc_probe(component: *mut snd_soc_component) -> c_int {
    let mut ac97: *mut snd_ac97 = ptr::null_mut();
    let mut ac97_bus: *mut snd_ac97_bus = ptr::null_mut();
    let mut ac97_template: snd_ac97_template = mem::zeroed();
    let mut ret: c_int;

    /* add codec as bus device for standard ac97 */
    ret = snd_ac97_bus(
        (*(*component).card).snd_card,
        0,
        soc_ac97_ops,
        ptr::null_mut(),
        &mut ac97_bus,
    );
    if ret < 0 {
        return ret;
    }

    ac97_template = mem::zeroed();
    ret = snd_ac97_mixer(ac97_bus, &mut ac97_template, &mut ac97);
    if ret < 0 {
        return ret;
    }

    snd_soc_component_set_drvdata(component, ac97 as *mut c_void);

    0
}

// CONFIG_PM conditional: when CONFIG_PM is disabled, ac97_soc_suspend and
// ac97_soc_resume are NULL in the component driver.
unsafe extern "C" fn ac97_soc_suspend(component: *mut snd_soc_component) -> c_int {
    let ac97: *mut snd_ac97 = snd_soc_component_get_drvdata(component) as *mut snd_ac97;

    snd_ac97_suspend(ac97);

    0
}

unsafe extern "C" fn ac97_soc_resume(component: *mut snd_soc_component) -> c_int {
    let ac97: *mut snd_ac97 = snd_soc_component_get_drvdata(component) as *mut snd_ac97;

    snd_ac97_resume(ac97);

    0
}

static soc_component_dev_ac97: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ac97_soc_probe),
    suspend: Some(ac97_soc_suspend),
    resume: Some(ac97_soc_resume),
    dapm_widgets: unsafe { ac97_widgets.as_ptr() },
    num_dapm_widgets: 2,
    dapm_routes: ac97_routes.as_ptr(),
    num_dapm_routes: 2,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn ac97_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_ac97,
        &mut ac97_dai,
        1,
    )
}

// CONFIG_OF conditional.
static ac97_codec_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: REALTEK_ALC203.as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, ac97_codec_of_match);

static mut ac97_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: AC97_CODEC.as_ptr() as *const c_char,
        of_match_table: ac97_codec_of_match.as_ptr(),
    },

    probe: Some(ac97_probe),
};

unsafe extern "C" fn ac97_module_platform_driver_init() {
    module_platform_driver(&mut ac97_codec_driver);
}

// MODULE_DESCRIPTION("Soc Generic AC97 driver");
// MODULE_AUTHOR("Liam Girdwood");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:ac97-codec");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
