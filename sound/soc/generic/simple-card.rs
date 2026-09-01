// SPDX-License-Identifier: GPL-2.0
//
// ASoC simple sound card support
//
// Copyright (C) 2012 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

// C dependencies:
// linux/cleanup.h, linux/clk.h, linux/device.h, linux/module.h, linux/of.h,
// linux/of_platform.h, linux/platform_device.h, linux/string.h,
// sound/simple_card_utils.h, sound/soc.h, sound/soc-dai.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const DPCM_SELECTABLE: usize = 1;

const DAI: *const c_char = b"sound-dai\0".as_ptr() as *const c_char;
const CELL: *const c_char = b"#sound-dai-cells\0".as_ptr() as *const c_char;
const PREFIX: *const c_char = b"simple-audio-card,\0".as_ptr() as *const c_char;

const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const GFP_KERNEL: c_uint = 0;
const SNDRV_MAX_LINKS: c_int = 32;
const SND_SOC_TRIGGER_ORDER_DEFAULT: snd_soc_trigger_order = 0;
const THIS_MODULE: *mut module = ptr::null_mut();

type bool_ = bool;
type uintptr_t = usize;
type snd_soc_trigger_order = c_int;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_phandle_args {
    pub np: *mut device_node,
    pub args_count: c_int,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
    pub dai_args: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct simple_util_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct simple_util_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct simple_dai_props {
    pub mclk_fs: c_uint,
    pub adata: simple_util_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_num {
    pub cpus: c_int,
    pub codecs: c_int,
    pub platforms: c_int,
}

#[repr(C)]
pub struct link_info {
    pub link: c_int,
    pub cpu: c_int,
    pub num: [link_num; SNDRV_MAX_LINKS as usize],
}

#[repr(C)]
pub struct simple_util_priv {
    pub hp_jack: c_void,
    pub mic_jack: c_void,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub dai_fmt: c_uint,
    pub playback_only: bool_,
    pub capture_only: bool_,
    pub trigger_start: snd_soc_trigger_order,
    pub trigger_stop: snd_soc_trigger_order,
    pub dynamic: c_int,
    pub dpcm_merged_format: c_int,
    pub no_pcm: c_int,
    pub be_hw_params_fixup: Option<unsafe extern "C" fn() -> c_int>,
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub ops: *const snd_soc_ops,
}

#[repr(C)]
pub struct snd_soc_card {
    pub owner: *mut module,
    pub dev: *mut device,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub driver_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn() -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn()>,
    pub hw_params: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn() -> c_int>,
}

type link_func = unsafe extern "C" fn(
    *mut simple_util_priv,
    *mut device_node,
    *mut device_node,
    *mut link_info,
    bool_,
) -> c_int;

extern "C" {
    static snd_soc_pm_ops: dev_pm_ops;

    fn simple_util_startup() -> c_int;
    fn simple_util_shutdown();
    fn simple_util_hw_params() -> c_int;
    fn simple_util_be_hw_params_fixup() -> c_int;
    fn simple_util_dai_init() -> c_int;
    fn simple_util_remove() -> c_int;

    fn simple_priv_to_dev(priv_: *mut simple_util_priv) -> *mut device;
    fn simple_priv_to_card(priv_: *mut simple_util_priv) -> *mut snd_soc_card;
    fn simple_priv_to_link(priv_: *mut simple_util_priv, link: c_int) -> *mut snd_soc_dai_link;
    fn simple_priv_to_props(priv_: *mut simple_util_priv, link: c_int) -> *mut simple_dai_props;
    fn simple_props_to_dai_cpu(props: *mut simple_dai_props, id: c_int) -> *mut simple_util_dai;
    fn simple_props_to_dai_codec(props: *mut simple_dai_props, id: c_int) -> *mut simple_util_dai;
    fn simple_props_to_codec_conf(props: *mut simple_dai_props, id: c_int) -> *mut snd_soc_codec_conf;

    fn snd_soc_ret(dev: *mut device, ret: c_int, fmt: *const c_char, ...) -> c_int;
    fn of_parse_phandle_with_args(
        np: *mut device_node,
        list_name: *const c_char,
        cells_name: *const c_char,
        index: c_int,
        out_args: *mut of_phandle_args,
    ) -> c_int;
    fn snd_soc_get_dai_via_args(args: *mut of_phandle_args) -> *mut snd_soc_dai;
    fn snd_soc_dai_name_get(dai: *mut snd_soc_dai) -> *const c_char;
    fn snd_soc_copy_dai_args(dev: *mut device, args: *mut of_phandle_args) -> *mut c_void;
    fn snd_soc_get_dlc(args: *mut of_phandle_args, dlc: *mut snd_soc_dai_link_component) -> c_int;
    fn devm_kstrdup_const(dev: *mut device, s: *const c_char, gfp: c_uint) -> *const c_char;
    fn of_node_put(node: *mut device_node);
    fn of_get_parent(node: *mut device_node) -> *mut device_node;
    fn simple_util_parse_convert(np: *mut device_node, prefix: *const c_char, adata: *mut simple_util_data);
    fn snd_soc_link_to_cpu(link: *mut snd_soc_dai_link, id: c_int) -> *mut snd_soc_dai_link_component;
    fn snd_soc_link_to_codec(link: *mut snd_soc_dai_link, id: c_int) -> *mut snd_soc_dai_link_component;
    fn snd_soc_link_to_platform(link: *mut snd_soc_dai_link, id: c_int) -> *mut snd_soc_dai_link_component;
    fn simple_util_parse_clk(
        dev: *mut device,
        np: *mut device_node,
        dai: *mut simple_util_dai,
        dlc: *mut snd_soc_dai_link_component,
    ) -> c_int;
    fn simple_util_parse_tdm(np: *mut device_node, dai: *mut simple_util_dai) -> c_int;
    fn simple_util_parse_daifmt(
        dev: *mut device,
        node: *mut device_node,
        codec: *mut device_node,
        prefix: *mut c_char,
        dai_fmt: *mut c_uint,
    ) -> c_int;
    fn graph_util_parse_link_direction(np: *mut device_node, playback_only: *mut bool_, capture_only: *mut bool_);
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut c_uint) -> c_int;
    fn graph_util_parse_trigger_order(
        priv_: *mut simple_util_priv,
        np: *mut device_node,
        trigger_start: *mut snd_soc_trigger_order,
        trigger_stop: *mut snd_soc_trigger_order,
    );
    fn simple_util_set_dailink_name(
        priv_: *mut simple_util_priv,
        dai_link: *mut snd_soc_dai_link,
        name: *const c_char,
    ) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn snprintf(s: *mut c_char, maxlen: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_of_parse_node_prefix(
        top: *mut device_node,
        cconf: *mut snd_soc_codec_conf,
        of_node: *mut device_node,
        propname: *const c_char,
    );
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_node_get(node: *mut device_node) -> *mut device_node;
    fn of_get_child_count(node: *mut device_node) -> c_int;
    fn of_get_next_child(node: *mut device_node, prev: *mut device_node) -> *mut device_node;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn simple_util_is_convert_required(adata: *mut simple_util_data) -> bool_;
    fn of_platform_depopulate(dev: *mut device);
    fn of_platform_populate(
        root: *mut device_node,
        matches: *const c_void,
        lookup: *const c_void,
        parent: *mut device,
    ) -> c_int;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn simple_util_init_priv(priv_: *mut simple_util_priv, li: *mut link_info) -> c_int;
    fn simple_util_parse_widgets(priv_: *mut simple_util_priv, prefix: *const c_char) -> c_int;
    fn simple_util_parse_routing(priv_: *mut simple_util_priv, prefix: *const c_char) -> c_int;
    fn simple_util_parse_pin_switches(priv_: *mut simple_util_priv, prefix: *const c_char) -> c_int;
    fn simple_util_parse_aux_devs(priv_: *mut simple_util_priv, prefix: *const c_char) -> c_int;
    fn simple_util_parse_card_name(priv_: *mut simple_util_priv, prefix: *const c_char) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut simple_util_priv);
    fn simple_util_debug_info(priv_: *mut simple_util_priv);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn simple_util_clean_reference(priv_: *mut simple_util_priv);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut simple_util_priv;
    fn simple_util_init_hp(card: *mut snd_soc_card, jack: *mut c_void, prefix: *const c_char) -> c_int;
    fn simple_util_init_mic(card: *mut snd_soc_card, jack: *mut c_void, prefix: *const c_char) -> c_int;
    fn simple_util_init_aux_jacks(card: *mut snd_soc_card, prefix: *const c_char) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
}

static simple_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(simple_util_startup),
    shutdown: Some(simple_util_shutdown),
    hw_params: Some(simple_util_hw_params),
};

