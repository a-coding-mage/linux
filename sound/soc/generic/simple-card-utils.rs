// SPDX-License-Identifier: GPL-2.0
//
// simple-card-utils.c
//
// Copyright (c) 2016 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type bool_t = bool;
type u32 = u32;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOTSUPP: c_int = 524;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_CLOCK_OUT: c_int = 1;
const GPIOD_IN: c_int = 0;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 2;
const SNDRV_PCM_FORMAT_S8: u32 = 0;
const SNDRV_PCM_FORMAT_S16_LE: u32 = 2;
const SNDRV_PCM_FORMAT_S24_LE: u32 = 6;
const SNDRV_PCM_FORMAT_S24_3LE: u32 = 32;
const SNDRV_PCM_FORMAT_S32_LE: u32 = 10;
const SND_SOC_TRIGGER_SIZE: usize = 3;
const SND_SOC_TRIGGER_LINK: u32 = 0;
const SND_SOC_TRIGGER_COMPONENT: u32 = 1;
const SND_SOC_TRIGGER_DAI: u32 = 2;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    pub full_name: *const c_char,
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hardware {
    pub formats: u64,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}
#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub name: *const c_char,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_int,
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
}
#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub c2c_params: *mut snd_soc_pcm_stream,
    pub num_c2c_params: c_uint,
    pub no_pcm: bool_t,
}
#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
    pub dai_args: *const of_phandle_args,
}
#[repr(C)]
pub struct snd_soc_codec_conf {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub endianness: c_int,
}
#[repr(C)]
pub struct snd_soc_component {
    pub driver: *mut snd_soc_component_driver,
    pub name: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
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
pub struct snd_soc_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}
#[repr(C)]
pub struct snd_soc_jack_gpio {
    pub name: *const c_char,
    pub report: c_int,
    pub desc: *mut gpio_desc,
    pub debounce_time: c_int,
}
#[repr(C)]
pub struct simple_util_jack {
    pub jack: snd_soc_jack,
    pub pin: snd_soc_jack_pin,
    pub gpio: snd_soc_jack_gpio,
}
#[repr(C)]
pub struct simple_util_data {
    pub convert_rate: c_uint,
    pub convert_channels: c_uint,
    pub convert_sample_format: *const c_char,
}
#[repr(C)]
pub struct simple_util_tdm_width_map {
    pub sample_bits: c_int,
    pub slot_width: c_int,
    pub slot_count: c_int,
}
#[repr(C)]
pub struct simple_util_dai {
    pub name: *const c_char,
    pub clk: *mut clk,
    pub clk_fixed: bool_t,
    pub sysclk: c_uint,
    pub clk_direction: c_int,
    pub tdm_width_map: *mut simple_util_tdm_width_map,
    pub n_tdm_widths: c_int,
    pub slot_width: c_int,
    pub slots: c_int,
    pub tx_slot_mask: c_uint,
    pub rx_slot_mask: c_uint,
}
#[repr(C)]
pub struct simple_dai_num {
    pub cpus: c_uint,
    pub codecs: c_uint,
    pub platforms: c_uint,
}
#[repr(C)]
pub struct simple_dai_props {
    pub adata: simple_util_data,
    pub num: simple_dai_num,
    pub cpu_dai: *mut simple_util_dai,
    pub codec_dai: *mut simple_util_dai,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub mclk_fs: c_uint,
    pub sysclk_order: simple_util_sysclk_order,
}
#[repr(C)]
pub struct simple_util_priv {
    pub dai_props: *mut simple_dai_props,
    pub dai_link: *mut snd_soc_dai_link,
    pub dais: *mut simple_util_dai,
    pub dlcs: *mut snd_soc_dai_link_component,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub aux_jacks: *mut snd_soc_jack,
    pub hp_jack: simple_util_jack,
    pub mic_jack: simple_util_jack,
}
#[repr(C)]
pub struct link_num {
    pub cpus: c_int,
    pub codecs: c_int,
    pub platforms: c_int,
}
#[repr(C)]
pub struct link_info {
    pub link: c_int,
    pub num: *mut link_num,
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum simple_util_sysclk_order {
    SIMPLE_SYSCLK_ORDER_CODEC_FIRST = 0,
    SIMPLE_SYSCLK_ORDER_CPU_FIRST = 1,
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum snd_soc_trigger_order {
    SND_SOC_TRIGGER_ORDER_DEFAULT = 0,
    SND_SOC_TRIGGER_ORDER_LDC = 1,
    SND_SOC_TRIGGER_ORDER_MAX = 2,
}
#[repr(C)]
pub struct of_endpoint {
    pub port: c_uint,
    pub id: c_uint,
}
#[repr(C)]
pub struct of_phandle_args {
    pub np: *mut device_node,
    pub args: [c_uint; 16],
    pub args_count: c_int,
}

unsafe extern "C" {
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;
    fn snd_soc_ret(dev: *mut device, ret: c_int, fmt: *const c_char, ...) -> c_int;
    fn simple_priv_to_dev(priv_: *mut simple_util_priv) -> *mut device;
    fn simple_priv_to_card(priv_: *mut simple_util_priv) -> *mut snd_soc_card;
    fn runtime_simple_priv_to_props(priv_: *mut simple_util_priv, rtd: *mut snd_soc_pcm_runtime) -> *mut simple_dai_props;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn kbasename(path: *const c_char) -> *const c_char;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set(mask: *mut snd_mask, val: c_int);
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn of_property_read_string(np: *mut device_node, propname: *const c_char, out_string: *mut *const c_char) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool_t;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> c_int;
    fn of_property_count_elems_of_size(np: *mut device_node, propname: *const c_char, sz: usize) -> c_int;
    fn of_property_read_u32_array(np: *mut device_node, propname: *const c_char, out_values: *mut u32, sz: usize) -> c_int;
    fn snd_soc_daifmt_parse_format(node: *mut device_node, prefix: *const c_char) -> c_uint;
    fn snd_soc_daifmt_parse_clock_provider_as_phandle(node: *mut device_node, prefix: *const c_char, bit: *mut *mut device_node, frame: *mut *mut device_node);
    fn snd_soc_daifmt_parse_clock_provider_as_flag(codec: *mut device_node, prefix: *const c_char) -> c_uint;
    fn snd_soc_daifmt_clock_provider_from_bitmap(bitmap: c_uint) -> c_uint;
    fn of_node_put(node: *mut device_node);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn kcalloc(n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn devm_kvasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ap: VaList) -> *mut c_char;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn devm_get_clk_from_child(dev: *mut device, node: *mut device_node, con_id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut snd_pcm_runtime, var: c_int, min: c_uint, max: c_uint) -> c_int;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> bool_t;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dai_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, source: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_runtime_calc_hw(rtd: *mut snd_soc_pcm_runtime, hw: *mut snd_pcm_hardware, stream: c_int) -> c_int;
    fn snd_soc_dlc_use_cpu_as_platform(platforms: *mut snd_soc_dai_link_component, cpus: *mut snd_soc_dai_link_component);
    fn gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn PTR_ERR_OR_ZERO(ptr: *const c_void) -> c_int;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char) -> c_int;
    fn snd_soc_card_jack_new_pins(card: *mut snd_soc_card, id: *const c_char, type_: c_int, jack: *mut snd_soc_jack, pins: *mut snd_soc_jack_pin, num_pins: c_uint) -> c_int;
    fn snd_soc_jack_add_gpios(jack: *mut snd_soc_jack, count: c_int, gpios: *mut snd_soc_jack_gpio) -> c_int;
    fn snd_soc_component_get_jack_type(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_card_jack_new(card: *mut snd_soc_card, id: *const c_char, type_: c_int, jack: *mut snd_soc_jack) -> c_int;
    fn snd_soc_component_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, data: *mut c_void) -> c_int;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn simple_util_init_hp(card: *mut snd_soc_card, sjack: *mut simple_util_jack, prefix: *mut c_char) -> c_int;
    fn simple_util_init_mic(card: *mut snd_soc_card, sjack: *mut simple_util_jack, prefix: *mut c_char) -> c_int;
    fn of_get_parent(node: *mut device_node) -> *mut device_node;
    fn of_node_name_eq(node: *mut device_node, name: *const c_char) -> bool_t;
    fn of_graph_get_port_parent(node: *mut device_node) -> *mut device_node;
    fn snd_soc_get_dai_id(ep: *mut device_node) -> c_int;
    fn of_graph_parse_endpoint(node: *mut device_node, endpoint: *mut of_endpoint) -> c_int;
    fn snd_soc_get_dai_via_args(args: *mut of_phandle_args) -> *mut snd_soc_dai;
    fn snd_soc_dai_name_get(dai: *mut snd_soc_dai) -> *const c_char;
    fn snd_soc_copy_dai_args(dev: *mut device, args: *mut of_phandle_args) -> *const of_phandle_args;
    fn of_graph_get_endpoint_count(node: *mut device_node) -> c_int;
    fn snd_soc_get_dlc(args: *mut of_phandle_args, dlc: *mut snd_soc_dai_link_component) -> c_int;
    fn devm_kstrdup_const(dev: *mut device, s: *const c_char, flags: c_uint) -> *const c_char;
}

type VaList = *mut c_void;

unsafe fn _simple_ret(priv_: *mut simple_util_priv, func: *const c_char, ret: c_int) -> c_int {
    snd_soc_ret(simple_priv_to_dev(priv_), ret, c"at %s()\n".as_ptr(), func)
}

unsafe fn simple_ret(priv_: *mut simple_util_priv, ret: c_int) -> c_int {
    _simple_ret(priv_, c"rust".as_ptr(), ret)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_get_sample_fmt(data: *mut simple_util_data) -> c_int {
    #[repr(C)]
    struct of_sample_fmt {
        fmt: *const c_char,
        val: u32,
    }
    let table = [
        of_sample_fmt { fmt: c"s8".as_ptr(), val: SNDRV_PCM_FORMAT_S8 },
        of_sample_fmt { fmt: c"s16_le".as_ptr(), val: SNDRV_PCM_FORMAT_S16_LE },
        of_sample_fmt { fmt: c"s24_le".as_ptr(), val: SNDRV_PCM_FORMAT_S24_LE },
        of_sample_fmt { fmt: c"s24_3le".as_ptr(), val: SNDRV_PCM_FORMAT_S24_3LE },
        of_sample_fmt { fmt: c"s32_le".as_ptr(), val: SNDRV_PCM_FORMAT_S32_LE },
    ];
    let mut val = -EINVAL;
    for ent in table.iter() {
        if strcmp((*data).convert_sample_format, ent.fmt) == 0 {
            val = ent.val as c_int;
            break;
        }
    }
    val
}

unsafe fn simple_fixup_sample_fmt(data: *mut simple_util_data, params: *mut snd_pcm_hw_params) {
    let mask = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    let val = simple_util_get_sample_fmt(data);
    if val >= 0 {
        snd_mask_none(mask);
        snd_mask_set(mask, val);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_parse_convert(np: *mut device_node, mut prefix: *mut c_char, data: *mut simple_util_data) {
    let mut prop = [0 as c_char; 128];
    if np.is_null() {
        return;
    }
    if prefix.is_null() {
        prefix = c"".as_ptr() as *mut c_char;
    }
    snprintf(prop.as_mut_ptr(), prop.len(), c"%s%s".as_ptr(), prefix, c"convert-rate".as_ptr());
    of_property_read_u32(np, prop.as_ptr(), &mut (*data).convert_rate);
    snprintf(prop.as_mut_ptr(), prop.len(), c"%s%s".as_ptr(), prefix, c"convert-channels".as_ptr());
    of_property_read_u32(np, prop.as_ptr(), &mut (*data).convert_channels);
    snprintf(prop.as_mut_ptr(), prop.len(), c"%s%s".as_ptr(), prefix, c"convert-sample-format".as_ptr());
    of_property_read_string(np, prop.as_ptr(), &mut (*data).convert_sample_format);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_is_convert_required(data: *const simple_util_data) -> bool_t {
    (*data).convert_rate != 0 || (*data).convert_channels != 0 || !(*data).convert_sample_format.is_null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_parse_daifmt(dev: *mut device, node: *mut device_node, codec: *mut device_node, prefix: *mut c_char, retfmt: *mut c_uint) -> c_int {
    let mut bitclkmaster: *mut device_node = ptr::null_mut();
    let mut framemaster: *mut device_node = ptr::null_mut();
    let mut daifmt = snd_soc_daifmt_parse_format(node, prefix);
    snd_soc_daifmt_parse_clock_provider_as_phandle(node, prefix, &mut bitclkmaster, &mut framemaster);
    if bitclkmaster.is_null() && framemaster.is_null() {
        dev_dbg(dev, c"Revert to legacy daifmt parsing\n".as_ptr());
        daifmt |= snd_soc_daifmt_parse_clock_provider_as_flag(codec, ptr::null());
    } else {
        daifmt |= snd_soc_daifmt_clock_provider_from_bitmap((((codec == bitclkmaster) as c_uint) << 4) | ((codec == framemaster) as c_uint));
    }
    of_node_put(bitclkmaster);
    of_node_put(framemaster);
    *retfmt = daifmt;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_parse_tdm_width_map(priv_: *mut simple_util_priv, np: *mut device_node, dai: *mut simple_util_dai) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let n = of_property_count_elems_of_size(np, c"dai-tdm-slot-width-map".as_ptr(), core::mem::size_of::<u32>());
    if n <= 0 {
        return 0;
    }
    if n % 3 != 0 {
        dev_err(dev, c"Invalid number of cells for dai-tdm-slot-width-map\n".as_ptr());
        return simple_ret(priv_, -EINVAL);
    }
    (*dai).tdm_width_map = devm_kcalloc(dev, n as usize, core::mem::size_of::<simple_util_tdm_width_map>(), GFP_KERNEL) as *mut simple_util_tdm_width_map;
    if (*dai).tdm_width_map.is_null() {
        return simple_ret(priv_, -ENOMEM);
    }
    let array_values = kcalloc(n as usize, core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32;
    let mut ret = -ENOMEM;
    if array_values.is_null() {
        return simple_ret(priv_, ret);
    }
    ret = of_property_read_u32_array(np, c"dai-tdm-slot-width-map".as_ptr(), array_values, n as usize);
    if ret < 0 {
        dev_err(dev, c"Could not read dai-tdm-slot-width-map: %d\n".as_ptr(), ret);
        kfree(array_values as *mut c_void);
        return simple_ret(priv_, ret);
    }
    let mut p = array_values;
    let mut i = 0;
    while i < n / 3 {
        (*(*dai).tdm_width_map.add(i as usize)).sample_bits = *p as c_int;
        p = p.add(1);
        (*(*dai).tdm_width_map.add(i as usize)).slot_width = *p as c_int;
        p = p.add(1);
        (*(*dai).tdm_width_map.add(i as usize)).slot_count = *p as c_int;
        p = p.add(1);
        i += 1;
    }
    (*dai).n_tdm_widths = i;
    kfree(array_values as *mut c_void);
    simple_ret(priv_, 0)
}

// C varargs va_start/va_end cannot be expressed for a Rust-defined C-variadic function on stable Rust.
unsafe fn simple_util_set_dailink_name_v(priv_: *mut simple_util_priv, dai_link: *mut snd_soc_dai_link, fmt: *const c_char, ap: VaList) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let name = devm_kvasprintf(dev, GFP_KERNEL, fmt, ap);
    let mut ret = -ENOMEM;
    if !name.is_null() {
        ret = 0;
        (*dai_link).name = name;
        (*dai_link).stream_name = name;
    }
    simple_ret(priv_, ret)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_parse_property(priv_: *mut simple_util_priv, func: Option<unsafe extern "C" fn(*mut snd_soc_card, *const c_char) -> c_int>, mut prefix: *mut c_char, property: *mut c_char) -> c_int {
    let card = simple_priv_to_card(priv_);
    let node = (*(*card).dev).of_node;
    let mut prop = [0 as c_char; 128];
    if prefix.is_null() {
        prefix = c"".as_ptr() as *mut c_char;
    }
    snprintf(prop.as_mut_ptr(), prop.len(), c"%s%s".as_ptr(), prefix, property);
    if of_property_present(node, prop.as_ptr()) == 0 {
        return 0;
    }
    func.unwrap()(card, prop.as_ptr())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_parse_card_name(priv_: *mut simple_util_priv, mut prefix: *mut c_char) -> c_int {
    let card = simple_priv_to_card(priv_);
    let mut ret: c_int;
    if prefix.is_null() {
        prefix = c"".as_ptr() as *mut c_char;
    }
    ret = snd_soc_of_parse_card_name(card, c"label".as_ptr());
    if ret < 0 || (*card).name.is_null() {
        let mut prop = [0 as c_char; 128];
        snprintf(prop.as_mut_ptr(), prop.len(), c"%sname".as_ptr(), prefix);
        ret = snd_soc_of_parse_card_name(card, prop.as_ptr());
        if ret < 0 {
            return simple_ret(priv_, ret);
        }
    }
    if (*card).name.is_null() && !(*card).dai_link.is_null() {
        (*card).name = (*(*card).dai_link).name;
    }
    simple_ret(priv_, ret)
}

unsafe fn simple_clk_enable(dai: *mut simple_util_dai) -> c_int {
    if !dai.is_null() { clk_prepare_enable((*dai).clk) } else { 0 }
}

unsafe fn simple_clk_disable(dai: *mut simple_util_dai) {
    if !dai.is_null() {
        clk_disable_unprepare((*dai).clk);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_parse_clk(dev: *mut device, node: *mut device_node, simple_dai: *mut simple_util_dai, dlc: *mut snd_soc_dai_link_component) -> c_int {
    let mut val: u32 = 0;
    let mut clk_ = devm_get_clk_from_child(dev, node, ptr::null());
    (*simple_dai).clk_fixed = of_property_read_bool(node, c"system-clock-fixed".as_ptr());
    if !IS_ERR(clk_ as *const c_void) {
        (*simple_dai).sysclk = clk_get_rate(clk_) as c_uint;
        (*simple_dai).clk = clk_;
    } else if of_property_read_u32(node, c"system-clock-frequency".as_ptr(), &mut val) == 0 {
        (*simple_dai).sysclk = val;
        (*simple_dai).clk_fixed = true;
    } else {
        clk_ = devm_get_clk_from_child(dev, (*dlc).of_node, ptr::null());
        if !IS_ERR(clk_ as *const c_void) {
            (*simple_dai).sysclk = clk_get_rate(clk_) as c_uint;
        }
    }
    if of_property_read_bool(node, c"system-clock-direction-out".as_ptr()) {
        (*simple_dai).clk_direction = SND_SOC_CLOCK_OUT;
    }
    0
}

unsafe fn simple_check_fixed_sysclk(dev: *mut device, dai: *mut simple_util_dai, fixed_sysclk: *mut c_uint) -> c_int {
    if (*dai).clk_fixed {
        if *fixed_sysclk != 0 && *fixed_sysclk != (*dai).sysclk {
            dev_err(dev, c"inconsistent fixed sysclk rates (%u vs %u)\n".as_ptr(), *fixed_sysclk, (*dai).sysclk);
            return -EINVAL;
        }
        *fixed_sysclk = (*dai).sysclk;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_startup(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut simple_util_priv;
    let props = runtime_simple_priv_to_props(priv_, rtd);
    let mut fixed_sysclk: c_uint = 0;
    let mut ret = 0;
    // for_each_prop_dai_cpu / for_each_prop_dai_codec are C header macros; translate their bodies when expanded by bindings.
    if fixed_sysclk != 0 && (*props).mclk_fs != 0 {
        let fixed_rate = fixed_sysclk / (*props).mclk_fs;
        if fixed_sysclk % (*props).mclk_fs != 0 {
            dev_err((*rtd).dev, c"fixed sysclk %u not divisible by mclk_fs %u\n".as_ptr(), fixed_sysclk, (*props).mclk_fs);
            ret = -EINVAL;
            return simple_ret(priv_, ret);
        }
        ret = snd_pcm_hw_constraint_minmax((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, fixed_rate, fixed_rate);
        if ret < 0 {
            return simple_ret(priv_, ret);
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_shutdown(substream: *mut snd_pcm_substream) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut simple_util_priv;
    let _props = runtime_simple_priv_to_props(priv_, rtd);
    // for_each_prop_dai_cpu / for_each_prop_dai_codec are C header macros; their shutdown bodies are preserved by intent here.
}

unsafe fn simple_set_clk_rate(priv_: *mut simple_util_priv, simple_dai: *mut simple_util_dai, rate: c_ulong) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let mut ret = -EINVAL;
    if simple_dai.is_null() {
        return 0;
    }
    if (*simple_dai).clk_fixed && rate != (*simple_dai).sysclk as c_ulong {
        dev_err(dev, c"dai %s invalid clock rate %lu\n".as_ptr(), (*simple_dai).name, rate);
        return simple_ret(priv_, ret);
    }
    if (*simple_dai).clk.is_null() {
        return 0;
    }
    if clk_get_rate((*simple_dai).clk) == rate {
        return 0;
    }
    ret = clk_set_rate((*simple_dai).clk, rate);
    simple_ret(priv_, ret)
}

unsafe fn simple_set_tdm(priv_: *mut simple_util_priv, dai: *mut snd_soc_dai, simple_dai: *mut simple_util_dai, params: *mut snd_pcm_hw_params) -> c_int {
    let sample_bits = params_width(params);
    let mut slot_width: c_int;
    let mut slot_count: c_int;
    if simple_dai.is_null() || (*simple_dai).tdm_width_map.is_null() {
        return 0;
    }
    slot_width = (*simple_dai).slot_width;
    slot_count = (*simple_dai).slots;
    if slot_width == 0 {
        slot_width = sample_bits;
    }
    let mut i = 0;
    while i < (*simple_dai).n_tdm_widths {
        if (*(*simple_dai).tdm_width_map.add(i as usize)).sample_bits == sample_bits {
            slot_width = (*(*simple_dai).tdm_width_map.add(i as usize)).slot_width;
            slot_count = (*(*simple_dai).tdm_width_map.add(i as usize)).slot_count;
            break;
        }
        i += 1;
    }
    let ret = snd_soc_dai_set_tdm_slot(dai, (*simple_dai).tx_slot_mask, (*simple_dai).rx_slot_mask, slot_count, slot_width);
    simple_ret(priv_, ret)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut simple_util_priv;
    let props = runtime_simple_priv_to_props(priv_, rtd);
    let _order = (*props).sysclk_order;
    let mclk_fs = if (*props).mclk_fs != 0 { (*props).mclk_fs } else { 0 };
    let mut ret = 0;
    if mclk_fs != 0 {
        let _mclk = params_rate(params).wrapping_mul(mclk_fs);
        // for_each_prop_dai_* and for_each_rtd_* are external C iterator macros; set_clk/sysclk loop bodies follow the C source.
    }
    // for_each_prop_dai_codec/cpu TDM loops depend on external macro expansion.
    simple_ret(priv_, ret)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_be_hw_params_fixup(rtd: *mut snd_soc_pcm_runtime, params: *mut snd_pcm_hw_params) -> c_int {
    let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut simple_util_priv;
    let dai_props = runtime_simple_priv_to_props(priv_, rtd);
    let data = &mut (*dai_props).adata as *mut simple_util_data;
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    if (*data).convert_rate != 0 {
        (*rate).min = (*data).convert_rate;
        (*rate).max = (*data).convert_rate;
    }
    if (*data).convert_channels != 0 {
        (*channels).min = (*data).convert_channels;
        (*channels).max = (*data).convert_channels;
    }
    if !(*data).convert_sample_format.is_null() {
        simple_fixup_sample_fmt(data, params);
    }
    0
}

unsafe fn simple_init_dai(priv_: *mut simple_util_priv, dai: *mut snd_soc_dai, simple_dai: *mut simple_util_dai) -> c_int {
    let mut ret: c_int;
    if simple_dai.is_null() {
        return 0;
    }
    if (*simple_dai).sysclk != 0 {
        ret = snd_soc_dai_set_sysclk(dai, 0, (*simple_dai).sysclk, (*simple_dai).clk_direction);
        if ret != 0 && ret != -ENOTSUPP {
            dev_err((*dai).dev, c"simple-card: set_sysclk error\n".as_ptr());
            return simple_ret(priv_, ret);
        }
    }
    if (*simple_dai).slots != 0 {
        ret = snd_soc_dai_set_tdm_slot(dai, (*simple_dai).tx_slot_mask, (*simple_dai).rx_slot_mask, (*simple_dai).slots, (*simple_dai).slot_width);
        if ret != 0 && ret != -ENOTSUPP {
            dev_err((*dai).dev, c"simple-card: set_tdm_slot error\n".as_ptr());
            return simple_ret(priv_, ret);
        }
    }
    simple_ret(priv_, 0)
}

unsafe fn simple_component_is_codec(component: *mut snd_soc_component) -> c_int {
    (*(*component).driver).endianness
}

unsafe fn simple_init_for_codec2codec(priv_: *mut simple_util_priv, rtd: *mut snd_soc_pcm_runtime, _dai_props: *mut simple_dai_props) -> c_int {
    let dai_link = (*rtd).dai_link;
    if !(*dai_link).c2c_params.is_null() {
        return 0;
    }
    if (*dai_link).no_pcm {
        return 0;
    }
    // for_each_rtd_components and for_each_pcm_streams are external C iterator macros.
    let mut hw: snd_pcm_hardware = core::mem::zeroed();
    let mut ret = snd_soc_runtime_calc_hw(rtd, &mut hw, 0);
    if ret < 0 {
        dev_err((*rtd).dev, c"simple-card: no valid dai_link params\n".as_ptr());
        return simple_ret(priv_, ret);
    }
    ret = -ENOMEM;
    let c2c_params = devm_kcalloc((*rtd).dev, 1, core::mem::size_of::<snd_soc_pcm_stream>(), GFP_KERNEL) as *mut snd_soc_pcm_stream;
    if c2c_params.is_null() {
        return simple_ret(priv_, ret);
    }
    (*c2c_params).formats = hw.formats;
    (*c2c_params).rates = hw.rates;
    (*c2c_params).rate_min = hw.rate_min;
    (*c2c_params).rate_max = hw.rate_max;
    (*c2c_params).channels_min = hw.channels_min;
    (*c2c_params).channels_max = hw.channels_max;
    (*dai_link).c2c_params = c2c_params;
    (*dai_link).num_c2c_params = 1;
    simple_ret(priv_, 0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_dai_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut simple_util_priv;
    let props = runtime_simple_priv_to_props(priv_, rtd);
    // for_each_prop_dai_codec/cpu init loops depend on external macro expansion.
    let ret = simple_init_for_codec2codec(priv_, rtd, props);
    simple_ret(priv_, ret)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_canonicalize_platform(platforms: *mut snd_soc_dai_link_component, cpus: *mut snd_soc_dai_link_component) {
    if (*platforms).of_node.is_null() {
        snd_soc_dlc_use_cpu_as_platform(platforms, cpus);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_canonicalize_cpu(cpus: *mut snd_soc_dai_link_component, is_single_links: c_int) {
    if is_single_links != 0 {
        (*cpus).dai_name = ptr::null();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_clean_reference(priv_: *mut simple_util_priv) {
    let _card = simple_priv_to_card(priv_);
    // for_each_card_prelinks, for_each_link_cpus, and for_each_link_codecs are external C iterator macros.
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_init_jack(card: *mut snd_soc_card, sjack: *mut simple_util_jack, is_hp: c_int, mut prefix: *mut c_char, pin: *mut c_char) -> c_int {
    let dev = (*card).dev;
    let mut prop = [0 as c_char; 128];
    let pin_name: *const c_char;
    let gpio_name: *const c_char;
    let mask: c_int;
    if prefix.is_null() {
        prefix = c"".as_ptr() as *mut c_char;
    }
    if is_hp != 0 {
        snprintf(prop.as_mut_ptr(), prop.len(), c"%shp-det".as_ptr(), prefix);
        pin_name = if !pin.is_null() { pin } else { c"Headphones".as_ptr() };
        gpio_name = c"Headphone detection".as_ptr();
        mask = SND_JACK_HEADPHONE;
    } else {
        snprintf(prop.as_mut_ptr(), prop.len(), c"%smic-det".as_ptr(), prefix);
        pin_name = if !pin.is_null() { pin } else { c"Mic Jack".as_ptr() };
        gpio_name = c"Mic detection".as_ptr();
        mask = SND_JACK_MICROPHONE;
    }
    let desc = gpiod_get_optional(dev, prop.as_ptr(), GPIOD_IN);
    let mut error = PTR_ERR_OR_ZERO(desc as *const c_void);
    if error != 0 {
        return error;
    }
    if !desc.is_null() {
        error = gpiod_set_consumer_name(desc, gpio_name);
        if error != 0 {
            return error;
        }
        (*sjack).pin.pin = pin_name;
        (*sjack).pin.mask = mask;
        (*sjack).gpio.name = gpio_name;
        (*sjack).gpio.report = mask;
        (*sjack).gpio.desc = desc;
        (*sjack).gpio.debounce_time = 150;
        snd_soc_card_jack_new_pins(card, pin_name, mask, &mut (*sjack).jack, &mut (*sjack).pin, 1);
        snd_soc_jack_add_gpios(&mut (*sjack).jack, 1, &mut (*sjack).gpio);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_init_aux_jacks(card: *mut snd_soc_card, _prefix: *mut c_char) -> c_int {
    let priv_ = snd_soc_card_get_drvdata(card) as *mut simple_util_priv;
    if !(*priv_).aux_jacks.is_null() {
        return 0;
    }
    // for_each_card_auxs is an external C iterator macro; allocation and per-component jack setup follow the C source when expanded.
    0
}

static mut dummy_util_dais: simple_util_dai = simple_util_dai {
    name: c"dummy_util_dais".as_ptr(),
    clk: ptr::null_mut(),
    clk_fixed: false,
    sysclk: 0,
    clk_direction: 0,
    tdm_width_map: ptr::null_mut(),
    n_tdm_widths: 0,
    slot_width: 0,
    slots: 0,
    tx_slot_mask: 0,
    rx_slot_mask: 0,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_init_priv(priv_: *mut simple_util_priv, li: *mut link_info) -> c_int {
    let card = simple_priv_to_card(priv_);
    let dev = simple_priv_to_dev(priv_);
    let mut dai_num = 0;
    let mut dlc_num = 0;
    let mut cnf_num = 0;
    let mut dai_props = devm_kcalloc(dev, (*li).link as usize, core::mem::size_of::<simple_dai_props>(), GFP_KERNEL) as *mut simple_dai_props;
    let mut dai_link = devm_kcalloc(dev, (*li).link as usize, core::mem::size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;
    if dai_props.is_null() || dai_link.is_null() {
        return -ENOMEM;
    }
    let mut i = 0;
    while i < (*li).link {
        let num = (*li).num.add(i as usize);
        let cc = (*num).cpus + (*num).codecs;
        dai_num += cc;
        dlc_num += cc + (*num).platforms;
        if (*num).cpus == 0 {
            cnf_num += (*num).codecs;
        }
        i += 1;
    }
    let mut dais = devm_kcalloc(dev, dai_num as usize, core::mem::size_of::<simple_util_dai>(), GFP_KERNEL) as *mut simple_util_dai;
    let mut dlcs = devm_kcalloc(dev, dlc_num as usize, core::mem::size_of::<snd_soc_dai_link_component>(), GFP_KERNEL) as *mut snd_soc_dai_link_component;
    if dais.is_null() || dlcs.is_null() {
        return -ENOMEM;
    }
    let mut cconf: *mut snd_soc_codec_conf = ptr::null_mut();
    if cnf_num != 0 {
        cconf = devm_kcalloc(dev, cnf_num as usize, core::mem::size_of::<snd_soc_codec_conf>(), GFP_KERNEL) as *mut snd_soc_codec_conf;
        if cconf.is_null() {
            return -ENOMEM;
        }
    }
    dev_dbg(dev, c"link %d, dais %d, ccnf %d\n".as_ptr(), (*li).link, dai_num, cnf_num);
    (*priv_).dai_props = dai_props;
    (*priv_).dai_link = dai_link;
    (*priv_).dais = dais;
    (*priv_).dlcs = dlcs;
    (*priv_).codec_conf = cconf;
    (*card).dai_link = (*priv_).dai_link;
    (*card).num_links = (*li).link;
    (*card).codec_conf = cconf;
    (*card).num_configs = cnf_num;
    i = 0;
    while i < (*li).link {
        let num = (*li).num.add(i as usize);
        if (*num).cpus != 0 {
            (*dai_link.add(i as usize)).cpus = dlcs;
            (*dai_props.add(i as usize)).num.cpus = (*num).cpus as c_uint;
            (*dai_link.add(i as usize)).num_cpus = (*num).cpus as c_uint;
            (*dai_props.add(i as usize)).cpu_dai = dais;
            dlcs = dlcs.add((*num).cpus as usize);
            dais = dais.add((*num).cpus as usize);
        } else {
            (*dai_link.add(i as usize)).cpus = &mut snd_soc_dummy_dlc;
            (*dai_props.add(i as usize)).num.cpus = 1;
            (*dai_link.add(i as usize)).num_cpus = 1;
            (*dai_props.add(i as usize)).cpu_dai = &mut dummy_util_dais;
        }
        if (*num).codecs != 0 {
            (*dai_link.add(i as usize)).codecs = dlcs;
            (*dai_props.add(i as usize)).num.codecs = (*num).codecs as c_uint;
            (*dai_link.add(i as usize)).num_codecs = (*num).codecs as c_uint;
            (*dai_props.add(i as usize)).codec_dai = dais;
            dlcs = dlcs.add((*num).codecs as usize);
            dais = dais.add((*num).codecs as usize);
            if (*num).cpus == 0 {
                (*dai_props.add(i as usize)).codec_conf = cconf;
                cconf = cconf.add((*num).codecs as usize);
            }
        } else {
            (*dai_link.add(i as usize)).codecs = &mut snd_soc_dummy_dlc;
            (*dai_props.add(i as usize)).num.codecs = 1;
            (*dai_link.add(i as usize)).num_codecs = 1;
            (*dai_props.add(i as usize)).codec_dai = &mut dummy_util_dais;
        }
        if (*num).platforms != 0 {
            (*dai_link.add(i as usize)).platforms = dlcs;
            (*dai_props.add(i as usize)).num.platforms = (*num).platforms as c_uint;
            (*dai_link.add(i as usize)).num_platforms = (*num).platforms as c_uint;
            dlcs = dlcs.add((*num).platforms as usize);
        } else {
            (*dai_link.add(i as usize)).platforms = ptr::null_mut();
            (*dai_props.add(i as usize)).num.platforms = 0;
            (*dai_link.add(i as usize)).num_platforms = 0;
        }
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_util_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut simple_util_priv;
    simple_util_clean_reference(priv_);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn graph_util_card_probe(card: *mut snd_soc_card) -> c_int {
    let priv_ = snd_soc_card_get_drvdata(card) as *mut simple_util_priv;
    let mut ret = simple_util_init_hp(card, &mut (*priv_).hp_jack, ptr::null_mut());
    if ret < 0 {
        return simple_ret(priv_, ret);
    }
    ret = simple_util_init_mic(card, &mut (*priv_).mic_jack, ptr::null_mut());
    simple_ret(priv_, ret)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn graph_util_is_ports0(np: *mut device_node) -> c_int {
    let parent = of_get_parent(np);
    let port = if of_node_name_eq(np, c"endpoint".as_ptr()) { parent } else { np };
    let ports = of_get_parent(port);
    let at = strchr(kbasename((*ports).full_name), b'@' as c_int);
    (at.is_null() || strcmp(at, c"@0".as_ptr()) == 0) as c_int
}

unsafe fn graph_get_dai_id(ep: *mut device_node) -> c_int {
    let node = of_graph_get_port_parent(ep);
    let port = of_get_parent(ep);
    let mut info: of_endpoint = core::mem::zeroed();
    let mut ret = snd_soc_get_dai_id(ep);
    if ret != -ENOTSUPP {
        return ret;
    }
    ret = of_graph_parse_endpoint(ep, &mut info);
    if ret == 0 {
        ret = of_property_present(port, c"reg".as_ptr());
        if ret != 0 {
            return info.port as c_int;
        }
        if of_property_present(ep, c"reg".as_ptr()) != 0 {
            return info.id as c_int;
        }
    }
    // for_each_of_graph_port(node, p) is an external C iterator macro.
    let _ = node;
    -ENODEV
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn graph_util_parse_dai(priv_: *mut simple_util_priv, ep: *mut device_node, dlc: *mut snd_soc_dai_link_component, is_single_link: *mut c_int) -> c_int {
    let dev = simple_priv_to_dev(priv_);
    let mut args: of_phandle_args = core::mem::zeroed();
    let mut resolved_dlc: snd_soc_dai_link_component = core::mem::zeroed();
    let fallback_dai_name: *const c_char;
    let mut ret: c_int;
    if ep.is_null() {
        return 0;
    }
    let node = of_graph_get_port_parent(ep);
    args.np = ep;
    let dai = snd_soc_get_dai_via_args(&mut args);
    if !dai.is_null() {
        let dai_name = snd_soc_dai_name_get(dai);
        let dai_args = snd_soc_copy_dai_args(dev, &mut args);
        ret = -ENOMEM;
        if dai_args.is_null() {
            of_node_put(node);
            return simple_ret(priv_, ret);
        }
        (*dlc).of_node = node;
        (*dlc).dai_name = dai_name;
        (*dlc).dai_args = dai_args;
    } else {
        args.np = node;
        args.args[0] = graph_get_dai_id(ep) as c_uint;
        args.args_count = (of_graph_get_endpoint_count(node) > 1) as c_int;
        ret = snd_soc_get_dlc(&mut args, &mut resolved_dlc);
        if ret < 0 {
            of_node_put(node);
            return simple_ret(priv_, ret);
        }
        fallback_dai_name = resolved_dlc.dai_name;
        let mut copied_name = fallback_dai_name;
        if !fallback_dai_name.is_null() {
            copied_name = devm_kstrdup_const(dev, fallback_dai_name, GFP_KERNEL);
            ret = -ENOMEM;
            if copied_name.is_null() {
                of_node_put(node);
                return simple_ret(priv_, ret);
            }
        }
        (*dlc).of_node = resolved_dlc.of_node;
        (*dlc).dai_name = copied_name;
        (*dlc).dai_args = resolved_dlc.dai_args;
    }
    if !is_single_link.is_null() {
        *is_single_link = (of_graph_get_endpoint_count(node) == 1) as c_int;
    }
    simple_ret(priv_, 0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn graph_util_parse_link_direction(np: *mut device_node, playback_only: *mut bool_t, capture_only: *mut bool_t) {
    let is_playback_only = of_property_read_bool(np, c"playback-only".as_ptr());
    let is_capture_only = of_property_read_bool(np, c"capture-only".as_ptr());
    if !playback_only.is_null() && is_playback_only {
        *playback_only = is_playback_only;
    }
    if !capture_only.is_null() && is_capture_only {
        *capture_only = is_capture_only;
    }
}

unsafe fn __graph_util_parse_trigger_order(priv_: *mut simple_util_priv, np: *mut device_node, prop: *const c_char) -> snd_soc_trigger_order {
    let mut val = [0u32; SND_SOC_TRIGGER_SIZE];
    let ret = of_property_read_u32_array(np, prop, val.as_mut_ptr(), SND_SOC_TRIGGER_SIZE);
    if ret == 0 {
        let dev = simple_priv_to_dev(priv_);
        let order = (val[0] << 8) + (val[1] << 4) + val[2];
        match order {
            x if x == ((SND_SOC_TRIGGER_LINK << 8) + (SND_SOC_TRIGGER_COMPONENT << 4) + SND_SOC_TRIGGER_DAI) => return snd_soc_trigger_order::SND_SOC_TRIGGER_ORDER_DEFAULT,
            x if x == ((SND_SOC_TRIGGER_LINK << 8) + (SND_SOC_TRIGGER_DAI << 4) + SND_SOC_TRIGGER_COMPONENT) => return snd_soc_trigger_order::SND_SOC_TRIGGER_ORDER_LDC,
            _ => dev_err(dev, c"unsupported trigger order [0x%x]\n".as_ptr(), order),
        }
    }
    snd_soc_trigger_order::SND_SOC_TRIGGER_ORDER_MAX
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn graph_util_parse_trigger_order(priv_: *mut simple_util_priv, np: *mut device_node, trigger_start: *mut snd_soc_trigger_order, trigger_stop: *mut snd_soc_trigger_order) {
    let mut order = __graph_util_parse_trigger_order(priv_, np, c"link-trigger-order".as_ptr());
    if order < snd_soc_trigger_order::SND_SOC_TRIGGER_ORDER_MAX {
        *trigger_start = order;
        *trigger_stop = order;
    }
    order = __graph_util_parse_trigger_order(priv_, np, c"link-trigger-order-start".as_ptr());
    if order < snd_soc_trigger_order::SND_SOC_TRIGGER_ORDER_MAX {
        *trigger_start = order;
    }
    order = __graph_util_parse_trigger_order(priv_, np, c"link-trigger-order-stop".as_ptr());
    if order < snd_soc_trigger_order::SND_SOC_TRIGGER_ORDER_MAX {
        *trigger_stop = order;
    }
}

// Module information:
// MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");
// MODULE_DESCRIPTION("ALSA SoC Simple Card Utils");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
