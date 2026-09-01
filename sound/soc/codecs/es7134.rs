// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

/*
 * C includes translated as external dependencies:
 * <linux/of_platform.h>, <linux/module.h>, <sound/soc.h>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const EINVAL: c_int = 22;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;

const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_NOPM: c_int = -1;

const SNDRV_PCM_RATE_8000_48000: c_uint = 0;
const SNDRV_PCM_RATE_88200: c_uint = 0;
const SNDRV_PCM_RATE_96000: c_uint = 0;
const SNDRV_PCM_RATE_176400: c_uint = 0;
const SNDRV_PCM_RATE_192000: c_uint = 0;

const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S18_3LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_3LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

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
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_to_dapm(c: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_component_get_drvdata(c: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_uint,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

/*
 * The everest 7134 is a very simple DA converter with no register
 */

#[repr(C)]
struct es7134_clock_mode {
    rate_min: c_uint,
    rate_max: c_uint,
    mclk_fs: *const c_uint,
    mclk_fs_num: c_uint,
}

#[repr(C)]
struct es7134_chip {
    dai_drv: *mut snd_soc_dai_driver,
    modes: *const es7134_clock_mode,
    mode_num: c_uint,
    extra_widgets: *const snd_soc_dapm_widget,
    extra_widget_num: c_uint,
    extra_routes: *const snd_soc_dapm_route,
    extra_route_num: c_uint,
}

#[repr(C)]
struct es7134_data {
    mclk: c_uint,
    chip: *const es7134_chip,
}

unsafe extern "C" fn es7134_check_mclk(
    dai: *mut snd_soc_dai,
    priv_: *mut es7134_data,
    rate: c_uint,
) -> c_int {
    let mfs: c_uint = (*priv_).mclk / rate;
    let mut i: c_int = 0;

    while i < (*(*priv_).chip).mode_num as c_int {
        let mode: *const es7134_clock_mode = (*(*priv_).chip).modes.add(i as usize);

        if rate < (*mode).rate_min || rate > (*mode).rate_max {
            i += 1;
            continue;
        }

        let mut j: c_int = 0;
        while j < (*mode).mclk_fs_num as c_int {
            if *(*mode).mclk_fs.add(j as usize) == mfs {
                return 0;
            }
            j += 1;
        }

        dev_err(
            (*dai).dev,
            c"unsupported mclk_fs %u for rate %u\n".as_ptr(),
            mfs,
            rate,
        );
        return -EINVAL;
    }

    /* should not happen */
    dev_err((*dai).dev, c"unsupported rate: %u\n".as_ptr(), rate);
    -EINVAL
}

unsafe extern "C" fn es7134_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_: *mut es7134_data = snd_soc_dai_get_drvdata(dai) as *mut es7134_data;

    /* mclk has not been provided, assume it is OK */
    if (*priv_).mclk == 0 {
        return 0;
    }

    es7134_check_mclk(dai, priv_, params_rate(params))
}

unsafe extern "C" fn es7134_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let priv_: *mut es7134_data = snd_soc_dai_get_drvdata(dai) as *mut es7134_data;

    if dir == SND_SOC_CLOCK_IN && clk_id == 0 {
        (*priv_).mclk = freq;
        return 0;
    }

    -ENOTSUPP
}

