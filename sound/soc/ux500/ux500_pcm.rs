// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) ST-Ericsson SA 2012
 *
 * Author: Ola Lilja <ola.o.lilja@stericsson.com>,
 *         Roger Nilsson <roger.xr.nilsson@stericsson.com>
 *         for ST-Ericsson.
 */

// Dependencies from included headers:
// #include <asm/page.h>
// #include <linux/module.h>
// #include <linux/dma-mapping.h>
// #include <linux/dmaengine.h>
// #include <linux/slab.h>
// #include <sound/pcm.h>
// #include <sound/pcm_params.h>
// #include <sound/soc.h>
// #include <sound/dmaengine_pcm.h>
// #include "ux500_msp_i2s.h"
// #include "ux500_pcm.h"

use std::ffi::c_void;

extern "C" {
    static PAGE_SIZE: usize;
}

const UX500_PLATFORM_PERIODS_BYTES_MIN: usize = 128;
// UX500_PLATFORM_PERIODS_BYTES_MAX = 64 * PAGE_SIZE (PAGE_SIZE from <asm/page.h>)
const UX500_PLATFORM_PERIODS_MIN: usize = 2;
const UX500_PLATFORM_PERIODS_MAX: usize = 48;
// UX500_PLATFORM_BUFFER_BYTES_MAX = 2048 * PAGE_SIZE (PAGE_SIZE from <asm/page.h>)

extern "C" {
    type snd_pcm_substream;
    type snd_pcm_hw_params;
    type dma_slave_config;
    type snd_dmaengine_dai_dma_data;
    type snd_soc_pcm_runtime;
    type snd_soc_dai;
    type platform_device;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, index: u32) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_dmaengine_dai_dma_data;
    fn snd_hwparams_to_dma_slave_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        slave_config: *mut dma_slave_config,
    ) -> i32;
    fn snd_dmaengine_pcm_register(
        dev: *mut c_void,
        config: *const snd_dmaengine_pcm_config,
        flags: u32,
    ) -> i32;
    fn snd_dmaengine_pcm_unregister(dev: *mut c_void);
    fn dev_err(dev: *const c_void, fmt: *const u8, ...);

    // External constants from kernel headers
    static DMA_SLAVE_BUSWIDTH_2_BYTES: u32;
    static SNDRV_PCM_STREAM_PLAYBACK: u32;
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    prepare_slave_config: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut dma_slave_config,
        ) -> i32,
    >,
}

unsafe extern "C" fn ux500_pcm_prepare_slave_config(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    slave_config: *mut dma_slave_config,
) -> i32 {
    let rtd = snd_soc_substream_to_rtd(substream);
    let snd_dma_params = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
    let dma_addr = (*snd_dma_params).addr;

    let ret = snd_hwparams_to_dma_slave_config(substream, params, slave_config);
    if ret != 0 {
        return ret;
    }

    (*slave_config).dst_maxburst = 4;
    (*slave_config).src_maxburst = 4;

    (*slave_config).src_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    (*slave_config).dst_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*slave_config).dst_addr = dma_addr;
    } else {
        (*slave_config).src_addr = dma_addr;
    }

    0
}

static UX500_DMAENGINE_OF_PCM_CONFIG: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    prepare_slave_config: Some(ux500_pcm_prepare_slave_config),
};

#[no_mangle]
pub extern "C" fn ux500_pcm_register_platform(pdev: *mut platform_device) -> i32 {
    unsafe {
        let ret = snd_dmaengine_pcm_register(
            &(*pdev).dev as *const _ as *mut c_void,
            &UX500_DMAENGINE_OF_PCM_CONFIG,
            0,
        );
        if ret < 0 {
            dev_err(
                &(*pdev).dev as *const _ as *const c_void,
                b"%s: ERROR: Failed to register platform '%s' (%d)!\n\0".as_ptr(),
                b"ux500_pcm_register_platform\0".as_ptr(),
                (*pdev).name,
                ret,
            );
            return ret;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn ux500_pcm_unregister_platform(pdev: *mut platform_device) -> i32 {
    unsafe {
        snd_dmaengine_pcm_unregister(&(*pdev).dev as *const _ as *mut c_void);
    }
    0
}

// MODULE_AUTHOR("Ola Lilja")
// MODULE_AUTHOR("Roger Nilsson")
// MODULE_DESCRIPTION("ASoC UX500 driver")
// MODULE_LICENSE("GPL v2")
// Linux kernel module metadata macros - would be provided by module build infrastructure

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
