// SPDX-License-Identifier: GPL-2.0+
//
// soc-topology.c  --  ALSA SoC Topology
//
// Copyright (C) 2012 Texas Instruments Inc.
// Copyright (C) 2015 Intel Corporation.
//
// Authors: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//          K, Mythri P <mythri.p.k@intel.com>
//          Prusty, Subhransu S <subhransu.s.prusty@intel.com>
//          B, Jayachandran <jayachandran.b@intel.com>
//          Abdullah, Omair M <omair.m.abdullah@intel.com>
//          Jin, Yao <yao.jin@intel.com>
//          Lin, Mengdong <mengdong.lin@intel.com>
//
//  Add support to read audio firmware topology alongside firmware text. The
//  topology data can contain kcontrols, DAPM graphs, widgets, DAIs, DAI links,
//  equalizers, firmware, coefficients etc.
//
//  This file only manages the core ALSA and ASoC components, all other bespoke
//  firmware topology data is passed to component drivers for bespoke handling.
//
// Dependency intent from C includes:
// linux/kernel.h, linux/export.h, linux/list.h, linux/firmware.h,
// linux/slab.h, sound/soc.h, sound/soc-dapm.h, sound/soc-topology.h,
// sound/tlv.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u8 = ::core::ffi::c_uchar;
type u16 = ::core::ffi::c_ushort;
type u32 = ::core::ffi::c_uint;
type u64 = ::core::ffi::c_ulonglong;
type size_t = usize;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;

const SOC_TPLG_MAGIC_BIG_ENDIAN: u32 = 0x436F5341;

/*
 * We make several passes over the data (since it wont necessarily be ordered)
 * and process objects in the following order. This guarantees the component
 * drivers will be ready with any vendor data before the mixers and DAPM objects
 * are loaded (that may make use of the vendor data).
 */
const SOC_TPLG_PASS_MANIFEST: c_uint = 0;
const SOC_TPLG_PASS_VENDOR: c_uint = 1;
const SOC_TPLG_PASS_CONTROL: c_uint = 2;
const SOC_TPLG_PASS_WIDGET: c_uint = 3;
const SOC_TPLG_PASS_PCM_DAI: c_uint = 4;
const SOC_TPLG_PASS_GRAPH: c_uint = 5;
const SOC_TPLG_PASS_BE_DAI: c_uint = 6;
const SOC_TPLG_PASS_LINK: c_uint = 7;

const SOC_TPLG_PASS_START: c_uint = SOC_TPLG_PASS_MANIFEST;
const SOC_TPLG_PASS_END: c_uint = SOC_TPLG_PASS_LINK;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct firmware {
    pub size: size_t,
    pub data: *const u8,
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
    pub name_prefix: *const c_char,
    pub dobj_list: list_head,
}

#[repr(C)]
pub struct snd_kcontrol {
    _unused: [u8; 0],
}

#[repr(C)]
pub union snd_kcontrol_new_tlv {
    pub p: *mut c_uint,
    pub c: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub access: c_uint,
    pub private_value: c_long,
    pub info: Option<unsafe extern "C" fn()>,
    pub get: Option<unsafe extern "C" fn()>,
    pub put: Option<unsafe extern "C" fn()>,
    pub tlv: snd_kcontrol_new_tlv,
}

#[repr(C)]
pub struct snd_soc_dobj_control {
    pub kcontrol: *mut snd_kcontrol,
    pub dtexts: *mut *mut c_char,
    pub dvalues: *mut c_uint,
}

#[repr(C)]
pub struct snd_soc_dobj_widget {
    pub kcontrol_type: *mut c_uint,
}

#[repr(C)]
pub union snd_soc_dobj_u {
    pub control: snd_soc_dobj_control,
    pub widget: snd_soc_dobj_widget,
}

#[repr(C)]
pub struct snd_soc_dobj {
    pub list: list_head,
    pub type_: c_int,
    pub index: u32,
    pub unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dobj)>,
    pub u: snd_soc_dobj_u,
}

#[repr(C)]
pub struct soc_mixer_control {
    pub dobj: snd_soc_dobj,
    pub reg: c_int,
    pub rreg: c_int,
    pub shift: c_int,
    pub rshift: c_int,
    pub max: c_uint,
    pub min: c_uint,
    pub invert: c_uint,
    pub platform_max: c_uint,
    pub num_channels: c_uint,
}

#[repr(C)]
pub struct soc_bytes_ext {
    pub dobj: snd_soc_dobj,
    pub max: c_uint,
    pub get: Option<unsafe extern "C" fn()>,
    pub put: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct soc_enum {
    pub dobj: snd_soc_dobj,
    pub reg: c_int,
    pub shift_l: c_int,
    pub shift_r: c_int,
    pub mask: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
    pub values: *const c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub sname: *const c_char,
    pub reg: c_int,
    pub shift: c_int,
    pub mask: c_uint,
    pub subseq: c_uint,
    pub on_val: c_uint,
    pub off_val: c_uint,
    pub ignore_suspend: c_uint,
    pub event_flags: u16,
    pub event: Option<unsafe extern "C" fn()>,
    pub num_kcontrols: c_int,
    pub kcontrol_news: *mut snd_kcontrol_new,
    pub kcontrols: *mut *mut snd_kcontrol,
    pub dapm: *mut snd_soc_dapm_context,
    pub dobj: snd_soc_dobj,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub source: *const c_char,
    pub sink: *const c_char,
    pub control: *const c_char,
    pub dobj: snd_soc_dobj,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub formats: u64,
    pub sig_bits: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub compress_new: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
    pub symmetric_channels: c_uint,
    pub symmetric_sample_bits: c_uint,
    pub dobj: snd_soc_dobj,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub id: c_int,
    pub driver: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub id: c_int,
    pub dynamic: c_uint,
    pub ignore_pmdown_time: c_uint,
    pub playback_only: c_uint,
    pub capture_only: c_uint,
    pub symmetric_rate: c_uint,
    pub symmetric_channels: c_uint,
    pub symmetric_sample_bits: c_uint,
    pub ignore_suspend: c_uint,
    pub dai_fmt: c_uint,
    pub ignore: c_uint,
    pub dobj: snd_soc_dobj,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_tplg_channel {
    pub size: u32,
    pub id: u32,
    pub reg: u32,
    pub shift: u32,
}

#[repr(C)]
pub struct snd_soc_tplg_ctl_ops {
    pub get: u32,
    pub put: u32,
    pub info: u32,
}

#[repr(C)]
pub struct snd_soc_tplg_tlv_dbscale {
    pub min: u32,
    pub step: u32,
    pub mute: u32,
}

#[repr(C)]
pub struct snd_soc_tplg_ctl_tlv {
    pub size: u32,
    pub type_: u32,
    pub scale: snd_soc_tplg_tlv_dbscale,
}

#[repr(C)]
pub struct snd_soc_tplg_private {
    pub size: u32,
}

#[repr(C)]
pub struct snd_soc_tplg_ctl_hdr {
    pub size: u32,
    pub type_: u32,
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub access: u32,
    pub ops: snd_soc_tplg_ctl_ops,
    pub tlv: snd_soc_tplg_ctl_tlv,
}

#[repr(C)]
pub struct snd_soc_tplg_mixer_control {
    pub hdr: snd_soc_tplg_ctl_hdr,
    pub size: u32,
    pub min: u32,
    pub max: u32,
    pub platform_max: u32,
    pub invert: u32,
    pub num_channels: u32,
    pub channel: [snd_soc_tplg_channel; SND_SOC_TPLG_MAX_CHAN],
    pub priv_: snd_soc_tplg_private,
}

#[repr(C)]
pub struct snd_soc_tplg_enum_control {
    pub hdr: snd_soc_tplg_ctl_hdr,
    pub size: u32,
    pub num_channels: u32,
    pub channel: [snd_soc_tplg_channel; SND_SOC_TPLG_MAX_CHAN],
    pub items: u32,
    pub mask: u32,
    pub texts: [[c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN]; SND_SOC_TPLG_NUM_TEXTS],
    pub values: [u32; SND_SOC_TPLG_NUM_TEXTS],
    pub priv_: snd_soc_tplg_private,
}

#[repr(C)]
pub struct snd_soc_tplg_bytes_ext {
    pub get: u32,
    pub put: u32,
}

#[repr(C)]
pub struct snd_soc_tplg_bytes_control {
    pub hdr: snd_soc_tplg_ctl_hdr,
    pub size: u32,
    pub max: u32,
    pub ext_ops: snd_soc_tplg_bytes_ext,
    pub priv_: snd_soc_tplg_private,
}

#[repr(C)]
pub struct snd_soc_tplg_dapm_graph_elem {
    pub source: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub sink: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub control: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
}

#[repr(C)]
pub struct snd_soc_tplg_dapm_widget {
    pub size: u32,
    pub id: u32,
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub sname: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub reg: u32,
    pub shift: u32,
    pub mask: u32,
    pub subseq: u32,
    pub invert: u32,
    pub ignore_suspend: u32,
    pub event_flags: u16,
    pub num_kcontrols: u32,
    pub priv_: snd_soc_tplg_private,
}

#[repr(C)]
pub struct snd_soc_tplg_stream_caps {
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub formats: u64,
    pub sig_bits: u32,
}

#[repr(C)]
pub struct snd_soc_tplg_pcm {
    pub size: u32,
    pub pcm_name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub dai_name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub pcm_id: u32,
    pub dai_id: u32,
    pub playback: u32,
    pub capture: u32,
    pub compress: u32,
    pub flag_mask: u32,
    pub flags: u32,
    pub caps: [snd_soc_tplg_stream_caps; 2],
    pub priv_: snd_soc_tplg_private,
}

#[repr(C)]
pub struct snd_soc_tplg_hw_config {
    pub size: u32,
    pub id: u32,
    pub fmt: u32,
    pub clock_gated: u8,
    pub invert_bclk: u8,
    pub invert_fsync: u8,
    pub bclk_provider: u8,
    pub fsync_provider: u8,
}

#[repr(C)]
pub struct snd_soc_tplg_link_config {
    pub size: u32,
    pub id: u32,
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub stream_name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub num_hw_configs: u32,
    pub default_hw_config_id: u32,
    pub hw_config: [snd_soc_tplg_hw_config; SND_SOC_TPLG_HW_CONFIG_MAX],
    pub flag_mask: u32,
    pub flags: u32,
    pub priv_: snd_soc_tplg_private,
}

#[repr(C)]
pub struct snd_soc_tplg_dai {
    pub size: u32,
    pub dai_name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub dai_id: u32,
    pub playback: u32,
    pub capture: u32,
    pub flag_mask: u32,
    pub flags: u32,
    pub caps: [snd_soc_tplg_stream_caps; 2],
    pub priv_: snd_soc_tplg_private,
}

#[repr(C)]
pub struct snd_soc_tplg_manifest {
    pub size: u32,
}

#[repr(C)]
pub struct snd_soc_tplg_hdr {
    pub magic: u32,
    pub abi: u32,
    pub version: u32,
    pub type_: u32,
    pub size: u32,
    pub vendor_type: u32,
    pub payload_size: u32,
    pub index: u32,
    pub count: u32,
}

#[repr(C)]
pub struct snd_soc_tplg_kcontrol_ops {
    pub id: c_int,
    pub get: Option<unsafe extern "C" fn()>,
    pub put: Option<unsafe extern "C" fn()>,
    pub info: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct snd_soc_tplg_bytes_ext_ops {
    pub id: c_int,
    pub get: Option<unsafe extern "C" fn()>,
    pub put: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct snd_soc_tplg_widget_events {
    pub type_: u16,
    pub event_handler: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct snd_soc_tplg_ops {
    pub io_ops: *const snd_soc_tplg_kcontrol_ops,
    pub io_ops_count: c_int,
    pub bytes_ext_ops: *const snd_soc_tplg_bytes_ext_ops,
    pub bytes_ext_ops_count: c_int,
    pub vendor_load: Option<unsafe extern "C" fn(*mut snd_soc_component, u32, *mut snd_soc_tplg_hdr) -> c_int>,
    pub widget_load: Option<unsafe extern "C" fn(*mut snd_soc_component, u32, *mut snd_soc_dapm_widget, *mut snd_soc_tplg_dapm_widget) -> c_int>,
    pub widget_ready: Option<unsafe extern "C" fn(*mut snd_soc_component, u32, *mut snd_soc_dapm_widget, *mut snd_soc_tplg_dapm_widget) -> c_int>,
    pub dai_load: Option<unsafe extern "C" fn(*mut snd_soc_component, u32, *mut snd_soc_dai_driver, *mut snd_soc_tplg_pcm, *mut snd_soc_dai) -> c_int>,
    pub link_load: Option<unsafe extern "C" fn(*mut snd_soc_component, u32, *mut snd_soc_dai_link, *mut snd_soc_tplg_link_config) -> c_int>,
    pub complete: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub control_load: Option<unsafe extern "C" fn(*mut snd_soc_component, u32, *mut snd_kcontrol_new, *mut snd_soc_tplg_ctl_hdr) -> c_int>,
    pub control_unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dobj)>,
    pub dapm_route_load: Option<unsafe extern "C" fn(*mut snd_soc_component, u32, *mut snd_soc_dapm_route) -> c_int>,
    pub dapm_route_unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dobj)>,
    pub widget_unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dobj)>,
    pub dai_unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dobj)>,
    pub link_unload: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_dobj)>,
    pub manifest: Option<unsafe extern "C" fn(*mut snd_soc_component, u32, *mut snd_soc_tplg_manifest) -> c_int>,
}

