// SPDX-License-Identifier: GPL-2.0
//
// ASoC Audio Graph Card2 support
//
// Copyright (C) 2020 Renesas Electronics Corp.
// Copyright (C) 2020 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// based on ${LINUX}/sound/soc/generic/audio-graph-card.c
//
// Dependency intent from C includes:
// linux/clk.h, linux/device.h, linux/gpio/consumer.h, linux/module.h,
// linux/of.h, linux/of_graph.h, linux/platform_device.h, linux/string.h,
// sound/graph_card.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type bool_ = c_int;
type u32 = u32;
type GRAPH2_CUSTOM = Option<
    unsafe extern "C" fn(
        priv_: *mut simple_util_priv,
        lnk: *mut device_node,
        li: *mut link_info,
    ) -> c_int,
>;

const NULL: usize = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const SNDRV_MAX_LINKS: c_int = 32;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;
const SNDRV_PCM_RATE_8000_384000: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_CLOCK_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x0f00;
const THIS_MODULE: *mut c_void = null_mut();

const GRAPH_NODENAME_MULTI: &[u8] = b"multi\0";
const GRAPH_NODENAME_DPCM: &[u8] = b"dpcm\0";
const GRAPH_NODENAME_C2C: &[u8] = b"codec2codec\0";

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum graph_type {
    GRAPH_NORMAL,
    GRAPH_DPCM,
    GRAPH_C2C,
    GRAPH_MULTI, /* don't use ! Use this only in __graph_get_type() */
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    pub fwnode: fwnode_handle,
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
pub struct simple_util_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct simple_util_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
    pub ext_fmt: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_link_ch_map {
    pub cpu: c_int,
    pub codec: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub formats: u64,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn() -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn()>,
    pub hw_params: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum snd_soc_trigger_order {
    SND_SOC_TRIGGER_ORDER_DEFAULT,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub num_cpus: c_int,
    pub num_codecs: c_int,
    pub ch_maps: *mut snd_soc_dai_link_ch_map,
    pub playback_only: bool_,
    pub capture_only: bool_,
    pub trigger_start: snd_soc_trigger_order,
    pub trigger_stop: snd_soc_trigger_order,
    pub dai_fmt: c_uint,
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub ops: *const snd_soc_ops,
    pub dynamic: c_int,
    pub dpcm_merged_format: c_int,
    pub no_pcm: c_int,
    pub be_hw_params_fixup: Option<unsafe extern "C" fn() -> c_int>,
    pub c2c_params: *mut snd_soc_pcm_stream,
    pub num_c2c_params: c_int,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct simple_dai_props {
    pub adata: simple_util_data,
    pub mclk_fs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_count {
    pub cpus: c_int,
    pub codecs: c_int,
    pub platforms: c_int,
}

#[repr(C)]
pub struct link_info {
    pub link: c_int,
    pub num: [link_count; SNDRV_MAX_LINKS as usize],
}

#[repr(C)]
pub struct snd_soc_card {
    pub probe: Option<unsafe extern "C" fn() -> c_int>,
    pub owner: *mut c_void,
    pub dev: *mut device,
}

#[repr(C)]
pub struct simple_util_priv {
    pub ops: *const snd_soc_ops,
    pub pa_gpio: *mut c_void,
}

#[repr(C)]
pub struct graph2_custom_hooks {
    pub custom_normal: GRAPH2_CUSTOM,
    pub custom_dpcm: GRAPH2_CUSTOM,
    pub custom_c2c: GRAPH2_CUSTOM,
    pub hook_pre: Option<unsafe extern "C" fn(*mut simple_util_priv) -> c_int>,
    pub hook_post: Option<unsafe extern "C" fn(*mut simple_util_priv) -> c_int>,
}

#[repr(C)]
pub struct of_phandle_iterator {
    pub node: *mut device_node,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub pm: *const c_void,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    fn snd_soc_ret(dev: *mut device, ret: c_int, fmt: *const c_char, ...) -> c_int;
    fn simple_priv_to_dev(priv_: *mut simple_util_priv) -> *mut device;
    fn simple_priv_to_link(priv_: *mut simple_util_priv, link: c_int) -> *mut snd_soc_dai_link;
    fn simple_priv_to_props(priv_: *mut simple_util_priv, link: c_int) -> *mut simple_dai_props;
    fn simple_priv_to_card(priv_: *mut simple_util_priv) -> *mut snd_soc_card;
    fn simple_props_to_dai_cpu(props: *mut simple_dai_props, idx: c_int) -> *mut simple_util_dai;
    fn simple_props_to_dai_codec(props: *mut simple_dai_props, idx: c_int) -> *mut simple_util_dai;
    fn simple_props_to_codec_conf(props: *mut simple_dai_props, idx: c_int) -> *mut snd_soc_codec_conf;
    fn snd_soc_link_to_cpu(link: *mut snd_soc_dai_link, idx: c_int) -> *mut snd_soc_dai_link_component;
    fn snd_soc_link_to_codec(link: *mut snd_soc_dai_link, idx: c_int) -> *mut snd_soc_dai_link_component;
    fn snd_soc_link_to_platform(link: *mut snd_soc_dai_link, idx: c_int) -> *mut snd_soc_dai_link_component;
    fn graph_util_parse_dai(priv_: *mut simple_util_priv, ep: *mut device_node, dlc: *mut snd_soc_dai_link_component, single: *mut c_int) -> c_int;
    fn graph_util_is_ports0(lnk: *mut device_node) -> c_int;
    fn graph_util_parse_link_direction(node: *mut device_node, playback_only: *mut bool_, capture_only: *mut bool_);
    fn graph_util_parse_trigger_order(priv_: *mut simple_util_priv, node: *mut device_node, start: *mut snd_soc_trigger_order, stop: *mut snd_soc_trigger_order);
    fn graph_util_card_probe() -> c_int;
    fn graph_util_parse_dai_dummy() -> c_int;
    fn simple_util_startup() -> c_int;
    fn simple_util_shutdown();
    fn simple_util_hw_params() -> c_int;
    fn simple_util_parse_convert(node: *mut device_node, prefix: *const c_char, adata: *mut simple_util_data);
    fn simple_util_parse_tdm(ep: *mut device_node, dai: *mut simple_util_dai) -> c_int;
    fn simple_util_parse_tdm_width_map(priv_: *mut simple_util_priv, ep: *mut device_node, dai: *mut simple_util_dai) -> c_int;
    fn simple_util_parse_clk(dev: *mut device, ep: *mut device_node, dai: *mut simple_util_dai, dlc: *mut snd_soc_dai_link_component) -> c_int;
    fn simple_util_set_dailink_name(priv_: *mut simple_util_priv, link: *mut snd_soc_dai_link, fmt: *const c_char, ...);
    fn simple_util_canonicalize_cpu(cpus: *mut snd_soc_dai_link_component, single: c_int);
    fn simple_util_canonicalize_platform(platforms: *mut snd_soc_dai_link_component, cpus: *mut snd_soc_dai_link_component);
    fn simple_util_dai_init() -> c_int;
    fn simple_util_be_hw_params_fixup() -> c_int;
    fn simple_util_init_priv(priv_: *mut simple_util_priv, li: *mut link_info) -> c_int;
    fn simple_util_parse_widgets(priv_: *mut simple_util_priv, prefix: *const c_char) -> c_int;
    fn simple_util_parse_routing(priv_: *mut simple_util_priv, prefix: *const c_char) -> c_int;
    fn simple_util_parse_aux_devs(priv_: *mut simple_util_priv, prefix: *const c_char) -> c_int;
    fn simple_util_parse_card_name(priv_: *mut simple_util_priv, prefix: *const c_char) -> c_int;
    fn simple_util_debug_info(priv_: *mut simple_util_priv);
    fn simple_util_clean_reference(priv_: *mut simple_util_priv);
    fn simple_util_remove(pdev: *mut platform_device) -> c_int;
    fn snd_soc_of_parse_node_prefix(node: *mut device_node, conf: *mut snd_soc_codec_conf, of_node: *mut device_node, prop: *const c_char);
    fn snd_soc_daifmt_parse_format(node: *mut device_node, prefix: *const c_char) -> c_uint;
    fn snd_soc_daifmt_parse_clock_provider_as_bitmap(node: *mut device_node, prefix: *const c_char) -> c_uint;
    fn snd_soc_daifmt_clock_provider_from_bitmap(bitmap: c_uint) -> c_uint;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, priv_: *mut simple_util_priv);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn of_get_parent(node: *mut device_node) -> *mut device_node;
    fn of_node_name_eq(node: *mut device_node, name: *const c_char) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn of_node_get(node: *mut device_node) -> *mut device_node;
    fn of_graph_get_next_port_endpoint(port: *mut device_node, prev: *mut device_node) -> *mut device_node;
    fn of_graph_get_remote_endpoint(ep: *mut device_node) -> *mut device_node;
    fn of_graph_get_remote_port(ep: *mut device_node) -> *mut device_node;
    fn of_graph_get_next_port(parent: *mut device_node, prev: *mut device_node) -> *mut device_node;
    fn of_graph_get_port_by_id(parent: *mut device_node, id: c_int) -> *mut device_node;
    fn of_graph_get_port_count(parent: *mut device_node) -> c_int;
    fn of_property_read_u32(node: *mut device_node, prop: *const c_char, out: *mut u32) -> c_int;
    fn fw_devlink_purge_absent_suppliers(fwnode: *mut fwnode_handle);
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    static snd_soc_pm_ops: c_void;
}

#[inline]
unsafe fn graph_ret(priv_: *mut simple_util_priv, ret: c_int) -> c_int {
    _graph_ret(priv_, b"unknown\0".as_ptr() as *const c_char, ret)
}

#[inline]
unsafe fn _graph_ret(priv_: *mut simple_util_priv, func: *const c_char, ret: c_int) -> c_int {
    snd_soc_ret(simple_priv_to_dev(priv_), ret, b"at %s()\n\0".as_ptr() as *const c_char, func)
}

#[inline]
unsafe fn ep_to_port(ep: *mut device_node) -> *mut device_node {
    of_get_parent(ep)
}

unsafe fn port_to_ports(port: *mut device_node) -> *mut device_node {
    let ports = of_get_parent(port);

    if of_node_name_eq(ports, b"ports\0".as_ptr() as *const c_char) == 0 {
        of_node_put(ports);
        return null_mut();
    }
    ports
}

unsafe fn __graph_get_type(lnk: *mut device_node) -> graph_type {
    let mut np: *mut device_node;
    let parent_np: *mut device_node;
    let ret: graph_type;

    /*
     * target {
     *	ports {
     * =>		lnk:	port@0 { ... };
     *			port@1 { ... };
     *	};
     * };
     */
    np = of_get_parent(lnk);
    if of_node_name_eq(np, b"ports\0".as_ptr() as *const c_char) != 0 {
        parent_np = of_get_parent(np);
        of_node_put(np);
        np = parent_np;
    }

    if of_node_name_eq(np, GRAPH_NODENAME_MULTI.as_ptr() as *const c_char) != 0 {
        ret = graph_type::GRAPH_MULTI;
        fw_devlink_purge_absent_suppliers(&mut (*np).fwnode);
        of_node_put(np);
        return ret;
    }

    if of_node_name_eq(np, GRAPH_NODENAME_DPCM.as_ptr() as *const c_char) != 0 {
        ret = graph_type::GRAPH_DPCM;
        fw_devlink_purge_absent_suppliers(&mut (*np).fwnode);
        of_node_put(np);
        return ret;
    }

    if of_node_name_eq(np, GRAPH_NODENAME_C2C.as_ptr() as *const c_char) != 0 {
        ret = graph_type::GRAPH_C2C;
        fw_devlink_purge_absent_suppliers(&mut (*np).fwnode);
        of_node_put(np);
        return ret;
    }

    ret = graph_type::GRAPH_NORMAL;
    of_node_put(np);
    ret
}

unsafe fn graph_get_type(priv_: *mut simple_util_priv, lnk: *mut device_node) -> graph_type {
    let mut type_ = __graph_get_type(lnk);

    /* GRAPH_MULTI here means GRAPH_NORMAL */
    if type_ == graph_type::GRAPH_MULTI {
        type_ = graph_type::GRAPH_NORMAL;
    }

    // C DEBUG block intentionally preserved as conditional intent:
    // when DEBUG is enabled, print Normal/DPCM Front-End/DPCM Back-End/Codec2Codec.
    let _ = priv_;
    type_
}

unsafe fn graph_lnk_is_multi(lnk: *mut device_node) -> c_int {
    (__graph_get_type(lnk) == graph_type::GRAPH_MULTI) as c_int
}

unsafe fn graph_get_next_multi_ep(port: *mut *mut device_node, idx: c_int) -> *mut device_node {
    let ports = port_to_ports(*port);
    let mut rep: *mut device_node = null_mut();

    /*
     * Don't use of_graph_get_next_port() here
     *
     * In overlay case, "port" are not necessarily in order. So we need to use
     * of_graph_get_port_by_id() instead
     */
    of_node_put(*port);

    *port = of_graph_get_port_by_id(ports, idx);
    if !(*port).is_null() {
        let ep = of_graph_get_next_port_endpoint(*port, null_mut());
        rep = of_graph_get_remote_endpoint(ep);
        of_node_put(ep);
    }
    of_node_put(ports);

    rep
}

static graph_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(simple_util_startup),
    shutdown: Some(simple_util_shutdown),
    hw_params: Some(simple_util_hw_params),
};

unsafe fn graph_parse_convert(ep: *mut device_node, props: *mut simple_dai_props) {
    let port = ep_to_port(ep);
    let ports = port_to_ports(port);
    let adata = &mut (*props).adata as *mut simple_util_data;

    simple_util_parse_convert(ports, null(), adata);
    simple_util_parse_convert(port, null(), adata);
    simple_util_parse_convert(ep, null(), adata);
    of_node_put(ports);
    of_node_put(port);
}

unsafe fn __graph_parse_node(
    priv_: *mut simple_util_priv,
    gtype: graph_type,
    ep: *mut device_node,
    li: *mut link_info,
    is_cpu: c_int,
    idx: c_int,
) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let dai_link = simple_priv_to_link(priv_, (*li).link);
    let dai_props = simple_priv_to_props(priv_, (*li).link);
    let dlc: *mut snd_soc_dai_link_component;
    let dai: *mut simple_util_dai;
    let mut ret: c_int;
    let mut is_single_links: c_int = 0;

    if is_cpu != 0 {
        dlc = snd_soc_link_to_cpu(dai_link, idx);
        dai = simple_props_to_dai_cpu(dai_props, idx);
    } else {
        dlc = snd_soc_link_to_codec(dai_link, idx);
        dai = simple_props_to_dai_codec(dai_props, idx);
    }

    ret = graph_util_parse_dai(priv_, ep, dlc, &mut is_single_links);
    if ret < 0 { return graph_ret(priv_, ret); }
    ret = simple_util_parse_tdm(ep, dai);
    if ret < 0 { return graph_ret(priv_, ret); }
    ret = simple_util_parse_tdm_width_map(priv_, ep, dai);
    if ret < 0 { return graph_ret(priv_, ret); }
    ret = simple_util_parse_clk(dev, ep, dai, dlc);
    if ret < 0 { return graph_ret(priv_, ret); }

    /*
     * set DAI Name
     */
    if (*dai_link).name.is_null() {
        let cpus = dlc;
        let codecs = snd_soc_link_to_codec(dai_link, idx);
        let mut cpu_multi = b"\0".as_ptr() as *const c_char;
        let mut codec_multi = b"\0".as_ptr() as *const c_char;

        if (*dai_link).num_cpus > 1 {
            cpu_multi = b"_multi\0".as_ptr() as *const c_char;
        }
        if (*dai_link).num_codecs > 1 {
            codec_multi = b"_multi\0".as_ptr() as *const c_char;
        }

        match gtype {
            graph_type::GRAPH_NORMAL => {
                /* run is_cpu only. see audio_graph2_link_normal() */
                if is_cpu != 0 {
                    simple_util_set_dailink_name(priv_, dai_link, b"%s%s-%s%s\0".as_ptr() as *const c_char, (*cpus).dai_name, cpu_multi, (*codecs).dai_name, codec_multi);
                }
            }
            graph_type::GRAPH_DPCM => {
                if is_cpu != 0 {
                    simple_util_set_dailink_name(priv_, dai_link, b"fe.%pOFP.%s%s\0".as_ptr() as *const c_char, (*cpus).of_node, (*cpus).dai_name, cpu_multi);
                } else {
                    simple_util_set_dailink_name(priv_, dai_link, b"be.%pOFP.%s%s\0".as_ptr() as *const c_char, (*codecs).of_node, (*codecs).dai_name, codec_multi);
                }
            }
            graph_type::GRAPH_C2C => {
                /* run is_cpu only. see audio_graph2_link_c2c() */
                if is_cpu != 0 {
                    simple_util_set_dailink_name(priv_, dai_link, b"c2c.%s%s-%s%s\0".as_ptr() as *const c_char, (*cpus).dai_name, cpu_multi, (*codecs).dai_name, codec_multi);
                }
            }
            _ => {}
        }
    }

    /*
     * Check "prefix" from top node
     * if DPCM-BE case
     */
    if is_cpu == 0 && gtype == graph_type::GRAPH_DPCM {
        let codecs = snd_soc_link_to_codec(dai_link, idx);
        let cconf = simple_props_to_codec_conf(dai_props, idx);
        let rport = ep_to_port(ep);
        let rports = port_to_ports(rport);

        snd_soc_of_parse_node_prefix(rports, cconf, (*codecs).of_node, b"prefix\0".as_ptr() as *const c_char);
        snd_soc_of_parse_node_prefix(rport, cconf, (*codecs).of_node, b"prefix\0".as_ptr() as *const c_char);
        of_node_put(rports);
        of_node_put(rport);
    }

    if is_cpu != 0 {
        let cpus = dlc;
        let platforms = snd_soc_link_to_platform(dai_link, idx);

        simple_util_canonicalize_cpu(cpus, is_single_links);
        simple_util_canonicalize_platform(platforms, cpus);
    }
    graph_ret(priv_, ret)
}

unsafe fn graph_parse_node_multi_nm(
    priv_: *mut simple_util_priv,
    dai_link: *mut snd_soc_dai_link,
    nm_idx: *mut c_int,
    cpu_idx: c_int,
    mcpu_port: *mut device_node,
) -> c_int {
    let mcpu_ep = of_graph_get_next_port_endpoint(mcpu_port, null_mut());
    let mcpu_ports = port_to_ports(mcpu_port);
    let mcpu_port_top = of_graph_get_next_port(mcpu_ports, null_mut());
    let mcpu_ep_top = of_graph_get_next_port_endpoint(mcpu_port_top, null_mut());
    let mcodec_ep_top = of_graph_get_remote_endpoint(mcpu_ep_top);
    let mcodec_port_top = ep_to_port(mcodec_ep_top);
    let mcodec_ports = port_to_ports(mcodec_port_top);
    let nm_max = core::cmp::max((*dai_link).num_cpus, (*dai_link).num_codecs);
    let mut ret = -EINVAL;

    if cpu_idx > (*dai_link).num_cpus {
        return graph_ret(priv_, ret);
    }

    let mut mcpu_ep_n = of_graph_get_next_port_endpoint(mcpu_port, null_mut());
    while !mcpu_ep_n.is_null() {
        let mut codec_idx = 0;

        /* ignore 1st ep which is for element */
        if mcpu_ep_n == mcpu_ep {
            mcpu_ep_n = of_graph_get_next_port_endpoint(mcpu_port, mcpu_ep_n);
            continue;
        }

        if *nm_idx > nm_max {
            break;
        }

        let mcodec_ep_n = of_graph_get_remote_endpoint(mcpu_ep_n);
        let mcodec_port = ep_to_port(mcodec_ep_n);

        ret = -EINVAL;
        let tmp_ports = port_to_ports(mcodec_port);
        if mcodec_ports != tmp_ports {
            of_node_put(tmp_ports);
            break;
        }
        of_node_put(tmp_ports);

        let mut mcodec_port_i = of_graph_get_next_port(mcodec_ports, null_mut());
        while !mcodec_port_i.is_null() {
            /* ignore 1st port which is for pair connection */
            if mcodec_port_top == mcodec_port_i {
                mcodec_port_i = of_graph_get_next_port(mcodec_ports, mcodec_port_i);
                continue;
            }

            if codec_idx > (*dai_link).num_codecs {
                break;
            }

            if mcodec_port_i == mcodec_port {
                (*(*dai_link).ch_maps.add(*nm_idx as usize)).cpu = cpu_idx;
                (*(*dai_link).ch_maps.add(*nm_idx as usize)).codec = codec_idx;

                *nm_idx += 1;
                ret = 0;
                break;
            }
            codec_idx += 1;
            mcodec_port_i = of_graph_get_next_port(mcodec_ports, mcodec_port_i);
        }
        of_node_put(mcodec_ep_n);
        of_node_put(mcodec_port);
        if ret < 0 {
            break;
        }
        mcpu_ep_n = of_graph_get_next_port_endpoint(mcpu_port, mcpu_ep_n);
    }

    of_node_put(mcpu_ep);
    of_node_put(mcpu_ports);
    of_node_put(mcpu_port_top);
    of_node_put(mcpu_ep_top);
    of_node_put(mcodec_ep_top);
    of_node_put(mcodec_port_top);
    of_node_put(mcodec_ports);
    graph_ret(priv_, ret)
}

unsafe fn graph_parse_node_multi(
    priv_: *mut simple_util_priv,
    gtype: graph_type,
    mut port: *mut device_node,
    li: *mut link_info,
    is_cpu: c_int,
) -> c_int {
    let dai_link = simple_priv_to_link(priv_, (*li).link);
    let dev = simple_priv_to_dev(priv_);
    let mut ret = -ENOMEM;
    let mut nm_idx = 0;
    let nm_max = core::cmp::max((*dai_link).num_cpus, (*dai_link).num_codecs);

    /*
     * create ch_maps if CPU:Codec = N:M
     * DPCM is out of scope
     */
    if gtype != graph_type::GRAPH_DPCM && (*dai_link).ch_maps.is_null()
        && (*dai_link).num_cpus > 1 && (*dai_link).num_codecs > 1
        && (*dai_link).num_cpus != (*dai_link).num_codecs {
        (*dai_link).ch_maps = devm_kcalloc(dev, nm_max as usize, size_of::<snd_soc_dai_link_ch_map>(), GFP_KERNEL) as *mut snd_soc_dai_link_ch_map;
        if (*dai_link).ch_maps.is_null() {
            return graph_ret(priv_, ret);
        }
    }

    let mut idx = 0;
    loop {
        let ep = graph_get_next_multi_ep(&mut port, idx + 1);
        if ep.is_null() {
            break;
        }

        ret = __graph_parse_node(priv_, gtype, ep, li, is_cpu, idx);
        if ret < 0 {
            return graph_ret(priv_, ret);
        }

        /* CPU:Codec = N:M */
        if is_cpu != 0 && !(*dai_link).ch_maps.is_null() {
            ret = graph_parse_node_multi_nm(priv_, dai_link, &mut nm_idx, idx, port);
            if ret < 0 {
                return graph_ret(priv_, ret);
            }
        }
        idx += 1;
    }

    if is_cpu != 0 && !(*dai_link).ch_maps.is_null() && nm_idx != nm_max {
        ret = -EINVAL;
    }

    graph_ret(priv_, ret)
}

unsafe fn graph_parse_node_single(
    priv_: *mut simple_util_priv,
    gtype: graph_type,
    ep: *mut device_node,
    li: *mut link_info,
    is_cpu: c_int,
) -> c_int {
    graph_ret(priv_, __graph_parse_node(priv_, gtype, ep, li, is_cpu, 0))
}

unsafe fn graph_parse_node(
    priv_: *mut simple_util_priv,
    gtype: graph_type,
    ep: *mut device_node,
    li: *mut link_info,
    is_cpu: c_int,
) -> c_int {
    let port = ep_to_port(ep);
    let ret;

    if graph_lnk_is_multi(port) != 0 {
        ret = graph_parse_node_multi(priv_, gtype, port, li, is_cpu);
    } else {
        ret = graph_parse_node_single(priv_, gtype, ep, li, is_cpu);
    }

    of_node_put(port);
    graph_ret(priv_, ret)
}

unsafe fn graph_parse_daifmt(node: *mut device_node, daifmt: *mut c_uint) {
    let fmt: c_uint;

    if node.is_null() {
        return;
    }

    /*
     * format
     *
     * This function is called by (C) -> (B) -> (A) order.
     * Set if applicable part was not yet set.
     */
    fmt = snd_soc_daifmt_parse_format(node, null());
    if (*daifmt & SND_SOC_DAIFMT_FORMAT_MASK) == 0 && (fmt & SND_SOC_DAIFMT_FORMAT_MASK) != 0 {
        *daifmt |= fmt & SND_SOC_DAIFMT_FORMAT_MASK;
    }
    if (*daifmt & SND_SOC_DAIFMT_CLOCK_MASK) == 0 && (fmt & SND_SOC_DAIFMT_CLOCK_MASK) != 0 {
        *daifmt |= fmt & SND_SOC_DAIFMT_CLOCK_MASK;
    }
    if (*daifmt & SND_SOC_DAIFMT_INV_MASK) == 0 && (fmt & SND_SOC_DAIFMT_INV_MASK) != 0 {
        *daifmt |= fmt & SND_SOC_DAIFMT_INV_MASK;
    }
}

unsafe fn graph_parse_bitframe(ep: *mut device_node) -> c_uint {
    let port = ep_to_port(ep);
    let ports = port_to_ports(port);
    let ret = snd_soc_daifmt_clock_provider_from_bitmap(
        snd_soc_daifmt_parse_clock_provider_as_bitmap(ep, null())
            | snd_soc_daifmt_parse_clock_provider_as_bitmap(port, null())
            | snd_soc_daifmt_parse_clock_provider_as_bitmap(ports, null()),
    );
    of_node_put(ports);
    of_node_put(port);
    ret
}

unsafe fn graph_link_init(
    priv_: *mut simple_util_priv,
    lnk: *mut device_node,
    mut ep_cpu: *mut device_node,
    mut ep_codec: *mut device_node,
    li: *mut link_info,
    _is_cpu_node: c_int,
) {
    let dai_link = simple_priv_to_link(priv_, (*li).link);
    let dai_props = simple_priv_to_props(priv_, (*li).link);
    let mut port_cpu = ep_to_port(ep_cpu);
    let mut port_codec = ep_to_port(ep_codec);
    let mut multi_cpu_port: *mut device_node = null_mut();
    let mut multi_codec_port: *mut device_node = null_mut();
    let mut daifmt: c_uint = 0;
    let mut playback_only: bool_ = 0;
    let mut capture_only: bool_ = 0;
    let mut trigger_start = snd_soc_trigger_order::SND_SOC_TRIGGER_ORDER_DEFAULT;
    let mut trigger_stop = snd_soc_trigger_order::SND_SOC_TRIGGER_ORDER_DEFAULT;
    let mut multi_cpu_port_idx = 1;
    let mut multi_codec_port_idx = 1;

    if graph_lnk_is_multi(port_cpu) != 0 {
        multi_cpu_port = port_cpu;
        ep_cpu = graph_get_next_multi_ep(&mut multi_cpu_port, multi_cpu_port_idx);
        multi_cpu_port_idx += 1;
        of_node_put(port_cpu);
        port_cpu = ep_to_port(ep_cpu);
    } else {
        of_node_get(ep_cpu);
    }
    let ports_cpu = port_to_ports(port_cpu);

    if graph_lnk_is_multi(port_codec) != 0 {
        multi_codec_port = port_codec;
        ep_codec = graph_get_next_multi_ep(&mut multi_codec_port, multi_codec_port_idx);
        multi_codec_port_idx += 1;
        of_node_put(port_codec);
        port_codec = ep_to_port(ep_codec);
    } else {
        of_node_get(ep_codec);
    }
    let ports_codec = port_to_ports(port_codec);

    graph_parse_daifmt(ep_cpu, &mut daifmt);
    graph_parse_daifmt(ep_codec, &mut daifmt);
    graph_parse_daifmt(port_cpu, &mut daifmt);
    graph_parse_daifmt(port_codec, &mut daifmt);
    graph_parse_daifmt(ports_cpu, &mut daifmt);
    graph_parse_daifmt(ports_codec, &mut daifmt);
    graph_parse_daifmt(lnk, &mut daifmt);

    graph_util_parse_link_direction(lnk, &mut playback_only, &mut capture_only);
    graph_util_parse_link_direction(ports_cpu, &mut playback_only, &mut capture_only);
    graph_util_parse_link_direction(ports_codec, &mut playback_only, &mut capture_only);
    graph_util_parse_link_direction(port_cpu, &mut playback_only, &mut capture_only);
    graph_util_parse_link_direction(port_codec, &mut playback_only, &mut capture_only);
    graph_util_parse_link_direction(ep_cpu, &mut playback_only, &mut capture_only);
    graph_util_parse_link_direction(ep_codec, &mut playback_only, &mut capture_only);

    of_property_read_u32(lnk, b"mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(ports_cpu, b"mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(ports_codec, b"mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(port_cpu, b"mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(port_codec, b"mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(ep_cpu, b"mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);
    of_property_read_u32(ep_codec, b"mclk-fs\0".as_ptr() as *const c_char, &mut (*dai_props).mclk_fs);

    graph_util_parse_trigger_order(priv_, lnk, &mut trigger_start, &mut trigger_stop);
    graph_util_parse_trigger_order(priv_, ports_cpu, &mut trigger_start, &mut trigger_stop);
    graph_util_parse_trigger_order(priv_, ports_codec, &mut trigger_start, &mut trigger_stop);
    graph_util_parse_trigger_order(priv_, port_cpu, &mut trigger_start, &mut trigger_stop);
    graph_util_parse_trigger_order(priv_, port_cpu, &mut trigger_start, &mut trigger_stop);
    graph_util_parse_trigger_order(priv_, ep_cpu, &mut trigger_start, &mut trigger_stop);
    graph_util_parse_trigger_order(priv_, ep_codec, &mut trigger_start, &mut trigger_stop);

    let mut i = 0;
    while i < (*dai_link).num_cpus {
        let dlc = snd_soc_link_to_cpu(dai_link, i);
        (*dlc).ext_fmt = graph_parse_bitframe(ep_cpu);

        if !multi_cpu_port.is_null() {
            ep_cpu = graph_get_next_multi_ep(&mut multi_cpu_port, multi_cpu_port_idx);
            multi_cpu_port_idx += 1;
        }
        i += 1;
    }

    i = 0;
    while i < (*dai_link).num_codecs {
        let dlc = snd_soc_link_to_codec(dai_link, i);
        (*dlc).ext_fmt = graph_parse_bitframe(ep_codec);

        if !multi_codec_port.is_null() {
            ep_codec = graph_get_next_multi_ep(&mut multi_codec_port, multi_codec_port_idx);
            multi_codec_port_idx += 1;
        }
        i += 1;
    }

    /*** Don't use port_cpu / port_codec after here ***/

    (*dai_link).playback_only = playback_only;
    (*dai_link).capture_only = capture_only;
    (*dai_link).trigger_start = trigger_start;
    (*dai_link).trigger_stop = trigger_stop;
    (*dai_link).dai_fmt = daifmt;
    (*dai_link).init = Some(simple_util_dai_init);
    (*dai_link).ops = &graph_ops;
    if !(*priv_).ops.is_null() {
        (*dai_link).ops = (*priv_).ops;
    }

    of_node_put(port_cpu);
    of_node_put(port_codec);
    of_node_put(ep_cpu);
    of_node_put(ep_codec);
    of_node_put(ports_cpu);
    of_node_put(ports_codec);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn audio_graph2_link_normal(
    priv_: *mut simple_util_priv,
    lnk: *mut device_node,
    li: *mut link_info,
) -> c_int {
    let cpu_port = lnk;
    let cpu_ep = of_graph_get_next_port_endpoint(cpu_port, null_mut());
    let codec_ep = of_graph_get_remote_endpoint(cpu_ep);
    let mut ret: c_int;

    /*
     * call Codec first.
     * see
     *	__graph_parse_node() :: DAI Naming
     */
    ret = graph_parse_node(priv_, graph_type::GRAPH_NORMAL, codec_ep, li, 0);
    if ret >= 0 {
        /*
         * call CPU, and set DAI Name
         */
        ret = graph_parse_node(priv_, graph_type::GRAPH_NORMAL, cpu_ep, li, 1);
        if ret >= 0 {
            graph_link_init(priv_, lnk, cpu_ep, codec_ep, li, 1);
        }
    }

    of_node_put(cpu_ep);
    of_node_put(codec_ep);
    graph_ret(priv_, ret)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn audio_graph2_link_dpcm(
    priv_: *mut simple_util_priv,
    lnk: *mut device_node,
    li: *mut link_info,
) -> c_int {
    let ep = of_graph_get_next_port_endpoint(lnk, null_mut());
    let rep = of_graph_get_remote_endpoint(ep);
    let mut cpu_ep: *mut device_node = null_mut();
    let mut codec_ep: *mut device_node = null_mut();
    let dai_link = simple_priv_to_link(priv_, (*li).link);
    let dai_props = simple_priv_to_props(priv_, (*li).link);
    let is_cpu = graph_util_is_ports0(lnk);
    let ret: c_int;

    if is_cpu != 0 {
        cpu_ep = rep;
        (*dai_link).dynamic = 1;
        (*dai_link).dpcm_merged_format = 1;

        ret = graph_parse_node(priv_, graph_type::GRAPH_DPCM, cpu_ep, li, 1);
        if ret != 0 {
            return ret;
        }
    } else {
        codec_ep = rep;
        (*dai_link).no_pcm = 1;
        (*dai_link).be_hw_params_fixup = Some(simple_util_be_hw_params_fixup);

        ret = graph_parse_node(priv_, graph_type::GRAPH_DPCM, codec_ep, li, 0);
        if ret < 0 {
            return ret;
        }
    }

    graph_parse_convert(ep, dai_props); /* at node of <dpcm> */
    graph_parse_convert(rep, dai_props); /* at node of <CPU/Codec> */

    graph_link_init(priv_, lnk, cpu_ep, codec_ep, li, is_cpu);
    of_node_put(ep);
    of_node_put(rep);

    graph_ret(priv_, ret)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn audio_graph2_link_c2c(
    priv_: *mut simple_util_priv,
    lnk: *mut device_node,
    li: *mut link_info,
) -> c_int {
    let dai_link = simple_priv_to_link(priv_, (*li).link);
    let port0 = lnk;
    let ports = port_to_ports(port0);
    let port1 = of_graph_get_next_port(ports, port0);
    let mut val: u32 = 0;
    let mut ret = -EINVAL;

    of_property_read_u32(ports, b"rate\0".as_ptr() as *const c_char, &mut val);
    if val != 0 {
        let dev = simple_priv_to_dev(priv_);
        let c2c_conf = devm_kzalloc(dev, size_of::<snd_soc_pcm_stream>(), GFP_KERNEL) as *mut snd_soc_pcm_stream;
        if c2c_conf.is_null() {
            return graph_ret(priv_, -ENOMEM);
        }

        (*c2c_conf).formats = SNDRV_PCM_FMTBIT_S32_LE; /* update ME */
        (*c2c_conf).rates = SNDRV_PCM_RATE_8000_384000;
        (*c2c_conf).rate_min = val;
        (*c2c_conf).rate_max = val;
        (*c2c_conf).channels_min = 2; /* update ME */
        (*c2c_conf).channels_max = 2;

        (*dai_link).c2c_params = c2c_conf;
        (*dai_link).num_c2c_params = 1;
    }

    let ep0 = of_graph_get_next_port_endpoint(port0, null_mut());
    let ep1 = of_graph_get_next_port_endpoint(port1, null_mut());
    let codec0_ep = of_graph_get_remote_endpoint(ep0);
    let codec1_ep = of_graph_get_remote_endpoint(ep1);

    /*
     * call Codec first.
     * see
     *	__graph_parse_node() :: DAI Naming
     */
    ret = graph_parse_node(priv_, graph_type::GRAPH_C2C, codec1_ep, li, 0);
    if ret >= 0 {
        /*
         * call CPU, and set DAI Name
         */
        ret = graph_parse_node(priv_, graph_type::GRAPH_C2C, codec0_ep, li, 1);
        if ret >= 0 {
            graph_link_init(priv_, lnk, codec0_ep, codec1_ep, li, 1);
        }
    }

    of_node_put(ports);
    of_node_put(port1);
    of_node_put(ep0);
    of_node_put(ep1);
    of_node_put(codec0_ep);
    of_node_put(codec1_ep);
    graph_ret(priv_, ret)
}

unsafe fn graph_link(
    priv_: *mut simple_util_priv,
    hooks: *mut graph2_custom_hooks,
    gtype: graph_type,
    lnk: *mut device_node,
    li: *mut link_info,
) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let mut func: GRAPH2_CUSTOM = None;
    let mut ret = -EINVAL;

    match gtype {
        graph_type::GRAPH_NORMAL => {
            if !hooks.is_null() && (*hooks).custom_normal.is_some() { func = (*hooks).custom_normal; } else { func = Some(audio_graph2_link_normal); }
        }
        graph_type::GRAPH_DPCM => {
            if !hooks.is_null() && (*hooks).custom_dpcm.is_some() { func = (*hooks).custom_dpcm; } else { func = Some(audio_graph2_link_dpcm); }
        }
        graph_type::GRAPH_C2C => {
            if !hooks.is_null() && (*hooks).custom_c2c.is_some() { func = (*hooks).custom_c2c; } else { func = Some(audio_graph2_link_c2c); }
        }
        _ => {}
    }

    if func.is_none() {
        dev_err(dev, b"non supported gtype (%d)\n\0".as_ptr() as *const c_char, gtype as c_int);
        return graph_ret(priv_, ret);
    }

    ret = func.unwrap()(priv_, lnk, li);
    if ret >= 0 {
        (*li).link += 1;
    }
    graph_ret(priv_, ret)
}

unsafe fn graph_counter(lnk: *mut device_node) -> c_int {
    /*
     * Multi CPU / Codec
     * ignore first lnk part
     */
    if graph_lnk_is_multi(lnk) != 0 {
        let ports = port_to_ports(lnk);
        /*
         * CPU/Codec = N:M case has many endpoints.
         * We can't use of_graph_get_endpoint_count() here
         */
        let ret = of_graph_get_port_count(ports) - 1;
        of_node_put(ports);
        ret
    } else {
        /*
         * Single CPU / Codec
         */
        1
    }
}

unsafe fn graph_count_normal(
    _priv: *mut simple_util_priv,
    lnk: *mut device_node,
    li: *mut link_info,
) -> c_int {
    let cpu_port = lnk;
    let cpu_ep = of_graph_get_next_port_endpoint(cpu_port, null_mut());
    let codec_port = of_graph_get_remote_port(cpu_ep);

    /*
     * DON'T REMOVE platforms
     * see
     *	simple-card.c :: simple_count_noml()
     */
    (*li).num[(*li).link as usize].cpus = graph_counter(cpu_port);
    (*li).num[(*li).link as usize].platforms = (*li).num[(*li).link as usize].cpus;
    (*li).num[(*li).link as usize].codecs = graph_counter(codec_port);

    of_node_put(cpu_ep);
    of_node_put(codec_port);
    0
}

unsafe fn graph_count_dpcm(
    _priv: *mut simple_util_priv,
    lnk: *mut device_node,
    li: *mut link_info,
) -> c_int {
    let ep = of_graph_get_next_port_endpoint(lnk, null_mut());
    let rport = of_graph_get_remote_port(ep);

    if graph_util_is_ports0(lnk) != 0 {
        /*
         * DON'T REMOVE platforms
         * see
         *	simple-card.c :: simple_count_noml()
         */
        (*li).num[(*li).link as usize].cpus = graph_counter(rport); /* FE */
        (*li).num[(*li).link as usize].platforms = graph_counter(rport);
    } else {
        (*li).num[(*li).link as usize].codecs = graph_counter(rport); /* BE */
    }

    of_node_put(ep);
    of_node_put(rport);
    0
}

unsafe fn graph_count_c2c(
    _priv: *mut simple_util_priv,
    lnk: *mut device_node,
    li: *mut link_info,
) -> c_int {
    let ports = port_to_ports(lnk);
    let port0 = of_node_get(lnk);
    let port1 = of_node_get(of_graph_get_next_port(ports, of_node_get(port0)));
    let ep0 = of_graph_get_next_port_endpoint(port0, null_mut());
    let ep1 = of_graph_get_next_port_endpoint(port1, null_mut());
    let codec0 = of_graph_get_remote_port(ep0);
    let codec1 = of_graph_get_remote_port(ep1);

    /*
     * DON'T REMOVE platforms
     * see
     *	simple-card.c :: simple_count_noml()
     */
    (*li).num[(*li).link as usize].cpus = graph_counter(codec0);
    (*li).num[(*li).link as usize].platforms = (*li).num[(*li).link as usize].cpus;
    (*li).num[(*li).link as usize].codecs = graph_counter(codec1);

    of_node_put(ports);
    of_node_put(port0);
    of_node_put(port1);
    of_node_put(ep0);
    of_node_put(ep1);
    of_node_put(codec0);
    of_node_put(codec1);
    0
}

type graph_each_func = unsafe fn(
    priv_: *mut simple_util_priv,
    hooks: *mut graph2_custom_hooks,
    gtype: graph_type,
    lnk: *mut device_node,
    li: *mut link_info,
) -> c_int;

unsafe fn graph_count(
    priv_: *mut simple_util_priv,
    _hooks: *mut graph2_custom_hooks,
    gtype: graph_type,
    lnk: *mut device_node,
    li: *mut link_info,
) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let mut func: Option<unsafe fn(*mut simple_util_priv, *mut device_node, *mut link_info) -> c_int> = None;
    let mut ret = -EINVAL;

    if (*li).link >= SNDRV_MAX_LINKS {
        dev_err(dev, b"too many links\n\0".as_ptr() as *const c_char);
        return ret;
    }

    match gtype {
        graph_type::GRAPH_NORMAL => func = Some(graph_count_normal),
        graph_type::GRAPH_DPCM => func = Some(graph_count_dpcm),
        graph_type::GRAPH_C2C => func = Some(graph_count_c2c),
        _ => {}
    }

    if func.is_none() {
        dev_err(dev, b"non supported gtype (%d)\n\0".as_ptr() as *const c_char, gtype as c_int);
        return graph_ret(priv_, ret);
    }

    ret = func.unwrap()(priv_, lnk, li);
    if ret >= 0 {
        (*li).link += 1;
    }
    graph_ret(priv_, ret)
}

unsafe fn graph_for_each_link(
    priv_: *mut simple_util_priv,
    hooks: *mut graph2_custom_hooks,
    li: *mut link_info,
    func: graph_each_func,
) -> c_int {
    let mut it: of_phandle_iterator = zeroed();
    let dev = simple_priv_to_dev(priv_);
    let node = (*dev).of_node;
    let mut ret = 0;

    /* loop for all listed CPU port */
    // C used of_for_each_phandle(&it, rc, node, "links", NULL, 0).
    // The iterator primitive is external to this file; this preserves the loop intent.
    unsafe extern "C" {
        fn of_phandle_iterator_init(it: *mut of_phandle_iterator, node: *mut device_node, list_name: *const c_char, cells_name: *const c_char, cell_count: c_int) -> c_int;
        fn of_phandle_iterator_next(it: *mut of_phandle_iterator) -> c_int;
    }
    let mut rc = of_phandle_iterator_init(&mut it, node, b"links\0".as_ptr() as *const c_char, null(), 0);
    while rc == 0 {
        let lnk = it.node;
        let gtype = graph_get_type(priv_, lnk);

        ret = func(priv_, hooks, gtype, lnk, li);
        if ret < 0 {
            break;
        }
        rc = of_phandle_iterator_next(&mut it);
    }

    graph_ret(priv_, ret)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn audio_graph2_parse_of(
    priv_: *mut simple_util_priv,
    dev: *mut device,
    hooks: *mut graph2_custom_hooks,
) -> c_int {
    let card = simple_priv_to_card(priv_);
    let mut ret = -ENOMEM;
    let li = kzalloc(size_of::<link_info>(), GFP_KERNEL) as *mut link_info;
    if li.is_null() {
        return graph_ret(priv_, ret);
    }

    (*card).probe = Some(graph_util_card_probe);
    (*card).owner = THIS_MODULE;
    (*card).dev = dev;

    if !hooks.is_null() && (*hooks).hook_pre.is_some() {
        ret = (*hooks).hook_pre.unwrap()(priv_);
        if ret < 0 {
            kfree(li as *mut c_void);
            return graph_ret(priv_, ret);
        }
    }

    ret = graph_for_each_link(priv_, hooks, li, graph_count);
    if (*li).link == 0 {
        ret = -EINVAL;
    }
    if ret < 0 {
        kfree(li as *mut c_void);
        return graph_ret(priv_, ret);
    }

    ret = simple_util_init_priv(priv_, li);
    if ret < 0 {
        kfree(li as *mut c_void);
        return graph_ret(priv_, ret);
    }

    (*priv_).pa_gpio = devm_gpiod_get_optional(dev, b"pa\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*priv_).pa_gpio) != 0 {
        ret = PTR_ERR((*priv_).pa_gpio);
        dev_err(dev, b"failed to get amplifier gpio: %d\n\0".as_ptr() as *const c_char, ret);
        kfree(li as *mut c_void);
        return graph_ret(priv_, ret);
    }

    ret = simple_util_parse_widgets(priv_, null());
    if ret < 0 {
        kfree(li as *mut c_void);
        return graph_ret(priv_, ret);
    }

    ret = simple_util_parse_routing(priv_, null());
    if ret < 0 {
        kfree(li as *mut c_void);
        return graph_ret(priv_, ret);
    }

    ret = simple_util_parse_aux_devs(priv_, null());
    if ret < 0 {
        simple_util_clean_reference(priv_);
        kfree(li as *mut c_void);
        return dev_err_probe(dev, ret, b"parse error\n\0".as_ptr() as *const c_char);
    }

    core::ptr::write_bytes(li as *mut u8, 0, size_of::<link_info>());
    ret = graph_for_each_link(priv_, hooks, li, graph_link);
    if ret < 0 {
        simple_util_clean_reference(priv_);
        kfree(li as *mut c_void);
        return dev_err_probe(dev, ret, b"parse error\n\0".as_ptr() as *const c_char);
    }

    /* Card name should be set after graph_for_each_link() */
    ret = simple_util_parse_card_name(priv_, null());
    if ret < 0 {
        simple_util_clean_reference(priv_);
        kfree(li as *mut c_void);
        return dev_err_probe(dev, ret, b"parse error\n\0".as_ptr() as *const c_char);
    }

    snd_soc_card_set_drvdata(card, priv_);

    if !hooks.is_null() && (*hooks).hook_post.is_some() {
        ret = (*hooks).hook_post.unwrap()(priv_);
        if ret < 0 {
            simple_util_clean_reference(priv_);
            kfree(li as *mut c_void);
            return dev_err_probe(dev, ret, b"parse error\n\0".as_ptr() as *const c_char);
        }
    }

    simple_util_debug_info(priv_);

    ret = devm_snd_soc_register_card(dev, card);
    if ret < 0 {
        simple_util_clean_reference(priv_);
        kfree(li as *mut c_void);
        return dev_err_probe(dev, ret, b"parse error\n\0".as_ptr() as *const c_char);
    }

    kfree(li as *mut c_void);
    graph_ret(priv_, ret)
}

unsafe extern "C" fn graph_probe(pdev: *mut platform_device) -> c_int {
    let priv_: *mut simple_util_priv;
    let dev = &mut (*pdev).dev as *mut device;

    /* Allocate the private data and the DAI link array */
    priv_ = devm_kzalloc(dev, size_of::<simple_util_priv>(), GFP_KERNEL) as *mut simple_util_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    audio_graph2_parse_of(priv_, dev, null_mut())
}

static graph_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"audio-graph-card2\0".as_ptr() as *const c_char },
    of_device_id { compatible: null() },
];
// MODULE_DEVICE_TABLE(of, graph_of_match);

static mut graph_card: platform_driver = platform_driver {
    driver: driver_private {
        name: b"asoc-audio-graph-card2\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
        of_match_table: graph_of_match.as_ptr(),
    },
    probe: Some(graph_probe),
    remove: Some(simple_util_remove),
};
// module_platform_driver(graph_card);

// MODULE_ALIAS("platform:asoc-audio-graph-card2");
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("ASoC Audio Graph Card2");
// MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