unsafe fn simple_ret(priv_: *mut simple_util_priv, ret: c_int) -> c_int {
    _simple_ret(priv_, b"unknown\0".as_ptr() as *const c_char, ret)
}

unsafe fn _simple_ret(priv_: *mut simple_util_priv, func: *const c_char, ret: c_int) -> c_int {
    snd_soc_ret(simple_priv_to_dev(priv_), ret, b"at %s()\n\0".as_ptr() as *const c_char, func)
}

unsafe extern "C" fn simple_parse_platform(
    priv_: *mut simple_util_priv,
    node: *mut device_node,
    dlc: *mut snd_soc_dai_link_component,
) -> c_int {
    let mut args: of_phandle_args = core::mem::zeroed();
    let mut ret: c_int;

    if node.is_null() {
        return 0;
    }

    /*
     * Get node via "sound-dai = <&phandle port>"
     * It will be used as the of_node for component matching during
     * snd_soc_add_pcm_runtime().
     */
    ret = of_parse_phandle_with_args(node, DAI, CELL, 0, &mut args);
    if ret != 0 {
        return simple_ret(priv_, ret);
    }

    /* dai_name is not required and may not exist for plat component */

    (*dlc).of_node = args.np;

    0
}

unsafe extern "C" fn simple_parse_dai(
    priv_: *mut simple_util_priv,
    node: *mut device_node,
    dlc: *mut snd_soc_dai_link_component,
    is_single_link: *mut c_int,
) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let mut args: of_phandle_args = core::mem::zeroed();
    let mut resolved_dlc: snd_soc_dai_link_component = core::mem::zeroed();
    let dai: *mut snd_soc_dai;
    let mut fallback_dai_name: *const c_char;
    let mut ret: c_int;

    if node.is_null() {
        return 0;
    }

    /*
     * Get node via "sound-dai = <&phandle port>"
     * It will be used as the of_node for component matching during
     * snd_soc_add_pcm_runtime().
     */
    ret = of_parse_phandle_with_args(node, DAI, CELL, 0, &mut args);
    if ret != 0 {
        return simple_ret(priv_, ret);
    }

    /*
     * Try to find from DAI args
     */
    dai = snd_soc_get_dai_via_args(&mut args);
    if !dai.is_null() {
        ret = -ENOMEM;
        (*dlc).dai_name = snd_soc_dai_name_get(dai);
        (*dlc).dai_args = snd_soc_copy_dai_args(dev, &mut args);
        if (*dlc).dai_args.is_null() {
            return simple_ret(priv_, ret);
        }
    } else {
        ret = snd_soc_get_dlc(&mut args, &mut resolved_dlc);
        if ret < 0 {
            return simple_ret(priv_, ret);
        }

        /* Keep fallback dai_name valid across component rebind */
        fallback_dai_name = resolved_dlc.dai_name;
        if !fallback_dai_name.is_null() {
            fallback_dai_name = devm_kstrdup_const(dev, fallback_dai_name, GFP_KERNEL);
            ret = -ENOMEM;
            if fallback_dai_name.is_null() {
                of_node_put(resolved_dlc.of_node);
                return simple_ret(priv_, ret);
            }
        }

        (*dlc).of_node = resolved_dlc.of_node;
        (*dlc).dai_name = fallback_dai_name;
        (*dlc).dai_args = resolved_dlc.dai_args;
    }

    if !is_single_link.is_null() {
        *is_single_link = (args.args_count == 0) as c_int;
    }
    ret = 0;

    simple_ret(priv_, ret)
}

