// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2020 Intel Corporation

/*
 * ehl_rt5660 - ASOC Machine driver for Elkhart Lake platforms
 * with rt5660 codec
 */

/*
 * C dependency intent:
 * linux/acpi.h, sound/core.h, linux/device.h, linux/errno.h, linux/gfp.h,
 * sound/jack.h, linux/kernel.h, linux/list.h, linux/module.h, sound/pcm.h,
 * sound/pcm_params.h, sound/soc.h, sound/soc-acpi.h, hda_dsp_common.h,
 * ../../codecs/rt5660.h
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const HDMI_LINK_START: c_int = 3;
const HDMI_LINE_END: c_int = 6;
const IDISP_CODEC_MASK: c_uint = 0x4;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const RT5660_SCLK_S_PLL1: c_int = 0;
const RT5660_PLL1_S_BCLK: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub id: c_int,
    pub no_pcm: c_uint,
    pub ops: *const snd_soc_ops,
    pub ignore_suspend: c_uint,
    pub capture_only: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub playback_only: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
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
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub fully_routed: bool,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dai_link: *mut snd_soc_dai_link,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub codec_mask: c_uint,
    pub platform: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub id_table: *const platform_device_id,
}

#[repr(C)]
struct sof_card_private {
    hdmi_pcm_list: list_head,
    idisp_codec: bool,
}

#[repr(C)]
struct sof_hdmi_pcm {
    head: list_head,
    codec_dai: *mut snd_soc_dai,
    device: c_int,
}

unsafe extern "C" {
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;
    static snd_soc_pm_ops: c_void;
    static mut THIS_MODULE: c_void;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
    fn hda_dsp_hdmi_build_controls(
        card: *mut snd_soc_card,
        component: *mut snd_soc_component,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_fixup_dai_links_platform_name(
        card: *mut snd_soc_card,
        platform_name: *const c_char,
    ) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

macro_rules! SOC_DAPM_PIN_SWITCH {
    ($name:expr) => {
        snd_kcontrol_new { _unused: [] }
    };
}

macro_rules! SND_SOC_DAPM_SPK {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget { _unused: [] }
    };
}

macro_rules! SND_SOC_DAPM_MIC {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget { _unused: [] }
    };
}

macro_rules! SND_SOC_DAPM_LINE {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget { _unused: [] }
    };
}

static rt5660_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_PIN_SWITCH!("Speaker"),
    /* There are two MICBIAS in rt5660, each for one MIC */
    SOC_DAPM_PIN_SWITCH!("Headset Mic"),
    SOC_DAPM_PIN_SWITCH!("Headset Mic2"),
    SOC_DAPM_PIN_SWITCH!("Line Out"),
];

static rt5660_widgets: [snd_soc_dapm_widget; 5] = [
    SND_SOC_DAPM_SPK!("Speaker", ptr::null()),
    SND_SOC_DAPM_MIC!("Headset Mic", ptr::null()),
    SND_SOC_DAPM_MIC!("Headset Mic2", ptr::null()),
    SND_SOC_DAPM_MIC!("SoC DMIC", ptr::null()),
    SND_SOC_DAPM_LINE!("Line Out", ptr::null()),
];

static rt5660_map: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: cstr(b"Speaker\0"), control: ptr::null(), source: cstr(b"SPO\0") },
    snd_soc_dapm_route { sink: cstr(b"Headset Mic\0"), control: ptr::null(), source: cstr(b"MICBIAS1\0") },
    snd_soc_dapm_route { sink: cstr(b"Headset Mic2\0"), control: ptr::null(), source: cstr(b"MICBIAS2\0") },
    snd_soc_dapm_route { sink: cstr(b"IN1P\0"), control: ptr::null(), source: cstr(b"Headset Mic\0") },
    snd_soc_dapm_route { sink: cstr(b"IN2P\0"), control: ptr::null(), source: cstr(b"Headset Mic2\0") },
    snd_soc_dapm_route { sink: cstr(b"Line Out\0"), control: ptr::null(), source: cstr(b"LOUTL\0") },
    snd_soc_dapm_route { sink: cstr(b"Line Out\0"), control: ptr::null(), source: cstr(b"LOUTR\0") },
    snd_soc_dapm_route { sink: cstr(b"DMic\0"), control: ptr::null(), source: cstr(b"SoC DMIC\0") },
];

unsafe extern "C" fn hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let ctx = unsafe { snd_soc_card_get_drvdata((*rtd).card) as *mut sof_card_private };
    let dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let pcm: *mut sof_hdmi_pcm;

    pcm = unsafe { devm_kzalloc((*(*rtd).card).dev, size_of::<sof_hdmi_pcm>(), GFP_KERNEL) as *mut sof_hdmi_pcm };
    if pcm.is_null() {
        return -ENOMEM;
    }

    /* dai_link id is 1:1 mapped to the PCM device */
    unsafe {
        (*pcm).device = (*(*rtd).dai_link).id;
        (*pcm).codec_dai = dai;

        list_add_tail(&mut (*pcm).head, &mut (*ctx).hdmi_pcm_list);
    }

    0
}

