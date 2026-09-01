// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc. All rights reserved.
//
// Authors: Vishnuvardhanrao Ravuapati <vishnuvardhanrao.ravulapati@amd.com>
//          V Sujith Kumar Reddy <Vsujithkumar.Reddy@amd.com>

/*This file support Host TRACE Logger driver callback for SOF FW */

// C dependency: "acp.h"

const ACP_LOGGER_STREAM: i32 = 8;
const NUM_PAGES: i32 = 16;
const ENODEV: i32 = 19;

#[repr(C)]
pub struct snd_sof_dev {
    pub pdata: *mut snd_sof_pdata,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut acp_dev_data,
}

#[repr(C)]
pub struct acp_dev_data {
    pub dtrace_stream: *mut acp_dsp_stream,
}

#[repr(C)]
pub struct acp_dsp_stream {
    pub dmab: *mut snd_dma_buffer,
    pub num_pages: i32,
    pub stream_tag: u32,
    pub reg_offset: u64,
}

#[repr(C)]
pub struct snd_dma_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sof_ipc_dma_trace_params_ext {
    pub stream_tag: u32,
    pub buffer: sof_ipc_dma_trace_params_ext_buffer,
}

#[repr(C)]
pub struct sof_ipc_dma_trace_params_ext_buffer {
    pub phy_addr: u64,
}

unsafe extern "C" {
    fn acp_dsp_stream_put(sdev: *mut snd_sof_dev, stream: *mut acp_dsp_stream) -> i32;
    fn acp_dsp_stream_get(sdev: *mut snd_sof_dev, stream_tag: i32) -> *mut acp_dsp_stream;
    fn acp_dsp_stream_config(sdev: *mut snd_sof_dev, stream: *mut acp_dsp_stream) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_sof_trace_release(sdev: *mut snd_sof_dev) -> i32 {
    let stream: *mut acp_dsp_stream;
    let adata: *mut acp_dev_data;
    let ret: i32;

    unsafe {
        adata = (*(*sdev).pdata).hw_pdata;
        stream = (*adata).dtrace_stream;
        ret = acp_dsp_stream_put(sdev, stream);
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                c"Failed to release trace stream\n".as_ptr(),
            );
            return ret;
        }

        (*adata).dtrace_stream = core::ptr::null_mut();
    }
    0
}
// EXPORT_SYMBOL_NS(acp_sof_trace_release, "SND_SOC_SOF_AMD_COMMON");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_sof_trace_init(
    sdev: *mut snd_sof_dev,
    dmab: *mut snd_dma_buffer,
    dtrace_params: *mut sof_ipc_dma_trace_params_ext,
) -> i32 {
    let stream: *mut acp_dsp_stream;
    let adata: *mut acp_dev_data;
    let ret: i32;

    unsafe {
        adata = (*(*sdev).pdata).hw_pdata;
        stream = acp_dsp_stream_get(sdev, ACP_LOGGER_STREAM);
        if stream.is_null() {
            return -ENODEV;
        }

        (*stream).dmab = dmab;
        (*stream).num_pages = NUM_PAGES;

        ret = acp_dsp_stream_config(sdev, stream);
        if ret < 0 {
            acp_dsp_stream_put(sdev, stream);
            return ret;
        }

        (*adata).dtrace_stream = stream;
        (*dtrace_params).stream_tag = (*stream).stream_tag;
        (*dtrace_params).buffer.phy_addr = (*stream).reg_offset;
    }

    0
}
// EXPORT_SYMBOL_NS(acp_sof_trace_init, "SND_SOC_SOF_AMD_COMMON");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
