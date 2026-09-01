// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// Translated from:
// #include <linux/module.h>
// #include <linux/of_platform.h>
// #include <sound/soc.h>
// #include "meson-card.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const GFP_KERNEL: c_uint = 0;
const __GFP_ZERO: c_uint = 0;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_CLOCK_OUT: c_int = 1;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFP: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFC: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct device_node {
    pub full_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *mut c_char,
    pub stream_name: *mut c_char,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub cpus: *mut snd_soc_dai_link_component,
    pub dynamic: c_uint,
    pub dpcm_merged_format: c_uint,
    pub dpcm_merged_chan: c_uint,
    pub dpcm_merged_rate: c_uint,
    pub playback_only: c_uint,
    pub capture_only: c_uint,
}

#[repr(C)]
pub struct snd_soc_aux_dev_dlc {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_aux_dev {
    pub dlc: snd_soc_aux_dev_dlc,
}

#[repr(C)]
pub struct snd_soc_card {
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub driver_name: *const c_char,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_uint,
    pub aux_dev: *mut snd_soc_aux_dev,
    pub num_aux_devs: c_uint,
}

#[repr(C)]
pub struct meson_card_match_data {
    pub add_link: Option<
        unsafe extern "C" fn(
            card: *mut snd_soc_card,
            np: *mut device_node,
            i: *mut c_int,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct meson_card {
    pub card: snd_soc_card,
    pub link_data: *mut *mut c_void,
    pub match_data: *const meson_card_match_data,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn krealloc(ptr: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_soc_of_get_dlc(
        node: *mut device_node,
        args: *mut c_void,
        dlc: *mut snd_soc_dai_link_component,
        index: c_int,
    ) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_kasprintf(
        dev: *mut device,
        flags: c_uint,
        fmt: *const c_char,
        ...
    ) -> *mut c_char;
    fn snd_soc_daifmt_parse_format(node: *mut device_node, prefix: *mut c_void) -> c_uint;
    fn snd_soc_daifmt_parse_clock_provider_as_phandle(
        node: *mut device_node,
        prefix: *mut c_void,
        bitclkmaster: *mut *mut device_node,
        framemaster: *mut *mut device_node,
    );
    fn of_node_put(node: *mut device_node);
    fn of_get_child_count(node: *mut device_node) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn of_property_present(node: *mut device_node, propname: *const c_char) -> bool;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_simple_widgets(
        card: *mut snd_soc_card,
        propname: *const c_char,
    ) -> c_int;
    fn snd_soc_of_parse_aux_devs(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;

    // C iterator macros from Linux headers / meson-card.h.
    fn for_each_rtd_codec_dais_next(
        rtd: *mut snd_soc_pcm_runtime,
        i: *mut c_int,
    ) -> *mut snd_soc_dai;
    fn for_each_child_of_node_scoped_next(
        node: *mut device_node,
        cursor: *mut *mut device_node,
    ) -> *mut device_node;
    fn for_each_card_prelinks_next(
        card: *mut snd_soc_card,
        i: *mut c_int,
    ) -> *mut snd_soc_dai_link;
    fn for_each_link_codecs_next(
        link: *mut snd_soc_dai_link,
        j: *mut c_int,
    ) -> *mut snd_soc_dai_link_component;
    fn for_each_card_pre_auxs_next(
        card: *mut snd_soc_card,
        i: *mut c_int,
    ) -> *mut snd_soc_aux_dev;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn meson_card_i2s_set_sysclk(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    mclk_fs: c_uint,
) -> c_int {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let mut codec_dai: *mut snd_soc_dai;
    let mclk: c_uint;
    let mut ret: c_int;
    let mut i: c_int;

    if mclk_fs == 0 {
        return 0;
    }

    mclk = unsafe { params_rate(params) }.wrapping_mul(mclk_fs);

    i = 0;
    loop {
        codec_dai = unsafe { for_each_rtd_codec_dais_next(rtd, &mut i) };
        if codec_dai.is_null() {
            break;
        }
        ret = unsafe { snd_soc_dai_set_sysclk(codec_dai, 0, mclk, SND_SOC_CLOCK_IN) };
        if ret != 0 && ret != -ENOTSUPP {
            return ret;
        }
    }

    ret = unsafe {
        snd_soc_dai_set_sysclk(
            snd_soc_rtd_to_cpu(rtd, 0),
            0,
            mclk,
            SND_SOC_CLOCK_OUT,
        )
    };
    if ret != 0 && ret != -ENOTSUPP {
        return ret;
    }

    0
}

// EXPORT_SYMBOL_GPL(meson_card_i2s_set_sysclk);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn meson_card_reallocate_links(
    card: *mut snd_soc_card,
    num_links: c_uint,
) -> c_int {
    let priv_ = unsafe { snd_soc_card_get_drvdata(card) as *mut meson_card };
    let mut links: *mut snd_soc_dai_link;
    let ldata: *mut *mut c_void;

    links = unsafe {
        krealloc(
            (*priv_).card.dai_link as *mut c_void,
            (num_links as usize).wrapping_mul(size_of::<snd_soc_dai_link>()),
            GFP_KERNEL | __GFP_ZERO,
        ) as *mut snd_soc_dai_link
    };
    if links.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*priv_).card.dai_link = links;
        (*priv_).card.num_links = num_links;
    }

    ldata = unsafe {
        krealloc(
            (*priv_).link_data as *mut c_void,
            (num_links as usize).wrapping_mul(size_of::<*mut c_void>()),
            GFP_KERNEL | __GFP_ZERO,
        ) as *mut *mut c_void
    };
    /* meson_card_clean_references() will free the links on this error path */
    if ldata.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*priv_).link_data = ldata;
    }
    0
}

// EXPORT_SYMBOL_GPL(meson_card_reallocate_links);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn meson_card_parse_dai(
    card: *mut snd_soc_card,
    node: *mut device_node,
    dlc: *mut snd_soc_dai_link_component,
) -> c_int {
    let dev = unsafe { (*card).dev };
    let ret: c_int;

    if dlc.is_null() || node.is_null() {
        return -EINVAL;
    }

    ret = unsafe { snd_soc_of_get_dlc(node, ptr::null_mut(), dlc, 0) };
    if ret != 0 {
        return unsafe { dev_err_probe(dev, ret, c"can't parse dai\n".as_ptr()) };
    }

    ret
}

