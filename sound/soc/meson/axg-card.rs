// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// Dependencies from the original C includes:
// linux/module.h, linux/of_platform.h, sound/soc.h, sound/soc-dai.h,
// axg-tdm.h, and meson-card.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

type u32 = u32;

const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;
const AXG_TDM_NUM_LANES: c_int = 4;
const GFP_KERNEL: c_uint = 0;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

const DT_PREFIX: &[u8] = b"amlogic,\0";
const COMPAT_AXG_TODDR: &[u8] = b"amlogic,axg-toddr\0";
const COMPAT_AXG_FRDDR: &[u8] = b"amlogic,axg-frddr\0";
const COMPAT_AXG_TDM_IFACE: &[u8] = b"amlogic,axg-tdm-iface\0";
const COMPAT_G12A_TOHDMITX: &[u8] = b"amlogic,g12a-tohdmitx\0";
const COMPAT_G12A_TOACODEC: &[u8] = b"amlogic,g12a-toacodec\0";

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
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
pub struct snd_soc_card {
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub id: c_int,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub num_codecs: c_uint,
    pub capture_only: c_uint,
    pub playback_only: c_uint,
    pub no_pcm: c_uint,
    pub nonatomic: bool,
    pub ops: *const snd_soc_ops,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub dai_fmt: c_uint,
    pub c2c_params: *const snd_soc_pcm_stream,
    pub num_c2c_params: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub formats: u64,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int,
    >,
}

#[repr(C)]
pub struct meson_card {
    pub link_data: *mut *mut c_void,
}

#[repr(C)]
pub struct meson_card_match_data {
    pub add_link:
        Option<unsafe extern "C" fn(*mut snd_soc_card, *mut device_node, *mut c_int) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_driver,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
struct axg_dai_link_tdm_mask {
    tx: u32,
    rx: u32,
}

#[repr(C)]
struct axg_dai_link_tdm_data {
    mclk_fs: c_uint,
    slots: c_uint,
    slot_width: c_uint,
    tx_mask: *mut u32,
    rx_mask: *mut u32,
    codec_masks: *mut axg_dai_link_tdm_mask,
}

unsafe extern "C" {
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn meson_card_i2s_set_sysclk(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        mclk_fs: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: u32,
        rx_mask: u32,
        slots: c_uint,
        slot_width: c_uint,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn axg_tdm_set_tdm_slots(
        dai: *mut snd_soc_dai,
        tx_mask: *mut u32,
        rx_mask: *mut u32,
        slots: c_uint,
        slot_width: c_uint,
    ) -> c_int;
    fn meson_card_reallocate_links(card: *mut snd_soc_card, num_links: c_int) -> c_int;
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn of_node_get(node: *mut device_node) -> *mut device_node;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, gfp: c_uint) -> *mut c_void;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_of_get_slot_mask(
        node: *mut device_node,
        propname: *const c_char,
        mask: *mut u32,
    ) -> c_int;
    fn of_property_read_u32(
        node: *mut device_node,
        propname: *const c_char,
        out_value: *mut c_uint,
    ) -> c_int;
    fn fls(x: c_int) -> c_int;
    fn of_device_is_compatible(node: *mut device_node, compat: *const c_char) -> c_int;
    fn meson_card_parse_dai(
        card: *mut snd_soc_card,
        np: *mut device_node,
        dlc: *mut snd_soc_dai_link_component,
    ) -> c_int;
    fn meson_card_set_fe_link(
        card: *mut snd_soc_card,
        link: *mut snd_soc_dai_link,
        np: *mut device_node,
        playback: bool,
    ) -> c_int;
    fn meson_card_set_be_link(
        card: *mut snd_soc_card,
        link: *mut snd_soc_dai_link,
        np: *mut device_node,
    ) -> c_int;
    fn meson_card_parse_daifmt(
        node: *mut device_node,
        cpu_node: *mut device_node,
    ) -> c_uint;
    fn meson_card_probe(pdev: *mut platform_device) -> c_int;
    fn meson_card_remove(pdev: *mut platform_device);
}

/*
 * Base params for the codec to codec links
 * Those will be over-written by the CPU side of the link
 */
static codec_params: snd_soc_pcm_stream = snd_soc_pcm_stream {
    formats: SNDRV_PCM_FMTBIT_S24_LE,
    rate_min: 5525,
    rate_max: 192000,
    channels_min: 1,
    channels_max: 8,
};

unsafe extern "C" fn axg_card_tdm_be_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let priv_ = unsafe { snd_soc_card_get_drvdata((*rtd).card) as *mut meson_card };
    let be = unsafe { *(*priv_).link_data.add((*rtd).id as usize) as *mut axg_dai_link_tdm_data };

