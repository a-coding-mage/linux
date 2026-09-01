/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2021 Intel Corporation
 *
 * Authors: Cezary Rojewski <cezary.rojewski@intel.com>
 *          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
 */

use core::ffi::{c_int, c_long};

/* Dependencies from the original C header:
 * #include <linux/list.h>
 * #include "avs.h"
 * #include "topology.h"
 */

pub const AVS_COND_TYPE_NONE: u32 = 0;
pub const AVS_COND_TYPE_AECREF: u32 = 1;

#[repr(C)]
pub struct avs_path {
    pub dma_id: u32,
    pub ppl_list: list_head,
    pub state: u32,

    /* condpath navigation for standard paths */
    pub source_list: list_head,
    pub sink_list: list_head,

    /* conditional path fields */
    pub source: *mut avs_path,
    pub sink: *mut avs_path,
    pub source_node: list_head,
    pub sink_node: list_head,

    pub template: *mut avs_tplg_path,
    pub owner: *mut avs_dev,
    /* device path management */
    pub node: list_head,
}

#[repr(C)]
pub struct avs_path_pipeline {
    pub instance_id: u8,
    pub mod_list: list_head,
    pub binding_list: list_head,

    pub template: *mut avs_tplg_pipeline,
    pub owner: *mut avs_path,
    /* path pipelines management */
    pub node: list_head,
}

#[repr(C)]
pub struct avs_path_module {
    pub module_id: u16,
    pub instance_id: u8,
    pub gtw_attrs: avs_gtw_attributes,

    pub template: *mut avs_tplg_module,
    pub owner: *mut avs_path_pipeline,
    /* pipeline modules management */
    pub node: list_head,
}

#[repr(C)]
pub struct avs_path_binding {
    pub source: *mut avs_path_module,
    pub source_pin: u8,
    pub sink: *mut avs_path_module,
    pub sink_pin: u8,

    pub template: *mut avs_tplg_binding,
    pub owner: *mut avs_path_pipeline,
    /* pipeline bindings management */
    pub node: list_head,
}

unsafe extern "C" {
    pub fn avs_path_free(path: *mut avs_path);
    pub fn avs_path_create(
        adev: *mut avs_dev,
        dma_id: u32,
        template: *mut avs_tplg_path_template,
        fe_params: *mut snd_pcm_hw_params,
        be_params: *mut snd_pcm_hw_params,
    ) -> *mut avs_path;
    pub fn avs_path_bind(path: *mut avs_path) -> c_int;
    pub fn avs_path_unbind(path: *mut avs_path) -> c_int;
    pub fn avs_path_reset(path: *mut avs_path) -> c_int;
    pub fn avs_path_pause(path: *mut avs_path) -> c_int;
    pub fn avs_path_run(path: *mut avs_path, trigger: c_int) -> c_int;

    pub fn avs_path_set_constraint(
        adev: *mut avs_dev,
        template: *mut avs_tplg_path_template,
        rate_list: *mut snd_pcm_hw_constraint_list,
        channels_list: *mut snd_pcm_hw_constraint_list,
        sample_bits_list: *mut snd_pcm_hw_constraint_list,
    ) -> c_int;

    pub fn avs_peakvol_set_volume(
        adev: *mut avs_dev,
        mod_: *mut avs_path_module,
        mc: *mut soc_mixer_control,
        input: *mut c_long,
    ) -> c_int;
    pub fn avs_peakvol_set_mute(
        adev: *mut avs_dev,
        mod_: *mut avs_path_module,
        mc: *mut soc_mixer_control,
        input: *mut c_long,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