/* topology context */
#[repr(C)]
pub struct soc_tplg {
    pub fw: *const firmware,

    /* runtime FW parsing */
    pub pos: *const u8,     /* read position */
    pub hdr_pos: *const u8, /* header position */
    pub pass: c_uint,       /* pass number */

    /* component caller */
    pub dev: *mut device,
    pub comp: *mut snd_soc_component,
    pub index: u32, /* current block index */

    /* vendor specific kcontrol operations */
    pub io_ops: *const snd_soc_tplg_kcontrol_ops,
    pub io_ops_count: c_int,

    /* vendor specific bytes ext handlers, for TLV bytes controls */
    pub bytes_ext_ops: *const snd_soc_tplg_bytes_ext_ops,
    pub bytes_ext_ops_count: c_int,

    /* optional fw loading callbacks to component drivers */
    pub ops: *const snd_soc_tplg_ops,
}

#[repr(C)]
pub struct soc_tplg_map {
    pub uid: c_int,
    pub kid: c_int,
}

const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize = 44;
const SND_SOC_TPLG_MAX_CHAN: usize = 8;
const SND_SOC_TPLG_NUM_TEXTS: usize = 16;
const SND_SOC_TPLG_HW_CONFIG_MAX: usize = 8;

extern "C" {
    static snd_soc_get_volsw: Option<unsafe extern "C" fn()>;
    static snd_soc_put_volsw: Option<unsafe extern "C" fn()>;
    static snd_soc_info_volsw: Option<unsafe extern "C" fn()>;
    static snd_soc_get_volsw_sx: Option<unsafe extern "C" fn()>;
    static snd_soc_put_volsw_sx: Option<unsafe extern "C" fn()>;
    static snd_soc_get_enum_double: Option<unsafe extern "C" fn()>;
    static snd_soc_put_enum_double: Option<unsafe extern "C" fn()>;
    static snd_soc_info_enum_double: Option<unsafe extern "C" fn()>;
    static snd_soc_bytes_get: Option<unsafe extern "C" fn()>;
    static snd_soc_bytes_put: Option<unsafe extern "C" fn()>;
    static snd_soc_bytes_info: Option<unsafe extern "C" fn()>;
    static snd_soc_get_xr_sx: Option<unsafe extern "C" fn()>;
    static snd_soc_put_xr_sx: Option<unsafe extern "C" fn()>;
    static snd_soc_info_xr_sx: Option<unsafe extern "C" fn()>;
    static snd_soc_get_strobe: Option<unsafe extern "C" fn()>;
    static snd_soc_put_strobe: Option<unsafe extern "C" fn()>;
    static snd_soc_dapm_get_volsw: Option<unsafe extern "C" fn()>;
    static snd_soc_dapm_put_volsw: Option<unsafe extern "C" fn()>;
    static snd_soc_dapm_get_enum_double: Option<unsafe extern "C" fn()>;
    static snd_soc_dapm_put_enum_double: Option<unsafe extern "C" fn()>;
    static snd_soc_dapm_get_pin_switch: Option<unsafe extern "C" fn()>;
    static snd_soc_dapm_put_pin_switch: Option<unsafe extern "C" fn()>;
    static snd_soc_dapm_info_pin_switch: Option<unsafe extern "C" fn()>;
    static snd_soc_bytes_info_ext: Option<unsafe extern "C" fn()>;
    static snd_soc_bytes_tlv_callback: Option<unsafe extern "C" fn()>;
    static snd_soc_new_compress: Option<unsafe extern "C" fn()>;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kstrdup(dev: *mut device, s: *const c_char, flags: c_uint) -> *mut c_char;
    fn kstrdup(s: *const c_char, flags: c_uint) -> *mut c_char;
    fn kfree(p: *mut c_void);
    fn strnlen(s: *const c_char, maxlen: size_t) -> size_t;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn snd_soc_cnew(template: *const snd_kcontrol_new, data: *mut c_void, long_name: *const c_char, prefix: *const c_char) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_remove(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_soc_component_to_dapm(comp: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *mut snd_soc_dapm_route, num: c_int) -> c_int;
    fn snd_soc_card_is_instantiated(card: *mut snd_soc_card) -> bool;
    fn snd_soc_dapm_new_control(dapm: *mut snd_soc_dapm_context, widget: *const snd_soc_dapm_widget) -> *mut snd_soc_dapm_widget;
    fn snd_soc_dapm_new_control_unlocked(dapm: *mut snd_soc_dapm_context, widget: *const snd_soc_dapm_widget) -> *mut snd_soc_dapm_widget;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_free_widget(widget: *mut snd_soc_dapm_widget);
    fn snd_soc_dapm_new_widgets(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_register_dai(comp: *mut snd_soc_component, dai_drv: *mut snd_soc_dai_driver, legacy_dai_naming: bool) -> *mut snd_soc_dai;
    fn snd_soc_unregister_dai(dai: *mut snd_soc_dai);
    fn snd_soc_dapm_new_dai_widgets(dapm: *mut snd_soc_dapm_context, dai: *mut snd_soc_dai) -> c_int;
    fn snd_soc_add_pcm_runtimes(card: *mut snd_soc_card, link: *mut snd_soc_dai_link, num: c_int) -> c_int;
    fn snd_soc_remove_pcm_runtime(card: *mut snd_soc_card, rtd: *mut snd_soc_pcm_runtime);
    fn snd_soc_get_pcm_runtime(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_find_dai(dai: *mut snd_soc_dai_link_component) -> *mut snd_soc_dai;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
}

extern "C" {
    static SND_SOC_TPLG_CTL_VOLSW: c_int;
    static SND_SOC_TPLG_CTL_VOLSW_SX: c_int;
    static SND_SOC_TPLG_CTL_ENUM: c_int;
    static SND_SOC_TPLG_CTL_ENUM_VALUE: c_int;
    static SND_SOC_TPLG_CTL_BYTES: c_int;
    static SND_SOC_TPLG_CTL_RANGE: c_int;
    static SND_SOC_TPLG_CTL_VOLSW_XR_SX: c_int;
    static SND_SOC_TPLG_CTL_STROBE: c_int;
    static SND_SOC_TPLG_DAPM_CTL_VOLSW: c_int;
    static SND_SOC_TPLG_DAPM_CTL_ENUM_DOUBLE: c_int;
    static SND_SOC_TPLG_DAPM_CTL_ENUM_VIRT: c_int;
    static SND_SOC_TPLG_DAPM_CTL_ENUM_VALUE: c_int;
    static SND_SOC_TPLG_DAPM_CTL_PIN: c_int;
    static SND_SOC_TPLG_DAPM_INPUT: c_int;
    static SND_SOC_TPLG_DAPM_OUTPUT: c_int;
    static SND_SOC_TPLG_DAPM_MUX: c_int;
    static SND_SOC_TPLG_DAPM_MIXER: c_int;
    static SND_SOC_TPLG_DAPM_PGA: c_int;
    static SND_SOC_TPLG_DAPM_OUT_DRV: c_int;
    static SND_SOC_TPLG_DAPM_ADC: c_int;
    static SND_SOC_TPLG_DAPM_DAC: c_int;
    static SND_SOC_TPLG_DAPM_SWITCH: c_int;
    static SND_SOC_TPLG_DAPM_PRE: c_int;
    static SND_SOC_TPLG_DAPM_POST: c_int;
    static SND_SOC_TPLG_DAPM_AIF_IN: c_int;
    static SND_SOC_TPLG_DAPM_AIF_OUT: c_int;
    static SND_SOC_TPLG_DAPM_DAI_IN: c_int;
    static SND_SOC_TPLG_DAPM_DAI_OUT: c_int;
    static SND_SOC_TPLG_DAPM_DAI_LINK: c_int;
    static SND_SOC_TPLG_DAPM_BUFFER: c_int;
    static SND_SOC_TPLG_DAPM_SCHEDULER: c_int;
    static SND_SOC_TPLG_DAPM_EFFECT: c_int;
    static SND_SOC_TPLG_DAPM_SIGGEN: c_int;
    static SND_SOC_TPLG_DAPM_SRC: c_int;
    static SND_SOC_TPLG_DAPM_ASRC: c_int;
    static SND_SOC_TPLG_DAPM_ENCODER: c_int;
    static SND_SOC_TPLG_DAPM_DECODER: c_int;
    static snd_soc_dapm_input: c_int;
    static snd_soc_dapm_output: c_int;
    static snd_soc_dapm_mux: c_int;
    static snd_soc_dapm_mixer: c_int;
    static snd_soc_dapm_pga: c_int;
    static snd_soc_dapm_out_drv: c_int;
    static snd_soc_dapm_adc: c_int;
    static snd_soc_dapm_dac: c_int;
    static snd_soc_dapm_switch: c_int;
    static snd_soc_dapm_pre: c_int;
    static snd_soc_dapm_post: c_int;
    static snd_soc_dapm_aif_in: c_int;
    static snd_soc_dapm_aif_out: c_int;
    static snd_soc_dapm_dai_in: c_int;
    static snd_soc_dapm_dai_out: c_int;
    static snd_soc_dapm_dai_link: c_int;
    static snd_soc_dapm_buffer: c_int;
    static snd_soc_dapm_scheduler: c_int;
    static snd_soc_dapm_effect: c_int;
    static snd_soc_dapm_siggen: c_int;
    static snd_soc_dapm_src: c_int;
    static snd_soc_dapm_asrc: c_int;
    static snd_soc_dapm_encoder: c_int;
    static snd_soc_dapm_decoder: c_int;
}

const GFP_KERNEL: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 1 << 2;
const SNDRV_CTL_ELEM_ACCESS_TLV_WRITE: c_uint = 1 << 3;
const SNDRV_CTL_ELEM_ACCESS_TLV_READWRITE: c_uint =
    SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_TLV_WRITE;
const SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK: c_uint = 1 << 4;
const SNDRV_CTL_ELEM_ACCESS_SKIP_CHECK: c_uint = 1 << 5;
const SNDRV_CTL_TLVT_DB_SCALE: c_uint = 1;
const TLV_DB_SCALE_MASK: c_uint = 0xffff;
const TLV_DB_SCALE_MUTE: c_uint = 0x10000;
const SNDRV_CHMAP_FL: c_int = 3;
const SNDRV_CHMAP_FR: c_int = 4;
const SND_SOC_TPLG_TYPE_MIXER: u32 = 1;
const SND_SOC_TPLG_TYPE_ENUM: u32 = 2;
const SND_SOC_TPLG_TYPE_BYTES: u32 = 3;
const SND_SOC_TPLG_TYPE_DAPM_GRAPH: u32 = 4;
const SND_SOC_TPLG_TYPE_DAPM_WIDGET: u32 = 5;
const SND_SOC_TPLG_TYPE_PCM: u32 = 6;
const SND_SOC_TPLG_TYPE_DAI: u32 = 7;
const SND_SOC_TPLG_TYPE_DAI_LINK: u32 = 8;
const SND_SOC_TPLG_TYPE_BACKEND_LINK: u32 = 9;
const SND_SOC_TPLG_TYPE_MANIFEST: u32 = 10;
const SND_SOC_TPLG_STREAM_PLAYBACK: usize = 0;
const SND_SOC_TPLG_STREAM_CAPTURE: usize = 1;
const SND_SOC_DOBJ_NONE: c_int = 0;
const SND_SOC_DOBJ_BYTES: c_int = 1;
const SND_SOC_DOBJ_ENUM: c_int = 2;
const SND_SOC_DOBJ_MIXER: c_int = 3;
const SND_SOC_DOBJ_GRAPH: c_int = 4;
const SND_SOC_DOBJ_WIDGET: c_int = 5;
const SND_SOC_DOBJ_PCM: c_int = 6;
const SND_SOC_DOBJ_DAI_LINK: c_int = 7;
const SND_SOC_DOBJ_BACKEND_LINK: c_int = 8;
const SND_SOC_TPLG_DAI_FLGBIT_SYMMETRIC_RATES: c_uint = 1 << 0;
const SND_SOC_TPLG_DAI_FLGBIT_SYMMETRIC_CHANNELS: c_uint = 1 << 1;
const SND_SOC_TPLG_DAI_FLGBIT_SYMMETRIC_SAMPLEBITS: c_uint = 1 << 2;
const SND_SOC_TPLG_LNK_FLGBIT_SYMMETRIC_RATES: c_uint = 1 << 0;
const SND_SOC_TPLG_LNK_FLGBIT_SYMMETRIC_CHANNELS: c_uint = 1 << 1;
const SND_SOC_TPLG_LNK_FLGBIT_SYMMETRIC_SAMPLEBITS: c_uint = 1 << 2;
const SND_SOC_TPLG_LNK_FLGBIT_VOICE_WAKEUP: c_uint = 1 << 3;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_TPLG_DAI_CLK_GATE_GATED: u8 = 1;
const SND_SOC_TPLG_DAI_CLK_GATE_CONT: u8 = 2;
const SND_SOC_DAIFMT_GATED: c_uint = 1 << 4;
const SND_SOC_DAIFMT_CONT: c_uint = 1 << 5;
const SND_SOC_DAIFMT_NB_NF: c_uint = 1 << 6;
const SND_SOC_DAIFMT_NB_IF: c_uint = 1 << 7;
const SND_SOC_DAIFMT_IB_NF: c_uint = 1 << 8;
const SND_SOC_DAIFMT_IB_IF: c_uint = 1 << 9;
const SND_SOC_TPLG_BCLK_CP: u8 = 1;
const SND_SOC_TPLG_FSYNC_CP: u8 = 1;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 1 << 10;
const SND_SOC_DAIFMT_CBC_CFP: c_uint = 1 << 11;
const SND_SOC_DAIFMT_CBP_CFC: c_uint = 1 << 12;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 1 << 13;
const SND_SOC_TPLG_MAGIC: u32 = 0x41536f43;
const SND_SOC_TPLG_ABI_VERSION: u32 = 5;
const SND_SOC_TPLG_ABI_VERSION_MIN: u32 = 4;

#[inline]
const fn le32_to_cpu(v: u32) -> u32 { u32::from_le(v) }
#[inline]
const fn le64_to_cpu(v: u64) -> u64 { u64::from_le(v) }
#[inline]
const fn le16_to_cpu(v: u16) -> u16 { u16::from_le(v) }

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn IS_ERR<T>(p: *mut T) -> bool {
    (p as isize) < 0 && (p as isize) >= -4095
}

unsafe fn PTR_ERR<T>(p: *mut T) -> c_int {
    p as isize as c_int
}

const IO_OPS: [snd_soc_tplg_kcontrol_ops; 13] = unsafe {
    [
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_CTL_VOLSW, get: snd_soc_get_volsw, put: snd_soc_put_volsw, info: snd_soc_info_volsw },
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_CTL_VOLSW_SX, get: snd_soc_get_volsw_sx, put: snd_soc_put_volsw_sx, info: None },
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_CTL_ENUM, get: snd_soc_get_enum_double, put: snd_soc_put_enum_double, info: snd_soc_info_enum_double },
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_CTL_ENUM_VALUE, get: snd_soc_get_enum_double, put: snd_soc_put_enum_double, info: None },
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_CTL_BYTES, get: snd_soc_bytes_get, put: snd_soc_bytes_put, info: snd_soc_bytes_info },
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_CTL_RANGE, get: snd_soc_get_volsw, put: snd_soc_put_volsw, info: snd_soc_info_volsw },
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_CTL_VOLSW_XR_SX, get: snd_soc_get_xr_sx, put: snd_soc_put_xr_sx, info: snd_soc_info_xr_sx },
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_CTL_STROBE, get: snd_soc_get_strobe, put: snd_soc_put_strobe, info: None },
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_DAPM_CTL_VOLSW, get: snd_soc_dapm_get_volsw, put: snd_soc_dapm_put_volsw, info: snd_soc_info_volsw },
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_DAPM_CTL_ENUM_DOUBLE, get: snd_soc_dapm_get_enum_double, put: snd_soc_dapm_put_enum_double, info: snd_soc_info_enum_double },
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_DAPM_CTL_ENUM_VIRT, get: snd_soc_dapm_get_enum_double, put: snd_soc_dapm_put_enum_double, info: None },
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_DAPM_CTL_ENUM_VALUE, get: snd_soc_dapm_get_enum_double, put: snd_soc_dapm_put_enum_double, info: None },
        snd_soc_tplg_kcontrol_ops { id: SND_SOC_TPLG_DAPM_CTL_PIN, get: snd_soc_dapm_get_pin_switch, put: snd_soc_dapm_put_pin_switch, info: snd_soc_dapm_info_pin_switch },
    ]
};