unsafe fn simple_parse_convert(dev: *mut device, np: *mut device_node, adata: *mut simple_util_data) {
    let top = (*dev).of_node;
    let node = of_get_parent(np);

    simple_util_parse_convert(top, PREFIX, adata);
    simple_util_parse_convert(node, PREFIX, adata);
    simple_util_parse_convert(node, ptr::null(), adata);
    simple_util_parse_convert(np, ptr::null(), adata);

    of_node_put(node);
}

unsafe extern "C" fn simple_parse_node(
    priv_: *mut simple_util_priv,
    np: *mut device_node,
    li: *mut link_info,
    _prefix: *mut c_char,
    cpu: *mut c_int,
) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let dai_link = simple_priv_to_link(priv_, (*li).link);
    let dai_props = simple_priv_to_props(priv_, (*li).link);
    let dlc: *mut snd_soc_dai_link_component;
    let dai: *mut simple_util_dai;
    let mut ret: c_int;

    if !cpu.is_null() {
        dlc = snd_soc_link_to_cpu(dai_link, 0);
        dai = simple_props_to_dai_cpu(dai_props, 0);
    } else {
        dlc = snd_soc_link_to_codec(dai_link, 0);
        dai = simple_props_to_dai_codec(dai_props, 0);
    }

    ret = simple_parse_dai(priv_, np, dlc, cpu);
    if ret != 0 {
        return simple_ret(priv_, ret);
    }

    ret = simple_util_parse_clk(dev, np, dai, dlc);
    if ret != 0 {
        return simple_ret(priv_, ret);
    }

    ret = simple_util_parse_tdm(np, dai);
    simple_ret(priv_, ret)
}

