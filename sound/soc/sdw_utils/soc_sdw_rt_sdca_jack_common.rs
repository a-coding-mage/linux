// SPDX-License-Identifier: GPL-2.0-only
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2020 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.

/*
 *  soc_sdw_rt711_sdca - Helpers to handle RT711-SDCA from generic machine driver
 */

// Dependencies from Linux, SoundWire, and ASoC headers are declared externally.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::zeroed;
use core::ptr::{null, null_mut};

const SOC_SDW_MAX_NO_PROPS: usize = 8;
const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EPROBE_DEFER: c_int = 517;

const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;

const KEY_PLAYPAUSE: c_int = 164;
const KEY_VOICECOMMAND: c_int = 246;
const KEY_VOLUMEUP: c_int = 115;
const KEY_VOLUMEDOWN: c_int = 114;

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
pub struct property_entry {
    _private: [usize; 0],
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub codecs: *mut snd_soc_dai_link_component,
}

#[repr(C)]
pub struct asoc_sdw_mc_private {
    pub sdw_headset: snd_soc_jack,
    pub headset_codec_dev: *mut device,
    pub mc_quirk: c_ulong,
}

#[repr(C)]
pub struct asoc_sdw_codec_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bus_type {
    _private: [u8; 0],
}

unsafe extern "C" {
    static sdw_bus_type: bus_type;

    fn SOC_SDW_JACK_JDSRC(quirk: c_ulong) -> c_int;
    fn PROPERTY_ENTRY_U32(name: *const c_char, value: c_int) -> property_entry;
    fn fwnode_create_software_node(
        properties: *const property_entry,
        parent: *const fwnode_handle,
    ) -> *mut fwnode_handle;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn device_add_software_node(dev: *mut device, snode: *mut software_node) -> c_int;
    fn to_software_node(fwnode: *mut fwnode_handle) -> *mut software_node;
    fn fwnode_handle_put(fwnode: *mut fwnode_handle);
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut asoc_sdw_mc_private;
    fn devm_kasprintf(dev: *mut device, gfp: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
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
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn device_remove_software_node(dev: *mut device);
    fn put_device(dev: *mut device);
    fn bus_find_device_by_name(
        bus: *const bus_type,
        start: *mut device,
        name: *const c_char,
    ) -> *mut device;
}

/*
 * Note this MUST be called before snd_soc_register_card(), so that the props
 * are in place before the codec component driver's probe function parses them.
 */
unsafe fn rt_sdca_jack_add_codec_device_props(
    sdw_dev: *mut device,
    quirk: c_ulong,
) -> c_int {
    let mut props: [property_entry; SOC_SDW_MAX_NO_PROPS] = zeroed();
    let fwnode: *mut fwnode_handle;
    let ret: c_int;

    if SOC_SDW_JACK_JDSRC(quirk) == 0 {
        return 0;
    }

    props[0] = PROPERTY_ENTRY_U32(
        b"realtek,jd-src\0".as_ptr() as *const c_char,
        SOC_SDW_JACK_JDSRC(quirk),
    );

    fwnode = fwnode_create_software_node(props.as_ptr(), null());
    if IS_ERR(fwnode as *const c_void) {
        return PTR_ERR(fwnode as *const c_void);
    }

    ret = device_add_software_node(sdw_dev, to_software_node(fwnode));

    fwnode_handle_put(fwnode);

    ret
}

static RT711_SDCA_MAP: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"Headphone\0".as_ptr() as *const c_char,
        control: null(),
        source: b"rt711 HP\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"rt711 MIC2\0".as_ptr() as *const c_char,
        control: null(),
        source: b"Headset Mic\0".as_ptr() as *const c_char,
    },
];

static RT712_SDCA_MAP: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"Headphone\0".as_ptr() as *const c_char,
        control: null(),
        source: b"rt712 HP\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"rt712 MIC2\0".as_ptr() as *const c_char,
        control: null(),
        source: b"Headset Mic\0".as_ptr() as *const c_char,
    },
];

static RT713_SDCA_MAP: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"Headphone\0".as_ptr() as *const c_char,
        control: null(),
        source: b"rt713 HP\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"rt713 MIC2\0".as_ptr() as *const c_char,
        control: null(),
        source: b"Headset Mic\0".as_ptr() as *const c_char,
    },
];

static RT721_SDCA_MAP: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"Headphone\0".as_ptr() as *const c_char,
        control: null(),
        source: b"rt721 HP\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"rt721 MIC2\0".as_ptr() as *const c_char,
        control: null(),
        source: b"Headset Mic\0".as_ptr() as *const c_char,
    },
];

static RT722_SDCA_MAP: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"Headphone\0".as_ptr() as *const c_char,
        control: null(),
        source: b"rt722 HP\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"rt722 MIC2\0".as_ptr() as *const c_char,
        control: null(),
        source: b"Headset Mic\0".as_ptr() as *const c_char,
    },
];

static mut RT_SDCA_JACK_PINS: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

/*
 * The sdca suffix is required for rt711 since there are two generations of the same chip.
 * RT713 is an SDCA device but the sdca suffix is required for backwards-compatibility with
 * previous UCM definitions.
 */
