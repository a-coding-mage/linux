// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

// C dependencies:
// linux/input.h, linux/module.h, linux/platform_device.h
// sound/core.h, sound/jack.h, sound/pcm.h, sound/pcm_params.h
// sound/soc.h, sound/soc-acpi.h, ../../../codecs/nau8825.h, ../utils.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const SKL_NUVOTON_CODEC_DAI: &[u8] = b"nau8825-hifi\0";

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_NOPM: c_int = 0;
const SND_SOC_DAPM_PRE_PMU: c_int = 0;
const SND_SOC_DAPM_POST_PMD: c_int = 0;
const SND_JACK_HEADPHONE: c_uint = 0;
const SND_JACK_MICROPHONE: c_uint = 0;
const SND_JACK_HEADSET: c_uint = 0;
const SND_JACK_BTN_0: c_uint = 0;
const SND_JACK_BTN_1: c_uint = 0;
const SND_JACK_BTN_2: c_uint = 0;
const SND_JACK_BTN_3: c_uint = 0;
const KEY_PLAYPAUSE: c_uint = 0;
const KEY_VOICECOMMAND: c_uint = 0;
const KEY_VOLUMEUP: c_uint = 0;
const KEY_VOLUMEDOWN: c_uint = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const NAU8825_CLK_MCLK: c_int = 0;
const NAU8825_CLK_INTERNAL: c_int = 0;
const NAU8825_CLK_FLL_FS: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
    active: bool,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    dev: *mut device,
    component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
pub struct snd_interval {
    min: c_uint,
    max: c_uint,
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
pub struct snd_pcm_runtime {
    rate: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    name: *const c_char,
    cpus: *mut snd_soc_dai_link_component,
    codecs: *mut snd_soc_dai_link_component,
    num_cpus: c_uint,
    num_codecs: c_uint,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_uint,
    id: c_int,
    dai_fmt: c_uint,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    ops: *const snd_soc_ops,
    nonatomic: c_uint,
    no_pcm: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    name: *const c_char,
    driver_name: *const c_char,
    long_name: *const c_char,
    dev: *mut device,
    owner: *mut c_void,
    suspend_pre: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    resume_post: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_uint,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget_desc,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    fully_routed: bool,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pdata: *mut avs_mach_pdata,
}

#[repr(C)]
pub struct avs_mach_pdata {
    obsolete_card_names: bool,
}

#[repr(C)]
pub struct platform_device_id {
    name: [c_char; 20],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver_inner {
    name: *const c_char,
    pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: platform_driver_inner,
    id_table: *const platform_device_id,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_codec_dai(card: *mut snd_soc_card, dai: *const c_char) -> *mut snd_soc_dai;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn devm_kmemdup_array(
        dev: *mut device,
        src: *const c_void,
        n: usize,
        size: usize,
        flags: c_uint,
    ) -> *mut c_void;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_int,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_uint, keytype: c_uint);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn snd_soc_dai_stream_active(dai: *mut snd_soc_dai, stream: c_int) -> bool;
    fn snd_soc_dai_get_widget(dai: *mut snd_soc_dai, stream: c_int) -> *mut snd_soc_dapm_widget;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn avs_mach_get_ssp_tdm(
        dev: *mut device,
        mach: *mut snd_soc_acpi_mach,
        ssp_port: *mut c_int,
        tdm_slot: *mut c_int,
    ) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_deferrable_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

unsafe extern "C" fn avs_nau8825_clock_control(
    w: *mut snd_soc_dapm_widget,
    _control: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let codec_dai: *mut snd_soc_dai;
    let ret: c_int;

    codec_dai = snd_soc_card_get_codec_dai(card, SKL_NUVOTON_CODEC_DAI.as_ptr() as *const c_char);
    if codec_dai.is_null() {
        dev_err((*card).dev, c"Codec dai not found\n".as_ptr());
        return -EINVAL;
    }

    if SND_SOC_DAPM_EVENT_ON(event) {
        ret = snd_soc_dai_set_sysclk(codec_dai, NAU8825_CLK_MCLK, 24000000, SND_SOC_CLOCK_IN);
    } else {
        ret = snd_soc_dai_set_sysclk(codec_dai, NAU8825_CLK_INTERNAL, 0, SND_SOC_CLOCK_IN);
    }
    if ret < 0 {
        dev_err((*card).dev, c"Set sysclk failed: %d\n".as_ptr(), ret);
    }

    ret
}

// Original C uses SOC_DAPM_PIN_SWITCH("Headphone Jack") and
// SOC_DAPM_PIN_SWITCH("Headset Mic").
static card_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

// Original C uses SND_SOC_DAPM_HP, SND_SOC_DAPM_MIC, and SND_SOC_DAPM_SUPPLY
// with avs_nau8825_clock_control and PRE_PMU | POST_PMD events.
static card_widgets: [snd_soc_dapm_widget_desc; 3] = [
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
];

static card_base_routes: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route {
        sink: c"Headphone Jack".as_ptr(),
        control: ptr::null(),
        source: c"HPOL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone Jack".as_ptr(),
        control: ptr::null(),
        source: c"HPOR".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"MIC".as_ptr(),
        control: ptr::null(),
        source: c"Headset Mic".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone Jack".as_ptr(),
        control: ptr::null(),
        source: c"Platform Clock".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headset Mic".as_ptr(),
        control: ptr::null(),
        source: c"Platform Clock".as_ptr(),
    },
];

static card_headset_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone Jack".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn avs_nau8825_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*runtime).card;
    let pins: *mut snd_soc_jack_pin;
    let jack: *mut snd_soc_jack;
    let num_pins: c_int;
    let mut ret: c_int;

