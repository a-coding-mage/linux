// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

// C includes translated as external dependencies:
// <linux/module.h>
// <sound/jack.h>
// <sound/pcm.h>
// <sound/pcm_params.h>
// <sound/soc.h>
// <sound/soc-acpi.h>
// "../../../codecs/rt274.h"
// "../utils.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const AVS_RT274_FREQ_OUT: c_int = 24000000;
const AVS_RT274_BE_FIXUP_RATE: c_int = 48000;
const RT274_CODEC_DAI: *const c_char = b"rt274-aif1\0".as_ptr() as *const c_char;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const RT274_SCLK_S_PLL2: c_int = 0;
const RT274_PLL2_S_BCLK: c_int = 0;
const SND_SOC_NOPM: c_int = 0;
const SND_SOC_DAPM_PRE_PMU: c_int = 0;
const SND_SOC_DAPM_POST_PMD: c_int = 0;
const SND_JACK_HEADPHONE: c_int = 0;
const SND_JACK_MICROPHONE: c_int = 0;
const SND_JACK_HEADSET: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

type c_uint = u32;
type bool_c = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
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
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_cpus: c_int,
    pub num_codecs: c_int,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_int,
    pub id: c_int,
    pub dai_fmt: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    pub be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub nonatomic: c_uint,
    pub no_pcm: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub driver_name: *const c_char,
    pub long_name: *const c_char,
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub suspend_pre: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub resume_post: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub fully_routed: bool_c,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub pdata: *mut avs_mach_pdata,
}

#[repr(C)]
pub struct avs_mach_pdata {
    pub obsolete_card_names: bool_c,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: [c_char; 20],
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
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
    pub id_table: *const platform_device_id,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_codec_dai(card: *mut snd_soc_card, dai_name: *const c_char)
        -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_int,
        dir: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_dai_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_int) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_int,
        freq_out: c_int,
    ) -> c_int;
    fn snd_soc_rtd_to_codec(runtime: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn devm_kmemdup_array(
        dev: *mut device,
        src: *const c_void,
        n: usize,
        size: usize,
        flags: c_int,
    ) -> *mut c_void;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_int,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_soc_dapm_set_idle_bias(dapm: *mut snd_soc_dapm_context, idle_bias: bool_c);
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn avs_mach_get_ssp_tdm(
        dev: *mut device,
        mach: *mut snd_soc_acpi_mach,
        ssp_port: *mut c_int,
        tdm_slot: *mut c_int,
    ) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_deferrable_card(
        dev: *mut device,
        card: *mut snd_soc_card,
    ) -> c_int;
}

macro_rules! SOC_DAPM_PIN_SWITCH {
    ($name:expr) => {
        snd_kcontrol_new { _private: [] }
    };
}

macro_rules! SND_SOC_DAPM_HP {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget { dapm: ptr::null_mut() }
    };
}

macro_rules! SND_SOC_DAPM_MIC {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget { dapm: ptr::null_mut() }
    };
}

macro_rules! SND_SOC_DAPM_SUPPLY {
    ($name:expr, $reg:expr, $shift:expr, $invert:expr, $event:expr, $flags:expr) => {
        snd_soc_dapm_widget { dapm: ptr::null_mut() }
    };
}

fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool {
    event != 0
}

static card_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_PIN_SWITCH!(b"Headphone Jack\0".as_ptr() as *const c_char),
    SOC_DAPM_PIN_SWITCH!(b"Mic Jack\0".as_ptr() as *const c_char),
];