unsafe fn simple_link_init(
    priv_: *mut simple_util_priv,
    cpu: *mut device_node,
    codec: *mut device_node,
    li: *mut link_info,
    prefix: *mut c_char,
    name: *mut c_char,
) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let top = (*dev).of_node;
    let dai_link = simple_priv_to_link(priv_, (*li).link);
    let dai_props = simple_priv_to_props(priv_, (*li).link);
    let node = of_get_parent(cpu);
    let mut trigger_start = SND_SOC_TRIGGER_ORDER_DEFAULT;
    let mut trigger_stop = SND_SOC_TRIGGER_ORDER_DEFAULT;
    let mut playback_only: bool_ = false;
    let mut capture_only: bool_ = false;
    let mut ret: c_int;

    ret = simple_util_parse_daifmt(dev, node, codec, prefix, &mut (*dai_link).dai_fmt);
    if ret < 0 {
        of_node_put(node);
        return simple_ret(priv_, ret);
    }

    graph_util_parse_link_direction(top, &mut playback_only, &mut capture_only);
    graph_util_parse_link_direction(node, &mut playback_only, &mut capture_only);
    graph_util_parse_link_direction(cpu, &mut playback_only, &mut capture_only);
    graph_util_parse_link_direction(codec, &mut playback_only, &mut capture_only);

    of_property_read_u32(top, b"mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(top, b"simple-audio-card,mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(node, b"mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(node, b"simple-audio-card,mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(cpu, b"mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(cpu, b"simple-audio-card,mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(codec, b"mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(codec, b"simple-audio-card,mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);

    graph_util_parse_trigger_order(priv_, top, &mut trigger_start, &mut trigger_stop);
    graph_util_parse_trigger_order(priv_, node, &mut trigger_start, &mut trigger_stop);
    graph_util_parse_trigger_order(priv_, cpu, &mut trigger_start, &mut trigger_stop);
    graph_util_parse_trigger_order(priv_, codec, &mut trigger_start, &mut trigger_stop);

    (*dai_link).playback_only = playback_only;
    (*dai_link).capture_only = capture_only;

    (*dai_link).trigger_start = trigger_start;
    (*dai_link).trigger_stop = trigger_stop;

    (*dai_link).init = Some(simple_util_dai_init);
    (*dai_link).ops = &simple_ops;

    ret = simple_util_set_dailink_name(priv_, dai_link, name);
    of_node_put(node);
    simple_ret(priv_, ret)
}

unsafe extern "C" fn simple_dai_link_of_dpcm(
    priv_: *mut simple_util_priv,
    np: *mut device_node,
    codec: *mut device_node,
    li: *mut link_info,
    is_top: bool_,
) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let dai_link = simple_priv_to_link(priv_, (*li).link);
    let dai_props = simple_priv_to_props(priv_, (*li).link);
    let top = (*dev).of_node;
    let node = of_get_parent(np);
    let mut prefix: *mut c_char = b"\0".as_ptr() as *mut c_char;
    let mut dai_name = [0 as c_char; 64];
    let mut ret: c_int;

    dev_dbg(dev, b"link_of DPCM (%pOF)\n\0".as_ptr() as *const c_char, np);

    /* For single DAI link & old style of DT node */
    if is_top {
        prefix = PREFIX as *mut c_char;
    }

    if (*li).cpu != 0 {
        let cpus = snd_soc_link_to_cpu(dai_link, 0);
        let platforms = snd_soc_link_to_platform(dai_link, 0);
        let mut is_single_links: c_int = 0;

        /* Codec is dummy */

        /* FE settings */
        (*dai_link).dynamic = 1;
        (*dai_link).dpcm_merged_format = 1;

        ret = simple_parse_node(priv_, np, li, prefix, &mut is_single_links);
        if ret < 0 {
            of_node_put(node);
            (*li).link += 1;
            return simple_ret(priv_, ret);
        }

        snprintf(
            dai_name.as_mut_ptr(),
            dai_name.len(),
            b"fe.%s\0".as_ptr() as *const c_char,
            (*cpus).dai_name,
        );

        simple_util_canonicalize_cpu(cpus, is_single_links);
        simple_util_canonicalize_platform(platforms, cpus);
    } else {
        let codecs = snd_soc_link_to_codec(dai_link, 0);
        let cconf: *mut snd_soc_codec_conf;

        /* CPU is dummy */

        /* BE settings */
        (*dai_link).no_pcm = 1;
        (*dai_link).be_hw_params_fixup = Some(simple_util_be_hw_params_fixup);

        cconf = simple_props_to_codec_conf(dai_props, 0);

        ret = simple_parse_node(priv_, np, li, prefix, ptr::null_mut());
        if ret < 0 {
            of_node_put(node);
            (*li).link += 1;
            return simple_ret(priv_, ret);
        }

        snprintf(
            dai_name.as_mut_ptr(),
            dai_name.len(),
            b"be.%s\0".as_ptr() as *const c_char,
            (*codecs).dai_name,
        );

        /* check "prefix" from top node */
        snd_soc_of_parse_node_prefix(top, cconf, (*codecs).of_node, b"simple-audio-card,prefix\0".as_ptr() as *const c_char);
        snd_soc_of_parse_node_prefix(node, cconf, (*codecs).of_node, b"prefix\0".as_ptr() as *const c_char);
        snd_soc_of_parse_node_prefix(np, cconf, (*codecs).of_node, b"prefix\0".as_ptr() as *const c_char);
    }

    simple_parse_convert(dev, np, &mut (*dai_props).adata);

    ret = simple_link_init(priv_, np, codec, li, prefix, dai_name.as_mut_ptr());

    of_node_put(node);
    (*li).link += 1;
    simple_ret(priv_, ret)
}

unsafe extern "C" fn simple_dai_link_of(
    priv_: *mut simple_util_priv,
    np: *mut device_node,
    codec: *mut device_node,
    li: *mut link_info,
    is_top: bool_,
) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let dai_link = simple_priv_to_link(priv_, (*li).link);
    let cpus = snd_soc_link_to_cpu(dai_link, 0);
    let codecs = snd_soc_link_to_codec(dai_link, 0);
    let platforms = snd_soc_link_to_platform(dai_link, 0);
    let cpu = np;
    let mut dai_name = [0 as c_char; 64];
    let mut prop = [0 as c_char; 128];
    let mut prefix: *mut c_char = b"\0".as_ptr() as *mut c_char;
    let mut ret: c_int;
    let mut single_cpu: c_int = 0;
    let node = of_get_parent(np);

    dev_dbg(dev, b"link_of (%pOF)\n\0".as_ptr() as *const c_char, node);

    /* For single DAI link & old style of DT node */
    if is_top {
        prefix = PREFIX as *mut c_char;
    }

    snprintf(prop.as_mut_ptr(), prop.len(), b"%splat\0".as_ptr() as *const c_char, prefix);
    let plat = of_get_child_by_name(node, prop.as_ptr());

    ret = simple_parse_node(priv_, cpu, li, prefix, &mut single_cpu);
    if ret < 0 {
        of_node_put(plat);
        of_node_put(node);
        (*li).link += 1;
        return simple_ret(priv_, ret);
    }

    ret = simple_parse_node(priv_, codec, li, prefix, ptr::null_mut());
    if ret < 0 {
        of_node_put(plat);
        of_node_put(node);
        (*li).link += 1;
        return simple_ret(priv_, ret);
    }

    ret = simple_parse_platform(priv_, plat, platforms);
    if ret < 0 {
        of_node_put(plat);
        of_node_put(node);
        (*li).link += 1;
        return simple_ret(priv_, ret);
    }

    snprintf(
        dai_name.as_mut_ptr(),
        dai_name.len(),
        b"%s-%s\0".as_ptr() as *const c_char,
        (*cpus).dai_name,
        (*codecs).dai_name,
    );

    simple_util_canonicalize_cpu(cpus, single_cpu);
    simple_util_canonicalize_platform(platforms, cpus);

    ret = simple_link_init(priv_, cpu, codec, li, prefix, dai_name.as_mut_ptr());

    of_node_put(plat);
    of_node_put(node);
    (*li).link += 1;
    simple_ret(priv_, ret)
}

