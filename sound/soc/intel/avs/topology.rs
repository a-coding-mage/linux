// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, MaybeUninit};
use core::ptr;

type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type size_t = usize;
type bool_ = bool;

extern "C" {
    static AVS_COPIER_MOD_UUID: guid_t;
    static avs_dai_fe_ops: snd_soc_dai_ops;

    fn guid_copy(dst: *mut guid_t, src: *const guid_t);
    fn guid_equal(a: *const guid_t, b: *const guid_t) -> bool_;
    fn le32_to_cpu(x: u32) -> u32;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_int) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: size_t, size: size_t, flags: c_int) -> *mut c_void;
    fn devm_kmemdup(dev: *mut device, src: *const c_void, len: size_t, flags: c_int) -> *mut c_void;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn avs_mach_singular_ssp(mach: *mut snd_soc_acpi_mach) -> bool_;
    fn avs_mach_ssp_port(mach: *mut snd_soc_acpi_mach) -> c_int;
    fn avs_mach_singular_tdm(mach: *mut snd_soc_acpi_mach, port: c_int) -> bool_;
    fn avs_mach_ssp_tdm(mach: *mut snd_soc_acpi_mach, port: c_int) -> c_int;
    fn to_avs_soc_component(comp: *mut snd_soc_component) -> *mut avs_soc_component;
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn list_add_tail(node: *mut list_head, head: *mut list_head);
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn snd_soc_tplg_component_load(
        comp: *mut snd_soc_component,
        ops: *const snd_soc_tplg_ops,
        fw: *const firmware,
    ) -> c_int;
    fn snd_soc_tplg_component_remove(comp: *mut snd_soc_component);

    fn avs_control_volume_get(kcontrol: *mut c_void, ucontrol: *mut c_void) -> c_int;
    fn avs_control_volume_put(kcontrol: *mut c_void, ucontrol: *mut c_void) -> c_int;
    fn avs_control_volume_info(kcontrol: *mut c_void, uinfo: *mut c_void) -> c_int;
    fn avs_control_mute_get(kcontrol: *mut c_void, ucontrol: *mut c_void) -> c_int;
    fn avs_control_mute_put(kcontrol: *mut c_void, ucontrol: *mut c_void) -> c_int;
    fn avs_control_mute_info(kcontrol: *mut c_void, uinfo: *mut c_void) -> c_int;
}

const GFP_KERNEL: c_int = 0;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: size_t = 44;
const SND_SOC_TPLG_TUPLE_TYPE_UUID: u32 = 0;
const SND_SOC_TPLG_TUPLE_TYPE_STRING: u32 = 1;
const SND_SOC_TPLG_TUPLE_TYPE_BOOL: u32 = 2;
const SND_SOC_TPLG_TUPLE_TYPE_BYTE: u32 = 3;
const SND_SOC_TPLG_TUPLE_TYPE_SHORT: u32 = 4;
const SND_SOC_TPLG_TUPLE_TYPE_WORD: u32 = 5;
const SND_SOC_TPLG_TYPE_MIXER: u32 = 1;
const SNDRV_PCM_SUBFMTBIT_MSBITS_20: u32 = 1 << 20;
const SNDRV_PCM_SUBFMTBIT_MSBITS_24: u32 = 1 << 24;
const SNDRV_PCM_SUBFMTBIT_MSBITS_MAX: u32 = 1 << 31;
const SND_SOC_DPCM_TRIGGER_PRE: c_int = 0;
const AVS_CONTROL_OPS_VOLUME: u32 = 257;
const AVS_CONTROL_OPS_MUTE: u32 = 258;

extern "C" {
    static AVS_S0IX_SUPPORTED: bool_;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! offset_of {
    ($ty:ty, $field:tt) => {
        core::mem::offset_of!($ty, $field) as u32
    };
}

unsafe fn ERR_PTR<T>(err: c_int) -> *mut T {
    err as isize as *mut T
}
unsafe fn PTR_ERR<T>(ptr: *mut T) -> isize {
    ptr as isize
}
unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as usize) > usize::MAX - 4096
}
unsafe fn ERR_CAST<T, U>(ptr: *mut T) -> *mut U {
    ptr as *mut U
}

#[repr(C)] pub struct device { _priv: [u8; 0] }
#[repr(C)] pub struct firmware { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_acpi_mach { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_ops { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { _priv: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { pub private_value: usize }
#[repr(C)] pub struct guid_t { pub b: [u8; 16] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }

#[repr(C)] pub struct snd_soc_card { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device, pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_soc_tplg_vendor_value_elem { pub token: u32, pub value: u32 }
#[repr(C)] pub struct snd_soc_tplg_vendor_uuid_elem { pub token: u32, pub uuid: guid_t }
#[repr(C)] pub struct snd_soc_tplg_vendor_string_elem { pub token: u32, pub string: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN] }

#[repr(C)]
pub union snd_soc_tplg_vendor_array_data {
    pub value: [snd_soc_tplg_vendor_value_elem; 0],
    pub uuid: [snd_soc_tplg_vendor_uuid_elem; 0],
    pub string: [snd_soc_tplg_vendor_string_elem; 0],
}

#[repr(C)]
pub struct snd_soc_tplg_vendor_array {
    pub size: u32,
    pub type_: u32,
    pub num_elems: u32,
    pub data: snd_soc_tplg_vendor_array_data,
}

impl snd_soc_tplg_vendor_array {
    unsafe fn value(&mut self) -> *mut snd_soc_tplg_vendor_value_elem { self.data.value.as_mut_ptr() }
    unsafe fn uuid(&mut self) -> *mut snd_soc_tplg_vendor_uuid_elem { self.data.uuid.as_mut_ptr() }
    unsafe fn string(&mut self) -> *mut snd_soc_tplg_vendor_string_elem { self.data.string.as_mut_ptr() }
}

#[repr(C)] pub struct snd_soc_tplg_private { pub size: u32, pub array: *mut snd_soc_tplg_vendor_array }
#[repr(C)] pub struct snd_soc_tplg_manifest { pub priv_: snd_soc_tplg_private }
#[repr(C)] pub struct snd_soc_tplg_dapm_widget { pub name: *mut c_char, pub priv_: snd_soc_tplg_private }
#[repr(C)] pub struct snd_soc_tplg_pcm { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_tplg_link_config { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_tplg_ctl_hdr { pub type_: u32 }
#[repr(C)] pub struct snd_soc_tplg_mixer_control { pub hdr: snd_soc_tplg_ctl_hdr, pub invert: u32, pub priv_: snd_soc_tplg_private }
#[repr(C)] pub struct snd_soc_dapm_route { pub source: *mut c_char, pub sink: *mut c_char, pub control: *mut c_char }
#[repr(C)] pub struct snd_soc_dapm_widget { pub no_wname_in_kcontrol_name: bool_, pub ignore_suspend: bool_, pub name: *mut c_char, pub priv_: *mut c_void }
#[repr(C)] pub struct snd_soc_dai_driver_stream { pub subformats: u32 }
#[repr(C)] pub struct snd_soc_dai_driver { pub ops: *const snd_soc_dai_ops, pub capture: snd_soc_dai_driver_stream, pub playback: snd_soc_dai_driver_stream }
#[repr(C)] pub struct snd_soc_dai_link { pub ignore_suspend: bool_, pub no_pcm: bool_, pub nonatomic: bool_, pub trigger: [c_int; 2], pub dpcm_merged_format: c_int }
#[repr(C)] pub struct snd_soc_tplg_kcontrol_ops { pub id: u32, pub get: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>, pub put: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>, pub info: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int> }
#[repr(C)] pub struct snd_soc_tplg_ops {
    pub io_ops: *const snd_soc_tplg_kcontrol_ops,
    pub io_ops_count: c_int,
    pub control_load: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, *mut snd_kcontrol_new, *mut snd_soc_tplg_ctl_hdr) -> c_int>,
    pub dapm_route_load: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, *mut snd_soc_dapm_route) -> c_int>,
    pub widget_load: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, *mut snd_soc_dapm_widget, *mut snd_soc_tplg_dapm_widget) -> c_int>,
    pub widget_ready: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, *mut snd_soc_dapm_widget, *mut snd_soc_tplg_dapm_widget) -> c_int>,
    pub dai_load: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, *mut snd_soc_dai_driver, *mut snd_soc_tplg_pcm, *mut snd_soc_dai) -> c_int>,
    pub link_load: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, *mut snd_soc_dai_link, *mut snd_soc_tplg_link_config) -> c_int>,
    pub manifest: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, *mut snd_soc_tplg_manifest) -> c_int>,
}