unsafe extern "C" fn avs_rt274_clock_control(
    w: *mut snd_soc_dapm_widget,
    _control: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card: *mut snd_soc_card = snd_soc_dapm_to_card((*w).dapm);
    let codec_dai: *mut snd_soc_dai;
    let mut ret: c_int;

    codec_dai = snd_soc_card_get_codec_dai(card, RT274_CODEC_DAI);
    if codec_dai.is_null() {
        return -EINVAL;
    }

    /* Codec needs clock for Jack detection and button press */
    ret = snd_soc_dai_set_sysclk(
        codec_dai,
        RT274_SCLK_S_PLL2,
        AVS_RT274_FREQ_OUT,
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        dev_err(
            (*codec_dai).dev,
            b"set codec sysclk failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    if SND_SOC_DAPM_EVENT_ON(event) {
        let ratio: c_int = 100;

        snd_soc_dai_set_bclk_ratio(codec_dai, ratio);

        ret = snd_soc_dai_set_pll(
            codec_dai,
            0,
            RT274_PLL2_S_BCLK,
            AVS_RT274_BE_FIXUP_RATE * ratio,
            AVS_RT274_FREQ_OUT,
        );
        if ret != 0 {
            dev_err(
                (*codec_dai).dev,
                b"failed to enable PLL2: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
    }

    0
}

static card_widgets: [snd_soc_dapm_widget; 3] = [
    SND_SOC_DAPM_HP!(b"Headphone Jack\0".as_ptr() as *const c_char, ptr::null()),
    SND_SOC_DAPM_MIC!(b"Mic Jack\0".as_ptr() as *const c_char, ptr::null()),
    SND_SOC_DAPM_SUPPLY!(
        b"Platform Clock\0".as_ptr() as *const c_char,
        SND_SOC_NOPM,
        0,
        0,
        avs_rt274_clock_control,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
];

static card_base_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPO Pin\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"MIC\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Mic Jack\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Platform Clock\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"MIC\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Platform Clock\0".as_ptr() as *const c_char,
    },
];

static card_headset_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone Jack\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Mic Jack\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn avs_rt274_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(runtime, 0);
    let component: *mut snd_soc_component = (*codec_dai).component;
    let pins: *mut snd_soc_jack_pin;
    let jack: *mut snd_soc_jack;
    let card: *mut snd_soc_card = (*runtime).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let num_pins: c_int;
    let mut ret: c_int;

    jack = snd_soc_card_get_drvdata(card) as *mut snd_soc_jack;
    num_pins = card_headset_pins.len() as c_int;

    pins = devm_kmemdup_array(
        (*card).dev,
        card_headset_pins.as_ptr() as *const c_void,
        num_pins as usize,
        size_of::<snd_soc_jack_pin>(),
        GFP_KERNEL,
    ) as *mut snd_soc_jack_pin;
    if pins.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET,
        jack,
        pins,
        num_pins,
    );
    if ret != 0 {
        return ret;
    }

    snd_soc_component_set_jack(component, jack, ptr::null_mut());

    /* TDM 4 slots 24 bit, set Rx & Tx bitmask to 4 active slots */
    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0xF, 0xF, 4, 24);
    if ret < 0 {
        dev_err(
            (*card).dev,
            b"can't set codec pcm format %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    snd_soc_dapm_set_idle_bias(dapm, false);

    0
}

unsafe extern "C" fn avs_rt274_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
    snd_soc_component_set_jack(
        (*snd_soc_rtd_to_codec(rtd, 0)).component,
        ptr::null_mut(),
        ptr::null_mut(),
    );
}

unsafe extern "C" fn avs_rt274_be_fixup(
    _runtime: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate: *mut snd_interval;
    let channels: *mut snd_interval;
    let fmt: *mut snd_mask;

    rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);

    /* The ADSP will convert the FE rate to 48k, stereo */
    (*rate).max = AVS_RT274_BE_FIXUP_RATE as c_uint;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;

    /* set SSPN to 24 bit */
    snd_mask_none(fmt);
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE);

    0
}

