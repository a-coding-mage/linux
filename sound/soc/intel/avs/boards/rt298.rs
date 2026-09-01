// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

// C dependencies:
// linux/dmi.h, linux/module.h, sound/jack.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-acpi.h,
// ../../../codecs/rt298.h, ../utils.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const RT298_CODEC_DAI: *const c_char = b"rt298-aif1\0".as_ptr() as *const c_char;

const DMI_SYS_VENDOR: c_int = 0;
const DMI_BOARD_NAME: c_int = 0;
const GFP_KERNEL: gfp_t = 0;
const ENOMEM: c_int = 12;
const SND_JACK_HEADPHONE: c_int = 0;
const SND_JACK_MICROPHONE: c_int = 0;
const SND_JACK_BTN_0: c_int = 0;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 0;
const RT298_SCLK_S_PLL: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

type gfp_t = c_uint;
type bool_ = bool;

#[repr(C)]
pub struct device {
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
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct dmi_strmatch {
    pub slot: c_int,
    pub substr: *const c_char,
}

#[repr(C)]
pub struct dmi_system_id {
    pub matches: [dmi_strmatch; 4],
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
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_jack {
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
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
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
    pub num_cpus: c_uint,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub id: c_int,
    pub dai_fmt: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    pub be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub nonatomic: c_uint,
    pub no_pcm: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub driver_name: *const c_char,
    pub long_name: *const c_char,
    pub dev: *mut device,
    pub owner: *mut module,
    pub suspend_pre: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub resume_post: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub fully_routed: bool_,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub pdata: *mut avs_mach_pdata,
}

#[repr(C)]
pub struct avs_mach_pdata {
    pub obsolete_card_names: bool_,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_driver,
    pub id_table: *const platform_device_id,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn dmi_first_match(ids: *const dmi_system_id) -> *const dmi_system_id;
    fn devm_kmemdup_array(
        dev: *mut device,
        src: *const c_void,
        n: usize,
        size: usize,
        flags: gfp_t,
    ) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: gfp_t, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *const device) -> *const c_char;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, val: c_int);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_card_get_codec_dai(card: *mut snd_soc_card, dai_name: *const c_char)
        -> *mut snd_soc_dai;
    fn avs_mach_get_ssp_tdm(
        dev: *mut device,
        mach: *mut snd_soc_acpi_mach,
        ssp_port: *mut c_int,
        tdm_slot: *mut c_int,
    ) -> c_int;
    fn devm_snd_soc_register_deferrable_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

const fn dmi_match(slot: c_int, substr: &'static [u8]) -> dmi_strmatch {
    dmi_strmatch {
        slot,
        substr: substr.as_ptr() as *const c_char,
    }
}

const fn empty_dmi_match() -> dmi_strmatch {
    dmi_strmatch {
        slot: 0,
        substr: ptr::null(),
    }
}

static kblr_dmi_table: [dmi_system_id; 2] = [
    dmi_system_id {
        matches: [
            dmi_match(DMI_SYS_VENDOR, b"Intel Corporation\0"),
            dmi_match(DMI_BOARD_NAME, b"Kabylake R DDR4 RVP\0"),
            empty_dmi_match(),
            empty_dmi_match(),
        ],
    },
    dmi_system_id {
        matches: [
            empty_dmi_match(),
            empty_dmi_match(),
            empty_dmi_match(),
            empty_dmi_match(),
        ],
    },
];

// SOC_DAPM_PIN_SWITCH("Headphone Jack")
// SOC_DAPM_PIN_SWITCH("Mic Jack")
// SOC_DAPM_PIN_SWITCH("Speaker")
static card_controls: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

// SND_SOC_DAPM_HP("Headphone Jack", NULL)
// SND_SOC_DAPM_MIC("Mic Jack", NULL)
// SND_SOC_DAPM_SPK("Speaker", NULL)
static card_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static card_base_routes: [snd_soc_dapm_route; 4] = [
    /* HP jack connectors - unknown if we have jack detect */
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPO Pin\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"MIC1\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Mic Jack\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Speaker\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"SPOR\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Speaker\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"SPOL\0".as_ptr() as *const c_char,
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

unsafe extern "C" fn avs_rt298_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card: *mut snd_soc_card = unsafe { (*runtime).card };
    let pins: *mut snd_soc_jack_pin;
    let jack: *mut snd_soc_jack;
    let num_pins: c_uint;
    let ret: c_int;

    jack = unsafe { snd_soc_card_get_drvdata(card) as *mut snd_soc_jack };
    num_pins = card_headset_pins.len() as c_uint;

    pins = unsafe {
        devm_kmemdup_array(
            (*card).dev,
            card_headset_pins.as_ptr() as *const c_void,
            num_pins as usize,
            core::mem::size_of::<snd_soc_jack_pin>(),
            GFP_KERNEL,
        ) as *mut snd_soc_jack_pin
    };
    if pins.is_null() {
        return -ENOMEM;
    }

    ret = unsafe {
        snd_soc_card_jack_new_pins(
            card,
            b"Headset Jack\0".as_ptr() as *const c_char,
            SND_JACK_HEADSET | SND_JACK_BTN_0,
            jack,
            pins,
            num_pins,
        )
    };
    if ret != 0 {
        return ret;
    }

    unsafe {
        snd_soc_component_set_jack(
            (*snd_soc_rtd_to_codec(runtime, 0)).component,
            jack,
            ptr::null_mut(),
        )
    }
}

unsafe extern "C" fn avs_rt298_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
    unsafe {
        snd_soc_component_set_jack(
            (*snd_soc_rtd_to_codec(rtd, 0)).component,
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }
}

unsafe extern "C" fn avs_rt298_be_fixup(
    runtime: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate: *mut snd_interval;
    let channels: *mut snd_interval;
    let fmt: *mut snd_mask;

    let _ = runtime;
    rate = unsafe { hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE) };
    channels = unsafe { hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS) };
    fmt = unsafe { hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT) };

    /* The ADSP will convert the FE rate to 48k, stereo */
    unsafe {
        (*rate).max = 48000;
        (*rate).min = (*rate).max;
        (*channels).max = 2;
        (*channels).min = (*channels).max;
    }

    /* set SSP0 to 24 bit */
    unsafe {
        snd_mask_none(fmt);
        snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE);
    }

    0
}

