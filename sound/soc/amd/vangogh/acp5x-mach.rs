// SPDX-License-Identifier: GPL-2.0+
/*
 * Machine driver for AMD Vangogh platform using either
 * NAU8821 & CS35L41 or NAU8821 & MAX98388 codecs.
 *
 * Copyright 2021 Advanced Micro Devices, Inc.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

const DRV_NAME: *const c_char = b"acp5x_mach\0".as_ptr() as *const c_char;
const DUAL_CHANNEL: c_uint = 2;
const ACP5X_NAU8821_BCLK: c_uint = 3072000;
const ACP5X_NAU8821_FREQ_OUT: c_uint = 12288000;
const ACP5X_NAU8821_COMP_NAME: *const c_char = b"i2c-NVTN2020:00\0".as_ptr() as *const c_char;
const ACP5X_NAU8821_DAI_NAME: *const c_char = b"nau8821-hifi\0".as_ptr() as *const c_char;
const ACP5X_CS35L41_COMP_LNAME: *const c_char = b"spi-VLV1776:00\0".as_ptr() as *const c_char;
const ACP5X_CS35L41_COMP_RNAME: *const c_char = b"spi-VLV1776:01\0".as_ptr() as *const c_char;
const ACP5X_CS35L41_DAI_NAME: *const c_char = b"cs35l41-pcm\0".as_ptr() as *const c_char;
const ACP5X_MAX98388_COMP_LNAME: *const c_char = b"i2c-ADS8388:00\0".as_ptr() as *const c_char;
const ACP5X_MAX98388_COMP_RNAME: *const c_char = b"i2c-ADS8388:01\0".as_ptr() as *const c_char;
const ACP5X_MAX98388_DAI_NAME: *const c_char = b"max98388-aif1\0".as_ptr() as *const c_char;

const EIO: c_int = 5;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const KEY_MEDIA: c_uint = 226;
const NAU8821_CLK_INTERNAL: c_int = 0;
const NAU8821_CLK_FLL_BLK: c_int = 1;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_SOC_CLOCK_IN: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 1;
const SNDRV_PCM_HW_PARAM_SAMPLE_BITS: c_int = 2;
const I2S_SP_INSTANCE: c_int = 0;
const I2S_HS_INSTANCE: c_int = 1;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 2;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 4;
const SND_SOC_NOPM: c_int = -1;
const SND_SOC_DAPM_PRE_PMU: c_int = 1;
const SND_SOC_DAPM_POST_PMD: c_int = 2;
const DMI_BOARD_VENDOR: c_int = 0;
const DMI_PRODUCT_NAME: c_int = 1;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub name: *const c_char,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_int,
    pub invert: c_int,
    pub event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
    pub event_flags: c_int,
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
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
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const c_char,
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
    pub playback_only: c_int,
    pub ops: *const snd_soc_ops,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub cpus: *const snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *const snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *const snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct dmi_strmatch {
    pub slot: c_int,
    pub substr: *const c_char,
}

#[repr(C)]
pub struct dmi_system_id {
    pub matches: [dmi_strmatch; 4],
    pub driver_data: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct acp5x_platform_info {
    pub play_i2s_instance: c_int,
    pub cap_i2s_instance: c_int,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_codec_dai(card: *mut snd_soc_card, dai_name: *const c_char) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        typ: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, typ: c_int, keytype: c_uint);
    fn nau8821_enable_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn snd_soc_component_set_sysclk(comp: *mut snd_soc_component, clk_id: c_int, source: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn dmi_first_match(list: *const dmi_system_id) -> *const dmi_system_id;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe fn SND_SOC_DAPM_EVENT_OFF(event: c_int) -> bool {
    event == SND_SOC_DAPM_POST_PMD
}

const fn COMP_PLATFORM(name: *const c_char) -> snd_soc_dai_link_component {
    snd_soc_dai_link_component { name, dai_name: null() }
}

const fn COMP_CPU(name: *const c_char) -> snd_soc_dai_link_component {
    snd_soc_dai_link_component { name, dai_name: null() }
}

const fn COMP_CODEC(name: *const c_char, dai_name: *const c_char) -> snd_soc_dai_link_component {
    snd_soc_dai_link_component { name, dai_name }
}

const fn COMP_CODEC_CONF(name: *const c_char) -> snd_soc_dai_link_component {
    snd_soc_dai_link_component { name, dai_name: null() }
}

const fn DMI_EXACT_MATCH(slot: c_int, substr: *const c_char) -> dmi_strmatch {
    dmi_strmatch { slot, substr }
}

const fn SOC_DAPM_PIN_SWITCH(_pin: *const c_char) -> snd_kcontrol_new {
    snd_kcontrol_new { _private: [] }
}

const fn SND_SOC_DAPM_HP(name: *const c_char, _event: *const c_void) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget {
        id: 0,
        name,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        event: None,
        event_flags: 0,
    }
}

const fn SND_SOC_DAPM_MIC(name: *const c_char, _event: *const c_void) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget {
        id: 0,
        name,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        event: None,
        event_flags: 0,
    }
}

const fn SND_SOC_DAPM_SPK(name: *const c_char, _event: *const c_void) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget {
        id: 0,
        name,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        event: None,
        event_flags: 0,
    }
}

const fn SND_SOC_DAPM_SUPPLY(
    name: *const c_char,
    reg: c_int,
    shift: c_int,
    invert: c_int,
    event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
    event_flags: c_int,
) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget {
        id: 0,
        name,
        reg,
        shift,
        invert,
        event,
        event_flags,
    }
}

static mut vg_headset: snd_soc_jack = snd_soc_jack { jack: null_mut() };

static platform: [snd_soc_dai_link_component; 1] = [COMP_PLATFORM(b"acp5x_i2s_dma.0\0".as_ptr() as *const c_char)];
static acp5x_i2s: [snd_soc_dai_link_component; 1] = [COMP_CPU(b"acp5x_i2s_playcap.0\0".as_ptr() as *const c_char)];
static acp5x_bt: [snd_soc_dai_link_component; 1] = [COMP_CPU(b"acp5x_i2s_playcap.1\0".as_ptr() as *const c_char)];
static nau8821: [snd_soc_dai_link_component; 1] = [COMP_CODEC(ACP5X_NAU8821_COMP_NAME, ACP5X_NAU8821_DAI_NAME)];

static mut acp5x_nau8821_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

static acp5x_8821_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_PIN_SWITCH(b"Headphone\0".as_ptr() as *const c_char),
    SOC_DAPM_PIN_SWITCH(b"Headset Mic\0".as_ptr() as *const c_char),
    SOC_DAPM_PIN_SWITCH(b"Int Mic\0".as_ptr() as *const c_char),
];

unsafe extern "C" fn platform_clock_control(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let dai: *mut snd_soc_dai;
    let mut ret: c_int = 0;

    dai = snd_soc_card_get_codec_dai(card, ACP5X_NAU8821_DAI_NAME);
    if dai.is_null() {
        dev_err((*card).dev, b"Codec dai not found\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    if SND_SOC_DAPM_EVENT_OFF(event) {
        ret = snd_soc_dai_set_sysclk(dai, NAU8821_CLK_INTERNAL, 0, SND_SOC_CLOCK_IN);
        if ret < 0 {
            dev_err((*card).dev, b"set sysclk err = %d\n\0".as_ptr() as *const c_char, ret);
            return -EIO;
        }
    } else {
        ret = snd_soc_dai_set_sysclk(dai, NAU8821_CLK_FLL_BLK, 0, SND_SOC_CLOCK_IN);
        if ret < 0 {
            dev_err((*dai).dev, b"can't set BLK clock %d\n\0".as_ptr() as *const c_char, ret);
        }
        ret = snd_soc_dai_set_pll(dai, 0, 0, ACP5X_NAU8821_BCLK, ACP5X_NAU8821_FREQ_OUT);
        if ret < 0 {
            dev_err((*dai).dev, b"can't set FLL: %d\n\0".as_ptr() as *const c_char, ret);
        }
    }

    ret
}

unsafe extern "C" fn acp5x_8821_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let ret: c_int;

    /*
     * Headset buttons map to the google Reference headset.
     * These can be configured by userspace.
     */
    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_BTN_0,
        &mut vg_headset,
        acp5x_nau8821_jack_pins.as_mut_ptr(),
        acp5x_nau8821_jack_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*rtd).dev, b"Headset Jack creation failed %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    snd_jack_set_key(vg_headset.jack, SND_JACK_BTN_0, KEY_MEDIA);
    nau8821_enable_jack_detect(component, &mut vg_headset);

    ret
}