extern "C" {
    fn simple_util_canonicalize_cpu(cpus: *mut snd_soc_dai_link_component, is_single_links: c_int);
    fn simple_util_canonicalize_platform(
        platforms: *mut snd_soc_dai_link_component,
        cpus: *mut snd_soc_dai_link_component,
    );
}

unsafe fn __simple_for_each_link(
    priv_: *mut simple_util_priv,
    li: *mut link_info,
    func_noml: link_func,
    func_dpcm: link_func,
) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let top = (*dev).of_node;
    let mut node: *mut device_node;
    let dpcm_selectable = of_device_get_match_data(dev) as uintptr_t;
    let mut is_top = false;
    let mut ret: c_int = 0;

    /* Check if it has dai-link */
    node = of_get_child_by_name(top, b"simple-audio-card,dai-link\0".as_ptr() as *const c_char);
    if node.is_null() {
        node = of_node_get(top);
        is_top = true;
    }

    let add_devs = of_get_child_by_name(top, b"simple-audio-card,additional-devs\0".as_ptr() as *const c_char);

    /* loop for all dai-link */
    loop {
        let mut adata: simple_util_data = core::mem::zeroed();
        let num = of_get_child_count(node);

        /* Skip additional-devs node */
        if node == add_devs {
            node = of_get_next_child(top, node);
            continue;
        }

        /* get codec */
        let codec = of_get_child_by_name(
            node,
            if is_top {
                b"simple-audio-card,codec\0".as_ptr() as *const c_char
            } else {
                b"codec\0".as_ptr() as *const c_char
            },
        );
        if codec.is_null() {
            ret = -ENODEV;
            break;
        }
        /* get platform */
        let plat = of_get_child_by_name(
            node,
            if is_top {
                b"simple-audio-card,plat\0".as_ptr() as *const c_char
            } else {
                b"plat\0".as_ptr() as *const c_char
            },
        );

        /* get convert-xxx property */
        memset(&mut adata as *mut _ as *mut c_void, 0, size_of::<simple_util_data>());
        let mut np = of_get_next_child(node, ptr::null_mut());
        while !np.is_null() {
            if np != add_devs {
                simple_parse_convert(dev, np, &mut adata);
            }
            let prev = np;
            np = of_get_next_child(node, prev);
        }

        /* loop for all CPU/Codec node */
        np = of_get_next_child(node, ptr::null_mut());
        while !np.is_null() {
            if plat == np || add_devs == np {
                let prev = np;
                np = of_get_next_child(node, prev);
                continue;
            }
            /*
             * It is DPCM
             * if it has many CPUs,
             * or has convert-xxx property
             */
            if dpcm_selectable != 0 && (num > 2 || simple_util_is_convert_required(&mut adata)) {
                /*
                 * np
                 *       |1(CPU)|0(Codec)  li->cpu
                 * CPU   |Pass  |return
                 * Codec |return|Pass
                 */
                if ((*li).cpu != 0) != (np == codec) {
                    ret = func_dpcm(priv_, np, codec, li, is_top);
                }
            /* else normal sound */
            } else {
                /*
                 * np
                 *       |1(CPU)|0(Codec)  li->cpu
                 * CPU   |Pass  |return
                 * Codec |return|return
                 */
                if (*li).cpu != 0 && np != codec {
                    ret = func_noml(priv_, np, codec, li, is_top);
                }
            }

            if ret < 0 {
                of_node_put(plat);
                of_node_put(codec);
                of_node_put(add_devs);
                of_node_put(node);
                return simple_ret(priv_, ret);
            }
            let prev = np;
            np = of_get_next_child(node, prev);
        }

        of_node_put(plat);
        of_node_put(codec);
        node = of_get_next_child(top, node);
        if is_top || node.is_null() {
            break;
        }
    }

    of_node_put(add_devs);
    of_node_put(node);

    simple_ret(priv_, ret)
}

