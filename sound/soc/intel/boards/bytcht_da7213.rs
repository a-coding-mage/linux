// SPDX-License-Identifier: GPL-2.0-only
/*
 *  bytcht-da7213.c - ASoc Machine driver for Intel Baytrail and
 *             Cherrytrail-based platforms, with Dialog DA7213 codec
 *
 *  Copyright (C) 2017 Intel Corporation
 *  Author: Pierre-Louis Bossart <pierre-louis.bossart@linux.intel.com>
 *
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_interval {
    min: c_uint,
    max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_soc_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    nonatomic: bool,
    dynamic: c_uint,
    ops: *const snd_soc_ops,
    playback_only: c_uint,
    id: c_int,
    no_pcm: c_uint,
    dai_fmt: c_uint,
    be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    cpus: *mut snd_soc_dai_link_component,
    num_cpus: c_uint,
    codecs: *mut snd_soc_dai_link_component,
    num_codecs: c_uint,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    name: *const c_char,
    driver_name: *const c_char,
    owner: *mut module,
    dev: *mut device,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    controls: *const snd_kcontrol_new,
    num_controls: c_int,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
}

#[repr(C)]
pub struct device_driver {
    pm: *const dev_pm_ops,
    name: *const c_char,
}

#[repr(C)]
pub struct device {
    platform_data: *mut c_void,
    driver: *mut device_driver,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    platform: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    id: *const c_char,
    mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const DA7213_CLKSRC_MCLK: c_int = 0;
const DA7213_SYSCLK_PLL_SRM: c_int = 0;
const DA7213_SYSCLK_MCLK: c_int = 0;
const DA7213_PLL_FREQ_OUT_98304000: c_uint = 98_304_000;
const EIO: c_int = 5;
const ENOENT: c_int = 2;
const MERR_DPCM_AUDIO: usize = 0;
const MERR_DPCM_DEEP_BUFFER: usize = 1;
const SND_ACPI_I2C_ID_LEN: usize = 32;

extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        param: c_int,
    ) -> *mut snd_interval;
    fn params_set_format(params: *mut snd_pcm_hw_params, format: c_int);
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_hw_constraint_single(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn acpi_dev_get_first_match_dev(
        hid: *const c_char,
        uid: *const c_char,
        hrv: c_int,
    ) -> *mut acpi_device;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn acpi_dev_name(adev: *mut acpi_device) -> *const c_char;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn snd_soc_fixup_dai_links_platform_name(
        card: *mut snd_soc_card,
        platform_name: *const c_char,
    ) -> c_int;
    fn snd_soc_acpi_sof_parent(dev: *mut device) -> bool;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
}

/* SOC_DAPM_PIN_SWITCH entries; concrete initializer is provided by ASoC macros in C. */
static controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

/* SND_SOC_DAPM_* widget entries; concrete initializer is provided by ASoC macros in C. */
static dapm_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static audio_map: [snd_soc_dapm_route; 12] = [
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AUXL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Aux In\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AUXR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Aux In\0".as_ptr() as *const c_char },
    /* Assume Mic1 is linked to Headset and Mic2 to on-board mic */
    snd_soc_dapm_route { sink: b"MIC1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MIC2\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Mic\0".as_ptr() as *const c_char },
    /* SOC-codec link */
    snd_soc_dapm_route { sink: b"ssp2 Tx\0".as_ptr() as *const c_char, control: ptr::null(), source: b"codec_out0\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ssp2 Tx\0".as_ptr() as *const c_char, control: ptr::null(), source: b"codec_out1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"codec_in0\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ssp2 Rx\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"codec_in1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ssp2 Rx\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ssp2 Tx\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ssp2 Rx\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Capture\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn codec_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut ret: c_int;
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);

    /* The DSP will convert the FE rate to 48k, stereo, 24bits */
    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;

    /* set SSP2 to 24-bit */
    params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);

    /*
     * Default mode for SSP configuration is TDM 4 slot, override config
     * with explicit setting to I2S 2ch 24-bit. The word length is set with
     * dai_set_tdm_slot() since there is no other API exposed
     */
    ret = snd_soc_dai_set_fmt(
        snd_soc_rtd_to_cpu(rtd, 0),
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_BP_FP,
    );
    if ret < 0 {
        dev_err((*rtd).dev, b"can't set format to I2S, err %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_cpu(rtd, 0), 0x3, 0x3, 2, 24);
    if ret < 0 {
        dev_err((*rtd).dev, b"can't set I2S config, err %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 48000)
}

unsafe extern "C" fn aif1_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut ret: c_int;

    ret = snd_soc_dai_set_sysclk(codec_dai, DA7213_CLKSRC_MCLK, 19200000, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*codec_dai).dev, b"can't set codec sysclk configuration\n\0".as_ptr() as *const c_char);
    }

    ret = snd_soc_dai_set_pll(codec_dai, 0, DA7213_SYSCLK_PLL_SRM, 0, DA7213_PLL_FREQ_OUT_98304000);
    if ret < 0 {
        dev_err((*codec_dai).dev, b"failed to start PLL: %d\n\0".as_ptr() as *const c_char, ret);
        return -EIO;
    }

    ret
}

