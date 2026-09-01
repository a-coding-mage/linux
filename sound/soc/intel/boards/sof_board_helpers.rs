// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2023 Intel Corporation

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

type c_uint = u32;
type bool_t = bool;

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
pub struct snd_soc_card {
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_uchar,
    pub invert: c_uchar,
    pub kcontrol_news: *const c_void,
    pub num_kcontrols: c_int,
}

type c_uchar = u8;

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
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
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ignore_suspend: c_uint,
    pub no_pcm: c_uint,
    pub capture_only: c_uint,
    pub playback_only: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_acpi_intel_codec {
    CODEC_NONE = 0,
}

#[repr(C)]
pub struct sof_hdmi_private {
    pub hdmi_comp: *mut snd_soc_component,
    pub idisp_codec: bool_t,
}

#[repr(C)]
pub struct sof_card_private {
    pub hdmi: sof_hdmi_private,
    pub hdmi_num: c_int,
    pub codec_type: snd_soc_acpi_intel_codec,
    pub amp_type: snd_soc_acpi_intel_codec,
    pub dmic_be_num: c_int,
    pub ssp_codec: c_int,
    pub ssp_amp: c_int,
    pub bt_offload_present: bool_t,
    pub ssp_bt: c_int,
    pub ssp_mask_hdmi_in: c_ulong,
    pub hda_codec_present: bool_t,
    pub link_order_overwrite: c_ulong,
    pub link_id_overwrite: c_ulong,
    pub codec_link: *mut snd_soc_dai_link,
    pub amp_link: *mut snd_soc_dai_link,
}

unsafe extern "C" {
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn hda_dsp_hdmi_build_controls(
        card: *mut snd_soc_card,
        component: *mut snd_soc_component,
    ) -> c_int;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, gfp: c_uint) -> *mut c_void;
    fn soc_intel_is_byt() -> bool_t;
    fn soc_intel_is_cht() -> bool_t;
    fn snd_soc_acpi_intel_get_codec_name(codec: snd_soc_acpi_intel_codec) -> *const c_char;
    fn hweight32(w: c_uint) -> c_uint;
    fn snd_soc_acpi_intel_detect_codec_type(dev: *mut device) -> snd_soc_acpi_intel_codec;
    fn snd_soc_acpi_intel_detect_amp_type(dev: *mut device) -> snd_soc_acpi_intel_codec;

    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const SND_SOC_DAPM_MIC: c_int = 0;
const SND_SOC_DAPM_HP: c_int = 1;
const SND_SOC_DAPM_SPK: c_int = 2;

const fn dapm_widget(id: c_int, name: *const c_char) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget {
        id,
        name,
        reg: 0,
        shift: 0,
        invert: 0,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    }
}

/*
 * Intel HDMI DAI Link
 */
unsafe extern "C" fn hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let ctx = snd_soc_card_get_drvdata((*rtd).card) as *mut sof_card_private;
    let dai = snd_soc_rtd_to_codec(rtd, 0);

    (*ctx).hdmi.hdmi_comp = (*dai).component;

    0
}

#[no_mangle]
pub unsafe extern "C" fn sof_intel_board_card_late_probe(card: *mut snd_soc_card) -> c_int {
    let ctx = snd_soc_card_get_drvdata(card) as *mut sof_card_private;

    if (*ctx).hdmi_num == 0 {
        return 0;
    }

    if !(*ctx).hdmi.idisp_codec {
        return 0;
    }

    if (*ctx).hdmi.hdmi_comp.is_null() {
        return -EINVAL;
    }

    hda_dsp_hdmi_build_controls(card, (*ctx).hdmi.hdmi_comp)
}

// EXPORT_SYMBOL_NS(sof_intel_board_card_late_probe, "SND_SOC_INTEL_SOF_BOARD_HELPERS");

/*
 * DMIC DAI Link
 */
static dmic_widgets: [snd_soc_dapm_widget; 1] = [dapm_widget(SND_SOC_DAPM_MIC, cstr!("SoC DMIC"))];

static dmic_routes: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: cstr!("DMic"),
    control: ptr::null(),
    source: cstr!("SoC DMIC"),
}];