unsafe fn simple_for_each_link(
    priv_: *mut simple_util_priv,
    li: *mut link_info,
    func_noml: link_func,
    func_dpcm: link_func,
) -> c_int {
    let mut ret: c_int = 0;
    /*
     * Detect all CPU first, and Detect all Codec 2nd.
     *
     * In Normal sound case, all DAIs are detected
     * as "CPU-Codec".
     *
     * In DPCM sound case,
     * all CPUs   are detected as "CPU-dummy", and
     * all Codecs are detected as "dummy-Codec".
     * To avoid random sub-device numbering,
     * detect "dummy-Codec" in last;
     */
    (*li).cpu = 1;
    while (*li).cpu >= 0 {
        ret = __simple_for_each_link(priv_, li, func_noml, func_dpcm);
        if ret < 0 {
            break;
        }
        (*li).cpu -= 1;
    }

    simple_ret(priv_, ret)
}

unsafe extern "C" fn simple_depopulate_aux(data: *mut c_void) {
    let priv_ = data as *mut simple_util_priv;

    of_platform_depopulate(simple_priv_to_dev(priv_));
}

unsafe fn simple_populate_aux(priv_: *mut simple_util_priv) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let node = of_get_child_by_name((*dev).of_node, b"simple-audio-card,additional-devs\0".as_ptr() as *const c_char);
    let mut ret: c_int;

    if node.is_null() {
        return 0;
    }

    ret = of_platform_populate(node, ptr::null(), ptr::null(), dev);
    if ret != 0 {
        of_node_put(node);
        return simple_ret(priv_, ret);
    }

    ret = devm_add_action_or_reset(dev, simple_depopulate_aux, priv_ as *mut c_void);
    of_node_put(node);
    simple_ret(priv_, ret)
}

