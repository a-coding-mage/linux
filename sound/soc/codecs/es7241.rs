// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

use core::ffi::{c_char, c_int, c_uint, c_void};

const EINVAL: c_int = 22;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_3LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;

#[repr(C)]
pub struct gpio_desc {
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
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
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
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
struct es7241_clock_mode {
    rate_min: c_uint,
    rate_max: c_uint,
    slv_mfs: *const c_uint,
    slv_mfs_num: c_uint,
    mst_mfs: c_uint,
    mst_m0: c_uint,
    mst_m1: c_uint,
}

#[repr(C)]
struct es7241_chip {
    modes: *const es7241_clock_mode,
    mode_num: c_uint,
}

#[repr(C)]
struct es7241_data {
    reset: *mut gpio_desc,
    m0: *mut gpio_desc,
    m1: *mut gpio_desc,
    fmt: c_uint,
    mclk: c_uint,
    is_consumer: bool,
    chip: *const es7241_chip,
}

unsafe extern "C" {
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_property_read_bool(node: *mut device_node, propname: *const c_char) -> bool;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn module_platform_driver(driver: *mut platform_driver);
}

unsafe fn es7241_set_mode(priv_: *mut es7241_data, m0: c_int, m1: c_int) {
    /* put the device in reset */
    unsafe {
        gpiod_set_value_cansleep((*priv_).reset, 0);
    }

    /* set the mode */
    unsafe {
        gpiod_set_value_cansleep((*priv_).m0, m0);
        gpiod_set_value_cansleep((*priv_).m1, m1);
    }

    /* take the device out of reset - datasheet does not specify a delay */
    unsafe {
        gpiod_set_value_cansleep((*priv_).reset, 1);
    }
}

unsafe extern "C" fn es7241_set_consumer_mode(
    priv_: *mut es7241_data,
    mode: *const es7241_clock_mode,
    mfs: c_uint,
) -> c_int {
    let mut j: c_int;

    if mfs == 0 {
        unsafe {
            es7241_set_mode(priv_, 1, 1);
        }
        return 0;
    }

    j = 0;
    while j < unsafe { (*mode).slv_mfs_num as c_int } {
        if unsafe { *(*mode).slv_mfs.add(j as usize) } == mfs {
            unsafe {
                es7241_set_mode(priv_, 1, 1);
            }
            return 0;
        }
        j += 1;
    }

    -EINVAL
}

unsafe extern "C" fn es7241_set_provider_mode(
    priv_: *mut es7241_data,
    mode: *const es7241_clock_mode,
    mfs: c_uint,
) -> c_int {
    /*
     * We can't really set clock ratio, if the mclk/lrclk is different
     * from what we provide, then error out
     */
    if mfs != 0 && mfs != unsafe { (*mode).mst_mfs } {
        return -EINVAL;
    }

    unsafe {
        es7241_set_mode(priv_, (*mode).mst_m0 as c_int, (*mode).mst_m1 as c_int);
    }

    0
}

unsafe extern "C" fn es7241_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_ = unsafe { snd_soc_dai_get_drvdata(dai) as *mut es7241_data };
    let rate = unsafe { params_rate(params) };
    let mfs = unsafe { (*priv_).mclk / rate };
    let mut i: c_int;

    i = 0;
    while i < unsafe { (*(*priv_).chip).mode_num as c_int } {
        let mode = unsafe { (*(*priv_).chip).modes.add(i as usize) };

        if rate < unsafe { (*mode).rate_min } || rate >= unsafe { (*mode).rate_max } {
            i += 1;
            continue;
        }

        if unsafe { (*priv_).is_consumer } {
            return unsafe { es7241_set_consumer_mode(priv_, mode, mfs) };
        } else {
            return unsafe { es7241_set_provider_mode(priv_, mode, mfs) };
        }
    }

    /* should not happen */
    unsafe {
        dev_err((*dai).dev, c"unsupported rate: %u\n".as_ptr());
    }
    -EINVAL
}

unsafe extern "C" fn es7241_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let priv_ = unsafe { snd_soc_dai_get_drvdata(dai) as *mut es7241_data };

    if dir == SND_SOC_CLOCK_IN && clk_id == 0 {
        unsafe {
            (*priv_).mclk = freq;
        }
        return 0;
    }

    -ENOTSUPP
}

unsafe extern "C" fn es7241_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let priv_ = unsafe { snd_soc_dai_get_drvdata(dai) as *mut es7241_data };

    if (fmt & SND_SOC_DAIFMT_INV_MASK) != SND_SOC_DAIFMT_NB_NF {
        unsafe {
            dev_err((*dai).dev, c"Unsupported dai clock inversion\n".as_ptr());
        }
        return -EINVAL;
    }

    if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) != unsafe { (*priv_).fmt } {
        unsafe {
            dev_err((*dai).dev, c"Invalid dai format\n".as_ptr());
        }
        return -EINVAL;
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {
            unsafe {
                (*priv_).is_consumer = true;
            }
        }
        SND_SOC_DAIFMT_CBP_CFP => {
            unsafe {
                (*priv_).is_consumer = false;
            }
        }
        _ => {
            unsafe {
                dev_err((*dai).dev, c"Unsupported clock configuration\n".as_ptr());
            }
            return -EINVAL;
        }
    }

    0
}

static ES7241_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(es7241_set_fmt),
    hw_params: Some(es7241_hw_params),
    set_sysclk: Some(es7241_set_sysclk),
};

static mut ES7241_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"es7241-hifi".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_LE,
    },
    ops: &ES7241_DAI_OPS,
};