unsafe extern "C" fn dmic_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, dmic_widgets.as_ptr(), dmic_widgets.len() as c_int);
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            cstr!("fail to add dmic widgets, ret %d\n"),
            ret,
        );
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, dmic_routes.as_ptr(), dmic_routes.len() as c_int);
    if ret != 0 {
        dev_err((*rtd).dev, cstr!("fail to add dmic routes, ret %d\n"), ret);
        return ret;
    }

    0
}

/*
 * HDA External Codec DAI Link
 */
static hda_widgets: [snd_soc_dapm_widget; 6] = [
    dapm_widget(SND_SOC_DAPM_MIC, cstr!("Analog In")),
    dapm_widget(SND_SOC_DAPM_MIC, cstr!("Digital In")),
    dapm_widget(SND_SOC_DAPM_MIC, cstr!("Alt Analog In")),
    dapm_widget(SND_SOC_DAPM_HP, cstr!("Analog Out")),
    dapm_widget(SND_SOC_DAPM_SPK, cstr!("Digital Out")),
    dapm_widget(SND_SOC_DAPM_HP, cstr!("Alt Analog Out")),
];

static hda_routes: [snd_soc_dapm_route; 18] = [
    snd_soc_dapm_route { sink: cstr!("Codec Input Pin1"), control: ptr::null(), source: cstr!("Analog In") },
    snd_soc_dapm_route { sink: cstr!("Codec Input Pin2"), control: ptr::null(), source: cstr!("Digital In") },
    snd_soc_dapm_route { sink: cstr!("Codec Input Pin3"), control: ptr::null(), source: cstr!("Alt Analog In") },
    snd_soc_dapm_route { sink: cstr!("Analog Out"), control: ptr::null(), source: cstr!("Codec Output Pin1") },
    snd_soc_dapm_route { sink: cstr!("Digital Out"), control: ptr::null(), source: cstr!("Codec Output Pin2") },
    snd_soc_dapm_route { sink: cstr!("Alt Analog Out"), control: ptr::null(), source: cstr!("Codec Output Pin3") },
    /* CODEC BE connections */
    snd_soc_dapm_route { sink: cstr!("codec0_in"), control: ptr::null(), source: cstr!("Analog CPU Capture") },
    snd_soc_dapm_route { sink: cstr!("Analog CPU Capture"), control: ptr::null(), source: cstr!("Analog Codec Capture") },
    snd_soc_dapm_route { sink: cstr!("codec1_in"), control: ptr::null(), source: cstr!("Digital CPU Capture") },
    snd_soc_dapm_route { sink: cstr!("Digital CPU Capture"), control: ptr::null(), source: cstr!("Digital Codec Capture") },
    snd_soc_dapm_route { sink: cstr!("codec2_in"), control: ptr::null(), source: cstr!("Alt Analog CPU Capture") },
    snd_soc_dapm_route { sink: cstr!("Alt Analog CPU Capture"), control: ptr::null(), source: cstr!("Alt Analog Codec Capture") },
    snd_soc_dapm_route { sink: cstr!("Analog Codec Playback"), control: ptr::null(), source: cstr!("Analog CPU Playback") },
    snd_soc_dapm_route { sink: cstr!("Analog CPU Playback"), control: ptr::null(), source: cstr!("codec0_out") },
    snd_soc_dapm_route { sink: cstr!("Digital Codec Playback"), control: ptr::null(), source: cstr!("Digital CPU Playback") },
    snd_soc_dapm_route { sink: cstr!("Digital CPU Playback"), control: ptr::null(), source: cstr!("codec1_out") },
    snd_soc_dapm_route { sink: cstr!("Alt Analog Codec Playback"), control: ptr::null(), source: cstr!("Alt Analog CPU Playback") },
    snd_soc_dapm_route { sink: cstr!("Alt Analog CPU Playback"), control: ptr::null(), source: cstr!("codec2_out") },
];

unsafe extern "C" fn hda_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, hda_widgets.as_ptr(), hda_widgets.len() as c_int);
    if ret != 0 {
        dev_err((*rtd).dev, cstr!("fail to add hda widgets, ret %d\n"), ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, hda_routes.as_ptr(), hda_routes.len() as c_int);
    if ret != 0 {
        dev_err((*rtd).dev, cstr!("fail to add hda routes, ret %d\n"), ret);
    }

    ret
}