unsafe extern "C" fn simple_count_noml(
    priv_: *mut simple_util_priv,
    _np: *mut device_node,
    _codec: *mut device_node,
    li: *mut link_info,
    _is_top: bool_,
) -> c_int {
    let mut ret = -EINVAL;

    if (*li).link >= SNDRV_MAX_LINKS {
        return simple_ret(priv_, ret);
    }

    /*
     * DON'T REMOVE platforms
     *
     * Some CPU might be using soc-generic-dmaengine-pcm. This means CPU and Platform
     * are different Component, but are sharing same component->dev.
     * Simple Card had been supported it without special Platform selection.
     * We need platforms here.
     *
     * In case of no Platform, it will be Platform == CPU, but Platform will be
     * ignored by snd_soc_rtd_add_component().
     *
     * see
     *      simple-card-utils.c :: simple_util_canonicalize_platform()
     */
    (*li).num[(*li).link as usize].cpus = 1;
    (*li).num[(*li).link as usize].platforms = 1;

    (*li).num[(*li).link as usize].codecs = 1;

    (*li).link += 1;
    ret = 0;
    simple_ret(priv_, ret)
}

unsafe extern "C" fn simple_count_dpcm(
    priv_: *mut simple_util_priv,
    _np: *mut device_node,
    _codec: *mut device_node,
    li: *mut link_info,
    _is_top: bool_,
) -> c_int {
    let mut ret = -EINVAL;

    if (*li).link >= SNDRV_MAX_LINKS {
        return simple_ret(priv_, ret);
    }

    if (*li).cpu != 0 {
        /*
         * DON'T REMOVE platforms
         * see
         *      simple_count_noml()
         */
        (*li).num[(*li).link as usize].cpus = 1;
        (*li).num[(*li).link as usize].platforms = 1;

        (*li).link += 1; /* CPU-dummy */
    } else {
        (*li).num[(*li).link as usize].codecs = 1;

        (*li).link += 1; /* dummy-Codec */
    }
    ret = 0;
    simple_ret(priv_, ret)
}

unsafe fn simple_get_dais_count(priv_: *mut simple_util_priv, li: *mut link_info) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let top = (*dev).of_node;

    /*
     * link_num :  number of links.
     *             CPU-Codec / CPU-dummy / dummy-Codec
     * dais_num :  number of DAIs
     * ccnf_num :  number of codec_conf
     *             same number for "dummy-Codec"
     *
     * ex1)
     * CPU0 --- Codec0     link : 5
     * CPU1 --- Codec1     dais : 7
     * CPU2 -/             ccnf : 1
     * CPU3 --- Codec2
     *
     *      => 5 links = 2xCPU-Codec + 2xCPU-dummy + 1xdummy-Codec
     *      => 7 DAIs  = 4xCPU + 3xCodec
     *      => 1 ccnf  = 1xdummy-Codec
     *
     * ex2)
     * CPU0 --- Codec0     link : 5
     * CPU1 --- Codec1     dais : 6
     * CPU2 -/             ccnf : 1
     * CPU3 -/
     *
     *      => 5 links = 1xCPU-Codec + 3xCPU-dummy + 1xdummy-Codec
     *      => 6 DAIs  = 4xCPU + 2xCodec
     *      => 1 ccnf  = 1xdummy-Codec
     *
     * ex3)
     * CPU0 --- Codec0     link : 6
     * CPU1 -/             dais : 6
     * CPU2 --- Codec1     ccnf : 2
     * CPU3 -/
     *
     *      => 6 links = 0xCPU-Codec + 4xCPU-dummy + 2xdummy-Codec
     *      => 6 DAIs  = 4xCPU + 2xCodec
     *      => 2 ccnf  = 2xdummy-Codec
     *
     * ex4)
     * CPU0 --- Codec0 (convert-rate)       link : 3
     * CPU1 --- Codec1                      dais : 4
     *                                      ccnf : 1
     *
     *      => 3 links = 1xCPU-Codec + 1xCPU-dummy + 1xdummy-Codec
     *      => 4 DAIs  = 2xCPU + 2xCodec
     *      => 1 ccnf  = 1xdummy-Codec
     */
    if top.is_null() {
        (*li).num[0].cpus = 1;
        (*li).num[0].codecs = 1;
        (*li).num[0].platforms = 1;

        (*li).link = 1;
        return 0;
    }

    simple_for_each_link(priv_, li, simple_count_noml, simple_count_dpcm)
}

unsafe extern "C" fn simple_soc_probe(card: *mut snd_soc_card) -> c_int {
    let priv_ = snd_soc_card_get_drvdata(card);
    let mut ret: c_int;

    ret = simple_util_init_hp(card, &mut (*priv_).hp_jack, PREFIX);
    if ret < 0 {
        return simple_ret(priv_, ret);
    }

    ret = simple_util_init_mic(card, &mut (*priv_).mic_jack, PREFIX);
    if ret < 0 {
        return simple_ret(priv_, ret);
    }

    ret = simple_util_init_aux_jacks(card, PREFIX);
    simple_ret(priv_, ret)
}