#[repr(C)] pub struct soc_mixer_control { pub dobj: snd_soc_dobj, pub max: u32, pub num_channels: c_int }
#[repr(C)] pub struct snd_soc_dobj { pub private: *mut c_void }
#[repr(C)] pub struct avs_control_data { pub id: u32, pub values: [u32; 8] }
#[repr(C)] pub struct acpi_nhlt_config { pub capabilities_size: u32, pub capabilities: [u8; 0] }
#[repr(C)] pub struct avs_audio_format { pub sampling_freq: u32, pub bit_depth: u32, pub channel_map: u32, pub channel_config: u32, pub interleaving: u32, pub num_channels: u32, pub valid_bit_depth: u32, pub sample_type: u32 }
#[repr(C)] pub struct avs_tplg_library { pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN] }
#[repr(C)] pub struct avs_tplg_modcfg_base { pub cpc: u32, pub ibs: u32, pub obs: u32, pub is_pages: u32 }
#[repr(C)] pub struct avs_i2s_vindex { pub instance: c_int, pub time_slot: c_int }
#[repr(C)] pub union avs_copier_vindex { pub val: u32, pub i2s: core::mem::ManuallyDrop<avs_i2s_vindex> }
#[repr(C)] pub struct avs_tplg_copier_cfg { pub out_fmt: *mut avs_audio_format, pub feature_mask: u32, pub vindex: avs_copier_vindex, pub dma_type: u32, pub dma_buffer_size: u32, pub blob_fmt: *mut avs_audio_format }
#[repr(C)] pub struct avs_tplg_generic_cfg { pub num_input_pins: u16, pub num_output_pins: u16, pub pin_fmts: *mut avs_tplg_pin_format }
#[repr(C)] pub struct avs_tplg_modcfg_ext { pub type_: guid_t, pub copier: avs_tplg_copier_cfg, pub micsel: avs_fmt1, pub wov: avs_wov, pub src: avs_src, pub mux: avs_fmt2, pub aec: avs_aec, pub asrc: avs_asrc, pub updown_mix: avs_updown_mix, pub generic: avs_tplg_generic_cfg, pub whm: avs_whm, pub peakvol: avs_peakvol }
#[repr(C)] pub struct avs_fmt1 { pub out_fmt: *mut avs_audio_format }
#[repr(C)] pub struct avs_wov { pub cpc_lp_mode: u32 }
#[repr(C)] pub struct avs_src { pub out_freq: u32 }
#[repr(C)] pub struct avs_fmt2 { pub ref_fmt: *mut avs_audio_format, pub out_fmt: *mut avs_audio_format }
#[repr(C)] pub struct avs_aec { pub ref_fmt: *mut avs_audio_format, pub out_fmt: *mut avs_audio_format, pub cpc_lp_mode: u32 }
#[repr(C)] pub struct avs_asrc { pub out_freq: u32, pub mode: u8, pub disable_jitter_buffer: u8 }
#[repr(C)] pub struct avs_updown_mix { pub out_channel_config: u32, pub coefficients_select: u32, pub coefficients: [u32; 8], pub channel_map: u32 }
#[repr(C)] pub struct avs_whm { pub ref_fmt: *mut avs_audio_format, pub out_fmt: *mut avs_audio_format, pub wake_tick_period: u32, pub vindex: u8, pub dma_type: u32, pub dma_buffer_size: u32, pub blob_fmt: *mut avs_audio_format }
#[repr(C)] pub struct avs_peakvol { pub target_volume: u32, pub curve_type: u32, pub curve_duration: u32 }
#[repr(C)] pub struct avs_tplg_pin_format { pub pin_index: u32, pub iobs: u32, pub fmt: *mut avs_audio_format }
#[repr(C)] pub struct avs_tplg_pplcfg { pub req_size: u16, pub priority: u8, pub lp: bool_, pub attributes: u16, pub trigger: u32 }
#[repr(C)] pub struct avs_tplg_binding { pub target_tplg_name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub target_path_tmpl_id: u32, pub target_ppl_id: u32, pub target_mod_id: u32, pub target_mod_pin: u8, pub mod_id: u32, pub mod_pin: u8, pub is_sink: u8 }
#[repr(C)] pub struct avs_tplg_module { pub id: u32, pub cfg_base: *mut avs_tplg_modcfg_base, pub in_fmt: *mut avs_audio_format, pub core_id: u8, pub domain: u8, pub cfg_ext: *mut avs_tplg_modcfg_ext, pub ctl_id: u8, pub num_config_ids: u8, pub nhlt_config: *mut avs_tplg_nhlt_config, pub config_ids: *mut u32, pub owner: *mut avs_tplg_pipeline, pub node: list_head }
#[repr(C)] pub struct avs_tplg_pipeline { pub id: u32, pub cfg: *mut avs_tplg_pplcfg, pub num_bindings: u32, pub bindings: *mut *mut avs_tplg_binding, pub owner: *mut avs_tplg_path, pub mod_list: list_head, pub node: list_head }
#[repr(C)] pub struct avs_link_ref { pub tplg_name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN], pub id: u32 }
#[repr(C)] pub struct avs_tplg_path { pub id: u32, pub source_path_id: u32, pub sink_path_id: u32, pub fe_fmt: *mut avs_audio_format, pub be_fmt: *mut avs_audio_format, pub owner: *mut avs_tplg_path_template, pub ppl_list: list_head, pub node: list_head }
#[repr(C)] pub struct avs_tplg_path_template { pub id: u32, pub source: avs_link_ref, pub sink: avs_link_ref, pub owner: *mut avs_tplg, pub path_list: list_head, pub node: list_head, pub w: *mut snd_soc_dapm_widget }
#[repr(C)] pub struct avs_tplg_init_config { pub id: u32, pub param: u8, pub length: u32, pub data: *mut c_void }
#[repr(C)] pub struct avs_tplg_nhlt_config { pub id: u32, pub blob: *mut acpi_nhlt_config }
#[repr(C)] pub struct avs_tplg {
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub version: u32,
    pub libs: *mut avs_tplg_library, pub num_libs: u32,
    pub fmts: *mut avs_audio_format, pub num_fmts: u32,
    pub modcfgs_base: *mut avs_tplg_modcfg_base, pub num_modcfgs_base: u32,
    pub modcfgs_ext: *mut avs_tplg_modcfg_ext, pub num_modcfgs_ext: u32,
    pub pplcfgs: *mut avs_tplg_pplcfg, pub num_pplcfgs: u32,
    pub bindings: *mut avs_tplg_binding, pub num_bindings: u32,
    pub condpath_tmpls: *mut avs_tplg_path_template, pub num_condpath_tmpls: u32,
    pub init_configs: *mut avs_tplg_init_config, pub num_init_configs: u32,
    pub nhlt_configs: *mut avs_tplg_nhlt_config, pub num_nhlt_configs: u32,
    pub comp: *mut snd_soc_component,
    pub path_tmpl_list: list_head,
}
#[repr(C)] pub struct avs_soc_component { pub tplg: *mut avs_tplg }

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avs_tplg_token { Dummy = 0 }

// Get pointer to vendor array at the specified offset.
unsafe fn avs_tplg_vendor_array_at(array: *mut snd_soc_tplg_vendor_array, offset: u32) -> *mut snd_soc_tplg_vendor_array {
    (array as *mut u8).add(offset as usize) as *mut snd_soc_tplg_vendor_array
}

// Get pointer to vendor array that is next in line.
unsafe fn avs_tplg_vendor_array_next(array: *mut snd_soc_tplg_vendor_array) -> *mut snd_soc_tplg_vendor_array {
    avs_tplg_vendor_array_at(array, le32_to_cpu((*array).size))
}

#[repr(C)]
struct avs_tplg_token_parser {
    token: u32,
    type_: u32,
    offset: u32,
    parse: unsafe extern "C" fn(*mut snd_soc_component, *mut c_void, *mut c_void, u32) -> c_int,
}