/*
 * DAI Link Helpers
 */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum sof_dmic_be_type {
    SOF_DMIC_01,
    SOF_DMIC_16K,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum sof_hda_be_type {
    SOF_HDA_ANALOG,
    SOF_HDA_DIGITAL,
}

const SOF_LINK_NONE: c_ulong = 0;
const SOF_LINK_CODEC: c_ulong = 1;
const SOF_LINK_DMIC01: c_ulong = 2;
const SOF_LINK_DMIC16K: c_ulong = 3;
const SOF_LINK_IDISP_HDMI: c_ulong = 4;
const SOF_LINK_AMP: c_ulong = 5;
const SOF_LINK_BT_OFFLOAD: c_ulong = 6;
const SOF_LINK_HDMI_IN: c_ulong = 7;
const SOF_LINK_HDA: c_ulong = 8;
const SOF_LINK_ORDER_SHIFT: c_ulong = 4;
const SOF_LINK_ORDER_MASK: c_ulong = 0xf;
const SOF_LINK_IDS_SHIFT: c_ulong = 4;
const SOF_LINK_IDS_MASK: c_ulong = 0xf;

const fn SOF_LINK_ORDER(
    a: c_ulong,
    b: c_ulong,
    c: c_ulong,
    d: c_ulong,
    e: c_ulong,
    f: c_ulong,
    g: c_ulong,
) -> c_ulong {
    a | (b << SOF_LINK_ORDER_SHIFT)
        | (c << (SOF_LINK_ORDER_SHIFT * 2))
        | (d << (SOF_LINK_ORDER_SHIFT * 3))
        | (e << (SOF_LINK_ORDER_SHIFT * 4))
        | (f << (SOF_LINK_ORDER_SHIFT * 5))
        | (g << (SOF_LINK_ORDER_SHIFT * 6))
}

/* DEFAULT_LINK_ORDER: the order used in sof_rt5682 */
const DEFAULT_LINK_ORDER: c_ulong = SOF_LINK_ORDER(
    SOF_LINK_CODEC,
    SOF_LINK_DMIC01,
    SOF_LINK_DMIC16K,
    SOF_LINK_IDISP_HDMI,
    SOF_LINK_AMP,
    SOF_LINK_BT_OFFLOAD,
    SOF_LINK_HDMI_IN,
);

static mut dmic_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: cstr!("dmic-codec"),
    dai_name: cstr!("dmic-hifi"),
}];

static mut hda_analog_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: cstr!("Analog CPU DAI"),
}];

static mut hda_analog_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: cstr!("ehdaudio0D0"),
    dai_name: cstr!("Analog Codec DAI"),
}];

static mut hda_digital_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: cstr!("Digital CPU DAI"),
}];

static mut hda_digital_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: cstr!("ehdaudio0D0"),
    dai_name: cstr!("Digital Codec DAI"),
}];

static mut platform_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    /* name might be overridden during probe */
    name: cstr!("0000:00:1f.3"),
    dai_name: ptr::null(),
}];

unsafe fn set_ssp_codec_link(
    dev: *mut device,
    link: *mut snd_soc_dai_link,
    be_id: c_int,
    codec_type: snd_soc_acpi_intel_codec,
    ssp_codec: c_int,
) -> c_int {
    let cpus: *mut snd_soc_dai_link_component;

    dev_dbg(
        dev,
        cstr!("link %d: ssp codec %s, ssp %d\n"),
        be_id,
        snd_soc_acpi_intel_get_codec_name(codec_type),
        ssp_codec,
    );

    /* link name */
    (*link).name = devm_kasprintf(dev, GFP_KERNEL, cstr!("SSP%d-Codec"), ssp_codec);
    if (*link).name.is_null() {
        return -ENOMEM;
    }

    /* cpus */
    cpus = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if cpus.is_null() {
        return -ENOMEM;
    }

    if soc_intel_is_byt() || soc_intel_is_cht() {
        /* backward-compatibility for BYT/CHT boards */
        (*cpus).dai_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("ssp%d-port"), ssp_codec);
    } else {
        (*cpus).dai_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("SSP%d Pin"), ssp_codec);
    }
    if (*cpus).dai_name.is_null() {
        return -ENOMEM;
    }

    (*link).cpus = cpus;
    (*link).num_cpus = 1;

    /* codecs - caller to handle */

    /* platforms */
    (*link).platforms = platform_component.as_mut_ptr();
    (*link).num_platforms = platform_component.len() as c_uint;

    (*link).id = be_id;
    (*link).no_pcm = 1;

    0
}

