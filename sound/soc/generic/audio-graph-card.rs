// SPDX-License-Identifier: GPL-2.0
//
// ASoC audio graph sound card support
//
// Copyright (C) 2016 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// based on ${LINUX}/sound/soc/generic/simple-card.c

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const DPCM_SELECTABLE: usize = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const SNDRV_MAX_LINKS: c_int = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 0;
const SND_SOC_DAPM_PRE_PMD: c_int = 0;
const SND_SOC_NOPM: c_int = 0;
const SND_SOC_TRIGGER_ORDER_DEFAULT: snd_soc_trigger_order = 0;
const THIS_MODULE: *mut c_void = ptr::null_mut();

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
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

pub type snd_soc_trigger_order = c_int;

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn() -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn()>,
    pub hw_params: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_init {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub dapm_widgets: *const snd_soc_dapm_widget_init,
    pub num_dapm_widgets: c_int,
    pub probe: Option<unsafe extern "C" fn() -> c_int>,
    pub component_chaining: bool,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub pcm_new: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)]
pub struct snd_soc_component {
    pub driver: *mut snd_soc_component_driver,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub pcm_new: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub ops: *mut snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub driver: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub dai_fmt: c_uint,
    pub playback_only: bool,
    pub capture_only: bool,
    pub trigger_start: snd_soc_trigger_order,
    pub trigger_stop: snd_soc_trigger_order,
    pub init: Option<unsafe extern "C" fn() -> c_int>,
    pub ops: *const snd_soc_ops,
    pub dynamic: c_uint,
    pub dpcm_merged_format: c_uint,
    pub no_pcm: c_uint,
    pub be_hw_params_fixup: Option<unsafe extern "C" fn() -> c_int>,
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
pub struct snd_soc_codec_conf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct simple_dai_props {
    pub mclk_fs: c_uint,
    pub adata: simple_util_data,
}

#[repr(C)]
pub struct simple_util_priv {
    pub pa_gpio: *mut gpio_desc,
    pub ops: *const snd_soc_ops,
    pub force_dpcm: bool,
    pub dpcm_selectable: c_uint,
}

#[repr(C)]
pub struct link_info_num {
    pub cpus: c_int,
    pub platforms: c_int,
    pub codecs: c_int,
}

#[repr(C)]
pub struct link_info {
    pub link: c_int,
    pub cpu: c_int,
    pub num: [link_info_num; 0],
}

#[repr(C)]
pub struct of_phandle_iterator {
    pub node: *mut device_node,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const c_void,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn() -> c_int>,
}

type LinkFunc = unsafe extern "C" fn(
    *mut simple_util_priv,
    *mut device_node,
    *mut device_node,
    *mut link_info,
) -> c_int;

unsafe extern "C" {
    static snd_soc_pm_ops: c_void;
    static graph_dapm_widgets: [snd_soc_dapm_widget_init; 1];

    fn snd_soc_ret(dev: *mut device, ret: c_int, fmt: *const c_char, ...) -> c_int;
    fn simple_priv_to_dev(priv_: *mut simple_util_priv) -> *mut device;
    fn of_get_parent(node: *mut device_node) -> *mut device_node;
    fn of_node_name_eq(node: *mut device_node, name: *const c_char) -> bool;
    fn of_node_put(node: *mut device_node);
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut simple_util_priv;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn snd_soc_find_dai_with_mutex(dlc: *mut snd_soc_dai_link_component) -> *mut snd_soc_dai;
    fn of_graph_get_port_parent(ep: *mut device_node) -> *mut device_node;
    fn simple_util_parse_convert(
        node: *mut device_node,
        prefix: *const c_char,
        adata: *mut simple_util_data,
    );
    fn simple_priv_to_link(priv_: *mut simple_util_priv, link: c_int) -> *mut snd_soc_dai_link;
    fn simple_priv_to_props(priv_: *mut simple_util_priv, link: c_int) -> *mut simple_dai_props;
    fn snd_soc_link_to_cpu(
        dai_link: *mut snd_soc_dai_link,
        n: c_int,
    ) -> *mut snd_soc_dai_link_component;
    fn snd_soc_link_to_codec(
        dai_link: *mut snd_soc_dai_link,
        n: c_int,
    ) -> *mut snd_soc_dai_link_component;
    fn snd_soc_link_to_platform(
        dai_link: *mut snd_soc_dai_link,
        n: c_int,
    ) -> *mut snd_soc_dai_link_component;
    fn simple_props_to_dai_cpu(props: *mut simple_dai_props, n: c_int) -> *mut simple_util_dai;
    fn simple_props_to_dai_codec(props: *mut simple_dai_props, n: c_int) -> *mut simple_util_dai;
    fn graph_util_parse_dai(
        priv_: *mut simple_util_priv,
        ep: *mut device_node,
        dlc: *mut snd_soc_dai_link_component,
        cpu: *mut c_int,
    ) -> c_int;
    fn simple_util_parse_tdm(ep: *mut device_node, dai: *mut simple_util_dai) -> c_int;
    fn simple_util_parse_clk(
        dev: *mut device,
        ep: *mut device_node,
        dai: *mut simple_util_dai,
        dlc: *mut snd_soc_dai_link_component,
    ) -> c_int;
    fn simple_util_parse_daifmt(
        dev: *mut device,
        ep_cpu: *mut device_node,
        ep_codec: *mut device_node,
        prefix: *const c_char,
        dai_fmt: *mut c_uint,
    ) -> c_int;
    fn graph_util_parse_link_direction(
        node: *mut device_node,
        playback_only: *mut bool,
        capture_only: *mut bool,
    );
    fn of_property_read_u32(node: *mut device_node, prop: *const c_char, out: *mut c_uint) -> c_int;
    fn graph_util_parse_trigger_order(
        priv_: *mut simple_util_priv,
        node: *mut device_node,
        trigger_start: *mut snd_soc_trigger_order,
        trigger_stop: *mut snd_soc_trigger_order,
    );
    fn simple_util_set_dailink_name(
        priv_: *mut simple_util_priv,
        dai_link: *mut snd_soc_dai_link,
        name: *mut c_char,
    ) -> c_int;
    fn simple_priv_to_card(priv_: *mut simple_util_priv) -> *mut snd_soc_card;
    fn simple_util_canonicalize_cpu(cpus: *mut snd_soc_dai_link_component, is_single_links: c_int);
    fn simple_util_canonicalize_platform(
        platforms: *mut snd_soc_dai_link_component,
        cpus: *mut snd_soc_dai_link_component,
    );
    fn simple_props_to_codec_conf(
        props: *mut simple_dai_props,
        n: c_int,
    ) -> *mut snd_soc_codec_conf;
    fn snd_soc_of_parse_node_prefix(
        top: *mut device_node,
        cconf: *mut snd_soc_codec_conf,
        node: *mut device_node,
        prop: *const c_char,
    );
    fn simple_util_is_convert_required(adata: *mut simple_util_data) -> bool;
    fn of_get_child_count(node: *mut device_node) -> c_uint;
    fn of_graph_get_remote_endpoint(ep: *mut device_node) -> *mut device_node;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint)
        -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn simple_util_parse_widgets(priv_: *mut simple_util_priv, prefix: *const c_char) -> c_int;
    fn simple_util_parse_routing(priv_: *mut simple_util_priv, prefix: *const c_char) -> c_int;
    fn simple_util_init_priv(priv_: *mut simple_util_priv, li: *mut link_info) -> c_int;
    fn simple_util_parse_card_name(priv_: *mut simple_util_priv, prefix: *const c_char) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut simple_util_priv);
    fn simple_util_debug_info(priv_: *mut simple_util_priv);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn simple_util_clean_reference(priv_: *mut simple_util_priv);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn simple_util_startup() -> c_int;
    fn simple_util_shutdown();
    fn simple_util_hw_params() -> c_int;
    fn simple_util_dai_init() -> c_int;
    fn simple_util_be_hw_params_fixup() -> c_int;
    fn graph_util_card_probe() -> c_int;
    fn simple_util_remove() -> c_int;
}

