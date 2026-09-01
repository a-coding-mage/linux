// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/sound/arm/ep93xx-pcm.c - EP93xx ALSA PCM interface
 *
 * Copyright (C) 2006 Lennert Buytenhek <buytenh@wantstofly.org>
 * Copyright (C) 2006 Applied Data Systems
 *
 * Rewritten for the SoC audio subsystem (Based on PXA2xx code):
 *   Copyright (c) 2008 Ryan Mallon
 */

// C dependencies:
// linux/module.h, linux/init.h, linux/platform_device.h, linux/dmaengine.h
// sound/pcm.h, sound/soc.h, sound/dmaengine_pcm.h
// "ep93xx-pcm.h"

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: usize,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prealloc_buffer_size: usize,
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
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint;
}

static ep93xx_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe {
        SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_BLOCK_TRANSFER
    },
    buffer_bytes_max: 131072,
    period_bytes_min: 32,
    period_bytes_max: 32768,
    periods_min: 1,
    periods_max: 32,
    fifo_size: 32,
};

static ep93xx_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &ep93xx_pcm_hardware,
    prealloc_buffer_size: 131072,
};

#[no_mangle]
pub unsafe extern "C" fn devm_ep93xx_pcm_platform_register(dev: *mut device) -> c_int {
    devm_snd_dmaengine_pcm_register(dev, &ep93xx_dmaengine_pcm_config, 0)
}

// EXPORT_SYMBOL_GPL(devm_ep93xx_pcm_platform_register);

// MODULE_AUTHOR("Ryan Mallon");
// MODULE_DESCRIPTION("EP93xx ALSA PCM interface");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