unsafe fn avs_create_dai_link(
    dev: *mut device,
    ssp_port: c_int,
    tdm_slot: c_int,
    dai_link: *mut *mut snd_soc_dai_link,
) -> c_int {
    let platform: *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link;

    dl = devm_kzalloc(dev, size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;
    platform = devm_kzalloc(
        dev,
        size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if dl.is_null() || platform.is_null() {
        return -ENOMEM;
    }

    (*dl).name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        b"SSP%d-%d-Codec\0".as_ptr() as *const c_char,
        ssp_port,
        tdm_slot,
    );
    (*dl).cpus = devm_kzalloc(
        dev,
        size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    (*dl).codecs = devm_kzalloc(
        dev,
        size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if (*dl).name.is_null() || (*dl).cpus.is_null() || (*dl).codecs.is_null() {
        return -ENOMEM;
    }

    (*(*dl).cpus).dai_name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        b"SSP%d-%d Pin\0".as_ptr() as *const c_char,
        ssp_port,
        tdm_slot,
    );
    (*(*dl).codecs).name =
        devm_kasprintf(dev, GFP_KERNEL, b"i2c-INT34C2:00\0".as_ptr() as *const c_char);
    (*(*dl).codecs).dai_name = devm_kasprintf(dev, GFP_KERNEL, RT274_CODEC_DAI);
    if (*(*dl).cpus).dai_name.is_null()
        || (*(*dl).codecs).name.is_null()
        || (*(*dl).codecs).dai_name.is_null()
    {
        return -ENOMEM;
    }

    (*platform).name = dev_name(dev);
    (*dl).num_cpus = 1;
    (*dl).num_codecs = 1;
    (*dl).platforms = platform;
    (*dl).num_platforms = 1;
    (*dl).id = 0;
    (*dl).dai_fmt = SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    (*dl).init = Some(avs_rt274_codec_init);
    (*dl).exit = Some(avs_rt274_codec_exit);
    (*dl).be_hw_params_fixup = Some(avs_rt274_be_fixup);
    (*dl).nonatomic = 1;
    (*dl).no_pcm = 1;

    *dai_link = dl;

    0
}

unsafe extern "C" fn avs_card_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let codec_dai: *mut snd_soc_dai = snd_soc_card_get_codec_dai(card, RT274_CODEC_DAI);

    snd_soc_component_set_jack((*codec_dai).component, ptr::null_mut(), ptr::null_mut())
}

unsafe extern "C" fn avs_card_resume_post(card: *mut snd_soc_card) -> c_int {
    let codec_dai: *mut snd_soc_dai = snd_soc_card_get_codec_dai(card, RT274_CODEC_DAI);
    let jack: *mut snd_soc_jack = snd_soc_card_get_drvdata(card) as *mut snd_soc_jack;

    snd_soc_component_set_jack((*codec_dai).component, jack, ptr::null_mut())
}

unsafe extern "C" fn avs_rt274_probe(pdev: *mut platform_device) -> c_int {
    let mut dai_link: *mut snd_soc_dai_link = ptr::null_mut();
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let card: *mut snd_soc_card;
    let jack: *mut snd_soc_jack;
    let dev: *mut device = &mut (*pdev).dev;
    let mut ssp_port: c_int = 0;
    let mut tdm_slot: c_int = 0;
    let mut ret: c_int;

    mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    pdata = (*mach).pdata;

    ret = avs_mach_get_ssp_tdm(dev, mach, &mut ssp_port, &mut tdm_slot);
    if ret != 0 {
        return ret;
    }

    ret = avs_create_dai_link(dev, ssp_port, tdm_slot, &mut dai_link);
    if ret != 0 {
        dev_err(
            dev,
            b"Failed to create dai link: %d\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    jack = devm_kzalloc(dev, size_of::<snd_soc_jack>(), GFP_KERNEL) as *mut snd_soc_jack;
    card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if jack.is_null() || card.is_null() {
        return -ENOMEM;
    }

    if (*pdata).obsolete_card_names {
        (*card).name = b"avs_rt274\0".as_ptr() as *const c_char;
    } else {
        (*card).driver_name = b"avs_rt274\0".as_ptr() as *const c_char;
        (*card).name = b"AVS I2S ALC274\0".as_ptr() as *const c_char;
        (*card).long_name = (*card).name;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).suspend_pre = Some(avs_card_suspend_pre);
    (*card).resume_post = Some(avs_card_resume_post);
    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).controls = card_controls.as_ptr();
    (*card).num_controls = card_controls.len() as c_int;
    (*card).dapm_widgets = card_widgets.as_ptr();
    (*card).num_dapm_widgets = card_widgets.len() as c_int;
    (*card).dapm_routes = card_base_routes.as_ptr();
    (*card).num_dapm_routes = card_base_routes.len() as c_int;
    (*card).fully_routed = true;
    snd_soc_card_set_drvdata(card, jack as *mut c_void);

    devm_snd_soc_register_deferrable_card(dev, card)
}

static avs_rt274_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char,
            b'v' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b'r' as c_char,
            b't' as c_char,
            b'2' as c_char,
            b'7' as c_char,
            b'4' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    platform_device_id { name: [0; 20] },
];

// MODULE_DEVICE_TABLE(platform, avs_rt274_driver_ids);

static mut avs_rt274_driver: platform_driver = platform_driver {
    probe: Some(avs_rt274_probe),
    driver: device_driver {
        name: b"avs_rt274\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const dev_pm_ops },
    },
    id_table: avs_rt274_driver_ids.as_ptr(),
};

// module_platform_driver(avs_rt274_driver);
// MODULE_DESCRIPTION("Intel rt274 machine driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