unsafe extern "C" fn avs_rt298_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let clk_freq: c_uint;
    let ret: c_int;

    let _ = params;
    if unsafe { !dmi_first_match(kblr_dmi_table.as_ptr()).is_null() } {
        clk_freq = 24000000;
    } else {
        clk_freq = 19200000;
    }

    ret = unsafe {
        snd_soc_dai_set_sysclk(codec_dai, RT298_SCLK_S_PLL, clk_freq, SND_SOC_CLOCK_IN)
    };
    if ret < 0 {
        unsafe {
            dev_err(
                (*rtd).dev,
                b"Set codec sysclk failed: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
    }

    ret
}

static avs_rt298_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(avs_rt298_hw_params),
};

unsafe fn avs_string_fmt(
    dev: *mut device,
    prefix: &'static [u8],
    suffix: &'static [u8],
    ssp_port: c_int,
    tdm_slot: c_int,
) -> *mut c_char {
    // Translation of AVS_STRING_FMT(prefix, suffix, ssp_port, tdm_slot).
    unsafe {
        devm_kasprintf(
            dev,
            GFP_KERNEL,
            b"%s%d%s%d\0".as_ptr() as *const c_char,
            prefix.as_ptr() as *const c_char,
            ssp_port,
            suffix.as_ptr() as *const c_char,
            tdm_slot,
        )
    }
}