static NEED_SDCA_SUFFIX: [*const c_char; 2] = [
    b"rt711\0".as_ptr() as *const c_char,
    b"rt713\0".as_ptr() as *const c_char,
];

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rt_sdca_jack_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let ctx: *mut asoc_sdw_mc_private = snd_soc_card_get_drvdata(card);
    let component: *mut snd_soc_component;
    let jack: *mut snd_soc_jack;
    let mut ret: c_int;
    let mut i: usize;

    component = (*dai).component;
    (*card).components = devm_kasprintf(
        (*card).dev,
        GFP_KERNEL,
        b"%s hs:%s\0".as_ptr() as *const c_char,
        (*card).components,
        (*component).name_prefix,
    );
    if (*card).components.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < NEED_SDCA_SUFFIX.len() {
        if !strstr((*component).name_prefix, NEED_SDCA_SUFFIX[i]).is_null() {
            /* Add -sdca suffix for existing UCMs */
            (*card).components = devm_kasprintf(
                (*card).dev,
                GFP_KERNEL,
                b"%s-sdca\0".as_ptr() as *const c_char,
                (*card).components,
            );
            if (*card).components.is_null() {
                return -ENOMEM;
            }
            break;
        }
        i += 1;
    }

    if !strstr((*component).name_prefix, b"rt711\0".as_ptr() as *const c_char).is_null() {
        ret = snd_soc_dapm_add_routes(
            dapm,
            RT711_SDCA_MAP.as_ptr(),
            RT711_SDCA_MAP.len() as c_int,
        );
    } else if !strstr((*component).name_prefix, b"rt712\0".as_ptr() as *const c_char).is_null() {
        ret = snd_soc_dapm_add_routes(
            dapm,
            RT712_SDCA_MAP.as_ptr(),
            RT712_SDCA_MAP.len() as c_int,
        );
    } else if !strstr((*component).name_prefix, b"rt713\0".as_ptr() as *const c_char).is_null() {
        ret = snd_soc_dapm_add_routes(
            dapm,
            RT713_SDCA_MAP.as_ptr(),
            RT713_SDCA_MAP.len() as c_int,
        );
    } else if !strstr((*component).name_prefix, b"rt721\0".as_ptr() as *const c_char).is_null() {
        ret = snd_soc_dapm_add_routes(
            dapm,
            RT721_SDCA_MAP.as_ptr(),
            RT721_SDCA_MAP.len() as c_int,
        );
    } else if !strstr((*component).name_prefix, b"rt722\0".as_ptr() as *const c_char).is_null() {
        ret = snd_soc_dapm_add_routes(
            dapm,
            RT722_SDCA_MAP.as_ptr(),
            RT722_SDCA_MAP.len() as c_int,
        );
    } else {
        dev_err(
            (*card).dev,
            b"%s is not supported\n\0".as_ptr() as *const c_char,
            (*component).name_prefix,
        );
        return -EINVAL;
    }

    if ret != 0 {
        dev_err(
            (*card).dev,
            b"rt sdca jack map addition failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        &mut (*ctx).sdw_headset,
        RT_SDCA_JACK_PINS.as_mut_ptr(),
        RT_SDCA_JACK_PINS.len() as c_int,
    );
    if ret != 0 {
        dev_err(
            (*(*rtd).card).dev,
            b"Headset Jack creation failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    jack = &mut (*ctx).sdw_headset;

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

    ret = snd_soc_component_set_jack(component, jack, null_mut());

    if ret != 0 {
        dev_err(
            (*(*rtd).card).dev,
            b"Headset Jack call-back failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    ret
}

// EXPORT_SYMBOL_NS(asoc_sdw_rt_sdca_jack_rtd_init, "SND_SOC_SDW_UTILS");

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rt_sdca_jack_exit(
    card: *mut snd_soc_card,
    _dai_link: *mut snd_soc_dai_link,
) -> c_int {
    let ctx: *mut asoc_sdw_mc_private = snd_soc_card_get_drvdata(card);

    if (*ctx).headset_codec_dev.is_null() {
        return 0;
    }

    if SOC_SDW_JACK_JDSRC((*ctx).mc_quirk) == 0 {
        return 0;
    }

    device_remove_software_node((*ctx).headset_codec_dev);
    put_device((*ctx).headset_codec_dev);
    (*ctx).headset_codec_dev = null_mut();

    0
}

// EXPORT_SYMBOL_NS(asoc_sdw_rt_sdca_jack_exit, "SND_SOC_SDW_UTILS");

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rt_sdca_jack_init(
    card: *mut snd_soc_card,
    dai_links: *mut snd_soc_dai_link,
    _info: *mut asoc_sdw_codec_info,
    _playback: bool,
) -> c_int {
    let ctx: *mut asoc_sdw_mc_private = snd_soc_card_get_drvdata(card);
    let sdw_dev: *mut device;
    let ret: c_int;

    /*
     * Jack detection should be only initialized once for headsets since
     * the playback/capture is sharing the same jack
     */
    if !(*ctx).headset_codec_dev.is_null() {
        return 0;
    }

    sdw_dev = bus_find_device_by_name(
        &sdw_bus_type,
        null_mut(),
        (*(*dai_links).codecs.add(0)).name,
    );
    if sdw_dev.is_null() {
        return -EPROBE_DEFER;
    }

    ret = rt_sdca_jack_add_codec_device_props(sdw_dev, (*ctx).mc_quirk);
    if ret < 0 {
        put_device(sdw_dev);
        return ret;
    }
    (*ctx).headset_codec_dev = sdw_dev;

    0
}

// EXPORT_SYMBOL_NS(asoc_sdw_rt_sdca_jack_init, "SND_SOC_SDW_UTILS");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
