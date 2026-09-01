// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * spitz.rs  --  SoC audio for Sharp SL-Cxx00 models Spitz, Borzoi and Akita
 *
 * Copyright 2005 Wolfson Microelectronics PLC.
 * Copyright 2005 Openedhand Ltd.
 *
 * Authors: Liam Girdwood <lrg@slimlogic.co.uk>
 *          Richard Purdie <richard@openedhand.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const SPITZ_HP: c_int = 0;
const SPITZ_MIC: c_int = 1;
const SPITZ_LINE: c_int = 2;
const SPITZ_HEADSET: c_int = 3;
const SPITZ_HP_OFF: c_int = 4;
const SPITZ_SPK_ON: c_int = 0;
const SPITZ_SPK_OFF: c_int = 1;

/* audio clock in Hz - rounded from 12.235MHz */
const SPITZ_AUDIO_CLOCK: c_uint = 12288000;

const WM8750_SYSCLK: c_int = 0;
const PXA2XX_I2S_SYSCLK: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const GPIOD_OUT_LOW: c_int = 0;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
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
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
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

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub shift_r: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn() -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
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
    pub owner: *mut module,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub fully_routed: bool,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync_unlocked(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_card;
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

static mut spitz_jack_func: c_int = 0;
static mut spitz_spk_func: c_int = 0;
static mut gpiod_mic: *mut gpio_desc = ptr::null_mut();
static mut gpiod_mute_l: *mut gpio_desc = ptr::null_mut();
static mut gpiod_mute_r: *mut gpio_desc = ptr::null_mut();

unsafe extern "C" fn spitz_ext_control(dapm: *mut snd_soc_dapm_context) {
    snd_soc_dapm_mutex_lock(dapm);

    if spitz_spk_func == SPITZ_SPK_ON {
        snd_soc_dapm_enable_pin_unlocked(dapm, c"Ext Spk".as_ptr());
    } else {
        snd_soc_dapm_disable_pin_unlocked(dapm, c"Ext Spk".as_ptr());
    }

    /* set up jack connection */
    match spitz_jack_func {
        SPITZ_HP => {
            /* enable and unmute hp jack, disable mic bias */
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Headset Jack".as_ptr());
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Mic Jack".as_ptr());
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Line Jack".as_ptr());
            snd_soc_dapm_enable_pin_unlocked(dapm, c"Headphone Jack".as_ptr());
            gpiod_set_value(gpiod_mute_l, 1);
            gpiod_set_value(gpiod_mute_r, 1);
        }
        SPITZ_MIC => {
            /* enable mic jack and bias, mute hp */
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Headphone Jack".as_ptr());
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Headset Jack".as_ptr());
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Line Jack".as_ptr());
            snd_soc_dapm_enable_pin_unlocked(dapm, c"Mic Jack".as_ptr());
            gpiod_set_value(gpiod_mute_l, 0);
            gpiod_set_value(gpiod_mute_r, 0);
        }
        SPITZ_LINE => {
            /* enable line jack, disable mic bias and mute hp */
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Headphone Jack".as_ptr());
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Headset Jack".as_ptr());
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Mic Jack".as_ptr());
            snd_soc_dapm_enable_pin_unlocked(dapm, c"Line Jack".as_ptr());
            gpiod_set_value(gpiod_mute_l, 0);
            gpiod_set_value(gpiod_mute_r, 0);
        }
        SPITZ_HEADSET => {
            /* enable and unmute headset jack enable mic bias, mute L hp */
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Headphone Jack".as_ptr());
            snd_soc_dapm_enable_pin_unlocked(dapm, c"Mic Jack".as_ptr());
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Line Jack".as_ptr());
            snd_soc_dapm_enable_pin_unlocked(dapm, c"Headset Jack".as_ptr());
            gpiod_set_value(gpiod_mute_l, 0);
            gpiod_set_value(gpiod_mute_r, 1);
        }
        SPITZ_HP_OFF => {
            /* jack removed, everything off */
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Headphone Jack".as_ptr());
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Headset Jack".as_ptr());
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Mic Jack".as_ptr());
            snd_soc_dapm_disable_pin_unlocked(dapm, c"Line Jack".as_ptr());
            gpiod_set_value(gpiod_mute_l, 0);
            gpiod_set_value(gpiod_mute_r, 0);
        }
        _ => {}
    }

    snd_soc_dapm_sync_unlocked(dapm);

    snd_soc_dapm_mutex_unlock(dapm);
}

unsafe extern "C" fn spitz_startup(substream: *mut snd_pcm_substream) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm((*rtd).card);

    /* check the jack status at stream startup */
    spitz_ext_control(dapm);

    0
}

