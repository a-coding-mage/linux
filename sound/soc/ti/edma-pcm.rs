// SPDX-License-Identifier: GPL-2.0-only
/*
 * edma-pcm.c - eDMA PCM driver using dmaengine for AM3xxx, AM4xxx
 *
 * Copyright (C) 2014 Texas Instruments, Inc.
 *
 * Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 *
 * Based on: sound/soc/tegra/tegra_pcm.c
 */

/* Dependencies from:
 * linux/module.h
 * sound/core.h
 * sound/pcm.h
 * sound/pcm_params.h
 * sound/soc.h
 * sound/dmaengine_pcm.h
 * edma-pcm.h
 */

extern "C" {
    static snd_dmaengine_pcm_prepare_slave_config: unsafe extern "C" fn();

    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: u32,
    ) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
}

#[repr(C)]
pub struct device {
    pub of_node: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dmaengine_pcm_config {
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prepare_slave_config: unsafe extern "C" fn(),
    pub prealloc_buffer_size: usize,
    pub chan_names: [*const i8; 2],
}

extern "C" {
    static SNDRV_PCM_INFO_MMAP: u32;
    static SNDRV_PCM_INFO_MMAP_VALID: u32;
    static SNDRV_PCM_INFO_PAUSE: u32;
    static SNDRV_PCM_INFO_RESUME: u32;
    static SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: u32;
    static SNDRV_PCM_INFO_INTERLEAVED: u32;
    static GFP_KERNEL: u32;
}

const ENOMEM: i32 = 12;

static edma_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe {
        SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_PAUSE
            | SNDRV_PCM_INFO_RESUME
            | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP
            | SNDRV_PCM_INFO_INTERLEAVED
    },
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 32,
    period_bytes_max: 64 * 1024,
    periods_min: 2,
    periods_max: 19, /* Limit by edma dmaengine driver */
};

static edma_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &edma_pcm_hardware as *const snd_pcm_hardware,
    prepare_slave_config: unsafe { snd_dmaengine_pcm_prepare_slave_config },
    prealloc_buffer_size: 128 * 1024,
    chan_names: [core::ptr::null(), core::ptr::null()],
};

#[no_mangle]
pub unsafe extern "C" fn edma_pcm_platform_register(dev: *mut device) -> i32 {
    let mut config: *mut snd_dmaengine_pcm_config;

    if !(*dev).of_node.is_null() {
        return devm_snd_dmaengine_pcm_register(dev, &edma_dmaengine_pcm_config, 0);
    }

    config = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_dmaengine_pcm_config>(),
        GFP_KERNEL,
    ) as *mut snd_dmaengine_pcm_config;
    if config.is_null() {
        return -ENOMEM;
    }

    *config = edma_dmaengine_pcm_config;

    (*config).chan_names[0] = b"tx\0".as_ptr() as *const i8;
    (*config).chan_names[1] = b"rx\0".as_ptr() as *const i8;

    devm_snd_dmaengine_pcm_register(dev, config, 0)
}

/* EXPORT_SYMBOL_GPL(edma_pcm_platform_register); */

/* MODULE_AUTHOR("Peter Ujfalusi <peter.ujfalusi@ti.com>"); */
/* MODULE_DESCRIPTION("eDMA PCM ASoC platform driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