unsafe fn avs_create_dai_link(
    dev: *mut device,
    ssp_port: c_int,
    tdm_slot: c_int,
    dai_link: *mut *mut snd_soc_dai_link,
) -> c_int {
    let platform: *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link;

    dl = unsafe {
        devm_kzalloc(dev, core::mem::size_of::<snd_soc_dai_link>(), GFP_KERNEL)
            as *mut snd_soc_dai_link
    };
    platform = unsafe {
        devm_kzalloc(
            dev,
            core::mem::size_of::<snd_soc_dai_link_component>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component
    };
    if dl.is_null() || platform.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*dl).name = avs_string_fmt(dev, b"SSP\0", b"-Codec\0", ssp_port, tdm_slot);
        (*dl).cpus = devm_kzalloc(
            dev,
            core::mem::size_of_val(&*(*dl).cpus),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component;
        (*dl).codecs = devm_kzalloc(
            dev,
            core::mem::size_of_val(&*(*dl).codecs),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component;
    }
    if unsafe { (*dl).name.is_null() || (*dl).cpus.is_null() || (*dl).codecs.is_null() } {
        return -ENOMEM;
    }

    unsafe {
        (*(*dl).cpus).dai_name = avs_string_fmt(dev, b"SSP\0", b" Pin\0", ssp_port, tdm_slot);
        (*(*dl).codecs).name = devm_kasprintf(
            dev,
            GFP_KERNEL,
            b"i2c-INT343A:00\0".as_ptr() as *const c_char,
        );
        (*(*dl).codecs).dai_name = devm_kasprintf(dev, GFP_KERNEL, RT298_CODEC_DAI);
    }
    if unsafe {
        (*(*dl).cpus).dai_name.is_null()
            || (*(*dl).codecs).name.is_null()
            || (*(*dl).codecs).dai_name.is_null()
    } {
        return -ENOMEM;
    }

    unsafe {
        (*platform).name = dev_name(dev);
        (*dl).num_cpus = 1;
        (*dl).num_codecs = 1;
        (*dl).platforms = platform;
        (*dl).num_platforms = 1;
        (*dl).id = 0;
        if !dmi_first_match(kblr_dmi_table.as_ptr()).is_null() {
            (*dl).dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
        } else {
            (*dl).dai_fmt =
                SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
        }
        (*dl).init = Some(avs_rt298_codec_init);
        (*dl).exit = Some(avs_rt298_codec_exit);
        (*dl).be_hw_params_fixup = Some(avs_rt298_be_fixup);
        (*dl).ops = &avs_rt298_ops;
        (*dl).nonatomic = 1;
        (*dl).no_pcm = 1;

        *dai_link = dl;
    }

    0
}

unsafe extern "C" fn avs_card_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let codec_dai: *mut snd_soc_dai =
        unsafe { snd_soc_card_get_codec_dai(card, RT298_CODEC_DAI) };

    unsafe { snd_soc_component_set_jack((*codec_dai).component, ptr::null_mut(), ptr::null_mut()) }
}

unsafe extern "C" fn avs_card_resume_post(card: *mut snd_soc_card) -> c_int {
    let codec_dai: *mut snd_soc_dai =
        unsafe { snd_soc_card_get_codec_dai(card, RT298_CODEC_DAI) };
    let jack: *mut snd_soc_jack = unsafe { snd_soc_card_get_drvdata(card) as *mut snd_soc_jack };

    unsafe { snd_soc_component_set_jack((*codec_dai).component, jack, ptr::null_mut()) }
}

unsafe extern "C" fn avs_rt298_probe(pdev: *mut platform_device) -> c_int {
    let mut dai_link: *mut snd_soc_dai_link = ptr::null_mut();
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let card: *mut snd_soc_card;
    let jack: *mut snd_soc_jack;
    let dev: *mut device = unsafe { &mut (*pdev).dev };
    let mut ssp_port: c_int = 0;
    let mut tdm_slot: c_int = 0;
    let mut ret: c_int;

    mach = unsafe { dev_get_platdata(dev) as *mut snd_soc_acpi_mach };
    pdata = unsafe { (*mach).pdata };

    ret = unsafe { avs_mach_get_ssp_tdm(dev, mach, &mut ssp_port, &mut tdm_slot) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { avs_create_dai_link(dev, ssp_port, tdm_slot, &mut dai_link) };
    if ret != 0 {
        unsafe {
            dev_err(
                dev,
                b"Failed to create dai link: %d\0".as_ptr() as *const c_char,
                ret,
            );
        }
        return ret;
    }

    jack = unsafe {
        devm_kzalloc(dev, core::mem::size_of::<snd_soc_jack>(), GFP_KERNEL) as *mut snd_soc_jack
    };
    card = unsafe {
        devm_kzalloc(dev, core::mem::size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card
    };
    if jack.is_null() || card.is_null() {
        return -ENOMEM;
    }

    unsafe {
        if (*pdata).obsolete_card_names {
            (*card).name = b"avs_rt298\0".as_ptr() as *const c_char;
        } else {
            (*card).driver_name = b"avs_rt298\0".as_ptr() as *const c_char;
            (*card).name = b"AVS I2S ALC298\0".as_ptr() as *const c_char;
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
}

static avs_rt298_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char,
            b'v' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b'r' as c_char,
            b't' as c_char,
            b'2' as c_char,
            b'9' as c_char,
            b'8' as c_char,
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
// MODULE_DEVICE_TABLE(platform, avs_rt298_driver_ids);

static mut avs_rt298_driver: platform_driver = platform_driver {
    probe: Some(avs_rt298_probe),
    driver: platform_driver_driver {
        name: b"avs_rt298\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops },
    },
    id_table: avs_rt298_driver_ids.as_ptr(),
};

// module_platform_driver(avs_rt298_driver);
// MODULE_DESCRIPTION("Intel rt298 machine driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