unsafe fn set_dmic_link(
    dev: *mut device,
    link: *mut snd_soc_dai_link,
    be_id: c_int,
    be_type: sof_dmic_be_type,
) -> c_int {
    let cpus: *mut snd_soc_dai_link_component;

    /* cpus */
    cpus = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if cpus.is_null() {
        return -ENOMEM;
    }

    match be_type {
        sof_dmic_be_type::SOF_DMIC_01 => {
            dev_dbg(dev, cstr!("link %d: dmic01\n"), be_id);

            (*link).name = cstr!("dmic01");
            (*cpus).dai_name = cstr!("DMIC01 Pin");
        }
        sof_dmic_be_type::SOF_DMIC_16K => {
            dev_dbg(dev, cstr!("link %d: dmic16k\n"), be_id);

            (*link).name = cstr!("dmic16k");
            (*cpus).dai_name = cstr!("DMIC16k Pin");
        }
    }

    (*link).cpus = cpus;
    (*link).num_cpus = 1;

    /* codecs */
    (*link).codecs = dmic_component.as_mut_ptr();
    (*link).num_codecs = dmic_component.len() as c_uint;

    /* platforms */
    (*link).platforms = platform_component.as_mut_ptr();
    (*link).num_platforms = platform_component.len() as c_uint;

    (*link).id = be_id;
    if be_type == sof_dmic_be_type::SOF_DMIC_01 {
        (*link).init = Some(dmic_init);
    }
    (*link).ignore_suspend = 1;
    (*link).no_pcm = 1;
    (*link).capture_only = 1;

    0
}

unsafe fn set_idisp_hdmi_link(
    dev: *mut device,
    link: *mut snd_soc_dai_link,
    be_id: c_int,
    hdmi_id: c_int,
    idisp_codec: bool_t,
) -> c_int {
    let cpus: *mut snd_soc_dai_link_component;
    let codecs: *mut snd_soc_dai_link_component;

    dev_dbg(
        dev,
        cstr!("link %d: idisp hdmi %d, idisp codec %d\n"),
        be_id,
        hdmi_id,
        idisp_codec as c_int,
    );

    /* link name */
    (*link).name = devm_kasprintf(dev, GFP_KERNEL, cstr!("iDisp%d"), hdmi_id);
    if (*link).name.is_null() {
        return -ENOMEM;
    }

    /* cpus */
    cpus = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if cpus.is_null() {
        return -ENOMEM;
    }

    (*cpus).dai_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("iDisp%d Pin"), hdmi_id);
    if (*cpus).dai_name.is_null() {
        return -ENOMEM;
    }

    (*link).cpus = cpus;
    (*link).num_cpus = 1;

    /* codecs */
    if idisp_codec {
        codecs = devm_kzalloc(
            dev,
            core::mem::size_of::<snd_soc_dai_link_component>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component;
        if codecs.is_null() {
            return -ENOMEM;
        }

        (*codecs).name = cstr!("ehdaudio0D2");
        (*codecs).dai_name =
            devm_kasprintf(dev, GFP_KERNEL, cstr!("intel-hdmi-hifi%d"), hdmi_id);
        if (*codecs).dai_name.is_null() {
            return -ENOMEM;
        }

        (*link).codecs = codecs;
    } else {
        (*link).codecs = &mut snd_soc_dummy_dlc;
    }
    (*link).num_codecs = 1;

    /* platforms */
    (*link).platforms = platform_component.as_mut_ptr();
    (*link).num_platforms = platform_component.len() as c_uint;

    (*link).id = be_id;
    (*link).init = if hdmi_id == 1 { Some(hdmi_init) } else { None };
    (*link).no_pcm = 1;
    (*link).playback_only = 1;

    0
}