unsafe extern "C" fn es7134_set_fmt(codec_dai: *mut snd_soc_dai, mut fmt: c_uint) -> c_int {
    fmt &= SND_SOC_DAIFMT_FORMAT_MASK | SND_SOC_DAIFMT_INV_MASK | SND_SOC_DAIFMT_MASTER_MASK;

    if fmt != (SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC) {
        dev_err((*codec_dai).dev, c"Invalid DAI format\n".as_ptr());
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn es7134_component_probe(c: *mut snd_soc_component) -> c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(c);
    let priv_: *mut es7134_data = snd_soc_component_get_drvdata(c) as *mut es7134_data;
    let chip: *const es7134_chip = (*priv_).chip;
    let mut ret: c_int;

    if (*chip).extra_widget_num != 0 {
        ret = snd_soc_dapm_new_controls(dapm, (*chip).extra_widgets, (*chip).extra_widget_num);
        if ret != 0 {
            dev_err((*c).dev, c"failed to add extra widgets\n".as_ptr());
            return ret;
        }
    }

    if (*chip).extra_route_num != 0 {
        ret = snd_soc_dapm_add_routes(dapm, (*chip).extra_routes, (*chip).extra_route_num);
        if ret != 0 {
            dev_err((*c).dev, c"failed to add extra routes\n".as_ptr());
            return ret;
        }
    }

    0
}

static es7134_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(es7134_set_fmt),
    hw_params: Some(es7134_hw_params),
    set_sysclk: Some(es7134_set_sysclk),
};

static mut es7134_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"es7134-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000
            | SNDRV_PCM_RATE_88200
            | SNDRV_PCM_RATE_96000
            | SNDRV_PCM_RATE_176400
            | SNDRV_PCM_RATE_192000,
        formats: SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S18_3LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_3LE
            | SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &es7134_dai_ops,
};

static es7134_mclk_fs_0: [c_uint; 5] = [256, 384, 512, 768, 1024];
static es7134_mclk_fs_1: [c_uint; 5] = [128, 192, 256, 384, 512];
static es7134_mclk_fs_2: [c_uint; 3] = [128, 192, 256];

static es7134_modes: [es7134_clock_mode; 3] = [
    es7134_clock_mode {
        /* Single speed mode */
        rate_min: 8000,
        rate_max: 50000,
        mclk_fs: es7134_mclk_fs_0.as_ptr(),
        mclk_fs_num: 5,
    },
    es7134_clock_mode {
        /* Double speed mode */
        rate_min: 84000,
        rate_max: 100000,
        mclk_fs: es7134_mclk_fs_1.as_ptr(),
        mclk_fs_num: 5,
    },
    es7134_clock_mode {
        /* Quad speed mode */
        rate_min: 167000,
        rate_max: 192000,
        mclk_fs: es7134_mclk_fs_2.as_ptr(),
        mclk_fs_num: 3,
    },
];

/* Digital I/O are also supplied by VDD on the es7134 */
static es7134_extra_routes: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: c"Playback".as_ptr(),
    control: ptr::null(),
    source: c"VDD".as_ptr(),
}];

static es7134_chip: es7134_chip = es7134_chip {
    dai_drv: unsafe { &raw mut es7134_dai },
    modes: es7134_modes.as_ptr(),
    mode_num: es7134_modes.len() as c_uint,
    extra_routes: es7134_extra_routes.as_ptr(),
    extra_route_num: es7134_extra_routes.len() as c_uint,
    extra_widgets: ptr::null(),
    extra_widget_num: 0,
};

/*
 * SND_SOC_DAPM_* macro initializers depend on <sound/soc.h>; represented here
 * as opaque widget objects preserving the array cardinality and ordering.
 */
static es7134_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { _private: [] }, /* SND_SOC_DAPM_OUTPUT("AOUTL") */
    snd_soc_dapm_widget { _private: [] }, /* SND_SOC_DAPM_OUTPUT("AOUTR") */
    snd_soc_dapm_widget { _private: [] }, /* SND_SOC_DAPM_DAC("DAC", "Playback", SND_SOC_NOPM, 0, 0) */
    snd_soc_dapm_widget { _private: [] }, /* SND_SOC_DAPM_REGULATOR_SUPPLY("VDD", 0, 0) */
];

static es7134_dapm_routes: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route {
        sink: c"AOUTL".as_ptr(),
        control: ptr::null(),
        source: c"DAC".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"AOUTR".as_ptr(),
        control: ptr::null(),
        source: c"DAC".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DAC".as_ptr(),
        control: ptr::null(),
        source: c"VDD".as_ptr(),
    },
];

