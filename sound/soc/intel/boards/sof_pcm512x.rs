// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2018-2020 Intel Corporation.

/*
 * Intel SOF Machine Driver for Intel platforms with TI PCM512x codec,
 * e.g. Up or Up2 with Hifiberry DAC+ HAT
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null_mut};

const NAME_SIZE: usize = 32;

const fn BIT(nr: c_int) -> c_ulong {
    1_c_ulong << nr
}

const fn GENMASK(h: c_int, l: c_int) -> c_ulong {
    ((!0_c_ulong) << l) & ((!0_c_ulong) >> ((c_ulong::BITS as c_int - 1) - h))
}

const fn SOF_PCM512X_SSP_CODEC(quirk: c_ulong) -> c_ulong {
    quirk & GENMASK(3, 0)
}

const SOF_PCM512X_SSP_CODEC_MASK: c_ulong = GENMASK(3, 0);
const SOF_PCM512X_ENABLE_SSP_CAPTURE: c_ulong = BIT(4);
const SOF_PCM512X_ENABLE_DMIC: c_ulong = BIT(5);

const IDISP_CODEC_MASK: c_int = 0x4;

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const PCM512x_GPIO_EN: c_int = 0;
const PCM512x_GPIO_OUTPUT_4: c_int = 0;
const PCM512x_GPIO_CONTROL_1: c_int = 0;
const DMI_SYS_VENDOR: c_int = 0;
const DMI_PRODUCT_NAME: c_int = 0;

/* Default: SSP5 */
static mut sof_pcm512x_quirk: c_ulong =
    SOF_PCM512X_SSP_CODEC(5) | SOF_PCM512X_ENABLE_SSP_CAPTURE | SOF_PCM512X_ENABLE_DMIC;

static mut is_legacy_cpu: bool = false;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub fully_routed: bool,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub num_links: c_int,
    pub dai_link: *mut snd_soc_dai_link,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
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
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub id: c_int,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_int,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_int,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub playback_only: c_int,
    pub no_pcm: c_int,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_int,
    pub ignore_suspend: c_int,
    pub capture_only: c_int,
}

#[repr(C)]
pub struct dmi_system_id {
    pub callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> c_int>,
    pub matches: [dmi_strmatch; 2],
    pub driver_data: *mut c_void,
}

#[repr(C)]
pub struct dmi_strmatch {
    pub slot: c_int,
    pub substr: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub codec_mask: c_int,
    pub platform: *const c_char,
}

#[repr(C)]
pub struct platform_device {
    pub dev: platform_device_dev,
}

#[repr(C)]
pub struct platform_device_dev {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const c_void,
}

#[repr(C)]
struct sof_hdmi_pcm {
    head: list_head,
    codec_dai: *mut snd_soc_dai,
    device: c_int,
}

#[repr(C)]
struct sof_card_private {
    hdmi_pcm_list: list_head,
    idisp_codec: bool,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static mut snd_soc_pm_ops: c_void;
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_int,
        mask: c_int,
        val: c_int,
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
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut platform_device_dev, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_int) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
    fn soc_intel_is_byt() -> bool;
    fn soc_intel_is_cht() -> bool;
    fn hda_dsp_hdmi_build_controls(card: *mut snd_soc_card, component: *mut snd_soc_component)
        -> c_int;
    fn snd_soc_fixup_dai_links_platform_name(
        card: *mut snd_soc_card,
        platform_name: *const c_char,
    ) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut platform_device_dev, card: *mut snd_soc_card)
        -> c_int;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut c_void,
        data: *mut c_void,
    ) -> c_int;
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn list_empty(head: *const list_head) -> bool {
    (*head).next == head as *mut list_head
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    let prev = (*head).prev;
    (*new).next = head;
    (*new).prev = prev;
    (*prev).next = new;
    (*head).prev = new;
}

unsafe fn list_first_entry_sof_hdmi_pcm(head: *mut list_head) -> *mut sof_hdmi_pcm {
    (*head).next as *mut sof_hdmi_pcm
}

unsafe extern "C" fn sof_pcm512x_quirk_cb(id: *const dmi_system_id) -> c_int {
    sof_pcm512x_quirk = (*id).driver_data as c_ulong;
    1
}

