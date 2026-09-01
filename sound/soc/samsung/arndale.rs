// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2014, Insignal Co., Ltd.
//
//  Author: Claude <claude@insginal.co.kr>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

const EINVAL: c_int = 22;

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int)
        -> *mut device_node;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
}

const SAMSUNG_I2S_CDCLK: c_int = 2;
const SAMSUNG_I2S_RCLKSRC_0: c_int = 4;
const SND_SOC_CLOCK_OUT: c_int = 1;
const SND_SOC_CLOCK_IN: c_int = 0;
const WM8994_SYSCLK_MCLK1: c_int = 1;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0x3000;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x4000;

unsafe extern "C" fn arndale_rt5631_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let rfs: c_int;
    let mut ret: c_int;
    let rclk: c_ulong;

    rfs = 256;

    rclk = (params_rate(params) as c_ulong).wrapping_mul(rfs as c_ulong);

    ret = snd_soc_dai_set_sysclk(cpu_dai, SAMSUNG_I2S_CDCLK, 0, SND_SOC_CLOCK_OUT);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(cpu_dai, SAMSUNG_I2S_RCLKSRC_0, 0, SND_SOC_CLOCK_OUT);

    if ret < 0 {
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, 0, rclk as c_uint, SND_SOC_CLOCK_OUT);
    if ret < 0 {
        return ret;
    }

    0
}

static arndale_rt5631_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(arndale_rt5631_hw_params),
};

unsafe extern "C" fn arndale_wm1811_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let rfs: c_uint;
    let rclk: c_uint;

    /* Ensure AIF1CLK is >= 3 MHz for optimal performance */
    if params_width(params) == 24 {
        rfs = 384;
    } else if params_rate(params) == 8000 || params_rate(params) == 11025 {
        rfs = 512;
    } else {
        rfs = 256;
    }

    rclk = params_rate(params).wrapping_mul(rfs);

    /*
     * We add 1 to the frequency value to ensure proper EPLL setting
     * for each audio sampling rate (see epll_24mhz_tbl in drivers/clk/
     * samsung/clk-exynos5250.c for list of available EPLL rates).
     * The CODEC uses clk API and the value will be rounded hence the MCLK1
     * clock's frequency will still be exact multiple of the sample rate.
     */
    snd_soc_dai_set_sysclk(
        codec_dai,
        WM8994_SYSCLK_MCLK1,
        rclk.wrapping_add(1),
        SND_SOC_CLOCK_IN,
    )
}

static arndale_wm1811_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(arndale_wm1811_hw_params),
};

static mut rt5631_hifi_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: core::ptr::null(),
    dai_name: core::ptr::null(),
    of_node: core::ptr::null_mut(),
}];

static mut rt5631_hifi_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: core::ptr::null(),
    dai_name: b"rt5631-aif1\0".as_ptr() as *const c_char,
    of_node: core::ptr::null_mut(),
}];

static mut rt5631_hifi_platforms: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component {
        name: core::ptr::null(),
        dai_name: core::ptr::null(),
        of_node: core::ptr::null_mut(),
    }];

static mut arndale_rt5631_dai: [snd_soc_dai_link; 1] = [snd_soc_dai_link {
    name: b"RT5631 HiFi\0".as_ptr() as *const c_char,
    stream_name: b"Primary\0".as_ptr() as *const c_char,
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
    ops: &arndale_rt5631_ops,
    cpus: unsafe { rt5631_hifi_cpus.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { rt5631_hifi_codecs.as_mut_ptr() },
    num_codecs: 1,
    platforms: unsafe { rt5631_hifi_platforms.as_mut_ptr() },
    num_platforms: 1,
}];

static mut wm1811_hifi_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: core::ptr::null(),
    dai_name: core::ptr::null(),
    of_node: core::ptr::null_mut(),
}];

static mut wm1811_hifi_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: core::ptr::null(),
    dai_name: b"wm8994-aif1\0".as_ptr() as *const c_char,
    of_node: core::ptr::null_mut(),
}];

static mut wm1811_hifi_platforms: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component {
        name: core::ptr::null(),
        dai_name: core::ptr::null(),
        of_node: core::ptr::null_mut(),
    }];

