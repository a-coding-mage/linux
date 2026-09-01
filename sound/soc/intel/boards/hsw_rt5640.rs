// SPDX-License-Identifier: GPL-2.0-only
/*
 * Sound card driver for Intel Haswell Lynx Point with Realtek 5640
 *
 * Copyright (C) 2013, Intel Corporation
 */

// C includes translated as external dependency intent:
// linux/module.h, linux/platform_device.h, sound/core.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-acpi.h, ../../codecs/rt5640.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub kind: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub id: c_int,
    pub nonatomic: c_uint,
    pub dynamic: c_uint,
    pub trigger: [c_uint; 2],
    pub playback_only: c_uint,
    pub capture_only: c_uint,
    pub no_pcm: c_uint,
    pub dai_fmt: c_uint,
    pub ignore_pmdown_time: c_uint,
    pub be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub cpus: *const snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *const snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *const snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub fully_routed: bool,
}

#[repr(C)]
pub struct mach_params {
    pub platform: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub mach_params: mach_params,
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
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: dev_pm_ops;

    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn params_set_format(params: *mut snd_pcm_hw_params, format: c_int) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn dev_get_platdata(dev: *const device) -> *mut c_void;
    fn snd_soc_fixup_dai_links_platform_name(
        card: *mut snd_soc_card,
        platform_name: *const c_char,
    ) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 1;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const RT5640_SCLK_S_MCLK: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DPCM_TRIGGER_POST: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1 << 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 1 << 1;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 1 << 2;

const HEADPHONES: *const c_char = b"Headphones\0".as_ptr() as *const c_char;
const MIC: *const c_char = b"Mic\0".as_ptr() as *const c_char;
const HPOR: *const c_char = b"HPOR\0".as_ptr() as *const c_char;
const HPOL: *const c_char = b"HPOL\0".as_ptr() as *const c_char;
const IN2P: *const c_char = b"IN2P\0".as_ptr() as *const c_char;
const SSP0_CODEC_IN: *const c_char = b"SSP0 CODEC IN\0".as_ptr() as *const c_char;
const AIF1_CAPTURE: *const c_char = b"AIF1 Capture\0".as_ptr() as *const c_char;
const AIF1_PLAYBACK: *const c_char = b"AIF1 Playback\0".as_ptr() as *const c_char;
const SSP0_CODEC_OUT: *const c_char = b"SSP0 CODEC OUT\0".as_ptr() as *const c_char;

static card_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget {
        name: HEADPHONES,
        kind: ptr::null(),
    },
    snd_soc_dapm_widget {
        name: MIC,
        kind: ptr::null(),
    },
];

static card_routes: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route {
        sink: HEADPHONES,
        control: ptr::null(),
        source: HPOR,
    },
    snd_soc_dapm_route {
        sink: HEADPHONES,
        control: ptr::null(),
        source: HPOL,
    },
    snd_soc_dapm_route {
        sink: IN2P,
        control: ptr::null(),
        source: MIC,
    },
    /*
     * CODEC BE connections
     */
    snd_soc_dapm_route {
        sink: SSP0_CODEC_IN,
        control: ptr::null(),
        source: AIF1_CAPTURE,
    },
    snd_soc_dapm_route {
        sink: AIF1_PLAYBACK,
        control: ptr::null(),
        source: SSP0_CODEC_OUT,
    },
];

unsafe extern "C" fn codec_link_hw_params_fixup(
    _rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);

    /*
     * The ADSP will convert the FE rate to 48k, stereo.
     */
    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;
    /*
     * Set SSP0 to 16 bit.
     */
    params_set_format(params, SNDRV_PCM_FORMAT_S16_LE);

    0
}

unsafe extern "C" fn codec_link_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let ret: c_int;

    ret = snd_soc_dai_set_sysclk(codec_dai, RT5640_SCLK_S_MCLK, 12288000, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err(
            (*rtd).dev,
            b"set codec sysclk failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    /*
     * Set correct codec filter for DAI format and clock config.
     */
    snd_soc_component_update_bits((*codec_dai).component, 0x83, 0xffff, 0x8000);

    ret
}

static codec_link_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(codec_link_hw_params),
};

static system: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"System Pin\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];
static offload0: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"Offload0 Pin\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];
static offload1: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"Offload1 Pin\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];
static loopback: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"Loopback Pin\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];

static dummy: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
}];
static codec: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"i2c-INT33CA:00\0".as_ptr() as *const c_char,
    dai_name: b"rt5640-aif1\0".as_ptr() as *const c_char,
}];
static platform: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"haswell-pcm-audio\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];
static ssp0_port: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"ssp0-port\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];

