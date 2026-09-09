/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are referenced here.

#[repr(C)]
pub struct simple_util_tdm_width_map {
    pub sample_bits: u8,
    pub slot_count: u8,
    pub slot_width: u16,
}

#[repr(C)]
pub struct simple_util_dai {
    pub name: *const ::std::os::raw::c_char,
    pub sysclk: u32,
    pub clk_direction: ::std::os::raw::c_int,
    pub slots: ::std::os::raw::c_int,
    pub slot_width: ::std::os::raw::c_int,
    pub tx_slot_mask: u32,
    pub rx_slot_mask: u32,
    pub clk: *mut clk,
    pub clk_fixed: bool,
    pub tdm_width_map: *mut simple_util_tdm_width_map,
    pub n_tdm_widths: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct simple_util_data {
    pub convert_rate: u32,
    pub convert_channels: u32,
    pub convert_sample_format: *const ::std::os::raw::c_char,
}

#[repr(C)]
pub struct simple_util_jack {
    pub jack: snd_soc_jack,
    pub pin: snd_soc_jack_pin,
    pub gpio: snd_soc_jack_gpio,
}

#[repr(C)]
pub struct prop_nums {
    pub cpus: ::std::os::raw::c_int,
    pub codecs: ::std::os::raw::c_int,
    pub platforms: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum simple_util_sysclk_order {
    SIMPLE_SYSCLK_ORDER_CODEC_FIRST = 0,
    SIMPLE_SYSCLK_ORDER_CPU_FIRST,
}

#[repr(C)]
pub struct simple_dai_props {
    pub cpu_dai: *mut simple_util_dai,
    pub codec_dai: *mut simple_util_dai,
    pub adata: simple_util_data,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num: prop_nums,
    pub mclk_fs: u32,
    pub sysclk_order: simple_util_sysclk_order,
}

#[repr(C)]
pub struct simple_util_priv {
    pub snd_card: snd_soc_card,
    pub dai_props: *mut simple_dai_props,
    pub hp_jack: simple_util_jack,
    pub mic_jack: simple_util_jack,
    pub aux_jacks: *mut snd_soc_jack,
    pub dai_link: *mut snd_soc_dai_link,
    pub dais: *mut simple_util_dai,
    pub dlcs: *mut snd_soc_dai_link_component,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub pa_gpio: *mut gpio_desc,
    pub ops: *const snd_soc_ops,
    pub dpcm_selectable: u32,
    pub force_dpcm: u32,
}

pub const SNDRV_MAX_LINKS: usize = 512;

#[repr(C)]
pub struct link_info {
    pub link: ::std::os::raw::c_int,
    pub cpu: ::std::os::raw::c_int,
    pub num: [prop_nums; SNDRV_MAX_LINKS],
}

#[macro_export]
macro_rules! simple_util_init_hp { ($card:expr, $sjack:expr, $prefix:expr) => { unsafe { simple_util_init_jack($card, $sjack, 1, $prefix, ::std::ptr::null_mut()) } }; }
#[macro_export]
macro_rules! simple_util_init_mic { ($card:expr, $sjack:expr, $prefix:expr) => { unsafe { simple_util_init_jack($card, $sjack, 0, $prefix, ::std::ptr::null_mut()) } }; }

#[inline]
pub unsafe fn simple_priv_to_card(priv_: *mut simple_util_priv) -> *mut snd_soc_card { &mut (*priv_).snd_card }
#[inline]
pub unsafe fn simple_priv_to_props(priv_: *mut simple_util_priv, i: isize) -> *mut simple_dai_props { (*priv_).dai_props.offset(i) }
#[inline]
pub unsafe fn simple_priv_to_dev(priv_: *mut simple_util_priv) -> *mut device { (*simple_priv_to_card(priv_)).dev }
#[inline]
pub unsafe fn simple_priv_to_link(priv_: *mut simple_util_priv, i: isize) -> *mut snd_soc_dai_link { (*simple_priv_to_card(priv_)).dai_link.offset(i) }
#[inline]
pub unsafe fn simple_props_to_dlc_cpu(props: *mut simple_dai_props, i: isize) -> *mut snd_soc_dai_link_component { (*props).num.cpus as *mut snd_soc_dai_link_component /* external array base supplied by layout */ .offset(i) }
#[inline]
pub unsafe fn simple_props_to_dlc_codec(props: *mut simple_dai_props, i: isize) -> *mut snd_soc_dai_link_component { (*props).num.codecs as *mut snd_soc_dai_link_component .offset(i) }
#[inline]
pub unsafe fn simple_props_to_dlc_platform(props: *mut simple_dai_props, i: isize) -> *mut snd_soc_dai_link_component { (*props).num.platforms as *mut snd_soc_dai_link_component .offset(i) }
#[inline]
pub unsafe fn simple_props_to_dai_cpu(props: *mut simple_dai_props, i: isize) -> *mut simple_util_dai { (*props).cpu_dai.offset(i) }
#[inline]
pub unsafe fn simple_props_to_dai_codec(props: *mut simple_dai_props, i: isize) -> *mut simple_util_dai { (*props).codec_dai.offset(i) }
#[inline]
pub unsafe fn simple_props_to_codec_conf(props: *mut simple_dai_props, i: isize) -> *mut snd_soc_codec_conf { (*props).codec_conf.offset(i) }

extern "C" {
    pub fn simple_util_parse_daifmt(dev: *mut device, node: *mut device_node, codec: *mut device_node, prefix: *mut ::std::os::raw::c_char, retfmt: *mut u32) -> ::std::os::raw::c_int;
    pub fn simple_util_parse_tdm_width_map(priv_: *mut simple_util_priv, np: *mut device_node, dai: *mut simple_util_dai) -> ::std::os::raw::c_int;
    pub fn simple_util_set_dailink_name(priv_: *mut simple_util_priv, dai_link: *mut snd_soc_dai_link, fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    pub fn simple_util_parse_card_name(priv_: *mut simple_util_priv, prefix: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn simple_util_parse_clk(dev: *mut device, node: *mut device_node, simple_dai: *mut simple_util_dai, dlc: *mut snd_soc_dai_link_component) -> ::std::os::raw::c_int;
    pub fn simple_util_startup(substream: *mut snd_pcm_substream) -> ::std::os::raw::c_int;
    pub fn simple_util_shutdown(substream: *mut snd_pcm_substream);
    pub fn simple_util_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> ::std::os::raw::c_int;
    pub fn simple_util_dai_init(rtd: *mut snd_soc_pcm_runtime) -> ::std::os::raw::c_int;
    pub fn simple_util_be_hw_params_fixup(rtd: *mut snd_soc_pcm_runtime, params: *mut snd_pcm_hw_params) -> ::std::os::raw::c_int;
    pub fn simple_util_canonicalize_platform(platforms: *mut snd_soc_dai_link_component, cpus: *mut snd_soc_dai_link_component);
    pub fn simple_util_canonicalize_cpu(cpus: *mut snd_soc_dai_link_component, is_single_links: ::std::os::raw::c_int);
    pub fn simple_util_clean_reference(priv_: *mut simple_util_priv);
    pub fn simple_util_parse_convert(np: *mut device_node, prefix: *mut ::std::os::raw::c_char, data: *mut simple_util_data);
    pub fn simple_util_is_convert_required(data: *const simple_util_data) -> bool;
    pub fn simple_util_get_sample_fmt(data: *mut simple_util_data) -> ::std::os::raw::c_int;
    pub fn simple_util_init_jack(card: *mut snd_soc_card, sjack: *mut simple_util_jack, is_hp: ::std::os::raw::c_int, prefix: *mut ::std::os::raw::c_char, pin: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn simple_util_init_aux_jacks(card: *mut snd_soc_card, prefix: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn simple_util_init_priv(priv_: *mut simple_util_priv, li: *mut link_info) -> ::std::os::raw::c_int;
    pub fn simple_util_remove(pdev: *mut platform_device);
    pub fn graph_util_card_probe(card: *mut snd_soc_card) -> ::std::os::raw::c_int;
    pub fn graph_util_is_ports0(port: *mut device_node) -> ::std::os::raw::c_int;
    pub fn graph_util_parse_dai(priv_: *mut simple_util_priv, ep: *mut device_node, dlc: *mut snd_soc_dai_link_component, is_single_link: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn graph_util_parse_link_direction(np: *mut device_node, is_playback_only: *mut bool, is_capture_only: *mut bool);
    pub fn graph_util_parse_trigger_order(priv_: *mut simple_util_priv, np: *mut device_node, trigger_start: *mut snd_soc_trigger_order, trigger_stop: *mut snd_soc_trigger_order);
}

#[inline]
pub unsafe fn simple_util_parse_routing(priv_: *mut simple_util_priv, prefix: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int { simple_util_parse_property(priv_, snd_soc_of_parse_audio_routing, prefix, b"routing\0" as *const u8 as *mut _) }
#[inline]
pub unsafe fn simple_util_parse_widgets(priv_: *mut simple_util_priv, prefix: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int { simple_util_parse_property(priv_, snd_soc_of_parse_audio_simple_widgets, prefix, b"widgets\0" as *const u8 as *mut _) }
#[inline]
pub unsafe fn simple_util_parse_pin_switches(priv_: *mut simple_util_priv, prefix: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int { simple_util_parse_property(priv_, snd_soc_of_parse_pin_switches, prefix, b"pin-switches\0" as *const u8 as *mut _) }
#[inline]
pub unsafe fn simple_util_parse_aux_devs(priv_: *mut simple_util_priv, prefix: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int { simple_util_parse_property(priv_, snd_soc_of_parse_aux_devs, prefix, b"aux-devs\0" as *const u8 as *mut _) }

#[macro_export]
macro_rules! simple_util_parse_tdm { ($np:expr, $dai:expr) => { unsafe { snd_soc_of_parse_tdm_slot($np, &mut (*$dai).tx_slot_mask, &mut (*$dai).rx_slot_mask, &mut (*$dai).slots, &mut (*$dai).slot_width) } }; }

// C iteration macros preserve their assignment-and-condition semantics here.
#[macro_export]
macro_rules! for_each_prop_dai_cpu { ($props:expr, $i:ident, $cpu:ident, $body:block) => {{ let mut $i: isize = 0; while $i < unsafe { (*$props).num.cpus as isize } { let $cpu = unsafe { simple_props_to_dai_cpu($props, $i) }; if !$cpu.is_null() $body $i += 1; } }}; }
#[macro_export]
macro_rules! for_each_prop_dai_codec { ($props:expr, $i:ident, $codec:ident, $body:block) => {{ let mut $i: isize = 0; while $i < unsafe { (*$props).num.codecs as isize } { let $codec = unsafe { simple_props_to_dai_codec($props, $i) }; if !$codec.is_null() $body $i += 1; } }}; }
#[macro_export]
macro_rules! for_each_prop_codec_conf { ($props:expr, $i:ident, $conf:ident, $body:block) => {{ let mut $i: isize = 0; while $i < unsafe { (*$props).num.codecs as isize } { let $conf = unsafe { if (*$props).codec_conf.is_null() { ::std::ptr::null_mut() } else { simple_props_to_codec_conf($props, $i) } }; if !$conf.is_null() $body $i += 1; } }}; }

// The DEBUG-only logging helpers retain their source-level conditional intent.
#[cfg(feature = "DEBUG")]
pub unsafe fn simple_util_debug_dai(_priv_: *mut simple_util_priv, _name: *mut ::std::os::raw::c_char, _dai: *mut simple_util_dai) {
    // Logging is supplied by the surrounding kernel environment.
}
#[cfg(feature = "DEBUG")]
pub unsafe fn simple_util_debug_info(_priv_: *mut simple_util_priv) {
    // Logging is supplied by the surrounding kernel environment.
}

extern "C" {
    pub fn simple_util_parse_property(priv_: *mut simple_util_priv, func: Option<unsafe extern "C" fn(*mut snd_soc_card, *const ::std::os::raw::c_char) -> ::std::os::raw::c_int>, prefix: *mut ::std::os::raw::c_char, property: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
