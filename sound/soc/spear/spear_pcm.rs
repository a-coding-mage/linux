// SPDX-License-Identifier: GPL-2.0
/*
 * ALSA PCM interface for ST SPEAr Processors
 *
 * sound/soc/spear/spear_pcm.c
 *
 * Copyright (C) 2012 ST Microelectronics
 * Rajeev Kumar<rajeevkumar.linux@gmail.com>
 */

/* Dependencies from the original C includes:
 * linux/module.h, linux/dmaengine.h, linux/platform_device.h,
 * sound/dmaengine_pcm.h, sound/pcm.h, sound/soc.h, sound/spear_dma.h,
 * and "spear_pcm.h".
 */

extern "C" {
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *mut snd_dmaengine_pcm_config,
        flags: u32,
    ) -> i32;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
    pub fifo_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dmaengine_pcm_config {
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prealloc_buffer_size: usize,
    pub compat_filter_fn: Option<unsafe extern "C" fn(chan: *mut dma_chan, slave: *mut core::ffi::c_void) -> bool>,
}

extern "C" {
    static SNDRV_PCM_INFO_INTERLEAVED: u32;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: u32;
    static SNDRV_PCM_INFO_MMAP: u32;
    static SNDRV_PCM_INFO_MMAP_VALID: u32;
    static SNDRV_PCM_INFO_PAUSE: u32;
    static SNDRV_PCM_INFO_RESUME: u32;
    static SND_DMAENGINE_PCM_FLAG_NO_DT: u32;
    static SND_DMAENGINE_PCM_FLAG_COMPAT: u32;
}

static spear_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe {
        SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_BLOCK_TRANSFER
            | SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_PAUSE
            | SNDRV_PCM_INFO_RESUME
    },
    buffer_bytes_max: 16 * 1024, /* max buffer size */
    period_bytes_min: 2 * 1024, /* 1 msec data minimum period size */
    period_bytes_max: 2 * 1024, /* maximum period size */
    periods_min: 1, /* min # periods */
    periods_max: 8, /* max # of periods */
    fifo_size: 0, /* fifo size in bytes */
};

static spear_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &spear_pcm_hardware,
    prealloc_buffer_size: 16 * 1024,
    compat_filter_fn: None,
};

#[no_mangle]
pub unsafe extern "C" fn devm_spear_pcm_platform_register(
    dev: *mut device,
    config: *mut snd_dmaengine_pcm_config,
    filter: Option<unsafe extern "C" fn(chan: *mut dma_chan, slave: *mut core::ffi::c_void) -> bool>,
) -> i32 {
    *config = spear_dmaengine_pcm_config;
    (*config).compat_filter_fn = filter;

    devm_snd_dmaengine_pcm_register(
        dev,
        config,
        SND_DMAENGINE_PCM_FLAG_NO_DT | SND_DMAENGINE_PCM_FLAG_COMPAT,
    )
}

/* EXPORT_SYMBOL_GPL(devm_spear_pcm_platform_register); */

/* MODULE_AUTHOR("Rajeev Kumar <rajeevkumar.linux@gmail.com>"); */
/* MODULE_DESCRIPTION("SPEAr PCM DMA module"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
