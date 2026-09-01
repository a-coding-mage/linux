/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::{c_char, c_int, c_uint, c_void};

/*
 * Forward declarations from the C header:
 * struct snd_compr_stream;
 * struct snd_compr_tstamp64;
 * struct snd_compr_params;
 * struct sof_client_dev;
 * struct snd_soc_dai;
 *
 * Additional dependency types referenced by this header:
 * struct dentry;
 * struct snd_soc_card;
 */

/*
 * Callbacks used on platforms where the control for audio is split between
 * DSP and host, like HDA.
 */
#[repr(C)]
pub struct sof_probes_host_ops {
    pub startup: Option<
        unsafe extern "C" fn(
            cdev: *mut sof_client_dev,
            cstream: *mut snd_compr_stream,
            dai: *mut snd_soc_dai,
            stream_id: *mut u32,
        ) -> c_int,
    >,
    pub shutdown: Option<
        unsafe extern "C" fn(
            cdev: *mut sof_client_dev,
            cstream: *mut snd_compr_stream,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_params: Option<
        unsafe extern "C" fn(
            cdev: *mut sof_client_dev,
            cstream: *mut snd_compr_stream,
            params: *mut snd_compr_params,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub trigger: Option<
        unsafe extern "C" fn(
            cdev: *mut sof_client_dev,
            cstream: *mut snd_compr_stream,
            cmd: c_int,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub pointer: Option<
        unsafe extern "C" fn(
            cdev: *mut sof_client_dev,
            cstream: *mut snd_compr_stream,
            tstamp: *mut snd_compr_tstamp64,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C, packed)]
pub struct sof_probe_point_desc {
    pub buffer_id: c_uint,
    pub purpose: c_uint,
    pub stream_tag: c_uint,
}

#[repr(C)]
pub enum sof_probe_info_type {
    PROBES_INFO_ACTIVE_PROBES,
    PROBES_INFO_AVAILABE_PROBES,
}

#[repr(C)]
pub struct sof_probes_ipc_ops {
    pub init: Option<
        unsafe extern "C" fn(
            cdev: *mut sof_client_dev,
            stream_tag: u32,
            buffer_size: usize,
        ) -> c_int,
    >,
    pub deinit: Option<unsafe extern "C" fn(cdev: *mut sof_client_dev) -> c_int>,
    pub points_info: Option<
        unsafe extern "C" fn(
            cdev: *mut sof_client_dev,
            desc: *mut *mut sof_probe_point_desc,
            num_desc: *mut usize,
            type_: sof_probe_info_type,
        ) -> c_int,
    >,
    pub point_print: Option<
        unsafe extern "C" fn(
            cdev: *mut sof_client_dev,
            buf: *mut c_char,
            size: usize,
            desc: *mut sof_probe_point_desc,
        ) -> c_int,
    >,
    pub points_add: Option<
        unsafe extern "C" fn(
            cdev: *mut sof_client_dev,
            desc: *mut sof_probe_point_desc,
            num_desc: usize,
        ) -> c_int,
    >,
    pub points_remove: Option<
        unsafe extern "C" fn(
            cdev: *mut sof_client_dev,
            buffer_id: *mut c_uint,
            num_buffer_id: usize,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    pub static ipc3_probe_ops: sof_probes_ipc_ops;
    pub static ipc4_probe_ops: sof_probes_ipc_ops;
}

#[repr(C)]
pub struct sof_probes_priv {
    pub dfs_points: *mut dentry,
    pub dfs_points_remove: *mut dentry,
    pub extractor_stream_tag: u32,
    pub card: snd_soc_card,
    pub ipc_priv: *mut c_void,

    pub host_ops: *const sof_probes_host_ops,
    pub ipc_ops: *const sof_probes_ipc_ops,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
