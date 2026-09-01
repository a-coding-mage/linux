// SPDX-License-Identifier: GPL-2.0+
//
// Lowland audio support
//
// Copyright 2011 Wolfson Microelectronics

// C includes translated as external dependencies:
// <sound/soc.h>, <sound/soc-dapm.h>, <sound/jack.h>, <linux/module.h>
// "../codecs/wm5100.h", "../codecs/wm9081.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

const MCLK1_RATE: c_uint = 44100 * 512;
const CLKOUT_RATE: c_uint = 44100 * 256;

const SND_JACK_HEADPHONE: c_uint = 0;
const SND_JACK_MICROPHONE: c_uint = 0;
const SND_JACK_LINEOUT: c_uint = 0;
const SND_JACK_HEADSET: c_uint = 0;
const SND_JACK_BTN_0: c_uint = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const WM5100_CLK_SYSCLK: c_int = 0;
const WM5100_CLKSRC_MCLK1: c_int = 0;
const WM5100_CLK_OPCLK: c_int = 0;
const WM9081_SYSCLK_MCLK: c_int = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const THIS_MODULE: *mut module = ptr::null_mut();

#[repr(C)]
pub struct module {
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
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub formats: u64,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub ignore_suspend: c_int,
    pub c2c_params: *const snd_soc_pcm_stream,
    pub num_c2c_params: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_uint,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_component_set_sysclk(
        component: *mut snd_soc_component,
        clk_id: c_int,
        source: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn wm5100_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack);
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

static mut lowland_headset: snd_soc_jack = snd_soc_jack { _private: [] };

/* Headset jack detection DAPM pins */
static mut lowland_headset_pins: [snd_soc_jack_pin; 3] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Line Out".as_ptr(),
        mask: SND_JACK_LINEOUT,
    },
];

unsafe extern "C" fn lowland_wm5100_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let mut ret: c_int;

    ret = snd_soc_component_set_sysclk(
        component,
        WM5100_CLK_SYSCLK,
        WM5100_CLKSRC_MCLK1,
        MCLK1_RATE,
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        pr_err(c"Failed to set SYSCLK clock source: %d\n".as_ptr(), ret);
        return ret;
    }

    /* Clock OPCLK, used by the other audio components. */
    ret = snd_soc_component_set_sysclk(component, WM5100_CLK_OPCLK, 0, CLKOUT_RATE, 0);
    if ret < 0 {
        pr_err(c"Failed to set OPCLK rate: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"Headset".as_ptr(),
        SND_JACK_LINEOUT | SND_JACK_HEADSET | SND_JACK_BTN_0,
        &raw mut lowland_headset,
        lowland_headset_pins.as_mut_ptr(),
        lowland_headset_pins.len() as c_uint,
    );
    if ret != 0 {
        return ret;
    }

    wm5100_detect(component, &raw mut lowland_headset);

    0
}

unsafe extern "C" fn lowland_wm9081_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let dapm = snd_soc_card_to_dapm((*rtd).card);

    snd_soc_dapm_disable_pin(dapm, c"LINEOUT".as_ptr());

    /* At any time the WM9081 is active it will have this clock */
    snd_soc_component_set_sysclk(component, WM9081_SYSCLK_MCLK, 0, CLKOUT_RATE, 0)
}

static sub_params: snd_soc_pcm_stream = snd_soc_pcm_stream {
    formats: SNDRV_PCM_FMTBIT_S32_LE,
    rate_min: 44100,
    rate_max: 44100,
    channels_min: 2,
    channels_max: 2,
};

// SND_SOC_DAILINK_DEFS(cpu,
//      DAILINK_COMP_ARRAY(COMP_CPU("samsung-i2s.0")),
//      DAILINK_COMP_ARRAY(COMP_CODEC("wm5100.1-001a", "wm5100-aif1")),
//      DAILINK_COMP_ARRAY(COMP_PLATFORM("samsung-i2s.0")));
static mut cpu_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"samsung-i2s.0".as_ptr(),
    dai_name: ptr::null(),
}];
static mut cpu_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"wm5100.1-001a".as_ptr(),
    dai_name: c"wm5100-aif1".as_ptr(),
}];
static mut cpu_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"samsung-i2s.0".as_ptr(),
    dai_name: ptr::null(),
}];

// SND_SOC_DAILINK_DEFS(baseband,
//      DAILINK_COMP_ARRAY(COMP_CPU("wm5100-aif2")),
//      DAILINK_COMP_ARRAY(COMP_CODEC("wm1250-ev1.1-0027", "wm1250-ev1")));
static mut baseband_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"wm5100-aif2".as_ptr(),
    dai_name: ptr::null(),
}];
static mut baseband_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"wm1250-ev1.1-0027".as_ptr(),
    dai_name: c"wm1250-ev1".as_ptr(),
}];

// SND_SOC_DAILINK_DEFS(speaker,
//      DAILINK_COMP_ARRAY(COMP_CPU("wm5100-aif3")),
//      DAILINK_COMP_ARRAY(COMP_CODEC("wm9081.1-006c", "wm9081-hifi")));
static mut speaker_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"wm5100-aif3".as_ptr(),
    dai_name: ptr::null(),
}];
static mut speaker_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"wm9081.1-006c".as_ptr(),
    dai_name: c"wm9081-hifi".as_ptr(),
}];

