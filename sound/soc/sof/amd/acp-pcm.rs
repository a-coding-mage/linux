// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>

/*
 * PCM interface for generic AMD audio ACP DSP block
 */

/* C includes translated as dependency intent:
 * <sound/pcm_params.h>
 * "../ops.h"
 * "acp.h"
 * "acp-dsp-offset.h"
 */

extern "C" {
    fn acp_dsp_stream_config(
        sdev: *mut snd_sof_dev,
        stream: *mut acp_dsp_stream,
    ) -> core::ffi::c_int;
    fn acp_dsp_stream_get(
        sdev: *mut snd_sof_dev,
        stream_tag: core::ffi::c_int,
    ) -> *mut acp_dsp_stream;
    fn acp_dsp_stream_put(
        sdev: *mut snd_sof_dev,
        stream: *mut acp_dsp_stream,
    ) -> core::ffi::c_int;
    fn snd_sof_dsp_write(
        sdev: *mut snd_sof_dev,
        bar: core::ffi::c_int,
        offset: core::ffi::c_uint,
        value: u32,
    );
    fn snd_soc_substream_to_rtd(
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_soc_pcm_runtime;
    fn snd_sof_find_spcm_dai(
        scomp: *mut snd_soc_component,
        rtd: *mut snd_soc_pcm_runtime,
    ) -> *mut snd_sof_pcm;
    fn snd_sof_ipc_msg_data(
        sdev: *mut snd_sof_dev,
        stream: *mut snd_sof_pcm_stream,
        data: *mut core::ffi::c_void,
        size: usize,
    ) -> core::ffi::c_int;
    fn bytes_to_frames(
        runtime: *mut snd_pcm_runtime,
        bytes: snd_pcm_uframes_t,
    ) -> snd_pcm_uframes_t;
}

extern "C" {
    static ACP_DSP_BAR: core::ffi::c_int;
    static ACP_SCRATCH_REG_0: core::ffi::c_uint;
    static ENODEV: core::ffi::c_int;
    static EINVAL: core::ffi::c_int;
}

extern "C" {
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_warn_ratelimited(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

pub type snd_pcm_uframes_t = core::ffi::c_ulong;

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
    pub debug_box: snd_sof_debug_box,
}

#[repr(C)]
pub struct snd_sof_debug_box {
    pub offset: core::ffi::c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: core::ffi::c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_bytes: u32,
    pub private_data: *mut core::ffi::c_void,
    pub dma_buffer_p: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct acp_dsp_stream {
    pub num_pages: core::ffi::c_uint,
    pub dmab: *mut core::ffi::c_void,
    pub reg_offset: u64,
    pub stream_tag: core::ffi::c_uint,
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_sof_platform_stream_params {
    pub use_phy_address: bool,
    pub phy_addr: u64,
    pub stream_tag: core::ffi::c_uint,
    pub cont_update_posn: core::ffi::c_int,
}

#[repr(C)]
pub struct scratch_reg_conf {
    pub buf_size: u32,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub id: core::ffi::c_int,
}

#[repr(C)]
pub struct snd_sof_pcm {
    pub stream: *mut snd_sof_pcm_stream,
}

#[repr(C)]
pub struct snd_sof_pcm_stream {
    pub posn: sof_ipc_stream_posn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_stream_posn {
    pub host_posn: snd_pcm_uframes_t,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[inline]
unsafe fn PFN_UP(size: u32) -> core::ffi::c_uint {
    ((size as core::ffi::c_ulong)
        .wrapping_add(PAGE_SIZE as core::ffi::c_ulong)
        .wrapping_sub(1)
        >> PAGE_SHIFT) as core::ffi::c_uint
}

extern "C" {
    static PAGE_SIZE: core::ffi::c_ulong;
    static PAGE_SHIFT: core::ffi::c_ulong;
}

#[no_mangle]
pub unsafe extern "C" fn acp_pcm_hw_params(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
    platform_params: *mut snd_sof_platform_stream_params,
) -> core::ffi::c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let stream: *mut acp_dsp_stream = (*runtime).private_data as *mut acp_dsp_stream;
    let mut buf_offset: core::ffi::c_uint;
    let index: core::ffi::c_uint;
    let size: u32;
    let ret: core::ffi::c_int;

    size = (*runtime).dma_bytes;
    (*stream).num_pages = PFN_UP((*runtime).dma_bytes);
    (*stream).dmab = (*(*substream).runtime).dma_buffer_p;

    ret = acp_dsp_stream_config(sdev, stream);
    if ret < 0 {
        dev_err((*sdev).dev, c"stream configuration failed\n".as_ptr());
        return ret;
    }

    (*platform_params).use_phy_address = true;
    (*platform_params).phy_addr = (*stream).reg_offset;
    (*platform_params).stream_tag = (*stream).stream_tag;
    (*platform_params).cont_update_posn = 1;

    /* write buffer size of stream in scratch memory */

    buf_offset = (*sdev).debug_box.offset
        + core::mem::offset_of!(scratch_reg_conf, buf_size) as core::ffi::c_uint;
    index = (*stream).stream_tag - 1;
    buf_offset = buf_offset + index * 4;

    snd_sof_dsp_write(
        sdev,
        ACP_DSP_BAR,
        ACP_SCRATCH_REG_0 + buf_offset,
        size,
    );

    return 0;
}
/* EXPORT_SYMBOL_NS(acp_pcm_hw_params, "SND_SOC_SOF_AMD_COMMON"); */

#[no_mangle]
pub unsafe extern "C" fn acp_pcm_open(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let stream: *mut acp_dsp_stream;

    stream = acp_dsp_stream_get(sdev, 0);
    if stream.is_null() {
        return -ENODEV;
    }

    (*(*substream).runtime).private_data = stream as *mut core::ffi::c_void;
    (*stream).substream = substream;

    return 0;
}
/* EXPORT_SYMBOL_NS(acp_pcm_open, "SND_SOC_SOF_AMD_COMMON"); */

#[no_mangle]
pub unsafe extern "C" fn acp_pcm_close(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let stream: *mut acp_dsp_stream;

    stream = (*(*substream).runtime).private_data as *mut acp_dsp_stream;
    if stream.is_null() {
        dev_err((*sdev).dev, c"No open stream\n".as_ptr());
        return -EINVAL;
    }

    (*stream).substream = core::ptr::null_mut();
    (*(*substream).runtime).private_data = core::ptr::null_mut();

    return acp_dsp_stream_put(sdev, stream);
}
/* EXPORT_SYMBOL_NS(acp_pcm_close, "SND_SOC_SOF_AMD_COMMON"); */

#[no_mangle]
pub unsafe extern "C" fn acp_pcm_pointer(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let scomp: *mut snd_soc_component = (*sdev).component;
    let stream: *mut snd_sof_pcm_stream;
    let mut posn: sof_ipc_stream_posn = core::mem::zeroed();
    let spcm: *mut snd_sof_pcm;
    let mut pos: snd_pcm_uframes_t;
    let ret: core::ffi::c_int;

    spcm = snd_sof_find_spcm_dai(scomp, rtd);
    if spcm.is_null() {
        dev_warn_ratelimited(
            (*sdev).dev,
            c"warn: can't find PCM with DAI ID %d\n".as_ptr(),
            (*(*rtd).dai_link).id,
        );
        return 0;
    }

    stream = (*spcm).stream.add((*substream).stream as usize);
    ret = snd_sof_ipc_msg_data(
        sdev,
        stream,
        &mut posn as *mut sof_ipc_stream_posn as *mut core::ffi::c_void,
        core::mem::size_of_val(&posn),
    );
    if ret < 0 {
        dev_warn(
            (*sdev).dev,
            c"failed to read stream position: %d\n".as_ptr(),
            ret,
        );
        return 0;
    }

    core::ptr::copy_nonoverlapping(
        &posn as *const sof_ipc_stream_posn,
        &mut (*stream).posn as *mut sof_ipc_stream_posn,
        1,
    );
    pos = (*(*spcm).stream.add((*substream).stream as usize)).posn.host_posn;
    pos = bytes_to_frames((*substream).runtime, pos);

    return pos;
}
/* EXPORT_SYMBOL_NS(acp_pcm_pointer, "SND_SOC_SOF_AMD_COMMON"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