    jack = snd_soc_card_get_drvdata(card) as *mut snd_soc_jack;
    num_pins = card_headset_pins.len() as c_int;

    pins = devm_kmemdup_array(
        (*card).dev,
        card_headset_pins.as_ptr() as *const c_void,
        num_pins as usize,
        core::mem::size_of::<snd_soc_jack_pin>(),
        GFP_KERNEL,
    ) as *mut snd_soc_jack_pin;
    if pins.is_null() {
        return -ENOMEM;
    }

    /*
     * 4 buttons here map to the google Reference headset.
     * The use of these buttons can be decided by the user space.
     */
    ret = snd_soc_card_jack_new_pins(
        card,
        c"Headset Jack".as_ptr(),
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        jack,
        pins,
        num_pins,
    );
    if ret != 0 {
        return ret;
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

    snd_soc_component_set_jack((*snd_soc_rtd_to_codec(runtime, 0)).component, jack, ptr::null_mut())
}

unsafe extern "C" fn avs_nau8825_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
    snd_soc_component_set_jack(
        (*snd_soc_rtd_to_codec(rtd, 0)).component,
        ptr::null_mut(),
        ptr::null_mut(),
    );
}

unsafe extern "C" fn avs_nau8825_be_fixup(
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
    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;

    /* set SSP to 24 bit */
    snd_mask_none(fmt);
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE);

    0
}

unsafe extern "C" fn avs_nau8825_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let runtime = (*substream).runtime;
    let rtm = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtm, 0);
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            ret = snd_soc_dai_set_sysclk(codec_dai, NAU8825_CLK_FLL_FS, 0, SND_SOC_CLOCK_IN);
            if ret < 0 {
                dev_err((*codec_dai).dev, c"can't set FS clock %d\n".as_ptr(), ret);
            } else {
                ret = snd_soc_dai_set_pll(
                    codec_dai,
                    0,
                    0,
                    (*runtime).rate,
                    (*runtime).rate.wrapping_mul(256),
                );
                if ret < 0 {
                    dev_err((*codec_dai).dev, c"can't set FLL: %d\n".as_ptr(), ret);
                }
            }
        }
        SNDRV_PCM_TRIGGER_RESUME => {
            ret = snd_soc_dai_set_pll(
                codec_dai,
                0,
                0,
                (*runtime).rate,
                (*runtime).rate.wrapping_mul(256),
            );
            if ret < 0 {
                dev_err((*codec_dai).dev, c"can't set FLL: %d\n".as_ptr(), ret);
            }
        }
        _ => {}
    }

    ret
}

static avs_nau8825_ops: snd_soc_ops = snd_soc_ops {
    trigger: Some(avs_nau8825_trigger),
};

