// SPDX-License-Identifier: GPL-2.0-only
// Based on sof_sdw_rt5682.c
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2023 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.

/*
 *  soc_sdw_cs42l43 - Helpers to handle CS42L43 from generic machine driver
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const CS42L43_SPK_VOLUME_0DB: c_int = 128; /* 0dB Max */

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;

const SND_JACK_HEADPHONE: c_uint = 0x0001;
const SND_JACK_MICROPHONE: c_uint = 0x0002;
const SND_JACK_LINEOUT: c_uint = 0x0004;
const SND_JACK_MECHANICAL: c_uint = 0x0008;
const SND_JACK_AVOUT: c_uint = 0x0010;
const SND_JACK_HEADSET: c_uint = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_uint = 0x4000;
const SND_JACK_BTN_1: c_uint = 0x2000;
const SND_JACK_BTN_2: c_uint = 0x1000;
const SND_JACK_BTN_3: c_uint = 0x0800;

const KEY_PLAYPAUSE: c_uint = 164;
const KEY_VOICECOMMAND: c_uint = 246;
const KEY_VOLUMEUP: c_uint = 115;
const KEY_VOLUMEDOWN: c_uint = 114;

const CS42L43_SYSCLK: c_int = 0;
const CS42L43_SYSCLK_SDW: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;

#[repr(C)]
pub struct device {
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub components: *mut c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct asoc_sdw_codec_info {
    pub amp_num: c_int,
}

#[repr(C)]
pub struct asoc_sdw_mc_private {
    pub sdw_headset: snd_soc_jack,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_rtd_codec {
    pub component: *mut snd_soc_component,
}

static CS42L43_HS_MAP: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: ptr::null(),
        source: c"cs42l43 AMP3_OUT".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: ptr::null(),
        source: c"cs42l43 AMP4_OUT".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"cs42l43 ADC1_IN1_P".as_ptr(),
        control: ptr::null(),
        source: c"Headset Mic".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"cs42l43 ADC1_IN1_N".as_ptr(),
        control: ptr::null(),
        source: c"Headset Mic".as_ptr(),
    },
];

static CS42L43_SPK_MAP: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"cs42l43 AMP1_OUT_P".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"cs42l43 AMP1_OUT_N".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"cs42l43 AMP2_OUT_P".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"cs42l43 AMP2_OUT_N".as_ptr(),
    },
];

static CS42L43_DMIC_MAP: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"cs42l43 PDM1_DIN".as_ptr(),
        control: ptr::null(),
        source: c"DMIC".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"cs42l43 PDM2_DIN".as_ptr(),
        control: ptr::null(),
        source: c"DMIC".as_ptr(),
    },
];