static ES7241_SLV_MFS_0: [c_uint; 5] = [256, 384, 512, 768, 1024];
static ES7241_SLV_MFS_1: [c_uint; 2] = [128, 192];
static ES7241_SLV_MFS_2: [c_uint; 1] = [64];

static ES7241_MODES: [es7241_clock_mode; 3] = [
    es7241_clock_mode {
        /* Single speed mode */
        rate_min: 8000,
        rate_max: 50000,
        slv_mfs: ES7241_SLV_MFS_0.as_ptr(),
        slv_mfs_num: 5,
        mst_mfs: 256,
        mst_m0: 0,
        mst_m1: 0,
    },
    es7241_clock_mode {
        /* Double speed mode */
        rate_min: 50000,
        rate_max: 100000,
        slv_mfs: ES7241_SLV_MFS_1.as_ptr(),
        slv_mfs_num: 2,
        mst_mfs: 128,
        mst_m0: 1,
        mst_m1: 0,
    },
    es7241_clock_mode {
        /* Quad speed mode */
        rate_min: 100000,
        rate_max: 200000,
        slv_mfs: ES7241_SLV_MFS_2.as_ptr(),
        slv_mfs_num: 1,
        mst_mfs: 64,
        mst_m0: 0,
        mst_m1: 1,
    },
];

static ES7241_CHIP: es7241_chip = es7241_chip {
    modes: ES7241_MODES.as_ptr(),
    mode_num: ES7241_MODES.len() as c_uint,
};

// SND_SOC_DAPM_* macro-created widget contents depend on external ASoC definitions.
static ES7241_DAPM_WIDGETS: [snd_soc_dapm_widget; 6] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static ES7241_DAPM_ROUTES: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route {
        sink: c"ADC".as_ptr(),
        control: core::ptr::null(),
        source: c"AINL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"ADC".as_ptr(),
        control: core::ptr::null(),
        source: c"AINR".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"ADC".as_ptr(),
        control: core::ptr::null(),
        source: c"VDDA".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"VDDP".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Capture".as_ptr(),
        control: core::ptr::null(),
        source: c"VDDD".as_ptr(),
    },
];

static ES7241_COMPONENT_DRIVER: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: ES7241_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: ES7241_DAPM_WIDGETS.len() as c_uint,
    dapm_routes: ES7241_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: ES7241_DAPM_ROUTES.len() as c_uint,
    idle_bias_on: 1,
    endianness: 1,
};

unsafe fn es7241_parse_fmt(dev: *mut device, priv_: *mut es7241_data) {
    let is_leftj: bool;

    /*
     * The format is given by a pull resistor on the SDOUT pin:
     * pull-up for i2s, pull-down for left justified.
     */
    is_leftj = unsafe {
        of_property_read_bool((*dev).of_node, c"everest,sdout-pull-down".as_ptr())
    };
    if is_leftj {
        unsafe {
            (*priv_).fmt = SND_SOC_DAIFMT_LEFT_J;
        }
    } else {
        unsafe {
            (*priv_).fmt = SND_SOC_DAIFMT_I2S;
        }
    }
}

unsafe extern "C" fn es7241_probe(pdev: *mut platform_device) -> c_int {
    let dev = unsafe { &mut (*pdev).dev as *mut device };
    let priv_: *mut es7241_data;

    priv_ = unsafe { devm_kzalloc(dev, core::mem::size_of::<es7241_data>(), GFP_KERNEL) }
        as *mut es7241_data;
    if priv_.is_null() {
        return -ENOMEM;
    }
    unsafe {
        platform_set_drvdata(pdev, priv_ as *mut c_void);
    }

    unsafe {
        (*priv_).chip = of_device_get_match_data(dev) as *const es7241_chip;
    }
    if unsafe { (*priv_).chip.is_null() } {
        unsafe {
            dev_err(dev, c"failed to match device\n".as_ptr());
        }
        return -ENODEV;
    }

    unsafe {
        es7241_parse_fmt(dev, priv_);
    }

    unsafe {
        (*priv_).reset = devm_gpiod_get_optional(dev, c"reset".as_ptr(), GPIOD_OUT_LOW);
    }
    if unsafe { IS_ERR((*priv_).reset as *const c_void) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR((*priv_).reset as *const c_void),
                c"Failed to get 'reset' gpio".as_ptr(),
            )
        };
    }

    unsafe {
        (*priv_).m0 = devm_gpiod_get_optional(dev, c"m0".as_ptr(), GPIOD_OUT_LOW);
    }
    if unsafe { IS_ERR((*priv_).m0 as *const c_void) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR((*priv_).m0 as *const c_void),
                c"Failed to get 'm0' gpio".as_ptr(),
            )
        };
    }

    unsafe {
        (*priv_).m1 = devm_gpiod_get_optional(dev, c"m1".as_ptr(), GPIOD_OUT_LOW);
    }
    if unsafe { IS_ERR((*priv_).m1 as *const c_void) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR((*priv_).m1 as *const c_void),
                c"Failed to get 'm1' gpio".as_ptr(),
            )
        };
    }

    unsafe {
        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &ES7241_COMPONENT_DRIVER,
            &mut ES7241_DAI,
            1,
        )
    }
}

// Original C condition: #ifdef CONFIG_OF
static ES7241_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: c"everest,es7241".as_ptr(),
        data: &ES7241_CHIP as *const es7241_chip as *const c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, es7241_ids);

static mut ES7241_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: c"es7241".as_ptr(),
        of_match_table: ES7241_IDS.as_ptr(),
    },
    probe: Some(es7241_probe),
};

unsafe fn es7241_init_module() {
    unsafe {
        module_platform_driver(&mut ES7241_DRIVER);
    }
}

// MODULE_DESCRIPTION("ASoC ES7241 audio codec driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