unsafe fn DMI_MATCH(slot: c_int, substr: *const c_char) -> dmi_strmatch {
    dmi_strmatch { slot, substr }
}

static mut sof_pcm512x_quirk_table: [dmi_system_id; 2] = [
    dmi_system_id {
        callback: Some(sof_pcm512x_quirk_cb),
        matches: [
            dmi_strmatch {
                slot: DMI_SYS_VENDOR,
                substr: b"AAEON\0".as_ptr() as *const c_char,
            },
            dmi_strmatch {
                slot: DMI_PRODUCT_NAME,
                substr: b"UP-CHT01\0".as_ptr() as *const c_char,
            },
        ],
        driver_data: SOF_PCM512X_SSP_CODEC(2) as *mut c_void,
    },
    dmi_system_id {
        callback: None,
        matches: [
            dmi_strmatch {
                slot: 0,
                substr: null_mut(),
            },
            dmi_strmatch {
                slot: 0,
                substr: null_mut(),
            },
        ],
        driver_data: null_mut(),
    },
];

unsafe extern "C" fn sof_hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let ctx = snd_soc_card_get_drvdata((*rtd).card) as *mut sof_card_private;
    let dai = snd_soc_rtd_to_codec(rtd, 0);
    let pcm: *mut sof_hdmi_pcm;

    pcm = devm_kzalloc((*(*rtd).card).dev, size_of::<sof_hdmi_pcm>(), GFP_KERNEL) as *mut sof_hdmi_pcm;
    if pcm.is_null() {
        return -ENOMEM;
    }

    /* dai_link id is 1:1 mapped to the PCM device */
    (*pcm).device = (*(*rtd).dai_link).id;
    (*pcm).codec_dai = dai;

    list_add_tail(addr_of_mut!((*pcm).head), addr_of_mut!((*ctx).hdmi_pcm_list));

    0
}

unsafe extern "C" fn sof_pcm512x_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    snd_soc_component_update_bits(codec, PCM512x_GPIO_EN, 0x08, 0x08);
    snd_soc_component_update_bits(codec, PCM512x_GPIO_OUTPUT_4, 0x0f, 0x02);
    snd_soc_component_update_bits(codec, PCM512x_GPIO_CONTROL_1, 0x08, 0x08);

    0
}

unsafe extern "C" fn aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    snd_soc_component_update_bits(codec, PCM512x_GPIO_CONTROL_1, 0x08, 0x08);

    0
}

unsafe extern "C" fn aif1_shutdown(substream: *mut snd_pcm_substream) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    snd_soc_component_update_bits(codec, PCM512x_GPIO_CONTROL_1, 0x08, 0x00);
}

static sof_pcm512x_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(aif1_startup),
    shutdown: Some(aif1_shutdown),
};

static mut platform_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    /* name might be overridden during probe */
    name: b"0000:00:1f.3\0".as_ptr() as *const c_char,
    dai_name: null_mut(),
}];

unsafe extern "C" fn sof_card_late_probe(card: *mut snd_soc_card) -> c_int {
    let ctx = snd_soc_card_get_drvdata(card) as *mut sof_card_private;
    let pcm: *mut sof_hdmi_pcm;

    /* HDMI is not supported by SOF on Baytrail/CherryTrail */
    if is_legacy_cpu {
        return 0;
    }

    if list_empty(addr_of_mut!((*ctx).hdmi_pcm_list)) {
        return -EINVAL;
    }

    if !(*ctx).idisp_codec {
        return 0;
    }

    pcm = list_first_entry_sof_hdmi_pcm(addr_of_mut!((*ctx).hdmi_pcm_list));

    hda_dsp_hdmi_build_controls(card, (*(*(*pcm).codec_dai).component).into())
}

static sof_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    name: b"Ext Spk\0".as_ptr() as *const c_char,
}];

static sof_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget {
    name: b"Ext Spk\0".as_ptr() as *const c_char,
}];

static dmic_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget {
    name: b"SoC DMIC\0".as_ptr() as *const c_char,
}];

static sof_map: [snd_soc_dapm_route; 2] = [
    /* Speaker */
    snd_soc_dapm_route {
        sink: b"Ext Spk\0".as_ptr() as *const c_char,
        control: null_mut(),
        source: b"OUTR\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Ext Spk\0".as_ptr() as *const c_char,
        control: null_mut(),
        source: b"OUTL\0".as_ptr() as *const c_char,
    },
];