unsafe extern "C" fn card_late_probe(card: *mut snd_soc_card) -> c_int {
    let ctx = unsafe { snd_soc_card_get_drvdata(card) as *mut sof_card_private };
    let pcm: *mut sof_hdmi_pcm;

    if unsafe { list_empty(&(*ctx).hdmi_pcm_list) } != 0 {
        return -ENOENT;
    }

    if unsafe { !(*ctx).idisp_codec } {
        return 0;
    }

    pcm = unsafe { (*ctx).hdmi_pcm_list.next as *mut sof_hdmi_pcm };

    unsafe { hda_dsp_hdmi_build_controls(card, (*(*pcm).codec_dai).component) }
}

unsafe extern "C" fn rt5660_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let codec_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let mut ret: c_int;

    ret = unsafe {
        snd_soc_dai_set_sysclk(
            codec_dai,
            RT5660_SCLK_S_PLL1,
            params_rate(params).wrapping_mul(512),
            SND_SOC_CLOCK_IN,
        )
    };
    if ret < 0 {
        unsafe { dev_err((*rtd).dev, cstr(b"snd_soc_dai_set_sysclk err = %d\n\0"), ret) };
        return ret;
    }

    ret = unsafe {
        snd_soc_dai_set_pll(
            codec_dai,
            0,
            RT5660_PLL1_S_BCLK,
            params_rate(params).wrapping_mul(50),
            params_rate(params).wrapping_mul(512),
        )
    };
    if ret < 0 {
        unsafe { dev_err((*rtd).dev, cstr(b"can't set codec pll: %d\n\0"), ret) };
    }

    ret
}

static rt5660_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(rt5660_hw_params),
};

/*
 * SND_SOC_DAILINK_DEF(ssp0_pin, DAILINK_COMP_ARRAY(COMP_CPU("SSP0 Pin")));
 * SND_SOC_DAILINK_DEF(rt5660_codec,
 *     DAILINK_COMP_ARRAY(COMP_CODEC("i2c-10EC5660:00", "rt5660-aif1")));
 * SND_SOC_DAILINK_DEF(platform, DAILINK_COMP_ARRAY(COMP_PLATFORM("0000:00:1f.3")));
 * SND_SOC_DAILINK_DEF(dmic_pin, DAILINK_COMP_ARRAY(COMP_CPU("DMIC01 Pin")));
 * SND_SOC_DAILINK_DEF(dmic_codec, DAILINK_COMP_ARRAY(COMP_CODEC("dmic-codec", "dmic-hifi")));
 * SND_SOC_DAILINK_DEF(dmic16k, DAILINK_COMP_ARRAY(COMP_CPU("DMIC16k Pin")));
 * SND_SOC_DAILINK_DEF(idisp1_pin, DAILINK_COMP_ARRAY(COMP_CPU("iDisp1 Pin")));
 * SND_SOC_DAILINK_DEF(idisp1_codec,
 *     DAILINK_COMP_ARRAY(COMP_CODEC("ehdaudio0D2", "intel-hdmi-hifi1")));
 * SND_SOC_DAILINK_DEF(idisp2_pin, DAILINK_COMP_ARRAY(COMP_CPU("iDisp2 Pin")));
 * SND_SOC_DAILINK_DEF(idisp2_codec,
 *     DAILINK_COMP_ARRAY(COMP_CODEC("ehdaudio0D2", "intel-hdmi-hifi2")));
 * SND_SOC_DAILINK_DEF(idisp3_pin, DAILINK_COMP_ARRAY(COMP_CPU("iDisp3 Pin")));
 * SND_SOC_DAILINK_DEF(idisp3_codec,
 *     DAILINK_COMP_ARRAY(COMP_CODEC("ehdaudio0D2", "intel-hdmi-hifi3")));
 * SND_SOC_DAILINK_DEF(idisp4_pin, DAILINK_COMP_ARRAY(COMP_CPU("iDisp4 Pin")));
 * SND_SOC_DAILINK_DEF(idisp4_codec,
 *     DAILINK_COMP_ARRAY(COMP_CODEC("ehdaudio0D2", "intel-hdmi-hifi4")));
 */