unsafe extern "C" fn avs_parse_uuid_token(_: *mut snd_soc_component, elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int {
    let tuple = elem as *mut snd_soc_tplg_vendor_uuid_elem;
    let val = (object as *mut u8).add(offset as usize) as *mut guid_t;
    guid_copy(val, &(*tuple).uuid);
    0
}

unsafe extern "C" fn avs_parse_bool_token(_: *mut snd_soc_component, elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int {
    let tuple = elem as *mut snd_soc_tplg_vendor_value_elem;
    *((object as *mut u8).add(offset as usize) as *mut bool_) = le32_to_cpu((*tuple).value) != 0;
    0
}

unsafe extern "C" fn avs_parse_byte_token(_: *mut snd_soc_component, elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int {
    let tuple = elem as *mut snd_soc_tplg_vendor_value_elem;
    *((object as *mut u8).add(offset as usize)) = le32_to_cpu((*tuple).value) as u8;
    0
}

unsafe extern "C" fn avs_parse_short_token(_: *mut snd_soc_component, elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int {
    let tuple = elem as *mut snd_soc_tplg_vendor_value_elem;
    *((object as *mut u8).add(offset as usize) as *mut u16) = le32_to_cpu((*tuple).value) as u16;
    0
}

unsafe extern "C" fn avs_parse_word_token(_: *mut snd_soc_component, elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int {
    let tuple = elem as *mut snd_soc_tplg_vendor_value_elem;
    *((object as *mut u8).add(offset as usize) as *mut u32) = le32_to_cpu((*tuple).value);
    0
}

unsafe extern "C" fn avs_parse_string_token(_: *mut snd_soc_component, elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int {
    let tuple = elem as *mut snd_soc_tplg_vendor_string_elem;
    let val = (object as *mut u8).add(offset as usize) as *mut c_char;
    snprintf(val, SNDRV_CTL_ELEM_ID_NAME_MAXLEN, cstr!("%s"), (*tuple).string.as_ptr());
    0
}

unsafe fn avs_tplg_vendor_array_lookup(mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32, token: u32, offset: *mut u32) -> c_int {
    let mut pos = 0;
    while block_size > 0 {
        let tuples_size = le32_to_cpu((*tuples).size);
        if tuples_size > block_size { return -EINVAL; }
        let tuple = (*tuples).value();
        if le32_to_cpu((*tuple).token) == token {
            *offset = pos;
            return 0;
        }
        block_size -= tuples_size;
        pos += tuples_size;
        tuples = avs_tplg_vendor_array_next(tuples);
    }
    -ENOENT
}

unsafe fn avs_tplg_vendor_array_lookup_next(mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32, token: u32, offset: *mut u32) -> c_int {
    let tuples_size = le32_to_cpu((*tuples).size);
    if tuples_size > block_size { return -EINVAL; }
    tuples = avs_tplg_vendor_array_next(tuples);
    block_size -= tuples_size;
    let ret = avs_tplg_vendor_array_lookup(tuples, block_size, token, offset);
    if ret == 0 { *offset += tuples_size; }
    ret
}

unsafe fn avs_tplg_vendor_entry_size(tuples: *mut snd_soc_tplg_vendor_array, block_size: u32, entry_id_token: u32, size: *mut u32) -> c_int {
    let mut ret = avs_tplg_vendor_array_lookup_next(tuples, block_size, entry_id_token, size);
    if ret == -ENOENT {
        *size = block_size;
        ret = 0;
    }
    ret
}

unsafe fn avs_parse_uuid_tokens(comp: *mut snd_soc_component, object: *mut c_void, parsers: *const avs_tplg_token_parser, count: c_int, tuples: *mut snd_soc_tplg_vendor_array) -> c_int {
    for i in 0..le32_to_cpu((*tuples).num_elems) as isize {
        let tuple = (*tuples).uuid().offset(i);
        for j in 0..count as isize {
            let parser = parsers.offset(j);
            if (*parser).type_ != SND_SOC_TPLG_TUPLE_TYPE_UUID || (*parser).token != le32_to_cpu((*tuple).token) { continue; }
            let ret = ((*parser).parse)(comp, tuple as *mut c_void, object, (*parser).offset);
            if ret != 0 { return ret; }
        }
    }
    0
}

unsafe fn avs_parse_string_tokens(comp: *mut snd_soc_component, object: *mut c_void, parsers: *const avs_tplg_token_parser, count: c_int, tuples: *mut snd_soc_tplg_vendor_array) -> c_int {
    for i in 0..le32_to_cpu((*tuples).num_elems) as isize {
        let tuple = (*tuples).string().offset(i);
        for j in 0..count as isize {
            let parser = parsers.offset(j);
            if (*parser).type_ != SND_SOC_TPLG_TUPLE_TYPE_STRING || (*parser).token != le32_to_cpu((*tuple).token) { continue; }
            let ret = ((*parser).parse)(comp, tuple as *mut c_void, object, (*parser).offset);
            if ret != 0 { return ret; }
        }
    }
    0
}

unsafe fn avs_parse_word_tokens(comp: *mut snd_soc_component, object: *mut c_void, parsers: *const avs_tplg_token_parser, count: c_int, tuples: *mut snd_soc_tplg_vendor_array) -> c_int {
    for i in 0..le32_to_cpu((*tuples).num_elems) as isize {
        let tuple = (*tuples).value().offset(i);
        for j in 0..count as isize {
            let parser = parsers.offset(j);
            if !((*parser).type_ == SND_SOC_TPLG_TUPLE_TYPE_WORD || (*parser).type_ == SND_SOC_TPLG_TUPLE_TYPE_SHORT || (*parser).type_ == SND_SOC_TPLG_TUPLE_TYPE_BYTE || (*parser).type_ == SND_SOC_TPLG_TUPLE_TYPE_BOOL) { continue; }
            if (*parser).token != le32_to_cpu((*tuple).token) { continue; }
            let ret = ((*parser).parse)(comp, tuple as *mut c_void, object, (*parser).offset);
            if ret != 0 { return ret; }
        }
    }
    0
}

unsafe fn avs_parse_tokens(comp: *mut snd_soc_component, object: *mut c_void, parsers: *const avs_tplg_token_parser, count: size_t, mut tuples: *mut snd_soc_tplg_vendor_array, mut priv_size: c_int) -> c_int {
    while priv_size > 0 {
        let array_size = le32_to_cpu((*tuples).size) as c_int;
        if array_size <= 0 { return -EINVAL; }
        priv_size -= array_size;
        if priv_size < 0 { return -EINVAL; }
        let ret = match le32_to_cpu((*tuples).type_) {
            SND_SOC_TPLG_TUPLE_TYPE_UUID => avs_parse_uuid_tokens(comp, object, parsers, count as c_int, tuples),
            SND_SOC_TPLG_TUPLE_TYPE_STRING => avs_parse_string_tokens(comp, object, parsers, count as c_int, tuples),
            SND_SOC_TPLG_TUPLE_TYPE_BOOL | SND_SOC_TPLG_TUPLE_TYPE_BYTE | SND_SOC_TPLG_TUPLE_TYPE_SHORT | SND_SOC_TPLG_TUPLE_TYPE_WORD => avs_parse_word_tokens(comp, object, parsers, count as c_int, tuples),
            _ => -EINVAL,
        };
        if ret != 0 { return ret; }
        tuples = avs_tplg_vendor_array_next(tuples);
    }
    0
}

macro_rules! define_ptr_parser {
    ($name:ident, $ty:ty, $num:ident, $member:ident) => {
        unsafe extern "C" fn $name(comp: *mut snd_soc_component, elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int {
            let tuple = elem as *mut snd_soc_tplg_vendor_value_elem;
            let acomp = to_avs_soc_component(comp);
            let val = (object as *mut u8).add(offset as usize) as *mut *mut $ty;
            let idx = le32_to_cpu((*tuple).value);
            if idx >= (*(*acomp).tplg).$num { return -EINVAL; }
            *val = (*(*acomp).tplg).$member.add(idx as usize);
            0
        }
    };
}

define_ptr_parser!(avs_parse_audio_format_ptr, avs_audio_format, num_fmts, fmts);
define_ptr_parser!(avs_parse_modcfg_base_ptr, avs_tplg_modcfg_base, num_modcfgs_base, modcfgs_base);
define_ptr_parser!(avs_parse_modcfg_ext_ptr, avs_tplg_modcfg_ext, num_modcfgs_ext, modcfgs_ext);
define_ptr_parser!(avs_parse_pplcfg_ptr, avs_tplg_pplcfg, num_pplcfgs, pplcfgs);
define_ptr_parser!(avs_parse_binding_ptr, avs_tplg_binding, num_bindings, bindings);
define_ptr_parser!(avs_parse_nhlt_config_ptr, avs_tplg_nhlt_config, num_nhlt_configs, nhlt_configs);

unsafe extern "C" fn parse_audio_format_bitfield(_: *mut snd_soc_component, elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int {
    let velem = elem as *mut snd_soc_tplg_vendor_value_elem;
    let audio_format = object as *mut avs_audio_format;
    match offset {
        AVS_TKN_AFMT_NUM_CHANNELS_U32 => (*audio_format).num_channels = le32_to_cpu((*velem).value),
        AVS_TKN_AFMT_VALID_BIT_DEPTH_U32 => (*audio_format).valid_bit_depth = le32_to_cpu((*velem).value),
        AVS_TKN_AFMT_SAMPLE_TYPE_U32 => (*audio_format).sample_type = le32_to_cpu((*velem).value),
        _ => {}
    }
    0
}

unsafe fn avs_ssp_sprint(buf: *mut c_char, size: size_t, fmt: *const c_char, port: c_int, tdm: c_int) -> c_int {
    let needle = strstr(fmt, cstr!("%d"));
    if !needle.is_null() {
        let mut retsize = scnprintf(buf, core::cmp::min(size, needle.offset_from(fmt) as usize + 1), cstr!("%s"), fmt);
        retsize += scnprintf(buf.add(retsize as usize), size - retsize as usize, cstr!("%d"), port);
        if tdm != 0 {
            retsize += scnprintf(buf.add(retsize as usize), size - retsize as usize, cstr!(":%d"), tdm);
        }
        retsize += scnprintf(buf.add(retsize as usize), size - retsize as usize, cstr!("%s"), needle.add(2));
        return retsize;
    }
    snprintf(buf, size, cstr!("%s"), fmt)
}

unsafe extern "C" fn parse_link_formatted_string(comp: *mut snd_soc_component, elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int {
    let tuple = elem as *mut snd_soc_tplg_vendor_string_elem;
    let mach = dev_get_platdata((*(*comp).card).dev) as *mut snd_soc_acpi_mach;
    let val = (object as *mut u8).add(offset as usize) as *mut c_char;
    if !avs_mach_singular_ssp(mach) { return avs_parse_string_token(comp, elem, object, offset); }
    let ssp_port = avs_mach_ssp_port(mach);
    if !avs_mach_singular_tdm(mach, ssp_port) { return avs_parse_string_token(comp, elem, object, offset); }
    let tdm_slot = avs_mach_ssp_tdm(mach, ssp_port);
    avs_ssp_sprint(val, SNDRV_CTL_ELEM_ID_NAME_MAXLEN, (*tuple).string.as_ptr(), ssp_port, tdm_slot);
    0
}

unsafe extern "C" fn avs_parse_nhlt_config_size(comp: *mut snd_soc_component, elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int {
    let tuple = elem as *mut snd_soc_tplg_vendor_value_elem;
    let blob = (object as *mut u8).add(offset as usize) as *mut *mut acpi_nhlt_config;
    let size = le32_to_cpu((*tuple).value);
    *blob = devm_kzalloc((*(*comp).card).dev, size_of::<acpi_nhlt_config>() + size as usize, GFP_KERNEL) as *mut acpi_nhlt_config;
    if (*blob).is_null() { return -ENOMEM; }
    (**blob).capabilities_size = size;
    0
}

unsafe fn parse_dictionary_header(comp: *mut snd_soc_component, tuples: *mut snd_soc_tplg_vendor_array, dict: *mut *mut c_void, num_entries: *mut u32, entry_size: size_t, num_entries_token: u32) -> c_int {
    let tuple = (*tuples).value();
    if le32_to_cpu((*tuple).token) != num_entries_token { return -EINVAL; }
    *num_entries = le32_to_cpu((*tuple).value);
    *dict = devm_kcalloc((*(*comp).card).dev, *num_entries as usize, entry_size, GFP_KERNEL);
    if (*dict).is_null() { return -ENOMEM; }
    0
}

unsafe fn parse_dictionary_entries(comp: *mut snd_soc_component, mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32, dict: *mut c_void, num_entries: u32, entry_size: size_t, entry_id_token: u32, parsers: *const avs_tplg_token_parser, num_parsers: size_t) -> c_int {
    let mut pos = dict as *mut u8;
    for i in 0..num_entries {
        let mut esize = 0;
        let mut ret = avs_tplg_vendor_entry_size(tuples, block_size, entry_id_token, &mut esize);
        if ret != 0 { return ret; }
        ret = avs_parse_tokens(comp, pos as *mut c_void, parsers, num_parsers, tuples, esize as c_int);
        if ret < 0 { let _ = i; return ret; }
        pos = pos.add(entry_size);
        block_size -= esize;
        tuples = avs_tplg_vendor_array_at(tuples, esize);
    }
    0
}

unsafe fn parse_dictionary(comp: *mut snd_soc_component, mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32, dict: *mut *mut c_void, num_entries: *mut u32, entry_size: size_t, num_entries_token: u32, entry_id_token: u32, parsers: *const avs_tplg_token_parser, num_parsers: size_t) -> c_int {
    let ret = parse_dictionary_header(comp, tuples, dict, num_entries, entry_size, num_entries_token);
    if ret != 0 { return ret; }
    block_size -= le32_to_cpu((*tuples).size);
    tuples = avs_tplg_vendor_array_next(tuples);
    parse_dictionary_entries(comp, tuples, block_size, *dict, *num_entries, entry_size, entry_id_token, parsers, num_parsers)
}

macro_rules! parser {
    ($token:expr, $type:expr, $offset:expr, $parse:path) => {
        avs_tplg_token_parser { token: $token, type_: $type, offset: $offset, parse: $parse }
    };
}

static library_parsers: [avs_tplg_token_parser; 1] = [
    parser!(AVS_TKN_LIBRARY_NAME_STRING, SND_SOC_TPLG_TUPLE_TYPE_STRING, offset_of!(avs_tplg_library, name), avs_parse_string_token),
];

unsafe fn avs_tplg_parse_libraries(comp: *mut snd_soc_component, tuples: *mut snd_soc_tplg_vendor_array, block_size: u32) -> c_int {
    let tplg = (*to_avs_soc_component(comp)).tplg;
    parse_dictionary(comp, tuples, block_size, &mut (*tplg).libs as *mut _ as *mut *mut c_void, &mut (*tplg).num_libs, size_of::<avs_tplg_library>(), AVS_TKN_MANIFEST_NUM_LIBRARIES_U32, AVS_TKN_LIBRARY_ID_U32, library_parsers.as_ptr(), library_parsers.len())
}

static audio_format_parsers: [avs_tplg_token_parser; 8] = [
    parser!(AVS_TKN_AFMT_SAMPLE_RATE_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_audio_format, sampling_freq), avs_parse_word_token),
    parser!(AVS_TKN_AFMT_BIT_DEPTH_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_audio_format, bit_depth), avs_parse_word_token),
    parser!(AVS_TKN_AFMT_CHANNEL_MAP_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_audio_format, channel_map), avs_parse_word_token),
    parser!(AVS_TKN_AFMT_CHANNEL_CFG_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_audio_format, channel_config), avs_parse_word_token),
    parser!(AVS_TKN_AFMT_INTERLEAVING_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_audio_format, interleaving), avs_parse_word_token),
    parser!(AVS_TKN_AFMT_NUM_CHANNELS_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, AVS_TKN_AFMT_NUM_CHANNELS_U32, parse_audio_format_bitfield),
    parser!(AVS_TKN_AFMT_VALID_BIT_DEPTH_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, AVS_TKN_AFMT_VALID_BIT_DEPTH_U32, parse_audio_format_bitfield),
    parser!(AVS_TKN_AFMT_SAMPLE_TYPE_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, AVS_TKN_AFMT_SAMPLE_TYPE_U32, parse_audio_format_bitfield),
];

unsafe fn avs_tplg_parse_audio_formats(comp: *mut snd_soc_component, tuples: *mut snd_soc_tplg_vendor_array, block_size: u32) -> c_int {
    let tplg = (*to_avs_soc_component(comp)).tplg;
    parse_dictionary(comp, tuples, block_size, &mut (*tplg).fmts as *mut _ as *mut *mut c_void, &mut (*tplg).num_fmts, size_of::<avs_audio_format>(), AVS_TKN_MANIFEST_NUM_AFMTS_U32, AVS_TKN_AFMT_ID_U32, audio_format_parsers.as_ptr(), audio_format_parsers.len())
}

static modcfg_base_parsers: [avs_tplg_token_parser; 4] = [
    parser!(AVS_TKN_MODCFG_BASE_CPC_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_modcfg_base, cpc), avs_parse_word_token),
    parser!(AVS_TKN_MODCFG_BASE_IBS_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_modcfg_base, ibs), avs_parse_word_token),
    parser!(AVS_TKN_MODCFG_BASE_OBS_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_modcfg_base, obs), avs_parse_word_token),
    parser!(AVS_TKN_MODCFG_BASE_PAGES_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_modcfg_base, is_pages), avs_parse_word_token),
];

unsafe fn avs_tplg_parse_modcfgs_base(comp: *mut snd_soc_component, tuples: *mut snd_soc_tplg_vendor_array, block_size: u32) -> c_int {
    let tplg = (*to_avs_soc_component(comp)).tplg;
    parse_dictionary(comp, tuples, block_size, &mut (*tplg).modcfgs_base as *mut _ as *mut *mut c_void, &mut (*tplg).num_modcfgs_base, size_of::<avs_tplg_modcfg_base>(), AVS_TKN_MANIFEST_NUM_MODCFGS_BASE_U32, AVS_TKN_MODCFG_BASE_ID_U32, modcfg_base_parsers.as_ptr(), modcfg_base_parsers.len())
}

static modcfg_ext_parsers: &[avs_tplg_token_parser] = &[
    parser!(AVS_TKN_MODCFG_EXT_TYPE_UUID, SND_SOC_TPLG_TUPLE_TYPE_UUID, offset_of!(avs_tplg_modcfg_ext, type_), avs_parse_uuid_token),
    parser!(AVS_TKN_MODCFG_CPR_OUT_AFMT_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_modcfg_ext, copier), avs_parse_audio_format_ptr),
    parser!(AVS_TKN_MODCFG_CPR_FEATURE_MASK_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_copier_cfg, feature_mask) + offset_of!(avs_tplg_modcfg_ext, copier), avs_parse_word_token),
    parser!(AVS_TKN_MODCFG_CPR_VINDEX_U8, SND_SOC_TPLG_TUPLE_TYPE_BYTE, offset_of!(avs_tplg_copier_cfg, vindex) + offset_of!(avs_tplg_modcfg_ext, copier), avs_parse_byte_token),
    parser!(AVS_TKN_MODCFG_CPR_DMA_TYPE_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_copier_cfg, dma_type) + offset_of!(avs_tplg_modcfg_ext, copier), avs_parse_word_token),
    parser!(AVS_TKN_MODCFG_CPR_DMABUFF_SIZE_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_copier_cfg, dma_buffer_size) + offset_of!(avs_tplg_modcfg_ext, copier), avs_parse_word_token),
    parser!(AVS_TKN_MODCFG_CPR_BLOB_FMT_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_copier_cfg, blob_fmt) + offset_of!(avs_tplg_modcfg_ext, copier), avs_parse_audio_format_ptr),
    parser!(AVS_TKN_MODCFG_EXT_NUM_INPUT_PINS_U16, SND_SOC_TPLG_TUPLE_TYPE_SHORT, offset_of!(avs_tplg_modcfg_ext, generic) + offset_of!(avs_tplg_generic_cfg, num_input_pins), avs_parse_short_token),
    parser!(AVS_TKN_MODCFG_EXT_NUM_OUTPUT_PINS_U16, SND_SOC_TPLG_TUPLE_TYPE_SHORT, offset_of!(avs_tplg_modcfg_ext, generic) + offset_of!(avs_tplg_generic_cfg, num_output_pins), avs_parse_short_token),
];