unsafe fn set_ssp_amp_link(
    dev: *mut device,
    link: *mut snd_soc_dai_link,
    be_id: c_int,
    amp_type: snd_soc_acpi_intel_codec,
    ssp_amp: c_int,
) -> c_int {
    let cpus: *mut snd_soc_dai_link_component;

    dev_dbg(
        dev,
        cstr!("link %d: ssp amp %s, ssp %d\n"),
        be_id,
        snd_soc_acpi_intel_get_codec_name(amp_type),
        ssp_amp,
    );

    /* link name */
    (*link).name = devm_kasprintf(dev, GFP_KERNEL, cstr!("SSP%d-Codec"), ssp_amp);
    if (*link).name.is_null() {
        return -ENOMEM;
    }

    /* cpus */
    cpus = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if cpus.is_null() {
        return -ENOMEM;
    }

    (*cpus).dai_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("SSP%d Pin"), ssp_amp);
    if (*cpus).dai_name.is_null() {
        return -ENOMEM;
    }

    (*link).cpus = cpus;
    (*link).num_cpus = 1;

    /* codecs - caller to handle */

    /* platforms */
    /* feedback stream or firmware-generated echo reference */
    (*link).platforms = platform_component.as_mut_ptr();
    (*link).num_platforms = platform_component.len() as c_uint;

    (*link).id = be_id;
    (*link).no_pcm = 1;

    0
}

unsafe fn set_bt_offload_link(
    dev: *mut device,
    link: *mut snd_soc_dai_link,
    be_id: c_int,
    ssp_bt: c_int,
) -> c_int {
    let cpus: *mut snd_soc_dai_link_component;

    dev_dbg(dev, cstr!("link %d: bt offload, ssp %d\n"), be_id, ssp_bt);

    /* link name */
    (*link).name = devm_kasprintf(dev, GFP_KERNEL, cstr!("SSP%d-BT"), ssp_bt);
    if (*link).name.is_null() {
        return -ENOMEM;
    }

    /* cpus */
    cpus = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if cpus.is_null() {
        return -ENOMEM;
    }

    (*cpus).dai_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("SSP%d Pin"), ssp_bt);
    if (*cpus).dai_name.is_null() {
        return -ENOMEM;
    }

    (*link).cpus = cpus;
    (*link).num_cpus = 1;

    /* codecs */
    (*link).codecs = &mut snd_soc_dummy_dlc;
    (*link).num_codecs = 1;

    /* platforms */
    (*link).platforms = platform_component.as_mut_ptr();
    (*link).num_platforms = platform_component.len() as c_uint;

    (*link).id = be_id;
    (*link).no_pcm = 1;

    0
}

unsafe fn set_hdmi_in_link(
    dev: *mut device,
    link: *mut snd_soc_dai_link,
    be_id: c_int,
    ssp_hdmi: c_int,
) -> c_int {
    let cpus: *mut snd_soc_dai_link_component;

    dev_dbg(dev, cstr!("link %d: hdmi-in, ssp %d\n"), be_id, ssp_hdmi);

    /* link name */
    (*link).name = devm_kasprintf(dev, GFP_KERNEL, cstr!("SSP%d-HDMI"), ssp_hdmi);
    if (*link).name.is_null() {
        return -ENOMEM;
    }

    /* cpus */
    cpus = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if cpus.is_null() {
        return -ENOMEM;
    }

    (*cpus).dai_name = devm_kasprintf(dev, GFP_KERNEL, cstr!("SSP%d Pin"), ssp_hdmi);
    if (*cpus).dai_name.is_null() {
        return -ENOMEM;
    }

    (*link).cpus = cpus;
    (*link).num_cpus = 1;

    /* codecs */
    (*link).codecs = &mut snd_soc_dummy_dlc;
    (*link).num_codecs = 1;

    /* platforms */
    (*link).platforms = platform_component.as_mut_ptr();
    (*link).num_platforms = platform_component.len() as c_uint;

    (*link).id = be_id;
    (*link).no_pcm = 1;
    (*link).capture_only = 1;

    0
}

unsafe fn set_hda_codec_link(
    dev: *mut device,
    link: *mut snd_soc_dai_link,
    be_id: c_int,
    be_type: sof_hda_be_type,
) -> c_int {
    match be_type {
        sof_hda_be_type::SOF_HDA_ANALOG => {
            dev_dbg(dev, cstr!("link %d: hda analog\n"), be_id);

            (*link).name = cstr!("Analog Playback and Capture");

            /* cpus */
            (*link).cpus = hda_analog_cpus.as_mut_ptr();
            (*link).num_cpus = hda_analog_cpus.len() as c_uint;

            /* codecs */
            (*link).codecs = hda_analog_codecs.as_mut_ptr();
            (*link).num_codecs = hda_analog_codecs.len() as c_uint;
        }
        sof_hda_be_type::SOF_HDA_DIGITAL => {
            dev_dbg(dev, cstr!("link %d: hda digital\n"), be_id);

            (*link).name = cstr!("Digital Playback and Capture");

            /* cpus */
            (*link).cpus = hda_digital_cpus.as_mut_ptr();
            (*link).num_cpus = hda_digital_cpus.len() as c_uint;

            /* codecs */
            (*link).codecs = hda_digital_codecs.as_mut_ptr();
            (*link).num_codecs = hda_digital_codecs.len() as c_uint;
        }
    }

    /* platforms */
    (*link).platforms = platform_component.as_mut_ptr();
    (*link).num_platforms = platform_component.len() as c_uint;

    (*link).id = be_id;
    if be_type == sof_hda_be_type::SOF_HDA_ANALOG {
        (*link).init = Some(hda_init);
    }
    (*link).no_pcm = 1;

    0
}