unsafe fn graph_ret(priv_: *mut simple_util_priv, ret: c_int) -> c_int {
    _graph_ret(priv_, c"_graph_ret".as_ptr(), ret)
}

unsafe fn _graph_ret(priv_: *mut simple_util_priv, func: *const c_char, ret: c_int) -> c_int {
    unsafe { snd_soc_ret(simple_priv_to_dev(priv_), ret, c"at %s()\n".as_ptr(), func) }
}

unsafe fn ep_to_port(ep: *mut device_node) -> *mut device_node {
    unsafe { of_get_parent(ep) }
}

unsafe extern "C" fn port_to_ports(port: *mut device_node) -> *mut device_node {
    let ports = unsafe { of_get_parent(port) };

    if unsafe { !of_node_name_eq(ports, c"ports".as_ptr()) } {
        unsafe { of_node_put(ports) };
        return ptr::null_mut();
    }
    ports
}

unsafe extern "C" fn graph_outdrv_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = unsafe { snd_soc_dapm_to_card((*w).dapm) };
    let priv_ = unsafe { snd_soc_card_get_drvdata(card) };

    match event {
        SND_SOC_DAPM_POST_PMU => unsafe { gpiod_set_value_cansleep((*priv_).pa_gpio, 1) },
        SND_SOC_DAPM_PRE_PMD => unsafe { gpiod_set_value_cansleep((*priv_).pa_gpio, 0) },
        _ => return -EINVAL,
    }

    0
}