static mut ehl_rt5660_dailink: [snd_soc_dai_link; 7] = [
    /* back ends */
    snd_soc_dai_link { name: cstr(b"SSP0-Codec\0"), id: 0, no_pcm: 1, ops: &rt5660_ops, ignore_suspend: 0, capture_only: 0, init: None, playback_only: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: cstr(b"dmic48k\0"), id: 1, no_pcm: 1, ops: ptr::null(), ignore_suspend: 1, capture_only: 1, init: None, playback_only: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: cstr(b"dmic16k\0"), id: 2, no_pcm: 1, ops: ptr::null(), ignore_suspend: 1, capture_only: 1, init: None, playback_only: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: cstr(b"iDisp1\0"), id: 5, no_pcm: 1, ops: ptr::null(), ignore_suspend: 0, capture_only: 0, init: Some(hdmi_init), playback_only: 1, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: cstr(b"iDisp2\0"), id: 6, no_pcm: 1, ops: ptr::null(), ignore_suspend: 0, capture_only: 0, init: Some(hdmi_init), playback_only: 1, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: cstr(b"iDisp3\0"), id: 7, no_pcm: 1, ops: ptr::null(), ignore_suspend: 0, capture_only: 0, init: Some(hdmi_init), playback_only: 1, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: cstr(b"iDisp4\0"), id: 8, no_pcm: 1, ops: ptr::null(), ignore_suspend: 0, capture_only: 0, init: Some(hdmi_init), playback_only: 1, codecs: ptr::null_mut() },
];

/* SoC card */
static mut snd_soc_card_ehl_rt5660: snd_soc_card = snd_soc_card {
    name: cstr(b"ehl-rt5660\0"),
    owner: unsafe { &mut THIS_MODULE as *mut c_void },
    dai_link: unsafe { ehl_rt5660_dailink.as_mut_ptr() },
    num_links: 7,
    dapm_widgets: rt5660_widgets.as_ptr(),
    num_dapm_widgets: 5,
    dapm_routes: rt5660_map.as_ptr(),
    num_dapm_routes: 8,
    controls: rt5660_controls.as_ptr(),
    num_controls: 4,
    fully_routed: true,
    late_probe: Some(card_late_probe),
    dev: ptr::null_mut(),
};

/* If hdmi codec is not supported, switch to use dummy codec */
unsafe fn hdmi_link_init(
    card: *mut snd_soc_card,
    ctx: *mut sof_card_private,
    mach: *mut snd_soc_acpi_mach,
) {
    let mut i: c_int;

    if unsafe { (*mach).mach_params.codec_mask & IDISP_CODEC_MASK } != 0 {
        unsafe {
            (*ctx).idisp_codec = true;
        }
        return;
    }

    /*
     * if HDMI is not enabled in kernel config, or
     * hdmi codec is not supported
     */
    i = HDMI_LINK_START;
    while i <= HDMI_LINE_END {
        unsafe {
            *(*(*card).dai_link.add(i as usize)).codecs.add(0) = snd_soc_dummy_dlc;
        }
        i += 1;
    }
}

unsafe extern "C" fn snd_ehl_rt5660_probe(pdev: *mut platform_device) -> c_int {
    let mach: *mut snd_soc_acpi_mach;
    let card: *mut snd_soc_card = unsafe { &mut snd_soc_card_ehl_rt5660 };
    let ctx: *mut sof_card_private;
    let ret: c_int;

    unsafe {
        (*card).dev = &mut (*pdev).dev;
    }

    ctx = unsafe { devm_kzalloc(&mut (*pdev).dev, size_of::<sof_card_private>(), GFP_KERNEL) as *mut sof_card_private };
    if ctx.is_null() {
        return -ENOMEM;
    }
    unsafe {
        INIT_LIST_HEAD(&mut (*ctx).hdmi_pcm_list);
        snd_soc_card_set_drvdata(card, ctx as *mut c_void);
    }

    mach = unsafe { (*pdev).dev.platform_data as *mut snd_soc_acpi_mach };
    ret = unsafe { snd_soc_fixup_dai_links_platform_name(card, (*mach).mach_params.platform) };
    if ret != 0 {
        return ret;
    }

    unsafe {
        hdmi_link_init(card, ctx, mach);
    }

    unsafe { devm_snd_soc_register_card(&mut (*pdev).dev, card) }
}

static ehl_board_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'e' as c_char, b'h' as c_char, b'l' as c_char, b'_' as c_char,
            b'r' as c_char, b't' as c_char, b'5' as c_char, b'6' as c_char,
            b'6' as c_char, b'0' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    },
    platform_device_id { name: [0; 20] },
];
/* MODULE_DEVICE_TABLE(platform, ehl_board_ids); */

static mut snd_ehl_rt5660_driver: platform_driver = platform_driver {
    driver: driver_private {
        name: cstr(b"ehl_rt5660\0"),
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    probe: Some(snd_ehl_rt5660_probe),
    id_table: ehl_board_ids.as_ptr(),
};

/* module_platform_driver(snd_ehl_rt5660_driver); */

/* MODULE_DESCRIPTION("ASoC Intel(R) Elkhartlake + rt5660 Machine driver"); */
/* MODULE_AUTHOR("libin.yang@intel.com"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_IMPORT_NS("SND_SOC_INTEL_HDA_DSP_COMMON"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