static pin_format_parsers: [avs_tplg_token_parser; 3] = [
    parser!(AVS_TKN_PIN_FMT_INDEX_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_pin_format, pin_index), avs_parse_word_token),
    parser!(AVS_TKN_PIN_FMT_IOBS_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_pin_format, iobs), avs_parse_word_token),
    parser!(AVS_TKN_PIN_FMT_AFMT_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_pin_format, fmt), avs_parse_audio_format_ptr),
];

unsafe fn assign_copier_gtw_instance(comp: *mut snd_soc_component, cfg: *mut avs_tplg_modcfg_ext) {
    if !guid_equal(&(*cfg).type_, &AVS_COPIER_MOD_UUID) { return; }
    match (*cfg).copier.dma_type {
        AVS_DMA_I2S_LINK_OUTPUT | AVS_DMA_I2S_LINK_INPUT => {}
        _ => return,
    }
    if (*cfg).copier.vindex.val != 0 { return; }
    let mach = dev_get_platdata((*(*comp).card).dev) as *mut snd_soc_acpi_mach;
    if !avs_mach_singular_ssp(mach) { return; }
    let ssp_port = avs_mach_ssp_port(mach);
    if !avs_mach_singular_tdm(mach, ssp_port) { return; }
    let tdm_slot = avs_mach_ssp_tdm(mach, ssp_port);
    (*cfg).copier.vindex.i2s.instance = ssp_port;
    (*cfg).copier.vindex.i2s.time_slot = tdm_slot;
}