static rates: [c_uint; 1] = [48000];

static constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates.len() as c_uint,
    list: rates.as_ptr(),
    mask: 0,
};

static channels: [c_uint; 1] = [2];

static constraints_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: channels.len() as c_uint,
    list: channels.as_ptr(),
    mask: 0,
};

static acp5x_nau8821_format: [c_uint; 1] = [32];

static mut constraints_sample_bits: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: acp5x_nau8821_format.as_ptr(),
    count: acp5x_nau8821_format.len() as c_uint,
    mask: 0,
};

unsafe extern "C" fn acp5x_8821_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let machine = snd_soc_card_get_drvdata((*rtd).card) as *mut acp5x_platform_info;

    (*machine).play_i2s_instance = I2S_SP_INSTANCE;
    (*machine).cap_i2s_instance = I2S_SP_INSTANCE;

    (*runtime).hw.channels_max = DUAL_CHANNEL;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_SAMPLE_BITS, &constraints_sample_bits);

    0
}

unsafe extern "C" fn acp5x_nau8821_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let dai = snd_soc_card_get_codec_dai(card, ACP5X_NAU8821_DAI_NAME);
    let mut ret: c_int;
    let bclk: c_int;

    if dai.is_null() {
        return -EINVAL;
    }

    ret = snd_soc_dai_set_sysclk(dai, NAU8821_CLK_FLL_BLK, 0, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*card).dev, b"can't set FS clock %d\n\0".as_ptr() as *const c_char, ret);
    }

    bclk = snd_soc_params_to_bclk(params);
    if bclk < 0 {
        dev_err((*dai).dev, b"Fail to get BCLK rate: %d\n\0".as_ptr() as *const c_char, bclk);
        return bclk;
    }

    ret = snd_soc_dai_set_pll(dai, 0, 0, bclk as c_uint, params_rate(params) * 256);
    if ret < 0 {
        dev_err((*card).dev, b"can't set FLL: %d\n\0".as_ptr() as *const c_char, ret);
    }

    ret
}

