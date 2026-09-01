// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8524.c  --  WM8524 ALSA SoC Audio driver
 *
 * Copyright 2009 Wolfson Microelectronics plc
 * Copyright 2017 NXP
 *
 * Based on WM8523 ALSA SoC Audio driver written by Mark Brown
 */

// C dependencies:
// linux/module.h, linux/moduleparam.h, linux/init.h, linux/delay.h,
// linux/slab.h, linux/gpio/consumer.h, sound/core.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h, sound/initval.h

const WM8524_NUM_RATES: usize = 12;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: gfp_t = 0;
const GPIOD_OUT_LOW: c_int = 0;

const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 0;

const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;

type c_int = i32;
type c_uint = u32;
type c_char = i8;
type size_t = usize;
type gfp_t = c_uint;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
    dev: *mut device,
}

#[repr(C)]
struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *mut c_uint,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
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
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    idle_bias_on: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: device_driver,
}

/* codec private data */
#[repr(C)]
struct wm8524_priv {
    mute: *mut gpio_desc,
    sysclk: c_uint,
    rate_constraint_list: [c_uint; WM8524_NUM_RATES],
    rate_constraint: snd_pcm_hw_constraint_list,
}

#[repr(C)]
struct lrclk_ratio {
    value: c_int,
    ratio: c_int,
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut wm8524_priv;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *mut snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: gfp_t) -> *mut core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

// Original C used SND_SOC_DAPM_DAC("DAC", "Playback", SND_SOC_NOPM, 0, 0)
// and SND_SOC_DAPM_OUTPUT("LINEVOUTL"/"LINEVOUTR") macro initializers here.
// Those macro-expanded struct bodies are supplied by the ALSA SoC headers.
static WM8524_DAPM_WIDGETS: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static WM8524_DAPM_ROUTES: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"LINEVOUTL\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"LINEVOUTR\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
];

static LRCLK_RATIOS: [lrclk_ratio; 7] = [
    lrclk_ratio { value: 1, ratio: 128 },
    lrclk_ratio { value: 2, ratio: 192 },
    lrclk_ratio { value: 3, ratio: 256 },
    lrclk_ratio { value: 4, ratio: 384 },
    lrclk_ratio { value: 5, ratio: 512 },
    lrclk_ratio { value: 6, ratio: 768 },
    lrclk_ratio { value: 7, ratio: 1152 },
];

unsafe extern "C" fn wm8524_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let wm8524: *mut wm8524_priv = snd_soc_component_get_drvdata(component);

    /* The set of sample rates that can be supported depends on the
     * MCLK supplied to the CODEC.
     */
    if (*wm8524).sysclk != 0 {
        snd_pcm_hw_constraint_list(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            &mut (*wm8524).rate_constraint,
        );
    }

    gpiod_set_value_cansleep((*wm8524).mute, 1);

    0
}

unsafe extern "C" fn wm8524_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let component: *mut snd_soc_component = (*dai).component;
    let wm8524: *mut wm8524_priv = snd_soc_component_get_drvdata(component);

    gpiod_set_value_cansleep((*wm8524).mute, 0);
}

