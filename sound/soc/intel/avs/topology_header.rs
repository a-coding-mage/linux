/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2021 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

// C dependencies:
// #include <linux/list.h>
// #include "messages.h"

use core::ffi::{c_char, c_void};

pub const INVALID_OBJECT_ID: u32 = u32::MAX;

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct acpi_nhlt_config {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct avs_tplg {
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub version: u32,
    pub comp: *mut snd_soc_component,

    pub libs: *mut avs_tplg_library,
    pub num_libs: u32,
    pub fmts: *mut avs_audio_format,
    pub num_fmts: u32,
    pub modcfgs_base: *mut avs_tplg_modcfg_base,
    pub num_modcfgs_base: u32,
    pub modcfgs_ext: *mut avs_tplg_modcfg_ext,
    pub num_modcfgs_ext: u32,
    pub pplcfgs: *mut avs_tplg_pplcfg,
    pub num_pplcfgs: u32,
    pub bindings: *mut avs_tplg_binding,
    pub num_bindings: u32,
    pub condpath_tmpls: *mut avs_tplg_path_template,
    pub num_condpath_tmpls: u32,
    pub init_configs: *mut avs_tplg_init_config,
    pub num_init_configs: u32,
    pub nhlt_configs: *mut avs_tplg_nhlt_config,
    pub num_nhlt_configs: u32,

    pub path_tmpl_list: list_head,
}

#[repr(C)]
pub struct avs_tplg_library {
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
}

/* Matches header of struct avs_mod_cfg_base. */
#[repr(C)]
pub struct avs_tplg_modcfg_base {
    pub cpc: u32,
    pub ibs: u32,
    pub obs: u32,
    pub is_pages: u32,
}

#[repr(C)]
pub struct avs_tplg_pin_format {
    pub pin_index: u32,
    pub iobs: u32,
    pub fmt: *mut avs_audio_format,
}

#[repr(C)]
pub struct avs_tplg_modcfg_ext {
    pub type_: guid_t,
    pub u: avs_tplg_modcfg_ext_union,
}

#[repr(C)]
pub union avs_tplg_modcfg_ext_union {
    pub generic: avs_tplg_modcfg_ext_generic,
    pub copier: avs_tplg_modcfg_ext_copier,
    pub whm: avs_tplg_modcfg_ext_whm,
    pub updown_mix: avs_tplg_modcfg_ext_updown_mix,
    pub src: avs_tplg_modcfg_ext_src,
    pub asrc: avs_tplg_modcfg_ext_asrc,
    pub wov: avs_tplg_modcfg_ext_wov,
    pub aec: avs_tplg_modcfg_ext_aec,
    pub mux: avs_tplg_modcfg_ext_mux,
    pub micsel: avs_tplg_modcfg_ext_micsel,
    pub peakvol: avs_tplg_modcfg_ext_peakvol,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct avs_tplg_modcfg_ext_generic {
    pub num_input_pins: u16,
    pub num_output_pins: u16,
    pub pin_fmts: *mut avs_tplg_pin_format,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct avs_tplg_modcfg_ext_copier {
    pub out_fmt: *mut avs_audio_format,
    pub blob_fmt: *mut avs_audio_format, /* optional override */
    pub feature_mask: u32,
    pub vindex: avs_virtual_index,
    pub dma_type: u32,
    pub dma_buffer_size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct avs_tplg_modcfg_ext_whm {
    pub ref_fmt: *mut avs_audio_format,
    pub out_fmt: *mut avs_audio_format,
    pub wake_tick_period: u32,
    pub vindex: avs_virtual_index,
    pub dma_type: u32,
    pub dma_buffer_size: u32,
    pub blob_fmt: *mut avs_audio_format, /* optional override */
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct avs_tplg_modcfg_ext_updown_mix {
    pub out_channel_config: u32,
    pub coefficients_select: u32,
    pub coefficients: [i32; AVS_COEFF_CHANNELS_MAX],
    pub channel_map: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct avs_tplg_modcfg_ext_src {
    pub out_freq: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct avs_tplg_modcfg_ext_asrc {
    pub out_freq: u32,
    pub mode: u8,
    pub disable_jitter_buffer: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct avs_tplg_modcfg_ext_wov {
    pub cpc_lp_mode: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct avs_tplg_modcfg_ext_aec {
    pub ref_fmt: *mut avs_audio_format,
    pub out_fmt: *mut avs_audio_format,
    pub cpc_lp_mode: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct avs_tplg_modcfg_ext_mux {
    pub ref_fmt: *mut avs_audio_format,
    pub out_fmt: *mut avs_audio_format,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct avs_tplg_modcfg_ext_micsel {
    pub out_fmt: *mut avs_audio_format,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct avs_tplg_modcfg_ext_peakvol {
    pub target_volume: u32,
    pub curve_type: u32,
    pub curve_duration: u32,
}

/* Specifies path behaviour during PCM ->trigger(START) command. */
#[repr(C)]
pub enum avs_tplg_trigger {
    AVS_TPLG_TRIGGER_AUTO = 0,
}

#[repr(C)]
pub struct avs_tplg_pplcfg {
    pub req_size: u16,
    pub priority: u8,
    pub lp: bool,
    pub attributes: u16,
    pub trigger: avs_tplg_trigger,
}

#[repr(C)]
pub struct avs_tplg_binding {
    pub target_tplg_name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub target_path_tmpl_id: u32,
    pub target_ppl_id: u32,
    pub target_mod_id: u32,
    pub target_mod_pin: u8,
    pub mod_id: u32,
    pub mod_pin: u8,
    pub is_sink: u8,
}

#[repr(C)]
pub struct avs_tplg_path_template_id {
    pub id: u32,
    pub tplg_name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
}

#[repr(C)]
pub struct avs_tplg_path_template {
    pub id: u32,

    pub w: *mut snd_soc_dapm_widget,

    /* Conditional path. */
    pub source: avs_tplg_path_template_id,
    pub sink: avs_tplg_path_template_id,

    pub path_list: list_head,

    pub owner: *mut avs_tplg,
    /* Driver path templates management. */
    pub node: list_head,
}

#[repr(C)]
pub struct avs_tplg_init_config {
    pub id: u32,

    pub param: u8,
    pub length: usize,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct avs_tplg_nhlt_config {
    pub id: u32,
    pub blob: *mut acpi_nhlt_config,
}

#[repr(C)]
pub struct avs_tplg_path {
    pub id: u32,

    /* Path format requirements. */
    pub fe_fmt: *mut avs_audio_format,
    pub be_fmt: *mut avs_audio_format,
    /* Condpath path-variant requirements. */
    pub source_path_id: u32,
    pub sink_path_id: u32,

    pub ppl_list: list_head,

    pub owner: *mut avs_tplg_path_template,
    /* Path template path-variants management. */
    pub node: list_head,
}

#[repr(C)]
pub struct avs_tplg_pipeline {
    pub id: u32,

    pub cfg: *mut avs_tplg_pplcfg,
    pub bindings: *mut *mut avs_tplg_binding,
    pub num_bindings: u32,
    pub mod_list: list_head,

    pub owner: *mut avs_tplg_path,
    /* Path pipelines management. */
    pub node: list_head,
}

#[repr(C)]
pub struct avs_tplg_module {
    pub id: u32,

    pub cfg_base: *mut avs_tplg_modcfg_base,
    pub in_fmt: *mut avs_audio_format,
    pub core_id: u8,
    pub domain: u8,
    pub cfg_ext: *mut avs_tplg_modcfg_ext,
    pub ctl_id: u32,
    pub num_config_ids: u32,
    pub config_ids: *mut u32,
    pub nhlt_config: *mut avs_tplg_nhlt_config,

    pub owner: *mut avs_tplg_pipeline,
    /* Pipeline modules management. */
    pub node: list_head,
}

unsafe extern "C" {
    pub fn avs_tplg_new(comp: *mut snd_soc_component) -> *mut avs_tplg;

    pub fn avs_load_topology(comp: *mut snd_soc_component, filename: *const c_char) -> i32;
    pub fn avs_remove_topology(comp: *mut snd_soc_component) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
