// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const fn BIT(n: u32) -> c_ulong {
    1u64.wrapping_shl(n) as c_ulong
}

const fn GENMASK(h: u32, l: u32) -> c_ulong {
    (((!0u64).wrapping_shl(l)) & ((!0u64).wrapping_shr(63 - h))) as c_ulong
}

const fn AVS_RT5682_SSP_CODEC(quirk: c_ulong) -> c_ulong {
    quirk & GENMASK(2, 0)
}

const AVS_RT5682_SSP_CODEC_MASK: c_ulong = GENMASK(2, 0);
const AVS_RT5682_MCLK_EN: c_ulong = BIT(3);
const AVS_RT5682_MCLK_24MHZ: c_ulong = BIT(4);
const AVS_RT5682_CODEC_DAI_NAME: *const c_char = b"rt5682-aif1\0".as_ptr() as *const c_char;

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;
const DMI_SYS_VENDOR: c_int = 0;
const DMI_PRODUCT_NAME: c_int = 0;
const SND_JACK_HEADPHONE: c_int = 0;
const SND_JACK_MICROPHONE: c_int = 0;
const SND_JACK_HEADSET: c_int = 0;
const SND_JACK_BTN_0: c_int = 0;
const SND_JACK_BTN_1: c_int = 0;
const SND_JACK_BTN_2: c_int = 0;
const SND_JACK_BTN_3: c_int = 0;
const KEY_PLAYPAUSE: c_int = 0;
const KEY_VOICECOMMAND: c_int = 0;
const KEY_VOLUMEUP: c_int = 0;
const KEY_VOLUMEDOWN: c_int = 0;
const RT5682_DA_STEREO1_FILTER: c_int = 0;
const RT5682_AD_STEREO1_FILTER: c_int = 0;
const RT5682_CLK_SEL_I2S1_ASRC: c_int = 0;
const RT5682_PLL1_S_MCLK: c_int = 0;
const RT5682_PLL1_S_BCLK1: c_int = 0;
const RT5682_PLL1: c_int = 0;
const RT5682_SCLK_S_PLL1: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

type c_uint = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub id_entry: *mut platform_device_id,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: [c_char; 20],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
    pub id_table: *const platform_device_id,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct dmi_system_id {
    pub callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> c_int>,
    pub ident: *const c_char,
    pub matches: [dmi_strmatch; 4],
    pub driver_data: *mut c_void,
}

#[repr(C)]
pub struct dmi_strmatch {
    pub slot: c_int,
    pub substr: *const c_char,
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
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
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
    pub fully_routed: bool,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
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
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
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
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub id: c_int,
    pub dai_fmt: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    pub be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub nonatomic: c_uint,
    pub no_pcm: c_uint,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub pdata: *mut avs_mach_pdata,
}

#[repr(C)]
pub struct avs_mach_pdata {
    pub obsolete_card_names: bool,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn DMI_MATCH(slot: c_int, substr: *const c_char) -> dmi_strmatch;
    fn SOC_DAPM_PIN_SWITCH(pin: *const c_char) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_HP(wname: *const c_char, wevent: *const c_void) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MIC(wname: *const c_char, wevent: *const c_void) -> snd_soc_dapm_widget;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn devm_kmemdup_array(dev: *mut device, src: *const c_void, n: usize, size: usize, flags: c_int) -> *mut c_void;
    fn rt5682_sel_asrc_clk_src(component: *mut snd_soc_component, filter_mask: c_int, clk_src: c_int);
    fn snd_soc_card_jack_new_pins(card: *mut snd_soc_card, id: *const c_char, type_: c_int, jack: *mut snd_soc_jack, pins: *mut snd_soc_jack_pin, num_pins: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn snd_soc_component_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, data: *mut c_void) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_int, freq_out: c_int) -> c_int;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_int, dir: c_int) -> c_int;
    fn snd_soc_dai_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn snd_soc_card_get_codec_dai(card: *mut snd_soc_card, dai_name: *const c_char) -> *mut snd_soc_dai;
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn avs_mach_get_ssp_tdm(dev: *mut device, mach: *mut snd_soc_acpi_mach, ssp_port: *mut c_int, tdm_slot: *mut c_int) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_deferrable_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

