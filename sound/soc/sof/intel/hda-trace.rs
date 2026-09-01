// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Authors: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//	    Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//	    Rander Wang <rander.wang@intel.com>
//          Keyon Jie <yang.jie@linux.intel.com>
//

/*
 * Hardware interface for generic Intel audio DSP HDA IP
 */

/* C dependencies: <sound/hdaudio_ext.h>, "../ops.h", "hda.h" */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct snd_sof_dev {
    pub pdata: *mut snd_sof_pdata,
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut sof_intel_hda_dev,
}

#[repr(C)]
pub struct sof_intel_hda_dev {
    pub dtrace_stream: *mut hdac_ext_stream,
}

#[repr(C)]
pub struct hdac_ext_stream {
    pub hstream: hdac_stream,
}

#[repr(C)]
pub struct hdac_stream {
    pub period_bytes: usize,
    pub bufsize: usize,
    pub stream_tag: c_int,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub bytes: usize,
}

#[repr(C)]
pub struct sof_ipc_dma_trace_params_ext {
    pub stream_tag: c_int,
}

unsafe extern "C" {
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SOF_HDA_STREAM_DMI_L1_COMPATIBLE: c_int;
    static ENODEV: c_int;

    fn hda_dsp_stream_hw_params(
        sdev: *mut snd_sof_dev,
        hext_stream: *mut hdac_ext_stream,
        dmab: *mut snd_dma_buffer,
        params: *mut c_void,
    ) -> c_int;
    fn hda_dsp_stream_get(
        sdev: *mut snd_sof_dev,
        direction: c_int,
        flags: c_int,
    ) -> *mut hdac_ext_stream;
    fn hda_dsp_stream_put(sdev: *mut snd_sof_dev, direction: c_int, stream_tag: c_int);
    fn hda_dsp_stream_trigger(
        sdev: *mut snd_sof_dev,
        hext_stream: *mut hdac_ext_stream,
        cmd: c_int,
    ) -> c_int;

    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
}

unsafe fn hda_dsp_trace_prepare(
    sdev: *mut snd_sof_dev,
    dmab: *mut snd_dma_buffer,
) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata;
    let hext_stream: *mut hdac_ext_stream = (*hda).dtrace_stream;
    let hstream: *mut hdac_stream = &mut (*hext_stream).hstream;
    let ret: c_int;

    (*hstream).period_bytes = 0; /* initialize period_bytes */
    (*hstream).bufsize = (*dmab).bytes;

    ret = hda_dsp_stream_hw_params(sdev, hext_stream, dmab, ptr::null_mut());
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            c"error: hdac prepare failed: %d\n".as_ptr(),
            ret,
        );
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_dsp_trace_init(
    sdev: *mut snd_sof_dev,
    dmab: *mut snd_dma_buffer,
    dtrace_params: *mut sof_ipc_dma_trace_params_ext,
) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata;
    let ret: c_int;

    (*hda).dtrace_stream = hda_dsp_stream_get(
        sdev,
        SNDRV_PCM_STREAM_CAPTURE,
        SOF_HDA_STREAM_DMI_L1_COMPATIBLE,
    );

    if (*hda).dtrace_stream.is_null() {
        dev_err(
            (*sdev).dev,
            c"error: no available capture stream for DMA trace\n".as_ptr(),
        );
        return -ENODEV;
    }

    (*dtrace_params).stream_tag = (*(*hda).dtrace_stream).hstream.stream_tag;

    /*
     * initialize capture stream, set BDL address and return corresponding
     * stream tag which will be sent to the firmware by IPC message.
     */
    ret = hda_dsp_trace_prepare(sdev, dmab);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            c"error: hdac trace init failed: %d\n".as_ptr(),
            ret,
        );
        hda_dsp_stream_put(
            sdev,
            SNDRV_PCM_STREAM_CAPTURE,
            (*dtrace_params).stream_tag,
        );
        (*hda).dtrace_stream = ptr::null_mut();
        (*dtrace_params).stream_tag = 0;
    }

    ret
}
/* EXPORT_SYMBOL_NS(hda_dsp_trace_init, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_dsp_trace_release(sdev: *mut snd_sof_dev) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata;
    let hstream: *mut hdac_stream;

    if !(*hda).dtrace_stream.is_null() {
        hstream = &mut (*(*hda).dtrace_stream).hstream;
        hda_dsp_stream_put(sdev, SNDRV_PCM_STREAM_CAPTURE, (*hstream).stream_tag);
        (*hda).dtrace_stream = ptr::null_mut();
        return 0;
    }

    dev_dbg((*sdev).dev, c"DMA trace stream is not opened!\n".as_ptr());
    -ENODEV
}
/* EXPORT_SYMBOL_NS(hda_dsp_trace_release, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hda_dsp_trace_trigger(
    sdev: *mut snd_sof_dev,
    cmd: c_int,
) -> c_int {
    let hda: *mut sof_intel_hda_dev = (*(*sdev).pdata).hw_pdata;

    hda_dsp_stream_trigger(sdev, (*hda).dtrace_stream, cmd)
}
/* EXPORT_SYMBOL_NS(hda_dsp_trace_trigger, "SND_SOC_SOF_INTEL_HDA_COMMON"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