unsafe extern "C" fn spitz_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut clk: c_uint = 0;
    let mut ret: c_int = 0;

    match params_rate(params) {
        8000 | 16000 | 48000 | 96000 => {
            clk = 12288000;
        }
        11025 | 22050 | 44100 => {
            clk = 11289600;
        }
        _ => {}
    }

    /* set the codec system clock for DAC and ADC */
    ret = snd_soc_dai_set_sysclk(codec_dai, WM8750_SYSCLK, clk, SND_SOC_CLOCK_IN);
    if ret < 0 {
        return ret;
    }

    /* set the I2S system clock as input (unused) */
    ret = snd_soc_dai_set_sysclk(cpu_dai, PXA2XX_I2S_SYSCLK, 0, SND_SOC_CLOCK_IN);
    if ret < 0 {
        return ret;
    }

    0
}

static spitz_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(spitz_startup),
    hw_params: Some(spitz_hw_params),
};

unsafe extern "C" fn spitz_get_jack(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    (*ucontrol).value.enumerated.item[0] = spitz_jack_func as c_uint;
    0
}

unsafe extern "C" fn spitz_set_jack(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let card: *mut snd_soc_card = snd_kcontrol_chip(kcontrol);
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);

    if spitz_jack_func == (*ucontrol).value.enumerated.item[0] as c_int {
        return 0;
    }

    spitz_jack_func = (*ucontrol).value.enumerated.item[0] as c_int;
    spitz_ext_control(dapm);
    1
}

unsafe extern "C" fn spitz_get_spk(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    (*ucontrol).value.enumerated.item[0] = spitz_spk_func as c_uint;
    0
}

unsafe extern "C" fn spitz_set_spk(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let card: *mut snd_soc_card = snd_kcontrol_chip(kcontrol);
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);

    if spitz_spk_func == (*ucontrol).value.enumerated.item[0] as c_int {
        return 0;
    }

    spitz_spk_func = (*ucontrol).value.enumerated.item[0] as c_int;
    spitz_ext_control(dapm);
    1
}

fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> c_int {
    event
}

unsafe extern "C" fn spitz_mic_bias(
    _w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    gpiod_set_value_cansleep(gpiod_mic, SND_SOC_DAPM_EVENT_ON(event));
    0
}

/* spitz machine dapm widgets */
static wm8750_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget {
        id: 0,
        name: c"Headphone Jack".as_ptr(),
        event: None,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: c"Mic Jack".as_ptr(),
        event: Some(spitz_mic_bias),
    },
    snd_soc_dapm_widget {
        id: 0,
        name: c"Ext Spk".as_ptr(),
        event: None,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: c"Line Jack".as_ptr(),
        event: None,
    },

    /* headset is a mic and mono headphone */
    snd_soc_dapm_widget {
        id: 0,
        name: c"Headset Jack".as_ptr(),
        event: None,
    },
];

/* Spitz machine audio_map */
static spitz_audio_map: [snd_soc_dapm_route; 9] = [
    /* headphone connected to LOUT1, ROUT1 */
    snd_soc_dapm_route {
        sink: c"Headphone Jack".as_ptr(),
        control: ptr::null(),
        source: c"LOUT1".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone Jack".as_ptr(),
        control: ptr::null(),
        source: c"ROUT1".as_ptr(),
    },

    /* headset connected to ROUT1 and LINPUT1 with bias (def below) */
    snd_soc_dapm_route {
        sink: c"Headset Jack".as_ptr(),
        control: ptr::null(),
        source: c"ROUT1".as_ptr(),
    },

    /* ext speaker connected to LOUT2, ROUT2  */
    snd_soc_dapm_route {
        sink: c"Ext Spk".as_ptr(),
        control: ptr::null(),
        source: c"ROUT2".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Ext Spk".as_ptr(),
        control: ptr::null(),
        source: c"LOUT2".as_ptr(),
    },

    /* mic is connected to input 1 - with bias */
    snd_soc_dapm_route {
        sink: c"LINPUT1".as_ptr(),
        control: ptr::null(),
        source: c"Mic Bias".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Mic Bias".as_ptr(),
        control: ptr::null(),
        source: c"Mic Jack".as_ptr(),
    },

    /* line is connected to input 1 - no bias */
    snd_soc_dapm_route {
        sink: c"LINPUT1".as_ptr(),
        control: ptr::null(),
        source: c"Line Jack".as_ptr(),
    },
];