// EXPORT_SYMBOL_GPL(meson_card_parse_dai);

unsafe extern "C" fn meson_card_set_link_name(
    card: *mut snd_soc_card,
    link: *mut snd_soc_dai_link,
    node: *mut device_node,
    prefix: *const c_char,
) -> c_int {
    let dev = unsafe { (*card).dev };
    let name = unsafe {
        devm_kasprintf(
            dev,
            GFP_KERNEL,
            c"%s.%s".as_ptr(),
            prefix,
            (*node).full_name,
        )
    };
    if name.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*link).name = name;
        (*link).stream_name = name;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn meson_card_parse_daifmt(
    node: *mut device_node,
    cpu_node: *mut device_node,
) -> c_uint {
    let mut bitclkmaster: *mut device_node = ptr::null_mut();
    let mut framemaster: *mut device_node = ptr::null_mut();
    let mut daifmt: c_uint;

    daifmt = unsafe { snd_soc_daifmt_parse_format(node, ptr::null_mut()) };

    unsafe {
        snd_soc_daifmt_parse_clock_provider_as_phandle(
            node,
            ptr::null_mut(),
            &mut bitclkmaster,
            &mut framemaster,
        );
    }

    /* If no master is provided, default to cpu master */
    if bitclkmaster.is_null() || bitclkmaster == cpu_node {
        daifmt |= if framemaster.is_null() || framemaster == cpu_node {
            SND_SOC_DAIFMT_CBC_CFC
        } else {
            SND_SOC_DAIFMT_CBC_CFP
        };
    } else {
        daifmt |= if framemaster.is_null() || framemaster == cpu_node {
            SND_SOC_DAIFMT_CBP_CFC
        } else {
            SND_SOC_DAIFMT_CBP_CFP
        };
    }

    unsafe {
        of_node_put(bitclkmaster);
        of_node_put(framemaster);
    }

    daifmt
}