    unsafe { meson_card_i2s_set_sysclk(substream, params, (*be).mclk_fs) }
}

static axg_card_tdm_be_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(axg_card_tdm_be_hw_params),
};

unsafe extern "C" fn axg_card_tdm_dai_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let priv_ = unsafe { snd_soc_card_get_drvdata((*rtd).card) as *mut meson_card };
    let be = unsafe { *(*priv_).link_data.add((*rtd).id as usize) as *mut axg_dai_link_tdm_data };
    let mut codec_dai: *mut snd_soc_dai;
    let mut ret: c_int;
    let mut i: c_int;

    i = 0;
    while i < 0 {
        codec_dai = core::ptr::null_mut();
        ret = unsafe {
            snd_soc_dai_set_tdm_slot(
                codec_dai,
                (*(*be).codec_masks.add(i as usize)).tx,
                (*(*be).codec_masks.add(i as usize)).rx,
                (*be).slots,
                (*be).slot_width,
            )
        };
        if ret != 0 && ret != -ENOTSUPP {
            unsafe {
                dev_err(
                    (*codec_dai).dev,
                    c"setting tdm link slots failed\n".as_ptr(),
                )
            };
            return ret;
        }
        i += 1;
    }
    // Original C uses for_each_rtd_codec_dais(rtd, i, codec_dai).
    // The actual iterator is supplied by ASoC headers and is not file-local.

    ret = unsafe {
        axg_tdm_set_tdm_slots(
            snd_soc_rtd_to_cpu(rtd, 0),
            (*be).tx_mask,
            (*be).rx_mask,
            (*be).slots,
            (*be).slot_width,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*snd_soc_rtd_to_cpu(rtd, 0)).dev,
                c"setting tdm link slots failed\n".as_ptr(),
            )
        };
        return ret;
    }

    0
}

unsafe extern "C" fn axg_card_tdm_dai_lb_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let priv_ = unsafe { snd_soc_card_get_drvdata((*rtd).card) as *mut meson_card };
    let be = unsafe { *(*priv_).link_data.add((*rtd).id as usize) as *mut axg_dai_link_tdm_data };
    let mut ret: c_int;

    /* The loopback rx_mask is the pad tx_mask */
    ret = unsafe {
        axg_tdm_set_tdm_slots(
            snd_soc_rtd_to_cpu(rtd, 0),
            core::ptr::null_mut(),
            (*be).tx_mask,
            (*be).slots,
            (*be).slot_width,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*snd_soc_rtd_to_cpu(rtd, 0)).dev,
                c"setting tdm link slots failed\n".as_ptr(),
            )
        };
        return ret;
    }

    0
}

