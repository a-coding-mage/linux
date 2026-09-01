// SPDX-License-Identifier: GPL-2.0-only
// Based on sof_sdw_rt5682.c
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2023 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.
// Copyright (c) 2025 Everest Semiconductor Co., Ltd

/*
 *  soc_sdw_es9356 - Helpers to handle ES9356 from generic machine driver
 */

// C dependencies:
// linux/device.h, linux/errno.h, linux/input.h, linux/soundwire/sdw.h,
// linux/soundwire/sdw_type.h, sound/control.h, sound/soc.h,
// sound/soc-acpi.h, sound/soc-dapm.h, sound/jack.h,
// sound/soc_sdw_utils.h

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct software_node {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct property_entry {
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
pub struct snd_soc_jack_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub codecs: *mut snd_soc_dai_link_component,
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_soc_jack_jack,
}

#[repr(C)]
pub struct asoc_sdw_mc_private {
    pub sdw_headset: snd_soc_jack,
    pub headset_codec_dev: *mut device,
    pub mc_quirk: c_ulong,
}

#[repr(C)]
pub struct asoc_sdw_codec_info {
    pub amp_num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

unsafe extern "C" {
    static sdw_bus_type: c_void;

    static GFP_KERNEL: c_int;
    static ENOMEM: c_int;
    static EPROBE_DEFER: c_int;
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_MICROPHONE: c_int;
    static SND_JACK_HEADSET: c_int;
    static SND_JACK_BTN_0: c_int;
    static SND_JACK_BTN_1: c_int;
    static SND_JACK_BTN_2: c_int;
    static SND_JACK_BTN_3: c_int;
    static SND_JACK_BTN_4: c_int;
    static KEY_PLAYPAUSE: c_int;
    static KEY_VOLUMEUP: c_int;
    static KEY_VOLUMEDOWN: c_int;
    static KEY_NEXTSONG: c_int;
    static KEY_PREVIOUSSONG: c_int;

    fn SOC_SDW_JACK_JDSRC(quirk: c_ulong) -> u32;
    fn PROPERTY_ENTRY_U32(name: *const c_char, value: u32) -> property_entry;
    fn fwnode_create_software_node(
        props: *const property_entry,
        parent: *const software_node,
    ) -> *mut fwnode_handle;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn device_add_software_node(dev: *mut device, node: *const software_node) -> c_int;
    fn to_software_node(fwnode: *mut fwnode_handle) -> *const software_node;
    fn fwnode_handle_put(fwnode: *mut fwnode_handle);
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, gfp: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_int,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_soc_jack_jack, type_: c_int, keytype: c_int);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn device_remove_software_node(dev: *mut device);
    fn put_device(dev: *mut device);
    fn bus_find_device_by_name(
        bus: *const c_void,
        start: *mut device,
        name: *const c_char,
    ) -> *mut device;
}

const SOC_SDW_MAX_NO_PROPS: usize = 8;

/*
 * Note this MUST be called before snd_soc_register_card(), so that the props
 * are in place before the codec component driver's probe function parses them.
 */
unsafe fn es9356_add_codec_device_props(sdw_dev: *mut device, quirk: c_ulong) -> c_int {
    let mut props: [property_entry; SOC_SDW_MAX_NO_PROPS] =
        core::mem::zeroed::<[property_entry; SOC_SDW_MAX_NO_PROPS]>();
    let fwnode: *mut fwnode_handle;
    let ret: c_int;

    if SOC_SDW_JACK_JDSRC(quirk) == 0 {
        return 0;
    }
    props[0] = PROPERTY_ENTRY_U32(
        c"everest,jd-src".as_ptr(),
        SOC_SDW_JACK_JDSRC(quirk),
    );

    fwnode = fwnode_create_software_node(props.as_ptr(), ptr::null());
    if IS_ERR(fwnode as *const c_void) {
        return PTR_ERR(fwnode as *const c_void);
    }

    ret = device_add_software_node(sdw_dev, to_software_node(fwnode));

    fwnode_handle_put(fwnode);

    ret
}

static es9356_map: [snd_soc_dapm_route; 2] = [
    /* Headphones */
    snd_soc_dapm_route {
        sink: c"Headphone".as_ptr(),
        control: ptr::null(),
        source: c"es9356 HP".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"es9356 MIC1".as_ptr(),
        control: ptr::null(),
        source: c"Headset Mic".as_ptr(),
    },
];

static es9356_spk_map: [snd_soc_dapm_route; 1] = [
    /* Speaker */
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ptr::null(),
        source: c"es9356 SPK".as_ptr(),
    },
];

static es9356_dmic_map: [snd_soc_dapm_route; 1] = [
    /* DMIC */
    snd_soc_dapm_route {
        sink: c"es9356 PDM_DIN".as_ptr(),
        control: ptr::null(),
        source: c"DMIC".as_ptr(),
    },
];

