// SPDX-License-Identifier: GPL-2.0
//
// Lochnagar sound card driver
//
// Copyright (c) 2017-2019 Cirrus Logic, Inc. and
//                         Cirrus Logic International Semiconductor Ltd.
//
// Author: Charles Keepax <ckeepax@opensource.cirrus.com>
//         Piotr Stankiewicz <piotrs@opensource.cirrus.com>

// C dependencies:
// linux/clk.h, linux/module.h, sound/soc.h
// linux/mfd/lochnagar.h, linux/mfd/lochnagar1_regs.h,
// linux/mfd/lochnagar2_regs.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type u64 = u64;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_HW_PARAM_FRAME_BITS: c_uint = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_uint = 2;
const SNDRV_PCM_RATE_KNOT: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 0;

const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_CLOCK_MASK: c_uint = 0;
const SND_SOC_POSSIBLE_DAIFMT_I2S: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_NB_NF: u64 = 0;

#[repr(C)]
struct clk {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
    dev: *mut device,
}

#[repr(C)]
struct snd_interval {
    min: c_uint,
    max: c_uint,
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_rule {
    var: c_uint,
    deps: [c_uint; 5],
}

#[repr(C)]
struct snd_soc_dapm_widget {
    name: *const c_char,
    event: *const core::ffi::c_void,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *const c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    auto_selectable_formats: *const u64,
    num_auto_selectable_formats: c_uint,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: bool,
    symmetric_sample_bits: bool,
}

#[repr(C)]
struct snd_soc_component_driver {
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
struct lochnagar_sc_priv {
    mclk: *mut clk,
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut core::ffi::c_void;
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_uint) -> *mut snd_interval;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        func: Option<unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int>,
        private: *mut core::ffi::c_void,
        dep: c_uint,
        sentinel: c_int,
    ) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut core::ffi::c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

const fn snd_soc_dapm_line(name: *const c_char) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget {
        name,
        event: core::ptr::null(),
    }
}

static lochnagar_sc_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_line(c"Line Jack".as_ptr()),
    snd_soc_dapm_line(c"USB Audio".as_ptr()),
];

static lochnagar_sc_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route {
        sink: c"Line Jack".as_ptr(),
        control: core::ptr::null(),
        source: c"AIF1 Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"AIF1 Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"Line Jack".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"USB Audio".as_ptr(),
        control: core::ptr::null(),
        source: c"USB1 Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"USB Audio".as_ptr(),
        control: core::ptr::null(),
        source: c"USB2 Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"USB1 Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"USB Audio".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"USB2 Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"USB Audio".as_ptr(),
    },
];

static lochnagar_sc_chan_vals: [c_uint; 2] = [4, 8];

static lochnagar_sc_chan_constraint: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list {
        count: lochnagar_sc_chan_vals.len() as c_uint,
        list: lochnagar_sc_chan_vals.as_ptr(),
    };

static lochnagar_sc_rate_vals: [c_uint; 11] = [
    8000, 16000, 24000, 32000, 48000, 96000, 192000, 22050, 44100, 88200, 176400,
];

static lochnagar_sc_rate_constraint: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list {
        count: lochnagar_sc_rate_vals.len() as c_uint,
        list: lochnagar_sc_rate_vals.as_ptr(),
    };

unsafe extern "C" fn lochnagar_sc_hw_rule_rate(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let range = snd_interval {
        min: 8000,
        max: 24576000u32 / (*hw_param_interval(params, (*rule).deps[0] as c_uint)).max,
    };

    snd_interval_refine(hw_param_interval(params, (*rule).var), &range)
}

unsafe extern "C" fn lochnagar_sc_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let comp = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(comp) as *mut lochnagar_sc_priv;
    let mut ret: c_int;

    ret = snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &lochnagar_sc_rate_constraint,
    );
    if ret != 0 {
        return ret;
    }

    snd_pcm_hw_rule_add(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        Some(lochnagar_sc_hw_rule_rate),
        priv_ as *mut core::ffi::c_void,
        SNDRV_PCM_HW_PARAM_FRAME_BITS,
        -1,
    )
}

unsafe extern "C" fn lochnagar_sc_line_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let comp = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(comp) as *mut lochnagar_sc_priv;
    let mut ret: c_int;

    ret = clk_prepare_enable((*priv_).mclk);
    if ret < 0 {
        dev_err((*dai).dev, c"Failed to enable MCLK: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = lochnagar_sc_startup(substream, dai);
    if ret != 0 {
        return ret;
    }

    snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        &lochnagar_sc_chan_constraint,
    )
}

