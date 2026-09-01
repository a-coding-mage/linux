// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011 Freescale Semiconductor, Inc. All Rights Reserved.
 *
 * Based on sound/soc/imx/imx-pcm-dma-mx2.c
 */

// Dependencies from:
// <linux/device.h>
// <linux/init.h>
// <linux/module.h>
// <sound/core.h>
// <sound/pcm.h>
// <sound/soc.h>
// <sound/dmaengine_pcm.h>
// "mxs-pcm.h"

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub buffer_bytes_max: c_uint,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prealloc_buffer_size: c_uint,
}

extern "C" {
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
}

extern "C" {
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_PAUSE: c_uint;
    static SNDRV_PCM_INFO_RESUME: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_HALF_DUPLEX: c_uint;
    static SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX: c_uint;
}

static snd_mxs_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe {
        SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_PAUSE
            | SNDRV_PCM_INFO_RESUME
            | SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_HALF_DUPLEX
    },
    period_bytes_min: 32,
    period_bytes_max: 8192,
    periods_min: 1,
    periods_max: 52,
    buffer_bytes_max: 64 * 1024,
    fifo_size: 32,
};

static mxs_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &snd_mxs_hardware,
    prealloc_buffer_size: 64 * 1024,
};

#[no_mangle]
pub unsafe extern "C" fn mxs_pcm_platform_register(dev: *mut device) -> c_int {
    devm_snd_dmaengine_pcm_register(
        dev,
        &mxs_dmaengine_pcm_config,
        SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX,
    )
}

// EXPORT_SYMBOL_GPL(mxs_pcm_platform_register);

// MODULE_DESCRIPTION("MXS ASoC PCM driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