static mut arndale_wm1811_dai: [snd_soc_dai_link; 1] = [snd_soc_dai_link {
    name: b"WM1811 HiFi\0".as_ptr() as *const c_char,
    stream_name: b"Primary\0".as_ptr() as *const c_char,
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
    ops: &arndale_wm1811_ops,
    cpus: unsafe { wm1811_hifi_cpus.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { wm1811_hifi_codecs.as_mut_ptr() },
    num_codecs: 1,
    platforms: unsafe { wm1811_hifi_platforms.as_mut_ptr() },
    num_platforms: 1,
}];

static mut arndale_rt5631: snd_soc_card = snd_soc_card {
    name: b"Arndale RT5631\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { arndale_rt5631_dai.as_mut_ptr() },
    num_links: 1,
    dev: core::ptr::null_mut(),
};

static mut arndale_wm1811: snd_soc_card = snd_soc_card {
    name: b"Arndale WM1811\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { arndale_wm1811_dai.as_mut_ptr() },
    num_links: 1,
    dev: core::ptr::null_mut(),
};

unsafe extern "C" fn arndale_put_of_nodes(card: *mut snd_soc_card) {
    let mut i: c_int = 0;

    while i < (*card).num_links {
        let dai_link: *mut snd_soc_dai_link = (*card).dai_link.offset(i as isize);
        of_node_put((*(*dai_link).cpus).of_node);
        of_node_put((*(*dai_link).codecs).of_node);
        i += 1;
    }
}

unsafe extern "C" fn arndale_audio_probe(pdev: *mut platform_device) -> c_int {
    let np: *mut device_node = (*pdev).dev.of_node;
    let card: *mut snd_soc_card;
    let dai_link: *mut snd_soc_dai_link;
    let mut ret: c_int;

    card = of_device_get_match_data(&mut (*pdev).dev) as *mut snd_soc_card;
    (*card).dev = &mut (*pdev).dev;
    dai_link = (*card).dai_link;

    (*(*dai_link).cpus).of_node = of_parse_phandle(np, b"samsung,audio-cpu\0".as_ptr() as *const c_char, 0);
    if (*(*dai_link).cpus).of_node.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"Property 'samsung,audio-cpu' missing or invalid\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    if (*(*dai_link).platforms).name.is_null() {
        (*(*dai_link).platforms).of_node = (*(*dai_link).cpus).of_node;
    }

    (*(*dai_link).codecs).of_node =
        of_parse_phandle(np, b"samsung,audio-codec\0".as_ptr() as *const c_char, 0);
    if (*(*dai_link).codecs).of_node.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"Property 'samsung,audio-codec' missing or invalid\n\0".as_ptr() as *const c_char,
        );
        ret = -EINVAL;
        arndale_put_of_nodes(card);
        return ret;
    }

    ret = devm_snd_soc_register_card((*card).dev, card);
    if ret != 0 {
        dev_err_probe(
            &mut (*pdev).dev,
            ret,
            b"snd_soc_register_card() failed\n\0".as_ptr() as *const c_char,
        );
        arndale_put_of_nodes(card);
        return ret;
    }
    0
}

unsafe extern "C" fn arndale_audio_remove(pdev: *mut platform_device) {
    let card: *mut snd_soc_card = platform_get_drvdata(pdev) as *mut snd_soc_card;

    arndale_put_of_nodes(card);
}

static arndale_audio_of_match: [of_device_id; 4] = [
    of_device_id {
        compatible: b"samsung,arndale-rt5631\0".as_ptr() as *const c_char,
        data: unsafe { &arndale_rt5631 as *const snd_soc_card as *const c_void },
    },
    of_device_id {
        compatible: b"samsung,arndale-alc5631\0".as_ptr() as *const c_char,
        data: unsafe { &arndale_rt5631 as *const snd_soc_card as *const c_void },
    },
    of_device_id {
        compatible: b"samsung,arndale-wm1811\0".as_ptr() as *const c_char,
        data: unsafe { &arndale_wm1811 as *const snd_soc_card as *const c_void },
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, arndale_audio_of_match);

static mut arndale_audio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"arndale-audio\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops },
        of_match_table: arndale_audio_of_match.as_ptr(),
    },
    probe: Some(arndale_audio_probe),
    remove: Some(arndale_audio_remove),
};

// module_platform_driver(arndale_audio_driver);
// MODULE_AUTHOR("Claude <claude@insignal.co.kr>");
// MODULE_DESCRIPTION("ALSA SoC Driver for Arndale Board");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