static acp5x_8821_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp5x_8821_startup),
    hw_params: Some(acp5x_nau8821_hw_params),
};

unsafe extern "C" fn acp5x_cs35l41_startup(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let machine = snd_soc_card_get_drvdata((*rtd).card) as *mut acp5x_platform_info;
    let runtime = (*substream).runtime;

    (*machine).play_i2s_instance = I2S_HS_INSTANCE;

    (*runtime).hw.channels_max = DUAL_CHANNEL;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);

    0
}

unsafe extern "C" fn acp5x_cs35l41_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let rate: c_uint = params_rate(params);
    let bclk: c_uint;
    let mut comp: *mut snd_soc_component;
    let mut ret: c_int;
    let mut i: c_int;

    match rate {
        48000 => {
            bclk = 1536000;
        }
        _ => {
            bclk = 0;
        }
    }

    i = 0;
    while for_each_rtd_components(rtd, &mut i, &mut comp) {
        if strcmp((*comp).name, ACP5X_CS35L41_COMP_LNAME) == 0
            || strcmp((*comp).name, ACP5X_CS35L41_COMP_RNAME) == 0
        {
            if bclk == 0 {
                dev_err((*comp).dev, b"Invalid sample rate: 0x%x\n\0".as_ptr() as *const c_char, rate);
                return -EINVAL;
            }

            ret = snd_soc_component_set_sysclk(comp, 0, 0, bclk, SND_SOC_CLOCK_IN);
            if ret != 0 {
                dev_err((*comp).dev, b"failed to set SYSCLK: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
        }
    }

    0
}

extern "C" {
    fn for_each_rtd_components(rtd: *mut snd_soc_pcm_runtime, i: *mut c_int, comp: *mut *mut snd_soc_component) -> bool;
}

static acp5x_cs35l41_play_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp5x_cs35l41_startup),
    hw_params: Some(acp5x_cs35l41_hw_params),
};