static graph_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(simple_util_startup),
    shutdown: Some(simple_util_shutdown),
    hw_params: Some(simple_util_hw_params),
};

unsafe extern "C" fn soc_component_is_pcm(dlc: *mut snd_soc_dai_link_component) -> bool {
    let dai = unsafe { snd_soc_find_dai_with_mutex(dlc) };

    if !dai.is_null()
        && unsafe {
            !(*dai).component.is_null()
                && !(*(*dai).component).driver.is_null()
                && ((*(*(*dai).component).driver).pcm_new.is_some()
                    || (!(*dai).driver.is_null()
                        && !(*(*dai).driver).ops.is_null()
                        && (*(*(*dai).driver).ops).pcm_new.is_some()))
        }
    {
        return true;
    }

    false
}

unsafe extern "C" fn graph_parse_convert(
    dev: *mut device,
    ep: *mut device_node,
    adata: *mut simple_util_data,
) {
    let top = unsafe { (*dev).of_node };
    let port = unsafe { ep_to_port(ep) };
    let ports = unsafe { port_to_ports(port) };
    let _node = unsafe { of_graph_get_port_parent(ep) };

    unsafe {
        simple_util_parse_convert(top, ptr::null(), adata);
        simple_util_parse_convert(ports, ptr::null(), adata);
        simple_util_parse_convert(port, ptr::null(), adata);
        simple_util_parse_convert(ep, ptr::null(), adata);
    }
}

unsafe extern "C" fn graph_parse_node(
    priv_: *mut simple_util_priv,
    ep: *mut device_node,
    li: *mut link_info,
    cpu: *mut c_int,
) -> c_int {
    let dev = unsafe { simple_priv_to_dev(priv_) };
    let dai_link = unsafe { simple_priv_to_link(priv_, (*li).link) };
    let dai_props = unsafe { simple_priv_to_props(priv_, (*li).link) };
    let dlc: *mut snd_soc_dai_link_component;
    let dai: *mut simple_util_dai;
    let mut ret: c_int;

    if !cpu.is_null() {
        dlc = unsafe { snd_soc_link_to_cpu(dai_link, 0) };
        dai = unsafe { simple_props_to_dai_cpu(dai_props, 0) };
    } else {
        dlc = unsafe { snd_soc_link_to_codec(dai_link, 0) };
        dai = unsafe { simple_props_to_dai_codec(dai_props, 0) };
    }

    ret = unsafe { graph_util_parse_dai(priv_, ep, dlc, cpu) };
    if ret < 0 {
        return unsafe { graph_ret(priv_, ret) };
    }

    ret = unsafe { simple_util_parse_tdm(ep, dai) };
    if ret < 0 {
        return unsafe { graph_ret(priv_, ret) };
    }

    ret = unsafe { simple_util_parse_clk(dev, ep, dai, dlc) };
    unsafe { graph_ret(priv_, ret) }
}