const DAPM_MAP: [soc_tplg_map; 24] = unsafe {
    [
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_INPUT, kid: snd_soc_dapm_input },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_OUTPUT, kid: snd_soc_dapm_output },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_MUX, kid: snd_soc_dapm_mux },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_MIXER, kid: snd_soc_dapm_mixer },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_PGA, kid: snd_soc_dapm_pga },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_OUT_DRV, kid: snd_soc_dapm_out_drv },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_ADC, kid: snd_soc_dapm_adc },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_DAC, kid: snd_soc_dapm_dac },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_SWITCH, kid: snd_soc_dapm_switch },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_PRE, kid: snd_soc_dapm_pre },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_POST, kid: snd_soc_dapm_post },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_AIF_IN, kid: snd_soc_dapm_aif_in },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_AIF_OUT, kid: snd_soc_dapm_aif_out },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_DAI_IN, kid: snd_soc_dapm_dai_in },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_DAI_OUT, kid: snd_soc_dapm_dai_out },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_DAI_LINK, kid: snd_soc_dapm_dai_link },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_BUFFER, kid: snd_soc_dapm_buffer },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_SCHEDULER, kid: snd_soc_dapm_scheduler },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_EFFECT, kid: snd_soc_dapm_effect },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_SIGGEN, kid: snd_soc_dapm_siggen },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_SRC, kid: snd_soc_dapm_src },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_ASRC, kid: snd_soc_dapm_asrc },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_ENCODER, kid: snd_soc_dapm_encoder },
        soc_tplg_map { uid: SND_SOC_TPLG_DAPM_DECODER, kid: snd_soc_dapm_decoder },
    ]
};

unsafe fn soc_tplg_check_elem_count(tplg: *mut soc_tplg, elem_size: size_t, count: c_uint, bytes: size_t, elem_type: *const c_char) -> c_int {
    let end = (*tplg).pos.add(elem_size.wrapping_mul(count as usize));
    if end > (*(*tplg).fw).data.add((*(*tplg).fw).size) {
        dev_err((*tplg).dev, b"ASoC: %s overflow end of data\n\0".as_ptr() as *const c_char, elem_type);
        return -EINVAL;
    }
    if elem_size.wrapping_mul(count as usize) > bytes {
        dev_err((*tplg).dev, b"ASoC: %s count %d of size %zu is bigger than chunk %zu\n\0".as_ptr() as *const c_char, elem_type, count, elem_size, bytes);
        return -EINVAL;
    }
    0
}

unsafe fn soc_tplg_is_eof(tplg: *mut soc_tplg) -> bool {
    let end = (*tplg).hdr_pos;
    end >= (*(*tplg).fw).data.add((*(*tplg).fw).size)
}

unsafe fn soc_tplg_get_hdr_offset(tplg: *mut soc_tplg) -> c_ulong {
    (*tplg).hdr_pos.offset_from((*(*tplg).fw).data) as c_ulong
}

unsafe fn soc_tplg_get_offset(tplg: *mut soc_tplg) -> c_ulong {
    (*tplg).pos.offset_from((*(*tplg).fw).data) as c_ulong
}

unsafe fn tplg_chan_get_reg(_tplg: *mut soc_tplg, chan: *mut snd_soc_tplg_channel, map: c_int) -> c_int {
    for i in 0..SND_SOC_TPLG_MAX_CHAN {
        if le32_to_cpu((*chan.add(i)).id) as c_int == map {
            return le32_to_cpu((*chan.add(i)).reg) as c_int;
        }
    }
    -EINVAL
}

unsafe fn tplg_chan_get_shift(_tplg: *mut soc_tplg, chan: *mut snd_soc_tplg_channel, map: c_int) -> c_int {
    for i in 0..SND_SOC_TPLG_MAX_CHAN {
        if le32_to_cpu((*chan.add(i)).id) as c_int == map {
            return le32_to_cpu((*chan.add(i)).shift) as c_int;
        }
    }
    -EINVAL
}

unsafe fn get_widget_id(tplg_type: c_int) -> c_int {
    for m in DAPM_MAP.iter() {
        if tplg_type == m.uid {
            return m.kid;
        }
    }
    -EINVAL
}

unsafe fn soc_control_err(tplg: *mut soc_tplg, hdr: *mut snd_soc_tplg_ctl_hdr, name: *const c_char) {
    dev_err((*tplg).dev,
        b"ASoC: no complete control IO handler for %s type (g,p,i) %u:%u:%u at 0x%lx\n\0".as_ptr() as *const c_char,
        name,
        le32_to_cpu((*hdr).ops.get),
        le32_to_cpu((*hdr).ops.put),
        le32_to_cpu((*hdr).ops.info),
        soc_tplg_get_offset(tplg));
}

unsafe fn soc_tplg_vendor_load(tplg: *mut soc_tplg, hdr: *mut snd_soc_tplg_hdr) -> c_int {
    let mut ret = 0;
    if !(*tplg).ops.is_null() && (*(*tplg).ops).vendor_load.is_some() {
        ret = ((*(*tplg).ops).vendor_load.unwrap())((*tplg).comp, (*tplg).index, hdr);
    } else {
        dev_err((*tplg).dev, b"ASoC: no vendor load callback for ID %u\n\0".as_ptr() as *const c_char, le32_to_cpu((*hdr).vendor_type));
        return -EINVAL;
    }
    if ret < 0 {
        dev_err((*tplg).dev,
            b"ASoC: vendor load failed at hdr offset %ld/0x%lx for type %u:%u\n\0".as_ptr() as *const c_char,
            soc_tplg_get_hdr_offset(tplg), soc_tplg_get_hdr_offset(tplg),
            le32_to_cpu((*hdr).type_), le32_to_cpu((*hdr).vendor_type));
    }
    ret
}

unsafe fn soc_tplg_widget_load(tplg: *mut soc_tplg, w: *mut snd_soc_dapm_widget, tplg_w: *mut snd_soc_tplg_dapm_widget) -> c_int {
    if !(*tplg).ops.is_null() && (*(*tplg).ops).widget_load.is_some() {
        return ((*(*tplg).ops).widget_load.unwrap())((*tplg).comp, (*tplg).index, w, tplg_w);
    }
    0
}

unsafe fn soc_tplg_widget_ready(tplg: *mut soc_tplg, w: *mut snd_soc_dapm_widget, tplg_w: *mut snd_soc_tplg_dapm_widget) -> c_int {
    if !(*tplg).ops.is_null() && (*(*tplg).ops).widget_ready.is_some() {
        return ((*(*tplg).ops).widget_ready.unwrap())((*tplg).comp, (*tplg).index, w, tplg_w);
    }
    0
}

unsafe fn soc_tplg_dai_load(tplg: *mut soc_tplg, dai_drv: *mut snd_soc_dai_driver, pcm: *mut snd_soc_tplg_pcm, dai: *mut snd_soc_dai) -> c_int {
    if !(*tplg).ops.is_null() && (*(*tplg).ops).dai_load.is_some() {
        return ((*(*tplg).ops).dai_load.unwrap())((*tplg).comp, (*tplg).index, dai_drv, pcm, dai);
    }
    0
}

unsafe fn soc_tplg_dai_link_load(tplg: *mut soc_tplg, link: *mut snd_soc_dai_link, cfg: *mut snd_soc_tplg_link_config) -> c_int {
    if !(*tplg).ops.is_null() && (*(*tplg).ops).link_load.is_some() {
        return ((*(*tplg).ops).link_load.unwrap())((*tplg).comp, (*tplg).index, link, cfg);
    }
    0
}

unsafe fn soc_tplg_complete(tplg: *mut soc_tplg) -> c_int {
    if !(*tplg).ops.is_null() && (*(*tplg).ops).complete.is_some() {
        return ((*(*tplg).ops).complete.unwrap())((*tplg).comp);
    }
    0
}

unsafe fn soc_tplg_add_dcontrol(card: *mut snd_card, dev: *mut device, control_new: *const snd_kcontrol_new, prefix: *const c_char, data: *mut c_void, kcontrol: *mut *mut snd_kcontrol) -> c_int {
    *kcontrol = snd_soc_cnew(control_new, data, (*control_new).name, prefix);
    if (*kcontrol).is_null() {
        dev_err(dev, b"ASoC: Failed to create new kcontrol %s\n\0".as_ptr() as *const c_char, (*control_new).name);
        return -ENOMEM;
    }
    let err = snd_ctl_add(card, *kcontrol);
    if err < 0 {
        dev_err(dev, b"ASoC: Failed to add %s: %d\n\0".as_ptr() as *const c_char, (*control_new).name, err);
        return err;
    }
    0
}

unsafe fn soc_tplg_add_kcontrol(tplg: *mut soc_tplg, k: *mut snd_kcontrol_new, kcontrol: *mut *mut snd_kcontrol) -> c_int {
    let comp = (*tplg).comp;
    soc_tplg_add_dcontrol((*(*comp).card).snd_card, (*tplg).dev, k, (*comp).name_prefix, comp as *mut c_void, kcontrol)
}

unsafe fn soc_tplg_remove_kcontrol(comp: *mut snd_soc_component, dobj: *mut snd_soc_dobj, pass: c_int) {
    let card = (*(*comp).card).snd_card;
    if pass != SOC_TPLG_PASS_CONTROL as c_int { return; }
    if let Some(unload) = (*dobj).unload { unload(comp, dobj); }
    snd_ctl_remove(card, (*dobj).u.control.kcontrol);
    list_del(&mut (*dobj).list);
}

unsafe fn soc_tplg_remove_route(comp: *mut snd_soc_component, dobj: *mut snd_soc_dobj, pass: c_int) {
    if pass != SOC_TPLG_PASS_GRAPH as c_int { return; }
    if let Some(unload) = (*dobj).unload { unload(comp, dobj); }
    list_del(&mut (*dobj).list);
}