unsafe fn avs_tplg_parse_modcfg_ext(comp: *mut snd_soc_component, cfg: *mut avs_tplg_modcfg_ext, mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32) -> c_int {
    let mut esize = 0;
    let mut ret = avs_tplg_vendor_entry_size(tuples, block_size, AVS_TKN_PIN_FMT_INDEX_U32, &mut esize);
    if ret != 0 { return ret; }
    ret = avs_parse_tokens(comp, cfg as *mut c_void, modcfg_ext_parsers.as_ptr(), modcfg_ext_parsers.len(), tuples, esize as c_int);
    if ret != 0 { return ret; }
    assign_copier_gtw_instance(comp, cfg);
    block_size -= esize;
    if block_size != 0 {
        let num_pins = (*cfg).generic.num_input_pins as u32 + (*cfg).generic.num_output_pins as u32;
        if num_pins == 0 { return -EINVAL; }
        let pins = devm_kcalloc((*(*comp).card).dev, num_pins as usize, size_of::<avs_tplg_pin_format>(), GFP_KERNEL) as *mut avs_tplg_pin_format;
        if pins.is_null() { return -ENOMEM; }
        tuples = avs_tplg_vendor_array_at(tuples, esize);
        ret = parse_dictionary_entries(comp, tuples, block_size, pins as *mut c_void, num_pins, size_of::<avs_tplg_pin_format>(), AVS_TKN_PIN_FMT_INDEX_U32, pin_format_parsers.as_ptr(), pin_format_parsers.len());
        if ret != 0 { return ret; }
        (*cfg).generic.pin_fmts = pins;
    }
    0
}

unsafe fn avs_tplg_parse_modcfgs_ext(comp: *mut snd_soc_component, mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32) -> c_int {
    let tplg = (*to_avs_soc_component(comp)).tplg;
    let mut ret = parse_dictionary_header(comp, tuples, &mut (*tplg).modcfgs_ext as *mut _ as *mut *mut c_void, &mut (*tplg).num_modcfgs_ext, size_of::<avs_tplg_modcfg_ext>(), AVS_TKN_MANIFEST_NUM_MODCFGS_EXT_U32);
    if ret != 0 { return ret; }
    block_size -= le32_to_cpu((*tuples).size);
    tuples = avs_tplg_vendor_array_next(tuples);
    for i in 0..(*tplg).num_modcfgs_ext {
        let cfg = (*tplg).modcfgs_ext.add(i as usize);
        let mut esize = 0;
        ret = avs_tplg_vendor_entry_size(tuples, block_size, AVS_TKN_MODCFG_EXT_ID_U32, &mut esize);
        if ret != 0 { return ret; }
        ret = avs_tplg_parse_modcfg_ext(comp, cfg, tuples, esize);
        if ret != 0 { return ret; }
        block_size -= esize;
        tuples = avs_tplg_vendor_array_at(tuples, esize);
    }
    0
}

static pplcfg_parsers: [avs_tplg_token_parser; 5] = [
    parser!(AVS_TKN_PPLCFG_REQ_SIZE_U16, SND_SOC_TPLG_TUPLE_TYPE_SHORT, offset_of!(avs_tplg_pplcfg, req_size), avs_parse_short_token),
    parser!(AVS_TKN_PPLCFG_PRIORITY_U8, SND_SOC_TPLG_TUPLE_TYPE_BYTE, offset_of!(avs_tplg_pplcfg, priority), avs_parse_byte_token),
    parser!(AVS_TKN_PPLCFG_LOW_POWER_BOOL, SND_SOC_TPLG_TUPLE_TYPE_BOOL, offset_of!(avs_tplg_pplcfg, lp), avs_parse_bool_token),
    parser!(AVS_TKN_PPLCFG_ATTRIBUTES_U16, SND_SOC_TPLG_TUPLE_TYPE_SHORT, offset_of!(avs_tplg_pplcfg, attributes), avs_parse_short_token),
    parser!(AVS_TKN_PPLCFG_TRIGGER_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_pplcfg, trigger), avs_parse_word_token),
];

unsafe fn avs_tplg_parse_pplcfgs(comp: *mut snd_soc_component, tuples: *mut snd_soc_tplg_vendor_array, block_size: u32) -> c_int {
    let tplg = (*to_avs_soc_component(comp)).tplg;
    parse_dictionary(comp, tuples, block_size, &mut (*tplg).pplcfgs as *mut _ as *mut *mut c_void, &mut (*tplg).num_pplcfgs, size_of::<avs_tplg_pplcfg>(), AVS_TKN_MANIFEST_NUM_PPLCFGS_U32, AVS_TKN_PPLCFG_ID_U32, pplcfg_parsers.as_ptr(), pplcfg_parsers.len())
}

static binding_parsers: [avs_tplg_token_parser; 8] = [
    parser!(AVS_TKN_BINDING_TARGET_TPLG_NAME_STRING, SND_SOC_TPLG_TUPLE_TYPE_STRING, offset_of!(avs_tplg_binding, target_tplg_name), parse_link_formatted_string),
    parser!(AVS_TKN_BINDING_TARGET_PATH_TMPL_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_binding, target_path_tmpl_id), avs_parse_word_token),
    parser!(AVS_TKN_BINDING_TARGET_PPL_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_binding, target_ppl_id), avs_parse_word_token),
    parser!(AVS_TKN_BINDING_TARGET_MOD_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_binding, target_mod_id), avs_parse_word_token),
    parser!(AVS_TKN_BINDING_TARGET_MOD_PIN_U8, SND_SOC_TPLG_TUPLE_TYPE_BYTE, offset_of!(avs_tplg_binding, target_mod_pin), avs_parse_byte_token),
    parser!(AVS_TKN_BINDING_MOD_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_binding, mod_id), avs_parse_word_token),
    parser!(AVS_TKN_BINDING_MOD_PIN_U8, SND_SOC_TPLG_TUPLE_TYPE_BYTE, offset_of!(avs_tplg_binding, mod_pin), avs_parse_byte_token),
    parser!(AVS_TKN_BINDING_IS_SINK_U8, SND_SOC_TPLG_TUPLE_TYPE_BYTE, offset_of!(avs_tplg_binding, is_sink), avs_parse_byte_token),
];

unsafe fn avs_tplg_parse_bindings(comp: *mut snd_soc_component, tuples: *mut snd_soc_tplg_vendor_array, block_size: u32) -> c_int {
    let tplg = (*to_avs_soc_component(comp)).tplg;
    parse_dictionary(comp, tuples, block_size, &mut (*tplg).bindings as *mut _ as *mut *mut c_void, &mut (*tplg).num_bindings, size_of::<avs_tplg_binding>(), AVS_TKN_MANIFEST_NUM_BINDINGS_U32, AVS_TKN_BINDING_ID_U32, binding_parsers.as_ptr(), binding_parsers.len())
}

static module_parsers: [avs_tplg_token_parser; 9] = [
    parser!(AVS_TKN_MOD_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_module, id), avs_parse_word_token),
    parser!(AVS_TKN_MOD_MODCFG_BASE_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_module, cfg_base), avs_parse_modcfg_base_ptr),
    parser!(AVS_TKN_MOD_IN_AFMT_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_module, in_fmt), avs_parse_audio_format_ptr),
    parser!(AVS_TKN_MOD_CORE_ID_U8, SND_SOC_TPLG_TUPLE_TYPE_BYTE, offset_of!(avs_tplg_module, core_id), avs_parse_byte_token),
    parser!(AVS_TKN_MOD_PROC_DOMAIN_U8, SND_SOC_TPLG_TUPLE_TYPE_BYTE, offset_of!(avs_tplg_module, domain), avs_parse_byte_token),
    parser!(AVS_TKN_MOD_MODCFG_EXT_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_module, cfg_ext), avs_parse_modcfg_ext_ptr),
    parser!(AVS_TKN_MOD_KCONTROL_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_module, ctl_id), avs_parse_byte_token),
    parser!(AVS_TKN_MOD_INIT_CONFIG_NUM_IDS_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_module, num_config_ids), avs_parse_byte_token),
    parser!(AVS_TKN_MOD_NHLT_CONFIG_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_module, nhlt_config), avs_parse_nhlt_config_ptr),
];

static init_config_parsers: [avs_tplg_token_parser; 1] = [
    parser!(AVS_TKN_MOD_INIT_CONFIG_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, 0, avs_parse_word_token),
];

unsafe fn avs_tplg_module_create(comp: *mut snd_soc_component, owner: *mut avs_tplg_pipeline, mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32) -> *mut avs_tplg_module {
    let mut esize = 0;
    let mut ret = avs_tplg_vendor_entry_size(tuples, block_size, AVS_TKN_MOD_INIT_CONFIG_ID_U32, &mut esize);
    if ret != 0 { return ERR_PTR(ret); }
    let module = devm_kzalloc((*(*comp).card).dev, size_of::<avs_tplg_module>(), GFP_KERNEL) as *mut avs_tplg_module;
    if module.is_null() { return ERR_PTR(-ENOMEM); }
    ret = avs_parse_tokens(comp, module as *mut c_void, module_parsers.as_ptr(), module_parsers.len(), tuples, esize as c_int);
    if ret < 0 { return ERR_PTR(ret); }
    block_size -= esize;
    if block_size != 0 {
        let num_config_ids = (*module).num_config_ids as u32;
        if num_config_ids == 0 { return ERR_PTR(-EINVAL); }
        let config_ids = devm_kcalloc((*(*comp).card).dev, num_config_ids as usize, size_of::<u32>(), GFP_KERNEL) as *mut u32;
        if config_ids.is_null() { return ERR_PTR(-ENOMEM); }
        tuples = avs_tplg_vendor_array_at(tuples, esize);
        ret = parse_dictionary_entries(comp, tuples, block_size, config_ids as *mut c_void, num_config_ids, size_of::<u32>(), AVS_TKN_MOD_INIT_CONFIG_ID_U32, init_config_parsers.as_ptr(), init_config_parsers.len());
        if ret != 0 { return ERR_PTR(ret); }
        (*module).config_ids = config_ids;
    }
    (*module).owner = owner;
    INIT_LIST_HEAD(&mut (*module).node);
    module
}