unsafe extern "C" fn graph_link_init(
    priv_: *mut simple_util_priv,
    ep_cpu: *mut device_node,
    ep_codec: *mut device_node,
    li: *mut link_info,
    name: *mut c_char,
) -> c_int {
    let dev = unsafe { simple_priv_to_dev(priv_) };
    let top = unsafe { (*dev).of_node };
    let dai_link = unsafe { simple_priv_to_link(priv_, (*li).link) };
    let dai_props = unsafe { simple_priv_to_props(priv_, (*li).link) };
    let port_cpu = unsafe { ep_to_port(ep_cpu) };
    let port_codec = unsafe { ep_to_port(ep_codec) };
    let ports_cpu = unsafe { port_to_ports(port_cpu) };
    let ports_codec = unsafe { port_to_ports(port_codec) };
    let mut trigger_start = SND_SOC_TRIGGER_ORDER_DEFAULT;
    let mut trigger_stop = SND_SOC_TRIGGER_ORDER_DEFAULT;
    let mut playback_only = false;
    let mut capture_only = false;
    let mut ret: c_int;

    ret = unsafe {
        simple_util_parse_daifmt(dev, ep_cpu, ep_codec, ptr::null(), &mut (*dai_link).dai_fmt)
    };
    if ret < 0 {
        return unsafe { graph_ret(priv_, ret) };
    }

    unsafe {
        graph_util_parse_link_direction(top, &mut playback_only, &mut capture_only);
        graph_util_parse_link_direction(port_cpu, &mut playback_only, &mut capture_only);
        graph_util_parse_link_direction(port_codec, &mut playback_only, &mut capture_only);
        graph_util_parse_link_direction(ep_cpu, &mut playback_only, &mut capture_only);
        graph_util_parse_link_direction(ep_codec, &mut playback_only, &mut capture_only);

        of_property_read_u32(top, c"mclk-fs".as_ptr(), &mut (*dai_props).mclk_fs);
        of_property_read_u32(ports_cpu, c"mclk-fs".as_ptr(), &mut (*dai_props).mclk_fs);
        of_property_read_u32(ports_codec, c"mclk-fs".as_ptr(), &mut (*dai_props).mclk_fs);
        of_property_read_u32(port_cpu, c"mclk-fs".as_ptr(), &mut (*dai_props).mclk_fs);
        of_property_read_u32(port_codec, c"mclk-fs".as_ptr(), &mut (*dai_props).mclk_fs);
        of_property_read_u32(ep_cpu, c"mclk-fs".as_ptr(), &mut (*dai_props).mclk_fs);
        of_property_read_u32(ep_codec, c"mclk-fs".as_ptr(), &mut (*dai_props).mclk_fs);

        graph_util_parse_trigger_order(priv_, top, &mut trigger_start, &mut trigger_stop);
        graph_util_parse_trigger_order(priv_, ports_cpu, &mut trigger_start, &mut trigger_stop);
        graph_util_parse_trigger_order(priv_, ports_codec, &mut trigger_start, &mut trigger_stop);
        graph_util_parse_trigger_order(priv_, port_cpu, &mut trigger_start, &mut trigger_stop);
        graph_util_parse_trigger_order(priv_, port_cpu, &mut trigger_start, &mut trigger_stop);
        graph_util_parse_trigger_order(priv_, ep_cpu, &mut trigger_start, &mut trigger_stop);
        graph_util_parse_trigger_order(priv_, ep_codec, &mut trigger_start, &mut trigger_stop);

        (*dai_link).playback_only = playback_only;
        (*dai_link).capture_only = capture_only;
        (*dai_link).trigger_start = trigger_start;
        (*dai_link).trigger_stop = trigger_stop;
        (*dai_link).init = Some(simple_util_dai_init);
        (*dai_link).ops = &graph_ops;
        if !(*priv_).ops.is_null() {
            (*dai_link).ops = (*priv_).ops;
        }

        ret = simple_util_set_dailink_name(priv_, dai_link, name);
        graph_ret(priv_, ret)
    }
}