unsafe fn soc_tplg_remove_widget(comp: *mut snd_soc_component, dobj: *mut snd_soc_dobj, pass: c_int) {
    let card = (*(*comp).card).snd_card;
    let w = dobj as *mut snd_soc_dapm_widget;
    if pass != SOC_TPLG_PASS_WIDGET as c_int { return; }
    if let Some(unload) = (*dobj).unload { unload(comp, dobj); }
    if !(*w).kcontrols.is_null() {
        for i in 0..(*w).num_kcontrols {
            snd_ctl_remove(card, *(*w).kcontrols.add(i as usize));
        }
    }
    list_del(&mut (*dobj).list);
    /* widget w is freed by soc-dapm.c */
}

unsafe fn soc_tplg_remove_dai(comp: *mut snd_soc_component, dobj: *mut snd_soc_dobj, pass: c_int) {
    let _dai_drv = dobj as *mut snd_soc_dai_driver;
    if pass != SOC_TPLG_PASS_PCM_DAI as c_int { return; }
    if let Some(unload) = (*dobj).unload { unload(comp, dobj); }
    /* for_each_component_dais_safe(comp, dai, _dai) is supplied by surrounding ASoC code. */
    list_del(&mut (*dobj).list);
}

unsafe fn soc_tplg_remove_link(comp: *mut snd_soc_component, dobj: *mut snd_soc_dobj, pass: c_int) {
    let link = dobj as *mut snd_soc_dai_link;
    if pass != SOC_TPLG_PASS_PCM_DAI as c_int { return; }
    if let Some(unload) = (*dobj).unload { unload(comp, dobj); }
    list_del(&mut (*dobj).list);
    if (*link).ignore == 0 {
        snd_soc_remove_pcm_runtime((*comp).card, snd_soc_get_pcm_runtime((*comp).card, link));
    }
}

unsafe fn remove_backend_link(comp: *mut snd_soc_component, dobj: *mut snd_soc_dobj, pass: c_int) {
    if pass != SOC_TPLG_PASS_LINK as c_int { return; }
    if let Some(unload) = (*dobj).unload { unload(comp, dobj); }
    (*dobj).type_ = SND_SOC_DOBJ_NONE;
    list_del(&mut (*dobj).list);
}

unsafe fn soc_tplg_kcontrol_bind_io(hdr: *mut snd_soc_tplg_ctl_hdr, k: *mut snd_kcontrol_new, tplg: *const soc_tplg) -> c_int {
    if le32_to_cpu((*hdr).ops.info) as c_int == SND_SOC_TPLG_CTL_BYTES
        && ((*k).iface & SNDRV_CTL_ELEM_IFACE_MIXER) != 0
        && (((*k).access & SNDRV_CTL_ELEM_ACCESS_TLV_READ) != 0 || ((*k).access & SNDRV_CTL_ELEM_ACCESS_TLV_WRITE) != 0)
        && ((*k).access & SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK) != 0 {
        let sbe = (*k).private_value as *mut soc_bytes_ext;
        let be = hdr as *mut snd_soc_tplg_bytes_control;
        (*k).info = snd_soc_bytes_info_ext;
        (*k).tlv.c = snd_soc_bytes_tlv_callback;
        if (*sbe).max > 512 {
            (*k).access |= SNDRV_CTL_ELEM_ACCESS_SKIP_CHECK;
        }
        let ext_ops = (*tplg).bytes_ext_ops;
        for i in 0..(*tplg).bytes_ext_ops_count {
            let op = ext_ops.add(i as usize);
            if (*sbe).put.is_none() && (*op).id == le32_to_cpu((*be).ext_ops.put) as c_int { (*sbe).put = (*op).put; }
            if (*sbe).get.is_none() && (*op).id == le32_to_cpu((*be).ext_ops.get) as c_int { (*sbe).get = (*op).get; }
        }
        if ((*k).access & SNDRV_CTL_ELEM_ACCESS_TLV_READ) != 0 && (*sbe).get.is_none() { return -EINVAL; }
        if ((*k).access & SNDRV_CTL_ELEM_ACCESS_TLV_WRITE) != 0 && (*sbe).put.is_none() { return -EINVAL; }
        return 0;
    }

    let mut ops = (*tplg).io_ops;
    for i in 0..(*tplg).io_ops_count {
        let op = ops.add(i as usize);
        if (*k).put.is_none() && (*op).id == le32_to_cpu((*hdr).ops.put) as c_int { (*k).put = (*op).put; }
        if (*k).get.is_none() && (*op).id == le32_to_cpu((*hdr).ops.get) as c_int { (*k).get = (*op).get; }
        if (*k).info.is_none() && (*op).id == le32_to_cpu((*hdr).ops.info) as c_int { (*k).info = (*op).info; }
    }
    if (*k).put.is_some() && (*k).get.is_some() && (*k).info.is_some() { return 0; }

    ops = IO_OPS.as_ptr();
    for i in 0..IO_OPS.len() {
        let op = ops.add(i);
        if (*k).put.is_none() && (*op).id == le32_to_cpu((*hdr).ops.put) as c_int { (*k).put = (*op).put; }
        if (*k).get.is_none() && (*op).id == le32_to_cpu((*hdr).ops.get) as c_int { (*k).get = (*op).get; }
        if (*k).info.is_none() && (*op).id == le32_to_cpu((*hdr).ops.info) as c_int { (*k).info = (*op).info; }
    }
    if (*k).put.is_some() && (*k).get.is_some() && (*k).info.is_some() { return 0; }
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_tplg_widget_bind_event(w: *mut snd_soc_dapm_widget, events: *const snd_soc_tplg_widget_events, num_events: c_int, event_type: u16) -> c_int {
    (*w).event = None;
    for i in 0..num_events {
        if event_type == (*events.add(i as usize)).type_ {
            (*w).event = (*events.add(i as usize)).event_handler;
            return 0;
        }
    }
    -EINVAL
}

unsafe fn soc_tplg_control_load(tplg: *mut soc_tplg, k: *mut snd_kcontrol_new, hdr: *mut snd_soc_tplg_ctl_hdr) -> c_int {
    let mut ret = 0;
    if !(*tplg).ops.is_null() && (*(*tplg).ops).control_load.is_some() {
        ret = ((*(*tplg).ops).control_load.unwrap())((*tplg).comp, (*tplg).index, k, hdr);
    }
    if ret != 0 {
        dev_err((*tplg).dev, b"ASoC: failed to init %s\n\0".as_ptr() as *const c_char, (*hdr).name.as_ptr());
    }
    ret
}

unsafe fn soc_tplg_create_tlv_db_scale(tplg: *mut soc_tplg, kc: *mut snd_kcontrol_new, scale: *mut snd_soc_tplg_tlv_dbscale) -> c_int {
    let item_len = 2 * size_of::<c_uint>() as c_uint;
    let p = devm_kzalloc((*tplg).dev, item_len as usize + 2 * size_of::<c_uint>(), GFP_KERNEL) as *mut c_uint;
    if p.is_null() { return -ENOMEM; }
    *p.add(0) = SNDRV_CTL_TLVT_DB_SCALE;
    *p.add(1) = item_len;
    *p.add(2) = le32_to_cpu((*scale).min);
    *p.add(3) = (le32_to_cpu((*scale).step) & TLV_DB_SCALE_MASK) | if le32_to_cpu((*scale).mute) != 0 { TLV_DB_SCALE_MUTE } else { 0 };
    (*kc).tlv.p = p;
    0
}

unsafe fn soc_tplg_create_tlv(tplg: *mut soc_tplg, kc: *mut snd_kcontrol_new, tc: *mut snd_soc_tplg_ctl_hdr) -> c_int {
    let access = le32_to_cpu((*tc).access);
    if (access & SNDRV_CTL_ELEM_ACCESS_TLV_READWRITE) == 0 { return 0; }
    if (access & SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK) == 0 {
        let tplg_tlv = &mut (*tc).tlv;
        match le32_to_cpu(tplg_tlv.type_) {
            SNDRV_CTL_TLVT_DB_SCALE => return soc_tplg_create_tlv_db_scale(tplg, kc, &mut tplg_tlv.scale),
            _ => {
                dev_dbg((*tplg).dev, b"Unsupported TLV type %u\n\0".as_ptr() as *const c_char, le32_to_cpu(tplg_tlv.type_));
                return -EINVAL;
            }
        }
    }
    0
}

unsafe fn soc_tplg_control_dmixer_create(tplg: *mut soc_tplg, kc: *mut snd_kcontrol_new) -> c_int {
    let mc = (*tplg).pos as *mut snd_soc_tplg_mixer_control;
    if strnlen((*mc).hdr.name.as_ptr(), SNDRV_CTL_ELEM_ID_NAME_MAXLEN) == SNDRV_CTL_ELEM_ID_NAME_MAXLEN { return -EINVAL; }
    let sm = devm_kzalloc((*tplg).dev, size_of::<soc_mixer_control>(), GFP_KERNEL) as *mut soc_mixer_control;
    if sm.is_null() { return -ENOMEM; }
    (*tplg).pos = (*tplg).pos.add(size_of::<snd_soc_tplg_mixer_control>() + le32_to_cpu((*mc).priv_.size) as usize);
    dev_dbg((*tplg).dev, b"ASoC: adding mixer kcontrol %s with access 0x%x\n\0".as_ptr() as *const c_char, (*mc).hdr.name.as_ptr(), le32_to_cpu((*mc).hdr.access));
    (*kc).name = devm_kstrdup((*tplg).dev, (*mc).hdr.name.as_ptr(), GFP_KERNEL);
    if (*kc).name.is_null() { return -ENOMEM; }
    (*kc).private_value = sm as c_long;
    (*kc).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*kc).access = le32_to_cpu((*mc).hdr.access);
    (*sm).reg = tplg_chan_get_reg(tplg, (*mc).channel.as_mut_ptr(), SNDRV_CHMAP_FL);
    (*sm).rreg = tplg_chan_get_reg(tplg, (*mc).channel.as_mut_ptr(), SNDRV_CHMAP_FR);
    (*sm).shift = tplg_chan_get_shift(tplg, (*mc).channel.as_mut_ptr(), SNDRV_CHMAP_FL);
    (*sm).rshift = tplg_chan_get_shift(tplg, (*mc).channel.as_mut_ptr(), SNDRV_CHMAP_FR);
    (*sm).max = le32_to_cpu((*mc).max);
    (*sm).min = le32_to_cpu((*mc).min);
    (*sm).invert = le32_to_cpu((*mc).invert);
    (*sm).platform_max = le32_to_cpu((*mc).platform_max);
    (*sm).num_channels = le32_to_cpu((*mc).num_channels);
    let mut err = soc_tplg_kcontrol_bind_io(&mut (*mc).hdr, kc, tplg);
    if err != 0 { soc_control_err(tplg, &mut (*mc).hdr, (*mc).hdr.name.as_ptr()); return err; }
    err = soc_tplg_create_tlv(tplg, kc, &mut (*mc).hdr);
    if err < 0 {
        dev_err((*tplg).dev, b"ASoC: failed to create TLV %s\n\0".as_ptr() as *const c_char, (*mc).hdr.name.as_ptr());
        return err;
    }
    soc_tplg_control_load(tplg, kc, &mut (*mc).hdr)
}

unsafe fn soc_tplg_denum_create_texts(tplg: *mut soc_tplg, se: *mut soc_enum, ec: *mut snd_soc_tplg_enum_control) -> c_int {
    if le32_to_cpu((*ec).items) as usize > (*ec).texts.len() { return -EINVAL; }
    (*se).dobj.u.control.dtexts = devm_kcalloc((*tplg).dev, le32_to_cpu((*ec).items) as usize, size_of::<*mut c_char>(), GFP_KERNEL) as *mut *mut c_char;
    if (*se).dobj.u.control.dtexts.is_null() { return -ENOMEM; }
    for i in 0..le32_to_cpu((*ec).items) as usize {
        if strnlen((*ec).texts[i].as_ptr(), SNDRV_CTL_ELEM_ID_NAME_MAXLEN) == SNDRV_CTL_ELEM_ID_NAME_MAXLEN { return -EINVAL; }
        *(*se).dobj.u.control.dtexts.add(i) = devm_kstrdup((*tplg).dev, (*ec).texts[i].as_ptr(), GFP_KERNEL);
        if (*(*se).dobj.u.control.dtexts.add(i)).is_null() { return -ENOMEM; }
    }
    (*se).items = le32_to_cpu((*ec).items);
    (*se).texts = (*se).dobj.u.control.dtexts as *const *const c_char;
    0
}

unsafe fn soc_tplg_denum_create_values(tplg: *mut soc_tplg, se: *mut soc_enum, ec: *mut snd_soc_tplg_enum_control) -> c_int {
    if le32_to_cpu((*ec).items) as usize > SND_SOC_TPLG_NUM_TEXTS { return -EINVAL; }
    (*se).dobj.u.control.dvalues = devm_kcalloc((*tplg).dev, le32_to_cpu((*ec).items) as usize, size_of::<c_uint>(), GFP_KERNEL) as *mut c_uint;
    if (*se).dobj.u.control.dvalues.is_null() { return -ENOMEM; }
    for i in 0..le32_to_cpu((*ec).items) as usize {
        *(*se).dobj.u.control.dvalues.add(i) = le32_to_cpu((*ec).values[i]);
    }
    (*se).items = le32_to_cpu((*ec).items);
    (*se).values = (*se).dobj.u.control.dvalues as *const c_uint;
    0
}