unsafe fn simple_parse_of(priv_: *mut simple_util_priv) -> c_int {
    let card = simple_priv_to_card(priv_);
    let dev = (*card).dev;
    let mut ret = -EINVAL;

    if dev.is_null() {
        return simple_ret(priv_, ret);
    }

    ret = -ENOMEM;
    let li = kzalloc(size_of::<link_info>(), GFP_KERNEL) as *mut link_info;
    if li.is_null() {
        return simple_ret(priv_, ret);
    }

    ret = simple_get_dais_count(priv_, li);
    if ret < 0 {
        kfree(li as *mut c_void);
        return simple_ret(priv_, ret);
    }

    ret = -EINVAL;
    if (*li).link == 0 {
        kfree(li as *mut c_void);
        return simple_ret(priv_, ret);
    }

    ret = simple_util_init_priv(priv_, li);
    if ret < 0 {
        kfree(li as *mut c_void);
        return simple_ret(priv_, ret);
    }

    ret = simple_util_parse_widgets(priv_, PREFIX);
    if ret < 0 {
        kfree(li as *mut c_void);
        return simple_ret(priv_, ret);
    }

    ret = simple_util_parse_routing(priv_, PREFIX);
    if ret < 0 {
        kfree(li as *mut c_void);
        return simple_ret(priv_, ret);
    }

    ret = simple_util_parse_pin_switches(priv_, PREFIX);
    if ret < 0 {
        kfree(li as *mut c_void);
        return simple_ret(priv_, ret);
    }

    ret = simple_util_parse_aux_devs(priv_, PREFIX);
    if ret < 0 {
        simple_util_clean_reference(priv_);
        kfree(li as *mut c_void);
        return dev_err_probe(dev, ret, b"parse error\n\0".as_ptr() as *const c_char);
    }

    /* Single/Muti DAI link(s) & New style of DT node */
    memset(li as *mut c_void, 0, size_of::<link_info>());
    ret = simple_for_each_link(priv_, li, simple_dai_link_of, simple_dai_link_of_dpcm);
    if ret < 0 {
        simple_util_clean_reference(priv_);
        kfree(li as *mut c_void);
        return dev_err_probe(dev, ret, b"parse error\n\0".as_ptr() as *const c_char);
    }

    /* Card name should be set after simple_for_each_link() */
    ret = simple_util_parse_card_name(priv_, PREFIX);
    if ret < 0 {
        simple_util_clean_reference(priv_);
        kfree(li as *mut c_void);
        return dev_err_probe(dev, ret, b"parse error\n\0".as_ptr() as *const c_char);
    }

    ret = simple_populate_aux(priv_);
    if ret < 0 {
        simple_util_clean_reference(priv_);
        kfree(li as *mut c_void);
        return dev_err_probe(dev, ret, b"parse error\n\0".as_ptr() as *const c_char);
    }

    snd_soc_card_set_drvdata(card, priv_);

    simple_util_debug_info(priv_);

    ret = devm_snd_soc_register_card(dev, card);
    if ret < 0 {
        simple_util_clean_reference(priv_);
        kfree(li as *mut c_void);
        return dev_err_probe(dev, ret, b"parse error\n\0".as_ptr() as *const c_char);
    }

    kfree(li as *mut c_void);
    simple_ret(priv_, ret)
}

unsafe extern "C" fn simple_probe(pdev: *mut platform_device) -> c_int {
    let priv_: *mut simple_util_priv;
    let dev = &mut (*pdev).dev as *mut device;
    let card: *mut snd_soc_card;

    /* Allocate the private data and the DAI link array */
    priv_ = devm_kzalloc(dev, size_of::<simple_util_priv>(), GFP_KERNEL) as *mut simple_util_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    card = simple_priv_to_card(priv_);
    (*card).owner = THIS_MODULE;
    (*card).dev = dev;
    (*card).probe = Some(simple_soc_probe);
    (*card).driver_name = b"simple-card\0".as_ptr() as *const c_char;

    simple_parse_of(priv_)
}

static simple_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: b"simple-audio-card\0".as_ptr() as *const c_char,
        data: ptr::null(),
    },
    of_device_id {
        compatible: b"simple-scu-audio-card\0".as_ptr() as *const c_char,
        data: DPCM_SELECTABLE as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, simple_of_match);

static simple_card: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"asoc-simple-card\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops },
        of_match_table: simple_of_match.as_ptr(),
    },
    probe: Some(simple_probe),
    remove: Some(simple_util_remove),
};

// module_platform_driver(simple_card);

// MODULE_ALIAS("platform:asoc-simple-card");
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("ASoC Simple Sound Card");
// MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