static mut card_dai_links: [snd_soc_dai_link; 5] = [
    /*
     * Front End DAI links
     */
    snd_soc_dai_link {
        name: b"System\0".as_ptr() as *const c_char,
        stream_name: b"System Playback/Capture\0".as_ptr() as *const c_char,
        id: 0,
        nonatomic: 1,
        dynamic: 1,
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        playback_only: 0,
        capture_only: 0,
        no_pcm: 0,
        dai_fmt: 0,
        ignore_pmdown_time: 0,
        be_hw_params_fixup: None,
        ops: ptr::null(),
        cpus: system.as_ptr(),
        num_cpus: system.len() as c_uint,
        codecs: dummy.as_ptr(),
        num_codecs: dummy.len() as c_uint,
        platforms: platform.as_ptr(),
        num_platforms: platform.len() as c_uint,
    },
    snd_soc_dai_link {
        name: b"Offload0\0".as_ptr() as *const c_char,
        stream_name: b"Offload0 Playback\0".as_ptr() as *const c_char,
        id: 0,
        nonatomic: 1,
        dynamic: 1,
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        playback_only: 1,
        capture_only: 0,
        no_pcm: 0,
        dai_fmt: 0,
        ignore_pmdown_time: 0,
        be_hw_params_fixup: None,
        ops: ptr::null(),
        cpus: offload0.as_ptr(),
        num_cpus: offload0.len() as c_uint,
        codecs: dummy.as_ptr(),
        num_codecs: dummy.len() as c_uint,
        platforms: platform.as_ptr(),
        num_platforms: platform.len() as c_uint,
    },
    snd_soc_dai_link {
        name: b"Offload1\0".as_ptr() as *const c_char,
        stream_name: b"Offload1 Playback\0".as_ptr() as *const c_char,
        id: 0,
        nonatomic: 1,
        dynamic: 1,
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        playback_only: 1,
        capture_only: 0,
        no_pcm: 0,
        dai_fmt: 0,
        ignore_pmdown_time: 0,
        be_hw_params_fixup: None,
        ops: ptr::null(),
        cpus: offload1.as_ptr(),
        num_cpus: offload1.len() as c_uint,
        codecs: dummy.as_ptr(),
        num_codecs: dummy.len() as c_uint,
        platforms: platform.as_ptr(),
        num_platforms: platform.len() as c_uint,
    },
    snd_soc_dai_link {
        name: b"Loopback\0".as_ptr() as *const c_char,
        stream_name: b"Loopback\0".as_ptr() as *const c_char,
        id: 0,
        nonatomic: 1,
        dynamic: 1,
        trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
        playback_only: 0,
        capture_only: 1,
        no_pcm: 0,
        dai_fmt: 0,
        ignore_pmdown_time: 0,
        be_hw_params_fixup: None,
        ops: ptr::null(),
        cpus: loopback.as_ptr(),
        num_cpus: loopback.len() as c_uint,
        codecs: dummy.as_ptr(),
        num_codecs: dummy.len() as c_uint,
        platforms: platform.as_ptr(),
        num_platforms: platform.len() as c_uint,
    },
    /*
     * Back End DAI links
     */
    snd_soc_dai_link {
        /*
         * SSP0 - Codec
         */
        name: b"Codec\0".as_ptr() as *const c_char,
        stream_name: ptr::null(),
        id: 0,
        nonatomic: 1,
        dynamic: 0,
        trigger: [0, 0],
        playback_only: 0,
        capture_only: 0,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        ignore_pmdown_time: 1,
        be_hw_params_fixup: Some(codec_link_hw_params_fixup),
        ops: &codec_link_ops,
        cpus: ssp0_port.as_ptr(),
        num_cpus: ssp0_port.len() as c_uint,
        codecs: codec.as_ptr(),
        num_codecs: codec.len() as c_uint,
        platforms: platform.as_ptr(),
        num_platforms: platform.len() as c_uint,
    },
];

static mut hsw_rt5640_card: snd_soc_card = snd_soc_card {
    name: b"haswell-rt5640\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    dev: ptr::null_mut(),
    dai_link: ptr::null_mut(),
    num_links: 5,
    dapm_widgets: card_widgets.as_ptr(),
    num_dapm_widgets: 2,
    dapm_routes: card_routes.as_ptr(),
    num_dapm_routes: 5,
    fully_routed: true,
};

unsafe extern "C" fn hsw_rt5640_probe(pdev: *mut platform_device) -> c_int {
    let mach: *mut snd_soc_acpi_mach;
    let dev = &mut (*pdev).dev as *mut device;
    let ret: c_int;

    hsw_rt5640_card.dev = dev;
    hsw_rt5640_card.owner = THIS_MODULE;
    hsw_rt5640_card.dai_link = card_dai_links.as_mut_ptr();
    mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;

    ret = snd_soc_fixup_dai_links_platform_name(
        &mut hsw_rt5640_card,
        (*mach).mach_params.platform,
    );
    if ret != 0 {
        return ret;
    }

    devm_snd_soc_register_card(dev, &mut hsw_rt5640_card)
}

static mut hsw_rt5640_driver: platform_driver = platform_driver {
    probe: Some(hsw_rt5640_probe),
    driver: device_driver {
        name: b"hsw_rt5640\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops },
    },
};

// module_platform_driver(hsw_rt5640_driver)

// MODULE_AUTHOR("Liam Girdwood, Xingchao Wang");
// MODULE_DESCRIPTION("Sound card driver for Intel Haswell Lynx Point with Realtek 5640");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:hsw_rt5640");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