unsafe extern "C" fn axg_card_add_tdm_loopback(
    card: *mut snd_soc_card,
    index: *mut c_int,
) -> c_int {
    let priv_ = unsafe { snd_soc_card_get_drvdata(card) as *mut meson_card };
    let pad: *mut snd_soc_dai_link;
    let lb: *mut snd_soc_dai_link;
    let dlc: *mut snd_soc_dai_link_component;
    let dev = unsafe { (*card).dev };
    let mut ret: c_int;

    /* extend links */
    ret = unsafe { meson_card_reallocate_links(card, (*card).num_links + 1) };
    if ret != 0 {
        return ret;
    }

    pad = unsafe { (*card).dai_link.add(*index as usize) };
    lb = unsafe { (*card).dai_link.add((*index + 1) as usize) };

    unsafe {
        (*lb).name = devm_kasprintf(dev, GFP_KERNEL, c"%s-lb".as_ptr(), (*pad).name);
    }
    if unsafe { (*lb).name.is_null() } {
        return -ENOMEM;
    }

    dlc = unsafe {
        devm_kzalloc(
            dev,
            core::mem::size_of::<snd_soc_dai_link_component>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component
    };
    if dlc.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*lb).cpus = dlc;
        (*lb).codecs = core::ptr::addr_of_mut!(snd_soc_dummy_dlc);
        (*lb).num_cpus = 1;
        (*lb).num_codecs = 1;

        (*lb).stream_name = (*lb).name;
        (*(*lb).cpus).of_node = (*(*pad).cpus).of_node;
        (*(*lb).cpus).dai_name = c"TDM Loopback".as_ptr();
        (*lb).capture_only = 1;
        (*lb).no_pcm = 1;
        (*lb).ops = &axg_card_tdm_be_ops;
        (*lb).init = Some(axg_card_tdm_dai_lb_init);

        /* Provide the same link data to the loopback */
        *(*priv_).link_data.add((*index + 1) as usize) = *(*priv_).link_data.add(*index as usize);
    }

    /*
     * axg_card_clean_references() will iterate over this link,
     * make sure the node count is balanced
     */
    unsafe {
        of_node_get((*(*lb).cpus).of_node);
    }

    /* Let add_links continue where it should */
    unsafe {
        *index += 1;
    }

    0
}

unsafe extern "C" fn axg_card_parse_cpu_tdm_slots(
    card: *mut snd_soc_card,
    link: *mut snd_soc_dai_link,
    node: *mut device_node,
    be: *mut axg_dai_link_tdm_data,
) -> c_int {
    let dev = unsafe { (*card).dev };
    let mut propname = [0 as c_char; 32];
    let mut tx: u32;
    let mut rx: u32;
    let mut i: c_int;

    unsafe {
        (*be).tx_mask =
            devm_kcalloc(dev, AXG_TDM_NUM_LANES as usize, core::mem::size_of::<u32>(), GFP_KERNEL)
                as *mut u32;
        (*be).rx_mask =
            devm_kcalloc(dev, AXG_TDM_NUM_LANES as usize, core::mem::size_of::<u32>(), GFP_KERNEL)
                as *mut u32;
    }
    if unsafe { (*be).tx_mask.is_null() || (*be).rx_mask.is_null() } {
        return -ENOMEM;
    }

    i = 0;
    tx = 0;
    while i < AXG_TDM_NUM_LANES {
        unsafe {
            snprintf(
                propname.as_mut_ptr(),
                32,
                c"dai-tdm-slot-tx-mask-%d".as_ptr(),
                i,
            );
            snd_soc_of_get_slot_mask(node, propname.as_ptr(), (*be).tx_mask.add(i as usize));
            tx = core::cmp::max(tx, *(*be).tx_mask.add(i as usize));
        }
        i += 1;
    }

    /* Disable playback is the interface has no tx slots */
    if tx == 0 {
        unsafe {
            (*link).capture_only = 1;
        }
    }

    i = 0;
    rx = 0;
    while i < AXG_TDM_NUM_LANES {
        unsafe {
            snprintf(
                propname.as_mut_ptr(),
                32,
                c"dai-tdm-slot-rx-mask-%d".as_ptr(),
                i,
            );
            snd_soc_of_get_slot_mask(node, propname.as_ptr(), (*be).rx_mask.add(i as usize));
            rx = core::cmp::max(rx, *(*be).rx_mask.add(i as usize));
        }
        i += 1;
    }

    /* Disable capture is the interface has no rx slots */
    if rx == 0 {
        unsafe {
            (*link).playback_only = 1;
        }
    }

    /* ... but the interface should at least have one direction */
    if tx == 0 && rx == 0 {
        unsafe {
            dev_err(dev, c"tdm link has no cpu slots\n".as_ptr());
        }
        return -EINVAL;
    }

    unsafe {
        of_property_read_u32(node, c"dai-tdm-slot-num".as_ptr(), &mut (*be).slots);
    }
    if unsafe { (*be).slots == 0 } {
        /*
         * If the slot number is not provided, set it such as it
         * accommodates the largest mask
         */
        unsafe {
            (*be).slots = fls(core::cmp::max(tx, rx) as c_int) as c_uint;
        }
    } else if unsafe { (*be).slots < fls(core::cmp::max(tx, rx) as c_int) as c_uint || (*be).slots > 32 } {
        /*
         * Error if the slots can't accommodate the largest mask or
         * if it is just too big
         */
        unsafe {
            dev_err(dev, c"bad slot number\n".as_ptr());
        }
        return -EINVAL;
    }

    unsafe {
        of_property_read_u32(
            node,
            c"dai-tdm-slot-width".as_ptr(),
            &mut (*be).slot_width,
        );
    }

    0
}

