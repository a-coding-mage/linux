/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2021 Intel Corporation
 */

use core::ffi::{c_int, c_void};

/* Depends on declarations from "sof-priv.h". */

/* IPC3 specific ops */
extern "C" {
    pub static ipc3_pcm_ops: sof_ipc_pcm_ops;
    pub static ipc3_tplg_ops: sof_ipc_tplg_ops;
    pub static tplg_ipc3_control_ops: sof_ipc_tplg_control_ops;
    pub static ipc3_loader_ops: sof_ipc_fw_loader_ops;
    pub static ipc3_dtrace_ops: sof_ipc_fw_tracing_ops;
}

extern "C" {
    /* helpers for fw_ready and ext_manifest parsing */
    pub fn sof_ipc3_get_ext_windows(
        sdev: *mut snd_sof_dev,
        ext_hdr: *const sof_ipc_ext_data_hdr,
    ) -> c_int;
    pub fn sof_ipc3_get_cc_info(
        sdev: *mut snd_sof_dev,
        ext_hdr: *const sof_ipc_ext_data_hdr,
    ) -> c_int;
    pub fn sof_ipc3_validate_fw_version(sdev: *mut snd_sof_dev) -> c_int;

    /* dtrace position update */
    pub fn ipc3_dtrace_posn_update(
        sdev: *mut snd_sof_dev,
        posn: *mut sof_ipc_dma_trace_posn,
    ) -> c_int;

    /* RX handler backend */
    pub fn sof_ipc3_do_rx_work(
        sdev: *mut snd_sof_dev,
        hdr: *mut sof_ipc_cmd_hdr,
        msg_buf: *mut c_void,
    );
}

/* dtrace platform callback wrappers */
#[inline]
pub unsafe fn sof_dtrace_host_init(
    sdev: *mut snd_sof_dev,
    dmatb: *mut snd_dma_buffer,
    dtrace_params: *mut sof_ipc_dma_trace_params_ext,
) -> c_int {
    let dsp_ops: *const snd_sof_dsp_ops = (*(*(*sdev).pdata).desc).ops;

    if let Some(trace_init) = (*dsp_ops).trace_init {
        return trace_init(sdev, dmatb, dtrace_params);
    }

    0
}

#[inline]
pub unsafe fn sof_dtrace_host_release(sdev: *mut snd_sof_dev) -> c_int {
    let dsp_ops: *const snd_sof_dsp_ops = (*(*(*sdev).pdata).desc).ops;

    if let Some(trace_release) = (*dsp_ops).trace_release {
        return trace_release(sdev);
    }

    0
}

#[inline]
pub unsafe fn sof_dtrace_host_trigger(sdev: *mut snd_sof_dev, cmd: c_int) -> c_int {
    let dsp_ops: *const snd_sof_dsp_ops = (*(*(*sdev).pdata).desc).ops;

    if let Some(trace_trigger) = (*dsp_ops).trace_trigger {
        return trace_trigger(sdev, cmd);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