unsafe extern "C" fn graph_dai_link_of_dpcm(
    priv_: *mut simple_util_priv,
    cpu_ep: *mut device_node,
    codec_ep: *mut device_node,
    li: *mut link_info,
) -> c_int {
    let dev = unsafe { simple_priv_to_dev(priv_) };
    let dai_link = unsafe { simple_priv_to_link(priv_, (*li).link) };
    let dai_props = unsafe { simple_priv_to_props(priv_, (*li).link) };
    let top = unsafe { (*dev).of_node };
    let ep = unsafe { if (*li).cpu != 0 { cpu_ep } else { codec_ep } };
    let mut dai_name = [0 as c_char; 64];
    let mut ret: c_int;

    unsafe { dev_dbg(dev, c"link_of DPCM (%pOF)\n".as_ptr(), ep) };

    if unsafe { (*li).cpu != 0 } {
        let card = unsafe { simple_priv_to_card(priv_) };
        let cpus = unsafe { snd_soc_link_to_cpu(dai_link, 0) };
        let platforms = unsafe { snd_soc_link_to_platform(dai_link, 0) };
        let mut is_single_links = 0;

        /* Codec is dummy */

        /* FE settings */
        unsafe {
            (*dai_link).dynamic = 1;
            (*dai_link).dpcm_merged_format = 1;
        }

        ret = unsafe { graph_parse_node(priv_, cpu_ep, li, &mut is_single_links) };
        if ret != 0 {
            return unsafe { graph_ret(priv_, ret) };
        }

        unsafe {
            snprintf(
                dai_name.as_mut_ptr(),
                dai_name.len(),
                c"fe.%pOFP.%s".as_ptr(),
                (*cpus).of_node,
                (*cpus).dai_name,
            );
        }
        /*
         * In BE<->BE connections it is not required to create
         * PCM devices at CPU end of the dai link and thus 'no_pcm'
         * flag needs to be set. It is useful when there are many
         * BE components and some of these have to be connected to
         * form a valid audio path.
         *
         * For example: FE <-> BE1 <-> BE2 <-> ... <-> BEn where
         * there are 'n' BE components in the path.
         */
        if unsafe { (*card).component_chaining && !soc_component_is_pcm(cpus) } {
            unsafe {
                (*dai_link).no_pcm = 1;
                (*dai_link).be_hw_params_fixup = Some(simple_util_be_hw_params_fixup);
            }
        }

        unsafe {
            simple_util_canonicalize_cpu(cpus, is_single_links);
            simple_util_canonicalize_platform(platforms, cpus);
        }
    } else {
        let cconf = unsafe { simple_props_to_codec_conf(dai_props, 0) };
        let codecs = unsafe { snd_soc_link_to_codec(dai_link, 0) };

        /* CPU is dummy */

        /* BE settings */
        unsafe {
            (*dai_link).no_pcm = 1;
            (*dai_link).be_hw_params_fixup = Some(simple_util_be_hw_params_fixup);
        }

        ret = unsafe { graph_parse_node(priv_, codec_ep, li, ptr::null_mut()) };
        if ret < 0 {
            return unsafe { graph_ret(priv_, ret) };
        }

        unsafe {
            snprintf(
                dai_name.as_mut_ptr(),
                dai_name.len(),
                c"be.%pOFP.%s".as_ptr(),
                (*codecs).of_node,
                (*codecs).dai_name,
            );
        }

        /* check "prefix" from top node */
        let port = unsafe { ep_to_port(ep) };
        let ports = unsafe { port_to_ports(port) };

        unsafe {
            snd_soc_of_parse_node_prefix(top, cconf, (*codecs).of_node, c"prefix".as_ptr());
            snd_soc_of_parse_node_prefix(ports, cconf, (*codecs).of_node, c"prefix".as_ptr());
            snd_soc_of_parse_node_prefix(port, cconf, (*codecs).of_node, c"prefix".as_ptr());
        }
    }

    unsafe {
        graph_parse_convert(dev, ep, &mut (*dai_props).adata);
        ret = graph_link_init(priv_, cpu_ep, codec_ep, li, dai_name.as_mut_ptr());
        (*li).link += 1;
        graph_ret(priv_, ret)
    }
}

unsafe extern "C" fn graph_dai_link_of(
    priv_: *mut simple_util_priv,
    cpu_ep: *mut device_node,
    codec_ep: *mut device_node,
    li: *mut link_info,
) -> c_int {
    let dev = unsafe { simple_priv_to_dev(priv_) };
    let dai_link = unsafe { simple_priv_to_link(priv_, (*li).link) };
    let cpus = unsafe { snd_soc_link_to_cpu(dai_link, 0) };
    let codecs = unsafe { snd_soc_link_to_codec(dai_link, 0) };
    let platforms = unsafe { snd_soc_link_to_platform(dai_link, 0) };
    let mut dai_name = [0 as c_char; 64];
    let mut is_single_links = 0;
    let mut ret: c_int;

    unsafe { dev_dbg(dev, c"link_of (%pOF)\n".as_ptr(), cpu_ep) };

    ret = unsafe { graph_parse_node(priv_, cpu_ep, li, &mut is_single_links) };
    if ret < 0 {
        return unsafe { graph_ret(priv_, ret) };
    }

    ret = unsafe { graph_parse_node(priv_, codec_ep, li, ptr::null_mut()) };
    if ret < 0 {
        return unsafe { graph_ret(priv_, ret) };
    }

    unsafe {
        snprintf(
            dai_name.as_mut_ptr(),
            dai_name.len(),
            c"%s-%s".as_ptr(),
            (*cpus).dai_name,
            (*codecs).dai_name,
        );

        simple_util_canonicalize_cpu(cpus, is_single_links);
        simple_util_canonicalize_platform(platforms, cpus);

        ret = graph_link_init(priv_, cpu_ep, codec_ep, li, dai_name.as_mut_ptr());
    }
    if ret < 0 {
        return unsafe { graph_ret(priv_, ret) };
    }

    unsafe {
        (*li).link += 1;
        graph_ret(priv_, ret)
    }
}