unsafe fn soc_tplg_control_denum_create(tplg: *mut soc_tplg, kc: *mut snd_kcontrol_new) -> c_int {
    let ec = (*tplg).pos as *mut snd_soc_tplg_enum_control;
    if strnlen((*ec).hdr.name.as_ptr(), SNDRV_CTL_ELEM_ID_NAME_MAXLEN) == SNDRV_CTL_ELEM_ID_NAME_MAXLEN { return -EINVAL; }
    let se = devm_kzalloc((*tplg).dev, size_of::<soc_enum>(), GFP_KERNEL) as *mut soc_enum;
    if se.is_null() { return -ENOMEM; }
    (*tplg).pos = (*tplg).pos.add(size_of::<snd_soc_tplg_enum_control>() + le32_to_cpu((*ec).priv_.size) as usize);
    dev_dbg((*tplg).dev, b"ASoC: adding enum kcontrol %s size %u\n\0".as_ptr() as *const c_char, (*ec).hdr.name.as_ptr(), le32_to_cpu((*ec).items));
    (*kc).name = devm_kstrdup((*tplg).dev, (*ec).hdr.name.as_ptr(), GFP_KERNEL);
    if (*kc).name.is_null() { return -ENOMEM; }
    (*kc).private_value = se as c_long;
    (*kc).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*kc).access = le32_to_cpu((*ec).hdr.access);
    (*se).reg = tplg_chan_get_reg(tplg, (*ec).channel.as_mut_ptr(), SNDRV_CHMAP_FL);
    (*se).shift_l = tplg_chan_get_shift(tplg, (*ec).channel.as_mut_ptr(), SNDRV_CHMAP_FL);
    (*se).shift_r = tplg_chan_get_shift(tplg, (*ec).channel.as_mut_ptr(), SNDRV_CHMAP_FR);
    (*se).mask = le32_to_cpu((*ec).mask);
    match le32_to_cpu((*ec).hdr.ops.info) as c_int {
        x if x == SND_SOC_TPLG_CTL_ENUM_VALUE || x == SND_SOC_TPLG_DAPM_CTL_ENUM_VALUE => {
            let err = soc_tplg_denum_create_values(tplg, se, ec);
            if err < 0 { dev_err((*tplg).dev, b"ASoC: could not create values for %s\n\0".as_ptr() as *const c_char, (*ec).hdr.name.as_ptr()); return err; }
            let err = soc_tplg_denum_create_texts(tplg, se, ec);
            if err < 0 { dev_err((*tplg).dev, b"ASoC: could not create texts for %s\n\0".as_ptr() as *const c_char, (*ec).hdr.name.as_ptr()); return err; }
        }
        x if x == SND_SOC_TPLG_CTL_ENUM || x == SND_SOC_TPLG_DAPM_CTL_ENUM_DOUBLE || x == SND_SOC_TPLG_DAPM_CTL_ENUM_VIRT => {
            let err = soc_tplg_denum_create_texts(tplg, se, ec);
            if err < 0 { dev_err((*tplg).dev, b"ASoC: could not create texts for %s\n\0".as_ptr() as *const c_char, (*ec).hdr.name.as_ptr()); return err; }
        }
        _ => {
            dev_err((*tplg).dev, b"ASoC: invalid enum control type %u for %s\n\0".as_ptr() as *const c_char, le32_to_cpu((*ec).hdr.ops.info), (*ec).hdr.name.as_ptr());
            return -EINVAL;
        }
    }
    let err = soc_tplg_kcontrol_bind_io(&mut (*ec).hdr, kc, tplg);
    if err != 0 { soc_control_err(tplg, &mut (*ec).hdr, (*ec).hdr.name.as_ptr()); return err; }
    soc_tplg_control_load(tplg, kc, &mut (*ec).hdr)
}

unsafe fn soc_tplg_control_dbytes_create(tplg: *mut soc_tplg, kc: *mut snd_kcontrol_new) -> c_int {
    let be = (*tplg).pos as *mut snd_soc_tplg_bytes_control;
    if strnlen((*be).hdr.name.as_ptr(), SNDRV_CTL_ELEM_ID_NAME_MAXLEN) == SNDRV_CTL_ELEM_ID_NAME_MAXLEN { return -EINVAL; }
    let sbe = devm_kzalloc((*tplg).dev, size_of::<soc_bytes_ext>(), GFP_KERNEL) as *mut soc_bytes_ext;
    if sbe.is_null() { return -ENOMEM; }
    (*tplg).pos = (*tplg).pos.add(size_of::<snd_soc_tplg_bytes_control>() + le32_to_cpu((*be).priv_.size) as usize);
    dev_dbg((*tplg).dev, b"ASoC: adding bytes kcontrol %s with access 0x%x\n\0".as_ptr() as *const c_char, (*be).hdr.name.as_ptr(), le32_to_cpu((*be).hdr.access));
    (*kc).name = devm_kstrdup((*tplg).dev, (*be).hdr.name.as_ptr(), GFP_KERNEL);
    if (*kc).name.is_null() { return -ENOMEM; }
    (*kc).private_value = sbe as c_long;
    (*kc).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*kc).access = le32_to_cpu((*be).hdr.access);
    (*sbe).max = le32_to_cpu((*be).max);
    let err = soc_tplg_kcontrol_bind_io(&mut (*be).hdr, kc, tplg);
    if err != 0 { soc_control_err(tplg, &mut (*be).hdr, (*be).hdr.name.as_ptr()); return err; }
    soc_tplg_control_load(tplg, kc, &mut (*be).hdr)
}

unsafe fn soc_tplg_dbytes_create(tplg: *mut soc_tplg, size: size_t) -> c_int {
    let mut kc: snd_kcontrol_new = zeroed();
    if soc_tplg_check_elem_count(tplg, size_of::<snd_soc_tplg_bytes_control>(), 1, size, b"mixer bytes\0".as_ptr() as *const c_char) != 0 { return -EINVAL; }
    let ret = soc_tplg_control_dbytes_create(tplg, &mut kc);
    if ret != 0 { return ret; }
    let sbe = kc.private_value as *mut soc_bytes_ext;
    INIT_LIST_HEAD(&mut (*sbe).dobj.list);
    (*sbe).dobj.type_ = SND_SOC_DOBJ_BYTES;
    (*sbe).dobj.index = (*tplg).index;
    if !(*tplg).ops.is_null() { (*sbe).dobj.unload = (*(*tplg).ops).control_unload; }
    let ret = soc_tplg_add_kcontrol(tplg, &mut kc, &mut (*sbe).dobj.u.control.kcontrol);
    if ret < 0 { return ret; }
    list_add(&mut (*sbe).dobj.list, &mut (*(*tplg).comp).dobj_list);
    ret
}

unsafe fn soc_tplg_dmixer_create(tplg: *mut soc_tplg, size: size_t) -> c_int {
    let mut kc: snd_kcontrol_new = zeroed();
    if soc_tplg_check_elem_count(tplg, size_of::<snd_soc_tplg_mixer_control>(), 1, size, b"mixers\0".as_ptr() as *const c_char) != 0 { return -EINVAL; }
    let ret = soc_tplg_control_dmixer_create(tplg, &mut kc);
    if ret != 0 { return ret; }
    let sm = kc.private_value as *mut soc_mixer_control;
    INIT_LIST_HEAD(&mut (*sm).dobj.list);
    (*sm).dobj.type_ = SND_SOC_DOBJ_MIXER;
    (*sm).dobj.index = (*tplg).index;
    if !(*tplg).ops.is_null() { (*sm).dobj.unload = (*(*tplg).ops).control_unload; }
    let ret = soc_tplg_add_kcontrol(tplg, &mut kc, &mut (*sm).dobj.u.control.kcontrol);
    if ret < 0 { return ret; }
    list_add(&mut (*sm).dobj.list, &mut (*(*tplg).comp).dobj_list);
    ret
}

unsafe fn soc_tplg_denum_create(tplg: *mut soc_tplg, size: size_t) -> c_int {
    let mut kc: snd_kcontrol_new = zeroed();
    if soc_tplg_check_elem_count(tplg, size_of::<snd_soc_tplg_enum_control>(), 1, size, b"enums\0".as_ptr() as *const c_char) != 0 { return -EINVAL; }
    let ret = soc_tplg_control_denum_create(tplg, &mut kc);
    if ret != 0 { return ret; }
    let se = kc.private_value as *mut soc_enum;
    INIT_LIST_HEAD(&mut (*se).dobj.list);
    (*se).dobj.type_ = SND_SOC_DOBJ_ENUM;
    (*se).dobj.index = (*tplg).index;
    if !(*tplg).ops.is_null() { (*se).dobj.unload = (*(*tplg).ops).control_unload; }
    let ret = soc_tplg_add_kcontrol(tplg, &mut kc, &mut (*se).dobj.u.control.kcontrol);
    if ret < 0 { return ret; }
    list_add(&mut (*se).dobj.list, &mut (*(*tplg).comp).dobj_list);
    ret
}

