// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC driver for Migo-R
//
// Copyright (C) 2009-2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};
use core::ptr;

const ENOMEM: c_int = 12;
const WM8978_PLL: c_int = 0;
const WM8978_OPCLKRATE: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SIU_CLKB_EXT: c_int = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

#[repr(C)]
pub struct clk {
    pub ops: *mut sh_clk_ops,
    pub rate: c_ulong,
}

#[repr(C)]
pub struct sh_clk_ops {
    pub recalc: Option<unsafe extern "C" fn(*mut clk) -> c_ulong>,
}

#[repr(C)]
pub struct clk_lookup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
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
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub kind: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_clkdiv(dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn clk_register(clk: *mut clk) -> c_int;
    fn clk_unregister(clk: *mut clk);
    fn clkdev_create(clk: *mut clk, con_id: *const c_char, dev_id: *const c_char) -> *mut clk_lookup;
    fn clkdev_drop(cl: *mut clk_lookup);
    fn platform_device_alloc(name: *const c_char, id: c_int) -> *mut platform_device;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_device_add(pdev: *mut platform_device) -> c_int;
    fn platform_device_put(pdev: *mut platform_device);
    fn platform_device_unregister(pdev: *mut platform_device);
}

/* Default 8000Hz sampling frequency */
static mut codec_freq: c_ulong = 8000 * 512;

static mut use_count: c_uint = 0;

/* External clock, sourced from the codec at the SIUMCKB pin */
unsafe extern "C" fn siumckb_recalc(_clk: *mut clk) -> c_ulong {
    unsafe { codec_freq }
}

static mut siumckb_clk_ops: sh_clk_ops = sh_clk_ops {
    recalc: Some(siumckb_recalc),
};

static mut siumckb_clk: clk = clk {
    ops: unsafe { &raw mut siumckb_clk_ops },
    rate: 0, /* initialised at run-time */
};

static mut siumckb_lookup: *mut clk_lookup = ptr::null_mut();

unsafe extern "C" fn migor_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let mut ret: c_int;
    let rate: c_uint = unsafe { params_rate(params) };

    ret = unsafe {
        snd_soc_dai_set_sysclk(codec_dai, WM8978_PLL, 13000000, SND_SOC_CLOCK_IN)
    };
    if ret < 0 {
        return ret;
    }

    ret = unsafe {
        snd_soc_dai_set_clkdiv(codec_dai, WM8978_OPCLKRATE, (rate.wrapping_mul(512)) as c_int)
    };
    if ret < 0 {
        return ret;
    }

    unsafe {
        codec_freq = rate.wrapping_mul(512) as c_ulong;
    }
    /*
     * This propagates the parent frequency change to children and
     * recalculates the frequency table
     */
    unsafe {
        clk_set_rate(&raw mut siumckb_clk, codec_freq);
        dev_dbg(
            (*codec_dai).dev,
            c"%s: configure %luHz\n".as_ptr(),
            c"migor_hw_params".as_ptr(),
            codec_freq,
        );
    }

    ret = unsafe {
        snd_soc_dai_set_sysclk(
            snd_soc_rtd_to_cpu(rtd, 0),
            SIU_CLKB_EXT,
            (codec_freq / 2) as c_uint,
            SND_SOC_CLOCK_IN,
        )
    };

    if ret == 0 {
        unsafe {
            use_count = use_count.wrapping_add(1);
        }
    }

    ret
}

unsafe extern "C" fn migor_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };

    unsafe {
        if use_count != 0 {
            use_count = use_count.wrapping_sub(1);

            if use_count == 0 {
                snd_soc_dai_set_sysclk(codec_dai, WM8978_PLL, 0, SND_SOC_CLOCK_IN);
            }
        } else {
            dev_dbg((*codec_dai).dev, c"Unbalanced hw_free!\n".as_ptr());
        }
    }

    0
}

static migor_dai_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(migor_hw_params),
    hw_free: Some(migor_hw_free),
};

const fn snd_soc_dapm_hp(name: *const c_char, _wname: *const c_void) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget { name, kind: 0 }
}

const fn snd_soc_dapm_mic(name: *const c_char, _wname: *const c_void) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget { name, kind: 0 }
}

static migor_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_hp(c"Headphone".as_ptr(), ptr::null()),
    snd_soc_dapm_mic(c"Onboard Microphone".as_ptr(), ptr::null()),
    snd_soc_dapm_mic(c"External Microphone".as_ptr(), ptr::null()),
];