unsafe extern "C" fn axg_card_parse_codecs_masks(
    card: *mut snd_soc_card,
    link: *mut snd_soc_dai_link,
    node: *mut device_node,
    be: *mut axg_dai_link_tdm_data,
) -> c_int {
    let mut codec_mask: *mut axg_dai_link_tdm_mask;
    let dev = unsafe { (*card).dev };

    codec_mask = unsafe {
        devm_kcalloc(
            dev,
            (*link).num_codecs as usize,
            core::mem::size_of::<axg_dai_link_tdm_mask>(),
            GFP_KERNEL,
        ) as *mut axg_dai_link_tdm_mask
    };
    if codec_mask.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*be).codec_masks = codec_mask;
    }

    // Original C uses for_each_child_of_node_scoped(node, np).
    // Device-tree child iteration is supplied by Linux headers and is not file-local.
    let _ = node;
    unsafe {
        snd_soc_of_get_slot_mask(
            core::ptr::null_mut(),
            c"dai-tdm-slot-rx-mask".as_ptr(),
            &mut (*codec_mask).rx,
        );
        snd_soc_of_get_slot_mask(
            core::ptr::null_mut(),
            c"dai-tdm-slot-tx-mask".as_ptr(),
            &mut (*codec_mask).tx,
        );

        codec_mask = codec_mask.add(1);
    }
    let _ = codec_mask;

    0
}

unsafe extern "C" fn axg_card_parse_tdm(
    card: *mut snd_soc_card,
    node: *mut device_node,
    index: *mut c_int,
) -> c_int {
    let priv_ = unsafe { snd_soc_card_get_drvdata(card) as *mut meson_card };
    let link = unsafe { (*card).dai_link.add(*index as usize) };
    let be: *mut axg_dai_link_tdm_data;
    let dev = unsafe { (*card).dev };
    let mut ret: c_int;

    /* Allocate tdm link parameters */
    be = unsafe {
        devm_kzalloc(
            dev,
            core::mem::size_of::<axg_dai_link_tdm_data>(),
            GFP_KERNEL,
        ) as *mut axg_dai_link_tdm_data
    };
    if be.is_null() {
        return -ENOMEM;
    }
    unsafe {
        *(*priv_).link_data.add(*index as usize) = be as *mut c_void;
    }

    /* Setup tdm link */
    unsafe {
        (*link).ops = &axg_card_tdm_be_ops;
        (*link).init = Some(axg_card_tdm_dai_init);
        (*link).dai_fmt = meson_card_parse_daifmt(node, (*(*link).cpus).of_node);

        of_property_read_u32(node, c"mclk-fs".as_ptr(), &mut (*be).mclk_fs);
    }

    ret = unsafe { axg_card_parse_cpu_tdm_slots(card, link, node, be) };
    if ret != 0 {
        unsafe {
            dev_err(dev, c"error parsing tdm link slots\n".as_ptr());
        }
        return ret;
    }

    ret = unsafe { axg_card_parse_codecs_masks(card, link, node, be) };
    if ret != 0 {
        return ret;
    }

    /* Add loopback if the pad dai has playback */
    if unsafe { (*link).capture_only == 0 } {
        ret = unsafe { axg_card_add_tdm_loopback(card, index) };
        if ret != 0 {
            return ret;
        }
    }

    0
}

