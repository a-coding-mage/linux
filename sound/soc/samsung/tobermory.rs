// SPDX-License-Identifier: GPL-2.0+
//
// Tobermory audio support
//
// Copyright 2011 Wolfson Microelectronics

// C dependencies: <sound/soc.h>, <sound/soc-dapm.h>, <sound/jack.h>,
// <linux/module.h>, "../codecs/wm8962.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
    // SND_SOC_DAILINK_REG(cpu)
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int,
    >,
}

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
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub set_bias_level: Option<
        unsafe extern "C" fn(
            *mut snd_soc_card,
            *mut snd_soc_dapm_context,
            snd_soc_bias_level,
        ) -> c_int,
    >,
    pub set_bias_level_post: Option<
        unsafe extern "C" fn(
            *mut snd_soc_card,
            *mut snd_soc_dapm_context,
            snd_soc_bias_level,
        ) -> c_int,
    >,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub fully_routed: bool,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub dev: *mut device,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_OFF = 0,
    SND_SOC_BIAS_STANDBY = 1,
    SND_SOC_BIAS_PREPARE = 2,
    SND_SOC_BIAS_ON = 3,
}

const WM8962_FLL: c_int = 1;
const WM8962_FLL_MCLK: c_int = 2;
const WM8962_SYSCLK_FLL: c_int = 3;
const WM8962_SYSCLK_MCLK: c_int = 4;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 2;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 4;
const SND_JACK_MICROPHONE: c_int = 0x0001;
const SND_JACK_HEADSET: c_int = 0x0002;
const SND_JACK_BTN_0: c_int = 0x0004;

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_get_pcm_runtime(
        card: *mut snd_soc_card,
        dai_link: *mut snd_soc_dai_link,
    ) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_dapm_to_dev(dapm: *mut snd_soc_dapm_context) -> *mut device;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn wm8962_mic_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
}

static mut sample_rate: c_int = 44100;

unsafe extern "C" fn tobermory_set_bias_level(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime;
    let codec_dai: *mut snd_soc_dai;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, &mut *(*card).dai_link.add(0));
    codec_dai = snd_soc_rtd_to_codec(rtd, 0);

    if snd_soc_dapm_to_dev(dapm) != (*codec_dai).dev {
        return 0;
    }

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_STANDBY {
                ret = snd_soc_dai_set_pll(
                    codec_dai,
                    WM8962_FLL,
                    WM8962_FLL_MCLK,
                    32768,
                    (sample_rate * 512) as c_uint,
                );
                if ret < 0 {
                    pr_err(c"Failed to start FLL: %d\n".as_ptr(), ret);
                }

                ret = snd_soc_dai_set_sysclk(
                    codec_dai,
                    WM8962_SYSCLK_FLL,
                    (sample_rate * 512) as c_uint,
                    SND_SOC_CLOCK_IN,
                );
                if ret < 0 {
                    pr_err(c"Failed to set SYSCLK: %d\n".as_ptr(), ret);
                    snd_soc_dai_set_pll(codec_dai, WM8962_FLL, 0, 0, 0);
                    return ret;
                }
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn tobermory_set_bias_level_post(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime;
    let codec_dai: *mut snd_soc_dai;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, &mut *(*card).dai_link.add(0));
    codec_dai = snd_soc_rtd_to_codec(rtd, 0);

    if snd_soc_dapm_to_dev(dapm) != (*codec_dai).dev {
        return 0;
    }

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            ret = snd_soc_dai_set_sysclk(
                codec_dai,
                WM8962_SYSCLK_MCLK,
                32768,
                SND_SOC_CLOCK_IN,
            );
            if ret < 0 {
                pr_err(c"Failed to switch away from FLL: %d\n".as_ptr(), ret);
                return ret;
            }

            ret = snd_soc_dai_set_pll(codec_dai, WM8962_FLL, 0, 0, 0);
            if ret < 0 {
                pr_err(c"Failed to stop FLL: %d\n".as_ptr(), ret);
                return ret;
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn tobermory_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    sample_rate = params_rate(params);

    0
}

static tobermory_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(tobermory_hw_params),
};

// SND_SOC_DAILINK_DEFS(cpu,
//     DAILINK_COMP_ARRAY(COMP_CPU("samsung-i2s.0")),
//     DAILINK_COMP_ARRAY(COMP_CODEC("wm8962.1-001a", "wm8962")),
//     DAILINK_COMP_ARRAY(COMP_PLATFORM("samsung-i2s.0")));