static mut acp5x_cs35l41_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf {
        dlc: COMP_CODEC_CONF(ACP5X_CS35L41_COMP_LNAME),
        name_prefix: b"Left\0".as_ptr() as *const c_char,
    },
    snd_soc_codec_conf {
        dlc: COMP_CODEC_CONF(ACP5X_CS35L41_COMP_RNAME),
        name_prefix: b"Right\0".as_ptr() as *const c_char,
    },
];

static cs35l41: [snd_soc_dai_link_component; 2] = [
    COMP_CODEC(ACP5X_CS35L41_COMP_LNAME, ACP5X_CS35L41_DAI_NAME),
    COMP_CODEC(ACP5X_CS35L41_COMP_RNAME, ACP5X_CS35L41_DAI_NAME),
];

static mut acp5x_8821_35l41_dai: [snd_soc_dai_link; 2] = [
    snd_soc_dai_link {
        name: b"acp5x-8821-play\0".as_ptr() as *const c_char,
        stream_name: b"Playback/Capture\0".as_ptr() as *const c_char,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        playback_only: 0,
        ops: &acp5x_8821_ops,
        init: Some(acp5x_8821_init),
        cpus: acp5x_i2s.as_ptr(),
        num_cpus: acp5x_i2s.len() as c_uint,
        codecs: nau8821.as_ptr(),
        num_codecs: nau8821.len() as c_uint,
        platforms: platform.as_ptr(),
        num_platforms: platform.len() as c_uint,
    },
    snd_soc_dai_link {
        name: b"acp5x-CS35L41-Stereo\0".as_ptr() as *const c_char,
        stream_name: b"CS35L41 Stereo Playback\0".as_ptr() as *const c_char,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        playback_only: 1,
        ops: &acp5x_cs35l41_play_ops,
        init: None,
        cpus: acp5x_bt.as_ptr(),
        num_cpus: acp5x_bt.len() as c_uint,
        codecs: cs35l41.as_ptr(),
        num_codecs: cs35l41.len() as c_uint,
        platforms: platform.as_ptr(),
        num_platforms: platform.len() as c_uint,
    },
];

static acp5x_8821_35l41_widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_HP(b"Headphone\0".as_ptr() as *const c_char, null()),
    SND_SOC_DAPM_MIC(b"Headset Mic\0".as_ptr() as *const c_char, null()),
    SND_SOC_DAPM_MIC(b"Int Mic\0".as_ptr() as *const c_char, null()),
    SND_SOC_DAPM_SUPPLY(
        b"Platform Clock\0".as_ptr() as *const c_char,
        SND_SOC_NOPM,
        0,
        0,
        Some(platform_clock_control),
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
    ),
];

static acp5x_8821_35l41_audio_route: [snd_soc_dapm_route; 8] = [
    /* HP jack connectors - unknown if we have jack detection */
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: null(), source: b"HPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: null(), source: b"HPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MICL\0".as_ptr() as *const c_char, control: null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MICR\0".as_ptr() as *const c_char, control: null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC\0".as_ptr() as *const c_char, control: null(), source: b"Int Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headset Mic\0".as_ptr() as *const c_char, control: null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Int Mic\0".as_ptr() as *const c_char, control: null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
];

static mut acp5x_8821_35l41_card: snd_soc_card = unsafe {
    snd_soc_card {
        name: b"acp5x\0".as_ptr() as *const c_char,
        owner: THIS_MODULE,
        dai_link: acp5x_8821_35l41_dai.as_mut_ptr(),
        num_links: acp5x_8821_35l41_dai.len() as c_int,
        dapm_widgets: acp5x_8821_35l41_widgets.as_ptr(),
        num_dapm_widgets: acp5x_8821_35l41_widgets.len() as c_int,
        dapm_routes: acp5x_8821_35l41_audio_route.as_ptr(),
        num_dapm_routes: acp5x_8821_35l41_audio_route.len() as c_int,
        codec_conf: acp5x_cs35l41_conf.as_mut_ptr(),
        num_configs: acp5x_cs35l41_conf.len() as c_int,
        controls: acp5x_8821_controls.as_ptr(),
        num_controls: acp5x_8821_controls.len() as c_int,
        dev: null_mut(),
    }
};