/* Default: MCLK on, MCLK 19.2M, SSP0 */
static mut avs_rt5682_quirk: c_ulong = AVS_RT5682_MCLK_EN | AVS_RT5682_SSP_CODEC(0);

unsafe extern "C" fn avs_rt5682_quirk_cb(id: *const dmi_system_id) -> c_int {
    avs_rt5682_quirk = (*id).driver_data as c_ulong;
    1
}

static avs_rt5682_quirk_table: [dmi_system_id; 3] = [
    dmi_system_id {
        callback: Some(avs_rt5682_quirk_cb),
        ident: ptr::null(),
        matches: [
            dmi_strmatch { slot: DMI_SYS_VENDOR, substr: b"Intel Corporation\0".as_ptr() as *const c_char },
            dmi_strmatch { slot: DMI_PRODUCT_NAME, substr: b"WhiskeyLake Client\0".as_ptr() as *const c_char },
            dmi_strmatch { slot: 0, substr: ptr::null() },
            dmi_strmatch { slot: 0, substr: ptr::null() },
        ],
        driver_data: (AVS_RT5682_MCLK_EN | AVS_RT5682_MCLK_24MHZ | AVS_RT5682_SSP_CODEC(1)) as *mut c_void,
    },
    dmi_system_id {
        callback: Some(avs_rt5682_quirk_cb),
        ident: ptr::null(),
        matches: [
            dmi_strmatch { slot: DMI_SYS_VENDOR, substr: b"Intel Corporation\0".as_ptr() as *const c_char },
            dmi_strmatch { slot: DMI_PRODUCT_NAME, substr: b"Ice Lake Client\0".as_ptr() as *const c_char },
            dmi_strmatch { slot: 0, substr: ptr::null() },
            dmi_strmatch { slot: 0, substr: ptr::null() },
        ],
        driver_data: (AVS_RT5682_MCLK_EN | AVS_RT5682_SSP_CODEC(0)) as *mut c_void,
    },
    dmi_system_id {
        callback: None,
        ident: ptr::null(),
        matches: [
            dmi_strmatch { slot: 0, substr: ptr::null() },
            dmi_strmatch { slot: 0, substr: ptr::null() },
            dmi_strmatch { slot: 0, substr: ptr::null() },
            dmi_strmatch { slot: 0, substr: ptr::null() },
        ],
        driver_data: ptr::null_mut(),
    },
];

static card_controls: [snd_kcontrol_new; 2] = [
    unsafe { SOC_DAPM_PIN_SWITCH(b"Headphone Jack\0".as_ptr() as *const c_char) },
    unsafe { SOC_DAPM_PIN_SWITCH(b"Headset Mic\0".as_ptr() as *const c_char) },
];

static card_widgets: [snd_soc_dapm_widget; 2] = [
    unsafe { SND_SOC_DAPM_HP(b"Headphone Jack\0".as_ptr() as *const c_char, ptr::null()) },
    unsafe { SND_SOC_DAPM_MIC(b"Headset Mic\0".as_ptr() as *const c_char, ptr::null()) },
];