unsafe extern "C" fn aif1_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let ret: c_int;

    ret = snd_soc_dai_set_pll(codec_dai, 0, DA7213_SYSCLK_MCLK, 0, 0);
    if ret < 0 {
        dev_err((*codec_dai).dev, b"failed to stop PLL: %d\n\0".as_ptr() as *const c_char, ret);
        return -EIO;
    }

    ret
}

static aif1_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(aif1_startup),
    hw_params: None,
    hw_free: None,
};

static ssp2_ops: snd_soc_ops = snd_soc_ops {
    startup: None,
    hw_params: Some(aif1_hw_params),
    hw_free: Some(aif1_hw_free),
};

static mut dummy: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
}];

static mut media: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"media-cpu-dai\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];

static mut deepbuffer: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"deepbuffer-cpu-dai\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];

static mut ssp2_port: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"ssp2-port\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];

static mut ssp2_codec: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"i2c-DLGS7213:00\0".as_ptr() as *const c_char,
    dai_name: b"da7213-hifi\0".as_ptr() as *const c_char,
}];

static mut platform: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"sst-mfld-platform\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];

static mut dailink: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: b"Audio Port\0".as_ptr() as *const c_char,
        stream_name: b"Audio\0".as_ptr() as *const c_char,
        nonatomic: true,
        dynamic: 1,
        ops: &aif1_ops,
        playback_only: 0,
        id: 0,
        no_pcm: 0,
        dai_fmt: 0,
        be_hw_params_fixup: None,
        cpus: unsafe { media.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { dummy.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: b"Deep-Buffer Audio Port\0".as_ptr() as *const c_char,
        stream_name: b"Deep-Buffer Audio\0".as_ptr() as *const c_char,
        nonatomic: true,
        dynamic: 1,
        ops: &aif1_ops,
        playback_only: 1,
        id: 0,
        no_pcm: 0,
        dai_fmt: 0,
        be_hw_params_fixup: None,
        cpus: unsafe { deepbuffer.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { dummy.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
    },
    /* CODEC<->CODEC link */
    /* back ends */
    snd_soc_dai_link {
        name: b"SSP2-Codec\0".as_ptr() as *const c_char,
        stream_name: ptr::null(),
        nonatomic: false,
        dynamic: 0,
        ops: &ssp2_ops,
        playback_only: 0,
        id: 0,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        be_hw_params_fixup: Some(codec_fixup),
        cpus: unsafe { ssp2_port.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { ssp2_codec.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
    },
];

/* use space before codec name to simplify card ID, and simplify driver name */
const SOF_CARD_NAME: *const c_char = b"bytcht da7213\0".as_ptr() as *const c_char; /* card name will be 'sof-bytcht da7213' */
const SOF_DRIVER_NAME: *const c_char = b"SOF\0".as_ptr() as *const c_char;

const CARD_NAME: *const c_char = b"bytcht-da7213\0".as_ptr() as *const c_char;
const DRIVER_NAME: *const c_char = ptr::null(); /* card name will be used for driver name */

/* SoC card */
static mut bytcht_da7213_card: snd_soc_card = snd_soc_card {
    name: CARD_NAME,
    driver_name: DRIVER_NAME,
    owner: unsafe { THIS_MODULE },
    dev: ptr::null_mut(),
    dai_link: unsafe { dailink.as_mut_ptr() },
    num_links: 3,
    controls: controls.as_ptr(),
    num_controls: 4,
    dapm_widgets: dapm_widgets.as_ptr(),
    num_dapm_widgets: 4,
    dapm_routes: audio_map.as_ptr(),
    num_dapm_routes: 12,
};

static mut codec_name: [c_char; SND_ACPI_I2C_ID_LEN] = [0; SND_ACPI_I2C_ID_LEN];

unsafe extern "C" fn bytcht_da7213_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card;
    let mach: *mut snd_soc_acpi_mach;
    let platform_name: *const c_char;
    let adev: *mut acpi_device;
    let sof_parent: bool;
    let mut dai_index: c_int = 0;
    let mut ret_val: c_int = 0;
    let mut i: usize;

    mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;
    card = &mut bytcht_da7213_card;
    (*card).dev = &mut (*pdev).dev;

    /* fix index of codec dai */
    i = 0;
    while i < dailink.len() {
        if dailink[i].num_codecs != 0
            && strcmp((*dailink[i].codecs).name, b"i2c-DLGS7213:00\0".as_ptr() as *const c_char) == 0
        {
            dai_index = i as c_int;
            break;
        }
        i += 1;
    }

    /* fixup codec name based on HID */
    adev = acpi_dev_get_first_match_dev((*mach).id, ptr::null(), -1);
    if !adev.is_null() {
        snprintf(
            codec_name.as_mut_ptr(),
            codec_name.len(),
            b"i2c-%s\0".as_ptr() as *const c_char,
            acpi_dev_name(adev),
        );
        (*dailink[dai_index as usize].codecs).name = codec_name.as_mut_ptr();
    } else {
        dev_err(
            &mut (*pdev).dev,
            b"Error cannot find '%s' dev\n\0".as_ptr() as *const c_char,
            (*mach).id,
        );
        return -ENOENT;
    }

    acpi_dev_put(adev);

    /* override platform name, if required */
    platform_name = (*mach).mach_params.platform;

    ret_val = snd_soc_fixup_dai_links_platform_name(card, platform_name);
    if ret_val != 0 {
        return ret_val;
    }

    sof_parent = snd_soc_acpi_sof_parent(&mut (*pdev).dev);

    /* set card and driver name */
    if sof_parent {
        bytcht_da7213_card.name = SOF_CARD_NAME;
        bytcht_da7213_card.driver_name = SOF_DRIVER_NAME;
    } else {
        bytcht_da7213_card.name = CARD_NAME;
        bytcht_da7213_card.driver_name = DRIVER_NAME;
    }

    /* set pm ops */
    if sof_parent {
        (*(*pdev).dev.driver).pm = &snd_soc_pm_ops;
    }

    ret_val = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret_val != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"snd_soc_register_card failed %d\n\0".as_ptr() as *const c_char,
            ret_val,
        );
        return ret_val;
    }
    platform_set_drvdata(pdev, card as *mut c_void);
    ret_val
}

static mut bytcht_da7213_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"bytcht_da7213\0".as_ptr() as *const c_char,
        pm: ptr::null(),
    },
    probe: Some(bytcht_da7213_probe),
};

/* module_platform_driver(bytcht_da7213_driver); */

/* MODULE_DESCRIPTION("ASoC Intel(R) Baytrail/Cherrytrail+DA7213 Machine driver"); */
/* MODULE_AUTHOR("Pierre-Louis Bossart"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:bytcht_da7213"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