unsafe extern "C" fn acp5x_max98388_startup(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let machine = snd_soc_card_get_drvdata((*rtd).card) as *mut acp5x_platform_info;
    let runtime = (*substream).runtime;

    (*machine).play_i2s_instance = I2S_HS_INSTANCE;

    (*runtime).hw.channels_max = DUAL_CHANNEL;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    0
}

static acp5x_max98388_play_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp5x_max98388_startup),
    hw_params: None,
};

static mut acp5x_max98388_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf {
        dlc: COMP_CODEC_CONF(ACP5X_MAX98388_COMP_LNAME),
        name_prefix: b"Left\0".as_ptr() as *const c_char,
    },
    snd_soc_codec_conf {
        dlc: COMP_CODEC_CONF(ACP5X_MAX98388_COMP_RNAME),
        name_prefix: b"Right\0".as_ptr() as *const c_char,
    },
];

static max98388: [snd_soc_dai_link_component; 2] = [
    COMP_CODEC(ACP5X_MAX98388_COMP_LNAME, ACP5X_MAX98388_DAI_NAME),
    COMP_CODEC(ACP5X_MAX98388_COMP_RNAME, ACP5X_MAX98388_DAI_NAME),
];

static mut acp5x_8821_98388_dai: [snd_soc_dai_link; 2] = [
    snd_soc_dai_link {
        name: b"acp5x-8821-play\0".as_ptr() as *const c_char,
        stream_name: b"Playback/Capture\0".as_ptr() as *const c_char,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        playback_only: 0,
        ops: &acp5x_8821_ops,
        init: Some(acp5x_8821_init),
        cpus: acp5x_i2s.as_ptr(),
        num_cpus: acp5x_i2s.len() as c_uint,
        codecs: nau8821.as_ptr(),
        num_codecs: nau8821.len() as c_uint,
        platforms: platform.as_ptr(),
        num_platforms: platform.len() as c_uint,
    },
    snd_soc_dai_link {
        name: b"acp5x-max98388-play\0".as_ptr() as *const c_char,
        stream_name: b"MAX98388 Playback\0".as_ptr() as *const c_char,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        playback_only: 1,
        ops: &acp5x_max98388_play_ops,
        init: None,
        cpus: acp5x_bt.as_ptr(),
        num_cpus: acp5x_bt.len() as c_uint,
        codecs: max98388.as_ptr(),
        num_codecs: max98388.len() as c_uint,
        platforms: platform.as_ptr(),
        num_platforms: platform.len() as c_uint,
    },
];

static acp5x_8821_98388_widgets: [snd_soc_dapm_widget; 5] = [
    SND_SOC_DAPM_HP(b"Headphone\0".as_ptr() as *const c_char, null()),
    SND_SOC_DAPM_MIC(b"Headset Mic\0".as_ptr() as *const c_char, null()),
    SND_SOC_DAPM_MIC(b"Int Mic\0".as_ptr() as *const c_char, null()),
    SND_SOC_DAPM_SUPPLY(
        b"Platform Clock\0".as_ptr() as *const c_char,
        SND_SOC_NOPM,
        0,
        0,
        Some(platform_clock_control),
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
    ),
    SND_SOC_DAPM_SPK(b"SPK\0".as_ptr() as *const c_char, null()),
];

static acp5x_8821_98388_route: [snd_soc_dapm_route; 10] = [
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: null(), source: b"HPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: null(), source: b"HPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MICL\0".as_ptr() as *const c_char, control: null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MICR\0".as_ptr() as *const c_char, control: null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC\0".as_ptr() as *const c_char, control: null(), source: b"Int Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone\0".as_ptr() as *const c_char, control: null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headset Mic\0".as_ptr() as *const c_char, control: null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Int Mic\0".as_ptr() as *const c_char, control: null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPK\0".as_ptr() as *const c_char, control: null(), source: b"Left BE_OUT\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPK\0".as_ptr() as *const c_char, control: null(), source: b"Right BE_OUT\0".as_ptr() as *const c_char },
];