static pipeline_parsers: [avs_tplg_token_parser; 3] = [
    parser!(AVS_TKN_PPL_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_pipeline, id), avs_parse_word_token),
    parser!(AVS_TKN_PPL_PPLCFG_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_pipeline, cfg), avs_parse_pplcfg_ptr),
    parser!(AVS_TKN_PPL_NUM_BINDING_IDS_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_pipeline, num_bindings), avs_parse_word_token),
];

static bindings_parsers: [avs_tplg_token_parser; 1] = [
    parser!(AVS_TKN_PPL_BINDING_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, 0, avs_parse_binding_ptr),
];

unsafe fn avs_tplg_pipeline_create(comp: *mut snd_soc_component, owner: *mut avs_tplg_path, mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32) -> *mut avs_tplg_pipeline {
    let pipeline = devm_kzalloc((*(*comp).card).dev, size_of::<avs_tplg_pipeline>(), GFP_KERNEL) as *mut avs_tplg_pipeline;
    if pipeline.is_null() { return ERR_PTR(-ENOMEM); }
    (*pipeline).owner = owner;
    INIT_LIST_HEAD(&mut (*pipeline).mod_list);
    let mut offset = 0;
    let mut ret = avs_tplg_vendor_array_lookup(tuples, block_size, AVS_TKN_MOD_ID_U32, &mut offset);
    if ret == 0 && offset == 0 { ret = -EINVAL; }
    if ret != 0 { return ERR_PTR(ret); }
    ret = avs_parse_tokens(comp, pipeline as *mut c_void, pipeline_parsers.as_ptr(), pipeline_parsers.len(), tuples, offset as c_int);
    if ret < 0 { return ERR_PTR(ret); }
    block_size -= offset;
    tuples = avs_tplg_vendor_array_at(tuples, offset);
    let modblk_size;
    ret = avs_tplg_vendor_array_lookup_next(tuples, block_size, AVS_TKN_PPL_BINDING_ID_U32, &mut offset);
    if ret != 0 {
        if ret != -ENOENT { return ERR_PTR(ret); }
        if (*pipeline).num_bindings != 0 { return ERR_PTR(-EINVAL); }
        modblk_size = block_size;
    } else {
        (*pipeline).bindings = devm_kcalloc((*(*comp).card).dev, (*pipeline).num_bindings as usize, size_of::<*mut avs_tplg_binding>(), GFP_KERNEL) as *mut *mut avs_tplg_binding;
        if (*pipeline).bindings.is_null() { return ERR_PTR(-ENOMEM); }
        modblk_size = offset;
    }
    block_size -= modblk_size;
    let mut rem = modblk_size;
    while rem > 0 {
        let mut esize = 0;
        ret = avs_tplg_vendor_entry_size(tuples, rem, AVS_TKN_MOD_ID_U32, &mut esize);
        if ret != 0 { return ERR_PTR(ret); }
        let module = avs_tplg_module_create(comp, pipeline, tuples, esize);
        if IS_ERR(module) { return ERR_CAST(module); }
        list_add_tail(&mut (*module).node, &mut (*pipeline).mod_list);
        rem -= esize;
        tuples = avs_tplg_vendor_array_at(tuples, esize);
    }
    ret = parse_dictionary_entries(comp, tuples, block_size, (*pipeline).bindings as *mut c_void, (*pipeline).num_bindings, size_of::<*mut avs_tplg_binding>(), AVS_TKN_PPL_BINDING_ID_U32, bindings_parsers.as_ptr(), bindings_parsers.len());
    if ret != 0 { return ERR_PTR(ret); }
    pipeline
}

static path_parsers: [avs_tplg_token_parser; 3] = [
    parser!(AVS_TKN_PATH_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_path, id), avs_parse_word_token),
    parser!(AVS_TKN_PATH_FE_FMT_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_path, fe_fmt), avs_parse_audio_format_ptr),
    parser!(AVS_TKN_PATH_BE_FMT_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_path, be_fmt), avs_parse_audio_format_ptr),
];

static condpath_parsers: [avs_tplg_token_parser; 3] = [
    parser!(AVS_TKN_CONDPATH_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_path, id), avs_parse_word_token),
    parser!(AVS_TKN_CONDPATH_SOURCE_PATH_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_path, source_path_id), avs_parse_word_token),
    parser!(AVS_TKN_CONDPATH_SINK_PATH_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_path, sink_path_id), avs_parse_word_token),
];

unsafe fn avs_tplg_path_create(comp: *mut snd_soc_component, owner: *mut avs_tplg_path_template, mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32, parsers: *const avs_tplg_token_parser, num_parsers: u32) -> *mut avs_tplg_path {
    let path = devm_kzalloc((*(*comp).card).dev, size_of::<avs_tplg_path>(), GFP_KERNEL) as *mut avs_tplg_path;
    if path.is_null() { return ERR_PTR(-ENOMEM); }
    (*path).owner = owner;
    INIT_LIST_HEAD(&mut (*path).ppl_list);
    INIT_LIST_HEAD(&mut (*path).node);
    let mut offset = 0;
    let ret = avs_tplg_vendor_array_lookup(tuples, block_size, AVS_TKN_PPL_ID_U32, &mut offset);
    if ret == -ENOENT { offset = block_size; } else if ret != 0 { return ERR_PTR(ret); } else if offset == 0 { return ERR_PTR(-EINVAL); }
    let ret2 = avs_parse_tokens(comp, path as *mut c_void, parsers, num_parsers as usize, tuples, offset as c_int);
    if ret2 < 0 { return ERR_PTR(ret2); }
    block_size -= offset;
    tuples = avs_tplg_vendor_array_at(tuples, offset);
    while block_size > 0 {
        let mut esize = 0;
        let ret3 = avs_tplg_vendor_entry_size(tuples, block_size, AVS_TKN_PPL_ID_U32, &mut esize);
        if ret3 != 0 { return ERR_PTR(ret3); }
        let pipeline = avs_tplg_pipeline_create(comp, path, tuples, esize);
        if IS_ERR(pipeline) { return ERR_CAST(pipeline); }
        list_add_tail(&mut (*pipeline).node, &mut (*path).ppl_list);
        block_size -= esize;
        tuples = avs_tplg_vendor_array_at(tuples, esize);
    }
    path
}

static path_tmpl_parsers: [avs_tplg_token_parser; 1] = [
    parser!(AVS_TKN_PATH_TMPL_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_path_template, id), avs_parse_word_token),
];

static condpath_tmpl_parsers: [avs_tplg_token_parser; 5] = [
    parser!(AVS_TKN_CONDPATH_TMPL_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_path_template, id), avs_parse_word_token),
    parser!(AVS_TKN_CONDPATH_TMPL_SOURCE_TPLG_NAME_STRING, SND_SOC_TPLG_TUPLE_TYPE_STRING, offset_of!(avs_tplg_path_template, source), avs_parse_string_token),
    parser!(AVS_TKN_CONDPATH_TMPL_SOURCE_PATH_TMPL_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_path_template, source) + offset_of!(avs_link_ref, id), avs_parse_word_token),
    parser!(AVS_TKN_CONDPATH_TMPL_SINK_TPLG_NAME_STRING, SND_SOC_TPLG_TUPLE_TYPE_STRING, offset_of!(avs_tplg_path_template, sink), avs_parse_string_token),
    parser!(AVS_TKN_CONDPATH_TMPL_SINK_PATH_TMPL_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_path_template, sink) + offset_of!(avs_link_ref, id), avs_parse_word_token),
];

unsafe fn parse_path_template(comp: *mut snd_soc_component, mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32, template: *mut avs_tplg_path_template, tmpl_tokens: *const avs_tplg_token_parser, num_tmpl_tokens: u32, path_tokens: *const avs_tplg_token_parser, num_path_tokens: u32) -> c_int {
    let mut offset = 0;
    let mut ret = avs_tplg_vendor_array_lookup(tuples, block_size, AVS_TKN_PATH_ID_U32, &mut offset);
    if ret != 0 { return ret; }
    ret = avs_parse_tokens(comp, template as *mut c_void, tmpl_tokens, num_tmpl_tokens as usize, tuples, offset as c_int);
    if ret < 0 { return ret; }
    block_size -= offset;
    tuples = avs_tplg_vendor_array_at(tuples, offset);
    while block_size > 0 {
        let mut esize = 0;
        ret = avs_tplg_vendor_entry_size(tuples, block_size, AVS_TKN_PATH_ID_U32, &mut esize);
        if ret != 0 { return ret; }
        let path = avs_tplg_path_create(comp, template, tuples, esize, path_tokens, num_path_tokens);
        if IS_ERR(path) { return PTR_ERR(path) as c_int; }
        list_add_tail(&mut (*path).node, &mut (*template).path_list);
        block_size -= esize;
        tuples = avs_tplg_vendor_array_at(tuples, esize);
    }
    0
}

unsafe fn avs_tplg_path_template_create(comp: *mut snd_soc_component, owner: *mut avs_tplg, tuples: *mut snd_soc_tplg_vendor_array, block_size: u32) -> *mut avs_tplg_path_template {
    let template = devm_kzalloc((*(*comp).card).dev, size_of::<avs_tplg_path_template>(), GFP_KERNEL) as *mut avs_tplg_path_template;
    if template.is_null() { return ERR_PTR(-ENOMEM); }
    (*template).owner = owner;
    INIT_LIST_HEAD(&mut (*template).path_list);
    INIT_LIST_HEAD(&mut (*template).node);
    let ret = parse_path_template(comp, tuples, block_size, template, path_tmpl_parsers.as_ptr(), path_tmpl_parsers.len() as u32, path_parsers.as_ptr(), path_parsers.len() as u32);
    if ret != 0 { return ERR_PTR(ret); }
    template
}