unsafe extern "C" fn wm8524_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let wm8524: *mut wm8524_priv = snd_soc_component_get_drvdata(component);
    let mut val: c_uint;
    let mut j: c_int = 0;

    (*wm8524).rate_constraint.count = 0;
    (*wm8524).sysclk = freq;
    if (*wm8524).sysclk == 0 {
        return 0;
    }

    for i in 0..LRCLK_RATIOS.len() {
        val = freq / LRCLK_RATIOS[i].ratio as c_uint;
        /* Check that it's a standard rate since core can't
         * cope with others and having the odd rates confuses
         * constraint matching.
         */
        match val {
            8000 | 11025 | 16000 | 22050 | 32000 | 44100 | 48000 | 64000 | 88200 | 96000
            | 176400 | 192000 => {
                dev_dbg(
                    (*component).dev,
                    b"Supported sample rate: %dHz\n\0".as_ptr() as *const c_char,
                    val,
                );
                (*wm8524).rate_constraint_list[j as usize] = val;
                j += 1;
                (*wm8524).rate_constraint.count += 1;
            }
            _ => {
                dev_dbg(
                    (*component).dev,
                    b"Skipping sample rate: %dHz\n\0".as_ptr() as *const c_char,
                    val,
                );
            }
        }
    }

    /* Need at least one supported rate... */
    if (*wm8524).rate_constraint.count == 0 {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn wm8524_set_fmt(codec_dai: *mut snd_soc_dai, mut fmt: c_uint) -> c_int {
    fmt &= SND_SOC_DAIFMT_FORMAT_MASK | SND_SOC_DAIFMT_INV_MASK | SND_SOC_DAIFMT_MASTER_MASK;

    if fmt != (SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC) {
        dev_err(
            (*codec_dai).dev,
            b"Invalid DAI format\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn wm8524_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _stream: c_int,
) -> c_int {
    let wm8524: *mut wm8524_priv = snd_soc_component_get_drvdata((*dai).component);

    if !(*wm8524).mute.is_null() {
        gpiod_set_value_cansleep((*wm8524).mute, mute);
    }

    0
}

unsafe extern "C" fn wm8524_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let wm8524: *mut wm8524_priv = snd_soc_component_get_drvdata(component);
    let mut i: c_uint;

    /* If sysclk is not configured, no need to check the rate */
    if (*wm8524).sysclk == 0 {
        return 0;
    }

    /* Find a supported LRCLK rate */
    i = 0;
    while i < (*wm8524).rate_constraint.count {
        if *(*wm8524).rate_constraint.list.add(i as usize) == params_rate(params) {
            break;
        }
        i += 1;
    }

    if i == (*wm8524).rate_constraint.count {
        dev_err(
            (*component).dev,
            b"LRCLK %d unsupported with MCLK %d\n\0".as_ptr() as *const c_char,
            params_rate(params),
            (*wm8524).sysclk,
        );
        return -EINVAL;
    }

    0
}

const WM8524_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;

const WM8524_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static WM8524_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(wm8524_startup),
    shutdown: Some(wm8524_shutdown),
    set_sysclk: Some(wm8524_set_dai_sysclk),
    set_fmt: Some(wm8524_set_fmt),
    mute_stream: Some(wm8524_mute_stream),
    hw_params: Some(wm8524_hw_params),
};

static mut WM8524_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"wm8524-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: WM8524_RATES,
        formats: WM8524_FORMATS,
    },
    ops: &WM8524_DAI_OPS,
};

unsafe extern "C" fn wm8524_probe(component: *mut snd_soc_component) -> c_int {
    let wm8524: *mut wm8524_priv = snd_soc_component_get_drvdata(component);

    (*wm8524).rate_constraint.list = (*wm8524).rate_constraint_list.as_mut_ptr();
    (*wm8524).rate_constraint.count = (*wm8524).rate_constraint_list.len() as c_uint;

    0
}

static SOC_COMPONENT_DEV_WM8524: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8524_probe),
    dapm_widgets: WM8524_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: WM8524_DAPM_WIDGETS.len() as c_uint,
    dapm_routes: WM8524_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: WM8524_DAPM_ROUTES.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static WM8524_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: b"wlf,wm8524\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, wm8524_of_match);

unsafe extern "C" fn wm8524_codec_probe(pdev: *mut platform_device) -> c_int {
    let wm8524: *mut wm8524_priv;
    let mut ret: c_int;

    wm8524 = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<wm8524_priv>(),
        GFP_KERNEL,
    ) as *mut wm8524_priv;
    if wm8524.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, wm8524 as *mut core::ffi::c_void);

    (*wm8524).mute = devm_gpiod_get(
        &mut (*pdev).dev,
        b"wlf,mute\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR((*wm8524).mute as *const core::ffi::c_void) {
        ret = PTR_ERR((*wm8524).mute as *const core::ffi::c_void);
        dev_err_probe(
            &mut (*pdev).dev,
            ret,
            b"Failed to get mute line\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &SOC_COMPONENT_DEV_WM8524,
        &mut WM8524_DAI,
        1,
    );
    if ret < 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Failed to register component: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    ret
}

static mut WM8524_CODEC_DRIVER: platform_driver = platform_driver {
    probe: Some(wm8524_codec_probe),
    driver: device_driver {
        name: b"wm8524-codec\0".as_ptr() as *const c_char,
        of_match_table: WM8524_OF_MATCH.as_ptr(),
    },
};
// module_platform_driver(wm8524_codec_driver);

// MODULE_DESCRIPTION("ASoC WM8524 driver");
// MODULE_AUTHOR("Mihai Serban <mihai.serban@nxp.com>");
// MODULE_ALIAS("platform:wm8524-codec");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