static card_base_routes: [snd_soc_dapm_route; 3] = [
    /* HP jack connectors - unknown if we have jack detect */
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOR\0".as_ptr() as *const c_char },

    /* other jacks */
    snd_soc_dapm_route { sink: b"IN1P\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
];

static card_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone Jack\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn avs_rt5682_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let component = (*snd_soc_rtd_to_codec(runtime, 0)).component;
    let card = (*runtime).card;
    let pins: *mut snd_soc_jack_pin;
    let jack: *mut snd_soc_jack;
    let num_pins: c_int;
    let mut ret: c_int;

    jack = snd_soc_card_get_drvdata(card) as *mut snd_soc_jack;
    num_pins = card_jack_pins.len() as c_int;

    pins = devm_kmemdup_array(
        (*card).dev,
        card_jack_pins.as_ptr() as *const c_void,
        num_pins as usize,
        size_of::<snd_soc_jack_pin>(),
        GFP_KERNEL,
    ) as *mut snd_soc_jack_pin;
    if pins.is_null() {
        return -ENOMEM;
    }

    /* Need to enable ASRC function for 24MHz mclk rate */
    if (avs_rt5682_quirk & AVS_RT5682_MCLK_EN) != 0
        && (avs_rt5682_quirk & AVS_RT5682_MCLK_24MHZ) != 0
    {
        rt5682_sel_asrc_clk_src(
            component,
            RT5682_DA_STEREO1_FILTER | RT5682_AD_STEREO1_FILTER,
            RT5682_CLK_SEL_I2S1_ASRC,
        );
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        jack,
        pins,
        num_pins,
    );
    if ret != 0 {
        dev_err((*card).dev, b"Headset Jack creation failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

    ret = snd_soc_component_set_jack(component, jack, ptr::null_mut());
    if ret != 0 {
        dev_err((*card).dev, b"Headset Jack call-back failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn avs_rt5682_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
    snd_soc_component_set_jack((*snd_soc_rtd_to_codec(rtd, 0)).component, ptr::null_mut(), ptr::null_mut());
}

unsafe extern "C" fn avs_rt5682_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(runtime, 0);
    let pll_source: c_int;
    let freq_in: c_int;
    let freq_out: c_int;
    let mut ret: c_int;

    if (avs_rt5682_quirk & AVS_RT5682_MCLK_EN) != 0 {
        pll_source = RT5682_PLL1_S_MCLK;
        if (avs_rt5682_quirk & AVS_RT5682_MCLK_24MHZ) != 0 {
            freq_in = 24000000;
        } else {
            freq_in = 19200000;
        }
    } else {
        pll_source = RT5682_PLL1_S_BCLK1;
        freq_in = params_rate(params) * 50;
    }

    freq_out = params_rate(params) * 512;

    ret = snd_soc_dai_set_pll(codec_dai, RT5682_PLL1, pll_source, freq_in, freq_out);
    if ret < 0 {
        dev_err((*runtime).dev, b"Set PLL failed: %d\n\0".as_ptr() as *const c_char, ret);
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, RT5682_SCLK_S_PLL1, freq_out, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*runtime).dev, b"Set sysclk failed: %d\n\0".as_ptr() as *const c_char, ret);
    }

    /* slot_width should be equal or larger than data length. */
    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x0, 0x0, 2, params_width(params));
    if ret < 0 {
        dev_err((*runtime).dev, b"Set TDM slot failed: %d\n\0".as_ptr() as *const c_char, ret);
    }

    ret
}

static avs_rt5682_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(avs_rt5682_hw_params),
};

unsafe extern "C" fn avs_rt5682_be_fixup(
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

    /* set SSPN to 24 bit */
    snd_mask_none(fmt);
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE);

    0
}

unsafe extern "C" fn avs_create_dai_link(
    dev: *mut device,
    ssp_port: c_int,
    tdm_slot: c_int,
    dai_link: *mut *mut snd_soc_dai_link,
) -> c_int {
    let platform: *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link;

    dl = devm_kzalloc(dev, size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;
    platform = devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
        as *mut snd_soc_dai_link_component;
    if dl.is_null() || platform.is_null() {
        return -ENOMEM;
    }

    (*dl).name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        b"SSP%d-Codec%d\0".as_ptr() as *const c_char,
        ssp_port,
        tdm_slot,
    );
    (*dl).cpus = devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
        as *mut snd_soc_dai_link_component;
    (*dl).codecs = devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
        as *mut snd_soc_dai_link_component;
    if (*dl).name.is_null() || (*dl).cpus.is_null() || (*dl).codecs.is_null() {
        return -ENOMEM;
    }

    (*(*dl).cpus).dai_name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        b"SSP%d Pin%d\0".as_ptr() as *const c_char,
        ssp_port,
        tdm_slot,
    );
    (*(*dl).codecs).name = devm_kasprintf(dev, GFP_KERNEL, b"i2c-10EC5682:00\0".as_ptr() as *const c_char);
    (*(*dl).codecs).dai_name = devm_kasprintf(dev, GFP_KERNEL, AVS_RT5682_CODEC_DAI_NAME);
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
    (*dl).init = Some(avs_rt5682_codec_init);
    (*dl).exit = Some(avs_rt5682_codec_exit);
    (*dl).be_hw_params_fixup = Some(avs_rt5682_be_fixup);
    (*dl).ops = &avs_rt5682_ops;
    (*dl).nonatomic = 1;
    (*dl).no_pcm = 1;

    *dai_link = dl;

    0
}