// EXPORT_SYMBOL_GPL(meson_card_parse_daifmt);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn meson_card_set_be_link(
    card: *mut snd_soc_card,
    link: *mut snd_soc_dai_link,
    node: *mut device_node,
) -> c_int {
    let mut codec: *mut snd_soc_dai_link_component;
    let dev = unsafe { (*card).dev };
    let mut ret: c_int;
    let num_codecs: c_int;

    num_codecs = unsafe { of_get_child_count(node) };
    if num_codecs == 0 {
        unsafe {
            dev_err(
                dev,
                c"be link %s has no codec\n".as_ptr(),
                (*node).full_name,
            );
        }
        return -EINVAL;
    }

    codec = unsafe {
        devm_kcalloc(
            dev,
            num_codecs as usize,
            size_of::<snd_soc_dai_link_component>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component
    };
    if codec.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*link).codecs = codec;
        (*link).num_codecs = num_codecs as c_uint;
    }

    let mut np_cursor: *mut device_node = ptr::null_mut();
    loop {
        let np = unsafe { for_each_child_of_node_scoped_next(node, &mut np_cursor) };
        if np.is_null() {
            break;
        }
        ret = unsafe { meson_card_parse_dai(card, np, codec) };
        if ret != 0 {
            return ret;
        }

        codec = unsafe { codec.add(1) };
    }

    ret = unsafe { meson_card_set_link_name(card, link, node, c"be".as_ptr()) };
    if ret != 0 {
        unsafe {
            dev_err(
                dev,
                c"error setting %pOFn link name\n".as_ptr(),
                node,
            );
        }
    }

    ret
}

// EXPORT_SYMBOL_GPL(meson_card_set_be_link);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn meson_card_set_fe_link(
    _card: *mut snd_soc_card,
    link: *mut snd_soc_dai_link,
    node: *mut device_node,
    is_playback: bool,
) -> c_int {
    unsafe {
        (*link).codecs = &raw mut snd_soc_dummy_dlc;
        (*link).num_codecs = 1;

        (*link).dynamic = 1;
        (*link).dpcm_merged_format = 1;
        (*link).dpcm_merged_chan = 1;
        (*link).dpcm_merged_rate = 1;

        if is_playback {
            (*link).playback_only = 1;
        } else {
            (*link).capture_only = 1;
        }

        meson_card_set_link_name(_card, link, node, c"fe".as_ptr())
    }
}

// EXPORT_SYMBOL_GPL(meson_card_set_fe_link);

unsafe extern "C" fn meson_card_add_links(card: *mut snd_soc_card) -> c_int {
    let priv_ = unsafe { snd_soc_card_get_drvdata(card) as *mut meson_card };
    let dev = unsafe { (*card).dev };
    let node = unsafe { (*dev).of_node };
    let num: c_int;
    let mut i: c_int;
    let mut ret: c_int;

    num = unsafe { of_get_child_count(node) };
    if num == 0 {
        unsafe {
            dev_err(dev, c"card has no links\n".as_ptr());
        }
        return -EINVAL;
    }

    ret = unsafe { meson_card_reallocate_links(card, num as c_uint) };
    if ret != 0 {
        return ret;
    }

    i = 0;
    let mut np_cursor: *mut device_node = ptr::null_mut();
    loop {
        let np = unsafe { for_each_child_of_node_scoped_next(node, &mut np_cursor) };
        if np.is_null() {
            break;
        }
        ret = unsafe { ((*(*priv_).match_data).add_link.unwrap())(card, np, &mut i) };
        if ret != 0 {
            return ret;
        }

        i += 1;
    }

    0
}

unsafe extern "C" fn meson_card_parse_of_optional(
    card: *mut snd_soc_card,
    propname: *const c_char,
    func: Option<unsafe extern "C" fn(c: *mut snd_soc_card, p: *const c_char) -> c_int>,
) -> c_int {
    let dev = unsafe { (*card).dev };

    /* If property is not provided, don't fail ... */
    if unsafe { !of_property_present((*dev).of_node, propname) } {
        return 0;
    }

    /* ... but do fail if it is provided and the parsing fails */
    unsafe { func.unwrap()(card, propname) }
}