unsafe fn parse_as_dpcm_link(
    priv_: *mut simple_util_priv,
    codec_port: *mut device_node,
    adata: *mut simple_util_data,
) -> bool {
    if unsafe { (*priv_).force_dpcm } {
        return true;
    }

    if unsafe { (*priv_).dpcm_selectable == 0 } {
        return false;
    }

    /*
     * It is DPCM
     * if Codec port has many endpoints,
     * or has convert-xxx property
     */
    if unsafe { of_get_child_count(codec_port) > 1 || simple_util_is_convert_required(adata) } {
        return true;
    }

    false
}

unsafe extern "C" fn __graph_for_each_link(
    priv_: *mut simple_util_priv,
    li: *mut link_info,
    func_noml: LinkFunc,
    func_dpcm: LinkFunc,
) -> c_int {
    let mut it = of_phandle_iterator { node: ptr::null_mut() };
    let dev = unsafe { simple_priv_to_dev(priv_) };
    let node = unsafe { (*dev).of_node };
    let mut codec_port_old: *mut device_node = ptr::null_mut();
    let mut adata: simple_util_data = unsafe { core::mem::zeroed() };
    let mut ret = 0;

    /*
     * C source uses of_for_each_phandle() and for_each_of_graph_port_endpoint().
     * These iterator macros are preserved here as intent; their expansion depends
     * on external kernel definitions unavailable in this isolated source.
     */
    let mut rc = 0;
    while unsafe { of_for_each_phandle_rust(&mut it, &mut rc, node, c"dais".as_ptr()) } {
        let cpu_port = it.node;
        let mut cpu_ep: *mut device_node = ptr::null_mut();

        while unsafe { for_each_of_graph_port_endpoint_rust(cpu_port, &mut cpu_ep) } {
            /* get codec */
            let codec_ep = unsafe { of_graph_get_remote_endpoint(cpu_ep) };
            let codec_port = unsafe { ep_to_port(codec_ep) };

            /* get convert-xxx property */
            unsafe {
                memset(
                    &mut adata as *mut simple_util_data as *mut c_void,
                    0,
                    size_of::<simple_util_data>(),
                );
                graph_parse_convert(dev, codec_ep, &mut adata);
                graph_parse_convert(dev, cpu_ep, &mut adata);
            }

            /* check if link requires DPCM parsing */
            if unsafe { parse_as_dpcm_link(priv_, codec_port, &mut adata) } {
                /*
                 * Codec endpoint can be NULL for pluggable audio HW.
                 * Platform DT can populate the Codec endpoint depending on the
                 * plugged HW.
                 */
                /* Do it all CPU endpoint, and 1st Codec endpoint */
                if unsafe { (*li).cpu != 0 } || ((codec_port_old != codec_port) && !codec_ep.is_null()) {
                    ret = unsafe { func_dpcm(priv_, cpu_ep, codec_ep, li) };
                }
            /* else normal sound */
            } else if unsafe { (*li).cpu != 0 } {
                ret = unsafe { func_noml(priv_, cpu_ep, codec_ep, li) };
            }

            if ret < 0 {
                return unsafe { graph_ret(priv_, ret) };
            }

            codec_port_old = codec_port;
        }
    }

    unsafe { graph_ret(priv_, ret) }
}

unsafe extern "C" {
    fn of_for_each_phandle_rust(
        it: *mut of_phandle_iterator,
        rc: *mut c_int,
        node: *mut device_node,
        list_name: *const c_char,
    ) -> bool;
    fn for_each_of_graph_port_endpoint_rust(
        port: *mut device_node,
        ep: *mut *mut device_node,
    ) -> bool;
}

unsafe extern "C" fn graph_for_each_link(
    priv_: *mut simple_util_priv,
    li: *mut link_info,
    func_noml: LinkFunc,
    func_dpcm: LinkFunc,
) -> c_int {
    let mut ret = 0;
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
    unsafe { (*li).cpu = 1 };
    while unsafe { (*li).cpu >= 0 } {
        ret = unsafe { __graph_for_each_link(priv_, li, func_noml, func_dpcm) };
        if ret < 0 {
            break;
        }
        unsafe { (*li).cpu -= 1 };
    }

    unsafe { graph_ret(priv_, ret) }
}