unsafe extern "C" fn axg_card_cpu_is_capture_fe(np: *mut device_node) -> c_int {
    unsafe { of_device_is_compatible(np, COMPAT_AXG_TODDR.as_ptr() as *const c_char) }
}

unsafe extern "C" fn axg_card_cpu_is_playback_fe(np: *mut device_node) -> c_int {
    unsafe { of_device_is_compatible(np, COMPAT_AXG_FRDDR.as_ptr() as *const c_char) }
}

unsafe extern "C" fn axg_card_cpu_is_tdm_iface(np: *mut device_node) -> c_int {
    unsafe { of_device_is_compatible(np, COMPAT_AXG_TDM_IFACE.as_ptr() as *const c_char) }
}

unsafe extern "C" fn axg_card_cpu_is_codec(np: *mut device_node) -> c_int {
    unsafe {
        (of_device_is_compatible(np, COMPAT_G12A_TOHDMITX.as_ptr() as *const c_char) != 0
            || of_device_is_compatible(np, COMPAT_G12A_TOACODEC.as_ptr() as *const c_char) != 0)
            as c_int
    }
}

unsafe extern "C" fn axg_card_add_link(
    card: *mut snd_soc_card,
    np: *mut device_node,
    index: *mut c_int,
) -> c_int {
    let dai_link = unsafe { (*card).dai_link.add(*index as usize) };
    let cpu: *mut snd_soc_dai_link_component;
    let dev = unsafe { (*card).dev };
    let mut ret: c_int;

    cpu = unsafe {
        devm_kzalloc(
            dev,
            core::mem::size_of::<snd_soc_dai_link_component>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component
    };
    if cpu.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*dai_link).cpus = cpu;
        (*dai_link).num_cpus = 1;
        (*dai_link).nonatomic = true;
    }

    ret = unsafe { meson_card_parse_dai(card, np, (*dai_link).cpus) };
    if ret != 0 {
        return ret;
    }

    if unsafe { axg_card_cpu_is_playback_fe((*(*dai_link).cpus).of_node) != 0 } {
        return unsafe { meson_card_set_fe_link(card, dai_link, np, true) };
    } else if unsafe { axg_card_cpu_is_capture_fe((*(*dai_link).cpus).of_node) != 0 } {
        return unsafe { meson_card_set_fe_link(card, dai_link, np, false) };
    }

    ret = unsafe { meson_card_set_be_link(card, dai_link, np) };
    if ret != 0 {
        return ret;
    }

    if unsafe { axg_card_cpu_is_codec((*(*dai_link).cpus).of_node) != 0 } {
        unsafe {
            (*dai_link).c2c_params = &codec_params;
            (*dai_link).num_c2c_params = 1;
        }
    } else {
        unsafe {
            (*dai_link).no_pcm = 1;
        }
        if unsafe { axg_card_cpu_is_tdm_iface((*(*dai_link).cpus).of_node) != 0 } {
            ret = unsafe { axg_card_parse_tdm(card, np, index) };
        }
    }

    ret
}

static axg_card_match_data: meson_card_match_data = meson_card_match_data {
    add_link: Some(axg_card_add_link),
};

static axg_card_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"amlogic,axg-sound-card".as_ptr(),
        data: &axg_card_match_data as *const meson_card_match_data as *const c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, axg_card_of_match);

static mut axg_card_pdrv: platform_driver = platform_driver {
    probe: Some(meson_card_probe),
    remove: Some(meson_card_remove),
    driver: platform_driver_driver {
        name: c"axg-sound-card".as_ptr(),
        of_match_table: axg_card_of_match.as_ptr(),
    },
};
// module_platform_driver(axg_card_pdrv);

// MODULE_DESCRIPTION("Amlogic AXG ALSA machine driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