unsafe extern "C" fn meson_card_clean_references(priv_: *mut meson_card) {
    let card = unsafe { &raw mut (*priv_).card };
    let mut link: *mut snd_soc_dai_link;
    let mut codec: *mut snd_soc_dai_link_component;
    let mut aux: *mut snd_soc_aux_dev;
    let mut i: c_int;
    let mut j: c_int;

    if unsafe { !(*card).dai_link.is_null() } {
        i = 0;
        loop {
            link = unsafe { for_each_card_prelinks_next(card, &mut i) };
            if link.is_null() {
                break;
            }
            unsafe {
                if !(*link).cpus.is_null() {
                    of_node_put((*(*link).cpus).of_node);
                }
            }
            j = 0;
            loop {
                codec = unsafe { for_each_link_codecs_next(link, &mut j) };
                if codec.is_null() {
                    break;
                }
                unsafe {
                    of_node_put((*codec).of_node);
                }
            }
        }
    }

    if unsafe { !(*card).aux_dev.is_null() } {
        i = 0;
        loop {
            aux = unsafe { for_each_card_pre_auxs_next(card, &mut i) };
            if aux.is_null() {
                break;
            }
            unsafe {
                of_node_put((*aux).dlc.of_node);
            }
        }
    }

    unsafe {
        kfree((*card).dai_link as *mut c_void);
        kfree((*priv_).link_data as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn meson_card_probe(pdev: *mut platform_device) -> c_int {
    let data: *const meson_card_match_data;
    let dev = unsafe { &raw mut (*pdev).dev };
    let priv_: *mut meson_card;
    let mut ret: c_int;

    data = unsafe { of_device_get_match_data(dev) as *const meson_card_match_data };
    if data.is_null() {
        unsafe {
            dev_err(dev, c"failed to match device\n".as_ptr());
        }
        return -ENODEV;
    }

    priv_ = unsafe { devm_kzalloc(dev, size_of::<meson_card>(), GFP_KERNEL) as *mut meson_card };
    if priv_.is_null() {
        return -ENOMEM;
    }

    unsafe {
        platform_set_drvdata(pdev, priv_ as *mut c_void);
        snd_soc_card_set_drvdata(&raw mut (*priv_).card, priv_ as *mut c_void);

        (*priv_).card.owner = THIS_MODULE;
        (*priv_).card.dev = dev;
        (*priv_).card.driver_name = (*(*dev).driver).name;
        (*priv_).match_data = data;
    }

    ret = unsafe { snd_soc_of_parse_card_name(&raw mut (*priv_).card, c"model".as_ptr()) };
    if ret < 0 {
        return ret;
    }

    ret = unsafe {
        meson_card_parse_of_optional(
            &raw mut (*priv_).card,
            c"audio-routing".as_ptr(),
            Some(snd_soc_of_parse_audio_routing),
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(dev, c"error while parsing routing\n".as_ptr());
        }
        return ret;
    }

    ret = unsafe {
        meson_card_parse_of_optional(
            &raw mut (*priv_).card,
            c"audio-widgets".as_ptr(),
            Some(snd_soc_of_parse_audio_simple_widgets),
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(dev, c"error while parsing widgets\n".as_ptr());
        }
        return ret;
    }

    ret = unsafe { meson_card_add_links(&raw mut (*priv_).card) };
    if ret != 0 {
        unsafe {
            meson_card_clean_references(priv_);
        }
        return ret;
    }

    ret = unsafe { snd_soc_of_parse_aux_devs(&raw mut (*priv_).card, c"audio-aux-devs".as_ptr()) };
    if ret != 0 {
        unsafe {
            meson_card_clean_references(priv_);
        }
        return ret;
    }

    ret = unsafe { devm_snd_soc_register_card(dev, &raw mut (*priv_).card) };
    if ret != 0 {
        unsafe {
            meson_card_clean_references(priv_);
        }
        return ret;
    }

    0
}

// EXPORT_SYMBOL_GPL(meson_card_probe);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn meson_card_remove(pdev: *mut platform_device) {
    let priv_ = unsafe { platform_get_drvdata(pdev) as *mut meson_card };

    unsafe {
        meson_card_clean_references(priv_);
    }
}

// EXPORT_SYMBOL_GPL(meson_card_remove);

// MODULE_DESCRIPTION("Amlogic Sound Card Utils");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