unsafe extern "C" fn avs_create_dai_link(
    dev: *mut device,
    ssp_port: c_int,
    tdm_slot: c_int,
    dai_link: *mut *mut snd_soc_dai_link,
) -> c_int {
    let platform: *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link;

    dl = devm_kzalloc(dev, core::mem::size_of::<snd_soc_dai_link>(), GFP_KERNEL)
        as *mut snd_soc_dai_link;
    platform = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if dl.is_null() || platform.is_null() {
        return -ENOMEM;
    }

    (*dl).name = devm_kasprintf(dev, GFP_KERNEL, c"SSP%d-Codec%d".as_ptr(), ssp_port, tdm_slot);
    (*dl).cpus = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    (*dl).codecs = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if (*dl).name.is_null() || (*dl).cpus.is_null() || (*dl).codecs.is_null() {
        return -ENOMEM;
    }

    (*(*dl).cpus).dai_name =
        devm_kasprintf(dev, GFP_KERNEL, c"SSP%d Pin%d".as_ptr(), ssp_port, tdm_slot);
    (*(*dl).codecs).name = devm_kasprintf(dev, GFP_KERNEL, c"i2c-10508825:00".as_ptr());
    (*(*dl).codecs).dai_name =
        devm_kasprintf(dev, GFP_KERNEL, SKL_NUVOTON_CODEC_DAI.as_ptr() as *const c_char);
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
    (*dl).dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    (*dl).init = Some(avs_nau8825_codec_init);
    (*dl).exit = Some(avs_nau8825_codec_exit);
    (*dl).be_hw_params_fixup = Some(avs_nau8825_be_fixup);
    (*dl).ops = &avs_nau8825_ops;
    (*dl).nonatomic = 1;
    (*dl).no_pcm = 1;

    *dai_link = dl;

    0
}

unsafe extern "C" fn avs_card_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let codec_dai = snd_soc_card_get_codec_dai(card, SKL_NUVOTON_CODEC_DAI.as_ptr() as *const c_char);

    snd_soc_component_set_jack((*codec_dai).component, ptr::null_mut(), ptr::null_mut())
}

unsafe extern "C" fn avs_card_resume_post(card: *mut snd_soc_card) -> c_int {
    let codec_dai = snd_soc_card_get_codec_dai(card, SKL_NUVOTON_CODEC_DAI.as_ptr() as *const c_char);
    let jack = snd_soc_card_get_drvdata(card) as *mut snd_soc_jack;
    let stream = SNDRV_PCM_STREAM_PLAYBACK;

    if codec_dai.is_null() {
        dev_err((*card).dev, c"Codec dai not found\n".as_ptr());
        return -EINVAL;
    }

    if snd_soc_dai_stream_active(codec_dai, stream) && (*snd_soc_dai_get_widget(codec_dai, stream)).active {
        snd_soc_dai_set_sysclk(codec_dai, NAU8825_CLK_FLL_FS, 0, SND_SOC_CLOCK_IN);
    }

    snd_soc_component_set_jack((*codec_dai).component, jack, ptr::null_mut())
}

unsafe extern "C" fn avs_nau8825_probe(pdev: *mut platform_device) -> c_int {
    let mut dai_link: *mut snd_soc_dai_link = ptr::null_mut();
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let card: *mut snd_soc_card;
    let jack: *mut snd_soc_jack;
    let dev = &mut (*pdev).dev as *mut device;
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
        dev_err(dev, c"Failed to create dai link: %d".as_ptr(), ret);
        return ret;
    }

    jack = devm_kzalloc(dev, core::mem::size_of::<snd_soc_jack>(), GFP_KERNEL) as *mut snd_soc_jack;
    card = devm_kzalloc(dev, core::mem::size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if jack.is_null() || card.is_null() {
        return -ENOMEM;
    }

    if (*pdata).obsolete_card_names {
        (*card).name = c"avs_nau8825".as_ptr();
    } else {
        (*card).driver_name = c"avs_nau8825".as_ptr();
        (*card).name = c"AVS I2S NAU8825".as_ptr();
        (*card).long_name = (*card).name;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).suspend_pre = Some(avs_card_suspend_pre);
    (*card).resume_post = Some(avs_card_resume_post);
    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).controls = card_controls.as_ptr();
    (*card).num_controls = card_controls.len() as c_uint;
    (*card).dapm_widgets = card_widgets.as_ptr();
    (*card).num_dapm_widgets = card_widgets.len() as c_uint;
    (*card).dapm_routes = card_base_routes.as_ptr();
    (*card).num_dapm_routes = card_base_routes.len() as c_uint;
    (*card).fully_routed = true;
    snd_soc_card_set_drvdata(card, jack as *mut c_void);

    devm_snd_soc_register_deferrable_card(dev, card)
}

static avs_nau8825_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char,
            b'v' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b'n' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'8' as c_char,
            b'8' as c_char,
            b'2' as c_char,
            b'5' as c_char,
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

// MODULE_DEVICE_TABLE(platform, avs_nau8825_driver_ids);

static mut avs_nau8825_driver: platform_driver = platform_driver {
    probe: Some(avs_nau8825_probe),
    driver: platform_driver_inner {
        name: c"avs_nau8825".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    id_table: avs_nau8825_driver_ids.as_ptr(),
};

// module_platform_driver(avs_nau8825_driver)
// MODULE_DESCRIPTION("Intel nau8825 machine driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