unsafe fn avs_tplg_parse_condpath_templates(comp: *mut snd_soc_component, mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32) -> c_int {
    let tplg = (*to_avs_soc_component(comp)).tplg;
    let mut ret = parse_dictionary_header(comp, tuples, &mut (*tplg).condpath_tmpls as *mut _ as *mut *mut c_void, &mut (*tplg).num_condpath_tmpls, size_of::<avs_tplg_path_template>(), AVS_TKN_MANIFEST_NUM_CONDPATH_TMPLS_U32);
    if ret != 0 { return ret; }
    block_size -= le32_to_cpu((*tuples).size);
    tuples = avs_tplg_vendor_array_next(tuples);
    for i in 0..(*tplg).num_condpath_tmpls {
        let template = (*tplg).condpath_tmpls.add(i as usize);
        (*template).owner = tplg;
        INIT_LIST_HEAD(&mut (*template).path_list);
        INIT_LIST_HEAD(&mut (*template).node);
        let mut esize = 0;
        ret = avs_tplg_vendor_entry_size(tuples, block_size, AVS_TKN_CONDPATH_TMPL_ID_U32, &mut esize);
        if ret != 0 { return ret; }
        ret = parse_path_template(comp, tuples, esize, template, condpath_tmpl_parsers.as_ptr(), condpath_tmpl_parsers.len() as u32, condpath_parsers.as_ptr(), condpath_parsers.len() as u32);
        if ret < 0 { return ret; }
        block_size -= esize;
        tuples = avs_tplg_vendor_array_at(tuples, esize);
    }
    0
}

static mod_init_config_parsers: [avs_tplg_token_parser; 3] = [
    parser!(AVS_TKN_INIT_CONFIG_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_init_config, id), avs_parse_word_token),
    parser!(AVS_TKN_INIT_CONFIG_PARAM_U8, SND_SOC_TPLG_TUPLE_TYPE_BYTE, offset_of!(avs_tplg_init_config, param), avs_parse_byte_token),
    parser!(AVS_TKN_INIT_CONFIG_LENGTH_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_init_config, length), avs_parse_word_token),
];

unsafe fn avs_tplg_parse_initial_configs(comp: *mut snd_soc_component, mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32, offset: *mut u32) -> c_int {
    let tplg = (*to_avs_soc_component(comp)).tplg;
    *offset = 0;
    let mut ret = parse_dictionary_header(comp, tuples, &mut (*tplg).init_configs as *mut _ as *mut *mut c_void, &mut (*tplg).num_init_configs, size_of::<avs_tplg_init_config>(), AVS_TKN_MANIFEST_NUM_INIT_CONFIGS_U32);
    if ret != 0 { return ret; }
    block_size -= le32_to_cpu((*tuples).size);
    *offset += le32_to_cpu((*tuples).size);
    tuples = avs_tplg_vendor_array_next(tuples);
    let mut i = 0;
    while i < (*tplg).num_init_configs && block_size > 0 {
        let config = (*tplg).init_configs.add(i as usize);
        let tmp = avs_tplg_vendor_array_next(tuples);
        let mut esize = le32_to_cpu((*tuples).size) + le32_to_cpu((*tmp).size);
        *offset += esize;
        ret = parse_dictionary_entries(comp, tuples, esize, config as *mut c_void, 1, size_of::<avs_tplg_init_config>(), AVS_TKN_INIT_CONFIG_ID_U32, mod_init_config_parsers.as_ptr(), mod_init_config_parsers.len());
        if ret != 0 { return ret; }
        block_size -= esize;
        let init_config_data = (tuples as *mut u8).add(esize as usize) as *mut c_void;
        esize = (*config).length;
        *offset += esize;
        (*config).data = devm_kmemdup((*(*comp).card).dev, init_config_data, esize as usize, GFP_KERNEL);
        if (*config).data.is_null() { return -ENOMEM; }
        tuples = (init_config_data as *mut u8).add(esize as usize) as *mut snd_soc_tplg_vendor_array;
        block_size -= esize;
        i += 1;
    }
    0
}

static mod_nhlt_config_parsers: [avs_tplg_token_parser; 2] = [
    parser!(AVS_TKN_NHLT_CONFIG_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_nhlt_config, id), avs_parse_word_token),
    parser!(AVS_TKN_NHLT_CONFIG_SIZE_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg_nhlt_config, blob), avs_parse_nhlt_config_size),
];

unsafe fn avs_tplg_parse_nhlt_configs(comp: *mut snd_soc_component, mut tuples: *mut snd_soc_tplg_vendor_array, mut block_size: u32) -> c_int {
    let tplg = (*to_avs_soc_component(comp)).tplg;
    let mut ret = parse_dictionary_header(comp, tuples, &mut (*tplg).nhlt_configs as *mut _ as *mut *mut c_void, &mut (*tplg).num_nhlt_configs, size_of::<avs_tplg_nhlt_config>(), AVS_TKN_MANIFEST_NUM_NHLT_CONFIGS_U32);
    if ret != 0 { return ret; }
    block_size -= le32_to_cpu((*tuples).size);
    tuples = avs_tplg_vendor_array_next(tuples);
    let mut i = 0;
    while i < (*tplg).num_nhlt_configs && block_size > 0 {
        let config = (*tplg).nhlt_configs.add(i as usize);
        let mut esize = le32_to_cpu((*tuples).size);
        ret = parse_dictionary_entries(comp, tuples, esize, config as *mut c_void, 1, size_of::<avs_tplg_nhlt_config>(), AVS_TKN_NHLT_CONFIG_ID_U32, mod_nhlt_config_parsers.as_ptr(), mod_nhlt_config_parsers.len());
        if ret != 0 { return ret; }
        if (*config).blob.is_null() { return -EINVAL; }
        memcpy((*(*config).blob).capabilities.as_mut_ptr() as *mut c_void, (tuples as *mut u8).add(esize as usize) as *const c_void, (*(*config).blob).capabilities_size as usize);
        esize += (*(*config).blob).capabilities_size;
        block_size -= esize;
        tuples = avs_tplg_vendor_array_at(tuples, esize);
        i += 1;
    }
    0
}

unsafe extern "C" fn avs_route_load(comp: *mut snd_soc_component, _: c_int, route: *mut snd_soc_dapm_route) -> c_int {
    let mach = dev_get_platdata((*(*comp).card).dev) as *mut snd_soc_acpi_mach;
    let len = SNDRV_CTL_ELEM_ID_NAME_MAXLEN;
    if !avs_mach_singular_ssp(mach) { return 0; }
    let ssp_port = avs_mach_ssp_port(mach);
    if !avs_mach_singular_tdm(mach, ssp_port) { return 0; }
    let tdm_slot = avs_mach_ssp_tdm(mach, ssp_port);
    let mut buf = devm_kzalloc((*(*comp).card).dev, len, GFP_KERNEL) as *mut c_char;
    if buf.is_null() { return -ENOMEM; }
    avs_ssp_sprint(buf, len, (*route).source, ssp_port, tdm_slot);
    (*route).source = buf;
    buf = devm_kzalloc((*(*comp).card).dev, len, GFP_KERNEL) as *mut c_char;
    if buf.is_null() { return -ENOMEM; }
    avs_ssp_sprint(buf, len, (*route).sink, ssp_port, tdm_slot);
    (*route).sink = buf;
    if !(*route).control.is_null() {
        buf = devm_kzalloc((*(*comp).card).dev, len, GFP_KERNEL) as *mut c_char;
        if buf.is_null() { return -ENOMEM; }
        avs_ssp_sprint(buf, len, (*route).control, ssp_port, tdm_slot);
        (*route).control = buf;
    }
    0
}

unsafe extern "C" fn avs_widget_load(comp: *mut snd_soc_component, _: c_int, w: *mut snd_soc_dapm_widget, dw: *mut snd_soc_tplg_dapm_widget) -> c_int {
    if le32_to_cpu((*dw).priv_.size) == 0 { return 0; }
    (*w).no_wname_in_kcontrol_name = true;
    if (*w).ignore_suspend && !AVS_S0IX_SUPPORTED { (*w).ignore_suspend = false; }
    let tplg = (*to_avs_soc_component(comp)).tplg;
    let mach = dev_get_platdata((*(*comp).card).dev) as *mut snd_soc_acpi_mach;
    if avs_mach_singular_ssp(mach) {
        let ssp_port = avs_mach_ssp_port(mach);
        if avs_mach_singular_tdm(mach, ssp_port) {
            let size = strlen((*dw).name) + 3;
            let buf = kmalloc(size, GFP_KERNEL) as *mut c_char;
            if buf.is_null() { return -ENOMEM; }
            avs_ssp_sprint(buf, size, (*dw).name, ssp_port, avs_mach_ssp_tdm(mach, ssp_port));
            kfree((*w).name as *mut c_void);
            (*w).name = buf;
        }
    }
    let template = avs_tplg_path_template_create(comp, tplg, (*dw).priv_.array, le32_to_cpu((*dw).priv_.size));
    if IS_ERR(template) { return PTR_ERR(template) as c_int; }
    (*w).priv_ = template as *mut c_void;
    list_add_tail(&mut (*template).node, &mut (*tplg).path_tmpl_list);
    0
}

unsafe extern "C" fn avs_widget_ready(_: *mut snd_soc_component, _: c_int, w: *mut snd_soc_dapm_widget, _: *mut snd_soc_tplg_dapm_widget) -> c_int {
    let template = (*w).priv_ as *mut avs_tplg_path_template;
    (*template).w = w;
    0
}