static audio_map: [snd_soc_dapm_route; 9] = [
    /* Headphone output connected to LHP/RHP, enable OUT4 for VMID */
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: ptr::null(),
        source: c"OUT4 VMID".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OUT4 VMID".as_ptr(),
        control: ptr::null(),
        source: c"LHP".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OUT4 VMID".as_ptr(),
        control: ptr::null(),
        source: c"RHP".as_ptr(),
    },
    /* On-board microphone */
    snd_soc_dapm_route {
        sink: c"RMICN".as_ptr(),
        control: ptr::null(),
        source: c"Mic Bias".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"RMICP".as_ptr(),
        control: ptr::null(),
        source: c"Mic Bias".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Mic Bias".as_ptr(),
        control: ptr::null(),
        source: c"Onboard Microphone".as_ptr(),
    },
    /* External microphone */
    snd_soc_dapm_route {
        sink: c"LMICN".as_ptr(),
        control: ptr::null(),
        source: c"Mic Bias".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"LMICP".as_ptr(),
        control: ptr::null(),
        source: c"Mic Bias".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Mic Bias".as_ptr(),
        control: ptr::null(),
        source: c"External Microphone".as_ptr(),
    },
];

/* migor digital audio interface glue - connects codec <--> CPU */
/* SND_SOC_DAILINK_DEFS(wm8978,
 *     DAILINK_COMP_ARRAY(COMP_CPU("siu-pcm-audio")),
 *     DAILINK_COMP_ARRAY(COMP_CODEC("wm8978.0-001a", "wm8978-hifi")),
 *     DAILINK_COMP_ARRAY(COMP_PLATFORM("siu-pcm-audio")));
 */

static mut migor_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: c"wm8978".as_ptr(),
    stream_name: c"WM8978".as_ptr(),
    dai_fmt: SND_SOC_DAIFMT_NB_IF | SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC,
    ops: &migor_dai_ops,
    /* SND_SOC_DAILINK_REG(wm8978) */
};

/* migor audio machine driver */
static mut snd_soc_migor: snd_soc_card = snd_soc_card {
    name: c"Migo-R".as_ptr(),
    owner: ptr::null_mut(),
    dai_link: unsafe { &raw mut migor_dai },
    num_links: 1,

    dapm_widgets: migor_dapm_widgets.as_ptr(),
    num_dapm_widgets: migor_dapm_widgets.len() as c_int,
    dapm_routes: audio_map.as_ptr(),
    num_dapm_routes: audio_map.len() as c_int,
};

static mut migor_snd_device: *mut platform_device = ptr::null_mut();

unsafe extern "C" fn migor_init() -> c_int {
    let mut ret: c_int;

    unsafe {
        snd_soc_migor.owner = THIS_MODULE;
    }

    ret = unsafe { clk_register(&raw mut siumckb_clk) };
    if ret < 0 {
        return ret;
    }

    unsafe {
        siumckb_lookup = clkdev_create(&raw mut siumckb_clk, c"siumckb_clk".as_ptr(), ptr::null());
    }
    if unsafe { siumckb_lookup.is_null() } {
        ret = -ENOMEM;
        goto_eclkdevalloc(ret);
        return ret;
    }

    /* Port number used on this machine: port B */
    unsafe {
        migor_snd_device = platform_device_alloc(c"soc-audio".as_ptr(), 1);
    }
    if unsafe { migor_snd_device.is_null() } {
        ret = -ENOMEM;
        unsafe {
            clkdev_drop(siumckb_lookup);
        }
        goto_eclkdevalloc(ret);
        return ret;
    }

    unsafe {
        platform_set_drvdata(migor_snd_device, (&raw mut snd_soc_migor).cast::<c_void>());
    }

    ret = unsafe { platform_device_add(migor_snd_device) };
    if ret != 0 {
        unsafe {
            platform_device_put(migor_snd_device);
            clkdev_drop(siumckb_lookup);
        }
        goto_eclkdevalloc(ret);
        return ret;
    }

    0
}

unsafe fn goto_eclkdevalloc(ret: c_int) -> c_int {
    unsafe {
        clk_unregister(&raw mut siumckb_clk);
    }
    ret
}

unsafe extern "C" fn migor_exit() {
    unsafe {
        clkdev_drop(siumckb_lookup);
        clk_unregister(&raw mut siumckb_clk);
        platform_device_unregister(migor_snd_device);
    }
}

/* module_init(migor_init); */
/* module_exit(migor_exit); */

/* MODULE_AUTHOR("Guennadi Liakhovetski <g.liakhovetski@gmx.de>"); */
/* MODULE_DESCRIPTION("ALSA SoC Migor"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