unsafe extern "C" fn avs_card_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let codec_dai = snd_soc_card_get_codec_dai(card, AVS_RT5682_CODEC_DAI_NAME);

    snd_soc_component_set_jack((*codec_dai).component, ptr::null_mut(), ptr::null_mut())
}

unsafe extern "C" fn avs_card_resume_post(card: *mut snd_soc_card) -> c_int {
    let codec_dai = snd_soc_card_get_codec_dai(card, AVS_RT5682_CODEC_DAI_NAME);
    let jack = snd_soc_card_get_drvdata(card) as *mut snd_soc_jack;

    snd_soc_component_set_jack((*codec_dai).component, jack, ptr::null_mut())
}

unsafe extern "C" fn avs_rt5682_probe(pdev: *mut platform_device) -> c_int {
    let mut dai_link: *mut snd_soc_dai_link = ptr::null_mut();
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let card: *mut snd_soc_card;
    let jack: *mut snd_soc_jack;
    let dev = &mut (*pdev).dev as *mut device;
    let mut ssp_port: c_int = 0;
    let mut tdm_slot: c_int = 0;
    let mut ret: c_int;

    if !(*pdev).id_entry.is_null() && (*(*pdev).id_entry).driver_data != 0 {
        avs_rt5682_quirk = (*(*pdev).id_entry).driver_data as c_ulong;
    }

    dmi_check_system(avs_rt5682_quirk_table.as_ptr());
    dev_dbg(dev, b"avs_rt5682_quirk = %lx\n\0".as_ptr() as *const c_char, avs_rt5682_quirk);

    mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    pdata = (*mach).pdata;

    ret = avs_mach_get_ssp_tdm(dev, mach, &mut ssp_port, &mut tdm_slot);
    if ret != 0 {
        return ret;
    }

    ret = avs_create_dai_link(dev, ssp_port, tdm_slot, &mut dai_link);
    if ret != 0 {
        dev_err(dev, b"Failed to create dai link: %d\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    jack = devm_kzalloc(dev, size_of::<snd_soc_jack>(), GFP_KERNEL) as *mut snd_soc_jack;
    card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if jack.is_null() || card.is_null() {
        return -ENOMEM;
    }

    if (*pdata).obsolete_card_names {
        (*card).name = b"avs_rt5682\0".as_ptr() as *const c_char;
    } else {
        (*card).driver_name = b"avs_rt5682\0".as_ptr() as *const c_char;
        (*card).name = b"AVS I2S ALC5682\0".as_ptr() as *const c_char;
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

static avs_rt5682_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char, b'v' as c_char, b's' as c_char, b'_' as c_char,
            b'r' as c_char, b't' as c_char, b'5' as c_char, b'6' as c_char,
            b'8' as c_char, b'2' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        driver_data: 0,
    },
    platform_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(platform, avs_rt5682_driver_ids); */

static mut avs_rt5682_driver: platform_driver = platform_driver {
    probe: Some(avs_rt5682_probe),
    driver: device_driver {
        name: b"avs_rt5682\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    id_table: avs_rt5682_driver_ids.as_ptr(),
};

/* module_platform_driver(avs_rt5682_driver) */

/* MODULE_DESCRIPTION("Intel rt5682 machine driver"); */
/* MODULE_AUTHOR("Cezary Rojewski <cezary.rojewski@intel.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