static dmic_map: [snd_soc_dapm_route; 1] = [
    /* digital mics */
    snd_soc_dapm_route {
        sink: b"DMic\0".as_ptr() as *const c_char,
        control: null_mut(),
        source: b"SoC DMIC\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn dmic_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, dmic_widgets.as_ptr(), dmic_widgets.len() as c_int);
    if ret != 0 {
        dev_err(
            (*card).dev,
            b"DMic widget addition failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        /* Don't need to add routes if widget addition failed */
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, dmic_map.as_ptr(), dmic_map.len() as c_int);

    if ret != 0 {
        dev_err(
            (*card).dev,
            b"DMic map addition failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    ret
}

/* sof audio machine driver for pcm512x codec */
static mut sof_audio_card_pcm512x: snd_soc_card = snd_soc_card {
    name: b"pcm512x\0".as_ptr() as *const c_char,
    owner: null_mut(),
    controls: sof_controls.as_ptr(),
    num_controls: sof_controls.len() as c_int,
    dapm_widgets: sof_widgets.as_ptr(),
    num_dapm_widgets: sof_widgets.len() as c_int,
    dapm_routes: sof_map.as_ptr(),
    num_dapm_routes: sof_map.len() as c_int,
    fully_routed: true,
    late_probe: Some(sof_card_late_probe),
    num_links: 0,
    dai_link: null_mut(),
    dev: null_mut(),
};

/* SND_SOC_DAILINK_DEF(pcm512x_component,
 *     DAILINK_COMP_ARRAY(COMP_CODEC("i2c-104C5122:00", "pcm512x-hifi")));
 * SND_SOC_DAILINK_DEF(dmic_component,
 *     DAILINK_COMP_ARRAY(COMP_CODEC("dmic-codec", "dmic-hifi")));
 */
static mut pcm512x_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"i2c-104C5122:00\0".as_ptr() as *const c_char,
    dai_name: b"pcm512x-hifi\0".as_ptr() as *const c_char,
}];

static mut dmic_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"dmic-codec\0".as_ptr() as *const c_char,
    dai_name: b"dmic-hifi\0".as_ptr() as *const c_char,
}];

