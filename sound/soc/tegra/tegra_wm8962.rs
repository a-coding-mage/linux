// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra_wm8962.c - Tegra machine ASoC driver for boards using WM8962 codec.
 *
 * Copyright (C) 2021-2024 Jonas Schwöbel <jonasschwoebel@yahoo.de>
 *			   Svyatoslav Ryhel <clamor95@gmail.com>
 *
 * Based on tegra_wm8903 code copyright/by:
 *
 * Author: Stephen Warren <swarren@nvidia.com>
 * Copyright (C) 2010-2012 - NVIDIA, Inc.
 *
 * Based on code copyright/by:
 *
 * (c) 2009, 2010 Nvidia Graphics Pvt. Ltd.
 *
 * Copyright 2007 Wolfson Microelectronics PLC.
 * Author: Graeme Gregory
 *         graeme.gregory@wolfsonmicro.com or linux@wolfsonmicro.com
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

// Dependencies originally supplied by Linux, ASoC, WM8962, and Tegra headers.
extern "C" {
    static THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn tegra_asoc_machine_init(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn wm8962_mic_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack);
    fn snd_soc_dapm_force_enable_pin(
        dapm: *mut snd_soc_dapm_context,
        pin: *const c_char,
    ) -> c_int;
    fn snd_soc_get_pcm_runtime(
        card: *mut snd_soc_card,
        dai_link: *mut snd_soc_dai_link,
    ) -> *mut snd_soc_pcm_runtime;
    fn tegra_asoc_machine_probe(pdev: *mut platform_device) -> c_int;
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
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub dai_fmt: c_uint,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub components: *const c_char,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub fully_routed: bool,
}

#[repr(C)]
pub struct tegra_asoc_data {
    pub mclk_rate: Option<unsafe extern "C" fn(c_uint) -> c_uint>,
    pub card: *mut snd_soc_card,
    pub add_common_dapm_widgets: bool,
    pub add_common_controls: bool,
    pub add_common_snd_ops: bool,
    pub add_mic_jack: bool,
    pub add_hp_jack: bool,
}

#[repr(C)]
pub struct tegra_machine {
    pub gpiod_mic_det: *mut c_void,
    pub asoc: *const tegra_asoc_data,
    pub mic_jack: *mut snd_soc_jack,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

const SND_JACK_MICROPHONE: c_int = 0x0008;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

static mut tegra_wm8962_mic_jack_pins: [snd_soc_jack_pin; 1] = [snd_soc_jack_pin {
    pin: b"Mic Jack\0".as_ptr() as *const c_char,
    mask: SND_JACK_MICROPHONE,
}];

unsafe extern "C" fn tegra_wm8962_mclk_rate(srate: c_uint) -> c_uint {
    let mclk: c_uint;

    match srate {
        8000 | 16000 | 24000 | 32000 | 48000 | 64000 | 96000 => {
            mclk = 12288000;
        }
        11025 | 22050 | 44100 | 88200 => {
            mclk = 11289600;
        }
        _ => {
            mclk = 12000000;
        }
    }

    mclk
}

unsafe extern "C" fn tegra_wm8962_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let machine = snd_soc_card_get_drvdata((*rtd).card) as *mut tegra_machine;
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut err: c_int;

    err = tegra_asoc_machine_init(rtd);
    if err != 0 {
        return err;
    }

    if (*machine).gpiod_mic_det.is_null() && (*(*machine).asoc).add_mic_jack {
        let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
        let component = (*codec_dai).component;

        err = snd_soc_card_jack_new_pins(
            (*rtd).card,
            b"Mic Jack\0".as_ptr() as *const c_char,
            SND_JACK_MICROPHONE,
            (*machine).mic_jack,
            tegra_wm8962_mic_jack_pins.as_mut_ptr(),
            tegra_wm8962_mic_jack_pins.len() as c_uint,
        );
        if err != 0 {
            dev_err(
                (*rtd).dev,
                b"Mic Jack creation failed: %d\n\0".as_ptr() as *const c_char,
                err,
            );
            return err;
        }

        wm8962_mic_detect(component, (*machine).mic_jack);
    }

    snd_soc_dapm_force_enable_pin(dapm, b"MICBIAS\0".as_ptr() as *const c_char);

    0
}

unsafe extern "C" fn tegra_wm8962_remove(card: *mut snd_soc_card) -> c_int {
    let link = (*card).dai_link.add(0);
    let rtd = snd_soc_get_pcm_runtime(card, link);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component = (*codec_dai).component;

    wm8962_mic_detect(component, ptr::null_mut());

    0
}

// Translation of SND_SOC_DAILINK_DEFS(wm8962_hifi,
//     DAILINK_COMP_ARRAY(COMP_EMPTY()),
//     DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "wm8962")),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut wm8962_hifi_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
}];

static mut wm8962_hifi_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: b"wm8962\0".as_ptr() as *const c_char,
}];

static mut wm8962_hifi_platforms: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component {
        name: ptr::null(),
        dai_name: ptr::null(),
    }];

static mut tegra_wm8962_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: b"WM8962\0".as_ptr() as *const c_char,
    stream_name: b"WM8962 PCM\0".as_ptr() as *const c_char,
    init: Some(tegra_wm8962_init),
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
    cpus: unsafe { wm8962_hifi_cpus.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { wm8962_hifi_codecs.as_mut_ptr() },
    num_codecs: 1,
    platforms: unsafe { wm8962_hifi_platforms.as_mut_ptr() },
    num_platforms: 1,
};

static mut snd_soc_tegra_wm8962: snd_soc_card = snd_soc_card {
    components: b"codec:wm8962\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &mut tegra_wm8962_dai },
    num_links: 1,
    remove: Some(tegra_wm8962_remove),
    fully_routed: true,
};

static tegra_wm8962_data: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_wm8962_mclk_rate),
    card: unsafe { &mut snd_soc_tegra_wm8962 },
    add_common_dapm_widgets: true,
    add_common_controls: true,
    add_common_snd_ops: true,
    add_mic_jack: true,
    add_hp_jack: true,
};

static tegra_wm8962_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"nvidia,tegra-audio-wm8962\0".as_ptr() as *const c_char,
        data: &tegra_wm8962_data as *const tegra_asoc_data as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, tegra_wm8962_of_match);

static mut tegra_wm8962_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"tegra-wm8962\0".as_ptr() as *const c_char,
        of_match_table: tegra_wm8962_of_match.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(tegra_asoc_machine_probe),
};

// module_platform_driver(tegra_wm8962_driver);

// MODULE_AUTHOR("Jonas Schwöbel <jonasschwoebel@yahoo.de>");
// MODULE_AUTHOR("Svyatoslav Ryhel <clamor95@gmail.com>");
// MODULE_DESCRIPTION("Tegra+WM8962 machine ASoC driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