unsafe fn soc_tplg_kcontrol_elems_load(tplg: *mut soc_tplg, hdr: *mut snd_soc_tplg_hdr) -> c_int {
    dev_dbg((*tplg).dev, b"ASoC: adding %u kcontrols at 0x%lx\n\0".as_ptr() as *const c_char, le32_to_cpu((*hdr).count), soc_tplg_get_offset(tplg));
    for i in 0..le32_to_cpu((*hdr).count) {
        let control_hdr = (*tplg).pos as *mut snd_soc_tplg_ctl_hdr;
        if le32_to_cpu((*control_hdr).size) as usize != size_of::<snd_soc_tplg_ctl_hdr>() {
            dev_err((*tplg).dev, b"ASoC: invalid control size\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        let ret = match le32_to_cpu((*control_hdr).type_) {
            SND_SOC_TPLG_TYPE_MIXER => soc_tplg_dmixer_create(tplg, le32_to_cpu((*hdr).payload_size) as usize),
            SND_SOC_TPLG_TYPE_ENUM => soc_tplg_denum_create(tplg, le32_to_cpu((*hdr).payload_size) as usize),
            SND_SOC_TPLG_TYPE_BYTES => soc_tplg_dbytes_create(tplg, le32_to_cpu((*hdr).payload_size) as usize),
            _ => -EINVAL,
        };
        if ret < 0 {
            dev_err((*tplg).dev, b"ASoC: invalid control type: %u, index: %d at 0x%lx\n\0".as_ptr() as *const c_char, le32_to_cpu((*control_hdr).type_), i as c_int, soc_tplg_get_offset(tplg));
            return ret;
        }
    }
    0
}

unsafe fn soc_tplg_add_route(tplg: *mut soc_tplg, route: *mut snd_soc_dapm_route) -> c_int {
    if !(*tplg).ops.is_null() && (*(*tplg).ops).dapm_route_load.is_some() {
        return ((*(*tplg).ops).dapm_route_load.unwrap())((*tplg).comp, (*tplg).index, route);
    }
    0
}

unsafe fn soc_tplg_dapm_graph_elems_load(tplg: *mut soc_tplg, hdr: *mut snd_soc_tplg_hdr) -> c_int {
    let dapm = snd_soc_component_to_dapm((*tplg).comp);
    let maxlen = SNDRV_CTL_ELEM_ID_NAME_MAXLEN;
    let count = le32_to_cpu((*hdr).count) as c_int;
    let mut ret = 0;
    if soc_tplg_check_elem_count(tplg, size_of::<snd_soc_tplg_dapm_graph_elem>(), count as c_uint, le32_to_cpu((*hdr).payload_size) as usize, b"graph\0".as_ptr() as *const c_char) != 0 { return -EINVAL; }
    dev_dbg((*tplg).dev, b"ASoC: adding %d DAPM routes for index %u\n\0".as_ptr() as *const c_char, count, le32_to_cpu((*hdr).index));
    for _i in 0..count {
        let route = devm_kzalloc((*tplg).dev, size_of::<snd_soc_dapm_route>(), GFP_KERNEL) as *mut snd_soc_dapm_route;
        if route.is_null() { return -ENOMEM; }
        let elem = (*tplg).pos as *mut snd_soc_tplg_dapm_graph_elem;
        (*tplg).pos = (*tplg).pos.add(size_of::<snd_soc_tplg_dapm_graph_elem>());
        if strnlen((*elem).source.as_ptr(), maxlen) == maxlen || strnlen((*elem).sink.as_ptr(), maxlen) == maxlen || strnlen((*elem).control.as_ptr(), maxlen) == maxlen {
            ret = -EINVAL; break;
        }
        (*route).source = devm_kstrdup((*tplg).dev, (*elem).source.as_ptr(), GFP_KERNEL);
        (*route).sink = devm_kstrdup((*tplg).dev, (*elem).sink.as_ptr(), GFP_KERNEL);
        if (*route).source.is_null() || (*route).sink.is_null() { ret = -ENOMEM; break; }
        if strnlen((*elem).control.as_ptr(), maxlen) != 0 {
            (*route).control = devm_kstrdup((*tplg).dev, (*elem).control.as_ptr(), GFP_KERNEL);
            if (*route).control.is_null() { ret = -ENOMEM; break; }
        }
        (*route).dobj.type_ = SND_SOC_DOBJ_GRAPH;
        if !(*tplg).ops.is_null() { (*route).dobj.unload = (*(*tplg).ops).dapm_route_unload; }
        (*route).dobj.index = (*tplg).index;
        list_add(&mut (*route).dobj.list, &mut (*(*tplg).comp).dobj_list);
        ret = soc_tplg_add_route(tplg, route);
        if ret < 0 { dev_err((*tplg).dev, b"ASoC: topology: add_route failed: %d\n\0".as_ptr() as *const c_char, ret); break; }
        ret = snd_soc_dapm_add_routes(dapm, route, 1);
        if ret != 0 { break; }
    }
    ret
}

unsafe fn soc_tplg_dapm_widget_create(tplg: *mut soc_tplg, w: *mut snd_soc_tplg_dapm_widget) -> c_int {
    let dapm = snd_soc_component_to_dapm((*tplg).comp);
    let card = (*(*tplg).comp).card;
    let mut template: snd_soc_dapm_widget = zeroed();
    let mut kcontrol_type: *mut c_uint = null_mut();
    let mut mixer_count = 0;
    let mut bytes_count = 0;
    let mut enum_count = 0;
    let mut ret: c_int;
    if strnlen((*w).name.as_ptr(), SNDRV_CTL_ELEM_ID_NAME_MAXLEN) == SNDRV_CTL_ELEM_ID_NAME_MAXLEN { return -EINVAL; }
    if strnlen((*w).sname.as_ptr(), SNDRV_CTL_ELEM_ID_NAME_MAXLEN) == SNDRV_CTL_ELEM_ID_NAME_MAXLEN { return -EINVAL; }
    dev_dbg((*tplg).dev, b"ASoC: creating DAPM widget %s id %u\n\0".as_ptr() as *const c_char, (*w).name.as_ptr(), le32_to_cpu((*w).id));
    template.id = get_widget_id(le32_to_cpu((*w).id) as c_int);
    if template.id < 0 { return template.id; }
    template.name = kstrdup((*w).name.as_ptr(), GFP_KERNEL);
    if template.name.is_null() { return -ENOMEM; }
    template.sname = kstrdup((*w).sname.as_ptr(), GFP_KERNEL);
    if template.sname.is_null() { ret = -ENOMEM; kfree(template.name as *mut c_void); return ret; }
    template.reg = le32_to_cpu((*w).reg) as c_int;
    template.shift = le32_to_cpu((*w).shift) as c_int;
    template.mask = le32_to_cpu((*w).mask);
    template.subseq = le32_to_cpu((*w).subseq);
    template.on_val = if (*w).invert != 0 { 0 } else { 1 };
    template.off_val = if (*w).invert != 0 { 1 } else { 0 };
    template.ignore_suspend = le32_to_cpu((*w).ignore_suspend);
    template.event_flags = le16_to_cpu((*w).event_flags);
    template.dobj.index = (*tplg).index;
    (*tplg).pos = (*tplg).pos.add(size_of::<snd_soc_tplg_dapm_widget>() + le32_to_cpu((*w).priv_.size) as usize);
    if (*w).num_kcontrols == 0 {
        template.num_kcontrols = 0;
    } else {
        template.num_kcontrols = le32_to_cpu((*w).num_kcontrols) as c_int;
        let kc = devm_kcalloc((*tplg).dev, le32_to_cpu((*w).num_kcontrols) as usize, size_of::<snd_kcontrol_new>(), GFP_KERNEL) as *mut snd_kcontrol_new;
        if kc.is_null() { kfree(template.sname as *mut c_void); kfree(template.name as *mut c_void); return -ENOMEM; }
        kcontrol_type = devm_kcalloc((*tplg).dev, le32_to_cpu((*w).num_kcontrols) as usize, size_of::<c_uint>(), GFP_KERNEL) as *mut c_uint;
        if kcontrol_type.is_null() { kfree(template.sname as *mut c_void); kfree(template.name as *mut c_void); return -ENOMEM; }
        for i in 0..le32_to_cpu((*w).num_kcontrols) as usize {
            let control_hdr = (*tplg).pos as *mut snd_soc_tplg_ctl_hdr;
            match le32_to_cpu((*control_hdr).type_) {
                SND_SOC_TPLG_TYPE_MIXER => { (*kc.add(i)).index = mixer_count; *kcontrol_type.add(i) = SND_SOC_TPLG_TYPE_MIXER; mixer_count += 1; ret = soc_tplg_control_dmixer_create(tplg, kc.add(i)); }
                SND_SOC_TPLG_TYPE_ENUM => { (*kc.add(i)).index = enum_count; *kcontrol_type.add(i) = SND_SOC_TPLG_TYPE_ENUM; enum_count += 1; ret = soc_tplg_control_denum_create(tplg, kc.add(i)); }
                SND_SOC_TPLG_TYPE_BYTES => { (*kc.add(i)).index = bytes_count; *kcontrol_type.add(i) = SND_SOC_TPLG_TYPE_BYTES; bytes_count += 1; ret = soc_tplg_control_dbytes_create(tplg, kc.add(i)); }
                _ => {
                    dev_err((*tplg).dev, b"ASoC: invalid widget control type %u:%u:%u\n\0".as_ptr() as *const c_char, le32_to_cpu((*control_hdr).ops.get), le32_to_cpu((*control_hdr).ops.put), le32_to_cpu((*control_hdr).ops.info));
                    ret = -EINVAL;
                }
            }
            if ret < 0 { kfree(template.sname as *mut c_void); kfree(template.name as *mut c_void); return ret; }
        }
        template.kcontrol_news = kc;
        dev_dbg((*tplg).dev, b"ASoC: template %s with %d/%d/%d (mixer/enum/bytes) control\n\0".as_ptr() as *const c_char, (*w).name.as_ptr(), mixer_count, enum_count, bytes_count);
    }
    ret = soc_tplg_widget_load(tplg, &mut template, w);
    if ret < 0 { kfree(template.sname as *mut c_void); kfree(template.name as *mut c_void); return ret; }
    let widget = if snd_soc_card_is_instantiated(card) { snd_soc_dapm_new_control(dapm, &template) } else { snd_soc_dapm_new_control_unlocked(dapm, &template) };
    if IS_ERR(widget) { ret = PTR_ERR(widget); kfree(template.sname as *mut c_void); kfree(template.name as *mut c_void); return ret; }
    (*widget).dobj.type_ = SND_SOC_DOBJ_WIDGET;
    (*widget).dobj.u.widget.kcontrol_type = kcontrol_type;
    if !(*tplg).ops.is_null() { (*widget).dobj.unload = (*(*tplg).ops).widget_unload; }
    (*widget).dobj.index = (*tplg).index;
    list_add(&mut (*widget).dobj.list, &mut (*(*tplg).comp).dobj_list);
    ret = soc_tplg_widget_ready(tplg, widget, w);
    if ret < 0 {
        soc_tplg_remove_widget(snd_soc_dapm_to_component((*widget).dapm), &mut (*widget).dobj, SOC_TPLG_PASS_WIDGET as c_int);
        snd_soc_dapm_free_widget(widget);
        kfree(template.sname as *mut c_void);
        kfree(template.name as *mut c_void);
        return ret;
    }
    kfree(template.sname as *mut c_void);
    kfree(template.name as *mut c_void);
    0
}

unsafe fn soc_tplg_dapm_widget_elems_load(tplg: *mut soc_tplg, hdr: *mut snd_soc_tplg_hdr) -> c_int {
    let count = le32_to_cpu((*hdr).count) as c_int;
    dev_dbg((*tplg).dev, b"ASoC: adding %d DAPM widgets\n\0".as_ptr() as *const c_char, count);
    for _ in 0..count {
        let widget = (*tplg).pos as *mut snd_soc_tplg_dapm_widget;
        if soc_tplg_get_offset(tplg) as usize + size_of::<snd_soc_tplg_dapm_widget>() >= (*(*tplg).fw).size {
            dev_err((*tplg).dev, b"ASoC: invalid widget data size\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        if le32_to_cpu((*widget).size) as usize != size_of::<snd_soc_tplg_dapm_widget>() {
            dev_err((*tplg).dev, b"ASoC: invalid widget size\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        if soc_tplg_get_offset(tplg) as usize + le32_to_cpu((*widget).priv_.size) as usize >= (*(*tplg).fw).size {
            dev_err((*tplg).dev, b"ASoC: invalid widget private data size\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        let ret = soc_tplg_dapm_widget_create(tplg, widget);
        if ret < 0 {
            dev_err((*tplg).dev, b"ASoC: failed to load widget %s\n\0".as_ptr() as *const c_char, (*widget).name.as_ptr());
            return ret;
        }
    }
    0
}

unsafe fn soc_tplg_dapm_complete(tplg: *mut soc_tplg) -> c_int {
    let card = (*(*tplg).comp).card;
    if !snd_soc_card_is_instantiated(card) {
        dev_warn((*tplg).dev, b"ASoC: Parent card not yet available, widget card binding deferred\n\0".as_ptr() as *const c_char);
        return 0;
    }
    let ret = snd_soc_dapm_new_widgets(card);
    if ret < 0 { dev_err((*tplg).dev, b"ASoC: failed to create new widgets %d\n\0".as_ptr() as *const c_char, ret); }
    ret
}

unsafe fn soc_tplg_check_name(name: *const c_char) -> c_int {
    if strnlen(name, SNDRV_CTL_ELEM_ID_NAME_MAXLEN) == SNDRV_CTL_ELEM_ID_NAME_MAXLEN { return -EINVAL; }
    0
}

unsafe fn set_stream_info(tplg: *mut soc_tplg, stream: *mut snd_soc_pcm_stream, caps: *mut snd_soc_tplg_stream_caps) -> c_int {
    let ret = soc_tplg_check_name((*caps).name.as_ptr());
    if ret != 0 { return ret; }
    (*stream).stream_name = devm_kstrdup((*tplg).dev, (*caps).name.as_ptr(), GFP_KERNEL);
    if (*stream).stream_name.is_null() { return -ENOMEM; }
    (*stream).channels_min = le32_to_cpu((*caps).channels_min);
    (*stream).channels_max = le32_to_cpu((*caps).channels_max);
    (*stream).rates = le32_to_cpu((*caps).rates);
    (*stream).rate_min = le32_to_cpu((*caps).rate_min);
    (*stream).rate_max = le32_to_cpu((*caps).rate_max);
    (*stream).formats = le64_to_cpu((*caps).formats);
    (*stream).sig_bits = le32_to_cpu((*caps).sig_bits);
    0
}

unsafe fn set_dai_flags(dai_drv: *mut snd_soc_dai_driver, flag_mask: c_uint, flags: c_uint) {
    if (flag_mask & SND_SOC_TPLG_DAI_FLGBIT_SYMMETRIC_RATES) != 0 {
        (*dai_drv).symmetric_rate = if (flags & SND_SOC_TPLG_DAI_FLGBIT_SYMMETRIC_RATES) != 0 { 1 } else { 0 };
    }
    if (flag_mask & SND_SOC_TPLG_DAI_FLGBIT_SYMMETRIC_CHANNELS) != 0 {
        (*dai_drv).symmetric_channels = if (flags & SND_SOC_TPLG_DAI_FLGBIT_SYMMETRIC_CHANNELS) != 0 { 1 } else { 0 };
    }
    if (flag_mask & SND_SOC_TPLG_DAI_FLGBIT_SYMMETRIC_SAMPLEBITS) != 0 {
        (*dai_drv).symmetric_sample_bits = if (flags & SND_SOC_TPLG_DAI_FLGBIT_SYMMETRIC_SAMPLEBITS) != 0 { 1 } else { 0 };
    }
}

static tplg_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops { compress_new: unsafe { snd_soc_new_compress } };

unsafe fn soc_tplg_dai_create(tplg: *mut soc_tplg, pcm: *mut snd_soc_tplg_pcm) -> c_int {
    let dai_drv = devm_kzalloc((*tplg).dev, size_of::<snd_soc_dai_driver>(), GFP_KERNEL) as *mut snd_soc_dai_driver;
    if dai_drv.is_null() { return -ENOMEM; }
    let mut ret = soc_tplg_check_name((*pcm).dai_name.as_ptr());
    if ret != 0 { return ret; }
    if (*pcm).dai_name[0] != 0 {
        (*dai_drv).name = devm_kstrdup((*tplg).dev, (*pcm).dai_name.as_ptr(), GFP_KERNEL);
        if (*dai_drv).name.is_null() { return -ENOMEM; }
    }
    (*dai_drv).id = le32_to_cpu((*pcm).dai_id) as c_int;
    if (*pcm).playback != 0 {
        ret = set_stream_info(tplg, &mut (*dai_drv).playback, &mut (*pcm).caps[SND_SOC_TPLG_STREAM_PLAYBACK]);
        if ret < 0 { return ret; }
    }
    if (*pcm).capture != 0 {
        ret = set_stream_info(tplg, &mut (*dai_drv).capture, &mut (*pcm).caps[SND_SOC_TPLG_STREAM_CAPTURE]);
        if ret < 0 { return ret; }
    }
    if (*pcm).compress != 0 { (*dai_drv).ops = &tplg_dai_ops; }
    ret = soc_tplg_dai_load(tplg, dai_drv, pcm, null_mut());
    if ret < 0 { dev_err((*tplg).dev, b"ASoC: DAI loading failed\n\0".as_ptr() as *const c_char); return ret; }
    (*dai_drv).dobj.index = (*tplg).index;
    (*dai_drv).dobj.type_ = SND_SOC_DOBJ_PCM;
    if !(*tplg).ops.is_null() { (*dai_drv).dobj.unload = (*(*tplg).ops).dai_unload; }
    list_add(&mut (*dai_drv).dobj.list, &mut (*(*tplg).comp).dobj_list);
    let dai = snd_soc_register_dai((*tplg).comp, dai_drv, false);
    if dai.is_null() { return -ENOMEM; }
    ret = snd_soc_dapm_new_dai_widgets(snd_soc_component_to_dapm((*tplg).comp), dai);
    if ret != 0 {
        dev_err((*dai).dev, b"Failed to create DAI widgets %d\n\0".as_ptr() as *const c_char, ret);
        snd_soc_unregister_dai(dai);
        return ret;
    }
    0
}

unsafe fn set_link_flags(link: *mut snd_soc_dai_link, flag_mask: c_uint, flags: c_uint) {
    if (flag_mask & SND_SOC_TPLG_LNK_FLGBIT_SYMMETRIC_RATES) != 0 { (*link).symmetric_rate = if (flags & SND_SOC_TPLG_LNK_FLGBIT_SYMMETRIC_RATES) != 0 { 1 } else { 0 }; }
    if (flag_mask & SND_SOC_TPLG_LNK_FLGBIT_SYMMETRIC_CHANNELS) != 0 { (*link).symmetric_channels = if (flags & SND_SOC_TPLG_LNK_FLGBIT_SYMMETRIC_CHANNELS) != 0 { 1 } else { 0 }; }
    if (flag_mask & SND_SOC_TPLG_LNK_FLGBIT_SYMMETRIC_SAMPLEBITS) != 0 { (*link).symmetric_sample_bits = if (flags & SND_SOC_TPLG_LNK_FLGBIT_SYMMETRIC_SAMPLEBITS) != 0 { 1 } else { 0 }; }
    if (flag_mask & SND_SOC_TPLG_LNK_FLGBIT_VOICE_WAKEUP) != 0 { (*link).ignore_suspend = if (flags & SND_SOC_TPLG_LNK_FLGBIT_VOICE_WAKEUP) != 0 { 1 } else { 0 }; }
}

unsafe fn soc_tplg_fe_link_create(tplg: *mut soc_tplg, pcm: *mut snd_soc_tplg_pcm) -> c_int {
    let link = devm_kzalloc((*tplg).dev, size_of::<snd_soc_dai_link>() + 3 * size_of::<snd_soc_dai_link_component>(), GFP_KERNEL) as *mut snd_soc_dai_link;
    if link.is_null() { return -ENOMEM; }
    let dlc = link.add(1) as *mut snd_soc_dai_link_component;
    (*link).cpus = dlc.add(0);
    (*link).num_cpus = 1;
    (*link).dobj.index = (*tplg).index;
    (*link).dobj.type_ = SND_SOC_DOBJ_DAI_LINK;
    if !(*tplg).ops.is_null() { (*link).dobj.unload = (*(*tplg).ops).link_unload; }
    let mut ret = soc_tplg_check_name((*pcm).pcm_name.as_ptr());
    if ret != 0 { return ret; }
    if (*pcm).pcm_name[0] != 0 {
        (*link).name = devm_kstrdup((*tplg).dev, (*pcm).pcm_name.as_ptr(), GFP_KERNEL);
        (*link).stream_name = devm_kstrdup((*tplg).dev, (*pcm).pcm_name.as_ptr(), GFP_KERNEL);
        if (*link).name.is_null() || (*link).stream_name.is_null() { return -ENOMEM; }
    }
    (*link).id = le32_to_cpu((*pcm).pcm_id) as c_int;
    ret = soc_tplg_check_name((*pcm).dai_name.as_ptr());
    if ret != 0 { return ret; }
    if (*pcm).dai_name[0] != 0 {
        (*(*link).cpus).dai_name = devm_kstrdup((*tplg).dev, (*pcm).dai_name.as_ptr(), GFP_KERNEL);
        if (*(*link).cpus).dai_name.is_null() { return -ENOMEM; }
    }
    (*link).codecs = dlc.add(1);
    (*(*link).codecs).name = b"snd-soc-dummy\0".as_ptr() as *const c_char;
    (*(*link).codecs).dai_name = b"snd-soc-dummy-dai\0".as_ptr() as *const c_char;
    (*link).num_codecs = 1;
    (*link).platforms = dlc.add(2);
    (*(*link).platforms).name = b"snd-soc-dummy\0".as_ptr() as *const c_char;
    (*link).num_platforms = 1;
    (*link).dynamic = 1;
    (*link).ignore_pmdown_time = 1;
    (*link).playback_only = if le32_to_cpu((*pcm).playback) != 0 && le32_to_cpu((*pcm).capture) == 0 { 1 } else { 0 };
    (*link).capture_only = if le32_to_cpu((*pcm).playback) == 0 && le32_to_cpu((*pcm).capture) != 0 { 1 } else { 0 };
    if (*pcm).flag_mask != 0 { set_link_flags(link, le32_to_cpu((*pcm).flag_mask), le32_to_cpu((*pcm).flags)); }
    ret = soc_tplg_dai_link_load(tplg, link, null_mut());
    if ret < 0 { dev_err((*tplg).dev, b"ASoC: FE link loading failed\n\0".as_ptr() as *const c_char); return ret; }
    ret = snd_soc_add_pcm_runtimes((*(*tplg).comp).card, link, 1);
    if ret < 0 {
        if ret != -EPROBE_DEFER { dev_err((*tplg).dev, b"ASoC: adding FE link failed\n\0".as_ptr() as *const c_char); }
        return ret;
    }
    list_add(&mut (*link).dobj.list, &mut (*(*tplg).comp).dobj_list);
    0
}

unsafe fn soc_tplg_pcm_create(tplg: *mut soc_tplg, pcm: *mut snd_soc_tplg_pcm) -> c_int {
    let ret = soc_tplg_dai_create(tplg, pcm);
    if ret < 0 { return ret; }
    soc_tplg_fe_link_create(tplg, pcm)
}

unsafe fn soc_tplg_pcm_elems_load(tplg: *mut soc_tplg, hdr: *mut snd_soc_tplg_hdr) -> c_int {
    let count = le32_to_cpu((*hdr).count) as c_int;
    let mut pcm = (*tplg).pos as *mut snd_soc_tplg_pcm;
    let mut size = le32_to_cpu((*pcm).size) as c_int;
    if size as usize > size_of::<snd_soc_tplg_pcm>() {
        dev_err((*tplg).dev, b"ASoC: invalid size %d for PCM elems\n\0".as_ptr() as *const c_char, size);
        return -EINVAL;
    }
    if soc_tplg_check_elem_count(tplg, size as usize, count as c_uint, le32_to_cpu((*hdr).payload_size) as usize, b"PCM DAI\0".as_ptr() as *const c_char) != 0 { return -EINVAL; }
    for _ in 0..count {
        pcm = (*tplg).pos as *mut snd_soc_tplg_pcm;
        size = le32_to_cpu((*pcm).size) as c_int;
        if size as usize != size_of::<snd_soc_tplg_pcm>() { return -EINVAL; }
        let ret = soc_tplg_pcm_create(tplg, pcm);
        if ret < 0 { return ret; }
        (*tplg).pos = (*tplg).pos.add(size as usize + le32_to_cpu((*pcm).priv_.size) as usize);
    }
    dev_dbg((*tplg).dev, b"ASoC: adding %d PCM DAIs\n\0".as_ptr() as *const c_char, count);
    0
}

unsafe fn set_link_hw_format(link: *mut snd_soc_dai_link, cfg: *mut snd_soc_tplg_link_config) {
    for i in 0..le32_to_cpu((*cfg).num_hw_configs) as usize {
        let hw_config = &mut (*cfg).hw_config[i] as *mut snd_soc_tplg_hw_config;
        if (*hw_config).id != (*cfg).default_hw_config_id { continue; }
        (*link).dai_fmt = le32_to_cpu((*hw_config).fmt) & SND_SOC_DAIFMT_FORMAT_MASK;
        match (*hw_config).clock_gated {
            SND_SOC_TPLG_DAI_CLK_GATE_GATED => (*link).dai_fmt |= SND_SOC_DAIFMT_GATED,
            SND_SOC_TPLG_DAI_CLK_GATE_CONT => (*link).dai_fmt |= SND_SOC_DAIFMT_CONT,
            _ => {}
        }
        let invert_bclk = (*hw_config).invert_bclk;
        let invert_fsync = (*hw_config).invert_fsync;
        if invert_bclk == 0 && invert_fsync == 0 { (*link).dai_fmt |= SND_SOC_DAIFMT_NB_NF; }
        else if invert_bclk == 0 && invert_fsync != 0 { (*link).dai_fmt |= SND_SOC_DAIFMT_NB_IF; }
        else if invert_bclk != 0 && invert_fsync == 0 { (*link).dai_fmt |= SND_SOC_DAIFMT_IB_NF; }
        else { (*link).dai_fmt |= SND_SOC_DAIFMT_IB_IF; }
        let bclk_provider = (*hw_config).bclk_provider == SND_SOC_TPLG_BCLK_CP;
        let fsync_provider = (*hw_config).fsync_provider == SND_SOC_TPLG_FSYNC_CP;
        if bclk_provider && fsync_provider { (*link).dai_fmt |= SND_SOC_DAIFMT_CBP_CFP; }
        else if !bclk_provider && fsync_provider { (*link).dai_fmt |= SND_SOC_DAIFMT_CBC_CFP; }
        else if bclk_provider && !fsync_provider { (*link).dai_fmt |= SND_SOC_DAIFMT_CBP_CFC; }
        else { (*link).dai_fmt |= SND_SOC_DAIFMT_CBC_CFC; }
    }
}

unsafe fn snd_soc_find_dai_link(_card: *mut snd_soc_card, _id: c_int, _name: *const c_char, _stream_name: *const c_char) -> *mut snd_soc_dai_link {
    /* for_each_card_rtds(card, rtd) is provided by the surrounding kernel list machinery. */
    null_mut()
}

unsafe fn soc_tplg_link_config(tplg: *mut soc_tplg, cfg: *mut snd_soc_tplg_link_config) -> c_int {
    let len = strnlen((*cfg).name.as_ptr(), SNDRV_CTL_ELEM_ID_NAME_MAXLEN);
    let name = if len == SNDRV_CTL_ELEM_ID_NAME_MAXLEN { return -EINVAL; } else if len != 0 { (*cfg).name.as_ptr() } else { null() };
    let len2 = strnlen((*cfg).stream_name.as_ptr(), SNDRV_CTL_ELEM_ID_NAME_MAXLEN);
    let stream_name = if len2 == SNDRV_CTL_ELEM_ID_NAME_MAXLEN { return -EINVAL; } else if len2 != 0 { (*cfg).stream_name.as_ptr() } else { null() };
    let link = snd_soc_find_dai_link((*(*tplg).comp).card, le32_to_cpu((*cfg).id) as c_int, name, stream_name);
    if link.is_null() {
        dev_err((*tplg).dev, b"ASoC: physical link %s (id %u) not exist\n\0".as_ptr() as *const c_char, name, le32_to_cpu((*cfg).id));
        return -EINVAL;
    }
    if (*cfg).num_hw_configs != 0 { set_link_hw_format(link, cfg); }
    if (*cfg).flag_mask != 0 { set_link_flags(link, le32_to_cpu((*cfg).flag_mask), le32_to_cpu((*cfg).flags)); }
    let ret = soc_tplg_dai_link_load(tplg, link, cfg);
    if ret < 0 {
        dev_err((*tplg).dev, b"ASoC: physical link loading failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    (*link).dobj.index = (*tplg).index;
    (*link).dobj.type_ = SND_SOC_DOBJ_BACKEND_LINK;
    if !(*tplg).ops.is_null() { (*link).dobj.unload = (*(*tplg).ops).link_unload; }
    list_add(&mut (*link).dobj.list, &mut (*(*tplg).comp).dobj_list);
    0
}

unsafe fn soc_tplg_link_elems_load(tplg: *mut soc_tplg, hdr: *mut snd_soc_tplg_hdr) -> c_int {
    let count = le32_to_cpu((*hdr).count) as c_int;
    let mut link = (*tplg).pos as *mut snd_soc_tplg_link_config;
    let mut size = le32_to_cpu((*link).size) as c_int;
    if size as usize > size_of::<snd_soc_tplg_link_config>() {
        dev_err((*tplg).dev, b"ASoC: invalid size %d for physical link elems\n\0".as_ptr() as *const c_char, size);
        return -EINVAL;
    }
    if soc_tplg_check_elem_count(tplg, size as usize, count as c_uint, le32_to_cpu((*hdr).payload_size) as usize, b"physical link config\0".as_ptr() as *const c_char) != 0 { return -EINVAL; }
    for _ in 0..count {
        link = (*tplg).pos as *mut snd_soc_tplg_link_config;
        size = le32_to_cpu((*link).size) as c_int;
        if size as usize != size_of::<snd_soc_tplg_link_config>() { return -EINVAL; }
        let ret = soc_tplg_link_config(tplg, link);
        if ret < 0 { return ret; }
        (*tplg).pos = (*tplg).pos.add(size as usize + le32_to_cpu((*link).priv_.size) as usize);
    }
    0
}

unsafe fn soc_tplg_dai_config(tplg: *mut soc_tplg, d: *mut snd_soc_tplg_dai) -> c_int {
    let mut dai_component: snd_soc_dai_link_component = zeroed();
    let ret = soc_tplg_check_name((*d).dai_name.as_ptr());
    if ret != 0 { return ret; }
    dai_component.dai_name = (*d).dai_name.as_ptr();
    let dai = snd_soc_find_dai(&mut dai_component);
    if dai.is_null() {
        dev_err((*tplg).dev, b"ASoC: physical DAI %s not registered\n\0".as_ptr() as *const c_char, (*d).dai_name.as_ptr());
        return -EINVAL;
    }
    if le32_to_cpu((*d).dai_id) as c_int != (*dai).id {
        dev_err((*tplg).dev, b"ASoC: physical DAI %s id mismatch\n\0".as_ptr() as *const c_char, (*d).dai_name.as_ptr());
        return -EINVAL;
    }
    let dai_drv = (*dai).driver;
    if dai_drv.is_null() { return -EINVAL; }
    if (*d).playback != 0 {
        let r = set_stream_info(tplg, &mut (*dai_drv).playback, &mut (*d).caps[SND_SOC_TPLG_STREAM_PLAYBACK]);
        if r < 0 { return r; }
    }
    if (*d).capture != 0 {
        let r = set_stream_info(tplg, &mut (*dai_drv).capture, &mut (*d).caps[SND_SOC_TPLG_STREAM_CAPTURE]);
        if r < 0 { return r; }
    }
    if (*d).flag_mask != 0 { set_dai_flags(dai_drv, le32_to_cpu((*d).flag_mask), le32_to_cpu((*d).flags)); }
    let ret = soc_tplg_dai_load(tplg, dai_drv, null_mut(), dai);
    if ret < 0 {
        dev_err((*tplg).dev, b"ASoC: DAI loading failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    0
}

unsafe fn soc_tplg_dai_elems_load(tplg: *mut soc_tplg, hdr: *mut snd_soc_tplg_hdr) -> c_int {
    let count = le32_to_cpu((*hdr).count) as c_int;
    for _ in 0..count {
        let dai = (*tplg).pos as *mut snd_soc_tplg_dai;
        if le32_to_cpu((*dai).size) as usize != size_of::<snd_soc_tplg_dai>() {
            dev_err((*tplg).dev, b"ASoC: invalid physical DAI size\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        let ret = soc_tplg_dai_config(tplg, dai);
        if ret < 0 {
            dev_err((*tplg).dev, b"ASoC: failed to configure DAI\n\0".as_ptr() as *const c_char);
            return ret;
        }
        (*tplg).pos = (*tplg).pos.add(size_of::<snd_soc_tplg_dai>() + le32_to_cpu((*dai).priv_.size) as usize);
    }
    dev_dbg((*tplg).dev, b"ASoC: Configure %d BE DAIs\n\0".as_ptr() as *const c_char, count);
    0
}

unsafe fn soc_tplg_manifest_load(tplg: *mut soc_tplg, _hdr: *mut snd_soc_tplg_hdr) -> c_int {
    let manifest = (*tplg).pos as *mut snd_soc_tplg_manifest;
    if le32_to_cpu((*manifest).size) as usize != size_of::<snd_soc_tplg_manifest>() { return -EINVAL; }
    if !(*tplg).ops.is_null() && (*(*tplg).ops).manifest.is_some() {
        return ((*(*tplg).ops).manifest.unwrap())((*tplg).comp, (*tplg).index, manifest);
    }
    0
}

unsafe fn soc_tplg_valid_header(tplg: *mut soc_tplg, hdr: *mut snd_soc_tplg_hdr) -> c_int {
    if le32_to_cpu((*hdr).size) as usize != size_of::<snd_soc_tplg_hdr>() {
        dev_err((*tplg).dev, b"ASoC: invalid header size for type %u at offset 0x%lx size 0x%zx.\n\0".as_ptr() as *const c_char, le32_to_cpu((*hdr).type_), soc_tplg_get_hdr_offset(tplg), (*(*tplg).fw).size);
        return -EINVAL;
    }
    if soc_tplg_get_hdr_offset(tplg) as usize + le32_to_cpu((*hdr).payload_size) as usize >= (*(*tplg).fw).size {
        dev_err((*tplg).dev, b"ASoC: invalid header of type %u at offset %ld payload_size %u\n\0".as_ptr() as *const c_char, le32_to_cpu((*hdr).type_), soc_tplg_get_hdr_offset(tplg), le32_to_cpu((*hdr).payload_size));
        return -EINVAL;
    }
    if le32_to_cpu((*hdr).magic) == SOC_TPLG_MAGIC_BIG_ENDIAN {
        dev_err((*tplg).dev, b"ASoC: pass %d big endian not supported header got %x at offset 0x%lx size 0x%zx.\n\0".as_ptr() as *const c_char, (*tplg).pass, le32_to_cpu((*hdr).magic), soc_tplg_get_hdr_offset(tplg), (*(*tplg).fw).size);
        return -EINVAL;
    }
    if le32_to_cpu((*hdr).magic) != SND_SOC_TPLG_MAGIC {
        dev_err((*tplg).dev, b"ASoC: pass %d does not have a valid header got %x at offset 0x%lx size 0x%zx.\n\0".as_ptr() as *const c_char, (*tplg).pass, le32_to_cpu((*hdr).magic), soc_tplg_get_hdr_offset(tplg), (*(*tplg).fw).size);
        return -EINVAL;
    }
    if le32_to_cpu((*hdr).abi) > SND_SOC_TPLG_ABI_VERSION || le32_to_cpu((*hdr).abi) < SND_SOC_TPLG_ABI_VERSION_MIN {
        dev_err((*tplg).dev, b"ASoC: pass %d invalid ABI version got 0x%x need 0x%x at offset 0x%lx size 0x%zx.\n\0".as_ptr() as *const c_char, (*tplg).pass, le32_to_cpu((*hdr).abi), SND_SOC_TPLG_ABI_VERSION, soc_tplg_get_hdr_offset(tplg), (*(*tplg).fw).size);
        return -EINVAL;
    }
    if (*hdr).payload_size == 0 {
        dev_err((*tplg).dev, b"ASoC: header has 0 size at offset 0x%lx.\n\0".as_ptr() as *const c_char, soc_tplg_get_hdr_offset(tplg));
        return -EINVAL;
    }
    0
}

type elem_load_fn = unsafe fn(*mut soc_tplg, *mut snd_soc_tplg_hdr) -> c_int;

unsafe fn soc_tplg_load_header(tplg: *mut soc_tplg, hdr: *mut snd_soc_tplg_hdr) -> c_int {
    (*tplg).pos = (*tplg).hdr_pos.add(size_of::<snd_soc_tplg_hdr>());
    (*tplg).index = le32_to_cpu((*hdr).index);
    let (hdr_pass, elem_load): (c_uint, elem_load_fn) = match le32_to_cpu((*hdr).type_) {
        SND_SOC_TPLG_TYPE_MIXER | SND_SOC_TPLG_TYPE_ENUM | SND_SOC_TPLG_TYPE_BYTES => (SOC_TPLG_PASS_CONTROL, soc_tplg_kcontrol_elems_load),
        SND_SOC_TPLG_TYPE_DAPM_GRAPH => (SOC_TPLG_PASS_GRAPH, soc_tplg_dapm_graph_elems_load),
        SND_SOC_TPLG_TYPE_DAPM_WIDGET => (SOC_TPLG_PASS_WIDGET, soc_tplg_dapm_widget_elems_load),
        SND_SOC_TPLG_TYPE_PCM => (SOC_TPLG_PASS_PCM_DAI, soc_tplg_pcm_elems_load),
        SND_SOC_TPLG_TYPE_DAI => (SOC_TPLG_PASS_BE_DAI, soc_tplg_dai_elems_load),
        SND_SOC_TPLG_TYPE_DAI_LINK | SND_SOC_TPLG_TYPE_BACKEND_LINK => (SOC_TPLG_PASS_LINK, soc_tplg_link_elems_load),
        SND_SOC_TPLG_TYPE_MANIFEST => (SOC_TPLG_PASS_MANIFEST, soc_tplg_manifest_load),
        _ => (SOC_TPLG_PASS_VENDOR, soc_tplg_vendor_load),
    };
    if (*tplg).pass == hdr_pass {
        dev_dbg((*tplg).dev, b"ASoC: Got 0x%x bytes of type %u version %u vendor %u at pass %d\n\0".as_ptr() as *const c_char, le32_to_cpu((*hdr).payload_size), le32_to_cpu((*hdr).type_), le32_to_cpu((*hdr).version), le32_to_cpu((*hdr).vendor_type), (*tplg).pass);
        return elem_load(tplg, hdr);
    }
    0
}

unsafe fn soc_tplg_process_headers(tplg: *mut soc_tplg) -> c_int {
    let mut ret: c_int;
    (*tplg).pass = SOC_TPLG_PASS_START;
    while (*tplg).pass <= SOC_TPLG_PASS_END {
        (*tplg).hdr_pos = (*(*tplg).fw).data;
        let mut hdr = (*tplg).hdr_pos as *mut snd_soc_tplg_hdr;
        while !soc_tplg_is_eof(tplg) {
            ret = soc_tplg_valid_header(tplg, hdr);
            if ret < 0 { return ret; }
            ret = soc_tplg_load_header(tplg, hdr);
            if ret < 0 {
                if ret != -EPROBE_DEFER {
                    dev_err((*tplg).dev, b"ASoC: topology: could not load header: %d\n\0".as_ptr() as *const c_char, ret);
                }
                return ret;
            }
            (*tplg).hdr_pos = (*tplg).hdr_pos.add(le32_to_cpu((*hdr).payload_size) as usize + size_of::<snd_soc_tplg_hdr>());
            hdr = (*tplg).hdr_pos as *mut snd_soc_tplg_hdr;
        }
        (*tplg).pass += 1;
    }
    soc_tplg_dapm_complete(tplg)
}

unsafe fn soc_tplg_load(tplg: *mut soc_tplg) -> c_int {
    let ret = soc_tplg_process_headers(tplg);
    if ret == 0 { return soc_tplg_complete(tplg); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_tplg_component_load(comp: *mut snd_soc_component, ops: *const snd_soc_tplg_ops, fw: *const firmware) -> c_int {
    let mut tplg: soc_tplg = zeroed();
    let mut ret: c_int;
    if comp.is_null() || (*comp).card.is_null() || (*(*comp).card).dev.is_null() || fw.is_null() {
        return -EINVAL;
    }
    memset(&mut tplg as *mut _ as *mut c_void, 0, size_of::<soc_tplg>());
    tplg.fw = fw;
    tplg.dev = (*(*comp).card).dev;
    tplg.comp = comp;
    if !ops.is_null() {
        tplg.ops = ops;
        tplg.io_ops = (*ops).io_ops;
        tplg.io_ops_count = (*ops).io_ops_count;
        tplg.bytes_ext_ops = (*ops).bytes_ext_ops;
        tplg.bytes_ext_ops_count = (*ops).bytes_ext_ops_count;
    }
    ret = soc_tplg_load(&mut tplg);
    if ret != 0 {
        snd_soc_tplg_component_remove(comp);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_tplg_component_remove(comp: *mut snd_soc_component) -> c_int {
    let mut pass = SOC_TPLG_PASS_END as c_int;
    while pass >= SOC_TPLG_PASS_START as c_int {
        /*
         * list_for_each_entry_safe(dobj, next_dobj, &comp->dobj_list, list)
         * is supplied by the kernel list implementation. The switch body is
         * translated below for each object produced by that iteration.
         */
        let mut dobj: *mut snd_soc_dobj = null_mut();
        while !dobj.is_null() {
            match (*dobj).type_ {
                SND_SOC_DOBJ_BYTES | SND_SOC_DOBJ_ENUM | SND_SOC_DOBJ_MIXER => soc_tplg_remove_kcontrol(comp, dobj, pass),
                SND_SOC_DOBJ_GRAPH => soc_tplg_remove_route(comp, dobj, pass),
                SND_SOC_DOBJ_WIDGET => soc_tplg_remove_widget(comp, dobj, pass),
                SND_SOC_DOBJ_PCM => soc_tplg_remove_dai(comp, dobj, pass),
                SND_SOC_DOBJ_DAI_LINK => soc_tplg_remove_link(comp, dobj, pass),
                SND_SOC_DOBJ_BACKEND_LINK => remove_backend_link(comp, dobj, pass),
                _ => dev_err((*comp).dev, b"ASoC: invalid component type %d for removal\n\0".as_ptr() as *const c_char, (*dobj).type_),
            }
            dobj = null_mut();
        }
        pass -= 1;
    }
    if list_empty(&(*comp).dobj_list) == 0 { 1 } else { 0 }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