static jack_function: [*const c_char; 5] = [
    c"Headphone".as_ptr(),
    c"Mic".as_ptr(),
    c"Line".as_ptr(),
    c"Headset".as_ptr(),
    c"Off".as_ptr(),
];
static spk_function: [*const c_char; 2] = [c"On".as_ptr(), c"Off".as_ptr()];
static spitz_enum: [soc_enum; 2] = [
    soc_enum {
        reg: 0,
        shift_l: 0,
        shift_r: 0,
        items: 5,
        texts: jack_function.as_ptr(),
        mask: 0,
    },
    soc_enum {
        reg: 0,
        shift_l: 0,
        shift_r: 0,
        items: 2,
        texts: spk_function.as_ptr(),
        mask: 0,
    },
];

static wm8750_spitz_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: 0,
        name: c"Jack Function".as_ptr(),
        info: None,
        get: Some(spitz_get_jack),
        put: Some(spitz_set_jack),
        private_value: unsafe { &spitz_enum[0] as *const soc_enum as usize },
    },
    snd_kcontrol_new {
        iface: 0,
        name: c"Speaker Function".as_ptr(),
        info: None,
        get: Some(spitz_get_spk),
        put: Some(spitz_set_spk),
        private_value: unsafe { &spitz_enum[1] as *const soc_enum as usize },
    },
];

/* spitz digital audio interface glue - connects codec <--> CPU */
/* SND_SOC_DAILINK_DEFS(wm8750,
 *     DAILINK_COMP_ARRAY(COMP_CPU("pxa2xx-i2s")),
 *     DAILINK_COMP_ARRAY(COMP_CODEC("wm8750.0-001b", "wm8750-hifi")),
 *     DAILINK_COMP_ARRAY(COMP_PLATFORM("pxa-pcm-audio")));
 */

static mut spitz_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: c"wm8750".as_ptr(),
    stream_name: c"WM8750".as_ptr(),
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
    ops: &spitz_ops,
    /* SND_SOC_DAILINK_REG(wm8750) */
};

/* spitz audio machine driver */
static mut snd_soc_spitz: snd_soc_card = snd_soc_card {
    name: c"Spitz".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dev: ptr::null_mut(),
    dai_link: unsafe { &mut spitz_dai },
    num_links: 1,

    controls: wm8750_spitz_controls.as_ptr(),
    num_controls: wm8750_spitz_controls.len() as c_uint,
    dapm_widgets: wm8750_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8750_dapm_widgets.len() as c_uint,
    dapm_routes: spitz_audio_map.as_ptr(),
    num_dapm_routes: spitz_audio_map.len() as c_uint,
    fully_routed: true,
};

unsafe extern "C" fn spitz_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &mut snd_soc_spitz;
    let mut ret: c_int;

    gpiod_mic = devm_gpiod_get(&mut (*pdev).dev, c"mic".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR(gpiod_mic as *const c_void) {
        return PTR_ERR(gpiod_mic as *const c_void);
    }
    gpiod_mute_l = devm_gpiod_get(&mut (*pdev).dev, c"mute-l".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR(gpiod_mute_l as *const c_void) {
        return PTR_ERR(gpiod_mute_l as *const c_void);
    }
    gpiod_mute_r = devm_gpiod_get(&mut (*pdev).dev, c"mute-r".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR(gpiod_mute_r as *const c_void) {
        return PTR_ERR(gpiod_mute_r as *const c_void);
    }

    (*card).dev = &mut (*pdev).dev;

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            c"snd_soc_register_card() failed: %d\n".as_ptr(),
            ret,
        );
    }

    ret
}

static mut spitz_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: c"spitz-audio".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(spitz_probe),
};

/* module_platform_driver(spitz_driver); */

/* MODULE_AUTHOR("Richard Purdie"); */
/* MODULE_DESCRIPTION("ALSA SoC Spitz"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:spitz-audio"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