static mut SOC_JACK_PINS: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE | SND_JACK_LINEOUT,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" {
    fn snd_soc_rtd_to_codec(
        rtd: *mut snd_soc_pcm_runtime,
        num: c_int,
    ) -> *mut snd_soc_rtd_codec;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_uint, keytype: c_uint);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_component_set_sysclk(
        component: *mut snd_soc_component,
        clk_id: c_int,
        source: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_limit_volume(
        card: *mut snd_soc_card,
        name: *const c_char,
        max: c_int,
    ) -> c_int;
    fn asoc_sdw_bridge_cs35l56_spk_init(
        card: *mut snd_soc_card,
        dai_links: *mut snd_soc_dai_link,
        info: *mut asoc_sdw_codec_info,
        playback: bool,
    ) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_cs42l43_hs_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let ctx = snd_soc_card_get_drvdata((*rtd).card) as *mut asoc_sdw_mc_private;
    let jack = &mut (*ctx).sdw_headset as *mut snd_soc_jack;
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    (*card).components = devm_kasprintf(
        (*card).dev,
        GFP_KERNEL,
        c"%s hs:cs42l43".as_ptr(),
        (*card).components,
    );
    if (*card).components.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_dapm_add_routes(
        dapm,
        CS42L43_HS_MAP.as_ptr(),
        CS42L43_HS_MAP.len() as c_int,
    );
    if ret != 0 {
        dev_err(
            (*card).dev,
            c"cs42l43 hs map addition failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        c"Jack".as_ptr(),
        SND_JACK_MECHANICAL
            | SND_JACK_AVOUT
            | SND_JACK_HEADSET
            | SND_JACK_LINEOUT
            | SND_JACK_BTN_0
            | SND_JACK_BTN_1
            | SND_JACK_BTN_2
            | SND_JACK_BTN_3,
        jack,
        SOC_JACK_PINS.as_mut_ptr(),
        SOC_JACK_PINS.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*card).dev, c"Failed to create jack: %d\n".as_ptr(), ret);
        return ret;
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

    ret = snd_soc_component_set_jack(component, jack, ptr::null_mut());
    if ret != 0 {
        dev_err((*card).dev, c"Failed to register jack: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_component_set_sysclk(
        component,
        CS42L43_SYSCLK,
        CS42L43_SYSCLK_SDW,
        0,
        SND_SOC_CLOCK_IN,
    );
    if ret != 0 {
        dev_err((*card).dev, c"Failed to set sysclk: %d\n".as_ptr(), ret);
    }

    ret
}
// EXPORT_SYMBOL_NS(asoc_sdw_cs42l43_hs_rtd_init, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_cs42l43_spk_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_limit_volume(
        card,
        c"cs42l43 Speaker Digital Volume".as_ptr(),
        CS42L43_SPK_VOLUME_0DB,
    );
    if ret != 0 {
        dev_err(
            (*card).dev,
            c"cs42l43 speaker volume limit failed: %d\n".as_ptr(),
            ret,
        );
    } else {
        dev_info(
            (*card).dev,
            c"Setting CS42L43 Speaker volume limit to %d\n".as_ptr(),
            CS42L43_SPK_VOLUME_0DB,
        );
    }

    ret = snd_soc_dapm_add_routes(
        dapm,
        CS42L43_SPK_MAP.as_ptr(),
        CS42L43_SPK_MAP.len() as c_int,
    );
    if ret != 0 {
        dev_err(
            (*card).dev,
            c"cs42l43 speaker map addition failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = snd_soc_component_set_sysclk(
        component,
        CS42L43_SYSCLK,
        CS42L43_SYSCLK_SDW,
        0,
        SND_SOC_CLOCK_IN,
    );
    if ret != 0 {
        dev_err((*card).dev, c"Failed to set sysclk: %d\n".as_ptr(), ret);
    }

    ret
}
// EXPORT_SYMBOL_NS(asoc_sdw_cs42l43_spk_rtd_init, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_cs42l43_spk_init(
    card: *mut snd_soc_card,
    dai_links: *mut snd_soc_dai_link,
    info: *mut asoc_sdw_codec_info,
    playback: bool,
) -> c_int {
    /* Do init on playback link only. */
    if !playback {
        return 0;
    }

    (*info).amp_num += 1;

    asoc_sdw_bridge_cs35l56_spk_init(card, dai_links, info, playback)
}
// EXPORT_SYMBOL_NS(asoc_sdw_cs42l43_spk_init, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_cs42l43_dmic_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    (*card).components = devm_kasprintf(
        (*card).dev,
        GFP_KERNEL,
        c"%s mic:cs42l43-dmic".as_ptr(),
        (*card).components,
    );
    if (*card).components.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_dapm_add_routes(
        dapm,
        CS42L43_DMIC_MAP.as_ptr(),
        CS42L43_DMIC_MAP.len() as c_int,
    );
    if ret != 0 {
        dev_err(
            (*card).dev,
            c"cs42l43 dmic map addition failed: %d\n".as_ptr(),
            ret,
        );
    }

    ret
}
// EXPORT_SYMBOL_NS(asoc_sdw_cs42l43_dmic_rtd_init, "SND_SOC_SDW_UTILS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
