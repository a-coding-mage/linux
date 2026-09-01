// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 2018 Texas Instruments Incorporated - https://www.ti.com
 *  Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// C dependency intent:
// #include <linux/device.h>
// #include <linux/module.h>
// #include <sound/core.h>
// #include <sound/pcm.h>
// #include <sound/pcm_params.h>
// #include <sound/soc.h>
// #include <sound/dmaengine_pcm.h>
// #include "sdma-pcm.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

pub enum device {}

pub const SNDRV_PCM_INFO_MMAP: c_uint = 0;
pub const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 0;
pub const SNDRV_PCM_INFO_PAUSE: c_uint = 0;
pub const SNDRV_PCM_INFO_RESUME: c_uint = 0;
pub const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: c_uint = 0;
pub const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 0;
pub const GFP_KERNEL: c_uint = 0;
pub const ENOMEM: c_int = 12;
pub const SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX: c_uint = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dmaengine_pcm_config {
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prepare_slave_config: Option<unsafe extern "C" fn()>,
    pub compat_request_channel: Option<unsafe extern "C" fn()>,
    pub compat_filter_fn: Option<unsafe extern "C" fn()>,
    pub dma_dev: *mut device,
    pub chan_names: [*mut c_char; 2],
    pub prealloc_buffer_size: usize,
}

unsafe extern "C" {
    pub fn snd_dmaengine_pcm_prepare_slave_config();
    pub fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
    pub fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
}

static sdma_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP
        | SNDRV_PCM_INFO_INTERLEAVED,
    period_bytes_min: 32,
    period_bytes_max: 64 * 1024,
    buffer_bytes_max: 128 * 1024,
    periods_min: 2,
    periods_max: 255,
    formats: 0,
    rates: 0,
    rate_min: 0,
    rate_max: 0,
    channels_min: 0,
    channels_max: 0,
    fifo_size: 0,
};

static sdma_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &sdma_pcm_hardware,
    prepare_slave_config: Some(snd_dmaengine_pcm_prepare_slave_config),
    prealloc_buffer_size: 128 * 1024,
    compat_request_channel: None,
    compat_filter_fn: None,
    dma_dev: core::ptr::null_mut(),
    chan_names: [core::ptr::null_mut(), core::ptr::null_mut()],
};

#[no_mangle]
pub unsafe extern "C" fn sdma_pcm_platform_register(
    dev: *mut device,
    mut txdmachan: *mut c_char,
    mut rxdmachan: *mut c_char,
) -> c_int {
    let config: *mut snd_dmaengine_pcm_config;
    let mut flags: c_uint = 0;

    /* Standard names for the directions: 'tx' and 'rx' */
    if txdmachan.is_null() && rxdmachan.is_null() {
        return devm_snd_dmaengine_pcm_register(dev, &sdma_dmaengine_pcm_config, 0);
    }

    config = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_dmaengine_pcm_config>(),
        GFP_KERNEL,
    ) as *mut snd_dmaengine_pcm_config;
    if config.is_null() {
        return -ENOMEM;
    }

    *config = sdma_dmaengine_pcm_config;

    if txdmachan.is_null() || rxdmachan.is_null() {
        /* One direction only PCM */
        flags |= SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX;
        if txdmachan.is_null() {
            txdmachan = rxdmachan;
            rxdmachan = core::ptr::null_mut();
        }
    }

    (*config).chan_names[0] = txdmachan;
    (*config).chan_names[1] = rxdmachan;

    devm_snd_dmaengine_pcm_register(dev, config, flags)
}

// EXPORT_SYMBOL_GPL(sdma_pcm_platform_register);

// MODULE_AUTHOR("Peter Ujfalusi <peter.ujfalusi@ti.com>");
// MODULE_DESCRIPTION("sDMA PCM ASoC platform driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
