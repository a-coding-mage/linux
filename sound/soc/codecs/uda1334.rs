// SPDX-License-Identifier: GPL-2.0-only
//
// uda1334.c  --  UDA1334 ALSA SoC Audio driver
//
// Based on WM8523 ALSA SoC Audio driver written by Mark Brown

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const UDA1334_NUM_RATES: usize = 6;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_int = 0;

const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_RATE_8000_96000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;

const SND_SOC_NOPM: c_int = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

const UDA1334_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;
const UDA1334_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *mut c_uint,
    pub mask: c_uint,
}

/* codec private data */
#[repr(C)]
pub struct uda1334_priv {
    pub mute: *mut gpio_desc,
    pub deemph: *mut gpio_desc,
    pub sysclk: c_uint,
    pub rate_constraint_list: [c_uint; UDA1334_NUM_RATES],
    pub rate_constraint: snd_pcm_hw_constraint_list,
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
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lrclk_ratio {
    pub value: c_int,
    pub ratio: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
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
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
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
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_inner,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        l: *mut snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

// SND_SOC_DAPM_DAC("DAC", "Playback", SND_SOC_NOPM, 0, 0)
// SND_SOC_DAPM_OUTPUT("LINEVOUTL")
// SND_SOC_DAPM_OUTPUT("LINEVOUTR")
static uda1334_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static uda1334_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"LINEVOUTL\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"LINEVOUTR\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn uda1334_put_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let uda1334 = snd_soc_component_get_drvdata(component) as *mut uda1334_priv;
    let deemph = unsafe { (*ucontrol).value.integer.value[0] as c_int };

    if deemph > 1 {
        return -EINVAL;
    }

    unsafe { gpiod_set_value_cansleep((*uda1334).deemph, deemph) };

    0
}

unsafe extern "C" fn uda1334_get_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let uda1334 = snd_soc_component_get_drvdata(component) as *mut uda1334_priv;
    let ret: c_int;

    ret = unsafe { gpiod_get_value_cansleep((*uda1334).deemph) };
    if ret < 0 {
        return -EINVAL;
    }

    unsafe {
        (*ucontrol).value.integer.value[0] = ret as i64;
    }

    0
}

// SOC_SINGLE_BOOL_EXT("Playback Deemphasis Switch", 0,
//                     uda1334_get_deemph, uda1334_put_deemph)
static uda1334_snd_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new { _private: [] }];

static lrclk_ratios: [lrclk_ratio; UDA1334_NUM_RATES] = [
    lrclk_ratio { value: 1, ratio: 128 },
    lrclk_ratio { value: 2, ratio: 192 },
    lrclk_ratio { value: 3, ratio: 256 },
    lrclk_ratio { value: 4, ratio: 384 },
    lrclk_ratio { value: 5, ratio: 512 },
    lrclk_ratio { value: 6, ratio: 768 },
];

unsafe extern "C" fn uda1334_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = unsafe { (*dai).component };
    let uda1334 = snd_soc_component_get_drvdata(component) as *mut uda1334_priv;

    /*
     * The set of sample rates that can be supported depends on the
     * MCLK supplied to the CODEC - enforce this.
     */
    if unsafe { (*uda1334).sysclk } == 0 {
        unsafe {
            dev_err(
                (*component).dev,
                b"No MCLK configured, call set_sysclk() on init\n\0".as_ptr() as *const c_char,
            );
        }
        return -EINVAL;
    }

    unsafe {
        snd_pcm_hw_constraint_list(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            &mut (*uda1334).rate_constraint,
        );
    }

    unsafe { gpiod_set_value_cansleep((*uda1334).mute, 1) };

    0
}

unsafe extern "C" fn uda1334_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let component = unsafe { (*dai).component };
    let uda1334 = snd_soc_component_get_drvdata(component) as *mut uda1334_priv;

    unsafe { gpiod_set_value_cansleep((*uda1334).mute, 0) };
}