static mut es9356_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: unsafe { SND_JACK_HEADPHONE },
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: unsafe { SND_JACK_MICROPHONE },
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_es9356_spk_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    (*card).components = devm_kasprintf(
        (*card).dev,
        GFP_KERNEL,
        c"%s spk:es9356-spk".as_ptr(),
        (*card).components,
    );
    if (*card).components.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_dapm_add_routes(dapm, es9356_spk_map.as_ptr(), es9356_spk_map.len() as c_int);
    if ret != 0 {
        dev_err((*card).dev, c"es9356 map addition failed: %d\n".as_ptr(), ret);
    }

    ret
}
// EXPORT_SYMBOL_NS(asoc_sdw_es9356_spk_rtd_init, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_es9356_dmic_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    _dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    (*card).components = devm_kasprintf(
        (*card).dev,
        GFP_KERNEL,
        c"%s mic:es9356-dmic".as_ptr(),
        (*card).components,
    );
    if (*card).components.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_dapm_add_routes(dapm, es9356_dmic_map.as_ptr(), es9356_dmic_map.len() as c_int);
    if ret != 0 {
        dev_err((*card).dev, c"es9356 map addition failed: %d\n".as_ptr(), ret);
    }

    ret
}
// EXPORT_SYMBOL_NS(asoc_sdw_es9356_dmic_rtd_init, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_es9356_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let ctx: *mut asoc_sdw_mc_private = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let component: *mut snd_soc_component;
    let jack: *mut snd_soc_jack;
    let mut ret: c_int;

    component = (*dai).component;
    (*card).components = devm_kasprintf(
        (*card).dev,
        GFP_KERNEL,
        c"%s hs:es9356".as_ptr(),
        (*card).components,
    );
    if (*card).components.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_dapm_add_routes(dapm, es9356_map.as_ptr(), es9356_map.len() as c_int);

    if ret != 0 {
        dev_err((*card).dev, c"es9356 map addition failed: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"Headset Jack".as_ptr(),
        SND_JACK_HEADSET
            | SND_JACK_BTN_0
            | SND_JACK_BTN_1
            | SND_JACK_BTN_2
            | SND_JACK_BTN_3
            | SND_JACK_BTN_4,
        &mut (*ctx).sdw_headset,
        es9356_jack_pins.as_mut_ptr(),
        es9356_jack_pins.len() as c_int,
    );
    if ret != 0 {
        dev_err(
            (*(*rtd).card).dev,
            c"Headset Jack creation failed: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    jack = &mut (*ctx).sdw_headset;

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_NEXTSONG);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_4, KEY_PREVIOUSSONG);

    ret = snd_soc_component_set_jack(component, jack, ptr::null_mut());

    if ret != 0 {
        dev_err(
            (*(*rtd).card).dev,
            c"Headset Jack call-back failed: %d\n".as_ptr(),
            ret,
        );
    }

    ret
}
// EXPORT_SYMBOL_NS(asoc_sdw_es9356_rtd_init, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_es9356_exit(
    card: *mut snd_soc_card,
    _dai_link: *mut snd_soc_dai_link,
) -> c_int {
    let ctx: *mut asoc_sdw_mc_private = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;

    if (*ctx).headset_codec_dev.is_null() {
        return 0;
    }

    device_remove_software_node((*ctx).headset_codec_dev);
    put_device((*ctx).headset_codec_dev);

    0
}
// EXPORT_SYMBOL_NS(asoc_sdw_es9356_exit, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_es9356_init(
    card: *mut snd_soc_card,
    dai_links: *mut snd_soc_dai_link,
    _info: *mut asoc_sdw_codec_info,
    playback: bool,
) -> c_int {
    let ctx: *mut asoc_sdw_mc_private = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let sdw_dev: *mut device;
    let ret: c_int;

    /*
     * headset should be initialized once.
     * Do it with dai link for playback.
     */
    if !playback {
        return 0;
    }

    sdw_dev = bus_find_device_by_name(
        &sdw_bus_type as *const c_void,
        ptr::null_mut(),
        (*(*dai_links).codecs.add(0)).name,
    );
    if sdw_dev.is_null() {
        return -EPROBE_DEFER;
    }

    ret = es9356_add_codec_device_props(sdw_dev, (*ctx).mc_quirk);
    if ret < 0 {
        put_device(sdw_dev);
        return ret;
    }
    (*ctx).headset_codec_dev = sdw_dev;

    0
}
// EXPORT_SYMBOL_NS(asoc_sdw_es9356_init, "SND_SOC_SDW_UTILS");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn asoc_sdw_es9356_amp_init(
    _card: *mut snd_soc_card,
    _dai_links: *mut snd_soc_dai_link,
    info: *mut asoc_sdw_codec_info,
    playback: bool,
) -> c_int {
    if !playback {
        return 0;
    }

    (*info).amp_num += 1;

    0
}
// EXPORT_SYMBOL_NS(asoc_sdw_es9356_amp_init, "SND_SOC_SDW_UTILS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