unsafe extern "C" fn lochnagar_sc_line_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let comp = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(comp) as *mut lochnagar_sc_priv;

    clk_disable_unprepare((*priv_).mclk);
}

unsafe extern "C" fn lochnagar_sc_check_fmt(
    _dai: *mut snd_soc_dai,
    fmt: c_uint,
    mut tar: c_uint,
) -> c_int {
    tar |= SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF;

    if (fmt & !SND_SOC_DAIFMT_CLOCK_MASK) != tar {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn lochnagar_sc_set_line_fmt(
    dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    lochnagar_sc_check_fmt(dai, fmt, SND_SOC_DAIFMT_CBC_CFC)
}

unsafe extern "C" fn lochnagar_sc_set_usb_fmt(
    dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    lochnagar_sc_check_fmt(dai, fmt, SND_SOC_DAIFMT_CBP_CFP)
}

static lochnagar_selectable_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_NB_NF;

static lochnagar_sc_line_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(lochnagar_sc_line_startup),
    shutdown: Some(lochnagar_sc_line_shutdown),
    set_fmt: Some(lochnagar_sc_set_line_fmt),
    auto_selectable_formats: &lochnagar_selectable_formats,
    num_auto_selectable_formats: 1,
};

static lochnagar_sc_usb_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(lochnagar_sc_startup),
    shutdown: None,
    set_fmt: Some(lochnagar_sc_set_usb_fmt),
    auto_selectable_formats: &lochnagar_selectable_formats,
    num_auto_selectable_formats: 1,
};

static mut lochnagar_sc_dai: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        name: c"lochnagar-line".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"AIF1 Playback".as_ptr(),
            channels_min: 4,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_KNOT,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"AIF1 Capture".as_ptr(),
            channels_min: 4,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_KNOT,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
        },
        ops: &lochnagar_sc_line_ops,
        symmetric_rate: true,
        symmetric_sample_bits: true,
    },
    snd_soc_dai_driver {
        name: c"lochnagar-usb1".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"USB1 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_KNOT,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"USB1 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_KNOT,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
        },
        ops: &lochnagar_sc_usb_ops,
        symmetric_rate: true,
        symmetric_sample_bits: true,
    },
    snd_soc_dai_driver {
        name: c"lochnagar-usb2".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"USB2 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_KNOT,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"USB2 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_KNOT,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
        },
        ops: &lochnagar_sc_usb_ops,
        symmetric_rate: true,
        symmetric_sample_bits: true,
    },
];

static lochnagar_sc_driver: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: lochnagar_sc_widgets.as_ptr(),
    num_dapm_widgets: lochnagar_sc_widgets.len() as c_uint,
    dapm_routes: lochnagar_sc_routes.as_ptr(),
    num_dapm_routes: lochnagar_sc_routes.len() as c_uint,

    endianness: 1,
};

unsafe extern "C" fn lochnagar_sc_probe(pdev: *mut platform_device) -> c_int {
    let priv_: *mut lochnagar_sc_priv;
    let ret: c_int;

    priv_ = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<lochnagar_sc_priv>(),
        GFP_KERNEL,
    ) as *mut lochnagar_sc_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).mclk = devm_clk_get(&mut (*pdev).dev, c"mclk".as_ptr());
    if IS_ERR((*priv_).mclk as *const core::ffi::c_void) {
        ret = PTR_ERR((*priv_).mclk as *const core::ffi::c_void);
        dev_err(&mut (*pdev).dev, c"Failed to get MCLK: %d\n".as_ptr(), ret);
        return ret;
    }

    platform_set_drvdata(pdev, priv_ as *mut core::ffi::c_void);

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &lochnagar_sc_driver,
        lochnagar_sc_dai.as_mut_ptr(),
        lochnagar_sc_dai.len() as c_int,
    )
}

static lochnagar_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"cirrus,lochnagar2-soundcard".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, lochnagar_of_match);

static mut lochnagar_sc_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"lochnagar-soundcard".as_ptr(),
        of_match_table: lochnagar_of_match.as_ptr(),
    },

    probe: Some(lochnagar_sc_probe),
};
// module_platform_driver(lochnagar_sc_codec_driver);

// MODULE_DESCRIPTION("ASoC Lochnagar Sound Card Driver");
// MODULE_AUTHOR("Piotr Stankiewicz <piotrs@opensource.cirrus.com>");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:lochnagar-soundcard");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