static mut tobermory_dai: [snd_soc_dai_link; 1] = [snd_soc_dai_link {
    name: c"CPU".as_ptr(),
    stream_name: c"CPU".as_ptr(),
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
    ops: &tobermory_ops,
}];

// SOC_DAPM_PIN_SWITCH("Main Speaker"), SOC_DAPM_PIN_SWITCH("DMIC")
static controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { _private: [] }, snd_kcontrol_new {
    _private: [],
}];

// SND_SOC_DAPM_HP/MIC/SPK widget macro initializers.
static widgets: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static audio_paths: [snd_soc_dapm_route; 13] = [
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: core::ptr::null(), source: c"HPOUTL".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: core::ptr::null(), source: c"HPOUTR".as_ptr() },
    snd_soc_dapm_route { sink: c"Main Speaker".as_ptr(), control: core::ptr::null(), source: c"SPKOUTL".as_ptr() },
    snd_soc_dapm_route { sink: c"Main Speaker".as_ptr(), control: core::ptr::null(), source: c"SPKOUTR".as_ptr() },
    snd_soc_dapm_route { sink: c"Headset Mic".as_ptr(), control: core::ptr::null(), source: c"MICBIAS".as_ptr() },
    snd_soc_dapm_route { sink: c"IN4L".as_ptr(), control: core::ptr::null(), source: c"Headset Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"IN4R".as_ptr(), control: core::ptr::null(), source: c"Headset Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"AMIC".as_ptr(), control: core::ptr::null(), source: c"MICBIAS".as_ptr() },
    snd_soc_dapm_route { sink: c"IN1L".as_ptr(), control: core::ptr::null(), source: c"AMIC".as_ptr() },
    snd_soc_dapm_route { sink: c"IN1R".as_ptr(), control: core::ptr::null(), source: c"AMIC".as_ptr() },
    snd_soc_dapm_route { sink: c"DMIC".as_ptr(), control: core::ptr::null(), source: c"MICBIAS".as_ptr() },
    snd_soc_dapm_route { sink: c"DMICDAT".as_ptr(), control: core::ptr::null(), source: c"DMIC".as_ptr() },
];

static mut tobermory_headset: snd_soc_jack = snd_soc_jack { _private: [] };

/* Headset jack detection DAPM pins */
static mut tobermory_headset_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn tobermory_late_probe(card: *mut snd_soc_card) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime;
    let component: *mut snd_soc_component;
    let codec_dai: *mut snd_soc_dai;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, &mut *(*card).dai_link.add(0));
    component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    codec_dai = snd_soc_rtd_to_codec(rtd, 0);

    ret = snd_soc_dai_set_sysclk(codec_dai, WM8962_SYSCLK_MCLK, 32768, SND_SOC_CLOCK_IN);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        c"Headset".as_ptr(),
        SND_JACK_HEADSET | SND_JACK_BTN_0,
        &mut tobermory_headset,
        tobermory_headset_pins.as_mut_ptr(),
        tobermory_headset_pins.len() as c_uint,
    );
    if ret != 0 {
        return ret;
    }

    wm8962_mic_detect(component, &mut tobermory_headset);

    0
}

static mut tobermory: snd_soc_card = snd_soc_card {
    name: c"Tobermory".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { tobermory_dai.as_mut_ptr() },
    num_links: 1,
    set_bias_level: Some(tobermory_set_bias_level),
    set_bias_level_post: Some(tobermory_set_bias_level_post),
    controls: controls.as_ptr(),
    num_controls: controls.len() as c_int,
    dapm_widgets: widgets.as_ptr(),
    num_dapm_widgets: widgets.len() as c_int,
    dapm_routes: audio_paths.as_ptr(),
    num_dapm_routes: audio_paths.len() as c_int,
    fully_routed: true,
    late_probe: Some(tobermory_late_probe),
    dev: core::ptr::null_mut(),
};

unsafe extern "C" fn tobermory_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &mut tobermory;
    let ret: c_int;

    (*card).dev = &mut (*pdev).dev;

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        dev_err_probe(
            &mut (*pdev).dev,
            ret,
            c"snd_soc_register_card() failed\n".as_ptr(),
        );
    }

    ret
}

static mut tobermory_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: c"tobermory".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    probe: Some(tobermory_probe),
};

// module_platform_driver(tobermory_driver);
//
// MODULE_DESCRIPTION("Tobermory audio support");
// MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:tobermory");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