static mut lowland_dai: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: c"CPU".as_ptr(),
        stream_name: c"CPU".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        ignore_suspend: 0,
        c2c_params: ptr::null(),
        num_c2c_params: 0,
        init: Some(lowland_wm5100_init),
        cpus: cpu_cpus.as_mut_ptr(),
        num_cpus: cpu_cpus.len() as c_uint,
        codecs: cpu_codecs.as_mut_ptr(),
        num_codecs: cpu_codecs.len() as c_uint,
        platforms: cpu_platforms.as_mut_ptr(),
        num_platforms: cpu_platforms.len() as c_uint,
    },
    snd_soc_dai_link {
        name: c"Baseband".as_ptr(),
        stream_name: c"Baseband".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        ignore_suspend: 1,
        c2c_params: ptr::null(),
        num_c2c_params: 0,
        init: None,
        cpus: baseband_cpus.as_mut_ptr(),
        num_cpus: baseband_cpus.len() as c_uint,
        codecs: baseband_codecs.as_mut_ptr(),
        num_codecs: baseband_codecs.len() as c_uint,
        platforms: ptr::null_mut(),
        num_platforms: 0,
    },
    snd_soc_dai_link {
        name: c"Sub Speaker".as_ptr(),
        stream_name: c"Sub Speaker".as_ptr(),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        ignore_suspend: 1,
        c2c_params: &sub_params,
        num_c2c_params: 1,
        init: Some(lowland_wm9081_init),
        cpus: speaker_cpus.as_mut_ptr(),
        num_cpus: speaker_cpus.len() as c_uint,
        codecs: speaker_codecs.as_mut_ptr(),
        num_codecs: speaker_codecs.len() as c_uint,
        platforms: ptr::null_mut(),
        num_platforms: 0,
    },
];

static mut lowland_codec_conf: [snd_soc_codec_conf; 1] = [snd_soc_codec_conf {
    dlc: snd_soc_dai_link_component {
        name: c"wm9081.1-006c".as_ptr(),
        dai_name: ptr::null(),
    },
    name_prefix: c"Sub".as_ptr(),
}];

static controls: [snd_kcontrol_new; 7] = [
    snd_kcontrol_new {
        name: c"Main Speaker".as_ptr(),
    },
    snd_kcontrol_new {
        name: c"Main DMIC".as_ptr(),
    },
    snd_kcontrol_new {
        name: c"Main AMIC".as_ptr(),
    },
    snd_kcontrol_new {
        name: c"WM1250 Input".as_ptr(),
    },
    snd_kcontrol_new {
        name: c"WM1250 Output".as_ptr(),
    },
    snd_kcontrol_new {
        name: c"Headphone".as_ptr(),
    },
    snd_kcontrol_new {
        name: c"Line Out".as_ptr(),
    },
];

static widgets: [snd_soc_dapm_widget; 6] = [
    snd_soc_dapm_widget {
        name: c"Headphone".as_ptr(),
    },
    snd_soc_dapm_widget {
        name: c"Headset Mic".as_ptr(),
    },
    snd_soc_dapm_widget {
        name: c"Line Out".as_ptr(),
    },
    snd_soc_dapm_widget {
        name: c"Main Speaker".as_ptr(),
    },
    snd_soc_dapm_widget {
        name: c"Main AMIC".as_ptr(),
    },
    snd_soc_dapm_widget {
        name: c"Main DMIC".as_ptr(),
    },
];

static audio_paths: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route {
        sink: c"Sub IN1".as_ptr(),
        control: ptr::null(),
        source: c"HPOUT2L".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Sub IN2".as_ptr(),
        control: ptr::null(),
        source: c"HPOUT2R".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Main Speaker".as_ptr(),
        control: ptr::null(),
        source: c"Sub SPKN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Main Speaker".as_ptr(),
        control: ptr::null(),
        source: c"Sub SPKP".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Main Speaker".as_ptr(),
        control: ptr::null(),
        source: c"SPKDAT1".as_ptr(),
    },
];

static mut lowland: snd_soc_card = snd_soc_card {
    name: c"Lowland".as_ptr(),
    owner: THIS_MODULE,
    dev: ptr::null_mut(),
    dai_link: lowland_dai.as_mut_ptr(),
    num_links: lowland_dai.len() as c_uint,
    codec_conf: lowland_codec_conf.as_mut_ptr(),
    num_configs: lowland_codec_conf.len() as c_uint,
    controls: controls.as_ptr(),
    num_controls: controls.len() as c_uint,
    dapm_widgets: widgets.as_ptr(),
    num_dapm_widgets: widgets.len() as c_uint,
    dapm_routes: audio_paths.as_ptr(),
    num_dapm_routes: audio_paths.len() as c_uint,
};

unsafe extern "C" fn lowland_probe(pdev: *mut platform_device) -> c_int {
    let card = &raw mut lowland;
    let mut ret: c_int;

    (*card).dev = &raw mut (*pdev).dev;

    ret = devm_snd_soc_register_card(&raw mut (*pdev).dev, card);
    if ret != 0 {
        dev_err_probe(
            &raw mut (*pdev).dev,
            ret,
            c"snd_soc_register_card() failed\n".as_ptr(),
        );
    }

    ret
}

static mut lowland_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"lowland".as_ptr(),
        pm: &raw const snd_soc_pm_ops,
    },
    probe: Some(lowland_probe),
};

// module_platform_driver(lowland_driver);

// MODULE_DESCRIPTION("Lowland audio support");
// MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:lowland");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
