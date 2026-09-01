/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2022 Intel Corporation
 */

// Includes from the C header:
// <linux/idr.h>
// <sound/sof/ext_manifest4.h>
// <sound/sof/ipc4/header.h>
// "sof-priv.h"

use core::ffi::{c_char, c_void};

/* The DSP window indices are fixed */
pub const SOF_IPC4_INBOX_WINDOW_IDX: u32 = 0;
pub const SOF_IPC4_OUTBOX_WINDOW_IDX: u32 = 1;
pub const SOF_IPC4_DEBUG_WINDOW_IDX: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sof_ipc4_mtrace_type {
    SOF_IPC4_MTRACE_NOT_AVAILABLE = 0,
    SOF_IPC4_MTRACE_INTEL_CAVS_1_5 = 1,
    SOF_IPC4_MTRACE_INTEL_CAVS_1_8 = 2,
    SOF_IPC4_MTRACE_INTEL_CAVS_2 = 3,
}

/**
 * struct sof_ipc4_fw_module - IPC4 module info
 * @sof_man4_module: Module info
 * @fw_mod_cfg: Pointer to the module config start of the module
 * @m_ida: Module instance identifier
 * @private: Module private data
 */
#[repr(C)]
pub struct sof_ipc4_fw_module {
    pub man4_module_entry: sof_man4_module,
    pub fw_mod_cfg: *const sof_man4_module_config,
    pub m_ida: ida,
    pub private: *mut c_void,
}

/**
 * struct sof_ipc4_fw_library - IPC4 library information
 * @sof_fw: SOF Firmware of the library
 * @id: Library ID. 0 is reserved for basefw, external libraries must have unique
 *	ID number between 1 and (sof_ipc4_fw_data.max_libs_count - 1)
 *	Note: sof_ipc4_fw_data.max_libs_count == 1 implies that external libraries
 *	are not supported
 * @num_modules : Number of FW modules in the library
 * @modules: Array of FW modules
 */
#[repr(C)]
pub struct sof_ipc4_fw_library {
    pub sof_fw: sof_firmware,
    pub name: *const c_char,
    pub id: u32,
    pub num_modules: i32,
    pub modules: *mut sof_ipc4_fw_module,
}

/**
 * struct sof_ipc4_fw_data - IPC4-specific data
 * @manifest_fw_hdr_offset: FW header offset in the manifest
 * @fw_lib_xa: XArray for firmware libraries, including basefw (ID = 0)
 *	       Used to store the FW libraries and to manage the unique IDs of the
 *	       libraries.
 * @nhlt: NHLT table either from the BIOS or the topology manifest
 * @mtrace_type: mtrace type supported on the booted platform
 * @mtrace_log_bytes: log bytes as reported by the firmware via fw_config reply
 * @num_playback_streams: max number of playback DMAs, needed for CHAIN_DMA offset
 * @num_capture_streams: max number of capture DMAs
 * @max_num_pipelines: max number of pipelines
 * @max_libs_count: Maximum number of libraries support by the FW including the
 *		    base firmware
 * @fw_context_save: Firmware supports full context save and restore
 * @libraries_restored: The libraries have been retained during firmware boot
 *
 * @load_library: Callback function for platform dependent library loading
 * @pipeline_state_mutex: Mutex to protect pipeline triggers, ref counts, states and deletion
 */
#[repr(C)]
pub struct sof_ipc4_fw_data {
    pub manifest_fw_hdr_offset: u32,
    pub fw_lib_xa: xarray,
    pub nhlt: *mut c_void,
    pub mtrace_type: sof_ipc4_mtrace_type,
    pub mtrace_log_bytes: u32,
    pub num_playback_streams: i32,
    pub num_capture_streams: i32,
    pub max_num_pipelines: i32,
    pub max_libs_count: u32,
    pub fw_context_save: bool,
    pub libraries_restored: bool,

    pub load_library: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            fw_lib: *mut sof_ipc4_fw_library,
            reload: bool,
        ) -> i32,
    >,
    pub intel_configure_mic_privacy: Option<
        unsafe extern "C" fn(
            sdev: *mut snd_sof_dev,
            caps: *mut sof_ipc4_intel_mic_privacy_cap,
        ),
    >,
    pub pipeline_state_mutex: mutex, /* protect pipeline triggers, ref counts and states */
}

extern "C" {
    pub static ipc4_loader_ops: sof_ipc_fw_loader_ops;
    pub static ipc4_tplg_ops: sof_ipc_tplg_ops;
    pub static tplg_ipc4_control_ops: sof_ipc_tplg_control_ops;
    pub static ipc4_pcm_ops: sof_ipc_pcm_ops;
    pub static ipc4_mtrace_ops: sof_ipc_fw_tracing_ops;

    pub fn sof_ipc4_set_pipeline_state(
        sdev: *mut snd_sof_dev,
        instance_id: u32,
        state: u32,
    ) -> i32;
    pub fn sof_ipc4_mtrace_update_pos(sdev: *mut snd_sof_dev, core: i32) -> i32;

    pub fn sof_ipc4_complete_split_release(sdev: *mut snd_sof_dev) -> i32;
    pub fn sof_ipc4_query_fw_configuration(sdev: *mut snd_sof_dev) -> i32;
    pub fn sof_ipc4_reload_fw_libraries(sdev: *mut snd_sof_dev) -> i32;
    pub fn sof_ipc4_find_module_by_uuid(
        sdev: *mut snd_sof_dev,
        uuid: *const guid_t,
    ) -> *mut sof_ipc4_fw_module;

    pub fn sof_ipc4_find_swidget_by_ids(
        sdev: *mut snd_sof_dev,
        module_id: u32,
        instance_id: i32,
    ) -> *mut snd_sof_widget;

    pub fn sof_ipc4_update_cpc_from_manifest(
        sdev: *mut snd_sof_dev,
        fw_module: *mut sof_ipc4_fw_module,
        basecfg: *mut sof_ipc4_base_module_cfg,
    );

    pub fn sof_ipc4_find_debug_slot_offset_by_type(
        sdev: *mut snd_sof_dev,
        slot_type: u32,
    ) -> usize;

    pub fn sof_ipc4_mic_privacy_state_change(sdev: *mut snd_sof_dev, state: bool);

    pub fn sof_ipc4_pipeline_state_str(state: sof_ipc4_pipeline_state) -> *const c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