unsafe extern "C" fn avs_dai_load(_: *mut snd_soc_component, _: c_int, dai_drv: *mut snd_soc_dai_driver, pcm: *mut snd_soc_tplg_pcm, _: *mut snd_soc_dai) -> c_int {
    let fe_subformats = SNDRV_PCM_SUBFMTBIT_MSBITS_20 | SNDRV_PCM_SUBFMTBIT_MSBITS_24 | SNDRV_PCM_SUBFMTBIT_MSBITS_MAX;
    if !pcm.is_null() {
        (*dai_drv).ops = &avs_dai_fe_ops;
        (*dai_drv).capture.subformats = fe_subformats;
        (*dai_drv).playback.subformats = fe_subformats;
    }
    0
}

unsafe extern "C" fn avs_link_load(_: *mut snd_soc_component, _: c_int, link: *mut snd_soc_dai_link, _: *mut snd_soc_tplg_link_config) -> c_int {
    if (*link).ignore_suspend && !AVS_S0IX_SUPPORTED { (*link).ignore_suspend = false; }
    if !(*link).no_pcm {
        (*link).nonatomic = true;
        (*link).trigger[0] = SND_SOC_DPCM_TRIGGER_PRE;
        (*link).trigger[1] = SND_SOC_DPCM_TRIGGER_PRE;
    } else {
        (*link).dpcm_merged_format = 1;
    }
    0
}

static manifest_parsers: [avs_tplg_token_parser; 2] = [
    parser!(AVS_TKN_MANIFEST_NAME_STRING, SND_SOC_TPLG_TUPLE_TYPE_STRING, offset_of!(avs_tplg, name), parse_link_formatted_string),
    parser!(AVS_TKN_MANIFEST_VERSION_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_tplg, version), avs_parse_word_token),
];

unsafe extern "C" fn avs_manifest(comp: *mut snd_soc_component, _: c_int, manifest: *mut snd_soc_tplg_manifest) -> c_int {
    let mut tuples = (*manifest).priv_.array;
    let acomp = to_avs_soc_component(comp);
    let mut remaining = le32_to_cpu((*manifest).priv_.size);
    let mut has_init_config = true;
    let mut offset = 0;
    let mut ret = avs_tplg_vendor_array_lookup(tuples, remaining, AVS_TKN_MANIFEST_NUM_LIBRARIES_U32, &mut offset);
    if ret == 0 && offset == 0 { ret = -EINVAL; }
    if ret != 0 { return ret; }
    ret = avs_parse_tokens(comp, (*acomp).tplg as *mut c_void, manifest_parsers.as_ptr(), manifest_parsers.len(), tuples, offset as c_int);
    if ret < 0 { return ret; }
    remaining -= offset; tuples = avs_tplg_vendor_array_at(tuples, offset);
    ret = avs_tplg_vendor_array_lookup(tuples, remaining, AVS_TKN_MANIFEST_NUM_AFMTS_U32, &mut offset); if ret != 0 { return ret; }
    ret = avs_tplg_parse_libraries(comp, tuples, offset); if ret < 0 { return ret; }
    remaining -= offset; tuples = avs_tplg_vendor_array_at(tuples, offset);
    ret = avs_tplg_vendor_array_lookup(tuples, remaining, AVS_TKN_MANIFEST_NUM_MODCFGS_BASE_U32, &mut offset); if ret != 0 { return ret; }
    ret = avs_tplg_parse_audio_formats(comp, tuples, offset); if ret < 0 { return ret; }
    remaining -= offset; tuples = avs_tplg_vendor_array_at(tuples, offset);
    ret = avs_tplg_vendor_array_lookup(tuples, remaining, AVS_TKN_MANIFEST_NUM_MODCFGS_EXT_U32, &mut offset); if ret != 0 { return ret; }
    ret = avs_tplg_parse_modcfgs_base(comp, tuples, offset); if ret < 0 { return ret; }
    remaining -= offset; tuples = avs_tplg_vendor_array_at(tuples, offset);
    ret = avs_tplg_vendor_array_lookup(tuples, remaining, AVS_TKN_MANIFEST_NUM_PPLCFGS_U32, &mut offset); if ret != 0 { return ret; }
    ret = avs_tplg_parse_modcfgs_ext(comp, tuples, offset); if ret < 0 { return ret; }
    remaining -= offset; tuples = avs_tplg_vendor_array_at(tuples, offset);
    ret = avs_tplg_vendor_array_lookup(tuples, remaining, AVS_TKN_MANIFEST_NUM_BINDINGS_U32, &mut offset); if ret != 0 { return ret; }
    ret = avs_tplg_parse_pplcfgs(comp, tuples, offset); if ret < 0 { return ret; }
    remaining -= offset; tuples = avs_tplg_vendor_array_at(tuples, offset);
    ret = avs_tplg_vendor_array_lookup(tuples, remaining, AVS_TKN_MANIFEST_NUM_CONDPATH_TMPLS_U32, &mut offset); if ret != 0 { return ret; }
    ret = avs_tplg_parse_bindings(comp, tuples, offset); if ret < 0 { return ret; }
    remaining -= offset; tuples = avs_tplg_vendor_array_at(tuples, offset);
    ret = avs_tplg_vendor_array_lookup(tuples, remaining, AVS_TKN_MANIFEST_NUM_INIT_CONFIGS_U32, &mut offset);
    if ret == -ENOENT { has_init_config = false; } else if ret != 0 { return ret; }
    ret = avs_tplg_parse_condpath_templates(comp, tuples, if has_init_config { offset } else { remaining });
    if ret < 0 { return ret; }
    if !has_init_config { return 0; }
    remaining -= offset; tuples = avs_tplg_vendor_array_at(tuples, offset);
    ret = avs_tplg_parse_initial_configs(comp, tuples, remaining, &mut offset); if ret < 0 { return ret; }
    remaining -= offset; tuples = avs_tplg_vendor_array_at(tuples, offset);
    ret = avs_tplg_vendor_array_lookup(tuples, remaining, AVS_TKN_MANIFEST_NUM_NHLT_CONFIGS_U32, &mut offset);
    if ret == -ENOENT { return 0; }
    if ret != 0 { return ret; }
    tuples = avs_tplg_vendor_array_at(tuples, offset);
    avs_tplg_parse_nhlt_configs(comp, tuples, remaining)
}

static avs_control_ops: [snd_soc_tplg_kcontrol_ops; 2] = [
    snd_soc_tplg_kcontrol_ops { id: AVS_CONTROL_OPS_VOLUME, get: Some(avs_control_volume_get), put: Some(avs_control_volume_put), info: Some(avs_control_volume_info) },
    snd_soc_tplg_kcontrol_ops { id: AVS_CONTROL_OPS_MUTE, get: Some(avs_control_mute_get), put: Some(avs_control_mute_put), info: Some(avs_control_mute_info) },
];

static control_parsers: [avs_tplg_token_parser; 1] = [
    parser!(AVS_TKN_KCONTROL_ID_U32, SND_SOC_TPLG_TUPLE_TYPE_WORD, offset_of!(avs_control_data, id), avs_parse_word_token),
];

unsafe extern "C" fn avs_control_load(comp: *mut snd_soc_component, _: c_int, ctmpl: *mut snd_kcontrol_new, hdr: *mut snd_soc_tplg_ctl_hdr) -> c_int {
    match le32_to_cpu((*hdr).type_) {
        SND_SOC_TPLG_TYPE_MIXER => {}
        _ => return -EINVAL,
    }
    let mc = (*ctmpl).private_value as *mut soc_mixer_control;
    let tmc = hdr as *mut snd_soc_tplg_mixer_control;
    let tuples = (*tmc).priv_.array;
    let block_size = le32_to_cpu((*tmc).priv_.size);
    let ctl_data = devm_kzalloc((*(*comp).card).dev, size_of::<avs_control_data>(), GFP_KERNEL) as *mut avs_control_data;
    if ctl_data.is_null() { return -ENOMEM; }
    let ret = parse_dictionary_entries(comp, tuples, block_size, ctl_data as *mut c_void, 1, size_of::<avs_control_data>(), AVS_TKN_KCONTROL_ID_U32, control_parsers.as_ptr(), control_parsers.len());
    if ret != 0 { return ret; }
    (*mc).dobj.private = ctl_data as *mut c_void;
    if (*tmc).invert != 0 {
        (*ctl_data).values[0] = (*mc).max;
        for i in 1..(*mc).num_channels as usize { (*ctl_data).values[i] = (*mc).max; }
    }
    0
}

static avs_tplg_ops: snd_soc_tplg_ops = snd_soc_tplg_ops {
    io_ops: avs_control_ops.as_ptr(),
    io_ops_count: avs_control_ops.len() as c_int,
    control_load: Some(avs_control_load),
    dapm_route_load: Some(avs_route_load),
    widget_load: Some(avs_widget_load),
    widget_ready: Some(avs_widget_ready),
    dai_load: Some(avs_dai_load),
    link_load: Some(avs_link_load),
    manifest: Some(avs_manifest),
};

#[no_mangle]
pub unsafe extern "C" fn avs_tplg_new(comp: *mut snd_soc_component) -> *mut avs_tplg {
    let tplg = devm_kzalloc((*(*comp).card).dev, size_of::<avs_tplg>(), GFP_KERNEL) as *mut avs_tplg;
    if tplg.is_null() { return ptr::null_mut(); }
    (*tplg).comp = comp;
    INIT_LIST_HEAD(&mut (*tplg).path_tmpl_list);
    tplg
}

#[no_mangle]
pub unsafe extern "C" fn avs_load_topology(comp: *mut snd_soc_component, filename: *const c_char) -> c_int {
    let mut fw: *const firmware = ptr::null();
    let mut ret = request_firmware(&mut fw, filename, (*comp).dev);
    if ret < 0 { return ret; }
    ret = snd_soc_tplg_component_load(comp, &avs_tplg_ops, fw);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn avs_remove_topology(comp: *mut snd_soc_component) -> c_int {
    snd_soc_tplg_component_remove(comp);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
