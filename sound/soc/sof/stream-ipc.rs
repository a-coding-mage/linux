// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2019 Intel Corporation
//
// Authors: Guennadi Liakhovetski <guennadi.liakhovetski@linux.intel.com>

/* Generic SOF IPC code */

/* Rust translation of dependencies from:
 * linux/device.h, linux/export.h, linux/module.h, linux/types.h,
 * sound/pcm.h, sound/sof/stream.h, ops.h, sof-priv.h, sof-audio.h
 */

use core::ffi::{c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct sof_stream {
    pub posn_offset: usize,
}

/* Mailbox-based Generic IPC implementation */
#[no_mangle]
pub unsafe extern "C" fn sof_ipc_msg_data(
    sdev: *mut snd_sof_dev,
    sps: *mut snd_sof_pcm_stream,
    p: *mut c_void,
    sz: usize,
) -> c_int {
    if sps.is_null() || (*sdev).stream_box.size == 0 {
        snd_sof_dsp_mailbox_read(sdev, (*sdev).dsp_box.offset, p, sz);
    } else {
        let posn_offset: usize;

        if !(*sps).substream.is_null() {
            let stream =
                (*(*(*sps).substream).runtime).private_data as *mut sof_stream;

            /* The stream might already be closed */
            if stream.is_null() {
                return -ESTRPIPE;
            }

            posn_offset = (*stream).posn_offset;
        } else if !(*sps).cstream.is_null() {
            let sstream =
                (*(*(*sps).cstream).runtime).private_data as *mut sof_compr_stream;

            if sstream.is_null() {
                return -ESTRPIPE;
            }

            posn_offset = (*sstream).posn_offset;
        } else {
            dev_err!(
                (*sdev).dev,
                "%s: No stream opened\n",
                "sof_ipc_msg_data\0".as_ptr()
            );
            return -EINVAL;
        }

        snd_sof_dsp_mailbox_read(sdev, posn_offset, p, sz);
    }

    0
}
/* EXPORT_SYMBOL(sof_ipc_msg_data); */

#[no_mangle]
pub unsafe extern "C" fn sof_set_stream_data_offset(
    sdev: *mut snd_sof_dev,
    sps: *mut snd_sof_pcm_stream,
    mut posn_offset: usize,
) -> c_int {
    /* check if offset is overflow or it is not aligned */
    if posn_offset > (*sdev).stream_box.size
        || posn_offset % core::mem::size_of::<sof_ipc_stream_posn>() != 0
    {
        return -EINVAL;
    }

    posn_offset += (*sdev).stream_box.offset;

    if !(*sps).substream.is_null() {
        let stream =
            (*(*(*sps).substream).runtime).private_data as *mut sof_stream;

        (*stream).posn_offset = posn_offset;
        dev_dbg!(
            (*sdev).dev,
            "pcm: stream dir %d, posn mailbox offset is %zu",
            (*(*sps).substream).stream,
            posn_offset
        );
    } else if !(*sps).cstream.is_null() {
        let sstream =
            (*(*(*sps).cstream).runtime).private_data as *mut sof_compr_stream;

        (*sstream).posn_offset = posn_offset;
        dev_dbg!(
            (*sdev).dev,
            "compr: stream dir %d, posn mailbox offset is %zu",
            (*(*sps).cstream).direction,
            posn_offset
        );
    } else {
        dev_err!((*sdev).dev, "No stream opened");
        return -EINVAL;
    }

    0
}
/* EXPORT_SYMBOL(sof_set_stream_data_offset); */

#[no_mangle]
pub unsafe extern "C" fn sof_stream_pcm_open(
    _sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let stream: *mut sof_stream = kmalloc_obj::<sof_stream>();

    if stream.is_null() {
        return -ENOMEM;
    }

    /* binding pcm substream to hda stream */
    (*(*substream).runtime).private_data = stream as *mut c_void;

    /* align to DMA minimum transfer size */
    snd_pcm_hw_constraint_step(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_PERIOD_BYTES,
        4,
    );

    /* avoid circular buffer wrap in middle of period */
    snd_pcm_hw_constraint_integer(
        (*substream).runtime,
        SNDRV_PCM_HW_PARAM_PERIODS,
    );

    0
}
/* EXPORT_SYMBOL(sof_stream_pcm_open); */

#[no_mangle]
pub unsafe extern "C" fn sof_stream_pcm_close(
    _sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let stream = (*(*substream).runtime).private_data as *mut sof_stream;

    (*(*substream).runtime).private_data = ptr::null_mut();
    kfree(stream as *mut c_void);

    0
}
/* EXPORT_SYMBOL(sof_stream_pcm_close); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