unsafe extern "C" fn graph_count_noml(
    priv_: *mut simple_util_priv,
    _cpu_ep: *mut device_node,
    _codec_ep: *mut device_node,
    li: *mut link_info,
) -> c_int {
    let dev = unsafe { simple_priv_to_dev(priv_) };
    let mut ret = -EINVAL;

    if unsafe { (*li).link >= SNDRV_MAX_LINKS } {
        return unsafe { graph_ret(priv_, ret) };
    }

    /*
     * DON'T REMOVE platforms
     * see
     *	simple-card.c :: simple_count_noml()
     */
    unsafe {
        (*li).num[(*li).link as usize].cpus = 1;
        (*li).num[(*li).link as usize].platforms = 1;
        (*li).num[(*li).link as usize].codecs = 1;

        (*li).link += 1; /* 1xCPU-Codec */

        dev_dbg(dev, c"Count As Normal\n".as_ptr());
    }
    ret = 0;
    unsafe { graph_ret(priv_, ret) }
}

unsafe extern "C" fn graph_count_dpcm(
    priv_: *mut simple_util_priv,
    _cpu_ep: *mut device_node,
    _codec_ep: *mut device_node,
    li: *mut link_info,
) -> c_int {
    let dev = unsafe { simple_priv_to_dev(priv_) };
    let mut ret = -EINVAL;

    if unsafe { (*li).link >= SNDRV_MAX_LINKS } {
        return unsafe { graph_ret(priv_, ret) };
    }

    if unsafe { (*li).cpu != 0 } {
        /*
         * DON'T REMOVE platforms
         * see
         *	simple-card.c :: simple_count_noml()
         */
        unsafe {
            (*li).num[(*li).link as usize].cpus = 1;
            (*li).num[(*li).link as usize].platforms = 1;

            (*li).link += 1; /* 1xCPU-dummy */
        }
    } else {
        unsafe {
            (*li).num[(*li).link as usize].codecs = 1;

            (*li).link += 1; /* 1xdummy-Codec */
        }
    }

    unsafe { dev_dbg(dev, c"Count As DPCM\n".as_ptr()) };
    ret = 0;
    unsafe { graph_ret(priv_, ret) }
}

unsafe extern "C" fn graph_get_dais_count(
    priv_: *mut simple_util_priv,
    li: *mut link_info,
) -> c_int {
    /*
     * link_num :	number of links.
     *		CPU-Codec / CPU-dummy / dummy-Codec
     * dais_num :	number of DAIs
     * ccnf_num :	number of codec_conf
     *		same number for "dummy-Codec"
     *
     * ex1)
     * CPU0 --- Codec0	link : 5
     * CPU1 --- Codec1	dais : 7
     * CPU2 -/		ccnf : 1
     * CPU3 --- Codec2
     *
     *	=> 5 links = 2xCPU-Codec + 2xCPU-dummy + 1xdummy-Codec
     *	=> 7 DAIs  = 4xCPU + 3xCodec
     *	=> 1 ccnf  = 1xdummy-Codec
     *
     * ex2)
     * CPU0 --- Codec0	link : 5
     * CPU1 --- Codec1	dais : 6
     * CPU2 -/		ccnf : 1
     * CPU3 -/
     *
     *	=> 5 links = 1xCPU-Codec + 3xCPU-dummy + 1xdummy-Codec
     *	=> 6 DAIs  = 4xCPU + 2xCodec
     *	=> 1 ccnf  = 1xdummy-Codec
     *
     * ex3)
     * CPU0 --- Codec0	link : 6
     * CPU1 -/		dais : 6
     * CPU2 --- Codec1	ccnf : 2
     * CPU3 -/
     *
     *	=> 6 links = 0xCPU-Codec + 4xCPU-dummy + 2xdummy-Codec
     *	=> 6 DAIs  = 4xCPU + 2xCodec
     *	=> 2 ccnf  = 2xdummy-Codec
     *
     * ex4)
     * CPU0 --- Codec0 (convert-rate)	link : 3
     * CPU1 --- Codec1			dais : 4
     *					ccnf : 1
     *
     *	=> 3 links = 1xCPU-Codec + 1xCPU-dummy + 1xdummy-Codec
     *	=> 4 DAIs  = 2xCPU + 2xCodec
     *	=> 1 ccnf  = 1xdummy-Codec
     */
    unsafe { graph_for_each_link(priv_, li, graph_count_noml, graph_count_dpcm) }
}