static es7134_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(es7134_component_probe),
    dapm_widgets: es7134_dapm_widgets.as_ptr(),
    num_dapm_widgets: es7134_dapm_widgets.len() as c_uint,
    dapm_routes: es7134_dapm_routes.as_ptr(),
    num_dapm_routes: es7134_dapm_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static mut es7154_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"es7154-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000,
        formats: SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S18_3LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_3LE
            | SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &es7134_dai_ops,
};

static es7154_mclk_fs_0: [c_uint; 9] = [32, 64, 128, 192, 256, 384, 512, 768, 1024];
static es7154_mclk_fs_1: [c_uint; 7] = [128, 192, 256, 384, 512, 768, 1024];

static es7154_modes: [es7134_clock_mode; 2] = [
    es7134_clock_mode {
        /* Single speed mode */
        rate_min: 8000,
        rate_max: 50000,
        mclk_fs: es7154_mclk_fs_0.as_ptr(),
        mclk_fs_num: 9,
    },
    es7134_clock_mode {
        /* Double speed mode */
        rate_min: 84000,
        rate_max: 100000,
        mclk_fs: es7154_mclk_fs_1.as_ptr(),
        mclk_fs_num: 7,
    },
];

/* Es7154 has a separate supply for digital I/O  */
static es7154_extra_widgets: [snd_soc_dapm_widget; 1] = [
    snd_soc_dapm_widget { _private: [] }, /* SND_SOC_DAPM_REGULATOR_SUPPLY("PVDD", 0, 0) */
];

static es7154_extra_routes: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: c"Playback".as_ptr(),
    control: ptr::null(),
    source: c"PVDD".as_ptr(),
}];

static es7154_chip: es7134_chip = es7134_chip {
    dai_drv: unsafe { &raw mut es7154_dai },
    modes: es7154_modes.as_ptr(),
    mode_num: es7154_modes.len() as c_uint,
    extra_routes: es7154_extra_routes.as_ptr(),
    extra_route_num: es7154_extra_routes.len() as c_uint,
    extra_widgets: es7154_extra_widgets.as_ptr(),
    extra_widget_num: es7154_extra_widgets.len() as c_uint,
};

unsafe extern "C" fn es7134_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let priv_: *mut es7134_data;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<es7134_data>(), GFP_KERNEL) as *mut es7134_data;
    if priv_.is_null() {
        return -ENOMEM;
    }
    platform_set_drvdata(pdev, priv_ as *mut c_void);

    (*priv_).chip = of_device_get_match_data(dev) as *const es7134_chip;
    if (*priv_).chip.is_null() {
        dev_err(dev, c"failed to match device\n".as_ptr());
        return -ENODEV;
    }

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &es7134_component_driver,
        (*(*priv_).chip).dai_drv,
        1,
    )
}

/*
 * #ifdef CONFIG_OF
 */
static es7134_ids: [of_device_id; 4] = [
    of_device_id {
        compatible: c"everest,es7134".as_ptr(),
        data: &es7134_chip as *const es7134_chip as *const c_void,
    },
    of_device_id {
        compatible: c"everest,es7144".as_ptr(),
        data: &es7134_chip as *const es7134_chip as *const c_void,
    },
    of_device_id {
        compatible: c"everest,es7154".as_ptr(),
        data: &es7154_chip as *const es7134_chip as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, es7134_ids); */
/*
 * #endif
 */

static mut es7134_driver: platform_driver = platform_driver {
    driver: driver {
        name: c"es7134".as_ptr(),
        of_match_table: es7134_ids.as_ptr(), /* of_match_ptr(es7134_ids) */
    },
    probe: Some(es7134_probe),
};

/* module_platform_driver(es7134_driver); */

/* MODULE_DESCRIPTION("ASoC ES7134 audio codec driver"); */
/* MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