static mut acp5x_8821_98388_card: snd_soc_card = unsafe {
    snd_soc_card {
        name: b"acp5x-max98388\0".as_ptr() as *const c_char,
        owner: THIS_MODULE,
        dai_link: acp5x_8821_98388_dai.as_mut_ptr(),
        num_links: acp5x_8821_98388_dai.len() as c_int,
        dapm_widgets: acp5x_8821_98388_widgets.as_ptr(),
        num_dapm_widgets: acp5x_8821_98388_widgets.len() as c_int,
        dapm_routes: acp5x_8821_98388_route.as_ptr(),
        num_dapm_routes: acp5x_8821_98388_route.len() as c_int,
        codec_conf: acp5x_max98388_conf.as_mut_ptr(),
        num_configs: acp5x_max98388_conf.len() as c_int,
        controls: acp5x_8821_controls.as_ptr(),
        num_controls: acp5x_8821_controls.len() as c_int,
        dev: null_mut(),
    }
};

static acp5x_vg_quirk_table: [dmi_system_id; 3] = [
    dmi_system_id {
        matches: [
            DMI_EXACT_MATCH(DMI_BOARD_VENDOR, b"Valve\0".as_ptr() as *const c_char),
            DMI_EXACT_MATCH(DMI_PRODUCT_NAME, b"Jupiter\0".as_ptr() as *const c_char),
            DMI_EXACT_MATCH(0, null()),
            DMI_EXACT_MATCH(0, null()),
        ],
        driver_data: unsafe { &mut acp5x_8821_35l41_card as *mut snd_soc_card as *mut c_void },
    },
    dmi_system_id {
        matches: [
            DMI_EXACT_MATCH(DMI_BOARD_VENDOR, b"Valve\0".as_ptr() as *const c_char),
            DMI_EXACT_MATCH(DMI_PRODUCT_NAME, b"Galileo\0".as_ptr() as *const c_char),
            DMI_EXACT_MATCH(0, null()),
            DMI_EXACT_MATCH(0, null()),
        ],
        driver_data: unsafe { &mut acp5x_8821_98388_card as *mut snd_soc_card as *mut c_void },
    },
    dmi_system_id {
        matches: [
            DMI_EXACT_MATCH(0, null()),
            DMI_EXACT_MATCH(0, null()),
            DMI_EXACT_MATCH(0, null()),
            DMI_EXACT_MATCH(0, null()),
        ],
        driver_data: null_mut(),
    },
];

unsafe extern "C" fn acp5x_probe(pdev: *mut platform_device) -> c_int {
    let dmi_id: *const dmi_system_id;
    let machine: *mut acp5x_platform_info;
    let dev = &mut (*pdev).dev as *mut device;
    let card: *mut snd_soc_card;
    let ret: c_int;

    dmi_id = dmi_first_match(acp5x_vg_quirk_table.as_ptr());
    if dmi_id.is_null() || (*dmi_id).driver_data.is_null() {
        return -ENODEV;
    }

    machine = devm_kzalloc(dev, size_of::<acp5x_platform_info>(), GFP_KERNEL) as *mut acp5x_platform_info;
    if machine.is_null() {
        return -ENOMEM;
    }

    card = (*dmi_id).driver_data as *mut snd_soc_card;
    (*card).dev = dev;
    platform_set_drvdata(pdev, card as *mut c_void);
    snd_soc_card_set_drvdata(card, machine as *mut c_void);

    ret = devm_snd_soc_register_card(dev, card);
    if ret != 0 {
        return dev_err_probe(dev, ret, b"Register card (%s) failed\n\0".as_ptr() as *const c_char, (*card).name);
    }

    0
}

static mut acp5x_mach_driver: platform_driver = unsafe {
    platform_driver {
        driver: platform_driver_inner {
            name: DRV_NAME,
            pm: &snd_soc_pm_ops as *const c_void,
        },
        probe: Some(acp5x_probe),
    }
};

// module_platform_driver(acp5x_mach_driver);
// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");
// MODULE_DESCRIPTION("NAU8821/CS35L41 & NAU8821/MAX98388 audio support");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