#[no_mangle]
pub unsafe extern "C" fn audio_graph_parse_of(
    priv_: *mut simple_util_priv,
    dev: *mut device,
) -> c_int {
    let card = unsafe { simple_priv_to_card(priv_) };
    let mut ret = -ENOMEM;

    let li = unsafe { kzalloc(size_of::<link_info>(), GFP_KERNEL) as *mut link_info };
    if li.is_null() {
        return unsafe { graph_ret(priv_, ret) };
    }

    unsafe {
        (*card).owner = THIS_MODULE;
        (*card).dev = dev;
    }

    ret = unsafe { graph_get_dais_count(priv_, li) };
    if ret < 0 {
        return unsafe { graph_ret(priv_, ret) };
    }

    ret = -EINVAL;
    if unsafe { (*li).link == 0 } {
        return unsafe { graph_ret(priv_, ret) };
    }

    ret = unsafe { simple_util_init_priv(priv_, li) };
    if ret < 0 {
        return unsafe { graph_ret(priv_, ret) };
    }

    unsafe {
        (*priv_).pa_gpio = devm_gpiod_get_optional(dev, c"pa".as_ptr(), GPIOD_OUT_LOW);
    }
    if unsafe { IS_ERR((*priv_).pa_gpio as *const c_void) } {
        ret = unsafe { PTR_ERR((*priv_).pa_gpio as *const c_void) };
        unsafe { dev_err(dev, c"failed to get amplifier gpio: %d\n".as_ptr(), ret) };
        return unsafe { graph_ret(priv_, ret) };
    }

    ret = unsafe { simple_util_parse_widgets(priv_, ptr::null()) };
    if ret < 0 {
        return unsafe { graph_ret(priv_, ret) };
    }

    ret = unsafe { simple_util_parse_routing(priv_, ptr::null()) };
    if ret < 0 {
        return unsafe { graph_ret(priv_, ret) };
    }

    unsafe {
        memset(li as *mut c_void, 0, size_of::<link_info>());
    }
    ret = unsafe { graph_for_each_link(priv_, li, graph_dai_link_of, graph_dai_link_of_dpcm) };
    if ret < 0 {
        unsafe {
            simple_util_clean_reference(priv_);
            return dev_err_probe(dev, ret, c"parse error\n".as_ptr());
        }
    }

    /* Card name should be set after graph_for_each_link() */
    ret = unsafe { simple_util_parse_card_name(priv_, ptr::null()) };
    if ret < 0 {
        unsafe {
            simple_util_clean_reference(priv_);
            return dev_err_probe(dev, ret, c"parse error\n".as_ptr());
        }
    }

    unsafe {
        snd_soc_card_set_drvdata(card, priv_);
        simple_util_debug_info(priv_);
    }

    ret = unsafe { devm_snd_soc_register_card(dev, card) };
    if ret < 0 {
        unsafe {
            simple_util_clean_reference(priv_);
            return dev_err_probe(dev, ret, c"parse error\n".as_ptr());
        }
    }

    unsafe { graph_ret(priv_, ret) }
}

unsafe extern "C" fn graph_probe(pdev: *mut platform_device) -> c_int {
    let priv_: *mut simple_util_priv;
    let dev = unsafe { &mut (*pdev).dev as *mut device };
    let card: *mut snd_soc_card;

    /* Allocate the private data and the DAI link array */
    priv_ = unsafe { devm_kzalloc(dev, size_of::<simple_util_priv>(), GFP_KERNEL) as *mut simple_util_priv };
    if priv_.is_null() {
        return -ENOMEM;
    }

    card = unsafe { simple_priv_to_card(priv_) };
    unsafe {
        (*card).dapm_widgets = graph_dapm_widgets.as_ptr();
        (*card).num_dapm_widgets = graph_dapm_widgets.len() as c_int;
        (*card).probe = Some(graph_util_card_probe);

        if !of_device_get_match_data(dev).is_null() {
            (*priv_).dpcm_selectable = 1;
        }
    }

    unsafe { audio_graph_parse_of(priv_, dev) }
}

static graph_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: c"audio-graph-card".as_ptr(),
        data: ptr::null(),
    },
    of_device_id {
        compatible: c"audio-graph-scu-card".as_ptr(),
        data: DPCM_SELECTABLE as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

static graph_card: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: c"asoc-audio-graph-card".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
        of_match_table: graph_of_match.as_ptr(),
    },
    probe: Some(graph_probe),
    remove: Some(simple_util_remove),
};

/*
 * C module metadata translated as comments:
 * MODULE_DEVICE_TABLE(of, graph_of_match);
 * module_platform_driver(graph_card);
 * MODULE_ALIAS("platform:asoc-audio-graph-card");
 * MODULE_LICENSE("GPL v2");
 * MODULE_DESCRIPTION("ASoC Audio Graph Sound Card");
 * MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");
 * EXPORT_SYMBOL_GPL(audio_graph_parse_of);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