unsafe fn sof_card_dai_links_create(
    dev: *mut device,
    ssp_codec: c_int,
    mut dmic_be_num: c_int,
    hdmi_num: c_int,
    idisp_codec: bool,
) -> *mut snd_soc_dai_link {
    let mut idisp_components: *mut snd_soc_dai_link_component = null_mut();
    let cpus: *mut snd_soc_dai_link_component;
    let links: *mut snd_soc_dai_link;
    let mut i: c_int;
    let mut id: c_int = 0;

    links = devm_kcalloc(
        dev,
        sof_audio_card_pcm512x.num_links as usize,
        size_of::<snd_soc_dai_link>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link;
    cpus = devm_kcalloc(
        dev,
        sof_audio_card_pcm512x.num_links as usize,
        size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if links.is_null() || cpus.is_null() {
        return null_mut();
    }

    /* codec SSP */
    (*links.add(id as usize)).name =
        devm_kasprintf(dev, GFP_KERNEL, b"SSP%d-Codec\0".as_ptr() as *const c_char, ssp_codec);
    if (*links.add(id as usize)).name.is_null() {
        return null_mut();
    }

    (*links.add(id as usize)).id = id;
    (*links.add(id as usize)).codecs = pcm512x_component.as_mut_ptr();
    (*links.add(id as usize)).num_codecs = pcm512x_component.len() as c_int;
    (*links.add(id as usize)).platforms = platform_component.as_mut_ptr();
    (*links.add(id as usize)).num_platforms = platform_component.len() as c_int;
    (*links.add(id as usize)).init = Some(sof_pcm512x_codec_init);
    (*links.add(id as usize)).ops = &sof_pcm512x_ops;
    /*
     * capture only supported with specific versions of the Hifiberry DAC+
     */
    if (sof_pcm512x_quirk & SOF_PCM512X_ENABLE_SSP_CAPTURE) == 0 {
        (*links.add(id as usize)).playback_only = 1;
    }
    (*links.add(id as usize)).no_pcm = 1;
    (*links.add(id as usize)).cpus = cpus.add(id as usize);
    (*links.add(id as usize)).num_cpus = 1;
    if is_legacy_cpu {
        (*(*links.add(id as usize)).cpus).dai_name =
            devm_kasprintf(dev, GFP_KERNEL, b"ssp%d-port\0".as_ptr() as *const c_char, ssp_codec);
        if (*(*links.add(id as usize)).cpus).dai_name.is_null() {
            return null_mut();
        }
    } else {
        (*(*links.add(id as usize)).cpus).dai_name =
            devm_kasprintf(dev, GFP_KERNEL, b"SSP%d Pin\0".as_ptr() as *const c_char, ssp_codec);
        if (*(*links.add(id as usize)).cpus).dai_name.is_null() {
            return null_mut();
        }
    }
    id += 1;

    /* dmic */
    if dmic_be_num > 0 {
        /* at least we have dmic01 */
        (*links.add(id as usize)).name = b"dmic01\0".as_ptr() as *const c_char;
        (*links.add(id as usize)).cpus = cpus.add(id as usize);
        (*(*links.add(id as usize)).cpus).dai_name = b"DMIC01 Pin\0".as_ptr() as *const c_char;
        (*links.add(id as usize)).init = Some(dmic_init);
        if dmic_be_num > 1 {
            /* set up 2 BE links at most */
            (*links.add((id + 1) as usize)).name = b"dmic16k\0".as_ptr() as *const c_char;
            (*links.add((id + 1) as usize)).cpus = cpus.add((id + 1) as usize);
            (*(*links.add((id + 1) as usize)).cpus).dai_name =
                b"DMIC16k Pin\0".as_ptr() as *const c_char;
            dmic_be_num = 2;
        }
    }

    i = 0;
    while i < dmic_be_num {
        (*links.add(id as usize)).id = id;
        (*links.add(id as usize)).num_cpus = 1;
        (*links.add(id as usize)).codecs = dmic_component.as_mut_ptr();
        (*links.add(id as usize)).num_codecs = dmic_component.len() as c_int;
        (*links.add(id as usize)).platforms = platform_component.as_mut_ptr();
        (*links.add(id as usize)).num_platforms = platform_component.len() as c_int;
        (*links.add(id as usize)).ignore_suspend = 1;
        (*links.add(id as usize)).capture_only = 1;
        (*links.add(id as usize)).no_pcm = 1;
        id += 1;
        i += 1;
    }

    /* HDMI */
    if hdmi_num > 0 {
        idisp_components = devm_kcalloc(
            dev,
            hdmi_num as usize,
            size_of::<snd_soc_dai_link_component>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component;
        if idisp_components.is_null() {
            return null_mut();
        }
    }
    i = 1;
    while i <= hdmi_num {
        (*links.add(id as usize)).name =
            devm_kasprintf(dev, GFP_KERNEL, b"iDisp%d\0".as_ptr() as *const c_char, i);
        if (*links.add(id as usize)).name.is_null() {
            return null_mut();
        }

        (*links.add(id as usize)).id = id;
        (*links.add(id as usize)).cpus = cpus.add(id as usize);
        (*links.add(id as usize)).num_cpus = 1;
        (*(*links.add(id as usize)).cpus).dai_name =
            devm_kasprintf(dev, GFP_KERNEL, b"iDisp%d Pin\0".as_ptr() as *const c_char, i);
        if (*(*links.add(id as usize)).cpus).dai_name.is_null() {
            return null_mut();
        }

        /*
         * topology cannot be loaded if codec is missing, so
         * use the dummy codec if needed
         */
        if idisp_codec {
            (*idisp_components.add((i - 1) as usize)).name =
                b"ehdaudio0D2\0".as_ptr() as *const c_char;
            (*idisp_components.add((i - 1) as usize)).dai_name = devm_kasprintf(
                dev,
                GFP_KERNEL,
                b"intel-hdmi-hifi%d\0".as_ptr() as *const c_char,
                i,
            );
        } else {
            *idisp_components.add((i - 1) as usize) = snd_soc_dummy_dlc;
        }
        if (*idisp_components.add((i - 1) as usize)).dai_name.is_null() {
            return null_mut();
        }

        (*links.add(id as usize)).codecs = idisp_components.add((i - 1) as usize);
        (*links.add(id as usize)).num_codecs = 1;
        (*links.add(id as usize)).platforms = platform_component.as_mut_ptr();
        (*links.add(id as usize)).num_platforms = platform_component.len() as c_int;
        (*links.add(id as usize)).init = Some(sof_hdmi_init);
        (*links.add(id as usize)).playback_only = 1;
        (*links.add(id as usize)).no_pcm = 1;
        id += 1;
        i += 1;
    }

    links
}

unsafe extern "C" fn sof_audio_probe(pdev: *mut platform_device) -> c_int {
    let mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;
    let dai_links: *mut snd_soc_dai_link;
    let ctx: *mut sof_card_private;
    let mut dmic_be_num: c_int;
    let mut hdmi_num: c_int;
    let ret: c_int;
    let ssp_codec: c_int;

    ctx = devm_kzalloc(
        &mut (*pdev).dev as *mut platform_device_dev as *mut device,
        size_of::<sof_card_private>(),
        GFP_KERNEL,
    ) as *mut sof_card_private;
    if ctx.is_null() {
        return -ENOMEM;
    }

    hdmi_num = 0;
    if soc_intel_is_byt() || soc_intel_is_cht() {
        is_legacy_cpu = true;
        dmic_be_num = 0;
        /* default quirk for legacy cpu */
        sof_pcm512x_quirk = SOF_PCM512X_SSP_CODEC(2);
    } else {
        dmic_be_num = 2;
        if ((*mach).mach_params.codec_mask & IDISP_CODEC_MASK) != 0 {
            (*ctx).idisp_codec = true;
        }

        /* links are always present in topology */
        hdmi_num = 3;
    }

    dmi_check_system(sof_pcm512x_quirk_table.as_ptr());

    dev_dbg(
        &mut (*pdev).dev,
        b"sof_pcm512x_quirk = %lx\n\0".as_ptr() as *const c_char,
        sof_pcm512x_quirk,
    );

    ssp_codec = (sof_pcm512x_quirk & SOF_PCM512X_SSP_CODEC_MASK) as c_int;

    if (sof_pcm512x_quirk & SOF_PCM512X_ENABLE_DMIC) == 0 {
        dmic_be_num = 0;
    }

    /* compute number of dai links */
    sof_audio_card_pcm512x.num_links = 1 + dmic_be_num + hdmi_num;

    dai_links = sof_card_dai_links_create(
        &mut (*pdev).dev as *mut platform_device_dev as *mut device,
        ssp_codec,
        dmic_be_num,
        hdmi_num,
        (*ctx).idisp_codec,
    );
    if dai_links.is_null() {
        return -ENOMEM;
    }

    sof_audio_card_pcm512x.dai_link = dai_links;

    INIT_LIST_HEAD(addr_of_mut!((*ctx).hdmi_pcm_list));

    sof_audio_card_pcm512x.dev = &mut (*pdev).dev as *mut platform_device_dev as *mut device;

    /* set platform name for each dailink */
    ret = snd_soc_fixup_dai_links_platform_name(&mut sof_audio_card_pcm512x, (*mach).mach_params.platform);
    if ret != 0 {
        return ret;
    }

    snd_soc_card_set_drvdata(&mut sof_audio_card_pcm512x, ctx as *mut c_void);

    devm_snd_soc_register_card(&mut (*pdev).dev, &mut sof_audio_card_pcm512x)
}

unsafe extern "C" fn sof_pcm512x_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    let mut component: *mut snd_soc_component;

    /* for_each_card_components(card, component) */
    component = null_mut();
    while !component.is_null() {
        if strcmp((*component).name, pcm512x_component[0].name) == 0 {
            snd_soc_component_set_jack(component, null_mut(), null_mut());
            break;
        }
    }
}

static mut sof_audio: platform_driver = platform_driver {
    probe: Some(sof_audio_probe),
    remove: Some(sof_pcm512x_remove),
    driver: device_driver {
        name: b"sof_pcm512x\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
};

/* module_platform_driver(sof_audio) */

/* MODULE_DESCRIPTION("ASoC Intel(R) SOF + PCM512x Machine driver"); */
/* MODULE_AUTHOR("Pierre-Louis Bossart"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:sof_pcm512x"); */
/* MODULE_IMPORT_NS("SND_SOC_INTEL_HDA_DSP_COMMON"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