unsafe extern "C" fn uda1334_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = unsafe { (*codec_dai).component };
    let uda1334 = snd_soc_component_get_drvdata(component) as *mut uda1334_priv;
    let mut val: c_uint;
    let mut i: usize;
    let mut j: usize = 0;

    unsafe {
        (*uda1334).sysclk = freq;
    }

    unsafe {
        (*uda1334).rate_constraint.count = 0;
    }
    i = 0;
    while i < lrclk_ratios.len() {
        val = freq / lrclk_ratios[i].ratio as c_uint;
        /*
         * Check that it's a standard rate since core can't
         * cope with others and having the odd rates confuses
         * constraint matching.
         */

        match val {
            8000 | 32000 | 44100 | 48000 | 64000 | 88200 | 96000 => {
                unsafe {
                    dev_dbg(
                        (*component).dev,
                        b"Supported sample rate: %dHz\n\0".as_ptr() as *const c_char,
                        val,
                    );
                    (*uda1334).rate_constraint_list[j] = val;
                    j += 1;
                    (*uda1334).rate_constraint.count += 1;
                }
            }
            _ => unsafe {
                dev_dbg(
                    (*component).dev,
                    b"Skipping sample rate: %dHz\n\0".as_ptr() as *const c_char,
                    val,
                );
            },
        }
        i += 1;
    }

    /* Need at least one supported rate... */
    if unsafe { (*uda1334).rate_constraint.count } == 0 {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn uda1334_set_fmt(codec_dai: *mut snd_soc_dai, mut fmt: c_uint) -> c_int {
    fmt &= SND_SOC_DAIFMT_FORMAT_MASK | SND_SOC_DAIFMT_INV_MASK | SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;

    if fmt != (SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC) {
        unsafe {
            dev_err(
                (*codec_dai).dev,
                b"Invalid DAI format\n\0".as_ptr() as *const c_char,
            );
        }
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn uda1334_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _stream: c_int,
) -> c_int {
    let uda1334 = snd_soc_component_get_drvdata(unsafe { (*dai).component }) as *mut uda1334_priv;

    if unsafe { !(*uda1334).mute.is_null() } {
        unsafe { gpiod_set_value_cansleep((*uda1334).mute, mute) };
    }

    0
}

static uda1334_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(uda1334_startup),
    shutdown: Some(uda1334_shutdown),
    set_sysclk: Some(uda1334_set_dai_sysclk),
    set_fmt: Some(uda1334_set_fmt),
    mute_stream: Some(uda1334_mute_stream),
};

static mut uda1334_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"uda1334-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: UDA1334_RATES,
        formats: UDA1334_FORMATS,
    },
    ops: &uda1334_dai_ops,
};

unsafe extern "C" fn uda1334_probe(component: *mut snd_soc_component) -> c_int {
    let uda1334 = snd_soc_component_get_drvdata(component) as *mut uda1334_priv;

    unsafe {
        (*uda1334).rate_constraint.list = (*uda1334).rate_constraint_list.as_mut_ptr();
        (*uda1334).rate_constraint.count = (*uda1334).rate_constraint_list.len() as c_uint;
    }

    0
}

static soc_component_dev_uda1334: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(uda1334_probe),
    controls: uda1334_snd_controls.as_ptr(),
    num_controls: uda1334_snd_controls.len() as c_uint,
    dapm_widgets: uda1334_dapm_widgets.as_ptr(),
    num_dapm_widgets: uda1334_dapm_widgets.len() as c_uint,
    dapm_routes: uda1334_dapm_routes.as_ptr(),
    num_dapm_routes: uda1334_dapm_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static uda1334_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"nxp,uda1334\0".as_ptr() as *const c_char,
    },
    of_device_id {
        /* sentinel*/
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, uda1334_of_match);

unsafe extern "C" fn uda1334_codec_probe(pdev: *mut platform_device) -> c_int {
    let uda1334: *mut uda1334_priv;
    let mut ret: c_int;

    uda1334 = unsafe {
        devm_kzalloc(
            &mut (*pdev).dev,
            size_of::<uda1334_priv>(),
            GFP_KERNEL,
        ) as *mut uda1334_priv
    };
    if uda1334.is_null() {
        return -ENOMEM;
    }

    unsafe {
        platform_set_drvdata(pdev, uda1334 as *mut c_void);
    }

    unsafe {
        (*uda1334).mute = devm_gpiod_get(
            &mut (*pdev).dev,
            b"nxp,mute\0".as_ptr() as *const c_char,
            GPIOD_OUT_LOW,
        );
    }
    if unsafe { IS_ERR((*uda1334).mute as *const c_void) } {
        ret = unsafe { PTR_ERR((*uda1334).mute as *const c_void) };
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                b"Failed to get mute line: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
        return ret;
    }

    unsafe {
        (*uda1334).deemph = devm_gpiod_get(
            &mut (*pdev).dev,
            b"nxp,deemph\0".as_ptr() as *const c_char,
            GPIOD_OUT_LOW,
        );
    }
    if unsafe { IS_ERR((*uda1334).deemph as *const c_void) } {
        ret = unsafe { PTR_ERR((*uda1334).deemph as *const c_void) };
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                b"Failed to get deemph line: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
        return ret;
    }

    ret = unsafe {
        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &soc_component_dev_uda1334,
            &mut uda1334_dai,
            1,
        )
    };
    if ret < 0 {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                b"Failed to register component: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
    }

    ret
}

static mut uda1334_codec_driver: platform_driver = platform_driver {
    probe: Some(uda1334_codec_probe),
    driver: platform_driver_inner {
        name: b"uda1334-codec\0".as_ptr() as *const c_char,
        of_match_table: uda1334_of_match.as_ptr(),
    },
};
// module_platform_driver(uda1334_codec_driver);

// MODULE_DESCRIPTION("ASoC UDA1334 driver");
// MODULE_AUTHOR("Andra Danciu <andradanciu1997@gmail.com>");
// MODULE_ALIAS("platform:uda1334-codec");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