unsafe fn calculate_num_links(ctx: *mut sof_card_private) -> c_int {
    let mut num_links: c_int = 0;

    /* headphone codec */
    if (*ctx).codec_type != snd_soc_acpi_intel_codec::CODEC_NONE {
        num_links += 1;
    }

    /* dmic01 and dmic16k */
    if (*ctx).dmic_be_num > 0 {
        num_links += 1;
    }

    if (*ctx).dmic_be_num > 1 {
        num_links += 1;
    }

    /* idisp HDMI */
    num_links += (*ctx).hdmi_num;

    /* speaker amp */
    if (*ctx).amp_type != snd_soc_acpi_intel_codec::CODEC_NONE {
        num_links += 1;
    }

    /* BT audio offload */
    if (*ctx).bt_offload_present {
        num_links += 1;
    }

    /* HDMI-In */
    num_links += hweight32((*ctx).ssp_mask_hdmi_in as c_uint) as c_int;

    /* HDA external codec */
    if (*ctx).hda_codec_present {
        num_links += 2;
    }

    num_links
}

#[no_mangle]
pub unsafe extern "C" fn sof_intel_board_set_dai_link(
    dev: *mut device,
    card: *mut snd_soc_card,
    ctx: *mut sof_card_private,
) -> c_int {
    let links: *mut snd_soc_dai_link;
    let num_links: c_int;
    let mut i: c_int;
    let mut idx: c_int = 0;
    let mut ret: c_int;
    let mut ssp_hdmi_in: c_int = 0;
    let mut link_order: c_ulong;
    let mut link: c_ulong;
    let mut link_ids: c_ulong;
    let mut be_id: c_ulong;

    num_links = calculate_num_links(ctx);

    links = devm_kcalloc(
        dev,
        num_links as usize,
        core::mem::size_of::<snd_soc_dai_link>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link;
    if links.is_null() {
        return -ENOMEM;
    }

    if (*ctx).link_order_overwrite != 0 {
        link_order = (*ctx).link_order_overwrite;
    } else {
        link_order = DEFAULT_LINK_ORDER;
    }

    if (*ctx).link_id_overwrite != 0 {
        link_ids = (*ctx).link_id_overwrite;
    } else {
        link_ids = 0;
    }

    dev_dbg(
        dev,
        cstr!("create dai links, link_order 0x%lx, id_overwrite 0x%lx\n"),
        link_order,
        link_ids,
    );

    while link_order != 0 {
        link = link_order & SOF_LINK_ORDER_MASK;
        link_order >>= SOF_LINK_ORDER_SHIFT;

        if (*ctx).link_id_overwrite != 0 {
            be_id = link_ids & SOF_LINK_IDS_MASK;
            link_ids >>= SOF_LINK_IDS_SHIFT;
        } else {
            /* use array index as link id */
            be_id = idx as c_ulong;
        }

        match link {
            SOF_LINK_CODEC => {
                /* headphone codec */
                if (*ctx).codec_type == snd_soc_acpi_intel_codec::CODEC_NONE {
                    continue;
                }

                ret = set_ssp_codec_link(
                    dev,
                    links.offset(idx as isize),
                    be_id as c_int,
                    (*ctx).codec_type,
                    (*ctx).ssp_codec,
                );
                if ret != 0 {
                    dev_err(dev, cstr!("fail to set codec link, ret %d\n"), ret);
                    return ret;
                }

                (*ctx).codec_link = links.offset(idx as isize);
                idx += 1;
            }
            SOF_LINK_DMIC01 => {
                /* dmic01 */
                if (*ctx).dmic_be_num == 0 {
                    continue;
                }

                /* at least we have dmic01 */
                ret = set_dmic_link(
                    dev,
                    links.offset(idx as isize),
                    be_id as c_int,
                    sof_dmic_be_type::SOF_DMIC_01,
                );
                if ret != 0 {
                    dev_err(dev, cstr!("fail to set dmic01 link, ret %d\n"), ret);
                    return ret;
                }

                idx += 1;
            }
            SOF_LINK_DMIC16K => {
                /* dmic16k */
                if (*ctx).dmic_be_num <= 1 {
                    continue;
                }

                /* set up 2 BE links at most */
                ret = set_dmic_link(
                    dev,
                    links.offset(idx as isize),
                    be_id as c_int,
                    sof_dmic_be_type::SOF_DMIC_16K,
                );
                if ret != 0 {
                    dev_err(dev, cstr!("fail to set dmic16k link, ret %d\n"), ret);
                    return ret;
                }

                idx += 1;
            }
            SOF_LINK_IDISP_HDMI => {
                /* idisp HDMI */
                i = 1;
                while i <= (*ctx).hdmi_num {
                    ret = set_idisp_hdmi_link(
                        dev,
                        links.offset(idx as isize),
                        be_id as c_int,
                        i,
                        (*ctx).hdmi.idisp_codec,
                    );
                    if ret != 0 {
                        dev_err(dev, cstr!("fail to set hdmi link, ret %d\n"), ret);
                        return ret;
                    }

                    idx += 1;
                    be_id += 1;
                    i += 1;
                }
            }
            SOF_LINK_AMP => {
                /* speaker amp */
                if (*ctx).amp_type == snd_soc_acpi_intel_codec::CODEC_NONE {
                    continue;
                }

                ret = set_ssp_amp_link(
                    dev,
                    links.offset(idx as isize),
                    be_id as c_int,
                    (*ctx).amp_type,
                    (*ctx).ssp_amp,
                );
                if ret != 0 {
                    dev_err(dev, cstr!("fail to set amp link, ret %d\n"), ret);
                    return ret;
                }

                (*ctx).amp_link = links.offset(idx as isize);
                idx += 1;
            }
            SOF_LINK_BT_OFFLOAD => {
                /* BT audio offload */
                if !(*ctx).bt_offload_present {
                    continue;
                }

                ret = set_bt_offload_link(
                    dev,
                    links.offset(idx as isize),
                    be_id as c_int,
                    (*ctx).ssp_bt,
                );
                if ret != 0 {
                    dev_err(dev, cstr!("fail to set bt link, ret %d\n"), ret);
                    return ret;
                }

                idx += 1;
            }
            SOF_LINK_HDMI_IN => {
                /* HDMI-In */
                ssp_hdmi_in = 0;
                while ssp_hdmi_in < 32 {
                    if (((*ctx).ssp_mask_hdmi_in >> ssp_hdmi_in) & 1) != 0 {
                        ret = set_hdmi_in_link(
                            dev,
                            links.offset(idx as isize),
                            be_id as c_int,
                            ssp_hdmi_in,
                        );
                        if ret != 0 {
                            dev_err(dev, cstr!("fail to set hdmi-in link, ret %d\n"), ret);
                            return ret;
                        }

                        idx += 1;
                        be_id += 1;
                    }
                    ssp_hdmi_in += 1;
                }
            }
            SOF_LINK_HDA => {
                /* HDA external codec */
                if !(*ctx).hda_codec_present {
                    continue;
                }

                ret = set_hda_codec_link(
                    dev,
                    links.offset(idx as isize),
                    be_id as c_int,
                    sof_hda_be_type::SOF_HDA_ANALOG,
                );
                if ret != 0 {
                    dev_err(dev, cstr!("fail to set hda analog link, ret %d\n"), ret);
                    return ret;
                }

                idx += 1;
                be_id += 1;

                ret = set_hda_codec_link(
                    dev,
                    links.offset(idx as isize),
                    be_id as c_int,
                    sof_hda_be_type::SOF_HDA_DIGITAL,
                );
                if ret != 0 {
                    dev_err(dev, cstr!("fail to set hda digital link, ret %d\n"), ret);
                    return ret;
                }

                idx += 1;
            }
            SOF_LINK_NONE | _ => {
                /* caught here if it's not used as terminator in macro */
                dev_err(dev, cstr!("invalid link type %ld\n"), link);
                return -EINVAL;
            }
        }
    }

    if idx != num_links {
        dev_err(
            dev,
            cstr!("link number mismatch, idx %d, num_links %d\n"),
            idx,
            num_links,
        );
        return -EINVAL;
    }

    (*card).dai_link = links;
    (*card).num_links = num_links;

    0
}

// EXPORT_SYMBOL_NS(sof_intel_board_set_dai_link, "SND_SOC_INTEL_SOF_BOARD_HELPERS");

const SOF_NUM_IDISP_HDMI_MASK: c_ulong = 0;
const SOF_NUM_IDISP_HDMI_SHIFT: c_ulong = 0;
const SOF_SSP_PORT_CODEC_MASK: c_ulong = 0;
const SOF_SSP_PORT_CODEC_SHIFT: c_ulong = 0;
const SOF_SSP_PORT_AMP_MASK: c_ulong = 0;
const SOF_SSP_PORT_AMP_SHIFT: c_ulong = 0;
const SOF_BT_OFFLOAD_PRESENT: c_ulong = 0;
const SOF_SSP_PORT_BT_OFFLOAD_MASK: c_ulong = 0;
const SOF_SSP_PORT_BT_OFFLOAD_SHIFT: c_ulong = 0;
const SOF_SSP_MASK_HDMI_CAPTURE_MASK: c_ulong = 0;
const SOF_SSP_MASK_HDMI_CAPTURE_SHIFT: c_ulong = 0;

#[no_mangle]
pub unsafe extern "C" fn sof_intel_board_get_ctx(
    dev: *mut device,
    board_quirk: c_ulong,
) -> *mut sof_card_private {
    let ctx: *mut sof_card_private;

    dev_dbg(dev, cstr!("create ctx, board_quirk 0x%lx\n"), board_quirk);

    ctx = devm_kzalloc(
        dev,
        core::mem::size_of::<sof_card_private>(),
        GFP_KERNEL,
    ) as *mut sof_card_private;
    if ctx.is_null() {
        return ptr::null_mut();
    }

    (*ctx).codec_type = snd_soc_acpi_intel_detect_codec_type(dev);
    (*ctx).amp_type = snd_soc_acpi_intel_detect_amp_type(dev);

    (*ctx).dmic_be_num = 2;
    (*ctx).hdmi_num =
        ((board_quirk & SOF_NUM_IDISP_HDMI_MASK) >> SOF_NUM_IDISP_HDMI_SHIFT) as c_int;
    /* default number of HDMI DAI's */
    if (*ctx).hdmi_num == 0 {
        (*ctx).hdmi_num = 3;
    }

    /* port number/mask of peripherals attached to ssp interface */
    if (*ctx).codec_type != snd_soc_acpi_intel_codec::CODEC_NONE {
        (*ctx).ssp_codec =
            ((board_quirk & SOF_SSP_PORT_CODEC_MASK) >> SOF_SSP_PORT_CODEC_SHIFT) as c_int;
    }

    if (*ctx).amp_type != snd_soc_acpi_intel_codec::CODEC_NONE {
        (*ctx).ssp_amp = ((board_quirk & SOF_SSP_PORT_AMP_MASK) >> SOF_SSP_PORT_AMP_SHIFT) as c_int;
    }

    if (board_quirk & SOF_BT_OFFLOAD_PRESENT) != 0 {
        (*ctx).bt_offload_present = true;
        (*ctx).ssp_bt = ((board_quirk & SOF_SSP_PORT_BT_OFFLOAD_MASK)
            >> SOF_SSP_PORT_BT_OFFLOAD_SHIFT) as c_int;
    }

    (*ctx).ssp_mask_hdmi_in =
        (board_quirk & SOF_SSP_MASK_HDMI_CAPTURE_MASK) >> SOF_SSP_MASK_HDMI_CAPTURE_SHIFT;

    ctx
}

// EXPORT_SYMBOL_NS(sof_intel_board_get_ctx, "SND_SOC_INTEL_SOF_BOARD_HELPERS");
// MODULE_DESCRIPTION("ASoC Intel SOF Machine Driver Board Helpers");
// MODULE_AUTHOR("Brent Lu <brent.lu@intel.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_INTEL_HDA_DSP_COMMON");
// MODULE_IMPORT_NS("SND_SOC_ACPI_INTEL_MATCH");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
